//! Port of: engine/engine_name.c
//! IR hash: e878892ca48fe222
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: _getnumadr (engine/engine_name.c:28)
/// Calls: FilePath::size, mjCActuator::act
#[allow(unused_variables, non_snake_case)]
pub fn getnumadr(m: *const mjModel, r#type: u32, padr: *mut *mut i32, mapadr: *mut i32) -> i32 {
    todo!() // _getnumadr
}

/// C: mj_hashString (engine/engine_name.h:30)
#[allow(unused_variables, non_snake_case)]
pub fn mj_hash_string(s: *const i8, n: u64) -> u64 {
    todo!() // mj_hashString
}

/// C: mj_name2id (engine/engine_name.h:33)
/// Calls: _getnumadr, mj_hashString
#[allow(unused_variables, non_snake_case)]
pub fn mj_name2id(m: *const mjModel, r#type: i32, name: *const i8) -> i32 {
    todo!() // mj_name2id
}

/// C: mj_id2name (engine/engine_name.h:36)
/// Calls: _getnumadr
#[allow(unused_variables, non_snake_case)]
pub fn mj_id2name(m: *const mjModel, r#type: i32, id: i32) -> *const i8 {
    todo!() // mj_id2name
}

