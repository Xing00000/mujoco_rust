//! Port of: Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c
//! IR hash: 1fa44043fe383ce9
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: align8 (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:49)
/// Calls: cxx:__ZNK6mujoco4user8FilePath4sizeEv
#[allow(unused_variables, non_snake_case)]
pub fn align8(size: usize) -> usize {
    todo!() // cxx-internal:engine_collision_gjk.c.o:_align8
}

/// C: subdistance (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:56)
/// Calls: cxx-internal:engine_collision_gjk.c.o:_S1D, cxx-internal:engine_collision_gjk.c.o:_S2D, cxx-internal:engine_collision_gjk.c.o:_S3D
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn subdistance(lambda: *mut f64, n: i32, simplex: *const Vertex) {
    todo!() // cxx-internal:engine_collision_gjk.c.o:_subdistance
}

/// C: S3D (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:60)
/// Calls: cxx-internal:engine_collision_gjk.c.o:_S2D, cxx-internal:engine_collision_gjk.c.o:_det3, cxx-internal:engine_collision_gjk.c.o:_dot3, cxx-internal:engine_collision_gjk.c.o:_lincomb, cxx-internal:engine_collision_gjk.c.o:_sameSign2
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn s3d(lambda: *mut f64, s1: *const f64, s2: *const f64, s3: *const f64, s4: *const f64) {
    todo!() // cxx-internal:engine_collision_gjk.c.o:_S3D
}

/// C: S2D (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:62)
/// Calls: cxx-internal:engine_collision_gjk.c.o:_S1D, cxx-internal:engine_collision_gjk.c.o:_dot3, cxx-internal:engine_collision_gjk.c.o:_lincomb, cxx-internal:engine_collision_gjk.c.o:_projectOriginPlane, cxx-internal:engine_collision_gjk.c.o:_sameSign2
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn s2d(lambda: *mut f64, s1: *const f64, s2: *const f64, s3: *const f64) {
    todo!() // cxx-internal:engine_collision_gjk.c.o:_S2D
}

/// C: S1D (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:63)
/// Calls: cxx-internal:engine_collision_gjk.c.o:_projectOriginLine, cxx-internal:engine_collision_gjk.c.o:_sameSign2
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn s1d(lambda: *mut f64, s1: *const f64, s2: *const f64) {
    todo!() // cxx-internal:engine_collision_gjk.c.o:_S1D
}

/// C: gjkSupport (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:66)
/// Calls: cxx-internal:engine_collision_gjk.c.o:_scl3, cxx-internal:engine_collision_gjk.c.o:_support
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn gjk_support(v: *mut Vertex, obj1: *mut mjCCDObj, obj2: *mut mjCCDObj, x_k: *const f64, x_norm: f64) {
    todo!() // cxx-internal:engine_collision_gjk.c.o:_gjkSupport
}

/// C: lincomb (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:70)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn lincomb(res: *mut f64, coef: *const f64, n: i32, v1: *const f64, v2: *const f64, v3: *const f64, v4: *const f64) {
    todo!() // cxx-internal:engine_collision_gjk.c.o:_lincomb
}

/// C: epaSupport (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:108)
/// Calls: cxx-internal:engine_collision_gjk.c.o:_scl3, cxx-internal:engine_collision_gjk.c.o:_support
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn epa_support(pt: *mut Polytope, obj1: *mut mjCCDObj, obj2: *mut mjCCDObj, d: *const f64, dnorm: f64) -> i32 {
    todo!() // cxx-internal:engine_collision_gjk.c.o:_epaSupport
}

/// C: insertVertex (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:112)
#[allow(unused_variables, non_snake_case)]
pub fn insert_vertex(pt: *mut Polytope, v: *const Vertex) -> i32 {
    todo!() // cxx-internal:engine_collision_gjk.c.o:_insertVertex
}

/// C: attachFace (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:115)
/// Calls: cxx-internal:engine_collision_gjk.c.o:_dot3, cxx-internal:engine_collision_gjk.c.o:_projectOriginPlane, cxx-internal:engine_collision_gjk.c.o:_scl3, cxx-internal:engine_collision_gjk.c.o:_sub3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn attach_face(pt: *mut Polytope, v1: i32, v2: i32, v3: i32, adj1: i32, adj2: i32, adj3: i32) -> f64 {
    todo!() // cxx-internal:engine_collision_gjk.c.o:_attachFace
}

/// C: gjkIntersect (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:119)
/// Calls: cxx-internal:engine_collision_gjk.c.o:_dot3, cxx-internal:engine_collision_gjk.c.o:_gjkIntersectSupport, cxx-internal:engine_collision_gjk.c.o:_signedDistance
#[allow(unused_variables, non_snake_case)]
pub fn gjk_intersect(status: *mut mjCCDStatus, obj1: *mut mjCCDObj, obj2: *mut mjCCDObj) -> i32 {
    todo!() // cxx-internal:engine_collision_gjk.c.o:_gjkIntersect
}

/// C: polytope2 (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:122)
/// Calls: cxx-internal:engine_collision_gjk.c.o:_add3, cxx-internal:engine_collision_gjk.c.o:_attachFace, cxx-internal:engine_collision_gjk.c.o:_cross3, cxx-internal:engine_collision_gjk.c.o:_epaSupport, cxx-internal:engine_collision_gjk.c.o:_insertVertex, cxx-internal:engine_collision_gjk.c.o:_norm3, cxx-internal:engine_collision_gjk.c.o:_polytope3, cxx-internal:engine_collision_gjk.c.o:_rayTriangle, cxx-internal:engine_collision_gjk.c.o:_replaceSimplex3, cxx-internal:engine_collision_gjk.c.o:_rotmat, cxx-internal:engine_collision_gjk.c.o:_scl3, cxx-internal:engine_collision_gjk.c.o:_sub3, cxx:_mju_mulMatVec3
#[allow(unused_variables, non_snake_case)]
pub fn polytope2(pt: *mut Polytope, status: *mut mjCCDStatus, obj1: *mut mjCCDObj, obj2: *mut mjCCDObj) -> i32 {
    todo!() // cxx-internal:engine_collision_gjk.c.o:_polytope2
}

/// C: polytope3 (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:123)
/// Calls: cxx-internal:engine_collision_gjk.c.o:_add3, cxx-internal:engine_collision_gjk.c.o:_attachFace, cxx-internal:engine_collision_gjk.c.o:_cross3, cxx-internal:engine_collision_gjk.c.o:_epaSupport, cxx-internal:engine_collision_gjk.c.o:_insertVertex, cxx-internal:engine_collision_gjk.c.o:_norm3, cxx-internal:engine_collision_gjk.c.o:_scl3, cxx-internal:engine_collision_gjk.c.o:_sub3, cxx-internal:engine_collision_gjk.c.o:_testTetra, cxx-internal:engine_collision_gjk.c.o:_triPointIntersect
#[allow(unused_variables, non_snake_case)]
pub fn polytope3(pt: *mut Polytope, status: *mut mjCCDStatus, obj1: *mut mjCCDObj, obj2: *mut mjCCDObj) -> i32 {
    todo!() // cxx-internal:engine_collision_gjk.c.o:_polytope3
}

/// C: polytope4 (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:124)
/// Calls: cxx-internal:engine_collision_gjk.c.o:_add3, cxx-internal:engine_collision_gjk.c.o:_attachFace, cxx-internal:engine_collision_gjk.c.o:_insertVertex, cxx-internal:engine_collision_gjk.c.o:_polytope3, cxx-internal:engine_collision_gjk.c.o:_replaceSimplex3, cxx-internal:engine_collision_gjk.c.o:_scl3, cxx-internal:engine_collision_gjk.c.o:_testTetra
#[allow(unused_variables, non_snake_case)]
pub fn polytope4(pt: *mut Polytope, status: *mut mjCCDStatus, obj1: *mut mjCCDObj, obj2: *mut mjCCDObj) -> i32 {
    todo!() // cxx-internal:engine_collision_gjk.c.o:_polytope4
}

/// C: epa (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:128)
/// Calls: cxx-internal:engine_collision_gjk.c.o:_attachFace, cxx-internal:engine_collision_gjk.c.o:_discreteGeoms, cxx-internal:engine_collision_gjk.c.o:_dot3, cxx-internal:engine_collision_gjk.c.o:_epaSupport, cxx-internal:engine_collision_gjk.c.o:_epaWitness, cxx-internal:engine_collision_gjk.c.o:_horizon, cxx-internal:engine_collision_gjk.c.o:_maxFaces, cxx:_mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn epa(status: *mut mjCCDStatus, pt: *mut Polytope, obj1: *mut mjCCDObj, obj2: *mut mjCCDObj) -> *mut Face {
    todo!() // cxx-internal:engine_collision_gjk.c.o:_epa
}

/// C: equal3 (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:133)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn equal3(v1: *const f64, v2: *const f64) -> i32 {
    todo!() // cxx-internal:engine_collision_gjk.c.o:_equal3
}

/// C: add3 (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:140)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn add3(res: *mut f64, v1: *const f64, v2: *const f64) {
    todo!() // cxx-internal:engine_collision_gjk.c.o:_add3
}

/// C: sub3 (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:145)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn sub3(res: *mut f64, v1: *const f64, v2: *const f64) {
    todo!() // cxx-internal:engine_collision_gjk.c.o:_sub3
}

/// C: dot3 (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:150)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn dot3(v1: *const f64, v2: *const f64) -> f64 {
    todo!() // cxx-internal:engine_collision_gjk.c.o:_dot3
}

/// C: norm3 (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:155)
/// Calls: cxx-internal:engine_collision_gjk.c.o:_dot3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn norm3(v: *const f64) -> f64 {
    todo!() // cxx-internal:engine_collision_gjk.c.o:_norm3
}

/// C: copy3 (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:160)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn copy3(res: *mut f64, v: *const f64) {
    todo!() // cxx-internal:engine_collision_gjk.c.o:_copy3
}

/// C: scl3 (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:165)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn scl3(res: *mut f64, v: *const f64, s: f64) {
    todo!() // cxx-internal:engine_collision_gjk.c.o:_scl3
}

/// C: cross3 (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:170)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn cross3(res: *mut f64, v1: *const f64, v2: *const f64) {
    todo!() // cxx-internal:engine_collision_gjk.c.o:_cross3
}

/// C: det3 (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:177)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn det3(v1: *const f64, v2: *const f64, v3: *const f64) -> f64 {
    todo!() // cxx-internal:engine_collision_gjk.c.o:_det3
}

/// C: discreteGeoms (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:188)
#[allow(unused_variables, non_snake_case)]
pub fn discrete_geoms(obj1: *mut mjCCDObj, obj2: *mut mjCCDObj) -> i32 {
    todo!() // cxx-internal:engine_collision_gjk.c.o:_discreteGeoms
}

/// C: gjk (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:200)
/// Calls: cxx-internal:engine_collision_gjk.c.o:_copy3, cxx-internal:engine_collision_gjk.c.o:_discreteGeoms, cxx-internal:engine_collision_gjk.c.o:_dot3, cxx-internal:engine_collision_gjk.c.o:_equal3, cxx-internal:engine_collision_gjk.c.o:_gjkIntersect, cxx-internal:engine_collision_gjk.c.o:_gjkSupport, cxx-internal:engine_collision_gjk.c.o:_lincomb, cxx-internal:engine_collision_gjk.c.o:_sub3, cxx-internal:engine_collision_gjk.c.o:_subdistance
#[allow(unused_variables, non_snake_case)]
pub fn gjk(status: *mut mjCCDStatus, obj1: *mut mjCCDObj, obj2: *mut mjCCDObj) {
    todo!() // cxx-internal:engine_collision_gjk.c.o:_gjk
}

/// C: support (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:334)
/// Calls: cxx-internal:engine_collision_gjk.c.o:_sub3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn support(v: *mut Vertex, obj1: *mut mjCCDObj, obj2: *mut mjCCDObj, dir: *const f64, dir_neg: *const f64) {
    todo!() // cxx-internal:engine_collision_gjk.c.o:_support
}

/// C: gjkIntersectSupport (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:396)
/// Calls: cxx-internal:engine_collision_gjk.c.o:_support
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn gjk_intersect_support(v: *mut Vertex, obj1: *mut mjCCDObj, obj2: *mut mjCCDObj, dir: *const f64) {
    todo!() // cxx-internal:engine_collision_gjk.c.o:_gjkIntersectSupport
}

/// C: signedDistance (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:404)
/// Calls: cxx-internal:engine_collision_gjk.c.o:_cross3, cxx-internal:engine_collision_gjk.c.o:_dot3, cxx-internal:engine_collision_gjk.c.o:_scl3, cxx-internal:engine_collision_gjk.c.o:_sub3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn signed_distance(normal: *mut f64, v1: *const Vertex, v2: *const Vertex, v3: *const Vertex) -> f64 {
    todo!() // cxx-internal:engine_collision_gjk.c.o:_signedDistance
}

/// C: projectOriginPlane (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:507)
/// Calls: cxx-internal:engine_collision_gjk.c.o:_cross3, cxx-internal:engine_collision_gjk.c.o:_dot3, cxx-internal:engine_collision_gjk.c.o:_scl3, cxx-internal:engine_collision_gjk.c.o:_sub3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn project_origin_plane(res: *mut f64, v1: *const f64, v2: *const f64, v3: *const f64) -> i32 {
    todo!() // cxx-internal:engine_collision_gjk.c.o:_projectOriginPlane
}

/// C: projectOriginLine (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:544)
/// Calls: cxx-internal:engine_collision_gjk.c.o:_dot3, cxx-internal:engine_collision_gjk.c.o:_sub3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn project_origin_line(res: *mut f64, v1: *const f64, v2: *const f64) {
    todo!() // cxx-internal:engine_collision_gjk.c.o:_projectOriginLine
}

/// C: sameSign2 (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:556)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn same_sign2(a: f64, b: f64) -> i32 {
    todo!() // cxx-internal:engine_collision_gjk.c.o:_sameSign2
}

/// C: replaceSimplex3 (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:849)
#[allow(unused_variables, non_snake_case)]
pub fn replace_simplex3(pt: *mut Polytope, status: *mut mjCCDStatus, v1: i32, v2: i32, v3: i32) {
    todo!() // cxx-internal:engine_collision_gjk.c.o:_replaceSimplex3
}

/// C: sameSide (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:864)
/// Calls: cxx-internal:engine_collision_gjk.c.o:_cross3, cxx-internal:engine_collision_gjk.c.o:_dot3, cxx-internal:engine_collision_gjk.c.o:_scl3, cxx-internal:engine_collision_gjk.c.o:_sub3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn same_side(p0: *const f64, p1: *const f64, p2: *const f64, p3: *const f64) -> i32 {
    todo!() // cxx-internal:engine_collision_gjk.c.o:_sameSide
}

/// C: testTetra (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:883)
/// Calls: cxx-internal:engine_collision_gjk.c.o:_sameSide
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn test_tetra(p0: *const f64, p1: *const f64, p2: *const f64, p3: *const f64) -> i32 {
    todo!() // cxx-internal:engine_collision_gjk.c.o:_testTetra
}

/// C: rotmat (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:893)
/// Calls: cxx-internal:engine_collision_gjk.c.o:_norm3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn rotmat(R: *mut f64, axis: *const f64) {
    todo!() // cxx-internal:engine_collision_gjk.c.o:_rotmat
}

/// C: rayTriangle (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:911)
/// Calls: cxx-internal:engine_collision_gjk.c.o:_det3, cxx-internal:engine_collision_gjk.c.o:_sub3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn ray_triangle(v1: *const f64, v2: *const f64, v3: *const f64, v4: *const f64, v5: *const f64) -> i32 {
    todo!() // cxx-internal:engine_collision_gjk.c.o:_rayTriangle
}

/// C: triAffineCoord (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:1016)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn tri_affine_coord(lambda: *mut f64, v1: *const f64, v2: *const f64, v3: *const f64, p: *const f64) {
    todo!() // cxx-internal:engine_collision_gjk.c.o:_triAffineCoord
}

/// C: triPointIntersect (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:1061)
/// Calls: cxx-internal:engine_collision_gjk.c.o:_norm3, cxx-internal:engine_collision_gjk.c.o:_sub3, cxx-internal:engine_collision_gjk.c.o:_triAffineCoord
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn tri_point_intersect(v1: *const f64, v2: *const f64, v3: *const f64, p: *const f64) -> i32 {
    todo!() // cxx-internal:engine_collision_gjk.c.o:_triPointIntersect
}

/// C: deleteFace (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:1216)
#[allow(unused_variables, non_snake_case)]
pub fn delete_face(pt: *mut Polytope, face: *mut Face) {
    todo!() // cxx-internal:engine_collision_gjk.c.o:_deleteFace
}

/// C: maxFaces (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:1226)
#[allow(unused_variables, non_snake_case)]
pub fn max_faces(pt: *mut Polytope) -> i32 {
    todo!() // cxx-internal:engine_collision_gjk.c.o:_maxFaces
}

/// C: getEdge (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:1270)
#[allow(unused_variables, non_snake_case)]
pub fn get_edge(face: *mut Face, vertex: i32) -> i32 {
    todo!() // cxx-internal:engine_collision_gjk.c.o:_getEdge
}

/// C: horizonRec (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:1279)
/// Calls: cxx-internal:engine_collision_gjk.c.o:_deleteFace, cxx-internal:engine_collision_gjk.c.o:_dot3, cxx-internal:engine_collision_gjk.c.o:_getEdge
#[allow(unused_variables, non_snake_case)]
pub fn horizon_rec(pt: *mut Polytope, face: *mut Face, e: i32) -> i32 {
    todo!() // cxx-internal:engine_collision_gjk.c.o:_horizonRec
}

/// C: horizon (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:1303)
/// Calls: cxx-internal:engine_collision_gjk.c.o:_deleteFace, cxx-internal:engine_collision_gjk.c.o:_getEdge, cxx-internal:engine_collision_gjk.c.o:_horizonRec
#[allow(unused_variables, non_snake_case)]
pub fn horizon(pt: *mut Polytope, face: *mut Face) {
    todo!() // cxx-internal:engine_collision_gjk.c.o:_horizon
}

/// C: epaWitness (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:1331)
/// Calls: cxx-internal:engine_collision_gjk.c.o:_lincomb, cxx-internal:engine_collision_gjk.c.o:_triAffineCoord
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn epa_witness(pt: *const Polytope, face: *const Face, x1: *mut f64, x2: *mut f64) -> f64 {
    todo!() // cxx-internal:engine_collision_gjk.c.o:_epaWitness
}

/// C: area4 (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:1505)
/// Calls: cxx-internal:engine_collision_gjk.c.o:_add3, cxx-internal:engine_collision_gjk.c.o:_cross3, cxx-internal:engine_collision_gjk.c.o:_norm3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn area4(a: *const f64, b: *const f64, c: *const f64, d: *const f64) -> f64 {
    todo!() // cxx-internal:engine_collision_gjk.c.o:_area4
}

/// C: next (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:1520)
/// Calls: cxx:__ZNK7mjCMesh5nvertEv
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn next(polygon: *mut f64, nvert: i32, curr: *mut f64) -> *mut f64 {
    todo!() // cxx-internal:engine_collision_gjk.c.o:_next
}

/// C: polygonQuad (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:1529)
/// Calls: cxx-internal:engine_collision_gjk.c.o:_area4, cxx-internal:engine_collision_gjk.c.o:_next
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn polygon_quad(res: *mut *mut f64, polygon: *mut f64, nvert: i32) {
    todo!() // cxx-internal:engine_collision_gjk.c.o:_polygonQuad
}

/// C: planeNormal (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:1577)
/// Calls: cxx-internal:engine_collision_gjk.c.o:_add3, cxx-internal:engine_collision_gjk.c.o:_cross3, cxx-internal:engine_collision_gjk.c.o:_dot3, cxx-internal:engine_collision_gjk.c.o:_sub3, cxx:_mju_normalize3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn plane_normal(res: *mut f64, v1: *const f64, v2: *const f64, n: *const f64) -> f64 {
    todo!() // cxx-internal:engine_collision_gjk.c.o:_planeNormal
}

/// C: halfspace (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:1592)
/// Calls: cxx-internal:engine_collision_gjk.c.o:_dot3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn halfspace(a: *const f64, n: *const f64, p: *const f64) -> i32 {
    todo!() // cxx-internal:engine_collision_gjk.c.o:_halfspace
}

/// C: planeIntersect (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:1599)
/// Calls: cxx-internal:engine_collision_gjk.c.o:_dot3, cxx-internal:engine_collision_gjk.c.o:_sub3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn plane_intersect(res: *mut f64, pn: *const f64, pd: f64, a: *const f64, b: *const f64) -> f64 {
    todo!() // cxx-internal:engine_collision_gjk.c.o:_planeIntersect
}

/// C: polygonClip (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:1616)
/// Calls: cxx-internal:engine_collision_gjk.c.o:_copy3, cxx-internal:engine_collision_gjk.c.o:_dot3, cxx-internal:engine_collision_gjk.c.o:_halfspace, cxx-internal:engine_collision_gjk.c.o:_planeIntersect, cxx-internal:engine_collision_gjk.c.o:_planeNormal, cxx-internal:engine_collision_gjk.c.o:_polygonQuad, cxx-internal:engine_collision_gjk.c.o:_sub3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn polygon_clip(status: *mut mjCCDStatus, face1: *const f64, nface1: i32, face2: *const f64, nface2: i32, n: *const f64, dir: *const f64) {
    todo!() // cxx-internal:engine_collision_gjk.c.o:_polygonClip
}

/// C: globalcoord (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:1744)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn globalcoord(res: *mut f64, mat: *const f64, pos: *const f64, l1: f64, l2: f64, l3: f64) {
    todo!() // cxx-internal:engine_collision_gjk.c.o:_globalcoord
}

/// C: intersect (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:1759)
#[allow(unused_variables, non_snake_case)]
pub fn intersect(res: *mut i32, arr1: *const i32, arr2: *const i32, n: i32, m: i32) -> i32 {
    todo!() // cxx-internal:engine_collision_gjk.c.o:_intersect
}

/// C: meshNormals (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:1774)
/// Calls: cxx-internal:engine_collision_gjk.c.o:_globalcoord, cxx-internal:engine_collision_gjk.c.o:_intersect
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mesh_normals(res: *mut f64, resind: *mut i32, dim: i32, obj: *mut mjCCDObj, v1: i32, v2: i32, v3: i32) -> i32 {
    todo!() // cxx-internal:engine_collision_gjk.c.o:_meshNormals
}

/// C: meshEdgeNormals (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:1840)
/// Calls: cxx-internal:engine_collision_gjk.c.o:_copy3, cxx-internal:engine_collision_gjk.c.o:_globalcoord, cxx-internal:engine_collision_gjk.c.o:_sub3, cxx:_mju_normalize3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mesh_edge_normals(res: *mut f64, endverts: *mut f64, dim: i32, obj: *mut mjCCDObj, v1: *const f64, v2: *const f64, v1i: i32, v2i: i32) -> i32 {
    todo!() // cxx-internal:engine_collision_gjk.c.o:_meshEdgeNormals
}

/// C: boxNormals2 (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:1885)
/// Calls: cxx-internal:engine_collision_gjk.c.o:_dot3, cxx-internal:engine_collision_gjk.c.o:_globalcoord, cxx-internal:engine_collision_gjk.c.o:_scl3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn box_normals2(res: *mut f64, resind: *mut i32, mat: *const f64, n: *const f64) -> i32 {
    todo!() // cxx-internal:engine_collision_gjk.c.o:_boxNormals2
}

/// C: boxNormals (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:1911)
/// Calls: cxx-internal:engine_collision_gjk.c.o:_boxNormals2, cxx-internal:engine_collision_gjk.c.o:_globalcoord
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn box_normals(res: *mut f64, resind: *mut i32, dim: i32, obj: *mut mjCCDObj, v1: i32, v2: i32, v3: i32, dir: *const f64) -> i32 {
    todo!() // cxx-internal:engine_collision_gjk.c.o:_boxNormals
}

/// C: boxEdgeNormals (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:1965)
/// Calls: cxx-internal:engine_collision_gjk.c.o:_copy3, cxx-internal:engine_collision_gjk.c.o:_globalcoord, cxx-internal:engine_collision_gjk.c.o:_sub3, cxx:_mju_normalize3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn box_edge_normals(res: *mut f64, endverts: *mut f64, dim: i32, obj: *mut mjCCDObj, v1: *const f64, v2: *const f64, v1i: i32, v2i: i32) -> i32 {
    todo!() // cxx-internal:engine_collision_gjk.c.o:_boxEdgeNormals
}

/// C: boxFace (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:2002)
/// Calls: cxx-internal:engine_collision_gjk.c.o:_globalcoord
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn box_face(res: *mut f64, obj: *mut mjCCDObj, idx: i32) -> i32 {
    todo!() // cxx-internal:engine_collision_gjk.c.o:_boxFace
}

/// C: meshFace (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:2052)
/// Calls: cxx-internal:engine_collision_gjk.c.o:_globalcoord
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mesh_face(res: *mut f64, obj: *mut mjCCDObj, idx: i32) -> i32 {
    todo!() // cxx-internal:engine_collision_gjk.c.o:_meshFace
}

/// C: alignedFaces (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:2072)
/// Calls: cxx-internal:engine_collision_gjk.c.o:_dot3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn aligned_faces(res: *mut i32, v: *const f64, nv: i32, w: *const f64, nw: i32) -> i32 {
    todo!() // cxx-internal:engine_collision_gjk.c.o:_alignedFaces
}

/// C: alignedFaceEdge (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:2088)
/// Calls: cxx-internal:engine_collision_gjk.c.o:_dot3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn aligned_face_edge(res: *mut i32, edge: *const f64, nedge: i32, face: *const f64, nface: i32) -> i32 {
    todo!() // cxx-internal:engine_collision_gjk.c.o:_alignedFaceEdge
}

/// C: simplexDim (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:2104)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn simplex_dim(v1i: *mut i32, v2i: *mut i32, v3i: *mut i32, v1: *mut *mut f64, v2: *mut *mut f64, v3: *mut *mut f64) -> i32 {
    todo!() // cxx-internal:engine_collision_gjk.c.o:_simplexDim
}

/// C: multicontact (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:2122)
/// Calls: cxx-internal:engine_collision_gjk.c.o:_alignedFaceEdge, cxx-internal:engine_collision_gjk.c.o:_alignedFaces, cxx-internal:engine_collision_gjk.c.o:_boxEdgeNormals, cxx-internal:engine_collision_gjk.c.o:_boxFace, cxx-internal:engine_collision_gjk.c.o:_boxNormals, cxx-internal:engine_collision_gjk.c.o:_copy3, cxx-internal:engine_collision_gjk.c.o:_meshEdgeNormals, cxx-internal:engine_collision_gjk.c.o:_meshFace, cxx-internal:engine_collision_gjk.c.o:_meshNormals, cxx-internal:engine_collision_gjk.c.o:_norm3, cxx-internal:engine_collision_gjk.c.o:_polygonClip, cxx-internal:engine_collision_gjk.c.o:_scl3, cxx-internal:engine_collision_gjk.c.o:_simplexDim, cxx-internal:engine_collision_gjk.c.o:_sub3
#[allow(unused_variables, non_snake_case)]
pub fn multicontact(pt: *mut Polytope, face: *mut Face, status: *mut mjCCDStatus, obj1: *mut mjCCDObj, obj2: *mut mjCCDObj) {
    todo!() // cxx-internal:engine_collision_gjk.c.o:_multicontact
}

/// C: inflate (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.c:2264)
/// Calls: cxx-internal:engine_collision_gjk.c.o:_sub3, cxx:_mju_normalize3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn inflate(status: *mut mjCCDStatus, margin1: f64, margin2: f64) {
    todo!() // cxx-internal:engine_collision_gjk.c.o:_inflate
}

/// C: mjc_ccdSize (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.h:105)
/// Calls: cxx-internal:engine_collision_gjk.c.o:_align8
#[allow(unused_variables, non_snake_case)]
pub fn mjc_ccd_size(iterations: i32) -> usize {
    todo!() // cxx:_mjc_ccdSize
}

/// C: mjc_ccd (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_gjk.h:108)
/// Calls: cxx-internal:engine_collision_gjk.c.o:_align8, cxx-internal:engine_collision_gjk.c.o:_epa, cxx-internal:engine_collision_gjk.c.o:_gjk, cxx-internal:engine_collision_gjk.c.o:_inflate, cxx-internal:engine_collision_gjk.c.o:_multicontact, cxx-internal:engine_collision_gjk.c.o:_polytope2, cxx-internal:engine_collision_gjk.c.o:_polytope3, cxx-internal:engine_collision_gjk.c.o:_polytope4
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_ccd(config: *const mjCCDConfig, status: *mut mjCCDStatus, obj1: *mut mjCCDObj, obj2: *mut mjCCDObj) -> f64 {
    todo!() // cxx:_mjc_ccd
}

