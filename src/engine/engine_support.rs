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
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, sig : mjtState)
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
pub fn mj_state_elem_const_ptr(m: *const mjModel, d: *const mjData, sig: mjtState) -> *const f64 {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * const mjData, sig : mjtState)
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
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * const mjData, res : * mut f64, vec : * const f64)
    // Previous return: ()
    extern "C" { fn mj_mulM_impl (m : * const mjModel , d : * const mjData , res : * mut f64 , vec : * const f64) ; } unsafe { mj_mulM_impl (m , d , res , vec) }
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
    extern "C" { fn mj_differentiatePos_impl(m: *const mjModel, qvel: *mut f64, dt: f64, qpos1: *const f64, qpos2: *const f64); }
    // SAFETY: delegates to C implementation
    unsafe { mj_differentiatePos_impl(m, qvel, dt, qpos1, qpos2) }
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
    extern "C" { fn mj_integratePosInd_impl(m: *const mjModel, qpos: *mut f64, qvel: *const f64, dt: f64, index: *const i32, nbody: i32); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_integratePosInd_impl(m, qpos, qvel, dt, index, nbody) }
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
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, i : i32)
    // Previous return: i32
    extern "C" { fn mj_actuatorDisabled_impl (m : * const mjModel , i : i32) -> i32 ; } unsafe { mj_actuatorDisabled_impl (m , i) }
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
    extern "C" {
        fn mju_condataSize_impl(dataSpec: i32) -> i32;
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mju_condataSize_impl(dataSpec) }
}

/// C: mju_raydataSize (engine/engine_support.h:130)
#[allow(unused_variables, non_snake_case)]
pub fn mju_raydata_size(dataspec: i32) -> i32 {
    extern "C" { fn mju_raydataSize_impl(dataspec: i32) -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { mju_raydataSize_impl(dataspec) }
}

/// C: mju_camIntrinsics (engine/engine_support.h:134)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_cam_intrinsics(m: *const mjModel, camid: i32, fx: *mut f64, fy: *mut f64, cx: *mut f64, cy: *mut f64, ortho_extent: *mut f64) {
    extern "C" { fn mju_camIntrinsics_impl(m: *const mjModel, camid: i32, fx: *mut f64, fy: *mut f64, cx: *mut f64, cy: *mut f64, ortho_extent: *mut f64); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mju_camIntrinsics_impl(m, camid, fx, fy, cx, cy, ortho_extent) }
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

