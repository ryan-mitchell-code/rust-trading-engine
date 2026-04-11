mod binance;

use crate::models::Candle;
use crate::paths;
use std::fs;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub use binance::fetch_klines;

/// Local CSV candles (e.g. ad-hoc scripts); `main` uses [`load_from_binance`] with file cache.
#[allow(dead_code)]
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

/// Load candles from Binance, using `outputs/binance_cache_<symbol>_<interval>_<limit>.json` when present.
pub async fn load_from_binance(symbol: &str, interval: &str, limit: u16) -> Vec<Candle> {
    let cache_path = paths::binance_cache_file(symbol, interval, limit);
    if cache_path.exists() {
        let text = fs::read_to_string(&cache_path).expect("read binance cache");
        return serde_json::from_str(&text).expect("parse binance cache");
    }

    let candles = fetch_klines(symbol, interval, limit).await;
    let json = serde_json::to_string_pretty(&candles).expect("serialize binance cache");
    if let Some(parent) = cache_path.parent() {
        fs::create_dir_all(parent).expect("create outputs dir");
    }
    fs::write(&cache_path, json).expect("write binance cache");
    candles
}
