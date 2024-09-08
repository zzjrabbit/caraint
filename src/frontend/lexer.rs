use alloc::string::String;
use dashu_int::IBig;

/// This enum defines all the token types with their values

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum KeywordTypes {
    Var,
    Const,
    Fn,
    Return,
    If,
    Else,
    For,
    In,
    While,
    Break,
    Continue,
}

impl KeywordTypes {
    #[must_use]
    pub fn from_string(string: &str) -> Option<Self> {
        match string {
            "var" => Some(Self::Var),
            "const" => Some(Self::Const),
            "fn" => Some(Self::Fn),
            "return" => Some(Self::Return),
            "if" => Some(Self::If),
            "else" => Some(Self::Else),
            "for" => Some(Self::For),
            "in" => Some(Self::In),
            "while" => Some(Self::While),
            "break" => Some(Self::Break),
            "continue" => Some(Self::Continue),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    /// Numbers, such as 0,1,2,1234,114514 and so on.
    Number(IBig),
    /// Operators, +,-,*,/,......
    Operator(String),
    /// `Left paren`, (
    LParen,
    /// `Right paren`, )
    RParen,
    /// `Identifiers`
    Id(String),
    /// `Keywords`
    Keyword(KeywordTypes),
    /// `Assign`, =
    Assign,
    /// `Semi`
    Semi,
    /// `LBrace`, {
    LBrace,
    /// `RBrace`, }
    RBrace,
    /// `Comma`, ,
    Comma,
    /// `LBracket`, {
    LBracket,
    /// `RBracket`, }
    RBracket,
}

impl Token {
    /// This function returns the operator if the token is, otherwise it returns None.
    #[must_use]
    pub fn as_operator(&self) -> Option<String> {
        match self {
            Self::Operator(ch) => Some(ch.clone()),
            _ => None,
        }
    }

    #[must_use]
    pub fn as_ident(&self) -> Option<String> {
        match self {
            Self::Id(id) => Some(id.clone()),
            _ => None,
        }
    }
}

/// A simple and stupid Lexer
pub struct Lexer {
    input: String,
    position: usize,
}

impl Lexer {
    /// Creates a new Lexer with the input. \
    /// Example
    /// ``` rust
    /// use cara::frontend::Lexer;
    /// let lexer = Lexer::new("1+2*3".into());
    /// ```
    #[must_use]
    pub const fn new(input: String) -> Self {
        Self { input, position: 0 }
    }

    fn advance(&mut self) -> Option<char> {
        if self.position >= self.input.len() {
            return None;
        }
        let c = self.input.chars().nth(self.position);
        self.position += 1;
        c
    }

    #[must_use]
    pub fn current_char(&self) -> char {
        self.input.chars().nth(self.position).unwrap_or('\0')
    }

    /// Let the lexer parse a token and return it. \
    /// Example
    /// ```rust
    /// use cara::frontend::Lexer;
    /// let mut lexer = Lexer::new("1+2*3".into());
    /// while let Some(token) = lexer.get_token() {
    ///     print!("{:?} ", token);
    /// }
    /// println!();
    /// // Output: Number(1) Operator('+') Number(2) Operator('*') Number(3)
    /// ```
    pub fn get_token(&mut self) -> Option<Token> {
        while let Some(ch) = self.advance() {
            match ch {
                '0'..='9' => {
                    let mut num = String::new();
                    num.push(ch);
                    while let Some(ch) = self.advance() {
                        if !ch.is_numeric() {
                            self.position -= 1;
                            break;
                        }
                        num.push(ch);
                    }
                    let number = IBig::from_str_radix(&num, 10).unwrap();
                    return Some(Token::Number(number));
                }
                '+' | '-' | '*' | '/' => {
                    return Some(Token::Operator(<char as Into<String>>::into(ch)))
                }
                '(' => return Some(Token::LParen),
                ')' => return Some(Token::RParen),
                '=' => {
                    if self.current_char() == '=' {
                        self.advance();
                        return Some(Token::Operator("==".into()));
                    }
                    return Some(Token::Assign);
                }
                '!' => {
                    if self.current_char() == '=' {
                        self.advance();
                        return Some(Token::Operator("!=".into()));
                    }
                    panic!("Unexpected charactor {}!", ch)
                }
                '>' => {
                    if self.current_char() == '=' {
                        self.advance();
                        return Some(Token::Operator(">=".into()));
                    } else if self.current_char() == '>' {
                        self.advance();
                        return Some(Token::Operator(">>".into()));
                    }
                    return Some(Token::Operator(">".into()));
                }
                '<' => {
                    if self.current_char() == '=' {
                        self.advance();
                        return Some(Token::Operator("<=".into()));
                    } else if self.current_char() == '<' {
                        self.advance();
                        return Some(Token::Operator("<<".into()));
                    }
                    return Some(Token::Operator("<".into()));
                }
                '|' => {
                    if self.current_char() == '|' {
                        self.advance();
                        return Some(Token::Operator("||".into()));
                    }
                    panic!("Unexpected charactor {}!", ch)
                }
                '&' => {
                    if self.current_char() == '&' {
                        self.advance();
                        return Some(Token::Operator("&&".into()));
                    }
                    panic!("Unexpected charactor {}!", ch)
                }
                ';' => return Some(Token::Semi),
                '{' => return Some(Token::LBrace),
                '}' => return Some(Token::RBrace),
                '[' => return Some(Token::LBracket),
                ']' => return Some(Token::RBracket),
                ',' => return Some(Token::Comma),
                ' ' | '\n' | '\r' => continue,
                _ => {
                    if ch.is_alphabetic() || ch == '_' {
                        let mut id = String::new();
                        id.push(ch);
                        while let Some(ch) = self.advance() {
                            if !ch.is_alphabetic() && ch != '_' {
                                self.position -= 1;
                                break;
                            }
                            id.push(ch);
                        }
                        if let Some(keyword_type) = KeywordTypes::from_string(&id) {
                            return Some(Token::Keyword(keyword_type));
                        }
                        return Some(Token::Id(id));
                    }
                    panic!("Unexpected charactor {}!", ch)
                }
            }
        }
        None
    }
}
