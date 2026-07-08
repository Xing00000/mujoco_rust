//! Port of: engine/engine_island.c
//! IR hash: 05737965add36adb
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: clearIsland (engine/engine_island.c:39)
#[allow(unused_variables, non_snake_case)]
pub fn clear_island(d: *mut mjData, parena: usize) {
    extern "C" { fn clearIsland_impl(d: *mut mjData, parena: usize); }
    // SAFETY: delegates to C implementation
    unsafe { clearIsland_impl(d, parena) }
}

/// C: arenaAllocIsland (engine/engine_island.c:57)
/// Calls: clearIsland, mj_arenaAllocByte, mj_warning
#[allow(unused_variables, non_snake_case)]
pub fn arena_alloc_island(m: *const mjModel, d: *mut mjData) -> i32 {
    // SAFETY: uses C X-macros (MJDATA_ARENA_POINTERS_ISLAND), delegating to C implementation
    extern "C" { fn arenaAllocIsland(m: *const mjModel, d: *mut mjData) -> i32; }
    unsafe { arenaAllocIsland(m, d) }
}

/// C: treeNext (engine/engine_island.c:149)
/// Calls: mj_isSparse
#[allow(unused_variables, non_snake_case)]
pub fn tree_next(m: *const mjModel, d: *const mjData, i: i32, iter: *mut mjTreeIter) -> i32 {
    extern "C" {
        fn treeNext_impl(m: *const mjModel, d: *const mjData, i: i32, iter: *mut mjTreeIter) -> i32;
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { treeNext_impl(m, d, i, iter) }
}

/// C: treeIterInit (engine/engine_island.c:212)
/// Calls: mju_message
#[allow(unused_variables, non_snake_case)]
pub fn tree_iter_init(m: *const mjModel, d: *const mjData, i: i32, iter: *mut mjTreeIter) {
    extern "C" {
        fn treeIterInit_impl(m: *const mjModel, d: *const mjData, i: i32, iter: *mut mjTreeIter);
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { treeIterInit_impl(m, d, i, iter) }
}

/// C: findEdges (engine/engine_island.c:317)
/// Calls: addEdge, mju_message, mju_zeroInt, treeIterInit, treeNext
#[allow(unused_variables, non_snake_case)]
pub fn find_edges(m: *const mjModel, d: *const mjData, rownnz: *mut i32, colind: *mut i32, tree_tree: *mut u8, efc_tree: *mut i32, ntree: i32) -> i32 {
    extern "C" {
        fn findEdges_impl(m: *const mjModel, d: *const mjData, rownnz: *mut i32, colind: *mut i32, tree_tree: *mut u8, efc_tree: *mut i32, ntree: i32) -> i32;
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { findEdges_impl(m, d, rownnz, colind, tree_tree, efc_tree, ntree) }
}

/// C: mj_floodFill (engine/engine_island.h:28)
/// Calls: mju_copyInt, mju_fillInt
#[allow(unused_variables, non_snake_case)]
pub fn mj_flood_fill(island: *mut i32, nr: i32, rownnz: *const i32, rowadr: *const i32, colind: *const i32, stack: *mut i32) -> i32 {
    // SAFETY: all pointers valid. island and stack have nr elements each.
    // rownnz, rowadr, colind describe a sparse adjacency matrix.
    unsafe {
        // initialize island count, set ids to -1
        let mut nisland: i32 = 0;
        crate::engine::engine_util_misc::mju_fill_int(island, -1, nr);

        // iterate over vertices, discover islands
        let mut i: i32 = 0;
        while i < nr {
            // vertex already in island or singleton with no edges: skip
            if *island.add(i as usize) != -1 || *rownnz.add(i as usize) == 0 {
                i += 1;
                continue;
            }

            // push i onto stack
            let mut nstack: i32 = 0;
            *stack.add(nstack as usize) = i;
            nstack += 1;

            // DFS traversal of island
            while nstack != 0 {
                // pop v from stack
                nstack -= 1;
                let v = *stack.add(nstack as usize);

                // if v is already assigned, continue
                if *island.add(v as usize) != -1 {
                    continue;
                }

                // assign v to current island
                *island.add(v as usize) = nisland;

                // push adjacent vertices onto stack
                crate::engine::engine_util_misc::mju_copy_int(
                    stack.add(nstack as usize),
                    colind.add(*rowadr.add(v as usize) as usize),
                    *rownnz.add(v as usize),
                );
                nstack += *rownnz.add(v as usize);
            }

            // island is filled: increment nisland
            nisland += 1;
            i += 1;
        }

        nisland
    }
}

/// C: mj_island (engine/engine_island.h:35)
/// Calls: arenaAllocIsland, findEdges, mj_floodFill, mj_freeStack, mj_markStack, mj_stackAllocInfo, mju_compare, mju_copyInt, mju_gather, mju_gatherInt, mju_message, mju_zeroInt
#[allow(unused_variables, non_snake_case)]
pub fn mj_island(m: *const mjModel, d: *mut mjData) {
    extern "C" {
        fn mj_island_impl(m: *const mjModel, d: *mut mjData);
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_island_impl(m, d) }
}

