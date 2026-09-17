//! Port of: Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_island.c
//! IR hash: 1fa44043fe383ce9
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: clearIsland (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_island.c:39)
#[allow(unused_variables, non_snake_case)]
pub fn clear_island(d: *mut mjData, parena: usize) {
    todo!() // cxx-internal:engine_island.c.o:_clearIsland
}

/// C: arenaAllocIsland (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_island.c:57)
/// Calls: cxx-internal:engine_island.c.o:_clearIsland, cxx:_mj_arenaAllocByte, cxx:_mj_warning
#[allow(unused_variables, non_snake_case)]
pub fn arena_alloc_island(m: *const mjModel, d: *mut mjData) -> i32 {
    todo!() // cxx-internal:engine_island.c.o:_arenaAllocIsland
}

/// C: treeNext (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_island.c:149)
/// Calls: cxx:_mj_isSparse
#[allow(unused_variables, non_snake_case)]
pub fn tree_next(m: *const mjModel, d: *const mjData, i: i32, iter: *mut mjTreeIter) -> i32 {
    todo!() // cxx-internal:engine_island.c.o:_treeNext
}

/// C: treeIterInit (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_island.c:212)
/// Calls: cxx:_mju_message
#[allow(unused_variables, non_snake_case)]
pub fn tree_iter_init(m: *const mjModel, d: *const mjData, i: i32, iter: *mut mjTreeIter) {
    todo!() // cxx-internal:engine_island.c.o:_treeIterInit
}

/// C: findEdges (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_island.c:317)
/// Calls: cxx-internal:engine_island.c.o:_treeIterInit, cxx-internal:engine_island.c.o:_treeNext, cxx:_mju_message, cxx:_mju_zeroInt
#[allow(unused_variables, non_snake_case)]
pub fn find_edges(m: *const mjModel, d: *const mjData, rownnz: *mut i32, colind: *mut i32, tree_tree: *mut u8, efc_tree: *mut i32, ntree: i32) -> i32 {
    todo!() // cxx-internal:engine_island.c.o:_findEdges
}

/// C: mj_floodFill (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_island.h:28)
/// Calls: cxx:_mju_copyInt, cxx:_mju_fillInt
#[allow(unused_variables, non_snake_case)]
pub fn mj_flood_fill(island: *mut i32, nr: i32, rownnz: *const i32, rowadr: *const i32, colind: *const i32, stack: *mut i32) -> i32 {
    todo!() // cxx:_mj_floodFill
}

/// C: mj_island (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_island.h:35)
/// Calls: cxx-internal:engine_island.c.o:_arenaAllocIsland, cxx-internal:engine_island.c.o:_findEdges, cxx:_mj_floodFill, cxx:_mj_freeStack, cxx:_mj_markStack, cxx:_mj_stackAllocInfo, cxx:_mju_copyInt, cxx:_mju_gather, cxx:_mju_gatherInt, cxx:_mju_message, cxx:_mju_zeroInt
#[allow(unused_variables, non_snake_case)]
pub fn mj_island(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx:_mj_island
}

