use num_bigint::BigInt;

use crate::ast::AstNodes;

use super::{value::CrValue, Interpreter, result::{Result, Error}};

impl Interpreter {
    pub(super) fn print(&mut self, args: Vec<Box<AstNodes>>) -> Result<()> {
        let values: Vec<CrValue> = args.iter().map(|x| self.visit(x.clone()).unwrap()).collect();
        for value in values {
            print!("{} ", value);
        }
        println!();
        Ok(())
    }

    pub(super) fn append(&mut self, args: Vec<Box<AstNodes>>) -> Result<()> {
        if args.len() != 2 {
            return Err(Error::ArgMismatch);
        }
        if let AstNodes::ReadVar(id) = args[0].as_ref() {
            let value = self.visit(args[1].clone())?;

            let mut symbol_table = self.current_symbol_table.write();

            let symbol = symbol_table.get_mut(id).unwrap().get_value_mut()?;
            let (start_len,list) = symbol.into_list_mut()?;

            list.push(Box::new(value));

            if list.len() > *start_len*2 {
                *start_len = list.len()*2;
                let _ = list.try_reserve(list.len());
            }

            Ok(())
        } else {
            Err(Error::ArgMismatch)
        }
    }

    pub(super) fn insert(&mut self, args: Vec<Box<AstNodes>>) -> Result<()> {
        if args.len() != 3 {
            return Err(Error::ArgMismatch);
        }
        if let AstNodes::ReadVar(id) = args[0].as_ref() {
            let index = *self.visit(args[1].clone())?.into_int()?.to_u64_digits().1.get(0).unwrap_or(&0);
            let value = self.visit(args[2].clone())?;

            let mut symbol_table = self.current_symbol_table.write();

            let symbol = symbol_table.get_mut(id).unwrap().get_value_mut()?;
            let (start_len,list) = symbol.into_list_mut()?;

            list.insert(index as usize, Box::new(value));

            if list.len() > *start_len*2 {
                *start_len = list.len()*2;
                let _ = list.try_reserve(list.len()*2);
            }

            Ok(())
        } else {
            Err(Error::ArgMismatch)
        }
    }

    pub(super) fn len(&mut self, args: Vec<Box<AstNodes>>) -> Result<CrValue> {
        if args.len() != 1 {
            return Err(Error::ArgMismatch);
        }
        if let AstNodes::ReadVar(id) = args[0].as_ref() {
            let symbol_table = self.current_symbol_table.write();

            let symbol = symbol_table.get(id).unwrap().get_value()?;
            let (_,list) = symbol.into_list()?;

            Ok(CrValue::Number(BigInt::from(list.len())))
        } else {
            Err(Error::ArgMismatch)
        }
    }

    pub(super) fn remove(&mut self, args: Vec<Box<AstNodes>>) -> Result<CrValue> {
        if args.len() != 2 {
            return Err(Error::ArgMismatch);
        }
        if let AstNodes::ReadVar(id) = args[0].as_ref() {
            let index = *self.visit(args[1].clone())?.into_int()?.to_u64_digits().1.get(0).unwrap_or(&0);

            let mut symbol_table = self.current_symbol_table.write();

            let symbol = symbol_table.get_mut(id).unwrap().get_value_mut()?;
            let (_,list) = symbol.into_list_mut()?;

            Ok(*list.remove(index as usize))
        } else {
            Err(Error::ArgMismatch)
        }
    }
}
