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
    use crate::types::*;
    const MJ_CNSTR_EQUALITY: i32 = mjtConstraint_mjCNSTR_EQUALITY as i32;
    const MJ_EQ_FLEX: i32 = mjtEq_mjEQ_FLEX as i32;
    const MJ_EQ_FLEXVERT: i32 = mjtEq_mjEQ_FLEXVERT as i32;
    const MJ_EQ_FLEXSTRAIN: i32 = mjtEq_mjEQ_FLEXSTRAIN as i32;

    // SAFETY: m, d are valid model/data pointers. All pointer accesses bounded.
    unsafe {
        let nefc = (*d).nefc;
        let mut nnz: i32 = 0;
        let mut prev_efc_type: i32 = -1;
        let mut prev_efc_id: i32 = -1;

        // clear row nonzeros
        crate::engine::engine_util_misc::mju_zero_int(rownnz, ntree);

        // inline addEdge: adds edge(s) between tree1 and tree2, returns count
        let add_edge = |rownnz: *mut i32, colind: *mut i32, tree_tree: *mut u8,
                        ntree: i32, mut t1: i32, mut t2: i32| -> i32 {
            if t1 == -1 && t2 == -1 {
                crate::engine::engine_util_errmem::mju_error(
                    b"findEdges: self-edge of the static tree\0".as_ptr() as *const i8);
                return 0;
            }
            if t1 == -1 { t1 = t2; }
            if t2 == -1 { t2 = t1; }
            // skip if edge already present
            if *tree_tree.add((t1 * ntree + t2) as usize) != 0 {
                return 0;
            }
            *tree_tree.add((t1 * ntree + t2) as usize) = 1;
            let rnz = rownnz.add(t1 as usize);
            *colind.add((t1 * ntree + *rnz) as usize) = t2;
            *rnz += 1;
            if t1 != t2 {
                *tree_tree.add((t2 * ntree + t1) as usize) = 1;
                let rnz2 = rownnz.add(t2 as usize);
                *colind.add((t2 * ntree + *rnz2) as usize) = t1;
                *rnz2 += 1;
                return 2;
            }
            1
        };

        for i in 0..nefc as usize {
            let cur_type = *(*d).efc_type.add(i);
            let cur_id   = *(*d).efc_id.add(i);

            // same constraint row: skip unless flex equality
            if cur_type == prev_efc_type && cur_id == prev_efc_id {
                let is_flex_eq = cur_type == MJ_CNSTR_EQUALITY
                    && (*(*m).eq_type.add(cur_id as usize) == MJ_EQ_FLEX
                        || *(*m).eq_type.add(cur_id as usize) == MJ_EQ_FLEXVERT
                        || *(*m).eq_type.add(cur_id as usize) == MJ_EQ_FLEXSTRAIN);
                if !is_flex_eq {
                    *efc_tree.add(i) = *efc_tree.add(i - 1);
                    continue;
                }
            }
            prev_efc_type = cur_type;
            prev_efc_id   = cur_id;

            // initialize tree iterator
            let mut iter_buf = [0i32; 4];  // mjTreeIter: 4 x i32 = 16 bytes
            let iter = iter_buf.as_mut_ptr() as *mut mjTreeIter;
            tree_iter_init(m, d, i as i32, iter);

            let tree1 = tree_next(m, d, i as i32, iter);
            if tree1 != -2 {
                let tree2 = tree_next(m, d, i as i32, iter);

                // assign tree to constraint
                *efc_tree.add(i) = if tree1 >= 0 { tree1 } else { tree2 };
                if *efc_tree.add(i) < 0 {
                    crate::engine::engine_util_errmem::mju_error(
                        b"findEdges: no tree found for constraint\0".as_ptr() as *const i8);
                }

                if tree2 == -2 {
                    nnz += add_edge(rownnz, colind, tree_tree, ntree, tree1, -1);
                } else {
                    let mut t1 = tree1;
                    let mut t2 = tree2;
                    while t2 != -2 {
                        nnz += add_edge(rownnz, colind, tree_tree, ntree, t1, t2);
                        t1 = t2;
                        t2 = tree_next(m, d, i as i32, iter);
                    }
                }
            } else {
                crate::engine::engine_util_errmem::mju_error(
                    b"findEdges: no tree found for constraint\0".as_ptr() as *const i8);
            }
        }

        nnz
    }
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
    use crate::types::*;
    const MJ_DSBL_ISLAND: i32 = mjtDisableBit_mjDSBL_ISLAND as i32;
    const MJ_CNSTR_EQUALITY: i32 = mjtConstraint_mjCNSTR_EQUALITY as i32;
    const MJ_CNSTR_FRICTION_DOF: i32 = mjtConstraint_mjCNSTR_FRICTION_DOF as i32;
    const MJ_CNSTR_FRICTION_TENDON: i32 = mjtConstraint_mjCNSTR_FRICTION_TENDON as i32;

    // SAFETY: m, d are valid model/data pointers. All arena/stack allocations bounded.
    unsafe {
        let nv   = (*m).nv as i32;
        let nefc = (*d).nefc;
        let ntree = (*m).ntree as i32;

        // no constraints or islands disabled: quick return
        if ((*m).opt.disableflags & MJ_DSBL_ISLAND) != 0 || nefc == 0 {
            (*d).nisland = 0;
            (*d).nidof = 0;
            return;
        }

        crate::engine::engine_memory::mj_mark_stack(d);

        let ntree2 = ntree * ntree;

        // dense tree-tree adjacency matrix
        let tree_tree = crate::engine::engine_memory::mj_stack_alloc_info(
            d, ntree2 as usize * std::mem::size_of::<u8>(), std::mem::align_of::<u8>(),
            std::ptr::null(), 0) as *mut u8;
        std::ptr::write_bytes(tree_tree, 0, ntree2 as usize);

        // CSR representation (uncompressed)
        let colind = crate::engine::engine_memory::mj_stack_alloc_info(
            d, ntree2 as usize * 4, 4, std::ptr::null(), 0) as *mut i32;
        let rownnz = crate::engine::engine_memory::mj_stack_alloc_info(
            d, ntree as usize * 4, 4, std::ptr::null(), 0) as *mut i32;
        let rowadr = crate::engine::engine_memory::mj_stack_alloc_info(
            d, ntree as usize * 4, 4, std::ptr::null(), 0) as *mut i32;
        for r in 0..ntree as usize {
            *rowadr.add(r) = r as i32 * ntree;
        }

        // first non-negative tree index of each constraint
        let efc_tree = crate::engine::engine_memory::mj_stack_alloc_info(
            d, nefc as usize * 4, 4, std::ptr::null(), 0) as *mut i32;

        let nnz = find_edges(m, d, rownnz, colind, tree_tree, efc_tree, ntree);

        // discover islands
        let tree_island = crate::engine::engine_memory::mj_stack_alloc_info(
            d, ntree as usize * 4, 4, std::ptr::null(), 0) as *mut i32;
        let stack = crate::engine::engine_memory::mj_stack_alloc_info(
            d, nnz as usize * 4, 4, std::ptr::null(), 0) as *mut i32;

        (*d).nisland = mj_flood_fill(tree_island, ntree, rownnz, rowadr, colind, stack);

        if (*d).nisland == 0 {
            (*d).nidof = 0;
            crate::engine::engine_memory::mj_free_stack(d);
            return;
        }

        // count nidof
        let mut nidof: i32 = 0;
        for i in 0..ntree as usize {
            if *tree_island.add(i) >= 0 {
                nidof += *(*m).tree_dofnum.add(i);
            }
        }
        (*d).nidof = nidof;

        if arena_alloc_island(m, d) == 0 {
            crate::engine::engine_memory::mj_free_stack(d);
            return;
        }

        let nisland = (*d).nisland as usize;

        // copy tree_island to arena
        crate::engine::engine_util_misc::mju_copy_int((*d).tree_island, tree_island as *const i32, ntree);

        // compute island_ntree
        crate::engine::engine_util_misc::mju_zero_int((*d).island_ntree, nisland as i32);
        for i in 0..ntree as usize {
            let island = *tree_island.add(i);
            if island >= 0 {
                *(*d).island_ntree.add(island as usize) += 1;
            }
        }

        // compute island_itreeadr (cumsum)
        *(*d).island_itreeadr.add(0) = 0;
        for i in 1..nisland {
            *(*d).island_itreeadr.add(i) = *(*d).island_itreeadr.add(i - 1) + *(*d).island_ntree.add(i - 1);
        }
        let last_tree = *(*d).island_itreeadr.add(nisland - 1) + *(*d).island_ntree.add(nisland - 1);

        // compute map_itree2tree
        let island_ntree2 = crate::engine::engine_memory::mj_stack_alloc_info(
            d, (nisland + 1) * 4, 4, std::ptr::null(), 0) as *mut i32;
        crate::engine::engine_util_misc::mju_zero_int(island_ntree2, (nisland + 1) as i32);
        for i in 0..ntree as usize {
            let island = *tree_island.add(i);
            if island >= 0 {
                let idx = *(*d).island_itreeadr.add(island as usize) + *island_ntree2.add(island as usize);
                *(*d).map_itree2tree.add(idx as usize) = i as i32;
                *island_ntree2.add(island as usize) += 1;
            } else {
                let idx = last_tree + *island_ntree2.add(nisland);
                *(*d).map_itree2tree.add(idx as usize) = i as i32;
                *island_ntree2.add(nisland) += 1;
            }
        }

        // compute dof_island, island_nv
        crate::engine::engine_util_misc::mju_zero_int((*d).island_nv, nisland as i32);
        for i in 0..nv as usize {
            let tree_id = *(*m).dof_treeid.add(i);
            let island = *tree_island.add(tree_id as usize);
            *(*d).dof_island.add(i) = island;
            if island >= 0 {
                *(*d).island_nv.add(island as usize) += 1;
            }
        }

        // compute island_idofadr (cumsum)
        *(*d).island_idofadr.add(0) = 0;
        for i in 1..nisland {
            *(*d).island_idofadr.add(i) = *(*d).island_idofadr.add(i - 1) + *(*d).island_nv.add(i - 1);
        }

        // compute dof <-> idof maps
        let island_nv2 = crate::engine::engine_memory::mj_stack_alloc_info(
            d, (nisland + 1) * 4, 4, std::ptr::null(), 0) as *mut i32;
        crate::engine::engine_util_misc::mju_zero_int(island_nv2, (nisland + 1) as i32);
        for dof in 0..nv as usize {
            let island = *(*d).dof_island.add(dof);
            let idof = if island >= 0 {
                let v = *(*d).island_idofadr.add(island as usize) + *island_nv2.add(island as usize);
                *island_nv2.add(island as usize) += 1;
                v
            } else {
                let v = nidof + *island_nv2.add(nisland);
                *island_nv2.add(nisland) += 1;
                v
            };
            *(*d).map_dof2idof.add(dof) = idof;
            *(*d).map_idof2dof.add(idof as usize) = dof as i32;
        }

        // compute island_dofadr
        for i in 0..nisland {
            *(*d).island_dofadr.add(i) = *(*d).map_idof2dof.add(*(*d).island_idofadr.add(i) as usize);
        }

        // compute efc_island, island_{ne,nf,nefc}
        crate::engine::engine_util_misc::mju_zero_int((*d).island_ne, nisland as i32);
        crate::engine::engine_util_misc::mju_zero_int((*d).island_nf, nisland as i32);
        crate::engine::engine_util_misc::mju_zero_int((*d).island_nefc, nisland as i32);
        for i in 0..nefc as usize {
            let island = *tree_island.add(*efc_tree.add(i) as usize);
            *(*d).efc_island.add(i) = island;
            *(*d).island_nefc.add(island as usize) += 1;
            let efc_t = *(*d).efc_type.add(i);
            if efc_t == MJ_CNSTR_EQUALITY {
                *(*d).island_ne.add(island as usize) += 1;
            } else if efc_t == MJ_CNSTR_FRICTION_DOF || efc_t == MJ_CNSTR_FRICTION_TENDON {
                *(*d).island_nf.add(island as usize) += 1;
            }
        }

        // compute island_iefcadr
        *(*d).island_iefcadr.add(0) = 0;
        for i in 1..nisland {
            *(*d).island_iefcadr.add(i) = *(*d).island_iefcadr.add(i - 1) + *(*d).island_nefc.add(i - 1);
        }

        // compute efc <-> iefc maps
        crate::engine::engine_util_misc::mju_zero_int(island_nv2, nisland as i32);
        for c in 0..nefc as usize {
            let island = *(*d).efc_island.add(c);
            let ic = *(*d).island_iefcadr.add(island as usize) + *island_nv2.add(island as usize);
            *island_nv2.add(island as usize) += 1;
            *(*d).map_efc2iefc.add(c) = ic;
            *(*d).map_iefc2efc.add(ic as usize) = c as i32;
        }

        // copy position-dependent efc vectors
        crate::engine::engine_util_misc::mju_gather_int((*d).iefc_type, (*d).efc_type as *const i32, (*d).map_iefc2efc as *const i32, nefc);
        crate::engine::engine_util_misc::mju_gather_int((*d).iefc_id, (*d).efc_id as *const i32, (*d).map_iefc2efc as *const i32, nefc);
        crate::engine::engine_util_misc::mju_gather((*d).iefc_frictionloss, (*d).efc_frictionloss as *const f64, (*d).map_iefc2efc as *const i32, nefc);
        crate::engine::engine_util_misc::mju_gather((*d).iefc_D, (*d).efc_D as *const f64, (*d).map_iefc2efc as *const i32, nefc);
        crate::engine::engine_util_misc::mju_gather((*d).iefc_R, (*d).efc_R as *const f64, (*d).map_iefc2efc as *const i32, nefc);

        crate::engine::engine_memory::mj_free_stack(d);
    }
}

