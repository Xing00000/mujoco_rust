//! Port of: engine/engine_util_blas.c
//! IR hash: fd5288ad7242f001
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mju_zero3 (engine/engine_util_blas.c:26)
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

/// C: mju_equal3 (engine/engine_util_blas.c:34)
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

/// C: mju_copy3 (engine/engine_util_blas.c:42)
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

/// C: mju_copy9 (engine/engine_util_blas.c:49)
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

/// C: mju_scl3 (engine/engine_util_blas.c:57)
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

/// C: mju_add3 (engine/engine_util_blas.c:65)
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

/// C: mju_sub3 (engine/engine_util_blas.c:73)
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

/// C: mju_addTo3 (engine/engine_util_blas.c:81)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_add_to3(res: *mut f64, vec: *const f64) {
    todo!() // cxx:_mju_addTo3
}

/// C: mju_subFrom3 (engine/engine_util_blas.c:89)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_sub_from3(res: *mut f64, vec: *const f64) {
    todo!() // cxx:_mju_subFrom3
}

/// C: mju_addToScl3 (engine/engine_util_blas.c:97)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_add_to_scl3(res: *mut f64, vec: *const f64, scl: f64) {
    todo!() // cxx:_mju_addToScl3
}

/// C: mju_addScl3 (engine/engine_util_blas.c:105)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_add_scl3(res: *mut f64, vec1: *const f64, vec2: *const f64, scl: f64) {
    todo!() // cxx:_mju_addScl3
}

/// C: mju_normalize3 (engine/engine_util_blas.c:113)
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

/// C: mju_norm3 (engine/engine_util_blas.c:132)
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

/// C: mju_dot3 (engine/engine_util_blas.c:138)
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

/// C: mju_dist3 (engine/engine_util_blas.c:144)
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

/// C: mju_mulMatVec3 (engine/engine_util_blas.c:151)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_mul_mat_vec3(res: *mut f64, mat: *const f64, vec: *const f64) {
    todo!() // cxx:_mju_mulMatVec3
}

/// C: mju_mulMatTVec3 (engine/engine_util_blas.c:164)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_mul_mat_t_vec3(res: *mut f64, mat: *const f64, vec: *const f64) {
    todo!() // cxx:_mju_mulMatTVec3
}

/// C: mju_mulMatMat3 (engine/engine_util_blas.c:177)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_mul_mat_mat3(res: *mut f64, mat1: *const f64, mat2: *const f64) {
    todo!() // cxx:_mju_mulMatMat3
}

/// C: mju_mulMatTMat3 (engine/engine_util_blas.c:191)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_mul_mat_t_mat3(res: *mut f64, mat1: *const f64, mat2: *const f64) {
    todo!() // cxx:_mju_mulMatTMat3
}

/// C: mju_mulMatMatT3 (engine/engine_util_blas.c:205)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_mul_mat_mat_t3(res: *mut f64, mat1: *const f64, mat2: *const f64) {
    todo!() // cxx:_mju_mulMatMatT3
}

/// C: mju_zero4 (engine/engine_util_blas.c:221)
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

/// C: mju_unit4 (engine/engine_util_blas.c:230)
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

/// C: mju_copy4 (engine/engine_util_blas.c:239)
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

/// C: mju_normalize4 (engine/engine_util_blas.c:248)
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

/// C: mju_zero (engine/engine_util_blas.c:271)
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

/// C: mju_zeroInd (engine/engine_util_blas.c:277)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_zero_ind(res: *mut f64, n: i32, ind: *const i32) {
    todo!() // cxx:_mju_zeroInd
}

/// C: mju_fill (engine/engine_util_blas.c:285)
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

/// C: mju_copy (engine/engine_util_blas.c:293)
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

/// C: mju_copyInd (engine/engine_util_blas.c:299)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_copy_ind(res: *mut f64, vec: *const f64, ind: *const i32, n: i32) {
    todo!() // cxx:_mju_copyInd
}

/// C: mju_sum (engine/engine_util_blas.c:307)
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

/// C: mju_L1 (engine/engine_util_blas.c:319)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_l1(vec: *const f64, n: i32) -> f64 {
    todo!() // cxx:_mju_L1
}

/// C: mju_scl (engine/engine_util_blas.c:331)
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

/// C: mju_add (engine/engine_util_blas.c:343)
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

/// C: mju_addInd (engine/engine_util_blas.c:355)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_add_ind(res: *mut f64, vec1: *const f64, vec2: *const f64, ind: *const i32, n: i32) {
    todo!() // cxx:_mju_addInd
}

/// C: mju_sub (engine/engine_util_blas.c:364)
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

/// C: mju_subInd (engine/engine_util_blas.c:376)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_sub_ind(res: *mut f64, vec1: *const f64, vec2: *const f64, ind: *const i32, n: i32) {
    todo!() // cxx:_mju_subInd
}

/// C: mju_addTo (engine/engine_util_blas.c:385)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_add_to(res: *mut f64, vec: *const f64, n: i32) {
    todo!() // cxx:_mju_addTo
}

/// C: mju_addToInd (engine/engine_util_blas.c:397)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_add_to_ind(res: *mut f64, vec: *const f64, ind: *const i32, n: i32) {
    todo!() // cxx:_mju_addToInd
}

/// C: mju_subFrom (engine/engine_util_blas.c:406)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_sub_from(res: *mut f64, vec: *const f64, n: i32) {
    todo!() // cxx:_mju_subFrom
}

/// C: mju_addToScl (engine/engine_util_blas.c:418)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_add_to_scl(res: *mut f64, vec: *const f64, scl: f64, n: i32) {
    todo!() // cxx:_mju_addToScl
}

/// C: mju_addToSclInd (engine/engine_util_blas.c:431)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_add_to_scl_ind(res: *mut f64, vec: *const f64, ind: *const i32, scl: f64, n: i32) {
    todo!() // cxx:_mju_addToSclInd
}

/// C: mju_addScl (engine/engine_util_blas.c:441)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_add_scl(res: *mut f64, vec1: *const f64, vec2: *const f64, scl: f64, n: i32) {
    todo!() // cxx:_mju_addScl
}

/// C: mju_normalize (engine/engine_util_blas.c:453)
/// Calls: cxx:_mju_dot, cxx:_mju_zero
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

/// C: mju_norm (engine/engine_util_blas.c:471)
/// Calls: cxx:_mju_dot
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

/// C: mju_dot (engine/engine_util_blas.c:477)
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

/// C: mju_dotInd (engine/engine_util_blas.c:515)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_dot_ind(vec1: *const f64, vec2: *const f64, ind: *const i32, n: i32) -> f64 {
    todo!() // cxx:_mju_dotInd
}

/// C: mju_mulMatVec (engine/engine_util_blas.c:529)
/// Calls: cxx:_mju_dot
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_mul_mat_vec(res: *mut f64, mat: *const f64, vec: *const f64, nr: i32, nc: i32) {
    todo!() // cxx:_mju_mulMatVec
}

/// C: mju_mulMatTVec (engine/engine_util_blas.c:537)
/// Calls: cxx:_mju_addToScl, cxx:_mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_mul_mat_t_vec(res: *mut f64, mat: *const f64, vec: *const f64, nr: i32, nc: i32) {
    todo!() // cxx:_mju_mulMatTVec
}

/// C: mju_mulVecMatVec (engine/engine_util_blas.c:550)
/// Calls: cxx:_mju_dot
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_mul_vec_mat_vec(vec1: *const f64, mat: *const f64, vec2: *const f64, n: i32) -> f64 {
    todo!() // cxx:_mju_mulVecMatVec
}

/// C: mju_transpose (engine/engine_util_blas.c:562)
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

/// C: mju_symmetrize (engine/engine_util_blas.c:572)
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

/// C: mju_eye (engine/engine_util_blas.c:583)
/// Calls: cxx:_mju_zero
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

/// C: mju_copyRows (engine/engine_util_blas.c:592)
/// Calls: cxx:_mju_copy
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_copy_rows(res: *mut f64, mat: *const f64, ind: *const i32, n: i32, nc: i32) {
    todo!() // cxx:_mju_copyRows
}

/// C: mju_mulMatMat (engine/engine_util_blas.c:603)
/// Calls: cxx:_mju_addToScl, cxx:_mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_mul_mat_mat(res: *mut f64, mat1: *const f64, mat2: *const f64, r1: i32, c1: i32, c2: i32) {
    todo!() // cxx:_mju_mulMatMat
}

/// C: mju_mulMatMatT (engine/engine_util_blas.c:620)
/// Calls: cxx:_mju_dot
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_mul_mat_mat_t(res: *mut f64, mat1: *const f64, mat2: *const f64, r1: i32, c1: i32, r2: i32) {
    todo!() // cxx:_mju_mulMatMatT
}

/// C: mju_sqrMatTD_impl (engine/engine_util_blas.c:631)
/// Calls: cxx:_mju_addToScl, cxx:_mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_sqr_mat_td_impl(res: *mut f64, mat: *const f64, diag: *const f64, nr: i32, nc: i32, flg_upper: i32) {
    todo!() // cxx:_mju_sqrMatTD_impl
}

/// C: mju_sqrMatTD (engine/engine_util_blas.c:668)
/// Calls: cxx:_mju_sqrMatTD_impl
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_sqr_mat_td(res: *mut f64, mat: *const f64, diag: *const f64, nr: i32, nc: i32) {
    todo!() // cxx:_mju_sqrMatTD
}

/// C: mju_mulMatTMat (engine/engine_util_blas.c:675)
/// Calls: cxx:_mju_addToScl, cxx:_mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_mul_mat_t_mat(res: *mut f64, mat1: *const f64, mat2: *const f64, r1: i32, c1: i32, c2: i32) {
    todo!() // cxx:_mju_mulMatTMat
}

