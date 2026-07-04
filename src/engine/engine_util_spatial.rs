//! Port of: engine/engine_util_spatial.h
//! IR hash: 1b139f44af8230f9
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
    unsafe {
        // C: if (vec[0] == 0 && vec[1] == 0 && vec[2] == 0)
        if *vec.add(0) == 0.0 && *vec.add(1) == 0.0 && *vec.add(2) == 0.0 {
            crate::engine::engine_util_blas::mju_zero3(res); // C: mju_zero3(res)
        // C: else if (quat[0] == 1 && quat[1] == 0 && quat[2] == 0 && quat[3] == 0)
        } else if *quat.add(0) == 1.0 && *quat.add(1) == 0.0 && *quat.add(2) == 0.0 && *quat.add(3) == 0.0 {
            crate::engine::engine_util_blas::mju_copy3(res, vec); // C: mju_copy3(res, vec)
        } else {
            // C: mjtNum tmp[3] = { quat[0]*vec[0] + quat[2]*vec[2] - quat[3]*vec[1], ... }
            let mut tmp: [f64; 3] = [0.0; 3];
            tmp[0] = *quat.add(0) * *vec.add(0) + *quat.add(2) * *vec.add(2) - *quat.add(3) * *vec.add(1); // C: quat[0]*vec[0] + quat[2]*vec[2] - quat[3]*vec[1]
            tmp[1] = *quat.add(0) * *vec.add(1) + *quat.add(3) * *vec.add(0) - *quat.add(1) * *vec.add(2); // C: quat[0]*vec[1] + quat[3]*vec[0] - quat[1]*vec[2]
            tmp[2] = *quat.add(0) * *vec.add(2) + *quat.add(1) * *vec.add(1) - *quat.add(2) * *vec.add(0); // C: quat[0]*vec[2] + quat[1]*vec[1] - quat[2]*vec[0]
            *res.add(0) = *vec.add(0) + 2.0 * (*quat.add(2) * tmp[2] - *quat.add(3) * tmp[1]); // C: vec[0] + 2*(quat[2]*tmp[2] - quat[3]*tmp[1])
            *res.add(1) = *vec.add(1) + 2.0 * (*quat.add(3) * tmp[0] - *quat.add(1) * tmp[2]); // C: vec[1] + 2*(quat[3]*tmp[0] - quat[1]*tmp[2])
            *res.add(2) = *vec.add(2) + 2.0 * (*quat.add(1) * tmp[1] - *quat.add(2) * tmp[0]); // C: vec[2] + 2*(quat[1]*tmp[1] - quat[2]*tmp[0])
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
    unsafe {
        *res.add(0) = *quat.add(0);   // C: res[0] = quat[0]
        *res.add(1) = -*quat.add(1);  // C: res[1] = -quat[1]
        *res.add(2) = -*quat.add(2);  // C: res[2] = -quat[2]
        *res.add(3) = -*quat.add(3);  // C: res[3] = -quat[3]
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
    unsafe {
        // C: mjtNum tmp[4] = { qa[0]*qb[0] - qa[1]*qb[1] - qa[2]*qb[2] - qa[3]*qb[3], ... }
        let mut tmp: [f64; 4] = [0.0; 4];
        tmp[0] = *quat1.add(0) * *quat2.add(0) - *quat1.add(1) * *quat2.add(1) - *quat1.add(2) * *quat2.add(2) - *quat1.add(3) * *quat2.add(3); // C: qa[0]*qb[0] - qa[1]*qb[1] - qa[2]*qb[2] - qa[3]*qb[3]
        tmp[1] = *quat1.add(0) * *quat2.add(1) + *quat1.add(1) * *quat2.add(0) + *quat1.add(2) * *quat2.add(3) - *quat1.add(3) * *quat2.add(2); // C: qa[0]*qb[1] + qa[1]*qb[0] + qa[2]*qb[3] - qa[3]*qb[2]
        tmp[2] = *quat1.add(0) * *quat2.add(2) - *quat1.add(1) * *quat2.add(3) + *quat1.add(2) * *quat2.add(0) + *quat1.add(3) * *quat2.add(1); // C: qa[0]*qb[2] - qa[1]*qb[3] + qa[2]*qb[0] + qa[3]*qb[1]
        tmp[3] = *quat1.add(0) * *quat2.add(3) + *quat1.add(1) * *quat2.add(2) - *quat1.add(2) * *quat2.add(1) + *quat1.add(3) * *quat2.add(0); // C: qa[0]*qb[3] + qa[1]*qb[2] - qa[2]*qb[1] + qa[3]*qb[0]
        *res.add(0) = tmp[0]; // C: res[0] = tmp[0]
        *res.add(1) = tmp[1]; // C: res[1] = tmp[1]
        *res.add(2) = tmp[2]; // C: res[2] = tmp[2]
        *res.add(3) = tmp[3]; // C: res[3] = tmp[3]
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
    unsafe {
        // C: mjtNum tmp[4] = { -quat[1]*axis[0] - quat[2]*axis[1] - quat[3]*axis[2], ... }
        let mut tmp: [f64; 4] = [0.0; 4];
        tmp[0] = -*quat.add(1) * *axis.add(0) - *quat.add(2) * *axis.add(1) - *quat.add(3) * *axis.add(2); // C: -quat[1]*axis[0] - quat[2]*axis[1] - quat[3]*axis[2]
        tmp[1] = *quat.add(0) * *axis.add(0) + *quat.add(2) * *axis.add(2) - *quat.add(3) * *axis.add(1); // C: quat[0]*axis[0] + quat[2]*axis[2] - quat[3]*axis[1]
        tmp[2] = *quat.add(0) * *axis.add(1) + *quat.add(3) * *axis.add(0) - *quat.add(1) * *axis.add(2); // C: quat[0]*axis[1] + quat[3]*axis[0] - quat[1]*axis[2]
        tmp[3] = *quat.add(0) * *axis.add(2) + *quat.add(1) * *axis.add(1) - *quat.add(2) * *axis.add(0); // C: quat[0]*axis[2] + quat[1]*axis[1] - quat[2]*axis[0]
        *res.add(0) = tmp[0]; // C: res[0] = tmp[0]
        *res.add(1) = tmp[1]; // C: res[1] = tmp[1]
        *res.add(2) = tmp[2]; // C: res[2] = tmp[2]
        *res.add(3) = tmp[3]; // C: res[3] = tmp[3]
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
    unsafe {
        // C: if (angle == 0.0)
        if angle == 0.0 {
            *res.add(0) = 1.0; // C: res[0] = 1
            *res.add(1) = 0.0; // C: res[1] = 0
            *res.add(2) = 0.0; // C: res[2] = 0
            *res.add(3) = 0.0; // C: res[3] = 0
        } else {
            let s: f64 = (angle * 0.5).sin();   // C: mjtNum s = mju_sin(angle*0.5)
            *res.add(0) = (angle * 0.5).cos();  // C: res[0] = mju_cos(angle*0.5)
            *res.add(1) = *axis.add(0) * s;     // C: res[1] = axis[0]*s
            *res.add(2) = *axis.add(1) * s;     // C: res[2] = axis[1]*s
            *res.add(3) = *axis.add(2) * s;     // C: res[3] = axis[2]*s
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
    unsafe {
        // C: mjtNum axis[3] = {quat[1], quat[2], quat[3]}
        let mut axis: [f64; 3] = [*quat.add(1), *quat.add(2), *quat.add(3)];
        // C: mjtNum sin_a_2 = mju_normalize3(axis)
        let sin_a_2: f64 = crate::engine::engine_util_blas::mju_normalize3(axis.as_mut_ptr());
        // C: mjtNum speed = 2 * mju_atan2(sin_a_2, quat[0])
        let mut speed: f64 = 2.0 * sin_a_2.atan2(*quat.add(0));
        // C: if (speed > mjPI) { speed -= 2*mjPI; }
        if speed > std::f64::consts::PI {
            speed -= 2.0 * std::f64::consts::PI;
        }
        // C: speed /= dt
        speed /= dt;
        // C: mju_scl3(res, axis, speed)
        crate::engine::engine_util_blas::mju_scl3(res, axis.as_ptr(), speed);
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
    unsafe {
        // C: mjtNum qneg[4], qdif[4]
        let mut qneg: [f64; 4] = [0.0; 4];
        let mut qdif: [f64; 4] = [0.0; 4];
        // C: mju_negQuat(qneg, qb)
        mju_neg_quat(qneg.as_mut_ptr(), qb);
        // C: mju_mulQuat(qdif, qneg, qa)
        mju_mul_quat(qdif.as_mut_ptr(), qneg.as_ptr(), qa);
        // C: mju_quat2Vel(res, qdif, 1)
        mju_quat2vel(res, qdif.as_ptr(), 1.0);
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
    unsafe {
        // C: if (quat[0] == 1 && quat[1] == 0 && quat[2] == 0 && quat[3] == 0)
        if *quat.add(0) == 1.0 && *quat.add(1) == 0.0 && *quat.add(2) == 0.0 && *quat.add(3) == 0.0 {
            *res.add(0) = 1.0; // C: res[0]=1
            *res.add(1) = 0.0; // C: res[1]=0
            *res.add(2) = 0.0; // C: res[2]=0
            *res.add(3) = 0.0; // C: res[3]=0
            *res.add(4) = 1.0; // C: res[4]=1
            *res.add(5) = 0.0; // C: res[5]=0
            *res.add(6) = 0.0; // C: res[6]=0
            *res.add(7) = 0.0; // C: res[7]=0
            *res.add(8) = 1.0; // C: res[8]=1
        } else {
            let q00: f64 = *quat.add(0) * *quat.add(0); // C: q00=quat[0]*quat[0]
            let q01: f64 = *quat.add(0) * *quat.add(1); // C: q01=quat[0]*quat[1]
            let q02: f64 = *quat.add(0) * *quat.add(2); // C: q02=quat[0]*quat[2]
            let q03: f64 = *quat.add(0) * *quat.add(3); // C: q03=quat[0]*quat[3]
            let q11: f64 = *quat.add(1) * *quat.add(1); // C: q11=quat[1]*quat[1]
            let q12: f64 = *quat.add(1) * *quat.add(2); // C: q12=quat[1]*quat[2]
            let q13: f64 = *quat.add(1) * *quat.add(3); // C: q13=quat[1]*quat[3]
            let q22: f64 = *quat.add(2) * *quat.add(2); // C: q22=quat[2]*quat[2]
            let q23: f64 = *quat.add(2) * *quat.add(3); // C: q23=quat[2]*quat[3]
            let q33: f64 = *quat.add(3) * *quat.add(3); // C: q33=quat[3]*quat[3]
            *res.add(0) = q00 + q11 - q22 - q33; // C: res[0]=q00+q11-q22-q33
            *res.add(4) = q00 - q11 + q22 - q33; // C: res[4]=q00-q11+q22-q33
            *res.add(8) = q00 - q11 - q22 + q33; // C: res[8]=q00-q11-q22+q33
            *res.add(1) = 2.0 * (q12 - q03); // C: res[1]=2.0*(q12-q03)
            *res.add(2) = 2.0 * (q13 + q02); // C: res[2]=2.0*(q13+q02)
            *res.add(3) = 2.0 * (q12 + q03); // C: res[3]=2.0*(q12+q03)
            *res.add(5) = 2.0 * (q23 - q01); // C: res[5]=2.0*(q23-q01)
            *res.add(6) = 2.0 * (q13 - q02); // C: res[6]=2.0*(q13-q02)
            *res.add(7) = 2.0 * (q23 + q01); // C: res[7]=2.0*(q23+q01)
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
    unsafe {
        // C: if (mat[0]+mat[4]+mat[8] > 0)
        if *mat.add(0) + *mat.add(4) + *mat.add(8) > 0.0 {
            *quat.add(0) = 0.5 * (1.0 + *mat.add(0) + *mat.add(4) + *mat.add(8)).sqrt(); // C: quat[0] = 0.5 * mju_sqrt(1+mat[0]+mat[4]+mat[8])
            *quat.add(1) = 0.25 * (*mat.add(7) - *mat.add(5)) / *quat.add(0); // C: quat[1] = 0.25*(mat[7]-mat[5])/quat[0]
            *quat.add(2) = 0.25 * (*mat.add(2) - *mat.add(6)) / *quat.add(0); // C: quat[2] = 0.25*(mat[2]-mat[6])/quat[0]
            *quat.add(3) = 0.25 * (*mat.add(3) - *mat.add(1)) / *quat.add(0); // C: quat[3] = 0.25*(mat[3]-mat[1])/quat[0]
        // C: else if (mat[0]>mat[4] && mat[0]>mat[8])
        } else if *mat.add(0) > *mat.add(4) && *mat.add(0) > *mat.add(8) {
            *quat.add(1) = 0.5 * (1.0 + *mat.add(0) - *mat.add(4) - *mat.add(8)).sqrt(); // C: quat[1] = 0.5*mju_sqrt(1+mat[0]-mat[4]-mat[8])
            *quat.add(0) = 0.25 * (*mat.add(7) - *mat.add(5)) / *quat.add(1); // C: quat[0] = 0.25*(mat[7]-mat[5])/quat[1]
            *quat.add(2) = 0.25 * (*mat.add(1) + *mat.add(3)) / *quat.add(1); // C: quat[2] = 0.25*(mat[1]+mat[3])/quat[1]
            *quat.add(3) = 0.25 * (*mat.add(2) + *mat.add(6)) / *quat.add(1); // C: quat[3] = 0.25*(mat[2]+mat[6])/quat[1]
        // C: else if (mat[4]>mat[8])
        } else if *mat.add(4) > *mat.add(8) {
            *quat.add(2) = 0.5 * (1.0 - *mat.add(0) + *mat.add(4) - *mat.add(8)).sqrt(); // C: quat[2] = 0.5*mju_sqrt(1-mat[0]+mat[4]-mat[8])
            *quat.add(0) = 0.25 * (*mat.add(2) - *mat.add(6)) / *quat.add(2); // C: quat[0] = 0.25*(mat[2]-mat[6])/quat[2]
            *quat.add(1) = 0.25 * (*mat.add(1) + *mat.add(3)) / *quat.add(2); // C: quat[1] = 0.25*(mat[1]+mat[3])/quat[2]
            *quat.add(3) = 0.25 * (*mat.add(5) + *mat.add(7)) / *quat.add(2); // C: quat[3] = 0.25*(mat[5]+mat[7])/quat[2]
        } else {
            *quat.add(3) = 0.5 * (1.0 - *mat.add(0) - *mat.add(4) + *mat.add(8)).sqrt(); // C: quat[3] = 0.5*mju_sqrt(1-mat[0]-mat[4]+mat[8])
            *quat.add(0) = 0.25 * (*mat.add(3) - *mat.add(1)) / *quat.add(3); // C: quat[0] = 0.25*(mat[3]-mat[1])/quat[3]
            *quat.add(1) = 0.25 * (*mat.add(2) + *mat.add(6)) / *quat.add(3); // C: quat[1] = 0.25*(mat[2]+mat[6])/quat[3]
            *quat.add(2) = 0.25 * (*mat.add(5) + *mat.add(7)) / *quat.add(3); // C: quat[2] = 0.25*(mat[5]+mat[7])/quat[3]
        }
        // C: mju_normalize4(quat)
        crate::engine::engine_util_blas::mju_normalize4(quat);
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
    unsafe {
        *res.add(0) = 0.5 * (-*vel.add(0) * *quat.add(1) - *vel.add(1) * *quat.add(2) - *vel.add(2) * *quat.add(3)); // C: res[0] = 0.5*(-vel[0]*quat[1] - vel[1]*quat[2] - vel[2]*quat[3])
        *res.add(1) = 0.5 * (*vel.add(0) * *quat.add(0) + *vel.add(1) * *quat.add(3) - *vel.add(2) * *quat.add(2)); // C: res[1] = 0.5*( vel[0]*quat[0] + vel[1]*quat[3] - vel[2]*quat[2])
        *res.add(2) = 0.5 * (-*vel.add(0) * *quat.add(3) + *vel.add(1) * *quat.add(0) + *vel.add(2) * *quat.add(1)); // C: res[2] = 0.5*(-vel[0]*quat[3] + vel[1]*quat[0] + vel[2]*quat[1])
        *res.add(3) = 0.5 * (*vel.add(0) * *quat.add(2) - *vel.add(1) * *quat.add(1) + *vel.add(2) * *quat.add(0)); // C: res[3] = 0.5*( vel[0]*quat[2] - vel[1]*quat[1] + vel[2]*quat[0])
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
    unsafe {
        // C: mjtNum angle, tmp[4], qrot[4]
        let mut tmp: [f64; 4] = [0.0; 4];
        let mut qrot: [f64; 4] = [0.0; 4];
        // C: mju_copy3(tmp, vel)
        crate::engine::engine_util_blas::mju_copy3(tmp.as_mut_ptr(), vel);
        // C: angle = scale * mju_normalize3(tmp)
        let angle: f64 = scale * crate::engine::engine_util_blas::mju_normalize3(tmp.as_mut_ptr());
        // C: mju_axisAngle2Quat(qrot, tmp, angle)
        mju_axis_angle2quat(qrot.as_mut_ptr(), tmp.as_ptr(), angle);
        // C: mju_normalize4(quat)
        crate::engine::engine_util_blas::mju_normalize4(quat);
        // C: mju_mulQuat(quat, quat, qrot) — quat is both src and dst, use local copy
        let quat_copy: [f64; 4] = [*quat.add(0), *quat.add(1), *quat.add(2), *quat.add(3)];
        mju_mul_quat(quat, quat_copy.as_ptr(), qrot.as_ptr());
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
    unsafe {
        // C: mjtNum axis[3], a, vn[3] = {vec[0], vec[1], vec[2]}, z[3] = {0, 0, 1}
        let mut axis: [f64; 3] = [0.0; 3];
        let mut vn: [f64; 3] = [*vec.add(0), *vec.add(1), *vec.add(2)];
        let z: [f64; 3] = [0.0, 0.0, 1.0];
        // C: quat[0] = 1; mju_zero3(quat+1)
        *quat.add(0) = 1.0;
        crate::engine::engine_util_blas::mju_zero3(quat.add(1));
        // C: if (mju_normalize3(vn) < mjMINVAL) { return; }
        if crate::engine::engine_util_blas::mju_normalize3(vn.as_mut_ptr()) < 1e-15_f64 {
            return;
        }
        // C: mju_cross(axis, z, vn)
        mju_cross(axis.as_mut_ptr(), z.as_ptr(), vn.as_ptr());
        // C: a = mju_normalize3(axis)
        let mut a: f64 = crate::engine::engine_util_blas::mju_normalize3(axis.as_mut_ptr());
        // C: if (mju_abs(a) < mjMINVAL)
        if a.abs() < 1e-15_f64 {
            // C: if (mju_dot3(vn, z) < 0.0) { quat[0] = 0; quat[1] = 1; }
            if crate::engine::engine_util_blas::mju_dot3(vn.as_ptr(), z.as_ptr()) < 0.0 {
                *quat.add(0) = 0.0;
                *quat.add(1) = 1.0;
            }
            return;
        }
        // C: a = mju_atan2(a, mju_dot3(vn, z))
        a = a.atan2(crate::engine::engine_util_blas::mju_dot3(vn.as_ptr(), z.as_ptr()));
        // C: mju_axisAngle2Quat(quat, axis, a)
        mju_axis_angle2quat(quat, axis.as_ptr(), a);
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
    unsafe {
        const ROT_EPS: f64 = 1e-9; // C: static const mjtNum rotEPS = 1e-9

        let mut iter: i32 = 0; // C: int iter
        // C: mjtNum col1_mat[3] = {mat[0], mat[3], mat[6]}
        let col1_mat: [f64; 3] = [*mat.add(0), *mat.add(3), *mat.add(6)];
        // C: mjtNum col2_mat[3] = {mat[1], mat[4], mat[7]}
        let col2_mat: [f64; 3] = [*mat.add(1), *mat.add(4), *mat.add(7)];
        // C: mjtNum col3_mat[3] = {mat[2], mat[5], mat[8]}
        let col3_mat: [f64; 3] = [*mat.add(2), *mat.add(5), *mat.add(8)];

        // C: for (iter = 0; iter < 500; iter++)
        while iter < 500 {
            // C: mjtNum rot[9]; mju_quat2Mat(rot, quat)
            let mut rot: [f64; 9] = [0.0; 9];
            mju_quat2mat(rot.as_mut_ptr(), quat as *const f64);
            // C: mjtNum col1_rot[3] = {rot[0], rot[3], rot[6]}
            let col1_rot: [f64; 3] = [rot[0], rot[3], rot[6]];
            // C: mjtNum col2_rot[3] = {rot[1], rot[4], rot[7]}
            let col2_rot: [f64; 3] = [rot[1], rot[4], rot[7]];
            // C: mjtNum col3_rot[3] = {rot[2], rot[5], rot[8]}
            let col3_rot: [f64; 3] = [rot[2], rot[5], rot[8]];

            // C: mjtNum omega[3], vec1[3], vec2[3], vec3[3]
            let mut omega: [f64; 3] = [0.0; 3];
            let mut vec1: [f64; 3] = [0.0; 3];
            let mut vec2: [f64; 3] = [0.0; 3];
            let mut vec3: [f64; 3] = [0.0; 3];
            // C: mji_cross(vec1, col1_rot, col1_mat)
            mju_cross(vec1.as_mut_ptr(), col1_rot.as_ptr(), col1_mat.as_ptr());
            // C: mji_cross(vec2, col2_rot, col2_mat)
            mju_cross(vec2.as_mut_ptr(), col2_rot.as_ptr(), col2_mat.as_ptr());
            // C: mji_cross(vec3, col3_rot, col3_mat)
            mju_cross(vec3.as_mut_ptr(), col3_rot.as_ptr(), col3_mat.as_ptr());
            // C: mji_add3(omega, vec1, vec2)
            omega[0] = vec1[0] + vec2[0]; // C: omega[0] = vec1[0] + vec2[0]
            omega[1] = vec1[1] + vec2[1]; // C: omega[1] = vec1[1] + vec2[1]
            omega[2] = vec1[2] + vec2[2]; // C: omega[2] = vec1[2] + vec2[2]
            // C: mji_addTo3(omega, vec3)
            omega[0] += vec3[0]; // C: omega[0] += vec3[0]
            omega[1] += vec3[1]; // C: omega[1] += vec3[1]
            omega[2] += vec3[2]; // C: omega[2] += vec3[2]

            // C: mju_scl3(omega, omega, 1.0 / (mju_abs(...) + mjMINVAL))
            let denom: f64 = (crate::engine::engine_util_blas::mju_dot3(col1_rot.as_ptr(), col1_mat.as_ptr())
                + crate::engine::engine_util_blas::mju_dot3(col2_rot.as_ptr(), col2_mat.as_ptr())
                + crate::engine::engine_util_blas::mju_dot3(col3_rot.as_ptr(), col3_mat.as_ptr())).abs() + 1e-15_f64;
            crate::engine::engine_util_blas::mju_scl3(omega.as_mut_ptr(), omega.as_ptr(), 1.0 / denom);

            // C: mjtNum w = mju_normalize3(omega)
            let w: f64 = crate::engine::engine_util_blas::mju_normalize3(omega.as_mut_ptr());
            // C: if (w < rotEPS) { break; }
            if w < ROT_EPS {
                break;
            }
            // C: mjtNum qrot[4]; mji_axisAngle2Quat(qrot, omega, w)
            let mut qrot: [f64; 4] = [0.0; 4];
            mju_axis_angle2quat(qrot.as_mut_ptr(), omega.as_ptr(), w);
            // C: mju_mulQuat(quat, qrot, quat)
            let quat_copy: [f64; 4] = [*quat.add(0), *quat.add(1), *quat.add(2), *quat.add(3)];
            mju_mul_quat(quat, qrot.as_ptr(), quat_copy.as_ptr());
            // C: mju_normalize4(quat)
            crate::engine::engine_util_blas::mju_normalize4(quat);

            iter += 1;
        }
        iter // C: return iter
    }
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
    unsafe {
        // C: mji_mulQuat(quatres, quat1, quat2)
        mju_mul_quat(quatres, quat1, quat2);
        // C: mju_normalize4(quatres)
        crate::engine::engine_util_blas::mju_normalize4(quatres);
        // C: mji_rotVecQuat(posres, pos2, quat1)
        mju_rot_vec_quat(posres, pos2, quat1);
        // C: mji_addTo3(posres, pos1)  — posres[i] += pos1[i]
        *posres.add(0) += *pos1.add(0); // C: posres[0] += pos1[0]
        *posres.add(1) += *pos1.add(1); // C: posres[1] += pos1[1]
        *posres.add(2) += *pos1.add(2); // C: posres[2] += pos1[2]
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
    unsafe {
        // C: mji_negQuat(quatres, quat)
        mju_neg_quat(quatres, quat);
        // C: mji_rotVecQuat(posres, pos, quatres)
        mju_rot_vec_quat(posres, pos, quatres as *const f64);
        // C: mju_scl3(posres, posres, -1)
        crate::engine::engine_util_blas::mju_scl3(posres, posres as *const f64, -1.0);
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
    unsafe {
        // C: mji_rotVecQuat(res, vec, quat)
        mju_rot_vec_quat(res, vec, quat);
        // C: mji_addTo3(res, pos) — res[i] += pos[i]
        *res.add(0) += *pos.add(0); // C: res[0] += pos[0]
        *res.add(1) += *pos.add(1); // C: res[1] += pos[1]
        *res.add(2) += *pos.add(2); // C: res[2] += pos[2]
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
    unsafe {
        // C: if (strnlen(seq, 4) != 3) { mjERROR(...) }
        // Check length: seq must have exactly 3 chars
        let mut len: usize = 0;
        while len < 4 && *seq.add(len) != 0 {
            len += 1;
        }
        if len != 3 {
            crate::engine::engine_util_errmem::mju_error(
                b"seq must contain exactly 3 characters\0".as_ptr() as *const i8,
            );
        }

        // C: mjtNum tmp[4] = {1, 0, 0, 0}
        let mut tmp: [f64; 4] = [1.0, 0.0, 0.0, 0.0];

        // C: for (int i=0; i<3; i++)
        for i in 0..3_usize {
            // C: mjtNum rot[4] = {cos(euler[i]/2), 0, 0, 0}
            let mut rot: [f64; 4] = [(*euler.add(i) / 2.0).cos(), 0.0, 0.0, 0.0];
            // C: mjtNum sa = sin(euler[i]/2)
            let sa: f64 = (*euler.add(i) / 2.0).sin();
            let c: i8 = *seq.add(i); // C: seq[i]

            // C: if (seq[i]=='x' || seq[i]=='X')
            if c == b'x' as i8 || c == b'X' as i8 {
                rot[1] = sa; // C: rot[1] = sa
            // C: else if (seq[i]=='y' || seq[i]=='Y')
            } else if c == b'y' as i8 || c == b'Y' as i8 {
                rot[2] = sa; // C: rot[2] = sa
            // C: else if (seq[i]=='z' || seq[i]=='Z')
            } else if c == b'z' as i8 || c == b'Z' as i8 {
                rot[3] = sa; // C: rot[3] = sa
            } else {
                // C: mjERROR("seq[%d] is '%c', ...")
                crate::engine::engine_util_errmem::mju_error(
                    b"seq element should be one of x, y, z, X, Y, Z\0".as_ptr() as *const i8,
                );
            }

            // C: if (seq[i]=='x' || seq[i]=='y' || seq[i]=='z')
            if c == b'x' as i8 || c == b'y' as i8 || c == b'z' as i8 {
                // C: mju_mulQuat(tmp, tmp, rot) — moving axes: post-multiply
                let tmp_copy: [f64; 4] = [tmp[0], tmp[1], tmp[2], tmp[3]];
                mju_mul_quat(tmp.as_mut_ptr(), tmp_copy.as_ptr(), rot.as_ptr());
            } else {
                // C: mju_mulQuat(tmp, rot, tmp) — fixed axes: pre-multiply
                let tmp_copy: [f64; 4] = [tmp[0], tmp[1], tmp[2], tmp[3]];
                mju_mul_quat(tmp.as_mut_ptr(), rot.as_ptr(), tmp_copy.as_ptr());
            }
        }

        // C: mji_copy4(quat, tmp)
        *quat.add(0) = tmp[0]; // C: quat[0] = tmp[0]
        *quat.add(1) = tmp[1]; // C: quat[1] = tmp[1]
        *quat.add(2) = tmp[2]; // C: quat[2] = tmp[2]
        *quat.add(3) = tmp[3]; // C: quat[3] = tmp[3]
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
    unsafe {
        // C: mjtNum tmp[3] = { a[1]*b[2] - a[2]*b[1], ... }
        let mut tmp: [f64; 3] = [0.0; 3];
        tmp[0] = *a.add(1) * *b.add(2) - *a.add(2) * *b.add(1); // C: a[1]*b[2] - a[2]*b[1]
        tmp[1] = *a.add(2) * *b.add(0) - *a.add(0) * *b.add(2); // C: a[2]*b[0] - a[0]*b[2]
        tmp[2] = *a.add(0) * *b.add(1) - *a.add(1) * *b.add(0); // C: a[0]*b[1] - a[1]*b[0]
        *res.add(0) = tmp[0]; // C: res[0] = tmp[0]
        *res.add(1) = tmp[1]; // C: res[1] = tmp[1]
        *res.add(2) = tmp[2]; // C: res[2] = tmp[2]
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
    unsafe {
        // C: res[0] = -vel[2]*v[1] + vel[1]*v[2]
        *res.add(0) = -*vel.add(2) * *v.add(1) + *vel.add(1) * *v.add(2);
        // C: res[1] = vel[2]*v[0] - vel[0]*v[2]
        *res.add(1) = *vel.add(2) * *v.add(0) - *vel.add(0) * *v.add(2);
        // C: res[2] = -vel[1]*v[0] + vel[0]*v[1]
        *res.add(2) = -*vel.add(1) * *v.add(0) + *vel.add(0) * *v.add(1);
        // C: res[3] = -vel[2]*v[4] + vel[1]*v[5]
        *res.add(3) = -*vel.add(2) * *v.add(4) + *vel.add(1) * *v.add(5);
        // C: res[4] = vel[2]*v[3] - vel[0]*v[5]
        *res.add(4) = *vel.add(2) * *v.add(3) - *vel.add(0) * *v.add(5);
        // C: res[5] = -vel[1]*v[3] + vel[0]*v[4]
        *res.add(5) = -*vel.add(1) * *v.add(3) + *vel.add(0) * *v.add(4);
        // C: res[3] += -vel[5]*v[1] + vel[4]*v[2]
        *res.add(3) += -*vel.add(5) * *v.add(1) + *vel.add(4) * *v.add(2);
        // C: res[4] += vel[5]*v[0] - vel[3]*v[2]
        *res.add(4) += *vel.add(5) * *v.add(0) - *vel.add(3) * *v.add(2);
        // C: res[5] += -vel[4]*v[0] + vel[3]*v[1]
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
    unsafe {
        // C: res[0] = -vel[2]*f[1] + vel[1]*f[2]
        *res.add(0) = -*vel.add(2) * *f.add(1) + *vel.add(1) * *f.add(2);
        // C: res[1] = vel[2]*f[0] - vel[0]*f[2]
        *res.add(1) = *vel.add(2) * *f.add(0) - *vel.add(0) * *f.add(2);
        // C: res[2] = -vel[1]*f[0] + vel[0]*f[1]
        *res.add(2) = -*vel.add(1) * *f.add(0) + *vel.add(0) * *f.add(1);
        // C: res[3] = -vel[2]*f[4] + vel[1]*f[5]
        *res.add(3) = -*vel.add(2) * *f.add(4) + *vel.add(1) * *f.add(5);
        // C: res[4] = vel[2]*f[3] - vel[0]*f[5]
        *res.add(4) = *vel.add(2) * *f.add(3) - *vel.add(0) * *f.add(5);
        // C: res[5] = -vel[1]*f[3] + vel[0]*f[4]
        *res.add(5) = -*vel.add(1) * *f.add(3) + *vel.add(0) * *f.add(4);
        // C: res[0] += -vel[5]*f[4] + vel[4]*f[5]
        *res.add(0) += -*vel.add(5) * *f.add(4) + *vel.add(4) * *f.add(5);
        // C: res[1] += vel[5]*f[3] - vel[3]*f[5]
        *res.add(1) += *vel.add(5) * *f.add(3) - *vel.add(3) * *f.add(5);
        // C: res[2] += -vel[4]*f[3] + vel[3]*f[4]
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
    unsafe {
        // C: mjtNum tmp[9] = {mat[0]*inert[0], mat[3]*inert[0], mat[6]*inert[0], ...}
        let mut tmp: [f64; 9] = [0.0; 9];
        tmp[0] = *mat.add(0) * *inert.add(0); // C: mat[0]*inert[0]
        tmp[1] = *mat.add(3) * *inert.add(0); // C: mat[3]*inert[0]
        tmp[2] = *mat.add(6) * *inert.add(0); // C: mat[6]*inert[0]
        tmp[3] = *mat.add(1) * *inert.add(1); // C: mat[1]*inert[1]
        tmp[4] = *mat.add(4) * *inert.add(1); // C: mat[4]*inert[1]
        tmp[5] = *mat.add(7) * *inert.add(1); // C: mat[7]*inert[1]
        tmp[6] = *mat.add(2) * *inert.add(2); // C: mat[2]*inert[2]
        tmp[7] = *mat.add(5) * *inert.add(2); // C: mat[5]*inert[2]
        tmp[8] = *mat.add(8) * *inert.add(2); // C: mat[8]*inert[2]

        // C: res_rot = mat * diag(inert) * mat'
        *res.add(0) = *mat.add(0) * tmp[0] + *mat.add(1) * tmp[3] + *mat.add(2) * tmp[6]; // C: res[0] = mat[0]*tmp[0] + mat[1]*tmp[3] + mat[2]*tmp[6]
        *res.add(1) = *mat.add(3) * tmp[1] + *mat.add(4) * tmp[4] + *mat.add(5) * tmp[7]; // C: res[1] = mat[3]*tmp[1] + mat[4]*tmp[4] + mat[5]*tmp[7]
        *res.add(2) = *mat.add(6) * tmp[2] + *mat.add(7) * tmp[5] + *mat.add(8) * tmp[8]; // C: res[2] = mat[6]*tmp[2] + mat[7]*tmp[5] + mat[8]*tmp[8]
        *res.add(3) = *mat.add(0) * tmp[1] + *mat.add(1) * tmp[4] + *mat.add(2) * tmp[7]; // C: res[3] = mat[0]*tmp[1] + mat[1]*tmp[4] + mat[2]*tmp[7]
        *res.add(4) = *mat.add(0) * tmp[2] + *mat.add(1) * tmp[5] + *mat.add(2) * tmp[8]; // C: res[4] = mat[0]*tmp[2] + mat[1]*tmp[5] + mat[2]*tmp[8]
        *res.add(5) = *mat.add(3) * tmp[2] + *mat.add(4) * tmp[5] + *mat.add(5) * tmp[8]; // C: res[5] = mat[3]*tmp[2] + mat[4]*tmp[5] + mat[5]*tmp[8]

        // C: res_rot -= mass * dif_cross * dif_cross
        *res.add(0) += mass * (*dif.add(1) * *dif.add(1) + *dif.add(2) * *dif.add(2)); // C: res[0] += mass*(dif[1]*dif[1] + dif[2]*dif[2])
        *res.add(1) += mass * (*dif.add(0) * *dif.add(0) + *dif.add(2) * *dif.add(2)); // C: res[1] += mass*(dif[0]*dif[0] + dif[2]*dif[2])
        *res.add(2) += mass * (*dif.add(0) * *dif.add(0) + *dif.add(1) * *dif.add(1)); // C: res[2] += mass*(dif[0]*dif[0] + dif[1]*dif[1])
        *res.add(3) -= mass * *dif.add(0) * *dif.add(1); // C: res[3] -= mass*dif[0]*dif[1]
        *res.add(4) -= mass * *dif.add(0) * *dif.add(2); // C: res[4] -= mass*dif[0]*dif[2]
        *res.add(5) -= mass * *dif.add(1) * *dif.add(2); // C: res[5] -= mass*dif[1]*dif[2]

        // C: res_tran = mass * dif
        *res.add(6) = mass * *dif.add(0); // C: res[6] = mass*dif[0]
        *res.add(7) = mass * *dif.add(1); // C: res[7] = mass*dif[1]
        *res.add(8) = mass * *dif.add(2); // C: res[8] = mass*dif[2]

        // C: res_mass = mass
        *res.add(9) = mass; // C: res[9] = mass
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
    unsafe {
        // C: if (offset) — hinge
        if !offset.is_null() {
            // C: mji_copy3(res, axis)
            crate::engine::engine_util_blas::mju_copy3(res, axis);
            // C: mji_cross(res+3, axis, offset)
            mju_cross(res.add(3), axis, offset);
        }
        // C: else — slide
        else {
            // C: mju_zero3(res)
            crate::engine::engine_util_blas::mju_zero3(res);
            // C: mji_copy3(res+3, axis)
            crate::engine::engine_util_blas::mju_copy3(res.add(3), axis);
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
    unsafe {
        // C: res[0] = i[0]*v[0] + i[3]*v[1] + i[4]*v[2] - i[8]*v[4] + i[7]*v[5]
        *res.add(0) = *inert.add(0) * *vec.add(0) + *inert.add(3) * *vec.add(1) + *inert.add(4) * *vec.add(2) - *inert.add(8) * *vec.add(4) + *inert.add(7) * *vec.add(5);
        // C: res[1] = i[3]*v[0] + i[1]*v[1] + i[5]*v[2] + i[8]*v[3] - i[6]*v[5]
        *res.add(1) = *inert.add(3) * *vec.add(0) + *inert.add(1) * *vec.add(1) + *inert.add(5) * *vec.add(2) + *inert.add(8) * *vec.add(3) - *inert.add(6) * *vec.add(5);
        // C: res[2] = i[4]*v[0] + i[5]*v[1] + i[2]*v[2] - i[7]*v[3] + i[6]*v[4]
        *res.add(2) = *inert.add(4) * *vec.add(0) + *inert.add(5) * *vec.add(1) + *inert.add(2) * *vec.add(2) - *inert.add(7) * *vec.add(3) + *inert.add(6) * *vec.add(4);
        // C: res[3] = i[8]*v[1] - i[7]*v[2] + i[9]*v[3]
        *res.add(3) = *inert.add(8) * *vec.add(1) - *inert.add(7) * *vec.add(2) + *inert.add(9) * *vec.add(3);
        // C: res[4] = i[6]*v[2] - i[8]*v[0] + i[9]*v[4]
        *res.add(4) = *inert.add(6) * *vec.add(2) - *inert.add(8) * *vec.add(0) + *inert.add(9) * *vec.add(4);
        // C: res[5] = i[7]*v[0] - i[6]*v[1] + i[9]*v[5]
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
    unsafe {
        // C: if (n == 1)
        if n == 1 {
            // C: mju_scl(res, dof, vec[0], 6)
            crate::engine::engine_util_blas::mju_scl(res, mat, *vec.add(0), 6);
        // C: else if (n <= 0)
        } else if n <= 0 {
            // C: mju_zero(res, 6)
            crate::engine::engine_util_blas::mju_zero(res, 6);
        } else {
            // C: mju_mulMatTVec(res, dof, vec, n, 6)
            crate::engine::engine_util_blas::mju_mul_mat_t_vec(res, mat, vec, n, 6);
        }
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
    unsafe {
        // C: mjtNum cros[3], dif[3], tran[6]
        let mut cros: [f64; 3] = [0.0; 3];
        let mut dif: [f64; 3] = [0.0; 3];
        let mut tran: [f64; 6] = [0.0; 6];

        // C: mju_copy(tran, vec, 6)
        crate::engine::engine_util_blas::mju_copy(tran.as_mut_ptr(), vec, 6);
        // C: mji_sub3(dif, newpos, oldpos)
        crate::engine::engine_util_blas::mju_sub3(dif.as_mut_ptr(), newpos, oldpos);

        // C: if (flg_force)
        if flg_force != 0 {
            // C: mji_cross(cros, dif, vec+3)
            mju_cross(cros.as_mut_ptr(), dif.as_ptr(), vec.add(3));
            // C: mji_sub3(tran, vec, cros)
            crate::engine::engine_util_blas::mju_sub3(tran.as_mut_ptr(), vec, cros.as_ptr());
        } else {
            // C: mji_cross(cros, dif, vec)
            mju_cross(cros.as_mut_ptr(), dif.as_ptr(), vec);
            // C: mji_sub3(tran+3, vec+3, cros)
            crate::engine::engine_util_blas::mju_sub3(tran.as_mut_ptr().add(3), vec.add(3), cros.as_ptr());
        }

        // C: if (rotnew2old)
        if !rotnew2old.is_null() {
            // C: mji_mulMatTVec3(res, rotnew2old, tran)
            crate::engine::engine_util_blas::mju_mul_mat_t_vec3(res, rotnew2old, tran.as_ptr());
            // C: mji_mulMatTVec3(res+3, rotnew2old, tran+3)
            crate::engine::engine_util_blas::mju_mul_mat_t_vec3(res.add(3), rotnew2old, tran.as_ptr().add(3));
        }
        // C: else { mji_copy6(res, tran); }
        else {
            *res.add(0) = tran[0]; // C: res[0] = tran[0]
            *res.add(1) = tran[1]; // C: res[1] = tran[1]
            *res.add(2) = tran[2]; // C: res[2] = tran[2]
            *res.add(3) = tran[3]; // C: res[3] = tran[3]
            *res.add(4) = tran[4]; // C: res[4] = tran[4]
            *res.add(5) = tran[5]; // C: res[5] = tran[5]
        }
    }
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
    unsafe {
        // C: mjtNum tmp[3]
        let mut tmp: [f64; 3] = [0.0; 3];

        // C: if (mju_normalize3(frame) < 0.5) { mjERROR(...) }
        if crate::engine::engine_util_blas::mju_normalize3(frame) < 0.5 {
            crate::engine::engine_util_errmem::mju_error(
                b"xaxis of contact frame undefined\0".as_ptr() as *const i8,
            );
        }

        // C: if (mju_dot3(frame+3, frame+3) < 0.25)
        if crate::engine::engine_util_blas::mju_dot3(frame.add(3) as *const f64, frame.add(3) as *const f64) < 0.25 {
            // C: mju_zero3(frame+3)
            crate::engine::engine_util_blas::mju_zero3(frame.add(3));

            // C: if (frame[1] < 0.5 && frame[1] > -0.5)
            if *frame.add(1) < 0.5 && *frame.add(1) > -0.5 {
                *frame.add(4) = 1.0; // C: frame[4] = 1
            } else {
                *frame.add(5) = 1.0; // C: frame[5] = 1
            }
        }

        // C: mji_scl3(tmp, frame, mju_dot3(frame, frame+3))
        let dot: f64 = crate::engine::engine_util_blas::mju_dot3(frame as *const f64, frame.add(3) as *const f64);
        crate::engine::engine_util_blas::mju_scl3(tmp.as_mut_ptr(), frame as *const f64, dot);
        // C: mji_subFrom3(frame+3, tmp) — frame[3+i] -= tmp[i]
        *frame.add(3) -= tmp[0]; // C: frame[3] -= tmp[0]
        *frame.add(4) -= tmp[1]; // C: frame[4] -= tmp[1]
        *frame.add(5) -= tmp[2]; // C: frame[5] -= tmp[2]
        // C: mju_normalize3(frame+3)
        crate::engine::engine_util_blas::mju_normalize3(frame.add(3));

        // C: mji_cross(frame+6, frame, frame+3)
        mju_cross(frame.add(6), frame as *const f64, frame.add(3) as *const f64);
    }
}

