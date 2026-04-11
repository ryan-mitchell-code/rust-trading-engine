use crate::models::{Candle, Signal};

use super::Strategy;

pub struct MovingAverage {
    short: usize,
    long: usize,
    prices: Vec<f64>,
    previous_short: Option<f64>,
    previous_long: Option<f64>,
}

impl MovingAverage {
    pub fn new(short: usize, long: usize) -> Self {
        Self {
            short,
            long,
            prices: Vec::new(),
            previous_short: None,
            previous_long: None,
        }
    }
}

impl Strategy for MovingAverage {
    fn next(&mut self, candle: &Candle) -> Signal {
        let price = candle.close;
        self.prices.push(price);

        if self.prices.len() < self.long {
            return Signal::Hold;
        }

        let short_avg: f64 = self
            .prices
            .iter()
            .rev()
            .take(self.short)
            .sum::<f64>()
            / self.short as f64;

        let long_avg: f64 = self
            .prices
            .iter()
            .rev()
            .take(self.long)
            .sum::<f64>()
            / self.long as f64;

        let mut signal = Signal::Hold;

        if let (Some(prev_s), Some(prev_l)) = (self.previous_short, self.previous_long) {
            // BUY: short crosses above long
            if prev_s <= prev_l && short_avg > long_avg {
                signal = Signal::Buy;
            }

            // SELL: short crosses below long
            if prev_s >= prev_l && short_avg < long_avg {
                signal = Signal::Sell;
            }
        }

        self.previous_short = Some(short_avg);
        self.previous_long = Some(long_avg);

        signal
    }
}

#[cfg(test)]
mod tests {
    use crate::models::Candle;

    use super::*;

    fn assert_hold(s: Signal) {
        assert!(matches!(s, Signal::Hold));
    }

    #[test]
    fn holds_while_insufficient_history() {
        let mut ma = MovingAverage::new(2, 5);
        for _ in 0..4 {
            assert_hold(ma.next(&Candle::test_close(100.0)));
        }
    }

    #[test]
    fn first_bar_at_long_length_still_hold_without_prior_averages() {
        let mut ma = MovingAverage::new(2, 3);
        assert_hold(ma.next(&Candle::test_close(10.0)));
        assert_hold(ma.next(&Candle::test_close(10.0)));
        // Third bar: enough points for averages, but no previous short/long yet
        assert_hold(ma.next(&Candle::test_close(10.0)));
    }

    #[test]
    fn produces_buy_on_short_crossing_above_long() {
        let mut ma = MovingAverage::new(2, 3);
        assert_hold(ma.next(&Candle::test_close(10.0)));
        assert_hold(ma.next(&Candle::test_close(10.0)));
        assert_hold(ma.next(&Candle::test_close(10.0)));
        let s = ma.next(&Candle::test_close(12.0));
        assert!(matches!(s, Signal::Buy));
    }

    #[test]
    fn signals_are_always_buy_sell_or_hold() {
        let mut ma = MovingAverage::new(2, 3);
        let prices = [10.0, 11.0, 12.0, 11.0, 10.0, 9.0, 8.0, 20.0, 21.0];
        for p in prices {
            let s = ma.next(&Candle::test_close(p));
            assert!(
                matches!(s, Signal::Buy | Signal::Sell | Signal::Hold),
                "unexpected signal variant"
            );
        }
    }
}
