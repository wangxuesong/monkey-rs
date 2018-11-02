use std::fmt;

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Token {
    Illegal,
    EOF,

    // 标识符
    Ident(String),
    Int(i64),

    // 操作符
    Assign,
    Plus,
    Minus,
    Asterisk,
    Slash,

    // 分隔符
    Comma,
    Semicolon,
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,

    // 关键字
    Function,
    Let,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            Token::Int(value) => write!(f, "{}", value),
            tok => write!(f, "{:?}", tok),
        }
    }
}

pub fn lookup_ident(ident: String) -> Token {
    match ident.as_str() {
        "let" => Token::Let,
        _ => Token::Ident(ident),
    }
}
