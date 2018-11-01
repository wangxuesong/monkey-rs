use ast::*;
use lexer::Lexer;
use token::Token;

type ParseError = String;
pub type ParseResult<T> = Result<T, ParseError>;
type PrefixFunc = fn(parser: &mut Parser) -> ParseResult<Expression>;
type InfixFunc = fn(parser: &mut Parser, left: Expression) -> ParseResult<Expression>;

pub struct Parser<'a> {
    l: Lexer<'a>,

    cur_token: Token,
    peek_token: Token,
}

pub fn parse(input: &str) -> Result<Node, ParseError> {
    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let prog = parser.parse_program()?;

    Ok(Node::Program(Box::new(prog)))
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

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.l.next_token();
    }

    fn prefix_fn(&mut self) -> Option<PrefixFunc> {
        match self.cur_token {
            Token::Int(_) => Some(Parser::parse_integer_literal),
            _ => None,
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

    fn parse_statement(&mut self) -> ParseResult<Statement> {
        match self.cur_token {
            Token::Let => self.parse_let_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_let_statement(&mut self) -> ParseResult<Statement> {
        let name = self.expect_ident()?;

        self.expect_token(Token::Assign);

        self.next_token();
        let value = self.parse_expression()?;

        self.expect_token(Token::Semicolon);

        Ok(Statement::Let(Box::new(LetStatement { name, value })))
    }

    fn parse_expression_statement(&mut self) -> ParseResult<Statement> {
        let expr = self.parse_expression()?;

        self.expect_token(Token::Semicolon);

        Ok(Statement::Expression(Box::new(ExpressionStatement {
            expression: expr,
        })))
    }

    fn parse_expression(&mut self) -> ParseResult<Expression> {
        let mut left;

        if let Some(f) = self.prefix_fn() {
            left = f(self)?
        } else {
            return Err(format!("invalid token {:?}", self.cur_token));
        }

        while self.cur_token != Token::Semicolon {
            self.next_token();
        }
        Ok(left)
    }

    fn parse_integer_literal(parser: &mut Parser) -> ParseResult<Expression> {
        //        self.next_token();
        if let Token::Int(value) = parser.cur_token {
            return Ok(Expression::Integer(value));
        };
        Err(format!("invalid token {}", parser.cur_token))
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

        let expects = vec![("birthday", Expression::Integer(1103))];

        let program = p.parse_program().unwrap();
        let mut iter = program.statements.iter();

        for e in expects {
            match iter.next().unwrap() {
                Statement::Let(ref l) => {
                    assert_eq!(e.0, l.name);
                    assert_eq!(e.1, l.value);
                }
                stmt => panic!("expected let statement but got {:?}", stmt),
            }
        }
    }

    #[test]
    fn parse_let_statement_error() {
        let input = r#"let birthday = ;"#;
        let l = Lexer::new(input);
        let mut p = Parser::new(l);

        let expects = vec![("birthday", Expression::Integer(1103))];

        match p.parse_program() {
            Ok(_) => panic!("error"),
            Err(err) => assert_eq!("invalid token Semicolon", err),
        }
    }

    fn setup(input: &str) -> Parser {
        let l = Lexer::new(input);
        Parser::new(l)
    }

    #[test]
    fn parse_expression_statement() {
        let expects = vec![
            ("1103;", Expression::Integer(1103)),
            //            ("-1103", Expression::Prefix()),
        ];

        for e in expects {
            let mut p = setup(e.0);
            let program = p.parse_program().unwrap();
            let mut iter = program.statements.iter();

            match iter.next().unwrap() {
                Statement::Expression(ref l) => {
                    assert_eq!(e.1, l.expression);
                }
                _ => panic!("expected let statement"),
            }
        }
    }
}
