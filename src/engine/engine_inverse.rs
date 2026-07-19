//! Port of: engine/engine_inverse.c
//! IR hash: adc2f24e872d94f7
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mj_discreteAcc (engine/engine_inverse.c:84)
/// Calls: mj_actuatorDamping, mj_freeStack, mj_markStack, mj_mulM, mj_solveM, mj_stackAllocInfo, mjd_smooth_vel, mjd_xPolyForce, mju_addScl, mju_addToScl, mju_copy, mju_gather, mju_gatherMasked, mju_isZero, mju_message, mju_mulMatVecSparse, mju_mulSymVecSparse
#[allow(unused_variables, non_snake_case)]
pub fn mj_discrete_acc(m: *const mjModel, d: *mut mjData) {
    todo!() // mj_discreteAcc
}

/// C: mj_inverse (engine/engine_inverse.h:27)
/// Calls: mj_inverseSkip
#[allow(unused_variables, non_snake_case)]
pub fn mj_inverse(m: *const mjModel, d: *mut mjData) {
    todo!() // mj_inverse
}

/// C: mj_inverseSkip (engine/engine_inverse.h:30)
/// Calls: mj_discreteAcc, mj_energyPos, mj_energyVel, mj_freeStack, mj_invConstraint, mj_invPosition, mj_invVelocity, mj_markStack, mj_mulM, mj_rne, mj_sensorAcc, mj_sensorPos, mj_sensorVel, mj_stackAllocInfo, mj_tendonBias, mju_copy
#[allow(unused_variables, non_snake_case)]
pub fn mj_inverse_skip(m: *const mjModel, d: *mut mjData, skipstage: i32, skipsensor: i32) {
    todo!() // mj_inverseSkip
}

/// C: mj_invPosition (engine/engine_inverse.h:34)
/// Calls: mj_camlight, mj_collision, mj_comPos, mj_factorM, mj_flex, mj_kinematics, mj_makeConstraint, mj_makeM, mj_projectConstraint, mj_tendon, mj_transmission
#[allow(unused_variables, non_snake_case)]
pub fn mj_inv_position(m: *const mjModel, d: *mut mjData) {
    todo!() // mj_invPosition
}

/// C: mj_invVelocity (engine/engine_inverse.h:37)
/// Calls: mj_fwdVelocity
#[allow(unused_variables, non_snake_case)]
pub fn mj_inv_velocity(m: *const mjModel, d: *mut mjData) {
    todo!() // mj_invVelocity
}

/// C: mj_invConstraint (engine/engine_inverse.h:40)
/// Calls: mj_constraintUpdate, mj_freeStack, mj_markStack, mj_mulJacVec, mj_stackAllocInfo, mju_subFrom, mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mj_inv_constraint(m: *const mjModel, d: *mut mjData) {
    todo!() // mj_invConstraint
}

/// C: mj_compareFwdInv (engine/engine_inverse.h:43)
/// Calls: mj_freeStack, mj_inverseSkip, mj_markStack, mj_stackAllocInfo, mj_xfrcAccumulate, mju_add, mju_copy, mju_norm, mju_sub
#[allow(unused_variables, non_snake_case)]
pub fn mj_compare_fwd_inv(m: *const mjModel, d: *mut mjData) {
    todo!() // mj_compareFwdInv
}

