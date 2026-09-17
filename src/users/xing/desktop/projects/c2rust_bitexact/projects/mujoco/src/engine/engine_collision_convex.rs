//! Port of: Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_convex.c
//! IR hash: 1fa44043fe383ce9
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: prism_firstdir (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_convex.c:47)
#[allow(unused_variables, non_snake_case)]
pub fn prism_firstdir(o1: *const (), o2: *const (), vec: *mut ccd_vec3_t) {
    todo!() // cxx-internal:engine_collision_convex.c.o:_prism_firstdir
}

/// C: _libccd_wrapper (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_convex.c:52)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn libccd_wrapper(m: *const mjModel, obj1: *mut mjCCDObj, obj2: *mut mjCCDObj, con: *mut mjPreContact, margin: f64) -> i32 {
    todo!() // cxx-internal:engine_collision_convex.c.o:__libccd_wrapper
}

/// C: mjc_penetration (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_convex.c:87)
/// Calls: cxx-internal:engine_collision_convex.c.o:__libccd_wrapper, cxx:_mj_freeStack, cxx:_mj_markStack, cxx:_mj_stackAllocByte, cxx:_mjc_ccd, cxx:_mjc_ccdSize, cxx:_mju_normalize3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_penetration(m: *const mjModel, d: *mut mjData, obj1: *mut mjCCDObj, obj2: *mut mjCCDObj, con: *mut mjPreContact, ncon: i32, margin: f64) -> i32 {
    todo!() // cxx-internal:engine_collision_convex.c.o:_mjc_penetration
}

/// C: mulMatTVec3 (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_convex.c:174)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mul_mat_t_vec3(res: *mut f64, mat: *const f64, dir: *const f64) {
    todo!() // cxx-internal:engine_collision_convex.c.o:_mulMatTVec3
}

/// C: localToGlobal (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_convex.c:183)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn local_to_global(res: *mut f64, mat: *const f64, dir: *const f64, pos: *const f64) {
    todo!() // cxx-internal:engine_collision_convex.c.o:_localToGlobal
}

/// C: mjc_sphereSupport (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_convex.c:202)
/// Calls: cxx-internal:render_context.c.o:_sphere
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_sphere_support(res: *mut f64, obj: *mut mjCCDObj, dir: *const f64) {
    todo!() // cxx-internal:engine_collision_convex.c.o:_mjc_sphereSupport
}

/// C: mjc_capsuleSupport (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_convex.c:231)
/// Calls: cxx-internal:engine_collision_convex.c.o:_localToGlobal, cxx-internal:engine_collision_convex.c.o:_mulMatTVec3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_capsule_support(res: *mut f64, obj: *mut mjCCDObj, dir: *const f64) {
    todo!() // cxx-internal:engine_collision_convex.c.o:_mjc_capsuleSupport
}

/// C: mjc_ellipsoidSupport (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_convex.c:256)
/// Calls: cxx-internal:engine_collision_convex.c.o:_localToGlobal, cxx-internal:engine_collision_convex.c.o:_mulMatTVec3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_ellipsoid_support(res: *mut f64, obj: *mut mjCCDObj, dir: *const f64) {
    todo!() // cxx-internal:engine_collision_convex.c.o:_mjc_ellipsoidSupport
}

/// C: mjc_cylinderSupport (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_convex.c:293)
/// Calls: cxx-internal:engine_collision_convex.c.o:_localToGlobal, cxx-internal:engine_collision_convex.c.o:_mulMatTVec3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_cylinder_support(res: *mut f64, obj: *mut mjCCDObj, dir: *const f64) {
    todo!() // cxx-internal:engine_collision_convex.c.o:_mjc_cylinderSupport
}

/// C: mjc_boxSupport (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_convex.c:317)
/// Calls: cxx-internal:engine_collision_convex.c.o:_localToGlobal, cxx-internal:engine_collision_convex.c.o:_mulMatTVec3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_box_support(res: *mut f64, obj: *mut mjCCDObj, dir: *const f64) {
    todo!() // cxx-internal:engine_collision_convex.c.o:_mjc_boxSupport
}

/// C: dot3f (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_convex.c:343)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn dot3f(a: *const f64, b: *const f32) -> f64 {
    todo!() // cxx-internal:engine_collision_convex.c.o:_dot3f
}

/// C: mjc_meshSupport (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_convex.c:349)
/// Calls: cxx-internal:engine_collision_convex.c.o:_dot3f, cxx-internal:engine_collision_convex.c.o:_localToGlobal, cxx-internal:engine_collision_convex.c.o:_mulMatTVec3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_mesh_support(res: *mut f64, obj: *mut mjCCDObj, dir: *const f64) {
    todo!() // cxx-internal:engine_collision_convex.c.o:_mjc_meshSupport
}

/// C: mjc_hillclimbSupport (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_convex.c:391)
/// Calls: cxx-internal:engine_collision_convex.c.o:_dot3f, cxx-internal:engine_collision_convex.c.o:_localToGlobal, cxx-internal:engine_collision_convex.c.o:_mulMatTVec3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_hillclimb_support(res: *mut f64, obj: *mut mjCCDObj, dir: *const f64) {
    todo!() // cxx-internal:engine_collision_convex.c.o:_mjc_hillclimbSupport
}

/// C: mjc_prism_support (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_convex.c:436)
/// Calls: cxx:_mju_dot3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_prism_support(res: *mut f64, obj: *mut mjCCDObj, dir: *const f64) {
    todo!() // cxx-internal:engine_collision_convex.c.o:_mjc_prism_support
}

/// C: mjc_flexSupport (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_convex.c:458)
/// Calls: cxx:_mju_dot3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_flex_support(res: *mut f64, obj: *mut mjCCDObj, dir: *const f64) {
    todo!() // cxx-internal:engine_collision_convex.c.o:_mjc_flexSupport
}

/// C: mjc_setCCDObjFlex (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_convex.c:790)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_set_ccd_obj_flex(obj: *mut mjCCDObj, flex: i32, elem: i32, vert: i32) {
    todo!() // cxx-internal:engine_collision_convex.c.o:_mjc_setCCDObjFlex
}

/// C: mjc_isDistinctContact (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_convex.c:798)
/// Calls: cxx:_mju_dist3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_is_distinct_contact(con: *const mjPreContact, ncon: i32, tolerance: f64) -> i32 {
    todo!() // cxx-internal:engine_collision_convex.c.o:_mjc_isDistinctContact
}

/// C: mju_rotateFrame (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_convex.c:810)
/// Calls: cxx:_mju_copy, cxx:_mju_mulMatMat3, cxx:_mju_mulMatVec3, cxx:_mju_subFrom3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_rotate_frame(origin: *const f64, rot: *const f64, xmat: *mut f64, xpos: *mut f64) {
    todo!() // cxx-internal:engine_collision_convex.c.o:_mju_rotateFrame
}

/// C: maxContacts (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_convex.c:831)
#[allow(unused_variables, non_snake_case)]
pub fn max_contacts(m: *const mjModel, obj1: *const mjCCDObj, obj2: *const mjCCDObj) -> i32 {
    todo!() // cxx-internal:engine_collision_convex.c.o:_maxContacts
}

/// C: addplanemesh (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_convex.c:946)
/// Calls: cxx:_mju_addTo3, cxx:_mju_dist3, cxx:_mju_dot3, cxx:_mju_mulMatVec3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn addplanemesh(con: *mut mjPreContact, vertex: *const f32, pos1: *const f64, normal1: *const f64, pos2: *const f64, mat2: *const f64, first: *const f64, rbound: f64) -> i32 {
    todo!() // cxx-internal:engine_collision_convex.c.o:_addplanemesh
}

/// C: addPrismVert (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_convex.c:1100)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn add_prism_vert(obj: *mut mjCCDObj, r: i32, c: i32, i: i32, dx: f64, dy: f64, margin: f64) {
    todo!() // cxx-internal:engine_collision_convex.c.o:_addPrismVert
}

/// C: mjc_ellipsoidInside (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_convex.c:1282)
/// Calls: cxx:_mju_dist3, cxx:_mju_normalize3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_ellipsoid_inside(nrm: *mut f64, pos: *const f64, size: *const f64) -> i32 {
    todo!() // cxx-internal:engine_collision_convex.c.o:_mjc_ellipsoidInside
}

/// C: mjc_ellipsoidOutside (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_convex.c:1337)
/// Calls: cxx:_mju_normalize3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_ellipsoid_outside(nrm: *mut f64, pos: *const f64, size: *const f64) -> i32 {
    todo!() // cxx-internal:engine_collision_convex.c.o:_mjc_ellipsoidOutside
}

/// C: mjc_initCCDObj (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_convex.h:94)
/// Calls: cxx:_mju_copy, cxx:_mju_zero4
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_init_ccd_obj(obj: *mut mjCCDObj, m: *const mjModel, d: *const mjData, g: i32, margin: f64) {
    todo!() // cxx:_mjc_initCCDObj
}

/// C: mjc_center (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_convex.h:97)
/// Calls: cxx:_mju_scl3, cxx:_mju_zero3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_center(res: *mut f64, obj: *const mjCCDObj) {
    todo!() // cxx:_mjc_center
}

/// C: mjccd_center (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_convex.h:100)
/// Calls: cxx:_mjc_center
#[allow(unused_variables, non_snake_case)]
pub fn mjccd_center(obj: *const (), center: *mut ccd_vec3_t) {
    todo!() // cxx:_mjccd_center
}

/// C: mjccd_support (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_convex.h:103)
/// Calls: cxx-internal:engine_collision_convex.c.o:_mjc_prism_support, cxx:_mju_dot3, cxx:_mju_message, cxx:_mju_mulMatTVec3, cxx:_mju_mulMatVec3, cxx:_mju_normalize3, cxx:_mju_sign, cxx:_mju_warning, cxx:_mju_zero3
#[allow(unused_variables, non_snake_case)]
pub fn mjccd_support(obj: *const (), dir: *const ccd_vec3_t, vec: *mut ccd_vec3_t) {
    todo!() // cxx:_mjccd_support
}

/// C: mjc_pointSupport (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_convex.h:106)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_point_support(res: *mut f64, obj: *mut mjCCDObj, dir: *const f64) {
    todo!() // cxx:_mjc_pointSupport
}

/// C: mjc_lineSupport (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_convex.h:109)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_line_support(res: *mut f64, obj: *mut mjCCDObj, dir: *const f64) {
    todo!() // cxx:_mjc_lineSupport
}

/// C: mjc_PlaneConvex (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_convex.h:112)
/// Calls: cxx-internal:engine_collision_convex.c.o:_addplanemesh, cxx:_mjc_initCCDObj, cxx:_mjccd_support, cxx:_mju_dot3, cxx:_mju_mulMatTVec3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_plane_convex(m: *const mjModel, d: *mut mjData, con: *mut mjPreContact, g1: i32, g2: i32, margin: f64) -> i32 {
    todo!() // cxx:_mjc_PlaneConvex
}

/// C: mjc_ConvexHField (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_convex.h:113)
/// Calls: cxx-internal:engine_collision_convex.c.o:_addPrismVert, cxx-internal:engine_collision_convex.c.o:_mjc_penetration, cxx:_mjc_fixNormal, cxx:_mjc_initCCDObj, cxx:_mju_mulMatTVec3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_convex_h_field(m: *const mjModel, d: *mut mjData, con: *mut mjPreContact, g1: i32, g2: i32, margin: f64) -> i32 {
    todo!() // cxx:_mjc_ConvexHField
}

/// C: mjc_Convex (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_convex.h:114)
/// Calls: cxx-internal:engine_collision_convex.c.o:_maxContacts, cxx-internal:engine_collision_convex.c.o:_mjc_isDistinctContact, cxx-internal:engine_collision_convex.c.o:_mjc_penetration, cxx-internal:engine_collision_convex.c.o:_mju_rotateFrame, cxx:_mjc_fixNormal, cxx:_mjc_initCCDObj, cxx:_mju_makeFrame, cxx:_mju_min, cxx:_mju_quat2Mat, cxx:_mju_transpose, cxx:_mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_convex(m: *const mjModel, d: *mut mjData, con: *mut mjPreContact, g1: i32, g2: i32, margin: f64) -> i32 {
    todo!() // cxx:_mjc_Convex
}

/// C: mjc_ConvexElem (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_convex.h:117)
/// Calls: cxx-internal:engine_collision_convex.c.o:_mjc_penetration, cxx-internal:engine_collision_convex.c.o:_mjc_setCCDObjFlex, cxx:_mjc_fixNormal, cxx:_mjc_initCCDObj
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_convex_elem(m: *const mjModel, d: *mut mjData, con: *mut mjPreContact, g1: i32, f1: i32, e1: i32, v1: i32, f2: i32, e2: i32, margin: f64) -> i32 {
    todo!() // cxx:_mjc_ConvexElem
}

/// C: mjc_HFieldElem (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_convex.h:121)
/// Calls: cxx-internal:engine_collision_convex.c.o:_mjc_penetration, cxx-internal:engine_collision_convex.c.o:_mjc_setCCDObjFlex, cxx:_mjc_initCCDObj, cxx:_mju_max, cxx:_mju_min, cxx:_mju_mulMatVec3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_h_field_elem(m: *const mjModel, d: *mut mjData, con: *mut mjPreContact, g: i32, f: i32, e: i32, margin: f64) -> i32 {
    todo!() // cxx:_mjc_HFieldElem
}

/// C: mjc_fixNormal (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_convex.h:125)
/// Calls: cxx-internal:engine_collision_convex.c.o:_mjc_ellipsoidInside, cxx-internal:engine_collision_convex.c.o:_mjc_ellipsoidOutside, cxx:_mju_mulMatTVec3, cxx:_mju_norm, cxx:_mju_normalize3, cxx:_mju_sub3
#[allow(unused_variables, non_snake_case)]
pub fn mjc_fix_normal(m: *const mjModel, d: *const mjData, con: *mut mjPreContact, g1: i32, g2: i32) {
    todo!() // cxx:_mjc_fixNormal
}

/// C: mjc_setCCDBuffer (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_convex.h:128)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_set_ccd_buffer(buffer: *mut ()) {
    todo!() // cxx:_mjc_setCCDBuffer
}

