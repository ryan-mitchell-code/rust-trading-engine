//! Shared backtest parameters for [`crate::arena::run_arena`].

/// Strategy parameters for a single arena run (moving average windows and RSI thresholds).
#[derive(Debug, Clone)]
pub struct RunConfig {
    pub ma_short: usize,
    pub ma_long: usize,
    pub rsi_period: usize,
    pub rsi_overbought: f64,
    pub rsi_oversold: f64,
}

impl RunConfig {
    /// Builds config from validated MA windows; RSI fields match historical hardcoded defaults (14 / 70 / 30).
    pub fn with_ma(ma_short: usize, ma_long: usize) -> Self {
        Self {
            ma_short,
            ma_long,
            rsi_period: 14,
            rsi_overbought: 70.0,
            rsi_oversold: 30.0,
        }
    }
}
