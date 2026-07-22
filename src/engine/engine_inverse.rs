//! Port of: engine/engine_inverse.c
//! IR hash: 73a9f665ec0ecfc0
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

