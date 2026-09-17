//! Port of: Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_constraint.c
//! IR hash: 1fa44043fe383ce9
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: cell_pos_and_jac (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_constraint.c:55)
/// Calls: cxx:_mj_bodyChain, cxx:_mj_jacSparse, cxx:_mj_stackAllocInfo, cxx:_mju_zero, cxx:_mju_zeroInt
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn cell_pos_and_jac(m: *const mjModel, d: *mut mjData, flex_id: i32, npc: i32, gindices: *const i32, nv: i32, xpos_c: *const f64, cell_chain: *mut i32, cell_nnz: *mut i32) -> *mut f64 {
    todo!() // cxx-internal:engine_core_constraint.c.o:_cell_pos_and_jac
}

/// C: cell_strain_jacobian (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_constraint.c:111)
/// Calls: cxx:_mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn cell_strain_jacobian(npc: i32, cell_nnz: i32, dSdx_local: *const f64, cell_node_jac: *const f64, strain_jac: *mut f64) {
    todo!() // cxx-internal:engine_core_constraint.c.o:_cell_strain_jacobian
}

/// C: arenaAllocEfc (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_constraint.c:130)
/// Calls: cxx:_mj_arenaAllocByte, cxx:_mj_warning
#[allow(unused_variables, non_snake_case)]
pub fn arena_alloc_efc(m: *const mjModel, d: *mut mjData) -> i32 {
    todo!() // cxx-internal:engine_core_constraint.c.o:_arenaAllocEfc
}

/// C: mj_elemBodyWeight (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_constraint.c:223)
/// Calls: cxx:_mju_dist3, cxx:_mju_max, cxx:_mju_message, cxx:_mju_scl, cxx:_mju_sum
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_elem_body_weight(m: *const mjModel, d: *const mjData, f: i32, e: i32, v: i32, point: *const f64, body: *mut i32, weight: *mut f64) -> i32 {
    todo!() // cxx-internal:engine_core_constraint.c.o:_mj_elemBodyWeight
}

/// C: mj_vertBodyWeight (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_constraint.c:265)
/// Calls: cxx:_mju_addToScl3, cxx:_mju_cellLookup, cxx:_mju_evalBasis, cxx:_mju_evalBasisArray, cxx:_mju_shellTFIWeights
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_vert_body_weight(m: *const mjModel, d: *const mjData, f: i32, v: *mut i32, body: *mut i32, bweight: *mut f64, vweight: *const f64, nw: i32) -> i32 {
    todo!() // cxx-internal:engine_core_constraint.c.o:_mj_vertBodyWeight
}

/// C: mj_addConstraint (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_constraint.c:414)
/// Calls: cxx:_mj_isSparse, cxx:_mju_copy, cxx:_mju_copyInt, cxx:_mju_fillInt, cxx:_mju_message
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_add_constraint(m: *const mjModel, d: *mut mjData, jac: *const f64, pos: *const f64, margin: *const f64, frictionloss: f64, size: i32, r#type: i32, id: i32, NV: i32, chain: *const i32) {
    todo!() // cxx-internal:engine_core_constraint.c.o:_mj_addConstraint
}

/// C: mj_equalityAnchors (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_constraint.c:561)
/// Calls: cxx:_mju_addTo3, cxx:_mju_copy3, cxx:_mju_mulMatVec3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_equality_anchors(m: *const mjModel, d: *const mjData, eq_id: i32, pos1: *mut f64, pos2: *mut f64, body1: *mut i32, body2: *mut i32) {
    todo!() // cxx-internal:engine_core_constraint.c.o:_mj_equalityAnchors
}

/// C: mj_addConstraintCount (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_constraint.c:1259)
/// Calls: cxx:_mj_isSparse
#[allow(unused_variables, non_snake_case)]
pub fn mj_add_constraint_count(m: *const mjModel, size: i32, NV: i32) -> i32 {
    todo!() // cxx-internal:engine_core_constraint.c.o:_mj_addConstraintCount
}

/// C: mj_instantiateFriction (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_constraint.c:1270)
/// Calls: cxx-internal:engine_core_constraint.c.o:_mj_addConstraint, cxx-internal:engine_core_constraint.c.o:_mj_addConstraintCount, cxx:_mj_freeStack, cxx:_mj_isSparse, cxx:_mj_markStack, cxx:_mj_sleepState, cxx:_mj_stackAllocInfo, cxx:_mju_sparse2dense, cxx:_mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mj_instantiate_friction(m: *const mjModel, d: *mut mjData, count_only: i32, nnz: *mut i32) -> i32 {
    todo!() // cxx-internal:engine_core_constraint.c.o:_mj_instantiateFriction
}

/// C: mj_instantiateLimit (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_constraint.c:1360)
/// Calls: cxx-internal:engine_core_constraint.c.o:_mj_addConstraint, cxx-internal:engine_core_constraint.c.o:_mj_addConstraintCount, cxx:_mj_freeStack, cxx:_mj_isSparse, cxx:_mj_markStack, cxx:_mj_sleepState, cxx:_mj_stackAllocInfo, cxx:_mju_max, cxx:_mju_normalize3, cxx:_mju_normalize4, cxx:_mju_quat2Vel, cxx:_mju_scl, cxx:_mju_scl3, cxx:_mju_sparse2dense, cxx:_mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mj_instantiate_limit(m: *const mjModel, d: *mut mjData, count_only: i32, nnz: *mut i32) -> i32 {
    todo!() // cxx-internal:engine_core_constraint.c.o:_mj_instantiateLimit
}

/// C: getsolparam (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_constraint.c:1978)
/// Calls: cxx:_mj_defaultSolRefImp, cxx:_mju_copy, cxx:_mju_max, cxx:_mju_min, cxx:_mju_warning, cxx:_mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn getsolparam(m: *const mjModel, d: *const mjData, i: i32, solref: *mut f64, solreffriction: *mut f64, solimp: *mut f64) {
    todo!() // cxx-internal:engine_core_constraint.c.o:_getsolparam
}

/// C: getposdim (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_constraint.c:2053)
/// Calls: cxx:_mju_norm
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn getposdim(m: *const mjModel, d: *const mjData, i: i32, pos: *mut f64, dim: *mut i32) {
    todo!() // cxx-internal:engine_core_constraint.c.o:_getposdim
}

/// C: power (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_constraint.c:2089)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn power(a: f64, b: f64) -> f64 {
    todo!() // cxx-internal:engine_core_constraint.c.o:_power
}

/// C: getimpedance (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_constraint.c:2100)
/// Calls: cxx-internal:engine_core_constraint.c.o:_power
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn getimpedance(solimp: *const f64, pos: f64, margin: f64, imp: *mut f64, impP: *mut f64) {
    todo!() // cxx-internal:engine_core_constraint.c.o:_getimpedance
}

/// C: mj_jacSumCount (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_constraint.c:2272)
/// Calls: cxx:_mj_bodyChain, cxx:_mj_freeStack, cxx:_mj_markStack, cxx:_mj_stackAllocInfo, cxx:_mju_addChains, cxx:_mju_copyInt
#[allow(unused_variables, non_snake_case)]
pub fn mj_jac_sum_count(m: *const mjModel, d: *mut mjData, chain: *mut i32, n: i32, body: *const i32) -> i32 {
    todo!() // cxx-internal:engine_core_constraint.c.o:_mj_jacSumCount
}

/// C: mj_ne (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_constraint.c:2303)
/// Calls: cxx-internal:engine_core_constraint.c.o:_mj_addConstraintCount, cxx-internal:engine_core_constraint.c.o:_mj_jacSumCount, cxx:_mj_freeStack, cxx:_mj_jacDifPair, cxx:_mj_markStack, cxx:_mj_sleepState, cxx:_mj_stackAllocInfo, cxx:_mju_combineSparseCount, cxx:_mju_copyInt, cxx:_mju_flexGatherCellState, cxx:_mju_flexGatherFaceState, cxx:_mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_ne(m: *const mjModel, d: *mut mjData, nnz: *mut i32) -> i32 {
    todo!() // cxx-internal:engine_core_constraint.c.o:_mj_ne
}

/// C: mj_nc (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_constraint.c:2536)
/// Calls: cxx-internal:engine_core_constraint.c.o:_mj_elemBodyWeight, cxx-internal:engine_core_constraint.c.o:_mj_jacSumCount, cxx-internal:engine_core_constraint.c.o:_mj_vertBodyWeight, cxx:_mj_flexBody, cxx:_mj_freeStack, cxx:_mj_isPyramidal, cxx:_mj_isSparse, cxx:_mj_jacDifPair, cxx:_mj_markStack, cxx:_mj_stackAllocInfo, cxx:_mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_nc(m: *const mjModel, d: *mut mjData, nnz: *mut i32) -> i32 {
    todo!() // cxx-internal:engine_core_constraint.c.o:_mj_nc
}

/// C: computeY_precount (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_constraint.c:2688)
/// Calls: cxx:_mju_fillInt
#[allow(unused_variables, non_snake_case)]
pub fn compute_y_precount(Y_rownnz: *mut i32, Y_rowadr: *mut i32, nefc: i32, nv: i32, J_rownnz: *const i32, J_rowadr: *const i32, J_colind: *const i32, M_rownnz: *const i32, M_rowadr: *const i32, M_colind: *const i32, marker: *mut i32) -> i32 {
    todo!() // cxx-internal:engine_core_constraint.c.o:_computeY_precount
}

/// C: computeY_fill (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_constraint.c:2734)
/// Calls: cxx:_mju_message
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn compute_y_fill(Y: *mut f64, Y_colind: *mut i32, Y_rownnz: *const i32, Y_rowadr: *const i32, nefc: i32, J: *const f64, J_rownnz: *const i32, J_rowadr: *const i32, J_colind: *const i32, dof_parentid: *const i32) {
    todo!() // cxx-internal:engine_core_constraint.c.o:_computeY_fill
}

/// C: computeY_backsub (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_constraint.c:2781)
/// Calls: cxx:_mju_addToSclSparseInc
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn compute_y_backsub(Y: *mut f64, Y_rownnz: *const i32, Y_rowadr: *const i32, Y_colind: *const i32, nefc: i32, qLD: *const f64, M_rownnz: *const i32, M_rowadr: *const i32, M_colind: *const i32, sqrtInvD: *const f64) {
    todo!() // cxx-internal:engine_core_constraint.c.o:_computeY_backsub
}

/// C: mj_makeY (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_constraint.c:2908)
/// Calls: cxx-internal:engine_core_constraint.c.o:_computeY_backsub, cxx-internal:engine_core_constraint.c.o:_computeY_fill, cxx-internal:engine_core_constraint.c.o:_computeY_precount, cxx:_mj_arenaAllocByte, cxx:_mj_freeStack, cxx:_mj_isSparse, cxx:_mj_markStack, cxx:_mj_solveM2, cxx:_mj_stackAllocInfo, cxx:_mj_warning, cxx:_mju_dot
#[allow(unused_variables, non_snake_case)]
pub fn mj_make_y(m: *const mjModel, d: *mut mjData, flg_diagexact: i32) {
    todo!() // cxx-internal:engine_core_constraint.c.o:_mj_makeY
}

/// C: mj_makeAR (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_constraint.c:2999)
/// Calls: cxx:_mj_arenaAllocByte, cxx:_mj_freeStack, cxx:_mj_isSparse, cxx:_mj_markStack, cxx:_mj_stackAllocInfo, cxx:_mj_warning, cxx:_mju_sqrMatTD, cxx:_mju_sqrMatTDSparseNumeric, cxx:_mju_sqrMatTDSparseSymbolic, cxx:_mju_transpose, cxx:_mju_transposeSparse
#[allow(unused_variables, non_snake_case)]
pub fn mj_make_ar(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx-internal:engine_core_constraint.c.o:_mj_makeAR
}

/// C: mj_isDual (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_constraint.h:31)
#[allow(unused_variables, non_snake_case)]
pub fn mj_is_dual(m: *const mjModel) -> i32 {
    todo!() // cxx:_mj_isDual
}

/// C: mj_mulJacVec (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_constraint.h:34)
/// Calls: cxx:_mj_isSparse, cxx:_mju_mulMatVec, cxx:_mju_mulMatVecSparse
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_mul_jac_vec(m: *const mjModel, d: *const mjData, res: *mut f64, vec: *const f64) {
    todo!() // cxx:_mj_mulJacVec
}

/// C: mj_mulJacTVec (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_constraint.h:37)
/// Calls: cxx:_mj_isSparse, cxx:_mju_mulMatTVec, cxx:_mju_mulMatTVecSparse
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_mul_jac_t_vec(m: *const mjModel, d: *const mjData, res: *mut f64, vec: *const f64) {
    todo!() // cxx:_mj_mulJacTVec
}

/// C: mj_Jdotv (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_constraint.h:40)
/// Calls: cxx-internal:engine_core_constraint.c.o:_mj_equalityAnchors, cxx:_mj_freeStack, cxx:_mj_isSparse, cxx:_mj_jacDot, cxx:_mj_jacDotSparse, cxx:_mj_markStack, cxx:_mj_mergeChain, cxx:_mj_stackAllocInfo, cxx:_mju_copy4, cxx:_mju_derivQuat, cxx:_mju_dotSparseX3, cxx:_mju_mulMatVec, cxx:_mju_mulQuat, cxx:_mju_mulQuatAxis, cxx:_mju_negQuat, cxx:_mju_sub3, cxx:_mju_zero3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_jdotv(m: *const mjModel, d: *mut mjData, result: *mut f64) {
    todo!() // cxx:_mj_Jdotv
}

/// C: mj_assignRef (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_constraint.h:46)
/// Calls: cxx:_mju_copy
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_assign_ref(m: *const mjModel, target: *mut f64, source: *const f64) {
    todo!() // cxx:_mj_assignRef
}

/// C: mj_assignImp (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_constraint.h:49)
/// Calls: cxx:_mju_copy
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_assign_imp(m: *const mjModel, target: *mut f64, source: *const f64) {
    todo!() // cxx:_mj_assignImp
}

/// C: mj_assignFriction (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_constraint.h:52)
/// Calls: cxx:_mju_max
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_assign_friction(m: *const mjModel, target: *mut f64, source: *const f64) {
    todo!() // cxx:_mj_assignFriction
}

/// C: mj_assignMargin (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_constraint.h:55)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_assign_margin(m: *const mjModel, source: f64) -> f64 {
    todo!() // cxx:_mj_assignMargin
}

/// C: mj_addContact (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_constraint.h:58)
/// Calls: cxx:_mj_arenaAllocByte, cxx:_mj_warning
#[allow(unused_variables, non_snake_case)]
pub fn mj_add_contact(m: *const mjModel, d: *mut mjData, con: *const mjContact) -> i32 {
    todo!() // cxx:_mj_addContact
}

/// C: mj_instantiateEquality (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_constraint.h:63)
/// Calls: cxx-internal:engine_core_constraint.c.o:_cell_pos_and_jac, cxx-internal:engine_core_constraint.c.o:_cell_strain_jacobian, cxx-internal:engine_core_constraint.c.o:_mj_addConstraint, cxx-internal:engine_core_constraint.c.o:_mj_equalityAnchors, cxx:_mj_freeStack, cxx:_mj_isSparse, cxx:_mj_jacDifPair, cxx:_mj_markStack, cxx:_mj_sleepState, cxx:_mj_stackAllocInfo, cxx:_mju_addTo3, cxx:_mju_addToScl, cxx:_mju_copy, cxx:_mju_copy3, cxx:_mju_copyInt, cxx:_mju_defGradient, cxx:_mju_flexGatherCellState, cxx:_mju_flexGatherFaceState, cxx:_mju_flexInterpRotation2D, cxx:_mju_mat2Rot, cxx:_mju_message, cxx:_mju_mulMatVec3, cxx:_mju_mulQuat, cxx:_mju_mulQuatAxis, cxx:_mju_negQuat, cxx:_mju_rotVecQuat, cxx:_mju_scl, cxx:_mju_scl3, cxx:_mju_sparse2dense, cxx:_mju_sub3, cxx:_mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mj_instantiate_equality(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx:_mj_instantiateEquality
}

/// C: mj_instantiateContact (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_constraint.h:66)
/// Calls: cxx-internal:engine_core_constraint.c.o:_mj_addConstraint, cxx:_mj_contactJacobian, cxx:_mj_freeStack, cxx:_mj_isPyramidal, cxx:_mj_isSparse, cxx:_mj_markStack, cxx:_mj_stackAllocInfo, cxx:_mju_addScl, cxx:_mju_mulMatMat, cxx:_mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mj_instantiate_contact(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx:_mj_instantiateContact
}

/// C: mj_contactJacobian (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_constraint.h:69)
/// Calls: cxx-internal:engine_core_constraint.c.o:_mj_elemBodyWeight, cxx-internal:engine_core_constraint.c.o:_mj_vertBodyWeight, cxx:_mj_isSparse, cxx:_mj_jacDifPair, cxx:_mj_jacSum, cxx:_mju_scl
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_contact_jacobian(m: *const mjModel, d: *mut mjData, con: *const mjContact, dim: i32, jac: *mut f64, jacdif: *mut f64, jacdifp: *mut f64, jacdifr: *mut f64, jac1p: *mut f64, jac2p: *mut f64, jac1r: *mut f64, jac2r: *mut f64, chain: *mut i32) -> i32 {
    todo!() // cxx:_mj_contactJacobian
}

/// C: mj_diagApprox (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_constraint.h:78)
/// Calls: cxx-internal:engine_core_constraint.c.o:_mj_elemBodyWeight, cxx-internal:engine_core_constraint.c.o:_mj_vertBodyWeight, cxx:_mju_flexGatherCellState, cxx:_mju_flexGatherFaceState, cxx:_mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_diag_approx(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx:_mj_diagApprox
}

/// C: mj_makeImpedance (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_constraint.h:81)
/// Calls: cxx-internal:engine_core_constraint.c.o:_getimpedance, cxx-internal:engine_core_constraint.c.o:_getposdim, cxx-internal:engine_core_constraint.c.o:_getsolparam, cxx:_mju_max
#[allow(unused_variables, non_snake_case)]
pub fn mj_make_impedance(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx:_mj_makeImpedance
}

/// C: mj_makeConstraint (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_constraint.h:87)
/// Calls: cxx-internal:engine_core_constraint.c.o:_arenaAllocEfc, cxx-internal:engine_core_constraint.c.o:_mj_instantiateFriction, cxx-internal:engine_core_constraint.c.o:_mj_instantiateLimit, cxx-internal:engine_core_constraint.c.o:_mj_nc, cxx-internal:engine_core_constraint.c.o:_mj_ne, cxx:_mj_diagApprox, cxx:_mj_instantiateContact, cxx:_mj_instantiateEquality, cxx:_mj_isSparse, cxx:_mj_makeImpedance, cxx:_mju_fillInt, cxx:_mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_make_constraint(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx:_mj_makeConstraint
}

/// C: mj_projectConstraint (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_constraint.h:90)
/// Calls: cxx-internal:engine_core_constraint.c.o:_mj_makeAR, cxx-internal:engine_core_constraint.c.o:_mj_makeY, cxx:_mj_isDual, cxx:_mj_makeImpedance, cxx:_mju_gather
#[allow(unused_variables, non_snake_case)]
pub fn mj_project_constraint(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx:_mj_projectConstraint
}

/// C: mj_referenceConstraint (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_constraint.h:93)
/// Calls: cxx:_mj_Jdotv, cxx:_mj_mulJacVec
#[allow(unused_variables, non_snake_case)]
pub fn mj_reference_constraint(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx:_mj_referenceConstraint
}

/// C: mj_constraintUpdate_impl (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_constraint.h:97)
/// Calls: cxx:_mju_norm, cxx:_mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_constraint_update_impl(ne: i32, nf: i32, nefc: i32, D: *const f64, R: *const f64, floss: *const f64, jar: *const f64, r#type: *const i32, id: *const i32, contact: *mut mjContact, state: *mut i32, force: *mut f64, cost: *mut f64, flg_coneHessian: i32) {
    todo!() // cxx:_mj_constraintUpdate_impl
}

/// C: mj_constraintUpdate (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_constraint.h:105)
/// Calls: cxx:_mj_constraintUpdate_impl, cxx:_mj_mulJacTVec
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_constraint_update(m: *const mjModel, d: *mut mjData, jar: *const f64, cost: *mut f64, flg_coneHessian: i32) {
    todo!() // cxx:_mj_constraintUpdate
}

