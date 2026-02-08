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
        if let Some(Ok(data)) = engine.status.get() {
            if form_latch.get_untracked().is_none() {
                form_latch.set(Some(data));
            }
        }
    });

    view! {
        <div class="container" style="padding: 2rem; max-width: 800px; margin: 0 auto;">

            <Transition fallback=|| "Connecting to Database...">
                {move || engine.status.get().map(|res| match res {
                    Ok(data) => view! {
                        
                        <Divider text="Buttons"/>
                        
                        <Toggle 
                            label="Enable lyrics pulling" 
                            value=data.active
                            on_toggle=engine.set_active
                            on_text="Active"
                            off_text="Dead"
                        />
                        
                        <Toggle 
                            label="Force try to overwrite every lyric in library" 
                            value=data.nuke
                            on_toggle=engine.set_nuke
                            on_text="Nuke"
                            off_text="Nuke"
                        />

                    }.into_any(),
                    
                    Err(e) => view! { 
                        <div class="card" style="border-color: #EE6D85;">
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
                        initial_destructive=data.destructive
                        initial_interval=data.interval
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
    initial_destructive: bool,
    engine: EngineController,
) -> impl IntoView {
    let workdir_buf = RwSignal::new(initial_workdir);
    let interval_buf = RwSignal::new(initial_interval);
    let destructive_buf = RwSignal::new(initial_destructive);

    view! {
        <BufferedPanel 
            title="Settings"
            on_apply=move |_| {
                engine.set_workdir.run(workdir_buf.get());
                engine.set_interval.run(interval_buf.get());
                engine.set_destructive.run(destructive_buf.get());
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
                label="Scanning interval [minutes]" 
                value=interval_buf
                min=1
                max=144000
                on_input=move |v| interval_buf.set(v)
            />
            <Toggle 
                label="Should subtearium overwrite already existing lyrics" 
                value=destructive_buf
                on_toggle=move |v| destructive_buf.set(v)
                on_text="Yes"
                off_text="Nope"
            />
        </BufferedPanel>
    }
}
