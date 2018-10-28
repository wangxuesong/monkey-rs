extern crate monkey;

use monkey::repl;
use std::io;

fn main() -> io::Result<()> {
    println!("Hello, world!");
    let input = std::io::stdin();
    let output = std::io::stdout();
    let result = repl::start(input.lock(), output.lock());
    result
}
