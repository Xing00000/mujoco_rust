//! Port of: engine/engine_core_smooth.c
//! IR hash: 05737965add36adb
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mj_updateDynamicBVH (engine/engine_core_smooth.c:490)
/// Calls: mj_freeStack, mj_markStack, mj_stackAllocInfo, mju_max, mju_min, mju_zeroInt
#[allow(unused_variables, non_snake_case)]
pub fn mj_update_dynamic_bvh(m: *const mjModel, d: *mut mjData, bvhadr: i32, bvhnum: i32) {
    extern "C" {
        fn mj_updateDynamicBVH_impl(m: *const mjModel, d: *mut mjData, bvhadr: i32, bvhnum: i32);
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_updateDynamicBVH_impl(m, d, bvhadr, bvhnum) }
}

/// C: mju_mulMatMat322 (engine/engine_core_smooth.c:537)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_mul_mat_mat322(C: *mut f64, A: *const f64, B: *const f64) {
    // SAFETY: C points to 6 f64, A points to 6 f64, B points to 4 f64.
    // Direct translation of C(3x2) = A(3x2) * B(2x2).
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
    // SAFETY: m, d valid per caller. All pointer arithmetic within allocated arrays.
    unsafe {
        const MJENBL_SLEEP: i32 = 1 << 4;
        const MJS_STATIC: i32 = -1;
        const MJS_ASLEEP: i32 = 0;
        const MJJNT_FREE: i32 = 0;
        const MJJNT_BALL: i32 = 1;
        const MJJNT_SLIDE: i32 = 2;
        const MJJNT_HINGE: i32 = 3;

        let nbody: i32 = (*m).nbody as i32;

        // set world position and orientation
        crate::engine::engine_util_blas::mju_zero3((*d).xpos);
        crate::engine::engine_util_blas::mju_unit4((*d).xquat);
        crate::engine::engine_util_blas::mju_zero3((*d).xipos);
        crate::engine::engine_util_blas::mju_zero((*d).xmat, 9);
        crate::engine::engine_util_blas::mju_zero((*d).ximat, 9);
        *(*d).xmat.add(0) = 1.0;
        *(*d).xmat.add(4) = 1.0;
        *(*d).xmat.add(8) = 1.0;
        *(*d).ximat.add(0) = 1.0;
        *(*d).ximat.add(4) = 1.0;
        *(*d).ximat.add(8) = 1.0;

        let sleep_filter: i32 = (((*m).opt.enableflags & MJENBL_SLEEP) != 0) as i32;

        // compute global cartesian positions and orientations of all bodies
        let mut i: i32 = 1;
        while i < nbody {
            // skip static bodies
            if sleep_filter != 0 {
                if *(*d).body_awake.add(i as usize) == MJS_STATIC {
                    i += 1;
                    continue;
                }
            }

            let mut xpos: [f64; 3] = [0.0; 3];
            let mut xquat: [f64; 4] = [0.0; 4];
            let jntadr: i32 = *(*m).body_jntadr.add(i as usize);
            let jntnum: i32 = *(*m).body_jntnum.add(i as usize);

            // free joint
            if jntnum == 1 && *(*m).jnt_type.add(jntadr as usize) == MJJNT_FREE {
                let qadr: i32 = *(*m).jnt_qposadr.add(jntadr as usize);

                // copy pos and quat from qpos
                crate::engine::engine_inline::mji_copy3(
                    xpos.as_mut_ptr(), (*d).qpos.add(qadr as usize));
                crate::engine::engine_inline::mji_copy4(
                    xquat.as_mut_ptr(), (*d).qpos.add(qadr as usize + 3));
                crate::engine::engine_util_blas::mju_normalize4(xquat.as_mut_ptr());

                // assign xanchor and xaxis
                crate::engine::engine_inline::mji_copy3(
                    (*d).xanchor.add(3 * jntadr as usize), xpos.as_ptr());
                crate::engine::engine_inline::mji_copy3(
                    (*d).xaxis.add(3 * jntadr as usize),
                    (*m).jnt_axis.add(3 * jntadr as usize));
            }
            // regular or no joint
            else {
                let pid: i32 = *(*m).body_parentid.add(i as usize);

                // get body pos and quat: from model or mocap
                let bodypos: *const f64;
                let bodyquat: *const f64;
                let mut quat: [f64; 4] = [0.0; 4];
                if *(*m).body_mocapid.add(i as usize) >= 0 {
                    bodypos = (*d).mocap_pos.add(
                        3 * *(*m).body_mocapid.add(i as usize) as usize);
                    crate::engine::engine_inline::mji_copy4(
                        quat.as_mut_ptr(),
                        (*d).mocap_quat.add(4 * *(*m).body_mocapid.add(i as usize) as usize));
                    crate::engine::engine_util_blas::mju_normalize4(quat.as_mut_ptr());
                    bodyquat = quat.as_ptr();
                } else {
                    bodypos = (*m).body_pos.add(3 * i as usize);
                    bodyquat = (*m).body_quat.add(4 * i as usize);
                }

                // apply fixed translation and rotation relative to parent
                if pid != 0 {
                    crate::engine::engine_inline::mji_mul_mat_vec3(
                        xpos.as_mut_ptr(), (*d).xmat.add(9 * pid as usize), bodypos);
                    crate::engine::engine_inline::mji_add_to3(
                        xpos.as_mut_ptr(), (*d).xpos.add(3 * pid as usize));
                    crate::engine::engine_inline::mji_mul_quat(
                        xquat.as_mut_ptr(), (*d).xquat.add(4 * pid as usize), bodyquat);
                } else {
                    // parent is the world
                    crate::engine::engine_inline::mji_copy3(xpos.as_mut_ptr(), bodypos);
                    crate::engine::engine_inline::mji_copy4(xquat.as_mut_ptr(), bodyquat);
                }

                // accumulate joints, compute xpos and xquat for this body
                let mut xanchor: [f64; 3] = [0.0; 3];
                let mut xaxis: [f64; 3] = [0.0; 3];
                let mut j: i32 = 0;
                while j < jntnum {
                    let jid: i32 = jntadr + j;
                    let qadr: i32 = *(*m).jnt_qposadr.add(jid as usize);
                    let jtype: i32 = *(*m).jnt_type.add(jid as usize);

                    // compute axis in global frame
                    crate::engine::engine_inline::mji_rot_vec_quat(
                        xaxis.as_mut_ptr(),
                        (*m).jnt_axis.add(3 * jid as usize),
                        xquat.as_ptr());

                    // compute anchor in global frame
                    crate::engine::engine_inline::mji_rot_vec_quat(
                        xanchor.as_mut_ptr(),
                        (*m).jnt_pos.add(3 * jid as usize),
                        xquat.as_ptr());
                    crate::engine::engine_inline::mji_add_to3(
                        xanchor.as_mut_ptr(), xpos.as_ptr());

                    // apply joint transformation
                    if jtype == MJJNT_SLIDE {
                        crate::engine::engine_inline::mji_add_to_scl3(
                            xpos.as_mut_ptr(), xaxis.as_ptr(),
                            *(*d).qpos.add(qadr as usize) - *(*m).qpos0.add(qadr as usize));
                    } else if jtype == MJJNT_BALL || jtype == MJJNT_HINGE {
                        // compute local quaternion rotation
                        let mut qloc: [f64; 4] = [0.0; 4];
                        if jtype == MJJNT_BALL {
                            crate::engine::engine_inline::mji_copy4(
                                qloc.as_mut_ptr(), (*d).qpos.add(qadr as usize));
                            crate::engine::engine_util_blas::mju_normalize4(qloc.as_mut_ptr());
                        } else {
                            crate::engine::engine_inline::mji_axis_angle2quat(
                                qloc.as_mut_ptr(),
                                (*m).jnt_axis.add(3 * jid as usize),
                                *(*d).qpos.add(qadr as usize) - *(*m).qpos0.add(qadr as usize));
                        }

                        // apply rotation
                        crate::engine::engine_util_spatial::mju_mul_quat(
                            xquat.as_mut_ptr(), xquat.as_ptr(), qloc.as_ptr());

                        // correct for off-center rotation
                        let mut vec: [f64; 3] = [0.0; 3];
                        crate::engine::engine_inline::mji_rot_vec_quat(
                            vec.as_mut_ptr(),
                            (*m).jnt_pos.add(3 * jid as usize),
                            xquat.as_ptr());
                        crate::engine::engine_inline::mji_sub3(
                            xpos.as_mut_ptr(), xanchor.as_ptr(), vec.as_ptr());
                    } else {
                        extern "C" {
                            fn mju_error(msg: *const i8, ...);
                        }
                        mju_error(
                            b"unknown joint type %d\0".as_ptr() as *const i8,
                            jtype);
                    }

                    // assign xanchor and xaxis
                    crate::engine::engine_inline::mji_copy3(
                        (*d).xanchor.add(3 * jid as usize), xanchor.as_ptr());
                    crate::engine::engine_inline::mji_copy3(
                        (*d).xaxis.add(3 * jid as usize), xaxis.as_ptr());

                    j += 1;
                }
            }

            // normalize quaternion
            crate::engine::engine_util_blas::mju_normalize4(xquat.as_mut_ptr());

            // sleeping body, check for mismatch
            if sleep_filter != 0 && jntnum != 0
                && *(*d).body_awake.add(i as usize) == MJS_ASLEEP
            {
                let pos: *const f64 = (*d).xpos.add(3 * i as usize);
                let xq: *const f64 = (*d).xquat.add(4 * i as usize);
                let match_: i32 = (xpos[0] == *pos.add(0)
                    && xpos[1] == *pos.add(1)
                    && xpos[2] == *pos.add(2)
                    && xquat[0] == *xq.add(0)
                    && xquat[1] == *xq.add(1)
                    && xquat[2] == *xq.add(2)
                    && xquat[3] == *xq.add(3)) as i32;

                if match_ != 0 {
                    i += 1;
                    continue;
                } else {
                    *(*d).tree_awake.add(*(*m).body_treeid.add(i as usize) as usize) = 1;
                }
            }

            // assign xquat and xpos, construct xmat
            crate::engine::engine_inline::mji_copy4(
                (*d).xquat.add(4 * i as usize), xquat.as_ptr());
            crate::engine::engine_inline::mji_copy3(
                (*d).xpos.add(3 * i as usize), xpos.as_ptr());
            crate::engine::engine_util_spatial::mju_quat2mat(
                (*d).xmat.add(9 * i as usize), xquat.as_ptr());

            i += 1;
        }
    }
}

/// C: mj_kinematics2 (engine/engine_core_smooth.h:32)
/// Calls: mj_local2Global
#[allow(unused_variables, non_snake_case)]
pub fn mj_kinematics2(m: *const mjModel, d: *mut mjData) {
    // SAFETY: m, d valid per caller. All pointer arithmetic within allocated arrays.
    unsafe {
        const MJENBL_SLEEP: i32 = 1 << 4;
        const MJS_AWAKE: i32 = 1;

        let sleep_filter: i32 = (((*m).opt.enableflags & MJENBL_SLEEP) != 0
            && (*d).nbody_awake < (*m).nbody as i32) as i32;
        let nbody: i32 = if sleep_filter != 0 { (*d).nbody_awake } else { (*m).nbody as i32 };

        // compute/copy Cartesian positions and orientations of body inertial frames
        let mut b: i32 = 1;
        while b < nbody {
            let i: i32 = if sleep_filter != 0 { *(*d).body_awake_ind.add(b as usize) } else { b };

            crate::engine::engine_core_util::mj_local2global(
                d,
                (*d).xipos.add(3 * i as usize),
                (*d).ximat.add(9 * i as usize),
                (*m).body_ipos.add(3 * i as usize),
                (*m).body_iquat.add(4 * i as usize),
                i,
                *(*m).body_sameframe.add(i as usize),
            );
            b += 1;
        }

        // compute/copy Cartesian positions and orientations of geoms
        b = 0;
        while b < nbody {
            let i: i32 = if sleep_filter != 0 { *(*d).body_awake_ind.add(b as usize) } else { b };

            // skip geom in sleeping or static body
            if sleep_filter != 0 && *(*d).body_awake.add(i as usize) != MJS_AWAKE {
                b += 1;
                continue;
            }

            let start: i32 = *(*m).body_geomadr.add(i as usize);
            let end: i32 = start + *(*m).body_geomnum.add(i as usize);
            let mut g: i32 = start;
            while g < end {
                crate::engine::engine_core_util::mj_local2global(
                    d,
                    (*d).geom_xpos.add(3 * g as usize),
                    (*d).geom_xmat.add(9 * g as usize),
                    (*m).geom_pos.add(3 * g as usize),
                    (*m).geom_quat.add(4 * g as usize),
                    *(*m).geom_bodyid.add(g as usize),
                    *(*m).geom_sameframe.add(g as usize),
                );
                g += 1;
            }
            b += 1;
        }

        // compute/copy Cartesian positions and orientations of sites
        let nsite: i32 = (*m).nsite as i32;
        let mut i: i32 = 0;
        while i < nsite {
            let bodyid: i32 = *(*m).site_bodyid.add(i as usize);

            // skip site in sleeping or static body
            if sleep_filter != 0 && *(*d).body_awake.add(bodyid as usize) != MJS_AWAKE {
                i += 1;
                continue;
            }

            crate::engine::engine_core_util::mj_local2global(
                d,
                (*d).site_xpos.add(3 * i as usize),
                (*d).site_xmat.add(9 * i as usize),
                (*m).site_pos.add(3 * i as usize),
                (*m).site_quat.add(4 * i as usize),
                bodyid,
                *(*m).site_sameframe.add(i as usize),
            );
            i += 1;
        }
    }
}

/// C: mj_kinematics (engine/engine_core_smooth.h:35)
/// Calls: mj_kinematics1, mj_kinematics2, mj_updateSleep, mj_wake
#[allow(unused_variables, non_snake_case)]
pub fn mj_kinematics(m: *const mjModel, d: *mut mjData) {
    extern "C" { fn mj_kinematics_impl(m: *const mjModel, d: *mut mjData); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_kinematics_impl(m, d) }
}

/// C: mj_comPos (engine/engine_core_smooth.h:38)
/// Calls: mji_addTo3, mji_copy3, mji_scl3, mji_sub3, mju_dofCom, mju_inertCom, mju_scl3, mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mj_com_pos(m: *const mjModel, d: *mut mjData) {
    // SAFETY: m, d valid per caller. All pointer arithmetic stays within allocated arrays.
    unsafe {
        const MJMINVAL: f64 = 1e-15;
        const MJENBL_SLEEP: i32 = 1 << 4;
        const MJS_ASLEEP: i32 = 0;
        const MJJNT_FREE: i32 = 0;
        const MJJNT_BALL: i32 = 1;
        const MJJNT_SLIDE: i32 = 2;
        const MJJNT_HINGE: i32 = 3;

        let sleep_filter: i32 = (((*m).opt.enableflags & MJENBL_SLEEP) != 0 && (*d).nbody_awake < (*m).nbody as i32) as i32;
        let nbody: i32 = if sleep_filter != 0 { (*d).nbody_awake } else { (*m).nbody as i32 };
        let nparent: i32 = if sleep_filter != 0 { (*d).nparent_awake } else { (*m).nbody as i32 };

        // subtree_com: initialize with body moment
        let mut b: i32 = 0;
        while b < nbody {
            let i = if sleep_filter != 0 { *(*d).body_awake_ind.add(b as usize) } else { b };
            crate::engine::engine_inline::mji_scl3(
                (*d).subtree_com.add(3 * i as usize),
                (*d).xipos.add(3 * i as usize),
                *(*m).body_mass.add(i as usize),
            );
            b += 1;
        }

        // subtree_com: accumulate to parent in backward pass
        b = nparent - 1;
        while b >= 0 {
            let i = if sleep_filter != 0 { *(*d).parent_awake_ind.add(b as usize) } else { b };
            if i == 0 {
                b -= 1;
                continue;
            }

            // accumulate moment to parent, rescale if sleeping
            let parent = *(*m).body_parentid.add(i as usize);
            if sleep_filter != 0 && *(*d).body_awake.add(i as usize) == MJS_ASLEEP {
                let mut child_moment: [f64; 3] = [0.0; 3];
                crate::engine::engine_inline::mji_scl3(
                    child_moment.as_mut_ptr(),
                    (*d).subtree_com.add(3 * i as usize),
                    *(*m).body_subtreemass.add(i as usize),
                );
                crate::engine::engine_inline::mji_add_to3(
                    (*d).subtree_com.add(3 * parent as usize),
                    child_moment.as_ptr(),
                );
            } else {
                crate::engine::engine_inline::mji_add_to3(
                    (*d).subtree_com.add(3 * parent as usize),
                    (*d).subtree_com.add(3 * i as usize),
                );
            }
            b -= 1;
        }

        // subtree_com: normalize
        b = 0;
        while b < nbody {
            let i = if sleep_filter != 0 { *(*d).body_awake_ind.add(b as usize) } else { b };
            if *(*m).body_subtreemass.add(i as usize) < MJMINVAL {
                crate::engine::engine_inline::mji_copy3(
                    (*d).subtree_com.add(3 * i as usize),
                    (*d).xipos.add(3 * i as usize),
                );
            } else {
                crate::engine::engine_util_blas::mju_scl3(
                    (*d).subtree_com.add(3 * i as usize),
                    (*d).subtree_com.add(3 * i as usize),
                    1.0 / *(*m).body_subtreemass.add(i as usize),
                );
            }
            b += 1;
        }

        // zero out CoM frame inertia for the world body
        crate::engine::engine_util_blas::mju_zero((*d).cinert, 10);

        // map inertias to frame centered at subtree_com
        b = 1;
        while b < nbody {
            let i = if sleep_filter != 0 { *(*d).body_awake_ind.add(b as usize) } else { b };
            let mut offset: [f64; 3] = [0.0; 3];
            crate::engine::engine_inline::mji_sub3(
                offset.as_mut_ptr(),
                (*d).xipos.add(3 * i as usize),
                (*d).subtree_com.add(3 * *(*m).body_rootid.add(i as usize) as usize),
            );
            crate::engine::engine_util_spatial::mju_inert_com(
                (*d).cinert.add(10 * i as usize),
                (*m).body_inertia.add(3 * i as usize),
                (*d).ximat.add(9 * i as usize),
                offset.as_ptr(),
                *(*m).body_mass.add(i as usize),
            );
            b += 1;
        }

        // map motion dofs to global frame centered at subtree_com
        b = 1;
        while b < nbody {
            let i = if sleep_filter != 0 { *(*d).body_awake_ind.add(b as usize) } else { b };

            let jntnum = *(*m).body_jntnum.add(i as usize);
            if jntnum == 0 {
                b += 1;
                continue;
            }

            let start = *(*m).body_jntadr.add(i as usize);
            let end = start + jntnum;
            let mut j = start;
            while j < end {
                // get cdof address
                let da = 6 * *(*m).jnt_dofadr.add(j as usize);

                // compute com-anchor vector
                let mut offset: [f64; 3] = [0.0; 3];
                let mut axis: [f64; 3] = [0.0; 3];
                crate::engine::engine_inline::mji_sub3(
                    offset.as_mut_ptr(),
                    (*d).subtree_com.add(3 * *(*m).body_rootid.add(i as usize) as usize),
                    (*d).xanchor.add(3 * j as usize),
                );

                // create motion dof
                let jnt_type = *(*m).jnt_type.add(j as usize);
                if jnt_type == MJJNT_FREE {
                    // translation components: x, y, z in global frame
                    crate::engine::engine_util_blas::mju_zero((*d).cdof.add(da as usize), 18);
                    *(*d).cdof.add((da + 3 + 7 * 0) as usize) = 1.0;
                    *(*d).cdof.add((da + 3 + 7 * 1) as usize) = 1.0;
                    *(*d).cdof.add((da + 3 + 7 * 2) as usize) = 1.0;

                    // rotation components: same as ball (fallthrough)
                    let mut k: i32 = 0;
                    while k < 3 {
                        axis[0] = *(*d).xmat.add((9 * i + k + 0) as usize);
                        axis[1] = *(*d).xmat.add((9 * i + k + 3) as usize);
                        axis[2] = *(*d).xmat.add((9 * i + k + 6) as usize);
                        crate::engine::engine_util_spatial::mju_dof_com(
                            (*d).cdof.add((da + 18 + 6 * k) as usize),
                            axis.as_ptr(),
                            offset.as_ptr(),
                        );
                        k += 1;
                    }
                } else if jnt_type == MJJNT_BALL {
                    let mut k: i32 = 0;
                    while k < 3 {
                        axis[0] = *(*d).xmat.add((9 * i + k + 0) as usize);
                        axis[1] = *(*d).xmat.add((9 * i + k + 3) as usize);
                        axis[2] = *(*d).xmat.add((9 * i + k + 6) as usize);
                        crate::engine::engine_util_spatial::mju_dof_com(
                            (*d).cdof.add((da + 6 * k) as usize),
                            axis.as_ptr(),
                            offset.as_ptr(),
                        );
                        k += 1;
                    }
                } else if jnt_type == MJJNT_SLIDE {
                    crate::engine::engine_util_spatial::mju_dof_com(
                        (*d).cdof.add(da as usize),
                        (*d).xaxis.add(3 * j as usize),
                        core::ptr::null(),
                    );
                } else if jnt_type == MJJNT_HINGE {
                    crate::engine::engine_util_spatial::mju_dof_com(
                        (*d).cdof.add(da as usize),
                        (*d).xaxis.add(3 * j as usize),
                        offset.as_ptr(),
                    );
                }

                j += 1;
            }
            b += 1;
        }
    }
}

/// C: mj_camlight (engine/engine_core_smooth.h:41)
/// Calls: mj_local2Global, mji_add3, mji_copy3, mji_copy9, mji_cross, mji_rotVecQuat, mji_sub3, mju_normalize3, mju_transpose
#[allow(unused_variables, non_snake_case)]
pub fn mj_camlight(m: *const mjModel, d: *mut mjData) {
    // SAFETY: m, d valid per caller. All pointer arithmetic within allocated arrays.
    unsafe {
        const MJENBL_SLEEP: i32 = 1 << 4;
        const MJS_AWAKE: i32 = 1;
        const CAMLIGHT_FIXED: i32 = 0;
        const CAMLIGHT_TRACK: i32 = 1;
        const CAMLIGHT_TRACKCOM: i32 = 2;
        const CAMLIGHT_TARGETBODY: i32 = 3;
        const CAMLIGHT_TARGETBODYCOM: i32 = 4;

        let ncam: i32 = (*m).ncam as i32;
        let nlight: i32 = (*m).nlight as i32;
        let sleep_filter: i32 = (((*m).opt.enableflags & MJENBL_SLEEP) != 0
            && (*d).nbody_awake < (*m).nbody as i32) as i32;

        // compute Cartesian positions and orientations of cameras
        let mut i: i32 = 0;
        while i < ncam {
            let id: i32 = *(*m).cam_bodyid.add(i as usize);
            let id1: i32 = *(*m).cam_targetbodyid.add(i as usize);

            // skip camera if both body and target body are asleep or static
            if sleep_filter != 0 && *(*d).body_awake.add(id as usize) != MJS_AWAKE {
                if id1 < 0 || *(*d).body_awake.add(id1 as usize) != MJS_AWAKE {
                    i += 1;
                    continue;
                }
            }

            // default processing for fixed mode
            crate::engine::engine_core_util::mj_local2global(
                d,
                (*d).cam_xpos.add(3 * i as usize),
                (*d).cam_xmat.add(9 * i as usize),
                (*m).cam_pos.add(3 * i as usize),
                (*m).cam_quat.add(4 * i as usize),
                id, 0,
            );

            // adjust for mode
            let mode: i32 = *(*m).cam_mode.add(i as usize);
            if mode == CAMLIGHT_FIXED {
                // nothing
            } else if mode == CAMLIGHT_TRACK || mode == CAMLIGHT_TRACKCOM {
                // fixed global orientation
                crate::engine::engine_inline::mji_copy9(
                    (*d).cam_xmat.add(9 * i as usize),
                    (*m).cam_mat0.add(9 * i as usize),
                );

                if mode == CAMLIGHT_TRACK {
                    // position: track camera body
                    crate::engine::engine_inline::mji_add3(
                        (*d).cam_xpos.add(3 * i as usize),
                        (*d).xpos.add(3 * id as usize),
                        (*m).cam_pos0.add(3 * i as usize),
                    );
                } else {
                    // position: track subtree com
                    crate::engine::engine_inline::mji_add3(
                        (*d).cam_xpos.add(3 * i as usize),
                        (*d).subtree_com.add(3 * id as usize),
                        (*m).cam_poscom0.add(3 * i as usize),
                    );
                }
            } else if mode == CAMLIGHT_TARGETBODY || mode == CAMLIGHT_TARGETBODYCOM {
                // only if target body is specified
                if id1 >= 0 {
                    let mut pos: [f64; 3] = [0.0; 3];
                    // get position to look at
                    if mode == CAMLIGHT_TARGETBODY {
                        crate::engine::engine_inline::mji_copy3(
                            pos.as_mut_ptr(),
                            (*d).xpos.add(3 * id1 as usize),
                        );
                    } else {
                        crate::engine::engine_inline::mji_copy3(
                            pos.as_mut_ptr(),
                            (*d).subtree_com.add(3 * id1 as usize),
                        );
                    }

                    // zaxis = -desired camera direction, in global frame
                    let mut matT: [f64; 9] = [0.0; 9];
                    crate::engine::engine_inline::mji_sub3(
                        matT.as_mut_ptr().add(6),
                        (*d).cam_xpos.add(3 * i as usize),
                        pos.as_ptr(),
                    );
                    crate::engine::engine_util_blas::mju_normalize3(matT.as_mut_ptr().add(6));

                    // xaxis: orthogonal to zaxis and to (0,0,1)
                    matT[3] = 0.0;
                    matT[4] = 0.0;
                    matT[5] = 1.0;
                    crate::engine::engine_inline::mji_cross(
                        matT.as_mut_ptr(),
                        matT.as_ptr().add(3),
                        matT.as_ptr().add(6),
                    );
                    crate::engine::engine_util_blas::mju_normalize3(matT.as_mut_ptr());

                    // yaxis: orthogonal to xaxis and zaxis
                    crate::engine::engine_inline::mji_cross(
                        matT.as_mut_ptr().add(3),
                        matT.as_ptr().add(6),
                        matT.as_ptr(),
                    );
                    crate::engine::engine_util_blas::mju_normalize3(matT.as_mut_ptr().add(3));

                    // set camera frame
                    crate::engine::engine_util_blas::mju_transpose(
                        (*d).cam_xmat.add(9 * i as usize),
                        matT.as_ptr(),
                        3, 3,
                    );
                }
            }

            i += 1;
        }

        // compute Cartesian positions and directions of lights
        let mut i: i32 = 0;
        while i < nlight {
            let id: i32 = *(*m).light_bodyid.add(i as usize);
            let id1: i32 = *(*m).light_targetbodyid.add(i as usize);

            // skip light if both body and target body are asleep or static
            if sleep_filter != 0 && *(*d).body_awake.add(id as usize) != MJS_AWAKE {
                if id1 < 0 || *(*d).body_awake.add(id1 as usize) != MJS_AWAKE {
                    i += 1;
                    continue;
                }
            }

            // default processing for fixed mode
            crate::engine::engine_core_util::mj_local2global(
                d,
                (*d).light_xpos.add(3 * i as usize),
                core::ptr::null_mut(),
                (*m).light_pos.add(3 * i as usize),
                core::ptr::null(),
                id, 0,
            );
            crate::engine::engine_inline::mji_rot_vec_quat(
                (*d).light_xdir.add(3 * i as usize),
                (*m).light_dir.add(3 * i as usize),
                (*d).xquat.add(4 * id as usize),
            );

            // adjust for mode
            let mode: i32 = *(*m).light_mode.add(i as usize);
            if mode == CAMLIGHT_FIXED {
                // nothing
            } else if mode == CAMLIGHT_TRACK || mode == CAMLIGHT_TRACKCOM {
                // fixed global orientation
                crate::engine::engine_inline::mji_copy3(
                    (*d).light_xdir.add(3 * i as usize),
                    (*m).light_dir0.add(3 * i as usize),
                );

                if mode == CAMLIGHT_TRACK {
                    // position: track light body
                    crate::engine::engine_inline::mji_add3(
                        (*d).light_xpos.add(3 * i as usize),
                        (*d).xpos.add(3 * id as usize),
                        (*m).light_pos0.add(3 * i as usize),
                    );
                } else {
                    // position: track subtree com
                    crate::engine::engine_inline::mji_add3(
                        (*d).light_xpos.add(3 * i as usize),
                        (*d).subtree_com.add(3 * id as usize),
                        (*m).light_poscom0.add(3 * i as usize),
                    );
                }
            } else if mode == CAMLIGHT_TARGETBODY || mode == CAMLIGHT_TARGETBODYCOM {
                // only if target body is specified
                if id1 >= 0 {
                    let mut lookat: [f64; 3] = [0.0; 3];
                    if mode == CAMLIGHT_TARGETBODY {
                        crate::engine::engine_inline::mji_copy3(
                            lookat.as_mut_ptr(),
                            (*d).xpos.add(3 * id1 as usize),
                        );
                    } else {
                        crate::engine::engine_inline::mji_copy3(
                            lookat.as_mut_ptr(),
                            (*d).subtree_com.add(3 * id1 as usize),
                        );
                    }

                    // set dir
                    crate::engine::engine_inline::mji_sub3(
                        (*d).light_xdir.add(3 * i as usize),
                        lookat.as_ptr(),
                        (*d).light_xpos.add(3 * i as usize),
                    );
                }
            }

            // normalize dir
            crate::engine::engine_util_blas::mju_normalize3((*d).light_xdir.add(3 * i as usize));

            i += 1;
        }
    }
}

/// C: mj_flex (engine/engine_core_smooth.h:44)
/// Calls: mj_bodyChain, mj_freeStack, mj_jacDifPair, mj_jacSparse, mj_markStack, mj_stackAllocInfo, mj_updateDynamicBVH, mji_addTo3, mji_copy3, mji_copy6, mji_mulMatVec3, mji_sub3, mju_cellLookup, mju_interpolate3D, mju_max, mju_message, mju_min, mju_mulMatMat322, mju_mulMatTVec, mju_mulMatVec, mju_normalize3, mju_scl3, mju_shellTrackInterior, mju_sub3, mju_zero, mju_zero3
#[allow(unused_variables, non_snake_case)]
pub fn mj_flex(m: *const mjModel, d: *mut mjData) {
    extern "C" {
        fn mj_flex_impl(m: *const mjModel, d: *mut mjData);
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_flex_impl(m, d) }
}

/// C: mj_tendon (engine/engine_core_smooth.h:47)
/// Calls: mj_freeStack, mj_jacDifPair, mj_markStack, mj_sleepState, mj_stackAllocInfo, mji_copy3, mji_copy9, mji_sub3, mju_combineSparseInc, mju_dist3, mju_message, mju_mulMatTVec, mju_normalize3, mju_round, mju_wrap, mju_zero, mju_zero3
#[allow(unused_variables, non_snake_case)]
pub fn mj_tendon(m: *const mjModel, d: *mut mjData) {
    extern "C" { fn mj_tendon_impl(m: *const mjModel, d: *mut mjData); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_tendon_impl(m, d) }
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
    extern "C" { fn mj_tendonDot_impl(m: *const mjModel, d: *mut mjData, id: i32, vec: *const f64) -> f64; }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_tendonDot_impl(m, d, id, vec) }
}

/// C: mj_transmission (engine/engine_core_smooth.h:53)
/// Calls: mj_freeStack, mj_isSparse, mj_jacDifPair, mj_jacPointAxis, mj_jacSite, mj_markStack, mj_mulJacTVec, mj_sleepState, mj_stackAllocInfo, mji_addTo3, mji_copy3, mji_copy4, mji_mulMatVec3, mji_mulQuat, mji_quat2Vel, mji_rotVecQuat, mji_subQuat, mju_addTo, mju_copyInt, mju_dot3, mju_isZero, mju_message, mju_mulMatMat, mju_mulMatTVec, mju_mulMatTVec3, mju_negQuat, mju_normalize4, mju_scl, mju_scl3, mju_sub3, mju_subFrom, mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mj_transmission(m: *const mjModel, d: *mut mjData) {
    extern "C" { fn mj_transmission_impl(m: *const mjModel, d: *mut mjData); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_transmission_impl(m, d) }
}

/// C: mj_crb (engine/engine_core_smooth.h:59)
/// Calls: mj_actuatorArmature, mji_dot6, mju_addTo, mju_copy, mju_copyRows, mju_mulInertVec, mju_zero, mju_zeroSparse
#[allow(unused_variables, non_snake_case)]
pub fn mj_crb(m: *const mjModel, d: *mut mjData) {
    extern "C" {
        fn mj_crb_impl(m: *const mjModel, d: *mut mjData);
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_crb_impl(m, d) }
}

/// C: mj_tendonArmature (engine/engine_core_smooth.h:62)
/// Calls: mj_actuatorArmature, mj_sleepState, mju_addToSclSparseInc
#[allow(unused_variables, non_snake_case)]
pub fn mj_tendon_armature(m: *const mjModel, d: *mut mjData) {
    extern "C" { fn mj_tendonArmature_impl(m: *const mjModel, d: *mut mjData); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_tendonArmature_impl(m, d) }
}

/// C: mj_makeM (engine/engine_core_smooth.h:65)
/// Calls: mj_crb, mj_tendonArmature, mju_scatter
#[allow(unused_variables, non_snake_case)]
pub fn mj_make_m(m: *const mjModel, d: *mut mjData) {
    extern "C" { fn mj_makeM_impl(m: *const mjModel, d: *mut mjData); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_makeM_impl(m, d) }
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
    extern "C" {
        fn mj_factorI_legacy_impl(m: *const mjModel, d: *mut mjData, M: *const f64, qLD: *mut f64, qLDiagInv: *mut f64);
    }
    // SAFETY: delegates to C implementation which accesses mjModel sparse matrix layout
    unsafe { mj_factorI_legacy_impl(m, d, M, qLD, qLDiagInv) }
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
    // SAFETY: all pointers valid. mat is sparse matrix. diaginv may be null.
    // index may be null (identity permutation).
    unsafe {
        // backward loop over rows
        let mut j: i32 = nv - 1;
        while j >= 0 {
            let k = if !index.is_null() { *index.add(j as usize) } else { j };

            // get row k's address, diagonal index, inverse diagonal value
            let start = *rowadr.add(k as usize);
            let diag = *rownnz.add(k as usize) - 1;
            let end = start + diag;
            let invD = 1.0 / *mat.add(end as usize);
            if !diaginv.is_null() {
                *diaginv.add(k as usize) = invD;
            }

            // update triangle above row k
            let mut adr = end - 1;
            while adr >= start {
                // update row i < k
                let i = *colind.add(adr as usize);
                crate::engine::engine_util_blas::mju_add_to_scl(
                    mat.add(*rowadr.add(i as usize) as usize),
                    mat.add(start as usize),
                    -*mat.add(adr as usize) * invD,
                    *rownnz.add(i as usize),
                );
                adr -= 1;
            }

            // update row k: L(k, :) /= L(k, k)
            crate::engine::engine_util_blas::mju_scl(
                mat.add(start as usize),
                mat.add(start as usize),
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
    // SAFETY: m, d valid. Copies M into qLD then factorizes in-place.
    unsafe {
        const MJENBL_SLEEP: i32 = 1 << 4;

        // sleep filtering
        let sleep_filter = ((*m).opt.enableflags & MJENBL_SLEEP) != 0 && (*d).nv_awake < (*m).nv as i32;
        let index: *const i32;
        let nv: i32;

        if !sleep_filter {
            index = core::ptr::null();
            nv = (*m).nv as i32;
            crate::engine::engine_util_blas::mju_copy((*d).qLD, (*d).M, (*m).nC as i32);
        } else {
            index = (*d).dof_awake_ind;
            nv = (*d).nv_awake;
            crate::engine::engine_util_sparse::mju_copy_sparse(
                (*d).qLD, (*d).M, (*m).M_rownnz, (*m).M_rowadr, (*d).dof_awake_ind, (*d).nv_awake);
        }

        // factorize
        mj_factor_i((*d).qLD, (*d).qLDiagInv, nv, (*m).M_rownnz, (*m).M_rowadr, (*m).M_colind, index);
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
    extern "C" {
        fn mj_solveLD_legacy_impl(m: *const mjModel, x: *mut f64, n: i32, qLD: *const f64, qLDiagInv: *const f64);
    }
    // SAFETY: delegates to C implementation which accesses mjModel sparse matrix layout
    unsafe { mj_solveLD_legacy_impl(m, x, n, qLD, qLDiagInv) }
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
    // SAFETY: all pointers valid per caller contract; index may be null
    unsafe {
        // x <- L^-T x
        let mut k: i32 = nv - 1;
        while k >= 0 {
            let i = if !index.is_null() { *index.add(k as usize) } else { k };
            if *rownnz.add(i as usize) == 1 {
                k -= 1;
                continue;
            }
            if n == 1 {
                let x_i = *x.add(i as usize);
                if x_i != 0.0 {
                    let start = *rowadr.add(i as usize);
                    let end = start + *rownnz.add(i as usize) - 1;
                    let mut adr = start;
                    while adr < end {
                        *x.add(*colind.add(adr as usize) as usize) -= *qLD.add(adr as usize) * x_i;
                        adr += 1;
                    }
                }
            } else {
                let start = *rowadr.add(i as usize);
                let end = start + *rownnz.add(i as usize) - 1;
                let mut offset: i32 = 0;
                while offset < n * nv {
                    let x_i = *x.add((i + offset) as usize);
                    if x_i != 0.0 {
                        let mut adr = start;
                        while adr < end {
                            *x.add((offset + *colind.add(adr as usize)) as usize) -= *qLD.add(adr as usize) * x_i;
                            adr += 1;
                        }
                    }
                    offset += nv;
                }
            }
            k -= 1;
        }

        // x <- D^-1 x
        let mut k: i32 = 0;
        while k < nv {
            let i = if !index.is_null() { *index.add(k as usize) } else { k };
            let inv_d_i = *qLDiagInv.add(i as usize);
            if n == 1 {
                *x.add(i as usize) *= inv_d_i;
            } else {
                let mut offset: i32 = 0;
                while offset < n * nv {
                    *x.add((i + offset) as usize) *= inv_d_i;
                    offset += nv;
                }
            }
            k += 1;
        }

        // x <- L^-1 x
        let mut k: i32 = 0;
        while k < nv {
            let i = if !index.is_null() { *index.add(k as usize) } else { k };
            if *rownnz.add(i as usize) == 1 {
                k += 1;
                continue;
            }
            let d = *rownnz.add(i as usize) - 1;
            if d > 0 {
                let adr = *rowadr.add(i as usize);
                if n == 1 {
                    *x.add(i as usize) -= crate::engine::engine_util_sparse::mju_dot_sparse(
                        qLD.add(adr as usize), x, d, colind.add(adr as usize));
                } else {
                    let mut offset: i32 = 0;
                    while offset < n * nv {
                        *x.add((i + offset) as usize) -= crate::engine::engine_util_sparse::mju_dot_sparse(
                            qLD.add(adr as usize), x.add(offset as usize), d, colind.add(adr as usize));
                        offset += nv;
                    }
                }
            }
            k += 1;
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
    // SAFETY: all pointers valid per caller contract. m, d, x, y are non-null
    // and properly aligned. nv is the number of DOFs.
    unsafe {
        let nv = (*m).nv as i32;
        if x != y as *mut f64 {
            crate::engine::engine_util_blas::mju_copy(x, y, n * nv);
        }
        mj_solve_ld(
            x, (*d).qLD, (*d).qLDiagInv, nv, n,
            (*m).M_rownnz, (*m).M_rowadr, (*m).M_colind, core::ptr::null(),
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
    // SAFETY: all pointers valid per caller contract. Arrays have adequate capacity.
    unsafe {
        let nv = (*m).nv as i32;

        // local copies of key variables
        let rownnz = (*m).M_rownnz;
        let rowadr = (*m).M_rowadr;
        let colind = (*m).M_colind;
        let diagnum = (*m).dof_simplenum;
        let qLD = (*d).qLD;

        // x = y
        crate::engine::engine_util_blas::mju_copy(x, y, n * nv);

        // x <- L^-T x
        let mut i: i32 = nv - 1;
        while i > 0 {
            // skip diagonal rows
            if *diagnum.add(i as usize) != 0 {
                i -= 1;
                continue;
            }

            // prepare row i column address range
            let start = *rowadr.add(i as usize);
            let end = start + *rownnz.add(i as usize) - 1;

            // process all vectors
            let mut offset: i32 = 0;
            while offset < n * nv {
                let x_i = *x.add((i + offset) as usize);
                if x_i != 0.0 {
                    let mut adr = start;
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
        i = 0;
        while i < nv {
            let invD_i = *sqrtInvD.add(i as usize);
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
    // SAFETY: m, d valid per caller. All pointer arithmetic within allocated arrays.
    unsafe {
        const MJENBL_SLEEP: i32 = 1 << 4;
        const MJJNT_FREE: i32 = 0;
        const MJJNT_BALL: i32 = 1;

        let sleep_filter: i32 = (((*m).opt.enableflags & MJENBL_SLEEP) != 0
            && (*d).nbody_awake < (*m).nbody as i32) as i32;
        let nbody: i32 = if sleep_filter != 0 { (*d).nbody_awake } else { (*m).nbody as i32 };

        // set world vel to 0
        crate::engine::engine_util_blas::mju_zero((*d).cvel, 6);

        // forward pass over bodies
        let mut b: i32 = 1;
        while b < nbody {
            let i: i32 = if sleep_filter != 0 { *(*d).body_awake_ind.add(b as usize) } else { b };

            // cvel = cvel_parent
            let mut cvel: [f64; 6] = [0.0; 6];
            crate::engine::engine_inline::mji_copy6(
                cvel.as_mut_ptr(),
                (*d).cvel.add(6 * *(*m).body_parentid.add(i as usize) as usize),
            );

            // cvel = cvel_parent + cdof * qvel,  cdofdot = cvel x cdof
            let dofnum: i32 = *(*m).body_dofnum.add(i as usize);
            let bda: i32 = *(*m).body_dofadr.add(i as usize);
            let mut cdofdot: [f64; 36] = [0.0; 36];
            let mut j: i32 = 0;
            while j < dofnum {
                let mut tmp: [f64; 6] = [0.0; 6];
                let jnt_type: i32 = *(*m).jnt_type.add(*(*m).dof_jntid.add((bda + j) as usize) as usize);

                if jnt_type == MJJNT_FREE {
                    // cdofdot = 0
                    crate::engine::engine_util_blas::mju_zero(cdofdot.as_mut_ptr(), 18);

                    // update velocity
                    crate::engine::engine_util_spatial::mju_mul_dof_vec(
                        tmp.as_mut_ptr(),
                        (*d).cdof.add(6 * bda as usize),
                        (*d).qvel.add(bda as usize),
                        3,
                    );
                    crate::engine::engine_util_blas::mju_add_to(cvel.as_mut_ptr(), tmp.as_ptr(), 6);

                    // continue with rotations
                    j += 3;

                    // fallthrough to BALL
                    crate::engine::engine_inline::mji_cross_motion(
                        cdofdot.as_mut_ptr().add(6 * (j + 0) as usize),
                        cvel.as_ptr(),
                        (*d).cdof.add(6 * (bda + j + 0) as usize),
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
                        tmp.as_mut_ptr(),
                        (*d).cdof.add(6 * (bda + j) as usize),
                        (*d).qvel.add((bda + j) as usize),
                        3,
                    );
                    crate::engine::engine_util_blas::mju_add_to(cvel.as_mut_ptr(), tmp.as_ptr(), 6);

                    // adjust for 3-dof joint
                    j += 2;
                } else if jnt_type == MJJNT_BALL {
                    // compute all 3 cdofdots using parent velocity
                    crate::engine::engine_inline::mji_cross_motion(
                        cdofdot.as_mut_ptr().add(6 * (j + 0) as usize),
                        cvel.as_ptr(),
                        (*d).cdof.add(6 * (bda + j + 0) as usize),
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
                        tmp.as_mut_ptr(),
                        (*d).cdof.add(6 * (bda + j) as usize),
                        (*d).qvel.add((bda + j) as usize),
                        3,
                    );
                    crate::engine::engine_util_blas::mju_add_to(cvel.as_mut_ptr(), tmp.as_ptr(), 6);

                    // adjust for 3-dof joint
                    j += 2;
                } else {
                    // crossMotion(cdof, cdof) = 0, using old velocity may be more accurate
                    crate::engine::engine_inline::mji_cross_motion(
                        cdofdot.as_mut_ptr().add(6 * j as usize),
                        cvel.as_ptr(),
                        (*d).cdof.add(6 * (bda + j) as usize),
                    );

                    // update velocity
                    crate::engine::engine_util_spatial::mju_mul_dof_vec(
                        tmp.as_mut_ptr(),
                        (*d).cdof.add(6 * (bda + j) as usize),
                        (*d).qvel.add((bda + j) as usize),
                        1,
                    );
                    crate::engine::engine_util_blas::mju_add_to(cvel.as_mut_ptr(), tmp.as_ptr(), 6);
                }

                j += 1;
            }

            // assign cvel, cdofdot
            crate::engine::engine_inline::mji_copy6((*d).cvel.add(6 * i as usize), cvel.as_ptr());
            crate::engine::engine_util_blas::mju_copy(
                (*d).cdof_dot.add(6 * bda as usize),
                cdofdot.as_ptr(),
                6 * dofnum,
            );
            b += 1;
        }
    }
}

/// C: mj_subtreeVel (engine/engine_core_smooth.h:101)
/// Calls: mj_freeStack, mj_markStack, mj_objectVelocity, mj_stackAllocInfo, mji_addTo3, mji_cross, mji_mulMatVec3, mju_max, mju_mulMatTVec3, mju_scl3, mju_sub3
#[allow(unused_variables, non_snake_case)]
pub fn mj_subtree_vel(m: *const mjModel, d: *mut mjData) {
    extern "C" { fn mj_subtreeVel_impl(m: *const mjModel, d: *mut mjData); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_subtreeVel_impl(m, d) }
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
    extern "C" {
        fn mj_rne_impl(m: *const mjModel, d: *mut mjData, flg_acc: i32, result: *mut f64);
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_rne_impl(m, d, flg_acc, result) }
}

/// C: mj_rnePostConstraint (engine/engine_core_smooth.h:110)
/// Calls: mj_contactForce, mj_local2Global, mji_copy3, mji_crossForce, mju_add, mju_addTo, mju_isZero, mju_message, mju_mulDofVec, mju_mulInertVec, mju_mulMatTVec3, mju_scl3, mju_sub, mju_subFrom, mju_transformSpatial, mju_zero, mju_zero3
#[allow(unused_variables, non_snake_case)]
pub fn mj_rne_post_constraint(m: *const mjModel, d: *mut mjData) {
    extern "C" { fn mj_rnePostConstraint_impl(m: *const mjModel, d: *mut mjData); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_rnePostConstraint_impl(m, d) }
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
    extern "C" { fn mj_tendonBias_impl(m: *const mjModel, d: *mut mjData, qfrc: *mut f64); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_tendonBias_impl(m, d, qfrc) }
}

