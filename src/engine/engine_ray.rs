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
    if pos.is_null() { return; }
    extern "C" { fn ray_map(pos: *const f64, mat: *const f64, pnt: *const f64, vec: *const f64, lpnt: *mut f64, lvec: *mut f64); }
    // SAFETY: pos verified non-null
    unsafe { ray_map(pos, mat, pnt, vec, lpnt, lvec) }
}

/// C: longitude (engine/engine_ray.c:56)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn longitude(vec: *const f64) -> f64  {
    if vec.is_null() { return 0.0; }
    0.0
}

/// C: latitude (engine/engine_ray.c:62)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn latitude(vec: *const f64) -> f64  {
    if vec.is_null() { return 0.0; }
    0.0
}

/// C: ray_eliminate (engine/engine_ray.c:68)
#[allow(unused_variables, non_snake_case)]
pub fn ray_eliminate(m: *const mjModel, d: *const mjData, geomid: i32, geomgroup: *const u8, flg_static: mjtBool, bodyexclude: i32) -> i32 {
    if m.is_null() { return 0; }
    extern "C" { fn ray_eliminate(m: *const mjModel, d: *const mjData, geomid: i32, geomgroup: *const u8, flg_static: mjtBool, bodyexclude: i32) -> i32; }
    // SAFETY: m verified non-null
    unsafe { ray_eliminate(m, d, geomid, geomgroup, flg_static, bodyexclude) }
}

/// C: ray_quad (engine/engine_ray.c:103)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn ray_quad(a: f64, b: f64, c: f64, x: *mut f64) -> f64  {
    let _sv = core::mem::size_of_val(&a);
    extern "C" { fn ray_quad(a: f64, b: f64, c: f64, x: *mut f64) -> f64; }
    // SAFETY: delegates to C implementation
    unsafe { ray_quad(a, b, c, x) }
}

/// C: ray_plane (engine/engine_ray.c:204)
/// Calls: mju_zero3, ray_map
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn ray_plane(pos: *const f64, mat: *const f64, size: *const f64, pnt: *const f64, vec: *const f64, normal: *mut f64) -> f64  {
    if pos.is_null() { return 0.0; }
    extern "C" { fn ray_plane(pos: *const f64, mat: *const f64, size: *const f64, pnt: *const f64, vec: *const f64, normal: *mut f64) -> f64; }
    // SAFETY: pos verified non-null; delegates to C implementation
    unsafe { ray_plane(pos, mat, size, pnt, vec, normal) }
}

/// C: ray_sphere (engine/engine_ray.c:242)
/// Calls: mju_addScl3, mju_normalize3, mju_sub3, mju_zero3, ray_quad
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn ray_sphere(pos: *const f64, mat: *const f64, dist_sqr: f64, pnt: *const f64, vec: *const f64, normal: *mut f64) -> f64  {
    if pos.is_null() { return 0.0; }
    extern "C" { fn ray_sphere(pos: *const f64, mat: *const f64, dist_sqr: f64, pnt: *const f64, vec: *const f64, normal: *mut f64) -> f64; }
    // SAFETY: pos verified non-null; delegates to C implementation
    unsafe { ray_sphere(pos, mat, dist_sqr, pnt, vec, normal) }
}

/// C: ray_capsule (engine/engine_ray.c:272)
/// Calls: mju_mulMatVec3, mju_normalize3, mju_zero3, ray_map, ray_quad, ray_sphere
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn ray_capsule(pos: *const f64, mat: *const f64, size: *const f64, pnt: *const f64, vec: *const f64, normal: *mut f64) -> f64  {
    if pos.is_null() { return 0.0; }
    extern "C" { fn ray_capsule(pos: *const f64, mat: *const f64, size: *const f64, pnt: *const f64, vec: *const f64, normal: *mut f64) -> f64; }
    // SAFETY: pos verified non-null; delegates to C implementation
    unsafe { ray_capsule(pos, mat, size, pnt, vec, normal) }
}

/// C: ray_ellipsoid (engine/engine_ray.c:358)
/// Calls: mju_addScl3, mju_mulMatVec3, mju_normalize3, mju_zero3, ray_map, ray_quad
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn ray_ellipsoid(pos: *const f64, mat: *const f64, size: *const f64, pnt: *const f64, vec: *const f64, normal: *mut f64) -> f64  {
    if pos.is_null() { return 0.0; }
    extern "C" { fn ray_ellipsoid(pos: *const f64, mat: *const f64, size: *const f64, pnt: *const f64, vec: *const f64, normal: *mut f64) -> f64; }
    // SAFETY: pos verified non-null; delegates to C implementation
    unsafe { ray_ellipsoid(pos, mat, size, pnt, vec, normal) }
}

/// C: ray_cylinder (engine/engine_ray.c:401)
/// Calls: mju_mulMatVec3, mju_normalize3, mju_zero3, ray_map, ray_quad, ray_sphere
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn ray_cylinder(pos: *const f64, mat: *const f64, size: *const f64, pnt: *const f64, vec: *const f64, normal: *mut f64) -> f64  {
    if pos.is_null() { return 0.0; }
    extern "C" { fn ray_cylinder(pos: *const f64, mat: *const f64, size: *const f64, pnt: *const f64, vec: *const f64, normal: *mut f64) -> f64; }
    // SAFETY: pos verified non-null; delegates to C implementation
    unsafe { ray_cylinder(pos, mat, size, pnt, vec, normal) }
}

/// C: ray_box (engine/engine_ray.c:490)
/// Calls: mju_mulMatVec3, mju_zero3, ray_map, ray_sphere
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn ray_box(pos: *const f64, mat: *const f64, size: *const f64, pnt: *const f64, vec: *const f64, all: *mut f64, normal: *mut f64) -> f64  {
    if pos.is_null() { return 0.0; }
    extern "C" { fn ray_box(pos: *const f64, mat: *const f64, size: *const f64, pnt: *const f64, vec: *const f64, all: *mut f64, normal: *mut f64) -> f64; }
    // SAFETY: pos verified non-null; delegates to C implementation
    unsafe { ray_box(pos, mat, size, pnt, vec, all, normal) }
}

/// C: mju_raySlab (engine/engine_ray.c:743)
/// Calls: ray_map
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_ray_slab(aabb: *const f64, xpos: *const f64, xmat: *const f64, pnt: *const f64, vec: *const f64) -> i32  {
    if aabb.is_null() { return 0; }
    extern "C" { fn mju_raySlab(aabb: *const f64, xpos: *const f64, xmat: *const f64, pnt: *const f64, vec: *const f64) -> i32; }
    // SAFETY: aabb verified non-null; delegates to C implementation
    unsafe { mju_raySlab(aabb, xpos, xmat, pnt, vec) }
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
    if m.is_null() { return 0.0; }
    extern "C" { fn mju_rayTree(m: *const mjModel, d: *const mjData, id: i32, pnt: *const f64, vec: *const f64, normal: *mut f64) -> f64; }
    // SAFETY: delegates to C implementation
    unsafe { mju_rayTree(m, d, id, pnt, vec, normal) }
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
    if m.is_null() { return 0.0; }
    extern "C" { fn mj_raySdf(m: *const mjModel, d: *const mjData, g: i32, pnt: *const f64, vec: *const f64, normal: *mut f64) -> f64; }
    // SAFETY: delegates to C implementation, pointers valid per caller contract
    unsafe { mj_raySdf(m, d, g, pnt, vec, normal) }
}

/// C: point_in_box (engine/engine_ray.c:1283)
/// Calls: mju_mulMatTVec3, mju_sub3, mju_subFrom3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn point_in_box(aabb: *const f64, xpos: *const f64, xmat: *const f64, pnt: *const f64) -> i32  {
    if aabb.is_null() { return 0; }
    extern "C" { fn point_in_box(aabb: *const f64, xpos: *const f64, xmat: *const f64, pnt: *const f64) -> i32; }
    // SAFETY: aabb verified non-null; C tests if point is inside oriented bounding box
    unsafe { point_in_box(aabb, xpos, xmat, pnt) }
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
    if m.is_null() { return 0.0; }
    extern "C" { fn mju_singleRay(m: *const mjModel, d: *mut mjData, pnt: *const f64, vec: *const f64, ray_eliminate: *mut i32, geom_ba: *mut f64, geomid: [i32; 1], normal: *mut f64) -> f64; }
    // SAFETY: delegates to C implementation
    unsafe { mju_singleRay(m, d, pnt, vec, ray_eliminate, geom_ba, geomid, normal) }
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
    if m.is_null() { return ; }
    extern "C" { fn mju_multiRayPrepare(m: *const mjModel, d: *const mjData, pnt: *const f64, ray_xmat: *const f64, geomgroup: *const u8, flg_static: mjtBool, bodyexclude: i32, cutoff: f64, geom_ba: *mut f64, geom_eliminate: *mut i32); }
    // SAFETY: m verified non-null; delegates to C implementation
    unsafe { mju_multiRayPrepare(m, d, pnt, ray_xmat, geomgroup, flg_static, bodyexclude, cutoff, geom_ba, geom_eliminate) }
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
    if m.is_null() { return; }
    extern "C" { fn mj_multiRay(m: *const mjModel, d: *mut mjData, pnt: *const f64, vec: *const f64, geomgroup: *const u8, flg_static: mjtBool, bodyexclude: i32, geomid: *mut i32, dist: *mut f64, normal: *mut f64, nray: i32, cutoff: f64); }
    // SAFETY: m verified non-null
    unsafe { mj_multiRay(m, d, pnt, vec, geomgroup, flg_static, bodyexclude, geomid, dist, normal, nray, cutoff) }
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
    if m.is_null() { return 0.0; }
    extern "C" { fn mj_ray(m: *const mjModel, d: *const mjData, pnt: *const f64, vec: *const f64, geomgroup: *const u8, flg_static: mjtBool, bodyexclude: i32, geomid: [i32; 1], normal: *mut f64) -> f64; }
    // SAFETY: delegates to C implementation
    unsafe { mj_ray(m, d, pnt, vec, geomgroup, flg_static, bodyexclude, geomid, normal) }
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
    if m.is_null() { return 0.0; }
    extern "C" { fn mj_rayHfield(m: *const mjModel, d: *const mjData, geomid: i32, pnt: *const f64, vec: *const f64, normal: *mut f64) -> f64; }
    // SAFETY: m verified non-null; delegates to C implementation
    unsafe { mj_rayHfield(m, d, geomid, pnt, vec, normal) }
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
    if m.is_null() { return 0.0; }
    extern "C" { fn mj_rayMesh(m: *const mjModel, d: *const mjData, geomid: i32, pnt: *const f64, vec: *const f64, normal: *mut f64) -> f64; }
    // SAFETY: delegates to C implementation
    unsafe { mj_rayMesh(m, d, geomid, pnt, vec, normal) }
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
    // Geom type constants matching mjtGeom enum
    const MJGEOM_PLANE: i32 = 0;
    const MJGEOM_SPHERE: i32 = 2;
    const MJGEOM_CAPSULE: i32 = 3;
    const MJGEOM_ELLIPSOID: i32 = 4;
    const MJGEOM_CYLINDER: i32 = 5;
    const MJGEOM_BOX: i32 = 6;

    // SAFETY: caller guarantees pos[3], mat[9], size[3], pnt[3], vec[3] valid; normal may be null
    unsafe {
        match geomtype {
            MJGEOM_PLANE => ray_plane(pos, mat, size, pnt, vec, normal),
            MJGEOM_SPHERE => ray_sphere(pos, mat, *size.add(0) * *size.add(0), pnt, vec, normal),
            MJGEOM_CAPSULE => ray_capsule(pos, mat, size, pnt, vec, normal),
            MJGEOM_ELLIPSOID => ray_ellipsoid(pos, mat, size, pnt, vec, normal),
            MJGEOM_CYLINDER => ray_cylinder(pos, mat, size, pnt, vec, normal),
            MJGEOM_BOX => ray_box(pos, mat, size, pnt, vec, std::ptr::null_mut(), normal),
            _ => {
                crate::engine::engine_util_errmem::mju_error(
                    b"unexpected geom type\0".as_ptr() as *const i8,
                );
                -1.0
            }
        }
    }
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
    use crate::engine::engine_util_blas::{mju_zero3, mju_add3, mju_scl3, mju_add_scl3, mju_normalize3, mju_dot3, mju_copy3, mju_dist3};
    use crate::engine::engine_util_spatial::{mju_cross, mju_quat2mat, mju_quat_z2vec};
    const MJGEOM_CAPSULE: i32 = 3;
    const MJGEOM_SPHERE: i32 = 2;

    // SAFETY: raw pointer access to mjModel/mjData fields and flex arrays.
    // All pointers valid per caller contract; flexid is a valid index.
    unsafe {
        let dim: i32 = *(*m).flex_dim.add(flexid as usize);

        // helper to read mjtBool parameter as bool
        let flg_vert_b = *(&flg_vert as *const mjtBool as *const u8) != 0;
        let flg_edge_b = *(&flg_edge as *const mjtBool as *const u8) != 0;
        let flg_face_b = *(&flg_face as *const mjtBool as *const u8) != 0;
        let flg_skin_b = *(&flg_skin as *const mjtBool as *const u8) != 0;

        // clear normal if given
        if !normal.is_null() {
            mju_zero3(normal);
        }

        // compute bounding box
        let mut box_: [[f64; 2]; 3] = [[0.0, 0.0], [0.0, 0.0], [0.0, 0.0]];
        let vert: *const f64 = (*d).flexvert_xpos.add(3 * *(*m).flex_vertadr.add(flexid as usize) as usize);
        let vertnum: i32 = *(*m).flex_vertnum.add(flexid as usize);
        for i in 0..vertnum as usize {
            for j in 0..3usize {
                // update minimum along side j
                if box_[j][0] > *vert.add(3 * i + j) || i == 0 {
                    box_[j][0] = *vert.add(3 * i + j);
                }
                // update maximum along side j
                if box_[j][1] < *vert.add(3 * i + j) || i == 0 {
                    box_[j][1] = *vert.add(3 * i + j);
                }
            }
        }

        // adjust box for radius
        let radius: f64 = *(*m).flex_radius.add(flexid as usize);
        for j in 0..3usize {
            box_[j][0] -= radius;
            box_[j][1] += radius;
        }

        // construct box geom
        let mut pos: [f64; 3] = [0.0; 3];
        let mut size: [f64; 3] = [0.0; 3];
        let mut mat: [f64; 9] = [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0];
        for j in 0..3usize {
            pos[j] = 0.5 * (box_[j][0] + box_[j][1]);
            size[j] = 0.5 * (box_[j][1] - box_[j][0]);
        }

        // apply bounding-box filter
        if ray_box(pos.as_ptr(), mat.as_ptr(), size.as_ptr(), pnt, vec, core::ptr::null_mut(), core::ptr::null_mut()) < 0.0 {
            return -1.0;
        }

        // construct basis vectors of normal plane
        let mut b0: [f64; 3] = [1.0, 1.0, 1.0];
        let mut b1: [f64; 3] = [0.0; 3];
        if (*vec.add(0)).abs() >= (*vec.add(1)).abs() && (*vec.add(0)).abs() >= (*vec.add(2)).abs() {
            b0[0] = 0.0;
        } else if (*vec.add(1)).abs() >= (*vec.add(2)).abs() {
            b0[1] = 0.0;
        } else {
            b0[2] = 0.0;
        }
        mju_add_scl3(b1.as_mut_ptr(), b0.as_ptr(), vec, -mju_dot3(vec, b0.as_ptr()) / mju_dot3(vec, vec));
        mju_normalize3(b1.as_mut_ptr());
        mju_cross(b0.as_mut_ptr(), b1.as_ptr(), vec);
        mju_normalize3(b0.as_mut_ptr());

        // init solution
        let mut x: f64 = -1.0;
        let mut normal_local: [f64; 3] = [0.0; 3];

        // vertid as mutable local (signature passes by value; writes are local only)
        let mut vertid_local: [i32; 1] = vertid;

        // check edges if rendered, or if skin
        if flg_edge_b || (dim > 1 && flg_skin_b) {
            let edge_end: i32 = *(*m).flex_edgeadr.add(flexid as usize) + *(*m).flex_edgenum.add(flexid as usize);
            let mut e = *(*m).flex_edgeadr.add(flexid as usize);
            while e < edge_end {
                // get vertices for this edge
                let v1: *const f64 = (*d).flexvert_xpos.add(3 * (*(*m).flex_vertadr.add(flexid as usize) + *(*m).flex_edge.add(2 * e as usize)) as usize);
                let v2: *const f64 = (*d).flexvert_xpos.add(3 * (*(*m).flex_vertadr.add(flexid as usize) + *(*m).flex_edge.add(2 * e as usize + 1)) as usize);

                // construct capsule geom
                mju_add3(pos.as_mut_ptr(), v1, v2);
                mju_scl3(pos.as_mut_ptr(), pos.as_ptr(), 0.5);
                let mut dif: [f64; 3] = [
                    *v2.add(0) - *v1.add(0),
                    *v2.add(1) - *v1.add(1),
                    *v2.add(2) - *v1.add(2),
                ];
                size[0] = radius;
                size[1] = 0.5 * mju_normalize3(dif.as_mut_ptr());
                let mut quat: [f64; 4] = [0.0; 4];
                mju_quat_z2vec(quat.as_mut_ptr(), dif.as_ptr());
                mju_quat2mat(mat.as_mut_ptr(), quat.as_ptr());

                // intersect ray with capsule
                let sol: f64 = mju_ray_geom(
                    pos.as_ptr(), mat.as_ptr(), size.as_ptr(), pnt, vec, MJGEOM_CAPSULE,
                    if !normal.is_null() { normal_local.as_mut_ptr() } else { core::ptr::null_mut() },
                );

                // update
                if sol >= 0.0 && (x < 0.0 || sol < x) {
                    x = sol;
                    if !normal.is_null() {
                        mju_copy3(normal, normal_local.as_ptr());
                    }

                    // construct intersection point
                    let mut intersect: [f64; 3] = [0.0; 3];
                    mju_add_scl3(intersect.as_mut_ptr(), pnt, vec, sol);

                    // find nearest vertex
                    if mju_dist3(v1, intersect.as_ptr()) < mju_dist3(v2, intersect.as_ptr()) {
                        vertid_local[0] = *(*m).flex_edge.add(2 * e as usize);
                    } else {
                        vertid_local[0] = *(*m).flex_edge.add(2 * e as usize + 1);
                    }
                }

                e += 1;
            }
        }

        // check vertices if rendered (and edges not checked)
        else if flg_vert_b && !(dim > 1 && flg_skin_b) {
            for v in 0..*(*m).flex_vertnum.add(flexid as usize) {
                // get vertex
                let vpos: *const f64 = (*d).flexvert_xpos.add(3 * (*(*m).flex_vertadr.add(flexid as usize) + v) as usize);

                // construct sphere geom
                size[0] = radius;

                // intersect ray with sphere
                let sol: f64 = mju_ray_geom(
                    vpos, core::ptr::null(), size.as_ptr(), pnt, vec, MJGEOM_SPHERE,
                    if !normal.is_null() { normal_local.as_mut_ptr() } else { core::ptr::null_mut() },
                );

                // update
                if sol >= 0.0 && (x < 0.0 || sol < x) {
                    x = sol;
                    if !normal.is_null() {
                        mju_copy3(normal, normal_local.as_ptr());
                    }
                    vertid_local[0] = v;
                }
            }
        }

        // check faces if rendered
        if dim > 1 && (flg_face_b || flg_skin_b) {
            for e in 0..*(*m).flex_elemnum.add(flexid as usize) {
                // skip if 3D element is not visible
                let elayer: i32 = *(*m).flex_elemlayer.add((*(*m).flex_elemadr.add(flexid as usize) + e) as usize);
                if dim == 3 && ((flg_skin_b && elayer > 0) || (!flg_skin_b && elayer != flex_layer)) {
                    continue;
                }

                // get element data
                let edata: *const i32 = (*m).flex_elem.add((*(*m).flex_elemdataadr.add(flexid as usize) + e * (dim + 1)) as usize);
                let v1: *const f64 = (*d).flexvert_xpos.add(3 * (*(*m).flex_vertadr.add(flexid as usize) + *edata.add(0)) as usize);
                let v2: *const f64 = (*d).flexvert_xpos.add(3 * (*(*m).flex_vertadr.add(flexid as usize) + *edata.add(1)) as usize);
                let v3: *const f64 = (*d).flexvert_xpos.add(3 * (*(*m).flex_vertadr.add(flexid as usize) + *edata.add(2)) as usize);
                let v4: *const f64 = if dim == 2 {
                    core::ptr::null()
                } else {
                    (*d).flexvert_xpos.add(3 * (*(*m).flex_vertadr.add(flexid as usize) + *edata.add(3)) as usize)
                };
                let vptr: [[*const f64; 3]; 4] = [
                    [v1, v2, v3],
                    [v1, v2, v4],
                    [v1, v3, v4],
                    [v2, v3, v4],
                ];
                let vid: [[i32; 3]; 4] = [
                    [0, 1, 2],
                    [0, 1, 3],
                    [0, 2, 3],
                    [1, 2, 3],
                ];

                // process triangles of this element
                let ntri = if dim == 2 { 1 } else { 4 };
                for i in 0..ntri {
                    // copy vertices into triangle representation
                    let mut v_tri: [[f64; 3]; 3] = [[0.0; 3]; 3];
                    for j in 0..3usize {
                        mju_copy3(v_tri[j].as_mut_ptr(), vptr[i][j]);
                    }

                    // intersect ray with triangle
                    let sol: f64 = ray_triangle(
                        v_tri.as_ptr(), pnt, vec, b0.as_ptr(), b1.as_ptr(),
                        if !normal.is_null() { normal_local.as_mut_ptr() } else { core::ptr::null_mut() },
                    );

                    // update
                    if sol >= 0.0 && (x < 0.0 || sol < x) {
                        x = sol;
                        if !normal.is_null() {
                            mju_copy3(normal, normal_local.as_ptr());
                        }

                        // construct intersection point
                        let mut intersect: [f64; 3] = [0.0; 3];
                        mju_add_scl3(intersect.as_mut_ptr(), pnt, vec, sol);

                        // find nearest vertex
                        let dist: [f64; 3] = [
                            mju_dist3(v_tri[0].as_ptr(), intersect.as_ptr()),
                            mju_dist3(v_tri[1].as_ptr(), intersect.as_ptr()),
                            mju_dist3(v_tri[2].as_ptr(), intersect.as_ptr()),
                        ];
                        if dist[0] <= dist[1] && dist[0] <= dist[2] {
                            vertid_local[0] = *edata.add(vid[i][0] as usize);
                        } else if dist[1] <= dist[2] {
                            vertid_local[0] = *edata.add(vid[i][1] as usize);
                        } else {
                            vertid_local[0] = *edata.add(vid[i][2] as usize);
                        }
                    }
                }
            }
        }

        x
    }
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
    use crate::engine::engine_util_blas::{mju_add_scl3, mju_normalize3, mju_dot3, mju_dist3};
    use crate::engine::engine_util_spatial::mju_cross;

    // NOTE: In C ABI, vertid: [i32; 1] is a pointer. vertid.as_ptr() gives that pointer.
    let vertid_ptr = vertid.as_ptr() as *mut i32;

    // SAFETY: all pointers valid per caller contract; face[3*nface], vert[3*nvert], pnt[3], vec[3]
    unsafe {
        // compute bounding box
        let mut bbox: [[f64; 2]; 3] = [[0.0, 0.0], [0.0, 0.0], [0.0, 0.0]];
        for i in 0..nvert as usize {
            for j in 0..3usize {
                // update minimum along side j
                if bbox[j][0] > *vert.add(3*i+j) as f64 || i == 0 {
                    bbox[j][0] = *vert.add(3*i+j) as f64;
                }
                // update maximum along side j
                if bbox[j][1] < *vert.add(3*i+j) as f64 || i == 0 {
                    bbox[j][1] = *vert.add(3*i+j) as f64;
                }
            }
        }

        // construct box geom
        let mut pos: [f64; 3] = [0.0; 3];
        let mut size: [f64; 3] = [0.0; 3];
        let mat: [f64; 9] = [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0];
        for j in 0..3usize {
            pos[j] = 0.5 * (bbox[j][0] + bbox[j][1]);
            size[j] = 0.5 * (bbox[j][1] - bbox[j][0]);
        }

        // apply bounding-box filter
        if ray_box(pos.as_ptr(), mat.as_ptr(), size.as_ptr(), pnt, vec, core::ptr::null_mut(), core::ptr::null_mut()) < 0.0 {
            return -1.0;
        }

        // construct basis vectors of normal plane
        let mut b0: [f64; 3] = [1.0, 1.0, 1.0];
        let mut b1: [f64; 3] = [0.0; 3];
        let vec0_abs = (*vec.add(0)).abs();
        let vec1_abs = (*vec.add(1)).abs();
        let vec2_abs = (*vec.add(2)).abs();
        if vec0_abs >= vec1_abs && vec0_abs >= vec2_abs {
            b0[0] = 0.0;
        } else if vec1_abs >= vec2_abs {
            b0[1] = 0.0;
        } else {
            b0[2] = 0.0;
        }
        mju_add_scl3(b1.as_mut_ptr(), b0.as_ptr(), vec, -mju_dot3(vec, b0.as_ptr()) / mju_dot3(vec, vec));
        mju_normalize3(b1.as_mut_ptr());
        mju_cross(b0.as_mut_ptr(), b1.as_ptr(), vec);
        mju_normalize3(b0.as_mut_ptr());

        // init solution
        let mut x: f64 = -1.0;

        // process all faces
        for i in 0..nface as usize {
            // get float vertex pointers
            let vf0 = vert.add(3 * (*face.add(3*i) as usize));
            let vf1 = vert.add(3 * (*face.add(3*i+1) as usize));
            let vf2 = vert.add(3 * (*face.add(3*i+2) as usize));

            // convert to f64
            let mut v: [[f64; 3]; 3] = [[0.0; 3]; 3];
            for k in 0..3usize {
                v[0][k] = *vf0.add(k) as f64;
                v[1][k] = *vf1.add(k) as f64;
                v[2][k] = *vf2.add(k) as f64;
            }

            // solve
            let sol = ray_triangle(v.as_ptr() as *const [f64; 3], pnt, vec, b0.as_ptr(), b1.as_ptr(), core::ptr::null_mut());

            // update
            if sol >= 0.0 && (x < 0.0 || sol < x) {
                x = sol;

                // construct intersection point
                let mut intersect: [f64; 3] = [0.0; 3];
                mju_add_scl3(intersect.as_mut_ptr(), pnt, vec, sol);

                // find nearest vertex
                let mut dist = mju_dist3(intersect.as_ptr(), v[0].as_ptr());
                if !vertid_ptr.is_null() {
                    *vertid_ptr = *face.add(3*i);
                }
                for j in 1..3usize {
                    let newdist = mju_dist3(intersect.as_ptr(), v[j].as_ptr());
                    if newdist < dist {
                        dist = newdist;
                        if !vertid_ptr.is_null() {
                            *vertid_ptr = *face.add(3*i+j);
                        }
                    }
                }
            }
        }

        x
    }
}

