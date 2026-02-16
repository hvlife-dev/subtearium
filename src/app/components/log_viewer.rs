use leptos::prelude::*;
use std::collections::VecDeque;

#[component]
pub fn LiveLogViewer(
    #[prop(into)] logs: VecDeque<String>,
) -> impl IntoView {
    let log_ref = NodeRef::<leptos::html::Div>::new();
    
    Effect::new(move |_| {
        if let Some(div) = log_ref.get() {
            div.set_scroll_top(div.scroll_height());
        }
    });

    view! {
        <div node_ref=log_ref class="code-block log-viewer-scroll">
            <div style="display: flex; flex-direction: column;">
                {logs.into_iter().map(|line| view! {
                    <div class="log-line">
                        <span class="prompt">"> "</span>
                        {line}
                    </div>
                }).collect_view()}
            </div>
        </div>
    }
}
