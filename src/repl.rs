use lexer;
use std::io;

pub fn start<R: io::BufRead, W: io::Write>(mut r: R, mut w: W) -> io::Result<()> {
    loop {
        let _i = w.write("> ".as_bytes());
        let _s = w.flush();
        let mut line = String::new();
        let _result = r.read_line(&mut line);

        let l = lexer::Lexer::new(line.as_str());
        for tok in l {
            println!("{:?}", tok)
        }
    }
}