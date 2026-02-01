use serde::{Serialize, Deserialize};
use std::sync::{Arc, RwLock};


#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum EngineCommand {
    Workdir(String),
    Interval(i32),
    Active(bool),
    Nuke(bool),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GlobalState {
    pub workdir: String,
    pub interval: i32,
    pub active: bool,
    pub nuke: bool,

    pub songs_amount: i32,
    pub songs_synced: i32,
    pub songs_plain: i32,
    pub songs_tagerr: i32,
    pub songs_noresult: i32,
    pub error_code: u32,
}

pub type AppState = Arc<RwLock<GlobalState>>;

pub fn init_state() -> AppState {
    Arc::new(RwLock::new(GlobalState {
        workdir: "default".to_string(),
        interval: 0,
        active: false,
        nuke: false,

        songs_amount: 0,
        songs_synced: 0,
        songs_plain: 0,
        songs_tagerr: 0,
        songs_noresult: 0,
        error_code: 0,
    }))
}
