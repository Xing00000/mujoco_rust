//! Port of: Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_support.c
//! IR hash: 1fa44043fe383ce9
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mj_stateElemSize (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_support.c:138)
/// Calls: cxx:_mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_state_elem_size(m: *const mjModel, sig: u32) -> i32 {
    todo!() // cxx-internal:engine_support.c.o:_mj_stateElemSize
}

/// C: mj_stateElemPtr (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_support.c:162)
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

/// C: mj_stateElemConstPtr (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_support.c:184)
/// Calls: cxx-internal:engine_support.c.o:_mj_stateElemPtr
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_state_elem_const_ptr(m: *const mjModel, d: *const mjData, sig: u32) -> *const f64 {
    todo!() // cxx-internal:engine_support.c.o:_mj_stateElemConstPtr
}

/// C: mj_geomDistanceCCD (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_support.c:519)
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

/// C: mj_stateSize (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_support.h:41)
/// Calls: cxx-internal:engine_support.c.o:_mj_stateElemSize, cxx:_mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_state_size(m: *const mjModel, sig: i32) -> i32 {
    todo!() // cxx:_mj_stateSize
}

/// C: mj_getState (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_support.h:44)
/// Calls: cxx-internal:engine_support.c.o:_mj_stateElemConstPtr, cxx-internal:engine_support.c.o:_mj_stateElemSize, cxx:_mju_copy, cxx:_mju_message
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_get_state(m: *const mjModel, d: *const mjData, state: *mut f64, sig: i32) {
    todo!() // cxx:_mj_getState
}

/// C: mj_extractState (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_support.h:47)
/// Calls: cxx-internal:engine_support.c.o:_mj_stateElemSize, cxx:_mju_copy, cxx:_mju_message
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_extract_state(m: *const mjModel, src: *const f64, srcsig: i32, dst: *mut f64, dstsig: i32) {
    todo!() // cxx:_mj_extractState
}

/// C: mj_setState (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_support.h:51)
/// Calls: cxx-internal:engine_support.c.o:_mj_stateElemPtr, cxx-internal:engine_support.c.o:_mj_stateElemSize, cxx:_mju_copy, cxx:_mju_message
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_set_state(m: *const mjModel, d: *mut mjData, state: *const f64, sig: i32) {
    todo!() // cxx:_mj_setState
}

/// C: mj_copyState (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_support.h:54)
/// Calls: cxx-internal:engine_support.c.o:_mj_stateElemConstPtr, cxx-internal:engine_support.c.o:_mj_stateElemPtr, cxx-internal:engine_support.c.o:_mj_stateElemSize, cxx:_mju_copy, cxx:_mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_copy_state(m: *const mjModel, src: *const mjData, dst: *mut mjData, sig: i32) {
    todo!() // cxx:_mj_copyState
}

/// C: mj_setKeyframe (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_support.h:57)
/// Calls: cxx:_mju_copy, cxx:_mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_set_keyframe(m: *mut mjModel, d: *const mjData, k: i32) {
    todo!() // cxx:_mj_setKeyframe
}

/// C: mj_fullM (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_support.h:62)
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

/// C: mj_mulM (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_support.h:65)
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

/// C: mj_mulM2 (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_support.h:68)
/// Calls: cxx:_mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_mul_m2(m: *const mjModel, d: *const mjData, res: *mut f64, vec: *const f64) {
    todo!() // cxx:_mj_mulM2
}

/// C: mj_addM (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_support.h:72)
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

/// C: mj_applyFT (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_support.h:79)
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

/// C: mj_xfrcAccumulate (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_support.h:84)
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

/// C: mj_geomDistance (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_support.h:90)
/// Calls: cxx-internal:engine_support.c.o:_mj_geomDistanceCCD, cxx:_mju_addScl3, cxx:_mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_geom_distance(m: *const mjModel, d: *mut mjData, geom1: i32, geom2: i32, distmax: f64, fromto: *mut f64) -> f64 {
    todo!() // cxx:_mj_geomDistance
}

/// C: mj_differentiatePos (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_support.h:94)
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

/// C: mj_integratePosInd (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_support.h:98)
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

/// C: mj_integratePos (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_support.h:102)
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

/// C: mj_normalizeQuat (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_support.h:105)
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

/// C: mj_actuatorDisabled (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_support.h:108)
#[allow(unused_variables, non_snake_case)]
pub fn mj_actuator_disabled(m: *const mjModel, i: i32) -> i32 {
    todo!() // cxx:_mj_actuatorDisabled
}

/// C: mj_nextActivation (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_support.h:111)
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

/// C: mj_getTotalmass (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_support.h:115)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_get_totalmass(m: *const mjModel) -> f64 {
    todo!() // cxx:_mj_getTotalmass
}

/// C: mj_setTotalmass (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_support.h:118)
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

/// C: mj_version (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_support.h:121)
#[allow(unused_variables, non_snake_case)]
pub fn mj_version() -> i32 {
    todo!() // cxx:_mj_version
}

/// C: mj_versionString (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_support.h:124)
#[allow(unused_variables, non_snake_case)]
pub fn mj_version_string() -> *const i8 {
    todo!() // cxx:_mj_versionString
}

/// C: mju_condataSize (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_support.h:127)
/// Calls: cxx:__ZNK6mujoco4user8FilePath4sizeEv
#[allow(unused_variables, non_snake_case)]
pub fn mju_condata_size(dataSpec: i32) -> i32 {
    todo!() // cxx:_mju_condataSize
}

/// C: mju_raydataSize (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_support.h:130)
/// Calls: cxx:__ZNK6mujoco4user8FilePath4sizeEv
#[allow(unused_variables, non_snake_case)]
pub fn mju_raydata_size(dataspec: i32) -> i32 {
    todo!() // cxx:_mju_raydataSize
}

/// C: mju_camIntrinsics (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_support.h:134)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_cam_intrinsics(m: *const mjModel, camid: i32, fx: *mut f64, fy: *mut f64, cx: *mut f64, cy: *mut f64, ortho_extent: *mut f64) {
    todo!() // cxx:_mju_camIntrinsics
}

/// C: mj_readCtrl (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_support.h:141)
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

/// C: mj_readSensor (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_support.h:147)
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

/// C: mj_initCtrlHistory (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_support.h:152)
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

/// C: mj_initSensorHistory (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_support.h:158)
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

