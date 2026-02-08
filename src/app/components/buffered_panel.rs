use leptos::prelude::*;

#[component]
pub fn BufferedPanel(
    #[prop(into)] title: String,
    #[prop(into)] on_apply: Callback<()>,
    children: Children,
) -> impl IntoView {
    view! {
        <div class="card" style="border: 1px solid #4C566A;">
            <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 1rem;">
                <h3>{title}</h3>
            </div>

            {children()}

            <hr style="border-color: #333; margin: 1.5rem 0;"/>

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
