use std::fmt::Display;

use num_bigint::BigInt;

use crate::ast::AstNodes;
use super::result::{Error, Result};

#[derive(Debug, Clone)]
pub enum CrValue {
    Number(BigInt),
    Function(Vec<String>,Vec<Box<AstNodes>>),
    Void,
}

impl Display for CrValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(number) => write!(f, "{}", number),
            Self::Function(_, _) => write!(f,"function"),
            Self::Void => write!(f, "void"),
        }
    }
}

impl CrValue {
    pub fn into_int(&self) -> Result<BigInt> {
        match self {
            CrValue::Number(num) => Ok(num.clone()),
            _ => Err(Error::UseVoidValue),
        }
    }
}

