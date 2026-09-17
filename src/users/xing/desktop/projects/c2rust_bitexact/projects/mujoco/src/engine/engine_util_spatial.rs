//! Port of: Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_spatial.h
//! IR hash: 1fa44043fe383ce9
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mju_rotVecQuat (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_spatial.h:27)
/// Calls: cxx:_mju_zero3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_rot_vec_quat(res: *mut f64, vec: *const f64, quat: *const f64) {
    todo!() // cxx:_mju_rotVecQuat
}

/// C: mju_negQuat (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_spatial.h:30)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_neg_quat(res: *mut f64, quat: *const f64) {
    todo!() // cxx:_mju_negQuat
}

/// C: mju_mulQuat (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_spatial.h:33)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_mul_quat(res: *mut f64, quat1: *const f64, quat2: *const f64) {
    todo!() // cxx:_mju_mulQuat
}

/// C: mju_mulQuatAxis (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_spatial.h:36)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_mul_quat_axis(res: *mut f64, quat: *const f64, axis: *const f64) {
    todo!() // cxx:_mju_mulQuatAxis
}

/// C: mju_axisAngle2Quat (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_spatial.h:39)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_axis_angle2quat(res: *mut f64, axis: *const f64, angle: f64) {
    todo!() // cxx:_mju_axisAngle2Quat
}

/// C: mju_quat2Vel (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_spatial.h:42)
/// Calls: cxx:_mju_normalize3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_quat2vel(res: *mut f64, quat: *const f64, dt: f64) {
    todo!() // cxx:_mju_quat2Vel
}

/// C: mju_subQuat (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_spatial.h:45)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_sub_quat(res: *mut f64, qa: *const f64, qb: *const f64) {
    todo!() // cxx:_mju_subQuat
}

/// C: mju_quat2Mat (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_spatial.h:48)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_quat2mat(res: *mut f64, quat: *const f64) {
    todo!() // cxx:_mju_quat2Mat
}

/// C: mju_mat2Quat (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_spatial.h:51)
/// Calls: cxx:_mju_normalize4
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_mat2quat(quat: *mut f64, mat: *const f64) {
    todo!() // cxx:_mju_mat2Quat
}

/// C: mju_derivQuat (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_spatial.h:54)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_deriv_quat(res: *mut f64, quat: *const f64, vel: *const f64) {
    todo!() // cxx:_mju_derivQuat
}

/// C: mju_quatIntegrate (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_spatial.h:57)
/// Calls: cxx:_mju_mulQuat, cxx:_mju_normalize3, cxx:_mju_normalize4
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_quat_integrate(quat: *mut f64, vel: *const f64, scale: f64) {
    todo!() // cxx:_mju_quatIntegrate
}

/// C: mju_quatZ2Vec (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_spatial.h:60)
/// Calls: cxx:_mju_dot3, cxx:_mju_normalize3, cxx:_mju_zero3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_quat_z2vec(quat: *mut f64, vec: *const f64) {
    todo!() // cxx:_mju_quatZ2Vec
}

/// C: mju_mat2Rot (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_spatial.h:64)
/// Calls: cxx:_mju_dot3, cxx:_mju_mulQuat, cxx:_mju_normalize3, cxx:_mju_normalize4, cxx:_mju_quat2Mat, cxx:_mju_scl3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_mat2rot(quat: *mut f64, mat: *const f64) -> i32 {
    todo!() // cxx:_mju_mat2Rot
}

/// C: mju_mulPose (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_spatial.h:70)
/// Calls: cxx:_mju_normalize4
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_mul_pose(posres: *mut f64, quatres: *mut f64, pos1: *const f64, quat1: *const f64, pos2: *const f64, quat2: *const f64) {
    todo!() // cxx:_mju_mulPose
}

/// C: mju_negPose (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_spatial.h:75)
/// Calls: cxx:_mju_scl3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_neg_pose(posres: *mut f64, quatres: *mut f64, pos: *const f64, quat: *const f64) {
    todo!() // cxx:_mju_negPose
}

/// C: mju_trnVecPose (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_spatial.h:79)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_trn_vec_pose(res: *mut f64, pos: *const f64, quat: *const f64, vec: *const f64) {
    todo!() // cxx:_mju_trnVecPose
}

/// C: mju_euler2Quat (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_spatial.h:84)
/// Calls: cxx:_mju_message, cxx:_mju_mulQuat
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_euler2quat(quat: *mut f64, euler: *const f64, seq: *const i8) {
    todo!() // cxx:_mju_euler2Quat
}

/// C: mju_cross (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_spatial.h:89)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_cross(res: *mut f64, a: *const f64, b: *const f64) {
    todo!() // cxx:_mju_cross
}

/// C: mju_crossMotion (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_spatial.h:92)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_cross_motion(res: *mut f64, vel: *const f64, v: *const f64) {
    todo!() // cxx:_mju_crossMotion
}

/// C: mju_crossForce (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_spatial.h:95)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_cross_force(res: *mut f64, vel: *const f64, f: *const f64) {
    todo!() // cxx:_mju_crossForce
}

/// C: mju_inertCom (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_spatial.h:98)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_inert_com(res: *mut f64, inert: *const f64, mat: *const f64, dif: *const f64, mass: f64) {
    todo!() // cxx:_mju_inertCom
}

/// C: mju_dofCom (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_spatial.h:102)
/// Calls: cxx:_mju_zero3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_dof_com(res: *mut f64, axis: *const f64, offset: *const f64) {
    todo!() // cxx:_mju_dofCom
}

/// C: mju_mulInertVec (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_spatial.h:105)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_mul_inert_vec(res: *mut f64, inert: *const f64, vec: *const f64) {
    todo!() // cxx:_mju_mulInertVec
}

/// C: mju_mulDofVec (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_spatial.h:108)
/// Calls: cxx:_mju_mulMatTVec, cxx:_mju_scl, cxx:_mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_mul_dof_vec(res: *mut f64, mat: *const f64, vec: *const f64, n: i32) {
    todo!() // cxx:_mju_mulDofVec
}

/// C: mju_transformSpatial (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_spatial.h:112)
/// Calls: cxx:_mju_copy
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_transform_spatial(res: *mut f64, vec: *const f64, flg_force: i32, newpos: *const f64, oldpos: *const f64, rotnew2old: *const f64) {
    todo!() // cxx:_mju_transformSpatial
}

/// C: mju_makeFrame (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_spatial.h:117)
/// Calls: cxx:_mju_dot3, cxx:_mju_message, cxx:_mju_normalize3, cxx:_mju_zero3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_make_frame(frame: *mut f64) {
    todo!() // cxx:_mju_makeFrame
}

