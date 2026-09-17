//! Port of: engine/engine_print.c
//! IR hash: fd5288ad7242f001
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: printInt (engine/engine_print.c:53)
#[allow(unused_variables, non_snake_case)]
pub fn print_int(fp: *mut FILE, name: *const i8, value: i32) {
    todo!() // cxx-internal:engine_print.c.o:_printInt
}

/// C: printStr (engine/engine_print.c:59)
#[allow(unused_variables, non_snake_case)]
pub fn print_str(fp: *mut FILE, name: *const i8, value: *const i8) {
    todo!() // cxx-internal:engine_print.c.o:_printStr
}

/// C: printNum (engine/engine_print.c:65)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn print_num(fp: *mut FILE, name: *const i8, value: f32, float_format: *const i8) {
    todo!() // cxx-internal:engine_print.c.o:_printNum
}

/// C: printArr (engine/engine_print.c:71)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn print_arr(fp: *mut FILE, name: *const i8, data: *const f32, n: i32, float_format: *const i8) {
    todo!() // cxx-internal:engine_print.c.o:_printArr
}

/// C: printArray2d (engine/engine_print.c:84)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn print_array2d(str: *const i8, nr: i32, nc: i32, data: *const f64, fp: *mut FILE, float_format: *const i8) {
    todo!() // cxx-internal:engine_print.c.o:_printArray2d
}

/// C: printArray2dInt (engine/engine_print.c:105)
#[allow(unused_variables, non_snake_case)]
pub fn print_array2d_int(str: *const i8, nr: i32, nc: i32, data: *const i32, fp: *mut FILE) {
    todo!() // cxx-internal:engine_print.c.o:_printArray2dInt
}

/// C: printDelayBuffer (engine/engine_print.c:125)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn print_delay_buffer(name: *const i8, buf: *const f64, nhistory: i32, dim: i32, fp: *mut FILE, float_format: *const i8) {
    todo!() // cxx-internal:engine_print.c.o:_printDelayBuffer
}

/// C: printSparse (engine/engine_print.c:170)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn print_sparse(str: *const i8, mat: *const f64, nr: i32, rownnz: *const i32, rowadr: *const i32, colind: *const i32, fp: *mut FILE, float_format: *const i8) {
    todo!() // cxx-internal:engine_print.c.o:_printSparse
}

/// C: printBlockArray (engine/engine_print.c:193)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn print_block_array(str: *const i8, data: *const f64, nc: i32, nisland: i32, island_nr: *const i32, island_nc: *const i32, island_r: *const i32, island_c: *const i32, map_r: *const i32, map_c: *const i32, fp: *mut FILE, float_format: *const i8) {
    todo!() // cxx-internal:engine_print.c.o:_printBlockArray
}

/// C: mj_printSparsity (engine/engine_print.c:248)
#[allow(unused_variables, non_snake_case)]
pub fn mj_print_sparsity(str: *const i8, nr: i32, nc: i32, rowadr: *const i32, diag: *const i32, rownnz: *const i32, rowsuper: *const i32, colind: *const i32, fp: *mut FILE) {
    todo!() // cxx:_mj_printSparsity
}

/// C: mj_printBlockSparsity (engine/engine_print.c:284)
#[allow(unused_variables, non_snake_case)]
pub fn mj_print_block_sparsity(str: *const i8, nr: i32, nc: i32, nisland: i32, island_block_ncols: *const i32, island_col_offset: *const i32, entity_island: *const i32, map_row_to_entity: *const i32, map_col_to_entity: *const i32, rownnz: *const i32, rowadr: *const i32, colind: *const i32, rowsuper: *const i32, fp: *mut FILE) {
    todo!() // cxx:_mj_printBlockSparsity
}

/// C: printVector (engine/engine_print.c:342)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn print_vector(str: *const i8, data: *const f64, n: i32, fp: *mut FILE, float_format: *const i8) {
    todo!() // cxx-internal:engine_print.c.o:_printVector
}

/// C: memorySize (engine/engine_print.c:360)
#[allow(unused_variables, non_snake_case)]
pub fn memory_size(nbytes: usize) -> *const i8 {
    todo!() // cxx-internal:engine_print.c.o:_memorySize
}

/// C: sizeMesh (engine/engine_print.c:375)
#[allow(unused_variables, non_snake_case)]
pub fn size_mesh(m: *const mjModel) -> usize {
    todo!() // cxx-internal:engine_print.c.o:_sizeMesh
}

/// C: sizeSkin (engine/engine_print.c:396)
#[allow(unused_variables, non_snake_case)]
pub fn size_skin(m: *const mjModel) -> usize {
    todo!() // cxx-internal:engine_print.c.o:_sizeSkin
}

/// C: sizeBVH (engine/engine_print.c:413)
#[allow(unused_variables, non_snake_case)]
pub fn size_bvh(m: *const mjModel) -> usize {
    todo!() // cxx-internal:engine_print.c.o:_sizeBVH
}

/// C: validateFloatFormat (engine/engine_print.c:428)
/// Calls: cxx:_mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn validate_float_format(float_format: *const i8) -> bool {
    todo!() // cxx-internal:engine_print.c.o:_validateFloatFormat
}

/// C: mj_printFormattedModel (engine/engine_print.c:507)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_print.c:_memorySize, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_print.c:_printInt, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_print.c:_sizeBVH, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_print.c:_sizeMesh, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_print.c:_sizeSkin, cxx-internal:engine_print.c.o:_printArray2dInt, cxx-internal:engine_print.c.o:_validateFloatFormat, cxx:_mj_printSparsity, cxx:_mj_sizeModel, cxx:_mj_versionString, cxx:_mju_type2Str, cxx:_mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn mj_print_formatted_model(m: *const mjModel, filename: *const i8, float_format: *const i8) {
    todo!() // cxx:_mj_printFormattedModel
}

/// C: mj_printModel (engine/engine_print.c:1203)
/// Calls: cxx:_mj_printFormattedModel
#[allow(unused_variables, non_snake_case)]
pub fn mj_print_model(m: *const mjModel, filename: *const i8) {
    todo!() // cxx:_mj_printModel
}

/// C: mj_printFormattedData (engine/engine_print.c:1211)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_print.c:_memorySize, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_print.c:_printBlockArray, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_print.c:_printVector, cxx-internal:engine_print.c.o:_printArray2d, cxx-internal:engine_print.c.o:_printArray2dInt, cxx-internal:engine_print.c.o:_printDelayBuffer, cxx-internal:engine_print.c.o:_printSparse, cxx-internal:engine_print.c.o:_validateFloatFormat, cxx:_mj_contactForce, cxx:_mj_id2name, cxx:_mj_isDual, cxx:_mj_isSparse, cxx:_mj_printBlockSparsity, cxx:_mj_printSparsity, cxx:_mju_isZero, cxx:_mju_message, cxx:_mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn mj_print_formatted_data(m: *const mjModel, d: *const mjData, filename: *const i8, float_format: *const i8) {
    todo!() // cxx:_mj_printFormattedData
}

/// C: mj_printData (engine/engine_print.c:1734)
/// Calls: cxx:_mj_printFormattedData
#[allow(unused_variables, non_snake_case)]
pub fn mj_print_data(m: *const mjModel, d: *const mjData, filename: *const i8) {
    todo!() // cxx:_mj_printData
}

/// C: mj_printScene (engine/engine_print.c:1738)
/// Calls: cxx:_mj_printFormattedScene
#[allow(unused_variables, non_snake_case)]
pub fn mj_print_scene(s: *const mjvScene, filename: *const i8) {
    todo!() // cxx:_mj_printScene
}

/// C: mj_printFormattedScene (engine/engine_print.c:1742)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_print.c:_printArr, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_print.c:_printInt, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_print.c:_printNum, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_print.c:_printStr, cxx-internal:engine_print.c.o:_validateFloatFormat, cxx:_mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn mj_print_formatted_scene(s: *const mjvScene, filename: *const i8, float_format: *const i8) {
    todo!() // cxx:_mj_printFormattedScene
}

