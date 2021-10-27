use std::{cell::RefCell, rc::Rc};

use crate::ast::Node;

pub fn compile_ast(_node: Rc<RefCell<Node>>) -> bool {
    /* You should remove this cast to void in your solution.
     * It is just here so the code compiles without warnings. */
    return false;
    // for now, every statement causes a compilation failure
}
