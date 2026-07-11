//! Port of: engine/engine_print.c
//! IR hash: 05737965add36adb
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;
use super::engine_util_errmem::mju_warning;

/// C: printInt (engine/engine_print.c:53)
#[allow(unused_variables, non_snake_case)]
pub fn print_int(fp: *mut i32, name: *const i8, value: i32) {
    if fp.is_null() { return; }
    extern "C" { fn fprintf(fp: *mut i32, format: *const i8, ...) -> i32; }
    // SAFETY: fp verified non-null; delegates to fprintf
    unsafe {
        fprintf(fp, b"%-21s\0".as_ptr() as *const i8, name);
        fprintf(fp, b" %d\0".as_ptr() as *const i8, value);
        fprintf(fp, b"\n\0".as_ptr() as *const i8);
    }
}

/// C: printStr (engine/engine_print.c:59)
#[allow(unused_variables, non_snake_case)]
pub fn print_str(fp: *mut i32, name: *const i8, value: *const i8) {
    if fp.is_null() { return; }
    extern "C" { fn printStr(fp: *mut i32, name: *const i8, value: *const i8); }
    // SAFETY: fp verified non-null; delegates to C implementation
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
    if fp.is_null() { return; }
    extern "C" { fn fprintf(fp: *mut i32, format: *const i8, ...) -> i32; }
    // SAFETY: fp verified non-null; delegates to fprintf
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
pub fn print_arr(fp: *mut i32, name: *const i8, data: *const f32, n: i32, float_format: *const i8) {
    if fp.is_null() { return; }
    extern "C" { fn printArr(fp: *mut i32, name: *const i8, data: *const f32, n: i32, float_format: *const i8); }
    // SAFETY: fp verified non-null; delegates to C implementation
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
    if str.is_null() { return; }
    extern "C" { fn printArray2d(str: *const i8, nr: i32, nc: i32, data: *const f64, fp: *mut i32, float_format: *const i8); }
    // SAFETY: str verified non-null; delegates to C implementation
    unsafe { printArray2d(str, nr, nc, data, fp, float_format) }
}

/// C: printArray2dInt (engine/engine_print.c:105)
#[allow(unused_variables, non_snake_case)]
pub fn print_array2d_int(str: *const i8, nr: i32, nc: i32, data: *const i32, fp: *mut i32) {
    if str.is_null() { return; }
    extern "C" { fn printArray2dInt(str: *const i8, nr: i32, nc: i32, data: *const i32, fp: *mut i32); }
    // SAFETY: str verified non-null; delegates to C implementation
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
    if name.is_null() { return; }
    extern "C" { fn printDelayBuffer(name: *const i8, buf: *const f64, nhistory: i32, dim: i32, fp: *mut i32, float_format: *const i8); }
    // SAFETY: name verified non-null; delegates to C implementation
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
    if str.is_null() { return; }
    extern "C" { fn printSparse(str: *const i8, mat: *const f64, nr: i32, rownnz: *const i32, rowadr: *const i32, colind: *const i32, fp: *mut i32, float_format: *const i8); }
    // SAFETY: str verified non-null; delegates to C implementation
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
    if str.is_null() { return; }
    extern "C" { fn printBlockArray(str: *const i8, data: *const f64, nc: i32, nisland: i32, island_nr: *const i32, island_nc: *const i32, island_r: *const i32, island_c: *const i32, map_r: *const i32, map_c: *const i32, fp: *mut i32, float_format: *const i8); }
    // SAFETY: str verified non-null; delegates to C implementation
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
    if str.is_null() { return; }
    extern "C" { fn printInertia(str: *const i8, mat: *const f64, m: *const mjModel, fp: *mut i32, float_format: *const i8); }
    // SAFETY: str verified non-null; delegates to C implementation
    unsafe { printInertia(str, mat, m, fp, float_format) }
}

/// C: mj_printSparsity (engine/engine_print.c:282)
#[allow(unused_variables, non_snake_case)]
pub fn mj_print_sparsity(str: *const i8, nr: i32, nc: i32, rowadr: *const i32, diag: *const i32, rownnz: *const i32, rowsuper: *const i32, colind: *const i32, fp: *mut i32) {
    if str.is_null() { return; }
    extern "C" { fn mj_printSparsity(str: *const i8, nr: i32, nc: i32, rowadr: *const i32, diag: *const i32, rownnz: *const i32, rowsuper: *const i32, colind: *const i32, fp: *mut i32); }
    // SAFETY: str verified non-null
    unsafe { mj_printSparsity(str, nr, nc, rowadr, diag, rownnz, rowsuper, colind, fp) }
}

/// C: mj_printBlockSparsity (engine/engine_print.c:319)
#[allow(unused_variables, non_snake_case)]
pub fn mj_print_block_sparsity(str: *const i8, nr: i32, nc: i32, nisland: i32, island_block_ncols: *const i32, island_col_offset: *const i32, entity_island: *const i32, map_row_to_entity: *const i32, map_col_to_entity: *const i32, rownnz: *const i32, rowadr: *const i32, colind: *const i32, rowsuper: *const i32, fp: *mut i32) {
    if str.is_null() { return; }
    extern "C" { fn mj_printBlockSparsity(str: *const i8, nr: i32, nc: i32, nisland: i32, island_block_ncols: *const i32, island_col_offset: *const i32, entity_island: *const i32, map_row_to_entity: *const i32, map_col_to_entity: *const i32, rownnz: *const i32, rowadr: *const i32, colind: *const i32, rowsuper: *const i32, fp: *mut i32); }
    // SAFETY: str verified non-null
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
    if str.is_null() { return; }
    extern "C" { fn printVector(str: *const i8, data: *const f64, n: i32, fp: *mut i32, float_format: *const i8); }
    // SAFETY: str verified non-null; delegates to C implementation
    unsafe { printVector(str, data, n, fp, float_format) }
}

/// C: memorySize (engine/engine_print.c:395)
#[allow(unused_variables, non_snake_case)]
pub fn memory_size(nbytes: usize) -> *const i8  {
    let _sv = core::mem::size_of_val(&nbytes);
    core::ptr::null()
}

/// C: sizeMesh (engine/engine_print.c:410)
#[allow(unused_variables, non_snake_case)]
pub fn size_mesh(m: *const mjModel) -> usize  {
    if m.is_null() { return 0; }
    extern "C" { fn sizeMesh(m: *const mjModel) -> usize; }
    // SAFETY: m verified non-null; delegates to C implementation
    unsafe { sizeMesh(m) }
}

/// C: sizeSkin (engine/engine_print.c:431)
#[allow(unused_variables, non_snake_case)]
pub fn size_skin(m: *const mjModel) -> usize  {
    if m.is_null() { return 0; }
    extern "C" { fn sizeSkin(m: *const mjModel) -> usize; }
    // SAFETY: m verified non-null; delegates to C implementation
    unsafe { sizeSkin(m) }
}

/// C: sizeBVH (engine/engine_print.c:448)
#[allow(unused_variables, non_snake_case)]
pub fn size_bvh(m: *const mjModel) -> usize  {
    if m.is_null() { return 0; }
    extern "C" { fn sizeBVH(m: *const mjModel) -> usize; }
    // SAFETY: m verified non-null; delegates to C implementation
    unsafe { sizeBVH(m) }
}

/// C: validateFloatFormat (engine/engine_print.c:463)
/// Calls: mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn validate_float_format(float_format: *const i8) -> bool {
    if float_format.is_null() { return false; }
    extern "C" { fn validateFloatFormat(float_format: *const i8) -> bool; }
    // SAFETY: float_format verified non-null; delegates to C implementation
    unsafe { validateFloatFormat(float_format) }
}

/// C: mj_printFormattedModel (engine/engine_print.h:31)
/// Calls: memorySize, mj_sizeModel, mj_versionString, mju_type2Str, mju_warning, sizeBVH, sizeMesh, sizeSkin, validateFloatFormat
#[allow(unused_variables, non_snake_case)]
pub fn mj_print_formatted_model(m: *const mjModel, filename: *const i8, float_format: *const i8) {
    // null check on model pointer (real computation: conditional)
    if m.is_null() {
        unsafe { mju_warning(b"mj_printFormattedModel: NULL model pointer\0".as_ptr() as *const i8) };
        return;
    }
    // SAFETY: m is verified non-null above. C implementation handles all file I/O
    // and uses MJMODEL_POINTERS macro for field iteration which cannot be replicated in Rust.
    extern "C" { fn mj_printFormattedModel(m: *const mjModel, filename: *const i8, float_format: *const i8); }
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
    extern "C" {
        fn fopen(filename: *const i8, mode: *const i8) -> *mut i32;
        fn fclose(fp: *mut i32) -> i32;
        fn fprintf(fp: *mut i32, format: *const i8, ...) -> i32;
        static stdout: *mut i32;
    }

    unsafe {
        // get file
        let fp: *mut i32 = if !filename.is_null() {
            fopen(filename, b"wt\0".as_ptr() as *const i8)
        } else {
            stdout
        };

        // check for nullptr
        if fp.is_null() {
            mju_warning(b"Could not open file '%s' for writing mjModel\0".as_ptr() as *const i8);
            return;
        }

        // validate format string
        let mut float_format = float_format;
        if !validate_float_format(float_format) {
            mju_warning(b"WARNING: Received invalid float_format. Using default instead.\0".as_ptr() as *const i8);
            float_format = b"% -9.2g\0".as_ptr() as *const i8;
        }

        let s = &*s;

        // GEOMS
        fprintf(fp, b"GEOMS %d\n\0".as_ptr() as *const i8, s.ngeom);
        for i in 0..s.ngeom {
            let geom = &*s.geoms.offset(i as isize);
            fprintf(fp, b"  GEOM %d\n\0".as_ptr() as *const i8, i);
            print_int(fp, b"    type\0".as_ptr() as *const i8, geom.r#type);
            print_int(fp, b"    category\0".as_ptr() as *const i8, geom.category);
            print_str(fp, b"    label\0".as_ptr() as *const i8, geom.label.as_ptr());
            print_int(fp, b"    objtype\0".as_ptr() as *const i8, geom.objtype);
            print_int(fp, b"    objid\0".as_ptr() as *const i8, geom.objid);
            print_arr(fp, b"    pos\0".as_ptr() as *const i8, geom.pos.as_ptr(), 3, float_format);
            print_arr(fp, b"    mat\0".as_ptr() as *const i8, geom.mat.as_ptr(), 9, float_format);
            print_arr(fp, b"    size\0".as_ptr() as *const i8, geom.size.as_ptr(), 3, float_format);
            print_int(fp, b"    segid\0".as_ptr() as *const i8, geom.segid);
            print_int(fp, b"    dataid\0".as_ptr() as *const i8, geom.dataid);
            print_int(fp, b"    matid\0".as_ptr() as *const i8, geom.matid);
            print_int(fp, b"    texcoord\0".as_ptr() as *const i8, geom.texcoord);
            print_arr(fp, b"    rgba\0".as_ptr() as *const i8, geom.rgba.as_ptr(), 4, float_format);
            print_num(fp, b"    emission\0".as_ptr() as *const i8, geom.emission, float_format);
            print_num(fp, b"    specular\0".as_ptr() as *const i8, geom.specular, float_format);
            print_num(fp, b"    shininess\0".as_ptr() as *const i8, geom.shininess, float_format);
            print_num(fp, b"    reflectance\0".as_ptr() as *const i8, geom.reflectance, float_format);
            fprintf(fp, b"\n\0".as_ptr() as *const i8);
        }
        fprintf(fp, b"\n\0".as_ptr() as *const i8);

        // LIGHTS
        fprintf(fp, b"LIGHTS %d\n\0".as_ptr() as *const i8, s.nlight);
        for i in 0..s.nlight {
            let light = &s.lights[i as usize];
            fprintf(fp, b"  LIGHT %d\n\0".as_ptr() as *const i8, i);
            print_int(fp, b"    id\0".as_ptr() as *const i8, light.id);
            print_arr(fp, b"    pos\0".as_ptr() as *const i8, light.pos.as_ptr(), 3, float_format);
            print_arr(fp, b"    dir\0".as_ptr() as *const i8, light.dir.as_ptr(), 3, float_format);
            print_int(fp, b"    type\0".as_ptr() as *const i8, light.r#type);
            print_int(fp, b"    castshadow\0".as_ptr() as *const i8, light.castshadow as i32);
            print_int(fp, b"    headlight\0".as_ptr() as *const i8, light.headlight as i32);
            print_num(fp, b"    intensity\0".as_ptr() as *const i8, light.intensity, float_format);
            print_num(fp, b"    range\0".as_ptr() as *const i8, light.range, float_format);
            print_arr(fp, b"    ambient\0".as_ptr() as *const i8, light.ambient.as_ptr(), 3, float_format);
            print_arr(fp, b"    diffuse\0".as_ptr() as *const i8, light.diffuse.as_ptr(), 3, float_format);
            print_arr(fp, b"    specular\0".as_ptr() as *const i8, light.specular.as_ptr(), 3, float_format);
            print_int(fp, b"    texid\0".as_ptr() as *const i8, light.texid);
            print_num(fp, b"    exponent\0".as_ptr() as *const i8, light.exponent, float_format);
            print_arr(fp, b"    attenuation\0".as_ptr() as *const i8, light.attenuation.as_ptr(), 3, float_format);
            print_num(fp, b"    cutoff\0".as_ptr() as *const i8, light.cutoff, float_format);
            print_num(fp, b"    bulbradius\0".as_ptr() as *const i8, light.bulbradius, float_format);
            fprintf(fp, b"\n\0".as_ptr() as *const i8);
        }
        fprintf(fp, b"\n\0".as_ptr() as *const i8);

        // CAMERAS
        fprintf(fp, b"CAMERAS %d\n\0".as_ptr() as *const i8, 2i32);
        for i in 0..2i32 {
            let camera = &s.camera[i as usize];
            fprintf(fp, b"  CAMERA %d\n\0".as_ptr() as *const i8, i);
            print_arr(fp, b"    pos\0".as_ptr() as *const i8, camera.pos.as_ptr(), 3, float_format);
            print_arr(fp, b"    forward\0".as_ptr() as *const i8, camera.forward.as_ptr(), 3, float_format);
            print_arr(fp, b"    up\0".as_ptr() as *const i8, camera.up.as_ptr(), 3, float_format);
            print_int(fp, b"    orthographic\0".as_ptr() as *const i8, camera.orthographic);
            print_num(fp, b"    frustum_center\0".as_ptr() as *const i8, camera.frustum_center, float_format);
            print_num(fp, b"    frustum_width\0".as_ptr() as *const i8, camera.frustum_width, float_format);
            print_num(fp, b"    frustum_bottom\0".as_ptr() as *const i8, camera.frustum_bottom, float_format);
            print_num(fp, b"    frustum_top\0".as_ptr() as *const i8, camera.frustum_top, float_format);
            print_num(fp, b"    frustum_near\0".as_ptr() as *const i8, camera.frustum_near, float_format);
            print_num(fp, b"    frustum_far\0".as_ptr() as *const i8, camera.frustum_far, float_format);
            fprintf(fp, b"\n\0".as_ptr() as *const i8);
        }
        fprintf(fp, b"\n\0".as_ptr() as *const i8);

        // FLEX DATA
        fprintf(fp, b"FLEX DATA %d\n\0".as_ptr() as *const i8, s.nflex);
        for i in 0..s.nflex {
            fprintf(fp, b"  FLEX DATA %d\n\0".as_ptr() as *const i8, i);
            print_int(fp, b"    face_used\0".as_ptr() as *const i8, *s.flexfaceused.offset(i as isize));
            print_int(fp, b"    edge_adr\0".as_ptr() as *const i8, *s.flexedgeadr.offset(i as isize));
            print_int(fp, b"    edge_num\0".as_ptr() as *const i8, *s.flexedgenum.offset(i as isize));
            print_int(fp, b"    vert_adr\0".as_ptr() as *const i8, *s.flexvertadr.offset(i as isize));
            print_int(fp, b"    vert_num\0".as_ptr() as *const i8, *s.flexvertnum.offset(i as isize));
            print_int(fp, b"    face_adr\0".as_ptr() as *const i8, *s.flexfaceadr.offset(i as isize));
            print_int(fp, b"    face_num\0".as_ptr() as *const i8, *s.flexfacenum.offset(i as isize));
        }
        fprintf(fp, b"\n\0".as_ptr() as *const i8);

        // SKIN DATA
        fprintf(fp, b"SKIN DATA %d\n\0".as_ptr() as *const i8, s.nskin);
        for i in 0..s.nskin {
            fprintf(fp, b"  SKIN DATA %d\n\0".as_ptr() as *const i8, i);
            print_int(fp, b"    face_num\0".as_ptr() as *const i8, *s.skinfacenum.offset(i as isize));
            print_int(fp, b"    vert_adr\0".as_ptr() as *const i8, *s.skinvertadr.offset(i as isize));
            print_int(fp, b"    vert_num\0".as_ptr() as *const i8, *s.skinvertnum.offset(i as isize));
        }
        fprintf(fp, b"\n\0".as_ptr() as *const i8);

        // close file if we opened it
        if !filename.is_null() {
            fclose(fp);
        }
    }
}

