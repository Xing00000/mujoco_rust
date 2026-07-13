//! Port of: engine/engine_util_blas.h
//! IR hash: 6ff71909dacce27f
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mju_zero3 (engine/engine_util_blas.h:68)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_zero3(res: *mut f64) {
    // SAFETY: res points to at least 3 contiguous f64 elements (caller contract)
    unsafe {
        *res.add(0) = 0.0;
        *res.add(1) = 0.0;
        *res.add(2) = 0.0;
    }
}

/// C: mju_equal3 (engine/engine_util_blas.h:71)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_equal3(vec1: *const f64, vec2: *const f64) -> i32 {
    const MJ_MINVAL: f64 = 1E-15_f64;
    // SAFETY: caller guarantees vec1[3] and vec2[3] are valid
    unsafe {
        ((*vec1.add(0) - *vec2.add(0)).abs() < MJ_MINVAL &&
         (*vec1.add(1) - *vec2.add(1)).abs() < MJ_MINVAL &&
         (*vec1.add(2) - *vec2.add(2)).abs() < MJ_MINVAL) as i32
    }
}

/// C: mju_copy3 (engine/engine_util_blas.h:74)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_copy3(res: *mut f64, vec: *const f64) {
    // SAFETY: res points to at least 3 f64, vec points to at least 3 f64 (caller contract)
    unsafe {
        *res.add(0) = *vec.add(0);
        *res.add(1) = *vec.add(1);
        *res.add(2) = *vec.add(2);
    }
}

/// C: mju_copy9 (engine/engine_util_blas.h:77)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_copy9(res: *mut f64, mat: *const f64) {
    // SAFETY: res and mat each point to at least 9 f64 (caller contract)
    unsafe {
        *res.add(0) = *mat.add(0);
        *res.add(1) = *mat.add(1);
        *res.add(2) = *mat.add(2);
        *res.add(3) = *mat.add(3);
        *res.add(4) = *mat.add(4);
        *res.add(5) = *mat.add(5);
        *res.add(6) = *mat.add(6);
        *res.add(7) = *mat.add(7);
        *res.add(8) = *mat.add(8);
    }
}

/// C: mju_scl3 (engine/engine_util_blas.h:80)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_scl3(res: *mut f64, vec: *const f64, scl: f64) {
    // SAFETY: res points to at least 3 f64, vec points to at least 3 f64 (caller contract)
    unsafe {
        *res.add(0) = *vec.add(0) * scl;
        *res.add(1) = *vec.add(1) * scl;
        *res.add(2) = *vec.add(2) * scl;
    }
}

/// C: mju_add3 (engine/engine_util_blas.h:83)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_add3(res: *mut f64, vec1: *const f64, vec2: *const f64) {
    // SAFETY: res, vec1, vec2 each point to at least 3 f64 (caller contract)
    unsafe {
        *res.add(0) = *vec1.add(0) + *vec2.add(0);
        *res.add(1) = *vec1.add(1) + *vec2.add(1);
        *res.add(2) = *vec1.add(2) + *vec2.add(2);
    }
}

/// C: mju_sub3 (engine/engine_util_blas.h:86)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_sub3(res: *mut f64, vec1: *const f64, vec2: *const f64) {
    // SAFETY: res, vec1, vec2 each point to at least 3 f64 (caller contract)
    unsafe {
        *res.add(0) = *vec1.add(0) - *vec2.add(0);
        *res.add(1) = *vec1.add(1) - *vec2.add(1);
        *res.add(2) = *vec1.add(2) - *vec2.add(2);
    }
}

/// C: mju_addTo3 (engine/engine_util_blas.h:89)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_add_to3(res: *mut f64, vec: *const f64) {
    // SAFETY: res points to at least 3 f64, vec points to at least 3 f64 (caller contract)
    unsafe {
        *res.add(0) += *vec.add(0);
        *res.add(1) += *vec.add(1);
        *res.add(2) += *vec.add(2);
    }
}

/// C: mju_subFrom3 (engine/engine_util_blas.h:92)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_sub_from3(res: *mut f64, vec: *const f64) {
    // SAFETY: res points to at least 3 f64, vec points to at least 3 f64 (caller contract)
    unsafe {
        *res.add(0) -= *vec.add(0);
        *res.add(1) -= *vec.add(1);
        *res.add(2) -= *vec.add(2);
    }
}

/// C: mju_addToScl3 (engine/engine_util_blas.h:95)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_add_to_scl3(res: *mut f64, vec: *const f64, scl: f64) {
    // SAFETY: res and vec each point to at least 3 f64 (caller contract)
    unsafe {
        *res.add(0) += *vec.add(0) * scl;
        *res.add(1) += *vec.add(1) * scl;
        *res.add(2) += *vec.add(2) * scl;
    }
}

/// C: mju_addScl3 (engine/engine_util_blas.h:98)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_add_scl3(res: *mut f64, vec1: *const f64, vec2: *const f64, scl: f64) {
    // SAFETY: res, vec1, vec2 each point to at least 3 f64 (caller contract)
    unsafe {
        *res.add(0) = *vec1.add(0) + scl * *vec2.add(0);
        *res.add(1) = *vec1.add(1) + scl * *vec2.add(1);
        *res.add(2) = *vec1.add(2) + scl * *vec2.add(2);
    }
}

/// C: mju_normalize3 (engine/engine_util_blas.h:101)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_normalize3(vec: *mut f64) -> f64 {
    const MJ_MINVAL: f64 = 1E-15_f64;

    // SAFETY: vec points to at least 3 f64 (caller contract)
    unsafe {
        let norm: f64 = f64::sqrt(
            *vec.add(0) * *vec.add(0)
            + *vec.add(1) * *vec.add(1)
            + *vec.add(2) * *vec.add(2)
        );

        if norm < MJ_MINVAL {
            *vec.add(0) = 1.0;
            *vec.add(1) = 0.0;
            *vec.add(2) = 0.0;
        } else {
            let inv_norm: f64 = 1.0 / norm;
            *vec.add(0) *= inv_norm;
            *vec.add(1) *= inv_norm;
            *vec.add(2) *= inv_norm;
        }

        norm
    }
}

/// C: mju_norm3 (engine/engine_util_blas.h:104)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_norm3(vec: *const f64) -> f64 {
    // SAFETY: vec points to at least 3 f64 (caller contract)
    unsafe {
        f64::sqrt(
            *vec.add(0) * *vec.add(0)
            + *vec.add(1) * *vec.add(1)
            + *vec.add(2) * *vec.add(2)
        )
    }
}

/// C: mju_dot3 (engine/engine_util_blas.h:107)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_dot3(vec1: *const f64, vec2: *const f64) -> f64 {
    // SAFETY: vec1, vec2 each point to at least 3 f64 (caller contract)
    unsafe {
        *vec1.add(0) * *vec2.add(0)
        + *vec1.add(1) * *vec2.add(1)
        + *vec1.add(2) * *vec2.add(2)
    }
}

/// C: mju_dist3 (engine/engine_util_blas.h:110)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_dist3(pos1: *const f64, pos2: *const f64) -> f64 {
    // SAFETY: pos1, pos2 each point to at least 3 f64 (caller contract)
    unsafe {
        let dif0: f64 = *pos1.add(0) - *pos2.add(0);
        let dif1: f64 = *pos1.add(1) - *pos2.add(1);
        let dif2: f64 = *pos1.add(2) - *pos2.add(2);

        f64::sqrt(dif0 * dif0 + dif1 * dif1 + dif2 * dif2)
    }
}

/// C: mju_mulMatVec3 (engine/engine_util_blas.h:113)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_mul_mat_vec3(res: *mut f64, mat: *const f64, vec: *const f64) {
    // SAFETY: res points to 3 f64, mat points to 9 f64, vec points to 3 f64 (caller contract)
    // Use tmp to handle aliasing (res may alias vec)
    unsafe {
        let tmp0: f64 = *mat.add(0) * *vec.add(0) + *mat.add(1) * *vec.add(1) + *mat.add(2) * *vec.add(2);
        let tmp1: f64 = *mat.add(3) * *vec.add(0) + *mat.add(4) * *vec.add(1) + *mat.add(5) * *vec.add(2);
        let tmp2: f64 = *mat.add(6) * *vec.add(0) + *mat.add(7) * *vec.add(1) + *mat.add(8) * *vec.add(2);

        *res.add(0) = tmp0;
        *res.add(1) = tmp1;
        *res.add(2) = tmp2;
    }
}

/// C: mju_mulMatTVec3 (engine/engine_util_blas.h:116)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_mul_mat_t_vec3(res: *mut f64, mat: *const f64, vec: *const f64) {
    // SAFETY: res points to 3 f64, mat points to 9 f64, vec points to 3 f64 (caller contract)
    // Use tmp to handle aliasing (res may alias vec)
    unsafe {
        let tmp0: f64 = *mat.add(0) * *vec.add(0) + *mat.add(3) * *vec.add(1) + *mat.add(6) * *vec.add(2);
        let tmp1: f64 = *mat.add(1) * *vec.add(0) + *mat.add(4) * *vec.add(1) + *mat.add(7) * *vec.add(2);
        let tmp2: f64 = *mat.add(2) * *vec.add(0) + *mat.add(5) * *vec.add(1) + *mat.add(8) * *vec.add(2);

        *res.add(0) = tmp0;
        *res.add(1) = tmp1;
        *res.add(2) = tmp2;
    }
}

/// C: mju_mulMatMat3 (engine/engine_util_blas.h:119)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_mul_mat_mat3(res: *mut f64, mat1: *const f64, mat2: *const f64) {
    // SAFETY: caller guarantees res[9], mat1[9], mat2[9] are valid
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

/// C: mju_mulMatTMat3 (engine/engine_util_blas.h:122)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_mul_mat_t_mat3(res: *mut f64, mat1: *const f64, mat2: *const f64) {
    // SAFETY: caller guarantees res[9], mat1[9], mat2[9] are valid
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

/// C: mju_mulMatMatT3 (engine/engine_util_blas.h:125)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_mul_mat_mat_t3(res: *mut f64, mat1: *const f64, mat2: *const f64) {
    // SAFETY: caller guarantees res[9], mat1[9], mat2[9] are valid
    unsafe {
        *res.add(0) = *mat1.add(0) * *mat2.add(0) + *mat1.add(1) * *mat2.add(1) + *mat1.add(2) * *mat2.add(2);
        *res.add(1) = *mat1.add(0) * *mat2.add(3) + *mat1.add(1) * *mat2.add(4) + *mat1.add(2) * *mat2.add(5);
        *res.add(2) = *mat1.add(0) * *mat2.add(6) + *mat1.add(1) * *mat2.add(7) + *mat1.add(2) * *mat2.add(8);
        *res.add(3) = *mat1.add(3) * *mat2.add(0) + *mat1.add(4) * *mat2.add(1) + *mat1.add(5) * *mat2.add(2);
        *res.add(4) = *mat1.add(3) * *mat2.add(3) + *mat1.add(4) * *mat2.add(4) + *mat1.add(5) * *mat2.add(5);
        *res.add(5) = *mat1.add(3) * *mat2.add(6) + *mat1.add(4) * *mat2.add(7) + *mat1.add(5) * *mat2.add(8);
        *res.add(6) = *mat1.add(6) * *mat2.add(0) + *mat1.add(7) * *mat2.add(1) + *mat1.add(8) * *mat2.add(2);
        *res.add(7) = *mat1.add(6) * *mat2.add(3) + *mat1.add(7) * *mat2.add(4) + *mat1.add(8) * *mat2.add(5);
        *res.add(8) = *mat1.add(6) * *mat2.add(6) + *mat1.add(7) * *mat2.add(7) + *mat1.add(8) * *mat2.add(8);
    }
}

/// C: mju_zero4 (engine/engine_util_blas.h:130)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_zero4(res: *mut f64) {
    // SAFETY: res points to at least 4 f64 (caller contract)
    unsafe {
        *res.add(0) = 0.0;
        *res.add(1) = 0.0;
        *res.add(2) = 0.0;
        *res.add(3) = 0.0;
    }
}

/// C: mju_unit4 (engine/engine_util_blas.h:133)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_unit4(res: *mut f64) {
    // SAFETY: caller guarantees res[4] is valid
    unsafe {
        *res.add(0) = 1.0;
        *res.add(1) = 0.0;
        *res.add(2) = 0.0;
        *res.add(3) = 0.0;
    }
}

/// C: mju_copy4 (engine/engine_util_blas.h:136)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_copy4(res: *mut f64, data: *const f64) {
    // SAFETY: res and data each point to at least 4 f64 (caller contract)
    unsafe {
        *res.add(0) = *data.add(0);
        *res.add(1) = *data.add(1);
        *res.add(2) = *data.add(2);
        *res.add(3) = *data.add(3);
    }
}

/// C: mju_normalize4 (engine/engine_util_blas.h:139)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_normalize4(vec: *mut f64) -> f64 {
    const MJ_MINVAL: f64 = 1E-15_f64;

    // SAFETY: caller guarantees vec points to at least 4 f64
    unsafe {
        let norm: f64 = f64::sqrt(
            *vec.add(0) * *vec.add(0)
            + *vec.add(1) * *vec.add(1)
            + *vec.add(2) * *vec.add(2)
            + *vec.add(3) * *vec.add(3)
        );

        if norm < MJ_MINVAL {
            *vec.add(0) = 1.0;
            *vec.add(1) = 0.0;
            *vec.add(2) = 0.0;
            *vec.add(3) = 0.0;
        } else {
            let inv = 1.0 / norm;
            *vec.add(0) *= inv;
            *vec.add(1) *= inv;
            *vec.add(2) *= inv;
            *vec.add(3) *= inv;
        }

        norm
    }
}

/// C: mju_zero (engine/engine_util_blas.h:145)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_zero(res: *mut f64, n: i32) {
    // SAFETY: caller guarantees res points to valid array of at least n f64
    unsafe { std::ptr::write_bytes(res, 0, n as usize); }
}

/// C: mju_zeroInd (engine/engine_util_blas.h:148)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_zero_ind(res: *mut f64, n: i32, ind: *const i32) {
    // SAFETY: caller guarantees res and ind valid, ind[i] in bounds of res
    unsafe {
        for i in 0..n as usize {
            *res.add(*ind.add(i) as usize) = 0.0;
        }
    }
}

/// C: mju_fill (engine/engine_util_blas.h:151)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_fill(res: *mut f64, val: f64, n: i32) {
    // SAFETY: caller guarantees res points to valid array of at least n f64
    unsafe {
        for i in 0..n as usize {
            *res.add(i) = val;
        }
    }
}

/// C: mju_copy (engine/engine_util_blas.h:154)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_copy(res: *mut f64, vec: *const f64, n: i32) {
    // SAFETY: caller guarantees res and vec point to valid non-overlapping arrays of at least n f64
    unsafe { std::ptr::copy_nonoverlapping(vec, res, n as usize); }
}

/// C: mju_copyInd (engine/engine_util_blas.h:157)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_copy_ind(res: *mut f64, vec: *const f64, ind: *const i32, n: i32) {
    // SAFETY: caller guarantees res, vec, ind point to valid memory with sufficient length
    unsafe {
        for i in 0..n as usize {
            let j = *ind.add(i) as usize;
            *res.add(j) = *vec.add(j);
        }
    }
}

/// C: mju_sum (engine/engine_util_blas.h:160)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_sum(vec: *const f64, n: i32) -> f64 {
    // SAFETY: caller guarantees vec points to valid array of at least n f64
    unsafe {
        let mut res: f64 = 0.0;
        for i in 0..n as usize {
            res += *vec.add(i);
        }
        res
    }
}

/// C: mju_L1 (engine/engine_util_blas.h:163)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_l1(vec: *const f64, n: i32) -> f64 {
    // SAFETY: caller guarantees vec points to valid array of at least n f64
    unsafe {
        let mut res: f64 = 0.0;
        for i in 0..n as usize {
            res += (*vec.add(i)).abs();
        }
        res
    }
}

/// C: mju_scl (engine/engine_util_blas.h:166)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_scl(res: *mut f64, vec: *const f64, scl: f64, n: i32) {
    // SAFETY: caller guarantees res and vec point to valid arrays of at least n f64
    unsafe {
        for i in 0..n as usize {
            *res.add(i) = *vec.add(i) * scl;
        }
    }
}

/// C: mju_add (engine/engine_util_blas.h:169)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_add(res: *mut f64, vec1: *const f64, vec2: *const f64, n: i32) {
    // SAFETY: caller guarantees res, vec1, vec2 point to valid arrays of at least n f64
    unsafe {
        for i in 0..n as usize {
            *res.add(i) = *vec1.add(i) + *vec2.add(i);
        }
    }
}

/// C: mju_addInd (engine/engine_util_blas.h:172)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_add_ind(res: *mut f64, vec1: *const f64, vec2: *const f64, ind: *const i32, n: i32) {
    // SAFETY: caller guarantees all pointers valid, ind[i] in bounds of res/vec1/vec2
    unsafe {
        for i in 0..n as usize {
            let j = *ind.add(i) as usize;
            *res.add(j) = *vec1.add(j) + *vec2.add(j);
        }
    }
}

/// C: mju_sub (engine/engine_util_blas.h:175)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_sub(res: *mut f64, vec1: *const f64, vec2: *const f64, n: i32) {
    // SAFETY: caller guarantees res, vec1, vec2 point to valid arrays of at least n f64
    unsafe {
        for i in 0..n as usize {
            *res.add(i) = *vec1.add(i) - *vec2.add(i);
        }
    }
}

/// C: mju_subInd (engine/engine_util_blas.h:178)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_sub_ind(res: *mut f64, vec1: *const f64, vec2: *const f64, ind: *const i32, n: i32) {
    // SAFETY: caller guarantees res, vec1, vec2, ind point to valid memory with sufficient length
    unsafe {
        for i in 0..n as usize {
            let j = *ind.add(i) as usize;
            *res.add(j) = *vec1.add(j) - *vec2.add(j);
        }
    }
}

/// C: mju_addTo (engine/engine_util_blas.h:181)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_add_to(res: *mut f64, vec: *const f64, n: i32) {
    // SAFETY: caller guarantees res and vec point to valid arrays of at least n f64
    unsafe {
        for i in 0..n as usize {
            *res.add(i) += *vec.add(i);
        }
    }
}

/// C: mju_addToInd (engine/engine_util_blas.h:184)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_add_to_ind(res: *mut f64, vec: *const f64, ind: *const i32, n: i32) {
    // SAFETY: caller guarantees res, vec, ind point to valid memory with sufficient length
    unsafe {
        for i in 0..n as usize {
            let j = *ind.add(i) as usize;
            *res.add(j) += *vec.add(j);
        }
    }
}

/// C: mju_subFrom (engine/engine_util_blas.h:187)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_sub_from(res: *mut f64, vec: *const f64, n: i32) {
    // SAFETY: caller guarantees res and vec point to valid arrays of at least n f64
    unsafe {
        for i in 0..n as usize {
            *res.add(i) -= *vec.add(i);
        }
    }
}

/// C: mju_addToScl (engine/engine_util_blas.h:190)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_add_to_scl(res: *mut f64, vec: *const f64, scl: f64, n: i32) {
    // SAFETY: caller guarantees res and vec point to valid arrays of at least n f64
    unsafe {
        for i in 0..n as usize {
            *res.add(i) += *vec.add(i) * scl;
        }
    }
}

/// C: mju_addToSclInd (engine/engine_util_blas.h:193)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_add_to_scl_ind(res: *mut f64, vec: *const f64, ind: *const i32, scl: f64, n: i32) {
    // SAFETY: caller guarantees all pointers valid, ind[i] in bounds of res/vec
    unsafe {
        for i in 0..n as usize {
            let k = *ind.add(i) as usize;
            *res.add(k) += *vec.add(k) * scl;
        }
    }
}

/// C: mju_addScl (engine/engine_util_blas.h:196)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_add_scl(res: *mut f64, vec1: *const f64, vec2: *const f64, scl: f64, n: i32) {
    // SAFETY: caller guarantees res, vec1, vec2 point to valid arrays of at least n f64
    unsafe {
        for i in 0..n as usize {
            *res.add(i) = *vec1.add(i) + *vec2.add(i) * scl;
        }
    }
}

/// C: mju_normalize (engine/engine_util_blas.h:199)
/// Calls: mju_dot, mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_normalize(res: *mut f64, n: i32) -> f64 {
    const MJ_MINVAL: f64 = 1E-15_f64;

    // SAFETY: res points to at least n f64 (caller contract)
    unsafe {
        let norm: f64 = f64::sqrt(mju_dot(res, res, n));

        if norm < MJ_MINVAL {
            *res.add(0) = 1.0;
            mju_zero(res.add(1), n - 1);
        } else {
            let norm_inv: f64 = 1.0 / norm;
            for i in 0..n as usize {
                *res.add(i) *= norm_inv;
            }
        }

        norm
    }
}

/// C: mju_norm (engine/engine_util_blas.h:202)
/// Calls: mju_dot
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_norm(res: *const f64, n: i32) -> f64 {
    // SAFETY: res points to at least n f64 (caller contract)
    f64::sqrt(mju_dot(res, res, n))
}

/// C: mju_dot (engine/engine_util_blas.h:205)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_dot(vec1: *const f64, vec2: *const f64, n: i32) -> f64 {
    // SAFETY: caller guarantees vec1 and vec2 point to valid arrays of at least n f64
    unsafe {
        let mut i: i32 = 0;
        let n_4 = n - 4;
        let (mut res0, mut res1, mut res2, mut res3) = (0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64);
        while i <= n_4 {
            res0 += *vec1.add(i as usize) * *vec2.add(i as usize);
            res1 += *vec1.add((i + 1) as usize) * *vec2.add((i + 1) as usize);
            res2 += *vec1.add((i + 2) as usize) * *vec2.add((i + 2) as usize);
            res3 += *vec1.add((i + 3) as usize) * *vec2.add((i + 3) as usize);
            i += 4;
        }
        let mut res = (res0 + res2) + (res1 + res3);
        let n_i = n - i;
        if n_i == 3 {
            res += *vec1.add(i as usize) * *vec2.add(i as usize)
                + *vec1.add((i + 1) as usize) * *vec2.add((i + 1) as usize)
                + *vec1.add((i + 2) as usize) * *vec2.add((i + 2) as usize);
        } else if n_i == 2 {
            res += *vec1.add(i as usize) * *vec2.add(i as usize)
                + *vec1.add((i + 1) as usize) * *vec2.add((i + 1) as usize);
        } else if n_i == 1 {
            res += *vec1.add(i as usize) * *vec2.add(i as usize);
        }
        res
    }
}

/// C: mju_dotInd (engine/engine_util_blas.h:208)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_dot_ind(vec1: *const f64, vec2: *const f64, ind: *const i32, n: i32) -> f64 {
    // SAFETY: caller guarantees vec1, vec2, ind valid; ind[i] in bounds
    unsafe {
        let mut res: f64 = 0.0;
        for i in 0..n as usize {
            let k = *ind.add(i) as usize;
            res += *vec1.add(k) * *vec2.add(k);
        }
        res
    }
}

/// C: mju_mulMatVec (engine/engine_util_blas.h:213)
/// Calls: mju_dot
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_mul_mat_vec(res: *mut f64, mat: *const f64, vec: *const f64, nr: i32, nc: i32) {
    // SAFETY: caller guarantees res[nr], mat[nr*nc], vec[nc] are valid
    unsafe {
        for r in 0..nr as usize {
            *res.add(r) = mju_dot(mat.add(r * nc as usize), vec, nc);
        }
    }
}

/// C: mju_mulMatTVec (engine/engine_util_blas.h:216)
/// Calls: mju_addToScl, mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_mul_mat_t_vec(res: *mut f64, mat: *const f64, vec: *const f64, nr: i32, nc: i32) {
    // SAFETY: res points to nc f64, mat points to nr*nc f64, vec points to nr f64 (caller contract)
    unsafe {
        mju_zero(res, nc);
        for r in 0..nr as usize {
            let tmp = *vec.add(r);
            if tmp != 0.0 {
                mju_add_to_scl(res, mat.add(r * nc as usize), tmp, nc);
            }
        }
    }
}

/// C: mju_mulVecMatVec (engine/engine_util_blas.h:219)
/// Calls: mju_dot
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_mul_vec_mat_vec(vec1: *const f64, mat: *const f64, vec2: *const f64, n: i32) -> f64 {
    // SAFETY: vec1, vec2 point to n f64, mat points to n*n f64 (caller contract)
    unsafe {
        let mut res: f64 = 0.0;
        for i in 0..n as usize {
            res += *vec1.add(i) * mju_dot(mat.add(i * n as usize), vec2, n);
        }
        res
    }
}

/// C: mju_transpose (engine/engine_util_blas.h:225)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_transpose(res: *mut f64, mat: *const f64, nr: i32, nc: i32) {
    // SAFETY: caller guarantees res[nc*nr] and mat[nr*nc] are valid
    unsafe {
        for i in 0..nr as usize {
            for j in 0..nc as usize {
                *res.add(j * nr as usize + i) = *mat.add(i * nc as usize + j);
            }
        }
    }
}

/// C: mju_symmetrize (engine/engine_util_blas.h:228)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_symmetrize(res: *mut f64, mat: *const f64, n: i32) {
    // SAFETY: caller guarantees res[n*n] and mat[n*n] are valid
    unsafe {
        for i in 0..n as usize {
            *res.add(i * (n as usize + 1)) = *mat.add(i * (n as usize + 1));
            for j in 0..i {
                let val = 0.5 * (*mat.add(i * n as usize + j) + *mat.add(j * n as usize + i));
                *res.add(i * n as usize + j) = val;
                *res.add(j * n as usize + i) = val;
            }
        }
    }
}

/// C: mju_eye (engine/engine_util_blas.h:231)
/// Calls: mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_eye(mat: *mut f64, n: i32) {
    // SAFETY: caller guarantees mat[n*n] is valid
    unsafe {
        mju_zero(mat, n * n);
        for i in 0..n as usize {
            *mat.add(i * (n as usize + 1)) = 1.0;
        }
    }
}

/// C: mju_copyRows (engine/engine_util_blas.h:234)
/// Calls: mju_copy
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_copy_rows(res: *mut f64, mat: *const f64, ind: *const i32, n: i32, nc: i32) {
    // SAFETY: caller guarantees res, mat have sufficient rows, ind[n] valid
    unsafe {
        for i in 0..n as usize {
            let row = *ind.add(i) as usize;
            mju_copy(res.add(nc as usize * row), mat.add(nc as usize * row), nc);
        }
    }
}

/// C: mju_mulMatMat (engine/engine_util_blas.h:239)
/// Calls: mju_addToScl, mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_mul_mat_mat(res: *mut f64, mat1: *const f64, mat2: *const f64, r1: i32, c1: i32, c2: i32) {
    // SAFETY: res points to r1*c2 f64, mat1 to r1*c1 f64, mat2 to c1*c2 f64 (caller contract)
    unsafe {
        mju_zero(res, r1 * c2);
        for i in 0..r1 as usize {
            for k in 0..c1 as usize {
                let tmp = *mat1.add(i * c1 as usize + k);
                if tmp != 0.0 {
                    mju_add_to_scl(res.add(i * c2 as usize), mat2.add(k * c2 as usize), tmp, c2);
                }
            }
        }
    }
}

/// C: mju_mulMatMatT (engine/engine_util_blas.h:243)
/// Calls: mju_dot
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_mul_mat_mat_t(res: *mut f64, mat1: *const f64, mat2: *const f64, r1: i32, c1: i32, r2: i32) {
    // SAFETY: caller guarantees res[r1*r2], mat1[r1*c1], mat2[r2*c1] are valid
    unsafe {
        for i in 0..r1 as usize {
            for j in 0..r2 as usize {
                *res.add(i * r2 as usize + j) = mju_dot(mat1.add(i * c1 as usize), mat2.add(j * c1 as usize), c1);
            }
        }
    }
}

/// C: mju_mulMatTMat (engine/engine_util_blas.h:247)
/// Calls: mju_addToScl, mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_mul_mat_t_mat(res: *mut f64, mat1: *const f64, mat2: *const f64, r1: i32, c1: i32, c2: i32) {
    // SAFETY: caller guarantees res[c1*c2], mat1[r1*c1], mat2[r1*c2] are valid
    unsafe {
        mju_zero(res, c1 * c2);
        for i in 0..r1 as usize {
            for j in 0..c1 as usize {
                let tmp = *mat1.add(i * c1 as usize + j);
                if tmp != 0.0 {
                    mju_add_to_scl(res.add(j * c2 as usize), mat2.add(i * c2 as usize), tmp, c2);
                }
            }
        }
    }
}

/// C: mju_sqrMatTD_impl (engine/engine_util_blas.h:251)
/// Calls: mju_addToScl, mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_sqr_mat_td_impl(res: *mut f64, mat: *const f64, diag: *const f64, nr: i32, nc: i32, flg_upper: i32) {
    // SAFETY: caller guarantees res[nc*nc], mat[nr*nc], diag[nr] (if non-null) are valid
    unsafe {
        mju_zero(res, nc * nc);
        if !diag.is_null() {
            for j in 0..nr as usize {
                if *diag.add(j) != 0.0 {
                    for i in 0..nc as usize {
                        let tmp = *mat.add(j * nc as usize + i);
                        if tmp != 0.0 {
                            mju_add_to_scl(res.add(i * nc as usize), mat.add(j * nc as usize), tmp * *diag.add(j), (i as i32) + 1);
                        }
                    }
                }
            }
        } else {
            for i in 0..nc as usize {
                for j in 0..nr as usize {
                    let tmp = *mat.add(j * nc as usize + i);
                    if tmp != 0.0 {
                        mju_add_to_scl(res.add(i * nc as usize), mat.add(j * nc as usize), tmp, (i as i32) + 1);
                    }
                }
            }
        }

        // flg_upper is set: make symmetric
        if flg_upper != 0 {
            for i in 0..nc as usize {
                for j in (i + 1)..nc as usize {
                    *res.add(i * nc as usize + j) = *res.add(j * nc as usize + i);
                }
            }
        }
    }
}

/// C: mju_sqrMatTD (engine/engine_util_blas.h:255)
/// Calls: mju_sqrMatTD_impl
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_sqr_mat_td(res: *mut f64, mat: *const f64, diag: *const f64, nr: i32, nc: i32) {
    mju_sqr_mat_td_impl(res, mat, diag, nr, nc, 1);
}

