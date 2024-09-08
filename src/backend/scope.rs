use alloc::{collections::BTreeMap, rc::Rc, string::String, vec::Vec};
use core::cell::RefCell;

use super::{
    result::{Error, Result},
    value::CrValue,
};
use crate::ast::AstNodes;

#[derive(Debug, Clone)]
pub enum Symbol {
    Const(String, CrValue),
    Var(String, CrValue),
    Function(String, Vec<String>, Vec<AstNodes>),
}

impl Symbol {
    pub fn get_id(&self) -> &str {
        match self {
            Self::Const(id, _) | Self::Var(id, _) | Self::Function(id, _, _) => id,
        }
    }

    pub const fn get_value(&self) -> Result<&CrValue> {
        match self {
            Self::Const(_, value) | Self::Var(_, value) => Ok(value),
            Self::Function(_, _, _) => Err(Error::UseVoidValue),
        }
    }

    pub fn get_value_mut(&mut self) -> Result<&mut CrValue> {
        match self {
            Self::Const(_, value) | Self::Var(_, value) => Ok(value),
            Self::Function(_, _, _) => Err(Error::UseVoidValue),
        }
    }

    pub fn assign(&mut self, value: CrValue) -> Result<()> {
        match self {
            Self::Const(_, _) | Self::Function(_, _, _) => return Err(Error::BadAssign),
            Self::Var(_, old_value) => *old_value = value,
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct SymbolTable {
    symbols: BTreeMap<String, Symbol>,
    father: Option<Rc<RefCell<Self>>>,
}

impl SymbolTable {
    pub const fn new(father: Option<Rc<RefCell<Self>>>) -> Self {
        Self {
            symbols: BTreeMap::new(),
            father,
        }
    }

    pub fn insert(&mut self, symbol: Symbol) {
        self.symbols.insert(String::from(symbol.get_id()), symbol);
    }

    pub fn clear(&mut self) {
        self.symbols.clear();
    }
}

impl SymbolTable {
    #[inline]
    pub fn symbol_clone(&self, id: &str) -> Result<Symbol> {
        if let Some(symbol) = self.symbols.get(id) {
            Ok(symbol.clone())
        } else if let Some(father) = &self.father {
            father.borrow().symbol_clone(id)
        } else {
            Err(Error::SymbolNotFound)
        }
    }

    #[inline]
    pub fn symbol_clone_value(&self, id: &str) -> Result<CrValue> {
        if let Some(symbol) = self.symbols.get(id) {
            Ok(symbol.get_value()?.clone())
        } else if let Some(father) = &self.father {
            father.borrow().symbol_clone_value(id)
        } else {
            Err(Error::SymbolNotFound)
        }
    }

    #[inline]
    pub fn symbol_crvalue_len(&self, id: &str) -> Result<usize> {
        if let Some(symbol) = self.symbols.get(id) {
            let list = symbol.get_value()?.as_list()?;
            Ok(list.len())
        } else if let Some(father) = &self.father {
            father.borrow().symbol_crvalue_len(id)
        } else {
            Err(Error::SymbolNotFound)
        }
    }

    #[inline]
    pub fn symbol_crvalue_list_item(&self, id: &str, index: usize) -> Result<CrValue> {
        if let Some(symbol) = self.symbols.get(id) {
            let list = symbol.get_value()?.as_list()?;
            Ok(list[index].clone())
        } else if let Some(father) = &self.father {
            father.borrow().symbol_crvalue_list_item(id, index)
        } else {
            Err(Error::SymbolNotFound)
        }
    }

    #[inline]
    pub fn symbol_assign(&mut self, id: &str, value: CrValue) -> Result<()> {
        if let Some(symbol) = self.symbols.get_mut(id) {
            symbol.assign(value)?;
        } else if let Some(father) = &self.father {
            father.borrow_mut().symbol_assign(id, value)?;
        }
        Ok(())
    }

    #[inline]
    pub fn symbol_list_append(&mut self, id: &str, value: CrValue) -> Result<()> {
        if let Some(symbol) = self.symbols.get_mut(id) {
            let list = symbol.get_value_mut()?.as_list_mut()?;
            list.push(value);
        } else if let Some(father) = &self.father {
            father.borrow_mut().symbol_list_append(id, value)?;
        }
        Ok(())
    }

    #[inline]
    pub fn symbol_list_insert(&mut self, id: &str, index: usize, value: CrValue) -> Result<()> {
        if let Some(symbol) = self.symbols.get_mut(id) {
            let list = symbol.get_value_mut()?.as_list_mut()?;
            list.insert(index, value);
        } else if let Some(father) = &self.father {
            father.borrow_mut().symbol_list_insert(id, index, value)?;
        }
        Ok(())
    }

    #[inline]
    pub fn symbol_list_modify(&mut self, id: &str, index: usize, value: CrValue) -> Result<()> {
        if let Some(symbol) = self.symbols.get_mut(id) {
            let list = symbol.get_value_mut()?.as_list_mut()?;
            list[index] = value;
        } else if let Some(father) = &self.father {
            father.borrow_mut().symbol_list_modify(id, index, value)?;
        }
        Ok(())
    }

    #[inline]
    pub fn symbol_list_remove(&mut self, id: &str, index: usize) -> Result<CrValue> {
        if let Some(symbol) = self.symbols.get_mut(id) {
            let list = symbol.get_value_mut()?.as_list_mut()?;
            Ok(list.remove(index))
        } else if let Some(father) = &self.father {
            Ok(father.borrow_mut().symbol_list_remove(id, index)?)
        } else {
            Err(Error::SymbolNotFound)
        }
    }
}
