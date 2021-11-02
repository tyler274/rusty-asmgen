use std::{cell::RefCell, rc::Rc};

use crate::ast::Node;

pub fn compile_ast(_node: Rc<RefCell<Node>>) -> bool {
    // match node.clone().borrow().deref() {
    //     Node::Num { value } => todo!(),
    //     Node::Binary { op, left, right } => todo!(),
    //     Node::Var { name } => todo!(),
    //     Node::Sequence {
    //         statement_count,
    //         statements,
    //     } => todo!(),
    //     Node::PrintNode { expr } => todo!(),
    //     Node::LetNode { var, value } => todo!(),
    //     Node::IfNode {
    //         condition,
    //         if_branch,
    //         else_branch,
    //     } => todo!(),
    //     Node::WhileNode { condition, body } => todo!(),
    // }

    // return false;
    true
    // for now, every statement causes a compilation failure
}
