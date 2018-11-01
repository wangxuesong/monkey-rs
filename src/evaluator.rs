use ast::*;
use object::Object;

pub type EvalResult = Result<Object, EvalError>;

#[derive(Debug)]
pub struct EvalError {
    pub message: String,
}

pub fn eval(node: &Node) -> EvalResult {
    match node {
        Node::Program(prog) => eval_program(&prog),
        Node::Statement(stmt) => eval_statement(&stmt),
        Node::Expression(exp) => eval_expression(&exp),
    }
}

fn eval_program(prog: &Program) -> EvalResult {
    let mut result = Object::Int(0);
    for stmt in &prog.statements {
        let res = eval_statement(&stmt)?;
        result = res
    }
    Ok(result)
}

fn eval_statement(stmt: &Statement) -> EvalResult {
    match stmt {
        Statement::Expression(exp) => eval_expression(&exp.expression),
        Statement::Let(_) => Err(EvalError {
            message: "eval let statement".to_string(),
        }),
    }
}

fn eval_expression(exp: &Expression) -> EvalResult {
    match exp {
        Expression::Integer(i) => Ok(Object::Int(*i)),
        _ => Err(EvalError {
            message: "eval expression".to_string(),
        }),
    }
}

#[cfg(test)]
mod test {
    use parser;
    use super::*;

    #[test]
    fn eval_integer_expression() {
        let test = vec![("1103;", 1103)];

        for t in test {
            let obj = match parser::parse(t.0) {
                Ok(node) => eval(&node).expect(t.0),
                Err(e) => panic!(e),
            };

            match obj {
                Object::Int(i) => assert_eq!(t.1, i),
                _ => panic!(),
            }
        }
    }
}
