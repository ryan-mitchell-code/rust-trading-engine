use crate::models::{Candle, Signal};

use super::Strategy;

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

/// Picks Buy, Sell, or Hold at random each bar (for experiments / baselines).
pub struct RandomStrategy {
    rng: StdRng,
}

impl RandomStrategy {
    pub fn new() -> Self {
        Self {
            rng: StdRng::from_entropy(),
        }
    }

    /// Seeded RNG for deterministic tests.
    #[cfg(test)]
    fn from_seed(seed: u64) -> Self {
        Self {
            rng: StdRng::seed_from_u64(seed),
        }
    }
}

impl Strategy for RandomStrategy {
    fn next(&mut self, _candle: &Candle) -> Signal {
        match self.rng.gen_range(0..3) {
            0 => Signal::Buy,
            1 => Signal::Sell,
            _ => Signal::Hold,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::models::Candle;

    use super::*;

    #[test]
    fn seeded_next_always_returns_valid_signal() {
        let mut s = RandomStrategy::from_seed(42);
        for _ in 0..50 {
            let sig = s.next(&Candle::test_close(1.0));
            assert!(matches!(
                sig,
                Signal::Buy | Signal::Sell | Signal::Hold
            ));
        }
    }
}
