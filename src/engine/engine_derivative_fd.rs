//! Port of: engine/engine_derivative_fd.c
//! IR hash: 9343293228317031
//! CODEGEN: source paths, owners, and callable names are locked.

use crate::types::*;

/// C: getState (engine/engine_derivative_fd.c:37)
/// Calls: cxx:_mj_getState, cxx:_mju_copy
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn getState(m: *const mjModel, d: *const mjData, state: *mut f64, sensordata: *mut f64) {
    const MJ_STATE_PHYSICS: i32 = 30;
    // SAFETY: caller guarantees m, d, state valid; sensordata valid if non-null
    unsafe {
        crate::engine::engine_support::mj_getState(m, d, state, MJ_STATE_PHYSICS);
        if !sensordata.is_null() {
            crate::engine::engine_util_blas::mju_copy(sensordata, (*d).sensordata, (*m).nsensordata as i32);
        }
    }
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

/// C: clampedDiff (engine/engine_derivative_fd.c:68)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative_fd.c:_diff, cxx:_mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn clampedDiff(dx: *mut f64, x: *const f64, x_plus: *const f64, x_minus: *const f64, h: f64, nx: i32) {
    use crate::engine::engine_util_blas::mju_zero;
    if !x_plus.is_null() && x_minus.is_null() {
        // forward differencing
        diff(dx, x_plus, x, h, nx);
    } else if x_plus.is_null() && !x_minus.is_null() {
        // backward differencing
        diff(dx, x, x_minus, h, nx);
    } else if !x_plus.is_null() && !x_minus.is_null() {
        // centered differencing
        diff(dx, x_plus, x_minus, 2.0 * h, nx);
    } else {
        // differencing failed, write zeros
        mju_zero(dx, nx);
    }
}

/// C: inRange (engine/engine_derivative_fd.c:106)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn inRange(x1: f64, x2: f64, range: *const f64) -> i32 {
    // SAFETY: range points to at least 2 f64 elements (caller contract)
    unsafe {
        if x1 >= *range.add(0) && x1 <= *range.add(1) && x2 >= *range.add(0) && x2 <= *range.add(1) {
            1
        } else {
            0
        }
    }
}

