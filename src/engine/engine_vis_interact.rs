//! Port of: engine/engine_vis_interact.c
//! IR hash: fd5288ad7242f001
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mjv_room2model (engine/engine_vis_interact.c:35)
/// Calls: cxx:_mju_copy3, cxx:_mju_copy4, cxx:_mju_f2n, cxx:_mju_message, cxx:_mju_mulPose, cxx:_mju_negPose, cxx:_mju_scl3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjv_room2model(modelpos: *mut f64, modelquat: *mut f64, roompos: *const f64, roomquat: *const f64, scn: *const mjvScene) {
    const MJMINVAL: f64 = 1e-15;

    // SAFETY: all pointers are valid (caller contract).
    unsafe {
        let mut translate: [f64; 3] = [0.0; 3];
        let mut rotate: [f64; 4] = [0.0; 4];
        let mut invpos: [f64; 3] = [0.0; 3];
        let mut invquat: [f64; 4] = [0.0; 4];

        // check scale
        if ((*scn).scale as f64) < MJMINVAL {
            crate::engine::engine_util_errmem::mju_error(
                b"mjvScene scale too small\0".as_ptr() as *const i8);
        }

        // enabled: transform
        if (*scn).enabletransform != 0 {
            // convert translate, rotate to mjtNum
            crate::engine::engine_util_misc::mju_f2n(
                translate.as_mut_ptr(), (*scn).translate.as_ptr(), 3);
            crate::engine::engine_util_misc::mju_f2n(
                rotate.as_mut_ptr(), (*scn).rotate.as_ptr(), 4);

            // invert model pose (without scale)
            crate::engine::engine_util_spatial::mju_negPose(
                invpos.as_mut_ptr(), invquat.as_mut_ptr(),
                translate.as_ptr(), rotate.as_ptr());

            // map from room to model space
            crate::engine::engine_util_spatial::mju_mulPose(
                modelpos, modelquat,
                invpos.as_ptr(), invquat.as_ptr(), roompos, roomquat);

            // divide position by scale
            crate::engine::engine_util_blas::mju_scl3(
                modelpos, modelpos, 1.0 / (*scn).scale as f64);
        }
        // disabled: copy
        else {
            crate::engine::engine_util_blas::mju_copy3(modelpos, roompos);
            crate::engine::engine_util_blas::mju_copy4(modelquat, roomquat);
        }
    }
}

/// C: mjv_model2room (engine/engine_vis_interact.c:69)
/// Calls: cxx:_mju_copy3, cxx:_mju_copy4, cxx:_mju_f2n, cxx:_mju_message, cxx:_mju_mulPose, cxx:_mju_scl3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjv_model2room(roompos: *mut f64, roomquat: *mut f64, modelpos: *const f64, modelquat: *const f64, scn: *const mjvScene) {
    const MJMINVAL: f64 = 1e-15;

    // SAFETY: all pointers are valid (caller contract).
    unsafe {
        let mut translate: [f64; 3] = [0.0; 3];
        let mut rotate: [f64; 4] = [0.0; 4];

        // check scale
        if ((*scn).scale as f64) < MJMINVAL {
            crate::engine::engine_util_errmem::mju_error(
                b"mjvScene scale too small\0".as_ptr() as *const i8);
        }

        // enabled: transform
        if (*scn).enabletransform != 0 {
            // convert translate, rotate to mjtNum
            crate::engine::engine_util_misc::mju_f2n(
                translate.as_mut_ptr(), (*scn).translate.as_ptr(), 3);
            crate::engine::engine_util_misc::mju_f2n(
                rotate.as_mut_ptr(), (*scn).rotate.as_ptr(), 4);

            // map from model to room space
            crate::engine::engine_util_spatial::mju_mulPose(
                roompos, roomquat,
                translate.as_ptr(), rotate.as_ptr(), modelpos, modelquat);

            // scale position
            crate::engine::engine_util_blas::mju_scl3(
                roompos, roompos, (*scn).scale as f64);
        }
        // disabled: copy
        else {
            crate::engine::engine_util_blas::mju_copy3(roompos, modelpos);
            crate::engine::engine_util_blas::mju_copy4(roomquat, modelquat);
        }
    }
}

/// C: mjv_cameraInModel (engine/engine_vis_interact.c:99)
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

/// C: mjv_cameraInRoom (engine/engine_vis_interact.c:172)
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

/// C: mjv_frustumHeight (engine/engine_vis_interact.c:221)
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

/// C: cameraFrustumHeight (engine/engine_vis_interact.c:251)
/// Calls: cxx:_mju_message, cxx:_mjv_cameraFrustum
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn camera_frustum_height(m: *const mjModel, cam: *const mjvCamera) -> f64 {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_interact.c:_cameraFrustumHeight
}

/// C: mjv_alignToCamera (engine/engine_vis_interact.c:266)
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

/// C: convert2D (engine/engine_vis_interact.c:285)
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

/// C: mjv_moveCamera (engine/engine_vis_interact.c:329)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_vis_interact.c:_cameraFrustumHeight, cxx-internal:engine_vis_interact.c.o:_convert2D, cxx:_mju_addScl3, cxx:_mju_addToScl3, cxx:_mju_cross, cxx:_mju_dot3, cxx:_mju_message, cxx:_mju_sub3, cxx:_mjv_cameraFrame
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjv_move_camera(m: *const mjModel, action: i32, reldx: f64, reldy: f64, cam: *mut mjvCamera) {
    todo!() // cxx:_mjv_moveCamera
}

/// C: mjv_movePerturb (engine/engine_vis_interact.c:439)
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

/// C: mjv_moveModel (engine/engine_vis_interact.c:529)
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

/// C: mjv_initPerturb (engine/engine_vis_interact.c:605)
/// Calls: cxx:_mj_freeStack, cxx:_mj_jac, cxx:_mj_markStack, cxx:_mj_solveM2, cxx:_mj_stackAllocInfo, cxx:_mju_addTo3, cxx:_mju_copy3, cxx:_mju_dot, cxx:_mju_dot3, cxx:_mju_max, cxx:_mju_mulMatVec3, cxx:_mju_mulQuat, cxx:_mju_sub3, cxx:_mjv_cameraInModel, cxx:_mjv_frustumHeight
#[allow(unused_variables, non_snake_case)]
pub fn mjv_init_perturb(m: *const mjModel, d: *mut mjData, scn: *const mjvScene, pert: *mut mjvPerturb) {
    todo!() // cxx:_mjv_initPerturb
}

/// C: mjv_applyPerturbPose (engine/engine_vis_interact.c:665)
/// Calls: cxx:_mju_copy3, cxx:_mju_copy4, cxx:_mju_mulPose, cxx:_mju_negPose
#[allow(unused_variables, non_snake_case)]
pub fn mjv_apply_perturb_pose(m: *const mjModel, d: *mut mjData, pert: *const mjvPerturb, flg_paused: i32) {
    todo!() // cxx:_mjv_applyPerturbPose
}

/// C: mjv_applyPerturbForce (engine/engine_vis_interact.c:717)
/// Calls: cxx:_mj_objectVelocity, cxx:_mju_addTo3, cxx:_mju_addToScl3, cxx:_mju_copy3, cxx:_mju_cross, cxx:_mju_dot3, cxx:_mju_max, cxx:_mju_mulMatVec3, cxx:_mju_mulQuat, cxx:_mju_negQuat, cxx:_mju_normalize3, cxx:_mju_quat2Vel, cxx:_mju_scl3, cxx:_mju_sub3
#[allow(unused_variables, non_snake_case)]
pub fn mjv_apply_perturb_force(m: *const mjModel, d: *mut mjData, pert: *const mjvPerturb) {
    todo!() // cxx:_mjv_applyPerturbForce
}

/// C: mjv_averageCamera (engine/engine_vis_interact.c:789)
/// Calls: cxx:_mju_add3, cxx:_mju_addToScl3, cxx:_mju_dot3, cxx:_mju_f2n, cxx:_mju_message, cxx:_mju_n2f, cxx:_mju_normalize3, cxx:_mju_scl3
#[allow(unused_variables, non_snake_case)]
pub fn mjv_average_camera(cam1: *const mjvGLCamera, cam2: *const mjvGLCamera) -> mjvGLCamera {
    todo!() // cxx:_mjv_averageCamera
}

/// C: mjv_camera2GLCamera (engine/engine_vis_interact.c:838)
/// Calls: cxx:_mjv_cameraFrame, cxx:_mjv_cameraFrustum
#[allow(unused_variables, non_snake_case)]
pub fn mjv_camera2gl_camera(model: *const mjModel, data: *const mjData, mjv_camera: *const mjvCamera) -> mjvGLCamera {
    todo!() // cxx:_mjv_camera2GLCamera
}

/// C: mjv_flexBodyId (engine/engine_vis_interact.c:869)
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

/// C: mjv_select (engine/engine_vis_interact.c:918)
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

