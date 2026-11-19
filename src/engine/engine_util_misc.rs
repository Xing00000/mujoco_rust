//! Port of: engine/engine_util_misc.c
//! IR hash: ea387639a811bdc4
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

#[allow(unused_variables, non_snake_case)]
pub fn history_physical_index(cursor: i32, n: i32, logical: i32) -> i32 {
    (cursor + 1 + logical) % n
}


#[allow(unused_variables, non_snake_case)]
pub fn mju_flex_dphi(s: f64, i: i32, order: i32) -> f64 {
    if order == 1 {
        return if i == 0 { -1.0 } else { 1.0 };
    }
    match i {
        0 => 4.0 * s - 3.0,
        1 => 4.0 * (1.0 - 2.0 * s),
        2 => 4.0 * s - 1.0,
        _ => 0.0,
    }
}

#[allow(unused_variables, non_snake_case)]
pub fn __c2rust_mju_flex_dphi_03023421067f2166df72bf42140336b696fcae5b2b2c183f5975de709f37c56e(s: f64, i: i32, order: i32) -> f64 {
    if order == 1 {
        return if i == 0 { -1.0 } else { 1.0 };
    }
    match i {
        0 => 4.0 * s - 3.0,
        1 => 4.0 * (1.0 - 2.0 * s),
        2 => 4.0 * s - 1.0,
        _ => 0.0,
    }
}
