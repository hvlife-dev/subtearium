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
        <div 
            node_ref=log_ref
            class="code-block"
            style="
                min-height: 100px;
                max-height: 600px;
                overflow-y: auto; 
                display: flex; 
                flex-direction: column-reverse;
            "
        >
            <div style="display: flex; flex-direction: column;">
                {logs.into_iter().map(|line| view! {
                    <div style="font-family: monospace; font-size: 0.85rem; border-bottom: 1px solid #333; padding: 2px 0;">
                        <span style="color: #61AFEF;">"> "</span>
                        {line}
                    </div>
                }).collect_view()}
            </div>
        </div>
    }
}
