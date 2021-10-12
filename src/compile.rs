#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
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
pub unsafe extern "C" fn compile_ast(mut node: *mut node_t) -> bool {
    /* You should remove this cast to void in your solution.
     * It is just here so the code compiles without warnings. */
    return 0 as libc::c_int != 0;
    // for now, every statement causes a compilation failure
}
