//! Port of: engine/engine_print.c
//! IR hash: 73393814548a07d1
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: printInt (engine/engine_print.c:53)
#[allow(unused_variables, non_snake_case)]
pub fn print_int(fp: *mut FILE, name: *const i8, value: i32) {
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
pub fn print_str(fp: *mut FILE, name: *const i8, value: *const i8) {
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
pub fn print_num(fp: *mut FILE, name: *const i8, value: f32, float_format: *const i8) {
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
pub fn print_arr(fp: *mut FILE, name: *const i8, data: *const f32, n: i32, float_format: *const i8) {
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
    extern "C" {
        fn fopen(path: *const i8, mode: *const i8) -> *mut FILE;
        fn fclose(stream: *mut FILE) -> i32;
        fn fprintf(stream: *mut FILE, fmt: *const i8, ...) -> i32;
        fn fflush(stream: *mut FILE) -> i32;
    }
    // SAFETY: m, d are valid pointers. filename may be null (use stdout).
    unsafe {
        // stack in use check
        if (*d).pstack != 0 {
            panic!("attempting to print mjData when stack is in use");
        }
        // validate format string
        let float_format = if !validate_float_format(float_format) {
            crate::engine::engine_util_errmem::mju_warning(
                b"WARNING: Received invalid float_format. Using default instead.\0".as_ptr() as *const i8,
            );
            b"% -9.2g\0".as_ptr() as *const i8
        } else {
            float_format
        };
        // get file
        let fp: *mut FILE = if !filename.is_null() {
            fopen(filename, b"wt\0".as_ptr() as *const i8)
        } else {
            extern "C" { static stdout: *mut FILE; }
            stdout
        };
        if fp.is_null() {
            crate::engine::engine_util_errmem::mju_warning(
                b"Could not open file for writing mjData\0".as_ptr() as *const i8,
            );
            return;
        }
        // MEMORY section
        let total = std::mem::size_of::<mjData>() as usize + (*d).nbuffer as usize + (*d).narena as usize;
        let struct_sz = std::mem::size_of::<mjData>();
        fprintf(fp, b"MEMORY\n\0".as_ptr() as *const i8);
        fprintf(fp, b"  total         %s\n\0".as_ptr() as *const i8, memory_size(total));
        fprintf(fp, b"  struct        %s\n\0".as_ptr() as *const i8, memory_size(struct_sz));
        fprintf(fp, b"  buffer        %s\n\0".as_ptr() as *const i8, memory_size((*d).nbuffer as usize));
        let arena_percent: f64 = if (*d).narena > 0 { 100.0 * (*d).maxuse_arena as f64 / (*d).narena as f64 } else { 0.0 };
        fprintf(fp, b"  arena         %s, used %.1f%%\n\n\0".as_ptr() as *const i8,
            memory_size((*d).narena as usize), arena_percent);
        // SIZES section — MJDATA_SCALAR expansion (skip pstack, pbase, parena, threadpool)
        fprintf(fp, b"SIZES\n\0".as_ptr() as *const i8);
        // size_t fields (SIZE_FORMAT = " %zu")
        macro_rules! prsz {
            ($name:expr, $val:expr) => {
                fprintf(fp, b"  \0".as_ptr() as *const i8);
                fprintf(fp, b"%-21s\0".as_ptr() as *const i8, $name.as_ptr() as *const i8);
                fprintf(fp, b" %zu\0".as_ptr() as *const i8, $val as usize);
                fprintf(fp, b"\n\0".as_ptr() as *const i8);
            };
        }
        macro_rules! print {
            ($name:expr, $val:expr) => {
                fprintf(fp, b"  \0".as_ptr() as *const i8);
                fprintf(fp, b"%-21s\0".as_ptr() as *const i8, $name.as_ptr() as *const i8);
                fprintf(fp, b" %d\0".as_ptr() as *const i8, $val as i32);
                fprintf(fp, b"\n\0".as_ptr() as *const i8);
            };
        }
        prsz!(b"narena\0", (*d).narena);
        prsz!(b"nbuffer\0", (*d).nbuffer);
        print!(b"nplugin\0", (*d).nplugin);
        prsz!(b"maxuse_stack\0", (*d).maxuse_stack);
        prsz!(b"maxuse_arena\0", (*d).maxuse_arena);
        print!(b"maxuse_con\0", (*d).maxuse_con);
        print!(b"maxuse_efc\0", (*d).maxuse_efc);
        print!(b"ncon\0", (*d).ncon);
        print!(b"ne\0", (*d).ne);
        print!(b"nf\0", (*d).nf);
        print!(b"nl\0", (*d).nl);
        print!(b"nefc\0", (*d).nefc);
        print!(b"nJ\0", (*d).nJ);
        print!(b"nY\0", (*d).nY);
        print!(b"nA\0", (*d).nA);
        print!(b"nisland\0", (*d).nisland);
        print!(b"nidof\0", (*d).nidof);
        print!(b"ntree_awake\0", (*d).ntree_awake);
        print!(b"nbody_awake\0", (*d).nbody_awake);
        print!(b"nparent_awake\0", (*d).nparent_awake);
        print!(b"nv_awake\0", (*d).nv_awake);
        print!(b"flg_energypos\0", (*d).flg_energypos as i32);
        print!(b"flg_energyvel\0", (*d).flg_energyvel as i32);
        print!(b"flg_subtreevel\0", (*d).flg_subtreevel as i32);
        print!(b"flg_rnepost\0", (*d).flg_rnepost as i32);
        // threadpool (special case: print as int 0 or 1)
        let threadpool_val: i32 = if (*d).threadpool != 0 { 1 } else { 0 };
        fprintf(fp, b"  \0".as_ptr() as *const i8);
        fprintf(fp, b"%-21s\0".as_ptr() as *const i8, b"threadpool\0".as_ptr() as *const i8);
        fprintf(fp, b" %d\0".as_ptr() as *const i8, threadpool_val);
        fprintf(fp, b"\n\0".as_ptr() as *const i8);
        fprintf(fp, b"\n\0".as_ptr() as *const i8);
        // WARNING section (mjWarningStat: lastinfo i32 + number i32 = 8 bytes each, 7 total)
        let warn_ptr = (*d).warning.as_ptr() as *const i32; // 2 i32 per stat
        let mut active_warnings = 0i32;
        for i in 0..7usize { active_warnings += *warn_ptr.add(i * 2 + 1); } // .number at offset 4
        if active_warnings != 0 {
            fprintf(fp, b"WARNING\n\0".as_ptr() as *const i8);
            for i in 0..7usize {
                let lastinfo = *warn_ptr.add(i * 2 + 0);
                let number   = *warn_ptr.add(i * 2 + 1);
                if number != 0 {
                    fprintf(fp, b"    %d:  lastinfo = %d   number = %d\n\0".as_ptr() as *const i8,
                        i as i32, lastinfo, number);
                }
            }
            fprintf(fp, b"\n\0".as_ptr() as *const i8);
        }
        // TIMER section (mjTimerStat: duration f64 + number i32 + pad i32 = 16 bytes each, 15 total)
        let timer_base = (*d).timer.as_ptr();
        let mut active_timers = 0.0f64;
        for i in 0..15usize {
            let dur = *(timer_base.add(i * 16) as *const f64);
            active_timers += dur;
        }
        if active_timers != 0.0 {
            fprintf(fp, b"TIMER\n\0".as_ptr() as *const i8);
            for i in 0..15usize {
                let dur = *(timer_base.add(i * 16) as *const f64);
                let num = *(timer_base.add(i * 16 + 8) as *const i32);
                fprintf(fp, b"    %d:  duration = \0".as_ptr() as *const i8, i as i32);
                fprintf(fp, float_format, dur);
                fprintf(fp, b"   number = %d\n\0".as_ptr() as *const i8, num);
            }
            fprintf(fp, b"\n\0".as_ptr() as *const i8);
        }
        // SOLVER STAT section
        // mjSolverStat: improvement(f64) gradient(f64) lineslope(f64) nactive(i32) nchange(i32) neval(i32) nupdate(i32) = 40 bytes
        if (*d).nefc != 0 {
            fprintf(fp, b"SOLVER STAT\n\0".as_ptr() as *const i8);
            print_vector(b"  solver_fwdinv = \0".as_ptr() as *const i8,
                (*d).solver_fwdinv.as_ptr(), 2, fp, float_format);
            let nisland_stat = if (*d).nisland > 0 {
                if (*d).nisland < 20 { (*d).nisland } else { 20 }
            } else { 1 };
            let solver_base = (*d).solver.as_ptr();
            for island in 0..nisland_stat as usize {
                let niter = (*d).solver_niter[island];
                let niter_stat = if niter < 200 { niter } else { 200 };
                if niter_stat != 0 {
                    fprintf(fp, b"  ISLAND %d\n\0".as_ptr() as *const i8, island as i32);
                    fprintf(fp, b"    solver_niter = %d\n\0".as_ptr() as *const i8, (*d).solver_niter[island]);
                    fprintf(fp, b"    solver_nnz = %d\n\0".as_ptr() as *const i8, (*d).solver_nnz[island]);
                    for i in 0..niter_stat as usize {
                        // offset: island * 200 * 40 + i * 40
                        let stat = solver_base.add(island * 200 * 40 + i * 40);
                        let improvement = *(stat as *const f64);
                        let gradient    = *(stat.add(8) as *const f64);
                        let lineslope   = *(stat.add(16) as *const f64);
                        let nactive  = *(stat.add(24) as *const i32);
                        let nchange  = *(stat.add(28) as *const i32);
                        let neval    = *(stat.add(32) as *const i32);
                        let nupdate  = *(stat.add(36) as *const i32);
                        fprintf(fp, b"      %d:  improvement = \0".as_ptr() as *const i8, i as i32);
                        fprintf(fp, float_format, improvement);
                        fprintf(fp, b"    gradient = \0".as_ptr() as *const i8);
                        fprintf(fp, float_format, gradient);
                        fprintf(fp, b"    lineslope = \0".as_ptr() as *const i8);
                        fprintf(fp, float_format, lineslope);
                        fprintf(fp, b"\n\0".as_ptr() as *const i8);
                        fprintf(fp, b"          nactive = %d   nchange = %d   neval = %d   nupdate = %d\n\0".as_ptr() as *const i8,
                            nactive, nchange, neval, nupdate);
                    }
                    fprintf(fp, b"\n\0".as_ptr() as *const i8);
                }
            }
        }
        // energy, time, basic arrays
        print_vector(b"ENERGY = \0".as_ptr() as *const i8, (*d).energy.as_ptr(), 2, fp, float_format);
        fprintf(fp, b"\n\0".as_ptr() as *const i8);
        fprintf(fp, b"TIME = \0".as_ptr() as *const i8);
        fprintf(fp, float_format, (*d).time);
        fprintf(fp, b"\n\n\0".as_ptr() as *const i8);
        // state arrays
        print_array2d(b"QPOS\0".as_ptr() as *const i8, (*m).nq as i32, 1, (*d).qpos, fp, float_format);
        print_array2d(b"QVEL\0".as_ptr() as *const i8, (*m).nv as i32, 1, (*d).qvel, fp, float_format);
        print_array2d(b"ACT\0".as_ptr() as *const i8, (*m).na as i32, 1, (*d).act, fp, float_format);
        // history (delay buffers)
        if (*m).nhistory as i32 != 0 {
            fprintf(fp, b"DELAY\n\0".as_ptr() as *const i8);
            for i in 0..(*m).nu as usize {
                let adr = *(*m).actuator_historyadr.add(i);
                if adr >= 0 {
                    print_delay_buffer(b"actuator\0".as_ptr() as *const i8,
                        (*d).history.add(adr as usize), *(*m).actuator_history.add(2*i), 1, fp, float_format);
                }
            }
            for i in 0..(*m).nsensor as usize {
                let adr = *(*m).sensor_historyadr.add(i);
                if adr >= 0 {
                    print_delay_buffer(b"sensor\0".as_ptr() as *const i8,
                        (*d).history.add(adr as usize), *(*m).sensor_history.add(2*i), *(*m).sensor_dim.add(i), fp, float_format);
                }
            }
            fprintf(fp, b"\n\0".as_ptr() as *const i8);
        }
        print_array2d(b"QACC_WARMSTART\0".as_ptr() as *const i8, (*m).nv as i32, 1, (*d).qacc_warmstart, fp, float_format);
        print_array2d(b"CTRL\0".as_ptr() as *const i8, (*m).nu as i32, 1, (*d).ctrl, fp, float_format);
        print_array2d(b"QFRC_APPLIED\0".as_ptr() as *const i8, (*m).nv as i32, 1, (*d).qfrc_applied, fp, float_format);
        print_array2d(b"XFRC_APPLIED\0".as_ptr() as *const i8, (*m).nbody as i32, 6, (*d).xfrc_applied, fp, float_format);
        if (*m).neq as i32 != 0 {
            fprintf(fp, b"%-21s\0".as_ptr() as *const i8, b"EQ_ACTIVE\0".as_ptr() as *const i8);
            for c in 0..(*m).neq as usize {
                fprintf(fp, b" %d\0".as_ptr() as *const i8, (*(*d).eq_active.add(c)) as i32);
            }
            fprintf(fp, b"\n\n\0".as_ptr() as *const i8);
        }
        print_array2d(b"MOCAP_POS\0".as_ptr() as *const i8, (*m).nmocap as i32, 3, (*d).mocap_pos, fp, float_format);
        print_array2d(b"MOCAP_QUAT\0".as_ptr() as *const i8, (*m).nmocap as i32, 4, (*d).mocap_quat, fp, float_format);
        print_array2d(b"QACC\0".as_ptr() as *const i8, (*m).nv as i32, 1, (*d).qacc, fp, float_format);
        print_array2d(b"ACT_DOT\0".as_ptr() as *const i8, (*m).na as i32, 1, (*d).act_dot, fp, float_format);
        print_array2d(b"USERDATA\0".as_ptr() as *const i8, (*m).nuserdata as i32, 1, (*d).userdata, fp, float_format);
        print_array2d(b"SENSOR\0".as_ptr() as *const i8, (*m).nsensordata as i32, 1, (*d).sensordata, fp, float_format);
        print_array2d_int(b"TREE_ASLEEP\0".as_ptr() as *const i8, (*m).ntree as i32, 1, (*d).tree_asleep, fp);
        print_array2d(b"XPOS\0".as_ptr() as *const i8, (*m).nbody as i32, 3, (*d).xpos, fp, float_format);
        print_array2d(b"XQUAT\0".as_ptr() as *const i8, (*m).nbody as i32, 4, (*d).xquat, fp, float_format);
        print_array2d(b"XMAT\0".as_ptr() as *const i8, (*m).nbody as i32, 9, (*d).xmat, fp, float_format);
        print_array2d(b"XIPOS\0".as_ptr() as *const i8, (*m).nbody as i32, 3, (*d).xipos, fp, float_format);
        print_array2d(b"XIMAT\0".as_ptr() as *const i8, (*m).nbody as i32, 9, (*d).ximat, fp, float_format);
        print_array2d(b"XANCHOR\0".as_ptr() as *const i8, (*m).njnt as i32, 3, (*d).xanchor, fp, float_format);
        print_array2d(b"XAXIS\0".as_ptr() as *const i8, (*m).njnt as i32, 3, (*d).xaxis, fp, float_format);
        print_array2d(b"GEOM_XPOS\0".as_ptr() as *const i8, (*m).ngeom as i32, 3, (*d).geom_xpos, fp, float_format);
        print_array2d(b"GEOM_XMAT\0".as_ptr() as *const i8, (*m).ngeom as i32, 9, (*d).geom_xmat, fp, float_format);
        print_array2d(b"SITE_XPOS\0".as_ptr() as *const i8, (*m).nsite as i32, 3, (*d).site_xpos, fp, float_format);
        print_array2d(b"SITE_XMAT\0".as_ptr() as *const i8, (*m).nsite as i32, 9, (*d).site_xmat, fp, float_format);
        print_array2d(b"CAM_XPOS\0".as_ptr() as *const i8, (*m).ncam as i32, 3, (*d).cam_xpos, fp, float_format);
        print_array2d(b"CAM_XMAT\0".as_ptr() as *const i8, (*m).ncam as i32, 9, (*d).cam_xmat, fp, float_format);
        print_array2d(b"LIGHT_XPOS\0".as_ptr() as *const i8, (*m).nlight as i32, 3, (*d).light_xpos, fp, float_format);
        print_array2d(b"LIGHT_XDIR\0".as_ptr() as *const i8, (*m).nlight as i32, 3, (*d).light_xdir, fp, float_format);
        print_array2d(b"SUBTREE_COM\0".as_ptr() as *const i8, (*m).nbody as i32, 3, (*d).subtree_com, fp, float_format);
        print_array2d(b"CDOF\0".as_ptr() as *const i8, (*m).nv as i32, 6, (*d).cdof, fp, float_format);
        print_array2d(b"CINERT\0".as_ptr() as *const i8, (*m).nbody as i32, 10, (*d).cinert, fp, float_format);
        print_array2d(b"FLEXVERT_XPOS\0".as_ptr() as *const i8, (*m).nflexvert as i32, 3, (*d).flexvert_xpos, fp, float_format);
        print_array2d(b"FLEXELEM_AABB\0".as_ptr() as *const i8, (*m).nflexelem as i32, 6, (*d).flexelem_aabb, fp, float_format);
        mj_print_sparsity(b"FLEXEDGE_J: flex edge connectivity\0".as_ptr() as *const i8,
            (*m).nflexedge as i32, (*m).nv as i32, (*m).flexedge_J_rowadr, std::ptr::null(), (*m).flexedge_J_rownnz,
            std::ptr::null(), (*m).flexedge_J_colind, fp);
        print_sparse(b"FLEXEDGE_J\0".as_ptr() as *const i8, (*d).flexedge_J, (*m).nflexedge as i32,
            (*m).flexedge_J_rownnz, (*m).flexedge_J_rowadr, (*m).flexedge_J_colind, fp, float_format);
        print_array2d(b"FLEXEDGE_LENGTH\0".as_ptr() as *const i8, (*m).nflexedge as i32, 1, (*d).flexedge_length, fp, float_format);
        print_array2d(b"TEN_LENGTH\0".as_ptr() as *const i8, (*m).ntendon as i32, 1, (*d).ten_length, fp, float_format);
        mj_print_sparsity(b"TEN_J: tendon moments\0".as_ptr() as *const i8,
            (*m).ntendon as i32, (*m).nv as i32, (*m).ten_J_rowadr, std::ptr::null(), (*m).ten_J_rownnz,
            std::ptr::null(), (*m).ten_J_colind, fp);
        print_array2d_int(b"TEN_J_ROWNNZ\0".as_ptr() as *const i8, (*m).ntendon as i32, 1, (*m).ten_J_rownnz, fp);
        print_array2d_int(b"TEN_J_ROWADR\0".as_ptr() as *const i8, (*m).ntendon as i32, 1, (*m).ten_J_rowadr, fp);
        print_sparse(b"TEN_J\0".as_ptr() as *const i8, (*d).ten_J, (*m).ntendon as i32,
            (*m).ten_J_rownnz, (*m).ten_J_rowadr, (*m).ten_J_colind, fp, float_format);
        for i in 0..(*m).ntendon as usize {
            fprintf(fp, b"TENDON %d: %d wrap points\n\0".as_ptr() as *const i8, i as i32, *(*d).ten_wrapnum.add(i));
            let wadr = *(*d).ten_wrapadr.add(i) as usize;
            for j in 0..*(*d).ten_wrapnum.add(i) as usize {
                fprintf(fp, b"    %d:  \0".as_ptr() as *const i8, *(*d).wrap_obj.add(wadr + j));
                print_vector(b"\0".as_ptr() as *const i8, (*d).wrap_xpos.add(3 * (wadr + j)), 3, fp, float_format);
            }
            fprintf(fp, b"\n\0".as_ptr() as *const i8);
        }
        print_array2d(b"ACTUATOR_LENGTH\0".as_ptr() as *const i8, (*m).nu as i32, 1, (*d).actuator_length, fp, float_format);
        mj_print_sparsity(b"actuator_moment\0".as_ptr() as *const i8,
            (*m).nu as i32, (*m).nv as i32, (*d).moment_rowadr, std::ptr::null(), (*d).moment_rownnz,
            std::ptr::null(), (*d).moment_colind, fp);
        print_sparse(b"ACTUATOR_MOMENT\0".as_ptr() as *const i8, (*d).actuator_moment, (*m).nu as i32,
            (*d).moment_rownnz, (*d).moment_rowadr, (*d).moment_colind, fp, float_format);
        print_array2d(b"CRB\0".as_ptr() as *const i8, (*m).nbody as i32, 10, (*d).crb, fp, float_format);
        print_inertia(b"QM\0".as_ptr() as *const i8, (*d).qM, m, fp, float_format);
        print_sparse(b"M\0".as_ptr() as *const i8, (*d).M, (*m).nv as i32,
            (*m).M_rownnz, (*m).M_rowadr, (*m).M_colind, fp, float_format);
        print_sparse(b"QLD\0".as_ptr() as *const i8, (*d).qLD, (*m).nv as i32,
            (*m).M_rownnz, (*m).M_rowadr, (*m).M_colind, fp, float_format);
        print_array2d(b"QLDIAGINV\0".as_ptr() as *const i8, (*m).nv as i32, 1, (*d).qLDiagInv, fp, float_format);
        // fflush and close
        fflush(fp);
        if !filename.is_null() {
            fclose(fp);
        }
    }
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
        let float_format = if !validate_float_format(float_format) {
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

        fprintf(fp, b"LIGHTS %d\n\0".as_ptr() as *const i8, (*s).nlight);
        for i in 0..(*s).nlight {
            let light = &(*s).lights[i as usize];
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

        fprintf(fp, b"CAMERAS %d\n\0".as_ptr() as *const i8, 2);
        for i in 0..2i32 {
            let camera = &(*s).camera[i as usize];
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

        fprintf(fp, b"FLEX DATA %d\n\0".as_ptr() as *const i8, (*s).nflex);
        for i in 0..(*s).nflex {
            fprintf(fp, b"  FLEX DATA %d\n\0".as_ptr() as *const i8, i);
            print_int(fp, b"    face_used\0".as_ptr() as *const i8, *(*s).flexfaceused.add(i as usize));
            print_int(fp, b"    edge_adr\0".as_ptr() as *const i8, *(*s).flexedgeadr.add(i as usize));
            print_int(fp, b"    edge_num\0".as_ptr() as *const i8, *(*s).flexedgenum.add(i as usize));
            print_int(fp, b"    vert_adr\0".as_ptr() as *const i8, *(*s).flexvertadr.add(i as usize));
            print_int(fp, b"    vert_num\0".as_ptr() as *const i8, *(*s).flexvertnum.add(i as usize));
            print_int(fp, b"    face_adr\0".as_ptr() as *const i8, *(*s).flexfaceadr.add(i as usize));
            print_int(fp, b"    face_num\0".as_ptr() as *const i8, *(*s).flexfacenum.add(i as usize));
            print_str(fp, b"    edges\0".as_ptr() as *const i8, b"...\0".as_ptr() as *const i8);
            print_str(fp, b"    verts\0".as_ptr() as *const i8, b"...\0".as_ptr() as *const i8);
            print_str(fp, b"    faces\0".as_ptr() as *const i8, b"...\0".as_ptr() as *const i8);
            print_str(fp, b"    normals\0".as_ptr() as *const i8, b"...\0".as_ptr() as *const i8);
            print_str(fp, b"    texcoords\0".as_ptr() as *const i8, b"...\0".as_ptr() as *const i8);
            fprintf(fp, b"\n\0".as_ptr() as *const i8);
        }
        fprintf(fp, b"\n\0".as_ptr() as *const i8);

        fprintf(fp, b"SKIN DATA %d\n\0".as_ptr() as *const i8, (*s).nskin);
        for i in 0..(*s).nskin {
            fprintf(fp, b"  SKIN DATA %d\n\0".as_ptr() as *const i8, i);
            print_int(fp, b"    face_num\0".as_ptr() as *const i8, *(*s).skinfacenum.add(i as usize));
            print_int(fp, b"    vert_adr\0".as_ptr() as *const i8, *(*s).skinvertadr.add(i as usize));
            print_int(fp, b"    vert_num\0".as_ptr() as *const i8, *(*s).skinvertnum.add(i as usize));
            print_str(fp, b"    verts\0".as_ptr() as *const i8, b"...\0".as_ptr() as *const i8);
            print_str(fp, b"    normals\0".as_ptr() as *const i8, b"...\0".as_ptr() as *const i8);
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
        print_int(fp, b"  flexvertopt\0".as_ptr() as *const i8, (*s).flexvertopt as i32);
        print_int(fp, b"  flexedgeopt\0".as_ptr() as *const i8, (*s).flexedgeopt as i32);
        print_int(fp, b"  flexfaceopt\0".as_ptr() as *const i8, (*s).flexfaceopt as i32);
        print_int(fp, b"  flexskinopt\0".as_ptr() as *const i8, (*s).flexskinopt as i32);
        print_int(fp, b"  stereo\0".as_ptr() as *const i8, (*s).stereo);
        fprintf(fp, b"\n\n\0".as_ptr() as *const i8);

        fprintf(fp, b"TRANSFORM %d\n\0".as_ptr() as *const i8, (*s).enabletransform as i32);
        if (*s).enabletransform != 0 {
            print_arr(fp, b"  translate\0".as_ptr() as *const i8, (*s).translate.as_ptr(), 3, float_format);
            print_arr(fp, b"  rotate\0".as_ptr() as *const i8, (*s).rotate.as_ptr(), 4, float_format);
            print_num(fp, b"  scale\0".as_ptr() as *const i8, (*s).scale, float_format);
            fprintf(fp, b"\n\0".as_ptr() as *const i8);
        }
        fprintf(fp, b"\n\0".as_ptr() as *const i8);

        fflush(fp);
        if !filename.is_null() {
            fclose(fp);
        }
    }
}

