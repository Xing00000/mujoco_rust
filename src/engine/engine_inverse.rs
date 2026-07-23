//! Port of: engine/engine_inverse.c
//! IR hash: 3fb6da908ad9d71c
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mj_discreteAcc (engine/engine_inverse.c:84)
/// Calls: mj_actuatorDamping, mj_freeStack, mj_markStack, mj_mulM, mj_solveM, mj_stackAllocInfo, mjd_smooth_vel, mjd_xPolyForce, mju_addScl, mju_addToScl, mju_copy, mju_gather, mju_gatherMasked, mju_isZero, mju_message, mju_mulMatVecSparse, mju_mulSymVecSparse
#[allow(unused_variables, non_snake_case)]
pub fn mj_discrete_acc(m: *const mjModel, d: *mut mjData) {
    const MJ_INT_EULER: i32 = 0;
    const MJ_INT_RK4: i32 = 1;
    const MJ_INT_IMPLICIT: i32 = 2;
    const MJ_INT_IMPLICITFAST: i32 = 3;
    const MJ_DSBL_EULERDAMP: i32 = 1 << 15;
    const MJ_NPOLY: i32 = 2;
    const MJ_OBJ_JOINT: u32 = 3;

    // SAFETY: m, d are valid pointers (caller contract)
    unsafe {
        let nv = (*m).nv as i32;
        let nC = (*m).nC as i32;
        let nD = (*m).nD as i32;
        let qacc = (*d).qacc;

        crate::engine::engine_memory::mj_mark_stack(d);
        let qfrc = crate::engine::engine_memory::mj_stack_alloc_num(d, nv as usize);

        // use selected integrator
        match (*m).opt.integrator {
            MJ_INT_RK4 => {
                // not supported by RK4
                crate::engine::engine_util_errmem::mju_error(
                    b"discrete inverse dynamics is not supported by RK4 integrator\0".as_ptr() as *const i8,
                );
                crate::engine::engine_memory::mj_free_stack(d);
                return;
            }

            MJ_INT_EULER => {
                // check for dof damping if disable flag is not set
                let mut dof_damping = 0;
                if ((*m).opt.disableflags & MJ_DSBL_EULERDAMP) == 0 {
                    for i in 0..nv {
                        if *(*m).dof_damping.add(i as usize) > 0.0
                            || crate::engine::engine_util_misc::mju_is_zero(
                                (*m).dof_dampingpoly.add((MJ_NPOLY * i) as usize), MJ_NPOLY) == 0
                            || *(*m).jnt_actuatorid.add(*(*m).dof_jntid.add(i as usize) as usize) != -1
                        {
                            dof_damping = 1;
                            break;
                        }
                    }
                }

                // if disabled or no dof damping, nothing to do
                if dof_damping == 0 {
                    crate::engine::engine_memory::mj_free_stack(d);
                    return;
                }

                // set qfrc = (M + h*diag(B)) * qacc
                crate::engine::engine_support::mj_mul_m(m, d as *const crate::types::mjData, qfrc, qacc);
                for i in 0..nv {
                    let v = *(*d).qvel.add(i as usize);
                    let mut poly: [f64; 2] = [0.0; 2];
                    crate::engine::engine_util_blas::mju_copy(
                        poly.as_mut_ptr(), (*m).dof_dampingpoly.add((MJ_NPOLY * i) as usize), MJ_NPOLY,
                    );
                    let damping = *(*m).dof_damping.add(i as usize)
                        + crate::engine::engine_core_util::mj_actuator_damping(
                            m, MJ_OBJ_JOINT, *(*m).dof_jntid.add(i as usize), poly.as_mut_ptr(),
                        );
                    let damp_deriv = crate::engine::engine_util_misc::mjd_x_poly_force(
                        damping, poly.as_ptr(), v, MJ_NPOLY, 1,
                    );
                    *qfrc.add(i as usize) += (*m).opt.timestep * damp_deriv * *qacc.add(i as usize);
                }
            }

            MJ_INT_IMPLICIT => {
                // compute qDeriv
                crate::engine::engine_derivative::mjd_smooth_vel(m, d, 1);

                // gather qLU <- qM (lower to full)
                crate::engine::engine_util_misc::mju_gather_masked(
                    (*d).qLU, (*d).M, (*m).mapM2D, nD,
                );

                // set qLU = qM - dt*qDeriv
                crate::engine::engine_util_blas::mju_add_to_scl(
                    (*d).qLU, (*d).qDeriv, -(*m).opt.timestep, nD,
                );

                // set qfrc = qLU * qacc
                crate::engine::engine_util_sparse::mju_mul_mat_vec_sparse(
                    qfrc, (*d).qLU, qacc, nv,
                    (*m).D_rownnz, (*m).D_rowadr, (*m).D_colind, std::ptr::null(),
                );
            }

            MJ_INT_IMPLICITFAST => {
                // compute analytical derivative qDeriv; skip rne derivative
                crate::engine::engine_derivative::mjd_smooth_vel(m, d, 0);

                // save mass matrix
                let Msave = crate::engine::engine_memory::mj_stack_alloc_num(d, nC as usize);
                crate::engine::engine_util_blas::mju_copy(Msave, (*d).M, nC);

                // modified mass matrix: gather qH <- qDeriv (full to lower)
                crate::engine::engine_util_misc::mju_gather(
                    (*d).qH, (*d).qDeriv, (*m).mapD2M, nC,
                );

                // set qH = M - dt*qDeriv
                crate::engine::engine_util_blas::mju_add_scl(
                    (*d).qH, (*d).M, (*d).qH, -(*m).opt.timestep, nC,
                );

                // set qfrc = (M - dt*qDeriv) * qacc
                crate::engine::engine_util_sparse::mju_mul_sym_vec_sparse(
                    qfrc, (*d).qH, qacc, nv,
                    (*m).M_rownnz, (*m).M_rowadr, (*m).M_colind,
                );
            }

            _ => {}
        }

        // solve for qacc: qfrc = M * qacc
        crate::engine::engine_core_smooth::mj_solve_m(m, d, qacc, qfrc, 1);

        crate::engine::engine_memory::mj_free_stack(d);
    }
}

/// C: mj_inverse (engine/engine_inverse.h:27)
/// Calls: mj_inverseSkip
#[allow(unused_variables, non_snake_case)]
pub fn mj_inverse(m: *const mjModel, d: *mut mjData) {
    mj_inverse_skip(m, d, 0, 0); // mjSTAGE_NONE = 0
}

/// C: mj_inverseSkip (engine/engine_inverse.h:30)
/// Calls: mj_discreteAcc, mj_energyPos, mj_energyVel, mj_freeStack, mj_invConstraint, mj_invPosition, mj_invVelocity, mj_markStack, mj_mulM, mj_rne, mj_sensorAcc, mj_sensorPos, mj_sensorVel, mj_stackAllocInfo, mj_tendonBias, mju_copy
#[allow(unused_variables, non_snake_case)]
pub fn mj_inverse_skip(m: *const mjModel, d: *mut mjData, skipstage: i32, skipsensor: i32) {
    const MJ_STAGE_POS: i32 = 1;
    const MJ_STAGE_VEL: i32 = 2;
    const MJ_ENBL_ENERGY: i32 = 1 << 1;
    const MJ_ENBL_INVDISCRETE: i32 = 1 << 3;

    // SAFETY: m, d are valid pointers (caller contract).
    unsafe {
        crate::engine::engine_memory::mj_mark_stack(d);
        let nv = (*m).nv as i32;

        // position-dependent
        if skipstage < MJ_STAGE_POS {
            mj_inv_position(m, d);
            if skipsensor == 0 {
                crate::engine::engine_sensor::mj_sensor_pos(m, d);
            }
            if ((*m).opt.enableflags & MJ_ENBL_ENERGY) != 0 && !(*d).flg_energypos {
                crate::engine::engine_sensor::mj_energy_pos(m, d);
            }
        }

        // velocity-dependent
        if skipstage < MJ_STAGE_VEL {
            mj_inv_velocity(m, d);
            if skipsensor == 0 {
                crate::engine::engine_sensor::mj_sensor_vel(m, d);
            }
            if ((*m).opt.enableflags & MJ_ENBL_ENERGY) != 0 && !(*d).flg_energyvel {
                crate::engine::engine_sensor::mj_energy_vel(m, d);
            }
        }

        let mut qacc_saved: *mut f64 = std::ptr::null_mut();
        if ((*m).opt.enableflags & MJ_ENBL_INVDISCRETE) != 0 {
            // save current qacc
            qacc_saved = crate::engine::engine_memory::mj_stack_alloc_num(d, nv as usize);
            crate::engine::engine_util_blas::mju_copy(qacc_saved, (*d).qacc, nv);

            // modify qacc in-place
            mj_discrete_acc(m, d);
        }

        // acceleration-dependent
        mj_inv_constraint(m, d);

        // sum of bias forces in qfrc_inverse = centripetal + Coriolis + tendon bias
        crate::engine::engine_core_smooth::mj_rne(m, d, 0, (*d).qfrc_inverse);
        crate::engine::engine_core_smooth::mj_tendon_bias(m, d, (*d).qfrc_inverse);

        if skipsensor == 0 {
            (*d).flg_rnepost = false;
            crate::engine::engine_sensor::mj_sensor_acc(m, d);
        }

        // compute Ma = M*qacc
        let Ma = crate::engine::engine_memory::mj_stack_alloc_num(d, nv as usize);
        crate::engine::engine_support::mj_mul_m(m, d as *const mjData, Ma, (*d).qacc);

        // qfrc_inverse += Ma - qfrc_passive - qfrc_constraint
        for i in 0..nv {
            *(*d).qfrc_inverse.add(i as usize) +=
                *Ma.add(i as usize)
                - *(*d).qfrc_passive.add(i as usize)
                - *(*d).qfrc_constraint.add(i as usize);
        }

        if ((*m).opt.enableflags & MJ_ENBL_INVDISCRETE) != 0 {
            // restore qacc
            crate::engine::engine_util_blas::mju_copy((*d).qacc, qacc_saved, nv);
        }

        crate::engine::engine_memory::mj_free_stack(d);
    }
}

/// C: mj_invPosition (engine/engine_inverse.h:34)
/// Calls: mj_camlight, mj_collision, mj_comPos, mj_factorM, mj_flex, mj_kinematics, mj_makeConstraint, mj_makeM, mj_projectConstraint, mj_tendon, mj_transmission
#[allow(unused_variables, non_snake_case)]
pub fn mj_inv_position(m: *const mjModel, d: *mut mjData) {
    const MJ_ENBL_DIAGEXACT: i32 = 1 << 5;

    // SAFETY: m, d are valid pointers (caller contract).
    unsafe {
        // clear flag for lazy evaluation
        (*d).flg_energypos = false;

        crate::engine::engine_core_smooth::mj_kinematics(m, d);
        crate::engine::engine_core_smooth::mj_com_pos(m, d);
        crate::engine::engine_core_smooth::mj_camlight(m, d);
        crate::engine::engine_core_smooth::mj_flex(m, d);
        crate::engine::engine_core_smooth::mj_tendon(m, d);

        crate::engine::engine_core_smooth::mj_make_m(m, d);
        crate::engine::engine_core_smooth::mj_factor_m(m, d);

        crate::engine::engine_collision_driver::mj_collision(m, d);

        crate::engine::engine_core_constraint::mj_make_constraint(m, d);

        // compute exact diagonal if enabled
        if ((*m).opt.enableflags & MJ_ENBL_DIAGEXACT) != 0 {
            crate::engine::engine_core_constraint::mj_project_constraint(m, d);
        }

        crate::engine::engine_core_smooth::mj_transmission(m, d);
    }
}

/// C: mj_invVelocity (engine/engine_inverse.h:37)
/// Calls: mj_fwdVelocity
#[allow(unused_variables, non_snake_case)]
pub fn mj_inv_velocity(m: *const mjModel, d: *mut mjData) {
    crate::engine::engine_forward::mj_fwd_velocity(m, d);
}

/// C: mj_invConstraint (engine/engine_inverse.h:40)
/// Calls: mj_constraintUpdate, mj_freeStack, mj_markStack, mj_mulJacVec, mj_stackAllocInfo, mju_subFrom, mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mj_inv_constraint(m: *const mjModel, d: *mut mjData) {
    // SAFETY: m, d are valid pointers (caller contract).
    unsafe {
        let nefc = (*d).nefc;

        // no constraints: clear, return
        if nefc == 0 {
            crate::engine::engine_util_blas::mju_zero((*d).qfrc_constraint, (*m).nv as i32);
            return;
        }

        crate::engine::engine_memory::mj_mark_stack(d);
        let jar = crate::engine::engine_memory::mj_stack_alloc_num(d, nefc as usize);

        // compute jar = Jac*qacc - aref
        crate::engine::engine_core_constraint::mj_mul_jac_vec(
            m, d as *const mjData, jar, (*d).qacc);
        crate::engine::engine_util_blas::mju_sub_from(jar, (*d).efc_aref, nefc);

        // call update function
        crate::engine::engine_core_constraint::mj_constraint_update(
            m, d, jar, std::ptr::null_mut(), 0);

        crate::engine::engine_memory::mj_free_stack(d);
    }
}

/// C: mj_compareFwdInv (engine/engine_inverse.h:43)
/// Calls: mj_freeStack, mj_inverseSkip, mj_markStack, mj_stackAllocInfo, mj_xfrcAccumulate, mju_add, mju_copy, mju_norm, mju_sub
#[allow(unused_variables, non_snake_case)]
pub fn mj_compare_fwd_inv(m: *const mjModel, d: *mut mjData) {
    const MJ_STAGE_VEL: i32 = 2;

    // SAFETY: m, d are valid pointers (caller contract).
    unsafe {
        let nv = (*m).nv as i32;
        let nefc = (*d).nefc;

        // clear result, return if no constraints
        (*d).solver_fwdinv[0] = 0.0;
        (*d).solver_fwdinv[1] = 0.0;
        if nefc == 0 {
            return;
        }

        // allocate
        crate::engine::engine_memory::mj_mark_stack(d);
        let qforce = crate::engine::engine_memory::mj_stack_alloc_num(d, nv as usize);
        let dif = crate::engine::engine_memory::mj_stack_alloc_num(d, nv as usize);
        let save_qfrc_constraint = crate::engine::engine_memory::mj_stack_alloc_num(d, nv as usize);
        let save_efc_force = crate::engine::engine_memory::mj_stack_alloc_num(d, nefc as usize);

        // qforce = qfrc_applied + J'*xfrc_applied + qfrc_actuator
        crate::engine::engine_util_blas::mju_add(
            qforce, (*d).qfrc_applied, (*d).qfrc_actuator, nv);
        crate::engine::engine_support::mj_xfrc_accumulate(m, d, qforce);

        // save forward dynamics results
        crate::engine::engine_util_blas::mju_copy(save_qfrc_constraint, (*d).qfrc_constraint, nv);
        crate::engine::engine_util_blas::mju_copy(save_efc_force, (*d).efc_force, nefc);

        // run inverse dynamics, do not update position and velocity
        mj_inverse_skip(m, d, MJ_STAGE_VEL, 1);

        // compute statistics
        crate::engine::engine_util_blas::mju_sub(dif, save_qfrc_constraint, (*d).qfrc_constraint, nv);
        (*d).solver_fwdinv[0] = crate::engine::engine_util_blas::mju_norm(dif, nv);
        crate::engine::engine_util_blas::mju_sub(dif, qforce, (*d).qfrc_inverse, nv);
        (*d).solver_fwdinv[1] = crate::engine::engine_util_blas::mju_norm(dif, nv);

        // restore forward dynamics results
        crate::engine::engine_util_blas::mju_copy((*d).qfrc_constraint, save_qfrc_constraint, nv);
        crate::engine::engine_util_blas::mju_copy((*d).efc_force, save_efc_force, nefc);

        crate::engine::engine_memory::mj_free_stack(d);
    }
}

