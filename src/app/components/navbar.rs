use leptos_router::components::Outlet;
use leptos::prelude::*;
use leptos_router::{
    components::A,
};

#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <div class="app-container">
        <nav class="sidebar">
            <div class="brand">"Subtearium"</div>

            <A href="/" attr:class="nav-link" exact=true>
                "Home"
            </A>
            <A href="/settings" attr:class="nav-link">
                "Settings"
            </A>
        </nav>

            <main class="main-content">
                <Outlet/>
            </main>
        </div>
    }
}
