//! Port of: engine/engine_util_spatial.h
//! IR hash: 32301b9dc9774d55
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
    // SAFETY: caller guarantees res[3], vec[3], quat[4] are valid
    unsafe {
        // zero vec: zero res
        if *vec.add(0) == 0.0 && *vec.add(1) == 0.0 && *vec.add(2) == 0.0 {
            *res.add(0) = 0.0;
            *res.add(1) = 0.0;
            *res.add(2) = 0.0;
        }
        // null quat: copy vec
        else if *quat.add(0) == 1.0 && *quat.add(1) == 0.0 && *quat.add(2) == 0.0 && *quat.add(3) == 0.0 {
            *res.add(0) = *vec.add(0);
            *res.add(1) = *vec.add(1);
            *res.add(2) = *vec.add(2);
        }
        // regular processing
        else {
            // tmp = q_w * v + cross(q_xyz, v)
            let tmp0 = *quat.add(0) * *vec.add(0) + *quat.add(2) * *vec.add(2) - *quat.add(3) * *vec.add(1);
            let tmp1 = *quat.add(0) * *vec.add(1) + *quat.add(3) * *vec.add(0) - *quat.add(1) * *vec.add(2);
            let tmp2 = *quat.add(0) * *vec.add(2) + *quat.add(1) * *vec.add(1) - *quat.add(2) * *vec.add(0);

            // res = v + 2 * cross(q_xyz, t)
            *res.add(0) = *vec.add(0) + 2.0 * (*quat.add(2) * tmp2 - *quat.add(3) * tmp1);
            *res.add(1) = *vec.add(1) + 2.0 * (*quat.add(3) * tmp0 - *quat.add(1) * tmp2);
            *res.add(2) = *vec.add(2) + 2.0 * (*quat.add(1) * tmp1 - *quat.add(2) * tmp0);
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
    // SAFETY: caller guarantees res[4] and quat[4] are valid
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
    // SAFETY: caller guarantees res[4], quat1[4], quat2[4] are valid
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
    // SAFETY: caller guarantees res[4], quat[4], axis[3] are valid
    unsafe {
        let tmp0 = -*quat.add(1) * *axis.add(0) - *quat.add(2) * *axis.add(1) - *quat.add(3) * *axis.add(2);
        let tmp1 = *quat.add(0) * *axis.add(0) + *quat.add(2) * *axis.add(2) - *quat.add(3) * *axis.add(1);
        let tmp2 = *quat.add(0) * *axis.add(1) + *quat.add(3) * *axis.add(0) - *quat.add(1) * *axis.add(2);
        let tmp3 = *quat.add(0) * *axis.add(2) + *quat.add(1) * *axis.add(1) - *quat.add(2) * *axis.add(0);
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
    const MJ_PI: f64 = 3.14159265358979323846_f64;
    // SAFETY: caller guarantees res[3] and quat[4] are valid
    unsafe {
        let mut axis: [f64; 3] = [*quat.add(1), *quat.add(2), *quat.add(3)];
        let sin_a_2 = crate::engine::engine_util_blas::mju_normalize3(axis.as_mut_ptr());
        let mut speed = 2.0 * f64::atan2(sin_a_2, *quat.add(0));

        // when axis-angle is larger than pi, rotation is in the opposite direction
        if speed > MJ_PI {
            speed -= 2.0 * MJ_PI;
        }
        speed /= dt;

        *res.add(0) = axis[0] * speed;
        *res.add(1) = axis[1] * speed;
        *res.add(2) = axis[2] * speed;
    }
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
    // qdif = neg(qb)*qa
    let mut qneg: [f64; 4] = [0.0; 4];
    let mut qdif: [f64; 4] = [0.0; 4];
    mju_neg_quat(qneg.as_mut_ptr(), qb);
    mju_mul_quat(qdif.as_mut_ptr(), qneg.as_ptr(), qa);

    // convert to velocity
    mju_quat2vel(res, qdif.as_ptr(), 1.0);
}

/// C: mju_quat2Mat (engine/engine_util_spatial.h:48)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_quat2mat(res: *mut f64, quat: *const f64) {
    // SAFETY: caller guarantees res[9] and quat[4] are valid
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
    // SAFETY: caller guarantees quat[4] and mat[9] are valid
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
    }
    crate::engine::engine_util_blas::mju_normalize4(quat);
}

/// C: mju_derivQuat (engine/engine_util_spatial.h:54)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_deriv_quat(res: *mut f64, quat: *const f64, vel: *const f64) {
    // SAFETY: caller guarantees res[4], quat[4], vel[3] are valid
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
    // SAFETY: caller guarantees quat[4] and vel[3] are valid
    unsafe {
        let mut tmp: [f64; 3] = [*vel.add(0), *vel.add(1), *vel.add(2)];
        let angle = scale * crate::engine::engine_util_blas::mju_normalize3(tmp.as_mut_ptr());

        let mut qrot: [f64; 4] = [0.0; 4];
        mju_axis_angle2quat(qrot.as_mut_ptr(), tmp.as_ptr(), angle);
        crate::engine::engine_util_blas::mju_normalize4(quat);
        mju_mul_quat(quat, quat, qrot.as_ptr());
    }
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
    const MJ_MINVAL: f64 = 1E-15_f64;
    // SAFETY: caller guarantees quat[4] and vec[3] are valid
    unsafe {
        let mut vn: [f64; 3] = [*vec.add(0), *vec.add(1), *vec.add(2)];
        let z: [f64; 3] = [0.0, 0.0, 1.0];

        // default: no-rotation quaternion
        *quat.add(0) = 1.0;
        *quat.add(1) = 0.0;
        *quat.add(2) = 0.0;
        *quat.add(3) = 0.0;

        // normalize vector; if too small, no rotation
        if crate::engine::engine_util_blas::mju_normalize3(vn.as_mut_ptr()) < MJ_MINVAL {
            return;
        }

        // compute angle and axis: axis = cross(z, vn)
        let mut axis: [f64; 3] = [
            z[1] * vn[2] - z[2] * vn[1],
            z[2] * vn[0] - z[0] * vn[2],
            z[0] * vn[1] - z[1] * vn[0],
        ];
        let a = crate::engine::engine_util_blas::mju_normalize3(axis.as_mut_ptr());

        // almost parallel
        if a.abs() < MJ_MINVAL {
            // opposite: 180 deg rotation around x axis
            let dot = vn[0] * z[0] + vn[1] * z[1] + vn[2] * z[2];
            if dot < 0.0 {
                *quat.add(0) = 0.0;
                *quat.add(1) = 1.0;
            }
            return;
        }

        // make quaternion from angle and axis
        let dot = vn[0] * z[0] + vn[1] * z[1] + vn[2] * z[2];
        let angle = f64::atan2(a, dot);
        mju_axis_angle2quat(quat, axis.as_ptr(), angle);
    }
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
    // SAFETY: caller guarantees posres[3], quatres[4], pos1[3], quat1[4], pos2[3], quat2[4]
    // quatres = quat1*quat2
    mju_mul_quat(quatres, quat1, quat2);
    crate::engine::engine_util_blas::mju_normalize4(quatres);

    // posres = quat1*pos2 + pos1
    mju_rot_vec_quat(posres, pos2, quat1);
    // SAFETY: posres[3] and pos1[3] valid
    unsafe {
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
    // qres = neg(quat)
    mju_neg_quat(quatres, quat);

    // pres = -neg(quat)*pos
    mju_rot_vec_quat(posres, pos, quatres);
    // SAFETY: posres[3] valid
    unsafe {
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
    // res = quat*vec + pos
    mju_rot_vec_quat(res, vec, quat);
    // SAFETY: res[3] and pos[3] valid
    unsafe {
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
    // SAFETY: res points to 3 f64, a points to 3 f64, b points to 3 f64 (caller contract)
    // Use tmp to handle aliasing (res may alias a or b)
    unsafe {
        let tmp0: f64 = *a.add(1) * *b.add(2) - *a.add(2) * *b.add(1);
        let tmp1: f64 = *a.add(2) * *b.add(0) - *a.add(0) * *b.add(2);
        let tmp2: f64 = *a.add(0) * *b.add(1) - *a.add(1) * *b.add(0);

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
        let tmp: [f64; 9] = [
            *mat.add(0) * *inert.add(0), *mat.add(3) * *inert.add(0), *mat.add(6) * *inert.add(0),
            *mat.add(1) * *inert.add(1), *mat.add(4) * *inert.add(1), *mat.add(7) * *inert.add(1),
            *mat.add(2) * *inert.add(2), *mat.add(5) * *inert.add(2), *mat.add(8) * *inert.add(2),
        ];

        // res_rot = mat * diag(inert) * mat'
        *res.add(0) = *mat.add(0) * tmp[0] + *mat.add(1) * tmp[3] + *mat.add(2) * tmp[6];
        *res.add(1) = *mat.add(3) * tmp[1] + *mat.add(4) * tmp[4] + *mat.add(5) * tmp[7];
        *res.add(2) = *mat.add(6) * tmp[2] + *mat.add(7) * tmp[5] + *mat.add(8) * tmp[8];
        *res.add(3) = *mat.add(0) * tmp[1] + *mat.add(1) * tmp[4] + *mat.add(2) * tmp[7];
        *res.add(4) = *mat.add(0) * tmp[2] + *mat.add(1) * tmp[5] + *mat.add(2) * tmp[8];
        *res.add(5) = *mat.add(3) * tmp[2] + *mat.add(4) * tmp[5] + *mat.add(5) * tmp[8];

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

