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
    use rusqlite::Connection;
    use std::time::Duration;
    use time::{Date, Month, OffsetDateTime, Time};

    let start_time = OffsetDateTime::new_utc(
        Date::from_calendar_date(2022, Month::March, 12)?,
        Time::from_hms(9, 3, 0)?,
    );
    let end_time = start_time + Duration::from_secs(range.seconds());

    let conn = Connection::open("measurements.sqlite3")?;

    let mut stmt = conn.prepare(
        "SELECT timestamp, temperature, humidity
        FROM measurements
        WHERE timestamp BETWEEN ?1 AND ?2
        ORDER BY timestamp ASC",
    )?;
    let measurements = stmt
        .query_map(
            [start_time.unix_timestamp(), end_time.unix_timestamp()],
            |row| {
                Ok(Measurement {
                    timestamp: row.get(0)?,
                    temperature: row.get(1)?,
                    humidity: row.get(2)?,
                })
            },
        )?
        .collect::<Result<Vec<_>, _>>()?;

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
