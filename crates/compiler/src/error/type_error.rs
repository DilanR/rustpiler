use proc_macro2::Span;
use thiserror::Error;

use crate::ast::Type;

#[derive(Debug, Error, Clone)]
pub enum TypeError {
    #[error("Type mismatch: expected {expected}, got {got}")]
    TypeMismatch {
        expected: Type,
        got: Type,
        span: Span,
    },

    #[error("Undefined function `{name}`")]
    UnknownFunction { name: String, span: Span },

    #[error("Undefined variable `{name}`")]
    UnknownVariable { name: String, span: Span },

    #[error("Assignment error: {kind:?}")]
    Assignment {
        kind: AssignmentErrorKind,
        span: Span,
        decl_span: Span,
    },

    #[error("Duplicate {kind:?} `{name}`")]
    Duplicate {
        kind: DuplicateKind,
        name: String,
        first_span: Span,
        second_span: Span,
    },

    #[error("Uninitialized value `{name}`")]
    Uninitialized { name: String, span: Span },

    #[error("{id} expected {expected} parameters, got {got} arguments")]
    ParameterArityMismatch {
        id: String,
        expected: usize,
        got: usize,
        call_span: Span,
        fn_span: Span,
    },
}

#[derive(Debug, Clone)]
pub enum UnknownKind {
    Variable,
    Function,
}

#[derive(Debug, Clone)]
pub enum AssignmentErrorKind {
    NotIdent,
    NotFound,
    NotMutable,
}

#[derive(Debug, Clone)]
pub enum DuplicateKind {
    Function,
}
