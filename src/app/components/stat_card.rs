use leptos::prelude::*;

#[component]
pub fn StatCard(
    #[prop(into)] label: String,
    #[prop(into)] value: String,
    #[prop(into, optional)] accent_class: Option<String>,
) -> impl IntoView {
    let accent = accent_class.unwrap_or_else(|| "accent-blue".to_string());

    view! {
        <div class=format!("stat-card {}", accent)>
            <div class="stat-value">
                {value}
            </div>
            <div class="stat-label">
                {label}
            </div>
        </div>
    }
}
