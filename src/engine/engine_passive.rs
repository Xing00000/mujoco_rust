//! Port of: engine/engine_passive.c
//! IR hash: 9343293228317031
//! CODEGEN: source paths, owners, and callable names are locked.

use crate::types::*;

/// C: GradSquaredLengths (engine/engine_passive.c:48)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn GradSquaredLengths(gradient: *mut [[f64; 3]; 2], xpos: *const f64, vert: *const i32, edge: *const [i32; 2], nedge: i32) {
    // SAFETY: caller guarantees all pointers are valid and arrays are properly sized
    unsafe {
        for e in 0..nedge as usize {
            for d in 0..3usize {
                let v0 = *vert.add((*edge.add(e))[0] as usize);
                let v1 = *vert.add((*edge.add(e))[1] as usize);
                (*gradient.add(e))[0][d] = *xpos.add(3 * v0 as usize + d) - *xpos.add(3 * v1 as usize + d);
                (*gradient.add(e))[1][d] = *xpos.add(3 * v1 as usize + d) - *xpos.add(3 * v0 as usize + d);
            }
        }
    }
}

/// C: mj_flexPassiveBend (engine/engine_passive.c:444)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_cross, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_sub3
#[allow(unused_variables, non_snake_case)]
pub fn mj_flexPassiveBend(m: *const mjModel, d: *mut mjData, f: i32, enbl_spring: i32, enbl_damper: i32) {
    use crate::engine::engine_inline::{mji_sub3, mji_cross};

    // SAFETY: caller guarantees m and d are valid
    unsafe {
        if *(*m).flex_dim.add(f as usize) != 2 {
            return;
        }

        let bendingadr: i32 = *(*m).flex_bendingadr.add(f as usize);
        if bendingadr < 0 {
            return;
        }

        let edgenum: i32 = *(*m).flex_edgenum.add(f as usize);
        let xpos: *mut f64 = (*d).flexvert_xpos.add(3 * *(*m).flex_vertadr.add(f as usize) as usize);
        let bodyid: *const i32 = (*m).flex_vertbodyid.add(*(*m).flex_vertadr.add(f as usize) as usize);
        let b: *const f64 = (*m).flex_bending.add(bendingadr as usize);

        for e in 0..edgenum {
            let edge: *const i32 = (*m).flex_edge.add(2 * (e + *(*m).flex_edgeadr.add(f as usize)) as usize);
            let flap: *const i32 = (*m).flex_edgeflap.add(2 * (e + *(*m).flex_edgeadr.add(f as usize)) as usize);
            let v: [i32; 4] = [*edge, *edge.add(1), *flap, *flap.add(1)];
            if v[3] == -1 {
                // skip boundary edges
                continue;
            }

            // flap edges
            let mut ed: [[f64; 3]; 3] = [[0.0; 3]; 3];
            mji_sub3(ed[0].as_mut_ptr(), xpos.add(3 * v[1] as usize), xpos.add(3 * v[0] as usize));
            mji_sub3(ed[1].as_mut_ptr(), xpos.add(3 * v[2] as usize), xpos.add(3 * v[0] as usize));
            mji_sub3(ed[2].as_mut_ptr(), xpos.add(3 * v[3] as usize), xpos.add(3 * v[0] as usize));

            // forces at the vertices due to curved reference
            let mut frc: [[f64; 3]; 4] = [[0.0; 3]; 4];
            mji_cross(frc[1].as_mut_ptr(), ed[1].as_ptr(), ed[2].as_ptr());
            mji_cross(frc[2].as_mut_ptr(), ed[2].as_ptr(), ed[0].as_ptr());
            mji_cross(frc[3].as_mut_ptr(), ed[0].as_ptr(), ed[1].as_ptr());
            frc[0][0] = -(frc[1][0] + frc[2][0] + frc[3][0]);
            frc[0][1] = -(frc[1][1] + frc[2][1] + frc[3][1]);
            frc[0][2] = -(frc[1][2] + frc[2][2] + frc[3][2]);

            // velocities
            let mut vel_ptrs: [*mut f64; 4] = [std::ptr::null_mut(); 4];
            for i in 0..4 {
                vel_ptrs[i] = (*d).qvel.add(*(*m).body_dofadr.add(*bodyid.add(v[i] as usize) as usize) as usize);
            }

            // force
            let mut spring: [f64; 12] = [0.0; 12];
            let mut damper: [f64; 12] = [0.0; 12];
            for i in 0..4usize {
                for x in 0..3usize {
                    for j in 0..4usize {
                        // thin plate bending force
                        if enbl_spring != 0 {
                            spring[3 * i + x] += *b.add(17 * e as usize + 4 * i + j) * *xpos.add(3 * v[j] as usize + x);
                        }

                        // thin plate damping force
                        if enbl_damper != 0 {
                            damper[3 * i + x] += *b.add(17 * e as usize + 4 * i + j) * *vel_ptrs[j].add(x);
                        }
                    }

                    // curved reference contribution
                    if enbl_spring != 0 {
                        spring[3 * i + x] += *b.add(17 * e as usize + 16) * frc[i][x];
                    }
                }
            }

            // insert into global force
            for i in 0..4usize {
                let bid: i32 = *bodyid.add(v[i] as usize);
                let body_dofnum: i32 = *(*m).body_dofnum.add(bid as usize);
                let body_dofadr: i32 = *(*m).body_dofadr.add(bid as usize);
                for x in 0..body_dofnum as usize {
                    if enbl_spring != 0 {
                        *(*d).qfrc_spring.add((body_dofadr as usize) + x) -= spring[3 * i + x];
                    }
                    if enbl_damper != 0 {
                        *(*d).qfrc_damper.add((body_dofadr as usize) + x) -= damper[3 * i + x] * *(*m).flex_damping.add(f as usize);
                    }
                }
            }
        }
    }
}

/// C: mji_pow4 (engine/engine_passive.c:1215)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_pow4(val: f64) -> f64 {
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
    val * val
}

/// C: mji_ellipsoid_max_moment (engine/engine_passive.c:1223)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_passive.c:_mji_pow4, cxx:_mju_max
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_ellipsoid_max_moment(size: *const f64, dir: i32) -> f64 {
    // SAFETY: caller guarantees size points to 3 contiguous f64s
    unsafe {
        let d0 = *size.add(dir as usize);
        let d1 = *size.add(((dir + 1) % 3) as usize);
        let d2 = *size.add(((dir + 2) % 3) as usize);
        8.0 / 15.0 * std::f64::consts::PI * d0
            * mji_pow4(crate::engine::engine_util_misc::mju_max(d1, d2))
    }
}

/// C: mj_addedMassForces (engine/engine_passive.h:43)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_addTo3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_cross
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_addedMassForces(local_vels: *const f64, local_accels: *const f64, fluid_density: f64, virtual_mass: *const f64, virtual_inertia: *const f64, local_force: *mut f64) {
    use crate::engine::engine_inline::{mji_cross, mji_addTo3};
    // SAFETY: caller guarantees all pointers are valid and arrays are properly sized
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

        // disabled due to dependency on qacc but included for completeness
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
        mji_cross(added_mass_force.as_mut_ptr(), virtual_lin_mom.as_ptr(), ang_vel.as_ptr());
        mji_cross(added_mass_torque1.as_mut_ptr(), virtual_lin_mom.as_ptr(), lin_vel.as_ptr());
        mji_cross(added_mass_torque2.as_mut_ptr(), virtual_ang_mom.as_ptr(), ang_vel.as_ptr());

        mji_addTo3(local_force, added_mass_torque1.as_ptr());
        mji_addTo3(local_force, added_mass_torque2.as_ptr());
        mji_addTo3(local_force.add(3), added_mass_force.as_ptr());
    }
}

/// C: mj_viscousForces (engine/engine_passive.h:49)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_cross, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_passive.c:_mji_ellipsoid_max_moment, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_passive.c:_mji_pow2, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_passive.c:_mji_pow4, cxx:_mju_max, cxx:_mju_min, cxx:_mju_norm3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_viscousForces(local_vels: *const f64, fluid_density: f64, fluid_viscosity: f64, size: *const f64, magnus_lift_coef: f64, kutta_lift_coef: f64, blunt_drag_coef: f64, slender_drag_coef: f64, ang_drag_coef: f64, local_force: *mut f64) {
    use crate::engine::engine_inline::mji_cross;
    use crate::engine::engine_util_blas::mju_norm3;
    use crate::engine::engine_util_misc::{mju_max, mju_min};
    const MJ_MINVAL: f64 = 1E-15;

    // SAFETY: caller guarantees all pointers are valid and arrays are properly sized
    unsafe {
        let lin_vel: [f64; 3] = [*local_vels.add(3), *local_vels.add(4), *local_vels.add(5)];
        let ang_vel: [f64; 3] = [*local_vels.add(0), *local_vels.add(1), *local_vels.add(2)];
        let volume: f64 = 4.0 / 3.0 * std::f64::consts::PI * *size.add(0) * *size.add(1) * *size.add(2);
        let d_max: f64 = mju_max(mju_max(*size.add(0), *size.add(1)), *size.add(2));
        let d_min: f64 = mju_min(mju_min(*size.add(0), *size.add(1)), *size.add(2));
        let d_mid: f64 = *size.add(0) + *size.add(1) + *size.add(2) - d_max - d_min;
        let a_max: f64 = std::f64::consts::PI * d_max * d_mid;

        let mut magnus_force: [f64; 3] = [0.0; 3];
        mji_cross(magnus_force.as_mut_ptr(), ang_vel.as_ptr(), lin_vel.as_ptr());
        magnus_force[0] *= magnus_lift_coef * fluid_density * volume;
        magnus_force[1] *= magnus_lift_coef * fluid_density * volume;
        magnus_force[2] *= magnus_lift_coef * fluid_density * volume;

        // the dot product between velocity and the normal to the cross-section that
        // defines the body's projection along velocity is proj_num/sqrt(proj_denom)
        let proj_denom: f64 = mji_pow4(*size.add(1) * *size.add(2)) * mji_pow2(lin_vel[0])
                            + mji_pow4(*size.add(2) * *size.add(0)) * mji_pow2(lin_vel[1])
                            + mji_pow4(*size.add(0) * *size.add(1)) * mji_pow2(lin_vel[2]);
        let proj_num: f64 = mji_pow2(*size.add(1) * *size.add(2) * lin_vel[0])
                          + mji_pow2(*size.add(2) * *size.add(0) * lin_vel[1])
                          + mji_pow2(*size.add(0) * *size.add(1) * lin_vel[2]);

        // projected surface in the direction of the velocity
        let a_proj: f64 = std::f64::consts::PI * (proj_denom / mju_max(MJ_MINVAL, proj_num)).sqrt();

        // not-unit normal to ellipsoid's projected area in the direction of velocity
        let norm: [f64; 3] = [
            mji_pow2(*size.add(1) * *size.add(2)) * lin_vel[0],
            mji_pow2(*size.add(2) * *size.add(0)) * lin_vel[1],
            mji_pow2(*size.add(0) * *size.add(1)) * lin_vel[2],
        ];

        // cosine between velocity and normal to the surface
        // divided by proj_denom instead of sqrt(proj_denom) to account for skipped normalization in norm
        let cos_alpha: f64 = proj_num / mju_max(
            MJ_MINVAL, mju_norm3(lin_vel.as_ptr()) * proj_denom);
        let mut kutta_circ: [f64; 3] = [0.0; 3];
        mji_cross(kutta_circ.as_mut_ptr(), norm.as_ptr(), lin_vel.as_ptr());
        kutta_circ[0] *= kutta_lift_coef * fluid_density * cos_alpha * a_proj;
        kutta_circ[1] *= kutta_lift_coef * fluid_density * cos_alpha * a_proj;
        kutta_circ[2] *= kutta_lift_coef * fluid_density * cos_alpha * a_proj;
        let mut kutta_force: [f64; 3] = [0.0; 3];
        mji_cross(kutta_force.as_mut_ptr(), kutta_circ.as_ptr(), lin_vel.as_ptr());

        // viscous force and torque in Stokes flow, analytical for spherical bodies
        let eq_sphere_d: f64 = 2.0 / 3.0 * (*size.add(0) + *size.add(1) + *size.add(2));
        let lin_visc_force_coef: f64 = 3.0 * std::f64::consts::PI * eq_sphere_d;
        let lin_visc_torq_coef: f64 = std::f64::consts::PI * eq_sphere_d * eq_sphere_d * eq_sphere_d;

        // moments of inertia used to compute angular quadratic drag
        let i_max: f64 = 8.0 / 15.0 * std::f64::consts::PI * d_mid * mji_pow4(d_max);
        let ii: [f64; 3] = [
            mji_ellipsoid_max_moment(size, 0),
            mji_ellipsoid_max_moment(size, 1),
            mji_ellipsoid_max_moment(size, 2),
        ];
        let mom_visc: [f64; 3] = [
            ang_vel[0] * (ang_drag_coef * ii[0] + slender_drag_coef * (i_max - ii[0])),
            ang_vel[1] * (ang_drag_coef * ii[1] + slender_drag_coef * (i_max - ii[1])),
            ang_vel[2] * (ang_drag_coef * ii[2] + slender_drag_coef * (i_max - ii[2])),
        ];

        let drag_lin_coef: f64 =  // linear plus quadratic
            fluid_viscosity * lin_visc_force_coef + fluid_density * mju_norm3(lin_vel.as_ptr()) * (
                a_proj * blunt_drag_coef + slender_drag_coef * (a_max - a_proj));
        let drag_ang_coef: f64 =  // linear plus quadratic
            fluid_viscosity * lin_visc_torq_coef
            + fluid_density * mju_norm3(mom_visc.as_ptr());

        *local_force.add(0) -= drag_ang_coef * ang_vel[0];
        *local_force.add(1) -= drag_ang_coef * ang_vel[1];
        *local_force.add(2) -= drag_ang_coef * ang_vel[2];
        *local_force.add(3) += magnus_force[0] + kutta_force[0] - drag_lin_coef * lin_vel[0];
        *local_force.add(4) += magnus_force[1] + kutta_force[1] - drag_lin_coef * lin_vel[1];
        *local_force.add(5) += magnus_force[2] + kutta_force[2] - drag_lin_coef * lin_vel[2];
    }
}

/// C: readFluidGeomInteraction (engine/engine_passive.h:56)
/// Calls: cxx:_mju_message
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn readFluidGeomInteraction(geom_fluid_coefs: *const f64, geom_fluid_coef: *mut f64, blunt_drag_coef: *mut f64, slender_drag_coef: *mut f64, ang_drag_coef: *mut f64, kutta_lift_coef: *mut f64, magnus_lift_coef: *mut f64, virtual_mass: *mut f64, virtual_inertia: *mut f64) {
    const MJ_NFLUID: usize = 12;
    // SAFETY: caller guarantees all pointers are valid and arrays are properly sized
    unsafe {
        let mut i: usize = 0;
        *geom_fluid_coef       = *geom_fluid_coefs.add(i); i += 1;
        *blunt_drag_coef       = *geom_fluid_coefs.add(i); i += 1;
        *slender_drag_coef     = *geom_fluid_coefs.add(i); i += 1;
        *ang_drag_coef         = *geom_fluid_coefs.add(i); i += 1;
        *kutta_lift_coef       = *geom_fluid_coefs.add(i); i += 1;
        *magnus_lift_coef      = *geom_fluid_coefs.add(i); i += 1;
        *virtual_mass.add(0)   = *geom_fluid_coefs.add(i); i += 1;
        *virtual_mass.add(1)   = *geom_fluid_coefs.add(i); i += 1;
        *virtual_mass.add(2)   = *geom_fluid_coefs.add(i); i += 1;
        *virtual_inertia.add(0) = *geom_fluid_coefs.add(i); i += 1;
        *virtual_inertia.add(1) = *geom_fluid_coefs.add(i); i += 1;
        *virtual_inertia.add(2) = *geom_fluid_coefs.add(i); i += 1;
        debug_assert_eq!(i, MJ_NFLUID);
    }
}

/// C: writeFluidGeomInteraction (engine/engine_passive.h:66)
/// Calls: cxx:_mju_message
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn writeFluidGeomInteraction(geom_fluid_coefs: *mut f64, geom_fluid_coef: *const f64, blunt_drag_coef: *const f64, slender_drag_coef: *const f64, ang_drag_coef: *const f64, kutta_lift_coef: *const f64, magnus_lift_coef: *const f64, virtual_mass: *const f64, virtual_inertia: *const f64) {
    const MJ_NFLUID: usize = 12;
    // SAFETY: caller guarantees all pointers are valid and arrays are properly sized
    unsafe {
        let mut i: usize = 0;
        *geom_fluid_coefs.add(i) = *geom_fluid_coef;       i += 1;
        *geom_fluid_coefs.add(i) = *blunt_drag_coef;       i += 1;
        *geom_fluid_coefs.add(i) = *slender_drag_coef;     i += 1;
        *geom_fluid_coefs.add(i) = *ang_drag_coef;         i += 1;
        *geom_fluid_coefs.add(i) = *kutta_lift_coef;       i += 1;
        *geom_fluid_coefs.add(i) = *magnus_lift_coef;      i += 1;
        *geom_fluid_coefs.add(i) = *virtual_mass.add(0);   i += 1;
        *geom_fluid_coefs.add(i) = *virtual_mass.add(1);   i += 1;
        *geom_fluid_coefs.add(i) = *virtual_mass.add(2);   i += 1;
        *geom_fluid_coefs.add(i) = *virtual_inertia.add(0); i += 1;
        *geom_fluid_coefs.add(i) = *virtual_inertia.add(1); i += 1;
        *geom_fluid_coefs.add(i) = *virtual_inertia.add(2); i += 1;
        debug_assert_eq!(i, MJ_NFLUID);
    }
}

