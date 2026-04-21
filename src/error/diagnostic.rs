use std::fmt;

use proc_macro2::Span;

use crate::error::{AssignmentErrorKind, DuplicateKind, TypeError};

#[derive(Debug)]
pub struct Diagnostic {
    pub message: String,
    pub severity: Severity,
    pub range: ErrRange,
}

#[derive(Debug)]
pub enum Severity {
    Error,
    Warning,
    Info,
}

impl From<TypeError> for Diagnostic {
    fn from(err: TypeError) -> Self {
        match err {
            TypeError::TypeMismatch {
                expected,
                got,
                span,
            } => Diagnostic {
                message: format!("expected {}, got {}", expected, got),
                severity: Severity::Error,
                range: span.into(),
            },
            TypeError::UnknownFunction { name, span } => Diagnostic {
                message: format!("undefined function `{}`", name),
                severity: Severity::Error,
                range: span.into(),
            },
            TypeError::UnknownVariable { name, span } => Diagnostic {
                message: format!("undefined variable `{}`", name),
                severity: Severity::Error,
                range: span.into(),
            },
            TypeError::Assignment { kind, span } => {
                let msg = match kind {
                    AssignmentErrorKind::NotIdent => "left-hand side is not assignable",
                    AssignmentErrorKind::NotFound => "variable not found",
                    AssignmentErrorKind::NotMutable => "variable is not mutable",
                };

                Diagnostic {
                    message: msg.to_string(),
                    severity: Severity::Error,
                    range: span.into(),
                }
            }
            TypeError::Duplicate { kind, name, span } => {
                let kind_str = match kind {
                    DuplicateKind::Function => "function",
                };

                Diagnostic {
                    message: format!("duplicate {} `{}`", kind_str, name),
                    severity: Severity::Error,
                    range: span.into(),
                }
            }
            TypeError::Uninitialized { name, span } => Diagnostic {
                message: format!("use of uninitialized variable `{}`", name),
                severity: Severity::Error,
                range: span.into(),
            },

            TypeError::ParameterArityMismatch {
                id,
                expected,
                got,
                span,
            } => todo!(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ErrRange {
    pub start_line: usize,
    pub start_column: usize,
    pub end_line: usize,
    pub end_column: usize,
}

impl ErrRange {
    pub fn dummy() -> Self {
        let dummy_line_column = proc_macro2::Span::call_site().start();

        Self {
            start_line: dummy_line_column.line,
            start_column: dummy_line_column.column,
            end_line: dummy_line_column.line,
            end_column: dummy_line_column.column,
        }
    }
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

impl fmt::Display for ErrRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "l{}, c{} to l{}, c{}",
            self.start_line, self.start_column, self.end_line, self.end_column
        )
    }
}
