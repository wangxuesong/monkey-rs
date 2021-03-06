use ast::*;
use lexer::Lexer;
use token::Token;

type ParseError = String;
pub type ParseResult<T> = Result<T, ParseError>;
type PrefixFunc = fn(parser: &mut Parser) -> ParseResult<Expression>;
type InfixFunc = fn(parser: &mut Parser, left: Expression) -> ParseResult<Expression>;

#[derive(Eq, PartialEq, Debug, Ord, PartialOrd)]
enum Precedence {
    Lowest,
    // ==
    Equals,
    // > <
    LessGreater,
    // + -
    Sum,
    // * /
    Product,
    // -
    Prefix,
    // function
    Call,
}

impl Precedence {
    fn token_precedence(tok: &Token) -> Precedence {
        match tok {
            Token::Plus | Token::Minus => Precedence::Sum,
            Token::Asterisk | Token::Slash => Precedence::Product,
            _ => Precedence::Lowest,
        }
    }
}

pub struct Parser<'a> {
    l: Lexer<'a>,

    cur_token: Token,
    peek_token: Token,
}

pub fn parse(input: &str) -> Result<Node, ParseError> {
    let lexer = Lexer::new(input);
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
            Token::Minus => Some(Parser::parse_prefix_expression),
            Token::Lparen => Some(Parser::parse_group_expression),
            _ => None,
        }
    }

    fn infix_fn(&mut self) -> Option<InfixFunc> {
        match self.cur_token {
            Token::Minus | Token::Plus | Token::Asterisk | Token::Slash => {
                Some(Parser::parse_infix_expression)
            }
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
        self.next_token(); // skip let
        let name = self.expect_ident()?;

        self.expect_token(Token::Assign)?;

        let value = self.parse_expression(&Precedence::Lowest)?;

        self.expect_token(Token::Semicolon)?;

        Ok(Statement::Let(Box::new(LetStatement { name, value })))
    }

    fn parse_expression_statement(&mut self) -> ParseResult<Statement> {
        let expr = self.parse_expression(&Precedence::Lowest)?;

        self.expect_token(Token::Semicolon)?;

        Ok(Statement::Expression(Box::new(ExpressionStatement {
            expression: expr,
        })))
    }

    fn parse_expression(&mut self, precedence: &Precedence) -> ParseResult<Expression> {
        let mut left;

        if let Some(f) = self.prefix_fn() {
            left = f(self)?
        } else {
            return Err(format!("invalid token {:?}", self.cur_token));
        }

        while self.cur_token != Token::Semicolon
            && *precedence < Precedence::token_precedence(&self.cur_token)
            {
            match self.infix_fn() {
                Some(f) => {
                    left = f(self, left)?;
                }
                None => return Ok(left),
            }
        }
        Ok(left)
    }

    fn parse_prefix_expression(parser: &mut Parser) -> ParseResult<Expression> {
        let operator = parser.cur_token.clone();
        parser.next_token();

        let right = parser.parse_expression(&Precedence::token_precedence(&operator))?;
        Ok(Expression::Prefix(Box::new(PrefixExpression {
            operator,
            right,
        })))
    }

    fn parse_group_expression(parser: &mut Parser) -> ParseResult<Expression> {
        parser.next_token(); // Skip Lparen
        let right = parser.parse_expression(&Precedence::Lowest)?;
        parser.expect_token(Token::Rparen)?;
        Ok(right)
    }

    fn parse_infix_expression(parser: &mut Parser, left: Expression) -> ParseResult<Expression> {
        let operator = parser.cur_token.clone();
        parser.next_token();

        let right = parser.parse_expression(&Precedence::token_precedence(&operator))?;
        Ok(Expression::Infix(Box::new(InfixExpression {
            operator,
            left,
            right,
        })))
    }

    fn parse_integer_literal(parser: &mut Parser) -> ParseResult<Expression> {
        if let Token::Int(value) = parser.cur_token {
            parser.next_token();
            return Ok(Expression::Integer(value));
        };
        Err(format!("invalid token {}", parser.cur_token))
    }

    fn expect_token(&mut self, tok: Token) -> ParseResult<()> {
        if tok == self.cur_token {
            self.next_token();
            return Ok(());
        };
        Err(format!("expect token {} but {}", tok, self.cur_token))
    }

    fn expect_peek(&mut self, tok: Token) -> ParseResult<()> {
        if tok == self.peek_token {
            self.next_token();
            return Ok(());
        };
        Err(format!("expect token {} but {}", tok, self.peek_token))
    }

    fn expect_ident(&mut self) -> ParseResult<String> {
        if let Token::Ident(name) = self.cur_token.clone() {
            self.next_token();
            return Ok(name);
        }
        Err(format!("invalid identifier {}", self.cur_token))
    }
}

#[cfg(test)]
mod tests {
    use ast::*;
    use lexer::Lexer;
    use parser::*;
    use token;

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
            (
                "-1103;",
                Expression::Prefix(Box::new(PrefixExpression {
                    operator: token::Token::Minus,
                    right: Expression::Integer(1103),
                })),
            ),
            (
                "2206-1103;",
                Expression::Infix(Box::new(InfixExpression {
                    operator: token::Token::Minus,
                    left: Expression::Integer(2206),
                    right: Expression::Integer(1103),
                })),
            ),
            (
                "1103-1103+1103;",
                Expression::Infix(Box::new(InfixExpression {
                    operator: token::Token::Plus,
                    right: Expression::Integer(1103),
                    left: Expression::Infix(Box::new(InfixExpression {
                        left: Expression::Integer(1103),
                        operator: token::Token::Minus,
                        right: Expression::Integer(1103),
                    })),
                })),
            ),
            (
                "1103*2;",
                Expression::Infix(Box::new(InfixExpression {
                    operator: token::Token::Asterisk,
                    left: Expression::Integer(1103),
                    right: Expression::Integer(2),
                })),
            ),
            (
                "-1103-1103*1103;",
                Expression::Infix(Box::new(InfixExpression {
                    operator: token::Token::Minus,
                    left: Expression::Prefix(Box::new(PrefixExpression {
                        operator: token::Token::Minus,
                        right: Expression::Integer(1103),
                    })),
                    right: Expression::Infix(Box::new(InfixExpression {
                        left: Expression::Integer(1103),
                        operator: token::Token::Asterisk,
                        right: Expression::Integer(1103),
                    })),
                })),
            ),
            (
                "1103-(1103+1103);",
                Expression::Infix(Box::new(InfixExpression {
                    operator: token::Token::Minus,
                    left: Expression::Integer(1103),
                    right: Expression::Infix(Box::new(InfixExpression {
                        left: Expression::Integer(1103),
                        operator: token::Token::Plus,
                        right: Expression::Integer(1103),
                    })),
                })),
            ),
        ];

        for e in expects {
            let mut p = setup(e.0);
            let program = p.parse_program().expect(e.0);
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
