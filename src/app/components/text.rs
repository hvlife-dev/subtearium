use leptos::prelude::*;

#[component]
pub fn TextInput(
    #[prop(into)] label: String,
    #[prop(into)] value: String,
    #[prop(into)] on_input: Callback<String>,
    #[prop(into, optional)] placeholder: String,
) -> impl IntoView {
    view! {
        <div class="form-group">
            <label>{label}</label>
            <input 
                type="text" 
                class="form-control"
                placeholder=placeholder
                prop:value=value
                on:input=move |ev| {
                    let val = event_target_value(&ev);
                    on_input.run(val);
                }
            />
        </div>
    }
}
