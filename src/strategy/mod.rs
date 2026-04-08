use crate::models::Signal;

pub trait Strategy {
    fn next(&mut self, price: f64) -> Signal;
}

pub mod buy_and_hold;
pub mod moving_average;
pub mod random;

pub use self::{
    buy_and_hold::BuyAndHold,
    moving_average::MovingAverage,
    random::RandomStrategy,
};
