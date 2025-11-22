//! Placeholder visualization protocol module.

use crate::core::types::{GeometricMetrics, SemanticAnchor};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualizationPacket {
    pub metrics: GeometricMetrics,
    pub anchors: Vec<SemanticAnchor>,
}

impl VisualizationPacket {
    pub fn new(metrics: GeometricMetrics, anchors: Vec<SemanticAnchor>) -> Self {
        Self { metrics, anchors }
    }
}
