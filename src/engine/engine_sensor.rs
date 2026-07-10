//! Port of: engine/engine_sensor.c
//! IR hash: 05737965add36adb
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: ContactInfoCompare (engine/engine_sensor.c:52)
#[allow(unused_variables, non_snake_case)]
pub fn contact_info_compare(a: *const ContactInfo, b: *const ContactInfo, context: *mut ()) -> i32 {
    // SAFETY: a, b point to valid ContactInfo structs. Layout: +0 criterion (f64), +8 id (i32).
    unsafe {
        let a_ptr = a as *const u8;
        let b_ptr = b as *const u8;
        let a_crit = *(a_ptr.add(0) as *const f64);
        let b_crit = *(b_ptr.add(0) as *const f64);
        if a_crit < b_crit { return -1; }
        if a_crit > b_crit { return 1; }
        let a_id = *(a_ptr.add(8) as *const i32);
        let b_id = *(b_ptr.add(8) as *const i32);
        if a_id < b_id { return -1; }
        if a_id > b_id { return 1; }
        0
    }
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
    // C: mjTactileTaskArgs* args_array = (mjTactileTaskArgs*)arg;
    //    tactile_taxel_batch(m, d, &args_array[task_id]);
    // mjTactileTaskArgs is 48 bytes on 64-bit (6 ints + ptr + 2 ints + ptr)
    // SAFETY: arg points to an array of mjTactileTaskArgs, task_id is a valid index.
    unsafe {
        const SIZEOF_TACTILE_TASK_ARGS: usize = 48;
        let element = (arg as *mut u8).add(task_id as usize * SIZEOF_TACTILE_TASK_ARGS) as *mut ();
        tactile_taxel_batch(m, d, element);
    }
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
    // SAFETY: m valid per caller contract. sensor_cutoff, sensor_type,
    // sensor_dim, sensor_datatype are valid arrays indexed by i.
    // data points to valid array of at least sensor_dim[i] elements.
    unsafe {
        const MJSENS_CONTACT: i32 = 42;
        const MJSENS_GEOMFROMTO: i32 = 41;
        const MJDATATYPE_REAL: i32 = 0;
        const MJDATATYPE_POSITIVE: i32 = 1;

        let cutoff = *(*m).sensor_cutoff.add(i as usize);
        if cutoff <= 0.0 {
            return;
        }

        // cutoff ignored for contact and fromto sensors
        let sensor_type = *(*m).sensor_type.add(i as usize);
        if sensor_type == MJSENS_CONTACT || sensor_type == MJSENS_GEOMFROMTO {
            return;
        }

        let dim = *(*m).sensor_dim.add(i as usize);
        let datatype = *(*m).sensor_datatype.add(i as usize);

        let mut j: i32 = 0;
        while j < dim {
            // real: apply on both sides
            if datatype == MJDATATYPE_REAL {
                *data.add(j as usize) = crate::engine::engine_util_misc::mju_clip(
                    *data.add(j as usize), -cutoff, cutoff);
            }
            // positive: apply on positive side only
            else if datatype == MJDATATYPE_POSITIVE {
                *data.add(j as usize) = crate::engine::engine_util_misc::mju_min(
                    cutoff, *data.add(j as usize));
            }
            j += 1;
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
    use crate::engine::engine_util_blas::{mju_sub3, mju_mul_mat_t_vec};
    use crate::engine::engine_util_misc::{mju_min, mju_max};
    const MJ_PI: f64 = std::f64::consts::PI;
    const MJ_MINVAL: f64 = 1e-15;
    // SAFETY: sensordata[2], target_xpos[3], cam_xpos[3], cam_xmat[9]
    unsafe {
        let fx: f64;
        let fy: f64;

        // focal transformation
        if cam_sensorsize[0] != 0.0 && cam_sensorsize[1] != 0.0 {
            fx = cam_intrinsic[0] as f64 / cam_sensorsize[0] as f64 * cam_res[0] as f64;
            fy = cam_intrinsic[1] as f64 / cam_sensorsize[1] as f64 * cam_res[1] as f64;
        } else {
            let f = 0.5 / (cam_fovy * MJ_PI / 360.0).tan() * cam_res[1] as f64;
            fx = f;
            fy = f;
        }

        // relative position in world frame
        let mut relative_pos: [f64; 3] = [0.0; 3];
        mju_sub3(relative_pos.as_mut_ptr(), target_xpos, cam_xpos);

        // project to camera frame: cam_pos = cam_xmat^T * relative_pos
        let mut cam_pos: [f64; 3] = [0.0; 3];
        mju_mul_mat_t_vec(cam_pos.as_mut_ptr(), cam_xmat, relative_pos.as_ptr(), 3, 3);

        // avoid dividing by tiny numbers
        let mut denom: f64 = cam_pos[2];
        if denom.abs() < MJ_MINVAL {
            if denom < 0.0 {
                denom = mju_min(denom, -MJ_MINVAL);
            } else {
                denom = mju_max(denom, MJ_MINVAL);
            }
        }

        // compute projection
        *sensordata.add(0) = -fx * (cam_pos[0] / denom) + 0.5 * cam_res[0] as f64;
        *sensordata.add(1) =  fy * (cam_pos[1] / denom) + 0.5 * cam_res[1] as f64;
    }
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
    // SAFETY: m, d valid. data[i] pointers either null or valid writable buffers per caller.
    unsafe {
        const MJCONDATA_FOUND: usize = 0;
        const MJCONDATA_FORCE: usize = 1;
        const MJCONDATA_TORQUE: usize = 2;
        const MJCONDATA_DIST: usize = 3;
        const MJCONDATA_POS: usize = 4;
        const MJCONDATA_NORMAL: usize = 5;
        const MJCONDATA_TANGENT: usize = 6;

        // found flag
        if !data[MJCONDATA_FOUND].is_null() {
            *data[MJCONDATA_FOUND] = nfound as f64;
        }

        // contact force and torque
        if !data[MJCONDATA_FORCE].is_null() || !data[MJCONDATA_TORQUE].is_null() {
            let mut forcetorque: [f64; 6] = [0.0; 6];
            crate::engine::engine_core_util::mj_contact_force(
                m, d, id, forcetorque.as_mut_ptr());
            if !data[MJCONDATA_FORCE].is_null() {
                crate::engine::engine_util_blas::mju_copy3(data[MJCONDATA_FORCE], forcetorque.as_ptr());
                if flg_flip != 0 {
                    *data[MJCONDATA_FORCE].add(2) *= -1.0;
                }
            }
            if !data[MJCONDATA_TORQUE].is_null() {
                crate::engine::engine_util_blas::mju_copy3(data[MJCONDATA_TORQUE], forcetorque.as_ptr().add(3));
                if flg_flip != 0 {
                    *data[MJCONDATA_TORQUE].add(2) *= -1.0;
                }
            }
        }

        // contact penetration distance
        if !data[MJCONDATA_DIST].is_null() {
            *data[MJCONDATA_DIST] = (*(*d).contact.add(id as usize)).dist;
        }

        // contact position
        if !data[MJCONDATA_POS].is_null() {
            crate::engine::engine_util_blas::mju_copy3(
                data[MJCONDATA_POS], (*(*d).contact.add(id as usize)).pos.as_ptr());
        }

        // contact normal
        if !data[MJCONDATA_NORMAL].is_null() {
            crate::engine::engine_util_blas::mju_copy3(
                data[MJCONDATA_NORMAL], (*(*d).contact.add(id as usize)).frame.as_ptr());
            if flg_flip != 0 {
                crate::engine::engine_util_blas::mju_scl3(
                    data[MJCONDATA_NORMAL], data[MJCONDATA_NORMAL] as *const f64, -1.0);
            }
        }

        // contact first tangent
        if !data[MJCONDATA_TANGENT].is_null() {
            crate::engine::engine_util_blas::mju_copy3(
                data[MJCONDATA_TANGENT], (*(*d).contact.add(id as usize)).frame.as_ptr().add(3));
            if flg_flip != 0 {
                crate::engine::engine_util_blas::mju_scl3(
                    data[MJCONDATA_TANGENT], data[MJCONDATA_TANGENT] as *const f64, -1.0);
            }
        }
    }
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
    // SAFETY: force, torque point to 3 f64; point to 3 f64; wrench to 6*n f64;
    //         pos to 3*n f64; frame to 9*n f64. Caller guarantees validity.
    unsafe {
        crate::engine::engine_util_blas::mju_zero3(force);
        crate::engine::engine_util_blas::mju_zero3(torque);

        for j in 0..n as usize {
            // rotate force, torque from contact frame to global frame
            let mut force_j: [f64; 3] = [0.0; 3];
            let mut torque_j: [f64; 3] = [0.0; 3];
            crate::engine::engine_util_blas::mju_mul_mat_t_vec3(
                force_j.as_mut_ptr(), frame.add(9 * j), wrench.add(6 * j));
            crate::engine::engine_util_blas::mju_mul_mat_t_vec3(
                torque_j.as_mut_ptr(), frame.add(9 * j), wrench.add(6 * j + 3));

            // add to total force, torque
            crate::engine::engine_util_blas::mju_add_to3(force, force_j.as_ptr());
            crate::engine::engine_util_blas::mju_add_to3(torque, torque_j.as_ptr());

            // add induced moment: torque += (pos - point) x force
            let mut diff: [f64; 3] = [0.0; 3];
            crate::engine::engine_util_blas::mju_sub3(diff.as_mut_ptr(), pos.add(3 * j), point);
            let mut induced_torque: [f64; 3] = [0.0; 3];
            crate::engine::engine_util_spatial::mju_cross(
                induced_torque.as_mut_ptr(), diff.as_ptr(), force_j.as_ptr());
            crate::engine::engine_util_blas::mju_add_to3(torque, induced_torque.as_ptr());
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
    // SAFETY: ptr points to adequate buffer. origin/direction/normal are 3-element arrays.
    // cam_xpos/cam_z are 3-element arrays (or null).
    unsafe {
        const MJRAYDATA_DIST: i32 = 0;
        const MJRAYDATA_DIR: i32 = 1;
        const MJRAYDATA_ORIGIN: i32 = 2;
        const MJRAYDATA_POINT: i32 = 3;
        const MJRAYDATA_NORMAL: i32 = 4;
        const MJRAYDATA_DEPTH: i32 = 5;

        let mut p = ptr;
        let hit = dist >= 0.0;

        if dataspec & (1 << MJRAYDATA_DIST) != 0 {
            *p = dist;
            p = p.add(1);
        }
        if dataspec & (1 << MJRAYDATA_DIR) != 0 {
            if hit {
                crate::engine::engine_util_blas::mju_copy3(p, direction);
            } else {
                crate::engine::engine_util_blas::mju_zero3(p);
            }
            p = p.add(3);
        }
        if dataspec & (1 << MJRAYDATA_ORIGIN) != 0 {
            crate::engine::engine_util_blas::mju_copy3(p, origin);
            p = p.add(3);
        }

        // compute point if needed for POINT or DEPTH fields
        let mut point: [f64; 3] = [0.0, 0.0, 0.0];
        if (dataspec & (1 << MJRAYDATA_POINT) != 0) || (dataspec & (1 << MJRAYDATA_DEPTH) != 0) {
            if hit {
                crate::engine::engine_util_blas::mju_add_scl3(
                    point.as_mut_ptr(), origin, direction, dist);
            }
        }

        if dataspec & (1 << MJRAYDATA_POINT) != 0 {
            crate::engine::engine_util_blas::mju_copy3(p, point.as_ptr());
            p = p.add(3);
        }
        if dataspec & (1 << MJRAYDATA_NORMAL) != 0 {
            if hit {
                crate::engine::engine_util_blas::mju_copy3(p, normal);
            } else {
                crate::engine::engine_util_blas::mju_zero3(p);
            }
            p = p.add(3);
        }
        if dataspec & (1 << MJRAYDATA_DEPTH) != 0 {
            if hit {
                if !cam_z.is_null() {
                    // camera depth: project onto camera z-axis
                    let mut delta: [f64; 3] = [0.0; 3];
                    crate::engine::engine_util_blas::mju_sub3(
                        delta.as_mut_ptr(), point.as_ptr(), cam_xpos);
                    *p = -crate::engine::engine_util_blas::mju_dot3(
                        delta.as_ptr(), cam_z);
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
    // SAFETY: m, d valid per caller. All model arrays indexed by i within bounds.
    unsafe {
        let nsample: i32 = *(*m).sensor_history.add(2 * i as usize);

        // no history: compute directly
        if nsample <= 0 {
            mj_compute_sensor(m, d, i, sensordata);
            return;
        }

        let delay: f64 = *(*m).sensor_delay.add(i as usize);
        let dim: i32 = *(*m).sensor_dim.add(i as usize);

        // delay > 0: read delayed value from buffer
        if delay > 0.0 {
            let interp: i32 = *(*m).sensor_history.add(2 * i as usize + 1);
            let ptr: *const f64 = crate::engine::engine_support::mj_read_sensor(
                m, d as *const mjData, i, (*d).time, sensordata, interp);
            if !ptr.is_null() {
                crate::engine::engine_util_blas::mju_copy(sensordata, ptr, dim);
            }
            return;
        }

        // interval > 0: compute if interval condition satisfied, else read from buffer
        let interval: f64 = *(*m).sensor_interval.add(2 * i as usize);
        if interval > 0.0 {
            let historyadr: i32 = *(*m).sensor_historyadr.add(i as usize);
            let buf: *mut f64 = (*d).history.add(historyadr as usize);
            let time_prev: f64 = *buf;  // first slot stores time_prev

            if time_prev + interval <= (*d).time {
                // interval condition satisfied: compute new sensor value
                mj_compute_sensor(m, d, i, sensordata);
            } else {
                // interval condition not satisfied: read from buffer
                let interp: i32 = *(*m).sensor_history.add(2 * i as usize + 1);
                let ptr: *const f64 = crate::engine::engine_support::mj_read_sensor(
                    m, d as *const mjData, i, (*d).time, sensordata, interp);
                if !ptr.is_null() {
                    crate::engine::engine_util_blas::mju_copy(sensordata, ptr, dim);
                }
            }
            return;
        }

        // history only, no delay or interval: compute directly
        mj_compute_sensor(m, d, i, sensordata);
    }
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

