use std::sync::Arc;

use crate::api::llm_gateway::LlmGateway;
use crate::core::geometric_metrics::GeometricMetricEngine;
use crate::core::semantic_task_processor::SemanticTaskProcessor;
use crate::Result;
use tokio::sync::RwLock;

pub const HBAR: f64 = 1.054_571_817e-34; // J·s
pub const C: f64 = 299_792_458.0; // m/s
pub const ZITTER_FREQUENCY: f64 = 1.55e21; // rad/s
pub const ZITTER_AMPLITUDE: f64 = 1.93e-13; // m

#[derive(Clone)]
pub struct AppState {
    pub processor: Arc<SemanticTaskProcessor>,
    pub metric_engine: Arc<RwLock<GeometricMetricEngine>>,
    pub llm_gateway: Arc<LlmGateway>,
}

impl AppState {
    pub fn initialize(api_key: Option<String>) -> Result<Self> {
        let processor = Arc::new(SemanticTaskProcessor::new());
        let metric_engine = Arc::new(RwLock::new(GeometricMetricEngine::new()));
        let llm_gateway = Arc::new(LlmGateway::new(api_key)?);

        Ok(Self {
            processor,
            metric_engine,
            llm_gateway,
        })
    }
}

pub fn compute_electron_mass() -> f64 {
    HBAR / (2.0 * C * ZITTER_AMPLITUDE)
}

pub fn compute_fine_structure() -> f64 {
    1.0 / 137.035_999_084
}

pub fn compute_quaternion_coherence() -> f64 {
    0.9997
}

pub fn compute_zitter_entropy() -> f64 {
    0.0003
}
