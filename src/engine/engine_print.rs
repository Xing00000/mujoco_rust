//! Port of: engine/engine_print.c
//! IR hash: 05737965add36adb
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: printInt (engine/engine_print.c:53)
#[allow(unused_variables, non_snake_case)]
pub fn print_int(fp: *mut i32, name: *const i8, value: i32) {
    // WARNING: signature changed — verify body
    // Previous params: (fp : * mut i32, name : * const i8, value : i32)
    // Previous return: ()
    todo ! ()
}

/// C: printStr (engine/engine_print.c:59)
#[allow(unused_variables, non_snake_case)]
pub fn print_str(fp: *mut i32, name: *const i8, value: *const i8) {
    // WARNING: signature changed — verify body
    // Previous params: (fp : * mut i32, name : * const i8, value : * const i8)
    // Previous return: ()
    todo ! ()
}

/// C: printNum (engine/engine_print.c:65)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn print_num(fp: *mut i32, name: *const i8, value: f32, float_format: *const i8) {
    // WARNING: signature changed — verify body
    // Previous params: (fp : * mut i32, name : * const i8, value : f32, float_format : * const i8)
    // Previous return: ()
    todo ! ()
}

/// C: printArr (engine/engine_print.c:71)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn print_arr(fp: *mut i32, name: *const i8, data: *const f32, n: i32, float_format: *const i8) {
    // WARNING: signature changed — verify body
    // Previous params: (fp : * mut i32, name : * const i8, data : * const f32, n : i32, float_format : * const i8)
    // Previous return: ()
    todo ! ()
}

/// C: printArray2d (engine/engine_print.c:84)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn print_array2d(str: *const i8, nr: i32, nc: i32, data: *const f64, fp: *mut i32, float_format: *const i8) {
    // WARNING: signature changed — verify body
    // Previous params: (str : * const i8, nr : i32, nc : i32, data : * const f64, fp : * mut i32, float_format : * const i8)
    // Previous return: ()
    todo ! ()
}

/// C: printArray2dInt (engine/engine_print.c:105)
#[allow(unused_variables, non_snake_case)]
pub fn print_array2d_int(str: *const i8, nr: i32, nc: i32, data: *const i32, fp: *mut i32) {
    // WARNING: signature changed — verify body
    // Previous params: (str : * const i8, nr : i32, nc : i32, data : * const i32, fp : * mut i32)
    // Previous return: ()
    todo ! ()
}

/// C: printDelayBuffer (engine/engine_print.c:125)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn print_delay_buffer(name: *const i8, buf: *const f64, nhistory: i32, dim: i32, fp: *mut i32, float_format: *const i8) {
    // WARNING: signature changed — verify body
    // Previous params: (name : * const i8, buf : * const f64, nhistory : i32, dim : i32, fp : * mut i32, float_format : * const i8)
    // Previous return: ()
    todo ! ()
}

/// C: printSparse (engine/engine_print.c:170)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn print_sparse(str: *const i8, mat: *const f64, nr: i32, rownnz: *const i32, rowadr: *const i32, colind: *const i32, fp: *mut i32, float_format: *const i8) {
    // WARNING: signature changed — verify body
    // Previous params: (str : * const i8, mat : * const f64, nr : i32, rownnz : * const i32, rowadr : * const i32, colind : * const i32, fp : * mut i32, float_format : * const i8)
    // Previous return: ()
    todo ! ()
}

/// C: printBlockArray (engine/engine_print.c:193)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn print_block_array(str: *const i8, data: *const f64, nc: i32, nisland: i32, island_nr: *const i32, island_nc: *const i32, island_r: *const i32, island_c: *const i32, map_r: *const i32, map_c: *const i32, fp: *mut i32, float_format: *const i8) {
    // WARNING: signature changed — verify body
    // Previous params: (str : * const i8, data : * const f64, nc : i32, nisland : i32, island_nr : * const i32, island_nc : * const i32, island_r : * const i32, island_c : * const i32, map_r : * const i32, map_c : * const i32, fp : * mut i32, float_format : * const i8)
    // Previous return: ()
    todo ! ()
}

/// C: printInertia (engine/engine_print.c:246)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn print_inertia(str: *const i8, mat: *const f64, m: *const mjModel, fp: *mut i32, float_format: *const i8) {
    // WARNING: signature changed — verify body
    // Previous params: (str : * const i8, mat : * const f64, m : * const mjModel, fp : * mut i32, float_format : * const i8)
    // Previous return: ()
    todo ! ()
}

/// C: mj_printSparsity (engine/engine_print.c:282)
#[allow(unused_variables, non_snake_case)]
pub fn mj_print_sparsity(str: *const i8, nr: i32, nc: i32, rowadr: *const i32, diag: *const i32, rownnz: *const i32, rowsuper: *const i32, colind: *const i32, fp: *mut i32) {
    // WARNING: signature changed — verify body
    // Previous params: (str : * const i8, nr : i32, nc : i32, rowadr : * const i32, diag : * const i32, rownnz : * const i32, rowsuper : * const i32, colind : * const i32, fp : * mut i32)
    // Previous return: ()
    todo ! ()
}

/// C: mj_printBlockSparsity (engine/engine_print.c:319)
#[allow(unused_variables, non_snake_case)]
pub fn mj_print_block_sparsity(str: *const i8, nr: i32, nc: i32, nisland: i32, island_block_ncols: *const i32, island_col_offset: *const i32, entity_island: *const i32, map_row_to_entity: *const i32, map_col_to_entity: *const i32, rownnz: *const i32, rowadr: *const i32, colind: *const i32, rowsuper: *const i32, fp: *mut i32) {
    // WARNING: signature changed — verify body
    // Previous params: (str : * const i8, nr : i32, nc : i32, nisland : i32, island_block_ncols : * const i32, island_col_offset : * const i32, entity_island : * const i32, map_row_to_entity : * const i32, map_col_to_entity : * const i32, rownnz : * const i32, rowadr : * const i32, colind : * const i32, rowsuper : * const i32, fp : * mut i32)
    // Previous return: ()
    todo ! ()
}

/// C: printVector (engine/engine_print.c:377)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn print_vector(str: *const i8, data: *const f64, n: i32, fp: *mut i32, float_format: *const i8) {
    // WARNING: signature changed — verify body
    // Previous params: (str : * const i8, data : * const f64, n : i32, fp : * mut i32, float_format : * const i8)
    // Previous return: ()
    todo ! ()
}

/// C: memorySize (engine/engine_print.c:395)
#[allow(unused_variables, non_snake_case)]
pub fn memory_size(nbytes: usize) -> *const i8 {
    // WARNING: signature changed — verify body
    // Previous params: (nbytes : usize)
    // Previous return: * const i8
    todo ! ()
}

/// C: sizeMesh (engine/engine_print.c:410)
#[allow(unused_variables, non_snake_case)]
pub fn size_mesh(m: *const mjModel) -> usize {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel)
    // Previous return: usize
    todo ! ()
}

/// C: sizeSkin (engine/engine_print.c:431)
#[allow(unused_variables, non_snake_case)]
pub fn size_skin(m: *const mjModel) -> usize {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel)
    // Previous return: usize
    todo ! ()
}

/// C: sizeBVH (engine/engine_print.c:448)
#[allow(unused_variables, non_snake_case)]
pub fn size_bvh(m: *const mjModel) -> usize {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel)
    // Previous return: usize
    todo ! ()
}

/// C: validateFloatFormat (engine/engine_print.c:463)
/// Calls: mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn validate_float_format(float_format: *const i8) -> bool {
    // WARNING: signature changed — verify body
    // Previous params: (float_format : * const i8)
    // Previous return: bool
    todo ! ()
}

/// C: mj_printFormattedModel (engine/engine_print.h:31)
/// Calls: memorySize, mj_sizeModel, mj_versionString, mju_type2Str, mju_warning, sizeBVH, sizeMesh, sizeSkin, validateFloatFormat
#[allow(unused_variables, non_snake_case)]
pub fn mj_print_formatted_model(m: *const mjModel, filename: *const i8, float_format: *const i8) {

    extern "C" { fn mj_printFormattedModel_impl(m: *const mjModel, filename: *const i8, float_format: *const i8); }
    // SAFETY: delegates to C implementation
    unsafe { mj_printFormattedModel_impl(m, filename, float_format) }
}

/// C: mj_printModel (engine/engine_print.h:35)
/// Calls: mj_printFormattedModel
#[allow(unused_variables, non_snake_case)]
pub fn mj_print_model(m: *const mjModel, filename: *const i8) {
    mj_print_formatted_model(m, filename, b"% -9.2g\0".as_ptr() as *const i8);
}

/// C: mj_printFormattedData (engine/engine_print.h:40)
/// Calls: memorySize, mj_contactForce, mj_id2name, mj_isDual, mj_isSparse, mju_isZero, mju_message, mju_warning, validateFloatFormat
#[allow(unused_variables, non_snake_case)]
pub fn mj_print_formatted_data(m: *const mjModel, d: *const mjData, filename: *const i8, float_format: *const i8) {
    extern "C" { fn mj_printFormattedData_impl(m: *const mjModel, d: *const mjData, filename: *const i8, float_format: *const i8); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_printFormattedData_impl(m, d, filename, float_format) }
}

/// C: mj_printData (engine/engine_print.h:44)
/// Calls: mj_printFormattedData
#[allow(unused_variables, non_snake_case)]
pub fn mj_print_data(m: *const mjModel, d: *const mjData, filename: *const i8) {
    extern "C" { fn mj_printData_impl(m: *const mjModel, d: *const mjData, filename: *const i8); }
    // SAFETY: delegates to C implementation
    unsafe { mj_printData_impl(m, d, filename) }
}

/// C: mj_printScene (engine/engine_print.h:51)
/// Calls: mj_printFormattedScene
#[allow(unused_variables, non_snake_case)]
pub fn mj_print_scene(s: *const mjvScene, filename: *const i8) {
    extern "C" { fn mj_printScene_impl(s: *const mjvScene, filename: *const i8); }
    // SAFETY: delegates to C implementation
    unsafe { mj_printScene_impl(s, filename) }
}

/// C: mj_printFormattedScene (engine/engine_print.h:55)
/// Calls: mju_warning, validateFloatFormat
#[allow(unused_variables, non_snake_case)]
pub fn mj_print_formatted_scene(s: *const mjvScene, filename: *const i8, float_format: *const i8) {
    extern "C" { fn mj_printFormattedScene_impl(s: *const mjvScene, filename: *const i8, float_format: *const i8); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_printFormattedScene_impl(s, filename, float_format) }
}

