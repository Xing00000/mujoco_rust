//! Port of: engine/engine_util_sparse.h
//! IR hash: 05737965add36adb
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
    // WARNING: signature changed — verify body
    // Previous params: (vec1 : * const f64, ind1 : * const i32, nnz1 : i32, vec2 : * const f64, ind2 : * const i32, nnz2 : i32)
    // Previous return: f64
    unsafe { let mut i1 : i32 = 0 ; let mut i2 : i32 = 0 ; let mut res : f64 = 0.0 ; if nnz1 == 0 || nnz2 == 0 { return 0.0 ; } while i1 < nnz1 && i2 < nnz2 { let adr1 : i32 = * ind1 . add (i1 as usize) ; let adr2 : i32 = * ind2 . add (i2 as usize) ; if adr1 == adr2 { res += * vec1 . add (i1 as usize) * * vec2 . add (i2 as usize) ; i1 += 1 ; i2 += 1 ; } else if adr1 < adr2 { i1 += 1 ; } else { i2 += 1 ; } } res }
}

/// C: mju_dotSparseX3 (engine/engine_util_sparse.h:36)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_dot_sparse_x3(res0: *mut f64, res1: *mut f64, res2: *mut f64, vec10: *const f64, vec11: *const f64, vec12: *const f64, vec2: *const f64, nnz1: i32, ind1: *const i32) {
    // WARNING: signature changed — verify body
    // Previous params: (res0 : * mut f64, res1 : * mut f64, res2 : * mut f64, vec10 : * const f64, vec11 : * const f64, vec12 : * const f64, vec2 : * const f64, nnz1 : i32, ind1 : * const i32)
    // Previous return: ()
    unsafe { let mut i : i32 = 0 ; let mut RES0 : f64 = 0.0 ; let mut RES1 : f64 = 0.0 ; let mut RES2 : f64 = 0.0 ; while i < nnz1 { let v2 : f64 = * vec2 . add (* ind1 . add (i as usize) as usize) ; RES0 += * vec10 . add (i as usize) * v2 ; RES1 += * vec11 . add (i as usize) * v2 ; RES2 += * vec12 . add (i as usize) * v2 ; i += 1 ; } * res0 = RES0 ; * res1 = RES1 ; * res2 = RES2 ; }
}

/// C: mju_dense2sparse (engine/engine_util_sparse.h:42)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_dense2sparse(res: *mut f64, mat: *const f64, nr: i32, nc: i32, rownnz: *mut i32, rowadr: *mut i32, colind: *mut i32, nnz: i32) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut f64, mat : * const f64, nr : i32, nc : i32, rownnz : * mut i32, rowadr : * mut i32, colind : * mut i32, nnz : i32)
    // Previous return: i32
    unsafe { if nnz <= 0 { return 1 ; } let mut adr : i32 = 0 ; for r in 0 .. nr { * rownnz . add (r as usize) = 0 ; * rowadr . add (r as usize) = adr ; for c in 0 .. nc { if * mat . add ((r * nc + c) as usize) != 0.0 { if adr >= nnz { return 1 ; } * colind . add (adr as usize) = c ; * rownnz . add (r as usize) += 1 ; * res . add (adr as usize) = * mat . add ((r * nc + c) as usize) ; adr += 1 ; } } } 0 }
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
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut f64, mat : * const f64, nr : i32, nc : i32, rownnz : * const i32, rowadr : * const i32, colind : * const i32)
    // Previous return: ()
    unsafe { crate :: engine :: engine_util_blas :: mju_zero (res , nr * nc) ; for r in 0 .. nr { for i in 0 .. * rownnz . add (r as usize) { let row_adr : i32 = * rowadr . add (r as usize) ; let col : i32 = * colind . add ((row_adr + i) as usize) ; * res . add ((r * nc + col) as usize) = * mat . add ((row_adr + i) as usize) ; } } }
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
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut f64, mat : * const f64, n : i32, rownnz : * const i32, rowadr : * const i32, colind : * const i32)
    // Previous return: ()
    unsafe { crate :: engine :: engine_util_blas :: mju_zero (res , n * n) ; for i in 0 .. n { let adr : i32 = * rowadr . add (i as usize) ; for j in 0 .. * rownnz . add (i as usize) { let col : i32 = * colind . add ((adr + j) as usize) ; if col <= i { let val : f64 = * mat . add ((adr + j) as usize) ; * res . add ((i * n + col) as usize) = val ; * res . add ((col * n + i) as usize) = val ; } } } }
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
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut f64, mat : * const f64, rownnz : * const i32, rowadr : * const i32, row : * const i32, nrow : i32)
    // Previous return: ()
    unsafe { for i in 0 .. nrow { let r : i32 = * row . add (i as usize) ; let adr : i32 = * rowadr . add (r as usize) ; let nnz : i32 = * rownnz . add (r as usize) ; crate :: engine :: engine_util_blas :: mju_copy (res . add (adr as usize) , mat . add (adr as usize) , nnz) ; } }
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
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut f64, rownnz : * const i32, rowadr : * const i32, row : * const i32, nrow : i32)
    // Previous return: ()
    unsafe { for i in 0 .. nrow { let r : i32 = * row . add (i as usize) ; let adr : i32 = * rowadr . add (r as usize) ; let nnz : i32 = * rownnz . add (r as usize) ; crate :: engine :: engine_util_blas :: mju_zero (res . add (adr as usize) , nnz) ; } }
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
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut f64, mat : * const f64, vec : * const f64, nr : i32, rownnz : * const i32, rowadr : * const i32, colind : * const i32, rowsuper : * const i32)
    // Previous return: ()
    unsafe { for r in 0 .. nr { let adr : i32 = * rowadr . add (r as usize) ; let nnz : i32 = * rownnz . add (r as usize) ; * res . add (r as usize) = mju_dot_sparse (mat . add (adr as usize) , vec , nnz , colind . add (adr as usize)) ; } }
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
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut f64, mat : * const f64, vec : * const f64, nr : i32, nc : i32, rownnz : * const i32, rowadr : * const i32, colind : * const i32)
    // Previous return: ()
    unsafe { crate :: engine :: engine_util_blas :: mju_zero (res , nc) ; for i in 0 .. nr { if * vec . add (i as usize) != 0.0 { let nnz : i32 = * rownnz . add (i as usize) ; let adr : i32 = * rowadr . add (i as usize) ; let vec_i : f64 = * vec . add (i as usize) ; for j in 0 .. nnz { let col : i32 = * colind . add ((adr + j) as usize) ; * res . add (col as usize) += * mat . add ((adr + j) as usize) * vec_i ; } } } }
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
    // WARNING: signature changed — verify body
    // Previous params: (dst : * mut f64, rownnz : * mut i32, rowadr : * mut i32, colind : * mut i32, nr : i32, M : * const f64, M_rownnz : * const i32, M_rowadr : * const i32, M_colind : * const i32)
    // Previous return: ()
    unsafe { for i in 0 .. nr { let adr = * rowadr . add (i as usize) ; let m_adr = * M_rowadr . add (i as usize) ; * rownnz . add (i as usize) = mju_combine_sparse (dst . add (adr as usize) , M . add (m_adr as usize) , 1.0 , 1.0 , * rownnz . add (i as usize) , * M_rownnz . add (i as usize) , colind . add (adr as usize) , M_colind . add (m_adr as usize) ,) ; } }
}

/// C: mju_addToSymSparse (engine/engine_util_sparse.h:75)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_add_to_sym_sparse(res: *mut f64, mat: *const f64, n: i32, rownnz: *const i32, rowadr: *const i32, colind: *const i32, flg_upper: i32) {
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut f64, mat : * const f64, n : i32, rownnz : * const i32, rowadr : * const i32, colind : * const i32, flg_upper : i32)
    // Previous return: ()
    unsafe { for i in 0 .. n { let start : i32 = * rowadr . add (i as usize) ; let end : i32 = start + * rownnz . add (i as usize) ; let mut adr = start ; while adr < end { let val : f64 = * mat . add (adr as usize) ; let j : i32 = * colind . add (adr as usize) ; * res . add ((i * n + j) as usize) += val ; if flg_upper != 0 && j < i { * res . add ((j * n + i) as usize) += val ; } adr += 1 ; } } }
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
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut f64, mat : * const f64, vec : * const f64, n : i32, rownnz : * const i32, rowadr : * const i32, colind : * const i32)
    // Previous return: ()
    unsafe { crate :: engine :: engine_util_blas :: mju_zero (res , n) ; for i in 0 .. n { let adr : i32 = * rowadr . add (i as usize) ; let diag : i32 = * rownnz . add (i as usize) - 1 ; let row : * const f64 = mat . add (adr as usize) ; * res . add (i as usize) = * row . add (diag as usize) * * vec . add (i as usize) ; let ind : * const i32 = colind . add (adr as usize) ; let mut k = diag - 1 ; while k >= 0 { let j : i32 = * ind . add (k as usize) ; let val : f64 = * row . add (k as usize) ; * res . add (i as usize) += val * * vec . add (j as usize) ; * res . add (j as usize) += val * * vec . add (i as usize) ; k -= 1 ; } } }
}

/// C: mju_compressSparse (engine/engine_util_sparse.h:85)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_compress_sparse(mat: *mut f64, nr: i32, nc: i32, rownnz: *mut i32, rowadr: *mut i32, colind: *mut i32, minval: f64) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (mat : * mut f64, nr : i32, nc : i32, rownnz : * mut i32, rowadr : * mut i32, colind : * mut i32, minval : f64)
    // Previous return: i32
    unsafe { let remove_small : i32 = if minval >= 0.0 { 1 } else { 0 } ; let mut adr : i32 = 0 ; for r in 0 .. nr { let rowadr_old : i32 = * rowadr . add (r as usize) ; * rowadr . add (r as usize) = adr ; let mut nnz : i32 = 0 ; let end : i32 = rowadr_old + * rownnz . add (r as usize) ; let mut adr_old = rowadr_old ; while adr_old < end { if remove_small != 0 && (* mat . add (adr_old as usize)) . abs () <= minval { adr_old += 1 ; continue ; } if adr != adr_old { * mat . add (adr as usize) = * mat . add (adr_old as usize) ; * colind . add (adr as usize) = * colind . add (adr_old as usize) ; } adr += 1 ; if remove_small != 0 { nnz += 1 ; } adr_old += 1 ; } if remove_small != 0 { * rownnz . add (r as usize) = nnz ; } } * rowadr . add ((nr - 1) as usize) + * rownnz . add ((nr - 1) as usize) }
}

/// C: mju_combineSparseCount (engine/engine_util_sparse.h:89)
#[allow(unused_variables, non_snake_case)]
pub fn mju_combine_sparse_count(a_nnz: i32, b_nnz: i32, a_ind: *const i32, b_ind: *const i32) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (a_nnz : i32, b_nnz : i32, a_ind : * const i32, b_ind : * const i32)
    // Previous return: i32
    unsafe { let mut a : i32 = 0 ; let mut b : i32 = 0 ; let mut c_nnz : i32 = 0 ; while a < a_nnz && b < b_nnz { if * a_ind . add (a as usize) == * b_ind . add (b as usize) { c_nnz += 1 ; a += 1 ; b += 1 ; } else if * a_ind . add (a as usize) < * b_ind . add (b as usize) { a += 1 ; } else { b += 1 ; } } a_nnz + b_nnz - c_nnz }
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
    // SAFETY: caller guarantees valid sparse arrays with correct nnz counts and index arrays
    unsafe {
        // check for identical pattern
        if dst_nnz == src_nnz {
            if crate::engine::engine_util_sparse::mju_compare(dst_ind, src_ind, dst_nnz) != 0 {
                // combine mjtNum data directly
                crate::engine::engine_util_sparse::mju_add_to_scl_scl(dst, src, a, b, dst_nnz);
                return;
            }
        }

        // scale dst by a
        if a != 1.0 {
            crate::engine::engine_util_blas::mju_scl(dst, dst, a, dst_nnz);
        }

        // prepare to merge
        let mut di: i32 = 0;
        let mut si: i32 = 0;
        let mut dadr: i32 = if di < dst_nnz { *dst_ind.add(di as usize) } else { n + 1 };
        let mut sadr: i32 = if si < src_nnz { *src_ind.add(si as usize) } else { n + 1 };

        // add src*b at common indices
        while di < dst_nnz {
            // both
            if dadr == sadr {
                *dst.add(di as usize) += b * *src.add(si as usize);
                di += 1;
                si += 1;
                dadr = if di < dst_nnz { *dst_ind.add(di as usize) } else { n + 1 };
                sadr = if si < src_nnz { *src_ind.add(si as usize) } else { n + 1 };
            }
            // dst only
            else if dadr < sadr {
                di += 1;
                dadr = if di < dst_nnz { *dst_ind.add(di as usize) } else { n + 1 };
            }
            // src only
            else {
                si += 1;
                sadr = if si < src_nnz { *src_ind.add(si as usize) } else { n + 1 };
            }
        }
    }
}

/// C: mju_addToSclSparseInc (engine/engine_util_sparse.h:96)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_add_to_scl_sparse_inc(dst: *mut f64, src: *const f64, nnzdst: i32, inddst: *const i32, nnzsrc: i32, indsrc: *const i32, scl: f64) {
    // WARNING: signature changed — verify body
    // Previous params: (dst : * mut f64, src : * const f64, nnzdst : i32, inddst : * const i32, nnzsrc : i32, indsrc : * const i32, scl : f64)
    // Previous return: ()
    unsafe { if nnzdst == 0 || nnzsrc == 0 { return ; } let mut adrs : i32 = 0 ; let mut adrd : i32 = 0 ; while adrs < nnzsrc && adrd < nnzdst { let inds : i32 = * indsrc . add (adrs as usize) ; let indd : i32 = * inddst . add (adrd as usize) ; if inds == indd { * dst . add (adrd as usize) += scl * * src . add (adrs as usize) ; adrs += 1 ; adrd += 1 ; } else if inds < indd { adrs += 1 ; } else { adrd += 1 ; } } }
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
    // SAFETY: all pointers valid. dst/src are column-major sparse matrices.
    // buf/buf_ind are scratch buffers with adequate capacity.
    unsafe {
        // check for identical pattern
        if dst_nnz == src_nnz {
            if dst_nnz == 0 {
                return 0;
            }
            if mju_compare(dst_ind, src_ind, dst_nnz) != 0 {
                // combine mjtNum data directly
                crate::engine::engine_util_blas::mju_add_to_scl(dst, src, scl, nrow * dst_nnz);
                return dst_nnz;
            }
        }

        // prepare to merge src and dst into buf^T
        let mut si: i32 = 0;
        let mut di: i32 = 0;
        let mut nnz: i32 = 0;
        let mut sadr: i32 = if src_nnz != 0 { *src_ind.add(0) } else { n + 1 };
        let mut dadr: i32 = if dst_nnz != 0 { *dst_ind.add(0) } else { n + 1 };

        // merge matrices
        while si < src_nnz || di < dst_nnz {
            // both
            if sadr == dadr {
                let mut k: i32 = 0;
                while k < nrow {
                    *buf.add((nrow * nnz + k) as usize) =
                        *dst.add((di + k * dst_nnz) as usize) + scl * *src.add((si + k * src_nnz) as usize);
                    k += 1;
                }
                *buf_ind.add(nnz as usize) = sadr;
                nnz += 1;
                si += 1;
                di += 1;
                sadr = if si < src_nnz { *src_ind.add(si as usize) } else { n + 1 };
                dadr = if di < dst_nnz { *dst_ind.add(di as usize) } else { n + 1 };
            }
            // dst only
            else if dadr < sadr {
                let mut k: i32 = 0;
                while k < nrow {
                    *buf.add((nrow * nnz + k) as usize) = *dst.add((di + k * dst_nnz) as usize);
                    k += 1;
                }
                *buf_ind.add(nnz as usize) = dadr;
                nnz += 1;
                di += 1;
                dadr = if di < dst_nnz { *dst_ind.add(di as usize) } else { n + 1 };
            }
            // src only
            else {
                let mut k: i32 = 0;
                while k < nrow {
                    *buf.add((nrow * nnz + k) as usize) = scl * *src.add((si + k * src_nnz) as usize);
                    k += 1;
                }
                *buf_ind.add(nnz as usize) = sadr;
                nnz += 1;
                si += 1;
                sadr = if si < src_nnz { *src_ind.add(si as usize) } else { n + 1 };
            }
        }

        // copy transposed buf into dst
        crate::engine::engine_util_blas::mju_transpose(dst, buf, nnz, nrow);
        crate::engine::engine_util_misc::mju_copy_int(dst_ind, buf_ind, nnz);

        nnz
    }
}

/// C: mju_addChains (engine/engine_util_sparse.h:106)
/// Calls: mju_compare, mju_copyInt
#[allow(unused_variables, non_snake_case)]
pub fn mju_add_chains(res: *mut i32, n: i32, NV1: i32, NV2: i32, chain1: *const i32, chain2: *const i32) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut i32, n : i32, NV1 : i32, NV2 : i32, chain1 : * const i32, chain2 : * const i32)
    // Previous return: i32
    unsafe { if NV1 == NV2 { if NV1 == 0 { return 0 ; } if mju_compare (chain1 , chain2 , NV1) != 0 { crate :: engine :: engine_util_misc :: mju_copy_int (res , chain1 , NV1) ; return NV1 ; } } let mut i1 : i32 = 0 ; let mut i2 : i32 = 0 ; let mut NV : i32 = 0 ; let mut adr1 : i32 = if NV1 != 0 { * chain1 . add (0) } else { n + 1 } ; let mut adr2 : i32 = if NV2 != 0 { * chain2 . add (0) } else { n + 1 } ; while i1 < NV1 || i2 < NV2 { if adr1 == adr2 { * res . add (NV as usize) = adr1 ; NV += 1 ; i1 += 1 ; i2 += 1 ; adr1 = if i1 < NV1 { * chain1 . add (i1 as usize) } else { n + 1 } ; adr2 = if i2 < NV2 { * chain2 . add (i2 as usize) } else { n + 1 } ; } else if adr1 < adr2 { * res . add (NV as usize) = adr1 ; NV += 1 ; i1 += 1 ; adr1 = if i1 < NV1 { * chain1 . add (i1 as usize) } else { n + 1 } ; } else { * res . add (NV as usize) = adr2 ; NV += 1 ; i2 += 1 ; adr2 = if i2 < NV2 { * chain2 . add (i2 as usize) } else { n + 1 } ; } } NV }
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
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut f64, mat : * const f64, nr : i32, nc : i32, res_rownnz : * mut i32, res_rowadr : * mut i32, res_colind : * mut i32, res_rowsuper : * mut i32, rownnz : * const i32, rowadr : * const i32, colind : * const i32)
    // Previous return: ()
    unsafe { if nr == 0 || nc == 0 { return ; } crate :: engine :: engine_util_misc :: mju_zero_int (res_rownnz , nc) ; let row_offset = * rowadr . add (0) ; for r in 0 .. nr { let start = * rowadr . add (r as usize) - row_offset ; let end = start + * rownnz . add (r as usize) ; for j in start .. end { let col = * colind . add (j as usize) ; * res_rownnz . add (col as usize) += 1 ; } } if ! res_rowsuper . is_null () { for i in 0 .. (nc - 1) { * res_rowsuper . add (i as usize) = (* res_rownnz . add (i as usize) == * res_rownnz . add ((i + 1) as usize)) as i32 ; } * res_rowsuper . add ((nc - 1) as usize) = 0 ; } * res_rowadr . add (0) = 0 ; for i in 1 .. nc { * res_rowadr . add (i as usize) = * res_rowadr . add ((i - 1) as usize) + * res_rownnz . add ((i - 1) as usize) ; } for r in 0 .. nr { let mut c_prev : i32 = - 1 ; let start = * rowadr . add (r as usize) - row_offset ; let end = start + * rownnz . add (r as usize) ; for i in start .. end { let c = * colind . add (i as usize) ; let adr = * res_rowadr . add (c as usize) ; * res_rowadr . add (c as usize) += 1 ; * res_colind . add (adr as usize) = r ; if ! res . is_null () { * res . add (adr as usize) = * mat . add (i as usize) ; } if ! res_rowsuper . is_null () { if c > 0 && c != c_prev + 1 && * res_rowsuper . add ((c - 1) as usize) != 0 { * res_rowsuper . add ((c - 1) as usize) = 0 ; } c_prev = c ; } } } for i in (1 .. nc) . rev () { * res_rowadr . add (i as usize) = * res_rowadr . add ((i - 1) as usize) ; } * res_rowadr . add (0) = 0 ; if ! res_rowsuper . is_null () { for i in (0 .. (nc - 1)) . rev () { if * res_rowsuper . add (i as usize) != 0 { * res_rowsuper . add (i as usize) += * res_rowsuper . add ((i + 1) as usize) ; } } } }
}

/// C: mju_superSparse (engine/engine_util_sparse.h:115)
/// Calls: mju_compare
#[allow(unused_variables, non_snake_case)]
pub fn mju_super_sparse(nr: i32, rowsuper: *mut i32, rownnz: *const i32, rowadr: *const i32, colind: *const i32) {
    // WARNING: signature changed — verify body
    // Previous params: (nr : i32, rowsuper : * mut i32, rownnz : * const i32, rowadr : * const i32, colind : * const i32)
    // Previous return: ()
    unsafe { if nr == 0 { return ; } for r in 0 .. (nr - 1) { if * rownnz . add (r as usize) != * rownnz . add ((r + 1) as usize) { * rowsuper . add (r as usize) = 0 ; } else { * rowsuper . add (r as usize) = mju_compare (colind . add (* rowadr . add (r as usize) as usize) , colind . add (* rowadr . add ((r + 1) as usize) as usize) , * rownnz . add (r as usize) ,) ; } } * rowsuper . add ((nr - 1) as usize) = 0 ; let mut r = nr - 2 ; while r >= 0 { if * rowsuper . add (r as usize) != 0 { * rowsuper . add (r as usize) += * rowsuper . add ((r + 1) as usize) ; } r -= 1 ; } }
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
    extern "C" { fn mju_sqrMatTDSparse(res: *mut f64, mat: *const f64, matT: *const f64, diag: *const f64, nr: i32, nc: i32, res_rownnz: *mut i32, res_rowadr: *const i32, res_colind: *mut i32, rownnz: *const i32, rowadr: *const i32, colind: *const i32, rowsuper: *const i32, rownnzT: *const i32, rowadrT: *const i32, colindT: *const i32, rowsuperT: *const i32, d: *mut mjData, diagind: *mut i32); }
    // SAFETY: delegates to C implementation
    unsafe { mju_sqrMatTDSparse(res, mat, matT, diag, nr, nc, res_rownnz, res_rowadr, res_colind, rownnz, rowadr, colind, rowsuper, rownnzT, rowadrT, colindT, rowsuperT, d, diagind) }
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
    extern "C" { fn mju_sqrMatTDSparse_row(res: *mut f64, mat: *const f64, matT: *const f64, diag: *const f64, nr: i32, nc: i32, res_rownnz: *mut i32, res_rowadr: *const i32, res_colind: *mut i32, rownnz: *const i32, rowadr: *const i32, colind: *const i32, rowsuper: *const i32, rownnzT: *const i32, rowadrT: *const i32, colindT: *const i32, rowsuperT: *const i32, d: *mut mjData, diagind: *mut i32); }
    // SAFETY: delegates to C implementation
    unsafe { mju_sqrMatTDSparse_row(res, mat, matT, diag, nr, nc, res_rownnz, res_rowadr, res_colind, rownnz, rowadr, colind, rowsuper, rownnzT, rowadrT, colindT, rowsuperT, d, diagind) }
}

/// C: mju_sqrMatTDSparseCount (engine/engine_util_sparse.h:139)
/// Calls: mj_freeStack, mj_markStack, mj_stackAllocInfo
#[allow(unused_variables, non_snake_case)]
pub fn mju_sqr_mat_td_sparse_count(res_rownnz: *mut i32, res_rowadr: *mut i32, nr: i32, rownnz: *const i32, rowadr: *const i32, colind: *const i32, rownnzT: *const i32, rowadrT: *const i32, colindT: *const i32, rowsuperT: *const i32, d: *mut mjData, flg_upper: i32) -> i32 {
    extern "C" { fn mju_sqrMatTDSparseCount(res_rownnz: *mut i32, res_rowadr: *mut i32, nr: i32, rownnz: *const i32, rowadr: *const i32, colind: *const i32, rownnzT: *const i32, rowadrT: *const i32, colindT: *const i32, rowsuperT: *const i32, d: *mut mjData, flg_upper: i32) -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { mju_sqrMatTDSparseCount(res_rownnz, res_rowadr, nr, rownnz, rowadr, colind, rownnzT, rowadrT, colindT, rowsuperT, d, flg_upper) }
}

/// C: mju_sqrMatTDSparseSymbolic (engine/engine_util_sparse.h:148)
/// Calls: mj_freeStack, mj_markStack, mj_stackAllocInfo, mju_copyInt, mju_zeroInt
#[allow(unused_variables, non_snake_case)]
pub fn mju_sqr_mat_td_sparse_symbolic(res_rownnz: *mut i32, res_rowadr: *mut i32, res_colind: *mut i32, res_diagind: *mut i32, nr: i32, nc: i32, rownnz: *const i32, rowadr: *const i32, colind: *const i32, rownnzT: *const i32, rowadrT: *const i32, colindT: *const i32, rowsuperT: *const i32, d: *mut mjData) -> i32 {
    extern "C" {
        fn mju_sqrMatTDSparseSymbolic(res_rownnz: *mut i32, res_rowadr: *mut i32, res_colind: *mut i32, res_diagind: *mut i32, nr: i32, nc: i32, rownnz: *const i32, rowadr: *const i32, colind: *const i32, rownnzT: *const i32, rowadrT: *const i32, colindT: *const i32, rowsuperT: *const i32, d: *mut mjData) -> i32;
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mju_sqrMatTDSparseSymbolic(res_rownnz, res_rowadr, res_colind, res_diagind, nr, nc, rownnz, rowadr, colind, rownnzT, rowadrT, colindT, rowsuperT, d) }
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
    extern "C" { fn mju_sqrMatTDSparseNumeric(res: *mut f64, nc: i32, res_rownnz: *const i32, res_rowadr: *const i32, res_colind: *const i32, res_diagind: *const i32, mat: *const f64, rownnz: *const i32, rowadr: *const i32, colind: *const i32, matT: *const f64, rownnzT: *const i32, rowadrT: *const i32, colindT: *const i32, rowsuperT: *const i32, diag: *const f64, d: *mut mjData); }
    // SAFETY: delegates to C implementation
    unsafe { mju_sqrMatTDSparseNumeric(res, nc, res_rownnz, res_rowadr, res_colind, res_diagind, mat, rownnz, rowadr, colind, matT, rownnzT, rowadrT, colindT, rowsuperT, diag, d) }
}

/// C: mju_sqrMatTDUncompressedInit (engine/engine_util_sparse.h:163)
#[allow(unused_variables, non_snake_case)]
pub fn mju_sqr_mat_td_uncompressed_init(res_rowadr: *mut i32, nc: i32) {
    // WARNING: signature changed — verify body
    // Previous params: (res_rowadr : * mut i32, nc : i32)
    // Previous return: ()
    unsafe { for r in 0 .. nc { * res_rowadr . add (r as usize) = r * nc ; } }
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
    // SAFETY: raw pointer arithmetic mirrors C exactly; caller guarantees valid bounds
    unsafe {
        for r in 0..nr {
            let res_r = res.add((r * nc_res) as usize);
            let mat_r = mat.add((*perm_r.add(r as usize) * nc_mat) as usize);
            crate::engine::engine_util_misc::mju_gather(res_r, mat_r, perm_c, nc_res);
        }
    }
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
    // SAFETY: caller guarantees all pointers are valid and arrays are properly sized
    unsafe {
        for b in 0..nb {
            let adr = nc_res * *block_r.add(b as usize);
            mju_block(
                res.add(adr as usize),
                mat,
                nc_mat,
                *block_nc.add(b as usize),
                *block_nr.add(b as usize),
                perm_r.add(*block_r.add(b as usize) as usize),
                perm_c.add(*block_c.add(b as usize) as usize),
            );
        }
    }
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
    // SAFETY: raw pointer arithmetic mirrors C exactly; caller guarantees valid bounds
    unsafe {
        for r in 0..nr {
            let k = *perm_r.add(r as usize);
            let nnz = *rownnz.add(k as usize);
            *res_rownnz.add(r as usize) = nnz;

            let res_adr = if r == 0 {
                res_offset
            } else {
                *res_rowadr.add((r - 1) as usize) + *res_rownnz.add((r - 1) as usize)
            };
            *res_rowadr.add(r as usize) = res_adr;

            let res_colind_r = res_colind.add((res_adr - res_offset) as usize);
            let mat_adr = *rowadr.add(k as usize);
            let colind_k = colind.add(mat_adr as usize);

            for j in 0..nnz {
                *res_colind_r.add(j as usize) =
                    *perm_c.add(*colind_k.add(j as usize) as usize) - col_offset;
            }

            crate::engine::engine_util_blas::mju_copy(
                res.add((res_adr - res_offset) as usize),
                mat.add(mat_adr as usize),
                nnz,
            );

            if !mat2.is_null() && !res2.is_null() {
                crate::engine::engine_util_blas::mju_copy(
                    res2.add((res_adr - res_offset) as usize),
                    mat2.add(mat_adr as usize),
                    nnz,
                );
            }
        }
    }
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
    // SAFETY: raw pointer arithmetic mirrors C exactly; caller guarantees valid bounds
    unsafe {
        for b in 0..nb {
            let nr_block: i32 = (if b + 1 < nb {
                *block_r.add((b + 1) as usize)
            } else {
                nr
            }) - *block_r.add(b as usize);

            let block_r_b = *block_r.add(b as usize);
            let res_adr: i32 = if block_r_b == 0 {
                0
            } else {
                *res_rowadr.add((block_r_b - 1) as usize) + *res_rownnz.add((block_r_b - 1) as usize)
            };

            let res2_arg: *mut f64 = if !res2.is_null() {
                res2.add(res_adr as usize)
            } else {
                std::ptr::null_mut()
            };

            mju_block_sparse(
                res.add(res_adr as usize),
                res_rownnz.add(block_r_b as usize),
                res_rowadr.add(block_r_b as usize),
                res_colind.add(res_adr as usize),
                mat,
                rownnz,
                rowadr,
                colind,
                nr_block,
                perm_r.add(block_r_b as usize),
                perm_c,
                *block_c.add(b as usize),
                res_adr,
                res2_arg,
                mat2,
            );
        }
    }
}

/// C: mju_dotSparse (engine/engine_util_sparse.h:197)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_dot_sparse(vec1: *const f64, vec2: *const f64, nnz1: i32, ind1: *const i32) -> f64  {
    extern "C" { fn mju_dotSparse(vec1: *const f64, vec2: *const f64, nnz1: i32, ind1: *const i32) -> f64; }
    // SAFETY: delegates to C implementation
    unsafe { mju_dotSparse(vec1, vec2, nnz1, ind1) }
}

/// C: mju_compare (engine/engine_util_sparse.h:231)
#[allow(unused_variables, non_snake_case)]
pub fn mju_compare(vec1: *const i32, vec2: *const i32, n: i32) -> i32  {
    // SAFETY: vec1 and vec2 point to arrays of at least n elements per caller contract.
    // Full translation of C: compares element-by-element, returns first difference sign.
    unsafe {
        for i in 0..n {
            let a = *vec1.add(i as usize);
            let b = *vec2.add(i as usize);
            if a < b {
                return -1;
            }
            if a > b {
                return 1;
            }
        }
        0
    }
}

/// C: mj_mergeSorted (engine/engine_util_sparse.h:243)
/// Calls: mju_compare
#[allow(unused_variables, non_snake_case)]
pub fn mj_merge_sorted(merge: *mut i32, chain1: *const i32, n1: i32, chain2: *const i32, n2: i32) -> i32  {
    extern "C" { fn mj_mergeSorted(merge: *mut i32, chain1: *const i32, n1: i32, chain2: *const i32, n2: i32) -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { mj_mergeSorted(merge, chain1, n1, chain2, n2) }
}

/// C: mju_addToSclScl (engine/engine_util_sparse.h:297)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_add_to_scl_scl(res: *mut f64, vec: *const f64, scl1: f64, scl2: f64, n: i32) {
    // SAFETY: res and vec point to arrays of at least n elements per caller contract.
    // Full translation: res[i] = scl1*res[i] + scl2*vec[i]
    unsafe {
        for i in 0..n {
            let idx = i as usize;
            *res.add(idx) = scl1 * *res.add(idx) + scl2 * *vec.add(idx);
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
pub fn mju_combine_sparse(dst: *mut f64, src: *const f64, a: f64, b: f64, dst_nnz: i32, src_nnz: i32, dst_ind: *mut i32, src_ind: *const i32) -> i32  {
    extern "C" { fn mju_combineSparse(dst: *mut f64, src: *const f64, a: f64, b: f64, dst_nnz: i32, src_nnz: i32, dst_ind: *mut i32, src_ind: *const i32) -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { mju_combineSparse(dst, src, a, b, dst_nnz, src_nnz, dst_ind, src_ind) }
}

