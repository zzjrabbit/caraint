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
    pub fn parse_compile_unit(&mut self) -> Box<AstNodes> {
        let mut children = Vec::new();
        while let Some(_) = self.current_token {
            children.push(self.parse_statement());
        }
        Box::new(AstNodes::CompileUnit(children))
    }

    fn parse_statement(&mut self) -> Box<AstNodes> {
        if let Some(current_token) = self.current_token.clone() {
            match current_token {
                Token::Keyword(key_word) => {
                    return match key_word {
                        KeywordTypes::Var => self.parse_var(),
                        KeywordTypes::Const => self.parse_const(),
                    }
                }
                Token::Id(_) => return self.parse_assign(),
                _ => panic!("Syntax error {:?}!", current_token),
            }
        }
        panic!("Nothing to parse!");
    }

    fn parse_const(&mut self) -> Box<AstNodes> {
        self.advance();
        let id = self.current_token.clone().unwrap();
        let id = match id {
            Token::Id(id) => id.clone(),
            _ => panic!("Syntax error! Expected ID!"),
        };
        self.advance();
        let assign = self.current_token.clone().unwrap();
        match assign {
            Token::Assign => {}
            _ => panic!("Syntax error! Expected '=' !"),
        }
        self.advance();
        let init_val = self.parse_expr();
        let semmi = self.current_token.clone().unwrap();
        match semmi {
            Token::Semi => {}
            _ => panic!("Syntax error! Expected ';' ,found {:?}!", semmi),
        }
        self.advance();
        Box::new(AstNodes::ConstDef(id, init_val))
    }

    fn parse_var(&mut self) -> Box<AstNodes> {
        self.advance();
        let id = self.current_token.clone().unwrap();
        let id = match id {
            Token::Id(id) => id.clone(),
            _ => panic!("Syntax error! Expected ID!"),
        };
        self.advance();
        let assign = self.current_token.clone().unwrap();
        match assign {
            Token::Assign => {}
            _ => panic!("Syntax error! Expected '=' !"),
        }
        self.advance();
        let init_val = self.parse_expr();
        let semmi = self.current_token.clone().unwrap();
        match semmi {
            Token::Semi => {}
            _ => panic!("Syntax error! Expected ';' ,found {:?}!", semmi),
        }
        self.advance();
        Box::new(AstNodes::VarDef(id, init_val))
    }

    fn parse_assign(&mut self) -> Box<AstNodes> {
        let id = self.current_token.clone().unwrap();
        let id = match id {
            Token::Id(id) => id.clone(),
            _ => panic!("Syntax error! Expected ID!"),
        };
        self.advance();
        let assign = self.current_token.clone().unwrap();
        match assign {
            Token::Assign => {}
            _ => panic!("Syntax error! Expected '='!"),
        }
        self.advance();
        let expr = self.parse_expr();
        let semmi = self.current_token.clone().unwrap();
        match semmi {
            Token::Semi => {}
            _ => panic!("Syntax error! Expected ';' ,found {:?}!", semmi),
        }
        self.advance();
        Box::new(AstNodes::Assign(id, expr))
    }

    fn parse_expr(&mut self) -> Box<AstNodes> {
        let mut node = self.parse_term();
        while let Some(current_token) = self.current_token.clone() {
            if let Some(op) = current_token.as_operator() {
                match op {
                    '+' | '-' => {
                        self.advance();
                        node = Box::new(AstNodes::BinaryOp(node, op, self.parse_term()));
                    }
                    _ => break,
                }
            } else {
                break;
            }
        }
        node
    }

    fn parse_term(&mut self) -> Box<AstNodes> {
        let mut node = self.parse_factor();
        while let Some(current_token) = self.current_token.clone() {
            if let Some(op) = current_token.as_operator() {
                match op {
                    '*' | '/' => {
                        self.advance();
                        node = Box::new(AstNodes::BinaryOp(node, op, self.parse_factor()));
                    }
                    _ => break,
                }
            } else {
                break;
            }
        }
        node
    }

    fn parse_factor(&mut self) -> Box<AstNodes> {
        match self.current_token.clone().unwrap() {
            Token::Number(num) => {
                self.advance();
                Box::new(AstNodes::Number(num))
            }
            Token::LParen => {
                self.advance();
                let node = self.parse_expr();
                match self.current_token.clone().unwrap() {
                    Token::RParen => {}
                    _ => panic!("Expected ')'!"),
                }
                self.advance();
                node
            }
            Token::Operator(op) => match op {
                '+' | '-' => {
                    self.advance();
                    let node = self.parse_expr();
                    Box::new(AstNodes::UnaryOp(op, node))
                }
                _ => panic!("Unexpected unary operator {}!", op),
            },
            Token::Id(id) => {
                self.advance();
                Box::new(AstNodes::ReadVar(id))
            }
            _ => panic!("Syntax error!"),
        }
    }
}
