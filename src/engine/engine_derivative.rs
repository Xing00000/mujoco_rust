//! Port of: engine/engine_derivative.c
//! IR hash: fd5288ad7242f001
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mjd_cross (engine/engine_derivative.c:41)
/// Calls: cxx:_mju_zero
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

/// C: mjd_crossMotion_vel (engine/engine_derivative.c:68)
/// Calls: cxx:_mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjd_cross_motion_vel(D: *mut f64, v: *const f64) {
    todo!() // cxx-internal:engine_derivative.c.o:_mjd_crossMotion_vel
}

/// C: mjd_crossForce_vel (engine/engine_derivative.c:104)
/// Calls: cxx:_mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjd_cross_force_vel(D: *mut f64, f: *const f64) {
    todo!() // cxx-internal:engine_derivative.c.o:_mjd_crossForce_vel
}

/// C: mjd_crossForce_frc (engine/engine_derivative.c:140)
/// Calls: cxx:_mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjd_cross_force_frc(D: *mut f64, vel: *const f64) {
    todo!() // cxx-internal:engine_derivative.c.o:_mjd_crossForce_frc
}

/// C: mjd_mulInertVec_vel (engine/engine_derivative.c:176)
/// Calls: cxx:_mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjd_mul_inert_vec_vel(D: *mut f64, i: *const f64) {
    todo!() // cxx-internal:engine_derivative.c.o:_mjd_mulInertVec_vel
}

/// C: mjd_subQuat (engine/engine_derivative.c:218)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_mulMatMat3, cxx:_mju_addToScl, cxx:_mju_copy9, cxx:_mju_normalize3, cxx:_mju_scl, cxx:_mju_subQuat, cxx:_mju_transpose
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjd_sub_quat(qa: *const f64, qb: *const f64, Da: *mut f64, Db: *mut f64) {
    todo!() // cxx:_mjd_subQuat
}

/// C: mjd_quatIntegrate (engine/engine_derivative.c:266)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_mulMatVec3, cxx:_mju_copy9, cxx:_mju_dot3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjd_quat_integrate(vel: *const f64, scale: f64, Dquat: *mut f64, Dvel: *mut f64, Dscale: *mut f64) {
    todo!() // cxx:_mjd_quatIntegrate
}

/// C: mjd_comVel_vel_dense (engine/engine_derivative.c:324)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative.c:_mjd_crossMotion_vel, cxx:_mju_copy, cxx:_mju_mulMatMat, cxx:_mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjd_com_vel_vel_dense(m: *const mjModel, d: *mut mjData, Dcvel: *mut f64, Dcdofdot: *mut f64) {
    todo!() // cxx-internal:engine_derivative.c.o:_mjd_comVel_vel_dense
}

/// C: mjd_rne_vel_dense (engine/engine_derivative.c:388)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative.c:_mjd_comVel_vel_dense, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative.c:_mjd_crossForce_frc, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative.c:_mjd_crossForce_vel, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative.c:_mjd_mulInertVec_vel, cxx:_mj_freeStack, cxx:_mj_markStack, cxx:_mj_stackAllocInfo, cxx:_mju_addTo, cxx:_mju_addToScl, cxx:_mju_copy, cxx:_mju_mulInertVec, cxx:_mju_mulMatMat, cxx:_mju_scl, cxx:_mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mjd_rne_vel_dense(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx:_mjd_rne_vel_dense
}

/// C: copyFromParent (engine/engine_derivative.c:471)
/// Calls: cxx:_mju_copy
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn copy_from_parent(m: *const mjModel, d: *mut mjData, mat: *mut f64, n: i32) {
    todo!() // cxx-internal:engine_derivative.c.o:_copyFromParent
}

/// C: addToParent (engine/engine_derivative.c:494)
/// Calls: cxx:_mju_addTo, cxx:_mju_message
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn add_to_parent(m: *const mjModel, d: *mut mjData, mat: *mut f64, n: i32) {
    todo!() // cxx-internal:engine_derivative.c.o:_addToParent
}

/// C: mjd_comVel_vel (engine/engine_derivative.c:527)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative.c:_copyFromParent, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative.c:_mjd_crossMotion_vel, cxx:_mju_addTo, cxx:_mju_message, cxx:_mju_mulMatMat, cxx:_mju_transpose
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjd_com_vel_vel(m: *const mjModel, d: *mut mjData, Dcvel: *mut f64, Dcdofdot: *mut f64) {
    todo!() // cxx-internal:engine_derivative.c.o:_mjd_comVel_vel
}

/// C: mjd_rne_vel (engine/engine_derivative.c:599)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative.c:_addToParent, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative.c:_copyFromParent, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative.c:_mjd_comVel_vel, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative.c:_mjd_crossForce_frc, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative.c:_mjd_crossForce_vel, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative.c:_mjd_mulInertVec_vel, cxx:_mj_freeStack, cxx:_mj_markStack, cxx:_mj_stackAllocInfo, cxx:_mju_addTo, cxx:_mju_addToScl, cxx:_mju_mulInertVec, cxx:_mju_mulMatMat, cxx:_mju_mulMatVec, cxx:_mju_subFrom, cxx:_mju_transpose, cxx:_mju_zero
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

        crate::engine::engine_memory::mj_markStack(d);
        let Dcdofdot = crate::engine::engine_memory::mj_stackAllocNum(d, (6 * (*m).nD) as usize);
        let Dcvel = crate::engine::engine_memory::mj_stackAllocNum(d, (6 * (*m).nB) as usize);
        let Dcacc = crate::engine::engine_memory::mj_stackAllocNum(d, (6 * (*m).nB) as usize);
        let Dcfrcbody = crate::engine::engine_memory::mj_stackAllocNum(d, (6 * (*m).nB) as usize);
        let row = crate::engine::engine_memory::mj_stackAllocNum(d, mnv as usize);

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
        mjd_comVel_vel(m, d, Dcvel, Dcdofdot);

        // forward pass over bodies: accumulate Dcacc, set Dcfrcbody
        for b in 1..nbody {
            let i = if sleep_filter { *(*d).body_awake_ind.add(b as usize) } else { b };

            // Dcacc = Dcacc_parent
            copyFromParent(m, d, Dcacc, i);

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
                crate::engine::engine_util_blas::mju_addTo(
                    Dcacc.add((6 * (*Badr.add(i as usize) + Jadr)) as usize),
                    (*d).cdof_dot.add((6 * j) as usize),
                    6,
                );

                // Dcacc += (D cdofdot) * qvel
                crate::engine::engine_util_blas::mju_addToScl(
                    Dcacc.add((6 * *Badr.add(i as usize)) as usize),
                    Dcdofdot.add((6 * *Dadr.add(j as usize)) as usize),
                    *(*d).qvel.add(j as usize),
                    6 * *Bnnz.add(i as usize),
                );
                j += 1;
            }

            // Dcfrcbody = D(cinert * cacc + cvel x (cinert * cvel))
            // Dcfrcbody = (D mul / D cacc) * Dcacc
            mjd_mulInertVec_vel(dmul.as_mut_ptr(), (*d).cinert.add((10 * i) as usize));
            crate::engine::engine_util_blas::mju_transpose(mat1.as_mut_ptr(), dmul.as_mut_ptr(), 6, 6);
            crate::engine::engine_util_blas::mju_mulMatMat(
                Dcfrcbody.add((6 * *Badr.add(i as usize)) as usize),
                Dcacc.add((6 * *Badr.add(i as usize)) as usize),
                mat1.as_mut_ptr(),
                *Bnnz.add(i as usize), 6, 6,
            );

            // mat = (D cross / D cvel) + (D cross / D mul) * (D mul / D cvel)
            crate::engine::engine_util_spatial::mju_mulInertVec(
                tmp.as_mut_ptr(), (*d).cinert.add((10 * i) as usize), (*d).cvel.add((i * 6) as usize),
            );
            mjd_crossForce_vel(mat.as_mut_ptr(), tmp.as_ptr());
            mjd_crossForce_frc(mat1.as_mut_ptr(), (*d).cvel.add((i * 6) as usize));
            crate::engine::engine_util_blas::mju_mulMatMat(
                mat2.as_mut_ptr(), mat1.as_mut_ptr(), dmul.as_mut_ptr(), 6, 6, 6,
            );
            crate::engine::engine_util_blas::mju_addTo(mat.as_mut_ptr(), mat2.as_ptr(), 36);

            // Dcfrcbody += mat * Dcvel  (use worldbody as temp)
            crate::engine::engine_util_blas::mju_transpose(mat1.as_mut_ptr(), mat.as_mut_ptr(), 6, 6);
            crate::engine::engine_util_blas::mju_mulMatMat(
                Dcfrcbody,
                Dcvel.add((6 * *Badr.add(i as usize)) as usize),
                mat1.as_mut_ptr(),
                *Bnnz.add(i as usize), 6, 6,
            );
            crate::engine::engine_util_blas::mju_addTo(
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
            addToParent(m, d, Dcfrcbody, i);
        }

        // process all dofs, update qDeriv
        for v in 0..nv {
            let j = if sleep_filter { *(*d).dof_awake_ind.add(v as usize) } else { v };

            // get body index
            let i = *(*m).dof_bodyid.add(j as usize);

            // qDeriv -= D(cdof * cfrc_body)
            crate::engine::engine_util_blas::mju_mulMatVec(
                row, Dcfrcbody.add((6 * *Badr.add(i as usize)) as usize),
                (*d).cdof.add((6 * j) as usize),
                *Bnnz.add(i as usize), 6,
            );
            crate::engine::engine_util_blas::mju_subFrom(
                (*d).qDeriv.add(*Dadr.add(j as usize) as usize),
                row,
                *Bnnz.add(i as usize),
            );
        }

        crate::engine::engine_memory::mj_freeStack(d);
    }
}

/// C: freeBias_vel_blocks (engine/engine_derivative.c:723)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_cross, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_mulMatMat3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_mulMatVec3, cxx-internal:engine_collision_convex.c.o:_mji_mulMatTMat3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn free_bias_vel_blocks(mass: f64, R: *const f64, Xi: *const f64, inertia: *const f64, s: *const f64, qvel_rot: *const f64, lin: *mut f64, rot: *mut f64) {
    todo!() // cxx-internal:engine_derivative.c.o:_freeBias_vel_blocks
}

/// C: mjd_freeBias_vel (engine/engine_derivative.c:796)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_sub3, cxx-internal:engine_derivative.c.o:_freeBias_vel_blocks, cxx:_mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjd_free_bias_vel(m: *const mjModel, d: *const mjData, jnt: i32, B: *mut f64) {
    todo!() // cxx:_mjd_freeBias_vel
}

/// C: mj_isFreeBody (engine/engine_derivative.c:822)
/// Calls: cxx:__ZN7mjCMesh4treeEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_is_free_body(m: *const mjModel, body: i32) -> bool {
    todo!() // cxx:_mj_isFreeBody
}

/// C: mjd_freeMhat (engine/engine_derivative.c:845)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_sub3, cxx-internal:engine_derivative.c.o:_freeBias_vel_blocks, cxx:_mj_isFreeBody, cxx:_mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjd_free_mhat(m: *const mjModel, d: *const mjData, jnt: i32, h: f64, A: *mut f64, flg_discrete: i32) -> i32 {
    todo!() // cxx:_mjd_freeMhat
}

/// C: mjd_freeGyroPossible (engine/engine_derivative.c:929)
/// Calls: cxx:_mj_isFreeBody, cxx:_mj_isSparse
#[allow(unused_variables, non_snake_case)]
pub fn mjd_free_gyro_possible(m: *const mjModel, d: *const mjData, jnt: i32) -> i32 {
    todo!() // cxx:_mjd_freeGyroPossible
}

/// C: addJTBJ (engine/engine_derivative.c:1002)
/// Calls: cxx:_mj_freeStack, cxx:_mj_markStack, cxx:_mj_stackAllocInfo, cxx:_mju_scl
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn add_jtbj(m: *const mjModel, d: *mut mjData, res: *mut f64, J: *const f64, B: *const f64, n: i32) {
    todo!() // cxx-internal:engine_derivative.c.o:_addJTBJ
}

/// C: addJTBJSparse (engine/engine_derivative.c:1038)
/// Calls: cxx:_mju_addToSclSparseInc
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn add_jtbj_sparse(m: *const mjModel, d: *mut mjData, res: *mut f64, J: *const f64, B: *const f64, n: i32, offset: i32, J_rownnz: *const i32, J_rowadr: *const i32, J_colind: *const i32) {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative.c:_addJTBJSparse
}

/// C: mjd_muscleGainLength (engine/engine_derivative.c:1073)
/// Calls: cxx:_mju_max
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjd_muscle_gain_length(length: f64, lmin: f64, lmax: f64) -> f64 {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative.c:_mjd_muscleGainLength
}

/// C: mjd_muscleGain_len (engine/engine_derivative.c:1103)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative.c:_mjd_muscleGainLength, cxx:_mju_max
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjd_muscle_gain_len(len: f64, vel: f64, lengthrange: *const f64, acc0: f64, prm: *const f64) -> f64 {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative.c:_mjd_muscleGain_len
}

/// C: mjd_muscleBias_len (engine/engine_derivative.c:1148)
/// Calls: cxx:_mju_max
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjd_muscle_bias_len(len: f64, lengthrange: *const f64, acc0: f64, prm: *const f64) -> f64 {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative.c:_mjd_muscleBias_len
}

/// C: mjd_muscleGain_vel (engine/engine_derivative.c:1188)
/// Calls: cxx:_mju_max, cxx:_mju_muscleGainLength
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjd_muscle_gain_vel(len: f64, vel: f64, lengthrange: *const f64, acc0: f64, prm: *const f64) -> f64 {
    todo!() // cxx-internal:engine_derivative.c.o:_mjd_muscleGain_vel
}

/// C: addJTBJ_mulSparse (engine/engine_derivative.c:1239)
/// Calls: cxx:_mj_freeStack, cxx:_mj_markStack, cxx:_mj_stackAllocInfo, cxx:_mju_mulMatVec, cxx:_mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn add_jtbj_mul_sparse(m: *const mjModel, d: *mut mjData, res: *mut f64, vec: *const f64, J_rownnz: *const i32, J_rowadr: *const i32, J_colind: *const i32, J: *const f64, B: *const f64, n: i32) {
    todo!() // cxx-internal:engine_derivative.c.o:_addJTBJ_mulSparse
}

/// C: mjd_flexInterp_kernel (engine/engine_derivative.c:1279)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative.c:_addJTBJ_mulSparse, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_mulMatMat3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_mulMatTVec3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_mulMatVec3, cxx:_mj_bodyChain, cxx:_mj_freeStack, cxx:_mj_jacSparse, cxx:_mj_markStack, cxx:_mj_stackAllocInfo, cxx:_mju_flexGatherCellState, cxx:_mju_flexGatherFaceState, cxx:_mju_flexGatherState, cxx:_mju_quat2Mat, cxx:_mju_transpose, cxx:_mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjd_flex_interp_kernel(m: *const mjModel, d: *mut mjData, res: *mut f64, vec: *const f64, s1: f64, s2: f64, K_rot_cache: *const f64, K_rot_out: *mut f64) {
    todo!() // cxx-internal:engine_derivative.c.o:_mjd_flexInterp_kernel
}

/// C: mjd_flexInterp_mul (engine/engine_derivative.c:1561)
/// Calls: cxx-internal:engine_derivative.c.o:_mjd_flexInterp_kernel
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjd_flex_interp_mul(m: *const mjModel, d: *mut mjData, res: *mut f64, vec: *const f64, s1: f64, s2: f64, K_rot_cache: *const f64) {
    todo!() // cxx:_mjd_flexInterp_mul
}

/// C: mjd_flexInterp_cacheKrot (engine/engine_derivative.c:1567)
/// Calls: cxx-internal:engine_derivative.c.o:_mjd_flexInterp_kernel
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjd_flex_interp_cache_krot(m: *const mjModel, d: *mut mjData, K_rot_out: *mut f64) {
    todo!() // cxx:_mjd_flexInterp_cacheKrot
}

/// C: mjd_flexBend_mul (engine/engine_derivative.c:1578)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_mulMatTVec3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_mulMatVec3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjd_flex_bend_mul(m: *const mjModel, d: *mut mjData, res: *mut f64, vec: *const f64, s1: f64, s2: f64) {
    todo!() // cxx:_mjd_flexBend_mul
}

/// C: mjd_flexStretch_mul (engine/engine_derivative.c:1663)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_mulMatTVec3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_mulMatVec3, cxx:_mju_max
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjd_flex_stretch_mul(m: *const mjModel, d: *mut mjData, res: *mut f64, vec: *const f64, s1: f64, s2: f64) {
    todo!() // cxx:_mjd_flexStretch_mul
}

/// C: flexInterp_processed (engine/engine_derivative.c:1780)
#[allow(unused_variables, non_snake_case)]
pub fn flex_interp_processed(m: *const mjModel, f: i32) -> bool {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative.c:_flexInterp_processed
}

/// C: mjd_flexInterpAssemblable (engine/engine_derivative.c:1800)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative.c:_flexInterp_processed
#[allow(unused_variables, non_snake_case)]
pub fn mjd_flex_interp_assemblable(m: *const mjModel) -> bool {
    todo!() // cxx:_mjd_flexInterpAssemblable
}

/// C: flexPassiveContact_any (engine/engine_derivative.c:1852)
/// Calls: cxx:_mj_effFlexContactPossible
#[allow(unused_variables, non_snake_case)]
pub fn flex_passive_contact_any(m: *const mjModel) -> bool {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative.c:_flexPassiveContact_any
}

/// C: mjd_flexStiff_any (engine/engine_derivative.c:1864)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative.c:_flexInterp_processed
#[allow(unused_variables, non_snake_case)]
pub fn mjd_flex_stiff_any(m: *const mjModel, flg_interp: i32) -> bool {
    todo!() // cxx:_mjd_flexStiff_any
}

/// C: flexStiff_active (engine/engine_derivative.c:1879)
#[allow(unused_variables, non_snake_case)]
pub fn flex_stiff_active(m: *const mjModel, f: i32, flg_bend: i32, flg_stretch: i32) -> bool {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative.c:_flexStiff_active
}

/// C: mjd_flexContactStiffness (engine/engine_derivative.c:1893)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjd_flex_contact_stiffness(m: *const mjModel, d: *const mjData, con: *const mjContact) -> f64 {
    todo!() // cxx:_mjd_flexContactStiffness
}

/// C: mjd_flexStiff_assemble (engine/engine_derivative.c:1939)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative.c:_flexInterp_processed, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative.c:_flexStiff_active, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_mulMatMat3, cxx-internal:engine_collision_convex.c.o:_mji_mulMatTMat3, cxx:_mj_freeStack, cxx:_mj_markStack, cxx:_mj_stackAllocInfo, cxx:_mju_flexGatherCellState, cxx:_mju_flexGatherFaceState, cxx:_mju_max, cxx:_mju_zero, cxx:_mju_zeroInt
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjd_flex_stiff_assemble(m: *const mjModel, d: *mut mjData, rownnz: *mut i32, rowadr: *mut i32, colind: *mut i32, val: *mut f64, s1: f64, s2: f64, flg_bend: i32, flg_stretch: i32, Krot: *const f64) -> i32 {
    todo!() // cxx:_mjd_flexStiff_assemble
}

/// C: actuatorDerivSkip (engine/engine_derivative.c:2387)
/// Calls: cxx:_mj_actuatorDisabled, cxx:_mj_sleepState, cxx:_mju_norm3
#[allow(unused_variables, non_snake_case)]
pub fn actuator_deriv_skip(m: *const mjModel, d: *const mjData, i: i32, sleep_filter: i32) -> i32 {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative.c:_actuatorDerivSkip
}

/// C: actuatorInput (engine/engine_derivative.c:2421)
/// Calls: cxx:_mj_nextActivation, cxx:_mj_readCtrl, cxx:_mju_clip
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn actuator_input(m: *const mjModel, d: *const mjData, i: i32) -> f64 {
    todo!() // cxx-internal:engine_derivative.c.o:_actuatorInput
}

/// C: actuatorLenDeriv (engine/engine_derivative.c:2448)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative.c:_mjd_muscleBias_len, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative.c:_mjd_muscleGain_len, cxx-internal:engine_derivative.c.o:_actuatorInput, cxx:_mju_max
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn actuator_len_deriv(m: *const mjModel, d: *const mjData, i: i32) -> f64 {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative.c:_actuatorLenDeriv
}

/// C: actuatorVelDeriv (engine/engine_derivative.c:2519)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative.c:_mjd_muscleGain_vel, cxx-internal:engine_derivative.c.o:_actuatorInput, cxx:_mju_max
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn actuator_vel_deriv(m: *const mjModel, d: *const mjData, i: i32) -> f64 {
    todo!() // cxx-internal:engine_derivative.c.o:_actuatorVelDeriv
}

/// C: mjd_actuator_vel (engine/engine_derivative.c:2609)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative.c:_actuatorDerivSkip, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative.c:_addJTBJSparse, cxx-internal:engine_derivative.c.o:_actuatorVelDeriv
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
            if crate::engine::engine_support::mj_actuatorDisabled(m, i) != 0 {
                continue;
            }

            // skip if sleeping
            if sleep_filter && crate::engine::engine_sleep::mj_sleepState(
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
                gain_vel = mjd_muscleGain_vel(
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
                        act = crate::engine::engine_support::mj_nextActivation(
                            m, d as *const crate::types::mjData, i, act_adr,
                            *(*d).act_dot.add(act_adr as usize));
                    }

                    bias_vel += gain_vel * act;
                }
            }

            // add
            if bias_vel != 0.0 {
                addJTBJSparse(m, d,
                    (*d).actuator_moment as *const f64,
                    &bias_vel as *const f64, 1, i,
                    (*d).moment_rownnz as *const i32,
                    (*d).moment_rowadr as *const i32,
                    (*d).moment_colind as *const i32);
            }
        }
    }
}

/// C: pow2 (engine/engine_derivative.c:2641)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn pow2(val: f64) -> f64 {
    val * val
}

/// C: ellipsoid_max_moment (engine/engine_derivative.c:2646)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative.c:_pow2, cxx:_mju_max
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

/// C: addToQuadrant (engine/engine_derivative.c:2656)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn add_to_quadrant(B: *mut f64, D: *const f64, col_quad: i32, row_quad: i32) {
    todo!() // cxx-internal:engine_derivative.c.o:_addToQuadrant
}

/// C: mjd_addedMassForces (engine/engine_derivative.c:2673)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative.c:_addToQuadrant, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative.c:_mjd_cross
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjd_added_mass_forces(B: *mut f64, local_vels: *const f64, fluid_density: f64, virtual_mass: *const f64, virtual_inertia: *const f64) {
    todo!() // cxx-internal:engine_derivative.c.o:_mjd_addedMassForces
}

/// C: mjd_viscous_torque (engine/engine_derivative.c:2718)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative.c:_ellipsoid_max_moment, cxx:_mju_addToScl3, cxx:_mju_max, cxx:_mju_min, cxx:_mju_norm3, cxx:_mju_zero
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
        crate::engine::engine_util_blas::mju_addToScl3(D, mom_sq.as_ptr(), x);
        crate::engine::engine_util_blas::mju_addToScl3(D.add(3), mom_sq.as_ptr(), y);
        crate::engine::engine_util_blas::mju_addToScl3(D.add(6), mom_sq.as_ptr(), z);
    }
}

/// C: mjd_viscous_drag (engine/engine_derivative.c:2771)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative.c:_pow2, cxx:_mju_addToScl3, cxx:_mju_max, cxx:_mju_min, cxx:_mju_scl
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
        crate::engine::engine_util_blas::mju_addToScl3(D.add(0), dAproj_dv.as_ptr(), -x);
        crate::engine::engine_util_blas::mju_addToScl3(D.add(3), dAproj_dv.as_ptr(), -y);
        crate::engine::engine_util_blas::mju_addToScl3(D.add(6), dAproj_dv.as_ptr(), -z);

        // diag(D) -= lin_coef
        *D.add(0) -= lin_coef;
        *D.add(4) -= lin_coef;
        *D.add(8) -= lin_coef;
    }
}

/// C: mjd_kutta_lift (engine/engine_derivative.c:2838)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative.c:_pow2, cxx:_mju_addToScl3, cxx:_mju_max, cxx:_mju_scl
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
        crate::engine::engine_util_blas::mju_addToScl3(D.add(0), inner_term.as_ptr(), dfx_coef);
        crate::engine::engine_util_blas::mju_addToScl3(D.add(3), inner_term.as_ptr(), dfy_coef);
        crate::engine::engine_util_blas::mju_addToScl3(D.add(6), inner_term.as_ptr(), dfz_coef);

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

/// C: mjd_magnus_force (engine/engine_derivative.c:2891)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative.c:_addToQuadrant, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative.c:_mjd_cross
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

        addToQuadrant(B, D_ang.as_ptr(), 1, 0);
        addToQuadrant(B, D_lin.as_ptr(), 1, 1);
    }
}

/// C: mjd_ellipsoidFluid (engine/engine_derivative.c:2920)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative.c:_addJTBJSparse, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative.c:_addToQuadrant, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative.c:_mjd_addedMassForces, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative.c:_mjd_kutta_lift, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative.c:_mjd_magnus_force, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative.c:_mjd_viscous_drag, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative.c:_mjd_viscous_torque, cxx-internal:engine_derivative.c.o:_addJTBJ, cxx:_mj_bodyChain, cxx:_mj_freeStack, cxx:_mj_isFreeBody, cxx:_mj_isSparse, cxx:_mj_jacGeom, cxx:_mj_jacSparse, cxx:_mj_markStack, cxx:_mj_objectVelocity, cxx:_mj_stackAllocInfo, cxx:_mju_copy, cxx:_mju_copy3, cxx:_mju_geomSemiAxes, cxx:_mju_mulMatTMat, cxx:_mju_subFrom3, cxx:_mju_symmetrize, cxx:_mju_transformSpatial, cxx:_mju_zero, cxx:_readFluidGeomInteraction
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjd_ellipsoid_fluid(m: *const mjModel, d: *mut mjData, res: *mut f64, bodyid: i32, flg_dragonly: i32) {
    todo!() // cxx:_mjd_ellipsoidFluid
}

/// C: mjd_inertiaBoxFluid (engine/engine_derivative.c:3035)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative.c:_addJTBJSparse, cxx-internal:engine_derivative.c.o:_addJTBJ, cxx:_mj_bodyChain, cxx:_mj_freeStack, cxx:_mj_isSparse, cxx:_mj_jacBodyCom, cxx:_mj_jacSparse, cxx:_mj_markStack, cxx:_mj_objectVelocity, cxx:_mj_stackAllocInfo, cxx:_mju_copy, cxx:_mju_copy3, cxx:_mju_max, cxx:_mju_mulMatTMat, cxx:_mju_subFrom3, cxx:_mju_transformSpatial, cxx:_mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjd_inertia_box_fluid(m: *const mjModel, d: *mut mjData, res: *mut f64, i: i32) {
    todo!() // cxx:_mjd_inertiaBoxFluid
}

/// C: mjd_passive_vel (engine/engine_derivative.c:3191)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative.c:_addJTBJSparse, cxx:_mj_actuatorDamping, cxx:_mjd_ellipsoidFluid, cxx:_mjd_inertiaBoxFluid, cxx:_mjd_xPolyForce, cxx:_mju_copy
#[allow(unused_variables, non_snake_case)]
pub fn mjd_passive_vel(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx:_mjd_passive_vel
}

/// C: mjd_smooth_vel (engine/engine_derivative.c:3297)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative.c:_mjd_rne_vel, cxx:_mjd_actuator_vel, cxx:_mjd_passive_vel, cxx:_mju_zero, cxx:_mju_zeroSparse
#[allow(unused_variables, non_snake_case)]
pub fn mjd_smooth_vel(m: *const mjModel, d: *mut mjData, flg_bias: i32) {
    todo!() // cxx:_mjd_smooth_vel
}

/// C: effAlloc (engine/engine_derivative.c:3328)
/// Calls: cxx:_mj_arenaAllocByte, cxx:_mju_message
#[allow(unused_variables, non_snake_case)]
pub fn eff_alloc(d: *mut mjData, bytes: usize, align: usize) -> *mut () {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative.c:_effAlloc
}

/// C: mjd_effRank1Next (engine/engine_derivative.c:3339)
#[allow(unused_variables, non_snake_case)]
pub fn mjd_eff_rank1next(m: *const mjModel, d: *const mjData, it: *mut mjEffRank1Iter, e: *mut mjEffRank1, flg_contact: i32) -> i32 {
    todo!() // cxx:_mjd_effRank1Next
}

/// C: mjd_effMulAdd (engine/engine_derivative.c:3410)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_sparse.h:_mju_dotSparse, cxx:_mjd_effRank1Next, cxx:_mjd_flexBend_mul, cxx:_mjd_flexInterpAssemblable, cxx:_mjd_flexInterp_mul
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjd_eff_mul_add(m: *const mjModel, d: *mut mjData, res: *mut f64, vec: *const f64, flg_contact: i32) {
    todo!() // cxx:_mjd_effMulAdd
}

/// C: effBlocks (engine/engine_derivative.c:3479)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative.c:_effAlloc, cxx:_mju_cholFactor, cxx:_mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn eff_blocks(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative.c:_effBlocks
}

/// C: effBlockApply (engine/engine_derivative.c:3525)
/// Calls: cxx:_mj_freeStack, cxx:_mj_markStack, cxx:_mj_solveLD, cxx:_mj_stackAllocInfo, cxx:_mju_cholSolve, cxx:_mju_cholSolveSparse, cxx:_mju_copy, cxx:_mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn eff_block_apply(m: *const mjModel, d: *mut mjData, x: *mut f64, b: *const f64) {
    todo!() // cxx-internal:engine_derivative.c.o:_effBlockApply
}

/// C: mjd_effSolve (engine/engine_derivative.c:3578)
/// Calls: cxx-internal:engine_derivative.c.o:_effBlockApply, cxx:_mj_effFlexPossible, cxx:_mj_freeStack, cxx:_mj_markStack, cxx:_mj_stackAllocInfo, cxx:_mj_warning, cxx:_mjd_effMulAdd, cxx:_mjd_effPrec, cxx:_mju_addScl, cxx:_mju_addToScl, cxx:_mju_copy, cxx:_mju_dot, cxx:_mju_mulSymVecSparse, cxx:_mju_warning, cxx:_mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjd_eff_solve(m: *const mjModel, d: *mut mjData, x: *mut f64, b: *const f64) {
    todo!() // cxx:_mjd_effSolve
}

/// C: mjd_effPrec (engine/engine_derivative.c:3660)
/// Calls: cxx-internal:engine_derivative.c.o:_effBlockApply, cxx:_mj_solveLD, cxx:_mju_copy
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjd_eff_prec(m: *const mjModel, d: *mut mjData, x: *mut f64, b: *const f64) {
    todo!() // cxx:_mjd_effPrec
}

/// C: dofDampPossible (engine/engine_derivative.c:3679)
/// Calls: cxx:_mju_isZero
#[allow(unused_variables, non_snake_case)]
pub fn dof_damp_possible(m: *const mjModel, i: i32) -> i32 {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative.c:_dofDampPossible
}

/// C: effDiagStiff (engine/engine_derivative.c:3691)
/// Calls: cxx:_mjd_xPolyForce, cxx:_mju_copy4, cxx:_mju_isZero, cxx:_mju_max, cxx:_mju_norm3, cxx:_mju_normalize4, cxx:_mju_sub3, cxx:_mju_subQuat, cxx:_mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn eff_diag_stiff(m: *const mjModel, d: *mut mjData) -> i32 {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative.c:_effDiagStiff
}

/// C: effDiagDamp (engine/engine_derivative.c:3757)
/// Calls: cxx:_mj_actuatorDamping, cxx:_mjd_xPolyForce, cxx:_mju_copy, cxx:_mju_max
#[allow(unused_variables, non_snake_case)]
pub fn eff_diag_damp(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative.c:_effDiagDamp
}

/// C: mjd_effShift (engine/engine_derivative.c:3781)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative.c:_effDiagDamp, cxx:_mj_actuatorDamping, cxx:_mj_freeStack, cxx:_mj_markStack, cxx:_mj_stackAllocInfo, cxx:_mjd_ellipsoidFluid, cxx:_mjd_flexBend_mul, cxx:_mjd_flexInterp_mul, cxx:_mjd_flexStretch_mul, cxx:_mjd_inertiaBoxFluid, cxx:_mjd_xPolyForce, cxx:_mju_copy, cxx:_mju_gather, cxx:_mju_max, cxx:_mju_scl, cxx:_mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mjd_eff_shift(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx:_mjd_effShift
}

/// C: mjd_effActuation (engine/engine_derivative.c:3902)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative.c:_actuatorDerivSkip, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative.c:_actuatorLenDeriv, cxx-internal:engine_derivative.c.o:_actuatorVelDeriv, cxx:_mj_effActuatorPossible, cxx:_mj_factorI, cxx:_mju_addTo, cxx:_mju_copy, cxx:_mju_max, cxx:_mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mjd_eff_actuation(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx:_mjd_effActuation
}

/// C: mjd_effMulAddIsland (engine/engine_derivative.c:3994)
/// Calls: cxx:_mjd_effRank1Next
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjd_eff_mul_add_island(m: *const mjModel, d: *const mjData, res: *mut f64, vec: *const f64, island: i32) {
    todo!() // cxx:_mjd_effMulAddIsland
}

/// C: effContactBuild (engine/engine_derivative.c:4051)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative.c:_effAlloc, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative.c:_flexPassiveContact_any, cxx:_mj_contactJacobian, cxx:_mj_freeStack, cxx:_mj_markStack, cxx:_mj_stackAllocInfo, cxx:_mjd_flexContactStiffness, cxx:_mju_mulMatMat
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn eff_contact_build(m: *const mjModel, d: *mut mjData, scale: f64) {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative.c:_effContactBuild
}

/// C: mjd_effBuild (engine/engine_derivative.c:4113)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative.c:_dofDampPossible, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative.c:_effAlloc, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative.c:_effBlocks, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative.c:_effContactBuild, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative.c:_effDiagStiff, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative.c:_flexPassiveContact_any, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_derivative.c:_flexStiff_active, cxx:_mj_effActuatorPossible, cxx:_mj_effCouplings, cxx:_mj_effTendonPossible, cxx:_mj_sleepState, cxx:_mjd_flexInterpAssemblable, cxx:_mjd_flexInterp_cacheKrot, cxx:_mjd_flexStiff_any, cxx:_mjd_flexStiff_assemble, cxx:_mju_zero, cxx:_mju_zeroInt
#[allow(unused_variables, non_snake_case)]
pub fn mjd_eff_build(m: *const mjModel, d: *mut mjData, active: i32, flg_factor: i32) {
    todo!() // cxx:_mjd_effBuild
}

