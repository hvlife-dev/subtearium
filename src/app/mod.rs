use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes, ParentRoute},
    path
};

mod pages;
mod components;
mod hooks;
use crate::app::pages::home::HomePage;
use crate::app::pages::settings::Settings;
use crate::app::components::navbar::Navbar;


pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/subtearium.css"/>
        <Title text="Subtearium"/>

        <Router>
            <Routes fallback=|| "Page not found.".into_view()>
                <ParentRoute path=path!("/") view=Navbar>
                    <Route path=path!("") view=HomePage/>
                    <Route path=path!("settings") view=Settings/>
                </ParentRoute>
            </Routes>
        </Router>
    }
}

