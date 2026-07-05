//! Port of: engine/engine_inline.h
//! IR hash: 699b5f0da57e8d78
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mji_zero3 (engine/engine_inline.h:46)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_zero3(res: mjtNum__restrict) {
    // WARNING: signature changed — verify body
    // Previous params: (res : mjtNum__restrict)
    // Previous return: ()
    todo ! ()
}

/// C: mji_copy3 (engine/engine_inline.h:55)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_copy3(res: mjtNum__restrict, vec: *const f64) {
    // WARNING: signature changed — verify body
    // Previous params: (res : mjtNum__restrict, vec : * const f64)
    // Previous return: ()
    todo ! ()
}

/// C: mji_scl3 (engine/engine_inline.h:64)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_scl3(res: mjtNum__restrict, vec: *const f64, scl: f64) {
    // WARNING: signature changed — verify body
    // Previous params: (res : mjtNum__restrict, vec : * const f64, scl : f64)
    // Previous return: ()
    todo ! ()
}

/// C: mji_add3 (engine/engine_inline.h:73)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_add3(res: mjtNum__restrict, vec1: *const f64, vec2: *const f64) {
    // WARNING: signature changed — verify body
    // Previous params: (res : mjtNum__restrict, vec1 : * const f64, vec2 : * const f64)
    // Previous return: ()
    todo ! ()
}

/// C: mji_sub3 (engine/engine_inline.h:82)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_sub3(res: mjtNum__restrict, vec1: *const f64, vec2: *const f64) {
    // WARNING: signature changed — verify body
    // Previous params: (res : mjtNum__restrict, vec1 : * const f64, vec2 : * const f64)
    // Previous return: ()
    todo ! ()
}

/// C: mji_addTo3 (engine/engine_inline.h:91)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_add_to3(res: mjtNum__restrict, vec: *const f64) {
    // WARNING: signature changed — verify body
    // Previous params: (res : mjtNum__restrict, vec : * const f64)
    // Previous return: ()
    todo ! ()
}

/// C: mji_subFrom3 (engine/engine_inline.h:100)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_sub_from3(res: mjtNum__restrict, vec: *const f64) {
    // WARNING: signature changed — verify body
    // Previous params: (res : mjtNum__restrict, vec : * const f64)
    // Previous return: ()
    todo ! ()
}

/// C: mji_addToScl3 (engine/engine_inline.h:109)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_add_to_scl3(res: mjtNum__restrict, vec: *const f64, scl: f64) {
    // WARNING: signature changed — verify body
    // Previous params: (res : mjtNum__restrict, vec : * const f64, scl : f64)
    // Previous return: ()
    todo ! ()
}

/// C: mji_addScl3 (engine/engine_inline.h:118)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_add_scl3(res: mjtNum__restrict, vec1: *const f64, vec2: *const f64, scl: f64) {
    // WARNING: signature changed — verify body
    // Previous params: (res : mjtNum__restrict, vec1 : * const f64, vec2 : * const f64, scl : f64)
    // Previous return: ()
    todo ! ()
}

/// C: mji__normalize3 (engine/engine_inline.h:128)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_normalize3(vec: *mut f64) -> f64 {
    // WARNING: signature changed — verify body
    // Previous params: (vec : * mut f64)
    // Previous return: f64
    todo ! ()
}

/// C: mji_mulMatVec3 (engine/engine_inline.h:148)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_mul_mat_vec3(res: mjtNum__restrict, mat: *const f64, vec: *const f64) {
    // WARNING: signature changed — verify body
    // Previous params: (res : mjtNum__restrict, mat : * const f64, vec : * const f64)
    // Previous return: ()
    todo ! ()
}

/// C: mji_mulMatTVec3 (engine/engine_inline.h:158)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_mul_mat_t_vec3(res: mjtNum__restrict, mat: *const f64, vec: *const f64) {
    // WARNING: signature changed — verify body
    // Previous params: (res : mjtNum__restrict, mat : * const f64, vec : * const f64)
    // Previous return: ()
    todo ! ()
}

/// C: mji_mulMatMat3 (engine/engine_inline.h:168)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_mul_mat_mat3(res: mjtNum__restrict, mat1: *const f64, mat2: *const f64) {
    // WARNING: signature changed — verify body
    // Previous params: (res : mjtNum__restrict, mat1 : * const f64, mat2 : * const f64)
    // Previous return: ()
    todo ! ()
}

/// C: mji_mulMatTMat3 (engine/engine_inline.h:184)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_mul_mat_t_mat3(res: mjtNum__restrict, mat1: *const f64, mat2: *const f64) {
    // WARNING: signature changed — verify body
    // Previous params: (res : mjtNum__restrict, mat1 : * const f64, mat2 : * const f64)
    // Previous return: ()
    todo ! ()
}

/// C: mji_transpose3 (engine/engine_inline.h:200)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_transpose3(res: mjtNum__restrict, mat: *const f64) {
    // WARNING: signature changed — verify body
    // Previous params: (res : mjtNum__restrict, mat : * const f64)
    // Previous return: ()
    todo ! ()
}

/// C: mji_copy4 (engine/engine_inline.h:218)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_copy4(res: mjtNum__restrict, data: *const f64) {
    // WARNING: signature changed — verify body
    // Previous params: (res : mjtNum__restrict, data : * const f64)
    // Previous return: ()
    todo ! ()
}

/// C: mji__normalize4 (engine/engine_inline.h:229)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_normalize4(vec: *mut f64) -> f64 {
    // WARNING: signature changed — verify body
    // Previous params: (vec : * mut f64)
    // Previous return: f64
    todo ! ()
}

/// C: mji_rotVecQuat (engine/engine_inline.h:253)
/// Calls: mji_copy3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_rot_vec_quat(res: mjtNum__restrict, vec: *const f64, quat: *const f64) {
    // WARNING: signature changed — verify body
    // Previous params: (res : mjtNum__restrict, vec : * const f64, quat : * const f64)
    // Previous return: ()
    todo ! ()
}

/// C: mji_negQuat (engine/engine_inline.h:277)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_neg_quat(res: mjtNum__restrict, quat: *const f64) {
    // WARNING: signature changed — verify body
    // Previous params: (res : mjtNum__restrict, quat : * const f64)
    // Previous return: ()
    todo ! ()
}

/// C: mji_mulQuat (engine/engine_inline.h:287)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_mul_quat(res: mjtNum__restrict, qa: *const f64, qb: *const f64) {
    // WARNING: signature changed — verify body
    // Previous params: (res : mjtNum__restrict, qa : * const f64, qb : * const f64)
    // Previous return: ()
    todo ! ()
}

/// C: mji_mulQuatAxis (engine/engine_inline.h:298)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_mul_quat_axis(res: mjtNum__restrict, quat: *const f64, axis: *const f64) {
    // WARNING: signature changed — verify body
    // Previous params: (res : mjtNum__restrict, quat : * const f64, axis : * const f64)
    // Previous return: ()
    todo ! ()
}

/// C: mji_axisAngle2Quat (engine/engine_inline.h:309)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_axis_angle2quat(res: mjtNum__restrict, axis: *const f64, angle: f64) {
    // WARNING: signature changed — verify body
    // Previous params: (res : mjtNum__restrict, axis : * const f64, angle : f64)
    // Previous return: ()
    todo ! ()
}

/// C: mji_quat2Vel (engine/engine_inline.h:331)
/// Calls: mji__normalize3, mji_scl3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_quat2vel(res: mjtNum__restrict, quat: *const f64, dt: f64) {
    // WARNING: signature changed — verify body
    // Previous params: (res : mjtNum__restrict, quat : * const f64, dt : f64)
    // Previous return: ()
    todo ! ()
}

/// C: mji_subQuat (engine/engine_inline.h:348)
/// Calls: mji_mulQuat, mji_negQuat, mji_quat2Vel
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_sub_quat(res: mjtNum__restrict, qa: *const f64, qb: *const f64) {
    // WARNING: signature changed — verify body
    // Previous params: (res : mjtNum__restrict, qa : * const f64, qb : * const f64)
    // Previous return: ()
    todo ! ()
}

/// C: mji_mat2Quat (engine/engine_inline.h:361)
/// Calls: mji__normalize4
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_mat2quat(quat: mjtNum__restrict, mat: *const f64) {
    // WARNING: signature changed — verify body
    // Previous params: (quat : mjtNum__restrict, mat : * const f64)
    // Previous return: ()
    todo ! ()
}

/// C: mji_quatIntegrate (engine/engine_inline.h:401)
/// Calls: mji__normalize3, mji__normalize4, mji_axisAngle2Quat, mji_copy3, mji_copy4, mji_mulQuat
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_quat_integrate(quat: mjtNum__restrict, vel: *const f64, scale: f64) {
    // WARNING: signature changed — verify body
    // Previous params: (quat : mjtNum__restrict, vel : * const f64, scale : f64)
    // Previous return: ()
    todo ! ()
}

/// C: mji_cross (engine/engine_inline.h:419)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_cross(res: mjtNum__restrict, a: *const f64, b: *const f64) {
    // WARNING: signature changed — verify body
    // Previous params: (res : mjtNum__restrict, a : * const f64, b : * const f64)
    // Previous return: ()
    todo ! ()
}

/// C: mji_crossMotion (engine/engine_inline.h:429)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_cross_motion(res: mjtNum__restrict, vel: *const f64, v: *const f64) {
    // WARNING: signature changed — verify body
    // Previous params: (res : mjtNum__restrict, vel : * const f64, v : * const f64)
    // Previous return: ()
    todo ! ()
}

/// C: mji_crossForce (engine/engine_inline.h:446)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_cross_force(res: mjtNum__restrict, vel: *const f64, f: *const f64) {
    // WARNING: signature changed — verify body
    // Previous params: (res : mjtNum__restrict, vel : * const f64, f : * const f64)
    // Previous return: ()
    todo ! ()
}

/// C: mji_dot6 (engine/engine_inline.h:463)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_dot6(vec1: *const f64, vec2: *const f64) -> f64 {
    // WARNING: signature changed — verify body
    // Previous params: (vec1 : * const f64, vec2 : * const f64)
    // Previous return: f64
    todo ! ()
}

/// C: mji_copy6 (engine/engine_inline.h:473)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_copy6(res: mjtNum__restrict, vec: *const f64) {
    // WARNING: signature changed — verify body
    // Previous params: (res : mjtNum__restrict, vec : * const f64)
    // Previous return: ()
    todo ! ()
}

/// C: mji_copy9 (engine/engine_inline.h:485)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_copy9(res: mjtNum__restrict, data: *const f64) {
    // WARNING: signature changed — verify body
    // Previous params: (res : mjtNum__restrict, data : * const f64)
    // Previous return: ()
    todo ! ()
}

