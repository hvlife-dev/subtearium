use leptos::prelude::*;
use crate::app::components::divider::Divider;
use crate::server::state::GlobalState;
use crate::app::hooks::use_engine::{
    use_engine,
    EngineController
};
use crate::app::components::{
    toggle::Toggle,
    number::NumberInput,
    text::TextInput,
    buffered_panel::BufferedPanel,
};

#[component]
pub fn Settings() -> impl IntoView {
    let engine = use_engine();

    let form_latch = RwSignal::new(None::<GlobalState>);
    Effect::new(move |_| {
        if let Some(Ok(data)) = engine.status.get()
            && form_latch.get_untracked().is_none() {
                form_latch.set(Some(data));
            }
    });

    view! {
        <div class="page-container">

            <Transition fallback=|| "Connecting to Database...">
                {move || engine.status.get().map(|res| match res {
                    Ok(data) => view! {
                        
                        <Divider text="Buttons"/>
                        <div class="toggle-row">
                        
                            <Toggle 
                                label="Enable lyrics pulling" 
                                value=data.active
                                on_toggle=engine.set_active
                                on_text="Active"
                                off_text="Dead"
                            />
                            <Toggle 
                                label="Force recheck of every lyric in library" 
                                value=data.nuke
                                on_toggle=engine.set_nuke
                                on_text="Scheduled"
                                off_text="Nuke"
                            />
                        </div>

                    }.into_any(),
                    
                    Err(e) => view! { 
                        <div class="card error">
                            "Connection Error: " {e.to_string()}
                        </div>
                    }.into_any()
                })}
            </Transition>
            
            <Divider text="Static Variables"/>

            <Transition fallback=|| "Loading Settings...">
                {move || form_latch.get().map(|data| view! {
                    <BufferedForm 
                        initial_workdir=data.workdir 
                        initial_interval=data.interval
                        initial_enable_synced=data.enable_synced
                        initial_enable_plain=data.enable_plain
                        engine=engine.clone() 
                    />
                })}
            </Transition>
        </div>
    }
}

#[component]
fn BufferedForm(
    initial_workdir: String,
    initial_interval: i32,
    initial_enable_synced: bool,
    initial_enable_plain: bool,
    engine: EngineController,
) -> impl IntoView {
    let workdir_buf = RwSignal::new(initial_workdir);
    let interval_buf = RwSignal::new(initial_interval);
    let enable_synced_buf = RwSignal::new(initial_enable_synced);
    let enable_plain_buf = RwSignal::new(initial_enable_plain);

    view! {
        <BufferedPanel 
            title="Settings"
            on_apply=move |_| {
                engine.set_workdir.run(workdir_buf.get());
                engine.set_interval.run(interval_buf.get());
                engine.set_enable_synced.run(enable_synced_buf.get());
                engine.set_enable_plain.run(enable_plain_buf.get());
                engine.set_savetrig.run(true);
            }
        >
            <TextInput 
                label="Workdir - your library path" 
                value=workdir_buf
                placeholder="Type library path..."
                on_input=move |v| workdir_buf.set(v)
            />
            <NumberInput 
                label="Scanning interval (0 - disabled) [minutes]" 
                value=interval_buf
                min=0
                max=144000
                on_input=move |v| interval_buf.set(v)
            />
            <Toggle 
                label="Download synced lyrics" 
                value=enable_synced_buf
                on_toggle=move |v| enable_synced_buf.set(v)
                on_text="Yes"
                off_text="Nope"
            />
            <Toggle 
                label="Download plain lyrics (as fallback)" 
                value=enable_plain_buf
                on_toggle=move |v| enable_plain_buf.set(v)
                on_text="Yes"
                off_text="Nope"
            />
        </BufferedPanel>
    }
}
