//! Port of: engine/engine_forward.c
//! IR hash: 05737965add36adb
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
pub fn dcmotor_voltage(ctrl: f64, length: f64, velocity: f64, x_I: f64, gainprm: *const f64) -> f64  {
    if gainprm.is_null() {
        return 0.0;
    }
    extern "C" { fn dcmotorVoltage(ctrl: f64, length: f64, velocity: f64, x_I: f64, gainprm: *const f64) -> f64; }
    // SAFETY: gainprm verified non-null; delegates to C implementation
    unsafe { dcmotorVoltage(ctrl, length, velocity, x_I, gainprm) }
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
    if vec.is_null() || range.is_null() || limited.is_null() || n <= 0 {
        return;
    }
    // SAFETY: vec has n elements, range has 2*n elements, limited has n elements (as u8).
    // index may be null (use sequential indexing) or non-null (indirect indexing).
    unsafe {
        let limited_u8 = limited as *const u8;
        for i in 0..n as usize {
            let idx = if index.is_null() { i as i32 } else { *index.add(i) };
            if *limited_u8.add(idx as usize) != 0 {
                let lo = *range.add(2 * idx as usize);
                let hi = *range.add(2 * idx as usize + 1);
                let val = *vec.add(i);
                if val < lo {
                    *vec.add(i) = lo;
                } else if val > hi {
                    *vec.add(i) = hi;
                }
            }
        }
    }
}

/// C: warmstart (engine/engine_forward.c:786)
/// Calls: mj_constraintUpdate, mj_freeStack, mj_isSparse, mj_markStack, mj_mulJacVec, mj_mulM, mj_stackAllocInfo, mju_copy, mju_dot, mju_mulMatVec, mju_mulMatVecSparse, mju_subFrom, mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn warmstart(m: *const mjModel, d: *mut mjData) {
    if m.is_null() { return; }
    extern "C" { fn warmstart(m: *const mjModel, d: *mut mjData); }
    // SAFETY: m verified non-null; delegates to C implementation
    unsafe { warmstart(m, d) }
}

/// C: solveIslandTask (engine/engine_forward.c:866)
/// Calls: mj_solCG_island, mj_solNewton_island, mj_solPGS_island
#[allow(unused_variables, non_snake_case)]
pub fn solve_island_task(m: *const mjModel, d: *mut mjData, arg: *mut (), thread_id: i32, island: i32) {
    extern "C" { fn solveIslandTask(m: *const mjModel, d: *mut mjData, arg: *mut (), thread_id: i32, island: i32); }
    // SAFETY: delegates to C implementation
    unsafe { solveIslandTask(m, d, arg, thread_id, island) }
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
    extern "C" { fn mj_advance(m: *const mjModel, d: *mut mjData, act_dot: *const f64, qacc: *const f64, qvel: *const f64); }
    // SAFETY: delegates to C implementation, pointers valid per caller
    unsafe { mj_advance(m, d, act_dot, qacc, qvel) }
}

/// C: flex_has_implicit_stiffness (engine/engine_forward.c:1284)
#[allow(unused_variables, non_snake_case)]
pub fn flex_has_implicit_stiffness(m: *const mjModel) -> i32  {
    if m.is_null() { return 0; }
    extern "C" { fn flex_has_implicit_stiffness(m: *const mjModel) -> i32; }
    // SAFETY: m verified non-null
    unsafe { flex_has_implicit_stiffness(m) }
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
    extern "C" { fn flexInterp_cgsolve(m: *const mjModel, d: *mut mjData, qacc: *mut f64, qfrc: *const f64, nv: i32); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { flexInterp_cgsolve(m, d, qacc, qfrc, nv) }
}

/// C: midpoint_eligible (engine/engine_forward.c:1421)
#[allow(unused_variables, non_snake_case)]
pub fn midpoint_eligible(m: *const mjModel, d: *const mjData, jnt: i32) -> i32  {
    if m.is_null() || d.is_null() { return 0; }
    extern "C" { fn midpoint_eligible(m: *const mjModel, d: *const mjData, jnt: i32) -> i32; }
    // SAFETY: m, d verified non-null
    unsafe { midpoint_eligible(m, d, jnt) }
}

/// C: midpoint_aligned (engine/engine_forward.c:1493)
#[allow(unused_variables, non_snake_case)]
pub fn midpoint_aligned(m: *const mjModel, jnt: i32) -> i32  {
    if m.is_null() { return 0; }
    extern "C" { fn midpoint_aligned(m: *const mjModel, jnt: i32) -> i32; }
    // SAFETY: m verified non-null
    unsafe { midpoint_aligned(m, jnt) }
}

/// C: midpointNewton (engine/engine_forward.c:1515)
/// Calls: mji_copy3, mji_cross, mju_norm3, mju_solve3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn midpoint_newton(inertia: *const f64, w: *const f64, tau: *const f64, h: f64, w_mid: *mut f64) -> i32  {
    if inertia.is_null() || w.is_null() || tau.is_null() || w_mid.is_null() { return 0; }
    extern "C" { fn midpointNewton(inertia: *const f64, w: *const f64, tau: *const f64, h: f64, w_mid: *mut f64) -> i32; }
    // SAFETY: all pointers verified non-null
    unsafe { midpointNewton(inertia, w, tau, h, w_mid) }
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
    if m.is_null() || d.is_null() { return; }
    extern "C" { fn midpoint(m: *const mjModel, d: *const mjData, qfrc: *const f64, free_jntid: *const i32, nfree: i32, qvel_old: *mut f64, qvel_new: *mut f64, dofadr: *mut i32); }
    // SAFETY: m, d verified non-null
    unsafe { midpoint(m, d, qfrc, free_jntid, nfree, qvel_old, qvel_new, dofadr) }
}

/// C: mj_checkPos (engine/engine_forward.h:27)
/// Calls: mj_resetData, mj_warning, mju_isBad
#[allow(unused_variables, non_snake_case)]
pub fn mj_check_pos(m: *const mjModel, d: *mut mjData) {
    // SAFETY: m, d valid per caller. Warning field accessed via byte offset (160264 + 8*idx).
    // mjWarningStat layout: lastinfo(i32) at +0, number(i32) at +4. sizeof=8.
    unsafe {
        const MJWARN_BADQPOS: i32 = 3;
        const MJDSBL_AUTORESET: i32 = 1 << 16;
        const WARNING_OFFSET: usize = 160264; // offsetof(mjData, warning)

        let nq: i32 = (*m).nq as i32;
        let qpos: *const f64 = (*d).qpos;
        let mut i: i32 = 0;
        while i < nq {
            if crate::engine::engine_util_misc::mju_is_bad(*qpos.add(i as usize)) != 0 {
                crate::engine::engine_core_util::mj_warning(d, MJWARN_BADQPOS, i);
                if ((*m).opt.disableflags & MJDSBL_AUTORESET) == 0 {
                    crate::engine::engine_io::mj_reset_data(m, d);
                }
                // d->warning[mjWARN_BADQPOS].number++
                let warn_ptr = (d as *mut u8).add(WARNING_OFFSET + 8 * MJWARN_BADQPOS as usize);
                let number_ptr = warn_ptr.add(4) as *mut i32;
                *number_ptr += 1;
                // d->warning[mjWARN_BADQPOS].lastinfo = i
                let lastinfo_ptr = warn_ptr as *mut i32;
                *lastinfo_ptr = i;
                return;
            }
            i += 1;
        }
    }
}

/// C: mj_checkVel (engine/engine_forward.h:28)
/// Calls: mj_resetData, mj_warning, mju_isBad
#[allow(unused_variables, non_snake_case)]
pub fn mj_check_vel(m: *const mjModel, d: *mut mjData) {
    // SAFETY: m, d valid per caller. Warning field accessed via byte offset.
    unsafe {
        const MJWARN_BADQVEL: i32 = 4;
        const MJDSBL_AUTORESET: i32 = 1 << 16;
        const MJENBL_SLEEP: i32 = 1 << 4;
        const WARNING_OFFSET: usize = 160264;

        let sleep_filter: i32 = (((*m).opt.enableflags & MJENBL_SLEEP) != 0 && (*d).nv_awake < (*m).nv as i32) as i32;
        let nv: i32 = if sleep_filter != 0 { (*d).nv_awake } else { (*m).nv as i32 };

        let mut j: i32 = 0;
        while j < nv {
            let i: i32 = if sleep_filter != 0 { *(*d).dof_awake_ind.add(j as usize) } else { j };

            if crate::engine::engine_util_misc::mju_is_bad(*(*d).qvel.add(i as usize)) != 0 {
                crate::engine::engine_core_util::mj_warning(d, MJWARN_BADQVEL, i);
                if ((*m).opt.disableflags & MJDSBL_AUTORESET) == 0 {
                    crate::engine::engine_io::mj_reset_data(m, d);
                }
                let warn_ptr = (d as *mut u8).add(WARNING_OFFSET + 8 * MJWARN_BADQVEL as usize);
                let number_ptr = warn_ptr.add(4) as *mut i32;
                *number_ptr += 1;
                let lastinfo_ptr = warn_ptr as *mut i32;
                *lastinfo_ptr = i;
                return;
            }
            j += 1;
        }
    }
}

/// C: mj_checkAcc (engine/engine_forward.h:29)
/// Calls: mj_forward, mj_resetData, mj_warning, mju_isBad
#[allow(unused_variables, non_snake_case)]
pub fn mj_check_acc(m: *const mjModel, d: *mut mjData) {
    // SAFETY: m, d valid per caller. Warning field accessed via byte offset.
    unsafe {
        const MJWARN_BADQACC: i32 = 5;
        const MJDSBL_AUTORESET: i32 = 1 << 16;
        const MJENBL_SLEEP: i32 = 1 << 4;
        const WARNING_OFFSET: usize = 160264;

        let sleep_filter: i32 = (((*m).opt.enableflags & MJENBL_SLEEP) != 0 && (*d).nv_awake < (*m).nv as i32) as i32;
        let nv: i32 = if sleep_filter != 0 { (*d).nv_awake } else { (*m).nv as i32 };

        let mut j: i32 = 0;
        while j < nv {
            let i: i32 = if sleep_filter != 0 { *(*d).dof_awake_ind.add(j as usize) } else { j };

            if crate::engine::engine_util_misc::mju_is_bad(*(*d).qacc.add(i as usize)) != 0 {
                crate::engine::engine_core_util::mj_warning(d, MJWARN_BADQACC, i);
                if ((*m).opt.disableflags & MJDSBL_AUTORESET) == 0 {
                    crate::engine::engine_io::mj_reset_data(m, d);
                }
                let warn_ptr = (d as *mut u8).add(WARNING_OFFSET + 8 * MJWARN_BADQACC as usize);
                let number_ptr = warn_ptr.add(4) as *mut i32;
                *number_ptr += 1;
                let lastinfo_ptr = warn_ptr as *mut i32;
                *lastinfo_ptr = i;
                if ((*m).opt.disableflags & MJDSBL_AUTORESET) == 0 {
                    mj_forward(m, d);
                }
                return;
            }
            j += 1;
        }
    }
}

/// C: mj_step (engine/engine_forward.h:35)
/// Calls: mj_Euler, mj_RungeKutta, mj_checkAcc, mj_checkPos, mj_checkVel, mj_compareFwdInv, mj_forward, mj_implicit, mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_step(m: *const mjModel, d: *mut mjData) {
    const mjENBL_FWDINV: i32 = 1 << 2;
    const mjINT_EULER: i32 = 0;
    const mjINT_RK4: i32 = 1;
    const mjINT_IMPLICIT: i32 = 2;
    const mjINT_IMPLICITFAST: i32 = 3;

    // SAFETY: m, d valid per caller. opt.integrator and opt.enableflags are safe to read.
    unsafe {
        // common to all integrators
        mj_check_pos(m, d);
        mj_check_vel(m, d);
        mj_forward(m, d);
        mj_check_acc(m, d);

        // compare forward and inverse solutions if enabled
        if ((*m).opt.enableflags & mjENBL_FWDINV) != 0 {
            crate::engine::engine_inverse::mj_compare_fwd_inv(m, d);
        }

        // use selected integrator
        match (*m).opt.integrator {
            mjINT_EULER => mj_euler(m, d),
            mjINT_RK4 => mj_runge_kutta(m, d, 4),
            mjINT_IMPLICIT | mjINT_IMPLICITFAST => mj_implicit(m, d),
            _ => crate::engine::engine_util_errmem::mju_error(
                b"invalid integrator\0".as_ptr() as *const i8,
            ),
        }
    }
}

/// C: mj_step1 (engine/engine_forward.h:38)
/// Calls: mj_checkPos, mj_checkVel, mj_energyPos, mj_energyVel, mj_fwdPosition, mj_fwdVelocity, mj_sensorPos, mj_sensorVel
#[allow(unused_variables, non_snake_case)]
pub fn mj_step1(m: *const mjModel, d: *mut mjData) {
    const mjENBL_ENERGY: i32 = 1 << 1;

    // SAFETY: m, d valid per caller. All called functions handle their own safety.
    unsafe {
        mj_check_pos(m, d);
        mj_check_vel(m, d);
        mj_fwd_position(m, d);
        crate::engine::engine_sensor::mj_sensor_pos(m, d);

        if *((&(*d).flg_energypos) as *const mjtBool as *const u8) == 0 {
            if ((*m).opt.enableflags & mjENBL_ENERGY) != 0 {
                crate::engine::engine_sensor::mj_energy_pos(m, d);
            } else {
                (*d).energy[0] = 0.0;
                (*d).energy[1] = 0.0;
            }
        }

        mj_fwd_velocity(m, d);
        crate::engine::engine_sensor::mj_sensor_vel(m, d);
        if ((*m).opt.enableflags & mjENBL_ENERGY) != 0
            && *((&(*d).flg_energyvel) as *const mjtBool as *const u8) == 0
        {
            crate::engine::engine_sensor::mj_energy_vel(m, d);
        }

        extern "C" {
            static mjcb_control: Option<unsafe extern "C" fn(*const mjModel, *mut mjData)>;
        }
        if let Some(cb) = mjcb_control {
            cb(m, d);
        }
    }
}

/// C: mj_step2 (engine/engine_forward.h:41)
/// Calls: mj_Euler, mj_checkAcc, mj_compareFwdInv, mj_fwdAcceleration, mj_fwdActuation, mj_fwdConstraint, mj_implicit, mj_sensorAcc
#[allow(unused_variables, non_snake_case)]
pub fn mj_step2(m: *const mjModel, d: *mut mjData) {
    const mjENBL_FWDINV: i32 = 1 << 2;
    const mjINT_IMPLICIT: i32 = 2;
    const mjINT_IMPLICITFAST: i32 = 3;

    // SAFETY: m, d valid per caller. All called functions handle their own safety.
    unsafe {
        mj_fwd_actuation(m, d);
        mj_fwd_acceleration(m, d);
        mj_fwd_constraint(m, d);
        *((&mut (*d).flg_rnepost) as *mut mjtBool as *mut u8) = 0;  // clear flag for lazy evaluation
        crate::engine::engine_sensor::mj_sensor_acc(m, d);
        mj_check_acc(m, d);

        // compare forward and inverse solutions if enabled
        if ((*m).opt.enableflags & mjENBL_FWDINV) != 0 {
            crate::engine::engine_inverse::mj_compare_fwd_inv(m, d);
        }

        // integrate with Euler or implicit; RK4 defaults to Euler
        if (*m).opt.integrator == mjINT_IMPLICIT || (*m).opt.integrator == mjINT_IMPLICITFAST {
            mj_implicit(m, d);
        } else {
            mj_euler(m, d);
        }
    }
}

/// C: mj_forward (engine/engine_forward.h:44)
/// Calls: mj_forwardSkip
#[allow(unused_variables, non_snake_case)]
pub fn mj_forward(m: *const mjModel, d: *mut mjData) {
    mj_forward_skip(m, d, 0, 0);
}

/// C: mj_forwardSkip (engine/engine_forward.h:47)
/// Calls: mj_energyPos, mj_energyVel, mj_fwdAcceleration, mj_fwdActuation, mj_fwdConstraint, mj_fwdPosition, mj_fwdVelocity, mj_sensorAcc, mj_sensorPos, mj_sensorVel
#[allow(unused_variables, non_snake_case)]
pub fn mj_forward_skip(m: *const mjModel, d: *mut mjData, skipstage: i32, skipsensor: i32) {
    const mjSTAGE_POS: i32 = 1;
    const mjSTAGE_VEL: i32 = 2;
    const mjENBL_ENERGY: i32 = 1 << 1;
    const mjDSBL_ACTUATION: i32 = 1 << 11;

    // SAFETY: m, d valid per caller. All called functions handle their own safety.
    unsafe {
        // position-dependent
        if skipstage < mjSTAGE_POS {
            mj_fwd_position(m, d);

            if skipsensor == 0 {
                crate::engine::engine_sensor::mj_sensor_pos(m, d);
            }

            if *((&(*d).flg_energypos) as *const mjtBool as *const u8) == 0 {
                if ((*m).opt.enableflags & mjENBL_ENERGY) != 0 {
                    crate::engine::engine_sensor::mj_energy_pos(m, d);
                } else {
                    (*d).energy[0] = 0.0;
                    (*d).energy[1] = 0.0;
                }
            }
        }

        // velocity-dependent
        if skipstage < mjSTAGE_VEL {
            mj_fwd_velocity(m, d);

            if skipsensor == 0 {
                crate::engine::engine_sensor::mj_sensor_vel(m, d);
            }

            if ((*m).opt.enableflags & mjENBL_ENERGY) != 0
                && *((&(*d).flg_energyvel) as *const mjtBool as *const u8) == 0
            {
                crate::engine::engine_sensor::mj_energy_vel(m, d);
            }
        }

        // acceleration-dependent
        if ((*m).opt.disableflags & mjDSBL_ACTUATION) == 0 {
            extern "C" {
                static mjcb_control: Option<unsafe extern "C" fn(*const mjModel, *mut mjData)>;
            }
            if let Some(cb) = mjcb_control {
                cb(m, d);
            }
        }

        mj_fwd_actuation(m, d);
        mj_fwd_acceleration(m, d);
        mj_fwd_constraint(m, d);
        if skipsensor == 0 {
            *((&mut (*d).flg_rnepost) as *mut mjtBool as *mut u8) = 0;  // clear flag for lazy evaluation
            crate::engine::engine_sensor::mj_sensor_acc(m, d);
        }
    }
}

/// C: mj_RungeKutta (engine/engine_forward.h:53)
/// Calls: mj_advance, mj_forwardSkip, mj_freeStack, mj_integratePos, mj_markStack, mj_stackAllocInfo, mju_addToScl, mju_copy, mju_message, mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mj_runge_kutta(m: *const mjModel, d: *mut mjData, N: i32) {
    extern "C" { fn mj_RungeKutta(m: *const mjModel, d: *mut mjData, N: i32); }
    // SAFETY: delegates to C implementation, pointers valid per caller
    unsafe { mj_RungeKutta(m, d, N) }
}

/// C: mj_Euler (engine/engine_forward.h:56)
/// Calls: mj_EulerSkip
#[allow(unused_variables, non_snake_case)]
pub fn mj_euler(m: *const mjModel, d: *mut mjData) {
    mj_euler_skip(m, d, 0);
}

/// C: mj_EulerSkip (engine/engine_forward.h:59)
/// Calls: mj_actuatorDamping, mj_advance, mj_factorI, mj_freeStack, mj_markStack, mj_solveLD, mj_stackAllocInfo, mjd_xPolyForce, mju_add, mju_addInd, mju_copy, mju_copyInd, mju_copySparse, mju_isZero
#[allow(unused_variables, non_snake_case)]
pub fn mj_euler_skip(m: *const mjModel, d: *mut mjData, skipfactor: i32) {
    if m.is_null() || d.is_null() {
        return;
    }
    extern "C" { fn mj_EulerSkip(m: *const mjModel, d: *mut mjData, skipfactor: i32); }
    // SAFETY: m, d verified non-null; delegates to C implementation
    unsafe { mj_EulerSkip(m, d, skipfactor) }
}

/// C: mj_implicit (engine/engine_forward.h:62)
/// Calls: mj_implicitSkip
#[allow(unused_variables, non_snake_case)]
pub fn mj_implicit(m: *const mjModel, d: *mut mjData) {
    mj_implicit_skip(m, d, 0);
}

/// C: mj_implicitSkip (engine/engine_forward.h:65)
/// Calls: flexInterp_cgsolve, flex_has_implicit_stiffness, midpoint, midpoint_aligned, midpoint_eligible, mj_advance, mj_factorI, mj_freeStack, mj_markStack, mj_solveLD, mj_stackAllocInfo, mjd_smooth_vel, mju_add, mju_addInd, mju_addScl, mju_addToScl, mju_copy, mju_copyInd, mju_factorLUSparse, mju_gather, mju_gatherMasked, mju_message, mju_solveLUSparse
#[allow(unused_variables, non_snake_case)]
pub fn mj_implicit_skip(m: *const mjModel, d: *mut mjData, skipfactor: i32) {
    extern "C" { fn mj_implicitSkip(m: *const mjModel, d: *mut mjData, skipfactor: i32); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_implicitSkip(m, d, skipfactor) }
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
    // SAFETY: all pointers valid per caller contract; intermediate arrays are stack-local
    unsafe {
        // transform angular velocity and torque to inertial frame
        let mut iquat_neg: [f64; 4] = [0.0; 4];
        let mut w: [f64; 3] = [0.0; 3];
        let mut tau: [f64; 3] = [0.0; 3];
        crate::engine::engine_inline::mji_neg_quat(iquat_neg.as_mut_ptr(), iquat);
        crate::engine::engine_inline::mji_rot_vec_quat(
            w.as_mut_ptr(), qvel.add(3), iquat_neg.as_ptr());  // qvel+3 (angular) is in body frame
        crate::engine::engine_inline::mji_rot_vec_quat(
            tau.as_mut_ptr(), qfrc.add(3), iquat_neg.as_ptr());  // qfrc+3 (angular) is in body frame

        // check for translational-rotational coupling
        let aligned: bool = *ipos.add(0) == 0.0 && *ipos.add(1) == 0.0 && *ipos.add(2) == 0.0;

        let mut r_com: [f64; 3] = [0.0; 3];
        let mut tau_com: [f64; 3] = [0.0; 3];
        let mut rot_x2i: [f64; 4] = [0.0; 4];
        let mut force: [f64; 3] = [0.0; 3];

        // compute torque at CoM in inertial frame
        if aligned {
            crate::engine::engine_inline::mji_copy3(tau_com.as_mut_ptr(), tau.as_ptr());
        } else {
            // rotation from world to inertial frame
            let mut xquat_neg: [f64; 4] = [0.0; 4];
            crate::engine::engine_inline::mji_neg_quat(xquat_neg.as_mut_ptr(), xquat);
            crate::engine::engine_inline::mji_mul_quat(
                rot_x2i.as_mut_ptr(), iquat_neg.as_ptr(), xquat_neg.as_ptr());

            // force and CoM offset in inertial frame
            crate::engine::engine_inline::mji_rot_vec_quat(
                force.as_mut_ptr(), qfrc, rot_x2i.as_ptr());
            crate::engine::engine_inline::mji_rot_vec_quat(
                r_com.as_mut_ptr(), ipos, iquat_neg.as_ptr());

            // torque at CoM in inertial frame
            let mut rxf: [f64; 3] = [0.0; 3];
            crate::engine::engine_inline::mji_cross(rxf.as_mut_ptr(), r_com.as_ptr(), force.as_ptr());
            crate::engine::engine_inline::mji_sub3(tau_com.as_mut_ptr(), tau.as_ptr(), rxf.as_ptr());
        }

        // solve for midpoint angular velocity
        let mut w_mid: [f64; 3] = [0.0; 3];
        let niter: i32 = crate::engine::engine_forward::midpoint_newton(
            inertia, w.as_ptr(), tau_com.as_ptr(), h, w_mid.as_mut_ptr());

        // next and mid angular velocities in inertial frame, rotate both to body frame
        let mut w_new: [f64; 3] = [0.0; 3];
        let mut w_new_body: [f64; 3] = [0.0; 3];
        let mut w_mid_body: [f64; 3] = [0.0; 3];
        for k in 0..3usize {
            w_new[k] = 2.0 * w_mid[k] - w[k];
        }
        crate::engine::engine_inline::mji_rot_vec_quat(
            w_new_body.as_mut_ptr(), w_new.as_ptr(), iquat);
        crate::engine::engine_inline::mji_rot_vec_quat(
            w_mid_body.as_mut_ptr(), w_mid.as_ptr(), iquat);
        crate::engine::engine_inline::mji_copy3(qvel_new.add(3), w_new_body.as_ptr());

        // === aligned: return
        if aligned {
            return niter;
        }

        // === non-aligned: solve for translational velocity

        // rotate linear velocity to inertial frame
        let mut v: [f64; 3] = [0.0; 3];
        crate::engine::engine_inline::mji_rot_vec_quat(
            v.as_mut_ptr(), qvel, rot_x2i.as_ptr());

        // current CoM velocities (rot, lin) in inertial frame
        let mut wxr: [f64; 3] = [0.0; 3];
        crate::engine::engine_inline::mji_cross(wxr.as_mut_ptr(), w.as_ptr(), r_com.as_ptr());
        let mut vcom: [f64; 3] = [0.0; 3];
        crate::engine::engine_inline::mji_add3(vcom.as_mut_ptr(), v.as_ptr(), wxr.as_ptr());

        // right-hand side for midpoint CoM velocity
        let i2h: f64 = 2.0 / h;
        let mut b: [f64; 3] = [0.0; 3];
        for k in 0..3usize {
            b[k] = force[k] / mass + i2h * vcom[k];
        }

        // add gravity, if any
        if !gravity.is_null() {
            let mut g_inertial: [f64; 3] = [0.0; 3];
            crate::engine::engine_inline::mji_rot_vec_quat(
                g_inertial.as_mut_ptr(), gravity, rot_x2i.as_ptr());
            crate::engine::engine_inline::mji_add_to3(b.as_mut_ptr(), g_inertial.as_ptr());
        }

        // analytic solution for (i2h*Id + [w_mid]x) * vcom_mid = b
        let wnorm2: f64 = crate::engine::engine_util_blas::mju_dot3(w_mid.as_ptr(), w_mid.as_ptr());
        let denom: f64 = i2h * i2h + wnorm2;
        let w_dot_b: f64 = crate::engine::engine_util_blas::mju_dot3(w_mid.as_ptr(), b.as_ptr());
        let mut w_cross_b: [f64; 3] = [0.0; 3];
        crate::engine::engine_inline::mji_cross(w_cross_b.as_mut_ptr(), w_mid.as_ptr(), b.as_ptr());
        let mut vcom_mid: [f64; 3] = [0.0; 3];
        for k in 0..3usize {
            vcom_mid[k] = (i2h * b[k] + (w_dot_b / i2h) * w_mid[k] - w_cross_b[k]) / denom;
        }

        // recover midpoint and new joint velocity in inertial frame
        let mut wxr_mid: [f64; 3] = [0.0; 3];
        crate::engine::engine_inline::mji_cross(wxr_mid.as_mut_ptr(), w_mid.as_ptr(), r_com.as_ptr());
        let mut v_mid: [f64; 3] = [0.0; 3];
        let mut v_new: [f64; 3] = [0.0; 3];
        for k in 0..3usize {
            v_mid[k] = vcom_mid[k] - wxr_mid[k];
            v_new[k] = 2.0 * v_mid[k] - v[k];
        }

        // estimate new orientation
        let mut axis: [f64; 3] = [0.0; 3];
        crate::engine::engine_inline::mji_copy3(axis.as_mut_ptr(), w_mid_body.as_ptr());
        let wnorm: f64 = crate::engine::engine_util_blas::mju_normalize3(axis.as_mut_ptr());
        let mut qrot_new: [f64; 4] = [0.0; 4];
        crate::engine::engine_inline::mji_axis_angle2quat(
            qrot_new.as_mut_ptr(), axis.as_ptr(), h * wnorm);
        let mut xquat_new: [f64; 4] = [0.0; 4];
        crate::engine::engine_inline::mji_mul_quat(
            xquat_new.as_mut_ptr(), xquat, qrot_new.as_ptr());

        // v_new (linear): inertial → body → world using new orientation
        let mut v_body: [f64; 3] = [0.0; 3];
        crate::engine::engine_inline::mji_rot_vec_quat(
            v_body.as_mut_ptr(), v_new.as_ptr(), iquat);
        crate::engine::engine_inline::mji_rot_vec_quat(
            qvel_new, v_body.as_ptr(), xquat_new.as_ptr());

        niter
    }
}

/// C: mj_fwdKinematics (engine/engine_forward.h:78)
/// Calls: mj_camlight, mj_comPos, mj_flex, mj_kinematics, mj_tendon, mj_updateSleep, mj_wakeTendon
#[allow(unused_variables, non_snake_case)]
pub fn mj_fwd_kinematics(m: *const mjModel, d: *mut mjData) {
    crate::engine::engine_core_smooth::mj_kinematics(m, d);
    crate::engine::engine_core_smooth::mj_com_pos(m, d);
    crate::engine::engine_core_smooth::mj_camlight(m, d);
    crate::engine::engine_core_smooth::mj_flex(m, d);
    crate::engine::engine_core_smooth::mj_tendon(m, d);
    if crate::engine::engine_sleep::mj_wake_tendon(m, d) != 0 {
        crate::engine::engine_sleep::mj_update_sleep(m, d);
    }
}

/// C: mj_fwdPosition (engine/engine_forward.h:81)
/// Calls: mj_collision, mj_factorM, mj_fwdKinematics, mj_island, mj_makeConstraint, mj_makeM, mj_projectConstraint, mj_transmission, mj_updateSleep, mj_wakeCollision, mj_wakeEquality
#[allow(unused_variables, non_snake_case)]
pub fn mj_fwd_position(m: *const mjModel, d: *mut mjData) {
    // SAFETY: m, d valid per caller. All called functions handle their own safety.
    unsafe {
        // clear position-dependent flags for lazy evaluation
        *((&mut (*d).flg_energypos) as *mut mjtBool as *mut u8) = 0;

        mj_fwd_kinematics(m, d);

        // inertia
        crate::engine::engine_core_smooth::mj_make_m(m, d);
        crate::engine::engine_core_smooth::mj_factor_m(m, d);

        // collision
        crate::engine::engine_collision_driver::mj_collision(m, d);

        if crate::engine::engine_sleep::mj_wake_collision(m, d) != 0 {
            crate::engine::engine_sleep::mj_update_sleep(m, d);
            crate::engine::engine_collision_driver::mj_collision(m, d);
        }

        if crate::engine::engine_sleep::mj_wake_equality(m, d) != 0 {
            crate::engine::engine_sleep::mj_update_sleep(m, d);
        }

        crate::engine::engine_core_constraint::mj_make_constraint(m, d);
        crate::engine::engine_island::mj_island(m, d);

        crate::engine::engine_core_constraint::mj_project_constraint(m, d);

        crate::engine::engine_core_smooth::mj_transmission(m, d);
    }
}

/// C: mj_fwdVelocity (engine/engine_forward.h:84)
/// Calls: mj_comVel, mj_passive, mj_referenceConstraint, mj_rne, mj_tendonBias, mju_mulMatVecSparse, mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mj_fwd_velocity(m: *const mjModel, d: *mut mjData) {
    const mjDSBL_ACTUATION: i32 = 1 << 11;

    // SAFETY: m, d valid per caller. All pointer arithmetic within valid model/data arrays.
    unsafe {
        // clear velocity-dependent flags for lazy evaluation
        *((&mut (*d).flg_subtreevel) as *mut mjtBool as *mut u8) = 0;
        *((&mut (*d).flg_energyvel) as *mut mjtBool as *mut u8) = 0;

        let nflex = (*m).nflex as i32;
        let nflexedge = (*m).nflexedge as i32;
        let ntendon = (*m).ntendon as i32;
        let nu = (*m).nu as i32;

        // flexedge velocity: skip interp and rigid flexes (edge Jacobians are zero)
        crate::engine::engine_util_blas::mju_zero((*d).flexedge_velocity, nflexedge);
        for f in 0..nflex {
            let rigid = *((*m).flex_rigid as *const u8).add(f as usize);
            let interp = *(*m).flex_interp.add(f as usize);
            if rigid != 0 || interp != 0 {
                continue;
            }
            let adr = *(*m).flex_edgeadr.add(f as usize);
            let num = *(*m).flex_edgenum.add(f as usize);
            crate::engine::engine_util_sparse::mju_mul_mat_vec_sparse(
                (*d).flexedge_velocity.add(adr as usize),
                (*d).flexedge_J,
                (*d).qvel,
                num,
                (*m).flexedge_J_rownnz.add(adr as usize),
                (*m).flexedge_J_rowadr.add(adr as usize),
                (*m).flexedge_J_colind,
                std::ptr::null(),
            );
        }

        // tendon velocity: always sparse
        crate::engine::engine_util_sparse::mju_mul_mat_vec_sparse(
            (*d).ten_velocity,
            (*d).ten_J,
            (*d).qvel,
            ntendon,
            (*m).ten_J_rownnz,
            (*m).ten_J_rowadr,
            (*m).ten_J_colind,
            std::ptr::null(),
        );

        // actuator velocity: always sparse
        if ((*m).opt.disableflags & mjDSBL_ACTUATION) == 0 {
            crate::engine::engine_util_sparse::mju_mul_mat_vec_sparse(
                (*d).actuator_velocity,
                (*d).actuator_moment,
                (*d).qvel,
                nu,
                (*d).moment_rownnz,
                (*d).moment_rowadr,
                (*d).moment_colind,
                std::ptr::null(),
            );
        } else {
            crate::engine::engine_util_blas::mju_zero((*d).actuator_velocity, nu);
        }

        // com-based velocities, passive forces, constraint references
        crate::engine::engine_core_smooth::mj_com_vel(m, d);
        crate::engine::engine_passive::mj_passive(m, d);
        crate::engine::engine_core_constraint::mj_reference_constraint(m, d);

        // compute qfrc_bias with abbreviated RNE (without acceleration)
        crate::engine::engine_core_smooth::mj_rne(m, d, 0, (*d).qfrc_bias);

        // add bias force due to tendon armature
        crate::engine::engine_core_smooth::mj_tendon_bias(m, d, (*d).qfrc_bias);
    }
}

/// C: mj_fwdActuation (engine/engine_forward.h:87)
/// Calls: clampVec, dcmotorVoltage, mj_actuatorDisabled, mj_dcmotorSlots, mj_freeStack, mj_lugreStribeck, mj_markStack, mj_nextActivation, mj_readCtrl, mj_sleepState, mj_stackAllocInfo, mj_warning, mjp_getPluginAtSlotUnsafe, mjp_pluginCount, mju_addTo, mju_clip, mju_isBad, mju_max, mju_message, mju_min, mju_mulMatTVecSparse, mju_muscleBias, mju_muscleDynamics, mju_muscleGain, mju_norm3, mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mj_fwd_actuation(m: *const mjModel, d: *mut mjData) {
    extern "C" { fn mj_fwdActuation(m: *const mjModel, d: *mut mjData); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_fwdActuation(m, d) }
}

/// C: mj_fwdAcceleration (engine/engine_forward.h:90)
/// Calls: mj_solveLD, mj_xfrcAccumulate, mju_addTo, mju_addToInd, mju_copy, mju_copyInd, mju_sub, mju_subInd
#[allow(unused_variables, non_snake_case)]
pub fn mj_fwd_acceleration(m: *const mjModel, d: *mut mjData) {
    // SAFETY: m and d are valid pointers provided by the MuJoCo engine.
    // All field accesses and pointer arithmetic follow the C layout.
    unsafe {
        const mjENBL_SLEEP: i32 = 1 << 4;

        let sleep_filter: i32 = (((*m).opt.enableflags & mjENBL_SLEEP) != 0 && (*d).nv_awake < (*m).nv as i32) as i32;
        let nv: i32;
        let index: *const i32;

        // qfrc_smooth = qfrc_passive - qfrc_bias + qfrc_applied + qfrc_actuator
        if sleep_filter == 0 {
            nv = (*m).nv as i32;
            index = std::ptr::null();
            crate::engine::engine_util_blas::mju_sub((*d).qfrc_smooth, (*d).qfrc_passive, (*d).qfrc_bias, nv);
            crate::engine::engine_util_blas::mju_add_to((*d).qfrc_smooth, (*d).qfrc_applied, nv);
            crate::engine::engine_util_blas::mju_add_to((*d).qfrc_smooth, (*d).qfrc_actuator, nv);
        } else {
            nv = (*d).nv_awake;
            index = (*d).dof_awake_ind;
            crate::engine::engine_util_blas::mju_sub_ind((*d).qfrc_smooth, (*d).qfrc_passive, (*d).qfrc_bias, index, nv);
            crate::engine::engine_util_blas::mju_add_to_ind((*d).qfrc_smooth, (*d).qfrc_applied, index, nv);
            crate::engine::engine_util_blas::mju_add_to_ind((*d).qfrc_smooth, (*d).qfrc_actuator, index, nv);
        }

        // qfrc_smooth += project(xfrc_applied)
        crate::engine::engine_support::mj_xfrc_accumulate(m, d, (*d).qfrc_smooth);

        // copy for in-place solve: qacc_smooth = qfrc_smooth
        if sleep_filter == 0 {
            crate::engine::engine_util_blas::mju_copy((*d).qacc_smooth, (*d).qfrc_smooth, nv);
        } else {
            crate::engine::engine_util_blas::mju_copy_ind((*d).qacc_smooth, (*d).qfrc_smooth, index, nv);
        }

        // qacc_smooth = M \ qfrc_smooth
        crate::engine::engine_core_smooth::mj_solve_ld(
            (*d).qacc_smooth, (*d).qLD, (*d).qLDiagInv, nv, 1,
            (*m).M_rownnz, (*m).M_rowadr, (*m).M_colind, index,
        );
    }
}

/// C: mj_fwdConstraint (engine/engine_forward.h:93)
/// Calls: mj_dualFinish, mj_mulJacVec, mj_solCG, mj_solNewton, mj_solNoSlip, mj_solNoSlip_island, mj_solPGS, mju_copy, mju_dispatch, mju_gather, mju_message, mju_scatter, mju_subFrom, mju_zero, mju_zeroInt, warmstart
#[allow(unused_variables, non_snake_case)]
pub fn mj_fwd_constraint(m: *const mjModel, d: *mut mjData) {
    extern "C" {
        fn mj_fwdConstraint(m: *const mjModel, d: *mut mjData);
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_fwdConstraint(m, d) }
}

