//! Port of: engine/engine_print.c
//! IR hash: 9614bd3d92e7766b
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: printInt (engine/engine_print.c:53)
#[allow(unused_variables, non_snake_case)]
pub fn print_int(fp: *mut FILE, name: *const i8, value: i32) {
    extern "C" {
        fn fprintf(fp: *mut FILE, fmt: *const i8, ...) -> i32;
    }

    // SAFETY: fp is a valid FILE pointer, name is a valid C string (caller contract)
    unsafe {
        fprintf(fp, b"%-21s\0".as_ptr() as *const i8, name);
        fprintf(fp, b" %d\0".as_ptr() as *const i8, value);
        fprintf(fp, b"\n\0".as_ptr() as *const i8);
    }
}

/// C: printStr (engine/engine_print.c:59)
#[allow(unused_variables, non_snake_case)]
pub fn print_str(fp: *mut FILE, name: *const i8, value: *const i8) {
    extern "C" {
        fn fprintf(fp: *mut FILE, fmt: *const i8, ...) -> i32;
    }

    // SAFETY: fp is a valid FILE pointer, name/value are valid C strings (caller contract)
    unsafe {
        fprintf(fp, b"%-21s\0".as_ptr() as *const i8, name);
        let empty = b"\0".as_ptr() as *const i8;
        fprintf(fp, b"%s\0".as_ptr() as *const i8, if !value.is_null() { value } else { empty });
        fprintf(fp, b"\n\0".as_ptr() as *const i8);
    }
}

/// C: printNum (engine/engine_print.c:65)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn print_num(fp: *mut FILE, name: *const i8, value: f32, float_format: *const i8) {
    extern "C" {
        fn fprintf(fp: *mut FILE, fmt: *const i8, ...) -> i32;
    }

    // SAFETY: fp is a valid FILE pointer, name/float_format are valid C strings (caller contract)
    unsafe {
        fprintf(fp, b"%-21s\0".as_ptr() as *const i8, name);
        fprintf(fp, float_format, value as f64);
        fprintf(fp, b"\n\0".as_ptr() as *const i8);
    }
}

/// C: printArr (engine/engine_print.c:71)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn print_arr(fp: *mut FILE, name: *const i8, data: *const f32, n: i32, float_format: *const i8) {
    extern "C" {
        fn fprintf(fp: *mut FILE, fmt: *const i8, ...) -> i32;
    }

    if data.is_null() {
        return;
    }

    // SAFETY: fp valid, data points to n floats, name/float_format are valid C strings
    unsafe {
        fprintf(fp, b"%-21s\0".as_ptr() as *const i8, name);
        for i in 0..n as usize {
            fprintf(fp, float_format, *data.add(i) as f64);
            fprintf(fp, b" \0".as_ptr() as *const i8);
        }
        fprintf(fp, b"\n\0".as_ptr() as *const i8);
    }
}

/// C: printArray2d (engine/engine_print.c:84)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn print_array2d(str: *const i8, nr: i32, nc: i32, data: *const f64, fp: *mut FILE, float_format: *const i8) {
    extern "C" {
        fn fprintf(fp: *mut FILE, fmt: *const i8, ...) -> i32;
    }

    if data.is_null() {
        return;
    }

    // SAFETY: fp valid, data points to nr*nc doubles, str/float_format are valid C strings
    unsafe {
        if nr != 0 && nc != 0 {
            fprintf(fp, b"%s\n\0".as_ptr() as *const i8, str);
            for r in 0..nr {
                fprintf(fp, b" \0".as_ptr() as *const i8);
                for c in 0..nc {
                    fprintf(fp, b" \0".as_ptr() as *const i8);
                    fprintf(fp, float_format, *data.add((c + r * nc) as usize));
                }
                fprintf(fp, b"\n\0".as_ptr() as *const i8);
            }
            fprintf(fp, b"\n\0".as_ptr() as *const i8);
        }
    }
}

/// C: printArray2dInt (engine/engine_print.c:105)
#[allow(unused_variables, non_snake_case)]
pub fn print_array2d_int(str: *const i8, nr: i32, nc: i32, data: *const i32, fp: *mut FILE) {
    todo!() // printArray2dInt
}

/// C: printDelayBuffer (engine/engine_print.c:125)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn print_delay_buffer(name: *const i8, buf: *const f64, nhistory: i32, dim: i32, fp: *mut FILE, float_format: *const i8) {
    todo!() // printDelayBuffer
}

/// C: printSparse (engine/engine_print.c:170)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn print_sparse(str: *const i8, mat: *const f64, nr: i32, rownnz: *const i32, rowadr: *const i32, colind: *const i32, fp: *mut FILE, float_format: *const i8) {
    todo!() // printSparse
}

/// C: printBlockArray (engine/engine_print.c:193)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn print_block_array(str: *const i8, data: *const f64, nc: i32, nisland: i32, island_nr: *const i32, island_nc: *const i32, island_r: *const i32, island_c: *const i32, map_r: *const i32, map_c: *const i32, fp: *mut FILE, float_format: *const i8) {
    todo!() // printBlockArray
}

/// C: printInertia (engine/engine_print.c:246)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn print_inertia(str: *const i8, mat: *const f64, m: *const mjModel, fp: *mut FILE, float_format: *const i8) {
    todo!() // printInertia
}

/// C: mj_printBlockSparsity (engine/engine_print.c:319)
#[allow(unused_variables, non_snake_case)]
pub fn mj_print_block_sparsity(str: *const i8, nr: i32, nc: i32, nisland: i32, island_block_ncols: *const i32, island_col_offset: *const i32, entity_island: *const i32, map_row_to_entity: *const i32, map_col_to_entity: *const i32, rownnz: *const i32, rowadr: *const i32, colind: *const i32, rowsuper: *const i32, fp: *mut FILE) {
    todo!() // mj_printBlockSparsity
}

/// C: printVector (engine/engine_print.c:377)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn print_vector(str: *const i8, data: *const f64, n: i32, fp: *mut FILE, float_format: *const i8) {
    todo!() // printVector
}

/// C: memorySize (engine/engine_print.c:395)
#[allow(unused_variables, non_snake_case)]
pub fn memory_size(nbytes: usize) -> *const i8 {
    thread_local! {
        static MESSAGE: std::cell::RefCell<[u8; 32]> = std::cell::RefCell::new([0u8; 32]);
    }
    extern "C" {
        fn snprintf(s: *mut i8, n: usize, fmt: *const i8, ...) -> i32;
    }

    let k: usize = 1024;

    MESSAGE.with(|cell| {
        let mut buf = cell.borrow_mut();
        let ptr = buf.as_mut_ptr() as *mut i8;
        // SAFETY: ptr points to 32-byte thread-local buffer, snprintf will not overflow
        unsafe {
            if nbytes < k {
                snprintf(ptr, 32, b"%5zu bytes\0".as_ptr() as *const i8, nbytes);
            } else {
                snprintf(ptr, 32, b"%7.0f KB\0".as_ptr() as *const i8, nbytes as f64 / k as f64);
            }
        }
        ptr as *const i8
    })
}

/// C: sizeMesh (engine/engine_print.c:410)
#[allow(unused_variables, non_snake_case)]
pub fn size_mesh(m: *const mjModel) -> usize {
    todo!() // sizeMesh
}

/// C: sizeSkin (engine/engine_print.c:431)
#[allow(unused_variables, non_snake_case)]
pub fn size_skin(m: *const mjModel) -> usize {
    todo!() // sizeSkin
}

/// C: sizeBVH (engine/engine_print.c:448)
#[allow(unused_variables, non_snake_case)]
pub fn size_bvh(m: *const mjModel) -> usize {
    todo!() // sizeBVH
}

/// C: validateFloatFormat (engine/engine_print.c:463)
/// Calls: mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn validate_float_format(float_format: *const i8) -> bool {
    todo!() // validateFloatFormat
}

/// C: mj_printFormattedModel (engine/engine_print.h:31)
/// Calls: memorySize, mj_printSparsity, mj_sizeModel, mj_versionString, mju_type2Str, mju_warning, printArray2dInt, sizeBVH, sizeMesh, sizeSkin, validateFloatFormat
#[allow(unused_variables, non_snake_case)]
pub fn mj_print_formatted_model(m: *const mjModel, filename: *const i8, float_format: *const i8) {
    todo!() // mj_printFormattedModel
}

/// C: mj_printModel (engine/engine_print.h:35)
/// Calls: mj_printFormattedModel
#[allow(unused_variables, non_snake_case)]
pub fn mj_print_model(m: *const mjModel, filename: *const i8) {
    todo!() // mj_printModel
}

/// C: mj_printFormattedData (engine/engine_print.h:40)
/// Calls: memorySize, mj_contactForce, mj_id2name, mj_isDual, mj_isSparse, mj_printBlockSparsity, mj_printSparsity, mju_isZero, mju_message, mju_warning, printArray2d, printArray2dInt, printBlockArray, printDelayBuffer, printInertia, printSparse, printVector, validateFloatFormat
#[allow(unused_variables, non_snake_case)]
pub fn mj_print_formatted_data(m: *const mjModel, d: *const mjData, filename: *const i8, float_format: *const i8) {
    todo!() // mj_printFormattedData
}

/// C: mj_printData (engine/engine_print.h:44)
/// Calls: mj_printFormattedData
#[allow(unused_variables, non_snake_case)]
pub fn mj_print_data(m: *const mjModel, d: *const mjData, filename: *const i8) {
    todo!() // mj_printData
}

/// C: mj_printSparsity (engine/engine_print.h:47)
#[allow(unused_variables, non_snake_case)]
pub fn mj_print_sparsity(str: *const i8, nr: i32, nc: i32, rowadr: *const i32, diag: *const i32, rownnz: *const i32, rowsuper: *const i32, colind: *const i32, fp: *mut FILE) {
    todo!() // mj_printSparsity
}

/// C: mj_printScene (engine/engine_print.h:51)
/// Calls: mj_printFormattedScene
#[allow(unused_variables, non_snake_case)]
pub fn mj_print_scene(s: *const mjvScene, filename: *const i8) {
    todo!() // mj_printScene
}

/// C: mj_printFormattedScene (engine/engine_print.h:55)
/// Calls: mju_warning, printArr, printInt, printNum, printStr, validateFloatFormat
#[allow(unused_variables, non_snake_case)]
pub fn mj_print_formatted_scene(s: *const mjvScene, filename: *const i8, float_format: *const i8) {
    todo!() // mj_printFormattedScene
}

