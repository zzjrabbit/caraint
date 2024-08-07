use spin::RwLock;
use std::sync::Arc;

use crate::ast::AstNodes;
use scope::{Symbol, SymbolTable};

mod scope;

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
            current_symbol_table: Arc::new(RwLock::new(SymbolTable::new())),
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
    pub fn visit(&mut self, node: Box<AstNodes>) -> i64 {
        match node.as_ref().clone() {
            AstNodes::Assign(id, value) => self.visit_assign(id, value),
            AstNodes::BinaryOp(left, op, right) => self.visit_binary_op(left, op, right),
            AstNodes::CompileUnit(statements) => self.visit_compile_unit(statements),
            AstNodes::Number(num) => self.visit_number(num),
            AstNodes::UnaryOp(op, val) => self.visit_unary_op(op, val),
            AstNodes::VarDef(id, init_value) => self.visit_var_def(id, init_value),
            AstNodes::ConstDef(id, const_value) => self.visit_const_def(id, const_value),
            AstNodes::ReadVar(id) => self.visit_read_var(id),
        }
    }

    fn visit_assign(&mut self, id: String, value: Box<AstNodes>) -> i64 {
        let value = self.visit(value);
        self.current_symbol_table
            .write()
            .get_mut(&id)
            .expect(format!("Unable to find variable {}!", id).as_str())
            .try_assign(value);
        value
    }

    fn visit_binary_op(&mut self, left: Box<AstNodes>, op: char, right: Box<AstNodes>) -> i64 {
        let left = self.visit(left);
        let right = self.visit(right);

        match op {
            '+' => left + right,
            '-' => left - right,
            '*' => left * right,
            '/' => left / right,
            _ => panic!("Unknown opeartor {}!", op),
        }
    }

    fn visit_compile_unit(&mut self, statements: Vec<Box<AstNodes>>) -> i64 {
        for item in statements.iter() {
            self.visit(item.clone());
        }
        0
    }

    fn visit_number(&mut self, num: i64) -> i64 {
        num
    }

    fn visit_unary_op(&mut self, op: char, val: Box<AstNodes>) -> i64 {
        let val = self.visit(val);
        match op {
            '-' => -val,
            _ => val,
        }
    }

    fn visit_const_def(&mut self, id: String, const_value: Box<AstNodes>) -> i64 {
        let const_value = self.visit(const_value);
        println!("Defined constant {} with value {}!", id, const_value);
        self.current_symbol_table
            .write()
            .insert(Symbol::Const(id, const_value));
        const_value
    }

    fn visit_var_def(&mut self, id: String, init_value: Box<AstNodes>) -> i64 {
        let init_value = self.visit(init_value);
        println!("Defined variable {} with value {}!", id, init_value);
        self.current_symbol_table
            .write()
            .insert(Symbol::Var(id, init_value));
        init_value
    }

    fn visit_read_var(&mut self, id: String) -> i64 {
        if let Some(symbol) = self.current_symbol_table.read().get(&id) {
            symbol.get_value()
        } else {
            panic!("Unable to find variable or constant {}!", id)
        }
    }
}
