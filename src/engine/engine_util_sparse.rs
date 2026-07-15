//! Port of: engine/engine_util_sparse.h
//! IR hash: d2209344472ae336
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mju_dotSparse2 (engine/engine_util_sparse.h:32)
/// Calls: FilePath::empty
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_dot_sparse2(vec1: *const f64, ind1: *const i32, nnz1: i32, vec2: *const f64, ind2: *const i32, nnz2: i32) -> f64 {
    // SAFETY: vec1/ind1 have nnz1 elements, vec2/ind2 have nnz2 elements per caller contract
    unsafe {
        let mut i1: i32 = 0;
        let mut i2: i32 = 0;
        let mut res: f64 = 0.0;

        // check for empty array
        if nnz1 == 0 || nnz2 == 0 {
            return 0.0;
        }

        while i1 < nnz1 && i2 < nnz2 {
            let adr1 = *ind1.add(i1 as usize);
            let adr2 = *ind2.add(i2 as usize);

            if adr1 == adr2 {
                res += *vec1.add(i1 as usize) * *vec2.add(i2 as usize);
                i1 += 1;
                i2 += 1;
            } else if adr1 < adr2 {
                i1 += 1;
            } else {
                i2 += 1;
            }
        }

        res
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
    // SAFETY: all pointers valid for nnz1 elements per caller contract
    unsafe {
        let mut RES0: f64 = 0.0;
        let mut RES1: f64 = 0.0;
        let mut RES2: f64 = 0.0;

        for i in 0..nnz1 as usize {
            let v2 = *vec2.add(*ind1.add(i) as usize);

            RES0 += *vec10.add(i) * v2;
            RES1 += *vec11.add(i) * v2;
            RES2 += *vec12.add(i) * v2;
        }

        *res0 = RES0;
        *res1 = RES1;
        *res2 = RES2;
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

/// C: mju_sparse2dense (engine/engine_util_sparse.h:46)
/// Calls: mju_zero
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

/// C: mju_sym2dense (engine/engine_util_sparse.h:50)
/// Calls: mju_zero
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

/// C: mju_copySparse (engine/engine_util_sparse.h:54)
/// Calls: mju_copy
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_copy_sparse(res: *mut f64, mat: *const f64, rownnz: *const i32, rowadr: *const i32, row: *const i32, nrow: i32) {
    // SAFETY: caller guarantees all pointers valid
    unsafe {
        for i in 0..nrow as usize {
            let r = *row.add(i) as usize;
            crate::engine::engine_util_blas::mju_copy(
                res.add(*rowadr.add(r) as usize),
                mat.add(*rowadr.add(r) as usize),
                *rownnz.add(r),
            );
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
    // SAFETY: caller guarantees all pointers valid
    unsafe {
        for i in 0..nrow as usize {
            let r = *row.add(i) as usize;
            crate::engine::engine_util_blas::mju_zero(
                res.add(*rowadr.add(r) as usize),
                *rownnz.add(r),
            );
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
    // SAFETY: caller guarantees all pointers valid for their dimensions
    unsafe {
        for r in 0..nr as usize {
            *res.add(r) = mju_dot_sparse(
                mat.add(*rowadr.add(r) as usize),
                vec,
                *rownnz.add(r),
                colind.add(*rowadr.add(r) as usize),
            );
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
    // SAFETY: caller guarantees all pointers valid for their dimensions
    unsafe {
        // clear res
        crate::engine::engine_util_blas::mju_zero(res, nc);

        for i in 0..nr as usize {
            let scl = *vec.add(i);

            // skip if 0
            if scl == 0.0 {
                continue;
            }

            // add row scaled by the corresponding vector element
            let nnz = *rownnz.add(i) as usize;
            let adr = *rowadr.add(i) as usize;
            let ind = colind.add(adr);
            let row = mat.add(adr);
            for j in 0..nnz {
                *res.add(*ind.add(j) as usize) += *row.add(j) * scl;
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
    // SAFETY: caller guarantees all pointers valid for sparse matrix dimensions
    unsafe {
        for i in 0..nr as usize {
            *rownnz.add(i) = mju_combine_sparse(
                dst.add(*rowadr.add(i) as usize),
                M.add(*M_rowadr.add(i) as usize),
                1.0,
                1.0,
                *rownnz.add(i),
                *M_rownnz.add(i),
                colind.add(*rowadr.add(i) as usize),
                M_colind.add(*M_rowadr.add(i) as usize),
            );
        }
    }
}

/// C: mju_addToSymSparse (engine/engine_util_sparse.h:75)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_add_to_sym_sparse(res: *mut f64, mat: *const f64, n: i32, rownnz: *const i32, rowadr: *const i32, colind: *const i32, flg_upper: i32) {
    // SAFETY: caller guarantees all pointers valid for the sparse matrix dimensions
    unsafe {
        for i in 0..n as usize {
            let start = *rowadr.add(i);
            let end = start + *rownnz.add(i);
            let mut adr = start;
            while adr < end {
                let val = *mat.add(adr as usize);
                let j = *colind.add(adr as usize) as usize;

                // lower + diagonal
                *res.add(i * (n as usize) + j) += val;

                // strict upper
                if flg_upper != 0 && j < i {
                    *res.add(j * (n as usize) + i) += val;
                }

                adr += 1;
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
    // SAFETY: caller guarantees all pointers valid for their dimensions
    unsafe {
        // clear res
        crate::engine::engine_util_blas::mju_zero(res, n);

        // multiply
        for i in 0..n as usize {
            let adr = *rowadr.add(i) as usize;
            let diag = *rownnz.add(i) as usize - 1;
            let row = mat.add(adr);

            // diagonal
            *res.add(i) = *row.add(diag) * *vec.add(i);

            // off-diagonals
            let ind = colind.add(adr);
            let mut k = diag as i32 - 1;
            while k >= 0 {
                let j = *ind.add(k as usize) as usize;
                let val = *row.add(k as usize);
                *res.add(i) += val * *vec.add(j); // strict lower
                *res.add(j) += val * *vec.add(i); // strict upper
                k -= 1;
            }
        }
    }
}

/// C: mju_compressSparse (engine/engine_util_sparse.h:85)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_compress_sparse(mat: *mut f64, nr: i32, nc: i32, rownnz: *mut i32, rowadr: *mut i32, colind: *mut i32, minval: f64) -> i32 {
    // SAFETY: caller guarantees all pointers valid for sparse matrix dimensions
    unsafe {
        let remove_small = minval >= 0.0;
        let mut adr: i32 = 0;

        for r in 0..nr as usize {
            // save old rowadr, record new
            let rowadr_old = *rowadr.add(r);
            *rowadr.add(r) = adr;

            // shift mat and colind
            let mut nnz: i32 = 0;
            let end = rowadr_old + *rownnz.add(r);
            let mut adr_old = rowadr_old;
            while adr_old < end {
                if remove_small && f64::abs(*mat.add(adr_old as usize)) <= minval {
                    adr_old += 1;
                    continue;
                }
                if adr != adr_old {
                    *mat.add(adr as usize) = *mat.add(adr_old as usize);
                    *colind.add(adr as usize) = *colind.add(adr_old as usize);
                }
                adr += 1;
                if remove_small {
                    nnz += 1;
                }
                adr_old += 1;
            }
            if remove_small {
                *rownnz.add(r) = nnz;
            }
        }

        *rowadr.add((nr - 1) as usize) + *rownnz.add((nr - 1) as usize)
    }
}

/// C: mju_combineSparseCount (engine/engine_util_sparse.h:89)
/// Calls: GlobalTable::count
#[allow(unused_variables, non_snake_case)]
pub fn mju_combine_sparse_count(a_nnz: i32, b_nnz: i32, a_ind: *const i32, b_ind: *const i32) -> i32 {
    // SAFETY: a_ind[a_nnz] and b_ind[b_nnz] valid per caller contract
    unsafe {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        let mut c_nnz: i32 = 0;

        while a < a_nnz && b < b_nnz {
            if *a_ind.add(a as usize) == *b_ind.add(b as usize) {
                c_nnz += 1;
                a += 1;
                b += 1;
            } else if *a_ind.add(a as usize) < *b_ind.add(b as usize) {
                a += 1;
            } else {
                b += 1;
            }
        }

        // union minus the intersection
        a_nnz + b_nnz - c_nnz
    }
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
    // SAFETY: caller guarantees all pointers valid for their dimensions
    unsafe {
        // check for identical pattern
        if dst_nnz == src_nnz {
            if mju_compare(dst_ind, src_ind, dst_nnz) != 0 {
                mju_add_to_scl_scl(dst, src, a, b, dst_nnz);
                return;
            }
        }

        // scale dst by a
        if a != 1.0 {
            crate::engine::engine_util_blas::mju_scl(dst, dst as *const f64, a, dst_nnz);
        }

        // prepare to merge
        let mut di: i32 = 0;
        let mut si: i32 = 0;
        let mut dadr = if di < dst_nnz { *dst_ind.add(di as usize) } else { n + 1 };
        let mut sadr = if si < src_nnz { *src_ind.add(si as usize) } else { n + 1 };

        // add src*b at common indices
        while di < dst_nnz {
            if dadr == sadr {
                *dst.add(di as usize) += b * *src.add(si as usize);
                di += 1;
                si += 1;
                dadr = if di < dst_nnz { *dst_ind.add(di as usize) } else { n + 1 };
                sadr = if si < src_nnz { *src_ind.add(si as usize) } else { n + 1 };
            } else if dadr < sadr {
                di += 1;
                dadr = if di < dst_nnz { *dst_ind.add(di as usize) } else { n + 1 };
            } else {
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
    if nnzdst == 0 || nnzsrc == 0 {
        return;
    }
    // SAFETY: caller guarantees dst, src, inddst, indsrc point to valid arrays of respective lengths
    unsafe {
        let mut adrs: i32 = 0;
        let mut adrd: i32 = 0;
        while adrs < nnzsrc && adrd < nnzdst {
            let inds = *indsrc.add(adrs as usize);
            let indd = *inddst.add(adrd as usize);

            if inds == indd {
                *dst.add(adrd as usize) += scl * *src.add(adrs as usize);
                adrs += 1;
                adrd += 1;
            } else if inds < indd {
                adrs += 1;
            } else {
                adrd += 1;
            }
        }
    }
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
    // SAFETY: caller guarantees all pointers valid for their dimensions
    unsafe {
        // check for identical pattern
        if dst_nnz == src_nnz {
            if dst_nnz == 0 {
                return 0;
            }
            if mju_compare(dst_ind as *const i32, src_ind, dst_nnz) != 0 {
                crate::engine::engine_util_blas::mju_add_to_scl(dst, src, scl, nrow * dst_nnz);
                return dst_nnz;
            }
        }

        // prepare to merge src and dst into buf^T
        let mut si: i32 = 0;
        let mut di: i32 = 0;
        let mut nnz: i32 = 0;
        let mut sadr = if src_nnz > 0 { *src_ind.add(0) } else { n + 1 };
        let mut dadr = if dst_nnz > 0 { *dst_ind.add(0) } else { n + 1 };

        // merge matrices
        while si < src_nnz || di < dst_nnz {
            if sadr == dadr {
                for k in 0..nrow {
                    *buf.add((nrow * nnz + k) as usize) =
                        *dst.add((di + k * dst_nnz) as usize)
                        + scl * *src.add((si + k * src_nnz) as usize);
                }
                *buf_ind.add(nnz as usize) = sadr;
                nnz += 1;
                si += 1;
                di += 1;
                sadr = if si < src_nnz { *src_ind.add(si as usize) } else { n + 1 };
                dadr = if di < dst_nnz { *dst_ind.add(di as usize) } else { n + 1 };
            } else if dadr < sadr {
                for k in 0..nrow {
                    *buf.add((nrow * nnz + k) as usize) =
                        *dst.add((di + k * dst_nnz) as usize);
                }
                *buf_ind.add(nnz as usize) = dadr;
                nnz += 1;
                di += 1;
                dadr = if di < dst_nnz { *dst_ind.add(di as usize) } else { n + 1 };
            } else {
                for k in 0..nrow {
                    *buf.add((nrow * nnz + k) as usize) =
                        scl * *src.add((si + k * src_nnz) as usize);
                }
                *buf_ind.add(nnz as usize) = sadr;
                nnz += 1;
                si += 1;
                sadr = if si < src_nnz { *src_ind.add(si as usize) } else { n + 1 };
            }
        }

        // copy transposed buf into dst
        crate::engine::engine_util_blas::mju_transpose(dst, buf as *const f64, nnz, nrow);
        crate::engine::engine_util_misc::mju_copy_int(dst_ind, buf_ind as *const i32, nnz);

        nnz
    }
}

/// C: mju_addChains (engine/engine_util_sparse.h:106)
/// Calls: mju_compare, mju_copyInt
#[allow(unused_variables, non_snake_case)]
pub fn mju_add_chains(res: *mut i32, n: i32, NV1: i32, NV2: i32, chain1: *const i32, chain2: *const i32) -> i32 {
    // SAFETY: caller guarantees all pointers valid
    unsafe {
        // check for identical pattern
        if NV1 == NV2 {
            if NV1 == 0 {
                return 0;
            }
            if mju_compare(chain1, chain2, NV1) != 0 {
                crate::engine::engine_util_misc::mju_copy_int(res, chain1, NV1);
                return NV1;
            }
        }

        // prepare to merge
        let mut i1: i32 = 0;
        let mut i2: i32 = 0;
        let mut NV: i32 = 0;
        let mut adr1 = if NV1 > 0 { *chain1.add(0) } else { n + 1 };
        let mut adr2 = if NV2 > 0 { *chain2.add(0) } else { n + 1 };

        // merge chains
        while i1 < NV1 || i2 < NV2 {
            if adr1 == adr2 {
                *res.add(NV as usize) = adr1;
                NV += 1;
                i1 += 1;
                i2 += 1;
                adr1 = if i1 < NV1 { *chain1.add(i1 as usize) } else { n + 1 };
                adr2 = if i2 < NV2 { *chain2.add(i2 as usize) } else { n + 1 };
            } else if adr1 < adr2 {
                *res.add(NV as usize) = adr1;
                NV += 1;
                i1 += 1;
                adr1 = if i1 < NV1 { *chain1.add(i1 as usize) } else { n + 1 };
            } else {
                *res.add(NV as usize) = adr2;
                NV += 1;
                i2 += 1;
                adr2 = if i2 < NV2 { *chain2.add(i2 as usize) } else { n + 1 };
            }
        }

        NV
    }
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
    // SAFETY: caller guarantees all pointers valid for their dimensions
    unsafe {
        if nr == 0 || nc == 0 {
            return;
        }

        // clear number of non-zeros for each row of transposed
        crate::engine::engine_util_misc::mju_zero_int(res_rownnz, nc);

        // handle the case where the first row of mat is nonzero (offset wrt the base pointers)
        let row_offset = *rowadr.add(0);

        // count the number of non-zeros for each row of the transposed matrix
        for r in 0..nr as usize {
            let start = (*rowadr.add(r) - row_offset) as usize;
            let end = start + *rownnz.add(r) as usize;
            for j in start..end {
                *res_rownnz.add(*colind.add(j) as usize) += 1;
            }
        }

        // init res_rowsuper
        if !res_rowsuper.is_null() {
            for i in 0..(nc - 1) as usize {
                *res_rowsuper.add(i) = if *res_rownnz.add(i) == *res_rownnz.add(i + 1) { 1 } else { 0 };
            }
            *res_rowsuper.add((nc - 1) as usize) = 0;
        }

        // compute the row addresses for the transposed matrix
        *res_rowadr.add(0) = 0;
        for i in 1..nc as usize {
            *res_rowadr.add(i) = *res_rowadr.add(i - 1) + *res_rownnz.add(i - 1);
        }

        // iterate through each row (column) of mat (res)
        for r in 0..nr as usize {
            let mut c_prev: i32 = -1;
            let start = (*rowadr.add(r) - row_offset) as usize;
            let end = start + *rownnz.add(r) as usize;
            for i in start..end {
                let c = *colind.add(i) as usize;
                let adr = *res_rowadr.add(c) as usize;
                *res_rowadr.add(c) += 1;
                *res_colind.add(adr) = r as i32;
                if !res.is_null() {
                    *res.add(adr) = *mat.add(i);
                }

                // mark non-supernodes
                if !res_rowsuper.is_null() {
                    if c > 0 && c as i32 != c_prev + 1 && *res_rowsuper.add(c - 1) != 0 {
                        *res_rowsuper.add(c - 1) = 0;
                    }
                    c_prev = c as i32;
                }
            }
        }

        // shift back row addresses
        for i in (1..nc as usize).rev() {
            *res_rowadr.add(i) = *res_rowadr.add(i - 1);
        }
        *res_rowadr.add(0) = 0;

        // accumulate supernodes
        if !res_rowsuper.is_null() {
            for i in (0..(nc - 1) as usize).rev() {
                if *res_rowsuper.add(i) != 0 {
                    *res_rowsuper.add(i) += *res_rowsuper.add(i + 1);
                }
            }
        }
    }
}

/// C: mju_superSparse (engine/engine_util_sparse.h:115)
/// Calls: mju_compare
#[allow(unused_variables, non_snake_case)]
pub fn mju_super_sparse(nr: i32, rowsuper: *mut i32, rownnz: *const i32, rowadr: *const i32, colind: *const i32) {
    // SAFETY: caller guarantees all pointers valid
    unsafe {
        // no rows: nothing to do
        if nr == 0 {
            return;
        }

        // find match to child
        for r in 0..(nr - 1) as usize {
            if *rownnz.add(r) != *rownnz.add(r + 1) {
                *rowsuper.add(r) = 0;
            } else {
                *rowsuper.add(r) = mju_compare(
                    colind.add(*rowadr.add(r) as usize),
                    colind.add(*rowadr.add(r + 1) as usize),
                    *rownnz.add(r),
                );
            }
        }

        // clear last (by definition)
        *rowsuper.add((nr - 1) as usize) = 0;

        // accumulate in reverse
        for r in (0..(nr - 1) as usize).rev() {
            if *rowsuper.add(r) != 0 {
                *rowsuper.add(r) += *rowsuper.add(r + 1);
            }
        }
    }
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
    todo!() // mju_sqrMatTDSparse
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
    todo!() // mju_sqrMatTDSparse_row
}

/// C: mju_sqrMatTDSparseCount (engine/engine_util_sparse.h:139)
/// Calls: mj_freeStack, mj_markStack, mj_stackAllocInfo
#[allow(unused_variables, non_snake_case)]
pub fn mju_sqr_mat_td_sparse_count(res_rownnz: *mut i32, res_rowadr: *mut i32, nr: i32, rownnz: *const i32, rowadr: *const i32, colind: *const i32, rownnzT: *const i32, rowadrT: *const i32, colindT: *const i32, rowsuperT: *const i32, d: *mut mjData, flg_upper: i32) -> i32 {
    todo!() // mju_sqrMatTDSparseCount
}

/// C: mju_sqrMatTDSparseSymbolic (engine/engine_util_sparse.h:148)
/// Calls: mj_freeStack, mj_markStack, mj_stackAllocInfo, mju_copyInt, mju_zeroInt
#[allow(unused_variables, non_snake_case)]
pub fn mju_sqr_mat_td_sparse_symbolic(res_rownnz: *mut i32, res_rowadr: *mut i32, res_colind: *mut i32, res_diagind: *mut i32, nr: i32, nc: i32, rownnz: *const i32, rowadr: *const i32, colind: *const i32, rownnzT: *const i32, rowadrT: *const i32, colindT: *const i32, rowsuperT: *const i32, d: *mut mjData) -> i32 {
    todo!() // mju_sqrMatTDSparseSymbolic
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
    todo!() // mju_sqrMatTDSparseNumeric
}

/// C: mju_sqrMatTDUncompressedInit (engine/engine_util_sparse.h:163)
#[allow(unused_variables, non_snake_case)]
pub fn mju_sqr_mat_td_uncompressed_init(res_rowadr: *mut i32, nc: i32) {
    // SAFETY: caller guarantees res_rowadr[nc] is valid
    unsafe {
        for r in 0..nc as usize {
            *res_rowadr.add(r) = (r as i32) * nc;
        }
    }
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
    // SAFETY: caller guarantees valid pointers and bounds
    unsafe {
        for r in 0..nr as usize {
            let res_r: *mut f64 = res.add(r * nc_res as usize);
            let mat_r: *const f64 = mat.add(*perm_r.add(r) as usize * nc_mat as usize);
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
    // SAFETY: caller guarantees all pointers are valid for their respective dimensions
    unsafe {
        for b in 0..nb as usize {
            let adr = nc_res as usize * *block_r.add(b) as usize;
            mju_block(
                res.add(adr),
                mat,
                nc_mat,
                *block_nc.add(b),
                *block_nr.add(b),
                perm_r.add(*block_r.add(b) as usize),
                perm_c.add(*block_c.add(b) as usize),
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
    // SAFETY: caller guarantees all pointers valid for their dimensions
    unsafe {
        for r in 0..nr as usize {
            let k = *perm_r.add(r) as usize;
            let nnz = *rownnz.add(k);
            *res_rownnz.add(r) = nnz;

            let res_adr = if r == 0 {
                res_offset
            } else {
                *res_rowadr.add(r - 1) + *res_rownnz.add(r - 1)
            };
            *res_rowadr.add(r) = res_adr;

            let res_colind_r = res_colind.add((res_adr - res_offset) as usize);
            let mat_adr = *rowadr.add(k) as usize;
            let colind_k = colind.add(mat_adr);
            for j in 0..nnz as usize {
                *res_colind_r.add(j) = *perm_c.add(*colind_k.add(j) as usize) - col_offset;
            }

            crate::engine::engine_util_blas::mju_copy(
                res.add((res_adr - res_offset) as usize),
                mat.add(mat_adr),
                nnz,
            );
            if !mat2.is_null() && !res2.is_null() {
                crate::engine::engine_util_blas::mju_copy(
                    res2.add((res_adr - res_offset) as usize),
                    mat2.add(mat_adr),
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
    // SAFETY: caller guarantees all pointers valid for their dimensions
    unsafe {
        for b in 0..nb as usize {
            let nr_block = (if (b + 1) < nb as usize { *block_r.add(b + 1) } else { nr })
                - *block_r.add(b);
            let block_r_b = *block_r.add(b) as usize;
            let res_adr = if block_r_b == 0 {
                0
            } else {
                *res_rowadr.add(block_r_b - 1) + *res_rownnz.add(block_r_b - 1)
            };

            let res2_ptr = if res2.is_null() { std::ptr::null_mut() } else { res2.add(res_adr as usize) };

            mju_block_sparse(
                res.add(res_adr as usize),
                res_rownnz.add(block_r_b),
                res_rowadr.add(block_r_b),
                res_colind.add(res_adr as usize),
                mat,
                rownnz,
                rowadr,
                colind,
                nr_block,
                perm_r.add(block_r_b),
                perm_c,
                *block_c.add(b),
                res_adr,
                res2_ptr,
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
pub fn mju_dot_sparse(vec1: *const f64, vec2: *const f64, nnz1: i32, ind1: *const i32) -> f64 {
    // SAFETY: vec1[nnz1], vec2[*], ind1[nnz1] valid per caller contract
    unsafe {
        let mut i: usize = 0;
        let n_4 = nnz1 as usize - 4;
        let mut res0: f64 = 0.0;
        let mut res1: f64 = 0.0;
        let mut res2: f64 = 0.0;
        let mut res3: f64 = 0.0;

        // note: n_4 wraps for nnz1 < 4, but the while condition handles it
        while i <= n_4 && (nnz1 as usize) >= 4 {
            res0 += *vec1.add(i) * *vec2.add(*ind1.add(i) as usize);
            res1 += *vec1.add(i + 1) * *vec2.add(*ind1.add(i + 1) as usize);
            res2 += *vec1.add(i + 2) * *vec2.add(*ind1.add(i + 2) as usize);
            res3 += *vec1.add(i + 3) * *vec2.add(*ind1.add(i + 3) as usize);
            i += 4;
        }

        let mut res = (res0 + res2) + (res1 + res3);

        // scalar part
        while i < nnz1 as usize {
            res += *vec1.add(i) * *vec2.add(*ind1.add(i) as usize);
            i += 1;
        }

        res
    }
}

/// C: mju_compare (engine/engine_util_sparse.h:231)
#[allow(unused_variables, non_snake_case)]
pub fn mju_compare(vec1: *const i32, vec2: *const i32, n: i32) -> i32 {
    // SAFETY: vec1[n] and vec2[n] valid per caller contract
    unsafe {
        let s1 = std::slice::from_raw_parts(vec1 as *const u8, n as usize * std::mem::size_of::<i32>());
        let s2 = std::slice::from_raw_parts(vec2 as *const u8, n as usize * std::mem::size_of::<i32>());
        if s1 == s2 { 1 } else { 0 }
    }
}

/// C: mj_mergeSorted (engine/engine_util_sparse.h:243)
/// Calls: mju_compare
#[allow(unused_variables, non_snake_case)]
pub fn mj_merge_sorted(merge: *mut i32, chain1: *const i32, n1: i32, chain2: *const i32, n2: i32) -> i32 {
    // SAFETY: caller guarantees merge is large enough, chain1[n1], chain2[n2] valid
    unsafe {
        // special case: one or both empty
        if n1 == 0 {
            if n2 == 0 {
                return 0;
            }
            std::ptr::copy_nonoverlapping(chain2, merge, n2 as usize);
            return n2;
        } else if n2 == 0 {
            std::ptr::copy_nonoverlapping(chain1, merge, n1 as usize);
            return n1;
        }

        // special case: identical pattern
        if n1 == n2 && mju_compare(chain1, chain2, n1) != 0 {
            std::ptr::copy_nonoverlapping(chain1, merge, n1 as usize);
            return n1;
        }

        // merge while both chains are non-empty
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        let mut k: i32 = 0;
        while i < n1 && j < n2 {
            let c1 = *chain1.add(i as usize);
            let c2 = *chain2.add(j as usize);

            if c1 < c2 {
                *merge.add(k as usize) = c1;
                k += 1;
                i += 1;
            } else if c1 > c2 {
                *merge.add(k as usize) = c2;
                k += 1;
                j += 1;
            } else {
                *merge.add(k as usize) = c1;
                k += 1;
                i += 1;
                j += 1;
            }
        }

        // copy remaining
        if i < n1 {
            std::ptr::copy_nonoverlapping(chain1.add(i as usize), merge.add(k as usize), (n1 - i) as usize);
            k += n1 - i;
        } else if j < n2 {
            std::ptr::copy_nonoverlapping(chain2.add(j as usize), merge.add(k as usize), (n2 - j) as usize);
            k += n2 - j;
        }

        k
    }
}

/// C: mju_addToSclScl (engine/engine_util_sparse.h:297)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_add_to_scl_scl(res: *mut f64, vec: *const f64, scl1: f64, scl2: f64, n: i32) {
    // SAFETY: res[n] and vec[n] valid per caller contract
    unsafe {
        for i in 0..n as usize {
            *res.add(i) = *res.add(i) * scl1 + *vec.add(i) * scl2;
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
    // SAFETY: caller guarantees all pointers valid, dst has enough space for merged result
    unsafe {
        // check for identical pattern
        if dst_nnz == src_nnz {
            if mju_compare(dst_ind as *const i32, src_ind, dst_nnz) != 0 {
                mju_add_to_scl_scl(dst, src, a, b, dst_nnz);
                return dst_nnz;
            }
        }

        // compute total nnz of result
        let nnz = mju_combine_sparse_count(dst_nnz, src_nnz, dst_ind as *const i32, src_ind);

        // set up read/write pointers at end of arrays
        let mut bi = dst_nnz - 1;
        let mut si = src_nnz - 1;
        let mut w = nnz - 1;

        // merge backwards
        while bi >= 0 && si >= 0 {
            let badr = *dst_ind.add(bi as usize);
            let sadr = *src_ind.add(si as usize);

            if badr == sadr {
                *dst.add(w as usize) = a * *dst.add(bi as usize) + b * *src.add(si as usize);
                *dst_ind.add(w as usize) = badr;
                bi -= 1;
                si -= 1;
            } else if badr > sadr {
                *dst.add(w as usize) = a * *dst.add(bi as usize);
                *dst_ind.add(w as usize) = badr;
                bi -= 1;
            } else {
                *dst.add(w as usize) = b * *src.add(si as usize);
                *dst_ind.add(w as usize) = sadr;
                si -= 1;
            }
            w -= 1;
        }

        // remaining src elements
        while si >= 0 {
            *dst.add(w as usize) = b * *src.add(si as usize);
            *dst_ind.add(w as usize) = *src_ind.add(si as usize);
            si -= 1;
            w -= 1;
        }

        // remaining dst elements: already in place, scale by a
        if a != 1.0 {
            while bi >= 0 {
                *dst.add(bi as usize) *= a;
                bi -= 1;
            }
        }

        nnz
    }
}

