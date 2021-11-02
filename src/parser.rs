use std::borrow::Borrow;

use std::fs::File;
use std::io::{prelude::*, SeekFrom};
use std::ops::Deref;
use std::{cell::RefCell, rc::Rc};

use crate::ast::{
    free_ast, init_binary_node, init_if_node, init_let_node, init_num_node, init_print_node,
    init_sequence_node, init_var_node, init_while_node, Node,
};

#[derive(Debug, Clone)]
pub struct ParserState {
    pub stream: Rc<RefCell<File>>,
}

pub type CharPredicate = Option<fn(_: u8) -> bool>;

pub static MAX_KEYWORD_LENGTH: usize = 100;
pub static DEFAULT_STEP: i64 = 1;

pub fn is_variable_name(c: u8) -> bool {
    return c.is_ascii_uppercase();
}

pub fn is_open_paren(c: u8) -> bool {
    return c == b'(';
}

pub fn is_close_paren(c: u8) -> bool {
    return c == b')';
}
pub fn is_factor_op(c: u8) -> bool {
    return c == b'*' || c == b'/';
}

pub fn is_term_op(c: u8) -> bool {
    return c == b'+' || c == b'-';
}

pub fn is_comparison_op(c: u8) -> bool {
    return c == b'<' || c == b'=' || c == b'>';
}

pub fn is_operator(c: u8) -> bool {
    return is_open_paren(c)
        || is_close_paren(c)
        || is_factor_op(c)
        || is_term_op(c)
        || is_comparison_op(c);
}

pub fn is_comment_start(c: u8) -> bool {
    return c == b'#';
}

pub fn save_position(state: &ParserState) -> u64 {
    let mut bor_reader = &*(*state.stream).borrow();
    bor_reader.seek(SeekFrom::Current(0)).unwrap()
}

pub fn restore_position(state: &ParserState, position: u64) {
    let mut bor_reader = &*(*state.stream).borrow();
    bor_reader.seek(SeekFrom::Start(position)).unwrap();
    // fseek((*state).stream, position as libc::c_long, 0 as libc::c_int);
}

pub fn rewind_one(state: &ParserState) {
    let mut bor_reader = &*(*state.stream).borrow();
    bor_reader.seek(SeekFrom::Current(-1)).unwrap();
}
/*
 * Advances the provided state to the next token.
 */

pub fn advance(state: &ParserState) -> u8 {
    // let bor_reader = &*(*state.stream).borrow();
    let bor_reader = &*(state.stream.deref()).borrow();
    loop {
        let mut reader = bor_reader.take(1);
        let mut buf = [0; 1];
        let result = reader.read(&mut buf).unwrap();
        if result == 0 {
            return b'\0';
        }
        if !(buf[0] as u8).is_ascii_whitespace() {
            return buf[0] as u8;
        }
    }
}

pub fn try_advance(state: &ParserState, predicate: CharPredicate) -> u8 {
    let next: u8 = advance(state);
    if next != b'\0' && !predicate.expect("null function pointer")(next) {
        rewind_one(state);
        return b'\0';
    }
    return next;
}

pub fn advance_until_separator(state: &ParserState) -> Option<Vec<u8>> {
    let mut result = Vec::with_capacity(MAX_KEYWORD_LENGTH + 1);
    assert!(result.capacity() == MAX_KEYWORD_LENGTH + 1);

    // let mut index: usize = 0;

    let bor_reader = &*(*state.stream).borrow();

    'outer: loop {
        let mut reader = bor_reader.take(1);
        if result.len() > MAX_KEYWORD_LENGTH {
            drop(result);
            return None;
        }

        let mut buf = [0; 1];
        let read_result = reader.read(&mut buf).unwrap();

        if read_result == 0 {
            if result.len() > 0 {
                break 'outer;
            }
            drop(result);
            return None;
        }
        // eprintln!("127 result size: {}", result.len());

        if is_operator(buf[0] as u8) && result.len() > 0 {
            rewind_one(state);
            break 'outer;
        }
        // eprintln!("133 result size: {}", result.len());
        if (buf[0] as u8).is_ascii_whitespace() {
            if result.len() > 0 {
                break 'outer;
            }

            continue 'outer;
        }
        // let fresh0 = index;
        // index = index.wrapping_add(1);
        // eprintln!("143 result size: {}, index: {}", result.len(), index);
        // result[fresh0] = buf[0];
        result.push(buf[0])
    }

    // result[index] = b'\0' as u8;
    result.push(b'\0');
    return Some(result);
}

pub fn at_end(state: &ParserState) -> bool {
    if advance(state) != b'\0' {
        rewind_one(state);
        return false;
    }
    return true;
}

pub fn skip_line(state: &ParserState) {
    let bor_reader = &*(*state.stream).borrow();
    // let mut reader = bor_reader.take(1);
    loop {
        let mut reader = bor_reader.take(1);
        let mut buf = [0; 1];
        let result = reader.read(&mut buf).expect("Error skipping line");
        if result == 0 || buf[0] == b'\n' {
            break;
        }
    }
}

pub fn num(state: &ParserState) -> Option<Rc<RefCell<Node>>> {
    let num_string = advance_until_separator(state);
    match num_string {
        Some(n_string) => {
            let parse_result = std::str::from_utf8(n_string.as_slice());
            match parse_result {
                Ok(value) => {
                    // the god damn null terminator really got me here
                    let val = &value[0..value.len() - 1];
                    let num: i64;
                    if val.starts_with("0x") {
                        num = i64::from_str_radix(val.trim_start_matches("0x"), 16).expect(
                        &format!("parsing into an i64 failed, check for terminator and whitespace handling: {:#?}", val),
                    );
                    } else {
                        num = i64::from_str_radix(val, 10).expect(
                        &format!("parsing into an i64 failed, check for terminator and whitespace handling: {:#?}", val),
                    );
                    }
                    return Some(init_num_node(num));
                }
                Err(_) => {
                    // drop(num_string);
                    return None;
                }
            }
        }
        None => None,
    }
}

pub fn factor(state: &ParserState) -> Option<Rc<RefCell<Node>>> {
    if try_advance(state, Some(is_open_paren as fn(_: u8) -> bool)) != b'\0' {
        let node = expression(state);
        if try_advance(state, Some(is_close_paren as fn(_: u8) -> bool)) == b'\0' {
            return None;
        }
        return node;
    }
    let var: u8 = try_advance(state, Some(is_variable_name as fn(_: u8) -> bool));
    if var != b'\0' {
        return init_var_node(var);
    }
    return num(state);
}

pub fn term(state: &ParserState) -> Option<Rc<RefCell<Node>>> {
    let mut result = factor(state);
    loop {
        let next = try_advance(state, Some(is_factor_op as fn(_: u8) -> bool));
        if next == b'\0' {
            break;
        }
        result = init_binary_node(next, result, factor(state));
    }
    return result;
}

pub fn expression(state: &ParserState) -> Option<Rc<RefCell<Node>>> {
    let mut result = term(state);
    loop {
        let next: u8 = try_advance(state, Some(is_term_op as fn(_: u8) -> bool));
        if next == b'\0' {
            break;
        }
        result = init_binary_node(next, result, term(state));
    }
    return result;
}

pub fn comparison(state: &ParserState) -> Option<Rc<RefCell<Node>>> {
    let left = expression(state);
    let op: u8 = try_advance(state, Some(is_comparison_op as fn(_: u8) -> bool));
    return init_binary_node(op, left, expression(state));
}

pub fn statement(state: &ParserState, end: &mut bool) -> Option<Rc<RefCell<Node>>> {
    while try_advance(state, Some(is_comment_start as fn(_: u8) -> bool)) != b'\0' {
        skip_line(state);
    }
    let start: u64 = save_position(state);
    let mut next = advance_until_separator(state);
    match next.clone() {
        Some(next_token) => match next_token.as_slice() {
            b"ELSE\x00" => {
                drop(next);
                restore_position(state, start);
                *end = true;
                return None;
            }
            b"END\x00" => {
                drop(next);
                next = advance_until_separator(state);
                match next.clone() {
                    Some(next_token) => {
                        if !(next_token == b"IF\x00" || next_token == b"WHILE\x00") {
                            drop(next_token);
                            *end = false;
                            eprint!("IF or WHILE token, returning None\n");
                            return None;
                        }
                    }
                    None => {
                        drop(next);
                        *end = false;
                        eprint!("no IF or WHILE token, returning None\n");
                        return None;
                    }
                }

                drop(next);
                restore_position(state, start);
                *end = true;
                return None;
            }
            b"PRINT\x00" => {
                *end = false;
                drop(next);
                return init_print_node(expression(state));
            }
            b"LET\x00" => {
                *end = false;
                drop(next);
                let var = advance(state);
                if !(is_variable_name(var) && advance(state) == b'=') {
                    return None;
                }
                return init_let_node(var, expression(state));
            }
            b"IF\x00" => {
                *end = false;
                drop(next);
                let condition = comparison(state);
                let if_branch = sequence(state);
                next = advance_until_separator(state);
                let else_branch: Option<Rc<RefCell<Node>>>;
                match next.clone() {
                    Some(next_token) => match next_token.as_slice() {
                        b"ELSE\x00" => {
                            drop(next);
                            else_branch = sequence(state);
                            next = advance_until_separator(state);
                        }
                        _ => {
                            else_branch = None;
                        }
                    },
                    None => {
                        else_branch = None;
                    }
                }

                match next.clone() {
                    Some(next_token) => match next_token.as_slice() {
                        b"END\x00" => {}
                        _ => {
                            drop(next);
                            free_ast(condition);
                            free_ast(if_branch);
                            free_ast(else_branch);
                            return None;
                        }
                    },
                    None => {
                        drop(next);
                        free_ast(condition);
                        free_ast(if_branch);
                        free_ast(else_branch);
                        return None;
                    }
                }

                drop(next);
                next = advance_until_separator(state);
                match next.clone() {
                    Some(next_token) => match next_token.as_slice() {
                        b"IF\x00" => {}
                        _ => {
                            drop(next);
                            free_ast(condition);
                            free_ast(if_branch);
                            free_ast(else_branch);
                            return None;
                        }
                    },
                    None => {
                        drop(next);
                        free_ast(condition);
                        free_ast(if_branch);
                        free_ast(else_branch);
                        return None;
                    }
                }

                drop(next);
                return init_if_node(condition, if_branch, else_branch);
            }
            b"WHILE\x00" => {
                drop(next);
                let condition_0 = comparison(state);
                let body = sequence(state.borrow());
                next = advance_until_separator(state);
                match next.clone() {
                    Some(next_token) => match next_token.as_slice() {
                        b"END\x00" => {}
                        _ => {
                            drop(next);
                            free_ast(condition_0);
                            free_ast(body);
                            return None;
                        }
                    },
                    None => {
                        drop(next);
                        free_ast(condition_0);
                        free_ast(body);
                        return None;
                    }
                }

                drop(next);
                next = advance_until_separator(state);
                match next.clone() {
                    Some(next_token) => match next_token.as_slice() {
                        b"WHILE\x00" => {}
                        _ => {
                            drop(next);
                            free_ast(condition_0.clone());
                            free_ast(body.clone());
                            return None;
                        }
                    },
                    None => {
                        drop(next);
                        free_ast(condition_0.clone());
                        free_ast(body.clone());
                        return None;
                    }
                }

                drop(next);
                return init_while_node(condition_0, body);
            }
            _ => {}
        },
        None => {
            *end = true;
            return None;
        }
    }

    drop(next);
    return None;
}

pub fn sequence(state: &ParserState) -> Option<Rc<RefCell<Node>>> {
    let mut statement_count: usize = 0;
    let statements: Rc<RefCell<Vec<Option<Rc<RefCell<Node>>>>>> = Rc::new(RefCell::new(Vec::new()));
    let mut statement_capacity: usize = 0;
    loop {
        let mut end: bool = false;
        let next = statement(state, &mut end);

        // eprint!("Statments in this loop: \n");
        // for i in 0..statement_count {
        //     eprint!(
        //         "    statement: {:#?}\n",
        //         statements
        //             .borrow_mut()
        //             .get(i)
        //             .as_ref()
        //             .unwrap()
        //             .as_ref()
        //             .unwrap()
        //             .deref()
        //             .borrow()
        //     );
        // }

        if end {
            break;
        }
        match next {
            Some(node) => {
                if statement_count == statement_capacity {
                    statement_capacity = if statement_capacity > 0 {
                        statement_capacity * 2
                    } else {
                        1
                    };
                    statements.borrow_mut().resize(statement_capacity, None)
                }
                (*statements.borrow_mut())[statement_count] = Some(node.clone());
                assert_eq!(
                    statements
                        .deref()
                        .borrow()
                        .get(statement_count)
                        .as_ref()
                        .unwrap()
                        .as_ref()
                        .unwrap()
                        .as_ref()
                        .borrow()
                        .deref(),
                    node.deref().borrow().deref()
                );
                statement_count += 1;
            }
            None => {
                for i in 0..statement_count {
                    // eprint!(
                    //     "\n statement: {:#?}\n",
                    //     statements
                    //         .borrow_mut()
                    //         .get(i)
                    //         .as_ref()
                    //         .unwrap()
                    //         .as_ref()
                    //         .unwrap()
                    //         .deref()
                    //         .borrow()
                    // );
                    free_ast(statements.borrow_mut()[i].clone());
                }
                eprint!("\n oh no Sequence Node returned None\n");
                drop(statements);
                return None;
            }
        }
    }
    // Avoid allocating a sequence_node_t if there is only one statement
    if statement_count == 1 {
        return Some(
            statements
                .deref()
                .borrow()
                .deref()
                .deref()
                .get(0)
                .expect("Failed to return a single statement")
                .as_ref()
                .unwrap()
                .clone(),
        );
        // return Some(
        //     statements.deref().borrow().deref().deref()[0]
        //         .as_ref()
        //         .unwrap()
        //         .clone(),
        // );
        // This is a sin upon god and anime, the parenthesis and derefs were needed to
        // make the type checker play nice.
        // return Some((*((*((*statements).borrow()))[0].as_ref().unwrap())).clone());
    }
    if statement_count > 0 {
        statements.borrow_mut().resize(statement_count, None);
    }
    return init_sequence_node(statement_count, statements);
}

pub fn parse(stream: File) -> Option<Rc<RefCell<Node>>> {
    // eprintln!("{} stream length", stream.metadata().unwrap().len());
    let state = ParserState {
        stream: Rc::new(RefCell::new(stream)),
    };

    let ast = sequence(state.borrow());
    if !at_end(&state) {
        free_ast(ast);
        eprint!("\n oh no AST didn't finish processing\n");
        return None;
    }
    return ast;
}
