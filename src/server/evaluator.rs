use crate::server::state::{AppState, SongStatus};
use crate::server::tracker::save_library;
use crate::server::calls::get_lyric;
use std::path::PathBuf;
use reqwest::Client;
use futures::stream::{self, StreamExt};
use audiotags::Tag;
use tokio::fs;
use tokio::io::AsyncWriteExt;
use crate::server::misc::{log, shift_lrc_timestamps, is_synced};

pub fn offset_lrc(state: &AppState, offset_lyric: Option<(String, f32)>){
    if let Some((path, offset)) = offset_lyric {
        let lrc_path = PathBuf::from(&path).with_extension("lrc");
        let lrc_path_str = lrc_path.to_string_lossy().to_string();

        match shift_lrc_timestamps(&lrc_path_str, offset) {
            Ok(_) => log(state, 2, &format!("Subtitle at path {} shifted by {}s", path, offset )),
            Err(e) => log(state, 3, &format!("Subtitle at path {} failed shifting with: {}", path, e )),
        }
    }
}

pub fn lock_lrc(state: &AppState, toggle_lock: Option<String>){
    if let Some(path) = toggle_lock {
        let mut data = state.write().unwrap();
        
        if let Some(current_status) = data.library.get(&path).cloned() {
            if current_status == SongStatus::Locked {
                let lrc_path = PathBuf::from(&path).with_extension("lrc");
                
                let new_status = if lrc_path.exists() {
                    if is_synced(&lrc_path) {
                        SongStatus::Synced
                    } else {
                        SongStatus::Plain
                    }
                } else {
                    SongStatus::NoResult
                };
                
                data.library.insert(path.clone(), new_status);
                
                drop(data); 
                log(state, 1, &format!("Unlocked: {}", path));
                
            } else {
                data.library.insert(path.clone(), SongStatus::Locked);
                
                drop(data);
                log(state, 1, &format!("Locked: {}", path));
            }
        }
        let _ = update_stats(state);
        save_library(state);
    }
}


pub fn update_stats(state: &AppState) -> std::io::Result<()> {
    let mut data = state.write().unwrap();
    data.songs_amount = data.library.len() as i32;
    data.songs_locked = data.library.values().filter(|s| **s==SongStatus::Locked).count() as i32;
    data.songs_unaccounted = data.library.values().filter(|s| **s==SongStatus::Unaccounted).count() as i32;
    data.songs_tagerr = data.library.values().filter(|s| **s==SongStatus::TagErr).count() as i32;
    data.songs_noresult = data.library.values().filter(|s| **s==SongStatus::NoResult).count() as i32;
    data.songs_plain = data.library.values().filter(|s| **s==SongStatus::Plain).count() as i32;
    data.songs_synced = data.library.values().filter(|s| **s==SongStatus::Synced).count() as i32;
    Ok(())
}


pub async fn search_missing(state: &AppState) -> std::io::Result<()> {
    let songs;
    let enable_synced;
    {
        let data = state.read().unwrap();
        songs = data.library.clone();
        enable_synced = data.enable_synced;
    }
    let paths: Vec<&String> = songs.iter().filter_map(|s| {
        if 
            *s.1 >= SongStatus::Synced || 
            (*s.1 == SongStatus::Plain && !enable_synced)
        {
            None
        } else {
            Some(s.0)
        }
    } ).collect();

    let client = Client::new();
    let concurrency_limit = 4;

    stream::iter(paths).for_each_concurrent(concurrency_limit, |path| {
        let state_clone = state.clone(); 
        let client_clone = client.clone();
        async move {
            let _ = search_single(&state_clone, &client_clone, path).await;
        }
    }).await;

    Ok(())
}

pub async fn search_single(state: &AppState, client: &Client, path: &str) -> std::io::Result<()> {
    if let Ok(status) = handle_song(state, client, path.into()).await {
        let mut data = state.write().unwrap(); 
        if 
            let Some(val) = data.library.get_mut(path)
            && status > *val 
        {
            let old_status = val.clone();
            *val = status.clone();
            match old_status {
                SongStatus::Unaccounted => data.songs_unaccounted -= 1,
                SongStatus::TagErr      => data.songs_tagerr -= 1,
                SongStatus::NoResult    => data.songs_noresult -= 1,
                SongStatus::Plain       => data.songs_plain -= 1,
                SongStatus::Synced      => data.songs_synced -= 1,
                SongStatus::Locked      => data.songs_locked -= 1,
            }
            match status {
                SongStatus::Unaccounted => data.songs_unaccounted += 1,
                SongStatus::TagErr      => data.songs_tagerr += 1,
                SongStatus::NoResult    => data.songs_noresult += 1,
                SongStatus::Plain       => data.songs_plain += 1,
                SongStatus::Synced      => data.songs_synced += 1,
                SongStatus::Locked      => data.songs_locked += 1,
            }
        }
    }
    
    Ok(())
}


pub async fn handle_song(state: &AppState, client: &Client, path: PathBuf) -> std::io::Result<SongStatus> {
    let enable_syced;
    let enable_plain;
    {
        let data = state.read().unwrap();
        enable_syced = data.enable_synced;
        enable_plain = data.enable_plain;
    }

    let tag = Tag::new().read_from_path(&path);
    if tag.is_err() {return Ok(SongStatus::TagErr);}
    let tag = tag.unwrap();

    let title = tag.title().unwrap_or_default();
    let artist = tag.artist().unwrap_or_default();
    let album = tag.album_title().unwrap_or_default();
    let duration = tag.duration().unwrap_or_default().round() as u32;

    if title.trim().is_empty() || artist.trim().is_empty() || album.trim().is_empty() || duration < 3 {
        log(state, 0, &format!("Missing tags: {}", path.to_str().unwrap_or("Logging err")));
        return Ok(SongStatus::TagErr);
    }
    let duration = duration.to_string();

    match get_lyric(client, title, artist, album, &duration).await {
        Ok(track) => {
            let mut file = fs::File::create(path.with_extension("lrc")).await?;
        
            if enable_syced && let Some(lyrics) = track.synced_lyrics {
                file.write_all(lyrics.as_bytes()).await?;
                log(state, 0, &format!("\nFound synced lyrics: {}", path.to_str().unwrap_or("Logging err")));
                Ok(SongStatus::Synced)
            } else if enable_plain && let Some(lyrics) = track.plain_lyrics {
                file.write_all(lyrics.as_bytes()).await?;
                log(state, 0, &format!("\nFound only plain lyrics: {}", path.to_str().unwrap_or("Logging err")));
                Ok(SongStatus::Plain)
            } else {
                log(state, 0, &format!("\nFound no results: {}", path.to_str().unwrap_or("Logging err")));
                Ok(SongStatus::NoResult)
            }
        },
        Err(e) => {
            if e.code == 400 {
                log(state, 0, &format!("Missing tags: {}", path.to_str().unwrap_or("Logging err")));
                Ok(SongStatus::TagErr)
            } else {
                log(state, 0, &format!("Found no results: {}", path.to_str().unwrap_or("Logging err")));
                Ok(SongStatus::NoResult)
            }
        }
    }
}
