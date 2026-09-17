//! Port of: Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_sdf.c
//! IR hash: 1fa44043fe383ce9
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: boxProjection (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_sdf.c:35)
/// Calls: cxx:_mju_max
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn box_projection(point: *mut f64, r#box: *const f64) -> f64 {
    todo!() // cxx:_boxProjection
}

/// C: findOct (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_sdf.c:69)
/// Calls: cxx:_mju_error
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn find_oct(w: *mut f64, dw: *mut [f64; 3], oct_aabb: *const f64, oct_child: *const i32, p: *const f64) -> i32 {
    todo!() // cxx-internal:engine_collision_sdf.c.o:_findOct
}

/// C: oct_distance (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_sdf.c:138)
/// Calls: cxx-internal:engine_collision_sdf.c.o:_findOct, cxx:_boxProjection, cxx:_mju_message
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn oct_distance(m: *const mjModel, p: *const f64, meshid: i32) -> f64 {
    todo!() // cxx:_oct_distance
}

/// C: oct_gradient (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_sdf.c:162)
/// Calls: cxx-internal:engine_collision_sdf.c.o:_findOct, cxx:_boxProjection, cxx:_mju_message, cxx:_mju_zero3, cxx:_oct_distance
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn oct_gradient(m: *const mjModel, grad: *mut f64, point: *const f64, meshid: i32) {
    todo!() // cxx:_oct_gradient
}

/// C: radialField3d (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_sdf.c:205)
/// Calls: cxx:_mju_normalize3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn radial_field3d(field: *mut f64, a: *const f64, x: *const f64, size: *const f64) {
    todo!() // cxx-internal:engine_collision_sdf.c.o:_radialField3d
}

/// C: geomDistance (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_sdf.c:218)
/// Calls: cxx-internal:engine_collision_sdf.c.o:_radialField3d, cxx:_mju_clip, cxx:_mju_max, cxx:_mju_message, cxx:_mju_min, cxx:_mju_norm, cxx:_mju_norm3, cxx:_oct_distance
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn geom_distance(m: *const mjModel, d: *const mjData, p: *const mjpPlugin, i: i32, x: *const f64, r#type: u32) -> f64 {
    todo!() // cxx-internal:engine_collision_sdf.c.o:_geomDistance
}

/// C: geomGradient (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_sdf.c:295)
/// Calls: cxx-internal:engine_collision_sdf.c.o:_radialField3d, cxx:_mju_clip, cxx:_mju_copy3, cxx:_mju_max, cxx:_mju_message, cxx:_mju_norm, cxx:_mju_norm3, cxx:_mju_normalize3, cxx:_mju_zero3, cxx:_oct_gradient
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn geom_gradient(gradient: *mut f64, m: *const mjModel, d: *const mjData, p: *const mjpPlugin, i: i32, x: *const f64, r#type: u32) {
    todo!() // cxx-internal:engine_collision_sdf.c.o:_geomGradient
}

/// C: mapPose (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_sdf.c:519)
/// Calls: cxx:_mju_mulPose, cxx:_mju_negPose, cxx:_mju_quat2Mat
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn map_pose(xpos1: *const f64, xquat1: *const f64, xpos2: *const f64, xquat2: *const f64, pos12: *mut f64, mat12: *mut f64) {
    todo!() // cxx-internal:engine_collision_sdf.c.o:_mapPose
}

/// C: isknown (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_sdf.c:532)
/// Calls: cxx:_mju_dist3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn isknown(points: *const f64, x: *const f64, cnt: i32) -> i32 {
    todo!() // cxx-internal:engine_collision_sdf.c.o:_isknown
}

/// C: addPreContact (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_sdf.c:545)
/// Calls: cxx-internal:engine_collision_sdf.c.o:_isknown, cxx:_mjc_gradient, cxx:_mju_addTo3, cxx:_mju_copy3, cxx:_mju_normalize3, cxx:_mju_rotVecQuat, cxx:_mju_scl3, cxx:_mju_zero3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn add_pre_contact(points: *mut f64, con: *mut mjPreContact, x: *const f64, pos2: *const f64, quat2: *const f64, dist: f64, cnt: i32, m: *const mjModel, s: *const mjSDF, d: *const mjData, flipNormal: i32) -> i32 {
    todo!() // cxx-internal:engine_collision_sdf.c.o:_addPreContact
}

/// C: stepFrankWolfe (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_sdf.c:585)
/// Calls: cxx:_mjc_distance, cxx:_mjc_gradient, cxx:_mju_addToScl3, cxx:_mju_copy3, cxx:_mju_dot3, cxx:_mju_subFrom3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn step_frank_wolfe(x: *mut f64, corners: *const f64, ncorners: i32, m: *const mjModel, sdf: *const mjSDF, d: *const mjData) -> f64 {
    todo!() // cxx-internal:engine_collision_sdf.c.o:_stepFrankWolfe
}

/// C: stepGradient (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_sdf.c:615)
/// Calls: cxx:_mjc_distance, cxx:_mjc_gradient, cxx:_mju_addScl3, cxx:_mju_dot3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn step_gradient(x: *mut f64, m: *const mjModel, s: *const mjSDF, d: *const mjData, niter: i32) -> f64 {
    todo!() // cxx-internal:engine_collision_sdf.c.o:_stepGradient
}

/// C: triangleIntersect (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_sdf.c:665)
/// Calls: cxx:_mjc_distance, cxx:_mju_addTo3, cxx:_mju_addToScl3, cxx:_mju_cross, cxx:_mju_dist3, cxx:_mju_dot3, cxx:_mju_max, cxx:_mju_norm3, cxx:_mju_normalize3, cxx:_mju_scl3, cxx:_mju_sub3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn triangle_intersect(triangle: *const f64, m: *const mjModel, sdf: *const mjSDF, d: *const mjData) -> i32 {
    todo!() // cxx-internal:engine_collision_sdf.c.o:_triangleIntersect
}

/// C: boxIntersect (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_sdf.c:737)
/// Calls: cxx:_mjc_distance, cxx:_mju_addTo3, cxx:_mju_mulMatVec3, cxx:_mju_norm3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn box_intersect(bvh: *const f64, offset: *const f64, rotation: *const f64, m: *const mjModel, s: *const mjSDF, d: *const mjData) -> i32 {
    todo!() // cxx-internal:engine_collision_sdf.c.o:_boxIntersect
}

/// C: selectFPS (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_sdf.c:752)
/// Calls: cxx-internal:engine_collision_gjk.c.o:_next
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn select_fps(candidate: *const f64, dist: *const f64, ncandidate: i32, selected_indices: *mut i32, max_select: i32) -> i32 {
    todo!() // cxx-internal:engine_collision_sdf.c.o:_selectFPS
}

/// C: processSdfCorners (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_sdf.c:808)
/// Calls: cxx-internal:engine_collision_sdf.c.o:_stepFrankWolfe, cxx-internal:engine_collision_sdf.c.o:_triangleIntersect, cxx:_mju_Halton, cxx:_mju_copy3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn process_sdf_corners(corners: *const f64, m: *const mjModel, d: *const mjData, sdf: *const mjSDF, nstartpts: i32, candidate: *mut f64, dist: *mut f64, ncandidate: *mut i32) {
    todo!() // cxx-internal:engine_collision_sdf.c.o:_processSdfCorners
}

/// C: processOneFace (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_sdf.c:866)
/// Calls: cxx-internal:engine_collision_sdf.c.o:_processSdfCorners, cxx:_mju_addTo3, cxx:_mju_mulMatVec3
#[allow(unused_variables, non_snake_case)]
pub fn process_one_face(faceid: i32, bvh_active: *mut bool, node: i32, ctx: *mut MeshSDFContext) {
    todo!() // cxx-internal:engine_collision_sdf.c.o:_processOneFace
}

/// C: traverseBVH (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_sdf.c:903)
/// Calls: cxx-internal:engine_collision_sdf.c.o:_boxIntersect, cxx:_mju_message
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn traverse_bvh(bvh: *const f64, nodeid: *const i32, child: *const i32, bvh_active: *mut bool, offset: *const f64, rotation: *const f64, m: *const mjModel, d: *const mjData, sdf: *const mjSDF, callback: BVHLeafCallback, ctx: *mut ()) {
    todo!() // cxx-internal:engine_collision_sdf.c.o:_traverseBVH
}

/// C: meshFaceCallback (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_sdf.c:943)
/// Calls: cxx-internal:engine_collision_sdf.c.o:_processOneFace
#[allow(unused_variables, non_snake_case)]
pub fn mesh_face_callback(face_id: i32, node: i32, ctx: *mut ()) -> i32 {
    todo!() // cxx-internal:engine_collision_sdf.c.o:_meshFaceCallback
}

/// C: flexElemCallback (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_sdf.c:1198)
/// Calls: cxx-internal:engine_collision_sdf.c.o:_processSdfCorners, cxx:_mju_addTo3, cxx:_mju_copy3, cxx:_mju_mulMatVec3
#[allow(unused_variables, non_snake_case)]
pub fn flex_elem_callback(elem_idx: i32, node: i32, ctx: *mut ()) -> i32 {
    todo!() // cxx-internal:engine_collision_sdf.c.o:_flexElemCallback
}

/// C: mjc_getSDF (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_sdf.h:29)
/// Calls: cxx:_mjp_getPluginAtSlotUnsafe, cxx:_mjp_pluginCount, cxx:_mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mjc_get_sdf(m: *const mjModel, id: i32) -> *const mjpPlugin {
    todo!() // cxx:_mjc_getSDF
}

/// C: mjc_distance (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_sdf.h:32)
/// Calls: cxx-internal:engine_collision_sdf.c.o:_geomDistance, cxx:_mju_addTo3, cxx:_mju_max, cxx:_mju_message, cxx:_mju_mulMatVec3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_distance(m: *const mjModel, d: *const mjData, s: *const mjSDF, x: *const f64) -> f64 {
    todo!() // cxx:_mjc_distance
}

/// C: mjc_gradient (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_sdf.h:35)
/// Calls: cxx-internal:engine_collision_sdf.c.o:_geomDistance, cxx-internal:engine_collision_sdf.c.o:_geomGradient, cxx:_mju_addTo3, cxx:_mju_addToScl3, cxx:_mju_max, cxx:_mju_message, cxx:_mju_mulMatTVec3, cxx:_mju_mulMatVec3, cxx:_mju_normalize3, cxx:_mju_sub3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_gradient(m: *const mjModel, d: *const mjData, s: *const mjSDF, gradient: *mut f64, x: *const f64) {
    todo!() // cxx:_mjc_gradient
}

/// C: mjc_HFieldSDF (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_sdf.h:39)
/// Calls: cxx:_mju_warning
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_h_field_sdf(m: *const mjModel, d: *mut mjData, con: *mut mjPreContact, g1: i32, g2: i32, margin: f64) -> i32 {
    todo!() // cxx:_mjc_HFieldSDF
}

/// C: mjc_MeshSDF (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_sdf.h:42)
/// Calls: cxx-internal:engine_collision_sdf.c.o:_addPreContact, cxx-internal:engine_collision_sdf.c.o:_mapPose, cxx-internal:engine_collision_sdf.c.o:_selectFPS, cxx-internal:engine_collision_sdf.c.o:_traverseBVH, cxx:_mjc_getSDF, cxx:_mju_mat2Quat, cxx:_mju_max
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_mesh_sdf(m: *const mjModel, d: *mut mjData, con: *mut mjPreContact, g1: i32, g2: i32, margin: f64) -> i32 {
    todo!() // cxx:_mjc_MeshSDF
}

/// C: mjc_SDF (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_sdf.h:45)
/// Calls: cxx-internal:engine_collision_sdf.c.o:_addPreContact, cxx-internal:engine_collision_sdf.c.o:_mapPose, cxx-internal:engine_collision_sdf.c.o:_stepGradient, cxx:_mjc_getSDF, cxx:_mju_Halton, cxx:_mju_addTo3, cxx:_mju_mat2Quat, cxx:_mju_max, cxx:_mju_message, cxx:_mju_min, cxx:_mju_mulMatVec3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_sdf(m: *const mjModel, d: *mut mjData, con: *mut mjPreContact, g1: i32, g2: i32, margin: f64) -> i32 {
    todo!() // cxx:_mjc_SDF
}

/// C: mjc_FlexSDF (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_collision_sdf.h:48)
/// Calls: cxx-internal:engine_collision_sdf.c.o:_addPreContact, cxx-internal:engine_collision_sdf.c.o:_mapPose, cxx-internal:engine_collision_sdf.c.o:_processSdfCorners, cxx-internal:engine_collision_sdf.c.o:_selectFPS, cxx-internal:engine_collision_sdf.c.o:_traverseBVH, cxx:_mjc_getSDF, cxx:_mju_addTo3, cxx:_mju_copy3, cxx:_mju_mat2Quat, cxx:_mju_max, cxx:_mju_mulMatVec3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_flex_sdf(m: *const mjModel, d: *const mjData, con: *mut mjPreContact, elem: *mut i32, g: i32, f: i32, margin: f64) -> i32 {
    todo!() // cxx:_mjc_FlexSDF
}

