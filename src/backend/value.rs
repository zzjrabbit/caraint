use alloc::{string::String, vec::Vec};
use core::fmt::{self, Display};
use dashu_int::IBig;

use super::result::{Error, Result};
use crate::ast::AstNodes;

#[derive(Debug, Clone)]
pub enum CrValue {
    Number(IBig),
    Function(Vec<String>, Vec<AstNodes>),
    List(Vec<CrValue>),
    Void,
}

impl Display for CrValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(number) => write!(f, "{number}"),
            Self::Function(_, _) => write!(f, "function"),
            Self::Void => write!(f, "void"),
            Self::List(data) => {
                write!(f, "[")?;
                data.iter().try_for_each(|item| write!(f, "{item},"))?;
                write!(f, "]")?;
                Ok(())
            }
        }
    }
}

impl CrValue {
    pub const fn as_int(&self) -> Result<&IBig> {
        match self {
            Self::Number(num) => Ok(num),
            _ => Err(Error::UseVoidValue),
        }
    }

    pub const fn as_list(&self) -> Result<&Vec<Self>> {
        match self {
            Self::List(list) => Ok(list),
            _ => Err(Error::UseVoidValue),
        }
    }

    pub fn as_list_mut(&mut self) -> Result<&mut Vec<Self>> {
        match self {
            Self::List(list) => Ok(list),
            _ => Err(Error::UseVoidValue),
        }
    }
}
