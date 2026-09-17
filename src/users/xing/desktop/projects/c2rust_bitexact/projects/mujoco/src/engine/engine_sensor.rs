//! Port of: Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_sensor.c
//! IR hash: 1fa44043fe383ce9
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: ContactInfoCompare (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_sensor.c:52)
#[allow(unused_variables, non_snake_case)]
pub fn contact_info_compare(a: *const ContactInfo, b: *const ContactInfo, context: *mut ()) -> i32 {
    todo!() // cxx-internal:engine_sensor.c.o:_ContactInfoCompare
}

/// C: ContactSelect (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_sensor.c:61)
/// Calls: cxx-internal:engine_sensor.c.o:_ContactInfoCompare
#[allow(unused_variables, non_snake_case)]
pub fn contact_select(arr: *mut ContactInfo, buf: *mut ContactInfo, n: i32, k: i32, context: *mut ()) {
    todo!() // cxx-internal:engine_sensor.c.o:_ContactSelect
}

/// C: tactile_taxel_batch (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_sensor.c:80)
/// Calls: cxx:_mjc_distance, cxx:_mjc_getSDF, cxx:_mju_addTo3, cxx:_mju_dot3, cxx:_mju_max, cxx:_mju_min, cxx:_mju_mulMatTVec3, cxx:_mju_mulMatVec3, cxx:_mju_quat2Mat, cxx:_mju_rotVecQuat, cxx:_mju_sub3, cxx:_mju_transformSpatial
#[allow(unused_variables, non_snake_case)]
pub fn tactile_taxel_batch(m: *const mjModel, d: *mut mjData, args: *mut ()) -> *mut () {
    todo!() // cxx-internal:engine_sensor.c.o:_tactile_taxel_batch
}

/// C: tactileTask (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_sensor.c:191)
/// Calls: cxx-internal:engine_sensor.c.o:_tactile_taxel_batch
#[allow(unused_variables, non_snake_case)]
pub fn tactile_task(m: *const mjModel, d: *mut mjData, arg: *mut (), thread_id: i32, task_id: i32) {
    todo!() // cxx-internal:engine_sensor.c.o:_tactileTask
}

/// C: apply_cutoff (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_sensor.c:198)
/// Calls: cxx:_mju_clip, cxx:_mju_min
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn apply_cutoff(m: *const mjModel, i: i32, data: *mut f64) {
    todo!() // cxx-internal:engine_sensor.c.o:_apply_cutoff
}

/// C: get_xpos_xmat (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_sensor.c:227)
/// Calls: cxx:_mju_message
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn get_xpos_xmat(d: *const mjData, r#type: u32, id: i32, sensor_id: i32, xpos: *mut *mut f64, xmat: *mut *mut f64) {
    todo!() // cxx-internal:engine_sensor.c.o:_get_xpos_xmat
}

/// C: get_xquat (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_sensor.c:257)
/// Calls: cxx:_mju_copy4, cxx:_mju_message, cxx:_mju_mulQuat
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn get_xquat(m: *const mjModel, d: *const mjData, r#type: u32, id: i32, sensor_id: i32, quat: *mut f64) {
    todo!() // cxx-internal:engine_sensor.c.o:_get_xquat
}

/// C: cam_project (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_sensor.c:281)
/// Calls: cxx:_mju_max, cxx:_mju_min, cxx:_mju_mulMatTVec, cxx:_mju_sub3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn cam_project(sensordata: *mut f64, target_xpos: *const f64, cam_xpos: *const f64, cam_xmat: *const f64, cam_res: *const i32, cam_fovy: f64, cam_intrinsic: *const f32, cam_sensorsize: *const f32) {
    todo!() // cxx-internal:engine_sensor.c.o:_cam_project
}

/// C: checkMatch (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_sensor.c:320)
/// Calls: cxx:__ZN7mjCMesh4treeEv
#[allow(unused_variables, non_snake_case)]
pub fn check_match(m: *const mjModel, body: i32, geom: i32, r#type: u32, id: i32) -> i32 {
    todo!() // cxx-internal:engine_sensor.c.o:_checkMatch
}

/// C: matchContact (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_sensor.c:339)
/// Calls: cxx-internal:engine_sensor.c.o:_checkMatch, cxx:_mj_flexBody, cxx:_mju_insideGeom
#[allow(unused_variables, non_snake_case)]
pub fn match_contact(m: *const mjModel, d: *const mjData, conid: i32, type1: u32, id1: i32, type2: u32, id2: i32) -> i32 {
    todo!() // cxx-internal:engine_sensor.c.o:_matchContact
}

/// C: copySensorData (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_sensor.c:398)
/// Calls: cxx:_mj_contactForce, cxx:_mju_copy3, cxx:_mju_scl3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn copy_sensor_data(m: *const mjModel, d: *const mjData, data: *mut *mut f64, id: i32, flg_flip: i32, nfound: i32) {
    todo!() // cxx-internal:engine_sensor.c.o:_copySensorData
}

/// C: total_wrench (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_sensor.c:442)
/// Calls: cxx:_mju_addTo3, cxx:_mju_cross, cxx:_mju_mulMatTVec3, cxx:_mju_sub3, cxx:_mju_zero3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn total_wrench(force: *mut f64, torque: *mut f64, point: *const f64, n: i32, wrench: *const f64, pos: *const f64, frame: *const f64) {
    todo!() // cxx-internal:engine_sensor.c.o:_total_wrench
}

/// C: fill_raydata (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_sensor.c:470)
/// Calls: cxx:_mju_addScl3, cxx:_mju_copy3, cxx:_mju_dot3, cxx:_mju_sub3, cxx:_mju_zero3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn fill_raydata(ptr: *mut f64, dataspec: i32, dist: f64, origin: *const f64, direction: *const f64, normal: *const f64, cam_xpos: *const f64, cam_z: *const f64) -> *mut f64 {
    todo!() // cxx-internal:engine_sensor.c.o:_fill_raydata
}

/// C: mj_computeSensorPos (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_sensor.c:525)
/// Calls: cxx-internal:engine_sensor.c.o:_cam_project, cxx-internal:engine_sensor.c.o:_fill_raydata, cxx-internal:engine_sensor.c.o:_get_xpos_xmat, cxx-internal:engine_sensor.c.o:_get_xquat, cxx:_mj_energyPos, cxx:_mj_energyVel, cxx:_mj_freeStack, cxx:_mj_geomDistance, cxx:_mj_markStack, cxx:_mj_multiRay, cxx:_mj_ray, cxx:_mj_stackAllocInfo, cxx:_mju_camIntrinsics, cxx:_mju_camPixelRay, cxx:_mju_copy, cxx:_mju_copy3, cxx:_mju_copy4, cxx:_mju_insideGeom, cxx:_mju_message, cxx:_mju_mulMatTVec, cxx:_mju_mulMatTVec3, cxx:_mju_mulQuat, cxx:_mju_negQuat, cxx:_mju_normalize3, cxx:_mju_normalize4, cxx:_mju_sub3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_compute_sensor_pos(m: *const mjModel, d: *mut mjData, i: i32, sensordata: *mut f64) {
    todo!() // cxx-internal:engine_sensor.c.o:_mj_computeSensorPos
}

/// C: mj_computeSensorVel (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_sensor.c:839)
/// Calls: cxx-internal:engine_sensor.c.o:_get_xpos_xmat, cxx:_mj_objectVelocity, cxx:_mj_subtreeVel, cxx:_mju_addTo3, cxx:_mju_copy3, cxx:_mju_cross, cxx:_mju_message, cxx:_mju_mulMatTVec3, cxx:_mju_sub, cxx:_mju_sub3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_compute_sensor_vel(m: *const mjModel, d: *mut mjData, i: i32, sensordata: *mut f64) {
    todo!() // cxx-internal:engine_sensor.c.o:_mj_computeSensorVel
}

/// C: mj_computeSensorAcc (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_sensor.c:958)
/// Calls: cxx-internal:engine_sensor.c.o:_ContactSelect, cxx-internal:engine_sensor.c.o:_copySensorData, cxx-internal:engine_sensor.c.o:_matchContact, cxx-internal:engine_sensor.c.o:_tactile_taxel_batch, cxx-internal:engine_sensor.c.o:_total_wrench, cxx:_mj_contactForce, cxx:_mj_flexBody, cxx:_mj_freeStack, cxx:_mj_markStack, cxx:_mj_objectAcceleration, cxx:_mj_rnePostConstraint, cxx:_mj_stackAllocInfo, cxx:_mj_stackAllocInt, cxx:_mj_stackAllocNum, cxx:_mju_addTo, cxx:_mju_addToScl3, cxx:_mju_condataSize, cxx:_mju_copy3, cxx:_mju_copy9, cxx:_mju_dispatch, cxx:_mju_dot3, cxx:_mju_isZero, cxx:_mju_message, cxx:_mju_min, cxx:_mju_norm3, cxx:_mju_normalize3, cxx:_mju_numThread, cxx:_mju_rayGeom, cxx:_mju_scl, cxx:_mju_scl3, cxx:_mju_transformSpatial, cxx:_mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_compute_sensor_acc(m: *const mjModel, d: *mut mjData, i: i32, sensordata: *mut f64) {
    todo!() // cxx-internal:engine_sensor.c.o:_mj_computeSensorAcc
}

/// C: compute_or_read_sensor (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_sensor.c:1387)
/// Calls: cxx:_mj_computeSensor, cxx:_mj_readSensor, cxx:_mju_copy
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn compute_or_read_sensor(m: *const mjModel, d: *mut mjData, i: i32, sensordata: *mut f64) {
    todo!() // cxx-internal:engine_sensor.c.o:_compute_or_read_sensor
}

/// C: compute_user_sensors (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_sensor.c:1432)
/// Calls: cxx-internal:engine_sensor.c.o:_apply_cutoff
#[allow(unused_variables, non_snake_case)]
pub fn compute_user_sensors(m: *const mjModel, d: *mut mjData, stage: u32) {
    todo!() // cxx-internal:engine_sensor.c.o:_compute_user_sensors
}

/// C: compute_plugin_sensors (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_sensor.c:1447)
/// Calls: cxx-internal:engine_sensor.c.o:_apply_cutoff, cxx:_mj_rnePostConstraint, cxx:_mj_subtreeVel, cxx:_mjp_getPluginAtSlotUnsafe, cxx:_mjp_pluginCount, cxx:_mju_message
#[allow(unused_variables, non_snake_case)]
pub fn compute_plugin_sensors(m: *const mjModel, d: *mut mjData, stage: u32) {
    todo!() // cxx-internal:engine_sensor.c.o:_compute_plugin_sensors
}

/// C: mj_computeSensor (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_sensor.h:29)
/// Calls: cxx-internal:engine_sensor.c.o:_apply_cutoff, cxx-internal:engine_sensor.c.o:_mj_computeSensorAcc, cxx-internal:engine_sensor.c.o:_mj_computeSensorPos, cxx-internal:engine_sensor.c.o:_mj_computeSensorVel, cxx:_mju_message
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_compute_sensor(m: *const mjModel, d: *mut mjData, i: i32, sensordata: *mut f64) {
    todo!() // cxx:_mj_computeSensor
}

/// C: mj_sensorPos (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_sensor.h:32)
/// Calls: cxx-internal:engine_sensor.c.o:_compute_or_read_sensor, cxx-internal:engine_sensor.c.o:_compute_plugin_sensors, cxx-internal:engine_sensor.c.o:_compute_user_sensors, cxx:_mj_sleepState, cxx:_mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mj_sensor_pos(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx:_mj_sensorPos
}

/// C: mj_sensorVel (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_sensor.h:35)
/// Calls: cxx-internal:engine_sensor.c.o:_compute_or_read_sensor, cxx-internal:engine_sensor.c.o:_compute_plugin_sensors, cxx-internal:engine_sensor.c.o:_compute_user_sensors, cxx:_mj_sleepState, cxx:_mj_subtreeVel, cxx:_mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mj_sensor_vel(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx:_mj_sensorVel
}

/// C: mj_sensorAcc (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_sensor.h:38)
/// Calls: cxx-internal:engine_sensor.c.o:_compute_or_read_sensor, cxx-internal:engine_sensor.c.o:_compute_plugin_sensors, cxx-internal:engine_sensor.c.o:_compute_user_sensors, cxx:_mj_rnePostConstraint, cxx:_mj_sleepState, cxx:_mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mj_sensor_acc(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx:_mj_sensorAcc
}

/// C: mj_energyPos (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_sensor.h:44)
/// Calls: cxx:_mj_sleepState, cxx:_mju_copy4, cxx:_mju_dot3, cxx:_mju_isZero, cxx:_mju_norm3, cxx:_mju_normalize4, cxx:_mju_polyPotential, cxx:_mju_sub3, cxx:_mju_subQuat
#[allow(unused_variables, non_snake_case)]
pub fn mj_energy_pos(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx:_mj_energyPos
}

/// C: mj_energyVel (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_sensor.h:47)
/// Calls: cxx:_mj_freeStack, cxx:_mj_markStack, cxx:_mj_mulM, cxx:_mj_stackAllocInfo, cxx:_mju_dot
#[allow(unused_variables, non_snake_case)]
pub fn mj_energy_vel(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx:_mj_energyVel
}

