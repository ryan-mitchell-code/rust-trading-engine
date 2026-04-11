use crate::models::{Candle, Signal};

pub trait Strategy {
    fn next(&mut self, candle: &Candle) -> Signal;
}

pub mod buy_and_hold;
pub mod moving_average;
pub mod random;

pub use self::{
    buy_and_hold::BuyAndHold,
    moving_average::MovingAverage,
    random::RandomStrategy,
};
