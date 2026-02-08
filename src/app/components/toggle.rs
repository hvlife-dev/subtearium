use leptos::prelude::*;

#[component]
pub fn Toggle(
    #[prop(into)] label: String,
    #[prop(into)] value: Signal<bool>,
    #[prop(into)] on_toggle: Callback<bool>,
    #[prop(into, optional)] on_text: Option<String>,
    #[prop(into, optional)] off_text: Option<String>,
) -> impl IntoView {
    let text_true = on_text.unwrap_or_else(|| "ONLINE".to_string());
    let text_false = off_text.unwrap_or_else(|| "OFFLINE".to_string());

    view! {
        <div class="form-group">
            <label>{label}</label>
            <button
                class=move || if value.get() { "btn-toggle active" } else { "btn-toggle" }
                on:click=move |_| on_toggle.run(!value.get())
            >
                {move || if value.get() { 
                    text_true.clone() 
                } else { 
                    text_false.clone() 
                }}
            </button>
        </div>
    }
}
