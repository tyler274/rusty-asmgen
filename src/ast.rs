use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub enum NodeEnum {
    Num {
        value: i64,
    },
    Binary {
        op: u8,
        left: Node,
        right: Node,
    },
    Var {
        name: u8,
    },
    Sequence {
        statement_count: usize,
        statements: NodeVec,
    },
    PrintNode {
        expr: Node,
    },
    LetNode {
        var: u8,
        value: Node,
    },
    IfNode {
        condition: Node,
        if_branch: Node,
        else_branch: Option<Node>,
    },
    WhileNode {
        condition: Node,
        body: Node,
    },
}

pub type Node = Rc<RefCell<NodeEnum>>;

pub type NodeVec = Rc<RefCell<Vec<Option<Node>>>>;

pub fn init_num_node(value: i64) -> Node {
    Rc::new(RefCell::new(NodeEnum::Num { value }))
}

pub fn init_binary_node(op: u8, left: Option<Node>, right: Option<Node>) -> Option<Node> {
    match (left, right) {
        (None, _) => {
            // free_ast(left);
            // free_ast(right);
            None
        }
        (_, None) => {
            // free_ast(left);
            // free_ast(right);
            None
        }
        (Some(l), Some(r)) => {
            let node = Rc::new(RefCell::new(NodeEnum::Binary {
                op,
                left: l,
                right: r,
            }));
            Some(node)
        }
    }
}

pub fn init_var_node(name: u8) -> Option<Node> {
    if name == b'\0' {
        return None;
    }
    let node = NodeEnum::Var { name };

    Some(Rc::new(RefCell::new(node)))
}

pub fn init_sequence_node(statement_count: usize, statements: NodeVec) -> Option<Node> {
    if (*statements).borrow().is_empty() && statement_count > 0 {
        return None;
    }
    let node = Rc::new(RefCell::new(NodeEnum::Sequence {
        statement_count,
        statements,
    }));
    Some(node)
}

pub fn init_print_node(expression: Option<Node>) -> Option<Node> {
    expression.map(|expr| Rc::new(RefCell::new(NodeEnum::PrintNode { expr })))
}

pub fn init_let_node(var: u8, value: Option<Node>) -> Option<Node> {
    if var == b'\0' {
        // free_ast(value);
        return None;
    }
    let node = Rc::new(RefCell::new(NodeEnum::LetNode {
        var,
        value: value.unwrap_or_else(|| panic!("Error initing LET node with var : {}", var)),
    }));
    Some(node)
}

pub fn init_if_node(
    condition: Option<Node>,
    if_branch: Option<Node>,
    else_branch: Option<Node>,
) -> Option<Node> {
    match (condition.clone(), if_branch.clone()) {
        (None, _) => {
            // free_ast(condition);
            // free_ast(if_branch);
            // free_ast(else_branch);
            None
        }
        (_, None) => {
            // free_ast(condition);
            // free_ast(if_branch);
            // free_ast(else_branch);
            None
        }
        (Some(_), Some(_)) => {
            let node = Rc::new(RefCell::new(NodeEnum::IfNode {
                condition: condition.unwrap(),
                if_branch: if_branch.unwrap(),
                else_branch,
            }));
            Some(node)
        }
    }
}

pub fn init_while_node(condition: Option<Node>, body: Option<Node>) -> Option<Node> {
    match (condition, body) {
        (None, _) => {
            // free_ast(condition);
            // free_ast(body);
            None
        }
        (_, None) => {
            // free_ast(condition);
            // free_ast(body);
            None
        }
        (Some(c), Some(b)) => {
            let node = Rc::new(RefCell::new(NodeEnum::WhileNode {
                body: b,
                condition: c,
            }));
            Some(node)
        }
    }
}

// I'm think this is taken care of by RAII and Drop.
// pub fn free_ast(node: Option<Node>) {
//     match node {
//         None => {
//             return;
//         }
//         // oh holy pointer hallowed be thy name I summon thee.
//         // Not really but the (de)referencing indirection here is amusing.
//         Some(n) => {
//             match &*(*n).borrow_mut() {
//                 NodeEnum::Num { value } => drop(value),
//                 NodeEnum::Binary { op, left, right } => {
//                     free_ast(Some(left.clone()));
//                     free_ast(Some(right.clone()));
//                     drop(op);
//                 }
//                 NodeEnum::Var { name } => drop(*name),
//                 NodeEnum::Sequence {
//                     statement_count,
//                     statements,
//                 } => {
//                     for i in 0..*statement_count {
//                         free_ast((*statements).borrow_mut()[i].clone());
//                     }
//                     // drop(*statements);
//                 }
//                 NodeEnum::PrintNode { expr } => free_ast(Some(expr.clone())),
//                 NodeEnum::LetNode { var, value } => {
//                     free_ast(Some(value.clone()));
//                 }
//                 NodeEnum::IfNode {
//                     condition,
//                     if_branch,
//                     else_branch,
//                 } => {
//                     free_ast(Some(condition.clone()));
//                     free_ast(Some(if_branch.clone()));
//                     free_ast(else_branch.clone());
//                 }
//                 NodeEnum::WhileNode { condition, body } => {
//                     free_ast(Some(condition.clone()));
//                     free_ast(Some(body.clone()));
//                 }
//             }
//             drop(n);
//         }
//     }
// }

pub fn eprint_indent(mut indent: usize) {
    while indent > 0 {
        eprint!("\t");
        indent = indent.wrapping_sub(1);
    }
}

pub fn print_ast_indented(node: Option<Node>, indent: usize) {
    match node {
        Some(n) => match &*(*n).borrow() {
            NodeEnum::Num { value } => {
                eprint!("{}", value);
            }
            NodeEnum::Binary { op, left, right } => {
                eprint!("{}(", std::str::from_utf8(&[*op]).unwrap());
                print_ast_indented(Some(left.clone()), indent);
                eprint!(", ");
                print_ast_indented(Some(right.clone()), indent);
                eprint!(")");
            }
            NodeEnum::Var { name } => {
                eprint!("{}", std::str::from_utf8(&[*name]).unwrap());
            }
            NodeEnum::Sequence {
                statement_count,
                statements,
            } => {
                for i in 0..*statement_count {
                    print_ast_indented((**statements).borrow()[i].clone(), indent)
                }
            }
            NodeEnum::PrintNode { expr } => {
                eprint_indent(indent);
                eprint!("PRINT(");
                print_ast_indented(Some(expr.clone()), indent);
                eprintln!(")");
            }
            NodeEnum::LetNode { var, value } => {
                eprint_indent(indent);
                eprint!("LET({}, ", std::str::from_utf8(&[*var]).unwrap());
                print_ast_indented(Some(value.clone()), indent);
                eprintln!(")");
            }
            NodeEnum::IfNode {
                condition,
                if_branch,
                else_branch,
            } => {
                eprint_indent(indent);
                eprint!("IF(");
                print_ast_indented(Some(condition.clone()), indent);
                eprintln!(",");
                print_ast_indented(Some(if_branch.clone()), indent + 1);
                match else_branch {
                    Some(e_branch) => {
                        eprint_indent(indent);
                        eprintln!(",");
                        print_ast_indented(Some(e_branch.clone()), indent + 1);
                    }
                    None => {}
                }
                eprint_indent(indent);
                eprintln!(")");
            }
            NodeEnum::WhileNode { condition, body } => {
                eprint_indent(indent);
                eprint!("WHILE(");
                print_ast_indented(Some(condition.clone()), indent);
                eprintln!(",");
                print_ast_indented(Some(body.clone()), indent + 1);
                eprint_indent(indent);
                eprintln!(")");
            }
        },
        None => eprint!("\nUnknown node type: {:#?}\n", *node.borrow()),
    }
}

pub fn print_ast(node: Node) {
    print_ast_indented(Some(node), 0);
}
