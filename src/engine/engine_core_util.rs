//! Port of: engine/engine_core_util.h
//! IR hash: c6d98e4f4b63b7f2
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mj_isPyramidal (engine/engine_core_util.h:31)
#[allow(unused_variables, non_snake_case)]
pub fn mj_is_pyramidal(m: *const mjModel) -> i32 {
    // SAFETY: caller guarantees m points to a valid mjModel
    unsafe {
        const mjCONE_PYRAMIDAL: i32 = 0;
        if (*m).opt.cone == mjCONE_PYRAMIDAL {
            1
        } else {
            0
        }
    }
}

/// C: mj_isSparse (engine/engine_core_util.h:34)
#[allow(unused_variables, non_snake_case)]
pub fn mj_is_sparse(m: *const mjModel) -> i32 {
    // SAFETY: caller guarantees m points to a valid mjModel
    unsafe {
        const mjJAC_SPARSE: i32 = 1;
        const mjJAC_AUTO: i32 = 2;
        if (*m).opt.jacobian == mjJAC_SPARSE
            || ((*m).opt.jacobian == mjJAC_AUTO && (*m).nv >= 60)
        {
            1
        } else {
            0
        }
    }
}

/// C: mj_mergeChain (engine/engine_core_util.h:40)
#[allow(unused_variables, non_snake_case)]
pub fn mj_merge_chain(m: *const mjModel, chain: *mut i32, b1: i32, b2: i32, flg_skipcommon: i32) -> i32 {
    // SAFETY: caller guarantees m points to a valid mjModel, chain has sufficient capacity,
    // b1/b2 are valid body indices.
    unsafe {
        let mut NV: i32 = 0;

        // skip fixed bodies
        let b1 = *(*m).body_weldid.add(b1 as usize);
        let b2 = *(*m).body_weldid.add(b2 as usize);

        // neither body is movable: empty chain
        if b1 == 0 && b2 == 0 {
            return 0;
        }

        // initialize last dof address for each body
        let mut da1: i32 = *(*m).body_dofadr.add(b1 as usize) + *(*m).body_dofnum.add(b1 as usize) - 1;
        let mut da2: i32 = *(*m).body_dofadr.add(b2 as usize) + *(*m).body_dofnum.add(b2 as usize) - 1;

        // merge chains
        while da1 >= 0 || da2 >= 0 {
            let da: i32 = if da1 > da2 { da1 } else { da2 };
            if flg_skipcommon != 0 && da1 == da && da2 == da {
                break;
            }
            *chain.add(NV as usize) = da;
            if da1 == da {
                da1 = *(*m).dof_parentid.add(da1 as usize);
            }
            if da2 == da {
                da2 = *(*m).dof_parentid.add(da2 as usize);
            }
            NV += 1;
        }

        // reverse order of chain: make it increasing
        let mut i: i32 = 0;
        while i < NV / 2 {
            let tmp = *chain.add(i as usize);
            *chain.add(i as usize) = *chain.add((NV - i - 1) as usize);
            *chain.add((NV - i - 1) as usize) = tmp;
            i += 1;
        }

        NV
    }
}

/// C: mj_mergeChainSimple (engine/engine_core_util.h:43)
#[allow(unused_variables, non_snake_case)]
pub fn mj_merge_chain_simple(m: *const mjModel, chain: *mut i32, b1: i32, b2: i32) -> i32 {
    // SAFETY: caller guarantees m points to a valid mjModel, chain has sufficient capacity,
    // b1/b2 are valid body indices.
    unsafe {
        // swap bodies if wrong order
        let mut b1 = b1;
        let mut b2 = b2;
        if b1 > b2 {
            let tmp = b1;
            b1 = b2;
            b2 = tmp;
        }

        // init
        let n1: i32 = *(*m).body_dofnum.add(b1 as usize);
        let n2: i32 = *(*m).body_dofnum.add(b2 as usize);

        // both fixed: nothing to do
        if n1 == 0 && n2 == 0 {
            return 0;
        }

        // copy b1 dofs
        let mut i: i32 = 0;
        while i < n1 {
            *chain.add(i as usize) = *(*m).body_dofadr.add(b1 as usize) + i;
            i += 1;
        }

        // copy b2 dofs
        i = 0;
        while i < n2 {
            *chain.add((n1 + i) as usize) = *(*m).body_dofadr.add(b2 as usize) + i;
            i += 1;
        }

        n1 + n2
    }
}

/// C: mj_bodyChain (engine/engine_core_util.h:46)
#[allow(unused_variables, non_snake_case)]
pub fn mj_body_chain(m: *const mjModel, body: i32, chain: *mut i32) -> i32 {
    // SAFETY: caller guarantees m points to a valid mjModel, chain has sufficient capacity,
    // body is a valid body index.
    unsafe {
        // simple body
        if *(*m).body_simple.add(body as usize) != 0 {
            let dofnum: i32 = *(*m).body_dofnum.add(body as usize);
            let mut i: i32 = 0;
            while i < dofnum {
                *chain.add(i as usize) = *(*m).body_dofadr.add(body as usize) + i;
                i += 1;
            }
            return dofnum;
        }

        // general case
        // skip fixed bodies
        let body = *(*m).body_weldid.add(body as usize);

        // not movable: empty chain
        if body == 0 {
            return 0;
        }

        // initialize last dof
        let mut da: i32 = *(*m).body_dofadr.add(body as usize) + *(*m).body_dofnum.add(body as usize) - 1;
        let mut NV: i32 = 0;

        // construct chain from child to parent
        while da >= 0 {
            *chain.add(NV as usize) = da;
            NV += 1;
            da = *(*m).dof_parentid.add(da as usize);
        }

        // reverse order of chain: make it increasing
        let mut i: i32 = 0;
        while i < NV / 2 {
            let tmp = *chain.add(i as usize);
            *chain.add(i as usize) = *chain.add((NV - i - 1) as usize);
            *chain.add((NV - i - 1) as usize) = tmp;
            i += 1;
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
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * const mjData, jacp : * mut f64, jacr : * mut f64, point : * const f64, body : i32)
    // Previous return: ()
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * const mjData, jacp : * mut f64, jacr : * mut f64, body : i32)
    // Previous return: ()
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * const mjData, jacp : * mut f64, jacr : * mut f64, body : i32)
    // Previous return: ()
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, jacp : * mut f64, body : i32)
    // Previous return: ()
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * const mjData, jacp : * mut f64, jacr : * mut f64, geom : i32)
    // Previous return: ()
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * const mjData, jacp : * mut f64, jacr : * mut f64, site : i32)
    // Previous return: ()
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, jacPoint : * mut f64, jacAxis : * mut f64, point : * const f64, axis : * const f64, body : i32)
    // Previous return: ()
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * const mjData, jacp : * mut f64, jacr : * mut f64, point : * const f64, body : i32, NV : i32, chain : * const i32, flg_skipcommon : i32)
    // Previous return: ()
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * const mjData, jacdifp : * mut f64, jacdifr : * mut f64, point : * const f64, body : i32, flg_second : i32, NV : i32, start : i32)
    // Previous return: ()
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * const mjData, jacp : * mut f64, jacr : * mut f64, point : * const f64, body : i32, NV : i32, chain : * const i32)
    // Previous return: ()
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * const mjData, chain : * mut i32, b1 : i32, b2 : i32, pos1 : * const f64, pos2 : * const f64, jac1p : * mut f64, jac2p : * mut f64, jacdifp : * mut f64, jac1r : * mut f64, jac2r : * mut f64, jacdifr : * mut f64, issparse : i32, flg_skipcommon : i32)
    // Previous return: i32
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, chain : * mut i32, n : i32, body : * const i32, weight : * const f64, point : * const f64, jac : * mut f64, flg_rot : i32)
    // Previous return: i32
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * const mjData, jacp : * mut f64, jacr : * mut f64, point : * const f64, body : i32)
    // Previous return: ()
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, mat : * mut f64, body : i32)
    // Previous return: ()
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * const mjData, objtype : i32, objid : i32, res : * mut f64, flg_local : i32)
    // Previous return: ()
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * const mjData, objtype : i32, objid : i32, res : * mut f64, flg_local : i32)
    // Previous return: ()
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (d : * mut mjData, xpos : * mut f64, xmat : * mut f64, pos : * const f64, quat : * const f64, body : i32, sameframe : u8)
    // Previous return: ()
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * const mjData, f : i32, xpos : * mut f64, vel : * mut f64)
    // Previous return: ()
    todo ! ()
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
    use crate::engine::engine_util_blas::{mju_zero, mju_copy};
    use crate::engine::engine_util_misc::mju_decode_pyramid;
    // SAFETY: caller guarantees m, d are valid model/data, result points to 6 f64
    unsafe {
        // clear result
        mju_zero(result, 6);

        // make sure contact is valid
        if id >= 0 && id < (*d).ncon && (*(*d).contact.add(id as usize)).efc_address >= 0 {
            // get contact pointer
            let con = &*(*d).contact.add(id as usize);

            if mj_is_pyramidal(m) != 0 {
                mju_decode_pyramid(result, (*d).efc_force.add(con.efc_address as usize), con.friction.as_ptr(), con.dim);
            } else {
                mju_copy(result, (*d).efc_force.add(con.efc_address as usize), con.dim);
            }
        }
    }
}

/// C: tendonLimit (engine/engine_core_util.h:139)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn tendon_limit(m: *const mjModel, ten_length: *const f64, i: i32) -> i32 {
    // SAFETY: caller guarantees m points to a valid mjModel, ten_length is valid array,
    // i is a valid tendon index.
    unsafe {
        // tendon_limited is *mut mjtBool which is effectively *mut u8
        let limited_ptr = (*m).tendon_limited as *const u8;
        if *limited_ptr.add(i as usize) == 0 {
            return 0;
        }

        let mut nl: i32 = 0;
        let value: f64 = *ten_length.add(i as usize);
        let margin: f64 = *(*m).tendon_margin.add(i as usize);

        // tendon limits can be bilateral, check both sides
        let mut side: i32 = -1;
        while side <= 1 {
            let range_idx: usize = (2 * i + (side + 1) / 2) as usize;
            let dist: f64 = (side as f64) * (*(*m).tendon_range.add(range_idx) - value);
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
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, r#type : u32, id : i32, poly : * mut f64)
    // Previous return: f64
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, r#type : u32, id : i32)
    // Previous return: f64
    todo ! ()
}

/// C: mj_warning (engine/engine_core_util.h:148)
/// Calls: mju_message, mju_warning, mju_warningText
#[allow(unused_variables, non_snake_case)]
pub fn mj_warning(d: *mut mjData, warning: i32, info: i32) {
    // WARNING: signature changed — verify body
    // Previous params: (d : * mut mjData, warning : i32, info : i32)
    // Previous return: ()
    todo ! ()
}

