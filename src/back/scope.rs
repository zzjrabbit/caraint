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
    Function(String, Vec<String>, Vec<Rc<AstNodes>>),
}

impl Symbol {
    pub fn get_value_mut(&mut self) -> Result<&mut CrValue> {
        match self {
            Symbol::Const(_, value) => Ok(value),
            Symbol::Var(_, value) => Ok(value),
            Symbol::Function(_, _, _) => Err(Error::UseVoidValue),
        }
    }

    pub fn get_id(&self) -> &str {
        match self {
            Symbol::Const(id, _) => id,
            Symbol::Var(id, _) => id,
            Symbol::Function(id, _, _) => id,
        }
    }

    pub fn get_value(&self) -> Result<&CrValue> {
        match self {
            Symbol::Const(_, value) => Ok(value),
            Symbol::Var(_, value) => Ok(value),
            Symbol::Function(_, _, _) => Err(Error::UseVoidValue),
        }
    }

    pub fn try_assign(&mut self, value: CrValue) -> Result<()> {
        match self {
            Symbol::Const(_, _) | Symbol::Function(_, _, _) => return Err(Error::BadAssign),
            Symbol::Var(_, old_value) => *old_value = value,
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
    pub fn new(father: Option<Rc<RefCell<Self>>>) -> Self {
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
            let (_, list) = symbol.get_value()?.into_list()?;
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
            let (_, list) = symbol.get_value()?.into_list()?;
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
            symbol.try_assign(value)?;
        } else if let Some(father) = &self.father {
            father.borrow_mut().symbol_assign(id, value)?;
        }
        Ok(())
    }

    #[inline]
    pub fn symbol_list_append(&mut self, id: &str, value: CrValue) -> Result<()> {
        if let Some(symbol) = self.symbols.get_mut(id) {
            let (start_len, list) = symbol.get_value_mut()?.into_list_mut()?;

            list.push(value);

            if list.len() > *start_len * 2 {
                *start_len = list.len() * 2;
                let _ = list.try_reserve(list.len() * 2);
            }
        } else if let Some(father) = &self.father {
            father.borrow_mut().symbol_list_append(id, value)?;
        }
        Ok(())
    }

    #[inline]
    pub fn symbol_list_insert(&mut self, id: &str, index: usize, value: CrValue) -> Result<()> {
        if let Some(symbol) = self.symbols.get_mut(id) {
            let (start_len, list) = symbol.get_value_mut()?.into_list_mut()?;

            list.insert(index, value);

            if list.len() > *start_len * 2 {
                *start_len = list.len() * 2;
                let _ = list.try_reserve(list.len() * 2);
            }
        } else if let Some(father) = &self.father {
            father.borrow_mut().symbol_list_insert(id, index, value)?;
        }
        Ok(())
    }

    #[inline]
    pub fn symbol_list_modify(&mut self, id: &str, index: usize, value: CrValue) -> Result<()> {
        if let Some(symbol) = self.symbols.get_mut(id) {
            let (_, list) = symbol.get_value_mut()?.into_list_mut()?;
            list[index] = value;
        } else if let Some(father) = &self.father {
            father.borrow_mut().symbol_list_modify(id, index, value)?;
        }
        Ok(())
    }

    #[inline]
    pub fn symbol_list_remove(&mut self, id: &str, index: usize) -> Result<CrValue> {
        if let Some(symbol) = self.symbols.get_mut(id) {
            let (_, list) = symbol.get_value_mut()?.into_list_mut()?;
            Ok(list.remove(index as usize))
        } else if let Some(father) = &self.father {
            Ok(father.borrow_mut().symbol_list_remove(id, index)?)
        } else {
            Err(Error::SymbolNotFound)
        }
    }
}
