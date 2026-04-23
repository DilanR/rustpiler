use thiserror::Error;

use crate::ast::Literal;

#[derive(Debug, Error, Clone)]
pub enum CodeGenError {
    #[error("Unsupported Literal {0}")]
    UnsupportedLiteral(Literal),
}
