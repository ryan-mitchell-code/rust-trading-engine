use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Candle {
    pub timestamp: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
}

#[cfg(test)]
impl Candle {
    pub(crate) fn test_close(close: f64) -> Self {
        Self {
            timestamp: "test".into(),
            open: close,
            high: close,
            low: close,
            close,
        }
    }
}

#[derive(Debug)]
pub enum Signal {
    Buy,
    Sell,
    Hold,
}