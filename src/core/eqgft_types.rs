use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Fundamental fields in EQGFT v2.1
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EQGFTFields {
    /// Unit quaternion rotor field Q(x) = q_0(x) + i q_1(x) + j q_2(x) + k q_3(x)
    pub quaternion_field: QuaternionField,
    /// Derived Dirac spinor field
    pub dirac_spinor: DiracSpinor,
    /// U(1) gauge field
    pub gauge_field: GaugeField,
    /// Lorentzian metric
    pub metric: Metric,
}

/// Unit quaternion rotor field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuaternionField {
    pub q0: f64,
    pub q1: f64,
    pub q2: f64,
    pub q3: f64,
    /// Spatial coordinates (x, y, z, t)
    pub coordinates: [f64; 4],
}

/// Derived Dirac spinor field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiracSpinor {
    /// Spinor components (4 complex numbers)
    pub components: [num_complex::Complex<f64>; 4],
    /// Vacuum seed spinor
    pub vacuum_seed: [f64; 4],
}

/// U(1) gauge field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GaugeField {
    /// Gauge potential A_μ
    pub potential: [f64; 4],
    /// Field strength F_μν
    pub field_strength: [[f64; 4]; 4],
}

/// Lorentzian metric
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metric {
    /// Metric tensor g_μν
    pub tensor: [[f64; 4]; 4],
    /// Signature (-,+,+,+)
    pub signature: [i8; 4],
}

/// EQGFT v2.1 action terms
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EQGFTAction {
    /// Gravity term: (1/2κ) R
    pub gravity: f64,
    /// Quaternion kinetic term: (M²/2) Tr(∂_μ Q† ∂^μ Q)
    pub quaternion_kinetic: f64,
    /// Constraint term: λ(Q†Q−1)²
    pub constraint: f64,
    /// Fermion mass term: m_0 Re(Q)
    pub fermion_mass: f64,
    /// Geometric current term: J^μ_geom = (1/2) Tr(Q† ∂^μ Q − ∂^μ Q† Q)
    pub geometric_current: [f64; 4],
}

/// Visualization packet for EQGFT v2.1
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualizationPacket {
    /// Unique identifier for this visualization
    pub id: Uuid,
    /// Timestamp of creation
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// EQGFT fields data
    pub fields: EQGFTFields,
    /// Action terms
    pub action: EQGFTAction,
    /// Derived metrics
    pub metrics: HashMap<String, f64>,
    /// Visualization type (2D, 3D, animation, etc.)
    pub visualization_type: VisualizationType,
    /// Additional metadata
    pub metadata: serde_json::Value,
}

/// Type of visualization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VisualizationType {
    /// 2D plot
    Plot2D,
    /// 3D visualization
    Plot3D,
    /// Animation
    Animation,
    /// Topological visualization
    Topology,
    /// Custom visualization type
    Custom(String),
}

/// Request for visualization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualizationRequest {
    /// Type of visualization to generate
    pub visualization_type: VisualizationType,
    /// Parameters for the visualization
    pub parameters: serde_json::Value,
    /// Optional callback URL for async processing
    pub callback_url: Option<String>,
}

/// Response from visualization service
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualizationResponse {
    /// Status of the visualization request
    pub status: VisualizationStatus,
    /// URL or path to the generated visualization
    pub result_url: Option<String>,
    /// Error message if any
    pub error: Option<String>,
    /// Additional metadata
    pub metadata: serde_json::Value,
}

/// Status of visualization processing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VisualizationStatus {
    /// Visualization is queued for processing
    Queued,
    /// Visualization is being processed
    Processing,
    /// Visualization is complete and available
    Completed,
    /// Visualization failed
    Failed,
}

/// External tool integration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalTool {
    /// Unique identifier for the tool
    pub tool_id: String,
    /// Display name
    pub name: String,
    /// Type of tool (visualizer, converter, etc.)
    pub tool_type: String,
    /// Description of the tool
    pub description: String,
    /// Source URL or reference
    pub source_url: Option<String>,
    /// Entry point (command, URL, etc.)
    pub entry_point: String,
    /// Interface specification
    pub interface_spec: serde_json::Value,
    /// Instructions schema
    pub instructions_schema: serde_json::Value,
}

/// Registry of external tools
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolRegistry {
    /// List of available tools
    pub tools: Vec<ExternalTool>,
    /// Last updated timestamp
    pub last_updated: chrono::DateTime<chrono::Utc>,
}

impl Default for ToolRegistry {
    fn default() -> Self {
        let calc_plot = ExternalTool {
            tool_id: "calcplot".to_string(),
            name: "CalcPlot".to_string(),
            tool_type: "visualizer".to_string(),
            description: "Utility for plotting mathematical equations in 2D/3D".to_string(),
            source_url: Some("https://github.com/MonJamp/CalcPlot/tree/master".to_string()),
            entry_point: "python -m CalcPlot".to_string(),
            interface_spec: serde_json::json!({
                "input": {
                    "type": "object",
                    "properties": {
                        "equation": {"type": "string"},
                        "range": {"type": "string"},
                        "options": {
                            "type": "object",
                            "properties": {
                                "color": {"type": "string"},
                                "style": {"type": "string"}
                            }
                        }
                    },
                    "required": ["equation"]
                },
                "output": {
                    "type": "object",
                    "properties": {
                        "image_url": {"type": "string"},
                        "data": {"type": "object"}
                    }
                }
            }),
            instructions_schema: serde_json::json!("Plot equation: y = sin(x) over x ∈ [0, 2π]; Options: color='blue', style='dashed'"),
        };

        Self {
            tools: vec![calc_plot],
            last_updated: chrono::Utc::now(),
        }
    }
}
