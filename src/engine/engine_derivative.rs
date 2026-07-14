//! Port of: engine/engine_derivative.c
//! IR hash: 47ee20b2bff3660e
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
    todo!() // mjd_muscleGain_vel
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
    todo!() // mjd_viscous_torque
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
    todo!() // mjd_viscous_drag
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
    todo!() // mjd_kutta_lift
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
    todo!() // mjd_magnus_force
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

