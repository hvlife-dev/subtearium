use leptos_router::components::Outlet;
use leptos::prelude::*;
use leptos_router::{
    components::A,
};
use crate::app::components::footer::Footer;
use crate::app::hooks::use_engine::use_engine;
use crate::app::components::toaster::{Toaster, ToastData};

#[component]
pub fn Navbar() -> impl IntoView {
    let engine = use_engine();
    let (toasts, set_toasts) = signal(Vec::<ToastData>::new());
    let last_toast_id = RwSignal::new(None::<usize>);

    Effect::new(move |_| {
        if let Some(Ok(data)) = engine.status.get() {
            let current_id = last_toast_id.get_untracked();
            
            match current_id {
                None => {
                    last_toast_id.set(Some(data.toast_counter));
                }
                
                Some(current_id) => {
                    if data.toast_counter > current_id {
                        last_toast_id.set(Some(data.toast_counter));
                        
                        if 
                            let Some((level, msg)) = data.latest_toast.clone() 
                            && level > 0 
                        {
                            let new_toast = ToastData { 
                                id: data.toast_counter, 
                                level, 
                                message: msg 
                            };
                                
                            set_toasts.update(|t| t.push(new_toast.clone()));
                                
                            let id_to_remove = new_toast.id;
                            set_timeout(move || {
                                set_toasts.update(|t| t.retain(|x| x.id != id_to_remove));
                            }, std::time::Duration::from_millis(3500));
                        }
                    }
                }
            }
        }
    });

    view! {
        <div class="app-container">
            <nav class="sidebar">
                <div class="brand">
                    <img src="/neon.svg" alt="Subtearium Logo" class="brand-logo" />
                    <span class="brand-text">"Subtearium"</span>
                </div>

                <div style="display: flex; flex-direction: column; gap: 0.5rem;">
                    <A href="/" attr:class="nav-link" exact=true>
                        "Home"
                    </A>
                    <A href="/status" attr:class="nav-link">
                        "Status"
                    </A>
                    <A href="/settings" attr:class="nav-link">
                        "Settings"
                    </A>
                </div>
            </nav>

            <main class="main-content" style="display: flex; flex-direction: column;">
                <div style="flex-grow: 1;">
                    <Outlet/>
                </div>

                <Footer />
            </main>
            <Toaster toasts=toasts />
        </div>
    }
}
