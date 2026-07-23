//! Port of: engine/engine_core_smooth.c
//! IR hash: bd605ac8158c32d6
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mj_updateDynamicBVH (engine/engine_core_smooth.c:490)
/// Calls: mj_freeStack, mj_markStack, mj_stackAllocInfo, mju_max, mju_min, mju_zeroInt
#[allow(unused_variables, non_snake_case)]
pub fn mj_update_dynamic_bvh(m: *const mjModel, d: *mut mjData, bvhadr: i32, bvhnum: i32) {
    // SAFETY: m, d are valid pointers (caller contract).
    unsafe {
        crate::engine::engine_memory::mj_mark_stack(d);
        let modified = crate::engine::engine_memory::mj_stack_alloc_int(d, bvhnum as usize);
        crate::engine::engine_util_misc::mju_zero_int(modified, bvhnum);

        // mark leafs as modified
        for i in 0..bvhnum {
            if *(*m).bvh_nodeid.add((bvhadr + i) as usize) >= 0 {
                *modified.add(i as usize) = 1;
            }
        }

        // update non-leafs in backward pass (parents come before children)
        for i in (0..bvhnum).rev() {
            if *(*m).bvh_nodeid.add((bvhadr + i) as usize) < 0 {
                let child1 = *(*m).bvh_child.add(2 * (bvhadr + i) as usize);
                let child2 = *(*m).bvh_child.add(2 * (bvhadr + i) as usize + 1);

                // update if either child is modified
                if *modified.add(child1 as usize) != 0 || *modified.add(child2 as usize) != 0 {
                    let nbvhstatic = (*m).nbvhstatic as i32;
                    let aabb = (*d).bvh_aabb_dyn.add(6 * (bvhadr - nbvhstatic + i) as usize);
                    let aabb1 = (*d).bvh_aabb_dyn.add(6 * (bvhadr - nbvhstatic + child1) as usize);
                    let aabb2 = (*d).bvh_aabb_dyn.add(6 * (bvhadr - nbvhstatic + child2) as usize);

                    // compute new (min, max)
                    let mut xmin: [f64; 3] = [0.0; 3];
                    let mut xmax: [f64; 3] = [0.0; 3];
                    for k in 0..3usize {
                        xmin[k] = crate::engine::engine_util_misc::mju_min(
                            *aabb1.add(k) - *aabb1.add(k + 3),
                            *aabb2.add(k) - *aabb2.add(k + 3),
                        );
                        xmax[k] = crate::engine::engine_util_misc::mju_max(
                            *aabb1.add(k) + *aabb1.add(k + 3),
                            *aabb2.add(k) + *aabb2.add(k + 3),
                        );
                    }

                    // convert to (center, size)
                    for k in 0..3usize {
                        *aabb.add(k) = 0.5 * (xmax[k] + xmin[k]);
                        *aabb.add(k + 3) = 0.5 * (xmax[k] - xmin[k]);
                    }

                    *modified.add(i as usize) = 1;
                }
            }
        }

        crate::engine::engine_memory::mj_free_stack(d);
    }
}

/// C: mju_mulMatMat322 (engine/engine_core_smooth.c:537)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_mul_mat_mat322(C: *mut f64, A: *const f64, B: *const f64) {
    // SAFETY: caller guarantees C[6], A[6], B[4] are valid
    unsafe {
        *C.add(0) = *A.add(0) * *B.add(0) + *A.add(1) * *B.add(2);
        *C.add(1) = *A.add(0) * *B.add(1) + *A.add(1) * *B.add(3);
        *C.add(2) = *A.add(2) * *B.add(0) + *A.add(3) * *B.add(2);
        *C.add(3) = *A.add(2) * *B.add(1) + *A.add(3) * *B.add(3);
        *C.add(4) = *A.add(4) * *B.add(0) + *A.add(5) * *B.add(2);
        *C.add(5) = *A.add(4) * *B.add(1) + *A.add(5) * *B.add(3);
    }
}

/// C: mj_kinematics1 (engine/engine_core_smooth.h:29)
/// Calls: mji_addTo3, mji_addToScl3, mji_axisAngle2Quat, mji_copy3, mji_copy4, mji_mulMatVec3, mji_mulQuat, mji_rotVecQuat, mji_sub3, mju_message, mju_mulQuat, mju_normalize4, mju_quat2Mat, mju_unit4, mju_zero, mju_zero3
#[allow(unused_variables, non_snake_case)]
pub fn mj_kinematics1(m: *const mjModel, d: *mut mjData) {
    todo!() // mj_kinematics1
}

/// C: mj_kinematics2 (engine/engine_core_smooth.h:32)
/// Calls: mj_local2Global
#[allow(unused_variables, non_snake_case)]
pub fn mj_kinematics2(m: *const mjModel, d: *mut mjData) {
    const MJ_ENBL_SLEEP: i32 = 1 << 4;
    const MJ_S_AWAKE: i32 = 1;
    // SAFETY: caller guarantees m, d are valid
    unsafe {
        let sleep_filter = ((*m).opt.enableflags & MJ_ENBL_SLEEP) != 0
            && ((*d).nbody_awake as i64) < (*m).nbody;
        let nbody = if sleep_filter { (*d).nbody_awake } else { (*m).nbody as i32 };

        // compute/copy Cartesian positions and orientations of body inertial frames
        for b in 1..nbody {
            let i = if sleep_filter { *(*d).body_awake_ind.add(b as usize) } else { b };
            crate::engine::engine_core_util::mj_local2global(
                d, (*d).xipos.add(3 * i as usize), (*d).ximat.add(9 * i as usize),
                (*m).body_ipos.add(3 * i as usize), (*m).body_iquat.add(4 * i as usize),
                i, *(*m).body_sameframe.add(i as usize));
        }

        // compute/copy Cartesian positions and orientations of geoms
        for b in 0..nbody {
            let i = if sleep_filter { *(*d).body_awake_ind.add(b as usize) } else { b };

            // skip geom in sleeping or static body
            if sleep_filter && *(*d).body_awake.add(i as usize) != MJ_S_AWAKE {
                continue;
            }

            let start = *(*m).body_geomadr.add(i as usize);
            let end = start + *(*m).body_geomnum.add(i as usize);
            for g in start..end {
                crate::engine::engine_core_util::mj_local2global(
                    d, (*d).geom_xpos.add(3 * g as usize), (*d).geom_xmat.add(9 * g as usize),
                    (*m).geom_pos.add(3 * g as usize), (*m).geom_quat.add(4 * g as usize),
                    *(*m).geom_bodyid.add(g as usize), *(*m).geom_sameframe.add(g as usize));
            }
        }

        // compute/copy Cartesian positions and orientations of sites
        let nsite = (*m).nsite as i32;
        for i in 0..nsite {
            let bodyid = *(*m).site_bodyid.add(i as usize);

            // skip site in sleeping or static body
            if sleep_filter && *(*d).body_awake.add(bodyid as usize) != MJ_S_AWAKE {
                continue;
            }

            crate::engine::engine_core_util::mj_local2global(
                d, (*d).site_xpos.add(3 * i as usize), (*d).site_xmat.add(9 * i as usize),
                (*m).site_pos.add(3 * i as usize), (*m).site_quat.add(4 * i as usize),
                bodyid, *(*m).site_sameframe.add(i as usize));
        }
    }
}

/// C: mj_kinematics (engine/engine_core_smooth.h:35)
/// Calls: mj_kinematics1, mj_kinematics2, mj_updateSleep, mj_wake
#[allow(unused_variables, non_snake_case)]
pub fn mj_kinematics(m: *const mjModel, d: *mut mjData) {
    todo!() // mj_kinematics
}

/// C: mj_comPos (engine/engine_core_smooth.h:38)
/// Calls: mji_addTo3, mji_copy3, mji_scl3, mji_sub3, mju_dofCom, mju_inertCom, mju_scl3, mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mj_com_pos(m: *const mjModel, d: *mut mjData) {
    const MJ_ENBL_SLEEP: i32 = 1 << 4;
    const MJ_MINVAL: f64 = 1E-15;
    const MJ_JNT_FREE: i32 = 0;
    const MJ_JNT_BALL: i32 = 1;
    const MJ_JNT_SLIDE: i32 = 2;
    const MJ_JNT_HINGE: i32 = 3;
    const MJ_S_ASLEEP: i32 = 0;

    // SAFETY: caller guarantees m, d are valid pointers to initialized model/data
    unsafe {
        let sleep_filter = ((*m).opt.enableflags & MJ_ENBL_SLEEP) != 0
            && ((*d).nbody_awake as i64) < (*m).nbody;
        let nbody = if sleep_filter { (*d).nbody_awake } else { (*m).nbody as i32 };
        let nparent = if sleep_filter { (*d).nparent_awake } else { (*m).nbody as i32 };

        // subtree_com: initialize with body moment
        for b in 0..nbody {
            let i = if sleep_filter { *(*d).body_awake_ind.add(b as usize) } else { b };
            crate::engine::engine_inline::mji_scl3(
                (*d).subtree_com.add(3 * i as usize),
                (*d).xipos.add(3 * i as usize),
                *(*m).body_mass.add(i as usize));
        }

        // subtree_com: accumulate to parent in backward pass
        for b in (0..nparent).rev() {
            let i = if sleep_filter { *(*d).parent_awake_ind.add(b as usize) } else { b };
            if i == 0 { continue; }

            let parent = *(*m).body_parentid.add(i as usize);
            if sleep_filter && *(*d).body_awake.add(i as usize) == MJ_S_ASLEEP {
                let mut child_moment: [f64; 3] = [0.0; 3];
                crate::engine::engine_inline::mji_scl3(
                    child_moment.as_mut_ptr(),
                    (*d).subtree_com.add(3 * i as usize),
                    *(*m).body_subtreemass.add(i as usize));
                crate::engine::engine_inline::mji_add_to3(
                    (*d).subtree_com.add(3 * parent as usize),
                    child_moment.as_ptr());
            } else {
                crate::engine::engine_inline::mji_add_to3(
                    (*d).subtree_com.add(3 * parent as usize),
                    (*d).subtree_com.add(3 * i as usize));
            }
        }

        // subtree_com: normalize
        for b in 0..nbody {
            let i = if sleep_filter { *(*d).body_awake_ind.add(b as usize) } else { b };
            if *(*m).body_subtreemass.add(i as usize) < MJ_MINVAL {
                crate::engine::engine_inline::mji_copy3(
                    (*d).subtree_com.add(3 * i as usize),
                    (*d).xipos.add(3 * i as usize));
            } else {
                crate::engine::engine_util_blas::mju_scl3(
                    (*d).subtree_com.add(3 * i as usize),
                    (*d).subtree_com.add(3 * i as usize),
                    1.0 / *(*m).body_subtreemass.add(i as usize));
            }
        }

        // zero out CoM frame inertia for the world body
        crate::engine::engine_util_blas::mju_zero((*d).cinert, 10);

        // map inertias to frame centered at subtree_com
        for b in 1..nbody {
            let i = if sleep_filter { *(*d).body_awake_ind.add(b as usize) } else { b };
            let mut offset: [f64; 3] = [0.0; 3];
            crate::engine::engine_inline::mji_sub3(
                offset.as_mut_ptr(),
                (*d).xipos.add(3 * i as usize),
                (*d).subtree_com.add(3 * *(*m).body_rootid.add(i as usize) as usize));
            crate::engine::engine_util_spatial::mju_inert_com(
                (*d).cinert.add(10 * i as usize),
                (*m).body_inertia.add(3 * i as usize),
                (*d).ximat.add(9 * i as usize),
                offset.as_ptr(),
                *(*m).body_mass.add(i as usize));
        }

        // map motion dofs to global frame centered at subtree_com
        for b in 1..nbody {
            let i = if sleep_filter { *(*d).body_awake_ind.add(b as usize) } else { b };
            let jntnum = *(*m).body_jntnum.add(i as usize);
            if jntnum == 0 { continue; }

            let start = *(*m).body_jntadr.add(i as usize);
            let end = start + jntnum;
            for j in start..end {
                let da = (6 * *(*m).jnt_dofadr.add(j as usize)) as usize;

                // compute com-anchor vector
                let mut offset: [f64; 3] = [0.0; 3];
                crate::engine::engine_inline::mji_sub3(
                    offset.as_mut_ptr(),
                    (*d).subtree_com.add(3 * *(*m).body_rootid.add(i as usize) as usize),
                    (*d).xanchor.add(3 * j as usize));

                let jnt_type = *(*m).jnt_type.add(j as usize);
                match jnt_type {
                    MJ_JNT_FREE => {
                        // translation components
                        crate::engine::engine_util_blas::mju_zero((*d).cdof.add(da), 18);
                        *(*d).cdof.add(da + 3 + 7 * 0) = 1.0;
                        *(*d).cdof.add(da + 3 + 7 * 1) = 1.0;
                        *(*d).cdof.add(da + 3 + 7 * 2) = 1.0;

                        // rotation components (fallthrough to ball)
                        let mut axis: [f64; 3] = [0.0; 3];
                        for k in 0..3usize {
                            axis[0] = *(*d).xmat.add(9 * i as usize + k + 0);
                            axis[1] = *(*d).xmat.add(9 * i as usize + k + 3);
                            axis[2] = *(*d).xmat.add(9 * i as usize + k + 6);
                            crate::engine::engine_util_spatial::mju_dof_com(
                                (*d).cdof.add(da + 18 + 6 * k), axis.as_ptr(), offset.as_ptr());
                        }
                    }
                    MJ_JNT_BALL => {
                        let mut axis: [f64; 3] = [0.0; 3];
                        for k in 0..3usize {
                            axis[0] = *(*d).xmat.add(9 * i as usize + k + 0);
                            axis[1] = *(*d).xmat.add(9 * i as usize + k + 3);
                            axis[2] = *(*d).xmat.add(9 * i as usize + k + 6);
                            crate::engine::engine_util_spatial::mju_dof_com(
                                (*d).cdof.add(da + 6 * k), axis.as_ptr(), offset.as_ptr());
                        }
                    }
                    MJ_JNT_SLIDE => {
                        crate::engine::engine_util_spatial::mju_dof_com(
                            (*d).cdof.add(da), (*d).xaxis.add(3 * j as usize), std::ptr::null());
                    }
                    MJ_JNT_HINGE => {
                        crate::engine::engine_util_spatial::mju_dof_com(
                            (*d).cdof.add(da), (*d).xaxis.add(3 * j as usize), offset.as_ptr());
                    }
                    _ => {}
                }
            }
        }
    }
}

/// C: mj_camlight (engine/engine_core_smooth.h:41)
/// Calls: mj_local2Global, mji_add3, mji_copy3, mji_copy9, mji_cross, mji_rotVecQuat, mji_sub3, mju_normalize3, mju_transpose
#[allow(unused_variables, non_snake_case)]
pub fn mj_camlight(m: *const mjModel, d: *mut mjData) {
    const MJ_ENBL_SLEEP: i32 = 1 << 4;
    const MJ_S_AWAKE: i32 = 1;
    const CAMLIGHT_FIXED: i32 = 0;
    const CAMLIGHT_TRACK: i32 = 1;
    const CAMLIGHT_TRACKCOM: i32 = 2;
    const CAMLIGHT_TARGETBODY: i32 = 3;
    const CAMLIGHT_TARGETBODYCOM: i32 = 4;

    // SAFETY: caller guarantees m, d valid
    unsafe {
        let ncam = (*m).ncam as i32;
        let nlight = (*m).nlight as i32;
        let sleep_filter = ((*m).opt.enableflags & MJ_ENBL_SLEEP) != 0
            && ((*d).nbody_awake as i64) < (*m).nbody;

        // compute Cartesian positions and orientations of cameras
        for i in 0..ncam as usize {
            let id = *(*m).cam_bodyid.add(i);
            let id1 = *(*m).cam_targetbodyid.add(i);

            if sleep_filter && *(*d).body_awake.add(id as usize) != MJ_S_AWAKE {
                if id1 < 0 || *(*d).body_awake.add(id1 as usize) != MJ_S_AWAKE {
                    continue;
                }
            }

            // default processing for fixed mode
            crate::engine::engine_core_util::mj_local2global(
                d, (*d).cam_xpos.add(3 * i), (*d).cam_xmat.add(9 * i),
                (*m).cam_pos.add(3 * i), (*m).cam_quat.add(4 * i), id, 0);

            let mode = *(*m).cam_mode.add(i);
            match mode {
                CAMLIGHT_FIXED => {}
                CAMLIGHT_TRACK | CAMLIGHT_TRACKCOM => {
                    crate::engine::engine_inline::mji_copy9(
                        (*d).cam_xmat.add(9 * i), (*m).cam_mat0.add(9 * i));
                    if mode == CAMLIGHT_TRACK {
                        crate::engine::engine_inline::mji_add3(
                            (*d).cam_xpos.add(3 * i), (*d).xpos.add(3 * id as usize),
                            (*m).cam_pos0.add(3 * i));
                    } else {
                        crate::engine::engine_inline::mji_add3(
                            (*d).cam_xpos.add(3 * i), (*d).subtree_com.add(3 * id as usize),
                            (*m).cam_poscom0.add(3 * i));
                    }
                }
                CAMLIGHT_TARGETBODY | CAMLIGHT_TARGETBODYCOM => {
                    if id1 >= 0 {
                        let mut pos: [f64; 3] = [0.0; 3];
                        if mode == CAMLIGHT_TARGETBODY {
                            crate::engine::engine_inline::mji_copy3(pos.as_mut_ptr(), (*d).xpos.add(3 * id1 as usize));
                        } else {
                            crate::engine::engine_inline::mji_copy3(pos.as_mut_ptr(), (*d).subtree_com.add(3 * id1 as usize));
                        }

                        let mut matT: [f64; 9] = [0.0; 9];
                        crate::engine::engine_inline::mji_sub3(matT.as_mut_ptr().add(6), (*d).cam_xpos.add(3 * i), pos.as_ptr());
                        crate::engine::engine_util_blas::mju_normalize3(matT.as_mut_ptr().add(6));

                        matT[3] = 0.0;
                        matT[4] = 0.0;
                        matT[5] = 1.0;
                        crate::engine::engine_inline::mji_cross(matT.as_mut_ptr(), matT.as_ptr().add(3), matT.as_ptr().add(6));
                        crate::engine::engine_util_blas::mju_normalize3(matT.as_mut_ptr());

                        crate::engine::engine_inline::mji_cross(matT.as_mut_ptr().add(3), matT.as_ptr().add(6), matT.as_ptr());
                        crate::engine::engine_util_blas::mju_normalize3(matT.as_mut_ptr().add(3));

                        crate::engine::engine_util_blas::mju_transpose((*d).cam_xmat.add(9 * i), matT.as_ptr(), 3, 3);
                    }
                }
                _ => {}
            }
        }

        // compute Cartesian positions and directions of lights
        for i in 0..nlight as usize {
            let id = *(*m).light_bodyid.add(i);
            let id1 = *(*m).light_targetbodyid.add(i);

            if sleep_filter && *(*d).body_awake.add(id as usize) != MJ_S_AWAKE {
                if id1 < 0 || *(*d).body_awake.add(id1 as usize) != MJ_S_AWAKE {
                    continue;
                }
            }

            // default processing for fixed mode
            crate::engine::engine_core_util::mj_local2global(
                d, (*d).light_xpos.add(3 * i), std::ptr::null_mut(),
                (*m).light_pos.add(3 * i), std::ptr::null(), id, 0);
            crate::engine::engine_inline::mji_rot_vec_quat(
                (*d).light_xdir.add(3 * i), (*m).light_dir.add(3 * i), (*d).xquat.add(4 * id as usize));

            let mode = *(*m).light_mode.add(i);
            match mode {
                CAMLIGHT_FIXED => {}
                CAMLIGHT_TRACK | CAMLIGHT_TRACKCOM => {
                    crate::engine::engine_inline::mji_copy3(
                        (*d).light_xdir.add(3 * i), (*m).light_dir0.add(3 * i));
                    if mode == CAMLIGHT_TRACK {
                        crate::engine::engine_inline::mji_add3(
                            (*d).light_xpos.add(3 * i), (*d).xpos.add(3 * id as usize),
                            (*m).light_pos0.add(3 * i));
                    } else {
                        crate::engine::engine_inline::mji_add3(
                            (*d).light_xpos.add(3 * i), (*d).subtree_com.add(3 * id as usize),
                            (*m).light_poscom0.add(3 * i));
                    }
                }
                CAMLIGHT_TARGETBODY | CAMLIGHT_TARGETBODYCOM => {
                    if id1 >= 0 {
                        let mut lookat: [f64; 3] = [0.0; 3];
                        if mode == CAMLIGHT_TARGETBODY {
                            crate::engine::engine_inline::mji_copy3(lookat.as_mut_ptr(), (*d).xpos.add(3 * id1 as usize));
                        } else {
                            crate::engine::engine_inline::mji_copy3(lookat.as_mut_ptr(), (*d).subtree_com.add(3 * id1 as usize));
                        }
                        crate::engine::engine_inline::mji_sub3(
                            (*d).light_xdir.add(3 * i), lookat.as_ptr(), (*d).light_xpos.add(3 * i));
                    }
                }
                _ => {}
            }

            // normalize dir
            crate::engine::engine_util_blas::mju_normalize3((*d).light_xdir.add(3 * i));
        }
    }
}

/// C: mj_flex (engine/engine_core_smooth.h:44)
/// Calls: mj_bodyChain, mj_freeStack, mj_jacDifPair, mj_jacSparse, mj_markStack, mj_stackAllocInfo, mj_updateDynamicBVH, mji_addTo3, mji_copy3, mji_copy6, mji_mulMatVec3, mji_sub3, mju_cellLookup, mju_interpolate3D, mju_max, mju_message, mju_min, mju_mulMatMat322, mju_mulMatTVec, mju_mulMatVec, mju_normalize3, mju_scl3, mju_shellTrackInterior, mju_sub3, mju_zero, mju_zero3
#[allow(unused_variables, non_snake_case)]
pub fn mj_flex(m: *const mjModel, d: *mut mjData) {
    const MJ_DSBL_MIDPHASE: i32 = 1 << 13;

    // SAFETY: m, d are valid pointers (caller contract)
    unsafe {
        let nv = (*m).nv as i32;
        let rowadr = (*m).flexedge_J_rowadr;
        let vrowadr = (*m).flexvert_J_rowadr;
        let vrownnz = (*m).flexvert_J_rownnz;

        // skip if no flexes
        if (*m).nflex == 0 {
            return;
        }

        let nflex = (*m).nflex as i32;

        // compute Cartesian positions of flex vertices
        for f in 0..nflex {
            let vstart = *(*m).flex_vertadr.add(f as usize);
            let vend = *(*m).flex_vertadr.add(f as usize) + *(*m).flex_vertnum.add(f as usize);
            let nstart = *(*m).flex_nodeadr.add(f as usize);
            let nend = *(*m).flex_nodeadr.add(f as usize) + *(*m).flex_nodenum.add(f as usize);

            // 0: vertices are the mesh vertices
            if *(*m).flex_interp.add(f as usize) == 0 {
                for i in vstart..vend {
                    if *(*m).flex_centered.add(f as usize)
                        || (*(*m).flex_vert.add((3 * i) as usize) == 0.0
                            && *(*m).flex_vert.add((3 * i + 1) as usize) == 0.0
                            && *(*m).flex_vert.add((3 * i + 2) as usize) == 0.0)
                    {
                        crate::engine::engine_inline::mji_copy3(
                            (*d).flexvert_xpos.add(3 * i as usize),
                            (*d).xpos.add(3 * *(*m).flex_vertbodyid.add(i as usize) as usize));
                    } else {
                        crate::engine::engine_inline::mji_mul_mat_vec3(
                            (*d).flexvert_xpos.add(3 * i as usize),
                            (*d).xmat.add(9 * *(*m).flex_vertbodyid.add(i as usize) as usize),
                            (*m).flex_vert.add(3 * i as usize));
                        crate::engine::engine_inline::mji_add_to3(
                            (*d).flexvert_xpos.add(3 * i as usize),
                            (*d).xpos.add(3 * *(*m).flex_vertbodyid.add(i as usize) as usize));
                    }
                }
            }
            // trilinear/quadratic interpolation
            else {
                let nodenum = nend - nstart;
                crate::engine::engine_memory::mj_mark_stack(d);
                let nodexpos = crate::engine::engine_memory::mj_stack_alloc_num(d, (3 * nodenum) as usize);
                for i in nstart..nend {
                    let j = i - nstart;
                    if *(*m).flex_centered.add(f as usize)
                        || (*(*m).flex_node.add((3 * i) as usize) == 0.0
                            && *(*m).flex_node.add((3 * i + 1) as usize) == 0.0
                            && *(*m).flex_node.add((3 * i + 2) as usize) == 0.0)
                    {
                        crate::engine::engine_inline::mji_copy3(
                            nodexpos.add(3 * j as usize),
                            (*d).xpos.add(3 * *(*m).flex_nodebodyid.add(i as usize) as usize));
                    } else {
                        crate::engine::engine_inline::mji_mul_mat_vec3(
                            nodexpos.add(3 * j as usize),
                            (*d).xmat.add(9 * *(*m).flex_nodebodyid.add(i as usize) as usize),
                            (*m).flex_node.add(3 * i as usize));
                        crate::engine::engine_inline::mji_add_to3(
                            nodexpos.add(3 * j as usize),
                            (*d).xpos.add(3 * *(*m).flex_nodebodyid.add(i as usize) as usize));
                    }
                }

                let interp = *(*m).flex_interp.add(f as usize);
                let order = if interp < 0 { -interp } else { interp };
                let cx = *(*m).flex_cellnum.add((3 * f) as usize);
                let cy = *(*m).flex_cellnum.add((3 * f + 1) as usize);
                let cz = *(*m).flex_cellnum.add((3 * f + 2) as usize);
                let nx_g = cx * order + 1;
                let ny_g = cy * order + 1;
                let nz_g = cz * order + 1;

                // shell mode: reconstruct interior
                if interp < 0 {
                    crate::engine::engine_util_misc::mju_shell_track_interior(
                        nodexpos, nx_g, ny_g, nz_g);
                }

                for i in vstart..vend {
                    crate::engine::engine_util_blas::mju_zero3(
                        (*d).flexvert_xpos.add(3 * i as usize));

                    let mut local = [0.0f64; 3];
                    let mut nodeindices = [0i32; 27];
                    crate::engine::engine_util_misc::mju_cell_lookup(
                        (*m).flex_vert0.add(3 * i as usize),
                        (*m).flex_cellnum.add(3 * f as usize),
                        order, local.as_mut_ptr(), nodeindices.as_mut_ptr());
                    crate::engine::engine_util_misc::mju_interpolate3d(
                        (*d).flexvert_xpos.add(3 * i as usize),
                        local.as_ptr(), nodexpos, order, nodeindices.as_ptr());
                }
                crate::engine::engine_memory::mj_free_stack(d);
            }
        }

        // compute flex element aabb
        for f in 0..nflex {
            let dim = *(*m).flex_dim.add(f as usize);
            let elemnum = *(*m).flex_elemnum.add(f as usize);
            for e in 0..elemnum {
                let edata = (*m).flex_elem.add((*(*m).flex_elemdataadr.add(f as usize) + e * (dim + 1)) as usize);
                let vert = (*d).flexvert_xpos.add(3 * *(*m).flex_vertadr.add(f as usize) as usize);

                let mut xmin = [0.0f64; 3];
                let mut xmax = [0.0f64; 3];
                crate::engine::engine_inline::mji_copy3(xmin.as_mut_ptr(), vert.add(3 * *edata as usize));
                crate::engine::engine_inline::mji_copy3(xmax.as_mut_ptr(), vert.add(3 * *edata as usize));
                for i in 1..=dim {
                    for j in 0..3i32 {
                        let value = *vert.add((3 * *edata.add(i as usize) + j) as usize);
                        xmin[j as usize] = crate::engine::engine_util_misc::mju_min(xmin[j as usize], value);
                        xmax[j as usize] = crate::engine::engine_util_misc::mju_max(xmax[j as usize], value);
                    }
                }

                let base = *(*m).flex_elemadr.add(f as usize) + e;
                *(*d).flexelem_aabb.add((6 * base) as usize) = 0.5 * (xmax[0] + xmin[0]);
                *(*d).flexelem_aabb.add((6 * base + 1) as usize) = 0.5 * (xmax[1] + xmin[1]);
                *(*d).flexelem_aabb.add((6 * base + 2) as usize) = 0.5 * (xmax[2] + xmin[2]);
                *(*d).flexelem_aabb.add((6 * base + 3) as usize) = 0.5 * (xmax[0] - xmin[0]) + *(*m).flex_radius.add(f as usize);
                *(*d).flexelem_aabb.add((6 * base + 4) as usize) = 0.5 * (xmax[1] - xmin[1]) + *(*m).flex_radius.add(f as usize);
                *(*d).flexelem_aabb.add((6 * base + 5) as usize) = 0.5 * (xmax[2] - xmin[2]) + *(*m).flex_radius.add(f as usize);
            }
        }

        // update flex bvh_aabb_dyn if needed
        if ((*m).opt.disableflags & MJ_DSBL_MIDPHASE) == 0 {
            for f in 0..nflex {
                if *(*m).flex_bvhadr.add(f as usize) >= 0 {
                    let flex_bvhadr = *(*m).flex_bvhadr.add(f as usize);
                    let flex_bvhnum = *(*m).flex_bvhnum.add(f as usize);

                    for i in flex_bvhadr..(flex_bvhadr + flex_bvhnum) {
                        if *(*m).bvh_nodeid.add(i as usize) >= 0 {
                            crate::engine::engine_inline::mji_copy6(
                                (*d).bvh_aabb_dyn.add(6 * (i - (*m).nbvhstatic as i32) as usize),
                                (*d).flexelem_aabb.add(6 * (*(*m).flex_elemadr.add(f as usize) + *(*m).bvh_nodeid.add(i as usize)) as usize));
                        }
                    }

                    mj_update_dynamic_bvh(m, d, flex_bvhadr, flex_bvhnum);
                }
            }
        }

        // allocate space
        crate::engine::engine_memory::mj_mark_stack(d);
        let jac1 = crate::engine::engine_memory::mj_stack_alloc_num(d, (3 * nv) as usize);
        let jac2 = crate::engine::engine_memory::mj_stack_alloc_num(d, (3 * nv) as usize);
        let jacdif = crate::engine::engine_memory::mj_stack_alloc_num(d, (3 * nv) as usize);
        let chain = crate::engine::engine_memory::mj_stack_alloc_int(d, nv as usize);

        // clear Jacobians
        crate::engine::engine_util_blas::mju_zero((*d).flexvert_J, 2 * (*m).nJfv as i32);
        crate::engine::engine_util_blas::mju_zero((*d).flexedge_J, (*m).nJfe as i32);

        // compute lengths and Jacobians of edges
        for f in 0..nflex {
            // skip if edges cannot generate forces
            if *(*m).flex_rigid.add(f as usize) || *(*m).flex_interp.add(f as usize) != 0 {
                continue;
            }

            // skip edge Jacobian if no built-in passive force is needed
            let skipjacobian = *(*m).flex_edgeequality.add(f as usize) != 1
                && *(*m).flex_edgedamping.add(f as usize) == 0.0
                && *(*m).flex_edgestiffness.add(f as usize) == 0.0
                && *(*m).flex_damping.add(f as usize) == 0.0;

            // process edges
            let vbase = *(*m).flex_vertadr.add(f as usize);
            let ebase = *(*m).flex_edgeadr.add(f as usize);
            let edgenum = *(*m).flex_edgenum.add(f as usize);
            for e in 0..edgenum {
                let v1 = *(*m).flex_edge.add((2 * (ebase + e)) as usize);
                let v2 = *(*m).flex_edge.add((2 * (ebase + e) + 1) as usize);
                let b1 = *(*m).flex_vertbodyid.add((vbase + v1) as usize);
                let b2 = *(*m).flex_vertbodyid.add((vbase + v2) as usize);
                let pos1 = (*d).flexvert_xpos.add(3 * (vbase + v1) as usize);
                let pos2 = (*d).flexvert_xpos.add(3 * (vbase + v2) as usize);

                // vec = unit vector from v1 to v2, compute edge length
                let mut vec = [0.0f64; 3];
                crate::engine::engine_inline::mji_sub3(vec.as_mut_ptr(), pos2, pos1);
                *(*d).flexedge_length.add((ebase + e) as usize) =
                    crate::engine::engine_util_blas::mju_normalize3(vec.as_mut_ptr());

                if skipjacobian {
                    continue;
                }

                // get endpoint Jacobians, subtract
                let NV = crate::engine::engine_core_util::mj_jac_dif_pair(
                    m, d as *const mjData, chain, b1, b2, pos1, pos2,
                    jac1, jac2, jacdif,
                    std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut(),
                    1, 0);

                if NV == 0 {
                    continue;
                }

                // apply chain rule to compute edge Jacobian
                crate::engine::engine_util_blas::mju_mul_mat_t_vec(
                    (*d).flexedge_J.add(*rowadr.add((ebase + e) as usize) as usize),
                    jacdif, vec.as_ptr(), 3, NV);
            }

            // dim=2 vertex-based constraint (Chen, Kry, Vouga 2019)
            if *(*m).flex_dim.add(f as usize) == 2 && *(*m).flex_edgeequality.add(f as usize) == 2 {
                let nvert = *(*m).flex_vertnum.add(f as usize);
                let v_edge_cnt = (*m).flex_vertedgenum.add(vbase as usize);
                let v_edge_adr = (*m).flex_vertedgeadr.add(vbase as usize);
                let adj_edges = (*m).flex_vertedge;

                crate::engine::engine_memory::mj_mark_stack(d);
                let chain1 = crate::engine::engine_memory::mj_stack_alloc_int(d, nv as usize);
                let chain2 = crate::engine::engine_memory::mj_stack_alloc_int(d, nv as usize);
                let J0_dense = crate::engine::engine_memory::mj_stack_alloc_num(d, nv as usize);
                let J1_dense = crate::engine::engine_memory::mj_stack_alloc_num(d, nv as usize);
                crate::engine::engine_util_blas::mju_zero(J0_dense, nv);
                crate::engine::engine_util_blas::mju_zero(J1_dense, nv);
                let J_local = crate::engine::engine_memory::mj_stack_alloc_num(d, nv as usize);

                for v in 0..nvert {
                    let mut A = [0.0f64; 6];
                    let vadr = vbase + v;
                    let metric = (*m).flex_vertmetric.add(4 * vadr as usize);

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

                        let mut dy = [0.0f64; 3];
                        crate::engine::engine_util_blas::mju_sub3(dy.as_mut_ptr(),
                            (*d).flexvert_xpos.add(3 * (vbase + ev2) as usize),
                            (*d).flexvert_xpos.add(3 * (vbase + ev1) as usize));

                        let neighbor_v = if v == ev1 { ev2 } else { ev1 };
                        let b_neighbor = *(*m).flex_vertbodyid.add((vbase + neighbor_v) as usize);
                        let mut weight: f64 = 1.0;
                        if b_neighbor >= 0 {
                            weight = *(*m).body_mass.add(b_neighbor as usize);
                            if weight < 1e-14 { weight = 1e-14; }
                        }

                        A[0] += weight * dy[0] * dx[0];
                        A[1] += weight * dy[0] * dx[1];
                        A[2] += weight * dy[1] * dx[0];
                        A[3] += weight * dy[1] * dx[1];
                        A[4] += weight * dy[2] * dx[0];
                        A[5] += weight * dy[2] * dx[1];
                    }

                    let mut F = [0.0f64; 6];
                    mju_mul_mat_mat322(F.as_mut_ptr(), A.as_ptr(), metric);

                    // Cauchy strain tensor F^T F
                    let mut cauchy = [0.0f64; 4];
                    cauchy[0] = F[0] * F[0] + F[2] * F[2] + F[4] * F[4];
                    cauchy[1] = F[0] * F[1] + F[2] * F[3] + F[4] * F[5];
                    cauchy[2] = F[1] * F[0] + F[3] * F[2] + F[5] * F[4];
                    cauchy[3] = F[1] * F[1] + F[3] * F[3] + F[5] * F[5];

                    // mass scaling
                    let mut scale: f64 = 1.0;
                    let b = *(*m).flex_vertbodyid.add(vadr as usize);
                    if b >= 0 {
                        let mass = *(*m).body_mass.add(b as usize);
                        if mass > 1e-14 {
                            scale = f64::sqrt(mass);
                        }
                    }

                    // tensor invariants
                    *(*d).flexvert_length.add((2 * vadr) as usize) = (cauchy[0] + cauchy[3] - 2.0) * scale;
                    *(*d).flexvert_length.add((2 * vadr + 1) as usize) =
                        (cauchy[0] * cauchy[3] - cauchy[1] * cauchy[2] - 1.0) * scale;

                    // Jacobian computation
                    let mut FB = [0.0f64; 6];
                    mju_mul_mat_mat322(FB.as_mut_ptr(), F.as_ptr(), metric);
                    let adj = [cauchy[3], -cauchy[1], -cauchy[2], cauchy[0]];
                    let mut Fadj = [0.0f64; 6];
                    mju_mul_mat_mat322(Fadj.as_mut_ptr(), F.as_ptr(), adj.as_ptr());
                    let mut FadjBinv = [0.0f64; 6];
                    mju_mul_mat_mat322(FadjBinv.as_mut_ptr(), Fadj.as_ptr(), metric);

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
                            if weight < 1e-14 { weight = 1e-14; }
                        }

                        let mut dI1dy1 = [0.0f64; 3];
                        let mut dI1dy2 = [0.0f64; 3];
                        let mut dI2dy = [0.0f64; 3];
                        let mut dI2dy1 = [0.0f64; 3];
                        let mut dI2dy2 = [0.0f64; 3];

                        crate::engine::engine_util_blas::mju_mul_mat_vec(dI1dy1.as_mut_ptr(), FB.as_ptr(), dx.as_ptr(), 3, 2);
                        crate::engine::engine_util_blas::mju_scl3(dI1dy1.as_mut_ptr(), dI1dy1.as_ptr(), -2.0 * weight);
                        crate::engine::engine_util_blas::mju_scl3(dI1dy2.as_mut_ptr(), dI1dy1.as_ptr(), -1.0);

                        crate::engine::engine_util_blas::mju_mul_mat_vec(dI2dy.as_mut_ptr(), FadjBinv.as_ptr(), dx.as_ptr(), 3, 2);
                        crate::engine::engine_util_blas::mju_scl3(dI2dy1.as_mut_ptr(), dI2dy.as_ptr(), -2.0 * weight);
                        crate::engine::engine_util_blas::mju_scl3(dI2dy2.as_mut_ptr(), dI2dy1.as_ptr(), -1.0);

                        // get endpoint Jacobians
                        let eb1 = *(*m).flex_vertbodyid.add((vbase + ev1) as usize);
                        let eb2 = *(*m).flex_vertbodyid.add((vbase + ev2) as usize);
                        let NV1 = crate::engine::engine_core_util::mj_body_chain(m, eb1, chain1);
                        crate::engine::engine_core_util::mj_jac_sparse(
                            m, d as *const mjData, jac1, std::ptr::null_mut(),
                            (*d).flexvert_xpos.add(3 * (vbase + ev1) as usize),
                            eb1, NV1, chain1, 0);
                        let NV2 = crate::engine::engine_core_util::mj_body_chain(m, eb2, chain2);
                        crate::engine::engine_core_util::mj_jac_sparse(
                            m, d as *const mjData, jac2, std::ptr::null_mut(),
                            (*d).flexvert_xpos.add(3 * (vbase + ev2) as usize),
                            eb2, NV2, chain2, 0);

                        // accumulate dense Jacobians
                        crate::engine::engine_util_blas::mju_mul_mat_t_vec(J_local, jac1, dI1dy1.as_ptr(), 3, NV1);
                        for jj in 0..NV1 { *J0_dense.add(*chain1.add(jj as usize) as usize) += *J_local.add(jj as usize); }
                        crate::engine::engine_util_blas::mju_mul_mat_t_vec(J_local, jac2, dI1dy2.as_ptr(), 3, NV2);
                        for jj in 0..NV2 { *J0_dense.add(*chain2.add(jj as usize) as usize) += *J_local.add(jj as usize); }

                        crate::engine::engine_util_blas::mju_mul_mat_t_vec(J_local, jac1, dI2dy1.as_ptr(), 3, NV1);
                        for jj in 0..NV1 { *J1_dense.add(*chain1.add(jj as usize) as usize) += *J_local.add(jj as usize); }
                        crate::engine::engine_util_blas::mju_mul_mat_t_vec(J_local, jac2, dI2dy2.as_ptr(), 3, NV2);
                        for jj in 0..NV2 { *J1_dense.add(*chain2.add(jj as usize) as usize) += *J_local.add(jj as usize); }
                    }

                    // copy to sparse flexvert_J
                    let row0 = 2 * vadr;
                    let nnz0 = *vrownnz.add(row0 as usize);
                    for jj in 0..nnz0 {
                        let col = *(*m).flexvert_J_colind.add((*vrowadr.add(row0 as usize) + jj) as usize);
                        *(*d).flexvert_J.add((*vrowadr.add(row0 as usize) + jj) as usize) += *J0_dense.add(col as usize) * scale;
                        *J0_dense.add(col as usize) = 0.0;
                    }
                    let row1 = 2 * vadr + 1;
                    let nnz1 = *vrownnz.add(row1 as usize);
                    for jj in 0..nnz1 {
                        let col = *(*m).flexvert_J_colind.add((*vrowadr.add(row1 as usize) + jj) as usize);
                        *(*d).flexvert_J.add((*vrowadr.add(row1 as usize) + jj) as usize) += *J1_dense.add(col as usize) * scale;
                        *J1_dense.add(col as usize) = 0.0;
                    }
                }

                crate::engine::engine_memory::mj_free_stack(d);
            }
        }

        crate::engine::engine_memory::mj_free_stack(d);
    }
}

/// C: mj_tendon (engine/engine_core_smooth.h:47)
/// Calls: mj_freeStack, mj_jacDifPair, mj_markStack, mj_sleepState, mj_stackAllocInfo, mji_copy3, mji_copy9, mji_sub3, mju_combineSparseInc, mju_dist3, mju_message, mju_mulMatTVec, mju_normalize3, mju_round, mju_wrap, mju_zero, mju_zero3
#[allow(unused_variables, non_snake_case)]
pub fn mj_tendon(m: *const mjModel, d: *mut mjData) {
    const MJ_ENBL_SLEEP: i32 = 1 << 4;
    const MJ_OBJ_TENDON: u32 = 18;
    const MJ_S_ASLEEP: i32 = 2;

    // SAFETY: m, d are valid pointers (caller contract)
    unsafe {
        let nv = (*m).nv as i32;
        let nten = (*m).ntendon as i32;
        let rownnz = (*m).ten_J_rownnz;
        let rowadr = (*m).ten_J_rowadr;
        let colind = (*m).ten_J_colind;
        let L = (*d).ten_length;
        let J = (*d).ten_J;

        if nten == 0 {
            return;
        }

        // allocate stack arrays
        crate::engine::engine_memory::mj_mark_stack(d);
        let jac1 = crate::engine::engine_memory::mj_stack_alloc_num(d, (3 * nv) as usize);
        let jac2 = crate::engine::engine_memory::mj_stack_alloc_num(d, (3 * nv) as usize);
        let jacdif = crate::engine::engine_memory::mj_stack_alloc_num(d, (3 * nv) as usize);
        let tmp = crate::engine::engine_memory::mj_stack_alloc_num(d, nv as usize);
        let chain = crate::engine::engine_memory::mj_stack_alloc_int(d, nv as usize);

        // clear results
        crate::engine::engine_util_blas::mju_zero(L, nten);

        // clear Jacobian
        crate::engine::engine_util_blas::mju_zero(J, (*m).nJten as i32);

        // sleep filtering
        let sleep_filter = ((*m).opt.enableflags & MJ_ENBL_SLEEP) != 0
            && (*d).ntree_awake < (*m).ntree as i32;

        // loop over tendons
        let mut wrapcount: i32 = 0;
        for i in 0..nten {
            // skip sleeping tendon
            if sleep_filter && crate::engine::engine_sleep::mj_sleep_state(
                m, d as *const mjData, MJ_OBJ_TENDON, i) == MJ_S_ASLEEP {
                continue;
            }

            // initialize tendon path
            let adr = *(*m).tendon_adr.add(i as usize);
            *(*d).ten_wrapadr.add(i as usize) = wrapcount;
            *(*d).ten_wrapnum.add(i as usize) = 0;
            let tendon_num = *(*m).tendon_num.add(i as usize);

            // process fixed tendon
            if *(*m).wrap_type.add(adr as usize) == mjtWrap_mjWRAP_JOINT as i32 {
                // process all defined joints
                for j in 0..tendon_num {
                    // get joint id
                    let k = *(*m).wrap_objid.add((adr + j) as usize);

                    // add to length
                    *L.add(i as usize) += *(*m).wrap_prm.add((adr + j) as usize)
                        * *(*d).qpos.add(*(*m).jnt_qposadr.add(k as usize) as usize);

                    let coef: f64 = 1.0;
                    let dofadr = *(*m).jnt_dofadr.add(k as usize);
                    crate::engine::engine_util_sparse::mju_combine_sparse_inc(
                        J.add(*rowadr.add(i as usize) as usize),
                        &coef, (*m).nv as i32, 1.0,
                        *(*m).wrap_prm.add((adr + j) as usize),
                        *rownnz.add(i as usize), 1,
                        colind.add(*rowadr.add(i as usize) as usize),
                        &dofadr);
                }
                continue;
            }

            // process spatial tendon
            let mut divisor: f64 = 1.0;
            let mut j: i32 = 0;
            while j < tendon_num - 1 {
                // get 1st and 2nd object
                let type0 = *(*m).wrap_type.add((adr + j) as usize);
                let type1 = *(*m).wrap_type.add((adr + j + 1) as usize);
                let id0 = *(*m).wrap_objid.add((adr + j) as usize);
                let id1 = *(*m).wrap_objid.add((adr + j + 1) as usize);

                // pulley
                if type0 == mjtWrap_mjWRAP_PULLEY as i32 || type1 == mjtWrap_mjWRAP_PULLEY as i32 {
                    if type0 == mjtWrap_mjWRAP_PULLEY as i32 {
                        divisor = *(*m).wrap_prm.add((adr + j) as usize);
                        crate::engine::engine_inline::mji_zero3((*d).wrap_xpos.add((wrapcount * 3) as usize));
                        *(*d).wrap_obj.add(wrapcount as usize) = -2;
                        *(*d).ten_wrapnum.add(i as usize) += 1;
                        wrapcount += 1;
                    }
                    j += 1;
                    continue;
                }

                // init sequence; assume it starts with site
                let mut wlen: f64 = -1.0;
                let mut wrapid: i32 = -1;
                let mut wpnt = [0.0f64; 12];
                crate::engine::engine_inline::mji_copy3(wpnt.as_mut_ptr(),
                    (*d).site_xpos.add(3 * id0 as usize));
                let mut wbody = [0i32; 4];
                wbody[0] = *(*m).site_bodyid.add(id0 as usize);

                // second object is geom: process site-geom-site
                let wraptype;
                if type1 == mjtWrap_mjWRAP_SPHERE as i32 || type1 == mjtWrap_mjWRAP_CYLINDER as i32 {
                    wraptype = type1;
                    wrapid = id1;
                    let type1_next = *(*m).wrap_type.add((adr + j + 2) as usize);
                    let id1_next = *(*m).wrap_objid.add((adr + j + 2) as usize);

                    // do wrapping
                    let sideid = crate::engine::engine_util_misc::mju_round(
                        *(*m).wrap_prm.add((adr + j + 1) as usize));

                    let side_ptr = if sideid >= 0 {
                        (*d).site_xpos.add(3 * sideid as usize)
                    } else {
                        std::ptr::null()
                    };

                    wlen = crate::engine::engine_util_misc::mju_wrap(
                        wpnt.as_mut_ptr().add(3),
                        (*d).site_xpos.add(3 * id0 as usize),
                        (*d).site_xpos.add(3 * id1_next as usize),
                        (*d).geom_xpos.add(3 * wrapid as usize),
                        (*d).geom_xmat.add(9 * wrapid as usize),
                        *(*m).geom_size.add(3 * wrapid as usize),
                        wraptype, side_ptr);

                    // use id1_next as the endpoint
                    let id1_actual = id1_next;

                    // complete sequence
                    if wlen < 0.0 {
                        crate::engine::engine_inline::mji_copy3(wpnt.as_mut_ptr().add(3),
                            (*d).site_xpos.add(3 * id1_actual as usize));
                        wbody[1] = *(*m).site_bodyid.add(id1_actual as usize);
                        *L.add(i as usize) += crate::engine::engine_util_blas::mju_dist3(
                            wpnt.as_ptr(), wpnt.as_ptr().add(3)) / divisor;
                    } else {
                        crate::engine::engine_inline::mji_copy3(wpnt.as_mut_ptr().add(9),
                            (*d).site_xpos.add(3 * id1_actual as usize));
                        wbody[1] = *(*m).geom_bodyid.add(wrapid as usize);
                        wbody[2] = *(*m).geom_bodyid.add(wrapid as usize);
                        wbody[3] = *(*m).site_bodyid.add(id1_actual as usize);
                        *L.add(i as usize) += (crate::engine::engine_util_blas::mju_dist3(
                            wpnt.as_ptr(), wpnt.as_ptr().add(3))
                            + wlen
                            + crate::engine::engine_util_blas::mju_dist3(
                                wpnt.as_ptr().add(6), wpnt.as_ptr().add(9))) / divisor;
                    }

                    // accumulate moments
                    let nseg = if wlen < 0.0 { 1 } else { 3 };
                    for k in 0..nseg {
                        if wbody[k as usize] != wbody[(k + 1) as usize] {
                            let mut dif = [0.0f64; 3];
                            crate::engine::engine_inline::mji_sub3(dif.as_mut_ptr(),
                                wpnt.as_ptr().add((3 * k + 3) as usize),
                                wpnt.as_ptr().add((3 * k) as usize));
                            crate::engine::engine_util_blas::mju_normalize3(dif.as_mut_ptr());

                            let NV = crate::engine::engine_core_util::mj_jac_dif_pair(
                                m, d as *const mjData, chain,
                                wbody[k as usize], wbody[(k + 1) as usize],
                                wpnt.as_ptr().add((3 * k) as usize),
                                wpnt.as_ptr().add((3 * k + 3) as usize),
                                jac1, jac2, jacdif, std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut(),
                                1, 0);

                            if NV == 0 {
                                continue;
                            }

                            crate::engine::engine_util_blas::mju_mul_mat_t_vec(
                                tmp, jacdif, dif.as_ptr(), 3, NV);

                            crate::engine::engine_util_sparse::mju_combine_sparse_inc(
                                J.add(*rowadr.add(i as usize) as usize),
                                tmp, nv, 1.0, 1.0 / divisor,
                                *rownnz.add(i as usize), NV,
                                colind.add(*rowadr.add(i as usize) as usize),
                                chain);
                        }
                    }

                    // assign to wrap
                    if wlen < 0.0 {
                        crate::engine::engine_inline::mji_copy3(
                            (*d).wrap_xpos.add((wrapcount * 3) as usize), wpnt.as_ptr());
                    } else {
                        crate::engine::engine_inline::mji_copy9(
                            (*d).wrap_xpos.add((wrapcount * 3) as usize), wpnt.as_ptr());
                    }
                    *(*d).wrap_obj.add(wrapcount as usize) = -1;
                    if wlen >= 0.0 {
                        *(*d).wrap_obj.add((wrapcount + 1) as usize) = wrapid;
                        *(*d).wrap_obj.add((wrapcount + 2) as usize) = wrapid;
                    }
                    *(*d).ten_wrapnum.add(i as usize) += if wlen < 0.0 { 1 } else { 3 };
                    wrapcount += if wlen < 0.0 { 1 } else { 3 };

                    // advance
                    j += 3;  // skip site-geom-site

                    // assign last site before pulley or tendon end
                    if j == tendon_num - 1 || *(*m).wrap_type.add((adr + j + 1) as usize) == mjtWrap_mjWRAP_PULLEY as i32 {
                        crate::engine::engine_inline::mji_copy3(
                            (*d).wrap_xpos.add((wrapcount * 3) as usize),
                            (*d).site_xpos.add(3 * id1_actual as usize));
                        *(*d).wrap_obj.add(wrapcount as usize) = -1;
                        *(*d).ten_wrapnum.add(i as usize) += 1;
                        wrapcount += 1;
                    }
                } else {
                    wraptype = mjtWrap_mjWRAP_NONE as i32;

                    // complete sequence (no wrap)
                    crate::engine::engine_inline::mji_copy3(wpnt.as_mut_ptr().add(3),
                        (*d).site_xpos.add(3 * id1 as usize));
                    wbody[1] = *(*m).site_bodyid.add(id1 as usize);
                    *L.add(i as usize) += crate::engine::engine_util_blas::mju_dist3(
                        wpnt.as_ptr(), wpnt.as_ptr().add(3)) / divisor;

                    // accumulate moments
                    if wbody[0] != wbody[1] {
                        let mut dif = [0.0f64; 3];
                        crate::engine::engine_inline::mji_sub3(dif.as_mut_ptr(),
                            wpnt.as_ptr().add(3), wpnt.as_ptr());
                        crate::engine::engine_util_blas::mju_normalize3(dif.as_mut_ptr());

                        let NV = crate::engine::engine_core_util::mj_jac_dif_pair(
                            m, d as *const mjData, chain,
                            wbody[0], wbody[1],
                            wpnt.as_ptr(), wpnt.as_ptr().add(3),
                            jac1, jac2, jacdif, std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut(),
                            1, 0);

                        if NV > 0 {
                            crate::engine::engine_util_blas::mju_mul_mat_t_vec(
                                tmp, jacdif, dif.as_ptr(), 3, NV);

                            crate::engine::engine_util_sparse::mju_combine_sparse_inc(
                                J.add(*rowadr.add(i as usize) as usize),
                                tmp, nv, 1.0, 1.0 / divisor,
                                *rownnz.add(i as usize), NV,
                                colind.add(*rowadr.add(i as usize) as usize),
                                chain);
                        }
                    }

                    // assign to wrap
                    crate::engine::engine_inline::mji_copy3(
                        (*d).wrap_xpos.add((wrapcount * 3) as usize), wpnt.as_ptr());
                    *(*d).wrap_obj.add(wrapcount as usize) = -1;
                    *(*d).ten_wrapnum.add(i as usize) += 1;
                    wrapcount += 1;

                    // advance
                    j += 1;

                    // assign last site before pulley or tendon end
                    if j == tendon_num - 1 || *(*m).wrap_type.add((adr + j + 1) as usize) == mjtWrap_mjWRAP_PULLEY as i32 {
                        crate::engine::engine_inline::mji_copy3(
                            (*d).wrap_xpos.add((wrapcount * 3) as usize),
                            (*d).site_xpos.add(3 * id1 as usize));
                        *(*d).wrap_obj.add(wrapcount as usize) = -1;
                        *(*d).ten_wrapnum.add(i as usize) += 1;
                        wrapcount += 1;
                    }
                }
            }
        }

        crate::engine::engine_memory::mj_free_stack(d);
    }
}

/// C: mj_tendonDot (engine/engine_core_smooth.h:50)
/// Calls: mj_freeStack, mj_isSparse, mj_jac, mj_jacDot, mj_jacDotSparse, mj_jacSparse, mj_markStack, mj_mergeChain, mj_objectVelocity, mj_stackAllocInfo, mji_copy3, mju_addToScl3, mju_dot, mju_dot3, mju_message, mju_mulMatTVec, mju_normalize3, mju_scl3, mju_sub, mju_sub3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_tendon_dot(m: *const mjModel, d: *mut mjData, id: i32, vec: *const f64) -> f64 {
    const MJWRAP_JOINT: i32 = 1;
    const MJWRAP_PULLEY: i32 = 2;
    const MJWRAP_SPHERE: i32 = 4;
    const MJWRAP_CYLINDER: i32 = 5;
    const MJWRAP_NONE: i32 = 0;
    const MJOBJ_SITE: i32 = 6;
    const MJMINVAL: f64 = 1e-15;

    // SAFETY: m, d, vec are valid pointers (caller contract).
    unsafe {
        let nv = (*m).nv as i32;
        let mut res: f64 = 0.0;

        // tendon id is invalid: return
        if id < 0 || id >= (*m).ntendon as i32 {
            return 0.0;
        }

        // fixed tendon has zero Jdot: return
        let adr = *(*m).tendon_adr.add(id as usize);
        if *(*m).wrap_type.add(adr as usize) == MJWRAP_JOINT {
            return 0.0;
        }

        // allocate stack arrays
        crate::engine::engine_memory::mj_mark_stack(d);
        let issparse = crate::engine::engine_core_util::mj_is_sparse(m);
        let chain = if issparse != 0 {
            crate::engine::engine_memory::mj_stack_alloc_int(d, nv as usize)
        } else {
            std::ptr::null_mut()
        };
        let jac1 = crate::engine::engine_memory::mj_stack_alloc_num(d, (3 * nv) as usize);
        let jac2 = crate::engine::engine_memory::mj_stack_alloc_num(d, (3 * nv) as usize);
        let jacdif = crate::engine::engine_memory::mj_stack_alloc_num(d, (3 * nv) as usize);
        let tmp = crate::engine::engine_memory::mj_stack_alloc_num(d, nv as usize);

        // process spatial tendon
        let mut divisor: f64 = 1.0;
        let mut j: i32 = 0;
        let num = *(*m).tendon_num.add(id as usize);
        while j < num - 1 {
            // get 1st and 2nd object
            let type0 = *(*m).wrap_type.add((adr + j) as usize);
            let type1 = *(*m).wrap_type.add((adr + j + 1) as usize);
            let id0 = *(*m).wrap_objid.add((adr + j) as usize);
            let id1 = *(*m).wrap_objid.add((adr + j + 1) as usize);

            // pulley
            if type0 == MJWRAP_PULLEY || type1 == MJWRAP_PULLEY {
                if type0 == MJWRAP_PULLEY {
                    divisor = *(*m).wrap_prm.add((adr + j) as usize);
                }
                j += 1;
                continue;
            }

            // init sequence; assume it starts with site
            let mut wpnt: [f64; 6] = [0.0; 6];
            crate::engine::engine_inline::mji_copy3(
                wpnt.as_mut_ptr(), (*d).site_xpos.add(3 * id0 as usize));
            let mut vel: [f64; 6] = [0.0; 6];
            crate::engine::engine_core_util::mj_object_velocity(
                m, d as *const mjData, MJOBJ_SITE, id0, vel.as_mut_ptr(), 0);
            let mut wvel: [f64; 6] = [vel[3], vel[4], vel[5], 0.0, 0.0, 0.0];
            let mut wbody: [i32; 2] = [0; 2];
            wbody[0] = *(*m).site_bodyid.add(id0 as usize);

            // second object is geom: not supported (matches C: mjERROR)
            let wraptype: i32;
            if type1 == MJWRAP_SPHERE || type1 == MJWRAP_CYLINDER {
                crate::engine::engine_memory::mj_free_stack(d);
                panic!("geom wrapping not supported in mj_tendonDot");
            } else {
                wraptype = MJWRAP_NONE;
            }

            // complete sequence
            wbody[1] = *(*m).site_bodyid.add(id1 as usize);
            crate::engine::engine_inline::mji_copy3(
                wpnt.as_mut_ptr().add(3), (*d).site_xpos.add(3 * id1 as usize));
            crate::engine::engine_core_util::mj_object_velocity(
                m, d as *const mjData, MJOBJ_SITE, id1, vel.as_mut_ptr(), 0);
            crate::engine::engine_inline::mji_copy3(wvel.as_mut_ptr().add(3), vel.as_ptr().add(3));

            // accumulate moments if consecutive points are in different bodies
            if wbody[0] != wbody[1] {
                // dpnt = 3D position difference, normalize
                let mut dpnt: [f64; 3] = [0.0; 3];
                crate::engine::engine_util_blas::mju_sub3(
                    dpnt.as_mut_ptr(), wpnt.as_ptr().add(3), wpnt.as_ptr());
                let norm = crate::engine::engine_util_blas::mju_normalize3(dpnt.as_mut_ptr());

                // dvel = d / dt (dpnt)
                let mut dvel: [f64; 3] = [0.0; 3];
                crate::engine::engine_util_blas::mju_sub3(
                    dvel.as_mut_ptr(), wvel.as_ptr().add(3), wvel.as_ptr());
                let dot = crate::engine::engine_util_blas::mju_dot3(
                    dpnt.as_ptr(), dvel.as_ptr());
                crate::engine::engine_util_blas::mju_add_to_scl3(
                    dvel.as_mut_ptr(), dpnt.as_ptr(), -dot);
                let scale = if norm > MJMINVAL { 1.0 / norm } else { 0.0 };
                crate::engine::engine_util_blas::mju_scl3(
                    dvel.as_mut_ptr(), dvel.as_ptr(), scale);

                if issparse != 0 {
                    // construct merged chain
                    let NV = crate::engine::engine_core_util::mj_merge_chain(
                        m, chain, wbody[0], wbody[1], 0);

                    if NV > 0 {
                        // get endpoint JacobianDots, subtract
                        crate::engine::engine_core_util::mj_jac_dot_sparse(
                            m, d as *const mjData, jac1, std::ptr::null_mut(),
                            wpnt.as_ptr(), wbody[0], NV, chain);
                        crate::engine::engine_core_util::mj_jac_dot_sparse(
                            m, d as *const mjData, jac2, std::ptr::null_mut(),
                            wpnt.as_ptr().add(3), wbody[1], NV, chain);
                        crate::engine::engine_util_blas::mju_sub(
                            jacdif, jac2, jac1, 3 * NV);

                        // chain rule, first term
                        crate::engine::engine_util_blas::mju_mul_mat_t_vec(
                            tmp, jacdif, dpnt.as_ptr(), 3, NV);
                        for k in 0..NV {
                            res += (*tmp.add(k as usize) / divisor)
                                * *vec.add(*chain.add(k as usize) as usize);
                        }

                        // get endpoint Jacobians, subtract
                        crate::engine::engine_core_util::mj_jac_sparse(
                            m, d as *const mjData, jac1, std::ptr::null_mut(),
                            wpnt.as_ptr(), wbody[0], NV, chain, 0);
                        crate::engine::engine_core_util::mj_jac_sparse(
                            m, d as *const mjData, jac2, std::ptr::null_mut(),
                            wpnt.as_ptr().add(3), wbody[1], NV, chain, 0);
                        crate::engine::engine_util_blas::mju_sub(
                            jacdif, jac2, jac1, 3 * NV);

                        // chain rule, second term
                        crate::engine::engine_util_blas::mju_mul_mat_t_vec(
                            tmp, jacdif, dvel.as_ptr(), 3, NV);
                        for k in 0..NV {
                            res += (*tmp.add(k as usize) / divisor)
                                * *vec.add(*chain.add(k as usize) as usize);
                        }
                    }
                } else {
                    // dense: get endpoint JacobianDots, subtract
                    crate::engine::engine_core_util::mj_jac_dot(
                        m, d as *const mjData, jac1, std::ptr::null_mut(),
                        wpnt.as_ptr(), wbody[0]);
                    crate::engine::engine_core_util::mj_jac_dot(
                        m, d as *const mjData, jac2, std::ptr::null_mut(),
                        wpnt.as_ptr().add(3), wbody[1]);
                    crate::engine::engine_util_blas::mju_sub(jacdif, jac2, jac1, 3 * nv);

                    // chain rule, first term
                    crate::engine::engine_util_blas::mju_mul_mat_t_vec(
                        tmp, jacdif, dpnt.as_ptr(), 3, nv);
                    res += crate::engine::engine_util_blas::mju_dot(tmp, vec, nv) / divisor;

                    // get endpoint Jacobians, subtract
                    crate::engine::engine_core_util::mj_jac(
                        m, d as *const mjData, jac1, std::ptr::null_mut(),
                        wpnt.as_ptr(), wbody[0]);
                    crate::engine::engine_core_util::mj_jac(
                        m, d as *const mjData, jac2, std::ptr::null_mut(),
                        wpnt.as_ptr().add(3), wbody[1]);
                    crate::engine::engine_util_blas::mju_sub(jacdif, jac2, jac1, 3 * nv);

                    // chain rule, second term
                    crate::engine::engine_util_blas::mju_mul_mat_t_vec(
                        tmp, jacdif, dvel.as_ptr(), 3, nv);
                    res += crate::engine::engine_util_blas::mju_dot(tmp, vec, nv) / divisor;
                }
            }

            // advance
            j += if wraptype != MJWRAP_NONE { 2 } else { 1 };
        }

        crate::engine::engine_memory::mj_free_stack(d);
        res
    }
}

/// C: mj_transmission (engine/engine_core_smooth.h:53)
/// Calls: mj_freeStack, mj_isSparse, mj_jacDifPair, mj_jacPointAxis, mj_jacSite, mj_markStack, mj_mulJacTVec, mj_sleepState, mj_stackAllocInfo, mji_addTo3, mji_copy3, mji_copy4, mji_mulMatVec3, mji_mulQuat, mji_quat2Vel, mji_rotVecQuat, mji_subQuat, mju_addTo, mju_copyInt, mju_dot3, mju_isZero, mju_message, mju_mulMatMat, mju_mulMatTVec, mju_mulMatTVec3, mju_negQuat, mju_normalize4, mju_scl, mju_scl3, mju_sub3, mju_subFrom, mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mj_transmission(m: *const mjModel, d: *mut mjData) {
    const MJ_ENBL_SLEEP: i32 = 1 << 4;
    const MJ_OBJ_ACTUATOR: u32 = 19;
    const MJ_S_ASLEEP: i32 = 2;
    const MJ_CONE_ELLIPTIC: i32 = 1;

    // SAFETY: m, d are valid pointers (caller contract)
    unsafe {
        let nv = (*m).nv as i32;
        let nu = (*m).nu as i32;

        // nothing to do
        if nu == 0 {
            return;
        }

        // outputs
        let length = (*d).actuator_length;
        let moment = (*d).actuator_moment;
        let rownnz = (*d).moment_rownnz;
        let rowadr = (*d).moment_rowadr;
        let colind = (*d).moment_colind;

        // allocate Jacobians
        crate::engine::engine_memory::mj_mark_stack(d);
        let jac = crate::engine::engine_memory::mj_stack_alloc_num(d, (3 * nv) as usize);
        let jacA = crate::engine::engine_memory::mj_stack_alloc_num(d, (3 * nv) as usize);
        let jacS = crate::engine::engine_memory::mj_stack_alloc_num(d, (3 * nv) as usize);

        // define stack variables required for body transmission, don't allocate
        let issparse = crate::engine::engine_core_util::mj_is_sparse(m);
        let mut efc_force: *mut f64 = std::ptr::null_mut();
        let mut moment_exclude: *mut f64 = std::ptr::null_mut();
        let mut jacdifp: *mut f64 = std::ptr::null_mut();
        let mut jac1p: *mut f64 = std::ptr::null_mut();
        let mut jac2p: *mut f64 = std::ptr::null_mut();
        let mut chain: *mut i32 = std::ptr::null_mut();

        // define stack variables required for site transmission, don't allocate
        let mut jacref: *mut f64 = std::ptr::null_mut();
        let mut moment_row: *mut f64 = std::ptr::null_mut();

        let sleep_filter = ((*m).opt.enableflags & MJ_ENBL_SLEEP) != 0
            && (*d).nv_awake < nv;

        // compute lengths and moments
        for i in 0..nu {
            *rowadr.add(i as usize) = if i == 0 { 0 } else { *rowadr.add((i - 1) as usize) + *rownnz.add((i - 1) as usize) };
            let mut nnz: i32;
            let adr = *rowadr.add(i as usize);

            // skip sleeping actuator
            if sleep_filter && crate::engine::engine_sleep::mj_sleep_state(m, d as *const mjData, MJ_OBJ_ACTUATOR, i) == MJ_S_ASLEEP {
                *rownnz.add(i as usize) = 0;
                continue;
            }

            // extract info
            let id = *(*m).actuator_trnid.add(2 * i as usize);
            let gear = (*m).actuator_gear.add(6 * i as usize);

            // process according to transmission type
            let trntype = *(*m).actuator_trntype.add(i as usize);

            if trntype == mjtTrn_mjTRN_JOINT as i32 || trntype == mjtTrn_mjTRN_JOINTINPARENT as i32 {
                // slide and hinge joint: scalar gear
                if *(*m).jnt_type.add(id as usize) == mjtJoint_mjJNT_SLIDE as i32
                    || *(*m).jnt_type.add(id as usize) == mjtJoint_mjJNT_HINGE as i32
                {
                    *rownnz.add(i as usize) = 1;
                    *colind.add(adr as usize) = *(*m).jnt_dofadr.add(id as usize);

                    *length.add(i as usize) = *(*d).qpos.add(*(*m).jnt_qposadr.add(id as usize) as usize) * *gear;
                    *moment.add(adr as usize) = *gear;
                }
                // ball joint: 3D wrench gear
                else if *(*m).jnt_type.add(id as usize) == mjtJoint_mjJNT_BALL as i32 {
                    let mut axis = [0.0f64; 3];
                    let mut quat = [0.0f64; 4];
                    crate::engine::engine_inline::mji_copy4(quat.as_mut_ptr(), (*d).qpos.add(*(*m).jnt_qposadr.add(id as usize) as usize));
                    crate::engine::engine_util_blas::mju_normalize4(quat.as_mut_ptr());
                    crate::engine::engine_inline::mji_quat2vel(axis.as_mut_ptr(), quat.as_ptr(), 1.0);

                    // gearAxis: rotate to parent frame if necessary
                    let mut gearAxis = [0.0f64; 3];
                    if trntype == mjtTrn_mjTRN_JOINT as i32 {
                        crate::engine::engine_inline::mji_copy3(gearAxis.as_mut_ptr(), gear);
                    } else {
                        crate::engine::engine_util_spatial::mju_neg_quat(quat.as_mut_ptr(), quat.as_ptr());
                        crate::engine::engine_inline::mji_rot_vec_quat(gearAxis.as_mut_ptr(), gear, quat.as_ptr());
                    }

                    // length: axis*gearAxis
                    *length.add(i as usize) = crate::engine::engine_util_blas::mju_dot3(axis.as_ptr(), gearAxis.as_ptr());

                    // dof start address
                    let jnt_dofadr = *(*m).jnt_dofadr.add(id as usize);

                    // sparsity
                    for j in 0..3 {
                        *colind.add((adr + j) as usize) = jnt_dofadr + j;
                    }
                    *rownnz.add(i as usize) = 3;

                    // moment: gearAxis
                    crate::engine::engine_inline::mji_copy3(moment.add(adr as usize), gearAxis.as_ptr());
                }
                // free joint: 6D wrench gear
                else {
                    *length.add(i as usize) = 0.0;

                    // gearAxis: rotate to world frame if necessary
                    let mut gearAxis = [0.0f64; 3];
                    if trntype == mjtTrn_mjTRN_JOINT as i32 {
                        crate::engine::engine_inline::mji_copy3(gearAxis.as_mut_ptr(), gear.add(3));
                    } else {
                        let mut quat = [0.0f64; 4];
                        crate::engine::engine_inline::mji_copy4(quat.as_mut_ptr(), (*d).qpos.add((*(*m).jnt_qposadr.add(id as usize) + 3) as usize));
                        crate::engine::engine_util_blas::mju_normalize4(quat.as_mut_ptr());
                        crate::engine::engine_util_spatial::mju_neg_quat(quat.as_mut_ptr(), quat.as_ptr());
                        crate::engine::engine_inline::mji_rot_vec_quat(gearAxis.as_mut_ptr(), gear.add(3), quat.as_ptr());
                    }

                    // dof start address
                    let jnt_dofadr = *(*m).jnt_dofadr.add(id as usize);

                    // sparsity
                    for j in 0..6 {
                        *colind.add((adr + j) as usize) = jnt_dofadr + j;
                    }
                    *rownnz.add(i as usize) = 6;

                    // moment: gear(tran), gearAxis
                    crate::engine::engine_inline::mji_copy3(moment.add(adr as usize), gear);
                    crate::engine::engine_inline::mji_copy3(moment.add((adr + 3) as usize), gearAxis.as_ptr());
                }
            } else if trntype == mjtTrn_mjTRN_SLIDERCRANK as i32 {
                // slider-crank
                let idslider = *(*m).actuator_trnid.add((2 * i + 1) as usize);
                let rod = *(*m).actuator_cranklength.add(i as usize);
                let mut axis_sc = [0.0f64; 3];
                axis_sc[0] = *(*d).site_xmat.add((9 * idslider + 2) as usize);
                axis_sc[1] = *(*d).site_xmat.add((9 * idslider + 5) as usize);
                axis_sc[2] = *(*d).site_xmat.add((9 * idslider + 8) as usize);
                let mut vec = [0.0f64; 3];
                crate::engine::engine_util_blas::mju_sub3(vec.as_mut_ptr(),
                    (*d).site_xpos.add(3 * id as usize),
                    (*d).site_xpos.add(3 * idslider as usize));

                // compute length and determinant
                let av = crate::engine::engine_util_blas::mju_dot3(vec.as_ptr(), axis_sc.as_ptr());
                let det = av * av + rod * rod - crate::engine::engine_util_blas::mju_dot3(vec.as_ptr(), vec.as_ptr());
                let mut ok = 1;
                let sdet;
                if det <= 0.0 {
                    ok = 0;
                    sdet = 0.0;
                    *length.add(i as usize) = av;
                } else {
                    sdet = f64::sqrt(det);
                    *length.add(i as usize) = av - sdet;
                }

                // compute derivatives of length w.r.t. vec and axis
                let mut dlda = [0.0f64; 3];
                let mut dldv = [0.0f64; 3];
                if ok != 0 {
                    crate::engine::engine_util_blas::mju_scl3(dldv.as_mut_ptr(), axis_sc.as_ptr(), 1.0 - av / sdet);
                    crate::engine::engine_util_blas::mju_scl3(dlda.as_mut_ptr(), vec.as_ptr(), 1.0 / sdet);
                    crate::engine::engine_inline::mji_add_to3(dldv.as_mut_ptr(), dlda.as_ptr());

                    crate::engine::engine_util_blas::mju_scl3(dlda.as_mut_ptr(), vec.as_ptr(), 1.0 - av / sdet);
                } else {
                    crate::engine::engine_inline::mji_copy3(dlda.as_mut_ptr(), vec.as_ptr());
                    crate::engine::engine_inline::mji_copy3(dldv.as_mut_ptr(), axis_sc.as_ptr());
                }

                // get Jacobians of axis(jacA) and vec(jac)
                crate::engine::engine_core_util::mj_jac_point_axis(
                    m, d, jacS, jacA,
                    (*d).site_xpos.add(3 * idslider as usize),
                    axis_sc.as_ptr(), *(*m).site_bodyid.add(idslider as usize));
                crate::engine::engine_core_util::mj_jac_site(m, d as *const mjData, jac, std::ptr::null_mut(), id);
                crate::engine::engine_util_blas::mju_sub_from(jac, jacS, 3 * nv);

                moment_row = crate::engine::engine_memory::mj_stack_alloc_num(d, nv as usize);

                // clear moment
                crate::engine::engine_util_blas::mju_zero(moment_row, nv);

                // apply chain rule
                for j in 0..nv {
                    for k in 0..3i32 {
                        *moment_row.add(j as usize) += dlda[k as usize] * *jacA.add((k * nv + j) as usize)
                            + dldv[k as usize] * *jac.add((k * nv + j) as usize);
                    }
                }

                // scale by gear ratio
                *length.add(i as usize) *= *gear;

                // sparsity (compress)
                nnz = 0;
                for j in 0..nv {
                    if *moment_row.add(j as usize) != 0.0 {
                        *moment.add((adr + nnz) as usize) = *moment_row.add(j as usize) * *gear;
                        *colind.add((adr + nnz) as usize) = j;
                        nnz += 1;
                    }
                }
                *rownnz.add(i as usize) = nnz;
            } else if trntype == mjtTrn_mjTRN_TENDON as i32 {
                // tendon
                *length.add(i as usize) = *(*d).ten_length.add(id as usize) * *gear;

                let ten_J_rownnz = *(*m).ten_J_rownnz.add(id as usize);
                let ten_J_rowadr = *(*m).ten_J_rowadr.add(id as usize);
                *rownnz.add(i as usize) = ten_J_rownnz;
                crate::engine::engine_util_misc::mju_copy_int(
                    colind.add(adr as usize),
                    (*m).ten_J_colind.add(ten_J_rowadr as usize),
                    ten_J_rownnz);
                crate::engine::engine_util_blas::mju_scl(
                    moment.add(adr as usize),
                    (*d).ten_J.add(ten_J_rowadr as usize),
                    *gear, ten_J_rownnz);
            } else if trntype == mjtTrn_mjTRN_SITE as i32 {
                // site
                crate::engine::engine_core_util::mj_jac_site(m, d as *const mjData, jac, jacS, id);

                // clear length
                *length.add(i as usize) = 0.0;

                if moment_row.is_null() {
                    moment_row = crate::engine::engine_memory::mj_stack_alloc_num(d, nv as usize);
                }

                // reference site undefined
                if *(*m).actuator_trnid.add((2 * i + 1) as usize) == -1 {
                    // wrench: gear expressed in global frame
                    let mut wrench = [0.0f64; 6];
                    crate::engine::engine_inline::mji_mul_mat_vec3(wrench.as_mut_ptr(), (*d).site_xmat.add(9 * id as usize), gear);
                    crate::engine::engine_inline::mji_mul_mat_vec3(wrench.as_mut_ptr().add(3), (*d).site_xmat.add(9 * id as usize), gear.add(3));

                    // moment: global Jacobian projected on wrench
                    crate::engine::engine_util_blas::mju_mul_mat_t_vec(moment_row, jac, wrench.as_ptr(), 3, nv);
                    crate::engine::engine_util_blas::mju_mul_mat_t_vec(jac, jacS, wrench.as_ptr().add(3), 3, nv);
                    crate::engine::engine_util_blas::mju_add_to(moment_row, jac, nv);
                } else {
                    // reference site defined
                    let refid = *(*m).actuator_trnid.add((2 * i + 1) as usize);
                    if jacref.is_null() {
                        jacref = crate::engine::engine_memory::mj_stack_alloc_num(d, (3 * nv) as usize);
                    }

                    // find common ancestral dof
                    let b0 = *(*m).body_weldid.add(*(*m).site_bodyid.add(id as usize) as usize);
                    let b1 = *(*m).body_weldid.add(*(*m).site_bodyid.add(refid as usize) as usize);
                    let mut dofadr0 = *(*m).body_dofadr.add(b0 as usize) + *(*m).body_dofnum.add(b0 as usize) - 1;
                    let mut dofadr1 = *(*m).body_dofadr.add(b1 as usize) + *(*m).body_dofnum.add(b1 as usize) - 1;

                    let mut dofadr_common: i32 = -1;
                    if dofadr0 >= 0 && dofadr1 >= 0 {
                        while dofadr0 != dofadr1 {
                            if dofadr0 < dofadr1 {
                                dofadr1 = *(*m).dof_parentid.add(dofadr1 as usize);
                            } else {
                                dofadr0 = *(*m).dof_parentid.add(dofadr0 as usize);
                            }
                            if dofadr0 == -1 || dofadr1 == -1 {
                                break;
                            }
                        }
                        if dofadr0 == dofadr1 {
                            dofadr_common = dofadr0;
                        }
                    }

                    // clear moment
                    crate::engine::engine_util_blas::mju_zero(moment_row, nv);

                    // translational transmission
                    if crate::engine::engine_util_misc::mju_is_zero(gear, 3) == 0 {
                        // vec: site position in reference site frame
                        let mut vec_site = [0.0f64; 3];
                        crate::engine::engine_util_blas::mju_sub3(vec_site.as_mut_ptr(),
                            (*d).site_xpos.add(3 * id as usize),
                            (*d).site_xpos.add(3 * refid as usize));
                        crate::engine::engine_util_blas::mju_mul_mat_t_vec3(vec_site.as_mut_ptr(),
                            (*d).site_xmat.add(9 * refid as usize), vec_site.as_ptr());

                        // length: dot product with gear
                        *length.add(i as usize) += crate::engine::engine_util_blas::mju_dot3(vec_site.as_ptr(), gear);

                        // jacref: global Jacobian of reference site
                        crate::engine::engine_core_util::mj_jac_site(m, d as *const mjData, jacref, std::ptr::null_mut(), refid);

                        // subtract jacref from jac
                        crate::engine::engine_util_blas::mju_sub_from(jac, jacref, 3 * nv);

                        // clear columns of common ancestral dof parental chain
                        let mut da = dofadr_common;
                        while da >= 0 {
                            *jac.add((nv * 0 + da) as usize) = 0.0;
                            *jac.add((nv * 1 + da) as usize) = 0.0;
                            *jac.add((nv * 2 + da) as usize) = 0.0;
                            da = *(*m).dof_parentid.add(da as usize);
                        }

                        // wrench: translational gear expressed in global frame
                        let mut wrench = [0.0f64; 6];
                        crate::engine::engine_inline::mji_mul_mat_vec3(wrench.as_mut_ptr(),
                            (*d).site_xmat.add(9 * refid as usize), gear);

                        // moment: global Jacobian projected on wrench
                        crate::engine::engine_util_blas::mju_mul_mat_t_vec(moment_row, jac, wrench.as_ptr(), 3, nv);
                    }

                    // rotational transmission
                    if crate::engine::engine_util_misc::mju_is_zero(gear.add(3), 3) == 0 {
                        // get site and refsite quats from parent bodies
                        let mut quat = [0.0f64; 4];
                        let mut refquat = [0.0f64; 4];
                        crate::engine::engine_inline::mji_mul_quat(quat.as_mut_ptr(),
                            (*m).site_quat.add(4 * id as usize),
                            (*d).xquat.add(4 * *(*m).site_bodyid.add(id as usize) as usize));
                        crate::engine::engine_inline::mji_mul_quat(refquat.as_mut_ptr(),
                            (*m).site_quat.add(4 * refid as usize),
                            (*d).xquat.add(4 * *(*m).site_bodyid.add(refid as usize) as usize));

                        // convert difference to expmap (axis-angle)
                        let mut vec_rot = [0.0f64; 3];
                        crate::engine::engine_inline::mji_sub_quat(vec_rot.as_mut_ptr(), quat.as_ptr(), refquat.as_ptr());

                        // add length: dot product with gear
                        *length.add(i as usize) += crate::engine::engine_util_blas::mju_dot3(vec_rot.as_ptr(), gear.add(3));

                        // jacref: global rotational Jacobian of reference site
                        crate::engine::engine_core_util::mj_jac_site(m, d as *const mjData, std::ptr::null_mut(), jacref, refid);

                        // subtract jacref from jacS
                        crate::engine::engine_util_blas::mju_sub_from(jacS, jacref, 3 * nv);

                        // clear columns of common ancestral dof parental chain
                        let mut da = dofadr_common;
                        while da >= 0 {
                            *jacS.add((nv * 0 + da) as usize) = 0.0;
                            *jacS.add((nv * 1 + da) as usize) = 0.0;
                            *jacS.add((nv * 2 + da) as usize) = 0.0;
                            da = *(*m).dof_parentid.add(da as usize);
                        }

                        // wrench: rotational gear expressed in global frame
                        let mut wrench = [0.0f64; 6];
                        crate::engine::engine_inline::mji_mul_mat_vec3(wrench.as_mut_ptr(),
                            (*d).site_xmat.add(9 * refid as usize), gear.add(3));

                        // global Jacobian projected on wrench, add to moment
                        crate::engine::engine_util_blas::mju_mul_mat_t_vec(jac, jacS, wrench.as_ptr(), 3, nv);
                        crate::engine::engine_util_blas::mju_add_to(moment_row, jac, nv);
                    }
                }

                // sparsity (compress)
                nnz = 0;
                for j in 0..nv {
                    if *moment_row.add(j as usize) != 0.0 {
                        *moment.add((adr + nnz) as usize) = *moment_row.add(j as usize);
                        *colind.add((adr + nnz) as usize) = j;
                        nnz += 1;
                    }
                }
                *rownnz.add(i as usize) = nnz;
            } else if trntype == mjtTrn_mjTRN_BODY as i32 {
                // body (adhesive contacts)
                *length.add(i as usize) = 0.0;

                if moment_row.is_null() {
                    moment_row = crate::engine::engine_memory::mj_stack_alloc_num(d, nv as usize);
                }
                crate::engine::engine_util_blas::mju_zero(moment_row, nv);

                // allocate stack variables for the first mjTRN_BODY
                if efc_force.is_null() {
                    efc_force = crate::engine::engine_memory::mj_stack_alloc_num(d, (*d).nefc as usize);
                    moment_exclude = crate::engine::engine_memory::mj_stack_alloc_num(d, nv as usize);
                    jacdifp = crate::engine::engine_memory::mj_stack_alloc_num(d, (3 * nv) as usize);
                    jac1p = crate::engine::engine_memory::mj_stack_alloc_num(d, (3 * nv) as usize);
                    jac2p = crate::engine::engine_memory::mj_stack_alloc_num(d, (3 * nv) as usize);
                    if issparse != 0 {
                        chain = crate::engine::engine_memory::mj_stack_alloc_int(d, nv as usize);
                    }
                }

                // clear efc_force and moment_exclude
                crate::engine::engine_util_blas::mju_zero(efc_force, (*d).nefc);
                crate::engine::engine_util_blas::mju_zero(moment_exclude, nv);

                // count all relevant contacts, accumulate Jacobians
                let mut counter: i32 = 0;
                let ncon = (*d).ncon;
                for j in 0..ncon {
                    let con = (*d).contact.add(j as usize);

                    // get geom ids
                    let g1 = (*con).geom[0];
                    let g2 = (*con).geom[1];

                    // contact involving flex, continue
                    if g1 < 0 || g2 < 0 {
                        continue;
                    }

                    // get body ids
                    let b1 = *(*m).geom_bodyid.add(g1 as usize);
                    let b2 = *(*m).geom_bodyid.add(g2 as usize);

                    // irrelevant contact
                    if b1 != id && b2 != id {
                        continue;
                    }

                    // mark contact normals in efc_force
                    if (*con).exclude == 0 {
                        counter += 1;

                        // condim 1 or elliptic cones
                        if (*con).dim == 1 || (*m).opt.cone == MJ_CONE_ELLIPTIC {
                            *efc_force.add((*con).efc_address as usize) = 1.0;
                        } else {
                            // pyramidal cones
                            let npyramid = (*con).dim - 1;
                            for k in 0..(2 * npyramid) {
                                *efc_force.add(((*con).efc_address + k) as usize) = 0.5 / npyramid as f64;
                            }
                        }
                    }
                    // excluded contact in gap
                    else if (*con).exclude == 1 {
                        counter += 1;

                        // get Jacobian difference
                        let NV = crate::engine::engine_core_util::mj_jac_dif_pair(
                            m, d as *const mjData, chain, b1, b2,
                            (*con).pos.as_ptr(), (*con).pos.as_ptr(),
                            jac1p, jac2p, jacdifp, std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut(),
                            issparse, 0);

                        // project Jacobian along the normal of the contact frame
                        crate::engine::engine_util_blas::mju_mul_mat_mat(
                            jac, (*con).frame.as_ptr(), jacdifp, 1, 3, NV);

                        // accumulate in moment_exclude
                        if issparse != 0 {
                            for k in 0..NV {
                                *moment_exclude.add(*chain.add(k as usize) as usize) += *jac.add(k as usize);
                            }
                        } else {
                            crate::engine::engine_util_blas::mju_add_to(moment_exclude, jac, nv);
                        }
                    }
                }

                // moment is average over contact normal Jacobians
                if counter > 0 {
                    // accumulate active contact Jacobians into moment
                    crate::engine::engine_core_constraint::mj_mul_jac_t_vec(m, d as *const mjData, moment_row, efc_force);

                    // add Jacobians from excluded contacts
                    crate::engine::engine_util_blas::mju_add_to(moment_row, moment_exclude, nv);

                    // normalize by total contacts, flip sign
                    crate::engine::engine_util_blas::mju_scl(moment_row, moment_row, -1.0 / counter as f64, nv);
                }

                // sparsity (compress)
                nnz = 0;
                for j in 0..nv {
                    if *moment_row.add(j as usize) != 0.0 {
                        *moment.add((adr + nnz) as usize) = *moment_row.add(j as usize);
                        *colind.add((adr + nnz) as usize) = j;
                        nnz += 1;
                    }
                }
                *rownnz.add(i as usize) = nnz;
            } else {
                // unknown type - should not occur
                *rownnz.add(i as usize) = 0;
            }
        }

        crate::engine::engine_memory::mj_free_stack(d);
    }
}

/// C: mj_crb (engine/engine_core_smooth.h:59)
/// Calls: mj_actuatorArmature, mji_dot6, mju_addTo, mju_copy, mju_copyRows, mju_mulInertVec, mju_zero, mju_zeroSparse
#[allow(unused_variables, non_snake_case)]
pub fn mj_crb(m: *const mjModel, d: *mut mjData) {
    const MJ_ENBL_SLEEP: i32 = 1 << 4;
    const MJ_OBJ_JOINT: u32 = 3;

    // SAFETY: m and d are valid pointers (caller contract). All field accesses are within bounds.
    unsafe {
        // outputs
        let crb = (*d).crb;
        let M = (*d).M;

        // inputs
        let cinert = (*d).cinert;
        let cdof = (*d).cdof;
        let dof_M0 = (*m).dof_M0;
        let dof_armature = (*m).dof_armature;
        let body_awake_ind = (*d).body_awake_ind;
        let parent_awake_ind = (*d).parent_awake_ind;
        let dof_awake_ind = (*d).dof_awake_ind;
        let rownnz = (*m).M_rownnz;
        let rowadr = (*m).M_rowadr;
        let body_parentid = (*m).body_parentid;
        let dof_parentid = (*m).dof_parentid;
        let dof_simplenum = (*m).dof_simplenum;
        let dof_bodyid = (*m).dof_bodyid;

        // sleep filtering
        let sleep_filter = ((*m).opt.enableflags & MJ_ENBL_SLEEP) != 0 && (*d).nv_awake < (*m).nv as i32;
        let nbody = if sleep_filter { (*d).nbody_awake } else { (*m).nbody as i32 };
        let nparent = if sleep_filter { (*d).nparent_awake } else { (*m).nbody as i32 };
        let nv = if sleep_filter { (*d).nv_awake } else { (*m).nv as i32 };

        // crb = cinert
        if !sleep_filter {
            crate::engine::engine_util_blas::mju_copy(crb, cinert as *const f64, 10 * nbody);
        } else {
            crate::engine::engine_util_blas::mju_copy_rows(crb, cinert as *const f64, body_awake_ind, nbody, 10);
        }

        // backward pass over bodies, accumulate composite inertias
        let mut b = nparent - 1;
        while b >= 0 {
            let i = if sleep_filter { *parent_awake_ind.add(b as usize) } else { b };
            if *body_parentid.add(i as usize) > 0 {
                crate::engine::engine_util_blas::mju_add_to(
                    crb.add(10 * *body_parentid.add(i as usize) as usize),
                    crb.add(10 * i as usize) as *const f64,
                    10,
                );
            }
            b -= 1;
        }

        // clear M
        if !sleep_filter {
            crate::engine::engine_util_blas::mju_zero(M, (*m).nC as i32);
        } else {
            crate::engine::engine_util_sparse::mju_zero_sparse(M, rownnz, rowadr, dof_awake_ind, nv);
        }

        // dense forward pass over dofs
        for v in 0..nv {
            let i = if sleep_filter { *dof_awake_ind.add(v as usize) } else { v };

            // simple dof: fixed diagonal inertia
            let adr = *rowadr.add(i as usize);
            if *dof_simplenum.add(i as usize) != 0 {
                *M.add(adr as usize) = *dof_M0.add(i as usize);
                continue;
            }

            // init M(i,i) with armature inertia
            let mut Madr_ij = adr + *rownnz.add(i as usize) - 1;
            *M.add(Madr_ij as usize) = *dof_armature.add(i as usize)
                + crate::engine::engine_core_util::mj_actuator_armature(m, MJ_OBJ_JOINT, *(*m).dof_jntid.add(i as usize));

            // precompute buf = crb_body_i * cdof_i
            let mut buf: [f64; 6] = [0.0; 6];
            crate::engine::engine_util_spatial::mju_mul_inert_vec(
                buf.as_mut_ptr(),
                crb.add(10 * *dof_bodyid.add(i as usize) as usize) as *const f64,
                cdof.add(6 * i as usize) as *const f64,
            );

            // sparse backward pass over ancestors
            let mut j = i;
            while j >= 0 {
                // M(i,j) += cdof_j * (crb_body_i * cdof_i)
                *M.add(Madr_ij as usize) += crate::engine::engine_inline::mji_dot6(
                    cdof.add(6 * j as usize) as *const f64,
                    buf.as_ptr(),
                );
                Madr_ij -= 1;
                j = *dof_parentid.add(j as usize);
            }
        }
    }
}

/// C: mj_tendonArmature (engine/engine_core_smooth.h:62)
/// Calls: mj_actuatorArmature, mj_sleepState, mju_addToSclSparseInc
#[allow(unused_variables, non_snake_case)]
pub fn mj_tendon_armature(m: *const mjModel, d: *mut mjData) {
    const MJ_ENBL_SLEEP: i32 = 1 << 4;
    const MJ_OBJ_TENDON: u32 = 18;
    const MJ_S_ASLEEP: i32 = 0;

    // SAFETY: m, d are valid pointers (caller contract).
    unsafe {
        let nv = (*m).nv as i32;
        let ntendon = (*m).ntendon as i32;
        let M_rownnz = (*m).M_rownnz;
        let M_rowadr = (*m).M_rowadr;
        let M_colind = (*m).M_colind;

        // sleep filtering
        let sleep_filter = ((*m).opt.enableflags & MJ_ENBL_SLEEP) != 0
            && (*d).nv_awake < nv;

        for k in 0..ntendon {
            // skip sleeping tendon
            if sleep_filter
                && crate::engine::engine_sleep::mj_sleep_state(
                    m, d as *const mjData, MJ_OBJ_TENDON, k) == MJ_S_ASLEEP
            {
                continue;
            }

            let armature = *(*m).tendon_armature.add(k as usize)
                + crate::engine::engine_core_util::mj_actuator_armature(m, MJ_OBJ_TENDON, k);
            if armature == 0.0 {
                continue;
            }

            // get sparse info for tendon k
            let J_rowadr = *(*m).ten_J_rowadr.add(k as usize);
            let J_rownnz = *(*m).ten_J_rownnz.add(k as usize);
            let J_colind = (*m).ten_J_colind.add(J_rowadr as usize);
            let ten_J = (*d).ten_J.add(J_rowadr as usize);

            // M += armature * ten_J' * ten_J
            for j in 0..J_rownnz {
                let ten_J_i = *ten_J.add(j as usize);
                if ten_J_i == 0.0 {
                    continue;
                }

                // M[i,:] += armature * ten_J[i] * ten_J
                let i = *J_colind.add(j as usize);
                let M_adr = *M_rowadr.add(i as usize);
                crate::engine::engine_util_sparse::mju_add_to_scl_sparse_inc(
                    (*d).M.add(M_adr as usize),
                    ten_J,
                    *M_rownnz.add(i as usize),
                    M_colind.add(M_adr as usize),
                    J_rownnz,
                    J_colind,
                    armature * ten_J_i,
                );
            }
        }
    }
}

/// C: mj_makeM (engine/engine_core_smooth.h:65)
/// Calls: mj_crb, mj_tendonArmature, mju_scatter
#[allow(unused_variables, non_snake_case)]
pub fn mj_make_m(m: *const mjModel, d: *mut mjData) {
    // SAFETY: m, d are valid pointers (caller contract).
    unsafe {
        mj_crb(m, d);
        mj_tendon_armature(m, d);
        crate::engine::engine_util_misc::mju_scatter(
            (*d).qM, (*d).M, (*m).mapM2M, (*m).nC as i32);
    }
}

/// C: mj_factorI_legacy (engine/engine_core_smooth.h:68)
/// Calls: mj_warning, mju_addToScl, mju_copy
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_factor_i_legacy(m: *const mjModel, d: *mut mjData, M: *const f64, qLD: *mut f64, qLDiagInv: *mut f64) {
    const MJMINVAL: f64 = 1e-15;
    const MJ_WARN_INERTIA: i32 = 0;

    // SAFETY: m, d, M, qLD, qLDiagInv are valid pointers (caller contract).
    unsafe {
        let dof_Madr = (*m).dof_Madr;
        let dof_parentid = (*m).dof_parentid;
        let nv = (*m).nv as i32;

        // copy M into LD
        crate::engine::engine_util_blas::mju_copy(qLD, M, (*m).nM as i32);

        // dense backward loop over dofs (regular only, simple diagonal already copied)
        for k in (0..nv).rev() {
            // get address of M(k,k)
            let Madr_kk = *dof_Madr.add(k as usize);

            // check for small/negative numbers on diagonal
            if *qLD.add(Madr_kk as usize) < MJMINVAL {
                crate::engine::engine_core_util::mj_warning(d, MJ_WARN_INERTIA, k);
                *qLD.add(Madr_kk as usize) = MJMINVAL;
            }

            // skip the rest if simple
            if *(*m).dof_simplenum.add(k as usize) != 0 {
                continue;
            }

            // sparse backward loop over ancestors of k (excluding k)
            let mut Madr_ki = Madr_kk + 1;
            let mut i = *dof_parentid.add(k as usize);
            while i >= 0 {
                let tmp = *qLD.add(Madr_ki as usize) / *qLD.add(Madr_kk as usize);

                // get number of ancestors of i (including i)
                let cnt: i32 = if i < nv - 1 {
                    *dof_Madr.add((i + 1) as usize) - *dof_Madr.add(i as usize)
                } else {
                    (*m).nM as i32 - *dof_Madr.add((i + 1) as usize)
                };

                // M(i,j) -= M(k,j) * tmp
                crate::engine::engine_util_blas::mju_add_to_scl(
                    qLD.add(*dof_Madr.add(i as usize) as usize),
                    qLD.add(Madr_ki as usize),
                    -tmp,
                    cnt,
                );

                *qLD.add(Madr_ki as usize) = tmp;

                // advance to i's parent
                i = *dof_parentid.add(i as usize);
                Madr_ki += 1;
            }
        }

        // compute 1/diag(D)
        for i in 0..nv {
            *qLDiagInv.add(i as usize) = 1.0 / *qLD.add(*dof_Madr.add(i as usize) as usize);
        }
    }
}

/// C: mj_factorI (engine/engine_core_smooth.h:72)
/// Calls: mju_addToScl, mju_scl
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_factor_i(mat: *mut f64, diaginv: *mut f64, nv: i32, rownnz: *const i32, rowadr: *const i32, colind: *const i32, index: *const i32) {
    // SAFETY: caller guarantees all pointers are valid for the sparse matrix layout.
    // index may be null (optional permutation). diaginv may be null.
    unsafe {
        let mut j: i32 = nv - 1;
        while j >= 0 {
            let k: i32 = if !index.is_null() { *index.add(j as usize) } else { j };
            let start: i32 = *rowadr.add(k as usize);
            let diag: i32 = *rownnz.add(k as usize) - 1;
            let end: i32 = start + diag;
            let invD: f64 = 1.0 / *mat.add(end as usize);
            if !diaginv.is_null() {
                *diaginv.add(k as usize) = invD;
            }
            let mut adr: i32 = end - 1;
            while adr >= start {
                let i: i32 = *colind.add(adr as usize);
                crate::engine::engine_util_blas::mju_add_to_scl(
                    mat.add(*rowadr.add(i as usize) as usize),
                    mat.add(start as usize) as *const f64,
                    -*mat.add(adr as usize) * invD,
                    *rownnz.add(i as usize),
                );
                adr -= 1;
            }
            crate::engine::engine_util_blas::mju_scl(
                mat.add(start as usize),
                mat.add(start as usize) as *const f64,
                invD,
                diag,
            );
            j -= 1;
        }
    }
}

/// C: mj_factorM (engine/engine_core_smooth.h:76)
/// Calls: mj_factorI, mju_copy, mju_copySparse
#[allow(unused_variables, non_snake_case)]
pub fn mj_factor_m(m: *const mjModel, d: *mut mjData) {
    // SAFETY: m and d are valid pointers from caller; all field accesses are within bounds
    unsafe {
        const mjENBL_SLEEP: i32 = 1 << 4;

        // sleep filtering
        let sleep_filter = ((*m).opt.enableflags & mjENBL_SLEEP) != 0
            && (*d).nv_awake < (*m).nv as i32;

        let index: *const i32;
        let nv: i32;

        // no sleep filtering: copy everything
        if sleep_filter == false {
            index = std::ptr::null();
            nv = (*m).nv as i32;
            crate::engine::engine_util_blas::mju_copy(
                (*d).qLD, (*d).M, (*m).nC as i32,
            );
        } else {
            // sleep filtering: copy only awake dofs
            index = (*d).dof_awake_ind;
            nv = (*d).nv_awake;
            crate::engine::engine_util_sparse::mju_copy_sparse(
                (*d).qLD, (*d).M,
                (*m).M_rownnz, (*m).M_rowadr,
                (*d).dof_awake_ind, (*d).nv_awake,
            );
        }

        // factorize
        mj_factor_i(
            (*d).qLD, (*d).qLDiagInv, nv,
            (*m).M_rownnz, (*m).M_rowadr, (*m).M_colind, index,
        );
    }
}

/// C: mj_solveLD_legacy (engine/engine_core_smooth.h:79)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_solve_ld_legacy(m: *const mjModel, x: *mut f64, n: i32, qLD: *const f64, qLDiagInv: *const f64) {
    // SAFETY: caller guarantees m is valid, x has n*nv elements,
    // qLD and qLDiagInv are valid for the model's sparse structure.
    unsafe {
        let dof_Madr = (*m).dof_Madr;
        let dof_parentid = (*m).dof_parentid;
        let nv = (*m).nv as i32;

        if n == 1 {
            // x <- inv(L') * x
            let mut i: i32 = nv - 1;
            while i >= 0 {
                if *(*m).dof_simplenum.add(i as usize) == 0 && *x.add(i as usize) != 0.0 {
                    let mut Madr_ij: i32 = *dof_Madr.add(i as usize) + 1;
                    let mut j: i32 = *dof_parentid.add(i as usize);
                    while j >= 0 {
                        *x.add(j as usize) -= *qLD.add(Madr_ij as usize) * *x.add(i as usize);
                        Madr_ij += 1;
                        j = *dof_parentid.add(j as usize);
                    }
                }
                i -= 1;
            }

            // x <- inv(D) * x
            let mut i: i32 = 0;
            while i < nv {
                *x.add(i as usize) *= *qLDiagInv.add(i as usize);
                i += 1;
            }

            // x <- inv(L) * x
            let mut i: i32 = 0;
            while i < nv {
                if *(*m).dof_simplenum.add(i as usize) == 0 {
                    let mut Madr_ij: i32 = *dof_Madr.add(i as usize) + 1;
                    let mut j: i32 = *dof_parentid.add(i as usize);
                    while j >= 0 {
                        *x.add(i as usize) -= *qLD.add(Madr_ij as usize) * *x.add(j as usize);
                        Madr_ij += 1;
                        j = *dof_parentid.add(j as usize);
                    }
                }
                i += 1;
            }
        } else {
            let mut offset: i32;
            let mut tmp: f64;

            // x <- inv(L') * x
            let mut i: i32 = nv - 1;
            while i >= 0 {
                if *(*m).dof_simplenum.add(i as usize) == 0 {
                    let mut Madr_ij: i32 = *dof_Madr.add(i as usize) + 1;
                    let mut j: i32 = *dof_parentid.add(i as usize);
                    while j >= 0 {
                        offset = 0;
                        while offset < n * nv {
                            tmp = *x.add((i + offset) as usize);
                            if tmp != 0.0 {
                                *x.add((j + offset) as usize) -= *qLD.add(Madr_ij as usize) * tmp;
                            }
                            offset += nv;
                        }
                        Madr_ij += 1;
                        j = *dof_parentid.add(j as usize);
                    }
                }
                i -= 1;
            }

            // x <- inv(D) * x
            let mut i: i32 = 0;
            while i < nv {
                offset = 0;
                while offset < n * nv {
                    *x.add((i + offset) as usize) *= *qLDiagInv.add(i as usize);
                    offset += nv;
                }
                i += 1;
            }

            // x <- inv(L) * x
            let mut i: i32 = 0;
            while i < nv {
                if *(*m).dof_simplenum.add(i as usize) == 0 {
                    let mut Madr_ij: i32 = *dof_Madr.add(i as usize) + 1;
                    let mut j: i32 = *dof_parentid.add(i as usize);
                    while j >= 0 {
                        offset = 0;
                        while offset < n * nv {
                            *x.add((i + offset) as usize) -= *qLD.add(Madr_ij as usize) * *x.add((j + offset) as usize);
                            offset += nv;
                        }
                        Madr_ij += 1;
                        j = *dof_parentid.add(j as usize);
                    }
                }
                i += 1;
            }
        }
    }
}

/// C: mj_solveLD (engine/engine_core_smooth.h:84)
/// Calls: mju_dotSparse
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_solve_ld(x: *mut f64, qLD: *const f64, qLDiagInv: *const f64, nv: i32, n: i32, rownnz: *const i32, rowadr: *const i32, colind: *const i32, index: *const i32) {
    // SAFETY: caller guarantees all pointers are valid and arrays are properly sized
    unsafe {
        // x <- L^-T x
        for k in (0..nv).rev() {
            let i = if !index.is_null() { *index.add(k as usize) } else { k };

            // skip diagonal rows
            if *rownnz.add(i as usize) == 1 {
                continue;
            }

            // one vector
            if n == 1 {
                let x_i = *x.add(i as usize);
                if x_i != 0.0 {
                    let start = *rowadr.add(i as usize);
                    let end = start + *rownnz.add(i as usize) - 1;
                    for adr in start..end {
                        *x.add(*colind.add(adr as usize) as usize) -= *qLD.add(adr as usize) * x_i;
                    }
                }
            }
            // multiple vectors
            else {
                let start = *rowadr.add(i as usize);
                let end = start + *rownnz.add(i as usize) - 1;
                let mut offset = 0i32;
                while offset < n * nv {
                    let x_i = *x.add((i + offset) as usize);
                    if x_i != 0.0 {
                        for adr in start..end {
                            *x.add((offset + *colind.add(adr as usize)) as usize) -= *qLD.add(adr as usize) * x_i;
                        }
                    }
                    offset += nv;
                }
            }
        }

        // x <- D^-1 x
        for k in 0..nv {
            let i = if !index.is_null() { *index.add(k as usize) } else { k };
            let invD_i = *qLDiagInv.add(i as usize);

            // one vector
            if n == 1 {
                *x.add(i as usize) *= invD_i;
            }
            // multiple vectors
            else {
                let mut offset = 0i32;
                while offset < n * nv {
                    *x.add((i + offset) as usize) *= invD_i;
                    offset += nv;
                }
            }
        }

        // x <- L^-1 x
        for k in 0..nv {
            let i = if !index.is_null() { *index.add(k as usize) } else { k };

            // skip diagonal rows
            if *rownnz.add(i as usize) == 1 {
                continue;
            }

            let d = *rownnz.add(i as usize) - 1;
            if d > 0 {
                let adr = *rowadr.add(i as usize);

                // one vector
                if n == 1 {
                    *x.add(i as usize) -= crate::engine::engine_util_sparse::mju_dot_sparse(
                        qLD.add(adr as usize), x, d, colind.add(adr as usize));
                }
                // multiple vectors
                else {
                    let mut offset = 0i32;
                    while offset < n * nv {
                        *x.add((i + offset) as usize) -= crate::engine::engine_util_sparse::mju_dot_sparse(
                            qLD.add(adr as usize), x.add(offset as usize), d, colind.add(adr as usize));
                        offset += nv;
                    }
                }
            }
        }
    }
}

/// C: mj_solveM (engine/engine_core_smooth.h:88)
/// Calls: mj_solveLD, mju_copy
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_solve_m(m: *const mjModel, d: *mut mjData, x: *mut f64, y: *const f64, n: i32) {
    // SAFETY: caller guarantees m, d are valid; x, y point to arrays of size n*m->nv
    unsafe {
        if x != y as *mut f64 {
            crate::engine::engine_util_blas::mju_copy(x, y, n * (*m).nv as i32);
        }
        mj_solve_ld(
            x,
            (*d).qLD,
            (*d).qLDiagInv,
            (*m).nv as i32,
            n,
            (*m).M_rownnz,
            (*m).M_rowadr,
            (*m).M_colind,
            std::ptr::null(),
        );
    }
}

/// C: mj_solveM2 (engine/engine_core_smooth.h:91)
/// Calls: mju_copy
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_solve_m2(m: *const mjModel, d: *mut mjData, x: *mut f64, y: *const f64, sqrtInvD: *const f64, n: i32) {
    // SAFETY: caller guarantees m, d are valid structs, x/y have n*nv elements,
    // sqrtInvD has nv elements, and the sparse matrix fields are valid.
    unsafe {
        let nv = (*m).nv as i32;
        let rownnz = (*m).M_rownnz;
        let rowadr = (*m).M_rowadr;
        let colind = (*m).M_colind;
        let diagnum = (*m).dof_simplenum;
        let qLD = (*d).qLD as *const f64;

        crate::engine::engine_util_blas::mju_copy(x, y, n * nv);

        // x <- L^-T x
        let mut i: i32 = nv - 1;
        while i > 0 {
            if *diagnum.add(i as usize) != 0 {
                i -= 1;
                continue;
            }
            let start: i32 = *rowadr.add(i as usize);
            let end: i32 = start + *rownnz.add(i as usize) - 1;
            let mut offset: i32 = 0;
            while offset < n * nv {
                let x_i: f64 = *x.add((i + offset) as usize);
                if x_i != 0.0 {
                    let mut adr: i32 = start;
                    while adr < end {
                        *x.add((offset + *colind.add(adr as usize)) as usize) -= *qLD.add(adr as usize) * x_i;
                        adr += 1;
                    }
                }
                offset += nv;
            }
            i -= 1;
        }

        // x <- D^-1/2 x
        let mut i: i32 = 0;
        while i < nv {
            let invD_i: f64 = *sqrtInvD.add(i as usize);
            let mut offset: i32 = 0;
            while offset < n * nv {
                *x.add((i + offset) as usize) *= invD_i;
                offset += nv;
            }
            i += 1;
        }
    }
}

/// C: mj_comVel (engine/engine_core_smooth.h:98)
/// Calls: mji_copy6, mji_crossMotion, mju_addTo, mju_copy, mju_mulDofVec, mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mj_com_vel(m: *const mjModel, d: *mut mjData) {
    const MJ_ENBL_SLEEP: i32 = 1 << 4;
    const MJ_JNT_FREE: i32 = 0;
    const MJ_JNT_BALL: i32 = 1;

    // SAFETY: m, d are valid pointers (caller contract).
    unsafe {
        let sleep_filter = ((*m).opt.enableflags & MJ_ENBL_SLEEP) != 0
            && ((*d).nbody_awake as i64) < (*m).nbody;
        let nbody = if sleep_filter { (*d).nbody_awake as i32 } else { (*m).nbody as i32 };

        // set world vel to 0
        crate::engine::engine_util_blas::mju_zero((*d).cvel, 6);

        // forward pass over bodies
        for b in 1..nbody {
            let i = if sleep_filter { *(*d).body_awake_ind.add(b as usize) } else { b };

            // cvel = cvel_parent
            let mut cvel: [f64; 6] = [0.0; 6];
            crate::engine::engine_inline::mji_copy6(
                cvel.as_mut_ptr(),
                (*d).cvel.add(6 * *(*m).body_parentid.add(i as usize) as usize),
            );

            // cvel = cvel_parent + cdof * qvel, cdofdot = cvel x cdof
            let dofnum = *(*m).body_dofnum.add(i as usize);
            let bda = *(*m).body_dofadr.add(i as usize);
            let mut cdofdot: [f64; 36] = [0.0; 36];
            let mut j: i32 = 0;
            while j < dofnum {
                let mut tmp: [f64; 6] = [0.0; 6];
                let jnt_type = *(*m).jnt_type.add(*(*m).dof_jntid.add((bda + j) as usize) as usize);

                if jnt_type == MJ_JNT_FREE {
                    // cdofdot = 0
                    crate::engine::engine_util_blas::mju_zero(cdofdot.as_mut_ptr(), 18);

                    // update velocity
                    crate::engine::engine_util_spatial::mju_mul_dof_vec(
                        tmp.as_mut_ptr(), (*d).cdof.add(6 * bda as usize), (*d).qvel.add(bda as usize), 3);
                    crate::engine::engine_util_blas::mju_add_to(cvel.as_mut_ptr(), tmp.as_ptr(), 6);

                    // continue with rotations
                    j += 3;
                    // fallthrough to BALL case
                    crate::engine::engine_inline::mji_cross_motion(
                        cdofdot.as_mut_ptr().add(6 * j as usize),
                        cvel.as_ptr(),
                        (*d).cdof.add(6 * (bda + j) as usize),
                    );
                    crate::engine::engine_inline::mji_cross_motion(
                        cdofdot.as_mut_ptr().add(6 * (j + 1) as usize),
                        cvel.as_ptr(),
                        (*d).cdof.add(6 * (bda + j + 1) as usize),
                    );
                    crate::engine::engine_inline::mji_cross_motion(
                        cdofdot.as_mut_ptr().add(6 * (j + 2) as usize),
                        cvel.as_ptr(),
                        (*d).cdof.add(6 * (bda + j + 2) as usize),
                    );

                    // update velocity
                    crate::engine::engine_util_spatial::mju_mul_dof_vec(
                        tmp.as_mut_ptr(), (*d).cdof.add(6 * (bda + j) as usize),
                        (*d).qvel.add((bda + j) as usize), 3);
                    crate::engine::engine_util_blas::mju_add_to(cvel.as_mut_ptr(), tmp.as_ptr(), 6);
                    j += 2;
                } else if jnt_type == MJ_JNT_BALL {
                    // compute all 3 cdofdots using parent velocity
                    crate::engine::engine_inline::mji_cross_motion(
                        cdofdot.as_mut_ptr().add(6 * j as usize),
                        cvel.as_ptr(),
                        (*d).cdof.add(6 * (bda + j) as usize),
                    );
                    crate::engine::engine_inline::mji_cross_motion(
                        cdofdot.as_mut_ptr().add(6 * (j + 1) as usize),
                        cvel.as_ptr(),
                        (*d).cdof.add(6 * (bda + j + 1) as usize),
                    );
                    crate::engine::engine_inline::mji_cross_motion(
                        cdofdot.as_mut_ptr().add(6 * (j + 2) as usize),
                        cvel.as_ptr(),
                        (*d).cdof.add(6 * (bda + j + 2) as usize),
                    );

                    // update velocity
                    crate::engine::engine_util_spatial::mju_mul_dof_vec(
                        tmp.as_mut_ptr(), (*d).cdof.add(6 * (bda + j) as usize),
                        (*d).qvel.add((bda + j) as usize), 3);
                    crate::engine::engine_util_blas::mju_add_to(cvel.as_mut_ptr(), tmp.as_ptr(), 6);
                    j += 2;
                } else {
                    // default (hinge/slide)
                    crate::engine::engine_inline::mji_cross_motion(
                        cdofdot.as_mut_ptr().add(6 * j as usize),
                        cvel.as_ptr(),
                        (*d).cdof.add(6 * (bda + j) as usize),
                    );

                    // update velocity
                    crate::engine::engine_util_spatial::mju_mul_dof_vec(
                        tmp.as_mut_ptr(), (*d).cdof.add(6 * (bda + j) as usize),
                        (*d).qvel.add((bda + j) as usize), 1);
                    crate::engine::engine_util_blas::mju_add_to(cvel.as_mut_ptr(), tmp.as_ptr(), 6);
                }

                j += 1;
            }

            // assign cvel, cdofdot
            crate::engine::engine_inline::mji_copy6((*d).cvel.add(6 * i as usize), cvel.as_ptr());
            crate::engine::engine_util_blas::mju_copy(
                (*d).cdof_dot.add(6 * bda as usize), cdofdot.as_ptr(), 6 * dofnum);
        }
    }
}

/// C: mj_subtreeVel (engine/engine_core_smooth.h:101)
/// Calls: mj_freeStack, mj_markStack, mj_objectVelocity, mj_stackAllocInfo, mji_addTo3, mji_cross, mji_mulMatVec3, mju_max, mju_mulMatTVec3, mju_scl3, mju_sub3
#[allow(unused_variables, non_snake_case)]
pub fn mj_subtree_vel(m: *const mjModel, d: *mut mjData) {
    const MJ_ENBL_SLEEP: i32 = 1 << 4;
    const MJ_OBJ_BODY: i32 = 1;
    const MJMINVAL: f64 = 1e-15;

    // SAFETY: m, d are valid pointers (caller contract).
    unsafe {
        let sleep_filter = ((*m).opt.enableflags & MJ_ENBL_SLEEP) != 0
            && ((*d).nbody_awake as i64) < (*m).nbody;
        let nbody = if sleep_filter { (*d).nbody_awake } else { (*m).nbody as i32 };

        crate::engine::engine_memory::mj_mark_stack(d);
        let body_vel = crate::engine::engine_memory::mj_stack_alloc_num(d, (6 * (*m).nbody) as usize);

        // bodywise quantities
        for b in 0..nbody {
            let i = if sleep_filter { *(*d).body_awake_ind.add(b as usize) } else { b };

            // compute and save body velocity
            crate::engine::engine_core_util::mj_object_velocity(
                m, d as *const mjData, MJ_OBJ_BODY, i, body_vel.add(6 * i as usize), 0);

            // body linear momentum
            crate::engine::engine_util_blas::mju_scl3(
                (*d).subtree_linvel.add(3 * i as usize),
                body_vel.add(6 * i as usize + 3),
                *(*m).body_mass.add(i as usize),
            );

            // body angular momentum
            let mut dv: [f64; 3] = [0.0; 3];
            crate::engine::engine_util_blas::mju_mul_mat_t_vec3(
                dv.as_mut_ptr(), (*d).ximat.add(9 * i as usize), body_vel.add(6 * i as usize));
            dv[0] *= *(*m).body_inertia.add(3 * i as usize);
            dv[1] *= *(*m).body_inertia.add(3 * i as usize + 1);
            dv[2] *= *(*m).body_inertia.add(3 * i as usize + 2);
            crate::engine::engine_inline::mji_mul_mat_vec3(
                (*d).subtree_angmom.add(3 * i as usize), (*d).ximat.add(9 * i as usize), dv.as_ptr());
        }

        // subtree linear velocity
        for b in (0..nbody).rev() {
            let i = if sleep_filter { *(*d).body_awake_ind.add(b as usize) } else { b };

            // non-world: add linear momentum to parent
            if i != 0 {
                crate::engine::engine_inline::mji_add_to3(
                    (*d).subtree_linvel.add(3 * *(*m).body_parentid.add(i as usize) as usize),
                    (*d).subtree_linvel.add(3 * i as usize),
                );
            }

            // convert linear momentum to linear velocity
            let inv_mass = 1.0 / crate::engine::engine_util_misc::mju_max(
                MJMINVAL, *(*m).body_subtreemass.add(i as usize));
            crate::engine::engine_util_blas::mju_scl3(
                (*d).subtree_linvel.add(3 * i as usize),
                (*d).subtree_linvel.add(3 * i as usize),
                inv_mass,
            );
        }

        // subtree angular momentum
        for b in (1..nbody).rev() {
            let i = if sleep_filter { *(*d).body_awake_ind.add(b as usize) } else { b };
            let parent = *(*m).body_parentid.add(i as usize);

            // momentum wrt body i
            let mut dx: [f64; 3] = [0.0; 3];
            let mut dv: [f64; 3] = [0.0; 3];
            let mut dp: [f64; 3] = [0.0; 3];
            let mut dL: [f64; 3] = [0.0; 3];
            crate::engine::engine_util_blas::mju_sub3(
                dx.as_mut_ptr(), (*d).xipos.add(3 * i as usize), (*d).subtree_com.add(3 * i as usize));
            crate::engine::engine_util_blas::mju_sub3(
                dv.as_mut_ptr(), body_vel.add(6 * i as usize + 3), (*d).subtree_linvel.add(3 * i as usize));
            crate::engine::engine_util_blas::mju_scl3(
                dp.as_mut_ptr(), dv.as_ptr(), *(*m).body_mass.add(i as usize));
            crate::engine::engine_inline::mji_cross(dL.as_mut_ptr(), dx.as_ptr(), dp.as_ptr());

            // add to subtree i
            crate::engine::engine_inline::mji_add_to3(
                (*d).subtree_angmom.add(3 * i as usize), dL.as_ptr());

            // add to parent
            crate::engine::engine_inline::mji_add_to3(
                (*d).subtree_angmom.add(3 * parent as usize),
                (*d).subtree_angmom.add(3 * i as usize),
            );

            // momentum wrt parent
            crate::engine::engine_util_blas::mju_sub3(
                dx.as_mut_ptr(), (*d).subtree_com.add(3 * i as usize), (*d).subtree_com.add(3 * parent as usize));
            crate::engine::engine_util_blas::mju_sub3(
                dv.as_mut_ptr(), (*d).subtree_linvel.add(3 * i as usize), (*d).subtree_linvel.add(3 * parent as usize));
            crate::engine::engine_util_blas::mju_scl3(
                dv.as_mut_ptr(), dv.as_ptr(), *(*m).body_subtreemass.add(i as usize));
            crate::engine::engine_inline::mji_cross(dL.as_mut_ptr(), dx.as_ptr(), dv.as_ptr());

            // add to parent
            crate::engine::engine_inline::mji_add_to3(
                (*d).subtree_angmom.add(3 * parent as usize), dL.as_ptr());
        }

        crate::engine::engine_memory::mj_free_stack(d);

        // mark as computed
        (*d).flg_subtreevel = true;
    }
}

/// C: mj_rne (engine/engine_core_smooth.h:107)
/// Calls: mj_freeStack, mj_markStack, mj_stackAllocInfo, mji_crossForce, mji_dot6, mju_add, mju_addTo, mju_mulDofVec, mju_mulInertVec, mju_scl3, mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_rne(m: *const mjModel, d: *mut mjData, flg_acc: i32, result: *mut f64) {
    const MJ_ENBL_SLEEP: i32 = 1 << 4;
    const MJ_DSBL_GRAVITY: i32 = 1 << 7;

    // SAFETY: m, d, result are valid pointers (caller contract).
    unsafe {
        let sleep_filter = ((*m).opt.enableflags & MJ_ENBL_SLEEP) != 0
            && ((*d).nbody_awake as i64) < (*m).nbody;
        let nbody = if sleep_filter { (*d).nbody_awake } else { (*m).nbody as i32 };
        let nparent = if sleep_filter { (*d).nparent_awake } else { (*m).nbody as i32 };
        let nv = if sleep_filter { (*d).nv_awake } else { (*m).nv as i32 };

        crate::engine::engine_memory::mj_mark_stack(d);
        let loc_cacc = crate::engine::engine_memory::mj_stack_alloc_num(d, ((*m).nbody * 6) as usize);
        let loc_cfrc_body = crate::engine::engine_memory::mj_stack_alloc_num(d, ((*m).nbody * 6) as usize);

        // set world acceleration to -gravity
        crate::engine::engine_util_blas::mju_zero(loc_cacc, 6);
        if ((*m).opt.disableflags & MJ_DSBL_GRAVITY) == 0 {
            crate::engine::engine_util_blas::mju_scl3(
                loc_cacc.add(3), (*m).opt.gravity.as_ptr(), -1.0);
        }

        // forward pass over bodies: accumulate cacc, set cfrc_body
        for b in 1..nbody {
            let i = if sleep_filter { *(*d).body_awake_ind.add(b as usize) } else { b };

            // get body's first dof address
            let bda = *(*m).body_dofadr.add(i as usize);

            // cacc = cacc_parent + cdofdot * qvel
            let mut tmp: [f64; 6] = [0.0; 6];
            crate::engine::engine_util_spatial::mju_mul_dof_vec(
                tmp.as_mut_ptr(),
                (*d).cdof_dot.add(6 * bda as usize),
                (*d).qvel.add(bda as usize),
                *(*m).body_dofnum.add(i as usize),
            );
            crate::engine::engine_util_blas::mju_add(
                loc_cacc.add(6 * i as usize),
                loc_cacc.add(6 * *(*m).body_parentid.add(i as usize) as usize),
                tmp.as_ptr(),
                6,
            );

            // cacc += cdof * qacc
            if flg_acc != 0 {
                crate::engine::engine_util_spatial::mju_mul_dof_vec(
                    tmp.as_mut_ptr(),
                    (*d).cdof.add(6 * bda as usize),
                    (*d).qacc.add(bda as usize),
                    *(*m).body_dofnum.add(i as usize),
                );
                crate::engine::engine_util_blas::mju_add_to(
                    loc_cacc.add(6 * i as usize), tmp.as_ptr(), 6);
            }

            // cfrc_body = cinert * cacc + cvel x (cinert * cvel)
            crate::engine::engine_util_spatial::mju_mul_inert_vec(
                loc_cfrc_body.add(6 * i as usize),
                (*d).cinert.add(10 * i as usize),
                loc_cacc.add(6 * i as usize),
            );
            crate::engine::engine_util_spatial::mju_mul_inert_vec(
                tmp.as_mut_ptr(),
                (*d).cinert.add(10 * i as usize),
                (*d).cvel.add(6 * i as usize),
            );
            let mut tmp1: [f64; 6] = [0.0; 6];
            crate::engine::engine_inline::mji_cross_force(
                tmp1.as_mut_ptr(), (*d).cvel.add(6 * i as usize), tmp.as_ptr());
            crate::engine::engine_util_blas::mju_add_to(
                loc_cfrc_body.add(6 * i as usize), tmp1.as_ptr(), 6);
        }

        // clear world cfrc_body
        crate::engine::engine_util_blas::mju_zero(loc_cfrc_body, 6);

        // backward pass over bodies: accumulate cfrc_body from children
        for b in (1..nparent).rev() {
            let i = if sleep_filter { *(*d).parent_awake_ind.add(b as usize) } else { b };
            let j = *(*m).body_parentid.add(i as usize);
            if j != 0 {
                crate::engine::engine_util_blas::mju_add_to(
                    loc_cfrc_body.add(6 * j as usize),
                    loc_cfrc_body.add(6 * i as usize),
                    6,
                );
            }
        }

        // result = cdof * cfrc_body
        for v in 0..nv {
            let i = if sleep_filter { *(*d).dof_awake_ind.add(v as usize) } else { v };
            *result.add(i as usize) = crate::engine::engine_inline::mji_dot6(
                (*d).cdof.add(6 * i as usize),
                loc_cfrc_body.add(6 * *(*m).dof_bodyid.add(i as usize) as usize),
            );
        }

        crate::engine::engine_memory::mj_free_stack(d);
    }
}

/// C: mj_rnePostConstraint (engine/engine_core_smooth.h:110)
/// Calls: mj_contactForce, mj_local2Global, mji_copy3, mji_crossForce, mju_add, mju_addTo, mju_isZero, mju_message, mju_mulDofVec, mju_mulInertVec, mju_mulMatTVec3, mju_scl3, mju_sub, mju_subFrom, mju_transformSpatial, mju_zero, mju_zero3
#[allow(unused_variables, non_snake_case)]
pub fn mj_rne_post_constraint(m: *const mjModel, d: *mut mjData) {
    const MJ_DSBL_GRAVITY: i32 = 1 << 7;
    const MJCNSTR_EQUALITY: i32 = 0;
    const MJEQ_CONNECT: i32 = 0;
    const MJEQ_WELD: i32 = 1;
    const MJEQ_JOINT: i32 = 2;
    const MJEQ_TENDON: i32 = 3;
    const MJEQ_FLEX: i32 = 4;
    const MJEQ_FLEXVERT: i32 = 5;
    const MJEQ_FLEXSTRAIN: i32 = 6;
    const MJNEQDATA: i32 = 11;
    const MJOBJ_BODY: i32 = 1;

    // SAFETY: m, d are valid pointers (caller contract). All array accesses are within
    // model-allocated bounds guaranteed by mujoco's constraint construction.
    unsafe {
        let nbody = (*m).nbody;
        let mut cfrc_com: [f64; 6] = [0.0; 6];
        let mut cfrc: [f64; 6] = [0.0; 6];
        let mut lfrc: [f64; 6] = [0.0; 6];

        // clear cacc, set world acceleration to -gravity
        crate::engine::engine_util_blas::mju_zero((*d).cacc, 6);
        if ((*m).opt.disableflags & MJ_DSBL_GRAVITY) == 0 {
            crate::engine::engine_util_blas::mju_scl3(
                (*d).cacc.add(3),
                (*m).opt.gravity.as_ptr(),
                -1.0,
            );
        }

        // cfrc_ext = perturb
        crate::engine::engine_util_blas::mju_zero((*d).cfrc_ext, 6 * nbody as i32);
        for i in 1..nbody {
            if crate::engine::engine_util_misc::mju_is_zero((*d).xfrc_applied.add(6 * i as usize), 6) == 0 {
                // rearrange as torque:force
                crate::engine::engine_inline::mji_copy3(
                    cfrc.as_mut_ptr(),
                    (*d).xfrc_applied.add(6 * i as usize + 3),
                );
                crate::engine::engine_inline::mji_copy3(
                    cfrc.as_mut_ptr().add(3),
                    (*d).xfrc_applied.add(6 * i as usize),
                );

                // map force from application point to com; both world-oriented
                crate::engine::engine_util_spatial::mju_transform_spatial(
                    cfrc_com.as_mut_ptr(),
                    cfrc.as_ptr(),
                    1,
                    (*d).subtree_com.add(3 * *(*m).body_rootid.add(i as usize) as usize),
                    (*d).xipos.add(3 * i as usize),
                    std::ptr::null(),
                );

                // accumulate
                crate::engine::engine_util_blas::mju_add_to(
                    (*d).cfrc_ext.add(6 * i as usize),
                    cfrc_com.as_ptr(),
                    6,
                );
            }
        }

        // cfrc_ext += contacts
        let ncon = (*d).ncon;
        for i in 0..ncon {
            // get contact pointer
            let con = (*d).contact.add(i as usize);

            // skip excluded contacts
            if (*con).efc_address < 0 {
                continue;
            }

            // skip contact involving flex
            if (*con).geom[0] < 0 || (*con).geom[1] < 0 {
                continue;
            }

            // tmp = contact-local force:torque vector
            crate::engine::engine_core_util::mj_contact_force(
                m, d as *const mjData, i, lfrc.as_mut_ptr());

            // cfrc = world-oriented torque:force vector (swap in the process)
            crate::engine::engine_util_blas::mju_mul_mat_t_vec3(
                cfrc.as_mut_ptr(),
                (*con).frame.as_ptr(),
                lfrc.as_ptr().add(3),
            );
            crate::engine::engine_util_blas::mju_mul_mat_t_vec3(
                cfrc.as_mut_ptr().add(3),
                (*con).frame.as_ptr(),
                lfrc.as_ptr(),
            );

            // body 1
            let k = *(*m).geom_bodyid.add((*con).geom[0] as usize);
            if k != 0 {
                // tmp = subtree CoM-based torque_force vector
                crate::engine::engine_util_spatial::mju_transform_spatial(
                    cfrc_com.as_mut_ptr(),
                    cfrc.as_ptr(),
                    1,
                    (*d).subtree_com.add(3 * *(*m).body_rootid.add(k as usize) as usize),
                    (*con).pos.as_ptr(),
                    std::ptr::null(),
                );

                // apply (opposite for body 1)
                crate::engine::engine_util_blas::mju_sub_from(
                    (*d).cfrc_ext.add(6 * k as usize),
                    cfrc_com.as_ptr(),
                    6,
                );
            }

            // body 2
            let k = *(*m).geom_bodyid.add((*con).geom[1] as usize);
            if k != 0 {
                // tmp = subtree CoM-based torque_force vector
                crate::engine::engine_util_spatial::mju_transform_spatial(
                    cfrc_com.as_mut_ptr(),
                    cfrc.as_ptr(),
                    1,
                    (*d).subtree_com.add(3 * *(*m).body_rootid.add(k as usize) as usize),
                    (*con).pos.as_ptr(),
                    std::ptr::null(),
                );

                // apply
                crate::engine::engine_util_blas::mju_add_to(
                    (*d).cfrc_ext.add(6 * k as usize),
                    cfrc_com.as_ptr(),
                    6,
                );
            }
        }

        // cfrc_ext += connect, weld, flex constraints
        let mut i: i32 = 0;
        let ne = (*d).ne;
        while i < ne {
            if *(*d).efc_type.add(i as usize) != MJCNSTR_EQUALITY {
                crate::engine::engine_util_errmem::mju_error(
                    b"row of efc is not an equality constraint\0".as_ptr() as *const i8);
            }

            let id = *(*d).efc_id.add(i as usize);
            let eq_data = (*m).eq_data.add(MJNEQDATA as usize * id as usize);
            let mut pos: [f64; 3] = [0.0; 3];

            let eq_type = *(*m).eq_type.add(id as usize);
            if eq_type == MJEQ_CONNECT || eq_type == MJEQ_WELD {
                // cfrc = world-oriented torque:force vector
                crate::engine::engine_inline::mji_copy3(
                    cfrc.as_mut_ptr().add(3),
                    (*d).efc_force.add(i as usize),
                );
                if eq_type == MJEQ_WELD {
                    crate::engine::engine_inline::mji_copy3(
                        cfrc.as_mut_ptr(),
                        (*d).efc_force.add((i + 3) as usize),
                    );
                } else {
                    crate::engine::engine_util_blas::mju_zero3(cfrc.as_mut_ptr());
                }

                let body_semantic = *(*m).eq_objtype.add(id as usize) == MJOBJ_BODY;

                // body 1
                let obj1 = *(*m).eq_obj1id.add(id as usize);
                let k = if body_semantic { obj1 } else { *(*m).site_bodyid.add(obj1 as usize) };
                if k != 0 {
                    let offset: *const f64 = if body_semantic {
                        eq_data.add(3 * (if eq_type == MJEQ_WELD { 1 } else { 0 }) as usize)
                    } else {
                        (*m).site_pos.add(3 * obj1 as usize)
                    };

                    // transform point on body1: local -> global
                    crate::engine::engine_core_util::mj_local2global(
                        d, pos.as_mut_ptr(), std::ptr::null_mut(),
                        offset, std::ptr::null(), k, 0);

                    // tmp = subtree CoM-based torque_force vector
                    crate::engine::engine_util_spatial::mju_transform_spatial(
                        cfrc_com.as_mut_ptr(),
                        cfrc.as_ptr(),
                        1,
                        (*d).subtree_com.add(3 * *(*m).body_rootid.add(k as usize) as usize),
                        pos.as_ptr(),
                        std::ptr::null(),
                    );

                    // apply (opposite for body 1)
                    crate::engine::engine_util_blas::mju_add_to(
                        (*d).cfrc_ext.add(6 * k as usize),
                        cfrc_com.as_ptr(),
                        6,
                    );
                }

                // body 2
                let obj2 = *(*m).eq_obj2id.add(id as usize);
                let k = if body_semantic { obj2 } else { *(*m).site_bodyid.add(obj2 as usize) };
                if k != 0 {
                    let offset: *const f64 = if body_semantic {
                        eq_data.add(3 * (if eq_type == MJEQ_CONNECT { 1 } else { 0 }) as usize)
                    } else {
                        (*m).site_pos.add(3 * obj2 as usize)
                    };

                    // transform point on body2: local -> global
                    crate::engine::engine_core_util::mj_local2global(
                        d, pos.as_mut_ptr(), std::ptr::null_mut(),
                        offset, std::ptr::null(), k, 0);

                    // tmp = subtree CoM-based torque_force vector
                    crate::engine::engine_util_spatial::mju_transform_spatial(
                        cfrc_com.as_mut_ptr(),
                        cfrc.as_ptr(),
                        1,
                        (*d).subtree_com.add(3 * *(*m).body_rootid.add(k as usize) as usize),
                        pos.as_ptr(),
                        std::ptr::null(),
                    );

                    // apply
                    crate::engine::engine_util_blas::mju_sub_from(
                        (*d).cfrc_ext.add(6 * k as usize),
                        cfrc_com.as_ptr(),
                        6,
                    );
                }

                // increment rows
                i += if eq_type == MJEQ_WELD { 6 } else { 3 };
            } else if eq_type == MJEQ_JOINT || eq_type == MJEQ_TENDON {
                // increment 1 row
                i += 1;
            } else if eq_type == MJEQ_FLEX {
                // increment with number of non-rigid edges
                let k = *(*m).eq_obj1id.add(id as usize);
                let flex_edgeadr = *(*m).flex_edgeadr.add(k as usize);
                let flex_edgenum = *(*m).flex_edgenum.add(k as usize);
                for e in flex_edgeadr..(flex_edgeadr + flex_edgenum) {
                    if !*(*m).flexedge_rigid.add(e as usize) {
                        i += 1;
                    }
                }
            } else if eq_type == MJEQ_FLEXVERT {
                let k = *(*m).eq_obj1id.add(id as usize);
                i += 2 * *(*m).flex_vertnum.add(k as usize);
            } else if eq_type == MJEQ_FLEXSTRAIN {
                let k = *(*m).eq_obj1id.add(id as usize);
                let interp_k = *(*m).flex_interp.add(k as usize);
                let order = if interp_k < 0 { -interp_k } else { interp_k };
                let nodenum = *(*m).flex_nodenum.add(k as usize);
                if order != 0 && nodenum != 0 {
                    let nquad = order + 1;
                    let ngauss = nquad * nquad * nquad;
                    let ncells = *(*m).flex_cellnum.add(3 * k as usize)
                        * *(*m).flex_cellnum.add(3 * k as usize + 1)
                        * *(*m).flex_cellnum.add(3 * k as usize + 2);
                    i += ncells * (if order == 1 { 2 + 3 * ngauss } else { 6 * ngauss });
                }
            } else {
                crate::engine::engine_util_errmem::mju_error(
                    b"unknown constraint type\0".as_ptr() as *const i8);
            }
        }

        // forward pass over bodies: compute cacc, cfrc_int
        let mut cacc: [f64; 6] = [0.0; 6];
        let mut cfrc_body: [f64; 6] = [0.0; 6];
        let mut cfrc_corr: [f64; 6] = [0.0; 6];
        crate::engine::engine_util_blas::mju_zero((*d).cfrc_int, 6);
        for j in 1..nbody {
            // get body's first dof address
            let bda = *(*m).body_dofadr.add(j as usize);

            // cacc = cacc_parent + cdofdot * qvel + cdof * qacc
            crate::engine::engine_util_spatial::mju_mul_dof_vec(
                cacc.as_mut_ptr(),
                (*d).cdof_dot.add(6 * bda as usize),
                (*d).qvel.add(bda as usize),
                *(*m).body_dofnum.add(j as usize),
            );
            crate::engine::engine_util_blas::mju_add(
                (*d).cacc.add(6 * j as usize),
                (*d).cacc.add(6 * *(*m).body_parentid.add(j as usize) as usize),
                cacc.as_ptr(),
                6,
            );
            crate::engine::engine_util_spatial::mju_mul_dof_vec(
                cacc.as_mut_ptr(),
                (*d).cdof.add(6 * bda as usize),
                (*d).qacc.add(bda as usize),
                *(*m).body_dofnum.add(j as usize),
            );
            crate::engine::engine_util_blas::mju_add_to(
                (*d).cacc.add(6 * j as usize),
                cacc.as_ptr(),
                6,
            );

            // cfrc_body = cinert * cacc + cvel x (cinert * cvel)
            crate::engine::engine_util_spatial::mju_mul_inert_vec(
                cfrc_body.as_mut_ptr(),
                (*d).cinert.add(10 * j as usize),
                (*d).cacc.add(6 * j as usize),
            );
            crate::engine::engine_util_spatial::mju_mul_inert_vec(
                cfrc_corr.as_mut_ptr(),
                (*d).cinert.add(10 * j as usize),
                (*d).cvel.add(6 * j as usize),
            );
            crate::engine::engine_inline::mji_cross_force(
                cfrc.as_mut_ptr(),
                (*d).cvel.add(6 * j as usize),
                cfrc_corr.as_ptr(),
            );
            crate::engine::engine_util_blas::mju_add_to(
                cfrc_body.as_mut_ptr(),
                cfrc.as_ptr(),
                6,
            );

            // set cfrc_int = cfrc_body - cfrc_ext
            crate::engine::engine_util_blas::mju_sub(
                (*d).cfrc_int.add(6 * j as usize),
                cfrc_body.as_ptr(),
                (*d).cfrc_ext.add(6 * j as usize),
                6,
            );
        }

        // backward pass over bodies: accumulate cfrc_int from children
        for j in (1..nbody).rev() {
            crate::engine::engine_util_blas::mju_add_to(
                (*d).cfrc_int.add(6 * *(*m).body_parentid.add(j as usize) as usize),
                (*d).cfrc_int.add(6 * j as usize),
                6,
            );
        }

        // mark as computed
        (*d).flg_rnepost = true;
    }
}

/// C: mj_tendonBias (engine/engine_core_smooth.h:116)
/// Calls: mj_actuatorArmature, mj_sleepState, mj_tendonDot
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_tendon_bias(m: *const mjModel, d: *mut mjData, qfrc: *mut f64) {
    const MJ_ENBL_SLEEP: i32 = 1 << 4;
    const MJ_OBJ_TENDON: u32 = 18;
    const MJ_S_ASLEEP: i32 = 0;

    // SAFETY: m, d, qfrc are valid pointers (caller contract).
    unsafe {
        let sleep_filter = ((*m).opt.enableflags & MJ_ENBL_SLEEP) != 0
            && (*d).ntree_awake < (*m).ntree as i32;
        let ntendon = (*m).ntendon as i32;

        // add bias term due to tendon armature
        for i in 0..ntendon {
            // skip sleeping tendon
            if sleep_filter
                && crate::engine::engine_sleep::mj_sleep_state(
                    m, d as *const mjData, MJ_OBJ_TENDON, i) == MJ_S_ASLEEP
            {
                continue;
            }

            let armature = *(*m).tendon_armature.add(i as usize)
                + crate::engine::engine_core_util::mj_actuator_armature(m, MJ_OBJ_TENDON, i);

            // no armature: skip
            if armature == 0.0 {
                continue;
            }

            // get d/dt(tendon Jacobian) dotted with qvel for tendon i
            let dot = mj_tendon_dot(m, d, i, (*d).qvel);

            // add bias term: qfrc += ten_J * armature * dot(ten_Jdot, qvel)
            let coef = armature * dot;

            if coef != 0.0 {
                // sparse
                let nnz = *(*m).ten_J_rownnz.add(i as usize);
                let adr = *(*m).ten_J_rowadr.add(i as usize);
                let colind = (*m).ten_J_colind.add(adr as usize);
                let ten_J = (*d).ten_J.add(adr as usize);
                for j in 0..nnz {
                    *qfrc.add(*colind.add(j as usize) as usize) +=
                        coef * *ten_J.add(j as usize);
                }
            }
        }
    }
}

