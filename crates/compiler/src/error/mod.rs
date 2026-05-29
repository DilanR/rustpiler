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
}

impl From<Error> for Diagnostic {
    // TODO: vec![Diagnostic]
    fn from(value: Error) -> Self {
        match value {
            Error::Vm(err) => err.into(),
            Error::Type(err) => err.into(),
            Error::Parse(err) => err.into(),
            Error::CodeGen(err) => err.into(),
        }
    }
}

pub enum EnvError {}
