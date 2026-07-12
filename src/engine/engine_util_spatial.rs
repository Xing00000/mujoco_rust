//! Port of: engine/engine_util_spatial.h
//! IR hash: c6d98e4f4b63b7f2
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mju_rotVecQuat (engine/engine_util_spatial.h:27)
/// Calls: mji_copy3, mju_zero3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_rot_vec_quat(res: *mut f64, vec: *const f64, quat: *const f64) {
    // NOTE: signature changed from previous IR version
    // Previous params: (res : * mut f64, vec : * const f64, quat : * const f64)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mju_negQuat (engine/engine_util_spatial.h:30)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_neg_quat(res: *mut f64, quat: *const f64) {
    // NOTE: signature changed from previous IR version
    // Previous params: (res : * mut f64, quat : * const f64)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mju_mulQuat (engine/engine_util_spatial.h:33)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_mul_quat(res: *mut f64, quat1: *const f64, quat2: *const f64) {
    // NOTE: signature changed from previous IR version
    // Previous params: (res : * mut f64, quat1 : * const f64, quat2 : * const f64)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mju_mulQuatAxis (engine/engine_util_spatial.h:36)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_mul_quat_axis(res: *mut f64, quat: *const f64, axis: *const f64) {
    // NOTE: signature changed from previous IR version
    // Previous params: (res : * mut f64, quat : * const f64, axis : * const f64)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mju_axisAngle2Quat (engine/engine_util_spatial.h:39)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_axis_angle2quat(res: *mut f64, axis: *const f64, angle: f64) {
    // NOTE: signature changed from previous IR version
    // Previous params: (res : * mut f64, axis : * const f64, angle : f64)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mju_quat2Vel (engine/engine_util_spatial.h:42)
/// Calls: mji_scl3, mju_normalize3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_quat2vel(res: *mut f64, quat: *const f64, dt: f64) {
    // NOTE: signature changed from previous IR version
    // Previous params: (res : * mut f64, quat : * const f64, dt : f64)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mju_subQuat (engine/engine_util_spatial.h:45)
/// Calls: mji_mulQuat, mji_negQuat, mji_quat2Vel
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_sub_quat(res: *mut f64, qa: *const f64, qb: *const f64) {
    // NOTE: signature changed from previous IR version
    // Previous params: (res : * mut f64, qa : * const f64, qb : * const f64)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mju_quat2Mat (engine/engine_util_spatial.h:48)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_quat2mat(res: *mut f64, quat: *const f64) {
    unsafe {
        // SAFETY: caller guarantees quat points to 4 f64 and res points to 9 f64
        if *quat.add(0) == 1.0 && *quat.add(1) == 0.0 && *quat.add(2) == 0.0 && *quat.add(3) == 0.0 {
            *res.add(0) = 1.0; *res.add(1) = 0.0; *res.add(2) = 0.0;
            *res.add(3) = 0.0; *res.add(4) = 1.0; *res.add(5) = 0.0;
            *res.add(6) = 0.0; *res.add(7) = 0.0; *res.add(8) = 1.0;
        } else {
            let q00 = *quat.add(0) * *quat.add(0);
            let q01 = *quat.add(0) * *quat.add(1);
            let q02 = *quat.add(0) * *quat.add(2);
            let q03 = *quat.add(0) * *quat.add(3);
            let q11 = *quat.add(1) * *quat.add(1);
            let q12 = *quat.add(1) * *quat.add(2);
            let q13 = *quat.add(1) * *quat.add(3);
            let q22 = *quat.add(2) * *quat.add(2);
            let q23 = *quat.add(2) * *quat.add(3);
            let q33 = *quat.add(3) * *quat.add(3);
            *res.add(0) = q00 + q11 - q22 - q33;
            *res.add(4) = q00 - q11 + q22 - q33;
            *res.add(8) = q00 - q11 - q22 + q33;
            *res.add(1) = 2.0 * (q12 - q03);
            *res.add(2) = 2.0 * (q13 + q02);
            *res.add(3) = 2.0 * (q12 + q03);
            *res.add(5) = 2.0 * (q23 - q01);
            *res.add(6) = 2.0 * (q13 - q02);
            *res.add(7) = 2.0 * (q23 + q01);
        }
    }
}

/// C: mju_mat2Quat (engine/engine_util_spatial.h:51)
/// Calls: mju_normalize4
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_mat2quat(quat: *mut f64, mat: *const f64) {
    // NOTE: signature changed from previous IR version
    // Previous params: (quat : * mut f64, mat : * const f64)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mju_derivQuat (engine/engine_util_spatial.h:54)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_deriv_quat(res: *mut f64, quat: *const f64, vel: *const f64) {
    // NOTE: signature changed from previous IR version
    // Previous params: (res : * mut f64, quat : * const f64, vel : * const f64)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mju_quatIntegrate (engine/engine_util_spatial.h:57)
/// Calls: mji_axisAngle2Quat, mji_copy3, mju_mulQuat, mju_normalize3, mju_normalize4
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_quat_integrate(quat: *mut f64, vel: *const f64, scale: f64) {
    // NOTE: signature changed from previous IR version
    // Previous params: (quat : * mut f64, vel : * const f64, scale : f64)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mju_quatZ2Vec (engine/engine_util_spatial.h:60)
/// Calls: mji_axisAngle2Quat, mji_cross, mju_dot3, mju_normalize3, mju_zero3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_quat_z2vec(quat: *mut f64, vec: *const f64) {
    // NOTE: signature changed from previous IR version
    // Previous params: (quat : * mut f64, vec : * const f64)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mju_mat2Rot (engine/engine_util_spatial.h:64)
/// Calls: mji_add3, mji_addTo3, mji_axisAngle2Quat, mji_cross, mju_dot3, mju_mulQuat, mju_normalize3, mju_normalize4, mju_quat2Mat, mju_scl3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_mat2rot(quat: *mut f64, mat: *const f64) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (quat : * mut f64, mat : * const f64)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: mju_mulPose (engine/engine_util_spatial.h:70)
/// Calls: mji_addTo3, mji_mulQuat, mji_rotVecQuat, mju_normalize4
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_mul_pose(posres: *mut f64, quatres: *mut f64, pos1: *const f64, quat1: *const f64, pos2: *const f64, quat2: *const f64) {
    // NOTE: signature changed from previous IR version
    // Previous params: (posres : * mut f64, quatres : * mut f64, pos1 : * const f64, quat1 : * const f64, pos2 : * const f64, quat2 : * const f64)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mju_negPose (engine/engine_util_spatial.h:75)
/// Calls: mji_negQuat, mji_rotVecQuat, mju_scl3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_neg_pose(posres: *mut f64, quatres: *mut f64, pos: *const f64, quat: *const f64) {
    // NOTE: signature changed from previous IR version
    // Previous params: (posres : * mut f64, quatres : * mut f64, pos : * const f64, quat : * const f64)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mju_trnVecPose (engine/engine_util_spatial.h:79)
/// Calls: mji_addTo3, mji_rotVecQuat
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_trn_vec_pose(res: *mut f64, pos: *const f64, quat: *const f64, vec: *const f64) {
    // NOTE: signature changed from previous IR version
    // Previous params: (res : * mut f64, pos : * const f64, quat : * const f64, vec : * const f64)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mju_euler2Quat (engine/engine_util_spatial.h:84)
/// Calls: mji_copy4, mju_message, mju_mulQuat
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_euler2quat(quat: *mut f64, euler: *const f64, seq: *const i8) {
    // NOTE: signature changed from previous IR version
    // Previous params: (quat : * mut f64, euler : * const f64, seq : * const i8)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mju_cross (engine/engine_util_spatial.h:89)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_cross(res: *mut f64, a: *const f64, b: *const f64) {
    // NOTE: signature changed from previous IR version
    // Previous params: (res : * mut f64, a : * const f64, b : * const f64)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mju_crossMotion (engine/engine_util_spatial.h:92)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_cross_motion(res: *mut f64, vel: *const f64, v: *const f64) {
    // NOTE: signature changed from previous IR version
    // Previous params: (res : * mut f64, vel : * const f64, v : * const f64)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mju_crossForce (engine/engine_util_spatial.h:95)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_cross_force(res: *mut f64, vel: *const f64, f: *const f64) {
    // NOTE: signature changed from previous IR version
    // Previous params: (res : * mut f64, vel : * const f64, f : * const f64)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mju_inertCom (engine/engine_util_spatial.h:98)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_inert_com(res: *mut f64, inert: *const f64, mat: *const f64, dif: *const f64, mass: f64) {
    unsafe {
        // SAFETY: caller guarantees inert points to 3 f64, mat points to 9 f64, dif points to 3 f64, res points to 10 f64
        let tmp: [f64; 9] = [
            *mat.add(0) * *inert.add(0),
            *mat.add(3) * *inert.add(0),
            *mat.add(6) * *inert.add(0),
            *mat.add(1) * *inert.add(1),
            *mat.add(4) * *inert.add(1),
            *mat.add(7) * *inert.add(1),
            *mat.add(2) * *inert.add(2),
            *mat.add(5) * *inert.add(2),
            *mat.add(8) * *inert.add(2),
        ];
        *res.add(0) = *mat.add(0) * tmp[0] + *mat.add(1) * tmp[3] + *mat.add(2) * tmp[6];
        *res.add(1) = *mat.add(3) * tmp[1] + *mat.add(4) * tmp[4] + *mat.add(5) * tmp[7];
        *res.add(2) = *mat.add(6) * tmp[2] + *mat.add(7) * tmp[5] + *mat.add(8) * tmp[8];
        *res.add(3) = *mat.add(0) * tmp[1] + *mat.add(1) * tmp[4] + *mat.add(2) * tmp[7];
        *res.add(4) = *mat.add(0) * tmp[2] + *mat.add(1) * tmp[5] + *mat.add(2) * tmp[8];
        *res.add(5) = *mat.add(3) * tmp[2] + *mat.add(4) * tmp[5] + *mat.add(5) * tmp[8];

        *res.add(0) += mass * (*dif.add(1) * *dif.add(1) + *dif.add(2) * *dif.add(2));
        *res.add(1) += mass * (*dif.add(0) * *dif.add(0) + *dif.add(2) * *dif.add(2));
        *res.add(2) += mass * (*dif.add(0) * *dif.add(0) + *dif.add(1) * *dif.add(1));
        *res.add(3) -= mass * *dif.add(0) * *dif.add(1);
        *res.add(4) -= mass * *dif.add(0) * *dif.add(2);
        *res.add(5) -= mass * *dif.add(1) * *dif.add(2);

        *res.add(6) = mass * *dif.add(0);
        *res.add(7) = mass * *dif.add(1);
        *res.add(8) = mass * *dif.add(2);
        *res.add(9) = mass;
    }
}

/// C: mju_dofCom (engine/engine_util_spatial.h:102)
/// Calls: mji_copy3, mji_cross, mju_zero3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_dof_com(res: *mut f64, axis: *const f64, offset: *const f64) {
    // NOTE: signature changed from previous IR version
    // Previous params: (res : * mut f64, axis : * const f64, offset : * const f64)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mju_mulInertVec (engine/engine_util_spatial.h:105)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_mul_inert_vec(res: *mut f64, inert: *const f64, vec: *const f64) {
    // NOTE: signature changed from previous IR version
    // Previous params: (res : * mut f64, inert : * const f64, vec : * const f64)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mju_mulDofVec (engine/engine_util_spatial.h:108)
/// Calls: mju_mulMatTVec, mju_scl, mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_mul_dof_vec(res: *mut f64, mat: *const f64, vec: *const f64, n: i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (res : * mut f64, mat : * const f64, vec : * const f64, n : i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mju_transformSpatial (engine/engine_util_spatial.h:112)
/// Calls: mji_copy6, mji_cross, mji_mulMatTVec3, mji_sub3, mju_copy
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_transform_spatial(res: *mut f64, vec: *const f64, flg_force: i32, newpos: *const f64, oldpos: *const f64, rotnew2old: *const f64) {
    // NOTE: signature changed from previous IR version
    // Previous params: (res : * mut f64, vec : * const f64, flg_force : i32, newpos : * const f64, oldpos : * const f64, rotnew2old : * const f64)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mju_makeFrame (engine/engine_util_spatial.h:117)
/// Calls: mji_cross, mji_scl3, mji_subFrom3, mju_dot3, mju_message, mju_normalize3, mju_zero3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_make_frame(frame: *mut f64) {
    // NOTE: signature changed from previous IR version
    // Previous params: (frame : * mut f64)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

