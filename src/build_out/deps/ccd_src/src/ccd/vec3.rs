//! Port of: build_out/_deps/ccd-src/src/ccd/vec3.h
//! IR hash: 4d1eafce2d6676a9
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;


#[allow(unused_variables, non_snake_case)]
pub fn ccd_vec3add(v: *mut ccd_vec3_t, w: *const ccd_vec3_t) -> () {
    unsafe {
        (*v).v[0] += (*w).v[0];
        (*v).v[1] += (*w).v[1];
        (*v).v[2] += (*w).v[2];
    }
}

#[allow(unused_variables, non_snake_case)]
pub unsafe fn __c2rust_ccd_is_zero_8a51f44dca72b0755b91c2f7a674a86fb87cbf0c839b555a0a99ea8bc96ce6a8(val: f64) -> i32 {
    (val.abs() < ::core::primitive::f64::from_bits(0x3cb0_0000_0000_0000)) as i32
}