mod error;
mod types;

pub use error::ProviderError;
pub use types::Message;
pub use types::MessageContent;
pub use types::Role;
pub use types::ToolCall;
pub use types::ToolResult;
pub use types::Usage;

/// A tool definition sent to the LLM.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ToolDefinition {
    pub name: String,
    pub description: String,
    pub input_schema: serde_json::Value,
}

/// Response from the LLM provider.
#[derive(Debug, Clone)]
pub struct ProviderResponse {
    pub content: Vec<MessageContent>,
    pub usage: Usage,
    pub stop_reason: StopReason,
}

/// Why the LLM stopped generating.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StopReason {
    EndTurn,
    ToolUse,
    MaxTokens,
    StopSequence,
    Other(String),
}

/// Trait for LLM providers.
#[async_trait::async_trait]
pub trait Provider: Send + Sync {
    /// Send a conversation to the LLM and receive a response.
    async fn complete(
        &self,
        system: &str,
        messages: &[Message],
        tools: &[ToolDefinition],
    ) -> Result<ProviderResponse, ProviderError>;

    /// Return the provider's display name.
    fn name(&self) -> &str;

    /// Return the model identifier being used.
    fn model(&self) -> &str;
}
