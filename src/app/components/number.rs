use leptos::prelude::*;

#[component]
pub fn NumberInput(
    #[prop(into)] label: String,
    #[prop(into)] value: i32,
    #[prop(into)] on_input: Callback<i32>,
    #[prop(into, optional)] min: i32,
    #[prop(into, optional)] max: i32,
) -> impl IntoView {
    view! {
        <div class="form-group">
            <label>{label}</label>
            <input 
                type="number" 
                class="form-control"
                min=min
                max=max
                prop:value=value.to_string()
                on:input=move |ev| {
                    let val_str = event_target_value(&ev);
                    let val = val_str.parse::<i32>().unwrap_or(0);
                    on_input.run(val);
                }
            />
        </div>
    }
}
