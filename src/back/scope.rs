use std::sync::Arc;

use spin::RwLock;

use crate::ast::AstNodes;
use super::{result::{Error, Result}, value::CrValue};

#[derive(Debug, Clone)]
pub enum Symbol {
    Const(String, CrValue),
    Var(String, CrValue),
    Function(String, Vec<String>, Vec<Box<AstNodes>>),
}

impl Symbol {
    pub fn get_id(&self) -> String {
        match self {
            Symbol::Const(id, _) => id,
            Symbol::Var(id, _) => id,
            Symbol::Function(id, _, _) => id,
        }
        .clone()
    }

    pub fn get_value(&self) -> Result<CrValue> {
        match self {
            Symbol::Const(_, value) => Ok(value.clone()),
            Symbol::Var(_, value) => Ok(value.clone()),
            Symbol::Function(_, _, _) => Err(Error::UseVoidValue),
        }
    }

    pub fn try_assign(&mut self, value: CrValue) -> Result<()> {
        match self {
            Symbol::Const(_, _) | Symbol::Function(_,_,_) => return Err(Error::BadAssign),
            Symbol::Var(_, old_value) => *old_value = value,
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct SymbolTable {
    symbols: Vec<Symbol>,
    father: Option<Arc<RwLock<Self>>>,
}

impl SymbolTable {
    pub fn new(father: Option<Arc<RwLock<Self>>>) -> Self {
        Self {
            symbols: Vec::new(),
            father,
        }
    }

    pub fn insert(&mut self, symbol: Symbol) {
        self.symbols.push(symbol);
    }

    pub fn get(&self, id: &str) -> Option<&Symbol> {
        if let Some(symbol) = self.symbols.iter().find(|s| s.get_id() == id) {
            Some(symbol)
        } else if self.father.is_some() {
            self.father
                .as_ref()
                .unwrap()
                .read()
                .get(id)
                .map(|v| unsafe { &*(v as *const Symbol) })
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, id: &str) -> Option<&mut Symbol> {
        if let Some(symbol) = self.symbols.iter_mut().find(|s| s.get_id() == id) {
            Some(symbol)
        } else if self.father.is_some() {
            self.father
                .as_mut()
                .unwrap()
                .write()
                .get_mut(id)
                .map(|v| unsafe { &mut *(v as *mut Symbol) })
        } else {
            None
        }
    }
}
