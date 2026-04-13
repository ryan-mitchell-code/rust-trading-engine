//! Apply deferred strategy signals to cash, positions, and metrics.
//! The orchestration loop supplies fill price and bar timestamps; slippage can adjust fills here later.

use super::BacktestParams;
use super::OpenPosition;
use crate::metrics::Metrics;
use crate::models::Signal;

pub(crate) fn signal_verb(signal: &Signal) -> &'static str {
    match signal {
        Signal::Buy => "BUY",
        Signal::Sell => "SELL",
        Signal::Hold => "HOLD",
    }
}

/// Apply a **deferred** signal at `fill_price` (e.g. current bar **open**). Log rows use `trade_timestamp`.
pub(crate) fn execute_signal(
    signal: Signal,
    fill_price: f64,
    trade_timestamp: &str,
    cash: &mut f64,
    position: &mut Option<OpenPosition>,
    open_trade_id: &mut Option<u32>,
    next_trade_id: &mut u32,
    metrics: &mut Metrics,
    params: &BacktestParams,
) -> (
    Option<(u32, String, f64)>,
    Option<(u32, String, f64, f64)>,
) {
    let mut buy_log: Option<(u32, String, f64)> = None;
    let mut sell_log: Option<(u32, String, f64, f64)> = None;

    match signal {
        Signal::Buy => {
            if position.is_none() {
                let allocation = *cash * params.position_fraction;

                if allocation > f64::EPSILON {
                    let buy_fee = allocation * params.fee_rate;
                    let cash_out = allocation + buy_fee;
                    if *cash + f64::EPSILON >= cash_out {
                        let trade_id = *next_trade_id;
                        *next_trade_id += 1;
                        *open_trade_id = Some(trade_id);

                        let size = allocation / fill_price;
                        *cash -= cash_out;
                        *position = Some(OpenPosition {
                            entry_price: fill_price,
                            size,
                            allocation,
                            buy_fee,
                        });

                        buy_log = Some((trade_id, trade_timestamp.to_string(), fill_price));
                    }
                }
            }
        }
        Signal::Sell => {
            if let Some(OpenPosition {
                size,
                allocation,
                buy_fee,
                ..
            }) = position.take()
            {
                let exit_price = fill_price;
                let proceeds = size * exit_price;
                let sell_fee = proceeds * params.fee_rate;
                let net_proceeds = proceeds - sell_fee;
                let pnl = net_proceeds - allocation - buy_fee;

                *cash += net_proceeds;
                metrics.record_trade(pnl);

                let trade_id = open_trade_id
                    .take()
                    .expect("sell should follow a logged buy");

                sell_log = Some((trade_id, trade_timestamp.to_string(), exit_price, pnl));
            }
        }
        Signal::Hold => {}
    }

    (buy_log, sell_log)
}
