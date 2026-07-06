//! Port of: engine/engine_passive.c
//! IR hash: 05737965add36adb
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: GradSquaredLengths (engine/engine_passive.c:48)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn grad_squared_lengths(gradient: [[[f64; 6]; 2]; 3], xpos: *const f64, vert: [i32; 4], edge: [[i32; 6]; 2], nedge: i32) {
    // WARNING: signature changed — verify body
    // Previous params: (gradient : [[[f64 ; 6] ; 2] ; 3], xpos : * const f64, vert : [i32 ; 4], edge : [[i32 ; 6] ; 2], nedge : i32)
    // Previous return: ()
    unsafe { let grad_ptr = & gradient as * const _ as * mut f64 ; let vert_ptr = vert . as_ptr () ; let edge_ptr = & edge as * const _ as * const i32 ; for e in 0 .. nedge as usize { for d in 0 .. 3usize { let e0 = * edge_ptr . add (e * 2 + 0) as usize ; let e1 = * edge_ptr . add (e * 2 + 1) as usize ; let v0 = * vert_ptr . add (e0) as usize ; let v1 = * vert_ptr . add (e1) as usize ; let val = * xpos . add (3 * v0 + d) - * xpos . add (3 * v1 + d) ; * grad_ptr . add (e * 6 + 0 * 3 + d) = val ; * grad_ptr . add (e * 6 + 1 * 3 + d) = - val ; } } }
}

/// C: mj_flexPassiveInterp (engine/engine_passive.c:63)
/// Calls: mj_applyFT, mj_freeStack, mj_markStack, mj_stackAllocInfo, mji_addScl3, mji_addTo3, mji_rotVecQuat, mju_flexGatherCellState, mju_flexGatherFaceState, mju_flexGatherState, mju_mulMatVec, mju_negQuat, mju_rotVecQuat, mju_scl3, mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mj_flex_passive_interp(m: *const mjModel, d: *mut mjData, f: i32, enbl_spring: i32, enbl_damper: i32) {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, f : i32, enbl_spring : i32, enbl_damper : i32)
    // Previous return: ()
    todo ! ()
}

/// C: mju_dphi2D (engine/engine_passive.c:211)
/// Calls: mju_flexDphi, mju_flexPhi
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_dphi2d(s0: f64, l0: i32, s1: f64, l1: i32, order: i32, dir: i32) -> f64 {
    // WARNING: signature changed — verify body
    // Previous params: (s0 : f64, l0 : i32, s1 : f64, l1 : i32, order : i32, dir : i32)
    // Previous return: f64
    use crate :: engine :: engine_util_misc :: { mju_flex_phi , mju_flex_dphi } ; if dir == 0 { mju_flex_dphi (s0 , l0 , order) * mju_flex_phi (s1 , l1 , order) } else { mju_flex_phi (s0 , l0 , order) * mju_flex_dphi (s1 , l1 , order) }
}

/// C: mj_flexPassiveBendInterp (engine/engine_passive.c:236)
/// Calls: mj_applyFT, mj_freeStack, mj_markStack, mj_stackAllocInfo, mji_addTo3, mji_cross, mji_sub3, mju_add, mju_copy, mju_copyInt, mju_dot, mju_dot3, mju_dphi2D, mju_flexFaceNormal2D, mju_flexGatherFaceState, mju_flexGatherState, mju_message, mju_negQuat, mju_norm3, mju_normalize, mju_rotVecQuat, mju_scl, mju_warning, mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mj_flex_passive_bend_interp(m: *const mjModel, d: *mut mjData, f: i32, enbl_spring: i32, enbl_damper: i32) {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, f : i32, enbl_spring : i32, enbl_damper : i32)
    // Previous return: ()
    todo ! ()
}

/// C: mj_flexPassiveBend (engine/engine_passive.c:444)
/// Calls: mji_cross, mji_sub3
#[allow(unused_variables, non_snake_case)]
pub fn mj_flex_passive_bend(m: *const mjModel, d: *mut mjData, f: i32, enbl_spring: i32, enbl_damper: i32) {
    extern "C" {
        fn mj_flexPassiveBend_impl(m: *const mjModel, d: *mut mjData, f: i32, enbl_spring: i32, enbl_damper: i32);
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_flexPassiveBend_impl(m, d, f, enbl_spring, enbl_damper) }
}

/// C: mj_flexPassiveStretch (engine/engine_passive.c:524)
/// Calls: GradSquaredLengths, mj_applyFT, mj_freeStack, mj_markStack, mj_stackAllocInfo, mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mj_flex_passive_stretch(m: *const mjModel, d: *mut mjData, f: i32, enbl_spring: i32, enbl_damper: i32) {
    extern "C" {
        fn mj_flexPassiveStretch_impl(m: *const mjModel, d: *mut mjData, f: i32, enbl_spring: i32, enbl_damper: i32);
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_flexPassiveStretch_impl(m, d, f, enbl_spring, enbl_damper) }
}

/// C: mj_springdamper (engine/engine_passive.c:626)
/// Calls: mj_actuatorDamping, mj_flexPassiveBend, mj_flexPassiveBendInterp, mj_flexPassiveInterp, mj_flexPassiveStretch, mj_sleepState, mji_addToScl3, mji_copy4, mji_sub3, mji_subQuat, mju_copy, mju_isZero, mju_norm3, mju_normalize4, mju_polyForce
#[allow(unused_variables, non_snake_case)]
pub fn mj_springdamper(m: *const mjModel, d: *mut mjData) {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData)
    // Previous return: ()
    todo ! ()
}

/// C: mj_gravcomp (engine/engine_passive.c:817)
/// Calls: mj_applyFT, mji_scl3, mju_norm3
#[allow(unused_variables, non_snake_case)]
pub fn mj_gravcomp(m: *const mjModel, d: *mut mjData) -> i32 {
    extern "C" {
        fn mj_gravcomp_impl(m: *const mjModel, d: *mut mjData) -> i32;
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_gravcomp_impl(m, d) }
}

/// C: mj_fluid (engine/engine_passive.c:842)
/// Calls: mj_ellipsoidFluidModel, mj_inertiaBoxFluidModel
#[allow(unused_variables, non_snake_case)]
pub fn mj_fluid(m: *const mjModel, d: *mut mjData) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData)
    // Previous return: i32
    todo ! ()
}

/// C: mj_contactPassive (engine/engine_passive.c:878)
/// Calls: mj_contactJacobian, mj_freeStack, mj_isSparse, mj_markStack, mj_stackAllocInfo, mju_addToScl, mju_mulMatMat, mju_scl
#[allow(unused_variables, non_snake_case)]
pub fn mj_contact_passive(m: *const mjModel, d: *mut mjData) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData)
    // Previous return: i32
    todo ! ()
}

/// C: mji_pow4 (engine/engine_passive.c:1215)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_pow4(val: f64) -> f64 {
    // WARNING: signature changed — verify body
    // Previous params: (val : f64)
    // Previous return: f64
    (val * val) * (val * val)
}

/// C: mji_pow2 (engine/engine_passive.c:1219)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_pow2(val: f64) -> f64 {
    // WARNING: signature changed — verify body
    // Previous params: (val : f64)
    // Previous return: f64
    val * val
}

/// C: mji_ellipsoid_max_moment (engine/engine_passive.c:1223)
/// Calls: mji_pow4, mju_max
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_ellipsoid_max_moment(size: *const f64, dir: i32) -> f64 {
    // WARNING: signature changed — verify body
    // Previous params: (size : * const f64, dir : i32)
    // Previous return: f64
    unsafe { let d0 = * size . add (dir as usize) ; let d1 = * size . add (((dir + 1) % 3) as usize) ; let d2 = * size . add (((dir + 2) % 3) as usize) ; let max_d = if d1 > d2 { d1 } else { d2 } ; 8.0 / 15.0 * std :: f64 :: consts :: PI * d0 * mji_pow4 (max_d) }
}

/// C: mj_passive (engine/engine_passive.h:29)
/// Calls: mj_contactPassive, mj_fluid, mj_gravcomp, mj_springdamper, mjp_getPluginAtSlotUnsafe, mjp_pluginCount, mju_add, mju_addInd, mju_addTo, mju_addToInd, mju_message, mju_zero, mju_zeroInd
#[allow(unused_variables, non_snake_case)]
pub fn mj_passive(m: *const mjModel, d: *mut mjData) {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData)
    // Previous return: ()
    todo ! ()
}

/// C: mj_inertiaBoxFluidModel (engine/engine_passive.h:37)
/// Calls: mj_applyFT, mj_objectVelocity, mji_copy3, mji_mulMatVec3, mji_scl3, mji_subFrom3, mju_max, mju_transformSpatial, mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mj_inertia_box_fluid_model(m: *const mjModel, d: *mut mjData, i: i32) {
    extern "C" {
        fn mj_inertiaBoxFluidModel_impl(m: *const mjModel, d: *mut mjData, i: i32);
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_inertiaBoxFluidModel_impl(m, d, i) }
}

/// C: mj_ellipsoidFluidModel (engine/engine_passive.h:40)
/// Calls: mj_addedMassForces, mj_applyFT, mj_objectVelocity, mj_viscousForces, mji_copy3, mji_mulMatVec3, mji_subFrom3, mju_geomSemiAxes, mju_scl, mju_transformSpatial, mju_zero, readFluidGeomInteraction
#[allow(unused_variables, non_snake_case)]
pub fn mj_ellipsoid_fluid_model(m: *const mjModel, d: *mut mjData, bodyid: i32) {
    extern "C" {
        fn mj_ellipsoidFluidModel_impl(m: *const mjModel, d: *mut mjData, bodyid: i32);
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_ellipsoidFluidModel_impl(m, d, bodyid) }
}

/// C: mj_addedMassForces (engine/engine_passive.h:43)
/// Calls: mji_addTo3, mji_cross
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_added_mass_forces(local_vels: *const f64, local_accels: *const f64, fluid_density: f64, virtual_mass: *const f64, virtual_inertia: *const f64, local_force: *mut f64) {
    // WARNING: signature changed — verify body
    // Previous params: (local_vels : * const f64, local_accels : * const f64, fluid_density : f64, virtual_mass : * const f64, virtual_inertia : * const f64, local_force : * mut f64)
    // Previous return: ()
    todo ! ()
}

/// C: mj_viscousForces (engine/engine_passive.h:49)
/// Calls: mji_cross, mji_ellipsoid_max_moment, mji_pow2, mji_pow4, mju_max, mju_min, mju_norm3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_viscous_forces(local_vels: *const f64, fluid_density: f64, fluid_viscosity: f64, size: *const f64, magnus_lift_coef: f64, kutta_lift_coef: f64, blunt_drag_coef: f64, slender_drag_coef: f64, ang_drag_coef: f64, local_force: *mut f64) {
    extern "C" {
        fn mj_viscousForces_impl(local_vels: *const f64, fluid_density: f64, fluid_viscosity: f64, size: *const f64, magnus_lift_coef: f64, kutta_lift_coef: f64, blunt_drag_coef: f64, slender_drag_coef: f64, ang_drag_coef: f64, local_force: *mut f64);
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_viscousForces_impl(local_vels, fluid_density, fluid_viscosity, size, magnus_lift_coef, kutta_lift_coef, blunt_drag_coef, slender_drag_coef, ang_drag_coef, local_force) }
}

/// C: readFluidGeomInteraction (engine/engine_passive.h:56)
/// Calls: mju_message
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn read_fluid_geom_interaction(geom_fluid_coefs: *const f64, geom_fluid_coef: *mut f64, blunt_drag_coef: *mut f64, slender_drag_coef: *mut f64, ang_drag_coef: *mut f64, kutta_lift_coef: *mut f64, magnus_lift_coef: *mut f64, virtual_mass: *mut f64, virtual_inertia: *mut f64) {
    extern "C" {
        fn readFluidGeomInteraction_impl(geom_fluid_coefs: *const f64, geom_fluid_coef: *mut f64, blunt_drag_coef: *mut f64, slender_drag_coef: *mut f64, ang_drag_coef: *mut f64, kutta_lift_coef: *mut f64, magnus_lift_coef: *mut f64, virtual_mass: *mut f64, virtual_inertia: *mut f64);
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { readFluidGeomInteraction_impl(geom_fluid_coefs, geom_fluid_coef, blunt_drag_coef, slender_drag_coef, ang_drag_coef, kutta_lift_coef, magnus_lift_coef, virtual_mass, virtual_inertia) }
}

/// C: writeFluidGeomInteraction (engine/engine_passive.h:66)
/// Calls: mju_message
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn write_fluid_geom_interaction(geom_fluid_coefs: *mut f64, geom_fluid_coef: *const f64, blunt_drag_coef: *const f64, slender_drag_coef: *const f64, ang_drag_coef: *const f64, kutta_lift_coef: *const f64, magnus_lift_coef: *const f64, virtual_mass: *const f64, virtual_inertia: *const f64) {
    // WARNING: signature changed — verify body
    // Previous params: (geom_fluid_coefs : * mut f64, geom_fluid_coef : * const f64, blunt_drag_coef : * const f64, slender_drag_coef : * const f64, ang_drag_coef : * const f64, kutta_lift_coef : * const f64, magnus_lift_coef : * const f64, virtual_mass : * const f64, virtual_inertia : * const f64)
    // Previous return: ()
    todo ! ()
}

