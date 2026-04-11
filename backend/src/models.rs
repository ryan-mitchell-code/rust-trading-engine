use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Candle {
    pub timestamp: String,
    pub close: f64,
}

#[derive(Debug)]
pub enum Signal {
    Buy,
    Sell,
    Hold,
}