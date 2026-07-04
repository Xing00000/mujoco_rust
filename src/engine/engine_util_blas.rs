//! Port of: engine/engine_util_blas.h
//! IR hash: 1b139f44af8230f9
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
    unsafe {
        *res.add(0) = 0.0; // C: res[0] = 0
        *res.add(1) = 0.0; // C: res[1] = 0
        *res.add(2) = 0.0; // C: res[2] = 0
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
    const MJMINVAL: f64 = 1e-15;
    unsafe {
        // C: return mju_abs(vec1[0] - vec2[0]) < mjMINVAL &&
        //         mju_abs(vec1[1] - vec2[1]) < mjMINVAL &&
        //         mju_abs(vec1[2] - vec2[2]) < mjMINVAL;
        ((*vec1.add(0) - *vec2.add(0)).abs() < MJMINVAL && // C: mju_abs(vec1[0] - vec2[0]) < mjMINVAL
         (*vec1.add(1) - *vec2.add(1)).abs() < MJMINVAL && // C: mju_abs(vec1[1] - vec2[1]) < mjMINVAL
         (*vec1.add(2) - *vec2.add(2)).abs() < MJMINVAL    // C: mju_abs(vec1[2] - vec2[2]) < mjMINVAL
        ) as i32
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
    unsafe {
        *res.add(0) = *vec.add(0); // C: res[0] = vec[0]
        *res.add(1) = *vec.add(1); // C: res[1] = vec[1]
        *res.add(2) = *vec.add(2); // C: res[2] = vec[2]
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
    unsafe {
        *res.add(0) = *mat.add(0); // C: res[0] = mat[0]
        *res.add(1) = *mat.add(1); // C: res[1] = mat[1]
        *res.add(2) = *mat.add(2); // C: res[2] = mat[2]
        *res.add(3) = *mat.add(3); // C: res[3] = mat[3]
        *res.add(4) = *mat.add(4); // C: res[4] = mat[4]
        *res.add(5) = *mat.add(5); // C: res[5] = mat[5]
        *res.add(6) = *mat.add(6); // C: res[6] = mat[6]
        *res.add(7) = *mat.add(7); // C: res[7] = mat[7]
        *res.add(8) = *mat.add(8); // C: res[8] = mat[8]
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
    unsafe {
        *res.add(0) = *vec.add(0) * scl; // C: res[0] = vec[0] * scl
        *res.add(1) = *vec.add(1) * scl; // C: res[1] = vec[1] * scl
        *res.add(2) = *vec.add(2) * scl; // C: res[2] = vec[2] * scl
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
    unsafe {
        *res.add(0) = *vec1.add(0) + *vec2.add(0); // C: res[0] = vec1[0] + vec2[0]
        *res.add(1) = *vec1.add(1) + *vec2.add(1); // C: res[1] = vec1[1] + vec2[1]
        *res.add(2) = *vec1.add(2) + *vec2.add(2); // C: res[2] = vec1[2] + vec2[2]
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
    unsafe {
        *res.add(0) = *vec1.add(0) - *vec2.add(0); // C: res[0] = vec1[0] - vec2[0]
        *res.add(1) = *vec1.add(1) - *vec2.add(1); // C: res[1] = vec1[1] - vec2[1]
        *res.add(2) = *vec1.add(2) - *vec2.add(2); // C: res[2] = vec1[2] - vec2[2]
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
    unsafe {
        *res.add(0) += *vec.add(0); // C: res[0] += vec[0]
        *res.add(1) += *vec.add(1); // C: res[1] += vec[1]
        *res.add(2) += *vec.add(2); // C: res[2] += vec[2]
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
    unsafe {
        *res.add(0) -= *vec.add(0); // C: res[0] -= vec[0]
        *res.add(1) -= *vec.add(1); // C: res[1] -= vec[1]
        *res.add(2) -= *vec.add(2); // C: res[2] -= vec[2]
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
    unsafe {
        *res.add(0) += *vec.add(0) * scl; // C: res[0] += vec[0] * scl
        *res.add(1) += *vec.add(1) * scl; // C: res[1] += vec[1] * scl
        *res.add(2) += *vec.add(2) * scl; // C: res[2] += vec[2] * scl
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
    unsafe {
        *res.add(0) = *vec1.add(0) + scl * *vec2.add(0); // C: res[0] = vec1[0] + scl*vec2[0]
        *res.add(1) = *vec1.add(1) + scl * *vec2.add(1); // C: res[1] = vec1[1] + scl*vec2[1]
        *res.add(2) = *vec1.add(2) + scl * *vec2.add(2); // C: res[2] = vec1[2] + scl*vec2[2]
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
    const MJMINVAL: f64 = 1e-15;
    unsafe {
        // C: mjtNum norm = mju_sqrt(vec[0]*vec[0] + vec[1]*vec[1] + vec[2]*vec[2])
        let norm = (*vec.add(0) * *vec.add(0) + *vec.add(1) * *vec.add(1) + *vec.add(2) * *vec.add(2)).sqrt();
        if norm < MJMINVAL { // C: if (norm < mjMINVAL)
            *vec.add(0) = 1.0; // C: vec[0] = 1
            *vec.add(1) = 0.0; // C: vec[1] = 0
            *vec.add(2) = 0.0; // C: vec[2] = 0
        } else {
            let norm_inv = 1.0 / norm; // C: mjtNum normInv = 1/norm
            *vec.add(0) *= norm_inv; // C: vec[0] *= normInv
            *vec.add(1) *= norm_inv; // C: vec[1] *= normInv
            *vec.add(2) *= norm_inv; // C: vec[2] *= normInv
        }
        norm // C: return norm
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
    unsafe {
        // C: return mju_sqrt(vec[0]*vec[0] + vec[1]*vec[1] + vec[2]*vec[2])
        (*vec.add(0) * *vec.add(0) + *vec.add(1) * *vec.add(1) + *vec.add(2) * *vec.add(2)).sqrt()
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
    unsafe {
        // C: return vec1[0]*vec2[0] + vec1[1]*vec2[1] + vec1[2]*vec2[2]
        *vec1.add(0) * *vec2.add(0) + *vec1.add(1) * *vec2.add(1) + *vec1.add(2) * *vec2.add(2)
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
    unsafe {
        // C: mjtNum dif[3] = {pos1[0]-pos2[0], pos1[1]-pos2[1], pos1[2]-pos2[2]}
        let dif0 = *pos1.add(0) - *pos2.add(0); // C: pos1[0]-pos2[0]
        let dif1 = *pos1.add(1) - *pos2.add(1); // C: pos1[1]-pos2[1]
        let dif2 = *pos1.add(2) - *pos2.add(2); // C: pos1[2]-pos2[2]
        // C: return mju_sqrt(dif[0]*dif[0] + dif[1]*dif[1] + dif[2]*dif[2])
        (dif0 * dif0 + dif1 * dif1 + dif2 * dif2).sqrt()
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
    unsafe {
        // C: mjtNum tmp[3] = { mat[0]*vec[0] + mat[1]*vec[1] + mat[2]*vec[2], ... }
        let tmp0 = *mat.add(0) * *vec.add(0) + *mat.add(1) * *vec.add(1) + *mat.add(2) * *vec.add(2); // C: mat[0]*vec[0] + mat[1]*vec[1] + mat[2]*vec[2]
        let tmp1 = *mat.add(3) * *vec.add(0) + *mat.add(4) * *vec.add(1) + *mat.add(5) * *vec.add(2); // C: mat[3]*vec[0] + mat[4]*vec[1] + mat[5]*vec[2]
        let tmp2 = *mat.add(6) * *vec.add(0) + *mat.add(7) * *vec.add(1) + *mat.add(8) * *vec.add(2); // C: mat[6]*vec[0] + mat[7]*vec[1] + mat[8]*vec[2]
        *res.add(0) = tmp0; // C: res[0] = tmp[0]
        *res.add(1) = tmp1; // C: res[1] = tmp[1]
        *res.add(2) = tmp2; // C: res[2] = tmp[2]
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
    unsafe {
        // C: mjtNum tmp[3] = { mat[0]*vec[0] + mat[3]*vec[1] + mat[6]*vec[2], ... }
        let tmp0 = *mat.add(0) * *vec.add(0) + *mat.add(3) * *vec.add(1) + *mat.add(6) * *vec.add(2); // C: mat[0]*vec[0] + mat[3]*vec[1] + mat[6]*vec[2]
        let tmp1 = *mat.add(1) * *vec.add(0) + *mat.add(4) * *vec.add(1) + *mat.add(7) * *vec.add(2); // C: mat[1]*vec[0] + mat[4]*vec[1] + mat[7]*vec[2]
        let tmp2 = *mat.add(2) * *vec.add(0) + *mat.add(5) * *vec.add(1) + *mat.add(8) * *vec.add(2); // C: mat[2]*vec[0] + mat[5]*vec[1] + mat[8]*vec[2]
        *res.add(0) = tmp0; // C: res[0] = tmp[0]
        *res.add(1) = tmp1; // C: res[1] = tmp[1]
        *res.add(2) = tmp2; // C: res[2] = tmp[2]
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
    unsafe {
        *res.add(0) = *mat1.add(0) * *mat2.add(0) + *mat1.add(1) * *mat2.add(3) + *mat1.add(2) * *mat2.add(6); // C: mat1[0]*mat2[0] + mat1[1]*mat2[3] + mat1[2]*mat2[6]
        *res.add(1) = *mat1.add(0) * *mat2.add(1) + *mat1.add(1) * *mat2.add(4) + *mat1.add(2) * *mat2.add(7); // C: mat1[0]*mat2[1] + mat1[1]*mat2[4] + mat1[2]*mat2[7]
        *res.add(2) = *mat1.add(0) * *mat2.add(2) + *mat1.add(1) * *mat2.add(5) + *mat1.add(2) * *mat2.add(8); // C: mat1[0]*mat2[2] + mat1[1]*mat2[5] + mat1[2]*mat2[8]
        *res.add(3) = *mat1.add(3) * *mat2.add(0) + *mat1.add(4) * *mat2.add(3) + *mat1.add(5) * *mat2.add(6); // C: mat1[3]*mat2[0] + mat1[4]*mat2[3] + mat1[5]*mat2[6]
        *res.add(4) = *mat1.add(3) * *mat2.add(1) + *mat1.add(4) * *mat2.add(4) + *mat1.add(5) * *mat2.add(7); // C: mat1[3]*mat2[1] + mat1[4]*mat2[4] + mat1[5]*mat2[7]
        *res.add(5) = *mat1.add(3) * *mat2.add(2) + *mat1.add(4) * *mat2.add(5) + *mat1.add(5) * *mat2.add(8); // C: mat1[3]*mat2[2] + mat1[4]*mat2[5] + mat1[5]*mat2[8]
        *res.add(6) = *mat1.add(6) * *mat2.add(0) + *mat1.add(7) * *mat2.add(3) + *mat1.add(8) * *mat2.add(6); // C: mat1[6]*mat2[0] + mat1[7]*mat2[3] + mat1[8]*mat2[6]
        *res.add(7) = *mat1.add(6) * *mat2.add(1) + *mat1.add(7) * *mat2.add(4) + *mat1.add(8) * *mat2.add(7); // C: mat1[6]*mat2[1] + mat1[7]*mat2[4] + mat1[8]*mat2[7]
        *res.add(8) = *mat1.add(6) * *mat2.add(2) + *mat1.add(7) * *mat2.add(5) + *mat1.add(8) * *mat2.add(8); // C: mat1[6]*mat2[2] + mat1[7]*mat2[5] + mat1[8]*mat2[8]
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
    unsafe {
        *res.add(0) = *mat1.add(0) * *mat2.add(0) + *mat1.add(3) * *mat2.add(3) + *mat1.add(6) * *mat2.add(6); // C: mat1[0]*mat2[0] + mat1[3]*mat2[3] + mat1[6]*mat2[6]
        *res.add(1) = *mat1.add(0) * *mat2.add(1) + *mat1.add(3) * *mat2.add(4) + *mat1.add(6) * *mat2.add(7); // C: mat1[0]*mat2[1] + mat1[3]*mat2[4] + mat1[6]*mat2[7]
        *res.add(2) = *mat1.add(0) * *mat2.add(2) + *mat1.add(3) * *mat2.add(5) + *mat1.add(6) * *mat2.add(8); // C: mat1[0]*mat2[2] + mat1[3]*mat2[5] + mat1[6]*mat2[8]
        *res.add(3) = *mat1.add(1) * *mat2.add(0) + *mat1.add(4) * *mat2.add(3) + *mat1.add(7) * *mat2.add(6); // C: mat1[1]*mat2[0] + mat1[4]*mat2[3] + mat1[7]*mat2[6]
        *res.add(4) = *mat1.add(1) * *mat2.add(1) + *mat1.add(4) * *mat2.add(4) + *mat1.add(7) * *mat2.add(7); // C: mat1[1]*mat2[1] + mat1[4]*mat2[4] + mat1[7]*mat2[7]
        *res.add(5) = *mat1.add(1) * *mat2.add(2) + *mat1.add(4) * *mat2.add(5) + *mat1.add(7) * *mat2.add(8); // C: mat1[1]*mat2[2] + mat1[4]*mat2[5] + mat1[7]*mat2[8]
        *res.add(6) = *mat1.add(2) * *mat2.add(0) + *mat1.add(5) * *mat2.add(3) + *mat1.add(8) * *mat2.add(6); // C: mat1[2]*mat2[0] + mat1[5]*mat2[3] + mat1[8]*mat2[6]
        *res.add(7) = *mat1.add(2) * *mat2.add(1) + *mat1.add(5) * *mat2.add(4) + *mat1.add(8) * *mat2.add(7); // C: mat1[2]*mat2[1] + mat1[5]*mat2[4] + mat1[8]*mat2[7]
        *res.add(8) = *mat1.add(2) * *mat2.add(2) + *mat1.add(5) * *mat2.add(5) + *mat1.add(8) * *mat2.add(8); // C: mat1[2]*mat2[2] + mat1[5]*mat2[5] + mat1[8]*mat2[8]
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
    unsafe {
        *res.add(0) = *mat1.add(0) * *mat2.add(0) + *mat1.add(1) * *mat2.add(1) + *mat1.add(2) * *mat2.add(2); // C: mat1[0]*mat2[0] + mat1[1]*mat2[1] + mat1[2]*mat2[2]
        *res.add(1) = *mat1.add(0) * *mat2.add(3) + *mat1.add(1) * *mat2.add(4) + *mat1.add(2) * *mat2.add(5); // C: mat1[0]*mat2[3] + mat1[1]*mat2[4] + mat1[2]*mat2[5]
        *res.add(2) = *mat1.add(0) * *mat2.add(6) + *mat1.add(1) * *mat2.add(7) + *mat1.add(2) * *mat2.add(8); // C: mat1[0]*mat2[6] + mat1[1]*mat2[7] + mat1[2]*mat2[8]
        *res.add(3) = *mat1.add(3) * *mat2.add(0) + *mat1.add(4) * *mat2.add(1) + *mat1.add(5) * *mat2.add(2); // C: mat1[3]*mat2[0] + mat1[4]*mat2[1] + mat1[5]*mat2[2]
        *res.add(4) = *mat1.add(3) * *mat2.add(3) + *mat1.add(4) * *mat2.add(4) + *mat1.add(5) * *mat2.add(5); // C: mat1[3]*mat2[3] + mat1[4]*mat2[4] + mat1[5]*mat2[5]
        *res.add(5) = *mat1.add(3) * *mat2.add(6) + *mat1.add(4) * *mat2.add(7) + *mat1.add(5) * *mat2.add(8); // C: mat1[3]*mat2[6] + mat1[4]*mat2[7] + mat1[5]*mat2[8]
        *res.add(6) = *mat1.add(6) * *mat2.add(0) + *mat1.add(7) * *mat2.add(1) + *mat1.add(8) * *mat2.add(2); // C: mat1[6]*mat2[0] + mat1[7]*mat2[1] + mat1[8]*mat2[2]
        *res.add(7) = *mat1.add(6) * *mat2.add(3) + *mat1.add(7) * *mat2.add(4) + *mat1.add(8) * *mat2.add(5); // C: mat1[6]*mat2[3] + mat1[7]*mat2[4] + mat1[8]*mat2[5]
        *res.add(8) = *mat1.add(6) * *mat2.add(6) + *mat1.add(7) * *mat2.add(7) + *mat1.add(8) * *mat2.add(8); // C: mat1[6]*mat2[6] + mat1[7]*mat2[7] + mat1[8]*mat2[8]
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
    unsafe {
        *res.add(0) = 0.0; // C: res[0] = 0
        *res.add(1) = 0.0; // C: res[1] = 0
        *res.add(2) = 0.0; // C: res[2] = 0
        *res.add(3) = 0.0; // C: res[3] = 0
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
    unsafe {
        *res.add(0) = 1.0; // C: res[0] = 1
        *res.add(1) = 0.0; // C: res[1] = 0
        *res.add(2) = 0.0; // C: res[2] = 0
        *res.add(3) = 0.0; // C: res[3] = 0
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
    unsafe {
        *res.add(0) = *data.add(0); // C: res[0] = data[0]
        *res.add(1) = *data.add(1); // C: res[1] = data[1]
        *res.add(2) = *data.add(2); // C: res[2] = data[2]
        *res.add(3) = *data.add(3); // C: res[3] = data[3]
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
    const MJMINVAL: f64 = 1e-15;
    unsafe {
        // C: mjtNum norm = mju_sqrt(vec[0]*vec[0] + vec[1]*vec[1] + vec[2]*vec[2] + vec[3]*vec[3])
        let norm = (*vec.add(0) * *vec.add(0) + *vec.add(1) * *vec.add(1) + *vec.add(2) * *vec.add(2) + *vec.add(3) * *vec.add(3)).sqrt();
        if norm < MJMINVAL { // C: if (norm < mjMINVAL)
            *vec.add(0) = 1.0; // C: vec[0] = 1
            *vec.add(1) = 0.0; // C: vec[1] = 0
            *vec.add(2) = 0.0; // C: vec[2] = 0
            *vec.add(3) = 0.0; // C: vec[3] = 0
        } else if (norm - 1.0).abs() > MJMINVAL { // C: else if (mju_abs(norm - 1) > mjMINVAL)
            let norm_inv = 1.0 / norm; // C: mjtNum normInv = 1/norm
            *vec.add(0) *= norm_inv; // C: vec[0] *= normInv
            *vec.add(1) *= norm_inv; // C: vec[1] *= normInv
            *vec.add(2) *= norm_inv; // C: vec[2] *= normInv
            *vec.add(3) *= norm_inv; // C: vec[3] *= normInv
        }
        norm // C: return norm
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
    unsafe {
        std::ptr::write_bytes(res, 0, n as usize); // C: memset(res, 0, n*sizeof(mjtNum))
    }
}

/// C: mju_zeroInd (engine/engine_util_blas.h:148)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_zero_ind(res: *mut f64, n: i32, ind: *const i32) {
    unsafe {
        for i in 0..n { // C: for (int i = 0; i < n; i++)
            *res.add(*ind.add(i as usize) as usize) = 0.0; // C: res[ind[i]] = 0
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
    unsafe {
        for i in 0..n as usize { // C: for (int i=0; i < n; i++)
            *res.add(i) = val; // C: res[i] = val
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
    unsafe {
        std::ptr::copy_nonoverlapping(vec, res, n as usize); // C: memcpy(res, vec, n*sizeof(mjtNum))
    }
}

/// C: mju_copyInd (engine/engine_util_blas.h:157)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_copy_ind(res: *mut f64, vec: *const f64, ind: *const i32, n: i32) {
    unsafe {
        for i in 0..n as usize { // C: for (int i = 0; i < n; i++)
            let idx = *ind.add(i) as usize; // C: ind[i]
            *res.add(idx) = *vec.add(idx); // C: res[ind[i]] = vec[ind[i]]
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
    unsafe {
        let mut res: f64 = 0.0; // C: mjtNum res = 0
        for i in 0..n as usize { // C: for (int i=0; i < n; i++)
            res += *vec.add(i); // C: res += vec[i]
        }
        res // C: return res
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
    unsafe {
        let mut res: f64 = 0.0; // C: mjtNum res = 0
        for i in 0..n as usize { // C: for (int i=0; i < n; i++)
            res += (*vec.add(i)).abs(); // C: res += mju_abs(vec[i])
        }
        res // C: return res
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
    unsafe {
        for i in 0..n as usize { // C: for (int i = 0; i < n; i++)
            *res.add(i) = *vec.add(i) * scl; // C: res[i] = vec[i]*scl
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
    unsafe {
        for i in 0..n as usize { // C: for (int i = 0; i < n; i++)
            *res.add(i) = *vec1.add(i) + *vec2.add(i); // C: res[i] = vec1[i] + vec2[i]
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
    unsafe {
        for i in 0..n as usize { // C: for (int i = 0; i < n; i++)
            let j = *ind.add(i) as usize; // C: int j = ind[i]
            *res.add(j) = *vec1.add(j) + *vec2.add(j); // C: res[j] = vec1[j] + vec2[j]
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
    unsafe {
        for i in 0..n as usize { // C: for (int i = 0; i < n; i++)
            *res.add(i) = *vec1.add(i) - *vec2.add(i); // C: res[i] = vec1[i] - vec2[i]
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
    unsafe {
        for i in 0..n as usize { // C: for (int i = 0; i < n; i++)
            let j = *ind.add(i) as usize; // C: int j = ind[i]
            *res.add(j) = *vec1.add(j) - *vec2.add(j); // C: res[j] = vec1[j] - vec2[j]
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
    unsafe {
        for i in 0..n as usize { // C: for (int i = 0; i < n; i++)
            *res.add(i) += *vec.add(i); // C: res[i] += vec[i]
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
    unsafe {
        for i in 0..n as usize { // C: for (int i = 0; i < n; i++)
            let j = *ind.add(i) as usize; // C: int j = ind[i]
            *res.add(j) += *vec.add(j); // C: res[j] += vec[j]
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
    unsafe {
        for i in 0..n as usize { // C: for (int i = 0; i < n; i++)
            *res.add(i) -= *vec.add(i); // C: res[i] -= vec[i]
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
    unsafe {
        for i in 0..n as usize { // C: for (int i = 0; i < n; i++)
            *res.add(i) += *vec.add(i) * scl; // C: res[i] += vec[i]*scl
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
    unsafe {
        for i in 0..n as usize { // C: for (int i=0; i < n; i++)
            let k = *ind.add(i) as usize; // C: int k = ind[i]
            *res.add(k) += *vec.add(k) * scl; // C: res[k] += vec[k]*scl
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
    unsafe {
        for i in 0..n as usize { // C: for (int i = 0; i < n; i++)
            *res.add(i) = *vec1.add(i) + *vec2.add(i) * scl; // C: res[i] = vec1[i] + vec2[i]*scl
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
    const MJMINVAL: f64 = 1e-15;
    unsafe {
        // C: mjtNum norm = mju_sqrt(mju_dot(res, res, n))
        let norm = mju_dot(res, res, n).sqrt();
        if norm < MJMINVAL { // C: if (norm < mjMINVAL)
            *res.add(0) = 1.0; // C: res[0] = 1
            mju_zero(res.add(1), n - 1); // C: mju_zero(res + 1, n - 1)
        } else {
            let norm_inv = 1.0 / norm; // C: mjtNum normInv = 1 / norm
            for i in 0..n as usize { // C: for (int i=0; i < n; i++)
                *res.add(i) *= norm_inv; // C: res[i] *= normInv
            }
        }
        norm // C: return norm
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
    // C: return mju_sqrt(mju_dot(res, res, n))
    mju_dot(res, res, n).sqrt()
}

/// C: mju_dot (engine/engine_util_blas.h:205)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_dot(vec1: *const f64, vec2: *const f64, n: i32) -> f64 {
    unsafe {
        let mut res: f64 = 0.0; // C: mjtNum res = 0
        let mut i: usize = 0; // C: int i = 0
        // C: int n_4 = n - 4
        let n_4 = n - 4;
        let mut res0: f64 = 0.0; // C: mjtNum res0 = 0
        let mut res1: f64 = 0.0; // C: mjtNum res1 = 0
        let mut res2: f64 = 0.0; // C: mjtNum res2 = 0
        let mut res3: f64 = 0.0; // C: mjtNum res3 = 0
        // C: for (; i <= n_4; i+=4)
        while i as i32 <= n_4 {
            res0 += *vec1.add(i) * *vec2.add(i);       // C: res0 += vec1[i] * vec2[i]
            res1 += *vec1.add(i + 1) * *vec2.add(i + 1); // C: res1 += vec1[i+1] * vec2[i+1]
            res2 += *vec1.add(i + 2) * *vec2.add(i + 2); // C: res2 += vec1[i+2] * vec2[i+2]
            res3 += *vec1.add(i + 3) * *vec2.add(i + 3); // C: res3 += vec1[i+3] * vec2[i+3]
            i += 4;
        }
        // C: res = (res0 + res2) + (res1 + res3)
        res = (res0 + res2) + (res1 + res3);
        // C: int n_i = n - i
        let n_i = n as usize - i;
        if n_i == 3 { // C: if (n_i == 3)
            // C: res += vec1[i]*vec2[i] + vec1[i+1]*vec2[i+1] + vec1[i+2]*vec2[i+2]
            res += *vec1.add(i) * *vec2.add(i) + *vec1.add(i + 1) * *vec2.add(i + 1) + *vec1.add(i + 2) * *vec2.add(i + 2);
        } else if n_i == 2 { // C: else if (n_i == 2)
            // C: res += vec1[i]*vec2[i] + vec1[i+1]*vec2[i+1]
            res += *vec1.add(i) * *vec2.add(i) + *vec1.add(i + 1) * *vec2.add(i + 1);
        } else if n_i == 1 { // C: else if (n_i == 1)
            // C: res += vec1[i]*vec2[i]
            res += *vec1.add(i) * *vec2.add(i);
        }
        res // C: return res
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
    unsafe {
        let mut res: f64 = 0.0; // C: mjtNum res = 0
        for i in 0..n as usize { // C: for (int i = 0; i < n; i++)
            let k = *ind.add(i) as usize; // C: int k = ind[i]
            res += *vec1.add(k) * *vec2.add(k); // C: res += vec1[k] * vec2[k]
        }
        res // C: return res
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
    unsafe {
        for r in 0..nr as usize { // C: for (int r=0; r < nr; r++)
            // C: res[r] = mju_dot(mat + r*nc, vec, nc)
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
    unsafe {
        mju_zero(res, nc); // C: mju_zero(res, nc)
        for r in 0..nr as usize { // C: for (int r=0; r < nr; r++)
            let tmp = *vec.add(r); // C: tmp = vec[r]
            if tmp != 0.0 { // C: if ((tmp = vec[r]) != 0.0)
                mju_add_to_scl(res, mat.add(r * nc as usize), tmp, nc); // C: mju_addToScl(res, mat+r*nc, tmp, nc)
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
    unsafe {
        let mut res: f64 = 0.0; // C: mjtNum res = 0
        for i in 0..n as usize { // C: for (int i=0; i < n; i++)
            // C: res += vec1[i] * mju_dot(mat + i*n, vec2, n)
            res += *vec1.add(i) * mju_dot(mat.add(i * n as usize), vec2, n);
        }
        res // C: return res
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
    unsafe {
        for i in 0..nr as usize { // C: for (int i=0; i < nr; i++)
            for j in 0..nc as usize { // C: for (int j=0; j < nc; j++)
                // C: res[j*nr+i] = mat[i*nc+j]
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
    unsafe {
        let n_us = n as usize;
        for i in 0..n_us { // C: for (int i=0; i < n; i++)
            // C: res[i*(n+1)] = mat[i*(n+1)]
            *res.add(i * (n_us + 1)) = *mat.add(i * (n_us + 1));
            for j in 0..i { // C: for (int j=0; j < i; j++)
                // C: res[i*n+j] = res[j*n+i] = 0.5 * (mat[i*n+j] + mat[j*n+i])
                let val = 0.5 * (*mat.add(i * n_us + j) + *mat.add(j * n_us + i));
                *res.add(i * n_us + j) = val;
                *res.add(j * n_us + i) = val;
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
    unsafe {
        mju_zero(mat, n * n); // C: mju_zero(mat, n*n)
        for i in 0..n as usize { // C: for (int i=0; i < n; i++)
            // C: mat[i*(n + 1)] = 1
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
    unsafe {
        let nc_us = nc as usize;
        for i in 0..n as usize { // C: for (int i = 0; i < n; i++)
            let row = *ind.add(i) as usize; // C: ind[i]
            // C: mju_copy(res + nc*ind[i], mat + nc*ind[i], nc)
            mju_copy(res.add(nc_us * row), mat.add(nc_us * row), nc);
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
    unsafe {
        let c1_us = c1 as usize;
        let c2_us = c2 as usize;
        mju_zero(res, r1 * c2); // C: mju_zero(res, r1*c2)
        for i in 0..r1 as usize { // C: for (int i=0; i < r1; i++)
            for k in 0..c1 as usize { // C: for (int k=0; k < c1; k++)
                let tmp = *mat1.add(i * c1_us + k); // C: tmp = mat1[i*c1+k]
                if tmp != 0.0 { // C: if ((tmp = mat1[i*c1+k]) != 0.0)
                    // C: mju_addToScl(res+i*c2, mat2+k*c2, tmp, c2)
                    mju_add_to_scl(res.add(i * c2_us), mat2.add(k * c2_us), tmp, c2);
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
    unsafe {
        let c1_us = c1 as usize;
        let r2_us = r2 as usize;
        for i in 0..r1 as usize { // C: for (int i=0; i < r1; i++)
            for j in 0..r2 as usize { // C: for (int j=0; j < r2; j++)
                // C: res[i*r2+j] = mju_dot(mat1+i*c1, mat2+j*c1, c1)
                *res.add(i * r2_us + j) = mju_dot(mat1.add(i * c1_us), mat2.add(j * c1_us), c1);
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
    unsafe {
        let mut tmp: f64; // C: mjtNum tmp
        mju_zero(res, c1 * c2); // C: mju_zero(res, c1*c2)
        for i in 0..r1 { // C: for (int i=0; i < r1; i++)
            for j in 0..c1 { // C: for (int j=0; j < c1; j++)
                tmp = *mat1.add((i * c1 + j) as usize); // C: tmp = mat1[i*c1+j]
                if tmp != 0.0 { // C: if ((tmp = mat1[i*c1+j]))
                    mju_add_to_scl(res.add((j * c2) as usize), mat2.add((i * c2) as usize), tmp, c2); // C: mju_addToScl(res+j*c2, mat2+i*c2, tmp, c2)
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
    unsafe {
        let mut tmp: f64; // C: mjtNum tmp
        // C: mju_zero(res, nc*nc)
        mju_zero(res, nc * nc);
        if !diag.is_null() { // C: if (diag)
            for j in 0..nr { // C: for (int j=0; j < nr; j++)
                if *diag.add(j as usize) != 0.0 { // C: if (diag[j])
                    for i in 0..nc { // C: for (int i=0; i < nc; i++)
                        tmp = *mat.add((j * nc + i) as usize); // C: tmp = mat[j*nc+i]
                        if tmp != 0.0 { // C: if ((tmp = mat[j*nc+i]))
                            mju_add_to_scl(res.add((i * nc) as usize), mat.add((j * nc) as usize), tmp * *diag.add(j as usize), i + 1); // C: mju_addToScl(res+i*nc, mat+j*nc, tmp*diag[j], i+1)
                        }
                    }
                }
            }
        } else { // C: else
            for i in 0..nc { // C: for (int i=0; i < nc; i++)
                for j in 0..nr { // C: for (int j=0; j < nr; j++)
                    tmp = *mat.add((j * nc + i) as usize); // C: tmp = mat[j*nc+i]
                    if tmp != 0.0 { // C: if ((tmp = mat[j*nc+i]))
                        mju_add_to_scl(res.add((i * nc) as usize), mat.add((j * nc) as usize), tmp, i + 1); // C: mju_addToScl(res+i*nc, mat+j*nc, tmp, i+1)
                    }
                }
            }
        }
        // C: if (flg_upper) - make symmetric
        if flg_upper != 0 {
            for i in 0..nc { // C: for (int i=0; i < nc; i++)
                for j in (i + 1)..nc { // C: for (int j=i+1; j < nc; j++)
                    *res.add((i * nc + j) as usize) = *res.add((j * nc + i) as usize); // C: res[i*nc+j] = res[j*nc+i]
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
    // C: mju_sqrMatTD_impl(res, mat, diag, nr, nc, /*flg_upper=*/ 1)
    mju_sqr_mat_td_impl(res, mat, diag, nr, nc, 1);
}

