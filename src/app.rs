use leptos::prelude::*;
use leptos_meta::{MetaTags, Stylesheet, Title, provide_meta_context};
use leptos_router::{
    StaticSegment,
    components::{Route, Router, Routes},
};

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
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    let res1 = Resource::new(move || (), move |_| res1());

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/mre.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <ErrorBoundary fallback=|_| ()>
                    <Suspense fallback=||()>
                        { move || res1.and_then(|v| v.clone()) }
                    </Suspense>
                </ErrorBoundary>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    let res3 = Resource::new(move || (), move |_| res3());
    let res2_1 = Resource::new(move || (), move |_| res2());
    let res2_2 = Resource::new(|| (), |_| res2());

    view! {
            <Suspense fallback=||()>
                { move || res2_1.and_then(|v| v.clone()) }
                { move || res3.and_then(|v| v.clone()) }
            </Suspense>
            <Suspense fallback=||()>
                { move || res2_2.and_then(|v| v.clone()) }
            </Suspense>
    }
}

#[server]
async fn res1() -> Result<String, ServerFnError> {
    Ok("Resource 1".into())
}

#[server]
async fn res2() -> Result<String, ServerFnError> {
    Ok("Resource 2".into())
}

#[server]
async fn res3() -> Result<String, ServerFnError> {
    Ok("Resource 3".into())
}
