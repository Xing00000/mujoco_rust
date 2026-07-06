//! Port of: engine/engine_support.c
//! IR hash: 545f394232195ad9
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mj_stateElemSize (engine/engine_support.c:138)
/// Calls: mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_state_elem_size(m: *const mjModel, sig: mjtState) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, sig : mjtState)
    // Previous return: i32
    todo ! ()
}

/// C: mj_stateElemPtr (engine/engine_support.c:162)
/// Calls: mju_message
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_state_elem_ptr(m: *const mjModel, d: *mut mjData, sig: mjtState) -> *mut f64 {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, sig : mjtState)
    // Previous return: * mut f64
    todo ! ()
}

/// C: mj_stateElemConstPtr (engine/engine_support.c:184)
/// Calls: mj_stateElemPtr
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_state_elem_const_ptr(m: *const mjModel, d: *const mjData, sig: mjtState) -> *const f64 {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * const mjData, sig : mjtState)
    // Previous return: * const f64
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, g1 : i32, g2 : i32, distmax : f64, fromto : * mut f64)
    // Previous return: f64
    todo ! ()
}

/// C: mj_stateSize (engine/engine_support.h:41)
/// Calls: mj_stateElemSize, mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_state_size(m: *const mjModel, sig: i32) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, sig : i32)
    // Previous return: i32
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * const mjData, state : * mut f64, sig : i32)
    // Previous return: ()
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, src : * const f64, srcsig : i32, dst : * mut f64, dstsig : i32)
    // Previous return: ()
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, state : * const f64, sig : i32)
    // Previous return: ()
    todo ! ()
}

/// C: mj_copyState (engine/engine_support.h:54)
/// Calls: mj_stateElemConstPtr, mj_stateElemPtr, mj_stateElemSize, mju_copy, mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_copy_state(m: *const mjModel, src: *const mjData, dst: *mut mjData, sig: i32) {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, src : * const mjData, dst : * mut mjData, sig : i32)
    // Previous return: ()
    todo ! ()
}

/// C: mj_setKeyframe (engine/engine_support.h:57)
/// Calls: mju_copy, mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_set_keyframe(m: *mut mjModel, d: *const mjData, k: i32) {
    // WARNING: signature changed — verify body
    // Previous params: (m : * mut mjModel, d : * const mjData, k : i32)
    // Previous return: ()
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * const mjData, dst : * mut f64)
    // Previous return: ()
    todo ! ()
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
    extern "C" {
        fn mj_mulM_impl(m: *const mjModel, d: *const mjData, res: *mut f64, vec: *const f64);
    }
    // SAFETY: Forwarding to linked C/C++ implementation.
    unsafe { mj_mulM_impl(m, d, res, vec) }
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
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * const mjData, res : * mut f64, vec : * const f64)
    // Previous return: ()
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, dst : * mut f64, rownnz : * mut i32, rowadr : * mut i32, colind : * mut i32)
    // Previous return: ()
    todo ! ()
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
    extern "C" {
        fn mj_applyFT_impl(m: *const mjModel, d: *mut mjData, force: *const f64, torque: *const f64, point: *const f64, body: i32, qfrc_target: *mut f64);
    }
    // SAFETY: Forwarding to linked C/C++ implementation.
    unsafe { mj_applyFT_impl(m, d, force, torque, point, body, qfrc_target) }
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
    extern "C" {
        fn mj_xfrcAccumulate_impl(m: *const mjModel, d: *mut mjData, qfrc: *mut f64);
    }
    // SAFETY: Forwarding to linked C/C++ implementation.
    unsafe { mj_xfrcAccumulate_impl(m, d, qfrc) }
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
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, geom1 : i32, geom2 : i32, distmax : f64, fromto : * mut f64)
    // Previous return: f64
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, qvel : * mut f64, dt : f64, qpos1 : * const f64, qpos2 : * const f64)
    // Previous return: ()
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, qpos : * mut f64, qvel : * const f64, dt : f64, index : * const i32, nbody : i32)
    // Previous return: ()
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, qpos : * mut f64, qvel : * const f64, dt : f64)
    // Previous return: ()
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, qpos : * mut f64)
    // Previous return: ()
    todo ! ()
}

/// C: mj_actuatorDisabled (engine/engine_support.h:108)
#[allow(unused_variables, non_snake_case)]
pub fn mj_actuator_disabled(m: *const mjModel, i: i32) -> i32 {
    extern "C" {
        fn mj_actuatorDisabled_impl(m: *const mjModel, i: i32) -> i32;
    }
    // SAFETY: Forwarding to linked C/C++ implementation.
    unsafe { mj_actuatorDisabled_impl(m, i) }
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
    extern "C" {
        fn mj_nextActivation_impl(m: *const mjModel, d: *const mjData, actuator_id: i32, act_adr: i32, act_dot: f64) -> f64;
    }
    // SAFETY: Forwarding to linked C/C++ implementation.
    unsafe { mj_nextActivation_impl(m, d, actuator_id, act_adr, act_dot) }
}

/// C: mj_getTotalmass (engine/engine_support.h:115)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_get_totalmass(m: *const mjModel) -> f64 {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel)
    // Previous return: f64
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (m : * mut mjModel, newmass : f64)
    // Previous return: ()
    todo ! ()
}

/// C: mj_version (engine/engine_support.h:121)
#[allow(unused_variables, non_snake_case)]
pub fn mj_version() -> i32 {
    todo ! ()
}

/// C: mj_versionString (engine/engine_support.h:124)
#[allow(unused_variables, non_snake_case)]
pub fn mj_version_string() -> *const i8 {
    // WARNING: signature changed — verify body
    // Previous params: ()
    // Previous return: * const i8
    todo ! ()
}

/// C: mju_condataSize (engine/engine_support.h:127)
#[allow(unused_variables, non_snake_case)]
pub fn mju_condata_size(dataSpec: i32) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (dataSpec : i32)
    // Previous return: i32
    todo ! ()
}

/// C: mju_raydataSize (engine/engine_support.h:130)
#[allow(unused_variables, non_snake_case)]
pub fn mju_raydata_size(dataspec: i32) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (dataspec : i32)
    // Previous return: i32
    todo ! ()
}

/// C: mju_camIntrinsics (engine/engine_support.h:134)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_cam_intrinsics(m: *const mjModel, camid: i32, fx: *mut f64, fy: *mut f64, cx: *mut f64, cy: *mut f64, ortho_extent: *mut f64) {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, camid : i32, fx : * mut f64, fy : * mut f64, cx : * mut f64, cy : * mut f64, ortho_extent : * mut f64)
    // Previous return: ()
    todo ! ()
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
    extern "C" {
        fn mj_readCtrl_impl(m: *const mjModel, d: *const mjData, id: i32, time: f64, interp: i32) -> f64;
    }
    // SAFETY: Forwarding to linked C/C++ implementation.
    unsafe { mj_readCtrl_impl(m, d, id, time, interp) }
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
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * const mjData, id : i32, time : f64, result : * mut f64, interp : i32)
    // Previous return: * const f64
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, id : i32, times : * const f64, values : * const f64)
    // Previous return: ()
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, id : i32, times : * const f64, values : * const f64, phase : f64)
    // Previous return: ()
    todo ! ()
}

