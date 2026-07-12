//! Port of: engine/engine_util_sparse.h
//! IR hash: c6d98e4f4b63b7f2
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
    // NOTE: signature changed from previous IR version
    // Previous params: (vec1 : * const f64, ind1 : * const i32, nnz1 : i32, vec2 : * const f64, ind2 : * const i32, nnz2 : i32)
    // Previous return: f64
    todo!("re-translate: params renamed")
}

/// C: mju_dotSparseX3 (engine/engine_util_sparse.h:36)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_dot_sparse_x3(res0: *mut f64, res1: *mut f64, res2: *mut f64, vec10: *const f64, vec11: *const f64, vec12: *const f64, vec2: *const f64, nnz1: i32, ind1: *const i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (res0 : * mut f64, res1 : * mut f64, res2 : * mut f64, vec10 : * const f64, vec11 : * const f64, vec12 : * const f64, vec2 : * const f64, nnz1 : i32, ind1 : * const i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mju_dense2sparse (engine/engine_util_sparse.h:42)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_dense2sparse(res: *mut f64, mat: *const f64, nr: i32, nc: i32, rownnz: *mut i32, rowadr: *mut i32, colind: *mut i32, nnz: i32) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (res : * mut f64, mat : * const f64, nr : i32, nc : i32, rownnz : * mut i32, rowadr : * mut i32, colind : * mut i32, nnz : i32)
    // Previous return: i32
    todo!("re-translate: params renamed")
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
    // NOTE: signature changed from previous IR version
    // Previous params: (res : * mut f64, mat : * const f64, nr : i32, nc : i32, rownnz : * const i32, rowadr : * const i32, colind : * const i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
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
    // NOTE: signature changed from previous IR version
    // Previous params: (res : * mut f64, mat : * const f64, n : i32, rownnz : * const i32, rowadr : * const i32, colind : * const i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
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
    // NOTE: signature changed from previous IR version
    // Previous params: (res : * mut f64, mat : * const f64, rownnz : * const i32, rowadr : * const i32, row : * const i32, nrow : i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
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
    // NOTE: signature changed from previous IR version
    // Previous params: (res : * mut f64, rownnz : * const i32, rowadr : * const i32, row : * const i32, nrow : i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
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
    // NOTE: signature changed from previous IR version
    // Previous params: (res : * mut f64, mat : * const f64, vec : * const f64, nr : i32, rownnz : * const i32, rowadr : * const i32, colind : * const i32, rowsuper : * const i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
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
    // NOTE: signature changed from previous IR version
    // Previous params: (res : * mut f64, mat : * const f64, vec : * const f64, nr : i32, nc : i32, rownnz : * const i32, rowadr : * const i32, colind : * const i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
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
    // NOTE: signature changed from previous IR version
    // Previous params: (dst : * mut f64, rownnz : * mut i32, rowadr : * mut i32, colind : * mut i32, nr : i32, M : * const f64, M_rownnz : * const i32, M_rowadr : * const i32, M_colind : * const i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mju_addToSymSparse (engine/engine_util_sparse.h:75)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_add_to_sym_sparse(res: *mut f64, mat: *const f64, n: i32, rownnz: *const i32, rowadr: *const i32, colind: *const i32, flg_upper: i32) {
    unsafe {
        // SAFETY: caller guarantees res points to n*n dense matrix,
        // mat/rownnz/rowadr/colind describe a valid sparse matrix with n rows
        for i in 0..n {
            let start: i32 = *rowadr.add(i as usize);
            let end: i32 = start + *rownnz.add(i as usize);

            for adr in start..end {
                let val: f64 = *mat.add(adr as usize);
                let j: i32 = *colind.add(adr as usize);

                // lower + diagonal
                *res.add((i as usize) * (n as usize) + (j as usize)) += val;

                // strict upper
                if flg_upper != 0 && j < i {
                    *res.add((j as usize) * (n as usize) + (i as usize)) += val;
                }
            }
        }
    }
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
    // NOTE: signature changed from previous IR version
    // Previous params: (res : * mut f64, mat : * const f64, vec : * const f64, n : i32, rownnz : * const i32, rowadr : * const i32, colind : * const i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mju_compressSparse (engine/engine_util_sparse.h:85)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_compress_sparse(mat: *mut f64, nr: i32, nc: i32, rownnz: *mut i32, rowadr: *mut i32, colind: *mut i32, minval: f64) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (mat : * mut f64, nr : i32, nc : i32, rownnz : * mut i32, rowadr : * mut i32, colind : * mut i32, minval : f64)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: mju_combineSparseCount (engine/engine_util_sparse.h:89)
#[allow(unused_variables, non_snake_case)]
pub fn mju_combine_sparse_count(a_nnz: i32, b_nnz: i32, a_ind: *const i32, b_ind: *const i32) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (a_nnz : i32, b_nnz : i32, a_ind : * const i32, b_ind : * const i32)
    // Previous return: i32
    todo!("re-translate: params renamed")
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
    // NOTE: signature changed from previous IR version
    // Previous params: (dst : * mut f64, src : * const f64, n : i32, a : f64, b : f64, dst_nnz : i32, src_nnz : i32, dst_ind : * const i32, src_ind : * const i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mju_addToSclSparseInc (engine/engine_util_sparse.h:96)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_add_to_scl_sparse_inc(dst: *mut f64, src: *const f64, nnzdst: i32, inddst: *const i32, nnzsrc: i32, indsrc: *const i32, scl: f64) {
    // NOTE: signature changed from previous IR version
    // Previous params: (dst : * mut f64, src : * const f64, nnzdst : i32, inddst : * const i32, nnzsrc : i32, indsrc : * const i32, scl : f64)
    // Previous return: ()
    todo!("re-translate: params renamed")
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
    // NOTE: signature changed from previous IR version
    // Previous params: (dst : * mut f64, src : * const f64, n : i32, nrow : i32, scl : f64, dst_nnz : i32, src_nnz : i32, dst_ind : * mut i32, src_ind : * const i32, buf : * mut f64, buf_ind : * mut i32)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: mju_addChains (engine/engine_util_sparse.h:106)
/// Calls: mju_compare, mju_copyInt
#[allow(unused_variables, non_snake_case)]
pub fn mju_add_chains(res: *mut i32, n: i32, NV1: i32, NV2: i32, chain1: *const i32, chain2: *const i32) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (res : * mut i32, n : i32, NV1 : i32, NV2 : i32, chain1 : * const i32, chain2 : * const i32)
    // Previous return: i32
    todo!("re-translate: params renamed")
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
    // NOTE: signature changed from previous IR version
    // Previous params: (res : * mut f64, mat : * const f64, nr : i32, nc : i32, res_rownnz : * mut i32, res_rowadr : * mut i32, res_colind : * mut i32, res_rowsuper : * mut i32, rownnz : * const i32, rowadr : * const i32, colind : * const i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mju_superSparse (engine/engine_util_sparse.h:115)
/// Calls: mju_compare
#[allow(unused_variables, non_snake_case)]
pub fn mju_super_sparse(nr: i32, rowsuper: *mut i32, rownnz: *const i32, rowadr: *const i32, colind: *const i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (nr : i32, rowsuper : * mut i32, rownnz : * const i32, rowadr : * const i32, colind : * const i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mju_sqrMatTDSparse (engine/engine_util_sparse.h:119)
/// Calls: mj_freeStack, mj_markStack, mj_stackAllocInfo, mju_dot, mju_zeroInt
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_sqr_mat_td_sparse(res: *mut f64, mat: *const f64, matT: *const f64, diag: *const f64, nr: i32, nc: i32, res_rownnz: *mut i32, res_rowadr: *const i32, res_colind: *mut i32, rownnz: *const i32, rowadr: *const i32, colind: *const i32, rowsuper: *const i32, rownnzT: *const i32, rowadrT: *const i32, colindT: *const i32, rowsuperT: *const i32, d: *mut mjData, diagind: *mut i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (res : * mut f64, mat : * const f64, matT : * const f64, diag : * const f64, nr : i32, nc : i32, res_rownnz : * mut i32, res_rowadr : * const i32, res_colind : * mut i32, rownnz : * const i32, rowadr : * const i32, colind : * const i32, rowsuper : * const i32, rownnzT : * const i32, rowadrT : * const i32, colindT : * const i32, rowsuperT : * const i32, d : * mut mjData, diagind : * mut i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mju_sqrMatTDSparse_row (engine/engine_util_sparse.h:129)
/// Calls: mj_freeStack, mj_markStack, mj_stackAllocInfo, mju_copyInt
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_sqr_mat_td_sparse_row(res: *mut f64, mat: *const f64, matT: *const f64, diag: *const f64, nr: i32, nc: i32, res_rownnz: *mut i32, res_rowadr: *const i32, res_colind: *mut i32, rownnz: *const i32, rowadr: *const i32, colind: *const i32, rowsuper: *const i32, rownnzT: *const i32, rowadrT: *const i32, colindT: *const i32, rowsuperT: *const i32, d: *mut mjData, diagind: *mut i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (res : * mut f64, mat : * const f64, matT : * const f64, diag : * const f64, nr : i32, nc : i32, res_rownnz : * mut i32, res_rowadr : * const i32, res_colind : * mut i32, rownnz : * const i32, rowadr : * const i32, colind : * const i32, rowsuper : * const i32, rownnzT : * const i32, rowadrT : * const i32, colindT : * const i32, rowsuperT : * const i32, d : * mut mjData, diagind : * mut i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mju_sqrMatTDSparseCount (engine/engine_util_sparse.h:139)
/// Calls: mj_freeStack, mj_markStack, mj_stackAllocInfo
#[allow(unused_variables, non_snake_case)]
pub fn mju_sqr_mat_td_sparse_count(res_rownnz: *mut i32, res_rowadr: *mut i32, nr: i32, rownnz: *const i32, rowadr: *const i32, colind: *const i32, rownnzT: *const i32, rowadrT: *const i32, colindT: *const i32, rowsuperT: *const i32, d: *mut mjData, flg_upper: i32) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (res_rownnz : * mut i32, res_rowadr : * mut i32, nr : i32, rownnz : * const i32, rowadr : * const i32, colind : * const i32, rownnzT : * const i32, rowadrT : * const i32, colindT : * const i32, rowsuperT : * const i32, d : * mut mjData, flg_upper : i32)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: mju_sqrMatTDSparseSymbolic (engine/engine_util_sparse.h:148)
/// Calls: mj_freeStack, mj_markStack, mj_stackAllocInfo, mju_copyInt, mju_zeroInt
#[allow(unused_variables, non_snake_case)]
pub fn mju_sqr_mat_td_sparse_symbolic(res_rownnz: *mut i32, res_rowadr: *mut i32, res_colind: *mut i32, res_diagind: *mut i32, nr: i32, nc: i32, rownnz: *const i32, rowadr: *const i32, colind: *const i32, rownnzT: *const i32, rowadrT: *const i32, colindT: *const i32, rowsuperT: *const i32, d: *mut mjData) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (res_rownnz : * mut i32, res_rowadr : * mut i32, res_colind : * mut i32, res_diagind : * mut i32, nr : i32, nc : i32, rownnz : * const i32, rowadr : * const i32, colind : * const i32, rownnzT : * const i32, rowadrT : * const i32, colindT : * const i32, rowsuperT : * const i32, d : * mut mjData)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: mju_sqrMatTDSparseNumeric (engine/engine_util_sparse.h:155)
/// Calls: mj_freeStack, mj_markStack, mj_stackAllocInfo, mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_sqr_mat_td_sparse_numeric(res: *mut f64, nc: i32, res_rownnz: *const i32, res_rowadr: *const i32, res_colind: *const i32, res_diagind: *const i32, mat: *const f64, rownnz: *const i32, rowadr: *const i32, colind: *const i32, matT: *const f64, rownnzT: *const i32, rowadrT: *const i32, colindT: *const i32, rowsuperT: *const i32, diag: *const f64, d: *mut mjData) {
    // NOTE: signature changed from previous IR version
    // Previous params: (res : * mut f64, nc : i32, res_rownnz : * const i32, res_rowadr : * const i32, res_colind : * const i32, res_diagind : * const i32, mat : * const f64, rownnz : * const i32, rowadr : * const i32, colind : * const i32, matT : * const f64, rownnzT : * const i32, rowadrT : * const i32, colindT : * const i32, rowsuperT : * const i32, diag : * const f64, d : * mut mjData)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mju_sqrMatTDUncompressedInit (engine/engine_util_sparse.h:163)
#[allow(unused_variables, non_snake_case)]
pub fn mju_sqr_mat_td_uncompressed_init(res_rowadr: *mut i32, nc: i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (res_rowadr : * mut i32, nc : i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
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
    // NOTE: signature changed from previous IR version
    // Previous params: (res : * mut f64, mat : * const f64, nc_mat : i32, nc_res : i32, nr : i32, perm_r : * const i32, perm_c : * const i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
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
    // NOTE: signature changed from previous IR version
    // Previous params: (res : * mut f64, mat : * const f64, nc_mat : i32, nc_res : i32, nb : i32, perm_r : * const i32, perm_c : * const i32, block_nr : * const i32, block_nc : * const i32, block_r : * const i32, block_c : * const i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
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
    // NOTE: signature changed from previous IR version
    // Previous params: (res : * mut f64, res_rownnz : * mut i32, res_rowadr : * mut i32, res_colind : * mut i32, mat : * const f64, rownnz : * const i32, rowadr : * const i32, colind : * const i32, nr : i32, perm_r : * const i32, perm_c : * const i32, col_offset : i32, res_offset : i32, res2 : * mut f64, mat2 : * const f64)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mju_blockDiagSparse (engine/engine_util_sparse.h:185)
/// Calls: mju_blockSparse
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_block_diag_sparse(res: *mut f64, res_rownnz: *mut i32, res_rowadr: *mut i32, res_colind: *mut i32, mat: *const f64, rownnz: *const i32, rowadr: *const i32, colind: *const i32, nr: i32, nb: i32, perm_r: *const i32, perm_c: *const i32, block_r: *const i32, block_c: *const i32, res2: *mut f64, mat2: *const f64) {
    // NOTE: signature changed from previous IR version
    // Previous params: (res : * mut f64, res_rownnz : * mut i32, res_rowadr : * mut i32, res_colind : * mut i32, mat : * const f64, rownnz : * const i32, rowadr : * const i32, colind : * const i32, nr : i32, nb : i32, perm_r : * const i32, perm_c : * const i32, block_r : * const i32, block_c : * const i32, res2 : * mut f64, mat2 : * const f64)
    // Previous return: ()
    todo!("re-translate: params renamed")
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
        // SAFETY: caller guarantees vec1 has at least nnz1 elements,
        // ind1 has at least nnz1 elements, vec2 is indexed by ind1 values
        let mut i: i32 = 0;
        let n_4: i32 = nnz1 - 4;
        let mut res0: f64 = 0.0;
        let mut res1: f64 = 0.0;
        let mut res2: f64 = 0.0;
        let mut res3: f64 = 0.0;

        while i <= n_4 {
            res0 += *vec1.add(i as usize) * *vec2.add(*ind1.add(i as usize) as usize);
            res1 += *vec1.add((i + 1) as usize) * *vec2.add(*ind1.add((i + 1) as usize) as usize);
            res2 += *vec1.add((i + 2) as usize) * *vec2.add(*ind1.add((i + 2) as usize) as usize);
            res3 += *vec1.add((i + 3) as usize) * *vec2.add(*ind1.add((i + 3) as usize) as usize);
            i += 4;
        }

        // CRITICAL: preserve exact reduction order (res0+res2)+(res1+res3)
        let mut res: f64 = (res0 + res2) + (res1 + res3);

        while i < nnz1 {
            res += *vec1.add(i as usize) * *vec2.add(*ind1.add(i as usize) as usize);
            i += 1;
        }

        res
    }
}

/// C: mju_compare (engine/engine_util_sparse.h:231)
#[allow(unused_variables, non_snake_case)]
pub fn mju_compare(vec1: *const i32, vec2: *const i32, n: i32) -> i32 {
    unsafe {
        // SAFETY: caller guarantees vec1 and vec2 point to at least n i32 elements
        let s1 = std::slice::from_raw_parts(vec1, n as usize);
        let s2 = std::slice::from_raw_parts(vec2, n as usize);
        if s1 == s2 { 1 } else { 0 }
    }
}

/// C: mj_mergeSorted (engine/engine_util_sparse.h:243)
/// Calls: mju_compare
#[allow(unused_variables, non_snake_case)]
pub fn mj_merge_sorted(merge: *mut i32, chain1: *const i32, n1: i32, chain2: *const i32, n2: i32) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (merge : * mut i32, chain1 : * const i32, n1 : i32, chain2 : * const i32, n2 : i32)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: mju_addToSclScl (engine/engine_util_sparse.h:297)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_add_to_scl_scl(res: *mut f64, vec: *const f64, scl1: f64, scl2: f64, n: i32) {
    unsafe {
        // SAFETY: caller guarantees res and vec point to at least n f64 elements
        for i in 0..n {
            *res.add(i as usize) = *res.add(i as usize) * scl1 + *vec.add(i as usize) * scl2;
        }
    }
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
    // NOTE: signature changed from previous IR version
    // Previous params: (dst : * mut f64, src : * const f64, a : f64, b : f64, dst_nnz : i32, src_nnz : i32, dst_ind : * mut i32, src_ind : * const i32)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

