//! Port of: engine/engine_print.c
//! IR hash: 9343293228317031
//! CODEGEN: source paths, owners, and callable names are locked.

use crate::types::*;

/// C: printInt (engine/engine_print.c:53)
#[allow(unused_variables, non_snake_case)]
pub fn printInt(fp: *mut FILE, name: *const i8, value: i32) {
    extern "C" {
        fn fprintf(stream: *mut FILE, fmt: *const i8, ...) -> i32;
    }
    // SAFETY: fp, name are valid pointers (caller contract)
    unsafe {
        fprintf(fp, b"%-21s\0".as_ptr() as *const i8, name);
        fprintf(fp, b" %d\0".as_ptr() as *const i8, value);
        fprintf(fp, b"\n\0".as_ptr() as *const i8);
    }
}

/// C: printStr (engine/engine_print.c:59)
#[allow(unused_variables, non_snake_case)]
pub fn printStr(fp: *mut FILE, name: *const i8, value: *const i8) {
    extern "C" {
        fn fprintf(stream: *mut FILE, fmt: *const i8, ...) -> i32;
    }
    // SAFETY: fp, name, value are valid pointers (caller contract)
    unsafe {
        fprintf(fp, b"%-21s\0".as_ptr() as *const i8, name);
        fprintf(fp, b"%s\0".as_ptr() as *const i8, if value.is_null() { b"\0".as_ptr() as *const i8 } else { value });
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
pub fn printNum(fp: *mut FILE, name: *const i8, value: f32, float_format: *const i8) {
    extern "C" {
        fn fprintf(stream: *mut FILE, fmt: *const i8, ...) -> i32;
    }
    // SAFETY: fp, name, float_format are valid pointers (caller contract)
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
pub fn printArr(fp: *mut FILE, name: *const i8, data: *const f32, n: i32, float_format: *const i8) {
    extern "C" {
        fn fprintf(stream: *mut FILE, fmt: *const i8, ...) -> i32;
    }
    if data.is_null() {
        return;
    }
    // SAFETY: fp, name, data, float_format are valid pointers (caller contract)
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
pub fn printArray2d(str: *const i8, nr: i32, nc: i32, data: *const f64, fp: *mut FILE, float_format: *const i8) {
    extern "C" {
        fn fprintf(stream: *mut FILE, fmt: *const i8, ...) -> i32;
    }
    if data.is_null() {
        return;
    }
    // SAFETY: str, data, fp, float_format are valid (caller contract)
    unsafe {
        if nr != 0 && nc != 0 {
            fprintf(fp, b"%s\n\0".as_ptr() as *const i8, str);
            for r in 0..nr as usize {
                fprintf(fp, b" \0".as_ptr() as *const i8);
                for c in 0..nc as usize {
                    fprintf(fp, b" \0".as_ptr() as *const i8);
                    fprintf(fp, float_format, *data.add(c + r * nc as usize));
                }
                fprintf(fp, b"\n\0".as_ptr() as *const i8);
            }
            fprintf(fp, b"\n\0".as_ptr() as *const i8);
        }
    }
}

/// C: printArray2dInt (engine/engine_print.c:105)
#[allow(unused_variables, non_snake_case)]
pub fn printArray2dInt(str: *const i8, nr: i32, nc: i32, data: *const i32, fp: *mut FILE) {
    extern "C" {
        fn fprintf(stream: *mut FILE, fmt: *const i8, ...) -> i32;
    }
    if data.is_null() {
        return;
    }
    // SAFETY: str, data, fp are valid (caller contract)
    unsafe {
        if nr != 0 && nc != 0 {
            fprintf(fp, b"%s\n\0".as_ptr() as *const i8, str);
            for r in 0..nr as usize {
                fprintf(fp, b" \0".as_ptr() as *const i8);
                for c in 0..nc as usize {
                    fprintf(fp, b" \0".as_ptr() as *const i8);
                    fprintf(fp, b"%d\0".as_ptr() as *const i8, *data.add(c + r * nc as usize));
                }
                fprintf(fp, b"\n\0".as_ptr() as *const i8);
            }
            fprintf(fp, b"\n\0".as_ptr() as *const i8);
        }
    }
}

/// C: printDelayBuffer (engine/engine_print.c:125)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn printDelayBuffer(name: *const i8, buf: *const f64, nhistory: i32, dim: i32, fp: *mut FILE, float_format: *const i8) {
    extern "C" {
        fn fprintf(stream: *mut FILE, fmt: *const i8, ...) -> i32;
    }
    if buf.is_null() || nhistory <= 0 {
        return;
    }
    // SAFETY: buf, fp, float_format, name are valid pointers (caller contract)
    unsafe {
        fprintf(fp, b"  %s:\n\0".as_ptr() as *const i8, name);

        // user value (first slot)
        fprintf(fp, b"    phase  = \0".as_ptr() as *const i8);
        fprintf(fp, float_format, *buf.add(0));
        fprintf(fp, b"\n\0".as_ptr() as *const i8);

        // cursor (second slot, stored as mjtNum but is an integer)
        fprintf(fp, b"    cursor =  %d\n\0".as_ptr() as *const i8, *buf.add(1) as i32);

        // timestamps
        let times = buf.add(2);
        fprintf(fp, b"    times  = \0".as_ptr() as *const i8);
        for i in 0..nhistory as usize {
            fprintf(fp, float_format, *times.add(i));
        }
        fprintf(fp, b"\n\0".as_ptr() as *const i8);

        // values
        let values = times.add(nhistory as usize);
        if dim == 1 {
            fprintf(fp, b"    values = \0".as_ptr() as *const i8);
            for i in 0..nhistory as usize {
                fprintf(fp, float_format, *values.add(i));
            }
            fprintf(fp, b"\n\0".as_ptr() as *const i8);
        } else {
            fprintf(fp, b"    values:\n\0".as_ptr() as *const i8);
            for i in 0..nhistory as usize {
                fprintf(fp, b"      [%d] =\0".as_ptr() as *const i8, i as i32);
                for j in 0..dim as usize {
                    fprintf(fp, float_format, *values.add(i * dim as usize + j));
                }
                fprintf(fp, b"\n\0".as_ptr() as *const i8);
            }
        }
    }
}

/// C: printSparse (engine/engine_print.c:170)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn printSparse(str: *const i8, mat: *const f64, nr: i32, rownnz: *const i32, rowadr: *const i32, colind: *const i32, fp: *mut FILE, float_format: *const i8) {
    extern "C" {
        fn fprintf(stream: *mut FILE, fmt: *const i8, ...) -> i32;
    }
    // if no data, or too many rows to be visually useful, return
    if mat.is_null() || nr == 0 || nr > 300 {
        return;
    }
    // SAFETY: all pointers valid per caller contract
    unsafe {
        fprintf(fp, b"%s\n\0".as_ptr() as *const i8, str);

        for r in 0..nr as usize {
            fprintf(fp, b"  \0".as_ptr() as *const i8);
            let row_adr = *rowadr.add(r);
            let row_nnz = *rownnz.add(r);
            for adr in row_adr..(row_adr + row_nnz) {
                fprintf(fp, b"  \0".as_ptr() as *const i8);
                fprintf(fp, b"%2d: \0".as_ptr() as *const i8, *colind.add(adr as usize));
                fprintf(fp, float_format, *mat.add(adr as usize));
            }
            fprintf(fp, b"\n\0".as_ptr() as *const i8);
        }
        fprintf(fp, b"\n\0".as_ptr() as *const i8);
    }
}

/// C: printBlockArray (engine/engine_print.c:193)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn printBlockArray(str: *const i8, data: *const f64, nc: i32, nisland: i32, island_nr: *const i32, island_nc: *const i32, island_r: *const i32, island_c: *const i32, map_r: *const i32, map_c: *const i32, fp: *mut FILE, float_format: *const i8) {
    extern "C" {
        fn fprintf(stream: *mut FILE, fmt: *const i8, ...) -> i32;
        fn snprintf(s: *mut i8, n: usize, fmt: *const i8, ...) -> i32;
    }
    if data.is_null() || nisland == 0 {
        return;
    }
    // SAFETY: all pointers valid per caller contract
    unsafe {
        fprintf(fp, b"%s\n\0".as_ptr() as *const i8, str);

        // determine the width of the float format
        let mut dummy_buffer = [0u8; 100];
        let format_width = snprintf(
            dummy_buffer.as_mut_ptr() as *mut i8,
            100,
            float_format,
            0.0f64,
        );

        for b in 0..nisland as usize {
            let bnr = *island_nr.add(b);
            let bnc = *island_nc.add(b);
            let r_start = *island_r.add(b);
            let c_start = *island_c.add(b);

            // print rows for this block
            for r_block in 0..bnr as usize {
                fprintf(fp, b" \0".as_ptr() as *const i8);
                // leading dots
                for _c in 0..c_start as usize {
                    for _i in 0..format_width {
                        fprintf(fp, b".\0".as_ptr() as *const i8);
                    }
                    fprintf(fp, b" \0".as_ptr() as *const i8);
                }

                let row = *map_r.add(r_start as usize + r_block);

                // block data
                for c in 0..bnc as usize {
                    let col = *map_c.add(c_start as usize + c);
                    fprintf(fp, b" \0".as_ptr() as *const i8);
                    fprintf(fp, float_format, *data.add(row as usize * nc as usize + col as usize));
                }

                // trailing dots
                for _c in (c_start + bnc) as usize..nc as usize {
                    for _i in 0..format_width {
                        fprintf(fp, b".\0".as_ptr() as *const i8);
                    }
                    fprintf(fp, b" \0".as_ptr() as *const i8);
                }
                fprintf(fp, b"\n\0".as_ptr() as *const i8);
            }
        }

        fprintf(fp, b"\n\0".as_ptr() as *const i8);
    }
}

/// C: mj_printBlockSparsity (engine/engine_print.c:319)
#[allow(unused_variables, non_snake_case)]
pub fn mj_printBlockSparsity(str: *const i8, nr: i32, nc: i32, nisland: i32, island_block_ncols: *const i32, island_col_offset: *const i32, entity_island: *const i32, map_row_to_entity: *const i32, map_col_to_entity: *const i32, rownnz: *const i32, rowadr: *const i32, colind: *const i32, rowsuper: *const i32, fp: *mut FILE) {
    extern "C" {
        fn fprintf(stream: *mut FILE, fmt: *const i8, ...) -> i32;
    }
    // if no rows / columns, or too many columns to be visually useful, return
    if nr == 0 || nc == 0 || nc > 300 {
        return;
    }
    // SAFETY: all pointers valid per caller contract
    unsafe {
        fprintf(fp, b"%s\n\0".as_ptr() as *const i8, str);

        for _c in 0..(nc + 2) {
            fprintf(fp, b"-\0".as_ptr() as *const i8);
        }
        fprintf(fp, b"\n\0".as_ptr() as *const i8);

        for r in 0..nr as usize {
            fprintf(fp, b" \0".as_ptr() as *const i8);
            let entity_r = *map_row_to_entity.add(r);
            let island = *entity_island.add(entity_r as usize);

            // SHOULD NOT OCCUR
            if island < 0 || island >= nisland {
                for _c in 0..nc {
                    fprintf(fp, b" \0".as_ptr() as *const i8);
                }
                fprintf(
                    fp,
                    b" | Error: invalid island %d for row %d (entity %d)\n\0".as_ptr() as *const i8,
                    island, r as i32, entity_r,
                );
                continue;
            }

            let c_start = *island_col_offset.add(island as usize);
            let bnc = *island_block_ncols.add(island as usize);
            let adr = *rowadr.add(entity_r as usize);
            let nnz = *rownnz.add(entity_r as usize);
            let nz_char: i32 = if island < 10 {
                b'0' as i32 + island
            } else {
                b'x' as i32
            };

            for c in 0..nc {
                let mut nonzero = false;
                if c >= c_start && c < c_start + bnc {
                    let target_col = *map_col_to_entity.add(c as usize);
                    for i in 0..nnz {
                        if *colind.add((adr + i) as usize) == target_col {
                            nonzero = true;
                            break;
                        }
                    }
                }
                fprintf(
                    fp,
                    b"%c\0".as_ptr() as *const i8,
                    if nonzero { nz_char } else { b' ' as i32 },
                );
            }
            fprintf(fp, b" |\0".as_ptr() as *const i8);
            if !rowsuper.is_null() && *rowsuper.add(entity_r as usize) > 0 {
                fprintf(fp, b" %d\0".as_ptr() as *const i8, *rowsuper.add(entity_r as usize));
            }
            fprintf(fp, b"\n\0".as_ptr() as *const i8);
        }
        for _c in 0..(nc + 2) {
            fprintf(fp, b"-\0".as_ptr() as *const i8);
        }
        fprintf(fp, b"\n\n\0".as_ptr() as *const i8);
    }
}

/// C: printVector (engine/engine_print.c:377)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn printVector(str: *const i8, data: *const f64, n: i32, fp: *mut FILE, float_format: *const i8) {
    extern "C" {
        fn fprintf(stream: *mut FILE, fmt: *const i8, ...) -> i32;
    }
    if data.is_null() || n == 0 {
        return;
    }
    // SAFETY: str, data, fp, float_format are valid (caller contract)
    unsafe {
        fprintf(fp, b"%s\0".as_ptr() as *const i8, str);

        for i in 0..n as usize {
            fprintf(fp, b" \0".as_ptr() as *const i8);
            fprintf(fp, float_format, *data.add(i));
        }
        fprintf(fp, b"\n\0".as_ptr() as *const i8);
    }
}

/// C: memorySize (engine/engine_print.c:395)
#[allow(unused_variables, non_snake_case)]
pub fn memorySize(nbytes: usize) -> *const i8 {
    // SAFETY: thread-local buffer matching C's `static mjTHREADLOCAL char message[32]`
    unsafe {
        thread_local! {
            static MESSAGE: std::cell::UnsafeCell<[u8; 32]> = std::cell::UnsafeCell::new([0u8; 32]);
        }
        MESSAGE.with(|msg| {
            let buf = &mut *msg.get();
            let k: usize = 1024;
            if nbytes < k {
                extern "C" { fn snprintf(s: *mut i8, n: usize, fmt: *const i8, ...) -> i32; }
                snprintf(
                    buf.as_mut_ptr() as *mut i8, 32,
                    b"%5zu bytes\0".as_ptr() as *const i8,
                    nbytes,
                );
            } else {
                extern "C" { fn snprintf(s: *mut i8, n: usize, fmt: *const i8, ...) -> i32; }
                snprintf(
                    buf.as_mut_ptr() as *mut i8, 32,
                    b"%7.0f KB\0".as_ptr() as *const i8,
                    nbytes as f64 / k as f64,
                );
            }
            buf.as_ptr() as *const i8
        })
    }
}

/// C: sizeMesh (engine/engine_print.c:410)
#[allow(unused_variables, non_snake_case)]
pub fn sizeMesh(m: *const mjModel) -> usize {
    // SAFETY: m is a valid mjModel pointer (caller contract)
    unsafe {
        let mut nbytes: usize = 0;
        nbytes += std::mem::size_of::<f32>() * 3 * (*m).nmeshvert as usize;      // mesh_vert
        nbytes += std::mem::size_of::<f32>() * 3 * (*m).nmeshnormal as usize;    // mesh_normal
        nbytes += std::mem::size_of::<f32>() * 2 * (*m).nmeshtexcoord as usize;  // mesh_texcoord
        nbytes += std::mem::size_of::<i32>() * 3 * (*m).nmeshface as usize;      // mesh_face
        nbytes += std::mem::size_of::<i32>() * 3 * (*m).nmeshface as usize;      // mesh_facenormal
        nbytes += std::mem::size_of::<i32>() * 3 * (*m).nmeshface as usize;      // mesh_facetexcoord
        nbytes += std::mem::size_of::<i32>() * (*m).nmeshgraph as usize;         // mesh_graph
        nbytes += std::mem::size_of::<f64>() * 3 * (*m).nmeshpoly as usize;      // mesh_polynormal
        nbytes += std::mem::size_of::<i32>() * (*m).nmeshpoly as usize;          // mesh_polyvertadr
        nbytes += std::mem::size_of::<i32>() * (*m).nmeshpoly as usize;          // mesh_polyvertnum
        nbytes += std::mem::size_of::<i32>() * (*m).nmeshpolyvert as usize;      // mesh_polyvert
        nbytes += std::mem::size_of::<i32>() * (*m).nmeshvert as usize;          // mesh_polymapadr
        nbytes += std::mem::size_of::<i32>() * (*m).nmeshvert as usize;          // mesh_polymapnum
        nbytes += std::mem::size_of::<i32>() * (*m).nmeshpolymap as usize;       // mesh_polymap
        nbytes
    }
}

/// C: sizeSkin (engine/engine_print.c:431)
#[allow(unused_variables, non_snake_case)]
pub fn sizeSkin(m: *const mjModel) -> usize {
    // SAFETY: m is a valid mjModel pointer (caller contract)
    unsafe {
        let mut nbytes: usize = 0;
        nbytes += std::mem::size_of::<f32>() * 3 * (*m).nskinvert as usize;       // skin_vert
        nbytes += std::mem::size_of::<f32>() * 2 * (*m).nskintexvert as usize;    // skin_texcoord
        nbytes += std::mem::size_of::<i32>() * 3 * (*m).nskinface as usize;       // skin_face
        nbytes += std::mem::size_of::<i32>() * (*m).nskinbone as usize;           // skin_bonevertadr
        nbytes += std::mem::size_of::<i32>() * (*m).nskinbone as usize;           // skin_bonevertnum
        nbytes += std::mem::size_of::<f32>() * 3 * (*m).nskinbone as usize;       // skin_bonebindpos
        nbytes += std::mem::size_of::<f32>() * 4 * (*m).nskinbone as usize;       // skin_bonebindquat
        nbytes += std::mem::size_of::<i32>() * (*m).nskinbone as usize;           // skin_bonebodyid
        nbytes += std::mem::size_of::<i32>() * (*m).nskinbonevert as usize;       // skin_bonevertid
        nbytes += std::mem::size_of::<f32>() * (*m).nskinbonevert as usize;       // skin_bonevertweight
        nbytes
    }
}

/// C: sizeBVH (engine/engine_print.c:448)
#[allow(unused_variables, non_snake_case)]
pub fn sizeBVH(m: *const mjModel) -> usize {
    // SAFETY: m is a valid mjModel pointer (caller contract)
    unsafe {
        let mut nbytes: usize = 0;
        nbytes += std::mem::size_of::<i32>() * (*m).nbvh as usize;               // bvh_depth
        nbytes += std::mem::size_of::<i32>() * 2 * (*m).nbvh as usize;           // bvh_child
        nbytes += std::mem::size_of::<i32>() * (*m).nbvh as usize;               // bvh_nodeid
        nbytes += std::mem::size_of::<f64>() * 6 * (*m).nbvhstatic as usize;     // bvh_aabb
        nbytes += std::mem::size_of::<i32>() * (*m).noct as usize;               // oct_depth
        nbytes += std::mem::size_of::<i32>() * 8 * (*m).noct as usize;           // oct_child
        nbytes += std::mem::size_of::<f64>() * 6 * (*m).noct as usize;           // oct_aabb
        nbytes += std::mem::size_of::<f64>() * 8 * (*m).noct as usize;           // oct_coeff
        nbytes
    }
}

/// C: validateFloatFormat (engine/engine_print.c:463)
/// Calls: cxx:_mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn validateFloatFormat(float_format: *const i8) -> bool {
    extern "C" {
        fn strnlen(s: *const i8, maxlen: usize) -> usize;
        fn strchr(s: *const i8, c: i32) -> *const i8;
        fn mju_warning(msg: *const i8, ...);
    }
    const FLOAT_FORMAT_MAX_LEN: usize = 20;

    // check for nullptr
    if float_format.is_null() {
        return false;
    }

    // SAFETY: float_format is a valid C string pointer (caller contract)
    unsafe {
        if strnlen(float_format, FLOAT_FORMAT_MAX_LEN + 1) > FLOAT_FORMAT_MAX_LEN {
            mju_warning(b"Format string longer than limit of %d.\0".as_ptr() as *const i8, FLOAT_FORMAT_MAX_LEN as i32);
            return false;
        }

        let mut cur_idx: usize = 0;
        if *float_format.add(cur_idx) != b'%' as i8 {
            mju_warning(b"Format string must start with '%%'.\0".as_ptr() as *const i8);
            return false;
        }
        cur_idx += 1;

        // flag characters. allow at most one of each flag
        let flag_characters = b"-+ #0\0";
        let mut flag_character_counts = [0i32; 6];
        loop {
            let c = strchr(flag_characters.as_ptr() as *const i8, *float_format.add(cur_idx) as i32);
            if c.is_null() {
                break;
            }
            let flag_idx = (c as usize - flag_characters.as_ptr() as usize) / std::mem::size_of::<i8>();
            flag_character_counts[flag_idx] += 1;
            if flag_character_counts[flag_idx] > 1 {
                mju_warning(b"Format string contains repeated flag.\0".as_ptr() as *const i8);
                return false;
            }
            cur_idx += 1;
        }

        // width. disallow *, which requires additional argument
        loop {
            let c = strchr(b"0123456789\0".as_ptr() as *const i8, *float_format.add(cur_idx) as i32);
            if c.is_null() {
                break;
            }
            cur_idx += 1;
        }

        // precision. disallow *, which requires additional argument
        if *float_format.add(cur_idx) == b'.' as i8 {
            cur_idx += 1;
            loop {
                let c = strchr(b"0123456789\0".as_ptr() as *const i8, *float_format.add(cur_idx) as i32);
                if c.is_null() {
                    break;
                }
                cur_idx += 1;
            }
        }

        // length
        if *float_format.add(cur_idx) == b'L' as i8 {
            cur_idx += 1;
        }

        // specifier must be a valid float format
        if strchr(b"fgGeE\0".as_ptr() as *const i8, *float_format.add(cur_idx) as i32).is_null() {
            mju_warning(b"Format string specifier must be one of \"fgGeE\".\0".as_ptr() as *const i8);
            return false;
        }
        cur_idx += 1;

        if *float_format.add(cur_idx) == 0 {
            true
        } else {
            mju_warning(
                b"Unable to match format string %s with expected pattern for a single float.\0".as_ptr() as *const i8,
                float_format,
            );
            false
        }
    }
}

/// C: mj_printSparsity (engine/engine_print.h:47)
#[allow(unused_variables, non_snake_case)]
pub fn mj_printSparsity(str: *const i8, nr: i32, nc: i32, rowadr: *const i32, diag: *const i32, rownnz: *const i32, rowsuper: *const i32, colind: *const i32, fp: *mut FILE) {
    extern "C" {
        fn fprintf(stream: *mut FILE, fmt: *const i8, ...) -> i32;
    }
    // if no rows / columns, or too many columns to be visually useful, return
    if nr == 0 || nc == 0 || nc > 300 {
        return;
    }
    // SAFETY: all pointers valid per caller contract
    unsafe {
        fprintf(fp, b"%s\n\0".as_ptr() as *const i8, str);

        for _c in 0..(nc + 2) {
            fprintf(fp, b"-\0".as_ptr() as *const i8);
        }
        fprintf(fp, b"\n \0".as_ptr() as *const i8);

        for r in 0..nr as usize {
            let adr = *rowadr.add(r);
            let mut nnz = 0i32;
            for c in 0..nc {
                if nnz < *rownnz.add(r) && *colind.add((adr + nnz) as usize) == c {
                    if !diag.is_null() && *diag.add(r) == nnz {
                        fprintf(fp, b"D\0".as_ptr() as *const i8);
                    } else {
                        fprintf(fp, b"x\0".as_ptr() as *const i8);
                    }
                    nnz += 1;
                } else {
                    fprintf(fp, b" \0".as_ptr() as *const i8);
                }
            }
            fprintf(fp, b" |\0".as_ptr() as *const i8);
            if !rowsuper.is_null() && *rowsuper.add(r) > 0 {
                fprintf(fp, b" %d\0".as_ptr() as *const i8, *rowsuper.add(r));
            }
            fprintf(fp, b"\n\0".as_ptr() as *const i8);
            if (r as i32) < nr - 1 {
                fprintf(fp, b" \0".as_ptr() as *const i8);
            }
        }
        for _c in 0..(nc + 2) {
            fprintf(fp, b"-\0".as_ptr() as *const i8);
        }
        fprintf(fp, b"\n\n\0".as_ptr() as *const i8);
    }
}

/// C: mj_printScene (engine/engine_print.h:51)
/// Calls: cxx:_mj_printFormattedScene
#[allow(unused_variables, non_snake_case)]
pub fn mj_printScene(s: *const mjvScene, filename: *const i8) {
    mj_printFormattedScene(s, filename, b"% -9.2g\0".as_ptr() as *const i8);
}

/// C: mj_printFormattedScene (engine/engine_print.h:55)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_print.c:_printArr, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_print.c:_printInt, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_print.c:_printNum, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_print.c:_printStr, cxx-internal:engine_print.c.o:_validateFloatFormat, cxx:_mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn mj_printFormattedScene(s: *const mjvScene, filename: *const i8, float_format: *const i8) {
    extern "C" {
        fn fopen(path: *const i8, mode: *const i8) -> *mut FILE;
        fn fclose(stream: *mut FILE) -> i32;
        fn fprintf(stream: *mut FILE, fmt: *const i8, ...) -> i32;
        fn fflush(stream: *mut FILE) -> i32;
    }

    // SAFETY: s, filename, float_format are valid pointers from caller.
    unsafe {
        // get file
        let fp: *mut FILE;
        if !filename.is_null() {
            fp = fopen(filename, b"wt\0".as_ptr() as *const i8);
        } else {
            extern "C" { static stdout: *mut FILE; }
            fp = stdout;
        }

        // check for null
        if fp.is_null() {
            crate::engine::engine_util_errmem::mju_warning(
                b"Could not open file for writing mjScene\0".as_ptr() as *const i8,
            );
            return;
        }

        // validate format string
        let float_format = if !validateFloatFormat(float_format) {
            crate::engine::engine_util_errmem::mju_warning(
                b"WARNING: Received invalid float_format. Using default instead.\0".as_ptr() as *const i8,
            );
            b"% -9.2g\0".as_ptr() as *const i8
        } else {
            float_format
        };

        fprintf(fp, b"GEOMS %d\n\0".as_ptr() as *const i8, (*s).ngeom);
        for i in 0..(*s).ngeom {
            let geom = &*(*s).geoms.add(i as usize);
            fprintf(fp, b"  GEOM %d\n\0".as_ptr() as *const i8, i);
            printInt(fp, b"    type\0".as_ptr() as *const i8, geom.r#type);
            printInt(fp, b"    category\0".as_ptr() as *const i8, geom.category);
            printStr(fp, b"    label\0".as_ptr() as *const i8, geom.label.as_ptr());
            printInt(fp, b"    objtype\0".as_ptr() as *const i8, geom.objtype);
            printInt(fp, b"    objid\0".as_ptr() as *const i8, geom.objid);
            printArr(fp, b"    pos\0".as_ptr() as *const i8, geom.pos.as_ptr(), 3, float_format);
            printArr(fp, b"    mat\0".as_ptr() as *const i8, geom.mat.as_ptr(), 9, float_format);
            printArr(fp, b"    size\0".as_ptr() as *const i8, geom.size.as_ptr(), 3, float_format);
            printInt(fp, b"    segid\0".as_ptr() as *const i8, geom.segid);
            printInt(fp, b"    dataid\0".as_ptr() as *const i8, geom.dataid);
            printInt(fp, b"    matid\0".as_ptr() as *const i8, geom.matid);
            printInt(fp, b"    texcoord\0".as_ptr() as *const i8, geom.texcoord);
            printArr(fp, b"    rgba\0".as_ptr() as *const i8, geom.rgba.as_ptr(), 4, float_format);
            printNum(fp, b"    emission\0".as_ptr() as *const i8, geom.emission, float_format);
            printNum(fp, b"    specular\0".as_ptr() as *const i8, geom.specular, float_format);
            printNum(fp, b"    shininess\0".as_ptr() as *const i8, geom.shininess, float_format);
            printNum(fp, b"    reflectance\0".as_ptr() as *const i8, geom.reflectance, float_format);
            fprintf(fp, b"\n\0".as_ptr() as *const i8);
        }
        fprintf(fp, b"\n\0".as_ptr() as *const i8);

        fprintf(fp, b"LIGHTS %d\n\0".as_ptr() as *const i8, (*s).nlight);
        for i in 0..(*s).nlight {
            let light = &(*s).lights[i as usize];
            fprintf(fp, b"  LIGHT %d\n\0".as_ptr() as *const i8, i);
            printInt(fp, b"    id\0".as_ptr() as *const i8, light.id);
            printArr(fp, b"    pos\0".as_ptr() as *const i8, light.pos.as_ptr(), 3, float_format);
            printArr(fp, b"    dir\0".as_ptr() as *const i8, light.dir.as_ptr(), 3, float_format);
            printInt(fp, b"    type\0".as_ptr() as *const i8, light.r#type);
            printInt(fp, b"    castshadow\0".as_ptr() as *const i8, light.castshadow as i32);
            printInt(fp, b"    headlight\0".as_ptr() as *const i8, light.headlight as i32);
            printNum(fp, b"    intensity\0".as_ptr() as *const i8, light.intensity, float_format);
            printNum(fp, b"    range\0".as_ptr() as *const i8, light.range, float_format);
            printArr(fp, b"    ambient\0".as_ptr() as *const i8, light.ambient.as_ptr(), 3, float_format);
            printArr(fp, b"    diffuse\0".as_ptr() as *const i8, light.diffuse.as_ptr(), 3, float_format);
            printArr(fp, b"    specular\0".as_ptr() as *const i8, light.specular.as_ptr(), 3, float_format);
            printInt(fp, b"    texid\0".as_ptr() as *const i8, light.texid);
            printNum(fp, b"    exponent\0".as_ptr() as *const i8, light.exponent, float_format);
            printArr(fp, b"    attenuation\0".as_ptr() as *const i8, light.attenuation.as_ptr(), 3, float_format);
            printNum(fp, b"    cutoff\0".as_ptr() as *const i8, light.cutoff, float_format);
            printNum(fp, b"    bulbradius\0".as_ptr() as *const i8, light.bulbradius, float_format);
            fprintf(fp, b"\n\0".as_ptr() as *const i8);
        }
        fprintf(fp, b"\n\0".as_ptr() as *const i8);

        fprintf(fp, b"CAMERAS %d\n\0".as_ptr() as *const i8, 2);
        for i in 0..2i32 {
            let camera = &(*s).camera[i as usize];
            fprintf(fp, b"  CAMERA %d\n\0".as_ptr() as *const i8, i);
            printArr(fp, b"    pos\0".as_ptr() as *const i8, camera.pos.as_ptr(), 3, float_format);
            printArr(fp, b"    forward\0".as_ptr() as *const i8, camera.forward.as_ptr(), 3, float_format);
            printArr(fp, b"    up\0".as_ptr() as *const i8, camera.up.as_ptr(), 3, float_format);
            printInt(fp, b"    orthographic\0".as_ptr() as *const i8, camera.orthographic);
            printNum(fp, b"    frustum_center\0".as_ptr() as *const i8, camera.frustum_center, float_format);
            printNum(fp, b"    frustum_width\0".as_ptr() as *const i8, camera.frustum_width, float_format);
            printNum(fp, b"    frustum_bottom\0".as_ptr() as *const i8, camera.frustum_bottom, float_format);
            printNum(fp, b"    frustum_top\0".as_ptr() as *const i8, camera.frustum_top, float_format);
            printNum(fp, b"    frustum_near\0".as_ptr() as *const i8, camera.frustum_near, float_format);
            printNum(fp, b"    frustum_far\0".as_ptr() as *const i8, camera.frustum_far, float_format);
            fprintf(fp, b"\n\0".as_ptr() as *const i8);
        }
        fprintf(fp, b"\n\0".as_ptr() as *const i8);

        fprintf(fp, b"FLEX DATA %d\n\0".as_ptr() as *const i8, (*s).nflex);
        for i in 0..(*s).nflex {
            fprintf(fp, b"  FLEX DATA %d\n\0".as_ptr() as *const i8, i);
            printInt(fp, b"    face_used\0".as_ptr() as *const i8, *(*s).flexfaceused.add(i as usize));
            printInt(fp, b"    edge_adr\0".as_ptr() as *const i8, *(*s).flexedgeadr.add(i as usize));
            printInt(fp, b"    edge_num\0".as_ptr() as *const i8, *(*s).flexedgenum.add(i as usize));
            printInt(fp, b"    vert_adr\0".as_ptr() as *const i8, *(*s).flexvertadr.add(i as usize));
            printInt(fp, b"    vert_num\0".as_ptr() as *const i8, *(*s).flexvertnum.add(i as usize));
            printInt(fp, b"    face_adr\0".as_ptr() as *const i8, *(*s).flexfaceadr.add(i as usize));
            printInt(fp, b"    face_num\0".as_ptr() as *const i8, *(*s).flexfacenum.add(i as usize));
            printStr(fp, b"    edges\0".as_ptr() as *const i8, b"...\0".as_ptr() as *const i8);
            printStr(fp, b"    verts\0".as_ptr() as *const i8, b"...\0".as_ptr() as *const i8);
            printStr(fp, b"    faces\0".as_ptr() as *const i8, b"...\0".as_ptr() as *const i8);
            printStr(fp, b"    normals\0".as_ptr() as *const i8, b"...\0".as_ptr() as *const i8);
            printStr(fp, b"    texcoords\0".as_ptr() as *const i8, b"...\0".as_ptr() as *const i8);
            fprintf(fp, b"\n\0".as_ptr() as *const i8);
        }
        fprintf(fp, b"\n\0".as_ptr() as *const i8);

        fprintf(fp, b"SKIN DATA %d\n\0".as_ptr() as *const i8, (*s).nskin);
        for i in 0..(*s).nskin {
            fprintf(fp, b"  SKIN DATA %d\n\0".as_ptr() as *const i8, i);
            printInt(fp, b"    face_num\0".as_ptr() as *const i8, *(*s).skinfacenum.add(i as usize));
            printInt(fp, b"    vert_adr\0".as_ptr() as *const i8, *(*s).skinvertadr.add(i as usize));
            printInt(fp, b"    vert_num\0".as_ptr() as *const i8, *(*s).skinvertnum.add(i as usize));
            printStr(fp, b"    verts\0".as_ptr() as *const i8, b"...\0".as_ptr() as *const i8);
            printStr(fp, b"    normals\0".as_ptr() as *const i8, b"...\0".as_ptr() as *const i8);
            fprintf(fp, b"\n\0".as_ptr() as *const i8);
        }
        fprintf(fp, b"\n\0".as_ptr() as *const i8);

        // FLAGS: mjNRNDFLAG=11, MJRNDSTRING is [u8; 264] = const char*[11][3]
        // Each entry is 3 pointers × 8 bytes. MJRNDSTRING[i][0] is at offset i*3*8.
        fprintf(fp, b"FLAGS\n\0".as_ptr() as *const i8);
        {
            let rndstring_guard = crate::types::MJRNDSTRING.lock().unwrap();
            let rndstring_ptr = rndstring_guard.as_ptr() as *const *const i8;
            for i in 0..mjNRNDFLAG as i32 {
                // MJRNDSTRING[i][0] is the name pointer
                let name_ptr = *rndstring_ptr.add((i * 3) as usize);
                fprintf(fp, b"  \0".as_ptr() as *const i8);
                fprintf(fp, b"%-21s\0".as_ptr() as *const i8, name_ptr);
                fprintf(fp, b" %d\0".as_ptr() as *const i8, (*s).flags[i as usize] as i32);
                fprintf(fp, b"\n\0".as_ptr() as *const i8);
            }
        }
        printInt(fp, b"  flexvertopt\0".as_ptr() as *const i8, (*s).flexvertopt as i32);
        printInt(fp, b"  flexedgeopt\0".as_ptr() as *const i8, (*s).flexedgeopt as i32);
        printInt(fp, b"  flexfaceopt\0".as_ptr() as *const i8, (*s).flexfaceopt as i32);
        printInt(fp, b"  flexskinopt\0".as_ptr() as *const i8, (*s).flexskinopt as i32);
        printInt(fp, b"  stereo\0".as_ptr() as *const i8, (*s).stereo);
        fprintf(fp, b"\n\n\0".as_ptr() as *const i8);

        fprintf(fp, b"TRANSFORM %d\n\0".as_ptr() as *const i8, (*s).enabletransform as i32);
        if (*s).enabletransform != 0 {
            printArr(fp, b"  translate\0".as_ptr() as *const i8, (*s).translate.as_ptr(), 3, float_format);
            printArr(fp, b"  rotate\0".as_ptr() as *const i8, (*s).rotate.as_ptr(), 4, float_format);
            printNum(fp, b"  scale\0".as_ptr() as *const i8, (*s).scale, float_format);
            fprintf(fp, b"\n\0".as_ptr() as *const i8);
        }
        fprintf(fp, b"\n\0".as_ptr() as *const i8);

        fflush(fp);
        if !filename.is_null() {
            fclose(fp);
        }
    }
}

