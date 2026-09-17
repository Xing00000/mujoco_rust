//! Port of: engine/engine_util_sparse.c
//! IR hash: fd5288ad7242f001
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mju_dotSparseX3 (engine/engine_util_sparse.c:32)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_dot_sparse_x3(res0: *mut f64, res1: *mut f64, res2: *mut f64, vec10: *const f64, vec11: *const f64, vec12: *const f64, vec2: *const f64, nnz1: i32, ind1: *const i32) {
    todo!() // cxx:_mju_dotSparseX3
}

/// C: mju_dotSparse2 (engine/engine_util_sparse.c:61)
/// Calls: cxx:__ZNK6mujoco4user8FilePath5emptyEv
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_dot_sparse2(vec1: *const f64, ind1: *const i32, nnz1: i32, vec2: *const f64, ind2: *const i32, nnz2: i32) -> f64 {
    todo!() // cxx:_mju_dotSparse2
}

/// C: mju_dense2sparse (engine/engine_util_sparse.c:94)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_dense2sparse(res: *mut f64, mat: *const f64, nr: i32, nc: i32, rownnz: *mut i32, rowadr: *mut i32, colind: *mut i32, nnz: i32) -> i32 {
    // SAFETY: caller guarantees all pointers are valid for their respective dimensions
    unsafe {
        if nnz <= 0 {
            return 1;
        }

        let mut adr: i32 = 0;

        // find non-zeros and construct sparse
        for r in 0..nr {
            // init row
            *rownnz.add(r as usize) = 0;
            *rowadr.add(r as usize) = adr;

            // find non-zeros
            for c in 0..nc {
                if *mat.add((r * nc + c) as usize) != 0.0 {
                    // check for out of bounds
                    if adr >= nnz {
                        return 1;
                    }

                    // record index and count
                    *colind.add(adr as usize) = c;
                    *rownnz.add(r as usize) += 1;

                    // copy element
                    *res.add(adr as usize) = *mat.add((r * nc + c) as usize);
                    adr += 1;
                }
            }
        }
        0
    }
}

/// C: mju_sparse2dense (engine/engine_util_sparse.c:130)
/// Calls: cxx:_mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_sparse2dense(res: *mut f64, mat: *const f64, nr: i32, nc: i32, rownnz: *const i32, rowadr: *const i32, colind: *const i32) {
    // SAFETY: caller guarantees all pointers valid for their dimensions
    unsafe {
        // clear
        crate::engine::engine_util_blas::mju_zero(res, nr * nc);

        // copy non-zeros
        for r in 0..nr as usize {
            for i in 0..*rownnz.add(r) as usize {
                let adr = *rowadr.add(r) as usize;
                *res.add(r * nc as usize + *colind.add(adr + i) as usize) = *mat.add(adr + i);
            }
        }
    }
}

/// C: mju_sym2dense (engine/engine_util_sparse.c:145)
/// Calls: cxx:_mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_sym2dense(res: *mut f64, mat: *const f64, n: i32, rownnz: *const i32, rowadr: *const i32, colind: *const i32) {
    use crate::engine::engine_util_blas::mju_zero;

    // SAFETY: all pointers valid per caller contract
    unsafe {
        mju_zero(res, n * n);
        for i in 0..n as usize {
            let adr = *rowadr.add(i) as usize;
            for j in 0..*rownnz.add(i) as usize {
                let col = *colind.add(adr + j) as usize;
                if col <= i {
                    *res.add(i * n as usize + col) = *mat.add(adr + j);
                    *res.add(col * n as usize + i) = *mat.add(adr + j);
                }
            }
        }
    }
}

/// C: mju_copySparse (engine/engine_util_sparse.c:162)
/// Calls: cxx:_mju_copy
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_copy_sparse(res: *mut f64, mat: *const f64, rownnz: *const i32, rowadr: *const i32, row: *const i32, nrow: i32) {
    todo!() // cxx:_mju_copySparse
}

/// C: mju_zeroSparse (engine/engine_util_sparse.c:171)
/// Calls: cxx:_mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_zero_sparse(res: *mut f64, rownnz: *const i32, rowadr: *const i32, row: *const i32, nrow: i32) {
    todo!() // cxx:_mju_zeroSparse
}

/// C: mju_mulMatVecSparse (engine/engine_util_sparse.c:182)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_sparse.h:_mju_dotSparse
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_mul_mat_vec_sparse(res: *mut f64, mat: *const f64, vec: *const f64, nr: i32, rownnz: *const i32, rowadr: *const i32, colind: *const i32, rowsuper: *const i32) {
    todo!() // cxx:_mju_mulMatVecSparse
}

/// C: mju_mulMatTVecSparse (engine/engine_util_sparse.c:196)
/// Calls: cxx:_mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_mul_mat_t_vec_sparse(res: *mut f64, mat: *const f64, vec: *const f64, nr: i32, nc: i32, rownnz: *const i32, rowadr: *const i32, colind: *const i32) {
    todo!() // cxx:_mju_mulMatTVecSparse
}

/// C: mju_addToMatSparse (engine/engine_util_sparse.c:221)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_sparse.h:_mju_combineSparse
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_add_to_mat_sparse(dst: *mut f64, rownnz: *mut i32, rowadr: *mut i32, colind: *mut i32, nr: i32, M: *const f64, M_rownnz: *const i32, M_rowadr: *const i32, M_colind: *const i32) {
    todo!() // cxx:_mju_addToMatSparse
}

/// C: mju_addToSymSparse (engine/engine_util_sparse.c:232)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_add_to_sym_sparse(res: *mut f64, mat: *const f64, n: i32, rownnz: *const i32, rowadr: *const i32, colind: *const i32, flg_upper: i32) {
    todo!() // cxx:_mju_addToSymSparse
}

/// C: mju_mulSymVecSparse (engine/engine_util_sparse.c:257)
/// Calls: cxx:_mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_mul_sym_vec_sparse(res: *mut f64, mat: *const f64, vec: *const f64, n: i32, rownnz: *const i32, rowadr: *const i32, colind: *const i32) {
    todo!() // cxx:_mju_mulSymVecSparse
}

/// C: mju_combineSparseCount (engine/engine_util_sparse.c:283)
#[allow(unused_variables, non_snake_case)]
pub fn mju_combine_sparse_count(a_nnz: i32, b_nnz: i32, a_ind: *const i32, b_ind: *const i32) -> i32 {
    todo!() // cxx:_mju_combineSparseCount
}

/// C: mju_combineSparseInc (engine/engine_util_sparse.c:310)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_sparse.h:_mju_addToSclScl, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_sparse.h:_mju_compare, cxx:_mju_scl
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_combine_sparse_inc(dst: *mut f64, src: *const f64, n: i32, a: f64, b: f64, dst_nnz: i32, src_nnz: i32, dst_ind: *const i32, src_ind: *const i32) {
    todo!() // cxx:_mju_combineSparseInc
}

/// C: mju_addToSclSparseInc (engine/engine_util_sparse.c:358)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_add_to_scl_sparse_inc(dst: *mut f64, src: *const f64, nnzdst: i32, inddst: *const i32, nnzsrc: i32, indsrc: *const i32, scl: f64) {
    todo!() // cxx:_mju_addToSclSparseInc
}

/// C: mju_addToSparseMat (engine/engine_util_sparse.c:384)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_sparse.h:_mju_compare, cxx:_mju_addToScl, cxx:_mju_copyInt, cxx:_mju_transpose
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_add_to_sparse_mat(dst: *mut f64, src: *const f64, n: i32, nrow: i32, scl: f64, dst_nnz: i32, src_nnz: i32, dst_ind: *mut i32, src_ind: *const i32, buf: *mut f64, buf_ind: *mut i32) -> i32 {
    todo!() // cxx:_mju_addToSparseMat
}

/// C: mju_addChains (engine/engine_util_sparse.c:450)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_sparse.h:_mju_compare, cxx:_mju_copyInt
#[allow(unused_variables, non_snake_case)]
pub fn mju_add_chains(res: *mut i32, n: i32, NV1: i32, NV2: i32, chain1: *const i32, chain2: *const i32) -> i32 {
    todo!() // cxx:_mju_addChains
}

/// C: mju_compressSparse (engine/engine_util_sparse.c:499)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_compress_sparse(mat: *mut f64, nr: i32, nc: i32, rownnz: *mut i32, rowadr: *mut i32, colind: *mut i32, minval: f64) -> i32 {
    todo!() // cxx:_mju_compressSparse
}

/// C: mju_transposeSparse (engine/engine_util_sparse.c:531)
/// Calls: cxx:_mju_zeroInt
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_transpose_sparse(res: *mut f64, mat: *const f64, nr: i32, nc: i32, res_rownnz: *mut i32, res_rowadr: *mut i32, res_colind: *mut i32, res_rowsuper: *mut i32, rownnz: *const i32, rowadr: *const i32, colind: *const i32) {
    todo!() // cxx:_mju_transposeSparse
}

/// C: mju_superSparse (engine/engine_util_sparse.c:606)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_sparse.h:_mju_compare
#[allow(unused_variables, non_snake_case)]
pub fn mju_super_sparse(nr: i32, rowsuper: *mut i32, rownnz: *const i32, rowadr: *const i32, colind: *const i32) {
    todo!() // cxx:_mju_superSparse
}

/// C: mju_sqrMatTDSparseCount (engine/engine_util_sparse.c:641)
/// Calls: cxx:_mj_freeStack, cxx:_mj_markStack, cxx:_mj_stackAllocInfo
#[allow(unused_variables, non_snake_case)]
pub fn mju_sqr_mat_td_sparse_count(res_rownnz: *mut i32, res_rowadr: *mut i32, nr: i32, rownnz: *const i32, rowadr: *const i32, colind: *const i32, rownnzT: *const i32, rowadrT: *const i32, colindT: *const i32, rowsuperT: *const i32, d: *mut mjData, flg_upper: i32) -> i32 {
    todo!() // cxx:_mju_sqrMatTDSparseCount
}

/// C: mju_sqrMatTDUncompressedInit (engine/engine_util_sparse.c:733)
#[allow(unused_variables, non_snake_case)]
pub fn mju_sqr_mat_td_uncompressed_init(res_rowadr: *mut i32, nc: i32) {
    todo!() // cxx:_mju_sqrMatTDUncompressedInit
}

/// C: mju_sqrMatTDSparseSymbolic (engine/engine_util_sparse.c:751)
/// Calls: cxx:_mj_freeStack, cxx:_mj_markStack, cxx:_mj_stackAllocInfo, cxx:_mju_copyInt, cxx:_mju_zeroInt
#[allow(unused_variables, non_snake_case)]
pub fn mju_sqr_mat_td_sparse_symbolic(res_rownnz: *mut i32, res_rowadr: *mut i32, res_colind: *mut i32, res_diagind: *mut i32, nr: i32, nc: i32, rownnz: *const i32, rowadr: *const i32, colind: *const i32, rownnzT: *const i32, rowadrT: *const i32, colindT: *const i32, rowsuperT: *const i32, d: *mut mjData) -> i32 {
    todo!() // cxx:_mju_sqrMatTDSparseSymbolic
}

/// C: mju_sqrMatTDSparseNumeric (engine/engine_util_sparse.c:915)
/// Calls: cxx:_mj_freeStack, cxx:_mj_markStack, cxx:_mj_stackAllocInfo, cxx:_mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_sqr_mat_td_sparse_numeric(res: *mut f64, nc: i32, res_rownnz: *const i32, res_rowadr: *const i32, res_colind: *const i32, res_diagind: *const i32, mat: *const f64, rownnz: *const i32, rowadr: *const i32, colind: *const i32, matT: *const f64, rownnzT: *const i32, rowadrT: *const i32, colindT: *const i32, rowsuperT: *const i32, diag: *const f64, d: *mut mjData) {
    todo!() // cxx:_mju_sqrMatTDSparseNumeric
}

/// C: mju_sqrMatTDSparse (engine/engine_util_sparse.c:1059)
/// Calls: cxx:_mj_freeStack, cxx:_mj_markStack, cxx:_mj_stackAllocInfo, cxx:_mju_dot, cxx:_mju_zeroInt
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_sqr_mat_td_sparse(res: *mut f64, mat: *const f64, matT: *const f64, diag: *const f64, nr: i32, nc: i32, res_rownnz: *mut i32, res_rowadr: *const i32, res_colind: *mut i32, rownnz: *const i32, rowadr: *const i32, colind: *const i32, rowsuper: *const i32, rownnzT: *const i32, rowadrT: *const i32, colindT: *const i32, rowsuperT: *const i32, d: *mut mjData, diagind: *mut i32) {
    todo!() // cxx:_mju_sqrMatTDSparse
}

/// C: mju_sqrMatTDSparse_row (engine/engine_util_sparse.c:1259)
/// Calls: cxx:_mj_freeStack, cxx:_mj_markStack, cxx:_mj_stackAllocInfo, cxx:_mju_copyInt
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_sqr_mat_td_sparse_row(res: *mut f64, mat: *const f64, matT: *const f64, diag: *const f64, nr: i32, nc: i32, res_rownnz: *mut i32, res_rowadr: *const i32, res_colind: *mut i32, rownnz: *const i32, rowadr: *const i32, colind: *const i32, rowsuper: *const i32, rownnzT: *const i32, rowadrT: *const i32, colindT: *const i32, rowsuperT: *const i32, d: *mut mjData, diagind: *mut i32) {
    todo!() // cxx:_mju_sqrMatTDSparse_row
}

/// C: mju_block (engine/engine_util_sparse.c:1406)
/// Calls: cxx:_mju_gather
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_block(res: *mut f64, mat: *const f64, nc_mat: i32, nc_res: i32, nr: i32, perm_r: *const i32, perm_c: *const i32) {
    // SAFETY: caller guarantees valid pointers and bounds
    unsafe {
        for r in 0..nr as usize {
            let res_r: *mut f64 = res.add(r * nc_res as usize);
            let mat_r: *const f64 = mat.add(*perm_r.add(r) as usize * nc_mat as usize);
            crate::engine::engine_util_misc::mju_gather(res_r, mat_r, perm_c, nc_res);
        }
    }
}

/// C: mju_blockDiag (engine/engine_util_sparse.c:1430)
/// Calls: cxx:_mju_block
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_block_diag(res: *mut f64, mat: *const f64, nc_mat: i32, nc_res: i32, nb: i32, perm_r: *const i32, perm_c: *const i32, block_nr: *const i32, block_nc: *const i32, block_r: *const i32, block_c: *const i32) {
    todo!() // cxx:_mju_blockDiag
}

/// C: mju_blockSparse (engine/engine_util_sparse.c:1460)
/// Calls: cxx:_mju_copy
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_block_sparse(res: *mut f64, res_rownnz: *mut i32, res_rowadr: *mut i32, res_colind: *mut i32, mat: *const f64, rownnz: *const i32, rowadr: *const i32, colind: *const i32, nr: i32, perm_r: *const i32, perm_c: *const i32, col_offset: i32, res_offset: i32, res2: *mut f64, mat2: *const f64) {
    todo!() // cxx:_mju_blockSparse
}

/// C: mju_blockDiagSparse (engine/engine_util_sparse.c:1508)
/// Calls: cxx:_mju_blockSparse
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_block_diag_sparse(res: *mut f64, res_rownnz: *mut i32, res_rowadr: *mut i32, res_colind: *mut i32, mat: *const f64, rownnz: *const i32, rowadr: *const i32, colind: *const i32, nr: i32, nb: i32, perm_r: *const i32, perm_c: *const i32, block_r: *const i32, block_c: *const i32, res2: *mut f64, mat2: *const f64) {
    todo!() // cxx:_mju_blockDiagSparse
}

/// C: mju_dotSparse (engine/engine_util_sparse.h:197)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_dot_sparse(vec1: *const f64, vec2: *const f64, nnz1: i32, ind1: *const i32) -> f64 {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_sparse.h:_mju_dotSparse
}

/// C: mju_compare (engine/engine_util_sparse.h:231)
#[allow(unused_variables, non_snake_case)]
pub fn mju_compare(vec1: *const i32, vec2: *const i32, n: i32) -> i32 {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_sparse.h:_mju_compare
}

/// C: mj_mergeSorted (engine/engine_util_sparse.h:243)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_sparse.h:_mju_compare
#[allow(unused_variables, non_snake_case)]
pub fn mj_merge_sorted(merge: *mut i32, chain1: *const i32, n1: i32, chain2: *const i32, n2: i32) -> i32 {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_sparse.h:_mj_mergeSorted
}

/// C: mju_addToSclScl (engine/engine_util_sparse.h:297)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_add_to_scl_scl(res: *mut f64, vec: *const f64, scl1: f64, scl2: f64, n: i32) {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_sparse.h:_mju_addToSclScl
}

/// C: mju_combineSparse (engine/engine_util_sparse.h:311)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_sparse.h:_mju_addToSclScl, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_sparse.h:_mju_compare, cxx:_mju_combineSparseCount
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_combine_sparse(dst: *mut f64, src: *const f64, a: f64, b: f64, dst_nnz: i32, src_nnz: i32, dst_ind: *mut i32, src_ind: *const i32) -> i32 {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_sparse.h:_mju_combineSparse
}

