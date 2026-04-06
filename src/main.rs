mod csv;
mod data;
mod engine;
mod models;
mod strategy;

use engine::run;
use strategy::{MovingAverage, RandomStrategy};

fn main() {
    let candles = data::load_csv("data/formatted_btc.csv");

    run(
        &candles,
        MovingAverage::new(5, 20),
        "moving_average_5_20",
    );
    run(&candles, RandomStrategy::new(), "random");
}
