use crate::server::{misc::{log, is_synced}, state::{AppState, GlobalState, SongStatus}};
use std::{fs::File, io::{Read, Write}, path::Path};
use walkdir::WalkDir;
use std::fs;


// soft-insert new songs
pub fn update_library(root_path: &str, state: &AppState, quick: bool) -> bool {
    let allowed = [
        "mp3", "mp4", "m4a", "flac"
    ];
    
    let mut library;
    {
        let data = state.read().unwrap();
        library = data.library.clone();
    }
    WalkDir::new(root_path).into_iter()
        .filter_map(|e| e.ok() )
        .filter(|e| e.path().is_file() )
        .filter(|e| allowed.contains( &e.path().extension().and_then(|e| e.to_str() ).unwrap_or("nil") ))
        .for_each(|e| { 
            let binding = e.path().with_extension("lrc");
            let p = Path::new(binding.as_path());
            if let Some(st) = e.path().to_str() {
                if p.exists() {
                    if !quick {
                        if is_synced(p) {
                            let _ = library.try_insert(st.to_string(), SongStatus::Synced);
                        } else {
                            let _ = library.try_insert(st.to_string(), SongStatus::Plain);
                        }
                    }
                } else {
                    let _ = library.try_insert(st.to_string(), SongStatus::Unaccounted);
                }
            }
        } )
    ;
    
    let mut data = state.write().unwrap();
    let diff = data.library != library;
    data.library = library;

    diff
}

pub fn init_library(root_path: &str, state: &AppState) -> std::io::Result<()> {
    log(state, 1, "Initializing service database");
    let allowed = [
        "mp3", "mp4", "m4a", "flac"
    ];
    
    let library = WalkDir::new(root_path).into_iter()
        .filter_map(|e| e.ok() )
        .filter(|e| e.path().is_file() )
        .filter(|e| allowed.contains( &e.path().extension().and_then(|e| e.to_str() ).unwrap_or("nil") ))
        .filter_map(|e| {
            let binding = e.path().with_extension("lrc");
            let p = Path::new(binding.as_path());
            if !p.exists() {
                e.path().to_str().map(|s| (s.to_string(), SongStatus::Unaccounted))
            } else if is_synced(p) {
                e.path().to_str().map(|s| (s.to_string(), SongStatus::Synced))
            } else {
                e.path().to_str().map(|s| (s.to_string(), SongStatus::Plain))
            }
        } )
        .collect()
    ;
    
    let mut data = state.write().unwrap();
    data.library = library;

    Ok(())
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
