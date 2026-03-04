use leptos::prelude::*;
use std::collections::VecDeque;

#[component]
pub fn LiveLogViewer(
    #[prop(into)] logs: Signal<VecDeque<String>>,
) -> impl IntoView {
    let log_ref = NodeRef::<leptos::html::Div>::new();
    
    let (auto_scroll, set_auto_scroll) = signal(true);
    let on_scroll = move |_| {
        if let Some(div) = log_ref.get_untracked() {
            let scroll_top = div.scroll_top();
            let scroll_height = div.scroll_height();
            let client_height = div.client_height();

            let is_at_bottom = (scroll_height - scroll_top - client_height).abs() < 10;
            set_auto_scroll.set(is_at_bottom);
        }
    };

    Effect::new(move |_| {
        logs.track();

        if auto_scroll.get_untracked() {
            request_animation_frame(move || {
                if let Some(div) = log_ref.get_untracked() {
                    div.set_scroll_top(div.scroll_height());
                }
            });
        }
    });

    view! {
        <div 
            node_ref=log_ref 
            class="code-block log-viewer-scroll"
            on:scroll=on_scroll
        >
            <div style="display: flex; flex-direction: column;">
                {move || logs.get().into_iter().map(|line| view! {
                    <div class="log-line">
                        <span class="prompt">"> "</span>
                        {line}
                    </div>
                }).collect_view()}
            </div>
        </div>
    }
}
