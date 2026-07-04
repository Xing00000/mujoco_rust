//! Port of: engine/engine_util_sparse.h
//! IR hash: 1b139f44af8230f9
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mju_dotSparse2 (engine/engine_util_sparse.h:32)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_dot_sparse2(vec1: *const f64, ind1: *const i32, nnz1: i32, vec2: *const f64, ind2: *const i32, nnz2: i32) -> f64 {
    unsafe {
        let mut i1: i32 = 0; // C: int i1 = 0
        let mut i2: i32 = 0; // C: int i2 = 0
        let mut res: f64 = 0.0; // C: mjtNum res = 0

        // C: if (!nnz1 || !nnz2) { return 0.0; }
        if nnz1 == 0 || nnz2 == 0 {
            return 0.0;
        }

        // C: while (i1 < nnz1 && i2 < nnz2)
        while i1 < nnz1 && i2 < nnz2 {
            let adr1: i32 = *ind1.add(i1 as usize); // C: int adr1 = ind1[i1]
            let adr2: i32 = *ind2.add(i2 as usize); // C: int adr2 = ind2[i2]

            if adr1 == adr2 {
                // C: res += vec1[i1++] * vec2[i2++]
                res += *vec1.add(i1 as usize) * *vec2.add(i2 as usize);
                i1 += 1;
                i2 += 1;
            } else if adr1 < adr2 {
                i1 += 1; // C: i1++
            } else {
                i2 += 1; // C: i2++
            }
        }

        res // C: return res
    }
}

/// C: mju_dotSparseX3 (engine/engine_util_sparse.h:36)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_dot_sparse_x3(res0: *mut f64, res1: *mut f64, res2: *mut f64, vec10: *const f64, vec11: *const f64, vec12: *const f64, vec2: *const f64, nnz1: i32, ind1: *const i32) {
    unsafe {
        let mut i: i32 = 0; // C: int i = 0
        let mut RES0: f64 = 0.0; // C: mjtNum RES0 = 0
        let mut RES1: f64 = 0.0; // C: mjtNum RES1 = 0
        let mut RES2: f64 = 0.0; // C: mjtNum RES2 = 0

        // C: for (; i < nnz1; i++)
        while i < nnz1 {
            let v2: f64 = *vec2.add(*ind1.add(i as usize) as usize); // C: mjtNum v2 = vec2[ind1[i]]
            RES0 += *vec10.add(i as usize) * v2; // C: RES0 += vec10[i] * v2
            RES1 += *vec11.add(i as usize) * v2; // C: RES1 += vec11[i] * v2
            RES2 += *vec12.add(i as usize) * v2; // C: RES2 += vec12[i] * v2
            i += 1;
        }

        *res0 = RES0; // C: *res0 = RES0
        *res1 = RES1; // C: *res1 = RES1
        *res2 = RES2; // C: *res2 = RES2
    }
}

/// C: mju_dense2sparse (engine/engine_util_sparse.h:42)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_dense2sparse(res: *mut f64, mat: *const f64, nr: i32, nc: i32, rownnz: *mut i32, rowadr: *mut i32, colind: *mut i32, nnz: i32) -> i32 {
    unsafe {
        // C: if (nnz <= 0) { return 1; }
        if nnz <= 0 {
            return 1;
        }

        let mut adr: i32 = 0; // C: int adr = 0

        // C: for (int r=0; r < nr; r++)
        for r in 0..nr {
            *rownnz.add(r as usize) = 0; // C: rownnz[r] = 0
            *rowadr.add(r as usize) = adr; // C: rowadr[r] = adr

            // C: for (int c=0; c < nc; c++)
            for c in 0..nc {
                // C: if (mat[r*nc+c] != 0.0)
                if *mat.add((r * nc + c) as usize) != 0.0 {
                    // C: if (adr >= nnz) { return 1; }
                    if adr >= nnz {
                        return 1;
                    }
                    *colind.add(adr as usize) = c; // C: colind[adr] = c
                    *rownnz.add(r as usize) += 1; // C: rownnz[r]++
                    *res.add(adr as usize) = *mat.add((r * nc + c) as usize); // C: res[adr++] = mat[r*nc+c]
                    adr += 1;
                }
            }
        }

        0 // C: return 0
    }
}

/// C: mju_sparse2dense (engine/engine_util_sparse.h:46)
/// Calls: mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_sparse2dense(res: *mut f64, mat: *const f64, nr: i32, nc: i32, rownnz: *const i32, rowadr: *const i32, colind: *const i32) {
    unsafe {
        crate::engine::engine_util_blas::mju_zero(res, nr * nc); // C: mju_zero(res, nr*nc)

        // C: for (int r=0; r < nr; r++)
        for r in 0..nr {
            // C: for (int i=0; i < rownnz[r]; i++)
            for i in 0..*rownnz.add(r as usize) {
                let row_adr: i32 = *rowadr.add(r as usize); // C: rowadr[r]
                let col: i32 = *colind.add((row_adr + i) as usize); // C: colind[rowadr[r]+i]
                // C: res[r*nc + colind[rowadr[r]+i]] = mat[rowadr[r]+i]
                *res.add((r * nc + col) as usize) = *mat.add((row_adr + i) as usize);
            }
        }
    }
}

/// C: mju_sym2dense (engine/engine_util_sparse.h:50)
/// Calls: mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_sym2dense(res: *mut f64, mat: *const f64, n: i32, rownnz: *const i32, rowadr: *const i32, colind: *const i32) {
    unsafe {
        crate::engine::engine_util_blas::mju_zero(res, n * n); // C: mju_zero(res, n*n)

        // C: for (int i = 0; i < n; i++)
        for i in 0..n {
            let adr: i32 = *rowadr.add(i as usize); // C: int adr = rowadr[i]

            // C: for (int j = 0; j < rownnz[i]; j++)
            for j in 0..*rownnz.add(i as usize) {
                let col: i32 = *colind.add((adr + j) as usize); // C: int col = colind[adr+j]

                // C: if (col <= i)
                if col <= i {
                    let val: f64 = *mat.add((adr + j) as usize); // C: mat[adr+j]
                    *res.add((i * n + col) as usize) = val; // C: res[i*n+col] = mat[adr+j]
                    *res.add((col * n + i) as usize) = val; // C: res[col*n+i] = mat[adr+j]
                }
            }
        }
    }
}

/// C: mju_copySparse (engine/engine_util_sparse.h:54)
/// Calls: mju_copy
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_copy_sparse(res: *mut f64, mat: *const f64, rownnz: *const i32, rowadr: *const i32, row: *const i32, nrow: i32) {
    unsafe {
        // C: for (int i=0; i < nrow; i++)
        for i in 0..nrow {
            let r: i32 = *row.add(i as usize); // C: int r = row[i]
            let adr: i32 = *rowadr.add(r as usize); // C: rowadr[r]
            let nnz: i32 = *rownnz.add(r as usize); // C: rownnz[r]
            // C: mju_copy(res + rowadr[r], mat + rowadr[r], rownnz[r])
            crate::engine::engine_util_blas::mju_copy(res.add(adr as usize), mat.add(adr as usize), nnz);
        }
    }
}

/// C: mju_zeroSparse (engine/engine_util_sparse.h:58)
/// Calls: mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_zero_sparse(res: *mut f64, rownnz: *const i32, rowadr: *const i32, row: *const i32, nrow: i32) {
    unsafe {
        // C: for (int i=0; i < nrow; i++)
        for i in 0..nrow {
            let r: i32 = *row.add(i as usize); // C: int r = row[i]
            let adr: i32 = *rowadr.add(r as usize); // C: rowadr[r]
            let nnz: i32 = *rownnz.add(r as usize); // C: rownnz[r]
            // C: mju_zero(res + rowadr[r], rownnz[r])
            crate::engine::engine_util_blas::mju_zero(res.add(adr as usize), nnz);
        }
    }
}

/// C: mju_mulMatVecSparse (engine/engine_util_sparse.h:61)
/// Calls: mju_dotSparse
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_mul_mat_vec_sparse(res: *mut f64, mat: *const f64, vec: *const f64, nr: i32, rownnz: *const i32, rowadr: *const i32, colind: *const i32, rowsuper: *const i32) {
    unsafe {
        // C: for (int r=0; r < nr; r++)
        for r in 0..nr {
            let adr: i32 = *rowadr.add(r as usize); // C: rowadr[r]
            let nnz: i32 = *rownnz.add(r as usize); // C: rownnz[r]
            // C: res[r] = mju_dotSparse(mat+rowadr[r], vec, rownnz[r], colind+rowadr[r])
            *res.add(r as usize) = mju_dot_sparse(mat.add(adr as usize), vec, nnz, colind.add(adr as usize));
        }
    }
}

/// C: mju_mulMatTVecSparse (engine/engine_util_sparse.h:66)
/// Calls: mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_mul_mat_t_vec_sparse(res: *mut f64, mat: *const f64, vec: *const f64, nr: i32, nc: i32, rownnz: *const i32, rowadr: *const i32, colind: *const i32) {
    unsafe {
        crate::engine::engine_util_blas::mju_zero(res, nc); // C: mju_zero(res, nc)

        // C: for (int i=0; i < nr; i++)
        for i in 0..nr {
            // C: if (vec[i] != 0.0)
            if *vec.add(i as usize) != 0.0 {
                let nnz: i32 = *rownnz.add(i as usize); // C: int nnz = rownnz[i]
                let adr: i32 = *rowadr.add(i as usize); // C: int adr = rowadr[i]
                let vec_i: f64 = *vec.add(i as usize); // C: vec[i]

                // C: for (int j=0; j < nnz; j++)
                for j in 0..nnz {
                    let col: i32 = *colind.add((adr + j) as usize); // C: colind[adr+j]
                    // C: res[colind[adr+j]] += mat[adr+j] * vec[i]
                    *res.add(col as usize) += *mat.add((adr + j) as usize) * vec_i;
                }
            }
        }
    }
}

/// C: mju_addToMatSparse (engine/engine_util_sparse.h:70)
/// Calls: mju_combineSparse
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_add_to_mat_sparse(dst: *mut f64, rownnz: *mut i32, rowadr: *mut i32, colind: *mut i32, nr: i32, M: *const f64, M_rownnz: *const i32, M_rowadr: *const i32, M_colind: *const i32) {
    todo!() // mju_addToMatSparse
}

/// C: mju_addToSymSparse (engine/engine_util_sparse.h:75)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_add_to_sym_sparse(res: *mut f64, mat: *const f64, n: i32, rownnz: *const i32, rowadr: *const i32, colind: *const i32, flg_upper: i32) {
    todo!() // mju_addToSymSparse
}

/// C: mju_mulSymVecSparse (engine/engine_util_sparse.h:81)
/// Calls: mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_mul_sym_vec_sparse(res: *mut f64, mat: *const f64, vec: *const f64, n: i32, rownnz: *const i32, rowadr: *const i32, colind: *const i32) {
    todo!() // mju_mulSymVecSparse
}

/// C: mju_compressSparse (engine/engine_util_sparse.h:85)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_compress_sparse(mat: *mut f64, nr: i32, nc: i32, rownnz: *mut i32, rowadr: *mut i32, colind: *mut i32, minval: f64) -> i32 {
    todo!() // mju_compressSparse
}

/// C: mju_combineSparseCount (engine/engine_util_sparse.h:89)
#[allow(unused_variables, non_snake_case)]
pub fn mju_combine_sparse_count(a_nnz: i32, b_nnz: i32, a_ind: *const i32, b_ind: *const i32) -> i32 {
    todo!() // mju_combineSparseCount
}

/// C: mju_combineSparseInc (engine/engine_util_sparse.h:92)
/// Calls: mju_addToSclScl, mju_compare, mju_scl
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_combine_sparse_inc(dst: *mut f64, src: *const f64, n: i32, a: f64, b: f64, dst_nnz: i32, src_nnz: i32, dst_ind: *const i32, src_ind: *const i32) {
    todo!() // mju_combineSparseInc
}

/// C: mju_addToSclSparseInc (engine/engine_util_sparse.h:96)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_add_to_scl_sparse_inc(dst: *mut f64, src: *const f64, nnzdst: i32, inddst: *const i32, nnzsrc: i32, indsrc: *const i32, scl: f64) {
    todo!() // mju_addToSclSparseInc
}

/// C: mju_addToSparseMat (engine/engine_util_sparse.h:101)
/// Calls: mju_addToScl, mju_compare, mju_copyInt, mju_transpose
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_add_to_sparse_mat(dst: *mut f64, src: *const f64, n: i32, nrow: i32, scl: f64, dst_nnz: i32, src_nnz: i32, dst_ind: *mut i32, src_ind: *const i32, buf: *mut f64, buf_ind: *mut i32) -> i32 {
    todo!() // mju_addToSparseMat
}

/// C: mju_addChains (engine/engine_util_sparse.h:106)
/// Calls: mju_compare, mju_copyInt
#[allow(unused_variables, non_snake_case)]
pub fn mju_add_chains(res: *mut i32, n: i32, NV1: i32, NV2: i32, chain1: *const i32, chain2: *const i32) -> i32 {
    todo!() // mju_addChains
}

/// C: mju_transposeSparse (engine/engine_util_sparse.h:110)
/// Calls: mju_zeroInt
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_transpose_sparse(res: *mut f64, mat: *const f64, nr: i32, nc: i32, res_rownnz: *mut i32, res_rowadr: *mut i32, res_colind: *mut i32, res_rowsuper: *mut i32, rownnz: *const i32, rowadr: *const i32, colind: *const i32) {
    todo!() // mju_transposeSparse
}

/// C: mju_superSparse (engine/engine_util_sparse.h:115)
/// Calls: mju_compare
#[allow(unused_variables, non_snake_case)]
pub fn mju_super_sparse(nr: i32, rowsuper: *mut i32, rownnz: *const i32, rowadr: *const i32, colind: *const i32) {
    todo!() // mju_superSparse
}

/// C: mju_sqrMatTDSparse (engine/engine_util_sparse.h:119)
/// Calls: mj_freeStack, mj_markStack, mju_dot, mju_zeroInt
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_sqr_mat_td_sparse(res: *mut f64, mat: *const f64, matT: *const f64, diag: *const f64, nr: i32, nc: i32, res_rownnz: *mut i32, res_rowadr: *const i32, res_colind: *mut i32, rownnz: *const i32, rowadr: *const i32, colind: *const i32, rowsuper: *const i32, rownnzT: *const i32, rowadrT: *const i32, colindT: *const i32, rowsuperT: *const i32, d: *mut mjData, diagind: *mut i32) {
    todo!() // mju_sqrMatTDSparse
}

/// C: mju_sqrMatTDSparse_row (engine/engine_util_sparse.h:129)
/// Calls: mj_freeStack, mj_markStack, mju_copyInt
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_sqr_mat_td_sparse_row(res: *mut f64, mat: *const f64, matT: *const f64, diag: *const f64, nr: i32, nc: i32, res_rownnz: *mut i32, res_rowadr: *const i32, res_colind: *mut i32, rownnz: *const i32, rowadr: *const i32, colind: *const i32, rowsuper: *const i32, rownnzT: *const i32, rowadrT: *const i32, colindT: *const i32, rowsuperT: *const i32, d: *mut mjData, diagind: *mut i32) {
    todo!() // mju_sqrMatTDSparse_row
}

/// C: mju_sqrMatTDSparseCount (engine/engine_util_sparse.h:139)
/// Calls: mj_freeStack, mj_markStack
#[allow(unused_variables, non_snake_case)]
pub fn mju_sqr_mat_td_sparse_count(res_rownnz: *mut i32, res_rowadr: *mut i32, nr: i32, rownnz: *const i32, rowadr: *const i32, colind: *const i32, rownnzT: *const i32, rowadrT: *const i32, colindT: *const i32, rowsuperT: *const i32, d: *mut mjData, flg_upper: i32) -> i32 {
    todo!() // mju_sqrMatTDSparseCount
}

/// C: mju_sqrMatTDSparseSymbolic (engine/engine_util_sparse.h:148)
/// Calls: mj_freeStack, mj_markStack, mju_copyInt, mju_zeroInt
#[allow(unused_variables, non_snake_case)]
pub fn mju_sqr_mat_td_sparse_symbolic(res_rownnz: *mut i32, res_rowadr: *mut i32, res_colind: *mut i32, res_diagind: *mut i32, nr: i32, nc: i32, rownnz: *const i32, rowadr: *const i32, colind: *const i32, rownnzT: *const i32, rowadrT: *const i32, colindT: *const i32, rowsuperT: *const i32, d: *mut mjData) -> i32 {
    todo!() // mju_sqrMatTDSparseSymbolic
}

/// C: mju_sqrMatTDSparseNumeric (engine/engine_util_sparse.h:155)
/// Calls: mj_freeStack, mj_markStack, mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_sqr_mat_td_sparse_numeric(res: *mut f64, nc: i32, res_rownnz: *const i32, res_rowadr: *const i32, res_colind: *const i32, res_diagind: *const i32, mat: *const f64, rownnz: *const i32, rowadr: *const i32, colind: *const i32, matT: *const f64, rownnzT: *const i32, rowadrT: *const i32, colindT: *const i32, rowsuperT: *const i32, diag: *const f64, d: *mut mjData) {
    todo!() // mju_sqrMatTDSparseNumeric
}

/// C: mju_sqrMatTDUncompressedInit (engine/engine_util_sparse.h:163)
#[allow(unused_variables, non_snake_case)]
pub fn mju_sqr_mat_td_uncompressed_init(res_rowadr: *mut i32, nc: i32) {
    todo!() // mju_sqrMatTDUncompressedInit
}

/// C: mju_block (engine/engine_util_sparse.h:166)
/// Calls: mju_gather
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_block(res: *mut f64, mat: *const f64, nc_mat: i32, nc_res: i32, nr: i32, perm_r: *const i32, perm_c: *const i32) {
    todo!() // mju_block
}

/// C: mju_blockDiag (engine/engine_util_sparse.h:170)
/// Calls: mju_block
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_block_diag(res: *mut f64, mat: *const f64, nc_mat: i32, nc_res: i32, nb: i32, perm_r: *const i32, perm_c: *const i32, block_nr: *const i32, block_nc: *const i32, block_r: *const i32, block_c: *const i32) {
    todo!() // mju_blockDiag
}

/// C: mju_blockSparse (engine/engine_util_sparse.h:176)
/// Calls: mju_copy
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_block_sparse(res: *mut f64, res_rownnz: *mut i32, res_rowadr: *mut i32, res_colind: *mut i32, mat: *const f64, rownnz: *const i32, rowadr: *const i32, colind: *const i32, nr: i32, perm_r: *const i32, perm_c: *const i32, col_offset: i32, res_offset: i32, res2: *mut f64, mat2: *const f64) {
    todo!() // mju_blockSparse
}

/// C: mju_blockDiagSparse (engine/engine_util_sparse.h:185)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_block_diag_sparse(res: *mut f64, res_rownnz: *mut i32, res_rowadr: *mut i32, res_colind: *mut i32, mat: *const f64, rownnz: *const i32, rowadr: *const i32, colind: *const i32, nr: i32, nb: i32, perm_r: *const i32, perm_c: *const i32, block_r: *const i32, block_c: *const i32, res2: *mut f64, mat2: *const f64) {
    todo!() // mju_blockDiagSparse
}

/// C: mju_dotSparse (engine/engine_util_sparse.h:197)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_dot_sparse(vec1: *const f64, vec2: *const f64, nnz1: i32, ind1: *const i32) -> f64 {
    unsafe {
        let mut i: i32 = 0; // C: int i = 0
        let n_4: i32 = nnz1 - 4; // C: int n_4 = nnz1 - 4
        let mut res0: f64 = 0.0; // C: mjtNum res0 = 0
        let mut res1: f64 = 0.0; // C: mjtNum res1 = 0
        let mut res2: f64 = 0.0; // C: mjtNum res2 = 0
        let mut res3: f64 = 0.0; // C: mjtNum res3 = 0

        // C: for (; i <= n_4; i+=4)
        while i <= n_4 {
            // C: res0 += vec1[i+0] * vec2[ind1[i+0]]
            res0 += *vec1.add((i + 0) as usize) * *vec2.add(*ind1.add((i + 0) as usize) as usize);
            // C: res1 += vec1[i+1] * vec2[ind1[i+1]]
            res1 += *vec1.add((i + 1) as usize) * *vec2.add(*ind1.add((i + 1) as usize) as usize);
            // C: res2 += vec1[i+2] * vec2[ind1[i+2]]
            res2 += *vec1.add((i + 2) as usize) * *vec2.add(*ind1.add((i + 2) as usize) as usize);
            // C: res3 += vec1[i+3] * vec2[ind1[i+3]]
            res3 += *vec1.add((i + 3) as usize) * *vec2.add(*ind1.add((i + 3) as usize) as usize);
            i += 4;
        }

        // C: res = (res0 + res2) + (res1 + res3)
        let mut res: f64 = (res0 + res2) + (res1 + res3);

        // C: for (; i < nnz1; i++) { res += vec1[i] * vec2[ind1[i]]; }
        while i < nnz1 {
            res += *vec1.add(i as usize) * *vec2.add(*ind1.add(i as usize) as usize);
            i += 1;
        }

        res // C: return res
    }
}

/// C: mju_compare (engine/engine_util_sparse.h:231)
#[allow(unused_variables, non_snake_case)]
pub fn mju_compare(vec1: *const i32, vec2: *const i32, n: i32) -> i32 {
    todo!() // mju_compare
}

/// C: mj_mergeSorted (engine/engine_util_sparse.h:243)
/// Calls: mju_compare
#[allow(unused_variables, non_snake_case)]
pub fn mj_merge_sorted(merge: *mut i32, chain1: *const i32, n1: i32, chain2: *const i32, n2: i32) -> i32 {
    todo!() // mj_mergeSorted
}

/// C: mju_addToSclScl (engine/engine_util_sparse.h:297)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_add_to_scl_scl(res: *mut f64, vec: *const f64, scl1: f64, scl2: f64, n: i32) {
    todo!() // mju_addToSclScl
}

/// C: mju_combineSparse (engine/engine_util_sparse.h:311)
/// Calls: mju_addToSclScl, mju_combineSparseCount, mju_compare
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_combine_sparse(dst: *mut f64, src: *const f64, a: f64, b: f64, dst_nnz: i32, src_nnz: i32, dst_ind: *mut i32, src_ind: *const i32) -> i32 {
    todo!() // mju_combineSparse
}

