//! Port of: engine/engine_util_solve.c
//! IR hash: 05737965add36adb
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
        let mut i: i32 = 0;
        while i < n {
            // diagonal
            res += *vec.add(i as usize) * *mat.add((n * i + i) as usize) * *vec.add(i as usize);
            // off-diagonal
            res += 2.0 * *vec.add(i as usize) * crate::engine::engine_util_blas::mju_dot(mat.add((n * i) as usize), vec, i);
            i += 1;
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
    // SAFETY: caller guarantees res[n], mat[n*n], vec[n] valid; res must not alias vec
    unsafe {
        let mut i: i32 = 0;
        while i < n {
            // diagonal + strict lower triangle: res[i] = sum_{j<=i} mat[i,j] * vec[j]
            *res.add(i as usize) = *mat.add((n * i + i) as usize) * *vec.add(i as usize)
                + crate::engine::engine_util_blas::mju_dot(mat.add((n * i) as usize), vec, i);

            // strict upper mirror contribution: res[k] += mat[i,k] * vec[i] for k < i
            crate::engine::engine_util_blas::mju_add_to_scl(res, mat.add((n * i) as usize), *vec.add(i as usize), i);
            i += 1;
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
    // WARNING: signature changed — verify body
    // Previous params: (mat : * mut f64, n : i32, mindiag : f64)
    // Previous return: i32
    unsafe { let mut rank : i32 = n ; let mut tmp : f64 ; let mut j : i32 = 0 ; while j < n { tmp = * mat . add ((j * (n + 1)) as usize) ; if j != 0 { tmp -= crate :: engine :: engine_util_blas :: mju_dot (mat . add ((j * n) as usize) , mat . add ((j * n) as usize) , j) ; } if tmp < mindiag { tmp = mindiag ; rank -= 1 ; } * mat . add ((j * (n + 1)) as usize) = tmp . sqrt () ; tmp = 1.0 / * mat . add ((j * (n + 1)) as usize) ; let mut i : i32 = j + 1 ; while i < n { * mat . add ((i * n + j) as usize) = (* mat . add ((i * n + j) as usize) - crate :: engine :: engine_util_blas :: mju_dot (mat . add ((i * n) as usize) , mat . add ((j * n) as usize) , j)) * tmp ; i += 1 ; } j += 1 ; } rank }
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
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut f64, mat : * const f64, vec : * const f64, n : i32)
    // Previous return: ()
    unsafe { if res != vec as * mut f64 { crate :: engine :: engine_util_blas :: mju_copy (res , vec , n) ; } let mut i : i32 = 0 ; while i < n { if i != 0 { * res . add (i as usize) -= crate :: engine :: engine_util_blas :: mju_dot (mat . add ((i * n) as usize) as * const f64 , res as * const f64 , i) ; } * res . add (i as usize) /= * mat . add ((i * (n + 1)) as usize) ; i += 1 ; } let mut i : i32 = n - 1 ; while i >= 0 { if i < n - 1 { let mut j : i32 = i + 1 ; while j < n { * res . add (i as usize) -= * mat . add ((j * n + i) as usize) * * res . add (j as usize) ; j += 1 ; } } * res . add (i as usize) /= * mat . add ((i * (n + 1)) as usize) ; i -= 1 ; } }
}

/// C: mju_cholUpdate (engine/engine_util_solve.h:33)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_chol_update(mat: *mut f64, x: *mut f64, n: i32, flg_plus: i32) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (mat : * mut f64, x : * mut f64, n : i32, flg_plus : i32)
    // Previous return: i32
    unsafe { let mut rank : i32 = n ; let mjMINVAL : f64 = 1e-15_f64 ; let mut k : i32 = 0 ; while k < n { if * x . add (k as usize) != 0.0 { let Lkk : f64 = * mat . add ((k * (n + 1)) as usize) ; let mut tmp : f64 = Lkk * Lkk + (if flg_plus != 0 { * x . add (k as usize) * * x . add (k as usize) } else { - (* x . add (k as usize) * * x . add (k as usize)) }) ; if tmp < mjMINVAL { tmp = mjMINVAL ; rank -= 1 ; } let r : f64 = tmp . sqrt () ; let c : f64 = r / Lkk ; let cinv : f64 = 1.0 / c ; let s : f64 = * x . add (k as usize) / Lkk ; * mat . add ((k * (n + 1)) as usize) = r ; if flg_plus != 0 { let mut i : i32 = k + 1 ; while i < n { * mat . add ((i * n + k) as usize) = (* mat . add ((i * n + k) as usize) + s * * x . add (i as usize)) * cinv ; i += 1 ; } } else { let mut i : i32 = k + 1 ; while i < n { * mat . add ((i * n + k) as usize) = (* mat . add ((i * n + k) as usize) - s * * x . add (i as usize)) * cinv ; i += 1 ; } } let mut i : i32 = k + 1 ; while i < n { * x . add (i as usize) = c * * x . add (i as usize) - s * * mat . add ((i * n + k) as usize) ; i += 1 ; } } k += 1 ; } rank }
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
    let _ = d;
    let mut rank: i32 = n;

    // SAFETY: raw pointer arithmetic mirrors C exactly; caller guarantees mat/rownnz/rowadr/colind
    // are valid with sufficient allocation for sparse Cholesky factor operations.
    unsafe {
        // backpass over rows
        let mut r: i32 = n - 1;
        while r >= 0 {
            // get rownnz and rowadr for row r
            let nnz: i32 = *rownnz.add(r as usize);
            let adr: i32 = *rowadr.add(r as usize);

            // update row r diagonal
            let mut tmp: f64 = *mat.add((adr + nnz - 1) as usize);
            if tmp < mindiag {
                tmp = mindiag;
                rank -= 1;
            }
            *mat.add((adr + nnz - 1) as usize) = tmp.sqrt();
            tmp = 1.0 / *mat.add((adr + nnz - 1) as usize);

            // update row r before diagonal
            let mut i: i32 = 0;
            while i < nnz - 1 {
                *mat.add((adr + i) as usize) *= tmp;
                i += 1;
            }

            // update row c<r where mat(r,c)!=0
            let mut i: i32 = 0;
            while i < nnz - 1 {
                // get column index
                let c: i32 = *colind.add((adr + i) as usize);

                // mat(c,0:c) = mat(c,0:c) - mat(r,c) * mat(r,0:c)
                let nnz_c: i32 = crate::engine::engine_util_sparse::mju_combine_sparse(
                    mat.add(*rowadr.add(c as usize) as usize),
                    mat.add(adr as usize) as *const f64,
                    1.0,
                    -*mat.add((adr + i) as usize),
                    *rownnz.add(c as usize),
                    i + 1,
                    colind.add(*rowadr.add(c as usize) as usize),
                    colind.add(adr as usize) as *const i32,
                );

                // assign new nnz to row c
                *rownnz.add(c as usize) = nnz_c;

                i += 1;
            }

            r -= 1;
        }
    }

    rank
}

/// C: mju_cholFactorSymbolic (engine/engine_util_solve.h:45)
/// Calls: mj_freeStack, mj_markStack, mj_stackAllocInfo
#[allow(unused_variables, non_snake_case)]
pub fn mju_chol_factor_symbolic(L_colind: *mut i32, L_rownnz: *mut i32, L_rowadr: *mut i32, LT_colind: *mut i32, LT_rownnz: *mut i32, LT_rowadr: *mut i32, LT_map: *mut i32, rownnz: *const i32, rowadr: *const i32, colind: *const i32, n: i32, d: *mut mjData) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (L_colind : * mut i32, L_rownnz : * mut i32, L_rowadr : * mut i32, LT_colind : * mut i32, LT_rownnz : * mut i32, LT_rowadr : * mut i32, LT_map : * mut i32, rownnz : * const i32, rowadr : * const i32, colind : * const i32, n : i32, d : * mut mjData)
    // Previous return: i32
    extern "C" { fn mju_cholFactorSymbolic(L_colind : * mut i32 , L_rownnz : * mut i32 , L_rowadr : * mut i32 , LT_colind : * mut i32 , LT_rownnz : * mut i32 , LT_rowadr : * mut i32 , LT_map : * mut i32 , rownnz : * const i32 , rowadr : * const i32 , colind : * const i32 , n : i32 , d : * mut mjData) -> i32 ; } unsafe { mju_cholFactorSymbolic(L_colind , L_rownnz , L_rowadr , LT_colind , LT_rownnz , LT_rowadr , LT_map , rownnz , rowadr , colind , n , d) }
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
    // WARNING: signature changed — verify body
    // Previous params: (L : * mut f64, n : i32, mindiag : f64, L_rownnz : * const i32, L_rowadr : * const i32, L_colind : * const i32, LT_rownnz : * const i32, LT_rowadr : * const i32, LT_colind : * const i32, LT_map : * const i32, H : * const f64, H_rownnz : * const i32, H_rowadr : * const i32, H_colind : * const i32, d : * mut mjData)
    // Previous return: i32
    extern "C" { fn mju_cholFactorNumeric(L : * mut f64 , n : i32 , mindiag : f64 , L_rownnz : * const i32 , L_rowadr : * const i32 , L_colind : * const i32 , LT_rownnz : * const i32 , LT_rowadr : * const i32 , LT_colind : * const i32 , LT_map : * const i32 , H : * const f64 , H_rownnz : * const i32 , H_rowadr : * const i32 , H_colind : * const i32 , d : * mut mjData) -> i32 ; } unsafe { mju_cholFactorNumeric(L , n , mindiag , L_rownnz , L_rowadr , L_colind , LT_rownnz , LT_rowadr , LT_colind , LT_map , H , H_rownnz , H_rowadr , H_colind , d) }
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
    // SAFETY: raw pointer arithmetic mirrors C exactly; caller guarantees valid bounds
    unsafe {
        crate::engine::engine_util_blas::mju_copy(res, vec, n);

        // L^-T pass
        let mut i = n - 1;
        while i >= 0 {
            if *res.add(i as usize) != 0.0 {
                let adr = *rowadr.add(i as usize);
                let nnz = *rownnz.add(i as usize);
                *res.add(i as usize) /= *mat.add((adr + nnz - 1) as usize);
                let tmp = *res.add(i as usize);
                for j in 0..(nnz - 1) {
                    let col = *colind.add((adr + j) as usize);
                    *res.add(col as usize) -= *mat.add((adr + j) as usize) * tmp;
                }
            }
            i -= 1;
        }

        // L^-1 pass
        for i in 0..n {
            let adr = *rowadr.add(i as usize);
            let nnz = *rownnz.add(i as usize);
            if nnz > 1 {
                *res.add(i as usize) -= crate::engine::engine_util_sparse::mju_dot_sparse(
                    mat.add(adr as usize),
                    res as *const f64,
                    nnz - 1,
                    colind.add(adr as usize),
                );
            }
            *res.add(i as usize) /= *mat.add((adr + nnz - 1) as usize);
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
    // WARNING: signature changed — verify body
    // Previous params: (mat : * mut f64, x : * const f64, n : i32, flg_plus : i32, rownnz : * const i32, rowadr : * const i32, colind : * const i32, x_nnz : i32, x_ind : * const i32, d : * mut mjData)
    // Previous return: i32
    extern "C" { fn mju_cholUpdateSparse(mat : * mut f64 , x : * const f64 , n : i32 , flg_plus : i32 , rownnz : * const i32 , rowadr : * const i32 , colind : * const i32 , x_nnz : i32 , x_ind : * const i32 , d : * mut mjData) -> i32 ; } unsafe { mju_cholUpdateSparse(mat , x , n , flg_plus , rownnz , rowadr , colind , x_nnz , x_ind , d) }
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
    // WARNING: signature changed — verify body
    // Previous params: (mat : * mut f64, ntotal : i32, nband : i32, ndense : i32, diagadd : f64, diagmul : f64)
    // Previous return: f64
    unsafe { let mjMINVAL : f64 = 1e-15_f64 ; let nsparse : i32 = ntotal - ndense ; let mut mindiag : f64 = - 1.0 ; let mut j : i32 = 0 ; while j < nsparse { let width_jj : i32 = if j < nband - 1 { j } else { nband - 1 } ; let height : i32 = if nsparse - j - 1 < nband - 1 { nsparse - j - 1 } else { nband - 1 } ; let adr_jj : i32 = (j + 1) * nband - 1 ; let mut left_ij : f64 = if width_jj > 0 { crate :: engine :: engine_util_blas :: mju_dot (mat . add ((adr_jj - width_jj) as usize) , mat . add ((adr_jj - width_jj) as usize) , width_jj) } else { 0.0 } ; let mut Ljj : f64 = diagadd + diagmul * * mat . add (adr_jj as usize) + * mat . add (adr_jj as usize) - left_ij ; if Ljj < mindiag || mindiag < 0.0 { mindiag = Ljj ; } if Ljj < mjMINVAL { return 0.0 ; } Ljj = Ljj . sqrt () ; let scale : f64 = 1.0 / Ljj ; let mut i : i32 = j + 1 ; while i <= j + height { let width_ij : i32 = if j < nband - 1 - i + j { j } else { nband - 1 - i + j } ; let adr_ij : i32 = (i + 1) * nband - 1 - i + j ; left_ij = if width_ij > 0 { crate :: engine :: engine_util_blas :: mju_dot (mat . add ((adr_jj - width_ij) as usize) , mat . add ((adr_ij - width_ij) as usize) , width_ij) } else { 0.0 } ; * mat . add (adr_ij as usize) = scale * (* mat . add (adr_ij as usize) - left_ij) ; i += 1 ; } let mut i : i32 = nsparse ; while i < ntotal { let adr_ij : i32 = nsparse * nband + (i - nsparse) * ntotal + j ; left_ij = if width_jj > 0 { crate :: engine :: engine_util_blas :: mju_dot (mat . add ((adr_jj - width_jj) as usize) , mat . add ((adr_ij - width_jj) as usize) , width_jj) } else { 0.0 } ; * mat . add (adr_ij as usize) = scale * (* mat . add (adr_ij as usize) - left_ij) ; i += 1 ; } * mat . add (adr_jj as usize) = Ljj ; j += 1 ; } let mut j : i32 = nsparse ; while j < ntotal { let adr_jj : i32 = nsparse * nband + (j - nsparse) * ntotal + j ; let mut Ljj : f64 = diagadd + diagmul * * mat . add (adr_jj as usize) + * mat . add (adr_jj as usize) - crate :: engine :: engine_util_blas :: mju_dot (mat . add ((adr_jj - j) as usize) , mat . add ((adr_jj - j) as usize) , j) ; if Ljj < mindiag || mindiag < 0.0 { mindiag = Ljj ; } if Ljj < mjMINVAL { return 0.0 ; } Ljj = Ljj . sqrt () ; let scale : f64 = 1.0 / Ljj ; let mut i : i32 = j + 1 ; while i < ntotal { let adr_ij : i32 = adr_jj + ntotal * (i - j) ; * mat . add (adr_ij as usize) = scale * (* mat . add (adr_ij as usize) - crate :: engine :: engine_util_blas :: mju_dot (mat . add ((adr_jj - j) as usize) , mat . add ((adr_ij - j) as usize) , j)) ; i += 1 ; } * mat . add (adr_jj as usize) = Ljj ; j += 1 ; } mindiag }
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
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut f64, mat : * const f64, vec : * const f64, ntotal : i32, nband : i32, ndense : i32)
    // Previous return: ()
    unsafe { let nsparse : i32 = ntotal - ndense ; if res != vec as * mut f64 { crate :: engine :: engine_util_blas :: mju_copy (res , vec , ntotal) ; } let mut i : i32 = 0 ; while i < nsparse { let width : i32 = if i < nband - 1 { i } else { nband - 1 } ; if width > 0 { * res . add (i as usize) -= crate :: engine :: engine_util_blas :: mju_dot (mat . add (((i + 1) * nband - 1 - width) as usize) , res . add ((i - width) as usize) as * const f64 , width) ; } * res . add (i as usize) /= * mat . add (((i + 1) * nband - 1) as usize) ; i += 1 ; } let mut i : i32 = nsparse ; while i < ntotal { * res . add (i as usize) -= crate :: engine :: engine_util_blas :: mju_dot (mat . add ((nsparse * nband + (i - nsparse) * ntotal) as usize) , res as * const f64 , i) ; * res . add (i as usize) /= * mat . add ((nsparse * nband + (i - nsparse) * ntotal + i) as usize) ; i += 1 ; } let mut i : i32 = ntotal - 1 ; while i >= nsparse { let mut j : i32 = i + 1 ; while j < ntotal { * res . add (i as usize) -= * mat . add ((nsparse * nband + (j - nsparse) * ntotal + i) as usize) * * res . add (j as usize) ; j += 1 ; } * res . add (i as usize) /= * mat . add ((nsparse * nband + (i - nsparse) * ntotal + i) as usize) ; i -= 1 ; } let mut i : i32 = nsparse - 1 ; while i >= 0 { let height : i32 = if nsparse - 1 - i < nband - 1 { nsparse - 1 - i } else { nband - 1 } ; let mut j : i32 = i + 1 ; while j <= i + height { * res . add (i as usize) -= * mat . add (((j + 1) * nband - 1 - (j - i)) as usize) * * res . add (j as usize) ; j += 1 ; } let mut j : i32 = nsparse ; while j < ntotal { * res . add (i as usize) -= * mat . add ((nsparse * nband + (j - nsparse) * ntotal + i) as usize) * * res . add (j as usize) ; j += 1 ; } * res . add (i as usize) /= * mat . add (((i + 1) * nband - 1) as usize) ; i -= 1 ; } }
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
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut f64, mat : * const f64, ntotal : i32, nband : i32, ndense : i32, flg_sym : mjtBool)
    // Previous return: ()
    unsafe { let nblock : i32 = ntotal - ndense ; crate :: engine :: engine_util_blas :: mju_zero (res , ntotal * ntotal) ; let mut i : i32 = 0 ; while i < nblock { let width : i32 = mju_band_diag (i , ntotal , nband , ndense) ; let mut j : i32 = 0 ; while j < width { * res . add (((i + j) * ntotal + i) as usize) = * mat . add ((i * nband + j) as usize) ; j += 1 ; } let mut j : i32 = 0 ; while j < ndense { * res . add (((nblock + j) * ntotal + i) as usize) = * mat . add ((nblock * nband + j * nblock + i) as usize) ; j += 1 ; } i += 1 ; } let mut i : i32 = 0 ; while i < ndense { let mut j : i32 = 0 ; while j <= i { * res . add (((nblock + i) * ntotal + nblock + j) as usize) = * mat . add ((nblock * nband + ndense * nblock + i * ndense + j) as usize) ; j += 1 ; } i += 1 ; } let flg_sym_i32 : i32 = * (& flg_sym as * const mjtBool as * const i32) ; if flg_sym_i32 != 0 { let mut i : i32 = 0 ; while i < ntotal { let mut j : i32 = i + 1 ; while j < ntotal { * res . add ((i * ntotal + j) as usize) = * res . add ((j * ntotal + i) as usize) ; j += 1 ; } i += 1 ; } } }
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
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut f64, mat : * const f64, ntotal : i32, nband : i32, ndense : i32)
    // Previous return: ()
    unsafe { let nblock : i32 = ntotal - ndense ; crate :: engine :: engine_util_blas :: mju_zero (res , nblock * nband + ndense * nblock + ndense * ndense) ; let mut i : i32 = 0 ; while i < nblock { let width : i32 = mju_band_diag (i , ntotal , nband , ndense) ; let mut j : i32 = 0 ; while j < width { * res . add ((i * nband + j) as usize) = * mat . add (((i + j) * ntotal + i) as usize) ; j += 1 ; } let mut j : i32 = 0 ; while j < ndense { * res . add ((nblock * nband + j * nblock + i) as usize) = * mat . add (((nblock + j) * ntotal + i) as usize) ; j += 1 ; } i += 1 ; } let mut i : i32 = 0 ; while i < ndense { let mut j : i32 = 0 ; while j <= i { * res . add ((nblock * nband + ndense * nblock + i * ndense + j) as usize) = * mat . add (((nblock + i) * ntotal + nblock + j) as usize) ; j += 1 ; } i += 1 ; } }
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
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut f64, mat : * const f64, vec : * const f64, ntotal : i32, nband : i32, ndense : i32, nvec : i32, flg_sym : mjtBool)
    // Previous return: ()
    unsafe { let nblock : i32 = ntotal - ndense ; crate :: engine :: engine_util_blas :: mju_zero (res , ntotal * nvec) ; let flg_sym_i32 : i32 = * (& flg_sym as * const mjtBool as * const i32) ; let mut iv : i32 = 0 ; while iv < nvec { let vec_iv : * const f64 = vec . add ((iv * ntotal) as usize) ; let res_iv : * mut f64 = res . add ((iv * ntotal) as usize) ; let mut i : i32 = 0 ; while i < nblock { let width : i32 = mju_band_diag (i , ntotal , nband , ndense) ; let mut j : i32 = 0 ; while j < width { * res_iv . add (i as usize) += * mat . add ((i * nband + j) as usize) * * vec_iv . add ((i + j) as usize) ; if j > 0 { * res_iv . add ((i + j) as usize) += * mat . add ((i * nband + j) as usize) * * vec_iv . add (i as usize) ; } j += 1 ; } let mut j : i32 = 0 ; while j < ndense { let val : f64 = * mat . add ((nblock * nband + j * nblock + i) as usize) ; * res_iv . add (i as usize) += val * * vec_iv . add ((nblock + j) as usize) ; * res_iv . add ((nblock + j) as usize) += val * * vec_iv . add (i as usize) ; j += 1 ; } i += 1 ; } let mut i : i32 = 0 ; while i < ndense { let mut j : i32 = 0 ; while j <= i { let val : f64 = * mat . add ((nblock * nband + ndense * nblock + i * ndense + j) as usize) ; * res_iv . add ((nblock + i) as usize) += val * * vec_iv . add ((nblock + j) as usize) ; if j < i { * res_iv . add ((nblock + j) as usize) += val * * vec_iv . add ((nblock + i) as usize) ; } j += 1 ; } i += 1 ; } if flg_sym_i32 == 0 { let mut i : i32 = 0 ; while i < nblock { let width : i32 = mju_band_diag (i , ntotal , nband , ndense) ; let mut j : i32 = 1 ; while j < width { * res_iv . add (i as usize) += * mat . add (((i + j) * nband) as usize) * * vec_iv . add ((i + j) as usize) ; j += 1 ; } i += 1 ; } } iv += 1 ; } }
}

/// C: mju_bandDiag (engine/engine_util_solve.h:95)
#[allow(unused_variables, non_snake_case)]
pub fn mju_band_diag(i: i32, ntotal: i32, nband: i32, ndense: i32) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (i : i32, ntotal : i32, nband : i32, ndense : i32)
    // Previous return: i32
    if i >= ntotal - ndense { return ntotal - i ; } let a = nband ; let b = ntotal - ndense - i ; if a < b { a } else { b }
}

/// C: mju_factorLU (engine/engine_util_solve.h:102)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_factor_lu(A: *mut f64, n: i32, pivot: *mut i32) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (A : * mut f64, n : i32, pivot : * mut i32)
    // Previous return: i32
    unsafe { let mjMINVAL : f64 = 1e-15_f64 ; let mut i : i32 = 0 ; while i < n { * pivot . add (i as usize) = i ; i += 1 ; } let mut k : i32 = 0 ; while k < n { let mut maxval : f64 = 0.0 ; let mut maxind : i32 = k ; let mut i : i32 = k ; while i < n { let val : f64 = (* A . add ((i * n + k) as usize)) . abs () ; if val > maxval { maxval = val ; maxind = i ; } i += 1 ; } if maxval < mjMINVAL { return 0 ; } if maxind != k { let tmp_pivot : i32 = * pivot . add (k as usize) ; * pivot . add (k as usize) = * pivot . add (maxind as usize) ; * pivot . add (maxind as usize) = tmp_pivot ; let mut j : i32 = 0 ; while j < n { let tmp : f64 = * A . add ((k * n + j) as usize) ; * A . add ((k * n + j) as usize) = * A . add ((maxind * n + j) as usize) ; * A . add ((maxind * n + j) as usize) = tmp ; j += 1 ; } } let mut i : i32 = k + 1 ; while i < n { * A . add ((i * n + k) as usize) /= * A . add ((k * n + k) as usize) ; let mut j : i32 = k + 1 ; while j < n { * A . add ((i * n + j) as usize) -= * A . add ((i * n + k) as usize) * * A . add ((k * n + j) as usize) ; j += 1 ; } i += 1 ; } k += 1 ; } 1 }
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
    // WARNING: signature changed — verify body
    // Previous params: (x : * mut f64, LU : * const f64, b : * const f64, pivot : * const i32, n : i32)
    // Previous return: ()
    unsafe { let mut i : i32 = 0 ; while i < n { * x . add (i as usize) = * b . add (* pivot . add (i as usize) as usize) ; i += 1 ; } let mut i : i32 = 0 ; while i < n { let mut j : i32 = 0 ; while j < i { * x . add (i as usize) -= * LU . add ((i * n + j) as usize) * * x . add (j as usize) ; j += 1 ; } i += 1 ; } let mut i : i32 = n - 1 ; while i >= 0 { let mut j : i32 = i + 1 ; while j < n { * x . add (i as usize) -= * LU . add ((i * n + j) as usize) * * x . add (j as usize) ; j += 1 ; } * x . add (i as usize) /= * LU . add ((i * n + i) as usize) ; i -= 1 ; } }
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
    const MJMINVAL: f64 = 1e-15;
    let remaining: *mut i32 = scratch;

    // SAFETY: all pointers valid per caller contract, indices within bounds per sparse matrix invariants
    unsafe {
    // set remaining = rownnz
    if !index.is_null() {
        let mut i: i32 = 0;
        while i < n {
            *remaining.add(i as usize) = *rownnz.add(*index.add(i as usize) as usize);
            i += 1;
        }
    } else {
        crate::engine::engine_util_misc::mju_copy_int(remaining, rownnz, n);
    }

    // diagonal elements (i,i)
    let mut r: i32 = n - 1;
    while r >= 0 {
        let i: i32 = if !index.is_null() { *index.add(r as usize) } else { r };

        // get address of last remaining element of row i, adjust remaining counter
        let ii: i32 = *rowadr.add(i as usize) + *remaining.add(r as usize) - 1;
        *remaining.add(r as usize) -= 1;

        // make sure ii is on diagonal
        if *colind.add(ii as usize) != i {
            crate::engine::engine_util_errmem::mju_error(
                b"missing diagonal element\0".as_ptr() as *const i8);
        }

        // make sure diagonal is not too small
        if (*LU.add(ii as usize)).abs() < MJMINVAL {
            crate::engine::engine_util_errmem::mju_error(
                b"diagonal element too small\0".as_ptr() as *const i8);
        }

        // rows j above i
        let mut c: i32 = r - 1;
        while c >= 0 {
            let j: i32 = if !index.is_null() { *index.add(c as usize) } else { c };

            // get address of last remaining element of row j
            let ji: i32 = *rowadr.add(j as usize) + *remaining.add(c as usize) - 1;

            // process row j if (j,i) is non-zero
            if *colind.add(ji as usize) == i {
                // adjust remaining counter
                *remaining.add(c as usize) -= 1;

                // (j,i) = (j,i) / (i,i)
                *LU.add(ji as usize) = *LU.add(ji as usize) / *LU.add(ii as usize);
                let LUji: f64 = *LU.add(ji as usize);

                // (j,k) = (j,k) - (i,k) * (j,i) for k<i; handle incompatible sparsity
                let mut icnt: i32 = *rowadr.add(i as usize);
                let mut jcnt: i32 = *rowadr.add(j as usize);
                while jcnt < *rowadr.add(j as usize) + *remaining.add(c as usize) {
                    // both non-zero
                    if *colind.add(icnt as usize) == *colind.add(jcnt as usize) {
                        *LU.add(jcnt as usize) -= *LU.add(icnt as usize) * LUji;
                        jcnt += 1;
                        icnt += 1;
                    }
                    // only (j,k) non-zero
                    else if *colind.add(icnt as usize) > *colind.add(jcnt as usize) {
                        jcnt += 1;
                    }
                    // only (i,k) non-zero
                    else {
                        crate::engine::engine_util_errmem::mju_error(
                            b"requires fill-in\0".as_ptr() as *const i8);
                    }
                }

                // make sure both rows fully processed
                if icnt != *rowadr.add(i as usize) + *remaining.add(r as usize)
                    || jcnt != *rowadr.add(j as usize) + *remaining.add(c as usize) {
                    crate::engine::engine_util_errmem::mju_error(
                        b"row processing incomplete\0".as_ptr() as *const i8);
                }
            }
            c -= 1;
        }
        r -= 1;
    }

    // make sure remaining points to diagonal
    let mut r: i32 = 0;
    while r < n {
        let i: i32 = if !index.is_null() { *index.add(r as usize) } else { r };
        if *remaining.add(r as usize) < 0
            || *colind.add((*rowadr.add(i as usize) + *remaining.add(r as usize)) as usize) != i {
            crate::engine::engine_util_errmem::mju_error(
                b"unexpected sparse matrix structure\0".as_ptr() as *const i8);
        }
        r += 1;
    }
    } // unsafe
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
    // SAFETY: all pointers valid. res/vec have n elements. LU is sparse matrix.
    // index may be null (use identity permutation).
    unsafe {
        // solve (U+I)*res = vec
        let mut k: i32 = n - 1;
        while k >= 0 {
            let i = if !index.is_null() { *index.add(k as usize) } else { k };

            // init: diagonal of (U+I) is 1
            *res.add(i as usize) = *vec.add(i as usize);

            let d1 = *diag.add(i as usize) + 1;
            let nnz = *rownnz.add(i as usize) - d1;
            if nnz > 0 {
                let adr = *rowadr.add(i as usize) + d1;
                *res.add(i as usize) -= crate::engine::engine_util_sparse::mju_dot_sparse(
                    LU.add(adr as usize), res, nnz, colind.add(adr as usize));
            }
            k -= 1;
        }

        // solve L*res(new) = res
        k = 0;
        while k < n {
            let i = if !index.is_null() { *index.add(k as usize) } else { k };

            // res[i] -= sum_k<i res[k]*LU(i,k)
            let d = *diag.add(i as usize);
            let adr = *rowadr.add(i as usize);
            if d > 0 {
                *res.add(i as usize) -= crate::engine::engine_util_sparse::mju_dot_sparse(
                    LU.add(adr as usize), res, d, colind.add(adr as usize));
            }

            // divide by diagonal element of L
            *res.add(i as usize) /= *LU.add((adr + d) as usize);
            k += 1;
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
    // WARNING: signature changed — verify body
    // Previous params: (x : * mut f64, A : * const f64, b : * const f64)
    // Previous return: ()
    unsafe { let mut M : [[f64 ; 4] ; 3] = [[* A . add (0) , * A . add (1) , * A . add (2) , * b . add (0)] , [* A . add (3) , * A . add (4) , * A . add (5) , * b . add (1)] , [* A . add (6) , * A . add (7) , * A . add (8) , * b . add (2)] ,] ; for i in 0 .. 3_usize { let pivot = M [i] [i] ; for j in i .. 4_usize { M [i] [j] /= pivot ; } for k in 0 .. 3_usize { if k != i { let factor = M [k] [i] ; for j in i .. 4_usize { M [k] [j] -= factor * M [i] [j] ; } } } } * x . add (0) = M [0] [3] ; * x . add (1) = M [1] [3] ; * x . add (2) = M [2] [3] ; }
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
    // WARNING: signature changed — verify body
    // Previous params: (eigval : * mut f64, eigvec : * mut f64, quat : * mut f64, mat : * const f64)
    // Previous return: i32
    unsafe { const eigEPS : f64 = 1e-15 * 1000.0 ; let mut D : [f64 ; 9] = [0.0 ; 9] ; let mut tmp : [f64 ; 9] = [0.0 ; 9] ; let mut tau : f64 ; let mut t : f64 ; let mut c : f64 ; let mut iter : i32 ; let mut rk : i32 ; let mut ck : i32 ; let mut rotk : i32 ; * quat . add (0) = 1.0 ; * quat . add (1) = 0.0 ; * quat . add (2) = 0.0 ; * quat . add (3) = 0.0 ; iter = 0 ; while iter < 500 { crate :: engine :: engine_util_spatial :: mju_quat2mat (eigvec , quat) ; crate :: engine :: engine_util_blas :: mju_mul_mat_t_mat3 (tmp . as_mut_ptr () , eigvec , mat) ; crate :: engine :: engine_util_blas :: mju_mul_mat_mat3 (D . as_mut_ptr () , tmp . as_ptr () , eigvec) ; * eigval . add (0) = D [0] ; * eigval . add (1) = D [4] ; * eigval . add (2) = D [8] ; if D [1] . abs () > D [2] . abs () && D [1] . abs () > D [5] . abs () { rk = 0 ; ck = 1 ; rotk = 2 ; } else if D [2] . abs () > D [5] . abs () { rk = 0 ; ck = 2 ; rotk = 1 ; } else { rk = 1 ; ck = 2 ; rotk = 0 ; } if D [(3 * rk + ck) as usize] . abs () < eigEPS { break ; } tau = (D [(4 * ck) as usize] - D [(4 * rk) as usize]) / (2.0 * D [(3 * rk + ck) as usize]) ; if tau >= 0.0 { t = 1.0 / (tau + (1.0 + tau * tau) . sqrt ()) ; } else { t = - 1.0 / (- tau + (1.0 + tau * tau) . sqrt ()) ; } c = 1.0 / (1.0 + t * t) . sqrt () ; if c > 1.0 - eigEPS { break ; } tmp [1] = 0.0 ; tmp [2] = 0.0 ; tmp [3] = 0.0 ; tmp [(rotk + 1) as usize] = if tau >= 0.0 { - (0.5 - 0.5 * c) . sqrt () } else { (0.5 - 0.5 * c) . sqrt () } ; if rotk == 1 { tmp [(rotk + 1) as usize] = - tmp [(rotk + 1) as usize] ; } tmp [0] = (1.0 - tmp [(rotk + 1) as usize] * tmp [(rotk + 1) as usize]) . sqrt () ; crate :: engine :: engine_util_blas :: mju_normalize4 (tmp . as_mut_ptr ()) ; crate :: engine :: engine_util_spatial :: mju_mul_quat (quat , quat as * const f64 , tmp . as_ptr ()) ; crate :: engine :: engine_util_blas :: mju_normalize4 (quat) ; iter += 1 ; } for j in 0 .. 3_i32 { let j1 = (j % 2) as usize ; if * eigval . add (j1) + eigEPS < * eigval . add (j1 + 1) { t = * eigval . add (j1) ; * eigval . add (j1) = * eigval . add (j1 + 1) ; * eigval . add (j1 + 1) = t ; tmp [0] = 0.707106781186548 ; tmp [1] = 0.0 ; tmp [2] = 0.0 ; tmp [3] = 0.0 ; tmp [((j1 as i32 + 2) % 3 + 1) as usize] = tmp [0] ; crate :: engine :: engine_util_spatial :: mju_mul_quat (quat , quat as * const f64 , tmp . as_ptr ()) ; crate :: engine :: engine_util_blas :: mju_normalize4 (quat) ; } } crate :: engine :: engine_util_spatial :: mju_quat2mat (eigvec , quat) ; iter }
}

/// C: mju_QCQP2 (engine/engine_util_solve.h:126)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_qcqp2(res: *mut f64, Ain: *const f64, bin: *const f64, d: *const f64, r: f64) -> i32 {
    // SAFETY: raw pointer arithmetic mirrors C exactly; caller guarantees valid bounds
    unsafe {
        let b1 = *bin.add(0) * *d.add(0);
        let b2 = *bin.add(1) * *d.add(1);
        let A11 = *Ain.add(0) * *d.add(0) * *d.add(0);
        let A22 = *Ain.add(3) * *d.add(1) * *d.add(1);
        let A12 = *Ain.add(1) * *d.add(0) * *d.add(1);

        let mut la: f64 = 0.0;
        let mut v1: f64 = 0.0;
        let mut v2: f64 = 0.0;

        for _iter in 0..20 {
            let det = (A11 + la) * (A22 + la) - A12 * A12;
            if det < 1e-10 {
                *res.add(0) = 0.0;
                *res.add(1) = 0.0;
                return 0;
            }
            let detinv = 1.0 / det;
            let P11 = (A22 + la) * detinv;
            let P22 = (A11 + la) * detinv;
            let P12 = -A12 * detinv;
            v1 = -P11 * b1 - P12 * b2;
            v2 = -P12 * b1 - P22 * b2;
            let val = v1 * v1 + v2 * v2 - r * r;
            if val < 1e-10 {
                break;
            }
            let deriv = -2.0 * (P11 * v1 * v1 + 2.0 * P12 * v1 * v2 + P22 * v2 * v2);
            let delta = -val / deriv;
            if delta < 1e-10 {
                break;
            }
            la += delta;
        }

        *res.add(0) = v1 * *d.add(0);
        *res.add(1) = v2 * *d.add(1);
        (la != 0.0) as i32
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
    // SAFETY: raw pointer arithmetic mirrors C exactly; caller guarantees valid bounds
    unsafe {
        let b1 = *bin.add(0) * *d.add(0);
        let b2 = *bin.add(1) * *d.add(1);
        let b3 = *bin.add(2) * *d.add(2);
        let A11 = *Ain.add(0) * *d.add(0) * *d.add(0);
        let A22 = *Ain.add(4) * *d.add(1) * *d.add(1);
        let A33 = *Ain.add(8) * *d.add(2) * *d.add(2);
        let A12 = *Ain.add(1) * *d.add(0) * *d.add(1);
        let A13 = *Ain.add(2) * *d.add(0) * *d.add(2);
        let A23 = *Ain.add(5) * *d.add(1) * *d.add(2);

        let mut la: f64 = 0.0;
        let mut v1: f64 = 0.0;
        let mut v2: f64 = 0.0;
        let mut v3: f64 = 0.0;

        for _iter in 0..20 {
            // unscaled P
            let P11 = (A22 + la) * (A33 + la) - A23 * A23;
            let P22 = (A11 + la) * (A33 + la) - A13 * A13;
            let P33 = (A11 + la) * (A22 + la) - A12 * A12;
            let P12 = A13 * A23 - A12 * (A33 + la);
            let P13 = A12 * A23 - A13 * (A22 + la);
            let P23 = A12 * A13 - A23 * (A11 + la);

            // det(A+la)
            let det = (A11 + la) * P11 + A12 * P12 + A13 * P13;

            if det < 1e-10 {
                *res.add(0) = 0.0;
                *res.add(1) = 0.0;
                *res.add(2) = 0.0;
                return 0;
            }

            let detinv = 1.0 / det;

            // final P
            let P11 = P11 * detinv;
            let P22 = P22 * detinv;
            let P33 = P33 * detinv;
            let P12 = P12 * detinv;
            let P13 = P13 * detinv;
            let P23 = P23 * detinv;

            // v = -P*b
            v1 = -P11 * b1 - P12 * b2 - P13 * b3;
            v2 = -P12 * b1 - P22 * b2 - P23 * b3;
            v3 = -P13 * b1 - P23 * b2 - P33 * b3;

            // val = v'*v - r*r
            let val = v1 * v1 + v2 * v2 + v3 * v3 - r * r;

            if val < 1e-10 {
                break;
            }

            // deriv = -2*v'*P*v
            let deriv = -2.0 * (P11 * v1 * v1 + P22 * v2 * v2 + P33 * v3 * v3)
                - 4.0 * (P12 * v1 * v2 + P13 * v1 * v3 + P23 * v2 * v3);

            let delta = -val / deriv;
            if delta < 1e-10 {
                break;
            }

            la += delta;
        }

        *res.add(0) = v1 * *d.add(0);
        *res.add(1) = v2 * *d.add(1);
        *res.add(2) = v3 * *d.add(2);
        (la != 0.0) as i32
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
    // SAFETY: raw pointer arithmetic mirrors C exactly; caller guarantees valid bounds
    unsafe {
        let mut A: [f64; 25] = [0.0; 25];
        let mut Ala: [f64; 25] = [0.0; 25];
        let mut b: [f64; 5] = [0.0; 5];
        let mut tmp: [f64; 5] = [0.0; 5];

        if n > 5 {
            crate::engine::engine_util_errmem::mju_error(
                b"n is only supported up to 5\0".as_ptr() as *const i8);
        }

        for i in 0..n {
            b[i as usize] = *bin.add(i as usize) * *d.add(i as usize);
            for j in 0..n {
                A[(j + i * n) as usize] =
                    *Ain.add((j + i * n) as usize) * *d.add(i as usize) * *d.add(j as usize);
            }
        }

        let mut la: f64 = 0.0;

        for _iter in 0..20 {
            crate::engine::engine_util_blas::mju_copy(
                Ala.as_mut_ptr(),
                A.as_ptr(),
                n * n,
            );
            for i in 0..n {
                Ala[(i * (n + 1)) as usize] += la;
            }
            if crate::engine::engine_util_solve::mju_chol_factor(Ala.as_mut_ptr(), n, 1e-10) < n {
                crate::engine::engine_util_blas::mju_zero(res, n);
                return 0;
            }
            crate::engine::engine_util_solve::mju_chol_solve(
                res,
                Ala.as_ptr(),
                b.as_ptr(),
                n,
            );
            crate::engine::engine_util_blas::mju_scl(res, res as *const f64, -1.0, n);

            let val = crate::engine::engine_util_blas::mju_dot(res as *const f64, res as *const f64, n) - r * r;
            if val < 1e-10 {
                break;
            }

            crate::engine::engine_util_solve::mju_chol_solve(
                tmp.as_mut_ptr(),
                Ala.as_ptr(),
                res as *const f64,
                n,
            );
            let deriv = -2.0 * crate::engine::engine_util_blas::mju_dot(res as *const f64, tmp.as_ptr(), n);
            let delta = -val / deriv;
            if delta < 1e-10 {
                break;
            }
            la += delta;
        }

        for i in 0..n {
            *res.add(i as usize) = *res.add(i as usize) * *d.add(i as usize);
        }
        (la != 0.0) as i32
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
    const MJMINVAL: f64 = 1e-15;

    // algorithm options
    let maxiter: i32 = 100;
    let mingrad: f64 = 1E-16;
    let backtrack: f64 = 0.5;
    let minstep: f64 = 1E-22;
    let armijo: f64 = 0.1;

    // local enum encoding solver status
    const BOXQP_NOT_SPD: i32 = -1;
    const BOXQP_NO_DESCENT: i32 = 0;
    const BOXQP_MAX_ITER: i32 = 1;
    const BOXQP_MAX_LS_ITER: i32 = 2;
    const BOXQP_TOL_GRAD: i32 = 3;
    const BOXQP_UNBOUNDED: i32 = 4;
    const BOXQP_ALL_CLAMPED: i32 = 5;

    // SAFETY: caller guarantees res[n], R[n*(n+7)], H[n*n], g[n] valid; index, lower, upper may be null
    unsafe {
        let mut status: i32 = BOXQP_NO_DESCENT;
        let mut factorize: i32 = 1;
        let mut nfree: i32 = n;
        let mut nfactor: i32 = 0;
        let mut sdotg: f64;
        let mut improvement: f64 = 0.0;
        let mut value: f64 = 0.0;
        let mut norm2: f64 = 0.0;

        // basic checks
        if n <= 0 {
            crate::engine::engine_util_errmem::mju_error(
                b"problem size n must be positive\0".as_ptr() as *const i8,
            );
        }
        if !upper.is_null() && !lower.is_null() {
            let mut i: i32 = 0;
            while i < n {
                if *lower.add(i as usize) >= *upper.add(i as usize) {
                    crate::engine::engine_util_errmem::mju_error(
                        b"upper bounds must be stricly larger than lower bounds\0".as_ptr() as *const i8,
                    );
                }
                i += 1;
            }
        }

        // local scratch vectors, allocate in R
        let scratch: *mut f64 = R.add((n * n) as usize);
        let grad: *mut f64 = scratch.add(0);
        let search: *mut f64 = scratch.add(n as usize);
        let candidate: *mut f64 = scratch.add((2 * n) as usize);
        let temp: *mut f64 = scratch.add((3 * n) as usize);
        let clamped: *mut i32 = scratch.add((4 * n) as usize) as *mut i32;
        let oldclamped: *mut i32 = scratch.add((5 * n) as usize) as *mut i32;

        // if index vector not given, use scratch space
        let index_ptr: *mut i32 = if index.is_null() {
            scratch.add((6 * n) as usize) as *mut i32
        } else {
            index
        };

        // no bounds: return Newton point
        if lower.is_null() && upper.is_null() {
            // try to factorize
            crate::engine::engine_util_blas::mju_copy(R, H, n * n);
            let rank: i32 = crate::engine::engine_util_solve::mju_chol_factor(R, n, MJMINVAL);
            if rank == n {
                crate::engine::engine_util_solve::mju_chol_solve(res, R as *const f64, g, n);
                crate::engine::engine_util_blas::mju_scl(res, res as *const f64, -1.0, n);
                nfactor = 1;
                status = BOXQP_UNBOUNDED;
            } else {
                status = BOXQP_NOT_SPD;
            }

            // full index set (no clamping)
            let mut i: i32 = 0;
            while i < n {
                *index_ptr.add(i as usize) = i;
                i += 1;
            }
        }
        // have bounds: clamp res
        else {
            let mut i: i32 = 0;
            while i < n {
                if !lower.is_null() {
                    *res.add(i as usize) = crate::engine::engine_util_misc::mju_max(*res.add(i as usize), *lower.add(i as usize));
                }
                if !upper.is_null() {
                    *res.add(i as usize) = crate::engine::engine_util_misc::mju_min(*res.add(i as usize), *upper.add(i as usize));
                }
                i += 1;
            }
        }

        // ------ main loop
        let mut iter: i32 = 0;
        let mut oldvalue: f64;
        while iter < maxiter {
            if status != BOXQP_NO_DESCENT {
                break;
            }

            // compute objective: value = 0.5*res'*H*res + res'*g
            value = 0.5 * mul_vec_mat_vec_sym(res as *const f64, H, n) + crate::engine::engine_util_blas::mju_dot(res as *const f64, g, n);

            // save last value
            oldvalue = value;

            // compute gradient
            mul_sym_vec(grad, H, res as *const f64, n);
            crate::engine::engine_util_blas::mju_add_to(grad, g, n);

            // find clamped dimensions
            {
                let mut i: i32 = 0;
                while i < n {
                    *clamped.add(i as usize) =
                        ((!lower.is_null() && *res.add(i as usize) == *lower.add(i as usize) && *grad.add(i as usize) > 0.0) ||
                         (!upper.is_null() && *res.add(i as usize) == *upper.add(i as usize) && *grad.add(i as usize) < 0.0)) as i32;
                    i += 1;
                }
            }

            // build index of free dimensions, count them
            nfree = 0;
            {
                let mut i: i32 = 0;
                while i < n {
                    if *clamped.add(i as usize) == 0 {
                        *index_ptr.add(nfree as usize) = i;
                        nfree += 1;
                    }
                    i += 1;
                }
            }

            // all dimensions are clamped: minimum found
            if nfree == 0 {
                status = BOXQP_ALL_CLAMPED;
                break;
            }

            // re-factorize if clamped dimensions have changed
            if iter != 0 {
                factorize = 0;
                let mut i: i32 = 0;
                while i < n {
                    if *clamped.add(i as usize) != *oldclamped.add(i as usize) {
                        factorize = 1;
                        break;
                    }
                    i += 1;
                }
            }

            // save last clamped
            {
                let mut i: i32 = 0;
                while i < n {
                    *oldclamped.add(i as usize) = *clamped.add(i as usize);
                    i += 1;
                }
            }

            // get search direction: search = g + H_all,clamped * res_clamped
            {
                let mut i: i32 = 0;
                while i < n {
                    *temp.add(i as usize) = if *clamped.add(i as usize) != 0 { *res.add(i as usize) } else { 0.0 };
                    i += 1;
                }
            }
            mul_sym_vec(search, H, temp as *const f64, n);
            crate::engine::engine_util_blas::mju_add_to(search, g, n);

            // search = compress_free(search)
            {
                let mut i: i32 = 0;
                while i < nfree {
                    *search.add(i as usize) = *search.add(*index_ptr.add(i as usize) as usize);
                    i += 1;
                }
            }

            // R = compress_free(H)
            if factorize != 0 {
                let mut i: i32 = 0;
                while i < nfree {
                    let mut j: i32 = 0;
                    while j < i + 1 {
                        *R.add((i * nfree + j) as usize) = *H.add((*index_ptr.add(i as usize) * n + *index_ptr.add(j as usize)) as usize);
                        j += 1;
                    }
                    i += 1;
                }
            }

            // re-factorize and increment counter, if required
            let rank: i32 = if factorize != 0 { crate::engine::engine_util_solve::mju_chol_factor(R, nfree, MJMINVAL) } else { nfree };
            nfactor += factorize;

            // abort if factorization failed
            if rank != nfree {
                status = BOXQP_NOT_SPD;
                break;
            }

            // temp = H_free,free \ search_free
            crate::engine::engine_util_solve::mju_chol_solve(temp, R as *const f64, search as *const f64, nfree);

            // search_free = expand_free(-temp) - x_free
            crate::engine::engine_util_blas::mju_zero(search, n);
            {
                let mut i: i32 = 0;
                while i < nfree {
                    *search.add(*index_ptr.add(i as usize) as usize) = -*temp.add(i as usize) - *res.add(*index_ptr.add(i as usize) as usize);
                    i += 1;
                }
            }

            // ------ check gradient

            // squared norm of free gradient
            norm2 = 0.0;
            {
                let mut i: i32 = 0;
                while i < nfree {
                    let grad_i: f64 = *grad.add(*index_ptr.add(i as usize) as usize);
                    norm2 += grad_i * grad_i;
                    i += 1;
                }
            }

            // small gradient: minimum found
            if norm2 < mingrad {
                status = if nfree == n { BOXQP_UNBOUNDED } else { BOXQP_TOL_GRAD };
                break;
            }

            // sanity check: make sure we have a descent direction
            sdotg = crate::engine::engine_util_blas::mju_dot(search as *const f64, grad as *const f64, n);
            if sdotg >= 0.0 {
                break; // SHOULD NOT OCCUR
            }

            // ------ projected Armijo line search
            let mut step: f64 = 1.0;
            loop {
                // candidate = clamp(x + step*search)
                crate::engine::engine_util_blas::mju_scl(candidate, search as *const f64, step, n);
                crate::engine::engine_util_blas::mju_add_to(candidate, res as *const f64, n);
                {
                    let mut i: i32 = 0;
                    while i < n {
                        if !lower.is_null() && *candidate.add(i as usize) < *lower.add(i as usize) {
                            *candidate.add(i as usize) = *lower.add(i as usize);
                        } else if !upper.is_null() && *candidate.add(i as usize) > *upper.add(i as usize) {
                            *candidate.add(i as usize) = *upper.add(i as usize);
                        }
                        i += 1;
                    }
                }

                // new objective value
                value = 0.5 * mul_vec_mat_vec_sym(candidate as *const f64, H, n) + crate::engine::engine_util_blas::mju_dot(candidate as *const f64, g, n);

                // increment and break if step is too small
                step = step * backtrack;
                if step < minstep {
                    status = BOXQP_MAX_LS_ITER;
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
            status = BOXQP_MAX_ITER;
        }

        // return nf or -1 if failure
        if status == BOXQP_NO_DESCENT || status == BOXQP_NOT_SPD { -1 } else { nfree }
    }
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
    // SAFETY: raw pointer writes mirror C mju_boxQPmalloc exactly; caller owns all out-pointers
    unsafe {
        let n_usize = n as usize;

        // required arrays
        *res = crate::engine::engine_util_errmem::mju_malloc(std::mem::size_of::<f64>() * n_usize) as *mut f64;
        *R = crate::engine::engine_util_errmem::mju_malloc(std::mem::size_of::<f64>() * n_usize * (n_usize + 7)) as *mut f64;
        *H = crate::engine::engine_util_errmem::mju_malloc(std::mem::size_of::<f64>() * n_usize * n_usize) as *mut f64;
        *g = crate::engine::engine_util_errmem::mju_malloc(std::mem::size_of::<f64>() * n_usize) as *mut f64;

        // optional arrays
        if !lower.is_null() {
            *lower = crate::engine::engine_util_errmem::mju_malloc(std::mem::size_of::<f64>() * n_usize) as *mut f64;
        }
        if !upper.is_null() {
            *upper = crate::engine::engine_util_errmem::mju_malloc(std::mem::size_of::<f64>() * n_usize) as *mut f64;
        }
        if !index.is_null() {
            *index = crate::engine::engine_util_errmem::mju_malloc(std::mem::size_of::<i32>() * n_usize) as *mut i32;
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
    const MJMINVAL: f64 = 1e-15;

    // local enum encoding solver status
    const BOXQP_NOT_SPD: i32 = -1;
    const BOXQP_NO_DESCENT: i32 = 0;
    const BOXQP_MAX_ITER: i32 = 1;
    const BOXQP_MAX_LS_ITER: i32 = 2;
    const BOXQP_TOL_GRAD: i32 = 3;
    const BOXQP_UNBOUNDED: i32 = 4;
    const BOXQP_ALL_CLAMPED: i32 = 5;

    // SAFETY: caller guarantees res[n], R[n*(n+7)], H[n*n], g[n] valid; index, lower, upper may be null
    unsafe {
        let mut status: i32 = BOXQP_NO_DESCENT;
        let mut factorize: i32 = 1;
        let mut nfree: i32 = n;
        let mut nfactor: i32 = 0;
        let mut sdotg: f64;
        let mut improvement: f64 = 0.0;
        let mut value: f64 = 0.0;
        let mut norm2: f64 = 0.0;

        // basic checks
        if n <= 0 {
            extern "C" { fn mju_error(msg: *const i8, ...); }
            mju_error(b"problem size n must be positive\0".as_ptr() as *const i8);
        }
        if !upper.is_null() && !lower.is_null() {
            let mut i: i32 = 0;
            while i < n {
                if *lower.add(i as usize) >= *upper.add(i as usize) {
                    extern "C" { fn mju_error(msg: *const i8, ...); }
                    mju_error(b"upper bounds must be stricly larger than lower bounds\0".as_ptr() as *const i8);
                }
                i += 1;
            }
        }

        // local scratch vectors, allocate in R
        let scratch: *mut f64 = R.add((n * n) as usize);
        let grad: *mut f64 = scratch.add(0);
        let search: *mut f64 = scratch.add(n as usize);
        let candidate: *mut f64 = scratch.add((2 * n) as usize);
        let temp: *mut f64 = scratch.add((3 * n) as usize);
        let clamped: *mut i32 = scratch.add((4 * n) as usize) as *mut i32;
        let oldclamped: *mut i32 = scratch.add((5 * n) as usize) as *mut i32;

        // if index vector not given, use scratch space
        let index_ptr: *mut i32 = if index.is_null() {
            scratch.add((6 * n) as usize) as *mut i32
        } else {
            index
        };

        // no bounds: return Newton point
        if lower.is_null() && upper.is_null() {
            // try to factorize
            crate::engine::engine_util_blas::mju_copy(R, H, n * n);
            let rank: i32 = mju_chol_factor(R, n, MJMINVAL);
            if rank == n {
                mju_chol_solve(res, R as *const f64, g, n);
                crate::engine::engine_util_blas::mju_scl(res, res as *const f64, -1.0, n);
                nfactor = 1;
                status = BOXQP_UNBOUNDED;
            } else {
                status = BOXQP_NOT_SPD;
            }

            // full index set (no clamping)
            let mut i: i32 = 0;
            while i < n {
                *index_ptr.add(i as usize) = i;
                i += 1;
            }
        }
        // have bounds: clamp res
        else {
            let mut i: i32 = 0;
            while i < n {
                if !lower.is_null() {
                    *res.add(i as usize) = crate::engine::engine_util_misc::mju_max(
                        *res.add(i as usize), *lower.add(i as usize));
                }
                if !upper.is_null() {
                    *res.add(i as usize) = crate::engine::engine_util_misc::mju_min(
                        *res.add(i as usize), *upper.add(i as usize));
                }
                i += 1;
            }
        }

        // ------ main loop
        let mut iter: i32 = 0;
        let mut logptr: i32 = 0;
        let mut oldvalue: f64 = 0.0;
        while iter < maxiter {
            if status != BOXQP_NO_DESCENT {
                break;
            }

            // compute objective: value = 0.5*res'*H*res + res'*g
            value = 0.5 * mul_vec_mat_vec_sym(res as *const f64, H, n)
                + crate::engine::engine_util_blas::mju_dot(res as *const f64, g, n);

            // save last value
            oldvalue = value;

            // compute gradient
            mul_sym_vec(grad, H, res as *const f64, n);
            crate::engine::engine_util_blas::mju_add_to(grad, g, n);

            // find clamped dimensions
            {
                let mut i: i32 = 0;
                while i < n {
                    *clamped.add(i as usize) =
                        ((!lower.is_null() && *res.add(i as usize) == *lower.add(i as usize)
                            && *grad.add(i as usize) > 0.0)
                        || (!upper.is_null() && *res.add(i as usize) == *upper.add(i as usize)
                            && *grad.add(i as usize) < 0.0)) as i32;
                    i += 1;
                }
            }

            // build index of free dimensions, count them
            nfree = 0;
            {
                let mut i: i32 = 0;
                while i < n {
                    if *clamped.add(i as usize) == 0 {
                        *index_ptr.add(nfree as usize) = i;
                        nfree += 1;
                    }
                    i += 1;
                }
            }

            // all dimensions are clamped: minimum found
            if nfree == 0 {
                status = BOXQP_ALL_CLAMPED;
                break;
            }

            // re-factorize if clamped dimensions have changed
            if iter != 0 {
                factorize = 0;
                let mut i: i32 = 0;
                while i < n {
                    if *clamped.add(i as usize) != *oldclamped.add(i as usize) {
                        factorize = 1;
                        break;
                    }
                    i += 1;
                }
            }

            // save last clamped
            {
                let mut i: i32 = 0;
                while i < n {
                    *oldclamped.add(i as usize) = *clamped.add(i as usize);
                    i += 1;
                }
            }

            // get search direction: search = g + H_all,clamped * res_clamped
            {
                let mut i: i32 = 0;
                while i < n {
                    *temp.add(i as usize) = if *clamped.add(i as usize) != 0 {
                        *res.add(i as usize)
                    } else {
                        0.0
                    };
                    i += 1;
                }
            }
            mul_sym_vec(search, H, temp as *const f64, n);
            crate::engine::engine_util_blas::mju_add_to(search, g, n);

            // search = compress_free(search)
            {
                let mut i: i32 = 0;
                while i < nfree {
                    *search.add(i as usize) = *search.add(*index_ptr.add(i as usize) as usize);
                    i += 1;
                }
            }

            // R = compress_free(H)
            if factorize != 0 {
                let mut i: i32 = 0;
                while i < nfree {
                    let mut j: i32 = 0;
                    while j < i + 1 {
                        *R.add((i * nfree + j) as usize) = *H.add(
                            (*index_ptr.add(i as usize) * n + *index_ptr.add(j as usize)) as usize,
                        );
                        j += 1;
                    }
                    i += 1;
                }
            }

            // re-factorize and increment counter, if required
            let rank: i32 = if factorize != 0 {
                mju_chol_factor(R, nfree, MJMINVAL)
            } else {
                nfree
            };
            nfactor += factorize;

            // abort if factorization failed
            if rank != nfree {
                status = BOXQP_NOT_SPD;
                break;
            }

            // temp = H_free,free \ search_free
            mju_chol_solve(temp, R as *const f64, search as *const f64, nfree);

            // search_free = expand_free(-temp) - x_free
            crate::engine::engine_util_blas::mju_zero(search, n);
            {
                let mut i: i32 = 0;
                while i < nfree {
                    *search.add(*index_ptr.add(i as usize) as usize) =
                        -*temp.add(i as usize) - *res.add(*index_ptr.add(i as usize) as usize);
                    i += 1;
                }
            }

            // ------ check gradient

            // squared norm of free gradient
            norm2 = 0.0;
            {
                let mut i: i32 = 0;
                while i < nfree {
                    let grad_i: f64 = *grad.add(*index_ptr.add(i as usize) as usize);
                    norm2 += grad_i * grad_i;
                    i += 1;
                }
            }

            // small gradient: minimum found
            if norm2 < mingrad {
                status = if nfree == n { BOXQP_UNBOUNDED } else { BOXQP_TOL_GRAD };
                break;
            }

            // sanity check: make sure we have a descent direction
            sdotg = crate::engine::engine_util_blas::mju_dot(
                search as *const f64, grad as *const f64, n);
            if sdotg >= 0.0 {
                break; // SHOULD NOT OCCUR
            }

            // ------ projected Armijo line search
            let mut step: f64 = 1.0;
            let mut nstep: i32 = 0;
            loop {
                // candidate = clamp(x + step*search)
                crate::engine::engine_util_blas::mju_scl(
                    candidate, search as *const f64, step, n);
                crate::engine::engine_util_blas::mju_add_to(candidate, res as *const f64, n);
                {
                    let mut i: i32 = 0;
                    while i < n {
                        if !lower.is_null() && *candidate.add(i as usize) < *lower.add(i as usize) {
                            *candidate.add(i as usize) = *lower.add(i as usize);
                        } else if !upper.is_null() && *candidate.add(i as usize) > *upper.add(i as usize) {
                            *candidate.add(i as usize) = *upper.add(i as usize);
                        }
                        i += 1;
                    }
                }

                // new objective value
                value = 0.5 * mul_vec_mat_vec_sym(candidate as *const f64, H, n)
                    + crate::engine::engine_util_blas::mju_dot(candidate as *const f64, g, n);

                // increment and break if step is too small
                nstep += 1;
                step = step * backtrack;
                if step < minstep {
                    status = BOXQP_MAX_LS_ITER;
                    break;
                }

                // repeat until relative improvement >= Armijo
                improvement = (value - oldvalue) / (step * sdotg);
                if improvement >= armijo {
                    break;
                }
            }

            // print iteration info
            if !log.is_null() && logptr < logsz {
                extern "C" {
                    fn snprintf(s: *mut i8, n: usize, fmt: *const i8, ...) -> i32;
                }
                let written: i32 = snprintf(
                    log.add(logptr as usize),
                    (logsz - logptr) as usize,
                    b"iter %-3d:  |grad|: %-8.2g  reduction: %-8.2g  improvement: %-8.4g  linesearch: %g^%-2d  factorized: %d  nfree: %d\n\0".as_ptr() as *const i8,
                    iter + 1,
                    norm2.sqrt(),
                    oldvalue - value,
                    improvement,
                    backtrack,
                    nstep - 1,
                    factorize,
                    nfree,
                );
                if written > 0 {
                    logptr = if logptr + written < logsz { logptr + written } else { logsz };
                }
            }

            // accept candidate
            crate::engine::engine_util_blas::mju_copy(res, candidate as *const f64, n);

            iter += 1;
        }

        // max iterations exceeded
        if iter == maxiter {
            status = BOXQP_MAX_ITER;
        }

        // print final info
        if !log.is_null() && logptr < logsz {
            extern "C" {
                fn snprintf(s: *mut i8, n: usize, fmt: *const i8, ...) -> i32;
            }
            // status_string lookup
            let status_str: *const i8 = match status {
                BOXQP_NOT_SPD => b"Hessian is not positive definite\0".as_ptr() as *const i8,
                BOXQP_NO_DESCENT => b"No descent direction found\0".as_ptr() as *const i8,
                BOXQP_MAX_ITER => b"Maximum main iterations exceeded\0".as_ptr() as *const i8,
                BOXQP_MAX_LS_ITER => b"Maximum line-search iterations exceeded\0".as_ptr() as *const i8,
                BOXQP_TOL_GRAD => b"Gradient norm smaller than tolerance\0".as_ptr() as *const i8,
                BOXQP_UNBOUNDED => b"No dimensions clamped, returning Newton point\0".as_ptr() as *const i8,
                BOXQP_ALL_CLAMPED => b"All dimensions clamped\0".as_ptr() as *const i8,
                _ => b"Unknown status\0".as_ptr() as *const i8,
            };
            snprintf(
                log.add(logptr as usize),
                (logsz - logptr) as usize,
                b"BOXQP: %s.\niterations= %d,  factorizations= %d,  |grad|= %-12.6g, final value= %-12.6g\n\0".as_ptr() as *const i8,
                status_str,
                iter,
                nfactor,
                norm2.sqrt(),
                value,
            );
        }

        // return nf or -1 if failure
        if status == BOXQP_NO_DESCENT || status == BOXQP_NOT_SPD { -1 } else { nfree }
    }
}

