use crate::server::state::{AppState, SongStatus};
use crate::server::tracker::{read_library, save_library, update_library};
use crate::server::calls::get_lyric;
use std::path::PathBuf;
use reqwest::Client;
use futures::stream::{self, StreamExt};
use tokio::time::{sleep, Duration};
use audiotags::Tag;
use chrono::Utc;
use tokio::fs;
use tokio::io::AsyncWriteExt;
use crate::server::misc::{log, shift_lrc_timestamps, is_synced};

pub async fn run_lyrics_engine(state: AppState) {

    if !read_library(&state) {
        save_library(&state);
    }
    log(&state, 1, "Subtearium ready");


    let disk_state = state.clone();
    tokio::spawn(async move {
        loop {
            let active;
            let path;
            {
                let data = disk_state.read().unwrap();
                active = data.active;
                path = data.workdir.clone();
            }

            if active && update_library(&path, &disk_state, true) {
                let _ = update_stats(&disk_state);
                save_library(&disk_state);
                let mut data = disk_state.write().unwrap();
                data.disk_trigger = true; 
            }
            
            sleep(Duration::from_secs(120)).await;
        }
    });

    { // to prevent reboot deadlock
        let mut data = state.write().unwrap();
        data.is_api_running = false;
        data.nuke = false;
        data.toast_counter = 0;
    }


    loop {
        let active;
        let path;
        let interval_trigger;
        let save_trig;
        let nuke;
        let offset_lyric;
        let toggle_lock;
        let disk_trigger;
        let is_api_running;
        {
            let mut data = state.write().unwrap();
            active = data.active;
            path = data.workdir.clone();
            interval_trigger = active && data.interval > 0 && 
                Utc::now().signed_duration_since(data.scan_time).num_minutes() > data.interval as i64;

            save_trig = data.save_trig; if save_trig {data.save_trig=false;}
            nuke = data.nuke; // reset inside thread
            offset_lyric = data.offset_lyric.take();
            toggle_lock = data.toggle_lock.take();
            disk_trigger = data.disk_trigger; if disk_trigger {data.disk_trigger=false;}
            is_api_running = data.is_api_running;
        }

        let should_search = nuke || interval_trigger || disk_trigger;

        if should_search && !is_api_running {
            {
                let mut data = state.write().unwrap();
                data.is_api_running = true;
                data.nuke = false;
            }

            let bg_state = state.clone();
            let bg_path = path.clone();

            tokio::spawn(async move {
                log(&bg_state, 1, "Begin library API search");
                
                update_library(&bg_path, &bg_state, false);
                let _ = update_stats(&bg_state);
                let _ = search_missing(&bg_state).await;
                let _ = update_stats(&bg_state);
                save_library(&bg_state);

                {
                    let mut data = bg_state.write().unwrap();
                    data.is_api_running = false;
                    data.scan_time = Utc::now();
                }
                
                log(&bg_state, 1, "API search complete");
            });
        }

        if save_trig {
            log(&state, 1, "Saved settings");
            save_library(&state);
        }

        offset_lrc(&state, offset_lyric);
        
        lock_lrc(&state, toggle_lock);
        
        sleep(Duration::from_millis(100)).await;
    }
}

fn offset_lrc(state: &AppState, offset_lyric: Option<(String, f32)>){
    if let Some((path, offset)) = offset_lyric {
        let lrc_path = PathBuf::from(&path).with_extension("lrc");
        let lrc_path_str = lrc_path.to_string_lossy().to_string();

        match shift_lrc_timestamps(&lrc_path_str, offset) {
            Ok(_) => log(state, 2, &format!("Subtitle at path {} shifted by {}s", path, offset )),
            Err(e) => log(state, 3, &format!("Subtitle at path {} failed shifting with: {}", path, e )),
        }
    }
}

fn lock_lrc(state: &AppState, toggle_lock: Option<String>){
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


fn update_stats(state: &AppState) -> std::io::Result<()> {
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


async fn search_missing(state: &AppState) -> std::io::Result<()> {
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
            if let Ok(status) = handle_song(&state_clone, &client_clone, path.clone().into()).await {
                let mut data = state.write().unwrap();
                if let Some(val) = data.library.get_mut(path) && status > *val {
                    *val = status;
                }
            }
        }
    }).await;

    Ok(())
}


async fn handle_song(state: &AppState, client: &Client, path: PathBuf) -> std::io::Result<SongStatus> {
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
    let duration = duration.to_string();

    match get_lyric(client, title, artist, album, &duration).await {
        Ok(track) => {
            let mut file = fs::File::create(path.with_extension("lrc")).await?;
        
            if enable_syced && let Some(lyrics) = track.synced_lyrics {
                file.write_all(lyrics.as_bytes()).await?;
                log(state, 0, &format!("\nFound synced lyrics for: {}", path.to_str().unwrap_or("Logging err")));
                Ok(SongStatus::Synced)
            } else if enable_plain && let Some(lyrics) = track.plain_lyrics {
                file.write_all(lyrics.as_bytes()).await?;
                log(state, 0, &format!("\nFound only plain lyrics for: {}", path.to_str().unwrap_or("Logging err")));
                Ok(SongStatus::Plain)
            } else {
                log(state, 0, &format!("\nFound no results for: {}", path.to_str().unwrap_or("Logging err")));
                Ok(SongStatus::NoResult)
            }
        },
        Err(e) => {
            log(state, 0, &format!("{} | {} | {}", e.code, e.name, e.message));
            Ok(SongStatus::NoResult)
        }
    }
}
