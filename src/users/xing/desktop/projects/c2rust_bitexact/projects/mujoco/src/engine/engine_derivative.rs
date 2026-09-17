//! Port of: Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative.c
//! IR hash: 1fa44043fe383ce9
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mjd_cross (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative.c:38)
/// Calls: cxx:_mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjd_cross(a: *const f64, b: *const f64, Da: *mut f64, Db: *mut f64) {
    todo!() // cxx-internal:engine_derivative.c.o:_mjd_cross
}

/// C: mjd_crossMotion_vel (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative.c:65)
/// Calls: cxx:_mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjd_cross_motion_vel(D: *mut f64, v: *const f64) {
    todo!() // cxx-internal:engine_derivative.c.o:_mjd_crossMotion_vel
}

/// C: mjd_crossForce_vel (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative.c:101)
/// Calls: cxx:_mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjd_cross_force_vel(D: *mut f64, f: *const f64) {
    todo!() // cxx-internal:engine_derivative.c.o:_mjd_crossForce_vel
}

/// C: mjd_crossForce_frc (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative.c:137)
/// Calls: cxx:_mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjd_cross_force_frc(D: *mut f64, vel: *const f64) {
    todo!() // cxx-internal:engine_derivative.c.o:_mjd_crossForce_frc
}

/// C: mjd_mulInertVec_vel (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative.c:173)
/// Calls: cxx:_mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjd_mul_inert_vec_vel(D: *mut f64, i: *const f64) {
    todo!() // cxx-internal:engine_derivative.c.o:_mjd_mulInertVec_vel
}

/// C: mjd_comVel_vel_dense (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative.c:321)
/// Calls: cxx-internal:engine_derivative.c.o:_mjd_crossMotion_vel, cxx:_mju_copy, cxx:_mju_mulMatMat, cxx:_mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjd_com_vel_vel_dense(m: *const mjModel, d: *mut mjData, Dcvel: *mut f64, Dcdofdot: *mut f64) {
    todo!() // cxx-internal:engine_derivative.c.o:_mjd_comVel_vel_dense
}

/// C: copyFromParent (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative.c:468)
/// Calls: cxx:_mju_copy
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn copy_from_parent(m: *const mjModel, d: *mut mjData, mat: *mut f64, n: i32) {
    todo!() // cxx-internal:engine_derivative.c.o:_copyFromParent
}

/// C: addToParent (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative.c:491)
/// Calls: cxx:_mju_addTo, cxx:_mju_message
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn add_to_parent(m: *const mjModel, d: *mut mjData, mat: *mut f64, n: i32) {
    todo!() // cxx-internal:engine_derivative.c.o:_addToParent
}

/// C: mjd_comVel_vel (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative.c:524)
/// Calls: cxx-internal:engine_derivative.c.o:_copyFromParent, cxx-internal:engine_derivative.c.o:_mjd_crossMotion_vel, cxx:_mju_addTo, cxx:_mju_message, cxx:_mju_mulMatMat, cxx:_mju_transpose
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjd_com_vel_vel(m: *const mjModel, d: *mut mjData, Dcvel: *mut f64, Dcdofdot: *mut f64) {
    todo!() // cxx-internal:engine_derivative.c.o:_mjd_comVel_vel
}

/// C: mjd_rne_vel (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative.c:596)
/// Calls: cxx-internal:engine_derivative.c.o:_addToParent, cxx-internal:engine_derivative.c.o:_copyFromParent, cxx-internal:engine_derivative.c.o:_mjd_comVel_vel, cxx-internal:engine_derivative.c.o:_mjd_crossForce_frc, cxx-internal:engine_derivative.c.o:_mjd_crossForce_vel, cxx-internal:engine_derivative.c.o:_mjd_mulInertVec_vel, cxx:_mj_freeStack, cxx:_mj_markStack, cxx:_mj_stackAllocInfo, cxx:_mju_addTo, cxx:_mju_addToScl, cxx:_mju_mulInertVec, cxx:_mju_mulMatMat, cxx:_mju_mulMatVec, cxx:_mju_subFrom, cxx:_mju_transpose, cxx:_mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mjd_rne_vel(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx-internal:engine_derivative.c.o:_mjd_rne_vel
}

/// C: addJTBJ (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative.c:711)
/// Calls: cxx:_mj_freeStack, cxx:_mj_markStack, cxx:_mj_stackAllocInfo, cxx:_mju_scl
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn add_jtbj(m: *const mjModel, d: *mut mjData, J: *const f64, B: *const f64, n: i32) {
    todo!() // cxx-internal:engine_derivative.c.o:_addJTBJ
}

/// C: addJTBJSparse (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative.c:746)
/// Calls: cxx:_mju_addToSclSparseInc
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn add_jtbj_sparse(m: *const mjModel, d: *mut mjData, J: *const f64, B: *const f64, n: i32, offset: i32, J_rownnz: *const i32, J_rowadr: *const i32, J_colind: *const i32) {
    todo!() // cxx-internal:engine_derivative.c.o:_addJTBJSparse
}

/// C: mjd_muscleGain_vel (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative.c:781)
/// Calls: cxx:_mju_max, cxx:_mju_muscleGainLength
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjd_muscle_gain_vel(len: f64, vel: f64, lengthrange: *const f64, acc0: f64, prm: *const f64) -> f64 {
    todo!() // cxx-internal:engine_derivative.c.o:_mjd_muscleGain_vel
}

/// C: addJTBJ_mulSparse (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative.c:832)
/// Calls: cxx:_mj_freeStack, cxx:_mj_markStack, cxx:_mj_stackAllocInfo, cxx:_mju_mulMatVec, cxx:_mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn add_jtbj_mul_sparse(m: *const mjModel, d: *mut mjData, res: *mut f64, vec: *const f64, J_rownnz: *const i32, J_rowadr: *const i32, J_colind: *const i32, J: *const f64, B: *const f64, n: i32) {
    todo!() // cxx-internal:engine_derivative.c.o:_addJTBJ_mulSparse
}

/// C: mjd_flexInterp_kernel (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative.c:872)
/// Calls: cxx-internal:engine_derivative.c.o:_addJTBJ_mulSparse, cxx:_mj_bodyChain, cxx:_mj_freeStack, cxx:_mj_jacSparse, cxx:_mj_markStack, cxx:_mj_stackAllocInfo, cxx:_mju_flexGatherCellState, cxx:_mju_flexGatherFaceState, cxx:_mju_flexGatherState, cxx:_mju_mulMatMat3, cxx:_mju_quat2Mat, cxx:_mju_transpose, cxx:_mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjd_flex_interp_kernel(m: *const mjModel, d: *mut mjData, res: *mut f64, vec: *const f64, s1: f64, s2: f64, K_rot_cache: *const f64, K_rot_out: *mut f64) {
    todo!() // cxx-internal:engine_derivative.c.o:_mjd_flexInterp_kernel
}

/// C: pow2 (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative.c:1339)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn pow2(val: f64) -> f64 {
    todo!() // cxx-internal:engine_derivative.c.o:_pow2
}

/// C: ellipsoid_max_moment (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative.c:1344)
/// Calls: cxx-internal:engine_derivative.c.o:_pow2, cxx:_mju_max
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn ellipsoid_max_moment(size: *const f64, dir: i32) -> f64 {
    todo!() // cxx-internal:engine_derivative.c.o:_ellipsoid_max_moment
}

/// C: addToQuadrant (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative.c:1354)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn add_to_quadrant(B: *mut f64, D: *const f64, col_quad: i32, row_quad: i32) {
    todo!() // cxx-internal:engine_derivative.c.o:_addToQuadrant
}

/// C: mjd_addedMassForces (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative.c:1371)
/// Calls: cxx-internal:engine_derivative.c.o:_addToQuadrant, cxx-internal:engine_derivative.c.o:_mjd_cross
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjd_added_mass_forces(B: *mut f64, local_vels: *const f64, fluid_density: f64, virtual_mass: *const f64, virtual_inertia: *const f64) {
    todo!() // cxx-internal:engine_derivative.c.o:_mjd_addedMassForces
}

/// C: mjd_viscous_torque (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative.c:1416)
/// Calls: cxx-internal:engine_derivative.c.o:_ellipsoid_max_moment, cxx:_mju_addToScl3, cxx:_mju_max, cxx:_mju_min, cxx:_mju_norm3, cxx:_mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjd_viscous_torque(D: *mut f64, lvel: *const f64, fluid_density: f64, fluid_viscosity: f64, size: *const f64, slender_drag_coef: f64, ang_drag_coef: f64) {
    todo!() // cxx-internal:engine_derivative.c.o:_mjd_viscous_torque
}

/// C: mjd_viscous_drag (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative.c:1469)
/// Calls: cxx-internal:engine_derivative.c.o:_pow2, cxx:_mju_addToScl3, cxx:_mju_max, cxx:_mju_min, cxx:_mju_scl
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjd_viscous_drag(D: *mut f64, lvel: *const f64, fluid_density: f64, fluid_viscosity: f64, size: *const f64, blunt_drag_coef: f64, slender_drag_coef: f64) {
    todo!() // cxx-internal:engine_derivative.c.o:_mjd_viscous_drag
}

/// C: mjd_kutta_lift (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative.c:1536)
/// Calls: cxx-internal:engine_derivative.c.o:_pow2, cxx:_mju_addToScl3, cxx:_mju_max, cxx:_mju_scl
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjd_kutta_lift(D: *mut f64, lvel: *const f64, fluid_density: f64, size: *const f64, kutta_lift_coef: f64) {
    todo!() // cxx-internal:engine_derivative.c.o:_mjd_kutta_lift
}

/// C: mjd_magnus_force (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative.c:1589)
/// Calls: cxx-internal:engine_derivative.c.o:_addToQuadrant, cxx-internal:engine_derivative.c.o:_mjd_cross
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjd_magnus_force(B: *mut f64, lvel: *const f64, fluid_density: f64, size: *const f64, magnus_lift_coef: f64) {
    todo!() // cxx-internal:engine_derivative.c.o:_mjd_magnus_force
}

/// C: mjd_ellipsoidFluid (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative.c:1618)
/// Calls: cxx-internal:engine_derivative.c.o:_addJTBJ, cxx-internal:engine_derivative.c.o:_addJTBJSparse, cxx-internal:engine_derivative.c.o:_addToQuadrant, cxx-internal:engine_derivative.c.o:_mjd_addedMassForces, cxx-internal:engine_derivative.c.o:_mjd_kutta_lift, cxx-internal:engine_derivative.c.o:_mjd_magnus_force, cxx-internal:engine_derivative.c.o:_mjd_viscous_drag, cxx-internal:engine_derivative.c.o:_mjd_viscous_torque, cxx:_mj_bodyChain, cxx:_mj_freeStack, cxx:_mj_isSparse, cxx:_mj_jacGeom, cxx:_mj_jacSparse, cxx:_mj_markStack, cxx:_mj_objectVelocity, cxx:_mj_stackAllocInfo, cxx:_mju_copy, cxx:_mju_copy3, cxx:_mju_geomSemiAxes, cxx:_mju_mulMatTMat, cxx:_mju_subFrom3, cxx:_mju_symmetrize, cxx:_mju_transformSpatial, cxx:_mju_zero, cxx:_readFluidGeomInteraction
#[allow(unused_variables, non_snake_case)]
pub fn mjd_ellipsoid_fluid(m: *const mjModel, d: *mut mjData, bodyid: i32) {
    todo!() // cxx:_mjd_ellipsoidFluid
}

/// C: mjd_inertiaBoxFluid (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative.c:1724)
/// Calls: cxx-internal:engine_derivative.c.o:_addJTBJ, cxx-internal:engine_derivative.c.o:_addJTBJSparse, cxx:_mj_bodyChain, cxx:_mj_freeStack, cxx:_mj_isSparse, cxx:_mj_jacBodyCom, cxx:_mj_jacSparse, cxx:_mj_markStack, cxx:_mj_objectVelocity, cxx:_mj_stackAllocInfo, cxx:_mju_copy, cxx:_mju_copy3, cxx:_mju_max, cxx:_mju_mulMatTMat, cxx:_mju_subFrom3, cxx:_mju_transformSpatial, cxx:_mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mjd_inertia_box_fluid(m: *const mjModel, d: *mut mjData, i: i32) {
    todo!() // cxx:_mjd_inertiaBoxFluid
}

/// C: mjd_subQuat (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative.h:27)
/// Calls: cxx:_mju_addToScl, cxx:_mju_copy9, cxx:_mju_mulMatMat3, cxx:_mju_normalize3, cxx:_mju_scl, cxx:_mju_subQuat, cxx:_mju_transpose
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjd_sub_quat(qa: *const f64, qb: *const f64, Da: *mut f64, Db: *mut f64) {
    todo!() // cxx:_mjd_subQuat
}

/// C: mjd_quatIntegrate (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative.h:30)
/// Calls: cxx:_mju_copy9, cxx:_mju_dot3, cxx:_mju_mulMatVec3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjd_quat_integrate(vel: *const f64, scale: f64, Dquat: *mut f64, Dvel: *mut f64, Dscale: *mut f64) {
    todo!() // cxx:_mjd_quatIntegrate
}

/// C: mjd_smooth_vel (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative.h:35)
/// Calls: cxx-internal:engine_derivative.c.o:_mjd_rne_vel, cxx:_mjd_actuator_vel, cxx:_mjd_passive_vel, cxx:_mju_zero, cxx:_mju_zeroSparse
#[allow(unused_variables, non_snake_case)]
pub fn mjd_smooth_vel(m: *const mjModel, d: *mut mjData, flg_bias: i32) {
    todo!() // cxx:_mjd_smooth_vel
}

/// C: mjd_actuator_vel (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative.h:38)
/// Calls: cxx-internal:engine_derivative.c.o:_addJTBJSparse, cxx-internal:engine_derivative.c.o:_mjd_muscleGain_vel, cxx:_mj_actuatorDisabled, cxx:_mj_nextActivation, cxx:_mj_sleepState, cxx:_mju_max
#[allow(unused_variables, non_snake_case)]
pub fn mjd_actuator_vel(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx:_mjd_actuator_vel
}

/// C: mjd_passive_vel (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative.h:41)
/// Calls: cxx-internal:engine_derivative.c.o:_addJTBJSparse, cxx:_mj_actuatorDamping, cxx:_mjd_ellipsoidFluid, cxx:_mjd_inertiaBoxFluid, cxx:_mjd_xPolyForce, cxx:_mju_copy
#[allow(unused_variables, non_snake_case)]
pub fn mjd_passive_vel(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx:_mjd_passive_vel
}

/// C: mjd_rne_vel_dense (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative.h:44)
/// Calls: cxx-internal:engine_derivative.c.o:_mjd_comVel_vel_dense, cxx-internal:engine_derivative.c.o:_mjd_crossForce_frc, cxx-internal:engine_derivative.c.o:_mjd_crossForce_vel, cxx-internal:engine_derivative.c.o:_mjd_mulInertVec_vel, cxx:_mj_freeStack, cxx:_mj_markStack, cxx:_mj_stackAllocInfo, cxx:_mju_addTo, cxx:_mju_addToScl, cxx:_mju_copy, cxx:_mju_mulInertVec, cxx:_mju_mulMatMat, cxx:_mju_scl, cxx:_mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mjd_rne_vel_dense(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx:_mjd_rne_vel_dense
}

/// C: mjd_flexInterp_mul (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative.h:48)
/// Calls: cxx-internal:engine_derivative.c.o:_mjd_flexInterp_kernel
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjd_flex_interp_mul(m: *const mjModel, d: *mut mjData, res: *mut f64, vec: *const f64, s1: f64, s2: f64, K_rot_cache: *const f64) {
    todo!() // cxx:_mjd_flexInterp_mul
}

/// C: mjd_flexInterp_cacheKrot (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative.h:52)
/// Calls: cxx-internal:engine_derivative.c.o:_mjd_flexInterp_kernel
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjd_flex_interp_cache_krot(m: *const mjModel, d: *mut mjData, K_rot_out: *mut f64) {
    todo!() // cxx:_mjd_flexInterp_cacheKrot
}

/// C: mjd_flexBend_mul (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative.h:56)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjd_flex_bend_mul(m: *const mjModel, d: *mut mjData, res: *mut f64, vec: *const f64, s1: f64, s2: f64) {
    todo!() // cxx:_mjd_flexBend_mul
}

