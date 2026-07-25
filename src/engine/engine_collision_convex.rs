//! Port of: engine/engine_collision_convex.c
//! IR hash: 190f29be1f1f223b
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: prism_firstdir (engine/engine_collision_convex.c:47)
#[allow(unused_variables, non_snake_case)]
pub fn prism_firstdir(o1: *const (), o2: *const (), vec: *mut ccd_vec3_t) {
    // SAFETY: caller guarantees vec points to valid ccd_vec3_t (holds 3 f64 = 24 bytes)
    unsafe {
        let v = vec as *mut f64;
        *v.add(0) = 0.0;
        *v.add(1) = 0.0;
        *v.add(2) = 1.0;
    }
}

/// C: _libccd_wrapper (engine/engine_collision_convex.c:52)
/// Calls: mji_copy3, mji_zero3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn libccd_wrapper(m: *const mjModel, obj1: *mut mjCCDObj, obj2: *mut mjCCDObj, con: *mut mjPreContact, margin: f64) -> i32 {
    todo!() // _libccd_wrapper
}

/// C: mjc_penetration (engine/engine_collision_convex.c:87)
/// Calls: _libccd_wrapper, mj_freeStack, mj_markStack, mj_stackAllocByte, mjc_ccd, mjc_ccdSize, mji_sub3, mji_zero3, mju_normalize3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_penetration(m: *const mjModel, d: *mut mjData, obj1: *mut mjCCDObj, obj2: *mut mjCCDObj, con: *mut mjPreContact, ncon: i32, margin: f64) -> i32 {
    todo!() // mjc_penetration
}

/// C: mulMatTVec3 (engine/engine_collision_convex.c:174)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mul_mat_t_vec3(res: *mut f64, mat: *const f64, dir: *const f64) {
    // SAFETY: caller guarantees res[3], mat[9], dir[3] are valid
    unsafe {
        *res.add(0) = *mat.add(0) * *dir.add(0) + *mat.add(3) * *dir.add(1) + *mat.add(6) * *dir.add(2);
        *res.add(1) = *mat.add(1) * *dir.add(0) + *mat.add(4) * *dir.add(1) + *mat.add(7) * *dir.add(2);
        *res.add(2) = *mat.add(2) * *dir.add(0) + *mat.add(5) * *dir.add(1) + *mat.add(8) * *dir.add(2);
    }
}

/// C: localToGlobal (engine/engine_collision_convex.c:183)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn local_to_global(res: *mut f64, mat: *const f64, dir: *const f64, pos: *const f64) {
    // SAFETY: caller guarantees res[3], mat[9], dir[3], pos[3] are valid
    unsafe {
        *res.add(0) = *mat.add(0) * *dir.add(0) + *mat.add(1) * *dir.add(1) + *mat.add(2) * *dir.add(2);
        *res.add(1) = *mat.add(3) * *dir.add(0) + *mat.add(4) * *dir.add(1) + *mat.add(5) * *dir.add(2);
        *res.add(2) = *mat.add(6) * *dir.add(0) + *mat.add(7) * *dir.add(1) + *mat.add(8) * *dir.add(2);
        *res.add(0) += *pos.add(0);
        *res.add(1) += *pos.add(1);
        *res.add(2) += *pos.add(2);
    }
}

/// C: mjc_sphereSupport (engine/engine_collision_convex.c:202)
/// Calls: sphere
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_sphere_support(res: *mut f64, obj: *mut mjCCDObj, dir: *const f64) {
    // SAFETY: res points to 3 f64, obj is a valid mjCCDObj pointer, dir points to 3 f64 (caller contract)
    unsafe {
        let pos = (*obj).pos.as_ptr();
        let radius = (*obj).size[0];

        *res.add(0) = radius * *dir.add(0) + *pos.add(0);
        *res.add(1) = radius * *dir.add(1) + *pos.add(1);
        *res.add(2) = radius * *dir.add(2) + *pos.add(2);
    }
}

/// C: mjc_capsuleSupport (engine/engine_collision_convex.c:231)
/// Calls: localToGlobal, mulMatTVec3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_capsule_support(res: *mut f64, obj: *mut mjCCDObj, dir: *const f64) {
    // SAFETY: res points to 3 f64, obj is a valid mjCCDObj pointer, dir points to 3 f64 (caller contract)
    unsafe {
        let mat = (*obj).mat.as_ptr();
        let pos = (*obj).pos.as_ptr();
        let radius = (*obj).size[0];
        let length = (*obj).size[1];

        // rotate dir to geom local frame
        let mut local_dir: [f64; 3] = [0.0; 3];
        let mut local_supp: [f64; 3] = [0.0; 3];
        mul_mat_t_vec3(local_dir.as_mut_ptr(), mat, dir);

        // start with sphere
        local_supp[0] = local_dir[0] * radius;
        local_supp[1] = local_dir[1] * radius;
        local_supp[2] = local_dir[2] * radius;

        // add cylinder contribution
        local_supp[2] += if local_dir[2] >= 0.0 { length } else { -length };

        // transform result to global frame
        local_to_global(res, mat, local_supp.as_ptr(), pos);
    }
}

/// C: mjc_ellipsoidSupport (engine/engine_collision_convex.c:256)
/// Calls: localToGlobal, mulMatTVec3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_ellipsoid_support(res: *mut f64, obj: *mut mjCCDObj, dir: *const f64) {
    const MJ_MINVAL2: f64 = 1E-15_f64 * 1E-15_f64;

    // SAFETY: res[3], obj valid, dir[3] valid (caller contract)
    unsafe {
        let mat = (*obj).mat.as_ptr();
        let pos = (*obj).pos.as_ptr();
        let size = (*obj).size.as_ptr();

        // rotate dir to geom local frame
        let mut local_dir: [f64; 3] = [0.0; 3];
        let mut local_supp: [f64; 3] = [0.0; 3];
        mul_mat_t_vec3(local_dir.as_mut_ptr(), mat, dir);

        // find support point on unit sphere: scale dir by ellipsoid sizes
        local_supp[0] = local_dir[0] * *size.add(0);
        local_supp[1] = local_dir[1] * *size.add(1);
        local_supp[2] = local_dir[2] * *size.add(2);

        let norm2 = local_supp[0] * local_supp[0]
            + local_supp[1] * local_supp[1]
            + local_supp[2] * local_supp[2];

        // too small to normalize
        if norm2 < MJ_MINVAL2 {
            *res.add(0) = *mat.add(0) * *size.add(0) + *pos.add(0);
            *res.add(1) = *mat.add(3) * *size.add(0) + *pos.add(1);
            *res.add(2) = *mat.add(6) * *size.add(0) + *pos.add(2);
            return;
        }

        // normalize and transform to ellipsoid
        let norm_inv = 1.0 / norm2.sqrt();
        local_supp[0] *= norm_inv * *size.add(0);
        local_supp[1] *= norm_inv * *size.add(1);
        local_supp[2] *= norm_inv * *size.add(2);

        // transform result to global frame
        local_to_global(res, mat, local_supp.as_ptr(), pos);
    }
}

/// C: mjc_cylinderSupport (engine/engine_collision_convex.c:293)
/// Calls: localToGlobal, mulMatTVec3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_cylinder_support(res: *mut f64, obj: *mut mjCCDObj, dir: *const f64) {
    const MJ_MINVAL: f64 = 1E-15;
    const MJ_MINVAL2: f64 = MJ_MINVAL * MJ_MINVAL;

    // SAFETY: res points to 3 f64, obj is a valid mjCCDObj pointer, dir points to 3 f64 (caller contract)
    unsafe {
        let mat = (*obj).mat.as_ptr();
        let pos = (*obj).pos.as_ptr();
        let size = (*obj).size.as_ptr();

        // rotate dir to geom local frame
        let mut local_dir: [f64; 3] = [0.0; 3];
        let mut local_supp: [f64; 3] = [0.0; 3];
        mul_mat_t_vec3(local_dir.as_mut_ptr(), mat, dir);

        let n2 = local_dir[0] * local_dir[0] + local_dir[1] * local_dir[1];
        let scl = if n2 >= MJ_MINVAL2 { *size.add(0) / f64::sqrt(n2) } else { 0.0 };
        local_supp[0] = scl * local_dir[0];
        local_supp[1] = scl * local_dir[1];

        // set result in Z direction
        local_supp[2] = if local_dir[2] >= 0.0 { *size.add(1) } else { -*size.add(1) };

        // transform result to global frame
        local_to_global(res, mat, local_supp.as_ptr(), pos);
    }
}

/// C: mjc_boxSupport (engine/engine_collision_convex.c:317)
/// Calls: localToGlobal, mulMatTVec3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_box_support(res: *mut f64, obj: *mut mjCCDObj, dir: *const f64) {
    // SAFETY: res points to 3 f64, obj is a valid mjCCDObj pointer, dir points to 3 f64 (caller contract)
    unsafe {
        let mat = (*obj).mat.as_ptr();
        let pos = (*obj).pos.as_ptr();
        let size = (*obj).size.as_ptr();

        // rotate dir to geom local frame
        let mut local_dir: [f64; 3] = [0.0; 3];
        let mut local_supp: [f64; 3] = [0.0; 3];
        mul_mat_t_vec3(local_dir.as_mut_ptr(), mat, dir);

        // find support point in local frame
        local_supp[0] = if local_dir[0] >= 0.0 { *size.add(0) } else { -*size.add(0) };
        local_supp[1] = if local_dir[1] >= 0.0 { *size.add(1) } else { -*size.add(1) };
        local_supp[2] = if local_dir[2] >= 0.0 { *size.add(2) } else { -*size.add(2) };

        // mark the index of the corner of the box for fast lookup
        (*obj).vertindex = (if local_supp[0] > 0.0 { 1 } else { 0 })
            | (if local_supp[1] > 0.0 { 2 } else { 0 })
            | (if local_supp[2] > 0.0 { 4 } else { 0 });

        // transform support point to global frame
        local_to_global(res, mat, local_supp.as_ptr(), pos);
    }
}

/// C: dot3f (engine/engine_collision_convex.c:343)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn dot3f(a: *const f64, b: *const f32) -> f64 {
    // SAFETY: a points to 3 f64 elements, b points to 3 f32 elements (caller contract)
    unsafe {
        (*a.add(0)) * (*b.add(0) as f64) + (*a.add(1)) * (*b.add(1) as f64) + (*a.add(2)) * (*b.add(2) as f64)
    }
}

/// C: mjc_meshSupport (engine/engine_collision_convex.c:349)
/// Calls: dot3f, localToGlobal, mulMatTVec3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_mesh_support(res: *mut f64, obj: *mut mjCCDObj, dir: *const f64) {
    // SAFETY: res[3], obj valid, dir[3] valid; data.mesh fields accessed via raw offsets (caller contract)
    unsafe {
        let mat = (*obj).mat.as_ptr();
        let pos = (*obj).pos.as_ptr();

        // access mesh union fields via raw pointer into the opaque data blob
        let data_ptr = (*obj).data._data.as_ptr();
        let nverts = *(data_ptr as *const i32);
        let verts = *(data_ptr.add(8) as *const *const f32);

        let mut local_dir: [f64; 3] = [0.0; 3];
        mul_mat_t_vec3(local_dir.as_mut_ptr(), mat, dir);

        let mut max: f64 = -f32::MAX as f64;
        let mut imax: i32 = 0;

        // use cached results from previous search
        if (*obj).vertindex >= 0 {
            imax = (*obj).vertindex;
            max = dot3f(local_dir.as_ptr(), verts.add(3 * imax as usize));
        }

        // search all vertices, find maximum dot product
        for i in 0..nverts {
            let vdot = dot3f(local_dir.as_ptr(), verts.add(3 * i as usize));

            // update max
            if vdot > max {
                max = vdot;
                imax = i;
            }
        }

        // record vertex index of maximum
        (*obj).vertindex = imax;

        local_dir[0] = *verts.add(3 * imax as usize + 0) as f64;
        local_dir[1] = *verts.add(3 * imax as usize + 1) as f64;
        local_dir[2] = *verts.add(3 * imax as usize + 2) as f64;

        // transform result to global frame
        local_to_global(res, mat, local_dir.as_ptr(), pos);
    }
}

/// C: mjc_hillclimbSupport (engine/engine_collision_convex.c:391)
/// Calls: dot3f, localToGlobal, mulMatTVec3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_hillclimb_support(res: *mut f64, obj: *mut mjCCDObj, dir: *const f64) {
    // SAFETY: res[3], obj valid, dir[3] valid; data.mesh fields accessed via raw offsets (caller contract)
    unsafe {
        let mat = (*obj).mat.as_ptr();
        let pos = (*obj).pos.as_ptr();

        // access mesh union fields via raw pointer into the opaque data blob
        let data_ptr = (*obj).data._data.as_ptr();
        let verts = *(data_ptr.add(8) as *const *const f32);
        let graph = *(data_ptr.add(72) as *const *const i32);

        let numvert = *graph.add(0);
        let vert_edgeadr = graph.add(2);
        let vert_globalid = graph.add(2 + numvert as usize);
        let edge_localid = graph.add(2 + 2 * numvert as usize);

        // rotate dir to geom local frame
        let mut local_dir: [f64; 3] = [0.0; 3];
        mul_mat_t_vec3(local_dir.as_mut_ptr(), mat, dir);

        let mut max: f64 = -f32::MAX as f64;
        let mut prev: i32 = -1;
        let mut imax: i32 = if (*obj).meshindex >= 0 { (*obj).meshindex } else { 0 };

        // hillclimb until no change
        while imax != prev {
            prev = imax;
            let mut i = *vert_edgeadr.add(imax as usize);
            loop {
                let subidx = *edge_localid.add(i as usize);
                if subidx < 0 {
                    break;
                }
                let vdot = dot3f(local_dir.as_ptr(), verts.add(3 * *vert_globalid.add(subidx as usize) as usize));
                if vdot > max {
                    max = vdot;
                    imax = subidx;
                }
                i += 1;
            }
        }

        // record vertex index of maximum (local id)
        (*obj).meshindex = imax;

        // get resulting support vertex
        let global_id = *vert_globalid.add(imax as usize);
        (*obj).vertindex = global_id;
        local_dir[0] = *verts.add(3 * global_id as usize + 0) as f64;
        local_dir[1] = *verts.add(3 * global_id as usize + 1) as f64;
        local_dir[2] = *verts.add(3 * global_id as usize + 2) as f64;

        // transform result to global frame
        local_to_global(res, mat, local_dir.as_ptr(), pos);
    }
}

/// C: mjc_prism_support (engine/engine_collision_convex.c:436)
/// Calls: mji_copy3, mju_dot3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_prism_support(res: *mut f64, obj: *mut mjCCDObj, dir: *const f64) {
    // SAFETY: obj is a valid mjCCDObj pointer, data.hfield.prism is at offset 0 in the union,
    // stored as mjtNum[6][3] = 6 rows of 3 f64 values. dir[3] and res[3] are valid.
    unsafe {
        // hfield.prism is at offset 0 of the union data blob
        let prism = (*obj).data._data.as_ptr() as *const f64;

        // find best vertex in halfspace determined by dir.z
        let istart: i32 = if *dir.add(2) < 0.0 { 0 } else { 3 };
        let mut ibest: i32 = istart;
        let mut best: f64 = crate::engine::engine_util_blas::mju_dot3(
            prism.add((istart * 3) as usize), dir);

        for i in 1..3_i32 {
            let tmp = crate::engine::engine_util_blas::mju_dot3(
                prism.add(((istart + i) * 3) as usize), dir);
            if tmp > best {
                ibest = istart + i;
                best = tmp;
            }
        }

        // copy best point
        crate::engine::engine_inline::mji_copy3(res, prism.add((ibest * 3) as usize));
    }
}

/// C: mjc_flexSupport (engine/engine_collision_convex.c:458)
/// Calls: mji_addScl3, mji_addToScl3, mji_copy3, mju_dot3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_flex_support(res: *mut f64, obj: *mut mjCCDObj, dir: *const f64) {
    // SAFETY: obj valid, dir[3] valid, res[3] valid. Flex union fields accessed via raw offsets:
    //   elem: *const i32 at offset 0, dim: *const i32 at offset 8,
    //   elemdataadr: *const i32 at offset 32, vert_xpos: *const f64 at offset 40,
    //   vertadr: *const i32 at offset 48, xradius: *const f64 at offset 56
    unsafe {
        let f = (*obj).flex;
        let data_ptr = (*obj).data._data.as_ptr();

        // flex union field pointers
        let flex_elem = *(data_ptr.add(0) as *const *const i32);
        let flex_dim = *(data_ptr.add(8) as *const *const i32);
        let flex_elemdataadr = *(data_ptr.add(32) as *const *const i32);
        let flex_vert_xpos = *(data_ptr.add(40) as *const *const f64);
        let flex_vertadr = *(data_ptr.add(48) as *const *const i32);
        let flex_xradius = *(data_ptr.add(56) as *const *const f64);

        let dim = *flex_dim.add(f as usize);

        // flex element
        if (*obj).elem >= 0 {
            let e = (*obj).elem;
            let edata = flex_elem.add(
                *flex_elemdataadr.add(f as usize) as usize + (e * (dim + 1)) as usize
            );
            let vert = flex_vert_xpos.add(3 * *flex_vertadr.add(f as usize) as usize);

            // find element vertex with largest projection along dir
            crate::engine::engine_inline::mji_copy3(res, vert.add(3 * *edata.add(0) as usize));
            let mut best = crate::engine::engine_util_blas::mju_dot3(res, dir);

            for i in 1..=dim {
                let dot = crate::engine::engine_util_blas::mju_dot3(
                    vert.add(3 * *edata.add(i as usize) as usize), dir);
                if dot > best {
                    best = dot;
                    crate::engine::engine_inline::mji_copy3(
                        res, vert.add(3 * *edata.add(i as usize) as usize));
                }
            }

            // add radius and margin/2
            crate::engine::engine_inline::mji_add_to_scl3(
                res, dir, *flex_xradius.add(f as usize) + 0.5 * (*obj).margin);
            return;
        }

        // flex vertex
        let vert = flex_vert_xpos.add(
            3 * (*flex_vertadr.add(f as usize) + (*obj).vert) as usize);
        crate::engine::engine_inline::mji_add_scl3(
            res, vert, dir, *flex_xradius.add(f as usize) + 0.5 * (*obj).margin);
    }
}

/// C: mjc_setCCDObjFlex (engine/engine_collision_convex.c:790)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_set_ccd_obj_flex(obj: *mut mjCCDObj, flex: i32, elem: i32, vert: i32) {
    // SAFETY: obj is a valid mjCCDObj pointer (caller contract)
    unsafe {
        (*obj).flex = flex;
        (*obj).elem = elem;
        (*obj).vert = vert;
    }
}

/// C: mjc_isDistinctContact (engine/engine_collision_convex.c:798)
/// Calls: mju_dist3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_is_distinct_contact(con: *const mjPreContact, ncon: i32, tolerance: f64) -> i32 {
    // SAFETY: con points to array of ncon mjPreContact elements (caller contract)
    unsafe {
        let last_pos = (*con.add((ncon - 1) as usize)).pos.as_ptr();
        for i in 0..(ncon - 1) as usize {
            if crate::engine::engine_util_blas::mju_dist3(
                (*con.add(i)).pos.as_ptr(),
                last_pos,
            ) <= tolerance
            {
                return 0;
            }
        }
        1
    }
}

/// C: mju_rotateFrame (engine/engine_collision_convex.c:810)
/// Calls: mji_sub3, mji_subFrom3, mju_copy, mju_mulMatMat3, mju_mulMatVec3, mju_subFrom3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_rotate_frame(origin: *const f64, rot: *const f64, xmat: *mut f64, xpos: *mut f64) {
    // SAFETY: origin[3], rot[9], xmat[9], xpos[3] are valid (caller contract).
    unsafe {
        let mut mat: [f64; 9] = [0.0; 9];
        let mut vec: [f64; 3] = [0.0; 3];
        let mut rel: [f64; 3] = [0.0; 3];

        // rotate frame: xmat = rot*xmat
        crate::engine::engine_util_blas::mju_mul_mat_mat3(mat.as_mut_ptr(), rot, xmat);
        crate::engine::engine_util_blas::mju_copy(xmat, mat.as_ptr(), 9);

        // vector to rotation origin: rel = origin - xpos
        crate::engine::engine_inline::mji_sub3(rel.as_mut_ptr(), origin, xpos);

        // displacement of origin due to rotation: vec = rot*rel - rel
        crate::engine::engine_util_blas::mju_mul_mat_vec3(vec.as_mut_ptr(), rot, rel.as_ptr());
        crate::engine::engine_util_blas::mju_sub_from3(vec.as_mut_ptr(), rel.as_ptr());

        // correct xpos by subtracting displacement: xpos = xpos - vec
        crate::engine::engine_inline::mji_sub_from3(xpos, vec.as_ptr());
    }
}

/// C: maxContacts (engine/engine_collision_convex.c:831)
#[allow(unused_variables, non_snake_case)]
pub fn max_contacts(m: *const mjModel, obj1: *const mjCCDObj, obj2: *const mjCCDObj) -> i32 {
    const mjGEOM_BOX: i32 = 6;
    const mjGEOM_MESH: i32 = 7;
    const mjDSBL_MULTICCD: i32 = 1 << 19;

    // SAFETY: m, obj1, obj2 are valid pointers (caller contract)
    unsafe {
        // single pass not supported for margins
        if (*obj1).margin > 0.0 || (*obj2).margin > 0.0 {
            return 1;
        }

        // can return 8 contacts for box-box collision in one pass
        let type1: i32 = (*obj1).geom_type;
        let type2: i32 = (*obj2).geom_type;
        if type1 == mjGEOM_BOX && type2 == mjGEOM_BOX {
            return 8;
        }

        // reduce mesh collisions to 4 contacts max
        if type1 == mjGEOM_BOX || type1 == mjGEOM_MESH {
            if type2 == mjGEOM_BOX || type2 == mjGEOM_MESH {
                return if ((*m).opt.disableflags & mjDSBL_MULTICCD) != 0 { 1 } else { 4 };
            }
        }

        // not supported for other geom types
        1
    }
}

/// C: addplanemesh (engine/engine_collision_convex.c:946)
/// Calls: mji_addToScl3, mji_copy3, mji_sub3, mji_zero3, mju_addTo3, mju_dist3, mju_dot3, mju_mulMatVec3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn addplanemesh(con: *mut mjPreContact, vertex: *const f32, pos1: *const f64, normal1: *const f64, pos2: *const f64, mat2: *const f64, first: *const f64, rbound: f64) -> i32 {
    const TOLPLANEMESH: f64 = 0.3;
    // SAFETY: caller guarantees all pointers valid and arrays properly sized
    unsafe {
        // compute point in global coordinates
        let mut pnt: [f64; 3] = [0.0; 3];
        let v: [f64; 3] = [*vertex.add(0) as f64, *vertex.add(1) as f64, *vertex.add(2) as f64];
        crate::engine::engine_util_blas::mju_mul_mat_vec3(pnt.as_mut_ptr(), mat2, v.as_ptr());
        crate::engine::engine_util_blas::mju_add_to3(pnt.as_mut_ptr(), pos2);

        // skip if too close to first contact
        if crate::engine::engine_util_blas::mju_dist3(pnt.as_ptr(), first) < TOLPLANEMESH * rbound {
            return 0;
        }

        // pnt-pos difference vector
        let mut dif: [f64; 3] = [0.0; 3];
        crate::engine::engine_inline::mji_sub3(dif.as_mut_ptr(), pnt.as_ptr(), pos1);

        // set distance
        (*con).dist = crate::engine::engine_util_blas::mju_dot3(normal1, dif.as_ptr());

        // set position
        crate::engine::engine_inline::mji_copy3((*con).pos.as_mut_ptr(), pnt.as_ptr());
        crate::engine::engine_inline::mji_add_to_scl3((*con).pos.as_mut_ptr(), normal1, -0.5 * (*con).dist);

        // set frame
        crate::engine::engine_inline::mji_copy3((*con).normal.as_mut_ptr(), normal1);
        crate::engine::engine_inline::mji_zero3((*con).tangent.as_mut_ptr());

        1
    }
}

/// C: addVert (engine/engine_collision_convex.c:1085)
/// Calls: mji_copy3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn add_vert(obj: *mut mjCCDObj, x: f64, y: f64, z: f64) {
    todo!() // addVert
}

/// C: addPrismVert (engine/engine_collision_convex.c:1100)
/// Calls: mji_copy3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn add_prism_vert(obj: *mut mjCCDObj, r: i32, c: i32, i: i32, dx: f64, dy: f64, margin: f64) {
    todo!() // addPrismVert
}

/// C: mjc_ellipsoidInside (engine/engine_collision_convex.c:1282)
/// Calls: mji_addScl3, mji_copy3, mju_dist3, mju_normalize3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_ellipsoid_inside(nrm: *mut f64, pos: *const f64, size: *const f64) -> i32 {
    todo!() // mjc_ellipsoidInside
}

/// C: mjc_ellipsoidOutside (engine/engine_collision_convex.c:1337)
/// Calls: mju_normalize3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_ellipsoid_outside(nrm: *mut f64, pos: *const f64, size: *const f64) -> i32 {
    const MJ_MINVAL: f64 = 1E-15_f64;

    // SAFETY: caller guarantees nrm[3], pos[3], size[3] are valid
    unsafe {
        // algorithm constants
        let maxiter: i32 = 30;
        let tolerance: f64 = 1e-6;

        // precompute quantities
        let S2: [f64; 3] = [
            *size.add(0) * *size.add(0),
            *size.add(1) * *size.add(1),
            *size.add(2) * *size.add(2),
        ];
        let PS2: [f64; 3] = [
            *pos.add(0) * *pos.add(0) * S2[0],
            *pos.add(1) * *pos.add(1) * S2[1],
            *pos.add(2) * *pos.add(2) * S2[2],
        ];

        // main iteration
        let mut la: f64 = 0.0;
        let mut iter: i32 = 0;
        while iter < maxiter {
            // precompute 1/(s^2+la)
            let R: [f64; 3] = [
                1.0 / (S2[0] + la),
                1.0 / (S2[1] + la),
                1.0 / (S2[2] + la),
            ];

            // value
            let val = PS2[0] * R[0] * R[0] + PS2[1] * R[1] * R[1] + PS2[2] * R[2] * R[2] - 1.0;
            if val < tolerance {
                break;
            }

            // derivative
            let deriv = -2.0
                * (PS2[0] * R[0] * R[0] * R[0]
                    + PS2[1] * R[1] * R[1] * R[1]
                    + PS2[2] * R[2] * R[2] * R[2]);
            if deriv > -MJ_MINVAL {
                break;
            }

            // delta
            let delta = -val / deriv;
            if delta < tolerance {
                break;
            }

            // update
            la += delta;
            iter += 1;
        }

        // compute normal given lambda
        *nrm.add(0) = *pos.add(0) / (S2[0] + la);
        *nrm.add(1) = *pos.add(1) / (S2[1] + la);
        *nrm.add(2) = *pos.add(2) / (S2[2] + la);
        crate::engine::engine_util_blas::mju_normalize3(nrm);

        1
    }
}

/// C: mjc_initCCDObj (engine/engine_collision_convex.h:94)
/// Calls: mju_copy, mju_zero4
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_init_ccd_obj(obj: *mut mjCCDObj, m: *const mjModel, d: *const mjData, g: i32, margin: f64) {
    const mjGEOM_HFIELD: i32 = 1;
    const mjGEOM_SPHERE: i32 = 2;
    const mjGEOM_CAPSULE: i32 = 3;
    const mjGEOM_ELLIPSOID: i32 = 4;
    const mjGEOM_CYLINDER: i32 = 5;
    const mjGEOM_BOX: i32 = 6;
    const mjGEOM_MESH: i32 = 7;
    const mjGEOM_SDF: i32 = 8;
    const mjGEOM_FLEX: i32 = 105;
    const mjMESH_HILLCLIMB_MIN: i32 = 10;

    #[repr(C)]
    struct MeshData {
        nvert: i32,
        mesh_polynum: i32,
        vert: *const f32,
        mpolymapadr: *const i32,
        mpolymapnum: *const i32,
        polymap: *const i32,
        polyvertadr: *const i32,
        polyvertnum: *const i32,
        polyvert: *const i32,
        polynormal: *const f64,
        graph: *const i32,
    }

    #[repr(C)]
    struct HfieldData {
        prism: [f64; 18],
        hfield_data: *const f32,
        hfield_nrow: i32,
        hfield_ncol: i32,
    }

    #[repr(C)]
    struct FlexData {
        elem: *const i32,
        dim: *const i32,
        aabb: *const f64,
        elemadr: *const i32,
        elemdataadr: *const i32,
        vert_xpos: *const f64,
        vertadr: *const i32,
        xradius: *const f64,
    }

    // SAFETY: obj, m, d are valid pointers from caller. All field accesses follow C layout exactly.
    unsafe {
        (*obj).geom = g;
        (*obj).margin = margin;
        (*obj).center = Some(std::mem::transmute::<_, unsafe extern "C" fn()>(mjc_center as usize));
        (*obj).vertindex = -1;
        (*obj).meshindex = -1;
        (*obj).flex = -1;
        (*obj).elem = -1;
        (*obj).vert = -1;

        // mju_zero4(obj->rotate)
        (*obj).rotate[0] = 0.0;
        (*obj).rotate[1] = 0.0;
        (*obj).rotate[2] = 0.0;
        (*obj).rotate[3] = 0.0;
        (*obj).rotate[0] = 1.0;

        if g >= 0 {
            // mju_copy(obj->size, m->geom_size+3*g, 3)
            crate::engine::engine_util_blas::mju_copy(
                (*obj).size.as_mut_ptr(),
                (*m).geom_size.add(3 * g as usize),
                3,
            );
            // mju_copy(obj->pos, d->geom_xpos+3*g, 3)
            crate::engine::engine_util_blas::mju_copy(
                (*obj).pos.as_mut_ptr(),
                (*d).geom_xpos.add(3 * g as usize),
                3,
            );
            // mju_copy(obj->mat, d->geom_xmat+9*g, 9)
            crate::engine::engine_util_blas::mju_copy(
                (*obj).mat.as_mut_ptr(),
                (*d).geom_xmat.add(9 * g as usize),
                9,
            );

            (*obj).geom_type = *(*m).geom_type.add(g as usize);

            match (*obj).geom_type {
                mjGEOM_ELLIPSOID => {
                    (*obj).support = Some(std::mem::transmute::<_, unsafe extern "C" fn()>(mjc_ellipsoid_support as usize));
                }
                mjGEOM_MESH | mjGEOM_SDF => {
                    let dataid = *(*m).geom_dataid.add(g as usize) as usize;
                    let graphadr = *(*m).mesh_graphadr.add(dataid);
                    let vertadr = *(*m).mesh_vertadr.add(dataid);
                    let polyadr = *(*m).mesh_polyadr.add(dataid);

                    let mesh_ptr = &mut (*obj).data as *mut _ as *mut MeshData;

                    if graphadr < 0 || *(*m).mesh_vertnum.add(dataid) < mjMESH_HILLCLIMB_MIN {
                        (*mesh_ptr).graph = std::ptr::null();
                        (*obj).support = Some(std::mem::transmute::<_, unsafe extern "C" fn()>(mjc_mesh_support as usize));
                    } else {
                        (*mesh_ptr).graph = (*m).mesh_graph.add(graphadr as usize);
                        (*obj).support = Some(std::mem::transmute::<_, unsafe extern "C" fn()>(mjc_hillclimb_support as usize));
                    }

                    (*mesh_ptr).vert = (*m).mesh_vert.add(3 * vertadr as usize);
                    (*mesh_ptr).nvert = *(*m).mesh_vertnum.add(dataid);
                    (*mesh_ptr).mpolymapadr = (*m).mesh_polymapadr.add(vertadr as usize);
                    (*mesh_ptr).mpolymapnum = (*m).mesh_polymapnum.add(vertadr as usize);
                    (*mesh_ptr).polymap = (*m).mesh_polymap;
                    (*mesh_ptr).polynormal = (*m).mesh_polynormal.add(3 * polyadr as usize);
                    (*mesh_ptr).polyvertadr = (*m).mesh_polyvertadr.add(polyadr as usize);
                    (*mesh_ptr).polyvertnum = (*m).mesh_polyvertnum.add(polyadr as usize);
                    (*mesh_ptr).polyvert = (*m).mesh_polyvert;
                    (*mesh_ptr).mesh_polynum = *(*m).mesh_polynum.add(dataid);
                }
                mjGEOM_SPHERE => {
                    (*obj).support = Some(std::mem::transmute::<_, unsafe extern "C" fn()>(mjc_sphere_support as usize));
                }
                mjGEOM_CAPSULE => {
                    (*obj).support = Some(std::mem::transmute::<_, unsafe extern "C" fn()>(mjc_capsule_support as usize));
                }
                mjGEOM_CYLINDER => {
                    (*obj).support = Some(std::mem::transmute::<_, unsafe extern "C" fn()>(mjc_cylinder_support as usize));
                }
                mjGEOM_BOX => {
                    (*obj).support = Some(std::mem::transmute::<_, unsafe extern "C" fn()>(mjc_box_support as usize));
                }
                mjGEOM_HFIELD => {
                    (*obj).center = Some(std::mem::transmute::<_, unsafe extern "C" fn()>(mjc_center as usize));
                    (*obj).support = Some(std::mem::transmute::<_, unsafe extern "C" fn()>(mjc_prism_support as usize));
                    let hid = *(*m).geom_dataid.add(g as usize) as usize;

                    let hfield_ptr = &mut (*obj).data as *mut _ as *mut HfieldData;
                    (*hfield_ptr).hfield_nrow = *(*m).hfield_nrow.add(hid);
                    (*hfield_ptr).hfield_ncol = *(*m).hfield_ncol.add(hid);

                    // mju_copy(obj->size, m->hfield_size + 4*hid, 4)
                    crate::engine::engine_util_blas::mju_copy(
                        (*obj).size.as_mut_ptr(),
                        (*m).hfield_size.add(4 * hid),
                        4,
                    );

                    (*hfield_ptr).hfield_data = (*m).hfield_data.add(*(*m).hfield_adr.add(hid) as usize);
                }
                _ => {
                    (*obj).support = None;
                }
            }
        } else {
            (*obj).geom_type = mjGEOM_FLEX;

            let flex_ptr = &mut (*obj).data as *mut _ as *mut FlexData;
            (*flex_ptr).dim = (*m).flex_dim;
            (*obj).support = Some(std::mem::transmute::<_, unsafe extern "C" fn()>(mjc_flex_support as usize));
            (*flex_ptr).aabb = (*d).flexelem_aabb;
            (*flex_ptr).elemadr = (*m).flex_elemadr;
            (*flex_ptr).vert_xpos = (*d).flexvert_xpos;
            (*flex_ptr).vertadr = (*m).flex_vertadr;
            (*flex_ptr).xradius = (*m).flex_radius;
            (*flex_ptr).elemdataadr = (*m).flex_elemdataadr;
            (*flex_ptr).elem = (*m).flex_elem;
        }
    }
}

/// C: mjc_center (engine/engine_collision_convex.h:97)
/// Calls: mji_addTo3, mji_copy3, mju_scl3, mju_zero3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_center(res: *mut f64, obj: *const mjCCDObj) {
    // SAFETY: obj valid, res[3] valid. Union data accessed via raw offsets.
    // hfield.prism at offset 0: mjtNum[6][3]. flex.aabb at offset 16, flex.elemadr at offset 24,
    // flex.vert_xpos at offset 40, flex.vertadr at offset 48.
    unsafe {
        let g = (*obj).geom;
        let f = (*obj).flex;
        let e = (*obj).elem;
        let v = (*obj).vert;

        const MJ_GEOM_HFIELD: i32 = 1;
        if (*obj).geom_type == MJ_GEOM_HFIELD {
            crate::engine::engine_util_blas::mju_zero3(res);
            let prism = (*obj).data._data.as_ptr() as *const f64;
            for i in 0..6_i32 {
                crate::engine::engine_inline::mji_add_to3(res, prism.add((i * 3) as usize));
            }
            crate::engine::engine_util_blas::mju_scl3(res, res, 1.0 / 6.0);
            return;
        }

        // return geom position
        if g >= 0 {
            crate::engine::engine_inline::mji_copy3(res, (*obj).pos.as_ptr());
            return;
        }

        // return flex element position
        if e >= 0 {
            let data_ptr = (*obj).data._data.as_ptr();
            let flex_aabb = *(data_ptr.add(16) as *const *const f64);
            let flex_elemadr = *(data_ptr.add(24) as *const *const i32);
            crate::engine::engine_inline::mji_copy3(
                res, flex_aabb.add(6 * (*flex_elemadr.add(f as usize) + e) as usize));
            return;
        }

        // return flex vertex position
        if f >= 0 {
            let data_ptr = (*obj).data._data.as_ptr();
            let flex_vert_xpos = *(data_ptr.add(40) as *const *const f64);
            let flex_vertadr = *(data_ptr.add(48) as *const *const i32);
            crate::engine::engine_inline::mji_copy3(
                res, flex_vert_xpos.add(3 * (*flex_vertadr.add(f as usize) + v) as usize));
            return;
        }
    }
}

/// C: mjccd_center (engine/engine_collision_convex.h:100)
/// Calls: mjc_center
#[allow(unused_variables, non_snake_case)]
pub fn mjccd_center(obj: *const (), center: *mut ccd_vec3_t) {
    // SAFETY: obj is actually a *const mjCCDObj. center.v is [u8;24] but represents [f64;3].
    unsafe {
        let v_ptr = (*center).v.as_mut_ptr() as *mut f64;
        mjc_center(v_ptr, obj as *const mjCCDObj);
    }
}

/// C: mjccd_support (engine/engine_collision_convex.h:103)
/// Calls: mjc_prism_support, mji_addScl3, mji_addTo3, mji_addToScl3, mji_copy3, mji_scl3, mju_dot3, mju_message, mju_mulMatTVec3, mju_mulMatVec3, mju_normalize3, mju_sign, mju_warning, mju_zero3
#[allow(unused_variables, non_snake_case)]
pub fn mjccd_support(obj: *const (), dir: *const ccd_vec3_t, vec: *mut ccd_vec3_t) {
    // SAFETY: obj is actually *mut mjCCDObj. dir.v and vec.v are [u8;24] representing [f64;3].
    // All pointer arithmetic follows the C original exactly.
    unsafe {
        let obj = obj as *mut mjCCDObj;
        let res: *mut f64 = (*vec).v.as_mut_ptr() as *mut f64;
        let dir: *const f64 = (*dir).v.as_ptr() as *const f64;
        let g = (*obj).geom;

        const MJ_MINVAL: f64 = 1E-15;
        const MJ_GEOM_HFIELD: i32 = 1;
        const MJ_GEOM_SPHERE: i32 = 2;
        const MJ_GEOM_CAPSULE: i32 = 3;
        const MJ_GEOM_ELLIPSOID: i32 = 4;
        const MJ_GEOM_CYLINDER: i32 = 5;
        const MJ_GEOM_BOX: i32 = 6;
        const MJ_GEOM_MESH: i32 = 7;
        const MJ_GEOM_SDF: i32 = 8;

        // flex support (g < 0)
        if g < 0 {
            let f = (*obj).flex;
            let data_ptr = (*obj).data._data.as_ptr();
            let flex_elem = *(data_ptr.add(0) as *const *const i32);
            let flex_dim = *(data_ptr.add(8) as *const *const i32);
            let flex_elemdataadr = *(data_ptr.add(32) as *const *const i32);
            let flex_vert_xpos = *(data_ptr.add(40) as *const *const f64);
            let flex_vertadr = *(data_ptr.add(48) as *const *const i32);
            let flex_xradius = *(data_ptr.add(56) as *const *const f64);
            let dim = *flex_dim.add(f as usize);

            // flex element
            if (*obj).elem >= 0 {
                let e = (*obj).elem;
                let edata = flex_elem.add(
                    *flex_elemdataadr.add(f as usize) as usize + (e * (dim + 1)) as usize);
                let vert = flex_vert_xpos.add(3 * *flex_vertadr.add(f as usize) as usize);

                crate::engine::engine_inline::mji_copy3(res, vert.add(3 * *edata.add(0) as usize));
                let mut best = crate::engine::engine_util_blas::mju_dot3(res, dir);
                for i in 1..=dim {
                    let dot = crate::engine::engine_util_blas::mju_dot3(
                        vert.add(3 * *edata.add(i as usize) as usize), dir);
                    if dot > best {
                        best = dot;
                        crate::engine::engine_inline::mji_copy3(
                            res, vert.add(3 * *edata.add(i as usize) as usize));
                    }
                }
                crate::engine::engine_inline::mji_add_to_scl3(
                    res, dir, *flex_xradius.add(f as usize) + 0.5 * (*obj).margin);
                return;
            }
            // flex vertex
            else {
                let vert = flex_vert_xpos.add(
                    3 * (*flex_vertadr.add(f as usize) + (*obj).vert) as usize);
                crate::engine::engine_inline::mji_add_scl3(
                    res, vert, dir, *flex_xradius.add(f as usize) + 0.5 * (*obj).margin);
                return;
            }
        }

        let size = (*obj).size.as_ptr();
        let mut local_dir: [f64; 3] = [0.0; 3];

        // rotate dir to geom local frame
        crate::engine::engine_util_blas::mju_mul_mat_t_vec3(
            local_dir.as_mut_ptr(), (*obj).mat.as_ptr(), dir);

        // compute result according to geom type
        match (*obj).geom_type {
            MJ_GEOM_SPHERE => {
                crate::engine::engine_inline::mji_scl3(res, local_dir.as_ptr(), *size.add(0));
            }

            MJ_GEOM_CAPSULE => {
                crate::engine::engine_inline::mji_scl3(res, local_dir.as_ptr(), *size.add(0));
                *res.add(2) += crate::engine::engine_util_misc::mju_sign(local_dir[2]) * *size.add(1);
            }

            MJ_GEOM_ELLIPSOID => {
                for i in 0..3_usize {
                    *res.add(i) = local_dir[i] * *size.add(i);
                }
                crate::engine::engine_util_blas::mju_normalize3(res);
                for i in 0..3_usize {
                    *res.add(i) *= *size.add(i);
                }
            }

            MJ_GEOM_CYLINDER => {
                let tmp = f64::sqrt(local_dir[0] * local_dir[0] + local_dir[1] * local_dir[1]);
                if tmp > MJ_MINVAL {
                    *res.add(0) = local_dir[0] / tmp * *size.add(0);
                    *res.add(1) = local_dir[1] / tmp * *size.add(0);
                } else {
                    *res.add(0) = 0.0;
                    *res.add(1) = 0.0;
                }
                *res.add(2) = crate::engine::engine_util_misc::mju_sign(local_dir[2]) * *size.add(1);
            }

            MJ_GEOM_BOX => {
                for i in 0..3_usize {
                    *res.add(i) = crate::engine::engine_util_misc::mju_sign(local_dir[i]) * *size.add(i);
                }
            }

            MJ_GEOM_MESH | MJ_GEOM_SDF => {
                let data_ptr = (*obj).data._data.as_ptr();
                let vertdata = *(data_ptr.add(8) as *const *const f32);
                let mut tmp: f64 = -1E+10;
                let mut ibest: i32 = -1;

                // check if graph is available (offset 72 in mesh union = graph pointer)
                let graph = *(data_ptr.add(72) as *const *const i32);

                if graph.is_null() {
                    // no graph: exhaustive search
                    let nvert = *(data_ptr as *const i32);
                    for i in 0..nvert {
                        let vdot = local_dir[0] * *vertdata.add(3 * i as usize + 0) as f64
                                 + local_dir[1] * *vertdata.add(3 * i as usize + 1) as f64
                                 + local_dir[2] * *vertdata.add(3 * i as usize + 2) as f64;
                        if vdot > tmp {
                            tmp = vdot;
                            ibest = i;
                        }
                    }
                    (*obj).meshindex = ibest;
                } else {
                    // hill-climb using graph
                    let numvert = *graph.add(0);
                    let vert_edgeadr = graph.add(2);
                    let vert_globalid = graph.add(2 + numvert as usize);
                    let edge_localid = graph.add(2 + 2 * numvert as usize);

                    ibest = if (*obj).meshindex < 0 { 0 } else { (*obj).meshindex };
                    tmp = local_dir[0] * *vertdata.add(3 * *vert_globalid.add(ibest as usize) as usize + 0) as f64
                        + local_dir[1] * *vertdata.add(3 * *vert_globalid.add(ibest as usize) as usize + 1) as f64
                        + local_dir[2] * *vertdata.add(3 * *vert_globalid.add(ibest as usize) as usize + 2) as f64;

                    let mut change: i32 = 1;
                    while change != 0 {
                        change = 0;
                        let mut i = *vert_edgeadr.add(ibest as usize);
                        loop {
                            let locid = *edge_localid.add(i as usize);
                            if locid < 0 { break; }
                            let vdot = local_dir[0] * *vertdata.add(3 * *vert_globalid.add(locid as usize) as usize) as f64
                                     + local_dir[1] * *vertdata.add(3 * *vert_globalid.add(locid as usize) as usize + 1) as f64
                                     + local_dir[2] * *vertdata.add(3 * *vert_globalid.add(locid as usize) as usize + 2) as f64;
                            if vdot > tmp {
                                tmp = vdot;
                                ibest = locid;
                                change = 1;
                            }
                            i += 1;
                        }
                    }
                    (*obj).meshindex = ibest;
                    ibest = *vert_globalid.add(ibest as usize);
                }

                if ibest < 0 {
                    crate::engine::engine_util_errmem::mju_warning(
                        b"mesh_support could not find support vertex\0".as_ptr() as *const i8);
                    crate::engine::engine_util_blas::mju_zero3(res);
                } else {
                    for i in 0..3_usize {
                        *res.add(i) = *vertdata.add(3 * ibest as usize + i) as f64;
                    }
                }
            }

            MJ_GEOM_HFIELD => {
                mjc_prism_support(res, obj, dir);
                return;
            }

            _ => {
                crate::engine::engine_util_errmem::mju_warning(
                    b"ccd support function is undefined for geom type\0".as_ptr() as *const i8);
            }
        }

        // add local_dir*margin/2 to result
        for i in 0..3_usize {
            *res.add(i) += local_dir[i] * (*obj).margin / 2.0;
        }

        // rotate result to global frame
        crate::engine::engine_util_blas::mju_mul_mat_vec3(res, (*obj).mat.as_ptr(), res);

        // add geom position
        crate::engine::engine_inline::mji_add_to3(res, (*obj).pos.as_ptr());
    }
}

/// C: mjc_pointSupport (engine/engine_collision_convex.h:106)
/// Calls: mji_copy3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_point_support(res: *mut f64, obj: *mut mjCCDObj, dir: *const f64) {
    // SAFETY: res points to 3 f64, obj is a valid mjCCDObj pointer (caller contract)
    unsafe {
        *res.add(0) = (*obj).pos[0];
        *res.add(1) = (*obj).pos[1];
        *res.add(2) = (*obj).pos[2];
    }
}

/// C: mjc_lineSupport (engine/engine_collision_convex.h:109)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_line_support(res: *mut f64, obj: *mut mjCCDObj, dir: *const f64) {
    // SAFETY: res, obj, dir are valid pointers from caller
    unsafe {
        let mat = (*obj).mat.as_ptr();
        let pos = (*obj).pos.as_ptr();
        let length = (*obj).size[1];

        let dot = *mat.add(2) * *dir.add(0) + *mat.add(5) * *dir.add(1) + *mat.add(8) * *dir.add(2);
        let scl = if dot >= 0.0 { length } else { -length };

        *res.add(0) = *mat.add(2) * scl + *pos.add(0);
        *res.add(1) = *mat.add(5) * scl + *pos.add(1);
        *res.add(2) = *mat.add(8) * scl + *pos.add(2);
    }
}

/// C: mjc_PlaneConvex (engine/engine_collision_convex.h:112)
/// Calls: addplanemesh, mjc_initCCDObj, mjccd_support, mji_addToScl3, mji_copy3, mji_sub3, mji_zero3, mju_dot3, mju_mulMatTVec3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_plane_convex(m: *const mjModel, d: *mut mjData, con: *mut mjPreContact, g1: i32, g2: i32, margin: f64) -> i32 {
    const MAXPLANEMESH: i32 = 3;

    // SAFETY: m, d, con valid pointers. All field accesses follow C layout exactly.
    unsafe {
        let pos1 = (*d).geom_xpos.add(3 * g1 as usize);
        let mat1 = (*d).geom_xmat.add(9 * g1 as usize);
        let pos2 = (*d).geom_xpos.add(3 * g2 as usize);
        let mat2 = (*d).geom_xmat.add(9 * g2 as usize);

        let mut dif: [f64; 3] = [0.0; 3];
        let normal: [f64; 3] = [*mat1.add(2), *mat1.add(5), *mat1.add(8)];

        let mut ccd_dir = ccd_vec3_t { v: [0u8; 24] };
        let mut ccd_vec = ccd_vec3_t { v: [0u8; 24] };
        let mut obj_bytes: [u8; std::mem::size_of::<mjCCDObj>()] = [0xAA; std::mem::size_of::<mjCCDObj>()];
        let obj_ptr: *mut mjCCDObj = obj_bytes.as_mut_ptr() as *mut mjCCDObj;
        mjc_init_ccd_obj(obj_ptr, m, d as *const mjData, g2, 0.0);

        // get support point in -normal direction: ccdVec3Set(&ccd_dir, -mat1[2], -mat1[5], -mat1[8])
        let dir_v = ccd_dir.v.as_mut_ptr() as *mut f64;
        *dir_v.add(0) = -*mat1.add(2);
        *dir_v.add(1) = -*mat1.add(5);
        *dir_v.add(2) = -*mat1.add(8);
        mjccd_support(obj_ptr as *const mjCCDObj as *const (), &ccd_dir, &mut ccd_vec);

        // compute normal distance, return if too far
        let vec_v = ccd_vec.v.as_ptr() as *const f64;
        crate::engine::engine_inline::mji_sub3(dif.as_mut_ptr(), vec_v, pos1);
        (*con.add(0)).dist = crate::engine::engine_util_blas::mju_dot3(
            normal.as_ptr(), dif.as_ptr());
        if (*con.add(0)).dist > margin {
            return 0;
        }

        // fill in contact data
        crate::engine::engine_inline::mji_copy3((*con.add(0)).pos.as_mut_ptr(), vec_v);
        crate::engine::engine_inline::mji_add_to_scl3(
            (*con.add(0)).pos.as_mut_ptr(), normal.as_ptr(), -0.5 * (*con.add(0)).dist);
        crate::engine::engine_inline::mji_copy3((*con.add(0)).normal.as_mut_ptr(), normal.as_ptr());
        crate::engine::engine_inline::mji_zero3((*con.add(0)).tangent.as_mut_ptr());

        // add all/connected vertices below margin
        let mut count: i32 = 1;
        let g = g2;

        // g is an ellipsoid: no need for further mesh-specific processing
        if *(*m).geom_dataid.add(g as usize) == -1 {
            return count;
        }

        // init
        let vertdata: *const f32 = (*m).mesh_vert.add(
            3 * *(*m).mesh_vertadr.add(*(*m).geom_dataid.add(g as usize) as usize) as usize);

        // express dir in geom local frame
        let mut locdir: [f64; 3] = [0.0; 3];
        crate::engine::engine_util_blas::mju_mul_mat_t_vec3(
            locdir.as_mut_ptr(), (*d).geom_xmat.add(9 * g as usize),
            ccd_dir.v.as_ptr() as *const f64);

        // inclusion threshold along locdir, relative to geom2 center
        crate::engine::engine_inline::mji_sub3(dif.as_mut_ptr(), pos2, pos1);
        let threshold: f64 = crate::engine::engine_util_blas::mju_dot3(
            normal.as_ptr(), dif.as_ptr()) - margin;

        // no graph data: exhaustive search
        let dataid = *(*m).geom_dataid.add(g as usize) as usize;
        if *(*m).mesh_graphadr.add(dataid) < 0 {
            let nvert = *(*m).mesh_vertnum.add(dataid);
            for i in 0..nvert {
                if count >= MAXPLANEMESH { break; }
                let vdot: f64 = locdir[0] * *vertdata.add(3 * i as usize) as f64
                              + locdir[1] * *vertdata.add(3 * i as usize + 1) as f64
                              + locdir[2] * *vertdata.add(3 * i as usize + 2) as f64;
                if vdot > threshold && i != (*obj_ptr).meshindex {
                    count += addplanemesh(
                        con.add(count as usize), vertdata.add(3 * i as usize),
                        pos1, normal.as_ptr(), pos2, mat2,
                        (*con.add(0)).pos.as_ptr(), *(*m).geom_rbound.add(g2 as usize));
                }
            }
        }
        // use graph data
        else if (*obj_ptr).meshindex >= 0 {
            let graphadr = *(*m).mesh_graphadr.add(dataid) as usize;
            let numvert = *(*m).mesh_graph.add(graphadr);
            let vert_edgeadr = (*m).mesh_graph.add(graphadr + 2);
            let vert_globalid = (*m).mesh_graph.add(graphadr + 2 + numvert as usize);
            let edge_localid = (*m).mesh_graph.add(graphadr + 2 + 2 * numvert as usize);

            let mut i = *vert_edgeadr.add((*obj_ptr).meshindex as usize);
            loop {
                let locid = *edge_localid.add(i as usize);
                if locid < 0 || count >= MAXPLANEMESH { break; }
                let vdot: f64 = locdir[0] * *vertdata.add(3 * *vert_globalid.add(locid as usize) as usize) as f64
                              + locdir[1] * *vertdata.add(3 * *vert_globalid.add(locid as usize) as usize + 1) as f64
                              + locdir[2] * *vertdata.add(3 * *vert_globalid.add(locid as usize) as usize + 2) as f64;
                if vdot > threshold {
                    count += addplanemesh(
                        con.add(count as usize),
                        vertdata.add(3 * *vert_globalid.add(locid as usize) as usize),
                        pos1, normal.as_ptr(), pos2, mat2,
                        (*con.add(0)).pos.as_ptr(), *(*m).geom_rbound.add(g2 as usize));
                }
                i += 1;
            }
        }

        count
    }
}

/// C: mjc_ConvexHField (engine/engine_collision_convex.h:113)
/// Calls: addPrismVert, mjc_fixNormal, mjc_initCCDObj, mjc_penetration, mji_addTo3, mji_copy3, mji_copy9, mji_mulMatTMat3, mji_mulMatVec3, mju_mulMatTVec3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_convex_h_field(m: *const mjModel, d: *mut mjData, con: *mut mjPreContact, g1: i32, g2: i32, margin: f64) -> i32 {
    todo!() // mjc_ConvexHField
}

/// C: mjc_Convex (engine/engine_collision_convex.h:114)
/// Calls: maxContacts, mjc_fixNormal, mjc_initCCDObj, mjc_isDistinctContact, mjc_penetration, mji_axisAngle2Quat, mji_copy3, mji_copy9, mju_makeFrame, mju_min, mju_quat2Mat, mju_rotateFrame, mju_transpose, mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_convex(m: *const mjModel, d: *mut mjData, con: *mut mjPreContact, g1: i32, g2: i32, margin: f64) -> i32 {
    todo!() // mjc_Convex
}

/// C: mjc_ConvexElem (engine/engine_collision_convex.h:117)
/// Calls: mjc_fixNormal, mjc_initCCDObj, mjc_penetration, mjc_setCCDObjFlex
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_convex_elem(m: *const mjModel, d: *mut mjData, con: *mut mjPreContact, g1: i32, f1: i32, e1: i32, v1: i32, f2: i32, e2: i32, margin: f64) -> i32 {
    todo!() // mjc_ConvexElem
}

/// C: mjc_HFieldElem (engine/engine_collision_convex.h:121)
/// Calls: addVert, mjc_initCCDObj, mjc_penetration, mjc_setCCDObjFlex, mji_addTo3, mji_copy3, mji_mulMatTVec3, mji_sub3, mji_zero3, mju_max, mju_min, mju_mulMatVec3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_h_field_elem(m: *const mjModel, d: *mut mjData, con: *mut mjPreContact, g: i32, f: i32, e: i32, margin: f64) -> i32 {
    todo!() // mjc_HFieldElem
}

/// C: mjc_fixNormal (engine/engine_collision_convex.h:125)
/// Calls: mjc_ellipsoidInside, mjc_ellipsoidOutside, mji_copy3, mji_mulMatVec3, mji_scl3, mji_sub3, mju_mulMatTVec3, mju_norm, mju_normalize3, mju_sub3
#[allow(unused_variables, non_snake_case)]
pub fn mjc_fix_normal(m: *const mjModel, d: *const mjData, con: *mut mjPreContact, g1: i32, g2: i32) {
    todo!() // mjc_fixNormal
}

/// C: mjc_setCCDBuffer (engine/engine_collision_convex.h:128)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_set_ccd_buffer(buffer: *mut ()) {
    // SAFETY: storing the raw pointer value as bytes into the mutex-protected CCD_BUFFER
    unsafe {
        let bytes = (buffer as usize).to_ne_bytes();
        let mut guard = CCD_BUFFER.lock().unwrap();
        guard.copy_from_slice(&bytes);
    }
}

