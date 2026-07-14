//! Port of: engine/engine_derivative.c
//! IR hash: 8cbd078414266fa8
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
    // SAFETY: caller guarantees a, b point to [3] arrays; Da, Db (if non-null) point to [9] arrays
    unsafe {
        if !Da.is_null() {
            crate::engine::engine_util_blas::mju_zero(Da, 9);
            *Da.add(1) = *b.add(2);
            *Da.add(2) = -*b.add(1);
            *Da.add(3) = -*b.add(2);
            *Da.add(5) = *b.add(0);
            *Da.add(6) = *b.add(1);
            *Da.add(7) = -*b.add(0);
        }
        if !Db.is_null() {
            crate::engine::engine_util_blas::mju_zero(Db, 9);
            *Db.add(1) = -*a.add(2);
            *Db.add(2) = *a.add(1);
            *Db.add(3) = *a.add(2);
            *Db.add(5) = -*a.add(0);
            *Db.add(6) = -*a.add(1);
            *Db.add(7) = *a.add(0);
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
    // SAFETY: caller guarantees D points to [36] array, v points to [6] array
    unsafe {
        crate::engine::engine_util_blas::mju_zero(D, 36);

        *D.add(0 + 2) = -*v.add(1);
        *D.add(0 + 1) = *v.add(2);

        *D.add(6 + 2) = *v.add(0);
        *D.add(6 + 0) = -*v.add(2);

        *D.add(12 + 1) = -*v.add(0);
        *D.add(12 + 0) = *v.add(1);

        *D.add(18 + 2) = -*v.add(4);
        *D.add(18 + 1) = *v.add(5);
        *D.add(18 + 5) = -*v.add(1);
        *D.add(18 + 4) = *v.add(2);

        *D.add(24 + 2) = *v.add(3);
        *D.add(24 + 0) = -*v.add(5);
        *D.add(24 + 5) = *v.add(0);
        *D.add(24 + 3) = -*v.add(2);

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
    // SAFETY: caller guarantees D points to [36] array, f points to [6] array
    unsafe {
        crate::engine::engine_util_blas::mju_zero(D, 36);

        *D.add(0 + 2) = -*f.add(1);
        *D.add(0 + 1) = *f.add(2);
        *D.add(0 + 5) = -*f.add(4);
        *D.add(0 + 4) = *f.add(5);

        *D.add(6 + 2) = *f.add(0);
        *D.add(6 + 0) = -*f.add(2);
        *D.add(6 + 5) = *f.add(3);
        *D.add(6 + 3) = -*f.add(5);

        *D.add(12 + 1) = -*f.add(0);
        *D.add(12 + 0) = *f.add(1);
        *D.add(12 + 4) = -*f.add(3);
        *D.add(12 + 3) = *f.add(4);

        *D.add(18 + 2) = -*f.add(4);
        *D.add(18 + 1) = *f.add(5);

        *D.add(24 + 2) = *f.add(3);
        *D.add(24 + 0) = -*f.add(5);

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
    // SAFETY: caller guarantees D points to [36] array, vel points to [6] array
    unsafe {
        crate::engine::engine_util_blas::mju_zero(D, 36);

        *D.add(0 + 1) = -*vel.add(2);
        *D.add(0 + 2) = *vel.add(1);
        *D.add(0 + 4) = -*vel.add(5);
        *D.add(0 + 5) = *vel.add(4);

        *D.add(6 + 0) = *vel.add(2);
        *D.add(6 + 2) = -*vel.add(0);
        *D.add(6 + 3) = *vel.add(5);
        *D.add(6 + 5) = -*vel.add(3);

        *D.add(12 + 0) = -*vel.add(1);
        *D.add(12 + 1) = *vel.add(0);
        *D.add(12 + 3) = -*vel.add(4);
        *D.add(12 + 4) = *vel.add(3);

        *D.add(18 + 4) = -*vel.add(2);
        *D.add(18 + 5) = *vel.add(1);

        *D.add(24 + 3) = *vel.add(2);
        *D.add(24 + 5) = -*vel.add(0);

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
    todo!() // mjd_mulInertVec_vel
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
    use crate::engine::engine_util_blas::{mju_zero, mju_copy, mju_mul_mat_mat};

    // SAFETY: m, d are valid model/data pointers; Dcvel is nbody*6*nv; Dcdofdot is nv*6*nv
    unsafe {
        let nv = (*m).nv as i32;
        let nbody = (*m).nbody as i32;
        let mut mat = [0.0f64; 36];

        // clear Dcvel
        mju_zero(Dcvel, nbody * 6 * nv);

        // forward pass over bodies: accumulate Dcvel, set Dcdofdot
        for i in 1..nbody {
            // Dcvel = Dcvel_parent
            let parent = *(*m).body_parentid.add(i as usize);
            mju_copy(
                Dcvel.add((i * 6 * nv) as usize),
                Dcvel.add((parent * 6 * nv) as usize),
                6 * nv,
            );

            // Dcvel += D(cdof * qvel),  Dcdofdot = D(cvel x cdof)
            let dofadr = *(*m).body_dofadr.add(i as usize);
            let dofnum = *(*m).body_dofnum.add(i as usize);
            let mut j = dofadr;
            while j < dofadr + dofnum {
                let jnt_type = *(*m).jnt_type.add(*(*m).dof_jntid.add(j as usize) as usize) as u32;
                match jnt_type {
                    // mjJNT_FREE = 0
                    0 => {
                        // Dcdofdot = 0
                        mju_zero(Dcdofdot.add((j * 6 * nv) as usize), 18 * nv);

                        // Dcvel += cdof * (D qvel)
                        for k in 0..6_i32 {
                            *Dcvel.add((i * 6 * nv + k * nv + j + 0) as usize) +=
                                *(*d).cdof.add(((j + 0) * 6 + k) as usize);
                            *Dcvel.add((i * 6 * nv + k * nv + j + 1) as usize) +=
                                *(*d).cdof.add(((j + 1) * 6 + k) as usize);
                            *Dcvel.add((i * 6 * nv + k * nv + j + 2) as usize) +=
                                *(*d).cdof.add(((j + 2) * 6 + k) as usize);
                        }

                        // continue with rotations
                        j += 3;

                        // FALLTHROUGH to mjJNT_BALL
                        // Dcdofdot = D crossMotion(cvel, cdof)
                        for k in 0..3_i32 {
                            mjd_cross_motion_vel(mat.as_mut_ptr(), (*d).cdof.add(((j + k) * 6) as usize));
                            mju_mul_mat_mat(
                                Dcdofdot.add(((j + k) * 6 * nv) as usize),
                                mat.as_ptr(),
                                Dcvel.add((i * 6 * nv) as usize),
                                6, 6, nv,
                            );
                        }

                        // Dcvel += cdof * (D qvel)
                        for k in 0..6_i32 {
                            *Dcvel.add((i * 6 * nv + k * nv + j + 0) as usize) +=
                                *(*d).cdof.add(((j + 0) * 6 + k) as usize);
                            *Dcvel.add((i * 6 * nv + k * nv + j + 1) as usize) +=
                                *(*d).cdof.add(((j + 1) * 6 + k) as usize);
                            *Dcvel.add((i * 6 * nv + k * nv + j + 2) as usize) +=
                                *(*d).cdof.add(((j + 2) * 6 + k) as usize);
                        }

                        // adjust for 3-dof joint
                        j += 2;
                    }
                    // mjJNT_BALL = 1
                    1 => {
                        // Dcdofdot = D crossMotion(cvel, cdof)
                        for k in 0..3_i32 {
                            mjd_cross_motion_vel(mat.as_mut_ptr(), (*d).cdof.add(((j + k) * 6) as usize));
                            mju_mul_mat_mat(
                                Dcdofdot.add(((j + k) * 6 * nv) as usize),
                                mat.as_ptr(),
                                Dcvel.add((i * 6 * nv) as usize),
                                6, 6, nv,
                            );
                        }

                        // Dcvel += cdof * (D qvel)
                        for k in 0..6_i32 {
                            *Dcvel.add((i * 6 * nv + k * nv + j + 0) as usize) +=
                                *(*d).cdof.add(((j + 0) * 6 + k) as usize);
                            *Dcvel.add((i * 6 * nv + k * nv + j + 1) as usize) +=
                                *(*d).cdof.add(((j + 1) * 6 + k) as usize);
                            *Dcvel.add((i * 6 * nv + k * nv + j + 2) as usize) +=
                                *(*d).cdof.add(((j + 2) * 6 + k) as usize);
                        }

                        // adjust for 3-dof joint
                        j += 2;
                    }
                    // default (SLIDE, HINGE)
                    _ => {
                        // Dcdofdot = D crossMotion(cvel, cdof) * Dcvel
                        mjd_cross_motion_vel(mat.as_mut_ptr(), (*d).cdof.add((j * 6) as usize));
                        mju_mul_mat_mat(
                            Dcdofdot.add((j * 6 * nv) as usize),
                            mat.as_ptr(),
                            Dcvel.add((i * 6 * nv) as usize),
                            6, 6, nv,
                        );

                        // Dcvel += cdof * (D qvel)
                        for k in 0..6_i32 {
                            *Dcvel.add((i * 6 * nv + k * nv + j) as usize) +=
                                *(*d).cdof.add((j * 6 + k) as usize);
                        }
                    }
                }
                j += 1;
            }
        }
    }
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
    // SAFETY: caller guarantees m is valid mjModel, mat is valid array, n is valid body index
    unsafe {
        // return if this is world or parent is world
        if n == 0 || *(*m).body_weldid.add(*(*m).body_parentid.add(n as usize) as usize) == 0 {
            return;
        }

        // count dofs in ancestors
        let mut ndof: i32 = 0;
        let mut np = *(*m).body_weldid.add(*(*m).body_parentid.add(n as usize) as usize);
        while np > 0 {
            // add self dofs
            ndof += *(*m).body_dofnum.add(np as usize);

            // advance to parent
            np = *(*m).body_weldid.add(*(*m).body_parentid.add(np as usize) as usize);
        }

        // copy: guaranteed to be at beginning of sparse array, due to sorting
        crate::engine::engine_util_blas::mju_copy(
            mat.add(6 * *(*m).B_rowadr.add(n as usize) as usize),
            mat.add(6 * *(*m).B_rowadr.add(*(*m).body_parentid.add(n as usize) as usize) as usize),
            6 * ndof,
        );
    }
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
    todo!() // addToParent
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
    todo!() // mjd_comVel_vel
}

/// C: mjd_rne_vel (engine/engine_derivative.c:596)
/// Calls: addToParent, copyFromParent, mj_freeStack, mj_markStack, mj_stackAllocInfo, mjd_comVel_vel, mjd_crossForce_frc, mjd_crossForce_vel, mjd_mulInertVec_vel, mju_addTo, mju_addToScl, mju_mulInertVec, mju_mulMatMat, mju_mulMatVec, mju_subFrom, mju_transpose, mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mjd_rne_vel(m: *const mjModel, d: *mut mjData) {
    todo!() // mjd_rne_vel
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
    todo!() // addJTBJ
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
    // SAFETY: caller guarantees m, d are valid pointers; J, B, J_rownnz, J_rowadr, J_colind
    // point to valid arrays with proper sizes for the sparse computation.
    unsafe {
        for i in 0..n {
            for j in 0..n {
                if *B.add((i * n + j) as usize) == 0.0 {
                    continue;
                }

                let nnz_i = *J_rownnz.add((offset + i) as usize);
                let adr_i = *J_rowadr.add((offset + i) as usize);
                let nnz_j = *J_rownnz.add((offset + j) as usize);
                let adr_j = *J_rowadr.add((offset + j) as usize);

                for k in 0..nnz_i {
                    let ik = adr_i + k;
                    let colik = *J_colind.add(ik as usize);

                    crate::engine::engine_util_sparse::mju_add_to_scl_sparse_inc(
                        (*d).qDeriv.add(*(*m).D_rowadr.add(colik as usize) as usize),
                        J.add(adr_j as usize),
                        *(*m).D_rownnz.add(colik as usize),
                        (*m).D_colind.add(*(*m).D_rowadr.add(colik as usize) as usize),
                        nnz_j,
                        J_colind.add(adr_j as usize),
                        *J.add(ik as usize) * *B.add((i * n + j) as usize),
                    );
                }
            }
        }
    }
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
    const MJ_MINVAL: f64 = 1E-15_f64;

    // SAFETY: caller guarantees lengthrange[2], prm[9] are valid
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
            force = scale / crate::engine::engine_util_misc::mju_max(MJ_MINVAL, acc0);
        }

        // optimum length
        let L0 = (*lengthrange.add(1) - *lengthrange.add(0))
            / crate::engine::engine_util_misc::mju_max(MJ_MINVAL, range1 - range0);

        // normalized length and velocity
        let L = range0 + (len - *lengthrange.add(0))
            / crate::engine::engine_util_misc::mju_max(MJ_MINVAL, L0);
        let V = vel / crate::engine::engine_util_misc::mju_max(MJ_MINVAL, L0 * vmax);

        // length curve
        let FL = crate::engine::engine_util_misc::mju_muscle_gain_length(L, lmin, lmax);

        // velocity curve derivative
        let dFV: f64;
        let y = fvmax - 1.0;
        if V <= -1.0 {
            dFV = 0.0;
        } else if V <= 0.0 {
            dFV = 2.0 * V + 2.0;
        } else if V <= y {
            dFV = (-2.0 * V + 2.0 * y)
                / crate::engine::engine_util_misc::mju_max(MJ_MINVAL, y);
        } else {
            dFV = 0.0;
        }

        // compute FVL and scale, make it negative
        -force * FL * dFV / crate::engine::engine_util_misc::mju_max(MJ_MINVAL, L0 * vmax)
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
    todo!() // addJTBJ_mulSparse
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
    todo!() // mjd_flexInterp_kernel
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
    // SAFETY: size points to array of 3 f64; dir is 0, 1, or 2 (caller contract)
    unsafe {
        const MJ_PI: f64 = 3.14159265358979323846_f64;
        let d0 = *size.add(dir as usize);
        let d1 = *size.add(((dir + 1) % 3) as usize);
        let d2 = *size.add(((dir + 2) % 3) as usize);
        8.0 / 15.0 * MJ_PI * d0 * pow2(pow2(crate::engine::engine_util_misc::mju_max(d1, d2)))
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
    // SAFETY: B points to a 6x6 matrix, D points to 9 f64 elements (caller contract)
    unsafe {
        let r = 3 * row_quad as usize;
        let c = 3 * col_quad as usize;
        *B.add(6 * (c + 0) + r + 0) += *D.add(0);
        *B.add(6 * (c + 0) + r + 1) += *D.add(1);
        *B.add(6 * (c + 0) + r + 2) += *D.add(2);
        *B.add(6 * (c + 1) + r + 0) += *D.add(3);
        *B.add(6 * (c + 1) + r + 1) += *D.add(4);
        *B.add(6 * (c + 1) + r + 2) += *D.add(5);
        *B.add(6 * (c + 2) + r + 0) += *D.add(6);
        *B.add(6 * (c + 2) + r + 1) += *D.add(7);
        *B.add(6 * (c + 2) + r + 2) += *D.add(8);
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
    // SAFETY: B points to 6x6 = 36 f64; local_vels points to [6];
    //   virtual_mass, virtual_inertia point to [3]
    unsafe {
        let lin_vel: [f64; 3] = [
            *local_vels.add(3),
            *local_vels.add(4),
            *local_vels.add(5),
        ];
        let ang_vel: [f64; 3] = [
            *local_vels.add(0),
            *local_vels.add(1),
            *local_vels.add(2),
        ];
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
        for i in 0..9_usize {
            Da[i] *= fluid_density * *virtual_inertia.add(i % 3);
        }
        add_to_quadrant(B, Da.as_ptr(), 0, 0);

        // force[:3] += cross(virtual_lin_mom, lin_vel)
        mjd_cross(virtual_lin_mom.as_ptr(), lin_vel.as_ptr(), Da.as_mut_ptr(), Db.as_mut_ptr());
        add_to_quadrant(B, Db.as_ptr(), 0, 1);
        for i in 0..9_usize {
            Da[i] *= fluid_density * *virtual_mass.add(i % 3);
        }
        add_to_quadrant(B, Da.as_ptr(), 0, 1);

        // force[3:] += cross(virtual_lin_mom, ang_vel)
        mjd_cross(virtual_lin_mom.as_ptr(), ang_vel.as_ptr(), Da.as_mut_ptr(), Db.as_mut_ptr());
        add_to_quadrant(B, Db.as_ptr(), 1, 0);
        for i in 0..9_usize {
            Da[i] *= fluid_density * *virtual_mass.add(i % 3);
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
    const MJ_PI: f64 = 3.14159265358979323846_f64;
    const MJ_MINVAL: f64 = 1E-15_f64;

    // SAFETY: caller guarantees D[9], lvel[6], size[3] are valid
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

        // viscous force and torque in Stokes flow
        let eq_sphere_D = 2.0 / 3.0 * (*size.add(0) + *size.add(1) + *size.add(2));
        let lin_visc_torq_coef = MJ_PI * eq_sphere_D * eq_sphere_D * eq_sphere_D;

        // moments of inertia used to compute angular quadratic drag
        let I_max = 8.0 / 15.0 * MJ_PI * d_mid * (d_max * d_max) * (d_max * d_max);
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
                MJ_MINVAL,
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
        let diag = x * mom_sq[0] + y * mom_sq[1] + z * mom_sq[2] - lin_coef;
        *D.add(0) = diag;
        *D.add(4) = diag;
        *D.add(8) = diag;

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
    const MJ_PI: f64 = 3.14159265358979323846_f64;
    const MJ_MINVAL: f64 = 1E-15_f64;

    // SAFETY: caller guarantees D[9], lvel[6], size[3] are valid
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

        let eq_sphere_D = 2.0 / 3.0 * (*size.add(0) + *size.add(1) + *size.add(2));
        let A_max = MJ_PI * d_max * d_mid;

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
        let dA_coef = MJ_PI
            / crate::engine::engine_util_misc::mju_max(
                MJ_MINVAL,
                f64::sqrt(proj_num * proj_num * proj_num * proj_denom),
            );

        let A_proj = MJ_PI
            * f64::sqrt(
                proj_denom / crate::engine::engine_util_misc::mju_max(MJ_MINVAL, proj_num),
            );

        let norm = f64::sqrt(xx + yy + zz);
        let inv_norm = 1.0 / crate::engine::engine_util_misc::mju_max(MJ_MINVAL, norm);

        let lin_coef = fluid_viscosity * 3.0 * MJ_PI * eq_sphere_D;
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
    const MJ_PI: f64 = 3.14159265358979323846_f64;
    const MJ_MINVAL: f64 = 1E-15_f64;

    // SAFETY: caller guarantees D[9], lvel[6], size[3] are valid
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
        let df_denom = MJ_PI * kutta_lift_coef * fluid_density
            / crate::engine::engine_util_misc::mju_max(
                MJ_MINVAL,
                f64::sqrt(proj_denom * proj_num * norm2),
            );

        let dfx_coef = yy * (a - b) + zz * (a - c);
        let dfy_coef = xx * (b - a) + zz * (b - c);
        let dfz_coef = xx * (c - a) + yy * (c - b);
        let proj_term = proj_num
            / crate::engine::engine_util_misc::mju_max(MJ_MINVAL, proj_denom);
        let cos_term = proj_num
            / crate::engine::engine_util_misc::mju_max(MJ_MINVAL, norm2);

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
    const MJ_PI: f64 = 3.14159265358979323846_f64;

    // SAFETY: caller guarantees B[36], lvel[6], size[3] are valid
    unsafe {
        let volume = 4.0 / 3.0 * MJ_PI * *size.add(0) * *size.add(1) * *size.add(2);

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
    todo!() // mjd_ellipsoidFluid
}

/// C: mjd_inertiaBoxFluid (engine/engine_derivative.c:1724)
/// Calls: addJTBJ, addJTBJSparse, mj_bodyChain, mj_freeStack, mj_isSparse, mj_jacBodyCom, mj_jacSparse, mj_markStack, mj_objectVelocity, mj_stackAllocInfo, mju_copy, mju_copy3, mju_max, mju_mulMatTMat, mju_subFrom3, mju_transformSpatial, mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mjd_inertia_box_fluid(m: *const mjModel, d: *mut mjData, i: i32) {
    todo!() // mjd_inertiaBoxFluid
}

/// C: mjd_subQuat (engine/engine_derivative.h:27)
/// Calls: mju_addToScl, mju_copy9, mju_mulMatMat3, mju_normalize3, mju_scl, mju_subQuat, mju_transpose
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjd_sub_quat(qa: *const f64, qb: *const f64, Da: *mut f64, Db: *mut f64) {
    todo!() // mjd_subQuat
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
    todo!() // mjd_quatIntegrate
}

/// C: mjd_smooth_vel (engine/engine_derivative.h:35)
/// Calls: mjd_actuator_vel, mjd_passive_vel, mjd_rne_vel, mju_zero, mju_zeroSparse
#[allow(unused_variables, non_snake_case)]
pub fn mjd_smooth_vel(m: *const mjModel, d: *mut mjData, flg_bias: i32) {
    todo!() // mjd_smooth_vel
}

/// C: mjd_actuator_vel (engine/engine_derivative.h:38)
/// Calls: addJTBJSparse, mj_actuatorDisabled, mj_nextActivation, mj_sleepState, mjd_muscleGain_vel, mju_max
#[allow(unused_variables, non_snake_case)]
pub fn mjd_actuator_vel(m: *const mjModel, d: *mut mjData) {
    todo!() // mjd_actuator_vel
}

/// C: mjd_passive_vel (engine/engine_derivative.h:41)
/// Calls: addJTBJSparse, mj_actuatorDamping, mjd_ellipsoidFluid, mjd_inertiaBoxFluid, mjd_xPolyForce, mju_copy
#[allow(unused_variables, non_snake_case)]
pub fn mjd_passive_vel(m: *const mjModel, d: *mut mjData) {
    todo!() // mjd_passive_vel
}

/// C: mjd_rne_vel_dense (engine/engine_derivative.h:44)
/// Calls: mj_freeStack, mj_markStack, mj_stackAllocInfo, mjd_comVel_vel_dense, mjd_crossForce_frc, mjd_crossForce_vel, mjd_mulInertVec_vel, mju_addTo, mju_addToScl, mju_copy, mju_mulInertVec, mju_mulMatMat, mju_scl, mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mjd_rne_vel_dense(m: *const mjModel, d: *mut mjData) {
    todo!() // mjd_rne_vel_dense
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
    todo!() // mjd_flexInterp_mul
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
    todo!() // mjd_flexInterp_cacheKrot
}

/// C: mjd_flexBend_mul (engine/engine_derivative.h:56)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjd_flex_bend_mul(m: *const mjModel, d: *mut mjData, res: *mut f64, vec: *const f64, s1: f64, s2: f64) {
    todo!() // mjd_flexBend_mul
}

