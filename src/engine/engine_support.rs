//! Port of: engine/engine_support.c
//! IR hash: fd5288ad7242f001
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mj_stateElemSize (engine/engine_support.c:138)
/// Calls: cxx:_mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_state_elem_size(m: *const mjModel, sig: u32) -> i32 {
    todo!() // cxx-internal:engine_support.c.o:_mj_stateElemSize
}

/// C: mj_stateElemPtr (engine/engine_support.c:162)
/// Calls: cxx:_mju_message
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_state_elem_ptr(m: *const mjModel, d: *mut mjData, sig: u32) -> *mut f64 {
    todo!() // cxx-internal:engine_support.c.o:_mj_stateElemPtr
}

/// C: mj_stateElemConstPtr (engine/engine_support.c:184)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_support.c:_mj_stateElemPtr
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_state_elem_const_ptr(m: *const mjModel, d: *const mjData, sig: u32) -> *const f64 {
    todo!() // cxx-internal:engine_support.c.o:_mj_stateElemConstPtr
}

/// C: mj_stateSize (engine/engine_support.c:190)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_support.c:_mj_stateElemSize, cxx:_mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_state_size(m: *const mjModel, sig: i32) -> i32 {
    todo!() // cxx:_mj_stateSize
}

/// C: mj_getState (engine/engine_support.c:214)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_support.c:_mj_stateElemConstPtr, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_support.c:_mj_stateElemSize, cxx:_mju_copy, cxx:_mju_message
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_get_state(m: *const mjModel, d: *const mjData, state: *mut f64, sig: i32) {
    todo!() // cxx:_mj_getState
}

/// C: mj_extractState (engine/engine_support.c:251)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_support.c:_mj_stateElemSize, cxx:_mju_copy, cxx:_mju_message
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_extract_state(m: *const mjModel, src: *const f64, srcsig: i32, dst: *mut f64, dstsig: i32) {
    todo!() // cxx:_mj_extractState
}

/// C: mj_setState (engine/engine_support.c:282)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_support.c:_mj_stateElemPtr, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_support.c:_mj_stateElemSize, cxx:_mju_copy, cxx:_mju_message
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_set_state(m: *const mjModel, d: *mut mjData, state: *const f64, sig: i32) {
    todo!() // cxx:_mj_setState
}

/// C: mj_copyState (engine/engine_support.c:319)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_support.c:_mj_stateElemConstPtr, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_support.c:_mj_stateElemPtr, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_support.c:_mj_stateElemSize, cxx:_mju_copy, cxx:_mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_copy_state(m: *const mjModel, src: *const mjData, dst: *mut mjData, sig: i32) {
    todo!() // cxx:_mj_copyState
}

/// C: mj_setKeyframe (engine/engine_support.c:355)
/// Calls: cxx:_mju_copy, cxx:_mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_set_keyframe(m: *mut mjModel, d: *const mjData, k: i32) {
    todo!() // cxx:_mj_setKeyframe
}

/// C: mj_fullM (engine/engine_support.c:378)
/// Calls: cxx:_mju_sym2dense
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_full_m(m: *const mjModel, d: *const mjData, dst: *mut f64) {
    todo!() // cxx:_mj_fullM
}

/// C: mj_mulM (engine/engine_support.c:384)
/// Calls: cxx:_mju_mulSymVecSparse
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_mul_m(m: *const mjModel, d: *const mjData, res: *mut f64, vec: *const f64) {
    todo!() // cxx:_mj_mulM
}

/// C: mj_mulM2 (engine/engine_support.c:390)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_sparse.h:_mju_dotSparse, cxx:_mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_mul_m2(m: *const mjModel, d: *const mjData, res: *mut f64, vec: *const f64) {
    todo!() // cxx:_mj_mulM2
}

/// C: mj_addM (engine/engine_support.c:419)
/// Calls: cxx:_mju_addToMatSparse, cxx:_mju_addToSymSparse
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_add_m(m: *const mjModel, d: *mut mjData, dst: *mut f64, rownnz: *mut i32, rowadr: *mut i32, colind: *mut i32) {
    todo!() // cxx:_mj_addM
}

/// C: mj_applyFT (engine/engine_support.c:440)
/// Calls: cxx:_mj_bodyChain, cxx:_mj_freeStack, cxx:_mj_isSparse, cxx:_mj_jac, cxx:_mj_jacSparse, cxx:_mj_markStack, cxx:_mj_stackAllocInfo, cxx:_mju_addTo, cxx:_mju_message, cxx:_mju_mulMatTVec
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_apply_ft(m: *const mjModel, d: *mut mjData, force: *const f64, torque: *const f64, point: *const f64, body: i32, qfrc_target: *mut f64) {
    todo!() // cxx:_mj_applyFT
}

/// C: mj_xfrcAccumulate (engine/engine_support.c:497)
/// Calls: cxx:_mj_applyFT, cxx:_mju_isZero, cxx:_mju_isZeroByte
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_xfrc_accumulate(m: *const mjModel, d: *mut mjData, qfrc: *mut f64) {
    todo!() // cxx:_mj_xfrcAccumulate
}

/// C: mj_geomDistanceCCD (engine/engine_support.c:519)
/// Calls: cxx:_mj_freeStack, cxx:_mj_markStack, cxx:_mj_stackAllocByte, cxx:_mjc_ccd, cxx:_mjc_ccdSize, cxx:_mjc_initCCDObj, cxx:_mju_copy3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_geom_distance_ccd(m: *const mjModel, d: *mut mjData, g1: i32, g2: i32, distmax: f64, fromto: *mut f64) -> f64 {
    todo!() // cxx-internal:engine_support.c.o:_mj_geomDistanceCCD
}

/// C: mj_geomDistance (engine/engine_support.c:554)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_support.c:_mj_geomDistanceCCD, cxx:_mju_addScl3, cxx:_mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_geom_distance(m: *const mjModel, d: *mut mjData, geom1: i32, geom2: i32, distmax: f64, fromto: *mut f64) -> f64 {
    todo!() // cxx:_mj_geomDistance
}

/// C: mj_insideSite (engine/engine_support.c:605)
/// Calls: cxx:_mju_insidePrimitive, cxx:_mju_mulMatTVec3, cxx:_mju_sub3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_inside_site(m: *const mjModel, d: *const mjData, siteid: i32, point: *const f64) -> i32 {
    todo!() // cxx:_mj_insideSite
}

/// C: mj_differentiatePos (engine/engine_support.c:654)
/// Calls: cxx:_mju_scl3, cxx:_mju_subQuat
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_differentiate_pos(m: *const mjModel, qvel: *mut f64, dt: f64, qpos1: *const f64, qpos2: *const f64) {
    todo!() // cxx:_mj_differentiatePos
}

/// C: mj_integratePosInd (engine/engine_support.c:688)
/// Calls: cxx:_mju_quatIntegrate
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_integrate_pos_ind(m: *const mjModel, qpos: *mut f64, qvel: *const f64, dt: f64, index: *const i32, nbody: i32) {
    todo!() // cxx:_mj_integratePosInd
}

/// C: mj_integratePos (engine/engine_support.c:726)
/// Calls: cxx:_mj_integratePosInd
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_integrate_pos(m: *const mjModel, qpos: *mut f64, qvel: *const f64, dt: f64) {
    todo!() // cxx:_mj_integratePos
}

/// C: mj_normalizeQuat (engine/engine_support.c:732)
/// Calls: cxx:_mju_normalize4
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_normalize_quat(m: *const mjModel, qpos: *mut f64) {
    todo!() // cxx:_mj_normalizeQuat
}

/// C: mj_actuatorDisabled (engine/engine_support.c:743)
#[allow(unused_variables, non_snake_case)]
pub fn mj_actuator_disabled(m: *const mjModel, i: i32) -> i32 {
    todo!() // cxx:_mj_actuatorDisabled
}

/// C: mj_nextActivation (engine/engine_support.c:755)
/// Calls: cxx:_mj_dcmotorSlots, cxx:_mj_lugreStribeck, cxx:_mju_clip, cxx:_mju_max
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_next_activation(m: *const mjModel, d: *const mjData, actuator_id: i32, act_adr: i32, act_dot: f64) -> f64 {
    todo!() // cxx:_mj_nextActivation
}

/// C: mj_getTotalmass (engine/engine_support.c:828)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_get_totalmass(m: *const mjModel) -> f64 {
    todo!() // cxx:_mj_getTotalmass
}

/// C: mj_setTotalmass (engine/engine_support.c:840)
/// Calls: cxx:_mj_getTotalmass, cxx:_mju_max
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_set_totalmass(m: *mut mjModel, newmass: f64) {
    todo!() // cxx:_mj_setTotalmass
}

/// C: mj_version (engine/engine_support.c:857)
#[allow(unused_variables, non_snake_case)]
pub fn mj_version() -> i32 {
    3010001
}

/// C: mj_versionString (engine/engine_support.c:863)
#[allow(unused_variables, non_snake_case)]
pub fn mj_version_string() -> *const i8 {
    todo!() // cxx:_mj_versionString
}

/// C: mju_condataSize (engine/engine_support.c:870)
/// Calls: cxx:__ZNK6mujoco4user8FilePath4sizeEv
#[allow(unused_variables, non_snake_case)]
pub fn mju_condata_size(dataSpec: i32) -> i32 {
    todo!() // cxx:_mju_condataSize
}

/// C: mju_raydataSize (engine/engine_support.c:882)
/// Calls: cxx:__ZNK6mujoco4user8FilePath4sizeEv
#[allow(unused_variables, non_snake_case)]
pub fn mju_raydata_size(dataspec: i32) -> i32 {
    todo!() // cxx:_mju_raydataSize
}

/// C: mju_camIntrinsics (engine/engine_support.c:897)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_cam_intrinsics(m: *const mjModel, camid: i32, fx: *mut f64, fy: *mut f64, cx: *mut f64, cy: *mut f64, ortho_extent: *mut f64) {
    todo!() // cxx:_mju_camIntrinsics
}

/// C: mj_readCtrl (engine/engine_support.c:934)
/// Calls: cxx:_mju_historyRead, cxx:_mju_message
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_read_ctrl(m: *const mjModel, d: *const mjData, id: i32, time: f64, interp: i32) -> f64 {
    todo!() // cxx:_mj_readCtrl
}

/// C: mj_readSensor (engine/engine_support.c:961)
/// Calls: cxx:_mju_historyRead, cxx:_mju_message
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_read_sensor(m: *const mjModel, d: *const mjData, id: i32, time: f64, result: *mut f64, interp: i32) -> *const f64 {
    todo!() // cxx:_mj_readSensor
}

/// C: mj_initCtrlHistory (engine/engine_support.c:987)
/// Calls: cxx:_mju_historyInit, cxx:_mju_message
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_init_ctrl_history(m: *const mjModel, d: *mut mjData, id: i32, times: *const f64, values: *const f64) {
    todo!() // cxx:_mj_initCtrlHistory
}

/// C: mj_initSensorHistory (engine/engine_support.c:1017)
/// Calls: cxx:_mju_historyInit, cxx:_mju_message
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_init_sensor_history(m: *const mjModel, d: *mut mjData, id: i32, times: *const f64, values: *const f64, phase: f64) {
    todo!() // cxx:_mj_initSensorHistory
}

