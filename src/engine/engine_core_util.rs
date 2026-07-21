//! Port of: engine/engine_core_util.h
//! IR hash: 73a9f665ec0ecfc0
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mj_isPyramidal (engine/engine_core_util.h:31)
/// Calls: cone
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
/// Calls: FilePath::empty
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
/// Calls: FilePath::empty
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
    // SAFETY: caller guarantees m, d, point valid; jacp/jacr point to 3*nv arrays if non-null
    unsafe {
        let nv = (*m).nv as i32;
        let mut offset: [f64; 3] = [0.0; 3];

        // clear jacobians, compute offset if required
        if !jacp.is_null() {
            crate::engine::engine_util_blas::mju_zero(jacp, 3 * nv);
            crate::engine::engine_util_blas::mju_sub3(
                offset.as_mut_ptr(), point,
                (*d).subtree_com.add(3 * *(*m).body_rootid.add(body as usize) as usize));
        }
        if !jacr.is_null() {
            crate::engine::engine_util_blas::mju_zero(jacr, 3 * nv);
        }

        // skip fixed bodies
        let body = *(*m).body_weldid.add(body as usize);

        // no movable body found
        if body == 0 {
            return;
        }

        // get last dof that affects this body
        let mut i = *(*m).body_dofadr.add(body as usize) + *(*m).body_dofnum.add(body as usize) - 1;

        // backward pass over dof ancestor chain
        while i >= 0 {
            let cdof = (*d).cdof.add(6 * i as usize);

            // construct rotation jacobian
            if !jacr.is_null() {
                *jacr.add((i + 0 * nv) as usize) = *cdof.add(0);
                *jacr.add((i + 1 * nv) as usize) = *cdof.add(1);
                *jacr.add((i + 2 * nv) as usize) = *cdof.add(2);
            }

            // construct translation jacobian (correct for rotation)
            if !jacp.is_null() {
                let mut tmp: [f64; 3] = [0.0; 3];
                crate::engine::engine_inline::mji_cross(tmp.as_mut_ptr(), cdof, offset.as_ptr());
                *jacp.add((i + 0 * nv) as usize) = *cdof.add(3) + tmp[0];
                *jacp.add((i + 1 * nv) as usize) = *cdof.add(4) + tmp[1];
                *jacp.add((i + 2 * nv) as usize) = *cdof.add(5) + tmp[2];
            }

            // advance to parent dof
            i = *(*m).dof_parentid.add(i as usize);
        }
    }
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
    // SAFETY: d->xpos is a valid pointer to body positions (caller contract)
    unsafe {
        mj_jac(m, d, jacp, jacr, (*d).xpos.add(3 * body as usize), body);
    }
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
    // SAFETY: caller guarantees m, d valid; xipos indexed by body
    unsafe {
        mj_jac(m, d, jacp, jacr, (*d).xipos.add(3 * body as usize), body);
    }
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
    todo!() // mj_jacSubtreeCom
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
    // SAFETY: caller guarantees m, d valid; geom_xpos and geom_bodyid indexed by geom
    unsafe {
        mj_jac(m, d, jacp, jacr, (*d).geom_xpos.add(3 * geom as usize), *(*m).geom_bodyid.add(geom as usize));
    }
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
    // SAFETY: caller guarantees m, d valid; site_xpos and site_bodyid indexed by site
    unsafe {
        mj_jac(m, d, jacp, jacr, (*d).site_xpos.add(3 * site as usize), *(*m).site_bodyid.add(site as usize));
    }
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
    todo!() // mj_jacPointAxis
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
    // SAFETY: m, d, point, chain are valid pointers; jacp/jacr may be null (caller contract)
    unsafe {
        // clear jacobians
        if !jacp.is_null() {
            crate::engine::engine_util_blas::mju_zero(jacp, 3 * NV);
        }
        if !jacr.is_null() {
            crate::engine::engine_util_blas::mju_zero(jacr, 3 * NV);
        }

        // compute point-com offset
        let mut offset: [f64; 3] = [0.0; 3];
        crate::engine::engine_util_blas::mju_sub3(
            offset.as_mut_ptr(),
            point,
            (*d).subtree_com.add(3 * (*(*m).body_rootid.add(body as usize)) as usize),
        );

        // skip fixed bodies
        let body = *(*m).body_weldid.add(body as usize);

        // no movable body found
        if body == 0 {
            return;
        }

        // get last dof that affects this body
        let mut da = *(*m).body_dofadr.add(body as usize) + *(*m).body_dofnum.add(body as usize) - 1;

        // start at end of chain
        let mut ci = NV - 1;

        // backward pass over dof ancestor chain
        while da >= 0 {
            // find chain index for this dof
            while ci >= 0 && *chain.add(ci as usize) > da {
                ci -= 1;
            }

            // dof not in chain
            if ci < 0 || *chain.add(ci as usize) != da {
                if flg_skipcommon != 0 {
                    da = *(*m).dof_parentid.add(da as usize);
                    continue;
                }
                crate::engine::engine_util_errmem::mju_error(
                    b"dof index %d not found in chain\0".as_ptr() as *const i8);
            }

            let cdof = (*d).cdof.add(6 * da as usize);

            // rotation jacobian
            if !jacr.is_null() {
                *jacr.add(ci as usize + 0 * NV as usize) = *cdof.add(0);
                *jacr.add(ci as usize + 1 * NV as usize) = *cdof.add(1);
                *jacr.add(ci as usize + 2 * NV as usize) = *cdof.add(2);
            }

            // translation jacobian (correct for rotation)
            if !jacp.is_null() {
                let mut tmp: [f64; 3] = [0.0; 3];
                crate::engine::engine_inline::mji_cross(tmp.as_mut_ptr(), cdof, offset.as_ptr());
                *jacp.add(ci as usize + 0 * NV as usize) = *cdof.add(3) + tmp[0];
                *jacp.add(ci as usize + 1 * NV as usize) = *cdof.add(4) + tmp[1];
                *jacp.add(ci as usize + 2 * NV as usize) = *cdof.add(5) + tmp[2];
            }

            // advance to parent dof
            da = *(*m).dof_parentid.add(da as usize);
        }
    }
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
    // SAFETY: caller guarantees all pointers valid with proper sizes
    unsafe {
        // compute point-com offset
        let mut offset: [f64; 3] = [0.0; 3];
        crate::engine::engine_util_blas::mju_sub3(
            offset.as_mut_ptr(), point,
            (*d).subtree_com.add(3 * *(*m).body_rootid.add(body as usize) as usize));

        // skip fixed body
        if *(*m).body_dofnum.add(body as usize) == 0 {
            return;
        }

        // process dofs
        let mut ci = start;
        let end = *(*m).body_dofadr.add(body as usize) + *(*m).body_dofnum.add(body as usize);
        let mut da = *(*m).body_dofadr.add(body as usize);
        while da < end {
            let cdof = (*d).cdof.add(6 * da as usize);

            // construct rotation jacobian
            if !jacdifr.is_null() {
                if flg_second != 0 {
                    *jacdifr.add((ci + 0 * NV) as usize) = *cdof.add(0);
                    *jacdifr.add((ci + 1 * NV) as usize) = *cdof.add(1);
                    *jacdifr.add((ci + 2 * NV) as usize) = *cdof.add(2);
                } else {
                    *jacdifr.add((ci + 0 * NV) as usize) = -*cdof.add(0);
                    *jacdifr.add((ci + 1 * NV) as usize) = -*cdof.add(1);
                    *jacdifr.add((ci + 2 * NV) as usize) = -*cdof.add(2);
                }
            }

            // construct translation jacobian (correct for rotation)
            if !jacdifp.is_null() {
                let mut tmp: [f64; 3] = [0.0; 3];
                crate::engine::engine_inline::mji_cross(tmp.as_mut_ptr(), cdof, offset.as_ptr());

                if flg_second != 0 {
                    *jacdifp.add((ci + 0 * NV) as usize) = *cdof.add(3) + tmp[0];
                    *jacdifp.add((ci + 1 * NV) as usize) = *cdof.add(4) + tmp[1];
                    *jacdifp.add((ci + 2 * NV) as usize) = *cdof.add(5) + tmp[2];
                } else {
                    *jacdifp.add((ci + 0 * NV) as usize) = -(*cdof.add(3) + tmp[0]);
                    *jacdifp.add((ci + 1 * NV) as usize) = -(*cdof.add(4) + tmp[1]);
                    *jacdifp.add((ci + 2 * NV) as usize) = -(*cdof.add(5) + tmp[2]);
                }
            }

            ci += 1;
            da += 1;
        }
    }
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
    todo!() // mj_jacDotSparse
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
    todo!() // mj_jacDifPair
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
    todo!() // mj_jacSum
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
    const MJ_JNT_BALL: i32 = 1;
    const MJ_JNT_FREE: i32 = 0;

    // SAFETY: caller guarantees m, d, point valid; jacp/jacr valid if non-null
    unsafe {
        let nv = (*m).nv as i32;
        let mut offset: [f64; 3] = [0.0; 3];
        let mut pvel: [f64; 6] = [0.0; 6];

        // clear jacobians, compute offset and pvel if required
        if !jacp.is_null() {
            crate::engine::engine_util_blas::mju_zero(jacp, 3 * nv);
            let com = (*d).subtree_com.add(3 * *(*m).body_rootid.add(body as usize) as usize);
            crate::engine::engine_util_blas::mju_sub3(offset.as_mut_ptr(), point, com);
            crate::engine::engine_util_spatial::mju_transform_spatial(
                pvel.as_mut_ptr(), (*d).cvel.add(6 * body as usize), 0, point, com, std::ptr::null());
        }
        if !jacr.is_null() {
            crate::engine::engine_util_blas::mju_zero(jacr, 3 * nv);
        }

        // skip fixed bodies
        let body = *(*m).body_weldid.add(body as usize);
        if body == 0 {
            return;
        }

        // get last dof
        let mut i = *(*m).body_dofadr.add(body as usize) + *(*m).body_dofnum.add(body as usize) - 1;

        // backward pass over dof ancestor chain
        while i >= 0 {
            let mut cdof_dot: [f64; 6] = [0.0; 6];
            crate::engine::engine_inline::mji_copy6(cdof_dot.as_mut_ptr(), (*d).cdof_dot.add(6 * i as usize));
            let cdof = (*d).cdof.add(6 * i as usize);

            // check for quaternion
            let jnt_type = *(*m).jnt_type.add(*(*m).dof_jntid.add(i as usize) as usize);
            let dofadr = *(*m).jnt_dofadr.add(*(*m).dof_jntid.add(i as usize) as usize);
            let is_quat = jnt_type == MJ_JNT_BALL || (jnt_type == MJ_JNT_FREE && i >= dofadr + 3);

            // compute cdof_dot for quaternion
            if is_quat {
                crate::engine::engine_inline::mji_cross_motion(
                    cdof_dot.as_mut_ptr(),
                    (*d).cvel.add(6 * *(*m).dof_bodyid.add(i as usize) as usize),
                    cdof);
            }

            // construct rotation jacobian
            if !jacr.is_null() {
                *jacr.add((i + 0 * nv) as usize) += cdof_dot[0];
                *jacr.add((i + 1 * nv) as usize) += cdof_dot[1];
                *jacr.add((i + 2 * nv) as usize) += cdof_dot[2];
            }

            // construct translation jacobian
            if !jacp.is_null() {
                let mut tmp1: [f64; 3] = [0.0; 3];
                crate::engine::engine_inline::mji_cross(tmp1.as_mut_ptr(), cdof_dot.as_ptr(), offset.as_ptr());

                let mut tmp2: [f64; 3] = [0.0; 3];
                crate::engine::engine_inline::mji_cross(tmp2.as_mut_ptr(), cdof, pvel.as_ptr().add(3));

                *jacp.add((i + 0 * nv) as usize) += cdof_dot[3] + tmp1[0] + tmp2[0];
                *jacp.add((i + 1 * nv) as usize) += cdof_dot[4] + tmp1[1] + tmp2[1];
                *jacp.add((i + 2 * nv) as usize) += cdof_dot[5] + tmp1[2] + tmp2[2];
            }

            // advance to parent dof
            i = *(*m).dof_parentid.add(i as usize);
        }
    }
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
    todo!() // mj_angmomMat
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
    // SAFETY: m, d, res are valid pointers; objtype/objid within bounds (caller contract)
    unsafe {
        let mut bodyid: i32 = 0;
        let mut pos: *const f64 = std::ptr::null();
        let mut rot: *const f64 = std::ptr::null();

        // body-inertial
        if objtype == 1 {  // mjOBJ_BODY
            bodyid = objid;
            pos = (*d).xipos.add(3 * objid as usize);
            rot = if flg_local != 0 { (*d).ximat.add(9 * objid as usize) } else { std::ptr::null() };
        }
        // body-regular
        else if objtype == 2 {  // mjOBJ_XBODY
            bodyid = objid;
            pos = (*d).xpos.add(3 * objid as usize);
            rot = if flg_local != 0 { (*d).xmat.add(9 * objid as usize) } else { std::ptr::null() };
        }
        // geom
        else if objtype == 5 {  // mjOBJ_GEOM
            bodyid = *(*m).geom_bodyid.add(objid as usize);
            pos = (*d).geom_xpos.add(3 * objid as usize);
            rot = if flg_local != 0 { (*d).geom_xmat.add(9 * objid as usize) } else { std::ptr::null() };
        }
        // site
        else if objtype == 6 {  // mjOBJ_SITE
            bodyid = *(*m).site_bodyid.add(objid as usize);
            pos = (*d).site_xpos.add(3 * objid as usize);
            rot = if flg_local != 0 { (*d).site_xmat.add(9 * objid as usize) } else { std::ptr::null() };
        }
        // camera
        else if objtype == 7 {  // mjOBJ_CAMERA
            bodyid = *(*m).cam_bodyid.add(objid as usize);
            pos = (*d).cam_xpos.add(3 * objid as usize);
            rot = if flg_local != 0 { (*d).cam_xmat.add(9 * objid as usize) } else { std::ptr::null() };
        }
        // invalid
        else {
            crate::engine::engine_util_errmem::mju_error(
                b"invalid object type %d\0".as_ptr() as *const i8);
        }

        // static body: quick return
        if *(*m).body_weldid.add(bodyid as usize) == 0 {
            crate::engine::engine_util_blas::mju_zero(res, 6);
            return;
        }

        // transform velocity
        crate::engine::engine_util_spatial::mju_transform_spatial(
            res,
            (*d).cvel.add(6 * bodyid as usize),
            0,
            pos,
            (*d).subtree_com.add(3 * (*(*m).body_rootid.add(bodyid as usize)) as usize),
            rot,
        );
    }
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
    todo!() // mj_objectAcceleration
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
    // mjtSameFrame enum values
    const SAMEFRAME_NONE: u8 = 0;
    const SAMEFRAME_BODY: u8 = 1;
    const SAMEFRAME_INERTIA: u8 = 2;
    const SAMEFRAME_BODYROT: u8 = 3;
    const SAMEFRAME_INERTIAROT: u8 = 4;

    // SAFETY: caller guarantees d is valid, xpos/xmat/pos/quat are valid arrays when non-null
    unsafe {
        let sf = sameframe;
        let b = body as usize;

        // position
        if !xpos.is_null() && !pos.is_null() {
            match sf {
                SAMEFRAME_NONE | SAMEFRAME_BODYROT | SAMEFRAME_INERTIAROT => {
                    crate::engine::engine_inline::mji_mul_mat_vec3(xpos, (*d).xmat.add(9 * b), pos);
                    crate::engine::engine_inline::mji_add_to3(xpos, (*d).xpos.add(3 * b));
                }
                SAMEFRAME_BODY => {
                    crate::engine::engine_inline::mji_copy3(xpos, (*d).xpos.add(3 * b));
                }
                SAMEFRAME_INERTIA => {
                    crate::engine::engine_inline::mji_copy3(xpos, (*d).xipos.add(3 * b));
                }
                _ => {}
            }
        }

        // orientation
        if !xmat.is_null() && !quat.is_null() {
            let mut tmp: [f64; 4] = [0.0; 4];
            match sf {
                SAMEFRAME_NONE => {
                    crate::engine::engine_inline::mji_mul_quat(tmp.as_mut_ptr(), (*d).xquat.add(4 * b), quat);
                    crate::engine::engine_util_spatial::mju_quat2mat(xmat, tmp.as_ptr());
                }
                SAMEFRAME_BODY | SAMEFRAME_BODYROT => {
                    crate::engine::engine_inline::mji_copy9(xmat, (*d).xmat.add(9 * b));
                }
                SAMEFRAME_INERTIA | SAMEFRAME_INERTIAROT => {
                    crate::engine::engine_inline::mji_copy9(xmat, (*d).ximat.add(9 * b));
                }
                _ => {}
            }
        }
    }
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
    todo!() // mju_flexGatherState
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
    unsafe {
        // SAFETY: Caller guarantees m, d, result are valid. id is checked.
        // Clear result
        crate::engine::engine_util_blas::mju_zero(result, 6);

        // Make sure contact is valid
        if id >= 0 && id < (*d).ncon && (*(*d).contact.add(id as usize)).efc_address >= 0 {
            let con = &*(*d).contact.add(id as usize);

            if crate::engine::engine_core_util::mj_is_pyramidal(m) != 0 {
                crate::engine::engine_util_misc::mju_decode_pyramid(
                    result,
                    (*d).efc_force.add(con.efc_address as usize),
                    con.friction.as_ptr(),
                    con.dim,
                );
            } else {
                crate::engine::engine_util_blas::mju_copy(
                    result,
                    (*d).efc_force.add(con.efc_address as usize),
                    con.dim,
                );
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
    const MJNPOLY: i32 = 2;

    // SAFETY: m is valid mjModel pointer; id within bounds (caller contract)
    unsafe {
        if r#type != 18 && r#type != 3 {  // mjOBJ_TENDON=18, mjOBJ_JOINT=3
            crate::engine::engine_util_errmem::mju_error(
                b"only joint and tendon objects can inherit damping from actuators\0".as_ptr() as *const i8);
            return 0.0;
        }

        // get actuator id
        let actuatorid = if r#type == 3 {
            *(*m).jnt_actuatorid.add(id as usize)
        } else {
            *(*m).tendon_actuatorid.add(id as usize)
        };

        if actuatorid == -1 {
            return 0.0;
        }

        let mut damping: f64 = 0.0;

        // single actuator contributes damping
        if actuatorid >= 0 {
            let gear = *(*m).actuator_gear.add(6 * actuatorid as usize);
            let gear2 = gear * gear;
            damping = *(*m).actuator_damping.add(actuatorid as usize) * gear2;
            for k in 0..MJNPOLY {
                *poly.add(k as usize) += *(*m).actuator_dampingpoly.add((MJNPOLY * actuatorid + k) as usize) * gear2;
            }
        }
        // scan all actuators
        else {
            let nu = (*m).nu as i32;
            for k in 0..nu {
                if *(*m).actuator_trnid.add(2 * k as usize) != id {
                    continue;
                }
                if r#type == 3 &&  // mjOBJ_JOINT
                   *(*m).actuator_trntype.add(k as usize) != 0 &&  // mjTRN_JOINT
                   *(*m).actuator_trntype.add(k as usize) != 1 {   // mjTRN_JOINTINPARENT
                    continue;
                }
                if r#type == 18 &&  // mjOBJ_TENDON
                   *(*m).actuator_trntype.add(k as usize) != 3 {   // mjTRN_TENDON
                    continue;
                }
                let gear = *(*m).actuator_gear.add(6 * k as usize);
                let gear2 = gear * gear;
                damping += *(*m).actuator_damping.add(k as usize) * gear2;
                for j in 0..MJNPOLY {
                    *poly.add(j as usize) += *(*m).actuator_dampingpoly.add((MJNPOLY * k + j) as usize) * gear2;
                }
            }
        }

        damping
    }
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
    // SAFETY: m is valid mjModel pointer; id within bounds (caller contract)
    unsafe {
        if r#type != 18 && r#type != 3 {  // mjOBJ_TENDON=18, mjOBJ_JOINT=3
            crate::engine::engine_util_errmem::mju_error(
                b"only joint and tendon objects can inherit armature from actuators\0".as_ptr() as *const i8);
            return 0.0;
        }

        // get actuator id
        let actuatorid = if r#type == 3 {
            *(*m).jnt_actuatorid.add(id as usize)
        } else {
            *(*m).tendon_actuatorid.add(id as usize)
        };

        // no actuator contribution
        if actuatorid == -1 {
            return 0.0;
        }

        let mut armature: f64 = 0.0;

        // single actuator contributes armature
        if actuatorid >= 0 {
            let gear = *(*m).actuator_gear.add(6 * actuatorid as usize);
            let gear2 = gear * gear;
            armature = *(*m).actuator_armature.add(actuatorid as usize) * gear2;
        }
        // scan all actuators
        else {
            let nu = (*m).nu as i32;
            for k in 0..nu {
                if *(*m).actuator_trnid.add(2 * k as usize) != id {
                    continue;
                }
                if r#type == 3 &&  // mjOBJ_JOINT
                   *(*m).actuator_trntype.add(k as usize) != 0 &&  // mjTRN_JOINT
                   *(*m).actuator_trntype.add(k as usize) != 1 {   // mjTRN_JOINTINPARENT
                    continue;
                }
                if r#type == 18 &&  // mjOBJ_TENDON
                   *(*m).actuator_trntype.add(k as usize) != 3 {   // mjTRN_TENDON
                    continue;
                }
                let gear = *(*m).actuator_gear.add(6 * k as usize);
                let gear2 = gear * gear;
                armature += *(*m).actuator_armature.add(k as usize) * gear2;
            }
        }

        armature
    }
}

/// C: mj_warning (engine/engine_core_util.h:148)
/// Calls: mju_message, mju_warning, mju_warningText
#[allow(unused_variables, non_snake_case)]
pub fn mj_warning(d: *mut mjData, warning: i32, info: i32) {
    const MJ_NWARNING: i32 = 7;
    // SAFETY: d is a valid pointer to mjData (caller contract)
    unsafe {
        // check type
        if warning < 0 || warning >= MJ_NWARNING {
            crate::engine::engine_util_errmem::mju_error(
                b"invalid warning type %d\0".as_ptr() as *const i8);
            return;
        }

        // d->warning is [u8; 56] = mjWarningStat[7], each 8 bytes: {lastinfo: i32, number: i32}
        let warn_ptr = (*d).warning.as_mut_ptr().add(warning as usize * 8);
        let lastinfo_ptr = warn_ptr as *mut i32;
        let number_ptr = (warn_ptr.add(4)) as *mut i32;

        // save info (override previous)
        *lastinfo_ptr = info;

        // print message only the first time this warning is encountered
        if *number_ptr == 0 {
            crate::engine::engine_util_errmem::mju_warning(
                crate::engine::engine_util_misc::mju_warning_text(warning, info as usize));
        }

        // increase counter
        *number_ptr += 1;
    }
}

