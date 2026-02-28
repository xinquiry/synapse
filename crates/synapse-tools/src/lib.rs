mod error;

pub use error::ToolError;

use synapse_permissions::PermissionRequest;

/// Trait for tools that the Agent can invoke.
#[async_trait::async_trait]
pub trait Tool: Send + Sync {
    /// The unique name of this tool (e.g., "Read", "Write", "Bash").
    fn name(&self) -> &str;

    /// Human-readable description of what this tool does.
    fn description(&self) -> &str;

    /// JSON Schema describing the tool's input parameters.
    fn input_schema(&self) -> &serde_json::Value;

    /// Build a permission request for this invocation.
    /// Returns `None` if no permission is needed.
    fn permission_request(&self, input: &serde_json::Value) -> Option<PermissionRequest>;

    /// Execute the tool with the given input.
    async fn execute(&self, input: serde_json::Value) -> Result<ToolOutput, ToolError>;
}

/// Output from a tool execution.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ToolOutput {
    pub content: String,
    pub is_error: bool,
}
