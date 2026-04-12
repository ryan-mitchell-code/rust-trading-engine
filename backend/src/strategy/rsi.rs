use crate::models::{Candle, Signal};

use super::Strategy;

/// Mean-reversion RSI strategy: buy when RSI is below `oversold`, sell when above `overbought`.
pub struct RsiStrategy {
    period: usize,
    overbought: f64,
    oversold: f64,
    prices: Vec<f64>,
}

impl RsiStrategy {
    pub fn new(period: usize, overbought: f64, oversold: f64) -> Self {
        Self {
            period,
            overbought,
            oversold,
            prices: Vec::new(),
        }
    }

    /// Simple average gain / average loss over the last `period` closes (needs `period + 1` prices).
    fn rsi(&self) -> Option<f64> {
        let p = self.period;
        if self.prices.len() < p + 1 {
            return None;
        }

        let n = self.prices.len();
        let mut sum_gain = 0.0_f64;
        let mut sum_loss = 0.0_f64;

        for i in (n - p - 1)..(n - 1) {
            let delta = self.prices[i + 1] - self.prices[i];
            if delta > 0.0 {
                sum_gain += delta;
            } else {
                sum_loss -= delta;
            }
        }

        let avg_gain = sum_gain / p as f64;
        let avg_loss = sum_loss / p as f64;

        if avg_gain < f64::EPSILON && avg_loss < f64::EPSILON {
            return None;
        }
        if avg_loss < f64::EPSILON {
            return Some(100.0);
        }
        if avg_gain < f64::EPSILON {
            return Some(0.0);
        }

        let rs = avg_gain / avg_loss;
        Some(100.0 - 100.0 / (1.0 + rs))
    }
}

impl Strategy for RsiStrategy {
    fn next(&mut self, candle: &Candle) -> Signal {
        self.prices.push(candle.close);

        let Some(rsi) = self.rsi() else {
            return Signal::Hold;
        };

        if rsi < self.oversold {
            Signal::Buy
        } else if rsi > self.overbought {
            Signal::Sell
        } else {
            Signal::Hold
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::models::Candle;

    use super::*;

    fn hold(s: Signal) -> bool {
        matches!(s, Signal::Hold)
    }

    #[test]
    fn holds_until_enough_bars() {
        let mut s = RsiStrategy::new(2, 70.0, 30.0);
        assert!(hold(s.next(&Candle::test_close(100.0))));
        assert!(hold(s.next(&Candle::test_close(100.0))));
    }

    #[test]
    fn oversold_triggers_buy() {
        let mut s = RsiStrategy::new(2, 70.0, 30.0);
        s.next(&Candle::test_close(100.0));
        s.next(&Candle::test_close(90.0));
        assert!(matches!(
            s.next(&Candle::test_close(80.0)),
            Signal::Buy
        ));
    }

    #[test]
    fn overbought_triggers_sell() {
        let mut s = RsiStrategy::new(2, 70.0, 30.0);
        s.next(&Candle::test_close(100.0));
        s.next(&Candle::test_close(110.0));
        assert!(matches!(
            s.next(&Candle::test_close(120.0)),
            Signal::Sell
        ));
    }
}
