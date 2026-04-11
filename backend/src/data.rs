use crate::models::Candle;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn load_csv<P: AsRef<Path>>(path: P) -> Vec<Candle> {
    let file = File::open(path).expect("Cannot open file");
    let reader = BufReader::new(file);

    reader
        .lines()
        .skip(1)
        .map(|line| {
            let line = line.unwrap();
            let parts: Vec<&str> = line.split(',').collect();

            Candle {
                timestamp: parts[0].to_string(),
                close: parts[4].parse::<f64>().unwrap(),
            }
        })
        .collect()
}