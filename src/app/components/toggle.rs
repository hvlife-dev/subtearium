use leptos::prelude::*;

#[component]
pub fn Toggle(
    #[prop(into)] label: String,
    #[prop(into)] value: bool,
    #[prop(into)] on_toggle: Callback<bool>,
) -> impl IntoView {
    view! {
        <div class="form-group">
            <label>{label}</label>
            <button
                class=if value { "btn-toggle active" } else { "btn-toggle" }
                on:click=move |_| on_toggle.run(!value)
            >
                {if value { "ONLINE" } else { "OFFLINE" }}
            </button>
        </div>
    }
}
