//! Port of: Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_primitive.c
//! IR hash: 1fa44043fe383ce9
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mjraw_PlaneSphere (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_primitive.c:28)
/// Calls: cxx:_mju_dot3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjraw_plane_sphere(con: *mut mjPreContact, margin: f64, pos1: *const f64, mat1: *const f64, size1: *const f64, pos2: *const f64, mat2: *const f64, size2: *const f64) -> i32 {
    todo!() // cxx-internal:engine_collision_primitive.c.o:_mjraw_PlaneSphere
}

/// C: mjraw_SphereSphere (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_primitive.c:262)
/// Calls: cxx:_mju_dot3, cxx:_mju_normalize3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjraw_sphere_sphere(con: *mut mjPreContact, margin: f64, pos1: *const f64, mat1: *const f64, size1: *const f64, pos2: *const f64, mat2: *const f64, size2: *const f64) -> i32 {
    todo!() // cxx-internal:engine_collision_primitive.c.o:_mjraw_SphereSphere
}

/// C: areaSign (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_primitive.c:534)
/// Calls: cxx:_mju_sign
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn area_sign(p1: *const f64, p2: *const f64, p3: *const f64) -> f64 {
    todo!() // cxx-internal:engine_collision_primitive.c.o:_areaSign
}

/// C: pointSegment (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_primitive.c:540)
/// Calls: cxx:_mju_addScl, cxx:_mju_dot, cxx:_mju_max
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn point_segment(res: *mut f64, p: *const f64, u: *const f64, v: *const f64) -> f64 {
    todo!() // cxx-internal:engine_collision_primitive.c.o:_pointSegment
}

/// C: mjraw_SphereCapsule (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_primitive.h:28)
/// Calls: cxx-internal:engine_collision_primitive.c.o:_mjraw_SphereSphere, cxx:_mju_clip, cxx:_mju_dot3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjraw_sphere_capsule(con: *mut mjPreContact, margin: f64, pos1: *const f64, mat1: *const f64, size1: *const f64, pos2: *const f64, mat2: *const f64, size2: *const f64) -> i32 {
    todo!() // cxx:_mjraw_SphereCapsule
}

/// C: mjraw_CapsuleCapsule (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_primitive.h:31)
/// Calls: cxx-internal:engine_collision_primitive.c.o:_mjraw_SphereSphere, cxx:_mju_clip, cxx:_mju_dot3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjraw_capsule_capsule(con: *mut mjPreContact, margin: f64, pos1: *const f64, mat1: *const f64, size1: *const f64, pos2: *const f64, mat2: *const f64, size2: *const f64) -> i32 {
    todo!() // cxx:_mjraw_CapsuleCapsule
}

/// C: mjraw_CapsuleBox (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_primitive.h:34)
/// Calls: cxx:_mjraw_SphereBox, cxx:_mju_addTo3, cxx:_mju_copy3, cxx:_mju_dot3, cxx:_mju_mulMatVec3, cxx:_mju_zero3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjraw_capsule_box(con: *mut mjPreContact, margin: f64, pos1: *const f64, mat1: *const f64, size1: *const f64, pos2: *const f64, mat2: *const f64, size2: *const f64) -> i32 {
    todo!() // cxx:_mjraw_CapsuleBox
}

/// C: mjraw_SphereTriangle (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_primitive.h:37)
/// Calls: cxx-internal:engine_collision_primitive.c.o:_areaSign, cxx-internal:engine_collision_primitive.c.o:_pointSegment, cxx:_mju_dot3, cxx:_mju_normalize3, cxx:_mju_zero3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjraw_sphere_triangle(con: *mut mjPreContact, margin: f64, s: *const f64, rs: f64, t1: *const f64, t2: *const f64, t3: *const f64, rt: f64) -> i32 {
    todo!() // cxx:_mjraw_SphereTriangle
}

/// C: mjraw_BoxTriangle (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_primitive.h:39)
/// Calls: cxx:_mjraw_SphereTriangle, cxx:_mju_addTo3, cxx:_mju_mulMatTVec3, cxx:_mju_mulMatVec3, cxx:_mju_sub3, cxx:_mju_zero3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjraw_box_triangle(con: *mut mjPreContact, margin: f64, pos: *const f64, mat: *const f64, size: *const f64, t1: *const f64, t2: *const f64, t3: *const f64, rt: f64) -> i32 {
    todo!() // cxx:_mjraw_BoxTriangle
}

/// C: mjraw_CapsuleTriangle (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_primitive.h:42)
/// Calls: cxx:_mjraw_SphereTriangle, cxx:_mju_addScl3, cxx:_mju_dot3, cxx:_mju_normalize3, cxx:_mju_scl3, cxx:_mju_sub3, cxx:_mju_zero3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjraw_capsule_triangle(con: *mut mjPreContact, margin: f64, pos: *const f64, mat: *const f64, size: *const f64, t1: *const f64, t2: *const f64, t3: *const f64, rt: f64) -> i32 {
    todo!() // cxx:_mjraw_CapsuleTriangle
}

/// C: mjc_PlaneSphere (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_primitive.h:47)
/// Calls: cxx-internal:engine_collision_primitive.c.o:_mjraw_PlaneSphere
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_plane_sphere(m: *const mjModel, d: *mut mjData, con: *mut mjPreContact, g1: i32, g2: i32, margin: f64) -> i32 {
    todo!() // cxx:_mjc_PlaneSphere
}

/// C: mjc_PlaneCapsule (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_primitive.h:49)
/// Calls: cxx-internal:engine_collision_primitive.c.o:_mjraw_PlaneSphere, cxx:_mju_add3, cxx:_mju_sub3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_plane_capsule(m: *const mjModel, d: *mut mjData, con: *mut mjPreContact, g1: i32, g2: i32, margin: f64) -> i32 {
    todo!() // cxx:_mjc_PlaneCapsule
}

/// C: mjc_PlaneCylinder (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_primitive.h:51)
/// Calls: cxx:_mju_dot3, cxx:_mju_normalize3, cxx:_mju_scl3, cxx:_mju_subFrom3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_plane_cylinder(m: *const mjModel, d: *mut mjData, con: *mut mjPreContact, g1: i32, g2: i32, margin: f64) -> i32 {
    todo!() // cxx:_mjc_PlaneCylinder
}

/// C: mjc_PlaneBox (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_primitive.h:53)
/// Calls: cxx:_mju_dot3, cxx:_mju_mulMatVec3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_plane_box(m: *const mjModel, d: *mut mjData, con: *mut mjPreContact, g1: i32, g2: i32, margin: f64) -> i32 {
    todo!() // cxx:_mjc_PlaneBox
}

/// C: mjc_SphereSphere (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_primitive.h:57)
/// Calls: cxx-internal:engine_collision_primitive.c.o:_mjraw_SphereSphere
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_sphere_sphere(m: *const mjModel, d: *mut mjData, con: *mut mjPreContact, g1: i32, g2: i32, margin: f64) -> i32 {
    todo!() // cxx:_mjc_SphereSphere
}

/// C: mjc_SphereCapsule (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_primitive.h:59)
/// Calls: cxx:_mjraw_SphereCapsule
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_sphere_capsule(m: *const mjModel, d: *mut mjData, con: *mut mjPreContact, g1: i32, g2: i32, margin: f64) -> i32 {
    todo!() // cxx:_mjc_SphereCapsule
}

/// C: mjc_SphereCylinder (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_primitive.h:61)
/// Calls: cxx-internal:engine_collision_primitive.c.o:_mjraw_PlaneSphere, cxx-internal:engine_collision_primitive.c.o:_mjraw_SphereSphere, cxx:_mju_addScl3, cxx:_mju_dot3, cxx:_mju_scl3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_sphere_cylinder(m: *const mjModel, d: *mut mjData, con: *mut mjPreContact, g1: i32, g2: i32, margin: f64) -> i32 {
    todo!() // cxx:_mjc_SphereCylinder
}

/// C: mjc_CapsuleCapsule (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_primitive.h:63)
/// Calls: cxx:_mjraw_CapsuleCapsule
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_capsule_capsule(m: *const mjModel, d: *mut mjData, con: *mut mjPreContact, g1: i32, g2: i32, margin: f64) -> i32 {
    todo!() // cxx:_mjc_CapsuleCapsule
}

/// C: mjc_CapsuleBox (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_primitive.h:67)
/// Calls: cxx:_mjraw_CapsuleBox
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_capsule_box(m: *const mjModel, d: *mut mjData, con: *mut mjPreContact, g1: i32, g2: i32, margin: f64) -> i32 {
    todo!() // cxx:_mjc_CapsuleBox
}

/// C: mjc_SphereBox (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_primitive.h:69)
/// Calls: cxx:_mjraw_SphereBox
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_sphere_box(m: *const mjModel, d: *mut mjData, con: *mut mjPreContact, g1: i32, g2: i32, margin: f64) -> i32 {
    todo!() // cxx:_mjc_SphereBox
}

/// C: mjc_BoxBox (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_primitive.h:71)
/// Calls: cxx-internal:engine_collision_box.c.o:__boxbox, cxx:_mju_outsideBox
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_box_box(m: *const mjModel, d: *mut mjData, con: *mut mjPreContact, g1: i32, g2: i32, margin: f64) -> i32 {
    todo!() // cxx:_mjc_BoxBox
}

