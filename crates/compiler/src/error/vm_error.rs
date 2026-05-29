use thiserror::Error;

use crate::vm::Val;

#[derive(Debug, Error, Clone)]
pub enum VmError {
    #[error("Expected Literal")]
    ExpectedLiteral,

    #[error("undefined function `{0}`")]
    UndefinedFunction(String),

    #[error("undefined value `{0}`")]
    UndefinedValue(String),

    #[error("undefined operation")]
    UndefinedOperation,

    #[error("invalid operand: {0}")]
    InvalidOperand(String),

    #[error("type mismatch: expected {expected}, got {got:?}")]
    TypeMismatch { expected: &'static str, got: Val },

    #[error("illegal assignment: {0}")]
    IllegalAssignment(String),
}
