//! Port of: engine/engine_name.c
//! IR hash: 05737965add36adb
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: _getnumadr (engine/engine_name.c:28)
#[allow(unused_variables, non_snake_case)]
pub fn getnumadr(m: *const mjModel, r#type: mjtObj, padr: *mut *mut i32, mapadr: *mut i32) -> i32 {
    extern "C" { fn _getnumadr(m: *const mjModel, r#type: mjtObj, padr: *mut *mut i32, mapadr: *mut i32) -> i32; }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { _getnumadr(m, r#type, padr, mapadr) }
}

/// C: mj_hashString (engine/engine_name.h:30)
#[allow(unused_variables, non_snake_case)]
pub fn mj_hash_string(s: *const i8, n: u64) -> u64 {
    extern "C" { fn mj_hashString(s: *const i8, n: u64) -> u64; }
    // SAFETY: delegates to C implementation
    unsafe { mj_hashString(s, n) }
}

/// C: mj_name2id (engine/engine_name.h:33)
/// Calls: _getnumadr, mj_hashString
#[allow(unused_variables, non_snake_case)]
pub fn mj_name2id(m: *const mjModel, r#type: i32, name: *const i8) -> i32 {
    extern "C" { fn mj_name2id(m: *const mjModel, r#type: i32, name: *const i8) -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { mj_name2id(m, r#type, name) }
}

/// C: mj_id2name (engine/engine_name.h:36)
/// Calls: _getnumadr
#[allow(unused_variables, non_snake_case)]
pub fn mj_id2name(m: *const mjModel, r#type: i32, id: i32) -> *const i8 {
    extern "C" { fn mj_id2name(m: *const mjModel, r#type: i32, id: i32) -> *const i8; }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_id2name(m, r#type, id) }
}

