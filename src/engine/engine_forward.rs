//! Port of: engine/engine_forward.c
//! IR hash: e878892ca48fe222
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: dcmotorVoltage (engine/engine_forward.c:222)
/// Calls: mju_clip
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn dcmotor_voltage(ctrl: f64, length: f64, velocity: f64, x_I: f64, gainprm: *const f64) -> f64 {
    use crate::engine::engine_util_misc::mju_clip;
    // SAFETY: gainprm points to at least 9 f64 elements (caller contract)
    unsafe {
        let input_mode = *gainprm.add(8) as i32;
        let Vmax = *gainprm.add(7);
        let mut voltage;
        if input_mode > 0 {
            let kp = *gainprm.add(4);
            let ki = *gainprm.add(5);
            let kd = *gainprm.add(6);
            if input_mode == 1 {
                voltage = kp * (ctrl - length) + ki * x_I - kd * velocity;
            } else {
                voltage = kp * (ctrl - velocity) + ki * (x_I - length);
            }
        } else {
            voltage = ctrl;
        }
        if Vmax > 0.0 {
            voltage = mju_clip(voltage, -Vmax, Vmax);
        }
        voltage
    }
}

/// C: clampVec (engine/engine_forward.c:253)
/// Calls: mju_clip
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn clamp_vec(vec: *mut f64, range: *const f64, limited: *const mjtBool, n: i32, index: *const i32) {
    use crate::engine::engine_util_misc::mju_clip;
    // SAFETY: caller guarantees all pointers are valid with at least n elements
    unsafe {
        for i in 0..n as usize {
            let j = if index.is_null() { i } else { *index.add(i) as usize };
            if (*limited.add(i))._data[0] != 0 {
                *vec.add(j) = mju_clip(*vec.add(j), *range.add(2 * i), *range.add(2 * i + 1));
            }
        }
    }
}

/// C: warmstart (engine/engine_forward.c:786)
/// Calls: mj_constraintUpdate, mj_freeStack, mj_isSparse, mj_markStack, mj_mulJacVec, mj_mulM, mj_stackAllocInfo, mju_copy, mju_dot, mju_mulMatVec, mju_mulMatVecSparse, mju_subFrom, mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn warmstart(m: *const mjModel, d: *mut mjData) {
    todo!() // warmstart
}

/// C: solveIslandTask (engine/engine_forward.c:866)
/// Calls: mj_solCG_island, mj_solNewton_island, mj_solPGS_island
#[allow(unused_variables, non_snake_case)]
pub fn solve_island_task(m: *const mjModel, d: *mut mjData, arg: *mut (), thread_id: i32, island: i32) {
    todo!() // solveIslandTask
}

/// C: mj_advance (engine/engine_forward.c:981)
/// Calls: mj_actuatorDisabled, mj_computeSensor, mj_forwardSkip, mj_integratePosInd, mj_nextActivation, mj_sleep, mj_updateSleep, mjp_getPluginAtSlotUnsafe, mjp_pluginCount, mju_addToScl, mju_addToSclInd, mju_copy, mju_historyInsert, mju_message
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_advance(m: *const mjModel, d: *mut mjData, act_dot: *const f64, qacc: *const f64, qvel: *const f64) {
    todo!() // mj_advance
}

/// C: flex_has_implicit_stiffness (engine/engine_forward.c:1284)
#[allow(unused_variables, non_snake_case)]
pub fn flex_has_implicit_stiffness(m: *const mjModel) -> i32 {
    todo!() // flex_has_implicit_stiffness
}

/// C: flexInterp_cgsolve (engine/engine_forward.c:1311)
/// Calls: mj_freeStack, mj_markStack, mj_solveLD, mj_stackAllocInfo, mjd_flexBend_mul, mjd_flexInterp_cacheKrot, mjd_flexInterp_mul, mju_addScl, mju_addToScl, mju_copy, mju_dot, mju_max, mju_mulMatVecSparse, mju_mulSymVecSparse, mju_solveLUSparse, mju_sub, mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn flex_interp_cgsolve(m: *const mjModel, d: *mut mjData, qacc: *mut f64, qfrc: *const f64, nv: i32) {
    todo!() // flexInterp_cgsolve
}

/// C: midpoint_eligible (engine/engine_forward.c:1421)
/// Calls: mjCMesh::tree
#[allow(unused_variables, non_snake_case)]
pub fn midpoint_eligible(m: *const mjModel, d: *const mjData, jnt: i32) -> i32 {
    todo!() // midpoint_eligible
}

/// C: midpoint_aligned (engine/engine_forward.c:1493)
#[allow(unused_variables, non_snake_case)]
pub fn midpoint_aligned(m: *const mjModel, jnt: i32) -> i32 {
    todo!() // midpoint_aligned
}

/// C: midpointNewton (engine/engine_forward.c:1515)
/// Calls: mji_copy3, mji_cross, mju_norm3, mju_solve3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn midpoint_newton(inertia: *const f64, w: *const f64, tau: *const f64, h: f64, w_mid: *mut f64) -> i32 {
    use crate::engine::engine_inline::{mji_copy3, mji_cross};
    use crate::engine::engine_util_blas::mju_norm3;
    use crate::engine::engine_util_solve::mju_solve3;

    // SAFETY: all pointers are arrays of 3 f64 passed from caller; h is nonzero by contract
    unsafe {
        // precompute constants
        let i2h: f64 = 2.0 / h;
        let dI: [f64; 3] = [
            *inertia.add(2) - *inertia.add(1),
            *inertia.add(0) - *inertia.add(2),
            *inertia.add(1) - *inertia.add(0),
        ];
        let i2h_I: [f64; 3] = [
            i2h * *inertia.add(0),
            i2h * *inertia.add(1),
            i2h * *inertia.add(2),
        ];

        // initialize solution to previous angular velocity
        mji_copy3(w_mid, w);

        // Newton iteration
        let mut niter: i32 = 0;
        while niter < 100 {
            // compute Coriolis term
            let Iw: [f64; 3] = [
                *inertia.add(0) * *w_mid.add(0),
                *inertia.add(1) * *w_mid.add(1),
                *inertia.add(2) * *w_mid.add(2),
            ];
            let mut coriolis: [f64; 3] = [0.0; 3];
            mji_cross(coriolis.as_mut_ptr(), w_mid, Iw.as_ptr());

            // residual: f = i2h*I*(w_mid - w) + w_mid x (I*w_mid) - tau
            let mut f: [f64; 3] = [0.0; 3];
            for k in 0..3 {
                f[k] = i2h_I[k] * (*w_mid.add(k) - *w.add(k)) + coriolis[k] - *tau.add(k);
            }

            // check convergence
            let fnorm: f64 = mju_norm3(f.as_ptr());
            let tol: f64 = 1e-13;
            if fnorm < tol * (1.0 + i2h * mju_norm3(Iw.as_ptr())) {
                break;
            }

            // Jacobian: J = i2h*diag(I) + d(w x Iw)/dw
            let mut J: [f64; 9] = [0.0; 9];
            J[0] = i2h_I[0];                    J[1] = *w_mid.add(2) * dI[0]; J[2] = *w_mid.add(1) * dI[0];
            J[3] = *w_mid.add(2) * dI[1];       J[4] = i2h_I[1];              J[5] = *w_mid.add(0) * dI[1];
            J[6] = *w_mid.add(1) * dI[2];       J[7] = *w_mid.add(0) * dI[2]; J[8] = i2h_I[2];

            // solve J*delta = -f for search direction delta
            let neg_f: [f64; 3] = [-f[0], -f[1], -f[2]];
            let mut delta: [f64; 3] = [0.0; 3];
            mju_solve3(delta.as_mut_ptr(), J.as_ptr(), neg_f.as_ptr());

            // backtracking line search
            let mut step: f64 = 1.0;
            for _ls in 0..20 {
                let mut w_try: [f64; 3] = [0.0; 3];
                let mut Iw_try: [f64; 3] = [0.0; 3];
                for k in 0..3 {
                    w_try[k] = *w_mid.add(k) + step * delta[k];
                    Iw_try[k] = *inertia.add(k) * w_try[k];
                }
                let mut coriolis_try: [f64; 3] = [0.0; 3];
                mji_cross(coriolis_try.as_mut_ptr(), w_try.as_ptr(), Iw_try.as_ptr());

                let mut f_try: [f64; 3] = [0.0; 3];
                for k in 0..3 {
                    f_try[k] = i2h_I[k] * (w_try[k] - *w.add(k)) + coriolis_try[k] - *tau.add(k);
                }

                if mju_norm3(f_try.as_ptr()) < fnorm {
                    mji_copy3(w_mid, w_try.as_ptr());
                    break;
                }
                step *= 0.5;
            }

            niter += 1;
        }

        niter
    }
}

/// C: midpoint (engine/engine_forward.c:1736)
/// Calls: mj_midpoint, mju_add, mju_copy
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn midpoint(m: *const mjModel, d: *const mjData, qfrc: *const f64, free_jntid: *const i32, nfree: i32, qvel_old: *mut f64, qvel_new: *mut f64, dofadr: *mut i32) {
    todo!() // midpoint
}

/// C: mj_checkPos (engine/engine_forward.h:27)
/// Calls: mj_resetData, mj_warning, mju_isBad
#[allow(unused_variables, non_snake_case)]
pub fn mj_check_pos(m: *const mjModel, d: *mut mjData) {
    todo!() // mj_checkPos
}

/// C: mj_checkVel (engine/engine_forward.h:28)
/// Calls: mj_resetData, mj_warning, mju_isBad
#[allow(unused_variables, non_snake_case)]
pub fn mj_check_vel(m: *const mjModel, d: *mut mjData) {
    todo!() // mj_checkVel
}

/// C: mj_checkAcc (engine/engine_forward.h:29)
/// Calls: mj_forward, mj_resetData, mj_warning, mju_isBad
#[allow(unused_variables, non_snake_case)]
pub fn mj_check_acc(m: *const mjModel, d: *mut mjData) {
    todo!() // mj_checkAcc
}

/// C: mj_step (engine/engine_forward.h:35)
/// Calls: mj_Euler, mj_RungeKutta, mj_checkAcc, mj_checkPos, mj_checkVel, mj_compareFwdInv, mj_forward, mj_implicit, mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_step(m: *const mjModel, d: *mut mjData) {
    todo!() // mj_step
}

/// C: mj_step1 (engine/engine_forward.h:38)
/// Calls: mj_checkPos, mj_checkVel, mj_energyPos, mj_energyVel, mj_fwdPosition, mj_fwdVelocity, mj_sensorPos, mj_sensorVel
#[allow(unused_variables, non_snake_case)]
pub fn mj_step1(m: *const mjModel, d: *mut mjData) {
    todo!() // mj_step1
}

/// C: mj_step2 (engine/engine_forward.h:41)
/// Calls: mj_Euler, mj_checkAcc, mj_compareFwdInv, mj_fwdAcceleration, mj_fwdActuation, mj_fwdConstraint, mj_implicit, mj_sensorAcc
#[allow(unused_variables, non_snake_case)]
pub fn mj_step2(m: *const mjModel, d: *mut mjData) {
    todo!() // mj_step2
}

/// C: mj_forward (engine/engine_forward.h:44)
/// Calls: mj_forwardSkip
#[allow(unused_variables, non_snake_case)]
pub fn mj_forward(m: *const mjModel, d: *mut mjData) {
    todo!() // mj_forward
}

/// C: mj_forwardSkip (engine/engine_forward.h:47)
/// Calls: mj_energyPos, mj_energyVel, mj_fwdAcceleration, mj_fwdActuation, mj_fwdConstraint, mj_fwdPosition, mj_fwdVelocity, mj_sensorAcc, mj_sensorPos, mj_sensorVel
#[allow(unused_variables, non_snake_case)]
pub fn mj_forward_skip(m: *const mjModel, d: *mut mjData, skipstage: i32, skipsensor: i32) {
    todo!() // mj_forwardSkip
}

/// C: mj_RungeKutta (engine/engine_forward.h:53)
/// Calls: mj_advance, mj_forwardSkip, mj_freeStack, mj_integratePos, mj_markStack, mj_stackAllocInfo, mju_addToScl, mju_copy, mju_message, mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mj_runge_kutta(m: *const mjModel, d: *mut mjData, N: i32) {
    todo!() // mj_RungeKutta
}

/// C: mj_Euler (engine/engine_forward.h:56)
/// Calls: mj_EulerSkip
#[allow(unused_variables, non_snake_case)]
pub fn mj_euler(m: *const mjModel, d: *mut mjData) {
    todo!() // mj_Euler
}

/// C: mj_EulerSkip (engine/engine_forward.h:59)
/// Calls: mj_actuatorDamping, mj_advance, mj_factorI, mj_freeStack, mj_markStack, mj_solveLD, mj_stackAllocInfo, mjd_xPolyForce, mju_add, mju_addInd, mju_copy, mju_copyInd, mju_copySparse, mju_isZero
#[allow(unused_variables, non_snake_case)]
pub fn mj_euler_skip(m: *const mjModel, d: *mut mjData, skipfactor: i32) {
    todo!() // mj_EulerSkip
}

/// C: mj_implicit (engine/engine_forward.h:62)
/// Calls: mj_implicitSkip
#[allow(unused_variables, non_snake_case)]
pub fn mj_implicit(m: *const mjModel, d: *mut mjData) {
    todo!() // mj_implicit
}

/// C: mj_implicitSkip (engine/engine_forward.h:65)
/// Calls: flexInterp_cgsolve, flex_has_implicit_stiffness, midpoint, midpoint_aligned, midpoint_eligible, mj_advance, mj_factorI, mj_freeStack, mj_markStack, mj_solveLD, mj_stackAllocInfo, mjd_smooth_vel, mju_add, mju_addInd, mju_addScl, mju_addToScl, mju_copy, mju_copyInd, mju_factorLUSparse, mju_gather, mju_gatherMasked, mju_message, mju_solveLUSparse
#[allow(unused_variables, non_snake_case)]
pub fn mj_implicit_skip(m: *const mjModel, d: *mut mjData, skipfactor: i32) {
    todo!() // mj_implicitSkip
}

/// C: mj_midpoint (engine/engine_forward.h:69)
/// Calls: midpointNewton, mji_add3, mji_addTo3, mji_axisAngle2Quat, mji_copy3, mji_cross, mji_mulQuat, mji_negQuat, mji_rotVecQuat, mji_sub3, mju_dot3, mju_normalize3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_midpoint(mass: f64, inertia: *const f64, ipos: *const f64, iquat: *const f64, xquat: *const f64, qvel: *const f64, qfrc: *const f64, gravity: *const f64, h: f64, qvel_new: *mut f64) -> i32 {
    // SAFETY: all pointer args point to valid f64 arrays of documented sizes
    // inertia[3], ipos[3], iquat[4], xquat[4], qvel[6], qfrc[6], gravity[3], qvel_new[6]
    unsafe {
        use crate::engine::engine_inline::{
            mji_neg_quat, mji_rot_vec_quat, mji_copy3, mji_mul_quat,
            mji_cross, mji_sub3, mji_add3, mji_add_to3, mji_axis_angle2quat,
        };
        use crate::engine::engine_util_blas::{mju_dot3, mju_normalize3};

        // transform angular velocity and torque to inertial frame
        let mut iquat_neg: [f64; 4] = [0.0; 4];
        let mut w: [f64; 3] = [0.0; 3];
        let mut tau: [f64; 3] = [0.0; 3];
        mji_neg_quat(iquat_neg.as_mut_ptr(), iquat);
        mji_rot_vec_quat(w.as_mut_ptr(), qvel.add(3), iquat_neg.as_ptr());
        mji_rot_vec_quat(tau.as_mut_ptr(), qfrc.add(3), iquat_neg.as_ptr());

        // check for translational-rotational coupling
        let aligned = *ipos.add(0) == 0.0 && *ipos.add(1) == 0.0 && *ipos.add(2) == 0.0;

        let mut r_com: [f64; 3] = [0.0; 3];
        let mut tau_com: [f64; 3] = [0.0; 3];
        let mut rot_x2i: [f64; 4] = [0.0; 4];
        let mut force: [f64; 3] = [0.0; 3];

        // compute torque at CoM in inertial frame
        if aligned {
            mji_copy3(tau_com.as_mut_ptr(), tau.as_ptr());
        } else {
            // rotation from world to inertial frame
            let mut xquat_neg: [f64; 4] = [0.0; 4];
            mji_neg_quat(xquat_neg.as_mut_ptr(), xquat);
            mji_mul_quat(rot_x2i.as_mut_ptr(), iquat_neg.as_ptr(), xquat_neg.as_ptr());

            // force and CoM offset in inertial frame
            mji_rot_vec_quat(force.as_mut_ptr(), qfrc, rot_x2i.as_ptr());
            mji_rot_vec_quat(r_com.as_mut_ptr(), ipos, iquat_neg.as_ptr());

            // torque at CoM in inertial frame
            let mut rxf: [f64; 3] = [0.0; 3];
            mji_cross(rxf.as_mut_ptr(), r_com.as_ptr(), force.as_ptr());
            mji_sub3(tau_com.as_mut_ptr(), tau.as_ptr(), rxf.as_ptr());
        }

        // solve for midpoint angular velocity
        let mut w_mid: [f64; 3] = [0.0; 3];
        let niter = midpoint_newton(inertia, w.as_ptr(), tau_com.as_ptr(), h, w_mid.as_mut_ptr());

        // next and mid angular velocities in inertial frame, rotate both to body frame
        let mut w_new: [f64; 3] = [0.0; 3];
        let mut w_new_body: [f64; 3] = [0.0; 3];
        let mut w_mid_body: [f64; 3] = [0.0; 3];
        for k in 0..3 {
            w_new[k] = 2.0 * w_mid[k] - w[k];
        }
        mji_rot_vec_quat(w_new_body.as_mut_ptr(), w_new.as_ptr(), iquat);
        mji_rot_vec_quat(w_mid_body.as_mut_ptr(), w_mid.as_ptr(), iquat);
        mji_copy3(qvel_new.add(3), w_new_body.as_ptr());

        // aligned: return
        if aligned {
            return niter;
        }

        // non-aligned: solve for translational velocity

        // rotate linear velocity to inertial frame
        let mut v: [f64; 3] = [0.0; 3];
        mji_rot_vec_quat(v.as_mut_ptr(), qvel, rot_x2i.as_ptr());

        // current CoM velocities (rot, lin) in inertial frame
        let mut wxr: [f64; 3] = [0.0; 3];
        mji_cross(wxr.as_mut_ptr(), w.as_ptr(), r_com.as_ptr());
        let mut vcom: [f64; 3] = [0.0; 3];
        mji_add3(vcom.as_mut_ptr(), v.as_ptr(), wxr.as_ptr());

        // right-hand side for midpoint CoM velocity
        let i2h = 2.0 / h;
        let mut b: [f64; 3] = [0.0; 3];
        for k in 0..3 {
            b[k] = force[k] / mass + i2h * vcom[k];
        }

        // add gravity, if any
        if !gravity.is_null() {
            let mut g_inertial: [f64; 3] = [0.0; 3];
            mji_rot_vec_quat(g_inertial.as_mut_ptr(), gravity, rot_x2i.as_ptr());
            mji_add_to3(b.as_mut_ptr(), g_inertial.as_ptr());
        }

        // analytic solution for (i2h*Id + [w_mid]x) * vcom_mid = b
        let wnorm2 = mju_dot3(w_mid.as_ptr(), w_mid.as_ptr());
        let denom = i2h * i2h + wnorm2;
        let w_dot_b = mju_dot3(w_mid.as_ptr(), b.as_ptr());
        let mut w_cross_b: [f64; 3] = [0.0; 3];
        mji_cross(w_cross_b.as_mut_ptr(), w_mid.as_ptr(), b.as_ptr());
        let mut vcom_mid: [f64; 3] = [0.0; 3];
        for k in 0..3 {
            vcom_mid[k] = (i2h * b[k] + (w_dot_b / i2h) * w_mid[k] - w_cross_b[k]) / denom;
        }

        // recover midpoint and new joint velocity in inertial frame
        let mut wxr_mid: [f64; 3] = [0.0; 3];
        mji_cross(wxr_mid.as_mut_ptr(), w_mid.as_ptr(), r_com.as_ptr());
        let mut v_mid: [f64; 3] = [0.0; 3];
        let mut v_new: [f64; 3] = [0.0; 3];
        for k in 0..3 {
            v_mid[k] = vcom_mid[k] - wxr_mid[k];
            v_new[k] = 2.0 * v_mid[k] - v[k];
        }

        // estimate new orientation
        let mut axis: [f64; 3] = [0.0; 3];
        mji_copy3(axis.as_mut_ptr(), w_mid_body.as_ptr());
        let wnorm = mju_normalize3(axis.as_mut_ptr());
        let mut qrot_new: [f64; 4] = [0.0; 4];
        mji_axis_angle2quat(qrot_new.as_mut_ptr(), axis.as_ptr(), h * wnorm);
        let mut xquat_new: [f64; 4] = [0.0; 4];
        mji_mul_quat(xquat_new.as_mut_ptr(), xquat, qrot_new.as_ptr());

        // v_new (linear): inertial → body → world using new orientation
        let mut v_body: [f64; 3] = [0.0; 3];
        mji_rot_vec_quat(v_body.as_mut_ptr(), v_new.as_ptr(), iquat);
        mji_rot_vec_quat(qvel_new, v_body.as_ptr(), xquat_new.as_ptr());

        niter
    }
}

/// C: mj_fwdKinematics (engine/engine_forward.h:78)
/// Calls: mj_camlight, mj_comPos, mj_flex, mj_kinematics, mj_tendon, mj_updateSleep, mj_wakeTendon
#[allow(unused_variables, non_snake_case)]
pub fn mj_fwd_kinematics(m: *const mjModel, d: *mut mjData) {
    todo!() // mj_fwdKinematics
}

/// C: mj_fwdPosition (engine/engine_forward.h:81)
/// Calls: mj_collision, mj_factorM, mj_fwdKinematics, mj_island, mj_makeConstraint, mj_makeM, mj_projectConstraint, mj_transmission, mj_updateSleep, mj_wakeCollision, mj_wakeEquality
#[allow(unused_variables, non_snake_case)]
pub fn mj_fwd_position(m: *const mjModel, d: *mut mjData) {
    todo!() // mj_fwdPosition
}

/// C: mj_fwdVelocity (engine/engine_forward.h:84)
/// Calls: mj_comVel, mj_passive, mj_referenceConstraint, mj_rne, mj_tendonBias, mju_mulMatVecSparse, mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mj_fwd_velocity(m: *const mjModel, d: *mut mjData) {
    todo!() // mj_fwdVelocity
}

/// C: mj_fwdActuation (engine/engine_forward.h:87)
/// Calls: clampVec, dcmotorVoltage, mj_actuatorDisabled, mj_dcmotorSlots, mj_freeStack, mj_lugreStribeck, mj_markStack, mj_nextActivation, mj_readCtrl, mj_sleepState, mj_stackAllocInfo, mj_warning, mjp_getPluginAtSlotUnsafe, mjp_pluginCount, mju_addTo, mju_clip, mju_isBad, mju_max, mju_message, mju_min, mju_mulMatTVecSparse, mju_muscleBias, mju_muscleDynamics, mju_muscleGain, mju_norm3, mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mj_fwd_actuation(m: *const mjModel, d: *mut mjData) {
    todo!() // mj_fwdActuation
}

/// C: mj_fwdAcceleration (engine/engine_forward.h:90)
/// Calls: mj_solveLD, mj_xfrcAccumulate, mju_addTo, mju_addToInd, mju_copy, mju_copyInd, mju_sub, mju_subInd
#[allow(unused_variables, non_snake_case)]
pub fn mj_fwd_acceleration(m: *const mjModel, d: *mut mjData) {
    todo!() // mj_fwdAcceleration
}

/// C: mj_fwdConstraint (engine/engine_forward.h:93)
/// Calls: mj_dualFinish, mj_mulJacVec, mj_solCG, mj_solNewton, mj_solNoSlip, mj_solNoSlip_island, mj_solPGS, mju_copy, mju_dispatch, mju_gather, mju_message, mju_scatter, mju_subFrom, mju_zero, mju_zeroInt, warmstart
#[allow(unused_variables, non_snake_case)]
pub fn mj_fwd_constraint(m: *const mjModel, d: *mut mjData) {
    todo!() // mj_fwdConstraint
}

