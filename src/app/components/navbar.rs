use leptos_router::components::Outlet;
use leptos::prelude::*;
use leptos_router::{
    components::A,
};

use crate::app::components::footer::Footer;

#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <div class="app-container">
            <nav class="sidebar">
                <div 
                    class="brand" 
                    style="margin-bottom: 2rem; font-size: 1.5rem; font-weight: bold; color: #61AFEF;"
                >
                    "Subtearium"
                </div>

                <div style="display: flex; flex-direction: column; gap: 0.5rem;">
                    <A href="/" attr:class="nav-link" exact=true>
                        "Home"
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
        </div>
    }
}
