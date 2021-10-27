extern crate asmgen;
use std::{cell::RefCell, fs::File, rc::Rc};

use asmgen::ast::{free_ast, print_ast, Node};
use asmgen::compile::compile_ast;
use asmgen::compiler::{footer, header};
use asmgen::parser::parse;

#[test]
fn basic_division() {
    let ast: Option<Rc<RefCell<Node>>>;

    let program = File::open("./progs/stage4-basic-division.bas").unwrap();

    header();
    ast = parse(program);
    // file is dropped when this scope exits.

    assert!(ast.is_some());

    match ast.clone() {
        None => {
            eprintln!("Parse error");
            // return 2;
        }
        Some(u_ast) => {
            // Display the AST for debugging purposes
            print_ast(u_ast.clone());
            // Compile the AST into assembly instructions
            if !compile_ast(u_ast.clone()) {
                free_ast(ast.clone());
                eprintln!("Compilation error\n");
                // return 3;
            }
        }
    }

    free_ast(ast);
    footer();
}

#[test]
fn lots_of_ops() {
    let ast: Option<Rc<RefCell<Node>>>;

    let program = File::open("./progs/stage4-lots-of-ops.bas").unwrap();

    header();
    ast = parse(program);
    // file is dropped when this scope exits.

    assert!(ast.is_some());

    match ast.clone() {
        None => {
            eprintln!("Parse error");
            // return 2;
        }
        Some(u_ast) => {
            // Display the AST for debugging purposes
            print_ast(u_ast.clone());
            // Compile the AST into assembly instructions
            if !compile_ast(u_ast.clone()) {
                free_ast(ast.clone());
                eprintln!("Compilation error\n");
                // return 3;
            }
        }
    }

    free_ast(ast);
    footer();
}
