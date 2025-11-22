"""
MMSS Visualization Module for EQGFT v2.1

This module provides visualization capabilities for the Effective Quaternion Geometric Field Theory (EQGFT) v2.1
implementation in the MMSS system. It includes 2D/3D plotting, topological visualization, and animation tools.
"""

__version__ = "0.1.0"

from .core import (
    Visualizer2D,
    Visualizer3D,
    TopologyVisualizer,
    AnimationRenderer,
    render_visualization,
    create_visualization_packet,
    save_visualization,
    load_visualization,
)

from .models import (
    QuaternionField,
    DiracSpinor,
    GaugeField,
    Metric,
    EQGFTFields,
    EQGFTAction,
    VisualizationPacket,
    VisualizationType,
    VisualizationRequest,
    VisualizationResponse,
    VisualizationStatus,
)

# Set up logging
import logging

logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s - %(name)s - %(levelname)s - %(message)s'
)
logger = logging.getLogger(__name__)

# Initialize default visualizers
visualizer_2d = Visualizer2D()
visualizer_3d = Visualizer3D()
topology_viz = TopologyVisualizer()
animation_renderer = AnimationRenderer()

def visualize(packet: VisualizationPacket, viz_type: str = "3d", **kwargs):
    """
    Main visualization function that routes to the appropriate visualizer.
    
    Args:
        packet: VisualizationPacket containing EQGFT data
        viz_type: Type of visualization ("2d", "3d", "topology", "animation")
        **kwargs: Additional arguments for the specific visualizer
        
    Returns:
        Visualization result (plot, figure, or animation)
    """
    viz_type = viz_type.lower()
    
    if viz_type == "2d":
        return visualizer_2d.render(packet, **kwargs)
    elif viz_type == "3d":
        return visualizer_3d.render(packet, **kwargs)
    elif viz_type == "topology":
        return topology_viz.render(packet, **kwargs)
    elif viz_type == "animation":
        return animation_renderer.render(packet, **kwargs)
    else:
        raise ValueError(f"Unknown visualization type: {viz_type}")

__all__ = [
    # Core visualization
    'Visualizer2D',
    'Visualizer3D',
    'TopologyVisualizer',
    'AnimationRenderer',
    'visualize',
    'render_visualization',
    'create_visualization_packet',
    'save_visualization',
    'load_visualization',
    
    # Data models
    'QuaternionField',
    'DiracSpinor',
    'GaugeField',
    'Metric',
    'EQGFTFields',
    'EQGFTAction',
    'VisualizationPacket',
    'VisualizationType',
    'VisualizationRequest',
    'VisualizationResponse',
    'VisualizationStatus',
]
