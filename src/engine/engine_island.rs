//! Port of: engine/engine_island.c
//! IR hash: 05737965add36adb
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: clearIsland (engine/engine_island.c:39)
#[allow(unused_variables, non_snake_case)]
pub fn clear_island(d: *mut mjData, parena: usize) {
    // WARNING: signature changed — verify body
    // Previous params: (d : * mut mjData, parena : usize)
    // Previous return: ()
    todo ! ()
}

/// C: arenaAllocIsland (engine/engine_island.c:57)
/// Calls: clearIsland, mj_arenaAllocByte, mj_warning
#[allow(unused_variables, non_snake_case)]
pub fn arena_alloc_island(m: *const mjModel, d: *mut mjData) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData)
    // Previous return: i32
    todo ! ()
}

/// C: treeNext (engine/engine_island.c:149)
/// Calls: mj_isSparse
#[allow(unused_variables, non_snake_case)]
pub fn tree_next(m: *const mjModel, d: *const mjData, i: i32, iter: *mut mjTreeIter) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * const mjData, i : i32, iter : * mut mjTreeIter)
    // Previous return: i32
    todo ! ()
}

/// C: treeIterInit (engine/engine_island.c:212)
/// Calls: mju_message
#[allow(unused_variables, non_snake_case)]
pub fn tree_iter_init(m: *const mjModel, d: *const mjData, i: i32, iter: *mut mjTreeIter) {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * const mjData, i : i32, iter : * mut mjTreeIter)
    // Previous return: ()
    todo ! ()
}

/// C: findEdges (engine/engine_island.c:317)
/// Calls: addEdge, mju_message, mju_zeroInt, treeIterInit, treeNext
#[allow(unused_variables, non_snake_case)]
pub fn find_edges(m: *const mjModel, d: *const mjData, rownnz: *mut i32, colind: *mut i32, tree_tree: *mut u8, efc_tree: *mut i32, ntree: i32) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * const mjData, rownnz : * mut i32, colind : * mut i32, tree_tree : * mut u8, efc_tree : * mut i32, ntree : i32)
    // Previous return: i32
    todo ! ()
}

/// C: mj_floodFill (engine/engine_island.h:28)
/// Calls: mju_copyInt, mju_fillInt
#[allow(unused_variables, non_snake_case)]
pub fn mj_flood_fill(island: *mut i32, nr: i32, rownnz: *const i32, rowadr: *const i32, colind: *const i32, stack: *mut i32) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (island : * mut i32, nr : i32, rownnz : * const i32, rowadr : * const i32, colind : * const i32, stack : * mut i32)
    // Previous return: i32
    todo ! ()
}

/// C: mj_island (engine/engine_island.h:35)
/// Calls: arenaAllocIsland, findEdges, mj_floodFill, mj_freeStack, mj_markStack, mj_stackAllocInfo, mju_compare, mju_copyInt, mju_gather, mju_gatherInt, mju_message, mju_zeroInt
#[allow(unused_variables, non_snake_case)]
pub fn mj_island(m: *const mjModel, d: *mut mjData) {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData)
    // Previous return: ()
    todo ! ()
}

