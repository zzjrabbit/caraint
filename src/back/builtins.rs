use core::fmt;
use alloc::{rc::Rc,vec::Vec};

use num_bigint::BigInt;
use spin::Mutex;

use crate::ast::AstNodes;

use super::{
    result::{Error, Result},
    value::CrValue,
    Interpreter,
};

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
            let index = *self
                .visit(&args[1])?
                .into_int()?
                .to_u64_digits()
                .1
                .get(0)
                .unwrap_or(&0);
            let value = self.visit(&args[2])?;

            let mut symbol_table = self.current_symbol_table.borrow_mut();
            symbol_table.symbol_list_insert(&id, index as usize, value)?;

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
            let value = CrValue::Number(BigInt::from(length));
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
            let (_sign, value) = self.visit(&args[1])?.into_int()?.to_u64_digits();
            let index = *value.get(0).unwrap_or(&0);

            let list = self
                .current_symbol_table
                .borrow_mut()
                .symbol_list_remove(&id, index as usize)?;

            Ok((*list).clone())
        } else {
            Err(Error::ArgMismatch)
        }
    }
}
