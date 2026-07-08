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
    // SAFETY: m, d valid. body is a valid body index.
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
    // SAFETY: m, d valid. geom is valid geom index.
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
    // SAFETY: m, d valid per caller. site is a valid site index.
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
    // SAFETY: all pointers valid per caller. body is a valid simple body index.
    // jacdifp/jacdifr have capacity for 3*NV elements. point has 3 elements.
    unsafe {
        // compute point-com offset
        let mut offset: [f64; 3] = [0.0; 3];
        crate::engine::engine_util_blas::mju_sub3(
            offset.as_mut_ptr(),
            point,
            (*d).subtree_com.add(3 * *(*m).body_rootid.add(body as usize) as usize),
        );

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

            // advance jacdif counter
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
    // SAFETY: m, d valid. jacp/jacr have 3*NV elements (may be null). point has 3 elements.
    // chain has NV elements in increasing order.
    unsafe {
        const MJJNT_BALL: i32 = 1;
        const MJJNT_FREE: i32 = 0;

        let mut offset: [f64; 3] = [0.0; 3];
        let mut pvel: [f64; 6] = [0.0; 6];

        // clear jacobians, compute offset and pvel if required
        if !jacp.is_null() {
            crate::engine::engine_util_blas::mju_zero(jacp, 3 * NV);
            let com = (*d).subtree_com.add(3 * *(*m).body_rootid.add(body as usize) as usize);
            crate::engine::engine_util_blas::mju_sub3(offset.as_mut_ptr(), point, com);
            crate::engine::engine_util_spatial::mju_transform_spatial(
                pvel.as_mut_ptr(), (*d).cvel.add(6 * body as usize), 0, point, com, core::ptr::null());
        }
        if !jacr.is_null() {
            crate::engine::engine_util_blas::mju_zero(jacr, 3 * NV);
        }

        // skip fixed bodies
        let body = *(*m).body_weldid.add(body as usize);
        if body == 0 {
            return;
        }

        // get last dof that affects this body
        let mut da = *(*m).body_dofadr.add(body as usize) + *(*m).body_dofnum.add(body as usize) - 1;

        // start at end of chain
        let mut ci: i32 = NV - 1;

        // backward pass over dof ancestor chain
        while da >= 0 {
            // find chain index for this dof
            while ci >= 0 && *chain.add(ci as usize) > da {
                ci -= 1;
            }

            // dof not in chain: error
            if ci < 0 || *chain.add(ci as usize) != da {
                extern "C" { fn mju_error_impl(msg: *const i8); }
                mju_error_impl(b"dof index not found in chain\0".as_ptr() as *const i8);
                return;
            }

            let mut cdof_dot: [f64; 6] = [0.0; 6];
            crate::engine::engine_inline::mji_copy6(cdof_dot.as_mut_ptr(), (*d).cdof_dot.add(6 * da as usize));
            let cdof = (*d).cdof.add(6 * da as usize);

            // check for quaternion
            let jnt_type = *(*m).jnt_type.add(*(*m).dof_jntid.add(da as usize) as usize);
            let dofadr = *(*m).jnt_dofadr.add(*(*m).dof_jntid.add(da as usize) as usize);
            let is_quat = jnt_type == MJJNT_BALL || (jnt_type == MJJNT_FREE && da >= dofadr + 3);

            // compute cdof_dot for quaternion
            if is_quat {
                crate::engine::engine_inline::mji_cross_motion(
                    cdof_dot.as_mut_ptr(),
                    (*d).cvel.add(6 * *(*m).dof_bodyid.add(da as usize) as usize),
                    cdof);
            }

            // construct rotation jacobian
            if !jacr.is_null() {
                *jacr.add((ci + 0 * NV) as usize) += cdof_dot[0];
                *jacr.add((ci + 1 * NV) as usize) += cdof_dot[1];
                *jacr.add((ci + 2 * NV) as usize) += cdof_dot[2];
            }

            // construct translation jacobian
            if !jacp.is_null() {
                let mut tmp1: [f64; 3] = [0.0; 3];
                crate::engine::engine_inline::mji_cross(tmp1.as_mut_ptr(), cdof_dot.as_ptr(), offset.as_ptr());
                let mut tmp2: [f64; 3] = [0.0; 3];
                crate::engine::engine_inline::mji_cross(tmp2.as_mut_ptr(), cdof, pvel.as_ptr().add(3));

                *jacp.add((ci + 0 * NV) as usize) += cdof_dot[3] + tmp1[0] + tmp2[0];
                *jacp.add((ci + 1 * NV) as usize) += cdof_dot[4] + tmp1[1] + tmp2[1];
                *jacp.add((ci + 2 * NV) as usize) += cdof_dot[5] + tmp1[2] + tmp2[2];
            }

            // advance to parent dof
            da = *(*m).dof_parentid.add(da as usize);
        }
    }
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
    // SAFETY: m, d valid. chain has adequate capacity. pos1/pos2 have 3 elements.
    // jac pointers have 3*nv elements (dense) or 3*NV (sparse). May be null.
    unsafe {
        let issimple = (*(*m).body_simple.add(b1 as usize) != 0) && (*(*m).body_simple.add(b2 as usize) != 0);
        let mut NV: i32 = (*m).nv as i32;

        // skip if no DOFs
        if NV == 0 {
            return 0;
        }

        // construct merged chain of body dofs
        if issparse != 0 {
            if issimple {
                NV = mj_merge_chain_simple(m, chain, b1, b2);
            } else {
                NV = mj_merge_chain(m, chain, b1, b2, flg_skipcommon);
            }
        }

        // skip if empty chain
        if NV == 0 {
            return 0;
        }

        // count-only mode
        if jacdifp.is_null() && jacdifr.is_null() && jac1p.is_null() && jac1r.is_null() {
            return NV;
        }

        // sparse case
        if issparse != 0 {
            if issimple {
                // first body
                let start1 = if b1 < b2 { 0 } else { *(*m).body_dofnum.add(b2 as usize) };
                mj_jac_sparse_simple(m, d, jacdifp, jacdifr, pos1, b1, 0, NV, start1);

                // second body
                let start2 = if b2 < b1 { 0 } else { *(*m).body_dofnum.add(b1 as usize) };
                mj_jac_sparse_simple(m, d, jacdifp, jacdifr, pos2, b2, 1, NV, start2);
            } else {
                // Jacobians
                mj_jac_sparse(m, d, jac1p, jac1r, pos1, b1, NV, chain, flg_skipcommon);
                mj_jac_sparse(m, d, jac2p, jac2r, pos2, b2, NV, chain, flg_skipcommon);

                // differences
                if !jacdifp.is_null() {
                    crate::engine::engine_util_blas::mju_sub(jacdifp, jac2p, jac1p, 3 * NV);
                }
                if !jacdifr.is_null() {
                    crate::engine::engine_util_blas::mju_sub(jacdifr, jac2r, jac1r, 3 * NV);
                }
            }
        }
        // dense case
        else {
            // Jacobians
            mj_jac(m, d, jac1p, jac1r, pos1, b1);
            mj_jac(m, d, jac2p, jac2r, pos2, b2);

            // differences
            if !jacdifp.is_null() {
                crate::engine::engine_util_blas::mju_sub(jacdifp, jac2p, jac1p, 3 * NV);
            }
            if !jacdifr.is_null() {
                crate::engine::engine_util_blas::mju_sub(jacdifr, jac2r, jac1r, 3 * NV);
            }
        }

        NV
    }
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
    // SAFETY: m, d valid. jacp/jacr have 3*nv elements (may be null). point has 3 elements.
    unsafe {
        const MJJNT_BALL: i32 = 1;
        const MJJNT_FREE: i32 = 0;

        let nv = (*m).nv as i32;
        let mut offset: [f64; 3] = [0.0; 3];
        let mut pvel: [f64; 6] = [0.0; 6];

        // clear jacobians, compute offset and pvel if required
        if !jacp.is_null() {
            crate::engine::engine_util_blas::mju_zero(jacp, 3 * nv);
            let com = (*d).subtree_com.add(3 * *(*m).body_rootid.add(body as usize) as usize);
            crate::engine::engine_util_blas::mju_sub3(offset.as_mut_ptr(), point, com);
            crate::engine::engine_util_spatial::mju_transform_spatial(
                pvel.as_mut_ptr(), (*d).cvel.add(6 * body as usize), 0, point, com, core::ptr::null());
        }
        if !jacr.is_null() {
            crate::engine::engine_util_blas::mju_zero(jacr, 3 * nv);
        }

        // skip fixed bodies
        let body = *(*m).body_weldid.add(body as usize);
        if body == 0 {
            return;
        }

        // get last dof that affects this body
        let mut i = *(*m).body_dofadr.add(body as usize) + *(*m).body_dofnum.add(body as usize) - 1;

        // backward pass over dof ancestor chain
        while i >= 0 {
            let mut cdof_dot: [f64; 6] = [0.0; 6];
            crate::engine::engine_inline::mji_copy6(cdof_dot.as_mut_ptr(), (*d).cdof_dot.add(6 * i as usize));
            let cdof = (*d).cdof.add(6 * i as usize);

            // check for quaternion
            let jnt_type = *(*m).jnt_type.add(*(*m).dof_jntid.add(i as usize) as usize);
            let dofadr = *(*m).jnt_dofadr.add(*(*m).dof_jntid.add(i as usize) as usize);
            let is_quat = jnt_type == MJJNT_BALL || (jnt_type == MJJNT_FREE && i >= dofadr + 3);

            // compute cdof_dot for quaternion (use current body cvel)
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

            // construct translation jacobian (correct for rotation)
            if !jacp.is_null() {
                // first correction term
                let mut tmp1: [f64; 3] = [0.0; 3];
                crate::engine::engine_inline::mji_cross(tmp1.as_mut_ptr(), cdof_dot.as_ptr(), offset.as_ptr());

                // second correction term
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
    // SAFETY: m, d valid. objid is valid index for the given objtype. res has 6 elements.
    unsafe {
        const MJOBJ_BODY: i32 = 1;
        const MJOBJ_XBODY: i32 = 2;
        const MJOBJ_GEOM: i32 = 5;
        const MJOBJ_SITE: i32 = 6;
        const MJOBJ_CAMERA: i32 = 7;

        let mut bodyid: i32 = 0;
        let mut pos: *const f64 = core::ptr::null();
        let mut rot: *const f64 = core::ptr::null();

        // body-inertial
        if objtype == MJOBJ_BODY {
            bodyid = objid;
            pos = (*d).xipos.add(3 * objid as usize);
            rot = if flg_local != 0 { (*d).ximat.add(9 * objid as usize) } else { core::ptr::null() };
        }
        // body-regular
        else if objtype == MJOBJ_XBODY {
            bodyid = objid;
            pos = (*d).xpos.add(3 * objid as usize);
            rot = if flg_local != 0 { (*d).xmat.add(9 * objid as usize) } else { core::ptr::null() };
        }
        // geom
        else if objtype == MJOBJ_GEOM {
            bodyid = *(*m).geom_bodyid.add(objid as usize);
            pos = (*d).geom_xpos.add(3 * objid as usize);
            rot = if flg_local != 0 { (*d).geom_xmat.add(9 * objid as usize) } else { core::ptr::null() };
        }
        // site
        else if objtype == MJOBJ_SITE {
            bodyid = *(*m).site_bodyid.add(objid as usize);
            pos = (*d).site_xpos.add(3 * objid as usize);
            rot = if flg_local != 0 { (*d).site_xmat.add(9 * objid as usize) } else { core::ptr::null() };
        }
        // camera
        else if objtype == MJOBJ_CAMERA {
            bodyid = *(*m).cam_bodyid.add(objid as usize);
            pos = (*d).cam_xpos.add(3 * objid as usize);
            rot = if flg_local != 0 { (*d).cam_xmat.add(9 * objid as usize) } else { core::ptr::null() };
        }
        // object without spatial frame
        else {
            extern "C" { fn mju_error_impl(msg: *const i8); }
            mju_error_impl(b"invalid object type\0".as_ptr() as *const i8);
            return;
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
            (*d).subtree_com.add(3 * *(*m).body_rootid.add(bodyid as usize) as usize),
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
    // SAFETY: m, d valid. objid is valid index for the given objtype. res has 6 elements.
    unsafe {
        const MJOBJ_BODY: i32 = 1;
        const MJOBJ_XBODY: i32 = 2;
        const MJOBJ_GEOM: i32 = 5;
        const MJOBJ_SITE: i32 = 6;
        const MJOBJ_CAMERA: i32 = 7;

        let mut bodyid: i32 = 0;
        let mut pos: *const f64 = core::ptr::null();
        let mut rot: *const f64 = core::ptr::null();

        // body-inertial
        if objtype == MJOBJ_BODY {
            bodyid = objid;
            pos = (*d).xipos.add(3 * objid as usize);
            rot = if flg_local != 0 { (*d).ximat.add(9 * objid as usize) } else { core::ptr::null() };
        }
        // body-regular
        else if objtype == MJOBJ_XBODY {
            bodyid = objid;
            pos = (*d).xpos.add(3 * objid as usize);
            rot = if flg_local != 0 { (*d).xmat.add(9 * objid as usize) } else { core::ptr::null() };
        }
        // geom
        else if objtype == MJOBJ_GEOM {
            bodyid = *(*m).geom_bodyid.add(objid as usize);
            pos = (*d).geom_xpos.add(3 * objid as usize);
            rot = if flg_local != 0 { (*d).geom_xmat.add(9 * objid as usize) } else { core::ptr::null() };
        }
        // site
        else if objtype == MJOBJ_SITE {
            bodyid = *(*m).site_bodyid.add(objid as usize);
            pos = (*d).site_xpos.add(3 * objid as usize);
            rot = if flg_local != 0 { (*d).site_xmat.add(9 * objid as usize) } else { core::ptr::null() };
        }
        // camera
        else if objtype == MJOBJ_CAMERA {
            bodyid = *(*m).cam_bodyid.add(objid as usize);
            pos = (*d).cam_xpos.add(3 * objid as usize);
            rot = if flg_local != 0 { (*d).cam_xmat.add(9 * objid as usize) } else { core::ptr::null() };
        }
        // object without spatial frame
        else {
            extern "C" { fn mju_error_impl(msg: *const i8); }
            mju_error_impl(b"invalid object type\0".as_ptr() as *const i8);
            return;
        }

        // static body: quick return
        if *(*m).body_weldid.add(bodyid as usize) == 0 {
            crate::engine::engine_util_blas::mju_zero(res, 6);
            return;
        }

        // transform com-based acceleration to local frame
        crate::engine::engine_util_spatial::mju_transform_spatial(
            res,
            (*d).cacc.add(6 * bodyid as usize),
            0,
            pos,
            (*d).subtree_com.add(3 * *(*m).body_rootid.add(bodyid as usize) as usize),
            rot,
        );

        // transform com-based velocity to local frame
        let mut vel: [f64; 6] = [0.0; 6];
        crate::engine::engine_util_spatial::mju_transform_spatial(
            vel.as_mut_ptr(),
            (*d).cvel.add(6 * bodyid as usize),
            0,
            pos,
            (*d).subtree_com.add(3 * *(*m).body_rootid.add(bodyid as usize) as usize),
            rot,
        );

        // add Coriolis correction due to rotating frame: acc_tran += vel_rot x vel_tran
        let mut correction: [f64; 3] = [0.0; 3];
        crate::engine::engine_inline::mji_cross(correction.as_mut_ptr(), vel.as_ptr(), vel.as_ptr().add(3));
        crate::engine::engine_inline::mji_add_to3(res.add(3), correction.as_ptr());
    }
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
    // SAFETY: m, d valid. xpos has 3*nodenum elements. vel may be null.
    unsafe {
        const MJOBJ_BODY: i32 = 1;

        let nodenum = *(*m).flex_nodenum.add(f as usize);
        let nstart = *(*m).flex_nodeadr.add(f as usize);
        let bodyid = (*m).flex_nodebodyid.add(nstart as usize);

        // compute positions and velocities
        let mut i: i32 = 0;
        while i < nodenum {
            let bid = *bodyid.add(i as usize);
            let centered = *((*m).flex_centered as *const u8).add(f as usize) != 0;
            if centered
                || (*(*m).flex_node.add(3 * (i + nstart) as usize + 0) == 0.0
                    && *(*m).flex_node.add(3 * (i + nstart) as usize + 1) == 0.0
                    && *(*m).flex_node.add(3 * (i + nstart) as usize + 2) == 0.0) {
                crate::engine::engine_util_blas::mju_copy3(xpos.add(3 * i as usize), (*d).xpos.add(3 * bid as usize));
            } else {
                crate::engine::engine_util_blas::mju_mul_mat_vec3(xpos.add(3 * i as usize), (*d).xmat.add(9 * bid as usize), (*m).flex_node.add(3 * (i + nstart) as usize));
                crate::engine::engine_util_blas::mju_add_to3(xpos.add(3 * i as usize), (*d).xpos.add(3 * bid as usize));
            }

            if !vel.is_null() {
                let mut body_vel: [f64; 6] = [0.0; 6];
                mj_object_velocity(m, d, MJOBJ_BODY, bid, body_vel.as_mut_ptr(), 0);

                // linear velocity at CoM
                crate::engine::engine_util_blas::mju_copy3(vel.add(3 * i as usize), body_vel.as_ptr().add(3));

                // add omega x (xpos - xipos)
                let mut r: [f64; 3] = [0.0; 3];
                let mut cross: [f64; 3] = [0.0; 3];
                crate::engine::engine_util_blas::mju_sub3(r.as_mut_ptr(), xpos.add(3 * i as usize), (*d).xipos.add(3 * bid as usize));
                crate::engine::engine_util_spatial::mju_cross(cross.as_mut_ptr(), body_vel.as_ptr(), r.as_ptr());
                crate::engine::engine_util_blas::mju_add_to3(vel.add(3 * i as usize), cross.as_ptr());
            }
            i += 1;
        }

        // shell mode: reconstruct interior node positions and velocities via TFI
        let interp = *(*m).flex_interp.add(f as usize);
        if interp < 0 {
            let order = -interp;
            let cx = *(*m).flex_cellnum.add(3 * f as usize + 0);
            let cy = *(*m).flex_cellnum.add(3 * f as usize + 1);
            let cz = *(*m).flex_cellnum.add(3 * f as usize + 2);
            let nx_g = cx * order + 1;
            let ny_g = cy * order + 1;
            let nz_g = cz * order + 1;

            crate::engine::engine_util_misc::mju_shell_track_interior(xpos, nx_g, ny_g, nz_g);
            if !vel.is_null() {
                crate::engine::engine_util_misc::mju_shell_track_interior(vel, nx_g, ny_g, nz_g);
            }
        }
    }
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
    // SAFETY: m, d valid. result has 6 elements. id is checked before use.
    unsafe {
        // clear result
        crate::engine::engine_util_blas::mju_zero(result, 6);

        // make sure contact is valid
        if id >= 0 && id < (*d).ncon && (*(*d).contact.add(id as usize)).efc_address >= 0 {
            // get contact pointer
            let con = &*(*d).contact.add(id as usize);

            if mj_is_pyramidal(m) != 0 {
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

