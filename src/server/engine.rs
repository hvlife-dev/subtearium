use crate::server::evaluator::{lock_lrc, offset_lrc, search_missing, update_stats};
use crate::server::state::{AppState, ScannerGuard};
use crate::server::tracker::{read_library, save_library, update_library, update_single};
use tokio::time::{sleep, Duration};
use chrono::Utc;
use crate::server::misc::log;

pub async fn run_lyrics_engine(state: AppState) {

    if !read_library(&state) {
        save_library(&state);
    }
    { // to prevent reboot deadlock
        let mut data = state.write().unwrap();
        data.is_api_running = false;
        data.nuke = false;
        data.toast_counter = 0;
    }

    log(&state, 1, "Subtearium ready");


    let disk_state = state.clone();
    tokio::spawn(async move {
        loop {
            if let Some(_guard) = ScannerGuard::try_claim(disk_state.clone()) {
                let path = disk_state.read().unwrap().workdir.clone();
                
                update_single(&path, &disk_state).await;
                let _ = update_stats(&disk_state);
                save_library(&disk_state);
            }

            sleep(Duration::from_secs(240)).await;
        }
    });

    loop {
        let active;
        let path;
        let interval_trigger;
        let save_trig;
        let nuke;
        let offset_lyric;
        let toggle_lock;
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
        }

        let should_search = nuke || interval_trigger;

        if should_search && let Some(_guard) = ScannerGuard::try_claim(state.clone()) {
            {
                let mut data = state.write().unwrap();
                data.is_api_running = true;
                data.nuke = false;
            }

            let bg_state = state.clone();
            let bg_path = path.clone();

            tokio::spawn(async move {
                let _active_guard = _guard;
                log(&bg_state, 1, "Begin library API search");
                
                update_library(&bg_path, &bg_state);
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

