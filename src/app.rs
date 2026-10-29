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
          <meta charset="utf-8" />
          <meta name="viewport" content="width=device-width, initial-scale=1" />
          <AutoReload options=options.clone() />
          <HydrationScripts options />
          <MetaTags />
        </head>
        <body>
          <App />
        </body>
      </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
      // injects a stylesheet into the document <head>
      // id=leptos means cargo-leptos will hot-reload this stylesheet
      <Stylesheet id="leptos" href="/pkg/mre.css" />

      // sets the document title
      <Title text="Welcome to Leptos" />

      // content for this welcome page
      <Router>
        <main>
          <Routes fallback=|| "Page not found.".into_view()>
            <Route path=StaticSegment("") view=HomePage />
          </Routes>
        </main>
      </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    let res = Resource::new(|| (), move |_| data2());
    let res3 = Resource::new(|| (), move |_| data3());

    view! {
      <ErrorBoundary fallback=|_| ()>
        <Await future=data() let:nums>
          {nums
            .clone()
            .map(|v| {
              v.into_iter()
                .map(|v| {
                  view! { <option value=v.clone()>{v.clone()}</option> }
                })
                .collect_view()
            })}
        </Await>
      </ErrorBoundary>
      <Suspense>{move || res.and_then(|v| v.iter().map(|n| view! { <p>{n.clone()}</p> }).collect_view())}</Suspense>
      <Suspense>
        {move || res3.and_then(|v| v.iter().map(|n| view! { <div>{n.clone()}"-"{n.clone()}</div> }).collect_view())}
      </Suspense>
    }
}

#[server]
async fn data() -> Result<Vec<String>, ServerFnError> {
    Ok(vec!["1".into(), "2".into(), "3".into()])
}

#[server]
async fn data2() -> Result<Vec<String>, ServerFnError> {
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    Ok(vec!["1".into(), "2".into(), "3".into(), "4".into()])
}

#[server]
async fn data3() -> Result<Vec<String>, ServerFnError> {
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    Ok(vec!["1".into(), "2".into(), "3".into(), "4".into(), "5".into()])
}
