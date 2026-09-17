//! Port of: Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_setconst.c
//! IR hash: 1fa44043fe383ce9
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mj_setM0 (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_setconst.c:37)
/// Calls: cxx:_mj_actuatorArmature, cxx:_mju_addTo, cxx:_mju_copy, cxx:_mju_dot, cxx:_mju_mulInertVec
#[allow(unused_variables, non_snake_case)]
pub fn mj_set_m0(m: *mut mjModel, d: *mut mjData) {
    todo!() // cxx-internal:engine_setconst.c.o:_mj_setM0
}

/// C: GetWrapBodyTreeId (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_setconst.c:64)
#[allow(unused_variables, non_snake_case)]
pub fn get_wrap_body_tree_id(m: *const mjModel, wrap_index: i32) -> i32 {
    todo!() // cxx-internal:engine_setconst.c.o:_GetWrapBodyTreeId
}

/// C: setFixed (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_setconst.c:86)
/// Calls: cxx-internal:engine_setconst.c.o:_GetWrapBodyTreeId, cxx:_mj_freeStack, cxx:_mj_markStack, cxx:_mj_stackAllocInfo, cxx:_mju_fillInt, cxx:_mju_isZero, cxx:_mju_message, cxx:_mju_zeroInt
#[allow(unused_variables, non_snake_case)]
pub fn set_fixed(m: *mut mjModel, d: *mut mjData) {
    todo!() // cxx-internal:engine_setconst.c.o:_setFixed
}

/// C: makeTendonSparse (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_setconst.c:337)
/// Calls: cxx:_mju_zeroInt
#[allow(unused_variables, non_snake_case)]
pub fn make_tendon_sparse(m: *mut mjModel) {
    todo!() // cxx-internal:engine_setconst.c.o:_makeTendonSparse
}

/// C: makeFlexSparse (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_setconst.c:424)
/// Calls: cxx:_mj_bodyChain, cxx:_mj_freeStack, cxx:_mj_jacDifPair, cxx:_mj_markStack, cxx:_mj_stackAllocInfo, cxx:_mju_copyInt, cxx:_mju_message, cxx:_mju_sub3, cxx:_mju_zero, cxx:_mju_zeroInt
#[allow(unused_variables, non_snake_case)]
pub fn make_flex_sparse(m: *mut mjModel, d: *mut mjData) {
    todo!() // cxx-internal:engine_setconst.c.o:_makeFlexSparse
}

/// C: mj_alignFlex (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_setconst.c:648)
/// Calls: cxx:_mju_copy3, cxx:_mju_cross, cxx:_mju_mulMatTVec3, cxx:_mju_normalize3, cxx:_mju_quat2Mat, cxx:_mju_quatZ2Vec, cxx:_mju_sub3, cxx:_mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn mj_align_flex(m: *mut mjModel, d: *mut mjData) {
    todo!() // cxx-internal:engine_setconst.c.o:_mj_alignFlex
}

/// C: set0 (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_setconst.c:695)
/// Calls: cxx-internal:engine_setconst.c.o:_makeFlexSparse, cxx-internal:engine_setconst.c.o:_makeTendonSparse, cxx-internal:engine_setconst.c.o:_mj_alignFlex, cxx-internal:engine_setconst.c.o:_mj_setM0, cxx:_mj_camlight, cxx:_mj_comPos, cxx:_mj_factorM, cxx:_mj_flex, cxx:_mj_freeStack, cxx:_mj_jacBodyCom, cxx:_mj_kinematics, cxx:_mj_local2Global, cxx:_mj_makeM, cxx:_mj_markStack, cxx:_mj_solveM, cxx:_mj_stackAllocInfo, cxx:_mj_tendon, cxx:_mj_transmission, cxx:_mju_copy, cxx:_mju_copy3, cxx:_mju_copy9, cxx:_mju_dot, cxx:_mju_isZero, cxx:_mju_max, cxx:_mju_message, cxx:_mju_mulMatMatT, cxx:_mju_mulMatTVec3, cxx:_mju_mulQuat, cxx:_mju_negQuat, cxx:_mju_norm, cxx:_mju_normalize4, cxx:_mju_sparse2dense, cxx:_mju_sub3, cxx:_mju_subFrom3, cxx:_mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn set0(m: *mut mjModel, d: *mut mjData) {
    todo!() // cxx-internal:engine_setconst.c.o:_set0
}

/// C: updateBox (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_setconst.c:1041)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn update_box(xmin: *mut f64, xmax: *mut f64, pos: *mut f64, radius: f64) {
    todo!() // cxx-internal:engine_setconst.c.o:_updateBox
}

/// C: setStat (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_setconst.c:1050)
/// Calls: cxx-internal:engine_setconst.c.o:_updateBox, cxx:_mj_freeStack, cxx:_mj_markStack, cxx:_mj_stackAllocInfo, cxx:_mju_add3, cxx:_mju_dist3, cxx:_mju_max, cxx:_mju_scl3, cxx:_mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn set_stat(m: *mut mjModel, d: *mut mjData) {
    todo!() // cxx-internal:engine_setconst.c.o:_setStat
}

/// C: setSpring (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_setconst.c:1198)
/// Calls: cxx:_mj_comPos, cxx:_mj_kinematics, cxx:_mj_tendon, cxx:_mj_transmission, cxx:_mju_copy
#[allow(unused_variables, non_snake_case)]
pub fn set_spring(m: *mut mjModel, d: *mut mjData) {
    todo!() // cxx-internal:engine_setconst.c.o:_setSpring
}

/// C: evalAct (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_setconst.c:1235)
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

/// C: mj_setConst (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_setconst.h:27)
/// Calls: cxx-internal:engine_setconst.c.o:_set0, cxx-internal:engine_setconst.c.o:_setFixed, cxx-internal:engine_setconst.c.o:_setSpring, cxx-internal:engine_setconst.c.o:_setStat
#[allow(unused_variables, non_snake_case)]
pub fn mj_set_const(m: *mut mjModel, d: *mut mjData) {
    todo!() // cxx:_mj_setConst
}

/// C: mj_setLengthRange (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_setconst.h:30)
/// Calls: cxx-internal:engine_setconst.c.o:_evalAct, cxx:_mj_resetData, cxx:_mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_set_length_range(m: *mut mjModel, d: *mut mjData, index: i32, opt: *const mjLROpt, error: *mut i8, error_sz: i32) -> i32 {
    todo!() // cxx:_mj_setLengthRange
}

