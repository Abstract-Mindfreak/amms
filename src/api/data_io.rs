use crate::core::error::{Error, Result};
use crate::core::types::{GeometricMetrics, SystemState};
use std::path::Path;

/// Very small persistence stub until real storage is implemented.
pub struct DataIoGateway;

impl DataIoGateway {
    pub fn load_latest_state(_base_path: &Path) -> Result<Option<SystemState>> {
        Ok(None)
    }

    pub fn persist_metrics(_base_path: &Path, _metrics: &GeometricMetrics) -> Result<()> {
        Err(Error::Io(std::io::Error::new(
            std::io::ErrorKind::Unsupported,
            "data persistence not implemented",
        )))
    }
}
