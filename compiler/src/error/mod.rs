pub mod code_gen_error;
pub mod diagnostic;
pub mod type_error;
pub mod vm_error;

pub use code_gen_error::*;
pub use diagnostic::*;
pub use type_error::*;
pub use vm_error::*;

#[derive(Debug, thiserror::Error, Clone)]
pub enum Error {
    #[error(transparent)]
    Vm(#[from] VmError),

    #[error(transparent)]
    Type(#[from] TypeError),

    #[error(transparent)]
    Parse(#[from] syn::Error),

    #[error(transparent)]
    CodeGen(#[from] CodeGenError),

    #[error("{0}")]
    Message(String),
}

impl Error {
    pub fn to_diagnostics(self) -> Vec<Diagnostic> {
        match self {
            Error::Type(type_err) => vec![type_err.into()],

            // Optional: fallback for non-spanned errors
            other => vec![Diagnostic {
                message: other.to_string(),
                severity: Severity::Error,
                range: ErrRange::dummy(), // not great, but works
            }],
        }
    }
}

pub enum EnvError {}
