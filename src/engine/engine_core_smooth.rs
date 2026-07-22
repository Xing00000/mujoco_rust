//! Port of: engine/engine_core_smooth.c
//! IR hash: 73a9f665ec0ecfc0
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
    todo!() // mj_flex
}

/// C: mj_tendon (engine/engine_core_smooth.h:47)
/// Calls: mj_freeStack, mj_jacDifPair, mj_markStack, mj_sleepState, mj_stackAllocInfo, mji_copy3, mji_copy9, mji_sub3, mju_combineSparseInc, mju_dist3, mju_message, mju_mulMatTVec, mju_normalize3, mju_round, mju_wrap, mju_zero, mju_zero3
#[allow(unused_variables, non_snake_case)]
pub fn mj_tendon(m: *const mjModel, d: *mut mjData) {
    todo!() // mj_tendon
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
    todo!() // mj_tendonDot
}

/// C: mj_transmission (engine/engine_core_smooth.h:53)
/// Calls: mj_freeStack, mj_isSparse, mj_jacDifPair, mj_jacPointAxis, mj_jacSite, mj_markStack, mj_mulJacTVec, mj_sleepState, mj_stackAllocInfo, mji_addTo3, mji_copy3, mji_copy4, mji_mulMatVec3, mji_mulQuat, mji_quat2Vel, mji_rotVecQuat, mji_subQuat, mju_addTo, mju_copyInt, mju_dot3, mju_isZero, mju_message, mju_mulMatMat, mju_mulMatTVec, mju_mulMatTVec3, mju_negQuat, mju_normalize4, mju_scl, mju_scl3, mju_sub3, mju_subFrom, mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mj_transmission(m: *const mjModel, d: *mut mjData) {
    todo!() // mj_transmission
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

