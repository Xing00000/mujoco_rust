//! Port of: engine/engine_setconst.c
//! IR hash: 05737965add36adb
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mj_setM0 (engine/engine_setconst.c:37)
/// Calls: mj_actuatorArmature, mju_addTo, mju_copy, mju_dot, mju_mulInertVec
#[allow(unused_variables, non_snake_case)]
pub fn mj_set_m0(m: *mut mjModel, d: *mut mjData) {
    extern "C" { fn mj_setM0(m: *mut mjModel, d: *mut mjData); }
    // SAFETY: delegates to C implementation
    unsafe { mj_setM0(m, d) }
}

/// C: GetWrapBodyTreeId (engine/engine_setconst.c:64)
#[allow(unused_variables, non_snake_case)]
pub fn get_wrap_body_tree_id(m: *const mjModel, wrap_index: i32) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, wrap_index : i32)
    // Previous return: i32
    extern "C" { fn GetWrapBodyTreeId(m : * const mjModel , wrap_index : i32) -> i32 ; } unsafe { GetWrapBodyTreeId(m , wrap_index) }
}

/// C: setFixed (engine/engine_setconst.c:86)
/// Calls: GetWrapBodyTreeId, mj_freeStack, mj_markStack, mj_stackAllocInfo, mju_fillInt, mju_isZero, mju_message, mju_zeroInt
#[allow(unused_variables, non_snake_case)]
pub fn set_fixed(m: *mut mjModel, d: *mut mjData) {
    extern "C" { fn setFixed(m: *mut mjModel, d: *mut mjData); }
    // SAFETY: delegates to C implementation
    unsafe { setFixed(m, d) }
}

/// C: makeTendonSparse (engine/engine_setconst.c:337)
/// Calls: mju_zeroInt
#[allow(unused_variables, non_snake_case)]
pub fn make_tendon_sparse(m: *mut mjModel) {
    // SAFETY: m valid per caller. All pointer arithmetic within allocated model arrays.
    unsafe {
        const MJWRAP_JOINT: i32 = 1;
        const MJWRAP_SITE: i32 = 3;
        const MJWRAP_SPHERE: i32 = 4;
        const MJWRAP_CYLINDER: i32 = 5;

        let ntendon: i32 = (*m).ntendon as i32;
        let rownnz: *mut i32 = (*m).ten_J_rownnz;
        let rowadr: *mut i32 = (*m).ten_J_rowadr;
        let colind: *mut i32 = (*m).ten_J_colind;

        if ntendon == 0 {
            return;
        }

        // clear
        crate::engine::engine_util_misc::mju_zero_int(rownnz, ntendon);
        crate::engine::engine_util_misc::mju_zero_int(rowadr, ntendon);

        // compute rownnz, rowadr, and colind for each tendon
        let mut i: i32 = 0;
        while i < ntendon {
            *rowadr.add(i as usize) = if i > 0 { *rowadr.add((i - 1) as usize) + *rownnz.add((i - 1) as usize) } else { 0 };
            let adr: i32 = *(*m).tendon_adr.add(i as usize);
            let num: i32 = *(*m).tendon_num.add(i as usize);

            // joint tendon: each wrap object is a joint, colind is its dofadr
            if *(*m).wrap_type.add(adr as usize) == MJWRAP_JOINT {
                let mut j: i32 = 0;
                while j < num {
                    *colind.add((*rowadr.add(i as usize) + j) as usize) =
                        *(*m).jnt_dofadr.add(*(*m).wrap_objid.add((adr + j) as usize) as usize);
                    j += 1;
                }
                *rownnz.add(i as usize) = num;
            } else {
                // spatial tendon: collect used dofs from wrap object bodies
                let mut nnz: i32 = 0;
                let mut j: i32 = 0;
                while j < num {
                    let wtype: i32 = *(*m).wrap_type.add((adr + j) as usize);

                    // get body id from site or geom wrap object
                    let mut bodyid: i32 = -1;
                    if wtype == MJWRAP_SITE {
                        bodyid = *(*m).site_bodyid.add(*(*m).wrap_objid.add((adr + j) as usize) as usize);
                    } else if wtype == MJWRAP_SPHERE || wtype == MJWRAP_CYLINDER {
                        bodyid = *(*m).geom_bodyid.add(*(*m).wrap_objid.add((adr + j) as usize) as usize);
                    }

                    // walk up the body tree, collecting used dofs
                    if bodyid > 0 {
                        let mut bid: i32 = bodyid;
                        while bid > 0 {
                            let bdofadr: i32 = *(*m).body_dofadr.add(bid as usize);
                            let bdofnum: i32 = *(*m).body_dofnum.add(bid as usize);
                            let mut k: i32 = 0;
                            while k < bdofnum {
                                let dof: i32 = bdofadr + k;

                                // check if dof already in colind
                                let mut found: i32 = 0;
                                let mut l: i32 = 0;
                                while l < nnz {
                                    if *colind.add((*rowadr.add(i as usize) + l) as usize) == dof {
                                        found = 1;
                                        break;
                                    }
                                    l += 1;
                                }

                                // append new dof
                                if found == 0 {
                                    *colind.add((*rowadr.add(i as usize) + nnz) as usize) = dof;
                                    nnz += 1;
                                }
                                k += 1;
                            }
                            bid = *(*m).body_parentid.add(bid as usize);
                        }
                    }
                    j += 1;
                }
                *rownnz.add(i as usize) = nnz;
            }

            // sort colind for this tendon
            let nnz: i32 = *rownnz.add(i as usize);
            let mut j: i32 = 0;
            while j < nnz - 1 {
                let mut k: i32 = j + 1;
                while k < nnz {
                    // swap out-of-order entries
                    if *colind.add((*rowadr.add(i as usize) + k) as usize) < *colind.add((*rowadr.add(i as usize) + j) as usize) {
                        let tmp: i32 = *colind.add((*rowadr.add(i as usize) + j) as usize);
                        *colind.add((*rowadr.add(i as usize) + j) as usize) = *colind.add((*rowadr.add(i as usize) + k) as usize);
                        *colind.add((*rowadr.add(i as usize) + k) as usize) = tmp;
                    }
                    k += 1;
                }
                j += 1;
            }

            i += 1;
        }
    }
}

/// C: makeFlexSparse (engine/engine_setconst.c:424)
/// Calls: mj_bodyChain, mj_freeStack, mj_jacDifPair, mj_markStack, mj_stackAllocInfo, mju_copyInt, mju_message, mju_sub3, mju_zero, mju_zeroInt
#[allow(unused_variables, non_snake_case)]
pub fn make_flex_sparse(m: *mut mjModel, d: *mut mjData) {
    extern "C" { fn makeFlexSparse(m: *mut mjModel, d: *mut mjData); }
    // SAFETY: delegates to C implementation
    unsafe { makeFlexSparse(m, d) }
}

/// C: mj_alignFlex (engine/engine_setconst.c:648)
/// Calls: mju_copy3, mju_cross, mju_mulMatTVec3, mju_normalize3, mju_quat2Mat, mju_quatZ2Vec, mju_sub3, mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn mj_align_flex(m: *mut mjModel, d: *mut mjData) {
    extern "C" { fn mj_alignFlex(m: *mut mjModel, d: *mut mjData); }
    // SAFETY: delegates to C implementation
    unsafe { mj_alignFlex(m, d) }
}

/// C: set0 (engine/engine_setconst.c:695)
/// Calls: makeFlexSparse, makeTendonSparse, mj_alignFlex, mj_camlight, mj_comPos, mj_factorM, mj_flex, mj_freeStack, mj_jacBodyCom, mj_kinematics, mj_local2Global, mj_makeM, mj_markStack, mj_setM0, mj_solveM, mj_stackAllocInfo, mj_tendon, mj_transmission, mju_copy, mju_copy3, mju_copy9, mju_dot, mju_isZero, mju_max, mju_message, mju_mulMatMatT, mju_mulMatTVec3, mju_mulQuat, mju_negQuat, mju_norm, mju_normalize4, mju_sparse2dense, mju_sub3, mju_subFrom3, mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn set0(m: *mut mjModel, d: *mut mjData) {
    extern "C" { fn set0(m: *mut mjModel, d: *mut mjData); }
    // SAFETY: delegates to C implementation
    unsafe { set0(m, d) }
}

/// C: updateBox (engine/engine_setconst.c:1041)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn update_box(xmin: *mut f64, xmax: *mut f64, pos: *mut f64, radius: f64) {
    // SAFETY: xmin, xmax, pos each point to at least 3 f64 elements per caller contract
    unsafe {
        for i in 0..3 {
            let lo = *pos.add(i) - radius;
            let hi = *pos.add(i) + radius;
            if lo < *xmin.add(i) { *xmin.add(i) = lo; }
            if hi > *xmax.add(i) { *xmax.add(i) = hi; }
        }
    }
}

/// C: setStat (engine/engine_setconst.c:1050)
/// Calls: mj_freeStack, mj_markStack, mj_stackAllocInfo, mju_add3, mju_dist3, mju_max, mju_scl3, mju_zero, updateBox
#[allow(unused_variables, non_snake_case)]
pub fn set_stat(m: *mut mjModel, d: *mut mjData) {
    extern "C" { fn setStat(m: *mut mjModel, d: *mut mjData); }
    // SAFETY: delegates to C implementation
    unsafe { setStat(m, d) }
}

/// C: setSpring (engine/engine_setconst.c:1198)
/// Calls: mj_comPos, mj_kinematics, mj_tendon, mj_transmission, mju_copy
#[allow(unused_variables, non_snake_case)]
pub fn set_spring(m: *mut mjModel, d: *mut mjData) {
    extern "C" { fn setSpring(m: *mut mjModel, d: *mut mjData); }
    // SAFETY: delegates to C implementation
    unsafe { setSpring(m, d) }
}

/// C: evalAct (engine/engine_setconst.c:1235)
/// Calls: mj_freeStack, mj_markStack, mj_solveM, mj_stackAllocInfo, mj_step1, mj_step2, mju_norm, mju_scl, mju_sparse2dense
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn eval_act(m: *const mjModel, d: *mut mjData, index: i32, side: i32, opt: *const mjLROpt) -> f64 {
    extern "C" { fn evalAct(m: *const mjModel, d: *mut mjData, index: i32, side: i32, opt: *const mjLROpt) -> f64; }
    // SAFETY: delegates to C implementation
    unsafe { evalAct(m, d, index, side, opt) }
}

/// C: mj_setConst (engine/engine_setconst.h:27)
/// Calls: set0, setFixed, setSpring, setStat
#[allow(unused_variables, non_snake_case)]
pub fn mj_set_const(m: *mut mjModel, d: *mut mjData) {
    set_fixed(m, d);
    set0(m, d);
    set_stat(m, d);
    set_spring(m, d);
}

/// C: mj_setLengthRange (engine/engine_setconst.h:30)
/// Calls: evalAct, mj_resetData, mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_set_length_range(m: *mut mjModel, d: *mut mjData, index: i32, opt: *const mjLROpt, error: *mut i8, error_sz: i32) -> i32 {
    extern "C" { fn mj_setLengthRange(m: *mut mjModel, d: *mut mjData, index: i32, opt: *const mjLROpt, error: *mut i8, error_sz: i32) -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { mj_setLengthRange(m, d, index, opt, error, error_sz) }
}

