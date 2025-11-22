"""
Data models for EQGFT v2.1 visualization.

This module defines the Python data models that correspond to the Rust types
defined in the core MMSS system for EQGFT v2.1.
"""

from typing import Dict, List, Optional, Union, Any
from enum import Enum
from datetime import datetime
from pydantic import BaseModel, Field
import numpy as np
import json

class QuaternionField(BaseModel):
    """Represents a unit quaternion rotor field Q(x) = q_0(x) + i q_1(x) + j q_2(x) + k q_3(x)"""
    q0: float = Field(..., description="Real component (q_0)")
    q1: float = Field(..., description="First imaginary component (i)")
    q2: float = Field(..., description="Second imaginary component (j)")
    q3: float = Field(..., description="Third imaginary component (k)")
    coordinates: List[float] = Field(..., description="Spatial coordinates [x, y, z, t]")
    
    def to_numpy(self) -> np.ndarray:
        """Convert to numpy array [q0, q1, q2, q3]"""
        return np.array([self.q0, self.q1, self.q2, self.q3])
    
    @classmethod
    def from_numpy(cls, arr: np.ndarray, coordinates: List[float] = None) -> 'QuaternionField':
        """Create from numpy array [q0, q1, q2, q3]"""
        if coordinates is None:
            coordinates = [0.0, 0.0, 0.0, 0.0]
        return cls(
            q0=float(arr[0]),
            q1=float(arr[1]),
            q2=float(arr[2]),
            q3=float(arr[3]),
            coordinates=coordinates
        )

class DiracSpinor(BaseModel):
    """Represents a Dirac spinor with 4 complex components"""
    components: List[complex] = Field(..., description="4 complex spinor components")
    vacuum_seed: List[float] = Field(..., description="Vacuum seed spinor (4 real values)")
    
    def to_numpy(self) -> np.ndarray:
        """Convert to numpy array of complex numbers"""
        return np.array(self.components, dtype=np.complex128)
    
    @classmethod
    def from_numpy(cls, arr: np.ndarray, vacuum_seed: List[float] = None) -> 'DiracSpinor':
        """Create from numpy array of complex numbers"""
        if vacuum_seed is None:
            vacuum_seed = [1.0, 0.0, 0.0, 0.0]  # Default vacuum state
        return cls(
            components=[complex(x) for x in arr],
            vacuum_seed=vacuum_seed
        )

class GaugeField(BaseModel):
    """Represents a U(1) gauge field with potential and field strength"""
    potential: List[float] = Field(..., description="Gauge potential A_μ (4 components)")
    field_strength: List[List[float]] = Field(..., description="Field strength F_μν (4x4 antisymmetric tensor)")
    
    def to_numpy(self) -> tuple[np.ndarray, np.ndarray]:
        """Convert to numpy arrays (potential, field_strength)"""
        return np.array(self.potential), np.array(self.field_strength)

class Metric(BaseModel):
    """Represents a Lorentzian metric tensor"""
    tensor: List[List[float]] = Field(..., description="Metric tensor g_μν (4x4)")
    signature: List[int] = Field(..., description="Signature of the metric, e.g., [-1, 1, 1, 1]")
    
    def to_numpy(self) -> np.ndarray:
        """Convert to numpy array"""
        return np.array(self.tensor)

class EQGFTFields(BaseModel):
    """Container for all EQGFT v2.1 fundamental fields"""
    quaternion_field: QuaternionField
    dirac_spinor: DiracSpinor
    gauge_field: GaugeField
    metric: Metric
    
    class Config:
        json_encoders = {
            np.ndarray: lambda v: v.tolist(),
            complex: lambda v: {"real": v.real, "imag": v.imag}
        }

class EQGFTAction(BaseModel):
    """Action terms for EQGFT v2.1"""
    gravity: float = Field(..., description="(1/2κ) R")
    quaternion_kinetic: float = Field(..., description="(M²/2) Tr(∂_μ Q† ∂^μ Q)")
    constraint: float = Field(..., description="λ(Q†Q−1)²")
    fermion_mass: float = Field(..., description="m_0 Re(Q)")
    geometric_current: List[float] = Field(..., description="J^μ_geom = (1/2) Tr(Q† ∂^μ Q − ∂^μ Q† Q)")

class VisualizationType(str, Enum):
    """Types of visualizations supported"""
    PLOT_2D = "2d"
    PLOT_3D = "3d"
    ANIMATION = "animation"
    TOPOLOGY = "topology"
    CUSTOM = "custom"

class VisualizationStatus(str, Enum):
    """Status of visualization processing"""
    QUEUED = "queued"
    PROCESSING = "processing"
    COMPLETED = "completed"
    FAILED = "failed"

class VisualizationPacket(BaseModel):
    """Container for visualization data and metadata"""
    id: str
    timestamp: datetime
    fields: EQGFTFields
    action: EQGFTAction
    metrics: Dict[str, float]
    visualization_type: VisualizationType
    metadata: Dict[str, Any] = {}
    
    def to_json(self, **kwargs) -> str:
        """Serialize to JSON string"""
        return self.json(**kwargs)
    
    @classmethod
    def from_json(cls, json_str: str) -> 'VisualizationPacket':
        """Deserialize from JSON string"""
        return cls.parse_raw(json_str)

class VisualizationRequest(BaseModel):
    """Request for visualization"""
    visualization_type: VisualizationType
    parameters: Dict[str, Any] = {}
    callback_url: Optional[str] = None

class VisualizationResponse(BaseModel):
    """Response from visualization service"""
    status: VisualizationStatus
    result_url: Optional[str] = None
    error: Optional[str] = None
    metadata: Dict[str, Any] = {}

# Utility functions
def save_visualization(packet: VisualizationPacket, filepath: str) -> None:
    """Save visualization packet to a JSON file"""
    with open(filepath, 'w') as f:
        f.write(packet.json(indent=2))

def load_visualization(filepath: str) -> VisualizationPacket:
    """Load visualization packet from a JSON file"""
    with open(filepath, 'r') as f:
        return VisualizationPacket.parse_raw(f.read())
