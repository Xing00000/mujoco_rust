//! Port of: Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_passive.c
//! IR hash: 1fa44043fe383ce9
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: GradSquaredLengths (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_passive.c:48)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn grad_squared_lengths(gradient: *mut [[f64; 3]; 2], xpos: *const f64, vert: *const i32, edge: *const [i32; 2], nedge: i32) {
    todo!() // cxx-internal:engine_passive.c.o:_GradSquaredLengths
}

/// C: mj_flexPassiveInterp (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_passive.c:63)
/// Calls: cxx:_mj_applyFT, cxx:_mj_freeStack, cxx:_mj_markStack, cxx:_mj_stackAllocInfo, cxx:_mju_flexGatherCellState, cxx:_mju_flexGatherFaceState, cxx:_mju_flexGatherState, cxx:_mju_mulMatVec, cxx:_mju_negQuat, cxx:_mju_rotVecQuat, cxx:_mju_scl3, cxx:_mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mj_flex_passive_interp(m: *const mjModel, d: *mut mjData, f: i32, enbl_spring: i32, enbl_damper: i32) {
    todo!() // cxx-internal:engine_passive.c.o:_mj_flexPassiveInterp
}

/// C: mju_dphi2D (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_passive.c:211)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_dphi2d(s0: f64, l0: i32, s1: f64, l1: i32, order: i32, dir: i32) -> f64 {
    todo!() // cxx-internal:engine_passive.c.o:_mju_dphi2D
}

/// C: mj_flexPassiveBendInterp (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_passive.c:236)
/// Calls: cxx-internal:engine_passive.c.o:_mju_dphi2D, cxx:_mj_applyFT, cxx:_mj_freeStack, cxx:_mj_markStack, cxx:_mj_stackAllocInfo, cxx:_mju_add, cxx:_mju_copy, cxx:_mju_copyInt, cxx:_mju_dot, cxx:_mju_dot3, cxx:_mju_flexFaceNormal2D, cxx:_mju_flexGatherFaceState, cxx:_mju_flexGatherState, cxx:_mju_message, cxx:_mju_negQuat, cxx:_mju_norm3, cxx:_mju_normalize, cxx:_mju_rotVecQuat, cxx:_mju_scl, cxx:_mju_warning, cxx:_mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mj_flex_passive_bend_interp(m: *const mjModel, d: *mut mjData, f: i32, enbl_spring: i32, enbl_damper: i32) {
    todo!() // cxx-internal:engine_passive.c.o:_mj_flexPassiveBendInterp
}

/// C: mj_flexPassiveBend (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_passive.c:444)
#[allow(unused_variables, non_snake_case)]
pub fn mj_flex_passive_bend(m: *const mjModel, d: *mut mjData, f: i32, enbl_spring: i32, enbl_damper: i32) {
    todo!() // cxx-internal:engine_passive.c.o:_mj_flexPassiveBend
}

/// C: mj_flexPassiveStretch (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_passive.c:524)
/// Calls: cxx-internal:engine_passive.c.o:_GradSquaredLengths, cxx:_mj_applyFT, cxx:_mj_freeStack, cxx:_mj_markStack, cxx:_mj_stackAllocInfo, cxx:_mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mj_flex_passive_stretch(m: *const mjModel, d: *mut mjData, f: i32, enbl_spring: i32, enbl_damper: i32) {
    todo!() // cxx-internal:engine_passive.c.o:_mj_flexPassiveStretch
}

/// C: mj_springdamper (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_passive.c:626)
/// Calls: cxx-internal:engine_passive.c.o:_mj_flexPassiveBend, cxx-internal:engine_passive.c.o:_mj_flexPassiveBendInterp, cxx-internal:engine_passive.c.o:_mj_flexPassiveInterp, cxx-internal:engine_passive.c.o:_mj_flexPassiveStretch, cxx:_mj_actuatorDamping, cxx:_mj_sleepState, cxx:_mju_copy, cxx:_mju_isZero, cxx:_mju_norm3, cxx:_mju_normalize4, cxx:_mju_polyForce
#[allow(unused_variables, non_snake_case)]
pub fn mj_springdamper(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx-internal:engine_passive.c.o:_mj_springdamper
}

/// C: mj_gravcomp (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_passive.c:817)
/// Calls: cxx:_mj_applyFT, cxx:_mju_norm3
#[allow(unused_variables, non_snake_case)]
pub fn mj_gravcomp(m: *const mjModel, d: *mut mjData) -> i32 {
    todo!() // cxx-internal:engine_passive.c.o:_mj_gravcomp
}

/// C: mj_fluid (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_passive.c:842)
/// Calls: cxx:_mj_ellipsoidFluidModel, cxx:_mj_inertiaBoxFluidModel
#[allow(unused_variables, non_snake_case)]
pub fn mj_fluid(m: *const mjModel, d: *mut mjData) -> i32 {
    todo!() // cxx-internal:engine_passive.c.o:_mj_fluid
}

/// C: mj_contactPassive (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_passive.c:878)
/// Calls: cxx:_mj_contactJacobian, cxx:_mj_freeStack, cxx:_mj_isSparse, cxx:_mj_markStack, cxx:_mj_stackAllocInfo, cxx:_mju_addToScl, cxx:_mju_mulMatMat, cxx:_mju_scl
#[allow(unused_variables, non_snake_case)]
pub fn mj_contact_passive(m: *const mjModel, d: *mut mjData) -> i32 {
    todo!() // cxx:_mj_contactPassive
}

/// C: mji_pow4 (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_passive.c:1215)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_pow4(val: f64) -> f64 {
    todo!() // cxx-internal:engine_passive.c.o:_mji_pow4
}

/// C: mji_pow2 (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_passive.c:1219)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_pow2(val: f64) -> f64 {
    todo!() // cxx-internal:engine_passive.c.o:_mji_pow2
}

/// C: mji_ellipsoid_max_moment (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_passive.c:1223)
/// Calls: cxx-internal:engine_passive.c.o:_mji_pow4, cxx:_mju_max
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_ellipsoid_max_moment(size: *const f64, dir: i32) -> f64 {
    todo!() // cxx-internal:engine_passive.c.o:_mji_ellipsoid_max_moment
}

/// C: mj_passive (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_passive.h:29)
/// Calls: cxx-internal:engine_passive.c.o:_mj_fluid, cxx-internal:engine_passive.c.o:_mj_gravcomp, cxx-internal:engine_passive.c.o:_mj_springdamper, cxx:_mj_contactPassive, cxx:_mjp_getPluginAtSlotUnsafe, cxx:_mjp_pluginCount, cxx:_mju_add, cxx:_mju_addInd, cxx:_mju_addTo, cxx:_mju_addToInd, cxx:_mju_message, cxx:_mju_zero, cxx:_mju_zeroInd
#[allow(unused_variables, non_snake_case)]
pub fn mj_passive(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx:_mj_passive
}

/// C: mj_inertiaBoxFluidModel (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_passive.h:37)
/// Calls: cxx:_mj_applyFT, cxx:_mj_objectVelocity, cxx:_mju_max, cxx:_mju_transformSpatial, cxx:_mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mj_inertia_box_fluid_model(m: *const mjModel, d: *mut mjData, i: i32) {
    todo!() // cxx:_mj_inertiaBoxFluidModel
}

/// C: mj_ellipsoidFluidModel (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_passive.h:40)
/// Calls: cxx:_mj_addedMassForces, cxx:_mj_applyFT, cxx:_mj_objectVelocity, cxx:_mj_viscousForces, cxx:_mju_geomSemiAxes, cxx:_mju_scl, cxx:_mju_transformSpatial, cxx:_mju_zero, cxx:_readFluidGeomInteraction
#[allow(unused_variables, non_snake_case)]
pub fn mj_ellipsoid_fluid_model(m: *const mjModel, d: *mut mjData, bodyid: i32) {
    todo!() // cxx:_mj_ellipsoidFluidModel
}

/// C: mj_addedMassForces (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_passive.h:43)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_added_mass_forces(local_vels: *const f64, local_accels: *const f64, fluid_density: f64, virtual_mass: *const f64, virtual_inertia: *const f64, local_force: *mut f64) {
    todo!() // cxx:_mj_addedMassForces
}

/// C: mj_viscousForces (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_passive.h:49)
/// Calls: cxx-internal:engine_passive.c.o:_mji_ellipsoid_max_moment, cxx-internal:engine_passive.c.o:_mji_pow2, cxx-internal:engine_passive.c.o:_mji_pow4, cxx:_mju_max, cxx:_mju_min, cxx:_mju_norm3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_viscous_forces(local_vels: *const f64, fluid_density: f64, fluid_viscosity: f64, size: *const f64, magnus_lift_coef: f64, kutta_lift_coef: f64, blunt_drag_coef: f64, slender_drag_coef: f64, ang_drag_coef: f64, local_force: *mut f64) {
    todo!() // cxx:_mj_viscousForces
}

/// C: readFluidGeomInteraction (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_passive.h:56)
/// Calls: cxx:_mju_message
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn read_fluid_geom_interaction(geom_fluid_coefs: *const f64, geom_fluid_coef: *mut f64, blunt_drag_coef: *mut f64, slender_drag_coef: *mut f64, ang_drag_coef: *mut f64, kutta_lift_coef: *mut f64, magnus_lift_coef: *mut f64, virtual_mass: *mut f64, virtual_inertia: *mut f64) {
    todo!() // cxx:_readFluidGeomInteraction
}

/// C: writeFluidGeomInteraction (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_passive.h:66)
/// Calls: cxx:_mju_message
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn write_fluid_geom_interaction(geom_fluid_coefs: *mut f64, geom_fluid_coef: *const f64, blunt_drag_coef: *const f64, slender_drag_coef: *const f64, ang_drag_coef: *const f64, kutta_lift_coef: *const f64, magnus_lift_coef: *const f64, virtual_mass: *const f64, virtual_inertia: *const f64) {
    todo!() // cxx:_writeFluidGeomInteraction
}

