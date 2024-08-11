use alloc::{borrow::ToOwned, vec};
use alloc::{format, rc::Rc, string::String, vec::Vec};
use core::{cell::RefCell, iter::zip};
use dashu_int::IBig;
use value::CrValue;

use crate::ast::AstNodes;
use result::{Error, Result};
use scope::{Symbol, SymbolTable};

mod builtins;
mod result;
mod scope;
mod value;

pub use builtins::set_printer;

/// The interpreter
pub struct Interpreter {
    current_symbol_table: Rc<RefCell<SymbolTable>>,
}

impl Interpreter {
    /// Creates a new Interpreter. \
    /// Example
    /// ```rust
    /// use cara::backend::Interpreter;
    /// let interpreter = Interpreter::new();
    /// ```
    pub fn new() -> Self {
        Self {
            current_symbol_table: Rc::new(RefCell::new(SymbolTable::new(None))),
        }
    }
}

impl Interpreter {
    /// Visits the AST node with the visitor mode. \
    /// Example
    /// ```rust
    /// use cara::backend::Interpreter;
    /// use cara::frontend::{Parser,Lexer};
    ///
    /// let mut lexer = Lexer::new("1+1".into());
    /// let mut parser = Parser::new(lexer);
    /// let node = parser.parse_compile_unit();
    ///
    /// let mut interpreter = Interpreter::new();
    /// assert_eq!(interpreter.visit(node),2);
    /// ```
    #[inline]
    pub fn visit(&mut self, node: &AstNodes) -> Result<CrValue> {
        match node {
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
            AstNodes::For(variable, start, end, step, body) => {
                self.visit_for(variable, start, end, step, body)
            }
            AstNodes::List(value_list) => self.visit_list(value_list),
            AstNodes::Index(id, index) => self.visit_index(id, index),
            AstNodes::TemplateList(template, num) => self.visit_template_list(template, num),
            AstNodes::While(condition, body) => self.visit_while(condition, body),
            AstNodes::Break => self.visit_break(),
            AstNodes::Continue => self.visit_continue(),
        }
    }

    #[inline]
    fn visit_break(&mut self) -> Result<CrValue> {
        Err(Error::Break)
    }

    #[inline]
    fn visit_continue(&mut self) -> Result<CrValue> {
        Err(Error::Continue)
    }

    fn visit_while(&mut self, condition: &Rc<AstNodes>, body: &Vec<AstNodes>) -> Result<CrValue> {
        let last_symbol_table = self.current_symbol_table.to_owned();
        let temp_symbol_table = SymbolTable::new(Some(last_symbol_table.clone()));
        let temp_symbol_table = Rc::new(RefCell::new(temp_symbol_table));

        while self.visit(condition)?.into_int()? > IBig::ZERO {
            self.current_symbol_table = temp_symbol_table.clone();
            for item in body {
                if let Err(error) = self.visit(item) {
                    match error {
                        Error::Break => return Ok(CrValue::Void),
                        Error::Continue => break,
                        _ => return Err(error),
                    }
                }
            }
            self.current_symbol_table = last_symbol_table.clone();
            last_symbol_table.borrow_mut().clear();
        }
        Ok(CrValue::Void)
    }

    #[inline]
    fn visit_index(&mut self, id: &String, index: &Rc<AstNodes>) -> Result<CrValue> {
        let number = self.visit(index)?.into_int()?;
        let index = usize::try_from(&number).unwrap();
        let value = self
            .current_symbol_table
            .borrow()
            .symbol_crvalue_list_item(&id, index)?;
        Ok(value)
    }

    #[inline]
    fn visit_template_list(
        &mut self,
        template: &Rc<AstNodes>,
        size: &Rc<AstNodes>,
    ) -> Result<CrValue> {
        let template_value = self.visit(template)?;
        let number = self.visit(size)?.into_int()?;
        let size = usize::try_from(&number).unwrap();
        Ok(CrValue::List(vec![template_value; size]))
    }

    #[inline]
    fn visit_list(&mut self, value_list: &Vec<AstNodes>) -> Result<CrValue> {
        let values = value_list
            .iter()
            .map(|value| self.visit(value))
            .collect::<Result<Vec<CrValue>>>()?;
        Ok(CrValue::List(values))
    }

    fn visit_for(
        &mut self,
        variable: &String,
        start: &Rc<AstNodes>,
        end: &Rc<AstNodes>,
        step: &Rc<AstNodes>,
        body: &Vec<AstNodes>,
    ) -> Result<CrValue> {
        let start = self.visit(&start)?.into_int()?;
        let end = self.visit(&end)?.into_int()?;
        let step = self.visit(&step)?.into_int()?;

        let start = isize::try_from(&start).unwrap();
        let end = isize::try_from(&end).unwrap();
        let step = usize::try_from(&step).unwrap();

        let last_symbol_table = self.current_symbol_table.to_owned();
        let temp_symbol_table = SymbolTable::new(Some(last_symbol_table.clone()));
        let temp_symbol_table = Rc::new(RefCell::new(temp_symbol_table));

        for number in (start..end).step_by(step) {
            let number = IBig::from(number);
            let value = Symbol::Const(variable.clone(), CrValue::Number(number));
            temp_symbol_table.borrow_mut().insert(value);

            self.current_symbol_table = temp_symbol_table.clone();
            for item in body {
                if let Err(error) = self.visit(item) {
                    match error {
                        Error::Break => return Ok(CrValue::Void),
                        Error::Continue => break,
                        _ => return Err(error),
                    }
                }
            }
            self.current_symbol_table = last_symbol_table.clone();
            temp_symbol_table.borrow_mut().clear();
        }
        Ok(CrValue::Void)
    }

    fn visit_if(
        &mut self,
        condition: &Rc<AstNodes>,
        then_block: &Vec<AstNodes>,
        else_block: &Vec<AstNodes>,
    ) -> Result<CrValue> {
        let condition = self.visit(condition)?;
        let last_symbol_table = self.current_symbol_table.to_owned();
        let temp_symbol_table = SymbolTable::new(Some(last_symbol_table.clone()));
        let temp_symbol_table = Rc::new(RefCell::new(temp_symbol_table));
        self.current_symbol_table = temp_symbol_table.clone();
        let result = if condition.into_int()? > IBig::ZERO {
            self.visit_compile_unit(then_block)
        } else {
            self.visit_compile_unit(else_block)
        };
        self.current_symbol_table = last_symbol_table;
        result
    }

    fn visit_assign(
        &mut self,
        id: &String,
        index: &Option<Rc<AstNodes>>,
        value: &Rc<AstNodes>,
    ) -> Result<CrValue> {
        let value = self.visit(&value)?;
        if let Some(index) = index {
            let number = self.visit(index)?.into_int()?;
            let index = usize::try_from(&number).unwrap();
            self.current_symbol_table
                .borrow_mut()
                .symbol_list_modify(&id, index, value)?;
        } else {
            self.current_symbol_table
                .borrow_mut()
                .symbol_assign(&id, value)?;
        }
        Ok(CrValue::Void)
    }

    fn visit_binary_op(
        &mut self,
        left: &Rc<AstNodes>,
        op: &&'static str,
        right: &Rc<AstNodes>,
    ) -> Result<CrValue> {
        let left = self.visit(&left)?.into_int()?;
        let right = self.visit(&right)?.into_int()?;

        Ok(CrValue::Number(match *op {
            "+" => left + right,
            "-" => left - right,
            "*" => left * right,
            "/" => left / right,
            "==" => IBig::from((left == right) as u8),
            "!=" => IBig::from((left != right) as u8),
            "<=" => IBig::from((left <= right) as u8),
            ">=" => IBig::from((left >= right) as u8),
            "<" => IBig::from((left < right) as u8),
            ">" => IBig::from((left > right) as u8),
            "||" => IBig::from((left > IBig::ZERO || right > IBig::ZERO) as u8),
            "&&" => IBig::from((left > IBig::ZERO && right > IBig::ZERO) as u8),
            "%" => left % right,
            "<<" => left << usize::try_from(&right).unwrap(),
            ">>" => left >> usize::try_from(&right).unwrap(),
            _ => return Err(Error::UnknownOperator),
        }))
    }

    #[inline]
    fn visit_compile_unit(&mut self, statements: &Vec<AstNodes>) -> Result<CrValue> {
        statements
            .iter()
            .map(|item| self.visit(item))
            .collect::<Result<Vec<CrValue>>>()?;
        Ok(CrValue::Void)
    }

    #[inline]
    fn visit_number(&mut self, num: &IBig) -> Result<CrValue> {
        Ok(CrValue::Number(num.clone()))
    }

    #[inline]
    fn visit_unary_op(&mut self, op: &&'static str, val: &Rc<AstNodes>) -> Result<CrValue> {
        let value = self.visit(&val)?.into_int()?;
        let result = match *op {
            "-" => -value,
            _ => value,
        };
        Ok(CrValue::Number(result))
    }

    #[inline]
    fn visit_const_def(&mut self, id: &String, const_value: &Rc<AstNodes>) -> Result<CrValue> {
        let const_value = self.visit(const_value)?;
        self.current_symbol_table
            .borrow_mut()
            .insert(Symbol::Const(id.clone(), const_value));
        Ok(CrValue::Void)
    }

    #[inline]
    fn visit_var_def(&mut self, id: &String, init_value: &Rc<AstNodes>) -> Result<CrValue> {
        let init_value = self.visit(init_value)?;
        self.current_symbol_table
            .borrow_mut()
            .insert(Symbol::Var(id.clone(), init_value));
        Ok(CrValue::Void)
    }

    #[inline]
    fn visit_read_var(&mut self, id: &String) -> Result<CrValue> {
        let value = self.current_symbol_table.borrow().symbol_clone_value(id)?;
        Ok(value)
    }

    #[inline]
    fn visit_function_def(
        &mut self,
        id: &String,
        params: &Vec<String>,
        body: &Vec<AstNodes>,
    ) -> Result<CrValue> {
        self.current_symbol_table
            .borrow_mut()
            .insert(Symbol::Function(id.clone(), params.clone(), body.to_vec()));
        Ok(CrValue::Void)
    }

    fn visit_call(&mut self, id: &String, args: &Vec<AstNodes>) -> Result<CrValue> {
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

        let function = self
            .current_symbol_table
            .borrow()
            .symbol_clone(id.as_str())
            .expect(format!("Unable to find function {id}!").as_str());

        match function {
            Symbol::Function(_, params, body) => {
                let last_symbol_table = self.current_symbol_table.to_owned();
                let new_symbol_table = Rc::new(RefCell::new(SymbolTable::new(Some(
                    last_symbol_table.clone(),
                ))));
                for (name, value) in zip(params, args) {
                    let value = Symbol::Const(name, self.visit(value)?);
                    new_symbol_table.borrow_mut().insert(value);
                }
                self.current_symbol_table = new_symbol_table;
                for item in &body {
                    if let Err(error) = self.visit(item) {
                        if let Error::Return(value) = error {
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

    #[inline]
    fn visit_return(&mut self, value: &Rc<AstNodes>) -> Result<CrValue> {
        let val = self.visit(&value)?;
        Err(Error::Return(val))
    }
}
