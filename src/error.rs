use crate::ast::Type;
use crate::vm::Val;
use proc_macro2::Span;
use thiserror::Error;

#[derive(Debug, Error, Clone)]
pub enum Error {
    #[error("Expected Literal")]
    ExpectedLiteral(),

    #[error("undefined function `{0}`")]
    UndefinedFunction(String),

    #[error("undefined value `{0}`")]
    UndefinedValue(String),

    #[error("undefined operation")]
    UndefinedOperation(),

    #[error("type mismatch: expected {expected}, got {got:?}")]
    TypeMismatch { expected: &'static str, got: Val },

    #[error("invalid operand: {0}")]
    InvalidOperand(String),

    #[error("{0}")]
    Message(String),

    #[error(transparent)]
    Vm(#[from] VmError),

    #[error(transparent)]
    Type(#[from] TypeError),

    #[error("Function {id} expected {expected} parameters, got {got}")]
    ParameterArityMismatch {
        id: String,
        expected: usize,
        got: usize,
    },
}

#[derive(Debug, Error, Clone)]
pub enum VmError {
    #[error("No Main function found")]
    NoMainFound,

    #[error("Failed to find Val with id {0}")]
    NoValFound(String),

    #[error("Failed to find Function with id {0}")]
    NoFunctionFound(String),

    #[error("Invalid assignment target: {0}")]
    IllegalAssignment(String),
}

#[derive(Debug, Error, Clone)]
pub enum TypeError {
    #[error("type mismatch at {range}\nExpected {expected}, got {got}")]
    InferenceMismatch {
        expected: Type,
        got: Type,
        range: ErrRange,
    },

    #[error("Uninitialized value {0} with no explicit type")]
    UnInitType(String),

    #[error("Invalid Assignment target {0}")]
    AssignmentToNonIdent(String),

    #[error("Assignment target {0} not found")]
    AssignmentTargetNotFound(String),

    #[error("NonMutable Assignment")]
    NonMutableAssignment(),

    #[error("undefined variable `{0}`")]
    UndefinedBinding(String),

    #[error("Duplicate function {0} found")]
    DuplicateFunction(String),
}

#[derive(Debug, Error, Clone)]
pub enum CodeGenError {}

#[derive(Debug, Error, Clone)]
pub enum EnvError {}

impl From<String> for Error {
    fn from(s: String) -> Self {
        Error::Message(s)
    }
}

impl From<&str> for Error {
    fn from(s: &str) -> Self {
        Error::Message(s.to_owned())
    }
}

#[derive(Debug, Clone)]
pub struct ErrRange {
    pub start_line: usize,
    pub start_column: usize,
    pub end_line: usize,
    pub end_column: usize,
}

impl From<Span> for ErrRange {
    fn from(span: Span) -> Self {
        let start = span.start();
        let end = span.end();

        Self {
            start_line: start.line,
            start_column: start.column,
            end_line: end.line,
            end_column: end.column,
        }
    }
}

use std::fmt;

impl fmt::Display for ErrRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "l{}, c{} to l{}, c{}",
            self.start_line, self.start_column, self.end_line, self.end_column
        )
    }
}
