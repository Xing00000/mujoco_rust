//! Port of: engine/engine_support.c
//! IR hash: c6d98e4f4b63b7f2
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mj_stateElemSize (engine/engine_support.c:138)
/// Calls: mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_state_elem_size(m: *const mjModel, sig: u32) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, sig : u32)
    // Previous return: i32
    todo ! ()
}

/// C: mj_stateElemPtr (engine/engine_support.c:162)
/// Calls: mju_message
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_state_elem_ptr(m: *const mjModel, d: *mut mjData, sig: u32) -> *mut f64 {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, sig : u32)
    // Previous return: * mut f64
    todo ! ()
}

/// C: mj_stateElemConstPtr (engine/engine_support.c:184)
/// Calls: mj_stateElemPtr
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_state_elem_const_ptr(m: *const mjModel, d: *const mjData, sig: u32) -> *const f64 {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * const mjData, sig : u32)
    // Previous return: * const f64
    todo ! ()
}

/// C: mj_geomDistanceCCD (engine/engine_support.c:519)
/// Calls: mj_freeStack, mj_markStack, mj_stackAllocByte, mjc_ccd, mjc_ccdSize, mjc_initCCDObj, mju_copy3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_geom_distance_ccd(m: *const mjModel, d: *mut mjData, g1: i32, g2: i32, distmax: f64, fromto: *mut f64) -> f64 {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, g1 : i32, g2 : i32, distmax : f64, fromto : * mut f64)
    // Previous return: f64
    todo ! ()
}

/// C: mj_stateSize (engine/engine_support.h:41)
/// Calls: mj_stateElemSize, mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_state_size(m: *const mjModel, sig: i32) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, sig : i32)
    // Previous return: i32
    todo ! ()
}

/// C: mj_getState (engine/engine_support.h:44)
/// Calls: mj_stateElemConstPtr, mj_stateElemSize, mju_copy, mju_message
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_get_state(m: *const mjModel, d: *const mjData, state: *mut f64, sig: i32) {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * const mjData, state : * mut f64, sig : i32)
    // Previous return: ()
    todo ! ()
}

/// C: mj_extractState (engine/engine_support.h:47)
/// Calls: mj_stateElemSize, mju_copy, mju_message
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_extract_state(m: *const mjModel, src: *const f64, srcsig: i32, dst: *mut f64, dstsig: i32) {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, src : * const f64, srcsig : i32, dst : * mut f64, dstsig : i32)
    // Previous return: ()
    todo ! ()
}

/// C: mj_setState (engine/engine_support.h:51)
/// Calls: mj_stateElemPtr, mj_stateElemSize, mju_copy, mju_message
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_set_state(m: *const mjModel, d: *mut mjData, state: *const f64, sig: i32) {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, state : * const f64, sig : i32)
    // Previous return: ()
    todo ! ()
}

/// C: mj_copyState (engine/engine_support.h:54)
/// Calls: mj_stateElemConstPtr, mj_stateElemPtr, mj_stateElemSize, mju_copy, mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_copy_state(m: *const mjModel, src: *const mjData, dst: *mut mjData, sig: i32) {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, src : * const mjData, dst : * mut mjData, sig : i32)
    // Previous return: ()
    todo ! ()
}

/// C: mj_setKeyframe (engine/engine_support.h:57)
/// Calls: mju_copy, mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_set_keyframe(m: *mut mjModel, d: *const mjData, k: i32) {
    // WARNING: signature changed — verify body
    // Previous params: (m : * mut mjModel, d : * const mjData, k : i32)
    // Previous return: ()
    todo ! ()
}

/// C: mj_fullM (engine/engine_support.h:62)
/// Calls: mju_sym2dense
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_full_m(m: *const mjModel, d: *const mjData, dst: *mut f64) {
    // SAFETY: caller guarantees m, d point to valid mjModel/mjData, dst has nv*nv capacity.
    unsafe {
        crate::engine::engine_util_sparse::mju_sym2dense(
            dst,
            (*d).M,
            (*m).nv as i32,
            (*m).M_rownnz,
            (*m).M_rowadr,
            (*m).M_colind,
        );
    }
}

/// C: mj_mulM (engine/engine_support.h:65)
/// Calls: mju_mulSymVecSparse
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_mul_m(m: *const mjModel, d: *const mjData, res: *mut f64, vec: *const f64) {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * const mjData, res : * mut f64, vec : * const f64)
    // Previous return: ()
    todo ! ()
}

/// C: mj_mulM2 (engine/engine_support.h:68)
/// Calls: mju_dotSparse, mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_mul_m2(m: *const mjModel, d: *const mjData, res: *mut f64, vec: *const f64) {
    use crate::engine::engine_util_blas::mju_zero;
    use crate::engine::engine_util_sparse::mju_dot_sparse;

    // SAFETY: caller guarantees m, d, res, vec are valid; res and vec have nv elements
    unsafe {
        let nv: usize = (*m).nv;
        let qLD: *const f64 = (*d).qLD;

        mju_zero(res, nv as i32);

        // res = L * vec
        let mut i: usize = 0;
        while i < nv {
            // diagonal
            *res.add(i) = *vec.add(i);

            // non-simple: add off-diagonals
            if *(*m).dof_simplenum.add(i) == 0 {
                let adr: usize = *(*m).M_rowadr.add(i) as usize;
                *res.add(i) += mju_dot_sparse(
                    qLD.add(adr),
                    vec,
                    *(*m).M_rownnz.add(i) - 1,
                    (*m).M_colind.add(adr),
                );
            }

            i += 1;
        }

        // res *= sqrt(D)
        let mut i: usize = 0;
        while i < nv {
            let diag: usize = (*(*m).M_rowadr.add(i) + *(*m).M_rownnz.add(i) - 1) as usize;
            *res.add(i) *= (*qLD.add(diag)).sqrt();

            i += 1;
        }
    }
}

/// C: mj_addM (engine/engine_support.h:72)
/// Calls: mju_addToMatSparse, mju_addToSymSparse
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_add_m(m: *const mjModel, d: *mut mjData, dst: *mut f64, rownnz: *mut i32, rowadr: *mut i32, colind: *mut i32) {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, dst : * mut f64, rownnz : * mut i32, rowadr : * mut i32, colind : * mut i32)
    // Previous return: ()
    todo ! ()
}

/// C: mj_applyFT (engine/engine_support.h:79)
/// Calls: mj_bodyChain, mj_freeStack, mj_isSparse, mj_jac, mj_jacSparse, mj_markStack, mj_stackAllocInfo, mju_addTo, mju_message, mju_mulMatTVec
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_apply_ft(m: *const mjModel, d: *mut mjData, force: *const f64, torque: *const f64, point: *const f64, body: i32, qfrc_target: *mut f64) {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, force : * const f64, torque : * const f64, point : * const f64, body : i32, qfrc_target : * mut f64)
    // Previous return: ()
    todo ! ()
}

/// C: mj_xfrcAccumulate (engine/engine_support.h:84)
/// Calls: mj_applyFT, mju_isZero, mju_isZeroByte
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_xfrc_accumulate(m: *const mjModel, d: *mut mjData, qfrc: *mut f64) {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, qfrc : * mut f64)
    // Previous return: ()
    todo ! ()
}

/// C: mj_geomDistance (engine/engine_support.h:90)
/// Calls: mj_geomDistanceCCD, mju_addScl3, mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_geom_distance(m: *const mjModel, d: *mut mjData, geom1: i32, geom2: i32, distmax: f64, fromto: *mut f64) -> f64 {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, geom1 : i32, geom2 : i32, distmax : f64, fromto : * mut f64)
    // Previous return: f64
    todo ! ()
}

/// C: mj_differentiatePos (engine/engine_support.h:94)
/// Calls: mju_scl3, mju_subQuat
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_differentiate_pos(m: *const mjModel, qvel: *mut f64, dt: f64, qpos1: *const f64, qpos2: *const f64) {
    // mjtJoint enum values
    const mjJNT_FREE: i32 = 0;
    const mjJNT_BALL: i32 = 1;
    const mjJNT_SLIDE: i32 = 2;
    const mjJNT_HINGE: i32 = 3;

    // SAFETY: caller guarantees m points to a valid mjModel, qvel/qpos1/qpos2 are valid arrays.
    unsafe {
        // loop over joints
        let mut j: i32 = 0;
        while j < (*m).njnt as i32 {
            // get addresses in qpos and qvel
            let mut padr: i32 = *(*m).jnt_qposadr.add(j as usize);
            let mut vadr: i32 = *(*m).jnt_dofadr.add(j as usize);

            let jnt_type: i32 = *(*m).jnt_type.add(j as usize);

            if jnt_type == mjJNT_FREE {
                let mut i: i32 = 0;
                while i < 3 {
                    *qvel.add((vadr + i) as usize) =
                        (*qpos2.add((padr + i) as usize) - *qpos1.add((padr + i) as usize)) / dt;
                    i += 1;
                }
                vadr += 3;
                padr += 3;

                // continue with rotations (fallthrough to BALL)
                crate::engine::engine_util_spatial::mju_sub_quat(
                    qvel.add(vadr as usize),
                    qpos2.add(padr as usize),
                    qpos1.add(padr as usize),
                );
                crate::engine::engine_util_blas::mju_scl3(
                    qvel.add(vadr as usize),
                    qvel.add(vadr as usize),
                    1.0 / dt,
                );
            } else if jnt_type == mjJNT_BALL {
                // solve: qpos1 * quat(qvel * dt) = qpos2
                crate::engine::engine_util_spatial::mju_sub_quat(
                    qvel.add(vadr as usize),
                    qpos2.add(padr as usize),
                    qpos1.add(padr as usize),
                );
                crate::engine::engine_util_blas::mju_scl3(
                    qvel.add(vadr as usize),
                    qvel.add(vadr as usize),
                    1.0 / dt,
                );
            } else if jnt_type == mjJNT_HINGE || jnt_type == mjJNT_SLIDE {
                *qvel.add(vadr as usize) =
                    (*qpos2.add(padr as usize) - *qpos1.add(padr as usize)) / dt;
            }

            j += 1;
        }
    }
}

/// C: mj_integratePosInd (engine/engine_support.h:98)
/// Calls: mju_quatIntegrate
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_integrate_pos_ind(m: *const mjModel, qpos: *mut f64, qvel: *const f64, dt: f64, index: *const i32, nbody: i32) {
    // mjtJoint enum values from mjtype.h
    const mjJNT_FREE: i32 = 0;
    const mjJNT_BALL: i32 = 1;
    const mjJNT_SLIDE: i32 = 2;
    const mjJNT_HINGE: i32 = 3;

    // SAFETY: caller guarantees m points to a valid mjModel, qpos/qvel are valid arrays
    // with sizes matching m->nq/m->nv, index (if non-null) has at least nbody elements.
    unsafe {
        let mut b: i32 = 1;
        while b < nbody {
            let k: i32 = if index.is_null() { b } else { *index.add(b as usize) };
            let start: i32 = *(*m).body_jntadr.add(k as usize);
            let end: i32 = start + *(*m).body_jntnum.add(k as usize);

            let mut j: i32 = start;
            while j < end {
                // get addresses in qpos and qvel
                let mut padr: i32 = *(*m).jnt_qposadr.add(j as usize);
                let mut vadr: i32 = *(*m).jnt_dofadr.add(j as usize);

                let jnt_type: i32 = *(*m).jnt_type.add(j as usize);

                if jnt_type == mjJNT_FREE {
                    // position update
                    let mut i: i32 = 0;
                    while i < 3 {
                        *qpos.add((padr + i) as usize) += dt * *qvel.add((vadr + i) as usize);
                        i += 1;
                    }
                    padr += 3;
                    vadr += 3;

                    // fallthrough to quaternion update
                    crate::engine::engine_util_spatial::mju_quat_integrate(
                        qpos.add(padr as usize),
                        qvel.add(vadr as usize),
                        dt,
                    );
                } else if jnt_type == mjJNT_BALL {
                    // quaternion update
                    crate::engine::engine_util_spatial::mju_quat_integrate(
                        qpos.add(padr as usize),
                        qvel.add(vadr as usize),
                        dt,
                    );
                } else if jnt_type == mjJNT_HINGE || jnt_type == mjJNT_SLIDE {
                    // scalar update: same for rotation and translation
                    *qpos.add(padr as usize) += dt * *qvel.add(vadr as usize);
                }

                j += 1;
            }
            b += 1;
        }
    }
}

/// C: mj_integratePos (engine/engine_support.h:102)
/// Calls: mj_integratePosInd
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_integrate_pos(m: *const mjModel, qpos: *mut f64, qvel: *const f64, dt: f64) {
    // SAFETY: caller guarantees m points to a valid mjModel, qpos/qvel are valid arrays.
    unsafe {
        mj_integrate_pos_ind(m, qpos, qvel, dt, std::ptr::null(), (*m).nbody as i32);
    }
}

/// C: mj_normalizeQuat (engine/engine_support.h:105)
/// Calls: mju_normalize4
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_normalize_quat(m: *const mjModel, qpos: *mut f64) {
    const mjJNT_FREE: i32 = 0;
    const mjJNT_BALL: i32 = 1;

    // SAFETY: caller guarantees m points to a valid mjModel, qpos is a valid array of size m->nq
    unsafe {
        let mut i: i32 = 0;
        while i < (*m).njnt as i32 {
            let jnt_type = *(*m).jnt_type.add(i as usize);
            if jnt_type == mjJNT_BALL || jnt_type == mjJNT_FREE {
                let offset = *(*m).jnt_qposadr.add(i as usize)
                    + 3 * (if jnt_type == mjJNT_FREE { 1 } else { 0 });
                crate::engine::engine_util_blas::mju_normalize4(qpos.add(offset as usize));
            }
            i += 1;
        }
    }
}

/// C: mj_actuatorDisabled (engine/engine_support.h:108)
#[allow(unused_variables, non_snake_case)]
pub fn mj_actuator_disabled(m: *const mjModel, i: i32) -> i32 {
    // SAFETY: caller guarantees m points to a valid mjModel, i is a valid actuator index
    unsafe {
        let group: i32 = *(*m).actuator_group.add(i as usize);
        if group < 0 || group > 30 {
            0
        } else {
            if (*m).opt.disableactuator & (1 << group) != 0 {
                1
            } else {
                0
            }
        }
    }
}

/// C: mj_nextActivation (engine/engine_support.h:111)
/// Calls: mj_dcmotorSlots, mj_lugreStribeck, mju_clip, mju_max
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_next_activation(m: *const mjModel, d: *const mjData, actuator_id: i32, act_adr: i32, act_dot: f64) -> f64 {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * const mjData, actuator_id : i32, act_adr : i32, act_dot : f64)
    // Previous return: f64
    todo ! ()
}

/// C: mj_getTotalmass (engine/engine_support.h:115)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_get_totalmass(m: *const mjModel) -> f64 {
    // SAFETY: caller guarantees m points to a valid mjModel with body_mass array of size nbody
    unsafe {
        let mut res: f64 = 0.0;
        let mut i: i32 = 1;
        while i < (*m).nbody as i32 {
            res += *(*m).body_mass.add(i as usize);
            i += 1;
        }
        res
    }
}

/// C: mj_setTotalmass (engine/engine_support.h:118)
/// Calls: mj_getTotalmass, mju_max
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_set_totalmass(m: *mut mjModel, newmass: f64) {
    use crate::engine::engine_util_misc::mju_max;

    const MJMINVAL: f64 = 1e-15;

    // SAFETY: caller guarantees m points to a valid mjModel
    unsafe {
        // compute scale factor, avoid zeros
        let scale: f64 = mju_max(MJMINVAL, newmass / mju_max(MJMINVAL, mj_get_totalmass(m as *const mjModel)));

        // scale all masses and inertias
        let mut i: usize = 1;
        while i < (*m).nbody {
            *(*m).body_mass.add(i) *= scale;
            *(*m).body_inertia.add(3 * i) *= scale;
            *(*m).body_inertia.add(3 * i + 1) *= scale;
            *(*m).body_inertia.add(3 * i + 2) *= scale;

            i += 1;
        }

        // don't forget to call mj_set0 after changing masses
    }
}

/// C: mj_version (engine/engine_support.h:121)
#[allow(unused_variables, non_snake_case)]
pub fn mj_version() -> i32 {
    3010001
}

/// C: mj_versionString (engine/engine_support.h:124)
#[allow(unused_variables, non_snake_case)]
pub fn mj_version_string() -> *const i8 {
    // SAFETY: static byte string with null terminator
    b"3.10.1\0".as_ptr() as *const i8
}

/// C: mju_condataSize (engine/engine_support.h:127)
#[allow(unused_variables, non_snake_case)]
pub fn mju_condata_size(dataSpec: i32) -> i32 {
    // mjNCONDATA = 7, sizes: [1, 3, 3, 1, 3, 3, 3]
    const MJNCONDATA: i32 = 7;
    const MJCONDATA_SIZE: [i32; 7] = [1, 3, 3, 1, 3, 3, 3];

    let mut size: i32 = 0;
    for i in 0..MJNCONDATA as usize {
        if (dataSpec & (1 << i)) != 0 {
            size += MJCONDATA_SIZE[i];
        }
    }
    size
}

/// C: mju_raydataSize (engine/engine_support.h:130)
#[allow(unused_variables, non_snake_case)]
pub fn mju_raydata_size(dataspec: i32) -> i32 {
    // mjNRAYDATA = 6, sizes: [1, 3, 3, 3, 3, 1]
    const MJNRAYDATA: i32 = 6;
    const MJRAYDATA_SIZE: [i32; 6] = [1, 3, 3, 3, 3, 1];

    let mut size: i32 = 0;
    for i in 0..MJNRAYDATA as usize {
        if (dataspec & (1 << i)) != 0 {
            size += MJRAYDATA_SIZE[i];
        }
    }
    size
}

/// C: mju_camIntrinsics (engine/engine_support.h:134)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_cam_intrinsics(m: *const mjModel, camid: i32, fx: *mut f64, fy: *mut f64, cx: *mut f64, cy: *mut f64, ortho_extent: *mut f64) {
    const MJPI: f64 = std::f64::consts::PI;
    const MJ_PROJ_PERSPECTIVE: i32 = 0;
    const MJ_PROJ_ORTHOGRAPHIC: i32 = 1;
    // SAFETY: caller guarantees model and output pointers are valid
    unsafe {
        let width = *(*m).cam_resolution.add((2 * camid) as usize) as f64;
        let height = *(*m).cam_resolution.add((2 * camid + 1) as usize) as f64;
        let sensorsize = (*m).cam_sensorsize.add((2 * camid) as usize);
        let intrinsic = (*m).cam_intrinsic.add((4 * camid) as usize);
        let projection = *(*m).cam_projection.add(camid as usize) as i32;

        if projection == MJ_PROJ_PERSPECTIVE {
            if *sensorsize.add(0) != 0.0 && *sensorsize.add(1) != 0.0 {
                // intrinsic-based perspective camera
                *fx = *intrinsic.add(0) as f64 / *sensorsize.add(0) as f64 * width;
                *fy = *intrinsic.add(1) as f64 / *sensorsize.add(1) as f64 * height;
                *cx = *intrinsic.add(2) as f64 / *sensorsize.add(0) as f64 * width;
                *cy = *intrinsic.add(3) as f64 / *sensorsize.add(1) as f64 * height;
            } else {
                // fovy-based perspective camera
                let val = 0.5 / (*(*m).cam_fovy.add(camid as usize) * MJPI / 360.0).tan() * height;
                *fx = val;
                *fy = val;
                *cx = width / 2.0;
                *cy = height / 2.0;
            }
        } else if projection == MJ_PROJ_ORTHOGRAPHIC {
            // orthographic
            *fx = width / 2.0;
            *fy = height / 2.0;
            *cx = *fx;
            *cy = *fy;
        }

        // extent only used for orthographic cameras
        *ortho_extent = *(*m).cam_fovy.add(camid as usize);
    }
}

/// C: mj_readCtrl (engine/engine_support.h:141)
/// Calls: mju_historyRead, mju_message
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_read_ctrl(m: *const mjModel, d: *const mjData, id: i32, time: f64, interp: i32) -> f64 {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * const mjData, id : i32, time : f64, interp : i32)
    // Previous return: f64
    todo ! ()
}

/// C: mj_readSensor (engine/engine_support.h:147)
/// Calls: mju_historyRead, mju_message
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_read_sensor(m: *const mjModel, d: *const mjData, id: i32, time: f64, result: *mut f64, interp: i32) -> *const f64 {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * const mjData, id : i32, time : f64, result : * mut f64, interp : i32)
    // Previous return: * const f64
    todo ! ()
}

/// C: mj_initCtrlHistory (engine/engine_support.h:152)
/// Calls: mju_historyInit, mju_message
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_init_ctrl_history(m: *const mjModel, d: *mut mjData, id: i32, times: *const f64, values: *const f64) {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, id : i32, times : * const f64, values : * const f64)
    // Previous return: ()
    todo ! ()
}

/// C: mj_initSensorHistory (engine/engine_support.h:158)
/// Calls: mju_historyInit, mju_message
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_init_sensor_history(m: *const mjModel, d: *mut mjData, id: i32, times: *const f64, values: *const f64, phase: f64) {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, id : i32, times : * const f64, values : * const f64, phase : f64)
    // Previous return: ()
    todo ! ()
}

