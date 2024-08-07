/// This enum defines all the token types with their values

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum KeywordTypes {
    Var,
    Const,
}

impl KeywordTypes {
    pub fn from_string(string: String) -> Option<Self> {
        match string.as_str() {
            "var" => Some(Self::Var),
            "const" => Some(Self::Const),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    /// Numbers, such as 0,1,2,1234,114514 and so on.
    Number(i64),
    /// Operators, +,-,*,/,......
    Operator(char),
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
}

impl Token {
    /// This function returns the operator if the token is, otherwise it returns None.
    pub fn as_operator(&self) -> Option<char> {
        match self {
            Token::Operator(ch) => Some(*ch),
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
                    let mut num = ch.to_digit(10).unwrap() as i64;
                    while let Some(ch) = self.advance() {
                        if !ch.is_numeric() {
                            self.position -= 1;
                            break;
                        }
                        num *= 10;
                        num += ch.to_digit(10).unwrap() as i64;
                    }
                    return Some(Token::Number(num));
                }
                '+' | '-' | '*' | '/' => {
                    return Some(Token::Operator(ch));
                }
                '(' => return Some(Token::LParen),
                ')' => return Some(Token::RParen),
                '=' => return Some(Token::Assign),
                ';' => return Some(Token::Semi),
                ' ' | '\n' | '\r' => continue,
                _ => {
                    if ch.is_alphabetic() || ch == '_' {
                        let mut id = String::new();
                        id.push(ch);
                        while let Some(ch) = self.advance() {
                            if !ch.is_alphabetic() {
                                self.position -= 1;
                                break;
                            }
                            id.push(ch);
                        }
                        if let Some(keyword_type) = KeywordTypes::from_string(id.clone()) {
                            return Some(Token::Keyword(keyword_type));
                        } else {
                            return Some(Token::Id(id));
                        }
                    }
                    panic!("Unexpected charactor {}!", ch)
                }
            }
        }
        None
    }
}
