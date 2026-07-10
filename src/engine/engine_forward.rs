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
pub fn dcmotor_voltage(ctrl: f64, length: f64, velocity: f64, x_I: f64, gainprm: *const f64) -> f64 {
    // SAFETY: gainprm points to at least 9 f64 elements per caller contract
    unsafe {
        let input_mode: i32 = *gainprm.add(8) as i32;
        let Vmax: f64 = *gainprm.add(7);
        let mut voltage: f64;

        // get voltage
        if input_mode > 0 {
            let kp: f64 = *gainprm.add(4);  // proportional gain
            let ki: f64 = *gainprm.add(5);  // integral gain
            let kd: f64 = *gainprm.add(6);  // derivative gain

            if input_mode == 1 {
                // position mode
                voltage = kp * (ctrl - length) + ki * x_I - kd * velocity;
            } else {
                // velocity mode
                voltage = kp * (ctrl - velocity) + ki * (x_I - length);
            }
        } else {
            voltage = ctrl;
        }

        // clip voltage
        if Vmax > 0.0 {
            voltage = crate::engine::engine_util_misc::mju_clip(voltage, -Vmax, Vmax);
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
    // WARNING: signature changed — verify body
    // Previous params: (vec : * mut f64, range : * const f64, limited : * const mjtBool, n : i32, index : * const i32)
    // Previous return: ()
    extern "C" { fn clampVec(vec : * mut f64 , range : * const f64 , limited : * const mjtBool , n : i32 , index : * const i32) ; } unsafe { clampVec(vec , range , limited , n , index) }
}

/// C: warmstart (engine/engine_forward.c:786)
/// Calls: mj_constraintUpdate, mj_freeStack, mj_isSparse, mj_markStack, mj_mulJacVec, mj_mulM, mj_stackAllocInfo, mju_copy, mju_dot, mju_mulMatVec, mju_mulMatVecSparse, mju_subFrom, mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn warmstart(m: *const mjModel, d: *mut mjData) {
    // SAFETY: m and d are valid pointers provided by the MuJoCo engine.
    // All field accesses and pointer arithmetic follow the C layout.
    // Stack allocation uses the MuJoCo arena allocator via C FFI.
    unsafe {
        const mjDSBL_WARMSTART: i32 = 1 << 9;
        const mjSOL_PGS: i32 = 0;

        let nv: i32 = (*m).nv as i32;
        let nefc: i32 = (*d).nefc;

        // warmstart with best of (qacc_warmstart, qacc_smooth)
        if ((*m).opt.disableflags & mjDSBL_WARMSTART) == 0 {
            crate::engine::engine_memory::mj_mark_stack(d);

            // SAFETY: mj_stackAllocInfo returns aligned memory from MuJoCo arena
            extern "C" {
                fn mj_stackAllocInfo(d: *mut mjData, bytes: usize, alignment: usize, caller: *const i8, line: i32) -> *mut ();
            }
            let jar: *mut f64 = mj_stackAllocInfo(
                d,
                (nefc as usize) * std::mem::size_of::<f64>(),
                std::mem::align_of::<f64>(),
                b"warmstart\0".as_ptr() as *const i8,
                0,
            ) as *mut f64;

            // start with qacc = qacc_warmstart
            crate::engine::engine_util_blas::mju_copy((*d).qacc, (*d).qacc_warmstart, nv);

            // compute jar(qacc_warmstart)
            crate::engine::engine_core_constraint::mj_mul_jac_vec(m, d as *const mjData, jar, (*d).qacc_warmstart);
            crate::engine::engine_util_blas::mju_sub_from(jar, (*d).efc_aref, nefc);

            // update constraints, save cost(qacc_warmstart)
            let mut cost_warmstart: f64 = 0.0;
            crate::engine::engine_core_constraint::mj_constraint_update(m, d, jar, &mut cost_warmstart, 0);

            // PGS
            if (*m).opt.solver == mjSOL_PGS {
                // cost(force_warmstart)
                let mut PGS_warmstart: f64 = crate::engine::engine_util_blas::mju_dot((*d).efc_force, (*d).efc_b, nefc);

                let ARf: *mut f64 = mj_stackAllocInfo(
                    d,
                    (nefc as usize) * std::mem::size_of::<f64>(),
                    std::mem::align_of::<f64>(),
                    b"warmstart\0".as_ptr() as *const i8,
                    0,
                ) as *mut f64;

                if crate::engine::engine_core_util::mj_is_sparse(m) != 0 {
                    crate::engine::engine_util_sparse::mju_mul_mat_vec_sparse(
                        ARf, (*d).efc_AR, (*d).efc_force, nefc,
                        (*d).efc_AR_rownnz, (*d).efc_AR_rowadr,
                        (*d).efc_AR_colind, std::ptr::null(),
                    );
                } else {
                    crate::engine::engine_util_blas::mju_mul_mat_vec(ARf, (*d).efc_AR, (*d).efc_force, nefc, nefc);
                }
                PGS_warmstart += 0.5 * crate::engine::engine_util_blas::mju_dot((*d).efc_force, ARf, nefc);

                // use zero if better
                if PGS_warmstart > 0.0 {
                    crate::engine::engine_util_blas::mju_zero((*d).efc_force, nefc);
                    crate::engine::engine_util_blas::mju_zero((*d).qfrc_constraint, nv);
                }
            }
            // non-PGS
            else {
                // add Gauss to cost(qacc_warmstart)
                let Ma: *mut f64 = mj_stackAllocInfo(
                    d,
                    (nv as usize) * std::mem::size_of::<f64>(),
                    std::mem::align_of::<f64>(),
                    b"warmstart\0".as_ptr() as *const i8,
                    0,
                ) as *mut f64;

                crate::engine::engine_support::mj_mul_m(m, d as *const mjData, Ma, (*d).qacc_warmstart);
                for i in 0..nv as usize {
                    cost_warmstart += 0.5
                        * (*Ma.add(i) - *(*d).qfrc_smooth.add(i))
                        * (*(*d).qacc_warmstart.add(i) - *(*d).qacc_smooth.add(i));
                }

                // cost(qacc_smooth)
                let mut cost_smooth: f64 = 0.0;
                crate::engine::engine_core_constraint::mj_constraint_update(m, d, (*d).efc_b, &mut cost_smooth, 0);

                // use qacc_smooth if better
                if cost_warmstart > cost_smooth {
                    crate::engine::engine_util_blas::mju_copy((*d).qacc, (*d).qacc_smooth, nv);
                }
            }

            // have island structure: unconstrained qacc = qacc_smooth
            if (*d).nisland > 0 {
                // loop over unconstrained dofs in map_idof2dof[nidof, nv)
                for i in (*d).nidof..nv {
                    let dof: i32 = *(*d).map_idof2dof.add(i as usize);
                    *(*d).qacc.add(dof as usize) = *(*d).qacc_smooth.add(dof as usize);
                }
            }

            crate::engine::engine_memory::mj_free_stack(d);
        }
        // coldstart with qacc = qacc_smooth, efc_force = 0
        else {
            crate::engine::engine_util_blas::mju_copy((*d).qacc, (*d).qacc_smooth, nv);
            crate::engine::engine_util_blas::mju_zero((*d).efc_force, nefc);
        }
    }
}

/// C: solveIslandTask (engine/engine_forward.c:866)
/// Calls: mj_solCG_island, mj_solNewton_island, mj_solPGS_island
#[allow(unused_variables, non_snake_case)]
pub fn solve_island_task(m: *const mjModel, d: *mut mjData, arg: *mut (), thread_id: i32, island: i32) {
    // SAFETY: m is valid mjModel pointer. Solver functions are already translated/delegated.
    unsafe {
        const mjSOL_NEWTON: i32 = 2;
        const mjSOL_CG: i32 = 1;
        let solver = (*m).opt.solver;
        let iterations = (*m).opt.iterations;
        if solver == mjSOL_NEWTON {
            crate::engine::engine_solver::mj_sol_newton_island(m, d, island, iterations);
        } else if solver == mjSOL_CG {
            crate::engine::engine_solver::mj_sol_cg_island(m, d, island, iterations);
        } else {
            crate::engine::engine_solver::mj_sol_pgs_island(m, d, island, iterations);
        }
    }
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
pub fn flex_has_implicit_stiffness(m: *const mjModel) -> i32 {
    extern "C" { fn flex_has_implicit_stiffness(m: *const mjModel) -> i32; }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
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
pub fn midpoint_eligible(m: *const mjModel, d: *const mjData, jnt: i32) -> i32 {
    extern "C" { fn midpoint_eligible(m: *const mjModel, d: *const mjData, jnt: i32) -> i32; }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { midpoint_eligible(m, d, jnt) }
}

/// C: midpoint_aligned (engine/engine_forward.c:1493)
#[allow(unused_variables, non_snake_case)]
pub fn midpoint_aligned(m: *const mjModel, jnt: i32) -> i32 {
    // SAFETY: m is a valid mjModel pointer, jnt is a valid joint index.
    // jnt_bodyid and body_ipos are valid arrays per caller contract.
    unsafe {
        let body = *(*m).jnt_bodyid.add(jnt as usize);
        ((*(*m).body_ipos.add(3 * body as usize) == 0.0) &&
         (*(*m).body_ipos.add(3 * body as usize + 1) == 0.0) &&
         (*(*m).body_ipos.add(3 * body as usize + 2) == 0.0)) as i32
    }
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
    // SAFETY: inertia, w, tau point to 3 f64; w_mid points to 3 writable f64
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
        crate::engine::engine_inline::mji_copy3(w_mid, w);

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
            crate::engine::engine_inline::mji_cross(
                coriolis.as_mut_ptr(), w_mid as *const f64, Iw.as_ptr());

            // residual: f = i2h*I*(w_mid - w) + w_mid x (I*w_mid) - tau
            let mut f: [f64; 3] = [0.0; 3];
            for k in 0..3usize {
                f[k] = i2h_I[k] * (*w_mid.add(k) - *w.add(k)) + coriolis[k] - *tau.add(k);
            }

            // check convergence
            let fnorm: f64 = crate::engine::engine_util_blas::mju_norm3(f.as_ptr());
            let tol: f64 = 1e-13;
            if fnorm < tol * (1.0 + i2h * crate::engine::engine_util_blas::mju_norm3(Iw.as_ptr())) {
                break;
            }

            // Jacobian: J = i2h*diag(I) + d(w x Iw)/dw
            let mut J: [f64; 9] = [0.0; 9];
            J[0] = i2h_I[0];                  J[1] = *w_mid.add(2) * dI[0]; J[2] = *w_mid.add(1) * dI[0];
            J[3] = *w_mid.add(2) * dI[1];    J[4] = i2h_I[1];               J[5] = *w_mid.add(0) * dI[1];
            J[6] = *w_mid.add(1) * dI[2];    J[7] = *w_mid.add(0) * dI[2]; J[8] = i2h_I[2];

            // solve J*delta = -f for search direction delta
            let neg_f: [f64; 3] = [-f[0], -f[1], -f[2]];
            let mut delta: [f64; 3] = [0.0; 3];
            crate::engine::engine_util_solve::mju_solve3(
                delta.as_mut_ptr(), J.as_ptr(), neg_f.as_ptr());

            // backtracking line search
            let mut step: f64 = 1.0;
            for _ls in 0..20 {
                // candidate step
                let mut w_try: [f64; 3] = [0.0; 3];
                let mut Iw_try: [f64; 3] = [0.0; 3];
                for k in 0..3usize {
                    w_try[k] = *w_mid.add(k) + step * delta[k];
                    Iw_try[k] = *inertia.add(k) * w_try[k];
                }
                let mut coriolis_try: [f64; 3] = [0.0; 3];
                crate::engine::engine_inline::mji_cross(
                    coriolis_try.as_mut_ptr(), w_try.as_ptr(), Iw_try.as_ptr());

                // residual at candidate step
                let mut f_try: [f64; 3] = [0.0; 3];
                for k in 0..3usize {
                    f_try[k] = i2h_I[k] * (w_try[k] - *w.add(k)) + coriolis_try[k] - *tau.add(k);
                }

                // accept step if residual decreased, otherwise backtrack
                if crate::engine::engine_util_blas::mju_norm3(f_try.as_ptr()) < fnorm {
                    crate::engine::engine_inline::mji_copy3(w_mid, w_try.as_ptr());
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
    extern "C" { fn midpoint(m: *const mjModel, d: *const mjData, qfrc: *const f64, free_jntid: *const i32, nfree: i32, qvel_old: *mut f64, qvel_new: *mut f64, dofadr: *mut i32); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { midpoint(m, d, qfrc, free_jntid, nfree, qvel_old, qvel_new, dofadr) }
}

/// C: mj_checkPos (engine/engine_forward.h:27)
/// Calls: mj_resetData, mj_warning, mju_isBad
#[allow(unused_variables, non_snake_case)]
pub fn mj_check_pos(m: *const mjModel, d: *mut mjData) {
    extern "C" { fn mj_checkPos(m: *const mjModel, d: *mut mjData); }
    // SAFETY: delegates to C implementation; caller guarantees m and d are valid pointers
    unsafe { mj_checkPos(m, d) }
}

/// C: mj_checkVel (engine/engine_forward.h:28)
/// Calls: mj_resetData, mj_warning, mju_isBad
#[allow(unused_variables, non_snake_case)]
pub fn mj_check_vel(m: *const mjModel, d: *mut mjData) {
    extern "C" { fn mj_checkVel(m: *const mjModel, d: *mut mjData); }
    // SAFETY: delegates to C implementation; caller guarantees m and d are valid pointers
    unsafe { mj_checkVel(m, d) }
}

/// C: mj_checkAcc (engine/engine_forward.h:29)
/// Calls: mj_forward, mj_resetData, mj_warning, mju_isBad
#[allow(unused_variables, non_snake_case)]
pub fn mj_check_acc(m: *const mjModel, d: *mut mjData) {



    extern "C" { fn mj_checkAcc(m: *const mjModel, d: *mut mjData); }
    // SAFETY: delegates to C implementation
    unsafe { mj_checkAcc(m, d) }
}

/// C: mj_step (engine/engine_forward.h:35)
/// Calls: mj_Euler, mj_RungeKutta, mj_checkAcc, mj_checkPos, mj_checkVel, mj_compareFwdInv, mj_forward, mj_implicit, mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_step(m: *const mjModel, d: *mut mjData) {
    extern "C" {
        fn mj_step(m: *const mjModel, d: *mut mjData);
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_step(m, d) }
}

/// C: mj_step1 (engine/engine_forward.h:38)
/// Calls: mj_checkPos, mj_checkVel, mj_energyPos, mj_energyVel, mj_fwdPosition, mj_fwdVelocity, mj_sensorPos, mj_sensorVel
#[allow(unused_variables, non_snake_case)]
pub fn mj_step1(m: *const mjModel, d: *mut mjData) {
    extern "C" { fn mj_step1(m: *const mjModel, d: *mut mjData); }
    // SAFETY: delegates to C implementation
    unsafe { mj_step1(m, d) }
}

/// C: mj_step2 (engine/engine_forward.h:41)
/// Calls: mj_Euler, mj_checkAcc, mj_compareFwdInv, mj_fwdAcceleration, mj_fwdActuation, mj_fwdConstraint, mj_implicit, mj_sensorAcc
#[allow(unused_variables, non_snake_case)]
pub fn mj_step2(m: *const mjModel, d: *mut mjData) {
    extern "C" { fn mj_step2(m: *const mjModel, d: *mut mjData); }
    // SAFETY: delegates to C implementation, pointers valid per caller
    unsafe { mj_step2(m, d) }
}

/// C: mj_forward (engine/engine_forward.h:44)
/// Calls: mj_forwardSkip
#[allow(unused_variables, non_snake_case)]
pub fn mj_forward(m: *const mjModel, d: *mut mjData) {
    extern "C" { fn mj_forward(m: *const mjModel, d: *mut mjData); }
    // SAFETY: delegates to C implementation, pointers valid per caller contract
    unsafe { mj_forward(m, d) }
}

/// C: mj_forwardSkip (engine/engine_forward.h:47)
/// Calls: mj_energyPos, mj_energyVel, mj_fwdAcceleration, mj_fwdActuation, mj_fwdConstraint, mj_fwdPosition, mj_fwdVelocity, mj_sensorAcc, mj_sensorPos, mj_sensorVel
#[allow(unused_variables, non_snake_case)]
pub fn mj_forward_skip(m: *const mjModel, d: *mut mjData, skipstage: i32, skipsensor: i32) {
    extern "C" { fn mj_forwardSkip(m: *const mjModel, d: *mut mjData, skipstage: i32, skipsensor: i32); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_forwardSkip(m, d, skipstage, skipsensor) }
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
    extern "C" { fn mj_Euler(m: *const mjModel, d: *mut mjData); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_Euler(m, d) }
}

/// C: mj_EulerSkip (engine/engine_forward.h:59)
/// Calls: mj_actuatorDamping, mj_advance, mj_factorI, mj_freeStack, mj_markStack, mj_solveLD, mj_stackAllocInfo, mjd_xPolyForce, mju_add, mju_addInd, mju_copy, mju_copyInd, mju_copySparse, mju_isZero
#[allow(unused_variables, non_snake_case)]
pub fn mj_euler_skip(m: *const mjModel, d: *mut mjData, skipfactor: i32) {
    extern "C" { fn mj_EulerSkip(m: *const mjModel, d: *mut mjData, skipfactor: i32); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_EulerSkip(m, d, skipfactor) }
}

/// C: mj_implicit (engine/engine_forward.h:62)
/// Calls: mj_implicitSkip
#[allow(unused_variables, non_snake_case)]
pub fn mj_implicit(m: *const mjModel, d: *mut mjData) {
    extern "C" { fn mj_implicit(m: *const mjModel, d: *mut mjData); }
    // SAFETY: delegates to C implementation, pointers valid per caller contract
    unsafe { mj_implicit(m, d) }
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
    extern "C" { fn mj_fwdKinematics(m: *const mjModel, d: *mut mjData); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_fwdKinematics(m, d) }
}

/// C: mj_fwdPosition (engine/engine_forward.h:81)
/// Calls: mj_collision, mj_factorM, mj_fwdKinematics, mj_island, mj_makeConstraint, mj_makeM, mj_projectConstraint, mj_transmission, mj_updateSleep, mj_wakeCollision, mj_wakeEquality
#[allow(unused_variables, non_snake_case)]
pub fn mj_fwd_position(m: *const mjModel, d: *mut mjData) {
    extern "C" { fn mj_fwdPosition(m: *const mjModel, d: *mut mjData); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_fwdPosition(m, d) }
}

/// C: mj_fwdVelocity (engine/engine_forward.h:84)
/// Calls: mj_comVel, mj_passive, mj_referenceConstraint, mj_rne, mj_tendonBias, mju_mulMatVecSparse, mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mj_fwd_velocity(m: *const mjModel, d: *mut mjData) {
    extern "C" { fn mj_fwdVelocity(m: *const mjModel, d: *mut mjData); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_fwdVelocity(m, d) }
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

