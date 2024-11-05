use core::fmt;
use dashu_int::IBig;
use spin::Mutex;

use super::result::{Error, Result};
use super::value::CrValue;
use super::Interpreter;
use crate::ast::AstNodes;

static PRINTER: Mutex<Option<fn(fmt::Arguments)>> = Mutex::new(None);

pub fn set_printer(printer: fn(fmt::Arguments)) {
    let mut log = PRINTER.lock();
    *log = Some(printer);
}

pub fn print_message(args: fmt::Arguments) {
    if let Some(printer) = *PRINTER.lock() {
        printer(args);
    }
}

impl Interpreter {
    pub(super) fn print(&mut self, args: &[AstNodes]) -> Result<()> {
        for arg in args {
            let value = self.visit(arg)?;
            print_message(format_args!("{}", value));
        }
        print_message(format_args!("\n"));
        Ok(())
    }

    pub(super) fn append(&mut self, args: &[AstNodes]) -> Result<()> {
        if args.len() != 2 {
            return Err(Error::ArgMismatch);
        }
        if let AstNodes::ReadVar(id) = args[0] {
            let value = self.visit(&args[1])?;
            self.symbol_tables.symbol_list_append(id, value)?;
            Ok(())
        } else {
            Err(Error::ArgMismatch)
        }
    }

    pub(super) fn insert(&mut self, args: &[AstNodes]) -> Result<()> {
        if args.len() != 3 {
            return Err(Error::ArgMismatch);
        }
        if let AstNodes::ReadVar(id) = args[0] {
            let number = self.visit(&args[1])?;
            let index = usize::try_from(number.as_int()?).unwrap();
            let value = self.visit(&args[2])?;

            self.symbol_tables.symbol_list_insert(id, index, value)?;

            Ok(())
        } else {
            Err(Error::ArgMismatch)
        }
    }

    pub(super) fn len(&self, args: &[AstNodes]) -> Result<CrValue> {
        if args.len() != 1 {
            return Err(Error::ArgMismatch);
        }
        if let AstNodes::ReadVar(id) = args[0] {
            let length = self.symbol_tables.symbol_crvalue_len(id)?;
            let value = CrValue::Number(IBig::from(length));
            Ok(value)
        } else {
            Err(Error::ArgMismatch)
        }
    }

    pub(super) fn remove(&mut self, args: &[AstNodes]) -> Result<CrValue> {
        if args.len() != 2 {
            return Err(Error::ArgMismatch);
        }
        if let AstNodes::ReadVar(id) = args[0] {
            let number = self.visit(&args[1])?;
            let index = usize::try_from(number.as_int()?).unwrap();

            let list = self.symbol_tables.symbol_list_remove(id, index)?;

            Ok(list)
        } else {
            Err(Error::ArgMismatch)
        }
    }
}
