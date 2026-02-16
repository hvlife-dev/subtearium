use chrono::{Timelike, Utc};
use std::fs::{self, File};
use std::io::{self, BufRead};
use std::path::Path;

use crate::server::state::AppState;

pub fn log(state: &AppState, level: u8, msg: &str) {
    let t = Utc::now();
    let formatted_msg = format!("{:02}:{:02}:{:02} UTC | {}", 
        t.hour(), t.minute(), t.second(), msg);
    // println!("{}", formatted_msg);

    let mut data = state.write().unwrap();
    data.logs.push_back(formatted_msg);
    if data.logs.len() > 64 {
        data.logs.pop_front();
    }

    if level > 0 {
        data.toast_counter += 1;
        data.latest_toast = Some((level, msg.to_string())); 
    }
}

pub fn is_synced(path: &Path) -> bool {
    let Ok(file) = File::open(path) else {
        return false;
    };

    let reader = io::BufReader::new(file);

    for content in reader.lines().map_while(Result::ok) {
        let trimmed = content.trim();
        let bytes = trimmed.as_bytes();

        if bytes.first() != Some(&b'[') {
            continue;
        }

        let has_digit = bytes.get(1).is_some_and(|b| b.is_ascii_digit());
        
        if has_digit && 
            (
                matches!(bytes.get(3), Some(b':')) ||   
                matches!(bytes.get(2), Some(b':')) || 
                matches!(bytes.get(4), Some(b':'))
            )
        {
            return true;
        }
    }

    false
}

pub fn shift_lrc_timestamps(filepath: &str, offset_sec: f32) -> Result<(), String> {
    let content = fs::read_to_string(filepath).map_err(|e| e.to_string())?;
    let mut result = String::new();

    for line in content.lines() {
        if line.starts_with('[')
            && let Some(end_idx) = line.find(']') {
                let time_str = &line[1..end_idx];
                let parts: Vec<&str> = time_str.split(':').collect();
                
                if parts.len() == 2
                    && let (Ok(min), Ok(sec)) = (parts[0].parse::<f32>(), parts[1].parse::<f32>()) {
                        let total_sec = min * 60.0 + sec + offset_sec;
                        let new_total = total_sec.max(0.0);
                        
                        let new_min = (new_total / 60.0).floor() as i32;
                        let new_sec = new_total % 60.0;
                        
                        let new_time_str = format!("[{:02}:{:05.2}]", new_min, new_sec);
                        result.push_str(&new_time_str);
                        result.push_str(&line[end_idx + 1..]);
                        result.push('\n');
                        continue;
                    }
            }
        result.push_str(line);
        result.push('\n');
    }

    fs::write(filepath, result).map_err(|e| e.to_string())
}
