#[derive(Debug, thiserror::Error)]
pub enum PermissionError {
    #[error("permission denied: {reason}")]
    Denied { reason: String },

    #[error("permission request timed out")]
    Timeout,
}
