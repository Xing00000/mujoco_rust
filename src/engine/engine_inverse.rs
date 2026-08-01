//! Port of: engine/engine_inverse.h
//! IR hash: 9343293228317031
//! CODEGEN: source paths, owners, and callable names are locked.

use crate::types::*;

/// C: mj_invConstraint (engine/engine_inverse.h:40)
/// Calls: cxx:_mj_constraintUpdate, cxx:_mj_freeStack, cxx:_mj_markStack, cxx:_mj_mulJacVec, cxx:_mj_stackAllocInfo, cxx:_mju_subFrom, cxx:_mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mj_invConstraint(m: *const mjModel, d: *mut mjData) {
    // SAFETY: m, d are valid pointers (caller contract).
    unsafe {
        let nefc = (*d).nefc;

        // no constraints: clear, return
        if nefc == 0 {
            crate::engine::engine_util_blas::mju_zero((*d).qfrc_constraint, (*m).nv as i32);
            return;
        }

        crate::engine::engine_memory::mj_markStack(d);
        let jar = crate::engine::engine_memory::mj_stackAllocNum(d, nefc as usize);

        // compute jar = Jac*qacc - aref
        crate::engine::engine_core_constraint::mj_mulJacVec(
            m, d as *const mjData, jar, (*d).qacc);
        crate::engine::engine_util_blas::mju_subFrom(jar, (*d).efc_aref, nefc);

        // call update function
        crate::engine::engine_core_constraint::mj_constraintUpdate(
            m, d, jar, std::ptr::null_mut(), 0);

        crate::engine::engine_memory::mj_freeStack(d);
    }
}

