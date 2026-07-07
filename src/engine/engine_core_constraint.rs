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
        fn cell_pos_and_jac_impl(m: *const mjModel, d: *mut mjData, flex_id: i32, npc: i32, gindices: *const i32, nv: i32, xpos_c: *const f64, cell_chain: *mut i32, cell_nnz: *mut i32) -> *mut f64;
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { cell_pos_and_jac_impl(m, d, flex_id, npc, gindices, nv, xpos_c, cell_chain, cell_nnz) }
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
        fn arenaAllocEfc_impl(m: *const mjModel, d: *mut mjData) -> i32;
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { arenaAllocEfc_impl(m, d) }
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
        fn mj_elemBodyWeight_impl(m: *const mjModel, d: *const mjData, f: i32, e: i32, v: i32, point: *const f64, body: *mut i32, weight: *mut f64) -> i32;
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_elemBodyWeight_impl(m, d, f, e, v, point, body, weight) }
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
        fn mj_vertBodyWeight_impl(m: *const mjModel, d: *const mjData, f: i32, v: *mut i32, body: *mut i32, bweight: *mut f64, vweight: *const f64, nw: i32) -> i32;
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_vertBodyWeight_impl(m, d, f, v, body, bweight, vweight, nw) }
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
        fn mj_addConstraint_impl(m: *const mjModel, d: *mut mjData, jac: *const f64, pos: *const f64, margin: *const f64, frictionloss: f64, size: i32, r#type: i32, id: i32, NV: i32, chain: *const i32);
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_addConstraint_impl(m, d, jac, pos, margin, frictionloss, size, r#type, id, NV, chain) }
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
        fn mj_equalityAnchors_impl(m: *const mjModel, d: *const mjData, eq_id: i32, pos1: *mut f64, pos2: *mut f64, body1: *mut i32, body2: *mut i32);
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_equalityAnchors_impl(m, d, eq_id, pos1, pos2, body1, body2) }
}

/// C: mj_addConstraintCount (engine/engine_core_constraint.c:1259)
/// Calls: mj_isSparse
#[allow(unused_variables, non_snake_case)]
pub fn mj_add_constraint_count(m: *const mjModel, size: i32, NV: i32) -> i32 {
    extern "C" {
        fn mj_addConstraintCount_impl(m: *const mjModel, size: i32, NV: i32) -> i32;
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_addConstraintCount_impl(m, size, NV) }
}

/// C: mj_instantiateFriction (engine/engine_core_constraint.c:1270)
/// Calls: mj_addConstraint, mj_addConstraintCount, mj_freeStack, mj_isSparse, mj_markStack, mj_sleepState, mj_stackAllocInfo, mju_sparse2dense, mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mj_instantiate_friction(m: *const mjModel, d: *mut mjData, count_only: i32, nnz: *mut i32) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, count_only : i32, nnz : * mut i32)
    // Previous return: i32
    todo ! ()
}

/// C: mj_instantiateLimit (engine/engine_core_constraint.c:1360)
/// Calls: mj_addConstraint, mj_addConstraintCount, mj_freeStack, mj_isSparse, mj_markStack, mj_sleepState, mj_stackAllocInfo, mju_max, mju_normalize3, mju_normalize4, mju_quat2Vel, mju_scl, mju_scl3, mju_sparse2dense, mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mj_instantiate_limit(m: *const mjModel, d: *mut mjData, count_only: i32, nnz: *mut i32) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, count_only : i32, nnz : * mut i32)
    // Previous return: i32
    todo ! ()
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
        fn getsolparam_impl(m: *const mjModel, d: *const mjData, i: i32, solref: *mut f64, solreffriction: *mut f64, solimp: *mut f64);
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { getsolparam_impl(m, d, i, solref, solreffriction, solimp) }
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
        fn getposdim_impl(m: *const mjModel, d: *const mjData, i: i32, pos: *mut f64, dim: *mut i32);
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { getposdim_impl(m, d, i, pos, dim) }
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
    extern "C" {
        fn getimpedance_impl(solimp: *const f64, pos: f64, margin: f64, imp: *mut f64, impP: *mut f64);
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { getimpedance_impl(solimp, pos, margin, imp, impP) }
}

/// C: mj_jacSumCount (engine/engine_core_constraint.c:2272)
/// Calls: mj_bodyChain, mj_freeStack, mj_markStack, mj_stackAllocInfo, mju_addChains, mju_copyInt
#[allow(unused_variables, non_snake_case)]
pub fn mj_jac_sum_count(m: *const mjModel, d: *mut mjData, chain: *mut i32, n: i32, body: *const i32) -> i32 {
    extern "C" {
        fn mj_jacSumCount_impl(m: *const mjModel, d: *mut mjData, chain: *mut i32, n: i32, body: *const i32) -> i32;
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_jacSumCount_impl(m, d, chain, n, body) }
}

/// C: mj_ne (engine/engine_core_constraint.c:2303)
/// Calls: mj_addConstraintCount, mj_freeStack, mj_jacDifPair, mj_jacSumCount, mj_markStack, mj_sleepState, mj_stackAllocInfo, mju_combineSparseCount, mju_copyInt, mju_flexGatherCellState, mju_flexGatherFaceState, mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_ne(m: *const mjModel, d: *mut mjData, nnz: *mut i32) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, nnz : * mut i32)
    // Previous return: i32
    todo ! ()
}

/// C: mj_nc (engine/engine_core_constraint.c:2536)
/// Calls: mj_elemBodyWeight, mj_flexBody, mj_freeStack, mj_isPyramidal, mj_isSparse, mj_jacDifPair, mj_jacSumCount, mj_markStack, mj_stackAllocInfo, mj_vertBodyWeight, mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_nc(m: *const mjModel, d: *mut mjData, nnz: *mut i32) -> i32 {
    extern "C" {
        fn mj_nc_impl(m: *const mjModel, d: *mut mjData, nnz: *mut i32) -> i32;
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_nc_impl(m, d, nnz) }
}

/// C: computeY_precount (engine/engine_core_constraint.c:2688)
/// Calls: mju_fillInt
#[allow(unused_variables, non_snake_case)]
pub fn compute_y_precount(Y_rownnz: *mut i32, Y_rowadr: *mut i32, nefc: i32, nv: i32, J_rownnz: *const i32, J_rowadr: *const i32, J_colind: *const i32, M_rownnz: *const i32, M_rowadr: *const i32, M_colind: *const i32, marker: *mut i32) -> i32 {
    extern "C" {
        fn computeY_precount_impl(Y_rownnz: *mut i32, Y_rowadr: *mut i32, nefc: i32, nv: i32, J_rownnz: *const i32, J_rowadr: *const i32, J_colind: *const i32, M_rownnz: *const i32, M_rowadr: *const i32, M_colind: *const i32, marker: *mut i32) -> i32;
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { computeY_precount_impl(Y_rownnz, Y_rowadr, nefc, nv, J_rownnz, J_rowadr, J_colind, M_rownnz, M_rowadr, M_colind, marker) }
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
    extern "C" {
        fn computeY_fill_impl(Y: *mut f64, Y_colind: *mut i32, Y_rownnz: *const i32, Y_rowadr: *const i32, nefc: i32, J: *const f64, J_rownnz: *const i32, J_rowadr: *const i32, J_colind: *const i32, dof_parentid: *const i32);
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { computeY_fill_impl(Y, Y_colind, Y_rownnz, Y_rowadr, nefc, J, J_rownnz, J_rowadr, J_colind, dof_parentid) }
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
    extern "C" {
        fn computeY_backsub_impl(Y: *mut f64, Y_rownnz: *const i32, Y_rowadr: *const i32, Y_colind: *const i32, nefc: i32, qLD: *const f64, M_rownnz: *const i32, M_rowadr: *const i32, M_colind: *const i32, sqrtInvD: *const f64);
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { computeY_backsub_impl(Y, Y_rownnz, Y_rowadr, Y_colind, nefc, qLD, M_rownnz, M_rowadr, M_colind, sqrtInvD) }
}

/// C: mj_makeY (engine/engine_core_constraint.c:2908)
/// Calls: computeY_backsub, computeY_fill, computeY_precount, mj_arenaAllocByte, mj_clearEfc, mj_freeStack, mj_isSparse, mj_markStack, mj_solveM2, mj_stackAllocInfo, mj_warning, mju_dot
#[allow(unused_variables, non_snake_case)]
pub fn mj_make_y(m: *const mjModel, d: *mut mjData, flg_diagexact: i32) {
    extern "C" {
        fn mj_makeY_impl(m: *const mjModel, d: *mut mjData, flg_diagexact: i32);
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_makeY_impl(m, d, flg_diagexact) }
}

/// C: mj_makeAR (engine/engine_core_constraint.c:2999)
/// Calls: mj_arenaAllocByte, mj_clearEfc, mj_freeStack, mj_isSparse, mj_markStack, mj_stackAllocInfo, mj_warning, mju_sqrMatTD, mju_sqrMatTDSparseNumeric, mju_sqrMatTDSparseSymbolic, mju_transpose, mju_transposeSparse
#[allow(unused_variables, non_snake_case)]
pub fn mj_make_ar(m: *const mjModel, d: *mut mjData) {
    extern "C" {
        fn mj_makeAR_impl(m: *const mjModel, d: *mut mjData);
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_makeAR_impl(m, d) }
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
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * const mjData, res : * mut f64, vec : * const f64)
    // Previous return: ()
    extern "C" { fn mj_mulJacVec_impl (m : * const mjModel , d : * const mjData , res : * mut f64 , vec : * const f64) ; } unsafe { mj_mulJacVec_impl (m , d , res , vec) }
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
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * const mjData, res : * mut f64, vec : * const f64)
    // Previous return: ()
    extern "C" { fn mj_mulJacTVec_impl (m : * const mjModel , d : * const mjData , res : * mut f64 , vec : * const f64) ; } unsafe { mj_mulJacTVec_impl (m , d , res , vec) }
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
    extern "C" { fn mj_Jdotv_impl(m: *const mjModel, d: *mut mjData, result: *mut f64); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_Jdotv_impl(m, d, result) }
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
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, target : * mut f64, source : * const f64)
    // Previous return: ()
    extern "C" { fn mj_assignImp_impl (m : * const mjModel , target : * mut f64 , source : * const f64) ; } unsafe { mj_assignImp_impl (m , target , source) }
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
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, target : * mut f64, source : * const f64)
    // Previous return: ()
    extern "C" { fn mj_assignFriction_impl (m : * const mjModel , target : * mut f64 , source : * const f64) ; } unsafe { mj_assignFriction_impl (m , target , source) }
}

/// C: mj_assignMargin (engine/engine_core_constraint.h:55)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_assign_margin(m: *const mjModel, source: f64) -> f64 {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, source : f64)
    // Previous return: f64
    extern "C" { fn mj_assignMargin_impl (m : * const mjModel , source : f64) -> f64 ; } unsafe { mj_assignMargin_impl (m , source) }
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
    extern "C" { fn mj_instantiateEquality_impl(m: *const mjModel, d: *mut mjData); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_instantiateEquality_impl(m, d) }
}

/// C: mj_instantiateContact (engine/engine_core_constraint.h:66)
/// Calls: mj_addConstraint, mj_contactJacobian, mj_freeStack, mj_isPyramidal, mj_isSparse, mj_markStack, mj_stackAllocInfo, mju_addScl, mju_mulMatMat, mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mj_instantiate_contact(m: *const mjModel, d: *mut mjData) {
    extern "C" { fn mj_instantiateContact_impl(m: *const mjModel, d: *mut mjData); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_instantiateContact_impl(m, d) }
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
        fn mj_contactJacobian_impl(m: *const mjModel, d: *mut mjData, con: *const mjContact, dim: i32, jac: *mut f64, jacdif: *mut f64, jacdifp: *mut f64, jacdifr: *mut f64, jac1p: *mut f64, jac2p: *mut f64, jac1r: *mut f64, jac2r: *mut f64, chain: *mut i32) -> i32;
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_contactJacobian_impl(m, d, con, dim, jac, jacdif, jacdifp, jacdifr, jac1p, jac2p, jac1r, jac2r, chain) }
}

/// C: mj_diagApprox (engine/engine_core_constraint.h:78)
/// Calls: mj_elemBodyWeight, mj_vertBodyWeight, mju_flexGatherCellState, mju_flexGatherFaceState, mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_diag_approx(m: *const mjModel, d: *mut mjData) {
    extern "C" {
        fn mj_diagApprox_impl(m: *const mjModel, d: *mut mjData);
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_diagApprox_impl(m, d) }
}

/// C: mj_makeImpedance (engine/engine_core_constraint.h:81)
/// Calls: getimpedance, getposdim, getsolparam, mju_max
#[allow(unused_variables, non_snake_case)]
pub fn mj_make_impedance(m: *const mjModel, d: *mut mjData) {
    extern "C" {
        fn mj_makeImpedance_impl(m: *const mjModel, d: *mut mjData);
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_makeImpedance_impl(m, d) }
}

/// C: mj_makeConstraint (engine/engine_core_constraint.h:87)
/// Calls: arenaAllocEfc, mj_diagApprox, mj_instantiateContact, mj_instantiateEquality, mj_instantiateFriction, mj_instantiateLimit, mj_isSparse, mj_makeImpedance, mj_nc, mj_ne, mju_fillInt, mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_make_constraint(m: *const mjModel, d: *mut mjData) {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData)
    // Previous return: ()
    todo ! ()
}

/// C: mj_projectConstraint (engine/engine_core_constraint.h:90)
/// Calls: mj_isDual, mj_makeAR, mj_makeImpedance, mj_makeY, mju_gather
#[allow(unused_variables, non_snake_case)]
pub fn mj_project_constraint(m: *const mjModel, d: *mut mjData) {
    extern "C" {
        fn mj_projectConstraint_impl(m: *const mjModel, d: *mut mjData);
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_projectConstraint_impl(m, d) }
}

/// C: mj_referenceConstraint (engine/engine_core_constraint.h:93)
/// Calls: mj_Jdotv, mj_mulJacVec
#[allow(unused_variables, non_snake_case)]
pub fn mj_reference_constraint(m: *const mjModel, d: *mut mjData) {
    extern "C" { fn mj_referenceConstraint_impl(m: *const mjModel, d: *mut mjData); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_referenceConstraint_impl(m, d) }
}

/// C: mj_constraintUpdate_impl (engine/engine_core_constraint.h:97)
/// Calls: mju_norm, mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_constraint_update_impl(ne: i32, nf: i32, nefc: i32, D: *const f64, R: *const f64, floss: *const f64, jar: *const f64, r#type: *const i32, id: *const i32, contact: *mut mjContact, state: *mut i32, force: *mut f64, cost: *mut f64, flg_coneHessian: i32) {
    // WARNING: signature changed — verify body
    // Previous params: (ne : i32, nf : i32, nefc : i32, D : * const f64, R : * const f64, floss : * const f64, jar : * const f64, r#type : * const i32, id : * const i32, contact : * mut mjContact, state : * mut i32, force : * mut f64, cost : * mut f64, flg_coneHessian : i32)
    // Previous return: ()
    extern "C" { fn mj_constraintUpdate_impl_impl (ne : i32 , nf : i32 , nefc : i32 , D : * const f64 , R : * const f64 , floss : * const f64 , jar : * const f64 , r#type : * const i32 , id : * const i32 , contact : * mut mjContact , state : * mut i32 , force : * mut f64 , cost : * mut f64 , flg_coneHessian : i32) ; } unsafe { mj_constraintUpdate_impl_impl (ne , nf , nefc , D , R , floss , jar , r#type , id , contact , state , force , cost , flg_coneHessian) }
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
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, jar : * const f64, cost : * mut f64, flg_coneHessian : i32)
    // Previous return: ()
    extern "C" { fn mj_constraintUpdate_impl (m : * const mjModel , d : * mut mjData , jar : * const f64 , cost : * mut f64 , flg_coneHessian : i32) ; } unsafe { mj_constraintUpdate_impl (m , d , jar , cost , flg_coneHessian) }
}

