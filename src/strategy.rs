use crate::models::{Signal};

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

    pub fn next(&mut self, price: f64) -> Signal {
        self.prices.push(price);

        if self.prices.len() < self.long {
            return Signal::Hold;
        }

        let short_avg: f64 = self.prices
            .iter()
            .rev()
            .take(self.short)
            .sum::<f64>() / self.short as f64;

        let long_avg: f64 = self.prices
            .iter()
            .rev()
            .take(self.long)
            .sum::<f64>() / self.long as f64;

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