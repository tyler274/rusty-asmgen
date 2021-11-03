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
    print!("pushq %rdi\n");
    compile_ast(right.clone());
    print_indent(1);
    print!("popq %rdx\n");
    print_indent(1);
    print!("addq %rdx, %rdi\n");
}

fn sub_helper(left: Rc<RefCell<Node>>, right: Rc<RefCell<Node>>) {
    compile_ast(left.clone());
    print_indent(1);
    print!("pushq %rdi\n");
    compile_ast(right.clone());
    print_indent(1);
    print!("movq %rdi, %rdx\n");
    print_indent(1);
    print!("popq %rdi\n");
    print_indent(1);
    print!("subq %rdx, %rdi\n");
}

fn mul_helper(left: Rc<RefCell<Node>>, right: Rc<RefCell<Node>>) {
    compile_ast(left.clone());
    print_indent(1);
    print!("pushq %rdi\n");
    compile_ast(right.clone());
    print_indent(1);
    print!("popq %rdx\n");
    print_indent(1);
    print!("imulq %rdx, %rdi\n");
}

fn div_helper(left: Rc<RefCell<Node>>, right: Rc<RefCell<Node>>) {
    compile_ast(left.clone());
    print_indent(1);
    print!("pushq %rdi\n");
    compile_ast(right.clone());
    print_indent(1);
    print!("popq %rax\n");

    // cast %rax to 128bit divisor using %rdx
    print_indent(1);
    print!("cqto\n");

    print_indent(1);
    print!("idivq %rdi\n");
    print_indent(1);
    print!("movq %rax, %rdi\n");
}

// 208=0xD0 bytes in the stack spill.
// fn stack_var_map() {}

pub fn compile_ast(_node: Rc<RefCell<Node>>) -> bool {
    match _node.clone().borrow().deref() {
        Node::Num { value } => {
            // print_indent(1);
            // print!("push %rdi\n");
            print_indent(1);
            print!("movq ${:#X}, %rdi\n", value);
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
        Node::Var { name } => {
            // Get the offset from the stack frame base pointer using ascii A=65...
            let n = name.to_ascii_uppercase() - 64;
            print_indent(1);
            print!("movq -{}(%rbp), %rdi\n", n * 8);
        }
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
        Node::LetNode { var, value } => {
            // Get the offset from the stack frame base pointer using ascii A=65...
            compile_ast(value.clone());
            let n = var.to_ascii_uppercase() - 64;
            print!("movq %rdi, -{}(%rbp)\n", n * 8);
        }
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
