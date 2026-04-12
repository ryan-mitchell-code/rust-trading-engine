mod binance;

use crate::models::Candle;
use crate::paths;
use std::fs;
use tracing::info;

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

            if parts.len() >= 5 {
                Candle {
                    timestamp: parts[0].to_string(),
                    open: parts[1].parse::<f64>().unwrap(),
                    high: parts[2].parse::<f64>().unwrap(),
                    low: parts[3].parse::<f64>().unwrap(),
                    close: parts[4].parse::<f64>().unwrap(),
                }
            } else if parts.len() >= 2 {
                let close = parts[1].parse::<f64>().unwrap();
                Candle {
                    timestamp: parts[0].to_string(),
                    open: close,
                    high: close,
                    low: close,
                    close,
                }
            } else {
                panic!("CSV row needs at least timestamp and close columns");
            }
        })
        .collect()
}

/// Load candles from Binance, using `outputs/binance_cache_<symbol>_<interval>_<limit>.json` when present.
pub async fn load_from_binance(symbol: &str, interval: &str, limit: u16) -> Result<Vec<Candle>, String> {
    let cache_path = paths::binance_cache_file(symbol, interval, limit);
    if cache_path.exists() {
        info!(
            %symbol,
            %interval,
            limit,
            path = %cache_path.display(),
            "loading candles from cache"
        );
        let text = fs::read_to_string(&cache_path).map_err(|e| e.to_string())?;
        let candles = candles_from_cache_json(&text)?;
        info!(bars = candles.len(), "loaded candles from cache");
        return Ok(candles);
    }

    info!(
        %symbol,
        %interval,
        limit,
        "cache miss; fetching from Binance"
    );
    let candles = fetch_klines(symbol, interval, limit).await?;
    let json = serde_json::to_string_pretty(&candles).map_err(|e| e.to_string())?;
    if let Some(parent) = cache_path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    fs::write(&cache_path, json).map_err(|e| e.to_string())?;
    info!(
        bars = candles.len(),
        path = %cache_path.display(),
        "wrote Binance response to cache"
    );
    Ok(candles)
}

#[derive(serde::Deserialize)]
struct LegacyCandle {
    timestamp: String,
    close: f64,
}

/// Binance cache may be legacy `{ timestamp, close }` only; upgrade to OHLC using `close` for all fields.
fn candles_from_cache_json(text: &str) -> Result<Vec<Candle>, String> {
    if let Ok(candles) = serde_json::from_str::<Vec<Candle>>(text) {
        return Ok(candles);
    }
    let legacy: Vec<LegacyCandle> = serde_json::from_str(text).map_err(|e| e.to_string())?;
    Ok(legacy
        .into_iter()
        .map(|c| Candle {
            timestamp: c.timestamp,
            open: c.close,
            high: c.close,
            low: c.close,
            close: c.close,
        })
        .collect())
}
