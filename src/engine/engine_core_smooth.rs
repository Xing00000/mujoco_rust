//! Port of: engine/engine_core_smooth.c
//! IR hash: fd5288ad7242f001
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mj_kinematics1 (engine/engine_core_smooth.c:40)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_addTo3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_addToScl3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_axisAngle2Quat, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_copy3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_copy4, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_mulMatVec3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_mulQuat, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_rotVecQuat, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_sub3, cxx:_mju_message, cxx:_mju_mulQuat, cxx:_mju_normalize4, cxx:_mju_quat2Mat, cxx:_mju_unit4, cxx:_mju_zero, cxx:_mju_zero3
#[allow(unused_variables, non_snake_case)]
pub fn mj_kinematics1(m: *const mjModel, d: *mut mjData) {
    use crate::types::*;
    const MJ_JNT_FREE: i32 = mjtJoint_mjJNT_FREE as i32;
    const MJ_JNT_SLIDE: i32 = mjtJoint_mjJNT_SLIDE as i32;
    const MJ_JNT_BALL: i32 = mjtJoint_mjJNT_BALL as i32;
    const MJ_JNT_HINGE: i32 = mjtJoint_mjJNT_HINGE as i32;
    const MJ_ENBL_SLEEP: i32 = mjtEnableBit_mjENBL_SLEEP as i32;
    const MJ_S_STATIC: i32 = mjtSleepState_mjS_STATIC as i32;
    const MJ_S_ASLEEP: i32 = mjtSleepState_mjS_ASLEEP as i32;

    // SAFETY: m and d are valid model/data pointers. All field accesses follow known types.
    unsafe {
        let nbody = (*m).nbody as usize;

        // set world position and orientation
        crate::engine::engine_inline::mji_zero3((*d).xpos);
        crate::engine::engine_util_blas::mju_unit4((*d).xquat);
        crate::engine::engine_inline::mji_zero3((*d).xipos);
        crate::engine::engine_util_blas::mju_zero((*d).xmat, 9);
        crate::engine::engine_util_blas::mju_zero((*d).ximat, 9);
        *(*d).xmat.add(0) = 1.0; *(*d).xmat.add(4) = 1.0; *(*d).xmat.add(8) = 1.0;
        *(*d).ximat.add(0) = 1.0; *(*d).ximat.add(4) = 1.0; *(*d).ximat.add(8) = 1.0;

        let sleep_filter = ((*m).opt.enableflags & MJ_ENBL_SLEEP) != 0;

        for i in 1..nbody {
            // skip static bodies
            if sleep_filter && *(*d).body_awake.add(i) == MJ_S_STATIC {
                continue;
            }

            let mut xpos = [0.0f64; 3];
            let mut xquat = [0.0f64; 4];

            let jntadr = *(*m).body_jntadr.add(i) as usize;
            let jntnum = *(*m).body_jntnum.add(i);

            // free joint
            if jntnum == 1 && *(*m).jnt_type.add(jntadr) == MJ_JNT_FREE {
                let qadr = *(*m).jnt_qposadr.add(jntadr) as usize;
                crate::engine::engine_inline::mji_copy3(xpos.as_mut_ptr(), (*d).qpos.add(qadr));
                crate::engine::engine_inline::mji_copy4(xquat.as_mut_ptr(), (*d).qpos.add(qadr + 3));
                crate::engine::engine_util_blas::mju_normalize4(xquat.as_mut_ptr());
                crate::engine::engine_inline::mji_copy3((*d).xanchor.add(3 * jntadr), xpos.as_ptr());
                crate::engine::engine_inline::mji_copy3((*d).xaxis.add(3 * jntadr), (*m).jnt_axis.add(3 * jntadr));
            } else {
                let pid = *(*m).body_parentid.add(i) as usize;

                // get body pos and quat
                let (bodypos, bodyquat): (*const f64, *const f64);
                let mut quat_buf = [0.0f64; 4];
                if *(*m).body_mocapid.add(i) >= 0 {
                    let mocapid = *(*m).body_mocapid.add(i) as usize;
                    bodypos = (*d).mocap_pos.add(3 * mocapid);
                    crate::engine::engine_inline::mji_copy4(quat_buf.as_mut_ptr(), (*d).mocap_quat.add(4 * mocapid));
                    crate::engine::engine_util_blas::mju_normalize4(quat_buf.as_mut_ptr());
                    bodyquat = quat_buf.as_ptr();
                } else {
                    bodypos = (*m).body_pos.add(3 * i);
                    bodyquat = (*m).body_quat.add(4 * i);
                }

                // apply fixed translation/rotation relative to parent
                if pid > 0 {
                    crate::engine::engine_inline::mji_mulMatVec3(xpos.as_mut_ptr(), (*d).xmat.add(9 * pid), bodypos);
                    crate::engine::engine_inline::mji_addTo3(xpos.as_mut_ptr(), (*d).xpos.add(3 * pid));
                    crate::engine::engine_inline::mji_mulQuat(xquat.as_mut_ptr(), (*d).xquat.add(4 * pid), bodyquat);
                } else {
                    crate::engine::engine_inline::mji_copy3(xpos.as_mut_ptr(), bodypos);
                    crate::engine::engine_inline::mji_copy4(xquat.as_mut_ptr(), bodyquat);
                }

                // accumulate joints
                for j in 0..jntnum as usize {
                    let jid = jntadr + j;
                    let qadr = *(*m).jnt_qposadr.add(jid) as usize;
                    let jtype = *(*m).jnt_type.add(jid);

                    let mut xaxis = [0.0f64; 3];
                    let mut xanchor = [0.0f64; 3];

                    // axis in global frame
                    crate::engine::engine_inline::mji_rotVecQuat(xaxis.as_mut_ptr(), (*m).jnt_axis.add(3 * jid), xquat.as_ptr());

                    // anchor in global frame
                    crate::engine::engine_inline::mji_rotVecQuat(xanchor.as_mut_ptr(), (*m).jnt_pos.add(3 * jid), xquat.as_ptr());
                    crate::engine::engine_inline::mji_addTo3(xanchor.as_mut_ptr(), xpos.as_ptr());

                    // apply joint transformation
                    if jtype == MJ_JNT_SLIDE {
                        let disp = *(*d).qpos.add(qadr) - *(*m).qpos0.add(qadr);
                        crate::engine::engine_inline::mji_addToScl3(xpos.as_mut_ptr(), xaxis.as_ptr(), disp);
                    } else if jtype == MJ_JNT_BALL || jtype == MJ_JNT_HINGE {
                        let mut qloc = [0.0f64; 4];
                        if jtype == MJ_JNT_BALL {
                            crate::engine::engine_inline::mji_copy4(qloc.as_mut_ptr(), (*d).qpos.add(qadr));
                            crate::engine::engine_util_blas::mju_normalize4(qloc.as_mut_ptr());
                        } else {
                            let angle = *(*d).qpos.add(qadr) - *(*m).qpos0.add(qadr);
                            crate::engine::engine_inline::mji_axisAngle2Quat(qloc.as_mut_ptr(), (*m).jnt_axis.add(3 * jid), angle);
                        }
                        crate::engine::engine_util_spatial::mju_mulQuat(xquat.as_mut_ptr(), xquat.as_ptr(), qloc.as_ptr());
                        let mut vec = [0.0f64; 3];
                        crate::engine::engine_inline::mji_rotVecQuat(vec.as_mut_ptr(), (*m).jnt_pos.add(3 * jid), xquat.as_ptr());
                        crate::engine::engine_inline::mji_sub3(xpos.as_mut_ptr(), xanchor.as_ptr(), vec.as_ptr());
                    } else {
                        crate::engine::engine_util_errmem::mju_error(
                            b"mj_kinematics1: unknown joint type\0".as_ptr() as *const i8);
                    }

                    crate::engine::engine_inline::mji_copy3((*d).xanchor.add(3 * jid), xanchor.as_ptr());
                    crate::engine::engine_inline::mji_copy3((*d).xaxis.add(3 * jid), xaxis.as_ptr());
                }
            }

            // normalize quaternion
            crate::engine::engine_util_blas::mju_normalize4(xquat.as_mut_ptr());

            // sleeping body: check for mismatch
            if sleep_filter && jntnum > 0 && *(*d).body_awake.add(i) == MJ_S_ASLEEP {
                let pos = (*d).xpos.add(3 * i);
                let xq = (*d).xquat.add(4 * i);
                let matched = xpos[0] == *pos.add(0) && xpos[1] == *pos.add(1) && xpos[2] == *pos.add(2)
                    && xquat[0] == *xq.add(0) && xquat[1] == *xq.add(1)
                    && xquat[2] == *xq.add(2) && xquat[3] == *xq.add(3);
                if matched { continue; }
                else {
                    *(*d).tree_awake.add(*(*m).body_treeid.add(i) as usize) = 1;
                }
            }

            // assign xquat, xpos, xmat
            crate::engine::engine_inline::mji_copy4((*d).xquat.add(4 * i), xquat.as_ptr());
            crate::engine::engine_inline::mji_copy3((*d).xpos.add(3 * i), xpos.as_ptr());
            crate::engine::engine_util_spatial::mju_quat2Mat((*d).xmat.add(9 * i), xquat.as_ptr());
        }
    }
}

/// C: mj_kinematics2 (engine/engine_core_smooth.c:191)
/// Calls: cxx:_mj_local2Global
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
            crate::engine::engine_core_util::mj_local2Global(
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
                crate::engine::engine_core_util::mj_local2Global(
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

            crate::engine::engine_core_util::mj_local2Global(
                d, (*d).site_xpos.add(3 * i as usize), (*d).site_xmat.add(9 * i as usize),
                (*m).site_pos.add(3 * i as usize), (*m).site_quat.add(4 * i as usize),
                bodyid, *(*m).site_sameframe.add(i as usize));
        }
    }
}

/// C: mj_kinematics (engine/engine_core_smooth.c:240)
/// Calls: cxx:_mj_kinematics1, cxx:_mj_kinematics2, cxx:_mj_updateSleep, cxx:_mj_wake
#[allow(unused_variables, non_snake_case)]
pub fn mj_kinematics(m: *const mjModel, d: *mut mjData) {
    // SAFETY: m, d are valid model/data pointers.
    unsafe {
        mj_kinematics1(m, d);
        if crate::engine::engine_sleep::mj_wake(m, d) != 0 {
            crate::engine::engine_sleep::mj_updateSleep(m, d);
        }
        mj_kinematics2(m, d);
    }
}

/// C: mj_comPos (engine/engine_core_smooth.c:250)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_addTo3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_copy3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_scl3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_sub3, cxx:_mju_dofCom, cxx:_mju_inertCom, cxx:_mju_scl3, cxx:_mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mj_com_pos(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx:_mj_comPos
}

/// C: mj_camlight (engine/engine_core_smooth.c:358)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_add3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_copy3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_copy9, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_cross, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_rotVecQuat, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_sub3, cxx:_mj_local2Global, cxx:_mju_normalize3, cxx:_mju_transpose
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
            crate::engine::engine_core_util::mj_local2Global(
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
            crate::engine::engine_core_util::mj_local2Global(
                d, (*d).light_xpos.add(3 * i), std::ptr::null_mut(),
                (*m).light_pos.add(3 * i), std::ptr::null(), id, 0);
            crate::engine::engine_inline::mji_rotVecQuat(
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

/// C: mj_updateDynamicBVH (engine/engine_core_smooth.c:494)
/// Calls: cxx:_mj_freeStack, cxx:_mj_markStack, cxx:_mj_stackAllocInfo, cxx:_mju_max, cxx:_mju_min, cxx:_mju_zeroInt
#[allow(unused_variables, non_snake_case)]
pub fn mj_update_dynamic_bvh(m: *const mjModel, d: *mut mjData, bvhadr: i32, bvhnum: i32) {
    todo!() // cxx:_mj_updateDynamicBVH
}

/// C: mju_mulMatMat322 (engine/engine_core_smooth.c:541)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_mul_mat_mat322(C: *mut f64, A: *const f64, B: *const f64) {
    todo!() // cxx-internal:engine_core_smooth.c.o:_mju_mulMatMat322
}

/// C: mj_flex (engine/engine_core_smooth.c:552)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_core_smooth.c:_mju_mulMatMat322, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_addTo3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_copy3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_copy6, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_mulMatVec3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_sub3, cxx:_mj_bodyChain, cxx:_mj_freeStack, cxx:_mj_jacDifPair, cxx:_mj_jacSparse, cxx:_mj_markStack, cxx:_mj_stackAllocInfo, cxx:_mj_updateDynamicBVH, cxx:_mju_cellLookup, cxx:_mju_interpolate3D, cxx:_mju_max, cxx:_mju_message, cxx:_mju_min, cxx:_mju_mulMatTVec, cxx:_mju_mulMatVec, cxx:_mju_normalize3, cxx:_mju_scl3, cxx:_mju_shellTrackInterior, cxx:_mju_sub3, cxx:_mju_zero, cxx:_mju_zero3
#[allow(unused_variables, non_snake_case)]
pub fn mj_flex(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx:_mj_flex
}

/// C: mj_tendon (engine/engine_core_smooth.c:931)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_copy3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_copy9, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_sub3, cxx:_mj_freeStack, cxx:_mj_jacDifPair, cxx:_mj_markStack, cxx:_mj_sleepState, cxx:_mj_stackAllocInfo, cxx:_mju_combineSparseInc, cxx:_mju_dist3, cxx:_mju_message, cxx:_mju_mulMatTVec, cxx:_mju_normalize3, cxx:_mju_round, cxx:_mju_wrap, cxx:_mju_zero, cxx:_mju_zero3
#[allow(unused_variables, non_snake_case)]
pub fn mj_tendon(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx:_mj_tendon
}

/// C: mj_tendonDot (engine/engine_core_smooth.c:1119)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_copy3, cxx:_mj_freeStack, cxx:_mj_isSparse, cxx:_mj_jac, cxx:_mj_jacDot, cxx:_mj_jacDotSparse, cxx:_mj_jacSparse, cxx:_mj_markStack, cxx:_mj_mergeChain, cxx:_mj_objectVelocity, cxx:_mj_stackAllocInfo, cxx:_mju_addToScl3, cxx:_mju_dot, cxx:_mju_dot3, cxx:_mju_message, cxx:_mju_mulMatTVec, cxx:_mju_normalize3, cxx:_mju_scl3, cxx:_mju_sub, cxx:_mju_sub3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_tendon_dot(m: *const mjModel, d: *mut mjData, id: i32, vec: *const f64) -> f64 {
    todo!() // cxx:_mj_tendonDot
}

/// C: mj_transmission (engine/engine_core_smooth.c:1269)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_addTo3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_copy3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_copy4, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_mulMatVec3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_mulQuat, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_quat2Vel, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_rotVecQuat, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_subQuat, cxx:_mj_freeStack, cxx:_mj_isSparse, cxx:_mj_jacDifPair, cxx:_mj_jacPointAxis, cxx:_mj_jacSite, cxx:_mj_markStack, cxx:_mj_mulJacTVec, cxx:_mj_sleepState, cxx:_mj_stackAllocInfo, cxx:_mju_addTo, cxx:_mju_copyInt, cxx:_mju_dot3, cxx:_mju_isZero, cxx:_mju_message, cxx:_mju_mulMatMat, cxx:_mju_mulMatTVec, cxx:_mju_mulMatTVec3, cxx:_mju_negQuat, cxx:_mju_normalize4, cxx:_mju_scl, cxx:_mju_scl3, cxx:_mju_sub3, cxx:_mju_subFrom, cxx:_mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mj_transmission(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx:_mj_transmission
}

/// C: mj_tendonArmature (engine/engine_core_smooth.c:1849)
/// Calls: cxx:_mj_actuatorArmature, cxx:_mj_sleepState, cxx:_mju_addToSclSparseInc
#[allow(unused_variables, non_snake_case)]
pub fn mj_tendon_armature(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx:_mj_tendonArmature
}

/// C: mj_crb (engine/engine_core_smooth.c:1894)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_dot6, cxx:_mj_actuatorArmature, cxx:_mju_addTo, cxx:_mju_copy, cxx:_mju_copyRows, cxx:_mju_mulInertVec, cxx:_mju_zero, cxx:_mju_zeroSparse
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
            crate::engine::engine_util_blas::mju_copyRows(crb, cinert as *const f64, body_awake_ind, nbody, 10);
        }

        // backward pass over bodies, accumulate composite inertias
        let mut b = nparent - 1;
        while b >= 0 {
            let i = if sleep_filter { *parent_awake_ind.add(b as usize) } else { b };
            if *body_parentid.add(i as usize) > 0 {
                crate::engine::engine_util_blas::mju_addTo(
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
            crate::engine::engine_util_sparse::mju_zeroSparse(M, rownnz, rowadr, dof_awake_ind, nv);
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
                + crate::engine::engine_core_util::mj_actuatorArmature(m, MJ_OBJ_JOINT, *(*m).dof_jntid.add(i as usize));

            // precompute buf = crb_body_i * cdof_i
            let mut buf: [f64; 6] = [0.0; 6];
            crate::engine::engine_util_spatial::mju_mulInertVec(
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

/// C: mj_makeM (engine/engine_core_smooth.c:1970)
/// Calls: cxx:_mj_crb, cxx:_mj_tendonArmature
#[allow(unused_variables, non_snake_case)]
pub fn mj_make_m(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx:_mj_makeM
}

/// C: mj_factorM (engine/engine_core_smooth.c:1979)
/// Calls: cxx:_mj_factorI, cxx:_mj_warning, cxx:_mju_copy, cxx:_mju_copySparse
#[allow(unused_variables, non_snake_case)]
pub fn mj_factor_m(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx:_mj_factorM
}

/// C: mj_factorI (engine/engine_core_smooth.c:2018)
/// Calls: cxx:_mju_addToScl, cxx:_mju_scl
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_factor_i(mat: *mut f64, diaginv: *mut f64, nv: i32, rownnz: *const i32, rowadr: *const i32, colind: *const i32, index: *const i32) -> i32 {
    todo!() // cxx:_mj_factorI
}

/// C: mj_solveLD (engine/engine_core_smooth.c:2059)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_sparse.h:_mju_dotSparse
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_solve_ld(x: *mut f64, qLD: *const f64, qLDiagInv: *const f64, nv: i32, n: i32, rownnz: *const i32, rowadr: *const i32, colind: *const i32, index: *const i32) {
    todo!() // cxx:_mj_solveLD
}

/// C: mj_solveM (engine/engine_core_smooth.c:2146)
/// Calls: cxx:_mj_solveLD, cxx:_mju_copy
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_solve_m(m: *const mjModel, d: *mut mjData, x: *mut f64, y: *const f64, n: i32) {
    todo!() // cxx:_mj_solveM
}

/// C: mj_solveM2 (engine/engine_core_smooth.c:2156)
/// Calls: cxx:_mj_solveM2_impl
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_solve_m2(m: *const mjModel, d: *mut mjData, x: *mut f64, y: *const f64, sqrtInvD: *const f64, n: i32) {
    todo!() // cxx:_mj_solveM2
}

/// C: mj_solveM2_impl (engine/engine_core_smooth.c:2165)
/// Calls: cxx:_mju_copy
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_solve_m2_impl(x: *mut f64, y: *const f64, sqrtInvD: *const f64, factor: *const f64, nv: i32, n: i32, rownnz: *const i32, rowadr: *const i32, colind: *const i32, diagnum: *const i32) {
    todo!() // cxx:_mj_solveM2_impl
}

/// C: mj_comVel (engine/engine_core_smooth.c:2206)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_copy6, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_crossMotion, cxx:_mju_addTo, cxx:_mju_copy, cxx:_mju_mulDofVec, cxx:_mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mj_com_vel(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx:_mj_comVel
}

/// C: mj_subtreeVel (engine/engine_core_smooth.c:2276)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_addTo3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_cross, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_mulMatVec3, cxx:_mj_freeStack, cxx:_mj_markStack, cxx:_mj_objectVelocity, cxx:_mj_stackAllocInfo, cxx:_mju_max, cxx:_mju_mulMatTVec3, cxx:_mju_scl3, cxx:_mju_sub3
#[allow(unused_variables, non_snake_case)]
pub fn mj_subtree_vel(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx:_mj_subtreeVel
}

/// C: mj_rne (engine/engine_core_smooth.c:2355)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_crossForce, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_dot6, cxx:_mj_freeStack, cxx:_mj_markStack, cxx:_mj_stackAllocInfo, cxx:_mju_add, cxx:_mju_addTo, cxx:_mju_mulDofVec, cxx:_mju_mulInertVec, cxx:_mju_scl3, cxx:_mju_zero
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

        crate::engine::engine_memory::mj_markStack(d);
        let loc_cacc = crate::engine::engine_memory::mj_stackAllocNum(d, ((*m).nbody * 6) as usize);
        let loc_cfrc_body = crate::engine::engine_memory::mj_stackAllocNum(d, ((*m).nbody * 6) as usize);

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
            crate::engine::engine_util_spatial::mju_mulDofVec(
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
                crate::engine::engine_util_spatial::mju_mulDofVec(
                    tmp.as_mut_ptr(),
                    (*d).cdof.add(6 * bda as usize),
                    (*d).qacc.add(bda as usize),
                    *(*m).body_dofnum.add(i as usize),
                );
                crate::engine::engine_util_blas::mju_addTo(
                    loc_cacc.add(6 * i as usize), tmp.as_ptr(), 6);
            }

            // cfrc_body = cinert * cacc + cvel x (cinert * cvel)
            crate::engine::engine_util_spatial::mju_mulInertVec(
                loc_cfrc_body.add(6 * i as usize),
                (*d).cinert.add(10 * i as usize),
                loc_cacc.add(6 * i as usize),
            );
            crate::engine::engine_util_spatial::mju_mulInertVec(
                tmp.as_mut_ptr(),
                (*d).cinert.add(10 * i as usize),
                (*d).cvel.add(6 * i as usize),
            );
            let mut tmp1: [f64; 6] = [0.0; 6];
            crate::engine::engine_inline::mji_crossForce(
                tmp1.as_mut_ptr(), (*d).cvel.add(6 * i as usize), tmp.as_ptr());
            crate::engine::engine_util_blas::mju_addTo(
                loc_cfrc_body.add(6 * i as usize), tmp1.as_ptr(), 6);
        }

        // clear world cfrc_body
        crate::engine::engine_util_blas::mju_zero(loc_cfrc_body, 6);

        // backward pass over bodies: accumulate cfrc_body from children
        for b in (1..nparent).rev() {
            let i = if sleep_filter { *(*d).parent_awake_ind.add(b as usize) } else { b };
            let j = *(*m).body_parentid.add(i as usize);
            if j != 0 {
                crate::engine::engine_util_blas::mju_addTo(
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

        crate::engine::engine_memory::mj_freeStack(d);
    }
}

/// C: mj_rnePostConstraint (engine/engine_core_smooth.c:2421)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_copy3, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_inline.h:_mji_crossForce, cxx:_mj_contactForce, cxx:_mj_local2Global, cxx:_mju_add, cxx:_mju_addTo, cxx:_mju_isZero, cxx:_mju_message, cxx:_mju_mulDofVec, cxx:_mju_mulInertVec, cxx:_mju_mulMatTVec3, cxx:_mju_scl3, cxx:_mju_sub, cxx:_mju_subFrom, cxx:_mju_transformSpatial, cxx:_mju_zero, cxx:_mju_zero3
#[allow(unused_variables, non_snake_case)]
pub fn mj_rne_post_constraint(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx:_mj_rnePostConstraint
}

/// C: mj_tendonBias (engine/engine_core_smooth.c:2633)
/// Calls: cxx:_mj_actuatorArmature, cxx:_mj_sleepState, cxx:_mj_tendonDot
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_tendon_bias(m: *const mjModel, d: *mut mjData, qfrc: *mut f64) {
    todo!() // cxx:_mj_tendonBias
}

