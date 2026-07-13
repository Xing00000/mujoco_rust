//! Port of: user/user_util.cc
//! IR hash: e878892ca48fe222
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mjuu_axisAngle2Quat (user/user_util.cc:564)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_axis_angle2quat(res: *mut f64, axis: *const f64, angle: f64) {
    // SAFETY: res[4], axis[3] valid (caller contract)
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

/// C: mjuu_isValidContentType (user/user_util.cc:973)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_is_valid_content_type(text: string_view) -> bool {
    todo!() // mjuu_isValidContentType
}

/// C: StrToNum (user/user_util.cc:1277)
#[allow(unused_variables, non_snake_case)]
pub fn str_to_num(str: *mut i8, c: *mut *mut i8) -> i32 {
    todo!() // StrToNum
}

/// C: IsNullOrSpace (user/user_util.cc:1301)
#[allow(unused_variables, non_snake_case)]
pub fn is_null_or_space(c: *mut i8) -> bool {
    todo!() // IsNullOrSpace
}

/// C: SkipSpace (user/user_util.cc:1305)
/// Calls: IsNullOrSpace
#[allow(unused_variables, non_snake_case)]
pub fn skip_space(c: *mut i8) -> *mut i8 {
    todo!() // SkipSpace
}

/// C: mjuu_defined (user/user_util.h:35)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_defined(num: f64) -> bool {
    todo!() // mjuu_defined
}

/// C: mjuu_matadr (user/user_util.h:39)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_matadr(g1: i32, g2: i32, n: i32) -> i32 {
    if g1 < 0 || g2 < 0 || g1 >= n || g2 >= n {
        return -1;
    }

    let (g1, g2) = if g1 > g2 { (g2, g1) } else { (g1, g2) };
    g1 * n + g2
}

/// C: mjuu_setvec (user/user_util.h:42)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_setvec(dest: *mut f64, x: f64, y: f64, z: f64, w: f64) {
    // SAFETY: dest points to at least 4 f64 (caller contract)
    unsafe {
        *dest.add(0) = x;
        *dest.add(1) = y;
        *dest.add(2) = z;
        *dest.add(3) = w;
    }
}

/// C: mjuu_copyvec (user/user_util.h:54)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_copyvec(dest: *mut T1, src: *const T2, n: i32) {
    // SAFETY: caller guarantees dest and src point to at least n f64 elements
    unsafe {
        let d = dest as *mut f64;
        let s = src as *const f64;
        let mut i: i32 = 0;
        while i < n {
            *d.offset(i as isize) = *s.offset(i as isize);
            i += 1;
        }
    }
}

/// C: mjuu_addtovec (user/user_util.h:59)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_addtovec(dest: *mut f64, src: *const f64, n: i32) {
    // SAFETY: caller guarantees dest and src point to at least n f64
    unsafe {
        let mut i: i32 = 0;
        while i < n {
            *dest.offset(i as isize) += *src.offset(i as isize);
            i += 1;
        }
    }
}

/// C: mjuu_zerovec (user/user_util.h:62)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_zerovec(dest: *mut f64, n: i32) {
    let mut i: i32 = 0;
    while i < n {
        // SAFETY: caller guarantees dest points to at least n doubles
        unsafe { *dest.offset(i as isize) = 0.0; }
        i += 1;
    }
}

/// C: mjuu_dot3 (user/user_util.h:68)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_dot3(a: *const f64, b: *const f64) -> f64 {
    // SAFETY: a and b point to at least 3 f64 (caller contract)
    unsafe {
        *a.add(0) * *b.add(0) + *a.add(1) * *b.add(1) + *a.add(2) * *b.add(2)
    }
}

/// C: mjuu_dist3 (user/user_util.h:71)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_dist3(a: *const f64, b: *const f64) -> f64 {
    // SAFETY: caller guarantees a and b point to at least 3 f64
    unsafe {
        f64::sqrt(
            (*a.add(0) - *b.add(0)) * (*a.add(0) - *b.add(0))
          + (*a.add(1) - *b.add(1)) * (*a.add(1) - *b.add(1))
          + (*a.add(2) - *b.add(2)) * (*a.add(2) - *b.add(2))
        )
    }
}

/// C: mjuu_L1 (user/user_util.h:74)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_l1(a: *const f64, b: *const f64, n: i32) -> f64 {
    // SAFETY: caller guarantees a and b point to at least n f64
    unsafe {
        let mut res: f64 = 0.0;
        let mut i: i32 = 0;
        while i < n {
            res += (*a.offset(i as isize) - *b.offset(i as isize)).abs();
            i += 1;
        }
        res
    }
}

/// C: mjuu_normvec (user/user_util.h:78)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_normvec(vec: *mut f64, n: i32) -> f64 {
    const MJEPS: f64 = 1e-14;
    // SAFETY: caller guarantees vec points to at least n f64
    unsafe {
        let mut nrm: f64 = 0.0;
        let mut i: i32 = 0;
        while i < n {
            nrm += *vec.offset(i as isize) * *vec.offset(i as isize);
            i += 1;
        }
        if nrm < MJEPS {
            return 0.0;
        }
        nrm = nrm.sqrt();
        if (nrm - 1.0).abs() > MJEPS {
            i = 0;
            while i < n {
                *vec.offset(i as isize) /= nrm;
                i += 1;
            }
        }
        nrm
    }
}

/// C: mjuu_scalevec (user/user_util.h:82)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_scalevec(res: *mut f64, vec: *const f64, s: f64, n: i32) {
    // SAFETY: res and vec are valid arrays of length >= n from caller
    unsafe {
        for i in 0..n as usize {
            *res.add(i) = s * *vec.add(i);
        }
    }
}

/// C: mjuu_quat2mat (user/user_util.h:85)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_quat2mat(res: *mut f64, quat: *const f64) {
    // SAFETY: res is 9-element array, quat is 4-element array from caller
    unsafe {
        // identity quat: identity mat
        if *quat.add(0) == 1.0 && *quat.add(1) == 0.0 && *quat.add(2) == 0.0 && *quat.add(3) == 0.0 {
            *res.add(0) = 1.0;
            *res.add(1) = 0.0;
            *res.add(2) = 0.0;
            *res.add(3) = 0.0;
            *res.add(4) = 1.0;
            *res.add(5) = 0.0;
            *res.add(6) = 0.0;
            *res.add(7) = 0.0;
            *res.add(8) = 1.0;
            return;
        }

        // regular processing
        let q00 = *quat.add(0) * *quat.add(0);
        let q01 = *quat.add(0) * *quat.add(1);
        let q02 = *quat.add(0) * *quat.add(2);
        let q03 = *quat.add(0) * *quat.add(3);
        let q11 = *quat.add(1) * *quat.add(1);
        let q12 = *quat.add(1) * *quat.add(2);
        let q13 = *quat.add(1) * *quat.add(3);
        let q22 = *quat.add(2) * *quat.add(2);
        let q23 = *quat.add(2) * *quat.add(3);
        let q33 = *quat.add(3) * *quat.add(3);

        *res.add(0) = q00 + q11 - q22 - q33;
        *res.add(4) = q00 - q11 + q22 - q33;
        *res.add(8) = q00 - q11 - q22 + q33;

        *res.add(1) = 2.0 * (q12 - q03);
        *res.add(2) = 2.0 * (q13 + q02);
        *res.add(3) = 2.0 * (q12 + q03);
        *res.add(5) = 2.0 * (q23 - q01);
        *res.add(6) = 2.0 * (q13 - q02);
        *res.add(7) = 2.0 * (q23 + q01);
    }
}

/// C: mjuu_mulquat (user/user_util.h:88)
/// Calls: mjuu_copyvec, mjuu_normvec
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_mulquat(res: *mut f64, qa: *const f64, qb: *const f64) {
    // SAFETY: res[4], qa[4], qb[4] valid (caller contract)
    unsafe {
        let mut tmp: [f64; 4] = [0.0; 4];
        tmp[0] = *qa.add(0) * *qb.add(0) - *qa.add(1) * *qb.add(1) - *qa.add(2) * *qb.add(2) - *qa.add(3) * *qb.add(3);
        tmp[1] = *qa.add(0) * *qb.add(1) + *qa.add(1) * *qb.add(0) + *qa.add(2) * *qb.add(3) - *qa.add(3) * *qb.add(2);
        tmp[2] = *qa.add(0) * *qb.add(2) - *qa.add(1) * *qb.add(3) + *qa.add(2) * *qb.add(0) + *qa.add(3) * *qb.add(1);
        tmp[3] = *qa.add(0) * *qb.add(3) + *qa.add(1) * *qb.add(2) - *qa.add(2) * *qb.add(1) + *qa.add(3) * *qb.add(0);
        mjuu_normvec(tmp.as_mut_ptr(), 4);
        // copyvec: res = tmp
        *res.add(0) = tmp[0];
        *res.add(1) = tmp[1];
        *res.add(2) = tmp[2];
        *res.add(3) = tmp[3];
    }
}

/// C: mjuu_mulvecmat (user/user_util.h:91)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_mulvecmat(res: *mut f64, vec: *const f64, mat: *const f64) {
    // SAFETY: res is 3-element, vec is 3-element, mat is 9-element (3x3 row-major)
    unsafe {
        let tmp0 = *mat.add(0) * *vec.add(0) + *mat.add(1) * *vec.add(1) + *mat.add(2) * *vec.add(2);
        let tmp1 = *mat.add(3) * *vec.add(0) + *mat.add(4) * *vec.add(1) + *mat.add(5) * *vec.add(2);
        let tmp2 = *mat.add(6) * *vec.add(0) + *mat.add(7) * *vec.add(1) + *mat.add(8) * *vec.add(2);
        *res.add(0) = tmp0;
        *res.add(1) = tmp1;
        *res.add(2) = tmp2;
    }
}

/// C: mjuu_mulvecmatT (user/user_util.h:94)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_mulvecmat_t(res: *mut f64, vec: *const f64, mat: *const f64) {
    // SAFETY: res[3], vec[3], mat[9] valid (caller contract)
    unsafe {
        let tmp0 = *mat.add(0) * *vec.add(0) + *mat.add(3) * *vec.add(1) + *mat.add(6) * *vec.add(2);
        let tmp1 = *mat.add(1) * *vec.add(0) + *mat.add(4) * *vec.add(1) + *mat.add(7) * *vec.add(2);
        let tmp2 = *mat.add(2) * *vec.add(0) + *mat.add(5) * *vec.add(1) + *mat.add(8) * *vec.add(2);
        *res.add(0) = tmp0;
        *res.add(1) = tmp1;
        *res.add(2) = tmp2;
    }
}

/// C: mjuu_mulRMRT (user/user_util.h:97)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_mul_rmrt(res: *mut f64, R: *const f64, M: *const f64) {
    // SAFETY: res[9], R[9], M[9] valid (caller contract)
    unsafe {
        // tmp = R*M
        let mut tmp: [f64; 9] = [0.0; 9];
        tmp[0] = *R.add(0) * *M.add(0) + *R.add(1) * *M.add(3) + *R.add(2) * *M.add(6);
        tmp[1] = *R.add(0) * *M.add(1) + *R.add(1) * *M.add(4) + *R.add(2) * *M.add(7);
        tmp[2] = *R.add(0) * *M.add(2) + *R.add(1) * *M.add(5) + *R.add(2) * *M.add(8);
        tmp[3] = *R.add(3) * *M.add(0) + *R.add(4) * *M.add(3) + *R.add(5) * *M.add(6);
        tmp[4] = *R.add(3) * *M.add(1) + *R.add(4) * *M.add(4) + *R.add(5) * *M.add(7);
        tmp[5] = *R.add(3) * *M.add(2) + *R.add(4) * *M.add(5) + *R.add(5) * *M.add(8);
        tmp[6] = *R.add(6) * *M.add(0) + *R.add(7) * *M.add(3) + *R.add(8) * *M.add(6);
        tmp[7] = *R.add(6) * *M.add(1) + *R.add(7) * *M.add(4) + *R.add(8) * *M.add(7);
        tmp[8] = *R.add(6) * *M.add(2) + *R.add(7) * *M.add(5) + *R.add(8) * *M.add(8);

        // res = tmp*R'
        *res.add(0) = tmp[0] * *R.add(0) + tmp[1] * *R.add(1) + tmp[2] * *R.add(2);
        *res.add(1) = tmp[0] * *R.add(3) + tmp[1] * *R.add(4) + tmp[2] * *R.add(5);
        *res.add(2) = tmp[0] * *R.add(6) + tmp[1] * *R.add(7) + tmp[2] * *R.add(8);
        *res.add(3) = tmp[3] * *R.add(0) + tmp[4] * *R.add(1) + tmp[5] * *R.add(2);
        *res.add(4) = tmp[3] * *R.add(3) + tmp[4] * *R.add(4) + tmp[5] * *R.add(5);
        *res.add(5) = tmp[3] * *R.add(6) + tmp[4] * *R.add(7) + tmp[5] * *R.add(8);
        *res.add(6) = tmp[6] * *R.add(0) + tmp[7] * *R.add(1) + tmp[8] * *R.add(2);
        *res.add(7) = tmp[6] * *R.add(3) + tmp[7] * *R.add(4) + tmp[8] * *R.add(5);
        *res.add(8) = tmp[6] * *R.add(6) + tmp[7] * *R.add(7) + tmp[8] * *R.add(8);
    }
}

/// C: mjuu_mulmat (user/user_util.h:100)
/// Calls: mjuu_copyvec
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_mulmat(res: *mut f64, A: *const f64, B: *const f64) {
    // SAFETY: res[9], A[9], B[9] valid (caller contract). Use tmp for aliasing safety.
    unsafe {
        let mut tmp: [f64; 9] = [0.0; 9];
        tmp[0] = *A.add(0) * *B.add(0) + *A.add(1) * *B.add(3) + *A.add(2) * *B.add(6);
        tmp[1] = *A.add(0) * *B.add(1) + *A.add(1) * *B.add(4) + *A.add(2) * *B.add(7);
        tmp[2] = *A.add(0) * *B.add(2) + *A.add(1) * *B.add(5) + *A.add(2) * *B.add(8);
        tmp[3] = *A.add(3) * *B.add(0) + *A.add(4) * *B.add(3) + *A.add(5) * *B.add(6);
        tmp[4] = *A.add(3) * *B.add(1) + *A.add(4) * *B.add(4) + *A.add(5) * *B.add(7);
        tmp[5] = *A.add(3) * *B.add(2) + *A.add(4) * *B.add(5) + *A.add(5) * *B.add(8);
        tmp[6] = *A.add(6) * *B.add(0) + *A.add(7) * *B.add(3) + *A.add(8) * *B.add(6);
        tmp[7] = *A.add(6) * *B.add(1) + *A.add(7) * *B.add(4) + *A.add(8) * *B.add(7);
        tmp[8] = *A.add(6) * *B.add(2) + *A.add(7) * *B.add(5) + *A.add(8) * *B.add(8);
        std::ptr::copy_nonoverlapping(tmp.as_ptr(), res, 9);
    }
}

/// C: mjuu_transposemat (user/user_util.h:103)
/// Calls: mjuu_copyvec
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_transposemat(res: *mut f64, mat: *const f64) {
    // SAFETY: res[9], mat[9] valid (caller contract). Use tmp for aliasing safety.
    unsafe {
        let tmp: [f64; 9] = [
            *mat.add(0), *mat.add(3), *mat.add(6),
            *mat.add(1), *mat.add(4), *mat.add(7),
            *mat.add(2), *mat.add(5), *mat.add(8),
        ];
        std::ptr::copy_nonoverlapping(tmp.as_ptr(), res, 9);
    }
}

/// C: mjuu_localaxis (user/user_util.h:106)
/// Calls: mjuu_mulvecmat, mjuu_quat2mat
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_localaxis(al: *mut f64, ag: *const f64, quat: *const f64) {
    // SAFETY: al[3], ag[3], quat[4] valid (caller contract)
    unsafe {
        let mut mat: [f64; 9] = [0.0; 9];
        let qneg: [f64; 4] = [*quat.add(0), -*quat.add(1), -*quat.add(2), -*quat.add(3)];
        mjuu_quat2mat(mat.as_mut_ptr(), qneg.as_ptr());
        mjuu_mulvecmat(al, ag, mat.as_ptr());
    }
}

/// C: mjuu_localpos (user/user_util.h:109)
/// Calls: mjuu_localaxis
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_localpos(pl: *mut f64, pg: *const f64, pos: *const f64, quat: *const f64) {
    // SAFETY: pl[3], pg[3], pos[3], quat[4] valid (caller contract)
    unsafe {
        let a: [f64; 3] = [
            *pg.add(0) - *pos.add(0),
            *pg.add(1) - *pos.add(1),
            *pg.add(2) - *pos.add(2),
        ];
        mjuu_localaxis(pl, a.as_ptr(), quat);
    }
}

/// C: mjuu_localquat (user/user_util.h:112)
/// Calls: mjuu_mulquat
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_localquat(local: *mut f64, child: *const f64, parent: *const f64) {
    todo!() // mjuu_localquat
}

/// C: mjuu_crossvec (user/user_util.h:115)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_crossvec(a: *mut f64, b: *const f64, c: *const f64) {
    // SAFETY: a, b, c are valid 3-element arrays from caller
    unsafe {
        *a.add(0) = *b.add(1) * *c.add(2) - *b.add(2) * *c.add(1);
        *a.add(1) = *b.add(2) * *c.add(0) - *b.add(0) * *c.add(2);
        *a.add(2) = *b.add(0) * *c.add(1) - *b.add(1) * *c.add(0);
    }
}

/// C: mjuu_makenormal (user/user_util.h:118)
/// Calls: mjuu_crossvec, mjuu_dot3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_makenormal(normal: *mut f64, a: *const type_parameter_0_0, b: *const type_parameter_0_0, c: *const type_parameter_0_0) -> f64 {
    todo!() // mjuu_makenormal
}

/// C: mjuu_z2quat (user/user_util.h:122)
/// Calls: mjuu_crossvec, mjuu_normvec
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_z2quat(quat: *mut f64, vec: *const f64) {
    todo!() // mjuu_z2quat
}

/// C: mjuu_frame2quat (user/user_util.h:125)
/// Calls: mjuu_normvec
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_frame2quat(quat: *mut f64, x: *const f64, y: *const f64, z: *const f64) {
    // SAFETY: quat[4], x[3], y[3], z[3] valid (caller contract)
    unsafe {
        let mat: [f64; 9] = [
            *x.add(0), *y.add(0), *z.add(0),
            *x.add(1), *y.add(1), *z.add(1),
            *x.add(2), *y.add(2), *z.add(2),
        ];
        crate::engine::engine_util_spatial::mju_mat2quat(quat, mat.as_ptr());
    }
}

/// C: mjuu_frameinvert (user/user_util.h:128)
/// Calls: mjuu_localaxis
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_frameinvert(newpos: *mut f64, newquat: *mut f64, oldpos: *const f64, oldquat: *const f64) {
    // SAFETY: newpos[3], newquat[4], oldpos[3], oldquat[4] valid (caller contract)
    unsafe {
        // position
        mjuu_localaxis(newpos, oldpos, oldquat);
        *newpos.add(0) = -*newpos.add(0);
        *newpos.add(1) = -*newpos.add(1);
        *newpos.add(2) = -*newpos.add(2);

        // orientation
        *newquat.add(0) = *oldquat.add(0);
        *newquat.add(1) = -*oldquat.add(1);
        *newquat.add(2) = -*oldquat.add(2);
        *newquat.add(3) = -*oldquat.add(3);
    }
}

/// C: mjuu_frameaccum (user/user_util.h:132)
/// Calls: mjuu_copyvec, mjuu_mulquat, mjuu_mulvecmat, mjuu_quat2mat
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_frameaccum(pos: *mut f64, quat: *mut f64, childpos: *const f64, childquat: *const f64) {
    todo!() // mjuu_frameaccum
}

/// C: mjuu_frameaccumChild (user/user_util.h:136)
/// Calls: mjuu_copyvec, mjuu_frameaccum
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_frameaccum_child(pos: *const f64, quat: *const f64, childpos: *mut f64, childquat: *mut f64) {
    todo!() // mjuu_frameaccumChild
}

/// C: mjuu_frameaccuminv (user/user_util.h:140)
/// Calls: mjuu_copyvec, mjuu_mulquat, mjuu_mulvecmat, mjuu_quat2mat
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_frameaccuminv(pos: *mut f64, quat: *mut f64, childpos: *const f64, childquat: *const f64) {
    todo!() // mjuu_frameaccuminv
}

/// C: mjuu_globalinertia (user/user_util.h:144)
/// Calls: mjuu_quat2mat
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_globalinertia(global: *mut f64, local: *const f64, quat: *const f64) {
    // SAFETY: global[6], local[3], quat[4] valid (caller contract)
    unsafe {
        let mut mat: [f64; 9] = [0.0; 9];
        mjuu_quat2mat(mat.as_mut_ptr(), quat);
        let tmp: [f64; 9] = [
            mat[0] * *local.add(0), mat[3] * *local.add(0), mat[6] * *local.add(0),
            mat[1] * *local.add(1), mat[4] * *local.add(1), mat[7] * *local.add(1),
            mat[2] * *local.add(2), mat[5] * *local.add(2), mat[8] * *local.add(2),
        ];
        *global.add(0) = mat[0] * tmp[0] + mat[1] * tmp[3] + mat[2] * tmp[6];
        *global.add(1) = mat[3] * tmp[1] + mat[4] * tmp[4] + mat[5] * tmp[7];
        *global.add(2) = mat[6] * tmp[2] + mat[7] * tmp[5] + mat[8] * tmp[8];
        *global.add(3) = mat[0] * tmp[1] + mat[1] * tmp[4] + mat[2] * tmp[7];
        *global.add(4) = mat[0] * tmp[2] + mat[1] * tmp[5] + mat[2] * tmp[8];
        *global.add(5) = mat[3] * tmp[2] + mat[4] * tmp[5] + mat[5] * tmp[8];
    }
}

/// C: mjuu_offcenter (user/user_util.h:147)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_offcenter(res: *mut f64, mass: f64, vec: *const f64) {
    // SAFETY: res[6], vec[3] valid (caller contract)
    unsafe {
        *res.add(0) = mass * (*vec.add(1) * *vec.add(1) + *vec.add(2) * *vec.add(2));
        *res.add(1) = mass * (*vec.add(0) * *vec.add(0) + *vec.add(2) * *vec.add(2));
        *res.add(2) = mass * (*vec.add(0) * *vec.add(0) + *vec.add(1) * *vec.add(1));
        *res.add(3) = -mass * *vec.add(0) * *vec.add(1);
        *res.add(4) = -mass * *vec.add(0) * *vec.add(2);
        *res.add(5) = -mass * *vec.add(1) * *vec.add(2);
    }
}

/// C: mjuu_visccoef (user/user_util.h:150)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_visccoef(visccoef: *mut f64, mass: f64, inertia: *const f64, scl: f64) {
    todo!() // mjuu_visccoef
}

/// C: mjuu_rotVecQuat (user/user_util.h:153)
/// Calls: mjuu_copyvec
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_rot_vec_quat(res: *mut f64, vec: *const f64, quat: *const f64) {
    // SAFETY: res[3], vec[3], quat[4] valid (caller contract)
    unsafe {
        // zero vec: zero res
        if *vec.add(0) == 0.0 && *vec.add(1) == 0.0 && *vec.add(2) == 0.0 {
            *res.add(0) = 0.0;
            *res.add(1) = 0.0;
            *res.add(2) = 0.0;
        }
        // null quat: copy vec
        else if *quat.add(0) == 1.0 && *quat.add(1) == 0.0 && *quat.add(2) == 0.0 && *quat.add(3) == 0.0 {
            std::ptr::copy_nonoverlapping(vec, res, 3);
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

/// C: mjuu_updateFrame (user/user_util.h:156)
/// Calls: mjuu_axisAngle2Quat, mjuu_copyvec, mjuu_crossvec, mjuu_dot3, mjuu_frame2quat, mjuu_normvec, mjuu_rotVecQuat
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_update_frame(quat: *mut f64, normal: *mut f64, edge: *const f64, tprv: *const f64, tnxt: *const f64, first: i32) -> f64 {
    // SAFETY: quat[4], normal[3], edge[3], tprv[3], tnxt[3] valid (caller contract)
    unsafe {
        let mut tangent: [f64; 3] = [0.0; 3];
        let mut binormal: [f64; 3] = [0.0; 3];

        // normalize tangent
        for i in 0..3 {
            tangent[i] = *edge.add(i);
        }
        mjuu_normvec(tangent.as_mut_ptr(), 3);

        // compute moving frame
        if first != 0 {
            // use the first vertex binormal for the first edge
            mjuu_crossvec(binormal.as_mut_ptr(), tangent.as_ptr(), tnxt);
            mjuu_normvec(binormal.as_mut_ptr(), 3);

            // compute edge normal given tangent and binormal
            mjuu_crossvec(normal, binormal.as_ptr(), tangent.as_ptr());
            mjuu_normvec(normal, 3);
        } else {
            let mut darboux: [f64; 4] = [0.0; 4];

            // rotate edge normal about the vertex binormal
            mjuu_crossvec(binormal.as_mut_ptr(), tprv, tangent.as_ptr());
            let angle = f64::atan2(
                mjuu_normvec(binormal.as_mut_ptr(), 3),
                mjuu_dot3(tprv, tangent.as_ptr()),
            );
            mjuu_axis_angle2quat(darboux.as_mut_ptr(), binormal.as_ptr(), angle);
            mjuu_rot_vec_quat(normal, normal, darboux.as_ptr());
            mjuu_normvec(normal, 3);

            // compute edge binormal given tangent and normal
            mjuu_crossvec(binormal.as_mut_ptr(), tangent.as_ptr(), normal);
            mjuu_normvec(binormal.as_mut_ptr(), 3);
        }

        // global orientation of the frame
        mjuu_frame2quat(quat, tangent.as_ptr(), normal, binormal.as_ptr());

        // return edge length
        f64::sqrt(mjuu_dot3(edge, edge))
    }
}

/// C: mjuu_eig3 (user/user_util.h:160)
/// Calls: mjuu_mulmat, mjuu_mulquat, mjuu_normvec, mjuu_quat2mat, mjuu_transposemat
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_eig3(eigval: *mut f64, eigvec: *mut f64, quat: *mut f64, mat: *const f64) -> i32 {
    // SAFETY: eigval[3], eigvec[9], quat[4], mat[9] valid (caller contract)
    unsafe {
        const K_EIG_EPS: f64 = 1e-12;
        let mut d: [f64; 9] = [0.0; 9];
        let mut tmp: [f64; 9] = [0.0; 9];
        let mut tmp2: [f64; 9] = [0.0; 9];

        // initialize with unit quaternion
        *quat.add(0) = 1.0;
        *quat.add(1) = 0.0;
        *quat.add(2) = 0.0;
        *quat.add(3) = 0.0;

        // Jacobi iteration
        let mut iter: i32 = 0;
        while iter < 500 {
            // make quaternion matrix eigvec, compute D = eigvec'*mat*eigvec
            mjuu_quat2mat(eigvec, quat);
            mjuu_transposemat(tmp2.as_mut_ptr(), eigvec);
            mjuu_mulmat(tmp.as_mut_ptr(), tmp2.as_ptr(), mat);
            mjuu_mulmat(d.as_mut_ptr(), tmp.as_ptr(), eigvec);

            // assign eigenvalues
            *eigval.add(0) = d[0];
            *eigval.add(1) = d[4];
            *eigval.add(2) = d[8];

            // find max off-diagonal element, set indices
            let (rk, ck, rotk): (usize, usize, usize);
            if f64::abs(d[1]) > f64::abs(d[2]) && f64::abs(d[1]) > f64::abs(d[5]) {
                rk = 0;
                ck = 1;
                rotk = 2;
            } else if f64::abs(d[2]) > f64::abs(d[5]) {
                rk = 0;
                ck = 2;
                rotk = 1;
            } else {
                rk = 1;
                ck = 2;
                rotk = 0;
            }

            // terminate if max off-diagonal element too small
            if f64::abs(d[3 * rk + ck]) < K_EIG_EPS {
                break;
            }

            // 2x2 symmetric Schur decomposition
            let tau = (d[4 * ck] - d[4 * rk]) / (2.0 * d[3 * rk + ck]);
            let t: f64;
            if tau >= 0.0 {
                t = 1.0 / (tau + f64::sqrt(1.0 + tau * tau));
            } else {
                t = -1.0 / (-tau + f64::sqrt(1.0 + tau * tau));
            }
            let c = 1.0 / f64::sqrt(1.0 + t * t);

            // terminate if cosine too close to 1
            if c > 1.0 - K_EIG_EPS {
                break;
            }

            // express rotation as quaternion
            let mut q_rot: [f64; 4] = [0.0; 4];
            q_rot[1] = 0.0;
            q_rot[2] = 0.0;
            q_rot[3] = 0.0;
            q_rot[rotk + 1] = if tau >= 0.0 {
                -f64::sqrt(0.5 - 0.5 * c)
            } else {
                f64::sqrt(0.5 - 0.5 * c)
            };
            if rotk == 1 {
                q_rot[rotk + 1] = -q_rot[rotk + 1];
            }
            q_rot[0] = f64::sqrt(1.0 - q_rot[rotk + 1] * q_rot[rotk + 1]);
            mjuu_normvec(q_rot.as_mut_ptr(), 4);

            // accumulate quaternion rotation
            mjuu_mulquat(quat, quat, q_rot.as_ptr());
            mjuu_normvec(quat, 4);

            iter += 1;
        }

        // sort eigenvalues in decreasing order (bubblesort: 0, 1, 0)
        for j in 0..3i32 {
            let j1 = (j % 2) as usize;

            // only swap if the eigenvalues are different
            if *eigval.add(j1) + K_EIG_EPS < *eigval.add(j1 + 1) {
                // swap eigenvalues
                let t_swap = *eigval.add(j1);
                *eigval.add(j1) = *eigval.add(j1 + 1);
                *eigval.add(j1 + 1) = t_swap;

                // rotate quaternion
                let mut q_swap: [f64; 4] = [0.0; 4];
                q_swap[0] = 0.707106781186548;
                q_swap[1] = 0.0;
                q_swap[2] = 0.0;
                q_swap[3] = 0.0;
                q_swap[(j1 + 2) % 3 + 1] = q_swap[0];
                mjuu_mulquat(quat, quat, q_swap.as_ptr());
                mjuu_normvec(quat, 4);
            }
        }

        // recompute eigvec
        mjuu_quat2mat(eigvec, quat);

        iter
    }
}

/// C: mjuu_eigendecompose (user/user_util.h:166)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_eigendecompose(mat: *mut f64, eigval: *mut f64, eigvec: *mut f64, n: i32) -> i32 {
    todo!() // mjuu_eigendecompose
}

/// C: mjuu_trnVecPose (user/user_util.h:169)
/// Calls: mjuu_rotVecQuat
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_trn_vec_pose(res: *mut f64, pos: *const f64, quat: *const f64, vec: *const f64) {
    // SAFETY: res[3], pos[3], quat[4], vec[3] valid (caller contract)
    unsafe {
        mjuu_rot_vec_quat(res, vec, quat);
        *res.add(0) += *pos.add(0);
        *res.add(1) += *pos.add(1);
        *res.add(2) += *pos.add(2);
    }
}

/// C: mjuu_fullInertia (user/user_util.h:172)
/// Calls: mjuu_copyvec, mjuu_defined, mjuu_eig3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_full_inertia(quat: *mut f64, inertia: *mut f64, fullinertia: *const f64) -> *const i8 {
    todo!() // mjuu_fullInertia
}

/// C: FilePath::IsAbs (user/user_util.h:191)
/// Calls: FilePath::AbsPrefix
#[allow(unused_variables, non_snake_case)]
pub fn file_path_is_abs(self_ptr: *mut FilePath) -> bool {
    todo!() // FilePath::IsAbs
}

/// C: FilePath::AbsPrefix (user/user_util.h:195)
#[allow(unused_variables, non_snake_case)]
pub fn file_path_abs_prefix(self_ptr: *mut FilePath) -> std__string {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut FilePath)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: FilePath::Str (user/user_util.h:198)
#[allow(unused_variables, non_snake_case)]
pub fn file_path_str(self_ptr: *mut FilePath) -> *const std__string {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut FilePath)
    // Previous return: * const i32
    todo!("re-translate: params renamed")
}

/// C: FilePath::StrLower (user/user_util.h:202)
#[allow(unused_variables, non_snake_case)]
pub fn file_path_str_lower(self_ptr: *mut FilePath) -> std__string {
    todo!() // FilePath::StrLower
}

/// C: FilePath::Ext (user/user_util.h:205)
#[allow(unused_variables, non_snake_case)]
pub fn file_path_ext(self_ptr: *mut FilePath) -> std__string {
    todo!() // FilePath::Ext
}

/// C: FilePath::StripExt (user/user_util.h:211)
/// Calls: FilePath::FilePathFast
#[allow(unused_variables, non_snake_case)]
pub fn file_path_strip_ext(self_ptr: *mut FilePath) -> FilePath {
    todo!() // FilePath::StripExt
}

/// C: FilePath::StripPath (user/user_util.h:214)
/// Calls: FilePath::FilePathFast
#[allow(unused_variables, non_snake_case)]
pub fn file_path_strip_path(self_ptr: *mut FilePath) -> FilePath {
    todo!() // FilePath::StripPath
}

/// C: FilePath::Lower (user/user_util.h:217)
/// Calls: FilePath::FilePathFast, FilePath::StrLower
#[allow(unused_variables, non_snake_case)]
pub fn file_path_lower(self_ptr: *mut FilePath) -> FilePath {
    todo!() // FilePath::Lower
}

/// C: FilePath::size (user/user_util.h:220)
#[allow(unused_variables, non_snake_case)]
pub fn file_path_size(self_ptr: *mut FilePath) -> std__size_t {
    todo!() // FilePath::size
}

/// C: FilePath::c_str (user/user_util.h:221)
#[allow(unused_variables, non_snake_case)]
pub fn file_path_c_str(self_ptr: *mut FilePath) -> *const i8 {
    todo!() // FilePath::c_str
}

/// C: FilePath::empty (user/user_util.h:222)
#[allow(unused_variables, non_snake_case)]
pub fn file_path_empty(self_ptr: *mut FilePath) -> bool {
    todo!() // FilePath::empty
}

/// C: FilePath::PathReduce (user/user_util.h:227)
/// Calls: FilePath::AbsPrefix, FilePath::IsSeparator
#[allow(unused_variables, non_snake_case)]
pub fn file_path_path_reduce(str: *const std__string) -> std__string {
    todo!() // FilePath::PathReduce
}

/// C: FilePath::IsSeparator (user/user_util.h:228)
#[allow(unused_variables, non_snake_case)]
pub fn file_path_is_separator(c: i8) -> bool {
    todo!() // FilePath::IsSeparator
}

/// C: FilePath::Combine (user/user_util.h:231)
/// Calls: FilePath::AbsPrefix
#[allow(unused_variables, non_snake_case)]
pub fn file_path_combine(s1: *const std__string, s2: *const std__string) -> std__string {
    todo!() // FilePath::Combine
}

/// C: FilePath::FilePathFast (user/user_util.h:234)
#[allow(unused_variables, non_snake_case)]
pub fn file_path_file_path_fast(str: *const std__string) -> FilePath {
    // NOTE: signature changed from previous IR version
    // Previous params: (str : * const i32)
    // Previous return: FilePath
    todo!("re-translate: params renamed")
}

/// C: FileToMemory (user/user_util.h:259)
/// Calls: mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn file_to_memory(filename: *const i8) -> *const () {
    // NOTE: signature changed from previous IR version
    // Previous params: (filename : * const i8)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: VectorToString (user/user_util.h:262)
#[allow(unused_variables, non_snake_case)]
pub fn vector_to_string(v: *const ()) -> std__string {
    // NOTE: signature changed from previous IR version
    // Previous params: (v : * const i32)
    // Previous return: std__string
    todo!("re-translate: params renamed")
}

/// C: StringToVector (user/user_util.h:265)
/// Calls: IsNullOrSpace, SkipSpace
#[allow(unused_variables, non_snake_case)]
pub fn string_to_vector(cs: *mut i8) -> *const () {
    // NOTE: signature changed from previous IR version
    // Previous params: (cs : * mut i8)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: mjuu_strippath (user/user_util.h:273)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_strippath(filename: string) -> std__string {
    todo!() // mjuu_strippath
}

/// C: mjuu_stripext (user/user_util.h:276)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_stripext(filename: string) -> std__string {
    todo!() // mjuu_stripext
}

/// C: mjuu_getext (user/user_util.h:279)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_getext(filename: string_view) -> std__string {
    todo!() // mjuu_getext
}

/// C: mjuu_isabspath (user/user_util.h:282)
/// Calls: mjp_getResourceProvider
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_isabspath(path: string) -> bool {
    todo!() // mjuu_isabspath
}

/// C: mjuu_combinePaths (user/user_util.h:285)
/// Calls: mjuu_isabspath
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_combine_paths(path1: *const std__string, path2: *const std__string) -> std__string {
    todo!() // mjuu_combinePaths
}

/// C: mjuu_parseContentTypeAttrType (user/user_util.h:290)
/// Calls: mjuu_isValidContentType
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_parse_content_type_attr_type(text: string_view) -> *const () {
    todo!() // mjuu_parseContentTypeAttrType
}

/// C: mjuu_parseContentTypeAttrSubtype (user/user_util.h:293)
/// Calls: mjuu_isValidContentType
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_parse_content_type_attr_subtype(text: string_view) -> *const () {
    todo!() // mjuu_parseContentTypeAttrSubtype
}

/// C: mjuu_extToContentType (user/user_util.h:296)
/// Calls: mjuu_getext
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_ext_to_content_type(filename: string_view) -> std__string {
    todo!() // mjuu_extToContentType
}

/// C: mjuu_dirnamelen (user/user_util.h:299)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_dirnamelen(path: *const i8) -> i32 {
    if path.is_null() {
        return 0;
    }
    // SAFETY: path is a valid null-terminated C string (caller contract)
    unsafe {
        let mut pos: i32 = -1;
        let mut i: i32 = 0;
        while *path.offset(i as isize) != 0 {
            if *path.offset(i as isize) == b'/' as i8 || *path.offset(i as isize) == b'\\' as i8 {
                pos = i;
            }
            i += 1;
        }
        pos + 1
    }
}

