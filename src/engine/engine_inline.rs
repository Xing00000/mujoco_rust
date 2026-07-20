//! Port of: engine/engine_inline.h
//! IR hash: 73a9f665ec0ecfc0
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
    // SAFETY: caller guarantees res points to 3 f64
    unsafe {
        *res.add(0) = 0.0;
        *res.add(1) = 0.0;
        *res.add(2) = 0.0;
    }
}

/// C: mji_copy3 (engine/engine_inline.h:55)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_copy3(res: *mut f64, vec: *const f64) {
    // SAFETY: caller guarantees res[3] and vec[3] are valid
    unsafe {
        *res.add(0) = *vec.add(0);
        *res.add(1) = *vec.add(1);
        *res.add(2) = *vec.add(2);
    }
}

/// C: mji_scl3 (engine/engine_inline.h:64)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_scl3(res: *mut f64, vec: *const f64, scl: f64) {
    // SAFETY: caller guarantees res[3] and vec[3] are valid
    unsafe {
        *res.add(0) = *vec.add(0) * scl;
        *res.add(1) = *vec.add(1) * scl;
        *res.add(2) = *vec.add(2) * scl;
    }
}

/// C: mji_add3 (engine/engine_inline.h:73)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_add3(res: *mut f64, vec1: *const f64, vec2: *const f64) {
    // SAFETY: caller guarantees res[3], vec1[3], vec2[3] are valid
    unsafe {
        *res.add(0) = *vec1.add(0) + *vec2.add(0);
        *res.add(1) = *vec1.add(1) + *vec2.add(1);
        *res.add(2) = *vec1.add(2) + *vec2.add(2);
    }
}

/// C: mji_sub3 (engine/engine_inline.h:82)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_sub3(res: *mut f64, vec1: *const f64, vec2: *const f64) {
    // SAFETY: caller guarantees res[3], vec1[3], vec2[3] are valid
    unsafe {
        *res.add(0) = *vec1.add(0) - *vec2.add(0);
        *res.add(1) = *vec1.add(1) - *vec2.add(1);
        *res.add(2) = *vec1.add(2) - *vec2.add(2);
    }
}

/// C: mji_addTo3 (engine/engine_inline.h:91)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_add_to3(res: *mut f64, vec: *const f64) {
    // SAFETY: caller guarantees res[3] and vec[3] are valid
    unsafe {
        *res.add(0) += *vec.add(0);
        *res.add(1) += *vec.add(1);
        *res.add(2) += *vec.add(2);
    }
}

/// C: mji_subFrom3 (engine/engine_inline.h:100)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_sub_from3(res: *mut f64, vec: *const f64) {
    // SAFETY: caller guarantees res[3] and vec[3] are valid
    unsafe {
        *res.add(0) -= *vec.add(0);
        *res.add(1) -= *vec.add(1);
        *res.add(2) -= *vec.add(2);
    }
}

/// C: mji_addToScl3 (engine/engine_inline.h:109)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_add_to_scl3(res: *mut f64, vec: *const f64, scl: f64) {
    // SAFETY: caller guarantees res[3] and vec[3] are valid
    unsafe {
        *res.add(0) += *vec.add(0) * scl;
        *res.add(1) += *vec.add(1) * scl;
        *res.add(2) += *vec.add(2) * scl;
    }
}

/// C: mji_addScl3 (engine/engine_inline.h:118)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_add_scl3(res: *mut f64, vec1: *const f64, vec2: *const f64, scl: f64) {
    // SAFETY: caller guarantees res[3], vec1[3], vec2[3] are valid
    unsafe {
        *res.add(0) = *vec1.add(0) + scl * *vec2.add(0);
        *res.add(1) = *vec1.add(1) + scl * *vec2.add(1);
        *res.add(2) = *vec1.add(2) + scl * *vec2.add(2);
    }
}

/// C: mji__normalize3 (engine/engine_inline.h:128)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_normalize3(vec: *mut f64) -> f64 {
    const MJ_MINVAL: f64 = 1E-15_f64;
    // SAFETY: caller guarantees vec[3] is valid
    unsafe {
        let norm = (*vec.add(0) * *vec.add(0)
            + *vec.add(1) * *vec.add(1)
            + *vec.add(2) * *vec.add(2)).sqrt();

        if norm < MJ_MINVAL {
            *vec.add(0) = 1.0;
            *vec.add(1) = 0.0;
            *vec.add(2) = 0.0;
        } else {
            let norm_inv = 1.0 / norm;
            *vec.add(0) *= norm_inv;
            *vec.add(1) *= norm_inv;
            *vec.add(2) *= norm_inv;
        }

        norm
    }
}

/// C: mji_mulMatVec3 (engine/engine_inline.h:148)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_mul_mat_vec3(res: *mut f64, mat: *const f64, vec: *const f64) {
    // SAFETY: caller guarantees res[3], mat[9], vec[3] are valid, res != vec
    unsafe {
        *res.add(0) = *mat.add(0) * *vec.add(0) + *mat.add(1) * *vec.add(1) + *mat.add(2) * *vec.add(2);
        *res.add(1) = *mat.add(3) * *vec.add(0) + *mat.add(4) * *vec.add(1) + *mat.add(5) * *vec.add(2);
        *res.add(2) = *mat.add(6) * *vec.add(0) + *mat.add(7) * *vec.add(1) + *mat.add(8) * *vec.add(2);
    }
}

/// C: mji_mulMatTVec3 (engine/engine_inline.h:158)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_mul_mat_t_vec3(res: *mut f64, mat: *const f64, vec: *const f64) {
    // SAFETY: caller guarantees res[3], mat[9], vec[3] are valid, res != vec
    unsafe {
        *res.add(0) = *mat.add(0) * *vec.add(0) + *mat.add(3) * *vec.add(1) + *mat.add(6) * *vec.add(2);
        *res.add(1) = *mat.add(1) * *vec.add(0) + *mat.add(4) * *vec.add(1) + *mat.add(7) * *vec.add(2);
        *res.add(2) = *mat.add(2) * *vec.add(0) + *mat.add(5) * *vec.add(1) + *mat.add(8) * *vec.add(2);
    }
}

/// C: mji_mulMatMat3 (engine/engine_inline.h:168)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_mul_mat_mat3(res: *mut f64, mat1: *const f64, mat2: *const f64) {
    // SAFETY: caller guarantees res[9], mat1[9], mat2[9] are valid, no aliasing
    unsafe {
        *res.add(0) = *mat1.add(0) * *mat2.add(0) + *mat1.add(1) * *mat2.add(3) + *mat1.add(2) * *mat2.add(6);
        *res.add(1) = *mat1.add(0) * *mat2.add(1) + *mat1.add(1) * *mat2.add(4) + *mat1.add(2) * *mat2.add(7);
        *res.add(2) = *mat1.add(0) * *mat2.add(2) + *mat1.add(1) * *mat2.add(5) + *mat1.add(2) * *mat2.add(8);
        *res.add(3) = *mat1.add(3) * *mat2.add(0) + *mat1.add(4) * *mat2.add(3) + *mat1.add(5) * *mat2.add(6);
        *res.add(4) = *mat1.add(3) * *mat2.add(1) + *mat1.add(4) * *mat2.add(4) + *mat1.add(5) * *mat2.add(7);
        *res.add(5) = *mat1.add(3) * *mat2.add(2) + *mat1.add(4) * *mat2.add(5) + *mat1.add(5) * *mat2.add(8);
        *res.add(6) = *mat1.add(6) * *mat2.add(0) + *mat1.add(7) * *mat2.add(3) + *mat1.add(8) * *mat2.add(6);
        *res.add(7) = *mat1.add(6) * *mat2.add(1) + *mat1.add(7) * *mat2.add(4) + *mat1.add(8) * *mat2.add(7);
        *res.add(8) = *mat1.add(6) * *mat2.add(2) + *mat1.add(7) * *mat2.add(5) + *mat1.add(8) * *mat2.add(8);
    }
}

/// C: mji_mulMatTMat3 (engine/engine_inline.h:184)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_mul_mat_t_mat3(res: *mut f64, mat1: *const f64, mat2: *const f64) {
    // SAFETY: caller guarantees res[9], mat1[9], mat2[9] are valid, no aliasing
    unsafe {
        *res.add(0) = *mat1.add(0) * *mat2.add(0) + *mat1.add(3) * *mat2.add(3) + *mat1.add(6) * *mat2.add(6);
        *res.add(1) = *mat1.add(0) * *mat2.add(1) + *mat1.add(3) * *mat2.add(4) + *mat1.add(6) * *mat2.add(7);
        *res.add(2) = *mat1.add(0) * *mat2.add(2) + *mat1.add(3) * *mat2.add(5) + *mat1.add(6) * *mat2.add(8);
        *res.add(3) = *mat1.add(1) * *mat2.add(0) + *mat1.add(4) * *mat2.add(3) + *mat1.add(7) * *mat2.add(6);
        *res.add(4) = *mat1.add(1) * *mat2.add(1) + *mat1.add(4) * *mat2.add(4) + *mat1.add(7) * *mat2.add(7);
        *res.add(5) = *mat1.add(1) * *mat2.add(2) + *mat1.add(4) * *mat2.add(5) + *mat1.add(7) * *mat2.add(8);
        *res.add(6) = *mat1.add(2) * *mat2.add(0) + *mat1.add(5) * *mat2.add(3) + *mat1.add(8) * *mat2.add(6);
        *res.add(7) = *mat1.add(2) * *mat2.add(1) + *mat1.add(5) * *mat2.add(4) + *mat1.add(8) * *mat2.add(7);
        *res.add(8) = *mat1.add(2) * *mat2.add(2) + *mat1.add(5) * *mat2.add(5) + *mat1.add(8) * *mat2.add(8);
    }
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
    // SAFETY: caller guarantees res[4] and data[4] are valid
    unsafe {
        *res.add(0) = *data.add(0);
        *res.add(1) = *data.add(1);
        *res.add(2) = *data.add(2);
        *res.add(3) = *data.add(3);
    }
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
/// Calls: mji_copy3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_rot_vec_quat(res: *mut f64, vec: *const f64, quat: *const f64) {
    // SAFETY: caller guarantees res[3], vec[3], quat[4] are valid, res != quat
    unsafe {
        // null quat: copy vec
        if *quat.add(0) == 1.0 && *quat.add(1) == 0.0 && *quat.add(2) == 0.0 && *quat.add(3) == 0.0 {
            mji_copy3(res, vec);
        }
        // regular processing
        else {
            // tmp = q_w * v + cross(q_xyz, v)
            let tmp0 = *quat.add(0) * *vec.add(0) + *quat.add(2) * *vec.add(2) - *quat.add(3) * *vec.add(1);
            let tmp1 = *quat.add(0) * *vec.add(1) + *quat.add(3) * *vec.add(0) - *quat.add(1) * *vec.add(2);
            let tmp2 = *quat.add(0) * *vec.add(2) + *quat.add(1) * *vec.add(1) - *quat.add(2) * *vec.add(0);
            // res = v + 2 * cross(q_xyz, t)
            *res.add(0) = *vec.add(0) + 2.0 * (*quat.add(2) * tmp2 - *quat.add(3) * tmp1);
            *res.add(1) = *vec.add(1) + 2.0 * (*quat.add(3) * tmp0 - *quat.add(1) * tmp2);
            *res.add(2) = *vec.add(2) + 2.0 * (*quat.add(1) * tmp1 - *quat.add(2) * tmp0);
        }
    }
}

/// C: mji_negQuat (engine/engine_inline.h:277)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_neg_quat(res: *mut f64, quat: *const f64) {
    // SAFETY: caller guarantees res[4] and quat[4] are valid
    unsafe {
        *res.add(0) = *quat.add(0);
        *res.add(1) = -*quat.add(1);
        *res.add(2) = -*quat.add(2);
        *res.add(3) = -*quat.add(3);
    }
}

/// C: mji_mulQuat (engine/engine_inline.h:287)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_mul_quat(res: *mut f64, qa: *const f64, qb: *const f64) {
    // SAFETY: caller guarantees res[4], qa[4], qb[4] are valid, no aliasing
    unsafe {
        *res.add(0) = *qa.add(0) * *qb.add(0) - *qa.add(1) * *qb.add(1) - *qa.add(2) * *qb.add(2) - *qa.add(3) * *qb.add(3);
        *res.add(1) = *qa.add(0) * *qb.add(1) + *qa.add(1) * *qb.add(0) + *qa.add(2) * *qb.add(3) - *qa.add(3) * *qb.add(2);
        *res.add(2) = *qa.add(0) * *qb.add(2) - *qa.add(1) * *qb.add(3) + *qa.add(2) * *qb.add(0) + *qa.add(3) * *qb.add(1);
        *res.add(3) = *qa.add(0) * *qb.add(3) + *qa.add(1) * *qb.add(2) - *qa.add(2) * *qb.add(1) + *qa.add(3) * *qb.add(0);
    }
}

/// C: mji_mulQuatAxis (engine/engine_inline.h:298)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_mul_quat_axis(res: *mut f64, quat: *const f64, axis: *const f64) {
    // SAFETY: caller guarantees res[4], quat[4], axis[3] are valid, no aliasing
    unsafe {
        *res.add(0) = -*quat.add(1) * *axis.add(0) - *quat.add(2) * *axis.add(1) - *quat.add(3) * *axis.add(2);
        *res.add(1) =  *quat.add(0) * *axis.add(0) + *quat.add(2) * *axis.add(2) - *quat.add(3) * *axis.add(1);
        *res.add(2) =  *quat.add(0) * *axis.add(1) + *quat.add(3) * *axis.add(0) - *quat.add(1) * *axis.add(2);
        *res.add(3) =  *quat.add(0) * *axis.add(2) + *quat.add(1) * *axis.add(1) - *quat.add(2) * *axis.add(0);
    }
}

/// C: mji_axisAngle2Quat (engine/engine_inline.h:309)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_axis_angle2quat(res: *mut f64, axis: *const f64, angle: f64) {
    // SAFETY: caller guarantees res[4] and axis[3] are valid
    unsafe {
        if angle == 0.0 {
            *res.add(0) = 1.0;
            *res.add(1) = 0.0;
            *res.add(2) = 0.0;
            *res.add(3) = 0.0;
        } else {
            let s = (angle * 0.5).sin();
            *res.add(0) = (angle * 0.5).cos();
            *res.add(1) = *axis.add(0) * s;
            *res.add(2) = *axis.add(1) * s;
            *res.add(3) = *axis.add(2) * s;
        }
    }
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
    const MJ_PI: f64 = 3.14159265358979323846_f64;
    // SAFETY: caller guarantees res[3] and quat[4] are valid
    unsafe {
        let mut axis: [f64; 3] = [*quat.add(1), *quat.add(2), *quat.add(3)];
        let sin_a_2 = mji_normalize3(axis.as_mut_ptr());
        let mut speed = 2.0 * f64::atan2(sin_a_2, *quat.add(0));

        if speed > MJ_PI {
            speed -= 2.0 * MJ_PI;
        }
        speed /= dt;

        mji_scl3(res, axis.as_ptr(), speed);
    }
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
    // qdif = neg(qb)*qa
    let mut qneg: [f64; 4] = [0.0; 4];
    let mut qdif: [f64; 4] = [0.0; 4];
    mji_neg_quat(qneg.as_mut_ptr(), qb);
    mji_mul_quat(qdif.as_mut_ptr(), qneg.as_ptr(), qa);

    // convert to 3D velocity
    mji_quat2vel(res, qdif.as_ptr(), 1.0);
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
    // SAFETY: caller guarantees quat[4] and mat[9] are valid, no aliasing
    unsafe {
        // q0 largest
        if *mat.add(0) + *mat.add(4) + *mat.add(8) > 0.0 {
            *quat.add(0) = 0.5 * (1.0 + *mat.add(0) + *mat.add(4) + *mat.add(8)).sqrt();
            *quat.add(1) = 0.25 * (*mat.add(7) - *mat.add(5)) / *quat.add(0);
            *quat.add(2) = 0.25 * (*mat.add(2) - *mat.add(6)) / *quat.add(0);
            *quat.add(3) = 0.25 * (*mat.add(3) - *mat.add(1)) / *quat.add(0);
        }
        // q1 largest
        else if *mat.add(0) > *mat.add(4) && *mat.add(0) > *mat.add(8) {
            *quat.add(1) = 0.5 * (1.0 + *mat.add(0) - *mat.add(4) - *mat.add(8)).sqrt();
            *quat.add(0) = 0.25 * (*mat.add(7) - *mat.add(5)) / *quat.add(1);
            *quat.add(2) = 0.25 * (*mat.add(1) + *mat.add(3)) / *quat.add(1);
            *quat.add(3) = 0.25 * (*mat.add(2) + *mat.add(6)) / *quat.add(1);
        }
        // q2 largest
        else if *mat.add(4) > *mat.add(8) {
            *quat.add(2) = 0.5 * (1.0 - *mat.add(0) + *mat.add(4) - *mat.add(8)).sqrt();
            *quat.add(0) = 0.25 * (*mat.add(2) - *mat.add(6)) / *quat.add(2);
            *quat.add(1) = 0.25 * (*mat.add(1) + *mat.add(3)) / *quat.add(2);
            *quat.add(3) = 0.25 * (*mat.add(5) + *mat.add(7)) / *quat.add(2);
        }
        // q3 largest
        else {
            *quat.add(3) = 0.5 * (1.0 - *mat.add(0) - *mat.add(4) + *mat.add(8)).sqrt();
            *quat.add(0) = 0.25 * (*mat.add(3) - *mat.add(1)) / *quat.add(3);
            *quat.add(1) = 0.25 * (*mat.add(2) + *mat.add(6)) / *quat.add(3);
            *quat.add(2) = 0.25 * (*mat.add(5) + *mat.add(7)) / *quat.add(3);
        }
    }
    mji_normalize4(quat);
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
    // SAFETY: caller guarantees quat[4] and vel[3] are valid, no aliasing
    unsafe {
        let mut tmp: [f64; 4] = [0.0; 4];
        let mut qrot: [f64; 4] = [0.0; 4];

        // form local rotation quaternion, apply
        mji_copy3(tmp.as_mut_ptr(), vel);
        let angle = scale * mji_normalize3(tmp.as_mut_ptr());
        mji_axis_angle2quat(qrot.as_mut_ptr(), tmp.as_ptr(), angle);
        mji_normalize4(quat);
        mji_copy4(tmp.as_mut_ptr(), quat);
        mji_mul_quat(quat, tmp.as_ptr(), qrot.as_ptr());
    }
}

/// C: mji_cross (engine/engine_inline.h:419)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_cross(res: *mut f64, a: *const f64, b: *const f64) {
    // SAFETY: caller guarantees res[3], a[3], b[3] are valid, no aliasing
    unsafe {
        *res.add(0) = *a.add(1) * *b.add(2) - *a.add(2) * *b.add(1);
        *res.add(1) = *a.add(2) * *b.add(0) - *a.add(0) * *b.add(2);
        *res.add(2) = *a.add(0) * *b.add(1) - *a.add(1) * *b.add(0);
    }
}

/// C: mji_crossMotion (engine/engine_inline.h:429)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_cross_motion(res: *mut f64, vel: *const f64, v: *const f64) {
    // SAFETY: caller guarantees res[6], vel[6], v[6] are valid, no aliasing
    unsafe {
        *res.add(0) = -*vel.add(2) * *v.add(1) + *vel.add(1) * *v.add(2);
        *res.add(1) =  *vel.add(2) * *v.add(0) - *vel.add(0) * *v.add(2);
        *res.add(2) = -*vel.add(1) * *v.add(0) + *vel.add(0) * *v.add(1);
        *res.add(3) = -*vel.add(2) * *v.add(4) + *vel.add(1) * *v.add(5);
        *res.add(4) =  *vel.add(2) * *v.add(3) - *vel.add(0) * *v.add(5);
        *res.add(5) = -*vel.add(1) * *v.add(3) + *vel.add(0) * *v.add(4);

        *res.add(3) += -*vel.add(5) * *v.add(1) + *vel.add(4) * *v.add(2);
        *res.add(4) +=  *vel.add(5) * *v.add(0) - *vel.add(3) * *v.add(2);
        *res.add(5) += -*vel.add(4) * *v.add(0) + *vel.add(3) * *v.add(1);
    }
}

/// C: mji_crossForce (engine/engine_inline.h:446)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_cross_force(res: *mut f64, vel: *const f64, f: *const f64) {
    // SAFETY: caller guarantees res[6], vel[6], f[6] are valid, no aliasing
    unsafe {
        *res.add(0) = -*vel.add(2) * *f.add(1) + *vel.add(1) * *f.add(2);
        *res.add(1) =  *vel.add(2) * *f.add(0) - *vel.add(0) * *f.add(2);
        *res.add(2) = -*vel.add(1) * *f.add(0) + *vel.add(0) * *f.add(1);
        *res.add(3) = -*vel.add(2) * *f.add(4) + *vel.add(1) * *f.add(5);
        *res.add(4) =  *vel.add(2) * *f.add(3) - *vel.add(0) * *f.add(5);
        *res.add(5) = -*vel.add(1) * *f.add(3) + *vel.add(0) * *f.add(4);

        *res.add(0) += -*vel.add(5) * *f.add(4) + *vel.add(4) * *f.add(5);
        *res.add(1) +=  *vel.add(5) * *f.add(3) - *vel.add(3) * *f.add(5);
        *res.add(2) += -*vel.add(4) * *f.add(3) + *vel.add(3) * *f.add(4);
    }
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
    // SAFETY: caller guarantees res[6] and vec[6] are valid
    unsafe {
        *res.add(0) = *vec.add(0);
        *res.add(1) = *vec.add(1);
        *res.add(2) = *vec.add(2);
        *res.add(3) = *vec.add(3);
        *res.add(4) = *vec.add(4);
        *res.add(5) = *vec.add(5);
    }
}

/// C: mji_copy9 (engine/engine_inline.h:485)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_copy9(res: *mut f64, data: *const f64) {
    // SAFETY: caller guarantees res[9] and data[9] are valid
    unsafe {
        *res.add(0) = *data.add(0);
        *res.add(1) = *data.add(1);
        *res.add(2) = *data.add(2);
        *res.add(3) = *data.add(3);
        *res.add(4) = *data.add(4);
        *res.add(5) = *data.add(5);
        *res.add(6) = *data.add(6);
        *res.add(7) = *data.add(7);
        *res.add(8) = *data.add(8);
    }
}

