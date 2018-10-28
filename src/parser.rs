use ast::*;
use lexer::Lexer;
use token::Token;

type ParseError = String;
pub type ParseResult<T> = Result<T, ParseError>;

pub struct Parser<'a> {
    l: Lexer<'a>,

    cur_token: Token,
    peek_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(l: Lexer) -> Parser {
        let mut l = l;
        let cur_token = l.next_token();
        let peek_token = l.next_token();
        Parser {
            l,
            cur_token,
            peek_token,
        }
    }

    pub fn parse_program(&mut self) -> ParseResult<Program> {
        let mut p = Program::new();

        while self.cur_token != Token::EOF {
            let stmt = self.parse_statement()?;
            p.statements.push(stmt);
            self.next_token();
        }
        Ok(p)
    }

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.l.next_token();
    }

    fn parse_statement(&mut self) -> ParseResult<Statement> {
        match self.cur_token {
            Token::Let => self.parse_let_statement(),
            _ => unimplemented!("{:?}", self.cur_token)
        }
    }

    fn parse_let_statement(&mut self) -> ParseResult<Statement> {
        let name = self.expect_ident()?;

        self.expect_token(Token::Assign);

        let value = self.parse_integer_literal()?;

        self.expect_token(Token::Semicolon);

        Ok(Statement::Let(Box::new(
            LetStatement {
                name,
                value,
            }))
        )
    }

    fn parse_integer_literal(&mut self) -> ParseResult<Expression> {
        self.next_token();
        if let Token::Int(value) = self.cur_token {
            return Ok(Expression::Integer(value));
        };
        Err(format!("invalid integer {}", self.cur_token))
    }

    fn expect_token(&mut self, tok: Token) {
        if tok == self.peek_token {
            self.next_token();
        };
    }

    fn expect_ident(&mut self) -> ParseResult<String> {
        self.next_token();
        if let Token::Ident(name) = self.cur_token.clone() {
            return Ok(name);
        }
        Err(format!("invalid identifier {}", self.cur_token))
    }
}

#[cfg(test)]
mod tests {
    use ast::*;
    use lexer::Lexer;
    use super::*;

    #[test]
    fn parse_let_statement() {
        let input = r#"let birthday = 1103;"#;
        let l = Lexer::new(input);
        let mut p = Parser::new(l);

        let expects = vec![
            ("birthday", Expression::Integer(1103)),
        ];

        let program = p.parse_program().unwrap();
        let mut iter = program.statements.iter();

        for e in expects {
            match iter.next().unwrap() {
                Statement::Let(ref l) => {
                    assert_eq!(e.0, l.name);
                    assert_eq!(e.1, l.value);
                }
            }
        }
    }

    #[test]
    fn parse_let_statement_error() {
        let input = r#"let birthday = ;"#;
        let l = Lexer::new(input);
        let mut p = Parser::new(l);

        let expects = vec![
            ("birthday", Expression::Integer(1103)),
        ];

        match p.parse_program() {
            Ok(_) => panic!("error"),
            Err(err) => {
                assert_eq!("invalid integer Semicolon", err)
            }
        }
    }
}