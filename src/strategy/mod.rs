use crate::models::Signal;

pub trait Strategy {
    fn next(&mut self, price: f64) -> Signal;
}

pub mod moving_average;
pub mod random;

pub use self::{
    moving_average::MovingAverage,
    random::RandomStrategy,
};
