use futures::stream::{self, StreamExt};
use std::path::Path;
use walkdir::{DirEntry, WalkDir};
use crate::server::state::{AppState, SongStatus};
use crate::server::misc::is_synced;
use crate::server::evaluator::search_single;
use crate::server::misc::log;

// remove key from library, if physical song disappeared
pub fn cleanup(state: &AppState) {
    let paths_to_check: Vec<String> = {
        state.read().unwrap().library.keys().cloned().collect()
    };

    let mut missing_paths = Vec::new();
    for path in paths_to_check {
        if !Path::new(&path).exists() {
            missing_paths.push(path);
        }
    }

    if !missing_paths.is_empty() {
        let mut data = state.write().unwrap();
        for path in missing_paths {
            data.library.remove(&path);
        }
    }
}

// walk through every file in a library
pub async fn update_library(root_path: &str, state: &AppState, search_new: bool) {
    let allowed = ["mp3", "mp4", "m4a", "flac"];
    let concurrency_limit = 4;
    let base_client = if search_new {
        Some(state.read().unwrap().client.clone())
    } else {
        None
    };
    
    let walker = WalkDir::new(root_path).into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file())
        .filter(|e| allowed.contains(&e.path().extension().and_then(|e| e.to_str()).unwrap_or("nil")));

    stream::iter(walker)
        .for_each_concurrent(concurrency_limit, |e| {
            let client_clone = base_client.clone();
            
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

pub async fn save_library(state: &AppState) -> bool {
    let toml_result = {
        let data = state.read().unwrap();
        toml::to_string(&*data)
    };

    if let Ok(toml_string) = toml_result {
        let _ = tokio::fs::create_dir_all("data").await;
        if tokio::fs::write("data/db.toml", toml_string).await.is_ok() {
            return true;
        }
    }
    
    log(state, 3, "Saving service state failed");
    false
}
