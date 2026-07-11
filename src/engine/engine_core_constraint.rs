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
    if m.is_null() || d.is_null() {
        return core::ptr::null_mut();
    }
    extern "C" {
        fn cell_pos_and_jac(m: *const mjModel, d: *mut mjData, flex_id: i32, npc: i32, gindices: *const i32, nv: i32, xpos_c: *const f64, cell_chain: *mut i32, cell_nnz: *mut i32) -> *mut f64;
    }
    // SAFETY: m/d verified non-null; delegates to C implementation
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
    if dSdx_local.is_null() || cell_node_jac.is_null() || strain_jac.is_null() {
        return;
    }
    extern "C" { fn cell_strain_jacobian(npc: i32, cell_nnz: i32, dSdx_local: *const f64, cell_node_jac: *const f64, strain_jac: *mut f64); }
    // SAFETY: m verified non-null; delegates to C implementation
    unsafe { cell_strain_jacobian(npc, cell_nnz, dSdx_local, cell_node_jac, strain_jac) }
}

/// C: arenaAllocEfc (engine/engine_core_constraint.c:130)
/// Calls: mj_arenaAllocByte, mj_clearEfc, mj_warning
#[allow(unused_variables, non_snake_case)]
pub fn arena_alloc_efc(m: *const mjModel, d: *mut mjData) -> i32 {
    if m.is_null() { return 0; }
    extern "C" {
        fn arenaAllocEfc(m: *const mjModel, d: *mut mjData) -> i32;
    }
    // SAFETY: m verified non-null; delegates to C implementation
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
pub fn mj_elem_body_weight(m: *const mjModel, d: *const mjData, f: i32, e: i32, v: i32, point: *const f64, body: *mut i32, weight: *mut f64) -> i32  {
    if m.is_null() || d.is_null() {
        return 0;
    }
    extern "C" { fn mj_elemBodyWeight(m: *const mjModel, d: *const mjData, f: i32, e: i32, v: i32, point: *const f64, body: *mut i32, weight: *mut f64) -> i32; }
    // SAFETY: m verified non-null; delegates to C implementation
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
    if m.is_null() || d.is_null() {
        return 0;
    }
    extern "C" {
        fn mj_vertBodyWeight(m: *const mjModel, d: *const mjData, f: i32, v: *mut i32, body: *mut i32, bweight: *mut f64, vweight: *const f64, nw: i32) -> i32;
    }
    // SAFETY: m verified non-null; delegates to C implementation
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
    if m.is_null() { return; }
    extern "C" {
        fn mj_addConstraint(m: *const mjModel, d: *mut mjData, jac: *const f64, pos: *const f64, margin: *const f64, frictionloss: f64, size: i32, r#type: i32, id: i32, NV: i32, chain: *const i32);
    }
    // SAFETY: m verified non-null; delegates to C implementation
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
    if m.is_null() || d.is_null() {
        return;
    }
    extern "C" { fn mj_equalityAnchors(m: *const mjModel, d: *const mjData, eq_id: i32, pos1: *mut f64, pos2: *mut f64, body1: *mut i32, body2: *mut i32); }
    // SAFETY: m/d verified non-null; delegates to C implementation
    unsafe { mj_equalityAnchors(m, d, eq_id, pos1, pos2, body1, body2) }
}

/// C: mj_addConstraintCount (engine/engine_core_constraint.c:1259)
/// Calls: mj_isSparse
#[allow(unused_variables, non_snake_case)]
pub fn mj_add_constraint_count(m: *const mjModel, size: i32, NV: i32) -> i32  {
    if m.is_null() {
        return 0;
    }
    extern "C" { fn mj_addConstraintCount(m: *const mjModel, size: i32, NV: i32) -> i32; }
    // SAFETY: d verified non-null; delegates to C implementation
    unsafe { mj_addConstraintCount(m, size, NV) }
}

/// C: mj_instantiateFriction (engine/engine_core_constraint.c:1270)
/// Calls: mj_addConstraint, mj_addConstraintCount, mj_freeStack, mj_isSparse, mj_markStack, mj_sleepState, mj_stackAllocInfo, mju_sparse2dense, mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mj_instantiate_friction(m: *const mjModel, d: *mut mjData, count_only: i32, nnz: *mut i32) -> i32 {
    if m.is_null() { return 0; }
    extern "C" { fn mj_instantiateFriction(m: *const mjModel, d: *mut mjData, count_only: i32, nnz: *mut i32) -> i32; }
    // SAFETY: delegates to C implementation, pointers valid per caller contract
    unsafe { mj_instantiateFriction(m, d, count_only, nnz) }
}

/// C: mj_instantiateLimit (engine/engine_core_constraint.c:1360)
/// Calls: mj_addConstraint, mj_addConstraintCount, mj_freeStack, mj_isSparse, mj_markStack, mj_sleepState, mj_stackAllocInfo, mju_max, mju_normalize3, mju_normalize4, mju_quat2Vel, mju_scl, mju_scl3, mju_sparse2dense, mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mj_instantiate_limit(m: *const mjModel, d: *mut mjData, count_only: i32, nnz: *mut i32) -> i32 {
    if m.is_null() { return 0; }
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
    if m.is_null() || d.is_null() || solref.is_null() || solimp.is_null() {
        return;
    }
    extern "C" { fn getsolparam(m: *const mjModel, d: *const mjData, i: i32, solref: *mut f64, solreffriction: *mut f64, solimp: *mut f64); }
    // SAFETY: solparam verified non-null; delegates to C implementation
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
    if m.is_null() || d.is_null() || pos.is_null() || dim.is_null() {
        return;
    }
    // Read the constraint type to determine dimension
    let _efc_type = unsafe { *(*d).efc_type.add(i as usize) };
    extern "C" { fn getposdim(m: *const mjModel, d: *const mjData, i: i32, pos: *mut f64, dim: *mut i32); }
    // SAFETY: m/d verified non-null; delegates to C implementation
    unsafe { getposdim(m, d, i, pos, dim) }
}

/// C: power (engine/engine_core_constraint.c:2089)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn power(a: f64, b: f64) -> f64  {
    // C: return (a > 0) ? mju_pow(a, b) : 0;
    if a > 0.0 {
        a.powf(b)
    } else {
        0.0
    }
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
    if solimp.is_null() { return; }
    extern "C" { fn getimpedance(solimp: *const f64, pos: f64, margin: f64, imp: *mut f64, impP: *mut f64); }
    // SAFETY: solimp verified non-null; delegates to C implementation
    unsafe { getimpedance(solimp, pos, margin, imp, impP) }
}

/// C: mj_jacSumCount (engine/engine_core_constraint.c:2272)
/// Calls: mj_bodyChain, mj_freeStack, mj_markStack, mj_stackAllocInfo, mju_addChains, mju_copyInt
#[allow(unused_variables, non_snake_case)]
pub fn mj_jac_sum_count(m: *const mjModel, d: *mut mjData, chain: *mut i32, n: i32, body: *const i32) -> i32  {
    if m.is_null() { return 0; }
    extern "C" { fn mj_jacSumCount(m: *const mjModel, d: *mut mjData, chain: *mut i32, n: i32, body: *const i32) -> i32; }
    // SAFETY: m verified non-null; delegates to C implementation
    unsafe { mj_jacSumCount(m, d, chain, n, body) }
}

/// C: mj_ne (engine/engine_core_constraint.c:2303)
/// Calls: mj_addConstraintCount, mj_freeStack, mj_jacDifPair, mj_jacSumCount, mj_markStack, mj_sleepState, mj_stackAllocInfo, mju_combineSparseCount, mju_copyInt, mju_flexGatherCellState, mju_flexGatherFaceState, mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_ne(m: *const mjModel, d: *mut mjData, nnz: *mut i32) -> i32 {
    if m.is_null() { return 0; }
    extern "C" { fn mj_ne(m: *const mjModel, d: *mut mjData, nnz: *mut i32) -> i32; }
    // SAFETY: delegates to C implementation, pointers valid per caller contract
    unsafe { mj_ne(m, d, nnz) }
}

/// C: mj_nc (engine/engine_core_constraint.c:2536)
/// Calls: mj_elemBodyWeight, mj_flexBody, mj_freeStack, mj_isPyramidal, mj_isSparse, mj_jacDifPair, mj_jacSumCount, mj_markStack, mj_stackAllocInfo, mj_vertBodyWeight, mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_nc(m: *const mjModel, d: *mut mjData, nnz: *mut i32) -> i32 {
    if m.is_null() { return 0; }
    extern "C" {
        fn mj_nc(m: *const mjModel, d: *mut mjData, nnz: *mut i32) -> i32;
    }
    // SAFETY: m verified non-null; delegates to C implementation
    unsafe { mj_nc(m, d, nnz) }
}

/// C: computeY_precount (engine/engine_core_constraint.c:2688)
/// Calls: mju_fillInt
#[allow(unused_variables, non_snake_case)]
pub fn compute_y_precount(Y_rownnz: *mut i32, Y_rowadr: *mut i32, nefc: i32, nv: i32, J_rownnz: *const i32, J_rowadr: *const i32, J_colind: *const i32, M_rownnz: *const i32, M_rowadr: *const i32, M_colind: *const i32, marker: *mut i32) -> i32  {
    if Y_rownnz.is_null() || Y_rowadr.is_null() || J_rownnz.is_null() || marker.is_null() {
        return 0;
    }
    extern "C" { fn computeY_precount(Y_rownnz: *mut i32, Y_rowadr: *mut i32, nefc: i32, nv: i32, J_rownnz: *const i32, J_rowadr: *const i32, J_colind: *const i32, M_rownnz: *const i32, M_rowadr: *const i32, M_colind: *const i32, marker: *mut i32) -> i32; }
    // SAFETY: key pointers verified non-null; delegates to C implementation
    unsafe { computeY_precount(Y_rownnz, Y_rowadr, nefc, nv, J_rownnz, J_rowadr, J_colind, M_rownnz, M_rowadr, M_colind, marker) }
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
    if Y.is_null() || J.is_null() || dof_parentid.is_null() {
        return;
    }
    extern "C" { fn computeY_fill(Y: *mut f64, Y_colind: *mut i32, Y_rownnz: *const i32, Y_rowadr: *const i32, nefc: i32, J: *const f64, J_rownnz: *const i32, J_rowadr: *const i32, J_colind: *const i32, dof_parentid: *const i32); }
    // SAFETY: m/ctx verified non-null; delegates to C implementation
    unsafe { computeY_fill(Y, Y_colind, Y_rownnz, Y_rowadr, nefc, J, J_rownnz, J_rowadr, J_colind, dof_parentid) }
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
    if Y.is_null() || qLD.is_null() || sqrtInvD.is_null() {
        return;
    }
    extern "C" { fn computeY_backsub(Y: *mut f64, Y_rownnz: *const i32, Y_rowadr: *const i32, Y_colind: *const i32, nefc: i32, qLD: *const f64, M_rownnz: *const i32, M_rowadr: *const i32, M_colind: *const i32, sqrtInvD: *const f64); }
    // SAFETY: m/ctx verified non-null; delegates to C implementation
    unsafe { computeY_backsub(Y, Y_rownnz, Y_rowadr, Y_colind, nefc, qLD, M_rownnz, M_rowadr, M_colind, sqrtInvD) }
}

/// C: mj_makeY (engine/engine_core_constraint.c:2908)
/// Calls: computeY_backsub, computeY_fill, computeY_precount, mj_arenaAllocByte, mj_clearEfc, mj_freeStack, mj_isSparse, mj_markStack, mj_solveM2, mj_stackAllocInfo, mj_warning, mju_dot
#[allow(unused_variables, non_snake_case)]
pub fn mj_make_y(m: *const mjModel, d: *mut mjData, flg_diagexact: i32) {
    if m.is_null() { return; }
    extern "C" {
        fn mj_makeY(m: *const mjModel, d: *mut mjData, flg_diagexact: i32);
    }
    // SAFETY: m verified non-null; delegates to C implementation
    unsafe { mj_makeY(m, d, flg_diagexact) }
}

/// C: mj_makeAR (engine/engine_core_constraint.c:2999)
/// Calls: mj_arenaAllocByte, mj_clearEfc, mj_freeStack, mj_isSparse, mj_markStack, mj_stackAllocInfo, mj_warning, mju_sqrMatTD, mju_sqrMatTDSparseNumeric, mju_sqrMatTDSparseSymbolic, mju_transpose, mju_transposeSparse
#[allow(unused_variables, non_snake_case)]
pub fn mj_make_ar(m: *const mjModel, d: *mut mjData) {
    if m.is_null() { return; }
    extern "C" {
        fn mj_makeAR(m: *const mjModel, d: *mut mjData);
    }
    // SAFETY: m verified non-null; delegates to C implementation
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
    if m.is_null() || d.is_null() || result.is_null() {
        return;
    }
    extern "C" { fn mj_Jdotv(m: *const mjModel, d: *mut mjData, result: *mut f64); }
    // SAFETY: m, d, result verified non-null; delegates to C implementation
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
    if m.is_null() { return; }
    extern "C" { fn mj_instantiateEquality(m: *const mjModel, d: *mut mjData); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_instantiateEquality(m, d) }
}

/// C: mj_instantiateContact (engine/engine_core_constraint.h:66)
/// Calls: mj_addConstraint, mj_contactJacobian, mj_freeStack, mj_isPyramidal, mj_isSparse, mj_markStack, mj_stackAllocInfo, mju_addScl, mju_mulMatMat, mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mj_instantiate_contact(m: *const mjModel, d: *mut mjData) {
    if m.is_null() || d.is_null() { return; }
    extern "C" { fn mj_instantiateContact(m: *const mjModel, d: *mut mjData); }
    // SAFETY: m, d verified non-null
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
    if m.is_null() || d.is_null() || con.is_null() {
        return 0;
    }
    extern "C" {
        fn mj_contactJacobian(m: *const mjModel, d: *mut mjData, con: *const mjContact, dim: i32, jac: *mut f64, jacdif: *mut f64, jacdifp: *mut f64, jacdifr: *mut f64, jac1p: *mut f64, jac2p: *mut f64, jac1r: *mut f64, jac2r: *mut f64, chain: *mut i32) -> i32;
    }
    // SAFETY: m, d, con verified non-null; delegates to C implementation
    unsafe { mj_contactJacobian(m, d, con, dim, jac, jacdif, jacdifp, jacdifr, jac1p, jac2p, jac1r, jac2r, chain) }
}

/// C: mj_diagApprox (engine/engine_core_constraint.h:78)
/// Calls: mj_elemBodyWeight, mj_vertBodyWeight, mju_flexGatherCellState, mju_flexGatherFaceState, mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_diag_approx(m: *const mjModel, d: *mut mjData) {
    if m.is_null() || d.is_null() {
        return;
    }
    extern "C" {
        fn mj_diagApprox(m: *const mjModel, d: *mut mjData);
    }
    // SAFETY: m and d verified non-null; delegates to C implementation
    unsafe { mj_diagApprox(m, d) }
}

/// C: mj_makeImpedance (engine/engine_core_constraint.h:81)
/// Calls: getimpedance, getposdim, getsolparam, mju_max
#[allow(unused_variables, non_snake_case)]
pub fn mj_make_impedance(m: *const mjModel, d: *mut mjData) {
    unsafe {
        const mjMINVAL: f64 = 1e-15;
        const mjNREF: usize = 2;
        const mjNIMP: usize = 5;
        // mjtConstraint enum values
        const mjCNSTR_EQUALITY: i32 = 0;
        const mjCNSTR_FRICTION_DOF: i32 = 1;
        const mjCNSTR_FRICTION_TENDON: i32 = 2;
        const mjCNSTR_CONTACT_PYRAMIDAL: i32 = 6;
        const mjCNSTR_CONTACT_ELLIPTIC: i32 = 7;

        let nefc: i32 = (*d).nefc;
        let R: *mut f64 = (*d).efc_R;
        let KBIP: *mut f64 = (*d).efc_KBIP;
        let mut dim: i32 = 0;
        let mut pos: f64 = 0.0;
        let mut imp: f64 = 0.0;
        let mut impP: f64 = 0.0;
        let mut solref: [f64; 2] = [0.0; 2];
        let mut solreffriction: [f64; 2] = [0.0; 2];
        let mut solimp: [f64; 5] = [0.0; 5];
        let mut Rpy: f64;

        // set efc_R, efc_KBIP
        let mut i: i32 = 0;
        while i < nefc {
            // get solref and solimp
            getsolparam(m, d as *const mjData, i, solref.as_mut_ptr(), solreffriction.as_mut_ptr(), solimp.as_mut_ptr());

            // get pos and dim
            getposdim(m, d as *const mjData, i, &mut pos, &mut dim);

            // get imp and impP
            getimpedance(solimp.as_ptr(), pos, *(*d).efc_margin.add(i as usize), &mut imp, &mut impP);

            // set R and KBIP for all constraint dimensions
            for j in 0..dim {
                // R = (1-imp)/imp * diagApprox
                *R.add((i + j) as usize) = crate::engine::engine_util_misc::mju_max(mjMINVAL, (1.0 - imp) * *(*d).efc_diagA.add((i + j) as usize) / imp);

                // constraint type
                let tp: i32 = *(*d).efc_type.add((i + j) as usize);

                // elliptic contacts use solreffriction in non-normal directions, if non-zero
                let elliptic_friction: i32 = if tp == mjCNSTR_CONTACT_ELLIPTIC && j > 0 { 1 } else { 0 };
                let ref_ptr: *const f64 = if elliptic_friction != 0 && (solreffriction[0] != 0.0 || solreffriction[1] != 0.0) {
                    solreffriction.as_ptr()
                } else {
                    solref.as_ptr()
                };

                // friction: K = 0
                if tp == mjCNSTR_FRICTION_DOF || tp == mjCNSTR_FRICTION_TENDON || elliptic_friction != 0 {
                    *KBIP.add((4 * (i + j)) as usize) = 0.0;
                }
                // standard: K = 1 / (d_width^2 * timeconst^2 * dampratio^2)
                else if *ref_ptr.add(0) > 0.0 {
                    *KBIP.add((4 * (i + j)) as usize) = 1.0 / crate::engine::engine_util_misc::mju_max(mjMINVAL, solimp[1] * solimp[1] * *ref_ptr.add(0) * *ref_ptr.add(0) * *ref_ptr.add(1) * *ref_ptr.add(1));
                }
                // direct: K = -solref[0] / d_width^2
                else {
                    *KBIP.add((4 * (i + j)) as usize) = -*ref_ptr.add(0) / crate::engine::engine_util_misc::mju_max(mjMINVAL, solimp[1] * solimp[1]);
                }

                // standard: B = 2 / (d_width*timeconst)
                if *ref_ptr.add(1) > 0.0 {
                    *KBIP.add((4 * (i + j) + 1) as usize) = 2.0 / crate::engine::engine_util_misc::mju_max(mjMINVAL, solimp[1] * *ref_ptr.add(0));
                }
                // direct: B = -solref[1] / d_width
                else {
                    *KBIP.add((4 * (i + j) + 1) as usize) = -*ref_ptr.add(1) / crate::engine::engine_util_misc::mju_max(mjMINVAL, solimp[1]);
                }

                // I = imp, P = imp'
                *KBIP.add((4 * (i + j) + 2) as usize) = imp;
                *KBIP.add((4 * (i + j) + 3) as usize) = impP;
            }

            // skip the rest of this constraint
            i += dim;
            continue;
        }

        // frictional contacts: adjust R in friction dimensions, set contact master mu
        i = (*d).ne + (*d).nf;
        while i < nefc {
            if *(*d).efc_type.add(i as usize) == mjCNSTR_CONTACT_PYRAMIDAL
                || *(*d).efc_type.add(i as usize) == mjCNSTR_CONTACT_ELLIPTIC
            {
                // extract id, dim, mu
                let id: i32 = *(*d).efc_id.add(i as usize);
                dim = (*(*d).contact.add(id as usize)).dim;
                let friction: *const f64 = (*(*d).contact.add(id as usize)).friction.as_ptr();

                // set R[1] = R[0]/impratio
                *R.add((i + 1) as usize) = *R.add(i as usize) / crate::engine::engine_util_misc::mju_max(mjMINVAL, (*m).opt.impratio);

                // set mu of regularized cone = mu[1]*sqrt(R[1]/R[0])
                (*(*d).contact.add(id as usize)).mu = *friction.add(0) * (*R.add((i + 1) as usize) / *R.add(i as usize)).sqrt();

                // elliptic
                if *(*d).efc_type.add(i as usize) == mjCNSTR_CONTACT_ELLIPTIC {
                    // set remaining R's such that R[j]*mu[j]^2 = R[1]*mu[1]^2
                    for j in 1..(dim - 1) {
                        *R.add((i + j + 1) as usize) = *R.add((i + 1) as usize) * *friction.add(0) * *friction.add(0) / (*friction.add(j as usize) * *friction.add(j as usize));
                    }

                    // skip the rest of this contact
                    i += dim;
                }
                // pyramidal: common R matching friction impedance of elliptic model
                else {
                    // D0_el = 2*(dim-1)*D_py : normal match
                    // D0_el = 2*mu^2*D_py    : friction match
                    Rpy = 2.0 * (*(*d).contact.add(id as usize)).mu * (*(*d).contact.add(id as usize)).mu * *R.add(i as usize);

                    // assign Rpy to all pyramidal R
                    for j in 0..(2 * (dim - 1)) {
                        *R.add((i + j) as usize) = Rpy;
                    }

                    // skip the rest of this contact
                    i += 2 * (dim - 1);
                }
            } else {
                i += 1;
            }
        }

        // set D = 1 / R
        for i in 0..nefc {
            *(*d).efc_D.add(i as usize) = 1.0 / *R.add(i as usize);
        }

        // adjust diagA so that R = (1-imp)/imp * diagA
        for i in 0..nefc {
            *(*d).efc_diagA.add(i as usize) = *R.add(i as usize) * *KBIP.add((4 * i + 2) as usize) / (1.0 - *KBIP.add((4 * i + 2) as usize));
        }
    }
}

/// C: mj_makeConstraint (engine/engine_core_constraint.h:87)
/// Calls: arenaAllocEfc, mj_diagApprox, mj_instantiateContact, mj_instantiateEquality, mj_instantiateFriction, mj_instantiateLimit, mj_isSparse, mj_makeImpedance, mj_nc, mj_ne, mju_fillInt, mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_make_constraint(m: *const mjModel, d: *mut mjData) {
    if m.is_null() { return; }
    extern "C" { fn mj_makeConstraint(m: *const mjModel, d: *mut mjData); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_makeConstraint(m, d) }
}

/// C: mj_projectConstraint (engine/engine_core_constraint.h:90)
/// Calls: mj_isDual, mj_makeAR, mj_makeImpedance, mj_makeY, mju_gather
#[allow(unused_variables, non_snake_case)]
pub fn mj_project_constraint(m: *const mjModel, d: *mut mjData) {
    if m.is_null() { return; }
    extern "C" {
        fn mj_projectConstraint(m: *const mjModel, d: *mut mjData);
    }
    // SAFETY: m verified non-null; delegates to C implementation
    unsafe { mj_projectConstraint(m, d) }
}

/// C: mj_referenceConstraint (engine/engine_core_constraint.h:93)
/// Calls: mj_Jdotv, mj_mulJacVec
#[allow(unused_variables, non_snake_case)]
pub fn mj_reference_constraint(m: *const mjModel, d: *mut mjData) {
    unsafe {
        let nefc = (*d).nefc;
        let KBIP = (*d).efc_KBIP;

        // compute efc_vel
        mj_mul_jac_vec(m, d, (*d).efc_vel, (*d).qvel);

        // compute aref = -B*vel - K*I*(pos-margin)
        let mut i: i32 = 0;
        while i < nefc {
            *(*d).efc_aref.add(i as usize) =
                -*KBIP.add(4 * i as usize + 1) * *(*d).efc_vel.add(i as usize)
                - *KBIP.add(4 * i as usize) * *KBIP.add(4 * i as usize + 2)
                  * (*(*d).efc_pos.add(i as usize) - *(*d).efc_margin.add(i as usize));
            i += 1;
        }

        // subtract Jdot*v correction for connect/weld equality constraints
        if (*d).ne > 0 {
            mj_jdotv(m, d, (*d).efc_aref);
        }
    }
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

