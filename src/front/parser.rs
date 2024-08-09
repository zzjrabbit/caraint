use alloc::{rc::Rc,string::String,vec::Vec};

use num_bigint::BigInt;

use crate::ast::*;

use super::{KeywordTypes, Lexer, Token};

/// This is a simple and stupid LL(1) parser.
pub struct Parser {
    lexer: Lexer,
    current_token: Option<Token>,
}

impl Parser {
    /// Creates a parser with a lexer. \
    /// Expample
    /// ```rust
    /// use cara::front::{Lexer,Parser};
    /// let lexer = Lexer::new("1+1".into());
    /// let mut parser = Parser::new(lexer);
    /// ```
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

    /// Returns the whole AST. \
    /// Expample
    /// ```rust
    /// use cara::front::{Lexer, Parser};
    /// let lexer = Lexer::new("1-(5+7)/2+2*3-100".into());
    /// let mut parser = Parser::new(lexer);
    /// let ast = parser.parse_compile_unit();
    /// println!("{:#?}",ast);
    /// /* Output:
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
    /// )*/
    /// ```
    pub fn parse_compile_unit(&mut self) -> Rc<AstNodes> {
        let mut children = Vec::new();
        while let Some(_) = self.current_token {
            children.push(self.parse_statement());
        }
        Rc::new(AstNodes::CompileUnit(children))
    }

    fn parse_statement(&mut self) -> Rc<AstNodes> {
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
                    } else {
                        return self.parse_assign();
                    }
                }
                _ => panic!("Syntax error {:?}!", current_token),
            }
        }
        panic!("Nothing to parse!");
    }

    fn parse_break(&mut self) -> Rc<AstNodes> {
        self.advance();
        Rc::new(AstNodes::Break)
    }

    fn parse_continue(&mut self) -> Rc<AstNodes> {
        self.advance();
        Rc::new(AstNodes::Continue)
    }

    fn parse_while(&mut self) -> Rc<AstNodes> {
        self.advance();
        let condition = self.parse_expr();
        self.eat(Token::LBrace);
        let body = self.parse_block();
        self.eat(Token::RBrace);
        Rc::new(AstNodes::While(condition, body))
    }

    fn parse_list(&mut self) -> Rc<AstNodes> {
        self.eat(Token::LBracket);
        let mut value_list = Vec::new();

        if let Some(Token::RBracket) = self.current_token {
        } else {
            let first_value = self.parse_expr();

            if let Some(Token::Semi) = self.current_token {
                self.advance();
                let num = self.parse_expr();
                self.eat(Token::RBracket);
                return Rc::new(AstNodes::TemplateList(first_value, num));
            } else {
                value_list.push(first_value.clone());
                while let Some(token) = self.current_token.clone() {
                    if let Token::RBracket = token {
                        break;
                    }
                    self.eat(Token::Comma);
                    let value = self.parse_expr();
                    value_list.push(value);
                }
            }
        }

        self.eat(Token::RBracket);
        Rc::new(AstNodes::List(value_list))
    }

    fn parse_for(&mut self) -> Rc<AstNodes> {
        self.advance();

        let variable = self.eat(Token::Id("".into())).as_ident().unwrap();

        self.eat(Token::Keyword(KeywordTypes::In));

        self.eat(Token::LParen);
        let start = self.parse_expr();
        self.eat(Token::Comma);
        let end = self.parse_expr();

        let step = if let Some(Token::Comma) = self.current_token {
            self.advance();
            self.parse_expr()
        } else {
            Rc::new(AstNodes::Number(BigInt::from(1)))
        };

        self.eat(Token::RParen);

        self.eat(Token::LBrace);
        let body = self.parse_block();
        self.eat(Token::RBrace);

        Rc::new(AstNodes::For(variable, start, end, step, body))
    }

    fn parse_if(&mut self) -> Rc<AstNodes> {
        self.advance();
        //self.eat(Token::LParen);
        let condition = self.parse_expr();
        //self.eat(Token::RParen);

        self.eat(Token::LBrace);
        let then_block = self.parse_block();
        self.eat(Token::RBrace);

        let else_block = if let Some(Token::Keyword(KeywordTypes::Else)) = self.current_token {
            self.advance();
            self.eat(Token::LBrace);
            let block = self.parse_block();
            self.eat(Token::RBrace);
            block
        } else {
            Vec::new()
        };

        Rc::new(AstNodes::If(condition, then_block, else_block))
    }

    fn parse_block(&mut self) -> Vec<Rc<AstNodes>> {
        let mut children = Vec::new();
        while let Some(_) = self.current_token {
            if let Some(Token::RBrace) = self.current_token {
                break;
            }
            children.push(self.parse_statement());
        }
        children
    }

    fn parse_return(&mut self) -> Rc<AstNodes> {
        self.advance();
        let expr = self.parse_expr();
        self.eat(Token::Semi);
        Rc::new(AstNodes::Return(expr))
    }

    fn parse_function(&mut self) -> Rc<AstNodes> {
        self.advance();
        let id = self.eat(Token::Id("".into())).as_ident().unwrap();

        self.eat(Token::LParen);
        let params = self.parse_params();
        self.eat(Token::RParen);

        self.eat(Token::LBrace);

        let mut body = Vec::new();
        while let Some(current) = self.current_token.clone() {
            if current != Token::RBrace {
                body.push(self.parse_statement());
            } else {
                break;
            }
        }

        self.eat(Token::RBrace);

        Rc::new(AstNodes::FunctionDef(id, params, body))
    }

    fn parse_params(&mut self) -> Vec<String> {
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

    fn parse_const(&mut self) -> Rc<AstNodes> {
        self.advance();

        let id = self.eat(Token::Id("".into())).as_ident().unwrap();

        self.eat(Token::Assign);

        let init_val = self.parse_expr();

        self.eat(Token::Semi);

        Rc::new(AstNodes::ConstDef(id, init_val))
    }

    fn parse_var(&mut self) -> Rc<AstNodes> {
        self.advance();

        let id = self.eat(Token::Id("".into())).as_ident().unwrap();

        self.eat(Token::Assign);

        let init_val = if let Some(Token::LBracket) = self.current_token {
            self.parse_list()
        } else {
            self.parse_expr()
        };

        self.eat(Token::Semi);

        Rc::new(AstNodes::VarDef(id, init_val))
    }

    fn parse_assign(&mut self) -> Rc<AstNodes> {
        let id = self.eat(Token::Id("".into())).as_ident().unwrap();

        let index = if let Some(Token::LBracket) = self.current_token {
            self.advance();
            let index = self.parse_expr();
            self.eat(Token::RBracket);
            Some(index)
        } else {
            None
        };

        self.eat(Token::Assign);

        let expr = self.parse_expr();

        self.eat(Token::Semi);

        Rc::new(AstNodes::Assign(id, index, expr))
    }

    fn parse_expr(&mut self) -> Rc<AstNodes> {
        let mut node = self.parse_eq_expr();
        while let Some(current_token) = self.current_token.clone() {
            if let Some(op) = current_token.as_operator() {
                match op {
                    "||" | "&&" => {
                        self.advance();
                        node = Rc::new(AstNodes::BinaryOp(node, op, self.parse_eq_expr()));
                    }
                    _ => break,
                }
            } else {
                break;
            }
        }
        node
    }

    fn parse_eq_expr(&mut self) -> Rc<AstNodes> {
        let mut node = self.parse_add_expr();
        while let Some(current_token) = self.current_token.clone() {
            if let Some(op) = current_token.as_operator() {
                match op {
                    "==" | "!=" | ">=" | "<=" | "<" | ">" => {
                        self.advance();
                        node = Rc::new(AstNodes::BinaryOp(node, op, self.parse_add_expr()));
                    }
                    _ => break,
                }
            } else {
                break;
            }
        }
        node
    }

    fn parse_add_expr(&mut self) -> Rc<AstNodes> {
        let mut node = self.parse_move_expr();
        while let Some(current_token) = self.current_token.clone() {
            if let Some(op) = current_token.as_operator() {
                match op {
                    "+" | "-" => {
                        self.advance();
                        node = Rc::new(AstNodes::BinaryOp(node, op, self.parse_move_expr()));
                    }
                    _ => break,
                }
            } else {
                break;
            }
        }
        node
    }

    fn parse_move_expr(&mut self) -> Rc<AstNodes> {
        let mut node = self.parse_term();
        while let Some(current_token) = self.current_token.clone() {
            if let Some(op) = current_token.as_operator() {
                match op {
                    "<<" | ">>" => {
                        self.advance();
                        node = Rc::new(AstNodes::BinaryOp(node, op, self.parse_term()));
                    }
                    _ => break,
                }
            } else {
                break;
            }
        }
        node
    }

    fn parse_term(&mut self) -> Rc<AstNodes> {
        let mut node = self.parse_factor();
        while let Some(current_token) = self.current_token.clone() {
            if let Some(op) = current_token.as_operator() {
                match op {
                    "*" | "/" | "%" => {
                        self.advance();
                        node = Rc::new(AstNodes::BinaryOp(node, op, self.parse_factor()));
                    }
                    _ => break,
                }
            } else {
                break;
            }
        }
        node
    }

    fn parse_factor(&mut self) -> Rc<AstNodes> {
        let token = self.current_token.clone().unwrap();
        match token {
            Token::Number(num) => {
                self.advance();
                Rc::new(AstNodes::Number(num))
            }
            Token::LParen => {
                self.advance();
                let node = self.parse_expr();
                self.eat(Token::RParen);
                node
            }
            Token::Operator(op) => match op {
                "+" | "-" => {
                    self.advance();
                    let node = self.parse_expr();
                    Rc::new(AstNodes::UnaryOp(op, node))
                }
                _ => panic!("Unexpected unary operator {}!", op),
            },
            Token::Id(id) => {
                if self.lexer.current_char() == '(' {
                    let node = self.parse_call(false);
                    //println!("{}",self.lexer.current_char());
                    node
                } else if self.lexer.current_char() == '[' {
                    self.advance();
                    self.advance();
                    let index_value = self.parse_expr();
                    self.eat(Token::RBracket);
                    Rc::new(AstNodes::Index(id, index_value))
                } else {
                    self.advance();
                    Rc::new(AstNodes::ReadVar(id))
                }
            }
            _ => panic!("Syntax error {:?}!", token),
        }
    }

    fn parse_call(&mut self, stmt: bool) -> Rc<AstNodes> {
        let id = self.eat(Token::Id("".into())).as_ident().unwrap();

        self.eat(Token::LParen);

        let args = self.parse_args();

        self.eat(Token::RParen);

        if stmt {
            self.eat(Token::Semi);
        }

        Rc::new(AstNodes::Call(id, args))
    }

    fn parse_args(&mut self) -> Vec<Rc<AstNodes>> {
        let mut args = Vec::new();
        while let Some(current_token) = self.current_token.clone() {
            if current_token != Token::RParen {
                args.push(self.parse_expr());
                if let Some(Token::Comma) = self.current_token {
                    self.advance();
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        args
    }
}
