use crate::server::state::{AppState, SongStatus};
use crate::server::tracker::{init_library, read_library, save_library, update_library};
use crate::server::calls::get_lyric;
use std::path::PathBuf;
use audiotags::Tag;
use chrono::Utc;
use tokio::fs;
use tokio::io::AsyncWriteExt;
use crate::server::misc::{log, shift_lrc_timestamps};

pub async fn run_lyrics_engine(state: AppState) {

    if !read_library(&state) {
        let path = "/music";
        let _ = init_library(path, &state);
        let _ = update_stats(&state);
        save_library(&state);
    }
    log(&state, 1, "Subtearium ready");

    loop {
        let scan_time;
        let interval;
        let path;
        let tobefound;
        let active;
        let save_trig;
        let offset_lyric;
        let nuke;
        {
            let data = state.read().unwrap();
            scan_time = data.scan_time;
            interval = data.interval;
            path = data.workdir.clone();
            tobefound = data.songs_plain + data.songs_noresult + data.songs_unaccounted;
            active = data.active;
            save_trig = data.save_trig;
            offset_lyric = data.offset_lyric.clone();
            nuke = data.nuke;
        }

        // if triggered database saving
        if save_trig {
            log(&state, 1, "Saved settings");
            save_library(&state);
            {
                let mut data = state.write().unwrap();
                data.save_trig = false;
            }
            update_library(&path, &state, false);
            let _ = update_stats(&state);
            let _ = search_missing(&state).await;
            let _ = update_stats(&state);
            save_library(&state);
        }        

        // if any new songs found
        if active && update_library(&path, &state, true) {
            let _ = update_stats(&state);
            let _ = search_missing(&state).await;
            let _ = update_stats(&state);
            save_library(&state);
        }
        // if periodic time reached
        if active && interval > 0 && Utc::now().signed_duration_since(scan_time).num_minutes() > interval as i64 {
            log(&state, 1, "Begin periodic scan");
            {
                let mut data = state.write().unwrap();
                data.scan_time = Utc::now();
            }
            let _ = update_stats(&state);
            if tobefound > 0 {
                log(&state, 2, &format!("Found {} missing lyrics, begin search", tobefound));
                let _ = search_missing(&state).await;
            }
            let _ = update_stats(&state);
            save_library(&state);
        }
        // if user wants to overwrite everything
        if nuke {
            {
                let mut data = state.write().unwrap();
                data.nuke = false;
            }
            log(&state, 1, "Begin reinitializing database");
            let _ = init_library(&path, &state);
            let _ = search_missing(&state).await;
            let _ = update_stats(&state);
            save_library(&state);
        }

        // offsetting .lrc selected in library
        if let Some((path, offset)) = offset_lyric {
            let lrc_path = PathBuf::from(&path).with_extension("lrc");
            let lrc_path_str = lrc_path.to_string_lossy().to_string();

            match shift_lrc_timestamps(&lrc_path_str, offset) {
                Ok(_) => log(&state, 2, &format!("Subtitle at path {} shifted by {}s", path, offset )),
                Err(e) => log(&state, 3, &format!("Subtitle at path {} failed shifting with: {}", path, e )),
            }
            let mut data = state.write().unwrap();
            data.offset_lyric = None;
        }
    }
}


fn update_stats(state: &AppState) -> std::io::Result<()> {
    let mut data = state.write().unwrap();
    data.songs_amount = data.library.len() as i32;
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
    let paths = songs.iter().filter_map(|s| {
        if 
            *s.1 == SongStatus::Synced || 
            (*s.1 == SongStatus::Plain && !enable_synced)
        {
            None
        } else {
            Some(s.0)
        }
    } );

    for path in paths {
        if let Ok(status) = handle_song(state, path.clone().into()).await {
            let mut data = state.write().unwrap();
            if let Some(val) = data.library.get_mut(path) {
                *val = status;
            }
        }
    }

    Ok(())
}


async fn handle_song(state: &AppState, path: PathBuf) -> std::io::Result<SongStatus> {
    log(state, 0, &format!("\nProcessing song at: {}", path.to_str().unwrap_or("Logging err")));

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

    match get_lyric(title, artist, album, &duration).await {
        Ok(track) => {
            let mut file = fs::File::create(path.with_extension("lrc")).await?;
        
            if enable_syced && let Some(lyrics) = track.synced_lyrics {
                file.write_all(lyrics.as_bytes()).await?;
                log(state, 0, "Found synced lyrics");
                Ok(SongStatus::Synced)
            } else if enable_plain && let Some(lyrics) = track.plain_lyrics {
                file.write_all(lyrics.as_bytes()).await?;
                log(state, 0, "Found only plain lyrics");
                Ok(SongStatus::Plain)
            } else {
                log(state, 0, "No result found");
                Ok(SongStatus::NoResult)
            }
        },
        Err(e) => {
            log(state, 0, &format!("{} | {} | {}", e.code, e.name, e.message));
            Ok(SongStatus::NoResult)
        }
    }
}
