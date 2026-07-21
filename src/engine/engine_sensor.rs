//! Port of: engine/engine_sensor.c
//! IR hash: 73a9f665ec0ecfc0
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

/// C: get_xquat (engine/engine_sensor.c:257)
/// Calls: mju_copy4, mju_message, mju_mulQuat
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
                crate::engine::engine_util_spatial::mju_mul_quat(
                    quat, (*d).xquat.add(4 * id as usize), (*m).body_iquat.add(4 * id as usize));
            }
            mjtObj_mjOBJ_GEOM => {
                crate::engine::engine_util_spatial::mju_mul_quat(
                    quat,
                    (*d).xquat.add(4 * *(*m).geom_bodyid.add(id as usize) as usize),
                    (*m).geom_quat.add(4 * id as usize));
            }
            mjtObj_mjOBJ_SITE => {
                crate::engine::engine_util_spatial::mju_mul_quat(
                    quat,
                    (*d).xquat.add(4 * *(*m).site_bodyid.add(id as usize) as usize),
                    (*m).site_quat.add(4 * id as usize));
            }
            mjtObj_mjOBJ_CAMERA => {
                crate::engine::engine_util_spatial::mju_mul_quat(
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
pub fn check_match(m: *const mjModel, body: i32, geom: i32, r#type: u32, id: i32) -> i32 {
    // SAFETY: m is valid mjModel pointer (caller contract)
    unsafe {
        if r#type == 0 { return 1; }     // mjOBJ_UNKNOWN
        if r#type == 6 { return 1; }     // mjOBJ_SITE — already passed site filter
        if r#type == 5 { return (id == geom) as i32; }  // mjOBJ_GEOM
        if r#type == 1 { return (id == body) as i32; }  // mjOBJ_BODY
        if r#type == 2 {  // mjOBJ_XBODY
            // traverse up the tree from body, return true if we land on id
            let mut b = body;
            while b > id {
                b = *(*m).body_parentid.add(b as usize);
            }
            return (b == id) as i32;
        }
        0
    }
}

/// C: matchContact (engine/engine_sensor.c:339)
/// Calls: checkMatch, mj_flexBody, mju_insideGeom
#[allow(unused_variables, non_snake_case)]
pub fn match_contact(m: *const mjModel, d: *const mjData, conid: i32, type1: u32, id1: i32, type2: u32, id2: i32) -> i32 {
    // SAFETY: m, d are valid model/data pointers; array accesses are within model bounds
    unsafe {
        // no criterion: quick match
        if type1 == mjtObj_mjOBJ_UNKNOWN && type2 == mjtObj_mjOBJ_UNKNOWN {
            return 1;
        }

        // site filter
        if type1 == mjtObj_mjOBJ_SITE {
            if crate::engine::engine_util_misc::mju_inside_geom(
                (*d).site_xpos.add(3 * id1 as usize),
                (*d).site_xmat.add(9 * id1 as usize),
                (*m).site_size.add(3 * id1 as usize),
                *(*m).site_type.add(id1 as usize) as u32,
                (*d).contact.add(conid as usize).as_ref().unwrap().pos.as_ptr(),
            ) == 0 {
                return 0;
            }
        }

        // get geom, body ids
        let contact = &*(*d).contact.add(conid as usize);
        let geom1 = contact.geom[0];
        let geom2 = contact.geom[1];
        let body1 = if geom1 >= 0 {
            *(*m).geom_bodyid.add(geom1 as usize)
        } else {
            crate::engine::engine_sleep::mj_flex_body(m, contact as *const mjContact, 0)
        };
        let body2 = if geom2 >= 0 {
            *(*m).geom_bodyid.add(geom2 as usize)
        } else {
            crate::engine::engine_sleep::mj_flex_body(m, contact as *const mjContact, 1)
        };

        // check match of sensor objects with contact objects
        let match11 = check_match(m, body1, geom1, type1, id1);
        let match12 = check_match(m, body2, geom2, type1, id1);
        let match21 = check_match(m, body1, geom1, type2, id2);
        let match22 = check_match(m, body2, geom2, type2, id2);

        // if a sensor object is specified, it must be involved in the contact
        if match11 == 0 && match12 == 0 { return 0; }
        if match21 == 0 && match22 == 0 { return 0; }

        // determine direction
        if type1 != mjtObj_mjOBJ_UNKNOWN && type2 != mjtObj_mjOBJ_UNKNOWN {
            let order_regular = (match11 != 0 && match22 != 0) as i32;
            let order_reverse = (match12 != 0 && match21 != 0) as i32;
            if order_regular != 0 && order_reverse == 0 { return 1; }
            if order_reverse != 0 && order_regular == 0 { return -1; }
            if order_regular != 0 && order_reverse != 0 { return 1; }
        } else if type1 != mjtObj_mjOBJ_UNKNOWN {
            return if match11 != 0 { 1 } else { -1 };
        } else if type2 != mjtObj_mjOBJ_UNKNOWN {
            return if match22 != 0 { 1 } else { -1 };
        }

        0
    }
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
    use crate::engine::engine_util_blas::{mju_copy3, mju_scl3};
    use crate::engine::engine_core_util::mj_contact_force;

    const mjCONDATA_FOUND: usize = 0;
    const mjCONDATA_FORCE: usize = 1;
    const mjCONDATA_TORQUE: usize = 2;
    const mjCONDATA_DIST: usize = 3;
    const mjCONDATA_POS: usize = 4;
    const mjCONDATA_NORMAL: usize = 5;
    const mjCONDATA_TANGENT: usize = 6;

    // SAFETY: all pointers valid per caller contract; data is array of mjNCONDATA (7) pointers
    unsafe {
        // found flag
        if !(*data.add(mjCONDATA_FOUND)).is_null() {
            *(*data.add(mjCONDATA_FOUND)) = nfound as f64;
        }

        // contact force and torque
        if !(*data.add(mjCONDATA_FORCE)).is_null() || !(*data.add(mjCONDATA_TORQUE)).is_null() {
            let mut forcetorque: [f64; 6] = [0.0; 6];
            mj_contact_force(m, d, id, forcetorque.as_mut_ptr());
            if !(*data.add(mjCONDATA_FORCE)).is_null() {
                mju_copy3(*data.add(mjCONDATA_FORCE), forcetorque.as_ptr());
                if flg_flip != 0 {
                    *(*data.add(mjCONDATA_FORCE)).add(2) *= -1.0;
                }
            }
            if !(*data.add(mjCONDATA_TORQUE)).is_null() {
                mju_copy3(*data.add(mjCONDATA_TORQUE), forcetorque.as_ptr().add(3));
                if flg_flip != 0 {
                    *(*data.add(mjCONDATA_TORQUE)).add(2) *= -1.0;
                }
            }
        }

        // contact penetration distance
        if !(*data.add(mjCONDATA_DIST)).is_null() {
            *(*data.add(mjCONDATA_DIST)) = (*(*d).contact.add(id as usize)).dist;
        }

        // contact position
        if !(*data.add(mjCONDATA_POS)).is_null() {
            mju_copy3(*data.add(mjCONDATA_POS), (*(*d).contact.add(id as usize)).pos.as_ptr());
        }

        // contact normal
        if !(*data.add(mjCONDATA_NORMAL)).is_null() {
            mju_copy3(*data.add(mjCONDATA_NORMAL), (*(*d).contact.add(id as usize)).frame.as_ptr());
            if flg_flip != 0 {
                mju_scl3(*data.add(mjCONDATA_NORMAL), *data.add(mjCONDATA_NORMAL), -1.0);
            }
        }

        // contact first tangent
        if !(*data.add(mjCONDATA_TANGENT)).is_null() {
            mju_copy3(*data.add(mjCONDATA_TANGENT), (*(*d).contact.add(id as usize)).frame.as_ptr().add(3));
            if flg_flip != 0 {
                mju_scl3(*data.add(mjCONDATA_TANGENT), *data.add(mjCONDATA_TANGENT), -1.0);
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
    use crate::engine::engine_util_blas::{mju_copy3, mju_zero3, mju_add_scl3, mju_sub3, mju_dot3};

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
                mju_add_scl3(point.as_mut_ptr(), origin, direction, dist);
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

