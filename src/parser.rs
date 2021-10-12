use libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;

    fn fgetc(__stream: *mut FILE) -> libc::c_int;

    fn fseek(__stream: *mut FILE, __off: libc::c_long, __whence: libc::c_int) -> libc::c_int;

    fn ftell(__stream: *mut FILE) -> libc::c_long;

    fn init_num_node(value: value_t) -> *mut node_t;

    fn init_binary_node(op: libc::c_char, left: *mut node_t, right: *mut node_t) -> *mut node_t;

    fn init_var_node(name: var_name_t) -> *mut node_t;

    fn init_sequence_node(statement_count: size_t, statements: *mut *mut node_t) -> *mut node_t;

    fn init_print_node(expr: *mut node_t) -> *mut node_t;

    fn init_let_node(var: var_name_t, value: *mut node_t) -> *mut node_t;

    fn init_if_node(
        condition: *mut binary_node_t,
        if_branch: *mut node_t,
        else_branch: *mut node_t,
    ) -> *mut node_t;

    fn init_while_node(condition: *mut binary_node_t, body: *mut node_t) -> *mut node_t;

    fn free_ast(node: *mut node_t);

    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;

    fn __ctype_b_loc() -> *mut *const libc::c_ushort;

    fn __errno_location() -> *mut libc::c_int;

    fn strtol(_: *const libc::c_char, _: *mut *mut libc::c_char, _: libc::c_int) -> libc::c_long;

    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;

    fn free(__ptr: *mut libc::c_void);

    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;

    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __int64_t = libc::c_long;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type int64_t = __int64_t;
pub type node_type_t = libc::c_uint;
pub const WHILE: node_type_t = 7;
pub const IF: node_type_t = 6;
pub const LET: node_type_t = 5;
pub const PRINT: node_type_t = 4;
pub const SEQUENCE: node_type_t = 3;
pub const VAR: node_type_t = 2;
pub const BINARY_OP: node_type_t = 1;
pub const NUM: node_type_t = 0;
pub type var_name_t = libc::c_char;
pub type value_t = int64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct node_t {
    pub type_0: node_type_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct binary_node_t {
    pub base: node_t,
    pub op: libc::c_char,
    pub left: *mut node_t,
    pub right: *mut node_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct parser_state_t {
    pub stream: *mut FILE,
}
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISupper: C2RustUnnamed = 256;
pub type char_predicate_t = Option<unsafe extern "C" fn(_: libc::c_char) -> bool>;
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
// maxint = -9223372036854775808
#[no_mangle]
pub static mut MAX_KEYWORD_LENGTH: size_t = 100 as libc::c_int as size_t;
#[no_mangle]
pub static mut DEFAULT_STEP: value_t = 1 as libc::c_int as value_t;
#[no_mangle]
pub unsafe extern "C" fn is_variable_name(mut c: libc::c_char) -> bool {
    return *(*__ctype_b_loc()).offset(c as libc::c_int as isize) as libc::c_int
        & _ISupper as libc::c_int as libc::c_ushort as libc::c_int
        != 0;
}
#[no_mangle]
pub unsafe extern "C" fn is_open_paren(mut c: libc::c_char) -> bool {
    return c as libc::c_int == '(' as i32;
}
#[no_mangle]
pub unsafe extern "C" fn is_close_paren(mut c: libc::c_char) -> bool {
    return c as libc::c_int == ')' as i32;
}
#[no_mangle]
pub unsafe extern "C" fn is_factor_op(mut c: libc::c_char) -> bool {
    return c as libc::c_int == '*' as i32 || c as libc::c_int == '/' as i32;
}
#[no_mangle]
pub unsafe extern "C" fn is_term_op(mut c: libc::c_char) -> bool {
    return c as libc::c_int == '+' as i32 || c as libc::c_int == '-' as i32;
}
#[no_mangle]
pub unsafe extern "C" fn is_comparison_op(mut c: libc::c_char) -> bool {
    return c as libc::c_int == '<' as i32
        || c as libc::c_int == '=' as i32
        || c as libc::c_int == '>' as i32;
}
#[no_mangle]
pub unsafe extern "C" fn is_operator(mut c: libc::c_char) -> bool {
    return is_open_paren(c) as libc::c_int != 0
        || is_close_paren(c) as libc::c_int != 0
        || is_factor_op(c) as libc::c_int != 0
        || is_term_op(c) as libc::c_int != 0
        || is_comparison_op(c) as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn is_comment_start(mut c: libc::c_char) -> bool {
    return c as libc::c_int == '#' as i32;
}
#[no_mangle]
pub unsafe extern "C" fn save_position(mut state: *mut parser_state_t) -> size_t {
    return ftell((*state).stream) as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn restore_position(mut state: *mut parser_state_t, mut position: size_t) {
    fseek((*state).stream, position as libc::c_long, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn rewind_one(mut state: *mut parser_state_t) {
    fseek(
        (*state).stream,
        -(1 as libc::c_int) as libc::c_long,
        1 as libc::c_int,
    );
}
/*
 * Advances the provided state to the next token.
 */
#[no_mangle]
pub unsafe extern "C" fn advance(mut state: *mut parser_state_t) -> libc::c_char {
    loop {
        let mut result: libc::c_int = fgetc((*state).stream);
        if result == -(1 as libc::c_int) {
            return '\u{0}' as i32 as libc::c_char;
        }
        if *(*__ctype_b_loc()).offset(result as isize) as libc::c_int
            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            == 0
        {
            return result as libc::c_char;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn try_advance(
    mut state: *mut parser_state_t,
    mut predicate: char_predicate_t,
) -> libc::c_char {
    let mut next: libc::c_char = advance(state);
    if next as libc::c_int != '\u{0}' as i32 && !predicate.expect("non-null function pointer")(next)
    {
        rewind_one(state);
        return '\u{0}' as i32 as libc::c_char;
    }
    return next;
}
#[no_mangle]
pub unsafe extern "C" fn advance_until_separator(
    mut state: *mut parser_state_t,
) -> *mut libc::c_char {
    let mut result: *mut libc::c_char =
        malloc(::std::mem::size_of::<[libc::c_char; 101]>() as libc::c_ulong) as *mut libc::c_char;
    if !result.is_null() {
    } else {
        __assert_fail(
            b"result != NULL\x00" as *const u8 as *const libc::c_char,
            b"src/parser.c\x00" as *const u8 as *const libc::c_char,
            86 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 48], &[libc::c_char; 48]>(
                b"char *advance_until_separator(parser_state_t *)\x00",
            ))
            .as_ptr(),
        );
    }
    let mut index: size_t = 0 as libc::c_int as size_t;
    loop {
        if index > MAX_KEYWORD_LENGTH {
            free(result as *mut libc::c_void);
            return 0 as *mut libc::c_char;
        }
        let mut c: libc::c_int = fgetc((*state).stream);
        if c == -(1 as libc::c_int) {
            if index > 0 as libc::c_int as libc::c_ulong {
                break;
            }
            free(result as *mut libc::c_void);
            return 0 as *mut libc::c_char;
        } else if is_operator(c as libc::c_char) as libc::c_int != 0
            && index > 0 as libc::c_int as libc::c_ulong
        {
            rewind_one(state);
            break;
        } else if *(*__ctype_b_loc()).offset(c as isize) as libc::c_int
            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            != 0
        {
            if index > 0 as libc::c_int as libc::c_ulong {
                break;
            }
        } else {
            let fresh0 = index;
            index = index.wrapping_add(1);
            *result.offset(fresh0 as isize) = c as libc::c_char
        }
    }
    *result.offset(index as isize) = '\u{0}' as i32 as libc::c_char;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn at_end(mut state: *mut parser_state_t) -> bool {
    if advance(state) as libc::c_int != '\u{0}' as i32 {
        rewind_one(state);
        return 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn skip_line(mut state: *mut parser_state_t) {
    loop {
        let mut character: libc::c_int = fgetc((*state).stream);
        if character == -(1 as libc::c_int) || character == '\n' as i32 {
            break;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn num(mut state: *mut parser_state_t) -> *mut node_t {
    let mut num_string: *mut libc::c_char = advance_until_separator(state);
    if num_string.is_null() {
        return 0 as *mut node_t;
    }
    let mut num_end: *mut libc::c_char = 0 as *mut libc::c_char;
    *__errno_location() = 0 as libc::c_int;
    let mut value: value_t = strtol(num_string, &mut num_end, 0 as libc::c_int);
    if *__errno_location() != 0 as libc::c_int || *num_end as libc::c_int != '\u{0}' as i32 {
        free(num_string as *mut libc::c_void);
        return 0 as *mut node_t;
    }
    free(num_string as *mut libc::c_void);
    return init_num_node(value);
}
#[no_mangle]
pub unsafe extern "C" fn factor(mut state: *mut parser_state_t) -> *mut node_t {
    if try_advance(
        state,
        Some(is_open_paren as unsafe extern "C" fn(_: libc::c_char) -> bool),
    ) != 0
    {
        let mut node: *mut node_t = expression(state);
        if try_advance(
            state,
            Some(is_close_paren as unsafe extern "C" fn(_: libc::c_char) -> bool),
        ) == 0
        {
            return 0 as *mut node_t;
        }
        return node;
    }
    let mut var: libc::c_char = try_advance(
        state,
        Some(is_variable_name as unsafe extern "C" fn(_: libc::c_char) -> bool),
    );
    if var != 0 {
        return init_var_node(var);
    }
    return num(state);
}
#[no_mangle]
pub unsafe extern "C" fn term(mut state: *mut parser_state_t) -> *mut node_t {
    let mut result: *mut node_t = factor(state);
    loop {
        let mut next: libc::c_char = try_advance(
            state,
            Some(is_factor_op as unsafe extern "C" fn(_: libc::c_char) -> bool),
        );
        if next == 0 {
            break;
        }
        result = init_binary_node(next, result, factor(state))
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn expression(mut state: *mut parser_state_t) -> *mut node_t {
    let mut result: *mut node_t = term(state);
    loop {
        let mut next: libc::c_char = try_advance(
            state,
            Some(is_term_op as unsafe extern "C" fn(_: libc::c_char) -> bool),
        );
        if next == 0 {
            break;
        }
        result = init_binary_node(next, result, term(state))
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn comparison(mut state: *mut parser_state_t) -> *mut binary_node_t {
    let mut left: *mut node_t = expression(state);
    let mut op: libc::c_char = try_advance(
        state,
        Some(is_comparison_op as unsafe extern "C" fn(_: libc::c_char) -> bool),
    );
    return init_binary_node(op, left, expression(state)) as *mut binary_node_t;
}
#[no_mangle]
pub unsafe extern "C" fn statement(
    mut state: *mut parser_state_t,
    mut end: *mut bool,
) -> *mut node_t {
    while try_advance(
        state,
        Some(is_comment_start as unsafe extern "C" fn(_: libc::c_char) -> bool),
    ) != 0
    {
        skip_line(state);
    }
    let mut start: size_t = save_position(state);
    let mut next: *mut libc::c_char = advance_until_separator(state);
    if next.is_null() {
        *end = 1 as libc::c_int != 0;
        return 0 as *mut node_t;
    }
    if strcmp(next, b"ELSE\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        free(next as *mut libc::c_void);
        restore_position(state, start);
        *end = 1 as libc::c_int != 0;
        return 0 as *mut node_t;
    }
    if strcmp(next, b"END\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        free(next as *mut libc::c_void);
        next = advance_until_separator(state);
        if next.is_null()
            || !(strcmp(next, b"IF\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int
                || strcmp(next, b"WHILE\x00" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int)
        {
            free(next as *mut libc::c_void);
            *end = 0 as libc::c_int != 0;
            return 0 as *mut node_t;
        }
        free(next as *mut libc::c_void);
        restore_position(state, start);
        *end = 1 as libc::c_int != 0;
        return 0 as *mut node_t;
    }
    *end = 0 as libc::c_int != 0;
    if strcmp(next, b"PRINT\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        free(next as *mut libc::c_void);
        return init_print_node(expression(state));
    }
    if strcmp(next, b"LET\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        free(next as *mut libc::c_void);
        let mut var: libc::c_char = advance(state);
        if !(is_variable_name(var) as libc::c_int != 0
            && advance(state) as libc::c_int == '=' as i32)
        {
            return 0 as *mut node_t;
        }
        return init_let_node(var, expression(state));
    }
    if strcmp(next, b"IF\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        free(next as *mut libc::c_void);
        let mut condition: *mut binary_node_t = comparison(state);
        let mut if_branch: *mut node_t = sequence(state);
        next = advance_until_separator(state);
        let mut else_branch: *mut node_t = 0 as *mut node_t;
        if !next.is_null()
            && strcmp(next, b"ELSE\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int
        {
            free(next as *mut libc::c_void);
            else_branch = sequence(state);
            next = advance_until_separator(state)
        } else {
            else_branch = 0 as *mut node_t
        }
        if next.is_null()
            || strcmp(next, b"END\x00" as *const u8 as *const libc::c_char) != 0 as libc::c_int
        {
            free(next as *mut libc::c_void);
            free_ast(condition as *mut node_t);
            free_ast(if_branch);
            free_ast(else_branch);
            return 0 as *mut node_t;
        }
        free(next as *mut libc::c_void);
        next = advance_until_separator(state);
        if next.is_null()
            || strcmp(next, b"IF\x00" as *const u8 as *const libc::c_char) != 0 as libc::c_int
        {
            free(next as *mut libc::c_void);
            free_ast(condition as *mut node_t);
            free_ast(if_branch);
            free_ast(else_branch);
            return 0 as *mut node_t;
        }
        free(next as *mut libc::c_void);
        return init_if_node(condition, if_branch, else_branch);
    }
    if strcmp(next, b"WHILE\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        free(next as *mut libc::c_void);
        let mut condition_0: *mut binary_node_t = comparison(state);
        let mut body: *mut node_t = sequence(state);
        next = advance_until_separator(state);
        if next.is_null()
            || strcmp(next, b"END\x00" as *const u8 as *const libc::c_char) != 0 as libc::c_int
        {
            free(next as *mut libc::c_void);
            free_ast(condition_0 as *mut node_t);
            free_ast(body);
            return 0 as *mut node_t;
        }
        free(next as *mut libc::c_void);
        next = advance_until_separator(state);
        if next.is_null()
            || strcmp(next, b"WHILE\x00" as *const u8 as *const libc::c_char) != 0 as libc::c_int
        {
            free(next as *mut libc::c_void);
            free_ast(condition_0 as *mut node_t);
            free_ast(body);
            return 0 as *mut node_t;
        }
        free(next as *mut libc::c_void);
        return init_while_node(condition_0, body);
    }
    free(next as *mut libc::c_void);
    return 0 as *mut node_t;
}
#[no_mangle]
pub unsafe extern "C" fn sequence(mut state: *mut parser_state_t) -> *mut node_t {
    let mut statement_count: size_t = 0 as libc::c_int as size_t;
    let mut statements: *mut *mut node_t = 0 as *mut *mut node_t;
    let mut statement_capacity: size_t = 0 as libc::c_int as size_t;
    loop {
        let mut end: bool = false;
        let mut next: *mut node_t = statement(state, &mut end);
        if end {
            break;
        }
        if next.is_null() {
            let mut i: size_t = 0 as libc::c_int as size_t;
            while i < statement_count {
                free_ast(*statements.offset(i as isize));
                i = i.wrapping_add(1)
            }
            free(statements as *mut libc::c_void);
            return 0 as *mut node_t;
        }
        if statement_count == statement_capacity {
            statement_capacity = if statement_capacity > 0 as libc::c_int as libc::c_ulong {
                statement_capacity.wrapping_mul(2 as libc::c_int as libc::c_ulong)
            } else {
                1 as libc::c_int as libc::c_ulong
            };
            statements = realloc(
                statements as *mut libc::c_void,
                (::std::mem::size_of::<*mut node_t>() * statement_capacity as usize)
                    as libc::c_ulong,
            ) as *mut *mut node_t;
            if !statements.is_null() {
            } else {
                __assert_fail(
                    b"statements != NULL\x00" as *const u8 as *const libc::c_char,
                    b"src/parser.c\x00" as *const u8 as *const libc::c_char,
                    344 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 35], &[libc::c_char; 35]>(
                        b"node_t *sequence(parser_state_t *)\x00",
                    ))
                    .as_ptr(),
                );
            }
        }
        let fresh1 = statement_count;
        statement_count = statement_count.wrapping_add(1);
        let ref mut fresh2 = *statements.offset(fresh1 as isize);
        *fresh2 = next
    }
    // Avoid allocating a sequence_node_t if there is only one statement
    if statement_count == 1 as libc::c_int as libc::c_ulong {
        let mut statement_0: *mut node_t = *statements;
        free(statements as *mut libc::c_void);
        return statement_0;
    }
    if statement_count > 0 as libc::c_int as libc::c_ulong {
        statements = realloc(
            statements as *mut libc::c_void,
            (::std::mem::size_of::<*mut node_t>() * statement_count as usize) as libc::c_ulong,
        ) as *mut *mut node_t;
        if !statements.is_null() {
        } else {
            __assert_fail(
                b"statements != NULL\x00" as *const u8 as *const libc::c_char,
                b"src/parser.c\x00" as *const u8 as *const libc::c_char,
                358 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 35], &[libc::c_char; 35]>(
                    b"node_t *sequence(parser_state_t *)\x00",
                ))
                .as_ptr(),
            );
        }
    }
    return init_sequence_node(statement_count, statements);
}
#[no_mangle]
pub unsafe extern "C" fn parse(mut stream: *mut FILE) -> *mut node_t {
    let mut state: parser_state_t = {
        let mut init = parser_state_t { stream: stream };
        init
    };
    let mut ast: *mut node_t = sequence(&mut state);
    if !at_end(&mut state) {
        free_ast(ast);
        return 0 as *mut node_t;
    }
    return ast;
}
