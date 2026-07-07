//! Port of: cc/array_safety.h
//! IR hash: 05737965add36adb
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: sizeof_arr (cc/array_safety.h:39)
#[allow(unused_variables, non_snake_case)]
pub fn sizeof_arr(arr: [T; 0]) -> std__size_t {
    extern "C" { fn sizeof_arr_impl(arr: [T; 0]) -> std__size_t; }
    // SAFETY: delegates to C implementation
    unsafe { sizeof_arr_impl(arr) }
}

/// C: strcmp_arr (cc/array_safety.h:45)
#[allow(unused_variables, non_snake_case)]
pub fn strcmp_arr(lhs: [char; 0], rhs: [char; 0]) -> i32 {
    extern "C" { fn strcmp_arr_impl(lhs: [char; 0], rhs: [char; 0]) -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { strcmp_arr_impl(lhs, rhs) }
}

/// C: strlen_arr (cc/array_safety.h:52)
#[allow(unused_variables, non_snake_case)]
pub fn strlen_arr(str: [char; 0]) -> std__size_t {
    extern "C" { fn strlen_arr_impl(str: [char; 0]) -> std__size_t; }
    // SAFETY: delegates to C implementation
    unsafe { strlen_arr_impl(str) }
}

/// C: sprintf_arr (cc/array_safety.h:64)
#[allow(unused_variables, non_snake_case)]
pub fn sprintf_arr(dest: [char; 0], format: *const i8) -> i32 {
    extern "C" { fn sprintf_arr_impl(dest: [char; 0], format: *const i8) -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { sprintf_arr_impl(dest, format) }
}

/// C: strcat_arr (cc/array_safety.h:75)
#[allow(unused_variables, non_snake_case)]
pub fn strcat_arr(dest: [char; 0], src: *const i8) -> *mut i8 {
    extern "C" { fn strcat_arr_impl(dest: [char; 0], src: *const i8) -> *mut i8; }
    // SAFETY: delegates to C implementation
    unsafe { strcat_arr_impl(dest, src) }
}

/// C: strcpy_arr (cc/array_safety.h:91)
#[allow(unused_variables, non_snake_case)]
pub fn strcpy_arr(dest: [char; 0], src: *const i8) -> *mut i8 {
    extern "C" { fn strcpy_arr_impl(dest: [char; 0], src: *const i8) -> *mut i8; }
    // SAFETY: delegates to C implementation
    unsafe { strcpy_arr_impl(dest, src) }
}

