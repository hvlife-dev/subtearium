use crate::server::{misc::log, state::{AppState, GlobalState, SongStatus}};
use std::{fs::File, io::{Read, Write}, path::Path};
use walkdir::WalkDir;
use std::fs;


// soft-insert new songs
pub fn update_library(root_path: &str, state: &AppState) -> bool {
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
            if Path::new(e.path().with_extension("lrc").as_path()).exists() {return;}
            if let Some(p) = e.path().to_str() {
                let _ = library.try_insert(p.to_string(), SongStatus::Unaccounted);
            }
        } )
    ;
    
    let mut data = state.write().unwrap();
    let diff = data.library != library;
    data.library = library;

    diff
}

pub fn init_library(root_path: &str, state: &AppState) -> std::io::Result<()> {
    log(state, "Initializing service database");
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

pub fn save_library(state: &AppState) -> bool {
    let _ = fs::create_dir("data");
    let data = state.read().unwrap().clone();

    if let Ok(toml) = toml::to_string(&data){
        if let Ok(mut file) = File::create("data/db.toml") {
            if file.write(toml.as_bytes()).is_ok() {
                return true;
            }
        };
    };
    
    log(state, "Saving service state failed");
    false
}

pub fn read_library(state: &AppState) -> bool {
    log(state, "Loading service state");

    if let Ok(mut file) = File::open("data/db.toml") {
        let mut toml = String::new();
        if file.read_to_string(&mut toml).is_ok() {
            if let Ok(decoded) = toml::from_str::<GlobalState>(&toml){
                {
                    let mut data = state.write().unwrap();
                    *data = decoded;
                }
                log(state, "Service state loaded succesfully");
                return true;
            }
        }
    };
    
    log(state, "Service state loading failure");
    false
}
