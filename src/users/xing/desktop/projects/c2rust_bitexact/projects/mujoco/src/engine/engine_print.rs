//! Port of: Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_print.c
//! IR hash: 1fa44043fe383ce9
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: printInt (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_print.c:53)
#[allow(unused_variables, non_snake_case)]
pub fn print_int(fp: *mut FILE, name: *const i8, value: i32) {
    todo!() // cxx-internal:engine_print.c.o:_printInt
}

/// C: printStr (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_print.c:59)
#[allow(unused_variables, non_snake_case)]
pub fn print_str(fp: *mut FILE, name: *const i8, value: *const i8) {
    todo!() // cxx-internal:engine_print.c.o:_printStr
}

/// C: printNum (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_print.c:65)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn print_num(fp: *mut FILE, name: *const i8, value: f32, float_format: *const i8) {
    todo!() // cxx-internal:engine_print.c.o:_printNum
}

/// C: printArr (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_print.c:71)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn print_arr(fp: *mut FILE, name: *const i8, data: *const f32, n: i32, float_format: *const i8) {
    todo!() // cxx-internal:engine_print.c.o:_printArr
}

/// C: printArray2d (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_print.c:84)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn print_array2d(str: *const i8, nr: i32, nc: i32, data: *const f64, fp: *mut FILE, float_format: *const i8) {
    todo!() // cxx-internal:engine_print.c.o:_printArray2d
}

/// C: printArray2dInt (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_print.c:105)
#[allow(unused_variables, non_snake_case)]
pub fn print_array2d_int(str: *const i8, nr: i32, nc: i32, data: *const i32, fp: *mut FILE) {
    todo!() // cxx-internal:engine_print.c.o:_printArray2dInt
}

/// C: printDelayBuffer (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_print.c:125)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn print_delay_buffer(name: *const i8, buf: *const f64, nhistory: i32, dim: i32, fp: *mut FILE, float_format: *const i8) {
    todo!() // cxx-internal:engine_print.c.o:_printDelayBuffer
}

/// C: printSparse (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_print.c:170)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn print_sparse(str: *const i8, mat: *const f64, nr: i32, rownnz: *const i32, rowadr: *const i32, colind: *const i32, fp: *mut FILE, float_format: *const i8) {
    todo!() // cxx-internal:engine_print.c.o:_printSparse
}

/// C: printBlockArray (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_print.c:193)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn print_block_array(str: *const i8, data: *const f64, nc: i32, nisland: i32, island_nr: *const i32, island_nc: *const i32, island_r: *const i32, island_c: *const i32, map_r: *const i32, map_c: *const i32, fp: *mut FILE, float_format: *const i8) {
    todo!() // cxx-internal:engine_print.c.o:_printBlockArray
}

/// C: printInertia (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_print.c:246)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn print_inertia(str: *const i8, mat: *const f64, m: *const mjModel, fp: *mut FILE, float_format: *const i8) {
    todo!() // cxx-internal:engine_print.c.o:_printInertia
}

/// C: mj_printBlockSparsity (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_print.c:319)
#[allow(unused_variables, non_snake_case)]
pub fn mj_print_block_sparsity(str: *const i8, nr: i32, nc: i32, nisland: i32, island_block_ncols: *const i32, island_col_offset: *const i32, entity_island: *const i32, map_row_to_entity: *const i32, map_col_to_entity: *const i32, rownnz: *const i32, rowadr: *const i32, colind: *const i32, rowsuper: *const i32, fp: *mut FILE) {
    todo!() // cxx:_mj_printBlockSparsity
}

/// C: printVector (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_print.c:377)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn print_vector(str: *const i8, data: *const f64, n: i32, fp: *mut FILE, float_format: *const i8) {
    todo!() // cxx-internal:engine_print.c.o:_printVector
}

/// C: memorySize (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_print.c:395)
#[allow(unused_variables, non_snake_case)]
pub fn memory_size(nbytes: usize) -> *const i8 {
    todo!() // cxx-internal:engine_print.c.o:_memorySize
}

/// C: sizeMesh (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_print.c:410)
#[allow(unused_variables, non_snake_case)]
pub fn size_mesh(m: *const mjModel) -> usize {
    todo!() // cxx-internal:engine_print.c.o:_sizeMesh
}

/// C: sizeSkin (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_print.c:431)
#[allow(unused_variables, non_snake_case)]
pub fn size_skin(m: *const mjModel) -> usize {
    todo!() // cxx-internal:engine_print.c.o:_sizeSkin
}

/// C: sizeBVH (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_print.c:448)
#[allow(unused_variables, non_snake_case)]
pub fn size_bvh(m: *const mjModel) -> usize {
    todo!() // cxx-internal:engine_print.c.o:_sizeBVH
}

/// C: validateFloatFormat (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_print.c:463)
/// Calls: cxx:_mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn validate_float_format(float_format: *const i8) -> bool {
    todo!() // cxx-internal:engine_print.c.o:_validateFloatFormat
}

/// C: mj_printFormattedModel (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_print.h:31)
/// Calls: cxx-internal:engine_print.c.o:_memorySize, cxx-internal:engine_print.c.o:_printArray2dInt, cxx-internal:engine_print.c.o:_sizeBVH, cxx-internal:engine_print.c.o:_sizeMesh, cxx-internal:engine_print.c.o:_sizeSkin, cxx-internal:engine_print.c.o:_validateFloatFormat, cxx:_mj_printSparsity, cxx:_mj_sizeModel, cxx:_mj_versionString, cxx:_mju_type2Str, cxx:_mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn mj_print_formatted_model(m: *const mjModel, filename: *const i8, float_format: *const i8) {
    todo!() // cxx:_mj_printFormattedModel
}

/// C: mj_printModel (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_print.h:35)
/// Calls: cxx:_mj_printFormattedModel
#[allow(unused_variables, non_snake_case)]
pub fn mj_print_model(m: *const mjModel, filename: *const i8) {
    todo!() // cxx:_mj_printModel
}

/// C: mj_printFormattedData (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_print.h:40)
/// Calls: cxx-internal:engine_print.c.o:_memorySize, cxx-internal:engine_print.c.o:_printArray2d, cxx-internal:engine_print.c.o:_printArray2dInt, cxx-internal:engine_print.c.o:_printBlockArray, cxx-internal:engine_print.c.o:_printDelayBuffer, cxx-internal:engine_print.c.o:_printInertia, cxx-internal:engine_print.c.o:_printSparse, cxx-internal:engine_print.c.o:_printVector, cxx-internal:engine_print.c.o:_validateFloatFormat, cxx:_mj_contactForce, cxx:_mj_id2name, cxx:_mj_isDual, cxx:_mj_isSparse, cxx:_mj_printBlockSparsity, cxx:_mj_printSparsity, cxx:_mju_isZero, cxx:_mju_message, cxx:_mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn mj_print_formatted_data(m: *const mjModel, d: *const mjData, filename: *const i8, float_format: *const i8) {
    todo!() // cxx:_mj_printFormattedData
}

/// C: mj_printData (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_print.h:44)
/// Calls: cxx:_mj_printFormattedData
#[allow(unused_variables, non_snake_case)]
pub fn mj_print_data(m: *const mjModel, d: *const mjData, filename: *const i8) {
    todo!() // cxx:_mj_printData
}

/// C: mj_printSparsity (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_print.h:47)
#[allow(unused_variables, non_snake_case)]
pub fn mj_print_sparsity(str: *const i8, nr: i32, nc: i32, rowadr: *const i32, diag: *const i32, rownnz: *const i32, rowsuper: *const i32, colind: *const i32, fp: *mut FILE) {
    todo!() // cxx:_mj_printSparsity
}

/// C: mj_printScene (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_print.h:51)
/// Calls: cxx:_mj_printFormattedScene
#[allow(unused_variables, non_snake_case)]
pub fn mj_print_scene(s: *const mjvScene, filename: *const i8) {
    todo!() // cxx:_mj_printScene
}

/// C: mj_printFormattedScene (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_print.h:55)
/// Calls: cxx-internal:engine_print.c.o:_printArr, cxx-internal:engine_print.c.o:_printInt, cxx-internal:engine_print.c.o:_printNum, cxx-internal:engine_print.c.o:_printStr, cxx-internal:engine_print.c.o:_validateFloatFormat, cxx:_mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn mj_print_formatted_scene(s: *const mjvScene, filename: *const i8, float_format: *const i8) {
    todo!() // cxx:_mj_printFormattedScene
}

