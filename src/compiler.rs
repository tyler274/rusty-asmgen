use std::{cell::RefCell, fs::File, rc::Rc};

use crate::ast::{free_ast, print_ast, Node};
use crate::compile::compile_ast;
use crate::parser::parse;

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
    print!(
        r#"# The code section of the assembly file
.text
.globl basic_main
basic_main:
    # Save these callee-saved registers before we store the stack 
    pushq %rbp
    movq %rsp, %rbp

    # Allocate 208 bytes on the stack
    subq $0xD0, %rsp
    # The main() function
"#
    );
}

/* *
 * Prints the end of the x86-64 assembly output.
 * The assembly code implementing the TeenyBASIC statements
 * goes between the header and the footer.
 */
pub fn footer() {
    print!(
        r#"
    # Free the stack space allocated in the header
    leave

    # Return
    ret
"#
    );
}

pub fn compiler_entrypoint(argc: usize, argv: Vec<String>) -> i32 {
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
        // file is dropped/freed when this scope exits.
    }

    match ast.clone() {
        None => {
            eprintln!("Parse error");
            return 2;
        }
        Some(u_ast) => {
            // Display the AST for debugging purposes
            print_ast(u_ast.clone());
            let mut program_counter: usize = 0;
            // Compile the AST into assembly instructions
            if !compile_ast(u_ast.clone(), &mut program_counter) {
                free_ast(ast);
                eprint!("Compilation error\n");
                return 3;
            }
        }
    }

    free_ast(ast);
    footer();
    return 0;
}
