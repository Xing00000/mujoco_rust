//! Port of: engine/engine_vis_interact.c
//! IR hash: 9343293228317031
//! CODEGEN: source paths, owners, and callable names are locked.

use crate::types::*;

/// C: convert2D (engine/engine_vis_interact.c:270)
/// Calls: cxx:_mju_message, cxx:_mjv_alignToCamera
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn convert2D(res: *mut f64, action: i32, dx: f64, dy: f64, forward: *const f64) {
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
    mjv_alignToCamera(res, vec.as_ptr(), forward);
}

/// C: mjv_room2model (engine/engine_vis_interact.h:28)
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

/// C: mjv_model2room (engine/engine_vis_interact.h:32)
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

/// C: mjv_cameraInModel (engine/engine_vis_interact.h:36)
/// Calls: cxx:_mju_addToScl3, cxx:_mju_cross, cxx:_mju_f2n, cxx:_mju_mat2Quat, cxx:_mju_message, cxx:_mju_normalize3, cxx:_mju_quat2Mat, cxx:_mju_zero3, cxx:_mjv_room2model
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjv_cameraInModel(headpos: *mut f64, forward: *mut f64, up: *mut f64, scn: *const mjvScene) {
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
            crate::engine::engine_util_spatial::mju_mat2Quat(quat.as_mut_ptr(), mat.as_ptr());

            // convert to model space, make orientation matrix
            mjv_room2model(
                modelpos.as_mut_ptr(), modelquat.as_mut_ptr(),
                pos.as_ptr(), quat.as_ptr(), scn);
            crate::engine::engine_util_spatial::mju_quat2Mat(
                modelmat.as_mut_ptr(), modelquat.as_ptr());

            // finalize results
            if !headpos.is_null() {
                crate::engine::engine_util_blas::mju_addToScl3(headpos, modelpos.as_ptr(), 0.5);
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
/// Calls: cxx:_mju_addToScl3, cxx:_mju_f2n, cxx:_mju_message, cxx:_mju_normalize3, cxx:_mju_zero3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjv_cameraInRoom(headpos: *mut f64, forward: *mut f64, up: *mut f64, scn: *const mjvScene) {
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
                crate::engine::engine_util_blas::mju_addToScl3(headpos, pos.as_ptr(), 0.5);
            }
            if !forward.is_null() {
                crate::engine::engine_util_blas::mju_addToScl3(forward, fwd.as_ptr(), 0.5);
            }
            if !up.is_null() {
                crate::engine::engine_util_blas::mju_addToScl3(up, u.as_ptr(), 0.5);
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
/// Calls: cxx:_mju_message
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjv_frustumHeight(scn: *const mjvScene) -> f64 {
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
/// Calls: cxx:_mju_copy, cxx:_mju_normalize
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjv_alignToCamera(res: *mut f64, vec: *const f64, forward: *const f64) {
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
/// Calls: cxx-internal:engine_vis_interact.c.o:_convert2D, cxx:_mju_addToScl3, cxx:_mju_cross, cxx:_mju_dot3, cxx:_mju_message, cxx:_mju_sub3, cxx:_mjv_cameraInModel, cxx:_mjv_frustumHeight
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjv_moveCamera(m: *const mjModel, action: i32, reldx: f64, reldy: f64, scn: *const mjvScene, cam: *mut mjvCamera) {
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
            mjv_cameraInModel(headpos.as_mut_ptr(), forward.as_mut_ptr(), std::ptr::null_mut(), scn);
            convert2D(vec.as_mut_ptr(), action, reldx, reldy, forward.as_ptr());

            // compute scaling
            crate::engine::engine_util_blas::mju_sub3(
                dif.as_mut_ptr(), (*cam).lookat.as_ptr(), headpos.as_ptr());
            let mut scl = mjv_frustumHeight(scn)
                * crate::engine::engine_util_blas::mju_dot3(dif.as_ptr(), forward.as_ptr());

            // mystery coefficient
            if (*cam).orthographic != 0 {
                scl *= 0.15;
            }

            // move lookat point in opposite direction
            crate::engine::engine_util_blas::mju_addToScl3(
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
            mjv_cameraInModel(
                headpos.as_mut_ptr(), forward.as_mut_ptr(), up.as_mut_ptr(), scn);
            crate::engine::engine_util_spatial::mju_cross(
                right.as_mut_ptr(), forward.as_ptr(), up.as_ptr());

            // y-axis: forward/up; x-axis: right
            let y_vec: *const f64 = if action == MJ_MOUSE_MOVE_V_REL {
                up.as_ptr()
            } else {
                forward.as_ptr()
            };
            crate::engine::engine_util_blas::mju_addToScl3(
                (*cam).lookat.as_mut_ptr(), y_vec, reldy);
            crate::engine::engine_util_blas::mju_addToScl3(
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

/// C: mjv_moveModel (engine/engine_vis_interact.h:58)
/// Calls: cxx:_mju_addScl3, cxx:_mju_axisAngle2Quat, cxx:_mju_cross, cxx:_mju_dot3, cxx:_mju_f2n, cxx:_mju_message, cxx:_mju_mulQuat, cxx:_mju_n2f, cxx:_mju_normalize3, cxx:_mju_normalize4, cxx:_mjv_cameraInRoom
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjv_moveModel(m: *const mjModel, action: i32, reldx: f64, reldy: f64, roomup: *const f64, scn: *mut mjvScene) {
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
        mjv_cameraInRoom(std::ptr::null_mut(), camforward.as_mut_ptr(), std::ptr::null_mut(), scn);

        // make orthogonal to roomright
        let mut roomforward: [f64; 3] = [0.0; 3];
        crate::engine::engine_util_blas::mju_addScl3(
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
            crate::engine::engine_util_spatial::mju_axisAngle2Quat(
                quat.as_mut_ptr(), vec.as_ptr(), scl * MJ_PI);

            // get current model rotation
            let mut rotate: [f64; 4] = [0.0; 4];
            crate::engine::engine_util_misc::mju_f2n(
                rotate.as_mut_ptr(), (*scn).rotate.as_ptr(), 4);

            // compose rotation, normalize and set
            let mut result: [f64; 4] = [0.0; 4];
            crate::engine::engine_util_spatial::mju_mulQuat(
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

/// C: mjv_applyPerturbPose (engine/engine_vis_interact.h:66)
/// Calls: cxx:_mju_copy3, cxx:_mju_copy4, cxx:_mju_mulPose, cxx:_mju_negPose
#[allow(unused_variables, non_snake_case)]
pub fn mjv_applyPerturbPose(m: *const mjModel, d: *mut mjData, pert: *const mjvPerturb, flg_paused: i32) {
    // SAFETY: m, d, pert are valid pointers (caller contract)
    unsafe {
        let sel = (*pert).select;
        let mut pos1 = [0.0f64; 3];
        let mut quat1 = [0.0f64; 4];
        let mut pos2 = [0.0f64; 3];
        let mut quat2 = [0.0f64; 4];
        let mut refpos = [0.0f64; 3];
        let mut refquat = [0.0f64; 4];

        // exit if nothing to do
        if sel <= 0 || sel as i64 >= (*m).nbody || ((*pert).active | (*pert).active2) == 0 {
            return;
        }

        // get rootid above selected body
        let rootid = *(*m).body_rootid.add(sel as usize) as usize;

        // transform refpos,refquat from I-frame to X-frame of body[sel]
        crate::engine::engine_util_spatial::mju_negPose(
            pos1.as_mut_ptr(), quat1.as_mut_ptr(),
            (*m).body_ipos.add(3 * sel as usize), (*m).body_iquat.add(4 * sel as usize));
        crate::engine::engine_util_spatial::mju_mulPose(
            refpos.as_mut_ptr(), refquat.as_mut_ptr(),
            (*pert).refpos.as_ptr(), (*pert).refquat.as_ptr(),
            pos1.as_ptr(), quat1.as_ptr());

        // mocap body
        if *(*m).body_mocapid.add(sel as usize) >= 0 {
            let mid = *(*m).body_mocapid.add(sel as usize) as usize;
            crate::engine::engine_util_blas::mju_copy3(
                (*d).mocap_pos.add(3 * mid), refpos.as_ptr());
            crate::engine::engine_util_blas::mju_copy4(
                (*d).mocap_quat.add(4 * mid), refquat.as_ptr());
        }
        // floating body, paused
        else if flg_paused != 0
            && *(*m).body_jntnum.add(sel as usize) == 1
            && *(*m).jnt_type.add(*(*m).body_jntadr.add(sel as usize) as usize) == mjtJoint_mjJNT_FREE as i32
        {
            let qadr = *(*m).jnt_qposadr.add(*(*m).body_jntadr.add(sel as usize) as usize) as usize;
            crate::engine::engine_util_blas::mju_copy3((*d).qpos.add(qadr), refpos.as_ptr());
            crate::engine::engine_util_blas::mju_copy4((*d).qpos.add(qadr + 3), refquat.as_ptr());
        }
        // child of floating body, paused
        else if flg_paused != 0
            && *(*m).body_jntnum.add(rootid) == 1
            && *(*m).jnt_type.add(*(*m).body_jntadr.add(rootid) as usize) == mjtJoint_mjJNT_FREE as i32
        {
            // get pointers to root
            let qadr = *(*m).jnt_qposadr.add(*(*m).body_jntadr.add(rootid) as usize) as usize;
            let Rpos = (*d).qpos.add(qadr);
            let Rquat = (*d).qpos.add(qadr + 3);

            // get pointers to child
            let Cpos = (*d).xpos.add(3 * sel as usize);
            let Cquat = (*d).xquat.add(4 * sel as usize);

            // set root <- ref*neg(child)*root
            crate::engine::engine_util_spatial::mju_negPose(
                pos1.as_mut_ptr(), quat1.as_mut_ptr(), Cpos, Cquat);  // neg(child)
            crate::engine::engine_util_spatial::mju_mulPose(
                pos2.as_mut_ptr(), quat2.as_mut_ptr(),
                pos1.as_ptr(), quat1.as_ptr(), Rpos, Rquat);          // neg(child)*root
            crate::engine::engine_util_spatial::mju_mulPose(
                Rpos, Rquat,
                refpos.as_ptr(), refquat.as_ptr(), pos2.as_ptr(), quat2.as_ptr());  // ref*neg(child)*root
        }
    }
}

/// C: mjv_applyPerturbForce (engine/engine_vis_interact.h:70)
/// Calls: cxx:_mj_objectVelocity, cxx:_mju_addTo3, cxx:_mju_addToScl3, cxx:_mju_copy3, cxx:_mju_cross, cxx:_mju_dot3, cxx:_mju_max, cxx:_mju_mulMatVec3, cxx:_mju_mulQuat, cxx:_mju_negQuat, cxx:_mju_normalize3, cxx:_mju_quat2Vel, cxx:_mju_scl3, cxx:_mju_sub3
#[allow(unused_variables, non_snake_case)]
pub fn mjv_applyPerturbForce(m: *const mjModel, d: *mut mjData, pert: *const mjvPerturb) {
    const MJ_MINVAL: f64 = 1e-15;
    const MJ_PERT_TRANSLATE: i32 = 1;
    const MJ_PERT_ROTATE: i32 = 2;

    // SAFETY: m, d, pert are valid pointers (caller contract)
    unsafe {
        let sel = (*pert).select;

        // exit if nothing to do
        if sel < 0 || sel as i64 >= (*m).nbody || ((*pert).active | (*pert).active2) == 0 {
            return;
        }

        // pointers to body xfrc_applied, force and torque
        let force = (*d).xfrc_applied.add(6 * sel as usize);
        let torque = (*d).xfrc_applied.add(6 * sel as usize + 3);

        // pointers to global selbody velocity, linear and rotational
        let mut bvel = [0.0f64; 6];
        crate::engine::engine_core_util::mj_objectVelocity(
            m, d as *const mjData, mjtObj_mjOBJ_BODY as i32, sel, bvel.as_mut_ptr(), 0);
        let body_linvel = bvel.as_ptr().add(3);
        let body_rotvel = bvel.as_ptr();

        // body rotational inertia
        let invweight = *(*m).body_invweight0.add(2 * sel as usize + 1);
        let inertia = if invweight != 0.0 {
            1.0 / crate::engine::engine_util_misc::mju_max(MJ_MINVAL, invweight)
        } else {
            1.0
        };

        // read stiffness and stiffnessrot from vis.map (f32 at offset 0 and 4)
        let map_bytes = &(*m).vis.map;
        let stiffness_map = f32::from_ne_bytes([map_bytes[0], map_bytes[1], map_bytes[2], map_bytes[3]]) as f64;
        let stiffnessrot_map = f32::from_ne_bytes([map_bytes[4], map_bytes[5], map_bytes[6], map_bytes[7]]) as f64;

        if (((*pert).active | (*pert).active2) & MJ_PERT_TRANSLATE) != 0 {
            let stiffness = stiffness_map;

            // compute selection point in world coordinates
            let mut selpos = [0.0f64; 3];
            crate::engine::engine_util_blas::mju_mulMatVec3(
                selpos.as_mut_ptr(), (*d).xmat.add(9 * sel as usize), (*pert).localpos.as_ptr());
            crate::engine::engine_util_blas::mju_addTo3(
                selpos.as_mut_ptr(), (*d).xpos.add(3 * sel as usize));

            // displacement of selection point from reference point
            let mut diff = [0.0f64; 3];
            crate::engine::engine_util_blas::mju_sub3(
                diff.as_mut_ptr(), selpos.as_ptr(), (*pert).refselpos.as_ptr());

            // spring perturbation force
            crate::engine::engine_util_blas::mju_copy3(force, diff.as_ptr());
            crate::engine::engine_util_blas::mju_scl3(force, force, -stiffness * (*pert).localmass);

            // moment arm w.r.t body com
            let mut moment_arm = [0.0f64; 3];
            crate::engine::engine_util_blas::mju_sub3(
                moment_arm.as_mut_ptr(), selpos.as_ptr(), (*d).xipos.add(3 * sel as usize));

            // translational velocity of selection point
            let mut svel = [0.0f64; 3];
            crate::engine::engine_util_spatial::mju_cross(
                svel.as_mut_ptr(), body_rotvel, moment_arm.as_ptr());
            crate::engine::engine_util_blas::mju_addTo3(svel.as_mut_ptr(), body_linvel);

            // add critical damping force of selection point
            crate::engine::engine_util_blas::mju_addToScl3(
                force, svel.as_ptr(), -stiffness.sqrt() * (*pert).localmass);

            // torque on body com due to force
            crate::engine::engine_util_spatial::mju_cross(torque, moment_arm.as_ptr(), force);

            // add critically damped torsional torque along displacement axis
            let stiffnessrot = stiffnessrot_map;
            crate::engine::engine_util_blas::mju_normalize3(diff.as_mut_ptr());
            crate::engine::engine_util_blas::mju_addToScl3(
                torque, diff.as_ptr(),
                -stiffnessrot.sqrt() * inertia * crate::engine::engine_util_blas::mju_dot3(
                    diff.as_ptr(), body_rotvel));
        }

        if (((*pert).active | (*pert).active2) & MJ_PERT_ROTATE) != 0 {
            // spring perturbation torque, with critical damping
            let stiffnessrot = stiffnessrot_map;
            let mut xiquat = [0.0f64; 4];
            let mut difquat = [0.0f64; 4];
            crate::engine::engine_util_spatial::mju_mulQuat(
                xiquat.as_mut_ptr(), (*d).xquat.add(4 * sel as usize), (*m).body_iquat.add(4 * sel as usize));
            crate::engine::engine_util_spatial::mju_negQuat(xiquat.as_mut_ptr(), xiquat.as_ptr());
            crate::engine::engine_util_spatial::mju_mulQuat(
                difquat.as_mut_ptr(), (*pert).refquat.as_ptr(), xiquat.as_ptr());
            crate::engine::engine_util_spatial::mju_quat2Vel(
                torque, difquat.as_ptr(), 1.0 / (stiffnessrot * inertia));
            crate::engine::engine_util_blas::mju_addToScl3(
                torque, body_rotvel, -stiffnessrot.sqrt() * inertia);
        }
    }
}

/// C: mjv_averageCamera (engine/engine_vis_interact.h:73)
/// Calls: cxx:_mju_add3, cxx:_mju_addToScl3, cxx:_mju_dot3, cxx:_mju_f2n, cxx:_mju_message, cxx:_mju_n2f, cxx:_mju_normalize3, cxx:_mju_scl3
#[allow(unused_variables, non_snake_case)]
pub fn mjv_averageCamera(cam1: *const mjvGLCamera, cam2: *const mjvGLCamera) -> mjvGLCamera {
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
        crate::engine::engine_util_blas::mju_addToScl3(up.as_mut_ptr(), forward.as_ptr(), -projection);
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

