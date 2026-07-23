//! Port of: engine/engine_support.c
//! IR hash: bd605ac8158c32d6
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mj_stateElemSize (engine/engine_support.c:138)
/// Calls: mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_state_elem_size(m: *const mjModel, sig: u32) -> i32 {
    // SAFETY: m is a valid mjModel pointer (caller contract)
    unsafe {
        match sig {
            mjtState_mjSTATE_TIME => 1,
            mjtState_mjSTATE_QPOS => (*m).nq as i32,
            mjtState_mjSTATE_QVEL => (*m).nv as i32,
            mjtState_mjSTATE_ACT => (*m).na as i32,
            mjtState_mjSTATE_HISTORY => (*m).nhistory as i32,
            mjtState_mjSTATE_WARMSTART => (*m).nv as i32,
            mjtState_mjSTATE_CTRL => (*m).nu as i32,
            mjtState_mjSTATE_QFRC_APPLIED => (*m).nv as i32,
            mjtState_mjSTATE_XFRC_APPLIED => (6 * (*m).nbody) as i32,
            mjtState_mjSTATE_EQ_ACTIVE => (*m).neq as i32,
            mjtState_mjSTATE_MOCAP_POS => (3 * (*m).nmocap) as i32,
            mjtState_mjSTATE_MOCAP_QUAT => (4 * (*m).nmocap) as i32,
            mjtState_mjSTATE_USERDATA => (*m).nuserdata as i32,
            mjtState_mjSTATE_PLUGIN => (*m).npluginstate as i32,
            _ => {
                crate::engine::engine_util_errmem::mju_error(
                    b"invalid state element %u\0".as_ptr() as *const i8);
                0
            }
        }
    }
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
    // SAFETY: m, d are valid pointers (caller contract)
    unsafe {
        match sig {
            mjtState_mjSTATE_TIME => &mut (*d).time as *mut f64,
            mjtState_mjSTATE_QPOS => (*d).qpos,
            mjtState_mjSTATE_QVEL => (*d).qvel,
            mjtState_mjSTATE_ACT => (*d).act,
            mjtState_mjSTATE_HISTORY => (*d).history,
            mjtState_mjSTATE_WARMSTART => (*d).qacc_warmstart,
            mjtState_mjSTATE_CTRL => (*d).ctrl,
            mjtState_mjSTATE_QFRC_APPLIED => (*d).qfrc_applied,
            mjtState_mjSTATE_XFRC_APPLIED => (*d).xfrc_applied,
            mjtState_mjSTATE_MOCAP_POS => (*d).mocap_pos,
            mjtState_mjSTATE_MOCAP_QUAT => (*d).mocap_quat,
            mjtState_mjSTATE_USERDATA => (*d).userdata,
            mjtState_mjSTATE_PLUGIN => (*d).plugin_state,
            _ => {
                crate::engine::engine_util_errmem::mju_error(
                    b"invalid state element %u\0".as_ptr() as *const i8);
                std::ptr::null_mut()
            }
        }
    }
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
    // SAFETY: discards const qualifier from d, same as C implementation
    mj_state_elem_ptr(m, d as *mut mjData, sig) as *const f64
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
    todo!() // mj_geomDistanceCCD
}

/// C: mj_stateSize (engine/engine_support.h:41)
/// Calls: mj_stateElemSize, mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_state_size(m: *const mjModel, sig: i32) -> i32 {
    const MJ_NSTATE: i32 = 14;

    // SAFETY: m is a valid pointer (caller contract).
    unsafe {
        if sig < 0 {
            crate::engine::engine_util_errmem::mju_error(
                b"invalid state signature < 0\0".as_ptr() as *const i8);
            return 0;
        }

        if sig >= (1 << MJ_NSTATE) {
            crate::engine::engine_util_errmem::mju_error(
                b"invalid state signature >= 2^mjNSTATE\0".as_ptr() as *const i8);
            return 0;
        }

        let mut size: i32 = 0;
        for i in 0..MJ_NSTATE {
            let element: u32 = 1 << i;
            if (element as i32 & sig) != 0 {
                size += mj_state_elem_size(m, element);
            }
        }

        size
    }
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
    const MJ_NSTATE: i32 = 14;
    const MJ_STATE_EQ_ACTIVE: u32 = 1 << 9;

    // SAFETY: m, d, state are valid pointers (caller contract).
    unsafe {
        if sig < 0 {
            crate::engine::engine_util_errmem::mju_error(
                b"invalid state signature < 0\0".as_ptr() as *const i8);
            return;
        }

        if sig >= (1 << MJ_NSTATE) {
            crate::engine::engine_util_errmem::mju_error(
                b"invalid state signature >= 2^mjNSTATE\0".as_ptr() as *const i8);
            return;
        }

        let mut adr: i32 = 0;
        for i in 0..MJ_NSTATE {
            let element: u32 = 1 << i;
            if (element as i32 & sig) != 0 {
                let size = mj_state_elem_size(m, element);

                // special handling of eq_active (mjtBool)
                if element == MJ_STATE_EQ_ACTIVE {
                    let neq = (*m).neq as i32;
                    for j in 0..neq {
                        *state.add(adr as usize) = if *(*d).eq_active.add(j as usize) { 1.0 } else { 0.0 };
                        adr += 1;
                    }
                }
                // regular state components (mjtNum)
                else {
                    let ptr = mj_state_elem_const_ptr(m, d, element);
                    crate::engine::engine_util_blas::mju_copy(state.add(adr as usize), ptr, size);
                    adr += size;
                }
            }
        }
    }
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
    const MJ_NSTATE: i32 = 14;

    // SAFETY: m, src, dst are valid pointers (caller contract).
    unsafe {
        if srcsig < 0 {
            crate::engine::engine_util_errmem::mju_error(
                b"invalid srcsig < 0\0".as_ptr() as *const i8);
            return;
        }

        if srcsig >= (1 << MJ_NSTATE) {
            crate::engine::engine_util_errmem::mju_error(
                b"invalid srcsig >= 2^mjNSTATE\0".as_ptr() as *const i8);
            return;
        }

        if (srcsig & dstsig) != dstsig {
            crate::engine::engine_util_errmem::mju_error(
                b"dstsig is not a subset of srcsig\0".as_ptr() as *const i8);
            return;
        }

        let mut src_off: i32 = 0;
        let mut dst_off: i32 = 0;
        for i in 0..MJ_NSTATE {
            let element: u32 = 1 << i;
            if (element as i32 & srcsig) != 0 {
                let size = mj_state_elem_size(m, element);
                if (element as i32 & dstsig) != 0 {
                    crate::engine::engine_util_blas::mju_copy(
                        dst.add(dst_off as usize), src.add(src_off as usize), size);
                    dst_off += size;
                }
                src_off += size;
            }
        }
    }
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
    const MJ_NSTATE: i32 = 14;
    const MJ_STATE_EQ_ACTIVE: u32 = 1 << 9;

    // SAFETY: m, d, state are valid pointers (caller contract).
    unsafe {
        if sig < 0 {
            crate::engine::engine_util_errmem::mju_error(
                b"invalid state signature < 0\0".as_ptr() as *const i8);
            return;
        }

        if sig >= (1 << MJ_NSTATE) {
            crate::engine::engine_util_errmem::mju_error(
                b"invalid state signature >= 2^mjNSTATE\0".as_ptr() as *const i8);
            return;
        }

        let mut adr: i32 = 0;
        for i in 0..MJ_NSTATE {
            let element: u32 = 1 << i;
            if (element as i32 & sig) != 0 {
                let size = mj_state_elem_size(m, element);

                // special handling of eq_active (mjtBool)
                if element == MJ_STATE_EQ_ACTIVE {
                    let neq = (*m).neq as i32;
                    for j in 0..neq {
                        *(*d).eq_active.add(j as usize) = *state.add(adr as usize) != 0.0;
                        adr += 1;
                    }
                }
                // regular state components (mjtNum)
                else {
                    let ptr = mj_state_elem_ptr(m, d, element);
                    crate::engine::engine_util_blas::mju_copy(ptr, state.add(adr as usize), size);
                    adr += size;
                }
            }
        }
    }
}

/// C: mj_copyState (engine/engine_support.h:54)
/// Calls: mj_stateElemConstPtr, mj_stateElemPtr, mj_stateElemSize, mju_copy, mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_copy_state(m: *const mjModel, src: *const mjData, dst: *mut mjData, sig: i32) {
    const MJ_NSTATE: i32 = 14;
    const MJ_STATE_EQ_ACTIVE: u32 = 1 << 9;

    // SAFETY: m, src, dst are valid pointers (caller contract).
    unsafe {
        if sig < 0 {
            crate::engine::engine_util_errmem::mju_error(
                b"invalid state signature < 0\0".as_ptr() as *const i8);
            return;
        }

        if sig >= (1 << MJ_NSTATE) {
            crate::engine::engine_util_errmem::mju_error(
                b"invalid state signature >= 2^mjNSTATE\0".as_ptr() as *const i8);
            return;
        }

        for i in 0..MJ_NSTATE {
            let element: u32 = 1 << i;
            if (element as i32 & sig) != 0 {
                let size = mj_state_elem_size(m, element);

                // special handling of eq_active (mjtBool)
                if element == MJ_STATE_EQ_ACTIVE {
                    let neq = (*m).neq as i32;
                    for j in 0..neq {
                        *(*dst).eq_active.add(j as usize) = *(*src).eq_active.add(j as usize);
                    }
                }
                // regular state components (mjtNum)
                else {
                    let dst_ptr = mj_state_elem_ptr(m, dst, element);
                    let src_ptr = mj_state_elem_const_ptr(m, src, element);
                    crate::engine::engine_util_blas::mju_copy(dst_ptr, src_ptr, size);
                }
            }
        }
    }
}

/// C: mj_setKeyframe (engine/engine_support.h:57)
/// Calls: mju_copy, mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_set_keyframe(m: *mut mjModel, d: *const mjData, k: i32) {
    // SAFETY: m, d are valid pointers (caller contract).
    unsafe {
        // check keyframe index
        if k as i64 >= (*m).nkey {
            crate::engine::engine_util_errmem::mju_error(
                b"keyframe index too large\0".as_ptr() as *const i8);
            return;
        }
        if k < 0 {
            crate::engine::engine_util_errmem::mju_error(
                b"keyframe index cannot be negative\0".as_ptr() as *const i8);
            return;
        }

        let nq = (*m).nq as i32;
        let nv = (*m).nv as i32;
        let na = (*m).na as i32;
        let nu = (*m).nu as i32;
        let nmocap = (*m).nmocap as i32;

        // copy state to model keyframe
        *(*m).key_time.add(k as usize) = (*d).time;
        crate::engine::engine_util_blas::mju_copy(
            (*m).key_qpos.add((k as i64 * (*m).nq) as usize), (*d).qpos, nq);
        crate::engine::engine_util_blas::mju_copy(
            (*m).key_qvel.add((k as i64 * (*m).nv) as usize), (*d).qvel, nv);
        crate::engine::engine_util_blas::mju_copy(
            (*m).key_act.add((k as i64 * (*m).na) as usize), (*d).act, na);
        crate::engine::engine_util_blas::mju_copy(
            (*m).key_mpos.add((k as i64 * 3 * (*m).nmocap) as usize), (*d).mocap_pos, 3 * nmocap);
        crate::engine::engine_util_blas::mju_copy(
            (*m).key_mquat.add((k as i64 * 4 * (*m).nmocap) as usize), (*d).mocap_quat, 4 * nmocap);
        crate::engine::engine_util_blas::mju_copy(
            (*m).key_ctrl.add((k as i64 * (*m).nu) as usize), (*d).ctrl, nu);
    }
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
    // SAFETY: m, d, dst valid. Calls mju_sym2dense with sparse M from mjData.
    //   mjModel: nv at 8, M_rownnz at 5424, M_rowadr at 5432, M_colind at 5440
    //   mjData: M at 161120
    unsafe {
        let m_ptr = m as *const u8;
        let d_ptr = d as *const u8;
        let nv = *(m_ptr.add(8) as *const usize) as i32;
        let M_rownnz = *(m_ptr.add(5424) as *const *const i32);
        let M_rowadr = *(m_ptr.add(5432) as *const *const i32);
        let M_colind = *(m_ptr.add(5440) as *const *const i32);
        let M = *(d_ptr.add(161120) as *const *const f64);

        crate::engine::engine_util_sparse::mju_sym2dense(dst, M, nv, M_rownnz, M_rowadr, M_colind);
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
    // SAFETY: m, d, res, vec valid. Calls mju_mulSymVecSparse.
    //   mjModel: nv at 8, M_rownnz at 5424, M_rowadr at 5432, M_colind at 5440
    //   mjData: M at 161120
    unsafe {
        let m_ptr = m as *const u8;
        let d_ptr = d as *const u8;
        let nv = *(m_ptr.add(8) as *const usize) as i32;
        let M_rownnz = *(m_ptr.add(5424) as *const *const i32);
        let M_rowadr = *(m_ptr.add(5432) as *const *const i32);
        let M_colind = *(m_ptr.add(5440) as *const *const i32);
        let M = *(d_ptr.add(161120) as *const *const f64);

        crate::engine::engine_util_sparse::mju_mul_sym_vec_sparse(res, M, vec, nv, M_rownnz, M_rowadr, M_colind);
    }
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
    // SAFETY: m, d, res, vec are valid pointers (caller contract). Arrays are nv-sized.
    unsafe {
        let nv = (*m).nv as i32;
        let qLD = (*d).qLD;

        crate::engine::engine_util_blas::mju_zero(res, nv);

        // res = L * vec
        for i in 0..nv {
            // diagonal
            *res.add(i as usize) = *vec.add(i as usize);

            // non-simple: add off-diagonals
            if *(*m).dof_simplenum.add(i as usize) == 0 {
                let adr = *(*m).M_rowadr.add(i as usize);
                *res.add(i as usize) += crate::engine::engine_util_sparse::mju_dot_sparse(
                    qLD.add(adr as usize),
                    vec,
                    *(*m).M_rownnz.add(i as usize) - 1,
                    (*m).M_colind.add(adr as usize),
                );
            }
        }

        // res *= sqrt(D)
        for i in 0..nv {
            let diag = *(*m).M_rowadr.add(i as usize) + *(*m).M_rownnz.add(i as usize) - 1;
            *res.add(i as usize) *= f64::sqrt(*qLD.add(diag as usize));
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
    // SAFETY: m, d, dst are valid pointers (caller contract)
    unsafe {
        let nv = (*m).nv as i32;

        // sparse
        if !rownnz.is_null() && !rowadr.is_null() && !colind.is_null() {
            crate::engine::engine_util_sparse::mju_add_to_mat_sparse(
                dst, rownnz, rowadr, colind, nv,
                (*d).M, (*m).M_rownnz, (*m).M_rowadr, (*m).M_colind);
        }
        // dense
        else {
            crate::engine::engine_util_sparse::mju_add_to_sym_sparse(
                dst, (*d).M, nv, (*m).M_rownnz, (*m).M_rowadr, (*m).M_colind, 0);
        }
    }
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
    // SAFETY: m, d are valid pointers. force, torque may be null.
    // point[3] valid. body is a valid body index. qfrc_target[nv] valid.
    unsafe {
        let nv = (*m).nv as i32;

        // allocate local variables
        crate::engine::engine_memory::mj_mark_stack(d);
        let jacp: *mut f64 = if !force.is_null() {
            crate::engine::engine_memory::mj_stack_alloc_num(d, (3 * nv) as usize)
        } else {
            std::ptr::null_mut()
        };
        let jacr: *mut f64 = if !torque.is_null() {
            crate::engine::engine_memory::mj_stack_alloc_num(d, (3 * nv) as usize)
        } else {
            std::ptr::null_mut()
        };
        let qforce: *mut f64 = crate::engine::engine_memory::mj_stack_alloc_num(d, nv as usize);

        // make sure body is in range
        if body < 0 || body >= (*m).nbody as i32 {
            crate::engine::engine_util_errmem::mju_error(
                b"invalid body\0".as_ptr() as *const i8);
        }

        // sparse case
        if crate::engine::engine_core_util::mj_is_sparse(m) != 0 {
            // construct chain and sparse Jacobians
            let chain: *mut i32 = crate::engine::engine_memory::mj_stack_alloc_int(d, nv as usize);
            let NV = crate::engine::engine_core_util::mj_body_chain(m, body, chain);
            crate::engine::engine_core_util::mj_jac_sparse(
                m, d as *const mjData, jacp, jacr, point, body, NV, chain, 0);

            // compute J'*f and accumulate
            if !force.is_null() {
                crate::engine::engine_util_blas::mju_mul_mat_t_vec(qforce, jacp, force, 3, NV);
                for i in 0..NV {
                    *qfrc_target.add(*chain.add(i as usize) as usize) += *qforce.add(i as usize);
                }
            }
            if !torque.is_null() {
                crate::engine::engine_util_blas::mju_mul_mat_t_vec(qforce, jacr, torque, 3, NV);
                for i in 0..NV {
                    *qfrc_target.add(*chain.add(i as usize) as usize) += *qforce.add(i as usize);
                }
            }
        }
        // dense case
        else {
            // compute Jacobians
            crate::engine::engine_core_util::mj_jac(
                m, d as *const mjData, jacp, jacr, point, body);

            // compute J'*f and accumulate
            if !force.is_null() {
                crate::engine::engine_util_blas::mju_mul_mat_t_vec(qforce, jacp, force, 3, nv);
                crate::engine::engine_util_blas::mju_add_to(qfrc_target, qforce, nv);
            }
            if !torque.is_null() {
                crate::engine::engine_util_blas::mju_mul_mat_t_vec(qforce, jacr, torque, 3, nv);
                crate::engine::engine_util_blas::mju_add_to(qfrc_target, qforce, nv);
            }
        }

        crate::engine::engine_memory::mj_free_stack(d);
    }
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
    // SAFETY: m, d, qfrc are valid pointers (caller contract).
    unsafe {
        let nbody = (*m).nbody as i32;
        let xfrc = (*d).xfrc_applied;

        // quick return if identically zero (efficient memcmp implementation)
        if crate::engine::engine_util_misc::mju_is_zero_byte(
            xfrc.add(6) as *const u8,
            (6 * (nbody - 1) as usize * std::mem::size_of::<f64>()) as i32,
        ) != 0 {
            return;
        }

        // some non-zero wrenches, apply them
        for i in 1..nbody {
            if crate::engine::engine_util_misc::mju_is_zero(xfrc.add(6 * i as usize), 6) == 0 {
                mj_apply_ft(
                    m, d,
                    xfrc.add(6 * i as usize),
                    xfrc.add(6 * i as usize + 3),
                    (*d).xipos.add(3 * i as usize),
                    i,
                    qfrc,
                );
            }
        }
    }
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
    todo!() // mj_geomDistance
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
    const MJ_JNT_FREE: i32 = 0;
    const MJ_JNT_BALL: i32 = 1;
    const MJ_JNT_HINGE: i32 = 3;
    const MJ_JNT_SLIDE: i32 = 2;

    // SAFETY: caller guarantees m, qvel, qpos1, qpos2 are valid
    unsafe {
        for j in 0..(*m).njnt as usize {
            let mut padr = *(*m).jnt_qposadr.add(j) as usize;
            let mut vadr = *(*m).jnt_dofadr.add(j) as usize;
            let jnt_type = *(*m).jnt_type.add(j);

            match jnt_type {
                MJ_JNT_FREE => {
                    for i in 0..3usize {
                        *qvel.add(vadr + i) = (*qpos2.add(padr + i) - *qpos1.add(padr + i)) / dt;
                    }
                    vadr += 3;
                    padr += 3;
                    // fallthrough to ball
                    crate::engine::engine_util_spatial::mju_sub_quat(
                        qvel.add(vadr), qpos2.add(padr), qpos1.add(padr));
                    crate::engine::engine_util_blas::mju_scl3(qvel.add(vadr), qvel.add(vadr), 1.0 / dt);
                }
                MJ_JNT_BALL => {
                    crate::engine::engine_util_spatial::mju_sub_quat(
                        qvel.add(vadr), qpos2.add(padr), qpos1.add(padr));
                    crate::engine::engine_util_blas::mju_scl3(qvel.add(vadr), qvel.add(vadr), 1.0 / dt);
                }
                MJ_JNT_HINGE | MJ_JNT_SLIDE => {
                    *qvel.add(vadr) = (*qpos2.add(padr) - *qpos1.add(padr)) / dt;
                }
                _ => {}
            }
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
    const MJ_JNT_FREE: i32 = 0;
    const MJ_JNT_BALL: i32 = 1;
    const MJ_JNT_HINGE: i32 = 3;
    const MJ_JNT_SLIDE: i32 = 2;

    // SAFETY: caller guarantees m, qpos, qvel valid; index valid if non-null
    unsafe {
        for b in 1..nbody {
            let k = if !index.is_null() { *index.add(b as usize) } else { b };
            let start = *(*m).body_jntadr.add(k as usize);
            let end = start + *(*m).body_jntnum.add(k as usize);
            for j in start..end {
                let mut padr = *(*m).jnt_qposadr.add(j as usize) as usize;
                let mut vadr = *(*m).jnt_dofadr.add(j as usize) as usize;
                let jnt_type = *(*m).jnt_type.add(j as usize);

                match jnt_type {
                    MJ_JNT_FREE => {
                        // position update
                        for i in 0..3usize {
                            *qpos.add(padr + i) += dt * *qvel.add(vadr + i);
                        }
                        padr += 3;
                        vadr += 3;
                        // fallthrough to ball quaternion update
                        crate::engine::engine_util_spatial::mju_quat_integrate(
                            qpos.add(padr), qvel.add(vadr), dt);
                    }
                    MJ_JNT_BALL => {
                        crate::engine::engine_util_spatial::mju_quat_integrate(
                            qpos.add(padr), qvel.add(vadr), dt);
                    }
                    MJ_JNT_HINGE | MJ_JNT_SLIDE => {
                        *qpos.add(padr) += dt * *qvel.add(vadr);
                    }
                    _ => {}
                }
            }
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
    // SAFETY: m is a valid mjModel pointer (caller contract)
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
    // SAFETY: m is valid mjModel, qpos is valid qpos array. Access fields via byte offsets:
    //   njnt: usize at offset 72
    //   jnt_type: *const i32 at offset 2048
    //   jnt_qposadr: *const i32 at offset 2056
    // mjJNT_BALL = 1, mjJNT_FREE = 0
    unsafe {
        let m_ptr = m as *const u8;
        let njnt = *(m_ptr.add(72) as *const usize);
        let jnt_type = *(m_ptr.add(2048) as *const *const i32);
        let jnt_qposadr = *(m_ptr.add(2056) as *const *const i32);

        for i in 0..njnt {
            let jtype = *jnt_type.add(i);
            // mjJNT_FREE = 0, mjJNT_BALL = 1
            if jtype == 1 || jtype == 0 {
                let offset = if jtype == 0 { 3 } else { 0 }; // FREE has 3 pos before quat
                let adr = *jnt_qposadr.add(i) as usize + offset as usize;
                crate::engine::engine_util_blas::mju_normalize4(qpos.add(adr));
            }
        }
    }
}

/// C: mj_actuatorDisabled (engine/engine_support.h:108)
#[allow(unused_variables, non_snake_case)]
pub fn mj_actuator_disabled(m: *const mjModel, i: i32) -> i32 {
    // SAFETY: m is a valid mjModel pointer; actuator_group ptr at offset 4696, opt.disableactuator i32 at offset 1024
    unsafe {
        let m_ptr = m as *const u8;
        let actuator_group_ptr = *(m_ptr.add(4696) as *const *const i32);
        let disableactuator = *(m_ptr.add(1024) as *const i32);
        let group = *actuator_group_ptr.offset(i as isize);
        if group < 0 || group > 30 {
            return 0;
        }
        if (disableactuator & (1 << group)) != 0 { 1 } else { 0 }
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
    const MJNDYN: i32 = 10;
    const MJNGAIN: i32 = 10;
    const MJNBIAS: i32 = 10;
    const MJMINVAL: f64 = 1e-15;
    const MJDYN_FILTEREXACT: i32 = 3;
    const MJDYN_DCMOTOR: i32 = 5;

    // SAFETY: m, d are valid pointers; actuator_id and act_adr within bounds (caller contract)
    unsafe {
        let mut act = *(*d).act.add(act_adr as usize);
        let dyntype = *(*m).actuator_dyntype.add(actuator_id as usize);

        if dyntype == MJDYN_FILTEREXACT {
            // exact filter: act(h) = act(0) + act_dot * tau * (1 - exp(-h/tau))
            let tau_raw = *(*m).actuator_dynprm.add((actuator_id * MJNDYN) as usize);
            let tau = if tau_raw > MJMINVAL { tau_raw } else { MJMINVAL };
            act = act + act_dot * tau * (1.0 - (-(*m).opt.timestep / tau).exp());
        } else if dyntype == MJDYN_DCMOTOR {
            let dynprm = (*m).actuator_dynprm.add((actuator_id * MJNDYN) as usize);
            let gainprm = (*m).actuator_gainprm.add((actuator_id * MJNGAIN) as usize);
            let slots = crate::engine::engine_util_misc::mj_dcmotor_slots(dynprm, gainprm);

            let offset = act_adr - *(*m).actuator_actadr.add(actuator_id as usize);

            if offset == slots.current {
                // current filter: exact integration
                let te_raw = *dynprm.add(0);
                let te = if te_raw > MJMINVAL { te_raw } else { MJMINVAL };
                act = act + act_dot * te * (1.0 - (-(*m).opt.timestep / te).exp());
            } else if offset == slots.bristle {
                // LuGre bristle: ZOH exact integration
                let biasprm = (*m).actuator_biasprm.add((MJNBIAS * actuator_id) as usize);
                let F_C = *biasprm.add(3);
                let F_S = *biasprm.add(4);
                let v_S = *biasprm.add(5);
                let sigma0 = *dynprm.add(5);
                let velocity = *(*d).actuator_velocity.add(actuator_id as usize);
                let g = crate::engine::engine_util_misc::mj_lugre_stribeck(velocity, F_C, F_S, v_S);

                let a = -sigma0 * velocity.abs() / (if g > MJMINVAL { g } else { MJMINVAL });
                let h = (*m).opt.timestep;
                let exp_ah = (a * h).exp();
                let int_h = if a.abs() > MJMINVAL { (exp_ah - 1.0) / a } else { h };
                act = exp_ah * act + int_h * velocity;
            } else if offset == slots.integral {
                // integral: Euler with anti-windup clamp
                act = act + act_dot * (*m).opt.timestep;
                let Imax = *dynprm.add(8);
                if Imax > 0.0 {
                    act = crate::engine::engine_util_misc::mju_clip(act, -Imax, Imax);
                }
            } else {
                // temperature and slew: Euler
                act = act + act_dot * (*m).opt.timestep;
            }
        } else {
            // default: Euler integration
            act = act + act_dot * (*m).opt.timestep;
        }

        // clamp to actrange unless DC motor
        if dyntype != MJDYN_DCMOTOR && *(*m).actuator_actlimited.add(actuator_id as usize) {
            let actrange = (*m).actuator_actrange.add(2 * actuator_id as usize);
            act = crate::engine::engine_util_misc::mju_clip(act, *actrange.add(0), *actrange.add(1));
        }

        act
    }
}

/// C: mj_getTotalmass (engine/engine_support.h:115)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_get_totalmass(m: *const mjModel) -> f64 {
    // SAFETY: m is valid mjModel. Access fields via byte offsets:
    //   nbody: usize at offset 32
    //   body_mass: *const f64 at offset 1888
    unsafe {
        let m_ptr = m as *const u8;
        let nbody = *(m_ptr.add(32) as *const usize);
        let body_mass = *(m_ptr.add(1888) as *const *const f64);

        let mut res: f64 = 0.0;
        for i in 1..nbody {
            res += *body_mass.add(i);
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
    const MJ_MINVAL: f64 = 1E-15_f64;
    // SAFETY: m is valid mutable mjModel. Access fields via byte offsets:
    //   nbody: usize at offset 32
    //   body_mass: *mut f64 at offset 1888
    //   body_inertia: *mut f64 at offset 1904
    unsafe {
        let m_ptr = m as *mut u8;
        let nbody = *(m_ptr.add(32) as *const usize);
        let body_mass = *(m_ptr.add(1888) as *const *mut f64);
        let body_inertia = *(m_ptr.add(1904) as *const *mut f64);

        // compute scale factor
        let current = mj_get_totalmass(m);
        let scale = f64::max(MJ_MINVAL, newmass / f64::max(MJ_MINVAL, current));

        // scale all masses and inertias
        for i in 1..nbody {
            *body_mass.add(i) *= scale;
            *body_inertia.add(3 * i) *= scale;
            *body_inertia.add(3 * i + 1) *= scale;
            *body_inertia.add(3 * i + 2) *= scale;
        }
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
    // SAFETY: returns pointer to static null-terminated string
    b"3.10.1\0".as_ptr() as *const i8
}

/// C: mju_condataSize (engine/engine_support.h:127)
/// Calls: FilePath::size
#[allow(unused_variables, non_snake_case)]
pub fn mju_condata_size(dataSpec: i32) -> i32 {
    // mjNCONDATA = 7, mjCONDATA_SIZE = [1, 3, 3, 1, 3, 3, 3]
    const MJ_NCONDATA: i32 = 7;
    const MJ_CONDATA_SIZE: [i32; 7] = [1, 3, 3, 1, 3, 3, 3];

    let mut size: i32 = 0;
    let mut i: i32 = 0;
    while i < MJ_NCONDATA {
        if (dataSpec & (1 << i)) != 0 {
            size += MJ_CONDATA_SIZE[i as usize];
        }
        i += 1;
    }
    size
}

/// C: mju_raydataSize (engine/engine_support.h:130)
/// Calls: FilePath::size
#[allow(unused_variables, non_snake_case)]
pub fn mju_raydata_size(dataspec: i32) -> i32 {
    // mjNRAYDATA = 6, mjRAYDATA_SIZE = [1, 3, 3, 3, 3, 1]
    const MJ_NRAYDATA: i32 = 6;
    const MJ_RAYDATA_SIZE: [i32; 6] = [1, 3, 3, 3, 3, 1];

    let mut size: i32 = 0;
    let mut i: i32 = 0;
    while i < MJ_NRAYDATA {
        if (dataspec & (1 << i)) != 0 {
            size += MJ_RAYDATA_SIZE[i as usize];
        }
        i += 1;
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
    // SAFETY: m valid mjModel. Byte offsets:
    //   cam_projection: *const i32 at 2704
    //   cam_fovy: *const f64 at 2712
    //   cam_resolution: *const i32 at 2728
    //   cam_sensorsize: *const f32 at 2744
    //   cam_intrinsic: *const f32 at 2752
    // mjPROJ_PERSPECTIVE = 0, mjPROJ_ORTHOGRAPHIC = 1
    unsafe {
        let m_ptr = m as *const u8;
        let cam_projection = *(m_ptr.add(2704) as *const *const i32);
        let cam_fovy = *(m_ptr.add(2712) as *const *const f64);
        let cam_resolution = *(m_ptr.add(2728) as *const *const i32);
        let cam_sensorsize = *(m_ptr.add(2744) as *const *const f32);
        let cam_intrinsic = *(m_ptr.add(2752) as *const *const f32);

        let cid = camid as usize;
        let width = *cam_resolution.add(2 * cid) as f64;
        let height = *cam_resolution.add(2 * cid + 1) as f64;
        let ss0 = *cam_sensorsize.add(2 * cid) as f64;
        let ss1 = *cam_sensorsize.add(2 * cid + 1) as f64;
        let projection = *cam_projection.add(cid);

        match projection {
            0 => {
                // mjPROJ_PERSPECTIVE
                if ss0 != 0.0 && ss1 != 0.0 {
                    let intr0 = *cam_intrinsic.add(4 * cid) as f64;
                    let intr1 = *cam_intrinsic.add(4 * cid + 1) as f64;
                    let intr2 = *cam_intrinsic.add(4 * cid + 2) as f64;
                    let intr3 = *cam_intrinsic.add(4 * cid + 3) as f64;
                    *fx = intr0 / ss0 * width;
                    *fy = intr1 / ss1 * height;
                    *cx = intr2 / ss0 * width;
                    *cy = intr3 / ss1 * height;
                } else {
                    let fovy = *cam_fovy.add(cid);
                    let f = 0.5 / f64::tan(fovy * std::f64::consts::PI / 360.0) * height;
                    *fx = f;
                    *fy = f;
                    *cx = width / 2.0;
                    *cy = height / 2.0;
                }
            }
            _ => {
                // mjPROJ_ORTHOGRAPHIC
                *fx = width / 2.0;
                *fy = height / 2.0;
                *cx = *fx;
                *cy = *fy;
            }
        }

        // extent only used for orthographic cameras
        *ortho_extent = *cam_fovy.add(cid);
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
    // SAFETY: m, d are valid model/data pointers; id validated below (caller contract)
    unsafe {
        // validate actuator id
        if id < 0 || id >= (*m).nu as i32 {
            crate::engine::engine_util_errmem::mju_error(
                b"invalid actuator id %d\0".as_ptr() as *const i8);
            return 0.0;
        }

        // no delay: return current ctrl value
        let nsample = *(*m).actuator_history.add(2 * id as usize);
        if nsample == 0 {
            return *(*d).ctrl.add(id as usize);
        }

        // resolve interpolation order
        let mut interp = interp;
        if interp < 0 {
            interp = *(*m).actuator_history.add(2 * id as usize + 1);
        }

        // get buffer pointer and read from history buffer
        let delay = *(*m).actuator_delay.add(id as usize);
        let buf = (*d).history.add(*(*m).actuator_historyadr.add(id as usize) as usize);
        let mut res: f64 = 0.0;
        let ptr = crate::engine::engine_util_misc::mju_history_read(
            buf, nsample, 1, &mut res, time - delay, interp);
        if !ptr.is_null() { *ptr } else { res }
    }
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
    // SAFETY: m, d are valid; result may be null for non-history case (caller contract)
    unsafe {
        // validate sensor id
        if id < 0 || id >= (*m).nsensor as i32 {
            crate::engine::engine_util_errmem::mju_error(
                b"invalid sensor id %d\0".as_ptr() as *const i8);
            return std::ptr::null();
        }

        // no history: return current sensor value
        let nsample = *(*m).sensor_history.add(2 * id as usize);
        if nsample == 0 {
            return (*d).sensordata.add(*(*m).sensor_adr.add(id as usize) as usize);
        }

        // resolve interpolation order
        let mut interp = interp;
        if interp < 0 {
            interp = *(*m).sensor_history.add(2 * id as usize + 1);
        }

        // get buffer pointer and read from history buffer
        let dim = *(*m).sensor_dim.add(id as usize);
        let delay = *(*m).sensor_delay.add(id as usize);
        let buf = (*d).history.add(*(*m).sensor_historyadr.add(id as usize) as usize);
        crate::engine::engine_util_misc::mju_history_read(
            buf, nsample, dim, result, time - delay, interp)
    }
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
    // SAFETY: m, d are valid pointers (caller contract). times and values may be null.
    unsafe {
        // validate actuator id
        if id < 0 || id as i64 >= (*m).nu {
            crate::engine::engine_util_errmem::mju_error(
                b"invalid actuator id\0".as_ptr() as *const i8);
            return;
        }

        // check that actuator has a history buffer
        let nsample = *(*m).actuator_history.add(2 * id as usize);
        if nsample == 0 {
            crate::engine::engine_util_errmem::mju_error(
                b"actuator has no history buffer\0".as_ptr() as *const i8);
            return;
        }

        // get buffer pointer
        let buf = (*d).history.add(*(*m).actuator_historyadr.add(id as usize) as usize);

        // if times is NULL, use existing buffer times
        let buf_times: *const f64 = if !times.is_null() { times } else { buf.add(2) };

        // get existing user value (preserve it)
        let user = *buf.add(0);

        // initialize history buffer
        crate::engine::engine_util_misc::mju_history_init(buf, nsample, 1, buf_times, values, user);
    }
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
    // SAFETY: m, d are valid pointers (caller contract). times and values may be null.
    unsafe {
        // validate sensor id
        if id < 0 || id as i64 >= (*m).nsensor {
            crate::engine::engine_util_errmem::mju_error(
                b"invalid sensor id\0".as_ptr() as *const i8);
            return;
        }

        // check that sensor has a history buffer
        let nsample = *(*m).sensor_history.add(2 * id as usize);
        if nsample == 0 {
            crate::engine::engine_util_errmem::mju_error(
                b"sensor has no history buffer\0".as_ptr() as *const i8);
            return;
        }

        // get buffer pointer and dimension
        let buf = (*d).history.add(*(*m).sensor_historyadr.add(id as usize) as usize);
        let dim = *(*m).sensor_dim.add(id as usize);

        // if times is NULL, use existing buffer times
        let buf_times: *const f64 = if !times.is_null() { times } else { buf.add(2) as *const f64 };

        // initialize history buffer with provided phase
        crate::engine::engine_util_misc::mju_history_init(buf, nsample, dim, buf_times, values, phase);
    }
}

