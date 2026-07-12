//! Port of: engine/engine_derivative_fd.c
//! IR hash: c6d98e4f4b63b7f2
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: getState (engine/engine_derivative_fd.c:37)
/// Calls: mj_getState, mju_copy
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn get_state(m: *const mjModel, d: *const mjData, state: *mut f64, sensordata: *mut f64) {
    // NOTE: signature changed from previous IR version
    // Previous params: (m : * const mjModel, d : * const mjData, state : * mut f64, sensordata : * mut f64)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: diff (engine/engine_derivative_fd.c:46)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn diff(dx: *mut f64, x1: *const f64, x2: *const f64, h: f64, n: i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (dx : * mut f64, x1 : * const f64, x2 : * const f64, h : f64, n : i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: stateDiff (engine/engine_derivative_fd.c:55)
/// Calls: diff, mj_differentiatePos
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn state_diff(m: *const mjModel, ds: *mut f64, s1: *const f64, s2: *const f64, h: f64) {
    // NOTE: signature changed from previous IR version
    // Previous params: (m : * const mjModel, ds : * mut f64, s1 : * const f64, s2 : * const f64, h : f64)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: clampedDiff (engine/engine_derivative_fd.c:68)
/// Calls: diff, mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn clamped_diff(dx: *mut f64, x: *const f64, x_plus: *const f64, x_minus: *const f64, h: f64, nx: i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (dx : * mut f64, x : * const f64, x_plus : * const f64, x_minus : * const f64, h : f64, nx : i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: clampedStateDiff (engine/engine_derivative_fd.c:87)
/// Calls: mju_zero, stateDiff
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn clamped_state_diff(m: *const mjModel, ds: *mut f64, s: *const f64, s_plus: *const f64, s_minus: *const f64, h: f64) {
    // NOTE: signature changed from previous IR version
    // Previous params: (m : * const mjModel, ds : * mut f64, s : * const f64, s_plus : * const f64, s_minus : * const f64, h : f64)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: inRange (engine/engine_derivative_fd.c:106)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn in_range(x1: f64, x2: f64, range: *const f64) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (x1 : f64, x2 : f64, range : * const f64)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: inverseSkip (engine/engine_derivative_fd.c:152)
/// Calls: mj_fwdActuation, mj_inverseSkip, mju_copy, mju_subFrom
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn inverse_skip(m: *const mjModel, d: *mut mjData, stage: u32, skipsensor: i32, flg_actuation: i32, force: *mut f64) {
    // NOTE: signature changed from previous IR version
    // Previous params: (m : * const mjModel, d : * mut mjData, stage : u32, skipsensor : i32, flg_actuation : i32, force : * mut f64)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjd_stepFD (engine/engine_derivative_fd.c:295)
/// Calls: clampedDiff, clampedStateDiff, diff, getState, inRange, mj_freeStack, mj_getState, mj_integratePos, mj_markStack, mj_setState, mj_stackAllocInfo, mj_stateSize, mj_stepSkip, mju_copy, mju_message, mju_zero, stateDiff
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjd_step_fd(m: *const mjModel, d: *mut mjData, eps: f64, flg_centered: mjtBool, DyDq: *mut f64, DyDv: *mut f64, DyDa: *mut f64, DyDu: *mut f64, DsDq: *mut f64, DsDv: *mut f64, DsDa: *mut f64, DsDu: *mut f64) {
    // NOTE: signature changed from previous IR version
    // Previous params: (m : * const mjModel, d : * mut mjData, eps : f64, flg_centered : mjtBool, DyDq : * mut f64, DyDv : * mut f64, DyDa : * mut f64, DyDu : * mut f64, DsDq : * mut f64, DsDv : * mut f64, DsDa : * mut f64, DsDu : * mut f64)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjd_smooth_velFD (engine/engine_derivative_fd.h:27)
/// Calls: mj_freeStack, mj_fwdActuation, mj_fwdVelocity, mj_markStack, mj_stackAllocInfo, mju_add, mju_message, mju_scl, mju_sub, mju_subFrom, mju_zeroInt
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjd_smooth_vel_fd(m: *const mjModel, d: *mut mjData, eps: f64) {
    // NOTE: signature changed from previous IR version
    // Previous params: (m : * const mjModel, d : * mut mjData, eps : f64)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjd_passive_velFD (engine/engine_derivative_fd.h:30)
/// Calls: mj_freeStack, mj_fwdVelocity, mj_markStack, mj_stackAllocInfo, mju_copy, mju_scl, mju_sub, mju_zeroInt
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjd_passive_vel_fd(m: *const mjModel, d: *mut mjData, eps: f64) {
    // NOTE: signature changed from previous IR version
    // Previous params: (m : * const mjModel, d : * mut mjData, eps : f64)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mj_stepSkip (engine/engine_derivative_fd.h:33)
/// Calls: mj_EulerSkip, mj_RungeKutta, mj_checkAcc, mj_checkPos, mj_checkVel, mj_compareFwdInv, mj_forwardSkip, mj_implicitSkip, mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_step_skip(m: *const mjModel, d: *mut mjData, skipstage: i32, skipsensor: i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (m : * const mjModel, d : * mut mjData, skipstage : i32, skipsensor : i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjd_transitionFD (engine/engine_derivative_fd.h:36)
/// Calls: mj_freeStack, mj_markStack, mj_stackAllocInfo, mjd_stepFD, mju_message, mju_transpose
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjd_transition_fd(m: *const mjModel, d: *mut mjData, eps: f64, centered: mjtBool, A: *mut f64, B: *mut f64, C: *mut f64, D: *mut f64) {
    // NOTE: signature changed from previous IR version
    // Previous params: (m : * const mjModel, d : * mut mjData, eps : f64, centered : mjtBool, A : * mut f64, B : * mut f64, C : * mut f64, D : * mut f64)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjd_inverseFD (engine/engine_derivative_fd.h:40)
/// Calls: diff, inverseSkip, mj_freeStack, mj_integratePos, mj_markStack, mj_stackAllocInfo, mju_copy, mju_message, mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjd_inverse_fd(m: *const mjModel, d: *mut mjData, eps: f64, flg_actuation: mjtBool, DfDq: *mut f64, DfDv: *mut f64, DfDa: *mut f64, DsDq: *mut f64, DsDv: *mut f64, DsDa: *mut f64, DmDq: *mut f64) {
    // NOTE: signature changed from previous IR version
    // Previous params: (m : * const mjModel, d : * mut mjData, eps : f64, flg_actuation : mjtBool, DfDq : * mut f64, DfDv : * mut f64, DfDa : * mut f64, DsDq : * mut f64, DsDv : * mut f64, DsDa : * mut f64, DmDq : * mut f64)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

