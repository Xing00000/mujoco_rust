//! Port of: Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_box.c
//! IR hash: 1fa44043fe383ce9
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mju_clampVec (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_box.c:23)
/// Calls: cxx:_mju_clip
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_clamp_vec(vec: *mut f64, limit: *const f64, n: i32) {
    todo!() // cxx-internal:engine_collision_box.c.o:_mju_clampVec
}

/// C: mjraw_SphereBox (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_box.c:34)
/// Calls: cxx-internal:engine_collision_box.c.o:_mju_clampVec, cxx:_mju_normalize3, cxx:_mju_zero3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjraw_sphere_box(con: *mut mjPreContact, margin: f64, pos1: *const f64, mat1: *const f64, size1: *const f64, pos2: *const f64, mat2: *const f64, size2: *const f64) -> i32 {
    todo!() // cxx:_mjraw_SphereBox
}

/// C: _boxbox (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_box.c:605)
/// Calls: cxx:_mju_copy3, cxx:_mju_dot3, cxx:_mju_mulMatMatT3, cxx:_mju_mulMatTMat3, cxx:_mju_normalize3, cxx:_mju_scl3, cxx:_mju_transpose, cxx:_mju_zero, cxx:_mju_zero3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn boxbox(M: *const mjModel, D: *const mjData, con: *mut mjPreContact, g1: i32, g2: i32, margin: f64) -> i32 {
    todo!() // cxx-internal:engine_collision_box.c.o:__boxbox
}

