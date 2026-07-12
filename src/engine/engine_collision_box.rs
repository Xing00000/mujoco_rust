//! Port of: engine/engine_collision_box.c
//! IR hash: c6d98e4f4b63b7f2
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mju_clampVec (engine/engine_collision_box.c:23)
/// Calls: mju_clip
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_clamp_vec(vec: *mut f64, limit: *const f64, n: i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (vec : * mut f64, limit : * const f64, n : i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjraw_SphereBox (engine/engine_collision_box.c:34)
/// Calls: mji_add3, mji_addToScl3, mji_copy3, mji_mulMatTVec3, mji_mulMatVec3, mji_sub3, mji_zero3, mju_clampVec, mju_normalize3, mju_zero3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjraw_sphere_box(con: *mut mjPreContact, margin: f64, pos1: *const f64, mat1: *const f64, size1: *const f64, pos2: *const f64, mat2: *const f64, size2: *const f64) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (con : * mut mjPreContact, margin : f64, pos1 : * const f64, mat1 : * const f64, size1 : * const f64, pos2 : * const f64, mat2 : * const f64, size2 : * const f64)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: _boxbox (engine/engine_collision_box.c:605)
/// Calls: mji_add3, mji_addTo3, mji_addToScl3, mji_copy3, mji_mulMatTVec3, mji_mulMatVec3, mji_scl3, mji_sub3, mji_zero3, mju_copy3, mju_dot3, mju_mulMatMatT3, mju_mulMatTMat3, mju_normalize3, mju_scl3, mju_transpose, mju_zero, mju_zero3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn boxbox(M: *const mjModel, D: *const mjData, con: *mut mjPreContact, g1: i32, g2: i32, margin: f64) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (M : * const mjModel, D : * const mjData, con : * mut mjPreContact, g1 : i32, g2 : i32, margin : f64)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

