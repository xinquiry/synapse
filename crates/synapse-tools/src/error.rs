#[derive(Debug, thiserror::Error)]
pub enum ToolError {
    #[error("tool execution failed: {0}")]
    Execution(String),

    #[error("invalid input: {0}")]
    InvalidInput(String),

    #[error("permission denied: {0}")]
    PermissionDenied(String),

    #[error("tool timed out after {timeout_ms}ms")]
    Timeout { timeout_ms: u64 },

    #[error(transparent)]
    Other(#[from] Box<dyn std::error::Error + Send + Sync>),
}
