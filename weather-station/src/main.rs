use eyre::{eyre, Result};
use reqwest::blocking::Client;
use serde::Serialize;
use std::{
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

const PIN: u8 = 4;
const ENDPOINT: &str =
    "https://weather-app.martindisch.workers.dev/api/measurements";

fn main() -> Result<()> {
    let measurement = measure()?;
    publish(&measurement)?;

    Ok(())
}

fn measure() -> Result<Measurement> {
    // The first reading after some time of inactivity tends to be off, so
    // discard it
    dht22_pi::read(PIN).ok();

    // Since about 30% of measurements fail, we just try until we succeed but
    // with an upper bound so we don't end up running forever
    for _ in 0..10 {
        // The sensor can't be read too frequently
        thread::sleep(Duration::from_secs(3));

        if let Ok(reading) = dht22_pi::read(PIN) {
            let timestamp =
                SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

            return Ok(Measurement {
                timestamp,
                temperature: reading.temperature,
                humidity: reading.humidity,
            });
        }
    }

    Err(eyre!("Unable to read sensor after 10 attempts"))
}

fn publish(measurement: &Measurement) -> Result<()> {
    let res = Client::new().post(ENDPOINT).json(&[measurement]).send()?;

    if res.status() != 201 {
        return Err(eyre!("Server responded with status {}", res.status()));
    }

    Ok(())
}

#[derive(Debug, PartialEq, Serialize)]
struct Measurement {
    timestamp: u64,
    temperature: f32,
    humidity: f32,
}
