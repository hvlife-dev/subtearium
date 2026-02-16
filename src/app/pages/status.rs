use leptos::prelude::*;
use crate::server::state::GlobalState;
use crate::app::hooks::use_engine::use_engine;
use crate::app::components::library_explorer::LibraryExplorer;

#[component]
pub fn Status() -> impl IntoView {
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
            <Transition fallback=|| "Loading Settings...">
                {move || form_latch.get().map(|data| view! {
                    <LibraryExplorer 
                        library=data.library
                        engine=engine.clone()
                    />
                })}
            </Transition>
        </div>
    }
}

