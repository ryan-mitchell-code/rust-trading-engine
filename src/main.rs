mod csv;
mod data;
mod engine;
mod models;
mod strategy;

fn main() {
    let candles = data::load_csv("data/formatted_btc.csv");
    engine::run(candles);
}