use leptos::prelude::*;

use crate::app::components::divider::Divider;

#[component]
pub fn BufferedPanel(
    #[prop(into)] title: String,
    #[prop(into)] on_apply: Callback<()>,
    children: Children,
) -> impl IntoView {
    view! {
        <div class="card">
            <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 1rem;">
                <h3>{title}</h3>
            </div>

            {children()}

            <Divider/>

            <button 
                class="btn-primary"
                style="width: 100%;"
                on:click=move |_| on_apply.run(())
            >
                "APPLY CHANGES"
            </button>
        </div>
    }
}
