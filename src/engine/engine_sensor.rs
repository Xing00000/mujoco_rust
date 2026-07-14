//! Port of: engine/engine_sensor.c
//! IR hash: 9614bd3d92e7766b
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: ContactInfoCompare (engine/engine_sensor.c:52)
#[allow(unused_variables, non_snake_case)]
pub fn contact_info_compare(a: *const ContactInfo, b: *const ContactInfo, context: *mut ()) -> i32 {
    // SAFETY: a and b point to valid ContactInfo structs with layout {criterion: f64, id: i32, flip: i32}
    unsafe {
        #[repr(C)]
        struct ContactInfoRepr {
            criterion: f64,
            id: i32,
            flip: i32,
        }
        let a = &*(a as *const ContactInfoRepr);
        let b = &*(b as *const ContactInfoRepr);

        if a.criterion < b.criterion { return -1; }
        if a.criterion > b.criterion { return 1; }

        if a.id < b.id { return -1; }
        if a.id > b.id { return 1; }

        0
    }
}

/// C: ContactSelect (engine/engine_sensor.c:61)
/// Calls: ContactInfoCompare
#[allow(unused_variables, non_snake_case)]
pub fn contact_select(arr: *mut ContactInfo, buf: *mut ContactInfo, n: i32, k: i32, context: *mut ()) {
    todo!() // ContactSelect
}

/// C: tactile_taxel_batch (engine/engine_sensor.c:80)
/// Calls: mjc_distance, mjc_getSDF, mju_addTo3, mju_dot3, mju_max, mju_min, mju_mulMatTVec3, mju_mulMatVec3, mju_quat2Mat, mju_rotVecQuat, mju_sub3, mju_transformSpatial
#[allow(unused_variables, non_snake_case)]
pub fn tactile_taxel_batch(m: *const mjModel, d: *mut mjData, args: *mut ()) -> *mut () {
    todo!() // tactile_taxel_batch
}

/// C: tactileTask (engine/engine_sensor.c:191)
/// Calls: tactile_taxel_batch
#[allow(unused_variables, non_snake_case)]
pub fn tactile_task(m: *const mjModel, d: *mut mjData, arg: *mut (), thread_id: i32, task_id: i32) {
    todo!() // tactileTask
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
    // SAFETY: caller guarantees m is valid mjModel, data has at least sensor_dim[i] elements
    unsafe {
        let cutoff = *(*m).sensor_cutoff.add(i as usize);
        if cutoff <= 0.0 {
            return;
        }

        // cutoff ignored for contact and fromto sensors
        let sensor_type = *(*m).sensor_type.add(i as usize) as u32;
        if sensor_type == mjtSensor_mjSENS_CONTACT || sensor_type == mjtSensor_mjSENS_GEOMFROMTO {
            return;
        }

        let dim = *(*m).sensor_dim.add(i as usize);

        for j in 0..dim as usize {
            // real: apply on both sides
            if *(*m).sensor_datatype.add(i as usize) as u32 == mjtDataType_mjDATATYPE_REAL {
                *data.add(j) = crate::engine::engine_util_misc::mju_clip(*data.add(j), -cutoff, cutoff);
            }
            // positive: apply on positive side only
            else if *(*m).sensor_datatype.add(i as usize) as u32 == mjtDataType_mjDATATYPE_POSITIVE {
                *data.add(j) = crate::engine::engine_util_misc::mju_min(cutoff, *data.add(j));
            }
        }
    }
}

/// C: get_xpos_xmat (engine/engine_sensor.c:227)
/// Calls: mju_message
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn get_xpos_xmat(d: *const mjData, r#type: u32, id: i32, sensor_id: i32, xpos: *mut *mut f64, xmat: *mut *mut f64) {
    todo!() // get_xpos_xmat
}

/// C: get_xquat (engine/engine_sensor.c:257)
/// Calls: mju_copy4, mju_message, mju_mulQuat
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn get_xquat(m: *const mjModel, d: *const mjData, r#type: u32, id: i32, sensor_id: i32, quat: *mut f64) {
    todo!() // get_xquat
}

/// C: cam_project (engine/engine_sensor.c:281)
/// Calls: mju_max, mju_min, mju_mulMatTVec, mju_sub3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn cam_project(sensordata: *mut f64, target_xpos: *const f64, cam_xpos: *const f64, cam_xmat: *const f64, cam_res: *const i32, cam_fovy: f64, cam_intrinsic: *const f32, cam_sensorsize: *const f32) {
    use crate::engine::engine_util_blas::{mju_sub3, mju_mul_mat_t_vec};

    const MJ_MINVAL: f64 = 1e-15;
    const MJ_PI: f64 = std::f64::consts::PI;

    // SAFETY: caller guarantees all pointers are valid with correct sizes
    unsafe {
        let fx: f64;
        let fy: f64;

        if *cam_sensorsize.add(0) != 0.0 && *cam_sensorsize.add(1) != 0.0 {
            fx = (*cam_intrinsic.add(0) as f64) / (*cam_sensorsize.add(0) as f64) * (*cam_res.add(0) as f64);
            fy = (*cam_intrinsic.add(1) as f64) / (*cam_sensorsize.add(1) as f64) * (*cam_res.add(1) as f64);
        } else {
            let val = 0.5 / f64::tan(cam_fovy * MJ_PI / 360.0) * (*cam_res.add(1) as f64);
            fx = val;
            fy = val;
        }

        let mut relative_pos: [f64; 3] = [0.0; 3];
        mju_sub3(relative_pos.as_mut_ptr(), target_xpos, cam_xpos);

        let mut cam_pos: [f64; 3] = [0.0; 3];
        mju_mul_mat_t_vec(cam_pos.as_mut_ptr(), cam_xmat, relative_pos.as_ptr(), 3, 3);

        let mut denom: f64 = cam_pos[2];
        if f64::abs(denom) < MJ_MINVAL {
            if denom < 0.0 {
                denom = if denom < -MJ_MINVAL { denom } else { -MJ_MINVAL };
            } else {
                denom = if denom > MJ_MINVAL { denom } else { MJ_MINVAL };
            }
        }

        *sensordata.add(0) = -fx * (cam_pos[0] / denom) + 0.5 * (*cam_res.add(0) as f64);
        *sensordata.add(1) =  fy * (cam_pos[1] / denom) + 0.5 * (*cam_res.add(1) as f64);
    }
}

/// C: checkMatch (engine/engine_sensor.c:320)
/// Calls: mjCMesh::tree
#[allow(unused_variables, non_snake_case)]
pub fn check_match(m: *const mjModel, mut body: i32, geom: i32, r#type: u32, id: i32) -> i32 {
    use crate::types::*;

    if r#type == mjtObj_mjOBJ_UNKNOWN { return 1; }
    if r#type == mjtObj_mjOBJ_SITE { return 1; }  // already passed site filter test
    if r#type == mjtObj_mjOBJ_GEOM { return if id == geom { 1 } else { 0 }; }
    if r#type == mjtObj_mjOBJ_BODY { return if id == body { 1 } else { 0 }; }
    if r#type == mjtObj_mjOBJ_XBODY {
        // SAFETY: m is a valid mjModel pointer, body_parentid valid (caller contract)
        unsafe {
            // traverse up the tree from body, return true if we land on id
            while body > id {
                body = *(*m).body_parentid.add(body as usize);
            }
        }
        return if body == id { 1 } else { 0 };
    }

    0
}

/// C: matchContact (engine/engine_sensor.c:339)
/// Calls: checkMatch, mj_flexBody, mju_insideGeom
#[allow(unused_variables, non_snake_case)]
pub fn match_contact(m: *const mjModel, d: *const mjData, conid: i32, type1: u32, id1: i32, type2: u32, id2: i32) -> i32 {
    todo!() // matchContact
}

/// C: copySensorData (engine/engine_sensor.c:398)
/// Calls: mj_contactForce, mju_copy3, mju_scl3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn copy_sensor_data(m: *const mjModel, d: *const mjData, data: *mut *mut f64, id: i32, flg_flip: i32, nfound: i32) {
    todo!() // copySensorData
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
    use crate::engine::engine_util_blas::{mju_zero3, mju_add_to3, mju_sub3, mju_mul_mat_t_vec3};
    use crate::engine::engine_util_spatial::mju_cross;

    // SAFETY: all pointers are valid arrays with sizes guaranteed by caller contract
    unsafe {
        mju_zero3(force);
        mju_zero3(torque);

        for j in 0..n as usize {
            // rotate force, torque from contact frame to global frame
            let mut force_j: [f64; 3] = [0.0; 3];
            let mut torque_j: [f64; 3] = [0.0; 3];
            mju_mul_mat_t_vec3(force_j.as_mut_ptr(), frame.add(9 * j), wrench.add(6 * j));
            mju_mul_mat_t_vec3(torque_j.as_mut_ptr(), frame.add(9 * j), wrench.add(6 * j + 3));

            // add to total force, torque
            mju_add_to3(force, force_j.as_ptr());
            mju_add_to3(torque, torque_j.as_ptr());

            // add induced moment: torque += (pos - point) x force
            let mut diff: [f64; 3] = [0.0; 3];
            mju_sub3(diff.as_mut_ptr(), pos.add(3 * j), point);
            let mut induced_torque: [f64; 3] = [0.0; 3];
            mju_cross(induced_torque.as_mut_ptr(), diff.as_ptr(), force_j.as_ptr());
            mju_add_to3(torque, induced_torque.as_ptr());
        }
    }
}

/// C: fill_raydata (engine/engine_sensor.c:470)
/// Calls: mju_addScl3, mju_copy3, mju_dot3, mju_sub3, mju_zero3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn fill_raydata(ptr: *mut f64, dataspec: i32, dist: f64, origin: *const f64, direction: *const f64, normal: *const f64, cam_xpos: *const f64, cam_z: *const f64) -> *mut f64 {
    todo!() // fill_raydata
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
    todo!() // mj_computeSensorPos
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
    todo!() // mj_computeSensorVel
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
    todo!() // mj_computeSensorAcc
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
    todo!() // compute_or_read_sensor
}

/// C: compute_user_sensors (engine/engine_sensor.c:1432)
/// Calls: apply_cutoff
#[allow(unused_variables, non_snake_case)]
pub fn compute_user_sensors(m: *const mjModel, d: *mut mjData, stage: u32) {
    todo!() // compute_user_sensors
}

/// C: compute_plugin_sensors (engine/engine_sensor.c:1447)
/// Calls: apply_cutoff, mj_rnePostConstraint, mj_subtreeVel, mjp_getPluginAtSlotUnsafe, mjp_pluginCount, mju_message
#[allow(unused_variables, non_snake_case)]
pub fn compute_plugin_sensors(m: *const mjModel, d: *mut mjData, stage: u32) {
    todo!() // compute_plugin_sensors
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
    todo!() // mj_computeSensor
}

/// C: mj_sensorPos (engine/engine_sensor.h:32)
/// Calls: compute_or_read_sensor, compute_plugin_sensors, compute_user_sensors, mj_sleepState, mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mj_sensor_pos(m: *const mjModel, d: *mut mjData) {
    todo!() // mj_sensorPos
}

/// C: mj_sensorVel (engine/engine_sensor.h:35)
/// Calls: compute_or_read_sensor, compute_plugin_sensors, compute_user_sensors, mj_sleepState, mj_subtreeVel, mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mj_sensor_vel(m: *const mjModel, d: *mut mjData) {
    todo!() // mj_sensorVel
}

/// C: mj_sensorAcc (engine/engine_sensor.h:38)
/// Calls: compute_or_read_sensor, compute_plugin_sensors, compute_user_sensors, mj_rnePostConstraint, mj_sleepState, mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mj_sensor_acc(m: *const mjModel, d: *mut mjData) {
    todo!() // mj_sensorAcc
}

/// C: mj_energyPos (engine/engine_sensor.h:44)
/// Calls: mj_sleepState, mju_copy4, mju_dot3, mju_isZero, mju_norm3, mju_normalize4, mju_polyPotential, mju_sub3, mju_subQuat
#[allow(unused_variables, non_snake_case)]
pub fn mj_energy_pos(m: *const mjModel, d: *mut mjData) {
    todo!() // mj_energyPos
}

/// C: mj_energyVel (engine/engine_sensor.h:47)
/// Calls: mj_freeStack, mj_markStack, mj_mulM, mj_stackAllocInfo, mju_dot
#[allow(unused_variables, non_snake_case)]
pub fn mj_energy_vel(m: *const mjModel, d: *mut mjData) {
    todo!() // mj_energyVel
}

