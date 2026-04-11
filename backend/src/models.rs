use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Candle {
    pub timestamp: String,
    pub close: f64,
}

#[cfg(test)]
impl Candle {
    pub(crate) fn test_close(close: f64) -> Self {
        Self {
            timestamp: "test".into(),
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