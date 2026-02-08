use leptos::prelude::*;

#[component]
pub fn TitleCard(
    #[prop(into)] title: String,
    #[prop(into)] subtitle: String,
    #[prop(into, optional)] icon: Option<String>,
) -> impl IntoView {
    view! {
        <div class="stat-card title-card">
            <div class="title-content">
                <div class="title-main">{title}</div>
                <div class="title-sub">{subtitle}</div>
            </div>
            
            {move || icon.clone().map(|i| view! {
                <div class="title-icon">{i}</div>
            })}
        </div>
    }
}
