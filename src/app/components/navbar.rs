use leptos_router::components::Outlet;
use leptos::prelude::*;
use leptos_router::components::A;
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
                </div>
                
                <div class="nav-links-container">
                    <A href="/" attr:class="nav-link" exact=true>
                        <svg xmlns="http://www.w3.org/2000/svg" class="nav-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                            <path d="M3 9l9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"></path>
                            <polyline points="9 22 9 12 15 12 15 22"></polyline>
                        </svg>
                        <span>"Home"</span>
                    </A>
                    
                    <A href="/status" attr:class="nav-link">
                        <svg xmlns="http://www.w3.org/2000/svg" class="nav-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                            <polyline points="22 12 18 12 15 21 9 3 6 12 2 12"></polyline>
                        </svg>
                        <span>"Status"</span>
                    </A>
                    
                    <A href="/settings" attr:class="nav-link">
                        <svg xmlns="http://www.w3.org/2000/svg" class="nav-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                            <circle cx="12" cy="12" r="3"></circle>
                            <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"></path>
                        </svg>
                        <span>"Settings"</span>
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
