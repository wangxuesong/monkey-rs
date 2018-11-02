use evaluator;
use parser;
use std::io;

pub fn start<R: io::BufRead, W: io::Write>(mut r: R, mut w: W) -> io::Result<()> {
    loop {
        let _i = w.write("> ".as_bytes());
        let _s = w.flush();
        let mut line = String::new();
        let _result = r.read_line(&mut line);

        let obj = match parser::parse(line.as_str()) {
            Ok(node) => evaluator::eval(&node),
            Err(e) => Err(evaluator::EvalError { message: e }),
        };
        match obj {
            Ok(o) => println!("{:?}", o.inspect()),
            Err(e) => println!("{:?}", e),
        }
    }
}
