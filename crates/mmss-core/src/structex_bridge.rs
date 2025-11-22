use serde_json::Value as JsonValue;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PatternError {
    #[error("Pattern compilation failed: {0}")]
    CompileError(String),
    #[error("Matching error: {0}")]
    MatchError(String),
}

pub struct PatternMatcher {
    pattern: String,
}

impl PatternMatcher {
    pub fn new(pattern: &str) -> Result<Self, PatternError> {
        Ok(Self {
            pattern: pattern.to_string(),
        })
    }

    pub fn matches(&self, _record: &MmssRecord) -> Result<bool, PatternError> {
        Ok(true)
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct MmssRecord {
    pub id: u64,
    pub kind: String,
    pub timestamp: i64,
    pub payload: JsonValue,
}
