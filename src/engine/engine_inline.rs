//! Port of: engine/engine_inline.h
//! IR hash: fd5288ad7242f001
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
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_zero3
}

/// C: mji_copy3 (engine/engine_inline.h:55)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_copy3(res: *mut f64, vec: *const f64) {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_copy3
}

/// C: mji_scl3 (engine/engine_inline.h:64)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_scl3(res: *mut f64, vec: *const f64, scl: f64) {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_scl3
}

/// C: mji_add3 (engine/engine_inline.h:73)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_add3(res: *mut f64, vec1: *const f64, vec2: *const f64) {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_add3
}

/// C: mji_sub3 (engine/engine_inline.h:82)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_sub3(res: *mut f64, vec1: *const f64, vec2: *const f64) {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_sub3
}

/// C: mji_addTo3 (engine/engine_inline.h:91)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_add_to3(res: *mut f64, vec: *const f64) {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_addTo3
}

/// C: mji_subFrom3 (engine/engine_inline.h:100)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_sub_from3(res: *mut f64, vec: *const f64) {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_subFrom3
}

/// C: mji_addToScl3 (engine/engine_inline.h:109)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_add_to_scl3(res: *mut f64, vec: *const f64, scl: f64) {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_addToScl3
}

/// C: mji_addScl3 (engine/engine_inline.h:118)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_add_scl3(res: *mut f64, vec1: *const f64, vec2: *const f64, scl: f64) {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_addScl3
}

/// C: mji__normalize3 (engine/engine_inline.h:128)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_normalize3(vec: *mut f64) -> f64 {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji__normalize3
}

/// C: mji_mulMatVec3 (engine/engine_inline.h:148)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_mul_mat_vec3(res: *mut f64, mat: *const f64, vec: *const f64) {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_mulMatVec3
}

/// C: mji_mulMatTVec3 (engine/engine_inline.h:158)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_mul_mat_t_vec3(res: *mut f64, mat: *const f64, vec: *const f64) {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_mulMatTVec3
}

/// C: mji_mulMatMat3 (engine/engine_inline.h:168)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_mul_mat_mat3(res: *mut f64, mat1: *const f64, mat2: *const f64) {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_mulMatMat3
}

/// C: mji_mulMatTMat3 (engine/engine_inline.h:184)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_mul_mat_t_mat3(res: *mut f64, mat1: *const f64, mat2: *const f64) {
    todo!() // cxx-internal:engine_collision_convex.c.o:_mji_mulMatTMat3
}

/// C: mji_transpose3 (engine/engine_inline.h:200)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_transpose3(res: *mut f64, mat: *const f64) {
    // SAFETY: caller guarantees res[9] and mat[9] are valid, no aliasing
    unsafe {
        *res.add(0) = *mat.add(0);
        *res.add(1) = *mat.add(3);
        *res.add(2) = *mat.add(6);
        *res.add(3) = *mat.add(1);
        *res.add(4) = *mat.add(4);
        *res.add(5) = *mat.add(7);
        *res.add(6) = *mat.add(2);
        *res.add(7) = *mat.add(5);
        *res.add(8) = *mat.add(8);
    }
}

/// C: mji_copy4 (engine/engine_inline.h:218)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_copy4(res: *mut f64, data: *const f64) {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_copy4
}

/// C: mji__normalize4 (engine/engine_inline.h:229)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_normalize4(vec: *mut f64) -> f64 {
    const MJ_MINVAL: f64 = 1E-15_f64;
    // SAFETY: caller guarantees vec[4] is valid
    unsafe {
        let norm = (*vec.add(0) * *vec.add(0)
            + *vec.add(1) * *vec.add(1)
            + *vec.add(2) * *vec.add(2)
            + *vec.add(3) * *vec.add(3)).sqrt();

        if norm < MJ_MINVAL {
            *vec.add(0) = 1.0;
            *vec.add(1) = 0.0;
            *vec.add(2) = 0.0;
            *vec.add(3) = 0.0;
        } else if (norm - 1.0).abs() > MJ_MINVAL {
            let norm_inv = 1.0 / norm;
            *vec.add(0) *= norm_inv;
            *vec.add(1) *= norm_inv;
            *vec.add(2) *= norm_inv;
            *vec.add(3) *= norm_inv;
        }

        norm
    }
}

/// C: mji_rotVecQuat (engine/engine_inline.h:253)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_copy3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_rot_vec_quat(res: *mut f64, vec: *const f64, quat: *const f64) {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_rotVecQuat
}

/// C: mji_negQuat (engine/engine_inline.h:277)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_neg_quat(res: *mut f64, quat: *const f64) {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_negQuat
}

/// C: mji_mulQuat (engine/engine_inline.h:287)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_mul_quat(res: *mut f64, qa: *const f64, qb: *const f64) {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_mulQuat
}

/// C: mji_mulQuatAxis (engine/engine_inline.h:298)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_mul_quat_axis(res: *mut f64, quat: *const f64, axis: *const f64) {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_mulQuatAxis
}

/// C: mji_axisAngle2Quat (engine/engine_inline.h:309)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_axis_angle2quat(res: *mut f64, axis: *const f64, angle: f64) {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_axisAngle2Quat
}

/// C: mji_quat2Vel (engine/engine_inline.h:331)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji__normalize3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_scl3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_quat2vel(res: *mut f64, quat: *const f64, dt: f64) {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_quat2Vel
}

/// C: mji_subQuat (engine/engine_inline.h:348)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_mulQuat, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_negQuat, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_quat2Vel
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_sub_quat(res: *mut f64, qa: *const f64, qb: *const f64) {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_subQuat
}

/// C: mji_mat2Quat (engine/engine_inline.h:361)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji__normalize4
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_mat2quat(quat: *mut f64, mat: *const f64) {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_mat2Quat
}

/// C: mji_quatIntegrate (engine/engine_inline.h:401)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji__normalize3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji__normalize4, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_axisAngle2Quat, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_copy3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_copy4, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_mulQuat
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_quat_integrate(quat: *mut f64, vel: *const f64, scale: f64) {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_quatIntegrate
}

/// C: mji_cross (engine/engine_inline.h:419)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_cross(res: *mut f64, a: *const f64, b: *const f64) {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_cross
}

/// C: mji_crossMotion (engine/engine_inline.h:429)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_cross_motion(res: *mut f64, vel: *const f64, v: *const f64) {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_crossMotion
}

/// C: mji_crossForce (engine/engine_inline.h:446)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_cross_force(res: *mut f64, vel: *const f64, f: *const f64) {
    todo!() // cxx-internal:engine_core_smooth.c.o:_mji_crossForce
}

/// C: mji_dot6 (engine/engine_inline.h:463)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_dot6(vec1: *const f64, vec2: *const f64) -> f64 {
    // SAFETY: caller guarantees vec1[6] and vec2[6] are valid
    // match order of operations to mju_dot
    unsafe {
        ((*vec1.add(0) * *vec2.add(0) + *vec1.add(2) * *vec2.add(2))
            + (*vec1.add(1) * *vec2.add(1) + *vec1.add(3) * *vec2.add(3)))
            + (*vec1.add(4) * *vec2.add(4) + *vec1.add(5) * *vec2.add(5))
    }
}

/// C: mji_copy6 (engine/engine_inline.h:473)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_copy6(res: *mut f64, vec: *const f64) {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_copy6
}

/// C: mji_copy9 (engine/engine_inline.h:485)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_copy9(res: *mut f64, data: *const f64) {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_copy9
}

