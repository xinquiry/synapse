#[derive(Debug, thiserror::Error)]
pub enum CoreError {
    #[error("provider error: {0}")]
    Provider(#[from] synapse_provider::ProviderError),

    #[error("tool error: {0}")]
    Tool(#[from] synapse_tools::ToolError),

    #[error("permission error: {0}")]
    Permission(#[from] synapse_permissions::PermissionError),

    #[error("max iterations ({max}) reached")]
    MaxIterations { max: usize },

    #[error("{0}")]
    Config(String),
}
