use ast::{Expression, InfixExpression, PrefixExpression};
use token::Token;

use nom::digit;
use nom::types::CompleteStr;

use std::str::FromStr;

// We parse any expr surrounded by parens, ignoring all whitespaces around those
named!(parens<CompleteStr, Expression>, ws!(delimited!( tag!("("), expr, tag!(")") )) );

// We transform an integer string into a i64, ignoring surrounding whitespaces
// We look for a digit suite, and try to convert it.
// If either str::from_utf8 or FromStr::from_str fail,
// we fallback to the parens parser defined above
named!(sfactor<CompleteStr, Expression>, alt!(
    map!(
        map_res!(
          ws!(digit),
          |s:CompleteStr| {FromStr::from_str(s.0)}
        ),
        Expression::Integer
    )
  | parens
  )
);

named!(factor<CompleteStr, Expression>, do_parse!(
  sign: opt!(tag_s!("-")) >>
  factor: sfactor >>
  (sign_exprs(sign, factor))
));

named!(term< CompleteStr, Expression >, do_parse!(
    initial: factor >>
    remainder: many0!(
           alt!(
             do_parse!(tag!("*") >> mul: factor >> (Token::Asterisk, mul)) |
             do_parse!(tag!("/") >> div: factor >> (Token::Slash, div))
           )
         ) >>
    (fold_exprs(initial, remainder))
));

named!(expr< CompleteStr, Expression >, do_parse!(
    initial: term >>
    remainder: many0!(
           alt!(
             do_parse!(tag!("+") >> add: term >> (Token::Plus, add)) |
             do_parse!(tag!("-") >> sub: term >> (Token::Minus, sub))
           )
         ) >>
    (fold_exprs(initial, remainder))
));

fn fold_exprs(initial: Expression, remainder: Vec<(Token, Expression)>) -> Expression {
    remainder.into_iter().fold(initial, |acc, pair| {
        let (oper, expr) = pair;
        Expression::Infix(Box::new(InfixExpression {
            operator: oper,
            left: acc,
            right: expr,
        }))
    })
}

fn sign_exprs(sign: Option<nom::types::CompleteStr<'_>>, factor: Expression) -> Expression {
    match sign {
        None => factor,
        Some(s) => Expression::Prefix(Box::new(PrefixExpression {
            operator: Token::Minus,
            right: factor,
        }))
    }
}

#[cfg(test)]
mod tests {
    use ast::*;
    use lexer::Lexer;
    use parser::*;
    use token;
    use combine::*;
    use nom::Context::Code;
    use nom::Err::Error;
    use nom::Err::Incomplete;
    use nom::ErrorKind::Alt;
    use nom::Needed::Size;
    use nom::types::CompleteStr;

    #[test]
    fn test_factor() {
        assert_eq!(factor(CompleteStr("1103")), Ok((CompleteStr(""), Expression::Integer(1103))));
        assert_eq!(factor(CompleteStr(" 1103")), Ok((CompleteStr(""), Expression::Integer(1103))));
        assert_eq!(factor(CompleteStr("1103  ")), Ok((CompleteStr(""), Expression::Integer(1103))));
        assert_eq!(factor(CompleteStr("  1103   1")), Ok((CompleteStr("1"), Expression::Integer(1103))));
    }

    #[test]
    fn test_expr() {
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
            assert_eq!(expr(CompleteStr(e.0)).unwrap(), (CompleteStr(";"), e.1));
        }
    }

    #[test]
    #[ignore]
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
    #[ignore]
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
    #[ignore]
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
