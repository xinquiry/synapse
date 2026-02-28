mod error;

pub use error::CoreError;

pub use synapse_permissions::Decision;
pub use synapse_permissions::PermissionRequest;
pub use synapse_provider::Message;
pub use synapse_provider::MessageContent;
pub use synapse_provider::Provider;
pub use synapse_provider::Role;
pub use synapse_provider::ToolCall;
pub use synapse_provider::ToolResult;
pub use synapse_provider::Usage;
pub use synapse_tools::Tool;
pub use synapse_tools::ToolOutput;

/// Configuration for the agent.
#[derive(Debug, Clone)]
pub struct AgentConfig {
    /// Maximum number of ReAct loop iterations before stopping.
    pub max_iterations: usize,
    /// Model identifier to use.
    pub model: String,
}

impl Default for AgentConfig {
    fn default() -> Self {
        Self {
            max_iterations: 50,
            model: String::from("claude-sonnet-4-20250514"),
        }
    }
}
