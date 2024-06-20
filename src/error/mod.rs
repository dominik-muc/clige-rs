//! Contains error types.

/// Custom error enum.
#[derive(Debug, Clone)]
pub enum GameError {
    InvalidAction,
    InvalidTarget
}

impl GameError{
    pub fn what(&self) -> &str {
        match *self {
            Self::InvalidAction => "Invalid action",
            Self::InvalidTarget => "Invalid target"
        }
    }
}