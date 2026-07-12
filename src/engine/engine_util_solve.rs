//! Port of: engine/engine_util_solve.c
//! IR hash: c6d98e4f4b63b7f2
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mulVecMatVecSym (engine/engine_util_solve.c:1400)
/// Calls: mju_dot
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mul_vec_mat_vec_sym(vec: *const f64, mat: *const f64, n: i32) -> f64 {
    // NOTE: signature changed from previous IR version
    // Previous params: (vec : * const f64, mat : * const f64, n : i32)
    // Previous return: f64
    todo!("re-translate: params renamed")
}

/// C: mulSymVec (engine/engine_util_solve.c:1412)
/// Calls: mju_addToScl, mju_dot
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mul_sym_vec(res: *mut f64, mat: *const f64, vec: *const f64, n: i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (res : * mut f64, mat : * const f64, vec : * const f64, n : i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mju_cholFactor (engine/engine_util_solve.h:27)
/// Calls: mju_dot
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_chol_factor(mat: *mut f64, n: i32, mindiag: f64) -> i32 {
    unsafe {
        // SAFETY: caller guarantees mat points to n*n contiguous f64 elements
        let mut rank: i32 = n;
        for j in 0..n {
            let mut tmp: f64 = *mat.add((j as usize) * ((n + 1) as usize));
            if j != 0 {
                tmp -= crate::engine::engine_util_blas::mju_dot(
                    mat.add((j as usize) * (n as usize)),
                    mat.add((j as usize) * (n as usize)),
                    j,
                );
            }
            if tmp < mindiag {
                tmp = mindiag;
                rank -= 1;
            }
            *mat.add((j as usize) * ((n + 1) as usize)) = tmp.sqrt();
            tmp = 1.0 / *mat.add((j as usize) * ((n + 1) as usize));
            for i in (j + 1)..n {
                *mat.add((i as usize) * (n as usize) + (j as usize)) =
                    (*mat.add((i as usize) * (n as usize) + (j as usize))
                        - crate::engine::engine_util_blas::mju_dot(
                            mat.add((i as usize) * (n as usize)),
                            mat.add((j as usize) * (n as usize)),
                            j,
                        ))
                        * tmp;
            }
        }
        rank
    }
}

/// C: mju_cholSolve (engine/engine_util_solve.h:30)
/// Calls: mju_copy, mju_dot
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_chol_solve(res: *mut f64, mat: *const f64, vec: *const f64, n: i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (res : * mut f64, mat : * const f64, vec : * const f64, n : i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mju_cholUpdate (engine/engine_util_solve.h:33)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_chol_update(mat: *mut f64, x: *mut f64, n: i32, flg_plus: i32) -> i32 {
    const MJ_MINVAL: f64 = 1E-15f64;

    unsafe {
        // SAFETY: caller guarantees mat points to n*n f64 matrix, x points to n f64 elements
        let mut rank: i32 = n;

        for k in 0..n {
            if *x.add(k as usize) != 0.0 {
                let Lkk: f64 = *mat.add((k as usize) * ((n + 1) as usize));
                let mut tmp: f64 = Lkk * Lkk
                    + if flg_plus != 0 {
                        *x.add(k as usize) * *x.add(k as usize)
                    } else {
                        -(*x.add(k as usize) * *x.add(k as usize))
                    };

                if tmp < MJ_MINVAL {
                    tmp = MJ_MINVAL;
                    rank -= 1;
                }

                let r: f64 = tmp.sqrt();
                let c: f64 = r / Lkk;
                let cinv: f64 = 1.0 / c;
                let s: f64 = *x.add(k as usize) / Lkk;

                *mat.add((k as usize) * ((n + 1) as usize)) = r;

                if flg_plus != 0 {
                    for i in (k + 1)..n {
                        *mat.add((i as usize) * (n as usize) + (k as usize)) =
                            (*mat.add((i as usize) * (n as usize) + (k as usize))
                                + s * *x.add(i as usize))
                                * cinv;
                    }
                } else {
                    for i in (k + 1)..n {
                        *mat.add((i as usize) * (n as usize) + (k as usize)) =
                            (*mat.add((i as usize) * (n as usize) + (k as usize))
                                - s * *x.add(i as usize))
                                * cinv;
                    }
                }

                for i in (k + 1)..n {
                    *x.add(i as usize) = c * *x.add(i as usize)
                        - s * *mat.add((i as usize) * (n as usize) + (k as usize));
                }
            }
        }

        rank
    }
}

/// C: mju_cholFactorSparse (engine/engine_util_solve.h:37)
/// Calls: mju_combineSparse
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_chol_factor_sparse(mat: *mut f64, n: i32, mindiag: f64, rownnz: *mut i32, rowadr: *const i32, colind: *mut i32, d: *mut mjData) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (mat : * mut f64, n : i32, mindiag : f64, rownnz : * mut i32, rowadr : * const i32, colind : * mut i32, d : * mut mjData)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: mju_cholFactorSymbolic (engine/engine_util_solve.h:45)
/// Calls: mj_freeStack, mj_markStack, mj_stackAllocInfo
#[allow(unused_variables, non_snake_case)]
pub fn mju_chol_factor_symbolic(L_colind: *mut i32, L_rownnz: *mut i32, L_rowadr: *mut i32, LT_colind: *mut i32, LT_rownnz: *mut i32, LT_rowadr: *mut i32, LT_map: *mut i32, rownnz: *const i32, rowadr: *const i32, colind: *const i32, n: i32, d: *mut mjData) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (L_colind : * mut i32, L_rownnz : * mut i32, L_rowadr : * mut i32, LT_colind : * mut i32, LT_rownnz : * mut i32, LT_rowadr : * mut i32, LT_map : * mut i32, rownnz : * const i32, rowadr : * const i32, colind : * const i32, n : i32, d : * mut mjData)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: mju_cholFactorNumeric (engine/engine_util_solve.h:53)
/// Calls: mj_freeStack, mj_markStack, mj_stackAllocInfo, mju_scatter, mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_chol_factor_numeric(L: *mut f64, n: i32, mindiag: f64, L_rownnz: *const i32, L_rowadr: *const i32, L_colind: *const i32, LT_rownnz: *const i32, LT_rowadr: *const i32, LT_colind: *const i32, LT_map: *const i32, H: *const f64, H_rownnz: *const i32, H_rowadr: *const i32, H_colind: *const i32, d: *mut mjData) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (L : * mut f64, n : i32, mindiag : f64, L_rownnz : * const i32, L_rowadr : * const i32, L_colind : * const i32, LT_rownnz : * const i32, LT_rowadr : * const i32, LT_colind : * const i32, LT_map : * const i32, H : * const f64, H_rownnz : * const i32, H_rowadr : * const i32, H_colind : * const i32, d : * mut mjData)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: mju_cholSolveSparse (engine/engine_util_solve.h:61)
/// Calls: mju_copy, mju_dotSparse
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_chol_solve_sparse(res: *mut f64, mat: *const f64, vec: *const f64, n: i32, rownnz: *const i32, rowadr: *const i32, colind: *const i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (res : * mut f64, mat : * const f64, vec : * const f64, n : i32, rownnz : * const i32, rowadr : * const i32, colind : * const i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mju_cholUpdateSparse (engine/engine_util_solve.h:66)
/// Calls: mj_freeStack, mj_markStack, mj_stackAllocInfo, mju_scatter, mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_chol_update_sparse(mat: *mut f64, x: *const f64, n: i32, flg_plus: i32, rownnz: *const i32, rowadr: *const i32, colind: *const i32, x_nnz: i32, x_ind: *const i32, d: *mut mjData) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (mat : * mut f64, x : * const f64, n : i32, flg_plus : i32, rownnz : * const i32, rowadr : * const i32, colind : * const i32, x_nnz : i32, x_ind : * const i32, d : * mut mjData)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: mju_cholFactorBand (engine/engine_util_solve.h:76)
/// Calls: mju_dot
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_chol_factor_band(mat: *mut f64, ntotal: i32, nband: i32, ndense: i32, diagadd: f64, diagmul: f64) -> f64 {
    // NOTE: signature changed from previous IR version
    // Previous params: (mat : * mut f64, ntotal : i32, nband : i32, ndense : i32, diagadd : f64, diagmul : f64)
    // Previous return: f64
    todo!("re-translate: params renamed")
}

/// C: mju_cholSolveBand (engine/engine_util_solve.h:80)
/// Calls: mju_copy, mju_dot
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_chol_solve_band(res: *mut f64, mat: *const f64, vec: *const f64, ntotal: i32, nband: i32, ndense: i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (res : * mut f64, mat : * const f64, vec : * const f64, ntotal : i32, nband : i32, ndense : i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mju_band2Dense (engine/engine_util_solve.h:84)
/// Calls: mju_copy, mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_band2dense(res: *mut f64, mat: *const f64, ntotal: i32, nband: i32, ndense: i32, flg_sym: mjtBool) {
    // NOTE: signature changed from previous IR version
    // Previous params: (res : * mut f64, mat : * const f64, ntotal : i32, nband : i32, ndense : i32, flg_sym : mjtBool)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mju_dense2Band (engine/engine_util_solve.h:88)
/// Calls: mju_copy
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_dense2band(res: *mut f64, mat: *const f64, ntotal: i32, nband: i32, ndense: i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (res : * mut f64, mat : * const f64, ntotal : i32, nband : i32, ndense : i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mju_bandMulMatVec (engine/engine_util_solve.h:91)
/// Calls: mju_addToScl, mju_dot
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_band_mul_mat_vec(res: *mut f64, mat: *const f64, vec: *const f64, ntotal: i32, nband: i32, ndense: i32, nvec: i32, flg_sym: mjtBool) {
    // NOTE: signature changed from previous IR version
    // Previous params: (res : * mut f64, mat : * const f64, vec : * const f64, ntotal : i32, nband : i32, ndense : i32, nvec : i32, flg_sym : mjtBool)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mju_bandDiag (engine/engine_util_solve.h:95)
#[allow(unused_variables, non_snake_case)]
pub fn mju_band_diag(i: i32, ntotal: i32, nband: i32, ndense: i32) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (i : i32, ntotal : i32, nband : i32, ndense : i32)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: mju_factorLU (engine/engine_util_solve.h:102)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_factor_lu(A: *mut f64, n: i32, pivot: *mut i32) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (A : * mut f64, n : i32, pivot : * mut i32)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: mju_solveLU (engine/engine_util_solve.h:105)
/// Calls: mju_copy
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_solve_lu(x: *mut f64, LU: *const f64, b: *const f64, pivot: *const i32, n: i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (x : * mut f64, LU : * const f64, b : * const f64, pivot : * const i32, n : i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mju_factorLUSparse (engine/engine_util_solve.h:109)
/// Calls: mju_copyInt, mju_message
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_factor_lu_sparse(LU: *mut f64, n: i32, scratch: *mut i32, rownnz: *const i32, rowadr: *const i32, colind: *const i32, index: *const i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (LU : * mut f64, n : i32, scratch : * mut i32, rownnz : * const i32, rowadr : * const i32, colind : * const i32, index : * const i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mju_solveLUSparse (engine/engine_util_solve.h:113)
/// Calls: mju_dotSparse
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_solve_lu_sparse(res: *mut f64, LU: *const f64, vec: *const f64, n: i32, rownnz: *const i32, rowadr: *const i32, diag: *const i32, colind: *const i32, index: *const i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (res : * mut f64, LU : * const f64, vec : * const f64, n : i32, rownnz : * const i32, rowadr : * const i32, diag : * const i32, colind : * const i32, index : * const i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mju_solve3 (engine/engine_util_solve.h:118)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_solve3(x: *mut f64, A: *const f64, b: *const f64) {
    // NOTE: signature changed from previous IR version
    // Previous params: (x : * mut f64, A : * const f64, b : * const f64)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mju_eig3 (engine/engine_util_solve.h:121)
/// Calls: mji_mulMatMat3, mji_mulMatTMat3, mju_mulQuat, mju_normalize4, mju_quat2Mat
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_eig3(eigval: *mut f64, eigvec: *mut f64, quat: *mut f64, mat: *const f64) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (eigval : * mut f64, eigvec : * mut f64, quat : * mut f64, mat : * const f64)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: mju_QCQP2 (engine/engine_util_solve.h:126)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_qcqp2(res: *mut f64, Ain: *const f64, bin: *const f64, d: *const f64, r: f64) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (res : * mut f64, Ain : * const f64, bin : * const f64, d : * const f64, r : f64)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: mju_QCQP3 (engine/engine_util_solve.h:131)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_qcqp3(res: *mut f64, Ain: *const f64, bin: *const f64, d: *const f64, r: f64) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (res : * mut f64, Ain : * const f64, bin : * const f64, d : * const f64, r : f64)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: mju_QCQP (engine/engine_util_solve.h:136)
/// Calls: mju_cholFactor, mju_cholSolve, mju_copy, mju_dot, mju_message, mju_scl, mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_qcqp(res: *mut f64, Ain: *const f64, bin: *const f64, d: *const f64, r: f64, n: i32) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (res : * mut f64, Ain : * const f64, bin : * const f64, d : * const f64, r : f64, n : i32)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: mju_boxQP (engine/engine_util_solve.h:141)
/// Calls: mju_boxQPoption
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_box_qp(res: *mut f64, R: *mut f64, index: *mut i32, H: *const f64, g: *const f64, n: i32, lower: *const f64, upper: *const f64) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (res : * mut f64, R : * mut f64, index : * mut i32, H : * const f64, g : * const f64, n : i32, lower : * const f64, upper : * const f64)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: mju_boxQPmalloc (engine/engine_util_solve.h:146)
/// Calls: mju_malloc
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_box_q_pmalloc(res: *mut *mut f64, R: *mut *mut f64, index: *mut *mut i32, H: *mut *mut f64, g: *mut *mut f64, n: i32, lower: *mut *mut f64, upper: *mut *mut f64) {
    // NOTE: signature changed from previous IR version
    // Previous params: (res : * mut * mut f64, R : * mut * mut f64, index : * mut * mut i32, H : * mut * mut f64, g : * mut * mut f64, n : i32, lower : * mut * mut f64, upper : * mut * mut f64)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mju_boxQPoption (engine/engine_util_solve.h:151)
/// Calls: mju_addTo, mju_cholFactor, mju_cholSolve, mju_copy, mju_dot, mju_max, mju_message, mju_min, mju_scl, mju_zero, mulSymVec, mulVecMatVecSym
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_box_q_poption(res: *mut f64, R: *mut f64, index: *mut i32, H: *const f64, g: *const f64, n: i32, lower: *const f64, upper: *const f64, maxiter: i32, mingrad: f64, backtrack: f64, minstep: f64, armijo: f64, log: *mut i8, logsz: i32) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (res : * mut f64, R : * mut f64, index : * mut i32, H : * const f64, g : * const f64, n : i32, lower : * const f64, upper : * const f64, maxiter : i32, mingrad : f64, backtrack : f64, minstep : f64, armijo : f64, log : * mut i8, logsz : i32)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

