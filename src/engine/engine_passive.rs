//! Port of: engine/engine_passive.c
//! IR hash: 3fb6da908ad9d71c
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: GradSquaredLengths (engine/engine_passive.c:48)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn grad_squared_lengths(gradient: *mut [[f64; 3]; 2], xpos: *const f64, vert: *const i32, edge: *const [i32; 2], nedge: i32) {
    // SAFETY: caller guarantees all pointers are valid and arrays are properly sized
    unsafe {
        for e in 0..nedge as usize {
            for d in 0..3usize {
                let v0 = *vert.add((*edge.add(e))[0] as usize);
                let v1 = *vert.add((*edge.add(e))[1] as usize);
                (*gradient.add(e))[0][d] = *xpos.add(3 * v0 as usize + d) - *xpos.add(3 * v1 as usize + d);
                (*gradient.add(e))[1][d] = *xpos.add(3 * v1 as usize + d) - *xpos.add(3 * v0 as usize + d);
            }
        }
    }
}

/// C: mj_flexPassiveInterp (engine/engine_passive.c:63)
/// Calls: mj_applyFT, mj_freeStack, mj_markStack, mj_stackAllocInfo, mji_addScl3, mji_addTo3, mji_rotVecQuat, mju_flexGatherCellState, mju_flexGatherFaceState, mju_flexGatherState, mju_mulMatVec, mju_negQuat, mju_rotVecQuat, mju_scl3, mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mj_flex_passive_interp(m: *const mjModel, d: *mut mjData, f: i32, enbl_spring: i32, enbl_damper: i32) {
    use crate::engine::engine_memory::{mj_mark_stack, mj_free_stack, mj_stack_alloc_num, mj_stack_alloc_int};
    use crate::engine::engine_util_blas::{mju_zero, mju_mul_mat_vec, mju_scl3};
    use crate::engine::engine_util_misc::{mju_flex_gather_cell_state, mju_flex_gather_face_state};
    use crate::engine::engine_util_spatial::{mju_neg_quat, mju_rot_vec_quat};
    use crate::engine::engine_inline::{mji_add_scl3, mji_add_to3, mji_rot_vec_quat};
    use crate::engine::engine_core_util::mju_flex_gather_state;
    use crate::engine::engine_support::mj_apply_ft;

    // SAFETY: caller guarantees m and d are valid
    unsafe {
        let stiffnessadr: i32 = *(*m).flex_stiffnessadr.add(f as usize);
        if stiffnessadr < 0 {
            return;
        }

        let k: *const f64 = (*m).flex_stiffness.add(stiffnessadr as usize);
        let nodenum: i32 = *(*m).flex_nodenum.add(f as usize);

        let order_raw: i32 = *(*m).flex_interp.add(f as usize);
        let shell_mode: bool = order_raw < 0;
        let order: i32 = if order_raw < 0 { -order_raw } else { order_raw };
        let cx: i32 = *(*m).flex_cellnum.add(3 * f as usize);
        let cy: i32 = *(*m).flex_cellnum.add(3 * f as usize + 1);
        let cz: i32 = *(*m).flex_cellnum.add(3 * f as usize + 2);

        // determine element type
        let npe: i32;
        let nelem_fe: i32;

        if shell_mode {
            npe = (order + 1) * (order + 1);
            nelem_fe = 2 * (cy * cz + cx * cz + cx * cy);
        } else {
            npe = (order + 1) * (order + 1) * (order + 1);
            nelem_fe = cx * cy * cz;
        }

        // check if we have any work to do
        let has_stretch: bool = *k != 0.0 && *(*m).flex_edgeequality.add(f as usize) != 3;
        if !has_stretch {
            return;
        }

        mj_mark_stack(d);

        // allocate global arrays
        let xpos_g: *mut f64 = mj_stack_alloc_num(d, (3 * nodenum) as usize);
        let vel_g: *mut f64 = mj_stack_alloc_num(d, (3 * nodenum) as usize);
        let frc_g: *mut f64 = mj_stack_alloc_num(d, (3 * nodenum) as usize);
        let dmp_g: *mut f64 = mj_stack_alloc_num(d, (3 * nodenum) as usize);
        let xpos0: *const f64 = (*m).flex_node0.add(3 * *(*m).flex_nodeadr.add(f as usize) as usize);
        let bodyid: *const i32 = (*m).flex_nodebodyid.add(*(*m).flex_nodeadr.add(f as usize) as usize);

        // gather global node positions and velocities
        mju_flex_gather_state(m, d as *const _, f, xpos_g, vel_g);

        // zero global force accumulators
        mju_zero(frc_g, 3 * nodenum);
        mju_zero(dmp_g, 3 * nodenum);

        // per-element arrays
        let xpos_e: *mut f64 = mj_stack_alloc_num(d, (3 * npe) as usize);
        let vel_e: *mut f64 = mj_stack_alloc_num(d, (3 * npe) as usize);
        let xpos0_e: *mut f64 = mj_stack_alloc_num(d, (3 * npe) as usize);
        let displ_e: *mut f64 = mj_stack_alloc_num(d, (3 * npe) as usize);
        let frc_e: *mut f64 = mj_stack_alloc_num(d, (3 * npe) as usize);
        let dmp_e: *mut f64 = mj_stack_alloc_num(d, (3 * npe) as usize);
        let gindices: *mut i32 = mj_stack_alloc_int(d, npe as usize);

        // stretch forces
        if has_stretch {
            for fe in 0..nelem_fe {
                // get element stiffness matrix
                let k_elem: *const f64 = k.add((fe as usize) * (3 * npe as usize) * (3 * npe as usize));

                // skip empty elements (zero stiffness)
                if *k_elem == 0.0 {
                    continue;
                }

                // gather element-local node data and compute corotational rotation
                let mut quat: [f64; 4] = [0.0; 4];
                if shell_mode {
                    mju_flex_gather_face_state(
                        order, cx, cy, cz, fe, xpos_g, vel_g, xpos0 as *const _,
                        xpos_e, vel_e, xpos0_e, gindices, quat.as_mut_ptr(),
                    );
                } else {
                    let ci: i32 = fe / (cy * cz);
                    let cj: i32 = (fe / cz) % cy;
                    let ck: i32 = fe % cz;
                    mju_flex_gather_cell_state(
                        order, cy, cz, ci, cj, ck, xpos_g, vel_g, xpos0 as *const _,
                        xpos_e, vel_e, xpos0_e, gindices, quat.as_mut_ptr(),
                    );
                }

                // rotate to corotational frame
                for n in 0..npe {
                    mju_rot_vec_quat(xpos_e.add(3 * n as usize), xpos_e.add(3 * n as usize), quat.as_ptr());
                    mju_rot_vec_quat(vel_e.add(3 * n as usize), vel_e.add(3 * n as usize), quat.as_ptr());
                }

                // compute displacement
                for n in 0..npe {
                    mji_add_scl3(displ_e.add(3 * n as usize), xpos_e.add(3 * n as usize), xpos0_e.add(3 * n as usize), -1.0);
                }

                // compute force in corotational frame
                if enbl_spring != 0 {
                    mju_mul_mat_vec(frc_e, k_elem, displ_e, 3 * npe, 3 * npe);
                }
                if enbl_damper != 0 {
                    mju_mul_mat_vec(dmp_e, k_elem, vel_e, 3 * npe, 3 * npe);
                }

                // rotate back to global frame and scatter using node indices
                mju_neg_quat(quat.as_mut_ptr(), quat.as_ptr());
                for n in 0..npe {
                    let mut qfrc: [f64; 3] = [0.0; 3];
                    let mut qdmp: [f64; 3] = [0.0; 3];
                    mji_rot_vec_quat(qfrc.as_mut_ptr(), frc_e.add(3 * n as usize), quat.as_ptr());
                    mji_rot_vec_quat(qdmp.as_mut_ptr(), dmp_e.add(3 * n as usize), quat.as_ptr());
                    let gidx: i32 = *gindices.add(n as usize);
                    if enbl_spring != 0 {
                        mji_add_to3(frc_g.add(3 * gidx as usize), qfrc.as_ptr());
                    }
                    if enbl_damper != 0 {
                        mji_add_to3(dmp_g.add(3 * gidx as usize), qdmp.as_ptr());
                    }
                }
            }
        }

        // apply accumulated forces to bodies
        for i in 0..nodenum {
            mju_scl3(dmp_g.add(3 * i as usize), dmp_g.add(3 * i as usize) as *const _, *(*m).flex_damping.add(f as usize));
            let bid: i32 = *bodyid.add(i as usize);
            let nidx: i32 = i + *(*m).flex_nodeadr.add(f as usize);

            // fast path: node at body origin, direct DOF write
            if *(*m).body_dofnum.add(bid as usize) > 0
                && (*(*m).flex_centered.add(f as usize)
                    || (*(*m).flex_node.add(3 * nidx as usize) == 0.0
                        && *(*m).flex_node.add(3 * nidx as usize + 1) == 0.0
                        && *(*m).flex_node.add(3 * nidx as usize + 2) == 0.0))
            {
                if enbl_spring != 0 {
                    mji_add_to3((*d).qfrc_spring.add(*(*m).body_dofadr.add(bid as usize) as usize), frc_g.add(3 * i as usize));
                }
                if enbl_damper != 0 {
                    mji_add_to3((*d).qfrc_damper.add(*(*m).body_dofadr.add(bid as usize) as usize), dmp_g.add(3 * i as usize));
                }
            } else {
                if enbl_spring != 0 {
                    mj_apply_ft(m, d, frc_g.add(3 * i as usize), std::ptr::null(), xpos_g.add(3 * i as usize), bid, (*d).qfrc_spring);
                }
                if enbl_damper != 0 {
                    mj_apply_ft(m, d, dmp_g.add(3 * i as usize), std::ptr::null(), xpos_g.add(3 * i as usize), bid, (*d).qfrc_damper);
                }
            }
        }

        mj_free_stack(d);
    }
}

/// C: mju_dphi2D (engine/engine_passive.c:211)
/// Calls: mju_flexDphi, mju_flexPhi
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_dphi2d(s0: f64, l0: i32, s1: f64, l1: i32, order: i32, dir: i32) -> f64 {
    use crate::engine::engine_util_misc::{mju_flex_dphi, mju_flex_phi};
    if dir == 0 {
        mju_flex_dphi(s0, l0, order) * mju_flex_phi(s1, l1, order)
    } else {
        mju_flex_phi(s0, l0, order) * mju_flex_dphi(s1, l1, order)
    }
}

/// C: mj_flexPassiveBendInterp (engine/engine_passive.c:236)
/// Calls: mj_applyFT, mj_freeStack, mj_markStack, mj_stackAllocInfo, mji_addTo3, mji_cross, mji_sub3, mju_add, mju_copy, mju_copyInt, mju_dot, mju_dot3, mju_dphi2D, mju_flexFaceNormal2D, mju_flexGatherFaceState, mju_flexGatherState, mju_message, mju_negQuat, mju_norm3, mju_normalize, mju_rotVecQuat, mju_scl, mju_warning, mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mj_flex_passive_bend_interp(m: *const mjModel, d: *mut mjData, f: i32, enbl_spring: i32, enbl_damper: i32) {
    use crate::engine::engine_memory::{mj_mark_stack, mj_free_stack, mj_stack_alloc_num, mj_stack_alloc_int};
    use crate::engine::engine_util_blas::{mju_zero, mju_copy, mju_normalize, mju_dot, mju_dot3, mju_norm3, mju_scl};
    use crate::engine::engine_util_misc::{mju_copy_int, mju_flex_gather_face_state, mju_flex_face_normal2d};
    use crate::engine::engine_util_spatial::{mju_neg_quat, mju_rot_vec_quat};
    use crate::engine::engine_inline::{mji_sub3, mji_cross, mji_add_to3};
    use crate::engine::engine_core_util::mju_flex_gather_state;
    use crate::engine::engine_support::mj_apply_ft;
    use crate::engine::engine_util_blas::mju_add;
    const MJ_MINVAL: f64 = 1E-15;
    const BEND_EDGE_SIZE: usize = 10;

    // SAFETY: caller guarantees m and d are valid
    unsafe {
        if *(*m).flex_interp.add(f as usize) >= 0 {
            return;
        }

        let bendingadr: i32 = *(*m).flex_bendingadr.add(f as usize);
        if bendingadr < 0 {
            return;
        }

        // read bending edge data
        let bdata: *const f64 = (*m).flex_bending.add(bendingadr as usize);
        let nedge: i32 = *bdata as i32;
        if nedge == 0 { return; }

        let order: i32 = -*(*m).flex_interp.add(f as usize);
        let cx: i32 = *(*m).flex_cellnum.add(3 * f as usize);
        let cy: i32 = *(*m).flex_cellnum.add(3 * f as usize + 1);
        let cz: i32 = *(*m).flex_cellnum.add(3 * f as usize + 2);
        let npe: i32 = (order + 1) * (order + 1);
        let nodenum: i32 = *(*m).flex_nodenum.add(f as usize);

        mj_mark_stack(d);

        // gather global state
        let xpos_g: *mut f64 = mj_stack_alloc_num(d, (3 * nodenum) as usize);
        let vel_g: *mut f64 = mj_stack_alloc_num(d, (3 * nodenum) as usize);
        let frc_g: *mut f64 = mj_stack_alloc_num(d, (3 * nodenum) as usize);
        let dmp_g: *mut f64 = mj_stack_alloc_num(d, (3 * nodenum) as usize);
        mju_flex_gather_state(m, d as *const _, f, xpos_g, vel_g);
        mju_zero(frc_g, 3 * nodenum);
        mju_zero(dmp_g, 3 * nodenum);

        // precompute per-face cache
        let nfaces: i32 = 2 * (cy * cz + cx * cz + cx * cy);
        let face_xpos: *mut f64 = mj_stack_alloc_num(d, (nfaces * 3 * npe) as usize);
        let face_gidx: *mut i32 = mj_stack_alloc_int(d, (nfaces * npe) as usize);
        let face_quat: *mut f64 = mj_stack_alloc_num(d, (nfaces * 4) as usize);
        for fi in 0..nfaces {
            mju_flex_gather_face_state(
                order, cx, cy, cz, fi, xpos_g, std::ptr::null(), std::ptr::null(),
                face_xpos.add((fi * 3 * npe) as usize),
                std::ptr::null_mut(), std::ptr::null_mut(),
                face_gidx.add((fi * npe) as usize),
                face_quat.add((fi * 4) as usize),
            );
        }

        // per-edge temporaries
        let xpos_a: *mut f64 = mj_stack_alloc_num(d, (3 * npe) as usize);
        let xpos_b: *mut f64 = mj_stack_alloc_num(d, (3 * npe) as usize);
        let gidx_a: *mut i32 = mj_stack_alloc_int(d, npe as usize);
        let gidx_b: *mut i32 = mj_stack_alloc_int(d, npe as usize);

        for e in 0..nedge {
            let edata: *const f64 = bdata.add(1 + e as usize * BEND_EDGE_SIZE);
            let fe_a: i32 = *edata as i32;
            let fe_b: i32 = *edata.add(1) as i32;
            let local_a: [f64; 2] = [*edata.add(2), *edata.add(3)];
            let local_b: [f64; 2] = [*edata.add(4), *edata.add(5)];
            let stiffness: f64 = *edata.add(6);
            let dn0: [f64; 3] = [*edata.add(7), *edata.add(8), *edata.add(9)];

            if stiffness == 0.0 { continue; }

            // look up cached face data
            mju_copy(xpos_a, face_xpos.add((fe_a * 3 * npe) as usize), 3 * npe);
            mju_copy(xpos_b, face_xpos.add((fe_b * 3 * npe) as usize), 3 * npe);
            mju_copy_int(gidx_a, face_gidx.add((fe_a * npe) as usize), npe);
            mju_copy_int(gidx_b, face_gidx.add((fe_b * npe) as usize), npe);
            let mut quat_a: [f64; 4] = [0.0; 4];
            let mut quat_b: [f64; 4] = [0.0; 4];
            mju_copy(quat_a.as_mut_ptr(), face_quat.add((fe_a * 4) as usize), 4);
            mju_copy(quat_b.as_mut_ptr(), face_quat.add((fe_b * 4) as usize), 4);

            // compute deformed normals at edge midpoint
            let mut n_a: [f64; 3] = [0.0; 3];
            let mut t1_a: [f64; 3] = [0.0; 3];
            let mut t2_a: [f64; 3] = [0.0; 3];
            let mut n_b: [f64; 3] = [0.0; 3];
            let mut t1_b: [f64; 3] = [0.0; 3];
            let mut t2_b: [f64; 3] = [0.0; 3];
            mju_flex_face_normal2d(n_a.as_mut_ptr(), t1_a.as_mut_ptr(), t2_a.as_mut_ptr(), order, xpos_a, local_a.as_ptr());
            mju_flex_face_normal2d(n_b.as_mut_ptr(), t1_b.as_mut_ptr(), t2_b.as_mut_ptr(), order, xpos_b, local_b.as_ptr());

            // normalize normals
            let len_a: f64 = mju_norm3(n_a.as_ptr());
            let len_b: f64 = mju_norm3(n_b.as_ptr());
            if len_a < MJ_MINVAL || len_b < MJ_MINVAL { continue; }
            let inv_a: f64 = 1.0 / len_a;
            let inv_b: f64 = 1.0 / len_b;
            n_a[0] *= inv_a; n_a[1] *= inv_a; n_a[2] *= inv_a;
            n_b[0] *= inv_b; n_b[1] *= inv_b; n_b[2] *= inv_b;

            // average corotational frame
            if mju_dot(quat_a.as_ptr(), quat_b.as_ptr(), 4) < 0.0 {
                mju_scl(quat_b.as_mut_ptr(), quat_b.as_ptr(), -1.0, 4);
            }
            let mut quat_avg: [f64; 4] = [0.0; 4];
            mju_add(quat_avg.as_mut_ptr(), quat_a.as_ptr(), quat_b.as_ptr(), 4);
            mju_normalize(quat_avg.as_mut_ptr(), 4);
            mju_neg_quat(quat_avg.as_mut_ptr(), quat_avg.as_ptr());

            // rotate dn0 from rest frame to current frame
            let mut dn0_rot: [f64; 3] = [0.0; 3];
            mju_rot_vec_quat(dn0_rot.as_mut_ptr(), dn0.as_ptr(), quat_avg.as_ptr());

            // normal jump residual: r = (n_A - n_B) - R_avg * dn0
            let mut r: [f64; 3] = [0.0; 3];
            mji_sub3(r.as_mut_ptr(), n_a.as_ptr(), n_b.as_ptr());
            r[0] -= dn0_rot[0]; r[1] -= dn0_rot[1]; r[2] -= dn0_rot[2];

            // --- spring force ---
            if enbl_spring != 0 {
                // w_A = P_A * r = (r - n_A*(n_A.r)) / |c_A|
                let dot_a: f64 = mju_dot3(n_a.as_ptr(), r.as_ptr());
                let w_a: [f64; 3] = [
                    (r[0] - n_a[0] * dot_a) * inv_a,
                    (r[1] - n_a[1] * dot_a) * inv_a,
                    (r[2] - n_a[2] * dot_a) * inv_a,
                ];

                let dot_b: f64 = mju_dot3(n_b.as_ptr(), r.as_ptr());
                let w_b: [f64; 3] = [
                    (r[0] - n_b[0] * dot_b) * inv_b,
                    (r[1] - n_b[1] * dot_b) * inv_b,
                    (r[2] - n_b[2] * dot_b) * inv_b,
                ];

                // precompute cross products
                let mut w_at2: [f64; 3] = [0.0; 3];
                let mut w_at1: [f64; 3] = [0.0; 3];
                let mut w_bt2: [f64; 3] = [0.0; 3];
                let mut w_bt1: [f64; 3] = [0.0; 3];
                mji_cross(w_at2.as_mut_ptr(), w_a.as_ptr(), t2_a.as_ptr());
                mji_cross(w_at1.as_mut_ptr(), w_a.as_ptr(), t1_a.as_ptr());
                mji_cross(w_bt2.as_mut_ptr(), w_b.as_ptr(), t2_b.as_ptr());
                mji_cross(w_bt1.as_mut_ptr(), w_b.as_ptr(), t1_b.as_ptr());

                // force on face A nodes
                let mut idx: i32 = 0;
                for l0 in 0..=order {
                    for l1 in 0..=order {
                        let g0: f64 = mju_dphi2d(local_a[0], l0, local_a[1], l1, order, 0);
                        let g1: f64 = mju_dphi2d(local_a[0], l0, local_a[1], l1, order, 1);
                        let gi: i32 = *gidx_a.add(idx as usize);
                        for j in 0..3usize {
                            *frc_g.add(3 * gi as usize + j) += stiffness * (g0 * w_at2[j] - g1 * w_at1[j]);
                        }
                        idx += 1;
                    }
                }

                // force on face B nodes (negative sign)
                idx = 0;
                for l0 in 0..=order {
                    for l1 in 0..=order {
                        let g0: f64 = mju_dphi2d(local_b[0], l0, local_b[1], l1, order, 0);
                        let g1: f64 = mju_dphi2d(local_b[0], l0, local_b[1], l1, order, 1);
                        let gi: i32 = *gidx_b.add(idx as usize);
                        for j in 0..3usize {
                            *frc_g.add(3 * gi as usize + j) -= stiffness * (g0 * w_bt2[j] - g1 * w_bt1[j]);
                        }
                        idx += 1;
                    }
                }
            }
        }

        // apply accumulated forces to bodies
        let bodyid: *const i32 = (*m).flex_nodebodyid.add(*(*m).flex_nodeadr.add(f as usize) as usize);
        for i in 0..nodenum {
            let bid: i32 = *bodyid.add(i as usize);
            let nidx: i32 = i + *(*m).flex_nodeadr.add(f as usize);

            // fast path: node at body origin, direct DOF write
            if *(*m).body_dofnum.add(bid as usize) > 0
                && (*(*m).flex_centered.add(f as usize)
                    || (*(*m).flex_node.add(3 * nidx as usize) == 0.0
                        && *(*m).flex_node.add(3 * nidx as usize + 1) == 0.0
                        && *(*m).flex_node.add(3 * nidx as usize + 2) == 0.0))
            {
                if enbl_spring != 0 {
                    mji_add_to3((*d).qfrc_spring.add(*(*m).body_dofadr.add(bid as usize) as usize), frc_g.add(3 * i as usize));
                }
                if enbl_damper != 0 {
                    mji_add_to3((*d).qfrc_damper.add(*(*m).body_dofadr.add(bid as usize) as usize), dmp_g.add(3 * i as usize));
                }
            } else {
                if enbl_spring != 0 {
                    mj_apply_ft(m, d, frc_g.add(3 * i as usize), std::ptr::null(), xpos_g.add(3 * i as usize), bid, (*d).qfrc_spring);
                }
                if enbl_damper != 0 {
                    mj_apply_ft(m, d, dmp_g.add(3 * i as usize), std::ptr::null(), xpos_g.add(3 * i as usize), bid, (*d).qfrc_damper);
                }
            }
        }

        mj_free_stack(d);
    }
}

/// C: mj_flexPassiveBend (engine/engine_passive.c:444)
/// Calls: mji_cross, mji_sub3
#[allow(unused_variables, non_snake_case)]
pub fn mj_flex_passive_bend(m: *const mjModel, d: *mut mjData, f: i32, enbl_spring: i32, enbl_damper: i32) {
    use crate::engine::engine_inline::{mji_sub3, mji_cross};

    // SAFETY: caller guarantees m and d are valid
    unsafe {
        if *(*m).flex_dim.add(f as usize) != 2 {
            return;
        }

        let bendingadr: i32 = *(*m).flex_bendingadr.add(f as usize);
        if bendingadr < 0 {
            return;
        }

        let edgenum: i32 = *(*m).flex_edgenum.add(f as usize);
        let xpos: *mut f64 = (*d).flexvert_xpos.add(3 * *(*m).flex_vertadr.add(f as usize) as usize);
        let bodyid: *const i32 = (*m).flex_vertbodyid.add(*(*m).flex_vertadr.add(f as usize) as usize);
        let b: *const f64 = (*m).flex_bending.add(bendingadr as usize);

        for e in 0..edgenum {
            let edge: *const i32 = (*m).flex_edge.add(2 * (e + *(*m).flex_edgeadr.add(f as usize)) as usize);
            let flap: *const i32 = (*m).flex_edgeflap.add(2 * (e + *(*m).flex_edgeadr.add(f as usize)) as usize);
            let v: [i32; 4] = [*edge, *edge.add(1), *flap, *flap.add(1)];
            if v[3] == -1 {
                // skip boundary edges
                continue;
            }

            // flap edges
            let mut ed: [[f64; 3]; 3] = [[0.0; 3]; 3];
            mji_sub3(ed[0].as_mut_ptr(), xpos.add(3 * v[1] as usize), xpos.add(3 * v[0] as usize));
            mji_sub3(ed[1].as_mut_ptr(), xpos.add(3 * v[2] as usize), xpos.add(3 * v[0] as usize));
            mji_sub3(ed[2].as_mut_ptr(), xpos.add(3 * v[3] as usize), xpos.add(3 * v[0] as usize));

            // forces at the vertices due to curved reference
            let mut frc: [[f64; 3]; 4] = [[0.0; 3]; 4];
            mji_cross(frc[1].as_mut_ptr(), ed[1].as_ptr(), ed[2].as_ptr());
            mji_cross(frc[2].as_mut_ptr(), ed[2].as_ptr(), ed[0].as_ptr());
            mji_cross(frc[3].as_mut_ptr(), ed[0].as_ptr(), ed[1].as_ptr());
            frc[0][0] = -(frc[1][0] + frc[2][0] + frc[3][0]);
            frc[0][1] = -(frc[1][1] + frc[2][1] + frc[3][1]);
            frc[0][2] = -(frc[1][2] + frc[2][2] + frc[3][2]);

            // velocities
            let mut vel_ptrs: [*mut f64; 4] = [std::ptr::null_mut(); 4];
            for i in 0..4 {
                vel_ptrs[i] = (*d).qvel.add(*(*m).body_dofadr.add(*bodyid.add(v[i] as usize) as usize) as usize);
            }

            // force
            let mut spring: [f64; 12] = [0.0; 12];
            let mut damper: [f64; 12] = [0.0; 12];
            for i in 0..4usize {
                for x in 0..3usize {
                    for j in 0..4usize {
                        // thin plate bending force
                        if enbl_spring != 0 {
                            spring[3 * i + x] += *b.add(17 * e as usize + 4 * i + j) * *xpos.add(3 * v[j] as usize + x);
                        }

                        // thin plate damping force
                        if enbl_damper != 0 {
                            damper[3 * i + x] += *b.add(17 * e as usize + 4 * i + j) * *vel_ptrs[j].add(x);
                        }
                    }

                    // curved reference contribution
                    if enbl_spring != 0 {
                        spring[3 * i + x] += *b.add(17 * e as usize + 16) * frc[i][x];
                    }
                }
            }

            // insert into global force
            for i in 0..4usize {
                let bid: i32 = *bodyid.add(v[i] as usize);
                let body_dofnum: i32 = *(*m).body_dofnum.add(bid as usize);
                let body_dofadr: i32 = *(*m).body_dofadr.add(bid as usize);
                for x in 0..body_dofnum as usize {
                    if enbl_spring != 0 {
                        *(*d).qfrc_spring.add((body_dofadr as usize) + x) -= spring[3 * i + x];
                    }
                    if enbl_damper != 0 {
                        *(*d).qfrc_damper.add((body_dofadr as usize) + x) -= damper[3 * i + x] * *(*m).flex_damping.add(f as usize);
                    }
                }
            }
        }
    }
}

/// C: mj_flexPassiveStretch (engine/engine_passive.c:524)
/// Calls: GradSquaredLengths, mj_applyFT, mj_freeStack, mj_markStack, mj_stackAllocInfo, mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mj_flex_passive_stretch(m: *const mjModel, d: *mut mjData, f: i32, enbl_spring: i32, enbl_damper: i32) {
    use crate::engine::engine_memory::{mj_mark_stack, mj_free_stack, mj_stack_alloc_num};
    use crate::engine::engine_util_blas::mju_zero;
    use crate::engine::engine_support::mj_apply_ft;

    // edges[dim-2]: triangle edges = edges[0], tet edges = edges[1]
    const EDGES: [[[i32; 2]; 6]; 2] = [
        [[1, 2], [2, 0], [0, 1], [0, 0], [0, 0], [0, 0]],
        [[0, 1], [1, 2], [2, 0], [2, 3], [0, 3], [1, 3]],
    ];

    // SAFETY: caller guarantees m and d are valid
    unsafe {
        let stiffnessadr: i32 = *(*m).flex_stiffnessadr.add(f as usize);
        if stiffnessadr < 0 {
            return;
        }
        let k: *const f64 = (*m).flex_stiffness.add(stiffnessadr as usize);
        if *k == 0.0 {
            return;
        }

        let dim: i32 = *(*m).flex_dim.add(f as usize);
        let nedge: i32 = if dim == 2 { 3 } else { 6 };
        let nvert: i32 = if dim == 2 { 3 } else { 4 };
        let elem: *const i32 = (*m).flex_elem.add(*(*m).flex_elemdataadr.add(f as usize) as usize);
        let edgeelem: *const i32 = (*m).flex_elemedge.add(*(*m).flex_elemedgeadr.add(f as usize) as usize);
        let xpos: *mut f64 = (*d).flexvert_xpos.add(3 * *(*m).flex_vertadr.add(f as usize) as usize);
        let vel: *mut f64 = (*d).flexedge_velocity.add(*(*m).flex_edgeadr.add(f as usize) as usize);
        let deformed: *mut f64 = (*d).flexedge_length.add(*(*m).flex_edgeadr.add(f as usize) as usize);
        let reference: *const f64 = (*m).flexedge_length0.add(*(*m).flex_edgeadr.add(f as usize) as usize);
        let bodyid: *const i32 = (*m).flex_vertbodyid.add(*(*m).flex_vertadr.add(f as usize) as usize);
        let k_d: f64 = if (*m).opt.timestep > 0.0 { *(*m).flex_damping.add(f as usize) / (*m).opt.timestep } else { 0.0 };

        mj_mark_stack(d);
        let vertnum: i32 = *(*m).flex_vertnum.add(f as usize);
        let qfrc: *mut f64 = mj_stack_alloc_num(d, (3 * vertnum) as usize);
        mju_zero(qfrc, 3 * vertnum);

        // compute force element-by-element
        let elemnum: i32 = *(*m).flex_elemnum.add(f as usize);
        for t in 0..elemnum {
            let vert: *const i32 = elem.add((dim + 1) as usize * t as usize);

            // compute length gradient with respect to dofs
            let mut gradient: [[[f64; 3]; 2]; 6] = [[[0.0; 3]; 2]; 6];
            grad_squared_lengths(
                gradient.as_mut_ptr() as *mut [[f64; 3]; 2],
                xpos as *const f64,
                vert,
                EDGES[(dim - 2) as usize].as_ptr() as *const [i32; 2],
                nedge,
            );

            // extract elongation of edges belonging to this element
            let mut elongation: [f64; 6] = [0.0; 6];
            for e in 0..nedge as usize {
                let idx: i32 = *edgeelem.add(t as usize * nedge as usize + e);
                let previous: f64 = *deformed.add(idx as usize) - *vel.add(idx as usize) * (*m).opt.timestep;
                elongation[e] = *deformed.add(idx as usize) * *deformed.add(idx as usize)
                    - *reference.add(idx as usize) * *reference.add(idx as usize)
                    + (*deformed.add(idx as usize) * *deformed.add(idx as usize) - previous * previous) * k_d;
            }

            // unpack triangular representation
            let mut metric: [f64; 36] = [0.0; 36];
            let mut id: usize = 0;
            for ed1 in 0..nedge as usize {
                for ed2 in ed1..nedge as usize {
                    metric[nedge as usize * ed1 + ed2] = *k.add(21 * t as usize + id);
                    metric[nedge as usize * ed2 + ed1] = *k.add(21 * t as usize + id);
                    id += 1;
                }
            }

            // compute local force
            let mut force: [f64; 12] = [0.0; 12];
            for ed1 in 0..nedge as usize {
                for ed2 in 0..nedge as usize {
                    for i in 0..2usize {
                        for x in 0..3usize {
                            force[3 * EDGES[(dim - 2) as usize][ed2][i] as usize + x] -=
                                elongation[ed1] * gradient[ed2][i][x]
                                * metric[nedge as usize * ed1 + ed2];
                        }
                    }
                }
            }

            // insert into global force
            for i in 0..nvert as usize {
                for x in 0..3usize {
                    *qfrc.add(3 * *vert.add(i) as usize + x) += force[3 * i + x];
                }
            }
        }

        // insert force into qfrc_passive
        for v in 0..vertnum {
            let bid: i32 = *bodyid.add(v as usize);
            if *(*m).body_simple.add(bid as usize) != 2 {
                // this should only occur for pinned flex vertices
                mj_apply_ft(m, d, qfrc.add(3 * v as usize), std::ptr::null(), xpos.add(3 * v as usize), bid, (*d).qfrc_spring);
            } else {
                let body_dofnum: i32 = *(*m).body_dofnum.add(bid as usize);
                let body_dofadr: i32 = *(*m).body_dofadr.add(bid as usize);
                for x in 0..body_dofnum {
                    *(*d).qfrc_spring.add((body_dofadr + x) as usize) += *qfrc.add(3 * v as usize + x as usize);
                }
            }
        }

        mj_free_stack(d);
    }
}

/// C: mj_springdamper (engine/engine_passive.c:626)
/// Calls: mj_actuatorDamping, mj_flexPassiveBend, mj_flexPassiveBendInterp, mj_flexPassiveInterp, mj_flexPassiveStretch, mj_sleepState, mji_addToScl3, mji_copy4, mji_sub3, mji_subQuat, mju_copy, mju_isZero, mju_norm3, mju_normalize4, mju_polyForce
#[allow(unused_variables, non_snake_case)]
pub fn mj_springdamper(m: *const mjModel, d: *mut mjData) {
    use crate::engine::engine_inline::{mji_sub3, mji_copy4, mji_add_to_scl3, mji_sub_quat};
    use crate::engine::engine_util_blas::{mju_normalize4, mju_norm3, mju_copy};
    use crate::engine::engine_util_misc::{mju_is_zero, mju_poly_force};
    use crate::engine::engine_core_util::mj_actuator_damping;
    use crate::engine::engine_sleep::mj_sleep_state;
    use crate::types::{
        mjtJoint_mjJNT_FREE, mjtJoint_mjJNT_BALL, mjtJoint_mjJNT_SLIDE, mjtJoint_mjJNT_HINGE,
        mjtObj_mjOBJ_TENDON, mjtSleepState_mjS_AWAKE,
    };
    const MJ_DSBL_SPRING: i32 = 1 << 8;
    const MJ_DSBL_DAMPER: i32 = 1 << 9;
    const MJ_ENBL_SLEEP: i32 = 1 << 4;
    const MJ_NPOLY: usize = 2;

    // SAFETY: caller guarantees m and d are valid
    unsafe {
        let nv: i32 = (*m).nv as i32;
        let ntendon: i32 = (*m).ntendon as i32;
        let enbl_spring: i32 = (((*m).opt.disableflags & MJ_DSBL_SPRING) == 0) as i32;
        let enbl_damper: i32 = (((*m).opt.disableflags & MJ_DSBL_DAMPER) == 0) as i32;
        let sleep_filter: bool = ((*m).opt.enableflags & MJ_ENBL_SLEEP) != 0
            && (*d).ntree_awake < (*m).ntree as i32;
        let nbody: i32 = if sleep_filter { (*d).nbody_awake } else { (*m).nbody as i32 };

        // joint-level springs
        if enbl_spring != 0 {
            for b in 0..nbody {
                let i: i32 = if sleep_filter { *(*d).body_awake_ind.add(b as usize) } else { b };
                let jnt_start: i32 = *(*m).body_jntadr.add(i as usize);
                let jnt_end: i32 = jnt_start + *(*m).body_jntnum.add(i as usize);
                for j in jnt_start..jnt_end {
                    let stiffness: f64 = *(*m).jnt_stiffness.add(j as usize);
                    let spoly: *const f64 = (*m).jnt_stiffnesspoly.add(MJ_NPOLY * j as usize);

                    if stiffness == 0.0 && mju_is_zero(spoly, MJ_NPOLY as i32) != 0 {
                        continue;
                    }

                    let mut padr: i32 = *(*m).jnt_qposadr.add(j as usize);
                    let mut dadr: i32 = *(*m).jnt_dofadr.add(j as usize);

                    let jnt_type: u32 = *(*m).jnt_type.add(j as usize) as u32;

                    if jnt_type == mjtJoint_mjJNT_FREE {
                        // apply force
                        let mut dif: [f64; 3] = [0.0; 3];
                        mji_sub3(dif.as_mut_ptr(),
                                 (*d).qpos.add(padr as usize),
                                 (*m).qpos_spring.add(padr as usize));
                        let r: f64 = mju_norm3(dif.as_ptr());
                        let k: f64 = mju_poly_force(stiffness, spoly, r, MJ_NPOLY as i32, 0);
                        mji_add_to_scl3((*d).qfrc_spring.add(dadr as usize), dif.as_ptr(), -k);

                        // continue with rotations
                        dadr += 3;
                        padr += 3;
                        // fallthrough to BALL
                        let mut quat: [f64; 4] = [0.0; 4];
                        mji_copy4(quat.as_mut_ptr(), (*d).qpos.add(padr as usize));
                        mju_normalize4(quat.as_mut_ptr());
                        let mut dif2: [f64; 3] = [0.0; 3];
                        mji_sub_quat(dif2.as_mut_ptr(), quat.as_ptr(), (*m).qpos_spring.add(padr as usize));
                        let r2: f64 = mju_norm3(dif2.as_ptr());
                        let k2: f64 = mju_poly_force(stiffness, spoly, r2, MJ_NPOLY as i32, 0);
                        mji_add_to_scl3((*d).qfrc_spring.add(dadr as usize), dif2.as_ptr(), -k2);
                    } else if jnt_type == mjtJoint_mjJNT_BALL {
                        // convert quaternion difference into angular "velocity"
                        let mut dif: [f64; 3] = [0.0; 3];
                        let mut quat: [f64; 4] = [0.0; 4];
                        mji_copy4(quat.as_mut_ptr(), (*d).qpos.add(padr as usize));
                        mju_normalize4(quat.as_mut_ptr());
                        mji_sub_quat(dif.as_mut_ptr(), quat.as_ptr(), (*m).qpos_spring.add(padr as usize));
                        let r: f64 = mju_norm3(dif.as_ptr());
                        let k: f64 = mju_poly_force(stiffness, spoly, r, MJ_NPOLY as i32, 0);

                        // apply torque
                        mji_add_to_scl3((*d).qfrc_spring.add(dadr as usize), dif.as_ptr(), -k);
                    } else if jnt_type == mjtJoint_mjJNT_SLIDE || jnt_type == mjtJoint_mjJNT_HINGE {
                        // apply force or torque
                        let x: f64 = *(*d).qpos.add(padr as usize) - *(*m).qpos_spring.add(padr as usize);
                        *(*d).qfrc_spring.add(dadr as usize) = -x * mju_poly_force(stiffness, spoly, x, MJ_NPOLY as i32, 0);
                    }
                }
            }
        }

        // dof-level dampers
        if enbl_damper != 0 {
            let nv_awake: i32 = if sleep_filter { (*d).nv_awake } else { nv };
            for j in 0..nv_awake {
                let i: i32 = if sleep_filter { *(*d).dof_awake_ind.add(j as usize) } else { j };
                let mut poly: [f64; MJ_NPOLY] = [0.0; MJ_NPOLY];
                mju_copy(poly.as_mut_ptr(), (*m).dof_dampingpoly.add(MJ_NPOLY * i as usize), MJ_NPOLY as i32);
                let damping: f64 = *(*m).dof_damping.add(i as usize)
                    + mj_actuator_damping(m, crate::types::mjtObj_mjOBJ_JOINT, *(*m).dof_jntid.add(i as usize), poly.as_mut_ptr());
                if damping != 0.0 || mju_is_zero(poly.as_ptr(), MJ_NPOLY as i32) == 0 {
                    let v: f64 = *(*d).qvel.add(i as usize);
                    *(*d).qfrc_damper.add(i as usize) = -v * mju_poly_force(damping, poly.as_ptr(), v, MJ_NPOLY as i32, 1);
                }
            }
        }

        // flex elasticity
        for f in 0..(*m).nflex as i32 {
            if *(*m).flex_dim.add(f as usize) == 1 || *(*m).flex_rigid.add(f as usize) {
                continue;
            }

            if *(*m).flex_interp.add(f as usize) != 0 {
                // interpolated flex: stretch forces
                mj_flex_passive_interp(m, d, f, enbl_spring, enbl_damper);

                // interpolated shell bending forces
                mj_flex_passive_bend_interp(m, d, f, enbl_spring, enbl_damper);
            } else {
                // add bending forces
                mj_flex_passive_bend(m, d, f, enbl_spring, enbl_damper);

                // stretch forces
                mj_flex_passive_stretch(m, d, f, enbl_spring, enbl_damper);
            }
        }

        // flexedge-level spring-dampers
        for f in 0..(*m).nflex as i32 {
            let stiffness: f64 = if enbl_spring != 0 { *(*m).flex_edgestiffness.add(f as usize) } else { 0.0 };
            let damping: f64 = if enbl_damper != 0 { *(*m).flex_edgedamping.add(f as usize) } else { 0.0 };

            // disabled or rigid: nothing to do
            if *(*m).flex_rigid.add(f as usize) || (stiffness == 0.0 && damping == 0.0) {
                continue;
            }

            // process non-rigid edges of this flex (global edge index)
            let edgeend: i32 = *(*m).flex_edgeadr.add(f as usize) + *(*m).flex_edgenum.add(f as usize);
            for e in *(*m).flex_edgeadr.add(f as usize)..edgeend {
                // skip rigid
                if *(*m).flexedge_rigid.add(e as usize) {
                    continue;
                }

                // compute spring-damper force along edge
                let frc_spring: f64 = stiffness * (*(*m).flexedge_length0.add(e as usize) - *(*d).flexedge_length.add(e as usize));
                let frc_damper: f64 = -damping * *(*d).flexedge_velocity.add(e as usize);

                // transform to joint torque, add to qfrc_{spring, damper}: always sparse
                let end: i32 = *(*m).flexedge_J_rowadr.add(e as usize) + *(*m).flexedge_J_rownnz.add(e as usize);
                for j in *(*m).flexedge_J_rowadr.add(e as usize)..end {
                    let colind: i32 = *(*m).flexedge_J_colind.add(j as usize);
                    let jval: f64 = *(*d).flexedge_J.add(j as usize);
                    *(*d).qfrc_spring.add(colind as usize) += jval * frc_spring;
                    *(*d).qfrc_damper.add(colind as usize) += jval * frc_damper;
                }
            }
        }

        // tendon-level spring-dampers
        for i in 0..ntendon {
            // skip sleeping or static tendon
            if sleep_filter && mj_sleep_state(m, d as *const _, mjtObj_mjOBJ_TENDON, i) != mjtSleepState_mjS_AWAKE as i32 {
                continue;
            }

            let mut stiffness: f64 = 0.0;
            let mut spoly: *const f64 = std::ptr::null();
            if enbl_spring != 0 {
                stiffness = *(*m).tendon_stiffness.add(i as usize);
                spoly = (*m).tendon_stiffnesspoly.add(MJ_NPOLY * i as usize);
            }

            let mut damping: f64 = 0.0;
            let mut dpoly: [f64; MJ_NPOLY] = [0.0; MJ_NPOLY];
            if enbl_damper != 0 {
                mju_copy(dpoly.as_mut_ptr(), (*m).tendon_dampingpoly.add(MJ_NPOLY * i as usize), MJ_NPOLY as i32);
                damping = *(*m).tendon_damping.add(i as usize) + mj_actuator_damping(m, mjtObj_mjOBJ_TENDON, i, dpoly.as_mut_ptr());
            }

            // both zero: nothing to do
            if stiffness == 0.0 && (enbl_spring == 0 || mju_is_zero(spoly, MJ_NPOLY as i32) != 0)
                && damping == 0.0 && mju_is_zero(dpoly.as_ptr(), MJ_NPOLY as i32) != 0 {
                continue;
            }

            // compute spring force along tendon
            let length: f64 = *(*d).ten_length.add(i as usize);
            let lower: f64 = *(*m).tendon_lengthspring.add(2 * i as usize);
            let upper: f64 = *(*m).tendon_lengthspring.add(2 * i as usize + 1);
            let x: f64 = if length > upper { length - upper } else if length < lower { length - lower } else { 0.0 };
            let frc_spring: f64 = if enbl_spring != 0 { -x * mju_poly_force(stiffness, spoly, x, MJ_NPOLY as i32, 0) } else { 0.0 };

            // compute damper force along tendon
            let v: f64 = *(*d).ten_velocity.add(i as usize);
            let frc_damper: f64 = if enbl_damper != 0 { -v * mju_poly_force(damping, dpoly.as_ptr(), v, MJ_NPOLY as i32, 1) } else { 0.0 };

            // transform to joint torque, add to qfrc_{spring, damper}
            if frc_spring != 0.0 || frc_damper != 0.0 {
                let end: i32 = *(*m).ten_J_rowadr.add(i as usize) + *(*m).ten_J_rownnz.add(i as usize);
                for j in *(*m).ten_J_rowadr.add(i as usize)..end {
                    let k: i32 = *(*m).ten_J_colind.add(j as usize);
                    let jval: f64 = *(*d).ten_J.add(j as usize);
                    *(*d).qfrc_spring.add(k as usize) += jval * frc_spring;
                    *(*d).qfrc_damper.add(k as usize) += jval * frc_damper;
                }
            }
        }
    }
}

/// C: mj_gravcomp (engine/engine_passive.c:817)
/// Calls: mj_applyFT, mji_scl3, mju_norm3
#[allow(unused_variables, non_snake_case)]
pub fn mj_gravcomp(m: *const mjModel, d: *mut mjData) -> i32 {
    use crate::engine::engine_inline::mji_scl3;
    use crate::engine::engine_support::mj_apply_ft;
    use crate::engine::engine_util_blas::mju_norm3;
    const MJ_DSBL_GRAVITY: i32 = 1 << 7;
    const MJ_ENBL_SLEEP: i32 = 1 << 4;

    // SAFETY: caller guarantees m and d are valid
    unsafe {
        if (*m).ngravcomp == 0
            || ((*m).opt.disableflags & MJ_DSBL_GRAVITY) != 0
            || mju_norm3((*m).opt.gravity.as_ptr()) == 0.0
        {
            return 0;
        }

        let mut has_gravcomp: i32 = 0;
        let mut force: [f64; 3] = [0.0; 3];
        let torque: [f64; 3] = [0.0; 3];
        let sleep_filter: bool = ((*m).opt.enableflags & MJ_ENBL_SLEEP) != 0
            && ((*d).nbody_awake as usize) < (*m).nbody as usize;
        let nbody: i32 = if sleep_filter { (*d).nbody_awake } else { (*m).nbody as i32 };

        // apply per-body gravity compensation
        for b in 1..nbody {
            let i: i32 = if sleep_filter {
                *(*d).body_awake_ind.add(b as usize)
            } else {
                b
            };
            if *(*m).body_gravcomp.add(i as usize) != 0.0 {
                has_gravcomp = 1;
                mji_scl3(
                    force.as_mut_ptr(),
                    (*m).opt.gravity.as_ptr(),
                    -(*(*m).body_mass.add(i as usize) * *(*m).body_gravcomp.add(i as usize)),
                );
                mj_apply_ft(
                    m, d,
                    force.as_ptr(),
                    torque.as_ptr(),
                    (*d).xipos.add(3 * i as usize),
                    i,
                    (*d).qfrc_gravcomp,
                );
            }
        }

        has_gravcomp
    }
}

/// C: mj_fluid (engine/engine_passive.c:842)
/// Calls: mj_ellipsoidFluidModel, mj_inertiaBoxFluidModel
#[allow(unused_variables, non_snake_case)]
pub fn mj_fluid(m: *const mjModel, d: *mut mjData) -> i32 {
    const MJ_ENBL_SLEEP: i32 = 1 << 4;
    const MJ_NFLUID: usize = 12;
    const MJ_MINVAL: f64 = 1E-15;

    // SAFETY: caller guarantees m and d are valid
    unsafe {
        // no fluid forces: early return
        if (*m).opt.viscosity == 0.0 && (*m).opt.density == 0.0 {
            return 0;
        }

        let sleep_filter: bool = ((*m).opt.enableflags & MJ_ENBL_SLEEP) != 0
            && ((*d).nbody_awake as usize) < (*m).nbody as usize;
        let nbody: i32 = if sleep_filter { (*d).nbody_awake } else { (*m).nbody as i32 };

        for b in 0..nbody {
            let i: i32 = if sleep_filter {
                *(*d).body_awake_ind.add(b as usize)
            } else {
                b
            };

            if *(*m).body_mass.add(i as usize) < MJ_MINVAL {
                continue;
            }

            // if any child geom uses the ellipsoid model, inertia-box model is disabled for parent body
            let mut use_ellipsoid_model: i32 = 0;
            let geomnum: i32 = *(*m).body_geomnum.add(i as usize);
            let mut j: i32 = 0;
            while j < geomnum && use_ellipsoid_model == 0 {
                let geomid: i32 = *(*m).body_geomadr.add(i as usize) + j;
                use_ellipsoid_model += (*(*m).geom_fluid.add(MJ_NFLUID * geomid as usize) > 0.0) as i32;
                j += 1;
            }

            if use_ellipsoid_model != 0 {
                mj_ellipsoid_fluid_model(m, d, i);
            } else {
                mj_inertia_box_fluid_model(m, d, i);
            }
        }

        1
    }
}

/// C: mj_contactPassive (engine/engine_passive.c:878)
/// Calls: mj_contactJacobian, mj_freeStack, mj_isSparse, mj_markStack, mj_stackAllocInfo, mju_addToScl, mju_mulMatMat, mju_scl
#[allow(unused_variables, non_snake_case)]
pub fn mj_contact_passive(m: *const mjModel, d: *mut mjData) -> i32 {
    use crate::engine::engine_memory::{mj_mark_stack, mj_free_stack, mj_stack_alloc_num, mj_stack_alloc_int};
    use crate::engine::engine_core_util::mj_is_sparse;
    use crate::engine::engine_core_constraint::mj_contact_jacobian;
    use crate::engine::engine_util_blas::{mju_add_to_scl, mju_scl, mju_mul_mat_mat};
    const MJ_DSBL_CONTACT: i32 = 1 << 4;
    const K_CONTACT_STIFFNESS: f64 = 1e4;

    // SAFETY: caller guarantees m and d are valid
    unsafe {
        let ncon: i32 = (*d).ncon;
        let issparse: i32 = mj_is_sparse(m);
        let nv: i32 = (*m).nv as i32;

        if ((*m).opt.disableflags & MJ_DSBL_CONTACT) != 0 || ncon == 0 || nv == 0 {
            return 0;
        }

        // early return if no contact to be included
        let mut has_contact: i32 = 0;
        for i in 0..ncon {
            if (*(*d).contact.add(i as usize)).exclude != 4 {
                continue;
            }
            has_contact = 1;
        }

        if has_contact == 0 {
            return 0;
        }

        // allocate Jacobian
        mj_mark_stack(d);
        let jac: *mut f64 = mj_stack_alloc_num(d, (6 * nv) as usize);
        let jacdif: *mut f64 = mj_stack_alloc_num(d, (6 * nv) as usize);
        let jacdifp: *mut f64 = jacdif;
        let jacdifr: *mut f64 = jacdif.add(3 * nv as usize);
        let jac1p: *mut f64 = mj_stack_alloc_num(d, (3 * nv) as usize);
        let jac2p: *mut f64 = mj_stack_alloc_num(d, (3 * nv) as usize);
        let jac1r: *mut f64 = mj_stack_alloc_num(d, (3 * nv) as usize);
        let jac2r: *mut f64 = mj_stack_alloc_num(d, (3 * nv) as usize);
        let qfrc: *mut f64 = mj_stack_alloc_num(d, nv as usize);
        let chain: *mut i32 = if issparse != 0 { mj_stack_alloc_int(d, nv as usize) } else { std::ptr::null_mut() };

        // find contacts to be included
        for i in 0..ncon {
            if (*(*d).contact.add(i as usize)).exclude != 4 {
                continue;
            }

            // get contact info, safe efc_address
            let con: *mut mjContact = (*d).contact.add(i as usize);
            let dim: i32 = (*con).dim;
            (*con).efc_address = -1;
            let nv_contact: i32 = mj_contact_jacobian(m, d, con, dim, jac, jacdif, jacdifp, jacdifr,
                                                      jac1p, jac2p, jac1r, jac2r, chain);

            // skip contact if no DOFs affected
            if nv_contact == 0 {
                (*con).efc_address = -1;
                (*con).exclude = 3;
                continue;
            }

            // rotate Jacobian differences to contact frame
            mju_mul_mat_mat(jac, (*con).frame.as_ptr(), jacdifp,
                           if dim > 1 { 3 } else { 1 }, 3, nv_contact);
            if dim > 3 {
                mju_mul_mat_mat(jac.add(3 * nv_contact as usize), (*con).frame.as_ptr(), jacdifr,
                               dim - 3, 3, nv_contact);
            }

            // compute passive contact force (dim = 1)
            let scl: f64 = -K_CONTACT_STIFFNESS * (*con).dist;
            if issparse == 0 {
                mju_add_to_scl((*d).qfrc_spring, jac, scl, nv);
            } else {
                mju_scl(qfrc, jac, scl, nv_contact);
                for j in 0..nv_contact {
                    *(*d).qfrc_spring.add(*chain.add(j as usize) as usize) += *qfrc.add(j as usize);
                }
            }
        }

        mj_free_stack(d);
        has_contact
    }
}

/// C: mji_pow4 (engine/engine_passive.c:1215)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_pow4(val: f64) -> f64 {
    (val * val) * (val * val)
}

/// C: mji_pow2 (engine/engine_passive.c:1219)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_pow2(val: f64) -> f64 {
    val * val
}

/// C: mji_ellipsoid_max_moment (engine/engine_passive.c:1223)
/// Calls: mji_pow4, mju_max
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mji_ellipsoid_max_moment(size: *const f64, dir: i32) -> f64 {
    // SAFETY: caller guarantees size points to 3 contiguous f64s
    unsafe {
        let d0 = *size.add(dir as usize);
        let d1 = *size.add(((dir + 1) % 3) as usize);
        let d2 = *size.add(((dir + 2) % 3) as usize);
        8.0 / 15.0 * std::f64::consts::PI * d0
            * mji_pow4(crate::engine::engine_util_misc::mju_max(d1, d2))
    }
}

/// C: mj_passive (engine/engine_passive.h:29)
/// Calls: mj_contactPassive, mj_fluid, mj_gravcomp, mj_springdamper, mjp_getPluginAtSlotUnsafe, mjp_pluginCount, mju_add, mju_addInd, mju_addTo, mju_addToInd, mju_message, mju_zero, mju_zeroInd
#[allow(unused_variables, non_snake_case)]
pub fn mj_passive(m: *const mjModel, d: *mut mjData) {
    use crate::engine::engine_util_blas::{mju_zero, mju_zero_ind, mju_add, mju_add_ind, mju_add_to, mju_add_to_ind};
    use crate::engine::engine_plugin::{mjp_plugin_count, mjp_get_plugin_at_slot_unsafe};
    const MJ_DSBL_SPRING: i32 = 1 << 8;
    const MJ_DSBL_DAMPER: i32 = 1 << 9;
    const MJ_ENBL_SLEEP: i32 = 1 << 4;
    const MJ_PLUGIN_PASSIVE: i32 = 1 << 2;

    // SAFETY: caller guarantees m and d are valid
    unsafe {
        let sleep_filter: bool = ((*m).opt.enableflags & MJ_ENBL_SLEEP) != 0
            && (*d).nv_awake < (*m).nv as i32;
        let nv: i32 = if sleep_filter { (*d).nv_awake } else { (*m).nv as i32 };
        let dof_awake_ind: *const i32 = if sleep_filter { (*d).dof_awake_ind } else { std::ptr::null() };

        // clear passive force vectors for awake dofs
        if sleep_filter {
            mju_zero_ind((*d).qfrc_spring,   nv, dof_awake_ind);
            mju_zero_ind((*d).qfrc_damper,   nv, dof_awake_ind);
            mju_zero_ind((*d).qfrc_gravcomp, nv, dof_awake_ind);
            mju_zero_ind((*d).qfrc_fluid,    nv, dof_awake_ind);
            mju_zero_ind((*d).qfrc_passive,  nv, dof_awake_ind);
        } else {
            mju_zero((*d).qfrc_spring,   nv);
            mju_zero((*d).qfrc_damper,   nv);
            mju_zero((*d).qfrc_gravcomp, nv);
            mju_zero((*d).qfrc_fluid,    nv);
            mju_zero((*d).qfrc_passive,  nv);
        }

        // both spring and damping disabled: skip all passive forces
        if ((*m).opt.disableflags & MJ_DSBL_SPRING) != 0
            && ((*m).opt.disableflags & MJ_DSBL_DAMPER) != 0 {
            return;
        }

        // springs and dampers
        mj_springdamper(m, d);

        // gravity compensation
        let has_gravcomp: i32 = mj_gravcomp(m, d);

        // fluid forces
        let has_fluid: i32 = mj_fluid(m, d);

        // contact forces
        mj_contact_passive(m, d);

        // add passive forces into qfrc_passive
        if sleep_filter {
            mju_add_ind((*d).qfrc_passive, (*d).qfrc_spring, (*d).qfrc_damper, dof_awake_ind, nv);
        } else {
            mju_add((*d).qfrc_passive, (*d).qfrc_spring, (*d).qfrc_damper, nv);
        }

        if has_fluid != 0 {
            if sleep_filter {
                mju_add_to_ind((*d).qfrc_passive, (*d).qfrc_fluid, dof_awake_ind, nv);
            } else {
                mju_add_to((*d).qfrc_passive, (*d).qfrc_fluid, nv);
            }
        }

        if has_gravcomp != 0 {
            let ndof: i32 = if sleep_filter { (*d).nv_awake } else { nv };
            for v in 0..ndof {
                let dof: i32 = if sleep_filter { *(*d).dof_awake_ind.add(v as usize) } else { v };

                // add gravity compensation force unless added via actuators
                if !*(*m).jnt_actgravcomp.add(*(*m).dof_jntid.add(dof as usize) as usize) {
                    *(*d).qfrc_passive.add(dof as usize) += *(*d).qfrc_gravcomp.add(dof as usize);
                }
            }
        }

        // user callback: add custom passive forces
        if let Some(cb) = crate::engine::engine_callback::mjcb_passive {
            cb(m, d);
        }

        // plugin: add custom passive forces
        if (*m).nplugin > 0 {
            let nslot: i32 = mjp_plugin_count();

            // iterate over plugins, call compute if type is mjPLUGIN_PASSIVE
            for i in 0..(*m).nplugin {
                let slot: i32 = *(*m).plugin.add(i as usize);
                let plugin: *const mjpPlugin = mjp_get_plugin_at_slot_unsafe(slot, nslot);
                if plugin.is_null() {
                    // mjERROR - in practice this would abort, but we can't call mjERROR from Rust
                    panic!("invalid plugin slot: {}", slot);
                }
                if ((*plugin).capabilityflags & MJ_PLUGIN_PASSIVE) != 0 {
                    if (*plugin).compute.is_none() {
                        panic!("`compute` is a null function pointer for plugin at slot {}", slot);
                    }
                    // SAFETY: compute is actually fn(m, d, i, type) — cast the function pointer
                    let compute_fn: unsafe extern "C" fn(*const mjModel, *mut mjData, i32, i32) =
                        std::mem::transmute((*plugin).compute.unwrap());
                    compute_fn(m, d, i as i32, MJ_PLUGIN_PASSIVE);
                }
            }
        }
    }
}

/// C: mj_inertiaBoxFluidModel (engine/engine_passive.h:37)
/// Calls: mj_applyFT, mj_objectVelocity, mji_copy3, mji_mulMatVec3, mji_scl3, mji_subFrom3, mju_max, mju_transformSpatial, mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mj_inertia_box_fluid_model(m: *const mjModel, d: *mut mjData, i: i32) {
    use crate::engine::engine_inline::{mji_copy3, mji_scl3, mji_sub_from3, mji_mul_mat_vec3};
    use crate::engine::engine_util_blas::mju_zero;
    use crate::engine::engine_util_misc::mju_max;
    use crate::engine::engine_util_spatial::mju_transform_spatial;
    use crate::engine::engine_core_util::mj_object_velocity;
    use crate::engine::engine_support::mj_apply_ft;
    use crate::types::mjtObj_mjOBJ_BODY;
    const MJ_MINVAL: f64 = 1E-15;

    // SAFETY: caller guarantees m and d are valid
    unsafe {
        let mut lvel: [f64; 6] = [0.0; 6];
        let mut wind: [f64; 6] = [0.0; 6];
        let mut lwind: [f64; 6] = [0.0; 6];
        let mut lfrc: [f64; 6] = [0.0; 6];
        let mut bfrc: [f64; 6] = [0.0; 6];
        let mut box_: [f64; 3] = [0.0; 3];

        let inertia: *mut f64 = (*m).body_inertia.add(3 * i as usize);
        box_[0] = (mju_max(MJ_MINVAL,
            *inertia.add(1) + *inertia.add(2) - *inertia.add(0)) / *(*m).body_mass.add(i as usize) * 6.0).sqrt();
        box_[1] = (mju_max(MJ_MINVAL,
            *inertia.add(0) + *inertia.add(2) - *inertia.add(1)) / *(*m).body_mass.add(i as usize) * 6.0).sqrt();
        box_[2] = (mju_max(MJ_MINVAL,
            *inertia.add(0) + *inertia.add(1) - *inertia.add(2)) / *(*m).body_mass.add(i as usize) * 6.0).sqrt();

        // map from CoM-centered to local body-centered 6D velocity
        mj_object_velocity(m, d as *const _, mjtObj_mjOBJ_BODY as i32, i, lvel.as_mut_ptr(), 1);

        // compute wind in local coordinates
        mju_zero(wind.as_mut_ptr(), 6);
        mji_copy3(wind.as_mut_ptr().add(3), (*m).opt.wind.as_ptr());
        mju_transform_spatial(
            lwind.as_mut_ptr(), wind.as_ptr(), 0,
            (*d).xipos.add(3 * i as usize),
            (*d).subtree_com.add(3 * *(*m).body_rootid.add(i as usize) as usize),
            (*d).ximat.add(9 * i as usize),
        );

        // subtract translational component from body velocity
        mji_sub_from3(lvel.as_mut_ptr().add(3), lwind.as_ptr().add(3));
        mju_zero(lfrc.as_mut_ptr(), 6);

        // set viscous force and torque
        if (*m).opt.viscosity > 0.0 {
            // diameter of sphere approximation
            let diam: f64 = (box_[0] + box_[1] + box_[2]) / 3.0;

            // angular viscosity
            mji_scl3(lfrc.as_mut_ptr(), lvel.as_ptr(),
                     -std::f64::consts::PI * diam * diam * diam * (*m).opt.viscosity);

            // linear viscosity
            mji_scl3(lfrc.as_mut_ptr().add(3), lvel.as_ptr().add(3),
                     -3.0 * std::f64::consts::PI * diam * (*m).opt.viscosity);
        }

        // add lift and drag force and torque
        if (*m).opt.density > 0.0 {
            // force
            lfrc[3] -= 0.5 * (*m).opt.density * box_[1] * box_[2] * lvel[3].abs() * lvel[3];
            lfrc[4] -= 0.5 * (*m).opt.density * box_[0] * box_[2] * lvel[4].abs() * lvel[4];
            lfrc[5] -= 0.5 * (*m).opt.density * box_[0] * box_[1] * lvel[5].abs() * lvel[5];

            // torque
            lfrc[0] -= (*m).opt.density * box_[0]
                * (box_[1] * box_[1] * box_[1] * box_[1] + box_[2] * box_[2] * box_[2] * box_[2])
                * lvel[0].abs() * lvel[0] / 64.0;
            lfrc[1] -= (*m).opt.density * box_[1]
                * (box_[0] * box_[0] * box_[0] * box_[0] + box_[2] * box_[2] * box_[2] * box_[2])
                * lvel[1].abs() * lvel[1] / 64.0;
            lfrc[2] -= (*m).opt.density * box_[2]
                * (box_[0] * box_[0] * box_[0] * box_[0] + box_[1] * box_[1] * box_[1] * box_[1])
                * lvel[2].abs() * lvel[2] / 64.0;
        }

        // rotate to global orientation: lfrc -> bfrc
        mji_mul_mat_vec3(bfrc.as_mut_ptr(), (*d).ximat.add(9 * i as usize), lfrc.as_ptr());
        mji_mul_mat_vec3(bfrc.as_mut_ptr().add(3), (*d).ximat.add(9 * i as usize), lfrc.as_ptr().add(3));

        // apply force and torque to body com
        mj_apply_ft(m, d, bfrc.as_ptr().add(3), bfrc.as_ptr(),
                    (*d).xipos.add(3 * i as usize), i, (*d).qfrc_fluid);
    }
}

/// C: mj_ellipsoidFluidModel (engine/engine_passive.h:40)
/// Calls: mj_addedMassForces, mj_applyFT, mj_objectVelocity, mj_viscousForces, mji_copy3, mji_mulMatVec3, mji_subFrom3, mju_geomSemiAxes, mju_scl, mju_transformSpatial, mju_zero, readFluidGeomInteraction
#[allow(unused_variables, non_snake_case)]
pub fn mj_ellipsoid_fluid_model(m: *const mjModel, d: *mut mjData, bodyid: i32) {
    use crate::engine::engine_inline::{mji_copy3, mji_sub_from3, mji_mul_mat_vec3};
    use crate::engine::engine_util_blas::{mju_zero, mju_scl};
    use crate::engine::engine_util_misc::mju_geom_semi_axes;
    use crate::engine::engine_util_spatial::mju_transform_spatial;
    use crate::engine::engine_core_util::mj_object_velocity;
    use crate::engine::engine_support::mj_apply_ft;
    use crate::types::mjtObj_mjOBJ_GEOM;
    const MJ_NFLUID: usize = 12;

    // SAFETY: caller guarantees m and d are valid
    unsafe {
        let mut lvel: [f64; 6] = [0.0; 6];
        let mut wind: [f64; 6] = [0.0; 6];
        let mut lwind: [f64; 6] = [0.0; 6];
        let mut lfrc: [f64; 6] = [0.0; 6];
        let mut bfrc: [f64; 6] = [0.0; 6];
        let mut geom_interaction_coef: f64 = 0.0;
        let mut magnus_lift_coef: f64 = 0.0;
        let mut kutta_lift_coef: f64 = 0.0;
        let mut semiaxes: [f64; 3] = [0.0; 3];
        let mut virtual_mass: [f64; 3] = [0.0; 3];
        let mut virtual_inertia: [f64; 3] = [0.0; 3];
        let mut blunt_drag_coef: f64 = 0.0;
        let mut slender_drag_coef: f64 = 0.0;
        let mut ang_drag_coef: f64 = 0.0;

        for j in 0..*(*m).body_geomnum.add(bodyid as usize) {
            let geomid: i32 = *(*m).body_geomadr.add(bodyid as usize) + j;

            mju_geom_semi_axes(
                semiaxes.as_mut_ptr(),
                (*m).geom_size.add(3 * geomid as usize),
                *(*m).geom_type.add(geomid as usize) as u32,
            );

            read_fluid_geom_interaction(
                (*m).geom_fluid.add(MJ_NFLUID * geomid as usize),
                &mut geom_interaction_coef,
                &mut blunt_drag_coef,
                &mut slender_drag_coef,
                &mut ang_drag_coef,
                &mut kutta_lift_coef,
                &mut magnus_lift_coef,
                virtual_mass.as_mut_ptr(),
                virtual_inertia.as_mut_ptr(),
            );

            // scales all forces, read from MJCF as boolean (0.0 or 1.0)
            if geom_interaction_coef == 0.0 {
                continue;
            }

            // map from CoM-centered to local body-centered 6D velocity
            mj_object_velocity(m, d as *const _, mjtObj_mjOBJ_GEOM as i32, geomid, lvel.as_mut_ptr(), 1);

            // compute wind in local coordinates
            mju_zero(wind.as_mut_ptr(), 6);
            mji_copy3(wind.as_mut_ptr().add(3), (*m).opt.wind.as_ptr());
            mju_transform_spatial(
                lwind.as_mut_ptr(), wind.as_ptr(), 0,
                (*d).geom_xpos.add(3 * geomid as usize),
                (*d).subtree_com.add(3 * *(*m).body_rootid.add(bodyid as usize) as usize),
                (*d).geom_xmat.add(9 * geomid as usize),
            );

            // subtract translational component from geom velocity
            mji_sub_from3(lvel.as_mut_ptr().add(3), lwind.as_ptr().add(3));

            // initialize viscous force and torque
            mju_zero(lfrc.as_mut_ptr(), 6);

            // added-mass forces and torques
            mj_added_mass_forces(
                lvel.as_ptr(), std::ptr::null(), (*m).opt.density,
                virtual_mass.as_ptr(), virtual_inertia.as_ptr(), lfrc.as_mut_ptr(),
            );

            // lift force orthogonal to lvel from Kutta-Joukowski theorem
            mj_viscous_forces(
                lvel.as_ptr(), (*m).opt.density, (*m).opt.viscosity,
                semiaxes.as_ptr(), magnus_lift_coef, kutta_lift_coef,
                blunt_drag_coef, slender_drag_coef, ang_drag_coef, lfrc.as_mut_ptr(),
            );

            // scale by geom_interaction_coef (1.0 by default)
            mju_scl(lfrc.as_mut_ptr(), lfrc.as_ptr(), geom_interaction_coef, 6);

            // rotate to global orientation: lfrc -> bfrc
            mji_mul_mat_vec3(bfrc.as_mut_ptr(), (*d).geom_xmat.add(9 * geomid as usize), lfrc.as_ptr());
            mji_mul_mat_vec3(bfrc.as_mut_ptr().add(3), (*d).geom_xmat.add(9 * geomid as usize), lfrc.as_ptr().add(3));

            // apply force and torque to body com
            mj_apply_ft(
                m, d, bfrc.as_ptr().add(3), bfrc.as_ptr(),
                (*d).geom_xpos.add(3 * geomid as usize),
                bodyid, (*d).qfrc_fluid,
            );
        }
    }
}

/// C: mj_addedMassForces (engine/engine_passive.h:43)
/// Calls: mji_addTo3, mji_cross
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_added_mass_forces(local_vels: *const f64, local_accels: *const f64, fluid_density: f64, virtual_mass: *const f64, virtual_inertia: *const f64, local_force: *mut f64) {
    use crate::engine::engine_inline::{mji_cross, mji_add_to3};
    // SAFETY: caller guarantees all pointers are valid and arrays are properly sized
    unsafe {
        let lin_vel: [f64; 3] = [*local_vels.add(3), *local_vels.add(4), *local_vels.add(5)];
        let ang_vel: [f64; 3] = [*local_vels.add(0), *local_vels.add(1), *local_vels.add(2)];
        let virtual_lin_mom: [f64; 3] = [
            fluid_density * *virtual_mass.add(0) * lin_vel[0],
            fluid_density * *virtual_mass.add(1) * lin_vel[1],
            fluid_density * *virtual_mass.add(2) * lin_vel[2],
        ];
        let virtual_ang_mom: [f64; 3] = [
            fluid_density * *virtual_inertia.add(0) * ang_vel[0],
            fluid_density * *virtual_inertia.add(1) * ang_vel[1],
            fluid_density * *virtual_inertia.add(2) * ang_vel[2],
        ];

        // disabled due to dependency on qacc but included for completeness
        if !local_accels.is_null() {
            *local_force.add(0) -= fluid_density * *virtual_inertia.add(0) * *local_accels.add(0);
            *local_force.add(1) -= fluid_density * *virtual_inertia.add(1) * *local_accels.add(1);
            *local_force.add(2) -= fluid_density * *virtual_inertia.add(2) * *local_accels.add(2);
            *local_force.add(3) -= fluid_density * *virtual_mass.add(0) * *local_accels.add(3);
            *local_force.add(4) -= fluid_density * *virtual_mass.add(1) * *local_accels.add(4);
            *local_force.add(5) -= fluid_density * *virtual_mass.add(2) * *local_accels.add(5);
        }

        let mut added_mass_force: [f64; 3] = [0.0; 3];
        let mut added_mass_torque1: [f64; 3] = [0.0; 3];
        let mut added_mass_torque2: [f64; 3] = [0.0; 3];
        mji_cross(added_mass_force.as_mut_ptr(), virtual_lin_mom.as_ptr(), ang_vel.as_ptr());
        mji_cross(added_mass_torque1.as_mut_ptr(), virtual_lin_mom.as_ptr(), lin_vel.as_ptr());
        mji_cross(added_mass_torque2.as_mut_ptr(), virtual_ang_mom.as_ptr(), ang_vel.as_ptr());

        mji_add_to3(local_force, added_mass_torque1.as_ptr());
        mji_add_to3(local_force, added_mass_torque2.as_ptr());
        mji_add_to3(local_force.add(3), added_mass_force.as_ptr());
    }
}

/// C: mj_viscousForces (engine/engine_passive.h:49)
/// Calls: mji_cross, mji_ellipsoid_max_moment, mji_pow2, mji_pow4, mju_max, mju_min, mju_norm3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_viscous_forces(local_vels: *const f64, fluid_density: f64, fluid_viscosity: f64, size: *const f64, magnus_lift_coef: f64, kutta_lift_coef: f64, blunt_drag_coef: f64, slender_drag_coef: f64, ang_drag_coef: f64, local_force: *mut f64) {
    use crate::engine::engine_inline::mji_cross;
    use crate::engine::engine_util_blas::mju_norm3;
    use crate::engine::engine_util_misc::{mju_max, mju_min};
    const MJ_MINVAL: f64 = 1E-15;

    // SAFETY: caller guarantees all pointers are valid and arrays are properly sized
    unsafe {
        let lin_vel: [f64; 3] = [*local_vels.add(3), *local_vels.add(4), *local_vels.add(5)];
        let ang_vel: [f64; 3] = [*local_vels.add(0), *local_vels.add(1), *local_vels.add(2)];
        let volume: f64 = 4.0 / 3.0 * std::f64::consts::PI * *size.add(0) * *size.add(1) * *size.add(2);
        let d_max: f64 = mju_max(mju_max(*size.add(0), *size.add(1)), *size.add(2));
        let d_min: f64 = mju_min(mju_min(*size.add(0), *size.add(1)), *size.add(2));
        let d_mid: f64 = *size.add(0) + *size.add(1) + *size.add(2) - d_max - d_min;
        let a_max: f64 = std::f64::consts::PI * d_max * d_mid;

        let mut magnus_force: [f64; 3] = [0.0; 3];
        mji_cross(magnus_force.as_mut_ptr(), ang_vel.as_ptr(), lin_vel.as_ptr());
        magnus_force[0] *= magnus_lift_coef * fluid_density * volume;
        magnus_force[1] *= magnus_lift_coef * fluid_density * volume;
        magnus_force[2] *= magnus_lift_coef * fluid_density * volume;

        // the dot product between velocity and the normal to the cross-section that
        // defines the body's projection along velocity is proj_num/sqrt(proj_denom)
        let proj_denom: f64 = mji_pow4(*size.add(1) * *size.add(2)) * mji_pow2(lin_vel[0])
                            + mji_pow4(*size.add(2) * *size.add(0)) * mji_pow2(lin_vel[1])
                            + mji_pow4(*size.add(0) * *size.add(1)) * mji_pow2(lin_vel[2]);
        let proj_num: f64 = mji_pow2(*size.add(1) * *size.add(2) * lin_vel[0])
                          + mji_pow2(*size.add(2) * *size.add(0) * lin_vel[1])
                          + mji_pow2(*size.add(0) * *size.add(1) * lin_vel[2]);

        // projected surface in the direction of the velocity
        let a_proj: f64 = std::f64::consts::PI * (proj_denom / mju_max(MJ_MINVAL, proj_num)).sqrt();

        // not-unit normal to ellipsoid's projected area in the direction of velocity
        let norm: [f64; 3] = [
            mji_pow2(*size.add(1) * *size.add(2)) * lin_vel[0],
            mji_pow2(*size.add(2) * *size.add(0)) * lin_vel[1],
            mji_pow2(*size.add(0) * *size.add(1)) * lin_vel[2],
        ];

        // cosine between velocity and normal to the surface
        // divided by proj_denom instead of sqrt(proj_denom) to account for skipped normalization in norm
        let cos_alpha: f64 = proj_num / mju_max(
            MJ_MINVAL, mju_norm3(lin_vel.as_ptr()) * proj_denom);
        let mut kutta_circ: [f64; 3] = [0.0; 3];
        mji_cross(kutta_circ.as_mut_ptr(), norm.as_ptr(), lin_vel.as_ptr());
        kutta_circ[0] *= kutta_lift_coef * fluid_density * cos_alpha * a_proj;
        kutta_circ[1] *= kutta_lift_coef * fluid_density * cos_alpha * a_proj;
        kutta_circ[2] *= kutta_lift_coef * fluid_density * cos_alpha * a_proj;
        let mut kutta_force: [f64; 3] = [0.0; 3];
        mji_cross(kutta_force.as_mut_ptr(), kutta_circ.as_ptr(), lin_vel.as_ptr());

        // viscous force and torque in Stokes flow, analytical for spherical bodies
        let eq_sphere_d: f64 = 2.0 / 3.0 * (*size.add(0) + *size.add(1) + *size.add(2));
        let lin_visc_force_coef: f64 = 3.0 * std::f64::consts::PI * eq_sphere_d;
        let lin_visc_torq_coef: f64 = std::f64::consts::PI * eq_sphere_d * eq_sphere_d * eq_sphere_d;

        // moments of inertia used to compute angular quadratic drag
        let i_max: f64 = 8.0 / 15.0 * std::f64::consts::PI * d_mid * mji_pow4(d_max);
        let ii: [f64; 3] = [
            mji_ellipsoid_max_moment(size, 0),
            mji_ellipsoid_max_moment(size, 1),
            mji_ellipsoid_max_moment(size, 2),
        ];
        let mom_visc: [f64; 3] = [
            ang_vel[0] * (ang_drag_coef * ii[0] + slender_drag_coef * (i_max - ii[0])),
            ang_vel[1] * (ang_drag_coef * ii[1] + slender_drag_coef * (i_max - ii[1])),
            ang_vel[2] * (ang_drag_coef * ii[2] + slender_drag_coef * (i_max - ii[2])),
        ];

        let drag_lin_coef: f64 =  // linear plus quadratic
            fluid_viscosity * lin_visc_force_coef + fluid_density * mju_norm3(lin_vel.as_ptr()) * (
                a_proj * blunt_drag_coef + slender_drag_coef * (a_max - a_proj));
        let drag_ang_coef: f64 =  // linear plus quadratic
            fluid_viscosity * lin_visc_torq_coef
            + fluid_density * mju_norm3(mom_visc.as_ptr());

        *local_force.add(0) -= drag_ang_coef * ang_vel[0];
        *local_force.add(1) -= drag_ang_coef * ang_vel[1];
        *local_force.add(2) -= drag_ang_coef * ang_vel[2];
        *local_force.add(3) += magnus_force[0] + kutta_force[0] - drag_lin_coef * lin_vel[0];
        *local_force.add(4) += magnus_force[1] + kutta_force[1] - drag_lin_coef * lin_vel[1];
        *local_force.add(5) += magnus_force[2] + kutta_force[2] - drag_lin_coef * lin_vel[2];
    }
}

/// C: readFluidGeomInteraction (engine/engine_passive.h:56)
/// Calls: mju_message
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn read_fluid_geom_interaction(geom_fluid_coefs: *const f64, geom_fluid_coef: *mut f64, blunt_drag_coef: *mut f64, slender_drag_coef: *mut f64, ang_drag_coef: *mut f64, kutta_lift_coef: *mut f64, magnus_lift_coef: *mut f64, virtual_mass: *mut f64, virtual_inertia: *mut f64) {
    const MJ_NFLUID: usize = 12;
    // SAFETY: caller guarantees all pointers are valid and arrays are properly sized
    unsafe {
        let mut i: usize = 0;
        *geom_fluid_coef       = *geom_fluid_coefs.add(i); i += 1;
        *blunt_drag_coef       = *geom_fluid_coefs.add(i); i += 1;
        *slender_drag_coef     = *geom_fluid_coefs.add(i); i += 1;
        *ang_drag_coef         = *geom_fluid_coefs.add(i); i += 1;
        *kutta_lift_coef       = *geom_fluid_coefs.add(i); i += 1;
        *magnus_lift_coef      = *geom_fluid_coefs.add(i); i += 1;
        *virtual_mass.add(0)   = *geom_fluid_coefs.add(i); i += 1;
        *virtual_mass.add(1)   = *geom_fluid_coefs.add(i); i += 1;
        *virtual_mass.add(2)   = *geom_fluid_coefs.add(i); i += 1;
        *virtual_inertia.add(0) = *geom_fluid_coefs.add(i); i += 1;
        *virtual_inertia.add(1) = *geom_fluid_coefs.add(i); i += 1;
        *virtual_inertia.add(2) = *geom_fluid_coefs.add(i); i += 1;
        debug_assert_eq!(i, MJ_NFLUID);
    }
}

/// C: writeFluidGeomInteraction (engine/engine_passive.h:66)
/// Calls: mju_message
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn write_fluid_geom_interaction(geom_fluid_coefs: *mut f64, geom_fluid_coef: *const f64, blunt_drag_coef: *const f64, slender_drag_coef: *const f64, ang_drag_coef: *const f64, kutta_lift_coef: *const f64, magnus_lift_coef: *const f64, virtual_mass: *const f64, virtual_inertia: *const f64) {
    const MJ_NFLUID: usize = 12;
    // SAFETY: caller guarantees all pointers are valid and arrays are properly sized
    unsafe {
        let mut i: usize = 0;
        *geom_fluid_coefs.add(i) = *geom_fluid_coef;       i += 1;
        *geom_fluid_coefs.add(i) = *blunt_drag_coef;       i += 1;
        *geom_fluid_coefs.add(i) = *slender_drag_coef;     i += 1;
        *geom_fluid_coefs.add(i) = *ang_drag_coef;         i += 1;
        *geom_fluid_coefs.add(i) = *kutta_lift_coef;       i += 1;
        *geom_fluid_coefs.add(i) = *magnus_lift_coef;      i += 1;
        *geom_fluid_coefs.add(i) = *virtual_mass.add(0);   i += 1;
        *geom_fluid_coefs.add(i) = *virtual_mass.add(1);   i += 1;
        *geom_fluid_coefs.add(i) = *virtual_mass.add(2);   i += 1;
        *geom_fluid_coefs.add(i) = *virtual_inertia.add(0); i += 1;
        *geom_fluid_coefs.add(i) = *virtual_inertia.add(1); i += 1;
        *geom_fluid_coefs.add(i) = *virtual_inertia.add(2); i += 1;
        debug_assert_eq!(i, MJ_NFLUID);
    }
}

