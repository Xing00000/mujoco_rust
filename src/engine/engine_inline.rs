//! Port of: engine/engine_inline.h
//! IR hash: 32301b9dc9774d55
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mji_zero3 (engine/engine_inline.h:46)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_zero3(res: *mut f64) {
    // NOTE: signature changed from previous IR version
    // Previous params: (res : * mut f64)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mji_copy3 (engine/engine_inline.h:55)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_copy3(res: *mut f64, vec: *const f64) {
    // NOTE: signature changed from previous IR version
    // Previous params: (res : * mut f64, vec : * const f64)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mji_scl3 (engine/engine_inline.h:64)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_scl3(res: *mut f64, vec: *const f64, scl: f64) {
    // NOTE: signature changed from previous IR version
    // Previous params: (res : * mut f64, vec : * const f64, scl : f64)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mji_add3 (engine/engine_inline.h:73)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_add3(res: *mut f64, vec1: *const f64, vec2: *const f64) {
    // NOTE: signature changed from previous IR version
    // Previous params: (res : * mut f64, vec1 : * const f64, vec2 : * const f64)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mji_sub3 (engine/engine_inline.h:82)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_sub3(res: *mut f64, vec1: *const f64, vec2: *const f64) {
    // NOTE: signature changed from previous IR version
    // Previous params: (res : * mut f64, vec1 : * const f64, vec2 : * const f64)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mji_addTo3 (engine/engine_inline.h:91)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_add_to3(res: *mut f64, vec: *const f64) {
    // NOTE: signature changed from previous IR version
    // Previous params: (res : * mut f64, vec : * const f64)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mji_subFrom3 (engine/engine_inline.h:100)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_sub_from3(res: *mut f64, vec: *const f64) {
    // NOTE: signature changed from previous IR version
    // Previous params: (res : * mut f64, vec : * const f64)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mji_addToScl3 (engine/engine_inline.h:109)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_add_to_scl3(res: *mut f64, vec: *const f64, scl: f64) {
    // NOTE: signature changed from previous IR version
    // Previous params: (res : * mut f64, vec : * const f64, scl : f64)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mji_addScl3 (engine/engine_inline.h:118)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_add_scl3(res: *mut f64, vec1: *const f64, vec2: *const f64, scl: f64) {
    // NOTE: signature changed from previous IR version
    // Previous params: (res : * mut f64, vec1 : * const f64, vec2 : * const f64, scl : f64)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mji__normalize3 (engine/engine_inline.h:128)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_normalize3(vec: *mut f64) -> f64 {
    // NOTE: signature changed from previous IR version
    // Previous params: (vec : * mut f64)
    // Previous return: f64
    todo!("re-translate: params renamed")
}

/// C: mji_mulMatVec3 (engine/engine_inline.h:148)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_mul_mat_vec3(res: *mut f64, mat: *const f64, vec: *const f64) {
    // NOTE: signature changed from previous IR version
    // Previous params: (res : * mut f64, mat : * const f64, vec : * const f64)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mji_mulMatTVec3 (engine/engine_inline.h:158)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_mul_mat_t_vec3(res: *mut f64, mat: *const f64, vec: *const f64) {
    // NOTE: signature changed from previous IR version
    // Previous params: (res : * mut f64, mat : * const f64, vec : * const f64)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mji_mulMatMat3 (engine/engine_inline.h:168)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_mul_mat_mat3(res: *mut f64, mat1: *const f64, mat2: *const f64) {
    // NOTE: signature changed from previous IR version
    // Previous params: (res : * mut f64, mat1 : * const f64, mat2 : * const f64)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mji_mulMatTMat3 (engine/engine_inline.h:184)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_mul_mat_t_mat3(res: *mut f64, mat1: *const f64, mat2: *const f64) {
    // NOTE: signature changed from previous IR version
    // Previous params: (res : * mut f64, mat1 : * const f64, mat2 : * const f64)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mji_transpose3 (engine/engine_inline.h:200)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_transpose3(res: *mut f64, mat: *const f64) {
    // NOTE: signature changed from previous IR version
    // Previous params: (res : * mut f64, mat : * const f64)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mji_copy4 (engine/engine_inline.h:218)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_copy4(res: *mut f64, data: *const f64) {
    // NOTE: signature changed from previous IR version
    // Previous params: (res : * mut f64, data : * const f64)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mji__normalize4 (engine/engine_inline.h:229)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_normalize4(vec: *mut f64) -> f64 {
    // NOTE: signature changed from previous IR version
    // Previous params: (vec : * mut f64)
    // Previous return: f64
    todo!("re-translate: params renamed")
}

/// C: mji_rotVecQuat (engine/engine_inline.h:253)
/// Calls: mji_copy3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_rot_vec_quat(res: *mut f64, vec: *const f64, quat: *const f64) {
    // NOTE: signature changed from previous IR version
    // Previous params: (res : * mut f64, vec : * const f64, quat : * const f64)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mji_negQuat (engine/engine_inline.h:277)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_neg_quat(res: *mut f64, quat: *const f64) {
    // NOTE: signature changed from previous IR version
    // Previous params: (res : * mut f64, quat : * const f64)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mji_mulQuat (engine/engine_inline.h:287)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_mul_quat(res: *mut f64, qa: *const f64, qb: *const f64) {
    // NOTE: signature changed from previous IR version
    // Previous params: (res : * mut f64, qa : * const f64, qb : * const f64)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mji_mulQuatAxis (engine/engine_inline.h:298)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_mul_quat_axis(res: *mut f64, quat: *const f64, axis: *const f64) {
    // NOTE: signature changed from previous IR version
    // Previous params: (res : * mut f64, quat : * const f64, axis : * const f64)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mji_axisAngle2Quat (engine/engine_inline.h:309)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_axis_angle2quat(res: *mut f64, axis: *const f64, angle: f64) {
    // NOTE: signature changed from previous IR version
    // Previous params: (res : * mut f64, axis : * const f64, angle : f64)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mji_quat2Vel (engine/engine_inline.h:331)
/// Calls: mji__normalize3, mji_scl3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_quat2vel(res: *mut f64, quat: *const f64, dt: f64) {
    // NOTE: signature changed from previous IR version
    // Previous params: (res : * mut f64, quat : * const f64, dt : f64)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mji_subQuat (engine/engine_inline.h:348)
/// Calls: mji_mulQuat, mji_negQuat, mji_quat2Vel
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_sub_quat(res: *mut f64, qa: *const f64, qb: *const f64) {
    // NOTE: signature changed from previous IR version
    // Previous params: (res : * mut f64, qa : * const f64, qb : * const f64)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mji_mat2Quat (engine/engine_inline.h:361)
/// Calls: mji__normalize4
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_mat2quat(quat: *mut f64, mat: *const f64) {
    // NOTE: signature changed from previous IR version
    // Previous params: (quat : * mut f64, mat : * const f64)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mji_quatIntegrate (engine/engine_inline.h:401)
/// Calls: mji__normalize3, mji__normalize4, mji_axisAngle2Quat, mji_copy3, mji_copy4, mji_mulQuat
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_quat_integrate(quat: *mut f64, vel: *const f64, scale: f64) {
    // NOTE: signature changed from previous IR version
    // Previous params: (quat : * mut f64, vel : * const f64, scale : f64)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mji_cross (engine/engine_inline.h:419)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_cross(res: *mut f64, a: *const f64, b: *const f64) {
    // NOTE: signature changed from previous IR version
    // Previous params: (res : * mut f64, a : * const f64, b : * const f64)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mji_crossMotion (engine/engine_inline.h:429)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_cross_motion(res: *mut f64, vel: *const f64, v: *const f64) {
    // NOTE: signature changed from previous IR version
    // Previous params: (res : * mut f64, vel : * const f64, v : * const f64)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mji_crossForce (engine/engine_inline.h:446)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_cross_force(res: *mut f64, vel: *const f64, f: *const f64) {
    // NOTE: signature changed from previous IR version
    // Previous params: (res : * mut f64, vel : * const f64, f : * const f64)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mji_dot6 (engine/engine_inline.h:463)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_dot6(vec1: *const f64, vec2: *const f64) -> f64 {
    // NOTE: signature changed from previous IR version
    // Previous params: (vec1 : * const f64, vec2 : * const f64)
    // Previous return: f64
    todo!("re-translate: params renamed")
}

/// C: mji_copy6 (engine/engine_inline.h:473)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_copy6(res: *mut f64, vec: *const f64) {
    // NOTE: signature changed from previous IR version
    // Previous params: (res : * mut f64, vec : * const f64)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mji_copy9 (engine/engine_inline.h:485)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_copy9(res: *mut f64, data: *const f64) {
    // NOTE: signature changed from previous IR version
    // Previous params: (res : * mut f64, data : * const f64)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

