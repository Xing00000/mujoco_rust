//! Port of: Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_ray.c
//! IR hash: 1fa44043fe383ce9
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: ray_map (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_ray.c:38)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn ray_map(pos: *const f64, mat: *const f64, pnt: *const f64, vec: *const f64, lpnt: *mut f64, lvec: *mut f64) {
    todo!() // cxx-internal:engine_ray.c.o:_ray_map
}

/// C: longitude (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_ray.c:56)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn longitude(vec: *const f64) -> f64 {
    todo!() // cxx-internal:engine_ray.c.o:_longitude
}

/// C: latitude (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_ray.c:62)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn latitude(vec: *const f64) -> f64 {
    todo!() // cxx-internal:engine_ray.c.o:_latitude
}

/// C: ray_eliminate (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_ray.c:68)
#[allow(unused_variables, non_snake_case)]
pub fn ray_eliminate(m: *const mjModel, d: *const mjData, geomid: i32, geomgroup: *const u8, flg_static: bool, bodyexclude: i32) -> i32 {
    todo!() // cxx-internal:engine_ray.c.o:_ray_eliminate
}

/// C: ray_quad (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_ray.c:103)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn ray_quad(a: f64, b: f64, c: f64, x: *mut f64) -> f64 {
    todo!() // cxx-internal:engine_ray.c.o:_ray_quad
}

/// C: ray_plane (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_ray.c:204)
/// Calls: cxx-internal:engine_ray.c.o:_ray_map, cxx:_mju_zero3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn ray_plane(pos: *const f64, mat: *const f64, size: *const f64, pnt: *const f64, vec: *const f64, normal: *mut f64) -> f64 {
    todo!() // cxx-internal:engine_ray.c.o:_ray_plane
}

/// C: ray_sphere (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_ray.c:242)
/// Calls: cxx-internal:engine_ray.c.o:_ray_quad, cxx:_mju_addScl3, cxx:_mju_normalize3, cxx:_mju_sub3, cxx:_mju_zero3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn ray_sphere(pos: *const f64, mat: *const f64, dist_sqr: f64, pnt: *const f64, vec: *const f64, normal: *mut f64) -> f64 {
    todo!() // cxx-internal:engine_ray.c.o:_ray_sphere
}

/// C: ray_capsule (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_ray.c:272)
/// Calls: cxx-internal:engine_ray.c.o:_ray_map, cxx-internal:engine_ray.c.o:_ray_quad, cxx-internal:engine_ray.c.o:_ray_sphere, cxx:_mju_mulMatVec3, cxx:_mju_normalize3, cxx:_mju_zero3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn ray_capsule(pos: *const f64, mat: *const f64, size: *const f64, pnt: *const f64, vec: *const f64, normal: *mut f64) -> f64 {
    todo!() // cxx-internal:engine_ray.c.o:_ray_capsule
}

/// C: ray_ellipsoid (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_ray.c:358)
/// Calls: cxx-internal:engine_ray.c.o:_ray_map, cxx-internal:engine_ray.c.o:_ray_quad, cxx:_mju_addScl3, cxx:_mju_mulMatVec3, cxx:_mju_normalize3, cxx:_mju_zero3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn ray_ellipsoid(pos: *const f64, mat: *const f64, size: *const f64, pnt: *const f64, vec: *const f64, normal: *mut f64) -> f64 {
    todo!() // cxx-internal:engine_ray.c.o:_ray_ellipsoid
}

/// C: ray_cylinder (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_ray.c:401)
/// Calls: cxx-internal:engine_ray.c.o:_ray_map, cxx-internal:engine_ray.c.o:_ray_quad, cxx-internal:engine_ray.c.o:_ray_sphere, cxx:_mju_mulMatVec3, cxx:_mju_normalize3, cxx:_mju_zero3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn ray_cylinder(pos: *const f64, mat: *const f64, size: *const f64, pnt: *const f64, vec: *const f64, normal: *mut f64) -> f64 {
    todo!() // cxx-internal:engine_ray.c.o:_ray_cylinder
}

/// C: ray_box (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_ray.c:490)
/// Calls: cxx-internal:engine_ray.c.o:_ray_map, cxx-internal:engine_ray.c.o:_ray_sphere, cxx:_mju_mulMatVec3, cxx:_mju_zero3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn ray_box(pos: *const f64, mat: *const f64, size: *const f64, pnt: *const f64, vec: *const f64, all: *mut f64, normal: *mut f64) -> f64 {
    todo!() // cxx-internal:engine_ray.c.o:_ray_box
}

/// C: mju_raySlab (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_ray.c:743)
/// Calls: cxx-internal:engine_ray.c.o:_ray_map
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_ray_slab(aabb: *const f64, xpos: *const f64, xmat: *const f64, pnt: *const f64, vec: *const f64) -> i32 {
    todo!() // cxx:_mju_raySlab
}

/// C: mju_rayTree (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_ray.c:771)
/// Calls: cxx-internal:engine_ray.c.o:_ray_map, cxx:_mju_addScl3, cxx:_mju_copy3, cxx:_mju_cross, cxx:_mju_dot3, cxx:_mju_message, cxx:_mju_mulMatVec3, cxx:_mju_normalize3, cxx:_mju_raySlab, cxx:_mju_zero3, cxx:_ray_triangle
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_ray_tree(m: *const mjModel, d: *const mjData, id: i32, pnt: *const f64, vec: *const f64, normal: *mut f64) -> f64 {
    todo!() // cxx:_mju_rayTree
}

/// C: mj_raySdf (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_ray.c:885)
/// Calls: cxx-internal:engine_ray.c.o:_ray_box, cxx-internal:engine_ray.c.o:_ray_map, cxx:_mjc_distance, cxx:_mjc_getSDF, cxx:_mjc_gradient, cxx:_mju_addScl3, cxx:_mju_mulMatVec3, cxx:_mju_normalize3, cxx:_mju_zero3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_ray_sdf(m: *const mjModel, d: *const mjData, g: i32, pnt: *const f64, vec: *const f64, normal: *mut f64) -> f64 {
    todo!() // cxx-internal:engine_ray.c.o:_mj_raySdf
}

/// C: point_in_box (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_ray.c:1283)
/// Calls: cxx:_mju_mulMatTVec3, cxx:_mju_sub3, cxx:_mju_subFrom3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn point_in_box(aabb: *const f64, xpos: *const f64, xmat: *const f64, pnt: *const f64) -> i32 {
    todo!() // cxx-internal:engine_ray.c.o:_point_in_box
}

/// C: mju_singleRay (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_ray.c:1457)
/// Calls: cxx-internal:engine_ray.c.o:_latitude, cxx-internal:engine_ray.c.o:_longitude, cxx-internal:engine_ray.c.o:_mj_raySdf, cxx-internal:engine_ray.c.o:_ray_sphere, cxx:_mj_rayHfield, cxx:_mj_rayMesh, cxx:_mju_add3, cxx:_mju_copy3, cxx:_mju_rayGeom, cxx:_mju_zero3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_single_ray(m: *const mjModel, d: *mut mjData, pnt: *const f64, vec: *const f64, ray_eliminate: *mut i32, geom_ba: *mut f64, geomid: *mut i32, normal: *mut f64) -> f64 {
    todo!() // cxx-internal:engine_ray.c.o:_mju_singleRay
}

/// C: mju_multiRayPrepare (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_ray.h:26)
/// Calls: cxx-internal:engine_ray.c.o:_latitude, cxx-internal:engine_ray.c.o:_longitude, cxx-internal:engine_ray.c.o:_point_in_box, cxx-internal:engine_ray.c.o:_ray_eliminate, cxx:_mju_addTo3, cxx:_mju_copy, cxx:_mju_dist3, cxx:_mju_max, cxx:_mju_message, cxx:_mju_min, cxx:_mju_mulMatVec3, cxx:_mju_sub3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_multi_ray_prepare(m: *const mjModel, d: *const mjData, pnt: *const f64, ray_xmat: *const f64, geomgroup: *const u8, flg_static: bool, bodyexclude: i32, cutoff: f64, geom_ba: *mut f64, geom_eliminate: *mut i32) {
    todo!() // cxx:_mju_multiRayPrepare
}

/// C: mj_multiRay (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_ray.h:34)
/// Calls: cxx-internal:engine_ray.c.o:_mju_singleRay, cxx:_mj_freeStack, cxx:_mj_markStack, cxx:_mj_stackAllocInfo, cxx:_mju_dot3, cxx:_mju_multiRayPrepare
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_multi_ray(m: *const mjModel, d: *mut mjData, pnt: *const f64, vec: *const f64, geomgroup: *const u8, flg_static: bool, bodyexclude: i32, geomid: *mut i32, dist: *mut f64, normal: *mut f64, nray: i32, cutoff: f64) {
    todo!() // cxx:_mj_multiRay
}

/// C: mj_ray (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_ray.h:42)
/// Calls: cxx-internal:engine_ray.c.o:_mj_raySdf, cxx-internal:engine_ray.c.o:_ray_eliminate, cxx:_mj_rayHfield, cxx:_mj_rayMesh, cxx:_mju_copy3, cxx:_mju_message, cxx:_mju_norm3, cxx:_mju_rayGeom, cxx:_mju_zero3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_ray(m: *const mjModel, d: *const mjData, pnt: *const f64, vec: *const f64, geomgroup: *const u8, flg_static: bool, bodyexclude: i32, geomid: *mut i32, normal: *mut f64) -> f64 {
    todo!() // cxx:_mj_ray
}

/// C: mj_rayHfield (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_ray.h:47)
/// Calls: cxx-internal:engine_ray.c.o:_ray_box, cxx-internal:engine_ray.c.o:_ray_map, cxx:_mju_addScl3, cxx:_mju_copy3, cxx:_mju_cross, cxx:_mju_dot3, cxx:_mju_message, cxx:_mju_mulMatTVec3, cxx:_mju_mulMatVec3, cxx:_mju_normalize3, cxx:_mju_round, cxx:_mju_zero3, cxx:_ray_triangle
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_ray_hfield(m: *const mjModel, d: *const mjData, geomid: i32, pnt: *const f64, vec: *const f64, normal: *mut f64) -> f64 {
    todo!() // cxx:_mj_rayHfield
}

/// C: ray_triangle (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_ray.h:51)
/// Calls: cxx:_mju_copy3, cxx:_mju_cross, cxx:_mju_dot3, cxx:_mju_normalize3, cxx:_mju_sub3, cxx:_mju_zero3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn ray_triangle(v: *mut [f64; 3], lpnt: *const f64, lvec: *const f64, b0: *const f64, b1: *const f64, normal: *mut f64) -> f64 {
    todo!() // cxx:_ray_triangle
}

/// C: mj_rayMesh (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_ray.h:55)
/// Calls: cxx-internal:engine_ray.c.o:_ray_box, cxx:_mju_message, cxx:_mju_rayTree, cxx:_mju_zero3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_ray_mesh(m: *const mjModel, d: *const mjData, geomid: i32, pnt: *const f64, vec: *const f64, normal: *mut f64) -> f64 {
    todo!() // cxx:_mj_rayMesh
}

/// C: mju_rayGeom (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_ray.h:59)
/// Calls: cxx-internal:engine_ray.c.o:_ray_box, cxx-internal:engine_ray.c.o:_ray_capsule, cxx-internal:engine_ray.c.o:_ray_cylinder, cxx-internal:engine_ray.c.o:_ray_ellipsoid, cxx-internal:engine_ray.c.o:_ray_plane, cxx-internal:engine_ray.c.o:_ray_sphere, cxx:_mju_message
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_ray_geom(pos: *const f64, mat: *const f64, size: *const f64, pnt: *const f64, vec: *const f64, geomtype: i32, normal: *mut f64) -> f64 {
    todo!() // cxx:_mju_rayGeom
}

/// C: mj_rayFlex (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_ray.h:64)
/// Calls: cxx-internal:engine_ray.c.o:_ray_box, cxx:_mju_add3, cxx:_mju_addScl3, cxx:_mju_copy3, cxx:_mju_cross, cxx:_mju_dist3, cxx:_mju_dot3, cxx:_mju_normalize3, cxx:_mju_quat2Mat, cxx:_mju_quatZ2Vec, cxx:_mju_rayGeom, cxx:_mju_scl3, cxx:_mju_zero3, cxx:_ray_triangle
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_ray_flex(m: *const mjModel, d: *const mjData, flex_layer: i32, flg_vert: bool, flg_edge: bool, flg_face: bool, flg_skin: bool, flexid: i32, pnt: *const f64, vec: *const f64, vertid: *mut i32, normal: *mut f64) -> f64 {
    todo!() // cxx:_mj_rayFlex
}

/// C: mju_raySkin (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_ray.h:70)
/// Calls: cxx-internal:engine_ray.c.o:_ray_box, cxx:_mju_addScl3, cxx:_mju_cross, cxx:_mju_dist3, cxx:_mju_dot3, cxx:_mju_normalize3, cxx:_ray_triangle
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_ray_skin(nface: i32, nvert: i32, face: *const i32, vert: *const f32, pnt: *const f64, vec: *const f64, vertid: *mut i32) -> f64 {
    todo!() // cxx:_mju_raySkin
}

