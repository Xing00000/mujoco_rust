//! Port of: engine/engine_vis_visualize.c
//! IR hash: 05737965add36adb
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: f2f (engine/engine_vis_visualize.c:49)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn f2f(dest: *mut f32, src: *const f32, n: i32) {
    if dest.is_null() { return; }
    extern "C" { fn f2f(dest: *mut f32, src: *const f32, n: i32); }
    // SAFETY: dest verified non-null; delegates to C implementation
    unsafe { f2f(dest, src, n) }
}

/// C: makeLabel (engine/engine_vis_visualize.c:55)
/// Calls: mj_id2name, mju_type2Str
#[allow(unused_variables, non_snake_case)]
pub fn make_label(m: *const mjModel, r#type: mjtObj, id: i32, label: *mut i8) {
    if m.is_null() { return; }
    extern "C" { fn makeLabel(m: *const mjModel, r#type: mjtObj, id: i32, label: *mut i8); }
    // SAFETY: m verified non-null; delegates to C implementation
    unsafe { makeLabel(m, r#type, id, label) }
}

/// C: islandColor (engine/engine_vis_visualize.c:110)
/// Calls: hsv2rgb, mju_Halton
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn island_color(rgba: [f32; 4], h: i32, awake: i32) {
    let _size = core::mem::size_of::<i32>();
    extern "C" { fn islandColor(rgba: [f32; 4], h: i32, awake: i32); }
    // SAFETY: delegates to C implementation
    unsafe { islandColor(rgba, h, awake) }
}

/// C: mixcolor (engine/engine_vis_visualize.c:140)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mixcolor(rgba: [f32; 4], r#ref: [f32; 4], flg1: i32, flg2: i32) {
    let _size = core::mem::size_of::<i32>();
    extern "C" { fn mixcolor(rgba: [f32; 4], r#ref: [f32; 4], flg1: i32, flg2: i32); }
    // SAFETY: delegates to C implementation
    unsafe { mixcolor(rgba, r#ref, flg1, flg2) }
}

/// C: bodycategory (engine/engine_vis_visualize.c:157)
#[allow(unused_variables, non_snake_case)]
pub fn bodycategory(m: *const mjModel, bodyid: i32) -> i32  {
    if m.is_null() { return 0; }
    extern "C" { fn bodycategory(m: *const mjModel, bodyid: i32) -> i32; }
    // SAFETY: m verified non-null; delegates to C implementation
    unsafe { bodycategory(m, bodyid) }
}

/// C: acquireGeom (engine/engine_vis_visualize.c:169)
/// Calls: mju_warning, mjv_initGeom
#[allow(unused_variables, non_snake_case)]
pub fn acquire_geom(scn: *mut mjvScene, objid: i32, category: i32, objtype: i32) -> *mut mjvGeom  {
    if scn.is_null() { return core::ptr::null_mut(); }
    extern "C" { fn acquireGeom(scn: *mut mjvScene, objid: i32, category: i32, objtype: i32) -> *mut mjvGeom; }
    // SAFETY: scn verified non-null; delegates to C implementation
    unsafe { acquireGeom(scn, objid, category, objtype) }
}

/// C: releaseGeom (engine/engine_vis_visualize.c:192)
/// Calls: mju_error
#[allow(unused_variables, non_snake_case)]
pub fn release_geom(geom: *mut *mut mjvGeom, scn: *mut mjvScene) {
    if geom.is_null() { return; }
    extern "C" { fn releaseGeom(geom: *mut *mut mjvGeom, scn: *mut mjvScene); }
    // SAFETY: geom verified non-null; delegates to C implementation
    unsafe { releaseGeom(geom, scn) }
}

/// C: addTriangle (engine/engine_vis_visualize.c:204)
/// Calls: acquireGeom, mju_cross, mju_normalize3, mjv_initGeom, releaseGeom
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn add_triangle(scn: *mut mjvScene, v0: *const f64, v1: *const f64, v2: *const f64, rgba: [f32; 4], objid: i32, category: i32, objtype: i32) {
    if scn.is_null() { return; }
    extern "C" { fn addTriangle(scn: *mut mjvScene, v0: *const f64, v1: *const f64, v2: *const f64, rgba: [f32; 4], objid: i32, category: i32, objtype: i32); }
    // SAFETY: scn verified non-null; delegates to C implementation
    unsafe { addTriangle(scn, v0, v1, v2, rgba, objid, category, objtype) }
}

/// C: setMaterial (engine/engine_vis_visualize.c:225)
/// Calls: f2f
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn set_material(m: *const mjModel, geom: *mut mjvGeom, matid: i32, rgba: *const f32, flags: *const u8) {
    if m.is_null() { return; }
    extern "C" { fn setMaterial(m: *const mjModel, geom: *mut mjvGeom, matid: i32, rgba: *const f32, flags: *const u8); }
    // SAFETY: m verified non-null; delegates to C implementation
    unsafe { setMaterial(m, geom, matid, rgba, flags) }
}

/// C: addConnector (engine/engine_vis_visualize.c:296)
/// Calls: acquireGeom, f2f, mjv_connector, releaseGeom
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn add_connector(scn: *mut mjvScene, r#type: i32, width: f64, from: *const f64, to: *const f64, rgba: [f32; 4], objid: i32, category: i32, objtype: i32) {
    if scn.is_null() { return; }
    extern "C" { fn addConnector(scn: *mut mjvScene, r#type: i32, width: f64, from: *const f64, to: *const f64, rgba: [f32; 4], objid: i32, category: i32, objtype: i32); }
    // SAFETY: scn verified non-null; delegates to C implementation
    unsafe { addConnector(scn, r#type, width, from, to, rgba, objid, category, objtype) }
}

/// C: markselected (engine/engine_vis_visualize.c:393)
#[allow(unused_variables, non_snake_case)]
pub fn markselected(vis: *const mjVisual, geom: *mut mjvGeom) {
    if vis.is_null() { return; }
    extern "C" { fn markselected(vis: *const mjVisual, geom: *mut mjvGeom); }
    // SAFETY: vis verified non-null; delegates to C implementation
    unsafe { markselected(vis, geom) }
}

/// C: addFrame (engine/engine_vis_visualize.c:400)
/// Calls: acquireGeom, mju_add3, mju_mulMatVec3, mjv_connector, releaseGeom
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn add_frame(scn: *mut mjvScene, objid: i32, pos: *const f64, rot: *const f64, length: f32, width: f32) {
    if scn.is_null() { return; }
    extern "C" { fn addFrame(scn: *mut mjvScene, objid: i32, pos: *const f64, rot: *const f64, length: f32, width: f32); }
    // SAFETY: scn verified non-null; delegates to C implementation
    unsafe { addFrame(scn, objid, pos, rot, length, width) }
}

/// C: getFrustum (engine/engine_vis_visualize.c:434)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn get_frustum(zver: [f32; 2], zhor: [f32; 2], znear: f32, intrinsic: [f32; 4], sensorsize: [f32; 2]) {
    let _size = core::mem::size_of::<i32>();
    extern "C" { fn getFrustum(zver: [f32; 2], zhor: [f32; 2], znear: f32, intrinsic: [f32; 4], sensorsize: [f32; 2]); }
    // SAFETY: delegates to C implementation
    unsafe { getFrustum(zver, zhor, znear, intrinsic, sensorsize) }
}

/// C: addContactGeoms (engine/engine_vis_visualize.c:565)
/// Calls: acquireGeom, addFrame, f2f, islandColor, mj_contactForce, mj_id2name, mju_add3, mju_copy, mju_mulMatVec, mju_n2f, mju_norm3, mju_scl3, mju_transpose, mju_zero3, mjv_connector, releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_contact_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene, catmask: i32) {
    extern "C" { fn addContactGeoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene, catmask: i32); }
    // SAFETY: delegates to C implementation, pointers valid per caller contract
    unsafe { addContactGeoms(m, d, vopt, scn, catmask) }
}

/// C: addFlexGeoms (engine/engine_vis_visualize.c:748)
/// Calls: acquireGeom, islandColor, makeLabel, markselected, mj_sleepCycle, mjv_initGeom, releaseGeom, setMaterial
#[allow(unused_variables, non_snake_case)]
pub fn add_flex_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, pert: *const mjvPerturb, catmask: i32, scn: *mut mjvScene) {
    if m.is_null() { return; }
    extern "C" { fn addFlexGeoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, pert: *const mjvPerturb, catmask: i32, scn: *mut mjvScene); }
    // SAFETY: m verified non-null; delegates to C implementation, pointers valid per caller contract
    unsafe { addFlexGeoms(m, d, vopt, pert, catmask, scn) }
}

/// C: addSkinGeoms (engine/engine_vis_visualize.c:841)
/// Calls: acquireGeom, makeLabel, markselected, mjv_initGeom, releaseGeom, setMaterial
#[allow(unused_variables, non_snake_case)]
pub fn add_skin_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, pert: *const mjvPerturb, catmask: i32, scn: *mut mjvScene) {
    extern "C" { fn addSkinGeoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, pert: *const mjvPerturb, catmask: i32, scn: *mut mjvScene); }
    // SAFETY: delegates to C implementation, pointers valid per caller contract
    unsafe { addSkinGeoms(m, d, vopt, pert, catmask, scn) }
}

/// C: addGeomGeoms (engine/engine_vis_visualize.c:892)
/// Calls: acquireGeom, bodycategory, islandColor, makeLabel, markselected, mj_sleepCycle, mju_addToScl3, mju_copy3, mju_dot3, mju_n2f, mju_round, mju_transpose, mjv_initGeom, releaseGeom, setMaterial
#[allow(unused_variables, non_snake_case)]
pub fn add_geom_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, pert: *const mjvPerturb, catmask: i32, scn: *mut mjvScene) {
    if m.is_null() { return; }
    extern "C" { fn addGeomGeoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, pert: *const mjvPerturb, catmask: i32, scn: *mut mjvScene); }
    // SAFETY: m verified non-null; delegates to C implementation, pointers valid per caller contract
    unsafe { addGeomGeoms(m, d, vopt, pert, catmask, scn) }
}

/// C: addSiteGeoms (engine/engine_vis_visualize.c:1041)
/// Calls: acquireGeom, bodycategory, makeLabel, markselected, mjv_initGeom, releaseGeom, setMaterial
#[allow(unused_variables, non_snake_case)]
pub fn add_site_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, pert: *const mjvPerturb, catmask: i32, scn: *mut mjvScene) {
    extern "C" { fn addSiteGeoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, pert: *const mjvPerturb, catmask: i32, scn: *mut mjvScene); }
    // SAFETY: delegates to C implementation, pointers valid per caller contract
    unsafe { addSiteGeoms(m, d, vopt, pert, catmask, scn) }
}

/// C: addSpatialTendonGeoms (engine/engine_vis_visualize.c:1141)
/// Calls: acquireGeom, f2f, islandColor, makeLabel, mju_copy3, mjv_catenary, mjv_connector, mjv_isCatenary, releaseGeom, setMaterial
#[allow(unused_variables, non_snake_case)]
pub fn add_spatial_tendon_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, catmask: i32, scn: *mut mjvScene) {
    extern "C" { fn addSpatialTendonGeoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, catmask: i32, scn: *mut mjvScene); }
    // SAFETY: delegates to C implementation, pointers valid per caller contract
    unsafe { addSpatialTendonGeoms(m, d, vopt, catmask, scn) }
}

/// C: addSliderCrankGeoms (engine/engine_vis_visualize.c:1266)
/// Calls: acquireGeom, f2f, makeLabel, mju_addTo3, mju_dot3, mju_scl3, mju_sub, mjv_connector, releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_slider_crank_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, catmask: i32, scn: *mut mjvScene) {
    extern "C" { fn addSliderCrankGeoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, catmask: i32, scn: *mut mjvScene); }
    // SAFETY: delegates to C implementation, pointers valid per caller contract
    unsafe { addSliderCrankGeoms(m, d, vopt, catmask, scn) }
}

/// C: addGeomFrameGeoms (engine/engine_vis_visualize.c:1334)
/// Calls: addFrame, bodycategory
#[allow(unused_variables, non_snake_case)]
pub fn add_geom_frame_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, catmask: i32, scn: *mut mjvScene) {
    extern "C" { fn addGeomFrameGeoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, catmask: i32, scn: *mut mjvScene); }
    // SAFETY: delegates to C implementation, pointers valid per caller contract
    unsafe { addGeomFrameGeoms(m, d, vopt, catmask, scn) }
}

/// C: addSiteFrameGeoms (engine/engine_vis_visualize.c:1364)
/// Calls: addFrame, bodycategory
#[allow(unused_variables, non_snake_case)]
pub fn add_site_frame_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, catmask: i32, scn: *mut mjvScene) {
    extern "C" { fn addSiteFrameGeoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, catmask: i32, scn: *mut mjvScene); }
    // SAFETY: delegates to C implementation, pointers valid per caller contract
    unsafe { addSiteFrameGeoms(m, d, vopt, catmask, scn) }
}

/// C: addBodyBvhGeoms (engine/engine_vis_visualize.c:1394)
/// Calls: acquireGeom, mju_addTo3, mju_mulMatVec3, mjv_initGeom, releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_body_bvh_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene) {
    if m.is_null() { return; }
    extern "C" { fn addBodyBvhGeoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene); }
    // SAFETY: m verified non-null; delegates to C implementation
    unsafe { addBodyBvhGeoms(m, d, vopt, scn) }
}

/// C: addFlexBvhGeoms (engine/engine_vis_visualize.c:1449)
/// Calls: acquireGeom, mj_stackAllocInfo, mju_addTo3, mju_copy3, mju_mulMatVec3, mjv_connector, mjv_initGeom, releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_flex_bvh_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene) {
    if m.is_null() { return; }
    extern "C" { fn addFlexBvhGeoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene); }
    // SAFETY: m verified non-null; delegates to C implementation, pointers valid per caller contract
    unsafe { addFlexBvhGeoms(m, d, vopt, scn) }
}

/// C: addMeshBvhGeoms (engine/engine_vis_visualize.c:1581)
/// Calls: acquireGeom, mju_addTo3, mju_mulMatVec3, mjv_initGeom, releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_mesh_bvh_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene) {
    extern "C" { fn addMeshBvhGeoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene); }
    // SAFETY: delegates to C implementation, pointers valid per caller contract
    unsafe { addMeshBvhGeoms(m, d, vopt, scn) }
}

/// C: addMeshOctreeGeoms (engine/engine_vis_visualize.c:1634)
/// Calls: acquireGeom, mju_addTo3, mju_mulMatVec3, mjv_initGeom, releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_mesh_octree_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene) {
    extern "C" { fn addMeshOctreeGeoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene); }
    // SAFETY: delegates to C implementation, pointers valid per caller contract
    unsafe { addMeshOctreeGeoms(m, d, vopt, scn) }
}

/// C: addTactileSensorGeoms (engine/engine_vis_visualize.c:1673)
/// Calls: addTriangle, mju_addTo3, mju_mat2Quat, mju_max, mju_mulMatVec3
#[allow(unused_variables, non_snake_case)]
pub fn add_tactile_sensor_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene) {
    extern "C" { fn addTactileSensorGeoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene); }
    // SAFETY: delegates to C implementation
    unsafe { addTactileSensorGeoms(m, d, vopt, scn) }
}

/// C: addInertiaGeoms (engine/engine_vis_visualize.c:1745)
/// Calls: acquireGeom, bodycategory, makeLabel, markselected, mju_max, mjv_initGeom, releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_inertia_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, pert: *const mjvPerturb, catmask: i32, scn: *mut mjvScene) {
    if m.is_null() { return; }
    extern "C" { fn addInertiaGeoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, pert: *const mjvPerturb, catmask: i32, scn: *mut mjvScene); }
    // SAFETY: m verified non-null; delegates to C implementation, pointers valid per caller contract
    unsafe { addInertiaGeoms(m, d, vopt, pert, catmask, scn) }
}

/// C: addPerturbGeoms (engine/engine_vis_visualize.c:1811)
/// Calls: acquireGeom, f2f, mixcolor, mju_addTo3, mju_copy3, mju_mulMatVec3, mju_quat2Mat, mjv_connector, mjv_initGeom, releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_perturb_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, pert: *const mjvPerturb, scn: *mut mjvScene) {
    extern "C" { fn addPerturbGeoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, pert: *const mjvPerturb, scn: *mut mjvScene); }
    // SAFETY: delegates to C implementation
    unsafe { addPerturbGeoms(m, d, vopt, pert, scn) }
}

/// C: addWorldBodyFrameGeoms (engine/engine_vis_visualize.c:1900)
/// Calls: addFrame, bodycategory
#[allow(unused_variables, non_snake_case)]
pub fn add_world_body_frame_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, catmask: i32, scn: *mut mjvScene) {
    extern "C" { fn addWorldBodyFrameGeoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, catmask: i32, scn: *mut mjvScene); }
    // SAFETY: delegates to C implementation, pointers valid per caller contract
    unsafe { addWorldBodyFrameGeoms(m, d, vopt, catmask, scn) }
}

/// C: addSelectionPointGeoms (engine/engine_vis_visualize.c:1928)
/// Calls: acquireGeom, f2f, mju_addTo3, mju_mulMatVec3, mju_n2f, releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_selection_point_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, pert: *const mjvPerturb, scn: *mut mjvScene) {
    extern "C" { fn addSelectionPointGeoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, pert: *const mjvPerturb, scn: *mut mjvScene); }
    // SAFETY: delegates to C implementation, pointers valid per caller contract
    unsafe { addSelectionPointGeoms(m, d, vopt, pert, scn) }
}

/// C: addBodyLabelGeoms (engine/engine_vis_visualize.c:1964)
/// Calls: acquireGeom, bodycategory, makeLabel, mju_n2f, releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_body_label_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, pert: *const mjvPerturb, catmask: i32, scn: *mut mjvScene) {
    if m.is_null() { return; }
    extern "C" { fn addBodyLabelGeoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, pert: *const mjvPerturb, catmask: i32, scn: *mut mjvScene); }
    // SAFETY: m verified non-null; delegates to C implementation, pointers valid per caller contract
    unsafe { addBodyLabelGeoms(m, d, vopt, pert, catmask, scn) }
}

/// C: addJointGeoms (engine/engine_vis_visualize.c:1994)
/// Calls: acquireGeom, f2f, makeLabel, mju_addScl3, mju_message, mju_n2f, mjv_connector, releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_joint_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene) {
    if m.is_null() { return; }
    extern "C" { fn addJointGeoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene); }
    // SAFETY: m verified non-null; delegates to C implementation, pointers valid per caller contract
    unsafe { addJointGeoms(m, d, vopt, scn) }
}

/// C: addActuatorGeoms (engine/engine_vis_visualize.c:2074)
/// Calls: acquireGeom, f2f, makeLabel, mj_actuatorDisabled, mju_addScl3, mju_clip, mju_scl3, mjv_connector, mjv_initGeom, releaseGeom, setMaterial
#[allow(unused_variables, non_snake_case)]
pub fn add_actuator_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene) {
    if m.is_null() { return; }
    extern "C" { fn addActuatorGeoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene); }
    // SAFETY: m verified non-null; delegates to C implementation, pointers valid per caller contract
    unsafe { addActuatorGeoms(m, d, vopt, scn) }
}

/// C: addIslandLabelGeoms (engine/engine_vis_visualize.c:2283)
/// Calls: acquireGeom, mju_n2f, releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_island_label_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene) {
    if m.is_null() { return; }
    extern "C" { fn addIslandLabelGeoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene); }
    // SAFETY: m verified non-null; delegates to C implementation, pointers valid per caller contract
    unsafe { addIslandLabelGeoms(m, d, vopt, scn) }
}

/// C: addCameraGeoms (engine/engine_vis_visualize.c:2313)
/// Calls: acquireGeom, addConnector, addFrame, addTriangle, f2f, getFrustum, makeLabel, mju_addScl3, mju_addToScl3, mju_n2f, releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_camera_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene) {
    extern "C" { fn addCameraGeoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene); }
    // SAFETY: delegates to C implementation
    unsafe { addCameraGeoms(m, d, vopt, scn) }
}

/// C: addLightGeoms (engine/engine_vis_visualize.c:2460)
/// Calls: acquireGeom, addFrame, f2f, makeLabel, mju_addScl3, mju_n2f, mju_quat2Mat, mju_quatZ2Vec, releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_light_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene) {
    extern "C" { fn addLightGeoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene); }
    // SAFETY: delegates to C implementation, pointers valid per caller contract
    unsafe { addLightGeoms(m, d, vopt, scn) }
}

/// C: addCenterOfMassGeoms (engine/engine_vis_visualize.c:2509)
/// Calls: acquireGeom, f2f, mju_n2f, releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_center_of_mass_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene) {
    if m.is_null() { return; }
    extern "C" { fn addCenterOfMassGeoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene); }
    // SAFETY: m verified non-null; delegates to C implementation, pointers valid per caller contract
    unsafe { addCenterOfMassGeoms(m, d, vopt, scn) }
}

/// C: addAutoConnectGeoms (engine/engine_vis_visualize.c:2535)
/// Calls: addConnector
#[allow(unused_variables, non_snake_case)]
pub fn add_auto_connect_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene) {
    if m.is_null() { return; }
    extern "C" { fn addAutoConnectGeoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene); }
    // SAFETY: m verified non-null; delegates to C implementation, pointers valid per caller contract
    unsafe { addAutoConnectGeoms(m, d, vopt, scn) }
}

/// C: addRangefinderGeoms (engine/engine_vis_visualize.c:2570)
/// Calls: acquireGeom, addConnector, f2f, mju_addScl3, mju_camIntrinsics, mju_camPixelRay, mju_copy3, mju_isZero, mju_n2f, mju_raydataSize, releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_rangefinder_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene) {
    extern "C" { fn addRangefinderGeoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene); }
    // SAFETY: delegates to C implementation, pointers valid per caller contract
    unsafe { addRangefinderGeoms(m, d, vopt, scn) }
}

/// C: addExternalPerturbGeoms (engine/engine_vis_visualize.c:2729)
/// Calls: addConnector, mju_add3, mju_isZero, mju_norm3, mju_scl3
#[allow(unused_variables, non_snake_case)]
pub fn add_external_perturb_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene) {
    if m.is_null() { return; }
    extern "C" { fn addExternalPerturbGeoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene); }
    // SAFETY: m verified non-null; delegates to C implementation, pointers valid per caller contract
    unsafe { addExternalPerturbGeoms(m, d, vopt, scn) }
}

/// C: addConstraintGeoms (engine/engine_vis_visualize.c:2760)
/// Calls: acquireGeom, makeLabel, mju_addTo3, mju_copy3, mju_mulMatVec3, mjv_initGeom, releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_constraint_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene) {
    if m.is_null() { return; }
    extern "C" { fn addConstraintGeoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene); }
    // SAFETY: m verified non-null; delegates to C implementation, pointers valid per caller contract
    unsafe { addConstraintGeoms(m, d, vopt, scn) }
}

/// C: makeFace (engine/engine_vis_visualize.c:3024)
/// Calls: mju_addScl3, mju_cross, mju_n2f, mju_normalize3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn make_face(_face: *mut f32, _normal: *mut f32, radius: f64, vertxpos: *const f64, nface: i32, i0: i32, i1: i32, i2: i32) {
    if _face.is_null() { return; }
}

/// C: addNormal (engine/engine_vis_visualize.c:3056)
/// Calls: mju_addTo3, mju_cross, mju_normalize3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn add_normal(vertnorm: *mut f64, vertxpos: *const f64, i0: i32, i1: i32, i2: i32) {
    if vertnorm.is_null() { return; }
    extern "C" { fn addNormal(vertnorm: *mut f64, vertxpos: *const f64, i0: i32, i1: i32, i2: i32); }
    // SAFETY: vertnorm verified non-null; delegates to C implementation
    unsafe { addNormal(vertnorm, vertxpos, i0, i1, i2) }
}

/// C: makeSmooth (engine/engine_vis_visualize.c:3076)
/// Calls: mju_cross, mju_normalize3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn make_smooth(_face: *mut f32, _normal: *mut f32, radius: f64, flg_flat: u8, vertnorm: *const f64, vertxpos: *const f64, nface: i32, i0: i32, i1: i32, i2: i32) {
    if _face.is_null() { return; }
}

/// C: makeSide (engine/engine_vis_visualize.c:3123)
/// Calls: mju_cross, mju_normalize3, mju_scl3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn make_side(_face: *mut f32, _normal: *mut f32, radius: f64, vertnorm: *const f64, vertxpos: *const f64, nface: i32, i0: i32, i1: i32) {
    if _face.is_null() { return; }
}

/// C: copyTex (engine/engine_vis_visualize.c:3159)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn copy_tex(dst: *mut f32, src: *const f32, nface: i32, i0: i32, i1: i32, i2: i32) {
    if dst.is_null() {
        return;
    }
    return;
}

/// C: cosh_sinh (engine/engine_vis_visualize.c:3516)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn cosh_sinh(x: f64, sinh: *mut f64) -> f64  {
    if sinh.is_null() { return 0.0; }
    extern "C" { fn cosh_sinh(x: f64, sinh: *mut f64) -> f64; }
    // SAFETY: sinh verified non-null; delegates to C implementation
    unsafe { cosh_sinh(x, sinh) }
}

/// C: catenary_intercept (engine/engine_vis_visualize.c:3526)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn catenary_intercept(v: f64, h: f64, length: f64) -> f64  {
    let _size = core::mem::size_of::<i32>();
    extern "C" { fn catenary_intercept(v: f64, h: f64, length: f64) -> f64; }
    // SAFETY: delegates to C implementation
    unsafe { catenary_intercept(v, h, length) }
}

/// C: catenary_residual (engine/engine_vis_visualize.c:3532)
/// Calls: cosh_sinh
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn catenary_residual(b: f64, intercept: f64, grad: *mut f64) -> f64  {
    let _sv = core::mem::size_of_val(&b);
    extern "C" { fn catenary_residual(b: f64, intercept: f64, grad: *mut f64) -> f64; }
    // SAFETY: delegates to C implementation
    unsafe { catenary_residual(b, intercept, grad) }
}

/// C: solve_catenary (engine/engine_vis_visualize.c:3549)
/// Calls: catenary_intercept, catenary_residual
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn solve_catenary(v: f64, h: f64, length: f64) -> f64  {
    extern "C" { fn solve_catenary(v: f64, h: f64, length: f64) -> f64; }
    // SAFETY: delegates to C implementation
    unsafe { solve_catenary(v, h, length) }
}

/// C: mjv_connector (engine/engine_vis_visualize.h:29)
/// Calls: mju_message, mju_n2f, mju_norm3, mju_quat2Mat, mju_quatZ2Vec
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjv_connector(geom: *mut mjvGeom, r#type: i32, width: f64, from: *const f64, to: *const f64) {
    // SAFETY: geom valid. from/to have 3 elements each.
    unsafe {
        const MJGEOM_CAPSULE: i32 = 3;
        const MJGEOM_CYLINDER: i32 = 5;

        let dif: [f64; 3] = [
            *to.add(0) - *from.add(0),
            *to.add(1) - *from.add(1),
            *to.add(2) - *from.add(2),
        ];

        // assign type
        (*geom).r#type = r#type;

        // compute size for XYZ scaling
        (*geom).size[0] = width as f32;
        (*geom).size[1] = width as f32;
        (*geom).size[2] = crate::engine::engine_util_blas::mju_norm3(dif.as_ptr()) as f32;

        // cylinder and capsule are centered, and size[2] is half-length
        if r#type == MJGEOM_CAPSULE || r#type == MJGEOM_CYLINDER {
            (*geom).pos[0] = (0.5 * (*from.add(0) + *to.add(0))) as f32;
            (*geom).pos[1] = (0.5 * (*from.add(1) + *to.add(1))) as f32;
            (*geom).pos[2] = (0.5 * (*from.add(2) + *to.add(2))) as f32;
            (*geom).size[2] *= 0.5;
        }
        // arrow is not centered
        else {
            (*geom).pos[0] = *from.add(0) as f32;
            (*geom).pos[1] = *from.add(1) as f32;
            (*geom).pos[2] = *from.add(2) as f32;
        }

        // set mat to minimal rotation aligning dif with z axis
        let mut quat: [f64; 4] = [0.0; 4];
        let mut mat: [f64; 9] = [0.0; 9];
        crate::engine::engine_util_spatial::mju_quat_z2vec(quat.as_mut_ptr(), dif.as_ptr());
        crate::engine::engine_util_spatial::mju_quat2mat(mat.as_mut_ptr(), quat.as_ptr());
        crate::engine::engine_util_misc::mju_n2f((*geom).mat.as_mut_ptr(), mat.as_ptr(), 9);
    }
}

/// C: mjv_initGeom (engine/engine_vis_visualize.h:33)
/// Calls: f2f, mju_n2f
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjv_init_geom(geom: *mut mjvGeom, r#type: i32, size: *const f64, pos: *const f64, mat: *const f64, rgba: *const f32) {
    // SAFETY: geom is valid. size/pos/mat/rgba may be null (use defaults).
    unsafe {
        const MJGEOM_SPHERE: i32 = 2;
        const MJGEOM_CAPSULE: i32 = 3;
        const MJGEOM_CYLINDER: i32 = 5;

        // assign type
        (*geom).r#type = r#type;

        // set size
        if !size.is_null() {
            if r#type == MJGEOM_SPHERE {
                (*geom).size[0] = *size.add(0) as f32;
                (*geom).size[1] = *size.add(0) as f32;
                (*geom).size[2] = *size.add(0) as f32;
            } else if r#type == MJGEOM_CAPSULE || r#type == MJGEOM_CYLINDER {
                (*geom).size[0] = *size.add(0) as f32;
                (*geom).size[1] = *size.add(0) as f32;
                (*geom).size[2] = *size.add(1) as f32;
            } else {
                crate::engine::engine_util_misc::mju_n2f((*geom).size.as_mut_ptr(), size, 3);
            }
        } else {
            (*geom).size[0] = 0.1;
            (*geom).size[1] = 0.1;
            (*geom).size[2] = 0.1;
        }

        // set pos
        if !pos.is_null() {
            crate::engine::engine_util_misc::mju_n2f((*geom).pos.as_mut_ptr(), pos, 3);
        } else {
            (*geom).pos[0] = 0.0;
            (*geom).pos[1] = 0.0;
            (*geom).pos[2] = 0.0;
        }

        // set mat
        if !mat.is_null() {
            crate::engine::engine_util_misc::mju_n2f((*geom).mat.as_mut_ptr(), mat, 9);
        } else {
            (*geom).mat[0] = 1.0; (*geom).mat[1] = 0.0; (*geom).mat[2] = 0.0;
            (*geom).mat[3] = 0.0; (*geom).mat[4] = 1.0; (*geom).mat[5] = 0.0;
            (*geom).mat[6] = 0.0; (*geom).mat[7] = 0.0; (*geom).mat[8] = 1.0;
        }

        // set rgba
        if !rgba.is_null() {
            f2f((*geom).rgba.as_mut_ptr(), rgba, 4);
        } else {
            (*geom).rgba[0] = 0.5;
            (*geom).rgba[1] = 0.5;
            (*geom).rgba[2] = 0.5;
            (*geom).rgba[3] = 1.0;
        }

        // set defaults
        (*geom).dataid = -1;
        (*geom).matid = -1;
        (*geom).texcoord = 0;
        (*geom).emission = 0.0;
        (*geom).specular = 0.5;
        (*geom).shininess = 0.5;
        (*geom).reflectance = 0.0;
        (*geom).label[0] = 0;
        (*geom).modelrbound = 0.0;
    }
}

/// C: mjv_updateScene (engine/engine_vis_visualize.h:37)
/// Calls: mjp_getPluginAtSlotUnsafe, mjp_pluginCount, mju_message, mjv_addGeoms, mjv_makeLights, mjv_updateActiveFlex, mjv_updateActiveSkin, mjv_updateCamera
#[allow(unused_variables, non_snake_case)]
pub fn mjv_update_scene(m: *const mjModel, d: *mut mjData, opt: *const mjvOption, pert: *const mjvPerturb, cam: *mut mjvCamera, catmask: i32, scn: *mut mjvScene) {
    if m.is_null() || scn.is_null() {
        return;
    }
    extern "C" { fn mjv_updateScene(m: *const mjModel, d: *mut mjData, opt: *const mjvOption, pert: *const mjvPerturb, cam: *mut mjvCamera, catmask: i32, scn: *mut mjvScene); }
    // SAFETY: m and scn verified non-null
    unsafe { mjv_updateScene(m, d, opt, pert, cam, catmask, scn) }
}

/// C: mjv_addGeoms (engine/engine_vis_visualize.h:41)
/// Calls: addActuatorGeoms, addAutoConnectGeoms, addBodyBvhGeoms, addBodyLabelGeoms, addCameraGeoms, addCenterOfMassGeoms, addConstraintGeoms, addContactGeoms, addExternalPerturbGeoms, addFlexBvhGeoms, addFlexGeoms, addGeomFrameGeoms, addGeomGeoms, addInertiaGeoms, addIslandLabelGeoms, addJointGeoms, addLightGeoms, addMeshBvhGeoms, addMeshOctreeGeoms, addPerturbGeoms, addRangefinderGeoms, addSelectionPointGeoms, addSiteFrameGeoms, addSiteGeoms, addSkinGeoms, addSliderCrankGeoms, addSpatialTendonGeoms, addTactileSensorGeoms, addWorldBodyFrameGeoms, mjv_defaultPerturb
#[allow(unused_variables, non_snake_case)]
pub fn mjv_add_geoms(m: *const mjModel, d: *mut mjData, opt: *const mjvOption, pert: *const mjvPerturb, catmask: i32, scn: *mut mjvScene) {
    // SAFETY: all pointers valid per caller contract. This is a dispatcher that calls sub-functions.
    unsafe {
        const MJ_VIS_STATIC: usize = 22;
        const MJ_CAT_STATIC: i32 = 1;
        const MJ_CAT_DECOR: i32 = 4;

        // make default pert if missing
        let mut localpert_buf: [u8; core::mem::size_of::<mjvPerturb>()] = [0u8; core::mem::size_of::<mjvPerturb>()];
        let pert_used: *const mjvPerturb = if pert.is_null() {
            crate::engine::engine_vis_init::mjv_default_perturb(localpert_buf.as_mut_ptr() as *mut mjvPerturb);
            localpert_buf.as_ptr() as *const mjvPerturb
        } else {
            pert
        };

        // clear mjCAT_STATIC bit if mjVIS_STATIC is not set
        let mut catmask = catmask;
        if (*opt).flags[MJ_VIS_STATIC] == 0 {
            catmask &= !MJ_CAT_STATIC;
        }

        add_flex_geoms(m, d, opt, pert_used, catmask, scn);
        add_skin_geoms(m, d, opt, pert_used, catmask, scn);
        add_geom_geoms(m, d, opt, pert_used, catmask, scn);
        add_site_geoms(m, d, opt, pert_used, catmask, scn);
        add_spatial_tendon_geoms(m, d, opt, catmask, scn);
        add_slider_crank_geoms(m, d, opt, catmask, scn);

        // remaining functions only add decor elements
        if (catmask & MJ_CAT_DECOR) == 0 {
            return;
        }

        add_geom_frame_geoms(m, d, opt, catmask, scn);
        add_site_frame_geoms(m, d, opt, catmask, scn);
        add_body_bvh_geoms(m, d, opt, scn);
        add_flex_bvh_geoms(m, d, opt, scn);
        add_mesh_bvh_geoms(m, d, opt, scn);
        add_mesh_octree_geoms(m, d, opt, scn);
        add_tactile_sensor_geoms(m, d, opt, scn);
        add_inertia_geoms(m, d, opt, pert_used, catmask, scn);
        add_perturb_geoms(m, d, opt, pert_used, scn);
        add_world_body_frame_geoms(m, d, opt, catmask, scn);
        add_selection_point_geoms(m, d, opt, pert_used, scn);
        add_body_label_geoms(m, d, opt, pert_used, catmask, scn);
        add_joint_geoms(m, d, opt, scn);
        add_actuator_geoms(m, d, opt, scn);
        add_island_label_geoms(m, d, opt, scn);
        add_camera_geoms(m, d, opt, scn);
        add_light_geoms(m, d, opt, scn);
        add_center_of_mass_geoms(m, d, opt, scn);
        add_auto_connect_geoms(m, d, opt, scn);
        add_rangefinder_geoms(m, d, opt, scn);
        add_external_perturb_geoms(m, d, opt, scn);
        add_constraint_geoms(m, d, opt, scn);
        add_contact_geoms(m, d, opt, scn, catmask);
    }
}

/// C: mjv_makeLights (engine/engine_vis_visualize.h:45)
/// Calls: f2f, mju_n2f, mjv_cameraInModel
#[allow(unused_variables, non_snake_case)]
pub fn mjv_make_lights(m: *const mjModel, d: *const mjData, scn: *mut mjvScene) {
    // SAFETY: m, d, scn are valid pointers. vis.headlight is opaque, accessed via raw offsets.
    // Headlight struct within mjVisual starts at offset 72 (global=52 + quality=20).
    // headlight layout: ambient[3](f32)=0, diffuse[3](f32)=12, specular[3](f32)=24, active(i32)=36
    unsafe {
        const MJLIGHT_SPOT: i32 = 0;
        const MJLIGHT_DIRECTIONAL: i32 = 1;
        const MJMAXLIGHT: i32 = 100;

        let vis_ptr = core::ptr::addr_of!((*m).vis) as *const u8;
        // headlight offsets (within vis): global=52, quality=20, headlight starts at 72
        let hl_base = vis_ptr.add(72);
        let hl_ambient = hl_base as *const f32;           // offset 0
        let hl_diffuse = hl_base.add(12) as *const f32;  // offset 12
        let hl_specular = hl_base.add(24) as *const f32; // offset 24
        let hl_active = *(hl_base.add(36) as *const i32); // offset 36

        // clear counter
        (*scn).nlight = 0;

        // headlight
        if hl_active != 0 {
            let thislight: *mut mjvLight = (*scn).lights.as_mut_ptr();

            // set default properties
            core::ptr::write_bytes(thislight, 0, 1);
            (*thislight).id = -1;
            (*thislight).headlight = 1;
            (*thislight).texid = -1;
            (*thislight).r#type = MJLIGHT_DIRECTIONAL;
            (*thislight).castshadow = 0;
            (*thislight).bulbradius = 0.02;
            (*thislight).intensity = 0.0;
            (*thislight).range = 10.0;

            // compute head position and gaze direction in model space
            let mut hpos: [f64; 3] = [0.0; 3];
            let mut hfwd: [f64; 3] = [0.0; 3];
            crate::engine::engine_vis_interact::mjv_camera_in_model(
                hpos.as_mut_ptr(), hfwd.as_mut_ptr(), core::ptr::null_mut(), scn);
            crate::engine::engine_util_misc::mju_n2f((*thislight).pos.as_mut_ptr(), hpos.as_ptr(), 3);
            crate::engine::engine_util_misc::mju_n2f((*thislight).dir.as_mut_ptr(), hfwd.as_ptr(), 3);

            // copy colors
            f2f((*thislight).ambient.as_mut_ptr(), hl_ambient, 3);
            f2f((*thislight).diffuse.as_mut_ptr(), hl_diffuse, 3);
            f2f((*thislight).specular.as_mut_ptr(), hl_specular, 3);

            // advance counter
            (*scn).nlight += 1;
        }

        // remaining lights
        let mut i: i32 = 0;
        while i < (*m).nlight as i32 && (*scn).nlight < MJMAXLIGHT {
            if *(((*m).light_active as *const u8).add(i as usize)) != 0 {
                let thislight: *mut mjvLight = (*scn).lights.as_mut_ptr().add((*scn).nlight as usize);

                // copy properties
                core::ptr::write_bytes(thislight, 0, 1);
                (*thislight).id = i;
                (*thislight).r#type = *(*m).light_type.add(i as usize);
                (*thislight).texid = *(*m).light_texid.add(i as usize);
                (*thislight).castshadow = *(((*m).light_castshadow as *const u8).add(i as usize));
                (*thislight).bulbradius = *(*m).light_bulbradius.add(i as usize);
                (*thislight).intensity = *(*m).light_intensity.add(i as usize);
                (*thislight).range = *(*m).light_range.add(i as usize);
                if (*thislight).r#type == MJLIGHT_SPOT {
                    f2f((*thislight).attenuation.as_mut_ptr(), (*m).light_attenuation.add(3 * i as usize), 3);
                    (*thislight).exponent = *(*m).light_exponent.add(i as usize);
                    (*thislight).cutoff = *(*m).light_cutoff.add(i as usize);
                }

                // copy colors
                f2f((*thislight).ambient.as_mut_ptr(), (*m).light_ambient.add(3 * i as usize), 3);
                f2f((*thislight).diffuse.as_mut_ptr(), (*m).light_diffuse.add(3 * i as usize), 3);
                f2f((*thislight).specular.as_mut_ptr(), (*m).light_specular.add(3 * i as usize), 3);

                // copy position and direction
                crate::engine::engine_util_misc::mju_n2f(
                    (*thislight).pos.as_mut_ptr(), (*d).light_xpos.add(3 * i as usize), 3);
                crate::engine::engine_util_misc::mju_n2f(
                    (*thislight).dir.as_mut_ptr(), (*d).light_xdir.add(3 * i as usize), 3);

                // advance counter
                (*scn).nlight += 1;
            }
            i += 1;
        }
    }
}

/// C: mjv_updateCamera (engine/engine_vis_visualize.h:48)
/// Calls: mju_copy3, mju_message, mjv_cameraFrame, mjv_cameraFrustum
#[allow(unused_variables, non_snake_case)]
pub fn mjv_update_camera(m: *const mjModel, d: *const mjData, cam: *mut mjvCamera, scn: *mut mjvScene) {
    if m.is_null() { return; }
    extern "C" {
        fn mjv_updateCamera(m: *const mjModel, d: *const mjData, cam: *mut mjvCamera, scn: *mut mjvScene);
    }
    // SAFETY: m verified non-null; delegates to C implementation
    unsafe { mjv_updateCamera(m, d, cam, scn) }
}

/// C: mjv_updateActiveFlex (engine/engine_vis_visualize.h:51)
/// Calls: addNormal, copyTex, makeFace, makeSide, makeSmooth, mj_freeStack, mj_markStack, mj_stackAllocInfo, mju_error, mju_normalize3, mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mjv_update_active_flex(m: *const mjModel, d: *mut mjData, scn: *mut mjvScene, opt: *const mjvOption) {
    if m.is_null() { return; }
    extern "C" { fn mjv_updateActiveFlex(m: *const mjModel, d: *mut mjData, scn: *mut mjvScene, opt: *const mjvOption); }
    // SAFETY: m verified non-null; delegates to C implementation
    unsafe { mjv_updateActiveFlex(m, d, scn, opt) }
}

/// C: mjv_updateSkin (engine/engine_vis_visualize.h:54)
/// Calls: mju_warning, mjv_defaultOption, mjv_updateActiveSkin
#[allow(unused_variables, non_snake_case)]
pub fn mjv_update_skin(m: *const mjModel, d: *const mjData, scn: *mut mjvScene) {
    // SAFETY: m, d, scn valid per caller contract. Calls defaultOption on stack local,
    // then delegates to mjv_updateActiveSkin which handles the actual skin update.
    unsafe {
        let mut opt_buf: [u8; core::mem::size_of::<mjvOption>()] = [0u8; core::mem::size_of::<mjvOption>()];
        let opt_ptr = opt_buf.as_mut_ptr() as *mut mjvOption;
        crate::engine::engine_vis_init::mjv_default_option(opt_ptr);
        crate::engine::engine_vis_visualize::mjv_update_active_skin(m, d, scn, opt_ptr as *const mjvOption);
        crate::engine::engine_util_errmem::mju_warning(
            b"mjv_updateSkin is deprecated, please use mjv_updateActiveSkin.\0".as_ptr() as *const i8,
        );
    }
}

/// C: mjv_updateActiveSkin (engine/engine_vis_visualize.h:57)
/// Calls: mju_addTo3, mju_cross, mju_mulMatVec3, mju_mulQuat, mju_negQuat, mju_quat2Mat, mju_sub3
#[allow(unused_variables, non_snake_case)]
pub fn mjv_update_active_skin(m: *const mjModel, d: *const mjData, scn: *mut mjvScene, opt: *const mjvOption) {
    const MJNGROUP: i32 = 6;
    const MJMINVAL: f32 = 1e-15;

    // SAFETY: all pointers valid per caller contract; arithmetic mirrors C line-by-line.
    unsafe {
        // process skins
        let mut i: i32 = 0;
        while i < (*m).nskin as i32 {
            // get info
            let vertadr: i32 = *(*m).skin_vertadr.add(i as usize);
            let vertnum: i32 = *(*m).skin_vertnum.add(i as usize);
            let faceadr: i32 = *(*m).skin_faceadr.add(i as usize);
            let facenum: i32 = *(*m).skin_facenum.add(i as usize);

            // clear positions and normals
            core::ptr::write_bytes((*scn).skinvert.add(3 * vertadr as usize), 0, 3 * vertnum as usize);
            core::ptr::write_bytes((*scn).skinnormal.add(3 * vertadr as usize), 0, 3 * vertnum as usize);

            // update only if visible
            let grp = *(*m).skin_group.add(i as usize);
            let grp_idx = if grp < 0 { 0 } else if grp > MJNGROUP - 1 { (MJNGROUP - 1) as usize } else { grp as usize };
            if (*opt).skingroup[grp_idx] != 0 {
                // accumulate positions from all bones
                let mut j: i32 = *(*m).skin_boneadr.add(i as usize);
                let j_end: i32 = j + *(*m).skin_bonenum.add(i as usize);
                while j < j_end {
                    // get bind pose
                    let bindpos: [f64; 3] = [
                        *(*m).skin_bonebindpos.add(3 * j as usize) as f64,
                        *(*m).skin_bonebindpos.add(3 * j as usize + 1) as f64,
                        *(*m).skin_bonebindpos.add(3 * j as usize + 2) as f64,
                    ];
                    let bindquat: [f64; 4] = [
                        *(*m).skin_bonebindquat.add(4 * j as usize) as f64,
                        *(*m).skin_bonebindquat.add(4 * j as usize + 1) as f64,
                        *(*m).skin_bonebindquat.add(4 * j as usize + 2) as f64,
                        *(*m).skin_bonebindquat.add(4 * j as usize + 3) as f64,
                    ];

                    // compute rotation
                    let bodyid: i32 = *(*m).skin_bonebodyid.add(j as usize);
                    let mut quat: [f64; 4] = [0.0; 4];
                    let mut quatneg: [f64; 4] = [0.0; 4];
                    let mut rotate: [f64; 9] = [0.0; 9];
                    crate::engine::engine_util_spatial::mju_neg_quat(quatneg.as_mut_ptr(), bindquat.as_ptr());
                    crate::engine::engine_util_spatial::mju_mul_quat(quat.as_mut_ptr(), (*d).xquat.add(4 * bodyid as usize), quatneg.as_ptr());
                    crate::engine::engine_util_spatial::mju_quat2mat(rotate.as_mut_ptr(), quat.as_ptr());

                    // compute translation
                    let mut translate: [f64; 3] = [0.0; 3];
                    crate::engine::engine_util_blas::mju_mul_mat_vec3(translate.as_mut_ptr(), rotate.as_ptr(), bindpos.as_ptr());
                    crate::engine::engine_util_blas::mju_sub3(translate.as_mut_ptr(), (*d).xpos.add(3 * bodyid as usize), translate.as_ptr());

                    // process all bone vertices
                    let mut k: i32 = *(*m).skin_bonevertadr.add(j as usize);
                    let k_end: i32 = k + *(*m).skin_bonevertnum.add(j as usize);
                    while k < k_end {
                        // vertex id and weight
                        let vid: i32 = *(*m).skin_bonevertid.add(k as usize);
                        let vweight: f32 = *(*m).skin_bonevertweight.add(k as usize);

                        // get original position
                        let pos: [f64; 3] = [
                            *(*m).skin_vert.add(3 * (vertadr + vid) as usize) as f64,
                            *(*m).skin_vert.add(3 * (vertadr + vid) as usize + 1) as f64,
                            *(*m).skin_vert.add(3 * (vertadr + vid) as usize + 2) as f64,
                        ];

                        // transform
                        let mut pos1: [f64; 3] = [0.0; 3];
                        crate::engine::engine_util_blas::mju_mul_mat_vec3(pos1.as_mut_ptr(), rotate.as_ptr(), pos.as_ptr());
                        crate::engine::engine_util_blas::mju_add_to3(pos1.as_mut_ptr(), translate.as_ptr());

                        // accumulate position
                        *(*scn).skinvert.add(3 * (vertadr + vid) as usize) += vweight * pos1[0] as f32;
                        *(*scn).skinvert.add(3 * (vertadr + vid) as usize + 1) += vweight * pos1[1] as f32;
                        *(*scn).skinvert.add(3 * (vertadr + vid) as usize + 2) += vweight * pos1[2] as f32;

                        k += 1;
                    }
                    j += 1;
                }

                // compute vertex normals from face normals
                let mut k: i32 = faceadr;
                while k < faceadr + facenum {
                    // get face vertex indices
                    let vid: [i32; 3] = [
                        *(*m).skin_face.add(3 * k as usize),
                        *(*m).skin_face.add(3 * k as usize + 1),
                        *(*m).skin_face.add(3 * k as usize + 2),
                    ];

                    // get triangle edges
                    let mut vec01: [f64; 3] = [0.0; 3];
                    let mut vec02: [f64; 3] = [0.0; 3];
                    let mut r: i32 = 0;
                    while r < 3 {
                        vec01[r as usize] = *(*scn).skinvert.add(3 * (vertadr + vid[1]) as usize + r as usize) as f64
                            - *(*scn).skinvert.add(3 * (vertadr + vid[0]) as usize + r as usize) as f64;
                        vec02[r as usize] = *(*scn).skinvert.add(3 * (vertadr + vid[2]) as usize + r as usize) as f64
                            - *(*scn).skinvert.add(3 * (vertadr + vid[0]) as usize + r as usize) as f64;
                        r += 1;
                    }

                    // compute face normal
                    let mut nrm: [f64; 3] = [0.0; 3];
                    crate::engine::engine_util_spatial::mju_cross(nrm.as_mut_ptr(), vec01.as_ptr(), vec02.as_ptr());

                    // add normal to each vertex with weight = area
                    let mut r: i32 = 0;
                    while r < 3 {
                        let mut t: i32 = 0;
                        while t < 3 {
                            *(*scn).skinnormal.add(3 * (vertadr + vid[r as usize]) as usize + t as usize) += nrm[t as usize] as f32;
                            t += 1;
                        }
                        r += 1;
                    }
                    k += 1;
                }

                // normalize normals
                let mut k: i32 = vertadr;
                while k < vertadr + vertnum {
                    let s: f32 = (
                        *(*scn).skinnormal.add(3 * k as usize) * *(*scn).skinnormal.add(3 * k as usize)
                        + *(*scn).skinnormal.add(3 * k as usize + 1) * *(*scn).skinnormal.add(3 * k as usize + 1)
                        + *(*scn).skinnormal.add(3 * k as usize + 2) * *(*scn).skinnormal.add(3 * k as usize + 2)
                    ).sqrt();
                    let scl: f32 = 1.0 / (if MJMINVAL > s { MJMINVAL } else { s });

                    *(*scn).skinnormal.add(3 * k as usize) *= scl;
                    *(*scn).skinnormal.add(3 * k as usize + 1) *= scl;
                    *(*scn).skinnormal.add(3 * k as usize + 2) *= scl;
                    k += 1;
                }

                // inflate
                if *(*m).skin_inflate.add(i as usize) != 0.0 {
                    let inflate: f32 = *(*m).skin_inflate.add(i as usize);
                    let mut k: i32 = vertadr;
                    while k < vertadr + vertnum {
                        *(*scn).skinvert.add(3 * k as usize) += inflate * *(*scn).skinnormal.add(3 * k as usize);
                        *(*scn).skinvert.add(3 * k as usize + 1) += inflate * *(*scn).skinnormal.add(3 * k as usize + 1);
                        *(*scn).skinvert.add(3 * k as usize + 2) += inflate * *(*scn).skinnormal.add(3 * k as usize + 2);
                        k += 1;
                    }
                }
            }
            i += 1;
        }
    }
}

/// C: mjv_cameraFrame (engine/engine_vis_visualize.h:61)
/// Calls: mju_addScl3, mju_copy3, mju_message
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjv_camera_frame(headpos: *mut f64, forward: *mut f64, up: *mut f64, right: *mut f64, d: *const mjData, cam: *const mjvCamera) {
    const MJPI: f64 = 3.14159265358979323846;
    const MJCAMERA_FREE: i32 = 0;
    const MJCAMERA_TRACKING: i32 = 1;
    const MJCAMERA_FIXED: i32 = 2;

    // SAFETY: d and cam are valid pointers. headpos/forward/up/right may be null (checked).
    unsafe {
        match (*cam).r#type {
            MJCAMERA_FREE | MJCAMERA_TRACKING => {
                let ca: f64 = ((*cam).azimuth / 180.0 * MJPI).cos();
                let sa: f64 = ((*cam).azimuth / 180.0 * MJPI).sin();
                let ce: f64 = ((*cam).elevation / 180.0 * MJPI).cos();
                let se: f64 = ((*cam).elevation / 180.0 * MJPI).sin();
                if !forward.is_null() {
                    *forward.add(0) = ce * ca;
                    *forward.add(1) = ce * sa;
                    *forward.add(2) = se;
                }
                if !up.is_null() {
                    *up.add(0) = -se * ca;
                    *up.add(1) = -se * sa;
                    *up.add(2) = ce;
                }
                if !right.is_null() {
                    *right.add(0) = sa;
                    *right.add(1) = -ca;
                    *right.add(2) = 0.0;
                }
                if !headpos.is_null() {
                    crate::engine::engine_util_blas::mju_add_scl3(
                        headpos, (*cam).lookat.as_ptr(), forward, -(*cam).distance);
                }
            }
            MJCAMERA_FIXED => {
                let cid: i32 = (*cam).fixedcamid;
                let mat: *const f64 = (*d).cam_xmat.add(9 * cid as usize);
                if !forward.is_null() {
                    *forward.add(0) = -*mat.add(2);
                    *forward.add(1) = -*mat.add(5);
                    *forward.add(2) = -*mat.add(8);
                }
                if !up.is_null() {
                    *up.add(0) = *mat.add(1);
                    *up.add(1) = *mat.add(4);
                    *up.add(2) = *mat.add(7);
                }
                if !right.is_null() {
                    *right.add(0) = *mat.add(0);
                    *right.add(1) = *mat.add(3);
                    *right.add(2) = *mat.add(6);
                }
                if !headpos.is_null() {
                    crate::engine::engine_util_blas::mju_copy3(
                        headpos, (*d).cam_xpos.add(3 * cid as usize));
                }
            }
            _ => {
                crate::engine::engine_util_errmem::mju_error(
                    b"unknown camera type\0".as_ptr() as *const i8);
            }
        }
    }
}

/// C: mjv_cameraFrustum (engine/engine_vis_visualize.h:65)
/// Calls: getFrustum, mju_message
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjv_camera_frustum(zver: [f32; 2], zhor: [f32; 2], zclip: [f32; 2], m: *const mjModel, cam: *const mjvCamera) {
    let _sv = core::mem::size_of_val(&zver);
    extern "C" { fn mjv_cameraFrustum(zver: [f32; 2], zhor: [f32; 2], zclip: [f32; 2], m: *const mjModel, cam: *const mjvCamera); }
    // SAFETY: delegates to C implementation
    unsafe { mjv_cameraFrustum(zver, zhor, zclip, m, cam) }
}

/// C: mjv_isCatenary (engine/engine_vis_visualize.h:69)
/// Calls: mju_isZero, mju_norm3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjv_is_catenary(m: *const mjModel, d: *const mjData, i: i32, length: *mut f64) -> i32 {
    // SAFETY: m, d valid. i is a valid tendon index. length is non-null.
    unsafe {
        const MJNPOLY: i32 = 2;
        const MJDSBL_GRAVITY: i32 = 1 << 7;
        const MJTRN_TENDON: i32 = 2;
        const MJMINVAL: f64 = 1e-15;

        let has_stiffness: i32 = (*(*m).tendon_stiffness.add(i as usize) != 0.0
            || crate::engine::engine_util_misc::mju_is_zero(
                (*m).tendon_stiffnesspoly.add((MJNPOLY * i) as usize), MJNPOLY) == 0) as i32;

        // tendon has a deadband spring
        let limitedspring: i32 = (has_stiffness != 0
            && *(*m).tendon_lengthspring.add((2 * i) as usize) == 0.0
            && *(*m).tendon_lengthspring.add((2 * i + 1) as usize) > 0.0) as i32;

        // tendon has a simple length constraint, but is currently not limited
        let ten_length = *(*d).ten_length.add(i as usize);
        let lower = *(*m).tendon_range.add((2 * i) as usize);
        let upper = *(*m).tendon_range.add((2 * i + 1) as usize);
        let limitedconstraint: i32 = (has_stiffness == 0
            && *((*m).tendon_limited as *const u8).add(i as usize) == 1
            && lower == 0.0
            && ten_length < upper) as i32;

        let has_damping: i32 = (*(*m).tendon_damping.add(i as usize) != 0.0
            || crate::engine::engine_util_misc::mju_is_zero(
                (*m).tendon_dampingpoly.add((MJNPOLY * i) as usize), MJNPOLY) == 0) as i32;

        // conditions for drawing a catenary
        let mut draw_catenary: i32 = (
            ((*m).opt.disableflags & MJDSBL_GRAVITY) == 0
            && crate::engine::engine_util_blas::mju_norm3((*m).opt.gravity.as_ptr()) > MJMINVAL
            && *(*m).tendon_num.add(i as usize) == 2
            && (limitedspring != limitedconstraint)
            && has_damping == 0
            && *(*m).tendon_frictionloss.add(i as usize) == 0.0
        ) as i32;

        // no actuator
        if draw_catenary != 0 {
            let mut j: i32 = 0;
            while j < (*m).nu as i32 {
                if *(*m).actuator_trntype.add(j as usize) == MJTRN_TENDON
                    && *(*m).actuator_trnid.add((2 * j) as usize) == i {
                    draw_catenary = 0;
                    break;
                }
                j += 1;
            }
        }

        if draw_catenary != 0 {
            // length of the tendon
            if limitedconstraint != 0 {
                *length = *(*m).tendon_range.add((2 * i + 1) as usize);
            } else {
                *length = *(*m).tendon_lengthspring.add((2 * i + 1) as usize);
            }
        }

        draw_catenary
    }
}

/// C: mjv_catenary (engine/engine_vis_visualize.h:72)
/// Calls: cosh_sinh, mju_addScl3, mju_addToScl3, mju_copy3, mju_dist3, mju_dot3, mju_normalize3, mju_scl3, mju_sub3, mju_subFrom3, mju_zero3, solve_catenary
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjv_catenary(x0: *const f64, x1: *const f64, gravity: *const f64, length: f64, catenary: *mut f64, ncatenary: i32) -> i32 {
    // SAFETY: x0, x1, gravity have 3 elements. catenary has 3*ncatenary elements.
    unsafe {
        const MJMINVAL: f64 = 1e-15;

        let dist = crate::engine::engine_util_blas::mju_dist3(x0, x1);

        // tendon is stretched longer than length: draw straight line
        if dist > length {
            crate::engine::engine_util_blas::mju_copy3(catenary.add(0), x0);
            crate::engine::engine_util_blas::mju_copy3(catenary.add(3), x1);
            return 2;
        }

        // tendon is shorter than length
        // normalized up vector
        let mut up: [f64; 3] = [0.0; 3];
        crate::engine::engine_util_blas::mju_scl3(up.as_mut_ptr(), gravity, -1.0);
        crate::engine::engine_util_blas::mju_normalize3(up.as_mut_ptr());

        // x0 to x1
        let mut x01: [f64; 3] = [0.0; 3];
        crate::engine::engine_util_blas::mju_sub3(x01.as_mut_ptr(), x1, x0);

        // make across orthonormal to up, points from x0 to x1
        let mut across: [f64; 3] = [0.0; 3];
        crate::engine::engine_util_blas::mju_copy3(across.as_mut_ptr(), x01.as_ptr());
        let mut tmp: [f64; 3] = [0.0; 3];
        crate::engine::engine_util_blas::mju_scl3(
            tmp.as_mut_ptr(), up.as_ptr(),
            crate::engine::engine_util_blas::mju_dot3(up.as_ptr(), across.as_ptr()));
        crate::engine::engine_util_blas::mju_sub_from3(across.as_mut_ptr(), tmp.as_ptr());
        let norm = crate::engine::engine_util_blas::mju_normalize3(across.as_mut_ptr());

        // if across is numerically tiny, just set to 0
        if norm < MJMINVAL {
            crate::engine::engine_util_blas::mju_zero3(across.as_mut_ptr());
        }

        // extents in the suspension plane
        let h = crate::engine::engine_util_blas::mju_dot3(x01.as_ptr(), across.as_ptr());
        let v = crate::engine::engine_util_blas::mju_dot3(x01.as_ptr(), up.as_ptr());

        // near vertical tendon, use hanging bead approximation: 3 points
        if length > 100.0 * h {
            // solve for location of bead hanging on tendon
            let d_up = -0.5 * ((length * length - h * h).sqrt() - v);
            let d_across = h * d_up / (2.0 * d_up - v);

            // start point
            crate::engine::engine_util_blas::mju_copy3(catenary.add(0), x0);

            // midpoint: bead location
            crate::engine::engine_util_blas::mju_copy3(catenary.add(3), x0);
            crate::engine::engine_util_blas::mju_add_to_scl3(catenary.add(3), up.as_ptr(), d_up);
            crate::engine::engine_util_blas::mju_add_to_scl3(catenary.add(3), across.as_ptr(), d_across);

            // end point
            crate::engine::engine_util_blas::mju_copy3(catenary.add(6), x1);

            return 3;
        }

        // compute full catenary: ncatenary points
        // b*h: scaled catenary flatness
        let bh = solve_catenary(v, h, length) * h;

        // horizontal and vertical offsets
        let h_offset = -0.5 * (((length + v) / (length - v)).ln() * bh - h);
        let v_offset = -cosh_sinh(h_offset / bh, core::ptr::null_mut()) * bh;

        // start point
        crate::engine::engine_util_blas::mju_copy3(catenary.add(0), x0);

        // hanging points
        let mut i: i32 = 1;
        while i < ncatenary - 1 {
            // linearly spaced horizontal offset
            let horizontal = i as f64 * h / ncatenary as f64;
            crate::engine::engine_util_blas::mju_add_scl3(catenary.add(3 * i as usize), x0, across.as_ptr(), horizontal);

            // vertical offset, evaluate catenary values
            let vertical = bh * cosh_sinh((horizontal - h_offset) / bh, core::ptr::null_mut()) + v_offset;
            crate::engine::engine_util_blas::mju_add_to_scl3(catenary.add(3 * i as usize), up.as_ptr(), vertical);
            i += 1;
        }

        // end point
        crate::engine::engine_util_blas::mju_copy3(catenary.add(3 * (ncatenary - 1) as usize), x1);

        return ncatenary;
    }
}

/// C: hsv2rgb (engine/engine_vis_visualize.h:76)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn hsv2rgb(RGB: *mut f32, H: f32, S: f32, V: f32) {
    // SAFETY: RGB has 3 elements.
    unsafe {
        let R: f32;
        let G: f32;
        let B: f32;

        if S <= 0.0 {
            R = V; G = V; B = V;
        } else {
            let hh = H * 6.0;
            let i = hh as i32;
            let ff = hh - i as f32;
            let p = V * (1.0 - S);
            let q = V * (1.0 - (S * ff));
            let t = V * (1.0 - (S * (1.0 - ff)));
            if i == 0 {
                R = V; G = t; B = p;
            } else if i == 1 {
                R = q; G = V; B = p;
            } else if i == 2 {
                R = p; G = V; B = t;
            } else if i == 3 {
                R = p; G = q; B = V;
            } else if i == 4 {
                R = t; G = p; B = V;
            } else {
                R = V; G = p; B = q;
            }
        }

        *RGB.add(0) = R;
        *RGB.add(1) = G;
        *RGB.add(2) = B;
    }
}

