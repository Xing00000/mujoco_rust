//! Port of: engine/engine_core_constraint.c
//! IR hash: 05737965add36adb
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: cell_pos_and_jac (engine/engine_core_constraint.c:55)
/// Calls: mj_bodyChain, mj_jacSparse, mj_stackAllocInfo, mju_zero, mju_zeroInt
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn cell_pos_and_jac(m: *const mjModel, d: *mut mjData, flex_id: i32, npc: i32, gindices: *const i32, nv: i32, xpos_c: *const f64, cell_chain: *mut i32, cell_nnz: *mut i32) -> *mut f64 {
    extern "C" {
        fn cell_pos_and_jac(m: *const mjModel, d: *mut mjData, flex_id: i32, npc: i32, gindices: *const i32, nv: i32, xpos_c: *const f64, cell_chain: *mut i32, cell_nnz: *mut i32) -> *mut f64;
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { cell_pos_and_jac(m, d, flex_id, npc, gindices, nv, xpos_c, cell_chain, cell_nnz) }
}

/// C: cell_strain_jacobian (engine/engine_core_constraint.c:111)
/// Calls: mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn cell_strain_jacobian(npc: i32, cell_nnz: i32, dSdx_local: *const f64, cell_node_jac: *const f64, strain_jac: *mut f64) {
    // WARNING: signature changed — verify body
    // Previous params: (npc : i32, cell_nnz : i32, dSdx_local : * const f64, cell_node_jac : * const f64, strain_jac : * mut f64)
    // Previous return: ()
    unsafe { use crate :: engine :: engine_util_blas :: mju_zero ; mju_zero (strain_jac , cell_nnz) ; for n in 0 .. npc as usize { for c in 0 .. 3usize { let w : f64 = * dSdx_local . add (3 * n + c) ; if w == 0.0 { continue ; } let row = 3 * n + c ; for k in 0 .. cell_nnz as usize { * strain_jac . add (k) += w * * cell_node_jac . add (row * cell_nnz as usize + k) ; } } } }
}

/// C: arenaAllocEfc (engine/engine_core_constraint.c:130)
/// Calls: mj_arenaAllocByte, mj_clearEfc, mj_warning
#[allow(unused_variables, non_snake_case)]
pub fn arena_alloc_efc(m: *const mjModel, d: *mut mjData) -> i32 {
    extern "C" {
        fn arenaAllocEfc(m: *const mjModel, d: *mut mjData) -> i32;
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { arenaAllocEfc(m, d) }
}

/// C: mj_elemBodyWeight (engine/engine_core_constraint.c:223)
/// Calls: mju_dist3, mju_max, mju_message, mju_scl, mju_sum
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_elem_body_weight(m: *const mjModel, d: *const mjData, f: i32, e: i32, v: i32, point: *const f64, body: *mut i32, weight: *mut f64) -> i32 {
    extern "C" {
        fn mj_elemBodyWeight(m: *const mjModel, d: *const mjData, f: i32, e: i32, v: i32, point: *const f64, body: *mut i32, weight: *mut f64) -> i32;
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_elemBodyWeight(m, d, f, e, v, point, body, weight) }
}

/// C: mj_vertBodyWeight (engine/engine_core_constraint.c:265)
/// Calls: mju_addToScl3, mju_cellLookup, mju_evalBasis, mju_evalBasisArray, mju_shellTFIWeights
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_vert_body_weight(m: *const mjModel, d: *const mjData, f: i32, v: *mut i32, body: *mut i32, bweight: *mut f64, vweight: *const f64, nw: i32) -> i32 {
    extern "C" {
        fn mj_vertBodyWeight(m: *const mjModel, d: *const mjData, f: i32, v: *mut i32, body: *mut i32, bweight: *mut f64, vweight: *const f64, nw: i32) -> i32;
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_vertBodyWeight(m, d, f, v, body, bweight, vweight, nw) }
}

/// C: mj_addConstraint (engine/engine_core_constraint.c:414)
/// Calls: mj_isSparse, mju_compare, mju_copy, mju_copyInt, mju_fillInt, mju_message
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_add_constraint(m: *const mjModel, d: *mut mjData, jac: *const f64, pos: *const f64, margin: *const f64, frictionloss: f64, size: i32, r#type: i32, id: i32, NV: i32, chain: *const i32) {
    extern "C" {
        fn mj_addConstraint(m: *const mjModel, d: *mut mjData, jac: *const f64, pos: *const f64, margin: *const f64, frictionloss: f64, size: i32, r#type: i32, id: i32, NV: i32, chain: *const i32);
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_addConstraint(m, d, jac, pos, margin, frictionloss, size, r#type, id, NV, chain) }
}

/// C: mj_equalityAnchors (engine/engine_core_constraint.c:561)
/// Calls: mju_addTo3, mju_copy3, mju_mulMatVec3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_equality_anchors(m: *const mjModel, d: *const mjData, eq_id: i32, pos1: *mut f64, pos2: *mut f64, body1: *mut i32, body2: *mut i32) {
    extern "C" {
        fn mj_equalityAnchors(m: *const mjModel, d: *const mjData, eq_id: i32, pos1: *mut f64, pos2: *mut f64, body1: *mut i32, body2: *mut i32);
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_equalityAnchors(m, d, eq_id, pos1, pos2, body1, body2) }
}

/// C: mj_addConstraintCount (engine/engine_core_constraint.c:1259)
/// Calls: mj_isSparse
#[allow(unused_variables, non_snake_case)]
pub fn mj_add_constraint_count(m: *const mjModel, size: i32, NV: i32) -> i32 {
    // SAFETY: m is valid per caller contract.
    unsafe {
        // over count for dense allocation
        if crate::engine::engine_core_util::mj_is_sparse(m) == 0 {
            return if (*m).nv != 0 { size } else { 0 };
        }
        if NV > 0 { size } else { 0 }
    }
}

/// C: mj_instantiateFriction (engine/engine_core_constraint.c:1270)
/// Calls: mj_addConstraint, mj_addConstraintCount, mj_freeStack, mj_isSparse, mj_markStack, mj_sleepState, mj_stackAllocInfo, mju_sparse2dense, mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mj_instantiate_friction(m: *const mjModel, d: *mut mjData, count_only: i32, nnz: *mut i32) -> i32 {
    extern "C" { fn mj_instantiateFriction(m: *const mjModel, d: *mut mjData, count_only: i32, nnz: *mut i32) -> i32; }
    // SAFETY: delegates to C implementation, pointers valid per caller contract
    unsafe { mj_instantiateFriction(m, d, count_only, nnz) }
}

/// C: mj_instantiateLimit (engine/engine_core_constraint.c:1360)
/// Calls: mj_addConstraint, mj_addConstraintCount, mj_freeStack, mj_isSparse, mj_markStack, mj_sleepState, mj_stackAllocInfo, mju_max, mju_normalize3, mju_normalize4, mju_quat2Vel, mju_scl, mju_scl3, mju_sparse2dense, mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mj_instantiate_limit(m: *const mjModel, d: *mut mjData, count_only: i32, nnz: *mut i32) -> i32 {
    extern "C" { fn mj_instantiateLimit(m: *const mjModel, d: *mut mjData, count_only: i32, nnz: *mut i32) -> i32; }
    // SAFETY: delegates to C implementation, pointers valid per caller contract
    unsafe { mj_instantiateLimit(m, d, count_only, nnz) }
}

/// C: getsolparam (engine/engine_core_constraint.c:1978)
/// Calls: mj_defaultSolRefImp, mju_copy, mju_max, mju_min, mju_warning, mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn getsolparam(m: *const mjModel, d: *const mjData, i: i32, solref: *mut f64, solreffriction: *mut f64, solimp: *mut f64) {
    extern "C" {
        fn getsolparam(m: *const mjModel, d: *const mjData, i: i32, solref: *mut f64, solreffriction: *mut f64, solimp: *mut f64);
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { getsolparam(m, d, i, solref, solreffriction, solimp) }
}

/// C: getposdim (engine/engine_core_constraint.c:2053)
/// Calls: mju_norm
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn getposdim(m: *const mjModel, d: *const mjData, i: i32, pos: *mut f64, dim: *mut i32) {
    extern "C" {
        fn getposdim(m: *const mjModel, d: *const mjData, i: i32, pos: *mut f64, dim: *mut i32);
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { getposdim(m, d, i, pos, dim) }
}

/// C: power (engine/engine_core_constraint.c:2089)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn power(a: f64, b: f64) -> f64 {
    // WARNING: signature changed — verify body
    // Previous params: (a : f64, b : f64)
    // Previous return: f64
    if b == 1.0 { a } else if b == 2.0 { a * a } else { a . powf (b) }
}

/// C: getimpedance (engine/engine_core_constraint.c:2100)
/// Calls: power
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn getimpedance(solimp: *const f64, pos: f64, margin: f64, imp: *mut f64, impP: *mut f64) {
    // SAFETY: solimp points to at least 5 f64; imp, impP point to single f64. Caller guarantees.
    unsafe {
        const MJMINVAL: f64 = 1e-15;

        // flat function
        if *solimp.add(0) == *solimp.add(1) || *solimp.add(2) <= MJMINVAL {
            *imp = 0.5 * (*solimp.add(0) + *solimp.add(1));
            *impP = 0.0;
            return;
        }

        // x = abs((pos-margin) / width)
        let mut x = (pos - margin) / *solimp.add(2);
        let mut sgn: f64 = 1.0;
        if x < 0.0 {
            x = -x;
            sgn = -1.0;
        }

        // fully saturated
        if x >= 1.0 || x <= 0.0 {
            *imp = if x >= 1.0 { *solimp.add(1) } else { *solimp.add(0) };
            *impP = 0.0;
            return;
        }

        // helper: power(a, b)
        #[inline]
        fn power(a: f64, b: f64) -> f64 {
            if b == 1.0 {
                a
            } else if b == 2.0 {
                a * a
            } else {
                a.powf(b)
            }
        }

        // linear
        let y: f64;
        let yP: f64;
        if *solimp.add(4) == 1.0 {
            y = x;
            yP = 1.0;
        }
        // y(x) = a*x^p if x<=midpoint
        else if x <= *solimp.add(3) {
            let a = 1.0 / power(*solimp.add(3), *solimp.add(4) - 1.0);
            y = a * power(x, *solimp.add(4));
            yP = *solimp.add(4) * a * power(x, *solimp.add(4) - 1.0);
        }
        // y(x) = 1-b*(1-x)^p if x>midpoint
        else {
            let b = 1.0 / power(1.0 - *solimp.add(3), *solimp.add(4) - 1.0);
            y = 1.0 - b * power(1.0 - x, *solimp.add(4));
            yP = *solimp.add(4) * b * power(1.0 - x, *solimp.add(4) - 1.0);
        }

        // scale
        *imp = *solimp.add(0) + y * (*solimp.add(1) - *solimp.add(0));
        *impP = yP * sgn * (*solimp.add(1) - *solimp.add(0)) / *solimp.add(2);
    }
}

/// C: mj_jacSumCount (engine/engine_core_constraint.c:2272)
/// Calls: mj_bodyChain, mj_freeStack, mj_markStack, mj_stackAllocInfo, mju_addChains, mju_copyInt
#[allow(unused_variables, non_snake_case)]
pub fn mj_jac_sum_count(m: *const mjModel, d: *mut mjData, chain: *mut i32, n: i32, body: *const i32) -> i32 {
    extern "C" {
        fn mj_jacSumCount(m: *const mjModel, d: *mut mjData, chain: *mut i32, n: i32, body: *const i32) -> i32;
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_jacSumCount(m, d, chain, n, body) }
}

/// C: mj_ne (engine/engine_core_constraint.c:2303)
/// Calls: mj_addConstraintCount, mj_freeStack, mj_jacDifPair, mj_jacSumCount, mj_markStack, mj_sleepState, mj_stackAllocInfo, mju_combineSparseCount, mju_copyInt, mju_flexGatherCellState, mju_flexGatherFaceState, mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_ne(m: *const mjModel, d: *mut mjData, nnz: *mut i32) -> i32 {
    extern "C" { fn mj_ne(m: *const mjModel, d: *mut mjData, nnz: *mut i32) -> i32; }
    // SAFETY: delegates to C implementation, pointers valid per caller contract
    unsafe { mj_ne(m, d, nnz) }
}

/// C: mj_nc (engine/engine_core_constraint.c:2536)
/// Calls: mj_elemBodyWeight, mj_flexBody, mj_freeStack, mj_isPyramidal, mj_isSparse, mj_jacDifPair, mj_jacSumCount, mj_markStack, mj_stackAllocInfo, mj_vertBodyWeight, mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_nc(m: *const mjModel, d: *mut mjData, nnz: *mut i32) -> i32 {
    extern "C" {
        fn mj_nc(m: *const mjModel, d: *mut mjData, nnz: *mut i32) -> i32;
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_nc(m, d, nnz) }
}

/// C: computeY_precount (engine/engine_core_constraint.c:2688)
/// Calls: mju_fillInt
#[allow(unused_variables, non_snake_case)]
pub fn compute_y_precount(Y_rownnz: *mut i32, Y_rowadr: *mut i32, nefc: i32, nv: i32, J_rownnz: *const i32, J_rowadr: *const i32, J_colind: *const i32, M_rownnz: *const i32, M_rowadr: *const i32, M_colind: *const i32, marker: *mut i32) -> i32 {
    // SAFETY: all pointers are valid arrays of appropriate size. Caller guarantees.
    unsafe {
        crate::engine::engine_util_misc::mju_fill_int(marker, -1, nv);

        *Y_rowadr.add(0) = 0;
        for r in 0..nefc {
            let mut nnz: i32 = 0; // nonzeros in row r of Y

            // traverse row r of J in reverse, count unique nonzeros
            let start = *J_rowadr.add(r as usize);
            let end = start + *J_rownnz.add(r as usize);
            let mut i = end - 1;
            while i >= start {
                let j = *J_colind.add(i as usize);

                // if dof j is marked, it was already counted by a child dof: skip it
                if *marker.add(j as usize) == r {
                    i -= 1;
                    continue;
                }

                // traverse row j of M, marking new unique nonzeros
                let nnzM = *M_rownnz.add(j as usize);
                let adrM = *M_rowadr.add(j as usize);
                for k in 0..nnzM {
                    let c = *M_colind.add((adrM + k) as usize);
                    if *marker.add(c as usize) != r {
                        *marker.add(c as usize) = r;
                        nnz += 1;
                    }
                }

                i -= 1;
            }

            // update rownnz and rowadr
            *Y_rownnz.add(r as usize) = nnz;
            if r < nefc - 1 {
                *Y_rowadr.add((r + 1) as usize) = *Y_rowadr.add(r as usize) + nnz;
            }
        }

        // total non-zeros in Y
        *Y_rowadr.add((nefc - 1) as usize) + *Y_rownnz.add((nefc - 1) as usize)
    }
}

/// C: computeY_fill (engine/engine_core_constraint.c:2734)
/// Calls: mju_message
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn compute_y_fill(Y: *mut f64, Y_colind: *mut i32, Y_rownnz: *const i32, Y_rowadr: *const i32, nefc: i32, J: *const f64, J_rownnz: *const i32, J_rowadr: *const i32, J_colind: *const i32, dof_parentid: *const i32) {
    // SAFETY: all pointers are valid arrays of appropriate size. Caller guarantees.
    unsafe {
        for r in 0..nefc {
            // init row
            let end = *Y_rowadr.add(r as usize) + *Y_rownnz.add(r as usize);
            let adrJ = *J_rowadr.add(r as usize);
            let mut remainJ = *J_rownnz.add(r as usize);
            let mut nnzY: i32 = 0;

            // complete chain in reverse
            loop {
                // get previous dof in src and dst
                let prev_src = if remainJ > 0 { *J_colind.add((adrJ + remainJ - 1) as usize) } else { -1 };
                let prev_dst = if nnzY > 0 { *dof_parentid.add(*Y_colind.add((end - nnzY) as usize) as usize) } else { -1 };

                // both finished: break
                if prev_src < 0 && prev_dst < 0 {
                    break;
                }
                // add src
                else if prev_src >= prev_dst {
                    nnzY += 1;
                    remainJ -= 1;
                    *Y_colind.add((end - nnzY) as usize) = prev_src;
                    *Y.add((end - nnzY) as usize) = *J.add((adrJ + remainJ) as usize);
                }
                // add dst
                else {
                    nnzY += 1;
                    *Y_colind.add((end - nnzY) as usize) = prev_dst;
                    *Y.add((end - nnzY) as usize) = 0.0;
                }
            }

            // compare with Y_rownnz: SHOULD NOT OCCUR
            if nnzY != *Y_rownnz.add(r as usize) {
                crate::engine::engine_util_errmem::mju_error(
                    b"pre and post-count of Y_rownnz are not equal\0".as_ptr() as *const i8);
            }
        }
    }
}

/// C: computeY_backsub (engine/engine_core_constraint.c:2781)
/// Calls: mju_addToSclSparseInc
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn compute_y_backsub(Y: *mut f64, Y_rownnz: *const i32, Y_rowadr: *const i32, Y_colind: *const i32, nefc: i32, qLD: *const f64, M_rownnz: *const i32, M_rowadr: *const i32, M_colind: *const i32, sqrtInvD: *const f64) {
    // SAFETY: all pointers are valid arrays of appropriate size. Caller guarantees.
    unsafe {
        for r in 0..nefc {
            let nnzY = *Y_rownnz.add(r as usize);
            let adrY = *Y_rowadr.add(r as usize);

            // Y(r,:) <- inv(L') * Y(r,:), exploit sparsity of input vector
            let mut i = adrY + nnzY - 1;
            while i >= adrY {
                let val = *Y.add(i as usize);
                if val == 0.0 {
                    i -= 1;
                    continue;
                }
                let j = *Y_colind.add(i as usize);
                let adrM = *M_rowadr.add(j as usize);
                crate::engine::engine_util_sparse::mju_add_to_scl_sparse_inc(
                    Y.add(adrY as usize), qLD.add(adrM as usize),
                    nnzY, Y_colind.add(adrY as usize),
                    *M_rownnz.add(j as usize) - 1, M_colind.add(adrM as usize), -val);
                i -= 1;
            }

            // Y(r,:) <- sqrt(inv(D)) * Y(r,:)
            for i in adrY..(adrY + nnzY) {
                let j = *Y_colind.add(i as usize);
                *Y.add(i as usize) *= *sqrtInvD.add(j as usize);
            }
        }
    }
}

/// C: mj_makeY (engine/engine_core_constraint.c:2908)
/// Calls: computeY_backsub, computeY_fill, computeY_precount, mj_arenaAllocByte, mj_clearEfc, mj_freeStack, mj_isSparse, mj_markStack, mj_solveM2, mj_stackAllocInfo, mj_warning, mju_dot
#[allow(unused_variables, non_snake_case)]
pub fn mj_make_y(m: *const mjModel, d: *mut mjData, flg_diagexact: i32) {
    extern "C" {
        fn mj_makeY(m: *const mjModel, d: *mut mjData, flg_diagexact: i32);
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_makeY(m, d, flg_diagexact) }
}

/// C: mj_makeAR (engine/engine_core_constraint.c:2999)
/// Calls: mj_arenaAllocByte, mj_clearEfc, mj_freeStack, mj_isSparse, mj_markStack, mj_stackAllocInfo, mj_warning, mju_sqrMatTD, mju_sqrMatTDSparseNumeric, mju_sqrMatTDSparseSymbolic, mju_transpose, mju_transposeSparse
#[allow(unused_variables, non_snake_case)]
pub fn mj_make_ar(m: *const mjModel, d: *mut mjData) {
    extern "C" {
        fn mj_makeAR(m: *const mjModel, d: *mut mjData);
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_makeAR(m, d) }
}

/// C: mj_isDual (engine/engine_core_constraint.h:31)
#[allow(unused_variables, non_snake_case)]
pub fn mj_is_dual(m: *const mjModel) -> i32 {
    unsafe {
        // SAFETY: caller guarantees m is a valid mjModel pointer
        const mjSOL_PGS: i32 = 0;
        if (*m).opt.solver == mjSOL_PGS || (*m).opt.noslip_iterations > 0 {
            1
        } else {
            0
        }
    }
}

/// C: mj_mulJacVec (engine/engine_core_constraint.h:34)
/// Calls: mj_isSparse, mju_mulMatVec, mju_mulMatVecSparse
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_mul_jac_vec(m: *const mjModel, d: *const mjData, res: *mut f64, vec: *const f64) {
    // SAFETY: all pointers valid per caller contract, field access matches C struct layout
    unsafe {
        if (*d).nefc == 0 {
            return;
        }

        if crate::engine::engine_core_util::mj_is_sparse(m) != 0 {
            crate::engine::engine_util_sparse::mju_mul_mat_vec_sparse(
                res, (*d).efc_J, vec, (*d).nefc,
                (*d).efc_J_rownnz, (*d).efc_J_rowadr,
                (*d).efc_J_colind, (*d).efc_J_rowsuper,
            );
        } else {
            crate::engine::engine_util_blas::mju_mul_mat_vec(
                res, (*d).efc_J, vec, (*d).nefc, (*m).nv as i32,
            );
        }
    }
}

/// C: mj_mulJacTVec (engine/engine_core_constraint.h:37)
/// Calls: mj_isSparse, mju_mulMatTVec, mju_mulMatTVecSparse
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_mul_jac_t_vec(m: *const mjModel, d: *const mjData, res: *mut f64, vec: *const f64) {
    // SAFETY: all pointers valid per caller contract, field access matches C struct layout
    unsafe {
        if (*d).nefc == 0 {
            return;
        }

        if crate::engine::engine_core_util::mj_is_sparse(m) != 0 {
            crate::engine::engine_util_sparse::mju_mul_mat_t_vec_sparse(
                res, (*d).efc_J, vec, (*d).nefc, (*m).nv as i32,
                (*d).efc_J_rownnz, (*d).efc_J_rowadr, (*d).efc_J_colind,
            );
        } else {
            crate::engine::engine_util_blas::mju_mul_mat_t_vec(
                res, (*d).efc_J, vec, (*d).nefc, (*m).nv as i32,
            );
        }
    }
}

/// C: mj_Jdotv (engine/engine_core_constraint.h:40)
/// Calls: mj_equalityAnchors, mj_freeStack, mj_isSparse, mj_jacDot, mj_jacDotSparse, mj_markStack, mj_mergeChain, mj_stackAllocInfo, mju_copy4, mju_derivQuat, mju_dotSparseX3, mju_mulMatVec, mju_mulQuat, mju_mulQuatAxis, mju_negQuat, mju_sub3, mju_zero3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_jdotv(m: *const mjModel, d: *mut mjData, result: *mut f64) {
    extern "C" { fn mj_Jdotv(m: *const mjModel, d: *mut mjData, result: *mut f64); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_Jdotv(m, d, result) }
}

/// C: mj_assignRef (engine/engine_core_constraint.h:46)
/// Calls: mju_copy
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_assign_ref(m: *const mjModel, target: *mut f64, source: *const f64) {
    unsafe {
        // SAFETY: m is a valid pointer to mjModel (caller contract)
        // mjENABLED(mjENBL_OVERRIDE) => m->opt.enableflags & (1<<0)
        if ((*m).opt.enableflags & 1) != 0 {
            // SAFETY: target is writable for mjNREF=2 elements;
            // o_solref is [f64; 2] within mjOption
            crate::engine::engine_util_blas::mju_copy(
                target,
                (*m).opt.o_solref.as_ptr(),
                2, // mjNREF
            );
        } else {
            // SAFETY: target is writable for mjNREF=2 elements;
            // source is readable for mjNREF=2 elements
            crate::engine::engine_util_blas::mju_copy(target, source, 2 /* mjNREF */);
        }
    }
}

/// C: mj_assignImp (engine/engine_core_constraint.h:49)
/// Calls: mju_copy
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_assign_imp(m: *const mjModel, target: *mut f64, source: *const f64) {
    // SAFETY: m is a valid mjModel pointer. target and source point to arrays of at least 5 elements.
    unsafe {
        if ((*m).opt.enableflags & 1) != 0 {
            crate::engine::engine_util_blas::mju_copy(target, (*m).opt.o_solimp.as_ptr(), 5);
        } else {
            crate::engine::engine_util_blas::mju_copy(target, source, 5);
        }
    }
}

/// C: mj_assignFriction (engine/engine_core_constraint.h:52)
/// Calls: mju_max
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_assign_friction(m: *const mjModel, target: *mut f64, source: *const f64) {
    // SAFETY: m is a valid mjModel pointer. target and source point to arrays of at least 5 elements.
    unsafe {
        if ((*m).opt.enableflags & 1) != 0 {
            let mut i: usize = 0;
            while i < 5 {
                *target.add(i) = crate::engine::engine_util_misc::mju_max(1e-8, (*m).opt.o_friction[i]);
                i += 1;
            }
        } else {
            let mut i: usize = 0;
            while i < 5 {
                *target.add(i) = crate::engine::engine_util_misc::mju_max(1e-8, *source.add(i));
                i += 1;
            }
        }
    }
}

/// C: mj_assignMargin (engine/engine_core_constraint.h:55)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_assign_margin(m: *const mjModel, source: f64) -> f64 {
    // SAFETY: m is a valid mjModel pointer.
    unsafe {
        if ((*m).opt.enableflags & 1) != 0 {
            (*m).opt.o_margin
        } else {
            source
        }
    }
}

/// C: mj_addContact (engine/engine_core_constraint.h:58)
/// Calls: mj_arenaAllocByte, mj_clearEfc, mj_warning
#[allow(unused_variables, non_snake_case)]
pub fn mj_add_contact(m: *const mjModel, d: *mut mjData, con: *const mjContact) -> i32 {
    // SAFETY: m, d, con are valid pointers. d->arena is a valid arena pointer.
    unsafe {
        // move arena pointer back to the end of the existing contact array and invalidate efc_ arrays
        (*d).parena = (*d).ncon as usize * std::mem::size_of::<mjContact>();
        crate::engine::engine_memory::mj_clear_efc(d);

        // copy contact
        let dst: *mut () = crate::engine::engine_memory::mj_arena_alloc_byte(
            d,
            std::mem::size_of::<mjContact>(),
            std::mem::align_of::<mjContact>(),
        );
        if dst.is_null() {
            crate::engine::engine_core_util::mj_warning(d, 1, (*d).ncon); // mjWARN_CONTACTFULL = 1
            return 1;
        }
        *(dst as *mut mjContact) = *con;

        // increase counter, return success
        (*d).ncon += 1;
        0
    }
}

/// C: mj_instantiateEquality (engine/engine_core_constraint.h:63)
/// Calls: cell_pos_and_jac, cell_strain_jacobian, mj_addConstraint, mj_equalityAnchors, mj_freeStack, mj_isSparse, mj_jacDifPair, mj_markStack, mj_sleepState, mj_stackAllocInfo, mju_addTo3, mju_addToScl, mju_combineSparse, mju_copy, mju_copy3, mju_copyInt, mju_defGradient, mju_flexGatherCellState, mju_flexGatherFaceState, mju_flexInterpRotation2D, mju_mat2Rot, mju_message, mju_mulMatVec3, mju_mulQuat, mju_mulQuatAxis, mju_negQuat, mju_rotVecQuat, mju_scl, mju_scl3, mju_sparse2dense, mju_sub3, mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mj_instantiate_equality(m: *const mjModel, d: *mut mjData) {
    extern "C" { fn mj_instantiateEquality(m: *const mjModel, d: *mut mjData); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_instantiateEquality(m, d) }
}

/// C: mj_instantiateContact (engine/engine_core_constraint.h:66)
/// Calls: mj_addConstraint, mj_contactJacobian, mj_freeStack, mj_isPyramidal, mj_isSparse, mj_markStack, mj_stackAllocInfo, mju_addScl, mju_mulMatMat, mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mj_instantiate_contact(m: *const mjModel, d: *mut mjData) {
    extern "C" { fn mj_instantiateContact(m: *const mjModel, d: *mut mjData); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_instantiateContact(m, d) }
}

/// C: mj_contactJacobian (engine/engine_core_constraint.h:69)
/// Calls: mj_elemBodyWeight, mj_isSparse, mj_jacDifPair, mj_jacSum, mj_vertBodyWeight, mju_scl
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_contact_jacobian(m: *const mjModel, d: *mut mjData, con: *const mjContact, dim: i32, jac: *mut f64, jacdif: *mut f64, jacdifp: *mut f64, jacdifr: *mut f64, jac1p: *mut f64, jac2p: *mut f64, jac1r: *mut f64, jac2r: *mut f64, chain: *mut i32) -> i32 {
    extern "C" {
        fn mj_contactJacobian(m: *const mjModel, d: *mut mjData, con: *const mjContact, dim: i32, jac: *mut f64, jacdif: *mut f64, jacdifp: *mut f64, jacdifr: *mut f64, jac1p: *mut f64, jac2p: *mut f64, jac1r: *mut f64, jac2r: *mut f64, chain: *mut i32) -> i32;
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_contactJacobian(m, d, con, dim, jac, jacdif, jacdifp, jacdifr, jac1p, jac2p, jac1r, jac2r, chain) }
}

/// C: mj_diagApprox (engine/engine_core_constraint.h:78)
/// Calls: mj_elemBodyWeight, mj_vertBodyWeight, mju_flexGatherCellState, mju_flexGatherFaceState, mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_diag_approx(m: *const mjModel, d: *mut mjData) {
    extern "C" {
        fn mj_diagApprox(m: *const mjModel, d: *mut mjData);
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_diagApprox(m, d) }
}

/// C: mj_makeImpedance (engine/engine_core_constraint.h:81)
/// Calls: getimpedance, getposdim, getsolparam, mju_max
#[allow(unused_variables, non_snake_case)]
pub fn mj_make_impedance(m: *const mjModel, d: *mut mjData) {
    extern "C" {
        fn mj_makeImpedance(m: *const mjModel, d: *mut mjData);
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_makeImpedance(m, d) }
}

/// C: mj_makeConstraint (engine/engine_core_constraint.h:87)
/// Calls: arenaAllocEfc, mj_diagApprox, mj_instantiateContact, mj_instantiateEquality, mj_instantiateFriction, mj_instantiateLimit, mj_isSparse, mj_makeImpedance, mj_nc, mj_ne, mju_fillInt, mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_make_constraint(m: *const mjModel, d: *mut mjData) {
    extern "C" { fn mj_makeConstraint(m: *const mjModel, d: *mut mjData); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_makeConstraint(m, d) }
}

/// C: mj_projectConstraint (engine/engine_core_constraint.h:90)
/// Calls: mj_isDual, mj_makeAR, mj_makeImpedance, mj_makeY, mju_gather
#[allow(unused_variables, non_snake_case)]
pub fn mj_project_constraint(m: *const mjModel, d: *mut mjData) {
    extern "C" {
        fn mj_projectConstraint(m: *const mjModel, d: *mut mjData);
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_projectConstraint(m, d) }
}

/// C: mj_referenceConstraint (engine/engine_core_constraint.h:93)
/// Calls: mj_Jdotv, mj_mulJacVec
#[allow(unused_variables, non_snake_case)]
pub fn mj_reference_constraint(m: *const mjModel, d: *mut mjData) {
    extern "C" { fn mj_referenceConstraint(m: *const mjModel, d: *mut mjData); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_referenceConstraint(m, d) }
}

/// C: mj_constraintUpdate(engine/engine_core_constraint.h:97)
/// Calls: mju_norm, mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_constraint_update_impl(ne: i32, nf: i32, nefc: i32, D: *const f64, R: *const f64, floss: *const f64, jar: *const f64, r#type: *const i32, id: *const i32, contact: *mut mjContact, state: *mut i32, force: *mut f64, cost: *mut f64, flg_coneHessian: i32) {
    // SAFETY: all pointers valid per caller contract. D, R, floss, jar, type, id,
    // state, force all have at least nefc elements. contact has adequate capacity.
    // cost may be null (skip cost accumulation if null).
    unsafe {
        const MJCNSTRSTATE_QUADRATIC: i32 = 0;
        const MJCNSTRSTATE_LINEARNEG: i32 = 1;
        const MJCNSTRSTATE_LINEARPOS: i32 = 2;
        const MJCNSTRSTATE_SATISFIED: i32 = 3;
        const MJCNSTRSTATE_CONE: i32 = 4;
        const MJCNSTR_CONTACT_ELLIPTIC: i32 = 3;

        let mut s: f64 = 0.0;

        // no constraints: clear cost, return
        if nefc == 0 {
            if !cost.is_null() {
                *cost = 0.0;
            }
            return;
        }

        // compute unconstrained efc_force
        let mut i: i32 = 0;
        while i < nefc {
            *force.add(i as usize) = -*D.add(i as usize) * *jar.add(i as usize);
            i += 1;
        }

        // update constraints
        i = 0;
        while i < nefc {
            // ==== equality
            if i < ne {
                if !cost.is_null() {
                    s += 0.5 * *D.add(i as usize) * *jar.add(i as usize) * *jar.add(i as usize);
                }
                *state.add(i as usize) = MJCNSTRSTATE_QUADRATIC;
                i += 1;
                continue;
            }

            // ==== friction
            if i < ne + nf {
                let jar_i = *jar.add(i as usize);
                let R_i = *R.add(i as usize);
                let floss_i = *floss.add(i as usize);
                let D_i = *D.add(i as usize);

                // linear negative
                if jar_i <= -R_i * floss_i {
                    if !cost.is_null() {
                        s += -0.5 * R_i * floss_i * floss_i - floss_i * jar_i;
                    }
                    *force.add(i as usize) = floss_i;
                    *state.add(i as usize) = MJCNSTRSTATE_LINEARNEG;
                }
                // linear positive
                else if jar_i >= R_i * floss_i {
                    if !cost.is_null() {
                        s += -0.5 * R_i * floss_i * floss_i + floss_i * jar_i;
                    }
                    *force.add(i as usize) = -floss_i;
                    *state.add(i as usize) = MJCNSTRSTATE_LINEARPOS;
                }
                // quadratic
                else {
                    if !cost.is_null() {
                        s += 0.5 * D_i * jar_i * jar_i;
                    }
                    *state.add(i as usize) = MJCNSTRSTATE_QUADRATIC;
                }
                i += 1;
                continue;
            }

            // ==== contact

            // non-negative constraint
            if *r#type.add(i as usize) != MJCNSTR_CONTACT_ELLIPTIC {
                // constraint is satisfied: no cost
                if *jar.add(i as usize) >= 0.0 {
                    *force.add(i as usize) = 0.0;
                    *state.add(i as usize) = MJCNSTRSTATE_SATISFIED;
                }
                // quadratic
                else {
                    if !cost.is_null() {
                        s += 0.5 * *D.add(i as usize) * *jar.add(i as usize) * *jar.add(i as usize);
                    }
                    *state.add(i as usize) = MJCNSTRSTATE_QUADRATIC;
                }
            }
            // contact with elliptic cone
            else {
                // get contact
                let con = &mut *contact.add(*id.add(i as usize) as usize);
                let mu = con.mu;
                let friction = &con.friction;
                let dim = con.dim;

                // map to regular dual cone space
                let mut U: [f64; 6] = [0.0; 6];
                U[0] = *jar.add(i as usize) * mu;
                let mut j: i32 = 1;
                while j < dim {
                    U[j as usize] = *jar.add((i + j) as usize) * friction[(j - 1) as usize];
                    j += 1;
                }

                // decompose into normal and tangent
                let N = U[0];
                let T = crate::engine::engine_util_blas::mju_norm(U.as_ptr().add(1), dim - 1);

                // top zone
                if N >= mu * T || (T <= 0.0 && N >= 0.0) {
                    crate::engine::engine_util_blas::mju_zero(force.add(i as usize), dim);
                    *state.add(i as usize) = MJCNSTRSTATE_SATISFIED;
                }
                // bottom zone
                else if mu * N + T <= 0.0 || (T <= 0.0 && N < 0.0) {
                    if !cost.is_null() {
                        let mut j: i32 = 0;
                        while j < dim {
                            s += 0.5 * *D.add((i + j) as usize) * *jar.add((i + j) as usize) * *jar.add((i + j) as usize);
                            j += 1;
                        }
                    }
                    *state.add(i as usize) = MJCNSTRSTATE_QUADRATIC;
                }
                // middle zone
                else {
                    // cost: 0.5*D0/(mu*mu*(1+mu*mu))*(N-mu*T)^2
                    let Dm = *D.add(i as usize) / (mu * mu * (1.0 + mu * mu));
                    let NmT = N - mu * T;

                    if !cost.is_null() {
                        s += 0.5 * Dm * NmT * NmT;
                    }

                    // force: - ds/djar = dU/djar * ds/dU
                    *force.add(i as usize) = -Dm * NmT * mu;
                    let mut j: i32 = 1;
                    while j < dim {
                        *force.add((i + j) as usize) = -*force.add(i as usize) / T * U[j as usize] * friction[(j - 1) as usize];
                        j += 1;
                    }

                    // set state
                    *state.add(i as usize) = MJCNSTRSTATE_CONE;

                    // cone Hessian
                    if flg_coneHessian != 0 {
                        // get Hessian pointer
                        let H = (*contact.add(*id.add(i as usize) as usize)).H.as_mut_ptr();

                        // set first row: (1, -mu/T * U)
                        let mut scl = -mu / T;
                        *H.add(0) = 1.0;
                        let mut j: i32 = 1;
                        while j < dim {
                            *H.add(j as usize) = scl * U[j as usize];
                            j += 1;
                        }

                        // set upper block: mu*N/T^3 * U*U'
                        scl = mu * N / (T * T * T);
                        let mut k: i32 = 1;
                        while k < dim {
                            let mut j: i32 = k;
                            while j < dim {
                                *H.add((k * dim + j) as usize) = scl * U[j as usize] * U[k as usize];
                                j += 1;
                            }
                            k += 1;
                        }

                        // add to diagonal: (mu^2 - mu*N/T) * I
                        scl = mu * mu - mu * N / T;
                        let mut j: i32 = 1;
                        while j < dim {
                            *H.add((j * (dim + 1)) as usize) += scl;
                            j += 1;
                        }

                        // pre and post multiply by diag(mu, friction), scale by Dm
                        let mut k: i32 = 0;
                        while k < dim {
                            scl = Dm * (if k == 0 { mu } else { friction[(k - 1) as usize] });
                            let mut j: i32 = k;
                            while j < dim {
                                *H.add((k * dim + j) as usize) *= scl * (if j == 0 { mu } else { friction[(j - 1) as usize] });
                                j += 1;
                            }
                            k += 1;
                        }

                        // make symmetric: copy upper into lower
                        let mut k: i32 = 0;
                        while k < dim {
                            let mut j: i32 = k + 1;
                            while j < dim {
                                *H.add((j * dim + k) as usize) = *H.add((k * dim + j) as usize);
                                j += 1;
                            }
                            k += 1;
                        }
                    }
                }

                // replicate state in all cone dimensions
                let mut j: i32 = 1;
                while j < dim {
                    *state.add((i + j) as usize) = *state.add(i as usize);
                    j += 1;
                }

                // advance to end of contact
                i += dim - 1;
            }

            i += 1;
        }

        // assign cost
        if !cost.is_null() {
            *cost = s;
        }
    }
}

/// C: mj_constraintUpdate (engine/engine_core_constraint.h:105)
/// Calls: mj_constraintUpdate_impl, mj_mulJacTVec
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_constraint_update(m: *const mjModel, d: *mut mjData, jar: *const f64, cost: *mut f64, flg_coneHessian: i32) {
    // SAFETY: m, d valid per caller. jar points to d->nefc elements.
    // Delegates to mj_constraint_update_impl then mj_mul_jac_t_vec.
    unsafe {
        mj_constraint_update_impl(
            (*d).ne, (*d).nf, (*d).nefc,
            (*d).efc_D, (*d).efc_R, (*d).efc_frictionloss,
            jar, (*d).efc_type, (*d).efc_id, (*d).contact, (*d).efc_state, (*d).efc_force,
            cost, flg_coneHessian,
        );
        mj_mul_jac_t_vec(m, d as *const mjData, (*d).qfrc_constraint, (*d).efc_force);
    }
}

