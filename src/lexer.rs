use std::iter::Peekable;
use std::str::Chars;
use token;
use token::Token;

#[derive(Debug)]
struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Lexer {
        Lexer {
            input: input.chars().peekable(),
        }
    }

    pub fn next_token(&mut self) -> Token {
        match self.read_char() {
            Some('=') => Token::Assign,
            Some('+') => Token::Plus,
            Some('(') => Token::Lparen,
            Some(')') => Token::Rparen,
            Some('{') => Token::Lbrace,
            Some('}') => Token::Rbrace,
            Some(',') => Token::Comma,
            Some(';') => Token::Semicolon,
            Some(_) => Token::EOF,
            None => Token::EOF,
        }
    }

    fn read_char(&mut self) -> Option<char> {
        self.input.next()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use token::Token;

    #[test]
    fn test_next_token() {
        let input = r#"=+(){},;"#;

        let tests = vec![
            Token::Assign,
            Token::Plus,
            Token::Lparen,
            Token::Rparen,
            Token::Lbrace,
            Token::Rbrace,
            Token::Comma,
            Token::Semicolon,
            Token::EOF,
        ];

        let mut l = Lexer::new(input);

        for t in tests.iter() {
            let tok = l.next_token();

            assert_eq!(*t, tok);
        }
    }
}