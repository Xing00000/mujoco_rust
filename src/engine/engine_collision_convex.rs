//! Port of: engine/engine_collision_convex.c
//! IR hash: 6ff71909dacce27f
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: prism_firstdir (engine/engine_collision_convex.c:47)
#[allow(unused_variables, non_snake_case)]
pub fn prism_firstdir(o1: *const (), o2: *const (), vec: *mut ccd_vec3_t) {
    todo!() // prism_firstdir
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
    todo!() // mulMatTVec3
}

/// C: localToGlobal (engine/engine_collision_convex.c:183)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn local_to_global(res: *mut f64, mat: *const f64, dir: *const f64, pos: *const f64) {
    todo!() // localToGlobal
}

/// C: mjc_sphereSupport (engine/engine_collision_convex.c:202)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_sphere_support(res: *mut f64, obj: *mut mjCCDObj, dir: *const f64) {
    todo!() // mjc_sphereSupport
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
    todo!() // mjc_capsuleSupport
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
    todo!() // mjc_ellipsoidSupport
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
    todo!() // mjc_cylinderSupport
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
    todo!() // mjc_boxSupport
}

/// C: dot3f (engine/engine_collision_convex.c:343)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn dot3f(a: *const f64, b: *const f32) -> f64 {
    todo!() // dot3f
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
    todo!() // mjc_meshSupport
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
    todo!() // mjc_hillclimbSupport
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
    todo!() // mjc_setCCDObjFlex
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
    todo!() // mjc_isDistinctContact
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
    todo!() // mju_rotateFrame
}

/// C: maxContacts (engine/engine_collision_convex.c:831)
#[allow(unused_variables, non_snake_case)]
pub fn max_contacts(m: *const mjModel, obj1: *const mjCCDObj, obj2: *const mjCCDObj) -> i32 {
    todo!() // maxContacts
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
    todo!() // addplanemesh
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
    todo!() // mjc_ellipsoidOutside
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
    todo!() // mjc_initCCDObj
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
    todo!() // mjc_pointSupport
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
    todo!("requires global state") // mjc_setCCDBuffer: sets thread-local ccd_buffer pointer
}

