use std::{cell::RefCell, ops::Deref, rc::Rc};

use crate::ast::Node;

pub fn print_indent(mut indent: usize) {
    while indent > 0 {
        print!("\t");
        indent = indent.wrapping_sub(1);
    }
}

fn add_helper(left: Rc<RefCell<Node>>, right: Rc<RefCell<Node>>) {
    compile_ast(left.clone());
    print_indent(1);
    print!("push %rdi\n");
    compile_ast(right.clone());
    // print_indent(1);
    // print!("push %rdi\n");
    // // easy optimization here
    // print_indent(1);
    // print!("pop %rdi\n");
    print_indent(1);
    print!("pop %rdx\n");
    print_indent(1);
    print!("addq %rdx, %rdi\n");
}

fn sub_helper(left: Rc<RefCell<Node>>, right: Rc<RefCell<Node>>) {
    compile_ast(left.clone());
    print_indent(1);
    print!("push %rdi\n");
    compile_ast(right.clone());
    print_indent(1);
    print!("mov %rdi, %rdx\n");
    print_indent(1);
    print!("pop %rdi\n");
    print_indent(1);
    print!("subq %rdx, %rdi\n");
}

fn mul_helper(left: Rc<RefCell<Node>>, right: Rc<RefCell<Node>>) {
    compile_ast(left.clone());
    print_indent(1);
    print!("push %rdi\n");
    compile_ast(right.clone());
    print_indent(1);
    print!("push %rdi\n");
    // easy optimization here
    print_indent(1);
    print!("pop %rdx\n");
    print_indent(1);
    print!("pop %rdi\n");
    print_indent(1);
    print!("imulq %rdx, %rdi\n");
}

fn div_helper(left: Rc<RefCell<Node>>, right: Rc<RefCell<Node>>) {
    compile_ast(left.clone());
    print_indent(1);
    print!("push %rdi\n");
    compile_ast(right.clone());
    print_indent(1);
    print!("push %rdi\n");
    // easy optimization here
    print_indent(1);
    print!("pop %rdi\n");
    print_indent(1);
    print!("pop %rax\n");

    // cast %rax to 128bit divisor using %rdx
    print_indent(1);
    print!("cqto\n");

    print_indent(1);
    print!("idivq %rdi\n");
    print_indent(1);
    print!("mov %rax, %rdi\n");
}

pub fn compile_ast(_node: Rc<RefCell<Node>>) -> bool {
    match _node.clone().borrow().deref() {
        Node::Num { value } => {
            // print_indent(1);
            // print!("push %rdi\n");
            print_indent(1);
            print!("mov ${:#X}, %rdi\n", value);
        }
        Node::Binary { op, left, right } => match *op {
            b'+' => add_helper(left.clone(), right.clone()),
            b'-' => sub_helper(left.clone(), right.clone()),
            b'*' => mul_helper(left.clone(), right.clone()),
            b'/' => div_helper(left.clone(), right.clone()),
            _ => {
                todo!();
            }
        },
        Node::Var { name: _ } => todo!(),
        Node::Sequence {
            statement_count,
            statements,
        } => {
            let mut program_counter: usize = 0;
            while program_counter < *statement_count {
                compile_ast(
                    statements
                        .deref()
                        .borrow()
                        .get(program_counter)
                        .unwrap()
                        .as_ref()
                        .unwrap()
                        .clone(),
                );
                program_counter += 1;
            }
        }
        Node::PrintNode { expr } => {
            compile_ast(expr.clone());
            print_indent(1);
            print!("call print_int\n");
        }
        Node::LetNode { var: _, value: _ } => todo!(),
        Node::IfNode {
            condition: _,
            if_branch: _,
            else_branch: _,
        } => todo!(),
        Node::WhileNode {
            condition: _,
            body: _,
        } => todo!(),
    }

    // return false;
    true
    // for now, every statement causes a compilation failure
}
