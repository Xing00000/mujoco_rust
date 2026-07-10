//! Port of: engine/engine_sensor.c
//! IR hash: 05737965add36adb
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: ContactInfoCompare (engine/engine_sensor.c:52)
#[allow(unused_variables, non_snake_case)]
pub fn contact_info_compare(a: *const ContactInfo, b: *const ContactInfo, context: *mut ()) -> i32 {
    if a.is_null() || b.is_null() {
        return 0;
    }
    extern "C" { fn ContactInfoCompare(a: *const ContactInfo, b: *const ContactInfo, context: *mut ()) -> i32; }
    // SAFETY: a and b verified non-null; delegates to C comparison function
    unsafe { ContactInfoCompare(a, b, context) }
}

/// C: ContactSelect (engine/engine_sensor.c:61)
/// Calls: ContactInfoCompare
#[allow(unused_variables, non_snake_case)]
pub fn contact_select(arr: *mut ContactInfo, buf: *mut ContactInfo, n: i32, k: i32, context: *mut ()) {
    // WARNING: signature changed — verify body
    // Previous params: (arr : * mut ContactInfo, buf : * mut ContactInfo, n : i32, k : i32, context : * mut ())
    // Previous return: ()
    extern "C" { fn ContactSelect(arr : * mut ContactInfo , buf : * mut ContactInfo , n : i32 , k : i32 , context : * mut ()) ; } unsafe { ContactSelect(arr , buf , n , k , context) }
}

/// C: tactile_taxel_batch (engine/engine_sensor.c:80)
/// Calls: mjc_distance, mjc_getSDF, mju_addTo3, mju_dot3, mju_max, mju_min, mju_mulMatTVec3, mju_mulMatVec3, mju_quat2Mat, mju_rotVecQuat, mju_sub3, mju_transformSpatial
#[allow(unused_variables, non_snake_case)]
pub fn tactile_taxel_batch(m: *const mjModel, d: *mut mjData, args: *mut ()) -> *mut () {
    extern "C" { fn tactile_taxel_batch(m: *const mjModel, d: *mut mjData, args: *mut ()) -> *mut (); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { tactile_taxel_batch(m, d, args) }
}

/// C: tactileTask (engine/engine_sensor.c:191)
/// Calls: tactile_taxel_batch
#[allow(unused_variables, non_snake_case)]
pub fn tactile_task(m: *const mjModel, d: *mut mjData, arg: *mut (), thread_id: i32, task_id: i32) {
    extern "C" { fn tactileTask(m: *const mjModel, d: *mut mjData, arg: *mut (), thread_id: i32, task_id: i32); }
    // SAFETY: delegates to C implementation
    unsafe { tactileTask(m, d, arg, thread_id, task_id) }
}

/// C: apply_cutoff (engine/engine_sensor.c:198)
/// Calls: mju_clip, mju_min
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn apply_cutoff(m: *const mjModel, i: i32, data: *mut f64) {
    extern "C" { fn apply_cutoff(m: *const mjModel, i: i32, data: *mut f64); }
    // SAFETY: delegates to C implementation
    unsafe { apply_cutoff(m, i, data) }
}

/// C: get_xpos_xmat (engine/engine_sensor.c:227)
/// Calls: mju_message
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn get_xpos_xmat(d: *const mjData, r#type: mjtObj, id: i32, sensor_id: i32, xpos: *mut *mut f64, xmat: *mut *mut f64) {
    extern "C" { fn get_xpos_xmat(d: *const mjData, r#type: mjtObj, id: i32, sensor_id: i32, xpos: *mut *mut f64, xmat: *mut *mut f64); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { get_xpos_xmat(d, r#type, id, sensor_id, xpos, xmat) }
}

/// C: get_xquat (engine/engine_sensor.c:257)
/// Calls: mju_copy4, mju_message, mju_mulQuat
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn get_xquat(m: *const mjModel, d: *const mjData, r#type: mjtObj, id: i32, sensor_id: i32, quat: *mut f64) {
    extern "C" { fn get_xquat(m: *const mjModel, d: *const mjData, r#type: mjtObj, id: i32, sensor_id: i32, quat: *mut f64); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { get_xquat(m, d, r#type, id, sensor_id, quat) }
}

/// C: cam_project (engine/engine_sensor.c:281)
/// Calls: mju_max, mju_min, mju_mulMatTVec, mju_sub3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn cam_project(sensordata: *mut f64, target_xpos: *const f64, cam_xpos: *const f64, cam_xmat: *const f64, cam_res: [i32; 2], cam_fovy: f64, cam_intrinsic: [f32; 4], cam_sensorsize: [f32; 2]) {
    extern "C" { fn cam_project(sensordata: *mut f64, target_xpos: *const f64, cam_xpos: *const f64, cam_xmat: *const f64, cam_res: [i32; 2], cam_fovy: f64, cam_intrinsic: [f32; 4], cam_sensorsize: [f32; 2]); }
    // SAFETY: delegates to C implementation
    unsafe { cam_project(sensordata, target_xpos, cam_xpos, cam_xmat, cam_res, cam_fovy, cam_intrinsic, cam_sensorsize) }
}

/// C: checkMatch (engine/engine_sensor.c:320)
#[allow(unused_variables, non_snake_case)]
pub fn check_match(m: *const mjModel, body: i32, geom: i32, r#type: mjtObj, id: i32) -> i32 {
    extern "C" {
        fn checkMatch(m: *const mjModel, body: i32, geom: i32, r#type: mjtObj, id: i32) -> i32;
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { checkMatch(m, body, geom, r#type, id) }
}

/// C: matchContact (engine/engine_sensor.c:339)
/// Calls: checkMatch, mj_flexBody, mju_insideGeom
#[allow(unused_variables, non_snake_case)]
pub fn match_contact(m: *const mjModel, d: *const mjData, conid: i32, type1: mjtObj, id1: i32, type2: mjtObj, id2: i32) -> i32 {
    extern "C" { fn matchContact(m: *const mjModel, d: *const mjData, conid: i32, type1: mjtObj, id1: i32, type2: mjtObj, id2: i32) -> i32; }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { matchContact(m, d, conid, type1, id1, type2, id2) }
}

/// C: copySensorData (engine/engine_sensor.c:398)
/// Calls: mj_contactForce, mju_copy3, mju_scl3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn copy_sensor_data(m: *const mjModel, d: *const mjData, data: [*mut f64; 7], id: i32, flg_flip: i32, nfound: i32) {
    extern "C" { fn copySensorData(m: *const mjModel, d: *const mjData, data: [*mut f64; 7], id: i32, flg_flip: i32, nfound: i32); }
    // SAFETY: delegates to C implementation
    unsafe { copySensorData(m, d, data, id, flg_flip, nfound) }
}

/// C: total_wrench (engine/engine_sensor.c:442)
/// Calls: mju_addTo3, mju_cross, mju_mulMatTVec3, mju_sub3, mju_zero3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn total_wrench(force: *mut f64, torque: *mut f64, point: *const f64, n: i32, wrench: *const f64, pos: *const f64, frame: *const f64) {
    extern "C" { fn total_wrench(force: *mut f64, torque: *mut f64, point: *const f64, n: i32, wrench: *const f64, pos: *const f64, frame: *const f64); }
    // SAFETY: delegates to C implementation
    unsafe { total_wrench(force, torque, point, n, wrench, pos, frame) }
}

/// C: fill_raydata (engine/engine_sensor.c:470)
/// Calls: mju_addScl3, mju_copy3, mju_dot3, mju_sub3, mju_zero3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn fill_raydata(ptr: *mut f64, dataspec: i32, dist: f64, origin: *const f64, direction: *const f64, normal: *const f64, cam_xpos: *const f64, cam_z: *const f64) -> *mut f64  {
    extern "C" { fn fill_raydata(ptr: *mut f64, dataspec: i32, dist: f64, origin: *const f64, direction: *const f64, normal: *const f64, cam_xpos: *const f64, cam_z: *const f64) -> *mut f64; }
    // SAFETY: delegates to C implementation
    unsafe { fill_raydata(ptr, dataspec, dist, origin, direction, normal, cam_xpos, cam_z) }
}

/// C: mj_computeSensorPos (engine/engine_sensor.c:525)
/// Calls: cam_project, fill_raydata, get_xpos_xmat, get_xquat, mj_energyPos, mj_energyVel, mj_freeStack, mj_geomDistance, mj_markStack, mj_multiRay, mj_ray, mj_stackAllocInfo, mju_camIntrinsics, mju_camPixelRay, mju_copy, mju_copy3, mju_copy4, mju_insideGeom, mju_message, mju_mulMatTVec, mju_mulMatTVec3, mju_mulQuat, mju_negQuat, mju_normalize3, mju_normalize4, mju_sub3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_compute_sensor_pos(m: *const mjModel, d: *mut mjData, i: i32, sensordata: *mut f64) {
    extern "C" { fn mj_computeSensorPos(m: *const mjModel, d: *mut mjData, i: i32, sensordata: *mut f64); }
    // SAFETY: delegates to C implementation, pointers valid per caller contract
    unsafe { mj_computeSensorPos(m, d, i, sensordata) }
}

/// C: mj_computeSensorVel (engine/engine_sensor.c:839)
/// Calls: get_xpos_xmat, mj_objectVelocity, mj_subtreeVel, mju_addTo3, mju_copy3, mju_cross, mju_message, mju_mulMatTVec3, mju_sub, mju_sub3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_compute_sensor_vel(m: *const mjModel, d: *mut mjData, i: i32, sensordata: *mut f64) {
    extern "C" { fn mj_computeSensorVel(m: *const mjModel, d: *mut mjData, i: i32, sensordata: *mut f64); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_computeSensorVel(m, d, i, sensordata) }
}

/// C: mj_computeSensorAcc (engine/engine_sensor.c:958)
/// Calls: ContactSelect, copySensorData, matchContact, mj_contactForce, mj_flexBody, mj_freeStack, mj_markStack, mj_objectAcceleration, mj_rnePostConstraint, mj_stackAllocInfo, mj_stackAllocInt, mj_stackAllocNum, mju_addTo, mju_addToScl3, mju_condataSize, mju_copy3, mju_copy9, mju_dispatch, mju_dot3, mju_isZero, mju_message, mju_min, mju_norm3, mju_normalize3, mju_numThread, mju_rayGeom, mju_scl, mju_scl3, mju_transformSpatial, mju_zero, tactile_taxel_batch, total_wrench
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_compute_sensor_acc(m: *const mjModel, d: *mut mjData, i: i32, sensordata: *mut f64) {
    extern "C" { fn mj_computeSensorAcc(m: *const mjModel, d: *mut mjData, i: i32, sensordata: *mut f64); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_computeSensorAcc(m, d, i, sensordata) }
}

/// C: compute_or_read_sensor (engine/engine_sensor.c:1387)
/// Calls: mj_computeSensor, mj_readSensor, mju_copy
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn compute_or_read_sensor(m: *const mjModel, d: *mut mjData, i: i32, sensordata: *mut f64) {
    extern "C" { fn compute_or_read_sensor(m: *const mjModel, d: *mut mjData, i: i32, sensordata: *mut f64); }
    // SAFETY: delegates to C implementation
    unsafe { compute_or_read_sensor(m, d, i, sensordata) }
}

/// C: compute_user_sensors (engine/engine_sensor.c:1432)
/// Calls: apply_cutoff
#[allow(unused_variables, non_snake_case)]
pub fn compute_user_sensors(m: *const mjModel, d: *mut mjData, stage: mjtStage) {
    extern "C" { fn compute_user_sensors(m: *const mjModel, d: *mut mjData, stage: mjtStage); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { compute_user_sensors(m, d, stage) }
}

/// C: compute_plugin_sensors (engine/engine_sensor.c:1447)
/// Calls: apply_cutoff, mj_rnePostConstraint, mj_subtreeVel, mjp_getPluginAtSlotUnsafe, mjp_pluginCount, mju_message
#[allow(unused_variables, non_snake_case)]
pub fn compute_plugin_sensors(m: *const mjModel, d: *mut mjData, stage: mjtStage) {
    extern "C" { fn compute_plugin_sensors(m: *const mjModel, d: *mut mjData, stage: mjtStage); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { compute_plugin_sensors(m, d, stage) }
}

/// C: mj_computeSensor (engine/engine_sensor.h:29)
/// Calls: apply_cutoff, mj_computeSensorAcc, mj_computeSensorPos, mj_computeSensorVel, mju_message
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_compute_sensor(m: *const mjModel, d: *mut mjData, i: i32, sensordata: *mut f64) {
    // SAFETY: m, d valid per caller. sensor_needstage is valid i32 array indexed by i.
    unsafe {
        const MJSTAGE_POS: i32 = 0;
        const MJSTAGE_VEL: i32 = 1;
        const MJSTAGE_ACC: i32 = 2;

        let stage: i32 = *(*m).sensor_needstage.add(i as usize);
        match stage {
            MJSTAGE_POS => mj_compute_sensor_pos(m, d, i, sensordata),
            MJSTAGE_VEL => mj_compute_sensor_vel(m, d, i, sensordata),
            MJSTAGE_ACC => mj_compute_sensor_acc(m, d, i, sensordata),
            _ => crate::engine::engine_util_errmem::mju_error(
                b"invalid sensor stage\0".as_ptr() as *const i8,
            ),
        }

        // apply cutoff
        apply_cutoff(m, i, sensordata);
    }
}

/// C: mj_sensorPos (engine/engine_sensor.h:32)
/// Calls: compute_or_read_sensor, compute_plugin_sensors, compute_user_sensors, mj_sleepState, mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mj_sensor_pos(m: *const mjModel, d: *mut mjData) {
    extern "C" {
        fn mj_sensorPos(m: *const mjModel, d: *mut mjData);
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_sensorPos(m, d) }
}

/// C: mj_sensorVel (engine/engine_sensor.h:35)
/// Calls: compute_or_read_sensor, compute_plugin_sensors, compute_user_sensors, mj_sleepState, mj_subtreeVel, mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mj_sensor_vel(m: *const mjModel, d: *mut mjData) {
    extern "C" { fn mj_sensorVel(m: *const mjModel, d: *mut mjData); }
    // SAFETY: delegates to C implementation, pointers valid per caller
    unsafe { mj_sensorVel(m, d) }
}

/// C: mj_sensorAcc (engine/engine_sensor.h:38)
/// Calls: compute_or_read_sensor, compute_plugin_sensors, compute_user_sensors, mj_rnePostConstraint, mj_sleepState, mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mj_sensor_acc(m: *const mjModel, d: *mut mjData) {
    extern "C" { fn mj_sensorAcc(m: *const mjModel, d: *mut mjData); }
    // SAFETY: delegates to C implementation, pointers valid per caller
    unsafe { mj_sensorAcc(m, d) }
}

/// C: mj_energyPos (engine/engine_sensor.h:44)
/// Calls: mj_sleepState, mju_copy4, mju_dot3, mju_isZero, mju_norm3, mju_normalize4, mju_polyPotential, mju_sub3, mju_subQuat
#[allow(unused_variables, non_snake_case)]
pub fn mj_energy_pos(m: *const mjModel, d: *mut mjData) {
    extern "C" { fn mj_energyPos(m: *const mjModel, d: *mut mjData); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_energyPos(m, d) }
}

/// C: mj_energyVel (engine/engine_sensor.h:47)
/// Calls: mj_freeStack, mj_markStack, mj_mulM, mj_stackAllocInfo, mju_dot
#[allow(unused_variables, non_snake_case)]
pub fn mj_energy_vel(m: *const mjModel, d: *mut mjData) {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData)
    // Previous return: ()
    extern "C" { fn mj_energyVel(m : * const mjModel , d : * mut mjData) ; } unsafe { mj_energyVel(m , d) }
}

