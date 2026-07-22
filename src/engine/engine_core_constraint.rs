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
    // SAFETY: All pointers are valid arrays (caller contract). d is valid for stack alloc.
    unsafe {
        let nstart = (*m).flex_nodeadr.add(flex_id as usize);
        let bodyid = (*m).flex_nodebodyid.add(*nstart as usize);

        // build per-cell sparse chain: union of bodyChain for npc nodes
        *cell_nnz = 0;
        let dof_used: *mut i32 = crate::engine::engine_memory::mj_stack_alloc_int(d, nv as usize);
        let temp_chain: *mut i32 = crate::engine::engine_memory::mj_stack_alloc_int(d, nv as usize);
        crate::engine::engine_util_misc::mju_zero_int(dof_used, nv);
        for n in 0..npc {
            let temp_nnz = crate::engine::engine_core_util::mj_body_chain(
                m, *bodyid.add(*gindices.add(n as usize) as usize), temp_chain);
            for k in 0..temp_nnz {
                *dof_used.add(*temp_chain.add(k as usize) as usize) = 1;
            }
        }
        for q in 0..nv {
            if *dof_used.add(q as usize) != 0 {
                *cell_chain.add(*cell_nnz as usize) = q;
                *cell_nnz += 1;
            }
        }

        // build per-cell node Jacobians: 3*npc x cell_nnz
        let cell_node_jac: *mut f64 = crate::engine::engine_memory::mj_stack_alloc_num(
            d, (3 * npc * *cell_nnz) as usize);
        crate::engine::engine_util_blas::mju_zero(cell_node_jac, 3 * npc * *cell_nnz);
        let chain_col: *mut i32 = crate::engine::engine_memory::mj_stack_alloc_int(d, nv as usize);
        let blk_jac: *mut f64 = crate::engine::engine_memory::mj_stack_alloc_num(d, (3 * nv) as usize);
        for n in 0..npc {
            let body = *bodyid.add(*gindices.add(n as usize) as usize);
            let chain_n = crate::engine::engine_core_util::mj_body_chain(m, body, chain_col);
            crate::engine::engine_util_blas::mju_zero(blk_jac, 3 * chain_n);
            crate::engine::engine_core_util::mj_jac_sparse(
                m, d as *const crate::types::mjData, blk_jac, std::ptr::null_mut(),
                xpos_c.add((3 * n) as usize), body, chain_n, chain_col as *const i32, 0);
            // map node's sparse chain into cell_chain indexing
            for r in 0..3i32 {
                for k in 0..chain_n {
                    // find chain_col[k] in cell_chain via linear scan (chain is short)
                    for cc in 0..*cell_nnz {
                        if *cell_chain.add(cc as usize) == *chain_col.add(k as usize) {
                            *cell_node_jac.add(((3 * n + r) * *cell_nnz + cc) as usize) =
                                *blk_jac.add((r * chain_n + k) as usize);
                            break;
                        }
                    }
                }
            }
        }

        cell_node_jac
    }
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
    // SAFETY: m and d are valid pointers (caller contract).
    unsafe {
        // move arena pointer to end of contact array
        (*d).parena = (*d).ncon as usize * std::mem::size_of::<mjContact>();

        // Macro: allocate each solver arena pointer, bail on failure
        macro_rules! arena_alloc_field {
            ($field:ident, $ty:ty, $nr:expr) => {
                (*d).$field = crate::engine::engine_memory::mj_arena_alloc_byte(
                    d,
                    std::mem::size_of::<$ty>() * ($nr as usize),
                    std::mem::align_of::<$ty>(),
                ) as *mut $ty;
                if (*d).$field.is_null() {
                    crate::engine::engine_core_util::mj_warning(d, 1, (*d).narena as i32);
                    crate::engine::engine_memory::mj_clear_efc(d);
                    (*d).parena = (*d).ncon as usize * std::mem::size_of::<mjContact>();
                    return 0;
                }
            };
        }

        // MJDATA_ARENA_POINTERS_SOLVER expansion (X and XNV both expand the same)
        arena_alloc_field!(efc_type,          i32, (*d).nefc);
        arena_alloc_field!(efc_id,            i32, (*d).nefc);
        arena_alloc_field!(efc_J_rownnz,      i32, (*d).nefc);
        arena_alloc_field!(efc_J_rowadr,      i32, (*d).nefc);
        arena_alloc_field!(efc_J_rowsuper,    i32, (*d).nefc);
        arena_alloc_field!(efc_J_colind,      i32, (*d).nJ);
        arena_alloc_field!(efc_J,             f64, (*d).nJ);
        arena_alloc_field!(efc_pos,           f64, (*d).nefc);
        arena_alloc_field!(efc_margin,        f64, (*d).nefc);
        arena_alloc_field!(efc_frictionloss,  f64, (*d).nefc);
        arena_alloc_field!(efc_diagA,         f64, (*d).nefc);
        arena_alloc_field!(efc_KBIP,          f64, (*d).nefc * 4);
        arena_alloc_field!(efc_D,             f64, (*d).nefc);
        arena_alloc_field!(efc_R,             f64, (*d).nefc);
        arena_alloc_field!(tendon_efcadr,     i32, (*m).ntendon);
        arena_alloc_field!(efc_vel,           f64, (*d).nefc);
        arena_alloc_field!(efc_aref,          f64, (*d).nefc);
        arena_alloc_field!(efc_b,             f64, (*d).nefc);
        arena_alloc_field!(efc_state,         i32, (*d).nefc);
        arena_alloc_field!(efc_force,         f64, (*d).nefc);

        1
    }
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
    const mjDSBL_FRICTIONLOSS: i32 = 1 << 2;
    const mjENBL_SLEEP: i32 = 1 << 4;
    const mjCNSTR_FRICTION_DOF: i32 = 1;
    const mjCNSTR_FRICTION_TENDON: i32 = 2;

    // SAFETY: All pointers are valid (caller contract). d is valid for stack alloc.
    unsafe {
        let nv = (*m).nv as i32;
        let issparse = crate::engine::engine_core_util::mj_is_sparse(m);
        let mut nf: i32 = 0;
        let mut jac: *mut f64 = std::ptr::null_mut();

        // disabled: return
        if ((*m).opt.disableflags & mjDSBL_FRICTIONLOSS) != 0 {
            return 0;
        }

        // sleep filtering
        let sleep_filter = (((*m).opt.enableflags & mjENBL_SLEEP) != 0)
            && (*d).ntree_awake < (*m).ntree as i32;

        if count_only == 0 {
            crate::engine::engine_memory::mj_mark_stack(d);
            // allocate Jacobian
            jac = crate::engine::engine_memory::mj_stack_alloc_num(d, nv as usize);
        }

        // find frictional DOFs
        for i in 0..nv {
            // no friction loss: skip
            if *(*m).dof_frictionloss.add(i as usize) == 0.0 {
                continue;
            }

            // sleeping tree: skip
            if sleep_filter && crate::engine::engine_sleep::mj_sleep_state(
                m, d as *const crate::types::mjData, 14 /*mjOBJ_DOF*/, i) == 3 /*mjS_ASLEEP*/ {
                continue;
            }

            if count_only != 0 {
                nf += mj_add_constraint_count(m, 1, 1);
                if !nnz.is_null() {
                    *nnz += 1;
                }
            } else {
                // prepare Jacobian: sparse or dense
                if issparse != 0 {
                    *jac.add(0) = 1.0;
                } else {
                    crate::engine::engine_util_blas::mju_zero(jac, nv);
                    *jac.add(i as usize) = 1.0;
                }

                // add constraint
                mj_add_constraint(m, d, jac as *const f64, std::ptr::null(),
                    std::ptr::null(), *(*m).dof_frictionloss.add(i as usize),
                    1, mjCNSTR_FRICTION_DOF, i,
                    if issparse != 0 { 1 } else { 0 },
                    if issparse != 0 { &i as *const i32 }
                    else { std::ptr::null() });
            }
        }

        // find frictional tendons
        for i in 0..(*m).ntendon as i32 {
            if *(*m).tendon_frictionloss.add(i as usize) > 0.0 {
                if count_only != 0 {
                    nf += mj_add_constraint_count(m, 1, *(*m).ten_J_rownnz.add(i as usize));
                    if !nnz.is_null() {
                        *nnz += *(*m).ten_J_rownnz.add(i as usize);
                    }
                } else {
                    let efcadr = (*d).nefc;
                    // add constraint
                    if issparse != 0 {
                        mj_add_constraint(m, d,
                            (*d).ten_J.add(*(*m).ten_J_rowadr.add(i as usize) as usize) as *const f64,
                            std::ptr::null(), std::ptr::null(),
                            *(*m).tendon_frictionloss.add(i as usize),
                            1, mjCNSTR_FRICTION_TENDON, i,
                            *(*m).ten_J_rownnz.add(i as usize),
                            (*m).ten_J_colind.add(*(*m).ten_J_rowadr.add(i as usize) as usize) as *const i32);
                    } else {
                        crate::engine::engine_util_sparse::mju_sparse2dense(
                            jac, (*d).ten_J as *const f64, 1, nv,
                            ((*m).ten_J_rownnz as *const i32).add(i as usize),
                            ((*m).ten_J_rowadr as *const i32).add(i as usize),
                            (*m).ten_J_colind as *const i32);
                        mj_add_constraint(m, d, jac as *const f64,
                            std::ptr::null(), std::ptr::null(),
                            *(*m).tendon_frictionloss.add(i as usize),
                            1, mjCNSTR_FRICTION_TENDON, i, 0, std::ptr::null());
                    }
                    // set tendon_efcadr
                    if *(*d).tendon_efcadr.add(i as usize) == -1 {
                        *(*d).tendon_efcadr.add(i as usize) = efcadr;
                    }
                }
            }
        }

        if count_only == 0 {
            crate::engine::engine_memory::mj_free_stack(d);
        }

        nf
    }
}

/// C: mj_instantiateLimit (engine/engine_core_constraint.c:1360)
/// Calls: mj_addConstraint, mj_addConstraintCount, mj_freeStack, mj_isSparse, mj_markStack, mj_sleepState, mj_stackAllocInfo, mju_max, mju_normalize3, mju_normalize4, mju_quat2Vel, mju_scl, mju_scl3, mju_sparse2dense, mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mj_instantiate_limit(m: *const mjModel, d: *mut mjData, count_only: i32, nnz: *mut i32) -> i32 {
    const mjDSBL_LIMIT: i32 = 1 << 3;
    const mjENBL_SLEEP: i32 = 1 << 4;
    const mjJNT_FREE: i32 = 0;
    const mjJNT_BALL: i32 = 1;
    const mjJNT_SLIDE: i32 = 2;
    const mjJNT_HINGE: i32 = 3;
    const mjCNSTR_LIMIT_JOINT: i32 = 3;
    const mjCNSTR_LIMIT_TENDON: i32 = 4;
    const mjS_ASLEEP: i32 = 3;

    // SAFETY: All pointers are valid (caller contract). d is valid for stack alloc.
    unsafe {
        let nv = (*m).nv as i32;
        let issparse = crate::engine::engine_core_util::mj_is_sparse(m);
        let mut nl: i32 = 0;
        let mut jac: *mut f64 = std::ptr::null_mut();

        // disabled: return
        if ((*m).opt.disableflags & mjDSBL_LIMIT) != 0 {
            return 0;
        }

        // sleep filtering
        let sleep_filter = (((*m).opt.enableflags & mjENBL_SLEEP) != 0)
            && (*d).ntree_awake < (*m).ntree as i32;

        if count_only == 0 {
            crate::engine::engine_memory::mj_mark_stack(d);
            jac = crate::engine::engine_memory::mj_stack_alloc_num(d, nv as usize);
        }

        // find joint limits
        for i in 0..(*m).njnt as i32 {
            // no limit: skip
            if !*(*m).jnt_limited.add(i as usize) {
                continue;
            }

            // sleeping tree: skip
            if sleep_filter && crate::engine::engine_sleep::mj_sleep_state(
                m, d as *const crate::types::mjData, 3 /*mjOBJ_JOINT*/, i) == mjS_ASLEEP {
                continue;
            }

            // get margin
            let margin = *(*m).jnt_margin.add(i as usize);

            // HINGE or SLIDE joint
            if *(*m).jnt_type.add(i as usize) == mjJNT_SLIDE
                || *(*m).jnt_type.add(i as usize) == mjJNT_HINGE
            {
                // get joint value
                let value = *(*d).qpos.add(*(*m).jnt_qposadr.add(i as usize) as usize);

                // process lower and upper limits
                let mut side: i32 = -1;
                while side <= 1 {
                    // compute distance (negative: penetration)
                    let mut dist = (side as f64)
                        * (*(*m).jnt_range.add((2 * i + (side + 1) / 2) as usize) - value);

                    // detect joint limit
                    if dist < margin {
                        if count_only != 0 {
                            nl += mj_add_constraint_count(m, 1, 1);
                            if !nnz.is_null() {
                                *nnz += 1;
                            }
                        } else {
                            // prepare Jacobian: sparse or dense
                            if issparse != 0 {
                                *jac.add(0) = -(side as f64);
                            } else {
                                crate::engine::engine_util_blas::mju_zero(jac, nv);
                                *jac.add(*(*m).jnt_dofadr.add(i as usize) as usize) = -(side as f64);
                            }

                            // add constraint
                            mj_add_constraint(m, d, jac as *const f64,
                                &dist as *const f64, &margin as *const f64, 0.0,
                                1, mjCNSTR_LIMIT_JOINT, i,
                                if issparse != 0 { 1 } else { 0 },
                                if issparse != 0 { (*m).jnt_dofadr.add(i as usize) as *const i32 }
                                else { std::ptr::null() });
                        }
                    }
                    side += 2;
                }
            }
            // BALL joint
            else if *(*m).jnt_type.add(i as usize) == mjJNT_BALL {
                // convert joint quaternion to axis-angle
                let adr = *(*m).jnt_qposadr.add(i as usize) as usize;
                let mut quat: [f64; 4] = [
                    *(*d).qpos.add(adr),
                    *(*d).qpos.add(adr + 1),
                    *(*d).qpos.add(adr + 2),
                    *(*d).qpos.add(adr + 3),
                ];
                crate::engine::engine_util_blas::mju_normalize4(quat.as_mut_ptr());
                let mut angleAxis: [f64; 3] = [0.0; 3];
                crate::engine::engine_util_spatial::mju_quat2vel(
                    angleAxis.as_mut_ptr(), quat.as_ptr(), 1.0);

                // get rotation angle, normalize
                let value = crate::engine::engine_util_blas::mju_normalize3(angleAxis.as_mut_ptr());

                // compute distance, using max of range (negative: penetration)
                let mut dist = crate::engine::engine_util_misc::mju_max(
                    *(*m).jnt_range.add((2 * i as i32) as usize),
                    *(*m).jnt_range.add((2 * i as i32 + 1) as usize)) - value;

                // detect joint limit
                if dist < margin {
                    if count_only != 0 {
                        nl += mj_add_constraint_count(m, 1, 3);
                        if !nnz.is_null() {
                            *nnz += 3;
                        }
                    }
                    // sparse
                    else if issparse != 0 {
                        // prepare dof index array
                        let chain_arr: [i32; 3] = [
                            *(*m).jnt_dofadr.add(i as usize),
                            *(*m).jnt_dofadr.add(i as usize) + 1,
                            *(*m).jnt_dofadr.add(i as usize) + 2,
                        ];

                        // prepare Jacobian
                        crate::engine::engine_util_blas::mju_scl3(
                            jac, angleAxis.as_ptr(), -1.0);

                        // add constraint
                        mj_add_constraint(m, d, jac as *const f64,
                            &dist as *const f64, &margin as *const f64, 0.0,
                            1, mjCNSTR_LIMIT_JOINT, i, 3, chain_arr.as_ptr());
                    }
                    // dense
                    else {
                        // prepare Jacobian
                        crate::engine::engine_util_blas::mju_zero(jac, nv);
                        crate::engine::engine_util_blas::mju_scl3(
                            jac.add(*(*m).jnt_dofadr.add(i as usize) as usize),
                            angleAxis.as_ptr(), -1.0);

                        // add constraint
                        mj_add_constraint(m, d, jac as *const f64,
                            &dist as *const f64, &margin as *const f64, 0.0,
                            1, mjCNSTR_LIMIT_JOINT, i, 0, std::ptr::null());
                    }
                }
            }
        }

        // find tendon limits
        for i in 0..(*m).ntendon as i32 {
            if !*(*m).tendon_limited.add(i as usize) {
                continue;
            }

            // get value = length, margin
            let value = *(*d).ten_length.add(i as usize);
            let margin = *(*m).tendon_margin.add(i as usize);

            // process lower and upper limits
            let mut side: i32 = -1;
            while side <= 1 {
                // compute distance (negative: penetration)
                let mut dist = (side as f64)
                    * (*(*m).tendon_range.add((2 * i + (side + 1) / 2) as usize) - value);

                // detect tendon limit
                if dist < margin {
                    if count_only != 0 {
                        nl += mj_add_constraint_count(m, 1, *(*m).ten_J_rownnz.add(i as usize));
                        if !nnz.is_null() {
                            *nnz += *(*m).ten_J_rownnz.add(i as usize);
                        }
                    } else {
                        // prepare Jacobian
                        let efcadr = (*d).nefc;
                        if issparse != 0 {
                            crate::engine::engine_util_blas::mju_scl(
                                jac,
                                (*d).ten_J.add(*(*m).ten_J_rowadr.add(i as usize) as usize) as *const f64,
                                -(side as f64),
                                *(*m).ten_J_rownnz.add(i as usize));
                            mj_add_constraint(m, d, jac as *const f64,
                                &dist as *const f64, &margin as *const f64, 0.0,
                                1, mjCNSTR_LIMIT_TENDON, i,
                                *(*m).ten_J_rownnz.add(i as usize),
                                (*m).ten_J_colind.add(*(*m).ten_J_rowadr.add(i as usize) as usize) as *const i32);
                        } else {
                            crate::engine::engine_util_sparse::mju_sparse2dense(
                                jac, (*d).ten_J as *const f64, 1, nv,
                                ((*m).ten_J_rownnz as *const i32).add(i as usize),
                                ((*m).ten_J_rowadr as *const i32).add(i as usize),
                                (*m).ten_J_colind as *const i32);
                            crate::engine::engine_util_blas::mju_scl(
                                jac, jac as *const f64, -(side as f64), nv);
                            mj_add_constraint(m, d, jac as *const f64,
                                &dist as *const f64, &margin as *const f64, 0.0,
                                1, mjCNSTR_LIMIT_TENDON, i, 0, std::ptr::null());
                        }
                        // set tendon_efcadr
                        if *(*d).tendon_efcadr.add(i as usize) == -1 {
                            *(*d).tendon_efcadr.add(i as usize) = efcadr;
                        }
                    }
                }
                side += 2;
            }
        }

        if count_only == 0 {
            crate::engine::engine_memory::mj_free_stack(d);
        }

        nl
    }
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
    // SAFETY: All pointers are valid arrays (caller contract). d is valid for stack alloc.
    unsafe {
        let nv = (*m).nv as i32;
        let mut NV: i32;

        crate::engine::engine_memory::mj_mark_stack(d);
        let bodychain: *mut i32 = crate::engine::engine_memory::mj_stack_alloc_int(d, nv as usize);
        let tempchain: *mut i32 = crate::engine::engine_memory::mj_stack_alloc_int(d, nv as usize);

        // set first
        NV = crate::engine::engine_core_util::mj_body_chain(m, *body.add(0), chain);

        // accumulate remaining
        for i in 1..n {
            // get body chain
            let bodyNV = crate::engine::engine_core_util::mj_body_chain(
                m, *body.add(i as usize), bodychain);
            if bodyNV == 0 {
                continue;
            }

            // accumulate chains
            NV = crate::engine::engine_util_sparse::mju_add_chains(
                tempchain, nv, NV, bodyNV, chain as *const i32, bodychain as *const i32);
            if NV != 0 {
                crate::engine::engine_util_misc::mju_copy_int(chain, tempchain as *const i32, NV);
            }
        }

        crate::engine::engine_memory::mj_free_stack(d);
        NV
    }
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
    const SIZEOF_MJCONTACT: usize = 576;
    const ALIGNOF_MJCONTACT: usize = 8;

    // SAFETY: m, d, con are valid pointers (caller contract)
    unsafe {
        // move arena pointer back to end of existing contact array, invalidate efc_ arrays
        (*d).parena = (*d).ncon as usize * SIZEOF_MJCONTACT;
        crate::engine::engine_memory::mj_clear_efc(d);

        // copy contact
        let dst = crate::engine::engine_memory::mj_arena_alloc_byte(
            d, SIZEOF_MJCONTACT, ALIGNOF_MJCONTACT) as *mut mjContact;
        if dst.is_null() {
            crate::engine::engine_core_util::mj_warning(d, 1, (*d).ncon);  // mjWARN_CONTACTFULL=1
            return 1;
        }
        *dst = *con;

        // increase counter
        (*d).ncon += 1;
        0
    }
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
    const MJCNSTR_EQUALITY: i32 = 0;
    const MJCNSTR_FRICTION_DOF: i32 = 1;
    const MJCNSTR_FRICTION_TENDON: i32 = 2;
    const MJCNSTR_LIMIT_JOINT: i32 = 3;
    const MJCNSTR_LIMIT_TENDON: i32 = 4;
    const MJCNSTR_CONTACT_FRICTIONLESS: i32 = 5;
    const MJCNSTR_CONTACT_PYRAMIDAL: i32 = 6;
    const MJCNSTR_CONTACT_ELLIPTIC: i32 = 7;

    const MJEQ_CONNECT: i32 = 0;
    const MJEQ_WELD: i32 = 1;
    const MJEQ_JOINT: i32 = 2;
    const MJEQ_TENDON: i32 = 3;
    const MJEQ_FLEX: i32 = 4;
    const MJEQ_FLEXVERT: i32 = 5;
    const MJEQ_FLEXSTRAIN: i32 = 6;

    const MJNEQDATA: i32 = 11;

    // SAFETY: m, d are valid pointers (caller contract). All array accesses are within
    // model-allocated bounds guaranteed by mujoco's constraint construction.
    unsafe {
        let nefc = (*d).nefc;
        let dA = (*d).efc_diagA;
        let mut weldcnt: i32 = 0;

        // loop over all constraints, compute approximate inverse inertia
        let mut i: i32 = 0;
        while i < nefc {
            // get constraint id
            let id = *(*d).efc_id.add(i as usize);

            // process according to constraint type
            let efc_type = *(*d).efc_type.add(i as usize);
            if efc_type == MJCNSTR_EQUALITY {
                // process according to equality-constraint type
                let eq_type = *(*m).eq_type.add(id as usize);
                if eq_type == MJEQ_CONNECT {
                    let mut b1 = *(*m).eq_obj1id.add(id as usize);
                    let mut b2 = *(*m).eq_obj2id.add(id as usize);

                    // get body ids if using site semantics
                    if *(*m).eq_objtype.add(id as usize) == 6 { // mjOBJ_SITE
                        b1 = *(*m).site_bodyid.add(b1 as usize);
                        b2 = *(*m).site_bodyid.add(b2 as usize);
                    }

                    // body translation
                    *dA.add(i as usize) = *(*m).body_invweight0.add(2 * b1 as usize)
                        + *(*m).body_invweight0.add(2 * b2 as usize);
                } else if eq_type == MJEQ_WELD {
                    let mut b1 = *(*m).eq_obj1id.add(id as usize);
                    let mut b2 = *(*m).eq_obj2id.add(id as usize);

                    // get body ids if using site semantics
                    if *(*m).eq_objtype.add(id as usize) == 6 { // mjOBJ_SITE
                        b1 = *(*m).site_bodyid.add(b1 as usize);
                        b2 = *(*m).site_bodyid.add(b2 as usize);
                    }

                    // body translation or rotation depending on weldcnt
                    let offset = if weldcnt > 2 { 1 } else { 0 };
                    *dA.add(i as usize) = *(*m).body_invweight0.add((2 * b1 + offset) as usize)
                        + *(*m).body_invweight0.add((2 * b2 + offset) as usize);
                    weldcnt = (weldcnt + 1) % 6;
                } else if eq_type == MJEQ_JOINT || eq_type == MJEQ_TENDON {
                    // object 1 contribution
                    if eq_type == MJEQ_JOINT {
                        *dA.add(i as usize) = *(*m).dof_invweight0.add(
                            *(*m).jnt_dofadr.add(*(*m).eq_obj1id.add(id as usize) as usize) as usize);
                    } else {
                        *dA.add(i as usize) = *(*m).tendon_invweight0.add(
                            *(*m).eq_obj1id.add(id as usize) as usize);
                    }

                    // add object 2 contribution if present
                    if *(*m).eq_obj2id.add(id as usize) >= 0 {
                        if eq_type == MJEQ_JOINT {
                            *dA.add(i as usize) += *(*m).dof_invweight0.add(
                                *(*m).jnt_dofadr.add(*(*m).eq_obj2id.add(id as usize) as usize) as usize);
                        } else {
                            *dA.add(i as usize) += *(*m).tendon_invweight0.add(
                                *(*m).eq_obj2id.add(id as usize) as usize);
                        }
                    }
                } else if eq_type == MJEQ_FLEX {
                    // process all non-rigid edges for this flex
                    let f = *(*m).eq_obj1id.add(id as usize);
                    let flex_edgeadr = *(*m).flex_edgeadr.add(f as usize);
                    let flex_edgenum = *(*m).flex_edgenum.add(f as usize);
                    for e in flex_edgeadr..(flex_edgeadr + flex_edgenum) {
                        if !*(*m).flexedge_rigid.add(e as usize) {
                            *dA.add(i as usize) = *(*m).flexedge_invweight0.add(e as usize);
                            i += 1;
                        }
                    }
                    // adjust constraint counter
                    i -= 1;
                } else if eq_type == MJEQ_FLEXVERT {
                    // process all vertices for this flex
                    let f = *(*m).eq_obj1id.add(id as usize);
                    let vertadr = *(*m).flex_vertadr.add(f as usize);
                    let vertnum = *(*m).flex_vertnum.add(f as usize);
                    for v in vertadr..(vertadr + vertnum) {
                        let bodyid = *(*m).flex_vertbodyid.add(v as usize);
                        *dA.add(i as usize) = *(*m).body_invweight0.add(2 * bodyid as usize);
                        i += 1;
                        *dA.add(i as usize) = *(*m).body_invweight0.add(2 * bodyid as usize);
                        i += 1;
                    }
                    // adjust constraint counter
                    i -= 1;
                } else if eq_type == MJEQ_FLEXSTRAIN {
                    // strain constraints: use avg inv weight of element's nodes
                    let flex_id = *(*m).eq_obj1id.add(id as usize);
                    let nstart = *(*m).flex_nodeadr.add(flex_id as usize);
                    let interp = *(*m).flex_interp.add(flex_id as usize);
                    let order = if interp < 0 { -interp } else { interp };
                    let is_shell = interp < 0;

                    let cx = *(*m).flex_cellnum.add(3 * flex_id as usize);
                    let cy = *(*m).flex_cellnum.add(3 * flex_id as usize + 1);
                    let cz = *(*m).flex_cellnum.add(3 * flex_id as usize + 2);

                    // nodes per element
                    let npe: i32;
                    let elem_idx: i32;
                    if is_shell {
                        npe = (order + 1) * (order + 1);
                        elem_idx = *(*m).eq_data.add(MJNEQDATA as usize * id as usize) as i32;
                    } else {
                        npe = (order + 1) * (order + 1) * (order + 1);
                        let ci_cell = *(*m).eq_data.add(MJNEQDATA as usize * id as usize) as i32;
                        let cj_cell = *(*m).eq_data.add(MJNEQDATA as usize * id as usize + 1) as i32;
                        let ck_cell = *(*m).eq_data.add(MJNEQDATA as usize * id as usize + 2) as i32;
                        elem_idx = ci_cell * cy * cz + cj_cell * cz + ck_cell;
                    }

                    // read neig from flex_stiffness
                    let ndof_elem = 3 * npe;
                    let k_elem = (*m).flex_stiffness.add(
                        (*(*m).flex_stiffnessadr.add(flex_id as usize)
                            + elem_idx * ndof_elem * ndof_elem) as usize);
                    let nconstraint = *k_elem as i32;

                    // get element node indices
                    let mut gindices: [i32; 125] = [0; 125];
                    if is_shell {
                        crate::engine::engine_util_misc::mju_flex_gather_face_state(
                            order, cx, cy, cz, elem_idx,
                            std::ptr::null(), std::ptr::null(), std::ptr::null(),
                            std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut(),
                            gindices.as_mut_ptr(), std::ptr::null_mut(),
                        );
                    } else {
                        let ci_cell = *(*m).eq_data.add(MJNEQDATA as usize * id as usize) as i32;
                        let cj_cell = *(*m).eq_data.add(MJNEQDATA as usize * id as usize + 1) as i32;
                        let ck_cell = *(*m).eq_data.add(MJNEQDATA as usize * id as usize + 2) as i32;
                        crate::engine::engine_util_misc::mju_flex_gather_cell_state(
                            order, cy, cz, ci_cell, cj_cell, ck_cell,
                            std::ptr::null(), std::ptr::null(), std::ptr::null(),
                            std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut(),
                            gindices.as_mut_ptr(), std::ptr::null_mut(),
                        );
                    }

                    let mut avg_invweight: f64 = 0.0;
                    for n in 0..npe {
                        let bodyid = *(*m).flex_nodebodyid.add(
                            (nstart + gindices[n as usize]) as usize);
                        avg_invweight += *(*m).body_invweight0.add(2 * bodyid as usize);
                    }
                    avg_invweight /= npe as f64;
                    for _c in 0..nconstraint {
                        *dA.add(i as usize) = avg_invweight;
                        i += 1;
                    }
                    // adjust constraint counter
                    i -= 1;
                } else {
                    crate::engine::engine_util_errmem::mju_error(
                        b"unknown constraint type\0".as_ptr() as *const i8);
                }
            } else if efc_type == MJCNSTR_FRICTION_DOF {
                *dA.add(i as usize) = *(*m).dof_invweight0.add(id as usize);
            } else if efc_type == MJCNSTR_LIMIT_JOINT {
                *dA.add(i as usize) = *(*m).dof_invweight0.add(
                    *(*m).jnt_dofadr.add(id as usize) as usize);
            } else if efc_type == MJCNSTR_FRICTION_TENDON || efc_type == MJCNSTR_LIMIT_TENDON {
                *dA.add(i as usize) = *(*m).tendon_invweight0.add(id as usize);
            } else if efc_type == MJCNSTR_CONTACT_FRICTIONLESS
                || efc_type == MJCNSTR_CONTACT_PYRAMIDAL
                || efc_type == MJCNSTR_CONTACT_ELLIPTIC
            {
                // get contact info
                let con = (*d).contact.add(id as usize);
                let dim = (*con).dim;

                // add the average translation and rotation components from both sides
                let mut tran: f64 = 0.0;
                let mut rot: f64 = 0.0;
                for side in 0..2i32 {
                    // get bodies and weights
                    let mut nb: i32 = 0;
                    let mut bid: [i32; 729] = [0; 729];
                    let mut bweight: [f64; 729] = [0.0; 729];

                    // geom
                    if (*con).geom[side as usize] >= 0 {
                        bid[0] = *(*m).geom_bodyid.add((*con).geom[side as usize] as usize);
                        bweight[0] = 1.0;
                        nb = 1;
                    }
                    // flex
                    else {
                        let mut nw: i32 = 0;
                        let mut vid: [i32; 4] = [0; 4];
                        let mut vweight: [f64; 4] = [0.0; 4];

                        // vert
                        if (*con).vert[side as usize] >= 0 {
                            vid[0] = *(*m).flex_vertadr.add((*con).flex[side as usize] as usize)
                                + (*con).vert[side as usize];
                            vweight[0] = 1.0;
                            nw = 1;
                        }
                        // elem
                        else {
                            nw = mj_elem_body_weight(
                                m, d as *const mjData,
                                (*con).flex[side as usize],
                                (*con).elem[side as usize],
                                (*con).vert[(1 - side) as usize],
                                (*con).pos.as_ptr(),
                                vid.as_mut_ptr(),
                                vweight.as_mut_ptr(),
                            );
                        }

                        // convert vertex ids and weights to body ids and weights
                        if *(*m).flex_interp.add((*con).flex[side as usize] as usize) == 0 {
                            for k in 0..nw {
                                bid[k as usize] = *(*m).flex_vertbodyid.add(vid[k as usize] as usize);
                                bweight[k as usize] = vweight[k as usize];
                                nb += 1;
                            }
                        } else {
                            nb += mj_vert_body_weight(
                                m, d as *const mjData,
                                (*con).flex[side as usize],
                                vid.as_mut_ptr(),
                                bid.as_mut_ptr().add(nb as usize),
                                bweight.as_mut_ptr().add(nb as usize),
                                vweight.as_ptr(),
                                nw,
                            );
                        }
                    }

                    // add weighted average over bodies
                    for k in 0..nb {
                        tran += *(*m).body_invweight0.add(2 * bid[k as usize] as usize)
                            * bweight[k as usize];
                        rot += *(*m).body_invweight0.add((2 * bid[k as usize] + 1) as usize)
                            * bweight[k as usize];
                    }
                }

                // set frictionless
                if efc_type == MJCNSTR_CONTACT_FRICTIONLESS {
                    *dA.add(i as usize) = tran;
                }
                // set elliptical
                else if efc_type == MJCNSTR_CONTACT_ELLIPTIC {
                    for j in 0..dim {
                        *dA.add((i + j) as usize) = if j < 3 { tran } else { rot };
                    }
                    // processed dim elements in one i-loop iteration; advance counter
                    i += dim - 1;
                }
                // set pyramidal
                else {
                    for j in 0..(dim - 1) {
                        let fri = (*con).friction[j as usize];
                        let val = tran + fri * fri * (if j < 2 { tran } else { rot });
                        *dA.add((i + 2 * j) as usize) = val;
                        *dA.add((i + 2 * j + 1) as usize) = val;
                    }
                    // processed 2*dim-2 elements in one i-loop iteration; advance counter
                    i += 2 * dim - 3;
                }
            }

            i += 1;
        }
    }
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

