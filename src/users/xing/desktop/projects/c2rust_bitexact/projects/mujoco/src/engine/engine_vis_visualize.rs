//! Port of: Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c
//! IR hash: 1fa44043fe383ce9
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: f2f (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:49)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn f2f(dest: *mut f32, src: *const f32, n: i32) {
    todo!() // cxx-internal:engine_vis_visualize.c.o:_f2f
}

/// C: makeLabel (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:55)
/// Calls: cxx:_mj_id2name, cxx:_mju_type2Str
#[allow(unused_variables, non_snake_case)]
pub fn make_label(m: *const mjModel, r#type: u32, id: i32, label: *mut i8) {
    todo!() // cxx-internal:engine_vis_visualize.c.o:_makeLabel
}

/// C: islandColor (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:110)
/// Calls: cxx:_hsv2rgb, cxx:_mju_Halton
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn island_color(rgba: *mut f32, h: i32, awake: i32) {
    todo!() // cxx-internal:engine_vis_visualize.c.o:_islandColor
}

/// C: mixcolor (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:140)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mixcolor(rgba: *mut f32, r#ref: *const f32, flg1: i32, flg2: i32) {
    todo!() // cxx-internal:engine_vis_visualize.c.o:_mixcolor
}

/// C: bodycategory (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:157)
#[allow(unused_variables, non_snake_case)]
pub fn bodycategory(m: *const mjModel, bodyid: i32) -> i32 {
    todo!() // cxx-internal:engine_vis_visualize.c.o:_bodycategory
}

/// C: acquireGeom (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:169)
/// Calls: cxx:_mju_warning, cxx:_mjv_initGeom
#[allow(unused_variables, non_snake_case)]
pub fn acquire_geom(scn: *mut mjvScene, objid: i32, category: i32, objtype: i32) -> *mut mjvGeom {
    todo!() // cxx:_acquireGeom
}

/// C: releaseGeom (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:192)
/// Calls: cxx:_mju_error
#[allow(unused_variables, non_snake_case)]
pub fn release_geom(geom: *mut *mut mjvGeom, scn: *mut mjvScene) {
    todo!() // cxx:_releaseGeom
}

/// C: addTriangle (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:204)
/// Calls: cxx:_acquireGeom, cxx:_mju_cross, cxx:_mju_normalize3, cxx:_mjv_initGeom, cxx:_releaseGeom
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn add_triangle(scn: *mut mjvScene, v0: *const f64, v1: *const f64, v2: *const f64, rgba: *const f32, objid: i32, category: i32, objtype: i32) {
    todo!() // cxx-internal:engine_vis_visualize.c.o:_addTriangle
}

/// C: setMaterial (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:225)
/// Calls: cxx-internal:engine_vis_visualize.c.o:_f2f
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn set_material(m: *const mjModel, geom: *mut mjvGeom, matid: i32, rgba: *const f32, flags: *const u8) {
    todo!() // cxx-internal:engine_vis_visualize.c.o:_setMaterial
}

/// C: addConnector (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:296)
/// Calls: cxx-internal:engine_vis_visualize.c.o:_f2f, cxx:_acquireGeom, cxx:_mjv_connector, cxx:_releaseGeom
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn add_connector(scn: *mut mjvScene, r#type: i32, width: f64, from: *const f64, to: *const f64, rgba: *const f32, objid: i32, category: i32, objtype: i32) {
    todo!() // cxx-internal:engine_vis_visualize.c.o:_addConnector
}

/// C: markselected (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:393)
#[allow(unused_variables, non_snake_case)]
pub fn markselected(vis: *const mjVisual, geom: *mut mjvGeom) {
    todo!() // cxx-internal:engine_vis_visualize.c.o:_markselected
}

/// C: addFrame (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:400)
/// Calls: cxx:_acquireGeom, cxx:_mju_add3, cxx:_mju_mulMatVec3, cxx:_mjv_connector, cxx:_releaseGeom
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn add_frame(scn: *mut mjvScene, objid: i32, pos: *const f64, rot: *const f64, length: f32, width: f32) {
    todo!() // cxx:_addFrame
}

/// C: getFrustum (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:434)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn get_frustum(zver: *mut f32, zhor: *mut f32, znear: f32, intrinsic: *const f32, sensorsize: *const f32) {
    todo!() // cxx-internal:engine_vis_visualize.c.o:_getFrustum
}

/// C: addContactGeoms (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:565)
/// Calls: cxx-internal:engine_vis_visualize.c.o:_f2f, cxx-internal:engine_vis_visualize.c.o:_islandColor, cxx:_acquireGeom, cxx:_addFrame, cxx:_mj_contactForce, cxx:_mj_id2name, cxx:_mju_add3, cxx:_mju_copy, cxx:_mju_mulMatVec, cxx:_mju_n2f, cxx:_mju_norm3, cxx:_mju_scl3, cxx:_mju_transpose, cxx:_mju_zero3, cxx:_mjv_connector, cxx:_releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_contact_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene, catmask: i32) {
    todo!() // cxx-internal:engine_vis_visualize.c.o:_addContactGeoms
}

/// C: addFlexGeoms (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:748)
/// Calls: cxx-internal:engine_vis_visualize.c.o:_islandColor, cxx-internal:engine_vis_visualize.c.o:_makeLabel, cxx-internal:engine_vis_visualize.c.o:_markselected, cxx-internal:engine_vis_visualize.c.o:_setMaterial, cxx:_acquireGeom, cxx:_mj_sleepCycle, cxx:_mjv_initGeom, cxx:_releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_flex_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, pert: *const mjvPerturb, catmask: i32, scn: *mut mjvScene) {
    todo!() // cxx-internal:engine_vis_visualize.c.o:_addFlexGeoms
}

/// C: addSkinGeoms (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:841)
/// Calls: cxx-internal:engine_vis_visualize.c.o:_makeLabel, cxx-internal:engine_vis_visualize.c.o:_markselected, cxx-internal:engine_vis_visualize.c.o:_setMaterial, cxx:_acquireGeom, cxx:_mjv_initGeom, cxx:_releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_skin_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, pert: *const mjvPerturb, catmask: i32, scn: *mut mjvScene) {
    todo!() // cxx-internal:engine_vis_visualize.c.o:_addSkinGeoms
}

/// C: addGeomGeoms (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:892)
/// Calls: cxx-internal:engine_vis_visualize.c.o:_bodycategory, cxx-internal:engine_vis_visualize.c.o:_islandColor, cxx-internal:engine_vis_visualize.c.o:_makeLabel, cxx-internal:engine_vis_visualize.c.o:_markselected, cxx-internal:engine_vis_visualize.c.o:_setMaterial, cxx:_acquireGeom, cxx:_mj_sleepCycle, cxx:_mju_addToScl3, cxx:_mju_copy3, cxx:_mju_dot3, cxx:_mju_n2f, cxx:_mju_round, cxx:_mju_transpose, cxx:_mjv_initGeom, cxx:_releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_geom_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, pert: *const mjvPerturb, catmask: i32, scn: *mut mjvScene) {
    todo!() // cxx-internal:engine_vis_visualize.c.o:_addGeomGeoms
}

/// C: addSiteGeoms (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:1041)
/// Calls: cxx-internal:engine_vis_visualize.c.o:_bodycategory, cxx-internal:engine_vis_visualize.c.o:_makeLabel, cxx-internal:engine_vis_visualize.c.o:_markselected, cxx-internal:engine_vis_visualize.c.o:_setMaterial, cxx:_acquireGeom, cxx:_mjv_initGeom, cxx:_releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_site_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, pert: *const mjvPerturb, catmask: i32, scn: *mut mjvScene) {
    todo!() // cxx-internal:engine_vis_visualize.c.o:_addSiteGeoms
}

/// C: addSpatialTendonGeoms (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:1141)
/// Calls: cxx-internal:engine_vis_visualize.c.o:_f2f, cxx-internal:engine_vis_visualize.c.o:_islandColor, cxx-internal:engine_vis_visualize.c.o:_makeLabel, cxx-internal:engine_vis_visualize.c.o:_setMaterial, cxx:_acquireGeom, cxx:_mju_copy3, cxx:_mjv_catenary, cxx:_mjv_connector, cxx:_mjv_isCatenary, cxx:_releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_spatial_tendon_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, catmask: i32, scn: *mut mjvScene) {
    todo!() // cxx-internal:engine_vis_visualize.c.o:_addSpatialTendonGeoms
}

/// C: addSliderCrankGeoms (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:1266)
/// Calls: cxx-internal:engine_vis_visualize.c.o:_f2f, cxx-internal:engine_vis_visualize.c.o:_makeLabel, cxx:_acquireGeom, cxx:_mju_addTo3, cxx:_mju_dot3, cxx:_mju_scl3, cxx:_mju_sub, cxx:_mjv_connector, cxx:_releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_slider_crank_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, catmask: i32, scn: *mut mjvScene) {
    todo!() // cxx-internal:engine_vis_visualize.c.o:_addSliderCrankGeoms
}

/// C: addGeomFrameGeoms (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:1334)
/// Calls: cxx-internal:engine_vis_visualize.c.o:_bodycategory, cxx:_addFrame
#[allow(unused_variables, non_snake_case)]
pub fn add_geom_frame_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, catmask: i32, scn: *mut mjvScene) {
    todo!() // cxx-internal:engine_vis_visualize.c.o:_addGeomFrameGeoms
}

/// C: addSiteFrameGeoms (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:1364)
/// Calls: cxx-internal:engine_vis_visualize.c.o:_bodycategory, cxx:_addFrame
#[allow(unused_variables, non_snake_case)]
pub fn add_site_frame_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, catmask: i32, scn: *mut mjvScene) {
    todo!() // cxx-internal:engine_vis_visualize.c.o:_addSiteFrameGeoms
}

/// C: addBodyBvhGeoms (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:1394)
/// Calls: cxx:_acquireGeom, cxx:_mju_addTo3, cxx:_mju_mulMatVec3, cxx:_mjv_initGeom, cxx:_releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_body_bvh_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene) {
    todo!() // cxx-internal:engine_vis_visualize.c.o:_addBodyBvhGeoms
}

/// C: addFlexBvhGeoms (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:1449)
/// Calls: cxx:_acquireGeom, cxx:_mj_stackAllocInfo, cxx:_mju_addTo3, cxx:_mju_copy3, cxx:_mju_mulMatVec3, cxx:_mjv_connector, cxx:_mjv_initGeom, cxx:_releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_flex_bvh_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene) {
    todo!() // cxx-internal:engine_vis_visualize.c.o:_addFlexBvhGeoms
}

/// C: addMeshBvhGeoms (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:1581)
/// Calls: cxx:_acquireGeom, cxx:_mju_addTo3, cxx:_mju_mulMatVec3, cxx:_mjv_initGeom, cxx:_releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_mesh_bvh_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene) {
    todo!() // cxx-internal:engine_vis_visualize.c.o:_addMeshBvhGeoms
}

/// C: addMeshOctreeGeoms (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:1634)
/// Calls: cxx:_acquireGeom, cxx:_mju_addTo3, cxx:_mju_mulMatVec3, cxx:_mjv_initGeom, cxx:_releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_mesh_octree_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene) {
    todo!() // cxx-internal:engine_vis_visualize.c.o:_addMeshOctreeGeoms
}

/// C: addTactileSensorGeoms (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:1673)
/// Calls: cxx-internal:engine_vis_visualize.c.o:_addTriangle, cxx:_mju_addTo3, cxx:_mju_mat2Quat, cxx:_mju_max, cxx:_mju_mulMatVec3
#[allow(unused_variables, non_snake_case)]
pub fn add_tactile_sensor_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene) {
    todo!() // cxx-internal:engine_vis_visualize.c.o:_addTactileSensorGeoms
}

/// C: addInertiaGeoms (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:1745)
/// Calls: cxx-internal:engine_vis_visualize.c.o:_bodycategory, cxx-internal:engine_vis_visualize.c.o:_makeLabel, cxx-internal:engine_vis_visualize.c.o:_markselected, cxx:_acquireGeom, cxx:_mju_max, cxx:_mjv_initGeom, cxx:_releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_inertia_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, pert: *const mjvPerturb, catmask: i32, scn: *mut mjvScene) {
    todo!() // cxx-internal:engine_vis_visualize.c.o:_addInertiaGeoms
}

/// C: addPerturbGeoms (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:1811)
/// Calls: cxx-internal:engine_vis_visualize.c.o:_f2f, cxx-internal:engine_vis_visualize.c.o:_mixcolor, cxx:_acquireGeom, cxx:_mju_addTo3, cxx:_mju_copy3, cxx:_mju_mulMatVec3, cxx:_mju_quat2Mat, cxx:_mjv_connector, cxx:_mjv_initGeom, cxx:_releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_perturb_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, pert: *const mjvPerturb, scn: *mut mjvScene) {
    todo!() // cxx-internal:engine_vis_visualize.c.o:_addPerturbGeoms
}

/// C: addWorldBodyFrameGeoms (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:1900)
/// Calls: cxx-internal:engine_vis_visualize.c.o:_bodycategory, cxx:_addFrame
#[allow(unused_variables, non_snake_case)]
pub fn add_world_body_frame_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, catmask: i32, scn: *mut mjvScene) {
    todo!() // cxx-internal:engine_vis_visualize.c.o:_addWorldBodyFrameGeoms
}

/// C: addSelectionPointGeoms (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:1928)
/// Calls: cxx-internal:engine_vis_visualize.c.o:_f2f, cxx:_acquireGeom, cxx:_mju_addTo3, cxx:_mju_mulMatVec3, cxx:_mju_n2f, cxx:_releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_selection_point_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, pert: *const mjvPerturb, scn: *mut mjvScene) {
    todo!() // cxx-internal:engine_vis_visualize.c.o:_addSelectionPointGeoms
}

/// C: addBodyLabelGeoms (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:1964)
/// Calls: cxx-internal:engine_vis_visualize.c.o:_bodycategory, cxx-internal:engine_vis_visualize.c.o:_makeLabel, cxx:_acquireGeom, cxx:_mju_n2f, cxx:_releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_body_label_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, pert: *const mjvPerturb, catmask: i32, scn: *mut mjvScene) {
    todo!() // cxx-internal:engine_vis_visualize.c.o:_addBodyLabelGeoms
}

/// C: addJointGeoms (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:1994)
/// Calls: cxx-internal:engine_vis_visualize.c.o:_f2f, cxx-internal:engine_vis_visualize.c.o:_makeLabel, cxx:_acquireGeom, cxx:_mju_addScl3, cxx:_mju_message, cxx:_mju_n2f, cxx:_mjv_connector, cxx:_releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_joint_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene) {
    todo!() // cxx-internal:engine_vis_visualize.c.o:_addJointGeoms
}

/// C: addActuatorGeoms (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:2074)
/// Calls: cxx-internal:engine_vis_visualize.c.o:_f2f, cxx-internal:engine_vis_visualize.c.o:_makeLabel, cxx-internal:engine_vis_visualize.c.o:_setMaterial, cxx:_acquireGeom, cxx:_mj_actuatorDisabled, cxx:_mju_addScl3, cxx:_mju_clip, cxx:_mju_scl3, cxx:_mjv_connector, cxx:_mjv_initGeom, cxx:_releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_actuator_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene) {
    todo!() // cxx-internal:engine_vis_visualize.c.o:_addActuatorGeoms
}

/// C: addIslandLabelGeoms (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:2283)
/// Calls: cxx:_acquireGeom, cxx:_mju_n2f, cxx:_releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_island_label_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene) {
    todo!() // cxx-internal:engine_vis_visualize.c.o:_addIslandLabelGeoms
}

/// C: addCameraGeoms (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:2313)
/// Calls: cxx-internal:engine_vis_visualize.c.o:_addConnector, cxx-internal:engine_vis_visualize.c.o:_addTriangle, cxx-internal:engine_vis_visualize.c.o:_f2f, cxx-internal:engine_vis_visualize.c.o:_getFrustum, cxx-internal:engine_vis_visualize.c.o:_makeLabel, cxx:_acquireGeom, cxx:_addFrame, cxx:_mju_addScl3, cxx:_mju_addToScl3, cxx:_mju_n2f, cxx:_releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_camera_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene) {
    todo!() // cxx-internal:engine_vis_visualize.c.o:_addCameraGeoms
}

/// C: addLightGeoms (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:2460)
/// Calls: cxx-internal:engine_vis_visualize.c.o:_f2f, cxx-internal:engine_vis_visualize.c.o:_makeLabel, cxx:_acquireGeom, cxx:_addFrame, cxx:_mju_addScl3, cxx:_mju_n2f, cxx:_mju_quat2Mat, cxx:_mju_quatZ2Vec, cxx:_releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_light_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene) {
    todo!() // cxx-internal:engine_vis_visualize.c.o:_addLightGeoms
}

/// C: addCenterOfMassGeoms (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:2509)
/// Calls: cxx-internal:engine_vis_visualize.c.o:_f2f, cxx:_acquireGeom, cxx:_mju_n2f, cxx:_releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_center_of_mass_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene) {
    todo!() // cxx-internal:engine_vis_visualize.c.o:_addCenterOfMassGeoms
}

/// C: addAutoConnectGeoms (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:2535)
/// Calls: cxx-internal:engine_vis_visualize.c.o:_addConnector
#[allow(unused_variables, non_snake_case)]
pub fn add_auto_connect_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene) {
    todo!() // cxx-internal:engine_vis_visualize.c.o:_addAutoConnectGeoms
}

/// C: addRangefinderGeoms (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:2570)
/// Calls: cxx-internal:engine_vis_visualize.c.o:_addConnector, cxx-internal:engine_vis_visualize.c.o:_f2f, cxx:_acquireGeom, cxx:_mju_addScl3, cxx:_mju_camIntrinsics, cxx:_mju_camPixelRay, cxx:_mju_copy3, cxx:_mju_isZero, cxx:_mju_n2f, cxx:_mju_raydataSize, cxx:_releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_rangefinder_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene) {
    todo!() // cxx-internal:engine_vis_visualize.c.o:_addRangefinderGeoms
}

/// C: addExternalPerturbGeoms (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:2729)
/// Calls: cxx-internal:engine_vis_visualize.c.o:_addConnector, cxx:_mju_add3, cxx:_mju_isZero, cxx:_mju_norm3, cxx:_mju_scl3
#[allow(unused_variables, non_snake_case)]
pub fn add_external_perturb_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene) {
    todo!() // cxx-internal:engine_vis_visualize.c.o:_addExternalPerturbGeoms
}

/// C: addConstraintGeoms (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:2760)
/// Calls: cxx-internal:engine_vis_visualize.c.o:_makeLabel, cxx:_acquireGeom, cxx:_mju_addTo3, cxx:_mju_copy3, cxx:_mju_mulMatVec3, cxx:_mjv_initGeom, cxx:_releaseGeom
#[allow(unused_variables, non_snake_case)]
pub fn add_constraint_geoms(m: *const mjModel, d: *mut mjData, vopt: *const mjvOption, scn: *mut mjvScene) {
    todo!() // cxx-internal:engine_vis_visualize.c.o:_addConstraintGeoms
}

/// C: makeFace (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:3024)
/// Calls: cxx:_mju_addScl3, cxx:_mju_cross, cxx:_mju_n2f, cxx:_mju_normalize3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn make_face(_face: *mut f32, _normal: *mut f32, radius: f64, vertxpos: *const f64, nface: i32, i0: i32, i1: i32, i2: i32) {
    todo!() // cxx-internal:engine_vis_visualize.c.o:_makeFace
}

/// C: addNormal (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:3056)
/// Calls: cxx:_mju_addTo3, cxx:_mju_cross, cxx:_mju_normalize3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn add_normal(vertnorm: *mut f64, vertxpos: *const f64, i0: i32, i1: i32, i2: i32) {
    todo!() // cxx-internal:engine_vis_visualize.c.o:_addNormal
}

/// C: makeSmooth (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:3076)
/// Calls: cxx:_mju_cross, cxx:_mju_normalize3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn make_smooth(_face: *mut f32, _normal: *mut f32, radius: f64, flg_flat: u8, vertnorm: *const f64, vertxpos: *const f64, nface: i32, i0: i32, i1: i32, i2: i32) {
    todo!() // cxx-internal:engine_vis_visualize.c.o:_makeSmooth
}

/// C: makeSide (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:3123)
/// Calls: cxx:_mju_cross, cxx:_mju_normalize3, cxx:_mju_scl3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn make_side(_face: *mut f32, _normal: *mut f32, radius: f64, vertnorm: *const f64, vertxpos: *const f64, nface: i32, i0: i32, i1: i32) {
    todo!() // cxx-internal:engine_vis_visualize.c.o:_makeSide
}

/// C: copyTex (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:3159)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn copy_tex(dst: *mut f32, src: *const f32, nface: i32, i0: i32, i1: i32, i2: i32) {
    todo!() // cxx-internal:engine_vis_visualize.c.o:_copyTex
}

/// C: cosh_sinh (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:3516)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn cosh_sinh(x: f64, sinh: *mut f64) -> f64 {
    todo!() // cxx-internal:engine_vis_visualize.c.o:_cosh_sinh
}

/// C: catenary_intercept (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:3526)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn catenary_intercept(v: f64, h: f64, length: f64) -> f64 {
    todo!() // cxx-internal:engine_vis_visualize.c.o:_catenary_intercept
}

/// C: catenary_residual (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:3532)
/// Calls: cxx-internal:engine_vis_visualize.c.o:_cosh_sinh
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn catenary_residual(b: f64, intercept: f64, grad: *mut f64) -> f64 {
    todo!() // cxx-internal:engine_vis_visualize.c.o:_catenary_residual
}

/// C: solve_catenary (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.c:3549)
/// Calls: cxx-internal:engine_vis_visualize.c.o:_catenary_intercept, cxx-internal:engine_vis_visualize.c.o:_catenary_residual
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn solve_catenary(v: f64, h: f64, length: f64) -> f64 {
    todo!() // cxx-internal:engine_vis_visualize.c.o:_solve_catenary
}

/// C: mjv_connector (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.h:29)
/// Calls: cxx:_mju_message, cxx:_mju_n2f, cxx:_mju_norm3, cxx:_mju_quat2Mat, cxx:_mju_quatZ2Vec
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjv_connector(geom: *mut mjvGeom, r#type: i32, width: f64, from: *const f64, to: *const f64) {
    todo!() // cxx:_mjv_connector
}

/// C: mjv_initGeom (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.h:33)
/// Calls: cxx-internal:engine_vis_visualize.c.o:_f2f, cxx:_mju_n2f
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjv_init_geom(geom: *mut mjvGeom, r#type: i32, size: *const f64, pos: *const f64, mat: *const f64, rgba: *const f32) {
    todo!() // cxx:_mjv_initGeom
}

/// C: mjv_updateScene (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.h:37)
/// Calls: cxx:_mjp_getPluginAtSlotUnsafe, cxx:_mjp_pluginCount, cxx:_mju_message, cxx:_mjv_addGeoms, cxx:_mjv_makeLights, cxx:_mjv_updateActiveFlex, cxx:_mjv_updateActiveSkin, cxx:_mjv_updateCamera
#[allow(unused_variables, non_snake_case)]
pub fn mjv_update_scene(m: *const mjModel, d: *mut mjData, opt: *const mjvOption, pert: *const mjvPerturb, cam: *mut mjvCamera, catmask: i32, scn: *mut mjvScene) {
    todo!() // cxx:_mjv_updateScene
}

/// C: mjv_addGeoms (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.h:41)
/// Calls: cxx-internal:engine_vis_visualize.c.o:_addActuatorGeoms, cxx-internal:engine_vis_visualize.c.o:_addAutoConnectGeoms, cxx-internal:engine_vis_visualize.c.o:_addBodyBvhGeoms, cxx-internal:engine_vis_visualize.c.o:_addBodyLabelGeoms, cxx-internal:engine_vis_visualize.c.o:_addCameraGeoms, cxx-internal:engine_vis_visualize.c.o:_addCenterOfMassGeoms, cxx-internal:engine_vis_visualize.c.o:_addConstraintGeoms, cxx-internal:engine_vis_visualize.c.o:_addContactGeoms, cxx-internal:engine_vis_visualize.c.o:_addExternalPerturbGeoms, cxx-internal:engine_vis_visualize.c.o:_addFlexBvhGeoms, cxx-internal:engine_vis_visualize.c.o:_addFlexGeoms, cxx-internal:engine_vis_visualize.c.o:_addGeomFrameGeoms, cxx-internal:engine_vis_visualize.c.o:_addGeomGeoms, cxx-internal:engine_vis_visualize.c.o:_addInertiaGeoms, cxx-internal:engine_vis_visualize.c.o:_addIslandLabelGeoms, cxx-internal:engine_vis_visualize.c.o:_addJointGeoms, cxx-internal:engine_vis_visualize.c.o:_addLightGeoms, cxx-internal:engine_vis_visualize.c.o:_addMeshBvhGeoms, cxx-internal:engine_vis_visualize.c.o:_addMeshOctreeGeoms, cxx-internal:engine_vis_visualize.c.o:_addPerturbGeoms, cxx-internal:engine_vis_visualize.c.o:_addRangefinderGeoms, cxx-internal:engine_vis_visualize.c.o:_addSelectionPointGeoms, cxx-internal:engine_vis_visualize.c.o:_addSiteFrameGeoms, cxx-internal:engine_vis_visualize.c.o:_addSiteGeoms, cxx-internal:engine_vis_visualize.c.o:_addSkinGeoms, cxx-internal:engine_vis_visualize.c.o:_addSliderCrankGeoms, cxx-internal:engine_vis_visualize.c.o:_addSpatialTendonGeoms, cxx-internal:engine_vis_visualize.c.o:_addTactileSensorGeoms, cxx-internal:engine_vis_visualize.c.o:_addWorldBodyFrameGeoms, cxx:_mjv_defaultPerturb
#[allow(unused_variables, non_snake_case)]
pub fn mjv_add_geoms(m: *const mjModel, d: *mut mjData, opt: *const mjvOption, pert: *const mjvPerturb, catmask: i32, scn: *mut mjvScene) {
    todo!() // cxx:_mjv_addGeoms
}

/// C: mjv_makeLights (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.h:45)
/// Calls: cxx-internal:engine_vis_visualize.c.o:_f2f, cxx:_mju_n2f, cxx:_mjv_cameraInModel
#[allow(unused_variables, non_snake_case)]
pub fn mjv_make_lights(m: *const mjModel, d: *const mjData, scn: *mut mjvScene) {
    todo!() // cxx:_mjv_makeLights
}

/// C: mjv_updateCamera (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.h:48)
/// Calls: cxx:_mju_copy3, cxx:_mju_message, cxx:_mjv_cameraFrame, cxx:_mjv_cameraFrustum
#[allow(unused_variables, non_snake_case)]
pub fn mjv_update_camera(m: *const mjModel, d: *const mjData, cam: *mut mjvCamera, scn: *mut mjvScene) {
    todo!() // cxx:_mjv_updateCamera
}

/// C: mjv_updateActiveFlex (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.h:51)
/// Calls: cxx-internal:engine_vis_visualize.c.o:_addNormal, cxx-internal:engine_vis_visualize.c.o:_copyTex, cxx-internal:engine_vis_visualize.c.o:_makeFace, cxx-internal:engine_vis_visualize.c.o:_makeSide, cxx-internal:engine_vis_visualize.c.o:_makeSmooth, cxx:_mj_freeStack, cxx:_mj_markStack, cxx:_mj_stackAllocInfo, cxx:_mju_error, cxx:_mju_normalize3, cxx:_mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mjv_update_active_flex(m: *const mjModel, d: *mut mjData, scn: *mut mjvScene, opt: *const mjvOption) {
    todo!() // cxx:_mjv_updateActiveFlex
}

/// C: mjv_updateSkin (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.h:54)
/// Calls: cxx:_mju_warning, cxx:_mjv_defaultOption, cxx:_mjv_updateActiveSkin
#[allow(unused_variables, non_snake_case)]
pub fn mjv_update_skin(m: *const mjModel, d: *const mjData, scn: *mut mjvScene) {
    todo!() // cxx:_mjv_updateSkin
}

/// C: mjv_updateActiveSkin (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.h:57)
/// Calls: cxx:_mju_addTo3, cxx:_mju_cross, cxx:_mju_mulMatVec3, cxx:_mju_mulQuat, cxx:_mju_negQuat, cxx:_mju_quat2Mat, cxx:_mju_sub3
#[allow(unused_variables, non_snake_case)]
pub fn mjv_update_active_skin(m: *const mjModel, d: *const mjData, scn: *mut mjvScene, opt: *const mjvOption) {
    todo!() // cxx:_mjv_updateActiveSkin
}

/// C: mjv_cameraFrame (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.h:61)
/// Calls: cxx:_mju_addScl3, cxx:_mju_copy3, cxx:_mju_message
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjv_camera_frame(headpos: *mut f64, forward: *mut f64, up: *mut f64, right: *mut f64, d: *const mjData, cam: *const mjvCamera) {
    todo!() // cxx:_mjv_cameraFrame
}

/// C: mjv_cameraFrustum (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.h:65)
/// Calls: cxx-internal:engine_vis_visualize.c.o:_getFrustum, cxx:_mju_message
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjv_camera_frustum(zver: *mut f32, zhor: *mut f32, zclip: *mut f32, m: *const mjModel, cam: *const mjvCamera) {
    todo!() // cxx:_mjv_cameraFrustum
}

/// C: mjv_isCatenary (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.h:69)
/// Calls: cxx:_mju_isZero, cxx:_mju_norm3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjv_is_catenary(m: *const mjModel, d: *const mjData, i: i32, length: *mut f64) -> i32 {
    todo!() // cxx:_mjv_isCatenary
}

/// C: mjv_catenary (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.h:72)
/// Calls: cxx-internal:engine_vis_visualize.c.o:_cosh_sinh, cxx-internal:engine_vis_visualize.c.o:_solve_catenary, cxx:_mju_addScl3, cxx:_mju_addToScl3, cxx:_mju_copy3, cxx:_mju_dist3, cxx:_mju_dot3, cxx:_mju_normalize3, cxx:_mju_scl3, cxx:_mju_sub3, cxx:_mju_subFrom3, cxx:_mju_zero3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjv_catenary(x0: *const f64, x1: *const f64, gravity: *const f64, length: f64, catenary: *mut f64, ncatenary: i32) -> i32 {
    todo!() // cxx:_mjv_catenary
}

/// C: hsv2rgb (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_visualize.h:76)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn hsv2rgb(RGB: *mut f32, H: f32, S: f32, V: f32) {
    todo!() // cxx:_hsv2rgb
}

