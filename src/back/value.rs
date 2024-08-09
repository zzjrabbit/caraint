use std::fmt::Display;

use num_bigint::BigInt;

use super::result::{Error, Result};
use crate::ast::AstNodes;

#[derive(Debug, Clone)]
pub enum CrValue {
    Number(BigInt),
    Function(Vec<String>, Vec<Box<AstNodes>>),
    List(usize, Vec<Box<CrValue>>),
    Void,
}

impl Display for CrValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(number) => write!(f, "{}", number),
            Self::Function(_, _) => write!(f, "function"),
            Self::Void => write!(f, "void"),
            Self::List(_, data) => {
                write!(f, "[")?;
                for item in data.iter() {
                    write!(f, "{},", item)?;
                }
                write!(f, "]")?;
                Ok(())
            }
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

    pub fn into_list(&self) -> Result<(usize, &Vec<Box<CrValue>>)> {
        match self {
            CrValue::List(start_len, list) => Ok((*start_len, list)),
            _ => Err(Error::UseVoidValue),
        }
    }

    pub fn into_list_mut(&mut self) -> Result<(&mut usize, &mut Vec<Box<CrValue>>)> {
        match self {
            CrValue::List(start_len, list) => Ok((start_len, list)),
            _ => Err(Error::UseVoidValue),
        }
    }
}
