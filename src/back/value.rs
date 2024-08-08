use std::fmt::Display;

use num_bigint::BigInt;

use crate::ast::AstNodes;
use super::result::{Error, Result};

#[derive(Debug,Clone)]
pub enum CrValue {
    Number(BigInt),
    Function(Vec<String>,Vec<Box<AstNodes>>),
    List(Vec<CrValue>),
    Void,
}

impl Display for CrValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(number) => write!(f, "{}", number),
            Self::Function(_, _) => write!(f,"function"),
            Self::Void => write!(f, "void"),
            Self::List(data) => {
                write!(f,"[")?;
                for item in data.iter() {
                    write!(f, "{},", item)?;
                }
                write!(f,"]")?;
                Ok(())
            },
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

    pub fn into_list(&self) -> Result<Vec<CrValue>> {
        match self {
            CrValue::List(list) => Ok(list.clone()),
            _ => Err(Error::UseVoidValue),
        }
    }

    pub fn into_list_mut(&mut self) -> Result<&mut Vec<CrValue>> {
        match self {
            CrValue::List(list) => Ok(list),
            _ => Err(Error::UseVoidValue),
        }
    }
}

