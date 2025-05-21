use alloc::{string::String, vec::Vec};
use dashu_int::IBig;

use super::{KeywordTypes, Lexer, Token};
use crate::ast::{AstNodes, Op};

/// This is a simple and stupid LL(1) parser.
pub struct Parser {
    pub lexer: Lexer,
    current_token: Option<Token>,
}

impl Parser {
    /// Creates a parser with a lexer. \
    /// Expample
    /// ```rust
    /// use cara::frontend::{Lexer,Parser};
    /// let lexer = Lexer::new("1+1".into());
    /// let mut parser = Parser::new(lexer);
    /// ```
    #[must_use]
    pub fn new(mut lexer: Lexer) -> Self {
        let tok = lexer.get_token();
        Self {
            lexer,
            current_token: tok,
        }
    }

    fn advance(&mut self) {
        self.current_token = self.lexer.get_token();
    }

    fn eat(&mut self, token: Token) -> Token {
        if let Some(t) = self.current_token.as_ref() {
            let ok = match (t, token.clone()) {
                (Token::Id(_), Token::Id(_))
                | (Token::Number(_), Token::Number(_))
                | (Token::Operator(_), Token::Operator(_)) => true,
                _ => token == *t,
            };
            if ok {
                let t = t.clone();
                self.advance();
                t
            } else {
                panic!("Expected {:?}, but found {:?}", token, t);
            }
        } else {
            panic!("Unexpected end of input");
        }
    }

    // FIXME: fix this test
    /// Returns the whole AST.
    ///
    /// ### Expample
    /// ```rust,no_run
    /// use cara::frontend::{Lexer, Parser};
    /// let lexer = Lexer::new("1-(5+7)/2+2*3-100".into());
    /// let mut parser = Parser::new(lexer);
    /// let ast = parser.parse_compile_unit();
    /// println!("{:#?}",ast);
    /// ```
    ///
    /// ### Output:
    /// ```ignore
    /// CompileUnit(
    ///     BinaryOp(
    ///         BinaryOp(
    ///             BinaryOp(
    ///                 Number(
    ///                     1,
    ///                 ),
    ///                 '-',
    ///                 BinaryOp(
    ///                     BinaryOp(
    ///                         Number(
    ///                             5,
    ///                         ),
    ///                         '+',
    ///                         Number(
    ///                             7,
    ///                         ),
    ///                     ),
    ///                     '/',
    ///                     Number(
    ///                         2,
    ///                     ),
    ///                 ),
    ///             ),
    ///             '+',
    ///             BinaryOp(
    ///                 Number(
    ///                     2,
    ///                 ),
    ///                 '*',
    ///                 Number(
    ///                     3,
    ///                 ),
    ///             ),
    ///         ),
    ///         '-',
    ///         Number(
    ///             100,
    ///         ),
    ///     ),
    /// )
    /// ```
    pub fn parse_compile_unit(&mut self) -> (AstNodes, Vec<String>) {
        let mut children = Vec::new();
        while self.current_token.is_some() {
            children.push(self.parse_statement());
        }
        (AstNodes::CompileUnit(children), self.lexer.string_table())
    }

    fn parse_statement(&mut self) -> AstNodes {
        if let Some(current_token) = self.current_token.clone() {
            match current_token {
                Token::Keyword(key_word) => {
                    return match key_word {
                        KeywordTypes::Var => self.parse_var(),
                        KeywordTypes::Const => self.parse_const(),
                        KeywordTypes::Fn => self.parse_function(),
                        KeywordTypes::Return => self.parse_return(),
                        KeywordTypes::If => self.parse_if(),
                        KeywordTypes::For => self.parse_for(),
                        KeywordTypes::Break => self.parse_break(),
                        KeywordTypes::Continue => self.parse_continue(),
                        KeywordTypes::While => self.parse_while(),
                        _ => unreachable!(),
                    }
                }
                Token::Id(_) => {
                    if self.lexer.current_char() == '(' {
                        return self.parse_call(true);
                    }
                    return self.parse_assign();
                }
                _ => panic!("Syntax error {:?}!", current_token),
            }
        }
        panic!("Nothing to parse!");
    }

    fn parse_break(&mut self) -> AstNodes {
        self.advance();
        AstNodes::Break
    }

    fn parse_continue(&mut self) -> AstNodes {
        self.advance();
        AstNodes::Continue
    }

    fn parse_while(&mut self) -> AstNodes {
        self.advance();
        let condition = self.parse_expr();
        self.eat(Token::LBrace);
        let body = self.parse_block();
        self.eat(Token::RBrace);
        AstNodes::While(condition.into(), body)
    }

    fn parse_list(&mut self) -> AstNodes {
        self.eat(Token::LBracket);
        let mut value_list = Vec::new();

        if self.current_token != Some(Token::RBracket) {
            let first_value = self.parse_expr();

            if self.current_token == Some(Token::Semi) {
                self.advance();
                let num = self.parse_expr();
                self.eat(Token::RBracket);
                return AstNodes::TemplateList(first_value.into(), num.into());
            }

            value_list.push(first_value);
            while let Some(token) = self.current_token.clone() {
                if token == Token::RBracket {
                    break;
                }
                self.eat(Token::Comma);
                let value = self.parse_expr();
                value_list.push(value);
            }
        }

        self.eat(Token::RBracket);
        AstNodes::List(value_list)
    }

    fn parse_for(&mut self) -> AstNodes {
        self.advance();

        let variable = self.eat(Token::Id(0)).as_ident().unwrap();

        self.eat(Token::Keyword(KeywordTypes::In));

        self.eat(Token::LParen);
        let start = self.parse_expr();
        self.eat(Token::Comma);
        let end = self.parse_expr();

        let step = if self.current_token == Some(Token::Comma) {
            self.advance();
            self.parse_expr()
        } else {
            AstNodes::Number(IBig::from(1))
        };

        self.eat(Token::RParen);

        self.eat(Token::LBrace);
        let body = self.parse_block();
        self.eat(Token::RBrace);

        AstNodes::For(variable, start.into(), end.into(), step.into(), body)
    }

    fn parse_if(&mut self) -> AstNodes {
        self.advance();
        //self.eat(Token::LParen);
        let condition = self.parse_expr();
        //self.eat(Token::RParen);

        self.eat(Token::LBrace);
        let then_block = self.parse_block();
        self.eat(Token::RBrace);

        let else_block = if self.current_token == Some(Token::Keyword(KeywordTypes::Else)) {
            self.advance();
            self.eat(Token::LBrace);
            let block = self.parse_block();
            self.eat(Token::RBrace);
            block
        } else {
            Vec::new()
        };

        AstNodes::If(condition.into(), then_block, else_block)
    }

    fn parse_block(&mut self) -> Vec<AstNodes> {
        let mut children = Vec::new();
        while self.current_token.is_some() {
            if self.current_token == Some(Token::RBrace) {
                break;
            }
            children.push(self.parse_statement());
        }
        children
    }

    fn parse_return(&mut self) -> AstNodes {
        self.advance();
        let expr = self.parse_expr();
        self.eat(Token::Semi);
        AstNodes::Return(expr.into())
    }

    fn parse_function(&mut self) -> AstNodes {
        self.advance();
        let id = self.eat(Token::Id(0)).as_ident().unwrap();

        self.eat(Token::LParen);
        let params = self.parse_params();
        self.eat(Token::RParen);

        self.eat(Token::LBrace);

        let mut body = Vec::new();
        while let Some(current) = self.current_token.clone() {
            if current == Token::RBrace {
                break;
            }
            body.push(self.parse_statement());
        }

        self.eat(Token::RBrace);

        AstNodes::FunctionDef(id, params, body)
    }

    fn parse_params(&mut self) -> Vec<usize> {
        let mut params = Vec::new();
        while let Some(current_token) = self.current_token.clone() {
            match current_token {
                Token::Id(id) => {
                    params.push(id);
                    self.advance();
                    if let Some(token) = self.current_token.clone() {
                        match token {
                            Token::Comma => self.advance(),
                            Token::RParen => break,
                            _ => panic!("Expected identifier or ',', found {token:?}!"),
                        }
                    }
                }
                Token::RParen => break,
                _ => panic!(
                    "Syntax error! Expected ID or ',', found {:?}!",
                    current_token
                ),
            }
        }
        params
    }

    fn parse_const(&mut self) -> AstNodes {
        self.advance();

        let id = self.eat(Token::Id(0)).as_ident().unwrap();

        self.eat(Token::Assign);

        let init_val = self.parse_expr();

        self.eat(Token::Semi);

        AstNodes::ConstDef(id, init_val.into())
    }

    fn parse_var(&mut self) -> AstNodes {
        self.advance();

        let id = self.eat(Token::Id(0)).as_ident().unwrap();

        self.eat(Token::Assign);

        let init_val = if self.current_token == Some(Token::LBracket) {
            self.parse_list()
        } else {
            self.parse_expr()
        };

        self.eat(Token::Semi);

        AstNodes::VarDef(id, init_val.into())
    }

    fn parse_assign(&mut self) -> AstNodes {
        let id = self.eat(Token::Id(0)).as_ident().unwrap();

        let index = if self.current_token == Some(Token::LBracket) {
            self.advance();
            let index = self.parse_expr();
            self.eat(Token::RBracket);
            Some(index.into())
        } else {
            None
        };

        self.eat(Token::Assign);

        let expr = self.parse_expr();

        self.eat(Token::Semi);

        AstNodes::Assign(id, index, expr.into())
    }

    fn parse_expr(&mut self) -> AstNodes {
        let mut node = self.parse_eq_expr();
        while let Some(current_token) = self.current_token.clone() {
            if let Some(op) = current_token.as_operator() {
                match op {
                    Op::Or | Op::And => {
                        self.advance();
                        node = AstNodes::BinaryOp(node.into(), op, self.parse_eq_expr().into());
                    }
                    _ => break,
                }
            } else {
                break;
            }
        }
        node
    }

    fn parse_eq_expr(&mut self) -> AstNodes {
        let mut node = self.parse_add_expr();
        while let Some(current_token) = self.current_token.clone() {
            if let Some(op) = current_token.as_operator() {
                match op {
                    Op::Eq | Op::Ne | Op::Ge | Op::Le | Op::Lt | Op::Gt => {
                        self.advance();
                        node = AstNodes::BinaryOp(node.into(), op, self.parse_add_expr().into());
                    }
                    _ => break,
                }
            } else {
                break;
            }
        }
        node
    }

    fn parse_add_expr(&mut self) -> AstNodes {
        let mut node = self.parse_move_expr();
        while let Some(current_token) = self.current_token.clone() {
            if let Some(op) = current_token.as_operator() {
                match op {
                    Op::Add | Op::Sub => {
                        self.advance();
                        node = AstNodes::BinaryOp(node.into(), op, self.parse_move_expr().into());
                    }
                    _ => break,
                }
            } else {
                break;
            }
        }
        node
    }

    fn parse_move_expr(&mut self) -> AstNodes {
        let mut node = self.parse_term();
        while let Some(current_token) = self.current_token.clone() {
            if let Some(op) = current_token.as_operator() {
                match op {
                    Op::LShift | Op::RShift => {
                        self.advance();
                        node = AstNodes::BinaryOp(node.into(), op, self.parse_term().into());
                    }
                    _ => break,
                }
            } else {
                break;
            }
        }
        node
    }

    fn parse_term(&mut self) -> AstNodes {
        let mut node = self.parse_factor();
        while let Some(current_token) = self.current_token.clone() {
            if let Some(op) = current_token.as_operator() {
                match op {
                    Op::Mul | Op::Div | Op::Rem => {
                        self.advance();
                        node = AstNodes::BinaryOp(node.into(), op, self.parse_factor().into());
                    }
                    _ => break,
                }
            } else {
                break;
            }
        }
        node
    }

    fn parse_factor(&mut self) -> AstNodes {
        let token = self.current_token.clone().unwrap();
        match token {
            Token::Number(num) => {
                self.advance();
                AstNodes::Number(num)
            }
            Token::LParen => {
                self.advance();
                let node = self.parse_expr();
                self.eat(Token::RParen);
                node
            }
            Token::Operator(op) => match op {
                Op::Add | Op::Sub => {
                    self.advance();
                    let node = self.parse_expr();
                    AstNodes::UnaryOp(op, node.into())
                }
                _ => panic!("Unexpected unary operator {:?}!", op),
            },
            Token::Id(id) => {
                if self.lexer.current_char() == '(' {
                    self.parse_call(false)
                } else if self.lexer.current_char() == '[' {
                    self.advance();
                    self.advance();
                    let index_value = self.parse_expr();
                    self.eat(Token::RBracket);
                    AstNodes::Index(id, index_value.into())
                } else {
                    self.advance();
                    AstNodes::ReadVar(id)
                }
            }
            _ => panic!("Syntax error {:?}!", token),
        }
    }

    fn parse_call(&mut self, stmt: bool) -> AstNodes {
        let id = self.eat(Token::Id(0)).as_ident().unwrap();

        self.eat(Token::LParen);

        let args = self.parse_args();

        self.eat(Token::RParen);

        if stmt {
            self.eat(Token::Semi);
        }

        AstNodes::Call(id, args)
    }

    fn parse_args(&mut self) -> Vec<AstNodes> {
        let mut args = Vec::new();
        while let Some(current_token) = self.current_token.clone() {
            if current_token == Token::RParen {
                break;
            }

            args.push(self.parse_expr());

            if self.current_token == Some(Token::Comma) {
                self.advance();
            } else {
                break;
            }
        }
        args
    }
}
