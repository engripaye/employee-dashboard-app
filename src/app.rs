pub mod db;
pub mod models;
pub mod server_functions;
pub mod pages;

use pages::HomePage;

use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // Injects a stylesheet into the document <head>
        <Stylesheet
            id="leptos"
            href="/pkg/employee-dashboard-app.css"
        />

        <link
            data-trunk
            rel="tailwind-css"
            href="/style/input.css"
        />

        // Sets the document title
        <Title text="Full-Stack Dashboard App"/>

        <Router>
            <main>
                <Routes fallback=|| view! {
                    <NotFound />
                }>
                    <Route
                        path=path!("/")
                        view=HomePage
                    />
                </Routes>
            </main>
        </Router>
    }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // Set an HTTP status code 404
    #[cfg(feature = "ssr")]
    {
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <h1>"Not Found"</h1>
    }
}