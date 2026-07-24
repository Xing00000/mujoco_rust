//! Port of: engine/engine_print.c
//! IR hash: 3fb6da908ad9d71c
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: printInt (engine/engine_print.c:53)
#[allow(unused_variables, non_snake_case)]
pub fn print_int(fp: *mut FILE, name: *const i8, value: i32) {
    todo!() // printInt
}

/// C: printStr (engine/engine_print.c:59)
#[allow(unused_variables, non_snake_case)]
pub fn print_str(fp: *mut FILE, name: *const i8, value: *const i8) {
    todo!() // printStr
}

/// C: printNum (engine/engine_print.c:65)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn print_num(fp: *mut FILE, name: *const i8, value: f32, float_format: *const i8) {
    todo!() // printNum
}

/// C: printArr (engine/engine_print.c:71)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn print_arr(fp: *mut FILE, name: *const i8, data: *const f32, n: i32, float_format: *const i8) {
    todo!() // printArr
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
pub fn print_array2d_int(str: *const i8, nr: i32, nc: i32, data: *const i32, fp: *mut FILE) {
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
pub fn print_delay_buffer(name: *const i8, buf: *const f64, nhistory: i32, dim: i32, fp: *mut FILE, float_format: *const i8) {
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
pub fn print_sparse(str: *const i8, mat: *const f64, nr: i32, rownnz: *const i32, rowadr: *const i32, colind: *const i32, fp: *mut FILE, float_format: *const i8) {
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
pub fn print_block_array(str: *const i8, data: *const f64, nc: i32, nisland: i32, island_nr: *const i32, island_nc: *const i32, island_r: *const i32, island_c: *const i32, map_r: *const i32, map_c: *const i32, fp: *mut FILE, float_format: *const i8) {
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

/// C: printInertia (engine/engine_print.c:246)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn print_inertia(str: *const i8, mat: *const f64, m: *const mjModel, fp: *mut FILE, float_format: *const i8) {
    extern "C" {
        fn fprintf(stream: *mut FILE, fmt: *const i8, ...) -> i32;
        fn snprintf(s: *mut i8, n: usize, fmt: *const i8, ...) -> i32;
    }
    // SAFETY: m is a valid mjModel pointer (caller contract)
    unsafe {
        let nv = (*m).nv;
        // if no data, or too many rows to be visually useful, return
        if mat.is_null() || nv == 0 || nv > 300 {
            return;
        }

        // get length of string produced by float_format
        let mut test = [0u8; 100];
        let len = snprintf(test.as_mut_ptr() as *mut i8, 100, float_format, 0.0f64);

        fprintf(fp, b"%s\n\0".as_ptr() as *const i8, str);

        for i in 0..nv as usize {
            fprintf(fp, b" \0".as_ptr() as *const i8);
            let mut adr: i32 = if i == (nv as usize - 1) {
                ((*m).nM - 1) as i32
            } else {
                *(*m).dof_Madr.add(i + 1) - 1
            };
            for k in 0..=i {
                let mut j = i as i32;
                while j != k as i32 && j >= 0 {
                    j = *(*m).dof_parentid.add(j as usize);
                }
                if j == k as i32 {
                    fprintf(fp, b" \0".as_ptr() as *const i8);
                    fprintf(fp, float_format, *mat.add(adr as usize));
                    adr -= 1;
                } else {
                    for _d in 0..(len + 1) {
                        fprintf(fp, b" \0".as_ptr() as *const i8);
                    }
                }
            }
            fprintf(fp, b"\n\0".as_ptr() as *const i8);
        }
        fprintf(fp, b"\n\0".as_ptr() as *const i8);
    }
}

/// C: mj_printBlockSparsity (engine/engine_print.c:319)
#[allow(unused_variables, non_snake_case)]
pub fn mj_print_block_sparsity(str: *const i8, nr: i32, nc: i32, nisland: i32, island_block_ncols: *const i32, island_col_offset: *const i32, entity_island: *const i32, map_row_to_entity: *const i32, map_col_to_entity: *const i32, rownnz: *const i32, rowadr: *const i32, colind: *const i32, rowsuper: *const i32, fp: *mut FILE) {
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
pub fn print_vector(str: *const i8, data: *const f64, n: i32, fp: *mut FILE, float_format: *const i8) {
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
pub fn memory_size(nbytes: usize) -> *const i8 {
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
pub fn size_mesh(m: *const mjModel) -> usize {
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
pub fn size_skin(m: *const mjModel) -> usize {
    todo!() // sizeSkin
}

/// C: sizeBVH (engine/engine_print.c:448)
#[allow(unused_variables, non_snake_case)]
pub fn size_bvh(m: *const mjModel) -> usize {
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
/// Calls: mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn validate_float_format(float_format: *const i8) -> bool {
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

