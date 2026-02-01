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
        EngineCommand::Workdir(q) => {
            data.workdir = q;
        },
        EngineCommand::Interval(v) => {
            let clamped = v.clamp(0, 100);
            data.interval = clamped;
        },
        EngineCommand::Active(status) => {
            data.active = status;
        },
        EngineCommand::Nuke(status) => {
            data.nuke = status;
        },
        EngineCommand::Destructive(status) => {
            data.destructive = status;
        }
    }

    Ok(())
}
