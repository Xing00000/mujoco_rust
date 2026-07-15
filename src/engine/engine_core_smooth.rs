//! Port of: engine/engine_core_smooth.c
//! IR hash: d2209344472ae336
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mj_updateDynamicBVH (engine/engine_core_smooth.c:490)
/// Calls: mj_freeStack, mj_markStack, mj_stackAllocInfo, mju_max, mju_min, mju_zeroInt
#[allow(unused_variables, non_snake_case)]
pub fn mj_update_dynamic_bvh(m: *const mjModel, d: *mut mjData, bvhadr: i32, bvhnum: i32) {
    todo!() // mj_updateDynamicBVH
}

/// C: mju_mulMatMat322 (engine/engine_core_smooth.c:537)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_mul_mat_mat322(C: *mut f64, A: *const f64, B: *const f64) {
    // SAFETY: caller guarantees C[6], A[6], B[4] are valid
    unsafe {
        *C.add(0) = *A.add(0) * *B.add(0) + *A.add(1) * *B.add(2);
        *C.add(1) = *A.add(0) * *B.add(1) + *A.add(1) * *B.add(3);
        *C.add(2) = *A.add(2) * *B.add(0) + *A.add(3) * *B.add(2);
        *C.add(3) = *A.add(2) * *B.add(1) + *A.add(3) * *B.add(3);
        *C.add(4) = *A.add(4) * *B.add(0) + *A.add(5) * *B.add(2);
        *C.add(5) = *A.add(4) * *B.add(1) + *A.add(5) * *B.add(3);
    }
}

/// C: mj_kinematics1 (engine/engine_core_smooth.h:29)
/// Calls: mji_addTo3, mji_addToScl3, mji_axisAngle2Quat, mji_copy3, mji_copy4, mji_mulMatVec3, mji_mulQuat, mji_rotVecQuat, mji_sub3, mju_message, mju_mulQuat, mju_normalize4, mju_quat2Mat, mju_unit4, mju_zero, mju_zero3
#[allow(unused_variables, non_snake_case)]
pub fn mj_kinematics1(m: *const mjModel, d: *mut mjData) {
    todo!() // mj_kinematics1
}

/// C: mj_kinematics2 (engine/engine_core_smooth.h:32)
/// Calls: mj_local2Global
#[allow(unused_variables, non_snake_case)]
pub fn mj_kinematics2(m: *const mjModel, d: *mut mjData) {
    todo!() // mj_kinematics2
}

/// C: mj_kinematics (engine/engine_core_smooth.h:35)
/// Calls: mj_kinematics1, mj_kinematics2, mj_updateSleep, mj_wake
#[allow(unused_variables, non_snake_case)]
pub fn mj_kinematics(m: *const mjModel, d: *mut mjData) {
    todo!() // mj_kinematics
}

/// C: mj_comPos (engine/engine_core_smooth.h:38)
/// Calls: mji_addTo3, mji_copy3, mji_scl3, mji_sub3, mju_dofCom, mju_inertCom, mju_scl3, mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mj_com_pos(m: *const mjModel, d: *mut mjData) {
    todo!() // mj_comPos
}

/// C: mj_camlight (engine/engine_core_smooth.h:41)
/// Calls: mj_local2Global, mji_add3, mji_copy3, mji_copy9, mji_cross, mji_rotVecQuat, mji_sub3, mju_normalize3, mju_transpose
#[allow(unused_variables, non_snake_case)]
pub fn mj_camlight(m: *const mjModel, d: *mut mjData) {
    todo!() // mj_camlight
}

/// C: mj_flex (engine/engine_core_smooth.h:44)
/// Calls: mj_bodyChain, mj_freeStack, mj_jacDifPair, mj_jacSparse, mj_markStack, mj_stackAllocInfo, mj_updateDynamicBVH, mji_addTo3, mji_copy3, mji_copy6, mji_mulMatVec3, mji_sub3, mju_cellLookup, mju_interpolate3D, mju_max, mju_message, mju_min, mju_mulMatMat322, mju_mulMatTVec, mju_mulMatVec, mju_normalize3, mju_scl3, mju_shellTrackInterior, mju_sub3, mju_zero, mju_zero3
#[allow(unused_variables, non_snake_case)]
pub fn mj_flex(m: *const mjModel, d: *mut mjData) {
    todo!() // mj_flex
}

/// C: mj_tendon (engine/engine_core_smooth.h:47)
/// Calls: mj_freeStack, mj_jacDifPair, mj_markStack, mj_sleepState, mj_stackAllocInfo, mji_copy3, mji_copy9, mji_sub3, mju_combineSparseInc, mju_dist3, mju_message, mju_mulMatTVec, mju_normalize3, mju_round, mju_wrap, mju_zero, mju_zero3
#[allow(unused_variables, non_snake_case)]
pub fn mj_tendon(m: *const mjModel, d: *mut mjData) {
    todo!() // mj_tendon
}

/// C: mj_tendonDot (engine/engine_core_smooth.h:50)
/// Calls: mj_freeStack, mj_isSparse, mj_jac, mj_jacDot, mj_jacDotSparse, mj_jacSparse, mj_markStack, mj_mergeChain, mj_objectVelocity, mj_stackAllocInfo, mji_copy3, mju_addToScl3, mju_dot, mju_dot3, mju_message, mju_mulMatTVec, mju_normalize3, mju_scl3, mju_sub, mju_sub3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_tendon_dot(m: *const mjModel, d: *mut mjData, id: i32, vec: *const f64) -> f64 {
    todo!() // mj_tendonDot
}

/// C: mj_transmission (engine/engine_core_smooth.h:53)
/// Calls: mj_freeStack, mj_isSparse, mj_jacDifPair, mj_jacPointAxis, mj_jacSite, mj_markStack, mj_mulJacTVec, mj_sleepState, mj_stackAllocInfo, mji_addTo3, mji_copy3, mji_copy4, mji_mulMatVec3, mji_mulQuat, mji_quat2Vel, mji_rotVecQuat, mji_subQuat, mju_addTo, mju_copyInt, mju_dot3, mju_isZero, mju_message, mju_mulMatMat, mju_mulMatTVec, mju_mulMatTVec3, mju_negQuat, mju_normalize4, mju_scl, mju_scl3, mju_sub3, mju_subFrom, mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mj_transmission(m: *const mjModel, d: *mut mjData) {
    todo!() // mj_transmission
}

/// C: mj_crb (engine/engine_core_smooth.h:59)
/// Calls: mj_actuatorArmature, mji_dot6, mju_addTo, mju_copy, mju_copyRows, mju_mulInertVec, mju_zero, mju_zeroSparse
#[allow(unused_variables, non_snake_case)]
pub fn mj_crb(m: *const mjModel, d: *mut mjData) {
    todo!() // mj_crb
}

/// C: mj_tendonArmature (engine/engine_core_smooth.h:62)
/// Calls: mj_actuatorArmature, mj_sleepState, mju_addToSclSparseInc
#[allow(unused_variables, non_snake_case)]
pub fn mj_tendon_armature(m: *const mjModel, d: *mut mjData) {
    todo!() // mj_tendonArmature
}

/// C: mj_makeM (engine/engine_core_smooth.h:65)
/// Calls: mj_crb, mj_tendonArmature, mju_scatter
#[allow(unused_variables, non_snake_case)]
pub fn mj_make_m(m: *const mjModel, d: *mut mjData) {
    todo!() // mj_makeM
}

/// C: mj_factorI_legacy (engine/engine_core_smooth.h:68)
/// Calls: mj_warning, mju_addToScl, mju_copy
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_factor_i_legacy(m: *const mjModel, d: *mut mjData, M: *const f64, qLD: *mut f64, qLDiagInv: *mut f64) {
    todo!() // mj_factorI_legacy
}

/// C: mj_factorI (engine/engine_core_smooth.h:72)
/// Calls: mju_addToScl, mju_scl
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_factor_i(mat: *mut f64, diaginv: *mut f64, nv: i32, rownnz: *const i32, rowadr: *const i32, colind: *const i32, index: *const i32) {
    // SAFETY: caller guarantees all pointers are valid for the sparse matrix layout.
    // index may be null (optional permutation). diaginv may be null.
    unsafe {
        let mut j: i32 = nv - 1;
        while j >= 0 {
            let k: i32 = if !index.is_null() { *index.add(j as usize) } else { j };
            let start: i32 = *rowadr.add(k as usize);
            let diag: i32 = *rownnz.add(k as usize) - 1;
            let end: i32 = start + diag;
            let invD: f64 = 1.0 / *mat.add(end as usize);
            if !diaginv.is_null() {
                *diaginv.add(k as usize) = invD;
            }
            let mut adr: i32 = end - 1;
            while adr >= start {
                let i: i32 = *colind.add(adr as usize);
                crate::engine::engine_util_blas::mju_add_to_scl(
                    mat.add(*rowadr.add(i as usize) as usize),
                    mat.add(start as usize) as *const f64,
                    -*mat.add(adr as usize) * invD,
                    *rownnz.add(i as usize),
                );
                adr -= 1;
            }
            crate::engine::engine_util_blas::mju_scl(
                mat.add(start as usize),
                mat.add(start as usize) as *const f64,
                invD,
                diag,
            );
            j -= 1;
        }
    }
}

/// C: mj_factorM (engine/engine_core_smooth.h:76)
/// Calls: mj_factorI, mju_copy, mju_copySparse
#[allow(unused_variables, non_snake_case)]
pub fn mj_factor_m(m: *const mjModel, d: *mut mjData) {
    // SAFETY: m and d are valid pointers from caller; all field accesses are within bounds
    unsafe {
        const mjENBL_SLEEP: i32 = 1 << 4;

        // sleep filtering
        let sleep_filter = ((*m).opt.enableflags & mjENBL_SLEEP) != 0
            && (*d).nv_awake < (*m).nv as i32;

        let index: *const i32;
        let nv: i32;

        // no sleep filtering: copy everything
        if sleep_filter == false {
            index = std::ptr::null();
            nv = (*m).nv as i32;
            crate::engine::engine_util_blas::mju_copy(
                (*d).qLD, (*d).M, (*m).nC as i32,
            );
        } else {
            // sleep filtering: copy only awake dofs
            index = (*d).dof_awake_ind;
            nv = (*d).nv_awake;
            crate::engine::engine_util_sparse::mju_copy_sparse(
                (*d).qLD, (*d).M,
                (*m).M_rownnz, (*m).M_rowadr,
                (*d).dof_awake_ind, (*d).nv_awake,
            );
        }

        // factorize
        mj_factor_i(
            (*d).qLD, (*d).qLDiagInv, nv,
            (*m).M_rownnz, (*m).M_rowadr, (*m).M_colind, index,
        );
    }
}

/// C: mj_solveLD_legacy (engine/engine_core_smooth.h:79)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_solve_ld_legacy(m: *const mjModel, x: *mut f64, n: i32, qLD: *const f64, qLDiagInv: *const f64) {
    // SAFETY: caller guarantees m is valid, x has n*nv elements,
    // qLD and qLDiagInv are valid for the model's sparse structure.
    unsafe {
        let dof_Madr = (*m).dof_Madr;
        let dof_parentid = (*m).dof_parentid;
        let nv = (*m).nv as i32;

        if n == 1 {
            // x <- inv(L') * x
            let mut i: i32 = nv - 1;
            while i >= 0 {
                if *(*m).dof_simplenum.add(i as usize) == 0 && *x.add(i as usize) != 0.0 {
                    let mut Madr_ij: i32 = *dof_Madr.add(i as usize) + 1;
                    let mut j: i32 = *dof_parentid.add(i as usize);
                    while j >= 0 {
                        *x.add(j as usize) -= *qLD.add(Madr_ij as usize) * *x.add(i as usize);
                        Madr_ij += 1;
                        j = *dof_parentid.add(j as usize);
                    }
                }
                i -= 1;
            }

            // x <- inv(D) * x
            let mut i: i32 = 0;
            while i < nv {
                *x.add(i as usize) *= *qLDiagInv.add(i as usize);
                i += 1;
            }

            // x <- inv(L) * x
            let mut i: i32 = 0;
            while i < nv {
                if *(*m).dof_simplenum.add(i as usize) == 0 {
                    let mut Madr_ij: i32 = *dof_Madr.add(i as usize) + 1;
                    let mut j: i32 = *dof_parentid.add(i as usize);
                    while j >= 0 {
                        *x.add(i as usize) -= *qLD.add(Madr_ij as usize) * *x.add(j as usize);
                        Madr_ij += 1;
                        j = *dof_parentid.add(j as usize);
                    }
                }
                i += 1;
            }
        } else {
            let mut offset: i32;
            let mut tmp: f64;

            // x <- inv(L') * x
            let mut i: i32 = nv - 1;
            while i >= 0 {
                if *(*m).dof_simplenum.add(i as usize) == 0 {
                    let mut Madr_ij: i32 = *dof_Madr.add(i as usize) + 1;
                    let mut j: i32 = *dof_parentid.add(i as usize);
                    while j >= 0 {
                        offset = 0;
                        while offset < n * nv {
                            tmp = *x.add((i + offset) as usize);
                            if tmp != 0.0 {
                                *x.add((j + offset) as usize) -= *qLD.add(Madr_ij as usize) * tmp;
                            }
                            offset += nv;
                        }
                        Madr_ij += 1;
                        j = *dof_parentid.add(j as usize);
                    }
                }
                i -= 1;
            }

            // x <- inv(D) * x
            let mut i: i32 = 0;
            while i < nv {
                offset = 0;
                while offset < n * nv {
                    *x.add((i + offset) as usize) *= *qLDiagInv.add(i as usize);
                    offset += nv;
                }
                i += 1;
            }

            // x <- inv(L) * x
            let mut i: i32 = 0;
            while i < nv {
                if *(*m).dof_simplenum.add(i as usize) == 0 {
                    let mut Madr_ij: i32 = *dof_Madr.add(i as usize) + 1;
                    let mut j: i32 = *dof_parentid.add(i as usize);
                    while j >= 0 {
                        offset = 0;
                        while offset < n * nv {
                            *x.add((i + offset) as usize) -= *qLD.add(Madr_ij as usize) * *x.add((j + offset) as usize);
                            offset += nv;
                        }
                        Madr_ij += 1;
                        j = *dof_parentid.add(j as usize);
                    }
                }
                i += 1;
            }
        }
    }
}

/// C: mj_solveLD (engine/engine_core_smooth.h:84)
/// Calls: mju_dotSparse
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_solve_ld(x: *mut f64, qLD: *const f64, qLDiagInv: *const f64, nv: i32, n: i32, rownnz: *const i32, rowadr: *const i32, colind: *const i32, index: *const i32) {
    todo!() // mj_solveLD
}

/// C: mj_solveM (engine/engine_core_smooth.h:88)
/// Calls: mj_solveLD, mju_copy
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_solve_m(m: *const mjModel, d: *mut mjData, x: *mut f64, y: *const f64, n: i32) {
    todo!() // mj_solveM
}

/// C: mj_solveM2 (engine/engine_core_smooth.h:91)
/// Calls: mju_copy
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_solve_m2(m: *const mjModel, d: *mut mjData, x: *mut f64, y: *const f64, sqrtInvD: *const f64, n: i32) {
    // SAFETY: caller guarantees m, d are valid structs, x/y have n*nv elements,
    // sqrtInvD has nv elements, and the sparse matrix fields are valid.
    unsafe {
        let nv = (*m).nv as i32;
        let rownnz = (*m).M_rownnz;
        let rowadr = (*m).M_rowadr;
        let colind = (*m).M_colind;
        let diagnum = (*m).dof_simplenum;
        let qLD = (*d).qLD as *const f64;

        crate::engine::engine_util_blas::mju_copy(x, y, n * nv);

        // x <- L^-T x
        let mut i: i32 = nv - 1;
        while i > 0 {
            if *diagnum.add(i as usize) != 0 {
                i -= 1;
                continue;
            }
            let start: i32 = *rowadr.add(i as usize);
            let end: i32 = start + *rownnz.add(i as usize) - 1;
            let mut offset: i32 = 0;
            while offset < n * nv {
                let x_i: f64 = *x.add((i + offset) as usize);
                if x_i != 0.0 {
                    let mut adr: i32 = start;
                    while adr < end {
                        *x.add((offset + *colind.add(adr as usize)) as usize) -= *qLD.add(adr as usize) * x_i;
                        adr += 1;
                    }
                }
                offset += nv;
            }
            i -= 1;
        }

        // x <- D^-1/2 x
        let mut i: i32 = 0;
        while i < nv {
            let invD_i: f64 = *sqrtInvD.add(i as usize);
            let mut offset: i32 = 0;
            while offset < n * nv {
                *x.add((i + offset) as usize) *= invD_i;
                offset += nv;
            }
            i += 1;
        }
    }
}

/// C: mj_comVel (engine/engine_core_smooth.h:98)
/// Calls: mji_copy6, mji_crossMotion, mju_addTo, mju_copy, mju_mulDofVec, mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mj_com_vel(m: *const mjModel, d: *mut mjData) {
    todo!() // mj_comVel
}

/// C: mj_subtreeVel (engine/engine_core_smooth.h:101)
/// Calls: mj_freeStack, mj_markStack, mj_objectVelocity, mj_stackAllocInfo, mji_addTo3, mji_cross, mji_mulMatVec3, mju_max, mju_mulMatTVec3, mju_scl3, mju_sub3
#[allow(unused_variables, non_snake_case)]
pub fn mj_subtree_vel(m: *const mjModel, d: *mut mjData) {
    todo!() // mj_subtreeVel
}

/// C: mj_rne (engine/engine_core_smooth.h:107)
/// Calls: mj_freeStack, mj_markStack, mj_stackAllocInfo, mji_crossForce, mji_dot6, mju_add, mju_addTo, mju_mulDofVec, mju_mulInertVec, mju_scl3, mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_rne(m: *const mjModel, d: *mut mjData, flg_acc: i32, result: *mut f64) {
    todo!() // mj_rne
}

/// C: mj_rnePostConstraint (engine/engine_core_smooth.h:110)
/// Calls: mj_contactForce, mj_local2Global, mji_copy3, mji_crossForce, mju_add, mju_addTo, mju_isZero, mju_message, mju_mulDofVec, mju_mulInertVec, mju_mulMatTVec3, mju_scl3, mju_sub, mju_subFrom, mju_transformSpatial, mju_zero, mju_zero3
#[allow(unused_variables, non_snake_case)]
pub fn mj_rne_post_constraint(m: *const mjModel, d: *mut mjData) {
    todo!() // mj_rnePostConstraint
}

/// C: mj_tendonBias (engine/engine_core_smooth.h:116)
/// Calls: mj_actuatorArmature, mj_sleepState, mj_tendonDot
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_tendon_bias(m: *const mjModel, d: *mut mjData, qfrc: *mut f64) {
    todo!() // mj_tendonBias
}

