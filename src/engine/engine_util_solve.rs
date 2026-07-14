//! Port of: engine/engine_util_solve.c
//! IR hash: 47ee20b2bff3660e
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
    // SAFETY: caller guarantees vec[n], mat[n*n] are valid
    unsafe {
        let mut res: f64 = 0.0;
        for i in 0..n as usize {
            // diagonal
            res += *vec.add(i) * *mat.add(n as usize * i + i) * *vec.add(i);
            // off-diagonal
            res += 2.0 * *vec.add(i) * crate::engine::engine_util_blas::mju_dot(
                mat.add(n as usize * i), vec, i as i32);
        }
        res
    }
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
    // SAFETY: caller guarantees res[n], mat[n*n], vec[n] are valid, res != vec
    unsafe {
        for i in 0..n as usize {
            // diagonal + strict lower triangle
            *res.add(i) = *mat.add(n as usize * i + i) * *vec.add(i)
                + crate::engine::engine_util_blas::mju_dot(
                    mat.add(n as usize * i), vec, i as i32);

            // strict upper mirror contribution
            crate::engine::engine_util_blas::mju_add_to_scl(
                res, mat.add(n as usize * i), *vec.add(i), i as i32);
        }
    }
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
    // SAFETY: caller guarantees mat[n*n] is valid
    unsafe {
        let mut rank: i32 = n;

        // in-place Cholesky factorization
        for j in 0..n as usize {
            // compute new diagonal
            let mut tmp = *mat.add(j * (n as usize + 1));
            if j > 0 {
                tmp -= crate::engine::engine_util_blas::mju_dot(
                    mat.add(j * n as usize), mat.add(j * n as usize) as *const f64, j as i32);
            }

            // correct diagonal values below threshold
            if tmp < mindiag {
                tmp = mindiag;
                rank -= 1;
            }

            // save diagonal
            *mat.add(j * (n as usize + 1)) = f64::sqrt(tmp);

            // process off-diagonal entries
            let scale = 1.0 / *mat.add(j * (n as usize + 1));
            for i in (j + 1)..n as usize {
                *mat.add(i * n as usize + j) = (*mat.add(i * n as usize + j)
                    - crate::engine::engine_util_blas::mju_dot(
                        mat.add(i * n as usize), mat.add(j * n as usize) as *const f64, j as i32))
                    * scale;
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
    // SAFETY: caller guarantees sparse matrix pointers are valid. d is unused.
    unsafe {
        let mut rank: i32 = n;

        // backpass over rows
        for r in (0..n as usize).rev() {
            // get rownnz and rowadr for row r
            let nnz = *rownnz.add(r) as usize;
            let adr = *rowadr.add(r) as usize;

            // update row r diagonal
            let mut tmp = *mat.add(adr + nnz - 1);
            if tmp < mindiag {
                tmp = mindiag;
                rank -= 1;
            }
            *mat.add(adr + nnz - 1) = f64::sqrt(tmp);
            let scale = 1.0 / *mat.add(adr + nnz - 1);

            // update row r before diagonal
            for i in 0..(nnz - 1) {
                *mat.add(adr + i) *= scale;
            }

            // update row c<r where mat(r,c)!=0
            for i in 0..(nnz - 1) {
                // get column index
                let c = *colind.add(adr + i) as usize;

                // mat(c,0:c) = mat(c,0:c) - mat(r,c) * mat(r,0:c)
                let nnz_c = crate::engine::engine_util_sparse::mju_combine_sparse(
                    mat.add(*rowadr.add(c) as usize),
                    mat.add(*rowadr.add(r) as usize),
                    1.0,
                    -*mat.add(adr + i),
                    *rownnz.add(c),
                    (i + 1) as i32,
                    colind.add(*rowadr.add(c) as usize),
                    colind.add(*rowadr.add(r) as usize),
                );

                // assign new nnz to row c
                *rownnz.add(c) = nnz_c;
            }
        }

        rank
    }
}

/// C: mju_cholFactorSymbolic (engine/engine_util_solve.h:45)
/// Calls: mj_freeStack, mj_markStack, mj_stackAllocInfo
#[allow(unused_variables, non_snake_case)]
pub fn mju_chol_factor_symbolic(L_colind: *mut i32, L_rownnz: *mut i32, L_rowadr: *mut i32, LT_colind: *mut i32, LT_rownnz: *mut i32, LT_rowadr: *mut i32, LT_map: *mut i32, rownnz: *const i32, rowadr: *const i32, colind: *const i32, n: i32, d: *mut mjData) -> i32 {
    todo!() // mju_cholFactorSymbolic
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
    todo!() // mju_cholFactorNumeric
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
    // SAFETY: caller guarantees all sparse matrix pointers are valid
    unsafe {
        // copy input into result
        crate::engine::engine_util_blas::mju_copy(res, vec, n);

        // vec <- L^-T vec
        for i in (0..n as usize).rev() {
            if *res.add(i) != 0.0 {
                let adr = *rowadr.add(i) as usize;
                let nnz = *rownnz.add(i) as usize;

                // x(i) /= L(i,i)
                *res.add(i) /= *mat.add(adr + nnz - 1);
                let tmp = *res.add(i);

                // x(j) -= L(i,j)*x(i), j=0:i-1
                for j in 0..(nnz - 1) {
                    *res.add(*colind.add(adr + j) as usize) -= *mat.add(adr + j) * tmp;
                }
            }
        }

        // vec <- L^-1 vec
        for i in 0..n as usize {
            let adr = *rowadr.add(i) as usize;
            let nnz = *rownnz.add(i) as usize;

            // x(i) -= sum_j L(i,j)*x(j), j=0:i-1
            if nnz > 1 {
                *res.add(i) -= crate::engine::engine_util_sparse::mju_dot_sparse(
                    mat.add(adr), res, (nnz - 1) as i32, colind.add(adr));
            }

            // x(i) /= L(i,i)
            *res.add(i) /= *mat.add(adr + nnz - 1);
        }
    }
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
    todo!() // mju_cholUpdateSparse
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

/// C: mju_band2Dense (engine/engine_util_solve.h:84)
/// Calls: mju_copy, mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_band2dense(res: *mut f64, mat: *const f64, ntotal: i32, nband: i32, ndense: i32, flg_sym: mjtBool) {
    // SAFETY: caller guarantees res[ntotal*ntotal] and mat are valid
    unsafe {
        let nsparse = ntotal - ndense;

        // clear all
        crate::engine::engine_util_blas::mju_zero(res, ntotal * ntotal);

        // sparse part
        for i in 0..nsparse {
            // number of non-zeros left of (i,i)
            let width = if i < nband - 1 { i } else { nband - 1 };

            // copy data
            crate::engine::engine_util_blas::mju_copy(
                res.add((i * ntotal + i - width) as usize),
                mat.add(((i + 1) * nband - (width + 1)) as usize),
                width + 1,
            );
        }

        // dense part
        for i in nsparse..ntotal {
            crate::engine::engine_util_blas::mju_copy(
                res.add((i * ntotal) as usize),
                mat.add((nsparse * nband + (i - nsparse) * ntotal) as usize),
                i + 1,
            );
        }

        // make symmetric
        if flg_sym._data[0] != 0 {
            for i in 0..ntotal as usize {
                for j in (i + 1)..ntotal as usize {
                    *res.add(i * ntotal as usize + j) = *res.add(j * ntotal as usize + i);
                }
            }
        }
    }
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
    // SAFETY: caller guarantees res and mat are valid for their dimensions
    unsafe {
        let nsparse = ntotal - ndense;

        // sparse part
        for i in 0..nsparse {
            // number of non-zeros left of (i,i)
            let width = if i < nband - 1 { i } else { nband - 1 };

            // copy data
            crate::engine::engine_util_blas::mju_copy(
                res.add(((i + 1) * nband - (width + 1)) as usize),
                mat.add((i * ntotal + i - width) as usize),
                width + 1,
            );
        }

        // dense part
        for i in nsparse..ntotal {
            crate::engine::engine_util_blas::mju_copy(
                res.add((nsparse * nband + (i - nsparse) * ntotal) as usize),
                mat.add((i * ntotal) as usize),
                i + 1,
            );
        }
    }
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
    // SAFETY: caller guarantees all pointers valid for their dimensions
    unsafe {
        let nsparse = ntotal - ndense;

        // handle multiple vectors
        for j in 0..nvec {
            let vec_j = vec.add((ntotal * j) as usize);
            let res_j = res.add((ntotal * j) as usize);

            // sparse part
            for i in 0..nsparse {
                let width = if i + 1 < nband { i + 1 } else { nband };
                let adr = (i * nband + nband - width) as usize;
                let offset = if i - nband + 1 > 0 { i - nband + 1 } else { 0 };
                *res_j.add(i as usize) = crate::engine::engine_util_blas::mju_dot(
                    mat.add(adr), vec_j.add(offset as usize), width);
                if flg_sym._data[0] != 0 {
                    // strict upper triangle
                    crate::engine::engine_util_blas::mju_add_to_scl(
                        res_j.add(offset as usize), mat.add(adr), *vec_j.add(i as usize), width - 1);
                }
            }

            // dense part
            for i in nsparse..ntotal {
                let adr = (nsparse * nband + (i - nsparse) * ntotal) as usize;
                *res_j.add(i as usize) = crate::engine::engine_util_blas::mju_dot(
                    mat.add(adr), vec_j, i + 1);
                if flg_sym._data[0] != 0 {
                    // strict upper triangle
                    crate::engine::engine_util_blas::mju_add_to_scl(
                        res_j, mat.add(adr), *vec_j.add(i as usize), i);
                }
            }
        }
    }
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
    const MJ_MINVAL: f64 = 1E-15_f64;

    // SAFETY: A points to n*n matrix, pivot points to n ints per caller contract
    unsafe {
        for k in 0..n {
            // initialize pivot
            *pivot.add(k as usize) = k;

            // find pivot: max absolute value in column k, rows k..n-1
            let mut maxval = (*A.add((k * n + k) as usize)).abs();
            let mut maxrow = k;
            for i in (k + 1)..n {
                let val = (*A.add((i * n + k) as usize)).abs();
                if val > maxval {
                    maxval = val;
                    maxrow = i;
                }
            }

            // check singularity
            if maxval < MJ_MINVAL {
                return 0;
            }

            // swap rows k and maxrow
            if maxrow != k {
                *pivot.add(k as usize) = maxrow;
                for j in 0..n {
                    let tmp = *A.add((k * n + j) as usize);
                    *A.add((k * n + j) as usize) = *A.add((maxrow * n + j) as usize);
                    *A.add((maxrow * n + j) as usize) = tmp;
                }
            }

            // compute multipliers and update trailing submatrix
            let diaginv = 1.0 / *A.add((k * n + k) as usize);
            for i in (k + 1)..n {
                *A.add((i * n + k) as usize) *= diaginv;
                let a_ik = *A.add((i * n + k) as usize);
                for j in (k + 1)..n {
                    *A.add((i * n + j) as usize) -= a_ik * *A.add((k * n + j) as usize);
                }
            }
        }

        1
    }
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
    // SAFETY: caller guarantees x[n], LU[n*n], b[n], pivot[n] are valid
    unsafe {
        // copy b into x
        crate::engine::engine_util_blas::mju_copy(x, b, n);

        // apply row permutation and forward substitution: solve L*y = P*b
        for i in 0..n as usize {
            // apply pivot swap
            if *pivot.add(i) as usize != i {
                let tmp = *x.add(i);
                *x.add(i) = *x.add(*pivot.add(i) as usize);
                *x.add(*pivot.add(i) as usize) = tmp;
            }

            // subtract known terms
            for j in 0..i {
                *x.add(i) -= *LU.add(i * n as usize + j) * *x.add(j);
            }
        }

        // back substitution: solve U*x = y
        for i in (0..n as usize).rev() {
            for j in (i + 1)..n as usize {
                *x.add(i) -= *LU.add(i * n as usize + j) * *x.add(j);
            }
            *x.add(i) /= *LU.add(i * n as usize + i);
        }
    }
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
    const MJ_MINVAL: f64 = 1E-15_f64;

    // SAFETY: caller guarantees all pointers valid for sparse matrix dimensions
    unsafe {
        let remaining = scratch;

        // set remaining = rownnz
        if !index.is_null() {
            for i in 0..n as usize {
                *remaining.add(i) = *rownnz.add(*index.add(i) as usize);
            }
        } else {
            crate::engine::engine_util_misc::mju_copy_int(remaining, rownnz, n);
        }

        // diagonal elements (i,i)
        for r in (0..n as usize).rev() {
            let i = if !index.is_null() { *index.add(r) as usize } else { r };

            // get address of last remaining element of row i, adjust remaining counter
            let ii = *rowadr.add(i) as usize + *remaining.add(r) as usize - 1;
            *remaining.add(r) -= 1;

            // rows j above i
            for c in (0..r).rev() {
                let j = if !index.is_null() { *index.add(c) as usize } else { c };

                // get address of last remaining element of row j
                let ji = *rowadr.add(j) as usize + *remaining.add(c) as usize - 1;

                // process row j if (j,i) is non-zero
                if *colind.add(ji) as usize == i {
                    // adjust remaining counter
                    *remaining.add(c) -= 1;

                    // (j,i) = (j,i) / (i,i)
                    *LU.add(ji) = *LU.add(ji) / *LU.add(ii);
                    let LUji = *LU.add(ji);

                    // (j,k) = (j,k) - (i,k) * (j,i) for k<i
                    let mut icnt = *rowadr.add(i) as usize;
                    let mut jcnt = *rowadr.add(j) as usize;
                    while jcnt < *rowadr.add(j) as usize + *remaining.add(c) as usize {
                        if *colind.add(icnt) == *colind.add(jcnt) {
                            *LU.add(jcnt) -= *LU.add(icnt) * LUji;
                            jcnt += 1;
                            icnt += 1;
                        } else if *colind.add(icnt) > *colind.add(jcnt) {
                            jcnt += 1;
                        } else {
                            icnt += 1;
                        }
                    }
                }
            }
        }
    }
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
    // SAFETY: caller guarantees all sparse matrix pointers valid
    unsafe {
        // solve (U+I)*res = vec
        for k in (0..n as usize).rev() {
            let i = if !index.is_null() { *index.add(k) as usize } else { k };

            // init: diagonal of (U+I) is 1
            *res.add(i) = *vec.add(i);

            let d1 = *diag.add(i) as usize + 1;
            let nnz = *rownnz.add(i) as usize - d1;
            if nnz > 0 {
                let adr = *rowadr.add(i) as usize + d1;
                *res.add(i) -= crate::engine::engine_util_sparse::mju_dot_sparse(
                    LU.add(adr), res as *const f64, nnz as i32, colind.add(adr));
            }
        }

        // solve L*res(new) = res
        for k in 0..n as usize {
            let i = if !index.is_null() { *index.add(k) as usize } else { k };

            // res[i] -= sum_k<i res[k]*LU(i,k)
            let d = *diag.add(i) as usize;
            let adr = *rowadr.add(i) as usize;
            if d > 0 {
                *res.add(i) -= crate::engine::engine_util_sparse::mju_dot_sparse(
                    LU.add(adr), res as *const f64, d as i32, colind.add(adr));
            }

            // divide by diagonal element of L
            *res.add(i) /= *LU.add(adr + d);
        }
    }
}

/// C: mju_solve3 (engine/engine_util_solve.h:118)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_solve3(x: *mut f64, A: *const f64, b: *const f64) {
    // SAFETY: caller guarantees x[3], A[9], b[3] are valid
    unsafe {
        let mut M: [[f64; 4]; 3] = [
            [*A.add(0), *A.add(1), *A.add(2), *b.add(0)],
            [*A.add(3), *A.add(4), *A.add(5), *b.add(1)],
            [*A.add(6), *A.add(7), *A.add(8), *b.add(2)],
        ];

        for i in 0..3 {
            let pivot = M[i][i];
            for j in i..4 {
                M[i][j] /= pivot;
            }

            for k in 0..3 {
                if k != i {
                    let factor = M[k][i];
                    for j in i..4 {
                        M[k][j] -= factor * M[i][j];
                    }
                }
            }
        }

        *x.add(0) = M[0][3];
        *x.add(1) = M[1][3];
        *x.add(2) = M[2][3];
    }
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
    const EIG_EPS: f64 = 1E-15_f64 * 1000.0;

    // SAFETY: caller guarantees eigval[3], eigvec[9], quat[4], mat[9] are valid
    unsafe {
        let mut D: [f64; 9] = [0.0; 9];
        let mut tmp: [f64; 9] = [0.0; 9];

        // initialize with unit quaternion
        *quat.add(0) = 1.0;
        *quat.add(1) = 0.0;
        *quat.add(2) = 0.0;
        *quat.add(3) = 0.0;

        // Jacobi iteration
        let mut iter: i32 = 0;
        while iter < 500 {
            // make quaternion matrix eigvec, compute D = eigvec'*mat*eigvec
            crate::engine::engine_util_spatial::mju_quat2mat(eigvec, quat);
            crate::engine::engine_inline::mji_mul_mat_t_mat3(tmp.as_mut_ptr(), eigvec, mat);
            crate::engine::engine_inline::mji_mul_mat_mat3(D.as_mut_ptr(), tmp.as_ptr(), eigvec);

            // assign eigenvalues
            *eigval.add(0) = D[0];
            *eigval.add(1) = D[4];
            *eigval.add(2) = D[8];

            // find max off-diagonal element, set indices
            let (rk, ck, rotk): (usize, usize, usize);
            if f64::abs(D[1]) > f64::abs(D[2]) && f64::abs(D[1]) > f64::abs(D[5]) {
                rk = 0;
                ck = 1;
                rotk = 2;
            } else if f64::abs(D[2]) > f64::abs(D[5]) {
                rk = 0;
                ck = 2;
                rotk = 1;
            } else {
                rk = 1;
                ck = 2;
                rotk = 0;
            }

            // terminate if max off-diagonal element too small
            if f64::abs(D[3 * rk + ck]) < EIG_EPS {
                break;
            }

            // 2x2 symmetric Schur decomposition
            let tau = (D[4 * ck] - D[4 * rk]) / (2.0 * D[3 * rk + ck]);
            let t: f64;
            if tau >= 0.0 {
                t = 1.0 / (tau + f64::sqrt(1.0 + tau * tau));
            } else {
                t = -1.0 / (-tau + f64::sqrt(1.0 + tau * tau));
            }
            let c = 1.0 / f64::sqrt(1.0 + t * t);

            // terminate if cosine too close to 1
            if c > 1.0 - EIG_EPS {
                break;
            }

            // express rotation as quaternion
            tmp[1] = 0.0;
            tmp[2] = 0.0;
            tmp[3] = 0.0;
            let val = if tau >= 0.0 {
                -f64::sqrt(0.5 - 0.5 * c)
            } else {
                f64::sqrt(0.5 - 0.5 * c)
            };
            tmp[rotk + 1] = if rotk == 1 { -val } else { val };
            tmp[0] = f64::sqrt(1.0 - tmp[rotk + 1] * tmp[rotk + 1]);
            crate::engine::engine_util_blas::mju_normalize4(tmp.as_mut_ptr());

            // accumulate quaternion rotation
            let mut quat_copy: [f64; 4] = [0.0; 4];
            quat_copy[0] = *quat.add(0);
            quat_copy[1] = *quat.add(1);
            quat_copy[2] = *quat.add(2);
            quat_copy[3] = *quat.add(3);
            crate::engine::engine_util_spatial::mju_mul_quat(quat, quat_copy.as_ptr(), tmp.as_ptr());
            crate::engine::engine_util_blas::mju_normalize4(quat);

            iter += 1;
        }

        // sort eigenvalues in decreasing order (bubble sort: 0, 1, 0)
        for j in 0..3i32 {
            let j1 = (j % 2) as usize;

            // only swap if the eigenvalues are different
            if *eigval.add(j1) + EIG_EPS < *eigval.add(j1 + 1) {
                // swap eigenvalues
                let t = *eigval.add(j1);
                *eigval.add(j1) = *eigval.add(j1 + 1);
                *eigval.add(j1 + 1) = t;

                // rotate quaternion
                tmp[0] = 0.707106781186548_f64;
                tmp[1] = 0.0;
                tmp[2] = 0.0;
                tmp[3] = 0.0;
                tmp[(j1 + 2) % 3 + 1] = tmp[0];
                let mut quat_copy: [f64; 4] = [0.0; 4];
                quat_copy[0] = *quat.add(0);
                quat_copy[1] = *quat.add(1);
                quat_copy[2] = *quat.add(2);
                quat_copy[3] = *quat.add(3);
                crate::engine::engine_util_spatial::mju_mul_quat(quat, quat_copy.as_ptr(), tmp.as_ptr());
                crate::engine::engine_util_blas::mju_normalize4(quat);
            }
        }

        // recompute eigvec
        crate::engine::engine_util_spatial::mju_quat2mat(eigvec, quat);

        iter
    }
}

/// C: mju_QCQP2 (engine/engine_util_solve.h:126)
/// Calls: inside
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
/// Calls: inside
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
    // SAFETY: caller guarantees res[n], Ain[n*n], bin[n], d[n] are valid, n <= 5
    unsafe {
        let mut A: [f64; 25] = [0.0; 25];
        let mut Ala: [f64; 25] = [0.0; 25];
        let mut b: [f64; 5] = [0.0; 5];
        let mut tmp: [f64; 5] = [0.0; 5];

        // check size
        if n > 5 {
            return 0;
        }

        // scale A,b so that constraint becomes x'*x <= r*r
        for i in 0..n as usize {
            b[i] = *bin.add(i) * *d.add(i);

            for j in 0..n as usize {
                A[j + i * n as usize] = *Ain.add(j + i * n as usize) * *d.add(i) * *d.add(j);
            }
        }

        // Newton iteration
        let mut la: f64 = 0.0;
        for _iter in 0..20 {
            // make A+la
            crate::engine::engine_util_blas::mju_copy(
                Ala.as_mut_ptr(), A.as_ptr(), n * n);
            for i in 0..n as usize {
                Ala[i * (n as usize + 1)] += la;
            }

            // factorize, check rank with 1e-10 threshold
            if mju_chol_factor(Ala.as_mut_ptr(), n, 1e-10) < n {
                crate::engine::engine_util_blas::mju_zero(res, n);
                return 0;
            }

            // set res = -Ala \ b
            mju_chol_solve(res, Ala.as_ptr(), b.as_ptr(), n);
            crate::engine::engine_util_blas::mju_scl(res, res as *const f64, -1.0, n);

            // val = res'*res - r*r
            let val = crate::engine::engine_util_blas::mju_dot(res as *const f64, res as *const f64, n) - r * r;

            // check for convergence
            if val < 1e-10 {
                break;
            }

            // deriv = -2 * res' * Ala^-1 * res
            mju_chol_solve(tmp.as_mut_ptr(), Ala.as_ptr(), res as *const f64, n);
            let deriv = -2.0 * crate::engine::engine_util_blas::mju_dot(
                res as *const f64, tmp.as_ptr(), n);

            // compute update, exit if too small
            let delta = -val / deriv;
            if delta < 1e-10 {
                break;
            }

            // update
            la += delta;
        }

        // undo scaling
        for i in 0..n as usize {
            *res.add(i) = *res.add(i) * *d.add(i);
        }

        if la != 0.0 { 1 } else { 0 }
    }
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
    let maxiter: i32 = 100;
    let mingrad: f64 = 1E-16;
    let backtrack: f64 = 0.5;
    let minstep: f64 = 1E-22;
    let armijo: f64 = 0.1;

    mju_box_q_poption(res, R, index, H, g, n, lower, upper,
                      maxiter, mingrad, backtrack, minstep, armijo,
                      std::ptr::null_mut(), 0)
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
    // SAFETY: caller guarantees all double-pointer arguments are valid
    unsafe {
        let n_u = n as usize;
        *res = crate::engine::engine_util_errmem::mju_malloc(std::mem::size_of::<f64>() * n_u) as *mut f64;
        *R = crate::engine::engine_util_errmem::mju_malloc(std::mem::size_of::<f64>() * n_u * (n_u + 7)) as *mut f64;
        *H = crate::engine::engine_util_errmem::mju_malloc(std::mem::size_of::<f64>() * n_u * n_u) as *mut f64;
        *g = crate::engine::engine_util_errmem::mju_malloc(std::mem::size_of::<f64>() * n_u) as *mut f64;

        if !lower.is_null() {
            *lower = crate::engine::engine_util_errmem::mju_malloc(std::mem::size_of::<f64>() * n_u) as *mut f64;
        }
        if !upper.is_null() {
            *upper = crate::engine::engine_util_errmem::mju_malloc(std::mem::size_of::<f64>() * n_u) as *mut f64;
        }
        if !index.is_null() {
            *index = crate::engine::engine_util_errmem::mju_malloc(std::mem::size_of::<i32>() * n_u) as *mut i32;
        }
    }
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
    const MJ_MINVAL: f64 = 1E-15_f64;

    // SAFETY: caller guarantees all pointers valid for n-dimensional QP
    unsafe {
        let mut status: i32 = 0; // mjBOXQP_NO_DESCENT
        let mut factorize: i32 = 1;
        let mut nfree: i32 = n;
        let mut nfactor: i32 = 0;
        let mut improvement: f64 = 0.0;
        let mut value: f64 = 0.0;
        let mut norm2: f64 = 0.0;

        // local scratch vectors, allocate in R
        let scratch = R.add((n * n) as usize);
        let grad = scratch;
        let search = scratch.add(n as usize);
        let candidate = scratch.add(2 * n as usize);
        let temp = scratch.add(3 * n as usize);
        let clamped = scratch.add(4 * n as usize) as *mut i32;
        let oldclamped = scratch.add(5 * n as usize) as *mut i32;

        // if index vector not given, use scratch space
        let index = if index.is_null() {
            scratch.add(6 * n as usize) as *mut i32
        } else {
            index
        };

        // no bounds: return Newton point
        if lower.is_null() && upper.is_null() {
            crate::engine::engine_util_blas::mju_copy(R, H, n * n);
            let rank = mju_chol_factor(R, n, MJ_MINVAL);
            if rank == n {
                mju_chol_solve(res, R as *const f64, g, n);
                crate::engine::engine_util_blas::mju_scl(res, res as *const f64, -1.0, n);
                nfactor = 1;
                status = 4; // mjBOXQP_UNBOUNDED
            } else {
                status = -1; // mjBOXQP_NOT_SPD
            }

            for i in 0..n as usize {
                *index.add(i) = i as i32;
            }
        }
        // have bounds: clamp res
        else {
            for i in 0..n as usize {
                if !lower.is_null() {
                    *res.add(i) = crate::engine::engine_util_misc::mju_max(*res.add(i), *lower.add(i));
                }
                if !upper.is_null() {
                    *res.add(i) = crate::engine::engine_util_misc::mju_min(*res.add(i), *upper.add(i));
                }
            }
        }

        // main loop
        let mut iter: i32 = 0;
        while iter < maxiter {
            if status != 0 {
                break;
            }

            // compute objective: value = 0.5*res'*H*res + res'*g
            value = 0.5 * mul_vec_mat_vec_sym(res as *const f64, H, n)
                + crate::engine::engine_util_blas::mju_dot(res as *const f64, g, n);

            let oldvalue = value;

            // compute gradient
            mul_sym_vec(grad, H, res as *const f64, n);
            crate::engine::engine_util_blas::mju_add_to(grad, g, n);

            // find clamped dimensions
            for i in 0..n as usize {
                let is_lower_clamped = !lower.is_null() && *res.add(i) == *lower.add(i) && *grad.add(i) > 0.0;
                let is_upper_clamped = !upper.is_null() && *res.add(i) == *upper.add(i) && *grad.add(i) < 0.0;
                *clamped.add(i) = if is_lower_clamped || is_upper_clamped { 1 } else { 0 };
            }

            // build index of free dimensions
            nfree = 0;
            for i in 0..n as usize {
                if *clamped.add(i) == 0 {
                    *index.add(nfree as usize) = i as i32;
                    nfree += 1;
                }
            }

            // all dimensions are clamped
            if nfree == 0 {
                status = 5; // mjBOXQP_ALL_CLAMPED
                break;
            }

            // re-factorize if clamped dimensions have changed
            if iter > 0 {
                factorize = 0;
                for i in 0..n as usize {
                    if *clamped.add(i) != *oldclamped.add(i) {
                        factorize = 1;
                        break;
                    }
                }
            }

            // save last clamped
            for i in 0..n as usize {
                *oldclamped.add(i) = *clamped.add(i);
            }

            // get search direction: search = g + H_all,clamped * res_clamped
            for i in 0..n as usize {
                *temp.add(i) = if *clamped.add(i) != 0 { *res.add(i) } else { 0.0 };
            }
            mul_sym_vec(search, H, temp as *const f64, n);
            crate::engine::engine_util_blas::mju_add_to(search, g, n);

            // search = compress_free(search)
            for i in 0..nfree as usize {
                *search.add(i) = *search.add(*index.add(i) as usize);
            }

            // R = compress_free(H)
            if factorize != 0 {
                for i in 0..nfree as usize {
                    for j in 0..=i {
                        *R.add(i * nfree as usize + j) =
                            *H.add(*index.add(i) as usize * n as usize + *index.add(j) as usize);
                    }
                }
            }

            // re-factorize
            let rank = if factorize != 0 { mju_chol_factor(R, nfree, MJ_MINVAL) } else { nfree };
            nfactor += factorize;

            // abort if factorization failed
            if rank != nfree {
                status = -1; // mjBOXQP_NOT_SPD
                break;
            }

            // temp = H_free,free \ search_free
            mju_chol_solve(temp, R as *const f64, search as *const f64, nfree);

            // search_free = expand_free(-temp) - x_free
            crate::engine::engine_util_blas::mju_zero(search, n);
            for i in 0..nfree as usize {
                *search.add(*index.add(i) as usize) = -*temp.add(i) - *res.add(*index.add(i) as usize);
            }

            // squared norm of free gradient
            norm2 = 0.0;
            for i in 0..nfree as usize {
                let grad_i = *grad.add(*index.add(i) as usize);
                norm2 += grad_i * grad_i;
            }

            // small gradient: minimum found
            if norm2 < mingrad {
                status = if nfree == n { 4 } else { 3 }; // UNBOUNDED or TOL_GRAD
                break;
            }

            // sanity check
            let sdotg = crate::engine::engine_util_blas::mju_dot(search as *const f64, grad as *const f64, n);
            if sdotg >= 0.0 {
                break;
            }

            // projected Armijo line search
            let mut step: f64 = 1.0;
            loop {
                // candidate = clamp(x + step*search)
                crate::engine::engine_util_blas::mju_scl(candidate, search as *const f64, step, n);
                crate::engine::engine_util_blas::mju_add_to(candidate, res as *const f64, n);
                for i in 0..n as usize {
                    if !lower.is_null() && *candidate.add(i) < *lower.add(i) {
                        *candidate.add(i) = *lower.add(i);
                    } else if !upper.is_null() && *candidate.add(i) > *upper.add(i) {
                        *candidate.add(i) = *upper.add(i);
                    }
                }

                // new objective value
                value = 0.5 * mul_vec_mat_vec_sym(candidate as *const f64, H, n)
                    + crate::engine::engine_util_blas::mju_dot(candidate as *const f64, g, n);

                // increment and break if step is too small
                step = step * backtrack;
                if step < minstep {
                    status = 2; // mjBOXQP_MAX_LS_ITER
                    break;
                }

                // repeat until relative improvement >= Armijo
                improvement = (value - oldvalue) / (step * sdotg);
                if improvement >= armijo {
                    break;
                }
            }

            // accept candidate
            crate::engine::engine_util_blas::mju_copy(res, candidate as *const f64, n);

            iter += 1;
        }

        // max iterations exceeded
        if iter == maxiter {
            status = 1; // mjBOXQP_MAX_ITER
        }

        // return nfree or -1 if failure
        if status == 0 || status == -1 { -1 } else { nfree }
    }
}

