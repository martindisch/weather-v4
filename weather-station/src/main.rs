use csv::{ReaderBuilder, WriterBuilder};
use eyre::{eyre, Result};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::Path,
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

const PIN: u8 = 4;
const ENDPOINT: &str =
    "https://weather-app.martindisch.workers.dev/api/measurements";

fn main() -> Result<()> {
    let measurement = measure()?;

    let mut pending_path =
        home::home_dir().ok_or(eyre!("No home directory found"))?;
    pending_path.push(".pending_measurements.csv");
    let mut pending_measurements = read_pending(&pending_path)?;
    pending_measurements.push(measurement);

    match publish(&pending_measurements) {
        Ok(_) => {
            fs::remove_file(pending_path).ok();
            Ok(())
        }
        Err(e) => {
            append_pending(&measurement, &pending_path)?;
            Err(e)
        }
    }
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

fn read_pending(path: &Path) -> Result<Vec<Measurement>> {
    if !path.exists() {
        return Ok(vec![]);
    }

    let mut reader = ReaderBuilder::new().has_headers(false).from_path(path)?;
    let pending_measurements: Vec<Measurement> = reader
        .deserialize()
        .filter_map(|result| result.ok())
        .collect();

    Ok(pending_measurements)
}

fn publish(measurements: &[Measurement]) -> Result<()> {
    let res = Client::new().post(ENDPOINT).json(&measurements).send()?;

    if res.status() != 201 {
        return Err(eyre!("Server responded with status {}", res.status()));
    }

    println!("Published {} measurement(s)", measurements.len());

    Ok(())
}

fn append_pending(measurement: &Measurement, path: &Path) -> Result<()> {
    let mut writer =
        WriterBuilder::new().has_headers(false).from_writer(vec![]);
    writer.serialize(measurement)?;
    let row = String::from_utf8(writer.into_inner()?)?;

    let mut file = OpenOptions::new().create(true).append(true).open(path)?;
    write!(file, "{row}")?;

    Ok(())
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
struct Measurement {
    timestamp: u64,
    temperature: f32,
    humidity: f32,
}
