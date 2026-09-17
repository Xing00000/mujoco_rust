//! Port of: Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_forward.c
//! IR hash: 1fa44043fe383ce9
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: dcmotorVoltage (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_forward.c:222)
/// Calls: cxx:_mju_clip
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn dcmotor_voltage(ctrl: f64, length: f64, velocity: f64, x_I: f64, gainprm: *const f64) -> f64 {
    todo!() // cxx-internal:engine_forward.c.o:_dcmotorVoltage
}

/// C: clampVec (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_forward.c:253)
/// Calls: cxx:_mju_clip
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn clamp_vec(vec: *mut f64, range: *const f64, limited: *const bool, n: i32, index: *const i32) {
    todo!() // cxx-internal:engine_forward.c.o:_clampVec
}

/// C: warmstart (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_forward.c:786)
/// Calls: cxx:_mj_constraintUpdate, cxx:_mj_freeStack, cxx:_mj_isSparse, cxx:_mj_markStack, cxx:_mj_mulJacVec, cxx:_mj_mulM, cxx:_mj_stackAllocInfo, cxx:_mju_copy, cxx:_mju_dot, cxx:_mju_mulMatVec, cxx:_mju_mulMatVecSparse, cxx:_mju_subFrom, cxx:_mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn warmstart(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx-internal:engine_forward.c.o:_warmstart
}

/// C: solveIslandTask (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_forward.c:866)
/// Calls: cxx:_mj_solCG_island, cxx:_mj_solNewton_island, cxx:_mj_solPGS_island
#[allow(unused_variables, non_snake_case)]
pub fn solve_island_task(m: *const mjModel, d: *mut mjData, arg: *mut (), thread_id: i32, island: i32) {
    todo!() // cxx-internal:engine_forward.c.o:_solveIslandTask
}

/// C: mj_advance (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_forward.c:981)
/// Calls: cxx:_mj_actuatorDisabled, cxx:_mj_computeSensor, cxx:_mj_forwardSkip, cxx:_mj_integratePosInd, cxx:_mj_nextActivation, cxx:_mj_sleep, cxx:_mj_updateSleep, cxx:_mjp_getPluginAtSlotUnsafe, cxx:_mjp_pluginCount, cxx:_mju_addToScl, cxx:_mju_addToSclInd, cxx:_mju_copy, cxx:_mju_historyInsert, cxx:_mju_message
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_advance(m: *const mjModel, d: *mut mjData, act_dot: *const f64, qacc: *const f64, qvel: *const f64) {
    todo!() // cxx-internal:engine_forward.c.o:_mj_advance
}

/// C: flex_has_implicit_stiffness (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_forward.c:1284)
#[allow(unused_variables, non_snake_case)]
pub fn flex_has_implicit_stiffness(m: *const mjModel) -> i32 {
    todo!() // cxx-internal:engine_forward.c.o:_flex_has_implicit_stiffness
}

/// C: flexInterp_cgsolve (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_forward.c:1311)
/// Calls: cxx:_mj_freeStack, cxx:_mj_markStack, cxx:_mj_solveLD, cxx:_mj_stackAllocInfo, cxx:_mjd_flexBend_mul, cxx:_mjd_flexInterp_cacheKrot, cxx:_mjd_flexInterp_mul, cxx:_mju_addScl, cxx:_mju_addToScl, cxx:_mju_copy, cxx:_mju_dot, cxx:_mju_max, cxx:_mju_mulMatVecSparse, cxx:_mju_mulSymVecSparse, cxx:_mju_solveLUSparse, cxx:_mju_sub, cxx:_mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn flex_interp_cgsolve(m: *const mjModel, d: *mut mjData, qacc: *mut f64, qfrc: *const f64, nv: i32) {
    todo!() // cxx-internal:engine_forward.c.o:_flexInterp_cgsolve
}

/// C: midpoint_eligible (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_forward.c:1421)
/// Calls: cxx:__ZN7mjCMesh4treeEv
#[allow(unused_variables, non_snake_case)]
pub fn midpoint_eligible(m: *const mjModel, d: *const mjData, jnt: i32) -> i32 {
    todo!() // cxx-internal:engine_forward.c.o:_midpoint_eligible
}

/// C: midpoint_aligned (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_forward.c:1493)
#[allow(unused_variables, non_snake_case)]
pub fn midpoint_aligned(m: *const mjModel, jnt: i32) -> i32 {
    todo!() // cxx-internal:engine_forward.c.o:_midpoint_aligned
}

/// C: midpointNewton (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_forward.c:1515)
/// Calls: cxx:_mju_norm3, cxx:_mju_solve3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn midpoint_newton(inertia: *const f64, w: *const f64, tau: *const f64, h: f64, w_mid: *mut f64) -> i32 {
    todo!() // cxx-internal:engine_forward.c.o:_midpointNewton
}

/// C: midpoint (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_forward.c:1736)
/// Calls: cxx:_mj_midpoint, cxx:_mju_add, cxx:_mju_copy
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn midpoint(m: *const mjModel, d: *const mjData, qfrc: *const f64, free_jntid: *const i32, nfree: i32, qvel_old: *mut f64, qvel_new: *mut f64, dofadr: *mut i32) {
    todo!() // cxx-internal:engine_forward.c.o:_midpoint
}

/// C: mj_checkPos (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_forward.h:27)
/// Calls: cxx:_mj_resetData, cxx:_mj_warning, cxx:_mju_isBad
#[allow(unused_variables, non_snake_case)]
pub fn mj_check_pos(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx:_mj_checkPos
}

/// C: mj_checkVel (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_forward.h:28)
/// Calls: cxx:_mj_resetData, cxx:_mj_warning, cxx:_mju_isBad
#[allow(unused_variables, non_snake_case)]
pub fn mj_check_vel(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx:_mj_checkVel
}

/// C: mj_checkAcc (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_forward.h:29)
/// Calls: cxx:_mj_forward, cxx:_mj_resetData, cxx:_mj_warning, cxx:_mju_isBad
#[allow(unused_variables, non_snake_case)]
pub fn mj_check_acc(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx:_mj_checkAcc
}

/// C: mj_step (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_forward.h:35)
/// Calls: cxx:_mj_Euler, cxx:_mj_RungeKutta, cxx:_mj_checkAcc, cxx:_mj_checkPos, cxx:_mj_checkVel, cxx:_mj_compareFwdInv, cxx:_mj_forward, cxx:_mj_implicit, cxx:_mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_step(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx:_mj_step
}

/// C: mj_step1 (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_forward.h:38)
/// Calls: cxx:_mj_checkPos, cxx:_mj_checkVel, cxx:_mj_energyPos, cxx:_mj_energyVel, cxx:_mj_fwdPosition, cxx:_mj_fwdVelocity, cxx:_mj_sensorPos, cxx:_mj_sensorVel
#[allow(unused_variables, non_snake_case)]
pub fn mj_step1(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx:_mj_step1
}

/// C: mj_step2 (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_forward.h:41)
/// Calls: cxx:_mj_Euler, cxx:_mj_checkAcc, cxx:_mj_compareFwdInv, cxx:_mj_fwdAcceleration, cxx:_mj_fwdActuation, cxx:_mj_fwdConstraint, cxx:_mj_implicit, cxx:_mj_sensorAcc
#[allow(unused_variables, non_snake_case)]
pub fn mj_step2(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx:_mj_step2
}

/// C: mj_forward (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_forward.h:44)
/// Calls: cxx:_mj_forwardSkip
#[allow(unused_variables, non_snake_case)]
pub fn mj_forward(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx:_mj_forward
}

/// C: mj_forwardSkip (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_forward.h:47)
/// Calls: cxx:_mj_energyPos, cxx:_mj_energyVel, cxx:_mj_fwdAcceleration, cxx:_mj_fwdActuation, cxx:_mj_fwdConstraint, cxx:_mj_fwdPosition, cxx:_mj_fwdVelocity, cxx:_mj_sensorAcc, cxx:_mj_sensorPos, cxx:_mj_sensorVel
#[allow(unused_variables, non_snake_case)]
pub fn mj_forward_skip(m: *const mjModel, d: *mut mjData, skipstage: i32, skipsensor: i32) {
    todo!() // cxx:_mj_forwardSkip
}

/// C: mj_RungeKutta (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_forward.h:53)
/// Calls: cxx-internal:engine_forward.c.o:_mj_advance, cxx:_mj_forwardSkip, cxx:_mj_freeStack, cxx:_mj_integratePos, cxx:_mj_markStack, cxx:_mj_stackAllocInfo, cxx:_mju_addToScl, cxx:_mju_copy, cxx:_mju_message, cxx:_mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mj_runge_kutta(m: *const mjModel, d: *mut mjData, N: i32) {
    todo!() // cxx:_mj_RungeKutta
}

/// C: mj_Euler (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_forward.h:56)
/// Calls: cxx:_mj_EulerSkip
#[allow(unused_variables, non_snake_case)]
pub fn mj_euler(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx:_mj_Euler
}

/// C: mj_EulerSkip (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_forward.h:59)
/// Calls: cxx-internal:engine_forward.c.o:_mj_advance, cxx:_mj_actuatorDamping, cxx:_mj_factorI, cxx:_mj_freeStack, cxx:_mj_markStack, cxx:_mj_solveLD, cxx:_mj_stackAllocInfo, cxx:_mjd_xPolyForce, cxx:_mju_add, cxx:_mju_addInd, cxx:_mju_copy, cxx:_mju_copyInd, cxx:_mju_copySparse, cxx:_mju_isZero
#[allow(unused_variables, non_snake_case)]
pub fn mj_euler_skip(m: *const mjModel, d: *mut mjData, skipfactor: i32) {
    todo!() // cxx:_mj_EulerSkip
}

/// C: mj_implicit (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_forward.h:62)
/// Calls: cxx:_mj_implicitSkip
#[allow(unused_variables, non_snake_case)]
pub fn mj_implicit(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx:_mj_implicit
}

/// C: mj_implicitSkip (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_forward.h:65)
/// Calls: cxx-internal:engine_forward.c.o:_flexInterp_cgsolve, cxx-internal:engine_forward.c.o:_flex_has_implicit_stiffness, cxx-internal:engine_forward.c.o:_midpoint, cxx-internal:engine_forward.c.o:_midpoint_aligned, cxx-internal:engine_forward.c.o:_midpoint_eligible, cxx-internal:engine_forward.c.o:_mj_advance, cxx:_mj_factorI, cxx:_mj_freeStack, cxx:_mj_markStack, cxx:_mj_solveLD, cxx:_mj_stackAllocInfo, cxx:_mjd_smooth_vel, cxx:_mju_add, cxx:_mju_addInd, cxx:_mju_addScl, cxx:_mju_addToScl, cxx:_mju_copy, cxx:_mju_copyInd, cxx:_mju_factorLUSparse, cxx:_mju_gather, cxx:_mju_gatherMasked, cxx:_mju_message, cxx:_mju_solveLUSparse
#[allow(unused_variables, non_snake_case)]
pub fn mj_implicit_skip(m: *const mjModel, d: *mut mjData, skipfactor: i32) {
    todo!() // cxx:_mj_implicitSkip
}

/// C: mj_midpoint (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_forward.h:69)
/// Calls: cxx-internal:engine_forward.c.o:_midpointNewton, cxx:_mju_dot3, cxx:_mju_normalize3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_midpoint(mass: f64, inertia: *const f64, ipos: *const f64, iquat: *const f64, xquat: *const f64, qvel: *const f64, qfrc: *const f64, gravity: *const f64, h: f64, qvel_new: *mut f64) -> i32 {
    todo!() // cxx:_mj_midpoint
}

/// C: mj_fwdKinematics (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_forward.h:78)
/// Calls: cxx:_mj_camlight, cxx:_mj_comPos, cxx:_mj_flex, cxx:_mj_kinematics, cxx:_mj_tendon, cxx:_mj_updateSleep, cxx:_mj_wakeTendon
#[allow(unused_variables, non_snake_case)]
pub fn mj_fwd_kinematics(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx:_mj_fwdKinematics
}

/// C: mj_fwdPosition (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_forward.h:81)
/// Calls: cxx:_mj_collision, cxx:_mj_factorM, cxx:_mj_fwdKinematics, cxx:_mj_island, cxx:_mj_makeConstraint, cxx:_mj_makeM, cxx:_mj_projectConstraint, cxx:_mj_transmission, cxx:_mj_updateSleep, cxx:_mj_wakeCollision, cxx:_mj_wakeEquality
#[allow(unused_variables, non_snake_case)]
pub fn mj_fwd_position(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx:_mj_fwdPosition
}

/// C: mj_fwdVelocity (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_forward.h:84)
/// Calls: cxx:_mj_comVel, cxx:_mj_passive, cxx:_mj_referenceConstraint, cxx:_mj_rne, cxx:_mj_tendonBias, cxx:_mju_mulMatVecSparse, cxx:_mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mj_fwd_velocity(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx:_mj_fwdVelocity
}

/// C: mj_fwdActuation (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_forward.h:87)
/// Calls: cxx-internal:engine_forward.c.o:_clampVec, cxx-internal:engine_forward.c.o:_dcmotorVoltage, cxx:_mj_actuatorDisabled, cxx:_mj_dcmotorSlots, cxx:_mj_freeStack, cxx:_mj_lugreStribeck, cxx:_mj_markStack, cxx:_mj_nextActivation, cxx:_mj_readCtrl, cxx:_mj_sleepState, cxx:_mj_stackAllocInfo, cxx:_mj_warning, cxx:_mjp_getPluginAtSlotUnsafe, cxx:_mjp_pluginCount, cxx:_mju_addTo, cxx:_mju_clip, cxx:_mju_isBad, cxx:_mju_max, cxx:_mju_message, cxx:_mju_min, cxx:_mju_mulMatTVecSparse, cxx:_mju_muscleBias, cxx:_mju_muscleDynamics, cxx:_mju_muscleGain, cxx:_mju_norm3, cxx:_mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mj_fwd_actuation(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx:_mj_fwdActuation
}

/// C: mj_fwdAcceleration (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_forward.h:90)
/// Calls: cxx:_mj_solveLD, cxx:_mj_xfrcAccumulate, cxx:_mju_addTo, cxx:_mju_addToInd, cxx:_mju_copy, cxx:_mju_copyInd, cxx:_mju_sub, cxx:_mju_subInd
#[allow(unused_variables, non_snake_case)]
pub fn mj_fwd_acceleration(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx:_mj_fwdAcceleration
}

/// C: mj_fwdConstraint (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_forward.h:93)
/// Calls: cxx-internal:engine_forward.c.o:_warmstart, cxx:_mj_dualFinish, cxx:_mj_mulJacVec, cxx:_mj_solCG, cxx:_mj_solNewton, cxx:_mj_solNoSlip, cxx:_mj_solNoSlip_island, cxx:_mj_solPGS, cxx:_mju_copy, cxx:_mju_dispatch, cxx:_mju_gather, cxx:_mju_message, cxx:_mju_scatter, cxx:_mju_subFrom, cxx:_mju_zero, cxx:_mju_zeroInt
#[allow(unused_variables, non_snake_case)]
pub fn mj_fwd_constraint(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx:_mj_fwdConstraint
}

