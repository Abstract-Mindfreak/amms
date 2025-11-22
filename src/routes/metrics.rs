use axum::{extract::State, Json};
use serde::Serialize;

use crate::state::AppState;

use super::{internal_error, ApiResult};

#[derive(Serialize)]
pub struct MetricsResponse {
    pub metrics: crate::core::types::GeometricMetrics,
    pub rule_names: Vec<String>,
    pub rule_count: usize,
}

pub async fn get_metrics(State(state): State<AppState>) -> ApiResult<Json<MetricsResponse>> {
    let metrics = state.processor.get_metrics().map_err(internal_error)?;
    let engine = state.metric_engine.read().await;
    let rule_names = engine.rule_names();
    let rule_count = rule_names.len();

    Ok(Json(MetricsResponse {
        metrics,
        rule_names,
        rule_count,
    }))
}

pub async fn get_vectorized_metrics(
    State(state): State<AppState>,
) -> ApiResult<Json<crate::core::types::GeometricMetrics>> {
    let metrics = state.processor.get_metrics().map_err(internal_error)?;
    Ok(Json(metrics))
}
