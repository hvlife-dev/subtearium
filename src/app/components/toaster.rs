use leptos::prelude::*;

#[derive(Clone, PartialEq)]
pub struct ToastData {
    pub id: usize,
    pub level: u8,
    pub message: String,
}

#[component]
pub fn Toaster(
    #[prop(into)] toasts: Signal<Vec<ToastData>>
) -> impl IntoView {
    view! {
        <div class="toast-container">
            <For
                each=move || toasts.get()
                key=|t| t.id
                children=move |t| {
                    let (class_name, icon) = match t.level {
                        1 => ("toast info", "ℹ"),
                        2 => ("toast success", "✓"),
                        3 => ("toast error", "✕"),
                        _ => ("toast info", "ℹ"),
                    };
                    
                    view! {
                        <div class=class_name>
                            <span class="toast-icon">{icon}</span>
                            <span class="toast-msg">{t.message.clone()}</span>
                        </div>
                    }
                }
            />
        </div>
    }
}
