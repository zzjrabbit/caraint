use alloc::{borrow::ToOwned, vec};
use alloc::{rc::Rc, string::String, vec::Vec};
use core::iter::zip;
use dashu_int::IBig;
use value::CrValue;

use crate::ast::{AstNodes, Op};
use result::{Error, Result};
use scope::{Symbol, SymbolTable, SymbolTables};

mod builtins;
mod result;
mod scope;
mod value;

pub use builtins::set_printer;

/// The interpreter
pub struct Interpreter {
    symbol_tables: SymbolTables,
    string_table: Vec<String>,
}

impl Interpreter {
    /// Creates a new Interpreter. \
    /// Example
    /// ```rust
    /// use cara::backend::Interpreter;
    /// let interpreter = Interpreter::new();
    /// ```
    #[must_use]
    pub fn new(string_table: Vec<String>) -> Self {
        Self {
            symbol_tables: vec![SymbolTable::new()].into(),
            string_table,
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
            AstNodes::Assign(id, index, value) => self.visit_assign(*id, index.as_ref(), value),
            AstNodes::BinaryOp(left, op, right) => self.visit_binary_op(left, op, right),
            AstNodes::CompileUnit(statements) => self.visit_compile_unit(statements),
            AstNodes::Number(num) => Ok(CrValue::Number(num.clone())),
            AstNodes::UnaryOp(op, val) => self.visit_unary_op(op, val),
            AstNodes::VarDef(id, init_value) => self.visit_var_def(*id, init_value),
            AstNodes::ConstDef(id, const_value) => self.visit_const_def(*id, const_value),
            AstNodes::ReadVar(id) => self.visit_read_var(*id),
            AstNodes::FunctionDef(id, params, body) => self.visit_function_def(*id, params, body),
            AstNodes::Call(id, args) => self.visit_call(*id, args),
            AstNodes::Return(value) => self.visit_return(value),
            AstNodes::If(condition, then_block, else_block) => {
                self.visit_if(condition, then_block, else_block)
            }
            AstNodes::For(variable, start, end, step, body) => {
                self.visit_for(*variable, start, end, step, body)
            }
            AstNodes::List(value_list) => self.visit_list(value_list),
            AstNodes::Index(id, index) => self.visit_index(*id, index),
            AstNodes::TemplateList(template, num) => self.visit_template_list(template, num),
            AstNodes::While(condition, body) => self.visit_while(condition, body),
            AstNodes::Break => Err(Error::Break),
            AstNodes::Continue => Err(Error::Continue),
        }
    }

    #[inline]
    fn with_block<F, R>(&mut self, f: F) -> R
    where
        F: FnOnce(&mut Self) -> R,
    {
        let cur_index = self.symbol_tables.len();
        self.symbol_tables.0.push(SymbolTable::new());

        let result = f(self);

        debug_assert_eq!(self.symbol_tables.len(), cur_index + 1);
        self.symbol_tables.0.pop().unwrap();
        result
    }

    fn visit_while(&mut self, condition: &Rc<AstNodes>, body: &[AstNodes]) -> Result<CrValue> {
        self.with_block(|this| {
            while *this.visit(condition)?.as_int()? > IBig::ZERO {
                this.symbol_tables.clear_last();

                for item in body {
                    match this.visit(item) {
                        Ok(_) => (),
                        Err(Error::Break) => return Ok(CrValue::Void),
                        Err(Error::Continue) => break,
                        Err(e) => return Err(e),
                    }
                }
            }

            Ok(CrValue::Void)
        })
    }

    #[inline]
    fn visit_index(&mut self, id: usize, index: &Rc<AstNodes>) -> Result<CrValue> {
        let number = self.visit(index)?;
        let index = usize::try_from(number.as_int()?).unwrap();
        self.symbol_tables.symbol_crvalue_list_item(id, index)
    }

    #[inline]
    fn visit_template_list(
        &mut self,
        template: &Rc<AstNodes>,
        size: &Rc<AstNodes>,
    ) -> Result<CrValue> {
        let template_value = self.visit(template)?;
        let number = self.visit(size)?;
        let size = usize::try_from(number.as_int()?).unwrap();
        Ok(CrValue::List(vec![template_value; size]))
    }

    #[inline]
    fn visit_list(&mut self, value_list: &[AstNodes]) -> Result<CrValue> {
        let values = value_list
            .iter()
            .map(|value| self.visit(value))
            .collect::<Result<Vec<CrValue>>>()?;
        Ok(CrValue::List(values))
    }

    fn visit_for(
        &mut self,
        variable: usize,
        start: &Rc<AstNodes>,
        end: &Rc<AstNodes>,
        step: &Rc<AstNodes>,
        body: &[AstNodes],
    ) -> Result<CrValue> {
        let start = self.visit(start)?;
        let end = self.visit(end)?;
        let step = self.visit(step)?;

        let start = isize::try_from(start.as_int()?).unwrap();
        let end = isize::try_from(end.as_int()?).unwrap();
        let step = usize::try_from(step.as_int()?).unwrap();

        self.with_block(|this| {
            for number in (start..end).step_by(step) {
                this.symbol_tables.clear_last();

                let number = IBig::from(number);
                let value = Symbol::Const(variable.to_owned(), CrValue::Number(number));
                this.symbol_tables.insert_sym(value);

                for item in body {
                    match this.visit(item) {
                        Ok(_) => (),
                        Err(Error::Break) => return Ok(CrValue::Void),
                        Err(Error::Continue) => break,
                        Err(e) => return Err(e),
                    }
                }
            }
            Ok(CrValue::Void)
        })
    }

    fn visit_if(
        &mut self,
        condition: &Rc<AstNodes>,
        then_block: &[AstNodes],
        else_block: &[AstNodes],
    ) -> Result<CrValue> {
        let condition = self.visit(condition)?;
        self.with_block(|this| {
            if *condition.as_int()? > IBig::ZERO {
                this.visit_compile_unit(then_block)
            } else {
                this.visit_compile_unit(else_block)
            }
        })
    }

    fn visit_assign(
        &mut self,
        id: usize,
        index: Option<&Rc<AstNodes>>,
        value: &Rc<AstNodes>,
    ) -> Result<CrValue> {
        let value = self.visit(value)?;
        if let Some(index) = index {
            let number = self.visit(index)?;
            let index = usize::try_from(number.as_int()?).unwrap();
            self.symbol_tables.symbol_list_modify(id, index, value)?;
        } else {
            self.symbol_tables.symbol_assign(id, value)?;
        }
        Ok(CrValue::Void)
    }

    fn visit_binary_op(
        &mut self,
        left: &Rc<AstNodes>,
        op: &Op,
        right: &Rc<AstNodes>,
    ) -> Result<CrValue> {
        let left = self.visit(left)?;
        let left = left.as_int()?;
        let right = self.visit(right)?;
        let right = right.as_int()?;

        Ok(CrValue::Number(match op {
            Op::Add => left + right,
            Op::Sub => left - right,
            Op::Mul => left * right,
            Op::Div => left / right,
            Op::Eq => IBig::from(u8::from(left == right)),
            Op::Ne => IBig::from(u8::from(left != right)),
            Op::Le => IBig::from(u8::from(left <= right)),
            Op::Ge => IBig::from(u8::from(left >= right)),
            Op::Lt => IBig::from(u8::from(left < right)),
            Op::Gt => IBig::from(u8::from(left > right)),
            Op::Or => IBig::from(u8::from(*left > IBig::ZERO || *right > IBig::ZERO)),
            Op::And => IBig::from(u8::from(*left > IBig::ZERO && *right > IBig::ZERO)),
            Op::Mod => left % right,
            Op::LShift => left << usize::try_from(right).unwrap(),
            Op::RShift => left >> usize::try_from(right).unwrap(),
        }))
    }

    #[inline]
    fn visit_compile_unit(&mut self, statements: &[AstNodes]) -> Result<CrValue> {
        statements
            .iter()
            .map(|item| self.visit(item))
            .collect::<Result<Vec<CrValue>>>()?;
        Ok(CrValue::Void)
    }

    #[inline]
    fn visit_unary_op(&mut self, op: &Op, val: &Rc<AstNodes>) -> Result<CrValue> {
        let value = self.visit(val)?;
        let result = match op {
            Op::Sub => -value.as_int()?,
            _ => value.as_int()?.clone(),
        };
        Ok(CrValue::Number(result))
    }

    #[inline]
    fn visit_const_def(&mut self, id: usize, const_value: &Rc<AstNodes>) -> Result<CrValue> {
        let const_value = self.visit(const_value)?;
        self.symbol_tables
            .insert_sym(Symbol::Const(id.to_owned(), const_value));
        Ok(CrValue::Void)
    }

    #[inline]
    fn visit_var_def(&mut self, id: usize, init_value: &Rc<AstNodes>) -> Result<CrValue> {
        let init_value = self.visit(init_value)?;
        self.symbol_tables
            .insert_sym(Symbol::Var(id.to_owned(), init_value));
        Ok(CrValue::Void)
    }

    #[inline]
    fn visit_read_var(&self, id: usize) -> Result<CrValue> {
        let value = self.symbol_tables.symbol_clone_value(id)?;
        Ok(value)
    }

    #[inline]
    fn visit_function_def(
        &mut self,
        id: usize,
        params: &[usize],
        body: &[AstNodes],
    ) -> Result<CrValue> {
        let symbol = Symbol::Function(
            id.to_owned(),
            params.to_owned().into(),
            body.to_vec().into(),
        );
        self.symbol_tables.insert_sym(symbol);
        Ok(CrValue::Void)
    }

    fn visit_call(&mut self, id: usize, args: &[AstNodes]) -> Result<CrValue> {
        match self.string_table[id].as_str() {
            "print" => {
                self.print(args);
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
            .symbol_tables
            .symbol_clone(id)
            .unwrap_or_else(|_| panic!("Unable to find function {id}!"));

        match function {
            Symbol::Function(_, params, body) => self.with_block(|this| {
                for (name, value) in zip(params.as_ref(), args) {
                    let value = Symbol::Const(*name, this.visit(value)?);
                    this.symbol_tables.insert_sym(value);
                }
                for item in body.as_ref() {
                    if let Err(error) = this.visit(item) {
                        if let Error::Return(value) = error {
                            return Ok(value);
                        }
                        return Err(error);
                    }
                }
                Ok(CrValue::Void)
            }),
            _ => Err(Error::SymbolNotFound),
        }
    }

    #[inline]
    fn visit_return(&mut self, value: &Rc<AstNodes>) -> Result<CrValue> {
        let val = self.visit(value)?;
        Err(Error::Return(val))
    }
}
