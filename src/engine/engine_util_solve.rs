//! Port of: engine/engine_util_solve.c
//! IR hash: 32301b9dc9774d55
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
    todo!() // mju_cholFactor
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
    // SAFETY: caller guarantees res[n], mat[n*n], vec[n] are valid
    unsafe {
        // copy if source and destination are different
        if res != vec as *mut f64 {
            crate::engine::engine_util_blas::mju_copy(res, vec, n);
        }

        let n = n as usize;

        // forward substitution: solve L*res = vec
        for i in 0..n {
            if i > 0 {
                *res.add(i) -= crate::engine::engine_util_blas::mju_dot(mat.add(i * n), res, i as i32);
            }
            // diagonal
            *res.add(i) /= *mat.add(i * (n + 1));
        }

        // backward substitution: solve L'*res = res
        for i in (0..n).rev() {
            if i < n - 1 {
                for j in (i + 1)..n {
                    *res.add(i) -= *mat.add(j * n + i) * *res.add(j);
                }
            }
            // diagonal
            *res.add(i) /= *mat.add(i * (n + 1));
        }
    }
}

/// C: mju_cholUpdate (engine/engine_util_solve.h:33)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_chol_update(mat: *mut f64, x: *mut f64, n: i32, flg_plus: i32) -> i32 {
    const MJ_MINVAL: f64 = 1E-15_f64;

    // SAFETY: caller guarantees mat[n*n] and x[n] are valid
    unsafe {
        let mut rank: i32 = n;

        for k in 0..n as usize {
            if *x.add(k) != 0.0 {
                // prepare constants
                let Lkk: f64 = *mat.add(k * (n as usize + 1));
                let mut tmp: f64 = Lkk * Lkk
                    + if flg_plus != 0 { *x.add(k) * *x.add(k) } else { -(*x.add(k) * *x.add(k)) };
                if tmp < MJ_MINVAL {
                    tmp = MJ_MINVAL;
                    rank -= 1;
                }
                let r: f64 = f64::sqrt(tmp);
                let c: f64 = r / Lkk;
                let cinv: f64 = 1.0 / c;
                let s: f64 = *x.add(k) / Lkk;

                // update diagonal
                *mat.add(k * (n as usize + 1)) = r;

                // update mat
                if flg_plus != 0 {
                    for i in (k + 1)..n as usize {
                        *mat.add(i * n as usize + k) =
                            (*mat.add(i * n as usize + k) + s * *x.add(i)) * cinv;
                    }
                } else {
                    for i in (k + 1)..n as usize {
                        *mat.add(i * n as usize + k) =
                            (*mat.add(i * n as usize + k) - s * *x.add(i)) * cinv;
                    }
                }

                // update x
                for i in (k + 1)..n as usize {
                    *x.add(i) = c * *x.add(i) - s * *mat.add(i * n as usize + k);
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
    const MJ_MINVAL: f64 = 1E-15_f64;

    // SAFETY: caller guarantees mat is valid banded matrix storage
    unsafe {
        let nsparse = ntotal - ndense;
        let mut mindiag: f64 = -1.0;

        // sparse part
        for j in 0..nsparse {
            // number of non-zeros left of (j,j)
            let width_jj = if j < nband - 1 { j } else { nband - 1 };

            // number of non-zeros below (j,j), sparse part
            let height = if nsparse - j - 1 < nband - 1 { nsparse - j - 1 } else { nband - 1 };

            // address of (j,j)
            let adr_jj = ((j + 1) * nband - 1) as usize;

            // compute L(j,j), before sqrt
            let left_ij = if width_jj > 0 {
                crate::engine::engine_util_blas::mju_dot(
                    mat.add(adr_jj - width_jj as usize),
                    mat.add(adr_jj - width_jj as usize) as *const f64,
                    width_jj)
            } else { 0.0 };
            let mut Ljj = diagadd + diagmul * *mat.add(adr_jj) + *mat.add(adr_jj) - left_ij;

            // update mindiag
            if Ljj < mindiag || mindiag < 0.0 {
                mindiag = Ljj;
            }

            // stop if rank-deficient
            if Ljj < MJ_MINVAL {
                return 0.0;
            }

            // compute Ljj, scale = 1/Ljj
            Ljj = f64::sqrt(Ljj);
            let scale = 1.0 / Ljj;

            // compute L(i,j) for i>j, sparse part
            for i in (j + 1)..=(j + height) {
                // number of non-zeros left of (i,j)
                let width_ij = if j < nband - 1 - i + j { j } else { nband - 1 - i + j };

                // address of (i,j)
                let adr_ij = ((i + 1) * nband - 1 - i + j) as usize;

                // in-place computation of L(i,j)
                let left = if width_ij > 0 {
                    crate::engine::engine_util_blas::mju_dot(
                        mat.add(adr_jj - width_ij as usize),
                        mat.add(adr_ij - width_ij as usize) as *const f64,
                        width_ij)
                } else { 0.0 };
                *mat.add(adr_ij) = scale * (*mat.add(adr_ij) - left);
            }

            // compute L(i,j) for i>j, dense part
            for i in nsparse..ntotal {
                // address of (i,j)
                let adr_ij = (nsparse * nband + (i - nsparse) * ntotal + j) as usize;

                // in-place computation of L(i,j)
                let left = if width_jj > 0 {
                    crate::engine::engine_util_blas::mju_dot(
                        mat.add(adr_jj - width_jj as usize),
                        mat.add(adr_ij - width_jj as usize) as *const f64,
                        width_jj)
                } else { 0.0 };
                *mat.add(adr_ij) = scale * (*mat.add(adr_ij) - left);
            }

            // save L(j,j)
            *mat.add(adr_jj) = Ljj;
        }

        // dense part
        for j in nsparse..ntotal {
            // address of (j,j)
            let adr_jj = (nsparse * nband + (j - nsparse) * ntotal + j) as usize;

            // compute Ljj
            let mut Ljj = diagadd + diagmul * *mat.add(adr_jj) + *mat.add(adr_jj)
                - crate::engine::engine_util_blas::mju_dot(
                    mat.add(adr_jj - j as usize),
                    mat.add(adr_jj - j as usize) as *const f64,
                    j);

            // update mindiag
            if Ljj < mindiag || mindiag < 0.0 {
                mindiag = Ljj;
            }

            // stop if rank-deficient
            if Ljj < MJ_MINVAL {
                return 0.0;
            }

            // compute Ljj, scale = 1/Ljj
            Ljj = f64::sqrt(Ljj);
            let scale = 1.0 / Ljj;

            // compute L(i,j) for i>j
            for i in (j + 1)..ntotal {
                // address of off-diagonal element
                let adr_ij = adr_jj + (ntotal as usize) * ((i - j) as usize);

                // in-place computation of L(i,j)
                *mat.add(adr_ij) = scale * (*mat.add(adr_ij)
                    - crate::engine::engine_util_blas::mju_dot(
                        mat.add(adr_jj - j as usize),
                        mat.add(adr_ij - j as usize) as *const f64,
                        j));
            }

            // save L(j,j)
            *mat.add(adr_jj) = Ljj;
        }

        mindiag
    }
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
    // SAFETY: caller guarantees res[ntotal], mat, vec[ntotal] are valid
    unsafe {
        let ntotal = ntotal as usize;
        let nband = nband as usize;
        let ndense = ndense as usize;
        let nsparse: usize = ntotal - ndense;

        // copy into result if different
        if res != vec as *mut f64 {
            crate::engine::engine_util_blas::mju_copy(res, vec, ntotal as i32);
        }

        //------- forward substitution: solve L*res = vec

        // sparse part
        for i in 0..nsparse {
            // number of non-zeros left of (i,i)
            let width: usize = if i < nband - 1 { i } else { nband - 1 };

            if width > 0 {
                *res.add(i) -= crate::engine::engine_util_blas::mju_dot(
                    mat.add((i + 1) * nband - 1 - width),
                    res.add(i - width),
                    width as i32,
                );
            }

            // diagonal
            *res.add(i) /= *mat.add((i + 1) * nband - 1);
        }

        // dense part
        for i in nsparse..ntotal {
            *res.add(i) -= crate::engine::engine_util_blas::mju_dot(
                mat.add(nsparse * nband + (i - nsparse) * ntotal),
                res,
                i as i32,
            );

            // diagonal
            *res.add(i) /= *mat.add(nsparse * nband + (i - nsparse) * ntotal + i);
        }

        //------- backward substitution: solve L'*res = res

        // dense part
        for i in (nsparse..ntotal).rev() {
            for j in (i + 1)..ntotal {
                *res.add(i) -= *mat.add(nsparse * nband + (j - nsparse) * ntotal + i) * *res.add(j);
            }

            // diagonal
            *res.add(i) /= *mat.add(nsparse * nband + (i - nsparse) * ntotal + i);
        }

        // sparse part
        for i in (0..nsparse).rev() {
            // number of non-zeros below (i,i), sparse part
            let height: usize = if nsparse - 1 - i < nband - 1 { nsparse - 1 - i } else { nband - 1 };

            // sparse rows
            for j in (i + 1)..=(i + height) {
                *res.add(i) -= *mat.add((j + 1) * nband - 1 - (j - i)) * *res.add(j);
            }

            // dense rows
            for j in nsparse..ntotal {
                *res.add(i) -= *mat.add(nsparse * nband + (j - nsparse) * ntotal + i) * *res.add(j);
            }

            // diagonal
            *res.add(i) /= *mat.add((i + 1) * nband - 1);
        }
    }
}

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
    let nsparse = ntotal - ndense;

    // sparse part
    if i < nsparse {
        i * nband + nband - 1
    }
    // dense part
    else {
        nsparse * nband + (i - nsparse) * ntotal + i
    }
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
    // SAFETY: caller guarantees res[2], Ain[4], bin[2], d[2] are valid
    unsafe {
        // scale A,b so that constraint becomes x'*x <= r*r
        let b1 = *bin.add(0) * *d.add(0);
        let b2 = *bin.add(1) * *d.add(1);
        let A11 = *Ain.add(0) * *d.add(0) * *d.add(0);
        let A22 = *Ain.add(3) * *d.add(1) * *d.add(1);
        let A12 = *Ain.add(1) * *d.add(0) * *d.add(1);

        // Newton iteration
        let mut la: f64 = 0.0;
        let mut v1: f64 = 0.0;
        let mut v2: f64 = 0.0;

        for _iter in 0..20 {
            // det(A+la)
            let det = (A11 + la) * (A22 + la) - A12 * A12;

            // check SPD
            if det < 1e-10 {
                *res.add(0) = 0.0;
                *res.add(1) = 0.0;
                return 0;
            }

            // P = inv(A+la)
            let detinv = 1.0 / det;
            let P11 = (A22 + la) * detinv;
            let P22 = (A11 + la) * detinv;
            let P12 = -A12 * detinv;

            // v = -P*b
            v1 = -P11 * b1 - P12 * b2;
            v2 = -P12 * b1 - P22 * b2;

            // val = v'*v - r*r
            let val = v1 * v1 + v2 * v2 - r * r;

            // check convergence
            if val < 1e-10 {
                break;
            }

            // deriv = -2 * v' * P * v
            let deriv = -2.0 * (P11 * v1 * v1 + 2.0 * P12 * v1 * v2 + P22 * v2 * v2);

            // compute update
            let delta = -val / deriv;
            if delta < 1e-10 {
                break;
            }

            la += delta;
        }

        // undo scaling
        *res.add(0) = v1 * *d.add(0);
        *res.add(1) = v2 * *d.add(1);

        if la != 0.0 { 1 } else { 0 }
    }
}

/// C: mju_QCQP3 (engine/engine_util_solve.h:131)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_qcqp3(res: *mut f64, Ain: *const f64, bin: *const f64, d: *const f64, r: f64) -> i32 {
    // SAFETY: caller guarantees res[3], Ain[9], bin[3], d[3] are valid
    unsafe {
        // scale A,b so that constraint becomes x'*x <= r*r
        let b1 = *bin.add(0) * *d.add(0);
        let b2 = *bin.add(1) * *d.add(1);
        let b3 = *bin.add(2) * *d.add(2);
        let A11 = *Ain.add(0) * *d.add(0) * *d.add(0);
        let A22 = *Ain.add(4) * *d.add(1) * *d.add(1);
        let A33 = *Ain.add(8) * *d.add(2) * *d.add(2);
        let A12 = *Ain.add(1) * *d.add(0) * *d.add(1);
        let A13 = *Ain.add(2) * *d.add(0) * *d.add(2);
        let A23 = *Ain.add(5) * *d.add(1) * *d.add(2);

        // Newton iteration
        let mut la: f64 = 0.0;
        let mut v1: f64 = 0.0;
        let mut v2: f64 = 0.0;
        let mut v3: f64 = 0.0;

        for _iter in 0..20 {
            // unscaled P
            let mut P11 = (A22 + la) * (A33 + la) - A23 * A23;
            let mut P22 = (A11 + la) * (A33 + la) - A13 * A13;
            let mut P33 = (A11 + la) * (A22 + la) - A12 * A12;
            let mut P12 = A13 * A23 - A12 * (A33 + la);
            let mut P13 = A12 * A23 - A13 * (A22 + la);
            let mut P23 = A12 * A13 - A23 * (A11 + la);

            // det(A+la)
            let det = (A11 + la) * P11 + A12 * P12 + A13 * P13;

            // check SPD
            if det < 1e-10 {
                *res.add(0) = 0.0;
                *res.add(1) = 0.0;
                *res.add(2) = 0.0;
                return 0;
            }

            // detinv
            let detinv = 1.0 / det;

            // final P
            P11 *= detinv;
            P22 *= detinv;
            P33 *= detinv;
            P12 *= detinv;
            P13 *= detinv;
            P23 *= detinv;

            // v = -P*b
            v1 = -P11 * b1 - P12 * b2 - P13 * b3;
            v2 = -P12 * b1 - P22 * b2 - P23 * b3;
            v3 = -P13 * b1 - P23 * b2 - P33 * b3;

            // val = v'*v - r*r
            let val = v1 * v1 + v2 * v2 + v3 * v3 - r * r;

            // check convergence
            if val < 1e-10 {
                break;
            }

            // deriv = -2 * v' * P * v
            let deriv = -2.0 * (P11 * v1 * v1 + P22 * v2 * v2 + P33 * v3 * v3)
                        - 4.0 * (P12 * v1 * v2 + P13 * v1 * v3 + P23 * v2 * v3);

            // compute update
            let delta = -val / deriv;
            if delta < 1e-10 {
                break;
            }

            la += delta;
        }

        // undo scaling
        *res.add(0) = v1 * *d.add(0);
        *res.add(1) = v2 * *d.add(1);
        *res.add(2) = v3 * *d.add(2);

        if la != 0.0 { 1 } else { 0 }
    }
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

