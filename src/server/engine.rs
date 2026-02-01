use crate::server::state::AppState;


pub async fn run_lyrics_engine(state: AppState) {
    loop {
        if nuke_check(&state) {
            let path;
            {
                let data = state.read().unwrap();
                path = data.workdir.clone();
            }
            let _ = parse_library(path, &state).await;
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


use std::path::PathBuf;
use audiotags::Tag;
use tokio::fs;
use tokio::io::AsyncWriteExt;

pub async fn parse_library(root_path: String, state: &AppState) -> std::io::Result<()> {
    let mut stack = vec![PathBuf::from(root_path)];

    while let Some(dir_path) = stack.pop() {
        let mut entries = match fs::read_dir(&dir_path).await {
            Ok(entries) => entries,
            Err(e) => {
                eprintln!("Failed to read directory {:?}: {}", dir_path, e);
                continue;
            }
        };

        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            
            if path.is_dir() {
                stack.push(path);
                continue;
            }
            if path.extension().is_none() {continue;}

            let allowed = [
                "mp3", "mp4", "m4a", "flac"
            ];
            let ext = path.extension().unwrap().to_str().unwrap_or("nil");
            if !allowed.contains( &ext ) {continue;}

            {
                let mut data = state.write().unwrap();
                data.songs_amount += 1;
            }

            if let Ok(status) = handle_song(path).await {
                match status {
                    SongStatus::TagErr => {
                        let mut data = state.write().unwrap();
                        data.songs_tagerr += 1;
                    },
                    SongStatus::NoResult => {
                        let mut data = state.write().unwrap();
                        data.songs_noresult += 1;
                    },
                    SongStatus::Plain => {
                        let mut data = state.write().unwrap();
                        data.songs_plain += 1;
                    },
                    SongStatus::Synced => {
                        let mut data = state.write().unwrap();
                        data.songs_synced += 1;
                    },
                }
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
#[derive(Debug, Clone)]
enum SongStatus {
    TagErr,
    NoResult,
    Plain,
    Synced
}
