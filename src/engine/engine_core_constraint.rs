//! Port of: engine/engine_core_constraint.c
//! IR hash: 8cbd078414266fa8
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: cell_pos_and_jac (engine/engine_core_constraint.c:55)
/// Calls: mj_bodyChain, mj_jacSparse, mj_stackAllocInfo, mju_zero, mju_zeroInt
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn cell_pos_and_jac(m: *const mjModel, d: *mut mjData, flex_id: i32, npc: i32, gindices: *const i32, nv: i32, xpos_c: *const f64, cell_chain: *mut i32, cell_nnz: *mut i32) -> *mut f64 {
    todo!() // cell_pos_and_jac
}

/// C: cell_strain_jacobian (engine/engine_core_constraint.c:111)
/// Calls: mju_zero
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
/// Calls: mj_arenaAllocByte, mj_clearEfc, mj_warning
#[allow(unused_variables, non_snake_case)]
pub fn arena_alloc_efc(m: *const mjModel, d: *mut mjData) -> i32 {
    todo!() // arenaAllocEfc
}

/// C: mj_elemBodyWeight (engine/engine_core_constraint.c:223)
/// Calls: mju_dist3, mju_max, mju_message, mju_scl, mju_sum
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_elem_body_weight(m: *const mjModel, d: *const mjData, f: i32, e: i32, v: i32, point: *const f64, body: *mut i32, weight: *mut f64) -> i32 {
    todo!() // mj_elemBodyWeight
}

/// C: mj_vertBodyWeight (engine/engine_core_constraint.c:265)
/// Calls: mju_addToScl3, mju_cellLookup, mju_evalBasis, mju_evalBasisArray, mju_shellTFIWeights
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_vert_body_weight(m: *const mjModel, d: *const mjData, f: i32, v: *mut i32, body: *mut i32, bweight: *mut f64, vweight: *const f64, nw: i32) -> i32 {
    todo!() // mj_vertBodyWeight
}

/// C: mj_addConstraint (engine/engine_core_constraint.c:414)
/// Calls: mj_isSparse, mju_compare, mju_copy, mju_copyInt, mju_fillInt, mju_message
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_add_constraint(m: *const mjModel, d: *mut mjData, jac: *const f64, pos: *const f64, margin: *const f64, frictionloss: f64, size: i32, r#type: i32, id: i32, NV: i32, chain: *const i32) {
    todo!() // mj_addConstraint
}

/// C: mj_equalityAnchors (engine/engine_core_constraint.c:561)
/// Calls: mju_addTo3, mju_copy3, mju_mulMatVec3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_equality_anchors(m: *const mjModel, d: *const mjData, eq_id: i32, pos1: *mut f64, pos2: *mut f64, body1: *mut i32, body2: *mut i32) {
    const mjNEQDATA: i32 = 11;

    // SAFETY: caller guarantees m, d, pos1, pos2, body1, body2 are valid
    unsafe {
        let eq_type = *(*m).eq_type.add(eq_id as usize);
        let obj1 = *(*m).eq_obj1id.add(eq_id as usize);
        let obj2 = *(*m).eq_obj2id.add(eq_id as usize);

        if *(*m).eq_objtype.add(eq_id as usize) == mjtObj_mjOBJ_BODY as i32 {
            let data = (*m).eq_data.add((mjNEQDATA * eq_id) as usize);
            if eq_type == mjtEq_mjEQ_CONNECT as i32 {
                crate::engine::engine_util_blas::mju_mul_mat_vec3(
                    pos1, (*d).xmat.add(9 * obj1 as usize), data);
                crate::engine::engine_util_blas::mju_add_to3(
                    pos1, (*d).xpos.add(3 * obj1 as usize));
                crate::engine::engine_util_blas::mju_mul_mat_vec3(
                    pos2, (*d).xmat.add(9 * obj2 as usize), data.add(3));
                crate::engine::engine_util_blas::mju_add_to3(
                    pos2, (*d).xpos.add(3 * obj2 as usize));
            } else {
                crate::engine::engine_util_blas::mju_mul_mat_vec3(
                    pos1, (*d).xmat.add(9 * obj1 as usize), data.add(3));
                crate::engine::engine_util_blas::mju_add_to3(
                    pos1, (*d).xpos.add(3 * obj1 as usize));
                crate::engine::engine_util_blas::mju_mul_mat_vec3(
                    pos2, (*d).xmat.add(9 * obj2 as usize), data);
                crate::engine::engine_util_blas::mju_add_to3(
                    pos2, (*d).xpos.add(3 * obj2 as usize));
            }
            *body1 = obj1;
            *body2 = obj2;
        } else {
            crate::engine::engine_util_blas::mju_copy3(
                pos1, (*d).site_xpos.add(3 * obj1 as usize));
            crate::engine::engine_util_blas::mju_copy3(
                pos2, (*d).site_xpos.add(3 * obj2 as usize));
            *body1 = *(*m).site_bodyid.add(obj1 as usize);
            *body2 = *(*m).site_bodyid.add(obj2 as usize);
        }
    }
}

/// C: mj_addConstraintCount (engine/engine_core_constraint.c:1259)
/// Calls: mj_isSparse
#[allow(unused_variables, non_snake_case)]
pub fn mj_add_constraint_count(m: *const mjModel, size: i32, NV: i32) -> i32 {
    // SAFETY: caller guarantees m is a valid mjModel pointer
    unsafe {
        if crate::engine::engine_core_util::mj_is_sparse(m) == 0 {
            return if (*m).nv != 0 { size } else { 0 };
        }
        let max_nv = if 0 > NV { 0 } else { NV };
        if max_nv != 0 { size } else { 0 }
    }
}

/// C: mj_instantiateFriction (engine/engine_core_constraint.c:1270)
/// Calls: mj_addConstraint, mj_addConstraintCount, mj_freeStack, mj_isSparse, mj_markStack, mj_sleepState, mj_stackAllocInfo, mju_sparse2dense, mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mj_instantiate_friction(m: *const mjModel, d: *mut mjData, count_only: i32, nnz: *mut i32) -> i32 {
    todo!() // mj_instantiateFriction
}

/// C: mj_instantiateLimit (engine/engine_core_constraint.c:1360)
/// Calls: mj_addConstraint, mj_addConstraintCount, mj_freeStack, mj_isSparse, mj_markStack, mj_sleepState, mj_stackAllocInfo, mju_max, mju_normalize3, mju_normalize4, mju_quat2Vel, mju_scl, mju_scl3, mju_sparse2dense, mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mj_instantiate_limit(m: *const mjModel, d: *mut mjData, count_only: i32, nnz: *mut i32) -> i32 {
    todo!() // mj_instantiateLimit
}

/// C: getsolparam (engine/engine_core_constraint.c:1978)
/// Calls: mj_defaultSolRefImp, mju_copy, mju_max, mju_min, mju_warning, mju_zero
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
            crate::engine::engine_init::mj_default_sol_ref_imp(solref, std::ptr::null_mut());
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

/// C: getposdim (engine/engine_core_constraint.c:2053)
/// Calls: mju_norm
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

/// C: power (engine/engine_core_constraint.c:2089)
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

/// C: getimpedance (engine/engine_core_constraint.c:2100)
/// Calls: power
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

/// C: mj_jacSumCount (engine/engine_core_constraint.c:2272)
/// Calls: mj_bodyChain, mj_freeStack, mj_markStack, mj_stackAllocInfo, mju_addChains, mju_copyInt
#[allow(unused_variables, non_snake_case)]
pub fn mj_jac_sum_count(m: *const mjModel, d: *mut mjData, chain: *mut i32, n: i32, body: *const i32) -> i32 {
    todo!() // mj_jacSumCount
}

/// C: mj_ne (engine/engine_core_constraint.c:2303)
/// Calls: mj_addConstraintCount, mj_freeStack, mj_jacDifPair, mj_jacSumCount, mj_markStack, mj_sleepState, mj_stackAllocInfo, mju_combineSparseCount, mju_copyInt, mju_flexGatherCellState, mju_flexGatherFaceState, mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_ne(m: *const mjModel, d: *mut mjData, nnz: *mut i32) -> i32 {
    todo!() // mj_ne
}

/// C: mj_nc (engine/engine_core_constraint.c:2536)
/// Calls: mj_elemBodyWeight, mj_flexBody, mj_freeStack, mj_isPyramidal, mj_isSparse, mj_jacDifPair, mj_jacSumCount, mj_markStack, mj_stackAllocInfo, mj_vertBodyWeight, mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_nc(m: *const mjModel, d: *mut mjData, nnz: *mut i32) -> i32 {
    todo!() // mj_nc
}

/// C: computeY_precount (engine/engine_core_constraint.c:2688)
/// Calls: mju_fillInt
#[allow(unused_variables, non_snake_case)]
pub fn compute_y_precount(Y_rownnz: *mut i32, Y_rowadr: *mut i32, nefc: i32, nv: i32, J_rownnz: *const i32, J_rowadr: *const i32, J_colind: *const i32, M_rownnz: *const i32, M_rowadr: *const i32, M_colind: *const i32, marker: *mut i32) -> i32 {
    todo!() // computeY_precount
}

/// C: computeY_fill (engine/engine_core_constraint.c:2734)
/// Calls: mju_message
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn compute_y_fill(Y: *mut f64, Y_colind: *mut i32, Y_rownnz: *const i32, Y_rowadr: *const i32, nefc: i32, J: *const f64, J_rownnz: *const i32, J_rowadr: *const i32, J_colind: *const i32, dof_parentid: *const i32) {
    todo!() // computeY_fill
}

/// C: computeY_backsub (engine/engine_core_constraint.c:2781)
/// Calls: mju_addToSclSparseInc
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn compute_y_backsub(Y: *mut f64, Y_rownnz: *const i32, Y_rowadr: *const i32, Y_colind: *const i32, nefc: i32, qLD: *const f64, M_rownnz: *const i32, M_rowadr: *const i32, M_colind: *const i32, sqrtInvD: *const f64) {
    // SAFETY: all pointers are valid sparse matrix data (caller contract)
    unsafe {
        for r in 0..nefc {
            let nnzY = *Y_rownnz.add(r as usize);
            let adrY = *Y_rowadr.add(r as usize);

            // Y(r,:) <- inv(L') * Y(r,:), exploit sparsity of input vector
            let mut i = adrY + nnzY - 1;
            while i >= adrY {
                let val = *Y.add(i as usize);
                if val != 0.0 {
                    let j = *Y_colind.add(i as usize);
                    let adrM = *M_rowadr.add(j as usize);
                    crate::engine::engine_util_sparse::mju_add_to_scl_sparse_inc(
                        Y.add(adrY as usize),
                        qLD.add(adrM as usize),
                        nnzY,
                        Y_colind.add(adrY as usize),
                        *M_rownnz.add(j as usize) - 1,
                        M_colind.add(adrM as usize),
                        -val,
                    );
                }
                i -= 1;
            }

            // Y(r,:) <- sqrt(inv(D)) * Y(r,:)
            for i in adrY..(adrY + nnzY) {
                let j = *Y_colind.add(i as usize);
                *Y.add(i as usize) *= *sqrtInvD.add(j as usize);
            }
        }
    }
}

/// C: mj_makeY (engine/engine_core_constraint.c:2908)
/// Calls: computeY_backsub, computeY_fill, computeY_precount, mj_arenaAllocByte, mj_clearEfc, mj_freeStack, mj_isSparse, mj_markStack, mj_solveM2, mj_stackAllocInfo, mj_warning, mju_dot
#[allow(unused_variables, non_snake_case)]
pub fn mj_make_y(m: *const mjModel, d: *mut mjData, flg_diagexact: i32) {
    todo!() // mj_makeY
}

/// C: mj_makeAR (engine/engine_core_constraint.c:2999)
/// Calls: mj_arenaAllocByte, mj_clearEfc, mj_freeStack, mj_isSparse, mj_markStack, mj_stackAllocInfo, mj_warning, mju_sqrMatTD, mju_sqrMatTDSparseNumeric, mju_sqrMatTDSparseSymbolic, mju_transpose, mju_transposeSparse
#[allow(unused_variables, non_snake_case)]
pub fn mj_make_ar(m: *const mjModel, d: *mut mjData) {
    todo!() // mj_makeAR
}

/// C: mj_isDual (engine/engine_core_constraint.h:31)
#[allow(unused_variables, non_snake_case)]
pub fn mj_is_dual(m: *const mjModel) -> i32 {
    const MJ_SOL_PGS: i32 = 0;

    // SAFETY: m is a valid mjModel pointer (caller contract)
    unsafe {
        if (*m).opt.solver == MJ_SOL_PGS || (*m).opt.noslip_iterations > 0 {
            1
        } else {
            0
        }
    }
}

/// C: mj_mulJacVec (engine/engine_core_constraint.h:34)
/// Calls: mj_isSparse, mju_mulMatVec, mju_mulMatVecSparse
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_mul_jac_vec(m: *const mjModel, d: *const mjData, res: *mut f64, vec: *const f64) {
    todo!() // mj_mulJacVec
}

/// C: mj_mulJacTVec (engine/engine_core_constraint.h:37)
/// Calls: mj_isSparse, mju_mulMatTVec, mju_mulMatTVecSparse
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_mul_jac_t_vec(m: *const mjModel, d: *const mjData, res: *mut f64, vec: *const f64) {
    // SAFETY: m, d are valid model/data pointers; res, vec are valid arrays (caller contract)
    unsafe {
        if (*d).nefc == 0 {
            return;
        }
        if crate::engine::engine_core_util::mj_is_sparse(m) != 0 {
            crate::engine::engine_util_sparse::mju_mul_mat_t_vec_sparse(
                res, (*d).efc_J, vec, (*d).nefc, (*m).nv as i32,
                (*d).efc_J_rownnz, (*d).efc_J_rowadr, (*d).efc_J_colind,
            );
        } else {
            crate::engine::engine_util_blas::mju_mul_mat_t_vec(
                res, (*d).efc_J, vec, (*d).nefc, (*m).nv as i32,
            );
        }
    }
}

/// C: mj_Jdotv (engine/engine_core_constraint.h:40)
/// Calls: mj_equalityAnchors, mj_freeStack, mj_isSparse, mj_jacDot, mj_jacDotSparse, mj_markStack, mj_mergeChain, mj_stackAllocInfo, mju_copy4, mju_derivQuat, mju_dotSparseX3, mju_mulMatVec, mju_mulQuat, mju_mulQuatAxis, mju_negQuat, mju_sub3, mju_zero3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_jdotv(m: *const mjModel, d: *mut mjData, result: *mut f64) {
    todo!() // mj_Jdotv
}

/// C: mj_assignRef (engine/engine_core_constraint.h:46)
/// Calls: mju_copy
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_assign_ref(m: *const mjModel, target: *mut f64, source: *const f64) {
    const mjENBL_OVERRIDE: i32 = 1 << 0;
    const mjNREF: i32 = 2;

    // SAFETY: m is a valid mjModel pointer; target and source are valid f64 arrays of size mjNREF
    unsafe {
        if ((*m).opt.enableflags & mjENBL_OVERRIDE) != 0 {
            crate::engine::engine_util_blas::mju_copy(target, (*m).opt.o_solref.as_ptr(), mjNREF);
        } else {
            crate::engine::engine_util_blas::mju_copy(target, source, mjNREF);
        }
    }
}

/// C: mj_assignImp (engine/engine_core_constraint.h:49)
/// Calls: mju_copy
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_assign_imp(m: *const mjModel, target: *mut f64, source: *const f64) {
    const mjENBL_OVERRIDE: i32 = 1 << 0;
    const mjNIMP: i32 = 5;

    // SAFETY: m is a valid mjModel pointer; target and source are valid f64 arrays of size mjNIMP
    unsafe {
        if ((*m).opt.enableflags & mjENBL_OVERRIDE) != 0 {
            crate::engine::engine_util_blas::mju_copy(target, (*m).opt.o_solimp.as_ptr(), mjNIMP);
        } else {
            crate::engine::engine_util_blas::mju_copy(target, source, mjNIMP);
        }
    }
}

/// C: mj_assignFriction (engine/engine_core_constraint.h:52)
/// Calls: mju_max
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_assign_friction(m: *const mjModel, target: *mut f64, source: *const f64) {
    const mjENBL_OVERRIDE: i32 = 1 << 0;
    const mjMINMU: f64 = 1E-5;

    // SAFETY: m is a valid mjModel pointer; target and source are valid f64[5] arrays
    unsafe {
        if ((*m).opt.enableflags & mjENBL_OVERRIDE) != 0 {
            for i in 0..5 {
                *target.add(i) = crate::engine::engine_util_misc::mju_max(mjMINMU, (*m).opt.o_friction[i]);
            }
        } else {
            for i in 0..5 {
                *target.add(i) = crate::engine::engine_util_misc::mju_max(mjMINMU, *source.add(i));
            }
        }
    }
}

/// C: mj_assignMargin (engine/engine_core_constraint.h:55)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_assign_margin(m: *const mjModel, source: f64) -> f64 {
    // SAFETY: m is a valid pointer to mjModel (caller contract)
    unsafe {
        const mjENBL_OVERRIDE: i32 = 1 << 0;
        if ((*m).opt.enableflags & mjENBL_OVERRIDE) != 0 {
            (*m).opt.o_margin
        } else {
            source
        }
    }
}

/// C: mj_addContact (engine/engine_core_constraint.h:58)
/// Calls: mj_arenaAllocByte, mj_clearEfc, mj_warning
#[allow(unused_variables, non_snake_case)]
pub fn mj_add_contact(m: *const mjModel, d: *mut mjData, con: *const mjContact) -> i32 {
    todo!() // mj_addContact
}

/// C: mj_instantiateEquality (engine/engine_core_constraint.h:63)
/// Calls: cell_pos_and_jac, cell_strain_jacobian, mj_addConstraint, mj_equalityAnchors, mj_freeStack, mj_isSparse, mj_jacDifPair, mj_markStack, mj_sleepState, mj_stackAllocInfo, mju_addTo3, mju_addToScl, mju_combineSparse, mju_copy, mju_copy3, mju_copyInt, mju_defGradient, mju_flexGatherCellState, mju_flexGatherFaceState, mju_flexInterpRotation2D, mju_mat2Rot, mju_message, mju_mulMatVec3, mju_mulQuat, mju_mulQuatAxis, mju_negQuat, mju_rotVecQuat, mju_scl, mju_scl3, mju_sparse2dense, mju_sub3, mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mj_instantiate_equality(m: *const mjModel, d: *mut mjData) {
    todo!() // mj_instantiateEquality
}

/// C: mj_instantiateContact (engine/engine_core_constraint.h:66)
/// Calls: mj_addConstraint, mj_contactJacobian, mj_freeStack, mj_isPyramidal, mj_isSparse, mj_markStack, mj_stackAllocInfo, mju_addScl, mju_mulMatMat, mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mj_instantiate_contact(m: *const mjModel, d: *mut mjData) {
    todo!() // mj_instantiateContact
}

/// C: mj_contactJacobian (engine/engine_core_constraint.h:69)
/// Calls: mj_elemBodyWeight, mj_isSparse, mj_jacDifPair, mj_jacSum, mj_vertBodyWeight, mju_scl
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_contact_jacobian(m: *const mjModel, d: *mut mjData, con: *const mjContact, dim: i32, jac: *mut f64, jacdif: *mut f64, jacdifp: *mut f64, jacdifr: *mut f64, jac1p: *mut f64, jac2p: *mut f64, jac1r: *mut f64, jac2r: *mut f64, chain: *mut i32) -> i32 {
    todo!() // mj_contactJacobian
}

/// C: mj_diagApprox (engine/engine_core_constraint.h:78)
/// Calls: mj_elemBodyWeight, mj_vertBodyWeight, mju_flexGatherCellState, mju_flexGatherFaceState, mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_diag_approx(m: *const mjModel, d: *mut mjData) {
    todo!() // mj_diagApprox
}

/// C: mj_makeImpedance (engine/engine_core_constraint.h:81)
/// Calls: getimpedance, getposdim, getsolparam, mju_max
#[allow(unused_variables, non_snake_case)]
pub fn mj_make_impedance(m: *const mjModel, d: *mut mjData) {
    todo!() // mj_makeImpedance
}

/// C: mj_makeConstraint (engine/engine_core_constraint.h:87)
/// Calls: arenaAllocEfc, mj_diagApprox, mj_instantiateContact, mj_instantiateEquality, mj_instantiateFriction, mj_instantiateLimit, mj_isSparse, mj_makeImpedance, mj_nc, mj_ne, mju_fillInt, mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_make_constraint(m: *const mjModel, d: *mut mjData) {
    todo!() // mj_makeConstraint
}

/// C: mj_projectConstraint (engine/engine_core_constraint.h:90)
/// Calls: mj_isDual, mj_makeAR, mj_makeImpedance, mj_makeY, mju_gather
#[allow(unused_variables, non_snake_case)]
pub fn mj_project_constraint(m: *const mjModel, d: *mut mjData) {
    todo!() // mj_projectConstraint
}

/// C: mj_referenceConstraint (engine/engine_core_constraint.h:93)
/// Calls: mj_Jdotv, mj_mulJacVec
#[allow(unused_variables, non_snake_case)]
pub fn mj_reference_constraint(m: *const mjModel, d: *mut mjData) {
    todo!() // mj_referenceConstraint
}

/// C: mj_constraintUpdate_impl (engine/engine_core_constraint.h:97)
/// Calls: mju_norm, mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_constraint_update_impl(ne: i32, nf: i32, nefc: i32, D: *const f64, R: *const f64, floss: *const f64, jar: *const f64, r#type: *const i32, id: *const i32, contact: *mut mjContact, state: *mut i32, force: *mut f64, cost: *mut f64, flg_coneHessian: i32) {
    use crate::engine::engine_util_blas::{mju_norm, mju_zero};

    const mjCNSTRSTATE_SATISFIED: i32 = 0;
    const mjCNSTRSTATE_QUADRATIC: i32 = 1;
    const mjCNSTRSTATE_LINEARNEG: i32 = 2;
    const mjCNSTRSTATE_LINEARPOS: i32 = 3;
    const mjCNSTRSTATE_CONE: i32 = 4;
    const mjCNSTR_CONTACT_ELLIPTIC: i32 = 7;

    // SAFETY: all pointer dereferences follow C contract — caller guarantees valid arrays
    // of size nefc for D, R, floss, jar, type, id, state, force; contact indexed by id[i].
    unsafe {
        let mut s: f64 = 0.0;

        if nefc == 0 {
            if !cost.is_null() {
                *cost = 0.0;
            }
            return;
        }

        // force[i] = -D[i]*jar[i]
        for i in 0..nefc as isize {
            *force.offset(i) = -*D.offset(i) * *jar.offset(i);
        }

        let mut i: i32 = 0;
        while i < nefc {
            if i < ne {
                if !cost.is_null() {
                    s += 0.5 * *D.offset(i as isize) * *jar.offset(i as isize) * *jar.offset(i as isize);
                }
                *state.offset(i as isize) = mjCNSTRSTATE_QUADRATIC;
                i += 1;
                continue;
            }

            if i < ne + nf {
                if *jar.offset(i as isize) <= -*R.offset(i as isize) * *floss.offset(i as isize) {
                    if !cost.is_null() {
                        s += -0.5 * *R.offset(i as isize) * *floss.offset(i as isize) * *floss.offset(i as isize)
                            - *floss.offset(i as isize) * *jar.offset(i as isize);
                    }
                    *force.offset(i as isize) = *floss.offset(i as isize);
                    *state.offset(i as isize) = mjCNSTRSTATE_LINEARNEG;
                } else if *jar.offset(i as isize) >= *R.offset(i as isize) * *floss.offset(i as isize) {
                    if !cost.is_null() {
                        s += -0.5 * *R.offset(i as isize) * *floss.offset(i as isize) * *floss.offset(i as isize)
                            + *floss.offset(i as isize) * *jar.offset(i as isize);
                    }
                    *force.offset(i as isize) = -*floss.offset(i as isize);
                    *state.offset(i as isize) = mjCNSTRSTATE_LINEARPOS;
                } else {
                    if !cost.is_null() {
                        s += 0.5 * *D.offset(i as isize) * *jar.offset(i as isize) * *jar.offset(i as isize);
                    }
                    *state.offset(i as isize) = mjCNSTRSTATE_QUADRATIC;
                }
                i += 1;
                continue;
            }

            // contact
            if *r#type.offset(i as isize) != mjCNSTR_CONTACT_ELLIPTIC {
                if *jar.offset(i as isize) >= 0.0 {
                    *force.offset(i as isize) = 0.0;
                    *state.offset(i as isize) = mjCNSTRSTATE_SATISFIED;
                } else {
                    if !cost.is_null() {
                        s += 0.5 * *D.offset(i as isize) * *jar.offset(i as isize) * *jar.offset(i as isize);
                    }
                    *state.offset(i as isize) = mjCNSTRSTATE_QUADRATIC;
                }
            } else {
                let con: *mut mjContact = contact.offset(*id.offset(i as isize) as isize);
                let mu: f64 = (*con).mu;
                let friction: *mut f64 = (*con).friction.as_mut_ptr();
                let dim: i32 = (*con).dim;

                let mut U: [f64; 6] = [0.0; 6];
                U[0] = *jar.offset(i as isize) * mu;
                for j in 1..dim {
                    U[j as usize] = *jar.offset((i + j) as isize) * *friction.offset((j - 1) as isize);
                }

                let N: f64 = U[0];
                let T: f64 = mju_norm(U.as_ptr().add(1), dim - 1);

                if N >= mu * T || (T <= 0.0 && N >= 0.0) {
                    mju_zero(force.offset(i as isize), dim);
                    *state.offset(i as isize) = mjCNSTRSTATE_SATISFIED;
                } else if mu * N + T <= 0.0 || (T <= 0.0 && N < 0.0) {
                    if !cost.is_null() {
                        for j in 0..dim {
                            s += 0.5 * *D.offset((i + j) as isize) * *jar.offset((i + j) as isize) * *jar.offset((i + j) as isize);
                        }
                    }
                    *state.offset(i as isize) = mjCNSTRSTATE_QUADRATIC;
                } else {
                    let Dm: f64 = *D.offset(i as isize) / (mu * mu * (1.0 + mu * mu));
                    let NmT: f64 = N - mu * T;

                    if !cost.is_null() {
                        s += 0.5 * Dm * NmT * NmT;
                    }

                    *force.offset(i as isize) = -Dm * NmT * mu;

                    for j in 1..dim {
                        *force.offset((i + j) as isize) = -*force.offset(i as isize) / T * U[j as usize] * *friction.offset((j - 1) as isize);
                    }

                    *state.offset(i as isize) = mjCNSTRSTATE_CONE;

                    if flg_coneHessian != 0 {
                        let H: *mut f64 = (*contact.offset(*id.offset(i as isize) as isize)).H.as_mut_ptr();

                        let mut scl: f64 = -mu / T;
                        *H.offset(0) = 1.0;
                        for j in 1..dim {
                            *H.offset(j as isize) = scl * U[j as usize];
                        }

                        scl = mu * N / (T * T * T);
                        for k in 1..dim {
                            for j in k..dim {
                                *H.offset((k * dim + j) as isize) = scl * U[j as usize] * U[k as usize];
                            }
                        }

                        scl = mu * mu - mu * N / T;
                        for j in 1..dim {
                            *H.offset((j * (dim + 1)) as isize) += scl;
                        }

                        for k in 0..dim {
                            scl = Dm * (if k == 0 { mu } else { *friction.offset((k - 1) as isize) });
                            for j in k..dim {
                                *H.offset((k * dim + j) as isize) *= scl * (if j == 0 { mu } else { *friction.offset((j - 1) as isize) });
                            }
                        }

                        for k in 0..dim {
                            for j in (k + 1)..dim {
                                *H.offset((j * dim + k) as isize) = *H.offset((k * dim + j) as isize);
                            }
                        }
                    }
                }

                for j in 1..dim {
                    *state.offset((i + j) as isize) = *state.offset(i as isize);
                }
                i += dim - 1;
            }

            i += 1;
        }

        if !cost.is_null() {
            *cost = s;
        }
    }
}

/// C: mj_constraintUpdate (engine/engine_core_constraint.h:105)
/// Calls: mj_constraintUpdate_impl, mj_mulJacTVec
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_constraint_update(m: *const mjModel, d: *mut mjData, jar: *const f64, cost: *mut f64, flg_coneHessian: i32) {
    todo!() // mj_constraintUpdate
}

