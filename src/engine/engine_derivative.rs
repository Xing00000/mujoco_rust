//! Port of: engine/engine_derivative.c
//! IR hash: 05737965add36adb
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mjd_cross (engine/engine_derivative.c:38)
/// Calls: mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjd_cross(a: *const f64, b: *const f64, Da: *mut f64, Db: *mut f64) {
    // SAFETY: a[3], b[3] are read-only inputs; Da[9], Db[9] are caller-allocated outputs.
    //         Null check matches C: if pointer is non-null, write to it.
    unsafe {
        // derivative w.r.t a
        if !Da.is_null() {
            crate::engine::engine_util_blas::mju_zero(Da, 9);
            *Da.add(1) =  *b.add(2);
            *Da.add(2) = -*b.add(1);
            *Da.add(3) = -*b.add(2);
            *Da.add(5) =  *b.add(0);
            *Da.add(6) =  *b.add(1);
            *Da.add(7) = -*b.add(0);
        }

        // derivative w.r.t b
        if !Db.is_null() {
            crate::engine::engine_util_blas::mju_zero(Db, 9);
            *Db.add(1) = -*a.add(2);
            *Db.add(2) =  *a.add(1);
            *Db.add(3) =  *a.add(2);
            *Db.add(5) = -*a.add(0);
            *Db.add(6) = -*a.add(1);
            *Db.add(7) =  *a.add(0);
        }
    }
}

/// C: mjd_crossMotion_vel (engine/engine_derivative.c:65)
/// Calls: mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjd_cross_motion_vel(D: *mut f64, v: *const f64) {
    // SAFETY: D[36] is caller-allocated output; v[6] is read-only input.
    unsafe {
        crate::engine::engine_util_blas::mju_zero(D, 36);

        // res[0] = -vel[2]*v[1] + vel[1]*v[2]
        *D.add(0 + 2) = -*v.add(1);
        *D.add(0 + 1) = *v.add(2);

        // res[1] =  vel[2]*v[0] - vel[0]*v[2]
        *D.add(6 + 2) = *v.add(0);
        *D.add(6 + 0) = -*v.add(2);

        // res[2] = -vel[1]*v[0] + vel[0]*v[1]
        *D.add(12 + 1) = -*v.add(0);
        *D.add(12 + 0) = *v.add(1);

        // res[3] = -vel[2]*v[4] + vel[1]*v[5] - vel[5]*v[1] + vel[4]*v[2]
        *D.add(18 + 2) = -*v.add(4);
        *D.add(18 + 1) = *v.add(5);
        *D.add(18 + 5) = -*v.add(1);
        *D.add(18 + 4) = *v.add(2);

        // res[4] =  vel[2]*v[3] - vel[0]*v[5] + vel[5]*v[0] - vel[3]*v[2]
        *D.add(24 + 2) = *v.add(3);
        *D.add(24 + 0) = -*v.add(5);
        *D.add(24 + 5) = *v.add(0);
        *D.add(24 + 3) = -*v.add(2);

        // res[5] = -vel[1]*v[3] + vel[0]*v[4] - vel[4]*v[0] + vel[3]*v[1]
        *D.add(30 + 1) = -*v.add(3);
        *D.add(30 + 0) = *v.add(4);
        *D.add(30 + 4) = -*v.add(0);
        *D.add(30 + 3) = *v.add(1);
    }
}

/// C: mjd_crossForce_vel (engine/engine_derivative.c:101)
/// Calls: mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjd_cross_force_vel(D: *mut f64, f: *const f64) {
    // SAFETY: D[36] is caller-allocated output; f[6] is read-only input.
    unsafe {
        crate::engine::engine_util_blas::mju_zero(D, 36);

        // res[0] = -vel[2]*f[1] + vel[1]*f[2] - vel[5]*f[4] + vel[4]*f[5]
        *D.add(0 + 2) = -*f.add(1);
        *D.add(0 + 1) = *f.add(2);
        *D.add(0 + 5) = -*f.add(4);
        *D.add(0 + 4) = *f.add(5);

        // res[1] =  vel[2]*f[0] - vel[0]*f[2] + vel[5]*f[3] - vel[3]*f[5]
        *D.add(6 + 2) = *f.add(0);
        *D.add(6 + 0) = -*f.add(2);
        *D.add(6 + 5) = *f.add(3);
        *D.add(6 + 3) = -*f.add(5);

        // res[2] = -vel[1]*f[0] + vel[0]*f[1] - vel[4]*f[3] + vel[3]*f[4]
        *D.add(12 + 1) = -*f.add(0);
        *D.add(12 + 0) = *f.add(1);
        *D.add(12 + 4) = -*f.add(3);
        *D.add(12 + 3) = *f.add(4);

        // res[3] = -vel[2]*f[4] + vel[1]*f[5]
        *D.add(18 + 2) = -*f.add(4);
        *D.add(18 + 1) = *f.add(5);

        // res[4] =  vel[2]*f[3] - vel[0]*f[5]
        *D.add(24 + 2) = *f.add(3);
        *D.add(24 + 0) = -*f.add(5);

        // res[5] = -vel[1]*f[3] + vel[0]*f[4]
        *D.add(30 + 1) = -*f.add(3);
        *D.add(30 + 0) = *f.add(4);
    }
}

/// C: mjd_crossForce_frc (engine/engine_derivative.c:137)
/// Calls: mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjd_cross_force_frc(D: *mut f64, vel: *const f64) {
    // SAFETY: D[36] is caller-allocated output; vel[6] is read-only input.
    unsafe {
        crate::engine::engine_util_blas::mju_zero(D, 36);

        // res[0] = -vel[2]*f[1] + vel[1]*f[2] - vel[5]*f[4] + vel[4]*f[5]
        *D.add(0 + 1) = -*vel.add(2);
        *D.add(0 + 2) = *vel.add(1);
        *D.add(0 + 4) = -*vel.add(5);
        *D.add(0 + 5) = *vel.add(4);

        // res[1] =  vel[2]*f[0] - vel[0]*f[2] + vel[5]*f[3] - vel[3]*f[5]
        *D.add(6 + 0) = *vel.add(2);
        *D.add(6 + 2) = -*vel.add(0);
        *D.add(6 + 3) = *vel.add(5);
        *D.add(6 + 5) = -*vel.add(3);

        // res[2] = -vel[1]*f[0] + vel[0]*f[1] - vel[4]*f[3] + vel[3]*f[4]
        *D.add(12 + 0) = -*vel.add(1);
        *D.add(12 + 1) = *vel.add(0);
        *D.add(12 + 3) = -*vel.add(4);
        *D.add(12 + 4) = *vel.add(3);

        // res[3] = -vel[2]*f[4] + vel[1]*f[5]
        *D.add(18 + 4) = -*vel.add(2);
        *D.add(18 + 5) = *vel.add(1);

        // res[4] =  vel[2]*f[3] - vel[0]*f[5]
        *D.add(24 + 3) = *vel.add(2);
        *D.add(24 + 5) = -*vel.add(0);

        // res[5] = -vel[1]*f[3] + vel[0]*f[4]
        *D.add(30 + 3) = -*vel.add(1);
        *D.add(30 + 4) = *vel.add(0);
    }
}

/// C: mjd_mulInertVec_vel (engine/engine_derivative.c:173)
/// Calls: mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjd_mul_inert_vec_vel(D: *mut f64, i: *const f64) {
    // SAFETY: D[36] is caller-allocated output; i[10] is read-only input.
    unsafe {
        crate::engine::engine_util_blas::mju_zero(D, 36);

        // res[0] = i[0]*v[0] + i[3]*v[1] + i[4]*v[2] - i[8]*v[4] + i[7]*v[5]
        *D.add(0 + 0) = *i.add(0);
        *D.add(0 + 1) = *i.add(3);
        *D.add(0 + 2) = *i.add(4);
        *D.add(0 + 4) = -*i.add(8);
        *D.add(0 + 5) = *i.add(7);

        // res[1] = i[3]*v[0] + i[1]*v[1] + i[5]*v[2] + i[8]*v[3] - i[6]*v[5]
        *D.add(6 + 0) = *i.add(3);
        *D.add(6 + 1) = *i.add(1);
        *D.add(6 + 2) = *i.add(5);
        *D.add(6 + 3) = *i.add(8);
        *D.add(6 + 5) = -*i.add(6);

        // res[2] = i[4]*v[0] + i[5]*v[1] + i[2]*v[2] - i[7]*v[3] + i[6]*v[4]
        *D.add(12 + 0) = *i.add(4);
        *D.add(12 + 1) = *i.add(5);
        *D.add(12 + 2) = *i.add(2);
        *D.add(12 + 3) = -*i.add(7);
        *D.add(12 + 4) = *i.add(6);

        // res[3] = i[8]*v[1] - i[7]*v[2] + i[9]*v[3]
        *D.add(18 + 1) = *i.add(8);
        *D.add(18 + 2) = -*i.add(7);
        *D.add(18 + 3) = *i.add(9);

        // res[4] = i[6]*v[2] - i[8]*v[0] + i[9]*v[4]
        *D.add(24 + 2) = *i.add(6);
        *D.add(24 + 0) = -*i.add(8);
        *D.add(24 + 4) = *i.add(9);

        // res[5] = i[7]*v[0] - i[6]*v[1] + i[9]*v[5]
        *D.add(30 + 0) = *i.add(7);
        *D.add(30 + 1) = -*i.add(6);
        *D.add(30 + 5) = *i.add(9);
    }
}

/// C: mjd_comVel_vel_dense (engine/engine_derivative.c:321)
/// Calls: mjd_crossMotion_vel, mju_copy, mju_mulMatMat, mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjd_com_vel_vel_dense(m: *const mjModel, d: *mut mjData, Dcvel: *mut f64, Dcdofdot: *mut f64) {
    extern "C" { fn mjd_comVel_vel_dense_impl(m: *const mjModel, d: *mut mjData, Dcvel: *mut f64, Dcdofdot: *mut f64); }
    // SAFETY: delegates to C implementation, pointers valid per caller
    unsafe { mjd_comVel_vel_dense_impl(m, d, Dcvel, Dcdofdot) }
}

/// C: copyFromParent (engine/engine_derivative.c:468)
/// Calls: mju_copy
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn copy_from_parent(m: *const mjModel, d: *mut mjData, mat: *mut f64, n: i32) {
    extern "C" { fn copyFromParent_impl(m: *const mjModel, d: *mut mjData, mat: *mut f64, n: i32); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { copyFromParent_impl(m, d, mat, n) }
}

/// C: addToParent (engine/engine_derivative.c:491)
/// Calls: mju_addTo, mju_message
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn add_to_parent(m: *const mjModel, d: *mut mjData, mat: *mut f64, n: i32) {
    extern "C" { fn addToParent_impl(m: *const mjModel, d: *mut mjData, mat: *mut f64, n: i32); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { addToParent_impl(m, d, mat, n) }
}

/// C: mjd_comVel_vel (engine/engine_derivative.c:524)
/// Calls: copyFromParent, mjd_crossMotion_vel, mju_addTo, mju_message, mju_mulMatMat, mju_transpose
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjd_com_vel_vel(m: *const mjModel, d: *mut mjData, Dcvel: *mut f64, Dcdofdot: *mut f64) {
    extern "C" { fn mjd_comVel_vel_impl(m: *const mjModel, d: *mut mjData, Dcvel: *mut f64, Dcdofdot: *mut f64); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mjd_comVel_vel_impl(m, d, Dcvel, Dcdofdot) }
}

/// C: mjd_rne_vel (engine/engine_derivative.c:596)
/// Calls: addToParent, copyFromParent, mj_freeStack, mj_markStack, mj_stackAllocInfo, mjd_comVel_vel, mjd_crossForce_frc, mjd_crossForce_vel, mjd_mulInertVec_vel, mju_addTo, mju_addToScl, mju_mulInertVec, mju_mulMatMat, mju_mulMatVec, mju_subFrom, mju_transpose, mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mjd_rne_vel(m: *const mjModel, d: *mut mjData) {
    extern "C" { fn mjd_rne_vel_impl(m: *const mjModel, d: *mut mjData); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mjd_rne_vel_impl(m, d) }
}

/// C: addJTBJ (engine/engine_derivative.c:711)
/// Calls: mj_freeStack, mj_markStack, mj_stackAllocInfo, mju_scl
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn add_jtbj(m: *const mjModel, d: *mut mjData, J: *const f64, B: *const f64, n: i32) {
    extern "C" { fn addJTBJ_impl(m: *const mjModel, d: *mut mjData, J: *const f64, B: *const f64, n: i32); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { addJTBJ_impl(m, d, J, B, n) }
}

/// C: addJTBJSparse (engine/engine_derivative.c:746)
/// Calls: mju_addToSclSparseInc
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn add_jtbj_sparse(m: *const mjModel, d: *mut mjData, J: *const f64, B: *const f64, n: i32, offset: i32, J_rownnz: *const i32, J_rowadr: *const i32, J_colind: *const i32) {
    extern "C" { fn addJTBJSparse_impl(m: *const mjModel, d: *mut mjData, J: *const f64, B: *const f64, n: i32, offset: i32, J_rownnz: *const i32, J_rowadr: *const i32, J_colind: *const i32); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { addJTBJSparse_impl(m, d, J, B, n, offset, J_rownnz, J_rowadr, J_colind) }
}

/// C: mjd_muscleGain_vel (engine/engine_derivative.c:781)
/// Calls: mju_max, mju_muscleGainLength
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjd_muscle_gain_vel(len: f64, vel: f64, lengthrange: *const f64, acc0: f64, prm: *const f64) -> f64 {
    const mjMINVAL: f64 = 1e-15;
    // SAFETY: lengthrange[2] and prm[9] are read-only inputs, valid per caller contract.
    unsafe {
        // unpack parameters
        let range0 = *prm.add(0);
        let range1 = *prm.add(1);
        let mut force = *prm.add(2);
        let scale = *prm.add(3);
        let lmin = *prm.add(4);
        let lmax = *prm.add(5);
        let vmax = *prm.add(6);
        let fvmax = *prm.add(8);

        // scale force if negative
        if force < 0.0 {
            force = scale / crate::engine::engine_util_misc::mju_max(mjMINVAL, acc0);
        }

        // optimum length
        let L0 = (*lengthrange.add(1) - *lengthrange.add(0))
            / crate::engine::engine_util_misc::mju_max(mjMINVAL, range1 - range0);

        // normalized length and velocity
        let L = range0 + (len - *lengthrange.add(0))
            / crate::engine::engine_util_misc::mju_max(mjMINVAL, L0);
        let V = vel / crate::engine::engine_util_misc::mju_max(mjMINVAL, L0 * vmax);

        // length curve
        let FL = crate::engine::engine_util_misc::mju_muscle_gain_length(L, lmin, lmax);

        // velocity curve
        let dFV: f64;
        let y = fvmax - 1.0;
        if V <= -1.0 {
            // FV = 0
            dFV = 0.0;
        } else if V <= 0.0 {
            // FV = (V+1)*(V+1)
            dFV = 2.0 * V + 2.0;
        } else if V <= y {
            // FV = fvmax - (y-V)*(y-V) / mju_max(mjMINVAL, y)
            dFV = (-2.0 * V + 2.0 * y) / crate::engine::engine_util_misc::mju_max(mjMINVAL, y);
        } else {
            // FV = fvmax
            dFV = 0.0;
        }

        // compute FVL and scale, make it negative
        -force * FL * dFV / crate::engine::engine_util_misc::mju_max(mjMINVAL, L0 * vmax)
    }
}

/// C: addJTBJ_mulSparse (engine/engine_derivative.c:832)
/// Calls: mj_freeStack, mj_markStack, mj_stackAllocInfo, mju_mulMatVec, mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn add_jtbj_mul_sparse(m: *const mjModel, d: *mut mjData, res: *mut f64, vec: *const f64, J_rownnz: *const i32, J_rowadr: *const i32, J_colind: *const i32, J: *const f64, B: *const f64, n: i32) {
    extern "C" { fn addJTBJ_mulSparse_impl(m: *const mjModel, d: *mut mjData, res: *mut f64, vec: *const f64, J_rownnz: *const i32, J_rowadr: *const i32, J_colind: *const i32, J: *const f64, B: *const f64, n: i32); }
    // SAFETY: delegates to C implementation
    unsafe { addJTBJ_mulSparse_impl(m, d, res, vec, J_rownnz, J_rowadr, J_colind, J, B, n) }
}

/// C: mjd_flexInterp_kernel (engine/engine_derivative.c:872)
/// Calls: addJTBJ_mulSparse, mj_bodyChain, mj_freeStack, mj_jacSparse, mj_markStack, mj_stackAllocInfo, mju_flexGatherCellState, mju_flexGatherFaceState, mju_flexGatherState, mju_mulMatMat3, mju_quat2Mat, mju_transpose, mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjd_flex_interp_kernel(m: *const mjModel, d: *mut mjData, res: *mut f64, vec: *const f64, s1: f64, s2: f64, K_rot_cache: *const f64, K_rot_out: *mut f64) {
    extern "C" { fn mjd_flexInterp_kernel_impl(m: *const mjModel, d: *mut mjData, res: *mut f64, vec: *const f64, s1: f64, s2: f64, K_rot_cache: *const f64, K_rot_out: *mut f64); }
    // SAFETY: delegates to C implementation
    unsafe { mjd_flexInterp_kernel_impl(m, d, res, vec, s1, s2, K_rot_cache, K_rot_out) }
}

/// C: pow2 (engine/engine_derivative.c:1339)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn pow2(val: f64) -> f64 {
    val * val
}

/// C: ellipsoid_max_moment (engine/engine_derivative.c:1344)
/// Calls: mju_max, pow2
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn ellipsoid_max_moment(size: *const f64, dir: i32) -> f64 {
    const mjPI: f64 = std::f64::consts::PI;
    // SAFETY: size[3] is read-only input, valid per caller contract.
    unsafe {
        let d0 = *size.add(dir as usize);
        let d1 = *size.add(((dir + 1) % 3) as usize);
        let d2 = *size.add(((dir + 2) % 3) as usize);
        8.0 / 15.0 * mjPI * d0 * pow2(pow2(crate::engine::engine_util_misc::mju_max(d1, d2)))
    }
}

/// C: addToQuadrant (engine/engine_derivative.c:1354)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn add_to_quadrant(B: *mut f64, D: *const f64, col_quad: i32, row_quad: i32) {
    // SAFETY: B[36] (6x6 matrix) is caller-allocated; D[9] is read-only input.
    //         col_quad and row_quad are 0 or 1.
    unsafe {
        let r = 3 * row_quad;
        let c = 3 * col_quad;
        *B.add((6 * (c + 0) + r + 0) as usize) += *D.add(0);
        *B.add((6 * (c + 0) + r + 1) as usize) += *D.add(1);
        *B.add((6 * (c + 0) + r + 2) as usize) += *D.add(2);
        *B.add((6 * (c + 1) + r + 0) as usize) += *D.add(3);
        *B.add((6 * (c + 1) + r + 1) as usize) += *D.add(4);
        *B.add((6 * (c + 1) + r + 2) as usize) += *D.add(5);
        *B.add((6 * (c + 2) + r + 0) as usize) += *D.add(6);
        *B.add((6 * (c + 2) + r + 1) as usize) += *D.add(7);
        *B.add((6 * (c + 2) + r + 2) as usize) += *D.add(8);
    }
}

/// C: mjd_addedMassForces (engine/engine_derivative.c:1371)
/// Calls: addToQuadrant, mjd_cross
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjd_added_mass_forces(B: *mut f64, local_vels: *const f64, fluid_density: f64, virtual_mass: *const f64, virtual_inertia: *const f64) {
    // SAFETY: B[36] is caller-allocated output (6x6, accumulated into);
    //         local_vels[6], virtual_mass[3], virtual_inertia[3] are read-only inputs.
    unsafe {
        let lin_vel: [f64; 3] = [*local_vels.add(3), *local_vels.add(4), *local_vels.add(5)];
        let ang_vel: [f64; 3] = [*local_vels.add(0), *local_vels.add(1), *local_vels.add(2)];
        let virtual_lin_mom: [f64; 3] = [
            fluid_density * *virtual_mass.add(0) * lin_vel[0],
            fluid_density * *virtual_mass.add(1) * lin_vel[1],
            fluid_density * *virtual_mass.add(2) * lin_vel[2],
        ];
        let virtual_ang_mom: [f64; 3] = [
            fluid_density * *virtual_inertia.add(0) * ang_vel[0],
            fluid_density * *virtual_inertia.add(1) * ang_vel[1],
            fluid_density * *virtual_inertia.add(2) * ang_vel[2],
        ];
        let mut Da: [f64; 9] = [0.0; 9];
        let mut Db: [f64; 9] = [0.0; 9];

        // force[:3] += cross(virtual_ang_mom, ang_vel)
        mjd_cross(virtual_ang_mom.as_ptr(), ang_vel.as_ptr(), Da.as_mut_ptr(), Db.as_mut_ptr());
        add_to_quadrant(B, Db.as_ptr(), 0, 0);
        for idx in 0..9 {
            Da[idx] *= fluid_density * *virtual_inertia.add(idx % 3);
        }
        add_to_quadrant(B, Da.as_ptr(), 0, 0);

        // force[:3] += cross(virtual_lin_mom, lin_vel)
        mjd_cross(virtual_lin_mom.as_ptr(), lin_vel.as_ptr(), Da.as_mut_ptr(), Db.as_mut_ptr());
        add_to_quadrant(B, Db.as_ptr(), 0, 1);
        for idx in 0..9 {
            Da[idx] *= fluid_density * *virtual_mass.add(idx % 3);
        }
        add_to_quadrant(B, Da.as_ptr(), 0, 1);

        // force[3:] += cross(virtual_lin_mom, ang_vel)
        mjd_cross(virtual_lin_mom.as_ptr(), ang_vel.as_ptr(), Da.as_mut_ptr(), Db.as_mut_ptr());
        add_to_quadrant(B, Db.as_ptr(), 1, 0);
        for idx in 0..9 {
            Da[idx] *= fluid_density * *virtual_mass.add(idx % 3);
        }
        add_to_quadrant(B, Da.as_ptr(), 1, 1);
    }
}

/// C: mjd_viscous_torque (engine/engine_derivative.c:1416)
/// Calls: ellipsoid_max_moment, mju_addToScl3, mju_max, mju_min, mju_norm3, mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjd_viscous_torque(D: *mut f64, lvel: *const f64, fluid_density: f64, fluid_viscosity: f64, size: *const f64, slender_drag_coef: f64, ang_drag_coef: f64) {
    const mjPI: f64 = std::f64::consts::PI;
    const mjMINVAL: f64 = 1e-15;
    // SAFETY: D[9] is caller-allocated output; lvel[6], size[3] are read-only inputs.
    unsafe {
        let d_max = crate::engine::engine_util_misc::mju_max(
            crate::engine::engine_util_misc::mju_max(*size.add(0), *size.add(1)),
            *size.add(2),
        );
        let d_min = crate::engine::engine_util_misc::mju_min(
            crate::engine::engine_util_misc::mju_min(*size.add(0), *size.add(1)),
            *size.add(2),
        );
        let d_mid = *size.add(0) + *size.add(1) + *size.add(2) - d_max - d_min;

        // viscous force and torque in Stokes flow, analytical for spherical bodies
        let eq_sphere_D = 2.0 / 3.0 * (*size.add(0) + *size.add(1) + *size.add(2));
        let lin_visc_torq_coef = mjPI * eq_sphere_D * eq_sphere_D * eq_sphere_D;

        // moments of inertia used to compute angular quadratic drag
        let I_max = 8.0 / 15.0 * mjPI * d_mid * (d_max * d_max) * (d_max * d_max);
        let II: [f64; 3] = [
            ellipsoid_max_moment(size, 0),
            ellipsoid_max_moment(size, 1),
            ellipsoid_max_moment(size, 2),
        ];
        let x = *lvel.add(0);
        let y = *lvel.add(1);
        let z = *lvel.add(2);
        let mom_coef: [f64; 3] = [
            ang_drag_coef * II[0] + slender_drag_coef * (I_max - II[0]),
            ang_drag_coef * II[1] + slender_drag_coef * (I_max - II[1]),
            ang_drag_coef * II[2] + slender_drag_coef * (I_max - II[2]),
        ];
        let mom_visc: [f64; 3] = [
            x * mom_coef[0],
            y * mom_coef[1],
            z * mom_coef[2],
        ];
        let density = fluid_density
            / crate::engine::engine_util_misc::mju_max(
                mjMINVAL,
                crate::engine::engine_util_blas::mju_norm3(mom_visc.as_ptr()),
            );

        // -density * [x, y, z] * mom_coef^2
        let mom_sq: [f64; 3] = [
            -density * x * mom_coef[0] * mom_coef[0],
            -density * y * mom_coef[1] * mom_coef[1],
            -density * z * mom_coef[2] * mom_coef[2],
        ];
        let lin_coef = fluid_viscosity * lin_visc_torq_coef;

        // initialize
        crate::engine::engine_util_blas::mju_zero(D, 9);

        // set diagonal
        let diag_val = x * mom_sq[0] + y * mom_sq[1] + z * mom_sq[2] - lin_coef;
        *D.add(0) = diag_val;
        *D.add(4) = diag_val;
        *D.add(8) = diag_val;

        // add outer product
        crate::engine::engine_util_blas::mju_add_to_scl3(D, mom_sq.as_ptr(), x);
        crate::engine::engine_util_blas::mju_add_to_scl3(D.add(3), mom_sq.as_ptr(), y);
        crate::engine::engine_util_blas::mju_add_to_scl3(D.add(6), mom_sq.as_ptr(), z);
    }
}

/// C: mjd_viscous_drag (engine/engine_derivative.c:1469)
/// Calls: mju_addToScl3, mju_max, mju_min, mju_scl, pow2
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjd_viscous_drag(D: *mut f64, lvel: *const f64, fluid_density: f64, fluid_viscosity: f64, size: *const f64, blunt_drag_coef: f64, slender_drag_coef: f64) {
    const mjPI: f64 = std::f64::consts::PI;
    const mjMINVAL: f64 = 1e-15;
    // SAFETY: D[9] is caller-allocated output; lvel[6], size[3] are read-only inputs.
    unsafe {
        let d_max = crate::engine::engine_util_misc::mju_max(
            crate::engine::engine_util_misc::mju_max(*size.add(0), *size.add(1)),
            *size.add(2),
        );
        let d_min = crate::engine::engine_util_misc::mju_min(
            crate::engine::engine_util_misc::mju_min(*size.add(0), *size.add(1)),
            *size.add(2),
        );
        let d_mid = *size.add(0) + *size.add(1) + *size.add(2) - d_max - d_min;

        // viscous force and torque in Stokes flow, analytical for spherical bodies
        let eq_sphere_D = 2.0 / 3.0 * (*size.add(0) + *size.add(1) + *size.add(2));
        let A_max = mjPI * d_max * d_mid;

        let a = pow2(*size.add(1) * *size.add(2));
        let b = pow2(*size.add(2) * *size.add(0));
        let c = pow2(*size.add(0) * *size.add(1));
        let aa = a * a;
        let bb = b * b;
        let cc = c * c;

        let x = *lvel.add(3);
        let y = *lvel.add(4);
        let z = *lvel.add(5);
        let xx = x * x;
        let yy = y * y;
        let zz = z * z;
        let xy = x * y;
        let yz = y * z;
        let xz = x * z;

        let proj_denom = aa * xx + bb * yy + cc * zz;
        let proj_num = a * xx + b * yy + c * zz;
        let dA_coef = mjPI
            / crate::engine::engine_util_misc::mju_max(
                mjMINVAL,
                (proj_num * proj_num * proj_num * proj_denom).sqrt(),
            );

        let A_proj = mjPI
            * (proj_denom / crate::engine::engine_util_misc::mju_max(mjMINVAL, proj_num)).sqrt();

        let norm = (xx + yy + zz).sqrt();
        let inv_norm = 1.0 / crate::engine::engine_util_misc::mju_max(mjMINVAL, norm);

        let lin_coef = fluid_viscosity * 3.0 * mjPI * eq_sphere_D;
        let quad_coef = fluid_density
            * (A_proj * blunt_drag_coef + slender_drag_coef * (A_max - A_proj));
        let Aproj_coef = fluid_density * norm * (blunt_drag_coef - slender_drag_coef);

        let dAproj_dv: [f64; 3] = [
            Aproj_coef * dA_coef * a * x * (b * yy * (a - b) + c * zz * (a - c)),
            Aproj_coef * dA_coef * b * y * (a * xx * (b - a) + c * zz * (b - c)),
            Aproj_coef * dA_coef * c * z * (a * xx * (c - a) + b * yy * (c - b)),
        ];

        // outer product
        *D.add(0) = xx;
        *D.add(1) = xy;
        *D.add(2) = xz;
        *D.add(3) = xy;
        *D.add(4) = yy;
        *D.add(5) = yz;
        *D.add(6) = xz;
        *D.add(7) = yz;
        *D.add(8) = zz;

        // diag(D) += dot([x y z], [x y z])
        let inner = xx + yy + zz;
        *D.add(0) += inner;
        *D.add(4) += inner;
        *D.add(8) += inner;

        // scale by -quad_coef*inv_norm
        crate::engine::engine_util_blas::mju_scl(D, D as *const f64, -quad_coef * inv_norm, 9);

        // D += outer_product(-[x y z], dAproj_dv)
        crate::engine::engine_util_blas::mju_add_to_scl3(D.add(0), dAproj_dv.as_ptr(), -x);
        crate::engine::engine_util_blas::mju_add_to_scl3(D.add(3), dAproj_dv.as_ptr(), -y);
        crate::engine::engine_util_blas::mju_add_to_scl3(D.add(6), dAproj_dv.as_ptr(), -z);

        // diag(D) -= lin_coef
        *D.add(0) -= lin_coef;
        *D.add(4) -= lin_coef;
        *D.add(8) -= lin_coef;
    }
}

/// C: mjd_kutta_lift (engine/engine_derivative.c:1536)
/// Calls: mju_addToScl3, mju_max, mju_scl, pow2
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjd_kutta_lift(D: *mut f64, lvel: *const f64, fluid_density: f64, size: *const f64, kutta_lift_coef: f64) {
    const mjPI: f64 = std::f64::consts::PI;
    const mjMINVAL: f64 = 1e-15;
    // SAFETY: D[9] is caller-allocated output; lvel[6], size[3] are read-only inputs.
    unsafe {
        let a = pow2(*size.add(1) * *size.add(2));
        let b = pow2(*size.add(2) * *size.add(0));
        let c = pow2(*size.add(0) * *size.add(1));
        let aa = a * a;
        let bb = b * b;
        let cc = c * c;
        let x = *lvel.add(3);
        let y = *lvel.add(4);
        let z = *lvel.add(5);
        let xx = x * x;
        let yy = y * y;
        let zz = z * z;
        let xy = x * y;
        let yz = y * z;
        let xz = x * z;

        let proj_denom = aa * xx + bb * yy + cc * zz;
        let proj_num = a * xx + b * yy + c * zz;
        let norm2 = xx + yy + zz;
        let df_denom = mjPI * kutta_lift_coef * fluid_density
            / crate::engine::engine_util_misc::mju_max(
                mjMINVAL,
                (proj_denom * proj_num * norm2).sqrt(),
            );

        let dfx_coef = yy * (a - b) + zz * (a - c);
        let dfy_coef = xx * (b - a) + zz * (b - c);
        let dfz_coef = xx * (c - a) + yy * (c - b);
        let proj_term = proj_num
            / crate::engine::engine_util_misc::mju_max(mjMINVAL, proj_denom);
        let cos_term = proj_num
            / crate::engine::engine_util_misc::mju_max(mjMINVAL, norm2);

        // D[i,j] = 2*proj_num * (col_coef - row_coef) where col/row from {a,b,c}
        *D.add(0) = a - a;
        *D.add(1) = b - a;
        *D.add(2) = c - a;
        *D.add(3) = a - b;
        *D.add(4) = b - b;
        *D.add(5) = c - b;
        *D.add(6) = a - c;
        *D.add(7) = b - c;
        *D.add(8) = c - c;
        crate::engine::engine_util_blas::mju_scl(D, D as *const f64, 2.0 * proj_num, 9);

        let inner_term: [f64; 3] = [
            aa * proj_term - a + cos_term,
            bb * proj_term - b + cos_term,
            cc * proj_term - c + cos_term,
        ];
        crate::engine::engine_util_blas::mju_add_to_scl3(D.add(0), inner_term.as_ptr(), dfx_coef);
        crate::engine::engine_util_blas::mju_add_to_scl3(D.add(3), inner_term.as_ptr(), dfy_coef);
        crate::engine::engine_util_blas::mju_add_to_scl3(D.add(6), inner_term.as_ptr(), dfz_coef);

        *D.add(0) *= xx;
        *D.add(1) *= xy;
        *D.add(2) *= xz;
        *D.add(3) *= xy;
        *D.add(4) *= yy;
        *D.add(5) *= yz;
        *D.add(6) *= xz;
        *D.add(7) *= yz;
        *D.add(8) *= zz;

        *D.add(0) -= dfx_coef * proj_num;
        *D.add(4) -= dfy_coef * proj_num;
        *D.add(8) -= dfz_coef * proj_num;

        crate::engine::engine_util_blas::mju_scl(D, D as *const f64, df_denom, 9);
    }
}

/// C: mjd_magnus_force (engine/engine_derivative.c:1589)
/// Calls: addToQuadrant, mjd_cross
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjd_magnus_force(B: *mut f64, lvel: *const f64, fluid_density: f64, size: *const f64, magnus_lift_coef: f64) {
    const mjPI: f64 = std::f64::consts::PI;
    // SAFETY: B[36] (6x6) is caller-allocated output (accumulated into);
    //         lvel[6], size[3] are read-only inputs.
    unsafe {
        let volume = 4.0 / 3.0 * mjPI * *size.add(0) * *size.add(1) * *size.add(2);

        // magnus_coef = magnus_lift_coef * fluid_density * volume
        let magnus_coef = magnus_lift_coef * fluid_density * volume;

        let mut D_lin: [f64; 9] = [0.0; 9];
        let mut D_ang: [f64; 9] = [0.0; 9];

        // premultiply by magnus_coef
        let lin_vel: [f64; 3] = [
            magnus_coef * *lvel.add(3),
            magnus_coef * *lvel.add(4),
            magnus_coef * *lvel.add(5),
        ];
        let ang_vel: [f64; 3] = [
            magnus_coef * *lvel.add(0),
            magnus_coef * *lvel.add(1),
            magnus_coef * *lvel.add(2),
        ];

        // force[3:] += magnus_coef * cross(ang_vel, lin_vel)
        mjd_cross(ang_vel.as_ptr(), lin_vel.as_ptr(), D_ang.as_mut_ptr(), D_lin.as_mut_ptr());

        add_to_quadrant(B, D_ang.as_ptr(), 1, 0);
        add_to_quadrant(B, D_lin.as_ptr(), 1, 1);
    }
}

/// C: mjd_ellipsoidFluid (engine/engine_derivative.c:1618)
/// Calls: addJTBJ, addJTBJSparse, addToQuadrant, mj_bodyChain, mj_freeStack, mj_isSparse, mj_jacGeom, mj_jacSparse, mj_markStack, mj_objectVelocity, mj_stackAllocInfo, mjd_addedMassForces, mjd_kutta_lift, mjd_magnus_force, mjd_viscous_drag, mjd_viscous_torque, mju_copy, mju_copy3, mju_geomSemiAxes, mju_mulMatTMat, mju_subFrom3, mju_symmetrize, mju_transformSpatial, mju_zero, readFluidGeomInteraction
#[allow(unused_variables, non_snake_case)]
pub fn mjd_ellipsoid_fluid(m: *const mjModel, d: *mut mjData, bodyid: i32) {
    extern "C" { fn mjd_ellipsoidFluid_impl(m: *const mjModel, d: *mut mjData, bodyid: i32); }
    // SAFETY: delegates to C implementation, pointers valid per caller
    unsafe { mjd_ellipsoidFluid_impl(m, d, bodyid) }
}

/// C: mjd_inertiaBoxFluid (engine/engine_derivative.c:1724)
/// Calls: addJTBJ, addJTBJSparse, mj_bodyChain, mj_freeStack, mj_isSparse, mj_jacBodyCom, mj_jacSparse, mj_markStack, mj_objectVelocity, mj_stackAllocInfo, mju_copy, mju_copy3, mju_max, mju_mulMatTMat, mju_subFrom3, mju_transformSpatial, mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mjd_inertia_box_fluid(m: *const mjModel, d: *mut mjData, i: i32) {
    extern "C" { fn mjd_inertiaBoxFluid_impl(m: *const mjModel, d: *mut mjData, i: i32); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mjd_inertiaBoxFluid_impl(m, d, i) }
}

/// C: mjd_subQuat (engine/engine_derivative.h:27)
/// Calls: mju_addToScl, mju_copy9, mju_mulMatMat3, mju_normalize3, mju_scl, mju_subQuat, mju_transpose
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case, unused_unsafe)]
pub fn mjd_sub_quat(qa: *const f64, qb: *const f64, Da: *mut f64, Db: *mut f64) {
    // SAFETY: raw pointer arithmetic on caller-guaranteed valid buffers.
    // qa, qb: 4 f64; Da, Db: 9 f64 (or null)
    unsafe {
        // no outputs, quick return
        if Da.is_null() && Db.is_null() {
            return;
        }

        // compute axis-angle quaternion difference
        let mut axis: [f64; 3] = [0.0; 3];
        crate::engine::engine_util_spatial::mju_sub_quat(axis.as_mut_ptr(), qa, qb);

        // normalize axis, get half-angle
        let half_angle: f64 = 0.5 * crate::engine::engine_util_blas::mju_normalize3(axis.as_mut_ptr());

        // identity
        let mut Da_tmp: [f64; 9] = [
            1.0, 0.0, 0.0,
            0.0, 1.0, 0.0,
            0.0, 0.0, 1.0,
        ];

        // add term linear in cross product matrix K
        let K: [f64; 9] = [
            0.0, -axis[2], axis[1],
            axis[2], 0.0, -axis[0],
            -axis[1], axis[0], 0.0,
        ];
        crate::engine::engine_util_blas::mju_add_to_scl(Da_tmp.as_mut_ptr(), K.as_ptr(), half_angle, 9);

        // add term linear in K * K
        let mut KK: [f64; 9] = [0.0; 9];
        crate::engine::engine_util_blas::mju_mul_mat_mat3(KK.as_mut_ptr(), K.as_ptr(), K.as_ptr());
        let coef: f64 = 1.0 - (if half_angle < 6e-8 { 1.0 } else { half_angle / half_angle.tan() });
        crate::engine::engine_util_blas::mju_add_to_scl(Da_tmp.as_mut_ptr(), KK.as_ptr(), coef, 9);

        if !Da.is_null() {
            crate::engine::engine_util_blas::mju_copy9(Da, Da_tmp.as_ptr());
        }

        if !Db.is_null() {
            // Db = -Da^T
            crate::engine::engine_util_blas::mju_transpose(Db, Da_tmp.as_ptr(), 3, 3);
            crate::engine::engine_util_blas::mju_scl(Db, Db, -1.0, 9);
        }
    }
}

/// C: mjd_quatIntegrate (engine/engine_derivative.h:30)
/// Calls: mju_copy9, mju_dot3, mju_mulMatVec3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjd_quat_integrate(vel: *const f64, scale: f64, Dquat: *mut f64, Dvel: *mut f64, Dscale: *mut f64) {
    // SAFETY: raw pointer arithmetic on caller-guaranteed valid buffers.
    // vel: 3 f64; Dquat, Dvel: 9 f64 (or null); Dscale: 3 f64 (or null)
    unsafe {
        // scaled velocity
        let s: [f64; 3] = [scale * *vel.add(0), scale * *vel.add(1), scale * *vel.add(2)];

        // 3 basis matrices
        let eye: [f64; 9] = [
            1.0, 0.0, 0.0,
            0.0, 1.0, 0.0,
            0.0, 0.0, 1.0,
        ];
        let cross: [f64; 9] = [
             0.0,    s[2], -s[1],
            -s[2],   0.0,   s[0],
             s[1],  -s[0],  0.0,
        ];
        let outer: [f64; 9] = [
            s[0]*s[0], s[0]*s[1], s[0]*s[2],
            s[1]*s[0], s[1]*s[1], s[1]*s[2],
            s[2]*s[0], s[2]*s[1], s[2]*s[2],
        ];

        // squared norm, norm of s
        let xx: f64 = crate::engine::engine_util_blas::mju_dot3(s.as_ptr(), s.as_ptr());
        let x: f64 = xx.sqrt();

        // 4 coefficients: a=cos(x), b=sin(x)/x, c=(1-cos(x))/x^2, d=(x-sin(x))/x^3
        let a: f64 = x.cos();
        let b: f64;
        let c: f64;
        let d: f64;

        // x is not small: use full expressions
        if x.abs() > 1.0 / 32.0 {
            b = x.sin() / x;
            c = (1.0 - a) / xx;
            d = (1.0 - b) / xx;
        }
        // |x| <= 1/32: use 6th order Taylor expansion (Horner form)
        else {
            b = 1.0 + xx / 6.0 * (xx / 20.0 * (1.0 - xx / 42.0) - 1.0);
            c = (1.0 + xx / 12.0 * (xx / 30.0 * (1.0 - xx / 56.0) - 1.0)) / 2.0;
            d = (1.0 + xx / 20.0 * (xx / 42.0 * (1.0 - xx / 72.0) - 1.0)) / 6.0;
        }

        // derivatives
        let mut Dvel_: [f64; 9] = [0.0; 9];
        for i in 0..9 {
            if !Dquat.is_null() {
                *Dquat.add(i) = a * eye[i] + b * cross[i] + c * outer[i];
            }
            if !Dvel.is_null() || !Dscale.is_null() {
                Dvel_[i] = b * eye[i] + c * cross[i] + d * outer[i];
            }
        }
        if !Dvel.is_null() {
            crate::engine::engine_util_blas::mju_copy9(Dvel, Dvel_.as_ptr());
        }
        if !Dscale.is_null() {
            crate::engine::engine_util_blas::mju_mul_mat_vec3(Dscale, Dvel_.as_ptr(), vel);
        }
    }
}

/// C: mjd_smooth_vel (engine/engine_derivative.h:35)
/// Calls: mjd_actuator_vel, mjd_passive_vel, mjd_rne_vel, mju_zero, mju_zeroSparse
#[allow(unused_variables, non_snake_case)]
pub fn mjd_smooth_vel(m: *const mjModel, d: *mut mjData, flg_bias: i32) {
    extern "C" { fn mjd_smooth_vel_impl(m: *const mjModel, d: *mut mjData, flg_bias: i32); }
    // SAFETY: delegates to C implementation, pointers valid per caller
    unsafe { mjd_smooth_vel_impl(m, d, flg_bias) }
}

/// C: mjd_actuator_vel (engine/engine_derivative.h:38)
/// Calls: addJTBJSparse, mj_actuatorDisabled, mj_nextActivation, mj_sleepState, mjd_muscleGain_vel, mju_max
#[allow(unused_variables, non_snake_case)]
pub fn mjd_actuator_vel(m: *const mjModel, d: *mut mjData) {
    extern "C" { fn mjd_actuator_vel_impl(m: *const mjModel, d: *mut mjData); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mjd_actuator_vel_impl(m, d) }
}

/// C: mjd_passive_vel (engine/engine_derivative.h:41)
/// Calls: addJTBJSparse, mj_actuatorDamping, mjd_ellipsoidFluid, mjd_inertiaBoxFluid, mjd_xPolyForce, mju_copy
#[allow(unused_variables, non_snake_case)]
pub fn mjd_passive_vel(m: *const mjModel, d: *mut mjData) {
    extern "C" { fn mjd_passive_vel_impl(m: *const mjModel, d: *mut mjData); }
    // SAFETY: delegates to C implementation, pointers valid per caller
    unsafe { mjd_passive_vel_impl(m, d) }
}

/// C: mjd_rne_vel_dense (engine/engine_derivative.h:44)
/// Calls: mj_freeStack, mj_markStack, mj_stackAllocInfo, mjd_comVel_vel_dense, mjd_crossForce_frc, mjd_crossForce_vel, mjd_mulInertVec_vel, mju_addTo, mju_addToScl, mju_copy, mju_mulInertVec, mju_mulMatMat, mju_scl, mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mjd_rne_vel_dense(m: *const mjModel, d: *mut mjData) {
    extern "C" { fn mjd_rne_vel_dense_impl(m: *const mjModel, d: *mut mjData); }
    // SAFETY: delegates to C implementation, pointers valid per caller
    unsafe { mjd_rne_vel_dense_impl(m, d) }
}

/// C: mjd_flexInterp_mul (engine/engine_derivative.h:48)
/// Calls: mjd_flexInterp_kernel
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjd_flex_interp_mul(m: *const mjModel, d: *mut mjData, res: *mut f64, vec: *const f64, s1: f64, s2: f64, K_rot_cache: *const f64) {
    // SAFETY: delegates to mjd_flex_interp_kernel with K_rot_out=NULL.
    unsafe {
        mjd_flex_interp_kernel(m, d, res, vec, s1, s2, K_rot_cache, core::ptr::null_mut());
    }
}

/// C: mjd_flexInterp_cacheKrot (engine/engine_derivative.h:52)
/// Calls: mjd_flexInterp_kernel
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjd_flex_interp_cache_krot(m: *const mjModel, d: *mut mjData, K_rot_out: *mut f64) {
    // SAFETY: uses s1=1, s2=0 so scale=1 and K_rot_out gets unscaled values.
    unsafe {
        mjd_flex_interp_kernel(m, d, core::ptr::null_mut(), core::ptr::null(), 1.0, 0.0, core::ptr::null(), K_rot_out);
    }
}

/// C: mjd_flexBend_mul (engine/engine_derivative.h:56)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjd_flex_bend_mul(m: *const mjModel, d: *mut mjData, res: *mut f64, vec: *const f64, s1: f64, s2: f64) {
    // SAFETY: m, d valid. res/vec have nv elements. All flex model arrays valid.
    unsafe {
        let mut f: i32 = 0;
        while f < (*m).nflex as i32 {
            // skip interp, rigid, or non-2D
            if *(*m).flex_interp.add(f as usize) != 0
                || *((*m).flex_rigid as *const u8).add(f as usize) != 0
                || *(*m).flex_dim.add(f as usize) != 2 {
                f += 1;
                continue;
            }

            let bendingadr = *(*m).flex_bendingadr.add(f as usize);
            if bendingadr < 0 {
                f += 1;
                continue;
            }

            let scale = s1 + s2 * *(*m).flex_damping.add(f as usize);
            if scale == 0.0 {
                f += 1;
                continue;
            }

            let b = (*m).flex_bending.add(bendingadr as usize);
            let bodyid = (*m).flex_vertbodyid.add(*(*m).flex_vertadr.add(f as usize) as usize);
            let edgenum = *(*m).flex_edgenum.add(f as usize);
            let edgeadr = *(*m).flex_edgeadr.add(f as usize);

            let mut e: i32 = 0;
            while e < edgenum {
                let edge = (*m).flex_edge.add(2 * (e + edgeadr) as usize);
                let flap = (*m).flex_edgeflap.add(2 * (e + edgeadr) as usize);
                let v: [i32; 4] = [*edge.add(0), *edge.add(1), *flap.add(0), *flap.add(1)];

                // skip boundary edges (no second flap vertex)
                if v[3] == -1 {
                    e += 1;
                    continue;
                }

                // apply 4x4 bending stencil, coordinate-wise
                let mut i: i32 = 0;
                while i < 4 {
                    let dof_i = *(*m).body_dofadr.add(*bodyid.add(v[i as usize] as usize) as usize);
                    let mut x: i32 = 0;
                    while x < 3 {
                        let mut val: f64 = 0.0;
                        let mut j: i32 = 0;
                        while j < 4 {
                            let dof_j = *(*m).body_dofadr.add(*bodyid.add(v[j as usize] as usize) as usize);
                            val += *b.add((17 * e + 4 * i + j) as usize) * *vec.add((dof_j + x) as usize);
                            j += 1;
                        }
                        *res.add((dof_i + x) as usize) += scale * val;
                        x += 1;
                    }
                    i += 1;
                }
                e += 1;
            }
            f += 1;
        }
    }
}

