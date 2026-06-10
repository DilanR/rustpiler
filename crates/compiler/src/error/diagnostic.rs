use std::fmt;

use proc_macro2::Span;
use serde::Serialize;

use crate::error::{AssignmentErrorKind, CodeGenError, DuplicateKind, TypeError, VmError};

#[derive(Debug, Serialize)]
pub struct Diagnostics(pub Vec<Diagnostic>);

impl Diagnostics {
    pub fn push(&mut self, diagnostic: Diagnostic) {
        self.0.push(diagnostic);
    }

    pub fn extend(&mut self, other: Diagnostics) {
        self.0.extend(other.0);
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Diagnostic {
    pub message: String,
    pub severity: Severity,
    pub range: Option<ErrRange>,
    pub related: Vec<Diagnostic>,
}

#[derive(Debug, Clone, Serialize)]
pub enum Severity {
    Error,
    Warning,
    Info,
    Help,
}

impl FromIterator<Diagnostic> for Diagnostics {
    fn from_iter<T: IntoIterator<Item = Diagnostic>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl From<syn::Error> for Diagnostic {
    fn from(err: syn::Error) -> Self {
        Diagnostic {
            message: err.to_string(),
            severity: Severity::Error,
            range: Some(err.span().into()),
            related: vec![],
        }
    }
}

//TODO: Fix spans
impl From<CodeGenError> for Diagnostic {
    fn from(err: CodeGenError) -> Self {
        Self {
            message: err.to_string(),
            severity: Severity::Error,
            range: None,
            related: vec![],
        }
    }
}

//TODO: Fix spans
impl From<VmError> for Diagnostic {
    fn from(err: VmError) -> Self {
        Self {
            message: err.to_string(),
            severity: Severity::Error,
            range: None,
            related: vec![],
        }
    }
}

impl From<TypeError> for Diagnostic {
    fn from(err: TypeError) -> Self {
        match err {
            TypeError::TypeMismatch {
                expected,
                got,
                span,
            } => Diagnostic {
                message: format!("mismatched types: expected `{}`, got `{}`", expected, got),
                severity: Severity::Error,
                range: Some(span.into()),
                related: vec![
                    Diagnostic {
                        message: format!("expected type `{}` originates here", expected),
                        severity: Severity::Info,
                        range: Some(expected.span.into()),
                        related: vec![],
                    },
                    Diagnostic {
                        message: format!("actual type `{}` originates here", got),
                        severity: Severity::Info,
                        range: Some(got.span.into()),
                        related: vec![],
                    },
                ],
            },

            TypeError::UnknownFunction { name, span } => Diagnostic {
                message: format!("cannot find function `{}` in this scope", name),
                severity: Severity::Error,
                range: Some(span.into()),
                related: vec![],
            },

            TypeError::UnknownVariable { name, span } => Diagnostic {
                message: format!("cannot find variable `{}` in this scope", name),
                severity: Severity::Error,
                range: Some(span.into()),
                related: vec![],
            },

            TypeError::Assignment {
                kind,
                span,
                decl_span,
            } => {
                let (message, related) = match kind {
                    AssignmentErrorKind::NotIdent => (
                        "invalid assignment target".to_string(),
                        vec![Diagnostic {
                            message:
                                "only variables may appear on the left-hand side of an assignment"
                                    .to_string(),
                            severity: Severity::Info,
                            range: Some(span.into()),
                            related: vec![],
                        }],
                    ),

                    AssignmentErrorKind::NotFound => {
                        ("cannot assign to an unknown variable".to_string(), vec![])
                    }

                    AssignmentErrorKind::NotMutable => (
                        "cannot assign to immutable variable".to_string(),
                        vec![Diagnostic {
                            message: "consider declaring the variable with `mut`".to_string(),
                            severity: Severity::Info,
                            range: Some(decl_span.into()),
                            related: vec![],
                        }],
                    ),
                };

                Diagnostic {
                    message,
                    severity: Severity::Error,
                    range: Some(span.into()),
                    related,
                }
            }

            TypeError::Duplicate {
                kind,
                name,
                first_span,
                second_span,
            } => {
                let kind_str = match kind {
                    DuplicateKind::Function => "function",
                };

                Diagnostic {
                    message: format!("duplicate {} `{}`", kind_str, name),
                    severity: Severity::Error,
                    range: Some(second_span.into()),
                    related: vec![Diagnostic {
                        message: "previous definition is here".to_string(),
                        severity: Severity::Info,
                        range: Some(first_span.into()),
                        related: vec![],
                    }],
                }
            }

            TypeError::Uninitialized { name, span } => Diagnostic {
                message: format!("variable `{}` may be uninitialized", name),
                severity: Severity::Error,
                range: Some(span.into()),
                related: vec![Diagnostic {
                    message: "initialize the variable before it is used".to_string(),
                    severity: Severity::Info,
                    range: None,
                    related: vec![],
                }],
            },

            TypeError::ParameterArityMismatch {
                id,
                expected,
                got,
                call_span,
                fn_span,
            } => Diagnostic {
                message: format!(
                    "function `{}` expects {} arguments but {} were supplied",
                    id, expected, got,
                ),
                severity: Severity::Error,
                range: Some(call_span.into()),
                related: vec![Diagnostic {
                    message: "function declaration is here".to_string(),
                    severity: Severity::Info,
                    range: Some(fn_span.into()),
                    related: vec![],
                }],
            },
        }
    }
}

#[derive(Debug, Clone, Serialize)]
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

impl fmt::Display for ErrRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "l{}, c{} to l{}, c{}",
            self.start_line, self.start_column, self.end_line, self.end_column
        )
    }
}
