use eyre::{eyre, Result};
use itertools::Itertools;
use std::{
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, BufWriter, Write},
    mem,
    process::Command,
    time::Instant,
};
use tempdir::TempDir;

const DB: &str = "weather-test";

fn main() -> Result<()> {
    let values = BufReader::new(File::open("history.csv")?)
        .lines()
        .filter_map(|r| r.ok())
        .map(|l| format!("({l})"));

    let dir = TempDir::new("weather_migration")?;
    let file_path = dir.path().join("command.sql");
    println!("Using temporary file {file_path:?} for SQL commands");

    let start = Instant::now();
    for chunk in &values.chunks(10_000) {
        let values_joined: String =
            itertools::Itertools::intersperse(chunk, ", ".into()).collect();

        let mut writer = BufWriter::new(
            OpenOptions::new()
                .write(true)
                .create(true)
                .open(&file_path)?,
        );
        writeln!(writer, "INSERT INTO measurements VALUES {values_joined};")?;
        mem::drop(writer);

        let output = Command::new("wrangler")
            .arg("d1")
            .arg("execute")
            .arg(DB)
            .arg("--file")
            .arg(file_path.to_str().ok_or(eyre!("Path not unicode"))?)
            .output()?;
        if !output.status.success() {
            eprintln!("{}", String::from_utf8_lossy(&output.stdout));
            return Err(eyre!("Command failed"));
        }

        println!(
            "Inserted another batch of values {:?} after start",
            start.elapsed()
        );
    }

    Ok(())
}
