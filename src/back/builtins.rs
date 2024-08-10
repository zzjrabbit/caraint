use alloc::{rc::Rc, vec::Vec};
use core::fmt;
use dashu_int::IBig;
use spin::Mutex;

use super::result::{Error, Result};
use super::value::CrValue;
use crate::ast::AstNodes;
use crate::back::Interpreter;

static PRINTER: Mutex<Option<fn(fmt::Arguments)>> = Mutex::new(None);

pub fn set_printer(printer: fn(fmt::Arguments)) {
    let mut log = PRINTER.lock();
    *log = Some(printer);
}

pub(crate) fn print_message(args: fmt::Arguments) {
    if let Some(printer) = *PRINTER.lock() {
        printer(args);
    }
}

impl Interpreter {
    pub(super) fn print(&mut self, args: &Vec<Rc<AstNodes>>) -> Result<()> {
        args.iter()
            .map(|x| self.visit(x).unwrap())
            .for_each(|value| print_message(format_args!("{} ", value)));
        print_message(format_args!("\n"));
        Ok(())
    }

    pub(super) fn append(&mut self, args: &Vec<Rc<AstNodes>>) -> Result<()> {
        if args.len() != 2 {
            return Err(Error::ArgMismatch);
        }
        if let AstNodes::ReadVar(id) = args[0].as_ref() {
            let value = self.visit(&args[1])?;
            self.current_symbol_table
                .borrow_mut()
                .symbol_list_append(&id, value)?;
            Ok(())
        } else {
            Err(Error::ArgMismatch)
        }
    }

    pub(super) fn insert(&mut self, args: &Vec<Rc<AstNodes>>) -> Result<()> {
        if args.len() != 3 {
            return Err(Error::ArgMismatch);
        }
        if let AstNodes::ReadVar(id) = args[0].as_ref() {
            let number = self.visit(&args[1])?.into_int()?;
            let index = usize::try_from(&number).unwrap();
            let value = self.visit(&args[2])?;

            let mut symbol_table = self.current_symbol_table.borrow_mut();
            symbol_table.symbol_list_insert(&id, index, value)?;

            Ok(())
        } else {
            Err(Error::ArgMismatch)
        }
    }

    pub(super) fn len(&mut self, args: &Vec<Rc<AstNodes>>) -> Result<CrValue> {
        if args.len() != 1 {
            return Err(Error::ArgMismatch);
        }
        if let AstNodes::ReadVar(id) = args[0].as_ref() {
            let length = self
                .current_symbol_table
                .borrow_mut()
                .symbol_crvalue_len(&id)?;
            let value = CrValue::Number(IBig::from(length));
            Ok(value)
        } else {
            Err(Error::ArgMismatch)
        }
    }

    pub(super) fn remove(&mut self, args: &Vec<Rc<AstNodes>>) -> Result<CrValue> {
        if args.len() != 2 {
            return Err(Error::ArgMismatch);
        }
        if let AstNodes::ReadVar(id) = args[0].as_ref() {
            let number = self.visit(&args[1])?.into_int()?;
            let index = usize::try_from(&number).unwrap();

            let list = self
                .current_symbol_table
                .borrow_mut()
                .symbol_list_remove(&id, index)?;

            Ok(list)
        } else {
            Err(Error::ArgMismatch)
        }
    }
}
