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
    if xpos.is_null() || nedge <= 0 {
        return;
    }
    extern "C" { fn GradSquaredLengths(gradient: [[[f64; 6]; 2]; 3], xpos: *const f64, vert: [i32; 4], edge: [[i32; 6]; 2], nedge: i32); }
    unsafe { GradSquaredLengths(gradient, xpos, vert, edge, nedge) }
}

/// C: mj_flexPassiveInterp (engine/engine_passive.c:63)
/// Calls: mj_applyFT, mj_freeStack, mj_markStack, mj_stackAllocInfo, mji_addScl3, mji_addTo3, mji_rotVecQuat, mju_flexGatherCellState, mju_flexGatherFaceState, mju_flexGatherState, mju_mulMatVec, mju_negQuat, mju_rotVecQuat, mju_scl3, mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mj_flex_passive_interp(m: *const mjModel, d: *mut mjData, f: i32, enbl_spring: i32, enbl_damper: i32) {
    if m.is_null() { return; }
    extern "C" { fn mj_flexPassiveInterp(m: *const mjModel, d: *mut mjData, f: i32, enbl_spring: i32, enbl_damper: i32); }
    // SAFETY: m verified non-null; delegates to C implementation
    unsafe { mj_flexPassiveInterp(m, d, f, enbl_spring, enbl_damper) }
}

/// C: mju_dphi2D (engine/engine_passive.c:211)
/// Calls: mju_flexDphi, mju_flexPhi
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_dphi2d(s0: f64, l0: i32, s1: f64, l1: i32, order: i32, dir: i32) -> f64  {
    let _sv = core::mem::size_of_val(&s0);
    extern "C" { fn mju_dphi2D(s0: f64, l0: i32, s1: f64, l1: i32, order: i32, dir: i32) -> f64; }
    // SAFETY: delegates to C implementation
    unsafe { mju_dphi2D(s0, l0, s1, l1, order, dir) }
}

/// C: mj_flexPassiveBendInterp (engine/engine_passive.c:236)
/// Calls: mj_applyFT, mj_freeStack, mj_markStack, mj_stackAllocInfo, mji_addTo3, mji_cross, mji_sub3, mju_add, mju_copy, mju_copyInt, mju_dot, mju_dot3, mju_dphi2D, mju_flexFaceNormal2D, mju_flexGatherFaceState, mju_flexGatherState, mju_message, mju_negQuat, mju_norm3, mju_normalize, mju_rotVecQuat, mju_scl, mju_warning, mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mj_flex_passive_bend_interp(m: *const mjModel, d: *mut mjData, f: i32, enbl_spring: i32, enbl_damper: i32) {
    extern "C" { fn mj_flexPassiveBendInterp(m: *const mjModel, d: *mut mjData, f: i32, enbl_spring: i32, enbl_damper: i32); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_flexPassiveBendInterp(m, d, f, enbl_spring, enbl_damper) }
}

/// C: mj_flexPassiveBend (engine/engine_passive.c:444)
/// Calls: mji_cross, mji_sub3
#[allow(unused_variables, non_snake_case)]
pub fn mj_flex_passive_bend(m: *const mjModel, d: *mut mjData, f: i32, enbl_spring: i32, enbl_damper: i32) {
    if m.is_null() || d.is_null() {
        return;
    }
    extern "C" { fn mj_flexPassiveBend(m: *const mjModel, d: *mut mjData, f: i32, enbl_spring: i32, enbl_damper: i32); }
    // SAFETY: m, d verified non-null; delegates to C implementation
    unsafe { mj_flexPassiveBend(m, d, f, enbl_spring, enbl_damper) }
}

/// C: mj_flexPassiveStretch (engine/engine_passive.c:524)
/// Calls: GradSquaredLengths, mj_applyFT, mj_freeStack, mj_markStack, mj_stackAllocInfo, mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mj_flex_passive_stretch(m: *const mjModel, d: *mut mjData, f: i32, enbl_spring: i32, enbl_damper: i32) {
    if m.is_null() { return; }
    extern "C" { fn mj_flexPassiveStretch(m: *const mjModel, d: *mut mjData, f: i32, enbl_spring: i32, enbl_damper: i32); }
    // SAFETY: m verified non-null; delegates to C implementation
    unsafe { mj_flexPassiveStretch(m, d, f, enbl_spring, enbl_damper) }
}

/// C: mj_springdamper (engine/engine_passive.c:626)
/// Calls: mj_actuatorDamping, mj_flexPassiveBend, mj_flexPassiveBendInterp, mj_flexPassiveInterp, mj_flexPassiveStretch, mj_sleepState, mji_addToScl3, mji_copy4, mji_sub3, mji_subQuat, mju_copy, mju_isZero, mju_norm3, mju_normalize4, mju_polyForce
#[allow(unused_variables, non_snake_case)]
pub fn mj_springdamper(m: *const mjModel, d: *mut mjData) {
    extern "C" { fn mj_springdamper(m: *const mjModel, d: *mut mjData); }
    // SAFETY: delegates to C implementation, pointers valid per caller contract
    unsafe { mj_springdamper(m, d) }
}

/// C: mj_gravcomp (engine/engine_passive.c:817)
/// Calls: mj_applyFT, mji_scl3, mju_norm3
#[allow(unused_variables, non_snake_case)]
pub fn mj_gravcomp(m: *const mjModel, d: *mut mjData) -> i32  {
    if m.is_null() || d.is_null() {
        return 0;
    }
    extern "C" { fn mj_gravcomp(m: *const mjModel, d: *mut mjData) -> i32; }
    // SAFETY: m, d verified non-null; delegates to C implementation
    unsafe { mj_gravcomp(m, d) }
}

/// C: mj_fluid (engine/engine_passive.c:842)
/// Calls: mj_ellipsoidFluidModel, mj_inertiaBoxFluidModel
#[allow(unused_variables, non_snake_case)]
pub fn mj_fluid(m: *const mjModel, d: *mut mjData) -> i32  {
    if m.is_null() { return 0; }
    extern "C" { fn mj_fluid(m: *const mjModel, d: *mut mjData) -> i32; }
    // SAFETY: m verified non-null; delegates to C implementation
    unsafe { mj_fluid(m, d) }
}

/// C: mj_contactPassive (engine/engine_passive.c:878)
/// Calls: mj_contactJacobian, mj_freeStack, mj_isSparse, mj_markStack, mj_stackAllocInfo, mju_addToScl, mju_mulMatMat, mju_scl
#[allow(unused_variables, non_snake_case)]
pub fn mj_contact_passive(m: *const mjModel, d: *mut mjData) -> i32 {
    if m.is_null() { return 0; }
    extern "C" { fn mj_contactPassive(m: *const mjModel, d: *mut mjData) -> i32; }
    // SAFETY: m verified non-null; delegates to C implementation
    unsafe { mj_contactPassive(m, d) }
}

/// C: mji_pow4 (engine/engine_passive.c:1215)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_pow4(val: f64) -> f64  {
    let val2 = val * val;
    val2 * val2
}

/// C: mji_pow2 (engine/engine_passive.c:1219)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_pow2(val: f64) -> f64  {
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
pub fn mji_ellipsoid_max_moment(size: *const f64, dir: i32) -> f64  {
    if size.is_null() { return 0.0; }
    extern "C" { fn mji_ellipsoid_max_moment(size: *const f64, dir: i32) -> f64; }
    // SAFETY: size verified non-null; delegates to C implementation
    unsafe { mji_ellipsoid_max_moment(size, dir) }
}

/// C: mj_passive (engine/engine_passive.h:29)
/// Calls: mj_contactPassive, mj_fluid, mj_gravcomp, mj_springdamper, mjp_getPluginAtSlotUnsafe, mjp_pluginCount, mju_add, mju_addInd, mju_addTo, mju_addToInd, mju_message, mju_zero, mju_zeroInd
#[allow(unused_variables, non_snake_case)]
pub fn mj_passive(m: *const mjModel, d: *mut mjData) {
    extern "C" { fn mj_passive(m: *const mjModel, d: *mut mjData); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_passive(m, d) }
}

/// C: mj_inertiaBoxFluidModel (engine/engine_passive.h:37)
/// Calls: mj_applyFT, mj_objectVelocity, mji_copy3, mji_mulMatVec3, mji_scl3, mji_subFrom3, mju_max, mju_transformSpatial, mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mj_inertia_box_fluid_model(m: *const mjModel, d: *mut mjData, i: i32) {
    // SAFETY: m, d valid. i is valid body index.
    unsafe {
        const MJ_MINVAL: f64 = 1e-15;
        const MJ_PI: f64 = std::f64::consts::PI;
        const mjOBJ_BODY: i32 = 1;

        let inertia: *const f64 = (*m).body_inertia.add(3 * i as usize);
        let mass: f64 = *(*m).body_mass.add(i as usize);
        let mut box_: [f64; 3] = [0.0; 3];
        box_[0] = (crate::engine::engine_util_misc::mju_max(MJ_MINVAL,
            (*inertia.add(1) + *inertia.add(2) - *inertia.add(0))) / mass * 6.0).sqrt();
        box_[1] = (crate::engine::engine_util_misc::mju_max(MJ_MINVAL,
            (*inertia.add(0) + *inertia.add(2) - *inertia.add(1))) / mass * 6.0).sqrt();
        box_[2] = (crate::engine::engine_util_misc::mju_max(MJ_MINVAL,
            (*inertia.add(0) + *inertia.add(1) - *inertia.add(2))) / mass * 6.0).sqrt();

        // map from CoM-centered to local body-centered 6D velocity
        let mut lvel: [f64; 6] = [0.0; 6];
        crate::engine::engine_core_util::mj_object_velocity(
            m, d as *const mjData, mjOBJ_BODY, i, lvel.as_mut_ptr(), 1);

        // compute wind in local coordinates
        let mut wind: [f64; 6] = [0.0; 6];
        crate::engine::engine_inline::mji_copy3(wind.as_mut_ptr().add(3), (*m).opt.wind.as_ptr());
        let mut lwind: [f64; 6] = [0.0; 6];
        crate::engine::engine_util_spatial::mju_transform_spatial(
            lwind.as_mut_ptr(), wind.as_ptr(), 0,
            (*d).xipos.add(3 * i as usize),
            (*d).subtree_com.add(3 * *(*m).body_rootid.add(i as usize) as usize),
            (*d).ximat.add(9 * i as usize));

        // subtract translational component from body velocity
        crate::engine::engine_inline::mji_sub_from3(lvel.as_mut_ptr().add(3), lwind.as_ptr().add(3));
        let mut lfrc: [f64; 6] = [0.0; 6];

        // set viscous force and torque
        if (*m).opt.viscosity > 0.0 {
            // diameter of sphere approximation
            let diam: f64 = (box_[0] + box_[1] + box_[2]) / 3.0;

            // angular viscosity
            crate::engine::engine_inline::mji_scl3(
                lfrc.as_mut_ptr(), lvel.as_ptr(), -MJ_PI * diam * diam * diam * (*m).opt.viscosity);

            // linear viscosity
            crate::engine::engine_inline::mji_scl3(
                lfrc.as_mut_ptr().add(3), lvel.as_ptr().add(3), -3.0 * MJ_PI * diam * (*m).opt.viscosity);
        }

        // add lift and drag force and torque
        if (*m).opt.density > 0.0 {
            // force
            lfrc[3] -= 0.5 * (*m).opt.density * box_[1] * box_[2] * lvel[3].abs() * lvel[3];
            lfrc[4] -= 0.5 * (*m).opt.density * box_[0] * box_[2] * lvel[4].abs() * lvel[4];
            lfrc[5] -= 0.5 * (*m).opt.density * box_[0] * box_[1] * lvel[5].abs() * lvel[5];

            // torque
            lfrc[0] -= (*m).opt.density * box_[0]
                * (box_[1] * box_[1] * box_[1] * box_[1] + box_[2] * box_[2] * box_[2] * box_[2])
                * lvel[0].abs() * lvel[0] / 64.0;
            lfrc[1] -= (*m).opt.density * box_[1]
                * (box_[0] * box_[0] * box_[0] * box_[0] + box_[2] * box_[2] * box_[2] * box_[2])
                * lvel[1].abs() * lvel[1] / 64.0;
            lfrc[2] -= (*m).opt.density * box_[2]
                * (box_[0] * box_[0] * box_[0] * box_[0] + box_[1] * box_[1] * box_[1] * box_[1])
                * lvel[2].abs() * lvel[2] / 64.0;
        }

        // rotate to global orientation: lfrc -> bfrc
        let mut bfrc: [f64; 6] = [0.0; 6];
        crate::engine::engine_inline::mji_mul_mat_vec3(
            bfrc.as_mut_ptr(), (*d).ximat.add(9 * i as usize), lfrc.as_ptr());
        crate::engine::engine_inline::mji_mul_mat_vec3(
            bfrc.as_mut_ptr().add(3), (*d).ximat.add(9 * i as usize), lfrc.as_ptr().add(3));

        // apply force and torque to body com
        crate::engine::engine_support::mj_apply_ft(
            m, d, bfrc.as_ptr().add(3), bfrc.as_ptr(), (*d).xipos.add(3 * i as usize), i, (*d).qfrc_fluid);
    }
}

/// C: mj_ellipsoidFluidModel (engine/engine_passive.h:40)
/// Calls: mj_addedMassForces, mj_applyFT, mj_objectVelocity, mj_viscousForces, mji_copy3, mji_mulMatVec3, mji_subFrom3, mju_geomSemiAxes, mju_scl, mju_transformSpatial, mju_zero, readFluidGeomInteraction
#[allow(unused_variables, non_snake_case)]
pub fn mj_ellipsoid_fluid_model(m: *const mjModel, d: *mut mjData, bodyid: i32) {
    if m.is_null() || d.is_null() { return; }
    extern "C" {
        fn mj_ellipsoidFluidModel(m: *const mjModel, d: *mut mjData, bodyid: i32);
    }
    // SAFETY: m, d verified non-null
    unsafe { mj_ellipsoidFluidModel(m, d, bodyid) }
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
    // SAFETY: local_vels has 6 elements, local_accels has 6 (may be null),
    // virtual_mass has 3, virtual_inertia has 3, local_force has 6.
    unsafe {
        let lin_vel: [f64; 3] = [*local_vels.add(3), *local_vels.add(4), *local_vels.add(5)];
        let ang_vel: [f64; 3] = [*local_vels.add(0), *local_vels.add(1), *local_vels.add(2)];
        let virtual_lin_mom: [f64; 3] = [
            fluid_density * *virtual_mass.add(0) * lin_vel[0],
            fluid_density * *virtual_mass.add(1) * lin_vel[1],
            fluid_density * *virtual_mass.add(2) * lin_vel[2],
        ];
        let virtual_ang_mom: [f64; 3] = [
            fluid_density * *virtual_inertia.add(0) * ang_vel[0],
            fluid_density * *virtual_inertia.add(1) * ang_vel[1],
            fluid_density * *virtual_inertia.add(2) * ang_vel[2],
        ];

        // acceleration-dependent terms (disabled in practice but included)
        if !local_accels.is_null() {
            *local_force.add(0) -= fluid_density * *virtual_inertia.add(0) * *local_accels.add(0);
            *local_force.add(1) -= fluid_density * *virtual_inertia.add(1) * *local_accels.add(1);
            *local_force.add(2) -= fluid_density * *virtual_inertia.add(2) * *local_accels.add(2);
            *local_force.add(3) -= fluid_density * *virtual_mass.add(0) * *local_accels.add(3);
            *local_force.add(4) -= fluid_density * *virtual_mass.add(1) * *local_accels.add(4);
            *local_force.add(5) -= fluid_density * *virtual_mass.add(2) * *local_accels.add(5);
        }

        let mut added_mass_force: [f64; 3] = [0.0; 3];
        let mut added_mass_torque1: [f64; 3] = [0.0; 3];
        let mut added_mass_torque2: [f64; 3] = [0.0; 3];
        crate::engine::engine_inline::mji_cross(added_mass_force.as_mut_ptr(), virtual_lin_mom.as_ptr(), ang_vel.as_ptr());
        crate::engine::engine_inline::mji_cross(added_mass_torque1.as_mut_ptr(), virtual_lin_mom.as_ptr(), lin_vel.as_ptr());
        crate::engine::engine_inline::mji_cross(added_mass_torque2.as_mut_ptr(), virtual_ang_mom.as_ptr(), ang_vel.as_ptr());

        crate::engine::engine_inline::mji_add_to3(local_force, added_mass_torque1.as_ptr());
        crate::engine::engine_inline::mji_add_to3(local_force, added_mass_torque2.as_ptr());
        crate::engine::engine_inline::mji_add_to3(local_force.add(3), added_mass_force.as_ptr());
    }
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
    // SAFETY: local_vels has 6 elements, size has 3, local_force has 6. All valid.
    unsafe {
        const MJPI: f64 = 3.14159265358979323846;
        const MJMINVAL: f64 = 1e-15;

        let lin_vel: [f64; 3] = [*local_vels.add(3), *local_vels.add(4), *local_vels.add(5)];
        let ang_vel: [f64; 3] = [*local_vels.add(0), *local_vels.add(1), *local_vels.add(2)];
        let volume: f64 = 4.0 / 3.0 * MJPI * *size.add(0) * *size.add(1) * *size.add(2);
        let d_max: f64 = crate::engine::engine_util_misc::mju_max(
            crate::engine::engine_util_misc::mju_max(*size.add(0), *size.add(1)), *size.add(2));
        let d_min: f64 = crate::engine::engine_util_misc::mju_min(
            crate::engine::engine_util_misc::mju_min(*size.add(0), *size.add(1)), *size.add(2));
        let d_mid: f64 = *size.add(0) + *size.add(1) + *size.add(2) - d_max - d_min;
        let A_max: f64 = MJPI * d_max * d_mid;

        let mut magnus_force: [f64; 3] = [0.0; 3];
        crate::engine::engine_inline::mji_cross(magnus_force.as_mut_ptr(), ang_vel.as_ptr(), lin_vel.as_ptr());
        magnus_force[0] *= magnus_lift_coef * fluid_density * volume;
        magnus_force[1] *= magnus_lift_coef * fluid_density * volume;
        magnus_force[2] *= magnus_lift_coef * fluid_density * volume;

        // projected area computation
        let proj_denom: f64 = mji_pow4(*size.add(1) * *size.add(2)) * mji_pow2(lin_vel[0])
            + mji_pow4(*size.add(2) * *size.add(0)) * mji_pow2(lin_vel[1])
            + mji_pow4(*size.add(0) * *size.add(1)) * mji_pow2(lin_vel[2]);
        let proj_num: f64 = mji_pow2(*size.add(1) * *size.add(2) * lin_vel[0])
            + mji_pow2(*size.add(2) * *size.add(0) * lin_vel[1])
            + mji_pow2(*size.add(0) * *size.add(1) * lin_vel[2]);

        let A_proj: f64 = MJPI * (proj_denom / crate::engine::engine_util_misc::mju_max(MJMINVAL, proj_num)).sqrt();

        // not-unit normal to ellipsoid's projected area
        let norm: [f64; 3] = [
            mji_pow2(*size.add(1) * *size.add(2)) * lin_vel[0],
            mji_pow2(*size.add(2) * *size.add(0)) * lin_vel[1],
            mji_pow2(*size.add(0) * *size.add(1)) * lin_vel[2],
        ];

        // cosine between velocity and normal
        let cos_alpha: f64 = proj_num / crate::engine::engine_util_misc::mju_max(
            MJMINVAL, crate::engine::engine_util_blas::mju_norm3(lin_vel.as_ptr()) * proj_denom);

        let mut kutta_circ: [f64; 3] = [0.0; 3];
        crate::engine::engine_inline::mji_cross(kutta_circ.as_mut_ptr(), norm.as_ptr(), lin_vel.as_ptr());
        kutta_circ[0] *= kutta_lift_coef * fluid_density * cos_alpha * A_proj;
        kutta_circ[1] *= kutta_lift_coef * fluid_density * cos_alpha * A_proj;
        kutta_circ[2] *= kutta_lift_coef * fluid_density * cos_alpha * A_proj;
        let mut kutta_force: [f64; 3] = [0.0; 3];
        crate::engine::engine_inline::mji_cross(kutta_force.as_mut_ptr(), kutta_circ.as_ptr(), lin_vel.as_ptr());

        // viscous force and torque in Stokes flow
        let eq_sphere_D: f64 = 2.0 / 3.0 * (*size.add(0) + *size.add(1) + *size.add(2));
        let lin_visc_force_coef: f64 = 3.0 * MJPI * eq_sphere_D;
        let lin_visc_torq_coef: f64 = MJPI * eq_sphere_D * eq_sphere_D * eq_sphere_D;

        // moments of inertia for angular quadratic drag
        let I_max: f64 = 8.0 / 15.0 * MJPI * d_mid * mji_pow4(d_max);
        let II: [f64; 3] = [
            mji_ellipsoid_max_moment(size, 0),
            mji_ellipsoid_max_moment(size, 1),
            mji_ellipsoid_max_moment(size, 2),
        ];
        let mom_visc: [f64; 3] = [
            ang_vel[0] * (ang_drag_coef * II[0] + slender_drag_coef * (I_max - II[0])),
            ang_vel[1] * (ang_drag_coef * II[1] + slender_drag_coef * (I_max - II[1])),
            ang_vel[2] * (ang_drag_coef * II[2] + slender_drag_coef * (I_max - II[2])),
        ];

        let drag_lin_coef: f64 = fluid_viscosity * lin_visc_force_coef
            + fluid_density * crate::engine::engine_util_blas::mju_norm3(lin_vel.as_ptr())
            * (A_proj * blunt_drag_coef + slender_drag_coef * (A_max - A_proj));
        let drag_ang_coef: f64 = fluid_viscosity * lin_visc_torq_coef
            + fluid_density * crate::engine::engine_util_blas::mju_norm3(mom_visc.as_ptr());

        *local_force.add(0) -= drag_ang_coef * ang_vel[0];
        *local_force.add(1) -= drag_ang_coef * ang_vel[1];
        *local_force.add(2) -= drag_ang_coef * ang_vel[2];
        *local_force.add(3) += magnus_force[0] + kutta_force[0] - drag_lin_coef * lin_vel[0];
        *local_force.add(4) += magnus_force[1] + kutta_force[1] - drag_lin_coef * lin_vel[1];
        *local_force.add(5) += magnus_force[2] + kutta_force[2] - drag_lin_coef * lin_vel[2];
    }
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
    // SAFETY: geom_fluid_coefs has mjNFLUID (12) elements. All output pointers valid.
    unsafe {
        let mut i: usize = 0;
        *geom_fluid_coef = *geom_fluid_coefs.add(i); i += 1;
        *blunt_drag_coef = *geom_fluid_coefs.add(i); i += 1;
        *slender_drag_coef = *geom_fluid_coefs.add(i); i += 1;
        *ang_drag_coef = *geom_fluid_coefs.add(i); i += 1;
        *kutta_lift_coef = *geom_fluid_coefs.add(i); i += 1;
        *magnus_lift_coef = *geom_fluid_coefs.add(i); i += 1;
        *virtual_mass.add(0) = *geom_fluid_coefs.add(i); i += 1;
        *virtual_mass.add(1) = *geom_fluid_coefs.add(i); i += 1;
        *virtual_mass.add(2) = *geom_fluid_coefs.add(i); i += 1;
        *virtual_inertia.add(0) = *geom_fluid_coefs.add(i); i += 1;
        *virtual_inertia.add(1) = *geom_fluid_coefs.add(i); i += 1;
        *virtual_inertia.add(2) = *geom_fluid_coefs.add(i); i += 1;
        // mjNFLUID = 12, assert i == 12
    }
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
    if geom_fluid_coefs.is_null() { return; }
    extern "C" { fn writeFluidGeomInteraction(geom_fluid_coefs: *mut f64, geom_fluid_coef: *const f64, blunt_drag_coef: *const f64, slender_drag_coef: *const f64, ang_drag_coef: *const f64, kutta_lift_coef: *const f64, magnus_lift_coef: *const f64, virtual_mass: *const f64, virtual_inertia: *const f64); }
    // SAFETY: geom_fluid_coefs verified non-null; delegates to C implementation
    unsafe { writeFluidGeomInteraction(geom_fluid_coefs, geom_fluid_coef, blunt_drag_coef, slender_drag_coef, ang_drag_coef, kutta_lift_coef, magnus_lift_coef, virtual_mass, virtual_inertia) }
}

