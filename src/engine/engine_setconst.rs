//! Port of: engine/engine_setconst.c
//! IR hash: 3fb6da908ad9d71c
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mj_setM0 (engine/engine_setconst.c:37)
/// Calls: mj_actuatorArmature, mju_addTo, mju_copy, mju_dot, mju_mulInertVec
#[allow(unused_variables, non_snake_case)]
pub fn mj_set_m0(m: *mut mjModel, d: *mut mjData) {
    todo!() // mj_setM0
}

/// C: GetWrapBodyTreeId (engine/engine_setconst.c:64)
#[allow(unused_variables, non_snake_case)]
pub fn get_wrap_body_tree_id(m: *const mjModel, wrap_index: i32) -> i32 {
    // SAFETY: caller guarantees m is valid and wrap_index is in bounds
    unsafe {
        let mut bodyid: i32 = -1;
        let objid: i32 = *(*m).wrap_objid.add(wrap_index as usize);

        match *(*m).wrap_type.add(wrap_index as usize) {
            1 => { // mjWRAP_JOINT
                bodyid = *(*m).jnt_bodyid.add(objid as usize);
            }
            4 => { // mjWRAP_SITE
                bodyid = *(*m).site_bodyid.add(objid as usize);
            }
            2 | 3 => { // mjWRAP_SPHERE | mjWRAP_CYLINDER
                bodyid = *(*m).geom_bodyid.add(objid as usize);
            }
            5 | 0 => { // mjWRAP_PULLEY | mjWRAP_NONE
            }
            _ => {}
        }

        if bodyid != -1 {
            *(*m).body_treeid.add(bodyid as usize)
        } else {
            -1
        }
    }
}

/// C: setFixed (engine/engine_setconst.c:86)
/// Calls: GetWrapBodyTreeId, mj_freeStack, mj_markStack, mj_stackAllocInfo, mju_fillInt, mju_isZero, mju_message, mju_zeroInt
#[allow(unused_variables, non_snake_case)]
pub fn set_fixed(m: *mut mjModel, d: *mut mjData) {
    todo!() // setFixed
}

/// C: makeTendonSparse (engine/engine_setconst.c:337)
/// Calls: mju_zeroInt
#[allow(unused_variables, non_snake_case)]
pub fn make_tendon_sparse(m: *mut mjModel) {
    todo!() // makeTendonSparse
}

/// C: makeFlexSparse (engine/engine_setconst.c:424)
/// Calls: mj_bodyChain, mj_freeStack, mj_jacDifPair, mj_markStack, mj_stackAllocInfo, mju_copyInt, mju_message, mju_sub3, mju_zero, mju_zeroInt
#[allow(unused_variables, non_snake_case)]
pub fn make_flex_sparse(m: *mut mjModel, d: *mut mjData) {
    const MJ_MINVAL: f64 = 1e-14;

    // SAFETY: m, d are valid pointers (caller contract)
    unsafe {
        let nv = (*m).nv as i32;
        let rowadr = (*m).flexedge_J_rowadr;
        let rownnz = (*m).flexedge_J_rownnz;
        let colind = (*m).flexedge_J_colind;
        let vrowadr = (*m).flexvert_J_rowadr;
        let vrownnz = (*m).flexvert_J_rownnz;

        if (*m).nflex == 0 {
            return;
        }

        let nflex = (*m).nflex as i32;

        crate::engine::engine_memory::mj_mark_stack(d);
        let chain = crate::engine::engine_memory::mj_stack_alloc_int(d, nv as usize);
        let chain1 = crate::engine::engine_memory::mj_stack_alloc_int(d, nv as usize);
        let chain2 = crate::engine::engine_memory::mj_stack_alloc_int(d, nv as usize);
        let buf_ind = crate::engine::engine_memory::mj_stack_alloc_int(d, nv as usize);
        let dummy_pos = crate::engine::engine_memory::mj_stack_alloc_num(d, 3);
        crate::engine::engine_util_blas::mju_zero(dummy_pos, 3);

        // clear
        let nflexedge = (*m).nflexedge as i32;
        let nflexvert = (*m).nflexvert as i32;
        crate::engine::engine_util_misc::mju_zero_int(rowadr, nflexedge);
        crate::engine::engine_util_misc::mju_zero_int(rownnz, nflexedge);
        crate::engine::engine_util_misc::mju_zero_int(vrowadr, 2 * nflexvert);
        crate::engine::engine_util_misc::mju_zero_int(vrownnz, 2 * nflexvert);
        crate::engine::engine_util_misc::mju_zero_int((*m).flex_vertedgeadr, nflexvert);
        crate::engine::engine_util_misc::mju_zero_int((*m).flex_vertedgenum, nflexvert);
        crate::engine::engine_util_misc::mju_zero_int((*m).flex_vertedge, 2 * nflexedge);
        crate::engine::engine_util_blas::mju_zero((*m).flex_vertmetric, 4 * nflexvert);
        crate::engine::engine_util_misc::mju_zero_int((*m).flexedge_J_colind, (*m).nJfe as i32);
        crate::engine::engine_util_misc::mju_zero_int((*m).flexvert_J_colind, 2 * (*m).nJfv as i32);
        let mut current_adj_offset: i32 = 0;

        // compute lengths and Jacobians of edges
        for f in 0..nflex {
            if *(*m).flex_rigid.add(f as usize) || *(*m).flex_interp.add(f as usize) != 0 {
                continue;
            }

            let skipjacobian = *(*m).flex_edgeequality.add(f as usize) == 0
                && *(*m).flex_edgedamping.add(f as usize) == 0.0
                && *(*m).flex_edgestiffness.add(f as usize) == 0.0
                && *(*m).flex_damping.add(f as usize) == 0.0;

            let vbase = *(*m).flex_vertadr.add(f as usize);
            let ebase = *(*m).flex_edgeadr.add(f as usize);
            for e in 0..*(*m).flex_edgenum.add(f as usize) {
                if skipjacobian {
                    continue;
                }

                // set rowadr
                if ebase + e > 0 {
                    *rowadr.add((ebase + e) as usize) = *rowadr.add((ebase + e - 1) as usize) + *rownnz.add((ebase + e - 1) as usize);
                }

                let v1 = *(*m).flex_edge.add((2 * (ebase + e)) as usize);
                let v2 = *(*m).flex_edge.add((2 * (ebase + e) + 1) as usize);
                let b1 = *(*m).flex_vertbodyid.add((vbase + v1) as usize);
                let b2 = *(*m).flex_vertbodyid.add((vbase + v2) as usize);

                // get sparsity
                let NV = crate::engine::engine_core_util::mj_jac_dif_pair(
                    m, d as *const mjData, chain, b1, b2, dummy_pos, dummy_pos,
                    std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut(),
                    std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut(),
                    1, 0);

                *rownnz.add((ebase + e) as usize) = NV;
                crate::engine::engine_util_misc::mju_copy_int(
                    colind.add(*rowadr.add((ebase + e) as usize) as usize), chain, NV);
            }

            // dim=2 vertex-based constraint
            if *(*m).flex_dim.add(f as usize) == 2 && *(*m).flex_edgeequality.add(f as usize) == 2 {
                let nvert = *(*m).flex_vertnum.add(f as usize);
                let v_edge_cnt = (*m).flex_vertedgenum.add(vbase as usize);
                let v_edge_adr = (*m).flex_vertedgeadr.add(vbase as usize);
                let adj_edges = (*m).flex_vertedge;

                for e in 0..*(*m).flex_edgenum.add(f as usize) {
                    *v_edge_cnt.add(*(*m).flex_edge.add((2 * (ebase + e)) as usize) as usize) += 1;
                    *v_edge_cnt.add(*(*m).flex_edge.add((2 * (ebase + e) + 1) as usize) as usize) += 1;
                }
                let mut total_adj_edges: i32 = 0;
                for v in 0..nvert {
                    *v_edge_adr.add(v as usize) = current_adj_offset + total_adj_edges;
                    total_adj_edges += *v_edge_cnt.add(v as usize);
                }
                let v_edge_fill = crate::engine::engine_memory::mj_stack_alloc_int(d, nvert as usize);
                crate::engine::engine_util_misc::mju_zero_int(v_edge_fill, nvert);
                for e in 0..*(*m).flex_edgenum.add(f as usize) {
                    let ev1 = *(*m).flex_edge.add((2 * (ebase + e)) as usize);
                    let ev2 = *(*m).flex_edge.add((2 * (ebase + e) + 1) as usize);
                    *adj_edges.add((*v_edge_adr.add(ev1 as usize) + *v_edge_fill.add(ev1 as usize)) as usize) = e;
                    *v_edge_fill.add(ev1 as usize) += 1;
                    *adj_edges.add((*v_edge_adr.add(ev2 as usize) + *v_edge_fill.add(ev2 as usize)) as usize) = e;
                    *v_edge_fill.add(ev2 as usize) += 1;
                }

                // precompute metric (Binv)
                for v in 0..nvert {
                    let mut B = [0.0f64; 4];
                    let v_global = vbase + v;

                    for k in 0..*v_edge_cnt.add(v as usize) {
                        let edge_e = *adj_edges.add((*v_edge_adr.add(v as usize) + k) as usize);
                        let mut dx = [0.0f64; 3];
                        let ev1 = *(*m).flex_edge.add((2 * (ebase + edge_e)) as usize);
                        let ev2 = *(*m).flex_edge.add((2 * (ebase + edge_e) + 1) as usize);
                        crate::engine::engine_util_blas::mju_sub3(dx.as_mut_ptr(),
                            (*m).flex_vert0.add(3 * (vbase + ev2) as usize),
                            (*m).flex_vert0.add(3 * (vbase + ev1) as usize));
                        dx[0] *= 2.0 * *(*m).flex_size.add((3 * f) as usize);
                        dx[1] *= 2.0 * *(*m).flex_size.add((3 * f + 1) as usize);
                        dx[2] *= 2.0 * *(*m).flex_size.add((3 * f + 2) as usize);

                        let neighbor_v = if v == ev1 { ev2 } else { ev1 };
                        let b_neighbor = *(*m).flex_vertbodyid.add((vbase + neighbor_v) as usize);
                        let mut weight: f64 = 1.0;
                        if b_neighbor >= 0 {
                            weight = *(*m).body_mass.add(b_neighbor as usize);
                            if weight < MJ_MINVAL { weight = MJ_MINVAL; }
                        }

                        for row in 0..2i32 {
                            for col in 0..2i32 {
                                B[(2 * row + col) as usize] += weight * dx[row as usize] * dx[col as usize];
                            }
                        }
                    }

                    let metric_ptr = (*m).flex_vertmetric.add(4 * v_global as usize);
                    let det = B[0] * B[3] - B[1] * B[2];

                    if f64::abs(det) < MJ_MINVAL {
                        crate::engine::engine_util_blas::mju_zero(metric_ptr, 4);
                    } else {
                        let invdet = 1.0 / det;
                        *metric_ptr.add(0) = B[3] * invdet;
                        *metric_ptr.add(1) = -B[1] * invdet;
                        *metric_ptr.add(2) = -B[2] * invdet;
                        *metric_ptr.add(3) = B[0] * invdet;
                    }
                }

                current_adj_offset += total_adj_edges;

                // vertex Jacobian sparsity
                let v0_base = 2 * vbase;
                let mut current_adr: i32 = 0;
                if v0_base > 0 {
                    current_adr = *vrowadr.add((v0_base - 1) as usize) + *vrownnz.add((v0_base - 1) as usize);
                }
                *vrowadr.add(v0_base as usize) = current_adr;

                for v in 0..nvert {
                    crate::engine::engine_util_misc::mju_zero_int(buf_ind, nv);
                    let mut current_nnz: i32 = 0;
                    for ii in 0..*v_edge_cnt.add(v as usize) {
                        let edge_e = *adj_edges.add((*v_edge_adr.add(v as usize) + ii) as usize);
                        let ev1 = *(*m).flex_edge.add((2 * (ebase + edge_e)) as usize);
                        let ev2 = *(*m).flex_edge.add((2 * (ebase + edge_e) + 1) as usize);

                        let eb1 = *(*m).flex_vertbodyid.add((vbase + ev1) as usize);
                        let eb2 = *(*m).flex_vertbodyid.add((vbase + ev2) as usize);
                        let NV1 = crate::engine::engine_core_util::mj_body_chain(m as *const mjModel, eb1, chain1);
                        let NV2 = crate::engine::engine_core_util::mj_body_chain(m as *const mjModel, eb2, chain2);

                        for jj in 0..NV1 {
                            if *buf_ind.add(*chain1.add(jj as usize) as usize) == 0 {
                                *buf_ind.add(*chain1.add(jj as usize) as usize) = 1;
                                current_nnz += 1;
                            }
                        }
                        for jj in 0..NV2 {
                            if *buf_ind.add(*chain2.add(jj as usize) as usize) == 0 {
                                *buf_ind.add(*chain2.add(jj as usize) as usize) = 1;
                                current_nnz += 1;
                            }
                        }
                    }
                    let row0 = 2 * (vbase + v);
                    let row1 = 2 * (vbase + v) + 1;
                    *vrownnz.add(row0 as usize) = current_nnz;
                    *vrownnz.add(row1 as usize) = current_nnz;

                    // set rowadr for next rows
                    *vrowadr.add(row1 as usize) = *vrowadr.add(row0 as usize) + current_nnz;
                    if row1 + 1 < 2 * nflexvert {
                        *vrowadr.add((row1 + 1) as usize) = *vrowadr.add(row1 as usize) + current_nnz;
                    }

                    // fill colind
                    let mut count: i32 = 0;
                    for jj in 0..nv {
                        if *buf_ind.add(jj as usize) != 0 {
                            *(*m).flexvert_J_colind.add((*vrowadr.add(row0 as usize) + count) as usize) = jj;
                            *(*m).flexvert_J_colind.add((*vrowadr.add(row1 as usize) + count) as usize) = jj;
                            count += 1;
                        }
                    }
                }
            }
        }

        crate::engine::engine_memory::mj_free_stack(d);
    }
}

/// C: mj_alignFlex (engine/engine_setconst.c:648)
/// Calls: mju_copy3, mju_cross, mju_mulMatTVec3, mju_normalize3, mju_quat2Mat, mju_quatZ2Vec, mju_sub3, mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn mj_align_flex(m: *mut mjModel, d: *mut mjData) {
    todo!() // mj_alignFlex
}

/// C: set0 (engine/engine_setconst.c:695)
/// Calls: makeFlexSparse, makeTendonSparse, mj_alignFlex, mj_camlight, mj_comPos, mj_factorM, mj_flex, mj_freeStack, mj_jacBodyCom, mj_kinematics, mj_local2Global, mj_makeM, mj_markStack, mj_setM0, mj_solveM, mj_stackAllocInfo, mj_tendon, mj_transmission, mju_copy, mju_copy3, mju_copy9, mju_dot, mju_isZero, mju_max, mju_message, mju_mulMatMatT, mju_mulMatTVec3, mju_mulQuat, mju_negQuat, mju_norm, mju_normalize4, mju_sparse2dense, mju_sub3, mju_subFrom3, mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn set0(m: *mut mjModel, d: *mut mjData) {
    todo!() // set0
}

/// C: updateBox (engine/engine_setconst.c:1041)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn update_box(xmin: *mut f64, xmax: *mut f64, pos: *mut f64, radius: f64) {
    // SAFETY: xmin, xmax, pos each point to at least 3 f64 (caller contract)
    unsafe {
        for i in 0..3_usize {
            let lo = *pos.add(i) - radius;
            let hi = *pos.add(i) + radius;
            if lo < *xmin.add(i) {
                *xmin.add(i) = lo;
            }
            if hi > *xmax.add(i) {
                *xmax.add(i) = hi;
            }
        }
    }
}

/// C: setStat (engine/engine_setconst.c:1050)
/// Calls: mj_freeStack, mj_markStack, mj_stackAllocInfo, mju_add3, mju_dist3, mju_max, mju_scl3, mju_zero, updateBox
#[allow(unused_variables, non_snake_case)]
pub fn set_stat(m: *mut mjModel, d: *mut mjData) {
    todo!() // setStat
}

/// C: setSpring (engine/engine_setconst.c:1198)
/// Calls: mj_comPos, mj_kinematics, mj_tendon, mj_transmission, mju_copy
#[allow(unused_variables, non_snake_case)]
pub fn set_spring(m: *mut mjModel, d: *mut mjData) {
    todo!() // setSpring
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
    const MJMINVAL: f64 = 1e-15;

    // SAFETY: m, d, opt are valid pointers (caller contract from pipeline)
    unsafe {
        let nv = (*m).nv as i32;

        // reduce velocity
        let timeconst = if 0.01_f64 > (*opt).timeconst { 0.01_f64 } else { (*opt).timeconst };
        let scl = (-(*m).opt.timestep / timeconst).exp();
        crate::engine::engine_util_blas::mju_scl((*d).qvel, (*d).qvel, scl, nv);

        // step1: compute inertia and actuator moments
        crate::engine::engine_forward::mj_step1(m, d);

        // dense actuator_moment row
        crate::engine::engine_memory::mj_mark_stack(d);
        let moment = crate::engine::engine_memory::mj_stack_alloc_num(d, nv as usize);
        crate::engine::engine_util_sparse::mju_sparse2dense(
            moment, (*d).actuator_moment, 1, nv,
            (*d).moment_rownnz.add(index as usize),
            (*d).moment_rowadr.add(index as usize),
            (*d).moment_colind,
        );

        // set force to generate desired acceleration
        crate::engine::engine_core_smooth::mj_solve_m(m, d, (*d).qfrc_applied, moment, 1);
        let nrm = crate::engine::engine_util_blas::mju_norm((*d).qfrc_applied, nv);
        let denom = if MJMINVAL > nrm { MJMINVAL } else { nrm };
        crate::engine::engine_util_blas::mju_scl(
            (*d).qfrc_applied, moment,
            (2 * side - 1) as f64 * (*opt).accel / denom, nv,
        );

        // impose maxforce
        let nrm = crate::engine::engine_util_blas::mju_norm((*d).qfrc_applied, nv);
        if (*opt).maxforce > 0.0 && nrm > (*opt).maxforce {
            let denom = if MJMINVAL > nrm { MJMINVAL } else { nrm };
            crate::engine::engine_util_blas::mju_scl(
                (*d).qfrc_applied, (*d).qfrc_applied,
                (*opt).maxforce / denom, nv,
            );
        }

        // step2: apply force
        crate::engine::engine_forward::mj_step2(m, d);

        crate::engine::engine_memory::mj_free_stack(d);

        // return actuator length
        *(*d).actuator_length.add(index as usize)
    }
}

/// C: mj_setConst (engine/engine_setconst.h:27)
/// Calls: set0, setFixed, setSpring, setStat
#[allow(unused_variables, non_snake_case)]
pub fn mj_set_const(m: *mut mjModel, d: *mut mjData) {
    todo!() // mj_setConst
}

/// C: mj_setLengthRange (engine/engine_setconst.h:30)
/// Calls: evalAct, mj_resetData, mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_set_length_range(m: *mut mjModel, d: *mut mjData, index: i32, opt: *const mjLROpt, error: *mut i8, error_sz: i32) -> i32 {
    todo!() // mj_setLengthRange
}

