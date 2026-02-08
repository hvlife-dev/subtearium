use chrono::{Timelike, Utc};

use crate::server::state::AppState;

pub fn log(state: &AppState, msg: &str) {
    let t = Utc::now();
    let msg = format!("{}:{}:{} UTC | {}", 
        t.hour(), t.minute(), t.second(), msg);
    // println!("{}", msg);

    let mut data = state.write().unwrap();
    data.logs.push_back(msg);

    if data.logs.len() > 64 {
        data.logs.pop_front();
    }
}
