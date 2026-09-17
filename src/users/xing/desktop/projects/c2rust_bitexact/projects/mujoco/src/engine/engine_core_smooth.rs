//! Port of: Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_smooth.c
//! IR hash: 1fa44043fe383ce9
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mj_updateDynamicBVH (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_smooth.c:490)
/// Calls: cxx:_mj_freeStack, cxx:_mj_markStack, cxx:_mj_stackAllocInfo, cxx:_mju_max, cxx:_mju_min, cxx:_mju_zeroInt
#[allow(unused_variables, non_snake_case)]
pub fn mj_update_dynamic_bvh(m: *const mjModel, d: *mut mjData, bvhadr: i32, bvhnum: i32) {
    todo!() // cxx:_mj_updateDynamicBVH
}

/// C: mju_mulMatMat322 (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_smooth.c:537)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_mul_mat_mat322(C: *mut f64, A: *const f64, B: *const f64) {
    todo!() // cxx-internal:engine_core_smooth.c.o:_mju_mulMatMat322
}

/// C: mj_kinematics1 (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_smooth.h:29)
/// Calls: cxx:_mju_message, cxx:_mju_mulQuat, cxx:_mju_normalize4, cxx:_mju_quat2Mat, cxx:_mju_unit4, cxx:_mju_zero, cxx:_mju_zero3
#[allow(unused_variables, non_snake_case)]
pub fn mj_kinematics1(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx:_mj_kinematics1
}

/// C: mj_kinematics2 (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_smooth.h:32)
/// Calls: cxx:_mj_local2Global
#[allow(unused_variables, non_snake_case)]
pub fn mj_kinematics2(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx:_mj_kinematics2
}

/// C: mj_kinematics (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_smooth.h:35)
/// Calls: cxx:_mj_kinematics1, cxx:_mj_kinematics2, cxx:_mj_updateSleep, cxx:_mj_wake
#[allow(unused_variables, non_snake_case)]
pub fn mj_kinematics(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx:_mj_kinematics
}

/// C: mj_comPos (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_smooth.h:38)
/// Calls: cxx:_mju_dofCom, cxx:_mju_inertCom, cxx:_mju_scl3, cxx:_mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mj_com_pos(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx:_mj_comPos
}

/// C: mj_camlight (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_smooth.h:41)
/// Calls: cxx:_mj_local2Global, cxx:_mju_normalize3, cxx:_mju_transpose
#[allow(unused_variables, non_snake_case)]
pub fn mj_camlight(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx:_mj_camlight
}

/// C: mj_flex (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_smooth.h:44)
/// Calls: cxx-internal:engine_core_smooth.c.o:_mju_mulMatMat322, cxx:_mj_bodyChain, cxx:_mj_freeStack, cxx:_mj_jacDifPair, cxx:_mj_jacSparse, cxx:_mj_markStack, cxx:_mj_stackAllocInfo, cxx:_mj_updateDynamicBVH, cxx:_mju_cellLookup, cxx:_mju_interpolate3D, cxx:_mju_max, cxx:_mju_message, cxx:_mju_min, cxx:_mju_mulMatTVec, cxx:_mju_mulMatVec, cxx:_mju_normalize3, cxx:_mju_scl3, cxx:_mju_shellTrackInterior, cxx:_mju_sub3, cxx:_mju_zero, cxx:_mju_zero3
#[allow(unused_variables, non_snake_case)]
pub fn mj_flex(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx:_mj_flex
}

/// C: mj_tendon (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_smooth.h:47)
/// Calls: cxx:_mj_freeStack, cxx:_mj_jacDifPair, cxx:_mj_markStack, cxx:_mj_sleepState, cxx:_mj_stackAllocInfo, cxx:_mju_combineSparseInc, cxx:_mju_dist3, cxx:_mju_message, cxx:_mju_mulMatTVec, cxx:_mju_normalize3, cxx:_mju_round, cxx:_mju_wrap, cxx:_mju_zero, cxx:_mju_zero3
#[allow(unused_variables, non_snake_case)]
pub fn mj_tendon(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx:_mj_tendon
}

/// C: mj_tendonDot (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_smooth.h:50)
/// Calls: cxx:_mj_freeStack, cxx:_mj_isSparse, cxx:_mj_jac, cxx:_mj_jacDot, cxx:_mj_jacDotSparse, cxx:_mj_jacSparse, cxx:_mj_markStack, cxx:_mj_mergeChain, cxx:_mj_objectVelocity, cxx:_mj_stackAllocInfo, cxx:_mju_addToScl3, cxx:_mju_dot, cxx:_mju_dot3, cxx:_mju_message, cxx:_mju_mulMatTVec, cxx:_mju_normalize3, cxx:_mju_scl3, cxx:_mju_sub, cxx:_mju_sub3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_tendon_dot(m: *const mjModel, d: *mut mjData, id: i32, vec: *const f64) -> f64 {
    todo!() // cxx:_mj_tendonDot
}

/// C: mj_transmission (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_smooth.h:53)
/// Calls: cxx:_mj_freeStack, cxx:_mj_isSparse, cxx:_mj_jacDifPair, cxx:_mj_jacPointAxis, cxx:_mj_jacSite, cxx:_mj_markStack, cxx:_mj_mulJacTVec, cxx:_mj_sleepState, cxx:_mj_stackAllocInfo, cxx:_mju_addTo, cxx:_mju_copyInt, cxx:_mju_dot3, cxx:_mju_isZero, cxx:_mju_message, cxx:_mju_mulMatMat, cxx:_mju_mulMatTVec, cxx:_mju_mulMatTVec3, cxx:_mju_negQuat, cxx:_mju_normalize4, cxx:_mju_scl, cxx:_mju_scl3, cxx:_mju_sub3, cxx:_mju_subFrom, cxx:_mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mj_transmission(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx:_mj_transmission
}

/// C: mj_crb (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_smooth.h:59)
/// Calls: cxx-internal:engine_core_smooth.c.o:_mji_dot6, cxx:_mj_actuatorArmature, cxx:_mju_addTo, cxx:_mju_copy, cxx:_mju_copyRows, cxx:_mju_mulInertVec, cxx:_mju_zero, cxx:_mju_zeroSparse
#[allow(unused_variables, non_snake_case)]
pub fn mj_crb(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx:_mj_crb
}

/// C: mj_tendonArmature (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_smooth.h:62)
/// Calls: cxx:_mj_actuatorArmature, cxx:_mj_sleepState, cxx:_mju_addToSclSparseInc
#[allow(unused_variables, non_snake_case)]
pub fn mj_tendon_armature(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx:_mj_tendonArmature
}

/// C: mj_makeM (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_smooth.h:65)
/// Calls: cxx:_mj_crb, cxx:_mj_tendonArmature, cxx:_mju_scatter
#[allow(unused_variables, non_snake_case)]
pub fn mj_make_m(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx:_mj_makeM
}

/// C: mj_factorI_legacy (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_smooth.h:68)
/// Calls: cxx:_mj_warning, cxx:_mju_addToScl, cxx:_mju_copy
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_factor_i_legacy(m: *const mjModel, d: *mut mjData, M: *const f64, qLD: *mut f64, qLDiagInv: *mut f64) {
    todo!() // cxx:_mj_factorI_legacy
}

/// C: mj_factorI (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_smooth.h:72)
/// Calls: cxx:_mju_addToScl, cxx:_mju_scl
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_factor_i(mat: *mut f64, diaginv: *mut f64, nv: i32, rownnz: *const i32, rowadr: *const i32, colind: *const i32, index: *const i32) {
    todo!() // cxx:_mj_factorI
}

/// C: mj_factorM (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_smooth.h:76)
/// Calls: cxx:_mj_factorI, cxx:_mju_copy, cxx:_mju_copySparse
#[allow(unused_variables, non_snake_case)]
pub fn mj_factor_m(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx:_mj_factorM
}

/// C: mj_solveLD_legacy (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_smooth.h:79)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_solve_ld_legacy(m: *const mjModel, x: *mut f64, n: i32, qLD: *const f64, qLDiagInv: *const f64) {
    todo!() // cxx:_mj_solveLD_legacy
}

/// C: mj_solveLD (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_smooth.h:84)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_solve_ld(x: *mut f64, qLD: *const f64, qLDiagInv: *const f64, nv: i32, n: i32, rownnz: *const i32, rowadr: *const i32, colind: *const i32, index: *const i32) {
    todo!() // cxx:_mj_solveLD
}

/// C: mj_solveM (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_smooth.h:88)
/// Calls: cxx:_mj_solveLD, cxx:_mju_copy
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_solve_m(m: *const mjModel, d: *mut mjData, x: *mut f64, y: *const f64, n: i32) {
    todo!() // cxx:_mj_solveM
}

/// C: mj_solveM2 (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_smooth.h:91)
/// Calls: cxx:_mju_copy
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_solve_m2(m: *const mjModel, d: *mut mjData, x: *mut f64, y: *const f64, sqrtInvD: *const f64, n: i32) {
    todo!() // cxx:_mj_solveM2
}

/// C: mj_comVel (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_smooth.h:98)
/// Calls: cxx:_mju_addTo, cxx:_mju_copy, cxx:_mju_mulDofVec, cxx:_mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mj_com_vel(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx:_mj_comVel
}

/// C: mj_subtreeVel (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_smooth.h:101)
/// Calls: cxx:_mj_freeStack, cxx:_mj_markStack, cxx:_mj_objectVelocity, cxx:_mj_stackAllocInfo, cxx:_mju_max, cxx:_mju_mulMatTVec3, cxx:_mju_scl3, cxx:_mju_sub3
#[allow(unused_variables, non_snake_case)]
pub fn mj_subtree_vel(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx:_mj_subtreeVel
}

/// C: mj_rne (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_smooth.h:107)
/// Calls: cxx-internal:engine_core_smooth.c.o:_mji_crossForce, cxx-internal:engine_core_smooth.c.o:_mji_dot6, cxx:_mj_freeStack, cxx:_mj_markStack, cxx:_mj_stackAllocInfo, cxx:_mju_add, cxx:_mju_addTo, cxx:_mju_mulDofVec, cxx:_mju_mulInertVec, cxx:_mju_scl3, cxx:_mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_rne(m: *const mjModel, d: *mut mjData, flg_acc: i32, result: *mut f64) {
    todo!() // cxx:_mj_rne
}

/// C: mj_rnePostConstraint (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_smooth.h:110)
/// Calls: cxx-internal:engine_core_smooth.c.o:_mji_crossForce, cxx:_mj_contactForce, cxx:_mj_local2Global, cxx:_mju_add, cxx:_mju_addTo, cxx:_mju_isZero, cxx:_mju_message, cxx:_mju_mulDofVec, cxx:_mju_mulInertVec, cxx:_mju_mulMatTVec3, cxx:_mju_scl3, cxx:_mju_sub, cxx:_mju_subFrom, cxx:_mju_transformSpatial, cxx:_mju_zero, cxx:_mju_zero3
#[allow(unused_variables, non_snake_case)]
pub fn mj_rne_post_constraint(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx:_mj_rnePostConstraint
}

/// C: mj_tendonBias (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_smooth.h:116)
/// Calls: cxx:_mj_actuatorArmature, cxx:_mj_sleepState, cxx:_mj_tendonDot
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_tendon_bias(m: *const mjModel, d: *mut mjData, qfrc: *mut f64) {
    todo!() // cxx:_mj_tendonBias
}

