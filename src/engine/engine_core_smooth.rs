//! Port of: engine/engine_core_smooth.c
//! IR hash: 05737965add36adb
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mj_updateDynamicBVH (engine/engine_core_smooth.c:490)
/// Calls: mj_freeStack, mj_markStack, mj_stackAllocInfo, mju_max, mju_min, mju_zeroInt
#[allow(unused_variables, non_snake_case)]
pub fn mj_update_dynamic_bvh(m: *const mjModel, d: *mut mjData, bvhadr: i32, bvhnum: i32) {
    extern "C" {
        fn mj_updateDynamicBVH_impl(m: *const mjModel, d: *mut mjData, bvhadr: i32, bvhnum: i32);
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_updateDynamicBVH_impl(m, d, bvhadr, bvhnum) }
}

/// C: mju_mulMatMat322 (engine/engine_core_smooth.c:537)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_mul_mat_mat322(C: *mut f64, A: *const f64, B: *const f64) {
    // SAFETY: C points to 6 f64, A points to 6 f64, B points to 4 f64.
    // Direct translation of C(3x2) = A(3x2) * B(2x2).
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
    extern "C" {
        fn mj_kinematics1_impl(m: *const mjModel, d: *mut mjData);
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_kinematics1_impl(m, d) }
}

/// C: mj_kinematics2 (engine/engine_core_smooth.h:32)
/// Calls: mj_local2Global
#[allow(unused_variables, non_snake_case)]
pub fn mj_kinematics2(m: *const mjModel, d: *mut mjData) {
    extern "C" {
        fn mj_kinematics2_impl(m: *const mjModel, d: *mut mjData);
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_kinematics2_impl(m, d) }
}

/// C: mj_kinematics (engine/engine_core_smooth.h:35)
/// Calls: mj_kinematics1, mj_kinematics2, mj_updateSleep, mj_wake
#[allow(unused_variables, non_snake_case)]
pub fn mj_kinematics(m: *const mjModel, d: *mut mjData) {
    extern "C" { fn mj_kinematics_impl(m: *const mjModel, d: *mut mjData); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_kinematics_impl(m, d) }
}

/// C: mj_comPos (engine/engine_core_smooth.h:38)
/// Calls: mji_addTo3, mji_copy3, mji_scl3, mji_sub3, mju_dofCom, mju_inertCom, mju_scl3, mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mj_com_pos(m: *const mjModel, d: *mut mjData) {
    // SAFETY: m, d valid per caller. All pointer arithmetic stays within allocated arrays.
    unsafe {
        const MJMINVAL: f64 = 1e-15;
        const MJENBL_SLEEP: i32 = 1 << 4;
        const MJS_ASLEEP: i32 = 0;
        const MJJNT_FREE: i32 = 0;
        const MJJNT_BALL: i32 = 1;
        const MJJNT_SLIDE: i32 = 2;
        const MJJNT_HINGE: i32 = 3;

        let sleep_filter: i32 = (((*m).opt.enableflags & MJENBL_SLEEP) != 0 && (*d).nbody_awake < (*m).nbody as i32) as i32;
        let nbody: i32 = if sleep_filter != 0 { (*d).nbody_awake } else { (*m).nbody as i32 };
        let nparent: i32 = if sleep_filter != 0 { (*d).nparent_awake } else { (*m).nbody as i32 };

        // subtree_com: initialize with body moment
        let mut b: i32 = 0;
        while b < nbody {
            let i = if sleep_filter != 0 { *(*d).body_awake_ind.add(b as usize) } else { b };
            crate::engine::engine_inline::mji_scl3(
                (*d).subtree_com.add(3 * i as usize),
                (*d).xipos.add(3 * i as usize),
                *(*m).body_mass.add(i as usize),
            );
            b += 1;
        }

        // subtree_com: accumulate to parent in backward pass
        b = nparent - 1;
        while b >= 0 {
            let i = if sleep_filter != 0 { *(*d).parent_awake_ind.add(b as usize) } else { b };
            if i == 0 {
                b -= 1;
                continue;
            }

            // accumulate moment to parent, rescale if sleeping
            let parent = *(*m).body_parentid.add(i as usize);
            if sleep_filter != 0 && *(*d).body_awake.add(i as usize) == MJS_ASLEEP {
                let mut child_moment: [f64; 3] = [0.0; 3];
                crate::engine::engine_inline::mji_scl3(
                    child_moment.as_mut_ptr(),
                    (*d).subtree_com.add(3 * i as usize),
                    *(*m).body_subtreemass.add(i as usize),
                );
                crate::engine::engine_inline::mji_add_to3(
                    (*d).subtree_com.add(3 * parent as usize),
                    child_moment.as_ptr(),
                );
            } else {
                crate::engine::engine_inline::mji_add_to3(
                    (*d).subtree_com.add(3 * parent as usize),
                    (*d).subtree_com.add(3 * i as usize),
                );
            }
            b -= 1;
        }

        // subtree_com: normalize
        b = 0;
        while b < nbody {
            let i = if sleep_filter != 0 { *(*d).body_awake_ind.add(b as usize) } else { b };
            if *(*m).body_subtreemass.add(i as usize) < MJMINVAL {
                crate::engine::engine_inline::mji_copy3(
                    (*d).subtree_com.add(3 * i as usize),
                    (*d).xipos.add(3 * i as usize),
                );
            } else {
                crate::engine::engine_util_blas::mju_scl3(
                    (*d).subtree_com.add(3 * i as usize),
                    (*d).subtree_com.add(3 * i as usize),
                    1.0 / *(*m).body_subtreemass.add(i as usize),
                );
            }
            b += 1;
        }

        // zero out CoM frame inertia for the world body
        crate::engine::engine_util_blas::mju_zero((*d).cinert, 10);

        // map inertias to frame centered at subtree_com
        b = 1;
        while b < nbody {
            let i = if sleep_filter != 0 { *(*d).body_awake_ind.add(b as usize) } else { b };
            let mut offset: [f64; 3] = [0.0; 3];
            crate::engine::engine_inline::mji_sub3(
                offset.as_mut_ptr(),
                (*d).xipos.add(3 * i as usize),
                (*d).subtree_com.add(3 * *(*m).body_rootid.add(i as usize) as usize),
            );
            crate::engine::engine_util_spatial::mju_inert_com(
                (*d).cinert.add(10 * i as usize),
                (*m).body_inertia.add(3 * i as usize),
                (*d).ximat.add(9 * i as usize),
                offset.as_ptr(),
                *(*m).body_mass.add(i as usize),
            );
            b += 1;
        }

        // map motion dofs to global frame centered at subtree_com
        b = 1;
        while b < nbody {
            let i = if sleep_filter != 0 { *(*d).body_awake_ind.add(b as usize) } else { b };

            let jntnum = *(*m).body_jntnum.add(i as usize);
            if jntnum == 0 {
                b += 1;
                continue;
            }

            let start = *(*m).body_jntadr.add(i as usize);
            let end = start + jntnum;
            let mut j = start;
            while j < end {
                // get cdof address
                let da = 6 * *(*m).jnt_dofadr.add(j as usize);

                // compute com-anchor vector
                let mut offset: [f64; 3] = [0.0; 3];
                let mut axis: [f64; 3] = [0.0; 3];
                crate::engine::engine_inline::mji_sub3(
                    offset.as_mut_ptr(),
                    (*d).subtree_com.add(3 * *(*m).body_rootid.add(i as usize) as usize),
                    (*d).xanchor.add(3 * j as usize),
                );

                // create motion dof
                let jnt_type = *(*m).jnt_type.add(j as usize);
                if jnt_type == MJJNT_FREE {
                    // translation components: x, y, z in global frame
                    crate::engine::engine_util_blas::mju_zero((*d).cdof.add(da as usize), 18);
                    *(*d).cdof.add((da + 3 + 7 * 0) as usize) = 1.0;
                    *(*d).cdof.add((da + 3 + 7 * 1) as usize) = 1.0;
                    *(*d).cdof.add((da + 3 + 7 * 2) as usize) = 1.0;

                    // rotation components: same as ball (fallthrough)
                    let mut k: i32 = 0;
                    while k < 3 {
                        axis[0] = *(*d).xmat.add((9 * i + k + 0) as usize);
                        axis[1] = *(*d).xmat.add((9 * i + k + 3) as usize);
                        axis[2] = *(*d).xmat.add((9 * i + k + 6) as usize);
                        crate::engine::engine_util_spatial::mju_dof_com(
                            (*d).cdof.add((da + 18 + 6 * k) as usize),
                            axis.as_ptr(),
                            offset.as_ptr(),
                        );
                        k += 1;
                    }
                } else if jnt_type == MJJNT_BALL {
                    let mut k: i32 = 0;
                    while k < 3 {
                        axis[0] = *(*d).xmat.add((9 * i + k + 0) as usize);
                        axis[1] = *(*d).xmat.add((9 * i + k + 3) as usize);
                        axis[2] = *(*d).xmat.add((9 * i + k + 6) as usize);
                        crate::engine::engine_util_spatial::mju_dof_com(
                            (*d).cdof.add((da + 6 * k) as usize),
                            axis.as_ptr(),
                            offset.as_ptr(),
                        );
                        k += 1;
                    }
                } else if jnt_type == MJJNT_SLIDE {
                    crate::engine::engine_util_spatial::mju_dof_com(
                        (*d).cdof.add(da as usize),
                        (*d).xaxis.add(3 * j as usize),
                        core::ptr::null(),
                    );
                } else if jnt_type == MJJNT_HINGE {
                    crate::engine::engine_util_spatial::mju_dof_com(
                        (*d).cdof.add(da as usize),
                        (*d).xaxis.add(3 * j as usize),
                        offset.as_ptr(),
                    );
                }

                j += 1;
            }
            b += 1;
        }
    }
}

/// C: mj_camlight (engine/engine_core_smooth.h:41)
/// Calls: mj_local2Global, mji_add3, mji_copy3, mji_copy9, mji_cross, mji_rotVecQuat, mji_sub3, mju_normalize3, mju_transpose
#[allow(unused_variables, non_snake_case)]
pub fn mj_camlight(m: *const mjModel, d: *mut mjData) {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData)
    // Previous return: ()
    extern "C" { fn mj_camlight_impl (m : * const mjModel , d : * mut mjData) ; } unsafe { mj_camlight_impl (m , d) }
}

/// C: mj_flex (engine/engine_core_smooth.h:44)
/// Calls: mj_bodyChain, mj_freeStack, mj_jacDifPair, mj_jacSparse, mj_markStack, mj_stackAllocInfo, mj_updateDynamicBVH, mji_addTo3, mji_copy3, mji_copy6, mji_mulMatVec3, mji_sub3, mju_cellLookup, mju_interpolate3D, mju_max, mju_message, mju_min, mju_mulMatMat322, mju_mulMatTVec, mju_mulMatVec, mju_normalize3, mju_scl3, mju_shellTrackInterior, mju_sub3, mju_zero, mju_zero3
#[allow(unused_variables, non_snake_case)]
pub fn mj_flex(m: *const mjModel, d: *mut mjData) {
    extern "C" {
        fn mj_flex_impl(m: *const mjModel, d: *mut mjData);
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_flex_impl(m, d) }
}

/// C: mj_tendon (engine/engine_core_smooth.h:47)
/// Calls: mj_freeStack, mj_jacDifPair, mj_markStack, mj_sleepState, mj_stackAllocInfo, mji_copy3, mji_copy9, mji_sub3, mju_combineSparseInc, mju_dist3, mju_message, mju_mulMatTVec, mju_normalize3, mju_round, mju_wrap, mju_zero, mju_zero3
#[allow(unused_variables, non_snake_case)]
pub fn mj_tendon(m: *const mjModel, d: *mut mjData) {
    extern "C" { fn mj_tendon_impl(m: *const mjModel, d: *mut mjData); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_tendon_impl(m, d) }
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
    extern "C" { fn mj_tendonDot_impl(m: *const mjModel, d: *mut mjData, id: i32, vec: *const f64) -> f64; }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_tendonDot_impl(m, d, id, vec) }
}

/// C: mj_transmission (engine/engine_core_smooth.h:53)
/// Calls: mj_freeStack, mj_isSparse, mj_jacDifPair, mj_jacPointAxis, mj_jacSite, mj_markStack, mj_mulJacTVec, mj_sleepState, mj_stackAllocInfo, mji_addTo3, mji_copy3, mji_copy4, mji_mulMatVec3, mji_mulQuat, mji_quat2Vel, mji_rotVecQuat, mji_subQuat, mju_addTo, mju_copyInt, mju_dot3, mju_isZero, mju_message, mju_mulMatMat, mju_mulMatTVec, mju_mulMatTVec3, mju_negQuat, mju_normalize4, mju_scl, mju_scl3, mju_sub3, mju_subFrom, mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mj_transmission(m: *const mjModel, d: *mut mjData) {
    extern "C" { fn mj_transmission_impl(m: *const mjModel, d: *mut mjData); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_transmission_impl(m, d) }
}

/// C: mj_crb (engine/engine_core_smooth.h:59)
/// Calls: mj_actuatorArmature, mji_dot6, mju_addTo, mju_copy, mju_copyRows, mju_mulInertVec, mju_zero, mju_zeroSparse
#[allow(unused_variables, non_snake_case)]
pub fn mj_crb(m: *const mjModel, d: *mut mjData) {
    extern "C" {
        fn mj_crb_impl(m: *const mjModel, d: *mut mjData);
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_crb_impl(m, d) }
}

/// C: mj_tendonArmature (engine/engine_core_smooth.h:62)
/// Calls: mj_actuatorArmature, mj_sleepState, mju_addToSclSparseInc
#[allow(unused_variables, non_snake_case)]
pub fn mj_tendon_armature(m: *const mjModel, d: *mut mjData) {
    extern "C" { fn mj_tendonArmature_impl(m: *const mjModel, d: *mut mjData); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_tendonArmature_impl(m, d) }
}

/// C: mj_makeM (engine/engine_core_smooth.h:65)
/// Calls: mj_crb, mj_tendonArmature, mju_scatter
#[allow(unused_variables, non_snake_case)]
pub fn mj_make_m(m: *const mjModel, d: *mut mjData) {
    extern "C" { fn mj_makeM_impl(m: *const mjModel, d: *mut mjData); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_makeM_impl(m, d) }
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
    extern "C" {
        fn mj_factorI_legacy_impl(m: *const mjModel, d: *mut mjData, M: *const f64, qLD: *mut f64, qLDiagInv: *mut f64);
    }
    // SAFETY: delegates to C implementation which accesses mjModel sparse matrix layout
    unsafe { mj_factorI_legacy_impl(m, d, M, qLD, qLDiagInv) }
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
    extern "C" {
        fn mj_factorI_impl(mat: *mut f64, diaginv: *mut f64, nv: i32, rownnz: *const i32, rowadr: *const i32, colind: *const i32, index: *const i32);
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_factorI_impl(mat, diaginv, nv, rownnz, rowadr, colind, index) }
}

/// C: mj_factorM (engine/engine_core_smooth.h:76)
/// Calls: mj_factorI, mju_copy, mju_copySparse
#[allow(unused_variables, non_snake_case)]
pub fn mj_factor_m(m: *const mjModel, d: *mut mjData) {
    extern "C" {
        fn mj_factorM_impl(m: *const mjModel, d: *mut mjData);
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_factorM_impl(m, d) }
}

/// C: mj_solveLD_legacy (engine/engine_core_smooth.h:79)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_solve_ld_legacy(m: *const mjModel, x: *mut f64, n: i32, qLD: *const f64, qLDiagInv: *const f64) {
    extern "C" {
        fn mj_solveLD_legacy_impl(m: *const mjModel, x: *mut f64, n: i32, qLD: *const f64, qLDiagInv: *const f64);
    }
    // SAFETY: delegates to C implementation which accesses mjModel sparse matrix layout
    unsafe { mj_solveLD_legacy_impl(m, x, n, qLD, qLDiagInv) }
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
    // SAFETY: all pointers valid per caller contract; index may be null
    unsafe {
        // x <- L^-T x
        let mut k: i32 = nv - 1;
        while k >= 0 {
            let i = if !index.is_null() { *index.add(k as usize) } else { k };
            if *rownnz.add(i as usize) == 1 {
                k -= 1;
                continue;
            }
            if n == 1 {
                let x_i = *x.add(i as usize);
                if x_i != 0.0 {
                    let start = *rowadr.add(i as usize);
                    let end = start + *rownnz.add(i as usize) - 1;
                    let mut adr = start;
                    while adr < end {
                        *x.add(*colind.add(adr as usize) as usize) -= *qLD.add(adr as usize) * x_i;
                        adr += 1;
                    }
                }
            } else {
                let start = *rowadr.add(i as usize);
                let end = start + *rownnz.add(i as usize) - 1;
                let mut offset: i32 = 0;
                while offset < n * nv {
                    let x_i = *x.add((i + offset) as usize);
                    if x_i != 0.0 {
                        let mut adr = start;
                        while adr < end {
                            *x.add((offset + *colind.add(adr as usize)) as usize) -= *qLD.add(adr as usize) * x_i;
                            adr += 1;
                        }
                    }
                    offset += nv;
                }
            }
            k -= 1;
        }

        // x <- D^-1 x
        let mut k: i32 = 0;
        while k < nv {
            let i = if !index.is_null() { *index.add(k as usize) } else { k };
            let inv_d_i = *qLDiagInv.add(i as usize);
            if n == 1 {
                *x.add(i as usize) *= inv_d_i;
            } else {
                let mut offset: i32 = 0;
                while offset < n * nv {
                    *x.add((i + offset) as usize) *= inv_d_i;
                    offset += nv;
                }
            }
            k += 1;
        }

        // x <- L^-1 x
        let mut k: i32 = 0;
        while k < nv {
            let i = if !index.is_null() { *index.add(k as usize) } else { k };
            if *rownnz.add(i as usize) == 1 {
                k += 1;
                continue;
            }
            let d = *rownnz.add(i as usize) - 1;
            if d > 0 {
                let adr = *rowadr.add(i as usize);
                if n == 1 {
                    *x.add(i as usize) -= crate::engine::engine_util_sparse::mju_dot_sparse(
                        qLD.add(adr as usize), x, d, colind.add(adr as usize));
                } else {
                    let mut offset: i32 = 0;
                    while offset < n * nv {
                        *x.add((i + offset) as usize) -= crate::engine::engine_util_sparse::mju_dot_sparse(
                            qLD.add(adr as usize), x.add(offset as usize), d, colind.add(adr as usize));
                        offset += nv;
                    }
                }
            }
            k += 1;
        }
    }
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
    // SAFETY: all pointers valid per caller contract. m, d, x, y are non-null
    // and properly aligned. nv is the number of DOFs.
    unsafe {
        let nv = (*m).nv as i32;
        if x != y as *mut f64 {
            crate::engine::engine_util_blas::mju_copy(x, y, n * nv);
        }
        mj_solve_ld(
            x, (*d).qLD, (*d).qLDiagInv, nv, n,
            (*m).M_rownnz, (*m).M_rowadr, (*m).M_colind, core::ptr::null(),
        );
    }
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
    extern "C" {
        fn mj_solveM2_impl(m: *const mjModel, d: *mut mjData, x: *mut f64, y: *const f64, sqrtInvD: *const f64, n: i32);
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_solveM2_impl(m, d, x, y, sqrtInvD, n) }
}

/// C: mj_comVel (engine/engine_core_smooth.h:98)
/// Calls: mji_copy6, mji_crossMotion, mju_addTo, mju_copy, mju_mulDofVec, mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mj_com_vel(m: *const mjModel, d: *mut mjData) {
    extern "C" {
        fn mj_comVel_impl(m: *const mjModel, d: *mut mjData);
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_comVel_impl(m, d) }
}

/// C: mj_subtreeVel (engine/engine_core_smooth.h:101)
/// Calls: mj_freeStack, mj_markStack, mj_objectVelocity, mj_stackAllocInfo, mji_addTo3, mji_cross, mji_mulMatVec3, mju_max, mju_mulMatTVec3, mju_scl3, mju_sub3
#[allow(unused_variables, non_snake_case)]
pub fn mj_subtree_vel(m: *const mjModel, d: *mut mjData) {
    extern "C" { fn mj_subtreeVel_impl(m: *const mjModel, d: *mut mjData); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_subtreeVel_impl(m, d) }
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
    extern "C" {
        fn mj_rne_impl(m: *const mjModel, d: *mut mjData, flg_acc: i32, result: *mut f64);
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_rne_impl(m, d, flg_acc, result) }
}

/// C: mj_rnePostConstraint (engine/engine_core_smooth.h:110)
/// Calls: mj_contactForce, mj_local2Global, mji_copy3, mji_crossForce, mju_add, mju_addTo, mju_isZero, mju_message, mju_mulDofVec, mju_mulInertVec, mju_mulMatTVec3, mju_scl3, mju_sub, mju_subFrom, mju_transformSpatial, mju_zero, mju_zero3
#[allow(unused_variables, non_snake_case)]
pub fn mj_rne_post_constraint(m: *const mjModel, d: *mut mjData) {
    extern "C" { fn mj_rnePostConstraint_impl(m: *const mjModel, d: *mut mjData); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_rnePostConstraint_impl(m, d) }
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
    extern "C" { fn mj_tendonBias_impl(m: *const mjModel, d: *mut mjData, qfrc: *mut f64); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_tendonBias_impl(m, d, qfrc) }
}

