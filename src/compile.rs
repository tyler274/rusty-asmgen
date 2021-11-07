use std::{cell::RefCell, ops::Deref, rc::Rc};

use crate::ast::{init_let_node, init_sequence_node, Node, NodeEnum, NodeVec};

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
    println!("addq %r12, %rdi");
    // print_indent(1);
    // println!("movq %r12, %rdi");
    callee_reg_restore();
    true
}

fn sub_helper(left: Node, right: Node, program_counter: &mut usize) -> bool {
    callee_reg_save();
    // avoid the extra movq
    binary_ops_reg_helper(right, left, program_counter);
    print_indent(1);
    println!("subq %r12, %rdi");
    // print_indent(1);
    // println!("movq %r12, %rdi");
    callee_reg_restore();
    true
}

fn mul_helper(left: Node, right: Node, program_counter: &mut usize) -> bool {
    callee_reg_save();
    binary_ops_reg_helper(left, right, program_counter);
    print_indent(1);
    // match (left.borrow().deref(), right.borrow().deref()) {
    //     (NodeEnum::Num { value }, _) => {
    //         if *value % 2 == 0 && (63 - value.leading_zeros() < 63) {
    //             println!("shlq ${}, %rdi", 63 - value.leading_zeros());
    //         } else {
    //             println!("imulq %r12, %rdi");
    //         }
    //     }
    //     // (_, NodeEnum::Num { value }) => {
    //     //     if *value % 2 == 0 && (63 - value.abs().leading_zeros() < 64) {
    //     //         println!("shlq ${}, %r12", 63 - value.leading_zeros());

    //     //         print_indent(1);
    //     //         println!("movq %r12, %rdi");
    //     //     } else {
    //     //         println!("imulq %r12, %rdi");
    //     //     }
    //     // }
    //     (_, _) => {
    //         println!("imulq %r12, %rdi");
    //     }
    // };

    // No strength reduction
    println!("imulq %r12, %rdi");

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
    // *program_counter += 1;
    compile_ast(left, program_counter);

    print_indent(1);
    println!("movq %rdi, %r12");

    // evaluate our right hand side
    // *program_counter += 1;
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
        // *program_counter += 1;
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
    // *program_counter += 1;
    compile_ast(expr, program_counter);
    print_indent(1);
    println!("call print_int");
    true
}

fn let_helper(var: u8, value: Node, program_counter: &mut usize) -> bool {
    // Get the offset from the stack frame base pointer using ascii A=65...
    // We have to 1 index here or its a caller-callee convention violation.
    // *program_counter += 1;
    compile_ast(value, program_counter);
    let n = var.to_ascii_uppercase() - 64;
    print_indent(1);
    println!("movq %rdi, -{}(%rbp)", n * 8);
    true
}

fn equality_helper(left: Node, right: Node, program_counter: &mut usize) -> bool {
    callee_reg_save();
    let saved_pc = *program_counter;
    binary_ops_reg_helper(left, right, program_counter);

    print_indent(1);
    println!("cmp %r12, %r13");
    callee_reg_restore();

    print_indent(1);
    println!("jne .LBIF{}_2", saved_pc);

    true
}

fn less_than_helper(left: Node, right: Node, program_counter: &mut usize) -> bool {
    callee_reg_save();
    let saved_pc = *program_counter;
    binary_ops_reg_helper(left, right, program_counter);

    print_indent(1);
    println!("cmp %r12, %r13");
    callee_reg_restore();

    print_indent(1);
    println!("jle .LBIF{}_2", saved_pc);

    true
}

fn greater_than_helper(left: Node, right: Node, program_counter: &mut usize) -> bool {
    callee_reg_save();
    let saved_pc = *program_counter;
    binary_ops_reg_helper(left, right, program_counter);

    print_indent(1);
    println!("cmp %r12, %r13");

    callee_reg_restore();

    print_indent(1);
    println!("jge .LBIF{}_2", saved_pc);

    true
}

fn if_helper(
    condition: Node,
    if_branch: Node,
    else_branch: Option<Node>,
    program_counter: &mut usize,
) -> bool {
    *program_counter += 1;
    let saved_pc = *program_counter;
    compile_ast(condition, program_counter);

    // *program_counter += 1;
    compile_ast(if_branch, program_counter);

    // structure of jumps is different if we have an else or not, so match on that
    // first to help setup that structure.
    match else_branch {
        Some(_e_b) => {
            print_indent(1);
            println!("jmp .LBIF{}_3", saved_pc);

            println!(".LBIF{}_2:", saved_pc);

            *program_counter += 1;
            compile_ast(_e_b, program_counter);

            println!(".LBIF{}_3:", saved_pc);
        }
        None => {
            println!(".LBIF{}_2:", saved_pc);
        }
    }
    true
}

fn while_helper(condition: Node, body: Node, program_counter: &mut usize) -> bool {
    *program_counter += 1;
    let saved_pc = *program_counter;

    println!(".LBWHILE{}_2:", saved_pc);
    compile_ast(condition, program_counter);

    compile_ast(body, program_counter);

    print_indent(1);
    println!("jmp .LBWHILE{}_2", saved_pc);

    println!(".LBIF{}_2:", saved_pc);

    true
}

// Implements the constant folding optimization by recursively executing nodes and putting
// the comple time computed values into the AST instead.
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
            condition,
            if_branch,
            else_branch,
        } => if_helper(
            condition.clone(),
            if_branch.clone(),
            else_branch.clone(),
            program_counter,
        ),
        NodeEnum::WhileNode { condition, body } => {
            while_helper(condition.clone(), body.clone(), program_counter)
        }
    };
}

pub fn optimize_ast(node: Node) -> Node {
    return match node.borrow().deref() {
        NodeEnum::Num { .. } => node.clone(),
        NodeEnum::Binary { op, left, right } => match *op {
            b'+' => match (
                (optimize_ast(left.clone())).borrow().deref(),
                (optimize_ast(right.clone())).borrow().deref(),
            ) {
                (NodeEnum::Num { value: value_l }, NodeEnum::Num { value: value_r }) => {
                    Rc::new(RefCell::new(NodeEnum::Num {
                        value: value_l.wrapping_add(*value_r),
                    }))
                }
                (_, _) => node.clone(),
            },
            b'-' => match (
                (optimize_ast(left.clone())).borrow().deref(),
                (optimize_ast(right.clone())).borrow().deref(),
            ) {
                (NodeEnum::Num { value: value_l }, NodeEnum::Num { value: value_r }) => {
                    Rc::new(RefCell::new(NodeEnum::Num {
                        value: value_l.wrapping_sub(*value_r),
                    }))
                }
                (_, _) => node.clone(),
            },
            b'*' => match (
                (optimize_ast(left.clone())).borrow().deref(),
                (optimize_ast(right.clone())).borrow().deref(),
            ) {
                (NodeEnum::Num { value: value_l }, NodeEnum::Num { value: value_r }) => {
                    Rc::new(RefCell::new(NodeEnum::Num {
                        value: value_l.wrapping_mul(*value_r),
                    }))
                }
                (_, _) => node.clone(),
            },
            b'/' => match (
                (optimize_ast(left.clone())).borrow().deref(),
                (optimize_ast(right.clone())).borrow().deref(),
            ) {
                (NodeEnum::Num { value: value_l }, NodeEnum::Num { value: value_r }) => {
                    Rc::new(RefCell::new(NodeEnum::Num {
                        value: value_l.wrapping_div(*value_r),
                    }))
                }
                (_, _) => node.clone(),
            },
            b'=' => Rc::new(RefCell::new(NodeEnum::Binary {
                op: *op,
                left: optimize_ast(left.clone()),
                right: optimize_ast(right.clone()),
            })),
            b'<' => Rc::new(RefCell::new(NodeEnum::Binary {
                op: *op,
                left: optimize_ast(left.clone()),
                right: optimize_ast(right.clone()),
            })),
            b'>' => Rc::new(RefCell::new(NodeEnum::Binary {
                op: *op,
                left: optimize_ast(left.clone()),
                right: optimize_ast(right.clone()),
            })),
            _ => {
                todo!();
            }
        },
        NodeEnum::Var { .. } => node.clone(),
        NodeEnum::Sequence {
            statement_count,
            statements,
        } => init_sequence_node(
            *statement_count,
            Rc::new(RefCell::new(
                statements
                    .deref()
                    .borrow_mut()
                    .deref()
                    .iter()
                    .map(|s| {
                        if s.is_none() {
                            None
                        } else {
                            Some(optimize_ast(s.as_ref().unwrap().clone()))
                        }
                    })
                    .collect(),
            )),
        )
        .unwrap(),
        NodeEnum::PrintNode { expr } => Rc::new(RefCell::new(NodeEnum::PrintNode {
            expr: optimize_ast(expr.clone()),
        })),
        NodeEnum::LetNode { var, value } => {
            init_let_node(*var, Some(optimize_ast(value.clone()))).unwrap()
        }
        NodeEnum::IfNode {
            condition,
            if_branch,
            else_branch,
        } => match else_branch {
            Some(e_b) => Rc::new(RefCell::new(NodeEnum::IfNode {
                condition: optimize_ast(condition.clone()),
                if_branch: optimize_ast(if_branch.clone()),
                else_branch: Some(optimize_ast(e_b.clone())),
            })),
            None => Rc::new(RefCell::new(NodeEnum::IfNode {
                condition: optimize_ast(condition.clone()),
                if_branch: optimize_ast(if_branch.clone()),
                else_branch: None,
            })),
        },
        NodeEnum::WhileNode { condition, body } => Rc::new(RefCell::new(NodeEnum::WhileNode {
            condition: optimize_ast(condition.clone()),
            body: optimize_ast(body.clone()),
        })),
    };
}
