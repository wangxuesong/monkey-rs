use lexer;
use parser;
use std::io;

pub fn start<R: io::BufRead, W: io::Write>(mut r: R, mut w: W) -> io::Result<()> {
    loop {
        let _i = w.write("> ".as_bytes());
        let _s = w.flush();
        let mut line = String::new();
        let _result = r.read_line(&mut line);

        let l = lexer::Lexer::new(line.as_str());
        let mut p = parser::Parser::new(l);
        let r = p.parse_program();
        match r {
            Ok(prog) => {
                for stmt in prog.statements {
                    println!("{:?}", stmt);
                }
            }
            Err(e) => println!("error: {:?}", e),
        }
    }
}
