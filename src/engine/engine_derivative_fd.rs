//! Port of: engine/engine_derivative_fd.c
//! IR hash: fd5288ad7242f001
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: getState (engine/engine_derivative_fd.c:37)
/// Calls: cxx:_mj_getState, cxx:_mju_copy
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn get_state(m: *const mjModel, d: *const mjData, state: *mut f64, sensordata: *mut f64) {
    todo!() // cxx-internal:engine_derivative_fd.c.o:_getState
}

/// C: diff (engine/engine_derivative_fd.c:46)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn diff(dx: *mut f64, x1: *const f64, x2: *const f64, h: f64, n: i32) {
    // SAFETY: caller guarantees dx, x1, x2 point to arrays of at least n elements
    unsafe {
        for i in 0..n as usize {
            *dx.add(i) = (*x1.add(i) - *x2.add(i)) / h;
        }
    }
}

/// C: stateDiff (engine/engine_derivative_fd.c:55)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative_fd.c:_diff, cxx:_mj_differentiatePos
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn state_diff(m: *const mjModel, ds: *mut f64, s1: *const f64, s2: *const f64, h: f64) {
    todo!() // cxx-internal:engine_derivative_fd.c.o:_stateDiff
}

/// C: clampedDiff (engine/engine_derivative_fd.c:68)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative_fd.c:_diff, cxx:_mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn clamped_diff(dx: *mut f64, x: *const f64, x_plus: *const f64, x_minus: *const f64, h: f64, nx: i32) {
    todo!() // cxx-internal:engine_derivative_fd.c.o:_clampedDiff
}

/// C: clampedStateDiff (engine/engine_derivative_fd.c:87)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative_fd.c:_stateDiff, cxx:_mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn clamped_state_diff(m: *const mjModel, ds: *mut f64, s: *const f64, s_plus: *const f64, s_minus: *const f64, h: f64) {
    todo!() // cxx-internal:engine_derivative_fd.c.o:_clampedStateDiff
}

/// C: inRange (engine/engine_derivative_fd.c:106)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn in_range(x1: f64, x2: f64, range: *const f64) -> i32 {
    todo!() // cxx-internal:engine_derivative_fd.c.o:_inRange
}

/// C: mj_stepSkip (engine/engine_derivative_fd.c:113)
/// Calls: cxx:_mj_EulerSkip, cxx:_mj_RungeKutta, cxx:_mj_checkAcc, cxx:_mj_checkPos, cxx:_mj_checkVel, cxx:_mj_compareFwdInv, cxx:_mj_discrete, cxx:_mj_forwardSkip, cxx:_mj_implicitSkip, cxx:_mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_step_skip(m: *const mjModel, d: *mut mjData, skipstage: i32, skipsensor: i32) {
    todo!() // cxx:_mj_stepSkip
}

/// C: inverseSkip (engine/engine_derivative_fd.c:157)
/// Calls: cxx:_mj_fwdActuation, cxx:_mj_inverseSkip, cxx:_mju_copy, cxx:_mju_subFrom
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn inverse_skip(m: *const mjModel, d: *mut mjData, stage: u32, skipsensor: i32, flg_actuation: i32, force: *mut f64) {
    todo!() // cxx-internal:engine_derivative_fd.c.o:_inverseSkip
}

/// C: mjd_passive_velFD (engine/engine_derivative_fd.c:171)
/// Calls: cxx:_mj_freeStack, cxx:_mj_fwdVelocity, cxx:_mj_markStack, cxx:_mj_stackAllocInfo, cxx:_mju_copy, cxx:_mju_scl, cxx:_mju_sub, cxx:_mju_zeroInt
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjd_passive_vel_fd(m: *const mjModel, d: *mut mjData, eps: f64) {
    todo!() // cxx:_mjd_passive_velFD
}

/// C: mjd_smooth_velFD (engine/engine_derivative_fd.c:221)
/// Calls: cxx:_mj_freeStack, cxx:_mj_fwdActuation, cxx:_mj_fwdVelocity, cxx:_mj_markStack, cxx:_mj_stackAllocInfo, cxx:_mju_add, cxx:_mju_message, cxx:_mju_scl, cxx:_mju_sub, cxx:_mju_subFrom, cxx:_mju_zeroInt
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjd_smooth_vel_fd(m: *const mjModel, d: *mut mjData, eps: f64) {
    todo!() // cxx:_mjd_smooth_velFD
}

/// C: mjd_stepFD (engine/engine_derivative_fd.c:300)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative_fd.c:_clampedDiff, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative_fd.c:_clampedStateDiff, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative_fd.c:_diff, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative_fd.c:_getState, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative_fd.c:_inRange, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative_fd.c:_stateDiff, cxx:_mj_freeStack, cxx:_mj_getState, cxx:_mj_integratePos, cxx:_mj_markStack, cxx:_mj_setState, cxx:_mj_stackAllocInfo, cxx:_mj_stateSize, cxx:_mj_stepSkip, cxx:_mju_copy, cxx:_mju_message, cxx:_mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjd_step_fd(m: *const mjModel, d: *mut mjData, eps: f64, flg_centered: bool, DyDq: *mut f64, DyDv: *mut f64, DyDa: *mut f64, DyDu: *mut f64, DsDq: *mut f64, DsDv: *mut f64, DsDa: *mut f64, DsDu: *mut f64) {
    todo!() // cxx:_mjd_stepFD
}

/// C: mjd_transitionFD (engine/engine_derivative_fd.c:548)
/// Calls: cxx:_mj_freeStack, cxx:_mj_markStack, cxx:_mj_stackAllocInfo, cxx:_mjd_stepFD, cxx:_mju_message, cxx:_mju_transpose
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjd_transition_fd(m: *const mjModel, d: *mut mjData, eps: f64, centered: bool, A: *mut f64, B: *mut f64, C: *mut f64, D: *mut f64) {
    todo!() // cxx:_mjd_transitionFD
}

/// C: mjd_inverseFD (engine/engine_derivative_fd.c:616)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative_fd.c:_diff, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative_fd.c:_inverseSkip, cxx:_mj_freeStack, cxx:_mj_integratePos, cxx:_mj_markStack, cxx:_mj_stackAllocInfo, cxx:_mju_copy, cxx:_mju_message, cxx:_mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjd_inverse_fd(m: *const mjModel, d: *mut mjData, eps: f64, flg_actuation: bool, DfDq: *mut f64, DfDv: *mut f64, DfDa: *mut f64, DsDq: *mut f64, DsDv: *mut f64, DsDa: *mut f64, DmDq: *mut f64) {
    todo!() // cxx:_mjd_inverseFD
}

