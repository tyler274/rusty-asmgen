use std::ops::Deref;

use crate::ast::{Node, NodeEnum, NodeVec};

pub fn print_indent(mut indent: usize) {
    while indent > 0 {
        print!("\t");
        indent = indent.wrapping_sub(1);
    }
}

fn add_helper(left: Node, right: Node, program_counter: &mut usize) -> bool {
    callee_reg_save();
    binary_ops_reg_helper(left, right, program_counter);
    print_indent(1);
    println!("addq %r13, %r12");
    print_indent(1);
    println!("movq %r12, %rdi");
    callee_reg_restore();
    true
}

fn sub_helper(left: Node, right: Node, program_counter: &mut usize) -> bool {
    callee_reg_save();
    binary_ops_reg_helper(left, right, program_counter);
    print_indent(1);
    println!("subq %rdi, %r12");
    print_indent(1);
    println!("movq %r12, %rdi");
    callee_reg_restore();
    true
}

fn mul_helper(left: Node, right: Node, program_counter: &mut usize) -> bool {
    callee_reg_save();
    binary_ops_reg_helper(left, right, program_counter);
    print_indent(1);
    println!("imulq %rdi, %r12");
    print_indent(1);
    println!("movq %r12, %rdi");
    callee_reg_restore();
    true
}

fn div_helper(left: Node, right: Node, program_counter: &mut usize) -> bool {
    callee_reg_save();
    binary_ops_reg_helper(left, right, program_counter);
    print_indent(1);
    println!("movq %r12, %rax");

    // cast %rax to 128bit divisor using %rdx
    print_indent(1);
    println!("cqto");

    print_indent(1);
    println!("idivq %r13");
    print_indent(1);
    println!("movq %rax, %rdi");

    callee_reg_restore();
    true
}

fn callee_reg_save() {
    print_indent(1);
    println!("pushq %r12");
    print_indent(1);
    println!("pushq %r13");
}

fn callee_reg_restore() {
    // restore our calle-saved registers that will survive being clobered
    print_indent(1);
    println!("popq %r13");
    print_indent(1);
    println!("popq %r12");
}

fn binary_ops_reg_helper(left: Node, right: Node, program_counter: &mut usize) {
    // evaluate our left hand side
    *program_counter += 1;
    compile_ast(left, program_counter);

    print_indent(1);
    println!("movq %rdi, %r12");

    // evaluate our right hand side
    *program_counter += 1;
    compile_ast(right, program_counter);

    print_indent(1);
    println!("movq %rdi, %r13");
}

fn num_helper(value: i64) -> bool {
    print_indent(1);
    println!("movq ${:#X}, %rdi", value);
    true
}

fn var_helper(name: u8) -> bool {
    // Get the offset from the stack frame base pointer using ascii A=65...
    // We have to 1 index here or its a caller-callee convention violation.
    let n = name.to_ascii_uppercase() - 64;
    print_indent(1);
    println!("movq -{}(%rbp), %rdi", n * 8);
    true
}

fn sequence_helper(
    statement_count: usize,
    statements: NodeVec,
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

fn print_helper(expr: Node, program_counter: &mut usize) -> bool {
    *program_counter += 1;
    compile_ast(expr, program_counter);
    print_indent(1);
    println!("call print_int");
    true
}

fn let_helper(var: u8, value: Node, program_counter: &mut usize) -> bool {
    // Get the offset from the stack frame base pointer using ascii A=65...
    // We have to 1 index here or its a caller-callee convention violation.
    *program_counter += 1;
    compile_ast(value, program_counter);
    let n = var.to_ascii_uppercase() - 64;
    print_indent(1);
    println!("movq %rdi, -{}(%rbp)", n * 8);
    true
}

fn equality_helper(left: Node, right: Node, program_counter: &mut usize) -> bool {
    callee_reg_save();
    binary_ops_reg_helper(left, right, program_counter);
    callee_reg_restore();
    todo!();
}

fn less_than_helper(left: Node, right: Node, program_counter: &mut usize) -> bool {
    callee_reg_save();
    binary_ops_reg_helper(left, right, program_counter);
    callee_reg_restore();
    todo!();
}

fn greater_than_helper(left: Node, right: Node, program_counter: &mut usize) -> bool {
    callee_reg_save();
    binary_ops_reg_helper(left, right, program_counter);
    callee_reg_restore();
    todo!();
}

pub fn compile_ast(node: Node, program_counter: &mut usize) -> bool {
    return match node.borrow().deref() {
        NodeEnum::Num { value } => num_helper(*value),
        NodeEnum::Binary { op, left, right } => match *op {
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
        NodeEnum::Var { name } => var_helper(*name),
        NodeEnum::Sequence {
            statement_count,
            statements,
        } => sequence_helper(*statement_count, statements.clone(), program_counter),
        NodeEnum::PrintNode { expr } => print_helper(expr.clone(), program_counter),
        NodeEnum::LetNode { var, value } => let_helper(*var, value.clone(), program_counter),
        NodeEnum::IfNode {
            condition: _,
            if_branch: _,
            else_branch,
        } => {
            // structure of jumps is different if we have an else or not, so match on that
            // first to help setup that structure.
            match else_branch {
                Some(_e_b) => {}
                None => {
                    // print_indent(1);
                    // print!("movq ");
                }
            }
            return false;
        }
        NodeEnum::WhileNode {
            condition: _,
            body: _,
        } => todo!(),
    };
}
