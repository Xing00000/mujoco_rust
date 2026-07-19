//! Port of: engine/engine_support.c
//! IR hash: adc2f24e872d94f7
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mj_stateElemSize (engine/engine_support.c:138)
/// Calls: mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_state_elem_size(m: *const mjModel, sig: u32) -> i32 {
    todo!() // mj_stateElemSize
}

/// C: mj_stateElemPtr (engine/engine_support.c:162)
/// Calls: mju_message
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_state_elem_ptr(m: *const mjModel, d: *mut mjData, sig: u32) -> *mut f64 {
    todo!() // mj_stateElemPtr
}

/// C: mj_stateElemConstPtr (engine/engine_support.c:184)
/// Calls: mj_stateElemPtr
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_state_elem_const_ptr(m: *const mjModel, d: *const mjData, sig: u32) -> *const f64 {
    todo!() // mj_stateElemConstPtr
}

/// C: mj_geomDistanceCCD (engine/engine_support.c:519)
/// Calls: mj_freeStack, mj_markStack, mj_stackAllocByte, mjc_ccd, mjc_ccdSize, mjc_initCCDObj, mju_copy3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_geom_distance_ccd(m: *const mjModel, d: *mut mjData, g1: i32, g2: i32, distmax: f64, fromto: *mut f64) -> f64 {
    todo!() // mj_geomDistanceCCD
}

/// C: mj_stateSize (engine/engine_support.h:41)
/// Calls: mj_stateElemSize, mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_state_size(m: *const mjModel, sig: i32) -> i32 {
    todo!() // mj_stateSize
}

/// C: mj_getState (engine/engine_support.h:44)
/// Calls: mj_stateElemConstPtr, mj_stateElemSize, mju_copy, mju_message
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_get_state(m: *const mjModel, d: *const mjData, state: *mut f64, sig: i32) {
    todo!() // mj_getState
}

/// C: mj_extractState (engine/engine_support.h:47)
/// Calls: mj_stateElemSize, mju_copy, mju_message
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_extract_state(m: *const mjModel, src: *const f64, srcsig: i32, dst: *mut f64, dstsig: i32) {
    todo!() // mj_extractState
}

/// C: mj_setState (engine/engine_support.h:51)
/// Calls: mj_stateElemPtr, mj_stateElemSize, mju_copy, mju_message
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_set_state(m: *const mjModel, d: *mut mjData, state: *const f64, sig: i32) {
    todo!() // mj_setState
}

/// C: mj_copyState (engine/engine_support.h:54)
/// Calls: mj_stateElemConstPtr, mj_stateElemPtr, mj_stateElemSize, mju_copy, mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_copy_state(m: *const mjModel, src: *const mjData, dst: *mut mjData, sig: i32) {
    todo!() // mj_copyState
}

/// C: mj_setKeyframe (engine/engine_support.h:57)
/// Calls: mju_copy, mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_set_keyframe(m: *mut mjModel, d: *const mjData, k: i32) {
    todo!() // mj_setKeyframe
}

/// C: mj_fullM (engine/engine_support.h:62)
/// Calls: mju_sym2dense
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_full_m(m: *const mjModel, d: *const mjData, dst: *mut f64) {
    // SAFETY: m, d, dst valid. Calls mju_sym2dense with sparse M from mjData.
    //   mjModel: nv at 8, M_rownnz at 5424, M_rowadr at 5432, M_colind at 5440
    //   mjData: M at 161120
    unsafe {
        let m_ptr = m as *const u8;
        let d_ptr = d as *const u8;
        let nv = *(m_ptr.add(8) as *const usize) as i32;
        let M_rownnz = *(m_ptr.add(5424) as *const *const i32);
        let M_rowadr = *(m_ptr.add(5432) as *const *const i32);
        let M_colind = *(m_ptr.add(5440) as *const *const i32);
        let M = *(d_ptr.add(161120) as *const *const f64);

        crate::engine::engine_util_sparse::mju_sym2dense(dst, M, nv, M_rownnz, M_rowadr, M_colind);
    }
}

/// C: mj_mulM (engine/engine_support.h:65)
/// Calls: mju_mulSymVecSparse
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_mul_m(m: *const mjModel, d: *const mjData, res: *mut f64, vec: *const f64) {
    // SAFETY: m, d, res, vec valid. Calls mju_mulSymVecSparse.
    //   mjModel: nv at 8, M_rownnz at 5424, M_rowadr at 5432, M_colind at 5440
    //   mjData: M at 161120
    unsafe {
        let m_ptr = m as *const u8;
        let d_ptr = d as *const u8;
        let nv = *(m_ptr.add(8) as *const usize) as i32;
        let M_rownnz = *(m_ptr.add(5424) as *const *const i32);
        let M_rowadr = *(m_ptr.add(5432) as *const *const i32);
        let M_colind = *(m_ptr.add(5440) as *const *const i32);
        let M = *(d_ptr.add(161120) as *const *const f64);

        crate::engine::engine_util_sparse::mju_mul_sym_vec_sparse(res, M, vec, nv, M_rownnz, M_rowadr, M_colind);
    }
}

/// C: mj_mulM2 (engine/engine_support.h:68)
/// Calls: mju_dotSparse, mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_mul_m2(m: *const mjModel, d: *const mjData, res: *mut f64, vec: *const f64) {
    todo!() // mj_mulM2
}

/// C: mj_addM (engine/engine_support.h:72)
/// Calls: mju_addToMatSparse, mju_addToSymSparse
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_add_m(m: *const mjModel, d: *mut mjData, dst: *mut f64, rownnz: *mut i32, rowadr: *mut i32, colind: *mut i32) {
    todo!() // mj_addM
}

/// C: mj_applyFT (engine/engine_support.h:79)
/// Calls: mj_bodyChain, mj_freeStack, mj_isSparse, mj_jac, mj_jacSparse, mj_markStack, mj_stackAllocInfo, mju_addTo, mju_message, mju_mulMatTVec
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_apply_ft(m: *const mjModel, d: *mut mjData, force: *const f64, torque: *const f64, point: *const f64, body: i32, qfrc_target: *mut f64) {
    todo!() // mj_applyFT
}

/// C: mj_xfrcAccumulate (engine/engine_support.h:84)
/// Calls: mj_applyFT, mju_isZero, mju_isZeroByte
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_xfrc_accumulate(m: *const mjModel, d: *mut mjData, qfrc: *mut f64) {
    todo!() // mj_xfrcAccumulate
}

/// C: mj_geomDistance (engine/engine_support.h:90)
/// Calls: mj_geomDistanceCCD, mju_addScl3, mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_geom_distance(m: *const mjModel, d: *mut mjData, geom1: i32, geom2: i32, distmax: f64, fromto: *mut f64) -> f64 {
    todo!() // mj_geomDistance
}

/// C: mj_differentiatePos (engine/engine_support.h:94)
/// Calls: mju_scl3, mju_subQuat
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_differentiate_pos(m: *const mjModel, qvel: *mut f64, dt: f64, qpos1: *const f64, qpos2: *const f64) {
    todo!() // mj_differentiatePos
}

/// C: mj_integratePosInd (engine/engine_support.h:98)
/// Calls: mju_quatIntegrate
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_integrate_pos_ind(m: *const mjModel, qpos: *mut f64, qvel: *const f64, dt: f64, index: *const i32, nbody: i32) {
    todo!() // mj_integratePosInd
}

/// C: mj_integratePos (engine/engine_support.h:102)
/// Calls: mj_integratePosInd
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_integrate_pos(m: *const mjModel, qpos: *mut f64, qvel: *const f64, dt: f64) {
    todo!() // mj_integratePos
}

/// C: mj_normalizeQuat (engine/engine_support.h:105)
/// Calls: mju_normalize4
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_normalize_quat(m: *const mjModel, qpos: *mut f64) {
    // SAFETY: m is valid mjModel, qpos is valid qpos array. Access fields via byte offsets:
    //   njnt: usize at offset 72
    //   jnt_type: *const i32 at offset 2048
    //   jnt_qposadr: *const i32 at offset 2056
    // mjJNT_BALL = 1, mjJNT_FREE = 0
    unsafe {
        let m_ptr = m as *const u8;
        let njnt = *(m_ptr.add(72) as *const usize);
        let jnt_type = *(m_ptr.add(2048) as *const *const i32);
        let jnt_qposadr = *(m_ptr.add(2056) as *const *const i32);

        for i in 0..njnt {
            let jtype = *jnt_type.add(i);
            // mjJNT_FREE = 0, mjJNT_BALL = 1
            if jtype == 1 || jtype == 0 {
                let offset = if jtype == 0 { 3 } else { 0 }; // FREE has 3 pos before quat
                let adr = *jnt_qposadr.add(i) as usize + offset as usize;
                crate::engine::engine_util_blas::mju_normalize4(qpos.add(adr));
            }
        }
    }
}

/// C: mj_actuatorDisabled (engine/engine_support.h:108)
#[allow(unused_variables, non_snake_case)]
pub fn mj_actuator_disabled(m: *const mjModel, i: i32) -> i32 {
    // SAFETY: m is a valid mjModel pointer; actuator_group ptr at offset 4696, opt.disableactuator i32 at offset 1024
    unsafe {
        let m_ptr = m as *const u8;
        let actuator_group_ptr = *(m_ptr.add(4696) as *const *const i32);
        let disableactuator = *(m_ptr.add(1024) as *const i32);
        let group = *actuator_group_ptr.offset(i as isize);
        if group < 0 || group > 30 {
            return 0;
        }
        if (disableactuator & (1 << group)) != 0 { 1 } else { 0 }
    }
}

/// C: mj_nextActivation (engine/engine_support.h:111)
/// Calls: mj_dcmotorSlots, mj_lugreStribeck, mju_clip, mju_max
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_next_activation(m: *const mjModel, d: *const mjData, actuator_id: i32, act_adr: i32, act_dot: f64) -> f64 {
    todo!() // mj_nextActivation
}

/// C: mj_getTotalmass (engine/engine_support.h:115)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_get_totalmass(m: *const mjModel) -> f64 {
    // SAFETY: m is valid mjModel. Access fields via byte offsets:
    //   nbody: usize at offset 32
    //   body_mass: *const f64 at offset 1888
    unsafe {
        let m_ptr = m as *const u8;
        let nbody = *(m_ptr.add(32) as *const usize);
        let body_mass = *(m_ptr.add(1888) as *const *const f64);

        let mut res: f64 = 0.0;
        for i in 1..nbody {
            res += *body_mass.add(i);
        }
        res
    }
}

/// C: mj_setTotalmass (engine/engine_support.h:118)
/// Calls: mj_getTotalmass, mju_max
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_set_totalmass(m: *mut mjModel, newmass: f64) {
    const MJ_MINVAL: f64 = 1E-15_f64;
    // SAFETY: m is valid mutable mjModel. Access fields via byte offsets:
    //   nbody: usize at offset 32
    //   body_mass: *mut f64 at offset 1888
    //   body_inertia: *mut f64 at offset 1904
    unsafe {
        let m_ptr = m as *mut u8;
        let nbody = *(m_ptr.add(32) as *const usize);
        let body_mass = *(m_ptr.add(1888) as *const *mut f64);
        let body_inertia = *(m_ptr.add(1904) as *const *mut f64);

        // compute scale factor
        let current = mj_get_totalmass(m);
        let scale = f64::max(MJ_MINVAL, newmass / f64::max(MJ_MINVAL, current));

        // scale all masses and inertias
        for i in 1..nbody {
            *body_mass.add(i) *= scale;
            *body_inertia.add(3 * i) *= scale;
            *body_inertia.add(3 * i + 1) *= scale;
            *body_inertia.add(3 * i + 2) *= scale;
        }
    }
}

/// C: mj_version (engine/engine_support.h:121)
#[allow(unused_variables, non_snake_case)]
pub fn mj_version() -> i32 {
    3010001
}

/// C: mj_versionString (engine/engine_support.h:124)
#[allow(unused_variables, non_snake_case)]
pub fn mj_version_string() -> *const i8 {
    // SAFETY: returns pointer to static null-terminated string
    b"3.10.1\0".as_ptr() as *const i8
}

/// C: mju_condataSize (engine/engine_support.h:127)
/// Calls: FilePath::size
#[allow(unused_variables, non_snake_case)]
pub fn mju_condata_size(dataSpec: i32) -> i32 {
    // mjNCONDATA = 7, mjCONDATA_SIZE = [1, 3, 3, 1, 3, 3, 3]
    const MJ_NCONDATA: i32 = 7;
    const MJ_CONDATA_SIZE: [i32; 7] = [1, 3, 3, 1, 3, 3, 3];

    let mut size: i32 = 0;
    let mut i: i32 = 0;
    while i < MJ_NCONDATA {
        if (dataSpec & (1 << i)) != 0 {
            size += MJ_CONDATA_SIZE[i as usize];
        }
        i += 1;
    }
    size
}

/// C: mju_raydataSize (engine/engine_support.h:130)
/// Calls: FilePath::size
#[allow(unused_variables, non_snake_case)]
pub fn mju_raydata_size(dataspec: i32) -> i32 {
    // mjNRAYDATA = 6, mjRAYDATA_SIZE = [1, 3, 3, 3, 3, 1]
    const MJ_NRAYDATA: i32 = 6;
    const MJ_RAYDATA_SIZE: [i32; 6] = [1, 3, 3, 3, 3, 1];

    let mut size: i32 = 0;
    let mut i: i32 = 0;
    while i < MJ_NRAYDATA {
        if (dataspec & (1 << i)) != 0 {
            size += MJ_RAYDATA_SIZE[i as usize];
        }
        i += 1;
    }
    size
}

/// C: mju_camIntrinsics (engine/engine_support.h:134)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_cam_intrinsics(m: *const mjModel, camid: i32, fx: *mut f64, fy: *mut f64, cx: *mut f64, cy: *mut f64, ortho_extent: *mut f64) {
    // SAFETY: m valid mjModel. Byte offsets:
    //   cam_projection: *const i32 at 2704
    //   cam_fovy: *const f64 at 2712
    //   cam_resolution: *const i32 at 2728
    //   cam_sensorsize: *const f32 at 2744
    //   cam_intrinsic: *const f32 at 2752
    // mjPROJ_PERSPECTIVE = 0, mjPROJ_ORTHOGRAPHIC = 1
    unsafe {
        let m_ptr = m as *const u8;
        let cam_projection = *(m_ptr.add(2704) as *const *const i32);
        let cam_fovy = *(m_ptr.add(2712) as *const *const f64);
        let cam_resolution = *(m_ptr.add(2728) as *const *const i32);
        let cam_sensorsize = *(m_ptr.add(2744) as *const *const f32);
        let cam_intrinsic = *(m_ptr.add(2752) as *const *const f32);

        let cid = camid as usize;
        let width = *cam_resolution.add(2 * cid) as f64;
        let height = *cam_resolution.add(2 * cid + 1) as f64;
        let ss0 = *cam_sensorsize.add(2 * cid) as f64;
        let ss1 = *cam_sensorsize.add(2 * cid + 1) as f64;
        let projection = *cam_projection.add(cid);

        match projection {
            0 => {
                // mjPROJ_PERSPECTIVE
                if ss0 != 0.0 && ss1 != 0.0 {
                    let intr0 = *cam_intrinsic.add(4 * cid) as f64;
                    let intr1 = *cam_intrinsic.add(4 * cid + 1) as f64;
                    let intr2 = *cam_intrinsic.add(4 * cid + 2) as f64;
                    let intr3 = *cam_intrinsic.add(4 * cid + 3) as f64;
                    *fx = intr0 / ss0 * width;
                    *fy = intr1 / ss1 * height;
                    *cx = intr2 / ss0 * width;
                    *cy = intr3 / ss1 * height;
                } else {
                    let fovy = *cam_fovy.add(cid);
                    let f = 0.5 / f64::tan(fovy * std::f64::consts::PI / 360.0) * height;
                    *fx = f;
                    *fy = f;
                    *cx = width / 2.0;
                    *cy = height / 2.0;
                }
            }
            _ => {
                // mjPROJ_ORTHOGRAPHIC
                *fx = width / 2.0;
                *fy = height / 2.0;
                *cx = *fx;
                *cy = *fy;
            }
        }

        // extent only used for orthographic cameras
        *ortho_extent = *cam_fovy.add(cid);
    }
}

/// C: mj_readCtrl (engine/engine_support.h:141)
/// Calls: mju_historyRead, mju_message
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_read_ctrl(m: *const mjModel, d: *const mjData, id: i32, time: f64, interp: i32) -> f64 {
    todo!() // mj_readCtrl
}

/// C: mj_readSensor (engine/engine_support.h:147)
/// Calls: mju_historyRead, mju_message
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_read_sensor(m: *const mjModel, d: *const mjData, id: i32, time: f64, result: *mut f64, interp: i32) -> *const f64 {
    todo!() // mj_readSensor
}

/// C: mj_initCtrlHistory (engine/engine_support.h:152)
/// Calls: mju_historyInit, mju_message
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_init_ctrl_history(m: *const mjModel, d: *mut mjData, id: i32, times: *const f64, values: *const f64) {
    todo!() // mj_initCtrlHistory
}

/// C: mj_initSensorHistory (engine/engine_support.h:158)
/// Calls: mju_historyInit, mju_message
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_init_sensor_history(m: *const mjModel, d: *mut mjData, id: i32, times: *const f64, values: *const f64, phase: f64) {
    todo!() // mj_initSensorHistory
}

