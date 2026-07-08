//! Port of: engine/engine_support.c
//! IR hash: 05737965add36adb
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mj_stateElemSize (engine/engine_support.c:138)
/// Calls: mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_state_elem_size(m: *const mjModel, sig: mjtState) -> i32 {
    // SAFETY: mjtState is unsigned int in C ABI; read through pointer cast.
    // Model pointer dereferences follow C source semantics.
    unsafe {
        let s: u32 = *(&sig as *const mjtState as *const u32);
        match s {
            1      => 1,                              // mjSTATE_TIME
            2      => (*m).nq as i32,                 // mjSTATE_QPOS
            4      => (*m).nv as i32,                 // mjSTATE_QVEL
            8      => (*m).na as i32,                 // mjSTATE_ACT
            16     => (*m).nhistory as i32,           // mjSTATE_HISTORY
            32     => (*m).nv as i32,                 // mjSTATE_WARMSTART
            64     => (*m).nu as i32,                 // mjSTATE_CTRL
            128    => (*m).nv as i32,                 // mjSTATE_QFRC_APPLIED
            256    => (6 * (*m).nbody) as i32,        // mjSTATE_XFRC_APPLIED
            512    => (*m).neq as i32,                // mjSTATE_EQ_ACTIVE
            1024   => (3 * (*m).nmocap) as i32,      // mjSTATE_MOCAP_POS
            2048   => (4 * (*m).nmocap) as i32,      // mjSTATE_MOCAP_QUAT
            4096   => (*m).nuserdata as i32,          // mjSTATE_USERDATA
            8192   => (*m).npluginstate as i32,       // mjSTATE_PLUGIN
            _ => {
                crate::engine::engine_util_errmem::mju_error(b"invalid state element\0".as_ptr() as *const i8);
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
pub fn mj_state_elem_ptr(m: *const mjModel, d: *mut mjData, sig: mjtState) -> *mut f64 {
    // SAFETY: mjtState is unsigned int in C ABI; read through pointer cast.
    // Data pointer dereferences follow C source semantics.
    unsafe {
        let s: u32 = *(&sig as *const mjtState as *const u32);
        match s {
            1    => &mut (*d).time as *mut f64,           // mjSTATE_TIME
            2    => (*d).qpos,                            // mjSTATE_QPOS
            4    => (*d).qvel,                            // mjSTATE_QVEL
            8    => (*d).act,                             // mjSTATE_ACT
            16   => (*d).history,                         // mjSTATE_HISTORY
            32   => (*d).qacc_warmstart,                  // mjSTATE_WARMSTART
            64   => (*d).ctrl,                            // mjSTATE_CTRL
            128  => (*d).qfrc_applied,                    // mjSTATE_QFRC_APPLIED
            256  => (*d).xfrc_applied as *mut f64,        // mjSTATE_XFRC_APPLIED
            512  => std::ptr::null_mut(),                  // mjSTATE_EQ_ACTIVE (handled separately)
            1024 => (*d).mocap_pos,                       // mjSTATE_MOCAP_POS
            2048 => (*d).mocap_quat,                      // mjSTATE_MOCAP_QUAT
            4096 => (*d).userdata,                        // mjSTATE_USERDATA
            8192 => (*d).plugin_state,                    // mjSTATE_PLUGIN
            _ => {
                crate::engine::engine_util_errmem::mju_error(b"invalid state element\0".as_ptr() as *const i8);
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
pub fn mj_state_elem_const_ptr(m: *const mjModel, d: *const mjData, sig: mjtState) -> *const f64 {
    // SAFETY: casts *const mjData to *mut mjData to reuse mj_state_elem_ptr, matching C source
    // which calls mj_stateElemPtr with a const-cast. Result is returned as *const f64.
    unsafe {
        mj_state_elem_ptr(m, d as *mut mjData, sig) as *const f64
    }
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
    extern "C" { fn mj_geomDistanceCCD_impl(m: *const mjModel, d: *mut mjData, g1: i32, g2: i32, distmax: f64, fromto: *mut f64) -> f64; }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_geomDistanceCCD_impl(m, d, g1, g2, distmax, fromto) }
}

/// C: mj_stateSize (engine/engine_support.h:41)
/// Calls: mj_stateElemSize, mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_state_size(m: *const mjModel, sig: i32) -> i32 {
    const MJ_NSTATE: i32 = 14;
    // SAFETY: m valid per caller contract; mjtState is u32 in C ABI
    unsafe {
        if sig < 0 {
            crate::engine::engine_util_errmem::mju_error(
                b"invalid state signature %d < 0\0".as_ptr() as *const i8);
            return 0;
        }
        if sig >= (1 << MJ_NSTATE) {
            crate::engine::engine_util_errmem::mju_error(
                b"invalid state signature %d >= 2^mjNSTATE\0".as_ptr() as *const i8);
            return 0;
        }
        let mut size: i32 = 0;
        for i in 0..MJ_NSTATE {
            let element: u32 = 1u32 << i;
            if (element as i32 & sig) != 0 {
                let elem_state: mjtState = core::mem::transmute_copy(&element);
                size += mj_state_elem_size(m, elem_state);
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
    const MJ_STATE_EQ_ACTIVE: u32 = 512;
    // SAFETY: m, d, state valid per caller contract
    unsafe {
        if sig < 0 {
            crate::engine::engine_util_errmem::mju_error(
                b"invalid state signature %d < 0\0".as_ptr() as *const i8);
            return;
        }
        if sig >= (1 << MJ_NSTATE) {
            crate::engine::engine_util_errmem::mju_error(
                b"invalid state signature %d >= 2^mjNSTATE\0".as_ptr() as *const i8);
            return;
        }
        let mut adr: i32 = 0;
        for i in 0..MJ_NSTATE {
            let element: u32 = 1u32 << i;
            if (element as i32 & sig) != 0 {
                let elem_state: mjtState = core::mem::transmute_copy(&element);
                let size = mj_state_elem_size(m, elem_state);
                if element == MJ_STATE_EQ_ACTIVE {
                    let neq = (*m).neq as i32;
                    for j in 0..neq {
                        let byte_ptr = (*d).eq_active as *const u8;
                        *state.add(adr as usize) = *byte_ptr.add(j as usize) as f64;
                        adr += 1;
                    }
                } else {
                    let elem_state2: mjtState = core::mem::transmute_copy(&element);
                    let ptr = mj_state_elem_const_ptr(m, d, elem_state2);
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
    // SAFETY: m, src, dst valid per caller contract
    unsafe {
        if srcsig < 0 {
            crate::engine::engine_util_errmem::mju_error(
                b"invalid srcsig %d < 0\0".as_ptr() as *const i8);
            return;
        }
        if srcsig >= (1 << MJ_NSTATE) {
            crate::engine::engine_util_errmem::mju_error(
                b"invalid srcsig %d >= 2^mjNSTATE\0".as_ptr() as *const i8);
            return;
        }
        if (srcsig & dstsig) != dstsig {
            crate::engine::engine_util_errmem::mju_error(
                b"dstsig is not a subset of srcsig\0".as_ptr() as *const i8);
            return;
        }

        let mut src_ptr = src;
        let mut dst_ptr = dst;
        for i in 0..MJ_NSTATE {
            let element: u32 = 1u32 << i;
            if (element as i32 & srcsig) != 0 {
                let elem_state: mjtState = core::mem::transmute_copy(&element);
                let size = mj_state_elem_size(m, elem_state);
                if (element as i32 & dstsig) != 0 {
                    crate::engine::engine_util_blas::mju_copy(dst_ptr, src_ptr, size);
                    dst_ptr = dst_ptr.add(size as usize);
                }
                src_ptr = src_ptr.add(size as usize);
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
    const MJ_STATE_EQ_ACTIVE: u32 = 512;
    // SAFETY: m, d, state valid per caller contract
    unsafe {
        if sig < 0 {
            crate::engine::engine_util_errmem::mju_error(
                b"invalid state signature %d < 0\0".as_ptr() as *const i8);
            return;
        }
        if sig >= (1 << MJ_NSTATE) {
            crate::engine::engine_util_errmem::mju_error(
                b"invalid state signature %d >= 2^mjNSTATE\0".as_ptr() as *const i8);
            return;
        }
        let mut adr: i32 = 0;
        for i in 0..MJ_NSTATE {
            let element: u32 = 1u32 << i;
            if (element as i32 & sig) != 0 {
                let elem_state: mjtState = core::mem::transmute_copy(&element);
                let size = mj_state_elem_size(m, elem_state);
                if element == MJ_STATE_EQ_ACTIVE {
                    let neq = (*m).neq as i32;
                    for j in 0..neq {
                        let byte_ptr = (*d).eq_active as *mut u8;
                        *byte_ptr.add(j as usize) = *state.add(adr as usize) as u8;
                        adr += 1;
                    }
                } else {
                    let elem_state2: mjtState = core::mem::transmute_copy(&element);
                    let ptr = mj_state_elem_ptr(m, d, elem_state2);
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
    const MJ_STATE_EQ_ACTIVE: u32 = 512;
    // SAFETY: m, src, dst valid per caller contract
    unsafe {
        if sig < 0 {
            crate::engine::engine_util_errmem::mju_error(
                b"invalid state signature %d < 0\0".as_ptr() as *const i8);
            return;
        }
        if sig >= (1 << MJ_NSTATE) {
            crate::engine::engine_util_errmem::mju_error(
                b"invalid state signature %d >= 2^mjNSTATE\0".as_ptr() as *const i8);
            return;
        }
        for i in 0..MJ_NSTATE {
            let element: u32 = 1u32 << i;
            if (element as i32 & sig) != 0 {
                let elem_state: mjtState = core::mem::transmute_copy(&element);
                let size = mj_state_elem_size(m, elem_state);
                if element == MJ_STATE_EQ_ACTIVE {
                    let neq = (*m).neq as i32;
                    for j in 0..neq {
                        let dst_byte = (*dst).eq_active as *mut u8;
                        let src_byte = (*src).eq_active as *const u8;
                        *dst_byte.add(j as usize) = *src_byte.add(j as usize);
                    }
                } else {
                    let elem_state2: mjtState = core::mem::transmute_copy(&element);
                    let elem_state3: mjtState = core::mem::transmute_copy(&element);
                    let dst_ptr = mj_state_elem_ptr(m, dst, elem_state2);
                    let src_ptr = mj_state_elem_const_ptr(m, src, elem_state3);
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
    // SAFETY: caller guarantees m and d are valid pointers with correct field layouts
    unsafe {
        // check keyframe index
        if k >= (*m).nkey as i32 {
            crate::engine::engine_util_errmem::mju_error(
                b"index must be smaller than nkey (keyframes allocated in model)\0".as_ptr() as *const i8
            );
        }
        if k < 0 {
            crate::engine::engine_util_errmem::mju_error(
                b"keyframe index cannot be negative\0".as_ptr() as *const i8
            );
        }

        // copy state to model keyframe
        *(*m).key_time.add(k as usize) = (*d).time;
        let nq = (*m).nq as i32;
        let nv = (*m).nv as i32;
        let na = (*m).na as i32;
        let nmocap = (*m).nmocap as i32;
        let nu = (*m).nu as i32;
        crate::engine::engine_util_blas::mju_copy((*m).key_qpos.add((k as usize) * (nq as usize)), (*d).qpos, nq);
        crate::engine::engine_util_blas::mju_copy((*m).key_qvel.add((k as usize) * (nv as usize)), (*d).qvel, nv);
        crate::engine::engine_util_blas::mju_copy((*m).key_act.add((k as usize) * (na as usize)), (*d).act, na);
        crate::engine::engine_util_blas::mju_copy((*m).key_mpos.add((k as usize) * 3 * (nmocap as usize)), (*d).mocap_pos, 3 * nmocap);
        crate::engine::engine_util_blas::mju_copy((*m).key_mquat.add((k as usize) * 4 * (nmocap as usize)), (*d).mocap_quat, 4 * nmocap);
        crate::engine::engine_util_blas::mju_copy((*m).key_ctrl.add((k as usize) * (nu as usize)), (*d).ctrl, nu);
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
    extern "C" { fn mj_fullM_impl(m: *const mjModel, d: *const mjData, dst: *mut f64); }
    // SAFETY: delegates to C implementation
    unsafe { mj_fullM_impl(m, d, dst) }
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
    // SAFETY: m, d are valid model/data pointers. res and vec are valid f64 arrays of size nv.
    unsafe {
        crate::engine::engine_util_sparse::mju_mul_sym_vec_sparse(
            res,
            (*d).M,
            vec,
            (*m).nv as i32,
            (*m).M_rownnz,
            (*m).M_rowadr,
            (*m).M_colind,
        );
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
    extern "C" { fn mj_mulM2_impl(m: *const mjModel, d: *const mjData, res: *mut f64, vec: *const f64); }
    // SAFETY: delegates to C implementation
    unsafe { mj_mulM2_impl(m, d, res, vec) }
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
    extern "C" { fn mj_addM_impl(m: *const mjModel, d: *mut mjData, dst: *mut f64, rownnz: *mut i32, rowadr: *mut i32, colind: *mut i32); }
    // SAFETY: delegates to C implementation
    unsafe { mj_addM_impl(m, d, dst, rownnz, rowadr, colind) }
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
    extern "C" { fn mj_applyFT_impl (m : * const mjModel , d : * mut mjData , force : * const f64 , torque : * const f64 , point : * const f64 , body : i32 , qfrc_target : * mut f64) ; } unsafe { mj_applyFT_impl (m , d , force , torque , point , body , qfrc_target) }
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
    extern "C" { fn mj_xfrcAccumulate_impl (m : * const mjModel , d : * mut mjData , qfrc : * mut f64) ; } unsafe { mj_xfrcAccumulate_impl (m , d , qfrc) }
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
    extern "C" { fn mj_geomDistance_impl(m: *const mjModel, d: *mut mjData, geom1: i32, geom2: i32, distmax: f64, fromto: *mut f64) -> f64; }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_geomDistance_impl(m, d, geom1, geom2, distmax, fromto) }
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
    // SAFETY: m valid. qvel/qpos1/qpos2 have adequate capacity for all joints.
    unsafe {
        const MJJNT_FREE: i32 = 0;
        const MJJNT_BALL: i32 = 1;
        const MJJNT_SLIDE: i32 = 2;
        const MJJNT_HINGE: i32 = 3;

        let njnt = (*m).njnt as i32;
        let mut j: i32 = 0;
        while j < njnt {
            let mut padr = *(*m).jnt_qposadr.add(j as usize);
            let mut vadr = *(*m).jnt_dofadr.add(j as usize);
            let jnt_type = *(*m).jnt_type.add(j as usize);

            if jnt_type == MJJNT_FREE {
                let mut i: i32 = 0;
                while i < 3 {
                    *qvel.add((vadr + i) as usize) = (*qpos2.add((padr + i) as usize) - *qpos1.add((padr + i) as usize)) / dt;
                    i += 1;
                }
                vadr += 3;
                padr += 3;
                // fallthrough to ball
                crate::engine::engine_util_spatial::mju_sub_quat(qvel.add(vadr as usize), qpos2.add(padr as usize), qpos1.add(padr as usize));
                crate::engine::engine_util_blas::mju_scl3(qvel.add(vadr as usize), qvel.add(vadr as usize), 1.0 / dt);
            } else if jnt_type == MJJNT_BALL {
                crate::engine::engine_util_spatial::mju_sub_quat(qvel.add(vadr as usize), qpos2.add(padr as usize), qpos1.add(padr as usize));
                crate::engine::engine_util_blas::mju_scl3(qvel.add(vadr as usize), qvel.add(vadr as usize), 1.0 / dt);
            } else if jnt_type == MJJNT_HINGE || jnt_type == MJJNT_SLIDE {
                *qvel.add(vadr as usize) = (*qpos2.add(padr as usize) - *qpos1.add(padr as usize)) / dt;
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
    // SAFETY: m valid, qpos/qvel have adequate capacity, index may be null.
    unsafe {
        const MJJNT_FREE: i32 = 0;
        const MJJNT_BALL: i32 = 1;
        const MJJNT_SLIDE: i32 = 2;
        const MJJNT_HINGE: i32 = 3;

        let mut b: i32 = 1;
        while b < nbody {
            let k = if !index.is_null() { *index.add(b as usize) } else { b };
            let start = *(*m).body_jntadr.add(k as usize);
            let end = start + *(*m).body_jntnum.add(k as usize);
            let mut j = start;
            while j < end {
                let mut padr = *(*m).jnt_qposadr.add(j as usize);
                let mut vadr = *(*m).jnt_dofadr.add(j as usize);
                let jnt_type = *(*m).jnt_type.add(j as usize);

                if jnt_type == MJJNT_FREE {
                    // position update
                    let mut i: i32 = 0;
                    while i < 3 {
                        *qpos.add((padr + i) as usize) += dt * *qvel.add((vadr + i) as usize);
                        i += 1;
                    }
                    padr += 3;
                    vadr += 3;
                    // fallthrough to ball rotation update
                    crate::engine::engine_util_spatial::mju_quat_integrate(
                        qpos.add(padr as usize), qvel.add(vadr as usize), dt);
                } else if jnt_type == MJJNT_BALL {
                    // quaternion update
                    crate::engine::engine_util_spatial::mju_quat_integrate(
                        qpos.add(padr as usize), qvel.add(vadr as usize), dt);
                } else if jnt_type == MJJNT_HINGE || jnt_type == MJJNT_SLIDE {
                    // scalar update
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
    extern "C" { fn mj_integratePos_impl(m: *const mjModel, qpos: *mut f64, qvel: *const f64, dt: f64); }
    // SAFETY: delegates to C implementation
    unsafe { mj_integratePos_impl(m, qpos, qvel, dt) }
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
    extern "C" { fn mj_normalizeQuat_impl(m: *const mjModel, qpos: *mut f64); }
    // SAFETY: delegates to C implementation
    unsafe { mj_normalizeQuat_impl(m, qpos) }
}

/// C: mj_actuatorDisabled (engine/engine_support.h:108)
#[allow(unused_variables, non_snake_case)]
pub fn mj_actuator_disabled(m: *const mjModel, i: i32) -> i32 {
    // SAFETY: m is a valid mjModel pointer, i is a valid actuator index.
    unsafe {
        let group: i32 = *(*m).actuator_group.add(i as usize);
        if group < 0 || group > 30 {
            0
        } else {
            if (*m).opt.disableactuator & (1 << group) != 0 { 1 } else { 0 }
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
    extern "C" { fn mj_nextActivation_impl (m : * const mjModel , d : * const mjData , actuator_id : i32 , act_adr : i32 , act_dot : f64) -> f64 ; } unsafe { mj_nextActivation_impl (m , d , actuator_id , act_adr , act_dot) }
}

/// C: mj_getTotalmass (engine/engine_support.h:115)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_get_totalmass(m: *const mjModel) -> f64 {
    extern "C" { fn mj_getTotalmass_impl(m: *const mjModel) -> f64; }
    // SAFETY: delegates to C implementation
    unsafe { mj_getTotalmass_impl(m) }
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
    extern "C" { fn mj_setTotalmass_impl(m: *mut mjModel, newmass: f64); }
    // SAFETY: delegates to C implementation
    unsafe { mj_setTotalmass_impl(m, newmass) }
}

/// C: mj_version (engine/engine_support.h:121)
#[allow(unused_variables, non_snake_case)]
pub fn mj_version() -> i32 {
    extern "C" { fn mj_version_impl() -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { mj_version_impl() }
}

/// C: mj_versionString (engine/engine_support.h:124)
#[allow(unused_variables, non_snake_case)]
pub fn mj_version_string() -> *const i8 {
    extern "C" { fn mj_versionString_impl() -> *const i8; }
    // SAFETY: delegates to C implementation
    unsafe { mj_versionString_impl() }
}

/// C: mju_condataSize (engine/engine_support.h:127)
#[allow(unused_variables, non_snake_case)]
pub fn mju_condata_size(dataSpec: i32) -> i32 {
    // SAFETY: pure computation, no pointer access.
    const MJNCONDATA: i32 = 7;
    const MJCONDATA_SIZE: [i32; 7] = [1, 3, 3, 1, 3, 3, 3];
    let mut size: i32 = 0;
    let mut i: i32 = 0;
    while i < MJNCONDATA {
        if (dataSpec & (1 << i)) != 0 {
            size += MJCONDATA_SIZE[i as usize];
        }
        i += 1;
    }
    size
}

/// C: mju_raydataSize (engine/engine_support.h:130)
#[allow(unused_variables, non_snake_case)]
pub fn mju_raydata_size(dataspec: i32) -> i32 {
    // SAFETY: pure computation, no pointer access.
    const MJNRAYDATA: i32 = 6;
    const MJRAYDATA_SIZE: [i32; 6] = [1, 3, 3, 3, 3, 1];
    let mut size: i32 = 0;
    let mut i: i32 = 0;
    while i < MJNRAYDATA {
        if (dataspec & (1 << i)) != 0 {
            size += MJRAYDATA_SIZE[i as usize];
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
    // SAFETY: m valid. camid is a valid camera index. fx/fy/cx/cy/ortho_extent are non-null.
    unsafe {
        const MJPROJ_PERSPECTIVE: i32 = 0;
        const MJPROJ_ORTHOGRAPHIC: i32 = 1;
        const MJPI: f64 = 3.14159265358979323846;

        let width = *(*m).cam_resolution.add(2 * camid as usize);
        let height = *(*m).cam_resolution.add(2 * camid as usize + 1);
        let sensorsize = (*m).cam_sensorsize.add(2 * camid as usize);
        let intrinsic = (*m).cam_intrinsic.add(4 * camid as usize);
        let projection = *(*m).cam_projection.add(camid as usize);

        if projection == MJPROJ_PERSPECTIVE {
            if *sensorsize.add(0) != 0.0 && *sensorsize.add(1) != 0.0 {
                // intrinsic-based perspective camera
                *fx = *intrinsic.add(0) as f64 / *sensorsize.add(0) as f64 * width as f64;
                *fy = *intrinsic.add(1) as f64 / *sensorsize.add(1) as f64 * height as f64;
                *cx = *intrinsic.add(2) as f64 / *sensorsize.add(0) as f64 * width as f64;
                *cy = *intrinsic.add(3) as f64 / *sensorsize.add(1) as f64 * height as f64;
            } else {
                // fovy-based perspective camera
                let val = 0.5 / (*(*m).cam_fovy.add(camid as usize) * MJPI / 360.0).tan() * height as f64;
                *fx = val;
                *fy = val;
                *cx = width as f64 / 2.0;
                *cy = height as f64 / 2.0;
            }
        } else if projection == MJPROJ_ORTHOGRAPHIC {
            // orthographic: normalize pixel offset to [-1, 1]
            *fx = width as f64 / 2.0;
            *fy = height as f64 / 2.0;
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
    extern "C" { fn mj_readCtrl_impl (m : * const mjModel , d : * const mjData , id : i32 , time : f64 , interp : i32) -> f64 ; } unsafe { mj_readCtrl_impl (m , d , id , time , interp) }
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
    extern "C" { fn mj_readSensor_impl(m: *const mjModel, d: *const mjData, id: i32, time: f64, result: *mut f64, interp: i32) -> *const f64; }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_readSensor_impl(m, d, id, time, result, interp) }
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
    extern "C" { fn mj_initCtrlHistory_impl(m: *const mjModel, d: *mut mjData, id: i32, times: *const f64, values: *const f64); }
    // SAFETY: delegates to C implementation
    unsafe { mj_initCtrlHistory_impl(m, d, id, times, values) }
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
    extern "C" { fn mj_initSensorHistory_impl(m: *const mjModel, d: *mut mjData, id: i32, times: *const f64, values: *const f64, phase: f64); }
    // SAFETY: delegates to C implementation
    unsafe { mj_initSensorHistory_impl(m, d, id, times, values, phase) }
}

