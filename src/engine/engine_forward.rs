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
    // WARNING: signature changed — verify body
    // Previous params: (ctrl : f64, length : f64, velocity : f64, x_I : f64, gainprm : * const f64)
    // Previous return: f64
    extern "C" { fn dcmotorVoltage_impl (ctrl : f64 , length : f64 , velocity : f64 , x_I : f64 , gainprm : * const f64) -> f64 ; } unsafe { dcmotorVoltage_impl (ctrl , length , velocity , x_I , gainprm) }
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
    extern "C" { fn clampVec_impl (vec : * mut f64 , range : * const f64 , limited : * const mjtBool , n : i32 , index : * const i32) ; } unsafe { clampVec_impl (vec , range , limited , n , index) }
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
                fn mj_stackAllocInfo_impl(d: *mut mjData, bytes: usize, alignment: usize, caller: *const i8, line: i32) -> *mut ();
            }
            let jar: *mut f64 = mj_stackAllocInfo_impl(
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

                let ARf: *mut f64 = mj_stackAllocInfo_impl(
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
                let Ma: *mut f64 = mj_stackAllocInfo_impl(
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
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, arg : * mut (), thread_id : i32, island : i32)
    // Previous return: ()
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, act_dot : * const f64, qacc : * const f64, qvel : * const f64)
    // Previous return: ()
    todo ! ()
}

/// C: flex_has_implicit_stiffness (engine/engine_forward.c:1284)
#[allow(unused_variables, non_snake_case)]
pub fn flex_has_implicit_stiffness(m: *const mjModel) -> i32 {
    extern "C" { fn flex_has_implicit_stiffness_impl(m: *const mjModel) -> i32; }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { flex_has_implicit_stiffness_impl(m) }
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
    extern "C" { fn flexInterp_cgsolve_impl(m: *const mjModel, d: *mut mjData, qacc: *mut f64, qfrc: *const f64, nv: i32); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { flexInterp_cgsolve_impl(m, d, qacc, qfrc, nv) }
}

/// C: midpoint_eligible (engine/engine_forward.c:1421)
#[allow(unused_variables, non_snake_case)]
pub fn midpoint_eligible(m: *const mjModel, d: *const mjData, jnt: i32) -> i32 {
    extern "C" { fn midpoint_eligible_impl(m: *const mjModel, d: *const mjData, jnt: i32) -> i32; }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { midpoint_eligible_impl(m, d, jnt) }
}

/// C: midpoint_aligned (engine/engine_forward.c:1493)
#[allow(unused_variables, non_snake_case)]
pub fn midpoint_aligned(m: *const mjModel, jnt: i32) -> i32 {
    extern "C" { fn midpoint_aligned_impl(m: *const mjModel, jnt: i32) -> i32; }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { midpoint_aligned_impl(m, jnt) }
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
    extern "C" { fn midpointNewton_impl(inertia: *const f64, w: *const f64, tau: *const f64, h: f64, w_mid: *mut f64) -> i32; }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { midpointNewton_impl(inertia, w, tau, h, w_mid) }
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
    extern "C" { fn midpoint_impl(m: *const mjModel, d: *const mjData, qfrc: *const f64, free_jntid: *const i32, nfree: i32, qvel_old: *mut f64, qvel_new: *mut f64, dofadr: *mut i32); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { midpoint_impl(m, d, qfrc, free_jntid, nfree, qvel_old, qvel_new, dofadr) }
}

/// C: mj_checkPos (engine/engine_forward.h:27)
/// Calls: mj_resetData, mj_warning, mju_isBad
#[allow(unused_variables, non_snake_case)]
pub fn mj_check_pos(m: *const mjModel, d: *mut mjData) {
    extern "C" { fn mj_checkPos_impl(m: *const mjModel, d: *mut mjData); }
    // SAFETY: delegates to C implementation; caller guarantees m and d are valid pointers
    unsafe { mj_checkPos_impl(m, d) }
}

/// C: mj_checkVel (engine/engine_forward.h:28)
/// Calls: mj_resetData, mj_warning, mju_isBad
#[allow(unused_variables, non_snake_case)]
pub fn mj_check_vel(m: *const mjModel, d: *mut mjData) {
    extern "C" { fn mj_checkVel_impl(m: *const mjModel, d: *mut mjData); }
    // SAFETY: delegates to C implementation; caller guarantees m and d are valid pointers
    unsafe { mj_checkVel_impl(m, d) }
}

/// C: mj_checkAcc (engine/engine_forward.h:29)
/// Calls: mj_forward, mj_resetData, mj_warning, mju_isBad
#[allow(unused_variables, non_snake_case)]
pub fn mj_check_acc(m: *const mjModel, d: *mut mjData) {



    extern "C" { fn mj_checkAcc_impl(m: *const mjModel, d: *mut mjData); }
    // SAFETY: delegates to C implementation
    unsafe { mj_checkAcc_impl(m, d) }
}

/// C: mj_step (engine/engine_forward.h:35)
/// Calls: mj_Euler, mj_RungeKutta, mj_checkAcc, mj_checkPos, mj_checkVel, mj_compareFwdInv, mj_forward, mj_implicit, mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_step(m: *const mjModel, d: *mut mjData) {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData)
    // Previous return: ()
    todo ! ()
}

/// C: mj_step1 (engine/engine_forward.h:38)
/// Calls: mj_checkPos, mj_checkVel, mj_energyPos, mj_energyVel, mj_fwdPosition, mj_fwdVelocity, mj_sensorPos, mj_sensorVel
#[allow(unused_variables, non_snake_case)]
pub fn mj_step1(m: *const mjModel, d: *mut mjData) {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData)
    // Previous return: ()
    todo ! ()
}

/// C: mj_step2 (engine/engine_forward.h:41)
/// Calls: mj_Euler, mj_checkAcc, mj_compareFwdInv, mj_fwdAcceleration, mj_fwdActuation, mj_fwdConstraint, mj_implicit, mj_sensorAcc
#[allow(unused_variables, non_snake_case)]
pub fn mj_step2(m: *const mjModel, d: *mut mjData) {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData)
    // Previous return: ()
    todo ! ()
}

/// C: mj_forward (engine/engine_forward.h:44)
/// Calls: mj_forwardSkip
#[allow(unused_variables, non_snake_case)]
pub fn mj_forward(m: *const mjModel, d: *mut mjData) {
    extern "C" { fn mj_forward_impl(m: *const mjModel, d: *mut mjData); }
    // SAFETY: delegates to C implementation, pointers valid per caller contract
    unsafe { mj_forward_impl(m, d) }
}

/// C: mj_forwardSkip (engine/engine_forward.h:47)
/// Calls: mj_energyPos, mj_energyVel, mj_fwdAcceleration, mj_fwdActuation, mj_fwdConstraint, mj_fwdPosition, mj_fwdVelocity, mj_sensorAcc, mj_sensorPos, mj_sensorVel
#[allow(unused_variables, non_snake_case)]
pub fn mj_forward_skip(m: *const mjModel, d: *mut mjData, skipstage: i32, skipsensor: i32) {
    extern "C" { fn mj_forwardSkip_impl(m: *const mjModel, d: *mut mjData, skipstage: i32, skipsensor: i32); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_forwardSkip_impl(m, d, skipstage, skipsensor) }
}

/// C: mj_RungeKutta (engine/engine_forward.h:53)
/// Calls: mj_advance, mj_forwardSkip, mj_freeStack, mj_integratePos, mj_markStack, mj_stackAllocInfo, mju_addToScl, mju_copy, mju_message, mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mj_runge_kutta(m: *const mjModel, d: *mut mjData, N: i32) {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, N : i32)
    // Previous return: ()
    todo ! ()
}

/// C: mj_Euler (engine/engine_forward.h:56)
/// Calls: mj_EulerSkip
#[allow(unused_variables, non_snake_case)]
pub fn mj_euler(m: *const mjModel, d: *mut mjData) {
    extern "C" { fn mj_Euler_impl(m: *const mjModel, d: *mut mjData); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_Euler_impl(m, d) }
}

/// C: mj_EulerSkip (engine/engine_forward.h:59)
/// Calls: mj_actuatorDamping, mj_advance, mj_factorI, mj_freeStack, mj_markStack, mj_solveLD, mj_stackAllocInfo, mjd_xPolyForce, mju_add, mju_addInd, mju_copy, mju_copyInd, mju_copySparse, mju_isZero
#[allow(unused_variables, non_snake_case)]
pub fn mj_euler_skip(m: *const mjModel, d: *mut mjData, skipfactor: i32) {
    extern "C" { fn mj_EulerSkip_impl(m: *const mjModel, d: *mut mjData, skipfactor: i32); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_EulerSkip_impl(m, d, skipfactor) }
}

/// C: mj_implicit (engine/engine_forward.h:62)
/// Calls: mj_implicitSkip
#[allow(unused_variables, non_snake_case)]
pub fn mj_implicit(m: *const mjModel, d: *mut mjData) {
    extern "C" { fn mj_implicit_impl(m: *const mjModel, d: *mut mjData); }
    // SAFETY: delegates to C implementation, pointers valid per caller contract
    unsafe { mj_implicit_impl(m, d) }
}

/// C: mj_implicitSkip (engine/engine_forward.h:65)
/// Calls: flexInterp_cgsolve, flex_has_implicit_stiffness, midpoint, midpoint_aligned, midpoint_eligible, mj_advance, mj_factorI, mj_freeStack, mj_markStack, mj_solveLD, mj_stackAllocInfo, mjd_smooth_vel, mju_add, mju_addInd, mju_addScl, mju_addToScl, mju_copy, mju_copyInd, mju_factorLUSparse, mju_gather, mju_gatherMasked, mju_message, mju_solveLUSparse
#[allow(unused_variables, non_snake_case)]
pub fn mj_implicit_skip(m: *const mjModel, d: *mut mjData, skipfactor: i32) {
    extern "C" { fn mj_implicitSkip_impl(m: *const mjModel, d: *mut mjData, skipfactor: i32); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_implicitSkip_impl(m, d, skipfactor) }
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
    extern "C" { fn mj_midpoint_impl(mass: f64, inertia: *const f64, ipos: *const f64, iquat: *const f64, xquat: *const f64, qvel: *const f64, qfrc: *const f64, gravity: *const f64, h: f64, qvel_new: *mut f64) -> i32; }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_midpoint_impl(mass, inertia, ipos, iquat, xquat, qvel, qfrc, gravity, h, qvel_new) }
}

/// C: mj_fwdKinematics (engine/engine_forward.h:78)
/// Calls: mj_camlight, mj_comPos, mj_flex, mj_kinematics, mj_tendon, mj_updateSleep, mj_wakeTendon
#[allow(unused_variables, non_snake_case)]
pub fn mj_fwd_kinematics(m: *const mjModel, d: *mut mjData) {
    extern "C" { fn mj_fwdKinematics_impl(m: *const mjModel, d: *mut mjData); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_fwdKinematics_impl(m, d) }
}

/// C: mj_fwdPosition (engine/engine_forward.h:81)
/// Calls: mj_collision, mj_factorM, mj_fwdKinematics, mj_island, mj_makeConstraint, mj_makeM, mj_projectConstraint, mj_transmission, mj_updateSleep, mj_wakeCollision, mj_wakeEquality
#[allow(unused_variables, non_snake_case)]
pub fn mj_fwd_position(m: *const mjModel, d: *mut mjData) {
    extern "C" { fn mj_fwdPosition_impl(m: *const mjModel, d: *mut mjData); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_fwdPosition_impl(m, d) }
}

/// C: mj_fwdVelocity (engine/engine_forward.h:84)
/// Calls: mj_comVel, mj_passive, mj_referenceConstraint, mj_rne, mj_tendonBias, mju_mulMatVecSparse, mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mj_fwd_velocity(m: *const mjModel, d: *mut mjData) {
    extern "C" { fn mj_fwdVelocity_impl(m: *const mjModel, d: *mut mjData); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_fwdVelocity_impl(m, d) }
}

/// C: mj_fwdActuation (engine/engine_forward.h:87)
/// Calls: clampVec, dcmotorVoltage, mj_actuatorDisabled, mj_dcmotorSlots, mj_freeStack, mj_lugreStribeck, mj_markStack, mj_nextActivation, mj_readCtrl, mj_sleepState, mj_stackAllocInfo, mj_warning, mjp_getPluginAtSlotUnsafe, mjp_pluginCount, mju_addTo, mju_clip, mju_isBad, mju_max, mju_message, mju_min, mju_mulMatTVecSparse, mju_muscleBias, mju_muscleDynamics, mju_muscleGain, mju_norm3, mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mj_fwd_actuation(m: *const mjModel, d: *mut mjData) {
    extern "C" { fn mj_fwdActuation_impl(m: *const mjModel, d: *mut mjData); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_fwdActuation_impl(m, d) }
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
        fn mj_fwdConstraint_impl(m: *const mjModel, d: *mut mjData);
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_fwdConstraint_impl(m, d) }
}

