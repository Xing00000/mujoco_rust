//! Port of: engine/engine_core_constraint.c
//! IR hash: 73a9f665ec0ecfc0
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
    const MJMINVAL: f64 = 1e-15;

    // SAFETY: m, d, point, body, weight are valid pointers (caller contract)
    unsafe {
        // get flex info
        let dim = *(*m).flex_dim.add(f as usize);
        let edata = (*m).flex_elem.add((*(*m).flex_elemdataadr.add(f as usize) + e * (dim + 1)) as usize);
        let vert = (*d).flexvert_xpos.add(3 * (*(*m).flex_vertadr.add(f as usize)) as usize);

        // compute inverse distances, save body ids, find vertex v
        let mut vid: i32 = -1;
        for i in 0..=dim {
            let ei = *edata.add(i as usize);
            let dist = crate::engine::engine_util_blas::mju_dist3(point, vert.add(3 * ei as usize));
            let denom = if dist > MJMINVAL { dist } else { MJMINVAL };
            *weight.add(i as usize) = 1.0 / denom;
            *body.add(i as usize) = *(*m).flex_vertadr.add(f as usize) + ei;

            // check if element vertex matches v
            if ei == v {
                vid = i;
            }
        }

        // v found in e: skip and shift remaining
        let mut dim_out = dim;
        if vid >= 0 {
            let mut j = vid;
            while j < dim_out {
                *weight.add(j as usize) = *weight.add((j + 1) as usize);
                *body.add(j as usize) = *body.add((j + 1) as usize);
                j += 1;
            }
            dim_out -= 1;
        }

        // normalize weights
        let sum = crate::engine::engine_util_blas::mju_sum(weight, dim_out + 1);
        if sum < MJMINVAL {
            crate::engine::engine_util_errmem::mju_error(
                b"element body weight sum < mjMINVAL\0".as_ptr() as *const i8);
        }
        crate::engine::engine_util_blas::mju_scl(weight, weight, 1.0 / sum, dim_out + 1);

        dim_out + 1
    }
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
    // SAFETY: caller guarantees all pointers valid with proper sizes
    unsafe {
        if nw == 0 {
            return 0;
        }

        // determine sign
        let sign: f64 = if *vweight.add(0) < 0.0 { -1.0 } else { 1.0 };

        // compute parametric coordinates using absolute weights
        let mut coord: [f64; 3] = [0.0, 0.0, 0.0];
        for i in 0..nw as usize {
            crate::engine::engine_util_blas::mju_add_to_scl3(
                coord.as_mut_ptr(),
                (*m).flex_vert0.add(3 * *v.add(i) as usize),
                (*vweight.add(i)).abs());
        }

        let interp = *(*m).flex_interp.add(f as usize);
        let order = if interp < 0 { -interp } else { interp };
        let npc = (order + 1) * (order + 1) * (order + 1);

        // grid dimensions for shell mode
        let mut nx: i32 = 0;
        let mut ny: i32 = 0;
        let mut nz: i32 = 0;
        if interp < 0 {
            nx = *(*m).flex_cellnum.add(3 * f as usize + 0) * order + 1;
            ny = *(*m).flex_cellnum.add(3 * f as usize + 1) * order + 1;
            nz = *(*m).flex_cellnum.add(3 * f as usize + 2) * order + 1;
        }

        // cell lookup
        let mut local: [f64; 3] = [0.0; 3];
        let mut nodeindices: [i32; 27] = [0; 27];
        crate::engine::engine_util_misc::mju_cell_lookup(
            coord.as_ptr(), (*m).flex_cellnum.add(3 * f as usize),
            order, local.as_mut_ptr(), nodeindices.as_mut_ptr());

        let nstart = *(*m).flex_nodeadr.add(f as usize);
        let mut nb: i32 = 0;

        if (*m).flex_nodebodyid.is_null() {
            return 0;
        }

        if npc > 27 {
            for j in 0..npc {
                let w = crate::engine::engine_util_misc::mju_eval_basis(local.as_ptr(), j, order);
                if w < 1e-5 {
                    continue;
                }
                let idx = nodeindices[j as usize];

                // shell mode
                if interp < 0 {
                    let k_idx = idx % nz;
                    let rest = idx / nz;
                    let j_idx = rest % ny;
                    let i_idx = rest / ny;
                    if i_idx > 0 && i_idx < nx - 1 && j_idx > 0 && j_idx < ny - 1 && k_idx > 0 && k_idx < nz - 1 {
                        crate::engine::engine_util_misc::mju_shell_tfi_weights(
                            nx, ny, nz, i_idx, j_idx, k_idx, sign * w,
                            &mut nb, body, bweight, (*m).flex_nodebodyid, nstart);
                        continue;
                    }
                }

                // add node, check for duplicates
                let b = *(*m).flex_nodebodyid.add((nstart + idx) as usize);
                let mut found = false;
                for k in 0..nb as usize {
                    if *body.add(k) == b {
                        if !bweight.is_null() { *bweight.add(k) += sign * w; }
                        found = true;
                        break;
                    }
                }
                if !found {
                    if !bweight.is_null() { *bweight.add(nb as usize) = sign * w; }
                    *body.add(nb as usize) = b;
                    nb += 1;
                }
            }
        } else {
            let mut basis: [f64; 27] = [0.0; 27];
            crate::engine::engine_util_misc::mju_eval_basis_array(basis.as_mut_ptr(), local.as_ptr(), order);

            for j in 0..npc {
                let w = basis[j as usize];
                if w < 1e-5 {
                    continue;
                }
                let idx = nodeindices[j as usize];

                // shell mode
                if interp < 0 {
                    let k_idx = idx % nz;
                    let rest = idx / nz;
                    let j_idx = rest % ny;
                    let i_idx = rest / ny;
                    if i_idx > 0 && i_idx < nx - 1 && j_idx > 0 && j_idx < ny - 1 && k_idx > 0 && k_idx < nz - 1 {
                        crate::engine::engine_util_misc::mju_shell_tfi_weights(
                            nx, ny, nz, i_idx, j_idx, k_idx, sign * w,
                            &mut nb, body, bweight, (*m).flex_nodebodyid, nstart);
                        continue;
                    }
                }

                // add node, check for duplicates
                let b = *(*m).flex_nodebodyid.add((nstart + idx) as usize);
                let mut found = false;
                for k in 0..nb as usize {
                    if *body.add(k) == b {
                        if !bweight.is_null() { *bweight.add(k) += sign * w; }
                        found = true;
                        break;
                    }
                }
                if !found {
                    if !bweight.is_null() { *bweight.add(nb as usize) = sign * w; }
                    *body.add(nb as usize) = b;
                    nb += 1;
                }
            }
        }

        nb
    }
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
    // mjCNSTR: EQUALITY=0, FRICTION_DOF=1, FRICTION_TENDON=2, LIMIT_JOINT=3, LIMIT_TENDON=4,
    //          CONTACT_FRICTIONLESS=5, CONTACT_PYRAMIDAL=6, CONTACT_ELLIPTIC=7
    // SAFETY: m, d, jac are valid; pos/margin/chain may be null (caller contract)
    unsafe {
        let nv = (*m).nv as i32;
        let nefc = (*d).nefc;
        let nnz = (*d).efc_J_rownnz;
        let adr = (*d).efc_J_rowadr;
        let ind = (*d).efc_J_colind;
        let J = (*d).efc_J;

        // init empty guard for constraints other than contact
        let mut empty: i32;
        if r#type == 5 || r#type == 6 || r#type == 7 {
            empty = 0;  // contact types
        } else {
            empty = 1;
        }

        // dense: copy entire Jacobian
        if crate::engine::engine_core_util::mj_is_sparse(m) == 0 {
            // make sure jac is not empty
            if empty != 0 {
                for i in 0..(size * nv) {
                    if *jac.add(i as usize) != 0.0 {
                        empty = 0;
                        break;
                    }
                }
            }

            // copy if not empty
            if empty == 0 {
                crate::engine::engine_util_blas::mju_copy(
                    J.add((nefc * nv) as usize), jac, size * nv);
            }
        }
        // sparse: copy chain
        else {
            // clamp NV
            let NV = if NV > 0 { NV } else { 0 };

            if NV != 0 {
                empty = 0;
            } else if empty != 0 {
                // all rows empty, return early
                return;
            }

            // chain required in sparse mode
            if NV != 0 && chain.is_null() {
                crate::engine::engine_util_errmem::mju_error(
                    b"called with dense arguments\0".as_ptr() as *const i8);
            }

            // process size elements
            for i in 0..size {
                // set row address
                *adr.add((nefc + i) as usize) = if nefc + i > 0 {
                    *adr.add((nefc + i - 1) as usize) + *nnz.add((nefc + i - 1) as usize)
                } else { 0 };

                // set row descriptor
                *nnz.add((nefc + i) as usize) = NV;

                // copy if not empty
                if NV != 0 {
                    crate::engine::engine_util_misc::mju_copy_int(
                        ind.add(*adr.add((nefc + i) as usize) as usize), chain, NV);
                    crate::engine::engine_util_blas::mju_copy(
                        J.add(*adr.add((nefc + i) as usize) as usize), jac.add((i * NV) as usize), NV);
                }
            }

            // set J row supernodes
            // cross-boundary: does previous row have same pattern?
            if nefc > 0 && NV == *nnz.add((nefc - 1) as usize) &&
               (NV == 0 || crate::engine::engine_util_sparse::mju_compare(
                   ind.add(*adr.add(nefc as usize) as usize),
                   ind.add(*adr.add((nefc - 1) as usize) as usize), NV) != 0) {
                *(*d).efc_J_rowsuper.add((nefc - 1) as usize) = 1;
            }

            // within-constraint: consecutive rows share same pattern
            crate::engine::engine_util_misc::mju_fill_int(
                (*d).efc_J_rowsuper.add(nefc as usize), 1, size - 1);
            *(*d).efc_J_rowsuper.add((nefc + size - 1) as usize) = 0;
        }

        // all rows empty: skip constraint
        if empty != 0 {
            return;
        }

        // set constraint pos, margin, frictionloss, type, id
        for i in 0..size {
            *(*d).efc_pos.add((nefc + i) as usize) = if !pos.is_null() { *pos.add(i as usize) } else { 0.0 };
            *(*d).efc_margin.add((nefc + i) as usize) = if !margin.is_null() { *margin.add(i as usize) } else { 0.0 };
            *(*d).efc_frictionloss.add((nefc + i) as usize) = frictionloss;
            *(*d).efc_type.add((nefc + i) as usize) = r#type;
            *(*d).efc_id.add((nefc + i) as usize) = id;
        }

        // increase counters
        (*d).nefc += size;
        if r#type == 0 {  // mjCNSTR_EQUALITY
            (*d).ne += size;
        } else if r#type == 1 || r#type == 2 {  // FRICTION_DOF or FRICTION_TENDON
            (*d).nf += size;
        } else if r#type == 3 || r#type == 4 {  // LIMIT_JOINT or LIMIT_TENDON
            (*d).nl += size;
        }
    }
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
    // SAFETY: all pointers are valid sparse matrix arrays (caller contract)
    unsafe {
        for r in 0..nefc as usize {
            // init row
            let end = *Y_rowadr.add(r) + *Y_rownnz.add(r);
            let adrJ = *J_rowadr.add(r);
            let mut remainJ = *J_rownnz.add(r);
            let mut nnzY: i32 = 0;

            // complete chain in reverse
            loop {
                // get previous dof in src and dst
                let prev_src = if remainJ > 0 {
                    *J_colind.add((adrJ + remainJ - 1) as usize)
                } else { -1 };
                let prev_dst = if nnzY > 0 {
                    *dof_parentid.add(*Y_colind.add((end - nnzY) as usize) as usize)
                } else { -1 };

                // both finished: break
                if prev_src < 0 && prev_dst < 0 {
                    break;
                }
                // add src
                else if prev_src >= prev_dst {
                    nnzY += 1;
                    remainJ -= 1;
                    *Y_colind.add((end - nnzY) as usize) = prev_src;
                    *Y.add((end - nnzY) as usize) = *J.add((adrJ + remainJ) as usize);
                }
                // add dst
                else {
                    nnzY += 1;
                    *Y_colind.add((end - nnzY) as usize) = prev_dst;
                    *Y.add((end - nnzY) as usize) = 0.0;
                }
            }

            // compare with Y_rownnz: SHOULD NOT OCCUR
            if nnzY != *Y_rownnz.add(r) {
                crate::engine::engine_util_errmem::mju_error(
                    b"pre and post-count of Y_rownnz are not equal on row %d\0".as_ptr() as *const i8);
            }
        }
    }
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
    // SAFETY: caller guarantees m, d are valid; res, vec point to valid arrays
    unsafe {
        // exit if no constraints
        if (*d).nefc == 0 {
            return;
        }

        // sparse Jacobian
        if crate::engine::engine_core_util::mj_is_sparse(m) != 0 {
            crate::engine::engine_util_sparse::mju_mul_mat_vec_sparse(
                res, (*d).efc_J, vec, (*d).nefc,
                (*d).efc_J_rownnz, (*d).efc_J_rowadr,
                (*d).efc_J_colind, (*d).efc_J_rowsuper,
            );
        }
        // dense Jacobian
        else {
            crate::engine::engine_util_blas::mju_mul_mat_vec(
                res, (*d).efc_J, vec, (*d).nefc, (*m).nv as i32,
            );
        }
    }
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
    const MJ_MINVAL: f64 = 1E-15;
    const MJ_NREF: usize = 2;
    const MJ_NIMP: usize = 5;
    const MJ_CNSTR_FRICTION_DOF: i32 = 1;
    const MJ_CNSTR_FRICTION_TENDON: i32 = 2;
    const MJ_CNSTR_CONTACT_PYRAMIDAL: i32 = 6;
    const MJ_CNSTR_CONTACT_ELLIPTIC: i32 = 7;

    // SAFETY: caller guarantees m, d valid
    unsafe {
        let nefc = (*d).nefc;
        let R = (*d).efc_R;
        let KBIP = (*d).efc_KBIP;

        // set efc_R, efc_KBIP
        let mut i: i32 = 0;
        while i < nefc {
            let mut solref: [f64; MJ_NREF] = [0.0; MJ_NREF];
            let mut solreffriction: [f64; MJ_NREF] = [0.0; MJ_NREF];
            let mut solimp: [f64; MJ_NIMP] = [0.0; MJ_NIMP];
            let mut pos: f64 = 0.0;
            let mut dim: i32 = 0;
            let mut imp: f64 = 0.0;
            let mut impP: f64 = 0.0;

            getsolparam(m, d, i, solref.as_mut_ptr(), solreffriction.as_mut_ptr(), solimp.as_mut_ptr());
            getposdim(m, d, i, &mut pos, &mut dim);
            getimpedance(solimp.as_ptr(), pos, *(*d).efc_margin.add(i as usize), &mut imp, &mut impP);

            for j in 0..dim {
                let idx = (i + j) as usize;
                // R = (1-imp)/imp * diagApprox
                *R.add(idx) = crate::engine::engine_util_misc::mju_max(
                    MJ_MINVAL, (1.0 - imp) * *(*d).efc_diagA.add(idx) / imp);

                let tp = *(*d).efc_type.add(idx);
                let elliptic_friction = (tp == MJ_CNSTR_CONTACT_ELLIPTIC) && (j > 0);
                let r = if elliptic_friction && (solreffriction[0] != 0.0 || solreffriction[1] != 0.0) {
                    solreffriction.as_ptr()
                } else {
                    solref.as_ptr()
                };

                // friction: K = 0
                if tp == MJ_CNSTR_FRICTION_DOF || tp == MJ_CNSTR_FRICTION_TENDON || elliptic_friction {
                    *KBIP.add(4 * idx) = 0.0;
                }
                // standard: K = 1 / (d_width^2 * timeconst^2 * dampratio^2)
                else if *r.add(0) > 0.0 {
                    *KBIP.add(4 * idx) = 1.0 / crate::engine::engine_util_misc::mju_max(
                        MJ_MINVAL, solimp[1] * solimp[1] * *r.add(0) * *r.add(0) * *r.add(1) * *r.add(1));
                }
                // direct: K = -solref[0] / d_width^2
                else {
                    *KBIP.add(4 * idx) = -*r.add(0) / crate::engine::engine_util_misc::mju_max(
                        MJ_MINVAL, solimp[1] * solimp[1]);
                }

                // standard: B = 2 / (d_width*timeconst)
                if *r.add(1) > 0.0 {
                    *KBIP.add(4 * idx + 1) = 2.0 / crate::engine::engine_util_misc::mju_max(
                        MJ_MINVAL, solimp[1] * *r.add(0));
                }
                // direct: B = -solref[1] / d_width
                else {
                    *KBIP.add(4 * idx + 1) = -*r.add(1) / crate::engine::engine_util_misc::mju_max(
                        MJ_MINVAL, solimp[1]);
                }

                // I = imp, P = imp'
                *KBIP.add(4 * idx + 2) = imp;
                *KBIP.add(4 * idx + 3) = impP;
            }

            i += dim;
        }

        // frictional contacts: adjust R in friction dimensions
        let mut i: i32 = (*d).ne + (*d).nf;
        while i < nefc {
            if *(*d).efc_type.add(i as usize) == MJ_CNSTR_CONTACT_PYRAMIDAL
                || *(*d).efc_type.add(i as usize) == MJ_CNSTR_CONTACT_ELLIPTIC
            {
                let id = *(*d).efc_id.add(i as usize) as usize;
                let dim = (*(*d).contact.add(id)).dim;
                let friction = (*(*d).contact.add(id)).friction.as_ptr();

                // set R[1] = R[0]/impratio
                *R.add(i as usize + 1) = *R.add(i as usize)
                    / crate::engine::engine_util_misc::mju_max(MJ_MINVAL, (*m).opt.impratio);

                // set mu
                (*(*d).contact.add(id)).mu = *friction.add(0)
                    * (*R.add(i as usize + 1) / *R.add(i as usize)).sqrt();

                // elliptic
                if *(*d).efc_type.add(i as usize) == MJ_CNSTR_CONTACT_ELLIPTIC {
                    for j in 1..(dim - 1) as usize {
                        *R.add(i as usize + j + 1) = *R.add(i as usize + 1)
                            * *friction.add(0) * *friction.add(0)
                            / (*friction.add(j) * *friction.add(j));
                    }
                    i += dim;
                }
                // pyramidal
                else {
                    let Rpy = 2.0 * (*(*d).contact.add(id)).mu * (*(*d).contact.add(id)).mu * *R.add(i as usize);
                    for j in 0..(2 * (dim - 1)) as usize {
                        *R.add(i as usize + j) = Rpy;
                    }
                    i += 2 * (dim - 1);
                }
            } else {
                i += 1;
            }
        }

        // set D = 1 / R
        for i in 0..nefc as usize {
            *(*d).efc_D.add(i) = 1.0 / *R.add(i);
        }

        // adjust diagA
        for i in 0..nefc as usize {
            *(*d).efc_diagA.add(i) = *R.add(i) * *KBIP.add(4 * i + 2) / (1.0 - *KBIP.add(4 * i + 2));
        }
    }
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
    // SAFETY: caller guarantees m, d, jar, cost are valid pointers with proper lifetimes
    unsafe {
        mj_constraint_update_impl(
            (*d).ne, (*d).nf, (*d).nefc,
            (*d).efc_D, (*d).efc_R, (*d).efc_frictionloss,
            jar, (*d).efc_type, (*d).efc_id, (*d).contact,
            (*d).efc_state, (*d).efc_force, cost, flg_coneHessian,
        );
        mj_mul_jac_t_vec(m, d as *const mjData, (*d).qfrc_constraint, (*d).efc_force);
    }
}

