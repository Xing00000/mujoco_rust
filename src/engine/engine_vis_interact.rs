//! Port of: engine/engine_vis_interact.c
//! IR hash: 73a9f665ec0ecfc0
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
    // mjtMouse enum: ROTATE_V=1, ROTATE_H=2, MOVE_V=3, MOVE_H=4, ZOOM=5, MOVE_V_REL=6, MOVE_H_REL=7
    let mut vec = [0.0f64; 3];

    match action {
        1 => { // mjMOUSE_ROTATE_V
            vec[0] = dy;
            vec[1] = 0.0;
            vec[2] = dx;
        }
        2 => { // mjMOUSE_ROTATE_H
            vec[0] = dy;
            vec[1] = dx;
            vec[2] = 0.0;
        }
        3 | 6 => { // mjMOUSE_MOVE_V | mjMOUSE_MOVE_V_REL
            vec[0] = dx;
            vec[1] = 0.0;
            vec[2] = -dy;
        }
        4 | 7 => { // mjMOUSE_MOVE_H | mjMOUSE_MOVE_H_REL
            vec[0] = dx;
            vec[1] = -dy;
            vec[2] = 0.0;
        }
        5 => { // mjMOUSE_ZOOM
            // no-op
        }
        _ => {
            crate::engine::engine_util_errmem::mju_error(
                b"unexpected mouse action %d in convert2D\0".as_ptr() as *const i8);
        }
    }

    // call 3D converter
    mjv_align_to_camera(res, vec.as_ptr(), forward);
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
            crate::engine::engine_util_spatial::mju_neg_pose(
                invpos.as_mut_ptr(), invquat.as_mut_ptr(),
                translate.as_ptr(), rotate.as_ptr());

            // map from room to model space
            crate::engine::engine_util_spatial::mju_mul_pose(
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

/// C: mjv_model2room (engine/engine_vis_interact.h:32)
/// Calls: mju_copy3, mju_copy4, mju_f2n, mju_message, mju_mulPose, mju_scl3
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
            crate::engine::engine_util_spatial::mju_mul_pose(
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

/// C: mjv_cameraInModel (engine/engine_vis_interact.h:36)
/// Calls: mju_addToScl3, mju_cross, mju_f2n, mju_mat2Quat, mju_message, mju_normalize3, mju_quat2Mat, mju_zero3, mjv_room2model
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjv_camera_in_model(headpos: *mut f64, forward: *mut f64, up: *mut f64, scn: *const mjvScene) {
    const MJMINVAL: f64 = 1e-15;

    // SAFETY: scn is a valid pointer (caller contract). headpos, forward, up may be null.
    unsafe {
        // check znear
        if ((*scn).camera[0].frustum_near as f64) < MJMINVAL
            || ((*scn).camera[1].frustum_near as f64) < MJMINVAL
        {
            crate::engine::engine_util_errmem::mju_error(
                b"mjvScene frustum_near too small\0".as_ptr() as *const i8);
        }

        // clear results
        if !headpos.is_null() {
            crate::engine::engine_util_blas::mju_zero3(headpos);
        }
        if !forward.is_null() {
            crate::engine::engine_util_blas::mju_zero3(forward);
        }
        if !up.is_null() {
            crate::engine::engine_util_blas::mju_zero3(up);
        }

        // average over cameras
        for n in 0..2usize {
            let mut pos: [f64; 3] = [0.0; 3];
            let mut fwd: [f64; 3] = [0.0; 3];
            let mut u: [f64; 3] = [0.0; 3];
            let mut quat: [f64; 4] = [0.0; 4];
            let mut modelpos: [f64; 3] = [0.0; 3];
            let mut modelquat: [f64; 4] = [0.0; 4];
            let mut modelmat: [f64; 9] = [0.0; 9];

            // convert pos, fwd, u
            crate::engine::engine_util_misc::mju_f2n(
                pos.as_mut_ptr(), (*scn).camera[n].pos.as_ptr(), 3);
            crate::engine::engine_util_misc::mju_f2n(
                fwd.as_mut_ptr(), (*scn).camera[n].forward.as_ptr(), 3);
            crate::engine::engine_util_misc::mju_f2n(
                u.as_mut_ptr(), (*scn).camera[n].up.as_ptr(), 3);

            // normalize just in case
            crate::engine::engine_util_blas::mju_normalize3(fwd.as_mut_ptr());
            crate::engine::engine_util_blas::mju_normalize3(u.as_mut_ptr());

            // make orientation matrix: x = left, y = up, z = forward
            let mut left: [f64; 3] = [0.0; 3];
            crate::engine::engine_util_spatial::mju_cross(
                left.as_mut_ptr(), u.as_ptr(), fwd.as_ptr());
            crate::engine::engine_util_blas::mju_normalize3(left.as_mut_ptr());
            let mat: [f64; 9] = [
                left[0], u[0], fwd[0],
                left[1], u[1], fwd[1],
                left[2], u[2], fwd[2],
            ];
            crate::engine::engine_util_spatial::mju_mat2quat(quat.as_mut_ptr(), mat.as_ptr());

            // convert to model space, make orientation matrix
            mjv_room2model(
                modelpos.as_mut_ptr(), modelquat.as_mut_ptr(),
                pos.as_ptr(), quat.as_ptr(), scn);
            crate::engine::engine_util_spatial::mju_quat2mat(
                modelmat.as_mut_ptr(), modelquat.as_ptr());

            // finalize results
            if !headpos.is_null() {
                crate::engine::engine_util_blas::mju_add_to_scl3(headpos, modelpos.as_ptr(), 0.5);
            }
            if !forward.is_null() {
                *forward.add(0) += 0.5 * modelmat[2];
                *forward.add(1) += 0.5 * modelmat[5];
                *forward.add(2) += 0.5 * modelmat[8];
            }
            if !up.is_null() {
                *up.add(0) += 0.5 * modelmat[1];
                *up.add(1) += 0.5 * modelmat[4];
                *up.add(2) += 0.5 * modelmat[7];
            }
        }

        // normalize forward and up
        if !forward.is_null() {
            crate::engine::engine_util_blas::mju_normalize3(forward);
        }
        if !up.is_null() {
            crate::engine::engine_util_blas::mju_normalize3(up);
        }
    }
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
    const MJMINVAL: f64 = 1e-15;

    // SAFETY: scn is a valid pointer (caller contract). headpos, forward, up may be null.
    unsafe {
        // check znear
        if ((*scn).camera[0].frustum_near as f64) < MJMINVAL
            || ((*scn).camera[1].frustum_near as f64) < MJMINVAL
        {
            crate::engine::engine_util_errmem::mju_error(
                b"mjvScene frustum_near too small\0".as_ptr() as *const i8);
        }

        // clear results
        if !headpos.is_null() {
            crate::engine::engine_util_blas::mju_zero3(headpos);
        }
        if !forward.is_null() {
            crate::engine::engine_util_blas::mju_zero3(forward);
        }
        if !up.is_null() {
            crate::engine::engine_util_blas::mju_zero3(up);
        }

        // average over cameras
        for n in 0..2usize {
            let mut pos: [f64; 3] = [0.0; 3];
            let mut fwd: [f64; 3] = [0.0; 3];
            let mut u: [f64; 3] = [0.0; 3];

            // convert pos, fwd, u
            crate::engine::engine_util_misc::mju_f2n(
                pos.as_mut_ptr(), (*scn).camera[n].pos.as_ptr(), 3);
            crate::engine::engine_util_misc::mju_f2n(
                fwd.as_mut_ptr(), (*scn).camera[n].forward.as_ptr(), 3);
            crate::engine::engine_util_misc::mju_f2n(
                u.as_mut_ptr(), (*scn).camera[n].up.as_ptr(), 3);

            // finalize results
            if !headpos.is_null() {
                crate::engine::engine_util_blas::mju_add_to_scl3(headpos, pos.as_ptr(), 0.5);
            }
            if !forward.is_null() {
                crate::engine::engine_util_blas::mju_add_to_scl3(forward, fwd.as_ptr(), 0.5);
            }
            if !up.is_null() {
                crate::engine::engine_util_blas::mju_add_to_scl3(up, u.as_ptr(), 0.5);
            }
        }

        // normalize
        if !forward.is_null() {
            crate::engine::engine_util_blas::mju_normalize3(forward);
        }
        if !up.is_null() {
            crate::engine::engine_util_blas::mju_normalize3(up);
        }
    }
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
    const MJMINVAL: f64 = 1e-15;

    // SAFETY: scn is a valid pointer (caller contract).
    unsafe {
        let cam1 = &(*scn).camera[0];
        let cam2 = &(*scn).camera[1];

        if cam1.orthographic != cam2.orthographic {
            crate::engine::engine_util_errmem::mju_error(
                b"cannot average frustums of perspective and orthographic cameras\0".as_ptr() as *const i8);
        }

        // get height
        let height: f64;
        if cam1.orthographic == 0 {
            // check znear
            if (cam1.frustum_near as f64) < MJMINVAL || (cam2.frustum_near as f64) < MJMINVAL {
                crate::engine::engine_util_errmem::mju_error(
                    b"mjvScene frustum_near too small\0".as_ptr() as *const i8);
            }

            // add normalized height for left and right cameras
            height = (cam1.frustum_top - cam1.frustum_bottom) as f64 / cam1.frustum_near as f64
                + (cam2.frustum_top - cam2.frustum_bottom) as f64 / cam2.frustum_near as f64;
        } else {
            // add height for left and right cameras
            height = (cam1.frustum_top - cam1.frustum_bottom) as f64
                + (cam2.frustum_top - cam2.frustum_bottom) as f64;
        }

        // average
        0.5 * height
    }
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
    // SAFETY: res[3], vec[3], forward[3] valid (caller contract)
    unsafe {
        // forward-aligned y-axis
        let mut yaxis: [f64; 2] = [*forward.add(0), *forward.add(1)];
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
    const MJ_CAMERA_FIXED: i32 = 2;
    const MJ_CAMERA_TRACKING: i32 = 1;
    const MJ_MOUSE_ROTATE_V: i32 = 1;
    const MJ_MOUSE_ROTATE_H: i32 = 2;
    const MJ_MOUSE_MOVE_V: i32 = 3;
    const MJ_MOUSE_MOVE_H: i32 = 4;
    const MJ_MOUSE_ZOOM: i32 = 5;
    const MJ_MOUSE_MOVE_V_REL: i32 = 6;
    const MJ_MOUSE_MOVE_H_REL: i32 = 7;

    // SAFETY: m, scn, cam are valid pointers (caller contract).
    unsafe {
        // fixed camera: nothing to do
        if (*cam).r#type == MJ_CAMERA_FIXED {
            return;
        }

        if action == MJ_MOUSE_ROTATE_V || action == MJ_MOUSE_ROTATE_H {
            (*cam).azimuth -= reldx * 180.0;
            (*cam).elevation -= reldy * 180.0;
        } else if action == MJ_MOUSE_MOVE_V || action == MJ_MOUSE_MOVE_H {
            // do not move lookat point of tracking camera
            if (*cam).r#type == MJ_CAMERA_TRACKING {
                return;
            }

            // get camera info and align
            let mut headpos: [f64; 3] = [0.0; 3];
            let mut forward: [f64; 3] = [0.0; 3];
            let mut vec: [f64; 3] = [0.0; 3];
            let mut dif: [f64; 3] = [0.0; 3];
            mjv_camera_in_model(headpos.as_mut_ptr(), forward.as_mut_ptr(), std::ptr::null_mut(), scn);
            convert2d(vec.as_mut_ptr(), action, reldx, reldy, forward.as_ptr());

            // compute scaling
            crate::engine::engine_util_blas::mju_sub3(
                dif.as_mut_ptr(), (*cam).lookat.as_ptr(), headpos.as_ptr());
            let mut scl = mjv_frustum_height(scn)
                * crate::engine::engine_util_blas::mju_dot3(dif.as_ptr(), forward.as_ptr());

            // mystery coefficient
            if (*cam).orthographic != 0 {
                scl *= 0.15;
            }

            // move lookat point in opposite direction
            crate::engine::engine_util_blas::mju_add_to_scl3(
                (*cam).lookat.as_mut_ptr(), vec.as_ptr(), -scl);
        } else if action == MJ_MOUSE_ZOOM {
            (*cam).distance -= f64::ln(1.0 + (*cam).distance / (*m).stat.extent / 3.0)
                * reldy * 9.0 * (*m).stat.extent;
        } else if action == MJ_MOUSE_MOVE_V_REL || action == MJ_MOUSE_MOVE_H_REL {
            // do not move lookat point of tracking camera
            if (*cam).r#type == MJ_CAMERA_TRACKING {
                return;
            }

            let mut headpos: [f64; 3] = [0.0; 3];
            let mut forward: [f64; 3] = [0.0; 3];
            let mut up: [f64; 3] = [0.0; 3];
            let mut right: [f64; 3] = [0.0; 3];
            mjv_camera_in_model(
                headpos.as_mut_ptr(), forward.as_mut_ptr(), up.as_mut_ptr(), scn);
            crate::engine::engine_util_spatial::mju_cross(
                right.as_mut_ptr(), forward.as_ptr(), up.as_ptr());

            // y-axis: forward/up; x-axis: right
            let y_vec: *const f64 = if action == MJ_MOUSE_MOVE_V_REL {
                up.as_ptr()
            } else {
                forward.as_ptr()
            };
            crate::engine::engine_util_blas::mju_add_to_scl3(
                (*cam).lookat.as_mut_ptr(), y_vec, reldy);
            crate::engine::engine_util_blas::mju_add_to_scl3(
                (*cam).lookat.as_mut_ptr(), right.as_ptr(), reldx);
        } else {
            crate::engine::engine_util_errmem::mju_error(
                b"unexpected action in mjv_moveCamera\0".as_ptr() as *const i8);
        }

        // clamp camera parameters
        if (*cam).azimuth > 180.0 {
            (*cam).azimuth -= 360.0;
        }
        if (*cam).azimuth < -180.0 {
            (*cam).azimuth += 360.0;
        }
        if (*cam).elevation > 89.0 {
            (*cam).elevation = 89.0;
        }
        if (*cam).elevation < -89.0 {
            (*cam).elevation = -89.0;
        }
        if (*cam).distance < 0.01 * (*m).stat.extent {
            (*cam).distance = 0.01 * (*m).stat.extent;
        }
        if (*cam).distance > 100.0 * (*m).stat.extent {
            (*cam).distance = 100.0 * (*m).stat.extent;
        }
    }
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
    todo!() // mjv_movePerturb
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
    const MJ_MOUSE_ROTATE_V: i32 = 1;
    const MJ_MOUSE_ROTATE_H: i32 = 2;
    const MJ_MOUSE_MOVE_V: i32 = 3;
    const MJ_MOUSE_MOVE_H: i32 = 4;
    const MJ_MOUSE_ZOOM: i32 = 5;
    const MJ_MOUSE_MOVE_V_REL: i32 = 6;
    const MJ_MOUSE_MOVE_H_REL: i32 = 7;
    const MJ_PI: f64 = std::f64::consts::PI;

    // SAFETY: m, roomup, scn are valid pointers (caller contract).
    unsafe {
        // transformation disabled: nothing to do
        if (*scn).enabletransform == 0 {
            return;
        }

        // get camera forward in room space
        let mut camforward: [f64; 3] = [0.0; 3];
        mjv_camera_in_room(std::ptr::null_mut(), camforward.as_mut_ptr(), std::ptr::null_mut(), scn);

        // make orthogonal to roomright
        let mut roomforward: [f64; 3] = [0.0; 3];
        crate::engine::engine_util_blas::mju_add_scl3(
            roomforward.as_mut_ptr(), camforward.as_ptr(), roomup,
            -crate::engine::engine_util_blas::mju_dot3(camforward.as_ptr(), roomup));
        crate::engine::engine_util_blas::mju_normalize3(roomforward.as_mut_ptr());

        // compute roomright
        let mut roomright: [f64; 3] = [0.0; 3];
        crate::engine::engine_util_spatial::mju_cross(
            roomright.as_mut_ptr(), roomforward.as_ptr(), roomup);
        crate::engine::engine_util_blas::mju_normalize3(roomright.as_mut_ptr());

        // process action
        if action == MJ_MOUSE_ROTATE_V || action == MJ_MOUSE_ROTATE_H {
            // construct rotation vector
            let mut vec: [f64; 3] = [0.0; 3];
            for i in 0..3usize {
                if action == MJ_MOUSE_ROTATE_V {
                    vec[i] = *roomup.add(i) * reldx + roomright[i] * reldy;
                } else {
                    vec[i] = roomforward[i] * reldx + roomright[i] * reldy;
                }
            }

            // make quaternion from angle-axis
            let scl = crate::engine::engine_util_blas::mju_normalize3(vec.as_mut_ptr());
            let mut quat: [f64; 4] = [0.0; 4];
            crate::engine::engine_util_spatial::mju_axis_angle2quat(
                quat.as_mut_ptr(), vec.as_ptr(), scl * MJ_PI);

            // get current model rotation
            let mut rotate: [f64; 4] = [0.0; 4];
            crate::engine::engine_util_misc::mju_f2n(
                rotate.as_mut_ptr(), (*scn).rotate.as_ptr(), 4);

            // compose rotation, normalize and set
            let mut result: [f64; 4] = [0.0; 4];
            crate::engine::engine_util_spatial::mju_mul_quat(
                result.as_mut_ptr(), quat.as_ptr(), rotate.as_ptr());
            crate::engine::engine_util_blas::mju_normalize4(result.as_mut_ptr());
            crate::engine::engine_util_misc::mju_n2f(
                (*scn).rotate.as_mut_ptr(), result.as_ptr(), 4);
        } else if action == MJ_MOUSE_MOVE_V || action == MJ_MOUSE_MOVE_V_REL {
            let extent = (*m).stat.extent as f32;
            for i in 0..3usize {
                (*scn).translate[i] += (roomright[i] * reldx - *roomup.add(i) * reldy) as f32 * extent;
            }
        } else if action == MJ_MOUSE_MOVE_H || action == MJ_MOUSE_MOVE_H_REL {
            let extent = (*m).stat.extent as f32;
            for i in 0..3usize {
                (*scn).translate[i] += (roomright[i] * reldx - roomforward[i] * reldy) as f32 * extent;
            }
        } else if action == MJ_MOUSE_ZOOM {
            (*scn).scale += (f64::ln(1.0 + (*scn).scale as f64 / 3.0) * reldy * 3.0) as f32;
            if (*scn).scale < 0.01 {
                (*scn).scale = 0.01;
            } else if (*scn).scale > 100.0 {
                (*scn).scale = 100.0;
            }
        } else {
            crate::engine::engine_util_errmem::mju_error(
                b"unexpected action in mjv_moveModel\0".as_ptr() as *const i8);
        }
    }
}

/// C: mjv_initPerturb (engine/engine_vis_interact.h:62)
/// Calls: mj_freeStack, mj_jac, mj_markStack, mj_solveM2, mj_stackAllocInfo, mju_addTo3, mju_copy3, mju_dot, mju_dot3, mju_max, mju_mulMatVec3, mju_mulQuat, mju_sub3, mjv_cameraInModel, mjv_frustumHeight
#[allow(unused_variables, non_snake_case)]
pub fn mjv_init_perturb(m: *const mjModel, d: *mut mjData, scn: *const mjvScene, pert: *mut mjvPerturb) {
    todo!() // mjv_initPerturb
}

/// C: mjv_applyPerturbPose (engine/engine_vis_interact.h:66)
/// Calls: mju_copy3, mju_copy4, mju_mulPose, mju_negPose
#[allow(unused_variables, non_snake_case)]
pub fn mjv_apply_perturb_pose(m: *const mjModel, d: *mut mjData, pert: *const mjvPerturb, flg_paused: i32) {
    todo!() // mjv_applyPerturbPose
}

/// C: mjv_applyPerturbForce (engine/engine_vis_interact.h:70)
/// Calls: mj_objectVelocity, mju_addTo3, mju_addToScl3, mju_copy3, mju_cross, mju_dot3, mju_max, mju_mulMatVec3, mju_mulQuat, mju_negQuat, mju_normalize3, mju_quat2Vel, mju_scl3, mju_sub3
#[allow(unused_variables, non_snake_case)]
pub fn mjv_apply_perturb_force(m: *const mjModel, d: *mut mjData, pert: *const mjvPerturb) {
    todo!() // mjv_applyPerturbForce
}

/// C: mjv_averageCamera (engine/engine_vis_interact.h:73)
/// Calls: mju_add3, mju_addToScl3, mju_dot3, mju_f2n, mju_message, mju_n2f, mju_normalize3, mju_scl3
#[allow(unused_variables, non_snake_case)]
pub fn mjv_average_camera(cam1: *const mjvGLCamera, cam2: *const mjvGLCamera) -> mjvGLCamera {
    // SAFETY: cam1, cam2 are valid pointers (caller contract).
    unsafe {
        let mut pos: [f64; 3] = [0.0; 3];
        let mut forward: [f64; 3] = [0.0; 3];
        let mut up: [f64; 3] = [0.0; 3];
        let mut tmp1: [f64; 3] = [0.0; 3];
        let mut tmp2: [f64; 3] = [0.0; 3];

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
        let projection = crate::engine::engine_util_blas::mju_dot3(up.as_ptr(), forward.as_ptr());
        crate::engine::engine_util_blas::mju_add_to_scl3(up.as_mut_ptr(), forward.as_ptr(), -projection);
        crate::engine::engine_util_blas::mju_normalize3(up.as_mut_ptr());

        // build result camera
        let mut cam = mjvGLCamera {
            pos: [0.0; 3],
            forward: [0.0; 3],
            up: [0.0; 3],
            frustum_center: 0.0,
            frustum_width: 0.0,
            frustum_bottom: 0.0,
            frustum_top: 0.0,
            frustum_near: 0.0,
            frustum_far: 0.0,
            orthographic: 0,
        };
        crate::engine::engine_util_misc::mju_n2f(cam.pos.as_mut_ptr(), pos.as_ptr(), 3);
        crate::engine::engine_util_misc::mju_n2f(cam.forward.as_mut_ptr(), forward.as_ptr(), 3);
        crate::engine::engine_util_misc::mju_n2f(cam.up.as_mut_ptr(), up.as_ptr(), 3);

        // average frustum
        cam.frustum_bottom = 0.5 * ((*cam1).frustum_bottom + (*cam2).frustum_bottom);
        cam.frustum_top = 0.5 * ((*cam1).frustum_top + (*cam2).frustum_top);
        cam.frustum_center = 0.5 * ((*cam1).frustum_center + (*cam2).frustum_center);
        cam.frustum_width = 0.5 * ((*cam1).frustum_width + (*cam2).frustum_width);
        cam.frustum_near = 0.5 * ((*cam1).frustum_near + (*cam2).frustum_near);
        cam.frustum_far = 0.5 * ((*cam1).frustum_far + (*cam2).frustum_far);

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
pub fn mjv_select(m: *const mjModel, d: *const mjData, vopt: *const mjvOption, aspectratio: f64, relx: f64, rely: f64, scn: *const mjvScene, selpnt: *mut f64, geomid: *mut i32, flexid: *mut i32, skinid: *mut i32) -> i32 {
    todo!() // mjv_select
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
    todo!() // mjv_flexBodyId
}

