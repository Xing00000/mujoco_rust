//! Port of: engine/engine_setconst.c
//! IR hash: 05737965add36adb
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mj_setM0 (engine/engine_setconst.c:37)
/// Calls: mj_actuatorArmature, mju_addTo, mju_copy, mju_dot, mju_mulInertVec
#[allow(unused_variables, non_snake_case)]
pub fn mj_set_m0(m: *mut mjModel, d: *mut mjData) {
    extern "C" { fn mj_setM0_impl(m: *mut mjModel, d: *mut mjData); }
    // SAFETY: delegates to C implementation
    unsafe { mj_setM0_impl(m, d) }
}

/// C: GetWrapBodyTreeId (engine/engine_setconst.c:64)
#[allow(unused_variables, non_snake_case)]
pub fn get_wrap_body_tree_id(m: *const mjModel, wrap_index: i32) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, wrap_index : i32)
    // Previous return: i32
    extern "C" { fn GetWrapBodyTreeId_impl (m : * const mjModel , wrap_index : i32) -> i32 ; } unsafe { GetWrapBodyTreeId_impl (m , wrap_index) }
}

/// C: setFixed (engine/engine_setconst.c:86)
/// Calls: GetWrapBodyTreeId, mj_freeStack, mj_markStack, mj_stackAllocInfo, mju_fillInt, mju_isZero, mju_message, mju_zeroInt
#[allow(unused_variables, non_snake_case)]
pub fn set_fixed(m: *mut mjModel, d: *mut mjData) {
    extern "C" { fn setFixed_impl(m: *mut mjModel, d: *mut mjData); }
    // SAFETY: delegates to C implementation
    unsafe { setFixed_impl(m, d) }
}

/// C: makeTendonSparse (engine/engine_setconst.c:337)
/// Calls: mju_zeroInt
#[allow(unused_variables, non_snake_case)]
pub fn make_tendon_sparse(m: *mut mjModel) {
    extern "C" { fn makeTendonSparse_impl(m: *mut mjModel); }
    // SAFETY: delegates to C implementation
    unsafe { makeTendonSparse_impl(m) }
}

/// C: makeFlexSparse (engine/engine_setconst.c:424)
/// Calls: mj_bodyChain, mj_freeStack, mj_jacDifPair, mj_markStack, mj_stackAllocInfo, mju_copyInt, mju_message, mju_sub3, mju_zero, mju_zeroInt
#[allow(unused_variables, non_snake_case)]
pub fn make_flex_sparse(m: *mut mjModel, d: *mut mjData) {
    extern "C" { fn makeFlexSparse_impl(m: *mut mjModel, d: *mut mjData); }
    // SAFETY: delegates to C implementation
    unsafe { makeFlexSparse_impl(m, d) }
}

/// C: mj_alignFlex (engine/engine_setconst.c:648)
/// Calls: mju_copy3, mju_cross, mju_mulMatTVec3, mju_normalize3, mju_quat2Mat, mju_quatZ2Vec, mju_sub3, mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn mj_align_flex(m: *mut mjModel, d: *mut mjData) {
    extern "C" { fn mj_alignFlex_impl(m: *mut mjModel, d: *mut mjData); }
    // SAFETY: delegates to C implementation
    unsafe { mj_alignFlex_impl(m, d) }
}

/// C: set0 (engine/engine_setconst.c:695)
/// Calls: makeFlexSparse, makeTendonSparse, mj_alignFlex, mj_camlight, mj_comPos, mj_factorM, mj_flex, mj_freeStack, mj_jacBodyCom, mj_kinematics, mj_local2Global, mj_makeM, mj_markStack, mj_setM0, mj_solveM, mj_stackAllocInfo, mj_tendon, mj_transmission, mju_copy, mju_copy3, mju_copy9, mju_dot, mju_isZero, mju_max, mju_message, mju_mulMatMatT, mju_mulMatTVec3, mju_mulQuat, mju_negQuat, mju_norm, mju_normalize4, mju_sparse2dense, mju_sub3, mju_subFrom3, mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn set0(m: *mut mjModel, d: *mut mjData) {
    extern "C" { fn set0_impl(m: *mut mjModel, d: *mut mjData); }
    // SAFETY: delegates to C implementation
    unsafe { set0_impl(m, d) }
}

/// C: updateBox (engine/engine_setconst.c:1041)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn update_box(xmin: *mut f64, xmax: *mut f64, pos: *mut f64, radius: f64) {
    extern "C" { fn updateBox_impl(xmin: *mut f64, xmax: *mut f64, pos: *mut f64, radius: f64); }
    // SAFETY: delegates to C implementation
    unsafe { updateBox_impl(xmin, xmax, pos, radius) }
}

/// C: setStat (engine/engine_setconst.c:1050)
/// Calls: mj_freeStack, mj_markStack, mj_stackAllocInfo, mju_add3, mju_dist3, mju_max, mju_scl3, mju_zero, updateBox
#[allow(unused_variables, non_snake_case)]
pub fn set_stat(m: *mut mjModel, d: *mut mjData) {
    extern "C" { fn setStat_impl(m: *mut mjModel, d: *mut mjData); }
    // SAFETY: delegates to C implementation
    unsafe { setStat_impl(m, d) }
}

/// C: setSpring (engine/engine_setconst.c:1198)
/// Calls: mj_comPos, mj_kinematics, mj_tendon, mj_transmission, mju_copy
#[allow(unused_variables, non_snake_case)]
pub fn set_spring(m: *mut mjModel, d: *mut mjData) {
    extern "C" { fn setSpring_impl(m: *mut mjModel, d: *mut mjData); }
    // SAFETY: delegates to C implementation
    unsafe { setSpring_impl(m, d) }
}

/// C: evalAct (engine/engine_setconst.c:1235)
/// Calls: mj_freeStack, mj_markStack, mj_solveM, mj_stackAllocInfo, mj_step1, mj_step2, mju_norm, mju_scl, mju_sparse2dense
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn eval_act(m: *const mjModel, d: *mut mjData, index: i32, side: i32, opt: *const mjLROpt) -> f64 {
    extern "C" { fn evalAct_impl(m: *const mjModel, d: *mut mjData, index: i32, side: i32, opt: *const mjLROpt) -> f64; }
    // SAFETY: delegates to C implementation
    unsafe { evalAct_impl(m, d, index, side, opt) }
}

/// C: mj_setConst (engine/engine_setconst.h:27)
/// Calls: set0, setFixed, setSpring, setStat
#[allow(unused_variables, non_snake_case)]
pub fn mj_set_const(m: *mut mjModel, d: *mut mjData) {
    extern "C" { fn mj_setConst_impl(m: *mut mjModel, d: *mut mjData); }
    // SAFETY: delegates to C implementation
    unsafe { mj_setConst_impl(m, d) }
}

/// C: mj_setLengthRange (engine/engine_setconst.h:30)
/// Calls: evalAct, mj_resetData, mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_set_length_range(m: *mut mjModel, d: *mut mjData, index: i32, opt: *const mjLROpt, error: *mut i8, error_sz: i32) -> i32 {
    extern "C" { fn mj_setLengthRange_impl(m: *mut mjModel, d: *mut mjData, index: i32, opt: *const mjLROpt, error: *mut i8, error_sz: i32) -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { mj_setLengthRange_impl(m, d, index, opt, error, error_sz) }
}

