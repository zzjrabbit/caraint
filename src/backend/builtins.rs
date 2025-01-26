use dashu_int::IBig;

use super::result::{Error, Result};
use super::value::CrValue;
use super::Interpreter;
use crate::ast::AstNodes;

impl Interpreter {
    pub(super) fn print(&mut self, args: &[AstNodes]) -> Result<()> {
        let printer = self.printer.ok_or(Error::NoPrinter)?;
        for arg in args {
            printer(format_args!("{}", self.visit(arg)?));
        }
        printer(format_args!("\n"));
        Ok(())
    }

    pub(super) fn append(&mut self, args: &[AstNodes]) -> Result<()> {
        let [AstNodes::ReadVar(id), arg] = args else {
            return Err(Error::ArgMismatch);
        };
        let value = self.visit(arg)?;
        self.symbol_tables.symbol_list_append(*id, value)?;
        Ok(())
    }

    pub(super) fn insert(&mut self, args: &[AstNodes]) -> Result<()> {
        let [AstNodes::ReadVar(id), arg1, arg2] = args else {
            return Err(Error::ArgMismatch);
        };
        let number = self.visit(arg1)?;
        let index = usize::try_from(number.as_int()?).unwrap();
        let value = self.visit(arg2)?;
        self.symbol_tables.symbol_list_insert(*id, index, value)
    }

    pub(super) fn len(&self, args: &[AstNodes]) -> Result<CrValue> {
        let [AstNodes::ReadVar(id)] = args else {
            return Err(Error::ArgMismatch);
        };
        let length = self.symbol_tables.symbol_crvalue_len(*id)?;
        let value = CrValue::Number(IBig::from(length));
        Ok(value)
    }

    pub(super) fn remove(&mut self, args: &[AstNodes]) -> Result<CrValue> {
        let [AstNodes::ReadVar(id), arg] = args else {
            return Err(Error::ArgMismatch);
        };
        let index = usize::try_from(self.visit(arg)?.as_int()?).unwrap();
        let list = self.symbol_tables.symbol_list_remove(*id, index)?;
        Ok(list)
    }
}
