//! Port of: engine/engine_name.c
//! IR hash: 699b5f0da57e8d78
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: _getnumadr (engine/engine_name.c:28)
#[allow(unused_variables, non_snake_case)]
pub fn getnumadr(m: *const mjModel, r#type: mjtObj, padr: *mut *mut i32, mapadr: *mut i32) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, r#type : mjtObj, padr : * mut * mut i32, mapadr : * mut i32)
    // Previous return: i32
    todo ! ()
}

/// C: mj_hashString (engine/engine_name.c:229)
#[allow(unused_variables, non_snake_case)]
pub fn mj_hash_string(s: *const i8, n: i32) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (s : * const i8, n : i32)
    // Previous return: i32
    todo ! ()
}

/// C: mj_name2id (engine/engine_name.h:33)
/// Calls: _getnumadr
#[allow(unused_variables, non_snake_case)]
pub fn mj_name2id(m: *const mjModel, r#type: i32, name: *const i8) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, r#type : i32, name : * const i8)
    // Previous return: i32
    todo ! ()
}

/// C: mj_id2name (engine/engine_name.h:36)
/// Calls: _getnumadr
#[allow(unused_variables, non_snake_case)]
pub fn mj_id2name(m: *const mjModel, r#type: i32, id: i32) -> *const i8 {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, r#type : i32, id : i32)
    // Previous return: * const i8
    todo ! ()
}

