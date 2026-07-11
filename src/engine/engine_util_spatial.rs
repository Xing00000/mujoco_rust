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
    // SAFETY: caller guarantees res points to 3 f64, vec to 3 f64, quat to 4 f64.
    unsafe {
        let v0 = *vec.add(0);
        let v1 = *vec.add(1);
        let v2 = *vec.add(2);
        let q0 = *quat.add(0);
        let q1 = *quat.add(1);
        let q2 = *quat.add(2);
        let q3 = *quat.add(3);

        // zero vec: zero res
        if v0 == 0.0 && v1 == 0.0 && v2 == 0.0 {
            *res.add(0) = 0.0;
            *res.add(1) = 0.0;
            *res.add(2) = 0.0;
        }
        // null quat: copy vec
        else if q0 == 1.0 && q1 == 0.0 && q2 == 0.0 && q3 == 0.0 {
            *res.add(0) = v0;
            *res.add(1) = v1;
            *res.add(2) = v2;
        }
        // regular processing
        else {
            // tmp = q_w * v + cross(q_xyz, v)
            let tmp0 = q0 * v0 + q2 * v2 - q3 * v1;
            let tmp1 = q0 * v1 + q3 * v0 - q1 * v2;
            let tmp2 = q0 * v2 + q1 * v1 - q2 * v0;

            // res = v + 2 * cross(q_xyz, t)
            *res.add(0) = v0 + 2.0 * (q2 * tmp2 - q3 * tmp1);
            *res.add(1) = v1 + 2.0 * (q3 * tmp0 - q1 * tmp2);
            *res.add(2) = v2 + 2.0 * (q1 * tmp1 - q2 * tmp0);
        }
    }
}

/// C: mju_negQuat (engine/engine_util_spatial.h:30)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_neg_quat(res: *mut f64, quat: *const f64) {
    // SAFETY: caller guarantees res, quat point to at least 4 contiguous f64
    unsafe {
        *res.add(0) = *quat.add(0);
        *res.add(1) = -*quat.add(1);
        *res.add(2) = -*quat.add(2);
        *res.add(3) = -*quat.add(3);
    }
}

/// C: mju_mulQuat (engine/engine_util_spatial.h:33)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_mul_quat(res: *mut f64, quat1: *const f64, quat2: *const f64) {
    // SAFETY: caller guarantees res, quat1, quat2 point to at least 4 contiguous f64
    unsafe {
        let tmp0 = *quat1.add(0) * *quat2.add(0) - *quat1.add(1) * *quat2.add(1) - *quat1.add(2) * *quat2.add(2) - *quat1.add(3) * *quat2.add(3);
        let tmp1 = *quat1.add(0) * *quat2.add(1) + *quat1.add(1) * *quat2.add(0) + *quat1.add(2) * *quat2.add(3) - *quat1.add(3) * *quat2.add(2);
        let tmp2 = *quat1.add(0) * *quat2.add(2) - *quat1.add(1) * *quat2.add(3) + *quat1.add(2) * *quat2.add(0) + *quat1.add(3) * *quat2.add(1);
        let tmp3 = *quat1.add(0) * *quat2.add(3) + *quat1.add(1) * *quat2.add(2) - *quat1.add(2) * *quat2.add(1) + *quat1.add(3) * *quat2.add(0);
        *res.add(0) = tmp0;
        *res.add(1) = tmp1;
        *res.add(2) = tmp2;
        *res.add(3) = tmp3;
    }
}

/// C: mju_mulQuatAxis (engine/engine_util_spatial.h:36)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_mul_quat_axis(res: *mut f64, quat: *const f64, axis: *const f64) {
    // SAFETY: caller guarantees res points to 4 f64, quat points to 4 f64, axis points to 3 f64
    unsafe {
        let tmp0 = -*quat.add(1) * *axis.add(0) - *quat.add(2) * *axis.add(1) - *quat.add(3) * *axis.add(2);
        let tmp1 =  *quat.add(0) * *axis.add(0) + *quat.add(2) * *axis.add(2) - *quat.add(3) * *axis.add(1);
        let tmp2 =  *quat.add(0) * *axis.add(1) + *quat.add(3) * *axis.add(0) - *quat.add(1) * *axis.add(2);
        let tmp3 =  *quat.add(0) * *axis.add(2) + *quat.add(1) * *axis.add(1) - *quat.add(2) * *axis.add(0);
        *res.add(0) = tmp0;
        *res.add(1) = tmp1;
        *res.add(2) = tmp2;
        *res.add(3) = tmp3;
    }
}

/// C: mju_axisAngle2Quat (engine/engine_util_spatial.h:39)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_axis_angle2quat(res: *mut f64, axis: *const f64, angle: f64) {
    // SAFETY: caller guarantees res[4] and axis[3] are valid
    unsafe {
        if angle == 0.0 {
            *res.add(0) = 1.0;
            *res.add(1) = 0.0;
            *res.add(2) = 0.0;
            *res.add(3) = 0.0;
        } else {
            let s = (angle * 0.5).sin();
            *res.add(0) = (angle * 0.5).cos();
            *res.add(1) = *axis.add(0) * s;
            *res.add(2) = *axis.add(1) * s;
            *res.add(3) = *axis.add(2) * s;
        }
    }
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
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut f64, quat : * const f64, dt : f64)
    // Previous return: ()
    todo ! ()
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
    const MJMINVAL: f64 = 1e-15;
    const MJPI: f64 = 3.14159265358979323846;

    // SAFETY: caller guarantees res points to 3 f64, qa/qb to 4 f64.
    unsafe {
        // qneg = neg(qb)
        let qneg0 = *qb.add(0);
        let qneg1 = -*qb.add(1);
        let qneg2 = -*qb.add(2);
        let qneg3 = -*qb.add(3);

        // qdif = qneg * qa (mji_mulQuat inlined)
        let a0 = *qa.add(0);
        let a1 = *qa.add(1);
        let a2 = *qa.add(2);
        let a3 = *qa.add(3);
        let qdif0 = qneg0 * a0 - qneg1 * a1 - qneg2 * a2 - qneg3 * a3;
        let qdif1 = qneg0 * a1 + qneg1 * a0 + qneg2 * a3 - qneg3 * a2;
        let qdif2 = qneg0 * a2 - qneg1 * a3 + qneg2 * a0 + qneg3 * a1;
        let qdif3 = qneg0 * a3 + qneg1 * a2 - qneg2 * a1 + qneg3 * a0;

        // mji_quat2Vel(res, qdif, 1) inlined
        let mut axis0 = qdif1;
        let mut axis1 = qdif2;
        let mut axis2 = qdif3;

        // mji__normalize3(axis) inlined
        let norm = (axis0 * axis0 + axis1 * axis1 + axis2 * axis2).sqrt();
        if norm < MJMINVAL {
            axis0 = 1.0;
            axis1 = 0.0;
            axis2 = 0.0;
        } else {
            let norm_inv = 1.0 / norm;
            axis0 *= norm_inv;
            axis1 *= norm_inv;
            axis2 *= norm_inv;
        }
        let sin_a_2 = norm;

        let mut speed = 2.0 * sin_a_2.atan2(qdif0);

        // when axis-angle is larger than pi, rotation is in the opposite direction
        if speed > MJPI {
            speed -= 2.0 * MJPI;
        }
        // speed /= dt (dt=1)

        // mji_scl3(res, axis, speed)
        *res.add(0) = axis0 * speed;
        *res.add(1) = axis1 * speed;
        *res.add(2) = axis2 * speed;
    }
}

/// C: mju_quat2Mat (engine/engine_util_spatial.h:48)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_quat2mat(res: *mut f64, quat: *const f64) {
    // SAFETY: caller guarantees res points to 9 f64, quat points to 4 f64
    unsafe {
        // null quat: identity
        if *quat.add(0) == 1.0 && *quat.add(1) == 0.0 && *quat.add(2) == 0.0 && *quat.add(3) == 0.0 {
            *res.add(0) = 1.0;
            *res.add(1) = 0.0;
            *res.add(2) = 0.0;
            *res.add(3) = 0.0;
            *res.add(4) = 1.0;
            *res.add(5) = 0.0;
            *res.add(6) = 0.0;
            *res.add(7) = 0.0;
            *res.add(8) = 1.0;
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
    use crate::engine::engine_util_blas::mju_normalize4;

    // SAFETY: caller guarantees quat points to 4 f64, mat points to 9 f64
    unsafe {
        // q0 largest
        if *mat.add(0) + *mat.add(4) + *mat.add(8) > 0.0 {
            *quat.add(0) = 0.5 * (1.0 + *mat.add(0) + *mat.add(4) + *mat.add(8)).sqrt();
            *quat.add(1) = 0.25 * (*mat.add(7) - *mat.add(5)) / *quat.add(0);
            *quat.add(2) = 0.25 * (*mat.add(2) - *mat.add(6)) / *quat.add(0);
            *quat.add(3) = 0.25 * (*mat.add(3) - *mat.add(1)) / *quat.add(0);
        }
        // q1 largest
        else if *mat.add(0) > *mat.add(4) && *mat.add(0) > *mat.add(8) {
            *quat.add(1) = 0.5 * (1.0 + *mat.add(0) - *mat.add(4) - *mat.add(8)).sqrt();
            *quat.add(0) = 0.25 * (*mat.add(7) - *mat.add(5)) / *quat.add(1);
            *quat.add(2) = 0.25 * (*mat.add(1) + *mat.add(3)) / *quat.add(1);
            *quat.add(3) = 0.25 * (*mat.add(2) + *mat.add(6)) / *quat.add(1);
        }
        // q2 largest
        else if *mat.add(4) > *mat.add(8) {
            *quat.add(2) = 0.5 * (1.0 - *mat.add(0) + *mat.add(4) - *mat.add(8)).sqrt();
            *quat.add(0) = 0.25 * (*mat.add(2) - *mat.add(6)) / *quat.add(2);
            *quat.add(1) = 0.25 * (*mat.add(1) + *mat.add(3)) / *quat.add(2);
            *quat.add(3) = 0.25 * (*mat.add(5) + *mat.add(7)) / *quat.add(2);
        }
        // q3 largest
        else {
            *quat.add(3) = 0.5 * (1.0 - *mat.add(0) - *mat.add(4) + *mat.add(8)).sqrt();
            *quat.add(0) = 0.25 * (*mat.add(3) - *mat.add(1)) / *quat.add(3);
            *quat.add(1) = 0.25 * (*mat.add(2) + *mat.add(6)) / *quat.add(3);
            *quat.add(2) = 0.25 * (*mat.add(5) + *mat.add(7)) / *quat.add(3);
        }

        mju_normalize4(quat);
    }
}

/// C: mju_derivQuat (engine/engine_util_spatial.h:54)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_deriv_quat(res: *mut f64, quat: *const f64, vel: *const f64) {
    // SAFETY: caller guarantees res points to 4 f64, quat points to 4 f64, vel points to 3 f64
    unsafe {
        *res.add(0) = 0.5 * (-*vel.add(0) * *quat.add(1) - *vel.add(1) * *quat.add(2) - *vel.add(2) * *quat.add(3));
        *res.add(1) = 0.5 * ( *vel.add(0) * *quat.add(0) + *vel.add(1) * *quat.add(3) - *vel.add(2) * *quat.add(2));
        *res.add(2) = 0.5 * (-*vel.add(0) * *quat.add(3) + *vel.add(1) * *quat.add(0) + *vel.add(2) * *quat.add(1));
        *res.add(3) = 0.5 * ( *vel.add(0) * *quat.add(2) - *vel.add(1) * *quat.add(1) + *vel.add(2) * *quat.add(0));
    }
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
    // WARNING: signature changed — verify body
    // Previous params: (quat : * mut f64, vel : * const f64, scale : f64)
    // Previous return: ()
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (quat : * mut f64, vec : * const f64)
    // Previous return: ()
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (quat : * mut f64, mat : * const f64)
    // Previous return: i32
    todo ! ()
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
    // SAFETY: caller guarantees all pointers are valid and non-overlapping.
    unsafe {
        // quatres = quat1 * quat2 (mji_mulQuat inlined)
        let q1_0 = *quat1.add(0);
        let q1_1 = *quat1.add(1);
        let q1_2 = *quat1.add(2);
        let q1_3 = *quat1.add(3);
        let q2_0 = *quat2.add(0);
        let q2_1 = *quat2.add(1);
        let q2_2 = *quat2.add(2);
        let q2_3 = *quat2.add(3);
        *quatres.add(0) = q1_0 * q2_0 - q1_1 * q2_1 - q1_2 * q2_2 - q1_3 * q2_3;
        *quatres.add(1) = q1_0 * q2_1 + q1_1 * q2_0 + q1_2 * q2_3 - q1_3 * q2_2;
        *quatres.add(2) = q1_0 * q2_2 - q1_1 * q2_3 + q1_2 * q2_0 + q1_3 * q2_1;
        *quatres.add(3) = q1_0 * q2_3 + q1_1 * q2_2 - q1_2 * q2_1 + q1_3 * q2_0;

        // mju_normalize4(quatres)
        crate::engine::engine_util_blas::mju_normalize4(quatres);

        // posres = mji_rotVecQuat(pos2, quat1) + pos1
        let p2_0 = *pos2.add(0);
        let p2_1 = *pos2.add(1);
        let p2_2 = *pos2.add(2);
        // mji_rotVecQuat inlined (using quat1)
        if q1_0 == 1.0 && q1_1 == 0.0 && q1_2 == 0.0 && q1_3 == 0.0 {
            *posres.add(0) = p2_0;
            *posres.add(1) = p2_1;
            *posres.add(2) = p2_2;
        } else {
            let tmp0 = q1_0 * p2_0 + q1_2 * p2_2 - q1_3 * p2_1;
            let tmp1 = q1_0 * p2_1 + q1_3 * p2_0 - q1_1 * p2_2;
            let tmp2 = q1_0 * p2_2 + q1_1 * p2_1 - q1_2 * p2_0;
            *posres.add(0) = p2_0 + 2.0 * (q1_2 * tmp2 - q1_3 * tmp1);
            *posres.add(1) = p2_1 + 2.0 * (q1_3 * tmp0 - q1_1 * tmp2);
            *posres.add(2) = p2_2 + 2.0 * (q1_1 * tmp1 - q1_2 * tmp0);
        }
        // mji_addTo3(posres, pos1)
        *posres.add(0) += *pos1.add(0);
        *posres.add(1) += *pos1.add(1);
        *posres.add(2) += *pos1.add(2);
    }
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
    // SAFETY: caller guarantees all pointers are valid.
    unsafe {
        let q0 = *quat.add(0);
        let q1 = *quat.add(1);
        let q2 = *quat.add(2);
        let q3 = *quat.add(3);

        // quatres = neg(quat)  (mji_negQuat inlined)
        *quatres.add(0) = q0;
        *quatres.add(1) = -q1;
        *quatres.add(2) = -q2;
        *quatres.add(3) = -q3;

        // posres = mji_rotVecQuat(pos, quatres)  using negated quat
        let nq0 = q0;
        let nq1 = -q1;
        let nq2 = -q2;
        let nq3 = -q3;
        let p0 = *pos.add(0);
        let p1 = *pos.add(1);
        let p2 = *pos.add(2);

        if nq0 == 1.0 && nq1 == 0.0 && nq2 == 0.0 && nq3 == 0.0 {
            *posres.add(0) = p0;
            *posres.add(1) = p1;
            *posres.add(2) = p2;
        } else {
            let tmp0 = nq0 * p0 + nq2 * p2 - nq3 * p1;
            let tmp1 = nq0 * p1 + nq3 * p0 - nq1 * p2;
            let tmp2 = nq0 * p2 + nq1 * p1 - nq2 * p0;
            *posres.add(0) = p0 + 2.0 * (nq2 * tmp2 - nq3 * tmp1);
            *posres.add(1) = p1 + 2.0 * (nq3 * tmp0 - nq1 * tmp2);
            *posres.add(2) = p2 + 2.0 * (nq1 * tmp1 - nq2 * tmp0);
        }

        // mju_scl3(posres, posres, -1)
        *posres.add(0) = -*posres.add(0);
        *posres.add(1) = -*posres.add(1);
        *posres.add(2) = -*posres.add(2);
    }
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
    // SAFETY: caller guarantees all pointers are valid and res != quat.
    unsafe {
        let q0 = *quat.add(0);
        let q1 = *quat.add(1);
        let q2 = *quat.add(2);
        let q3 = *quat.add(3);
        let v0 = *vec.add(0);
        let v1 = *vec.add(1);
        let v2 = *vec.add(2);

        // res = mji_rotVecQuat(vec, quat)
        if q0 == 1.0 && q1 == 0.0 && q2 == 0.0 && q3 == 0.0 {
            *res.add(0) = v0;
            *res.add(1) = v1;
            *res.add(2) = v2;
        } else {
            let tmp0 = q0 * v0 + q2 * v2 - q3 * v1;
            let tmp1 = q0 * v1 + q3 * v0 - q1 * v2;
            let tmp2 = q0 * v2 + q1 * v1 - q2 * v0;
            *res.add(0) = v0 + 2.0 * (q2 * tmp2 - q3 * tmp1);
            *res.add(1) = v1 + 2.0 * (q3 * tmp0 - q1 * tmp2);
            *res.add(2) = v2 + 2.0 * (q1 * tmp1 - q2 * tmp0);
        }

        // mji_addTo3(res, pos)
        *res.add(0) += *pos.add(0);
        *res.add(1) += *pos.add(1);
        *res.add(2) += *pos.add(2);
    }
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
    // SAFETY: caller guarantees quat points to 4 f64, euler to 3 f64,
    // seq is a valid C string with at least 3 characters.
    unsafe {
        // init
        let mut tmp = [1.0f64, 0.0, 0.0, 0.0];

        // loop over euler angles, accumulate rotations
        for i in 0..3 {
            let angle = *euler.add(i);
            let ca = (angle / 2.0).cos();
            let sa = (angle / 2.0).sin();
            let mut rot = [ca, 0.0f64, 0.0, 0.0];
            let ch = *seq.add(i) as u8;

            if ch == b'x' || ch == b'X' {
                rot[1] = sa;
            } else if ch == b'y' || ch == b'Y' {
                rot[2] = sa;
            } else if ch == b'z' || ch == b'Z' {
                rot[3] = sa;
            }

            // accumulate rotation: mulQuat inlined
            let mut res = [0.0f64; 4];
            if ch == b'x' || ch == b'y' || ch == b'z' {
                // moving axes: post-multiply: tmp = tmp * rot
                res[0] = tmp[0]*rot[0] - tmp[1]*rot[1] - tmp[2]*rot[2] - tmp[3]*rot[3];
                res[1] = tmp[0]*rot[1] + tmp[1]*rot[0] + tmp[2]*rot[3] - tmp[3]*rot[2];
                res[2] = tmp[0]*rot[2] - tmp[1]*rot[3] + tmp[2]*rot[0] + tmp[3]*rot[1];
                res[3] = tmp[0]*rot[3] + tmp[1]*rot[2] - tmp[2]*rot[1] + tmp[3]*rot[0];
            } else {
                // fixed axes: pre-multiply: tmp = rot * tmp
                res[0] = rot[0]*tmp[0] - rot[1]*tmp[1] - rot[2]*tmp[2] - rot[3]*tmp[3];
                res[1] = rot[0]*tmp[1] + rot[1]*tmp[0] + rot[2]*tmp[3] - rot[3]*tmp[2];
                res[2] = rot[0]*tmp[2] - rot[1]*tmp[3] + rot[2]*tmp[0] + rot[3]*tmp[1];
                res[3] = rot[0]*tmp[3] + rot[1]*tmp[2] - rot[2]*tmp[1] + rot[3]*tmp[0];
            }
            tmp = res;
        }

        // mji_copy4(quat, tmp)
        *quat.add(0) = tmp[0];
        *quat.add(1) = tmp[1];
        *quat.add(2) = tmp[2];
        *quat.add(3) = tmp[3];
    }
}

/// C: mju_cross (engine/engine_util_spatial.h:89)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_cross(res: *mut f64, a: *const f64, b: *const f64) {
    // SAFETY: caller guarantees res, a, b point to at least 3 contiguous f64
    unsafe {
        let tmp0 = *a.add(1) * *b.add(2) - *a.add(2) * *b.add(1);
        let tmp1 = *a.add(2) * *b.add(0) - *a.add(0) * *b.add(2);
        let tmp2 = *a.add(0) * *b.add(1) - *a.add(1) * *b.add(0);
        *res.add(0) = tmp0;
        *res.add(1) = tmp1;
        *res.add(2) = tmp2;
    }
}

/// C: mju_crossMotion (engine/engine_util_spatial.h:92)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_cross_motion(res: *mut f64, vel: *const f64, v: *const f64) {
    // SAFETY: caller guarantees res[6], vel[6], v[6] are valid
    unsafe {
        *res.add(0) = -*vel.add(2) * *v.add(1) + *vel.add(1) * *v.add(2);
        *res.add(1) =  *vel.add(2) * *v.add(0) - *vel.add(0) * *v.add(2);
        *res.add(2) = -*vel.add(1) * *v.add(0) + *vel.add(0) * *v.add(1);
        *res.add(3) = -*vel.add(2) * *v.add(4) + *vel.add(1) * *v.add(5);
        *res.add(4) =  *vel.add(2) * *v.add(3) - *vel.add(0) * *v.add(5);
        *res.add(5) = -*vel.add(1) * *v.add(3) + *vel.add(0) * *v.add(4);

        *res.add(3) += -*vel.add(5) * *v.add(1) + *vel.add(4) * *v.add(2);
        *res.add(4) +=  *vel.add(5) * *v.add(0) - *vel.add(3) * *v.add(2);
        *res.add(5) += -*vel.add(4) * *v.add(0) + *vel.add(3) * *v.add(1);
    }
}

/// C: mju_crossForce (engine/engine_util_spatial.h:95)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_cross_force(res: *mut f64, vel: *const f64, f: *const f64) {
    // SAFETY: caller guarantees res[6], vel[6], f[6] are valid
    unsafe {
        *res.add(0) = -*vel.add(2) * *f.add(1) + *vel.add(1) * *f.add(2);
        *res.add(1) =  *vel.add(2) * *f.add(0) - *vel.add(0) * *f.add(2);
        *res.add(2) = -*vel.add(1) * *f.add(0) + *vel.add(0) * *f.add(1);
        *res.add(3) = -*vel.add(2) * *f.add(4) + *vel.add(1) * *f.add(5);
        *res.add(4) =  *vel.add(2) * *f.add(3) - *vel.add(0) * *f.add(5);
        *res.add(5) = -*vel.add(1) * *f.add(3) + *vel.add(0) * *f.add(4);

        *res.add(0) += -*vel.add(5) * *f.add(4) + *vel.add(4) * *f.add(5);
        *res.add(1) +=  *vel.add(5) * *f.add(3) - *vel.add(3) * *f.add(5);
        *res.add(2) += -*vel.add(4) * *f.add(3) + *vel.add(3) * *f.add(4);
    }
}

/// C: mju_inertCom (engine/engine_util_spatial.h:98)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_inert_com(res: *mut f64, inert: *const f64, mat: *const f64, dif: *const f64, mass: f64) {
    // SAFETY: caller guarantees res[10], inert[3], mat[9], dif[3] are valid
    unsafe {
        // tmp = diag(inert) * mat'
        let tmp0 = *mat.add(0) * *inert.add(0);
        let tmp1 = *mat.add(3) * *inert.add(0);
        let tmp2 = *mat.add(6) * *inert.add(0);
        let tmp3 = *mat.add(1) * *inert.add(1);
        let tmp4 = *mat.add(4) * *inert.add(1);
        let tmp5 = *mat.add(7) * *inert.add(1);
        let tmp6 = *mat.add(2) * *inert.add(2);
        let tmp7 = *mat.add(5) * *inert.add(2);
        let tmp8 = *mat.add(8) * *inert.add(2);

        // res_rot = mat * diag(inert) * mat'
        *res.add(0) = *mat.add(0) * tmp0 + *mat.add(1) * tmp3 + *mat.add(2) * tmp6;
        *res.add(1) = *mat.add(3) * tmp1 + *mat.add(4) * tmp4 + *mat.add(5) * tmp7;
        *res.add(2) = *mat.add(6) * tmp2 + *mat.add(7) * tmp5 + *mat.add(8) * tmp8;
        *res.add(3) = *mat.add(0) * tmp1 + *mat.add(1) * tmp4 + *mat.add(2) * tmp7;
        *res.add(4) = *mat.add(0) * tmp2 + *mat.add(1) * tmp5 + *mat.add(2) * tmp8;
        *res.add(5) = *mat.add(3) * tmp2 + *mat.add(4) * tmp5 + *mat.add(5) * tmp8;

        // res_rot -= mass * dif_cross * dif_cross
        *res.add(0) += mass * (*dif.add(1) * *dif.add(1) + *dif.add(2) * *dif.add(2));
        *res.add(1) += mass * (*dif.add(0) * *dif.add(0) + *dif.add(2) * *dif.add(2));
        *res.add(2) += mass * (*dif.add(0) * *dif.add(0) + *dif.add(1) * *dif.add(1));
        *res.add(3) -= mass * *dif.add(0) * *dif.add(1);
        *res.add(4) -= mass * *dif.add(0) * *dif.add(2);
        *res.add(5) -= mass * *dif.add(1) * *dif.add(2);

        // res_tran = mass * dif
        *res.add(6) = mass * *dif.add(0);
        *res.add(7) = mass * *dif.add(1);
        *res.add(8) = mass * *dif.add(2);

        // res_mass = mass
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
    // SAFETY: caller guarantees res points to 6 f64, axis to 3 f64,
    // offset to 3 f64 (or null for slide joint).
    unsafe {
        if !offset.is_null() {
            // hinge: res[0:3] = axis, res[3:6] = cross(axis, offset)
            let a0 = *axis.add(0);
            let a1 = *axis.add(1);
            let a2 = *axis.add(2);
            *res.add(0) = a0;
            *res.add(1) = a1;
            *res.add(2) = a2;
            // mji_cross(res+3, axis, offset)
            let o0 = *offset.add(0);
            let o1 = *offset.add(1);
            let o2 = *offset.add(2);
            *res.add(3) = a1 * o2 - a2 * o1;
            *res.add(4) = a2 * o0 - a0 * o2;
            *res.add(5) = a0 * o1 - a1 * o0;
        } else {
            // slide: res[0:3] = 0, res[3:6] = axis
            *res.add(0) = 0.0;
            *res.add(1) = 0.0;
            *res.add(2) = 0.0;
            *res.add(3) = *axis.add(0);
            *res.add(4) = *axis.add(1);
            *res.add(5) = *axis.add(2);
        }
    }
}

/// C: mju_mulInertVec (engine/engine_util_spatial.h:105)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_mul_inert_vec(res: *mut f64, inert: *const f64, vec: *const f64) {
    // SAFETY: caller guarantees res points to 6 f64, inert points to 10 f64, vec points to 6 f64
    unsafe {
        *res.add(0) = *inert.add(0) * *vec.add(0) + *inert.add(3) * *vec.add(1) + *inert.add(4) * *vec.add(2) - *inert.add(8) * *vec.add(4) + *inert.add(7) * *vec.add(5);
        *res.add(1) = *inert.add(3) * *vec.add(0) + *inert.add(1) * *vec.add(1) + *inert.add(5) * *vec.add(2) + *inert.add(8) * *vec.add(3) - *inert.add(6) * *vec.add(5);
        *res.add(2) = *inert.add(4) * *vec.add(0) + *inert.add(5) * *vec.add(1) + *inert.add(2) * *vec.add(2) - *inert.add(7) * *vec.add(3) + *inert.add(6) * *vec.add(4);
        *res.add(3) = *inert.add(8) * *vec.add(1) - *inert.add(7) * *vec.add(2) + *inert.add(9) * *vec.add(3);
        *res.add(4) = *inert.add(6) * *vec.add(2) - *inert.add(8) * *vec.add(0) + *inert.add(9) * *vec.add(4);
        *res.add(5) = *inert.add(7) * *vec.add(0) - *inert.add(6) * *vec.add(1) + *inert.add(9) * *vec.add(5);
    }
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
    use crate::engine::engine_util_blas::{mju_scl, mju_zero, mju_mul_mat_t_vec};

    if n == 1 {
        // SAFETY: caller guarantees mat points to 6 f64, vec points to 1 f64
        unsafe {
            mju_scl(res, mat, *vec.add(0), 6);
        }
    } else if n <= 0 {
        mju_zero(res, 6);
    } else {
        mju_mul_mat_t_vec(res, mat, vec, n, 6);
    }
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
    // WARNING: signature changed — verify body
    // Previous params: (res : * mut f64, vec : * const f64, flg_force : i32, newpos : * const f64, oldpos : * const f64, rotnew2old : * const f64)
    // Previous return: ()
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (frame : * mut f64)
    // Previous return: ()
    todo ! ()
}

