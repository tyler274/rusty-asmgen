pub mod ast;
pub mod compile;
pub mod parser;

use std::{cell::RefCell, fs::File, rc::Rc};

use ast::{free_ast, print_ast, Node};
use compile::compile_ast;
use parser::parse;

pub fn usage(program: &str) {
    eprint!("USAGE: {} <program file>\n", program);
    std::process::exit(1)
}
/* *
 * Prints the start of the the x86-64 assembly output.
 * The assembly code implementing the TeenyBASIC statements
 * goes between the header and the footer.
 */
pub fn header() {
    print!("# The code section of the assembly file\n.text\n.globl basic_main\nbasic_main:\n    # The main() function\n");
}
/* *
 * Prints the end of the x86-64 assembly output.
 * The assembly code implementing the TeenyBASIC statements
 * goes between the header and the footer.
 */
pub fn footer() {
    // printf(b"    ret\n\x00" as *const u8 as *const libc::c_char);
    print!("    ret\n");
}
fn main_0(argc: usize, argv: Vec<String>) -> i32 {
    if argc != 2 {
        usage(argv.get(0).unwrap());
    }
    let ast: Option<Rc<RefCell<Node>>>;
    {
        let program = File::open(argv.get(1).unwrap()).unwrap();
        match program.metadata() {
            Ok(metadata) => {
                if metadata.len() == 0 {
                    usage(argv.get(0).unwrap());
                }
            }
            Err(e) => {
                usage(argv.get(0).unwrap());
                eprintln!("Error: {}", e);
            }
        }
        header();
        ast = parse(program);
        // file is dropped when this scope exits.
    }

    match ast.clone() {
        None => {
            eprintln!("Parse error");
            return 2;
        }
        Some(u_ast) => {
            // Display the AST for debugging purposes
            print_ast(u_ast.clone());
            // Compile the AST into assembly instructions
            if !compile_ast(u_ast.clone()) {
                free_ast(ast);
                eprintln!("Compilation error\n");
                return 3;
            }
        }
    }

    free_ast(ast);
    footer();
    return 0;
}

#[main]
pub fn main() {
    let args: Vec<String> = std::env::args().collect();
    // args.push(::std::ptr::null_mut());

    std::process::exit(main_0(args.len(), args) as i32);
}
