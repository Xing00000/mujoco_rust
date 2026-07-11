//! Port of: engine/engine_collision_gjk.c
//! IR hash: c6d98e4f4b63b7f2
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: align8 (engine/engine_collision_gjk.c:49)
#[allow(unused_variables, non_snake_case)]
pub fn align8(size: usize) -> usize {
    // WARNING: signature changed — verify body
    // Previous params: (size : usize)
    // Previous return: usize
    todo ! ()
}

/// C: subdistance (engine/engine_collision_gjk.c:56)
/// Calls: S1D, S2D, S3D
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn subdistance(lambda: *mut f64, n: i32, simplex: *const Vertex) {
    // WARNING: signature changed — verify body
    // Previous params: (lambda : * mut f64, n : i32, simplex : * const Vertex)
    // Previous return: ()
    todo ! ()
}

/// C: S3D (engine/engine_collision_gjk.c:60)
/// Calls: S2D, det3, dot3, lincomb, sameSign2
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn s3d(lambda: *mut f64, s1: *const f64, s2: *const f64, s3: *const f64, s4: *const f64) {
    // WARNING: signature changed — verify body
    // Previous params: (lambda : * mut f64, s1 : * const f64, s2 : * const f64, s3 : * const f64, s4 : * const f64)
    // Previous return: ()
    todo ! ()
}

/// C: S2D (engine/engine_collision_gjk.c:62)
/// Calls: S1D, dot3, lincomb, projectOriginPlane, sameSign2
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn s2d(lambda: *mut f64, s1: *const f64, s2: *const f64, s3: *const f64) {
    // WARNING: signature changed — verify body
    // Previous params: (lambda : * mut f64, s1 : * const f64, s2 : * const f64, s3 : * const f64)
    // Previous return: ()
    todo ! ()
}

/// C: S1D (engine/engine_collision_gjk.c:63)
/// Calls: projectOriginLine, sameSign2
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn s1d(lambda: *mut f64, s1: *const f64, s2: *const f64) {
    // WARNING: signature changed — verify body
    // Previous params: (lambda : * mut f64, s1 : * const f64, s2 : * const f64)
    // Previous return: ()
    todo ! ()
}

/// C: gjkSupport (engine/engine_collision_gjk.c:66)
/// Calls: scl3, support
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn gjk_support(v: *mut Vertex, obj1: *mut mjCCDObj, obj2: *mut mjCCDObj, x_k: *const f64, x_norm: f64) {
    // WARNING: signature changed — verify body
    // Previous params: (v : * mut Vertex, obj1 : * mut mjCCDObj, obj2 : * mut mjCCDObj, x_k : * const f64, x_norm : f64)
    // Previous return: ()
    todo ! ()
}

/// C: lincomb (engine/engine_collision_gjk.c:70)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn lincomb(res: *mut f64, coef: *const f64, n: i32, v1: *const f64, v2: *const f64, v3: *const f64, v4: *const f64) {
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut f64, coef : * const f64, n : i32, v1 : * const f64, v2 : * const f64, v3 : * const f64, v4 : * const f64)
    // Previous return: ()
    todo ! ()
}

/// C: epaSupport (engine/engine_collision_gjk.c:108)
/// Calls: scl3, support
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn epa_support(pt: *mut Polytope, obj1: *mut mjCCDObj, obj2: *mut mjCCDObj, d: *const f64, dnorm: f64) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (pt : * mut Polytope, obj1 : * mut mjCCDObj, obj2 : * mut mjCCDObj, d : * const f64, dnorm : f64)
    // Previous return: i32
    todo ! ()
}

/// C: insertVertex (engine/engine_collision_gjk.c:112)
#[allow(unused_variables, non_snake_case)]
pub fn insert_vertex(pt: *mut Polytope, v: *const Vertex) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (pt : * mut Polytope, v : * const Vertex)
    // Previous return: i32
    todo ! ()
}

/// C: attachFace (engine/engine_collision_gjk.c:115)
/// Calls: dot3, projectOriginPlane, scl3, sub3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn attach_face(pt: *mut Polytope, v1: i32, v2: i32, v3: i32, adj1: i32, adj2: i32, adj3: i32) -> f64 {
    // WARNING: signature changed — verify body
    // Previous params: (pt : * mut Polytope, v1 : i32, v2 : i32, v3 : i32, adj1 : i32, adj2 : i32, adj3 : i32)
    // Previous return: f64
    todo ! ()
}

/// C: gjkIntersect (engine/engine_collision_gjk.c:119)
/// Calls: dot3, gjkIntersectSupport, signedDistance
#[allow(unused_variables, non_snake_case)]
pub fn gjk_intersect(status: *mut mjCCDStatus, obj1: *mut mjCCDObj, obj2: *mut mjCCDObj) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (status : * mut mjCCDStatus, obj1 : * mut mjCCDObj, obj2 : * mut mjCCDObj)
    // Previous return: i32
    todo ! ()
}

/// C: polytope2 (engine/engine_collision_gjk.c:122)
/// Calls: add3, attachFace, cross3, epaSupport, insertVertex, mju_mulMatVec3, norm3, polytope3, rayTriangle, replaceSimplex3, rotmat, scl3, sub3
#[allow(unused_variables, non_snake_case)]
pub fn polytope2(pt: *mut Polytope, status: *mut mjCCDStatus, obj1: *mut mjCCDObj, obj2: *mut mjCCDObj) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (pt : * mut Polytope, status : * mut mjCCDStatus, obj1 : * mut mjCCDObj, obj2 : * mut mjCCDObj)
    // Previous return: i32
    todo ! ()
}

/// C: polytope3 (engine/engine_collision_gjk.c:123)
/// Calls: add3, attachFace, cross3, epaSupport, insertVertex, norm3, scl3, sub3, testTetra, triPointIntersect
#[allow(unused_variables, non_snake_case)]
pub fn polytope3(pt: *mut Polytope, status: *mut mjCCDStatus, obj1: *mut mjCCDObj, obj2: *mut mjCCDObj) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (pt : * mut Polytope, status : * mut mjCCDStatus, obj1 : * mut mjCCDObj, obj2 : * mut mjCCDObj)
    // Previous return: i32
    todo ! ()
}

/// C: polytope4 (engine/engine_collision_gjk.c:124)
/// Calls: add3, attachFace, insertVertex, polytope3, replaceSimplex3, scl3, testTetra
#[allow(unused_variables, non_snake_case)]
pub fn polytope4(pt: *mut Polytope, status: *mut mjCCDStatus, obj1: *mut mjCCDObj, obj2: *mut mjCCDObj) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (pt : * mut Polytope, status : * mut mjCCDStatus, obj1 : * mut mjCCDObj, obj2 : * mut mjCCDObj)
    // Previous return: i32
    todo ! ()
}

/// C: epa (engine/engine_collision_gjk.c:128)
/// Calls: attachFace, discreteGeoms, dot3, epaSupport, epaWitness, horizon, maxFaces, mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn epa(status: *mut mjCCDStatus, pt: *mut Polytope, obj1: *mut mjCCDObj, obj2: *mut mjCCDObj) -> *mut Face {
    // WARNING: signature changed — verify body
    // Previous params: (status : * mut mjCCDStatus, pt : * mut Polytope, obj1 : * mut mjCCDObj, obj2 : * mut mjCCDObj)
    // Previous return: * mut Face
    todo ! ()
}

/// C: equal3 (engine/engine_collision_gjk.c:133)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn equal3(v1: *const f64, v2: *const f64) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (v1 : * const f64, v2 : * const f64)
    // Previous return: i32
    todo ! ()
}

/// C: add3 (engine/engine_collision_gjk.c:140)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn add3(res: *mut f64, v1: *const f64, v2: *const f64) {
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut f64, v1 : * const f64, v2 : * const f64)
    // Previous return: ()
    todo ! ()
}

/// C: sub3 (engine/engine_collision_gjk.c:145)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn sub3(res: *mut f64, v1: *const f64, v2: *const f64) {
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut f64, v1 : * const f64, v2 : * const f64)
    // Previous return: ()
    todo ! ()
}

/// C: dot3 (engine/engine_collision_gjk.c:150)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn dot3(v1: *const f64, v2: *const f64) -> f64 {
    // WARNING: signature changed — verify body
    // Previous params: (v1 : * const f64, v2 : * const f64)
    // Previous return: f64
    todo ! ()
}

/// C: norm3 (engine/engine_collision_gjk.c:155)
/// Calls: dot3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn norm3(v: *const f64) -> f64 {
    // WARNING: signature changed — verify body
    // Previous params: (v : * const f64)
    // Previous return: f64
    todo ! ()
}

/// C: copy3 (engine/engine_collision_gjk.c:160)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn copy3(res: *mut f64, v: *const f64) {
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut f64, v : * const f64)
    // Previous return: ()
    todo ! ()
}

/// C: scl3 (engine/engine_collision_gjk.c:165)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn scl3(res: *mut f64, v: *const f64, s: f64) {
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut f64, v : * const f64, s : f64)
    // Previous return: ()
    todo ! ()
}

/// C: cross3 (engine/engine_collision_gjk.c:170)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn cross3(res: *mut f64, v1: *const f64, v2: *const f64) {
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut f64, v1 : * const f64, v2 : * const f64)
    // Previous return: ()
    todo ! ()
}

/// C: det3 (engine/engine_collision_gjk.c:177)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn det3(v1: *const f64, v2: *const f64, v3: *const f64) -> f64 {
    // WARNING: signature changed — verify body
    // Previous params: (v1 : * const f64, v2 : * const f64, v3 : * const f64)
    // Previous return: f64
    todo ! ()
}

/// C: discreteGeoms (engine/engine_collision_gjk.c:188)
#[allow(unused_variables, non_snake_case)]
pub fn discrete_geoms(obj1: *mut mjCCDObj, obj2: *mut mjCCDObj) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (obj1 : * mut mjCCDObj, obj2 : * mut mjCCDObj)
    // Previous return: i32
    todo ! ()
}

/// C: gjk (engine/engine_collision_gjk.c:200)
/// Calls: copy3, discreteGeoms, dot3, equal3, gjkIntersect, gjkSupport, lincomb, sub3, subdistance
#[allow(unused_variables, non_snake_case)]
pub fn gjk(status: *mut mjCCDStatus, obj1: *mut mjCCDObj, obj2: *mut mjCCDObj) {
    // WARNING: signature changed — verify body
    // Previous params: (status : * mut mjCCDStatus, obj1 : * mut mjCCDObj, obj2 : * mut mjCCDObj)
    // Previous return: ()
    todo ! ()
}

/// C: support (engine/engine_collision_gjk.c:334)
/// Calls: sub3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn support(v: *mut Vertex, obj1: *mut mjCCDObj, obj2: *mut mjCCDObj, dir: *const f64, dir_neg: *const f64) {
    // WARNING: signature changed — verify body
    // Previous params: (v : * mut Vertex, obj1 : * mut mjCCDObj, obj2 : * mut mjCCDObj, dir : * const f64, dir_neg : * const f64)
    // Previous return: ()
    todo ! ()
}

/// C: gjkIntersectSupport (engine/engine_collision_gjk.c:396)
/// Calls: support
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn gjk_intersect_support(v: *mut Vertex, obj1: *mut mjCCDObj, obj2: *mut mjCCDObj, dir: *const f64) {
    // WARNING: signature changed — verify body
    // Previous params: (v : * mut Vertex, obj1 : * mut mjCCDObj, obj2 : * mut mjCCDObj, dir : * const f64)
    // Previous return: ()
    todo ! ()
}

/// C: signedDistance (engine/engine_collision_gjk.c:404)
/// Calls: cross3, dot3, scl3, sub3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn signed_distance(normal: *mut f64, v1: *const Vertex, v2: *const Vertex, v3: *const Vertex) -> f64 {
    // WARNING: signature changed — verify body
    // Previous params: (normal : * mut f64, v1 : * const Vertex, v2 : * const Vertex, v3 : * const Vertex)
    // Previous return: f64
    todo ! ()
}

/// C: projectOriginPlane (engine/engine_collision_gjk.c:507)
/// Calls: cross3, dot3, scl3, sub3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn project_origin_plane(res: *mut f64, v1: *const f64, v2: *const f64, v3: *const f64) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut f64, v1 : * const f64, v2 : * const f64, v3 : * const f64)
    // Previous return: i32
    todo ! ()
}

/// C: projectOriginLine (engine/engine_collision_gjk.c:544)
/// Calls: dot3, sub3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn project_origin_line(res: *mut f64, v1: *const f64, v2: *const f64) {
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut f64, v1 : * const f64, v2 : * const f64)
    // Previous return: ()
    todo ! ()
}

/// C: sameSign2 (engine/engine_collision_gjk.c:556)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn same_sign2(a: f64, b: f64) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (a : f64, b : f64)
    // Previous return: i32
    todo ! ()
}

/// C: replaceSimplex3 (engine/engine_collision_gjk.c:849)
#[allow(unused_variables, non_snake_case)]
pub fn replace_simplex3(pt: *mut Polytope, status: *mut mjCCDStatus, v1: i32, v2: i32, v3: i32) {
    // WARNING: signature changed — verify body
    // Previous params: (pt : * mut Polytope, status : * mut mjCCDStatus, v1 : i32, v2 : i32, v3 : i32)
    // Previous return: ()
    todo ! ()
}

/// C: sameSide (engine/engine_collision_gjk.c:864)
/// Calls: cross3, dot3, scl3, sub3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn same_side(p0: *const f64, p1: *const f64, p2: *const f64, p3: *const f64) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (p0 : * const f64, p1 : * const f64, p2 : * const f64, p3 : * const f64)
    // Previous return: i32
    todo ! ()
}

/// C: testTetra (engine/engine_collision_gjk.c:883)
/// Calls: sameSide
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn test_tetra(p0: *const f64, p1: *const f64, p2: *const f64, p3: *const f64) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (p0 : * const f64, p1 : * const f64, p2 : * const f64, p3 : * const f64)
    // Previous return: i32
    todo ! ()
}

/// C: rotmat (engine/engine_collision_gjk.c:893)
/// Calls: norm3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn rotmat(R: *mut f64, axis: *const f64) {
    // WARNING: signature changed — verify body
    // Previous params: (R : * mut f64, axis : * const f64)
    // Previous return: ()
    todo ! ()
}

/// C: rayTriangle (engine/engine_collision_gjk.c:911)
/// Calls: det3, sub3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn ray_triangle(v1: *const f64, v2: *const f64, v3: *const f64, v4: *const f64, v5: *const f64) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (v1 : * const f64, v2 : * const f64, v3 : * const f64, v4 : * const f64, v5 : * const f64)
    // Previous return: i32
    todo ! ()
}

/// C: triAffineCoord (engine/engine_collision_gjk.c:1016)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn tri_affine_coord(lambda: *mut f64, v1: *const f64, v2: *const f64, v3: *const f64, p: *const f64) {
    // WARNING: signature changed — verify body
    // Previous params: (lambda : * mut f64, v1 : * const f64, v2 : * const f64, v3 : * const f64, p : * const f64)
    // Previous return: ()
    todo ! ()
}

/// C: triPointIntersect (engine/engine_collision_gjk.c:1061)
/// Calls: norm3, sub3, triAffineCoord
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn tri_point_intersect(v1: *const f64, v2: *const f64, v3: *const f64, p: *const f64) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (v1 : * const f64, v2 : * const f64, v3 : * const f64, p : * const f64)
    // Previous return: i32
    todo ! ()
}

/// C: deleteFace (engine/engine_collision_gjk.c:1216)
#[allow(unused_variables, non_snake_case)]
pub fn delete_face(pt: *mut Polytope, face: *mut Face) {
    // WARNING: signature changed — verify body
    // Previous params: (pt : * mut Polytope, face : * mut Face)
    // Previous return: ()
    todo ! ()
}

/// C: maxFaces (engine/engine_collision_gjk.c:1226)
#[allow(unused_variables, non_snake_case)]
pub fn max_faces(pt: *mut Polytope) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (pt : * mut Polytope)
    // Previous return: i32
    todo ! ()
}

/// C: addEdge (engine/engine_collision_gjk.c:1263)
#[allow(unused_variables, non_snake_case)]
pub fn add_edge(pt: *mut Polytope, index: i32, edge: i32) {
    // WARNING: signature changed — verify body
    // Previous params: (pt : * mut Polytope, index : i32, edge : i32)
    // Previous return: ()
    todo ! ()
}

/// C: getEdge (engine/engine_collision_gjk.c:1270)
#[allow(unused_variables, non_snake_case)]
pub fn get_edge(face: *mut Face, vertex: i32) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (face : * mut Face, vertex : i32)
    // Previous return: i32
    todo ! ()
}

/// C: horizonRec (engine/engine_collision_gjk.c:1279)
/// Calls: addEdge, deleteFace, dot3, getEdge
#[allow(unused_variables, non_snake_case)]
pub fn horizon_rec(pt: *mut Polytope, face: *mut Face, e: i32) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (pt : * mut Polytope, face : * mut Face, e : i32)
    // Previous return: i32
    todo ! ()
}

/// C: horizon (engine/engine_collision_gjk.c:1303)
/// Calls: addEdge, deleteFace, getEdge, horizonRec
#[allow(unused_variables, non_snake_case)]
pub fn horizon(pt: *mut Polytope, face: *mut Face) {
    // WARNING: signature changed — verify body
    // Previous params: (pt : * mut Polytope, face : * mut Face)
    // Previous return: ()
    todo ! ()
}

/// C: epaWitness (engine/engine_collision_gjk.c:1331)
/// Calls: lincomb, triAffineCoord
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn epa_witness(pt: *const Polytope, face: *const Face, x1: *mut f64, x2: *mut f64) -> f64 {
    // WARNING: signature changed — verify body
    // Previous params: (pt : * const Polytope, face : * const Face, x1 : * mut f64, x2 : * mut f64)
    // Previous return: f64
    todo ! ()
}

/// C: area4 (engine/engine_collision_gjk.c:1505)
/// Calls: add3, cross3, norm3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn area4(a: *const f64, b: *const f64, c: *const f64, d: *const f64) -> f64 {
    // WARNING: signature changed — verify body
    // Previous params: (a : * const f64, b : * const f64, c : * const f64, d : * const f64)
    // Previous return: f64
    todo ! ()
}

/// C: next (engine/engine_collision_gjk.c:1520)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn next(polygon: *mut f64, nvert: i32, curr: *mut f64) -> *mut f64 {
    // WARNING: signature changed — verify body
    // Previous params: (polygon : * mut f64, nvert : i32, curr : * mut f64)
    // Previous return: * mut f64
    todo ! ()
}

/// C: polygonQuad (engine/engine_collision_gjk.c:1529)
/// Calls: area4, next
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn polygon_quad(res: *mut *mut f64, polygon: *mut f64, nvert: i32) {
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut * mut f64, polygon : * mut f64, nvert : i32)
    // Previous return: ()
    todo ! ()
}

/// C: planeNormal (engine/engine_collision_gjk.c:1577)
/// Calls: add3, cross3, dot3, mju_normalize3, sub3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn plane_normal(res: *mut f64, v1: *const f64, v2: *const f64, n: *const f64) -> f64 {
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut f64, v1 : * const f64, v2 : * const f64, n : * const f64)
    // Previous return: f64
    todo ! ()
}

/// C: halfspace (engine/engine_collision_gjk.c:1592)
/// Calls: dot3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn halfspace(a: *const f64, n: *const f64, p: *const f64) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (a : * const f64, n : * const f64, p : * const f64)
    // Previous return: i32
    todo ! ()
}

/// C: planeIntersect (engine/engine_collision_gjk.c:1599)
/// Calls: dot3, sub3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn plane_intersect(res: *mut f64, pn: *const f64, pd: f64, a: *const f64, b: *const f64) -> f64 {
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut f64, pn : * const f64, pd : f64, a : * const f64, b : * const f64)
    // Previous return: f64
    todo ! ()
}

/// C: polygonClip (engine/engine_collision_gjk.c:1616)
/// Calls: copy3, dot3, halfspace, planeIntersect, planeNormal, polygonQuad, sub3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn polygon_clip(status: *mut mjCCDStatus, face1: *const f64, nface1: i32, face2: *const f64, nface2: i32, n: *const f64, dir: *const f64) {
    // WARNING: signature changed — verify body
    // Previous params: (status : * mut mjCCDStatus, face1 : * const f64, nface1 : i32, face2 : * const f64, nface2 : i32, n : * const f64, dir : * const f64)
    // Previous return: ()
    todo ! ()
}

/// C: globalcoord (engine/engine_collision_gjk.c:1744)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn globalcoord(res: *mut f64, mat: *const f64, pos: *const f64, l1: f64, l2: f64, l3: f64) {
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut f64, mat : * const f64, pos : * const f64, l1 : f64, l2 : f64, l3 : f64)
    // Previous return: ()
    todo ! ()
}

/// C: intersect (engine/engine_collision_gjk.c:1759)
#[allow(unused_variables, non_snake_case)]
pub fn intersect(res: *mut i32, arr1: *const i32, arr2: *const i32, n: i32, m: i32) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut i32, arr1 : * const i32, arr2 : * const i32, n : i32, m : i32)
    // Previous return: i32
    todo ! ()
}

/// C: meshNormals (engine/engine_collision_gjk.c:1774)
/// Calls: globalcoord, intersect
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mesh_normals(res: *mut f64, resind: *mut i32, dim: i32, obj: *mut mjCCDObj, v1: i32, v2: i32, v3: i32) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut f64, resind : * mut i32, dim : i32, obj : * mut mjCCDObj, v1 : i32, v2 : i32, v3 : i32)
    // Previous return: i32
    todo ! ()
}

/// C: meshEdgeNormals (engine/engine_collision_gjk.c:1840)
/// Calls: copy3, globalcoord, mju_normalize3, sub3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mesh_edge_normals(res: *mut f64, endverts: *mut f64, dim: i32, obj: *mut mjCCDObj, v1: *const f64, v2: *const f64, v1i: i32, v2i: i32) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut f64, endverts : * mut f64, dim : i32, obj : * mut mjCCDObj, v1 : * const f64, v2 : * const f64, v1i : i32, v2i : i32)
    // Previous return: i32
    todo ! ()
}

/// C: boxNormals2 (engine/engine_collision_gjk.c:1885)
/// Calls: dot3, globalcoord, scl3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn box_normals2(res: *mut f64, resind: *mut i32, mat: *const f64, n: *const f64) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut f64, resind : * mut i32, mat : * const f64, n : * const f64)
    // Previous return: i32
    todo ! ()
}

/// C: boxNormals (engine/engine_collision_gjk.c:1911)
/// Calls: boxNormals2, globalcoord
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn box_normals(res: *mut f64, resind: *mut i32, dim: i32, obj: *mut mjCCDObj, v1: i32, v2: i32, v3: i32, dir: *const f64) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut f64, resind : * mut i32, dim : i32, obj : * mut mjCCDObj, v1 : i32, v2 : i32, v3 : i32, dir : * const f64)
    // Previous return: i32
    todo ! ()
}

/// C: boxEdgeNormals (engine/engine_collision_gjk.c:1965)
/// Calls: copy3, globalcoord, mju_normalize3, sub3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn box_edge_normals(res: *mut f64, endverts: *mut f64, dim: i32, obj: *mut mjCCDObj, v1: *const f64, v2: *const f64, v1i: i32, v2i: i32) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut f64, endverts : * mut f64, dim : i32, obj : * mut mjCCDObj, v1 : * const f64, v2 : * const f64, v1i : i32, v2i : i32)
    // Previous return: i32
    todo ! ()
}

/// C: boxFace (engine/engine_collision_gjk.c:2002)
/// Calls: globalcoord
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn box_face(res: *mut f64, obj: *mut mjCCDObj, idx: i32) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut f64, obj : * mut mjCCDObj, idx : i32)
    // Previous return: i32
    todo ! ()
}

/// C: meshFace (engine/engine_collision_gjk.c:2052)
/// Calls: globalcoord
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mesh_face(res: *mut f64, obj: *mut mjCCDObj, idx: i32) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut f64, obj : * mut mjCCDObj, idx : i32)
    // Previous return: i32
    todo ! ()
}

/// C: alignedFaces (engine/engine_collision_gjk.c:2072)
/// Calls: dot3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn aligned_faces(res: *mut i32, v: *const f64, nv: i32, w: *const f64, nw: i32) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut i32, v : * const f64, nv : i32, w : * const f64, nw : i32)
    // Previous return: i32
    todo ! ()
}

/// C: alignedFaceEdge (engine/engine_collision_gjk.c:2088)
/// Calls: dot3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn aligned_face_edge(res: *mut i32, edge: *const f64, nedge: i32, face: *const f64, nface: i32) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut i32, edge : * const f64, nedge : i32, face : * const f64, nface : i32)
    // Previous return: i32
    todo ! ()
}

/// C: simplexDim (engine/engine_collision_gjk.c:2104)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn simplex_dim(v1i: *mut i32, v2i: *mut i32, v3i: *mut i32, v1: *mut *mut f64, v2: *mut *mut f64, v3: *mut *mut f64) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (v1i : * mut i32, v2i : * mut i32, v3i : * mut i32, v1 : * mut * mut f64, v2 : * mut * mut f64, v3 : * mut * mut f64)
    // Previous return: i32
    todo ! ()
}

/// C: multicontact (engine/engine_collision_gjk.c:2122)
/// Calls: alignedFaceEdge, alignedFaces, boxEdgeNormals, boxFace, boxNormals, copy3, meshEdgeNormals, meshFace, meshNormals, norm3, polygonClip, scl3, simplexDim, sub3
#[allow(unused_variables, non_snake_case)]
pub fn multicontact(pt: *mut Polytope, face: *mut Face, status: *mut mjCCDStatus, obj1: *mut mjCCDObj, obj2: *mut mjCCDObj) {
    // WARNING: signature changed — verify body
    // Previous params: (pt : * mut Polytope, face : * mut Face, status : * mut mjCCDStatus, obj1 : * mut mjCCDObj, obj2 : * mut mjCCDObj)
    // Previous return: ()
    todo ! ()
}

/// C: inflate (engine/engine_collision_gjk.c:2264)
/// Calls: mju_normalize3, sub3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn inflate(status: *mut mjCCDStatus, margin1: f64, margin2: f64) {
    // WARNING: signature changed — verify body
    // Previous params: (status : * mut mjCCDStatus, margin1 : f64, margin2 : f64)
    // Previous return: ()
    todo ! ()
}

/// C: mjc_ccdSize (engine/engine_collision_gjk.h:105)
/// Calls: align8
#[allow(unused_variables, non_snake_case)]
pub fn mjc_ccd_size(iterations: i32) -> usize {
    // WARNING: signature changed — verify body
    // Previous params: (iterations : i32)
    // Previous return: usize
    todo ! ()
}

/// C: mjc_ccd (engine/engine_collision_gjk.h:108)
/// Calls: align8, epa, gjk, inflate, multicontact, polytope2, polytope3, polytope4
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_ccd(config: *const mjCCDConfig, status: *mut mjCCDStatus, obj1: *mut mjCCDObj, obj2: *mut mjCCDObj) -> f64 {
    // WARNING: signature changed — verify body
    // Previous params: (config : * const mjCCDConfig, status : * mut mjCCDStatus, obj1 : * mut mjCCDObj, obj2 : * mut mjCCDObj)
    // Previous return: f64
    todo ! ()
}

