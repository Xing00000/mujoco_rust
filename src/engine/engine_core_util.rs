//! Port of: engine/engine_core_util.h
//! IR hash: 05737965add36adb
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mj_isPyramidal (engine/engine_core_util.h:31)
#[allow(unused_variables, non_snake_case)]
pub fn mj_is_pyramidal(m: *const mjModel) -> i32 {
    // SAFETY: m is a valid pointer to mjModel per caller contract
    // mjCONE_PYRAMIDAL == 0
    unsafe {
        if (*m).opt.cone == 0 {
            1
        } else {
            0
        }
    }
}

/// C: mj_isSparse (engine/engine_core_util.h:34)
#[allow(unused_variables, non_snake_case)]
pub fn mj_is_sparse(m: *const mjModel) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel)
    // Previous return: i32
    const MJ_JAC_SPARSE : i32 = 1 ; const MJ_JAC_AUTO : i32 = 2 ; # [repr (C)] struct MjOptionPartial { timestep : f64 , impratio : f64 , tolerance : f64 , ls_tolerance : f64 , noslip_tolerance : f64 , ccd_tolerance : f64 , sleep_tolerance : f64 , gravity : [f64 ; 3] , wind : [f64 ; 3] , magnetic : [f64 ; 3] , density : f64 , viscosity : f64 , o_margin : f64 , o_solref : [f64 ; 2] , o_solimp : [f64 ; 5] , o_friction : [f64 ; 5] , integrator : i32 , cone : i32 , jacobian : i32 , } # [repr (C)] struct MjModelPartial { nq : i64 , nv : i64 , _sizes : [i64 ; 90] , opt : MjOptionPartial , } unsafe { let mp = m as * const MjModelPartial ; let jacobian = (* mp) . opt . jacobian ; let nv = (* mp) . nv ; if jacobian == MJ_JAC_SPARSE || (jacobian == MJ_JAC_AUTO && nv >= 60) { return 1 ; } else { return 0 ; } }
}

/// C: mj_mergeChain (engine/engine_core_util.h:40)
#[allow(unused_variables, non_snake_case)]
pub fn mj_merge_chain(m: *const mjModel, chain: *mut i32, b1: i32, b2: i32, flg_skipcommon: i32) -> i32 {
    // SAFETY: m valid, chain points to buffer with adequate capacity.
    // b1, b2 are valid body indices.
    unsafe {
        // skip fixed bodies
        let b1 = *(*m).body_weldid.add(b1 as usize);
        let b2 = *(*m).body_weldid.add(b2 as usize);

        // neither body is movable: empty chain
        if b1 == 0 && b2 == 0 {
            return 0;
        }

        // initialize last dof address for each body
        let mut da1 = *(*m).body_dofadr.add(b1 as usize) + *(*m).body_dofnum.add(b1 as usize) - 1;
        let mut da2 = *(*m).body_dofadr.add(b2 as usize) + *(*m).body_dofnum.add(b2 as usize) - 1;
        let mut NV: i32 = 0;

        // merge chains
        while da1 >= 0 || da2 >= 0 {
            let da = if da1 > da2 { da1 } else { da2 };
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
    // SAFETY: m valid, chain points to buffer with adequate capacity.
    unsafe {
        // swap bodies if wrong order
        let (b1, b2) = if b1 > b2 { (b2, b1) } else { (b1, b2) };

        // init
        let n1 = *(*m).body_dofnum.add(b1 as usize);
        let n2 = *(*m).body_dofnum.add(b2 as usize);

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
    // SAFETY: m is valid mjModel pointer, chain is caller-allocated output buffer,
    // body is a valid body index. All pointer offsets are within model bounds per MuJoCo invariants.
    unsafe {
        if *(*m).body_simple.add(body as usize) != 0 {
            let dofnum: i32 = *(*m).body_dofnum.add(body as usize);
            let mut i: i32 = 0;
            while i < dofnum {
                *chain.add(i as usize) = *(*m).body_dofadr.add(body as usize) + i;
                i += 1;
            }
            return dofnum;
        } else {
            let body: i32 = *(*m).body_weldid.add(body as usize);
            if body == 0 {
                return 0;
            }
            let mut da: i32 = *(*m).body_dofadr.add(body as usize)
                + *(*m).body_dofnum.add(body as usize) - 1;
            let mut NV: i32 = 0;
            while da >= 0 {
                *chain.add(NV as usize) = da;
                NV += 1;
                da = *(*m).dof_parentid.add(da as usize);
            }
            let mut i: i32 = 0;
            while i < NV / 2 {
                let tmp: i32 = *chain.add(i as usize);
                *chain.add(i as usize) = *chain.add((NV - i - 1) as usize);
                *chain.add((NV - i - 1) as usize) = tmp;
                i += 1;
            }
            return NV;
        }
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
    // SAFETY: m, d are valid model/data pointers. jacp/jacr are caller-allocated 3*nv arrays (or null).
    // point is a valid 3-element array. body is a valid body index.
    unsafe {
        let nv: i32 = (*m).nv as i32;
        let mut offset: [f64; 3] = [0.0; 3];

        if !jacp.is_null() {
            crate::engine::engine_util_blas::mju_zero(jacp, 3 * nv);
            crate::engine::engine_util_blas::mju_sub3(
                offset.as_mut_ptr(),
                point,
                (*d).subtree_com.add(3 * (*(*m).body_rootid.add(body as usize)) as usize),
            );
        }
        if !jacr.is_null() {
            crate::engine::engine_util_blas::mju_zero(jacr, 3 * nv);
        }

        let body: i32 = *(*m).body_weldid.add(body as usize);
        if body == 0 {
            return;
        }

        let mut i: i32 = *(*m).body_dofadr.add(body as usize)
            + *(*m).body_dofnum.add(body as usize) - 1;

        while i >= 0 {
            let cdof: *const f64 = (*d).cdof.add(6 * i as usize);

            if !jacr.is_null() {
                *jacr.add((i + 0 * nv) as usize) = *cdof.add(0);
                *jacr.add((i + 1 * nv) as usize) = *cdof.add(1);
                *jacr.add((i + 2 * nv) as usize) = *cdof.add(2);
            }

            if !jacp.is_null() {
                let mut tmp: [f64; 3] = [0.0; 3];
                crate::engine::engine_inline::mji_cross(
                    tmp.as_mut_ptr(),
                    cdof,
                    offset.as_ptr(),
                );
                *jacp.add((i + 0 * nv) as usize) = *cdof.add(3) + tmp[0];
                *jacp.add((i + 1 * nv) as usize) = *cdof.add(4) + tmp[1];
                *jacp.add((i + 2 * nv) as usize) = *cdof.add(5) + tmp[2];
            }

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

    extern "C" { fn mj_jacBody_impl(m: *const mjModel, d: *const mjData, jacp: *mut f64, jacr: *mut f64, body: i32); }
    // SAFETY: delegates to C implementation
    unsafe { mj_jacBody_impl(m, d, jacp, jacr, body) }
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
    extern "C" { fn mj_jacBodyCom_impl(m: *const mjModel, d: *const mjData, jacp: *mut f64, jacr: *mut f64, body: i32); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_jacBodyCom_impl(m, d, jacp, jacr, body) }
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

    extern "C" { fn mj_jacSubtreeCom_impl(m: *const mjModel, d: *mut mjData, jacp: *mut f64, body: i32); }
    // SAFETY: delegates to C implementation
    unsafe { mj_jacSubtreeCom_impl(m, d, jacp, body) }
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
    extern "C" { fn mj_jacGeom_impl(m: *const mjModel, d: *const mjData, jacp: *mut f64, jacr: *mut f64, geom: i32); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_jacGeom_impl(m, d, jacp, jacr, geom) }
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
    extern "C" {
        fn mj_jacSite_impl(m: *const mjModel, d: *const mjData, jacp: *mut f64, jacr: *mut f64, site: i32);
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_jacSite_impl(m, d, jacp, jacr, site) }
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
    extern "C" {
        fn mj_jacPointAxis_impl(m: *const mjModel, d: *mut mjData, jacPoint: *mut f64, jacAxis: *mut f64, point: *const f64, axis: *const f64, body: i32);
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_jacPointAxis_impl(m, d, jacPoint, jacAxis, point, axis, body) }
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
    // SAFETY: m, d are valid model/data pointers. jacp/jacr are caller-allocated 3*NV arrays (or null).
    // point is a 3-element array. chain is a NV-element array. body is a valid body index.
    unsafe {
        if !jacp.is_null() {
            crate::engine::engine_util_blas::mju_zero(jacp, 3 * NV);
        }
        if !jacr.is_null() {
            crate::engine::engine_util_blas::mju_zero(jacr, 3 * NV);
        }

        let mut offset: [f64; 3] = [0.0; 3];
        crate::engine::engine_util_blas::mju_sub3(
            offset.as_mut_ptr(),
            point,
            (*d).subtree_com.add(3 * (*(*m).body_rootid.add(body as usize)) as usize),
        );

        let body: i32 = *(*m).body_weldid.add(body as usize);
        if body == 0 {
            return;
        }

        let mut da: i32 = *(*m).body_dofadr.add(body as usize)
            + *(*m).body_dofnum.add(body as usize) - 1;
        let mut ci: i32 = NV - 1;

        while da >= 0 {
            while ci >= 0 && *chain.add(ci as usize) > da {
                ci -= 1;
            }
            if ci < 0 || *chain.add(ci as usize) != da {
                if flg_skipcommon != 0 {
                    da = *(*m).dof_parentid.add(da as usize);
                    continue;
                }
                extern "C" {
                    fn mju_error(msg: *const i8, ...);
                }
                mju_error(b"dof index not found in chain\0".as_ptr() as *const i8);
            }

            let cdof: *const f64 = (*d).cdof.add(6 * da as usize);

            if !jacr.is_null() {
                *jacr.add((ci + 0 * NV) as usize) = *cdof.add(0);
                *jacr.add((ci + 1 * NV) as usize) = *cdof.add(1);
                *jacr.add((ci + 2 * NV) as usize) = *cdof.add(2);
            }

            if !jacp.is_null() {
                let mut tmp: [f64; 3] = [0.0; 3];
                crate::engine::engine_inline::mji_cross(
                    tmp.as_mut_ptr(),
                    cdof,
                    offset.as_ptr(),
                );
                *jacp.add((ci + 0 * NV) as usize) = *cdof.add(3) + tmp[0];
                *jacp.add((ci + 1 * NV) as usize) = *cdof.add(4) + tmp[1];
                *jacp.add((ci + 2 * NV) as usize) = *cdof.add(5) + tmp[2];
            }

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
    extern "C" {
        fn mj_jacSparseSimple_impl(m: *const mjModel, d: *const mjData, jacdifp: *mut f64, jacdifr: *mut f64, point: *const f64, body: i32, flg_second: i32, NV: i32, start: i32);
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_jacSparseSimple_impl(m, d, jacdifp, jacdifr, point, body, flg_second, NV, start) }
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
    extern "C" {
        fn mj_jacDotSparse_impl(m: *const mjModel, d: *const mjData, jacp: *mut f64, jacr: *mut f64, point: *const f64, body: i32, NV: i32, chain: *const i32);
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_jacDotSparse_impl(m, d, jacp, jacr, point, body, NV, chain) }
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
    extern "C" {
        fn mj_jacDifPair_impl(m: *const mjModel, d: *const mjData, chain: *mut i32, b1: i32, b2: i32, pos1: *const f64, pos2: *const f64, jac1p: *mut f64, jac2p: *mut f64, jacdifp: *mut f64, jac1r: *mut f64, jac2r: *mut f64, jacdifr: *mut f64, issparse: i32, flg_skipcommon: i32) -> i32;
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_jacDifPair_impl(m, d, chain, b1, b2, pos1, pos2, jac1p, jac2p, jacdifp, jac1r, jac2r, jacdifr, issparse, flg_skipcommon) }
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
    extern "C" {
        fn mj_jacSum_impl(m: *const mjModel, d: *mut mjData, chain: *mut i32, n: i32, body: *const i32, weight: *const f64, point: *const f64, jac: *mut f64, flg_rot: i32) -> i32;
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_jacSum_impl(m, d, chain, n, body, weight, point, jac, flg_rot) }
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
    extern "C" {
        fn mj_jacDot_impl(m: *const mjModel, d: *const mjData, jacp: *mut f64, jacr: *mut f64, point: *const f64, body: i32);
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_jacDot_impl(m, d, jacp, jacr, point, body) }
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

    extern "C" { fn mj_angmomMat_impl(m: *const mjModel, d: *mut mjData, mat: *mut f64, body: i32); }
    // SAFETY: delegates to C implementation
    unsafe { mj_angmomMat_impl(m, d, mat, body) }
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
    extern "C" {
        fn mj_objectVelocity_impl(m: *const mjModel, d: *const mjData, objtype: i32, objid: i32, res: *mut f64, flg_local: i32);
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_objectVelocity_impl(m, d, objtype, objid, res, flg_local) }
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
    extern "C" {
        fn mj_objectAcceleration_impl(m: *const mjModel, d: *const mjData, objtype: i32, objid: i32, res: *mut f64, flg_local: i32);
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_objectAcceleration_impl(m, d, objtype, objid, res, flg_local) }
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
    // SAFETY: d is valid mjData pointer. xpos/xmat are caller-allocated output arrays (or null).
    // pos is 3-element, quat is 4-element (or null). body is valid body index.
    // sameframe is an enum value 0..4.
    unsafe {
        let sf: u8 = sameframe;

        if !xpos.is_null() && !pos.is_null() {
            match sf {
                // mjSAMEFRAME_NONE=0, mjSAMEFRAME_BODYROT=2, mjSAMEFRAME_INERTIAROT=4
                0 | 2 | 4 => {
                    crate::engine::engine_inline::mji_mul_mat_vec3(
                        xpos,
                        (*d).xmat.add(9 * body as usize),
                        pos,
                    );
                    crate::engine::engine_inline::mji_add_to3(
                        xpos,
                        (*d).xpos.add(3 * body as usize),
                    );
                }
                // mjSAMEFRAME_BODY=1
                1 => {
                    crate::engine::engine_inline::mji_copy3(
                        xpos,
                        (*d).xpos.add(3 * body as usize),
                    );
                }
                // mjSAMEFRAME_INERTIA=3
                3 => {
                    crate::engine::engine_inline::mji_copy3(
                        xpos,
                        (*d).xipos.add(3 * body as usize),
                    );
                }
                _ => {}
            }
        }

        if !xmat.is_null() && !quat.is_null() {
            let mut tmp: [f64; 4] = [0.0; 4];
            match sf {
                // mjSAMEFRAME_NONE=0
                0 => {
                    crate::engine::engine_inline::mji_mul_quat(
                        tmp.as_mut_ptr(),
                        (*d).xquat.add(4 * body as usize),
                        quat,
                    );
                    crate::engine::engine_util_spatial::mju_quat2mat(xmat, tmp.as_ptr());
                }
                // mjSAMEFRAME_BODY=1, mjSAMEFRAME_BODYROT=2
                1 | 2 => {
                    crate::engine::engine_inline::mji_copy9(
                        xmat,
                        (*d).xmat.add(9 * body as usize),
                    );
                }
                // mjSAMEFRAME_INERTIA=3, mjSAMEFRAME_INERTIAROT=4
                3 | 4 => {
                    crate::engine::engine_inline::mji_copy9(
                        xmat,
                        (*d).ximat.add(9 * body as usize),
                    );
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
    extern "C" {
        fn mju_flexGatherState_impl(m: *const mjModel, d: *const mjData, f: i32, xpos: *mut f64, vel: *mut f64);
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mju_flexGatherState_impl(m, d, f, xpos, vel) }
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
    extern "C" {
        fn mj_contactForce_impl(m: *const mjModel, d: *const mjData, id: i32, result: *mut f64);
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_contactForce_impl(m, d, id, result) }
}

/// C: tendonLimit (engine/engine_core_util.h:139)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn tendon_limit(m: *const mjModel, ten_length: *const f64, i: i32) -> i32 {
    // SAFETY: caller guarantees valid model and length array
    unsafe {
        // mjtBool is 1-byte _Bool in C, read as u8
        if *((*m).tendon_limited as *const u8).add(i as usize) == 0 {
            return 0;
        }

        let mut nl: i32 = 0;
        let value: f64 = *ten_length.add(i as usize);
        let margin: f64 = *(*m).tendon_margin.add(i as usize);

        // tendon limits can be bilateral, check both sides
        let mut side: i32 = -1;
        while side <= 1 {
            let dist: f64 = (side as f64) * (*(*m).tendon_range.add((2 * i + (side + 1) / 2) as usize) - value);
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
pub fn mj_actuator_damping(m: *const mjModel, r#type: mjtObj, id: i32, poly: *mut f64) -> f64 {
    extern "C" {
        fn mj_actuatorDamping_impl(m: *const mjModel, r#type: mjtObj, id: i32, poly: *mut f64) -> f64;
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_actuatorDamping_impl(m, r#type, id, poly) }
}

/// C: mj_actuatorArmature (engine/engine_core_util.h:145)
/// Calls: mju_message
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_actuator_armature(m: *const mjModel, r#type: mjtObj, id: i32) -> f64 {
    extern "C" {
        fn mj_actuatorArmature_impl(m: *const mjModel, r#type: mjtObj, id: i32) -> f64;
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_actuatorArmature_impl(m, r#type, id) }
}

/// C: mj_warning (engine/engine_core_util.h:148)
/// Calls: mju_message, mju_warning, mju_warningText
#[allow(unused_variables, non_snake_case)]
pub fn mj_warning(d: *mut mjData, warning: i32, info: i32) {
    // WARNING: signature changed — verify body
    // Previous params: (d : * mut mjData, warning : i32, info : i32)
    // Previous return: ()
    extern "C" { fn mj_warning_impl (d : * mut mjData , warning : i32 , info : i32) ; } unsafe { mj_warning_impl (d , warning , info) }
}

