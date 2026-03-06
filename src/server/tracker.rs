use futures::stream::{self, StreamExt};
use crate::server::{evaluator::search_single, misc::{is_synced, log}, state::{AppState, GlobalState, SongStatus}};
use std::{fs::File, io::{Read, Write}, path::Path};
use reqwest::Client;
use walkdir::{DirEntry, WalkDir};
use std::fs;

// remove key from library, if physical song disappeared
pub fn cleanup(state: &AppState) {
    let mut songs = {
        state.read().unwrap().library.clone()
    };
    songs.retain(|key, _|{
        let path = Path::new(key);
        path.exists()
    });
    let mut data = state.write().unwrap();
    data.library = songs;
}

// walk through every file in a library
pub async fn update_library(root_path: &str, state: &AppState, search_new: Option<Client>) {
    let allowed = ["mp3", "mp4", "m4a", "flac"];
    let concurrency_limit = 4;
    
    let walker = WalkDir::new(root_path).into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file())
        .filter(|e| allowed.contains(&e.path().extension().and_then(|e| e.to_str()).unwrap_or("nil")));

    stream::iter(walker)
        .for_each_concurrent(concurrency_limit, |e| {
            let client_clone = search_new.clone();
            
            async move {
                let new = update_entry(state, &e);
                let active = {state.read().unwrap().active};
                if // change physical files only on those conditions
                    active
                    && let Some(client) = client_clone
                    && new
                    && let Some(path) = e.path().to_str()
                {
                    let _ = search_single(state, &client, path).await;
                }
            }
        }).await;
}

// returns, if that entry is new
fn update_entry(state: &AppState, entry: &DirEntry) -> bool {
    // checking which status should be assigned to that entry, based on lyric path
    let status = {
        let binding = entry.path().with_extension("lrc");
        let lyric_path = Path::new(binding.as_path());
        
        if !lyric_path.exists() {
            SongStatus::Unaccounted
        } else if is_synced(lyric_path) {
            SongStatus::Synced
        } else {
            SongStatus::Plain
        }
    };

    // getting music file path, it's used as library key
    let path = if let Some(path) = entry.path().to_str() {
        path.to_string()
    } else {
        return false;
    };

    let mut data = state.write().unwrap();

    if let Some(existing) = data.library.get_mut(&path) {
        if *existing == SongStatus::Locked {
            return false;
        }

        if // allows for detecting lrc deletion, while preventing other causes to disappear
            status == SongStatus::Unaccounted
            && (*existing == SongStatus::TagErr || *existing == SongStatus::NoResult) 
        {
            return false; 
        }

        if *existing != status {
            *existing = status;
        }
        false
    } else {
        data.library.insert(path, status.clone());
        true
    }
}

pub fn save_library(state: &AppState) -> bool {
    let _ = fs::create_dir("data");
    let data = state.read().unwrap().clone();

    if let Ok(toml) = toml::to_string(&data)
        && let Ok(mut file) = File::create("data/db.toml")
            && file.write(toml.as_bytes()).is_ok() {
                return true;
    };
    
    log(state, 3, "Saving service state failed");
    false
}

pub fn read_library(state: &AppState) -> bool {
    log(state, 0, "Loading service state");

    if let Ok(mut file) = File::open("data/db.toml") {
        let mut toml = String::new();
        if 
            file.read_to_string(&mut toml).is_ok() 
            && let Ok(decoded) = toml::from_str::<GlobalState>(&toml) 
        {
            {
                let mut data = state.write().unwrap();
                *data = decoded;
            }
            log(state, 2, "Service state loaded succesfully");
            return true;
        } else {
            log(state, 3, "Invalid service state file");
        }
    } else {
        log(state, 3, "Service state file does not exist or is inaccessible");
    }
    
    false
}
