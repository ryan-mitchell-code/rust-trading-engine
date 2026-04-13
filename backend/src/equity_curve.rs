//! Stateless analytics on a finished **equity curve** (one value per bar).
//!
//! This is separate from [`crate::metrics::Metrics`], which **accumulates** trade stats and
//! peak drawdown **while** the engine steps bars. Here we only transform a `&[f64]` curve
//! into derived series or scalars (Sharpe, per-bar drawdown path).

/// Per-period (sample) Sharpe ratio from an equity curve — **timeframe-agnostic**.
///
/// For each consecutive pair of equity values, computes simple return  
/// `(E_t - E_{t-1}) / E_{t-1}`, then returns `mean(returns) / sample_std_dev(returns)`
/// (sample variance uses `n - 1` in the denominator).
///
/// **Semantics**
///
/// - This is a **per-period** Sharpe: the period is whatever spacing the curve has (bars,
///   ticks, etc.). It does **not** assume daily, hourly, or any fixed calendar.
/// - It is **not annualized**. Converting to an annualized Sharpe requires explicit
///   assumptions (e.g. periods per year) and should be done outside this function.
/// - Raw values are **not directly comparable** across backtests that use different bar
///   spacing unless you normalize or annualize consistently.
///
/// Returns `0.0` when there are fewer than two returns, or when sample std dev is negligible.
/// Steps where the prior equity is non-positive or non-finite are skipped (avoids divide-by-zero and `inf`).
pub fn sharpe_ratio_from_equity_curve(equity_curve: &[f64]) -> f64 {
    if equity_curve.len() < 2 {
        return 0.0;
    }

    let mut returns: Vec<f64> = Vec::with_capacity(equity_curve.len() - 1);

    for w in equity_curve.windows(2) {
        let prev = w[0];
        let curr = w[1];

        if prev <= 0.0 || !prev.is_finite() || !curr.is_finite() {
            continue;
        }
        let step = (curr - prev) / prev;
        if step.is_finite() {
            returns.push(step);
        }
    }

    let n = returns.len();
    if n < 2 {
        return 0.0;
    }

    let mean = returns.iter().sum::<f64>() / n as f64;

    let variance = returns
        .iter()
        .map(|r| (r - mean).powi(2))
        .sum::<f64>()
        / (n as f64 - 1.0);

    let std_dev = variance.sqrt();

    if std_dev <= f64::EPSILON {
        0.0
    } else {
        mean / std_dev
    }
}

/// Per-timestep drawdown vs running peak: `(equity - peak) / peak` (0 at new highs, negative underwater).
/// Same length as `equity_curve`; bar times align with the series that produced the curve.
pub fn drawdown_curve_from_equity(equity_curve: &[f64]) -> Vec<f64> {
    if equity_curve.is_empty() {
        return Vec::new();
    }
    let mut peak = equity_curve[0];
    let mut out = Vec::with_capacity(equity_curve.len());
    for &eq in equity_curve {
        peak = peak.max(eq);
        let drawdown = if peak.abs() > f64::EPSILON {
            (eq - peak) / peak
        } else {
            0.0
        };
        out.push(drawdown);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPS: f64 = f64::EPSILON * 10.0;

    fn assert_close(a: f64, b: f64) {
        assert!(
            (a - b).abs() < EPS,
            "expected {:.12}, got {:.12}",
            b,
            a
        );
    }

    #[test]
    fn sharpe_ratio_flat_equity_is_zero() {
        let curve = vec![10_000.0, 10_000.0, 10_000.0];
        assert_close(sharpe_ratio_from_equity_curve(&curve), 0.0);
    }

    #[test]
    fn drawdown_curve_empty_is_empty() {
        assert!(drawdown_curve_from_equity(&[]).is_empty());
    }

    #[test]
    fn drawdown_curve_peaks_zero_underwater_negative() {
        let equity = vec![10_000.0, 11_000.0, 9_900.0, 12_000.0];
        let dd = drawdown_curve_from_equity(&equity);
        assert_close(dd[0], 0.0);
        assert_close(dd[1], 0.0);
        assert_close(dd[2], (9_900.0 - 11_000.0) / 11_000.0);
        assert_close(dd[3], 0.0);
    }

    #[test]
    fn sharpe_ratio_two_varying_returns() {
        // returns ≈ 1% and -0.5% → mean ≈ 0.25%, sample std > 0 → finite ratio
        let curve = vec![10_000.0, 10_100.0, 10_049.5];
        let s = sharpe_ratio_from_equity_curve(&curve);
        assert!(s.is_finite() && s > 0.0);
    }

    #[test]
    fn sharpe_ratio_skips_non_positive_prior_equity() {
        let curve = vec![0.0, 10_000.0, 10_100.0];
        assert_close(sharpe_ratio_from_equity_curve(&curve), 0.0);
    }
}
