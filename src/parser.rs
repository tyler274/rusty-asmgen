use std::borrow::{Borrow, BorrowMut};
use std::convert::TryInto;
use std::error::Error;
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

pub type CharPredicate = Option<fn(_: char) -> bool>;

pub static MAX_KEYWORD_LENGTH: usize = 100;
pub static DEFAULT_STEP: i64 = 1;

pub fn is_variable_name(c: char) -> bool {
    return c.is_uppercase();
}

pub fn is_open_paren(c: char) -> bool {
    return c == '(';
}

pub fn is_close_paren(c: char) -> bool {
    return c == ')';
}
pub fn is_factor_op(c: char) -> bool {
    return c == '*' || c == '/';
}

pub fn is_term_op(c: char) -> bool {
    return c == '+' || c == '-';
}

pub fn is_comparison_op(c: char) -> bool {
    return c == '<' || c == '=' || c == '>';
}

pub fn is_operator(c: char) -> bool {
    return is_open_paren(c)
        || is_close_paren(c)
        || is_factor_op(c)
        || is_term_op(c)
        || is_comparison_op(c);
}

pub fn is_comment_start(c: char) -> bool {
    return c == '#';
}

pub fn save_position(state: &ParserState) -> u64 {
    let mut bor_reader = &*(*state.stream).borrow_mut();
    bor_reader.seek(SeekFrom::Current(0)).unwrap()
}

pub fn restore_position(state: &ParserState, position: u64) {
    let mut bor_reader = &*(*state.stream).borrow_mut();
    bor_reader.seek(SeekFrom::Start(position)).unwrap();
    // fseek((*state).stream, position as libc::c_long, 0 as libc::c_int);
}

pub fn rewind_one(state: &ParserState) {
    let mut bor_reader = &*(*state.stream).borrow_mut();
    bor_reader.seek(SeekFrom::Current(-1)).unwrap();
}
/*
 * Advances the provided state to the next token.
 */

pub fn advance(state: &ParserState) -> char {
    let bor_reader = &*(*state.stream).borrow_mut();
    let mut reader = bor_reader.take(1);

    loop {
        let mut buf = [0; 10];
        let result = reader.read(&mut buf).unwrap();
        if result == 0 {
            return '\u{0}';
        }
        if !(buf[0] as char).is_whitespace() {
            return buf[0] as char;
        }
    }
}

pub fn try_advance(state: &ParserState, predicate: CharPredicate) -> char {
    let next: char = advance(state);
    if next != '\u{0}' && !predicate.expect("non-null function pointer")(next) {
        rewind_one(state);
        return '\u{0}';
    }
    return next;
}

pub fn advance_until_separator(state: &ParserState) -> Option<Vec<u8>> {
    let mut result = Vec::with_capacity(MAX_KEYWORD_LENGTH + 1);
    assert!(result.capacity() == MAX_KEYWORD_LENGTH + 1);

    let mut index: usize = 0;

    let mut bor_reader = &*(*state.stream).borrow_mut();
    let mut reader = bor_reader.take(1);
    loop {
        if index > MAX_KEYWORD_LENGTH {
            drop(result);
            return None;
        }
        let mut buf = [0; 10];
        let read_result = reader.read(&mut buf).unwrap();
        if read_result == 0 {
            if index > 0 {
                break;
            }
            drop(result);
            return None;
        } else if is_operator(buf[0] as char) && index > 0 {
            rewind_one(state);
            break;
        } else if (buf[0] as char).is_whitespace() {
            if index > 0 {
                break;
            }
        } else {
            let fresh0 = index;
            index = index.wrapping_add(1);
            result[fresh0] = buf[0];
        }
    }
    // result.as_bytes()[index] = '\u{0}' as u8;
    return Some(result);
}

pub fn at_end(state: &ParserState) -> bool {
    if advance(state) != '\u{0}' {
        rewind_one(state);
        return false;
    }
    return true;
}

pub fn skip_line(state: &ParserState) {
    // Rc::clone(&state.stream);
    let bor_reader = &*(*state.stream).borrow_mut();
    let mut reader = bor_reader.take(1);

    // .take(1);
    loop {
        let mut buf = [0; 10];
        let result = reader.read(&mut buf).unwrap();
        if result == 0 || (buf[0] as char) == '\n' {
            break;
        }
    }
}

pub fn num(state: &ParserState) -> Option<Rc<RefCell<Node>>> {
    let num_string = advance_until_separator(state);
    match num_string {
        Some(n_string) => {
            let parse_result = n_string.as_slice().try_into();
            match parse_result {
                Ok(value) => Some(init_num_node(i64::from_ne_bytes(value))),
                Err(_) => {
                    // drop(num_string);
                    None
                }
            }
        }
        None => None,
    }
}

pub fn factor(state: &ParserState) -> Option<Rc<RefCell<Node>>> {
    if try_advance(state, Some(is_open_paren as fn(_: char) -> bool)) != '\0' {
        let node = expression(state);
        if try_advance(state, Some(is_close_paren as fn(_: char) -> bool)) == '\0' {
            return None;
        }
        return node;
    }
    let var: char = try_advance(state, Some(is_variable_name as fn(_: char) -> bool));
    if var != '\0' {
        return init_var_node(var);
    }
    return num(state);
}

pub fn term(state: &ParserState) -> Option<Rc<RefCell<Node>>> {
    let mut result = factor(state);
    loop {
        let next = try_advance(state, Some(is_factor_op as fn(_: char) -> bool));
        if next == '\0' {
            break;
        }
        result = init_binary_node(next, result, factor(state));
    }
    return result;
}

pub fn expression(state: &ParserState) -> Option<Rc<RefCell<Node>>> {
    let mut result = term(state);
    loop {
        let next: char = try_advance(state, Some(is_term_op as fn(_: char) -> bool));
        if next == '\0' {
            break;
        }
        result = init_binary_node(next, result, term(state));
    }
    return result;
}

pub fn comparison(state: &ParserState) -> Option<Rc<RefCell<Node>>> {
    let left = expression(state);
    let op: char = try_advance(state, Some(is_comparison_op as fn(_: char) -> bool));
    return init_binary_node(op, left, expression(state));
}

pub fn statement(state: &ParserState, end: &mut bool) -> Option<Rc<RefCell<Node>>> {
    while try_advance(state, Some(is_comment_start as fn(_: char) -> bool)) != '\0' {
        skip_line(state);
    }
    let start: u64 = save_position(state);
    let mut next: Vec<u8> = advance_until_separator(state).unwrap();
    if next.is_empty() {
        *end = true;
        return None;
    }
    if next == b"ELSE\x00" {
        drop(next);
        restore_position(state, start);
        *end = false;
        return None;
    }
    if next == b"END\x00" {
        drop(next);
        next = advance_until_separator(state).unwrap();
        if next.is_empty() || !(next == b"IF\x00") || next == b"WHILE\x00" {
            drop(next);
            *end = false;
            return None;
        }
        drop(next);
        restore_position(state, start);
        *end = true;
        return None;
    }
    *end = false;
    if next == b"PRINT\x00" {
        drop(next);
        return init_print_node(expression(state));
    }
    if next == b"LET\x00" {
        drop(next);
        let var = advance(state);
        if !(is_variable_name(var) && advance(state) == '=') {
            return None;
        }
        return init_let_node(var, expression(state));
    }
    if next == b"IF\x00" {
        drop(next);
        let condition = comparison(state);
        let if_branch = sequence(&state);
        next = advance_until_separator(state).unwrap();
        let mut else_branch: Option<Rc<RefCell<Node>>> = None;
        if !next.is_empty() && next == b"ELSE\x00" {
            drop(next);
            else_branch = sequence(&state);
            next = advance_until_separator(state).unwrap();
        } else {
            else_branch = None
        }
        if next.is_empty() || next != b"END\x00" {
            drop(next);
            free_ast(condition);
            free_ast(if_branch);
            free_ast(else_branch);
            return None;
        }
        drop(next);
        next = advance_until_separator(state).unwrap();
        if next.is_empty() || next != b"IF\x00" {
            drop(next);
            free_ast(condition);
            free_ast(if_branch);
            free_ast(else_branch);
            return None;
        }
        drop(next);
        return init_if_node(condition, if_branch, else_branch);
    }
    if next == b"WHILE\x00" {
        drop(next);
        let condition_0 = comparison(state);
        let body = sequence(state.borrow());
        next = advance_until_separator(state).unwrap();
        if next.is_empty() || next != b"END\x00" {
            drop(next);
            free_ast(condition_0);
            free_ast(body);
            return None;
        }
        drop(next);
        next = advance_until_separator(state).unwrap();
        if next.is_empty() || next != b"WHILE\x00" {
            drop(next);
            free_ast(condition_0);
            free_ast(body);
            return None;
        }
        drop(next);
        return init_while_node(condition_0, body);
    }
    drop(next);
    return None;
}

pub fn sequence(mut state: &ParserState) -> Option<Rc<RefCell<Node>>> {
    let mut statement_count: usize = 0;
    let mut statements: Rc<RefCell<Vec<Rc<RefCell<Node>>>>> = Rc::new(RefCell::new(Vec::new()));
    let mut statement_capacity: usize = 0;
    loop {
        // let mut end: bool = false;
        // let mut next: *mut node_t = statement(state, &mut end);
        // if end {
        //     break;
        // }
        // if next.is_null() {
        //     let mut i: size_t = 0 as libc::c_int as size_t;
        //     while i < statement_count {
        //         free_ast(*statements.offset(i as isize));
        //         i = i.wrapping_add(1)
        //     }
        //     free(statements as *mut libc::c_void);
        //     return 0 as *mut node_t;
        // }
        // if statement_count == statement_capacity {
        //     statement_capacity = if statement_capacity > 0 as libc::c_int as libc::c_ulong {
        //         statement_capacity.wrapping_mul(2 as libc::c_int as libc::c_ulong)
        //     } else {
        //         1 as libc::c_int as libc::c_ulong
        //     };
        //     statements = realloc(
        //         statements as *mut libc::c_void,
        //         (::std::mem::size_of::<*mut node_t>() * statement_capacity as usize)
        //             as libc::c_ulong,
        //     ) as *mut *mut node_t;
        //     if !statements.is_null() {
        //     } else {
        //         __assert_fail(
        //             b"statements != NULL\x00" as *const u8 as *const libc::c_char,
        //             b"src/parser.c\x00" as *const u8 as *const libc::c_char,
        //             344 as libc::c_int as libc::c_uint,
        //             (*::std::mem::transmute::<&[u8; 35], &[libc::c_char; 35]>(
        //                 b"node_t *sequence(parser_state_t *)\x00",
        //             ))
        //             .as_ptr(),
        //         );
        //     }
        // }
        // let fresh1 = statement_count;
        // statement_count = statement_count.wrapping_add(1);
        // let ref mut fresh2 = *statements.offset(fresh1 as isize);
        // *fresh2 = next
    }
    // Avoid allocating a sequence_node_t if there is only one statement
    // if statement_count == 1 {
    //     let mut statement_0: *mut node_t = *statements;
    //     // free(statements as *mut libc::c_void);
    //     return statement_0;
    // }
    if statement_count > 0 {
        // statements = realloc(
        //     statements as *mut libc::c_void,
        //     (::std::mem::size_of::<*mut node_t>() * statement_count as usize) as libc::c_ulong,
        // ) as *mut *mut node_t;
        // if !statements.is_null() {
        // } else {
        //     __assert_fail(
        //         b"statements != NULL\x00" as *const u8 as *const libc::c_char,
        //         b"src/parser.c\x00" as *const u8 as *const libc::c_char,
        //         358 as libc::c_int as libc::c_uint,
        //         (*::std::mem::transmute::<&[u8; 35], &[libc::c_char; 35]>(
        //             b"node_t *sequence(parser_state_t *)\x00",
        //         ))
        //         .as_ptr(),
        //     );
        // }
    }
    return init_sequence_node(statement_count, statements);
}

pub fn parse(stream: File) -> Option<Rc<RefCell<Node>>> {
    let state = ParserState {
        stream: Rc::new(RefCell::new(stream)),
    };
    let ast = sequence(state.borrow());
    if !at_end(&state) {
        free_ast(ast);
        return None;
    }
    return ast;
}
