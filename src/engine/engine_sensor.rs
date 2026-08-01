//! Port of: engine/engine_sensor.c
//! IR hash: 9343293228317031
//! CODEGEN: source paths, owners, and callable names are locked.

use crate::types::*;

/// C: ContactInfoCompare (engine/engine_sensor.c:52)
#[allow(unused_variables, non_snake_case)]
pub fn ContactInfoCompare(a: *const ContactInfo, b: *const ContactInfo, context: *mut ()) -> i32 {
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

/// C: apply_cutoff (engine/engine_sensor.c:198)
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

/// C: get_xpos_xmat (engine/engine_sensor.c:227)
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

/// C: get_xquat (engine/engine_sensor.c:257)
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

/// C: cam_project (engine/engine_sensor.c:281)
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

/// C: checkMatch (engine/engine_sensor.c:320)
#[allow(unused_variables, non_snake_case)]
pub fn checkMatch(m: *const mjModel, body: i32, geom: i32, r#type: u32, id: i32) -> i32 {
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
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_sensor.c:_checkMatch, cxx:_mj_flexBody, cxx:_mju_insideGeom
#[allow(unused_variables, non_snake_case)]
pub fn matchContact(m: *const mjModel, d: *const mjData, conid: i32, type1: u32, id1: i32, type2: u32, id2: i32) -> i32 {
    // SAFETY: m, d are valid model/data pointers; array accesses are within model bounds
    unsafe {
        // no criterion: quick match
        if type1 == mjtObj_mjOBJ_UNKNOWN && type2 == mjtObj_mjOBJ_UNKNOWN {
            return 1;
        }

        // site filter
        if type1 == mjtObj_mjOBJ_SITE {
            if crate::engine::engine_util_misc::mju_insideGeom(
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
            crate::engine::engine_sleep::mj_flexBody(m, contact as *const mjContact, 0)
        };
        let body2 = if geom2 >= 0 {
            *(*m).geom_bodyid.add(geom2 as usize)
        } else {
            crate::engine::engine_sleep::mj_flexBody(m, contact as *const mjContact, 1)
        };

        // check match of sensor objects with contact objects
        let match11 = checkMatch(m, body1, geom1, type1, id1);
        let match12 = checkMatch(m, body2, geom2, type1, id1);
        let match21 = checkMatch(m, body1, geom1, type2, id2);
        let match22 = checkMatch(m, body2, geom2, type2, id2);

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
/// Calls: cxx:_mj_contactForce, cxx:_mju_copy3, cxx:_mju_scl3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn copySensorData(m: *const mjModel, d: *const mjData, data: *mut *mut f64, id: i32, flg_flip: i32, nfound: i32) {
    use crate::engine::engine_util_blas::{mju_copy3, mju_scl3};
    use crate::engine::engine_core_util::mj_contactForce;

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
            mj_contactForce(m, d, id, forcetorque.as_mut_ptr());
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

/// C: fill_raydata (engine/engine_sensor.c:470)
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

/// C: mj_computeSensorVel (engine/engine_sensor.c:839)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_sensor.c:_get_xpos_xmat, cxx:_mj_objectVelocity, cxx:_mj_subtreeVel, cxx:_mju_addTo3, cxx:_mju_copy3, cxx:_mju_cross, cxx:_mju_message, cxx:_mju_mulMatTVec3, cxx:_mju_sub, cxx:_mju_sub3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_computeSensorVel(m: *const mjModel, d: *mut mjData, i: i32, sensordata: *mut f64) {
    // Sensor type enum values
    const mjSENS_VELOCIMETER: i32 = 2;
    const mjSENS_GYRO: i32 = 3;
    const mjSENS_JOINTVEL: i32 = 10;
    const mjSENS_TENDONVEL: i32 = 12;
    const mjSENS_ACTUATORVEL: i32 = 14;
    const mjSENS_BALLANGVEL: i32 = 19;
    const mjSENS_JOINTLIMITVEL: i32 = 21;
    const mjSENS_TENDONLIMITVEL: i32 = 24;
    const mjSENS_FRAMELINVEL: i32 = 31;
    const mjSENS_FRAMEANGVEL: i32 = 32;
    const mjSENS_SUBTREELINVEL: i32 = 36;
    const mjSENS_SUBTREEANGMOM: i32 = 37;
    const mjCNSTR_LIMIT_JOINT: i32 = 3;
    const mjCNSTR_LIMIT_TENDON: i32 = 4;
    const mjOBJ_SITE: i32 = 6;

    // SAFETY: m, d are valid pointers. sensordata is writable output buffer (caller contract).
    unsafe {
        let ne = (*d).ne;
        let nf = (*d).nf;
        let nefc = (*d).nefc;
        let sensor_type = *(*m).sensor_type.add(i as usize);
        let objtype = *(*m).sensor_objtype.add(i as usize);
        let objid = *(*m).sensor_objid.add(i as usize);
        let refid = *(*m).sensor_refid.add(i as usize);
        let reftype = *(*m).sensor_reftype.add(i as usize);

        let mut xvel: [f64; 6] = [0.0; 6];

        // call mj_subtreeVel for sensors that need it
        if !(*d).flg_subtreevel
            && (sensor_type == mjSENS_SUBTREELINVEL || sensor_type == mjSENS_SUBTREEANGMOM)
        {
            crate::engine::engine_core_smooth::mj_subtreeVel(m, d);
        }

        // process according to type
        if sensor_type == mjSENS_VELOCIMETER {
            crate::engine::engine_core_util::mj_objectVelocity(
                m, d as *const crate::types::mjData, mjOBJ_SITE, objid, xvel.as_mut_ptr(), 1);
            crate::engine::engine_util_blas::mju_copy3(sensordata, xvel.as_ptr().add(3));
        } else if sensor_type == mjSENS_GYRO {
            crate::engine::engine_core_util::mj_objectVelocity(
                m, d as *const crate::types::mjData, mjOBJ_SITE, objid, xvel.as_mut_ptr(), 1);
            crate::engine::engine_util_blas::mju_copy3(sensordata, xvel.as_ptr());
        } else if sensor_type == mjSENS_JOINTVEL {
            *sensordata = *(*d).qvel.add(*(*m).jnt_dofadr.add(objid as usize) as usize);
        } else if sensor_type == mjSENS_TENDONVEL {
            *sensordata = *(*d).ten_velocity.add(objid as usize);
        } else if sensor_type == mjSENS_ACTUATORVEL {
            *sensordata = *(*d).actuator_velocity.add(objid as usize);
        } else if sensor_type == mjSENS_BALLANGVEL {
            crate::engine::engine_util_blas::mju_copy3(
                sensordata, (*d).qvel.add(*(*m).jnt_dofadr.add(objid as usize) as usize));
        } else if sensor_type == mjSENS_JOINTLIMITVEL {
            *sensordata = 0.0;
            for j in (ne + nf)..nefc {
                if *(*d).efc_type.add(j as usize) == mjCNSTR_LIMIT_JOINT
                    && *(*d).efc_id.add(j as usize) == objid
                {
                    *sensordata = *(*d).efc_vel.add(j as usize);
                    break;
                }
            }
        } else if sensor_type == mjSENS_TENDONLIMITVEL {
            *sensordata = 0.0;
            for j in (ne + nf)..nefc {
                if *(*d).efc_type.add(j as usize) == mjCNSTR_LIMIT_TENDON
                    && *(*d).efc_id.add(j as usize) == objid
                {
                    *sensordata = *(*d).efc_vel.add(j as usize);
                    break;
                }
            }
        } else if sensor_type == mjSENS_FRAMELINVEL || sensor_type == mjSENS_FRAMEANGVEL {
            // xvel = 6D object velocity, in global frame
            crate::engine::engine_core_util::mj_objectVelocity(
                m, d as *const crate::types::mjData, objtype, objid, xvel.as_mut_ptr(), 0);

            if refid > -1 {
                let mut xpos: *mut f64 = std::ptr::null_mut();
                let mut xmat: *mut f64 = std::ptr::null_mut();
                let mut xpos_ref: *mut f64 = std::ptr::null_mut();
                let mut xmat_ref: *mut f64 = std::ptr::null_mut();
                let mut xvel_ref: [f64; 6] = [0.0; 6];
                let mut rel_vel: [f64; 6] = [0.0; 6];
                let mut cross: [f64; 3] = [0.0; 3];
                let mut rvec: [f64; 3] = [0.0; 3];

                // get positions and orientations
                get_xpos_xmat(d as *const crate::types::mjData, objtype as u32, objid, i,
                    &mut xpos as *mut *mut f64, &mut xmat as *mut *mut f64);
                get_xpos_xmat(d as *const crate::types::mjData, reftype as u32, refid, i,
                    &mut xpos_ref as *mut *mut f64, &mut xmat_ref as *mut *mut f64);
                crate::engine::engine_core_util::mj_objectVelocity(
                    m, d as *const crate::types::mjData, reftype, refid, xvel_ref.as_mut_ptr(), 0);

                // subtract velocities
                crate::engine::engine_util_blas::mju_sub(
                    rel_vel.as_mut_ptr(), xvel.as_ptr(), xvel_ref.as_ptr(), 6);

                // linear velocity: add correction due to rotating reference frame
                crate::engine::engine_util_blas::mju_sub3(rvec.as_mut_ptr(), xpos as *const f64, xpos_ref as *const f64);
                crate::engine::engine_util_spatial::mju_cross(cross.as_mut_ptr(), rvec.as_ptr(), xvel_ref.as_ptr());
                crate::engine::engine_util_blas::mju_addTo3(rel_vel.as_mut_ptr().add(3), cross.as_ptr());

                // project into reference frame
                crate::engine::engine_util_blas::mju_mulMatTVec3(xvel.as_mut_ptr(), xmat_ref as *const f64, rel_vel.as_ptr());
                crate::engine::engine_util_blas::mju_mulMatTVec3(xvel.as_mut_ptr().add(3), xmat_ref as *const f64, rel_vel.as_ptr().add(3));
            }

            // copy linear or angular component
            if sensor_type == mjSENS_FRAMELINVEL {
                crate::engine::engine_util_blas::mju_copy3(sensordata, xvel.as_ptr().add(3));
            } else {
                crate::engine::engine_util_blas::mju_copy3(sensordata, xvel.as_ptr());
            }
        } else if sensor_type == mjSENS_SUBTREELINVEL {
            crate::engine::engine_util_blas::mju_copy3(
                sensordata, (*d).subtree_linvel.add((3 * objid) as usize) as *const f64);
        } else if sensor_type == mjSENS_SUBTREEANGMOM {
            crate::engine::engine_util_blas::mju_copy3(
                sensordata, (*d).subtree_angmom.add((3 * objid) as usize) as *const f64);
        } else {
            crate::engine::engine_util_errmem::mju_error(
                b"invalid type in VEL stage\0".as_ptr() as *const i8);
        }
    }
}

/// C: compute_user_sensors (engine/engine_sensor.c:1432)
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

/// C: mj_energyPos (engine/engine_sensor.h:44)
/// Calls: cxx:_mj_sleepState, cxx:_mju_copy4, cxx:_mju_dot3, cxx:_mju_isZero, cxx:_mju_norm3, cxx:_mju_normalize4, cxx:_mju_polyPotential, cxx:_mju_sub3, cxx:_mju_subQuat
#[allow(unused_variables, non_snake_case)]
pub fn mj_energyPos(m: *const mjModel, d: *mut mjData) {
    const mjDSBL_GRAVITY: i32 = 1 << 7;
    const mjDSBL_SPRING: i32 = 1 << 5;
    const mjENBL_SLEEP: i32 = 1 << 4;
    const mjJNT_FREE: i32 = 0;
    const mjJNT_BALL: i32 = 1;
    const mjJNT_SLIDE: i32 = 2;
    const mjJNT_HINGE: i32 = 3;
    const mjS_AWAKE: i32 = 1;
    const mjNPOLY: i32 = 2;
    const mjOBJ_TENDON: u32 = 18;

    // SAFETY: m, d are valid pointers (caller contract).
    unsafe {
        let mut dif: [f64; 3] = [0.0; 3];
        let mut quat: [f64; 4] = [0.0; 4];

        // init potential energy: -sum_i body(i).mass * mju_dot(body(i).pos, gravity)
        (*d).energy[0] = 0.0;
        if ((*m).opt.disableflags & mjDSBL_GRAVITY) == 0 {
            for i in 1..(*m).nbody as i32 {
                (*d).energy[0] -= *(*m).body_mass.add(i as usize)
                    * crate::engine::engine_util_blas::mju_dot3(
                        (*m).opt.gravity.as_ptr(),
                        (*d).xipos.add((3 * i) as usize));
            }
        }

        let sleep_filter = (((*m).opt.enableflags & mjENBL_SLEEP) != 0)
            && ((*d).nbody_awake < (*m).nbody as i32);

        // add joint-level springs
        if ((*m).opt.disableflags & mjDSBL_SPRING) == 0 {
            let nbody = (*m).nbody as i32;
            for b in 1..nbody {
                if sleep_filter && *(*d).body_awake.add(b as usize) != mjS_AWAKE {
                    continue;
                }

                let jnt_start = *(*m).body_jntadr.add(b as usize);
                let jnt_end = jnt_start + *(*m).body_jntnum.add(b as usize);
                for j in jnt_start..jnt_end {
                    let stiffness = *(*m).jnt_stiffness.add(j as usize);
                    let poly: *const f64 = (*m).jnt_stiffnesspoly.add((mjNPOLY * j) as usize);
                    if stiffness == 0.0
                        && crate::engine::engine_util_misc::mju_isZero(poly, mjNPOLY) != 0
                    {
                        continue;
                    }
                    let mut padr = *(*m).jnt_qposadr.add(j as usize);

                    let jnt_type = *(*m).jnt_type.add(j as usize);

                    if jnt_type == mjJNT_FREE {
                        crate::engine::engine_util_blas::mju_sub3(
                            dif.as_mut_ptr(),
                            (*d).qpos.add(padr as usize),
                            (*m).qpos_spring.add(padr as usize));
                        let x = crate::engine::engine_util_blas::mju_norm3(dif.as_ptr());
                        (*d).energy[0] += crate::engine::engine_util_misc::mju_polyPotential(
                            stiffness, poly, x, mjNPOLY, 0);
                        padr += 3;
                        // fallthrough to BALL case
                        crate::engine::engine_util_blas::mju_copy4(
                            quat.as_mut_ptr(), (*d).qpos.add(padr as usize));
                        crate::engine::engine_util_blas::mju_normalize4(quat.as_mut_ptr());
                        crate::engine::engine_util_spatial::mju_subQuat(
                            dif.as_mut_ptr(),
                            (*d).qpos.add(padr as usize),
                            (*m).qpos_spring.add(padr as usize));
                        let x = crate::engine::engine_util_blas::mju_norm3(dif.as_ptr());
                        (*d).energy[0] += crate::engine::engine_util_misc::mju_polyPotential(
                            stiffness, poly, x, mjNPOLY, 0);
                    } else if jnt_type == mjJNT_BALL {
                        crate::engine::engine_util_blas::mju_copy4(
                            quat.as_mut_ptr(), (*d).qpos.add(padr as usize));
                        crate::engine::engine_util_blas::mju_normalize4(quat.as_mut_ptr());
                        crate::engine::engine_util_spatial::mju_subQuat(
                            dif.as_mut_ptr(),
                            (*d).qpos.add(padr as usize),
                            (*m).qpos_spring.add(padr as usize));
                        let x = crate::engine::engine_util_blas::mju_norm3(dif.as_ptr());
                        (*d).energy[0] += crate::engine::engine_util_misc::mju_polyPotential(
                            stiffness, poly, x, mjNPOLY, 0);
                    } else if jnt_type == mjJNT_SLIDE || jnt_type == mjJNT_HINGE {
                        let x = *(*d).qpos.add(padr as usize)
                            - *(*m).qpos_spring.add(padr as usize);
                        (*d).energy[0] += crate::engine::engine_util_misc::mju_polyPotential(
                            stiffness, poly, x, mjNPOLY, 0);
                    }
                }
            }
        }

        // add tendon-level springs
        if ((*m).opt.disableflags & mjDSBL_SPRING) == 0 {
            for i in 0..(*m).ntendon as i32 {
                // skip sleeping or static tendon
                if sleep_filter
                    && crate::engine::engine_sleep::mj_sleepState(
                        m, d as *const crate::types::mjData, mjOBJ_TENDON, i)
                        != mjS_AWAKE
                {
                    continue;
                }

                let stiffness = *(*m).tendon_stiffness.add(i as usize);
                let poly: *const f64 = (*m).tendon_stiffnesspoly.add((mjNPOLY * i) as usize);
                let length = *(*d).ten_length.add(i as usize);

                // compute spring displacement x
                let lower = *(*m).tendon_lengthspring.add((2 * i) as usize);
                let upper = *(*m).tendon_lengthspring.add((2 * i + 1) as usize);
                let x = if length > upper {
                    length - upper
                } else if length < lower {
                    length - lower
                } else {
                    0.0
                };

                // add potential energy
                (*d).energy[0] += crate::engine::engine_util_misc::mju_polyPotential(
                    stiffness, poly, x, mjNPOLY, 0);
            }
        }

        // add flex-level springs for dim=1
        if ((*m).opt.disableflags & mjDSBL_SPRING) == 0 {
            for i in 0..(*m).nflex as i32 {
                let stiffness = *(*m).flex_edgestiffness.add(i as usize);
                if *(*m).flex_rigid.add(i as usize)
                    || stiffness == 0.0
                    || *(*m).flex_dim.add(i as usize) > 1
                {
                    continue;
                }

                // process non-rigid edges of this flex
                let flex_edgeadr = *(*m).flex_edgeadr.add(i as usize);
                let flex_edgenum = *(*m).flex_edgenum.add(i as usize);
                for e in flex_edgeadr..(flex_edgeadr + flex_edgenum) {
                    if !*(*m).flexedge_rigid.add(e as usize) {
                        let displacement = *(*m).flexedge_length0.add(e as usize)
                            - *(*d).flexedge_length.add(e as usize);
                        (*d).energy[0] += 0.5 * stiffness * displacement * displacement;
                    }
                }
            }
        }

        // mark as computed
        (*d).flg_energypos = true;
    }
}

/// C: mj_energyVel (engine/engine_sensor.h:47)
/// Calls: cxx:_mj_freeStack, cxx:_mj_markStack, cxx:_mj_mulM, cxx:_mj_stackAllocInfo, cxx:_mju_dot
#[allow(unused_variables, non_snake_case)]
pub fn mj_energyVel(m: *const mjModel, d: *mut mjData) {
    // SAFETY: m, d are valid pointers (caller contract).
    unsafe {
        crate::engine::engine_memory::mj_markStack(d);
        let vec = crate::engine::engine_memory::mj_stackAllocNum(d, (*m).nv as usize);

        // kinetic energy: 0.5 * qvel' * M * qvel
        crate::engine::engine_support::mj_mulM(m, d as *const mjData, vec, (*d).qvel);
        (*d).energy[1] = 0.5 * crate::engine::engine_util_blas::mju_dot(
            vec, (*d).qvel, (*m).nv as i32);

        crate::engine::engine_memory::mj_freeStack(d);

        // mark as computed
        (*d).flg_energyvel = true;
    }
}

