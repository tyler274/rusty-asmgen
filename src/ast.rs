#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, label_break_value,
           register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    #[no_mangle]
    static mut stderr: *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
}
pub type __int64_t = libc::c_long;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type int64_t = __int64_t;
pub type size_t = libc::c_ulong;
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
pub struct num_node_t {
    pub base: node_t,
    pub value: value_t,
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
pub struct var_node_t {
    pub base: node_t,
    pub name: var_name_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sequence_node_t {
    pub base: node_t,
    pub statement_count: size_t,
    pub statements: *mut *mut node_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct print_node_t {
    pub base: node_t,
    pub expr: *mut node_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct let_node_t {
    pub base: node_t,
    pub var: var_name_t,
    pub value: *mut node_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct if_node_t {
    pub base: node_t,
    pub condition: *mut binary_node_t,
    pub if_branch: *mut node_t,
    pub else_branch: *mut node_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct while_node_t {
    pub base: node_t,
    pub condition: *mut binary_node_t,
    pub body: *mut node_t,
}
pub type FILE = _IO_FILE;
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
#[no_mangle]
pub unsafe extern "C" fn init_num_node(mut value: value_t) -> *mut node_t {
    let mut node: *mut num_node_t =
        malloc(::std::mem::size_of::<num_node_t>() as libc::c_ulong) as
            *mut num_node_t;
    if !node.is_null() {
    } else {
        __assert_fail(b"node != NULL\x00" as *const u8 as *const libc::c_char,
                      b"src/ast.c\x00" as *const u8 as *const libc::c_char,
                      11 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 31],
                                                &[libc::c_char; 31]>(b"node_t *init_num_node(value_t)\x00")).as_ptr());
    }
    (*node).base.type_0 = NUM;
    (*node).value = value;
    return node as *mut node_t;
}
#[no_mangle]
pub unsafe extern "C" fn init_binary_node(mut op: libc::c_char,
                                          mut left: *mut node_t,
                                          mut right: *mut node_t)
 -> *mut node_t {
    if left.is_null() || right.is_null() {
        free_ast(left);
        free_ast(right);
        return 0 as *mut node_t
    }
    let mut node: *mut binary_node_t =
        malloc(::std::mem::size_of::<binary_node_t>() as libc::c_ulong) as
            *mut binary_node_t;
    if !node.is_null() {
    } else {
        __assert_fail(b"node != NULL\x00" as *const u8 as *const libc::c_char,
                      b"src/ast.c\x00" as *const u8 as *const libc::c_char,
                      25 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 51],
                                                &[libc::c_char; 51]>(b"node_t *init_binary_node(char, node_t *, node_t *)\x00")).as_ptr());
    }
    (*node).base.type_0 = BINARY_OP;
    (*node).op = op;
    (*node).left = left;
    (*node).right = right;
    return node as *mut node_t;
}
#[no_mangle]
pub unsafe extern "C" fn init_var_node(mut name: var_name_t) -> *mut node_t {
    if name as libc::c_int == '\u{0}' as i32 { return 0 as *mut node_t }
    let mut node: *mut var_node_t =
        malloc(::std::mem::size_of::<var_node_t>() as libc::c_ulong) as
            *mut var_node_t;
    if !node.is_null() {
    } else {
        __assert_fail(b"node != NULL\x00" as *const u8 as *const libc::c_char,
                      b"src/ast.c\x00" as *const u8 as *const libc::c_char,
                      39 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 34],
                                                &[libc::c_char; 34]>(b"node_t *init_var_node(var_name_t)\x00")).as_ptr());
    }
    (*node).base.type_0 = VAR;
    (*node).name = name;
    return node as *mut node_t;
}
#[no_mangle]
pub unsafe extern "C" fn init_sequence_node(mut statement_count: size_t,
                                            mut statements: *mut *mut node_t)
 -> *mut node_t {
    if statements.is_null() &&
           statement_count > 0 as libc::c_int as libc::c_ulong {
        return 0 as *mut node_t
    }
    let mut node: *mut sequence_node_t =
        malloc(::std::mem::size_of::<sequence_node_t>() as libc::c_ulong) as
            *mut sequence_node_t;
    if !node.is_null() {
    } else {
        __assert_fail(b"node != NULL\x00" as *const u8 as *const libc::c_char,
                      b"src/ast.c\x00" as *const u8 as *const libc::c_char,
                      51 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 46],
                                                &[libc::c_char; 46]>(b"node_t *init_sequence_node(size_t, node_t **)\x00")).as_ptr());
    }
    (*node).base.type_0 = SEQUENCE;
    (*node).statement_count = statement_count;
    (*node).statements = statements;
    return node as *mut node_t;
}
#[no_mangle]
pub unsafe extern "C" fn init_print_node(mut expr: *mut node_t)
 -> *mut node_t {
    if expr.is_null() { return 0 as *mut node_t }
    let mut node: *mut print_node_t =
        malloc(::std::mem::size_of::<print_node_t>() as libc::c_ulong) as
            *mut print_node_t;
    if !node.is_null() {
    } else {
        __assert_fail(b"node != NULL\x00" as *const u8 as *const libc::c_char,
                      b"src/ast.c\x00" as *const u8 as *const libc::c_char,
                      64 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 34],
                                                &[libc::c_char; 34]>(b"node_t *init_print_node(node_t *)\x00")).as_ptr());
    }
    (*node).base.type_0 = PRINT;
    (*node).expr = expr;
    return node as *mut node_t;
}
#[no_mangle]
pub unsafe extern "C" fn init_let_node(mut var: var_name_t,
                                       mut value: *mut node_t)
 -> *mut node_t {
    if var as libc::c_int == '\u{0}' as i32 || value.is_null() {
        free_ast(value);
        return 0 as *mut node_t
    }
    let mut node: *mut let_node_t =
        malloc(::std::mem::size_of::<let_node_t>() as libc::c_ulong) as
            *mut let_node_t;
    if !node.is_null() {
    } else {
        __assert_fail(b"node != NULL\x00" as *const u8 as *const libc::c_char,
                      b"src/ast.c\x00" as *const u8 as *const libc::c_char,
                      77 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 44],
                                                &[libc::c_char; 44]>(b"node_t *init_let_node(var_name_t, node_t *)\x00")).as_ptr());
    }
    (*node).base.type_0 = LET;
    (*node).var = var;
    (*node).value = value;
    return node as *mut node_t;
}
#[no_mangle]
pub unsafe extern "C" fn init_if_node(mut condition: *mut binary_node_t,
                                      mut if_branch: *mut node_t,
                                      mut else_branch: *mut node_t)
 -> *mut node_t {
    if condition.is_null() || if_branch.is_null() {
        free_ast(condition as *mut node_t);
        free_ast(if_branch);
        free_ast(else_branch);
        return 0 as *mut node_t
    }
    let mut node: *mut if_node_t =
        malloc(::std::mem::size_of::<if_node_t>() as libc::c_ulong) as
            *mut if_node_t;
    if !node.is_null() {
    } else {
        __assert_fail(b"node != NULL\x00" as *const u8 as *const libc::c_char,
                      b"src/ast.c\x00" as *const u8 as *const libc::c_char,
                      93 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 58],
                                                &[libc::c_char; 58]>(b"node_t *init_if_node(binary_node_t *, node_t *, node_t *)\x00")).as_ptr());
    }
    (*node).base.type_0 = IF;
    (*node).condition = condition;
    (*node).if_branch = if_branch;
    (*node).else_branch = else_branch;
    return node as *mut node_t;
}
#[no_mangle]
pub unsafe extern "C" fn init_while_node(mut condition: *mut binary_node_t,
                                         mut body: *mut node_t)
 -> *mut node_t {
    if condition.is_null() || body.is_null() {
        free_ast(condition as *mut node_t);
        free_ast(body);
        return 0 as *mut node_t
    }
    let mut node: *mut while_node_t =
        malloc(::std::mem::size_of::<while_node_t>() as libc::c_ulong) as
            *mut while_node_t;
    if !node.is_null() {
    } else {
        __assert_fail(b"node != NULL\x00" as *const u8 as *const libc::c_char,
                      b"src/ast.c\x00" as *const u8 as *const libc::c_char,
                      109 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 51],
                                                &[libc::c_char; 51]>(b"node_t *init_while_node(binary_node_t *, node_t *)\x00")).as_ptr());
    }
    (*node).base.type_0 = WHILE;
    (*node).condition = condition;
    (*node).body = body;
    return node as *mut node_t;
}
#[no_mangle]
pub unsafe extern "C" fn free_ast(mut node: *mut node_t) {
    if node.is_null() { return }
    if (*node).type_0 as libc::c_uint ==
           BINARY_OP as libc::c_int as libc::c_uint {
        let mut bin: *mut binary_node_t = node as *mut binary_node_t;
        free_ast((*bin).left);
        free_ast((*bin).right);
    } else if (*node).type_0 as libc::c_uint ==
                  SEQUENCE as libc::c_int as libc::c_uint {
        let mut sequence: *mut sequence_node_t = node as *mut sequence_node_t;
        let mut i: size_t = 0 as libc::c_int as size_t;
        while i < (*sequence).statement_count {
            free_ast(*(*sequence).statements.offset(i as isize));
            i = i.wrapping_add(1)
        }
        free((*sequence).statements as *mut libc::c_void);
    } else if (*node).type_0 as libc::c_uint ==
                  PRINT as libc::c_int as libc::c_uint {
        free_ast((*(node as *mut print_node_t)).expr);
    } else if (*node).type_0 as libc::c_uint ==
                  LET as libc::c_int as libc::c_uint {
        free_ast((*(node as *mut let_node_t)).value);
    } else if (*node).type_0 as libc::c_uint ==
                  IF as libc::c_int as libc::c_uint {
        let mut conditional: *mut if_node_t = node as *mut if_node_t;
        free_ast((*conditional).condition as *mut node_t);
        free_ast((*conditional).if_branch);
        free_ast((*conditional).else_branch);
    } else if (*node).type_0 as libc::c_uint ==
                  WHILE as libc::c_int as libc::c_uint {
        let mut loop_0: *mut while_node_t = node as *mut while_node_t;
        free_ast((*loop_0).condition as *mut node_t);
        free_ast((*loop_0).body);
    }
    free(node as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn print_indent(mut indent: size_t) {
    while indent > 0 as libc::c_int as libc::c_ulong {
        fprintf(stderr, b"\t\x00" as *const u8 as *const libc::c_char);
        indent = indent.wrapping_sub(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn print_ast_indented(mut node: *mut node_t,
                                            mut indent: size_t) {
    if (*node).type_0 as libc::c_uint == NUM as libc::c_int as libc::c_uint {
        fprintf(stderr, b"%ld\x00" as *const u8 as *const libc::c_char,
                (*(node as *mut num_node_t)).value);
    } else if (*node).type_0 as libc::c_uint ==
                  BINARY_OP as libc::c_int as libc::c_uint {
        let mut bin: *mut binary_node_t = node as *mut binary_node_t;
        fprintf(stderr, b"%c(\x00" as *const u8 as *const libc::c_char,
                (*bin).op as libc::c_int);
        print_ast_indented((*bin).left, indent);
        fprintf(stderr, b", \x00" as *const u8 as *const libc::c_char);
        print_ast_indented((*bin).right, indent);
        fprintf(stderr, b")\x00" as *const u8 as *const libc::c_char);
    } else if (*node).type_0 as libc::c_uint ==
                  VAR as libc::c_int as libc::c_uint {
        fprintf(stderr, b"%c\x00" as *const u8 as *const libc::c_char,
                (*(node as *mut var_node_t)).name as libc::c_int);
    } else if (*node).type_0 as libc::c_uint ==
                  SEQUENCE as libc::c_int as libc::c_uint {
        let mut sequence: *mut sequence_node_t = node as *mut sequence_node_t;
        let mut i: size_t = 0 as libc::c_int as size_t;
        while i < (*sequence).statement_count {
            print_ast_indented(*(*sequence).statements.offset(i as isize),
                               indent);
            i = i.wrapping_add(1)
        }
    } else if (*node).type_0 as libc::c_uint ==
                  PRINT as libc::c_int as libc::c_uint {
        print_indent(indent);
        fprintf(stderr, b"PRINT(\x00" as *const u8 as *const libc::c_char);
        print_ast_indented((*(node as *mut print_node_t)).expr, indent);
        fprintf(stderr, b")\n\x00" as *const u8 as *const libc::c_char);
    } else if (*node).type_0 as libc::c_uint ==
                  LET as libc::c_int as libc::c_uint {
        print_indent(indent);
        let mut let_0: *mut let_node_t = node as *mut let_node_t;
        fprintf(stderr, b"LET(%c, \x00" as *const u8 as *const libc::c_char,
                (*let_0).var as libc::c_int);
        print_ast_indented((*let_0).value, indent);
        fprintf(stderr, b")\n\x00" as *const u8 as *const libc::c_char);
    } else if (*node).type_0 as libc::c_uint ==
                  IF as libc::c_int as libc::c_uint {
        let mut conditional: *mut if_node_t = node as *mut if_node_t;
        print_indent(indent);
        fprintf(stderr, b"IF(\x00" as *const u8 as *const libc::c_char);
        print_ast_indented((*conditional).condition as *mut node_t, indent);
        fprintf(stderr, b",\n\x00" as *const u8 as *const libc::c_char);
        print_ast_indented((*conditional).if_branch,
                           indent.wrapping_add(1 as libc::c_int as
                                                   libc::c_ulong));
        if !(*conditional).else_branch.is_null() {
            print_indent(indent);
            fprintf(stderr, b",\n\x00" as *const u8 as *const libc::c_char);
            print_ast_indented((*conditional).else_branch,
                               indent.wrapping_add(1 as libc::c_int as
                                                       libc::c_ulong));
        }
        print_indent(indent);
        fprintf(stderr, b")\n\x00" as *const u8 as *const libc::c_char);
    } else if (*node).type_0 as libc::c_uint ==
                  WHILE as libc::c_int as libc::c_uint {
        let mut loop_0: *mut while_node_t = node as *mut while_node_t;
        print_indent(indent);
        fprintf(stderr, b"WHILE(\x00" as *const u8 as *const libc::c_char);
        print_ast_indented((*loop_0).condition as *mut node_t, indent);
        fprintf(stderr, b",\n\x00" as *const u8 as *const libc::c_char);
        print_ast_indented((*loop_0).body,
                           indent.wrapping_add(1 as libc::c_int as
                                                   libc::c_ulong));
        print_indent(indent);
        fprintf(stderr, b")\n\x00" as *const u8 as *const libc::c_char);
    } else {
        fprintf(stderr,
                b"\nUnknown node type: %d\n\x00" as *const u8 as
                    *const libc::c_char, (*node).type_0 as libc::c_uint);
        __assert_fail(b"false\x00" as *const u8 as *const libc::c_char,
                      b"src/ast.c\x00" as *const u8 as *const libc::c_char,
                      221 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 42],
                                                &[libc::c_char; 42]>(b"void print_ast_indented(node_t *, size_t)\x00")).as_ptr());
    };
}
#[no_mangle]
pub unsafe extern "C" fn print_ast(mut node: *mut node_t) {
    print_ast_indented(node, 0 as libc::c_int as size_t);
}
