//! Port of: engine/engine_core_util.h
//! IR hash: d3ac8715281cd691
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mj_isPyramidal (engine/engine_core_util.h:31)
#[allow(unused_variables, non_snake_case)]
pub fn mj_is_pyramidal(m: *const mjModel) -> i32 {
    // SAFETY: m is a valid mjModel pointer; cone is i32 at byte offset 988
    unsafe {
        let m_ptr = m as *const u8;
        let cone = *(m_ptr.add(988) as *const i32);
        if cone == 0 { 1 } else { 0 }
    }
}

/// C: mj_isSparse (engine/engine_core_util.h:34)
#[allow(unused_variables, non_snake_case)]
pub fn mj_is_sparse(m: *const mjModel) -> i32 {
    // SAFETY: m is a valid mjModel pointer; jacobian is i32 at offset 992, nv is usize at offset 8
    unsafe {
        let m_ptr = m as *const u8;
        let jacobian = *(m_ptr.add(992) as *const i32);
        let nv = *(m_ptr.add(8) as *const usize);
        if jacobian == 1 || (jacobian == 2 && nv >= 60) { 1 } else { 0 }
    }
}

/// C: mj_mergeChain (engine/engine_core_util.h:40)
#[allow(unused_variables, non_snake_case)]
pub fn mj_merge_chain(m: *const mjModel, chain: *mut i32, b1: i32, b2: i32, flg_skipcommon: i32) -> i32 {
    // SAFETY: m valid, chain valid output. Byte offsets:
    //   body_weldid: *const i32 at 1768, body_dofadr: *const i32 at 1808
    //   body_dofnum: *const i32 at 1800, dof_parentid: *const i32 at 2216
    unsafe {
        let m_ptr = m as *const u8;
        let body_weldid = *(m_ptr.add(1768) as *const *const i32);
        let body_dofadr = *(m_ptr.add(1808) as *const *const i32);
        let body_dofnum = *(m_ptr.add(1800) as *const *const i32);
        let dof_parentid = *(m_ptr.add(2216) as *const *const i32);

        // skip fixed bodies
        let b1w = *body_weldid.add(b1 as usize);
        let b2w = *body_weldid.add(b2 as usize);

        if b1w == 0 && b2w == 0 {
            return 0;
        }

        let mut da1 = *body_dofadr.add(b1w as usize) + *body_dofnum.add(b1w as usize) - 1;
        let mut da2 = *body_dofadr.add(b2w as usize) + *body_dofnum.add(b2w as usize) - 1;
        let mut NV: i32 = 0;

        while da1 >= 0 || da2 >= 0 {
            let da = if da1 > da2 { da1 } else { da2 };
            if flg_skipcommon != 0 && da1 == da && da2 == da {
                break;
            }
            *chain.add(NV as usize) = da;
            if da1 == da {
                da1 = *dof_parentid.add(da1 as usize);
            }
            if da2 == da {
                da2 = *dof_parentid.add(da2 as usize);
            }
            NV += 1;
        }

        // reverse
        for i in 0..(NV / 2) {
            let tmp = *chain.add(i as usize);
            *chain.add(i as usize) = *chain.add((NV - i - 1) as usize);
            *chain.add((NV - i - 1) as usize) = tmp;
        }

        NV
    }
}

/// C: mj_mergeChainSimple (engine/engine_core_util.h:43)
#[allow(unused_variables, non_snake_case)]
pub fn mj_merge_chain_simple(m: *const mjModel, chain: *mut i32, b1: i32, b2: i32) -> i32 {
    // SAFETY: m valid, chain valid. Byte offsets:
    //   body_dofnum: *const i32 at 1800, body_dofadr: *const i32 at 1808
    unsafe {
        let m_ptr = m as *const u8;
        let body_dofnum = *(m_ptr.add(1800) as *const *const i32);
        let body_dofadr = *(m_ptr.add(1808) as *const *const i32);

        // swap bodies if wrong order
        let (b1, b2) = if b1 > b2 { (b2, b1) } else { (b1, b2) };

        let n1 = *body_dofnum.add(b1 as usize);
        let n2 = *body_dofnum.add(b2 as usize);

        if n1 == 0 && n2 == 0 {
            return 0;
        }

        // copy b1 dofs
        for i in 0..n1 {
            *chain.add(i as usize) = *body_dofadr.add(b1 as usize) + i;
        }

        // copy b2 dofs
        for i in 0..n2 {
            *chain.add((n1 + i) as usize) = *body_dofadr.add(b2 as usize) + i;
        }

        n1 + n2
    }
}

/// C: mj_bodyChain (engine/engine_core_util.h:46)
#[allow(unused_variables, non_snake_case)]
pub fn mj_body_chain(m: *const mjModel, body: i32, chain: *mut i32) -> i32 {
    // SAFETY: m valid mjModel, chain valid output array. Byte offsets from IR:
    //   body_simple: *const u8 at offset 1840
    //   body_dofnum: *const i32 at offset 1800
    //   body_dofadr: *const i32 at offset 1808
    //   body_weldid: *const i32 at offset 1768
    //   dof_parentid: *const i32 at offset 2216
    unsafe {
        let m_ptr = m as *const u8;
        let body_simple = *(m_ptr.add(1840) as *const *const u8);
        let body_dofnum = *(m_ptr.add(1800) as *const *const i32);
        let body_dofadr = *(m_ptr.add(1808) as *const *const i32);
        let body_weldid = *(m_ptr.add(1768) as *const *const i32);
        let dof_parentid = *(m_ptr.add(2216) as *const *const i32);

        let b = body as usize;

        // simple body
        if *body_simple.add(b) != 0 {
            let dofnum = *body_dofnum.add(b);
            for i in 0..dofnum {
                *chain.add(i as usize) = *body_dofadr.add(b) + i;
            }
            return dofnum;
        }

        // general case: skip fixed bodies
        let mut body_w = *body_weldid.add(b);

        // not movable: empty chain
        if body_w == 0 {
            return 0;
        }

        // initialize last dof
        let mut da = *body_dofadr.add(body_w as usize) + *body_dofnum.add(body_w as usize) - 1;
        let mut NV: i32 = 0;

        // construct chain from child to parent
        while da >= 0 {
            *chain.add(NV as usize) = da;
            NV += 1;
            da = *dof_parentid.add(da as usize);
        }

        // reverse order of chain: make it increasing
        for i in 0..(NV / 2) {
            let tmp = *chain.add(i as usize);
            *chain.add(i as usize) = *chain.add((NV - i - 1) as usize);
            *chain.add((NV - i - 1) as usize) = tmp;
        }

        NV
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
    // SAFETY: m is valid mjModel, ten_length valid array. Access fields via byte offsets:
    //   tendon_limited: *const u8 at offset 4424
    //   tendon_range: *const f64 at offset 4480
    //   tendon_margin: *const f64 at offset 4496
    unsafe {
        let m_ptr = m as *const u8;
        let tendon_limited = *(m_ptr.add(4424) as *const *const u8);
        let tendon_range = *(m_ptr.add(4480) as *const *const f64);
        let tendon_margin = *(m_ptr.add(4496) as *const *const f64);

        if *tendon_limited.add(i as usize) == 0 {
            return 0;
        }

        let mut nl: i32 = 0;
        let value = *ten_length.add(i as usize);
        let margin = *tendon_margin.add(i as usize);

        // tendon limits can be bilateral, check both sides
        let mut side: i32 = -1;
        while side <= 1 {
            let idx = (2 * i + (side + 1) / 2) as usize;
            let dist = (side as f64) * (*tendon_range.add(idx) - value);
            if dist < margin {
                nl += 1;
            }
            side += 2;
        }

        nl
    }
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

