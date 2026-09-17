//! Port of: engine/engine_core_util.c
//! IR hash: fd5288ad7242f001
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mj_isSparse (engine/engine_core_util.c:33)
#[allow(unused_variables, non_snake_case)]
pub fn mj_is_sparse(m: *const mjModel) -> i32 {
    todo!() // cxx:_mj_isSparse
}

/// C: mj_isPyramidal (engine/engine_core_util.c:44)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_context.c:_cone
#[allow(unused_variables, non_snake_case)]
pub fn mj_is_pyramidal(m: *const mjModel) -> i32 {
    todo!() // cxx:_mj_isPyramidal
}

/// C: mj_mergeChain (engine/engine_core_util.c:56)
/// Calls: cxx:__ZNK6mujoco4user8FilePath5emptyEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_merge_chain(m: *const mjModel, chain: *mut i32, b1: i32, b2: i32, flg_skipcommon: i32) -> i32 {
    todo!() // cxx:_mj_mergeChain
}

/// C: mj_mergeChainSimple (engine/engine_core_util.c:100)
#[allow(unused_variables, non_snake_case)]
pub fn mj_merge_chain_simple(m: *const mjModel, chain: *mut i32, b1: i32, b2: i32) -> i32 {
    todo!() // cxx:_mj_mergeChainSimple
}

/// C: mj_bodyChain (engine/engine_core_util.c:132)
/// Calls: cxx:__ZNK6mujoco4user8FilePath5emptyEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_body_chain(m: *const mjModel, body: i32, chain: *mut i32) -> i32 {
    todo!() // cxx:_mj_bodyChain
}

/// C: mj_jac (engine/engine_core_util.c:178)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_cross, cxx:_mju_sub3, cxx:_mju_zero
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

/// C: mj_jacBody (engine/engine_core_util.c:229)
/// Calls: cxx:_mj_jac
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_jac_body(m: *const mjModel, d: *const mjData, jacp: *mut f64, jacr: *mut f64, body: i32) {
    todo!() // cxx:_mj_jacBody
}

/// C: mj_jacBodyCom (engine/engine_core_util.c:235)
/// Calls: cxx:_mj_jac
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_jac_body_com(m: *const mjModel, d: *const mjData, jacp: *mut f64, jacr: *mut f64, body: i32) {
    todo!() // cxx:_mj_jacBodyCom
}

/// C: mj_jacSubtreeCom (engine/engine_core_util.c:241)
/// Calls: cxx:_mj_freeStack, cxx:_mj_jac, cxx:_mj_markStack, cxx:_mj_stackAllocInfo, cxx:_mju_addToScl, cxx:_mju_scl, cxx:_mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_jac_subtree_com(m: *const mjModel, d: *mut mjData, jacp: *mut f64, body: i32) {
    todo!() // cxx:_mj_jacSubtreeCom
}

/// C: mj_jacGeom (engine/engine_core_util.c:269)
/// Calls: cxx:_mj_jac
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_jac_geom(m: *const mjModel, d: *const mjData, jacp: *mut f64, jacr: *mut f64, geom: i32) {
    todo!() // cxx:_mj_jacGeom
}

/// C: mj_jacSite (engine/engine_core_util.c:275)
/// Calls: cxx:_mj_jac
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_jac_site(m: *const mjModel, d: *const mjData, jacp: *mut f64, jacr: *mut f64, site: i32) {
    todo!() // cxx:_mj_jacSite
}

/// C: mj_jacPointAxis (engine/engine_core_util.c:282)
/// Calls: cxx:_mj_freeStack, cxx:_mj_jac, cxx:_mj_markStack, cxx:_mj_stackAllocInfo
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_jac_point_axis(m: *const mjModel, d: *mut mjData, jacPoint: *mut f64, jacAxis: *mut f64, point: *const f64, axis: *const f64, body: i32) {
    todo!() // cxx:_mj_jacPointAxis
}

/// C: mj_jacSparse (engine/engine_core_util.c:307)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_cross, cxx:_mju_message, cxx:_mju_sub3, cxx:_mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_jac_sparse(m: *const mjModel, d: *const mjData, jacp: *mut f64, jacr: *mut f64, point: *const f64, body: i32, NV: i32, chain: *const i32, flg_skipcommon: i32) {
    todo!() // cxx:_mj_jacSparse
}

/// C: mj_jacSparseSimple (engine/engine_core_util.c:378)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_cross, cxx:_mju_sub3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_jac_sparse_simple(m: *const mjModel, d: *const mjData, jacdifp: *mut f64, jacdifr: *mut f64, point: *const f64, body: i32, flg_second: i32, NV: i32, start: i32) {
    todo!() // cxx:_mj_jacSparseSimple
}

/// C: mj_jacDifPair (engine/engine_core_util.c:442)
/// Calls: cxx:_mj_jac, cxx:_mj_jacSparse, cxx:_mj_jacSparseSimple, cxx:_mj_mergeChain, cxx:_mj_mergeChainSimple, cxx:_mju_sub
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_jac_dif_pair(m: *const mjModel, d: *const mjData, chain: *mut i32, b1: i32, b2: i32, pos1: *const f64, pos2: *const f64, jac1p: *mut f64, jac2p: *mut f64, jacdifp: *mut f64, jac1r: *mut f64, jac2r: *mut f64, jacdifr: *mut f64, issparse: i32, flg_skipcommon: i32) -> i32 {
    todo!() // cxx:_mj_jacDifPair
}

/// C: mj_jacSum (engine/engine_core_util.c:521)
/// Calls: cxx:_mj_bodyChain, cxx:_mj_freeStack, cxx:_mj_isSparse, cxx:_mj_jac, cxx:_mj_jacSparse, cxx:_mj_jacSparseSimple, cxx:_mj_markStack, cxx:_mj_stackAllocInfo, cxx:_mju_addToScl, cxx:_mju_addToSparseMat, cxx:_mju_copy, cxx:_mju_scl
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_jac_sum(m: *const mjModel, d: *mut mjData, chain: *mut i32, n: i32, body: *const i32, weight: *const f64, point: *const f64, jacp: *mut f64, jacr: *mut f64, flg_rot: i32) -> i32 {
    todo!() // cxx:_mj_jacSum
}

/// C: mj_jacDot (engine/engine_core_util.c:607)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_copy6, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_cross, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_crossMotion, cxx:_mju_sub3, cxx:_mju_transformSpatial, cxx:_mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_jac_dot(m: *const mjModel, d: *const mjData, jacp: *mut f64, jacr: *mut f64, point: *const f64, body: i32) {
    todo!() // cxx:_mj_jacDot
}

/// C: mj_jacDotSparse (engine/engine_core_util.c:681)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_copy6, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_cross, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_crossMotion, cxx:_mju_message, cxx:_mju_sub3, cxx:_mju_transformSpatial, cxx:_mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_jac_dot_sparse(m: *const mjModel, d: *const mjData, jacp: *mut f64, jacr: *mut f64, point: *const f64, body: i32, NV: i32, chain: *const i32) {
    todo!() // cxx:_mj_jacDotSparse
}

/// C: mj_angmomMat (engine/engine_core_util.c:765)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_copy9, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_mulMatMat3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_sub3, cxx:_mj_freeStack, cxx:_mj_jacBodyCom, cxx:_mj_markStack, cxx:_mj_stackAllocInfo, cxx:_mju_addTo, cxx:_mju_copy3, cxx:_mju_mulMatMat, cxx:_mju_mulMatMatT3, cxx:_mju_scl, cxx:_mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_angmom_mat(m: *const mjModel, d: *mut mjData, mat: *mut f64, body: i32) {
    todo!() // cxx:_mj_angmomMat
}

/// C: mj_objectVelocity (engine/engine_core_util.c:837)
/// Calls: cxx:_mju_message, cxx:_mju_transformSpatial, cxx:_mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_object_velocity(m: *const mjModel, d: *const mjData, objtype: i32, objid: i32, res: *mut f64, flg_local: i32) {
    todo!() // cxx:_mj_objectVelocity
}

/// C: mj_geomSurfaceVelocity (engine/engine_core_util.c:894)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_addTo3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_cross, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_mulMatVec3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_sub3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_geom_surface_velocity(m: *const mjModel, d: *const mjData, geomid: i32, point: *const f64, linear: *mut f64, angular: *mut f64) {
    todo!() // cxx:_mj_geomSurfaceVelocity
}

/// C: mj_objectAcceleration (engine/engine_core_util.c:911)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_addTo3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_cross, cxx:_mju_message, cxx:_mju_transformSpatial, cxx:_mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_object_acceleration(m: *const mjModel, d: *const mjData, objtype: i32, objid: i32, res: *mut f64, flg_local: i32) {
    todo!() // cxx:_mj_objectAcceleration
}

/// C: mj_local2Global (engine/engine_core_util.c:978)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_addTo3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_copy3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_copy9, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_mulMatVec3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_mulQuat, cxx:_mju_quat2Mat
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_local2global(d: *mut mjData, xpos: *mut f64, xmat: *mut f64, pos: *const f64, quat: *const f64, body: i32, sameframe: u8) {
    todo!() // cxx:_mj_local2Global
}

/// C: mju_flexGatherState (engine/engine_core_util.c:1023)
/// Calls: cxx:_mj_objectVelocity, cxx:_mju_addTo3, cxx:_mju_copy3, cxx:_mju_cross, cxx:_mju_mulMatVec3, cxx:_mju_shellTrackInterior, cxx:_mju_sub3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_flex_gather_state(m: *const mjModel, d: *const mjData, f: i32, xpos: *mut f64, vel: *mut f64) {
    todo!() // cxx:_mju_flexGatherState
}

/// C: mj_contactForce (engine/engine_core_util.c:1076)
/// Calls: cxx:_mj_isPyramidal, cxx:_mju_copy, cxx:_mju_decodePyramid, cxx:_mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_contact_force(m: *const mjModel, d: *const mjData, id: i32, result: *mut f64) {
    todo!() // cxx:_mj_contactForce
}

/// C: tendonLimit (engine/engine_core_util.c:1100)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn tendon_limit(m: *const mjModel, ten_length: *const f64, i: i32) -> i32 {
    todo!() // cxx:_tendonLimit
}

/// C: mj_actuatorDamping (engine/engine_core_util.c:1120)
/// Calls: cxx:_mju_message
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_actuator_damping(m: *const mjModel, r#type: u32, id: i32, poly: *mut f64) -> f64 {
    todo!() // cxx:_mj_actuatorDamping
}

/// C: mj_actuatorArmature (engine/engine_core_util.c:1174)
/// Calls: cxx:_mju_message
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_actuator_armature(m: *const mjModel, r#type: u32, id: i32) -> f64 {
    todo!() // cxx:_mj_actuatorArmature
}

/// C: mj_warning (engine/engine_core_util.c:1223)
/// Calls: cxx:_mju_message, cxx:_mju_warning, cxx:_mju_warningText
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
                crate::engine::engine_util_misc::mju_warningText(warning, info as usize));
        }

        // increase counter
        *number_ptr += 1;
    }
}

/// C: mj_isMetric (engine/engine_core_util.c:1246)
#[allow(unused_variables, non_snake_case)]
pub fn mj_is_metric(m: *const mjModel) -> i32 {
    todo!() // cxx:_mj_isMetric
}

/// C: mj_effCouplings (engine/engine_core_util.c:1258)
/// Calls: cxx:_mj_isMetric
#[allow(unused_variables, non_snake_case)]
pub fn mj_eff_couplings(m: *const mjModel) -> i32 {
    todo!() // cxx:_mj_effCouplings
}

/// C: mj_tendonHasStiffness (engine/engine_core_util.c:1264)
/// Calls: cxx:_mju_isZero
#[allow(unused_variables, non_snake_case)]
pub fn mj_tendon_has_stiffness(m: *const mjModel, i: i32) -> i32 {
    todo!() // cxx:_mj_tendonHasStiffness
}

/// C: mj_tendonHasDamping (engine/engine_core_util.c:1271)
/// Calls: cxx:_mju_isZero
#[allow(unused_variables, non_snake_case)]
pub fn mj_tendon_has_damping(m: *const mjModel, i: i32) -> i32 {
    todo!() // cxx:_mj_tendonHasDamping
}

/// C: mj_effFlexContactPossible (engine/engine_core_util.c:1281)
#[allow(unused_variables, non_snake_case)]
pub fn mj_eff_flex_contact_possible(m: *const mjModel, f: i32) -> i32 {
    todo!() // cxx:_mj_effFlexContactPossible
}

/// C: mj_effFlexStiffPossible (engine/engine_core_util.c:1289)
/// Calls: cxx:_mju_isZero
#[allow(unused_variables, non_snake_case)]
pub fn mj_eff_flex_stiff_possible(m: *const mjModel, f: i32) -> i32 {
    todo!() // cxx:_mj_effFlexStiffPossible
}

/// C: mj_effFlexPossible (engine/engine_core_util.c:1319)
/// Calls: cxx:_mj_effFlexContactPossible, cxx:_mj_effFlexStiffPossible
#[allow(unused_variables, non_snake_case)]
pub fn mj_eff_flex_possible(m: *const mjModel, f: i32) -> i32 {
    todo!() // cxx:_mj_effFlexPossible
}

/// C: mj_effTendonPossible (engine/engine_core_util.c:1326)
/// Calls: cxx:_mj_tendonHasDamping, cxx:_mj_tendonHasStiffness
#[allow(unused_variables, non_snake_case)]
pub fn mj_eff_tendon_possible(m: *const mjModel, i: i32) -> i32 {
    todo!() // cxx:_mj_effTendonPossible
}

/// C: mj_effActuatorPossible (engine/engine_core_util.c:1333)
#[allow(unused_variables, non_snake_case)]
pub fn mj_eff_actuator_possible(m: *const mjModel, i: i32) -> i32 {
    todo!() // cxx:_mj_effActuatorPossible
}

