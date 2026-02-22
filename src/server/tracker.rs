use futures::stream::{self, StreamExt};
use crate::server::{evaluator::search_single, misc::{is_synced, log}, state::{AppState, GlobalState, SongStatus}};
use std::{fs::File, io::{Read, Write}, path::Path};
use reqwest::Client;
use walkdir::WalkDir;
use std::fs;


pub async fn update_single(root_path: &str, state: &AppState) {
    let allowed = ["mp3", "mp4", "m4a", "flac"];
    let client = Client::new(); 
    let concurrency_limit = 4;

    let walker = WalkDir::new(root_path).into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file())
        .filter(|e| allowed.contains(&e.path().extension().and_then(|e| e.to_str()).unwrap_or("nil")));

    stream::iter(walker)
        .for_each_concurrent(concurrency_limit, |e| {
            let client_clone = client.clone();
            
            async move {
                let binding = e.path().with_extension("lrc");
                let p = Path::new(binding.as_path());
                
                if let Some(st) = e.path().to_str() {
                    let path_str = st.to_string();
                    let is_new = {
                        let data = state.read().unwrap();
                        !p.exists() && !data.library.contains_key(&path_str)
                    };
                    
                    if is_new {
                        {
                            let mut data = state.write().unwrap();
                            data.library.insert(path_str.clone(), SongStatus::Unaccounted);
                            data.songs_amount += 1;
                            data.songs_unaccounted += 1;
                        }
                        let _ = search_single(state, &client_clone, &path_str).await;
                    }
                }
            }
        })
        .await;
}

pub fn update_library(root_path: &str, state: &AppState) {
    let allowed = ["mp3", "mp4", "m4a", "flac"];
    
    WalkDir::new(root_path).into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file())
        .filter(|e| allowed.contains(&e.path().extension().and_then(|e| e.to_str()).unwrap_or("nil")))
        .for_each(|e| { 
            let binding = e.path().with_extension("lrc");
            let p = Path::new(binding.as_path());
            
            if let Some(st) = e.path().to_str() {
                let path_str = st.to_string();
                
                if !p.exists() {
                    update_entry(state, path_str, SongStatus::Unaccounted);
                } else if is_synced(p) {
                    update_entry(state, path_str, SongStatus::Synced);
                } else {
                    update_entry(state, path_str, SongStatus::Plain);
                }
            }
        });
}

fn update_entry(state: &AppState, path: String, status: SongStatus) {
    let mut data = state.write().unwrap();

    if let Some(existing) = data.library.get_mut(&path) {
        if *existing == SongStatus::Locked {
            return;
        }

        if 
            status == SongStatus::Unaccounted
            && (*existing == SongStatus::TagErr || *existing == SongStatus::NoResult) 
        {
            return; 
        }

        if *existing != status {
            *existing = status;
        }
    } else {
        data.library.insert(path, status.clone());
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
        if file.read_to_string(&mut toml).is_ok() 
            && let Ok(decoded) = toml::from_str::<GlobalState>(&toml) {
            {
                let mut data = state.write().unwrap();
                *data = decoded;
            }
            log(state, 2, "Service state loaded succesfully");
            return true;
        }
    };
    
    log(state, 3, "Service state loading failure");
    false
}
