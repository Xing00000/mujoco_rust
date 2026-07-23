//! Port of: engine/engine_derivative.c
//! IR hash: 73a9f665ec0ecfc0
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
    // SAFETY: caller guarantees D points to 36 f64s, i points to 10 f64s
    unsafe {
        crate::engine::engine_util_blas::mju_zero(D, 36);

        *D.add(0 + 0) = *i.add(0);
        *D.add(0 + 1) = *i.add(3);
        *D.add(0 + 2) = *i.add(4);
        *D.add(0 + 4) = -*i.add(8);
        *D.add(0 + 5) = *i.add(7);

        *D.add(6 + 0) = *i.add(3);
        *D.add(6 + 1) = *i.add(1);
        *D.add(6 + 2) = *i.add(5);
        *D.add(6 + 3) = *i.add(8);
        *D.add(6 + 5) = -*i.add(6);

        *D.add(12 + 0) = *i.add(4);
        *D.add(12 + 1) = *i.add(5);
        *D.add(12 + 2) = *i.add(2);
        *D.add(12 + 3) = -*i.add(7);
        *D.add(12 + 4) = *i.add(6);

        *D.add(18 + 1) = *i.add(8);
        *D.add(18 + 2) = -*i.add(7);
        *D.add(18 + 3) = *i.add(9);

        *D.add(24 + 2) = *i.add(6);
        *D.add(24 + 0) = -*i.add(8);
        *D.add(24 + 4) = *i.add(9);

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
    // SAFETY: m is a valid mjModel pointer (caller contract); all array accesses
    // are within model-allocated bounds guaranteed by mujoco's data layout.
    unsafe {
        // return if this is world or parent is world
        if n == 0 || *(*m).body_weldid.add(*(*m).body_parentid.add(n as usize) as usize) == 0 {
            return;
        }

        // find matching nonzeros
        let np = *(*m).body_parentid.add(n as usize);
        let mut i: i32 = 0;
        let mut ip: i32 = 0;
        while i < *(*m).B_rownnz.add(n as usize) && ip < *(*m).B_rownnz.add(np as usize) {
            let col_n = *(*m).B_colind.add((*(*m).B_rowadr.add(n as usize) + i) as usize);
            let col_np = *(*m).B_colind.add((*(*m).B_rowadr.add(np as usize) + ip) as usize);

            // columns match
            if col_n == col_np {
                crate::engine::engine_util_blas::mju_add_to(
                    mat.add(6 * (*(*m).B_rowadr.add(np as usize) + ip) as usize),
                    mat.add(6 * (*(*m).B_rowadr.add(n as usize) + i) as usize),
                    6,
                );

                // advance both
                i += 1;
                ip += 1;
            }
            // mismatch columns: advance parent
            else if col_n > col_np {
                ip += 1;
            }
            // child nonzeroes must be subset of parent; SHOULD NOT OCCUR
            else {
                crate::engine::engine_util_errmem::mju_error(
                    b"child nonzeroes must be subset of parent\0".as_ptr() as *const i8,
                );
            }
        }
    }
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
    const mjENBL_SLEEP: i32 = 1 << 4;
    const mjJNT_FREE: i32 = 0;
    const mjJNT_BALL: i32 = 1;
    const mjJNT_HINGE: i32 = 3;
    const mjJNT_SLIDE: i32 = 2;

    // SAFETY: m, d, Dcvel, Dcdofdot are valid pointers (caller contract).
    unsafe {
        let nv = (*m).nv as i32;
        let nM = (*m).nM as i32;
        let sleep_filter = (((*m).opt.enableflags & mjENBL_SLEEP) != 0)
            && ((*d).nbody_awake < (*m).nbody as i32);
        let nbody = if sleep_filter { (*d).nbody_awake } else { (*m).nbody as i32 };
        let Badr: *const i32 = (*m).B_rowadr;
        let Dadr: *const i32 = (*m).D_rowadr;
        let mut mat: [f64; 36] = [0.0; 36];
        let mut matT: [f64; 36] = [0.0; 36];

        // forward pass over bodies: accumulate Dcvel, set Dcdofdot
        for b in 1..nbody {
            let i = if sleep_filter { *(*d).body_awake_ind.add(b as usize) } else { b };

            // Dcvel = Dcvel_parent
            copy_from_parent(m, d, Dcvel, i);

            // process all dofs of this body
            let doflast = *(*m).body_dofadr.add(i as usize) + *(*m).body_dofnum.add(i as usize);
            let mut j = *(*m).body_dofadr.add(i as usize);
            while j < doflast {
                // number of dof ancestors of dof j
                let Jadr = (if j < nv - 1 { *(*m).dof_Madr.add((j + 1) as usize) } else { nM })
                    - (*(*m).dof_Madr.add(j as usize) + 1);

                let jnt_type = *(*m).jnt_type.add(*(*m).dof_jntid.add(j as usize) as usize);

                if jnt_type == mjJNT_FREE {
                    // Dcvel += cdof * D(qvel)
                    crate::engine::engine_util_blas::mju_add_to(
                        Dcvel.add((6 * (*Badr.add(i as usize) + Jadr + 0)) as usize),
                        (*d).cdof.add((6 * (j + 0)) as usize), 6);
                    crate::engine::engine_util_blas::mju_add_to(
                        Dcvel.add((6 * (*Badr.add(i as usize) + Jadr + 1)) as usize),
                        (*d).cdof.add((6 * (j + 1)) as usize), 6);
                    crate::engine::engine_util_blas::mju_add_to(
                        Dcvel.add((6 * (*Badr.add(i as usize) + Jadr + 2)) as usize),
                        (*d).cdof.add((6 * (j + 2)) as usize), 6);

                    // continue with rotations (fallthrough to BALL)
                    j += 3;
                    let Jadr2 = Jadr + 3;

                    // Dcdofdot = Dcvel * D crossMotion(cvel, cdof)
                    for dj in 0..3i32 {
                        mjd_cross_motion_vel(mat.as_mut_ptr(), (*d).cdof.add((6 * (j + dj)) as usize));
                        crate::engine::engine_util_blas::mju_transpose(matT.as_mut_ptr(), mat.as_ptr(), 6, 6);
                        crate::engine::engine_util_blas::mju_mul_mat_mat(
                            Dcdofdot.add((6 * *Dadr.add((j + dj) as usize)) as usize),
                            Dcvel.add((6 * *Badr.add(i as usize)) as usize),
                            matT.as_ptr(), Jadr2 + dj, 6, 6);
                    }

                    // Dcvel += cdof * (D qvel)
                    crate::engine::engine_util_blas::mju_add_to(
                        Dcvel.add((6 * (*Badr.add(i as usize) + Jadr2 + 0)) as usize),
                        (*d).cdof.add((6 * (j + 0)) as usize), 6);
                    crate::engine::engine_util_blas::mju_add_to(
                        Dcvel.add((6 * (*Badr.add(i as usize) + Jadr2 + 1)) as usize),
                        (*d).cdof.add((6 * (j + 1)) as usize), 6);
                    crate::engine::engine_util_blas::mju_add_to(
                        Dcvel.add((6 * (*Badr.add(i as usize) + Jadr2 + 2)) as usize),
                        (*d).cdof.add((6 * (j + 2)) as usize), 6);

                    // adjust for 3-dof joint
                    j += 2;
                } else if jnt_type == mjJNT_BALL {
                    // Dcdofdot = Dcvel * D crossMotion(cvel, cdof)
                    for dj in 0..3i32 {
                        mjd_cross_motion_vel(mat.as_mut_ptr(), (*d).cdof.add((6 * (j + dj)) as usize));
                        crate::engine::engine_util_blas::mju_transpose(matT.as_mut_ptr(), mat.as_ptr(), 6, 6);
                        crate::engine::engine_util_blas::mju_mul_mat_mat(
                            Dcdofdot.add((6 * *Dadr.add((j + dj) as usize)) as usize),
                            Dcvel.add((6 * *Badr.add(i as usize)) as usize),
                            matT.as_ptr(), Jadr + dj, 6, 6);
                    }

                    // Dcvel += cdof * (D qvel)
                    crate::engine::engine_util_blas::mju_add_to(
                        Dcvel.add((6 * (*Badr.add(i as usize) + Jadr + 0)) as usize),
                        (*d).cdof.add((6 * (j + 0)) as usize), 6);
                    crate::engine::engine_util_blas::mju_add_to(
                        Dcvel.add((6 * (*Badr.add(i as usize) + Jadr + 1)) as usize),
                        (*d).cdof.add((6 * (j + 1)) as usize), 6);
                    crate::engine::engine_util_blas::mju_add_to(
                        Dcvel.add((6 * (*Badr.add(i as usize) + Jadr + 2)) as usize),
                        (*d).cdof.add((6 * (j + 2)) as usize), 6);

                    // adjust for 3-dof joint
                    j += 2;
                } else {
                    // HINGE or SLIDE
                    // Dcdofdot = D crossMotion(cvel, cdof) * Dcvel
                    mjd_cross_motion_vel(mat.as_mut_ptr(), (*d).cdof.add((6 * j) as usize));
                    crate::engine::engine_util_blas::mju_transpose(matT.as_mut_ptr(), mat.as_ptr(), 6, 6);
                    crate::engine::engine_util_blas::mju_mul_mat_mat(
                        Dcdofdot.add((6 * *Dadr.add(j as usize)) as usize),
                        Dcvel.add((6 * *Badr.add(i as usize)) as usize),
                        matT.as_ptr(), Jadr, 6, 6);

                    // Dcvel += cdof * (D qvel)
                    crate::engine::engine_util_blas::mju_add_to(
                        Dcvel.add((6 * (*Badr.add(i as usize) + Jadr)) as usize),
                        (*d).cdof.add((6 * j) as usize), 6);
                }

                j += 1;
            }
        }
    }
}

/// C: mjd_rne_vel (engine/engine_derivative.c:596)
/// Calls: addToParent, copyFromParent, mj_freeStack, mj_markStack, mj_stackAllocInfo, mjd_comVel_vel, mjd_crossForce_frc, mjd_crossForce_vel, mjd_mulInertVec_vel, mju_addTo, mju_addToScl, mju_mulInertVec, mju_mulMatMat, mju_mulMatVec, mju_subFrom, mju_transpose, mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mjd_rne_vel(m: *const mjModel, d: *mut mjData) {
    const MJ_ENBL_SLEEP: i32 = 1 << 4;

    // SAFETY: m, d are valid pointers (caller contract). All array accesses bounded by model dims.
    unsafe {
        let nM = (*m).nM as i32;
        let mnv = (*m).nv as i32;
        let sleep_filter = (((*m).opt.enableflags & MJ_ENBL_SLEEP) != 0)
            && (*d).nbody_awake < (*m).nbody as i32;
        let nbody = if sleep_filter { (*d).nbody_awake } else { (*m).nbody as i32 };
        let nparent = if sleep_filter { (*d).nparent_awake } else { (*m).nbody as i32 };
        let nv = if sleep_filter { (*d).nv_awake } else { mnv };

        let Badr = (*m).B_rowadr;
        let Dadr = (*m).D_rowadr;
        let Bnnz = (*m).B_rownnz;

        let mut mat: [f64; 36] = [0.0; 36];
        let mut mat1: [f64; 36] = [0.0; 36];
        let mut mat2: [f64; 36] = [0.0; 36];
        let mut dmul: [f64; 36] = [0.0; 36];
        let mut tmp: [f64; 6] = [0.0; 6];

        crate::engine::engine_memory::mj_mark_stack(d);
        let Dcdofdot = crate::engine::engine_memory::mj_stack_alloc_num(d, (6 * (*m).nD) as usize);
        let Dcvel = crate::engine::engine_memory::mj_stack_alloc_num(d, (6 * (*m).nB) as usize);
        let Dcacc = crate::engine::engine_memory::mj_stack_alloc_num(d, (6 * (*m).nB) as usize);
        let Dcfrcbody = crate::engine::engine_memory::mj_stack_alloc_num(d, (6 * (*m).nB) as usize);
        let row = crate::engine::engine_memory::mj_stack_alloc_num(d, mnv as usize);

        // clear
        if !sleep_filter {
            crate::engine::engine_util_blas::mju_zero(Dcdofdot, (6 * (*m).nD) as i32);
            crate::engine::engine_util_blas::mju_zero(Dcvel, (6 * (*m).nB) as i32);
            crate::engine::engine_util_blas::mju_zero(Dcacc, (6 * (*m).nB) as i32);
            crate::engine::engine_util_blas::mju_zero(Dcfrcbody, (6 * (*m).nB) as i32);
        } else {
            for i in 0..nv {
                let dof = *(*d).dof_awake_ind.add(i as usize);
                crate::engine::engine_util_blas::mju_zero(
                    Dcdofdot.add((6 * *(*m).D_rowadr.add(dof as usize)) as usize),
                    6 * *(*m).D_rownnz.add(dof as usize),
                );
            }
            for i in 0..nbody {
                let body = *(*d).body_awake_ind.add(i as usize);
                let adr = 6 * *(*m).B_rowadr.add(body as usize);
                let nnz = 6 * *(*m).B_rownnz.add(body as usize);
                crate::engine::engine_util_blas::mju_zero(Dcvel.add(adr as usize), nnz);
                crate::engine::engine_util_blas::mju_zero(Dcacc.add(adr as usize), nnz);
                crate::engine::engine_util_blas::mju_zero(Dcfrcbody.add(adr as usize), nnz);
            }
        }

        // compute Dcvel and Dcdofdot
        mjd_com_vel_vel(m, d, Dcvel, Dcdofdot);

        // forward pass over bodies: accumulate Dcacc, set Dcfrcbody
        for b in 1..nbody {
            let i = if sleep_filter { *(*d).body_awake_ind.add(b as usize) } else { b };

            // Dcacc = Dcacc_parent
            copy_from_parent(m, d, Dcacc, i);

            // process all dofs of this body
            let doflast = *(*m).body_dofadr.add(i as usize) + *(*m).body_dofnum.add(i as usize);
            let mut j = *(*m).body_dofadr.add(i as usize);
            while j < doflast {
                // number of dof ancestors of dof j
                let Jadr = (if j < mnv - 1 {
                    *(*m).dof_Madr.add((j + 1) as usize)
                } else {
                    nM
                }) - (*(*m).dof_Madr.add(j as usize) + 1);

                // Dcacc += cdofdot * (D qvel)
                crate::engine::engine_util_blas::mju_add_to(
                    Dcacc.add((6 * (*Badr.add(i as usize) + Jadr)) as usize),
                    (*d).cdof_dot.add((6 * j) as usize),
                    6,
                );

                // Dcacc += (D cdofdot) * qvel
                crate::engine::engine_util_blas::mju_add_to_scl(
                    Dcacc.add((6 * *Badr.add(i as usize)) as usize),
                    Dcdofdot.add((6 * *Dadr.add(j as usize)) as usize),
                    *(*d).qvel.add(j as usize),
                    6 * *Bnnz.add(i as usize),
                );
                j += 1;
            }

            // Dcfrcbody = D(cinert * cacc + cvel x (cinert * cvel))
            // Dcfrcbody = (D mul / D cacc) * Dcacc
            mjd_mul_inert_vec_vel(dmul.as_mut_ptr(), (*d).cinert.add((10 * i) as usize));
            crate::engine::engine_util_blas::mju_transpose(mat1.as_mut_ptr(), dmul.as_mut_ptr(), 6, 6);
            crate::engine::engine_util_blas::mju_mul_mat_mat(
                Dcfrcbody.add((6 * *Badr.add(i as usize)) as usize),
                Dcacc.add((6 * *Badr.add(i as usize)) as usize),
                mat1.as_mut_ptr(),
                *Bnnz.add(i as usize), 6, 6,
            );

            // mat = (D cross / D cvel) + (D cross / D mul) * (D mul / D cvel)
            crate::engine::engine_util_spatial::mju_mul_inert_vec(
                tmp.as_mut_ptr(), (*d).cinert.add((10 * i) as usize), (*d).cvel.add((i * 6) as usize),
            );
            mjd_cross_force_vel(mat.as_mut_ptr(), tmp.as_ptr());
            mjd_cross_force_frc(mat1.as_mut_ptr(), (*d).cvel.add((i * 6) as usize));
            crate::engine::engine_util_blas::mju_mul_mat_mat(
                mat2.as_mut_ptr(), mat1.as_mut_ptr(), dmul.as_mut_ptr(), 6, 6, 6,
            );
            crate::engine::engine_util_blas::mju_add_to(mat.as_mut_ptr(), mat2.as_ptr(), 36);

            // Dcfrcbody += mat * Dcvel  (use worldbody as temp)
            crate::engine::engine_util_blas::mju_transpose(mat1.as_mut_ptr(), mat.as_mut_ptr(), 6, 6);
            crate::engine::engine_util_blas::mju_mul_mat_mat(
                Dcfrcbody,
                Dcvel.add((6 * *Badr.add(i as usize)) as usize),
                mat1.as_mut_ptr(),
                *Bnnz.add(i as usize), 6, 6,
            );
            crate::engine::engine_util_blas::mju_add_to(
                Dcfrcbody.add((6 * *Badr.add(i as usize)) as usize),
                Dcfrcbody,
                6 * *Bnnz.add(i as usize),
            );
        }

        // clear worldbody Dcfrcbody
        crate::engine::engine_util_blas::mju_zero(Dcfrcbody, 6 * *Bnnz.add(0));

        // backward pass over bodies: accumulate Dcfrcbody
        for b in (1..nparent).rev() {
            let i = if sleep_filter { *(*d).parent_awake_ind.add(b as usize) } else { b };
            add_to_parent(m, d, Dcfrcbody, i);
        }

        // process all dofs, update qDeriv
        for v in 0..nv {
            let j = if sleep_filter { *(*d).dof_awake_ind.add(v as usize) } else { v };

            // get body index
            let i = *(*m).dof_bodyid.add(j as usize);

            // qDeriv -= D(cdof * cfrc_body)
            crate::engine::engine_util_blas::mju_mul_mat_vec(
                row, Dcfrcbody.add((6 * *Badr.add(i as usize)) as usize),
                (*d).cdof.add((6 * j) as usize),
                *Bnnz.add(i as usize), 6,
            );
            crate::engine::engine_util_blas::mju_sub_from(
                (*d).qDeriv.add(*Dadr.add(j as usize) as usize),
                row,
                *Bnnz.add(i as usize),
            );
        }

        crate::engine::engine_memory::mj_free_stack(d);
    }
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
    // SAFETY: m, d, J, B are valid pointers (caller contract). nv-sized arrays.
    unsafe {
        let nv = (*m).nv as i32;

        // allocate dense row
        crate::engine::engine_memory::mj_mark_stack(d);
        let row: *mut f64 = crate::engine::engine_memory::mj_stack_alloc_num(d, nv as usize);

        // process non-zero elements of B
        for i in 0..n {
            for j in 0..n {
                if *B.add((i * n + j) as usize) == 0.0 {
                    continue;
                }
                // process non-zero elements of J(i,:)
                for k in 0..nv {
                    if *J.add((i * nv + k) as usize) != 0.0 {
                        // row = J(i,k)*B(i,j)*J(j,:)
                        crate::engine::engine_util_blas::mju_scl(
                            row,
                            J.add((j * nv) as usize),
                            *J.add((i * nv + k) as usize) * *B.add((i * n + j) as usize),
                            nv,
                        );

                        // add row to qDeriv(k,:)
                        let rownnz_k = *(*m).D_rownnz.add(k as usize);
                        for s in 0..rownnz_k {
                            let adr = *(*m).D_rowadr.add(k as usize) + s;
                            *(*d).qDeriv.add(adr as usize) +=
                                *row.add(*(*m).D_colind.add(adr as usize) as usize);
                        }
                    }
                }
            }
        }

        crate::engine::engine_memory::mj_free_stack(d);
    }
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
    // SAFETY: all pointers are valid (caller contract).
    unsafe {
        // allocate temp vectors
        crate::engine::engine_memory::mj_mark_stack(d);
        let Jv = crate::engine::engine_memory::mj_stack_alloc_num(d, n as usize);
        let BJv = crate::engine::engine_memory::mj_stack_alloc_num(d, n as usize);

        // Jv = J*vec (Sparse Matrix-Vector Multiplication)
        crate::engine::engine_util_blas::mju_zero(Jv, n);
        for i in 0..n {
            let nnz = *J_rownnz.add(i as usize);
            let adr = *J_rowadr.add(i as usize);
            for k in 0..nnz {
                *Jv.add(i as usize) +=
                    *J.add((adr + k) as usize) * *vec.add(*J_colind.add((adr + k) as usize) as usize);
            }
        }

        // BJv = B*Jv (Dense Matrix-Vector Multiplication)
        crate::engine::engine_util_blas::mju_mul_mat_vec(BJv, B, Jv, n, n);

        // res += J'*BJv (Sparse Transpose Matrix-Vector Multiplication)
        for i in 0..n {
            let nnz = *J_rownnz.add(i as usize);
            let adr = *J_rowadr.add(i as usize);
            let val = *BJv.add(i as usize);
            for k in 0..nnz {
                *res.add(*J_colind.add((adr + k) as usize) as usize) +=
                    *J.add((adr + k) as usize) * val;
            }
        }

        crate::engine::engine_memory::mj_free_stack(d);
    }
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
    // SAFETY: m, d are valid pointers. All array accesses bounded by model dimensions.
    unsafe {
        let nv = (*m).nv as i32;

        // compute upper bounds across all interpolated flexes
        let mut max_nodenum: i32 = 0;
        let mut max_npe: i32 = 0;
        for f in 0..(*m).nflex as i32 {
            if *(*m).flex_interp.add(f as usize) == 0 { continue; }
            if *(*m).flex_rigid.add(f as usize) { continue; }
            let mut order = *(*m).flex_interp.add(f as usize);
            let shell_mode = order < 0;
            if order < 0 { order = -order; }
            let npe = if shell_mode {
                (order + 1) * (order + 1)
            } else {
                (order + 1) * (order + 1) * (order + 1)
            };
            if npe > max_npe { max_npe = npe; }
            if *(*m).flex_nodenum.add(f as usize) > max_nodenum {
                max_nodenum = *(*m).flex_nodenum.add(f as usize);
            }
        }

        // nothing to do
        if max_npe == 0 {
            return;
        }

        let max_dim_c = 3 * max_npe;

        // single unconditional markStack
        crate::engine::engine_memory::mj_mark_stack(d);

        // per-flex node positions (upper bound)
        let xpos = crate::engine::engine_memory::mj_stack_alloc_num(d, (3 * max_nodenum) as usize);

        // per-element arrays (upper bound)
        let xpos_c = crate::engine::engine_memory::mj_stack_alloc_num(d, (3 * max_npe) as usize);
        let K_rot_cell = crate::engine::engine_memory::mj_stack_alloc_num(d, (max_dim_c * max_dim_c) as usize);

        // sparse Jacobian for one cell (upper bound)
        let J_rownnz = crate::engine::engine_memory::mj_stack_alloc_int(d, max_dim_c as usize);
        let J_rowadr = crate::engine::engine_memory::mj_stack_alloc_int(d, max_dim_c as usize);
        let J_val = crate::engine::engine_memory::mj_stack_alloc_num(d, (max_dim_c * nv) as usize);
        let J_colind = crate::engine::engine_memory::mj_stack_alloc_int(d, (max_dim_c * nv) as usize);

        // temp allocations for chain
        let chain_colind = crate::engine::engine_memory::mj_stack_alloc_int(d, nv as usize);
        let blk_jac = crate::engine::engine_memory::mj_stack_alloc_num(d, (3 * nv) as usize);

        // loop over flexes
        for f in 0..(*m).nflex as i32 {
            // only process flex_interp
            if *(*m).flex_interp.add(f as usize) == 0 {
                continue;
            }

            // get stiffness
            let stiffnessadr = *(*m).flex_stiffnessadr.add(f as usize);
            if stiffnessadr < 0 {
                continue;
            }
            let K = (*m).flex_stiffness.offset(stiffnessadr as isize);

            // skip if rigid or no stiffness
            if *(*m).flex_rigid.add(f as usize) || *K == 0.0 {
                continue;
            }

            // skip if strain constraints present
            if *(*m).flex_edgeequality.add(f as usize) == 3 {
                continue;
            }

            // compute scale
            let damping = *(*m).flex_damping.add(f as usize);
            let scale = s1 + s2 * damping;

            // skip if scale is zero
            if scale == 0.0 {
                continue;
            }

            let mut order = *(*m).flex_interp.add(f as usize);
            let shell_mode = order < 0;
            if order < 0 { order = -order; }

            let cx = *(*m).flex_cellnum.add(3 * f as usize);
            let cy = *(*m).flex_cellnum.add(3 * f as usize + 1);
            let cz = *(*m).flex_cellnum.add(3 * f as usize + 2);

            let bodyid = (*m).flex_nodebodyid.offset(*(*m).flex_nodeadr.add(f as usize) as isize);

            // determine element type
            let npe: i32;
            let nelem_fe: i32;
            if shell_mode {
                npe = (order + 1) * (order + 1);
                nelem_fe = 2 * (cy * cz + cx * cz + cx * cy);
            } else {
                npe = (order + 1) * (order + 1) * (order + 1);
                nelem_fe = cx * cy * cz;
            }
            let dim_e = 3 * npe;

            // gather raw node positions (unrotated)
            crate::engine::engine_core_util::mju_flex_gather_state(
                m, d as *const crate::types::mjData, f, xpos, std::ptr::null_mut());

            // check if centered fast path applies
            let mut use_fast_path: i32 = if *(*m).flex_centered.add(f as usize) { 1 } else { 0 };
            if use_fast_path != 0 {
                let nodenum_f = *(*m).flex_nodenum.add(f as usize);
                for n in 0..nodenum_f {
                    if *(*m).body_simple.add(*bodyid.add(n as usize) as usize) != 2 {
                        use_fast_path = 0;
                        break;
                    }
                }
            }

            // loop over finite elements
            for fe in 0..nelem_fe {
                // get element stiffness
                let k_elem = K.offset((fe as isize) * (3 * npe as isize) * (3 * npe as isize));

                // skip empty elements
                if *k_elem == 0.0 {
                    continue;
                }

                // use cached K_rot or compute from scratch
                let mut gindices: [i32; 125] = [0; 125];
                let krot_adr = stiffnessadr + fe * dim_e * dim_e;
                if !K_rot_cache.is_null() {
                    // read K_rot from cache and apply scale
                    for ii in 0..(dim_e * dim_e) {
                        *K_rot_cell.add(ii as usize) = scale * *K_rot_cache.add(krot_adr as usize + ii as usize);
                    }

                    // recompute gindices
                    if shell_mode {
                        crate::engine::engine_util_misc::mju_flex_gather_face_state(
                            order, cx, cy, cz, fe,
                            std::ptr::null(), std::ptr::null(), std::ptr::null(),
                            std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut(),
                            gindices.as_mut_ptr(), std::ptr::null_mut());
                    } else {
                        let ci = fe / (cy * cz);
                        let cj = (fe / cz) % cy;
                        let ck = fe % cz;
                        crate::engine::engine_util_misc::mju_flex_gather_cell_state(
                            order, cy, cz, ci, cj, ck,
                            std::ptr::null(), std::ptr::null(), std::ptr::null(),
                            std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut(),
                            gindices.as_mut_ptr(), std::ptr::null_mut());
                    }
                } else {
                    // gather element-local node positions and rotation
                    let mut quat: [f64; 4] = [0.0; 4];
                    if shell_mode {
                        crate::engine::engine_util_misc::mju_flex_gather_face_state(
                            order, cx, cy, cz, fe, xpos, std::ptr::null(), std::ptr::null(),
                            xpos_c, std::ptr::null_mut(), std::ptr::null_mut(),
                            gindices.as_mut_ptr(), quat.as_mut_ptr());
                    } else {
                        let ci = fe / (cy * cz);
                        let cj = (fe / cz) % cy;
                        let ck = fe % cz;
                        crate::engine::engine_util_misc::mju_flex_gather_cell_state(
                            order, cy, cz, ci, cj, ck, xpos, std::ptr::null(), std::ptr::null(),
                            xpos_c, std::ptr::null_mut(), std::ptr::null_mut(),
                            gindices.as_mut_ptr(), quat.as_mut_ptr());
                    }

                    // R = R_global2local, RT = R_local2global
                    let mut R: [f64; 9] = [0.0; 9];
                    let mut RT: [f64; 9] = [0.0; 9];
                    crate::engine::engine_util_spatial::mju_quat2mat(R.as_mut_ptr(), quat.as_ptr());
                    crate::engine::engine_util_blas::mju_transpose(RT.as_mut_ptr(), R.as_ptr(), 3, 3);

                    // compute K_rot = RT * K_elem * R (block-wise)
                    crate::engine::engine_util_blas::mju_zero(K_rot_cell, dim_e * dim_e);
                    for a in 0..npe {
                        for b in 0..npe {
                            let mut blk: [f64; 9] = [0.0; 9];
                            let mut tmp: [f64; 9] = [0.0; 9];

                            // get K_elem(a,b) 3x3 block
                            let adr_cell = (3 * a) * (3 * npe) + 3 * b;
                            for r in 0..3_i32 {
                                for c in 0..3_i32 {
                                    blk[(3 * r + c) as usize] = *k_elem.add(
                                        (adr_cell + r * (3 * npe) + c) as usize);
                                }
                            }

                            // tmp = K * R
                            crate::engine::engine_util_blas::mju_mul_mat_mat3(
                                tmp.as_mut_ptr(), blk.as_ptr(), R.as_ptr());
                            // blk = RT * tmp = RT * K * R
                            crate::engine::engine_util_blas::mju_mul_mat_mat3(
                                blk.as_mut_ptr(), RT.as_ptr(), tmp.as_ptr());

                            // store in K_rot_cell at (a, b)
                            let adr_out = (3 * a) * dim_e + 3 * b;
                            for r in 0..3_i32 {
                                for c in 0..3_i32 {
                                    *K_rot_cell.add((adr_out + r * dim_e + c) as usize) =
                                        scale * blk[(3 * r + c) as usize];
                                }
                            }
                        }
                    }

                    // optionally store unscaled K_rot to output cache
                    if !K_rot_out.is_null() {
                        for ii in 0..(dim_e * dim_e) {
                            *K_rot_out.add(krot_adr as usize + ii as usize) =
                                *K_rot_cell.add(ii as usize) / scale;
                        }
                    }
                }

                // skip Jacobian construction when only caching (res == NULL)
                if res.is_null() {
                    continue;
                }

                // fast path: centered flex with 3 translational DOFs per body
                if use_fast_path != 0 {
                    for a in 0..npe {
                        let dof_a = *(*m).body_dofadr.add(*bodyid.add(gindices[a as usize] as usize) as usize);
                        for b in 0..npe {
                            let dof_b = *(*m).body_dofadr.add(*bodyid.add(gindices[b as usize] as usize) as usize);
                            let adr = (3 * a) * dim_e + 3 * b;
                            for r in 0..3_i32 {
                                let mut val: f64 = 0.0;
                                for c in 0..3_i32 {
                                    val += *K_rot_cell.add((adr + r * dim_e + c) as usize)
                                        * *vec.add((dof_b + c) as usize);
                                }
                                *res.add((dof_a + r) as usize) += val;
                            }
                        }
                    }
                } else {
                    // general path: construct sparse Jacobian for this element's nodes
                    let mut current_adr: i32 = 0;
                    for n in 0..npe {
                        let bid = *bodyid.add(gindices[n as usize] as usize);
                        let chain_nnz = crate::engine::engine_core_util::mj_body_chain(m, bid, chain_colind);
                        crate::engine::engine_core_util::mj_jac_sparse(
                            m, d as *const crate::types::mjData, blk_jac, std::ptr::null_mut(),
                            xpos.offset(3 * gindices[n as usize] as isize), bid,
                            chain_nnz, chain_colind, 0);

                        for r in 0..3_i32 {
                            let row_idx = 3 * n + r;
                            *J_rownnz.add(row_idx as usize) = chain_nnz;
                            *J_rowadr.add(row_idx as usize) = current_adr;

                            for idx in 0..chain_nnz {
                                *J_colind.add(current_adr as usize) = *chain_colind.add(idx as usize);
                                *J_val.add(current_adr as usize) = *blk_jac.add((r * chain_nnz + idx) as usize);
                                current_adr += 1;
                            }
                        }
                    }

                    // res += J'*K_rot*J*vec
                    add_jtbj_mul_sparse(m, d, res, vec, J_rownnz, J_rowadr, J_colind,
                                        J_val, K_rot_cell, dim_e);
                }
            }
        }

        crate::engine::engine_memory::mj_free_stack(d);
    }
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
    const mjNFLUID: i32 = 12;
    const mjOBJ_GEOM: i32 = 5;
    const mjINT_IMPLICITFAST: i32 = 3;

    // SAFETY: m, d are valid pointers (caller contract).
    unsafe {
        crate::engine::engine_memory::mj_mark_stack(d);

        let nv = (*m).nv as i32;
        let mut nnz: i32 = nv;
        let mut rownnz: [i32; 6] = [0; 6];
        let mut rowadr: [i32; 6] = [0; 6];
        let J: *mut f64 = crate::engine::engine_memory::mj_stack_alloc_num(d, (6 * nv) as usize);
        let tmp: *mut f64 = crate::engine::engine_memory::mj_stack_alloc_num(d, (3 * nv) as usize);
        let colind: *mut i32 = crate::engine::engine_memory::mj_stack_alloc_int(d, (6 * nv) as usize);
        let colind_compressed: *mut i32 = crate::engine::engine_memory::mj_stack_alloc_int(d, (6 * nv) as usize);

        let mut lvel: [f64; 6] = [0.0; 6];
        let mut wind: [f64; 6] = [0.0; 6];
        let mut lwind: [f64; 6] = [0.0; 6];
        let mut geom_interaction_coef: f64 = 0.0;
        let mut magnus_lift_coef: f64 = 0.0;
        let mut kutta_lift_coef: f64 = 0.0;
        let mut semiaxes: [f64; 3] = [0.0; 3];
        let mut virtual_mass: [f64; 3] = [0.0; 3];
        let mut virtual_inertia: [f64; 3] = [0.0; 3];
        let mut blunt_drag_coef: f64 = 0.0;
        let mut slender_drag_coef: f64 = 0.0;
        let mut ang_drag_coef: f64 = 0.0;

        if crate::engine::engine_core_util::mj_is_sparse(m) != 0 {
            // get sparse body Jacobian structure
            nnz = crate::engine::engine_core_util::mj_body_chain(m, bodyid, colind);

            // prepare rownnz, rowadr, colind for all 6 rows
            for i in 0..6i32 {
                rownnz[i as usize] = nnz;
                rowadr[i as usize] = if i == 0 { 0 } else { rowadr[(i - 1) as usize] + nnz };
                for k in 0..nnz {
                    *colind_compressed.add((i * nnz + k) as usize) = *colind.add(k as usize);
                }
            }
        }

        for j in 0..*(*m).body_geomnum.add(bodyid as usize) {
            let geomid = *(*m).body_geomadr.add(bodyid as usize) + j;

            crate::engine::engine_util_misc::mju_geom_semi_axes(
                semiaxes.as_mut_ptr(),
                (*m).geom_size.add((3 * geomid) as usize),
                *(*m).geom_type.add(geomid as usize) as u32);

            crate::engine::engine_passive::read_fluid_geom_interaction(
                (*m).geom_fluid.add((mjNFLUID * geomid) as usize),
                &mut geom_interaction_coef,
                &mut blunt_drag_coef, &mut slender_drag_coef, &mut ang_drag_coef,
                &mut kutta_lift_coef, &mut magnus_lift_coef,
                virtual_mass.as_mut_ptr(), virtual_inertia.as_mut_ptr());

            // scales all forces
            if geom_interaction_coef == 0.0 {
                continue;
            }

            // map from CoM-centered to local body-centered 6D velocity
            crate::engine::engine_core_util::mj_object_velocity(
                m, d as *const crate::types::mjData, mjOBJ_GEOM, geomid, lvel.as_mut_ptr(), 1);

            // compute wind in local coordinates
            crate::engine::engine_util_blas::mju_zero(wind.as_mut_ptr(), 6);
            crate::engine::engine_util_blas::mju_copy3(wind.as_mut_ptr().add(3), (*m).opt.wind.as_ptr());
            crate::engine::engine_util_spatial::mju_transform_spatial(
                lwind.as_mut_ptr(), wind.as_ptr(), 0,
                (*d).geom_xpos.add((3 * geomid) as usize),
                (*d).subtree_com.add((3 * *(*m).body_rootid.add(bodyid as usize)) as usize),
                (*d).geom_xmat.add((9 * geomid) as usize));

            // subtract translational component from geom velocity
            crate::engine::engine_util_blas::mju_sub_from3(lvel.as_mut_ptr().add(3), lwind.as_ptr().add(3));

            // get geom global Jacobian: rotation then translation
            if crate::engine::engine_core_util::mj_is_sparse(m) != 0 {
                crate::engine::engine_core_util::mj_jac_sparse(
                    m, d as *const crate::types::mjData,
                    J.add((3 * nnz) as usize), J,
                    (*d).geom_xpos.add((3 * geomid) as usize),
                    *(*m).geom_bodyid.add(geomid as usize), nnz, colind as *const i32, 0);
            } else {
                crate::engine::engine_core_util::mj_jac_geom(
                    m, d as *const crate::types::mjData,
                    J.add((3 * nv) as usize), J, geomid);
            }

            // rotate (compressed) Jacobian to local frame
            crate::engine::engine_util_blas::mju_mul_mat_t_mat(
                tmp, (*d).geom_xmat.add((9 * geomid) as usize), J, 3, 3, nnz);
            crate::engine::engine_util_blas::mju_copy(J, tmp as *const f64, 3 * nnz);
            crate::engine::engine_util_blas::mju_mul_mat_t_mat(
                tmp, (*d).geom_xmat.add((9 * geomid) as usize), J.add((3 * nnz) as usize), 3, 3, nnz);
            crate::engine::engine_util_blas::mju_copy(J.add((3 * nnz) as usize), tmp as *const f64, 3 * nnz);

            let mut B: [f64; 36] = [0.0; 36];
            let mut D: [f64; 9] = [0.0; 9];
            crate::engine::engine_util_blas::mju_zero(B.as_mut_ptr(), 36);
            mjd_magnus_force(B.as_mut_ptr(), lvel.as_ptr(), (*m).opt.density, semiaxes.as_ptr(), magnus_lift_coef);

            mjd_kutta_lift(D.as_mut_ptr(), lvel.as_ptr(), (*m).opt.density, semiaxes.as_ptr(), kutta_lift_coef);
            add_to_quadrant(B.as_mut_ptr(), D.as_ptr(), 1, 1);

            mjd_viscous_drag(D.as_mut_ptr(), lvel.as_ptr(), (*m).opt.density, (*m).opt.viscosity,
                            semiaxes.as_ptr(), blunt_drag_coef, slender_drag_coef);
            add_to_quadrant(B.as_mut_ptr(), D.as_ptr(), 1, 1);

            mjd_viscous_torque(D.as_mut_ptr(), lvel.as_ptr(), (*m).opt.density, (*m).opt.viscosity,
                              semiaxes.as_ptr(), slender_drag_coef, ang_drag_coef);
            add_to_quadrant(B.as_mut_ptr(), D.as_ptr(), 0, 0);

            mjd_added_mass_forces(B.as_mut_ptr(), lvel.as_ptr(), (*m).opt.density,
                                 virtual_mass.as_ptr(), virtual_inertia.as_ptr());

            // make B symmetric if integrator is IMPLICITFAST
            if (*m).opt.integrator == mjINT_IMPLICITFAST {
                crate::engine::engine_util_blas::mju_symmetrize(B.as_mut_ptr(), B.as_ptr(), 6);
            }

            if crate::engine::engine_core_util::mj_is_sparse(m) != 0 {
                add_jtbj_sparse(m, d, J as *const f64, B.as_ptr(), 6, 0,
                    rownnz.as_ptr(), rowadr.as_ptr(), colind_compressed as *const i32);
            } else {
                add_jtbj(m, d, J as *const f64, B.as_ptr(), 6);
            }
        }

        crate::engine::engine_memory::mj_free_stack(d);
    }
}

/// C: mjd_inertiaBoxFluid (engine/engine_derivative.c:1724)
/// Calls: addJTBJ, addJTBJSparse, mj_bodyChain, mj_freeStack, mj_isSparse, mj_jacBodyCom, mj_jacSparse, mj_markStack, mj_objectVelocity, mj_stackAllocInfo, mju_copy, mju_copy3, mju_max, mju_mulMatTMat, mju_subFrom3, mju_transformSpatial, mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mjd_inertia_box_fluid(m: *const mjModel, d: *mut mjData, i: i32) {
    const MJ_MINVAL: f64 = 1e-15;
    const MJ_PI: f64 = std::f64::consts::PI;
    const mjOBJ_BODY: i32 = 1;

    // SAFETY: m, d are valid pointers (caller contract).
    unsafe {
        crate::engine::engine_memory::mj_mark_stack(d);

        let nv = (*m).nv as i32;
        let mut rownnz: [i32; 6] = [0; 6];
        let mut rowadr: [i32; 6] = [0; 6];
        let J: *mut f64 = crate::engine::engine_memory::mj_stack_alloc_num(d, (6 * nv) as usize);
        let tmp: *mut f64 = crate::engine::engine_memory::mj_stack_alloc_num(d, (3 * nv) as usize);
        let colind: *mut i32 = crate::engine::engine_memory::mj_stack_alloc_int(d, (6 * nv) as usize);

        let mut lvel: [f64; 6] = [0.0; 6];
        let mut wind: [f64; 6] = [0.0; 6];
        let mut lwind: [f64; 6] = [0.0; 6];
        let mut box_: [f64; 3] = [0.0; 3];
        let inertia: *const f64 = (*m).body_inertia.add((3 * i) as usize);

        // equivalent inertia box
        box_[0] = (crate::engine::engine_util_misc::mju_max(MJ_MINVAL,
            *inertia.add(1) + *inertia.add(2) - *inertia.add(0))
            / *(*m).body_mass.add(i as usize) * 6.0).sqrt();
        box_[1] = (crate::engine::engine_util_misc::mju_max(MJ_MINVAL,
            *inertia.add(0) + *inertia.add(2) - *inertia.add(1))
            / *(*m).body_mass.add(i as usize) * 6.0).sqrt();
        box_[2] = (crate::engine::engine_util_misc::mju_max(MJ_MINVAL,
            *inertia.add(0) + *inertia.add(1) - *inertia.add(2))
            / *(*m).body_mass.add(i as usize) * 6.0).sqrt();

        // map from CoM-centered to local body-centered 6D velocity
        crate::engine::engine_core_util::mj_object_velocity(
            m, d as *const crate::types::mjData, mjOBJ_BODY, i, lvel.as_mut_ptr(), 1);

        // compute wind in local coordinates
        crate::engine::engine_util_blas::mju_zero(wind.as_mut_ptr(), 6);
        crate::engine::engine_util_blas::mju_copy3(wind.as_mut_ptr().add(3), (*m).opt.wind.as_ptr());
        crate::engine::engine_util_spatial::mju_transform_spatial(
            lwind.as_mut_ptr(), wind.as_ptr(), 0,
            (*d).xipos.add((3 * i) as usize),
            (*d).subtree_com.add((3 * *(*m).body_rootid.add(i as usize)) as usize),
            (*d).ximat.add((9 * i) as usize));

        // subtract translational component from body velocity
        crate::engine::engine_util_blas::mju_sub_from3(lvel.as_mut_ptr().add(3), lwind.as_ptr().add(3));

        // init with dense
        let mut nnz: i32 = nv;

        // sparse Jacobian
        if crate::engine::engine_core_util::mj_is_sparse(m) != 0 {
            // get sparse body Jacobian structure
            nnz = crate::engine::engine_core_util::mj_body_chain(m, i, colind);

            // get sparse jacBodyCom
            crate::engine::engine_core_util::mj_jac_sparse(
                m, d as *const crate::types::mjData,
                J.add((3 * nnz) as usize), J,
                (*d).xipos.add((3 * i) as usize), i, nnz, colind as *const i32, 0);

            // prepare rownnz, rowadr, colind for all 6 rows
            rownnz[0] = nnz;
            rowadr[0] = 0;
            for j in 1..6i32 {
                rownnz[j as usize] = nnz;
                rowadr[j as usize] = rowadr[(j - 1) as usize] + nnz;
                for k in 0..nnz {
                    *colind.add((j * nnz + k) as usize) = *colind.add(k as usize);
                }
            }
        }
        // dense Jacobian
        else {
            crate::engine::engine_core_util::mj_jac_body_com(
                m, d as *const crate::types::mjData,
                J.add((3 * nv) as usize), J, i);
        }

        // rotate (compressed) Jacobian to local frame
        crate::engine::engine_util_blas::mju_mul_mat_t_mat(
            tmp, (*d).ximat.add((9 * i) as usize), J, 3, 3, nnz);
        crate::engine::engine_util_blas::mju_copy(J, tmp as *const f64, 3 * nnz);
        crate::engine::engine_util_blas::mju_mul_mat_t_mat(
            tmp, (*d).ximat.add((9 * i) as usize), J.add((3 * nnz) as usize), 3, 3, nnz);
        crate::engine::engine_util_blas::mju_copy(J.add((3 * nnz) as usize), tmp as *const f64, 3 * nnz);

        // add viscous force and torque
        if (*m).opt.viscosity > 0.0 {
            // diameter of sphere approximation
            let diam = (box_[0] + box_[1] + box_[2]) / 3.0;

            // rotational viscous force
            let mut B: f64 = -MJ_PI * diam * diam * diam * (*m).opt.viscosity;
            for j in 0..3i32 {
                if crate::engine::engine_core_util::mj_is_sparse(m) != 0 {
                    add_jtbj_sparse(m, d, J as *const f64, &B, 1, j,
                        rownnz.as_ptr(), rowadr.as_ptr(), colind as *const i32);
                } else {
                    add_jtbj(m, d, J.add((j * nv) as usize) as *const f64, &B, 1);
                }
            }

            // translational viscous force
            B = -3.0 * MJ_PI * diam * (*m).opt.viscosity;
            for j in 0..3i32 {
                if crate::engine::engine_core_util::mj_is_sparse(m) != 0 {
                    add_jtbj_sparse(m, d, J as *const f64, &B, 1, 3 + j,
                        rownnz.as_ptr(), rowadr.as_ptr(), colind as *const i32);
                } else {
                    add_jtbj(m, d, J.add(((3 + j) * nv) as usize) as *const f64, &B, 1);
                }
            }
        }

        // add lift and drag force and torque
        if (*m).opt.density > 0.0 {
            let mut B: f64;

            // lfrc[0] drag
            B = -(*m).opt.density * box_[0]
                * (box_[1] * box_[1] * box_[1] * box_[1] + box_[2] * box_[2] * box_[2] * box_[2])
                * 2.0 * lvel[0].abs() / 64.0;
            if crate::engine::engine_core_util::mj_is_sparse(m) != 0 {
                add_jtbj_sparse(m, d, J as *const f64, &B, 1, 0,
                    rownnz.as_ptr(), rowadr.as_ptr(), colind as *const i32);
            } else {
                add_jtbj(m, d, J as *const f64, &B, 1);
            }

            // lfrc[1] drag
            B = -(*m).opt.density * box_[1]
                * (box_[0] * box_[0] * box_[0] * box_[0] + box_[2] * box_[2] * box_[2] * box_[2])
                * 2.0 * lvel[1].abs() / 64.0;
            if crate::engine::engine_core_util::mj_is_sparse(m) != 0 {
                add_jtbj_sparse(m, d, J as *const f64, &B, 1, 1,
                    rownnz.as_ptr(), rowadr.as_ptr(), colind as *const i32);
            } else {
                add_jtbj(m, d, J.add(nv as usize) as *const f64, &B, 1);
            }

            // lfrc[2] drag
            B = -(*m).opt.density * box_[2]
                * (box_[0] * box_[0] * box_[0] * box_[0] + box_[1] * box_[1] * box_[1] * box_[1])
                * 2.0 * lvel[2].abs() / 64.0;
            if crate::engine::engine_core_util::mj_is_sparse(m) != 0 {
                add_jtbj_sparse(m, d, J as *const f64, &B, 1, 2,
                    rownnz.as_ptr(), rowadr.as_ptr(), colind as *const i32);
            } else {
                add_jtbj(m, d, J.add((2 * nv) as usize) as *const f64, &B, 1);
            }

            // lfrc[3] drag
            B = -0.5 * (*m).opt.density * box_[1] * box_[2] * 2.0 * lvel[3].abs();
            if crate::engine::engine_core_util::mj_is_sparse(m) != 0 {
                add_jtbj_sparse(m, d, J as *const f64, &B, 1, 3,
                    rownnz.as_ptr(), rowadr.as_ptr(), colind as *const i32);
            } else {
                add_jtbj(m, d, J.add((3 * nv) as usize) as *const f64, &B, 1);
            }

            // lfrc[4] drag
            B = -0.5 * (*m).opt.density * box_[0] * box_[2] * 2.0 * lvel[4].abs();
            if crate::engine::engine_core_util::mj_is_sparse(m) != 0 {
                add_jtbj_sparse(m, d, J as *const f64, &B, 1, 4,
                    rownnz.as_ptr(), rowadr.as_ptr(), colind as *const i32);
            } else {
                add_jtbj(m, d, J.add((4 * nv) as usize) as *const f64, &B, 1);
            }

            // lfrc[5] drag
            B = -0.5 * (*m).opt.density * box_[0] * box_[1] * 2.0 * lvel[5].abs();
            if crate::engine::engine_core_util::mj_is_sparse(m) != 0 {
                add_jtbj_sparse(m, d, J as *const f64, &B, 1, 5,
                    rownnz.as_ptr(), rowadr.as_ptr(), colind as *const i32);
            } else {
                add_jtbj(m, d, J.add((5 * nv) as usize) as *const f64, &B, 1);
            }
        }

        crate::engine::engine_memory::mj_free_stack(d);
    }
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
    // SAFETY: qa[4], qb[4] are valid. Da and Db may be null (checked before use).
    unsafe {
        // no outputs, quick return
        if Da.is_null() && Db.is_null() {
            return;
        }

        // compute axis-angle quaternion difference
        let mut axis: [f64; 3] = [0.0; 3];
        crate::engine::engine_util_spatial::mju_sub_quat(axis.as_mut_ptr(), qa, qb);

        // normalize axis, get half-angle
        let half_angle = 0.5 * crate::engine::engine_util_blas::mju_normalize3(axis.as_mut_ptr());

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
        crate::engine::engine_util_blas::mju_add_to_scl(
            Da_tmp.as_mut_ptr(), K.as_ptr(), half_angle, 9);

        // add term linear in K * K
        let mut KK: [f64; 9] = [0.0; 9];
        crate::engine::engine_util_blas::mju_mul_mat_mat3(
            KK.as_mut_ptr(), K.as_ptr(), K.as_ptr());
        let coef = 1.0 - (if half_angle < 6e-8 {
            1.0
        } else {
            half_angle / f64::tan(half_angle)
        });
        crate::engine::engine_util_blas::mju_add_to_scl(
            Da_tmp.as_mut_ptr(), KK.as_ptr(), coef, 9);

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
    // SAFETY: vel points to [3]; Dquat, Dvel, Dscale (if non-null) point to [9], [9], [3]
    unsafe {
        // scaled velocity
        let s: [f64; 3] = [
            scale * *vel.add(0),
            scale * *vel.add(1),
            scale * *vel.add(2),
        ];

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
        let xx = crate::engine::engine_util_blas::mju_dot3(s.as_ptr(), s.as_ptr());
        let x = f64::sqrt(xx);

        // 4 coefficients
        let a = f64::cos(x);
        let b: f64;
        let c: f64;
        let d: f64;

        if f64::abs(x) > 1.0 / 32.0 {
            b = f64::sin(x) / x;
            c = (1.0 - a) / xx;
            d = (1.0 - b) / xx;
        } else {
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
    const MJ_ENBL_SLEEP: i32 = 1 << 4;

    // SAFETY: m, d are valid pointers (caller contract)
    unsafe {
        let sleep_filter = (((*m).opt.enableflags & MJ_ENBL_SLEEP) != 0)
            && ((*d).nv_awake < (*m).nv as i32);

        // clear qDeriv
        if !sleep_filter {
            crate::engine::engine_util_blas::mju_zero((*d).qDeriv, (*m).nD as i32);
        } else {
            crate::engine::engine_util_sparse::mju_zero_sparse(
                (*d).qDeriv, (*m).D_rownnz, (*m).D_rowadr, (*d).dof_awake_ind, (*d).nv_awake,
            );
        }

        // qDeriv += d qfrc_actuator / d qvel
        mjd_actuator_vel(m, d);

        // qDeriv += d qfrc_passive / d qvel
        mjd_passive_vel(m, d);

        // qDeriv -= d qfrc_bias / d qvel; optional
        if flg_bias != 0 {
            mjd_rne_vel(m, d);
        }
    }
}

/// C: mjd_actuator_vel (engine/engine_derivative.h:38)
/// Calls: addJTBJSparse, mj_actuatorDisabled, mj_nextActivation, mj_sleepState, mjd_muscleGain_vel, mju_max
#[allow(unused_variables, non_snake_case)]
pub fn mjd_actuator_vel(m: *const mjModel, d: *mut mjData) {
    const mjENBL_SLEEP: i32 = 1 << 4;
    const mjDSBL_ACTUATION: i32 = 1 << 11;
    const mjS_ASLEEP: i32 = 0;
    const mjOBJ_ACTUATOR: u32 = 19;
    const mjBIAS_AFFINE: i32 = 1;
    const mjBIAS_DCMOTOR: i32 = 3;
    const mjGAIN_AFFINE: i32 = 1;
    const mjGAIN_MUSCLE: i32 = 2;
    const mjGAIN_DCMOTOR: i32 = 3;
    const mjDYN_NONE: i32 = 0;
    const mjNDYN: i32 = 10;
    const mjNGAIN: i32 = 10;
    const mjNBIAS: i32 = 10;
    const MJ_MINVAL: f64 = 1e-15;

    // SAFETY: m, d are valid pointers (caller contract).
    unsafe {
        let nu = (*m).nu as i32;
        let sleep_filter = (((*m).opt.enableflags & mjENBL_SLEEP) != 0)
            && ((*d).ntree_awake < (*m).ntree as i32);

        // disabled: nothing to add
        if ((*m).opt.disableflags & mjDSBL_ACTUATION) != 0 {
            return;
        }

        // process actuators
        for i in 0..nu {
            // skip if disabled
            if crate::engine::engine_support::mj_actuator_disabled(m, i) != 0 {
                continue;
            }

            // skip if sleeping
            if sleep_filter && crate::engine::engine_sleep::mj_sleep_state(
                m, d as *const crate::types::mjData, mjOBJ_ACTUATOR, i) == mjS_ASLEEP {
                continue;
            }

            // skip if force is clamped by forcerange
            if *(*m).actuator_forcelimited.add(i as usize) {
                let force = *(*d).actuator_force.add(i as usize);
                let range = (*m).actuator_forcerange.add((2 * i) as usize);
                if force <= *range.add(0) || force >= *range.add(1) {
                    continue;
                }
            }

            let mut bias_vel: f64 = 0.0;
            let mut gain_vel: f64 = 0.0;

            // affine bias
            if *(*m).actuator_biastype.add(i as usize) == mjBIAS_AFFINE {
                bias_vel = *(*m).actuator_biasprm.add((mjNBIAS * i + 2) as usize);
            }
            // DC motor bias (back-EMF)
            else if *(*m).actuator_biastype.add(i as usize) == mjBIAS_DCMOTOR {
                let dynprm = (*m).actuator_dynprm.add((mjNDYN * i) as usize);
                let gainprm = (*m).actuator_gainprm.add((mjNGAIN * i) as usize);
                if *dynprm.add(0) <= 0.0 {
                    let R = crate::engine::engine_util_misc::mju_max(MJ_MINVAL, *gainprm.add(0));
                    let K = *gainprm.add(1);
                    bias_vel -= K * K / R;
                }
            }

            // affine gain
            if *(*m).actuator_gaintype.add(i as usize) == mjGAIN_AFFINE {
                gain_vel = *(*m).actuator_gainprm.add((mjNGAIN * i + 2) as usize);
            }
            // muscle gain
            else if *(*m).actuator_gaintype.add(i as usize) == mjGAIN_MUSCLE {
                gain_vel = mjd_muscle_gain_vel(
                    *(*d).actuator_length.add(i as usize),
                    *(*d).actuator_velocity.add(i as usize),
                    (*m).actuator_lengthrange.add((2 * i) as usize),
                    *(*m).actuator_acc0.add(i as usize),
                    (*m).actuator_gainprm.add((mjNGAIN * i) as usize));
            }
            // DC motor controller damping and LuGre micro-damping
            else if *(*m).actuator_gaintype.add(i as usize) == mjGAIN_DCMOTOR {
                let dynprm = (*m).actuator_dynprm.add((mjNDYN * i) as usize);
                let gainprm = (*m).actuator_gainprm.add((mjNGAIN * i) as usize);
                let te = *dynprm.add(0);

                // controller velocity derivative: dV/dω
                let input_mode = *gainprm.add(8) as i32;
                let mut dVdw: f64 = 0.0;
                if input_mode == 1 { dVdw = -*gainprm.add(6); }       // position: -kd
                else if input_mode == 2 { dVdw = -*gainprm.add(4); }   // velocity: -kp

                if te > 0.0 {
                    // stateful current with actearly
                    let R = crate::engine::engine_util_misc::mju_max(MJ_MINVAL, *gainprm.add(0));
                    let K = *gainprm.add(1);
                    let s = 1.0 - (-(*m).opt.timestep / te).exp();
                    bias_vel += K * (dVdw - K) * s / R;
                } else if dVdw != 0.0 {
                    // stateless: controller terms only
                    let R = crate::engine::engine_util_misc::mju_max(MJ_MINVAL, *gainprm.add(0));
                    let K = *gainprm.add(1);
                    bias_vel += K * dVdw / R;
                }

                // LuGre: sigma1
                let sigma1 = *dynprm.add(6);
                if sigma1 > 0.0 {
                    bias_vel -= sigma1;
                }
            }

            // force = gain .* [ctrl/act]
            if gain_vel != 0.0 {
                if *(*m).actuator_dyntype.add(i as usize) == mjDYN_NONE {
                    bias_vel += gain_vel * *(*d).ctrl.add(i as usize);
                } else {
                    let act_adr = *(*m).actuator_actadr.add(i as usize)
                        + *(*m).actuator_actnum.add(i as usize) - 1;
                    let mut act = *(*d).act.add(act_adr as usize);

                    // use next activation if actearly is set
                    if *(*m).actuator_actearly.add(i as usize) {
                        act = crate::engine::engine_support::mj_next_activation(
                            m, d as *const crate::types::mjData, i, act_adr,
                            *(*d).act_dot.add(act_adr as usize));
                    }

                    bias_vel += gain_vel * act;
                }
            }

            // add
            if bias_vel != 0.0 {
                add_jtbj_sparse(m, d,
                    (*d).actuator_moment as *const f64,
                    &bias_vel as *const f64, 1, i,
                    (*d).moment_rownnz as *const i32,
                    (*d).moment_rowadr as *const i32,
                    (*d).moment_colind as *const i32);
            }
        }
    }
}

/// C: mjd_passive_vel (engine/engine_derivative.h:41)
/// Calls: addJTBJSparse, mj_actuatorDamping, mjd_ellipsoidFluid, mjd_inertiaBoxFluid, mjd_xPolyForce, mju_copy
#[allow(unused_variables, non_snake_case)]
pub fn mjd_passive_vel(m: *const mjModel, d: *mut mjData) {
    const mjDSBL_SPRING: i32 = 1 << 5;
    const mjDSBL_DAMPER: i32 = 1 << 6;
    const mjENBL_SLEEP: i32 = 1 << 4;
    const MJ_MINVAL: f64 = 1e-15;
    const mjNFLUID: i32 = 12;
    const mjNPOLY: i32 = 2;
    const mjOBJ_JOINT: u32 = 3;
    const mjOBJ_TENDON: u32 = 18;

    // SAFETY: m, d are valid pointers (caller contract).
    unsafe {
        // all disabled: nothing to add
        if ((*m).opt.disableflags & mjDSBL_SPRING) != 0
            && ((*m).opt.disableflags & mjDSBL_DAMPER) != 0
        {
            return;
        }

        let sleep_filter = (((*m).opt.enableflags & mjENBL_SLEEP) != 0)
            && ((*d).ntree_awake < (*m).ntree as i32);
        let nbody = if sleep_filter { (*d).nbody_awake } else { (*m).nbody as i32 };

        // fluid drag model
        if (*m).opt.viscosity > 0.0 || (*m).opt.density > 0.0 {
            for b in 0..nbody {
                let i = if sleep_filter { *(*d).body_awake_ind.add(b as usize) } else { b };

                if *(*m).body_mass.add(i as usize) < MJ_MINVAL {
                    continue;
                }

                let mut use_ellipsoid_model: i32 = 0;
                let mut j = 0i32;
                while j < *(*m).body_geomnum.add(i as usize) && use_ellipsoid_model == 0 {
                    let geomid = *(*m).body_geomadr.add(i as usize) + j;
                    use_ellipsoid_model += if *(*m).geom_fluid.add((mjNFLUID * geomid) as usize) > 0.0 { 1 } else { 0 };
                    j += 1;
                }
                if use_ellipsoid_model != 0 {
                    mjd_ellipsoid_fluid(m, d, i);
                } else {
                    mjd_inertia_box_fluid(m, d, i);
                }
            }
        }

        // disabled: nothing to add
        if ((*m).opt.disableflags & mjDSBL_DAMPER) != 0 {
            return;
        }

        // dof damping
        let nv = (*m).nv as i32;
        let nv_awake = if sleep_filter { (*d).nv_awake } else { nv };
        for j in 0..nv_awake {
            let i = if sleep_filter { *(*d).dof_awake_ind.add(j as usize) } else { j };
            let v = *(*d).qvel.add(i as usize);
            let mut poly: [f64; 2] = [0.0; 2]; // mjNPOLY = 2
            crate::engine::engine_util_blas::mju_copy(
                poly.as_mut_ptr(), (*m).dof_dampingpoly.add((mjNPOLY * i) as usize), mjNPOLY);
            let damping = *(*m).dof_damping.add(i as usize)
                + crate::engine::engine_core_util::mj_actuator_damping(
                    m, mjOBJ_JOINT, *(*m).dof_jntid.add(i as usize), poly.as_mut_ptr());
            let adr = *(*m).D_rowadr.add(i as usize) + *(*m).D_diag.add(i as usize);
            *(*d).qDeriv.add(adr as usize) -= crate::engine::engine_util_misc::mjd_x_poly_force(
                damping, poly.as_ptr(), v, mjNPOLY, 1);
        }

        // flex edge damping
        for f in 0..(*m).nflex as i32 {
            let B = -*(*m).flex_edgedamping.add(f as usize);
            if *(*m).flex_rigid.add(f as usize) || B == 0.0 {
                continue;
            }

            let flex_edgeadr = *(*m).flex_edgeadr.add(f as usize);
            let flex_edgenum = *(*m).flex_edgenum.add(f as usize);

            // process non-rigid edges of this flex
            for e in flex_edgeadr..(flex_edgeadr + flex_edgenum) {
                if *(*m).flexedge_rigid.add(e as usize) {
                    continue;
                }

                // always sparse
                add_jtbj_sparse(m, d, (*d).flexedge_J as *const f64, &B, 1, e,
                    (*m).flexedge_J_rownnz as *const i32,
                    (*m).flexedge_J_rowadr as *const i32,
                    (*m).flexedge_J_colind as *const i32);
            }
        }

        // tendon damping
        let ntendon = (*m).ntendon as i32;
        for i in 0..ntendon {
            // skip tendon in sleeping trees
            if sleep_filter {
                let treenum = *(*m).tendon_treenum.add(i as usize);
                let id1 = *(*m).tendon_treeid.add((2 * i) as usize);
                if treenum == 1 && *(*d).tree_awake.add(id1 as usize) == 0 {
                    continue;
                }
                let id2 = *(*m).tendon_treeid.add((2 * i + 1) as usize);
                if treenum == 2 && *(*d).tree_awake.add(id1 as usize) == 0
                    && *(*d).tree_awake.add(id2 as usize) == 0
                {
                    continue;
                }
            }

            let v = *(*d).ten_velocity.add(i as usize);
            let mut poly: [f64; 2] = [0.0; 2];
            crate::engine::engine_util_blas::mju_copy(
                poly.as_mut_ptr(), (*m).tendon_dampingpoly.add((mjNPOLY * i) as usize), mjNPOLY);
            let damping = *(*m).tendon_damping.add(i as usize)
                + crate::engine::engine_core_util::mj_actuator_damping(
                    m, mjOBJ_TENDON, i, poly.as_mut_ptr());
            let B = -crate::engine::engine_util_misc::mjd_x_poly_force(
                damping, poly.as_ptr(), v, mjNPOLY, 1);

            if B == 0.0 {
                continue;
            }

            // add sparse
            add_jtbj_sparse(m, d, (*d).ten_J as *const f64, &B, 1, i,
                (*m).ten_J_rownnz as *const i32,
                (*m).ten_J_rowadr as *const i32,
                (*m).ten_J_colind as *const i32);
        }
    }
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
    // C: mjd_flexInterp_kernel(m, d, res, vec, s1, s2, K_rot_cache, NULL)
    let K_rot_out: *mut f64 = 0 as *mut f64;
    mjd_flex_interp_kernel(m, d, res, vec, s1, s2, K_rot_cache, K_rot_out);
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
    // C: mjd_flexInterp_kernel(m, d, NULL, NULL, 1, 0, NULL, K_rot_out)
    // use s1=1, s2=0 so scale=1 and K_rot_out gets unscaled values
    let no_res: *mut f64 = 0 as *mut f64;
    let no_vec: *const f64 = 0 as *const f64;
    let no_cache: *const f64 = 0 as *const f64;
    mjd_flex_interp_kernel(m, d, no_res, no_vec, 1.0, 0.0, no_cache, K_rot_out);
}

/// C: mjd_flexBend_mul (engine/engine_derivative.h:56)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjd_flex_bend_mul(m: *const mjModel, d: *mut mjData, res: *mut f64, vec: *const f64, s1: f64, s2: f64) {
    // SAFETY: m, d are valid model/data pointers; res, vec are valid arrays (caller contract)
    unsafe {
        for f in 0..(*m).nflex as i32 {
            // skip interp, rigid, or non-2D
            if *(*m).flex_interp.add(f as usize) != 0 {
                continue;
            }
            if *(*m).flex_rigid.add(f as usize) {
                continue;
            }
            if *(*m).flex_dim.add(f as usize) != 2 {
                continue;
            }

            let bendingadr = *(*m).flex_bendingadr.add(f as usize);
            if bendingadr < 0 {
                continue;
            }

            let scale = s1 + s2 * *(*m).flex_damping.add(f as usize);
            if scale == 0.0 {
                continue;
            }

            let b = (*m).flex_bending.add(bendingadr as usize);
            let bodyid = (*m).flex_vertbodyid.add(*(*m).flex_vertadr.add(f as usize) as usize);
            let edgenum = *(*m).flex_edgenum.add(f as usize);
            let edgeadr = *(*m).flex_edgeadr.add(f as usize);

            for e in 0..edgenum {
                let edge = (*m).flex_edge.add(2 * (e + edgeadr) as usize);
                let flap = (*m).flex_edgeflap.add(2 * (e + edgeadr) as usize);
                let v: [i32; 4] = [*edge.add(0), *edge.add(1), *flap.add(0), *flap.add(1)];

                // skip boundary edges (no second flap vertex)
                if v[3] == -1 {
                    continue;
                }

                // apply 4x4 bending stencil, coordinate-wise
                for i in 0..4 {
                    let dof_i = *(*m).body_dofadr.add(*bodyid.add(v[i] as usize) as usize);
                    for x in 0..3 {
                        let mut val: f64 = 0.0;
                        for j in 0..4 {
                            let dof_j = *(*m).body_dofadr.add(*bodyid.add(v[j] as usize) as usize);
                            val += *b.add(17 * e as usize + 4 * i + j) * *vec.add((dof_j + x) as usize);
                        }
                        *res.add((dof_i + x) as usize) += scale * val;
                    }
                }
            }
        }
    }
}

