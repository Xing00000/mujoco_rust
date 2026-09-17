//! Port of: engine/engine_setconst.c
//! IR hash: fd5288ad7242f001
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mj_setM0 (engine/engine_setconst.c:39)
/// Calls: cxx:_mj_actuatorArmature, cxx:_mju_addTo, cxx:_mju_copy, cxx:_mju_dot, cxx:_mju_mulInertVec
#[allow(unused_variables, non_snake_case)]
pub fn mj_set_m0(m: *mut mjModel, d: *mut mjData) {
    todo!() // cxx-internal:engine_setconst.c.o:_mj_setM0
}

/// C: GetWrapBodyTreeId (engine/engine_setconst.c:66)
#[allow(unused_variables, non_snake_case)]
pub fn get_wrap_body_tree_id(m: *const mjModel, wrap_index: i32) -> i32 {
    todo!() // cxx-internal:engine_setconst.c.o:_GetWrapBodyTreeId
}

/// C: isNullVec3 (engine/engine_setconst.c:93)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn is_null_vec3(v: *const f64) -> i32 {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_setconst.c:_isNullVec3
}

/// C: isNullQuat (engine/engine_setconst.c:101)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn is_null_quat(q: *const f64) -> i32 {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_setconst.c:_isNullQuat
}

/// C: isSameQuat (engine/engine_setconst.c:115)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn is_same_quat(q1: *const f64, q2: *const f64) -> i32 {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_setconst.c:_isSameQuat
}

/// C: isSameVec3 (engine/engine_setconst.c:129)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn is_same_vec3(v1: *const f64, v2: *const f64) -> i32 {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_setconst.c:_isSameVec3
}

/// C: setSameframe (engine/engine_setconst.c:137)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_setconst.c:_isNullQuat, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_setconst.c:_isNullVec3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_setconst.c:_isSameQuat, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_setconst.c:_isSameVec3
#[allow(unused_variables, non_snake_case)]
pub fn set_sameframe(m: *mut mjModel) {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_setconst.c:_setSameframe
}

/// C: setFixed (engine/engine_setconst.c:194)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_setconst.c:_GetWrapBodyTreeId, cxx:_mj_freeStack, cxx:_mj_markStack, cxx:_mj_stackAllocInfo, cxx:_mj_tendonHasDamping, cxx:_mj_tendonHasStiffness, cxx:_mju_fillInt, cxx:_mju_isZero, cxx:_mju_message, cxx:_mju_zeroInt
#[allow(unused_variables, non_snake_case)]
pub fn set_fixed(m: *mut mjModel, d: *mut mjData) {
    todo!() // cxx-internal:engine_setconst.c.o:_setFixed
}

/// C: makeTendonSparse (engine/engine_setconst.c:481)
/// Calls: cxx:_mju_zeroInt
#[allow(unused_variables, non_snake_case)]
pub fn make_tendon_sparse(m: *mut mjModel) {
    todo!() // cxx-internal:engine_setconst.c.o:_makeTendonSparse
}

/// C: makeFlexSparse (engine/engine_setconst.c:568)
/// Calls: cxx:_mj_bodyChain, cxx:_mj_freeStack, cxx:_mj_jacDifPair, cxx:_mj_markStack, cxx:_mj_stackAllocInfo, cxx:_mju_copyInt, cxx:_mju_message, cxx:_mju_sub3, cxx:_mju_zero, cxx:_mju_zeroInt
#[allow(unused_variables, non_snake_case)]
pub fn make_flex_sparse(m: *mut mjModel, d: *mut mjData) {
    todo!() // cxx-internal:engine_setconst.c.o:_makeFlexSparse
}

/// C: mj_alignFlex (engine/engine_setconst.c:792)
/// Calls: cxx:_mju_copy3, cxx:_mju_cross, cxx:_mju_mulMatTVec3, cxx:_mju_normalize3, cxx:_mju_quat2Mat, cxx:_mju_quatZ2Vec, cxx:_mju_sub3, cxx:_mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn mj_align_flex(m: *mut mjModel, d: *mut mjData) {
    todo!() // cxx-internal:engine_setconst.c.o:_mj_alignFlex
}

/// C: set0 (engine/engine_setconst.c:839)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_setconst.c:_makeFlexSparse, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_setconst.c:_makeTendonSparse, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_setconst.c:_mj_alignFlex, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_setconst.c:_mj_setM0, cxx:_mj_camlight, cxx:_mj_comPos, cxx:_mj_factorM, cxx:_mj_flex, cxx:_mj_freeStack, cxx:_mj_jacBodyCom, cxx:_mj_kinematics, cxx:_mj_local2Global, cxx:_mj_makeM, cxx:_mj_markStack, cxx:_mj_solveM, cxx:_mj_stackAllocInfo, cxx:_mj_tendon, cxx:_mj_transmission, cxx:_mju_copy, cxx:_mju_copy3, cxx:_mju_copy9, cxx:_mju_dot, cxx:_mju_isZero, cxx:_mju_max, cxx:_mju_message, cxx:_mju_mulMatMatT, cxx:_mju_mulMatTVec3, cxx:_mju_mulQuat, cxx:_mju_negQuat, cxx:_mju_norm, cxx:_mju_normalize4, cxx:_mju_sparse2dense, cxx:_mju_sub3, cxx:_mju_subFrom3, cxx:_mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn set0(m: *mut mjModel, d: *mut mjData) {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_setconst.c:_set0
}

/// C: updateBox (engine/engine_setconst.c:1187)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn update_box(xmin: *mut f64, xmax: *mut f64, pos: *mut f64, radius: f64) {
    todo!() // cxx-internal:engine_setconst.c.o:_updateBox
}

/// C: setStat (engine/engine_setconst.c:1196)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_setconst.c:_updateBox, cxx:_mj_freeStack, cxx:_mj_markStack, cxx:_mj_stackAllocInfo, cxx:_mju_add3, cxx:_mju_dist3, cxx:_mju_max, cxx:_mju_scl3, cxx:_mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn set_stat(m: *mut mjModel, d: *mut mjData) {
    todo!() // cxx-internal:engine_setconst.c.o:_setStat
}

/// C: setSpring (engine/engine_setconst.c:1344)
/// Calls: cxx:_mj_comPos, cxx:_mj_kinematics, cxx:_mj_tendon, cxx:_mj_transmission, cxx:_mju_copy
#[allow(unused_variables, non_snake_case)]
pub fn set_spring(m: *mut mjModel, d: *mut mjData) {
    todo!() // cxx-internal:engine_setconst.c.o:_setSpring
}

/// C: setEfm0Factor (engine/engine_setconst.c:1372)
/// Calls: cxx:_mj_freeStack, cxx:_mj_markStack, cxx:_mj_stackAllocInfo, cxx:_mjd_flexStiff_assemble, cxx:_mju_cholFactorNumeric, cxx:_mju_cholFactorSymbolic, cxx:_mju_message
#[allow(unused_variables, non_snake_case)]
pub fn set_efm0factor(m: *mut mjModel, d: *mut mjData) {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_setconst.c:_setEfm0Factor
}

/// C: mj_setConst (engine/engine_setconst.c:1523)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_setconst.c:_set0, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_setconst.c:_setEfm0Factor, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_setconst.c:_setFixed, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_setconst.c:_setSameframe, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_setconst.c:_setSpring, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_setconst.c:_setStat, cxx:_mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_set_const(m: *mut mjModel, d: *mut mjData) {
    todo!() // cxx:_mj_setConst
}

/// C: evalAct (engine/engine_setconst.c:1566)
/// Calls: cxx:_mj_freeStack, cxx:_mj_markStack, cxx:_mj_solveM, cxx:_mj_stackAllocInfo, cxx:_mj_step1, cxx:_mj_step2, cxx:_mju_norm, cxx:_mju_scl, cxx:_mju_sparse2dense
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn eval_act(m: *const mjModel, d: *mut mjData, index: i32, side: i32, opt: *const mjLROpt) -> f64 {
    todo!() // cxx-internal:engine_setconst.c.o:_evalAct
}

/// C: mj_setLengthRange (engine/engine_setconst.c:1606)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_setconst.c:_evalAct, cxx:_mj_resetData, cxx:_mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_set_length_range(m: *mut mjModel, d: *mut mjData, index: i32, opt: *const mjLROpt, error: *mut i8, error_sz: i32) -> i32 {
    todo!() // cxx:_mj_setLengthRange
}

