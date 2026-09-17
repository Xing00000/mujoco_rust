//! Port of: engine/engine_island.c
//! IR hash: fd5288ad7242f001
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: clearIsland (engine/engine_island.c:38)
#[allow(unused_variables, non_snake_case)]
pub fn clear_island(d: *mut mjData, parena: usize) {
    todo!() // cxx-internal:engine_island.c.o:_clearIsland
}

/// C: arenaAllocIsland (engine/engine_island.c:56)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_island.c:_clearIsland, cxx:_mj_arenaAllocByte, cxx:_mj_warning
#[allow(unused_variables, non_snake_case)]
pub fn arena_alloc_island(m: *const mjModel, d: *mut mjData) -> i32 {
    todo!() // cxx-internal:engine_island.c.o:_arenaAllocIsland
}

/// C: mj_dsuRoot (engine/engine_island.c:87)
#[allow(unused_variables, non_snake_case)]
pub fn mj_dsu_root(parent: *mut i32, tree: i32) -> i32 {
    todo!() // cxx:_mj_dsuRoot
}

/// C: mj_dsuMerge (engine/engine_island.c:104)
/// Calls: cxx:_mj_dsuRoot, cxx:_mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_dsu_merge(parent: *mut i32, tree1: i32, tree2: i32) {
    todo!() // cxx:_mj_dsuMerge
}

/// C: mj_dsuAssign (engine/engine_island.c:129)
#[allow(unused_variables, non_snake_case)]
pub fn mj_dsu_assign(island: *mut i32, parent: *mut i32, tree_dofnum: *const i32, ntree: i32, nidof: *mut i32) -> i32 {
    todo!() // cxx:_mj_dsuAssign
}

/// C: mj_floodFill (engine/engine_island.c:164)
/// Calls: cxx:_mju_copyInt, cxx:_mju_fillInt
#[allow(unused_variables, non_snake_case)]
pub fn mj_flood_fill(island: *mut i32, nr: i32, rownnz: *const i32, rowadr: *const i32, colind: *const i32, stack: *mut i32) -> i32 {
    todo!() // cxx:_mj_floodFill
}

/// C: treeNext (engine/engine_island.c:215)
/// Calls: cxx:_mj_isSparse
#[allow(unused_variables, non_snake_case)]
pub fn tree_next(m: *const mjModel, d: *const mjData, i: i32, iter: *mut mjTreeIter) -> i32 {
    todo!() // cxx-internal:engine_island.c.o:_treeNext
}

/// C: treeIterInit (engine/engine_island.c:278)
/// Calls: cxx:_mju_message
#[allow(unused_variables, non_snake_case)]
pub fn tree_iter_init(m: *const mjModel, d: *const mjData, i: i32, iter: *mut mjTreeIter) {
    todo!() // cxx-internal:engine_island.c.o:_treeIterInit
}

/// C: isFlexEquality (engine/engine_island.c:350)
#[allow(unused_variables, non_snake_case)]
pub fn is_flex_equality(m: *const mjModel, efc_type: i32, efc_id: i32) -> i32 {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_island.c:_isFlexEquality
}

/// C: unionTendonTrees (engine/engine_island.c:360)
/// Calls: cxx:_mj_dsuMerge
#[allow(unused_variables, non_snake_case)]
pub fn union_tendon_trees(m: *const mjModel, d: *const mjData, parent: *mut i32, i: i32) {
    todo!() // cxx-internal:engine_island.c.o:_unionTendonTrees
}

/// C: unionConstraintTrees (engine/engine_island.c:378)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_island.c:_isFlexEquality, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_island.c:_treeIterInit, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_island.c:_treeNext, cxx-internal:engine_island.c.o:_unionTendonTrees, cxx:_mj_dsuMerge, cxx:_mj_effActuatorPossible, cxx:_mj_effCouplings, cxx:_mj_effFlexContactPossible, cxx:_mj_effFlexStiffPossible, cxx:_mj_effTendonPossible
#[allow(unused_variables, non_snake_case)]
pub fn union_constraint_trees(m: *const mjModel, d: *const mjData, parent: *mut i32, efc_tree: *mut i32, err_i: *mut i32) -> *const i8 {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_island.c:_unionConstraintTrees
}

/// C: mj_island (engine/engine_island.c:526)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_island.c:_arenaAllocIsland, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_island.c:_unionConstraintTrees, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_sparse.h:_mju_compare, cxx:_mj_dsuAssign, cxx:_mj_freeStack, cxx:_mj_markStack, cxx:_mj_stackAllocInfo, cxx:_mju_copyInt, cxx:_mju_fillInt, cxx:_mju_message, cxx:_mju_zeroInt
#[allow(unused_variables, non_snake_case)]
pub fn mj_island(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx:_mj_island
}

