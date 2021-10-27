extern crate asmgen;
use std::{cell::RefCell, fs::File, rc::Rc};

use asmgen::ast::{free_ast, print_ast, Node};
use asmgen::compile::compile_ast;
use asmgen::compiler::{footer, header};
use asmgen::parser::parse;

#[test]
fn bad_order() {
    let ast: Option<Rc<RefCell<Node>>>;

    let program = File::open("./progs/stage3-bad-order.bas").unwrap();

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

    let program = File::open("./progs/stage3-lots-of-ops.bas").unwrap();

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
fn overflow() {
    let ast: Option<Rc<RefCell<Node>>>;

    let program = File::open("./progs/stage3-overflow.bas").unwrap();

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
