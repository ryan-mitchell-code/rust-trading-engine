use crate::models::{Candle, Signal};

use super::Strategy;

pub struct BuyAndHold {
    first: bool,
}

impl BuyAndHold {
    pub fn new() -> Self {
        Self { first: true }
    }
}

impl Strategy for BuyAndHold {
    fn next(&mut self, _candle: &Candle) -> Signal {
        if self.first {
            self.first = false;
            Signal::Buy
        } else {
            Signal::Hold
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::models::Candle;

    use super::*;

    #[test]
    fn buys_once_then_holds() {
        let mut s = BuyAndHold::new();
        assert!(matches!(s.next(&Candle::test_close(100.0)), Signal::Buy));
        assert!(matches!(s.next(&Candle::test_close(101.0)), Signal::Hold));
        assert!(matches!(s.next(&Candle::test_close(102.0)), Signal::Hold));
    }
}
