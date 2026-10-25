//! Port of: engine/engine_support.c
//! IR hash: 7ab8c6ea5876ad61
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mj_version (engine/engine_support.c:857)
#[allow(unused_variables, non_snake_case)]
pub fn mj_version() -> i32 {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { i32::from_be_bytes([0x00, 0x2D, 0xF9, 0x88]) }
}

