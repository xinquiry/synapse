mod error;

pub use error::PermissionError;

/// Represents a permission decision.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Decision {
    /// The action is allowed (this time only).
    Allow,
    /// The action is always allowed for this session.
    AllowAlways,
    /// The action is denied with a reason.
    Deny(String),
    /// The action is always denied for this session.
    DenyAlways(String),
    /// The action requires user approval.
    Ask(String),
}

/// A request for permission to perform an action.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PermissionRequest {
    /// The tool requesting permission.
    pub tool_name: String,
    /// Human-readable description of what the tool wants to do.
    pub description: String,
    /// Whether the action modifies the filesystem or system state.
    pub is_mutating: bool,
}
