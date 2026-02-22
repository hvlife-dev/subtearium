use leptos::prelude::*;

#[component]
pub fn EngineStatusCard(
    #[prop(into)] is_running: Signal<bool>,
    #[prop(into, optional)] accent_class: Option<String>,
) -> impl IntoView {
    let default_accent = accent_class.unwrap_or_else(|| "accent-blue".to_string());

    let card_class = move || {
        if is_running.get() {
            format!("stat-card status-card is-running {}", default_accent)
        } else {
            format!("stat-card status-card is-idle {}", default_accent)
        }
    };

    let status_text = move || if is_running.get() { "WORKING" } else { "IDLE" };

    view! {
        <div class=card_class>
            <div class="stat-value status-value">
                <span class="status-dot"></span>
                {status_text}
            </div>
            <div class="stat-label">
                "API Engine"
            </div>
        </div>
    }
}
