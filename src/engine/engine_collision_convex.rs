//! Port of: engine/engine_collision_convex.c
//! IR hash: 3fb6da908ad9d71c
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
    todo!() // mjc_prism_support
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
    todo!() // mjc_flexSupport
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
    todo!() // mjc_center
}

/// C: mjccd_center (engine/engine_collision_convex.h:100)
/// Calls: mjc_center
#[allow(unused_variables, non_snake_case)]
pub fn mjccd_center(obj: *const (), center: *mut ccd_vec3_t) {
    todo!() // mjccd_center
}

/// C: mjccd_support (engine/engine_collision_convex.h:103)
/// Calls: mjc_prism_support, mji_addScl3, mji_addTo3, mji_addToScl3, mji_copy3, mji_scl3, mju_dot3, mju_message, mju_mulMatTVec3, mju_mulMatVec3, mju_normalize3, mju_sign, mju_warning, mju_zero3
#[allow(unused_variables, non_snake_case)]
pub fn mjccd_support(obj: *const (), dir: *const ccd_vec3_t, vec: *mut ccd_vec3_t) {
    todo!() // mjccd_support
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
    todo!() // mjc_PlaneConvex
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

