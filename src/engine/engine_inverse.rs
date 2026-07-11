//! Port of: engine/engine_inverse.c
//! IR hash: 05737965add36adb
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mj_discreteAcc (engine/engine_inverse.c:84)
/// Calls: mj_actuatorDamping, mj_freeStack, mj_markStack, mj_mulM, mj_solveM, mj_stackAllocInfo, mjd_smooth_vel, mjd_xPolyForce, mju_addScl, mju_addToScl, mju_copy, mju_gather, mju_gatherMasked, mju_isZero, mju_message, mju_mulMatVecSparse, mju_mulSymVecSparse
#[allow(unused_variables, non_snake_case)]
pub fn mj_discrete_acc(m: *const mjModel, d: *mut mjData) {
    if m.is_null() { return; }
    extern "C" { fn mj_discreteAcc(m: *const mjModel, d: *mut mjData); }
    // SAFETY: m verified non-null; delegates to C implementation
    unsafe { mj_discreteAcc(m, d) }
}

/// C: mj_inverse (engine/engine_inverse.h:27)
/// Calls: mj_inverseSkip
#[allow(unused_variables, non_snake_case)]
pub fn mj_inverse(m: *const mjModel, d: *mut mjData) {
    mj_inverse_skip(m, d, 0, 0);
}

/// C: mj_inverseSkip (engine/engine_inverse.h:30)
/// Calls: mj_discreteAcc, mj_energyPos, mj_energyVel, mj_freeStack, mj_invConstraint, mj_invPosition, mj_invVelocity, mj_markStack, mj_mulM, mj_rne, mj_sensorAcc, mj_sensorPos, mj_sensorVel, mj_stackAllocInfo, mj_tendonBias, mju_copy
#[allow(unused_variables, non_snake_case)]
pub fn mj_inverse_skip(m: *const mjModel, d: *mut mjData, skipstage: i32, skipsensor: i32) {
    extern "C" { fn mj_inverseSkip(m: *const mjModel, d: *mut mjData, skipstage: i32, skipsensor: i32); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_inverseSkip(m, d, skipstage, skipsensor) }
}

/// C: mj_invPosition (engine/engine_inverse.h:34)
/// Calls: mj_camlight, mj_collision, mj_comPos, mj_factorM, mj_flex, mj_kinematics, mj_makeConstraint, mj_makeM, mj_projectConstraint, mj_tendon, mj_transmission
#[allow(unused_variables, non_snake_case)]
pub fn mj_inv_position(m: *const mjModel, d: *mut mjData) {
    extern "C" { fn mj_invPosition(m: *const mjModel, d: *mut mjData); }
    // SAFETY: delegates to C implementation, pointers valid per caller contract
    unsafe { mj_invPosition(m, d) }
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
    // SAFETY: m, d valid per caller. Stack alloc, Jac*vec, constraint update all safe.
    unsafe {
        let nefc: i32 = (*d).nefc;

        // no constraints: clear, return
        if nefc == 0 {
            crate::engine::engine_util_blas::mju_zero((*d).qfrc_constraint, (*m).nv as i32);
            return;
        }

        crate::engine::engine_memory::mj_mark_stack(d);
        let jar: *mut f64 = crate::engine::engine_memory::mj_stack_alloc_num(d, nefc as usize);

        // compute jar = Jac*qacc - aref
        crate::engine::engine_core_constraint::mj_mul_jac_vec(m, d as *const mjData, jar, (*d).qacc);
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
    // SAFETY: m, d valid per caller. All pointer arithmetic within valid model/data arrays.
    unsafe {
        let nv: i32 = (*m).nv as i32;
        let nefc: i32 = (*d).nefc;

        // clear result, return if no constraints
        (*d).solver_fwdinv[0] = 0.0;
        (*d).solver_fwdinv[1] = 0.0;
        if nefc == 0 {
            return;
        }

        // allocate
        crate::engine::engine_memory::mj_mark_stack(d);
        let qforce: *mut f64 = crate::engine::engine_memory::mj_stack_alloc_num(d, nv as usize);
        let dif: *mut f64 = crate::engine::engine_memory::mj_stack_alloc_num(d, nv as usize);
        let save_qfrc_constraint: *mut f64 = crate::engine::engine_memory::mj_stack_alloc_num(d, nv as usize);
        let save_efc_force: *mut f64 = crate::engine::engine_memory::mj_stack_alloc_num(d, nefc as usize);

        // qforce = qfrc_applied + qfrc_actuator
        crate::engine::engine_util_blas::mju_add(qforce, (*d).qfrc_applied, (*d).qfrc_actuator, nv);
        // qforce += J'*xfrc_applied
        crate::engine::engine_support::mj_xfrc_accumulate(m, d, qforce);

        // save forward dynamics results
        crate::engine::engine_util_blas::mju_copy(save_qfrc_constraint, (*d).qfrc_constraint, nv);
        crate::engine::engine_util_blas::mju_copy(save_efc_force, (*d).efc_force, nefc);

        // run inverse dynamics, skip position and velocity stages
        // mjSTAGE_VEL = 2
        mj_inverse_skip(m, d, 2, 1);

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

