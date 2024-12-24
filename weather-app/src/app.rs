use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};
use serde::{Deserialize, Serialize};

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
        <Stylesheet id="leptos" href="/pkg/weather-app.css" />

        // sets the document title
        <Title text="Weather" />

        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=MeasurementsPage />
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn MeasurementsPage() -> impl IntoView {
    view! { <Measurements /> }
}

#[component]
fn Measurements() -> impl IntoView {
    let measurements = OnceResource::new(load_measurements(TimeRange::Day));

    view! {
        <Suspense fallback=move || view! { <p>"Loading ..."</p> }>
            <table>
                <thead>
                    <tr>
                        <th>"Date"</th>
                        <th>"Temperature (°C)"</th>
                        <th>"Relative humidity (%)"</th>
                    </tr>
                </thead>
                <tbody>
                    {Suspend::new(async move {
                        measurements
                            .await
                            .map(|m| {
                                m.into_iter()
                                    .map(|m| {
                                        view! {
                                            <tr>
                                                <td>{m.timestamp}</td>
                                                <td>{m.temperature}</td>
                                                <td>{m.humidity}</td>
                                            </tr>
                                        }
                                    })
                                    .collect_view()
                            })
                    })}
                </tbody>
            </table>
        </Suspense>
    }
}

#[server]
async fn load_measurements(range: TimeRange) -> Result<Vec<Measurement>, ServerFnError> {
    use std::time::Duration;
    use tokio::time;

    time::sleep(Duration::from_secs(2)).await;

    let measurements = (0..12)
        .map(|i| Measurement {
            timestamp: i,
            temperature: i as f32,
            humidity: i as f32,
        })
        .collect();

    Ok(measurements)
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum TimeRange {
    Day,
    Month,
    Year,
}

impl TimeRange {
    fn seconds(&self) -> u64 {
        match &self {
            TimeRange::Day => 86_400,
            // Both of these are debatable but accurate enough for us
            TimeRange::Month => 30 * 86_400,
            TimeRange::Year => 365 * 86_400,
        }
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Measurement {
    timestamp: u64,
    temperature: f32,
    humidity: f32,
}
