//! Port of: engine/engine_core_constraint.c
//! IR hash: fd5288ad7242f001
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: cell_pos_and_jac (engine/engine_core_constraint.c:55)
/// Calls: cxx:_mj_bodyChain, cxx:_mj_jacSparse, cxx:_mj_stackAllocInfo, cxx:_mju_zero, cxx:_mju_zeroInt
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn cell_pos_and_jac(m: *const mjModel, d: *mut mjData, flex_id: i32, npc: i32, gindices: *const i32, nv: i32, xpos_c: *const f64, cell_chain: *mut i32, cell_nnz: *mut i32) -> *mut f64 {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_constraint.c:_cell_pos_and_jac
}

/// C: cell_strain_jacobian (engine/engine_core_constraint.c:111)
/// Calls: cxx:_mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn cell_strain_jacobian(npc: i32, cell_nnz: i32, dSdx_local: *const f64, cell_node_jac: *const f64, strain_jac: *mut f64) {
    // SAFETY: caller guarantees valid pointers with sufficient lengths:
    //   dSdx_local[3*npc], cell_node_jac[3*npc * cell_nnz], strain_jac[cell_nnz]
    unsafe {
        crate::engine::engine_util_blas::mju_zero(strain_jac, cell_nnz);
        for n in 0..npc {
            for c in 0..3 {
                let w = *dSdx_local.add((3 * n + c) as usize);
                if w == 0.0 {
                    continue;
                }
                let row = (3 * n + c) as usize;
                for k in 0..cell_nnz as usize {
                    *strain_jac.add(k) += w * *cell_node_jac.add(row * cell_nnz as usize + k);
                }
            }
        }
    }
}

/// C: arenaAllocEfc (engine/engine_core_constraint.c:130)
/// Calls: cxx:_mj_arenaAllocByte, cxx:_mj_warning
#[allow(unused_variables, non_snake_case)]
pub fn arena_alloc_efc(m: *const mjModel, d: *mut mjData) -> i32 {
    todo!() // cxx-internal:engine_core_constraint.c.o:_arenaAllocEfc
}

/// C: mj_isDual (engine/engine_core_constraint.c:167)
#[allow(unused_variables, non_snake_case)]
pub fn mj_is_dual(m: *const mjModel) -> i32 {
    todo!() // cxx:_mj_isDual
}

/// C: mj_assignFriction (engine/engine_core_constraint.c:177)
/// Calls: cxx:_mju_max
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_assign_friction(m: *const mjModel, target: *mut f64, source: *const f64) {
    todo!() // cxx:_mj_assignFriction
}

/// C: mj_assignRef (engine/engine_core_constraint.c:192)
/// Calls: cxx:_mju_copy
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_assign_ref(m: *const mjModel, target: *mut f64, source: *const f64) {
    todo!() // cxx:_mj_assignRef
}

/// C: mj_assignImp (engine/engine_core_constraint.c:202)
/// Calls: cxx:_mju_copy
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_assign_imp(m: *const mjModel, target: *mut f64, source: *const f64) {
    todo!() // cxx:_mj_assignImp
}

/// C: mj_assignMargin (engine/engine_core_constraint.c:212)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_assign_margin(m: *const mjModel, source: f64) -> f64 {
    todo!() // cxx:_mj_assignMargin
}

/// C: mj_elemBodyWeight (engine/engine_core_constraint.c:223)
/// Calls: cxx:_mju_dist3, cxx:_mju_max, cxx:_mju_message, cxx:_mju_scl, cxx:_mju_sum
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_elem_body_weight(m: *const mjModel, d: *const mjData, f: i32, e: i32, v: i32, point: *const f64, body: *mut i32, weight: *mut f64) -> i32 {
    todo!() // cxx-internal:engine_core_constraint.c.o:_mj_elemBodyWeight
}

/// C: mj_vertBodyWeight (engine/engine_core_constraint.c:265)
/// Calls: cxx:_mju_addToScl3, cxx:_mju_cellLookup, cxx:_mju_evalBasis, cxx:_mju_evalBasisArray, cxx:_mju_shellTFIWeights
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_vert_body_weight(m: *const mjModel, d: *const mjData, f: i32, v: *mut i32, body: *mut i32, bweight: *mut f64, vweight: *const f64, nw: i32) -> i32 {
    todo!() // cxx-internal:engine_core_constraint.c.o:_mj_vertBodyWeight
}

/// C: mj_addContact (engine/engine_core_constraint.c:390)
/// Calls: cxx:_mj_arenaAllocByte, cxx:_mj_warning
#[allow(unused_variables, non_snake_case)]
pub fn mj_add_contact(m: *const mjModel, d: *mut mjData, con: *const mjContact) -> i32 {
    todo!() // cxx:_mj_addContact
}

/// C: mj_addConstraint (engine/engine_core_constraint.c:414)
/// Calls: cxx:_mj_isSparse, cxx:_mju_copy, cxx:_mju_copyInt, cxx:_mju_message
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_add_constraint(m: *const mjModel, d: *mut mjData, jac: *const f64, pos: *const f64, margin: *const f64, frictionloss: f64, size: i32, r#type: i32, id: i32, NV: i32, chain: *const i32) {
    todo!() // cxx-internal:engine_core_constraint.c.o:_mj_addConstraint
}

/// C: mj_mulJacVec (engine/engine_core_constraint.c:509)
/// Calls: cxx:_mj_isSparse, cxx:_mju_mulMatVec, cxx:_mju_mulMatVecSparse
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_mul_jac_vec(m: *const mjModel, d: *const mjData, res: *mut f64, vec: *const f64) {
    todo!() // cxx:_mj_mulJacVec
}

/// C: mj_mulJacTVec (engine/engine_core_constraint.c:529)
/// Calls: cxx:_mj_isSparse, cxx:_mju_mulMatTVec, cxx:_mju_mulMatTVecSparse
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_mul_jac_t_vec(m: *const mjModel, d: *const mjData, res: *mut f64, vec: *const f64) {
    todo!() // cxx:_mj_mulJacTVec
}

/// C: mj_equalityAnchors (engine/engine_core_constraint.c:549)
/// Calls: cxx:_mju_addTo3, cxx:_mju_copy3, cxx:_mju_mulMatVec3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_equality_anchors(m: *const mjModel, d: *const mjData, eq_id: i32, pos1: *mut f64, pos2: *mut f64, body1: *mut i32, body2: *mut i32) {
    todo!() // cxx-internal:engine_core_constraint.c.o:_mj_equalityAnchors
}

/// C: mj_instantiateEquality (engine/engine_core_constraint.c:584)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_constraint.c:_cell_pos_and_jac, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_constraint.c:_cell_strain_jacobian, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_sparse.h:_mju_combineSparse, cxx-internal:engine_core_constraint.c.o:_mj_addConstraint, cxx-internal:engine_core_constraint.c.o:_mj_equalityAnchors, cxx:_mj_freeStack, cxx:_mj_isSparse, cxx:_mj_jacDifPair, cxx:_mj_markStack, cxx:_mj_sleepState, cxx:_mj_stackAllocInfo, cxx:_mju_addTo3, cxx:_mju_addToScl, cxx:_mju_copy, cxx:_mju_copy3, cxx:_mju_copyInt, cxx:_mju_defGradient, cxx:_mju_flexGatherCellState, cxx:_mju_flexGatherFaceState, cxx:_mju_flexInterpRotation2D, cxx:_mju_mat2Rot, cxx:_mju_message, cxx:_mju_mulMatVec3, cxx:_mju_mulQuat, cxx:_mju_mulQuatAxis, cxx:_mju_negQuat, cxx:_mju_rotVecQuat, cxx:_mju_scl, cxx:_mju_scl3, cxx:_mju_sparse2dense, cxx:_mju_sub3, cxx:_mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mj_instantiate_equality(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx:_mj_instantiateEquality
}

/// C: mj_Jdotv (engine/engine_core_constraint.c:1044)
/// Calls: cxx-internal:engine_core_constraint.c.o:_mj_equalityAnchors, cxx:_mj_freeStack, cxx:_mj_isSparse, cxx:_mj_jacDot, cxx:_mj_jacDotSparse, cxx:_mj_markStack, cxx:_mj_mergeChain, cxx:_mj_stackAllocInfo, cxx:_mju_copy4, cxx:_mju_derivQuat, cxx:_mju_dotSparseX3, cxx:_mju_mulMatVec, cxx:_mju_mulQuat, cxx:_mju_mulQuatAxis, cxx:_mju_negQuat, cxx:_mju_sub3, cxx:_mju_zero3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_jdotv(m: *const mjModel, d: *mut mjData, result: *mut f64) {
    todo!() // cxx:_mj_Jdotv
}

/// C: mj_addConstraintCount (engine/engine_core_constraint.c:1247)
/// Calls: cxx:_mj_isSparse
#[allow(unused_variables, non_snake_case)]
pub fn mj_add_constraint_count(m: *const mjModel, size: i32, NV: i32) -> i32 {
    todo!() // cxx-internal:engine_core_constraint.c.o:_mj_addConstraintCount
}

/// C: mj_instantiateFriction (engine/engine_core_constraint.c:1258)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_constraint.c:_mj_addConstraintCount, cxx-internal:engine_core_constraint.c.o:_mj_addConstraint, cxx:_mj_freeStack, cxx:_mj_isSparse, cxx:_mj_markStack, cxx:_mj_sleepState, cxx:_mj_stackAllocInfo, cxx:_mju_sparse2dense, cxx:_mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mj_instantiate_friction(m: *const mjModel, d: *mut mjData, count_only: i32, nnz: *mut i32) -> i32 {
    todo!() // cxx-internal:engine_core_constraint.c.o:_mj_instantiateFriction
}

/// C: mj_instantiateLimit (engine/engine_core_constraint.c:1348)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_constraint.c:_mj_addConstraintCount, cxx-internal:engine_core_constraint.c.o:_mj_addConstraint, cxx:_mj_freeStack, cxx:_mj_isSparse, cxx:_mj_markStack, cxx:_mj_sleepState, cxx:_mj_stackAllocInfo, cxx:_mju_max, cxx:_mju_normalize3, cxx:_mju_normalize4, cxx:_mju_quat2Vel, cxx:_mju_scl, cxx:_mju_scl3, cxx:_mju_sparse2dense, cxx:_mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mj_instantiate_limit(m: *const mjModel, d: *mut mjData, count_only: i32, nnz: *mut i32) -> i32 {
    todo!() // cxx-internal:engine_core_constraint.c.o:_mj_instantiateLimit
}

/// C: mj_contactJacobian (engine/engine_core_constraint.c:1526)
/// Calls: cxx-internal:engine_core_constraint.c.o:_mj_elemBodyWeight, cxx-internal:engine_core_constraint.c.o:_mj_vertBodyWeight, cxx:_mj_isSparse, cxx:_mj_jacDifPair, cxx:_mj_jacSum, cxx:_mju_scl
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_contact_jacobian(m: *const mjModel, d: *mut mjData, con: *const mjContact, dim: i32, jacdifp: *mut f64, jacdifr: *mut f64, jac1p: *mut f64, jac2p: *mut f64, jac1r: *mut f64, jac2r: *mut f64, chain: *mut i32) -> i32 {
    todo!() // cxx:_mj_contactJacobian
}

/// C: mj_instantiateContact (engine/engine_core_constraint.c:1605)
/// Calls: cxx-internal:engine_core_constraint.c.o:_mj_addConstraint, cxx:_mj_contactJacobian, cxx:_mj_freeStack, cxx:_mj_isPyramidal, cxx:_mj_isSparse, cxx:_mj_markStack, cxx:_mj_stackAllocInfo, cxx:_mju_addScl, cxx:_mju_mulMatMat, cxx:_mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mj_instantiate_contact(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx:_mj_instantiateContact
}

/// C: mj_diagApprox (engine/engine_core_constraint.c:1707)
/// Calls: cxx-internal:engine_core_constraint.c.o:_mj_elemBodyWeight, cxx-internal:engine_core_constraint.c.o:_mj_vertBodyWeight, cxx:_mju_flexGatherCellState, cxx:_mju_flexGatherFaceState, cxx:_mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_diag_approx(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx:_mj_diagApprox
}

/// C: getsolparam (engine/engine_core_constraint.c:1965)
/// Calls: cxx:_mj_defaultSolRefImp, cxx:_mj_isMetric, cxx:_mju_copy, cxx:_mju_max, cxx:_mju_min, cxx:_mju_warning, cxx:_mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn getsolparam(m: *const mjModel, d: *const mjData, i: i32, solref: *mut f64, solreffriction: *mut f64, solimp: *mut f64) {
    const mjNREF: i32 = 2;
    const mjNIMP: i32 = 5;
    const mjMINIMP: f64 = 1E-3;
    const mjMAXIMP: f64 = 0.9999;
    const mjDSBL_REFSAFE: i32 = 1 << 12;

    const mjCNSTR_EQUALITY: i32 = 0;
    const mjCNSTR_FRICTION_DOF: i32 = 1;
    const mjCNSTR_FRICTION_TENDON: i32 = 2;
    const mjCNSTR_LIMIT_JOINT: i32 = 3;
    const mjCNSTR_LIMIT_TENDON: i32 = 4;
    const mjCNSTR_CONTACT_FRICTIONLESS: i32 = 5;
    const mjCNSTR_CONTACT_PYRAMIDAL: i32 = 6;
    const mjCNSTR_CONTACT_ELLIPTIC: i32 = 7;

    // SAFETY: m, d are valid model/data pointers; solref, solreffriction, solimp are valid output buffers
    unsafe {
        // get constraint id
        let id: i32 = *(*d).efc_id.add(i as usize);

        // clear solreffriction (applies only to contacts)
        crate::engine::engine_util_blas::mju_zero(solreffriction, mjNREF);

        // extract solver parameters from corresponding model element
        let efc_type: i32 = *(*d).efc_type.add(i as usize);

        match efc_type {
            mjCNSTR_EQUALITY => {
                crate::engine::engine_util_blas::mju_copy(solref, (*m).eq_solref.add((mjNREF as usize) * (id as usize)), mjNREF);
                crate::engine::engine_util_blas::mju_copy(solimp, (*m).eq_solimp.add((mjNIMP as usize) * (id as usize)), mjNIMP);
            }
            mjCNSTR_LIMIT_JOINT => {
                crate::engine::engine_util_blas::mju_copy(solref, (*m).jnt_solref.add((mjNREF as usize) * (id as usize)), mjNREF);
                crate::engine::engine_util_blas::mju_copy(solimp, (*m).jnt_solimp.add((mjNIMP as usize) * (id as usize)), mjNIMP);
            }
            mjCNSTR_FRICTION_DOF => {
                crate::engine::engine_util_blas::mju_copy(solref, (*m).dof_solref.add((mjNREF as usize) * (id as usize)), mjNREF);
                crate::engine::engine_util_blas::mju_copy(solimp, (*m).dof_solimp.add((mjNIMP as usize) * (id as usize)), mjNIMP);
            }
            mjCNSTR_LIMIT_TENDON => {
                crate::engine::engine_util_blas::mju_copy(solref, (*m).tendon_solref_lim.add((mjNREF as usize) * (id as usize)), mjNREF);
                crate::engine::engine_util_blas::mju_copy(solimp, (*m).tendon_solimp_lim.add((mjNIMP as usize) * (id as usize)), mjNIMP);
            }
            mjCNSTR_FRICTION_TENDON => {
                crate::engine::engine_util_blas::mju_copy(solref, (*m).tendon_solref_fri.add((mjNREF as usize) * (id as usize)), mjNREF);
                crate::engine::engine_util_blas::mju_copy(solimp, (*m).tendon_solimp_fri.add((mjNIMP as usize) * (id as usize)), mjNIMP);
            }
            mjCNSTR_CONTACT_FRICTIONLESS | mjCNSTR_CONTACT_PYRAMIDAL | mjCNSTR_CONTACT_ELLIPTIC => {
                let contact = (*d).contact.add(id as usize);
                crate::engine::engine_util_blas::mju_copy(solref, (*contact).solref.as_ptr(), mjNREF);
                crate::engine::engine_util_blas::mju_copy(solreffriction, (*contact).solreffriction.as_ptr(), mjNREF);
                crate::engine::engine_util_blas::mju_copy(solimp, (*contact).solimp.as_ptr(), mjNIMP);
            }
            _ => {}
        }

        // check reference format: standard or direct, cannot be mixed
        if (*solref.add(0) > 0.0) ^ (*solref.add(1) > 0.0) {
            crate::engine::engine_util_errmem::mju_warning(b"mixed solref format, replacing with default\0".as_ptr() as *const i8);
            crate::engine::engine_init::mj_defaultSolRefImp(solref, std::ptr::null_mut());
        }

        // integrator safety: impose ref[0]>=2*timestep for standard format
        if ((*m).opt.disableflags & mjDSBL_REFSAFE) == 0 && *solref.add(0) > 0.0 {
            *solref.add(0) = crate::engine::engine_util_misc::mju_max(*solref.add(0), 2.0 * (*m).opt.timestep);
        }

        // check reference format: standard or direct, cannot be mixed
        if (*solreffriction.add(0) > 0.0) ^ (*solreffriction.add(1) > 0.0) {
            crate::engine::engine_util_errmem::mju_warning(b"solreffriction values should have the same sign, replacing with default\0".as_ptr() as *const i8);
            crate::engine::engine_util_blas::mju_zero(solreffriction, mjNREF);
        }

        // integrator safety: impose ref[0]>=2*timestep for standard format
        if ((*m).opt.disableflags & mjDSBL_REFSAFE) == 0 && *solreffriction.add(0) > 0.0 {
            *solreffriction.add(0) = crate::engine::engine_util_misc::mju_max(*solreffriction.add(0), 2.0 * (*m).opt.timestep);
        }

        // enforce constraints on solimp
        *solimp.add(0) = crate::engine::engine_util_misc::mju_min(mjMAXIMP, crate::engine::engine_util_misc::mju_max(mjMINIMP, *solimp.add(0)));
        *solimp.add(1) = crate::engine::engine_util_misc::mju_min(mjMAXIMP, crate::engine::engine_util_misc::mju_max(mjMINIMP, *solimp.add(1)));
        *solimp.add(2) = crate::engine::engine_util_misc::mju_max(0.0, *solimp.add(2));
        *solimp.add(3) = crate::engine::engine_util_misc::mju_min(mjMAXIMP, crate::engine::engine_util_misc::mju_max(mjMINIMP, *solimp.add(3)));
        *solimp.add(4) = crate::engine::engine_util_misc::mju_max(1.0, *solimp.add(4));
    }
}

/// C: getposdim (engine/engine_core_constraint.c:2043)
/// Calls: cxx:_mju_norm
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn getposdim(m: *const mjModel, d: *const mjData, i: i32, pos: *mut f64, dim: *mut i32) {
    // SAFETY: caller guarantees m, d are valid model/data pointers; pos, dim are writable
    unsafe {
        // get id of constraint-related object
        let id = *(*d).efc_id.add(i as usize);

        // set (dim, pos) for common case
        *dim = 1;
        *pos = *(*d).efc_pos.add(i as usize);

        // change (dim, distance) for special cases
        let efc_type = *(*d).efc_type.add(i as usize);

        const MJCNSTR_CONTACT_ELLIPTIC: i32 = 7;
        const MJCNSTR_CONTACT_PYRAMIDAL: i32 = 6;
        const MJCNSTR_EQUALITY: i32 = 0;

        if efc_type == MJCNSTR_CONTACT_ELLIPTIC {
            *dim = (*(*d).contact.add(id as usize)).dim;
        } else if efc_type == MJCNSTR_CONTACT_PYRAMIDAL {
            *dim = 2 * ((*(*d).contact.add(id as usize)).dim - 1);
        } else if efc_type == MJCNSTR_EQUALITY {
            if *(*m).eq_type.add(id as usize) == mjtEq_mjEQ_WELD as i32 {
                *dim = 6;
                *pos = crate::engine::engine_util_blas::mju_norm((*d).efc_pos.add(i as usize), 6);
            } else if *(*m).eq_type.add(id as usize) == mjtEq_mjEQ_CONNECT as i32 {
                *dim = 3;
                *pos = crate::engine::engine_util_blas::mju_norm((*d).efc_pos.add(i as usize), 3);
            }
        }
    }
}

/// C: power (engine/engine_core_constraint.c:2079)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn power(a: f64, b: f64) -> f64 {
    if b == 1.0 {
        a
    } else if b == 2.0 {
        a * a
    } else {
        f64::powf(a, b)
    }
}

/// C: getimpedance (engine/engine_core_constraint.c:2090)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_constraint.c:_power
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn getimpedance(solimp: *const f64, pos: f64, margin: f64, imp: *mut f64, impP: *mut f64) {
    // SAFETY: solimp points to at least 5 f64; imp, impP are writable
    unsafe {
        const MJ_MINVAL: f64 = 1E-15;

        // flat function
        if *solimp.add(0) == *solimp.add(1) || *solimp.add(2) <= MJ_MINVAL {
            *imp = 0.5 * (*solimp.add(0) + *solimp.add(1));
            *impP = 0.0;
            return;
        }

        // x = abs((pos-margin) / width)
        let mut x = (pos - margin) / *solimp.add(2);
        let mut sgn: f64 = 1.0;
        if x < 0.0 {
            x = -x;
            sgn = -1.0;
        }

        // fully saturated
        if x >= 1.0 || x <= 0.0 {
            *imp = if x >= 1.0 { *solimp.add(1) } else { *solimp.add(0) };
            *impP = 0.0;
            return;
        }

        // linear
        let y: f64;
        let yP: f64;
        if *solimp.add(4) == 1.0 {
            y = x;
            yP = 1.0;
        }
        // y(x) = a*x^p if x<=midpoint
        else if x <= *solimp.add(3) {
            let a = 1.0 / power(*solimp.add(3), *solimp.add(4) - 1.0);
            y = a * power(x, *solimp.add(4));
            yP = *solimp.add(4) * a * power(x, *solimp.add(4) - 1.0);
        }
        // y(x) = 1-b*(1-x)^p if x>midpoint
        else {
            let b = 1.0 / power(1.0 - *solimp.add(3), *solimp.add(4) - 1.0);
            y = 1.0 - b * power(1.0 - x, *solimp.add(4));
            yP = *solimp.add(4) * b * power(1.0 - x, *solimp.add(4) - 1.0);
        }

        // scale
        *imp = *solimp.add(0) + y * (*solimp.add(1) - *solimp.add(0));
        *impP = yP * sgn * (*solimp.add(1) - *solimp.add(0)) / *solimp.add(2);
    }
}

/// C: implicitFactor (engine/engine_core_constraint.c:2142)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn implicit_factor(kbip: *const f64, h: f64) -> f64 {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_constraint.c:_implicitFactor
}

/// C: mj_makeImpedance (engine/engine_core_constraint.c:2148)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_constraint.c:_getimpedance, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_constraint.c:_getposdim, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_constraint.c:_getsolparam, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_constraint.c:_implicitFactor, cxx:_mj_isMetric, cxx:_mju_max, cxx:_mju_min
#[allow(unused_variables, non_snake_case)]
pub fn mj_make_impedance(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx:_mj_makeImpedance
}

/// C: mj_makeYSymbolic (engine/engine_core_constraint.c:2297)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_constraint.c:_computeY_precount, cxx-internal:engine_core_constraint.c.o:_computeY_fill, cxx:_mj_arenaAllocByte, cxx:_mj_freeStack, cxx:_mj_isMetric, cxx:_mj_isSparse, cxx:_mj_markStack, cxx:_mj_stackAllocInfo, cxx:_mj_warning
#[allow(unused_variables, non_snake_case)]
pub fn mj_make_y_symbolic(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx-internal:engine_core_constraint.c.o:_mj_makeYSymbolic
}

/// C: mj_makeYNumeric (engine/engine_core_constraint.c:2298)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_constraint.c:_computeY_backsub, cxx-internal:engine_core_constraint.c.o:_computeY_fill, cxx:_mj_freeStack, cxx:_mj_isMetric, cxx:_mj_isSparse, cxx:_mj_markStack, cxx:_mj_solveM2_impl, cxx:_mj_stackAllocInfo, cxx:_mju_dot
#[allow(unused_variables, non_snake_case)]
pub fn mj_make_y_numeric(m: *const mjModel, d: *mut mjData, flg_diagexact: i32) {
    todo!() // cxx-internal:engine_core_constraint.c.o:_mj_makeYNumeric
}

/// C: mj_makeARSymbolic (engine/engine_core_constraint.c:2299)
/// Calls: cxx:_mj_arenaAllocByte, cxx:_mj_freeStack, cxx:_mj_isSparse, cxx:_mj_markStack, cxx:_mj_stackAllocInfo, cxx:_mj_warning, cxx:_mju_sqrMatTDSparseSymbolic, cxx:_mju_transposeSparse
#[allow(unused_variables, non_snake_case)]
pub fn mj_make_ar_symbolic(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx-internal:engine_core_constraint.c.o:_mj_makeARSymbolic
}

/// C: mj_makeARNumeric (engine/engine_core_constraint.c:2300)
/// Calls: cxx:_mj_freeStack, cxx:_mj_isSparse, cxx:_mj_markStack, cxx:_mj_stackAllocInfo, cxx:_mju_sqrMatTD, cxx:_mju_sqrMatTDSparseNumeric, cxx:_mju_transpose, cxx:_mju_transposeSparse
#[allow(unused_variables, non_snake_case)]
pub fn mj_make_ar_numeric(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx-internal:engine_core_constraint.c.o:_mj_makeARNumeric
}

/// C: mj_regularizeConstraint (engine/engine_core_constraint.c:2307)
/// Calls: cxx-internal:engine_core_constraint.c.o:_mj_makeARNumeric, cxx-internal:engine_core_constraint.c.o:_mj_makeYNumeric, cxx:_mj_diagApprox, cxx:_mj_isDual, cxx:_mj_isMetric, cxx:_mj_isSparse, cxx:_mj_makeImpedance, cxx:_mju_gather, cxx:_mju_max
#[allow(unused_variables, non_snake_case)]
pub fn mj_regularize_constraint(m: *const mjModel, d: *mut mjData, flg_AR: i32) {
    todo!() // cxx:_mj_regularizeConstraint
}

/// C: mj_jacSumCount (engine/engine_core_constraint.c:2366)
/// Calls: cxx:_mj_bodyChain, cxx:_mj_freeStack, cxx:_mj_markStack, cxx:_mj_stackAllocInfo, cxx:_mju_addChains, cxx:_mju_copyInt
#[allow(unused_variables, non_snake_case)]
pub fn mj_jac_sum_count(m: *const mjModel, d: *mut mjData, chain: *mut i32, n: i32, body: *const i32) -> i32 {
    todo!() // cxx-internal:engine_core_constraint.c.o:_mj_jacSumCount
}

/// C: mj_ne (engine/engine_core_constraint.c:2397)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_constraint.c:_mj_addConstraintCount, cxx-internal:engine_core_constraint.c.o:_mj_jacSumCount, cxx:_mj_freeStack, cxx:_mj_jacDifPair, cxx:_mj_markStack, cxx:_mj_sleepState, cxx:_mj_stackAllocInfo, cxx:_mju_combineSparseCount, cxx:_mju_copyInt, cxx:_mju_flexGatherCellState, cxx:_mju_flexGatherFaceState, cxx:_mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_ne(m: *const mjModel, d: *mut mjData, nnz: *mut i32) -> i32 {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_constraint.c:_mj_ne
}

/// C: mj_nc (engine/engine_core_constraint.c:2630)
/// Calls: cxx-internal:engine_core_constraint.c.o:_mj_elemBodyWeight, cxx-internal:engine_core_constraint.c.o:_mj_jacSumCount, cxx-internal:engine_core_constraint.c.o:_mj_vertBodyWeight, cxx:_mj_effFlexContactPossible, cxx:_mj_flexBody, cxx:_mj_freeStack, cxx:_mj_isPyramidal, cxx:_mj_isSparse, cxx:_mj_jacDifPair, cxx:_mj_markStack, cxx:_mj_stackAllocInfo, cxx:_mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_nc(m: *const mjModel, d: *mut mjData, nnz: *mut i32) -> i32 {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_constraint.c:_mj_nc
}

/// C: computeY_precount (engine/engine_core_constraint.c:2794)
/// Calls: cxx:_mju_fillInt
#[allow(unused_variables, non_snake_case)]
pub fn compute_y_precount(Y_rownnz: *mut i32, Y_rowadr: *mut i32, nefc: i32, nv: i32, J_rownnz: *const i32, J_rowadr: *const i32, J_colind: *const i32, M_rownnz: *const i32, M_rowadr: *const i32, M_colind: *const i32, marker: *mut i32) -> i32 {
    todo!() // cxx-internal:engine_core_constraint.c.o:_computeY_precount
}

/// C: computeY_fill (engine/engine_core_constraint.c:2841)
/// Calls: cxx:_mju_message
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn compute_y_fill(Y: *mut f64, Y_colind: *mut i32, Y_rownnz: *const i32, Y_rowadr: *const i32, nefc: i32, J: *const f64, J_rownnz: *const i32, J_rowadr: *const i32, J_colind: *const i32, dof_parentid: *const i32) {
    todo!() // cxx-internal:engine_core_constraint.c.o:_computeY_fill
}

/// C: computeY_backsub (engine/engine_core_constraint.c:2892)
/// Calls: cxx:_mju_addToSclSparseInc
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn compute_y_backsub(Y: *mut f64, Y_rownnz: *const i32, Y_rowadr: *const i32, Y_colind: *const i32, nefc: i32, qLD: *const f64, M_rownnz: *const i32, M_rowadr: *const i32, M_colind: *const i32, sqrtInvD: *const f64) {
    todo!() // cxx-internal:engine_core_constraint.c.o:_computeY_backsub
}

/// C: mj_makeConstraint (engine/engine_core_constraint.c:2925)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_constraint.c:_arenaAllocEfc, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_constraint.c:_mj_nc, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_constraint.c:_mj_ne, cxx-internal:engine_core_constraint.c.o:_mj_instantiateFriction, cxx-internal:engine_core_constraint.c.o:_mj_instantiateLimit, cxx:_mj_diagApprox, cxx:_mj_instantiateContact, cxx:_mj_instantiateEquality, cxx:_mj_isMetric, cxx:_mj_isSparse, cxx:_mj_makeImpedance, cxx:_mju_fillInt, cxx:_mju_message, cxx:_mju_superSparse
#[allow(unused_variables, non_snake_case)]
pub fn mj_make_constraint(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx:_mj_makeConstraint
}

/// C: mj_projectConstraint (engine/engine_core_constraint.c:3287)
/// Calls: cxx-internal:engine_core_constraint.c.o:_mj_makeARNumeric, cxx-internal:engine_core_constraint.c.o:_mj_makeARSymbolic, cxx-internal:engine_core_constraint.c.o:_mj_makeYNumeric, cxx-internal:engine_core_constraint.c.o:_mj_makeYSymbolic, cxx:_mj_isDual, cxx:_mj_isMetric, cxx:_mj_makeImpedance, cxx:_mju_gather
#[allow(unused_variables, non_snake_case)]
pub fn mj_project_constraint(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx:_mj_projectConstraint
}

/// C: mj_addSurfaceVel (engine/engine_core_constraint.c:3344)
/// Calls: cxx:_mj_geomSurfaceVelocity, cxx:_mj_isPyramidal, cxx:_mju_addToScl3, cxx:_mju_mulMatVec3
#[allow(unused_variables, non_snake_case)]
pub fn mj_add_surface_vel(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_constraint.c:_mj_addSurfaceVel
}

/// C: mj_adhesionRef (engine/engine_core_constraint.c:3417)
/// Calls: cxx:_mj_isPyramidal
#[allow(unused_variables, non_snake_case)]
pub fn mj_adhesion_ref(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_constraint.c:_mj_adhesionRef
}

/// C: mj_velocityConstraint (engine/engine_core_constraint.c:3448)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_constraint.c:_mj_addSurfaceVel, cxx:_mj_mulJacVec
#[allow(unused_variables, non_snake_case)]
pub fn mj_velocity_constraint(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx:_mj_velocityConstraint
}

/// C: mj_referenceConstraint (engine/engine_core_constraint.c:3455)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_constraint.c:_implicitFactor, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_constraint.c:_mj_adhesionRef, cxx:_mj_Jdotv, cxx:_mj_isMetric, cxx:_mj_velocityConstraint
#[allow(unused_variables, non_snake_case)]
pub fn mj_reference_constraint(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx:_mj_referenceConstraint
}

/// C: mj_constraintUpdate_impl (engine/engine_core_constraint.c:3501)
/// Calls: cxx:_mju_norm, cxx:_mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_constraint_update_impl(ne: i32, nf: i32, nefc: i32, D: *const f64, R: *const f64, floss: *const f64, jar: *const f64, r#type: *const i32, id: *const i32, contact: *mut mjContact, state: *mut i32, force: *mut f64, cost: *mut f64, flg_coneHessian: i32) {
    todo!() // cxx:_mj_constraintUpdate_impl
}

/// C: mj_constraintUpdate (engine/engine_core_constraint.c:3696)
/// Calls: cxx:_mj_constraintUpdate_impl, cxx:_mj_mulJacTVec
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_constraint_update(m: *const mjModel, d: *mut mjData, jar: *const f64, cost: *mut f64, flg_coneHessian: i32) {
    todo!() // cxx:_mj_constraintUpdate
}

