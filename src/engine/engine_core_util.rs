//! Port of: engine/engine_core_util.h
//! IR hash: c6d98e4f4b63b7f2
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mj_isPyramidal (engine/engine_core_util.h:31)
#[allow(unused_variables, non_snake_case)]
pub fn mj_is_pyramidal(m: *const mjModel) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (m : * const mjModel)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: mj_isSparse (engine/engine_core_util.h:34)
#[allow(unused_variables, non_snake_case)]
pub fn mj_is_sparse(m: *const mjModel) -> i32 {
    // mjModel offsets (verified via offsetof):
    //   nv: offset 8 (mjtSize = i64)
    //   opt.jacobian: offset 736 + 256 = 992 (int = i32)
    // enum values: mjJAC_SPARSE = 1, mjJAC_AUTO = 2
    const OFFSET_NV: usize = 8;
    const OFFSET_OPT_JACOBIAN: usize = 736 + 256; // opt at 736, jacobian at 256 within opt
    const MJ_JAC_SPARSE: i32 = 1;
    const MJ_JAC_AUTO: i32 = 2;

    // SAFETY: m is a valid pointer to mjModel (5512 bytes); we read fields at known offsets
    // derived from the C struct layout (verified with offsetof).
    unsafe {
        let base = m as *const u8;
        let jacobian = *(base.add(OFFSET_OPT_JACOBIAN) as *const i32);
        let nv = *(base.add(OFFSET_NV) as *const i64);

        if jacobian == MJ_JAC_SPARSE || (jacobian == MJ_JAC_AUTO && nv >= 60) {
            1
        } else {
            0
        }
    }
}

/// C: mj_mergeChain (engine/engine_core_util.h:40)
#[allow(unused_variables, non_snake_case)]
pub fn mj_merge_chain(m: *const mjModel, chain: *mut i32, b1: i32, b2: i32, flg_skipcommon: i32) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (m : * const mjModel, chain : * mut i32, b1 : i32, b2 : i32, flg_skipcommon : i32)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: mj_mergeChainSimple (engine/engine_core_util.h:43)
#[allow(unused_variables, non_snake_case)]
pub fn mj_merge_chain_simple(m: *const mjModel, chain: *mut i32, b1: i32, b2: i32) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (m : * const mjModel, chain : * mut i32, b1 : i32, b2 : i32)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: mj_bodyChain (engine/engine_core_util.h:46)
#[allow(unused_variables, non_snake_case)]
pub fn mj_body_chain(m: *const mjModel, mut body: i32, chain: *mut i32) -> i32 {
    // mjModel field offsets (verified via offsetof):
    const OFF_BODY_WELDID: usize = 1768;    // int* body_weldid
    const OFF_BODY_DOFNUM: usize = 1800;    // int* body_dofnum
    const OFF_BODY_DOFADR: usize = 1808;    // int* body_dofadr
    const OFF_BODY_SIMPLE: usize = 1840;    // int* body_simple
    const OFF_DOF_PARENTID: usize = 2216;   // int* dof_parentid

    // SAFETY: m is a valid mjModel pointer. All fields are int* pointers within the struct
    // at known offsets. body is a valid index into these arrays.
    unsafe {
        let base = m as *const u8;
        let body_simple = *(base.add(OFF_BODY_SIMPLE) as *const *const i32);
        let body_dofnum = *(base.add(OFF_BODY_DOFNUM) as *const *const i32);
        let body_dofadr = *(base.add(OFF_BODY_DOFADR) as *const *const i32);
        let body_weldid = *(base.add(OFF_BODY_WELDID) as *const *const i32);
        let dof_parentid = *(base.add(OFF_DOF_PARENTID) as *const *const i32);

        // simple body
        if *body_simple.add(body as usize) != 0 {
            let dofnum = *body_dofnum.add(body as usize);
            let dofadr = *body_dofadr.add(body as usize);
            for i in 0..dofnum {
                *chain.add(i as usize) = dofadr + i;
            }
            return dofnum;
        }

        // general case: skip fixed bodies
        body = *body_weldid.add(body as usize);

        // not movable: empty chain
        if body == 0 {
            return 0;
        }

        // initialize last dof
        let mut da = *body_dofadr.add(body as usize) + *body_dofnum.add(body as usize) - 1;
        let mut nv = 0i32;

        // construct chain from child to parent
        while da >= 0 {
            *chain.add(nv as usize) = da;
            nv += 1;
            da = *dof_parentid.add(da as usize);
        }

        // reverse order of chain: make it increasing
        for i in 0..nv / 2 {
            let tmp = *chain.add(i as usize);
            *chain.add(i as usize) = *chain.add((nv - i - 1) as usize);
            *chain.add((nv - i - 1) as usize) = tmp;
        }

        nv
    }
}

/// C: mj_jac (engine/engine_core_util.h:52)
/// Calls: mji_cross, mju_sub3, mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_jac(m: *const mjModel, d: *const mjData, jacp: *mut f64, jacr: *mut f64, point: *const f64, body: i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (m : * const mjModel, d : * const mjData, jacp : * mut f64, jacr : * mut f64, point : * const f64, body : i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mj_jacBody (engine/engine_core_util.h:56)
/// Calls: mj_jac
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_jac_body(m: *const mjModel, d: *const mjData, jacp: *mut f64, jacr: *mut f64, body: i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (m : * const mjModel, d : * const mjData, jacp : * mut f64, jacr : * mut f64, body : i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mj_jacBodyCom (engine/engine_core_util.h:60)
/// Calls: mj_jac
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_jac_body_com(m: *const mjModel, d: *const mjData, jacp: *mut f64, jacr: *mut f64, body: i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (m : * const mjModel, d : * const mjData, jacp : * mut f64, jacr : * mut f64, body : i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mj_jacSubtreeCom (engine/engine_core_util.h:64)
/// Calls: mj_freeStack, mj_jac, mj_markStack, mj_stackAllocInfo, mju_addToScl, mju_scl, mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_jac_subtree_com(m: *const mjModel, d: *mut mjData, jacp: *mut f64, body: i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (m : * const mjModel, d : * mut mjData, jacp : * mut f64, body : i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mj_jacGeom (engine/engine_core_util.h:67)
/// Calls: mj_jac
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_jac_geom(m: *const mjModel, d: *const mjData, jacp: *mut f64, jacr: *mut f64, geom: i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (m : * const mjModel, d : * const mjData, jacp : * mut f64, jacr : * mut f64, geom : i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mj_jacSite (engine/engine_core_util.h:71)
/// Calls: mj_jac
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_jac_site(m: *const mjModel, d: *const mjData, jacp: *mut f64, jacr: *mut f64, site: i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (m : * const mjModel, d : * const mjData, jacp : * mut f64, jacr : * mut f64, site : i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mj_jacPointAxis (engine/engine_core_util.h:75)
/// Calls: mj_freeStack, mj_jac, mj_markStack, mj_stackAllocInfo
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_jac_point_axis(m: *const mjModel, d: *mut mjData, jacPoint: *mut f64, jacAxis: *mut f64, point: *const f64, axis: *const f64, body: i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (m : * const mjModel, d : * mut mjData, jacPoint : * mut f64, jacAxis : * mut f64, point : * const f64, axis : * const f64, body : i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mj_jacSparse (engine/engine_core_util.h:80)
/// Calls: mji_cross, mju_message, mju_sub3, mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_jac_sparse(m: *const mjModel, d: *const mjData, jacp: *mut f64, jacr: *mut f64, point: *const f64, body: i32, NV: i32, chain: *const i32, flg_skipcommon: i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (m : * const mjModel, d : * const mjData, jacp : * mut f64, jacr : * mut f64, point : * const f64, body : i32, NV : i32, chain : * const i32, flg_skipcommon : i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mj_jacSparseSimple (engine/engine_core_util.h:85)
/// Calls: mji_cross, mju_sub3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_jac_sparse_simple(m: *const mjModel, d: *const mjData, jacdifp: *mut f64, jacdifr: *mut f64, point: *const f64, body: i32, flg_second: i32, NV: i32, start: i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (m : * const mjModel, d : * const mjData, jacdifp : * mut f64, jacdifr : * mut f64, point : * const f64, body : i32, flg_second : i32, NV : i32, start : i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mj_jacDotSparse (engine/engine_core_util.h:90)
/// Calls: mji_copy6, mji_cross, mji_crossMotion, mju_message, mju_sub3, mju_transformSpatial, mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_jac_dot_sparse(m: *const mjModel, d: *const mjData, jacp: *mut f64, jacr: *mut f64, point: *const f64, body: i32, NV: i32, chain: *const i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (m : * const mjModel, d : * const mjData, jacp : * mut f64, jacr : * mut f64, point : * const f64, body : i32, NV : i32, chain : * const i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mj_jacDifPair (engine/engine_core_util.h:95)
/// Calls: mj_jac, mj_jacSparse, mj_jacSparseSimple, mj_mergeChain, mj_mergeChainSimple, mju_sub
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_jac_dif_pair(m: *const mjModel, d: *const mjData, chain: *mut i32, b1: i32, b2: i32, pos1: *const f64, pos2: *const f64, jac1p: *mut f64, jac2p: *mut f64, jacdifp: *mut f64, jac1r: *mut f64, jac2r: *mut f64, jacdifr: *mut f64, issparse: i32, flg_skipcommon: i32) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (m : * const mjModel, d : * const mjData, chain : * mut i32, b1 : i32, b2 : i32, pos1 : * const f64, pos2 : * const f64, jac1p : * mut f64, jac2p : * mut f64, jacdifp : * mut f64, jac1r : * mut f64, jac2r : * mut f64, jacdifr : * mut f64, issparse : i32, flg_skipcommon : i32)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: mj_jacSum (engine/engine_core_util.h:102)
/// Calls: mj_bodyChain, mj_freeStack, mj_isSparse, mj_jac, mj_jacSparse, mj_jacSparseSimple, mj_markStack, mj_stackAllocInfo, mju_addToScl, mju_addToSparseMat, mju_scl
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_jac_sum(m: *const mjModel, d: *mut mjData, chain: *mut i32, n: i32, body: *const i32, weight: *const f64, point: *const f64, jac: *mut f64, flg_rot: i32) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (m : * const mjModel, d : * mut mjData, chain : * mut i32, n : i32, body : * const i32, weight : * const f64, point : * const f64, jac : * mut f64, flg_rot : i32)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: mj_jacDot (engine/engine_core_util.h:107)
/// Calls: mji_copy6, mji_cross, mji_crossMotion, mju_sub3, mju_transformSpatial, mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_jac_dot(m: *const mjModel, d: *const mjData, jacp: *mut f64, jacr: *mut f64, point: *const f64, body: i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (m : * const mjModel, d : * const mjData, jacp : * mut f64, jacr : * mut f64, point : * const f64, body : i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mj_angmomMat (engine/engine_core_util.h:111)
/// Calls: mj_freeStack, mj_jacBodyCom, mj_markStack, mj_stackAllocInfo, mji_copy9, mji_mulMatMat3, mji_sub3, mju_addTo, mju_copy3, mju_mulMatMat, mju_mulMatMatT3, mju_scl, mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_angmom_mat(m: *const mjModel, d: *mut mjData, mat: *mut f64, body: i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (m : * const mjModel, d : * mut mjData, mat : * mut f64, body : i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mj_objectVelocity (engine/engine_core_util.h:117)
/// Calls: mju_message, mju_transformSpatial, mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_object_velocity(m: *const mjModel, d: *const mjData, objtype: i32, objid: i32, res: *mut f64, flg_local: i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (m : * const mjModel, d : * const mjData, objtype : i32, objid : i32, res : * mut f64, flg_local : i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mj_objectAcceleration (engine/engine_core_util.h:121)
/// Calls: mji_addTo3, mji_cross, mju_message, mju_transformSpatial, mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_object_acceleration(m: *const mjModel, d: *const mjData, objtype: i32, objid: i32, res: *mut f64, flg_local: i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (m : * const mjModel, d : * const mjData, objtype : i32, objid : i32, res : * mut f64, flg_local : i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mj_local2Global (engine/engine_core_util.h:125)
/// Calls: mji_addTo3, mji_copy3, mji_copy9, mji_mulMatVec3, mji_mulQuat, mju_quat2Mat
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_local2global(d: *mut mjData, xpos: *mut f64, xmat: *mut f64, pos: *const f64, quat: *const f64, body: i32, sameframe: u8) {
    // NOTE: signature changed from previous IR version
    // Previous params: (d : * mut mjData, xpos : * mut f64, xmat : * mut f64, pos : * const f64, quat : * const f64, body : i32, sameframe : u8)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mju_flexGatherState (engine/engine_core_util.h:133)
/// Calls: mj_objectVelocity, mju_addTo3, mju_copy3, mju_cross, mju_mulMatVec3, mju_shellTrackInterior, mju_sub3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_flex_gather_state(m: *const mjModel, d: *const mjData, f: i32, xpos: *mut f64, vel: *mut f64) {
    // NOTE: signature changed from previous IR version
    // Previous params: (m : * const mjModel, d : * const mjData, f : i32, xpos : * mut f64, vel : * mut f64)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mj_contactForce (engine/engine_core_util.h:136)
/// Calls: mj_isPyramidal, mju_copy, mju_decodePyramid, mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_contact_force(m: *const mjModel, d: *const mjData, id: i32, result: *mut f64) {
    // NOTE: signature changed from previous IR version
    // Previous params: (m : * const mjModel, d : * const mjData, id : i32, result : * mut f64)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: tendonLimit (engine/engine_core_util.h:139)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn tendon_limit(m: *const mjModel, ten_length: *const f64, i: i32) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (m : * const mjModel, ten_length : * const f64, i : i32)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: mj_actuatorDamping (engine/engine_core_util.h:142)
/// Calls: mju_message
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_actuator_damping(m: *const mjModel, r#type: u32, id: i32, poly: *mut f64) -> f64 {
    // NOTE: signature changed from previous IR version
    // Previous params: (m : * const mjModel, r#type : u32, id : i32, poly : * mut f64)
    // Previous return: f64
    todo!("re-translate: params renamed")
}

/// C: mj_actuatorArmature (engine/engine_core_util.h:145)
/// Calls: mju_message
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_actuator_armature(m: *const mjModel, r#type: u32, id: i32) -> f64 {
    // NOTE: signature changed from previous IR version
    // Previous params: (m : * const mjModel, r#type : u32, id : i32)
    // Previous return: f64
    todo!("re-translate: params renamed")
}

/// C: mj_warning (engine/engine_core_util.h:148)
/// Calls: mju_message, mju_warning, mju_warningText
#[allow(unused_variables, non_snake_case)]
pub fn mj_warning(d: *mut mjData, warning: i32, info: i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (d : * mut mjData, warning : i32, info : i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

