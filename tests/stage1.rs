extern crate asmgen;
use std::{cell::RefCell, fs::File, rc::Rc};

use asmgen::ast::{free_ast, print_ast, Node};
use asmgen::compile::compile_ast;
use asmgen::compiler::{footer, header, usage};
use asmgen::parser::parse;

#[test]
fn leet() {
    let ast: Option<Rc<RefCell<Node>>>;

    let program = File::open("./progs/stage1-1337.bas").unwrap();

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
fn forty_two() {
    let ast: Option<Rc<RefCell<Node>>>;

    let program = File::open("./progs/stage1-42.bas").unwrap();

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
fn print_multiple() {
    let ast: Option<Rc<RefCell<Node>>>;

    let program = File::open("./progs/stage1-print-multiple.bas").unwrap();

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
