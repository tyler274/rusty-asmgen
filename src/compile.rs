use std::{cell::RefCell, ops::Deref, rc::Rc};

use crate::ast::Node;

pub fn print_indent(mut indent: usize) {
    while indent > 0 {
        print!("\t");
        indent = indent.wrapping_sub(1);
    }
}

pub fn compile_ast(_node: Rc<RefCell<Node>>) -> bool {
    match _node.clone().borrow().deref() {
        Node::Num { value } => {
            // print_indent(1);
            // print!("push %rdi\n");
            print_indent(1);
            print!("mov ${:#X}, %rdi\n", value);
        }
        Node::Binary { op, left, right } => todo!(),
        Node::Var { name } => todo!(),
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
        Node::LetNode { var, value } => todo!(),
        Node::IfNode {
            condition,
            if_branch,
            else_branch,
        } => todo!(),
        Node::WhileNode { condition, body } => todo!(),
    }

    // return false;
    true
    // for now, every statement causes a compilation failure
}
