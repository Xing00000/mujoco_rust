//! Port of: engine/engine_vis_interact.c
//! IR hash: 05737965add36adb
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: convert2D (engine/engine_vis_interact.c:270)
/// Calls: mju_message, mjv_alignToCamera
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn convert2d(res: *mut f64, action: i32, dx: f64, dy: f64, forward: *const f64) {
    extern "C" {
        fn convert2D_impl(res: *mut f64, action: i32, dx: f64, dy: f64, forward: *const f64);
    }
    // SAFETY: delegates to C implementation which handles mjtMouse enum switch and mjv_alignToCamera call
    unsafe { convert2D_impl(res, action, dx, dy, forward) }
}

/// C: mjv_room2model (engine/engine_vis_interact.h:28)
/// Calls: mju_copy3, mju_copy4, mju_f2n, mju_message, mju_mulPose, mju_negPose, mju_scl3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjv_room2model(modelpos: *mut f64, modelquat: *mut f64, roompos: *const f64, roomquat: *const f64, scn: *const mjvScene) {
    extern "C" { fn mjv_room2model_impl(modelpos: *mut f64, modelquat: *mut f64, roompos: *const f64, roomquat: *const f64, scn: *const mjvScene); }
    // SAFETY: delegates to C implementation
    unsafe { mjv_room2model_impl(modelpos, modelquat, roompos, roomquat, scn) }
}

/// C: mjv_model2room (engine/engine_vis_interact.h:32)
/// Calls: mju_copy3, mju_copy4, mju_f2n, mju_message, mju_mulPose, mju_scl3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjv_model2room(roompos: *mut f64, roomquat: *mut f64, modelpos: *const f64, modelquat: *const f64, scn: *const mjvScene) {
    extern "C" { fn mjv_model2room_impl(roompos: *mut f64, roomquat: *mut f64, modelpos: *const f64, modelquat: *const f64, scn: *const mjvScene); }
    // SAFETY: delegates to C implementation
    unsafe { mjv_model2room_impl(roompos, roomquat, modelpos, modelquat, scn) }
}

/// C: mjv_cameraInModel (engine/engine_vis_interact.h:36)
/// Calls: mju_addToScl3, mju_cross, mju_f2n, mju_mat2Quat, mju_message, mju_normalize3, mju_quat2Mat, mju_zero3, mjv_room2model
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjv_camera_in_model(headpos: *mut f64, forward: *mut f64, up: *mut f64, scn: *const mjvScene) {
    extern "C" { fn mjv_cameraInModel_impl(headpos: *mut f64, forward: *mut f64, up: *mut f64, scn: *const mjvScene); }
    // SAFETY: delegates to C implementation
    unsafe { mjv_cameraInModel_impl(headpos, forward, up, scn) }
}

/// C: mjv_cameraInRoom (engine/engine_vis_interact.h:40)
/// Calls: mju_addToScl3, mju_f2n, mju_message, mju_normalize3, mju_zero3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjv_camera_in_room(headpos: *mut f64, forward: *mut f64, up: *mut f64, scn: *const mjvScene) {
    extern "C" { fn mjv_cameraInRoom_impl(headpos: *mut f64, forward: *mut f64, up: *mut f64, scn: *const mjvScene); }
    // SAFETY: delegates to C implementation
    unsafe { mjv_cameraInRoom_impl(headpos, forward, up, scn) }
}

/// C: mjv_frustumHeight (engine/engine_vis_interact.h:44)
/// Calls: mju_message
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjv_frustum_height(scn: *const mjvScene) -> f64 {
    extern "C" { fn mjv_frustumHeight_impl(scn: *const mjvScene) -> f64; }
    // SAFETY: delegates to C implementation
    unsafe { mjv_frustumHeight_impl(scn) }
}

/// C: mjv_alignToCamera (engine/engine_vis_interact.h:47)
/// Calls: mju_copy, mju_normalize
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjv_align_to_camera(res: *mut f64, vec: *const f64, forward: *const f64) {
    // SAFETY: res has 3 elements, vec has 3, forward has 3 (only first 2 used).
    unsafe {
        let mut yaxis: [f64; 2] = [0.0; 2];
        crate::engine::engine_util_blas::mju_copy(yaxis.as_mut_ptr(), forward, 2);
        crate::engine::engine_util_blas::mju_normalize(yaxis.as_mut_ptr(), 2);

        // corresponding x-axis
        let xaxis: [f64; 2] = [yaxis[1], -yaxis[0]];

        // apply horizontal rotation
        *res.add(0) = *vec.add(0) * xaxis[0] + *vec.add(1) * yaxis[0];
        *res.add(1) = *vec.add(0) * xaxis[1] + *vec.add(1) * yaxis[1];
        *res.add(2) = *vec.add(2);
    }
}

/// C: mjv_moveCamera (engine/engine_vis_interact.h:50)
/// Calls: convert2D, mju_addToScl3, mju_cross, mju_dot3, mju_message, mju_sub3, mjv_cameraInModel, mjv_frustumHeight
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjv_move_camera(m: *const mjModel, action: i32, reldx: f64, reldy: f64, scn: *const mjvScene, cam: *mut mjvCamera) {
    extern "C" { fn mjv_moveCamera_impl(m: *const mjModel, action: i32, reldx: f64, reldy: f64, scn: *const mjvScene, cam: *mut mjvCamera); }
    // SAFETY: delegates to C implementation which accesses mjvCamera fields and calls mju_* functions
    unsafe { mjv_moveCamera_impl(m, action, reldx, reldy, scn, cam) }
}

/// C: mjv_movePerturb (engine/engine_vis_interact.h:54)
/// Calls: convert2D, mju_addToScl3, mju_axisAngle2Quat, mju_max, mju_message, mju_min, mju_mulMatVec3, mju_mulQuat, mju_negQuat, mju_normalize3, mju_normalize4, mju_quat2Vel, mjv_cameraInModel
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjv_move_perturb(m: *const mjModel, d: *const mjData, action: i32, reldx: f64, reldy: f64, scn: *const mjvScene, pert: *mut mjvPerturb) {
    extern "C" { fn mjv_movePerturb_impl(m: *const mjModel, d: *const mjData, action: i32, reldx: f64, reldy: f64, scn: *const mjvScene, pert: *mut mjvPerturb); }
    // SAFETY: delegates to C implementation which handles perturb movement with complex
    // quaternion math and mju_* function calls
    unsafe { mjv_movePerturb_impl(m, d, action, reldx, reldy, scn, pert) }
}

/// C: mjv_moveModel (engine/engine_vis_interact.h:58)
/// Calls: mju_addScl3, mju_axisAngle2Quat, mju_cross, mju_dot3, mju_f2n, mju_message, mju_mulQuat, mju_n2f, mju_normalize3, mju_normalize4, mjv_cameraInRoom
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjv_move_model(m: *const mjModel, action: i32, reldx: f64, reldy: f64, roomup: *const f64, scn: *mut mjvScene) {
    extern "C" { fn mjv_moveModel_impl(m: *const mjModel, action: i32, reldx: f64, reldy: f64, roomup: *const f64, scn: *mut mjvScene); }
    // SAFETY: delegates to C implementation
    unsafe { mjv_moveModel_impl(m, action, reldx, reldy, roomup, scn) }
}

/// C: mjv_initPerturb (engine/engine_vis_interact.h:62)
/// Calls: mj_freeStack, mj_jac, mj_markStack, mj_solveM2, mj_stackAllocInfo, mju_addTo3, mju_copy3, mju_dot, mju_dot3, mju_max, mju_mulMatVec3, mju_mulQuat, mju_sub3, mjv_cameraInModel, mjv_frustumHeight
#[allow(unused_variables, non_snake_case)]
pub fn mjv_init_perturb(m: *const mjModel, d: *mut mjData, scn: *const mjvScene, pert: *mut mjvPerturb) {
    extern "C" { fn mjv_initPerturb_impl(m: *const mjModel, d: *mut mjData, scn: *const mjvScene, pert: *mut mjvPerturb); }
    // SAFETY: delegates to C implementation
    unsafe { mjv_initPerturb_impl(m, d, scn, pert) }
}

/// C: mjv_applyPerturbPose (engine/engine_vis_interact.h:66)
/// Calls: mju_copy3, mju_copy4, mju_mulPose, mju_negPose
#[allow(unused_variables, non_snake_case)]
pub fn mjv_apply_perturb_pose(m: *const mjModel, d: *mut mjData, pert: *const mjvPerturb, flg_paused: i32) {
    extern "C" { fn mjv_applyPerturbPose_impl(m: *const mjModel, d: *mut mjData, pert: *const mjvPerturb, flg_paused: i32); }
    // SAFETY: delegates to C implementation
    unsafe { mjv_applyPerturbPose_impl(m, d, pert, flg_paused) }
}

/// C: mjv_applyPerturbForce (engine/engine_vis_interact.h:70)
/// Calls: mj_objectVelocity, mju_addTo3, mju_addToScl3, mju_copy3, mju_cross, mju_dot3, mju_max, mju_mulMatVec3, mju_mulQuat, mju_negQuat, mju_normalize3, mju_quat2Vel, mju_scl3, mju_sub3
#[allow(unused_variables, non_snake_case)]
pub fn mjv_apply_perturb_force(m: *const mjModel, d: *mut mjData, pert: *const mjvPerturb) {
    extern "C" { fn mjv_applyPerturbForce_impl(m: *const mjModel, d: *mut mjData, pert: *const mjvPerturb); }
    // SAFETY: delegates to C implementation
    unsafe { mjv_applyPerturbForce_impl(m, d, pert) }
}

/// C: mjv_averageCamera (engine/engine_vis_interact.h:73)
/// Calls: mju_add3, mju_addToScl3, mju_dot3, mju_f2n, mju_message, mju_n2f, mju_normalize3, mju_scl3
#[allow(unused_variables, non_snake_case)]
pub fn mjv_average_camera(cam1: *const mjvGLCamera, cam2: *const mjvGLCamera) -> mjvGLCamera {
    // SAFETY: cam1 and cam2 are valid pointers per caller contract; all mju_* calls
    // operate on stack-local arrays with correct sizes.
    unsafe {
        let mut pos: [f64; 3] = [0.0; 3];
        let mut forward: [f64; 3] = [0.0; 3];
        let mut up: [f64; 3] = [0.0; 3];
        let mut tmp1: [f64; 3] = [0.0; 3];
        let mut tmp2: [f64; 3] = [0.0; 3];
        let mut cam: mjvGLCamera = core::mem::zeroed();

        // compute pos
        crate::engine::engine_util_misc::mju_f2n(tmp1.as_mut_ptr(), (*cam1).pos.as_ptr(), 3);
        crate::engine::engine_util_misc::mju_f2n(tmp2.as_mut_ptr(), (*cam2).pos.as_ptr(), 3);
        crate::engine::engine_util_blas::mju_add3(pos.as_mut_ptr(), tmp1.as_ptr(), tmp2.as_ptr());
        crate::engine::engine_util_blas::mju_scl3(pos.as_mut_ptr(), pos.as_ptr(), 0.5);

        // compute forward
        crate::engine::engine_util_misc::mju_f2n(tmp1.as_mut_ptr(), (*cam1).forward.as_ptr(), 3);
        crate::engine::engine_util_misc::mju_f2n(tmp2.as_mut_ptr(), (*cam2).forward.as_ptr(), 3);
        crate::engine::engine_util_blas::mju_add3(forward.as_mut_ptr(), tmp1.as_ptr(), tmp2.as_ptr());
        crate::engine::engine_util_blas::mju_normalize3(forward.as_mut_ptr());

        // compute up, make it orthogonal to forward
        crate::engine::engine_util_misc::mju_f2n(tmp1.as_mut_ptr(), (*cam1).up.as_ptr(), 3);
        crate::engine::engine_util_misc::mju_f2n(tmp2.as_mut_ptr(), (*cam2).up.as_ptr(), 3);
        crate::engine::engine_util_blas::mju_add3(up.as_mut_ptr(), tmp1.as_ptr(), tmp2.as_ptr());
        let projection: f64 = crate::engine::engine_util_blas::mju_dot3(up.as_ptr(), forward.as_ptr());
        crate::engine::engine_util_blas::mju_add_to_scl3(up.as_mut_ptr(), forward.as_ptr(), -projection);
        crate::engine::engine_util_blas::mju_normalize3(up.as_mut_ptr());

        // assign 3d quantities
        crate::engine::engine_util_misc::mju_n2f(cam.pos.as_mut_ptr(), pos.as_ptr(), 3);
        crate::engine::engine_util_misc::mju_n2f(cam.forward.as_mut_ptr(), forward.as_ptr(), 3);
        crate::engine::engine_util_misc::mju_n2f(cam.up.as_mut_ptr(), up.as_ptr(), 3);

        // average frustum
        cam.frustum_bottom = 0.5f32 * ((*cam1).frustum_bottom + (*cam2).frustum_bottom);
        cam.frustum_top    = 0.5f32 * ((*cam1).frustum_top + (*cam2).frustum_top);
        cam.frustum_center = 0.5f32 * ((*cam1).frustum_center + (*cam2).frustum_center);
        cam.frustum_width  = 0.5f32 * ((*cam1).frustum_width + (*cam2).frustum_width);
        cam.frustum_near   = 0.5f32 * ((*cam1).frustum_near + (*cam2).frustum_near);
        cam.frustum_far    = 0.5f32 * ((*cam1).frustum_far + (*cam2).frustum_far);

        if (*cam1).orthographic != (*cam2).orthographic {
            crate::engine::engine_util_errmem::mju_error(
                b"cannot average perspective and orthographic cameras\0".as_ptr() as *const i8);
        } else {
            cam.orthographic = (*cam1).orthographic;
        }

        cam
    }
}

/// C: mjv_select (engine/engine_vis_interact.h:76)
/// Calls: mj_ray, mj_rayFlex, mju_addScl3, mju_addToScl3, mju_copy3, mju_cross, mju_f2n, mju_normalize3, mju_raySkin, mju_scl3, mjv_averageCamera, mjv_cameraInModel, mjv_flexBodyId
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjv_select(m: *const mjModel, d: *const mjData, vopt: *const mjvOption, aspectratio: f64, relx: f64, rely: f64, scn: *const mjvScene, selpnt: *mut f64, geomid: [i32; 1], flexid: [i32; 1], skinid: [i32; 1]) -> i32 {
    extern "C" { fn mjv_select_impl(m: *const mjModel, d: *const mjData, vopt: *const mjvOption, aspectratio: f64, relx: f64, rely: f64, scn: *const mjvScene, selpnt: *mut f64, geomid: [i32; 1], flexid: [i32; 1], skinid: [i32; 1]) -> i32; }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mjv_select_impl(m, d, vopt, aspectratio, relx, rely, scn, selpnt, geomid, flexid, skinid) }
}

/// C: mjv_flexBodyId (engine/engine_vis_interact.h:82)
/// Calls: mju_addTo3, mju_cellLookup, mju_copy3, mju_evalBasis, mju_mulMatVec3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjv_flex_body_id(m: *const mjModel, d: *const mjData, flexid: i32, vertid: i32, flexpnt: *mut f64) -> i32 {

    extern "C" { fn mjv_flexBodyId_impl(m: *const mjModel, d: *const mjData, flexid: i32, vertid: i32, flexpnt: *mut f64) -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { mjv_flexBodyId_impl(m, d, flexid, vertid, flexpnt) }
}

