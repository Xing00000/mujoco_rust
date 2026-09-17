//! Port of: engine/engine_inverse.c
//! IR hash: fd5288ad7242f001
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mj_invPosition (engine/engine_inverse.c:39)
/// Calls: cxx:_mj_camlight, cxx:_mj_collision, cxx:_mj_comPos, cxx:_mj_factorM, cxx:_mj_flex, cxx:_mj_isMetric, cxx:_mj_kinematics, cxx:_mj_makeConstraint, cxx:_mj_makeM, cxx:_mj_projectConstraint, cxx:_mj_tendon, cxx:_mj_transmission, cxx:_mjd_effBuild
#[allow(unused_variables, non_snake_case)]
pub fn mj_inv_position(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx:_mj_invPosition
}

/// C: mj_invVelocity (engine/engine_inverse.c:81)
/// Calls: cxx:_mj_fwdVelocity
#[allow(unused_variables, non_snake_case)]
pub fn mj_inv_velocity(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx:_mj_invVelocity
}

/// C: mj_discreteAcc (engine/engine_inverse.c:87)
/// Calls: cxx:_mj_actuatorDamping, cxx:_mj_freeStack, cxx:_mj_markStack, cxx:_mj_mulM, cxx:_mj_solveM, cxx:_mj_stackAllocInfo, cxx:_mjd_effShift, cxx:_mjd_freeMhat, cxx:_mjd_smooth_vel, cxx:_mjd_xPolyForce, cxx:_mju_addScl, cxx:_mju_addToScl, cxx:_mju_copy, cxx:_mju_gather, cxx:_mju_gatherMasked, cxx:_mju_isZero, cxx:_mju_message, cxx:_mju_mulMatVec, cxx:_mju_mulMatVecSparse, cxx:_mju_mulSymVecSparse
#[allow(unused_variables, non_snake_case)]
pub fn mj_discrete_acc(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx-internal:engine_inverse.c.o:_mj_discreteAcc
}

/// C: mj_invConstraint (engine/engine_inverse.c:196)
/// Calls: cxx:_mj_constraintUpdate, cxx:_mj_freeStack, cxx:_mj_markStack, cxx:_mj_mulJacVec, cxx:_mj_stackAllocInfo, cxx:_mju_subFrom, cxx:_mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mj_inv_constraint(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx:_mj_invConstraint
}

/// C: mj_inverseSkip (engine/engine_inverse.c:224)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inverse.c:_mj_discreteAcc, cxx:_mj_checkDiscrete, cxx:_mj_energyPos, cxx:_mj_energyVel, cxx:_mj_freeStack, cxx:_mj_invConstraint, cxx:_mj_invPosition, cxx:_mj_invVelocity, cxx:_mj_isMetric, cxx:_mj_markStack, cxx:_mj_mulM, cxx:_mj_referenceConstraint, cxx:_mj_regularizeConstraint, cxx:_mj_rne, cxx:_mj_sensorAcc, cxx:_mj_sensorPos, cxx:_mj_sensorVel, cxx:_mj_stackAllocInfo, cxx:_mj_tendonBias, cxx:_mjd_effActuation, cxx:_mjd_effMulAdd, cxx:_mjd_freeGyroPossible, cxx:_mjd_freeMhat, cxx:_mju_copy, cxx:_mju_mulMatVec, cxx:_mju_subFrom
#[allow(unused_variables, non_snake_case)]
pub fn mj_inverse_skip(m: *const mjModel, d: *mut mjData, skipstage: i32, skipsensor: i32) {
    todo!() // cxx:_mj_inverseSkip
}

/// C: mj_inverse (engine/engine_inverse.c:327)
/// Calls: cxx:_mj_inverseSkip
#[allow(unused_variables, non_snake_case)]
pub fn mj_inverse(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx:_mj_inverse
}

/// C: mj_compareFwdInv (engine/engine_inverse.c:335)
/// Calls: cxx:_mj_freeStack, cxx:_mj_inverseSkip, cxx:_mj_markStack, cxx:_mj_stackAllocInfo, cxx:_mj_xfrcAccumulate, cxx:_mju_add, cxx:_mju_copy, cxx:_mju_norm, cxx:_mju_sub
#[allow(unused_variables, non_snake_case)]
pub fn mj_compare_fwd_inv(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx:_mj_compareFwdInv
}

