pub struct Metrics {
    trades: u32,
    wins: u32,
    total_pnl: f64,
    peak_equity: f64,
    max_drawdown: f64,
}

impl Metrics {
    pub fn new(starting_capital: f64) -> Self {
        Self {
            trades: 0,
            wins: 0,
            total_pnl: 0.0,
            peak_equity: starting_capital,
            max_drawdown: 0.0,
        }
    }

    pub fn record_trade(&mut self, pnl: f64) {
        self.trades += 1;
        self.total_pnl += pnl;
        if pnl > 0.0 {
            self.wins += 1;
        }
    }

    pub fn trades(&self) -> u32 {
        self.trades
    }

    pub fn total_pnl(&self) -> f64 {
        self.total_pnl
    }

    pub fn win_rate(&self) -> f64 {
        if self.trades == 0 {
            0.0
        } else {
            (self.wins as f64 / self.trades as f64) * 100.0
        }
    }

    pub fn avg_pnl(&self) -> f64 {
        if self.trades == 0 {
            0.0
        } else {
            self.total_pnl / self.trades as f64
        }
    }

    pub fn update_equity(&mut self, equity: f64) {
        if equity > self.peak_equity {
            self.peak_equity = equity;
        }

        let drawdown = (self.peak_equity - equity) / self.peak_equity;
        if drawdown > self.max_drawdown {
            self.max_drawdown = drawdown;
        }
    }

    pub fn peak_equity(&self) -> f64 {
        self.peak_equity
    }

    pub fn max_drawdown(&self) -> f64 {
        self.max_drawdown
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPS: f64 = 1e-12;

    fn assert_close(a: f64, b: f64) {
        assert!((a - b).abs() < EPS, "expected {}, got {}", b, a);
    }

    #[test]
    fn starts_with_zero_drawdown() {
        let m = Metrics::new(10_000.0);
        assert_close(m.peak_equity(), 10_000.0);
        assert_close(m.max_drawdown(), 0.0);
    }

    #[test]
    fn updates_peak_and_drawdown() {
        let mut m = Metrics::new(100.0);
        m.update_equity(120.0);
        m.update_equity(90.0);

        assert_close(m.peak_equity(), 120.0);
        assert_close(m.max_drawdown(), 0.25); // (120 - 90) / 120
    }

    #[test]
    fn keeps_max_drawdown_across_recoveries() {
        let mut m = Metrics::new(100.0);
        m.update_equity(80.0); // 20%
        m.update_equity(95.0); // smaller drawdown than max
        m.update_equity(150.0); // new peak resets current drawdown, not max
        m.update_equity(120.0); // 20% from new peak

        assert_close(m.max_drawdown(), 0.20);
    }
}
