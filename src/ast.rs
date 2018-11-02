use std::fmt;
use token;

#[derive(Debug)]
pub enum Node {
    Program(Box<Program>),
    Statement(Box<Statement>),
    Expression(Box<Expression>),
}

#[derive(Debug, Clone)]
pub enum Statement {
    Let(Box<LetStatement>),
    Expression(Box<ExpressionStatement>),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Expression {
    Identifier(String),
    Integer(i64),
    Prefix(Box<PrefixExpression>),
    Infix(Box<InfixExpression>),
}

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl Program {
    pub fn new() -> Program {
        Program {
            statements: Vec::new(),
        }
    }
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!()
    }
}

#[derive(Debug, Clone)]
pub struct LetStatement {
    pub name: String,
    pub value: Expression,
}

#[derive(Debug, Clone)]
pub struct ExpressionStatement {
    pub expression: Expression,
}

impl fmt::Display for ExpressionStatement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!()
    }
}

// Expression

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct InfixExpression {
    pub operator: token::Token,
    pub left: Expression,
    pub right: Expression,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PrefixExpression {
    pub operator: token::Token,
    pub right: Expression,
}
