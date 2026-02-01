use leptos::prelude::*;

#[component]
pub fn Settings() -> impl IntoView {
    let (path, set_path) = signal("/usr/local/bin/tokyo".to_string());
    let (count, set_count) = signal(10);

    view! {
        <div class="card">
            <h2 class="card-title">"Configuration"</h2>

            <div class="form-group">
                <label for="path-input">"Project Path"</label>
                <input 
                    type="text" 
                    id="path-input"
                    class="form-control"
                    placeholder="/home/user/..."
                    prop:value=path
                    on:input=move |ev| {
                        set_path.set(event_target_value(&ev));
                    }
                />
                <p style="font-size: 0.8rem; color: #666; margin-top: 0.4rem;">
                    "Current Path: " <span style="color: #A485DD;">{path}</span>
                </p>
            </div>

            <div class="form-group">
                <label for="threads-input">"Thread Count"</label>
                <input 
                    type="number" 
                    id="threads-input"
                    class="form-control"
                    min="1" 
                    max="64"
                    prop:value=count
                    on:input=move |ev| {
                        let val = event_target_value(&ev).parse::<i32>().unwrap_or(0);
                        set_count.set(val);
                    }
                />
            </div>
        </div>
    }
}
