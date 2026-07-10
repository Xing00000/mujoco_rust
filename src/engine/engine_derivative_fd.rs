//! Port of: engine/engine_derivative_fd.c
//! IR hash: 05737965add36adb
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
    // mjSTATE_PHYSICS = mjSTATE_QPOS | mjSTATE_QVEL | mjSTATE_ACT | mjSTATE_HISTORY
    const mjSTATE_PHYSICS: i32 = (1 << 1) | (1 << 2) | (1 << 3) | (1 << 4);
    // SAFETY: m, d, state valid per caller contract. sensordata may be null.
    unsafe {
        crate::engine::engine_support::mj_get_state(m, d, state, mjSTATE_PHYSICS);
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
    // SAFETY: caller guarantees dx, x1, x2 point to valid arrays of at least n elements
    unsafe {
        let inv_h: f64 = 1.0 / h;
        for i in 0..n as usize {
            *dx.add(i) = inv_h * (*x2.add(i) - *x1.add(i));
        }
    }
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
    // SAFETY: m valid, ds/s1/s2 point to arrays of at least nq+nv+na elements.
    unsafe {
        let nq = (*m).nq as i32;
        let nv = (*m).nv as i32;
        let na = (*m).na as i32;

        if nq == nv {
            diff(ds, s1, s2, h, nq + nv + na);
        } else {
            crate::engine::engine_support::mj_differentiate_pos(m, ds, h, s1, s2);
            diff(ds.add(nv as usize), s1.add(nq as usize), s2.add(nq as usize), h, nv + na);
        }
    }
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
    // SAFETY: caller guarantees dx points to valid array of at least nx elements.
    //         x, x_plus, x_minus are either null or point to valid arrays of at least nx elements.
    if !x_plus.is_null() && x_minus.is_null() {
        // forward differencing
        diff(dx, x, x_plus, h, nx);
    } else if x_plus.is_null() && !x_minus.is_null() {
        // backward differencing
        diff(dx, x_minus, x, h, nx);
    } else if !x_plus.is_null() && !x_minus.is_null() {
        // centered differencing
        diff(dx, x_plus, x_minus, 2.0 * h, nx);
    } else {
        // differencing failed, write zeros
        crate::engine::engine_util_blas::mju_zero(dx, nx);
    }
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
    // SAFETY: m valid, ds/s/s_plus/s_minus are valid arrays (or null for s_plus/s_minus).
    unsafe {
        if !s_plus.is_null() && s_minus.is_null() {
            // forward differencing
            state_diff(m, ds, s, s_plus, h);
        } else if s_plus.is_null() && !s_minus.is_null() {
            // backward differencing
            state_diff(m, ds, s_minus, s, h);
        } else if !s_plus.is_null() && !s_minus.is_null() {
            // centered differencing
            state_diff(m, ds, s_minus, s_plus, 2.0 * h);
        } else {
            // differencing failed, write zeros
            crate::engine::engine_util_blas::mju_zero(ds, 2 * (*m).nv as i32 + (*m).na as i32);
        }
    }
}

/// C: inRange (engine/engine_derivative_fd.c:106)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn in_range(x1: f64, x2: f64, range: *const f64) -> i32 {
    // SAFETY: caller guarantees range points to a valid array of at least 2 elements.
    unsafe {
        if x1 >= *range.add(0) && x1 <= *range.add(1) &&
           x2 >= *range.add(0) && x2 <= *range.add(1) {
            1
        } else {
            0
        }
    }
}

/// C: inverseSkip (engine/engine_derivative_fd.c:152)
/// Calls: mj_fwdActuation, mj_inverseSkip, mju_copy, mju_subFrom
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn inverse_skip(m: *const mjModel, d: *mut mjData, stage: mjtStage, skipsensor: i32, flg_actuation: i32, force: *mut f64) {
    extern "C" { fn inverseSkip(m: *const mjModel, d: *mut mjData, stage: mjtStage, skipsensor: i32, flg_actuation: i32, force: *mut f64); }
    // SAFETY: delegates to C implementation
    unsafe { inverseSkip(m, d, stage, skipsensor, flg_actuation, force) }
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
    extern "C" { fn mjd_stepFD(m: *const mjModel, d: *mut mjData, eps: f64, flg_centered: mjtBool, DyDq: *mut f64, DyDv: *mut f64, DyDa: *mut f64, DyDu: *mut f64, DsDq: *mut f64, DsDv: *mut f64, DsDa: *mut f64, DsDu: *mut f64); }
    // SAFETY: delegates to C implementation
    unsafe { mjd_stepFD(m, d, eps, flg_centered, DyDq, DyDv, DyDa, DyDu, DsDq, DsDv, DsDa, DsDu) }
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
    // SAFETY: m, d valid. Stack alloc from mjData arena. All blas/forward functions valid.
    unsafe {
        let nv: i32 = (*m).nv as i32;

        crate::engine::engine_memory::mj_mark_stack(d);
        extern "C" {
            fn mj_stackAllocInfo(d: *mut mjData, bytes: usize, alignment: usize, caller: *const i8, line: i32) -> *mut ();
        }
        let plus: *mut f64 = mj_stackAllocInfo(
            d, (nv as usize) * std::mem::size_of::<f64>(), std::mem::align_of::<f64>(),
            b"mjd_smooth_velFD\0".as_ptr() as *const i8, 0,
        ) as *mut f64;
        let minus: *mut f64 = mj_stackAllocInfo(
            d, (nv as usize) * std::mem::size_of::<f64>(), std::mem::align_of::<f64>(),
            b"mjd_smooth_velFD\0".as_ptr() as *const i8, 0,
        ) as *mut f64;
        let fd: *mut f64 = mj_stackAllocInfo(
            d, (nv as usize) * std::mem::size_of::<f64>(), std::mem::align_of::<f64>(),
            b"mjd_smooth_velFD\0".as_ptr() as *const i8, 0,
        ) as *mut f64;
        let cnt: *mut i32 = mj_stackAllocInfo(
            d, (nv as usize) * std::mem::size_of::<i32>(), std::mem::align_of::<i32>(),
            b"mjd_smooth_velFD\0".as_ptr() as *const i8, 0,
        ) as *mut i32;

        // clear row counters
        crate::engine::engine_util_misc::mju_zero_int(cnt, nv);

        // loop over dofs
        let mut i: i32 = 0;
        while i < nv {
            // save qvel[i]
            let saveqvel: f64 = *(*d).qvel.add(i as usize);

            // eval at qvel[i]+eps
            *(*d).qvel.add(i as usize) = saveqvel + eps;
            crate::engine::engine_forward::mj_fwd_velocity(m, d);
            crate::engine::engine_forward::mj_fwd_actuation(m, d);
            crate::engine::engine_util_blas::mju_add(plus, (*d).qfrc_actuator, (*d).qfrc_passive, nv);
            crate::engine::engine_util_blas::mju_sub_from(plus, (*d).qfrc_bias, nv);

            // eval at qvel[i]-eps
            *(*d).qvel.add(i as usize) = saveqvel - eps;
            crate::engine::engine_forward::mj_fwd_velocity(m, d);
            crate::engine::engine_forward::mj_fwd_actuation(m, d);
            crate::engine::engine_util_blas::mju_add(minus, (*d).qfrc_actuator, (*d).qfrc_passive, nv);
            crate::engine::engine_util_blas::mju_sub_from(minus, (*d).qfrc_bias, nv);

            // restore qvel[i]
            *(*d).qvel.add(i as usize) = saveqvel;

            // finite difference result in fd
            crate::engine::engine_util_blas::mju_sub(fd, plus, minus, nv);
            crate::engine::engine_util_blas::mju_scl(fd, fd, 0.5 / eps, nv);

            // copy to sparse qDeriv
            let mut j: i32 = 0;
            while j < nv {
                if *cnt.add(j as usize) < *(*m).D_rownnz.add(j as usize)
                    && *(*m).D_colind.add((*(*m).D_rowadr.add(j as usize) + *cnt.add(j as usize)) as usize) == i
                {
                    *(*d).qDeriv.add((*(*m).D_rowadr.add(j as usize) + *cnt.add(j as usize)) as usize) = *fd.add(j as usize);
                    *cnt.add(j as usize) += 1;
                }
                j += 1;
            }

            i += 1;
        }

        // make sure final row counters equal rownnz
        i = 0;
        while i < nv {
            if *cnt.add(i as usize) != *(*m).D_rownnz.add(i as usize) {
                crate::engine::engine_util_errmem::mju_error(
                    b"error in constructing FD sparse derivative\0".as_ptr() as *const i8,
                );
            }
            i += 1;
        }

        // restore
        crate::engine::engine_forward::mj_fwd_velocity(m, d);
        crate::engine::engine_forward::mj_fwd_actuation(m, d);

        crate::engine::engine_memory::mj_free_stack(d);
    }
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
    // SAFETY: m, d valid. Stack alloc from mjData arena. All blas/forward functions valid.
    unsafe {
        let nv: i32 = (*m).nv as i32;

        crate::engine::engine_memory::mj_mark_stack(d);
        extern "C" {
            fn mj_stackAllocInfo(d: *mut mjData, bytes: usize, alignment: usize, caller: *const i8, line: i32) -> *mut ();
        }
        let qfrc_passive: *mut f64 = mj_stackAllocInfo(
            d, (nv as usize) * std::mem::size_of::<f64>(), std::mem::align_of::<f64>(),
            b"mjd_passive_velFD\0".as_ptr() as *const i8, 0,
        ) as *mut f64;
        let fd: *mut f64 = mj_stackAllocInfo(
            d, (nv as usize) * std::mem::size_of::<f64>(), std::mem::align_of::<f64>(),
            b"mjd_passive_velFD\0".as_ptr() as *const i8, 0,
        ) as *mut f64;
        let cnt: *mut i32 = mj_stackAllocInfo(
            d, (nv as usize) * std::mem::size_of::<i32>(), std::mem::align_of::<i32>(),
            b"mjd_passive_velFD\0".as_ptr() as *const i8, 0,
        ) as *mut i32;

        // clear row counters
        crate::engine::engine_util_misc::mju_zero_int(cnt, nv);

        // save qfrc_passive, assume mj_fwdVelocity was called
        crate::engine::engine_util_blas::mju_copy(qfrc_passive, (*d).qfrc_passive, nv);

        // loop over dofs
        let mut i: i32 = 0;
        while i < nv {
            // save qvel[i]
            let saveqvel: f64 = *(*d).qvel.add(i as usize);

            // eval at qvel[i]+eps
            *(*d).qvel.add(i as usize) = saveqvel + eps;
            crate::engine::engine_forward::mj_fwd_velocity(m, d);

            // restore qvel[i]
            *(*d).qvel.add(i as usize) = saveqvel;

            // finite difference result in fd
            crate::engine::engine_util_blas::mju_sub(fd, (*d).qfrc_passive, qfrc_passive, nv);
            crate::engine::engine_util_blas::mju_scl(fd, fd, 1.0 / eps, nv);

            // copy to i-th column of qDeriv
            let mut j: i32 = 0;
            while j < nv {
                let adr: i32 = *(*m).D_rowadr.add(j as usize) + *cnt.add(j as usize);
                if *cnt.add(j as usize) < *(*m).D_rownnz.add(j as usize)
                    && *(*m).D_colind.add(adr as usize) == i
                {
                    *(*d).qDeriv.add(adr as usize) = *fd.add(j as usize);
                    *cnt.add(j as usize) += 1;
                }
                j += 1;
            }

            i += 1;
        }

        // restore
        crate::engine::engine_forward::mj_fwd_velocity(m, d);

        crate::engine::engine_memory::mj_free_stack(d);
    }
}

/// C: mj_stepSkip (engine/engine_derivative_fd.h:33)
/// Calls: mj_EulerSkip, mj_RungeKutta, mj_checkAcc, mj_checkPos, mj_checkVel, mj_compareFwdInv, mj_forwardSkip, mj_implicitSkip, mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_step_skip(m: *const mjModel, d: *mut mjData, skipstage: i32, skipsensor: i32) {
    extern "C" {
        fn mj_stepSkip(m: *const mjModel, d: *mut mjData, skipstage: i32, skipsensor: i32);
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_stepSkip(m, d, skipstage, skipsensor) }
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
    extern "C" { fn mjd_transitionFD(m: *const mjModel, d: *mut mjData, eps: f64, centered: mjtBool, A: *mut f64, B: *mut f64, C: *mut f64, D: *mut f64); }
    // SAFETY: delegates to C implementation
    unsafe { mjd_transitionFD(m, d, eps, centered, A, B, C, D) }
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
    extern "C" { fn mjd_inverseFD(m: *const mjModel, d: *mut mjData, eps: f64, flg_actuation: mjtBool, DfDq: *mut f64, DfDv: *mut f64, DfDa: *mut f64, DsDq: *mut f64, DsDv: *mut f64, DsDa: *mut f64, DmDq: *mut f64); }
    // SAFETY: delegates to C implementation
    unsafe { mjd_inverseFD(m, d, eps, flg_actuation, DfDq, DfDv, DfDa, DsDq, DsDv, DsDa, DmDq) }
}

