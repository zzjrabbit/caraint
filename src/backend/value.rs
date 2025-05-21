use alloc::{rc::Rc, string::String, vec::Vec};
use core::fmt::{self, Display};
use dashu_int::IBig;

use super::result::{Error, Result};
use crate::ast::AstNodes;

#[derive(Debug, Clone)]
pub enum CrValue {
    Number(IBig),
    Function(Rc<Vec<String>>, Rc<Vec<AstNodes>>),
    List(Vec<CrValue>),
    Void,
}

impl PartialEq<IBig> for CrValue {
    fn eq(&self, other: &IBig) -> bool {
        matches!(self, Self::Number(num) if num == other)
    }
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
    pub fn as_int(&self) -> Result<&IBig> {
        match self {
            Self::Number(num) => Ok(num),
            _ => Err(Error::UseVoidValue),
        }
    }

    pub fn as_list(&self) -> Result<&Vec<Self>> {
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
