//! Port of: engine/engine_ray.c
//! IR hash: 699b5f0da57e8d78
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
    // WARNING: signature changed — verify body
    // Previous params: (pos : * const f64, mat : * const f64, pnt : * const f64, vec : * const f64, lpnt : * mut f64, lvec : * mut f64)
    // Previous return: ()
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * const mjData, geomid : i32, geomgroup : * const u8, flg_static : mjtBool, bodyexclude : i32)
    // Previous return: i32
    todo ! ()
}

/// C: ray_quad (engine/engine_ray.c:103)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn ray_quad(a: f64, b: f64, c: f64, x: *mut f64) -> f64 {
    // WARNING: signature changed — verify body
    // Previous params: (a : f64, b : f64, c : f64, x : * mut f64)
    // Previous return: f64
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (pos : * const f64, mat : * const f64, size : * const f64, pnt : * const f64, vec : * const f64, normal : * mut f64)
    // Previous return: f64
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (pos : * const f64, mat : * const f64, dist_sqr : f64, pnt : * const f64, vec : * const f64, normal : * mut f64)
    // Previous return: f64
    todo ! ()
}

/// C: ray_capsule (engine/engine_ray.c:272)
/// Calls: mju_mulMatVec3, mju_normalize3, mju_zero3, ray_map, ray_quad
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn ray_capsule(pos: *const f64, mat: *const f64, size: *const f64, pnt: *const f64, vec: *const f64, normal: *mut f64) -> f64 {
    // WARNING: signature changed — verify body
    // Previous params: (pos : * const f64, mat : * const f64, size : * const f64, pnt : * const f64, vec : * const f64, normal : * mut f64)
    // Previous return: f64
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (pos : * const f64, mat : * const f64, size : * const f64, pnt : * const f64, vec : * const f64, normal : * mut f64)
    // Previous return: f64
    todo ! ()
}

/// C: ray_cylinder (engine/engine_ray.c:401)
/// Calls: mju_mulMatVec3, mju_normalize3, mju_zero3, ray_map, ray_quad
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn ray_cylinder(pos: *const f64, mat: *const f64, size: *const f64, pnt: *const f64, vec: *const f64, normal: *mut f64) -> f64 {
    // WARNING: signature changed — verify body
    // Previous params: (pos : * const f64, mat : * const f64, size : * const f64, pnt : * const f64, vec : * const f64, normal : * mut f64)
    // Previous return: f64
    todo ! ()
}

/// C: ray_box (engine/engine_ray.c:490)
/// Calls: mju_mulMatVec3, mju_zero3, ray_map
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
/// Calls: mju_addScl3, mju_copy3, mju_cross, mju_dot3, mju_message, mju_mulMatVec3, mju_normalize3, mju_raySlab, mju_zero3, ray_map
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
/// Calls: mjc_distance, mjc_gradient, mju_addScl3, mju_mulMatVec3, mju_normalize3, mju_zero3, ray_map
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_ray_sdf(m: *const mjModel, d: *const mjData, g: i32, pnt: *const f64, vec: *const f64, normal: *mut f64) -> f64 {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * const mjData, g : i32, pnt : * const f64, vec : * const f64, normal : * mut f64)
    // Previous return: f64
    todo ! ()
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
/// Calls: latitude, longitude, mj_rayHfield, mj_rayMesh, mj_raySdf, mju_add3, mju_copy3, mju_rayGeom, mju_zero3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_single_ray(m: *const mjModel, d: *mut mjData, pnt: *const f64, vec: *const f64, ray_eliminate: *mut i32, geom_ba: *mut f64, geomid: *mut i32, normal: *mut f64) -> f64 {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, pnt : * const f64, vec : * const f64, ray_eliminate : * mut i32, geom_ba : * mut f64, geomid : * mut i32, normal : * mut f64)
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
/// Calls: mj_freeStack, mj_markStack, mju_dot3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_multi_ray(m: *const mjModel, d: *mut mjData, pnt: *const f64, vec: *const f64, geomgroup: *const u8, flg_static: mjtBool, bodyexclude: i32, geomid: *mut i32, dist: *mut f64, normal: *mut f64, nray: i32, cutoff: f64) {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, pnt : * const f64, vec : * const f64, geomgroup : * const u8, flg_static : mjtBool, bodyexclude : i32, geomid : * mut i32, dist : * mut f64, normal : * mut f64, nray : i32, cutoff : f64)
    // Previous return: ()
    todo ! ()
}

/// C: mj_ray (engine/engine_ray.h:42)
/// Calls: mj_rayHfield, mj_rayMesh, mj_raySdf, mju_copy3, mju_message, mju_norm3, mju_rayGeom, mju_zero3, ray_eliminate
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_ray(m: *const mjModel, d: *const mjData, pnt: *const f64, vec: *const f64, geomgroup: *const u8, flg_static: mjtBool, bodyexclude: i32, geomid: *mut i32, normal: *mut f64) -> f64 {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * const mjData, pnt : * const f64, vec : * const f64, geomgroup : * const u8, flg_static : mjtBool, bodyexclude : i32, geomid : * mut i32, normal : * mut f64)
    // Previous return: f64
    todo ! ()
}

/// C: mj_rayHfield (engine/engine_ray.h:47)
/// Calls: mju_addScl3, mju_copy3, mju_cross, mju_dot3, mju_message, mju_mulMatTVec3, mju_mulMatVec3, mju_normalize3, mju_round, mju_zero3, ray_map
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_ray_hfield(m: *const mjModel, d: *const mjData, geomid: i32, pnt: *const f64, vec: *const f64, normal: *mut f64) -> f64 {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * const mjData, geomid : i32, pnt : * const f64, vec : * const f64, normal : * mut f64)
    // Previous return: f64
    todo ! ()
}

/// C: ray_triangle (engine/engine_ray.h:51)
/// Calls: mju_copy3, mju_cross, mju_dot3, mju_normalize3, mju_sub3, mju_zero3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn ray_triangle(v: *mut f64, lpnt: *const f64, lvec: *const f64, b0: *const f64, b1: *const f64, normal: *mut f64) -> f64 {
    // WARNING: signature changed — verify body
    // Previous params: (v : * mut f64, lpnt : * const f64, lvec : * const f64, b0 : * const f64, b1 : * const f64, normal : * mut f64)
    // Previous return: f64
    todo ! ()
}

/// C: mj_rayMesh (engine/engine_ray.h:55)
/// Calls: mju_message, mju_rayTree, mju_zero3
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
/// Calls: mju_message, ray_capsule, ray_cylinder, ray_ellipsoid, ray_plane, ray_sphere
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
/// Calls: mju_add3, mju_addScl3, mju_copy3, mju_cross, mju_dist3, mju_dot3, mju_normalize3, mju_quat2Mat, mju_quatZ2Vec, mju_scl3, mju_zero3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_ray_flex(m: *const mjModel, d: *const mjData, flex_layer: i32, flg_vert: mjtBool, flg_edge: mjtBool, flg_face: mjtBool, flg_skin: mjtBool, flexid: i32, pnt: *const f64, vec: *const f64, vertid: *mut i32, normal: *mut f64) -> f64 {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * const mjData, flex_layer : i32, flg_vert : mjtBool, flg_edge : mjtBool, flg_face : mjtBool, flg_skin : mjtBool, flexid : i32, pnt : * const f64, vec : * const f64, vertid : * mut i32, normal : * mut f64)
    // Previous return: f64
    todo ! ()
}

/// C: mju_raySkin (engine/engine_ray.h:70)
/// Calls: mju_addScl3, mju_cross, mju_dist3, mju_dot3, mju_normalize3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_ray_skin(nface: i32, nvert: i32, face: *const i32, vert: *const f32, pnt: *const f64, vec: *const f64, vertid: *mut i32) -> f64 {
    // WARNING: signature changed — verify body
    // Previous params: (nface : i32, nvert : i32, face : * const i32, vert : * const f32, pnt : * const f64, vec : * const f64, vertid : * mut i32)
    // Previous return: f64
    todo ! ()
}

