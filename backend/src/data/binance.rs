//! Binance spot [`GET /api/v3/klines`](https://binance-docs.github.io/apidocs/spot/en/#kline-candlestick-data) client.

use crate::models::Candle;
use chrono::{SecondsFormat, TimeZone, Utc};
use serde_json::Value;

const KLINES_URL: &str = "https://api.binance.com/api/v3/klines";

fn json_f64(v: &Value) -> f64 {
    v.as_f64()
        .or_else(|| v.as_str().and_then(|s| s.parse().ok()))
        .expect("numeric field in kline")
}

/// Fetch OHLCV klines and map each bar to [`Candle`] (timestamp from open time ms, `close` only).
pub async fn fetch_klines(symbol: &str, interval: &str, limit: u16) -> Vec<Candle> {
    let client = reqwest::Client::new();
    let rows: Vec<Vec<Value>> = client
        .get(KLINES_URL)
        .query(&[
            ("symbol", symbol),
            ("interval", interval),
            ("limit", &limit.to_string()),
        ])
        .send()
        .await
        .expect("binance klines request")
        .error_for_status()
        .expect("binance klines status")
        .json()
        .await
        .expect("binance klines json");

    rows.into_iter()
        .map(|row| {
            let open_ms = row
                .first()
                .and_then(|v| v.as_i64().or_else(|| v.as_u64().map(|u| u as i64)))
                .expect("open time");
            // Indices 1–4: open, high, low, close (strings or numbers). Candle keeps close only.
            let close = json_f64(&row[4]);
            let timestamp = Utc
                .timestamp_millis_opt(open_ms)
                .single()
                .map(|dt| dt.to_rfc3339_opts(SecondsFormat::Secs, true))
                .unwrap_or_else(|| format!("{open_ms}"));
            Candle {
                timestamp,
                close,
            }
        })
        .collect()
}
