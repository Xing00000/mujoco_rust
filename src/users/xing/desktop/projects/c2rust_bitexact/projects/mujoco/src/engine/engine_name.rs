//! Port of: Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_name.c
//! IR hash: 1fa44043fe383ce9
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: _getnumadr (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_name.c:28)
/// Calls: cxx:__ZN11mjCActuator3actERKNSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEE, cxx:__ZNK6mujoco4user8FilePath4sizeEv
#[allow(unused_variables, non_snake_case)]
pub fn getnumadr(m: *const mjModel, r#type: u32, padr: *mut *mut i32, mapadr: *mut i32) -> i32 {
    todo!() // cxx-internal:engine_name.c.o:__getnumadr
}

/// C: mj_hashString (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_name.h:30)
#[allow(unused_variables, non_snake_case)]
pub fn mj_hash_string(s: *const i8, n: u64) -> u64 {
    todo!() // cxx:_mj_hashString
}

/// C: mj_name2id (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_name.h:33)
/// Calls: cxx-internal:engine_name.c.o:__getnumadr, cxx:_mj_hashString
#[allow(unused_variables, non_snake_case)]
pub fn mj_name2id(m: *const mjModel, r#type: i32, name: *const i8) -> i32 {
    todo!() // cxx:_mj_name2id
}

/// C: mj_id2name (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_name.h:36)
/// Calls: cxx-internal:engine_name.c.o:__getnumadr
#[allow(unused_variables, non_snake_case)]
pub fn mj_id2name(m: *const mjModel, r#type: i32, id: i32) -> *const i8 {
    todo!() // cxx:_mj_id2name
}

