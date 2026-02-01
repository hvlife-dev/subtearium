use crate::server::state::{AppState, SongStatus};


pub async fn run_lyrics_engine(state: AppState) {
    loop {
        if nuke_check(&state) {
            let path;
            {
                let data = state.read().unwrap();
                path = data.workdir.clone();
            }
            let _ = init_library(path, &state);
            let _ = search_missing(&state).await;
            let _ = update_stats(&state);
        }
    }
}

fn nuke_check(state: &AppState) -> bool {
    let nuke = {
        let data = state.read().unwrap();
        data.nuke
    };

    if nuke {
        let mut data = state.write().unwrap();
        data.nuke = false;
        data.songs_amount = 0;
        data.songs_tagerr = 0;
        data.songs_noresult = 0;
        data.songs_plain = 0;
        data.songs_synced = 0;
        return true;
    }
    false
}

fn update_stats(state: &AppState) -> std::io::Result<()> {
    let mut data = state.write().unwrap();
    data.songs_amount = data.library.len() as i32;
    data.songs_unaccounted = data.library.values().filter(|s| **s==SongStatus::Unaccounted).count() as i32;
    data.songs_tagerr = data.library.values().filter(|s| **s==SongStatus::TagErr).count() as i32;
    data.songs_noresult = data.library.values().filter(|s| **s==SongStatus::NoResult).count() as i32;
    data.songs_plain = data.library.values().filter(|s| **s==SongStatus::Plain).count() as i32;
    data.songs_synced = data.library.values().filter(|s| **s==SongStatus::Synced).count() as i32;
    data.songs_predating = data.library.values().filter(|s| **s==SongStatus::Predating).count() as i32;
    Ok(())
}

use std::path::{Path, PathBuf};
use audiotags::Tag;
use tokio::fs;
use tokio::io::AsyncWriteExt;
use walkdir::WalkDir;


fn init_library(root_path: String, state: &AppState) -> std::io::Result<()> {
    let allowed = [
        "mp3", "mp4", "m4a", "flac"
    ];
    
    let destructive;
    {
        let data = state.read().unwrap();
        destructive = data.destructive;
    }
    let library = WalkDir::new(root_path).into_iter()
        .filter_map(|e| e.ok() )
        .filter(|e| e.path().is_file() )
        .filter(|e| allowed.contains( &e.path().extension().and_then(|e| e.to_str() ).unwrap_or("nil") ))
        .filter_map(|e| { 
            if !destructive && Path::new(e.path().with_extension("lrc").as_path()).exists() {
                e.path().to_str().map(|s| (s.to_string(), SongStatus::Predating))
            } else {
                e.path().to_str().map(|s| (s.to_string(), SongStatus::Unaccounted))
            }
        } )
        .collect()
    ;
    
    let mut data = state.write().unwrap();
    data.library = library;

    Ok(())
}

async fn search_missing(state: &AppState) -> std::io::Result<()> {
    let songs = {
        let data = state.read().unwrap();
        data.library.clone()
    };
    let paths = songs.iter().filter_map(|s| {
        if *s.1 != SongStatus::Predating && *s.1 != SongStatus::Synced {
            Some(s.0)
        } else {
            None
        }
    } );

    for path in paths {
        if let Ok(status) = handle_song(path.clone().into()).await {
            let mut data = state.write().unwrap();
            if let Some(val) = data.library.get_mut(path) {
                *val = status;
            }
        }
    }

    Ok(())
}

use crate::server::calls::get_lyric;

async fn handle_song(path: PathBuf) -> std::io::Result<SongStatus> {
    println!("Found song at: {:?}", path);
    let tag = Tag::new().read_from_path(&path);
    if tag.is_err() {return Ok(SongStatus::TagErr);}
    let tag = tag.unwrap();

    let title = tag.title().unwrap_or_default();
    let artist = tag.artist().unwrap_or_default();
    let album = tag.album_title().unwrap_or_default();
    let duration = tag.duration().unwrap_or_default().round() as u32;
    let duration = duration.to_string();

    if let Ok(track) = get_lyric(title, artist, album, &duration).await {
        let mut file = fs::File::create(path.with_extension("lrc")).await?;
        
        if let Some(lyrics) = track.synced_lyrics {
            file.write_all(lyrics.as_bytes()).await?;
            return Ok(SongStatus::Synced);
        } else if let Some(lyrics) = track.plain_lyrics {
            file.write_all(lyrics.as_bytes()).await?;
            return Ok(SongStatus::Plain);
        }
    };

    Ok(SongStatus::NoResult)
}
