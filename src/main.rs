pub mod ast;
pub mod compile;
pub mod compiler;
pub mod parser;
use compiler::compiler_entrypoint;

pub fn main() {
    let args: Vec<String> = std::env::args().collect();
    std::process::exit(compiler_entrypoint(args.len(), args) as i32);
}
