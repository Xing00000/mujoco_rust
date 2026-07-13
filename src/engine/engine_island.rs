//! Port of: engine/engine_island.c
//! IR hash: e878892ca48fe222
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
    todo!() // arenaAllocIsland
}

/// C: treeNext (engine/engine_island.c:149)
/// Calls: mj_isSparse
#[allow(unused_variables, non_snake_case)]
pub fn tree_next(m: *const mjModel, d: *const mjData, i: i32, iter: *mut mjTreeIter) -> i32 {
    todo!() // treeNext
}

/// C: treeIterInit (engine/engine_island.c:212)
/// Calls: mju_message
#[allow(unused_variables, non_snake_case)]
pub fn tree_iter_init(m: *const mjModel, d: *const mjData, i: i32, iter: *mut mjTreeIter) {
    todo!() // treeIterInit
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
    todo!() // mj_floodFill
}

/// C: mj_island (engine/engine_island.h:35)
/// Calls: arenaAllocIsland, findEdges, mj_floodFill, mj_freeStack, mj_markStack, mj_stackAllocInfo, mju_compare, mju_copyInt, mju_gather, mju_gatherInt, mju_message, mju_zeroInt
#[allow(unused_variables, non_snake_case)]
pub fn mj_island(m: *const mjModel, d: *mut mjData) {
    todo!() // mj_island
}

