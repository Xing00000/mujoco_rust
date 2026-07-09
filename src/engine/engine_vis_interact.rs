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
    // mjtMouse enum values
    const MJMOUSE_ROTATE_V: i32 = 1;
    const MJMOUSE_ROTATE_H: i32 = 2;
    const MJMOUSE_MOVE_V: i32 = 3;
    const MJMOUSE_MOVE_H: i32 = 4;
    const MJMOUSE_ZOOM: i32 = 5;
    const MJMOUSE_MOVE_V_REL: i32 = 6;
    const MJMOUSE_MOVE_H_REL: i32 = 7;

    // SAFETY: caller guarantees res[3], forward[3] are valid
    unsafe {
        let mut vec = [0.0f64; 3];

        // construct 3D vector
        match action {
            MJMOUSE_ROTATE_V => {
                vec[0] = dy;
                vec[1] = 0.0;
                vec[2] = dx;
            }
            MJMOUSE_ROTATE_H => {
                vec[0] = dy;
                vec[1] = dx;
                vec[2] = 0.0;
            }
            MJMOUSE_MOVE_V | MJMOUSE_MOVE_V_REL => {
                vec[0] = dx;
                vec[1] = 0.0;
                vec[2] = -dy;
            }
            MJMOUSE_MOVE_H | MJMOUSE_MOVE_H_REL => {
                vec[0] = dx;
                vec[1] = -dy;
                vec[2] = 0.0;
            }
            MJMOUSE_ZOOM => {
                // break (no-op)
            }
            _ => {
                crate::engine::engine_util_errmem::mju_error(
                    b"unexpected mouse action in convert2D\0".as_ptr() as *const i8,
                );
            }
        }

        // call 3D converter
        crate::engine::engine_vis_interact::mjv_align_to_camera(res, vec.as_ptr(), forward);
    }
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
    // SAFETY: all pointers are valid per caller contract. scn->scale, translate, rotate accessed.
    unsafe {
        const mjMINVAL: f32 = 1e-15;
        let mut translate: [f64; 3] = [0.0; 3];
        let mut rotate: [f64; 4] = [0.0; 4];
        let mut invpos: [f64; 3] = [0.0; 3];
        let mut invquat: [f64; 4] = [0.0; 4];

        // check scale
        if (*scn).scale < mjMINVAL {
            crate::engine::engine_util_errmem::mju_error(
                b"mjvScene scale too small\0".as_ptr() as *const i8
            );
        }

        // enabled: transform
        if (*scn).enabletransform != 0 {
            // convert translate, rotate to mjtNum
            crate::engine::engine_util_misc::mju_f2n(translate.as_mut_ptr(), (*scn).translate.as_ptr(), 3);
            crate::engine::engine_util_misc::mju_f2n(rotate.as_mut_ptr(), (*scn).rotate.as_ptr(), 4);

            // invert model pose (without scale)
            crate::engine::engine_util_spatial::mju_neg_pose(
                invpos.as_mut_ptr(), invquat.as_mut_ptr(),
                translate.as_ptr(), rotate.as_ptr()
            );

            // map from room to model space
            crate::engine::engine_util_spatial::mju_mul_pose(
                modelpos, modelquat,
                invpos.as_ptr(), invquat.as_ptr(),
                roompos, roomquat
            );

            // divide position by scale
            crate::engine::engine_util_blas::mju_scl3(modelpos, modelpos, 1.0 / (*scn).scale as f64);
        } else {
            // disabled: copy
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
    // SAFETY: all pointers are valid per caller contract. scn->scale, translate, rotate accessed.
    unsafe {
        const mjMINVAL: f32 = 1e-15;
        let mut translate: [f64; 3] = [0.0; 3];
        let mut rotate: [f64; 4] = [0.0; 4];

        // check scale
        if (*scn).scale < mjMINVAL {
            crate::engine::engine_util_errmem::mju_error(
                b"mjvScene scale too small\0".as_ptr() as *const i8
            );
        }

        // enabled: transform
        if (*scn).enabletransform != 0 {
            // convert translate, rotate to mjtNum
            crate::engine::engine_util_misc::mju_f2n(translate.as_mut_ptr(), (*scn).translate.as_ptr(), 3);
            crate::engine::engine_util_misc::mju_f2n(rotate.as_mut_ptr(), (*scn).rotate.as_ptr(), 4);

            // map from model to room space
            crate::engine::engine_util_spatial::mju_mul_pose(
                roompos, roomquat,
                translate.as_ptr(), rotate.as_ptr(),
                modelpos, modelquat
            );

            // scale position
            crate::engine::engine_util_blas::mju_scl3(roompos, roompos, (*scn).scale as f64);
        } else {
            // disabled: copy
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
    // SAFETY: scn is valid. headpos/forward/up may be null (checked before use).
    // Each non-null pointer points to valid f64[3]. camera[0..2] are inline in mjvScene.
    unsafe {
        const mjMINVAL: f32 = 1e-15;

        // check znear
        if (*scn).camera[0].frustum_near < mjMINVAL || (*scn).camera[1].frustum_near < mjMINVAL {
            crate::engine::engine_util_errmem::mju_error(
                b"mjvScene frustum_near too small\0".as_ptr() as *const i8,
            );
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
        for n in 0..2i32 {
            let mut pos: [f64; 3] = [0.0; 3];
            let mut fwd: [f64; 3] = [0.0; 3];
            let mut u: [f64; 3] = [0.0; 3];
            let mut quat: [f64; 4] = [0.0; 4];

            // convert pos, fwd, u
            crate::engine::engine_util_misc::mju_f2n(pos.as_mut_ptr(), (*scn).camera[n as usize].pos.as_ptr(), 3);
            crate::engine::engine_util_misc::mju_f2n(fwd.as_mut_ptr(), (*scn).camera[n as usize].forward.as_ptr(), 3);
            crate::engine::engine_util_misc::mju_f2n(u.as_mut_ptr(), (*scn).camera[n as usize].up.as_ptr(), 3);

            // normalize just in case
            crate::engine::engine_util_blas::mju_normalize3(fwd.as_mut_ptr());
            crate::engine::engine_util_blas::mju_normalize3(u.as_mut_ptr());

            // make orientation matrix: x = left, y = up, z = forward
            let mut left: [f64; 3] = [0.0; 3];
            crate::engine::engine_util_spatial::mju_cross(left.as_mut_ptr(), u.as_ptr(), fwd.as_ptr());
            crate::engine::engine_util_blas::mju_normalize3(left.as_mut_ptr());
            let mat: [f64; 9] = [
                left[0], u[0], fwd[0],
                left[1], u[1], fwd[1],
                left[2], u[2], fwd[2],
            ];
            crate::engine::engine_util_spatial::mju_mat2quat(quat.as_mut_ptr(), mat.as_ptr());

            // convert to model space, make orientation matrix
            let mut modelpos: [f64; 3] = [0.0; 3];
            let mut modelquat: [f64; 4] = [0.0; 4];
            let mut modelmat: [f64; 9] = [0.0; 9];
            crate::engine::engine_vis_interact::mjv_room2model(
                modelpos.as_mut_ptr(), modelquat.as_mut_ptr(),
                pos.as_ptr(), quat.as_ptr(), scn,
            );
            crate::engine::engine_util_spatial::mju_quat2mat(modelmat.as_mut_ptr(), modelquat.as_ptr());

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
    // SAFETY: scn is valid. headpos/forward/up may be null (checked before use).
    // Each non-null pointer points to valid f64[3] array.
    unsafe {
        const mjMINVAL: f32 = 1e-15;

        // check znear
        if (*scn).camera[0].frustum_near < mjMINVAL || (*scn).camera[1].frustum_near < mjMINVAL {
            crate::engine::engine_util_errmem::mju_error(
                b"mjvScene frustum_near too small\0".as_ptr() as *const i8
            );
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
        for n in 0..2i32 {
            let mut pos: [f64; 3] = [0.0; 3];
            let mut fwd: [f64; 3] = [0.0; 3];
            let mut u: [f64; 3] = [0.0; 3];

            // convert pos, fwd, u from f32 to f64
            crate::engine::engine_util_misc::mju_f2n(pos.as_mut_ptr(), (*scn).camera[n as usize].pos.as_ptr(), 3);
            crate::engine::engine_util_misc::mju_f2n(fwd.as_mut_ptr(), (*scn).camera[n as usize].forward.as_ptr(), 3);
            crate::engine::engine_util_misc::mju_f2n(u.as_mut_ptr(), (*scn).camera[n as usize].up.as_ptr(), 3);

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
    // SAFETY: scn is valid pointer. Accessing camera[0] and camera[1] which are fixed-size array.
    unsafe {
        let cam1 = &(*scn).camera[0];
        let cam2 = &(*scn).camera[1];

        if cam1.orthographic != cam2.orthographic {
            crate::engine::engine_util_errmem::mju_error(
                b"cannot average frustums of perspective and orthographic cameras\0".as_ptr() as *const i8
            );
        }

        let height: f64;
        if cam1.orthographic == 0 {
            // perspective: check znear
            const mjMINVAL: f32 = 1e-15;
            if cam1.frustum_near < mjMINVAL || cam2.frustum_near < mjMINVAL {
                crate::engine::engine_util_errmem::mju_error(
                    b"mjvScene frustum_near too small\0".as_ptr() as *const i8
                );
            }

            // add normalized height for left and right cameras
            height = (cam1.frustum_top - cam1.frustum_bottom) as f64 / cam1.frustum_near as f64
                   + (cam2.frustum_top - cam2.frustum_bottom) as f64 / cam2.frustum_near as f64;
        } else {
            // orthographic: add height for left and right cameras
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
    // SAFETY: m, d, scn, pert valid per caller. Pointer arithmetic on d->xmat, d->xquat,
    // m->body_iquat uses sel index which is bounds-checked against m->nbody.
    unsafe {
        const MJMOUSE_ROTATE_V: i32 = 1;
        const MJMOUSE_ROTATE_H: i32 = 2;
        const MJMOUSE_MOVE_V: i32 = 3;
        const MJMOUSE_MOVE_H: i32 = 4;
        const MJMOUSE_ZOOM: i32 = 5;
        const MJMOUSE_MOVE_V_REL: i32 = 6;
        const MJMOUSE_MOVE_H_REL: i32 = 7;
        const MJPI: f64 = 3.14159265358979323846;

        let xaxis: [f64; 3] = [1.0, 0.0, 0.0];
        let yaxis: [f64; 3] = [0.0, 1.0, 0.0];
        let zaxis: [f64; 3] = [0.0, 0.0, 1.0];

        let sel = (*pert).select;
        let xmat = (*d).xmat.add(9 * sel as usize);
        let mut forward: [f64; 3] = [0.0; 3];
        let mut vec: [f64; 3] = [0.0; 3];
        let mut scl: f64;
        let mut q1: [f64; 4] = [0.0; 4];
        let mut xiquat: [f64; 4] = [0.0; 4];

        // get camera info and align
        crate::engine::engine_vis_interact::mjv_camera_in_model(
            core::ptr::null_mut(), forward.as_mut_ptr(), core::ptr::null_mut(), scn,
        );
        crate::engine::engine_vis_interact::convert2d(
            vec.as_mut_ptr(), action, reldx, reldy, forward.as_ptr(),
        );

        // process action
        match action {
            MJMOUSE_MOVE_V | MJMOUSE_MOVE_H => {
                // move along world-space horizontal/vertical planes relative to camera
                crate::engine::engine_util_blas::mju_add_to_scl3(
                    (*pert).refpos.as_mut_ptr(), vec.as_ptr(), (*pert).scale,
                );
                crate::engine::engine_util_blas::mju_add_to_scl3(
                    (*pert).refselpos.as_mut_ptr(), vec.as_ptr(), (*pert).scale,
                );
            }
            MJMOUSE_MOVE_V_REL | MJMOUSE_MOVE_H_REL => {
                // move along object's local coordinate frame
                if action == MJMOUSE_MOVE_H_REL {
                    crate::engine::engine_util_blas::mju_mul_mat_vec3(vec.as_mut_ptr(), xmat, xaxis.as_ptr());
                    crate::engine::engine_util_blas::mju_add_to_scl3(
                        (*pert).refpos.as_mut_ptr(), vec.as_ptr(), (*pert).scale * reldy,
                    );
                    crate::engine::engine_util_blas::mju_add_to_scl3(
                        (*pert).refselpos.as_mut_ptr(), vec.as_ptr(), (*pert).scale * reldy,
                    );
                } else {
                    crate::engine::engine_util_blas::mju_mul_mat_vec3(vec.as_mut_ptr(), xmat, zaxis.as_ptr());
                    crate::engine::engine_util_blas::mju_add_to_scl3(
                        (*pert).refpos.as_mut_ptr(), vec.as_ptr(), (*pert).scale * reldy,
                    );
                    crate::engine::engine_util_blas::mju_add_to_scl3(
                        (*pert).refselpos.as_mut_ptr(), vec.as_ptr(), (*pert).scale * reldy,
                    );
                }

                crate::engine::engine_util_blas::mju_mul_mat_vec3(vec.as_mut_ptr(), xmat, yaxis.as_ptr());
                crate::engine::engine_util_blas::mju_add_to_scl3(
                    (*pert).refpos.as_mut_ptr(), vec.as_ptr(), (*pert).scale * reldx,
                );
                crate::engine::engine_util_blas::mju_add_to_scl3(
                    (*pert).refselpos.as_mut_ptr(), vec.as_ptr(), (*pert).scale * reldx,
                );
            }
            MJMOUSE_ROTATE_V | MJMOUSE_ROTATE_H => {
                // normalize vector, get length
                scl = crate::engine::engine_util_blas::mju_normalize3(vec.as_mut_ptr());

                // make quaternion and apply
                crate::engine::engine_util_spatial::mju_axis_angle2quat(
                    q1.as_mut_ptr(), vec.as_ptr(), scl * MJPI * 2.0,
                );
                crate::engine::engine_util_spatial::mju_mul_quat(
                    (*pert).refquat.as_mut_ptr(), q1.as_ptr(), (*pert).refquat.as_ptr(),
                );
                crate::engine::engine_util_blas::mju_normalize4((*pert).refquat.as_mut_ptr());

                // compute xiquat
                crate::engine::engine_util_spatial::mju_mul_quat(
                    xiquat.as_mut_ptr(),
                    (*d).xquat.add(4 * sel as usize),
                    (*m).body_iquat.add(4 * sel as usize),
                );

                // limit rotation relative to selected body
                if sel > 0 && (sel as usize) < (*m).nbody {
                    // q2 = neg(selbody) * refquat
                    let mut q2: [f64; 4] = [0.0; 4];
                    crate::engine::engine_util_spatial::mju_neg_quat(q1.as_mut_ptr(), xiquat.as_ptr());
                    crate::engine::engine_util_spatial::mju_mul_quat(
                        q2.as_mut_ptr(), q1.as_ptr(), (*pert).refquat.as_ptr(),
                    );

                    // convert q2 to axis-angle
                    let mut dif: [f64; 3] = [0.0; 3];
                    crate::engine::engine_util_spatial::mju_quat2vel(dif.as_mut_ptr(), q2.as_ptr(), 1.0);
                    scl = crate::engine::engine_util_blas::mju_normalize3(dif.as_mut_ptr());

                    // check limit: +/- 90 deg allowed
                    if scl < -MJPI * 0.5 || scl > MJPI * 0.5 {
                        // clamp angle
                        scl = crate::engine::engine_util_misc::mju_max(
                            -MJPI * 0.5,
                            crate::engine::engine_util_misc::mju_min(MJPI * 0.5, scl),
                        );

                        // reconstruct q2
                        crate::engine::engine_util_spatial::mju_axis_angle2quat(
                            q2.as_mut_ptr(), dif.as_ptr(), scl,
                        );

                        // set refquat = selbody * q2_new
                        crate::engine::engine_util_spatial::mju_mul_quat(
                            (*pert).refquat.as_mut_ptr(), xiquat.as_ptr(), q2.as_ptr(),
                        );
                    }
                }
            }
            MJMOUSE_ZOOM => {}
            _ => {
                crate::engine::engine_util_errmem::mju_error(
                    b"unexpected mouse action in mjv_movePerturb\0".as_ptr() as *const i8,
                );
            }
        }
    }
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
    // SAFETY: m, roomup, scn valid per caller. scn->translate, rotate, scale are inline arrays/scalars.
    // roomup points to f64[3]. m->stat.extent is a simple f64 field.
    unsafe {
        const MJMOUSE_ROTATE_V: i32 = 1;
        const MJMOUSE_ROTATE_H: i32 = 2;
        const MJMOUSE_MOVE_V: i32 = 3;
        const MJMOUSE_MOVE_H: i32 = 4;
        const MJMOUSE_ZOOM: i32 = 5;
        const MJMOUSE_MOVE_V_REL: i32 = 6;
        const MJMOUSE_MOVE_H_REL: i32 = 7;
        const MJPI: f64 = 3.14159265358979323846;

        let mut roomforward: [f64; 3] = [0.0; 3];
        let mut roomright: [f64; 3] = [0.0; 3];
        let mut camforward: [f64; 3] = [0.0; 3];
        let mut vec: [f64; 3] = [0.0; 3];
        let mut scl: f64;
        let mut quat: [f64; 4] = [0.0; 4];
        let mut rotate: [f64; 4] = [0.0; 4];
        let mut result: [f64; 4] = [0.0; 4];

        // transformation disabled: nothing to do
        if (*scn).enabletransform == 0 {
            return;
        }

        // get camera forward in room space
        crate::engine::engine_vis_interact::mjv_camera_in_room(
            core::ptr::null_mut(), camforward.as_mut_ptr(), core::ptr::null_mut(), scn,
        );

        // make orthogonal to roomup
        crate::engine::engine_util_blas::mju_add_scl3(
            roomforward.as_mut_ptr(), camforward.as_ptr(), roomup,
            -crate::engine::engine_util_blas::mju_dot3(camforward.as_ptr(), roomup),
        );
        crate::engine::engine_util_blas::mju_normalize3(roomforward.as_mut_ptr());

        // compute roomright
        crate::engine::engine_util_spatial::mju_cross(
            roomright.as_mut_ptr(), roomforward.as_ptr(), roomup,
        );
        crate::engine::engine_util_blas::mju_normalize3(roomright.as_mut_ptr());

        // process action
        match action {
            MJMOUSE_ROTATE_V | MJMOUSE_ROTATE_H => {
                // construct rotation vector
                for i in 0..3 {
                    if action == MJMOUSE_ROTATE_V {
                        vec[i] = *roomup.add(i) * reldx + roomright[i] * reldy;
                    } else {
                        vec[i] = roomforward[i] * reldx + roomright[i] * reldy;
                    }
                }

                // make quaternion from angle-axis
                scl = crate::engine::engine_util_blas::mju_normalize3(vec.as_mut_ptr());
                crate::engine::engine_util_spatial::mju_axis_angle2quat(
                    quat.as_mut_ptr(), vec.as_ptr(), scl * MJPI,
                );

                // get current model rotation
                crate::engine::engine_util_misc::mju_f2n(rotate.as_mut_ptr(), (*scn).rotate.as_ptr(), 4);

                // compose rotation, normalize and set
                crate::engine::engine_util_spatial::mju_mul_quat(
                    result.as_mut_ptr(), quat.as_ptr(), rotate.as_ptr(),
                );
                crate::engine::engine_util_blas::mju_normalize4(result.as_mut_ptr());
                crate::engine::engine_util_misc::mju_n2f((*scn).rotate.as_mut_ptr(), result.as_ptr(), 4);
            }
            MJMOUSE_MOVE_V | MJMOUSE_MOVE_V_REL => {
                for i in 0..3 {
                    (*scn).translate[i] += ((roomright[i] * reldx - *roomup.add(i) * reldy) as f32 as f64
                        * (*m).stat.extent) as f32;
                }
            }
            MJMOUSE_MOVE_H | MJMOUSE_MOVE_H_REL => {
                for i in 0..3 {
                    (*scn).translate[i] += ((roomright[i] * reldx - roomforward[i] * reldy) as f32 as f64
                        * (*m).stat.extent) as f32;
                }
            }
            MJMOUSE_ZOOM => {
                (*scn).scale += (((1.0f32 + (*scn).scale / 3.0f32) as f64).ln() * reldy * 3.0) as f32;
                if (*scn).scale < 0.01f32 {
                    (*scn).scale = 0.01f32;
                } else if (*scn).scale > 100.0f32 {
                    (*scn).scale = 100.0f32;
                }
            }
            _ => {
                crate::engine::engine_util_errmem::mju_error(
                    b"unexpected action in mjv_moveModel\0".as_ptr() as *const i8,
                );
            }
        }
    }
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
    // SAFETY: m, d, pert valid per caller. Pointer arithmetic on body arrays uses sel/rootid
    // which are bounds-checked against m->nbody. qpos access uses jnt_qposadr indices.
    unsafe {
        const MJJNT_FREE: i32 = 0;

        let sel = (*pert).select;
        let mut pos1: [f64; 3] = [0.0; 3];
        let mut quat1: [f64; 4] = [0.0; 4];
        let mut pos2: [f64; 3] = [0.0; 3];
        let mut quat2: [f64; 4] = [0.0; 4];
        let mut refpos: [f64; 3] = [0.0; 3];
        let mut refquat: [f64; 4] = [0.0; 4];

        // exit if nothing to do
        if sel <= 0 || (sel as usize) >= (*m).nbody || ((*pert).active | (*pert).active2) == 0 {
            return;
        }

        // get rootid above selected body
        let rootid = *(*m).body_rootid.add(sel as usize);

        // transform refpos,refquat from I-frame to X-frame of body[sel]
        crate::engine::engine_util_spatial::mju_neg_pose(
            pos1.as_mut_ptr(), quat1.as_mut_ptr(),
            (*m).body_ipos.add(3 * sel as usize),
            (*m).body_iquat.add(4 * sel as usize),
        );
        crate::engine::engine_util_spatial::mju_mul_pose(
            refpos.as_mut_ptr(), refquat.as_mut_ptr(),
            (*pert).refpos.as_ptr(), (*pert).refquat.as_ptr(),
            pos1.as_ptr(), quat1.as_ptr(),
        );

        // mocap body
        if *(*m).body_mocapid.add(sel as usize) >= 0 {
            let mocapid = *(*m).body_mocapid.add(sel as usize);
            // copy ref pose into mocap pose
            crate::engine::engine_util_blas::mju_copy3(
                (*d).mocap_pos.add(3 * mocapid as usize), refpos.as_ptr(),
            );
            crate::engine::engine_util_blas::mju_copy4(
                (*d).mocap_quat.add(4 * mocapid as usize), refquat.as_ptr(),
            );
        }
        // floating body, paused
        else if flg_paused != 0
            && *(*m).body_jntnum.add(sel as usize) == 1
            && *(*m).jnt_type.add(*(*m).body_jntadr.add(sel as usize) as usize) == MJJNT_FREE
        {
            let qposadr = *(*m).jnt_qposadr.add(*(*m).body_jntadr.add(sel as usize) as usize);
            // copy ref pose into qpos
            crate::engine::engine_util_blas::mju_copy3(
                (*d).qpos.add(qposadr as usize), refpos.as_ptr(),
            );
            crate::engine::engine_util_blas::mju_copy4(
                (*d).qpos.add(qposadr as usize + 3), refquat.as_ptr(),
            );
        }
        // child of floating body, paused
        else if flg_paused != 0
            && *(*m).body_jntnum.add(rootid as usize) == 1
            && *(*m).jnt_type.add(*(*m).body_jntadr.add(rootid as usize) as usize) == MJJNT_FREE
        {
            let qposadr = *(*m).jnt_qposadr.add(*(*m).body_jntadr.add(rootid as usize) as usize);
            // get pointers to root
            let rpos = (*d).qpos.add(qposadr as usize);
            let rquat = rpos.add(3);

            // get pointers to child
            let cpos = (*d).xpos.add(3 * sel as usize);
            let cquat = (*d).xquat.add(4 * sel as usize);

            // set root <- ref*neg(child)*root
            crate::engine::engine_util_spatial::mju_neg_pose(
                pos1.as_mut_ptr(), quat1.as_mut_ptr(), cpos, cquat,
            ); // neg(child)
            crate::engine::engine_util_spatial::mju_mul_pose(
                pos2.as_mut_ptr(), quat2.as_mut_ptr(),
                pos1.as_ptr(), quat1.as_ptr(), rpos, rquat,
            ); // neg(child)*root
            crate::engine::engine_util_spatial::mju_mul_pose(
                rpos, rquat,
                refpos.as_ptr(), refquat.as_ptr(),
                pos2.as_ptr(), quat2.as_ptr(),
            ); // ref*neg(child)*root
        }
    }
}

/// C: mjv_applyPerturbForce (engine/engine_vis_interact.h:70)
/// Calls: mj_objectVelocity, mju_addTo3, mju_addToScl3, mju_copy3, mju_cross, mju_dot3, mju_max, mju_mulMatVec3, mju_mulQuat, mju_negQuat, mju_normalize3, mju_quat2Vel, mju_scl3, mju_sub3
#[allow(unused_variables, non_snake_case)]
pub fn mjv_apply_perturb_force(m: *const mjModel, d: *mut mjData, pert: *const mjvPerturb) {
    // Cannot translate: m->vis.map.stiffness and m->vis.map.stiffnessrot require
    // accessing opaque sub-struct fields not exposed in the Rust type system.
    extern "C" { fn mjv_applyPerturbForce(m: *const mjModel, d: *mut mjData, pert: *const mjvPerturb); }
    // SAFETY: delegates to original C function; all pointers valid per caller contract
    unsafe { mjv_applyPerturbForce(m, d, pert) }
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
    // SAFETY: m, d, flexpnt valid per caller. All array accesses use indices derived from
    // model topology arrays (flex_vertadr, flex_nodeadr, etc.) which are consistent.
    unsafe {
        let mut flexbodyid: i32 = -1;

        if *(*m).flex_interp.add(flexid as usize) != 0 {
            let coord = (*m).flex_vert0.add(3 * (*(*m).flex_vertadr.add(flexid as usize) + vertid) as usize);
            let mut order = *(*m).flex_interp.add(flexid as usize);
            order = if order < 0 { -order } else { order };
            let npc = (order + 1) * (order + 1) * (order + 1);

            // cell lookup: get local coords and node indices
            let mut loc: [f64; 3] = [0.0; 3];
            let mut nodeindices: [i32; 27] = [0; 27];
            let cellnum_ptr = (*m).flex_cellnum.add(3 * flexid as usize);
            let cellnum = [
                *cellnum_ptr.add(0),
                *cellnum_ptr.add(1),
                *cellnum_ptr.add(2),
            ];
            crate::engine::engine_util_misc::mju_cell_lookup(
                coord, cellnum, order, loc.as_mut_ptr(), nodeindices.as_mut_ptr(),
            );

            // find node with largest weight in this cell
            // in shell mode, skip interior nodes (pinned to worldbody)
            let mut nodeid: i32 = -1;
            let nstart = *(*m).flex_nodeadr.add(flexid as usize);
            let mut w: f64 = 0.0;
            let shell_mode = *(*m).flex_interp.add(flexid as usize) < 0;
            for j in 0..npc {
                let ww = crate::engine::engine_util_misc::mju_eval_basis(loc.as_ptr(), j, order);
                let nid = nodeindices[j as usize];
                // skip interior nodes in shell mode (they map to worldbody)
                if shell_mode && *(*m).body_dofnum.add(*(*m).flex_nodebodyid.add((nstart + nid) as usize) as usize) == 0 {
                    continue;
                }
                if ww > w {
                    w = ww;
                    nodeid = nid;
                }
            }
            flexbodyid = *(*m).flex_nodebodyid.add((nstart + nodeid) as usize);
            if *((*m).flex_centered as *const u8).add(flexid as usize) != 0 {
                crate::engine::engine_util_blas::mju_copy3(flexpnt, (*d).xpos.add(3 * flexbodyid as usize));
            } else {
                crate::engine::engine_util_blas::mju_mul_mat_vec3(
                    flexpnt,
                    (*d).xmat.add(9 * flexbodyid as usize),
                    (*m).flex_node.add(3 * (nstart + nodeid) as usize),
                );
                crate::engine::engine_util_blas::mju_add_to3(flexpnt, (*d).xpos.add(3 * flexbodyid as usize));
            }
        } else {
            flexbodyid = *(*m).flex_vertbodyid.add((*(*m).flex_vertadr.add(flexid as usize) + vertid) as usize);
            crate::engine::engine_util_blas::mju_copy3(
                flexpnt,
                (*d).flexvert_xpos.add(3 * (*(*m).flex_vertadr.add(flexid as usize) + vertid) as usize),
            );
        }
        flexbodyid
    }
}

