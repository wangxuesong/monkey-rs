#[derive(Eq, PartialEq, Debug)]
pub enum Token {
    Illegal,
    EOF,

    // 标识符
    Ident(String),
    Int(i64),

    // 操作符
    Assign,
    Plus,

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

pub fn lookup_ident(ident: String) -> Token {
    match ident.as_str() {
        "let" => Token::Let,
        _ => Token::Ident(ident),
    }
}