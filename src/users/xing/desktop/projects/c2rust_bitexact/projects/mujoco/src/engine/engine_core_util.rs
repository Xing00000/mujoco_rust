//! Port of: Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_util.h
//! IR hash: 1fa44043fe383ce9
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mj_isPyramidal (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_util.h:31)
/// Calls: cxx-internal:render_context.c.o:_cone
#[allow(unused_variables, non_snake_case)]
pub fn mj_is_pyramidal(m: *const mjModel) -> i32 {
    todo!() // cxx:_mj_isPyramidal
}

/// C: mj_isSparse (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_util.h:34)
#[allow(unused_variables, non_snake_case)]
pub fn mj_is_sparse(m: *const mjModel) -> i32 {
    todo!() // cxx:_mj_isSparse
}

/// C: mj_mergeChain (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_util.h:40)
/// Calls: cxx:__ZNK6mujoco4user8FilePath5emptyEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_merge_chain(m: *const mjModel, chain: *mut i32, b1: i32, b2: i32, flg_skipcommon: i32) -> i32 {
    todo!() // cxx:_mj_mergeChain
}

/// C: mj_mergeChainSimple (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_util.h:43)
#[allow(unused_variables, non_snake_case)]
pub fn mj_merge_chain_simple(m: *const mjModel, chain: *mut i32, b1: i32, b2: i32) -> i32 {
    todo!() // cxx:_mj_mergeChainSimple
}

/// C: mj_bodyChain (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_util.h:46)
/// Calls: cxx:__ZNK6mujoco4user8FilePath5emptyEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_body_chain(m: *const mjModel, body: i32, chain: *mut i32) -> i32 {
    todo!() // cxx:_mj_bodyChain
}

/// C: mj_jac (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_util.h:52)
/// Calls: cxx:_mju_sub3, cxx:_mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_jac(m: *const mjModel, d: *const mjData, jacp: *mut f64, jacr: *mut f64, point: *const f64, body: i32) {
    todo!() // cxx:_mj_jac
}

/// C: mj_jacBody (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_util.h:56)
/// Calls: cxx:_mj_jac
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_jac_body(m: *const mjModel, d: *const mjData, jacp: *mut f64, jacr: *mut f64, body: i32) {
    todo!() // cxx:_mj_jacBody
}

/// C: mj_jacBodyCom (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_util.h:60)
/// Calls: cxx:_mj_jac
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_jac_body_com(m: *const mjModel, d: *const mjData, jacp: *mut f64, jacr: *mut f64, body: i32) {
    todo!() // cxx:_mj_jacBodyCom
}

/// C: mj_jacSubtreeCom (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_util.h:64)
/// Calls: cxx:_mj_freeStack, cxx:_mj_jac, cxx:_mj_markStack, cxx:_mj_stackAllocInfo, cxx:_mju_addToScl, cxx:_mju_scl, cxx:_mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_jac_subtree_com(m: *const mjModel, d: *mut mjData, jacp: *mut f64, body: i32) {
    todo!() // cxx:_mj_jacSubtreeCom
}

/// C: mj_jacGeom (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_util.h:67)
/// Calls: cxx:_mj_jac
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_jac_geom(m: *const mjModel, d: *const mjData, jacp: *mut f64, jacr: *mut f64, geom: i32) {
    todo!() // cxx:_mj_jacGeom
}

/// C: mj_jacSite (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_util.h:71)
/// Calls: cxx:_mj_jac
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_jac_site(m: *const mjModel, d: *const mjData, jacp: *mut f64, jacr: *mut f64, site: i32) {
    todo!() // cxx:_mj_jacSite
}

/// C: mj_jacPointAxis (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_util.h:75)
/// Calls: cxx:_mj_freeStack, cxx:_mj_jac, cxx:_mj_markStack, cxx:_mj_stackAllocInfo
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_jac_point_axis(m: *const mjModel, d: *mut mjData, jacPoint: *mut f64, jacAxis: *mut f64, point: *const f64, axis: *const f64, body: i32) {
    todo!() // cxx:_mj_jacPointAxis
}

/// C: mj_jacSparse (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_util.h:80)
/// Calls: cxx:_mju_message, cxx:_mju_sub3, cxx:_mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_jac_sparse(m: *const mjModel, d: *const mjData, jacp: *mut f64, jacr: *mut f64, point: *const f64, body: i32, NV: i32, chain: *const i32, flg_skipcommon: i32) {
    todo!() // cxx:_mj_jacSparse
}

/// C: mj_jacSparseSimple (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_util.h:85)
/// Calls: cxx:_mju_sub3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_jac_sparse_simple(m: *const mjModel, d: *const mjData, jacdifp: *mut f64, jacdifr: *mut f64, point: *const f64, body: i32, flg_second: i32, NV: i32, start: i32) {
    todo!() // cxx:_mj_jacSparseSimple
}

/// C: mj_jacDotSparse (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_util.h:90)
/// Calls: cxx:_mju_message, cxx:_mju_sub3, cxx:_mju_transformSpatial, cxx:_mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_jac_dot_sparse(m: *const mjModel, d: *const mjData, jacp: *mut f64, jacr: *mut f64, point: *const f64, body: i32, NV: i32, chain: *const i32) {
    todo!() // cxx:_mj_jacDotSparse
}

/// C: mj_jacDifPair (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_util.h:95)
/// Calls: cxx:_mj_jac, cxx:_mj_jacSparse, cxx:_mj_jacSparseSimple, cxx:_mj_mergeChain, cxx:_mj_mergeChainSimple, cxx:_mju_sub
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_jac_dif_pair(m: *const mjModel, d: *const mjData, chain: *mut i32, b1: i32, b2: i32, pos1: *const f64, pos2: *const f64, jac1p: *mut f64, jac2p: *mut f64, jacdifp: *mut f64, jac1r: *mut f64, jac2r: *mut f64, jacdifr: *mut f64, issparse: i32, flg_skipcommon: i32) -> i32 {
    todo!() // cxx:_mj_jacDifPair
}

/// C: mj_jacSum (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_util.h:102)
/// Calls: cxx:_mj_bodyChain, cxx:_mj_freeStack, cxx:_mj_isSparse, cxx:_mj_jac, cxx:_mj_jacSparse, cxx:_mj_jacSparseSimple, cxx:_mj_markStack, cxx:_mj_stackAllocInfo, cxx:_mju_addToScl, cxx:_mju_addToSparseMat, cxx:_mju_scl
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_jac_sum(m: *const mjModel, d: *mut mjData, chain: *mut i32, n: i32, body: *const i32, weight: *const f64, point: *const f64, jac: *mut f64, flg_rot: i32) -> i32 {
    todo!() // cxx:_mj_jacSum
}

/// C: mj_jacDot (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_util.h:107)
/// Calls: cxx:_mju_sub3, cxx:_mju_transformSpatial, cxx:_mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_jac_dot(m: *const mjModel, d: *const mjData, jacp: *mut f64, jacr: *mut f64, point: *const f64, body: i32) {
    todo!() // cxx:_mj_jacDot
}

/// C: mj_angmomMat (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_util.h:111)
/// Calls: cxx:_mj_freeStack, cxx:_mj_jacBodyCom, cxx:_mj_markStack, cxx:_mj_stackAllocInfo, cxx:_mju_addTo, cxx:_mju_copy3, cxx:_mju_mulMatMat, cxx:_mju_mulMatMatT3, cxx:_mju_scl, cxx:_mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_angmom_mat(m: *const mjModel, d: *mut mjData, mat: *mut f64, body: i32) {
    todo!() // cxx:_mj_angmomMat
}

/// C: mj_objectVelocity (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_util.h:117)
/// Calls: cxx:_mju_message, cxx:_mju_transformSpatial, cxx:_mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_object_velocity(m: *const mjModel, d: *const mjData, objtype: i32, objid: i32, res: *mut f64, flg_local: i32) {
    todo!() // cxx:_mj_objectVelocity
}

/// C: mj_objectAcceleration (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_util.h:121)
/// Calls: cxx:_mju_message, cxx:_mju_transformSpatial, cxx:_mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_object_acceleration(m: *const mjModel, d: *const mjData, objtype: i32, objid: i32, res: *mut f64, flg_local: i32) {
    todo!() // cxx:_mj_objectAcceleration
}

/// C: mj_local2Global (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_util.h:125)
/// Calls: cxx:_mju_quat2Mat
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_local2global(d: *mut mjData, xpos: *mut f64, xmat: *mut f64, pos: *const f64, quat: *const f64, body: i32, sameframe: u8) {
    todo!() // cxx:_mj_local2Global
}

/// C: mju_flexGatherState (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_util.h:133)
/// Calls: cxx:_mj_objectVelocity, cxx:_mju_addTo3, cxx:_mju_copy3, cxx:_mju_cross, cxx:_mju_mulMatVec3, cxx:_mju_shellTrackInterior, cxx:_mju_sub3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_flex_gather_state(m: *const mjModel, d: *const mjData, f: i32, xpos: *mut f64, vel: *mut f64) {
    todo!() // cxx:_mju_flexGatherState
}

/// C: mj_contactForce (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_util.h:136)
/// Calls: cxx:_mj_isPyramidal, cxx:_mju_copy, cxx:_mju_decodePyramid, cxx:_mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_contact_force(m: *const mjModel, d: *const mjData, id: i32, result: *mut f64) {
    todo!() // cxx:_mj_contactForce
}

/// C: tendonLimit (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_util.h:139)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn tendon_limit(m: *const mjModel, ten_length: *const f64, i: i32) -> i32 {
    todo!() // cxx:_tendonLimit
}

/// C: mj_actuatorDamping (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_util.h:142)
/// Calls: cxx:_mju_message
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_actuator_damping(m: *const mjModel, r#type: u32, id: i32, poly: *mut f64) -> f64 {
    todo!() // cxx:_mj_actuatorDamping
}

/// C: mj_actuatorArmature (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_util.h:145)
/// Calls: cxx:_mju_message
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_actuator_armature(m: *const mjModel, r#type: u32, id: i32) -> f64 {
    todo!() // cxx:_mj_actuatorArmature
}

/// C: mj_warning (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_util.h:148)
/// Calls: cxx:_mju_message, cxx:_mju_warning, cxx:_mju_warningText
#[allow(unused_variables, non_snake_case)]
pub fn mj_warning(d: *mut mjData, warning: i32, info: i32) {
    todo!() // cxx:_mj_warning
}

