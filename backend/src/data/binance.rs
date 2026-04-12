//! Binance spot [`GET /api/v3/klines`](https://binance-docs.github.io/apidocs/spot/en/#kline-candlestick-data) client.

use crate::models::Candle;
use chrono::{SecondsFormat, TimeZone, Utc};
use serde_json::Value;

const KLINES_URL: &str = "https://api.binance.com/api/v3/klines";

fn json_f64(v: &Value) -> Result<f64, String> {
    v.as_f64()
        .or_else(|| v.as_str().and_then(|s| s.parse().ok()))
        .ok_or_else(|| "numeric field in kline".to_string())
}

/// Fetch OHLCV klines and map each bar to [`Candle`] (timestamp from open time ms, `close` only).
pub async fn fetch_klines(symbol: &str, interval: &str, limit: u16) -> Result<Vec<Candle>, String> {
    let client = reqwest::Client::new();
    let resp = client
        .get(KLINES_URL)
        .query(&[
            ("symbol", symbol),
            ("interval", interval),
            ("limit", &limit.to_string()),
        ])
        .send()
        .await
        .map_err(|e| e.to_string())?
        .error_for_status()
        .map_err(|e| e.to_string())?;
    let rows: Vec<Vec<Value>> = resp.json().await.map_err(|e| e.to_string())?;

    let mut candles = Vec::with_capacity(rows.len());
    for row in rows {
        let open_ms = row
            .first()
            .and_then(|v| v.as_i64().or_else(|| v.as_u64().map(|u| u as i64)))
            .ok_or_else(|| "open time in kline".to_string())?;
        let close = row
            .get(4)
            .ok_or_else(|| "close field in kline".to_string())
            .and_then(|v| json_f64(v))?;
        let timestamp = Utc
            .timestamp_millis_opt(open_ms)
            .single()
            .map(|dt| dt.to_rfc3339_opts(SecondsFormat::Secs, true))
            .unwrap_or_else(|| format!("{open_ms}"));
        candles.push(Candle { timestamp, close });
    }
    Ok(candles)
}
