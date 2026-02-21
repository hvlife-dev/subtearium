use leptos::prelude::*;
use crate::server::state::{GlobalState, EngineCommand};

#[server(GetEngineStatus, "/api")]
pub async fn get_engine_status() -> Result<GlobalState, ServerFnError> {
    use axum::Extension;
    use crate::server::state::AppState;
    use leptos_axum::extract;

    let Extension(state): Extension<AppState> = extract().await?;
    
    let data = state.read().map_err(|_| ServerFnError::new("Lock poisoned"))?.clone();
    
    Ok(data)
}

#[server(SendCommand, "/api/command")]
pub async fn send_command(cmd: EngineCommand) -> Result<(), ServerFnError> {
    use axum::Extension;
    use crate::server::state::AppState;
    use leptos_axum::extract;

    let Extension(state): Extension<AppState> = extract().await?;

    let mut data = state.write().map_err(|_| ServerFnError::new("Lock error"))?;

    match cmd {
        EngineCommand::Workdir(v) => {
            data.workdir = v;
        },
        EngineCommand::Interval(v) => {
            let clamped = v.max(0);
            data.interval = clamped;
        },
        EngineCommand::Active(v) => {
            data.active = v;
        },
        EngineCommand::Nuke(v) => {
            data.nuke = v;
        },
        EngineCommand::SaveTrig(v) => {
            data.save_trig = v
        },
        EngineCommand::EnableSynced(v) => {
            data.enable_synced = v;
        },
        EngineCommand::EnablePlain(v) => {
            data.enable_plain = v;
        },
        EngineCommand::OffsetLyric(path, offset) => {
            data.offset_lyric = Some((path, offset));
        },
        EngineCommand::ToggleLock(v) => {
            data.toggle_lock = Some(v);
        },
    }

    Ok(())
}

