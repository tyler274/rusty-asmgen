use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum Node {
    Num {
        value: i64,
    },
    Binary {
        op: u8,
        left: Rc<RefCell<Node>>,
        right: Rc<RefCell<Node>>,
    },
    Var {
        name: u8,
    },
    Sequence {
        statement_count: usize,
        statements: Rc<RefCell<Vec<Option<Rc<RefCell<Node>>>>>>,
    },
    PrintNode {
        expr: Rc<RefCell<Node>>,
    },
    LetNode {
        var: u8,
        value: Rc<RefCell<Node>>,
    },
    IfNode {
        condition: Rc<RefCell<Node>>,
        if_branch: Rc<RefCell<Node>>,
        else_branch: Option<Rc<RefCell<Node>>>,
    },
    WhileNode {
        condition: Rc<RefCell<Node>>,
        body: Rc<RefCell<Node>>,
    },
}

pub fn init_num_node(value: i64) -> Rc<RefCell<Node>> {
    Rc::new(RefCell::new(Node::Num { value }))
}

pub fn init_binary_node(
    op: u8,
    left: Option<Rc<RefCell<Node>>>,
    right: Option<Rc<RefCell<Node>>>,
) -> Option<Rc<RefCell<Node>>> {
    match (left.clone(), right.clone()) {
        (None, _) => {
            free_ast(left);
            free_ast(right);
            return None;
        }
        (_, None) => {
            free_ast(left);
            free_ast(right);
            return None;
        }
        (Some(l), Some(r)) => {
            let node = Rc::new(RefCell::new(Node::Binary {
                op,
                left: l,
                right: r,
            }));
            return Some(node);
        }
    }
}

pub fn init_var_node(name: u8) -> Option<Rc<RefCell<Node>>> {
    if name == b'\0' {
        return None;
    }
    let node = Node::Var { name };

    return Some(Rc::new(RefCell::new(node)));
}

pub fn init_sequence_node(
    statement_count: usize,
    statements: Rc<RefCell<Vec<Option<Rc<RefCell<Node>>>>>>,
) -> Option<Rc<RefCell<Node>>> {
    if (*statements).borrow().is_empty() && statement_count > 0 {
        return None;
    }
    let node = Rc::new(RefCell::new(Node::Sequence {
        statement_count,
        statements,
    }));
    return Some(node);
}

pub fn init_print_node(expression: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Node>>> {
    match expression {
        Some(expr) => Some(Rc::new(RefCell::new(Node::PrintNode { expr }))),
        None => None,
    }
}

pub fn init_let_node(var: u8, value: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Node>>> {
    if var == b'\0' {
        free_ast(value);
        return None;
    }
    let node = Rc::new(RefCell::new(Node::LetNode {
        var,
        value: value.expect(&format!("Error initing LET node with var : {}", var)),
    }));
    return Some(node);
}

pub fn init_if_node(
    condition: Option<Rc<RefCell<Node>>>,
    if_branch: Option<Rc<RefCell<Node>>>,
    else_branch: Option<Rc<RefCell<Node>>>,
) -> Option<Rc<RefCell<Node>>> {
    match (condition.clone(), if_branch.clone()) {
        (None, _) => {
            free_ast(condition);
            free_ast(if_branch);
            free_ast(else_branch);
            return None;
        }
        (_, None) => {
            free_ast(condition);
            free_ast(if_branch);
            free_ast(else_branch);
            return None;
        }
        (Some(_), Some(_)) => {
            let node = Rc::new(RefCell::new(Node::IfNode {
                condition: condition.unwrap(),
                if_branch: if_branch.unwrap(),
                else_branch: else_branch,
            }));
            return Some(node);
        }
    }
}

pub fn init_while_node(
    condition: Option<Rc<RefCell<Node>>>,
    body: Option<Rc<RefCell<Node>>>,
) -> Option<Rc<RefCell<Node>>> {
    match (condition.clone(), body.clone()) {
        (None, _) => {
            free_ast(condition);
            free_ast(body);
            return None;
        }
        (_, None) => {
            free_ast(condition);
            free_ast(body);
            return None;
        }
        (Some(b), Some(c)) => {
            let node = Rc::new(RefCell::new(Node::WhileNode {
                body: b,
                condition: c,
            }));
            return Some(node);
        }
    }
}

// I'm think this is taken care of by RAII and Drop.
pub fn free_ast(node: Option<Rc<RefCell<Node>>>) {
    match node {
        None => {
            return;
        }
        // oh holy pointer hallowed be thy name I summon thee.
        // Not really but the (de)referencing indirection here is amusing.
        Some(n) => {
            match &*(*n).borrow_mut() {
                Node::Num { value } => drop(value),
                Node::Binary { op, left, right } => {
                    free_ast(Some(left.clone()));
                    free_ast(Some(right.clone()));
                    drop(op);
                }
                Node::Var { name } => drop(*name),
                Node::Sequence {
                    statement_count,
                    statements,
                } => {
                    for i in 0..*statement_count {
                        free_ast((*statements).borrow_mut()[i].clone());
                    }
                    drop(statements);
                }
                Node::PrintNode { expr } => free_ast(Some(expr.clone())),
                Node::LetNode { var, value } => {
                    free_ast(Some(value.clone()));
                    drop(var);
                }
                Node::IfNode {
                    condition,
                    if_branch,
                    else_branch,
                } => {
                    free_ast(Some(condition.clone()));
                    free_ast(Some(if_branch.clone()));
                    free_ast(else_branch.clone());
                }
                Node::WhileNode { condition, body } => {
                    free_ast(Some(condition.clone()));
                    free_ast(Some(body.clone()));
                }
            }
            drop(n);
        }
    }
}

pub fn print_indent(mut indent: usize) {
    while indent > 0 {
        eprint!("\t");
        indent = indent.wrapping_sub(1)
    }
}

pub fn print_ast_indented(node: Option<Rc<RefCell<Node>>>, indent: usize) {
    match node {
        Some(n) => match &*(*n.clone()).borrow() {
            Node::Num { value } => {
                eprint!("{}", value);
            }
            Node::Binary { op, left, right } => {
                eprint!("{}(", op);
                print_ast_indented(Some(left.clone()), indent);
                eprint!(", ");
                print_ast_indented(Some(right.clone()), indent);
                eprint!(")");
            }
            Node::Var { name } => {
                eprint!("{}", name);
            }
            Node::Sequence {
                statement_count,
                statements,
            } => {
                for i in 0..*statement_count {
                    print_ast_indented((**statements).borrow()[i].clone(), indent)
                }
            }
            Node::PrintNode { expr } => {
                print_indent(indent);
                eprint!("PRINT(");
                print_ast_indented(Some(expr.clone()), indent);
                eprint!(")\n");
            }
            Node::LetNode { var, value } => {
                print_indent(indent);
                eprint!("LET({}, ", var);
                print_ast_indented(Some(value.clone()), indent);
                eprint!(")\n");
            }
            Node::IfNode {
                condition,
                if_branch,
                else_branch,
            } => {
                print_indent(indent);
                eprint!("IF(");
                print_ast_indented(Some(condition.clone()), indent);
                eprint!(",\n");
                print_ast_indented(Some(if_branch.clone()), indent + 1);
                match else_branch {
                    Some(e_branch) => {
                        print_indent(indent);
                        eprint!(",\n");
                        print_ast_indented(Some(e_branch.clone()), indent);
                    }
                    None => todo!(),
                }
                print_indent(indent);
                eprint!(")\n");
            }
            Node::WhileNode { condition, body } => {
                print_indent(indent);
                eprint!("WHILE(");
                print_ast_indented(Some(condition.clone()), indent);
                eprint!(",\n");
                print_ast_indented(Some(body.clone()), indent + 1);
                print_indent(indent);
                eprint!(")\n");
            }
        },
        None => eprint!("\nUnknown node type: {:#?}\n", *node.borrow()),
    }
}

pub fn print_ast(node: Rc<RefCell<Node>>) {
    print_ast_indented(Some(node), 0);
}
