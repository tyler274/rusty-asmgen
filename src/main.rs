pub mod ast;
pub mod compile;
pub mod compiler;
pub mod parser;
use compiler::main_0;

pub fn main() {
    let args: Vec<String> = std::env::args().collect();
    std::process::exit(main_0(args.len(), args) as i32);
}
