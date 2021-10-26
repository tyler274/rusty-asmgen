#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![register_tool(c2rust)]
#![feature(extern_types, register_tool)]
pub mod ast;
pub mod compile;
pub mod parser;
// use ::c2rust_out::*;

use std::fs::File;
use std::io::prelude::*;

use ast::Node;
use parser::parse;

pub fn usage(program: &str) {
    eprintln!("USAGE: {} <program file>\n", program);
    // fprintf(
    //     stderr,
    //     b"USAGE: %s <program file>\n\x00" as *const u8 as *const libc::c_char,
    //     program,
    // );
    // exit(1 as libc::c_int);
    std::process::exit(1)
}
/* *
 * Prints the start of the the x86-64 assembly output.
 * The assembly code implementing the TeenyBASIC statements
 * goes between the header and the footer.
 */
pub fn header() {
    println!("# The code section of the assembly file\n.text\n.globl basic_main\nbasic_main:\n    # The main() function");
}
/* *
 * Prints the end of the x86-64 assembly output.
 * The assembly code implementing the TeenyBASIC statements
 * goes between the header and the footer.
 */
pub fn footer() {
    // printf(b"    ret\n\x00" as *const u8 as *const libc::c_char);
    println!("    ret");
}
unsafe fn main_0(mut argc: usize, mut argv: Vec<String>) -> i32 {
    if argc != 2 {
        usage(argv.get(0).unwrap());
    }
    let mut ast: Option<String> = None;
    {
        let mut program = File::open(argv.get(1).unwrap()).unwrap();
        // if program.is_null() {
        //     usage(argv.get(0));
        // }
        header();
        let mut ast = parse(program);
        // ast = Some(" ".to_string());
        // fclose(program);
    }

    // just stubbing this, we actually want to check if the parse happened
    if ast.unwrap().len() == 0 {
        eprintln!("Parse error");
        return 2;
    }
    // Display the AST for debugging purposes
    // print_ast(ast);
    // Compile the AST into assembly instructions
    // if !compile_ast(ast) {
    //     free_ast(ast);
    //     fprintf(
    //         stderr,
    //         b"Compilation error\n\x00" as *const u8 as *const libc::c_char,
    //     );
    //     return 3 as libc::c_int;
    // }
    // free_ast(ast);
    footer();
    return 0;
}
#[main]
pub fn main() {
    let mut args: Vec<String> = std::env::args().collect();
    // args.push(::std::ptr::null_mut());
    unsafe {
        std::process::exit(main_0(args.len(), args) as i32);
    }
}
