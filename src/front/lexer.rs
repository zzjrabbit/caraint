use num_bigint::BigInt;

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
    pub fn from_string(string: String) -> Option<Self> {
        match string.as_str() {
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
    Number(BigInt),
    /// Operators, +,-,*,/,......
    Operator(&'static str),
    /// Left paren, (
    LParen,
    /// Right paren, )
    RParen,
    /// Identifiers
    Id(String),
    /// Keywords
    Keyword(KeywordTypes),
    /// Assign, =
    Assign,
    /// Semi
    Semi,
    /// LBrace, {
    LBrace,
    /// RBrace, }
    RBrace,
    /// Comma, ,
    Comma,
    /// LBracket, {
    LBracket,
    /// RBracket, }
    RBracket,
}

impl Token {
    /// This function returns the operator if the token is, otherwise it returns None.
    pub fn as_operator(&self) -> Option<&'static str> {
        match self {
            Token::Operator(ch) => Some(*ch),
            _ => None,
        }
    }

    pub fn as_ident(&self) -> Option<String> {
        match self {
            Token::Id(id) => Some(id.clone()),
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
    /// use cara::front::Lexer;
    /// let lexer = Lexer::new("1+2*3".into());
    /// ```
    pub fn new(input: String) -> Self {
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

    pub fn current_char(&self) -> char {
        self.input.chars().nth(self.position).unwrap_or('\0')
    }

    /// Let the lexer parse a token and return it. \
    /// Example
    /// ```rust
    /// use cara::front::Lexer;
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
                    let mut num = BigInt::from(ch.to_digit(10).unwrap());
                    while let Some(ch) = self.advance() {
                        if !ch.is_numeric() {
                            self.position -= 1;
                            break;
                        }
                        num = num * 10 + ch.to_digit(10).unwrap();
                    }
                    return Some(Token::Number(num));
                }
                '+' | '-' | '*' | '/' => {
                    return Some(Token::Operator(<char as Into<String>>::into(ch).leak()));
                }
                '(' => return Some(Token::LParen),
                ')' => return Some(Token::RParen),
                '=' => {
                    if self.current_char() == '=' {
                        self.advance();
                        return Some(Token::Operator("=="));
                    }
                    return Some(Token::Assign);
                }
                '!' => {
                    if self.current_char() == '=' {
                        self.advance();
                        return Some(Token::Operator("!="));
                    }
                    panic!("Unexpected charactor {}!", ch)
                }
                '>' => {
                    if self.current_char() == '=' {
                        self.advance();
                        return Some(Token::Operator(">="));
                    } else if self.current_char() == '>' {
                        self.advance();
                        return Some(Token::Operator(">>"));
                    }
                    return Some(Token::Operator(">"));
                }
                '<' => {
                    if self.current_char() == '=' {
                        self.advance();
                        return Some(Token::Operator("<="));
                    } else if self.current_char() == '<' {
                        self.advance();
                        return Some(Token::Operator("<<"));
                    }
                    return Some(Token::Operator("<"));
                }
                '|' => {
                    if self.current_char() == '|' {
                        self.advance();
                        return Some(Token::Operator("||"));
                    }
                    panic!("Unexpected charactor {}!", ch)
                }
                '&' => {
                    if self.current_char() == '&' {
                        self.advance();
                        return Some(Token::Operator("&&"));
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
                        if let Some(keyword_type) = KeywordTypes::from_string(id.clone()) {
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
