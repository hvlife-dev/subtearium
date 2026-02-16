use leptos::prelude::*;
use gloo_timers::callback::Interval;
use crate::server::api::{get_engine_status, send_command};
use crate::server::state::{EngineCommand, GlobalState};

#[derive(Clone)]
pub struct EngineController {
    pub status: Resource<Result<GlobalState, ServerFnError>>,
    
    pub set_active: Callback<bool>,
    pub set_workdir: Callback<String>,
    pub set_interval: Callback<i32>,
    pub set_nuke: Callback<bool>,
    pub set_savetrig: Callback<bool>,
    pub set_enable_synced: Callback<bool>,
    pub set_enable_plain: Callback<bool>,
    pub offset_lyric: Callback<(String, f32)>,
}

pub fn use_engine() -> EngineController {
    let status = Resource::new(|| (), |_| async move { get_engine_status().await });
    Effect::new(move |_| {
        let handle = Interval::new(1000, move || status.refetch());
        move || drop(handle)
    });
    let dispatch = Action::new(|cmd: &EngineCommand| {
        let cmd = cmd.clone();
        async move { send_command(cmd).await }
    });


    let set_active = Callback::new(move |new_val: bool| {
        dispatch.dispatch(EngineCommand::Active(new_val));
        status.update(|data| {
            if let Some(Ok(state)) = data {
                state.active = new_val;
            }
        });
        set_timeout(move || status.refetch(), std::time::Duration::from_millis(200));
    });

    let set_nuke = Callback::new(move |new_val: bool| {
        dispatch.dispatch(EngineCommand::Nuke(new_val));
        status.update(|data| {
            if let Some(Ok(state)) = data {
                state.nuke = new_val;
            }
        });
        set_timeout(move || status.refetch(), std::time::Duration::from_millis(200));
    });
    
    let set_workdir = Callback::new(move |new_query: String| {
        dispatch.dispatch(EngineCommand::Workdir(new_query.clone()));
        
        status.update(|data| {
            if let Some(Ok(state)) = data {
                state.workdir = new_query;
            }
        });
        set_timeout(move || status.refetch(), std::time::Duration::from_millis(200));
    });

    let set_interval = Callback::new(move |new_val: i32| {
        let safe_val = new_val.clamp(0, 100);
        
        dispatch.dispatch(EngineCommand::Interval(safe_val));

        status.update(|data| {
            if let Some(Ok(state)) = data {
                state.interval = safe_val;
            }
        });
        set_timeout(move || status.refetch(), std::time::Duration::from_millis(200));
    });
    
    let set_savetrig = Callback::new(move |new_val: bool| {
        dispatch.dispatch(EngineCommand::SaveTrig(new_val));
        status.update(|data| {
            if let Some(Ok(state)) = data {
                state.save_trig = new_val;
            }
        });
        set_timeout(move || status.refetch(), std::time::Duration::from_millis(200));
    });
    
    let set_enable_synced = Callback::new(move |new_val: bool| {
        dispatch.dispatch(EngineCommand::EnableSynced(new_val));
        status.update(|data| {
            if let Some(Ok(state)) = data {
                state.enable_synced = new_val;
            }
        });
        set_timeout(move || status.refetch(), std::time::Duration::from_millis(200));
    });

    let set_enable_plain = Callback::new(move |new_val: bool| {
        dispatch.dispatch(EngineCommand::EnablePlain(new_val));
        status.update(|data| {
            if let Some(Ok(state)) = data {
                state.enable_plain = new_val;
            }
        });
        set_timeout(move || status.refetch(), std::time::Duration::from_millis(200));
    });
    
    let offset_lyric = Callback::new(move |(path, offset): (String, f32)| {
        dispatch.dispatch(EngineCommand::OffsetLyric(path.clone(), offset));
        status.update(|data| {
            if let Some(Ok(state)) = data {
                state.offset_lyric = Some((path, offset));
            }
        });
        set_timeout(move || status.refetch(), std::time::Duration::from_millis(200));
    });

    EngineController {
        status,
        set_active,
        set_workdir,
        set_interval,
        set_nuke,
        set_savetrig,
        set_enable_synced,
        set_enable_plain,
        offset_lyric
    }
}
