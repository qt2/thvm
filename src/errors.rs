use std::fmt;

use thiserror::Error;

#[derive(Debug, Error)]
pub struct RuntimeError {
    pub kind: RuntimeErrorKind,
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.kind.fmt(f)
    }
}

#[derive(Debug, Error)]
pub enum RuntimeErrorKind {
    #[error("variable `{0}` could not found in this scope")]
    GlobalNotFound(String),
    #[error("invalid operator use")]
    InvalidOperator,
}
