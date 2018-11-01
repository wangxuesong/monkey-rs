use std::iter::Peekable;
use std::str::Chars;
use token;
use token::Token;

#[derive(Debug)]
pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Lexer {
        Lexer {
            input: input.chars().peekable(),
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        match self.read_char() {
            Some('=') => Token::Assign,
            Some('+') => Token::Plus,
            Some('-') => Token::Minus,
            Some('(') => Token::Lparen,
            Some(')') => Token::Rparen,
            Some('{') => Token::Lbrace,
            Some('}') => Token::Rbrace,
            Some(',') => Token::Comma,
            Some(';') => Token::Semicolon,
            Some(ch) => {
                if is_letter(ch) {
                    let ident = self.read_identifier(ch);
                    let tok = token::lookup_ident(ident);
                    tok
                } else if ch.is_digit(10) {
                    Token::Int(self.read_number(ch))
                } else { Token::Illegal }
            }
            None => Token::EOF,
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(&ch) = self.input.peek() {
            if ch.is_whitespace() {
                self.read_char();
            } else {
                break;
            }
        }
    }

    fn read_char(&mut self) -> Option<char> {
        self.input.next()
    }

    fn read_identifier(&mut self, ch: char) -> String {
        let mut ident = String::new();
        ident.push(ch);

        while let Some(&ch) = self.input.peek() {
            if is_letter(ch) {
                ident.push(self.read_char().unwrap())
            } else {
                break;
            }
        };

        ident
    }

    fn read_number(&mut self, ch: char) -> i64 {
        let mut number = String::new();
        number.push(ch);

        while let Some(&ch) = self.input.peek() {
            if ch.is_digit(10) {
                number.push(self.read_char().unwrap())
            } else {
                break;
            }
        }
        number.parse().unwrap()
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        let tok = self.next_token();
        if tok == Token::EOF {
            None
        } else {
            Some(tok)
        }
    }
}

fn is_letter(ch: char) -> bool {
    ch.is_alphabetic() || ch == '_'
}

#[cfg(test)]
mod tests {
    use super::*;
    use token::Token;

    #[test]
    fn test_next_token() {
        let input = r#"let five = 5;"#;

        let tests = vec![
            Token::Let,
            Token::Ident("five".to_string()),
            Token::Assign,
            Token::Int(5),
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