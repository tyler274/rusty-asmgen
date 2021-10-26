use std::cell::RefCell;
use std::char;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum Node {
    num {
        value: i64,
    },
    binary {
        op: char,
        left: Rc<RefCell<Node>>,
        right: Rc<RefCell<Node>>,
    },
    var {
        name: char,
    },
    sequence {
        statement_count: usize,
        statements: Rc<RefCell<Vec<Rc<RefCell<Node>>>>>,
    },
    print_node {
        expr: Rc<RefCell<Node>>,
    },
    let_node {
        var: char,
        value: Rc<RefCell<Node>>,
    },
    if_node {
        condition: Rc<RefCell<Node>>,
        if_branch: Rc<RefCell<Node>>,
        else_branch: Rc<RefCell<Node>>,
    },
    while_node {
        condition: Rc<RefCell<Node>>,
        body: Rc<RefCell<Node>>,
    },
}

pub fn init_num_node(value: i64) -> Rc<RefCell<Node>> {
    Rc::new(RefCell::new(Node::num { value }))
}

pub fn init_binary_node(
    mut op: char,
    mut left: Option<Rc<RefCell<Node>>>,
    mut right: Option<Rc<RefCell<Node>>>,
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
            let mut node = Rc::new(RefCell::new(Node::binary {
                op,
                left: l,
                right: r,
            }));
            return Some(node);
        }
    }
}

pub fn init_var_node(name: char) -> Option<Rc<RefCell<Node>>> {
    if name == '\u{0}' {
        return None;
    }
    let mut node = Node::var { name };
    // if !node.is_null() {
    // } else {
    //     __assert_fail(
    //         b"node != NULL\x00" as *const u8 as *const libc::c_char,
    //         b"src/ast.c\x00" as *const u8 as *const libc::c_char,
    //         39 as libc::c_int as libc::c_uint,
    //         (*::std::mem::transmute::<&[u8; 34], &[libc::c_char; 34]>(
    //             b"node_t *init_var_node(var_name_t)\x00",
    //         ))
    //         .as_ptr(),
    //     );
    // }
    return Some(Rc::new(RefCell::new(node)));
}

pub fn init_sequence_node(
    mut statement_count: usize,
    mut statements: Rc<RefCell<Vec<Rc<RefCell<Node>>>>>,
) -> Option<Rc<RefCell<Node>>> {
    if statements.borrow().is_empty() && statement_count > 0 {
        return None;
    }
    let mut node = Rc::new(RefCell::new(Node::sequence {
        statement_count,
        statements,
    }));
    return Some(node);
}

pub fn init_print_node(expression: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Node>>> {
    match expression {
        Some(expr) => Some(Rc::new(RefCell::new(Node::print_node { expr }))),
        None => None,
    }
}

pub fn init_let_node(var: char, value: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Node>>> {
    if var == '\u{0}' {
        free_ast(value);
        return None;
    }
    let node = Rc::new(RefCell::new(Node::let_node {
        var,
        value: value.unwrap(),
    }));
    return Some(node);
}

pub fn init_if_node(
    mut condition: Option<Rc<RefCell<Node>>>,
    mut if_branch: Option<Rc<RefCell<Node>>>,
    mut else_branch: Option<Rc<RefCell<Node>>>,
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
            let mut node = Rc::new(RefCell::new(Node::if_node {
                condition: condition.unwrap(),
                if_branch: if_branch.unwrap(),
                else_branch: else_branch.unwrap(),
            }));
            return Some(node);
        }
    }
}

pub fn init_while_node(
    mut condition: Option<Rc<RefCell<Node>>>,
    mut body: Option<Rc<RefCell<Node>>>,
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
            let mut node = Rc::new(RefCell::new(Node::while_node {
                body: b,
                condition: c,
            }));
            return Some(node);
        }
    }
}

pub fn free_ast(mut node: Option<Rc<RefCell<Node>>>) {
    // let mut bor_reader = &*(*state.stream).borrow_mut();
    // let mut reader = bor_reader.take(1);
    match node {
        None => {
            return;
        }
        Some(n) => match &*(*n).borrow() {
            Node::num { value } => todo!(),
            Node::binary { op, left, right } => todo!(),
            Node::var { name } => todo!(),
            Node::sequence {
                statement_count,
                statements,
            } => todo!(),
            Node::print_node { expr } => todo!(),
            Node::let_node { var, value } => todo!(),
            Node::if_node {
                condition,
                if_branch,
                else_branch,
            } => todo!(),
            Node::while_node { condition, body } => todo!(),
        },
    }
    // if (*node).type_0 as libc::c_uint == BINARY_OP as libc::c_int as libc::c_uint {
    //     let mut bin: *mut binary_node_t = node as *mut binary_node_t;
    //     free_ast((*bin).left);
    //     free_ast((*bin).right);
    // } else if (*node).type_0 as libc::c_uint == SEQUENCE as libc::c_int as libc::c_uint {
    //     let mut sequence: *mut sequence_node_t = node as *mut sequence_node_t;
    //     let mut i: size_t = 0 as libc::c_int as size_t;
    //     while i < (*sequence).statement_count {
    //         free_ast(*(*sequence).statements.offset(i as isize));
    //         i = i.wrapping_add(1)
    //     }
    //     free((*sequence).statements as *mut libc::c_void);
    // } else if (*node).type_0 as libc::c_uint == PRINT as libc::c_int as libc::c_uint {
    //     free_ast((*(node as *mut print_node_t)).expr);
    // } else if (*node).type_0 as libc::c_uint == LET as libc::c_int as libc::c_uint {
    //     free_ast((*(node as *mut let_node_t)).value);
    // } else if (*node).type_0 as libc::c_uint == IF as libc::c_int as libc::c_uint {
    //     let mut conditional: *mut if_node_t = node as *mut if_node_t;
    //     free_ast((*conditional).condition as *mut node_t);
    //     free_ast((*conditional).if_branch);
    //     free_ast((*conditional).else_branch);
    // } else if (*node).type_0 as libc::c_uint == WHILE as libc::c_int as libc::c_uint {
    //     let mut loop_0: *mut while_node_t = node as *mut while_node_t;
    //     free_ast((*loop_0).condition as *mut node_t);
    //     free_ast((*loop_0).body);
    // }
    // free(node as *mut libc::c_void);
}

pub fn print_indent(mut indent: usize) {
    while indent > 0 {
        eprintln!("\t\x00");
        indent = indent.wrapping_sub(1)
    }
}

pub fn print_ast_indented(mut node: Rc<RefCell<Node>>, indent: usize) {
    // if (*node).type_0 as libc::c_uint == NUM as libc::c_int as libc::c_uint {
    //     fprintf(
    //         stderr,
    //         b"%ld\x00" as *const u8 as *const libc::c_char,
    //         (*(node as *mut num_node_t)).value,
    //     );
    // } else if (*node).type_0 as libc::c_uint == BINARY_OP as libc::c_int as libc::c_uint {
    //     let mut bin: *mut binary_node_t = node as *mut binary_node_t;
    //     fprintf(
    //         stderr,
    //         b"%c(\x00" as *const u8 as *const libc::c_char,
    //         (*bin).op as libc::c_int,
    //     );
    //     print_ast_indented((*bin).left, indent);
    //     fprintf(stderr, b", \x00" as *const u8 as *const libc::c_char);
    //     print_ast_indented((*bin).right, indent);
    //     fprintf(stderr, b")\x00" as *const u8 as *const libc::c_char);
    // } else if (*node).type_0 as libc::c_uint == VAR as libc::c_int as libc::c_uint {
    //     fprintf(
    //         stderr,
    //         b"%c\x00" as *const u8 as *const libc::c_char,
    //         (*(node as *mut var_node_t)).name as libc::c_int,
    //     );
    // } else if (*node).type_0 as libc::c_uint == SEQUENCE as libc::c_int as libc::c_uint {
    //     let mut sequence: *mut sequence_node_t = node as *mut sequence_node_t;
    //     let mut i: size_t = 0 as libc::c_int as size_t;
    //     while i < (*sequence).statement_count {
    //         print_ast_indented(*(*sequence).statements.offset(i as isize), indent);
    //         i = i.wrapping_add(1)
    //     }
    // } else if (*node).type_0 as libc::c_uint == PRINT as libc::c_int as libc::c_uint {
    //     print_indent(indent);
    //     fprintf(stderr, b"PRINT(\x00" as *const u8 as *const libc::c_char);
    //     print_ast_indented((*(node as *mut print_node_t)).expr, indent);
    //     fprintf(stderr, b")\n\x00" as *const u8 as *const libc::c_char);
    // } else if (*node).type_0 as libc::c_uint == LET as libc::c_int as libc::c_uint {
    //     print_indent(indent);
    //     let mut let_0: *mut let_node_t = node as *mut let_node_t;
    //     fprintf(
    //         stderr,
    //         b"LET(%c, \x00" as *const u8 as *const libc::c_char,
    //         (*let_0).var as libc::c_int,
    //     );
    //     print_ast_indented((*let_0).value, indent);
    //     fprintf(stderr, b")\n\x00" as *const u8 as *const libc::c_char);
    // } else if (*node).type_0 as libc::c_uint == IF as libc::c_int as libc::c_uint {
    //     let mut conditional: *mut if_node_t = node as *mut if_node_t;
    //     print_indent(indent);
    //     fprintf(stderr, b"IF(\x00" as *const u8 as *const libc::c_char);
    //     print_ast_indented((*conditional).condition as *mut node_t, indent);
    //     fprintf(stderr, b",\n\x00" as *const u8 as *const libc::c_char);
    //     print_ast_indented(
    //         (*conditional).if_branch,
    //         indent.wrapping_add(1 as libc::c_int as libc::c_ulong),
    //     );
    //     if !(*conditional).else_branch.is_null() {
    //         print_indent(indent);
    //         fprintf(stderr, b",\n\x00" as *const u8 as *const libc::c_char);
    //         print_ast_indented(
    //             (*conditional).else_branch,
    //             indent.wrapping_add(1 as libc::c_int as libc::c_ulong),
    //         );
    //     }
    //     print_indent(indent);
    //     fprintf(stderr, b")\n\x00" as *const u8 as *const libc::c_char);
    // } else if (*node).type_0 as libc::c_uint == WHILE as libc::c_int as libc::c_uint {
    //     let mut loop_0: *mut while_node_t = node as *mut while_node_t;
    //     print_indent(indent);
    //     fprintf(stderr, b"WHILE(\x00" as *const u8 as *const libc::c_char);
    //     print_ast_indented((*loop_0).condition as *mut node_t, indent);
    //     fprintf(stderr, b",\n\x00" as *const u8 as *const libc::c_char);
    //     print_ast_indented(
    //         (*loop_0).body,
    //         indent.wrapping_add(1 as libc::c_int as libc::c_ulong),
    //     );
    //     print_indent(indent);
    //     fprintf(stderr, b")\n\x00" as *const u8 as *const libc::c_char);
    // } else {
    //     fprintf(
    //         stderr,
    //         b"\nUnknown node type: %d\n\x00" as *const u8 as *const libc::c_char,
    //         (*node).type_0 as libc::c_uint,
    //     );
    //     __assert_fail(
    //         b"false\x00" as *const u8 as *const libc::c_char,
    //         b"src/ast.c\x00" as *const u8 as *const libc::c_char,
    //         221 as libc::c_int as libc::c_uint,
    //         (*::std::mem::transmute::<&[u8; 42], &[libc::c_char; 42]>(
    //             b"void print_ast_indented(node_t *, size_t)\x00",
    //         ))
    //         .as_ptr(),
    //     );
    // };
}

pub fn print_ast(node: Rc<RefCell<Node>>) {
    print_ast_indented(node, 0);
}
