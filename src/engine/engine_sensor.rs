//! Port of: engine/engine_sensor.c
//! IR hash: fd5288ad7242f001
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: ContactInfoCompare (engine/engine_sensor.c:53)
#[allow(unused_variables, non_snake_case)]
pub fn contact_info_compare(a: *const ContactInfo, b: *const ContactInfo, context: *mut ()) -> i32 {
    todo!() // cxx-internal:engine_sensor.c.o:_ContactInfoCompare
}

/// C: ContactSelect (engine/engine_sensor.c:62)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_sensor.c:_ContactInfoCompare
#[allow(unused_variables, non_snake_case)]
pub fn contact_select(arr: *mut ContactInfo, buf: *mut ContactInfo, n: i32, k: i32, context: *mut ()) {
    todo!() // cxx-internal:engine_sensor.c.o:_ContactSelect
}

/// C: tactile_taxel_batch (engine/engine_sensor.c:81)
/// Calls: cxx:_mjc_distance, cxx:_mjc_getSDF, cxx:_mju_addTo3, cxx:_mju_dot3, cxx:_mju_max, cxx:_mju_min, cxx:_mju_mulMatTVec3, cxx:_mju_mulMatVec3, cxx:_mju_quat2Mat, cxx:_mju_sub3, cxx:_mju_transformSpatial
#[allow(unused_variables, non_snake_case)]
pub fn tactile_taxel_batch(m: *const mjModel, d: *mut mjData, args: *mut ()) -> *mut () {
    todo!() // cxx-internal:engine_sensor.c.o:_tactile_taxel_batch
}

/// C: tactileTask (engine/engine_sensor.c:189)
/// Calls: cxx-internal:engine_sensor.c.o:_tactile_taxel_batch
#[allow(unused_variables, non_snake_case)]
pub fn tactile_task(m: *const mjModel, d: *mut mjData, arg: *mut (), thread_id: i32, task_id: i32) {
    todo!() // cxx-internal:engine_sensor.c.o:_tactileTask
}

/// C: apply_cutoff (engine/engine_sensor.c:196)
/// Calls: cxx:_mju_clip, cxx:_mju_min
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

/// C: get_xpos_xmat (engine/engine_sensor.c:225)
/// Calls: cxx:_mju_message
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn get_xpos_xmat(d: *const mjData, r#type: u32, id: i32, sensor_id: i32, xpos: *mut *mut f64, xmat: *mut *mut f64) {
    // SAFETY: d is a valid mjData pointer, xpos/xmat are valid output pointers (caller contract)
    unsafe {
        match r#type {
            mjtObj_mjOBJ_XBODY => {
                *xpos = (*d).xpos.add(3 * id as usize);
                *xmat = (*d).xmat.add(9 * id as usize);
            }
            mjtObj_mjOBJ_BODY => {
                *xpos = (*d).xipos.add(3 * id as usize);
                *xmat = (*d).ximat.add(9 * id as usize);
            }
            mjtObj_mjOBJ_GEOM => {
                *xpos = (*d).geom_xpos.add(3 * id as usize);
                *xmat = (*d).geom_xmat.add(9 * id as usize);
            }
            mjtObj_mjOBJ_SITE => {
                *xpos = (*d).site_xpos.add(3 * id as usize);
                *xmat = (*d).site_xmat.add(9 * id as usize);
            }
            mjtObj_mjOBJ_CAMERA => {
                *xpos = (*d).cam_xpos.add(3 * id as usize);
                *xmat = (*d).cam_xmat.add(9 * id as usize);
            }
            _ => {
                crate::engine::engine_util_errmem::mju_error(
                    b"invalid object type in sensor %d\0".as_ptr() as *const i8);
            }
        }
    }
}

/// C: get_xquat (engine/engine_sensor.c:255)
/// Calls: cxx:_mju_copy4, cxx:_mju_message, cxx:_mju_mulQuat
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn get_xquat(m: *const mjModel, d: *const mjData, r#type: u32, id: i32, sensor_id: i32, quat: *mut f64) {
    // SAFETY: m, d are valid pointers; quat points to 4 f64 (caller contract)
    unsafe {
        match r#type {
            mjtObj_mjOBJ_XBODY => {
                crate::engine::engine_util_blas::mju_copy4(quat, (*d).xquat.add(4 * id as usize));
            }
            mjtObj_mjOBJ_BODY => {
                crate::engine::engine_util_spatial::mju_mulQuat(
                    quat, (*d).xquat.add(4 * id as usize), (*m).body_iquat.add(4 * id as usize));
            }
            mjtObj_mjOBJ_GEOM => {
                crate::engine::engine_util_spatial::mju_mulQuat(
                    quat,
                    (*d).xquat.add(4 * *(*m).geom_bodyid.add(id as usize) as usize),
                    (*m).geom_quat.add(4 * id as usize));
            }
            mjtObj_mjOBJ_SITE => {
                crate::engine::engine_util_spatial::mju_mulQuat(
                    quat,
                    (*d).xquat.add(4 * *(*m).site_bodyid.add(id as usize) as usize),
                    (*m).site_quat.add(4 * id as usize));
            }
            mjtObj_mjOBJ_CAMERA => {
                crate::engine::engine_util_spatial::mju_mulQuat(
                    quat,
                    (*d).xquat.add(4 * *(*m).cam_bodyid.add(id as usize) as usize),
                    (*m).cam_quat.add(4 * id as usize));
            }
            _ => {
                crate::engine::engine_util_errmem::mju_error(
                    b"invalid object type in sensor %d\0".as_ptr() as *const i8);
            }
        }
    }
}

/// C: cam_project (engine/engine_sensor.c:279)
/// Calls: cxx:_mju_max, cxx:_mju_min, cxx:_mju_mulMatTVec, cxx:_mju_sub3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn cam_project(sensordata: *mut f64, target_xpos: *const f64, cam_xpos: *const f64, cam_xmat: *const f64, cam_res: *const i32, cam_fovy: f64, cam_intrinsic: *const f32, cam_sensorsize: *const f32) {
    use crate::engine::engine_util_blas::{mju_sub3, mju_mulMatTVec};

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
        mju_mulMatTVec(cam_pos.as_mut_ptr(), cam_xmat, relative_pos.as_ptr(), 3, 3);

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

/// C: checkMatch (engine/engine_sensor.c:318)
/// Calls: cxx:__ZN7mjCMesh4treeEv
#[allow(unused_variables, non_snake_case)]
pub fn check_match(m: *const mjModel, body: i32, geom: i32, r#type: u32, id: i32) -> i32 {
    todo!() // cxx-internal:engine_sensor.c.o:_checkMatch
}

/// C: matchContact (engine/engine_sensor.c:337)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_sensor.c:_checkMatch, cxx:_mj_flexBody, cxx:_mj_insideSite
#[allow(unused_variables, non_snake_case)]
pub fn match_contact(m: *const mjModel, d: *const mjData, conid: i32, type1: u32, id1: i32, type2: u32, id2: i32) -> i32 {
    todo!() // cxx-internal:engine_sensor.c.o:_matchContact
}

/// C: copySensorData (engine/engine_sensor.c:395)
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

/// C: total_wrench (engine/engine_sensor.c:439)
/// Calls: cxx:_mju_addTo3, cxx:_mju_cross, cxx:_mju_mulMatTVec3, cxx:_mju_sub3, cxx:_mju_zero3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn total_wrench(force: *mut f64, torque: *mut f64, point: *const f64, n: i32, wrench: *const f64, pos: *const f64, frame: *const f64) {
    use crate::engine::engine_util_blas::{mju_zero3, mju_addTo3, mju_sub3, mju_mulMatTVec3};
    use crate::engine::engine_util_spatial::mju_cross;

    // SAFETY: all pointers are valid arrays with sizes guaranteed by caller contract
    unsafe {
        mju_zero3(force);
        mju_zero3(torque);

        for j in 0..n as usize {
            // rotate force, torque from contact frame to global frame
            let mut force_j: [f64; 3] = [0.0; 3];
            let mut torque_j: [f64; 3] = [0.0; 3];
            mju_mulMatTVec3(force_j.as_mut_ptr(), frame.add(9 * j), wrench.add(6 * j));
            mju_mulMatTVec3(torque_j.as_mut_ptr(), frame.add(9 * j), wrench.add(6 * j + 3));

            // add to total force, torque
            mju_addTo3(force, force_j.as_ptr());
            mju_addTo3(torque, torque_j.as_ptr());

            // add induced moment: torque += (pos - point) x force
            let mut diff: [f64; 3] = [0.0; 3];
            mju_sub3(diff.as_mut_ptr(), pos.add(3 * j), point);
            let mut induced_torque: [f64; 3] = [0.0; 3];
            mju_cross(induced_torque.as_mut_ptr(), diff.as_ptr(), force_j.as_ptr());
            mju_addTo3(torque, induced_torque.as_ptr());
        }
    }
}

/// C: fill_raydata (engine/engine_sensor.c:467)
/// Calls: cxx:_mju_addScl3, cxx:_mju_copy3, cxx:_mju_dot3, cxx:_mju_sub3, cxx:_mju_zero3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn fill_raydata(ptr: *mut f64, dataspec: i32, dist: f64, origin: *const f64, direction: *const f64, normal: *const f64, cam_xpos: *const f64, cam_z: *const f64) -> *mut f64 {
    use crate::engine::engine_util_blas::{mju_copy3, mju_zero3, mju_addScl3, mju_sub3, mju_dot3};

    const RAYDATA_DIST: i32 = 0;
    const RAYDATA_DIR: i32 = 1;
    const RAYDATA_ORIGIN: i32 = 2;
    const RAYDATA_POINT: i32 = 3;
    const RAYDATA_NORMAL: i32 = 4;
    const RAYDATA_DEPTH: i32 = 5;

    // SAFETY: caller guarantees all pointers are valid and ptr has sufficient space
    unsafe {
        let mut p = ptr;
        let hit: i32 = if dist >= 0.0 { 1 } else { 0 };

        if (dataspec & (1 << RAYDATA_DIST)) != 0 {
            *p = dist;
            p = p.add(1);
        }
        if (dataspec & (1 << RAYDATA_DIR)) != 0 {
            if hit != 0 {
                mju_copy3(p, direction);
            } else {
                mju_zero3(p);
            }
            p = p.add(3);
        }
        if (dataspec & (1 << RAYDATA_ORIGIN)) != 0 {
            mju_copy3(p, origin);
            p = p.add(3);
        }

        // compute point if needed for POINT or DEPTH fields
        let mut point: [f64; 3] = [0.0, 0.0, 0.0];
        if ((dataspec & (1 << RAYDATA_POINT)) != 0) || ((dataspec & (1 << RAYDATA_DEPTH)) != 0) {
            if hit != 0 {
                mju_addScl3(point.as_mut_ptr(), origin, direction, dist);
            }
        }

        if (dataspec & (1 << RAYDATA_POINT)) != 0 {
            mju_copy3(p, point.as_ptr());
            p = p.add(3);
        }
        if (dataspec & (1 << RAYDATA_NORMAL)) != 0 {
            if hit != 0 {
                mju_copy3(p, normal);
            } else {
                mju_zero3(p);
            }
            p = p.add(3);
        }
        if (dataspec & (1 << RAYDATA_DEPTH)) != 0 {
            if hit != 0 {
                if !cam_z.is_null() {
                    // camera depth: project onto camera z-axis
                    let mut delta: [f64; 3] = [0.0; 3];
                    mju_sub3(delta.as_mut_ptr(), point.as_ptr(), cam_xpos);
                    *p = -mju_dot3(delta.as_ptr(), cam_z);
                    p = p.add(1);
                } else {
                    // site sensor: depth = dist
                    *p = dist;
                    p = p.add(1);
                }
            } else {
                *p = -1.0;
                p = p.add(1);
            }
        }

        p
    }
}

/// C: mj_computeSensorPos (engine/engine_sensor.c:522)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_sensor.c:_cam_project, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_sensor.c:_get_xpos_xmat, cxx-internal:engine_sensor.c.o:_fill_raydata, cxx-internal:engine_sensor.c.o:_get_xquat, cxx:_mj_energyPos, cxx:_mj_energyVel, cxx:_mj_freeStack, cxx:_mj_geomDistance, cxx:_mj_insideSite, cxx:_mj_markStack, cxx:_mj_multiRay, cxx:_mj_ray, cxx:_mj_stackAllocInfo, cxx:_mju_camIntrinsics, cxx:_mju_camPixelRay, cxx:_mju_copy, cxx:_mju_copy3, cxx:_mju_copy4, cxx:_mju_message, cxx:_mju_mulMatTVec, cxx:_mju_mulMatTVec3, cxx:_mju_mulQuat, cxx:_mju_negQuat, cxx:_mju_normalize3, cxx:_mju_normalize4, cxx:_mju_sub3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_compute_sensor_pos(m: *const mjModel, d: *mut mjData, i: i32, sensordata: *mut f64) {
    todo!() // cxx-internal:engine_sensor.c.o:_mj_computeSensorPos
}

/// C: mj_computeSensorVel (engine/engine_sensor.c:832)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_sensor.c:_get_xpos_xmat, cxx:_mj_objectVelocity, cxx:_mj_subtreeVel, cxx:_mju_addTo3, cxx:_mju_copy, cxx:_mju_copy3, cxx:_mju_cross, cxx:_mju_message, cxx:_mju_mulMatTVec3, cxx:_mju_sub, cxx:_mju_sub3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_compute_sensor_vel(m: *const mjModel, d: *mut mjData, i: i32, sensordata: *mut f64) {
    todo!() // cxx-internal:engine_sensor.c.o:_mj_computeSensorVel
}

/// C: mj_computeSensorAcc (engine/engine_sensor.c:951)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_sensor.c:_ContactSelect, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_sensor.c:_copySensorData, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_sensor.c:_matchContact, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_sensor.c:_total_wrench, cxx-internal:engine_sensor.c.o:_tactile_taxel_batch, cxx:_mj_contactForce, cxx:_mj_flexBody, cxx:_mj_freeStack, cxx:_mj_markStack, cxx:_mj_objectAcceleration, cxx:_mj_rnePostConstraint, cxx:_mj_stackAllocInfo, cxx:_mj_stackAllocInt, cxx:_mju_addToScl3, cxx:_mju_condataSize, cxx:_mju_copy, cxx:_mju_copy3, cxx:_mju_copy9, cxx:_mju_dispatch, cxx:_mju_dot3, cxx:_mju_message, cxx:_mju_min, cxx:_mju_norm3, cxx:_mju_normalize3, cxx:_mju_numThread, cxx:_mju_rayGeom, cxx:_mju_scl, cxx:_mju_scl3, cxx:_mju_transformSpatial, cxx:_mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_compute_sensor_acc(m: *const mjModel, d: *mut mjData, i: i32, sensordata: *mut f64) {
    todo!() // cxx-internal:engine_sensor.c.o:_mj_computeSensorAcc
}

/// C: mj_computeSensor (engine/engine_sensor.c:1330)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_sensor.c:_apply_cutoff, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_sensor.c:_mj_computeSensorAcc, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_sensor.c:_mj_computeSensorPos, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_sensor.c:_mj_computeSensorVel, cxx:_mju_message
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_compute_sensor(m: *const mjModel, d: *mut mjData, i: i32, sensordata: *mut f64) {
    todo!() // cxx:_mj_computeSensor
}

/// C: compute_or_read_sensor (engine/engine_sensor.c:1354)
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

/// C: compute_user_sensors (engine/engine_sensor.c:1399)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_sensor.c:_apply_cutoff
#[allow(unused_variables, non_snake_case)]
pub fn compute_user_sensors(m: *const mjModel, d: *mut mjData, stage: u32) {
    // SAFETY: accessing global callback pointer and model/data fields, matching C semantics
    unsafe {
        if let Some(cb) = crate::engine::engine_callback::mjcb_sensor {
            cb(m, d, stage as i32);
        }

        // apply cutoff to user sensors
        let nsensor = (*m).nsensor as i32;
        let mut i: i32 = 0;
        while i < nsensor {
            if *(*m).sensor_type.add(i as usize) == mjtSensor_mjSENS_USER as i32
                && *(*m).sensor_needstage.add(i as usize) == stage as i32
            {
                let adr = *(*m).sensor_adr.add(i as usize);
                apply_cutoff(m, i, (*d).sensordata.add(adr as usize));
            }
            i += 1;
        }
    }
}

/// C: compute_plugin_sensors (engine/engine_sensor.c:1414)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_sensor.c:_apply_cutoff, cxx:_mj_rnePostConstraint, cxx:_mj_subtreeVel, cxx:_mjp_getPluginAtSlotUnsafe, cxx:_mjp_pluginCount, cxx:_mju_message
#[allow(unused_variables, non_snake_case)]
pub fn compute_plugin_sensors(m: *const mjModel, d: *mut mjData, stage: u32) {
    todo!() // cxx-internal:engine_sensor.c.o:_compute_plugin_sensors
}

/// C: mj_sensorPos (engine/engine_sensor.c:1465)
/// Calls: cxx-internal:engine_sensor.c.o:_compute_or_read_sensor, cxx-internal:engine_sensor.c.o:_compute_plugin_sensors, cxx-internal:engine_sensor.c.o:_compute_user_sensors, cxx:_mj_sleepState, cxx:_mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mj_sensor_pos(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx:_mj_sensorPos
}

/// C: mj_sensorVel (engine/engine_sensor.c:1516)
/// Calls: cxx-internal:engine_sensor.c.o:_compute_or_read_sensor, cxx-internal:engine_sensor.c.o:_compute_plugin_sensors, cxx-internal:engine_sensor.c.o:_compute_user_sensors, cxx:_mj_sleepState, cxx:_mj_subtreeVel, cxx:_mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mj_sensor_vel(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx:_mj_sensorVel
}

/// C: mj_sensorAcc (engine/engine_sensor.c:1570)
/// Calls: cxx-internal:engine_sensor.c.o:_compute_or_read_sensor, cxx-internal:engine_sensor.c.o:_compute_plugin_sensors, cxx-internal:engine_sensor.c.o:_compute_user_sensors, cxx:_mj_rnePostConstraint, cxx:_mj_sleepState, cxx:_mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mj_sensor_acc(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx:_mj_sensorAcc
}

/// C: mj_energyPos (engine/engine_sensor.c:1626)
/// Calls: cxx:_mj_sleepState, cxx:_mju_copy4, cxx:_mju_dot3, cxx:_mju_isZero, cxx:_mju_norm3, cxx:_mju_normalize4, cxx:_mju_polyPotential, cxx:_mju_sub3, cxx:_mju_subQuat
#[allow(unused_variables, non_snake_case)]
pub fn mj_energy_pos(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx:_mj_energyPos
}

/// C: mj_energyVel (engine/engine_sensor.c:1733)
/// Calls: cxx:_mj_freeStack, cxx:_mj_markStack, cxx:_mj_mulM, cxx:_mj_stackAllocInfo, cxx:_mju_dot
#[allow(unused_variables, non_snake_case)]
pub fn mj_energy_vel(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx:_mj_energyVel
}

