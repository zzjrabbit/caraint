use num_bigint::BigInt;
use spin::RwLock;
use std::{iter::zip, sync::Arc};
use value::CrValue;

use crate::ast::AstNodes;
use result::{Error, Result};
use scope::{Symbol, SymbolTable};

mod builtins;
mod result;
mod scope;
mod value;

/// The interpreter
pub struct Interpreter {
    current_symbol_table: Arc<RwLock<SymbolTable>>,
}

impl Interpreter {
    /// Creates a new Interpreter. \
    /// Example
    /// ```rust
    /// use cara::back::Interpreter;
    /// let interpreter = Interpreter::new();
    /// ```
    pub fn new() -> Self {
        Self {
            current_symbol_table: Arc::new(RwLock::new(SymbolTable::new(None))),
        }
    }
}

impl Interpreter {
    /// Visits the AST node with the visitor mode. \
    /// Example
    /// ```rust
    /// use cara::back::Interpreter;
    /// use cara::front::{Parser,Lexer};
    ///
    /// let mut lexer = Lexer::new("1+1".into());
    /// let mut parser = Parser::new(lexer);
    /// let node = parser.parse_compile_unit();
    ///
    /// let mut interpreter = Interpreter::new();
    /// assert_eq!(interpreter.visit(node),2);
    /// ```
    pub fn visit(&mut self, node: Box<AstNodes>) -> Result<CrValue> {
        match *node {
            AstNodes::Assign(id, index, value) => self.visit_assign(id, index, value),
            AstNodes::BinaryOp(left, op, right) => self.visit_binary_op(left, op, right),
            AstNodes::CompileUnit(statements) => self.visit_compile_unit(statements),
            AstNodes::Number(num) => self.visit_number(num),
            AstNodes::UnaryOp(op, val) => self.visit_unary_op(op, val),
            AstNodes::VarDef(id, init_value) => self.visit_var_def(id, init_value),
            AstNodes::ConstDef(id, const_value) => self.visit_const_def(id, const_value),
            AstNodes::ReadVar(id) => self.visit_read_var(id),
            AstNodes::FunctionDef(id, params, body) => self.visit_function_def(id, params, body),
            AstNodes::Call(id, args) => self.visit_call(id, args),
            AstNodes::Return(value) => self.visit_return(value),
            AstNodes::If(condition, then_block, else_block) => {
                self.visit_if(condition, then_block, else_block)
            }
            AstNodes::For(variable, start, end, body) => self.visit_for(variable, start, end, body),
            AstNodes::List(value_list) => self.visit_list(value_list),
            AstNodes::Index(id, index) => self.visit_index(id, index),
            AstNodes::TemplateList(template, num) => self.visit_template_list(template, num),
            AstNodes::While(condition, body) => self.visit_while(condition, body),
            AstNodes::Break => self.visit_break(),
            AstNodes::Continue => self.visit_continue(),
        }
    }

    fn visit_break(&mut self) -> Result<CrValue> {
        Err(Error::Break)
    }

    fn visit_continue(&mut self) -> Result<CrValue> {
        Err(Error::Continue)
    }

    fn visit_while(
        &mut self,
        condition: Box<AstNodes>,
        body: Vec<Box<AstNodes>>,
    ) -> Result<CrValue> {
        while self.visit(condition.clone())?.into_int()? > BigInt::ZERO {
            let last_symbol_table = self.current_symbol_table.clone();
            let temp_symbol_table = Arc::new(RwLock::new(SymbolTable::new(Some(
                last_symbol_table.clone(),
            ))));

            self.current_symbol_table = temp_symbol_table;
            for item in &body {
                if let Err(error) = self.visit(item.clone()) {
                    match error {
                        Error::Break => return Ok(CrValue::Void),
                        Error::Continue => break,
                        _ => return Err(error),
                    }
                }
            }
            self.current_symbol_table = last_symbol_table;
        }
        Ok(CrValue::Void)
    }

    fn visit_index(&mut self, id: String, index: Box<AstNodes>) -> Result<CrValue> {
        let index = self.visit(index)?.into_int()?;
        if let Some(symbol) = self.current_symbol_table.write().get(&id) {
            let array = symbol.get_value()?;
            Ok(
                *array.into_list()?.1[*index.to_u64_digits().1.get(0).unwrap_or(&0) as usize]
                    .clone(),
            )
        } else {
            Err(Error::SymbolNotFound)
        }
    }

    fn visit_template_list(
        &mut self,
        template: Box<AstNodes>,
        num: Box<AstNodes>,
    ) -> Result<CrValue> {
        let template_value = Box::new(self.visit(template)?);
        let num = *self
            .visit(num)?
            .into_int()?
            .to_u64_digits()
            .1
            .get(0)
            .unwrap_or(&0);
        let mut values = Vec::with_capacity(num as usize);
        for _ in 0..num {
            values.push(template_value.clone());
        }
        Ok(CrValue::List(num as usize, values))
    }

    fn visit_list(&mut self, value_list: Vec<Box<AstNodes>>) -> Result<CrValue> {
        let mut values = Vec::with_capacity(value_list.len());
        for value in value_list {
            values.push(Box::new(self.visit(value)?));
        }
        Ok(CrValue::List(values.len(), values))
    }

    fn visit_for(
        &mut self,
        variable: String,
        start: Box<AstNodes>,
        end: Box<AstNodes>,
        body: Vec<Box<AstNodes>>,
    ) -> Result<CrValue> {
        let start = self.visit(start)?.into_int()?;
        let end = self.visit(end)?.into_int()?;

        let need = *(end - start.clone()).to_u64_digits().1.get(0).unwrap_or(&0) as usize;

        for pos in 0..need {
            let last_symbol_table = self.current_symbol_table.clone();
            let temp_symbol_table = Arc::new(RwLock::new(SymbolTable::new(Some(
                last_symbol_table.clone(),
            ))));

            temp_symbol_table.write().insert(Symbol::Const(
                variable.clone(),
                CrValue::Number(start.clone() + pos),
            ));

            self.current_symbol_table = temp_symbol_table;
            for item in &body {
                if let Err(error) = self.visit(item.clone()) {
                    match error {
                        Error::Break => return Ok(CrValue::Void),
                        Error::Continue => break,
                        _ => return Err(error),
                    }
                }
            }
            self.current_symbol_table = last_symbol_table;
        }
        Ok(CrValue::Void)
    }

    fn visit_if(
        &mut self,
        condition: Box<AstNodes>,
        then_block: Vec<Box<AstNodes>>,
        else_block: Vec<Box<AstNodes>>,
    ) -> Result<CrValue> {
        let condition = self.visit(condition)?;
        if condition.into_int()? > BigInt::ZERO {
            let last_symbol_table = self.current_symbol_table.clone();
            let temp_symbol_table = Arc::new(RwLock::new(SymbolTable::new(Some(
                last_symbol_table.clone(),
            ))));
            self.current_symbol_table = temp_symbol_table;
            let ret = self.visit_compile_unit(then_block);
            self.current_symbol_table = last_symbol_table;
            ret
        } else {
            let last_symbol_table = self.current_symbol_table.clone();
            let temp_symbol_table = Arc::new(RwLock::new(SymbolTable::new(Some(
                last_symbol_table.clone(),
            ))));
            self.current_symbol_table = temp_symbol_table;
            let ret = self.visit_compile_unit(else_block);
            self.current_symbol_table = last_symbol_table;
            ret
        }
    }

    fn visit_assign(
        &mut self,
        id: String,
        index: Option<Box<AstNodes>>,
        value: Box<AstNodes>,
    ) -> Result<CrValue> {
        let value = self.visit(value)?;
        if let Some(index) = index {
            let index = self.visit(index)?.into_int()?;
            let mut symbol_table = self.current_symbol_table.write();
            let array = symbol_table
                .get_mut(&id)
                .expect(format!("Unable to find list variable {}!", id).as_str());
            array.get_value_mut()?.into_list_mut()?.1
                [*index.to_u64_digits().1.get(0).unwrap_or(&0) as usize] = Box::new(value);
        } else {
            self.current_symbol_table
                .write()
                .get_mut(&id)
                .expect(format!("Unable to find variable {}!", id).as_str())
                .try_assign(value)?;
        }
        Ok(CrValue::Void)
    }

    fn visit_binary_op(
        &mut self,
        left: Box<AstNodes>,
        op: &'static str,
        right: Box<AstNodes>,
    ) -> Result<CrValue> {
        let left = self.visit(left)?.into_int()?;
        let right = self.visit(right)?.into_int()?;

        Ok(CrValue::Number(match op {
            "+" => left + right,
            "-" => left - right,
            "*" => left * right,
            "/" => left / right,
            "==" => BigInt::from((left == right) as u8),
            "!=" => BigInt::from((left != right) as u8),
            "<=" => BigInt::from((left <= right) as u8),
            ">=" => BigInt::from((left >= right) as u8),
            "<" => BigInt::from((left < right) as u8),
            ">" => BigInt::from((left > right) as u8),
            "||" => BigInt::from((left > BigInt::ZERO || right > BigInt::ZERO) as u8),
            "&&" => BigInt::from((left > BigInt::ZERO && right > BigInt::ZERO) as u8),
            "%" => left % right,
            "<<" => left << *right.to_u64_digits().1.get(0).unwrap_or(&0),
            ">>" => left >> *right.to_u64_digits().1.get(0).unwrap_or(&0),
            _ => return Err(Error::UnknownOperator),
        }))
    }

    fn visit_compile_unit(&mut self, statements: Vec<Box<AstNodes>>) -> Result<CrValue> {
        for item in statements {
            self.visit(item)?;
        }
        Ok(CrValue::Void)
    }

    fn visit_number(&mut self, num: BigInt) -> Result<CrValue> {
        Ok(CrValue::Number(num))
    }

    fn visit_unary_op(&mut self, op: &'static str, val: Box<AstNodes>) -> Result<CrValue> {
        let val = self.visit(val)?.into_int()?;
        Ok(CrValue::Number(match op {
            "-" => -val,
            _ => val,
        }))
    }

    fn visit_const_def(&mut self, id: String, const_value: Box<AstNodes>) -> Result<CrValue> {
        let const_value = self.visit(const_value)?;
        #[cfg(debug_assertions)]
        println!("Defined constant {} with value {}!", id, const_value);
        self.current_symbol_table
            .write()
            .insert(Symbol::Const(id, const_value));
        Ok(CrValue::Void)
    }

    fn visit_var_def(&mut self, id: String, init_value: Box<AstNodes>) -> Result<CrValue> {
        let init_value = self.visit(init_value)?;
        #[cfg(debug_assertions)]
        println!("Defined variable {} with value {}!", id, init_value);
        self.current_symbol_table
            .write()
            .insert(Symbol::Var(id, init_value));
        Ok(CrValue::Void)
    }

    fn visit_read_var(&mut self, id: String) -> Result<CrValue> {
        if let Some(symbol) = self.current_symbol_table.read().get(&id) {
            return symbol.get_value().cloned();
        }
        Err(Error::SymbolNotFound)
    }

    fn visit_function_def(
        &mut self,
        id: String,
        params: Vec<String>,
        body: Vec<Box<AstNodes>>,
    ) -> Result<CrValue> {
        self.current_symbol_table
            .write()
            .insert(Symbol::Function(id, params, body));
        Ok(CrValue::Void)
    }

    fn visit_call(&mut self, id: String, args: Vec<Box<AstNodes>>) -> Result<CrValue> {
        match id.as_str() {
            "print" => {
                self.print(args)?;
                return Ok(CrValue::Void);
            }
            "append" => {
                self.append(args)?;
                return Ok(CrValue::Void);
            }
            "insert" => {
                self.insert(args)?;
                return Ok(CrValue::Void);
            }
            "len" => {
                return self.len(args);
            }
            "remove" => {
                return self.remove(args);
            }
            _ => {}
        }

        let symbol_table = self.current_symbol_table.read();
        let function = symbol_table
            .get(id.as_str())
            .expect(format!("Unable to find function {id}!").as_str())
            .clone();
        drop(symbol_table);
        match function {
            Symbol::Function(_, params, body) => {
                let last_symbol_table = self.current_symbol_table.clone();
                let new_symbol_table = Arc::new(RwLock::new(SymbolTable::new(Some(
                    last_symbol_table.clone(),
                ))));
                for (name, value) in zip(params, args) {
                    new_symbol_table
                        .write()
                        .insert(Symbol::Const(name, self.visit(value)?));
                }
                self.current_symbol_table = new_symbol_table;
                for item in body {
                    if let Err(error) = self.visit(item) {
                        if let Error::Return(value) = error {
                            //println!("return with {:?}", value);
                            self.current_symbol_table = last_symbol_table;
                            return Ok(value);
                        }
                        return Err(error);
                    }
                }
                self.current_symbol_table = last_symbol_table;
                return Ok(CrValue::Void);
            }
            _ => return Err(Error::SymbolNotFound),
        }
    }

    fn visit_return(&mut self, value: Box<AstNodes>) -> Result<CrValue> {
        let val = self.visit(value)?;
        Err(Error::Return(val))
    }
}
