#[derive(Debug, thiserror::Error)]
pub enum ProviderError {
    #[error("API request failed: {message}")]
    Api {
        message: String,
        status: Option<u16>,
    },

    #[error("rate limited, retry after {retry_after_ms}ms")]
    RateLimited { retry_after_ms: u64 },

    #[error("authentication failed: {0}")]
    Auth(String),

    #[error("response deserialization failed: {0}")]
    Deserialization(String),

    #[error(transparent)]
    Other(#[from] Box<dyn std::error::Error + Send + Sync>),
}
