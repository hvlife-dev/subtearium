use leptos::prelude::*;

#[component]
pub fn StatCard(
    #[prop(into)] label: String,
    #[prop(into)] value: String,
    #[prop(into, optional)] color: Option<String>,
) -> impl IntoView {
    let accent = color.unwrap_or_else(|| "#61AFEF".to_string());

    view! {
        <div 
            class="stat-card"
            style=format!("border-top: 3px solid {};", accent)
        >
            <div class="stat-value" style=format!("color: {};", accent)>
                {value}
            </div>
            <div class="stat-label">
                {label}
            </div>
        </div>
    }
}
