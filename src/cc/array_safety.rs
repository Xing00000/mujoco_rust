//! Port of: cc/array_safety.h
//! IR hash: bd605ac8158c32d6
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: sizeof_arr (cc/array_safety.h:39)
#[allow(unused_variables, non_snake_case)]
pub fn sizeof_arr(arr: [T; 0]) -> u64 {
    todo!() // sizeof_arr
}

/// C: strcmp_arr (cc/array_safety.h:45)
#[allow(unused_variables, non_snake_case)]
pub fn strcmp_arr(lhs: [char; 0], rhs: [char; 0]) -> i32 {
    todo!() // strcmp_arr
}

/// C: strlen_arr (cc/array_safety.h:52)
#[allow(unused_variables, non_snake_case)]
pub fn strlen_arr(str: [char; 0]) -> u64 {
    todo!() // strlen_arr
}

/// C: sprintf_arr (cc/array_safety.h:64)
#[allow(unused_variables, non_snake_case)]
pub fn sprintf_arr(dest: [char; 0], format: *const i8) -> i32 {
    todo!() // sprintf_arr
}

/// C: strcat_arr (cc/array_safety.h:75)
/// Calls: sizeof_arr, strlen_arr
#[allow(unused_variables, non_snake_case)]
pub fn strcat_arr(dest: [char; 0], src: *const i8) -> *mut i8 {
    todo!() // strcat_arr
}

/// C: strcpy_arr (cc/array_safety.h:91)
#[allow(unused_variables, non_snake_case)]
pub fn strcpy_arr(dest: [char; 0], src: *const i8) -> *mut i8 {
    todo!() // strcpy_arr
}

