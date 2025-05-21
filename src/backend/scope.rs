use alloc::{collections::BTreeMap, rc::Rc, vec::Vec};
use core::ops::{Deref, DerefMut};

use super::{
    result::{Error, Result},
    value::CrValue,
};
use crate::ast::AstNodes;

#[derive(Debug, Clone)]
pub enum Symbol {
    Const(usize, CrValue),
    Var(usize, CrValue),
    Function(usize, Rc<Vec<usize>>, Rc<Vec<AstNodes>>),
}

impl Symbol {
    pub fn get_id(&self) -> &usize {
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

pub struct SymbolTable {
    symbols: BTreeMap<usize, Symbol>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            symbols: BTreeMap::new(),
        }
    }

    pub fn clear(&mut self) {
        self.symbols.clear();
    }
}

pub struct SymbolTables {
    tables: Vec<SymbolTable>,
    len: usize,
}
impl SymbolTables {
    #[allow(unused)]
    #[must_use]
    pub fn pop_take(&mut self) -> Option<SymbolTable> {
        if self.len != 0 {
            self.len -= 1;
        }
        self.tables.pop()
    }

    pub fn pop(&mut self) -> bool {
        let not_empty = self.len != 0;
        if not_empty {
            self.len -= 1;
            let len = self.len;
            self.tables[len].clear();
        }
        not_empty
    }

    pub fn push_new(&mut self) {
        if self.len <= self.tables.len() {
            self.tables.push(SymbolTable::new());
        }
        self.len += 1;
    }
}

impl From<Vec<SymbolTable>> for SymbolTables {
    fn from(tables: Vec<SymbolTable>) -> Self {
        let len = tables.len();
        Self { tables, len }
    }
}

impl Deref for SymbolTables {
    type Target = [SymbolTable];

    fn deref(&self) -> &Self::Target {
        &self.tables[..self.len]
    }
}

impl DerefMut for SymbolTables {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.tables[..self.len]
    }
}

impl SymbolTables {
    #[allow(unused)]
    pub fn last(&self) -> &SymbolTable {
        (**self).last().unwrap()
    }

    pub fn last_mut(&mut self) -> &mut SymbolTable {
        (**self).last_mut().unwrap()
    }

    pub fn insert_sym(&mut self, symbol: Symbol) {
        self.last_mut().symbols.insert(*symbol.get_id(), symbol);
    }

    pub fn clear_last(&mut self) {
        self.last_mut().clear();
    }

    #[inline]
    fn get_var<F, R>(&self, id: usize, f: F) -> R
    where
        F: FnOnce(Result<&Symbol>) -> R,
    {
        let sym = self
            .iter()
            .filter_map(|symt| symt.symbols.get(&id))
            .next_back();
        f(sym.ok_or(Error::SymbolNotFound(id)))
    }

    #[inline]
    fn get_var_mut<'a, F, R>(&'a mut self, id: usize, f: F) -> R
    where
        F: FnOnce(Result<&'a mut Symbol>) -> R,
    {
        let sym = self
            .iter_mut()
            .filter_map(|symt| symt.symbols.get_mut(&id))
            .next_back();
        f(sym.ok_or(Error::SymbolNotFound(id)))
    }

    #[inline]
    pub fn symbol_clone(&self, id: usize) -> Result<Symbol> {
        self.get_var(id, |sym| sym.cloned())
    }

    #[inline]
    pub fn symbol_clone_value(&self, id: usize) -> Result<CrValue> {
        self.get_var(id, |sym| sym.and_then(Symbol::get_value).cloned())
    }

    #[inline]
    pub fn symbol_crvalue_len(&self, id: usize) -> Result<usize> {
        self.get_var(id, |sym| {
            sym.and_then(Symbol::get_value)
                .and_then(CrValue::as_list)
                .map(Vec::len)
        })
    }

    #[inline]
    pub fn symbol_crvalue_list_item(&self, id: usize, index: usize) -> Result<CrValue> {
        self.get_var(id, |sym| {
            sym.and_then(Symbol::get_value)
                .and_then(CrValue::as_list)
                .map(|list| list[index].clone())
        })
    }

    #[inline]
    pub fn symbol_assign(&mut self, id: usize, value: CrValue) -> Result<()> {
        self.get_var_mut(id, |sym| sym.and_then(|sym| sym.assign(value)))
    }

    #[inline]
    fn symbol_list_mut(&mut self, id: usize) -> Result<&mut Vec<CrValue>> {
        self.get_var_mut(id, |sym| {
            sym.and_then(Symbol::get_value_mut)
                .and_then(CrValue::as_list_mut)
        })
    }

    #[inline]
    pub fn symbol_list_append(&mut self, id: usize, value: CrValue) -> Result<()> {
        self.symbol_list_mut(id).map(|vec| vec.push(value))
    }

    #[inline]
    pub fn symbol_list_insert(&mut self, id: usize, index: usize, value: CrValue) -> Result<()> {
        self.symbol_list_mut(id).map(|vec| vec.insert(index, value))
    }

    #[inline]
    pub fn symbol_list_modify(&mut self, id: usize, index: usize, value: CrValue) -> Result<()> {
        self.symbol_list_mut(id).map(|vec| vec[index] = value)
    }

    #[inline]
    pub fn symbol_list_remove(&mut self, id: usize, index: usize) -> Result<CrValue> {
        self.symbol_list_mut(id).map(|vec| vec.remove(index))
    }
}
