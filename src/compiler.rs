#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, main, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    #[no_mangle]
    static mut stderr: *mut FILE;
    #[no_mangle]
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    fn compile_ast(node: *mut node_t) -> bool;
    #[no_mangle]
    fn free_ast(node: *mut node_t);
    #[no_mangle]
    fn print_ast(node: *mut node_t);
    #[no_mangle]
    fn parse(stream: *mut FILE) -> *mut node_t;
}
pub type size_t = libc::c_ulong;
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
pub type node_type_t = libc::c_uint;
pub const WHILE: node_type_t = 7;
pub const IF: node_type_t = 6;
pub const LET: node_type_t = 5;
pub const PRINT: node_type_t = 4;
pub const SEQUENCE: node_type_t = 3;
pub const VAR: node_type_t = 2;
pub const BINARY_OP: node_type_t = 1;
pub const NUM: node_type_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct node_t {
    pub type_0: node_type_t,
}
#[no_mangle]
pub unsafe extern "C" fn usage(mut program: *mut libc::c_char) {
    fprintf(stderr,
            b"USAGE: %s <program file>\n\x00" as *const u8 as
                *const libc::c_char, program);
    exit(1 as libc::c_int);
}
/* *
 * Prints the start of the the x86-64 assembly output.
 * The assembly code implementing the TeenyBASIC statements
 * goes between the header and the footer.
 */
#[no_mangle]
pub unsafe extern "C" fn header() {
    printf(b"# The code section of the assembly file\n.text\n.globl basic_main\nbasic_main:\n    # The main() function\n\x00"
               as *const u8 as *const libc::c_char);
}
/* *
 * Prints the end of the x86-64 assembly output.
 * The assembly code implementing the TeenyBASIC statements
 * goes between the header and the footer.
 */
#[no_mangle]
pub unsafe extern "C" fn footer() {
    printf(b"    ret\n\x00" as *const u8 as *const libc::c_char);
}
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char)
 -> libc::c_int {
    if argc != 2 as libc::c_int {
        usage(*argv.offset(0 as libc::c_int as isize));
    }
    let mut program: *mut FILE =
        fopen(*argv.offset(1 as libc::c_int as isize),
              b"r\x00" as *const u8 as *const libc::c_char);
    if program.is_null() { usage(*argv.offset(0 as libc::c_int as isize)); }
    header();
    let mut ast: *mut node_t = parse(program);
    fclose(program);
    if ast.is_null() {
        fprintf(stderr,
                b"Parse error\n\x00" as *const u8 as *const libc::c_char);
        return 2 as libc::c_int
    }
    // Display the AST for debugging purposes
    print_ast(ast);
    // Compile the AST into assembly instructions
    if !compile_ast(ast) {
        free_ast(ast);
        fprintf(stderr,
                b"Compilation error\n\x00" as *const u8 as
                    *const libc::c_char);
        return 3 as libc::c_int
    }
    free_ast(ast);
    footer();
    return 0;
}
#[main]
pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(::std::ffi::CString::new(arg).expect("Failed to convert argument into CString.").into_raw());
    };
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0((args.len() - 1) as libc::c_int,
                                    args.as_mut_ptr() as
                                        *mut *mut libc::c_char) as i32)
    }
}
