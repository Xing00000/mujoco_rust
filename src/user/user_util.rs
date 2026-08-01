//! Port of: user/user_util.cc
//! IR hash: 9343293228317031
//! CODEGEN: source paths, owners, and callable names are locked.

use crate::types::*;

/// C: mjuu_offcenter (user/user_util.cc:531)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_offcenter(res: *mut f64, mass: f64, vec: *const f64) {
    // SAFETY: res[6], vec[3] valid (caller contract)
    unsafe {
        *res.add(0) = mass * (*vec.add(1) * *vec.add(1) + *vec.add(2) * *vec.add(2));
        *res.add(1) = mass * (*vec.add(0) * *vec.add(0) + *vec.add(2) * *vec.add(2));
        *res.add(2) = mass * (*vec.add(0) * *vec.add(0) + *vec.add(1) * *vec.add(1));
        *res.add(3) = -mass * *vec.add(0) * *vec.add(1);
        *res.add(4) = -mass * *vec.add(0) * *vec.add(2);
        *res.add(5) = -mass * *vec.add(1) * *vec.add(2);
    }
}

/// C: mjuu_axisAngle2Quat (user/user_util.cc:564)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_axisAngle2Quat(res: *mut f64, axis: *const f64, angle: f64) {
    // SAFETY: res[4], axis[3] valid (caller contract)
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

/// C: mjuu_defined (user/user_util.h:35)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_defined(num: f64) -> bool {
    !num.is_nan()
}

/// C: mjuu_matadr (user/user_util.h:39)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_matadr(g1: i32, g2: i32, n: i32) -> i32 {
    if g1 < 0 || g2 < 0 || g1 >= n || g2 >= n {
        return -1;
    }

    let (g1, g2) = if g1 > g2 { (g2, g1) } else { (g1, g2) };
    g1 * n + g2
}

/// C: mjuu_addtovec (user/user_util.h:59)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_addtovec(dest: *mut f64, src: *const f64, n: i32) {
    // SAFETY: caller guarantees dest and src point to at least n f64
    unsafe {
        let mut i: i32 = 0;
        while i < n {
            *dest.offset(i as isize) += *src.offset(i as isize);
            i += 1;
        }
    }
}

/// C: mjuu_dot3 (user/user_util.h:68)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_dot3(a: *const f64, b: *const f64) -> f64 {
    // SAFETY: a and b point to at least 3 f64 (caller contract)
    unsafe {
        *a.add(0) * *b.add(0) + *a.add(1) * *b.add(1) + *a.add(2) * *b.add(2)
    }
}

/// C: mjuu_dist3 (user/user_util.h:71)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_dist3(a: *const f64, b: *const f64) -> f64 {
    // SAFETY: caller guarantees a and b point to at least 3 f64
    unsafe {
        f64::sqrt(
            (*a.add(0) - *b.add(0)) * (*a.add(0) - *b.add(0))
          + (*a.add(1) - *b.add(1)) * (*a.add(1) - *b.add(1))
          + (*a.add(2) - *b.add(2)) * (*a.add(2) - *b.add(2))
        )
    }
}

/// C: mjuu_L1 (user/user_util.h:74)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_L1(a: *const f64, b: *const f64, n: i32) -> f64 {
    // SAFETY: caller guarantees a and b point to at least n f64
    unsafe {
        let mut res: f64 = 0.0;
        let mut i: i32 = 0;
        while i < n {
            res += (*a.offset(i as isize) - *b.offset(i as isize)).abs();
            i += 1;
        }
        res
    }
}

/// C: mjuu_scalevec (user/user_util.h:82)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_scalevec(res: *mut f64, vec: *const f64, s: f64, n: i32) {
    // SAFETY: res and vec are valid arrays of length >= n from caller
    unsafe {
        for i in 0..n as usize {
            *res.add(i) = s * *vec.add(i);
        }
    }
}

/// C: mjuu_quat2mat (user/user_util.h:85)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_quat2mat(res: *mut f64, quat: *const f64) {
    // SAFETY: res is 9-element array, quat is 4-element array from caller
    unsafe {
        // identity quat: identity mat
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
            return;
        }

        // regular processing
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

/// C: mjuu_mulvecmat (user/user_util.h:91)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_mulvecmat(res: *mut f64, vec: *const f64, mat: *const f64) {
    // SAFETY: res is 3-element, vec is 3-element, mat is 9-element (3x3 row-major)
    unsafe {
        let tmp0 = *mat.add(0) * *vec.add(0) + *mat.add(1) * *vec.add(1) + *mat.add(2) * *vec.add(2);
        let tmp1 = *mat.add(3) * *vec.add(0) + *mat.add(4) * *vec.add(1) + *mat.add(5) * *vec.add(2);
        let tmp2 = *mat.add(6) * *vec.add(0) + *mat.add(7) * *vec.add(1) + *mat.add(8) * *vec.add(2);
        *res.add(0) = tmp0;
        *res.add(1) = tmp1;
        *res.add(2) = tmp2;
    }
}

/// C: mjuu_mulvecmatT (user/user_util.h:94)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_mulvecmatT(res: *mut f64, vec: *const f64, mat: *const f64) {
    // SAFETY: res[3], vec[3], mat[9] valid (caller contract)
    unsafe {
        let tmp0 = *mat.add(0) * *vec.add(0) + *mat.add(3) * *vec.add(1) + *mat.add(6) * *vec.add(2);
        let tmp1 = *mat.add(1) * *vec.add(0) + *mat.add(4) * *vec.add(1) + *mat.add(7) * *vec.add(2);
        let tmp2 = *mat.add(2) * *vec.add(0) + *mat.add(5) * *vec.add(1) + *mat.add(8) * *vec.add(2);
        *res.add(0) = tmp0;
        *res.add(1) = tmp1;
        *res.add(2) = tmp2;
    }
}

/// C: mjuu_mulRMRT (user/user_util.h:97)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_mulRMRT(res: *mut f64, R: *const f64, M: *const f64) {
    // SAFETY: res[9], R[9], M[9] valid (caller contract)
    unsafe {
        // tmp = R*M
        let mut tmp: [f64; 9] = [0.0; 9];
        tmp[0] = *R.add(0) * *M.add(0) + *R.add(1) * *M.add(3) + *R.add(2) * *M.add(6);
        tmp[1] = *R.add(0) * *M.add(1) + *R.add(1) * *M.add(4) + *R.add(2) * *M.add(7);
        tmp[2] = *R.add(0) * *M.add(2) + *R.add(1) * *M.add(5) + *R.add(2) * *M.add(8);
        tmp[3] = *R.add(3) * *M.add(0) + *R.add(4) * *M.add(3) + *R.add(5) * *M.add(6);
        tmp[4] = *R.add(3) * *M.add(1) + *R.add(4) * *M.add(4) + *R.add(5) * *M.add(7);
        tmp[5] = *R.add(3) * *M.add(2) + *R.add(4) * *M.add(5) + *R.add(5) * *M.add(8);
        tmp[6] = *R.add(6) * *M.add(0) + *R.add(7) * *M.add(3) + *R.add(8) * *M.add(6);
        tmp[7] = *R.add(6) * *M.add(1) + *R.add(7) * *M.add(4) + *R.add(8) * *M.add(7);
        tmp[8] = *R.add(6) * *M.add(2) + *R.add(7) * *M.add(5) + *R.add(8) * *M.add(8);

        // res = tmp*R'
        *res.add(0) = tmp[0] * *R.add(0) + tmp[1] * *R.add(1) + tmp[2] * *R.add(2);
        *res.add(1) = tmp[0] * *R.add(3) + tmp[1] * *R.add(4) + tmp[2] * *R.add(5);
        *res.add(2) = tmp[0] * *R.add(6) + tmp[1] * *R.add(7) + tmp[2] * *R.add(8);
        *res.add(3) = tmp[3] * *R.add(0) + tmp[4] * *R.add(1) + tmp[5] * *R.add(2);
        *res.add(4) = tmp[3] * *R.add(3) + tmp[4] * *R.add(4) + tmp[5] * *R.add(5);
        *res.add(5) = tmp[3] * *R.add(6) + tmp[4] * *R.add(7) + tmp[5] * *R.add(8);
        *res.add(6) = tmp[6] * *R.add(0) + tmp[7] * *R.add(1) + tmp[8] * *R.add(2);
        *res.add(7) = tmp[6] * *R.add(3) + tmp[7] * *R.add(4) + tmp[8] * *R.add(5);
        *res.add(8) = tmp[6] * *R.add(6) + tmp[7] * *R.add(7) + tmp[8] * *R.add(8);
    }
}

/// C: mjuu_localaxis (user/user_util.h:106)
/// Calls: cxx:__Z13mjuu_quat2matPdPKd, cxx:__Z14mjuu_mulvecmatPdPKdS1_
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_localaxis(al: *mut f64, ag: *const f64, quat: *const f64) {
    // SAFETY: al[3], ag[3], quat[4] valid (caller contract)
    unsafe {
        let mut mat: [f64; 9] = [0.0; 9];
        let qneg: [f64; 4] = [*quat.add(0), -*quat.add(1), -*quat.add(2), -*quat.add(3)];
        mjuu_quat2mat(mat.as_mut_ptr(), qneg.as_ptr());
        mjuu_mulvecmat(al, ag, mat.as_ptr());
    }
}

/// C: mjuu_localpos (user/user_util.h:109)
/// Calls: cxx:__Z14mjuu_localaxisPdPKdS1_
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_localpos(pl: *mut f64, pg: *const f64, pos: *const f64, quat: *const f64) {
    // SAFETY: pl[3], pg[3], pos[3], quat[4] valid (caller contract)
    unsafe {
        let a: [f64; 3] = [
            *pg.add(0) - *pos.add(0),
            *pg.add(1) - *pos.add(1),
            *pg.add(2) - *pos.add(2),
        ];
        mjuu_localaxis(pl, a.as_ptr(), quat);
    }
}

/// C: mjuu_crossvec (user/user_util.h:115)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_crossvec(a: *mut f64, b: *const f64, c: *const f64) {
    // SAFETY: a, b, c are valid 3-element arrays from caller
    unsafe {
        *a.add(0) = *b.add(1) * *c.add(2) - *b.add(2) * *c.add(1);
        *a.add(1) = *b.add(2) * *c.add(0) - *b.add(0) * *c.add(2);
        *a.add(2) = *b.add(0) * *c.add(1) - *b.add(1) * *c.add(0);
    }
}

/// C: mjuu_frameinvert (user/user_util.h:128)
/// Calls: cxx:__Z14mjuu_localaxisPdPKdS1_
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_frameinvert(newpos: *mut f64, newquat: *mut f64, oldpos: *const f64, oldquat: *const f64) {
    // SAFETY: newpos[3], newquat[4], oldpos[3], oldquat[4] valid (caller contract)
    unsafe {
        // position
        mjuu_localaxis(newpos, oldpos, oldquat);
        *newpos.add(0) = -*newpos.add(0);
        *newpos.add(1) = -*newpos.add(1);
        *newpos.add(2) = -*newpos.add(2);

        // orientation
        *newquat.add(0) = *oldquat.add(0);
        *newquat.add(1) = -*oldquat.add(1);
        *newquat.add(2) = -*oldquat.add(2);
        *newquat.add(3) = -*oldquat.add(3);
    }
}

/// C: mjuu_globalinertia (user/user_util.h:144)
/// Calls: cxx:__Z13mjuu_quat2matPdPKd
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_globalinertia(global: *mut f64, local: *const f64, quat: *const f64) {
    // SAFETY: global[6], local[3], quat[4] valid (caller contract)
    unsafe {
        let mut mat: [f64; 9] = [0.0; 9];
        mjuu_quat2mat(mat.as_mut_ptr(), quat);
        let tmp: [f64; 9] = [
            mat[0] * *local.add(0), mat[3] * *local.add(0), mat[6] * *local.add(0),
            mat[1] * *local.add(1), mat[4] * *local.add(1), mat[7] * *local.add(1),
            mat[2] * *local.add(2), mat[5] * *local.add(2), mat[8] * *local.add(2),
        ];
        *global.add(0) = mat[0] * tmp[0] + mat[1] * tmp[3] + mat[2] * tmp[6];
        *global.add(1) = mat[3] * tmp[1] + mat[4] * tmp[4] + mat[5] * tmp[7];
        *global.add(2) = mat[6] * tmp[2] + mat[7] * tmp[5] + mat[8] * tmp[8];
        *global.add(3) = mat[0] * tmp[1] + mat[1] * tmp[4] + mat[2] * tmp[7];
        *global.add(4) = mat[0] * tmp[2] + mat[1] * tmp[5] + mat[2] * tmp[8];
        *global.add(5) = mat[3] * tmp[2] + mat[4] * tmp[5] + mat[5] * tmp[8];
    }
}

/// C: mjuu_visccoef (user/user_util.h:150)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_visccoef(visccoef: *mut f64, mass: f64, inertia: *const f64, scl: f64) {
    // SAFETY: visccoef[6], inertia[3] valid (caller contract). mass > 0 assumed.
    const MJ_EPS: f64 = 1e-14;
    unsafe {
        // compute equivalent box
        let e0 = (((*inertia.add(1) + *inertia.add(2) - *inertia.add(0)) / mass * 6.0).max(MJ_EPS)).sqrt();
        let e1 = (((*inertia.add(0) + *inertia.add(2) - *inertia.add(1)) / mass * 6.0).max(MJ_EPS)).sqrt();
        let e2 = (((*inertia.add(0) + *inertia.add(1) - *inertia.add(2)) / mass * 6.0).max(MJ_EPS)).sqrt();
        // torque components
        *visccoef.add(0) = scl * 4.0 / 3.0 * e0 * (e1*e1*e1 + e2*e2*e2);
        *visccoef.add(1) = scl * 4.0 / 3.0 * e1 * (e0*e0*e0 + e2*e2*e2);
        *visccoef.add(2) = scl * 4.0 / 3.0 * e2 * (e0*e0*e0 + e1*e1*e1);
        // force components
        *visccoef.add(3) = scl * 4.0 * e1 * e2;
        *visccoef.add(4) = scl * 4.0 * e0 * e2;
        *visccoef.add(5) = scl * 4.0 * e0 * e1;
    }
}

/// C: mjuu_eigendecompose (user/user_util.h:166)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_eigendecompose(mat: *mut f64, eigval: *mut f64, eigvec: *mut f64, n: i32) -> i32 {
    // Jacobi eigendecomposition of a symmetric matrix.
    // SAFETY: mat[n*n], eigval[n], eigvec[n*n] are valid pointers (caller contract).
    unsafe {
        let n = n as usize;

        // initialize eigvec to identity
        for k in 0..(n * n) {
            *eigvec.add(k) = 0.0;
        }
        for i in 0..n {
            *eigvec.add(i * n + i) = 1.0;
        }

        let max_sweeps: i32 = 200;
        let tol: f64 = 1e-12;

        let mut sweep: i32 = 0;
        while sweep < max_sweeps {
            // check convergence: sum of squared off-diagonal elements
            let mut off_diag: f64 = 0.0;
            for i in 0..n {
                for j in (i + 1)..n {
                    let v = *mat.add(i * n + j);
                    off_diag += v * v;
                }
            }
            if off_diag < tol * tol { break; }

            // sweep over all off-diagonal pairs
            for p in 0..n {
                for q in (p + 1)..n {
                    let apq = *mat.add(p * n + q);
                    if apq.abs() < tol * 1e-3 { continue; }

                    let app = *mat.add(p * n + p);
                    let aqq = *mat.add(q * n + q);
                    let tau = (aqq - app) / (2.0 * apq);
                    let t = (if tau >= 0.0 { 1.0 } else { -1.0 })
                        / (tau.abs() + (1.0 + tau * tau).sqrt());
                    let c = 1.0 / (1.0 + t * t).sqrt();
                    let s = t * c;

                    // update matrix (Jacobi rotation)
                    *mat.add(p * n + p) -= t * apq;
                    *mat.add(q * n + q) += t * apq;
                    *mat.add(p * n + q) = 0.0;
                    *mat.add(q * n + p) = 0.0;

                    for r in 0..n {
                        if r == p || r == q { continue; }
                        let mrp = *mat.add(r * n + p);
                        let mrq = *mat.add(r * n + q);
                        let new_rp = c * mrp - s * mrq;
                        let new_rq = s * mrp + c * mrq;
                        *mat.add(r * n + p) = new_rp;
                        *mat.add(p * n + r) = new_rp;
                        *mat.add(r * n + q) = new_rq;
                        *mat.add(q * n + r) = new_rq;
                    }

                    // accumulate eigenvectors
                    for r in 0..n {
                        let vrp = *eigvec.add(r * n + p);
                        let vrq = *eigvec.add(r * n + q);
                        *eigvec.add(r * n + p) = c * vrp - s * vrq;
                        *eigvec.add(r * n + q) = s * vrp + c * vrq;
                    }
                }
            }
            sweep += 1;
        }

        // extract eigenvalues from diagonal
        for i in 0..n {
            *eigval.add(i) = *mat.add(i * n + i);
        }

        sweep
    }
}

/// C: mjuu_dirnamelen (user/user_util.h:299)
#[allow(unused_variables, non_snake_case)]
pub fn mjuu_dirnamelen(path: *const i8) -> i32 {
    if path.is_null() {
        return 0;
    }
    // SAFETY: path is a valid null-terminated C string (caller contract)
    unsafe {
        let mut pos: i32 = -1;
        let mut i: i32 = 0;
        while *path.offset(i as isize) != 0 {
            if *path.offset(i as isize) == b'/' as i8 || *path.offset(i as isize) == b'\\' as i8 {
                pos = i;
            }
            i += 1;
        }
        pos + 1
    }
}

