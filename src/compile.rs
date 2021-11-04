use std::{cell::RefCell, ops::Deref, rc::Rc};

use crate::ast::Node;

pub fn print_indent(mut indent: usize) {
    while indent > 0 {
        print!("\t");
        indent = indent.wrapping_sub(1);
    }
}

fn add_helper(
    left: Rc<RefCell<Node>>,
    right: Rc<RefCell<Node>>,
    program_counter: &mut usize,
) -> bool {
    callee_reg_save();
    binary_ops_reg_helper(left.clone(), right.clone(), program_counter);
    print_indent(1);
    print!("addq %r13, %r12\n");
    print_indent(1);
    print!("movq %r12, %rdi\n");
    callee_reg_restore();
    true
}

fn sub_helper(
    left: Rc<RefCell<Node>>,
    right: Rc<RefCell<Node>>,
    program_counter: &mut usize,
) -> bool {
    callee_reg_save();
    binary_ops_reg_helper(left.clone(), right.clone(), program_counter);
    print_indent(1);
    print!("subq %rdi, %r12\n");
    print_indent(1);
    print!("movq %r12, %rdi\n");
    callee_reg_restore();
    true
}

fn mul_helper(
    left: Rc<RefCell<Node>>,
    right: Rc<RefCell<Node>>,
    program_counter: &mut usize,
) -> bool {
    callee_reg_save();
    binary_ops_reg_helper(left.clone(), right.clone(), program_counter);
    print_indent(1);
    print!("imulq %rdi, %r12\n");
    print_indent(1);
    print!("movq %r12, %rdi\n");
    callee_reg_restore();
    true
}

fn div_helper(
    left: Rc<RefCell<Node>>,
    right: Rc<RefCell<Node>>,
    program_counter: &mut usize,
) -> bool {
    callee_reg_save();
    binary_ops_reg_helper(left.clone(), right.clone(), program_counter);
    print_indent(1);
    print!("movq %r12, %rax\n");

    // cast %rax to 128bit divisor using %rdx
    print_indent(1);
    print!("cqto\n");

    print_indent(1);
    print!("idivq %r13\n");
    print_indent(1);
    print!("movq %rax, %rdi\n");

    callee_reg_restore();
    true
}

fn callee_reg_save() {
    print_indent(1);
    print!("pushq %r12\n");
    print_indent(1);
    print!("pushq %r13\n");
}

fn callee_reg_restore() {
    // restore our calle-saved registers that will survive being clobered
    print_indent(1);
    print!("popq %r13\n");
    print_indent(1);
    print!("popq %r12\n");
}

fn binary_ops_reg_helper(
    left: Rc<RefCell<Node>>,
    right: Rc<RefCell<Node>>,
    program_counter: &mut usize,
) {
    // evaluate our left hand side
    *program_counter += 1;
    compile_ast(left.clone(), program_counter);

    print_indent(1);
    print!("movq %rdi, %r12\n");

    // evaluate our right hand side
    *program_counter += 1;
    compile_ast(right.clone(), program_counter);

    print_indent(1);
    print!("movq %rdi, %r13\n");
}

fn num_helper(value: i64) -> bool {
    print_indent(1);
    print!("movq ${:#X}, %rdi\n", value);
    true
}

fn var_helper(name: u8) -> bool {
    // Get the offset from the stack frame base pointer using ascii A=65...
    // We have to 1 index here or its a caller-callee convention violation.
    let n = name.to_ascii_uppercase() - 64;
    print_indent(1);
    print!("movq -{}(%rbp), %rdi\n", n * 8);
    true
}

fn sequence_helper(
    statement_count: usize,
    statements: Rc<RefCell<Vec<Option<Rc<RefCell<Node>>>>>>,
    program_counter: &mut usize,
) -> bool {
    let mut statement_counter: usize = 0;
    while statement_counter < statement_count {
        *program_counter += 1;
        compile_ast(
            statements
                .deref()
                .borrow()
                .get(statement_counter)
                .unwrap()
                .as_ref()
                .unwrap()
                .clone(),
            program_counter,
        );
        statement_counter += 1;
    }
    true
}

fn print_helper(expr: Rc<RefCell<Node>>, program_counter: &mut usize) -> bool {
    *program_counter += 1;
    compile_ast(expr.clone(), program_counter);
    print_indent(1);
    print!("call print_int\n");
    true
}

fn let_helper(var: u8, value: Rc<RefCell<Node>>, program_counter: &mut usize) -> bool {
    // Get the offset from the stack frame base pointer using ascii A=65...
    // We have to 1 index here or its a caller-callee convention violation.
    *program_counter += 1;
    compile_ast(value.clone(), program_counter);
    let n = var.to_ascii_uppercase() - 64;
    print_indent(1);
    print!("movq %rdi, -{}(%rbp)\n", n * 8);
    true
}

fn equality_helper(
    left: Rc<RefCell<Node>>,
    right: Rc<RefCell<Node>>,
    program_counter: &mut usize,
) -> bool {
    callee_reg_save();
    binary_ops_reg_helper(left.clone(), right.clone(), program_counter);
    callee_reg_restore();
    todo!();
    true
}

fn less_than_helper(
    left: Rc<RefCell<Node>>,
    right: Rc<RefCell<Node>>,
    program_counter: &mut usize,
) -> bool {
    callee_reg_save();
    binary_ops_reg_helper(left.clone(), right.clone(), program_counter);
    callee_reg_restore();
    todo!();
    true
}

fn greater_than_helper(
    left: Rc<RefCell<Node>>,
    right: Rc<RefCell<Node>>,
    program_counter: &mut usize,
) -> bool {
    callee_reg_save();
    binary_ops_reg_helper(left.clone(), right.clone(), program_counter);
    callee_reg_restore();
    todo!();
    true
}

pub fn compile_ast(node: Rc<RefCell<Node>>, program_counter: &mut usize) -> bool {
    return match node.clone().borrow().deref() {
        Node::Num { value } => num_helper(*value),
        Node::Binary { op, left, right } => match *op {
            b'+' => add_helper(left.clone(), right.clone(), program_counter),
            b'-' => sub_helper(left.clone(), right.clone(), program_counter),
            b'*' => mul_helper(left.clone(), right.clone(), program_counter),
            b'/' => div_helper(left.clone(), right.clone(), program_counter),
            b'=' => equality_helper(left.clone(), right.clone(), program_counter),
            b'<' => less_than_helper(left.clone(), right.clone(), program_counter),
            b'>' => greater_than_helper(left.clone(), right.clone(), program_counter),
            _ => {
                todo!();
            }
        },
        Node::Var { name } => var_helper(*name),
        Node::Sequence {
            statement_count,
            statements,
        } => sequence_helper(*statement_count, statements.clone(), program_counter),
        Node::PrintNode { expr } => print_helper(expr.clone(), program_counter),
        Node::LetNode { var, value } => let_helper(*var, value.clone(), program_counter),
        Node::IfNode {
            condition,
            if_branch,
            else_branch,
        } => {
            // structure of jumps is different if we have an else or not, so match on that
            // first to help setup that structure.
            match else_branch {
                Some(e_b) => {}
                None => {
                    // print_indent(1);
                    // print!("movq ");
                }
            }
            return false;
        }
        Node::WhileNode {
            condition: _,
            body: _,
        } => todo!(),
    };
}
