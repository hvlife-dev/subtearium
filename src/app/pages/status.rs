use leptos::prelude::*;
use crate::app::hooks::use_engine::use_engine;
use crate::app::components::library_explorer::LibraryExplorer;

#[component]
pub fn Status() -> impl IntoView {
    let engine = use_engine();

    let lib_sig = Signal::derive(move || {
        engine.status.get()
            .and_then(|res| res.ok())
            .map(|data| data.library)
            .unwrap_or_default()
    });

    view! {
        <div class="page-container">
            <Transition fallback=|| "Loading Library...">
                <Show 
                    when=move || !lib_sig.with(|lib| lib.is_empty()) 
                    fallback=|| view! { <div class="card">"No songs found in library yet."</div> }
                >
                    <LibraryExplorer 
                        library=lib_sig
                        engine=engine.clone()
                    />
                </Show>
            </Transition>
        </div>
    }
}
