//! Port of: engine/engine_derivative_fd.c
//! IR hash: 73393814548a07d1
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
    const MJ_STATE_PHYSICS: i32 = 30;
    // SAFETY: caller guarantees m, d, state valid; sensordata valid if non-null
    unsafe {
        crate::engine::engine_support::mj_get_state(m, d, state, MJ_STATE_PHYSICS);
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

/// C: stateDiff (engine/engine_derivative_fd.c:55)
/// Calls: diff, mj_differentiatePos
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn state_diff(m: *const mjModel, ds: *mut f64, s1: *const f64, s2: *const f64, h: f64) {
    // SAFETY: caller guarantees m, ds, s1, s2 valid with sizes nq+nv+na
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

/// C: clampedStateDiff (engine/engine_derivative_fd.c:87)
/// Calls: mju_zero, stateDiff
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn clamped_state_diff(m: *const mjModel, ds: *mut f64, s: *const f64, s_plus: *const f64, s_minus: *const f64, h: f64) {
    // SAFETY: caller guarantees all pointers valid with proper sizes
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
    // SAFETY: range points to at least 2 f64 elements (caller contract)
    unsafe {
        if x1 >= *range.add(0) && x1 <= *range.add(1) && x2 >= *range.add(0) && x2 <= *range.add(1) {
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
pub fn inverse_skip(m: *const mjModel, d: *mut mjData, stage: u32, skipsensor: i32, flg_actuation: i32, force: *mut f64) {
    // SAFETY: m, d, force are valid pointers (caller contract).
    unsafe {
        crate::engine::engine_inverse::mj_inverse_skip(m, d, stage as i32, skipsensor);
        crate::engine::engine_util_blas::mju_copy(force, (*d).qfrc_inverse, (*m).nv as i32);
        if flg_actuation != 0 {
            crate::engine::engine_forward::mj_fwd_actuation(m, d);
            crate::engine::engine_util_blas::mju_sub_from(force, (*d).qfrc_actuator, (*m).nv as i32);
        }
    }
}

/// C: mjd_stepFD (engine/engine_derivative_fd.c:295)
/// Calls: clampedDiff, clampedStateDiff, diff, getState, inRange, mj_freeStack, mj_getState, mj_integratePos, mj_markStack, mj_setState, mj_stackAllocInfo, mj_stateSize, mj_stepSkip, mju_copy, mju_message, mju_zero, stateDiff
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjd_step_fd(m: *const mjModel, d: *mut mjData, eps: f64, flg_centered: bool, DyDq: *mut f64, DyDv: *mut f64, DyDa: *mut f64, DyDu: *mut f64, DsDq: *mut f64, DsDv: *mut f64, DsDa: *mut f64, DsDu: *mut f64) {
    todo!() // mjd_stepFD
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
    // SAFETY: m and d are valid pointers with all arrays allocated (caller contract)
    unsafe {
        let nv = (*m).nv as i32;

        crate::engine::engine_memory::mj_mark_stack(d);
        let plus = crate::engine::engine_memory::mj_stack_alloc_num(d, nv as usize);
        let minus = crate::engine::engine_memory::mj_stack_alloc_num(d, nv as usize);
        let fd = crate::engine::engine_memory::mj_stack_alloc_num(d, nv as usize);
        let cnt = crate::engine::engine_memory::mj_stack_alloc_int(d, nv as usize);

        // clear row counters
        crate::engine::engine_util_misc::mju_zero_int(cnt, nv);

        // loop over dofs
        for i in 0..nv as usize {
            // save qvel[i]
            let saveqvel = *(*d).qvel.add(i);

            // eval at qvel[i]+eps
            *(*d).qvel.add(i) = saveqvel + eps;
            crate::engine::engine_forward::mj_fwd_velocity(m, d);
            crate::engine::engine_forward::mj_fwd_actuation(m, d);
            crate::engine::engine_util_blas::mju_add(plus, (*d).qfrc_actuator, (*d).qfrc_passive, nv);
            crate::engine::engine_util_blas::mju_sub_from(plus, (*d).qfrc_bias, nv);

            // eval at qvel[i]-eps
            *(*d).qvel.add(i) = saveqvel - eps;
            crate::engine::engine_forward::mj_fwd_velocity(m, d);
            crate::engine::engine_forward::mj_fwd_actuation(m, d);
            crate::engine::engine_util_blas::mju_add(minus, (*d).qfrc_actuator, (*d).qfrc_passive, nv);
            crate::engine::engine_util_blas::mju_sub_from(minus, (*d).qfrc_bias, nv);

            // restore qvel[i]
            *(*d).qvel.add(i) = saveqvel;

            // finite difference result in fd
            crate::engine::engine_util_blas::mju_sub(fd, plus, minus, nv);
            crate::engine::engine_util_blas::mju_scl(fd, fd, 0.5 / eps, nv);

            // copy to sparse qDeriv
            for j in 0..nv as usize {
                if *cnt.add(j) < *(*m).D_rownnz.add(j)
                    && *(*m).D_colind.add((*(*m).D_rowadr.add(j) + *cnt.add(j)) as usize) == i as i32
                {
                    *(*d).qDeriv.add((*(*m).D_rowadr.add(j) + *cnt.add(j)) as usize) = *fd.add(j);
                    *cnt.add(j) += 1;
                }
            }
        }

        // make sure final row counters equal rownnz
        for i in 0..nv as usize {
            if *cnt.add(i) != *(*m).D_rownnz.add(i) {
                crate::engine::engine_util_errmem::mju_error(
                    b"error in constructing FD sparse derivative\0".as_ptr() as *const i8);
            }
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
    // SAFETY: m and d are valid pointers with all arrays allocated (caller contract)
    unsafe {
        let nv = (*m).nv as i32;

        crate::engine::engine_memory::mj_mark_stack(d);
        let qfrc_passive = crate::engine::engine_memory::mj_stack_alloc_num(d, nv as usize);
        let fd = crate::engine::engine_memory::mj_stack_alloc_num(d, nv as usize);
        let cnt = crate::engine::engine_memory::mj_stack_alloc_int(d, nv as usize);

        // clear row counters
        crate::engine::engine_util_misc::mju_zero_int(cnt, nv);

        // save qfrc_passive, assume mj_fwdVelocity was called
        crate::engine::engine_util_blas::mju_copy(qfrc_passive, (*d).qfrc_passive, nv);

        // loop over dofs
        for i in 0..nv as usize {
            // save qvel[i]
            let saveqvel = *(*d).qvel.add(i);

            // eval at qvel[i]+eps
            *(*d).qvel.add(i) = saveqvel + eps;
            crate::engine::engine_forward::mj_fwd_velocity(m, d);

            // restore qvel[i]
            *(*d).qvel.add(i) = saveqvel;

            // finite difference result in fd
            crate::engine::engine_util_blas::mju_sub(fd, (*d).qfrc_passive, qfrc_passive, nv);
            crate::engine::engine_util_blas::mju_scl(fd, fd, 1.0 / eps, nv);

            // copy to i-th column of qDeriv
            for j in 0..nv as usize {
                let adr = (*(*m).D_rowadr.add(j) + *cnt.add(j)) as usize;
                if (*cnt.add(j) < *(*m).D_rownnz.add(j)) && (*(*m).D_colind.add(adr) == i as i32) {
                    *(*d).qDeriv.add(adr) = *fd.add(j);
                    *cnt.add(j) += 1;
                }
            }
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
    todo!() // mj_stepSkip
}

/// C: mjd_transitionFD (engine/engine_derivative_fd.h:36)
/// Calls: mj_freeStack, mj_markStack, mj_stackAllocInfo, mjd_stepFD, mju_message, mju_transpose
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjd_transition_fd(m: *const mjModel, d: *mut mjData, eps: f64, centered: bool, A: *mut f64, B: *mut f64, C: *mut f64, D: *mut f64) {
    todo!() // mjd_transitionFD
}

/// C: mjd_inverseFD (engine/engine_derivative_fd.h:40)
/// Calls: diff, inverseSkip, mj_freeStack, mj_integratePos, mj_markStack, mj_stackAllocInfo, mju_copy, mju_message, mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjd_inverse_fd(m: *const mjModel, d: *mut mjData, eps: f64, flg_actuation: bool, DfDq: *mut f64, DfDv: *mut f64, DfDa: *mut f64, DsDq: *mut f64, DsDv: *mut f64, DsDa: *mut f64, DmDq: *mut f64) {
    const MJ_INT_RK4: i32 = 1;

    // SAFETY: m and d are valid pointers with all arrays allocated (caller contract)
    unsafe {
        let nq = (*m).nq as i32;
        let nv = (*m).nv as i32;
        let nM = (*m).nM as i32;
        let ns = (*m).nsensordata as i32;

        if (*m).opt.integrator == MJ_INT_RK4 {
            crate::engine::engine_util_errmem::mju_error(
                b"RK4 integrator is not supported\0".as_ptr() as *const i8);
            return;
        }

        if (*m).opt.noslip_iterations != 0 {
            crate::engine::engine_util_errmem::mju_error(
                b"noslip solver is not supported\0".as_ptr() as *const i8);
            return;
        }

        // skip sensor computations if no sensor Jacobians requested
        let skipsensor = if DsDq.is_null() && DsDv.is_null() && DsDa.is_null() { 1 } else { 0 };

        // local vectors
        crate::engine::engine_memory::mj_mark_stack(d);
        let pos = crate::engine::engine_memory::mj_stack_alloc_num(d, nq as usize);
        let force = crate::engine::engine_memory::mj_stack_alloc_num(d, nv as usize);
        let force_plus = crate::engine::engine_memory::mj_stack_alloc_num(d, nv as usize);
        let sensor: *mut f64 = if skipsensor == 0 {
            crate::engine::engine_memory::mj_stack_alloc_num(d, ns as usize)
        } else {
            std::ptr::null_mut()
        };
        let mass: *mut f64 = if !DmDq.is_null() {
            crate::engine::engine_memory::mj_stack_alloc_num(d, nM as usize)
        } else {
            std::ptr::null_mut()
        };

        // save current positions
        crate::engine::engine_util_blas::mju_copy(pos, (*d).qpos, nq);

        // center point outputs
        let flg_act_int = if flg_actuation { 1 } else { 0 };
        inverse_skip(m, d, mjtStage_mjSTAGE_NONE, skipsensor, flg_act_int, force);
        if !sensor.is_null() {
            crate::engine::engine_util_blas::mju_copy(sensor, (*d).sensordata, ns);
        }
        if !mass.is_null() {
            crate::engine::engine_util_blas::mju_copy(mass, (*d).qM, nM);
        }

        // acceleration: skip = mjSTAGE_VEL
        if !DfDa.is_null() || !DsDa.is_null() {
            for i in 0..nv as usize {
                // nudge acceleration
                let tmp = *(*d).qacc.add(i);
                *(*d).qacc.add(i) += eps;

                // inverse dynamics, get force output
                inverse_skip(m, d, mjtStage_mjSTAGE_VEL, skipsensor, flg_act_int, force_plus);

                // restore
                *(*d).qacc.add(i) = tmp;

                // row of force Jacobian
                if !DfDa.is_null() {
                    diff(DfDa.add(i * nv as usize), force, force_plus, eps, nv);
                }

                // row of sensor Jacobian
                if !DsDa.is_null() {
                    diff(DsDa.add(i * ns as usize), sensor, (*d).sensordata, eps, ns);
                }
            }
        }

        // velocity: skip = mjSTAGE_POS
        if !DfDv.is_null() || !DsDv.is_null() {
            for i in 0..nv as usize {
                // nudge velocity
                let tmp = *(*d).qvel.add(i);
                *(*d).qvel.add(i) += eps;

                // inverse dynamics, get force output
                inverse_skip(m, d, mjtStage_mjSTAGE_POS, skipsensor, flg_act_int, force_plus);

                // restore
                *(*d).qvel.add(i) = tmp;

                // row of force Jacobian
                if !DfDv.is_null() {
                    diff(DfDv.add(i * nv as usize), force, force_plus, eps, nv);
                }

                // row of sensor Jacobian
                if !DsDv.is_null() {
                    diff(DsDv.add(i * ns as usize), sensor, (*d).sensordata, eps, ns);
                }
            }
        }

        // position: skip = mjSTAGE_NONE
        if !DfDq.is_null() || !DsDq.is_null() || !DmDq.is_null() {
            let dpos = crate::engine::engine_memory::mj_stack_alloc_num(d, nv as usize);
            for i in 0..nv as usize {
                // nudge
                crate::engine::engine_util_blas::mju_zero(dpos, nv);
                *dpos.add(i) = 1.0;
                crate::engine::engine_support::mj_integrate_pos(m, (*d).qpos, dpos, eps);

                // inverse dynamics, get force output
                inverse_skip(m, d, mjtStage_mjSTAGE_NONE, skipsensor, flg_act_int, force_plus);

                // restore
                crate::engine::engine_util_blas::mju_copy((*d).qpos, pos, nq);

                // row of force Jacobian
                if !DfDq.is_null() {
                    diff(DfDq.add(i * nv as usize), force, force_plus, eps, nv);
                }

                // row of sensor Jacobian
                if !DsDq.is_null() {
                    diff(DsDq.add(i * ns as usize), sensor, (*d).sensordata, eps, ns);
                }

                // row of inertia Jacobian
                if !DmDq.is_null() {
                    diff(DmDq.add(i * nM as usize), mass, (*d).qM, eps, nM);
                }
            }
        }

        crate::engine::engine_memory::mj_free_stack(d);
    }
}

