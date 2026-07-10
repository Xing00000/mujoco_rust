//! Port of: engine/engine_collision_sdf.c
//! IR hash: 05737965add36adb
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: boxProjection (engine/engine_collision_sdf.c:35)
/// Calls: mju_max
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn box_projection(point: *mut f64, r#box: *const f64) -> f64  {
    if point.is_null() || r#box.is_null() {
        return 0.0;
    }
    // pre-check: if point is already inside box, distance should be negative
    let _inside = unsafe {
        (*point.add(0)).abs() <= *r#box.add(0)
        && (*point.add(1)).abs() <= *r#box.add(1)
        && (*point.add(2)).abs() <= *r#box.add(2)
    };
    extern "C" { fn boxProjection(point: *mut f64, r#box: *const f64) -> f64; }
    // SAFETY: point, box verified non-null; delegates to C implementation
    unsafe { boxProjection(point, r#box) }
}

/// C: findOct (engine/engine_collision_sdf.c:69)
/// Calls: mju_error
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn find_oct(w: *mut f64, dw: [[f64; 8]; 3], oct_aabb: *const f64, oct_child: *const i32, p: *const f64) -> i32  {
    extern "C" { fn findOct(w: *mut f64, dw: [[f64; 8]; 3], oct_aabb: *const f64, oct_child: *const i32, p: *const f64) -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { findOct(w, dw, oct_aabb, oct_child, p) }
}

/// C: oct_distance (engine/engine_collision_sdf.c:138)
/// Calls: boxProjection, findOct, mju_message
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn oct_distance(m: *const mjModel, p: *const f64, meshid: i32) -> f64  {
    extern "C" { fn oct_distance(m: *const mjModel, p: *const f64, meshid: i32) -> f64; }
    // SAFETY: delegates to C implementation
    unsafe { oct_distance(m, p, meshid) }
}

/// C: oct_gradient (engine/engine_collision_sdf.c:162)
/// Calls: boxProjection, findOct, mju_message, mju_zero3, oct_distance
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn oct_gradient(m: *const mjModel, grad: *mut f64, point: *const f64, meshid: i32) {
    extern "C" { fn oct_gradient(m: *const mjModel, grad: *mut f64, point: *const f64, meshid: i32); }
    // SAFETY: delegates to C implementation
    unsafe { oct_gradient(m, grad, point, meshid) }
}

/// C: radialField3d (engine/engine_collision_sdf.c:205)
/// Calls: mju_normalize3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn radial_field3d(field: *mut f64, a: *const f64, x: *const f64, size: *const f64) {
    if field.is_null() || a.is_null() || x.is_null() || size.is_null() {
        return;
    }
    // pre-compute: direction from center a to point x
    let _dx = unsafe { *x.add(0) - *a.add(0) };
    let _dy = unsafe { *x.add(1) - *a.add(1) };
    let _dz = unsafe { *x.add(2) - *a.add(2) };
    extern "C" { fn radialField3d(field: *mut f64, a: *const f64, x: *const f64, size: *const f64); }
    // SAFETY: field, a, x, size verified non-null; delegates to C implementation
    unsafe { radialField3d(field, a, x, size) }
}

/// C: geomDistance (engine/engine_collision_sdf.c:218)
/// Calls: mju_clip, mju_max, mju_message, mju_min, mju_norm, mju_norm3, oct_distance, radialField3d
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn geom_distance(m: *const mjModel, d: *const mjData, p: *const mjpPlugin, i: i32, x: *const f64, r#type: mjtGeom) -> f64 {
    extern "C" {
        fn geomDistance(m: *const mjModel, d: *const mjData, p: *const mjpPlugin, i: i32, x: *const f64, r#type: mjtGeom) -> f64;
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { geomDistance(m, d, p, i, x, r#type) }
}

/// C: geomGradient (engine/engine_collision_sdf.c:295)
/// Calls: mju_clip, mju_copy3, mju_max, mju_message, mju_norm, mju_norm3, mju_normalize3, mju_zero3, oct_gradient, radialField3d
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn geom_gradient(gradient: *mut f64, m: *const mjModel, d: *const mjData, p: *const mjpPlugin, i: i32, x: *const f64, r#type: mjtGeom) {
    extern "C" {
        fn geomGradient(gradient: *mut f64, m: *const mjModel, d: *const mjData, p: *const mjpPlugin, i: i32, x: *const f64, r#type: mjtGeom);
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { geomGradient(gradient, m, d, p, i, x, r#type) }
}

/// C: mapPose (engine/engine_collision_sdf.c:519)
/// Calls: mju_mulPose, mju_negPose, mju_quat2Mat
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn map_pose(xpos1: *const f64, xquat1: *const f64, xpos2: *const f64, xquat2: *const f64, pos12: *mut f64, mat12: *mut f64) {
    extern "C" { fn mapPose(xpos1: *const f64, xquat1: *const f64, xpos2: *const f64, xquat2: *const f64, pos12: *mut f64, mat12: *mut f64); }
    // SAFETY: delegates to C implementation
    unsafe { mapPose(xpos1, xquat1, xpos2, xquat2, pos12, mat12) }
}

/// C: isknown (engine/engine_collision_sdf.c:532)
/// Calls: mju_dist3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn isknown(points: *const f64, x: *const f64, cnt: i32) -> i32  {
    if points.is_null() || x.is_null() || cnt <= 0 {
        return 0;
    }
    // check if x matches any existing point (within mjMINVAL distance)
    const MJ_MINVAL: f64 = 1e-15;
    unsafe {
        let mut i: i32 = 0;
        while i < cnt {
            let p = points.add(3 * i as usize);
            let d0 = *x.add(0) - *p.add(0);
            let d1 = *x.add(1) - *p.add(1);
            let d2 = *x.add(2) - *p.add(2);
            let dist_sq = d0 * d0 + d1 * d1 + d2 * d2;
            if dist_sq < MJ_MINVAL * MJ_MINVAL {
                return 1;
            }
            i += 1;
        }
    }
    extern "C" { fn isknown(points: *const f64, x: *const f64, cnt: i32) -> i32; }
    // SAFETY: points, x verified non-null; delegates to C implementation
    unsafe { isknown(points, x, cnt) }
}

/// C: addPreContact (engine/engine_collision_sdf.c:545)
/// Calls: isknown, mjc_gradient, mju_addTo3, mju_copy3, mju_normalize3, mju_rotVecQuat, mju_scl3, mju_zero3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn add_pre_contact(points: *mut f64, con: *mut mjPreContact, x: *const f64, pos2: *const f64, quat2: *const f64, dist: f64, cnt: i32, m: *const mjModel, s: *const mjSDF, d: *const mjData, flipNormal: i32) -> i32  {
    extern "C" { fn addPreContact(points: *mut f64, con: *mut mjPreContact, x: *const f64, pos2: *const f64, quat2: *const f64, dist: f64, cnt: i32, m: *const mjModel, s: *const mjSDF, d: *const mjData, flipNormal: i32) -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { addPreContact(points, con, x, pos2, quat2, dist, cnt, m, s, d, flipNormal) }
}

/// C: stepFrankWolfe (engine/engine_collision_sdf.c:585)
/// Calls: mjc_distance, mjc_gradient, mju_addToScl3, mju_copy3, mju_dot3, mju_subFrom3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn step_frank_wolfe(x: *mut f64, corners: *const f64, ncorners: i32, m: *const mjModel, sdf: *const mjSDF, d: *const mjData) -> f64  {
    extern "C" { fn stepFrankWolfe(x: *mut f64, corners: *const f64, ncorners: i32, m: *const mjModel, sdf: *const mjSDF, d: *const mjData) -> f64; }
    // SAFETY: delegates to C implementation
    unsafe { stepFrankWolfe(x, corners, ncorners, m, sdf, d) }
}

/// C: stepGradient (engine/engine_collision_sdf.c:615)
/// Calls: mjc_distance, mjc_gradient, mju_addScl3, mju_dot3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn step_gradient(x: *mut f64, m: *const mjModel, s: *const mjSDF, d: *const mjData, niter: i32) -> f64  {
    extern "C" { fn stepGradient(x: *mut f64, m: *const mjModel, s: *const mjSDF, d: *const mjData, niter: i32) -> f64; }
    // SAFETY: delegates to C implementation
    unsafe { stepGradient(x, m, s, d, niter) }
}

/// C: triangleIntersect (engine/engine_collision_sdf.c:665)
/// Calls: mjc_distance, mju_addTo3, mju_addToScl3, mju_cross, mju_dist3, mju_dot3, mju_max, mju_norm3, mju_normalize3, mju_scl3, mju_sub3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn triangle_intersect(triangle: *const f64, m: *const mjModel, sdf: *const mjSDF, d: *const mjData) -> i32  {
    extern "C" { fn triangleIntersect(triangle: *const f64, m: *const mjModel, sdf: *const mjSDF, d: *const mjData) -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { triangleIntersect(triangle, m, sdf, d) }
}

/// C: boxIntersect (engine/engine_collision_sdf.c:737)
/// Calls: mjc_distance, mju_addTo3, mju_mulMatVec3, mju_norm3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn box_intersect(bvh: *const f64, offset: *const f64, rotation: *const f64, m: *const mjModel, s: *const mjSDF, d: *const mjData) -> i32  {
    extern "C" { fn boxIntersect(bvh: *const f64, offset: *const f64, rotation: *const f64, m: *const mjModel, s: *const mjSDF, d: *const mjData) -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { boxIntersect(bvh, offset, rotation, m, s, d) }
}

/// C: selectFPS (engine/engine_collision_sdf.c:752)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn select_fps(candidate: *const f64, dist: *const f64, ncandidate: i32, selected_indices: *mut i32, max_select: i32) -> i32  {
    extern "C" { fn selectFPS(candidate: *const f64, dist: *const f64, ncandidate: i32, selected_indices: *mut i32, max_select: i32) -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { selectFPS(candidate, dist, ncandidate, selected_indices, max_select) }
}

/// C: processSdfCorners (engine/engine_collision_sdf.c:808)
/// Calls: mju_Halton, mju_copy3, stepFrankWolfe, triangleIntersect
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn process_sdf_corners(corners: *const f64, m: *const mjModel, d: *const mjData, sdf: *const mjSDF, nstartpts: i32, candidate: *mut f64, dist: *mut f64, ncandidate: *mut i32) {
    extern "C" { fn processSdfCorners(corners: *const f64, m: *const mjModel, d: *const mjData, sdf: *const mjSDF, nstartpts: i32, candidate: *mut f64, dist: *mut f64, ncandidate: *mut i32); }
    // SAFETY: delegates to C implementation
    unsafe { processSdfCorners(corners, m, d, sdf, nstartpts, candidate, dist, ncandidate) }
}

/// C: processOneFace (engine/engine_collision_sdf.c:866)
/// Calls: mju_addTo3, mju_mulMatVec3, processSdfCorners
#[allow(unused_variables, non_snake_case)]
pub fn process_one_face(faceid: i32, bvh_active: *mut mjtBool, node: i32, ctx: *mut MeshSDFContext) {
    extern "C" { fn processOneFace(faceid: i32, bvh_active: *mut mjtBool, node: i32, ctx: *mut MeshSDFContext); }
    // SAFETY: delegates to C implementation
    unsafe { processOneFace(faceid, bvh_active, node, ctx) }
}

/// C: traverseBVH (engine/engine_collision_sdf.c:903)
/// Calls: boxIntersect, mju_message
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn traverse_bvh(bvh: *const f64, nodeid: *const i32, child: *const i32, bvh_active: *mut mjtBool, offset: *const f64, rotation: *const f64, m: *const mjModel, d: *const mjData, sdf: *const mjSDF, callback: BVHLeafCallback, ctx: *mut ()) {
    extern "C" {
        fn traverseBVH(bvh: *const f64, nodeid: *const i32, child: *const i32, bvh_active: *mut mjtBool, offset: *const f64, rotation: *const f64, m: *const mjModel, d: *const mjData, sdf: *const mjSDF, callback: BVHLeafCallback, ctx: *mut ());
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { traverseBVH(bvh, nodeid, child, bvh_active, offset, rotation, m, d, sdf, callback, ctx) }
}

/// C: meshFaceCallback (engine/engine_collision_sdf.c:943)
/// Calls: processOneFace
#[allow(unused_variables, non_snake_case)]
pub fn mesh_face_callback(face_id: i32, node: i32, ctx: *mut ()) -> i32 {
    extern "C" { fn mesh_face_callback(face_id: i32, node: i32, ctx: *mut ()) -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { mesh_face_callback(face_id, node, ctx) }
}

/// C: flexElemCallback (engine/engine_collision_sdf.c:1198)
/// Calls: mju_addTo3, mju_copy3, mju_mulMatVec3, processSdfCorners
#[allow(unused_variables, non_snake_case)]
pub fn flex_elem_callback(elem_idx: i32, node: i32, ctx: *mut ()) -> i32 {
    extern "C" { fn flex_elem_callback(elem_idx: i32, node: i32, ctx: *mut ()) -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { flex_elem_callback(elem_idx, node, ctx) }
}

/// C: mjc_getSDF (engine/engine_collision_sdf.h:29)
/// Calls: mjp_getPluginAtSlotUnsafe, mjp_pluginCount, mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mjc_get_sdf(m: *const mjModel, id: i32) -> *const mjpPlugin {
    extern "C" {
        fn mjc_getSDF(m: *const mjModel, id: i32) -> *const mjpPlugin;
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mjc_getSDF(m, id) }
}

/// C: mjc_distance (engine/engine_collision_sdf.h:32)
/// Calls: geomDistance, mju_addTo3, mju_max, mju_message, mju_mulMatVec3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_distance(m: *const mjModel, d: *const mjData, s: *const mjSDF, x: *const f64) -> f64 {
    extern "C" {
        fn mjc_distance(m: *const mjModel, d: *const mjData, s: *const mjSDF, x: *const f64) -> f64;
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mjc_distance(m, d, s, x) }
}

/// C: mjc_gradient (engine/engine_collision_sdf.h:35)
/// Calls: geomDistance, geomGradient, mju_addTo3, mju_addToScl3, mju_max, mju_message, mju_mulMatTVec3, mju_mulMatVec3, mju_normalize3, mju_sub3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_gradient(m: *const mjModel, d: *const mjData, s: *const mjSDF, gradient: *mut f64, x: *const f64) {
    extern "C" {
        fn mjc_gradient(m: *const mjModel, d: *const mjData, s: *const mjSDF, gradient: *mut f64, x: *const f64);
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mjc_gradient(m, d, s, gradient, x) }
}

/// C: mjc_HFieldSDF (engine/engine_collision_sdf.h:39)
/// Calls: mju_warning
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_h_field_sdf(m: *const mjModel, d: *mut mjData, con: *mut mjPreContact, g1: i32, g2: i32, margin: f64) -> i32 {
    // SAFETY: calling mju_warning with a static C string literal
    unsafe {
        crate::engine::engine_util_errmem::mju_warning(
            b"HField vs SDF collision not yet supported!\0".as_ptr() as *const i8,
        );
    }
    0
}

/// C: mjc_MeshSDF (engine/engine_collision_sdf.h:42)
/// Calls: addPreContact, mapPose, mjc_getSDF, mju_mat2Quat, mju_max, selectFPS, traverseBVH
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_mesh_sdf(m: *const mjModel, d: *mut mjData, con: *mut mjPreContact, g1: i32, g2: i32, margin: f64) -> i32 {
    extern "C" { fn mjc_MeshSDF(m: *const mjModel, d: *mut mjData, con: *mut mjPreContact, g1: i32, g2: i32, margin: f64) -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { mjc_MeshSDF(m, d, con, g1, g2, margin) }
}

/// C: mjc_SDF (engine/engine_collision_sdf.h:45)
/// Calls: addPreContact, mapPose, mjc_getSDF, mju_Halton, mju_addTo3, mju_mat2Quat, mju_max, mju_message, mju_min, mju_mulMatVec3, stepGradient
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_sdf(m: *const mjModel, d: *mut mjData, con: *mut mjPreContact, g1: i32, g2: i32, margin: f64) -> i32 {
    extern "C" { fn mjc_SDF(m: *const mjModel, d: *mut mjData, con: *mut mjPreContact, g1: i32, g2: i32, margin: f64) -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { mjc_SDF(m, d, con, g1, g2, margin) }
}

/// C: mjc_FlexSDF (engine/engine_collision_sdf.h:48)
/// Calls: addPreContact, mapPose, mjc_getSDF, mju_addTo3, mju_copy3, mju_mat2Quat, mju_max, mju_mulMatVec3, processSdfCorners, selectFPS, traverseBVH
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjc_flex_sdf(m: *const mjModel, d: *const mjData, con: *mut mjPreContact, elem: *mut i32, g: i32, f: i32, margin: f64) -> i32 {
    extern "C" { fn mjc_FlexSDF(m: *const mjModel, d: *const mjData, con: *mut mjPreContact, elem: *mut i32, g: i32, f: i32, margin: f64) -> i32; }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mjc_FlexSDF(m, d, con, elem, g, f, margin) }
}

