use leptos::prelude::*;

#[component]
pub fn Divider(
    #[prop(into, optional)] text: Option<String>,
) -> impl IntoView {
    view! {
        <div class="separator-container">
            {move || match text.clone() {
                Some(txt) => view! {
                    <div class="separator-line"></div>
                    <span class="separator-text">{txt}</span>
                    <div class="separator-line"></div>
                }.into_any(),
                None => view! {
                    <div class="separator-line"></div>
                }.into_any()
            }}
        </div>
    }
}
