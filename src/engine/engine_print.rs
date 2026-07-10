//! Port of: engine/engine_print.c
//! IR hash: 05737965add36adb
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: printInt (engine/engine_print.c:53)
#[allow(unused_variables, non_snake_case)]
pub fn print_int(fp: *mut i32, name: *const i8, value: i32) {
    extern "C" { fn printInt(fp: *mut i32, name: *const i8, value: i32); }
    // SAFETY: delegates to C implementation
    unsafe { printInt(fp, name, value) }
}

/// C: printStr (engine/engine_print.c:59)
#[allow(unused_variables, non_snake_case)]
pub fn print_str(fp: *mut i32, name: *const i8, value: *const i8) {
    extern "C" { fn printStr(fp: *mut i32, name: *const i8, value: *const i8); }
    // SAFETY: delegates to C implementation
    unsafe { printStr(fp, name, value) }
}

/// C: printNum (engine/engine_print.c:65)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn print_num(fp: *mut i32, name: *const i8, value: f32, float_format: *const i8) {
    extern "C" { fn printNum(fp: *mut i32, name: *const i8, value: f32, float_format: *const i8); }
    // SAFETY: delegates to C implementation
    unsafe { printNum(fp, name, value, float_format) }
}

/// C: printArr (engine/engine_print.c:71)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn print_arr(fp: *mut i32, name: *const i8, data: *const f32, n: i32, float_format: *const i8) {
    extern "C" { fn printArr(fp: *mut i32, name: *const i8, data: *const f32, n: i32, float_format: *const i8); }
    // SAFETY: delegates to C implementation
    unsafe { printArr(fp, name, data, n, float_format) }
}

/// C: printArray2d (engine/engine_print.c:84)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn print_array2d(str: *const i8, nr: i32, nc: i32, data: *const f64, fp: *mut i32, float_format: *const i8) {
    extern "C" { fn printArray2d(str: *const i8, nr: i32, nc: i32, data: *const f64, fp: *mut i32, float_format: *const i8); }
    // SAFETY: delegates to C implementation
    unsafe { printArray2d(str, nr, nc, data, fp, float_format) }
}

/// C: printArray2dInt (engine/engine_print.c:105)
#[allow(unused_variables, non_snake_case)]
pub fn print_array2d_int(str: *const i8, nr: i32, nc: i32, data: *const i32, fp: *mut i32) {
    extern "C" { fn printArray2dInt(str: *const i8, nr: i32, nc: i32, data: *const i32, fp: *mut i32); }
    // SAFETY: delegates to C implementation
    unsafe { printArray2dInt(str, nr, nc, data, fp) }
}

/// C: printDelayBuffer (engine/engine_print.c:125)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn print_delay_buffer(name: *const i8, buf: *const f64, nhistory: i32, dim: i32, fp: *mut i32, float_format: *const i8) {
    extern "C" { fn printDelayBuffer(name: *const i8, buf: *const f64, nhistory: i32, dim: i32, fp: *mut i32, float_format: *const i8); }
    // SAFETY: delegates to C implementation
    unsafe { printDelayBuffer(name, buf, nhistory, dim, fp, float_format) }
}

/// C: printSparse (engine/engine_print.c:170)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn print_sparse(str: *const i8, mat: *const f64, nr: i32, rownnz: *const i32, rowadr: *const i32, colind: *const i32, fp: *mut i32, float_format: *const i8) {
    extern "C" { fn printSparse(str: *const i8, mat: *const f64, nr: i32, rownnz: *const i32, rowadr: *const i32, colind: *const i32, fp: *mut i32, float_format: *const i8); }
    // SAFETY: delegates to C implementation
    unsafe { printSparse(str, mat, nr, rownnz, rowadr, colind, fp, float_format) }
}

/// C: printBlockArray (engine/engine_print.c:193)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn print_block_array(str: *const i8, data: *const f64, nc: i32, nisland: i32, island_nr: *const i32, island_nc: *const i32, island_r: *const i32, island_c: *const i32, map_r: *const i32, map_c: *const i32, fp: *mut i32, float_format: *const i8) {
    extern "C" { fn printBlockArray(str: *const i8, data: *const f64, nc: i32, nisland: i32, island_nr: *const i32, island_nc: *const i32, island_r: *const i32, island_c: *const i32, map_r: *const i32, map_c: *const i32, fp: *mut i32, float_format: *const i8); }
    // SAFETY: delegates to C implementation
    unsafe { printBlockArray(str, data, nc, nisland, island_nr, island_nc, island_r, island_c, map_r, map_c, fp, float_format) }
}

/// C: printInertia (engine/engine_print.c:246)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn print_inertia(str: *const i8, mat: *const f64, m: *const mjModel, fp: *mut i32, float_format: *const i8) {
    extern "C" { fn printInertia(str: *const i8, mat: *const f64, m: *const mjModel, fp: *mut i32, float_format: *const i8); }
    // SAFETY: delegates to C implementation
    unsafe { printInertia(str, mat, m, fp, float_format) }
}

/// C: mj_printSparsity (engine/engine_print.c:282)
#[allow(unused_variables, non_snake_case)]
pub fn mj_print_sparsity(str: *const i8, nr: i32, nc: i32, rowadr: *const i32, diag: *const i32, rownnz: *const i32, rowsuper: *const i32, colind: *const i32, fp: *mut i32) {
    extern "C" { fn mj_printSparsity(str: *const i8, nr: i32, nc: i32, rowadr: *const i32, diag: *const i32, rownnz: *const i32, rowsuper: *const i32, colind: *const i32, fp: *mut i32); }
    // SAFETY: delegates to C implementation
    unsafe { mj_printSparsity(str, nr, nc, rowadr, diag, rownnz, rowsuper, colind, fp) }
}

/// C: mj_printBlockSparsity (engine/engine_print.c:319)
#[allow(unused_variables, non_snake_case)]
pub fn mj_print_block_sparsity(str: *const i8, nr: i32, nc: i32, nisland: i32, island_block_ncols: *const i32, island_col_offset: *const i32, entity_island: *const i32, map_row_to_entity: *const i32, map_col_to_entity: *const i32, rownnz: *const i32, rowadr: *const i32, colind: *const i32, rowsuper: *const i32, fp: *mut i32) {
    extern "C" { fn mj_printBlockSparsity(str: *const i8, nr: i32, nc: i32, nisland: i32, island_block_ncols: *const i32, island_col_offset: *const i32, entity_island: *const i32, map_row_to_entity: *const i32, map_col_to_entity: *const i32, rownnz: *const i32, rowadr: *const i32, colind: *const i32, rowsuper: *const i32, fp: *mut i32); }
    // SAFETY: delegates to C implementation
    unsafe { mj_printBlockSparsity(str, nr, nc, nisland, island_block_ncols, island_col_offset, entity_island, map_row_to_entity, map_col_to_entity, rownnz, rowadr, colind, rowsuper, fp) }
}

/// C: printVector (engine/engine_print.c:377)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn print_vector(str: *const i8, data: *const f64, n: i32, fp: *mut i32, float_format: *const i8) {
    extern "C" { fn printVector(str: *const i8, data: *const f64, n: i32, fp: *mut i32, float_format: *const i8); }
    // SAFETY: delegates to C implementation
    unsafe { printVector(str, data, n, fp, float_format) }
}

/// C: memorySize (engine/engine_print.c:395)
#[allow(unused_variables, non_snake_case)]
pub fn memory_size(nbytes: usize) -> *const i8 {
    extern "C" {
        fn snprintf(s: *mut i8, n: usize, format: *const i8, ...) -> i32;
    }
    // SAFETY: Uses a static buffer (mirrors C thread-local static).
    // snprintf is standard C library. The returned pointer is valid until next call.
    static mut MESSAGE: [i8; 32] = [0; 32];
    unsafe {
        let k: usize = 1024;
        if nbytes < k {
            snprintf(
                MESSAGE.as_mut_ptr(), 32,
                b"%5zu bytes\0".as_ptr() as *const i8,
                nbytes,
            );
        } else {
            snprintf(
                MESSAGE.as_mut_ptr(), 32,
                b"%7.0f KB\0".as_ptr() as *const i8,
                nbytes as f64 / k as f64,
            );
        }
        MESSAGE.as_ptr()
    }
}

/// C: sizeMesh (engine/engine_print.c:410)
#[allow(unused_variables, non_snake_case)]
pub fn size_mesh(m: *const mjModel) -> usize {
    // SAFETY: m is a valid mjModel pointer with all size fields valid.
    unsafe {
        let mut nbytes: usize = 0;
        nbytes += 4 * 3 * (*m).nmeshvert;       // mesh_vert (float*3)
        nbytes += 4 * 3 * (*m).nmeshnormal;     // mesh_normal (float*3)
        nbytes += 4 * 2 * (*m).nmeshtexcoord;   // mesh_texcoord (float*2)
        nbytes += 4 * 3 * (*m).nmeshface;       // mesh_face (int*3)
        nbytes += 4 * 3 * (*m).nmeshface;       // mesh_facenormal (int*3)
        nbytes += 4 * 3 * (*m).nmeshface;       // mesh_facetexcoord (int*3)
        nbytes += 4 * (*m).nmeshgraph;          // mesh_graph (int)
        nbytes += 8 * 3 * (*m).nmeshpoly;       // mesh_polynormal (mjtNum*3)
        nbytes += 4 * (*m).nmeshpoly;           // mesh_polyvertadr (int)
        nbytes += 4 * (*m).nmeshpoly;           // mesh_polyvertnum (int)
        nbytes += 4 * (*m).nmeshpolyvert;       // mesh_polyvert (int)
        nbytes += 4 * (*m).nmeshvert;           // mesh_polymapadr (int)
        nbytes += 4 * (*m).nmeshvert;           // mesh_polymapnum (int)
        nbytes += 4 * (*m).nmeshpolymap;        // mesh_polymap (int)
        nbytes
    }
}

/// C: sizeSkin (engine/engine_print.c:431)
#[allow(unused_variables, non_snake_case)]
pub fn size_skin(m: *const mjModel) -> usize {
    // SAFETY: m is a valid mjModel pointer with all size fields valid.
    unsafe {
        let mut nbytes: usize = 0;
        nbytes += 4 * 3 * (*m).nskinvert;       // skin_vert (float*3)
        nbytes += 4 * 2 * (*m).nskintexvert;    // skin_texcoord (float*2)
        nbytes += 4 * 3 * (*m).nskinface;       // skin_face (int*3)
        nbytes += 4 * (*m).nskinbone;           // skin_bonevertadr (int)
        nbytes += 4 * (*m).nskinbone;           // skin_bonevertnum (int)
        nbytes += 4 * 3 * (*m).nskinbone;       // skin_bonebindpos (float*3)
        nbytes += 4 * 4 * (*m).nskinbone;       // skin_bonebindquat (float*4)
        nbytes += 4 * (*m).nskinbone;           // skin_bonebodyid (int)
        nbytes += 4 * (*m).nskinbonevert;       // skin_bonevertid (int)
        nbytes += 4 * (*m).nskinbonevert;       // skin_bonevertweight (float)
        nbytes
    }
}

/// C: sizeBVH (engine/engine_print.c:448)
#[allow(unused_variables, non_snake_case)]
pub fn size_bvh(m: *const mjModel) -> usize {
    // SAFETY: m is a valid mjModel pointer with all size fields valid.
    unsafe {
        let mut nbytes: usize = 0;
        nbytes += 4 * (*m).nbvh;                // bvh_depth (int)
        nbytes += 4 * 2 * (*m).nbvh;            // bvh_child (int*2)
        nbytes += 4 * (*m).nbvh;                // bvh_nodeid (int)
        nbytes += 8 * 6 * (*m).nbvhstatic;      // bvh_aabb (mjtNum*6)
        nbytes += 4 * (*m).noct;                // oct_depth (int)
        nbytes += 4 * 8 * (*m).noct;            // oct_child (int*8)
        nbytes += 8 * 6 * (*m).noct;            // oct_aabb (mjtNum*6)
        nbytes += 8 * 8 * (*m).noct;            // oct_coeff (mjtNum*8)
        nbytes
    }
}

/// C: validateFloatFormat (engine/engine_print.c:463)
/// Calls: mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn validate_float_format(float_format: *const i8) -> bool {
    extern "C" { fn validateFloatFormat(float_format: *const i8) -> bool; }
    // SAFETY: delegates to C implementation
    unsafe { validateFloatFormat(float_format) }
}

/// C: mj_printFormattedModel (engine/engine_print.h:31)
/// Calls: memorySize, mj_sizeModel, mj_versionString, mju_type2Str, mju_warning, sizeBVH, sizeMesh, sizeSkin, validateFloatFormat
#[allow(unused_variables, non_snake_case)]
pub fn mj_print_formatted_model(m: *const mjModel, filename: *const i8, float_format: *const i8) {

    extern "C" { fn mj_printFormattedModel(m: *const mjModel, filename: *const i8, float_format: *const i8); }
    // SAFETY: delegates to C implementation which uses fprintf/MJMODEL_POINTERS macros
    unsafe { mj_printFormattedModel(m, filename, float_format) }
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
    extern "C" { fn mj_printFormattedData(m: *const mjModel, d: *const mjData, filename: *const i8, float_format: *const i8); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_printFormattedData(m, d, filename, float_format) }
}

/// C: mj_printData (engine/engine_print.h:44)
/// Calls: mj_printFormattedData
#[allow(unused_variables, non_snake_case)]
pub fn mj_print_data(m: *const mjModel, d: *const mjData, filename: *const i8) {
    mj_print_formatted_data(m, d, filename, b"% -9.2g\0".as_ptr() as *const i8);
}

/// C: mj_printScene (engine/engine_print.h:51)
/// Calls: mj_printFormattedScene
#[allow(unused_variables, non_snake_case)]
pub fn mj_print_scene(s: *const mjvScene, filename: *const i8) {
    // C: mj_printScene calls mj_printFormattedScene with FLOAT_FORMAT = "% -9.2g"
    mj_print_formatted_scene(s, filename, b"% -9.2g\0".as_ptr() as *const i8);
}

/// C: mj_printFormattedScene (engine/engine_print.h:55)
/// Calls: mju_warning, validateFloatFormat
#[allow(unused_variables, non_snake_case)]
pub fn mj_print_formatted_scene(s: *const mjvScene, filename: *const i8, float_format: *const i8) {
    extern "C" { fn mj_printFormattedScene(s: *const mjvScene, filename: *const i8, float_format: *const i8); }
    // SAFETY: delegates to C implementation which uses fprintf/FILE*
    unsafe { mj_printFormattedScene(s, filename, float_format) }
}

