//! Port of: Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_interact.c
//! IR hash: 1fa44043fe383ce9
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: convert2D (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_interact.c:270)
/// Calls: cxx:_mju_message, cxx:_mjv_alignToCamera
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn convert2d(res: *mut f64, action: i32, dx: f64, dy: f64, forward: *const f64) {
    todo!() // cxx-internal:engine_vis_interact.c.o:_convert2D
}

/// C: mjv_room2model (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_interact.h:28)
/// Calls: cxx:_mju_copy3, cxx:_mju_copy4, cxx:_mju_f2n, cxx:_mju_message, cxx:_mju_mulPose, cxx:_mju_negPose, cxx:_mju_scl3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjv_room2model(modelpos: *mut f64, modelquat: *mut f64, roompos: *const f64, roomquat: *const f64, scn: *const mjvScene) {
    todo!() // cxx:_mjv_room2model
}

/// C: mjv_model2room (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_interact.h:32)
/// Calls: cxx:_mju_copy3, cxx:_mju_copy4, cxx:_mju_f2n, cxx:_mju_message, cxx:_mju_mulPose, cxx:_mju_scl3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjv_model2room(roompos: *mut f64, roomquat: *mut f64, modelpos: *const f64, modelquat: *const f64, scn: *const mjvScene) {
    todo!() // cxx:_mjv_model2room
}

/// C: mjv_cameraInModel (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_interact.h:36)
/// Calls: cxx:_mju_addToScl3, cxx:_mju_cross, cxx:_mju_f2n, cxx:_mju_mat2Quat, cxx:_mju_message, cxx:_mju_normalize3, cxx:_mju_quat2Mat, cxx:_mju_zero3, cxx:_mjv_room2model
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjv_camera_in_model(headpos: *mut f64, forward: *mut f64, up: *mut f64, scn: *const mjvScene) {
    todo!() // cxx:_mjv_cameraInModel
}

/// C: mjv_cameraInRoom (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_interact.h:40)
/// Calls: cxx:_mju_addToScl3, cxx:_mju_f2n, cxx:_mju_message, cxx:_mju_normalize3, cxx:_mju_zero3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjv_camera_in_room(headpos: *mut f64, forward: *mut f64, up: *mut f64, scn: *const mjvScene) {
    todo!() // cxx:_mjv_cameraInRoom
}

/// C: mjv_frustumHeight (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_interact.h:44)
/// Calls: cxx:_mju_message
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjv_frustum_height(scn: *const mjvScene) -> f64 {
    todo!() // cxx:_mjv_frustumHeight
}

/// C: mjv_alignToCamera (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_interact.h:47)
/// Calls: cxx:_mju_copy, cxx:_mju_normalize
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjv_align_to_camera(res: *mut f64, vec: *const f64, forward: *const f64) {
    todo!() // cxx:_mjv_alignToCamera
}

/// C: mjv_moveCamera (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_interact.h:50)
/// Calls: cxx-internal:engine_vis_interact.c.o:_convert2D, cxx:_mju_addToScl3, cxx:_mju_cross, cxx:_mju_dot3, cxx:_mju_message, cxx:_mju_sub3, cxx:_mjv_cameraInModel, cxx:_mjv_frustumHeight
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjv_move_camera(m: *const mjModel, action: i32, reldx: f64, reldy: f64, scn: *const mjvScene, cam: *mut mjvCamera) {
    todo!() // cxx:_mjv_moveCamera
}

/// C: mjv_movePerturb (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_interact.h:54)
/// Calls: cxx-internal:engine_vis_interact.c.o:_convert2D, cxx:_mju_addToScl3, cxx:_mju_axisAngle2Quat, cxx:_mju_max, cxx:_mju_message, cxx:_mju_min, cxx:_mju_mulMatVec3, cxx:_mju_mulQuat, cxx:_mju_negQuat, cxx:_mju_normalize3, cxx:_mju_normalize4, cxx:_mju_quat2Vel, cxx:_mjv_cameraInModel
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjv_move_perturb(m: *const mjModel, d: *const mjData, action: i32, reldx: f64, reldy: f64, scn: *const mjvScene, pert: *mut mjvPerturb) {
    todo!() // cxx:_mjv_movePerturb
}

/// C: mjv_moveModel (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_interact.h:58)
/// Calls: cxx:_mju_addScl3, cxx:_mju_axisAngle2Quat, cxx:_mju_cross, cxx:_mju_dot3, cxx:_mju_f2n, cxx:_mju_message, cxx:_mju_mulQuat, cxx:_mju_n2f, cxx:_mju_normalize3, cxx:_mju_normalize4, cxx:_mjv_cameraInRoom
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjv_move_model(m: *const mjModel, action: i32, reldx: f64, reldy: f64, roomup: *const f64, scn: *mut mjvScene) {
    todo!() // cxx:_mjv_moveModel
}

/// C: mjv_initPerturb (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_interact.h:62)
/// Calls: cxx:_mj_freeStack, cxx:_mj_jac, cxx:_mj_markStack, cxx:_mj_solveM2, cxx:_mj_stackAllocInfo, cxx:_mju_addTo3, cxx:_mju_copy3, cxx:_mju_dot, cxx:_mju_dot3, cxx:_mju_max, cxx:_mju_mulMatVec3, cxx:_mju_mulQuat, cxx:_mju_sub3, cxx:_mjv_cameraInModel, cxx:_mjv_frustumHeight
#[allow(unused_variables, non_snake_case)]
pub fn mjv_init_perturb(m: *const mjModel, d: *mut mjData, scn: *const mjvScene, pert: *mut mjvPerturb) {
    todo!() // cxx:_mjv_initPerturb
}

/// C: mjv_applyPerturbPose (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_interact.h:66)
/// Calls: cxx:_mju_copy3, cxx:_mju_copy4, cxx:_mju_mulPose, cxx:_mju_negPose
#[allow(unused_variables, non_snake_case)]
pub fn mjv_apply_perturb_pose(m: *const mjModel, d: *mut mjData, pert: *const mjvPerturb, flg_paused: i32) {
    todo!() // cxx:_mjv_applyPerturbPose
}

/// C: mjv_applyPerturbForce (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_interact.h:70)
/// Calls: cxx:_mj_objectVelocity, cxx:_mju_addTo3, cxx:_mju_addToScl3, cxx:_mju_copy3, cxx:_mju_cross, cxx:_mju_dot3, cxx:_mju_max, cxx:_mju_mulMatVec3, cxx:_mju_mulQuat, cxx:_mju_negQuat, cxx:_mju_normalize3, cxx:_mju_quat2Vel, cxx:_mju_scl3, cxx:_mju_sub3
#[allow(unused_variables, non_snake_case)]
pub fn mjv_apply_perturb_force(m: *const mjModel, d: *mut mjData, pert: *const mjvPerturb) {
    todo!() // cxx:_mjv_applyPerturbForce
}

/// C: mjv_averageCamera (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_interact.h:73)
/// Calls: cxx:_mju_add3, cxx:_mju_addToScl3, cxx:_mju_dot3, cxx:_mju_f2n, cxx:_mju_message, cxx:_mju_n2f, cxx:_mju_normalize3, cxx:_mju_scl3
#[allow(unused_variables, non_snake_case)]
pub fn mjv_average_camera(cam1: *const mjvGLCamera, cam2: *const mjvGLCamera) -> mjvGLCamera {
    todo!() // cxx:_mjv_averageCamera
}

/// C: mjv_select (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_interact.h:76)
/// Calls: cxx:_mj_ray, cxx:_mj_rayFlex, cxx:_mju_addScl3, cxx:_mju_addToScl3, cxx:_mju_copy3, cxx:_mju_cross, cxx:_mju_f2n, cxx:_mju_normalize3, cxx:_mju_raySkin, cxx:_mju_scl3, cxx:_mjv_averageCamera, cxx:_mjv_cameraInModel, cxx:_mjv_flexBodyId
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjv_select(m: *const mjModel, d: *const mjData, vopt: *const mjvOption, aspectratio: f64, relx: f64, rely: f64, scn: *const mjvScene, selpnt: *mut f64, geomid: *mut i32, flexid: *mut i32, skinid: *mut i32) -> i32 {
    todo!() // cxx:_mjv_select
}

/// C: mjv_flexBodyId (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_interact.h:82)
/// Calls: cxx:_mju_addTo3, cxx:_mju_cellLookup, cxx:_mju_copy3, cxx:_mju_evalBasis, cxx:_mju_mulMatVec3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjv_flex_body_id(m: *const mjModel, d: *const mjData, flexid: i32, vertid: i32, flexpnt: *mut f64) -> i32 {
    todo!() // cxx:_mjv_flexBodyId
}

