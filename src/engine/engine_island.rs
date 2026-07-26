//! Port of: engine/engine_island.c
//! IR hash: 73393814548a07d1
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: clearIsland (engine/engine_island.c:39)
#[allow(unused_variables, non_snake_case)]
pub fn clear_island(d: *mut mjData, parena: usize) {
    // SAFETY: caller guarantees d is valid
    unsafe {
        (*d).tree_island = std::ptr::null_mut();
        (*d).island_ntree = std::ptr::null_mut();
        (*d).island_itreeadr = std::ptr::null_mut();
        (*d).map_itree2tree = std::ptr::null_mut();
        (*d).dof_island = std::ptr::null_mut();
        (*d).island_nv = std::ptr::null_mut();
        (*d).island_idofadr = std::ptr::null_mut();
        (*d).island_dofadr = std::ptr::null_mut();
        (*d).map_dof2idof = std::ptr::null_mut();
        (*d).map_idof2dof = std::ptr::null_mut();
        (*d).ifrc_smooth = std::ptr::null_mut();
        (*d).iacc_smooth = std::ptr::null_mut();
        (*d).iacc = std::ptr::null_mut();
        (*d).efc_island = std::ptr::null_mut();
        (*d).island_ne = std::ptr::null_mut();
        (*d).island_nf = std::ptr::null_mut();
        (*d).island_nefc = std::ptr::null_mut();
        (*d).island_iefcadr = std::ptr::null_mut();
        (*d).map_efc2iefc = std::ptr::null_mut();
        (*d).map_iefc2efc = std::ptr::null_mut();
        (*d).iefc_type = std::ptr::null_mut();
        (*d).iefc_id = std::ptr::null_mut();
        (*d).iefc_frictionloss = std::ptr::null_mut();
        (*d).iefc_D = std::ptr::null_mut();
        (*d).iefc_R = std::ptr::null_mut();
        (*d).iefc_aref = std::ptr::null_mut();
        (*d).iefc_state = std::ptr::null_mut();
        (*d).iefc_force = std::ptr::null_mut();
        (*d).ifrc_constraint = std::ptr::null_mut();

        (*d).nefc = 0;
        (*d).nisland = 0;
        (*d).nidof = 0;
        (*d).parena = parena;
    }
}

/// C: arenaAllocIsland (engine/engine_island.c:57)
/// Calls: clearIsland, mj_arenaAllocByte, mj_warning
#[allow(unused_variables, non_snake_case)]
pub fn arena_alloc_island(m: *const mjModel, d: *mut mjData) -> i32 {
    // SAFETY: m and d are valid pointers (caller contract). Arena allocation uses
    // mj_arenaAllocByte which returns null on failure.
    unsafe {
        let parena_old = (*d).parena;

        // Macro: allocate each island arena pointer, bail on failure
        macro_rules! arena_alloc_field {
            ($field:ident, $ty:ty, $nr:expr) => {
                (*d).$field = crate::engine::engine_memory::mj_arena_alloc_byte(
                    d,
                    std::mem::size_of::<$ty>() * ($nr as usize),
                    std::mem::align_of::<$ty>(),
                ) as *mut $ty;
                if (*d).$field.is_null() {
                    crate::engine::engine_core_util::mj_warning(d, 1, (*d).narena as i32);
                    clear_island(d, parena_old);
                    return 0;
                }
            };
        }

        // MJDATA_ARENA_POINTERS_ISLAND expansion
        arena_alloc_field!(tree_island,        i32, (*m).ntree);
        arena_alloc_field!(island_ntree,       i32, (*d).nisland);
        arena_alloc_field!(island_itreeadr,    i32, (*d).nisland);
        arena_alloc_field!(map_itree2tree,     i32, (*m).ntree);
        arena_alloc_field!(dof_island,         i32, (*m).nv);
        arena_alloc_field!(island_nv,          i32, (*d).nisland);
        arena_alloc_field!(island_idofadr,     i32, (*d).nisland);
        arena_alloc_field!(island_dofadr,      i32, (*d).nisland);
        arena_alloc_field!(map_dof2idof,       i32, (*m).nv);
        arena_alloc_field!(map_idof2dof,       i32, (*m).nv);
        arena_alloc_field!(ifrc_smooth,        f64, (*d).nidof);
        arena_alloc_field!(iacc_smooth,        f64, (*d).nidof);
        arena_alloc_field!(iacc,               f64, (*d).nidof);
        arena_alloc_field!(efc_island,         i32, (*d).nefc);
        arena_alloc_field!(island_ne,          i32, (*d).nisland);
        arena_alloc_field!(island_nf,          i32, (*d).nisland);
        arena_alloc_field!(island_nefc,        i32, (*d).nisland);
        arena_alloc_field!(island_iefcadr,     i32, (*d).nisland);
        arena_alloc_field!(map_efc2iefc,       i32, (*d).nefc);
        arena_alloc_field!(map_iefc2efc,       i32, (*d).nefc);
        arena_alloc_field!(iefc_type,          i32, (*d).nefc);
        arena_alloc_field!(iefc_id,            i32, (*d).nefc);
        arena_alloc_field!(iefc_frictionloss,  f64, (*d).nefc);
        arena_alloc_field!(iefc_D,             f64, (*d).nefc);
        arena_alloc_field!(iefc_R,             f64, (*d).nefc);
        arena_alloc_field!(iefc_aref,          f64, (*d).nefc);
        arena_alloc_field!(iefc_state,         i32, (*d).nefc);
        arena_alloc_field!(iefc_force,         f64, (*d).nefc);
        arena_alloc_field!(ifrc_constraint,    f64, (*d).nidof);

        1
    }
}

/// C: treeNext (engine/engine_island.c:149)
/// Calls: mj_isSparse
#[allow(unused_variables, non_snake_case)]
pub fn tree_next(m: *const mjModel, d: *const mjData, i: i32, iter: *mut mjTreeIter) -> i32 {
    // mjTreeIter layout (16 bytes): trees[2] at 0, jac_idx at 8, tree_prev at 12
    // SAFETY: m, d are valid; iter is valid mjTreeIter pointer.
    unsafe {
        let iter_base = iter as *mut i32;
        let trees_0 = *iter_base.add(0);

        // handle special cases: pre-calculated trees
        if trees_0 != -2 {
            // get first tree, queue up second tree, return first tree
            let tree = trees_0;
            *iter_base.add(0) = *iter_base.add(1);  // trees[0] = trees[1]
            *iter_base.add(1) = -2;                   // trees[1] = -2 (sentinel)
            return tree;
        }

        let jac_idx = *iter_base.add(2);

        // special case mode complete
        if jac_idx == -1 {
            return -2;
        }

        // generic scan mode
        let mut j = jac_idx;
        let mut tree_next_val: i32 = -2;
        let tree_prev = *iter_base.add(3);

        if crate::engine::engine_core_util::mj_is_sparse(m) != 0 {
            // sparse
            let rownnz = *(*d).efc_J_rownnz.add(i as usize);
            let colind = (*d).efc_J_colind.add(*(*d).efc_J_rowadr.add(i as usize) as usize);
            while j < rownnz {
                let tree_j = *(*m).dof_treeid.add(*colind.add(j as usize) as usize);
                if tree_j != tree_prev {
                    tree_next_val = tree_j;
                    break;
                }
                j += 1;
            }
        } else {
            // dense
            let nv = (*m).nv as i32;
            let J = (*d).efc_J.add(nv as usize * i as usize);
            while j < nv {
                if *J.add(j as usize) != 0.0 {
                    let tree_j = *(*m).dof_treeid.add(j as usize);
                    if tree_j != tree_prev {
                        tree_next_val = tree_j;
                        break;
                    }
                    // skip to end of tree's dof block
                    j = *(*m).tree_dofadr.add(tree_j as usize) + *(*m).tree_dofnum.add(tree_j as usize) - 1;
                }
                j += 1;
            }
        }

        // update iterator state
        *iter_base.add(2) = j;  // jac_idx = j
        if tree_next_val != -2 {
            *iter_base.add(3) = tree_next_val;  // tree_prev = tree_next
        }

        tree_next_val
    }
}

/// C: treeIterInit (engine/engine_island.c:212)
/// Calls: mju_message
#[allow(unused_variables, non_snake_case)]
pub fn tree_iter_init(m: *const mjModel, d: *const mjData, i: i32, iter: *mut mjTreeIter) {
    // mjTreeIter layout (16 bytes, align 4):
    //   trees[2]: i32 at offset 0 (sentinel -2 = empty)
    //   jac_idx:  i32 at offset 8 (-1 = disabled)
    //   tree_prev: i32 at offset 12
    use crate::types::*;

    // SAFETY: m, d are valid model/data pointers. i is a valid constraint index.
    //         iter is a valid mjTreeIter pointer (16 bytes).
    unsafe {
        let iter_base = iter as *mut i32;
        // iter->trees[0] = -2; iter->trees[1] = -2; iter->jac_idx = -1; iter->tree_prev = -1;
        *iter_base.add(0) = -2;
        *iter_base.add(1) = -2;
        *iter_base.add(2) = -1;  // jac_idx
        *iter_base.add(3) = -1;  // tree_prev

        let efc_type = *(*d).efc_type.add(i as usize);
        let efc_id   = *(*d).efc_id.add(i as usize);

        // joint friction
        if efc_type == mjtConstraint_mjCNSTR_FRICTION_DOF as i32 {
            *iter_base.add(0) = *(*m).dof_treeid.add(efc_id as usize);
        }
        // joint limit
        else if efc_type == mjtConstraint_mjCNSTR_LIMIT_JOINT as i32 {
            let dofadr = *(*m).jnt_dofadr.add(efc_id as usize);
            *iter_base.add(0) = *(*m).dof_treeid.add(dofadr as usize);
        }
        // contact
        else if efc_type == mjtConstraint_mjCNSTR_CONTACT_FRICTIONLESS as i32
            || efc_type == mjtConstraint_mjCNSTR_CONTACT_PYRAMIDAL as i32
            || efc_type == mjtConstraint_mjCNSTR_CONTACT_ELLIPTIC as i32
        {
            let contact = &*(*d).contact.add(efc_id as usize);
            let g1 = contact.geom[0];
            let g2 = contact.geom[1];

            // geom-geom contact
            if g1 >= 0 && g2 >= 0 {
                *iter_base.add(0) = *(*m).body_treeid.add(*(*m).geom_bodyid.add(g1 as usize) as usize);
                *iter_base.add(1) = *(*m).body_treeid.add(*(*m).geom_bodyid.add(g2 as usize) as usize);
                if *iter_base.add(0) < 0 && *iter_base.add(1) < 0 {
                    crate::engine::engine_util_errmem::mju_error(
                        b"contact is between two static bodies\0".as_ptr() as *const i8);
                }
            } else {
                // no shortcut for flex contacts: enable generic scan
                *iter_base.add(2) = 0;  // jac_idx = 0
            }
        }
        // connect or weld constraints
        else if efc_type == mjtConstraint_mjCNSTR_EQUALITY as i32
            && (*(*m).eq_type.add(efc_id as usize) == mjtEq_mjEQ_CONNECT as i32
                || *(*m).eq_type.add(efc_id as usize) == mjtEq_mjEQ_WELD as i32)
        {
            let mut b1 = *(*m).eq_obj1id.add(efc_id as usize);
            let mut b2 = *(*m).eq_obj2id.add(efc_id as usize);

            // get body ids if using site semantics
            if *(*m).eq_objtype.add(efc_id as usize) == mjtObj_mjOBJ_SITE as i32 {
                b1 = *(*m).site_bodyid.add(b1 as usize);
                b2 = *(*m).site_bodyid.add(b2 as usize);
            }

            *iter_base.add(0) = *(*m).body_treeid.add(b1 as usize);
            *iter_base.add(1) = *(*m).body_treeid.add(b2 as usize);
            if *iter_base.add(0) < 0 && *iter_base.add(1) < 0 {
                crate::engine::engine_util_errmem::mju_error(
                    b"equality is between two static bodies\0".as_ptr() as *const i8);
            }
        }
        // otherwise enable generic scan
        else {
            *iter_base.add(2) = 0;  // jac_idx = 0
        }
    }
}

/// C: findEdges (engine/engine_island.c:317)
/// Calls: addEdge, mju_message, mju_zeroInt, treeIterInit, treeNext
#[allow(unused_variables, non_snake_case)]
pub fn find_edges(m: *const mjModel, d: *const mjData, rownnz: *mut i32, colind: *mut i32, tree_tree: *mut u8, efc_tree: *mut i32, ntree: i32) -> i32 {
    todo!() // findEdges
}

/// C: mj_floodFill (engine/engine_island.h:28)
/// Calls: mju_copyInt, mju_fillInt
#[allow(unused_variables, non_snake_case)]
pub fn mj_flood_fill(island: *mut i32, nr: i32, rownnz: *const i32, rowadr: *const i32, colind: *const i32, stack: *mut i32) -> i32 {
    // SAFETY: caller guarantees all pointers are valid with sufficient length
    unsafe {
        let mut nisland: i32 = 0;
        crate::engine::engine_util_misc::mju_fill_int(island, -1, nr);

        for i in 0..nr {
            if *island.add(i as usize) != -1 || *rownnz.add(i as usize) == 0 {
                continue;
            }

            let mut nstack: i32 = 0;
            *stack.add(nstack as usize) = i;
            nstack += 1;

            while nstack != 0 {
                nstack -= 1;
                let v = *stack.add(nstack as usize);
                if *island.add(v as usize) != -1 {
                    continue;
                }
                *island.add(v as usize) = nisland;
                crate::engine::engine_util_misc::mju_copy_int(
                    stack.add(nstack as usize),
                    colind.add(*rowadr.add(v as usize) as usize),
                    *rownnz.add(v as usize),
                );
                nstack += *rownnz.add(v as usize);
            }

            nisland += 1;
        }

        nisland
    }
}

/// C: mj_island (engine/engine_island.h:35)
/// Calls: arenaAllocIsland, findEdges, mj_floodFill, mj_freeStack, mj_markStack, mj_stackAllocInfo, mju_compare, mju_copyInt, mju_gather, mju_gatherInt, mju_message, mju_zeroInt
#[allow(unused_variables, non_snake_case)]
pub fn mj_island(m: *const mjModel, d: *mut mjData) {
    todo!() // mj_island
}

