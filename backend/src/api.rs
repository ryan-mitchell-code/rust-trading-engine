//! Minimal HTTP API: `POST /run` triggers [`crate::data::load_from_binance`] + [`crate::arena::run_arena`].

use axum::http::Method;
use axum::routing::post;
use axum::{Json, response::IntoResponse, Router};
use http::StatusCode;
use serde::Deserialize;
use tower_http::cors::{Any, CorsLayer};
use tracing::{error, info};

use crate::arena;
use crate::data;

const DEFAULT_LIMIT: u16 = 1000;

#[derive(Deserialize)]
pub struct RunRequest {
    pub dataset: String,
    pub interval: String,
}

pub async fn serve() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let app = Router::new()
        .route("/run", post(run_handler))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods([Method::POST, Method::OPTIONS])
                .allow_headers(Any),
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("bind listener");
    info!("listening on http://0.0.0.0:3000");
    axum::serve(listener, app).await.expect("serve");
}

async fn run_handler(Json(req): Json<RunRequest>) -> impl IntoResponse {
    info!(
        dataset = %req.dataset,
        interval = %req.interval,
        "POST /run"
    );
    match data::load_from_binance(&req.dataset, &req.interval, DEFAULT_LIMIT).await {
        Ok(candles) => {
            info!(bars = candles.len(), "running backtest");
            let export = arena::run_arena(&candles, false);
            info!(
                strategies = export.results.len(),
                "backtest finished"
            );
            (StatusCode::OK, Json(export)).into_response()
        }
        Err(e) => {
            error!(error = %e, "failed to load market data");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "failed to load data"
                })),
            )
                .into_response()
        }
    }
}
