use crate::server::evaluator::{lock_lrc, offset_lrc, search_missing, update_stats};
use crate::server::state::{AppState, ScannerGuard};
use crate::server::tracker::{cleanup, read_library, save_library, update_library};
use reqwest::Client;
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
    let mut last_quick_scan = std::time::Instant::now();

    log(&state, 1, "Subtearium ready");

    loop {
        let path;
        let interval_trigger;
        let save_trig;
        let nuke;
        let offset_lyric;
        let toggle_lock;
        {
            let mut data = state.write().unwrap();
            path = data.workdir.clone();
            interval_trigger = 
                data.interval > 0 
                && Utc::now().signed_duration_since(data.scan_time).num_minutes() > data.interval as i64
            ;

            save_trig = data.save_trig; if save_trig {data.save_trig=false;}
            nuke = data.nuke; // reset inside thread
            offset_lyric = data.offset_lyric.take();
            toggle_lock = data.toggle_lock.take();
        }

        let should_search = nuke || interval_trigger;
        
        // main search loop, if unactive it's only supposed to update file state
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
                
                cleanup(&bg_state);
                update_library(&bg_path, &bg_state, None).await;
                let _ = update_stats(&bg_state);

                let active = {
                    bg_state.read().unwrap().active
                };
                if active {
                    let _ = search_missing(&bg_state).await;
                    let _ = update_stats(&bg_state);
                }

                save_library(&bg_state);
                {
                    let mut data = bg_state.write().unwrap();
                    data.is_api_running = false;
                    data.scan_time = Utc::now();
                }
                log(&bg_state, 1, "API search complete");
            });
        }

        // almost-continously running check for new music files
        if 
            !should_search 
            && last_quick_scan.elapsed().as_secs() >= 240 
            && let Some(_guard) = ScannerGuard::try_claim(state.clone()) 
        {
            last_quick_scan = std::time::Instant::now();
            let bg_state = state.clone();
            let bg_path = path.clone();
            tokio::spawn(async move {
                let _active_guard = _guard;
                log(&bg_state, 1, "Begin quick check for new files");
                let client = Client::new();

                update_library(&bg_path, &bg_state, Some(client)).await;
                let _ = update_stats(&bg_state);
                save_library(&bg_state);
                log(&bg_state, 1, "Quick check complete");
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

