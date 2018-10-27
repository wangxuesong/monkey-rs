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
        Token::Assign
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
        ];

        let mut l = Lexer::new(input);

        for t in tests.iter() {
            let tok = l.next_token();

            assert_eq!(*t, tok);
        }
    }
}