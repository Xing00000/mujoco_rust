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
    // WARNING: signature changed — verify body
    // Previous params: (dest : * mut f32, src : * const f32, n : i32)
    // Previous return: ()
    unsafe { core :: ptr :: copy_nonoverlapping (src , dest , n as usize) ; }
}

/// C: makeLabel (engine/engine_vis_visualize.c:55)
/// Calls: mj_id2name, mju_type2Str
#[allow(unused_variables, non_snake_case)]
pub fn make_label(m: *const mjModel, r#type: mjtObj, id: i32, label: *mut i8) {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, r#type : mjtObj, id : i32, label : * mut i8)
    // Previous return: ()
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (rgba : [f32 ; 4], h : i32, awake : i32)
    // Previous return: ()
    todo ! ()
}

/// C: mixcolor (engine/engine_vis_visualize.c:140)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mixcolor(rgba: [f32; 4], r#ref: [f32; 4], flg1: i32, flg2: i32) {
    // WARNING: signature changed — verify body
    // Previous params: (rgba : [f32 ; 4], r#ref : [f32 ; 4], flg1 : i32, flg2 : i32)
    // Previous return: ()
    todo ! ()
}

/// C: bodycategory (engine/engine_vis_visualize.c:157)
#[allow(unused_variables, non_snake_case)]
pub fn bodycategory(m: *const mjModel, bodyid: i32) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, bodyid : i32)
    // Previous return: i32
    todo ! ()
}

/// C: acquireGeom (engine/engine_vis_visualize.c:169)
/// Calls: mju_warning, mjv_initGeom
#[allow(unused_variables, non_snake_case)]
pub fn acquire_geom(scn: *mut mjvScene, objid: i32, category: i32, objtype: i32) -> *mut mjvGeom {
    // WARNING: signature changed — verify body
    // Previous params: (scn : * mut mjvScene, objid : i32, category : i32, objtype : i32)
    // Previous return: * mut mjvGeom
    todo ! ()
}

/// C: releaseGeom (engine/engine_vis_visualize.c:192)
/// Calls: mju_error
#[allow(unused_variables, non_snake_case)]
pub fn release_geom(geom: *mut *mut mjvGeom, scn: *mut mjvScene) {
    // WARNING: signature changed — verify body
    // Previous params: (geom : * mut * mut mjvGeom, scn : * mut mjvScene)
    // Previous return: ()
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (scn : * mut mjvScene, v0 : * const f64, v1 : * const f64, v2 : * const f64, rgba : [f32 ; 4], objid : i32, category : i32, objtype : i32)
    // Previous return: ()
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, geom : * mut mjvGeom, matid : i32, rgba : * const f32, flags : * const u8)
    // Previous return: ()
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (scn : * mut mjvScene, r#type : i32, width : f64, from : * const f64, to : * const f64, rgba : [f32 ; 4], objid : i32, category : i32, objtype : i32)
    // Previous return: ()
    todo ! ()
}

/// C: markselected (engine/engine_vis_visualize.c:393)
#[allow(unused_variables, non_snake_case)]
pub fn markselected(vis: *const mjVisual, geom: *mut mjvGeom) {
    // WARNING: signature changed — verify body
    // Previous params: (vis : * const mjVisual, geom : * mut mjvGeom)
    // Previous return: ()
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (scn : * mut mjvScene, objid : i32, pos : * const f64, rot : * const f64, length : f32, width : f32)
    // Previous return: ()
    todo ! ()
}

/// C: getFrustum (engine/engine_vis_visualize.c:434)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn get_frustum(zver: [f32; 2], zhor: [f32; 2], znear: f32, intrinsic: [f32; 4], sensorsize: [f32; 2]) {
    // WARNING: signature changed — verify body
    // Previous params: (zver : [f32 ; 2], zhor : [f32 ; 2], znear : f32, intrinsic : [f32 ; 4], sensorsize : [f32 ; 2])
    // Previous return: ()
    todo ! ()
}

/// C: addContactGeoms (engine/engine_vis_visualize.c:565)
/// Calls: acquireGeom, addFrame, f2f, islandColor, mj_contactForce, mj_id2name, mju_add3, mju_copy, mju_mulMatVec, mju_n2f, mju_norm3, mju_scl3, mju_transpose, mju_zero3, mjv_connector, releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_contact_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene, catmask: i32) {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, vopt : * const mjvOption, scn : * mut mjvScene, catmask : i32)
    // Previous return: ()
    todo ! ()
}

/// C: addFlexGeoms (engine/engine_vis_visualize.c:748)
/// Calls: acquireGeom, islandColor, makeLabel, markselected, mj_sleepCycle, mjv_initGeom, releaseGeom, setMaterial
#[allow(unused_variables, non_snake_case)]
pub fn add_flex_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, pert: *const mjvPerturb, catmask: i32, scn: *mut mjvScene) {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, vopt : * const mjvOption, pert : * const mjvPerturb, catmask : i32, scn : * mut mjvScene)
    // Previous return: ()
    todo ! ()
}

/// C: addSkinGeoms (engine/engine_vis_visualize.c:841)
/// Calls: acquireGeom, makeLabel, markselected, mjv_initGeom, releaseGeom, setMaterial
#[allow(unused_variables, non_snake_case)]
pub fn add_skin_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, pert: *const mjvPerturb, catmask: i32, scn: *mut mjvScene) {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, vopt : * const mjvOption, pert : * const mjvPerturb, catmask : i32, scn : * mut mjvScene)
    // Previous return: ()
    todo ! ()
}

/// C: addGeomGeoms (engine/engine_vis_visualize.c:892)
/// Calls: acquireGeom, bodycategory, islandColor, makeLabel, markselected, mj_sleepCycle, mju_addToScl3, mju_copy3, mju_dot3, mju_n2f, mju_round, mju_transpose, mjv_initGeom, releaseGeom, setMaterial
#[allow(unused_variables, non_snake_case)]
pub fn add_geom_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, pert: *const mjvPerturb, catmask: i32, scn: *mut mjvScene) {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, vopt : * const mjvOption, pert : * const mjvPerturb, catmask : i32, scn : * mut mjvScene)
    // Previous return: ()
    todo ! ()
}

/// C: addSiteGeoms (engine/engine_vis_visualize.c:1041)
/// Calls: acquireGeom, bodycategory, makeLabel, markselected, mjv_initGeom, releaseGeom, setMaterial
#[allow(unused_variables, non_snake_case)]
pub fn add_site_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, pert: *const mjvPerturb, catmask: i32, scn: *mut mjvScene) {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, vopt : * const mjvOption, pert : * const mjvPerturb, catmask : i32, scn : * mut mjvScene)
    // Previous return: ()
    todo ! ()
}

/// C: addSpatialTendonGeoms (engine/engine_vis_visualize.c:1141)
/// Calls: acquireGeom, f2f, islandColor, makeLabel, mju_copy3, mjv_catenary, mjv_connector, mjv_isCatenary, releaseGeom, setMaterial
#[allow(unused_variables, non_snake_case)]
pub fn add_spatial_tendon_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, catmask: i32, scn: *mut mjvScene) {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, vopt : * const mjvOption, catmask : i32, scn : * mut mjvScene)
    // Previous return: ()
    todo ! ()
}

/// C: addSliderCrankGeoms (engine/engine_vis_visualize.c:1266)
/// Calls: acquireGeom, f2f, makeLabel, mju_addTo3, mju_dot3, mju_scl3, mju_sub, mjv_connector, releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_slider_crank_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, catmask: i32, scn: *mut mjvScene) {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, vopt : * const mjvOption, catmask : i32, scn : * mut mjvScene)
    // Previous return: ()
    todo ! ()
}

/// C: addGeomFrameGeoms (engine/engine_vis_visualize.c:1334)
/// Calls: addFrame, bodycategory
#[allow(unused_variables, non_snake_case)]
pub fn add_geom_frame_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, catmask: i32, scn: *mut mjvScene) {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, vopt : * const mjvOption, catmask : i32, scn : * mut mjvScene)
    // Previous return: ()
    todo ! ()
}

/// C: addSiteFrameGeoms (engine/engine_vis_visualize.c:1364)
/// Calls: addFrame, bodycategory
#[allow(unused_variables, non_snake_case)]
pub fn add_site_frame_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, catmask: i32, scn: *mut mjvScene) {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, vopt : * const mjvOption, catmask : i32, scn : * mut mjvScene)
    // Previous return: ()
    todo ! ()
}

/// C: addBodyBvhGeoms (engine/engine_vis_visualize.c:1394)
/// Calls: acquireGeom, mju_addTo3, mju_mulMatVec3, mjv_initGeom, releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_body_bvh_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene) {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, vopt : * const mjvOption, scn : * mut mjvScene)
    // Previous return: ()
    todo ! ()
}

/// C: addFlexBvhGeoms (engine/engine_vis_visualize.c:1449)
/// Calls: acquireGeom, mj_stackAllocInfo, mju_addTo3, mju_copy3, mju_mulMatVec3, mjv_connector, mjv_initGeom, releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_flex_bvh_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene) {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, vopt : * const mjvOption, scn : * mut mjvScene)
    // Previous return: ()
    todo ! ()
}

/// C: addMeshBvhGeoms (engine/engine_vis_visualize.c:1581)
/// Calls: acquireGeom, mju_addTo3, mju_mulMatVec3, mjv_initGeom, releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_mesh_bvh_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene) {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, vopt : * const mjvOption, scn : * mut mjvScene)
    // Previous return: ()
    todo ! ()
}

/// C: addMeshOctreeGeoms (engine/engine_vis_visualize.c:1634)
/// Calls: acquireGeom, mju_addTo3, mju_mulMatVec3, mjv_initGeom, releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_mesh_octree_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene) {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, vopt : * const mjvOption, scn : * mut mjvScene)
    // Previous return: ()
    todo ! ()
}

/// C: addTactileSensorGeoms (engine/engine_vis_visualize.c:1673)
/// Calls: addTriangle, mju_addTo3, mju_mat2Quat, mju_max, mju_mulMatVec3
#[allow(unused_variables, non_snake_case)]
pub fn add_tactile_sensor_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene) {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, vopt : * const mjvOption, scn : * mut mjvScene)
    // Previous return: ()
    todo ! ()
}

/// C: addInertiaGeoms (engine/engine_vis_visualize.c:1745)
/// Calls: acquireGeom, bodycategory, makeLabel, markselected, mju_max, mjv_initGeom, releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_inertia_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, pert: *const mjvPerturb, catmask: i32, scn: *mut mjvScene) {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, vopt : * const mjvOption, pert : * const mjvPerturb, catmask : i32, scn : * mut mjvScene)
    // Previous return: ()
    todo ! ()
}

/// C: addPerturbGeoms (engine/engine_vis_visualize.c:1811)
/// Calls: acquireGeom, f2f, mixcolor, mju_addTo3, mju_copy3, mju_mulMatVec3, mju_quat2Mat, mjv_connector, mjv_initGeom, releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_perturb_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, pert: *const mjvPerturb, scn: *mut mjvScene) {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, vopt : * const mjvOption, pert : * const mjvPerturb, scn : * mut mjvScene)
    // Previous return: ()
    todo ! ()
}

/// C: addWorldBodyFrameGeoms (engine/engine_vis_visualize.c:1900)
/// Calls: addFrame, bodycategory
#[allow(unused_variables, non_snake_case)]
pub fn add_world_body_frame_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, catmask: i32, scn: *mut mjvScene) {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, vopt : * const mjvOption, catmask : i32, scn : * mut mjvScene)
    // Previous return: ()
    todo ! ()
}

/// C: addSelectionPointGeoms (engine/engine_vis_visualize.c:1928)
/// Calls: acquireGeom, f2f, mju_addTo3, mju_mulMatVec3, mju_n2f, releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_selection_point_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, pert: *const mjvPerturb, scn: *mut mjvScene) {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, vopt : * const mjvOption, pert : * const mjvPerturb, scn : * mut mjvScene)
    // Previous return: ()
    todo ! ()
}

/// C: addBodyLabelGeoms (engine/engine_vis_visualize.c:1964)
/// Calls: acquireGeom, bodycategory, makeLabel, mju_n2f, releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_body_label_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, pert: *const mjvPerturb, catmask: i32, scn: *mut mjvScene) {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, vopt : * const mjvOption, pert : * const mjvPerturb, catmask : i32, scn : * mut mjvScene)
    // Previous return: ()
    todo ! ()
}

/// C: addJointGeoms (engine/engine_vis_visualize.c:1994)
/// Calls: acquireGeom, f2f, makeLabel, mju_addScl3, mju_message, mju_n2f, mjv_connector, releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_joint_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene) {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, vopt : * const mjvOption, scn : * mut mjvScene)
    // Previous return: ()
    todo ! ()
}

/// C: addActuatorGeoms (engine/engine_vis_visualize.c:2074)
/// Calls: acquireGeom, f2f, makeLabel, mj_actuatorDisabled, mju_addScl3, mju_clip, mju_scl3, mjv_connector, mjv_initGeom, releaseGeom, setMaterial
#[allow(unused_variables, non_snake_case)]
pub fn add_actuator_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene) {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, vopt : * const mjvOption, scn : * mut mjvScene)
    // Previous return: ()
    todo ! ()
}

/// C: addIslandLabelGeoms (engine/engine_vis_visualize.c:2283)
/// Calls: acquireGeom, mju_n2f, releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_island_label_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene) {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, vopt : * const mjvOption, scn : * mut mjvScene)
    // Previous return: ()
    todo ! ()
}

/// C: addCameraGeoms (engine/engine_vis_visualize.c:2313)
/// Calls: acquireGeom, addConnector, addFrame, addTriangle, f2f, getFrustum, makeLabel, mju_addScl3, mju_addToScl3, mju_n2f, releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_camera_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene) {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, vopt : * const mjvOption, scn : * mut mjvScene)
    // Previous return: ()
    todo ! ()
}

/// C: addLightGeoms (engine/engine_vis_visualize.c:2460)
/// Calls: acquireGeom, addFrame, f2f, makeLabel, mju_addScl3, mju_n2f, mju_quat2Mat, mju_quatZ2Vec, releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_light_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene) {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, vopt : * const mjvOption, scn : * mut mjvScene)
    // Previous return: ()
    todo ! ()
}

/// C: addCenterOfMassGeoms (engine/engine_vis_visualize.c:2509)
/// Calls: acquireGeom, f2f, mju_n2f, releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_center_of_mass_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene) {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, vopt : * const mjvOption, scn : * mut mjvScene)
    // Previous return: ()
    todo ! ()
}

/// C: addAutoConnectGeoms (engine/engine_vis_visualize.c:2535)
/// Calls: addConnector
#[allow(unused_variables, non_snake_case)]
pub fn add_auto_connect_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene) {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, vopt : * const mjvOption, scn : * mut mjvScene)
    // Previous return: ()
    todo ! ()
}

/// C: addRangefinderGeoms (engine/engine_vis_visualize.c:2570)
/// Calls: acquireGeom, addConnector, f2f, mju_addScl3, mju_camIntrinsics, mju_camPixelRay, mju_copy3, mju_isZero, mju_n2f, mju_raydataSize, releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_rangefinder_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene) {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, vopt : * const mjvOption, scn : * mut mjvScene)
    // Previous return: ()
    todo ! ()
}

/// C: addExternalPerturbGeoms (engine/engine_vis_visualize.c:2729)
/// Calls: addConnector, mju_add3, mju_isZero, mju_norm3, mju_scl3
#[allow(unused_variables, non_snake_case)]
pub fn add_external_perturb_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene) {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, vopt : * const mjvOption, scn : * mut mjvScene)
    // Previous return: ()
    todo ! ()
}

/// C: addConstraintGeoms (engine/engine_vis_visualize.c:2760)
/// Calls: acquireGeom, makeLabel, mju_addTo3, mju_copy3, mju_mulMatVec3, mjv_initGeom, releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_constraint_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene) {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, vopt : * const mjvOption, scn : * mut mjvScene)
    // Previous return: ()
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (_face : * mut f32, _normal : * mut f32, radius : f64, vertxpos : * const f64, nface : i32, i0 : i32, i1 : i32, i2 : i32)
    // Previous return: ()
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (vertnorm : * mut f64, vertxpos : * const f64, i0 : i32, i1 : i32, i2 : i32)
    // Previous return: ()
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (_face : * mut f32, _normal : * mut f32, radius : f64, flg_flat : u8, vertnorm : * const f64, vertxpos : * const f64, nface : i32, i0 : i32, i1 : i32, i2 : i32)
    // Previous return: ()
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (_face : * mut f32, _normal : * mut f32, radius : f64, vertnorm : * const f64, vertxpos : * const f64, nface : i32, i0 : i32, i1 : i32)
    // Previous return: ()
    todo ! ()
}

/// C: copyTex (engine/engine_vis_visualize.c:3159)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn copy_tex(dst: *mut f32, src: *const f32, nface: i32, i0: i32, i1: i32, i2: i32) {
    // WARNING: signature changed — verify body
    // Previous params: (dst : * mut f32, src : * const f32, nface : i32, i0 : i32, i1 : i32, i2 : i32)
    // Previous return: ()
    todo ! ()
}

/// C: cosh_sinh (engine/engine_vis_visualize.c:3516)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn cosh_sinh(x: f64, sinh: *mut f64) -> f64 {
    // WARNING: signature changed — verify body
    // Previous params: (x : f64, sinh : * mut f64)
    // Previous return: f64
    todo ! ()
}

/// C: catenary_intercept (engine/engine_vis_visualize.c:3526)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn catenary_intercept(v: f64, h: f64, length: f64) -> f64 {
    // WARNING: signature changed — verify body
    // Previous params: (v : f64, h : f64, length : f64)
    // Previous return: f64
    todo ! ()
}

/// C: catenary_residual (engine/engine_vis_visualize.c:3532)
/// Calls: cosh_sinh
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn catenary_residual(b: f64, intercept: f64, grad: *mut f64) -> f64 {
    // WARNING: signature changed — verify body
    // Previous params: (b : f64, intercept : f64, grad : * mut f64)
    // Previous return: f64
    todo ! ()
}

/// C: solve_catenary (engine/engine_vis_visualize.c:3549)
/// Calls: catenary_intercept, catenary_residual
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn solve_catenary(v: f64, h: f64, length: f64) -> f64 {
    // WARNING: signature changed — verify body
    // Previous params: (v : f64, h : f64, length : f64)
    // Previous return: f64
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (geom : * mut mjvGeom, r#type : i32, width : f64, from : * const f64, to : * const f64)
    // Previous return: ()
    todo ! ()
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
    extern "C" { fn mjv_initGeom_impl(geom: *mut mjvGeom, r#type: i32, size: *const f64, pos: *const f64, mat: *const f64, rgba: *const f32); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mjv_initGeom_impl(geom, r#type, size, pos, mat, rgba) }
}

/// C: mjv_updateScene (engine/engine_vis_visualize.h:37)
/// Calls: mjp_getPluginAtSlotUnsafe, mjp_pluginCount, mju_message, mjv_addGeoms, mjv_makeLights, mjv_updateActiveFlex, mjv_updateActiveSkin, mjv_updateCamera
#[allow(unused_variables, non_snake_case)]
pub fn mjv_update_scene(m: *const mjModel, d: *mut mjData, opt: *const mjvOption, pert: *const mjvPerturb, cam: *mut mjvCamera, catmask: i32, scn: *mut mjvScene) {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, opt : * const mjvOption, pert : * const mjvPerturb, cam : * mut mjvCamera, catmask : i32, scn : * mut mjvScene)
    // Previous return: ()
    todo ! ()
}

/// C: mjv_addGeoms (engine/engine_vis_visualize.h:41)
/// Calls: addActuatorGeoms, addAutoConnectGeoms, addBodyBvhGeoms, addBodyLabelGeoms, addCameraGeoms, addCenterOfMassGeoms, addConstraintGeoms, addContactGeoms, addExternalPerturbGeoms, addFlexBvhGeoms, addFlexGeoms, addGeomFrameGeoms, addGeomGeoms, addInertiaGeoms, addIslandLabelGeoms, addJointGeoms, addLightGeoms, addMeshBvhGeoms, addMeshOctreeGeoms, addPerturbGeoms, addRangefinderGeoms, addSelectionPointGeoms, addSiteFrameGeoms, addSiteGeoms, addSkinGeoms, addSliderCrankGeoms, addSpatialTendonGeoms, addTactileSensorGeoms, addWorldBodyFrameGeoms, mjv_defaultPerturb
#[allow(unused_variables, non_snake_case)]
pub fn mjv_add_geoms(m: *const mjModel, d: *mut mjData, opt: *const mjvOption, pert: *const mjvPerturb, catmask: i32, scn: *mut mjvScene) {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, opt : * const mjvOption, pert : * const mjvPerturb, catmask : i32, scn : * mut mjvScene)
    // Previous return: ()
    todo ! ()
}

/// C: mjv_makeLights (engine/engine_vis_visualize.h:45)
/// Calls: f2f, mju_n2f, mjv_cameraInModel
#[allow(unused_variables, non_snake_case)]
pub fn mjv_make_lights(m: *const mjModel, d: *const mjData, scn: *mut mjvScene) {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * const mjData, scn : * mut mjvScene)
    // Previous return: ()
    todo ! ()
}

/// C: mjv_updateCamera (engine/engine_vis_visualize.h:48)
/// Calls: mju_copy3, mju_message, mjv_cameraFrame, mjv_cameraFrustum
#[allow(unused_variables, non_snake_case)]
pub fn mjv_update_camera(m: *const mjModel, d: *const mjData, cam: *mut mjvCamera, scn: *mut mjvScene) {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * const mjData, cam : * mut mjvCamera, scn : * mut mjvScene)
    // Previous return: ()
    todo ! ()
}

/// C: mjv_updateActiveFlex (engine/engine_vis_visualize.h:51)
/// Calls: addNormal, copyTex, makeFace, makeSide, makeSmooth, mj_freeStack, mj_markStack, mj_stackAllocInfo, mju_error, mju_normalize3, mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mjv_update_active_flex(m: *const mjModel, d: *mut mjData, scn: *mut mjvScene, opt: *const mjvOption) {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, scn : * mut mjvScene, opt : * const mjvOption)
    // Previous return: ()
    todo ! ()
}

/// C: mjv_updateSkin (engine/engine_vis_visualize.h:54)
/// Calls: mju_warning, mjv_defaultOption, mjv_updateActiveSkin
#[allow(unused_variables, non_snake_case)]
pub fn mjv_update_skin(m: *const mjModel, d: *const mjData, scn: *mut mjvScene) {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * const mjData, scn : * mut mjvScene)
    // Previous return: ()
    todo ! ()
}

/// C: mjv_updateActiveSkin (engine/engine_vis_visualize.h:57)
/// Calls: mju_addTo3, mju_cross, mju_mulMatVec3, mju_mulQuat, mju_negQuat, mju_quat2Mat, mju_sub3
#[allow(unused_variables, non_snake_case)]
pub fn mjv_update_active_skin(m: *const mjModel, d: *const mjData, scn: *mut mjvScene, opt: *const mjvOption) {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * const mjData, scn : * mut mjvScene, opt : * const mjvOption)
    // Previous return: ()
    todo ! ()
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
    extern "C" { fn mjv_cameraFrame_impl(headpos: *mut f64, forward: *mut f64, up: *mut f64, right: *mut f64, d: *const mjData, cam: *const mjvCamera); }
    // SAFETY: delegates to C implementation
    unsafe { mjv_cameraFrame_impl(headpos, forward, up, right, d, cam) }
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
    // WARNING: signature changed — verify body
    // Previous params: (zver : [f32 ; 2], zhor : [f32 ; 2], zclip : [f32 ; 2], m : * const mjModel, cam : * const mjvCamera)
    // Previous return: ()
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * const mjData, i : i32, length : * mut f64)
    // Previous return: i32
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (x0 : * const f64, x1 : * const f64, gravity : * const f64, length : f64, catenary : * mut f64, ncatenary : i32)
    // Previous return: i32
    todo ! ()
}

/// C: hsv2rgb (engine/engine_vis_visualize.h:76)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn hsv2rgb(RGB: *mut f32, H: f32, S: f32, V: f32) {
    extern "C" { fn hsv2rgb_impl(RGB: *mut f32, H: f32, S: f32, V: f32); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { hsv2rgb_impl(RGB, H, S, V) }
}

