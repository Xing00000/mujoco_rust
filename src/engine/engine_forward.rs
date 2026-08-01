//! Port of: engine/engine_forward.c
//! IR hash: 9343293228317031
//! CODEGEN: source paths, owners, and callable names are locked.

use crate::types::*;

/// C: dcmotorVoltage (engine/engine_forward.c:222)
/// Calls: cxx:_mju_clip
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn dcmotorVoltage(ctrl: f64, length: f64, velocity: f64, x_I: f64, gainprm: *const f64) -> f64 {
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
/// Calls: cxx:_mju_clip
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn clampVec(vec: *mut f64, range: *const f64, limited: *const bool, n: i32, index: *const i32) {
    // SAFETY: caller guarantees vec, range, limited point to arrays of at least n elements,
    // and index (if non-null) also has at least n elements. All pointers are valid.
    unsafe {
        for i in 0..n as usize {
            let j = if !index.is_null() {
                *index.add(i) as usize
            } else {
                i
            };
            if *limited.add(i) {
                *vec.add(j) = crate::engine::engine_util_misc::mju_clip(
                    *vec.add(j),
                    *range.add(2 * i),
                    *range.add(2 * i + 1),
                );
            }
        }
    }
}

/// C: flex_has_implicit_stiffness (engine/engine_forward.c:1284)
#[allow(unused_variables, non_snake_case)]
pub fn flex_has_implicit_stiffness(m: *const mjModel) -> i32 {
    // SAFETY: m is a valid pointer to mjModel (caller contract)
    unsafe {
        for f in 0..(*m).nflex as isize {
            if *(*m).flex_rigid.offset(f) {
                continue;
            }

            // interpolated flex with stiffness
            if *(*m).flex_interp.offset(f) != 0
                && *(*m).flex_edgeequality.offset(f) != 3
                && *(*m).flex_stiffness.offset(*(*m).flex_stiffnessadr.offset(f) as isize) != 0.0
            {
                return 1;
            }

            // standard flex with bending
            if *(*m).flex_interp.offset(f) == 0
                && *(*m).flex_dim.offset(f) == 2
                && *(*m).flex_bendingadr.offset(f) >= 0
            {
                return 1;
            }
        }
        0
    }
}

/// C: midpoint_aligned (engine/engine_forward.c:1493)
#[allow(unused_variables, non_snake_case)]
pub fn midpoint_aligned(m: *const mjModel, jnt: i32) -> i32 {
    // SAFETY: caller guarantees m is valid and jnt is within bounds
    unsafe {
        let body = *(*m).jnt_bodyid.add(jnt as usize);
        let ipos = (*m).body_ipos;
        let aligned = *ipos.add(3 * body as usize + 0) == 0.0
            && *ipos.add(3 * body as usize + 1) == 0.0
            && *ipos.add(3 * body as usize + 2) == 0.0;
        aligned as i32
    }
}

/// C: midpointNewton (engine/engine_forward.c:1515)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_copy3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_cross, cxx:_mju_norm3, cxx:_mju_solve3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn midpointNewton(inertia: *const f64, w: *const f64, tau: *const f64, h: f64, w_mid: *mut f64) -> i32 {
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
/// Calls: cxx:_mj_midpoint, cxx:_mju_add, cxx:_mju_copy
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn midpoint(m: *const mjModel, d: *const mjData, qfrc: *const f64, free_jntid: *const i32, nfree: i32, qvel_old: *mut f64, qvel_new: *mut f64, dofadr: *mut i32) {
    const MJ_DSBL_GRAVITY: i32 = 1 << 7;
    // SAFETY: caller guarantees all pointers are valid and arrays are properly sized
    unsafe {
        for i in 0..nfree as usize {
            let j = *free_jntid.add(i);
            let body = *(*m).jnt_bodyid.add(j as usize);

            // save DOF address
            let adr = *(*m).jnt_dofadr.add(j as usize);
            *dofadr.add(i) = adr;

            // save old (current) velocity
            crate::engine::engine_util_blas::mju_copy(
                qvel_old.add(6 * i), (*d).qvel.add(adr as usize), 6);

            // compute external force = qfrc + qfrc_bias
            let mut qfrc_total: [f64; 6] = [0.0; 6];
            crate::engine::engine_util_blas::mju_add(
                qfrc_total.as_mut_ptr(), qfrc.add(adr as usize),
                (*d).qfrc_bias.add(adr as usize), 6);

            // gravity
            let gravity: *const f64 = if ((*m).opt.disableflags & MJ_DSBL_GRAVITY) != 0 {
                std::ptr::null()
            } else {
                (*m).opt.gravity.as_ptr()
            };

            // midpoint solver for free joint
            mj_midpoint(
                *(*m).body_mass.add(body as usize),
                (*m).body_inertia.add(3 * body as usize),
                (*m).body_ipos.add(3 * body as usize),
                (*m).body_iquat.add(4 * body as usize),
                (*d).xquat.add(4 * body as usize),
                (*d).qvel.add(adr as usize),
                qfrc_total.as_ptr(),
                gravity,
                (*m).opt.timestep,
                qvel_new.add(6 * i),
            );
        }
    }
}

/// C: mj_midpoint (engine/engine_forward.h:69)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_forward.c:_midpointNewton, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_add3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_addTo3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_axisAngle2Quat, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_copy3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_cross, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_mulQuat, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_negQuat, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_rotVecQuat, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_sub3, cxx:_mju_dot3, cxx:_mju_normalize3
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
            mji_negQuat, mji_rotVecQuat, mji_copy3, mji_mulQuat,
            mji_cross, mji_sub3, mji_add3, mji_addTo3, mji_axisAngle2Quat,
        };
        use crate::engine::engine_util_blas::{mju_dot3, mju_normalize3};

        // transform angular velocity and torque to inertial frame
        let mut iquat_neg: [f64; 4] = [0.0; 4];
        let mut w: [f64; 3] = [0.0; 3];
        let mut tau: [f64; 3] = [0.0; 3];
        mji_negQuat(iquat_neg.as_mut_ptr(), iquat);
        mji_rotVecQuat(w.as_mut_ptr(), qvel.add(3), iquat_neg.as_ptr());
        mji_rotVecQuat(tau.as_mut_ptr(), qfrc.add(3), iquat_neg.as_ptr());

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
            mji_negQuat(xquat_neg.as_mut_ptr(), xquat);
            mji_mulQuat(rot_x2i.as_mut_ptr(), iquat_neg.as_ptr(), xquat_neg.as_ptr());

            // force and CoM offset in inertial frame
            mji_rotVecQuat(force.as_mut_ptr(), qfrc, rot_x2i.as_ptr());
            mji_rotVecQuat(r_com.as_mut_ptr(), ipos, iquat_neg.as_ptr());

            // torque at CoM in inertial frame
            let mut rxf: [f64; 3] = [0.0; 3];
            mji_cross(rxf.as_mut_ptr(), r_com.as_ptr(), force.as_ptr());
            mji_sub3(tau_com.as_mut_ptr(), tau.as_ptr(), rxf.as_ptr());
        }

        // solve for midpoint angular velocity
        let mut w_mid: [f64; 3] = [0.0; 3];
        let niter = midpointNewton(inertia, w.as_ptr(), tau_com.as_ptr(), h, w_mid.as_mut_ptr());

        // next and mid angular velocities in inertial frame, rotate both to body frame
        let mut w_new: [f64; 3] = [0.0; 3];
        let mut w_new_body: [f64; 3] = [0.0; 3];
        let mut w_mid_body: [f64; 3] = [0.0; 3];
        for k in 0..3 {
            w_new[k] = 2.0 * w_mid[k] - w[k];
        }
        mji_rotVecQuat(w_new_body.as_mut_ptr(), w_new.as_ptr(), iquat);
        mji_rotVecQuat(w_mid_body.as_mut_ptr(), w_mid.as_ptr(), iquat);
        mji_copy3(qvel_new.add(3), w_new_body.as_ptr());

        // aligned: return
        if aligned {
            return niter;
        }

        // non-aligned: solve for translational velocity

        // rotate linear velocity to inertial frame
        let mut v: [f64; 3] = [0.0; 3];
        mji_rotVecQuat(v.as_mut_ptr(), qvel, rot_x2i.as_ptr());

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
            mji_rotVecQuat(g_inertial.as_mut_ptr(), gravity, rot_x2i.as_ptr());
            mji_addTo3(b.as_mut_ptr(), g_inertial.as_ptr());
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
        mji_axisAngle2Quat(qrot_new.as_mut_ptr(), axis.as_ptr(), h * wnorm);
        let mut xquat_new: [f64; 4] = [0.0; 4];
        mji_mulQuat(xquat_new.as_mut_ptr(), xquat, qrot_new.as_ptr());

        // v_new (linear): inertial → body → world using new orientation
        let mut v_body: [f64; 3] = [0.0; 3];
        mji_rotVecQuat(v_body.as_mut_ptr(), v_new.as_ptr(), iquat);
        mji_rotVecQuat(qvel_new, v_body.as_ptr(), xquat_new.as_ptr());

        niter
    }
}

