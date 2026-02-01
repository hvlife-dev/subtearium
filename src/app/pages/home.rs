use leptos::prelude::*;
use crate::app::components::{
    toggle::Toggle,
    number::NumberInput,
    text::TextInput
};
use crate::app::hooks::use_engine::use_engine;

#[component]
pub fn HomePage() -> impl IntoView {
    let engine = use_engine();

    view! {
        <div class="container" style="padding: 2rem; max-width: 800px; margin: 0 auto;">

            <h2>"Parameters"</h2>

            <Transition fallback=|| "Connecting to Database...">
                {move || engine.status.get().map(|res| match res {
                    Ok(data) => view! {
                        
                        <TextInput 
                            label="Workdir - your library path" 
                            value=data.workdir.clone()
                            placeholder="Type to search..."
                            on_input=engine.set_workdir 
                        />
                        <Toggle 
                            label="Destructive?" 
                            value=data.destructive 
                            on_toggle=engine.set_destructive
                        />
                        <Toggle 
                            label="Nuke all lyrics?" 
                            value=data.nuke 
                            on_toggle=engine.set_nuke
                        />
                        <Toggle 
                            label="Active?" 
                            value=data.active 
                            on_toggle=engine.set_active
                        />
                        <NumberInput 
                            label="Scanning interval [seconds]" 
                            value=data.interval
                            min=0 
                            max=100
                            on_input=engine.set_interval 
                        />

                        <hr style="border-color: #333;"/>
                        
                        <div>
                            <h3>"Make-a-surer"</h3>
                            <pre style="color: #98C379;">"Totally: " {data.songs_amount}</pre>
                            <pre style="color: #98C379;">"Predating: " {data.songs_predating}</pre>
                            <pre style="color: #98C379;">"Synced: " {data.songs_synced}</pre>
                            <pre style="color: #98C379;">"Plain: " {data.songs_plain}</pre>
                            <pre style="color: #98C379;">"Not found: " {data.songs_noresult}</pre>
                            <pre style="color: #98C379;">"Badly tagged: " {data.songs_tagerr}</pre>
                            <pre style="color: #98C379;">"Unaccounted: " {data.songs_unaccounted}</pre>
                        </div>

                    }.into_any(),
                    
                    Err(e) => view! { 
                        <div class="card" style="border-color: #EE6D85;">
                            "Connection Error: " {e.to_string()}
                        </div>
                    }.into_any()
                })}
            </Transition>

            <hr style="border-color: #333;"/>
        </div>
    }
}
