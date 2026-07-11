//! Port of: engine/engine_core_constraint.c
//! IR hash: c6d98e4f4b63b7f2
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
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, flex_id : i32, npc : i32, gindices : * const i32, nv : i32, xpos_c : * const f64, cell_chain : * mut i32, cell_nnz : * mut i32)
    // Previous return: * mut f64
    todo ! ()
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
    todo ! ()
}

/// C: arenaAllocEfc (engine/engine_core_constraint.c:130)
/// Calls: mj_arenaAllocByte, mj_clearEfc, mj_warning
#[allow(unused_variables, non_snake_case)]
pub fn arena_alloc_efc(m: *const mjModel, d: *mut mjData) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData)
    // Previous return: i32
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * const mjData, f : i32, e : i32, v : i32, point : * const f64, body : * mut i32, weight : * mut f64)
    // Previous return: i32
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * const mjData, f : i32, v : * mut i32, body : * mut i32, bweight : * mut f64, vweight : * const f64, nw : i32)
    // Previous return: i32
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, jac : * const f64, pos : * const f64, margin : * const f64, frictionloss : f64, size : i32, r#type : i32, id : i32, NV : i32, chain : * const i32)
    // Previous return: ()
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * const mjData, eq_id : i32, pos1 : * mut f64, pos2 : * mut f64, body1 : * mut i32, body2 : * mut i32)
    // Previous return: ()
    todo ! ()
}

/// C: mj_addConstraintCount (engine/engine_core_constraint.c:1259)
/// Calls: mj_isSparse
#[allow(unused_variables, non_snake_case)]
pub fn mj_add_constraint_count(m: *const mjModel, size: i32, NV: i32) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, size : i32, NV : i32)
    // Previous return: i32
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * const mjData, i : i32, solref : * mut f64, solreffriction : * mut f64, solimp : * mut f64)
    // Previous return: ()
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * const mjData, i : i32, pos : * mut f64, dim : * mut i32)
    // Previous return: ()
    todo ! ()
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
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (solimp : * const f64, pos : f64, margin : f64, imp : * mut f64, impP : * mut f64)
    // Previous return: ()
    todo ! ()
}

/// C: mj_jacSumCount (engine/engine_core_constraint.c:2272)
/// Calls: mj_bodyChain, mj_freeStack, mj_markStack, mj_stackAllocInfo, mju_addChains, mju_copyInt
#[allow(unused_variables, non_snake_case)]
pub fn mj_jac_sum_count(m: *const mjModel, d: *mut mjData, chain: *mut i32, n: i32, body: *const i32) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, chain : * mut i32, n : i32, body : * const i32)
    // Previous return: i32
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, nnz : * mut i32)
    // Previous return: i32
    todo ! ()
}

/// C: computeY_precount (engine/engine_core_constraint.c:2688)
/// Calls: mju_fillInt
#[allow(unused_variables, non_snake_case)]
pub fn compute_y_precount(Y_rownnz: *mut i32, Y_rowadr: *mut i32, nefc: i32, nv: i32, J_rownnz: *const i32, J_rowadr: *const i32, J_colind: *const i32, M_rownnz: *const i32, M_rowadr: *const i32, M_colind: *const i32, marker: *mut i32) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (Y_rownnz : * mut i32, Y_rowadr : * mut i32, nefc : i32, nv : i32, J_rownnz : * const i32, J_rowadr : * const i32, J_colind : * const i32, M_rownnz : * const i32, M_rowadr : * const i32, M_colind : * const i32, marker : * mut i32)
    // Previous return: i32
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (Y : * mut f64, Y_colind : * mut i32, Y_rownnz : * const i32, Y_rowadr : * const i32, nefc : i32, J : * const f64, J_rownnz : * const i32, J_rowadr : * const i32, J_colind : * const i32, dof_parentid : * const i32)
    // Previous return: ()
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (Y : * mut f64, Y_rownnz : * const i32, Y_rowadr : * const i32, Y_colind : * const i32, nefc : i32, qLD : * const f64, M_rownnz : * const i32, M_rowadr : * const i32, M_colind : * const i32, sqrtInvD : * const f64)
    // Previous return: ()
    todo ! ()
}

/// C: mj_makeY (engine/engine_core_constraint.c:2908)
/// Calls: computeY_backsub, computeY_fill, computeY_precount, mj_arenaAllocByte, mj_clearEfc, mj_freeStack, mj_isSparse, mj_markStack, mj_solveM2, mj_stackAllocInfo, mj_warning, mju_dot
#[allow(unused_variables, non_snake_case)]
pub fn mj_make_y(m: *const mjModel, d: *mut mjData, flg_diagexact: i32) {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, flg_diagexact : i32)
    // Previous return: ()
    todo ! ()
}

/// C: mj_makeAR (engine/engine_core_constraint.c:2999)
/// Calls: mj_arenaAllocByte, mj_clearEfc, mj_freeStack, mj_isSparse, mj_markStack, mj_stackAllocInfo, mj_warning, mju_sqrMatTD, mju_sqrMatTDSparseNumeric, mju_sqrMatTDSparseSymbolic, mju_transpose, mju_transposeSparse
#[allow(unused_variables, non_snake_case)]
pub fn mj_make_ar(m: *const mjModel, d: *mut mjData) {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData)
    // Previous return: ()
    todo ! ()
}

/// C: mj_isDual (engine/engine_core_constraint.h:31)
#[allow(unused_variables, non_snake_case)]
pub fn mj_is_dual(m: *const mjModel) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel)
    // Previous return: i32
    todo ! ()
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
    todo ! ()
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
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, result : * mut f64)
    // Previous return: ()
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, target : * mut f64, source : * const f64)
    // Previous return: ()
    todo ! ()
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
    todo ! ()
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
    todo ! ()
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
    todo ! ()
}

/// C: mj_addContact (engine/engine_core_constraint.h:58)
/// Calls: mj_arenaAllocByte, mj_clearEfc, mj_warning
#[allow(unused_variables, non_snake_case)]
pub fn mj_add_contact(m: *const mjModel, d: *mut mjData, con: *const mjContact) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, con : * const mjContact)
    // Previous return: i32
    todo ! ()
}

/// C: mj_instantiateEquality (engine/engine_core_constraint.h:63)
/// Calls: cell_pos_and_jac, cell_strain_jacobian, mj_addConstraint, mj_equalityAnchors, mj_freeStack, mj_isSparse, mj_jacDifPair, mj_markStack, mj_sleepState, mj_stackAllocInfo, mju_addTo3, mju_addToScl, mju_combineSparse, mju_copy, mju_copy3, mju_copyInt, mju_defGradient, mju_flexGatherCellState, mju_flexGatherFaceState, mju_flexInterpRotation2D, mju_mat2Rot, mju_message, mju_mulMatVec3, mju_mulQuat, mju_mulQuatAxis, mju_negQuat, mju_rotVecQuat, mju_scl, mju_scl3, mju_sparse2dense, mju_sub3, mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mj_instantiate_equality(m: *const mjModel, d: *mut mjData) {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData)
    // Previous return: ()
    todo ! ()
}

/// C: mj_instantiateContact (engine/engine_core_constraint.h:66)
/// Calls: mj_addConstraint, mj_contactJacobian, mj_freeStack, mj_isPyramidal, mj_isSparse, mj_markStack, mj_stackAllocInfo, mju_addScl, mju_mulMatMat, mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mj_instantiate_contact(m: *const mjModel, d: *mut mjData) {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData)
    // Previous return: ()
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, con : * const mjContact, dim : i32, jac : * mut f64, jacdif : * mut f64, jacdifp : * mut f64, jacdifr : * mut f64, jac1p : * mut f64, jac2p : * mut f64, jac1r : * mut f64, jac2r : * mut f64, chain : * mut i32)
    // Previous return: i32
    todo ! ()
}

/// C: mj_diagApprox (engine/engine_core_constraint.h:78)
/// Calls: mj_elemBodyWeight, mj_vertBodyWeight, mju_flexGatherCellState, mju_flexGatherFaceState, mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_diag_approx(m: *const mjModel, d: *mut mjData) {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData)
    // Previous return: ()
    todo ! ()
}

/// C: mj_makeImpedance (engine/engine_core_constraint.h:81)
/// Calls: getimpedance, getposdim, getsolparam, mju_max
#[allow(unused_variables, non_snake_case)]
pub fn mj_make_impedance(m: *const mjModel, d: *mut mjData) {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData)
    // Previous return: ()
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData)
    // Previous return: ()
    todo ! ()
}

/// C: mj_referenceConstraint (engine/engine_core_constraint.h:93)
/// Calls: mj_Jdotv, mj_mulJacVec
#[allow(unused_variables, non_snake_case)]
pub fn mj_reference_constraint(m: *const mjModel, d: *mut mjData) {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData)
    // Previous return: ()
    todo ! ()
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
    use crate::engine::engine_util_blas::{mju_norm, mju_zero};

    const MJCNSTRSTATE_SATISFIED: i32 = 0;
    const MJCNSTRSTATE_QUADRATIC: i32 = 1;
    const MJCNSTRSTATE_LINEARNEG: i32 = 2;
    const MJCNSTRSTATE_LINEARPOS: i32 = 3;
    const MJCNSTRSTATE_CONE: i32 = 4;
    const MJCNSTR_CONTACT_ELLIPTIC: i32 = 7;

    // SAFETY: caller guarantees all pointers are valid
    unsafe {
        let mut s: f64 = 0.0;

        // no constraints: clear cost, return
        if nefc == 0 {
            if !cost.is_null() {
                *cost = 0.0;
            }
            return;
        }

        // compute unconstrained efc_force
        for i in 0..nefc as usize {
            *force.add(i) = -*D.add(i) * *jar.add(i);
        }

        // update constraints
        let mut i: usize = 0;
        while i < nefc as usize {
            // ==== equality
            if (i as i32) < ne {
                if !cost.is_null() {
                    s += 0.5 * *D.add(i) * *jar.add(i) * *jar.add(i);
                }
                *state.add(i) = MJCNSTRSTATE_QUADRATIC;
                i += 1;
                continue;
            }

            // ==== friction
            if (i as i32) < ne + nf {
                // linear negative
                if *jar.add(i) <= -*R.add(i) * *floss.add(i) {
                    if !cost.is_null() {
                        s += -0.5 * *R.add(i) * *floss.add(i) * *floss.add(i) - *floss.add(i) * *jar.add(i);
                    }
                    *force.add(i) = *floss.add(i);
                    *state.add(i) = MJCNSTRSTATE_LINEARNEG;
                }
                // linear positive
                else if *jar.add(i) >= *R.add(i) * *floss.add(i) {
                    if !cost.is_null() {
                        s += -0.5 * *R.add(i) * *floss.add(i) * *floss.add(i) + *floss.add(i) * *jar.add(i);
                    }
                    *force.add(i) = -*floss.add(i);
                    *state.add(i) = MJCNSTRSTATE_LINEARPOS;
                }
                // quadratic
                else {
                    if !cost.is_null() {
                        s += 0.5 * *D.add(i) * *jar.add(i) * *jar.add(i);
                    }
                    *state.add(i) = MJCNSTRSTATE_QUADRATIC;
                }
                i += 1;
                continue;
            }

            // ==== contact

            // non-negative constraint
            if *r#type.add(i) != MJCNSTR_CONTACT_ELLIPTIC {
                // constraint is satisfied: no cost
                if *jar.add(i) >= 0.0 {
                    *force.add(i) = 0.0;
                    *state.add(i) = MJCNSTRSTATE_SATISFIED;
                }
                // quadratic
                else {
                    if !cost.is_null() {
                        s += 0.5 * *D.add(i) * *jar.add(i) * *jar.add(i);
                    }
                    *state.add(i) = MJCNSTRSTATE_QUADRATIC;
                }
            }
            // contact with elliptic cone
            else {
                // get contact
                let con = &mut *contact.add(*id.add(i) as usize);
                let mu = con.mu;
                let friction = con.friction.as_ptr();
                let dim = con.dim as usize;

                // map to regular dual cone space
                let mut U = [0.0f64; 6];
                U[0] = *jar.add(i) * mu;
                for j in 1..dim {
                    U[j] = *jar.add(i + j) * *friction.add(j - 1);
                }

                // decompose into normal and tangent
                let N = U[0];
                let T = mju_norm(U.as_ptr().add(1), (dim - 1) as i32);

                // top zone
                if N >= mu * T || (T <= 0.0 && N >= 0.0) {
                    mju_zero(force.add(i), dim as i32);
                    *state.add(i) = MJCNSTRSTATE_SATISFIED;
                }
                // bottom zone
                else if mu * N + T <= 0.0 || (T <= 0.0 && N < 0.0) {
                    if !cost.is_null() {
                        for j in 0..dim {
                            s += 0.5 * *D.add(i + j) * *jar.add(i + j) * *jar.add(i + j);
                        }
                    }
                    *state.add(i) = MJCNSTRSTATE_QUADRATIC;
                }
                // middle zone
                else {
                    // cost: 0.5*D0/(mu*mu*(1+mu*mu))*(N-mu*T)^2
                    let Dm = *D.add(i) / (mu * mu * (1.0 + mu * mu));
                    let NmT = N - mu * T;

                    if !cost.is_null() {
                        s += 0.5 * Dm * NmT * NmT;
                    }

                    // force: - ds/djar = dU/djar * ds/dU
                    *force.add(i) = -Dm * NmT * mu;
                    for j in 1..dim {
                        *force.add(i + j) = -*force.add(i) / T * U[j] * *friction.add(j - 1);
                    }

                    // set state
                    *state.add(i) = MJCNSTRSTATE_CONE;

                    // cone Hessian
                    if flg_coneHessian != 0 {
                        let H = (*contact.add(*id.add(i) as usize)).H.as_mut_ptr();

                        // set first row: (1, -mu/T * U)
                        let mut scl = -mu / T;
                        *H.add(0) = 1.0;
                        for j in 1..dim {
                            *H.add(j) = scl * U[j];
                        }

                        // set upper block: mu*N/T^3 * U*U'
                        scl = mu * N / (T * T * T);
                        for k in 1..dim {
                            for j in k..dim {
                                *H.add(k * dim + j) = scl * U[j] * U[k];
                            }
                        }

                        // add to diagonal: (mu^2 - mu*N/T) * I
                        scl = mu * mu - mu * N / T;
                        for j in 1..dim {
                            *H.add(j * (dim + 1)) += scl;
                        }

                        // pre and post multiply by diag(mu, friction), scale by Dm
                        for k in 0..dim {
                            scl = Dm * (if k == 0 { mu } else { *friction.add(k - 1) });
                            for j in k..dim {
                                *H.add(k * dim + j) *= scl * (if j == 0 { mu } else { *friction.add(j - 1) });
                            }
                        }

                        // make symmetric: copy upper into lower
                        for k in 0..dim {
                            for j in (k + 1)..dim {
                                *H.add(j * dim + k) = *H.add(k * dim + j);
                            }
                        }
                    }
                }

                // replicate state in all cone dimensions
                for j in 1..dim {
                    *state.add(i + j) = *state.add(i);
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
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, jar : * const f64, cost : * mut f64, flg_coneHessian : i32)
    // Previous return: ()
    todo ! ()
}

