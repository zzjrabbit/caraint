use std::fmt;

use super::value::CrValue;

/// Error returned by IR generator.
pub enum Error {
    DuplicatedDef,
    SymbolNotFound,
    FailedToEval,
    InvalidArrayLen,
    InvalidInit,
    BadAssign,
    NotInLoop,
    RetValInVoidFunc,
    DerefInt,
    UseVoidValue,
    ArgMismatch,
    NonIntCalc,
    UnknownOperator,
    Return(CrValue),
    Break,
    Continue,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::DuplicatedDef => write!(f, "duplicated symbol definition"),
            Self::SymbolNotFound => write!(f, "symbol not found"),
            Self::FailedToEval => write!(f, "failed to evaluate constant"),
            Self::InvalidArrayLen => write!(f, "invalid array length"),
            Self::InvalidInit => write!(f, "invalid initializer"),
            Self::BadAssign => write!(f, "assigning to constant"),
            Self::NotInLoop => write!(f, "using break/continue outside of loop"),
            Self::RetValInVoidFunc => write!(f, "returning value in void fucntion"),
            Self::DerefInt => write!(f, "dereferencing an integer"),
            Self::UseVoidValue => write!(f, "using a void value"),
            Self::ArgMismatch => write!(f, "argument mismatch"),
            Self::NonIntCalc => write!(f, "non-integer calculation"),
            Self::UnknownOperator => write!(f, "unknown operator"),
            _ => Ok(()),
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

/// Result type of IR generator.
pub type Result<T> = std::result::Result<T, Error>;
