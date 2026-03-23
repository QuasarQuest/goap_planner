// src/error.rs
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlannerError {
    /// The goal is mathematically impossible to reach with current actions.
    NoPathFound,
    /// Limit the Search depth
    // DepthLimitExceeded,
    /// The search expanded too many nodes (Memory Limit).
    NodeLimitExceeded,
    /// The search took too long (Real-Time Limit).
    TimeLimitExceeded,
    /// The action list was empty.
    EmptyActionList,
}
impl fmt::Display for PlannerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PlannerError::NoPathFound => write!(f, "Goal unreachable (Search space exhausted)."),
            PlannerError::NodeLimitExceeded => write!(f, "Budget exceeded: Node limit reached."),
            PlannerError::TimeLimitExceeded => write!(f, "Budget exceeded: Time limit reached."),
            PlannerError::EmptyActionList => write!(f, "Configuration Error: Action list is empty."),
        }
    }
}
impl std::error::Error for PlannerError {}