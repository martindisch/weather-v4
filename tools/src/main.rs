use itertools::Itertools;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use eyre::Result;

fn main() -> Result<()> {
    let values = BufReader::new(File::open("history.csv")?)
        .lines()
        .filter_map(|r| r.ok())
        .map(|l| format!("({l})"));

    for chunk in &values.chunks(10_000) {
        let values_joined: String =
            itertools::Itertools::intersperse(chunk, ", ".into()).collect();

        println!("INSERT INTO measurements VALUES {values_joined};");
    }

    Ok(())
}
