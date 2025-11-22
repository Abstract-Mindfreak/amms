pub mod core {
    pub mod emergence_logic;
    pub mod eqgft_types;
    pub mod error;
    pub mod geometric_metrics;
    pub mod geometric_quaternion_core;
    pub mod semantic_task_processor;
    pub mod types;
    
    // Re-export commonly used types
    pub use eqgft_types::{
        EQGFTFields, QuaternionField, DiracSpinor, GaugeField, Metric, 
        EQGFTAction, VisualizationPacket, VisualizationType, 
        VisualizationRequest, VisualizationResponse, VisualizationStatus,
        ExternalTool, ToolRegistry
    };
}

pub mod api {
    pub mod data_io;
    pub mod llm_gateway;
}

pub mod visualization {
    pub mod protocol;
}

pub mod routes;
pub mod state;

pub use crate::core::error::{Error, Result};
pub use crate::core::types::*;
