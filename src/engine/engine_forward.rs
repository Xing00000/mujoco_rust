//! Port of: engine/engine_forward.c
//! IR hash: fd5288ad7242f001
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mj_checkPos (engine/engine_forward.c:55)
/// Calls: cxx:_mj_resetData, cxx:_mj_warning, cxx:_mju_isBad
#[allow(unused_variables, non_snake_case)]
pub fn mj_check_pos(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx:_mj_checkPos
}

/// C: mj_checkVel (engine/engine_forward.c:73)
/// Calls: cxx:_mj_resetData, cxx:_mj_warning, cxx:_mju_isBad
#[allow(unused_variables, non_snake_case)]
pub fn mj_check_vel(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx:_mj_checkVel
}

/// C: mj_checkAcc (engine/engine_forward.c:94)
/// Calls: cxx:_mj_forward, cxx:_mj_resetData, cxx:_mj_warning, cxx:_mju_isBad
#[allow(unused_variables, non_snake_case)]
pub fn mj_check_acc(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx:_mj_checkAcc
}

/// C: mj_fwdKinematics (engine/engine_forward.c:120)
/// Calls: cxx:_mj_camlight, cxx:_mj_comPos, cxx:_mj_flex, cxx:_mj_kinematics, cxx:_mj_tendon, cxx:_mj_updateSleep, cxx:_mj_wakeTendon
#[allow(unused_variables, non_snake_case)]
pub fn mj_fwd_kinematics(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx:_mj_fwdKinematics
}

/// C: mj_fwdPosition (engine/engine_forward.c:132)
/// Calls: cxx:_mj_collision, cxx:_mj_factorM, cxx:_mj_fwdKinematics, cxx:_mj_isMetric, cxx:_mj_island, cxx:_mj_makeConstraint, cxx:_mj_makeM, cxx:_mj_projectConstraint, cxx:_mj_transmission, cxx:_mj_updateSleep, cxx:_mj_wakeCollision, cxx:_mj_wakeEquality, cxx:_mjd_effBuild
#[allow(unused_variables, non_snake_case)]
pub fn mj_fwd_position(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx:_mj_fwdPosition
}

/// C: mj_fwdVelocity (engine/engine_forward.c:182)
/// Calls: cxx:_mj_comVel, cxx:_mj_isMetric, cxx:_mj_passive, cxx:_mj_referenceConstraint, cxx:_mj_rne, cxx:_mj_tendonBias, cxx:_mj_velocityConstraint, cxx:_mjd_effShift, cxx:_mju_mulMatVecSparse, cxx:_mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mj_fwd_velocity(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx:_mj_fwdVelocity
}

/// C: unpackServoInputs (engine/engine_forward.c:240)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn unpack_servo_inputs(u: *const f64, spec: i32, out: *mut f64) {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_forward.c:_unpackServoInputs
}

/// C: dcmotorVoltage (engine/engine_forward.c:250)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_forward.c:_unpackServoInputs, cxx:_mju_clip
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn dcmotor_voltage(u: *const f64, spec: i32, length: f64, velocity: f64, x_I: f64, gainprm: *const f64) -> f64 {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_forward.c:_dcmotorVoltage
}

/// C: clampVec (engine/engine_forward.c:282)
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

/// C: expmap2Quat (engine/engine_forward.c:294)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_axisAngle2Quat, cxx:_mju_norm3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn expmap2quat(quat: *mut f64, v: *const f64) {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_forward.c:_expmap2Quat
}

/// C: wrapPeriod (engine/engine_forward.c:307)
/// Calls: cxx:_mju_norm3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn wrap_period(m: *const mjModel, i: i32) -> f64 {
    todo!() // cxx-internal:engine_forward.c.o:_wrapPeriod
}

/// C: wrapSetpoint (engine/engine_forward.c:342)
/// Calls: cxx:_mju_round
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn wrap_setpoint(u: f64, length: f64, period: f64) -> f64 {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_forward.c:_wrapSetpoint
}

/// C: slewLimit (engine/engine_forward.c:350)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_forward.c:_wrapSetpoint, cxx:_mju_clip
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn slew_limit(u: f64, u_prev: f64, slew_s: f64, dt: f64, period: f64, act_dot: *mut f64) -> f64 {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_forward.c:_slewLimit
}

/// C: mj_fwdActuation (engine/engine_forward.c:363)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_forward.c:_clampVec, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_forward.c:_dcmotorVoltage, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_forward.c:_expmap2Quat, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_forward.c:_slewLimit, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_forward.c:_unpackServoInputs, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_forward.c:_wrapSetpoint, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_subQuat, cxx-internal:engine_forward.c.o:_wrapPeriod, cxx:_mj_actuatorDisabled, cxx:_mj_dcmotorSlots, cxx:_mj_freeStack, cxx:_mj_lugreStribeck, cxx:_mj_markStack, cxx:_mj_nextActivation, cxx:_mj_readCtrl, cxx:_mj_sleepState, cxx:_mj_stackAllocInfo, cxx:_mj_warning, cxx:_mjp_getPluginAtSlotUnsafe, cxx:_mjp_pluginCount, cxx:_mju_addTo, cxx:_mju_clip, cxx:_mju_copy, cxx:_mju_copy3, cxx:_mju_copy4, cxx:_mju_isBad, cxx:_mju_max, cxx:_mju_message, cxx:_mju_min, cxx:_mju_mulMatTVecSparse, cxx:_mju_muscleBias, cxx:_mju_muscleDynamics, cxx:_mju_muscleGain, cxx:_mju_norm3, cxx:_mju_normalize4, cxx:_mju_round, cxx:_mju_scl3, cxx:_mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mj_fwd_actuation(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx:_mj_fwdActuation
}

/// C: mj_fwdAcceleration (engine/engine_forward.c:1017)
/// Calls: cxx:_mj_freeStack, cxx:_mj_markStack, cxx:_mj_solveLD, cxx:_mj_stackAllocInfo, cxx:_mj_xfrcAccumulate, cxx:_mjd_effSolve, cxx:_mju_add, cxx:_mju_addInd, cxx:_mju_addTo, cxx:_mju_addToInd, cxx:_mju_copy, cxx:_mju_copyInd, cxx:_mju_sub, cxx:_mju_subInd, cxx:_mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mj_fwd_acceleration(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx:_mj_fwdAcceleration
}

/// C: warmstart (engine/engine_forward.c:1083)
/// Calls: cxx:_mj_constraintUpdate, cxx:_mj_freeStack, cxx:_mj_isSparse, cxx:_mj_markStack, cxx:_mj_mulJacVec, cxx:_mj_mulM, cxx:_mj_stackAllocInfo, cxx:_mjd_effMulAdd, cxx:_mju_copy, cxx:_mju_dot, cxx:_mju_mulMatVec, cxx:_mju_mulMatVecSparse, cxx:_mju_sub, cxx:_mju_subFrom, cxx:_mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn warmstart(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_forward.c:_warmstart
}

/// C: solveIslandTask (engine/engine_forward.c:1166)
/// Calls: cxx:_mj_solCG_island, cxx:_mj_solNewton_island, cxx:_mj_solPGS_island
#[allow(unused_variables, non_snake_case)]
pub fn solve_island_task(m: *const mjModel, d: *mut mjData, arg: *mut (), thread_id: i32, island: i32) {
    todo!() // cxx-internal:engine_forward.c.o:_solveIslandTask
}

/// C: effFlexAny (engine/engine_forward.c:1178)
/// Calls: cxx:_mj_effFlexPossible
#[allow(unused_variables, non_snake_case)]
pub fn eff_flex_any(m: *const mjModel) -> i32 {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_forward.c:_effFlexAny
}

/// C: mj_discreteGyro (engine/engine_forward.c:1192)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_copy6, cxx:_mjd_freeGyroPossible, cxx:_mjd_freeMhat, cxx:_mju_add, cxx:_mju_addTo, cxx:_mju_factorLU6, cxx:_mju_solveLU6
#[allow(unused_variables, non_snake_case)]
pub fn mj_discrete_gyro(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx-internal:engine_forward.c.o:_mj_discreteGyro
}

/// C: mj_fwdConstraint (engine/engine_forward.c:1223)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_forward.c:_effFlexAny, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_forward.c:_warmstart, cxx-internal:engine_forward.c.o:_mj_discreteGyro, cxx:_mj_dualFinish, cxx:_mj_isMetric, cxx:_mj_mulJacVec, cxx:_mj_solCG, cxx:_mj_solNewton, cxx:_mj_solNoSlip, cxx:_mj_solNoSlip_island, cxx:_mj_solPGS, cxx:_mju_copy, cxx:_mju_dispatch, cxx:_mju_gather, cxx:_mju_message, cxx:_mju_scatter, cxx:_mju_subFrom, cxx:_mju_zero, cxx:_mju_zeroInt
#[allow(unused_variables, non_snake_case)]
pub fn mj_fwd_constraint(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx:_mj_fwdConstraint
}

/// C: mj_advance (engine/engine_forward.c:1336)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_forward.c:_wrapSetpoint, cxx-internal:engine_forward.c.o:_wrapPeriod, cxx:_mj_actuatorDisabled, cxx:_mj_computeSensor, cxx:_mj_forwardSkip, cxx:_mj_integratePosInd, cxx:_mj_nextActivation, cxx:_mj_sleep, cxx:_mj_updateSleep, cxx:_mjp_getPluginAtSlotUnsafe, cxx:_mjp_pluginCount, cxx:_mju_addToScl, cxx:_mju_addToSclInd, cxx:_mju_copy, cxx:_mju_historyInsert, cxx:_mju_message, cxx:_mju_norm3, cxx:_mju_round
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_advance(m: *const mjModel, d: *mut mjData, act_dot: *const f64, qacc: *const f64, qvel: *const f64) {
    todo!() // cxx-internal:engine_forward.c.o:_mj_advance
}

/// C: mj_EulerSkip (engine/engine_forward.c:1473)
/// Calls: cxx-internal:engine_forward.c.o:_mj_advance, cxx:_mj_actuatorDamping, cxx:_mj_factorI, cxx:_mj_freeStack, cxx:_mj_markStack, cxx:_mj_solveLD, cxx:_mj_stackAllocInfo, cxx:_mjd_xPolyForce, cxx:_mju_add, cxx:_mju_addInd, cxx:_mju_copy, cxx:_mju_copyInd, cxx:_mju_copySparse, cxx:_mju_isZero
#[allow(unused_variables, non_snake_case)]
pub fn mj_euler_skip(m: *const mjModel, d: *mut mjData, skipfactor: i32) {
    todo!() // cxx:_mj_EulerSkip
}

/// C: mj_Euler (engine/engine_forward.c:1555)
/// Calls: cxx:_mj_EulerSkip
#[allow(unused_variables, non_snake_case)]
pub fn mj_euler(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx:_mj_Euler
}

/// C: mj_discrete (engine/engine_forward.c:1561)
/// Calls: cxx-internal:engine_forward.c.o:_mj_advance
#[allow(unused_variables, non_snake_case)]
pub fn mj_discrete(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx:_mj_discrete
}

/// C: mj_RungeKutta (engine/engine_forward.c:1582)
/// Calls: cxx-internal:engine_forward.c.o:_mj_advance, cxx:_mj_forwardSkip, cxx:_mj_freeStack, cxx:_mj_integratePos, cxx:_mj_markStack, cxx:_mj_stackAllocInfo, cxx:_mju_addToScl, cxx:_mju_copy, cxx:_mju_message, cxx:_mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mj_runge_kutta(m: *const mjModel, d: *mut mjData, N: i32) {
    todo!() // cxx:_mj_RungeKutta
}

/// C: mj_checkDiscrete (engine/engine_forward.c:1674)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_forward.c:_effFlexAny, cxx:_mj_effFlexContactPossible, cxx:_mj_effFlexStiffPossible, cxx:_mj_isMetric, cxx:_mjd_flexInterpAssemblable, cxx:_mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_check_discrete(m: *const mjModel) {
    todo!() // cxx:_mj_checkDiscrete
}

/// C: mj_implicitSkip (engine/engine_forward.c:1726)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_copy6, cxx-internal:engine_forward.c.o:_mj_advance, cxx:_mj_factorI, cxx:_mj_freeStack, cxx:_mj_isFreeBody, cxx:_mj_markStack, cxx:_mj_solveLD, cxx:_mj_stackAllocInfo, cxx:_mjd_freeMhat, cxx:_mjd_smooth_vel, cxx:_mju_add, cxx:_mju_addInd, cxx:_mju_addScl, cxx:_mju_addToScl, cxx:_mju_copy, cxx:_mju_copyInd, cxx:_mju_factorLU6, cxx:_mju_factorLUSparse, cxx:_mju_gather, cxx:_mju_gatherMasked, cxx:_mju_message, cxx:_mju_solveLU6, cxx:_mju_solveLUSparse
#[allow(unused_variables, non_snake_case)]
pub fn mj_implicit_skip(m: *const mjModel, d: *mut mjData, skipfactor: i32) {
    todo!() // cxx:_mj_implicitSkip
}

/// C: mj_implicit (engine/engine_forward.c:1841)
/// Calls: cxx:_mj_implicitSkip
#[allow(unused_variables, non_snake_case)]
pub fn mj_implicit(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx:_mj_implicit
}

/// C: mj_forwardSkip (engine/engine_forward.c:1849)
/// Calls: cxx:_mj_checkDiscrete, cxx:_mj_energyPos, cxx:_mj_energyVel, cxx:_mj_fwdAcceleration, cxx:_mj_fwdActuation, cxx:_mj_fwdConstraint, cxx:_mj_fwdPosition, cxx:_mj_fwdVelocity, cxx:_mj_isMetric, cxx:_mj_referenceConstraint, cxx:_mj_regularizeConstraint, cxx:_mj_sensorAcc, cxx:_mj_sensorPos, cxx:_mj_sensorVel, cxx:_mjd_effActuation
#[allow(unused_variables, non_snake_case)]
pub fn mj_forward_skip(m: *const mjModel, d: *mut mjData, skipstage: i32, skipsensor: i32) {
    todo!() // cxx:_mj_forwardSkip
}

/// C: mj_forward (engine/engine_forward.c:1908)
/// Calls: cxx:_mj_forwardSkip
#[allow(unused_variables, non_snake_case)]
pub fn mj_forward(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx:_mj_forward
}

/// C: mj_step (engine/engine_forward.c:1914)
/// Calls: cxx:_mj_Euler, cxx:_mj_RungeKutta, cxx:_mj_checkAcc, cxx:_mj_checkPos, cxx:_mj_checkVel, cxx:_mj_compareFwdInv, cxx:_mj_discrete, cxx:_mj_forward, cxx:_mj_implicit, cxx:_mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_step(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx:_mj_step
}

/// C: mj_step1 (engine/engine_forward.c:1956)
/// Calls: cxx:_mj_checkDiscrete, cxx:_mj_checkPos, cxx:_mj_checkVel, cxx:_mj_energyPos, cxx:_mj_energyVel, cxx:_mj_fwdPosition, cxx:_mj_fwdVelocity, cxx:_mj_sensorPos, cxx:_mj_sensorVel
#[allow(unused_variables, non_snake_case)]
pub fn mj_step1(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx:_mj_step1
}

/// C: mj_step2 (engine/engine_forward.c:1989)
/// Calls: cxx:_mj_Euler, cxx:_mj_checkAcc, cxx:_mj_compareFwdInv, cxx:_mj_discrete, cxx:_mj_fwdAcceleration, cxx:_mj_fwdActuation, cxx:_mj_fwdConstraint, cxx:_mj_implicit, cxx:_mj_isMetric, cxx:_mj_referenceConstraint, cxx:_mj_regularizeConstraint, cxx:_mj_sensorAcc, cxx:_mjd_effActuation
#[allow(unused_variables, non_snake_case)]
pub fn mj_step2(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx:_mj_step2
}

