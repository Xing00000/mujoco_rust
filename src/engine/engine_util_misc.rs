//! Port of: engine/engine_util_misc.c
//! IR hash: ea387639a811bdc4
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

#[allow(unused_variables, non_snake_case)]
pub fn history_physical_index(cursor: i32, n: i32, logical: i32) -> i32 {
    (cursor + 1 + logical) % n
}

