//! Port of: engine/engine_ray.c
//! IR hash: 545f394232195ad9
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: ray_map (engine/engine_ray.c:38)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn ray_map(pos: *const f64, mat: *const f64, pnt: *const f64, vec: *const f64, lpnt: *mut f64, lvec: *mut f64) {
    // SAFETY: caller guarantees pos[3], mat[9], pnt[3], vec[3], lpnt[3], lvec[3] are valid
    unsafe {
        let dif0: f64 = *pnt.add(0) - *pos.add(0);
        let dif1: f64 = *pnt.add(1) - *pos.add(1);
        let dif2: f64 = *pnt.add(2) - *pos.add(2);
        *lpnt.add(0) = *mat.add(0) * dif0 + *mat.add(3) * dif1 + *mat.add(6) * dif2;
        *lpnt.add(1) = *mat.add(1) * dif0 + *mat.add(4) * dif1 + *mat.add(7) * dif2;
        *lpnt.add(2) = *mat.add(2) * dif0 + *mat.add(5) * dif1 + *mat.add(8) * dif2;
        *lvec.add(0) = *mat.add(0) * *vec.add(0) + *mat.add(3) * *vec.add(1) + *mat.add(6) * *vec.add(2);
        *lvec.add(1) = *mat.add(1) * *vec.add(0) + *mat.add(4) * *vec.add(1) + *mat.add(7) * *vec.add(2);
        *lvec.add(2) = *mat.add(2) * *vec.add(0) + *mat.add(5) * *vec.add(1) + *mat.add(8) * *vec.add(2);
    }
}

/// C: longitude (engine/engine_ray.c:56)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn longitude(vec: *const f64) -> f64 {
    // WARNING: signature changed — verify body
    // Previous params: (vec : * const f64)
    // Previous return: f64
    todo ! ()
}

/// C: latitude (engine/engine_ray.c:62)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn latitude(vec: *const f64) -> f64 {
    // WARNING: signature changed — verify body
    // Previous params: (vec : * const f64)
    // Previous return: f64
    todo ! ()
}

/// C: ray_eliminate (engine/engine_ray.c:68)
#[allow(unused_variables, non_snake_case)]
pub fn ray_eliminate(m: *const mjModel, d: *const mjData, geomid: i32, geomgroup: *const u8, flg_static: mjtBool, bodyexclude: i32) -> i32 {
    extern "C" { fn ray_eliminate_impl(m: *const mjModel, d: *const mjData, geomid: i32, geomgroup: *const u8, flg_static: mjtBool, bodyexclude: i32) -> i32; }
    // SAFETY: delegates to C implementation; caller guarantees all pointers are valid
    unsafe { ray_eliminate_impl(m, d, geomid, geomgroup, flg_static, bodyexclude) }
}

/// C: ray_quad (engine/engine_ray.c:103)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn ray_quad(a: f64, b: f64, c: f64, x: *mut f64) -> f64 {
    const MJMINVAL: f64 = 1e-15;
    let mut det: f64 = b * b - a * c;
    if det < 0.0 || a < MJMINVAL {
        // SAFETY: caller guarantees x points to array of at least 2 f64
        unsafe {
            *x.add(0) = -1.0;
            *x.add(1) = -1.0;
        }
        return -1.0;
    }
    det = det.sqrt();
    // SAFETY: caller guarantees x points to array of at least 2 f64
    unsafe {
        *x.add(0) = (-b - det) / a;
        *x.add(1) = (-b + det) / a;
        if *x.add(0) >= 0.0 {
            return *x.add(0);
        } else if *x.add(1) >= 0.0 {
            return *x.add(1);
        }
    }
    -1.0
}

/// C: ray_plane (engine/engine_ray.c:204)
/// Calls: mju_zero3, ray_map
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn ray_plane(pos: *const f64, mat: *const f64, size: *const f64, pnt: *const f64, vec: *const f64, normal: *mut f64) -> f64 {
    extern "C" { fn ray_plane_impl(pos: *const f64, mat: *const f64, size: *const f64, pnt: *const f64, vec: *const f64, normal: *mut f64) -> f64; }
    // SAFETY: delegates to C implementation, pointers valid per caller contract
    unsafe { ray_plane_impl(pos, mat, size, pnt, vec, normal) }
}

/// C: ray_sphere (engine/engine_ray.c:242)
/// Calls: mju_addScl3, mju_normalize3, mju_sub3, mju_zero3, ray_quad
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn ray_sphere(pos: *const f64, mat: *const f64, dist_sqr: f64, pnt: *const f64, vec: *const f64, normal: *mut f64) -> f64 {
    extern "C" { fn ray_sphere_impl(pos: *const f64, mat: *const f64, dist_sqr: f64, pnt: *const f64, vec: *const f64, normal: *mut f64) -> f64; }
    // SAFETY: delegates to C implementation, pointers valid per caller contract
    unsafe { ray_sphere_impl(pos, mat, dist_sqr, pnt, vec, normal) }
}

/// C: ray_capsule (engine/engine_ray.c:272)
/// Calls: mju_mulMatVec3, mju_normalize3, mju_zero3, ray_map, ray_quad, ray_sphere
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn ray_capsule(pos: *const f64, mat: *const f64, size: *const f64, pnt: *const f64, vec: *const f64, normal: *mut f64) -> f64 {
    extern "C" { fn ray_capsule_impl(pos: *const f64, mat: *const f64, size: *const f64, pnt: *const f64, vec: *const f64, normal: *mut f64) -> f64; }
    // SAFETY: delegates to C implementation, pointers valid per caller contract
    unsafe { ray_capsule_impl(pos, mat, size, pnt, vec, normal) }
}

/// C: ray_ellipsoid (engine/engine_ray.c:358)
/// Calls: mju_addScl3, mju_mulMatVec3, mju_normalize3, mju_zero3, ray_map, ray_quad
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn ray_ellipsoid(pos: *const f64, mat: *const f64, size: *const f64, pnt: *const f64, vec: *const f64, normal: *mut f64) -> f64 {
    extern "C" { fn ray_ellipsoid_impl(pos: *const f64, mat: *const f64, size: *const f64, pnt: *const f64, vec: *const f64, normal: *mut f64) -> f64; }
    // SAFETY: delegates to C implementation, pointers valid per caller contract
    unsafe { ray_ellipsoid_impl(pos, mat, size, pnt, vec, normal) }
}

/// C: ray_cylinder (engine/engine_ray.c:401)
/// Calls: mju_mulMatVec3, mju_normalize3, mju_zero3, ray_map, ray_quad, ray_sphere
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn ray_cylinder(pos: *const f64, mat: *const f64, size: *const f64, pnt: *const f64, vec: *const f64, normal: *mut f64) -> f64 {
    extern "C" { fn ray_cylinder_impl(pos: *const f64, mat: *const f64, size: *const f64, pnt: *const f64, vec: *const f64, normal: *mut f64) -> f64; }
    // SAFETY: delegates to C implementation, pointers valid per caller contract
    unsafe { ray_cylinder_impl(pos, mat, size, pnt, vec, normal) }
}

/// C: ray_box (engine/engine_ray.c:490)
/// Calls: mju_mulMatVec3, mju_zero3, ray_map, ray_sphere
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn ray_box(pos: *const f64, mat: *const f64, size: *const f64, pnt: *const f64, vec: *const f64, all: *mut f64, normal: *mut f64) -> f64 {
    // WARNING: signature changed — verify body
    // Previous params: (pos : * const f64, mat : * const f64, size : * const f64, pnt : * const f64, vec : * const f64, all : * mut f64, normal : * mut f64)
    // Previous return: f64
    todo ! ()
}

/// C: mju_raySlab (engine/engine_ray.c:743)
/// Calls: ray_map
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_ray_slab(aabb: *const f64, xpos: *const f64, xmat: *const f64, pnt: *const f64, vec: *const f64) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (aabb : * const f64, xpos : * const f64, xmat : * const f64, pnt : * const f64, vec : * const f64)
    // Previous return: i32
    todo ! ()
}

/// C: mju_rayTree (engine/engine_ray.c:771)
/// Calls: mju_addScl3, mju_copy3, mju_cross, mju_dot3, mju_message, mju_mulMatVec3, mju_normalize3, mju_raySlab, mju_zero3, ray_map, ray_triangle
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_ray_tree(m: *const mjModel, d: *const mjData, id: i32, pnt: *const f64, vec: *const f64, normal: *mut f64) -> f64 {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * const mjData, id : i32, pnt : * const f64, vec : * const f64, normal : * mut f64)
    // Previous return: f64
    todo ! ()
}

/// C: mj_raySdf (engine/engine_ray.c:885)
/// Calls: mjc_distance, mjc_getSDF, mjc_gradient, mju_addScl3, mju_mulMatVec3, mju_normalize3, mju_zero3, ray_box, ray_map
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_ray_sdf(m: *const mjModel, d: *const mjData, g: i32, pnt: *const f64, vec: *const f64, normal: *mut f64) -> f64 {
    extern "C" { fn mj_raySdf_impl(m: *const mjModel, d: *const mjData, g: i32, pnt: *const f64, vec: *const f64, normal: *mut f64) -> f64; }
    // SAFETY: delegates to C implementation, pointers valid per caller contract
    unsafe { mj_raySdf_impl(m, d, g, pnt, vec, normal) }
}

/// C: point_in_box (engine/engine_ray.c:1283)
/// Calls: mju_mulMatTVec3, mju_sub3, mju_subFrom3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn point_in_box(aabb: *const f64, xpos: *const f64, xmat: *const f64, pnt: *const f64) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (aabb : * const f64, xpos : * const f64, xmat : * const f64, pnt : * const f64)
    // Previous return: i32
    todo ! ()
}

/// C: mju_singleRay (engine/engine_ray.c:1457)
/// Calls: latitude, longitude, mj_rayHfield, mj_rayMesh, mj_raySdf, mju_add3, mju_copy3, mju_rayGeom, mju_zero3, ray_sphere
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_single_ray(m: *const mjModel, d: *mut mjData, pnt: *const f64, vec: *const f64, ray_eliminate: *mut i32, geom_ba: *mut f64, geomid: [i32; 1], normal: *mut f64) -> f64 {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, pnt : * const f64, vec : * const f64, ray_eliminate : * mut i32, geom_ba : * mut f64, geomid : [i32 ; 1], normal : * mut f64)
    // Previous return: f64
    todo ! ()
}

/// C: mju_multiRayPrepare (engine/engine_ray.h:26)
/// Calls: latitude, longitude, mju_addTo3, mju_copy, mju_dist3, mju_max, mju_message, mju_min, mju_mulMatVec3, mju_sub3, point_in_box, ray_eliminate
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_multi_ray_prepare(m: *const mjModel, d: *const mjData, pnt: *const f64, ray_xmat: *const f64, geomgroup: *const u8, flg_static: mjtBool, bodyexclude: i32, cutoff: f64, geom_ba: *mut f64, geom_eliminate: *mut i32) {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * const mjData, pnt : * const f64, ray_xmat : * const f64, geomgroup : * const u8, flg_static : mjtBool, bodyexclude : i32, cutoff : f64, geom_ba : * mut f64, geom_eliminate : * mut i32)
    // Previous return: ()
    todo ! ()
}

/// C: mj_multiRay (engine/engine_ray.h:34)
/// Calls: mj_freeStack, mj_markStack, mj_stackAllocInfo, mju_dot3, mju_multiRayPrepare, mju_singleRay
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_multi_ray(m: *const mjModel, d: *mut mjData, pnt: *const f64, vec: *const f64, geomgroup: *const u8, flg_static: mjtBool, bodyexclude: i32, geomid: *mut i32, dist: *mut f64, normal: *mut f64, nray: i32, cutoff: f64) {

    extern "C" { fn mj_multiRay_impl(m: *const mjModel, d: *mut mjData, pnt: *const f64, vec: *const f64, geomgroup: *const u8, flg_static: mjtBool, bodyexclude: i32, geomid: *mut i32, dist: *mut f64, normal: *mut f64, nray: i32, cutoff: f64); }
    // SAFETY: delegates to C implementation
    unsafe { mj_multiRay_impl(m, d, pnt, vec, geomgroup, flg_static, bodyexclude, geomid, dist, normal, nray, cutoff) }
}

/// C: mj_ray (engine/engine_ray.h:42)
/// Calls: mj_rayHfield, mj_rayMesh, mj_raySdf, mju_copy3, mju_message, mju_norm3, mju_rayGeom, mju_zero3, ray_eliminate
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_ray(m: *const mjModel, d: *const mjData, pnt: *const f64, vec: *const f64, geomgroup: *const u8, flg_static: mjtBool, bodyexclude: i32, geomid: [i32; 1], normal: *mut f64) -> f64 {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * const mjData, pnt : * const f64, vec : * const f64, geomgroup : * const u8, flg_static : mjtBool, bodyexclude : i32, geomid : [i32 ; 1], normal : * mut f64)
    // Previous return: f64
    todo ! ()
}

/// C: mj_rayHfield (engine/engine_ray.h:47)
/// Calls: mju_addScl3, mju_copy3, mju_cross, mju_dot3, mju_message, mju_mulMatTVec3, mju_mulMatVec3, mju_normalize3, mju_round, mju_zero3, ray_box, ray_map, ray_triangle
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_ray_hfield(m: *const mjModel, d: *const mjData, geomid: i32, pnt: *const f64, vec: *const f64, normal: *mut f64) -> f64 {
    extern "C" { fn mj_rayHfield_impl(m: *const mjModel, d: *const mjData, geomid: i32, pnt: *const f64, vec: *const f64, normal: *mut f64) -> f64; }
    // SAFETY: delegates to C implementation, pointers valid per caller contract
    unsafe { mj_rayHfield_impl(m, d, geomid, pnt, vec, normal) }
}

/// C: ray_triangle (engine/engine_ray.h:51)
/// Calls: mju_copy3, mju_cross, mju_dot3, mju_normalize3, mju_sub3, mju_zero3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn ray_triangle(v: *const [f64; 3], lpnt: *const f64, lvec: *const f64, b0: *const f64, b1: *const f64, normal: *mut f64) -> f64 {
    use crate::engine::engine_util_blas::{mju_zero3, mju_dot3, mju_sub3, mju_copy3, mju_normalize3};
    use crate::engine::engine_util_spatial::mju_cross;
    const MJMINVAL: f64 = 1e-15;

    unsafe {
        // clear normal if given
        if !normal.is_null() {
            mju_zero3(normal);
        }

        // dif = v[i] - lpnt
        let mut dif: [[f64; 3]; 3] = [[0.0; 3]; 3];
        for i in 0..3 {
            for j in 0..3 {
                dif[i][j] = (*v.add(i))[j] - *lpnt.add(j);
            }
        }

        // project difference vectors in normal plane
        let mut planar: [[f64; 2]; 3] = [[0.0; 2]; 3];
        for i in 0..3 {
            planar[i][0] = mju_dot3(b0, dif[i].as_ptr());
            planar[i][1] = mju_dot3(b1, dif[i].as_ptr());
        }

        // reject if on the same side of any coordinate axis
        if (planar[0][0] > 0.0 && planar[1][0] > 0.0 && planar[2][0] > 0.0) ||
           (planar[0][0] < 0.0 && planar[1][0] < 0.0 && planar[2][0] < 0.0) ||
           (planar[0][1] > 0.0 && planar[1][1] > 0.0 && planar[2][1] > 0.0) ||
           (planar[0][1] < 0.0 && planar[1][1] < 0.0 && planar[2][1] < 0.0) {
            return -1.0;
        }

        // determine if origin is inside planar projection of triangle
        // A = (p0-p2, p1-p2), b = -p2, solve A*t = b
        let A: [f64; 4] = [
            planar[0][0] - planar[2][0], planar[1][0] - planar[2][0],
            planar[0][1] - planar[2][1], planar[1][1] - planar[2][1],
        ];
        let b: [f64; 2] = [-planar[2][0], -planar[2][1]];
        let det = A[0] * A[3] - A[1] * A[2];
        if det.abs() < MJMINVAL {
            return -1.0;
        }
        let t0 = (A[3] * b[0] - A[1] * b[1]) / det;
        let t1 = (-A[2] * b[0] + A[0] * b[1]) / det;

        // check if outside
        if t0 < 0.0 || t1 < 0.0 || t0 + t1 > 1.0 {
            return -1.0;
        }

        // intersect ray with plane of triangle
        mju_sub3(dif[0].as_mut_ptr(), (*v.add(0)).as_ptr(), (*v.add(2)).as_ptr());  // v0-v2
        mju_sub3(dif[1].as_mut_ptr(), (*v.add(1)).as_ptr(), (*v.add(2)).as_ptr());  // v1-v2
        mju_sub3(dif[2].as_mut_ptr(), lpnt, (*v.add(2)).as_ptr());                  // lp-v2
        let mut nrm: [f64; 3] = [0.0; 3];
        mju_cross(nrm.as_mut_ptr(), dif[0].as_ptr(), dif[1].as_ptr());  // normal to triangle plane
        let denom = mju_dot3(lvec, nrm.as_ptr());
        if denom.abs() < MJMINVAL {
            return -1.0;
        }

        // compute distance
        let x = -mju_dot3(dif[2].as_ptr(), nrm.as_ptr()) / denom;

        // compute normal if given
        if !normal.is_null() {
            mju_normalize3(nrm.as_mut_ptr());
            mju_copy3(normal, nrm.as_ptr());
        }

        x
    }
}

/// C: mj_rayMesh (engine/engine_ray.h:55)
/// Calls: mju_message, mju_rayTree, mju_zero3, ray_box
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_ray_mesh(m: *const mjModel, d: *const mjData, geomid: i32, pnt: *const f64, vec: *const f64, normal: *mut f64) -> f64 {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * const mjData, geomid : i32, pnt : * const f64, vec : * const f64, normal : * mut f64)
    // Previous return: f64
    todo ! ()
}

/// C: mju_rayGeom (engine/engine_ray.h:59)
/// Calls: mju_message, ray_box, ray_capsule, ray_cylinder, ray_ellipsoid, ray_plane, ray_sphere
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_ray_geom(pos: *const f64, mat: *const f64, size: *const f64, pnt: *const f64, vec: *const f64, geomtype: i32, normal: *mut f64) -> f64 {
    // WARNING: signature changed — verify body
    // Previous params: (pos : * const f64, mat : * const f64, size : * const f64, pnt : * const f64, vec : * const f64, geomtype : i32, normal : * mut f64)
    // Previous return: f64
    todo ! ()
}

/// C: mj_rayFlex (engine/engine_ray.h:64)
/// Calls: mju_add3, mju_addScl3, mju_copy3, mju_cross, mju_dist3, mju_dot3, mju_normalize3, mju_quat2Mat, mju_quatZ2Vec, mju_rayGeom, mju_scl3, mju_zero3, ray_box, ray_triangle
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_ray_flex(m: *const mjModel, d: *const mjData, flex_layer: i32, flg_vert: mjtBool, flg_edge: mjtBool, flg_face: mjtBool, flg_skin: mjtBool, flexid: i32, pnt: *const f64, vec: *const f64, vertid: [i32; 1], normal: *mut f64) -> f64 {

    extern "C" { fn mj_rayFlex_impl(m: *const mjModel, d: *const mjData, flex_layer: i32, flg_vert: mjtBool, flg_edge: mjtBool, flg_face: mjtBool, flg_skin: mjtBool, flexid: i32, pnt: *const f64, vec: *const f64, vertid: [i32; 1], normal: *mut f64) -> f64; }
    // SAFETY: delegates to C implementation
    unsafe { mj_rayFlex_impl(m, d, flex_layer, flg_vert, flg_edge, flg_face, flg_skin, flexid, pnt, vec, vertid, normal) }
}

/// C: mju_raySkin (engine/engine_ray.h:70)
/// Calls: mju_addScl3, mju_cross, mju_dist3, mju_dot3, mju_normalize3, ray_box, ray_triangle
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_ray_skin(nface: i32, nvert: i32, face: *const i32, vert: *const f32, pnt: *const f64, vec: *const f64, vertid: [i32; 1]) -> f64 {

    extern "C" { fn mju_raySkin_impl(nface: i32, nvert: i32, face: *const i32, vert: *const f32, pnt: *const f64, vec: *const f64, vertid: [i32; 1]) -> f64; }
    // SAFETY: delegates to C implementation
    unsafe { mju_raySkin_impl(nface, nvert, face, vert, pnt, vec, vertid) }
}

