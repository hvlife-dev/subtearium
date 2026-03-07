use chrono::{DateTime, Local, Utc, Timelike};
use reqwest::Client;
use serde::{Serialize, Deserialize};
use std::{collections::{HashMap, VecDeque}, sync::{Arc, RwLock}};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum SongStatus {
    Unaccounted = 0,
    TagErr = 1,
    NoResult = 2,
    Plain = 3,
    Synced = 4,
    Locked = 5,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum EngineCommand {
    Workdir(String),
    Interval(i32),
    Active(bool),
    Nuke(bool),
    SaveTrig(bool),
    EnableSynced(bool),
    EnablePlain(bool),
    OffsetLyric(String, f32),
    ToggleLock(String),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct GlobalState {
    pub workdir: String,
    pub interval: i32,
    pub active: bool,
    pub nuke: bool,
    pub enable_synced: bool,
    pub enable_plain: bool,
    
    pub songs_amount: i32,
    pub songs_synced: i32,
    pub songs_plain: i32,
    pub songs_noresult: i32,
    pub songs_tagerr: i32,
    pub songs_unaccounted: i32,
    pub songs_locked: i32,

    pub scan_time: DateTime<Utc>,
    pub library: HashMap<String, SongStatus>,
    pub logs: VecDeque<String>,
    
    pub offset_lyric: Option<(String, f32)>,
    pub toggle_lock: Option<String>,
    pub save_trig: bool,
    pub toast_counter: usize,
    pub latest_toast: Option<(u8, String)>,
    pub is_api_running: bool,
    
    #[serde(skip)]
    pub client: Client
}

impl Default for GlobalState {
    fn default() -> Self {
        // should fix memory leak
        #[cfg(feature = "ssr")]
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(15))
            .build()
            .unwrap();

        #[cfg(not(feature = "ssr"))]
        let client = reqwest::Client::new();
        Self {
            workdir: "/music".to_string(),
            interval: 60,
            active: false,
            nuke: false,
            enable_synced: true,
            enable_plain: true,
            
            offset_lyric: None,
            toggle_lock: None,
            save_trig: false,

            songs_amount: 0,
            songs_synced: 0,
            songs_plain: 0,
            songs_noresult: 0,
            songs_tagerr: 0,
            songs_unaccounted: 0,
            songs_locked: 0,

            scan_time: Utc::now(),
            library: HashMap::new(),
            logs: VecDeque::new(),
            
            toast_counter: 0,
            latest_toast: None,
            is_api_running: false,
            
            client,
        }
    }
}

pub type AppState = Arc<RwLock<GlobalState>>;
pub struct ScannerGuard {
    state: AppState,
}

impl ScannerGuard {
    pub fn try_claim(state: AppState) -> Option<Self> {
        let mut data = state.write().unwrap();
        if !data.is_api_running {
            data.is_api_running = true;
            drop(data);
            Some(Self { state })
        } else {
            None
        }
    }
}

impl Drop for ScannerGuard {
    fn drop(&mut self) {
        if let Ok(mut data) = self.state.write() {
            data.is_api_running = false;
        }
    }
}

pub fn init_state() -> AppState {
    let _ = std::fs::create_dir_all("data");
    let t = Local::now();
    let msg;

    let mut state = if 
        let Ok(toml_str) = std::fs::read_to_string("data/db.toml") 
        && let Ok(state) = toml::from_str::<GlobalState>(&toml_str)
    {
        msg = "Service state succesfully loaded";
        state
    } else {
        msg = "Service state invalid, overwriting";
        GlobalState::default()
    };

    let formatted_msg = format!("{:02}:{:02}:{:02} | {}", 
        t.hour(), t.minute(), t.second(), msg);
    println!("{}", formatted_msg);
    state.logs.push_back(formatted_msg);
    if state.logs.len() > 256 {
        state.logs.pop_front();
    }

    state.is_api_running = false;
    state.toast_counter = 0;
    state.latest_toast = None;
    state.save_trig = false;
    
    Arc::new(RwLock::new(state))
}

