//! Port of: engine/engine_util_solve.c
//! IR hash: 1b139f44af8230f9
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
    todo!() // mulVecMatVecSym
}

/// C: mulSymVec (engine/engine_util_solve.c:1412)
/// Calls: mju_addToScl, mju_dot
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mul_sym_vec(res: mjtNum__restrict, mat: *const f64, vec: *const f64, n: i32) {
    todo!() // mulSymVec
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
        let mut rank: i32 = n; // C: int rank = n;
        let mut tmp: f64; // C: mjtNum tmp;
        let mut j: i32 = 0; // C: int j=0;
        while j < n { // C: for (int j=0; j < n; j++)
            tmp = *mat.add((j * (n + 1)) as usize); // C: tmp = mat[j*(n+1)];
            if j != 0 { // C: if (j)
                tmp -= crate::engine::engine_util_blas::mju_dot(mat.add((j * n) as usize), mat.add((j * n) as usize), j); // C: tmp -= mju_dot(mat+j*n, mat+j*n, j);
            }
            if tmp < mindiag { // C: if (tmp < mindiag)
                tmp = mindiag; // C: tmp = mindiag;
                rank -= 1; // C: rank--;
            }
            *mat.add((j * (n + 1)) as usize) = tmp.sqrt(); // C: mat[j*(n+1)] = mju_sqrt(tmp);
            tmp = 1.0 / *mat.add((j * (n + 1)) as usize); // C: tmp = 1/mat[j*(n+1)];
            let mut i: i32 = j + 1; // C: int i=j+1;
            while i < n { // C: for (int i=j+1; i < n; i++)
                *mat.add((i * n + j) as usize) = (*mat.add((i * n + j) as usize) - crate::engine::engine_util_blas::mju_dot(mat.add((i * n) as usize), mat.add((j * n) as usize), j)) * tmp; // C: mat[i*n+j] = (mat[i*n+j] - mju_dot(mat+i*n, mat+j*n, j)) * tmp;
                i += 1;
            }
            j += 1;
        }
        rank // C: return rank;
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
    unsafe {
        if res != vec as *mut f64 { // C: if (res != vec)
            crate::engine::engine_util_blas::mju_copy(res, vec, n); // C: mju_copy(res, vec, n);
        }
        let mut i: i32 = 0; // C: int i=0;
        while i < n { // C: for (int i=0; i < n; i++)
            if i != 0 { // C: if (i)
                *res.add(i as usize) -= crate::engine::engine_util_blas::mju_dot(mat.add((i * n) as usize) as *const f64, res as *const f64, i); // C: res[i] -= mju_dot(mat+i*n, res, i);
            }
            *res.add(i as usize) /= *mat.add((i * (n + 1)) as usize); // C: res[i] /= mat[i*(n+1)];
            i += 1;
        }
        let mut i: i32 = n - 1; // C: int i=n-1;
        while i >= 0 { // C: for (int i=n-1; i >= 0; i--)
            if i < n - 1 { // C: if (i < n-1)
                let mut j: i32 = i + 1; // C: int j=i+1;
                while j < n { // C: for (int j=i+1; j < n; j++)
                    *res.add(i as usize) -= *mat.add((j * n + i) as usize) * *res.add(j as usize); // C: res[i] -= mat[j*n+i] * res[j];
                    j += 1;
                }
            }
            *res.add(i as usize) /= *mat.add((i * (n + 1)) as usize); // C: res[i] /= mat[i*(n+1)];
            i -= 1;
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
    unsafe {
        let mut rank: i32 = n; // C: int rank = n;
        let mjMINVAL: f64 = 1e-15_f64; // C: mjMINVAL
        let mut k: i32 = 0; // C: int k=0;
        while k < n { // C: for (int k=0; k < n; k++)
            if *x.add(k as usize) != 0.0 { // C: if (x[k] != 0.0)
                let Lkk: f64 = *mat.add((k * (n + 1)) as usize); // C: Lkk = mat[k*(n+1)];
                let mut tmp: f64 = Lkk * Lkk + (if flg_plus != 0 { *x.add(k as usize) * *x.add(k as usize) } else { -(*x.add(k as usize) * *x.add(k as usize)) }); // C: tmp = Lkk*Lkk + (flg_plus != 0 ? x[k]*x[k] : -x[k]*x[k]);
                if tmp < mjMINVAL { // C: if (tmp < mjMINVAL)
                    tmp = mjMINVAL; // C: tmp = mjMINVAL;
                    rank -= 1; // C: rank--;
                }
                let r: f64 = tmp.sqrt(); // C: r = mju_sqrt(tmp);
                let c: f64 = r / Lkk; // C: c = r / Lkk;
                let cinv: f64 = 1.0 / c; // C: cinv = 1.0 / c;
                let s: f64 = *x.add(k as usize) / Lkk; // C: s = x[k] / Lkk;
                *mat.add((k * (n + 1)) as usize) = r; // C: mat[k*(n+1)] = r;
                if flg_plus != 0 { // C: if (flg_plus != 0)
                    let mut i: i32 = k + 1; // C: int i=k+1;
                    while i < n { // C: for (int i=k+1; i < n; i++)
                        *mat.add((i * n + k) as usize) = (*mat.add((i * n + k) as usize) + s * *x.add(i as usize)) * cinv; // C: mat[i*n+k] = (mat[i*n+k] + s*x[i])*cinv;
                        i += 1;
                    }
                } else { // C: else
                    let mut i: i32 = k + 1; // C: int i=k+1;
                    while i < n { // C: for (int i=k+1; i < n; i++)
                        *mat.add((i * n + k) as usize) = (*mat.add((i * n + k) as usize) - s * *x.add(i as usize)) * cinv; // C: mat[i*n+k] = (mat[i*n+k] - s*x[i])*cinv;
                        i += 1;
                    }
                }
                let mut i: i32 = k + 1; // C: int i=k+1;
                while i < n { // C: for (int i=k+1; i < n; i++)
                    *x.add(i as usize) = c * *x.add(i as usize) - s * *mat.add((i * n + k) as usize); // C: x[i] = c*x[i] - s*mat[i*n+k];
                    i += 1;
                }
            }
            k += 1;
        }
        rank // C: return rank;
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
    todo!() // mju_cholFactorSparse
}

/// C: mju_cholFactorSymbolic (engine/engine_util_solve.h:45)
/// Calls: mj_freeStack, mj_markStack
#[allow(unused_variables, non_snake_case)]
pub fn mju_chol_factor_symbolic(L_colind: *mut i32, L_rownnz: *mut i32, L_rowadr: *mut i32, LT_colind: *mut i32, LT_rownnz: *mut i32, LT_rowadr: *mut i32, LT_map: *mut i32, rownnz: *const i32, rowadr: *const i32, colind: *const i32, n: i32, d: *mut mjData) -> i32 {
    todo!() // mju_cholFactorSymbolic
}

/// C: mju_cholFactorNumeric (engine/engine_util_solve.h:53)
/// Calls: mj_freeStack, mj_markStack, mju_scatter, mju_zero
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
    todo!() // mju_cholSolveSparse
}

/// C: mju_cholUpdateSparse (engine/engine_util_solve.h:66)
/// Calls: mj_freeStack, mj_markStack, mju_scatter, mju_zero
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
    unsafe {
        let mjMINVAL: f64 = 1e-15_f64; // C: mjMINVAL
        let nsparse: i32 = ntotal - ndense; // C: int nsparse = ntotal - ndense;
        let mut mindiag: f64 = -1.0; // C: mjtNum mindiag = -1.0;

        // sparse part
        let mut j: i32 = 0; // C: int j=0;
        while j < nsparse { // C: for (int j=0; j < nsparse; j++)
            let width_jj: i32 = if j < nband - 1 { j } else { nband - 1 }; // C: int width_jj = (j < nband-1) ? j : nband-1;
            let height: i32 = if nsparse - j - 1 < nband - 1 { nsparse - j - 1 } else { nband - 1 }; // C: int height = (nsparse-j-1 < nband-1) ? nsparse-j-1 : nband-1;
            let adr_jj: i32 = (j + 1) * nband - 1; // C: int adr_jj = (j+1)*nband-1;

            let mut left_ij: f64 = if width_jj > 0 { // C: mjtNum left_ij = width_jj > 0 ? mju_dot(mat+adr_jj-width_jj, mat+adr_jj-width_jj, width_jj) : 0.0;
                crate::engine::engine_util_blas::mju_dot(mat.add((adr_jj - width_jj) as usize), mat.add((adr_jj - width_jj) as usize), width_jj)
            } else {
                0.0
            };
            let mut Ljj: f64 = diagadd + diagmul * *mat.add(adr_jj as usize) + *mat.add(adr_jj as usize) - left_ij; // C: mjtNum Ljj = diagadd + diagmul*mat[adr_jj] + mat[adr_jj] - left_ij;

            if Ljj < mindiag || mindiag < 0.0 { // C: if (Ljj < mindiag || mindiag < 0.0)
                mindiag = Ljj; // C: mindiag = Ljj;
            }
            if Ljj < mjMINVAL { // C: if (Ljj < mjMINVAL)
                return 0.0; // C: return 0.0;
            }

            Ljj = Ljj.sqrt(); // C: Ljj = mju_sqrt(Ljj);
            let scale: f64 = 1.0 / Ljj; // C: mjtNum scale = 1.0/Ljj;

            let mut i: i32 = j + 1; // C: int i=j+1;
            while i <= j + height { // C: for (int i=j+1; i <= j+height; i++)
                let width_ij: i32 = if j < nband - 1 - i + j { j } else { nband - 1 - i + j }; // C: int width_ij = (j < nband-1-i+j) ? j : nband-1-i+j;
                let adr_ij: i32 = (i + 1) * nband - 1 - i + j; // C: int adr_ij = (i+1)*nband-1-i+j;
                left_ij = if width_ij > 0 { // C: left_ij = width_ij > 0 ? mju_dot(mat+adr_jj-width_ij, mat+adr_ij-width_ij, width_ij) : 0.0;
                    crate::engine::engine_util_blas::mju_dot(mat.add((adr_jj - width_ij) as usize), mat.add((adr_ij - width_ij) as usize), width_ij)
                } else {
                    0.0
                };
                *mat.add(adr_ij as usize) = scale * (*mat.add(adr_ij as usize) - left_ij); // C: mat[adr_ij] = scale * (mat[adr_ij] - left_ij);
                i += 1;
            }

            let mut i: i32 = nsparse; // C: int i=nsparse;
            while i < ntotal { // C: for (int i=nsparse; i < ntotal; i++)
                let adr_ij: i32 = nsparse * nband + (i - nsparse) * ntotal + j; // C: int adr_ij = nsparse*nband + (i-nsparse)*ntotal + j;
                left_ij = if width_jj > 0 { // C: left_ij = width_jj > 0 ? mju_dot(mat+adr_jj-width_jj, mat+adr_ij-width_jj, width_jj) : 0.0;
                    crate::engine::engine_util_blas::mju_dot(mat.add((adr_jj - width_jj) as usize), mat.add((adr_ij - width_jj) as usize), width_jj)
                } else {
                    0.0
                };
                *mat.add(adr_ij as usize) = scale * (*mat.add(adr_ij as usize) - left_ij); // C: mat[adr_ij] = scale * (mat[adr_ij] - left_ij);
                i += 1;
            }

            *mat.add(adr_jj as usize) = Ljj; // C: mat[adr_jj] = Ljj;
            j += 1;
        }

        // dense part
        let mut j: i32 = nsparse; // C: int j=nsparse;
        while j < ntotal { // C: for (int j=nsparse; j < ntotal; j++)
            let adr_jj: i32 = nsparse * nband + (j - nsparse) * ntotal + j; // C: int adr_jj = nsparse*nband + (j-nsparse)*ntotal + j;
            let mut Ljj: f64 = diagadd + diagmul * *mat.add(adr_jj as usize) + *mat.add(adr_jj as usize) - crate::engine::engine_util_blas::mju_dot(mat.add((adr_jj - j) as usize), mat.add((adr_jj - j) as usize), j); // C: mjtNum Ljj = diagadd + diagmul*mat[adr_jj] + mat[adr_jj] - mju_dot(mat+adr_jj-j, mat+adr_jj-j, j);

            if Ljj < mindiag || mindiag < 0.0 { // C: if (Ljj < mindiag || mindiag < 0.0)
                mindiag = Ljj; // C: mindiag = Ljj;
            }
            if Ljj < mjMINVAL { // C: if (Ljj < mjMINVAL)
                return 0.0; // C: return 0.0;
            }

            Ljj = Ljj.sqrt(); // C: Ljj = mju_sqrt(Ljj);
            let scale: f64 = 1.0 / Ljj; // C: mjtNum scale = 1.0/Ljj;

            let mut i: i32 = j + 1; // C: int i=j+1;
            while i < ntotal { // C: for (int i=j+1; i < ntotal; i++)
                let adr_ij: i32 = adr_jj + ntotal * (i - j); // C: int adr_ij = adr_jj + ntotal*(i-j);
                *mat.add(adr_ij as usize) = scale * (*mat.add(adr_ij as usize) - crate::engine::engine_util_blas::mju_dot(mat.add((adr_jj - j) as usize), mat.add((adr_ij - j) as usize), j)); // C: mat[adr_ij] = scale * (mat[adr_ij] - mju_dot(mat+adr_jj-j, mat+adr_ij-j, j));
                i += 1;
            }

            *mat.add(adr_jj as usize) = Ljj; // C: mat[adr_jj] = Ljj;
            j += 1;
        }

        mindiag // C: return mindiag;
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
    unsafe {
        let nsparse: i32 = ntotal - ndense; // C: int nsparse = ntotal - ndense;

        if res != vec as *mut f64 { // C: if (res != vec)
            crate::engine::engine_util_blas::mju_copy(res, vec, ntotal); // C: mju_copy(res, vec, ntotal);
        }

        // forward substitution: sparse part
        let mut i: i32 = 0; // C: int i=0;
        while i < nsparse { // C: for (int i=0; i < nsparse; i++)
            let width: i32 = if i < nband - 1 { i } else { nband - 1 }; // C: width = mjMIN(i, nband-1);
            if width > 0 { // C: if (width > 0)
                *res.add(i as usize) -= crate::engine::engine_util_blas::mju_dot(mat.add(((i + 1) * nband - 1 - width) as usize), res.add((i - width) as usize) as *const f64, width); // C: res[i] -= mju_dot(mat+(i+1)*nband-1-width, res+i-width, width);
            }
            *res.add(i as usize) /= *mat.add(((i + 1) * nband - 1) as usize); // C: res[i] /= mat[(i+1)*nband-1];
            i += 1;
        }

        // forward substitution: dense part
        let mut i: i32 = nsparse; // C: int i=nsparse;
        while i < ntotal { // C: for (int i=nsparse; i < ntotal; i++)
            *res.add(i as usize) -= crate::engine::engine_util_blas::mju_dot(mat.add((nsparse * nband + (i - nsparse) * ntotal) as usize), res as *const f64, i); // C: res[i] -= mju_dot(mat+nsparse*nband+(i-nsparse)*ntotal, res, i);
            *res.add(i as usize) /= *mat.add((nsparse * nband + (i - nsparse) * ntotal + i) as usize); // C: res[i] /= mat[nsparse*nband+(i-nsparse)*ntotal+i];
            i += 1;
        }

        // backward substitution: dense part
        let mut i: i32 = ntotal - 1; // C: int i=ntotal-1;
        while i >= nsparse { // C: for (int i=ntotal-1; i >= nsparse; i--)
            let mut j: i32 = i + 1; // C: int j=i+1;
            while j < ntotal { // C: for (int j=i+1; j < ntotal; j++)
                *res.add(i as usize) -= *mat.add((nsparse * nband + (j - nsparse) * ntotal + i) as usize) * *res.add(j as usize); // C: res[i] -= mat[nsparse*nband+(j-nsparse)*ntotal+i] * res[j];
                j += 1;
            }
            *res.add(i as usize) /= *mat.add((nsparse * nband + (i - nsparse) * ntotal + i) as usize); // C: res[i] /= mat[nsparse*nband+(i-nsparse)*ntotal+i];
            i -= 1;
        }

        // backward substitution: sparse part
        let mut i: i32 = nsparse - 1; // C: int i=nsparse-1;
        while i >= 0 { // C: for (int i=nsparse-1; i >= 0; i--)
            let height: i32 = if nsparse - 1 - i < nband - 1 { nsparse - 1 - i } else { nband - 1 }; // C: height = mjMIN(nsparse-1-i, nband-1);
            let mut j: i32 = i + 1; // C: int j=i+1;
            while j <= i + height { // C: for (int j=i+1; j <= i+height; j++)
                *res.add(i as usize) -= *mat.add(((j + 1) * nband - 1 - (j - i)) as usize) * *res.add(j as usize); // C: res[i] -= mat[(j+1)*nband-1-(j-i)] * res[j];
                j += 1;
            }
            let mut j: i32 = nsparse; // C: int j=nsparse;
            while j < ntotal { // C: for (int j=nsparse; j < ntotal; j++)
                *res.add(i as usize) -= *mat.add((nsparse * nband + (j - nsparse) * ntotal + i) as usize) * *res.add(j as usize); // C: res[i] -= mat[nsparse*nband+(j-nsparse)*ntotal+i] * res[j];
                j += 1;
            }
            *res.add(i as usize) /= *mat.add(((i + 1) * nband - 1) as usize); // C: res[i] /= mat[(i+1)*nband-1];
            i -= 1;
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
    unsafe {
        let nblock: i32 = ntotal - ndense; // C: int nblock = ntotal-ndense;
        crate::engine::engine_util_blas::mju_zero(res, ntotal * ntotal); // C: mju_zero(res, ntotal*ntotal);
        let mut i: i32 = 0; // C: int i=0;
        while i < nblock { // C: for (int i=0; i < nblock; i++)
            let width: i32 = mju_band_diag(i, ntotal, nband, ndense); // C: int width = mju_bandDiag(i, ntotal, nband, ndense);
            let mut j: i32 = 0; // C: int j=0;
            while j < width { // C: for (int j=0; j < width; j++)
                *res.add(((i + j) * ntotal + i) as usize) = *mat.add((i * nband + j) as usize); // C: res[(i+j)*ntotal+i] = mat[i*nband+j];
                j += 1;
            }
            let mut j: i32 = 0; // C: int j=0;
            while j < ndense { // C: for (int j=0; j < ndense; j++)
                *res.add(((nblock + j) * ntotal + i) as usize) = *mat.add((nblock * nband + j * nblock + i) as usize); // C: res[(nblock+j)*ntotal+i] = mat[nblock*nband+j*nblock+i];
                j += 1;
            }
            i += 1;
        }
        let mut i: i32 = 0; // C: int i=0;
        while i < ndense { // C: for (int i=0; i < ndense; i++)
            let mut j: i32 = 0; // C: int j=0;
            while j <= i { // C: for (int j=0; j <= i; j++)
                *res.add(((nblock + i) * ntotal + nblock + j) as usize) = *mat.add((nblock * nband + ndense * nblock + i * ndense + j) as usize); // C: res[(nblock+i)*ntotal+nblock+j] = mat[nblock*nband+ndense*nblock+i*ndense+j];
                j += 1;
            }
            i += 1;
        }
        // C: if (flg_sym)
        // NOTE: mjtBool is ZST in codegen; reinterpret via pointer cast (C ABI passes int)
        let flg_sym_i32: i32 = *(&flg_sym as *const mjtBool as *const i32);
        if flg_sym_i32 != 0 { // C: if (flg_sym)
            let mut i: i32 = 0; // C: int i=0;
            while i < ntotal { // C: for (int i=0; i < ntotal; i++)
                let mut j: i32 = i + 1; // C: int j=i+1;
                while j < ntotal { // C: for (int j=i+1; j < ntotal; j++)
                    *res.add((i * ntotal + j) as usize) = *res.add((j * ntotal + i) as usize); // C: res[i*ntotal+j] = res[j*ntotal+i];
                    j += 1;
                }
                i += 1;
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
    unsafe {
        let nblock: i32 = ntotal - ndense; // C: int nblock = ntotal-ndense;
        crate::engine::engine_util_blas::mju_zero(res, nblock * nband + ndense * nblock + ndense * ndense); // C: mju_zero(res, nblock*nband+ndense*nblock+ndense*ndense);
        let mut i: i32 = 0; // C: int i=0;
        while i < nblock { // C: for (int i=0; i < nblock; i++)
            let width: i32 = mju_band_diag(i, ntotal, nband, ndense); // C: int width = mju_bandDiag(i, ntotal, nband, ndense);
            let mut j: i32 = 0; // C: int j=0;
            while j < width { // C: for (int j=0; j < width; j++)
                *res.add((i * nband + j) as usize) = *mat.add(((i + j) * ntotal + i) as usize); // C: res[i*nband+j] = mat[(i+j)*ntotal+i];
                j += 1;
            }
            let mut j: i32 = 0; // C: int j=0;
            while j < ndense { // C: for (int j=0; j < ndense; j++)
                *res.add((nblock * nband + j * nblock + i) as usize) = *mat.add(((nblock + j) * ntotal + i) as usize); // C: res[nblock*nband+j*nblock+i] = mat[(nblock+j)*ntotal+i];
                j += 1;
            }
            i += 1;
        }
        let mut i: i32 = 0; // C: int i=0;
        while i < ndense { // C: for (int i=0; i < ndense; i++)
            let mut j: i32 = 0; // C: int j=0;
            while j <= i { // C: for (int j=0; j <= i; j++)
                *res.add((nblock * nband + ndense * nblock + i * ndense + j) as usize) = *mat.add(((nblock + i) * ntotal + nblock + j) as usize); // C: res[nblock*nband+ndense*nblock+i*ndense+j] = mat[(nblock+i)*ntotal+nblock+j];
                j += 1;
            }
            i += 1;
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
    unsafe {
        let nblock: i32 = ntotal - ndense; // C: int nblock = ntotal-ndense;
        crate::engine::engine_util_blas::mju_zero(res, ntotal * nvec); // C: mju_zero(res, ntotal*nVec);
        // NOTE: mjtBool is ZST in codegen; reinterpret via pointer cast (C ABI passes int)
        let flg_sym_i32: i32 = *(&flg_sym as *const mjtBool as *const i32);
        let mut iv: i32 = 0; // C: int iv=0;
        while iv < nvec { // C: for (int iv=0; iv < nVec; iv++)
            let vec_iv: *const f64 = vec.add((iv * ntotal) as usize); // C: const mjtNum* vec_iv = vec + iv*ntotal;
            let res_iv: *mut f64 = res.add((iv * ntotal) as usize); // C: mjtNum* res_iv = res + iv*ntotal;
            let mut i: i32 = 0; // C: int i=0;
            while i < nblock { // C: for (int i=0; i < nblock; i++)
                let width: i32 = mju_band_diag(i, ntotal, nband, ndense); // C: int width = mju_bandDiag(i, ntotal, nband, ndense);
                let mut j: i32 = 0; // C: int j=0;
                while j < width { // C: for (int j=0; j < width; j++)
                    *res_iv.add(i as usize) += *mat.add((i * nband + j) as usize) * *vec_iv.add((i + j) as usize); // C: res_iv[i] += mat[i*nband+j] * vec_iv[i+j];
                    if j > 0 { // C: if (j > 0)
                        *res_iv.add((i + j) as usize) += *mat.add((i * nband + j) as usize) * *vec_iv.add(i as usize); // C: res_iv[i+j] += mat[i*nband+j] * vec_iv[i];
                    }
                    j += 1;
                }
                let mut j: i32 = 0; // C: int j=0;
                while j < ndense { // C: for (int j=0; j < ndense; j++)
                    let val: f64 = *mat.add((nblock * nband + j * nblock + i) as usize); // C: mjtNum val = mat[nblock*nband+j*nblock+i];
                    *res_iv.add(i as usize) += val * *vec_iv.add((nblock + j) as usize); // C: res_iv[i] += val * vec_iv[nblock+j];
                    *res_iv.add((nblock + j) as usize) += val * *vec_iv.add(i as usize); // C: res_iv[nblock+j] += val * vec_iv[i];
                    j += 1;
                }
                i += 1;
            }
            let mut i: i32 = 0; // C: int i=0;
            while i < ndense { // C: for (int i=0; i < ndense; i++)
                let mut j: i32 = 0; // C: int j=0;
                while j <= i { // C: for (int j=0; j <= i; j++)
                    let val: f64 = *mat.add((nblock * nband + ndense * nblock + i * ndense + j) as usize); // C: mjtNum val = mat[nblock*nband+ndense*nblock+i*ndense+j];
                    *res_iv.add((nblock + i) as usize) += val * *vec_iv.add((nblock + j) as usize); // C: res_iv[nblock+i] += val * vec_iv[nblock+j];
                    if j < i { // C: if (j < i)
                        *res_iv.add((nblock + j) as usize) += val * *vec_iv.add((nblock + i) as usize); // C: res_iv[nblock+j] += val * vec_iv[nblock+i];
                    }
                    j += 1;
                }
                i += 1;
            }
            if flg_sym_i32 == 0 { // C: if (!flg_sym)
                let mut i: i32 = 0; // C: int i=0;
                while i < nblock { // C: for (int i=0; i < nblock; i++)
                    let width: i32 = mju_band_diag(i, ntotal, nband, ndense); // C: int width = mju_bandDiag(i, ntotal, nband, ndense);
                    let mut j: i32 = 1; // C: int j=1;
                    while j < width { // C: for (int j=1; j < width; j++)
                        *res_iv.add(i as usize) += *mat.add(((i + j) * nband) as usize) * *vec_iv.add((i + j) as usize); // C: res_iv[i] += mat[(i+j)*nband] * vec_iv[i+j];
                        j += 1;
                    }
                    i += 1;
                }
            }
            iv += 1;
        }
    }
}

/// C: mju_bandDiag (engine/engine_util_solve.h:95)
#[allow(unused_variables, non_snake_case)]
pub fn mju_band_diag(i: i32, ntotal: i32, nband: i32, ndense: i32) -> i32 {
    if i >= ntotal - ndense { // C: if (i >= ntotal-ndense)
        return ntotal - i; // C: return ntotal-i;
    }
    let a = nband; // C: mju_min_int(nband, ntotal-ndense-i)
    let b = ntotal - ndense - i; // C: ntotal-ndense-i
    if a < b { a } else { b } // C: mju_min_int(nband, ntotal-ndense-i)
}

/// C: mju_factorLU (engine/engine_util_solve.h:102)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_factor_lu(A: *mut f64, n: i32, pivot: *mut i32) -> i32 {
    unsafe {
        let mjMINVAL: f64 = 1e-15_f64; // C: mjMINVAL
        let mut i: i32 = 0; // C: int i=0;
        while i < n { // C: for (int i=0; i < n; i++)
            *pivot.add(i as usize) = i; // C: pivot[i] = i;
            i += 1;
        }
        let mut k: i32 = 0; // C: int k=0;
        while k < n { // C: for (int k=0; k < n; k++)
            let mut maxval: f64 = 0.0; // C: mjtNum maxval = 0;
            let mut maxind: i32 = k; // C: int maxind = k;
            let mut i: i32 = k; // C: int i=k;
            while i < n { // C: for (int i=k; i < n; i++)
                let val: f64 = (*A.add((i * n + k) as usize)).abs(); // C: mjtNum val = mju_abs(A[i*n+k]);
                if val > maxval { // C: if (val > maxval)
                    maxval = val; // C: maxval = val;
                    maxind = i; // C: maxind = i;
                }
                i += 1;
            }
            if maxval < mjMINVAL { // C: if (maxval < mjMINVAL)
                return 0; // C: return 0;
            }
            if maxind != k { // C: if (maxind != k)
                let tmp_pivot: i32 = *pivot.add(k as usize); // C: int tmp_pivot = pivot[k];
                *pivot.add(k as usize) = *pivot.add(maxind as usize); // C: pivot[k] = pivot[maxind];
                *pivot.add(maxind as usize) = tmp_pivot; // C: pivot[maxind] = tmp_pivot;
                let mut j: i32 = 0; // C: int j=0;
                while j < n { // C: for (int j=0; j < n; j++)
                    let tmp: f64 = *A.add((k * n + j) as usize); // C: mjtNum tmp = A[k*n+j];
                    *A.add((k * n + j) as usize) = *A.add((maxind * n + j) as usize); // C: A[k*n+j] = A[maxind*n+j];
                    *A.add((maxind * n + j) as usize) = tmp; // C: A[maxind*n+j] = tmp;
                    j += 1;
                }
            }
            let mut i: i32 = k + 1; // C: int i=k+1;
            while i < n { // C: for (int i=k+1; i < n; i++)
                *A.add((i * n + k) as usize) /= *A.add((k * n + k) as usize); // C: A[i*n+k] /= A[k*n+k];
                let mut j: i32 = k + 1; // C: int j=k+1;
                while j < n { // C: for (int j=k+1; j < n; j++)
                    *A.add((i * n + j) as usize) -= *A.add((i * n + k) as usize) * *A.add((k * n + j) as usize); // C: A[i*n+j] -= A[i*n+k] * A[k*n+j];
                    j += 1;
                }
                i += 1;
            }
            k += 1;
        }
        1 // C: return 1;
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
    unsafe {
        let mut i: i32 = 0; // C: int i=0;
        while i < n { // C: for (int i=0; i < n; i++)
            *x.add(i as usize) = *b.add(*pivot.add(i as usize) as usize); // C: x[i] = b[pivot[i]];
            i += 1;
        }
        let mut i: i32 = 0; // C: int i=0;
        while i < n { // C: for (int i=0; i < n; i++)
            let mut j: i32 = 0; // C: int j=0;
            while j < i { // C: for (int j=0; j < i; j++)
                *x.add(i as usize) -= *LU.add((i * n + j) as usize) * *x.add(j as usize); // C: x[i] -= LU[i*n+j] * x[j];
                j += 1;
            }
            i += 1;
        }
        let mut i: i32 = n - 1; // C: int i=n-1;
        while i >= 0 { // C: for (int i=n-1; i >= 0; i--)
            let mut j: i32 = i + 1; // C: int j=i+1;
            while j < n { // C: for (int j=i+1; j < n; j++)
                *x.add(i as usize) -= *LU.add((i * n + j) as usize) * *x.add(j as usize); // C: x[i] -= LU[i*n+j] * x[j];
                j += 1;
            }
            *x.add(i as usize) /= *LU.add((i * n + i) as usize); // C: x[i] /= LU[i*n+i];
            i -= 1;
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
    todo!() // mju_factorLUSparse
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
    todo!() // mju_solveLUSparse
}

/// C: mju_solve3 (engine/engine_util_solve.h:118)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_solve3(x: *mut f64, A: *const f64, b: *const f64) {
    unsafe {
        // C: mjtNum M[3][4] = {{A[0],A[1],A[2],b[0]}, {A[3],A[4],A[5],b[1]}, {A[6],A[7],A[8],b[2]}}
        let mut M: [[f64; 4]; 3] = [
            [*A.add(0), *A.add(1), *A.add(2), *b.add(0)],
            [*A.add(3), *A.add(4), *A.add(5), *b.add(1)],
            [*A.add(6), *A.add(7), *A.add(8), *b.add(2)],
        ];
        for i in 0..3_usize { // C: for (int i=0; i<3; i++)
            let pivot = M[i][i]; // C: mjtNum pivot = M[i][i]
            for j in i..4_usize { // C: for (int j=i; j<4; j++)
                M[i][j] /= pivot; // C: M[i][j] /= pivot
            }
            for k in 0..3_usize { // C: for (int k=0; k<3; k++)
                if k != i { // C: if (k != i)
                    let factor = M[k][i]; // C: mjtNum factor = M[k][i]
                    for j in i..4_usize { // C: for (int j=i; j<4; j++)
                        M[k][j] -= factor * M[i][j]; // C: M[k][j] -= factor * M[i][j]
                    }
                }
            }
        }
        *x.add(0) = M[0][3]; // C: x[0] = M[0][3]
        *x.add(1) = M[1][3]; // C: x[1] = M[1][3]
        *x.add(2) = M[2][3]; // C: x[2] = M[2][3]
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
    unsafe {
        const eigEPS: f64 = 1e-15 * 1000.0; // C: static const mjtNum eigEPS = mjMINVAL * 1000
        let mut D: [f64; 9] = [0.0; 9]; // C: mjtNum D[9]
        let mut tmp: [f64; 9] = [0.0; 9]; // C: mjtNum tmp[9]
        let mut tau: f64;
        let mut t: f64;
        let mut c: f64;
        let mut iter: i32;
        let mut rk: i32;
        let mut ck: i32;
        let mut rotk: i32;

        // C: quat[0] = 1; quat[1] = quat[2] = quat[3] = 0
        *quat.add(0) = 1.0;
        *quat.add(1) = 0.0;
        *quat.add(2) = 0.0;
        *quat.add(3) = 0.0;

        // C: Jacobi iteration
        iter = 0;
        while iter < 500 { // C: for (iter=0; iter < 500; iter++)
            // C: mju_quat2Mat(eigvec, quat)
            crate::engine::engine_util_spatial::mju_quat2mat(eigvec, quat);
            // C: mji_mulMatTMat3(tmp, eigvec, mat) - use non-restrict version
            crate::engine::engine_util_blas::mju_mul_mat_t_mat3(tmp.as_mut_ptr(), eigvec, mat);
            // C: mji_mulMatMat3(D, tmp, eigvec)
            crate::engine::engine_util_blas::mju_mul_mat_mat3(D.as_mut_ptr(), tmp.as_ptr(), eigvec);

            // C: eigval[0] = D[0]; eigval[1] = D[4]; eigval[2] = D[8]
            *eigval.add(0) = D[0];
            *eigval.add(1) = D[4];
            *eigval.add(2) = D[8];

            // C: find max off-diagonal element
            if D[1].abs() > D[2].abs() && D[1].abs() > D[5].abs() {
                rk = 0; ck = 1; rotk = 2;
            } else if D[2].abs() > D[5].abs() {
                rk = 0; ck = 2; rotk = 1;
            } else {
                rk = 1; ck = 2; rotk = 0;
            }

            // C: if (mju_abs(D[3*rk+ck]) < eigEPS) break
            if D[(3 * rk + ck) as usize].abs() < eigEPS {
                break;
            }

            // C: tau = (D[4*ck]-D[4*rk])/(2*D[3*rk+ck])
            tau = (D[(4 * ck) as usize] - D[(4 * rk) as usize]) / (2.0 * D[(3 * rk + ck) as usize]);
            // C: if (tau >= 0) t = ... else t = ...
            if tau >= 0.0 {
                t = 1.0 / (tau + (1.0 + tau * tau).sqrt());
            } else {
                t = -1.0 / (-tau + (1.0 + tau * tau).sqrt());
            }
            // C: c = 1.0/mju_sqrt(1 + t*t)
            c = 1.0 / (1.0 + t * t).sqrt();

            // C: if (c > 1.0-eigEPS) break
            if c > 1.0 - eigEPS {
                break;
            }

            // C: express rotation as quaternion
            tmp[1] = 0.0; tmp[2] = 0.0; tmp[3] = 0.0;
            // C: tmp[rotk+1] = (tau >= 0 ? -mju_sqrt(0.5-0.5*c) : mju_sqrt(0.5-0.5*c))
            tmp[(rotk + 1) as usize] = if tau >= 0.0 {
                -(0.5 - 0.5 * c).sqrt()
            } else {
                (0.5 - 0.5 * c).sqrt()
            };
            // C: if (rotk == 1) tmp[rotk+1] = -tmp[rotk+1]
            if rotk == 1 {
                tmp[(rotk + 1) as usize] = -tmp[(rotk + 1) as usize];
            }
            // C: tmp[0] = mju_sqrt(1.0 - tmp[rotk+1]*tmp[rotk+1])
            tmp[0] = (1.0 - tmp[(rotk + 1) as usize] * tmp[(rotk + 1) as usize]).sqrt();
            // C: mju_normalize4(tmp)
            crate::engine::engine_util_blas::mju_normalize4(tmp.as_mut_ptr());

            // C: mju_mulQuat(quat, quat, tmp)
            crate::engine::engine_util_spatial::mju_mul_quat(quat, quat as *const f64, tmp.as_ptr());
            // C: mju_normalize4(quat)
            crate::engine::engine_util_blas::mju_normalize4(quat);

            iter += 1;
        }

        // C: sort eigenvalues in decreasing order (bubble sort: 0, 1, 0)
        for j in 0..3_i32 { // C: for (int j=0; j < 3; j++)
            let j1 = (j % 2) as usize; // C: int j1 = j%2

            // C: if (eigval[j1]+eigEPS < eigval[j1+1])
            if *eigval.add(j1) + eigEPS < *eigval.add(j1 + 1) {
                // C: swap eigenvalues
                t = *eigval.add(j1); // C: t = eigval[j1]
                *eigval.add(j1) = *eigval.add(j1 + 1); // C: eigval[j1] = eigval[j1+1]
                *eigval.add(j1 + 1) = t; // C: eigval[j1+1] = t

                // C: rotate quaternion
                tmp[0] = 0.707106781186548; // C: tmp[0] = 0.707106781186548
                tmp[1] = 0.0; tmp[2] = 0.0; tmp[3] = 0.0;
                tmp[((j1 as i32 + 2) % 3 + 1) as usize] = tmp[0]; // C: tmp[(j1+2)%3+1] = tmp[0]
                crate::engine::engine_util_spatial::mju_mul_quat(quat, quat as *const f64, tmp.as_ptr());
                crate::engine::engine_util_blas::mju_normalize4(quat);
            }
        }

        // C: mju_quat2Mat(eigvec, quat)
        crate::engine::engine_util_spatial::mju_quat2mat(eigvec, quat);

        iter // C: return iter
    }
}

/// C: mju_QCQP2 (engine/engine_util_solve.h:126)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_qcqp2(res: *mut f64, Ain: *const f64, bin: *const f64, d: *const f64, r: f64) -> i32 {
    todo!() // mju_QCQP2
}

/// C: mju_QCQP3 (engine/engine_util_solve.h:131)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_qcqp3(res: *mut f64, Ain: *const f64, bin: *const f64, d: *const f64, r: f64) -> i32 {
    todo!() // mju_QCQP3
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
    todo!() // mju_QCQP
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
    todo!() // mju_boxQP
}

/// C: mju_boxQPmalloc (engine/engine_util_solve.h:146)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_box_q_pmalloc(res: *mut *mut f64, R: *mut *mut f64, index: *mut *mut i32, H: *mut *mut f64, g: *mut *mut f64, n: i32, lower: *mut *mut f64, upper: *mut *mut f64) {
    todo!() // mju_boxQPmalloc
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
    todo!() // mju_boxQPoption
}

