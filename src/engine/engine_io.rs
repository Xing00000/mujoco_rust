//! Port of: engine/engine_io.c
//! IR hash: 3fb6da908ad9d71c
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: getnsize (engine/engine_io.c:72)
#[allow(unused_variables, non_snake_case)]
pub fn getnsize() -> i32 {
    92
}

/// C: getnptr (engine/engine_io.c:84)
#[allow(unused_variables, non_snake_case)]
pub fn getnptr() -> i32 {
    471
}

/// C: bufwrite (engine/engine_io.c:96)
/// Calls: mju_message
#[allow(unused_variables, non_snake_case)]
pub fn bufwrite(src: *const (), num: i32, szbuf: i64, buf: *mut (), ptrbuf: *mut i64) {
    // SAFETY: caller guarantees src, buf, ptrbuf valid; num bytes available
    unsafe {
        if src.is_null() || buf.is_null() || ptrbuf.is_null() {
            return; // mjERROR would be called in C
        }
        if *ptrbuf + num as i64 > szbuf {
            return; // mjERROR would be called in C
        }
        std::ptr::copy_nonoverlapping(
            src as *const u8,
            (buf as *mut u8).add(*ptrbuf as usize),
            num as usize);
        *ptrbuf += num as i64;
    }
}

/// C: bufread (engine/engine_io.c:114)
/// Calls: mju_message
#[allow(unused_variables, non_snake_case)]
pub fn bufread(dest: *mut (), num: i32, szbuf: i64, buf: *const (), ptrbuf: *mut i64) {
    // SAFETY: caller guarantees dest, buf, ptrbuf valid; num bytes available
    unsafe {
        if dest.is_null() || buf.is_null() || ptrbuf.is_null() {
            return; // mjERROR would be called in C
        }
        if *ptrbuf + num as i64 > szbuf {
            return; // mjERROR would be called in C
        }
        std::ptr::copy_nonoverlapping(
            (buf as *const u8).add(*ptrbuf as usize),
            dest as *mut u8,
            num as usize);
        *ptrbuf += num as i64;
    }
}

/// C: SKIP (engine/engine_io.c:132)
#[allow(unused_variables, non_snake_case)]
pub fn skip(offset: isize) -> u32 {
    (64 - offset % 64) as u32
}

/// C: mj_setPtrModel (engine/engine_io.c:142)
/// Calls: SKIP, mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_set_ptr_model(m: *mut mjModel) {
    // SAFETY: m is valid with allocated buffer (caller contract)
    unsafe {
        let mut ptr = (*m).buffer as *mut u8;

        // MJMODEL_POINTERS expansion: assign pointers with alignment padding
        let pad = skip(ptr as isize) as usize;
        (*m).qpos0 = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nq as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).qpos_spring = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nq as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).body_parentid = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nbody as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).body_rootid = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nbody as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).body_weldid = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nbody as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).body_mocapid = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nbody as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).body_jntnum = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nbody as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).body_jntadr = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nbody as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).body_dofnum = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nbody as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).body_dofadr = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nbody as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).body_treeid = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nbody as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).body_geomnum = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nbody as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).body_geomadr = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nbody as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).body_simple = ptr.add(pad) as *mut u8;
        ptr = ptr.add(pad + 1 * (*m).nbody as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).body_sameframe = ptr.add(pad) as *mut u8;
        ptr = ptr.add(pad + 1 * (*m).nbody as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).body_pos = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nbody as usize * 3);
        let pad = skip(ptr as isize) as usize;
        (*m).body_quat = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nbody as usize * 4);
        let pad = skip(ptr as isize) as usize;
        (*m).body_ipos = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nbody as usize * 3);
        let pad = skip(ptr as isize) as usize;
        (*m).body_iquat = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nbody as usize * 4);
        let pad = skip(ptr as isize) as usize;
        (*m).body_mass = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nbody as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).body_subtreemass = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nbody as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).body_inertia = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nbody as usize * 3);
        let pad = skip(ptr as isize) as usize;
        (*m).body_invweight0 = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nbody as usize * 2);
        let pad = skip(ptr as isize) as usize;
        (*m).body_gravcomp = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nbody as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).body_margin = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nbody as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).body_user = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nbody as usize * (*m).nuser_body as usize);
        let pad = skip(ptr as isize) as usize;
        (*m).body_plugin = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nbody as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).body_contype = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nbody as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).body_conaffinity = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nbody as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).body_bvhadr = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nbody as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).body_bvhnum = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nbody as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).bvh_depth = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nbvh as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).bvh_child = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nbvh as usize * 2);
        let pad = skip(ptr as isize) as usize;
        (*m).bvh_nodeid = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nbvh as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).bvh_aabb = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nbvhstatic as usize * 6);
        let pad = skip(ptr as isize) as usize;
        (*m).oct_depth = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).noct as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).oct_child = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).noct as usize * 8);
        let pad = skip(ptr as isize) as usize;
        (*m).oct_aabb = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).noct as usize * 6);
        let pad = skip(ptr as isize) as usize;
        (*m).oct_coeff = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).noct as usize * 8);
        let pad = skip(ptr as isize) as usize;
        (*m).jnt_type = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).njnt as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).jnt_qposadr = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).njnt as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).jnt_dofadr = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).njnt as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).jnt_bodyid = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).njnt as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).jnt_actuatorid = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).njnt as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).jnt_group = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).njnt as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).jnt_limited = ptr.add(pad) as *mut bool;
        ptr = ptr.add(pad + 1 * (*m).njnt as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).jnt_actfrclimited = ptr.add(pad) as *mut bool;
        ptr = ptr.add(pad + 1 * (*m).njnt as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).jnt_actgravcomp = ptr.add(pad) as *mut bool;
        ptr = ptr.add(pad + 1 * (*m).njnt as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).jnt_solref = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).njnt as usize * 2);
        let pad = skip(ptr as isize) as usize;
        (*m).jnt_solimp = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).njnt as usize * 5);
        let pad = skip(ptr as isize) as usize;
        (*m).jnt_pos = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).njnt as usize * 3);
        let pad = skip(ptr as isize) as usize;
        (*m).jnt_axis = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).njnt as usize * 3);
        let pad = skip(ptr as isize) as usize;
        (*m).jnt_stiffness = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).njnt as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).jnt_stiffnesspoly = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).njnt as usize * 2);
        let pad = skip(ptr as isize) as usize;
        (*m).jnt_range = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).njnt as usize * 2);
        let pad = skip(ptr as isize) as usize;
        (*m).jnt_actfrcrange = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).njnt as usize * 2);
        let pad = skip(ptr as isize) as usize;
        (*m).jnt_margin = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).njnt as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).jnt_user = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).njnt as usize * (*m).nuser_jnt as usize);
        let pad = skip(ptr as isize) as usize;
        (*m).dof_bodyid = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nv as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).dof_jntid = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nv as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).dof_parentid = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nv as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).dof_treeid = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nv as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).dof_Madr = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nv as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).dof_simplenum = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nv as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).dof_solref = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nv as usize * 2);
        let pad = skip(ptr as isize) as usize;
        (*m).dof_solimp = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nv as usize * 5);
        let pad = skip(ptr as isize) as usize;
        (*m).dof_frictionloss = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nv as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).dof_armature = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nv as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).dof_damping = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nv as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).dof_dampingpoly = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nv as usize * 2);
        let pad = skip(ptr as isize) as usize;
        (*m).dof_invweight0 = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nv as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).dof_M0 = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nv as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).dof_length = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nv as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).tree_bodyadr = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).ntree as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).tree_bodynum = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).ntree as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).tree_dofadr = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).ntree as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).tree_dofnum = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).ntree as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).tree_sleep_policy = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).ntree as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).geom_type = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).ngeom as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).geom_contype = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).ngeom as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).geom_conaffinity = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).ngeom as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).geom_condim = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).ngeom as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).geom_bodyid = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).ngeom as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).geom_dataid = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).ngeom as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).geom_matid = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).ngeom as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).geom_group = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).ngeom as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).geom_priority = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).ngeom as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).geom_plugin = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).ngeom as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).geom_sameframe = ptr.add(pad) as *mut u8;
        ptr = ptr.add(pad + 1 * (*m).ngeom as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).geom_solmix = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).ngeom as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).geom_solref = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).ngeom as usize * 2);
        let pad = skip(ptr as isize) as usize;
        (*m).geom_solimp = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).ngeom as usize * 5);
        let pad = skip(ptr as isize) as usize;
        (*m).geom_size = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).ngeom as usize * 3);
        let pad = skip(ptr as isize) as usize;
        (*m).geom_aabb = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).ngeom as usize * 6);
        let pad = skip(ptr as isize) as usize;
        (*m).geom_rbound = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).ngeom as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).geom_pos = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).ngeom as usize * 3);
        let pad = skip(ptr as isize) as usize;
        (*m).geom_quat = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).ngeom as usize * 4);
        let pad = skip(ptr as isize) as usize;
        (*m).geom_friction = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).ngeom as usize * 3);
        let pad = skip(ptr as isize) as usize;
        (*m).geom_margin = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).ngeom as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).geom_gap = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).ngeom as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).geom_fluid = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).ngeom as usize * 12);
        let pad = skip(ptr as isize) as usize;
        (*m).geom_user = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).ngeom as usize * (*m).nuser_geom as usize);
        let pad = skip(ptr as isize) as usize;
        (*m).geom_rgba = ptr.add(pad) as *mut f32;
        ptr = ptr.add(pad + 4 * (*m).ngeom as usize * 4);
        let pad = skip(ptr as isize) as usize;
        (*m).site_type = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nsite as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).site_bodyid = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nsite as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).site_matid = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nsite as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).site_group = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nsite as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).site_sameframe = ptr.add(pad) as *mut u8;
        ptr = ptr.add(pad + 1 * (*m).nsite as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).site_size = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nsite as usize * 3);
        let pad = skip(ptr as isize) as usize;
        (*m).site_pos = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nsite as usize * 3);
        let pad = skip(ptr as isize) as usize;
        (*m).site_quat = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nsite as usize * 4);
        let pad = skip(ptr as isize) as usize;
        (*m).site_user = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nsite as usize * (*m).nuser_site as usize);
        let pad = skip(ptr as isize) as usize;
        (*m).site_rgba = ptr.add(pad) as *mut f32;
        ptr = ptr.add(pad + 4 * (*m).nsite as usize * 4);
        let pad = skip(ptr as isize) as usize;
        (*m).cam_mode = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).ncam as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).cam_bodyid = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).ncam as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).cam_targetbodyid = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).ncam as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).cam_pos = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).ncam as usize * 3);
        let pad = skip(ptr as isize) as usize;
        (*m).cam_quat = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).ncam as usize * 4);
        let pad = skip(ptr as isize) as usize;
        (*m).cam_poscom0 = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).ncam as usize * 3);
        let pad = skip(ptr as isize) as usize;
        (*m).cam_pos0 = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).ncam as usize * 3);
        let pad = skip(ptr as isize) as usize;
        (*m).cam_mat0 = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).ncam as usize * 9);
        let pad = skip(ptr as isize) as usize;
        (*m).cam_projection = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).ncam as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).cam_fovy = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).ncam as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).cam_ipd = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).ncam as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).cam_resolution = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).ncam as usize * 2);
        let pad = skip(ptr as isize) as usize;
        (*m).cam_output = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).ncam as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).cam_sensorsize = ptr.add(pad) as *mut f32;
        ptr = ptr.add(pad + 4 * (*m).ncam as usize * 2);
        let pad = skip(ptr as isize) as usize;
        (*m).cam_intrinsic = ptr.add(pad) as *mut f32;
        ptr = ptr.add(pad + 4 * (*m).ncam as usize * 4);
        let pad = skip(ptr as isize) as usize;
        (*m).cam_user = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).ncam as usize * (*m).nuser_cam as usize);
        let pad = skip(ptr as isize) as usize;
        (*m).light_mode = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nlight as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).light_bodyid = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nlight as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).light_targetbodyid = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nlight as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).light_type = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nlight as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).light_texid = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nlight as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).light_castshadow = ptr.add(pad) as *mut bool;
        ptr = ptr.add(pad + 1 * (*m).nlight as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).light_bulbradius = ptr.add(pad) as *mut f32;
        ptr = ptr.add(pad + 4 * (*m).nlight as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).light_intensity = ptr.add(pad) as *mut f32;
        ptr = ptr.add(pad + 4 * (*m).nlight as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).light_range = ptr.add(pad) as *mut f32;
        ptr = ptr.add(pad + 4 * (*m).nlight as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).light_active = ptr.add(pad) as *mut bool;
        ptr = ptr.add(pad + 1 * (*m).nlight as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).light_pos = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nlight as usize * 3);
        let pad = skip(ptr as isize) as usize;
        (*m).light_dir = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nlight as usize * 3);
        let pad = skip(ptr as isize) as usize;
        (*m).light_poscom0 = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nlight as usize * 3);
        let pad = skip(ptr as isize) as usize;
        (*m).light_pos0 = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nlight as usize * 3);
        let pad = skip(ptr as isize) as usize;
        (*m).light_dir0 = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nlight as usize * 3);
        let pad = skip(ptr as isize) as usize;
        (*m).light_attenuation = ptr.add(pad) as *mut f32;
        ptr = ptr.add(pad + 4 * (*m).nlight as usize * 3);
        let pad = skip(ptr as isize) as usize;
        (*m).light_cutoff = ptr.add(pad) as *mut f32;
        ptr = ptr.add(pad + 4 * (*m).nlight as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).light_exponent = ptr.add(pad) as *mut f32;
        ptr = ptr.add(pad + 4 * (*m).nlight as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).light_ambient = ptr.add(pad) as *mut f32;
        ptr = ptr.add(pad + 4 * (*m).nlight as usize * 3);
        let pad = skip(ptr as isize) as usize;
        (*m).light_diffuse = ptr.add(pad) as *mut f32;
        ptr = ptr.add(pad + 4 * (*m).nlight as usize * 3);
        let pad = skip(ptr as isize) as usize;
        (*m).light_specular = ptr.add(pad) as *mut f32;
        ptr = ptr.add(pad + 4 * (*m).nlight as usize * 3);
        let pad = skip(ptr as isize) as usize;
        (*m).flex_contype = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nflex as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).flex_conaffinity = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nflex as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).flex_condim = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nflex as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).flex_priority = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nflex as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).flex_solmix = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nflex as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).flex_solref = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nflex as usize * 2);
        let pad = skip(ptr as isize) as usize;
        (*m).flex_solimp = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nflex as usize * 5);
        let pad = skip(ptr as isize) as usize;
        (*m).flex_friction = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nflex as usize * 3);
        let pad = skip(ptr as isize) as usize;
        (*m).flex_margin = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nflex as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).flex_gap = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nflex as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).flex_internal = ptr.add(pad) as *mut bool;
        ptr = ptr.add(pad + 1 * (*m).nflex as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).flex_selfcollide = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nflex as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).flex_activelayers = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nflex as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).flex_passive = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nflex as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).flex_dim = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nflex as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).flex_matid = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nflex as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).flex_group = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nflex as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).flex_interp = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nflex as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).flex_cellnum = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nflex as usize * 3);
        let pad = skip(ptr as isize) as usize;
        (*m).flex_nodeadr = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nflex as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).flex_nodenum = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nflex as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).flex_vertadr = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nflex as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).flex_vertnum = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nflex as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).flex_edgeadr = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nflex as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).flex_edgenum = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nflex as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).flex_elemadr = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nflex as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).flex_elemnum = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nflex as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).flex_elemdataadr = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nflex as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).flex_stiffnessadr = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nflex as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).flex_elemedgeadr = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nflex as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).flex_bendingadr = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nflex as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).flex_shellnum = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nflex as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).flex_shelldataadr = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nflex as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).flex_evpairadr = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nflex as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).flex_evpairnum = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nflex as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).flex_texcoordadr = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nflex as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).flex_nodebodyid = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nflexnode as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).flex_vertbodyid = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nflexvert as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).flex_vertedgeadr = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nflexvert as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).flex_vertedgenum = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nflexvert as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).flex_vertedge = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nflexedge as usize * 2);
        let pad = skip(ptr as isize) as usize;
        (*m).flex_edge = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nflexedge as usize * 2);
        let pad = skip(ptr as isize) as usize;
        (*m).flex_edgeflap = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nflexedge as usize * 2);
        let pad = skip(ptr as isize) as usize;
        (*m).flex_elem = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nflexelemdata as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).flex_elemtexcoord = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nflexelemdata as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).flex_elemedge = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nflexelemedge as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).flex_elemlayer = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nflexelem as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).flex_shell = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nflexshelldata as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).flex_evpair = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nflexevpair as usize * 2);
        let pad = skip(ptr as isize) as usize;
        (*m).flex_vert = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nflexvert as usize * 3);
        let pad = skip(ptr as isize) as usize;
        (*m).flex_vert0 = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nflexvert as usize * 3);
        let pad = skip(ptr as isize) as usize;
        (*m).flex_vertmetric = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nflexvert as usize * 4);
        let pad = skip(ptr as isize) as usize;
        (*m).flex_node = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nflexnode as usize * 3);
        let pad = skip(ptr as isize) as usize;
        (*m).flex_node0 = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nflexnode as usize * 3);
        let pad = skip(ptr as isize) as usize;
        (*m).flexedge_length0 = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nflexedge as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).flexedge_invweight0 = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nflexedge as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).flex_radius = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nflex as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).flex_size = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nflex as usize * 3);
        let pad = skip(ptr as isize) as usize;
        (*m).flex_stiffness = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nflexstiffness as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).flex_bending = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nflexbending as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).flex_damping = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nflex as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).flex_edgestiffness = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nflex as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).flex_edgedamping = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nflex as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).flex_edgeequality = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nflex as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).flex_rigid = ptr.add(pad) as *mut bool;
        ptr = ptr.add(pad + 1 * (*m).nflex as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).flexedge_rigid = ptr.add(pad) as *mut bool;
        ptr = ptr.add(pad + 1 * (*m).nflexedge as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).flex_centered = ptr.add(pad) as *mut bool;
        ptr = ptr.add(pad + 1 * (*m).nflex as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).flex_flatskin = ptr.add(pad) as *mut bool;
        ptr = ptr.add(pad + 1 * (*m).nflex as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).flex_bvhadr = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nflex as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).flex_bvhnum = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nflex as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).flexedge_J_rownnz = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nflexedge as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).flexedge_J_rowadr = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nflexedge as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).flexedge_J_colind = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nJfe as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).flexvert_J_rownnz = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nflexvert as usize * 2);
        let pad = skip(ptr as isize) as usize;
        (*m).flexvert_J_rowadr = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nflexvert as usize * 2);
        let pad = skip(ptr as isize) as usize;
        (*m).flexvert_J_colind = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nJfv as usize * 2);
        let pad = skip(ptr as isize) as usize;
        (*m).flex_rgba = ptr.add(pad) as *mut f32;
        ptr = ptr.add(pad + 4 * (*m).nflex as usize * 4);
        let pad = skip(ptr as isize) as usize;
        (*m).flex_texcoord = ptr.add(pad) as *mut f32;
        ptr = ptr.add(pad + 4 * (*m).nflextexcoord as usize * 2);
        let pad = skip(ptr as isize) as usize;
        (*m).mesh_vertadr = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nmesh as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).mesh_vertnum = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nmesh as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).mesh_faceadr = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nmesh as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).mesh_facenum = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nmesh as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).mesh_bvhadr = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nmesh as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).mesh_bvhnum = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nmesh as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).mesh_octadr = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nmesh as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).mesh_octnum = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nmesh as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).mesh_normaladr = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nmesh as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).mesh_normalnum = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nmesh as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).mesh_texcoordadr = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nmesh as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).mesh_texcoordnum = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nmesh as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).mesh_graphadr = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nmesh as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).mesh_vert = ptr.add(pad) as *mut f32;
        ptr = ptr.add(pad + 4 * (*m).nmeshvert as usize * 3);
        let pad = skip(ptr as isize) as usize;
        (*m).mesh_normal = ptr.add(pad) as *mut f32;
        ptr = ptr.add(pad + 4 * (*m).nmeshnormal as usize * 3);
        let pad = skip(ptr as isize) as usize;
        (*m).mesh_texcoord = ptr.add(pad) as *mut f32;
        ptr = ptr.add(pad + 4 * (*m).nmeshtexcoord as usize * 2);
        let pad = skip(ptr as isize) as usize;
        (*m).mesh_face = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nmeshface as usize * 3);
        let pad = skip(ptr as isize) as usize;
        (*m).mesh_facenormal = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nmeshface as usize * 3);
        let pad = skip(ptr as isize) as usize;
        (*m).mesh_facetexcoord = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nmeshface as usize * 3);
        let pad = skip(ptr as isize) as usize;
        (*m).mesh_graph = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nmeshgraph as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).mesh_scale = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nmesh as usize * 3);
        let pad = skip(ptr as isize) as usize;
        (*m).mesh_pos = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nmesh as usize * 3);
        let pad = skip(ptr as isize) as usize;
        (*m).mesh_quat = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nmesh as usize * 4);
        let pad = skip(ptr as isize) as usize;
        (*m).mesh_pathadr = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nmesh as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).mesh_polynum = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nmesh as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).mesh_polyadr = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nmesh as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).mesh_polynormal = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nmeshpoly as usize * 3);
        let pad = skip(ptr as isize) as usize;
        (*m).mesh_polyvertadr = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nmeshpoly as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).mesh_polyvertnum = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nmeshpoly as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).mesh_polyvert = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nmeshpolyvert as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).mesh_polymapadr = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nmeshvert as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).mesh_polymapnum = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nmeshvert as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).mesh_polymap = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nmeshpolymap as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).skin_matid = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nskin as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).skin_group = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nskin as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).skin_rgba = ptr.add(pad) as *mut f32;
        ptr = ptr.add(pad + 4 * (*m).nskin as usize * 4);
        let pad = skip(ptr as isize) as usize;
        (*m).skin_inflate = ptr.add(pad) as *mut f32;
        ptr = ptr.add(pad + 4 * (*m).nskin as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).skin_vertadr = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nskin as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).skin_vertnum = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nskin as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).skin_texcoordadr = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nskin as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).skin_faceadr = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nskin as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).skin_facenum = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nskin as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).skin_boneadr = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nskin as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).skin_bonenum = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nskin as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).skin_vert = ptr.add(pad) as *mut f32;
        ptr = ptr.add(pad + 4 * (*m).nskinvert as usize * 3);
        let pad = skip(ptr as isize) as usize;
        (*m).skin_texcoord = ptr.add(pad) as *mut f32;
        ptr = ptr.add(pad + 4 * (*m).nskintexvert as usize * 2);
        let pad = skip(ptr as isize) as usize;
        (*m).skin_face = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nskinface as usize * 3);
        let pad = skip(ptr as isize) as usize;
        (*m).skin_bonevertadr = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nskinbone as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).skin_bonevertnum = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nskinbone as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).skin_bonebindpos = ptr.add(pad) as *mut f32;
        ptr = ptr.add(pad + 4 * (*m).nskinbone as usize * 3);
        let pad = skip(ptr as isize) as usize;
        (*m).skin_bonebindquat = ptr.add(pad) as *mut f32;
        ptr = ptr.add(pad + 4 * (*m).nskinbone as usize * 4);
        let pad = skip(ptr as isize) as usize;
        (*m).skin_bonebodyid = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nskinbone as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).skin_bonevertid = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nskinbonevert as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).skin_bonevertweight = ptr.add(pad) as *mut f32;
        ptr = ptr.add(pad + 4 * (*m).nskinbonevert as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).skin_pathadr = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nskin as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).hfield_size = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nhfield as usize * 4);
        let pad = skip(ptr as isize) as usize;
        (*m).hfield_nrow = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nhfield as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).hfield_ncol = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nhfield as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).hfield_adr = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nhfield as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).hfield_data = ptr.add(pad) as *mut f32;
        ptr = ptr.add(pad + 4 * (*m).nhfielddata as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).hfield_pathadr = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nhfield as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).tex_type = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).ntex as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).tex_colorspace = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).ntex as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).tex_height = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).ntex as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).tex_width = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).ntex as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).tex_nchannel = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).ntex as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).tex_adr = ptr.add(pad) as *mut i64;
        ptr = ptr.add(pad + 8 * (*m).ntex as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).tex_data = ptr.add(pad) as *mut u8;
        ptr = ptr.add(pad + 1 * (*m).ntexdata as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).tex_pathadr = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).ntex as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).mat_texid = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nmat as usize * 10);
        let pad = skip(ptr as isize) as usize;
        (*m).mat_texuniform = ptr.add(pad) as *mut bool;
        ptr = ptr.add(pad + 1 * (*m).nmat as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).mat_texrepeat = ptr.add(pad) as *mut f32;
        ptr = ptr.add(pad + 4 * (*m).nmat as usize * 2);
        let pad = skip(ptr as isize) as usize;
        (*m).mat_emission = ptr.add(pad) as *mut f32;
        ptr = ptr.add(pad + 4 * (*m).nmat as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).mat_specular = ptr.add(pad) as *mut f32;
        ptr = ptr.add(pad + 4 * (*m).nmat as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).mat_shininess = ptr.add(pad) as *mut f32;
        ptr = ptr.add(pad + 4 * (*m).nmat as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).mat_reflectance = ptr.add(pad) as *mut f32;
        ptr = ptr.add(pad + 4 * (*m).nmat as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).mat_metallic = ptr.add(pad) as *mut f32;
        ptr = ptr.add(pad + 4 * (*m).nmat as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).mat_roughness = ptr.add(pad) as *mut f32;
        ptr = ptr.add(pad + 4 * (*m).nmat as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).mat_rgba = ptr.add(pad) as *mut f32;
        ptr = ptr.add(pad + 4 * (*m).nmat as usize * 4);
        let pad = skip(ptr as isize) as usize;
        (*m).pair_dim = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).npair as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).pair_geom1 = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).npair as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).pair_geom2 = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).npair as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).pair_signature = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).npair as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).pair_solref = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).npair as usize * 2);
        let pad = skip(ptr as isize) as usize;
        (*m).pair_solreffriction = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).npair as usize * 2);
        let pad = skip(ptr as isize) as usize;
        (*m).pair_solimp = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).npair as usize * 5);
        let pad = skip(ptr as isize) as usize;
        (*m).pair_margin = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).npair as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).pair_gap = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).npair as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).pair_friction = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).npair as usize * 5);
        let pad = skip(ptr as isize) as usize;
        (*m).exclude_signature = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nexclude as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).eq_type = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).neq as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).eq_obj1id = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).neq as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).eq_obj2id = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).neq as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).eq_objtype = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).neq as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).eq_active0 = ptr.add(pad) as *mut bool;
        ptr = ptr.add(pad + 1 * (*m).neq as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).eq_solref = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).neq as usize * 2);
        let pad = skip(ptr as isize) as usize;
        (*m).eq_solimp = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).neq as usize * 5);
        let pad = skip(ptr as isize) as usize;
        (*m).eq_data = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).neq as usize * 11);
        let pad = skip(ptr as isize) as usize;
        (*m).tendon_adr = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).ntendon as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).tendon_num = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).ntendon as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).tendon_matid = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).ntendon as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).tendon_actuatorid = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).ntendon as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).tendon_group = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).ntendon as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).tendon_treenum = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).ntendon as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).tendon_treeid = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).ntendon as usize * 2);
        let pad = skip(ptr as isize) as usize;
        (*m).ten_J_rownnz = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).ntendon as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).ten_J_rowadr = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).ntendon as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).ten_J_colind = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nJten as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).tendon_limited = ptr.add(pad) as *mut bool;
        ptr = ptr.add(pad + 1 * (*m).ntendon as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).tendon_actfrclimited = ptr.add(pad) as *mut bool;
        ptr = ptr.add(pad + 1 * (*m).ntendon as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).tendon_width = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).ntendon as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).tendon_solref_lim = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).ntendon as usize * 2);
        let pad = skip(ptr as isize) as usize;
        (*m).tendon_solimp_lim = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).ntendon as usize * 5);
        let pad = skip(ptr as isize) as usize;
        (*m).tendon_solref_fri = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).ntendon as usize * 2);
        let pad = skip(ptr as isize) as usize;
        (*m).tendon_solimp_fri = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).ntendon as usize * 5);
        let pad = skip(ptr as isize) as usize;
        (*m).tendon_range = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).ntendon as usize * 2);
        let pad = skip(ptr as isize) as usize;
        (*m).tendon_actfrcrange = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).ntendon as usize * 2);
        let pad = skip(ptr as isize) as usize;
        (*m).tendon_margin = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).ntendon as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).tendon_stiffness = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).ntendon as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).tendon_stiffnesspoly = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).ntendon as usize * 2);
        let pad = skip(ptr as isize) as usize;
        (*m).tendon_damping = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).ntendon as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).tendon_dampingpoly = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).ntendon as usize * 2);
        let pad = skip(ptr as isize) as usize;
        (*m).tendon_armature = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).ntendon as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).tendon_frictionloss = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).ntendon as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).tendon_lengthspring = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).ntendon as usize * 2);
        let pad = skip(ptr as isize) as usize;
        (*m).tendon_length0 = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).ntendon as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).tendon_invweight0 = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).ntendon as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).tendon_user = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).ntendon as usize * (*m).nuser_tendon as usize);
        let pad = skip(ptr as isize) as usize;
        (*m).tendon_rgba = ptr.add(pad) as *mut f32;
        ptr = ptr.add(pad + 4 * (*m).ntendon as usize * 4);
        let pad = skip(ptr as isize) as usize;
        (*m).wrap_type = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nwrap as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).wrap_objid = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nwrap as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).wrap_prm = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nwrap as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).actuator_trntype = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nu as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).actuator_dyntype = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nu as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).actuator_gaintype = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nu as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).actuator_biastype = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nu as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).actuator_trnid = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nu as usize * 2);
        let pad = skip(ptr as isize) as usize;
        (*m).actuator_damping = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nu as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).actuator_dampingpoly = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nu as usize * 2);
        let pad = skip(ptr as isize) as usize;
        (*m).actuator_armature = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nu as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).actuator_actadr = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nu as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).actuator_actnum = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nu as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).actuator_group = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nu as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).actuator_history = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nu as usize * 2);
        let pad = skip(ptr as isize) as usize;
        (*m).actuator_historyadr = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nu as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).actuator_delay = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nu as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).actuator_ctrllimited = ptr.add(pad) as *mut bool;
        ptr = ptr.add(pad + 1 * (*m).nu as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).actuator_forcelimited = ptr.add(pad) as *mut bool;
        ptr = ptr.add(pad + 1 * (*m).nu as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).actuator_actlimited = ptr.add(pad) as *mut bool;
        ptr = ptr.add(pad + 1 * (*m).nu as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).actuator_dynprm = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nu as usize * 10);
        let pad = skip(ptr as isize) as usize;
        (*m).actuator_gainprm = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nu as usize * 10);
        let pad = skip(ptr as isize) as usize;
        (*m).actuator_biasprm = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nu as usize * 10);
        let pad = skip(ptr as isize) as usize;
        (*m).actuator_actearly = ptr.add(pad) as *mut bool;
        ptr = ptr.add(pad + 1 * (*m).nu as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).actuator_ctrlrange = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nu as usize * 2);
        let pad = skip(ptr as isize) as usize;
        (*m).actuator_forcerange = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nu as usize * 2);
        let pad = skip(ptr as isize) as usize;
        (*m).actuator_actrange = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nu as usize * 2);
        let pad = skip(ptr as isize) as usize;
        (*m).actuator_gear = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nu as usize * 6);
        let pad = skip(ptr as isize) as usize;
        (*m).actuator_cranklength = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nu as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).actuator_acc0 = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nu as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).actuator_length0 = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nu as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).actuator_lengthrange = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nu as usize * 2);
        let pad = skip(ptr as isize) as usize;
        (*m).actuator_user = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nu as usize * (*m).nuser_actuator as usize);
        let pad = skip(ptr as isize) as usize;
        (*m).actuator_plugin = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nu as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).sensor_type = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nsensor as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).sensor_datatype = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nsensor as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).sensor_needstage = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nsensor as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).sensor_objtype = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nsensor as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).sensor_objid = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nsensor as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).sensor_reftype = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nsensor as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).sensor_refid = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nsensor as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).sensor_intprm = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nsensor as usize * 3);
        let pad = skip(ptr as isize) as usize;
        (*m).sensor_dim = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nsensor as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).sensor_adr = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nsensor as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).sensor_cutoff = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nsensor as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).sensor_noise = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nsensor as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).sensor_history = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nsensor as usize * 2);
        let pad = skip(ptr as isize) as usize;
        (*m).sensor_historyadr = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nsensor as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).sensor_delay = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nsensor as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).sensor_interval = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nsensor as usize * 2);
        let pad = skip(ptr as isize) as usize;
        (*m).sensor_user = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nsensor as usize * (*m).nuser_sensor as usize);
        let pad = skip(ptr as isize) as usize;
        (*m).sensor_plugin = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nsensor as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).plugin = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nplugin as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).plugin_stateadr = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nplugin as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).plugin_statenum = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nplugin as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).plugin_attr = ptr.add(pad) as *mut i8;
        ptr = ptr.add(pad + 1 * (*m).npluginattr as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).plugin_attradr = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nplugin as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).numeric_adr = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nnumeric as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).numeric_size = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nnumeric as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).numeric_data = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nnumericdata as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).text_adr = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).ntext as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).text_size = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).ntext as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).text_data = ptr.add(pad) as *mut i8;
        ptr = ptr.add(pad + 1 * (*m).ntextdata as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).tuple_adr = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).ntuple as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).tuple_size = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).ntuple as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).tuple_objtype = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).ntupledata as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).tuple_objid = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).ntupledata as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).tuple_objprm = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).ntupledata as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).key_time = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nkey as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).key_qpos = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nkey as usize * (*m).nq as usize);
        let pad = skip(ptr as isize) as usize;
        (*m).key_qvel = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nkey as usize * (*m).nv as usize);
        let pad = skip(ptr as isize) as usize;
        (*m).key_act = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nkey as usize * (*m).na as usize);
        let pad = skip(ptr as isize) as usize;
        (*m).key_mpos = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nkey as usize * ((*m).nmocap as usize * 3));
        let pad = skip(ptr as isize) as usize;
        (*m).key_mquat = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nkey as usize * ((*m).nmocap as usize * 4));
        let pad = skip(ptr as isize) as usize;
        (*m).key_ctrl = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nkey as usize * (*m).nu as usize);
        let pad = skip(ptr as isize) as usize;
        (*m).name_bodyadr = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nbody as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).name_jntadr = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).njnt as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).name_geomadr = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).ngeom as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).name_siteadr = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nsite as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).name_camadr = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).ncam as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).name_lightadr = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nlight as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).name_flexadr = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nflex as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).name_meshadr = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nmesh as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).name_skinadr = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nskin as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).name_hfieldadr = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nhfield as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).name_texadr = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).ntex as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).name_matadr = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nmat as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).name_pairadr = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).npair as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).name_excludeadr = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nexclude as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).name_eqadr = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).neq as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).name_tendonadr = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).ntendon as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).name_actuatoradr = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nu as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).name_sensoradr = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nsensor as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).name_numericadr = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nnumeric as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).name_textadr = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).ntext as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).name_tupleadr = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).ntuple as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).name_keyadr = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nkey as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).name_pluginadr = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nplugin as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).names = ptr.add(pad) as *mut i8;
        ptr = ptr.add(pad + 1 * (*m).nnames as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).names_map = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nnames_map as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).paths = ptr.add(pad) as *mut i8;
        ptr = ptr.add(pad + 1 * (*m).npaths as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).B_rownnz = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nbody as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).B_rowadr = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nbody as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).B_colind = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nB as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).M_rownnz = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nv as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).M_rowadr = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nv as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).M_colind = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nC as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).mapM2M = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nC as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).D_rownnz = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nv as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).D_rowadr = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nv as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).D_diag = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nv as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).D_colind = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nD as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).mapM2D = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nD as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*m).mapD2M = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nC as usize * 1);

        // check size
        let sz = (ptr as usize) - ((*m).buffer as usize);
        if (*m).nbuffer != sz as i64 {
            crate::engine::engine_util_errmem::mju_error(
                b"mjModel buffer size mismatch\0".as_ptr() as *const i8);
        }
    }
}

/// C: safeAddToBufferSize (engine/engine_io.c:173)
/// Calls: SKIP
#[allow(unused_variables, non_snake_case)]
pub fn safe_add_to_buffer_size(offset: *mut isize, nbuffer: *mut i64, type_size: usize, nr: i64, nc: i64) -> i64 {
    // SAFETY: caller guarantees offset and nbuffer are valid pointers
    unsafe {
        if (type_size as i64) < 0 || nr < 0 || nc < 0 {
            return 0;
        }

        // SKIP: compute alignment padding (align to 64 bytes)
        let skip = {
            let align: usize = 64;
            (align - (*offset as usize % align)) % align
        };

        // nc * nr (overflow check)
        let product = match (nc as usize).checked_mul(nr as usize) {
            Some(v) => v,
            None => return 0,
        };

        // product * type_size (overflow check)
        let product = match product.checked_mul(type_size) {
            Some(v) => v,
            None => return 0,
        };

        // product + skip (overflow check)
        let to_add = match product.checked_add(skip) {
            Some(v) => v,
            None => return 0,
        };

        // *nbuffer + to_add (overflow check)
        let new_nbuffer = match (*nbuffer as usize).checked_add(to_add) {
            Some(v) => v,
            None => return 0,
        };
        *nbuffer = new_nbuffer as i64;

        // *offset + to_add (overflow check)
        if *offset > 0 && to_add > (isize::MAX as usize - *offset as usize) {
            return 0;
        }
        *offset += to_add as isize;

        1
    }
}

/// C: freeModelBuffers (engine/engine_io.c:221)
/// Calls: mju_free
#[allow(unused_variables, non_snake_case)]
pub fn free_model_buffers(m: *mut mjModel) {
    // SAFETY: m is a valid mjModel pointer (caller contract)
    unsafe {
        crate::engine::engine_util_errmem::mju_free((*m).buffer);
    }
}

/// C: checkDBSparse (engine/engine_io.c:895)
/// Calls: mju_message
#[allow(unused_variables, non_snake_case)]
pub fn check_db_sparse(m: *const mjModel) {
    // SAFETY: caller guarantees m is valid with all sparse arrays populated
    unsafe {
        for j in 0..(*m).nv as usize {
            let i = *(*m).dof_bodyid.add(j) as usize;

            // D[row j] and B[row i] should be identical
            if *(*m).D_rownnz.add(j) != *(*m).B_rownnz.add(i) {
                return; // mjERROR in C
            }
            for k in 0..*(*m).D_rownnz.add(j) as usize {
                if *(*m).D_colind.add(*(*m).D_rowadr.add(j) as usize + k)
                    != *(*m).B_colind.add(*(*m).B_rowadr.add(i) as usize + k)
                {
                    return; // mjERROR in C
                }
            }
        }
    }
}

/// C: copyM2Sparse (engine/engine_io.c:915)
/// Calls: mju_copyInt, mju_message
#[allow(unused_variables, non_snake_case)]
pub fn copy_m2sparse(nv: i32, dof_Madr: *const i32, dof_simplenum: *const i32, dof_parentid: *const i32, rownnz: *const i32, rowadr: *const i32, src: *const i32, dst: *mut i32, reduced: i32, upper: i32, remaining: *mut i32) {
    // SAFETY: all pointers are valid arrays of appropriate sizes (caller contract)
    unsafe {
        // init remaining
        crate::engine::engine_util_misc::mju_copy_int(remaining, rownnz, nv);

        // copy data
        let mut i = nv - 1;
        while i >= 0 {
            // init at diagonal
            let mut adr = *dof_Madr.add(i as usize);
            *remaining.add(i as usize) -= 1;
            *dst.add((*rowadr.add(i as usize) + *remaining.add(i as usize)) as usize) =
                *src.add(adr as usize);
            adr += 1;

            // process below diagonal unless reduced and dof is simple
            if !(reduced != 0 && *dof_simplenum.add(i as usize) != 0) {
                let mut j = i;
                loop {
                    j = *dof_parentid.add(j as usize);
                    if j < 0 { break; }
                    *remaining.add(i as usize) -= 1;
                    *dst.add((*rowadr.add(i as usize) + *remaining.add(i as usize)) as usize) =
                        *src.add(adr as usize);

                    // add upper triangle if requested
                    if upper != 0 {
                        *remaining.add(j as usize) -= 1;
                        *dst.add((*rowadr.add(j as usize) + *remaining.add(j as usize)) as usize) =
                            *src.add(adr as usize);
                    }

                    adr += 1;
                }
            }
            i -= 1;
        }

        // check that none remaining
        for i in 0..nv as usize {
            if *remaining.add(i) != 0 {
                crate::engine::engine_util_errmem::mju_error(
                    b"unassigned index\0".as_ptr() as *const i8);
            }
        }
    }
}

/// C: mj_setPtrData (engine/engine_io.c:989)
/// Calls: SKIP, mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_set_ptr_data(m: *const mjModel, d: *mut mjData) {
    // SAFETY: m and d are valid with allocated buffer (caller contract)
    unsafe {
        let mut ptr = (*d).buffer as *mut u8;

        // MJDATA_POINTERS expansion: assign pointers with alignment padding
        let pad = skip(ptr as isize) as usize;
        (*d).qpos = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nq as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*d).qvel = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nv as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*d).act = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).na as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*d).history = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nhistory as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*d).qacc_warmstart = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nv as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*d).plugin_state = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).npluginstate as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*d).ctrl = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nu as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*d).qfrc_applied = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nv as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*d).xfrc_applied = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nbody as usize * 6);
        let pad = skip(ptr as isize) as usize;
        (*d).eq_active = ptr.add(pad) as *mut bool;
        ptr = ptr.add(pad + 1 * (*m).neq as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*d).mocap_pos = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nmocap as usize * 3);
        let pad = skip(ptr as isize) as usize;
        (*d).mocap_quat = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nmocap as usize * 4);
        let pad = skip(ptr as isize) as usize;
        (*d).qacc = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nv as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*d).act_dot = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).na as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*d).userdata = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nuserdata as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*d).sensordata = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nsensordata as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*d).tree_asleep = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).ntree as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*d).plugin = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nplugin as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*d).plugin_data = ptr.add(pad) as *mut usize;
        ptr = ptr.add(pad + 4 * (*m).nplugin as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*d).xpos = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nbody as usize * 3);
        let pad = skip(ptr as isize) as usize;
        (*d).xquat = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nbody as usize * 4);
        let pad = skip(ptr as isize) as usize;
        (*d).xmat = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nbody as usize * 9);
        let pad = skip(ptr as isize) as usize;
        (*d).xipos = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nbody as usize * 3);
        let pad = skip(ptr as isize) as usize;
        (*d).ximat = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nbody as usize * 9);
        let pad = skip(ptr as isize) as usize;
        (*d).xanchor = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).njnt as usize * 3);
        let pad = skip(ptr as isize) as usize;
        (*d).xaxis = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).njnt as usize * 3);
        let pad = skip(ptr as isize) as usize;
        (*d).geom_xpos = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).ngeom as usize * 3);
        let pad = skip(ptr as isize) as usize;
        (*d).geom_xmat = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).ngeom as usize * 9);
        let pad = skip(ptr as isize) as usize;
        (*d).site_xpos = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nsite as usize * 3);
        let pad = skip(ptr as isize) as usize;
        (*d).site_xmat = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nsite as usize * 9);
        let pad = skip(ptr as isize) as usize;
        (*d).cam_xpos = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).ncam as usize * 3);
        let pad = skip(ptr as isize) as usize;
        (*d).cam_xmat = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).ncam as usize * 9);
        let pad = skip(ptr as isize) as usize;
        (*d).light_xpos = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nlight as usize * 3);
        let pad = skip(ptr as isize) as usize;
        (*d).light_xdir = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nlight as usize * 3);
        let pad = skip(ptr as isize) as usize;
        (*d).subtree_com = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nbody as usize * 3);
        let pad = skip(ptr as isize) as usize;
        (*d).cdof = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nv as usize * 6);
        let pad = skip(ptr as isize) as usize;
        (*d).cinert = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nbody as usize * 10);
        let pad = skip(ptr as isize) as usize;
        (*d).flexvert_xpos = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nflexvert as usize * 3);
        let pad = skip(ptr as isize) as usize;
        (*d).flexelem_aabb = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nflexelem as usize * 6);
        let pad = skip(ptr as isize) as usize;
        (*d).flexedge_J = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nJfe as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*d).flexedge_length = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nflexedge as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*d).flexvert_J = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nJfv as usize * 2);
        let pad = skip(ptr as isize) as usize;
        (*d).flexvert_length = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nflexvert as usize * 2);
        let pad = skip(ptr as isize) as usize;
        (*d).bvh_aabb_dyn = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nbvhdynamic as usize * 6);
        let pad = skip(ptr as isize) as usize;
        (*d).ten_wrapadr = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).ntendon as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*d).ten_wrapnum = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).ntendon as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*d).ten_J = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nJten as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*d).ten_length = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).ntendon as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*d).wrap_obj = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nwrap as usize * 2);
        let pad = skip(ptr as isize) as usize;
        (*d).wrap_xpos = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nwrap as usize * 6);
        let pad = skip(ptr as isize) as usize;
        (*d).actuator_length = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nu as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*d).moment_rownnz = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nu as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*d).moment_rowadr = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nu as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*d).moment_colind = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nJmom as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*d).actuator_moment = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nJmom as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*d).crb = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nbody as usize * 10);
        let pad = skip(ptr as isize) as usize;
        (*d).qM = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nM as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*d).M = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nC as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*d).qLD = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nC as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*d).qLDiagInv = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nv as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*d).bvh_active = ptr.add(pad) as *mut bool;
        ptr = ptr.add(pad + 1 * (*m).nbvh as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*d).tree_awake = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).ntree as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*d).body_awake = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nbody as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*d).body_awake_ind = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nbody as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*d).parent_awake_ind = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nbody as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*d).dof_awake_ind = ptr.add(pad) as *mut i32;
        ptr = ptr.add(pad + 4 * (*m).nv as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*d).flexedge_velocity = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nflexedge as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*d).ten_velocity = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).ntendon as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*d).actuator_velocity = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nu as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*d).cvel = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nbody as usize * 6);
        let pad = skip(ptr as isize) as usize;
        (*d).cdof_dot = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nv as usize * 6);
        let pad = skip(ptr as isize) as usize;
        (*d).qfrc_bias = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nv as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*d).qfrc_spring = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nv as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*d).qfrc_damper = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nv as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*d).qfrc_gravcomp = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nv as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*d).qfrc_fluid = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nv as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*d).qfrc_passive = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nv as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*d).subtree_linvel = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nbody as usize * 3);
        let pad = skip(ptr as isize) as usize;
        (*d).subtree_angmom = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nbody as usize * 3);
        let pad = skip(ptr as isize) as usize;
        (*d).qH = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nC as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*d).qHDiagInv = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nv as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*d).qDeriv = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nD as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*d).qLU = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nD as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*d).actuator_force = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nu as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*d).qfrc_actuator = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nv as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*d).qfrc_smooth = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nv as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*d).qacc_smooth = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nv as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*d).qfrc_constraint = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nv as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*d).qfrc_inverse = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nv as usize * 1);
        let pad = skip(ptr as isize) as usize;
        (*d).cacc = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nbody as usize * 6);
        let pad = skip(ptr as isize) as usize;
        (*d).cfrc_int = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nbody as usize * 6);
        let pad = skip(ptr as isize) as usize;
        (*d).cfrc_ext = ptr.add(pad) as *mut f64;
        ptr = ptr.add(pad + 8 * (*m).nbody as usize * 6);

        // check size
        let sz = (ptr as usize) - ((*d).buffer as usize);
        if (*d).nbuffer != sz as i64 {
            crate::engine::engine_util_errmem::mju_error(
                b"mjData buffer size mismatch\0".as_ptr() as *const i8);
        }

        // zero-initialize arena pointers (MJDATA_ARENA_POINTERS)
        (*d).contact = std::ptr::null_mut();
        (*d).efc_type = std::ptr::null_mut();
        (*d).efc_id = std::ptr::null_mut();
        (*d).efc_J_rownnz = std::ptr::null_mut();
        (*d).efc_J_rowadr = std::ptr::null_mut();
        (*d).efc_J_rowsuper = std::ptr::null_mut();
        (*d).efc_J_colind = std::ptr::null_mut();
        (*d).efc_J = std::ptr::null_mut();
        (*d).efc_pos = std::ptr::null_mut();
        (*d).efc_margin = std::ptr::null_mut();
        (*d).efc_frictionloss = std::ptr::null_mut();
        (*d).efc_diagA = std::ptr::null_mut();
        (*d).efc_KBIP = std::ptr::null_mut();
        (*d).efc_D = std::ptr::null_mut();
        (*d).efc_R = std::ptr::null_mut();
        (*d).tendon_efcadr = std::ptr::null_mut();
        (*d).efc_vel = std::ptr::null_mut();
        (*d).efc_aref = std::ptr::null_mut();
        (*d).efc_b = std::ptr::null_mut();
        (*d).efc_state = std::ptr::null_mut();
        (*d).efc_force = std::ptr::null_mut();
        (*d).efc_Y_rownnz = std::ptr::null_mut();
        (*d).efc_Y_rowadr = std::ptr::null_mut();
        (*d).efc_Y_colind = std::ptr::null_mut();
        (*d).efc_Y = std::ptr::null_mut();
        (*d).efc_AR_rownnz = std::ptr::null_mut();
        (*d).efc_AR_rowadr = std::ptr::null_mut();
        (*d).efc_AR_colind = std::ptr::null_mut();
        (*d).efc_AR = std::ptr::null_mut();
        (*d).tree_island = std::ptr::null_mut();
        (*d).island_ntree = std::ptr::null_mut();
        (*d).island_itreeadr = std::ptr::null_mut();
        (*d).map_itree2tree = std::ptr::null_mut();
        (*d).dof_island = std::ptr::null_mut();
        (*d).island_nv = std::ptr::null_mut();
        (*d).island_idofadr = std::ptr::null_mut();
        (*d).island_dofadr = std::ptr::null_mut();
        (*d).map_dof2idof = std::ptr::null_mut();
        (*d).map_idof2dof = std::ptr::null_mut();
        (*d).ifrc_smooth = std::ptr::null_mut();
        (*d).iacc_smooth = std::ptr::null_mut();
        (*d).iacc = std::ptr::null_mut();
        (*d).efc_island = std::ptr::null_mut();
        (*d).island_ne = std::ptr::null_mut();
        (*d).island_nf = std::ptr::null_mut();
        (*d).island_nefc = std::ptr::null_mut();
        (*d).island_iefcadr = std::ptr::null_mut();
        (*d).map_efc2iefc = std::ptr::null_mut();
        (*d).map_iefc2efc = std::ptr::null_mut();
        (*d).iefc_type = std::ptr::null_mut();
        (*d).iefc_id = std::ptr::null_mut();
        (*d).iefc_frictionloss = std::ptr::null_mut();
        (*d).iefc_D = std::ptr::null_mut();
        (*d).iefc_R = std::ptr::null_mut();
        (*d).iefc_aref = std::ptr::null_mut();
        (*d).iefc_state = std::ptr::null_mut();
        (*d).iefc_force = std::ptr::null_mut();
        (*d).ifrc_constraint = std::ptr::null_mut();

        (*d).contact = (*d).arena as *mut mjContact;
    }
}

/// C: freeDataBuffers (engine/engine_io.c:1036)
/// Calls: mjp_getPluginAtSlot, mju_free
#[allow(unused_variables, non_snake_case)]
pub fn free_data_buffers(d: *mut mjData) {
    // SAFETY: caller guarantees d is a valid pointer to initialized mjData
    unsafe {
        // destroy plugin instances
        for i in 0..(*d).nplugin {
            let plugin = crate::engine::engine_plugin::mjp_get_plugin_at_slot(*(*d).plugin.add(i as usize));
            if let Some(destroy_fn) = (*plugin).destroy {
                // SAFETY: destroy is actually fn(*mut mjData, i32) but codegen typed it as fn()
                let destroy: unsafe extern "C" fn(*mut mjData, i32) = std::mem::transmute(destroy_fn);
                destroy(d, i);
            }
        }
        crate::engine::engine_util_errmem::mju_free((*d).buffer);
        crate::engine::engine_util_errmem::mju_free((*d).arena);
    }
}

/// C: mj_copyDataVisual (engine/engine_io.c:1142)
/// Calls: mj_initPlugin, mj_makeRawData, mj_setPtrData, mjp_getPluginAtSlot, mju_free, mju_malloc, mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_copy_data_visual(dest: *mut mjData, m: *const mjModel, src: *const mjData, flg_all: i32) -> *mut mjData {
    todo!() // mj_copyDataVisual
}

/// C: _resetData (engine/engine_io.c:1286)
/// Calls: checkDBSparse, mj_camlight, mj_clearEfc, mj_comPos, mj_deleteData, mj_forward, mj_id2name, mj_kinematics, mj_sleep, mj_tendon, mj_updateSleep, mj_updateSleepInit, mjp_getPluginAtSlot, mju_copy, mju_copy3, mju_copy4, mju_fillInt, mju_free, mju_malloc, mju_message, mju_zero, mju_zeroInt
#[allow(unused_variables, non_snake_case)]
pub fn reset_data(m: *const mjModel, d: *mut mjData, debug_value: u8) {
    todo!() // _resetData
}

/// C: mj_logTimingDiagnostics (engine/engine_io.c:1570)
/// Calls: mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_log_timing_diagnostics(d: *const mjData) {
    // Timer enum constants
    const MJTIMER_STEP: usize = 0;
    const MJTIMER_POSITION: usize = 3;
    const MJTIMER_ADVANCE: usize = 7;
    const MJTIMER_POS_KINEMATICS: usize = 8;
    const MJTIMER_POS_PROJECT: usize = 12;
    const MJTIMER_POS_COLLISION: usize = 10;
    const MJTIMER_COL_BROAD: usize = 13;
    const MJTIMER_COL_NARROW: usize = 14;

    // mjTimerStat layout: { duration: f64, number: i32, _pad: [u8; 4] } = 16 bytes
    const TIMER_STAT_SIZE: usize = 16;

    extern "C" {
        fn snprintf(s: *mut i8, n: usize, fmt: *const i8, ...) -> i32;
    }

    // SAFETY: d is a valid mjData pointer (caller contract)
    // timer is [u8; 240] representing 15 mjTimerStat structs
    unsafe {
        let timer_base = (*d).timer.as_ptr();

        // helper: get duration (f64 at offset 0) from timer[i]
        let get_duration = |idx: usize| -> f64 {
            let ptr = timer_base.add(idx * TIMER_STAT_SIZE) as *const f64;
            *ptr
        };
        // helper: get number (i32 at offset 8) from timer[i]
        let get_number = |idx: usize| -> i32 {
            let ptr = timer_base.add(idx * TIMER_STAT_SIZE + 8) as *const i32;
            *ptr
        };

        let nstep = get_number(MJTIMER_STEP);
        if nstep <= 0 {
            return;
        }

        let tstep = get_duration(MJTIMER_STEP) / nstep as f64;
        if tstep <= 0.0 {
            return;
        }

        let mut buf = [0i8; 2048];
        let mut pos: usize = 0;
        let mut components: f64 = 0.0;

        let timer_names: [&[u8]; 15] = [
            b"step\0", b"forward\0", b"inverse\0",
            b"fwdPosition\0", b"fwdVelocity\0", b"fwdActuation\0",
            b"fwdConstraint\0", b"advance\0",
            b"pos_kinematics\0", b"pos_inertia\0", b"pos_collision\0",
            b"pos_make\0", b"pos_project\0",
            b"col_broad\0", b"col_narrow\0",
        ];

        for i in MJTIMER_POSITION..=MJTIMER_ADVANCE {
            if get_number(i) > 0 {
                let istep = get_duration(i) / get_number(i) as f64;
                components += istep;
                let written = snprintf(
                    buf.as_mut_ptr().add(pos),
                    2048 - pos,
                    b"%s  %-15s %8.1f  (%5.1f%%)\0".as_ptr() as *const i8,
                    if pos > 0 { b"\n\0".as_ptr() } else { b"\0".as_ptr() },
                    timer_names[i].as_ptr(),
                    istep * 1000.0,
                    100.0 * istep / tstep);
                if written > 0 { pos += written as usize; }

                // position sub-breakdown
                if i == MJTIMER_POSITION {
                    for p in MJTIMER_POS_KINEMATICS..=MJTIMER_POS_PROJECT {
                        if get_number(p) > 0 {
                            let pstep = get_duration(p) / get_number(p) as f64;
                            let written = snprintf(
                                buf.as_mut_ptr().add(pos),
                                2048 - pos,
                                b"\n    %-13s %8.1f  (%5.1f%%)\0".as_ptr() as *const i8,
                                timer_names[p].as_ptr().add(4),
                                pstep * 1000.0,
                                100.0 * pstep / tstep);
                            if written > 0 { pos += written as usize; }

                            // collision sub-breakdown
                            if p == MJTIMER_POS_COLLISION {
                                for c in MJTIMER_COL_BROAD..=MJTIMER_COL_NARROW {
                                    if get_number(c) > 0 {
                                        let cstep = get_duration(c) / get_number(c) as f64;
                                        let written = snprintf(
                                            buf.as_mut_ptr().add(pos),
                                            2048 - pos,
                                            b"\n      %-11s %8.1f  (%5.1f%%)\0".as_ptr() as *const i8,
                                            timer_names[c].as_ptr().add(4),
                                            cstep * 1000.0,
                                            100.0 * cstep / tstep);
                                        if written > 0 { pos += written as usize; }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        let other = tstep - components;
        let written = snprintf(
            buf.as_mut_ptr().add(pos),
            2048 - pos,
            b"%s  %-15s %8.1f  (%5.1f%%)\0".as_ptr() as *const i8,
            if pos > 0 { b"\n\0".as_ptr() } else { b"\0".as_ptr() },
            b"other\0".as_ptr(),
            other * 1000.0,
            100.0 * other / tstep);
        if written > 0 { pos += written as usize; }

        let written = snprintf(
            buf.as_mut_ptr().add(pos),
            2048 - pos,
            b"%s  %-15s %8.1f\0".as_ptr() as *const i8,
            if pos > 0 { b"\n\0".as_ptr() } else { b"\0".as_ptr() },
            b"total\0".as_ptr(),
            tstep * 1000.0);
        if written > 0 { pos += written as usize; }

        // emit log message
        let mut msg = mjLogMessage {
            level: 0,  // mjLOG_INFO
            topic: 1,  // mjTOPIC_TIME_STP
            subject: [0i8; 1024],
            body: buf.as_ptr(),
            func: std::ptr::null(),
            file: std::ptr::null(),
            line: 0,
            timestamp: false,
            _pad_0: [0u8; 3],
        };
        snprintf(
            msg.subject.as_mut_ptr(),
            std::mem::size_of_val(&msg.subject),
            b"average time per step (%d steps, units: \xc2\xb5s)\0".as_ptr() as *const i8,
            nstep);
        crate::engine::engine_util_errmem::mju_message(&msg);
    }
}

/// C: sensorSize (engine/engine_io.c:1685)
#[allow(unused_variables, non_snake_case)]
pub fn sensor_size(sensor_type: u32, sensor_dim: i32) -> i32 {
    use crate::types::*;

    match sensor_type {
        mjtSensor_mjSENS_TOUCH
        | mjtSensor_mjSENS_RANGEFINDER
        | mjtSensor_mjSENS_JOINTPOS
        | mjtSensor_mjSENS_JOINTVEL
        | mjtSensor_mjSENS_TENDONPOS
        | mjtSensor_mjSENS_TENDONVEL
        | mjtSensor_mjSENS_ACTUATORPOS
        | mjtSensor_mjSENS_ACTUATORVEL
        | mjtSensor_mjSENS_ACTUATORFRC
        | mjtSensor_mjSENS_JOINTACTFRC
        | mjtSensor_mjSENS_TENDONACTFRC
        | mjtSensor_mjSENS_JOINTLIMITPOS
        | mjtSensor_mjSENS_JOINTLIMITVEL
        | mjtSensor_mjSENS_JOINTLIMITFRC
        | mjtSensor_mjSENS_TENDONLIMITPOS
        | mjtSensor_mjSENS_TENDONLIMITVEL
        | mjtSensor_mjSENS_TENDONLIMITFRC
        | mjtSensor_mjSENS_GEOMDIST
        | mjtSensor_mjSENS_INSIDESITE
        | mjtSensor_mjSENS_E_POTENTIAL
        | mjtSensor_mjSENS_E_KINETIC
        | mjtSensor_mjSENS_CLOCK => 1,

        mjtSensor_mjSENS_CAMPROJECTION => 2,

        mjtSensor_mjSENS_ACCELEROMETER
        | mjtSensor_mjSENS_VELOCIMETER
        | mjtSensor_mjSENS_GYRO
        | mjtSensor_mjSENS_FORCE
        | mjtSensor_mjSENS_TORQUE
        | mjtSensor_mjSENS_MAGNETOMETER
        | mjtSensor_mjSENS_BALLANGVEL
        | mjtSensor_mjSENS_FRAMEPOS
        | mjtSensor_mjSENS_FRAMEXAXIS
        | mjtSensor_mjSENS_FRAMEYAXIS
        | mjtSensor_mjSENS_FRAMEZAXIS
        | mjtSensor_mjSENS_FRAMELINVEL
        | mjtSensor_mjSENS_FRAMEANGVEL
        | mjtSensor_mjSENS_FRAMELINACC
        | mjtSensor_mjSENS_FRAMEANGACC
        | mjtSensor_mjSENS_SUBTREECOM
        | mjtSensor_mjSENS_SUBTREELINVEL
        | mjtSensor_mjSENS_SUBTREEANGMOM
        | mjtSensor_mjSENS_GEOMNORMAL => 3,

        mjtSensor_mjSENS_GEOMFROMTO => 6,

        mjtSensor_mjSENS_BALLQUAT
        | mjtSensor_mjSENS_FRAMEQUAT => 4,

        mjtSensor_mjSENS_CONTACT
        | mjtSensor_mjSENS_TACTILE
        | mjtSensor_mjSENS_USER => sensor_dim,

        mjtSensor_mjSENS_PLUGIN => -1,

        _ => -1,
    }
}

/// C: numObjects (engine/engine_io.c:1759)
#[allow(unused_variables, non_snake_case)]
pub fn num_objects(m: *const mjModel, objtype: u32) -> i32 {
    use crate::types::*;

    // SAFETY: m is a valid mjModel pointer (caller contract)
    unsafe {
        match objtype {
            mjtObj_mjOBJ_DEFAULT | mjtObj_mjOBJ_FRAME | mjtObj_mjOBJ_UNKNOWN | mjtObj_mjOBJ_MODEL => -1,
            mjtObj_mjOBJ_BODY | mjtObj_mjOBJ_XBODY => (*m).nbody as i32,
            mjtObj_mjOBJ_JOINT => (*m).njnt as i32,
            mjtObj_mjOBJ_DOF => (*m).nv as i32,
            mjtObj_mjOBJ_GEOM => (*m).ngeom as i32,
            mjtObj_mjOBJ_SITE => (*m).nsite as i32,
            mjtObj_mjOBJ_CAMERA => (*m).ncam as i32,
            mjtObj_mjOBJ_LIGHT => (*m).nlight as i32,
            mjtObj_mjOBJ_FLEX => (*m).nflex as i32,
            mjtObj_mjOBJ_MESH => (*m).nmesh as i32,
            mjtObj_mjOBJ_SKIN => (*m).nskin as i32,
            mjtObj_mjOBJ_HFIELD => (*m).nhfield as i32,
            mjtObj_mjOBJ_TEXTURE => (*m).ntex as i32,
            mjtObj_mjOBJ_MATERIAL => (*m).nmat as i32,
            mjtObj_mjOBJ_PAIR => (*m).npair as i32,
            mjtObj_mjOBJ_EXCLUDE => (*m).nexclude as i32,
            mjtObj_mjOBJ_EQUALITY => (*m).neq as i32,
            mjtObj_mjOBJ_TENDON => (*m).ntendon as i32,
            mjtObj_mjOBJ_ACTUATOR => (*m).nu as i32,
            mjtObj_mjOBJ_SENSOR => (*m).nsensor as i32,
            mjtObj_mjOBJ_NUMERIC => (*m).nnumeric as i32,
            mjtObj_mjOBJ_TEXT => (*m).ntext as i32,
            mjtObj_mjOBJ_TUPLE => (*m).ntuple as i32,
            mjtObj_mjOBJ_KEY => (*m).nkey as i32,
            mjtObj_mjOBJ_PLUGIN => (*m).nplugin as i32,
            mjtObj_mjNOBJECT => -2,
            _ => -2,
        }
    }
}

/// C: mj_makeModel (engine/engine_io.h:50)
/// Calls: freeModelBuffers, mj_defaultOption, mj_defaultStatistic, mj_defaultVisual, mj_setPtrModel, mju_free, mju_malloc, mju_message, mju_warning, safeAddToBufferSize
#[allow(unused_variables, non_snake_case)]
pub fn mj_make_model(dest: *mut *mut mjModel, nq: i64, nv: i64, nu: i64, na: i64, nbody: i64, nbvh: i64, nbvhstatic: i64, nbvhdynamic: i64, noct: i64, njnt: i64, ntree: i64, nM: i64, nB: i64, nC: i64, nD: i64, ngeom: i64, nsite: i64, ncam: i64, nlight: i64, nflex: i64, nflexnode: i64, nflexvert: i64, nflexedge: i64, nflexelem: i64, nflexelemdata: i64, nflexstiffness: i64, nflexbending: i64, nflexelemedge: i64, nflexshelldata: i64, nflexevpair: i64, nflextexcoord: i64, nJfe: i64, nJfv: i64, nmesh: i64, nmeshvert: i64, nmeshnormal: i64, nmeshtexcoord: i64, nmeshface: i64, nmeshgraph: i64, nmeshpoly: i64, nmeshpolyvert: i64, nmeshpolymap: i64, nskin: i64, nskinvert: i64, nskintexvert: i64, nskinface: i64, nskinbone: i64, nskinbonevert: i64, nhfield: i64, nhfielddata: i64, ntex: i64, ntexdata: i64, nmat: i64, npair: i64, nexclude: i64, neq: i64, ntendon: i64, nJten: i64, nwrap: i64, nsensor: i64, nnumeric: i64, nnumericdata: i64, ntext: i64, ntextdata: i64, ntuple: i64, ntupledata: i64, nkey: i64, nmocap: i64, nplugin: i64, npluginattr: i64, nuser_body: i64, nuser_jnt: i64, nuser_geom: i64, nuser_site: i64, nuser_cam: i64, nuser_tendon: i64, nuser_actuator: i64, nuser_sensor: i64, nnames: i64, npaths: i64) {
    todo!() // mj_makeModel
}

/// C: mj_copyModel (engine/engine_io.h:69)
/// Calls: mj_deleteModel, mj_makeModel, mj_setPtrModel, mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_copy_model(dest: *mut mjModel, src: *const mjModel) -> *mut mjModel {
    todo!() // mj_copyModel
}

/// C: mjv_copyModel (engine/engine_io.h:72)
/// Calls: mj_setPtrModel, mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mjv_copy_model(dest: *mut mjModel, src: *const mjModel) {
    // SAFETY: dest and src are valid mjModel pointers (caller contract)
    unsafe {
        // check sizes
        if (*dest).nbuffer != (*src).nbuffer {
            crate::engine::engine_util_errmem::mju_error(
                b"dest and src models have different buffer size\0".as_ptr() as *const i8);
            return;
        }

        // save buffer ptr, copy struct, restore buffer and set pointers
        let save_bufptr = (*dest).buffer;
        std::ptr::copy_nonoverlapping(src as *const u8, dest as *mut u8, std::mem::size_of::<mjModel>());
        (*dest).buffer = save_bufptr;
        mj_set_ptr_model(dest);

        // MJMODEL_POINTERS: copy buffer contents
        std::ptr::copy_nonoverlapping((*src).qpos0 as *const u8, (*dest).qpos0 as *mut u8, 8 * (*src).nq as usize * 1);
        std::ptr::copy_nonoverlapping((*src).qpos_spring as *const u8, (*dest).qpos_spring as *mut u8, 8 * (*src).nq as usize * 1);
        std::ptr::copy_nonoverlapping((*src).body_parentid as *const u8, (*dest).body_parentid as *mut u8, 4 * (*src).nbody as usize * 1);
        std::ptr::copy_nonoverlapping((*src).body_rootid as *const u8, (*dest).body_rootid as *mut u8, 4 * (*src).nbody as usize * 1);
        std::ptr::copy_nonoverlapping((*src).body_weldid as *const u8, (*dest).body_weldid as *mut u8, 4 * (*src).nbody as usize * 1);
        std::ptr::copy_nonoverlapping((*src).body_mocapid as *const u8, (*dest).body_mocapid as *mut u8, 4 * (*src).nbody as usize * 1);
        std::ptr::copy_nonoverlapping((*src).body_jntnum as *const u8, (*dest).body_jntnum as *mut u8, 4 * (*src).nbody as usize * 1);
        std::ptr::copy_nonoverlapping((*src).body_jntadr as *const u8, (*dest).body_jntadr as *mut u8, 4 * (*src).nbody as usize * 1);
        std::ptr::copy_nonoverlapping((*src).body_dofnum as *const u8, (*dest).body_dofnum as *mut u8, 4 * (*src).nbody as usize * 1);
        std::ptr::copy_nonoverlapping((*src).body_dofadr as *const u8, (*dest).body_dofadr as *mut u8, 4 * (*src).nbody as usize * 1);
        std::ptr::copy_nonoverlapping((*src).body_treeid as *const u8, (*dest).body_treeid as *mut u8, 4 * (*src).nbody as usize * 1);
        std::ptr::copy_nonoverlapping((*src).body_geomnum as *const u8, (*dest).body_geomnum as *mut u8, 4 * (*src).nbody as usize * 1);
        std::ptr::copy_nonoverlapping((*src).body_geomadr as *const u8, (*dest).body_geomadr as *mut u8, 4 * (*src).nbody as usize * 1);
        std::ptr::copy_nonoverlapping((*src).body_simple as *const u8, (*dest).body_simple as *mut u8, 1 * (*src).nbody as usize * 1);
        std::ptr::copy_nonoverlapping((*src).body_sameframe as *const u8, (*dest).body_sameframe as *mut u8, 1 * (*src).nbody as usize * 1);
        std::ptr::copy_nonoverlapping((*src).body_pos as *const u8, (*dest).body_pos as *mut u8, 8 * (*src).nbody as usize * 3);
        std::ptr::copy_nonoverlapping((*src).body_quat as *const u8, (*dest).body_quat as *mut u8, 8 * (*src).nbody as usize * 4);
        std::ptr::copy_nonoverlapping((*src).body_ipos as *const u8, (*dest).body_ipos as *mut u8, 8 * (*src).nbody as usize * 3);
        std::ptr::copy_nonoverlapping((*src).body_iquat as *const u8, (*dest).body_iquat as *mut u8, 8 * (*src).nbody as usize * 4);
        std::ptr::copy_nonoverlapping((*src).body_mass as *const u8, (*dest).body_mass as *mut u8, 8 * (*src).nbody as usize * 1);
        std::ptr::copy_nonoverlapping((*src).body_subtreemass as *const u8, (*dest).body_subtreemass as *mut u8, 8 * (*src).nbody as usize * 1);
        std::ptr::copy_nonoverlapping((*src).body_inertia as *const u8, (*dest).body_inertia as *mut u8, 8 * (*src).nbody as usize * 3);
        std::ptr::copy_nonoverlapping((*src).body_invweight0 as *const u8, (*dest).body_invweight0 as *mut u8, 8 * (*src).nbody as usize * 2);
        std::ptr::copy_nonoverlapping((*src).body_gravcomp as *const u8, (*dest).body_gravcomp as *mut u8, 8 * (*src).nbody as usize * 1);
        std::ptr::copy_nonoverlapping((*src).body_margin as *const u8, (*dest).body_margin as *mut u8, 8 * (*src).nbody as usize * 1);
        std::ptr::copy_nonoverlapping((*src).body_user as *const u8, (*dest).body_user as *mut u8, 8 * (*src).nbody as usize * (*src).nuser_body as usize);
        std::ptr::copy_nonoverlapping((*src).body_plugin as *const u8, (*dest).body_plugin as *mut u8, 4 * (*src).nbody as usize * 1);
        std::ptr::copy_nonoverlapping((*src).body_contype as *const u8, (*dest).body_contype as *mut u8, 4 * (*src).nbody as usize * 1);
        std::ptr::copy_nonoverlapping((*src).body_conaffinity as *const u8, (*dest).body_conaffinity as *mut u8, 4 * (*src).nbody as usize * 1);
        std::ptr::copy_nonoverlapping((*src).body_bvhadr as *const u8, (*dest).body_bvhadr as *mut u8, 4 * (*src).nbody as usize * 1);
        std::ptr::copy_nonoverlapping((*src).body_bvhnum as *const u8, (*dest).body_bvhnum as *mut u8, 4 * (*src).nbody as usize * 1);
        std::ptr::copy_nonoverlapping((*src).bvh_depth as *const u8, (*dest).bvh_depth as *mut u8, 4 * (*src).nbvh as usize * 1);
        std::ptr::copy_nonoverlapping((*src).bvh_child as *const u8, (*dest).bvh_child as *mut u8, 4 * (*src).nbvh as usize * 2);
        std::ptr::copy_nonoverlapping((*src).bvh_nodeid as *const u8, (*dest).bvh_nodeid as *mut u8, 4 * (*src).nbvh as usize * 1);
        std::ptr::copy_nonoverlapping((*src).bvh_aabb as *const u8, (*dest).bvh_aabb as *mut u8, 8 * (*src).nbvhstatic as usize * 6);
        std::ptr::copy_nonoverlapping((*src).oct_depth as *const u8, (*dest).oct_depth as *mut u8, 4 * (*src).noct as usize * 1);
        std::ptr::copy_nonoverlapping((*src).oct_child as *const u8, (*dest).oct_child as *mut u8, 4 * (*src).noct as usize * 8);
        std::ptr::copy_nonoverlapping((*src).oct_aabb as *const u8, (*dest).oct_aabb as *mut u8, 8 * (*src).noct as usize * 6);
        std::ptr::copy_nonoverlapping((*src).oct_coeff as *const u8, (*dest).oct_coeff as *mut u8, 8 * (*src).noct as usize * 8);
        std::ptr::copy_nonoverlapping((*src).jnt_type as *const u8, (*dest).jnt_type as *mut u8, 4 * (*src).njnt as usize * 1);
        std::ptr::copy_nonoverlapping((*src).jnt_qposadr as *const u8, (*dest).jnt_qposadr as *mut u8, 4 * (*src).njnt as usize * 1);
        std::ptr::copy_nonoverlapping((*src).jnt_dofadr as *const u8, (*dest).jnt_dofadr as *mut u8, 4 * (*src).njnt as usize * 1);
        std::ptr::copy_nonoverlapping((*src).jnt_bodyid as *const u8, (*dest).jnt_bodyid as *mut u8, 4 * (*src).njnt as usize * 1);
        std::ptr::copy_nonoverlapping((*src).jnt_actuatorid as *const u8, (*dest).jnt_actuatorid as *mut u8, 4 * (*src).njnt as usize * 1);
        std::ptr::copy_nonoverlapping((*src).jnt_group as *const u8, (*dest).jnt_group as *mut u8, 4 * (*src).njnt as usize * 1);
        std::ptr::copy_nonoverlapping((*src).jnt_limited as *const u8, (*dest).jnt_limited as *mut u8, 1 * (*src).njnt as usize * 1);
        std::ptr::copy_nonoverlapping((*src).jnt_actfrclimited as *const u8, (*dest).jnt_actfrclimited as *mut u8, 1 * (*src).njnt as usize * 1);
        std::ptr::copy_nonoverlapping((*src).jnt_actgravcomp as *const u8, (*dest).jnt_actgravcomp as *mut u8, 1 * (*src).njnt as usize * 1);
        std::ptr::copy_nonoverlapping((*src).jnt_solref as *const u8, (*dest).jnt_solref as *mut u8, 8 * (*src).njnt as usize * 2);
        std::ptr::copy_nonoverlapping((*src).jnt_solimp as *const u8, (*dest).jnt_solimp as *mut u8, 8 * (*src).njnt as usize * 5);
        std::ptr::copy_nonoverlapping((*src).jnt_pos as *const u8, (*dest).jnt_pos as *mut u8, 8 * (*src).njnt as usize * 3);
        std::ptr::copy_nonoverlapping((*src).jnt_axis as *const u8, (*dest).jnt_axis as *mut u8, 8 * (*src).njnt as usize * 3);
        std::ptr::copy_nonoverlapping((*src).jnt_stiffness as *const u8, (*dest).jnt_stiffness as *mut u8, 8 * (*src).njnt as usize * 1);
        std::ptr::copy_nonoverlapping((*src).jnt_stiffnesspoly as *const u8, (*dest).jnt_stiffnesspoly as *mut u8, 8 * (*src).njnt as usize * 2);
        std::ptr::copy_nonoverlapping((*src).jnt_range as *const u8, (*dest).jnt_range as *mut u8, 8 * (*src).njnt as usize * 2);
        std::ptr::copy_nonoverlapping((*src).jnt_actfrcrange as *const u8, (*dest).jnt_actfrcrange as *mut u8, 8 * (*src).njnt as usize * 2);
        std::ptr::copy_nonoverlapping((*src).jnt_margin as *const u8, (*dest).jnt_margin as *mut u8, 8 * (*src).njnt as usize * 1);
        std::ptr::copy_nonoverlapping((*src).jnt_user as *const u8, (*dest).jnt_user as *mut u8, 8 * (*src).njnt as usize * (*src).nuser_jnt as usize);
        std::ptr::copy_nonoverlapping((*src).dof_bodyid as *const u8, (*dest).dof_bodyid as *mut u8, 4 * (*src).nv as usize * 1);
        std::ptr::copy_nonoverlapping((*src).dof_jntid as *const u8, (*dest).dof_jntid as *mut u8, 4 * (*src).nv as usize * 1);
        std::ptr::copy_nonoverlapping((*src).dof_parentid as *const u8, (*dest).dof_parentid as *mut u8, 4 * (*src).nv as usize * 1);
        std::ptr::copy_nonoverlapping((*src).dof_treeid as *const u8, (*dest).dof_treeid as *mut u8, 4 * (*src).nv as usize * 1);
        std::ptr::copy_nonoverlapping((*src).dof_Madr as *const u8, (*dest).dof_Madr as *mut u8, 4 * (*src).nv as usize * 1);
        std::ptr::copy_nonoverlapping((*src).dof_simplenum as *const u8, (*dest).dof_simplenum as *mut u8, 4 * (*src).nv as usize * 1);
        std::ptr::copy_nonoverlapping((*src).dof_solref as *const u8, (*dest).dof_solref as *mut u8, 8 * (*src).nv as usize * 2);
        std::ptr::copy_nonoverlapping((*src).dof_solimp as *const u8, (*dest).dof_solimp as *mut u8, 8 * (*src).nv as usize * 5);
        std::ptr::copy_nonoverlapping((*src).dof_frictionloss as *const u8, (*dest).dof_frictionloss as *mut u8, 8 * (*src).nv as usize * 1);
        std::ptr::copy_nonoverlapping((*src).dof_armature as *const u8, (*dest).dof_armature as *mut u8, 8 * (*src).nv as usize * 1);
        std::ptr::copy_nonoverlapping((*src).dof_damping as *const u8, (*dest).dof_damping as *mut u8, 8 * (*src).nv as usize * 1);
        std::ptr::copy_nonoverlapping((*src).dof_dampingpoly as *const u8, (*dest).dof_dampingpoly as *mut u8, 8 * (*src).nv as usize * 2);
        std::ptr::copy_nonoverlapping((*src).dof_invweight0 as *const u8, (*dest).dof_invweight0 as *mut u8, 8 * (*src).nv as usize * 1);
        std::ptr::copy_nonoverlapping((*src).dof_M0 as *const u8, (*dest).dof_M0 as *mut u8, 8 * (*src).nv as usize * 1);
        std::ptr::copy_nonoverlapping((*src).dof_length as *const u8, (*dest).dof_length as *mut u8, 8 * (*src).nv as usize * 1);
        std::ptr::copy_nonoverlapping((*src).tree_bodyadr as *const u8, (*dest).tree_bodyadr as *mut u8, 4 * (*src).ntree as usize * 1);
        std::ptr::copy_nonoverlapping((*src).tree_bodynum as *const u8, (*dest).tree_bodynum as *mut u8, 4 * (*src).ntree as usize * 1);
        std::ptr::copy_nonoverlapping((*src).tree_dofadr as *const u8, (*dest).tree_dofadr as *mut u8, 4 * (*src).ntree as usize * 1);
        std::ptr::copy_nonoverlapping((*src).tree_dofnum as *const u8, (*dest).tree_dofnum as *mut u8, 4 * (*src).ntree as usize * 1);
        std::ptr::copy_nonoverlapping((*src).tree_sleep_policy as *const u8, (*dest).tree_sleep_policy as *mut u8, 4 * (*src).ntree as usize * 1);
        std::ptr::copy_nonoverlapping((*src).geom_type as *const u8, (*dest).geom_type as *mut u8, 4 * (*src).ngeom as usize * 1);
        std::ptr::copy_nonoverlapping((*src).geom_contype as *const u8, (*dest).geom_contype as *mut u8, 4 * (*src).ngeom as usize * 1);
        std::ptr::copy_nonoverlapping((*src).geom_conaffinity as *const u8, (*dest).geom_conaffinity as *mut u8, 4 * (*src).ngeom as usize * 1);
        std::ptr::copy_nonoverlapping((*src).geom_condim as *const u8, (*dest).geom_condim as *mut u8, 4 * (*src).ngeom as usize * 1);
        std::ptr::copy_nonoverlapping((*src).geom_bodyid as *const u8, (*dest).geom_bodyid as *mut u8, 4 * (*src).ngeom as usize * 1);
        std::ptr::copy_nonoverlapping((*src).geom_dataid as *const u8, (*dest).geom_dataid as *mut u8, 4 * (*src).ngeom as usize * 1);
        std::ptr::copy_nonoverlapping((*src).geom_matid as *const u8, (*dest).geom_matid as *mut u8, 4 * (*src).ngeom as usize * 1);
        std::ptr::copy_nonoverlapping((*src).geom_group as *const u8, (*dest).geom_group as *mut u8, 4 * (*src).ngeom as usize * 1);
        std::ptr::copy_nonoverlapping((*src).geom_priority as *const u8, (*dest).geom_priority as *mut u8, 4 * (*src).ngeom as usize * 1);
        std::ptr::copy_nonoverlapping((*src).geom_plugin as *const u8, (*dest).geom_plugin as *mut u8, 4 * (*src).ngeom as usize * 1);
        std::ptr::copy_nonoverlapping((*src).geom_sameframe as *const u8, (*dest).geom_sameframe as *mut u8, 1 * (*src).ngeom as usize * 1);
        std::ptr::copy_nonoverlapping((*src).geom_solmix as *const u8, (*dest).geom_solmix as *mut u8, 8 * (*src).ngeom as usize * 1);
        std::ptr::copy_nonoverlapping((*src).geom_solref as *const u8, (*dest).geom_solref as *mut u8, 8 * (*src).ngeom as usize * 2);
        std::ptr::copy_nonoverlapping((*src).geom_solimp as *const u8, (*dest).geom_solimp as *mut u8, 8 * (*src).ngeom as usize * 5);
        std::ptr::copy_nonoverlapping((*src).geom_size as *const u8, (*dest).geom_size as *mut u8, 8 * (*src).ngeom as usize * 3);
        std::ptr::copy_nonoverlapping((*src).geom_aabb as *const u8, (*dest).geom_aabb as *mut u8, 8 * (*src).ngeom as usize * 6);
        std::ptr::copy_nonoverlapping((*src).geom_rbound as *const u8, (*dest).geom_rbound as *mut u8, 8 * (*src).ngeom as usize * 1);
        std::ptr::copy_nonoverlapping((*src).geom_pos as *const u8, (*dest).geom_pos as *mut u8, 8 * (*src).ngeom as usize * 3);
        std::ptr::copy_nonoverlapping((*src).geom_quat as *const u8, (*dest).geom_quat as *mut u8, 8 * (*src).ngeom as usize * 4);
        std::ptr::copy_nonoverlapping((*src).geom_friction as *const u8, (*dest).geom_friction as *mut u8, 8 * (*src).ngeom as usize * 3);
        std::ptr::copy_nonoverlapping((*src).geom_margin as *const u8, (*dest).geom_margin as *mut u8, 8 * (*src).ngeom as usize * 1);
        std::ptr::copy_nonoverlapping((*src).geom_gap as *const u8, (*dest).geom_gap as *mut u8, 8 * (*src).ngeom as usize * 1);
        std::ptr::copy_nonoverlapping((*src).geom_fluid as *const u8, (*dest).geom_fluid as *mut u8, 8 * (*src).ngeom as usize * 12);
        std::ptr::copy_nonoverlapping((*src).geom_user as *const u8, (*dest).geom_user as *mut u8, 8 * (*src).ngeom as usize * (*src).nuser_geom as usize);
        std::ptr::copy_nonoverlapping((*src).geom_rgba as *const u8, (*dest).geom_rgba as *mut u8, 4 * (*src).ngeom as usize * 4);
        std::ptr::copy_nonoverlapping((*src).site_type as *const u8, (*dest).site_type as *mut u8, 4 * (*src).nsite as usize * 1);
        std::ptr::copy_nonoverlapping((*src).site_bodyid as *const u8, (*dest).site_bodyid as *mut u8, 4 * (*src).nsite as usize * 1);
        std::ptr::copy_nonoverlapping((*src).site_matid as *const u8, (*dest).site_matid as *mut u8, 4 * (*src).nsite as usize * 1);
        std::ptr::copy_nonoverlapping((*src).site_group as *const u8, (*dest).site_group as *mut u8, 4 * (*src).nsite as usize * 1);
        std::ptr::copy_nonoverlapping((*src).site_sameframe as *const u8, (*dest).site_sameframe as *mut u8, 1 * (*src).nsite as usize * 1);
        std::ptr::copy_nonoverlapping((*src).site_size as *const u8, (*dest).site_size as *mut u8, 8 * (*src).nsite as usize * 3);
        std::ptr::copy_nonoverlapping((*src).site_pos as *const u8, (*dest).site_pos as *mut u8, 8 * (*src).nsite as usize * 3);
        std::ptr::copy_nonoverlapping((*src).site_quat as *const u8, (*dest).site_quat as *mut u8, 8 * (*src).nsite as usize * 4);
        std::ptr::copy_nonoverlapping((*src).site_user as *const u8, (*dest).site_user as *mut u8, 8 * (*src).nsite as usize * (*src).nuser_site as usize);
        std::ptr::copy_nonoverlapping((*src).site_rgba as *const u8, (*dest).site_rgba as *mut u8, 4 * (*src).nsite as usize * 4);
        std::ptr::copy_nonoverlapping((*src).cam_mode as *const u8, (*dest).cam_mode as *mut u8, 4 * (*src).ncam as usize * 1);
        std::ptr::copy_nonoverlapping((*src).cam_bodyid as *const u8, (*dest).cam_bodyid as *mut u8, 4 * (*src).ncam as usize * 1);
        std::ptr::copy_nonoverlapping((*src).cam_targetbodyid as *const u8, (*dest).cam_targetbodyid as *mut u8, 4 * (*src).ncam as usize * 1);
        std::ptr::copy_nonoverlapping((*src).cam_pos as *const u8, (*dest).cam_pos as *mut u8, 8 * (*src).ncam as usize * 3);
        std::ptr::copy_nonoverlapping((*src).cam_quat as *const u8, (*dest).cam_quat as *mut u8, 8 * (*src).ncam as usize * 4);
        std::ptr::copy_nonoverlapping((*src).cam_poscom0 as *const u8, (*dest).cam_poscom0 as *mut u8, 8 * (*src).ncam as usize * 3);
        std::ptr::copy_nonoverlapping((*src).cam_pos0 as *const u8, (*dest).cam_pos0 as *mut u8, 8 * (*src).ncam as usize * 3);
        std::ptr::copy_nonoverlapping((*src).cam_mat0 as *const u8, (*dest).cam_mat0 as *mut u8, 8 * (*src).ncam as usize * 9);
        std::ptr::copy_nonoverlapping((*src).cam_projection as *const u8, (*dest).cam_projection as *mut u8, 4 * (*src).ncam as usize * 1);
        std::ptr::copy_nonoverlapping((*src).cam_fovy as *const u8, (*dest).cam_fovy as *mut u8, 8 * (*src).ncam as usize * 1);
        std::ptr::copy_nonoverlapping((*src).cam_ipd as *const u8, (*dest).cam_ipd as *mut u8, 8 * (*src).ncam as usize * 1);
        std::ptr::copy_nonoverlapping((*src).cam_resolution as *const u8, (*dest).cam_resolution as *mut u8, 4 * (*src).ncam as usize * 2);
        std::ptr::copy_nonoverlapping((*src).cam_output as *const u8, (*dest).cam_output as *mut u8, 4 * (*src).ncam as usize * 1);
        std::ptr::copy_nonoverlapping((*src).cam_sensorsize as *const u8, (*dest).cam_sensorsize as *mut u8, 4 * (*src).ncam as usize * 2);
        std::ptr::copy_nonoverlapping((*src).cam_intrinsic as *const u8, (*dest).cam_intrinsic as *mut u8, 4 * (*src).ncam as usize * 4);
        std::ptr::copy_nonoverlapping((*src).cam_user as *const u8, (*dest).cam_user as *mut u8, 8 * (*src).ncam as usize * (*src).nuser_cam as usize);
        std::ptr::copy_nonoverlapping((*src).light_mode as *const u8, (*dest).light_mode as *mut u8, 4 * (*src).nlight as usize * 1);
        std::ptr::copy_nonoverlapping((*src).light_bodyid as *const u8, (*dest).light_bodyid as *mut u8, 4 * (*src).nlight as usize * 1);
        std::ptr::copy_nonoverlapping((*src).light_targetbodyid as *const u8, (*dest).light_targetbodyid as *mut u8, 4 * (*src).nlight as usize * 1);
        std::ptr::copy_nonoverlapping((*src).light_type as *const u8, (*dest).light_type as *mut u8, 4 * (*src).nlight as usize * 1);
        std::ptr::copy_nonoverlapping((*src).light_texid as *const u8, (*dest).light_texid as *mut u8, 4 * (*src).nlight as usize * 1);
        std::ptr::copy_nonoverlapping((*src).light_castshadow as *const u8, (*dest).light_castshadow as *mut u8, 1 * (*src).nlight as usize * 1);
        std::ptr::copy_nonoverlapping((*src).light_bulbradius as *const u8, (*dest).light_bulbradius as *mut u8, 4 * (*src).nlight as usize * 1);
        std::ptr::copy_nonoverlapping((*src).light_intensity as *const u8, (*dest).light_intensity as *mut u8, 4 * (*src).nlight as usize * 1);
        std::ptr::copy_nonoverlapping((*src).light_range as *const u8, (*dest).light_range as *mut u8, 4 * (*src).nlight as usize * 1);
        std::ptr::copy_nonoverlapping((*src).light_active as *const u8, (*dest).light_active as *mut u8, 1 * (*src).nlight as usize * 1);
        std::ptr::copy_nonoverlapping((*src).light_pos as *const u8, (*dest).light_pos as *mut u8, 8 * (*src).nlight as usize * 3);
        std::ptr::copy_nonoverlapping((*src).light_dir as *const u8, (*dest).light_dir as *mut u8, 8 * (*src).nlight as usize * 3);
        std::ptr::copy_nonoverlapping((*src).light_poscom0 as *const u8, (*dest).light_poscom0 as *mut u8, 8 * (*src).nlight as usize * 3);
        std::ptr::copy_nonoverlapping((*src).light_pos0 as *const u8, (*dest).light_pos0 as *mut u8, 8 * (*src).nlight as usize * 3);
        std::ptr::copy_nonoverlapping((*src).light_dir0 as *const u8, (*dest).light_dir0 as *mut u8, 8 * (*src).nlight as usize * 3);
        std::ptr::copy_nonoverlapping((*src).light_attenuation as *const u8, (*dest).light_attenuation as *mut u8, 4 * (*src).nlight as usize * 3);
        std::ptr::copy_nonoverlapping((*src).light_cutoff as *const u8, (*dest).light_cutoff as *mut u8, 4 * (*src).nlight as usize * 1);
        std::ptr::copy_nonoverlapping((*src).light_exponent as *const u8, (*dest).light_exponent as *mut u8, 4 * (*src).nlight as usize * 1);
        std::ptr::copy_nonoverlapping((*src).light_ambient as *const u8, (*dest).light_ambient as *mut u8, 4 * (*src).nlight as usize * 3);
        std::ptr::copy_nonoverlapping((*src).light_diffuse as *const u8, (*dest).light_diffuse as *mut u8, 4 * (*src).nlight as usize * 3);
        std::ptr::copy_nonoverlapping((*src).light_specular as *const u8, (*dest).light_specular as *mut u8, 4 * (*src).nlight as usize * 3);
        std::ptr::copy_nonoverlapping((*src).flex_contype as *const u8, (*dest).flex_contype as *mut u8, 4 * (*src).nflex as usize * 1);
        std::ptr::copy_nonoverlapping((*src).flex_conaffinity as *const u8, (*dest).flex_conaffinity as *mut u8, 4 * (*src).nflex as usize * 1);
        std::ptr::copy_nonoverlapping((*src).flex_condim as *const u8, (*dest).flex_condim as *mut u8, 4 * (*src).nflex as usize * 1);
        std::ptr::copy_nonoverlapping((*src).flex_priority as *const u8, (*dest).flex_priority as *mut u8, 4 * (*src).nflex as usize * 1);
        std::ptr::copy_nonoverlapping((*src).flex_solmix as *const u8, (*dest).flex_solmix as *mut u8, 8 * (*src).nflex as usize * 1);
        std::ptr::copy_nonoverlapping((*src).flex_solref as *const u8, (*dest).flex_solref as *mut u8, 8 * (*src).nflex as usize * 2);
        std::ptr::copy_nonoverlapping((*src).flex_solimp as *const u8, (*dest).flex_solimp as *mut u8, 8 * (*src).nflex as usize * 5);
        std::ptr::copy_nonoverlapping((*src).flex_friction as *const u8, (*dest).flex_friction as *mut u8, 8 * (*src).nflex as usize * 3);
        std::ptr::copy_nonoverlapping((*src).flex_margin as *const u8, (*dest).flex_margin as *mut u8, 8 * (*src).nflex as usize * 1);
        std::ptr::copy_nonoverlapping((*src).flex_gap as *const u8, (*dest).flex_gap as *mut u8, 8 * (*src).nflex as usize * 1);
        std::ptr::copy_nonoverlapping((*src).flex_internal as *const u8, (*dest).flex_internal as *mut u8, 1 * (*src).nflex as usize * 1);
        std::ptr::copy_nonoverlapping((*src).flex_selfcollide as *const u8, (*dest).flex_selfcollide as *mut u8, 4 * (*src).nflex as usize * 1);
        std::ptr::copy_nonoverlapping((*src).flex_activelayers as *const u8, (*dest).flex_activelayers as *mut u8, 4 * (*src).nflex as usize * 1);
        std::ptr::copy_nonoverlapping((*src).flex_passive as *const u8, (*dest).flex_passive as *mut u8, 4 * (*src).nflex as usize * 1);
        std::ptr::copy_nonoverlapping((*src).flex_dim as *const u8, (*dest).flex_dim as *mut u8, 4 * (*src).nflex as usize * 1);
        std::ptr::copy_nonoverlapping((*src).flex_matid as *const u8, (*dest).flex_matid as *mut u8, 4 * (*src).nflex as usize * 1);
        std::ptr::copy_nonoverlapping((*src).flex_group as *const u8, (*dest).flex_group as *mut u8, 4 * (*src).nflex as usize * 1);
        std::ptr::copy_nonoverlapping((*src).flex_interp as *const u8, (*dest).flex_interp as *mut u8, 4 * (*src).nflex as usize * 1);
        std::ptr::copy_nonoverlapping((*src).flex_cellnum as *const u8, (*dest).flex_cellnum as *mut u8, 4 * (*src).nflex as usize * 3);
        std::ptr::copy_nonoverlapping((*src).flex_nodeadr as *const u8, (*dest).flex_nodeadr as *mut u8, 4 * (*src).nflex as usize * 1);
        std::ptr::copy_nonoverlapping((*src).flex_nodenum as *const u8, (*dest).flex_nodenum as *mut u8, 4 * (*src).nflex as usize * 1);
        std::ptr::copy_nonoverlapping((*src).flex_vertadr as *const u8, (*dest).flex_vertadr as *mut u8, 4 * (*src).nflex as usize * 1);
        std::ptr::copy_nonoverlapping((*src).flex_vertnum as *const u8, (*dest).flex_vertnum as *mut u8, 4 * (*src).nflex as usize * 1);
        std::ptr::copy_nonoverlapping((*src).flex_edgeadr as *const u8, (*dest).flex_edgeadr as *mut u8, 4 * (*src).nflex as usize * 1);
        std::ptr::copy_nonoverlapping((*src).flex_edgenum as *const u8, (*dest).flex_edgenum as *mut u8, 4 * (*src).nflex as usize * 1);
        std::ptr::copy_nonoverlapping((*src).flex_elemadr as *const u8, (*dest).flex_elemadr as *mut u8, 4 * (*src).nflex as usize * 1);
        std::ptr::copy_nonoverlapping((*src).flex_elemnum as *const u8, (*dest).flex_elemnum as *mut u8, 4 * (*src).nflex as usize * 1);
        std::ptr::copy_nonoverlapping((*src).flex_elemdataadr as *const u8, (*dest).flex_elemdataadr as *mut u8, 4 * (*src).nflex as usize * 1);
        std::ptr::copy_nonoverlapping((*src).flex_stiffnessadr as *const u8, (*dest).flex_stiffnessadr as *mut u8, 4 * (*src).nflex as usize * 1);
        std::ptr::copy_nonoverlapping((*src).flex_elemedgeadr as *const u8, (*dest).flex_elemedgeadr as *mut u8, 4 * (*src).nflex as usize * 1);
        std::ptr::copy_nonoverlapping((*src).flex_bendingadr as *const u8, (*dest).flex_bendingadr as *mut u8, 4 * (*src).nflex as usize * 1);
        std::ptr::copy_nonoverlapping((*src).flex_shellnum as *const u8, (*dest).flex_shellnum as *mut u8, 4 * (*src).nflex as usize * 1);
        std::ptr::copy_nonoverlapping((*src).flex_shelldataadr as *const u8, (*dest).flex_shelldataadr as *mut u8, 4 * (*src).nflex as usize * 1);
        std::ptr::copy_nonoverlapping((*src).flex_evpairadr as *const u8, (*dest).flex_evpairadr as *mut u8, 4 * (*src).nflex as usize * 1);
        std::ptr::copy_nonoverlapping((*src).flex_evpairnum as *const u8, (*dest).flex_evpairnum as *mut u8, 4 * (*src).nflex as usize * 1);
        std::ptr::copy_nonoverlapping((*src).flex_texcoordadr as *const u8, (*dest).flex_texcoordadr as *mut u8, 4 * (*src).nflex as usize * 1);
        std::ptr::copy_nonoverlapping((*src).flex_nodebodyid as *const u8, (*dest).flex_nodebodyid as *mut u8, 4 * (*src).nflexnode as usize * 1);
        std::ptr::copy_nonoverlapping((*src).flex_vertbodyid as *const u8, (*dest).flex_vertbodyid as *mut u8, 4 * (*src).nflexvert as usize * 1);
        std::ptr::copy_nonoverlapping((*src).flex_vertedgeadr as *const u8, (*dest).flex_vertedgeadr as *mut u8, 4 * (*src).nflexvert as usize * 1);
        std::ptr::copy_nonoverlapping((*src).flex_vertedgenum as *const u8, (*dest).flex_vertedgenum as *mut u8, 4 * (*src).nflexvert as usize * 1);
        std::ptr::copy_nonoverlapping((*src).flex_vertedge as *const u8, (*dest).flex_vertedge as *mut u8, 4 * (*src).nflexedge as usize * 2);
        std::ptr::copy_nonoverlapping((*src).flex_edge as *const u8, (*dest).flex_edge as *mut u8, 4 * (*src).nflexedge as usize * 2);
        std::ptr::copy_nonoverlapping((*src).flex_edgeflap as *const u8, (*dest).flex_edgeflap as *mut u8, 4 * (*src).nflexedge as usize * 2);
        std::ptr::copy_nonoverlapping((*src).flex_elem as *const u8, (*dest).flex_elem as *mut u8, 4 * (*src).nflexelemdata as usize * 1);
        std::ptr::copy_nonoverlapping((*src).flex_elemtexcoord as *const u8, (*dest).flex_elemtexcoord as *mut u8, 4 * (*src).nflexelemdata as usize * 1);
        std::ptr::copy_nonoverlapping((*src).flex_elemedge as *const u8, (*dest).flex_elemedge as *mut u8, 4 * (*src).nflexelemedge as usize * 1);
        std::ptr::copy_nonoverlapping((*src).flex_elemlayer as *const u8, (*dest).flex_elemlayer as *mut u8, 4 * (*src).nflexelem as usize * 1);
        std::ptr::copy_nonoverlapping((*src).flex_shell as *const u8, (*dest).flex_shell as *mut u8, 4 * (*src).nflexshelldata as usize * 1);
        std::ptr::copy_nonoverlapping((*src).flex_evpair as *const u8, (*dest).flex_evpair as *mut u8, 4 * (*src).nflexevpair as usize * 2);
        std::ptr::copy_nonoverlapping((*src).flex_vert as *const u8, (*dest).flex_vert as *mut u8, 8 * (*src).nflexvert as usize * 3);
        std::ptr::copy_nonoverlapping((*src).flex_vert0 as *const u8, (*dest).flex_vert0 as *mut u8, 8 * (*src).nflexvert as usize * 3);
        std::ptr::copy_nonoverlapping((*src).flex_vertmetric as *const u8, (*dest).flex_vertmetric as *mut u8, 8 * (*src).nflexvert as usize * 4);
        std::ptr::copy_nonoverlapping((*src).flex_node as *const u8, (*dest).flex_node as *mut u8, 8 * (*src).nflexnode as usize * 3);
        std::ptr::copy_nonoverlapping((*src).flex_node0 as *const u8, (*dest).flex_node0 as *mut u8, 8 * (*src).nflexnode as usize * 3);
        std::ptr::copy_nonoverlapping((*src).flexedge_length0 as *const u8, (*dest).flexedge_length0 as *mut u8, 8 * (*src).nflexedge as usize * 1);
        std::ptr::copy_nonoverlapping((*src).flexedge_invweight0 as *const u8, (*dest).flexedge_invweight0 as *mut u8, 8 * (*src).nflexedge as usize * 1);
        std::ptr::copy_nonoverlapping((*src).flex_radius as *const u8, (*dest).flex_radius as *mut u8, 8 * (*src).nflex as usize * 1);
        std::ptr::copy_nonoverlapping((*src).flex_size as *const u8, (*dest).flex_size as *mut u8, 8 * (*src).nflex as usize * 3);
        std::ptr::copy_nonoverlapping((*src).flex_stiffness as *const u8, (*dest).flex_stiffness as *mut u8, 8 * (*src).nflexstiffness as usize * 1);
        std::ptr::copy_nonoverlapping((*src).flex_bending as *const u8, (*dest).flex_bending as *mut u8, 8 * (*src).nflexbending as usize * 1);
        std::ptr::copy_nonoverlapping((*src).flex_damping as *const u8, (*dest).flex_damping as *mut u8, 8 * (*src).nflex as usize * 1);
        std::ptr::copy_nonoverlapping((*src).flex_edgestiffness as *const u8, (*dest).flex_edgestiffness as *mut u8, 8 * (*src).nflex as usize * 1);
        std::ptr::copy_nonoverlapping((*src).flex_edgedamping as *const u8, (*dest).flex_edgedamping as *mut u8, 8 * (*src).nflex as usize * 1);
        std::ptr::copy_nonoverlapping((*src).flex_edgeequality as *const u8, (*dest).flex_edgeequality as *mut u8, 4 * (*src).nflex as usize * 1);
        std::ptr::copy_nonoverlapping((*src).flex_rigid as *const u8, (*dest).flex_rigid as *mut u8, 1 * (*src).nflex as usize * 1);
        std::ptr::copy_nonoverlapping((*src).flexedge_rigid as *const u8, (*dest).flexedge_rigid as *mut u8, 1 * (*src).nflexedge as usize * 1);
        std::ptr::copy_nonoverlapping((*src).flex_centered as *const u8, (*dest).flex_centered as *mut u8, 1 * (*src).nflex as usize * 1);
        std::ptr::copy_nonoverlapping((*src).flex_flatskin as *const u8, (*dest).flex_flatskin as *mut u8, 1 * (*src).nflex as usize * 1);
        std::ptr::copy_nonoverlapping((*src).flex_bvhadr as *const u8, (*dest).flex_bvhadr as *mut u8, 4 * (*src).nflex as usize * 1);
        std::ptr::copy_nonoverlapping((*src).flex_bvhnum as *const u8, (*dest).flex_bvhnum as *mut u8, 4 * (*src).nflex as usize * 1);
        std::ptr::copy_nonoverlapping((*src).flexedge_J_rownnz as *const u8, (*dest).flexedge_J_rownnz as *mut u8, 4 * (*src).nflexedge as usize * 1);
        std::ptr::copy_nonoverlapping((*src).flexedge_J_rowadr as *const u8, (*dest).flexedge_J_rowadr as *mut u8, 4 * (*src).nflexedge as usize * 1);
        std::ptr::copy_nonoverlapping((*src).flexedge_J_colind as *const u8, (*dest).flexedge_J_colind as *mut u8, 4 * (*src).nJfe as usize * 1);
        std::ptr::copy_nonoverlapping((*src).flexvert_J_rownnz as *const u8, (*dest).flexvert_J_rownnz as *mut u8, 4 * (*src).nflexvert as usize * 2);
        std::ptr::copy_nonoverlapping((*src).flexvert_J_rowadr as *const u8, (*dest).flexvert_J_rowadr as *mut u8, 4 * (*src).nflexvert as usize * 2);
        std::ptr::copy_nonoverlapping((*src).flexvert_J_colind as *const u8, (*dest).flexvert_J_colind as *mut u8, 4 * (*src).nJfv as usize * 2);
        std::ptr::copy_nonoverlapping((*src).flex_rgba as *const u8, (*dest).flex_rgba as *mut u8, 4 * (*src).nflex as usize * 4);
        std::ptr::copy_nonoverlapping((*src).flex_texcoord as *const u8, (*dest).flex_texcoord as *mut u8, 4 * (*src).nflextexcoord as usize * 2);
        std::ptr::copy_nonoverlapping((*src).mesh_vertadr as *const u8, (*dest).mesh_vertadr as *mut u8, 4 * (*src).nmesh as usize * 1);
        std::ptr::copy_nonoverlapping((*src).mesh_vertnum as *const u8, (*dest).mesh_vertnum as *mut u8, 4 * (*src).nmesh as usize * 1);
        std::ptr::copy_nonoverlapping((*src).mesh_faceadr as *const u8, (*dest).mesh_faceadr as *mut u8, 4 * (*src).nmesh as usize * 1);
        std::ptr::copy_nonoverlapping((*src).mesh_facenum as *const u8, (*dest).mesh_facenum as *mut u8, 4 * (*src).nmesh as usize * 1);
        std::ptr::copy_nonoverlapping((*src).mesh_bvhadr as *const u8, (*dest).mesh_bvhadr as *mut u8, 4 * (*src).nmesh as usize * 1);
        std::ptr::copy_nonoverlapping((*src).mesh_bvhnum as *const u8, (*dest).mesh_bvhnum as *mut u8, 4 * (*src).nmesh as usize * 1);
        std::ptr::copy_nonoverlapping((*src).mesh_octadr as *const u8, (*dest).mesh_octadr as *mut u8, 4 * (*src).nmesh as usize * 1);
        std::ptr::copy_nonoverlapping((*src).mesh_octnum as *const u8, (*dest).mesh_octnum as *mut u8, 4 * (*src).nmesh as usize * 1);
        std::ptr::copy_nonoverlapping((*src).mesh_normaladr as *const u8, (*dest).mesh_normaladr as *mut u8, 4 * (*src).nmesh as usize * 1);
        std::ptr::copy_nonoverlapping((*src).mesh_normalnum as *const u8, (*dest).mesh_normalnum as *mut u8, 4 * (*src).nmesh as usize * 1);
        std::ptr::copy_nonoverlapping((*src).mesh_texcoordadr as *const u8, (*dest).mesh_texcoordadr as *mut u8, 4 * (*src).nmesh as usize * 1);
        std::ptr::copy_nonoverlapping((*src).mesh_texcoordnum as *const u8, (*dest).mesh_texcoordnum as *mut u8, 4 * (*src).nmesh as usize * 1);
        std::ptr::copy_nonoverlapping((*src).mesh_graphadr as *const u8, (*dest).mesh_graphadr as *mut u8, 4 * (*src).nmesh as usize * 1);
        std::ptr::copy_nonoverlapping((*src).mesh_vert as *const u8, (*dest).mesh_vert as *mut u8, 4 * (*src).nmeshvert as usize * 3);
        std::ptr::copy_nonoverlapping((*src).mesh_normal as *const u8, (*dest).mesh_normal as *mut u8, 4 * (*src).nmeshnormal as usize * 3);
        std::ptr::copy_nonoverlapping((*src).mesh_texcoord as *const u8, (*dest).mesh_texcoord as *mut u8, 4 * (*src).nmeshtexcoord as usize * 2);
        std::ptr::copy_nonoverlapping((*src).mesh_face as *const u8, (*dest).mesh_face as *mut u8, 4 * (*src).nmeshface as usize * 3);
        std::ptr::copy_nonoverlapping((*src).mesh_facenormal as *const u8, (*dest).mesh_facenormal as *mut u8, 4 * (*src).nmeshface as usize * 3);
        std::ptr::copy_nonoverlapping((*src).mesh_facetexcoord as *const u8, (*dest).mesh_facetexcoord as *mut u8, 4 * (*src).nmeshface as usize * 3);
        std::ptr::copy_nonoverlapping((*src).mesh_graph as *const u8, (*dest).mesh_graph as *mut u8, 4 * (*src).nmeshgraph as usize * 1);
        std::ptr::copy_nonoverlapping((*src).mesh_scale as *const u8, (*dest).mesh_scale as *mut u8, 8 * (*src).nmesh as usize * 3);
        std::ptr::copy_nonoverlapping((*src).mesh_pos as *const u8, (*dest).mesh_pos as *mut u8, 8 * (*src).nmesh as usize * 3);
        std::ptr::copy_nonoverlapping((*src).mesh_quat as *const u8, (*dest).mesh_quat as *mut u8, 8 * (*src).nmesh as usize * 4);
        std::ptr::copy_nonoverlapping((*src).mesh_pathadr as *const u8, (*dest).mesh_pathadr as *mut u8, 4 * (*src).nmesh as usize * 1);
        std::ptr::copy_nonoverlapping((*src).mesh_polynum as *const u8, (*dest).mesh_polynum as *mut u8, 4 * (*src).nmesh as usize * 1);
        std::ptr::copy_nonoverlapping((*src).mesh_polyadr as *const u8, (*dest).mesh_polyadr as *mut u8, 4 * (*src).nmesh as usize * 1);
        std::ptr::copy_nonoverlapping((*src).mesh_polynormal as *const u8, (*dest).mesh_polynormal as *mut u8, 8 * (*src).nmeshpoly as usize * 3);
        std::ptr::copy_nonoverlapping((*src).mesh_polyvertadr as *const u8, (*dest).mesh_polyvertadr as *mut u8, 4 * (*src).nmeshpoly as usize * 1);
        std::ptr::copy_nonoverlapping((*src).mesh_polyvertnum as *const u8, (*dest).mesh_polyvertnum as *mut u8, 4 * (*src).nmeshpoly as usize * 1);
        std::ptr::copy_nonoverlapping((*src).mesh_polyvert as *const u8, (*dest).mesh_polyvert as *mut u8, 4 * (*src).nmeshpolyvert as usize * 1);
        std::ptr::copy_nonoverlapping((*src).mesh_polymapadr as *const u8, (*dest).mesh_polymapadr as *mut u8, 4 * (*src).nmeshvert as usize * 1);
        std::ptr::copy_nonoverlapping((*src).mesh_polymapnum as *const u8, (*dest).mesh_polymapnum as *mut u8, 4 * (*src).nmeshvert as usize * 1);
        std::ptr::copy_nonoverlapping((*src).mesh_polymap as *const u8, (*dest).mesh_polymap as *mut u8, 4 * (*src).nmeshpolymap as usize * 1);
        std::ptr::copy_nonoverlapping((*src).skin_matid as *const u8, (*dest).skin_matid as *mut u8, 4 * (*src).nskin as usize * 1);
        std::ptr::copy_nonoverlapping((*src).skin_group as *const u8, (*dest).skin_group as *mut u8, 4 * (*src).nskin as usize * 1);
        std::ptr::copy_nonoverlapping((*src).skin_rgba as *const u8, (*dest).skin_rgba as *mut u8, 4 * (*src).nskin as usize * 4);
        std::ptr::copy_nonoverlapping((*src).skin_inflate as *const u8, (*dest).skin_inflate as *mut u8, 4 * (*src).nskin as usize * 1);
        std::ptr::copy_nonoverlapping((*src).skin_vertadr as *const u8, (*dest).skin_vertadr as *mut u8, 4 * (*src).nskin as usize * 1);
        std::ptr::copy_nonoverlapping((*src).skin_vertnum as *const u8, (*dest).skin_vertnum as *mut u8, 4 * (*src).nskin as usize * 1);
        std::ptr::copy_nonoverlapping((*src).skin_texcoordadr as *const u8, (*dest).skin_texcoordadr as *mut u8, 4 * (*src).nskin as usize * 1);
        std::ptr::copy_nonoverlapping((*src).skin_faceadr as *const u8, (*dest).skin_faceadr as *mut u8, 4 * (*src).nskin as usize * 1);
        std::ptr::copy_nonoverlapping((*src).skin_facenum as *const u8, (*dest).skin_facenum as *mut u8, 4 * (*src).nskin as usize * 1);
        std::ptr::copy_nonoverlapping((*src).skin_boneadr as *const u8, (*dest).skin_boneadr as *mut u8, 4 * (*src).nskin as usize * 1);
        std::ptr::copy_nonoverlapping((*src).skin_bonenum as *const u8, (*dest).skin_bonenum as *mut u8, 4 * (*src).nskin as usize * 1);
        std::ptr::copy_nonoverlapping((*src).skin_vert as *const u8, (*dest).skin_vert as *mut u8, 4 * (*src).nskinvert as usize * 3);
        std::ptr::copy_nonoverlapping((*src).skin_texcoord as *const u8, (*dest).skin_texcoord as *mut u8, 4 * (*src).nskintexvert as usize * 2);
        std::ptr::copy_nonoverlapping((*src).skin_face as *const u8, (*dest).skin_face as *mut u8, 4 * (*src).nskinface as usize * 3);
        std::ptr::copy_nonoverlapping((*src).skin_bonevertadr as *const u8, (*dest).skin_bonevertadr as *mut u8, 4 * (*src).nskinbone as usize * 1);
        std::ptr::copy_nonoverlapping((*src).skin_bonevertnum as *const u8, (*dest).skin_bonevertnum as *mut u8, 4 * (*src).nskinbone as usize * 1);
        std::ptr::copy_nonoverlapping((*src).skin_bonebindpos as *const u8, (*dest).skin_bonebindpos as *mut u8, 4 * (*src).nskinbone as usize * 3);
        std::ptr::copy_nonoverlapping((*src).skin_bonebindquat as *const u8, (*dest).skin_bonebindquat as *mut u8, 4 * (*src).nskinbone as usize * 4);
        std::ptr::copy_nonoverlapping((*src).skin_bonebodyid as *const u8, (*dest).skin_bonebodyid as *mut u8, 4 * (*src).nskinbone as usize * 1);
        std::ptr::copy_nonoverlapping((*src).skin_bonevertid as *const u8, (*dest).skin_bonevertid as *mut u8, 4 * (*src).nskinbonevert as usize * 1);
        std::ptr::copy_nonoverlapping((*src).skin_bonevertweight as *const u8, (*dest).skin_bonevertweight as *mut u8, 4 * (*src).nskinbonevert as usize * 1);
        std::ptr::copy_nonoverlapping((*src).skin_pathadr as *const u8, (*dest).skin_pathadr as *mut u8, 4 * (*src).nskin as usize * 1);
        std::ptr::copy_nonoverlapping((*src).hfield_size as *const u8, (*dest).hfield_size as *mut u8, 8 * (*src).nhfield as usize * 4);
        std::ptr::copy_nonoverlapping((*src).hfield_nrow as *const u8, (*dest).hfield_nrow as *mut u8, 4 * (*src).nhfield as usize * 1);
        std::ptr::copy_nonoverlapping((*src).hfield_ncol as *const u8, (*dest).hfield_ncol as *mut u8, 4 * (*src).nhfield as usize * 1);
        std::ptr::copy_nonoverlapping((*src).hfield_adr as *const u8, (*dest).hfield_adr as *mut u8, 4 * (*src).nhfield as usize * 1);
        std::ptr::copy_nonoverlapping((*src).hfield_data as *const u8, (*dest).hfield_data as *mut u8, 4 * (*src).nhfielddata as usize * 1);
        std::ptr::copy_nonoverlapping((*src).hfield_pathadr as *const u8, (*dest).hfield_pathadr as *mut u8, 4 * (*src).nhfield as usize * 1);
        std::ptr::copy_nonoverlapping((*src).tex_type as *const u8, (*dest).tex_type as *mut u8, 4 * (*src).ntex as usize * 1);
        std::ptr::copy_nonoverlapping((*src).tex_colorspace as *const u8, (*dest).tex_colorspace as *mut u8, 4 * (*src).ntex as usize * 1);
        std::ptr::copy_nonoverlapping((*src).tex_height as *const u8, (*dest).tex_height as *mut u8, 4 * (*src).ntex as usize * 1);
        std::ptr::copy_nonoverlapping((*src).tex_width as *const u8, (*dest).tex_width as *mut u8, 4 * (*src).ntex as usize * 1);
        std::ptr::copy_nonoverlapping((*src).tex_nchannel as *const u8, (*dest).tex_nchannel as *mut u8, 4 * (*src).ntex as usize * 1);
        std::ptr::copy_nonoverlapping((*src).tex_adr as *const u8, (*dest).tex_adr as *mut u8, 8 * (*src).ntex as usize * 1);
        std::ptr::copy_nonoverlapping((*src).tex_data as *const u8, (*dest).tex_data as *mut u8, 1 * (*src).ntexdata as usize * 1);
        std::ptr::copy_nonoverlapping((*src).tex_pathadr as *const u8, (*dest).tex_pathadr as *mut u8, 4 * (*src).ntex as usize * 1);
        std::ptr::copy_nonoverlapping((*src).mat_texid as *const u8, (*dest).mat_texid as *mut u8, 4 * (*src).nmat as usize * 10);
        std::ptr::copy_nonoverlapping((*src).mat_texuniform as *const u8, (*dest).mat_texuniform as *mut u8, 1 * (*src).nmat as usize * 1);
        std::ptr::copy_nonoverlapping((*src).mat_texrepeat as *const u8, (*dest).mat_texrepeat as *mut u8, 4 * (*src).nmat as usize * 2);
        std::ptr::copy_nonoverlapping((*src).mat_emission as *const u8, (*dest).mat_emission as *mut u8, 4 * (*src).nmat as usize * 1);
        std::ptr::copy_nonoverlapping((*src).mat_specular as *const u8, (*dest).mat_specular as *mut u8, 4 * (*src).nmat as usize * 1);
        std::ptr::copy_nonoverlapping((*src).mat_shininess as *const u8, (*dest).mat_shininess as *mut u8, 4 * (*src).nmat as usize * 1);
        std::ptr::copy_nonoverlapping((*src).mat_reflectance as *const u8, (*dest).mat_reflectance as *mut u8, 4 * (*src).nmat as usize * 1);
        std::ptr::copy_nonoverlapping((*src).mat_metallic as *const u8, (*dest).mat_metallic as *mut u8, 4 * (*src).nmat as usize * 1);
        std::ptr::copy_nonoverlapping((*src).mat_roughness as *const u8, (*dest).mat_roughness as *mut u8, 4 * (*src).nmat as usize * 1);
        std::ptr::copy_nonoverlapping((*src).mat_rgba as *const u8, (*dest).mat_rgba as *mut u8, 4 * (*src).nmat as usize * 4);
        std::ptr::copy_nonoverlapping((*src).pair_dim as *const u8, (*dest).pair_dim as *mut u8, 4 * (*src).npair as usize * 1);
        std::ptr::copy_nonoverlapping((*src).pair_geom1 as *const u8, (*dest).pair_geom1 as *mut u8, 4 * (*src).npair as usize * 1);
        std::ptr::copy_nonoverlapping((*src).pair_geom2 as *const u8, (*dest).pair_geom2 as *mut u8, 4 * (*src).npair as usize * 1);
        std::ptr::copy_nonoverlapping((*src).pair_signature as *const u8, (*dest).pair_signature as *mut u8, 4 * (*src).npair as usize * 1);
        std::ptr::copy_nonoverlapping((*src).pair_solref as *const u8, (*dest).pair_solref as *mut u8, 8 * (*src).npair as usize * 2);
        std::ptr::copy_nonoverlapping((*src).pair_solreffriction as *const u8, (*dest).pair_solreffriction as *mut u8, 8 * (*src).npair as usize * 2);
        std::ptr::copy_nonoverlapping((*src).pair_solimp as *const u8, (*dest).pair_solimp as *mut u8, 8 * (*src).npair as usize * 5);
        std::ptr::copy_nonoverlapping((*src).pair_margin as *const u8, (*dest).pair_margin as *mut u8, 8 * (*src).npair as usize * 1);
        std::ptr::copy_nonoverlapping((*src).pair_gap as *const u8, (*dest).pair_gap as *mut u8, 8 * (*src).npair as usize * 1);
        std::ptr::copy_nonoverlapping((*src).pair_friction as *const u8, (*dest).pair_friction as *mut u8, 8 * (*src).npair as usize * 5);
        std::ptr::copy_nonoverlapping((*src).exclude_signature as *const u8, (*dest).exclude_signature as *mut u8, 4 * (*src).nexclude as usize * 1);
        std::ptr::copy_nonoverlapping((*src).eq_type as *const u8, (*dest).eq_type as *mut u8, 4 * (*src).neq as usize * 1);
        std::ptr::copy_nonoverlapping((*src).eq_obj1id as *const u8, (*dest).eq_obj1id as *mut u8, 4 * (*src).neq as usize * 1);
        std::ptr::copy_nonoverlapping((*src).eq_obj2id as *const u8, (*dest).eq_obj2id as *mut u8, 4 * (*src).neq as usize * 1);
        std::ptr::copy_nonoverlapping((*src).eq_objtype as *const u8, (*dest).eq_objtype as *mut u8, 4 * (*src).neq as usize * 1);
        std::ptr::copy_nonoverlapping((*src).eq_active0 as *const u8, (*dest).eq_active0 as *mut u8, 1 * (*src).neq as usize * 1);
        std::ptr::copy_nonoverlapping((*src).eq_solref as *const u8, (*dest).eq_solref as *mut u8, 8 * (*src).neq as usize * 2);
        std::ptr::copy_nonoverlapping((*src).eq_solimp as *const u8, (*dest).eq_solimp as *mut u8, 8 * (*src).neq as usize * 5);
        std::ptr::copy_nonoverlapping((*src).eq_data as *const u8, (*dest).eq_data as *mut u8, 8 * (*src).neq as usize * 11);
        std::ptr::copy_nonoverlapping((*src).tendon_adr as *const u8, (*dest).tendon_adr as *mut u8, 4 * (*src).ntendon as usize * 1);
        std::ptr::copy_nonoverlapping((*src).tendon_num as *const u8, (*dest).tendon_num as *mut u8, 4 * (*src).ntendon as usize * 1);
        std::ptr::copy_nonoverlapping((*src).tendon_matid as *const u8, (*dest).tendon_matid as *mut u8, 4 * (*src).ntendon as usize * 1);
        std::ptr::copy_nonoverlapping((*src).tendon_actuatorid as *const u8, (*dest).tendon_actuatorid as *mut u8, 4 * (*src).ntendon as usize * 1);
        std::ptr::copy_nonoverlapping((*src).tendon_group as *const u8, (*dest).tendon_group as *mut u8, 4 * (*src).ntendon as usize * 1);
        std::ptr::copy_nonoverlapping((*src).tendon_treenum as *const u8, (*dest).tendon_treenum as *mut u8, 4 * (*src).ntendon as usize * 1);
        std::ptr::copy_nonoverlapping((*src).tendon_treeid as *const u8, (*dest).tendon_treeid as *mut u8, 4 * (*src).ntendon as usize * 2);
        std::ptr::copy_nonoverlapping((*src).ten_J_rownnz as *const u8, (*dest).ten_J_rownnz as *mut u8, 4 * (*src).ntendon as usize * 1);
        std::ptr::copy_nonoverlapping((*src).ten_J_rowadr as *const u8, (*dest).ten_J_rowadr as *mut u8, 4 * (*src).ntendon as usize * 1);
        std::ptr::copy_nonoverlapping((*src).ten_J_colind as *const u8, (*dest).ten_J_colind as *mut u8, 4 * (*src).nJten as usize * 1);
        std::ptr::copy_nonoverlapping((*src).tendon_limited as *const u8, (*dest).tendon_limited as *mut u8, 1 * (*src).ntendon as usize * 1);
        std::ptr::copy_nonoverlapping((*src).tendon_actfrclimited as *const u8, (*dest).tendon_actfrclimited as *mut u8, 1 * (*src).ntendon as usize * 1);
        std::ptr::copy_nonoverlapping((*src).tendon_width as *const u8, (*dest).tendon_width as *mut u8, 8 * (*src).ntendon as usize * 1);
        std::ptr::copy_nonoverlapping((*src).tendon_solref_lim as *const u8, (*dest).tendon_solref_lim as *mut u8, 8 * (*src).ntendon as usize * 2);
        std::ptr::copy_nonoverlapping((*src).tendon_solimp_lim as *const u8, (*dest).tendon_solimp_lim as *mut u8, 8 * (*src).ntendon as usize * 5);
        std::ptr::copy_nonoverlapping((*src).tendon_solref_fri as *const u8, (*dest).tendon_solref_fri as *mut u8, 8 * (*src).ntendon as usize * 2);
        std::ptr::copy_nonoverlapping((*src).tendon_solimp_fri as *const u8, (*dest).tendon_solimp_fri as *mut u8, 8 * (*src).ntendon as usize * 5);
        std::ptr::copy_nonoverlapping((*src).tendon_range as *const u8, (*dest).tendon_range as *mut u8, 8 * (*src).ntendon as usize * 2);
        std::ptr::copy_nonoverlapping((*src).tendon_actfrcrange as *const u8, (*dest).tendon_actfrcrange as *mut u8, 8 * (*src).ntendon as usize * 2);
        std::ptr::copy_nonoverlapping((*src).tendon_margin as *const u8, (*dest).tendon_margin as *mut u8, 8 * (*src).ntendon as usize * 1);
        std::ptr::copy_nonoverlapping((*src).tendon_stiffness as *const u8, (*dest).tendon_stiffness as *mut u8, 8 * (*src).ntendon as usize * 1);
        std::ptr::copy_nonoverlapping((*src).tendon_stiffnesspoly as *const u8, (*dest).tendon_stiffnesspoly as *mut u8, 8 * (*src).ntendon as usize * 2);
        std::ptr::copy_nonoverlapping((*src).tendon_damping as *const u8, (*dest).tendon_damping as *mut u8, 8 * (*src).ntendon as usize * 1);
        std::ptr::copy_nonoverlapping((*src).tendon_dampingpoly as *const u8, (*dest).tendon_dampingpoly as *mut u8, 8 * (*src).ntendon as usize * 2);
        std::ptr::copy_nonoverlapping((*src).tendon_armature as *const u8, (*dest).tendon_armature as *mut u8, 8 * (*src).ntendon as usize * 1);
        std::ptr::copy_nonoverlapping((*src).tendon_frictionloss as *const u8, (*dest).tendon_frictionloss as *mut u8, 8 * (*src).ntendon as usize * 1);
        std::ptr::copy_nonoverlapping((*src).tendon_lengthspring as *const u8, (*dest).tendon_lengthspring as *mut u8, 8 * (*src).ntendon as usize * 2);
        std::ptr::copy_nonoverlapping((*src).tendon_length0 as *const u8, (*dest).tendon_length0 as *mut u8, 8 * (*src).ntendon as usize * 1);
        std::ptr::copy_nonoverlapping((*src).tendon_invweight0 as *const u8, (*dest).tendon_invweight0 as *mut u8, 8 * (*src).ntendon as usize * 1);
        std::ptr::copy_nonoverlapping((*src).tendon_user as *const u8, (*dest).tendon_user as *mut u8, 8 * (*src).ntendon as usize * (*src).nuser_tendon as usize);
        std::ptr::copy_nonoverlapping((*src).tendon_rgba as *const u8, (*dest).tendon_rgba as *mut u8, 4 * (*src).ntendon as usize * 4);
        std::ptr::copy_nonoverlapping((*src).wrap_type as *const u8, (*dest).wrap_type as *mut u8, 4 * (*src).nwrap as usize * 1);
        std::ptr::copy_nonoverlapping((*src).wrap_objid as *const u8, (*dest).wrap_objid as *mut u8, 4 * (*src).nwrap as usize * 1);
        std::ptr::copy_nonoverlapping((*src).wrap_prm as *const u8, (*dest).wrap_prm as *mut u8, 8 * (*src).nwrap as usize * 1);
        std::ptr::copy_nonoverlapping((*src).actuator_trntype as *const u8, (*dest).actuator_trntype as *mut u8, 4 * (*src).nu as usize * 1);
        std::ptr::copy_nonoverlapping((*src).actuator_dyntype as *const u8, (*dest).actuator_dyntype as *mut u8, 4 * (*src).nu as usize * 1);
        std::ptr::copy_nonoverlapping((*src).actuator_gaintype as *const u8, (*dest).actuator_gaintype as *mut u8, 4 * (*src).nu as usize * 1);
        std::ptr::copy_nonoverlapping((*src).actuator_biastype as *const u8, (*dest).actuator_biastype as *mut u8, 4 * (*src).nu as usize * 1);
        std::ptr::copy_nonoverlapping((*src).actuator_trnid as *const u8, (*dest).actuator_trnid as *mut u8, 4 * (*src).nu as usize * 2);
        std::ptr::copy_nonoverlapping((*src).actuator_damping as *const u8, (*dest).actuator_damping as *mut u8, 8 * (*src).nu as usize * 1);
        std::ptr::copy_nonoverlapping((*src).actuator_dampingpoly as *const u8, (*dest).actuator_dampingpoly as *mut u8, 8 * (*src).nu as usize * 2);
        std::ptr::copy_nonoverlapping((*src).actuator_armature as *const u8, (*dest).actuator_armature as *mut u8, 8 * (*src).nu as usize * 1);
        std::ptr::copy_nonoverlapping((*src).actuator_actadr as *const u8, (*dest).actuator_actadr as *mut u8, 4 * (*src).nu as usize * 1);
        std::ptr::copy_nonoverlapping((*src).actuator_actnum as *const u8, (*dest).actuator_actnum as *mut u8, 4 * (*src).nu as usize * 1);
        std::ptr::copy_nonoverlapping((*src).actuator_group as *const u8, (*dest).actuator_group as *mut u8, 4 * (*src).nu as usize * 1);
        std::ptr::copy_nonoverlapping((*src).actuator_history as *const u8, (*dest).actuator_history as *mut u8, 4 * (*src).nu as usize * 2);
        std::ptr::copy_nonoverlapping((*src).actuator_historyadr as *const u8, (*dest).actuator_historyadr as *mut u8, 4 * (*src).nu as usize * 1);
        std::ptr::copy_nonoverlapping((*src).actuator_delay as *const u8, (*dest).actuator_delay as *mut u8, 8 * (*src).nu as usize * 1);
        std::ptr::copy_nonoverlapping((*src).actuator_ctrllimited as *const u8, (*dest).actuator_ctrllimited as *mut u8, 1 * (*src).nu as usize * 1);
        std::ptr::copy_nonoverlapping((*src).actuator_forcelimited as *const u8, (*dest).actuator_forcelimited as *mut u8, 1 * (*src).nu as usize * 1);
        std::ptr::copy_nonoverlapping((*src).actuator_actlimited as *const u8, (*dest).actuator_actlimited as *mut u8, 1 * (*src).nu as usize * 1);
        std::ptr::copy_nonoverlapping((*src).actuator_dynprm as *const u8, (*dest).actuator_dynprm as *mut u8, 8 * (*src).nu as usize * 10);
        std::ptr::copy_nonoverlapping((*src).actuator_gainprm as *const u8, (*dest).actuator_gainprm as *mut u8, 8 * (*src).nu as usize * 10);
        std::ptr::copy_nonoverlapping((*src).actuator_biasprm as *const u8, (*dest).actuator_biasprm as *mut u8, 8 * (*src).nu as usize * 10);
        std::ptr::copy_nonoverlapping((*src).actuator_actearly as *const u8, (*dest).actuator_actearly as *mut u8, 1 * (*src).nu as usize * 1);
        std::ptr::copy_nonoverlapping((*src).actuator_ctrlrange as *const u8, (*dest).actuator_ctrlrange as *mut u8, 8 * (*src).nu as usize * 2);
        std::ptr::copy_nonoverlapping((*src).actuator_forcerange as *const u8, (*dest).actuator_forcerange as *mut u8, 8 * (*src).nu as usize * 2);
        std::ptr::copy_nonoverlapping((*src).actuator_actrange as *const u8, (*dest).actuator_actrange as *mut u8, 8 * (*src).nu as usize * 2);
        std::ptr::copy_nonoverlapping((*src).actuator_gear as *const u8, (*dest).actuator_gear as *mut u8, 8 * (*src).nu as usize * 6);
        std::ptr::copy_nonoverlapping((*src).actuator_cranklength as *const u8, (*dest).actuator_cranklength as *mut u8, 8 * (*src).nu as usize * 1);
        std::ptr::copy_nonoverlapping((*src).actuator_acc0 as *const u8, (*dest).actuator_acc0 as *mut u8, 8 * (*src).nu as usize * 1);
        std::ptr::copy_nonoverlapping((*src).actuator_length0 as *const u8, (*dest).actuator_length0 as *mut u8, 8 * (*src).nu as usize * 1);
        std::ptr::copy_nonoverlapping((*src).actuator_lengthrange as *const u8, (*dest).actuator_lengthrange as *mut u8, 8 * (*src).nu as usize * 2);
        std::ptr::copy_nonoverlapping((*src).actuator_user as *const u8, (*dest).actuator_user as *mut u8, 8 * (*src).nu as usize * (*src).nuser_actuator as usize);
        std::ptr::copy_nonoverlapping((*src).actuator_plugin as *const u8, (*dest).actuator_plugin as *mut u8, 4 * (*src).nu as usize * 1);
        std::ptr::copy_nonoverlapping((*src).sensor_type as *const u8, (*dest).sensor_type as *mut u8, 4 * (*src).nsensor as usize * 1);
        std::ptr::copy_nonoverlapping((*src).sensor_datatype as *const u8, (*dest).sensor_datatype as *mut u8, 4 * (*src).nsensor as usize * 1);
        std::ptr::copy_nonoverlapping((*src).sensor_needstage as *const u8, (*dest).sensor_needstage as *mut u8, 4 * (*src).nsensor as usize * 1);
        std::ptr::copy_nonoverlapping((*src).sensor_objtype as *const u8, (*dest).sensor_objtype as *mut u8, 4 * (*src).nsensor as usize * 1);
        std::ptr::copy_nonoverlapping((*src).sensor_objid as *const u8, (*dest).sensor_objid as *mut u8, 4 * (*src).nsensor as usize * 1);
        std::ptr::copy_nonoverlapping((*src).sensor_reftype as *const u8, (*dest).sensor_reftype as *mut u8, 4 * (*src).nsensor as usize * 1);
        std::ptr::copy_nonoverlapping((*src).sensor_refid as *const u8, (*dest).sensor_refid as *mut u8, 4 * (*src).nsensor as usize * 1);
        std::ptr::copy_nonoverlapping((*src).sensor_intprm as *const u8, (*dest).sensor_intprm as *mut u8, 4 * (*src).nsensor as usize * 3);
        std::ptr::copy_nonoverlapping((*src).sensor_dim as *const u8, (*dest).sensor_dim as *mut u8, 4 * (*src).nsensor as usize * 1);
        std::ptr::copy_nonoverlapping((*src).sensor_adr as *const u8, (*dest).sensor_adr as *mut u8, 4 * (*src).nsensor as usize * 1);
        std::ptr::copy_nonoverlapping((*src).sensor_cutoff as *const u8, (*dest).sensor_cutoff as *mut u8, 8 * (*src).nsensor as usize * 1);
        std::ptr::copy_nonoverlapping((*src).sensor_noise as *const u8, (*dest).sensor_noise as *mut u8, 8 * (*src).nsensor as usize * 1);
        std::ptr::copy_nonoverlapping((*src).sensor_history as *const u8, (*dest).sensor_history as *mut u8, 4 * (*src).nsensor as usize * 2);
        std::ptr::copy_nonoverlapping((*src).sensor_historyadr as *const u8, (*dest).sensor_historyadr as *mut u8, 4 * (*src).nsensor as usize * 1);
        std::ptr::copy_nonoverlapping((*src).sensor_delay as *const u8, (*dest).sensor_delay as *mut u8, 8 * (*src).nsensor as usize * 1);
        std::ptr::copy_nonoverlapping((*src).sensor_interval as *const u8, (*dest).sensor_interval as *mut u8, 8 * (*src).nsensor as usize * 2);
        std::ptr::copy_nonoverlapping((*src).sensor_user as *const u8, (*dest).sensor_user as *mut u8, 8 * (*src).nsensor as usize * (*src).nuser_sensor as usize);
        std::ptr::copy_nonoverlapping((*src).sensor_plugin as *const u8, (*dest).sensor_plugin as *mut u8, 4 * (*src).nsensor as usize * 1);
        std::ptr::copy_nonoverlapping((*src).plugin as *const u8, (*dest).plugin as *mut u8, 4 * (*src).nplugin as usize * 1);
        std::ptr::copy_nonoverlapping((*src).plugin_stateadr as *const u8, (*dest).plugin_stateadr as *mut u8, 4 * (*src).nplugin as usize * 1);
        std::ptr::copy_nonoverlapping((*src).plugin_statenum as *const u8, (*dest).plugin_statenum as *mut u8, 4 * (*src).nplugin as usize * 1);
        std::ptr::copy_nonoverlapping((*src).plugin_attr as *const u8, (*dest).plugin_attr as *mut u8, 1 * (*src).npluginattr as usize * 1);
        std::ptr::copy_nonoverlapping((*src).plugin_attradr as *const u8, (*dest).plugin_attradr as *mut u8, 4 * (*src).nplugin as usize * 1);
        std::ptr::copy_nonoverlapping((*src).numeric_adr as *const u8, (*dest).numeric_adr as *mut u8, 4 * (*src).nnumeric as usize * 1);
        std::ptr::copy_nonoverlapping((*src).numeric_size as *const u8, (*dest).numeric_size as *mut u8, 4 * (*src).nnumeric as usize * 1);
        std::ptr::copy_nonoverlapping((*src).numeric_data as *const u8, (*dest).numeric_data as *mut u8, 8 * (*src).nnumericdata as usize * 1);
        std::ptr::copy_nonoverlapping((*src).text_adr as *const u8, (*dest).text_adr as *mut u8, 4 * (*src).ntext as usize * 1);
        std::ptr::copy_nonoverlapping((*src).text_size as *const u8, (*dest).text_size as *mut u8, 4 * (*src).ntext as usize * 1);
        std::ptr::copy_nonoverlapping((*src).text_data as *const u8, (*dest).text_data as *mut u8, 1 * (*src).ntextdata as usize * 1);
        std::ptr::copy_nonoverlapping((*src).tuple_adr as *const u8, (*dest).tuple_adr as *mut u8, 4 * (*src).ntuple as usize * 1);
        std::ptr::copy_nonoverlapping((*src).tuple_size as *const u8, (*dest).tuple_size as *mut u8, 4 * (*src).ntuple as usize * 1);
        std::ptr::copy_nonoverlapping((*src).tuple_objtype as *const u8, (*dest).tuple_objtype as *mut u8, 4 * (*src).ntupledata as usize * 1);
        std::ptr::copy_nonoverlapping((*src).tuple_objid as *const u8, (*dest).tuple_objid as *mut u8, 4 * (*src).ntupledata as usize * 1);
        std::ptr::copy_nonoverlapping((*src).tuple_objprm as *const u8, (*dest).tuple_objprm as *mut u8, 8 * (*src).ntupledata as usize * 1);
        std::ptr::copy_nonoverlapping((*src).key_time as *const u8, (*dest).key_time as *mut u8, 8 * (*src).nkey as usize * 1);
        std::ptr::copy_nonoverlapping((*src).key_qpos as *const u8, (*dest).key_qpos as *mut u8, 8 * (*src).nkey as usize * (*src).nq as usize);
        std::ptr::copy_nonoverlapping((*src).key_qvel as *const u8, (*dest).key_qvel as *mut u8, 8 * (*src).nkey as usize * (*src).nv as usize);
        std::ptr::copy_nonoverlapping((*src).key_act as *const u8, (*dest).key_act as *mut u8, 8 * (*src).nkey as usize * (*src).na as usize);
        std::ptr::copy_nonoverlapping((*src).key_mpos as *const u8, (*dest).key_mpos as *mut u8, 8 * (*src).nkey as usize * ((*src).nmocap as usize * 3));
        std::ptr::copy_nonoverlapping((*src).key_mquat as *const u8, (*dest).key_mquat as *mut u8, 8 * (*src).nkey as usize * ((*src).nmocap as usize * 4));
        std::ptr::copy_nonoverlapping((*src).key_ctrl as *const u8, (*dest).key_ctrl as *mut u8, 8 * (*src).nkey as usize * (*src).nu as usize);
        std::ptr::copy_nonoverlapping((*src).name_bodyadr as *const u8, (*dest).name_bodyadr as *mut u8, 4 * (*src).nbody as usize * 1);
        std::ptr::copy_nonoverlapping((*src).name_jntadr as *const u8, (*dest).name_jntadr as *mut u8, 4 * (*src).njnt as usize * 1);
        std::ptr::copy_nonoverlapping((*src).name_geomadr as *const u8, (*dest).name_geomadr as *mut u8, 4 * (*src).ngeom as usize * 1);
        std::ptr::copy_nonoverlapping((*src).name_siteadr as *const u8, (*dest).name_siteadr as *mut u8, 4 * (*src).nsite as usize * 1);
        std::ptr::copy_nonoverlapping((*src).name_camadr as *const u8, (*dest).name_camadr as *mut u8, 4 * (*src).ncam as usize * 1);
        std::ptr::copy_nonoverlapping((*src).name_lightadr as *const u8, (*dest).name_lightadr as *mut u8, 4 * (*src).nlight as usize * 1);
        std::ptr::copy_nonoverlapping((*src).name_flexadr as *const u8, (*dest).name_flexadr as *mut u8, 4 * (*src).nflex as usize * 1);
        std::ptr::copy_nonoverlapping((*src).name_meshadr as *const u8, (*dest).name_meshadr as *mut u8, 4 * (*src).nmesh as usize * 1);
        std::ptr::copy_nonoverlapping((*src).name_skinadr as *const u8, (*dest).name_skinadr as *mut u8, 4 * (*src).nskin as usize * 1);
        std::ptr::copy_nonoverlapping((*src).name_hfieldadr as *const u8, (*dest).name_hfieldadr as *mut u8, 4 * (*src).nhfield as usize * 1);
        std::ptr::copy_nonoverlapping((*src).name_texadr as *const u8, (*dest).name_texadr as *mut u8, 4 * (*src).ntex as usize * 1);
        std::ptr::copy_nonoverlapping((*src).name_matadr as *const u8, (*dest).name_matadr as *mut u8, 4 * (*src).nmat as usize * 1);
        std::ptr::copy_nonoverlapping((*src).name_pairadr as *const u8, (*dest).name_pairadr as *mut u8, 4 * (*src).npair as usize * 1);
        std::ptr::copy_nonoverlapping((*src).name_excludeadr as *const u8, (*dest).name_excludeadr as *mut u8, 4 * (*src).nexclude as usize * 1);
        std::ptr::copy_nonoverlapping((*src).name_eqadr as *const u8, (*dest).name_eqadr as *mut u8, 4 * (*src).neq as usize * 1);
        std::ptr::copy_nonoverlapping((*src).name_tendonadr as *const u8, (*dest).name_tendonadr as *mut u8, 4 * (*src).ntendon as usize * 1);
        std::ptr::copy_nonoverlapping((*src).name_actuatoradr as *const u8, (*dest).name_actuatoradr as *mut u8, 4 * (*src).nu as usize * 1);
        std::ptr::copy_nonoverlapping((*src).name_sensoradr as *const u8, (*dest).name_sensoradr as *mut u8, 4 * (*src).nsensor as usize * 1);
        std::ptr::copy_nonoverlapping((*src).name_numericadr as *const u8, (*dest).name_numericadr as *mut u8, 4 * (*src).nnumeric as usize * 1);
        std::ptr::copy_nonoverlapping((*src).name_textadr as *const u8, (*dest).name_textadr as *mut u8, 4 * (*src).ntext as usize * 1);
        std::ptr::copy_nonoverlapping((*src).name_tupleadr as *const u8, (*dest).name_tupleadr as *mut u8, 4 * (*src).ntuple as usize * 1);
        std::ptr::copy_nonoverlapping((*src).name_keyadr as *const u8, (*dest).name_keyadr as *mut u8, 4 * (*src).nkey as usize * 1);
        std::ptr::copy_nonoverlapping((*src).name_pluginadr as *const u8, (*dest).name_pluginadr as *mut u8, 4 * (*src).nplugin as usize * 1);
        std::ptr::copy_nonoverlapping((*src).names as *const u8, (*dest).names as *mut u8, 1 * (*src).nnames as usize * 1);
        std::ptr::copy_nonoverlapping((*src).names_map as *const u8, (*dest).names_map as *mut u8, 4 * (*src).nnames_map as usize * 1);
        std::ptr::copy_nonoverlapping((*src).paths as *const u8, (*dest).paths as *mut u8, 1 * (*src).npaths as usize * 1);
        std::ptr::copy_nonoverlapping((*src).B_rownnz as *const u8, (*dest).B_rownnz as *mut u8, 4 * (*src).nbody as usize * 1);
        std::ptr::copy_nonoverlapping((*src).B_rowadr as *const u8, (*dest).B_rowadr as *mut u8, 4 * (*src).nbody as usize * 1);
        std::ptr::copy_nonoverlapping((*src).B_colind as *const u8, (*dest).B_colind as *mut u8, 4 * (*src).nB as usize * 1);
        std::ptr::copy_nonoverlapping((*src).M_rownnz as *const u8, (*dest).M_rownnz as *mut u8, 4 * (*src).nv as usize * 1);
        std::ptr::copy_nonoverlapping((*src).M_rowadr as *const u8, (*dest).M_rowadr as *mut u8, 4 * (*src).nv as usize * 1);
        std::ptr::copy_nonoverlapping((*src).M_colind as *const u8, (*dest).M_colind as *mut u8, 4 * (*src).nC as usize * 1);
        std::ptr::copy_nonoverlapping((*src).mapM2M as *const u8, (*dest).mapM2M as *mut u8, 4 * (*src).nC as usize * 1);
        std::ptr::copy_nonoverlapping((*src).D_rownnz as *const u8, (*dest).D_rownnz as *mut u8, 4 * (*src).nv as usize * 1);
        std::ptr::copy_nonoverlapping((*src).D_rowadr as *const u8, (*dest).D_rowadr as *mut u8, 4 * (*src).nv as usize * 1);
        std::ptr::copy_nonoverlapping((*src).D_diag as *const u8, (*dest).D_diag as *mut u8, 4 * (*src).nv as usize * 1);
        std::ptr::copy_nonoverlapping((*src).D_colind as *const u8, (*dest).D_colind as *mut u8, 4 * (*src).nD as usize * 1);
        std::ptr::copy_nonoverlapping((*src).mapM2D as *const u8, (*dest).mapM2D as *mut u8, 4 * (*src).nD as usize * 1);
        std::ptr::copy_nonoverlapping((*src).mapD2M as *const u8, (*dest).mapD2M as *mut u8, 4 * (*src).nC as usize * 1);
    }
}

/// C: mj_saveModel (engine/engine_io.h:75)
/// Calls: bufwrite, getnptr, getnsize, mj_version, mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn mj_save_model(m: *const mjModel, filename: *const i8, buffer: *mut (), buffer_sz: i32) {
    const NHEADER: usize = 5;
    const ID: i32 = 54321;

    extern "C" {
        fn fopen(filename: *const i8, mode: *const i8) -> *mut FILE;
        fn fclose(stream: *mut FILE) -> i32;
        fn fwrite(ptr: *const (), size: usize, count: usize, stream: *mut FILE) -> usize;
    }

    // SAFETY: m is valid; filename or buffer is valid (caller contract)
    unsafe {
        let mut fp: *mut FILE = std::ptr::null_mut();
        let mut ptrbuf: i64 = 0;

        // standard header
        let header: [i32; 5] = [
            ID,
            8,  // sizeof(mjtNum)
            getnsize(),
            crate::engine::engine_support::mj_version(),
            getnptr(),
        ];

        // open file for writing if no buffer
        if buffer.is_null() {
            fp = fopen(filename, b"wb\0".as_ptr() as *const i8);
            if fp.is_null() {
                crate::engine::engine_util_errmem::mju_warning(
                    b"Could not open file '%s'\0".as_ptr() as *const i8);
                return;
            }
        }

        // write standard header, info, options, buffer
        if !fp.is_null() {
            fwrite(header.as_ptr() as *const (), 4, NHEADER, fp);
            // MJMODEL_SIZES: write each size field
        fwrite(&(*m).nq as *const i64 as *const (), 8, 1, fp);
        fwrite(&(*m).nv as *const i64 as *const (), 8, 1, fp);
        fwrite(&(*m).nu as *const i64 as *const (), 8, 1, fp);
        fwrite(&(*m).na as *const i64 as *const (), 8, 1, fp);
        fwrite(&(*m).nbody as *const i64 as *const (), 8, 1, fp);
        fwrite(&(*m).nbvh as *const i64 as *const (), 8, 1, fp);
        fwrite(&(*m).nbvhstatic as *const i64 as *const (), 8, 1, fp);
        fwrite(&(*m).nbvhdynamic as *const i64 as *const (), 8, 1, fp);
        fwrite(&(*m).noct as *const i64 as *const (), 8, 1, fp);
        fwrite(&(*m).njnt as *const i64 as *const (), 8, 1, fp);
        fwrite(&(*m).ntree as *const i64 as *const (), 8, 1, fp);
        fwrite(&(*m).nM as *const i64 as *const (), 8, 1, fp);
        fwrite(&(*m).nB as *const i64 as *const (), 8, 1, fp);
        fwrite(&(*m).nC as *const i64 as *const (), 8, 1, fp);
        fwrite(&(*m).nD as *const i64 as *const (), 8, 1, fp);
        fwrite(&(*m).ngeom as *const i64 as *const (), 8, 1, fp);
        fwrite(&(*m).nsite as *const i64 as *const (), 8, 1, fp);
        fwrite(&(*m).ncam as *const i64 as *const (), 8, 1, fp);
        fwrite(&(*m).nlight as *const i64 as *const (), 8, 1, fp);
        fwrite(&(*m).nflex as *const i64 as *const (), 8, 1, fp);
        fwrite(&(*m).nflexnode as *const i64 as *const (), 8, 1, fp);
        fwrite(&(*m).nflexvert as *const i64 as *const (), 8, 1, fp);
        fwrite(&(*m).nflexedge as *const i64 as *const (), 8, 1, fp);
        fwrite(&(*m).nflexelem as *const i64 as *const (), 8, 1, fp);
        fwrite(&(*m).nflexelemdata as *const i64 as *const (), 8, 1, fp);
        fwrite(&(*m).nflexstiffness as *const i64 as *const (), 8, 1, fp);
        fwrite(&(*m).nflexbending as *const i64 as *const (), 8, 1, fp);
        fwrite(&(*m).nflexelemedge as *const i64 as *const (), 8, 1, fp);
        fwrite(&(*m).nflexshelldata as *const i64 as *const (), 8, 1, fp);
        fwrite(&(*m).nflexevpair as *const i64 as *const (), 8, 1, fp);
        fwrite(&(*m).nflextexcoord as *const i64 as *const (), 8, 1, fp);
        fwrite(&(*m).nJfe as *const i64 as *const (), 8, 1, fp);
        fwrite(&(*m).nJfv as *const i64 as *const (), 8, 1, fp);
        fwrite(&(*m).nmesh as *const i64 as *const (), 8, 1, fp);
        fwrite(&(*m).nmeshvert as *const i64 as *const (), 8, 1, fp);
        fwrite(&(*m).nmeshnormal as *const i64 as *const (), 8, 1, fp);
        fwrite(&(*m).nmeshtexcoord as *const i64 as *const (), 8, 1, fp);
        fwrite(&(*m).nmeshface as *const i64 as *const (), 8, 1, fp);
        fwrite(&(*m).nmeshgraph as *const i64 as *const (), 8, 1, fp);
        fwrite(&(*m).nmeshpoly as *const i64 as *const (), 8, 1, fp);
        fwrite(&(*m).nmeshpolyvert as *const i64 as *const (), 8, 1, fp);
        fwrite(&(*m).nmeshpolymap as *const i64 as *const (), 8, 1, fp);
        fwrite(&(*m).nskin as *const i64 as *const (), 8, 1, fp);
        fwrite(&(*m).nskinvert as *const i64 as *const (), 8, 1, fp);
        fwrite(&(*m).nskintexvert as *const i64 as *const (), 8, 1, fp);
        fwrite(&(*m).nskinface as *const i64 as *const (), 8, 1, fp);
        fwrite(&(*m).nskinbone as *const i64 as *const (), 8, 1, fp);
        fwrite(&(*m).nskinbonevert as *const i64 as *const (), 8, 1, fp);
        fwrite(&(*m).nhfield as *const i64 as *const (), 8, 1, fp);
        fwrite(&(*m).nhfielddata as *const i64 as *const (), 8, 1, fp);
        fwrite(&(*m).ntex as *const i64 as *const (), 8, 1, fp);
        fwrite(&(*m).ntexdata as *const i64 as *const (), 8, 1, fp);
        fwrite(&(*m).nmat as *const i64 as *const (), 8, 1, fp);
        fwrite(&(*m).npair as *const i64 as *const (), 8, 1, fp);
        fwrite(&(*m).nexclude as *const i64 as *const (), 8, 1, fp);
        fwrite(&(*m).neq as *const i64 as *const (), 8, 1, fp);
        fwrite(&(*m).ntendon as *const i64 as *const (), 8, 1, fp);
        fwrite(&(*m).nJten as *const i64 as *const (), 8, 1, fp);
        fwrite(&(*m).nwrap as *const i64 as *const (), 8, 1, fp);
        fwrite(&(*m).nsensor as *const i64 as *const (), 8, 1, fp);
        fwrite(&(*m).nnumeric as *const i64 as *const (), 8, 1, fp);
        fwrite(&(*m).nnumericdata as *const i64 as *const (), 8, 1, fp);
        fwrite(&(*m).ntext as *const i64 as *const (), 8, 1, fp);
        fwrite(&(*m).ntextdata as *const i64 as *const (), 8, 1, fp);
        fwrite(&(*m).ntuple as *const i64 as *const (), 8, 1, fp);
        fwrite(&(*m).ntupledata as *const i64 as *const (), 8, 1, fp);
        fwrite(&(*m).nkey as *const i64 as *const (), 8, 1, fp);
        fwrite(&(*m).nmocap as *const i64 as *const (), 8, 1, fp);
        fwrite(&(*m).nplugin as *const i64 as *const (), 8, 1, fp);
        fwrite(&(*m).npluginattr as *const i64 as *const (), 8, 1, fp);
        fwrite(&(*m).nuser_body as *const i64 as *const (), 8, 1, fp);
        fwrite(&(*m).nuser_jnt as *const i64 as *const (), 8, 1, fp);
        fwrite(&(*m).nuser_geom as *const i64 as *const (), 8, 1, fp);
        fwrite(&(*m).nuser_site as *const i64 as *const (), 8, 1, fp);
        fwrite(&(*m).nuser_cam as *const i64 as *const (), 8, 1, fp);
        fwrite(&(*m).nuser_tendon as *const i64 as *const (), 8, 1, fp);
        fwrite(&(*m).nuser_actuator as *const i64 as *const (), 8, 1, fp);
        fwrite(&(*m).nuser_sensor as *const i64 as *const (), 8, 1, fp);
        fwrite(&(*m).nnames as *const i64 as *const (), 8, 1, fp);
        fwrite(&(*m).npaths as *const i64 as *const (), 8, 1, fp);
        fwrite(&(*m).nnames_map as *const i64 as *const (), 8, 1, fp);
        fwrite(&(*m).nJmom as *const i64 as *const (), 8, 1, fp);
        fwrite(&(*m).ngravcomp as *const i64 as *const (), 8, 1, fp);
        fwrite(&(*m).nemax as *const i64 as *const (), 8, 1, fp);
        fwrite(&(*m).njmax as *const i64 as *const (), 8, 1, fp);
        fwrite(&(*m).nconmax as *const i64 as *const (), 8, 1, fp);
        fwrite(&(*m).nuserdata as *const i64 as *const (), 8, 1, fp);
        fwrite(&(*m).nsensordata as *const i64 as *const (), 8, 1, fp);
        fwrite(&(*m).npluginstate as *const i64 as *const (), 8, 1, fp);
        fwrite(&(*m).nhistory as *const i64 as *const (), 8, 1, fp);
        fwrite(&(*m).narena as *const i64 as *const (), 8, 1, fp);
        fwrite(&(*m).nbuffer as *const i64 as *const (), 8, 1, fp);
            fwrite(&(*m).opt as *const mjOption as *const (), 304, 1, fp);
            fwrite(&(*m).vis as *const mjVisual as *const (), 632, 1, fp);
            fwrite(&(*m).stat as *const mjStatistic as *const (), 56, 1, fp);
            // MJMODEL_POINTERS: write each pointer array
            fwrite((*m).qpos0 as *const (), 8, (((*m).nq) * (1_i64)) as usize, fp);
            fwrite((*m).qpos_spring as *const (), 8, (((*m).nq) * (1_i64)) as usize, fp);
            fwrite((*m).body_parentid as *const (), 4, (((*m).nbody) * (1_i64)) as usize, fp);
            fwrite((*m).body_rootid as *const (), 4, (((*m).nbody) * (1_i64)) as usize, fp);
            fwrite((*m).body_weldid as *const (), 4, (((*m).nbody) * (1_i64)) as usize, fp);
            fwrite((*m).body_mocapid as *const (), 4, (((*m).nbody) * (1_i64)) as usize, fp);
            fwrite((*m).body_jntnum as *const (), 4, (((*m).nbody) * (1_i64)) as usize, fp);
            fwrite((*m).body_jntadr as *const (), 4, (((*m).nbody) * (1_i64)) as usize, fp);
            fwrite((*m).body_dofnum as *const (), 4, (((*m).nbody) * (1_i64)) as usize, fp);
            fwrite((*m).body_dofadr as *const (), 4, (((*m).nbody) * (1_i64)) as usize, fp);
            fwrite((*m).body_treeid as *const (), 4, (((*m).nbody) * (1_i64)) as usize, fp);
            fwrite((*m).body_geomnum as *const (), 4, (((*m).nbody) * (1_i64)) as usize, fp);
            fwrite((*m).body_geomadr as *const (), 4, (((*m).nbody) * (1_i64)) as usize, fp);
            fwrite((*m).body_simple as *const (), 1, (((*m).nbody) * (1_i64)) as usize, fp);
            fwrite((*m).body_sameframe as *const (), 1, (((*m).nbody) * (1_i64)) as usize, fp);
            fwrite((*m).body_pos as *const (), 8, (((*m).nbody) * (3_i64)) as usize, fp);
            fwrite((*m).body_quat as *const (), 8, (((*m).nbody) * (4_i64)) as usize, fp);
            fwrite((*m).body_ipos as *const (), 8, (((*m).nbody) * (3_i64)) as usize, fp);
            fwrite((*m).body_iquat as *const (), 8, (((*m).nbody) * (4_i64)) as usize, fp);
            fwrite((*m).body_mass as *const (), 8, (((*m).nbody) * (1_i64)) as usize, fp);
            fwrite((*m).body_subtreemass as *const (), 8, (((*m).nbody) * (1_i64)) as usize, fp);
            fwrite((*m).body_inertia as *const (), 8, (((*m).nbody) * (3_i64)) as usize, fp);
            fwrite((*m).body_invweight0 as *const (), 8, (((*m).nbody) * (2_i64)) as usize, fp);
            fwrite((*m).body_gravcomp as *const (), 8, (((*m).nbody) * (1_i64)) as usize, fp);
            fwrite((*m).body_margin as *const (), 8, (((*m).nbody) * (1_i64)) as usize, fp);
            fwrite((*m).body_user as *const (), 8, (((*m).nbody) * ((*m).nuser_body)) as usize, fp);
            fwrite((*m).body_plugin as *const (), 4, (((*m).nbody) * (1_i64)) as usize, fp);
            fwrite((*m).body_contype as *const (), 4, (((*m).nbody) * (1_i64)) as usize, fp);
            fwrite((*m).body_conaffinity as *const (), 4, (((*m).nbody) * (1_i64)) as usize, fp);
            fwrite((*m).body_bvhadr as *const (), 4, (((*m).nbody) * (1_i64)) as usize, fp);
            fwrite((*m).body_bvhnum as *const (), 4, (((*m).nbody) * (1_i64)) as usize, fp);
            fwrite((*m).bvh_depth as *const (), 4, (((*m).nbvh) * (1_i64)) as usize, fp);
            fwrite((*m).bvh_child as *const (), 4, (((*m).nbvh) * (2_i64)) as usize, fp);
            fwrite((*m).bvh_nodeid as *const (), 4, (((*m).nbvh) * (1_i64)) as usize, fp);
            fwrite((*m).bvh_aabb as *const (), 8, (((*m).nbvhstatic) * (6_i64)) as usize, fp);
            fwrite((*m).oct_depth as *const (), 4, (((*m).noct) * (1_i64)) as usize, fp);
            fwrite((*m).oct_child as *const (), 4, (((*m).noct) * (8_i64)) as usize, fp);
            fwrite((*m).oct_aabb as *const (), 8, (((*m).noct) * (6_i64)) as usize, fp);
            fwrite((*m).oct_coeff as *const (), 8, (((*m).noct) * (8_i64)) as usize, fp);
            fwrite((*m).jnt_type as *const (), 4, (((*m).njnt) * (1_i64)) as usize, fp);
            fwrite((*m).jnt_qposadr as *const (), 4, (((*m).njnt) * (1_i64)) as usize, fp);
            fwrite((*m).jnt_dofadr as *const (), 4, (((*m).njnt) * (1_i64)) as usize, fp);
            fwrite((*m).jnt_bodyid as *const (), 4, (((*m).njnt) * (1_i64)) as usize, fp);
            fwrite((*m).jnt_actuatorid as *const (), 4, (((*m).njnt) * (1_i64)) as usize, fp);
            fwrite((*m).jnt_group as *const (), 4, (((*m).njnt) * (1_i64)) as usize, fp);
            fwrite((*m).jnt_limited as *const (), 1, (((*m).njnt) * (1_i64)) as usize, fp);
            fwrite((*m).jnt_actfrclimited as *const (), 1, (((*m).njnt) * (1_i64)) as usize, fp);
            fwrite((*m).jnt_actgravcomp as *const (), 1, (((*m).njnt) * (1_i64)) as usize, fp);
            fwrite((*m).jnt_solref as *const (), 8, (((*m).njnt) * (2_i64)) as usize, fp);
            fwrite((*m).jnt_solimp as *const (), 8, (((*m).njnt) * (5_i64)) as usize, fp);
            fwrite((*m).jnt_pos as *const (), 8, (((*m).njnt) * (3_i64)) as usize, fp);
            fwrite((*m).jnt_axis as *const (), 8, (((*m).njnt) * (3_i64)) as usize, fp);
            fwrite((*m).jnt_stiffness as *const (), 8, (((*m).njnt) * (1_i64)) as usize, fp);
            fwrite((*m).jnt_stiffnesspoly as *const (), 8, (((*m).njnt) * (2_i64)) as usize, fp);
            fwrite((*m).jnt_range as *const (), 8, (((*m).njnt) * (2_i64)) as usize, fp);
            fwrite((*m).jnt_actfrcrange as *const (), 8, (((*m).njnt) * (2_i64)) as usize, fp);
            fwrite((*m).jnt_margin as *const (), 8, (((*m).njnt) * (1_i64)) as usize, fp);
            fwrite((*m).jnt_user as *const (), 8, (((*m).njnt) * ((*m).nuser_jnt)) as usize, fp);
            fwrite((*m).dof_bodyid as *const (), 4, (((*m).nv) * (1_i64)) as usize, fp);
            fwrite((*m).dof_jntid as *const (), 4, (((*m).nv) * (1_i64)) as usize, fp);
            fwrite((*m).dof_parentid as *const (), 4, (((*m).nv) * (1_i64)) as usize, fp);
            fwrite((*m).dof_treeid as *const (), 4, (((*m).nv) * (1_i64)) as usize, fp);
            fwrite((*m).dof_Madr as *const (), 4, (((*m).nv) * (1_i64)) as usize, fp);
            fwrite((*m).dof_simplenum as *const (), 4, (((*m).nv) * (1_i64)) as usize, fp);
            fwrite((*m).dof_solref as *const (), 8, (((*m).nv) * (2_i64)) as usize, fp);
            fwrite((*m).dof_solimp as *const (), 8, (((*m).nv) * (5_i64)) as usize, fp);
            fwrite((*m).dof_frictionloss as *const (), 8, (((*m).nv) * (1_i64)) as usize, fp);
            fwrite((*m).dof_armature as *const (), 8, (((*m).nv) * (1_i64)) as usize, fp);
            fwrite((*m).dof_damping as *const (), 8, (((*m).nv) * (1_i64)) as usize, fp);
            fwrite((*m).dof_dampingpoly as *const (), 8, (((*m).nv) * (2_i64)) as usize, fp);
            fwrite((*m).dof_invweight0 as *const (), 8, (((*m).nv) * (1_i64)) as usize, fp);
            fwrite((*m).dof_M0 as *const (), 8, (((*m).nv) * (1_i64)) as usize, fp);
            fwrite((*m).dof_length as *const (), 8, (((*m).nv) * (1_i64)) as usize, fp);
            fwrite((*m).tree_bodyadr as *const (), 4, (((*m).ntree) * (1_i64)) as usize, fp);
            fwrite((*m).tree_bodynum as *const (), 4, (((*m).ntree) * (1_i64)) as usize, fp);
            fwrite((*m).tree_dofadr as *const (), 4, (((*m).ntree) * (1_i64)) as usize, fp);
            fwrite((*m).tree_dofnum as *const (), 4, (((*m).ntree) * (1_i64)) as usize, fp);
            fwrite((*m).tree_sleep_policy as *const (), 4, (((*m).ntree) * (1_i64)) as usize, fp);
            fwrite((*m).geom_type as *const (), 4, (((*m).ngeom) * (1_i64)) as usize, fp);
            fwrite((*m).geom_contype as *const (), 4, (((*m).ngeom) * (1_i64)) as usize, fp);
            fwrite((*m).geom_conaffinity as *const (), 4, (((*m).ngeom) * (1_i64)) as usize, fp);
            fwrite((*m).geom_condim as *const (), 4, (((*m).ngeom) * (1_i64)) as usize, fp);
            fwrite((*m).geom_bodyid as *const (), 4, (((*m).ngeom) * (1_i64)) as usize, fp);
            fwrite((*m).geom_dataid as *const (), 4, (((*m).ngeom) * (1_i64)) as usize, fp);
            fwrite((*m).geom_matid as *const (), 4, (((*m).ngeom) * (1_i64)) as usize, fp);
            fwrite((*m).geom_group as *const (), 4, (((*m).ngeom) * (1_i64)) as usize, fp);
            fwrite((*m).geom_priority as *const (), 4, (((*m).ngeom) * (1_i64)) as usize, fp);
            fwrite((*m).geom_plugin as *const (), 4, (((*m).ngeom) * (1_i64)) as usize, fp);
            fwrite((*m).geom_sameframe as *const (), 1, (((*m).ngeom) * (1_i64)) as usize, fp);
            fwrite((*m).geom_solmix as *const (), 8, (((*m).ngeom) * (1_i64)) as usize, fp);
            fwrite((*m).geom_solref as *const (), 8, (((*m).ngeom) * (2_i64)) as usize, fp);
            fwrite((*m).geom_solimp as *const (), 8, (((*m).ngeom) * (5_i64)) as usize, fp);
            fwrite((*m).geom_size as *const (), 8, (((*m).ngeom) * (3_i64)) as usize, fp);
            fwrite((*m).geom_aabb as *const (), 8, (((*m).ngeom) * (6_i64)) as usize, fp);
            fwrite((*m).geom_rbound as *const (), 8, (((*m).ngeom) * (1_i64)) as usize, fp);
            fwrite((*m).geom_pos as *const (), 8, (((*m).ngeom) * (3_i64)) as usize, fp);
            fwrite((*m).geom_quat as *const (), 8, (((*m).ngeom) * (4_i64)) as usize, fp);
            fwrite((*m).geom_friction as *const (), 8, (((*m).ngeom) * (3_i64)) as usize, fp);
            fwrite((*m).geom_margin as *const (), 8, (((*m).ngeom) * (1_i64)) as usize, fp);
            fwrite((*m).geom_gap as *const (), 8, (((*m).ngeom) * (1_i64)) as usize, fp);
            fwrite((*m).geom_fluid as *const (), 8, (((*m).ngeom) * (12_i64)) as usize, fp);
            fwrite((*m).geom_user as *const (), 8, (((*m).ngeom) * ((*m).nuser_geom)) as usize, fp);
            fwrite((*m).geom_rgba as *const (), 4, (((*m).ngeom) * (4_i64)) as usize, fp);
            fwrite((*m).site_type as *const (), 4, (((*m).nsite) * (1_i64)) as usize, fp);
            fwrite((*m).site_bodyid as *const (), 4, (((*m).nsite) * (1_i64)) as usize, fp);
            fwrite((*m).site_matid as *const (), 4, (((*m).nsite) * (1_i64)) as usize, fp);
            fwrite((*m).site_group as *const (), 4, (((*m).nsite) * (1_i64)) as usize, fp);
            fwrite((*m).site_sameframe as *const (), 1, (((*m).nsite) * (1_i64)) as usize, fp);
            fwrite((*m).site_size as *const (), 8, (((*m).nsite) * (3_i64)) as usize, fp);
            fwrite((*m).site_pos as *const (), 8, (((*m).nsite) * (3_i64)) as usize, fp);
            fwrite((*m).site_quat as *const (), 8, (((*m).nsite) * (4_i64)) as usize, fp);
            fwrite((*m).site_user as *const (), 8, (((*m).nsite) * ((*m).nuser_site)) as usize, fp);
            fwrite((*m).site_rgba as *const (), 4, (((*m).nsite) * (4_i64)) as usize, fp);
            fwrite((*m).cam_mode as *const (), 4, (((*m).ncam) * (1_i64)) as usize, fp);
            fwrite((*m).cam_bodyid as *const (), 4, (((*m).ncam) * (1_i64)) as usize, fp);
            fwrite((*m).cam_targetbodyid as *const (), 4, (((*m).ncam) * (1_i64)) as usize, fp);
            fwrite((*m).cam_pos as *const (), 8, (((*m).ncam) * (3_i64)) as usize, fp);
            fwrite((*m).cam_quat as *const (), 8, (((*m).ncam) * (4_i64)) as usize, fp);
            fwrite((*m).cam_poscom0 as *const (), 8, (((*m).ncam) * (3_i64)) as usize, fp);
            fwrite((*m).cam_pos0 as *const (), 8, (((*m).ncam) * (3_i64)) as usize, fp);
            fwrite((*m).cam_mat0 as *const (), 8, (((*m).ncam) * (9_i64)) as usize, fp);
            fwrite((*m).cam_projection as *const (), 4, (((*m).ncam) * (1_i64)) as usize, fp);
            fwrite((*m).cam_fovy as *const (), 8, (((*m).ncam) * (1_i64)) as usize, fp);
            fwrite((*m).cam_ipd as *const (), 8, (((*m).ncam) * (1_i64)) as usize, fp);
            fwrite((*m).cam_resolution as *const (), 4, (((*m).ncam) * (2_i64)) as usize, fp);
            fwrite((*m).cam_output as *const (), 4, (((*m).ncam) * (1_i64)) as usize, fp);
            fwrite((*m).cam_sensorsize as *const (), 4, (((*m).ncam) * (2_i64)) as usize, fp);
            fwrite((*m).cam_intrinsic as *const (), 4, (((*m).ncam) * (4_i64)) as usize, fp);
            fwrite((*m).cam_user as *const (), 8, (((*m).ncam) * ((*m).nuser_cam)) as usize, fp);
            fwrite((*m).light_mode as *const (), 4, (((*m).nlight) * (1_i64)) as usize, fp);
            fwrite((*m).light_bodyid as *const (), 4, (((*m).nlight) * (1_i64)) as usize, fp);
            fwrite((*m).light_targetbodyid as *const (), 4, (((*m).nlight) * (1_i64)) as usize, fp);
            fwrite((*m).light_type as *const (), 4, (((*m).nlight) * (1_i64)) as usize, fp);
            fwrite((*m).light_texid as *const (), 4, (((*m).nlight) * (1_i64)) as usize, fp);
            fwrite((*m).light_castshadow as *const (), 1, (((*m).nlight) * (1_i64)) as usize, fp);
            fwrite((*m).light_bulbradius as *const (), 4, (((*m).nlight) * (1_i64)) as usize, fp);
            fwrite((*m).light_intensity as *const (), 4, (((*m).nlight) * (1_i64)) as usize, fp);
            fwrite((*m).light_range as *const (), 4, (((*m).nlight) * (1_i64)) as usize, fp);
            fwrite((*m).light_active as *const (), 1, (((*m).nlight) * (1_i64)) as usize, fp);
            fwrite((*m).light_pos as *const (), 8, (((*m).nlight) * (3_i64)) as usize, fp);
            fwrite((*m).light_dir as *const (), 8, (((*m).nlight) * (3_i64)) as usize, fp);
            fwrite((*m).light_poscom0 as *const (), 8, (((*m).nlight) * (3_i64)) as usize, fp);
            fwrite((*m).light_pos0 as *const (), 8, (((*m).nlight) * (3_i64)) as usize, fp);
            fwrite((*m).light_dir0 as *const (), 8, (((*m).nlight) * (3_i64)) as usize, fp);
            fwrite((*m).light_attenuation as *const (), 4, (((*m).nlight) * (3_i64)) as usize, fp);
            fwrite((*m).light_cutoff as *const (), 4, (((*m).nlight) * (1_i64)) as usize, fp);
            fwrite((*m).light_exponent as *const (), 4, (((*m).nlight) * (1_i64)) as usize, fp);
            fwrite((*m).light_ambient as *const (), 4, (((*m).nlight) * (3_i64)) as usize, fp);
            fwrite((*m).light_diffuse as *const (), 4, (((*m).nlight) * (3_i64)) as usize, fp);
            fwrite((*m).light_specular as *const (), 4, (((*m).nlight) * (3_i64)) as usize, fp);
            fwrite((*m).flex_contype as *const (), 4, (((*m).nflex) * (1_i64)) as usize, fp);
            fwrite((*m).flex_conaffinity as *const (), 4, (((*m).nflex) * (1_i64)) as usize, fp);
            fwrite((*m).flex_condim as *const (), 4, (((*m).nflex) * (1_i64)) as usize, fp);
            fwrite((*m).flex_priority as *const (), 4, (((*m).nflex) * (1_i64)) as usize, fp);
            fwrite((*m).flex_solmix as *const (), 8, (((*m).nflex) * (1_i64)) as usize, fp);
            fwrite((*m).flex_solref as *const (), 8, (((*m).nflex) * (2_i64)) as usize, fp);
            fwrite((*m).flex_solimp as *const (), 8, (((*m).nflex) * (5_i64)) as usize, fp);
            fwrite((*m).flex_friction as *const (), 8, (((*m).nflex) * (3_i64)) as usize, fp);
            fwrite((*m).flex_margin as *const (), 8, (((*m).nflex) * (1_i64)) as usize, fp);
            fwrite((*m).flex_gap as *const (), 8, (((*m).nflex) * (1_i64)) as usize, fp);
            fwrite((*m).flex_internal as *const (), 1, (((*m).nflex) * (1_i64)) as usize, fp);
            fwrite((*m).flex_selfcollide as *const (), 4, (((*m).nflex) * (1_i64)) as usize, fp);
            fwrite((*m).flex_activelayers as *const (), 4, (((*m).nflex) * (1_i64)) as usize, fp);
            fwrite((*m).flex_passive as *const (), 4, (((*m).nflex) * (1_i64)) as usize, fp);
            fwrite((*m).flex_dim as *const (), 4, (((*m).nflex) * (1_i64)) as usize, fp);
            fwrite((*m).flex_matid as *const (), 4, (((*m).nflex) * (1_i64)) as usize, fp);
            fwrite((*m).flex_group as *const (), 4, (((*m).nflex) * (1_i64)) as usize, fp);
            fwrite((*m).flex_interp as *const (), 4, (((*m).nflex) * (1_i64)) as usize, fp);
            fwrite((*m).flex_cellnum as *const (), 4, (((*m).nflex) * (3_i64)) as usize, fp);
            fwrite((*m).flex_nodeadr as *const (), 4, (((*m).nflex) * (1_i64)) as usize, fp);
            fwrite((*m).flex_nodenum as *const (), 4, (((*m).nflex) * (1_i64)) as usize, fp);
            fwrite((*m).flex_vertadr as *const (), 4, (((*m).nflex) * (1_i64)) as usize, fp);
            fwrite((*m).flex_vertnum as *const (), 4, (((*m).nflex) * (1_i64)) as usize, fp);
            fwrite((*m).flex_edgeadr as *const (), 4, (((*m).nflex) * (1_i64)) as usize, fp);
            fwrite((*m).flex_edgenum as *const (), 4, (((*m).nflex) * (1_i64)) as usize, fp);
            fwrite((*m).flex_elemadr as *const (), 4, (((*m).nflex) * (1_i64)) as usize, fp);
            fwrite((*m).flex_elemnum as *const (), 4, (((*m).nflex) * (1_i64)) as usize, fp);
            fwrite((*m).flex_elemdataadr as *const (), 4, (((*m).nflex) * (1_i64)) as usize, fp);
            fwrite((*m).flex_stiffnessadr as *const (), 4, (((*m).nflex) * (1_i64)) as usize, fp);
            fwrite((*m).flex_elemedgeadr as *const (), 4, (((*m).nflex) * (1_i64)) as usize, fp);
            fwrite((*m).flex_bendingadr as *const (), 4, (((*m).nflex) * (1_i64)) as usize, fp);
            fwrite((*m).flex_shellnum as *const (), 4, (((*m).nflex) * (1_i64)) as usize, fp);
            fwrite((*m).flex_shelldataadr as *const (), 4, (((*m).nflex) * (1_i64)) as usize, fp);
            fwrite((*m).flex_evpairadr as *const (), 4, (((*m).nflex) * (1_i64)) as usize, fp);
            fwrite((*m).flex_evpairnum as *const (), 4, (((*m).nflex) * (1_i64)) as usize, fp);
            fwrite((*m).flex_texcoordadr as *const (), 4, (((*m).nflex) * (1_i64)) as usize, fp);
            fwrite((*m).flex_nodebodyid as *const (), 4, (((*m).nflexnode) * (1_i64)) as usize, fp);
            fwrite((*m).flex_vertbodyid as *const (), 4, (((*m).nflexvert) * (1_i64)) as usize, fp);
            fwrite((*m).flex_vertedgeadr as *const (), 4, (((*m).nflexvert) * (1_i64)) as usize, fp);
            fwrite((*m).flex_vertedgenum as *const (), 4, (((*m).nflexvert) * (1_i64)) as usize, fp);
            fwrite((*m).flex_vertedge as *const (), 4, (((*m).nflexedge) * (2_i64)) as usize, fp);
            fwrite((*m).flex_edge as *const (), 4, (((*m).nflexedge) * (2_i64)) as usize, fp);
            fwrite((*m).flex_edgeflap as *const (), 4, (((*m).nflexedge) * (2_i64)) as usize, fp);
            fwrite((*m).flex_elem as *const (), 4, (((*m).nflexelemdata) * (1_i64)) as usize, fp);
            fwrite((*m).flex_elemtexcoord as *const (), 4, (((*m).nflexelemdata) * (1_i64)) as usize, fp);
            fwrite((*m).flex_elemedge as *const (), 4, (((*m).nflexelemedge) * (1_i64)) as usize, fp);
            fwrite((*m).flex_elemlayer as *const (), 4, (((*m).nflexelem) * (1_i64)) as usize, fp);
            fwrite((*m).flex_shell as *const (), 4, (((*m).nflexshelldata) * (1_i64)) as usize, fp);
            fwrite((*m).flex_evpair as *const (), 4, (((*m).nflexevpair) * (2_i64)) as usize, fp);
            fwrite((*m).flex_vert as *const (), 8, (((*m).nflexvert) * (3_i64)) as usize, fp);
            fwrite((*m).flex_vert0 as *const (), 8, (((*m).nflexvert) * (3_i64)) as usize, fp);
            fwrite((*m).flex_vertmetric as *const (), 8, (((*m).nflexvert) * (4_i64)) as usize, fp);
            fwrite((*m).flex_node as *const (), 8, (((*m).nflexnode) * (3_i64)) as usize, fp);
            fwrite((*m).flex_node0 as *const (), 8, (((*m).nflexnode) * (3_i64)) as usize, fp);
            fwrite((*m).flexedge_length0 as *const (), 8, (((*m).nflexedge) * (1_i64)) as usize, fp);
            fwrite((*m).flexedge_invweight0 as *const (), 8, (((*m).nflexedge) * (1_i64)) as usize, fp);
            fwrite((*m).flex_radius as *const (), 8, (((*m).nflex) * (1_i64)) as usize, fp);
            fwrite((*m).flex_size as *const (), 8, (((*m).nflex) * (3_i64)) as usize, fp);
            fwrite((*m).flex_stiffness as *const (), 8, (((*m).nflexstiffness) * (1_i64)) as usize, fp);
            fwrite((*m).flex_bending as *const (), 8, (((*m).nflexbending) * (1_i64)) as usize, fp);
            fwrite((*m).flex_damping as *const (), 8, (((*m).nflex) * (1_i64)) as usize, fp);
            fwrite((*m).flex_edgestiffness as *const (), 8, (((*m).nflex) * (1_i64)) as usize, fp);
            fwrite((*m).flex_edgedamping as *const (), 8, (((*m).nflex) * (1_i64)) as usize, fp);
            fwrite((*m).flex_edgeequality as *const (), 4, (((*m).nflex) * (1_i64)) as usize, fp);
            fwrite((*m).flex_rigid as *const (), 1, (((*m).nflex) * (1_i64)) as usize, fp);
            fwrite((*m).flexedge_rigid as *const (), 1, (((*m).nflexedge) * (1_i64)) as usize, fp);
            fwrite((*m).flex_centered as *const (), 1, (((*m).nflex) * (1_i64)) as usize, fp);
            fwrite((*m).flex_flatskin as *const (), 1, (((*m).nflex) * (1_i64)) as usize, fp);
            fwrite((*m).flex_bvhadr as *const (), 4, (((*m).nflex) * (1_i64)) as usize, fp);
            fwrite((*m).flex_bvhnum as *const (), 4, (((*m).nflex) * (1_i64)) as usize, fp);
            fwrite((*m).flexedge_J_rownnz as *const (), 4, (((*m).nflexedge) * (1_i64)) as usize, fp);
            fwrite((*m).flexedge_J_rowadr as *const (), 4, (((*m).nflexedge) * (1_i64)) as usize, fp);
            fwrite((*m).flexedge_J_colind as *const (), 4, (((*m).nJfe) * (1_i64)) as usize, fp);
            fwrite((*m).flexvert_J_rownnz as *const (), 4, (((*m).nflexvert) * (2_i64)) as usize, fp);
            fwrite((*m).flexvert_J_rowadr as *const (), 4, (((*m).nflexvert) * (2_i64)) as usize, fp);
            fwrite((*m).flexvert_J_colind as *const (), 4, (((*m).nJfv) * (2_i64)) as usize, fp);
            fwrite((*m).flex_rgba as *const (), 4, (((*m).nflex) * (4_i64)) as usize, fp);
            fwrite((*m).flex_texcoord as *const (), 4, (((*m).nflextexcoord) * (2_i64)) as usize, fp);
            fwrite((*m).mesh_vertadr as *const (), 4, (((*m).nmesh) * (1_i64)) as usize, fp);
            fwrite((*m).mesh_vertnum as *const (), 4, (((*m).nmesh) * (1_i64)) as usize, fp);
            fwrite((*m).mesh_faceadr as *const (), 4, (((*m).nmesh) * (1_i64)) as usize, fp);
            fwrite((*m).mesh_facenum as *const (), 4, (((*m).nmesh) * (1_i64)) as usize, fp);
            fwrite((*m).mesh_bvhadr as *const (), 4, (((*m).nmesh) * (1_i64)) as usize, fp);
            fwrite((*m).mesh_bvhnum as *const (), 4, (((*m).nmesh) * (1_i64)) as usize, fp);
            fwrite((*m).mesh_octadr as *const (), 4, (((*m).nmesh) * (1_i64)) as usize, fp);
            fwrite((*m).mesh_octnum as *const (), 4, (((*m).nmesh) * (1_i64)) as usize, fp);
            fwrite((*m).mesh_normaladr as *const (), 4, (((*m).nmesh) * (1_i64)) as usize, fp);
            fwrite((*m).mesh_normalnum as *const (), 4, (((*m).nmesh) * (1_i64)) as usize, fp);
            fwrite((*m).mesh_texcoordadr as *const (), 4, (((*m).nmesh) * (1_i64)) as usize, fp);
            fwrite((*m).mesh_texcoordnum as *const (), 4, (((*m).nmesh) * (1_i64)) as usize, fp);
            fwrite((*m).mesh_graphadr as *const (), 4, (((*m).nmesh) * (1_i64)) as usize, fp);
            fwrite((*m).mesh_vert as *const (), 4, (((*m).nmeshvert) * (3_i64)) as usize, fp);
            fwrite((*m).mesh_normal as *const (), 4, (((*m).nmeshnormal) * (3_i64)) as usize, fp);
            fwrite((*m).mesh_texcoord as *const (), 4, (((*m).nmeshtexcoord) * (2_i64)) as usize, fp);
            fwrite((*m).mesh_face as *const (), 4, (((*m).nmeshface) * (3_i64)) as usize, fp);
            fwrite((*m).mesh_facenormal as *const (), 4, (((*m).nmeshface) * (3_i64)) as usize, fp);
            fwrite((*m).mesh_facetexcoord as *const (), 4, (((*m).nmeshface) * (3_i64)) as usize, fp);
            fwrite((*m).mesh_graph as *const (), 4, (((*m).nmeshgraph) * (1_i64)) as usize, fp);
            fwrite((*m).mesh_scale as *const (), 8, (((*m).nmesh) * (3_i64)) as usize, fp);
            fwrite((*m).mesh_pos as *const (), 8, (((*m).nmesh) * (3_i64)) as usize, fp);
            fwrite((*m).mesh_quat as *const (), 8, (((*m).nmesh) * (4_i64)) as usize, fp);
            fwrite((*m).mesh_pathadr as *const (), 4, (((*m).nmesh) * (1_i64)) as usize, fp);
            fwrite((*m).mesh_polynum as *const (), 4, (((*m).nmesh) * (1_i64)) as usize, fp);
            fwrite((*m).mesh_polyadr as *const (), 4, (((*m).nmesh) * (1_i64)) as usize, fp);
            fwrite((*m).mesh_polynormal as *const (), 8, (((*m).nmeshpoly) * (3_i64)) as usize, fp);
            fwrite((*m).mesh_polyvertadr as *const (), 4, (((*m).nmeshpoly) * (1_i64)) as usize, fp);
            fwrite((*m).mesh_polyvertnum as *const (), 4, (((*m).nmeshpoly) * (1_i64)) as usize, fp);
            fwrite((*m).mesh_polyvert as *const (), 4, (((*m).nmeshpolyvert) * (1_i64)) as usize, fp);
            fwrite((*m).mesh_polymapadr as *const (), 4, (((*m).nmeshvert) * (1_i64)) as usize, fp);
            fwrite((*m).mesh_polymapnum as *const (), 4, (((*m).nmeshvert) * (1_i64)) as usize, fp);
            fwrite((*m).mesh_polymap as *const (), 4, (((*m).nmeshpolymap) * (1_i64)) as usize, fp);
            fwrite((*m).skin_matid as *const (), 4, (((*m).nskin) * (1_i64)) as usize, fp);
            fwrite((*m).skin_group as *const (), 4, (((*m).nskin) * (1_i64)) as usize, fp);
            fwrite((*m).skin_rgba as *const (), 4, (((*m).nskin) * (4_i64)) as usize, fp);
            fwrite((*m).skin_inflate as *const (), 4, (((*m).nskin) * (1_i64)) as usize, fp);
            fwrite((*m).skin_vertadr as *const (), 4, (((*m).nskin) * (1_i64)) as usize, fp);
            fwrite((*m).skin_vertnum as *const (), 4, (((*m).nskin) * (1_i64)) as usize, fp);
            fwrite((*m).skin_texcoordadr as *const (), 4, (((*m).nskin) * (1_i64)) as usize, fp);
            fwrite((*m).skin_faceadr as *const (), 4, (((*m).nskin) * (1_i64)) as usize, fp);
            fwrite((*m).skin_facenum as *const (), 4, (((*m).nskin) * (1_i64)) as usize, fp);
            fwrite((*m).skin_boneadr as *const (), 4, (((*m).nskin) * (1_i64)) as usize, fp);
            fwrite((*m).skin_bonenum as *const (), 4, (((*m).nskin) * (1_i64)) as usize, fp);
            fwrite((*m).skin_vert as *const (), 4, (((*m).nskinvert) * (3_i64)) as usize, fp);
            fwrite((*m).skin_texcoord as *const (), 4, (((*m).nskintexvert) * (2_i64)) as usize, fp);
            fwrite((*m).skin_face as *const (), 4, (((*m).nskinface) * (3_i64)) as usize, fp);
            fwrite((*m).skin_bonevertadr as *const (), 4, (((*m).nskinbone) * (1_i64)) as usize, fp);
            fwrite((*m).skin_bonevertnum as *const (), 4, (((*m).nskinbone) * (1_i64)) as usize, fp);
            fwrite((*m).skin_bonebindpos as *const (), 4, (((*m).nskinbone) * (3_i64)) as usize, fp);
            fwrite((*m).skin_bonebindquat as *const (), 4, (((*m).nskinbone) * (4_i64)) as usize, fp);
            fwrite((*m).skin_bonebodyid as *const (), 4, (((*m).nskinbone) * (1_i64)) as usize, fp);
            fwrite((*m).skin_bonevertid as *const (), 4, (((*m).nskinbonevert) * (1_i64)) as usize, fp);
            fwrite((*m).skin_bonevertweight as *const (), 4, (((*m).nskinbonevert) * (1_i64)) as usize, fp);
            fwrite((*m).skin_pathadr as *const (), 4, (((*m).nskin) * (1_i64)) as usize, fp);
            fwrite((*m).hfield_size as *const (), 8, (((*m).nhfield) * (4_i64)) as usize, fp);
            fwrite((*m).hfield_nrow as *const (), 4, (((*m).nhfield) * (1_i64)) as usize, fp);
            fwrite((*m).hfield_ncol as *const (), 4, (((*m).nhfield) * (1_i64)) as usize, fp);
            fwrite((*m).hfield_adr as *const (), 4, (((*m).nhfield) * (1_i64)) as usize, fp);
            fwrite((*m).hfield_data as *const (), 4, (((*m).nhfielddata) * (1_i64)) as usize, fp);
            fwrite((*m).hfield_pathadr as *const (), 4, (((*m).nhfield) * (1_i64)) as usize, fp);
            fwrite((*m).tex_type as *const (), 4, (((*m).ntex) * (1_i64)) as usize, fp);
            fwrite((*m).tex_colorspace as *const (), 4, (((*m).ntex) * (1_i64)) as usize, fp);
            fwrite((*m).tex_height as *const (), 4, (((*m).ntex) * (1_i64)) as usize, fp);
            fwrite((*m).tex_width as *const (), 4, (((*m).ntex) * (1_i64)) as usize, fp);
            fwrite((*m).tex_nchannel as *const (), 4, (((*m).ntex) * (1_i64)) as usize, fp);
            fwrite((*m).tex_adr as *const (), 8, (((*m).ntex) * (1_i64)) as usize, fp);
            fwrite((*m).tex_data as *const (), 1, (((*m).ntexdata) * (1_i64)) as usize, fp);
            fwrite((*m).tex_pathadr as *const (), 4, (((*m).ntex) * (1_i64)) as usize, fp);
            fwrite((*m).mat_texid as *const (), 4, (((*m).nmat) * (10_i64)) as usize, fp);
            fwrite((*m).mat_texuniform as *const (), 1, (((*m).nmat) * (1_i64)) as usize, fp);
            fwrite((*m).mat_texrepeat as *const (), 4, (((*m).nmat) * (2_i64)) as usize, fp);
            fwrite((*m).mat_emission as *const (), 4, (((*m).nmat) * (1_i64)) as usize, fp);
            fwrite((*m).mat_specular as *const (), 4, (((*m).nmat) * (1_i64)) as usize, fp);
            fwrite((*m).mat_shininess as *const (), 4, (((*m).nmat) * (1_i64)) as usize, fp);
            fwrite((*m).mat_reflectance as *const (), 4, (((*m).nmat) * (1_i64)) as usize, fp);
            fwrite((*m).mat_metallic as *const (), 4, (((*m).nmat) * (1_i64)) as usize, fp);
            fwrite((*m).mat_roughness as *const (), 4, (((*m).nmat) * (1_i64)) as usize, fp);
            fwrite((*m).mat_rgba as *const (), 4, (((*m).nmat) * (4_i64)) as usize, fp);
            fwrite((*m).pair_dim as *const (), 4, (((*m).npair) * (1_i64)) as usize, fp);
            fwrite((*m).pair_geom1 as *const (), 4, (((*m).npair) * (1_i64)) as usize, fp);
            fwrite((*m).pair_geom2 as *const (), 4, (((*m).npair) * (1_i64)) as usize, fp);
            fwrite((*m).pair_signature as *const (), 4, (((*m).npair) * (1_i64)) as usize, fp);
            fwrite((*m).pair_solref as *const (), 8, (((*m).npair) * (2_i64)) as usize, fp);
            fwrite((*m).pair_solreffriction as *const (), 8, (((*m).npair) * (2_i64)) as usize, fp);
            fwrite((*m).pair_solimp as *const (), 8, (((*m).npair) * (5_i64)) as usize, fp);
            fwrite((*m).pair_margin as *const (), 8, (((*m).npair) * (1_i64)) as usize, fp);
            fwrite((*m).pair_gap as *const (), 8, (((*m).npair) * (1_i64)) as usize, fp);
            fwrite((*m).pair_friction as *const (), 8, (((*m).npair) * (5_i64)) as usize, fp);
            fwrite((*m).exclude_signature as *const (), 4, (((*m).nexclude) * (1_i64)) as usize, fp);
            fwrite((*m).eq_type as *const (), 4, (((*m).neq) * (1_i64)) as usize, fp);
            fwrite((*m).eq_obj1id as *const (), 4, (((*m).neq) * (1_i64)) as usize, fp);
            fwrite((*m).eq_obj2id as *const (), 4, (((*m).neq) * (1_i64)) as usize, fp);
            fwrite((*m).eq_objtype as *const (), 4, (((*m).neq) * (1_i64)) as usize, fp);
            fwrite((*m).eq_active0 as *const (), 1, (((*m).neq) * (1_i64)) as usize, fp);
            fwrite((*m).eq_solref as *const (), 8, (((*m).neq) * (2_i64)) as usize, fp);
            fwrite((*m).eq_solimp as *const (), 8, (((*m).neq) * (5_i64)) as usize, fp);
            fwrite((*m).eq_data as *const (), 8, (((*m).neq) * (11_i64)) as usize, fp);
            fwrite((*m).tendon_adr as *const (), 4, (((*m).ntendon) * (1_i64)) as usize, fp);
            fwrite((*m).tendon_num as *const (), 4, (((*m).ntendon) * (1_i64)) as usize, fp);
            fwrite((*m).tendon_matid as *const (), 4, (((*m).ntendon) * (1_i64)) as usize, fp);
            fwrite((*m).tendon_actuatorid as *const (), 4, (((*m).ntendon) * (1_i64)) as usize, fp);
            fwrite((*m).tendon_group as *const (), 4, (((*m).ntendon) * (1_i64)) as usize, fp);
            fwrite((*m).tendon_treenum as *const (), 4, (((*m).ntendon) * (1_i64)) as usize, fp);
            fwrite((*m).tendon_treeid as *const (), 4, (((*m).ntendon) * (2_i64)) as usize, fp);
            fwrite((*m).ten_J_rownnz as *const (), 4, (((*m).ntendon) * (1_i64)) as usize, fp);
            fwrite((*m).ten_J_rowadr as *const (), 4, (((*m).ntendon) * (1_i64)) as usize, fp);
            fwrite((*m).ten_J_colind as *const (), 4, (((*m).nJten) * (1_i64)) as usize, fp);
            fwrite((*m).tendon_limited as *const (), 1, (((*m).ntendon) * (1_i64)) as usize, fp);
            fwrite((*m).tendon_actfrclimited as *const (), 1, (((*m).ntendon) * (1_i64)) as usize, fp);
            fwrite((*m).tendon_width as *const (), 8, (((*m).ntendon) * (1_i64)) as usize, fp);
            fwrite((*m).tendon_solref_lim as *const (), 8, (((*m).ntendon) * (2_i64)) as usize, fp);
            fwrite((*m).tendon_solimp_lim as *const (), 8, (((*m).ntendon) * (5_i64)) as usize, fp);
            fwrite((*m).tendon_solref_fri as *const (), 8, (((*m).ntendon) * (2_i64)) as usize, fp);
            fwrite((*m).tendon_solimp_fri as *const (), 8, (((*m).ntendon) * (5_i64)) as usize, fp);
            fwrite((*m).tendon_range as *const (), 8, (((*m).ntendon) * (2_i64)) as usize, fp);
            fwrite((*m).tendon_actfrcrange as *const (), 8, (((*m).ntendon) * (2_i64)) as usize, fp);
            fwrite((*m).tendon_margin as *const (), 8, (((*m).ntendon) * (1_i64)) as usize, fp);
            fwrite((*m).tendon_stiffness as *const (), 8, (((*m).ntendon) * (1_i64)) as usize, fp);
            fwrite((*m).tendon_stiffnesspoly as *const (), 8, (((*m).ntendon) * (2_i64)) as usize, fp);
            fwrite((*m).tendon_damping as *const (), 8, (((*m).ntendon) * (1_i64)) as usize, fp);
            fwrite((*m).tendon_dampingpoly as *const (), 8, (((*m).ntendon) * (2_i64)) as usize, fp);
            fwrite((*m).tendon_armature as *const (), 8, (((*m).ntendon) * (1_i64)) as usize, fp);
            fwrite((*m).tendon_frictionloss as *const (), 8, (((*m).ntendon) * (1_i64)) as usize, fp);
            fwrite((*m).tendon_lengthspring as *const (), 8, (((*m).ntendon) * (2_i64)) as usize, fp);
            fwrite((*m).tendon_length0 as *const (), 8, (((*m).ntendon) * (1_i64)) as usize, fp);
            fwrite((*m).tendon_invweight0 as *const (), 8, (((*m).ntendon) * (1_i64)) as usize, fp);
            fwrite((*m).tendon_user as *const (), 8, (((*m).ntendon) * ((*m).nuser_tendon)) as usize, fp);
            fwrite((*m).tendon_rgba as *const (), 4, (((*m).ntendon) * (4_i64)) as usize, fp);
            fwrite((*m).wrap_type as *const (), 4, (((*m).nwrap) * (1_i64)) as usize, fp);
            fwrite((*m).wrap_objid as *const (), 4, (((*m).nwrap) * (1_i64)) as usize, fp);
            fwrite((*m).wrap_prm as *const (), 8, (((*m).nwrap) * (1_i64)) as usize, fp);
            fwrite((*m).actuator_trntype as *const (), 4, (((*m).nu) * (1_i64)) as usize, fp);
            fwrite((*m).actuator_dyntype as *const (), 4, (((*m).nu) * (1_i64)) as usize, fp);
            fwrite((*m).actuator_gaintype as *const (), 4, (((*m).nu) * (1_i64)) as usize, fp);
            fwrite((*m).actuator_biastype as *const (), 4, (((*m).nu) * (1_i64)) as usize, fp);
            fwrite((*m).actuator_trnid as *const (), 4, (((*m).nu) * (2_i64)) as usize, fp);
            fwrite((*m).actuator_damping as *const (), 8, (((*m).nu) * (1_i64)) as usize, fp);
            fwrite((*m).actuator_dampingpoly as *const (), 8, (((*m).nu) * (2_i64)) as usize, fp);
            fwrite((*m).actuator_armature as *const (), 8, (((*m).nu) * (1_i64)) as usize, fp);
            fwrite((*m).actuator_actadr as *const (), 4, (((*m).nu) * (1_i64)) as usize, fp);
            fwrite((*m).actuator_actnum as *const (), 4, (((*m).nu) * (1_i64)) as usize, fp);
            fwrite((*m).actuator_group as *const (), 4, (((*m).nu) * (1_i64)) as usize, fp);
            fwrite((*m).actuator_history as *const (), 4, (((*m).nu) * (2_i64)) as usize, fp);
            fwrite((*m).actuator_historyadr as *const (), 4, (((*m).nu) * (1_i64)) as usize, fp);
            fwrite((*m).actuator_delay as *const (), 8, (((*m).nu) * (1_i64)) as usize, fp);
            fwrite((*m).actuator_ctrllimited as *const (), 1, (((*m).nu) * (1_i64)) as usize, fp);
            fwrite((*m).actuator_forcelimited as *const (), 1, (((*m).nu) * (1_i64)) as usize, fp);
            fwrite((*m).actuator_actlimited as *const (), 1, (((*m).nu) * (1_i64)) as usize, fp);
            fwrite((*m).actuator_dynprm as *const (), 8, (((*m).nu) * (10_i64)) as usize, fp);
            fwrite((*m).actuator_gainprm as *const (), 8, (((*m).nu) * (10_i64)) as usize, fp);
            fwrite((*m).actuator_biasprm as *const (), 8, (((*m).nu) * (10_i64)) as usize, fp);
            fwrite((*m).actuator_actearly as *const (), 1, (((*m).nu) * (1_i64)) as usize, fp);
            fwrite((*m).actuator_ctrlrange as *const (), 8, (((*m).nu) * (2_i64)) as usize, fp);
            fwrite((*m).actuator_forcerange as *const (), 8, (((*m).nu) * (2_i64)) as usize, fp);
            fwrite((*m).actuator_actrange as *const (), 8, (((*m).nu) * (2_i64)) as usize, fp);
            fwrite((*m).actuator_gear as *const (), 8, (((*m).nu) * (6_i64)) as usize, fp);
            fwrite((*m).actuator_cranklength as *const (), 8, (((*m).nu) * (1_i64)) as usize, fp);
            fwrite((*m).actuator_acc0 as *const (), 8, (((*m).nu) * (1_i64)) as usize, fp);
            fwrite((*m).actuator_length0 as *const (), 8, (((*m).nu) * (1_i64)) as usize, fp);
            fwrite((*m).actuator_lengthrange as *const (), 8, (((*m).nu) * (2_i64)) as usize, fp);
            fwrite((*m).actuator_user as *const (), 8, (((*m).nu) * ((*m).nuser_actuator)) as usize, fp);
            fwrite((*m).actuator_plugin as *const (), 4, (((*m).nu) * (1_i64)) as usize, fp);
            fwrite((*m).sensor_type as *const (), 4, (((*m).nsensor) * (1_i64)) as usize, fp);
            fwrite((*m).sensor_datatype as *const (), 4, (((*m).nsensor) * (1_i64)) as usize, fp);
            fwrite((*m).sensor_needstage as *const (), 4, (((*m).nsensor) * (1_i64)) as usize, fp);
            fwrite((*m).sensor_objtype as *const (), 4, (((*m).nsensor) * (1_i64)) as usize, fp);
            fwrite((*m).sensor_objid as *const (), 4, (((*m).nsensor) * (1_i64)) as usize, fp);
            fwrite((*m).sensor_reftype as *const (), 4, (((*m).nsensor) * (1_i64)) as usize, fp);
            fwrite((*m).sensor_refid as *const (), 4, (((*m).nsensor) * (1_i64)) as usize, fp);
            fwrite((*m).sensor_intprm as *const (), 4, (((*m).nsensor) * (3_i64)) as usize, fp);
            fwrite((*m).sensor_dim as *const (), 4, (((*m).nsensor) * (1_i64)) as usize, fp);
            fwrite((*m).sensor_adr as *const (), 4, (((*m).nsensor) * (1_i64)) as usize, fp);
            fwrite((*m).sensor_cutoff as *const (), 8, (((*m).nsensor) * (1_i64)) as usize, fp);
            fwrite((*m).sensor_noise as *const (), 8, (((*m).nsensor) * (1_i64)) as usize, fp);
            fwrite((*m).sensor_history as *const (), 4, (((*m).nsensor) * (2_i64)) as usize, fp);
            fwrite((*m).sensor_historyadr as *const (), 4, (((*m).nsensor) * (1_i64)) as usize, fp);
            fwrite((*m).sensor_delay as *const (), 8, (((*m).nsensor) * (1_i64)) as usize, fp);
            fwrite((*m).sensor_interval as *const (), 8, (((*m).nsensor) * (2_i64)) as usize, fp);
            fwrite((*m).sensor_user as *const (), 8, (((*m).nsensor) * ((*m).nuser_sensor)) as usize, fp);
            fwrite((*m).sensor_plugin as *const (), 4, (((*m).nsensor) * (1_i64)) as usize, fp);
            fwrite((*m).plugin as *const (), 4, (((*m).nplugin) * (1_i64)) as usize, fp);
            fwrite((*m).plugin_stateadr as *const (), 4, (((*m).nplugin) * (1_i64)) as usize, fp);
            fwrite((*m).plugin_statenum as *const (), 4, (((*m).nplugin) * (1_i64)) as usize, fp);
            fwrite((*m).plugin_attr as *const (), 1, (((*m).npluginattr) * (1_i64)) as usize, fp);
            fwrite((*m).plugin_attradr as *const (), 4, (((*m).nplugin) * (1_i64)) as usize, fp);
            fwrite((*m).numeric_adr as *const (), 4, (((*m).nnumeric) * (1_i64)) as usize, fp);
            fwrite((*m).numeric_size as *const (), 4, (((*m).nnumeric) * (1_i64)) as usize, fp);
            fwrite((*m).numeric_data as *const (), 8, (((*m).nnumericdata) * (1_i64)) as usize, fp);
            fwrite((*m).text_adr as *const (), 4, (((*m).ntext) * (1_i64)) as usize, fp);
            fwrite((*m).text_size as *const (), 4, (((*m).ntext) * (1_i64)) as usize, fp);
            fwrite((*m).text_data as *const (), 1, (((*m).ntextdata) * (1_i64)) as usize, fp);
            fwrite((*m).tuple_adr as *const (), 4, (((*m).ntuple) * (1_i64)) as usize, fp);
            fwrite((*m).tuple_size as *const (), 4, (((*m).ntuple) * (1_i64)) as usize, fp);
            fwrite((*m).tuple_objtype as *const (), 4, (((*m).ntupledata) * (1_i64)) as usize, fp);
            fwrite((*m).tuple_objid as *const (), 4, (((*m).ntupledata) * (1_i64)) as usize, fp);
            fwrite((*m).tuple_objprm as *const (), 8, (((*m).ntupledata) * (1_i64)) as usize, fp);
            fwrite((*m).key_time as *const (), 8, (((*m).nkey) * (1_i64)) as usize, fp);
            fwrite((*m).key_qpos as *const (), 8, (((*m).nkey) * ((*m).nq)) as usize, fp);
            fwrite((*m).key_qvel as *const (), 8, (((*m).nkey) * ((*m).nv)) as usize, fp);
            fwrite((*m).key_act as *const (), 8, (((*m).nkey) * ((*m).na)) as usize, fp);
            fwrite((*m).key_mpos as *const (), 8, (((*m).nkey) * ((*m).nmocap * 3)) as usize, fp);
            fwrite((*m).key_mquat as *const (), 8, (((*m).nkey) * ((*m).nmocap * 4)) as usize, fp);
            fwrite((*m).key_ctrl as *const (), 8, (((*m).nkey) * ((*m).nu)) as usize, fp);
            fwrite((*m).name_bodyadr as *const (), 4, (((*m).nbody) * (1_i64)) as usize, fp);
            fwrite((*m).name_jntadr as *const (), 4, (((*m).njnt) * (1_i64)) as usize, fp);
            fwrite((*m).name_geomadr as *const (), 4, (((*m).ngeom) * (1_i64)) as usize, fp);
            fwrite((*m).name_siteadr as *const (), 4, (((*m).nsite) * (1_i64)) as usize, fp);
            fwrite((*m).name_camadr as *const (), 4, (((*m).ncam) * (1_i64)) as usize, fp);
            fwrite((*m).name_lightadr as *const (), 4, (((*m).nlight) * (1_i64)) as usize, fp);
            fwrite((*m).name_flexadr as *const (), 4, (((*m).nflex) * (1_i64)) as usize, fp);
            fwrite((*m).name_meshadr as *const (), 4, (((*m).nmesh) * (1_i64)) as usize, fp);
            fwrite((*m).name_skinadr as *const (), 4, (((*m).nskin) * (1_i64)) as usize, fp);
            fwrite((*m).name_hfieldadr as *const (), 4, (((*m).nhfield) * (1_i64)) as usize, fp);
            fwrite((*m).name_texadr as *const (), 4, (((*m).ntex) * (1_i64)) as usize, fp);
            fwrite((*m).name_matadr as *const (), 4, (((*m).nmat) * (1_i64)) as usize, fp);
            fwrite((*m).name_pairadr as *const (), 4, (((*m).npair) * (1_i64)) as usize, fp);
            fwrite((*m).name_excludeadr as *const (), 4, (((*m).nexclude) * (1_i64)) as usize, fp);
            fwrite((*m).name_eqadr as *const (), 4, (((*m).neq) * (1_i64)) as usize, fp);
            fwrite((*m).name_tendonadr as *const (), 4, (((*m).ntendon) * (1_i64)) as usize, fp);
            fwrite((*m).name_actuatoradr as *const (), 4, (((*m).nu) * (1_i64)) as usize, fp);
            fwrite((*m).name_sensoradr as *const (), 4, (((*m).nsensor) * (1_i64)) as usize, fp);
            fwrite((*m).name_numericadr as *const (), 4, (((*m).nnumeric) * (1_i64)) as usize, fp);
            fwrite((*m).name_textadr as *const (), 4, (((*m).ntext) * (1_i64)) as usize, fp);
            fwrite((*m).name_tupleadr as *const (), 4, (((*m).ntuple) * (1_i64)) as usize, fp);
            fwrite((*m).name_keyadr as *const (), 4, (((*m).nkey) * (1_i64)) as usize, fp);
            fwrite((*m).name_pluginadr as *const (), 4, (((*m).nplugin) * (1_i64)) as usize, fp);
            fwrite((*m).names as *const (), 1, (((*m).nnames) * (1_i64)) as usize, fp);
            fwrite((*m).names_map as *const (), 4, (((*m).nnames_map) * (1_i64)) as usize, fp);
            fwrite((*m).paths as *const (), 1, (((*m).npaths) * (1_i64)) as usize, fp);
            fwrite((*m).B_rownnz as *const (), 4, (((*m).nbody) * (1_i64)) as usize, fp);
            fwrite((*m).B_rowadr as *const (), 4, (((*m).nbody) * (1_i64)) as usize, fp);
            fwrite((*m).B_colind as *const (), 4, (((*m).nB) * (1_i64)) as usize, fp);
            fwrite((*m).M_rownnz as *const (), 4, (((*m).nv) * (1_i64)) as usize, fp);
            fwrite((*m).M_rowadr as *const (), 4, (((*m).nv) * (1_i64)) as usize, fp);
            fwrite((*m).M_colind as *const (), 4, (((*m).nC) * (1_i64)) as usize, fp);
            fwrite((*m).mapM2M as *const (), 4, (((*m).nC) * (1_i64)) as usize, fp);
            fwrite((*m).D_rownnz as *const (), 4, (((*m).nv) * (1_i64)) as usize, fp);
            fwrite((*m).D_rowadr as *const (), 4, (((*m).nv) * (1_i64)) as usize, fp);
            fwrite((*m).D_diag as *const (), 4, (((*m).nv) * (1_i64)) as usize, fp);
            fwrite((*m).D_colind as *const (), 4, (((*m).nD) * (1_i64)) as usize, fp);
            fwrite((*m).mapM2D as *const (), 4, (((*m).nD) * (1_i64)) as usize, fp);
            fwrite((*m).mapD2M as *const (), 4, (((*m).nC) * (1_i64)) as usize, fp);
        } else {
            bufwrite(header.as_ptr() as *const (), 20, buffer_sz as i64, buffer, &mut ptrbuf);
            // MJMODEL_SIZES: write each size field to buffer
        bufwrite(&(*m).nq as *const i64 as *const (), 8, buffer_sz as i64, buffer, &mut ptrbuf);
        bufwrite(&(*m).nv as *const i64 as *const (), 8, buffer_sz as i64, buffer, &mut ptrbuf);
        bufwrite(&(*m).nu as *const i64 as *const (), 8, buffer_sz as i64, buffer, &mut ptrbuf);
        bufwrite(&(*m).na as *const i64 as *const (), 8, buffer_sz as i64, buffer, &mut ptrbuf);
        bufwrite(&(*m).nbody as *const i64 as *const (), 8, buffer_sz as i64, buffer, &mut ptrbuf);
        bufwrite(&(*m).nbvh as *const i64 as *const (), 8, buffer_sz as i64, buffer, &mut ptrbuf);
        bufwrite(&(*m).nbvhstatic as *const i64 as *const (), 8, buffer_sz as i64, buffer, &mut ptrbuf);
        bufwrite(&(*m).nbvhdynamic as *const i64 as *const (), 8, buffer_sz as i64, buffer, &mut ptrbuf);
        bufwrite(&(*m).noct as *const i64 as *const (), 8, buffer_sz as i64, buffer, &mut ptrbuf);
        bufwrite(&(*m).njnt as *const i64 as *const (), 8, buffer_sz as i64, buffer, &mut ptrbuf);
        bufwrite(&(*m).ntree as *const i64 as *const (), 8, buffer_sz as i64, buffer, &mut ptrbuf);
        bufwrite(&(*m).nM as *const i64 as *const (), 8, buffer_sz as i64, buffer, &mut ptrbuf);
        bufwrite(&(*m).nB as *const i64 as *const (), 8, buffer_sz as i64, buffer, &mut ptrbuf);
        bufwrite(&(*m).nC as *const i64 as *const (), 8, buffer_sz as i64, buffer, &mut ptrbuf);
        bufwrite(&(*m).nD as *const i64 as *const (), 8, buffer_sz as i64, buffer, &mut ptrbuf);
        bufwrite(&(*m).ngeom as *const i64 as *const (), 8, buffer_sz as i64, buffer, &mut ptrbuf);
        bufwrite(&(*m).nsite as *const i64 as *const (), 8, buffer_sz as i64, buffer, &mut ptrbuf);
        bufwrite(&(*m).ncam as *const i64 as *const (), 8, buffer_sz as i64, buffer, &mut ptrbuf);
        bufwrite(&(*m).nlight as *const i64 as *const (), 8, buffer_sz as i64, buffer, &mut ptrbuf);
        bufwrite(&(*m).nflex as *const i64 as *const (), 8, buffer_sz as i64, buffer, &mut ptrbuf);
        bufwrite(&(*m).nflexnode as *const i64 as *const (), 8, buffer_sz as i64, buffer, &mut ptrbuf);
        bufwrite(&(*m).nflexvert as *const i64 as *const (), 8, buffer_sz as i64, buffer, &mut ptrbuf);
        bufwrite(&(*m).nflexedge as *const i64 as *const (), 8, buffer_sz as i64, buffer, &mut ptrbuf);
        bufwrite(&(*m).nflexelem as *const i64 as *const (), 8, buffer_sz as i64, buffer, &mut ptrbuf);
        bufwrite(&(*m).nflexelemdata as *const i64 as *const (), 8, buffer_sz as i64, buffer, &mut ptrbuf);
        bufwrite(&(*m).nflexstiffness as *const i64 as *const (), 8, buffer_sz as i64, buffer, &mut ptrbuf);
        bufwrite(&(*m).nflexbending as *const i64 as *const (), 8, buffer_sz as i64, buffer, &mut ptrbuf);
        bufwrite(&(*m).nflexelemedge as *const i64 as *const (), 8, buffer_sz as i64, buffer, &mut ptrbuf);
        bufwrite(&(*m).nflexshelldata as *const i64 as *const (), 8, buffer_sz as i64, buffer, &mut ptrbuf);
        bufwrite(&(*m).nflexevpair as *const i64 as *const (), 8, buffer_sz as i64, buffer, &mut ptrbuf);
        bufwrite(&(*m).nflextexcoord as *const i64 as *const (), 8, buffer_sz as i64, buffer, &mut ptrbuf);
        bufwrite(&(*m).nJfe as *const i64 as *const (), 8, buffer_sz as i64, buffer, &mut ptrbuf);
        bufwrite(&(*m).nJfv as *const i64 as *const (), 8, buffer_sz as i64, buffer, &mut ptrbuf);
        bufwrite(&(*m).nmesh as *const i64 as *const (), 8, buffer_sz as i64, buffer, &mut ptrbuf);
        bufwrite(&(*m).nmeshvert as *const i64 as *const (), 8, buffer_sz as i64, buffer, &mut ptrbuf);
        bufwrite(&(*m).nmeshnormal as *const i64 as *const (), 8, buffer_sz as i64, buffer, &mut ptrbuf);
        bufwrite(&(*m).nmeshtexcoord as *const i64 as *const (), 8, buffer_sz as i64, buffer, &mut ptrbuf);
        bufwrite(&(*m).nmeshface as *const i64 as *const (), 8, buffer_sz as i64, buffer, &mut ptrbuf);
        bufwrite(&(*m).nmeshgraph as *const i64 as *const (), 8, buffer_sz as i64, buffer, &mut ptrbuf);
        bufwrite(&(*m).nmeshpoly as *const i64 as *const (), 8, buffer_sz as i64, buffer, &mut ptrbuf);
        bufwrite(&(*m).nmeshpolyvert as *const i64 as *const (), 8, buffer_sz as i64, buffer, &mut ptrbuf);
        bufwrite(&(*m).nmeshpolymap as *const i64 as *const (), 8, buffer_sz as i64, buffer, &mut ptrbuf);
        bufwrite(&(*m).nskin as *const i64 as *const (), 8, buffer_sz as i64, buffer, &mut ptrbuf);
        bufwrite(&(*m).nskinvert as *const i64 as *const (), 8, buffer_sz as i64, buffer, &mut ptrbuf);
        bufwrite(&(*m).nskintexvert as *const i64 as *const (), 8, buffer_sz as i64, buffer, &mut ptrbuf);
        bufwrite(&(*m).nskinface as *const i64 as *const (), 8, buffer_sz as i64, buffer, &mut ptrbuf);
        bufwrite(&(*m).nskinbone as *const i64 as *const (), 8, buffer_sz as i64, buffer, &mut ptrbuf);
        bufwrite(&(*m).nskinbonevert as *const i64 as *const (), 8, buffer_sz as i64, buffer, &mut ptrbuf);
        bufwrite(&(*m).nhfield as *const i64 as *const (), 8, buffer_sz as i64, buffer, &mut ptrbuf);
        bufwrite(&(*m).nhfielddata as *const i64 as *const (), 8, buffer_sz as i64, buffer, &mut ptrbuf);
        bufwrite(&(*m).ntex as *const i64 as *const (), 8, buffer_sz as i64, buffer, &mut ptrbuf);
        bufwrite(&(*m).ntexdata as *const i64 as *const (), 8, buffer_sz as i64, buffer, &mut ptrbuf);
        bufwrite(&(*m).nmat as *const i64 as *const (), 8, buffer_sz as i64, buffer, &mut ptrbuf);
        bufwrite(&(*m).npair as *const i64 as *const (), 8, buffer_sz as i64, buffer, &mut ptrbuf);
        bufwrite(&(*m).nexclude as *const i64 as *const (), 8, buffer_sz as i64, buffer, &mut ptrbuf);
        bufwrite(&(*m).neq as *const i64 as *const (), 8, buffer_sz as i64, buffer, &mut ptrbuf);
        bufwrite(&(*m).ntendon as *const i64 as *const (), 8, buffer_sz as i64, buffer, &mut ptrbuf);
        bufwrite(&(*m).nJten as *const i64 as *const (), 8, buffer_sz as i64, buffer, &mut ptrbuf);
        bufwrite(&(*m).nwrap as *const i64 as *const (), 8, buffer_sz as i64, buffer, &mut ptrbuf);
        bufwrite(&(*m).nsensor as *const i64 as *const (), 8, buffer_sz as i64, buffer, &mut ptrbuf);
        bufwrite(&(*m).nnumeric as *const i64 as *const (), 8, buffer_sz as i64, buffer, &mut ptrbuf);
        bufwrite(&(*m).nnumericdata as *const i64 as *const (), 8, buffer_sz as i64, buffer, &mut ptrbuf);
        bufwrite(&(*m).ntext as *const i64 as *const (), 8, buffer_sz as i64, buffer, &mut ptrbuf);
        bufwrite(&(*m).ntextdata as *const i64 as *const (), 8, buffer_sz as i64, buffer, &mut ptrbuf);
        bufwrite(&(*m).ntuple as *const i64 as *const (), 8, buffer_sz as i64, buffer, &mut ptrbuf);
        bufwrite(&(*m).ntupledata as *const i64 as *const (), 8, buffer_sz as i64, buffer, &mut ptrbuf);
        bufwrite(&(*m).nkey as *const i64 as *const (), 8, buffer_sz as i64, buffer, &mut ptrbuf);
        bufwrite(&(*m).nmocap as *const i64 as *const (), 8, buffer_sz as i64, buffer, &mut ptrbuf);
        bufwrite(&(*m).nplugin as *const i64 as *const (), 8, buffer_sz as i64, buffer, &mut ptrbuf);
        bufwrite(&(*m).npluginattr as *const i64 as *const (), 8, buffer_sz as i64, buffer, &mut ptrbuf);
        bufwrite(&(*m).nuser_body as *const i64 as *const (), 8, buffer_sz as i64, buffer, &mut ptrbuf);
        bufwrite(&(*m).nuser_jnt as *const i64 as *const (), 8, buffer_sz as i64, buffer, &mut ptrbuf);
        bufwrite(&(*m).nuser_geom as *const i64 as *const (), 8, buffer_sz as i64, buffer, &mut ptrbuf);
        bufwrite(&(*m).nuser_site as *const i64 as *const (), 8, buffer_sz as i64, buffer, &mut ptrbuf);
        bufwrite(&(*m).nuser_cam as *const i64 as *const (), 8, buffer_sz as i64, buffer, &mut ptrbuf);
        bufwrite(&(*m).nuser_tendon as *const i64 as *const (), 8, buffer_sz as i64, buffer, &mut ptrbuf);
        bufwrite(&(*m).nuser_actuator as *const i64 as *const (), 8, buffer_sz as i64, buffer, &mut ptrbuf);
        bufwrite(&(*m).nuser_sensor as *const i64 as *const (), 8, buffer_sz as i64, buffer, &mut ptrbuf);
        bufwrite(&(*m).nnames as *const i64 as *const (), 8, buffer_sz as i64, buffer, &mut ptrbuf);
        bufwrite(&(*m).npaths as *const i64 as *const (), 8, buffer_sz as i64, buffer, &mut ptrbuf);
        bufwrite(&(*m).nnames_map as *const i64 as *const (), 8, buffer_sz as i64, buffer, &mut ptrbuf);
        bufwrite(&(*m).nJmom as *const i64 as *const (), 8, buffer_sz as i64, buffer, &mut ptrbuf);
        bufwrite(&(*m).ngravcomp as *const i64 as *const (), 8, buffer_sz as i64, buffer, &mut ptrbuf);
        bufwrite(&(*m).nemax as *const i64 as *const (), 8, buffer_sz as i64, buffer, &mut ptrbuf);
        bufwrite(&(*m).njmax as *const i64 as *const (), 8, buffer_sz as i64, buffer, &mut ptrbuf);
        bufwrite(&(*m).nconmax as *const i64 as *const (), 8, buffer_sz as i64, buffer, &mut ptrbuf);
        bufwrite(&(*m).nuserdata as *const i64 as *const (), 8, buffer_sz as i64, buffer, &mut ptrbuf);
        bufwrite(&(*m).nsensordata as *const i64 as *const (), 8, buffer_sz as i64, buffer, &mut ptrbuf);
        bufwrite(&(*m).npluginstate as *const i64 as *const (), 8, buffer_sz as i64, buffer, &mut ptrbuf);
        bufwrite(&(*m).nhistory as *const i64 as *const (), 8, buffer_sz as i64, buffer, &mut ptrbuf);
        bufwrite(&(*m).narena as *const i64 as *const (), 8, buffer_sz as i64, buffer, &mut ptrbuf);
        bufwrite(&(*m).nbuffer as *const i64 as *const (), 8, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite(&(*m).opt as *const mjOption as *const (), 304, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite(&(*m).vis as *const mjVisual as *const (), 632, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite(&(*m).stat as *const mjStatistic as *const (), 56, buffer_sz as i64, buffer, &mut ptrbuf);
            // MJMODEL_POINTERS: write each pointer array to buffer
            bufwrite((*m).qpos0 as *const (), (8_i64 * ((*m).nq) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).qpos_spring as *const (), (8_i64 * ((*m).nq) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).body_parentid as *const (), (4_i64 * ((*m).nbody) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).body_rootid as *const (), (4_i64 * ((*m).nbody) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).body_weldid as *const (), (4_i64 * ((*m).nbody) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).body_mocapid as *const (), (4_i64 * ((*m).nbody) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).body_jntnum as *const (), (4_i64 * ((*m).nbody) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).body_jntadr as *const (), (4_i64 * ((*m).nbody) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).body_dofnum as *const (), (4_i64 * ((*m).nbody) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).body_dofadr as *const (), (4_i64 * ((*m).nbody) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).body_treeid as *const (), (4_i64 * ((*m).nbody) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).body_geomnum as *const (), (4_i64 * ((*m).nbody) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).body_geomadr as *const (), (4_i64 * ((*m).nbody) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).body_simple as *const (), (1_i64 * ((*m).nbody) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).body_sameframe as *const (), (1_i64 * ((*m).nbody) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).body_pos as *const (), (8_i64 * ((*m).nbody) * (3_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).body_quat as *const (), (8_i64 * ((*m).nbody) * (4_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).body_ipos as *const (), (8_i64 * ((*m).nbody) * (3_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).body_iquat as *const (), (8_i64 * ((*m).nbody) * (4_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).body_mass as *const (), (8_i64 * ((*m).nbody) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).body_subtreemass as *const (), (8_i64 * ((*m).nbody) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).body_inertia as *const (), (8_i64 * ((*m).nbody) * (3_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).body_invweight0 as *const (), (8_i64 * ((*m).nbody) * (2_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).body_gravcomp as *const (), (8_i64 * ((*m).nbody) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).body_margin as *const (), (8_i64 * ((*m).nbody) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).body_user as *const (), (8_i64 * ((*m).nbody) * ((*m).nuser_body)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).body_plugin as *const (), (4_i64 * ((*m).nbody) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).body_contype as *const (), (4_i64 * ((*m).nbody) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).body_conaffinity as *const (), (4_i64 * ((*m).nbody) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).body_bvhadr as *const (), (4_i64 * ((*m).nbody) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).body_bvhnum as *const (), (4_i64 * ((*m).nbody) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).bvh_depth as *const (), (4_i64 * ((*m).nbvh) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).bvh_child as *const (), (4_i64 * ((*m).nbvh) * (2_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).bvh_nodeid as *const (), (4_i64 * ((*m).nbvh) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).bvh_aabb as *const (), (8_i64 * ((*m).nbvhstatic) * (6_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).oct_depth as *const (), (4_i64 * ((*m).noct) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).oct_child as *const (), (4_i64 * ((*m).noct) * (8_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).oct_aabb as *const (), (8_i64 * ((*m).noct) * (6_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).oct_coeff as *const (), (8_i64 * ((*m).noct) * (8_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).jnt_type as *const (), (4_i64 * ((*m).njnt) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).jnt_qposadr as *const (), (4_i64 * ((*m).njnt) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).jnt_dofadr as *const (), (4_i64 * ((*m).njnt) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).jnt_bodyid as *const (), (4_i64 * ((*m).njnt) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).jnt_actuatorid as *const (), (4_i64 * ((*m).njnt) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).jnt_group as *const (), (4_i64 * ((*m).njnt) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).jnt_limited as *const (), (1_i64 * ((*m).njnt) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).jnt_actfrclimited as *const (), (1_i64 * ((*m).njnt) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).jnt_actgravcomp as *const (), (1_i64 * ((*m).njnt) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).jnt_solref as *const (), (8_i64 * ((*m).njnt) * (2_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).jnt_solimp as *const (), (8_i64 * ((*m).njnt) * (5_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).jnt_pos as *const (), (8_i64 * ((*m).njnt) * (3_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).jnt_axis as *const (), (8_i64 * ((*m).njnt) * (3_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).jnt_stiffness as *const (), (8_i64 * ((*m).njnt) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).jnt_stiffnesspoly as *const (), (8_i64 * ((*m).njnt) * (2_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).jnt_range as *const (), (8_i64 * ((*m).njnt) * (2_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).jnt_actfrcrange as *const (), (8_i64 * ((*m).njnt) * (2_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).jnt_margin as *const (), (8_i64 * ((*m).njnt) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).jnt_user as *const (), (8_i64 * ((*m).njnt) * ((*m).nuser_jnt)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).dof_bodyid as *const (), (4_i64 * ((*m).nv) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).dof_jntid as *const (), (4_i64 * ((*m).nv) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).dof_parentid as *const (), (4_i64 * ((*m).nv) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).dof_treeid as *const (), (4_i64 * ((*m).nv) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).dof_Madr as *const (), (4_i64 * ((*m).nv) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).dof_simplenum as *const (), (4_i64 * ((*m).nv) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).dof_solref as *const (), (8_i64 * ((*m).nv) * (2_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).dof_solimp as *const (), (8_i64 * ((*m).nv) * (5_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).dof_frictionloss as *const (), (8_i64 * ((*m).nv) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).dof_armature as *const (), (8_i64 * ((*m).nv) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).dof_damping as *const (), (8_i64 * ((*m).nv) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).dof_dampingpoly as *const (), (8_i64 * ((*m).nv) * (2_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).dof_invweight0 as *const (), (8_i64 * ((*m).nv) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).dof_M0 as *const (), (8_i64 * ((*m).nv) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).dof_length as *const (), (8_i64 * ((*m).nv) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).tree_bodyadr as *const (), (4_i64 * ((*m).ntree) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).tree_bodynum as *const (), (4_i64 * ((*m).ntree) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).tree_dofadr as *const (), (4_i64 * ((*m).ntree) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).tree_dofnum as *const (), (4_i64 * ((*m).ntree) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).tree_sleep_policy as *const (), (4_i64 * ((*m).ntree) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).geom_type as *const (), (4_i64 * ((*m).ngeom) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).geom_contype as *const (), (4_i64 * ((*m).ngeom) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).geom_conaffinity as *const (), (4_i64 * ((*m).ngeom) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).geom_condim as *const (), (4_i64 * ((*m).ngeom) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).geom_bodyid as *const (), (4_i64 * ((*m).ngeom) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).geom_dataid as *const (), (4_i64 * ((*m).ngeom) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).geom_matid as *const (), (4_i64 * ((*m).ngeom) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).geom_group as *const (), (4_i64 * ((*m).ngeom) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).geom_priority as *const (), (4_i64 * ((*m).ngeom) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).geom_plugin as *const (), (4_i64 * ((*m).ngeom) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).geom_sameframe as *const (), (1_i64 * ((*m).ngeom) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).geom_solmix as *const (), (8_i64 * ((*m).ngeom) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).geom_solref as *const (), (8_i64 * ((*m).ngeom) * (2_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).geom_solimp as *const (), (8_i64 * ((*m).ngeom) * (5_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).geom_size as *const (), (8_i64 * ((*m).ngeom) * (3_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).geom_aabb as *const (), (8_i64 * ((*m).ngeom) * (6_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).geom_rbound as *const (), (8_i64 * ((*m).ngeom) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).geom_pos as *const (), (8_i64 * ((*m).ngeom) * (3_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).geom_quat as *const (), (8_i64 * ((*m).ngeom) * (4_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).geom_friction as *const (), (8_i64 * ((*m).ngeom) * (3_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).geom_margin as *const (), (8_i64 * ((*m).ngeom) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).geom_gap as *const (), (8_i64 * ((*m).ngeom) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).geom_fluid as *const (), (8_i64 * ((*m).ngeom) * (12_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).geom_user as *const (), (8_i64 * ((*m).ngeom) * ((*m).nuser_geom)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).geom_rgba as *const (), (4_i64 * ((*m).ngeom) * (4_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).site_type as *const (), (4_i64 * ((*m).nsite) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).site_bodyid as *const (), (4_i64 * ((*m).nsite) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).site_matid as *const (), (4_i64 * ((*m).nsite) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).site_group as *const (), (4_i64 * ((*m).nsite) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).site_sameframe as *const (), (1_i64 * ((*m).nsite) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).site_size as *const (), (8_i64 * ((*m).nsite) * (3_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).site_pos as *const (), (8_i64 * ((*m).nsite) * (3_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).site_quat as *const (), (8_i64 * ((*m).nsite) * (4_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).site_user as *const (), (8_i64 * ((*m).nsite) * ((*m).nuser_site)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).site_rgba as *const (), (4_i64 * ((*m).nsite) * (4_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).cam_mode as *const (), (4_i64 * ((*m).ncam) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).cam_bodyid as *const (), (4_i64 * ((*m).ncam) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).cam_targetbodyid as *const (), (4_i64 * ((*m).ncam) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).cam_pos as *const (), (8_i64 * ((*m).ncam) * (3_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).cam_quat as *const (), (8_i64 * ((*m).ncam) * (4_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).cam_poscom0 as *const (), (8_i64 * ((*m).ncam) * (3_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).cam_pos0 as *const (), (8_i64 * ((*m).ncam) * (3_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).cam_mat0 as *const (), (8_i64 * ((*m).ncam) * (9_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).cam_projection as *const (), (4_i64 * ((*m).ncam) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).cam_fovy as *const (), (8_i64 * ((*m).ncam) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).cam_ipd as *const (), (8_i64 * ((*m).ncam) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).cam_resolution as *const (), (4_i64 * ((*m).ncam) * (2_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).cam_output as *const (), (4_i64 * ((*m).ncam) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).cam_sensorsize as *const (), (4_i64 * ((*m).ncam) * (2_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).cam_intrinsic as *const (), (4_i64 * ((*m).ncam) * (4_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).cam_user as *const (), (8_i64 * ((*m).ncam) * ((*m).nuser_cam)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).light_mode as *const (), (4_i64 * ((*m).nlight) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).light_bodyid as *const (), (4_i64 * ((*m).nlight) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).light_targetbodyid as *const (), (4_i64 * ((*m).nlight) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).light_type as *const (), (4_i64 * ((*m).nlight) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).light_texid as *const (), (4_i64 * ((*m).nlight) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).light_castshadow as *const (), (1_i64 * ((*m).nlight) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).light_bulbradius as *const (), (4_i64 * ((*m).nlight) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).light_intensity as *const (), (4_i64 * ((*m).nlight) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).light_range as *const (), (4_i64 * ((*m).nlight) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).light_active as *const (), (1_i64 * ((*m).nlight) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).light_pos as *const (), (8_i64 * ((*m).nlight) * (3_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).light_dir as *const (), (8_i64 * ((*m).nlight) * (3_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).light_poscom0 as *const (), (8_i64 * ((*m).nlight) * (3_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).light_pos0 as *const (), (8_i64 * ((*m).nlight) * (3_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).light_dir0 as *const (), (8_i64 * ((*m).nlight) * (3_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).light_attenuation as *const (), (4_i64 * ((*m).nlight) * (3_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).light_cutoff as *const (), (4_i64 * ((*m).nlight) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).light_exponent as *const (), (4_i64 * ((*m).nlight) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).light_ambient as *const (), (4_i64 * ((*m).nlight) * (3_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).light_diffuse as *const (), (4_i64 * ((*m).nlight) * (3_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).light_specular as *const (), (4_i64 * ((*m).nlight) * (3_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).flex_contype as *const (), (4_i64 * ((*m).nflex) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).flex_conaffinity as *const (), (4_i64 * ((*m).nflex) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).flex_condim as *const (), (4_i64 * ((*m).nflex) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).flex_priority as *const (), (4_i64 * ((*m).nflex) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).flex_solmix as *const (), (8_i64 * ((*m).nflex) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).flex_solref as *const (), (8_i64 * ((*m).nflex) * (2_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).flex_solimp as *const (), (8_i64 * ((*m).nflex) * (5_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).flex_friction as *const (), (8_i64 * ((*m).nflex) * (3_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).flex_margin as *const (), (8_i64 * ((*m).nflex) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).flex_gap as *const (), (8_i64 * ((*m).nflex) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).flex_internal as *const (), (1_i64 * ((*m).nflex) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).flex_selfcollide as *const (), (4_i64 * ((*m).nflex) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).flex_activelayers as *const (), (4_i64 * ((*m).nflex) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).flex_passive as *const (), (4_i64 * ((*m).nflex) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).flex_dim as *const (), (4_i64 * ((*m).nflex) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).flex_matid as *const (), (4_i64 * ((*m).nflex) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).flex_group as *const (), (4_i64 * ((*m).nflex) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).flex_interp as *const (), (4_i64 * ((*m).nflex) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).flex_cellnum as *const (), (4_i64 * ((*m).nflex) * (3_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).flex_nodeadr as *const (), (4_i64 * ((*m).nflex) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).flex_nodenum as *const (), (4_i64 * ((*m).nflex) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).flex_vertadr as *const (), (4_i64 * ((*m).nflex) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).flex_vertnum as *const (), (4_i64 * ((*m).nflex) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).flex_edgeadr as *const (), (4_i64 * ((*m).nflex) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).flex_edgenum as *const (), (4_i64 * ((*m).nflex) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).flex_elemadr as *const (), (4_i64 * ((*m).nflex) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).flex_elemnum as *const (), (4_i64 * ((*m).nflex) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).flex_elemdataadr as *const (), (4_i64 * ((*m).nflex) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).flex_stiffnessadr as *const (), (4_i64 * ((*m).nflex) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).flex_elemedgeadr as *const (), (4_i64 * ((*m).nflex) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).flex_bendingadr as *const (), (4_i64 * ((*m).nflex) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).flex_shellnum as *const (), (4_i64 * ((*m).nflex) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).flex_shelldataadr as *const (), (4_i64 * ((*m).nflex) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).flex_evpairadr as *const (), (4_i64 * ((*m).nflex) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).flex_evpairnum as *const (), (4_i64 * ((*m).nflex) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).flex_texcoordadr as *const (), (4_i64 * ((*m).nflex) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).flex_nodebodyid as *const (), (4_i64 * ((*m).nflexnode) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).flex_vertbodyid as *const (), (4_i64 * ((*m).nflexvert) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).flex_vertedgeadr as *const (), (4_i64 * ((*m).nflexvert) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).flex_vertedgenum as *const (), (4_i64 * ((*m).nflexvert) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).flex_vertedge as *const (), (4_i64 * ((*m).nflexedge) * (2_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).flex_edge as *const (), (4_i64 * ((*m).nflexedge) * (2_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).flex_edgeflap as *const (), (4_i64 * ((*m).nflexedge) * (2_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).flex_elem as *const (), (4_i64 * ((*m).nflexelemdata) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).flex_elemtexcoord as *const (), (4_i64 * ((*m).nflexelemdata) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).flex_elemedge as *const (), (4_i64 * ((*m).nflexelemedge) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).flex_elemlayer as *const (), (4_i64 * ((*m).nflexelem) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).flex_shell as *const (), (4_i64 * ((*m).nflexshelldata) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).flex_evpair as *const (), (4_i64 * ((*m).nflexevpair) * (2_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).flex_vert as *const (), (8_i64 * ((*m).nflexvert) * (3_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).flex_vert0 as *const (), (8_i64 * ((*m).nflexvert) * (3_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).flex_vertmetric as *const (), (8_i64 * ((*m).nflexvert) * (4_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).flex_node as *const (), (8_i64 * ((*m).nflexnode) * (3_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).flex_node0 as *const (), (8_i64 * ((*m).nflexnode) * (3_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).flexedge_length0 as *const (), (8_i64 * ((*m).nflexedge) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).flexedge_invweight0 as *const (), (8_i64 * ((*m).nflexedge) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).flex_radius as *const (), (8_i64 * ((*m).nflex) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).flex_size as *const (), (8_i64 * ((*m).nflex) * (3_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).flex_stiffness as *const (), (8_i64 * ((*m).nflexstiffness) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).flex_bending as *const (), (8_i64 * ((*m).nflexbending) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).flex_damping as *const (), (8_i64 * ((*m).nflex) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).flex_edgestiffness as *const (), (8_i64 * ((*m).nflex) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).flex_edgedamping as *const (), (8_i64 * ((*m).nflex) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).flex_edgeequality as *const (), (4_i64 * ((*m).nflex) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).flex_rigid as *const (), (1_i64 * ((*m).nflex) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).flexedge_rigid as *const (), (1_i64 * ((*m).nflexedge) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).flex_centered as *const (), (1_i64 * ((*m).nflex) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).flex_flatskin as *const (), (1_i64 * ((*m).nflex) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).flex_bvhadr as *const (), (4_i64 * ((*m).nflex) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).flex_bvhnum as *const (), (4_i64 * ((*m).nflex) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).flexedge_J_rownnz as *const (), (4_i64 * ((*m).nflexedge) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).flexedge_J_rowadr as *const (), (4_i64 * ((*m).nflexedge) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).flexedge_J_colind as *const (), (4_i64 * ((*m).nJfe) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).flexvert_J_rownnz as *const (), (4_i64 * ((*m).nflexvert) * (2_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).flexvert_J_rowadr as *const (), (4_i64 * ((*m).nflexvert) * (2_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).flexvert_J_colind as *const (), (4_i64 * ((*m).nJfv) * (2_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).flex_rgba as *const (), (4_i64 * ((*m).nflex) * (4_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).flex_texcoord as *const (), (4_i64 * ((*m).nflextexcoord) * (2_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).mesh_vertadr as *const (), (4_i64 * ((*m).nmesh) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).mesh_vertnum as *const (), (4_i64 * ((*m).nmesh) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).mesh_faceadr as *const (), (4_i64 * ((*m).nmesh) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).mesh_facenum as *const (), (4_i64 * ((*m).nmesh) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).mesh_bvhadr as *const (), (4_i64 * ((*m).nmesh) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).mesh_bvhnum as *const (), (4_i64 * ((*m).nmesh) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).mesh_octadr as *const (), (4_i64 * ((*m).nmesh) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).mesh_octnum as *const (), (4_i64 * ((*m).nmesh) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).mesh_normaladr as *const (), (4_i64 * ((*m).nmesh) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).mesh_normalnum as *const (), (4_i64 * ((*m).nmesh) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).mesh_texcoordadr as *const (), (4_i64 * ((*m).nmesh) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).mesh_texcoordnum as *const (), (4_i64 * ((*m).nmesh) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).mesh_graphadr as *const (), (4_i64 * ((*m).nmesh) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).mesh_vert as *const (), (4_i64 * ((*m).nmeshvert) * (3_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).mesh_normal as *const (), (4_i64 * ((*m).nmeshnormal) * (3_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).mesh_texcoord as *const (), (4_i64 * ((*m).nmeshtexcoord) * (2_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).mesh_face as *const (), (4_i64 * ((*m).nmeshface) * (3_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).mesh_facenormal as *const (), (4_i64 * ((*m).nmeshface) * (3_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).mesh_facetexcoord as *const (), (4_i64 * ((*m).nmeshface) * (3_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).mesh_graph as *const (), (4_i64 * ((*m).nmeshgraph) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).mesh_scale as *const (), (8_i64 * ((*m).nmesh) * (3_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).mesh_pos as *const (), (8_i64 * ((*m).nmesh) * (3_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).mesh_quat as *const (), (8_i64 * ((*m).nmesh) * (4_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).mesh_pathadr as *const (), (4_i64 * ((*m).nmesh) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).mesh_polynum as *const (), (4_i64 * ((*m).nmesh) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).mesh_polyadr as *const (), (4_i64 * ((*m).nmesh) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).mesh_polynormal as *const (), (8_i64 * ((*m).nmeshpoly) * (3_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).mesh_polyvertadr as *const (), (4_i64 * ((*m).nmeshpoly) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).mesh_polyvertnum as *const (), (4_i64 * ((*m).nmeshpoly) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).mesh_polyvert as *const (), (4_i64 * ((*m).nmeshpolyvert) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).mesh_polymapadr as *const (), (4_i64 * ((*m).nmeshvert) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).mesh_polymapnum as *const (), (4_i64 * ((*m).nmeshvert) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).mesh_polymap as *const (), (4_i64 * ((*m).nmeshpolymap) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).skin_matid as *const (), (4_i64 * ((*m).nskin) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).skin_group as *const (), (4_i64 * ((*m).nskin) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).skin_rgba as *const (), (4_i64 * ((*m).nskin) * (4_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).skin_inflate as *const (), (4_i64 * ((*m).nskin) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).skin_vertadr as *const (), (4_i64 * ((*m).nskin) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).skin_vertnum as *const (), (4_i64 * ((*m).nskin) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).skin_texcoordadr as *const (), (4_i64 * ((*m).nskin) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).skin_faceadr as *const (), (4_i64 * ((*m).nskin) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).skin_facenum as *const (), (4_i64 * ((*m).nskin) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).skin_boneadr as *const (), (4_i64 * ((*m).nskin) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).skin_bonenum as *const (), (4_i64 * ((*m).nskin) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).skin_vert as *const (), (4_i64 * ((*m).nskinvert) * (3_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).skin_texcoord as *const (), (4_i64 * ((*m).nskintexvert) * (2_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).skin_face as *const (), (4_i64 * ((*m).nskinface) * (3_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).skin_bonevertadr as *const (), (4_i64 * ((*m).nskinbone) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).skin_bonevertnum as *const (), (4_i64 * ((*m).nskinbone) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).skin_bonebindpos as *const (), (4_i64 * ((*m).nskinbone) * (3_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).skin_bonebindquat as *const (), (4_i64 * ((*m).nskinbone) * (4_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).skin_bonebodyid as *const (), (4_i64 * ((*m).nskinbone) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).skin_bonevertid as *const (), (4_i64 * ((*m).nskinbonevert) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).skin_bonevertweight as *const (), (4_i64 * ((*m).nskinbonevert) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).skin_pathadr as *const (), (4_i64 * ((*m).nskin) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).hfield_size as *const (), (8_i64 * ((*m).nhfield) * (4_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).hfield_nrow as *const (), (4_i64 * ((*m).nhfield) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).hfield_ncol as *const (), (4_i64 * ((*m).nhfield) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).hfield_adr as *const (), (4_i64 * ((*m).nhfield) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).hfield_data as *const (), (4_i64 * ((*m).nhfielddata) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).hfield_pathadr as *const (), (4_i64 * ((*m).nhfield) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).tex_type as *const (), (4_i64 * ((*m).ntex) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).tex_colorspace as *const (), (4_i64 * ((*m).ntex) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).tex_height as *const (), (4_i64 * ((*m).ntex) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).tex_width as *const (), (4_i64 * ((*m).ntex) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).tex_nchannel as *const (), (4_i64 * ((*m).ntex) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).tex_adr as *const (), (8_i64 * ((*m).ntex) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).tex_data as *const (), (1_i64 * ((*m).ntexdata) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).tex_pathadr as *const (), (4_i64 * ((*m).ntex) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).mat_texid as *const (), (4_i64 * ((*m).nmat) * (10_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).mat_texuniform as *const (), (1_i64 * ((*m).nmat) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).mat_texrepeat as *const (), (4_i64 * ((*m).nmat) * (2_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).mat_emission as *const (), (4_i64 * ((*m).nmat) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).mat_specular as *const (), (4_i64 * ((*m).nmat) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).mat_shininess as *const (), (4_i64 * ((*m).nmat) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).mat_reflectance as *const (), (4_i64 * ((*m).nmat) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).mat_metallic as *const (), (4_i64 * ((*m).nmat) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).mat_roughness as *const (), (4_i64 * ((*m).nmat) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).mat_rgba as *const (), (4_i64 * ((*m).nmat) * (4_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).pair_dim as *const (), (4_i64 * ((*m).npair) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).pair_geom1 as *const (), (4_i64 * ((*m).npair) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).pair_geom2 as *const (), (4_i64 * ((*m).npair) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).pair_signature as *const (), (4_i64 * ((*m).npair) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).pair_solref as *const (), (8_i64 * ((*m).npair) * (2_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).pair_solreffriction as *const (), (8_i64 * ((*m).npair) * (2_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).pair_solimp as *const (), (8_i64 * ((*m).npair) * (5_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).pair_margin as *const (), (8_i64 * ((*m).npair) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).pair_gap as *const (), (8_i64 * ((*m).npair) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).pair_friction as *const (), (8_i64 * ((*m).npair) * (5_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).exclude_signature as *const (), (4_i64 * ((*m).nexclude) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).eq_type as *const (), (4_i64 * ((*m).neq) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).eq_obj1id as *const (), (4_i64 * ((*m).neq) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).eq_obj2id as *const (), (4_i64 * ((*m).neq) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).eq_objtype as *const (), (4_i64 * ((*m).neq) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).eq_active0 as *const (), (1_i64 * ((*m).neq) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).eq_solref as *const (), (8_i64 * ((*m).neq) * (2_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).eq_solimp as *const (), (8_i64 * ((*m).neq) * (5_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).eq_data as *const (), (8_i64 * ((*m).neq) * (11_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).tendon_adr as *const (), (4_i64 * ((*m).ntendon) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).tendon_num as *const (), (4_i64 * ((*m).ntendon) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).tendon_matid as *const (), (4_i64 * ((*m).ntendon) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).tendon_actuatorid as *const (), (4_i64 * ((*m).ntendon) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).tendon_group as *const (), (4_i64 * ((*m).ntendon) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).tendon_treenum as *const (), (4_i64 * ((*m).ntendon) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).tendon_treeid as *const (), (4_i64 * ((*m).ntendon) * (2_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).ten_J_rownnz as *const (), (4_i64 * ((*m).ntendon) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).ten_J_rowadr as *const (), (4_i64 * ((*m).ntendon) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).ten_J_colind as *const (), (4_i64 * ((*m).nJten) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).tendon_limited as *const (), (1_i64 * ((*m).ntendon) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).tendon_actfrclimited as *const (), (1_i64 * ((*m).ntendon) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).tendon_width as *const (), (8_i64 * ((*m).ntendon) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).tendon_solref_lim as *const (), (8_i64 * ((*m).ntendon) * (2_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).tendon_solimp_lim as *const (), (8_i64 * ((*m).ntendon) * (5_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).tendon_solref_fri as *const (), (8_i64 * ((*m).ntendon) * (2_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).tendon_solimp_fri as *const (), (8_i64 * ((*m).ntendon) * (5_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).tendon_range as *const (), (8_i64 * ((*m).ntendon) * (2_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).tendon_actfrcrange as *const (), (8_i64 * ((*m).ntendon) * (2_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).tendon_margin as *const (), (8_i64 * ((*m).ntendon) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).tendon_stiffness as *const (), (8_i64 * ((*m).ntendon) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).tendon_stiffnesspoly as *const (), (8_i64 * ((*m).ntendon) * (2_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).tendon_damping as *const (), (8_i64 * ((*m).ntendon) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).tendon_dampingpoly as *const (), (8_i64 * ((*m).ntendon) * (2_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).tendon_armature as *const (), (8_i64 * ((*m).ntendon) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).tendon_frictionloss as *const (), (8_i64 * ((*m).ntendon) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).tendon_lengthspring as *const (), (8_i64 * ((*m).ntendon) * (2_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).tendon_length0 as *const (), (8_i64 * ((*m).ntendon) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).tendon_invweight0 as *const (), (8_i64 * ((*m).ntendon) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).tendon_user as *const (), (8_i64 * ((*m).ntendon) * ((*m).nuser_tendon)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).tendon_rgba as *const (), (4_i64 * ((*m).ntendon) * (4_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).wrap_type as *const (), (4_i64 * ((*m).nwrap) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).wrap_objid as *const (), (4_i64 * ((*m).nwrap) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).wrap_prm as *const (), (8_i64 * ((*m).nwrap) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).actuator_trntype as *const (), (4_i64 * ((*m).nu) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).actuator_dyntype as *const (), (4_i64 * ((*m).nu) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).actuator_gaintype as *const (), (4_i64 * ((*m).nu) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).actuator_biastype as *const (), (4_i64 * ((*m).nu) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).actuator_trnid as *const (), (4_i64 * ((*m).nu) * (2_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).actuator_damping as *const (), (8_i64 * ((*m).nu) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).actuator_dampingpoly as *const (), (8_i64 * ((*m).nu) * (2_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).actuator_armature as *const (), (8_i64 * ((*m).nu) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).actuator_actadr as *const (), (4_i64 * ((*m).nu) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).actuator_actnum as *const (), (4_i64 * ((*m).nu) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).actuator_group as *const (), (4_i64 * ((*m).nu) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).actuator_history as *const (), (4_i64 * ((*m).nu) * (2_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).actuator_historyadr as *const (), (4_i64 * ((*m).nu) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).actuator_delay as *const (), (8_i64 * ((*m).nu) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).actuator_ctrllimited as *const (), (1_i64 * ((*m).nu) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).actuator_forcelimited as *const (), (1_i64 * ((*m).nu) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).actuator_actlimited as *const (), (1_i64 * ((*m).nu) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).actuator_dynprm as *const (), (8_i64 * ((*m).nu) * (10_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).actuator_gainprm as *const (), (8_i64 * ((*m).nu) * (10_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).actuator_biasprm as *const (), (8_i64 * ((*m).nu) * (10_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).actuator_actearly as *const (), (1_i64 * ((*m).nu) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).actuator_ctrlrange as *const (), (8_i64 * ((*m).nu) * (2_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).actuator_forcerange as *const (), (8_i64 * ((*m).nu) * (2_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).actuator_actrange as *const (), (8_i64 * ((*m).nu) * (2_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).actuator_gear as *const (), (8_i64 * ((*m).nu) * (6_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).actuator_cranklength as *const (), (8_i64 * ((*m).nu) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).actuator_acc0 as *const (), (8_i64 * ((*m).nu) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).actuator_length0 as *const (), (8_i64 * ((*m).nu) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).actuator_lengthrange as *const (), (8_i64 * ((*m).nu) * (2_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).actuator_user as *const (), (8_i64 * ((*m).nu) * ((*m).nuser_actuator)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).actuator_plugin as *const (), (4_i64 * ((*m).nu) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).sensor_type as *const (), (4_i64 * ((*m).nsensor) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).sensor_datatype as *const (), (4_i64 * ((*m).nsensor) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).sensor_needstage as *const (), (4_i64 * ((*m).nsensor) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).sensor_objtype as *const (), (4_i64 * ((*m).nsensor) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).sensor_objid as *const (), (4_i64 * ((*m).nsensor) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).sensor_reftype as *const (), (4_i64 * ((*m).nsensor) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).sensor_refid as *const (), (4_i64 * ((*m).nsensor) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).sensor_intprm as *const (), (4_i64 * ((*m).nsensor) * (3_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).sensor_dim as *const (), (4_i64 * ((*m).nsensor) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).sensor_adr as *const (), (4_i64 * ((*m).nsensor) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).sensor_cutoff as *const (), (8_i64 * ((*m).nsensor) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).sensor_noise as *const (), (8_i64 * ((*m).nsensor) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).sensor_history as *const (), (4_i64 * ((*m).nsensor) * (2_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).sensor_historyadr as *const (), (4_i64 * ((*m).nsensor) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).sensor_delay as *const (), (8_i64 * ((*m).nsensor) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).sensor_interval as *const (), (8_i64 * ((*m).nsensor) * (2_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).sensor_user as *const (), (8_i64 * ((*m).nsensor) * ((*m).nuser_sensor)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).sensor_plugin as *const (), (4_i64 * ((*m).nsensor) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).plugin as *const (), (4_i64 * ((*m).nplugin) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).plugin_stateadr as *const (), (4_i64 * ((*m).nplugin) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).plugin_statenum as *const (), (4_i64 * ((*m).nplugin) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).plugin_attr as *const (), (1_i64 * ((*m).npluginattr) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).plugin_attradr as *const (), (4_i64 * ((*m).nplugin) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).numeric_adr as *const (), (4_i64 * ((*m).nnumeric) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).numeric_size as *const (), (4_i64 * ((*m).nnumeric) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).numeric_data as *const (), (8_i64 * ((*m).nnumericdata) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).text_adr as *const (), (4_i64 * ((*m).ntext) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).text_size as *const (), (4_i64 * ((*m).ntext) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).text_data as *const (), (1_i64 * ((*m).ntextdata) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).tuple_adr as *const (), (4_i64 * ((*m).ntuple) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).tuple_size as *const (), (4_i64 * ((*m).ntuple) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).tuple_objtype as *const (), (4_i64 * ((*m).ntupledata) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).tuple_objid as *const (), (4_i64 * ((*m).ntupledata) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).tuple_objprm as *const (), (8_i64 * ((*m).ntupledata) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).key_time as *const (), (8_i64 * ((*m).nkey) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).key_qpos as *const (), (8_i64 * ((*m).nkey) * ((*m).nq)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).key_qvel as *const (), (8_i64 * ((*m).nkey) * ((*m).nv)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).key_act as *const (), (8_i64 * ((*m).nkey) * ((*m).na)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).key_mpos as *const (), (8_i64 * ((*m).nkey) * ((*m).nmocap * 3)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).key_mquat as *const (), (8_i64 * ((*m).nkey) * ((*m).nmocap * 4)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).key_ctrl as *const (), (8_i64 * ((*m).nkey) * ((*m).nu)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).name_bodyadr as *const (), (4_i64 * ((*m).nbody) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).name_jntadr as *const (), (4_i64 * ((*m).njnt) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).name_geomadr as *const (), (4_i64 * ((*m).ngeom) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).name_siteadr as *const (), (4_i64 * ((*m).nsite) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).name_camadr as *const (), (4_i64 * ((*m).ncam) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).name_lightadr as *const (), (4_i64 * ((*m).nlight) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).name_flexadr as *const (), (4_i64 * ((*m).nflex) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).name_meshadr as *const (), (4_i64 * ((*m).nmesh) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).name_skinadr as *const (), (4_i64 * ((*m).nskin) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).name_hfieldadr as *const (), (4_i64 * ((*m).nhfield) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).name_texadr as *const (), (4_i64 * ((*m).ntex) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).name_matadr as *const (), (4_i64 * ((*m).nmat) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).name_pairadr as *const (), (4_i64 * ((*m).npair) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).name_excludeadr as *const (), (4_i64 * ((*m).nexclude) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).name_eqadr as *const (), (4_i64 * ((*m).neq) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).name_tendonadr as *const (), (4_i64 * ((*m).ntendon) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).name_actuatoradr as *const (), (4_i64 * ((*m).nu) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).name_sensoradr as *const (), (4_i64 * ((*m).nsensor) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).name_numericadr as *const (), (4_i64 * ((*m).nnumeric) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).name_textadr as *const (), (4_i64 * ((*m).ntext) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).name_tupleadr as *const (), (4_i64 * ((*m).ntuple) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).name_keyadr as *const (), (4_i64 * ((*m).nkey) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).name_pluginadr as *const (), (4_i64 * ((*m).nplugin) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).names as *const (), (1_i64 * ((*m).nnames) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).names_map as *const (), (4_i64 * ((*m).nnames_map) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).paths as *const (), (1_i64 * ((*m).npaths) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).B_rownnz as *const (), (4_i64 * ((*m).nbody) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).B_rowadr as *const (), (4_i64 * ((*m).nbody) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).B_colind as *const (), (4_i64 * ((*m).nB) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).M_rownnz as *const (), (4_i64 * ((*m).nv) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).M_rowadr as *const (), (4_i64 * ((*m).nv) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).M_colind as *const (), (4_i64 * ((*m).nC) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).mapM2M as *const (), (4_i64 * ((*m).nC) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).D_rownnz as *const (), (4_i64 * ((*m).nv) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).D_rowadr as *const (), (4_i64 * ((*m).nv) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).D_diag as *const (), (4_i64 * ((*m).nv) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).D_colind as *const (), (4_i64 * ((*m).nD) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).mapM2D as *const (), (4_i64 * ((*m).nD) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
            bufwrite((*m).mapD2M as *const (), (4_i64 * ((*m).nC) * (1_i64)) as i32, buffer_sz as i64, buffer, &mut ptrbuf);
        }

        if !fp.is_null() {
            fclose(fp);
        }
    }
}

/// C: mj_loadModelBuffer (engine/engine_io.h:78)
/// Calls: bufread, getnptr, getnsize, mj_deleteModel, mj_makeModel, mj_validateReferences, mj_version, mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn mj_load_model_buffer(buffer: *const (), buffer_sz: i32) -> *mut mjModel {
    todo!() // mj_loadModelBuffer
}

/// C: mj_deleteModel (engine/engine_io.h:81)
/// Calls: freeModelBuffers, mju_free
#[allow(unused_variables, non_snake_case)]
pub fn mj_delete_model(m: *mut mjModel) {
    if !m.is_null() {
        free_model_buffers(m);
        crate::engine::engine_util_errmem::mju_free(m as *mut ());
    }
}

/// C: mj_sizeModel (engine/engine_io.h:84)
/// Calls: getnsize
#[allow(unused_variables, non_snake_case)]
pub fn mj_size_model(m: *const mjModel) -> i64 {
    const NHEADER: i64 = 5;

    // SAFETY: m is a valid mjModel pointer (caller contract)
    unsafe {
        let mut size: i64 =
            4 * NHEADER               // sizeof(int)*NHEADER
            + 8 * getnsize() as i64   // sizeof(mjtSize)*getnsize()
            + 304                     // sizeof(mjOption)
            + 632                     // sizeof(mjVisual)
            + 56;                     // sizeof(mjStatistic)

        // MJMODEL_POINTERS expansion (471 fields): size += sizeof(type) * m->nr * nc
        size += 8_i64 * ((*m).nq) * (1_i64);
        size += 8_i64 * ((*m).nq) * (1_i64);
        size += 4_i64 * ((*m).nbody) * (1_i64);
        size += 4_i64 * ((*m).nbody) * (1_i64);
        size += 4_i64 * ((*m).nbody) * (1_i64);
        size += 4_i64 * ((*m).nbody) * (1_i64);
        size += 4_i64 * ((*m).nbody) * (1_i64);
        size += 4_i64 * ((*m).nbody) * (1_i64);
        size += 4_i64 * ((*m).nbody) * (1_i64);
        size += 4_i64 * ((*m).nbody) * (1_i64);
        size += 4_i64 * ((*m).nbody) * (1_i64);
        size += 4_i64 * ((*m).nbody) * (1_i64);
        size += 4_i64 * ((*m).nbody) * (1_i64);
        size += 1_i64 * ((*m).nbody) * (1_i64);
        size += 1_i64 * ((*m).nbody) * (1_i64);
        size += 8_i64 * ((*m).nbody) * (3_i64);
        size += 8_i64 * ((*m).nbody) * (4_i64);
        size += 8_i64 * ((*m).nbody) * (3_i64);
        size += 8_i64 * ((*m).nbody) * (4_i64);
        size += 8_i64 * ((*m).nbody) * (1_i64);
        size += 8_i64 * ((*m).nbody) * (1_i64);
        size += 8_i64 * ((*m).nbody) * (3_i64);
        size += 8_i64 * ((*m).nbody) * (2_i64);
        size += 8_i64 * ((*m).nbody) * (1_i64);
        size += 8_i64 * ((*m).nbody) * (1_i64);
        size += 8_i64 * ((*m).nbody) * ((*m).nuser_body);
        size += 4_i64 * ((*m).nbody) * (1_i64);
        size += 4_i64 * ((*m).nbody) * (1_i64);
        size += 4_i64 * ((*m).nbody) * (1_i64);
        size += 4_i64 * ((*m).nbody) * (1_i64);
        size += 4_i64 * ((*m).nbody) * (1_i64);
        size += 4_i64 * ((*m).nbvh) * (1_i64);
        size += 4_i64 * ((*m).nbvh) * (2_i64);
        size += 4_i64 * ((*m).nbvh) * (1_i64);
        size += 8_i64 * ((*m).nbvhstatic) * (6_i64);
        size += 4_i64 * ((*m).noct) * (1_i64);
        size += 4_i64 * ((*m).noct) * (8_i64);
        size += 8_i64 * ((*m).noct) * (6_i64);
        size += 8_i64 * ((*m).noct) * (8_i64);
        size += 4_i64 * ((*m).njnt) * (1_i64);
        size += 4_i64 * ((*m).njnt) * (1_i64);
        size += 4_i64 * ((*m).njnt) * (1_i64);
        size += 4_i64 * ((*m).njnt) * (1_i64);
        size += 4_i64 * ((*m).njnt) * (1_i64);
        size += 4_i64 * ((*m).njnt) * (1_i64);
        size += 1_i64 * ((*m).njnt) * (1_i64);
        size += 1_i64 * ((*m).njnt) * (1_i64);
        size += 1_i64 * ((*m).njnt) * (1_i64);
        size += 8_i64 * ((*m).njnt) * (2_i64);
        size += 8_i64 * ((*m).njnt) * (5_i64);
        size += 8_i64 * ((*m).njnt) * (3_i64);
        size += 8_i64 * ((*m).njnt) * (3_i64);
        size += 8_i64 * ((*m).njnt) * (1_i64);
        size += 8_i64 * ((*m).njnt) * (2_i64);
        size += 8_i64 * ((*m).njnt) * (2_i64);
        size += 8_i64 * ((*m).njnt) * (2_i64);
        size += 8_i64 * ((*m).njnt) * (1_i64);
        size += 8_i64 * ((*m).njnt) * ((*m).nuser_jnt);
        size += 4_i64 * ((*m).nv) * (1_i64);
        size += 4_i64 * ((*m).nv) * (1_i64);
        size += 4_i64 * ((*m).nv) * (1_i64);
        size += 4_i64 * ((*m).nv) * (1_i64);
        size += 4_i64 * ((*m).nv) * (1_i64);
        size += 4_i64 * ((*m).nv) * (1_i64);
        size += 8_i64 * ((*m).nv) * (2_i64);
        size += 8_i64 * ((*m).nv) * (5_i64);
        size += 8_i64 * ((*m).nv) * (1_i64);
        size += 8_i64 * ((*m).nv) * (1_i64);
        size += 8_i64 * ((*m).nv) * (1_i64);
        size += 8_i64 * ((*m).nv) * (2_i64);
        size += 8_i64 * ((*m).nv) * (1_i64);
        size += 8_i64 * ((*m).nv) * (1_i64);
        size += 8_i64 * ((*m).nv) * (1_i64);
        size += 4_i64 * ((*m).ntree) * (1_i64);
        size += 4_i64 * ((*m).ntree) * (1_i64);
        size += 4_i64 * ((*m).ntree) * (1_i64);
        size += 4_i64 * ((*m).ntree) * (1_i64);
        size += 4_i64 * ((*m).ntree) * (1_i64);
        size += 4_i64 * ((*m).ngeom) * (1_i64);
        size += 4_i64 * ((*m).ngeom) * (1_i64);
        size += 4_i64 * ((*m).ngeom) * (1_i64);
        size += 4_i64 * ((*m).ngeom) * (1_i64);
        size += 4_i64 * ((*m).ngeom) * (1_i64);
        size += 4_i64 * ((*m).ngeom) * (1_i64);
        size += 4_i64 * ((*m).ngeom) * (1_i64);
        size += 4_i64 * ((*m).ngeom) * (1_i64);
        size += 4_i64 * ((*m).ngeom) * (1_i64);
        size += 4_i64 * ((*m).ngeom) * (1_i64);
        size += 1_i64 * ((*m).ngeom) * (1_i64);
        size += 8_i64 * ((*m).ngeom) * (1_i64);
        size += 8_i64 * ((*m).ngeom) * (2_i64);
        size += 8_i64 * ((*m).ngeom) * (5_i64);
        size += 8_i64 * ((*m).ngeom) * (3_i64);
        size += 8_i64 * ((*m).ngeom) * (6_i64);
        size += 8_i64 * ((*m).ngeom) * (1_i64);
        size += 8_i64 * ((*m).ngeom) * (3_i64);
        size += 8_i64 * ((*m).ngeom) * (4_i64);
        size += 8_i64 * ((*m).ngeom) * (3_i64);
        size += 8_i64 * ((*m).ngeom) * (1_i64);
        size += 8_i64 * ((*m).ngeom) * (1_i64);
        size += 8_i64 * ((*m).ngeom) * (12_i64);
        size += 8_i64 * ((*m).ngeom) * ((*m).nuser_geom);
        size += 4_i64 * ((*m).ngeom) * (4_i64);
        size += 4_i64 * ((*m).nsite) * (1_i64);
        size += 4_i64 * ((*m).nsite) * (1_i64);
        size += 4_i64 * ((*m).nsite) * (1_i64);
        size += 4_i64 * ((*m).nsite) * (1_i64);
        size += 1_i64 * ((*m).nsite) * (1_i64);
        size += 8_i64 * ((*m).nsite) * (3_i64);
        size += 8_i64 * ((*m).nsite) * (3_i64);
        size += 8_i64 * ((*m).nsite) * (4_i64);
        size += 8_i64 * ((*m).nsite) * ((*m).nuser_site);
        size += 4_i64 * ((*m).nsite) * (4_i64);
        size += 4_i64 * ((*m).ncam) * (1_i64);
        size += 4_i64 * ((*m).ncam) * (1_i64);
        size += 4_i64 * ((*m).ncam) * (1_i64);
        size += 8_i64 * ((*m).ncam) * (3_i64);
        size += 8_i64 * ((*m).ncam) * (4_i64);
        size += 8_i64 * ((*m).ncam) * (3_i64);
        size += 8_i64 * ((*m).ncam) * (3_i64);
        size += 8_i64 * ((*m).ncam) * (9_i64);
        size += 4_i64 * ((*m).ncam) * (1_i64);
        size += 8_i64 * ((*m).ncam) * (1_i64);
        size += 8_i64 * ((*m).ncam) * (1_i64);
        size += 4_i64 * ((*m).ncam) * (2_i64);
        size += 4_i64 * ((*m).ncam) * (1_i64);
        size += 4_i64 * ((*m).ncam) * (2_i64);
        size += 4_i64 * ((*m).ncam) * (4_i64);
        size += 8_i64 * ((*m).ncam) * ((*m).nuser_cam);
        size += 4_i64 * ((*m).nlight) * (1_i64);
        size += 4_i64 * ((*m).nlight) * (1_i64);
        size += 4_i64 * ((*m).nlight) * (1_i64);
        size += 4_i64 * ((*m).nlight) * (1_i64);
        size += 4_i64 * ((*m).nlight) * (1_i64);
        size += 1_i64 * ((*m).nlight) * (1_i64);
        size += 4_i64 * ((*m).nlight) * (1_i64);
        size += 4_i64 * ((*m).nlight) * (1_i64);
        size += 4_i64 * ((*m).nlight) * (1_i64);
        size += 1_i64 * ((*m).nlight) * (1_i64);
        size += 8_i64 * ((*m).nlight) * (3_i64);
        size += 8_i64 * ((*m).nlight) * (3_i64);
        size += 8_i64 * ((*m).nlight) * (3_i64);
        size += 8_i64 * ((*m).nlight) * (3_i64);
        size += 8_i64 * ((*m).nlight) * (3_i64);
        size += 4_i64 * ((*m).nlight) * (3_i64);
        size += 4_i64 * ((*m).nlight) * (1_i64);
        size += 4_i64 * ((*m).nlight) * (1_i64);
        size += 4_i64 * ((*m).nlight) * (3_i64);
        size += 4_i64 * ((*m).nlight) * (3_i64);
        size += 4_i64 * ((*m).nlight) * (3_i64);
        size += 4_i64 * ((*m).nflex) * (1_i64);
        size += 4_i64 * ((*m).nflex) * (1_i64);
        size += 4_i64 * ((*m).nflex) * (1_i64);
        size += 4_i64 * ((*m).nflex) * (1_i64);
        size += 8_i64 * ((*m).nflex) * (1_i64);
        size += 8_i64 * ((*m).nflex) * (2_i64);
        size += 8_i64 * ((*m).nflex) * (5_i64);
        size += 8_i64 * ((*m).nflex) * (3_i64);
        size += 8_i64 * ((*m).nflex) * (1_i64);
        size += 8_i64 * ((*m).nflex) * (1_i64);
        size += 1_i64 * ((*m).nflex) * (1_i64);
        size += 4_i64 * ((*m).nflex) * (1_i64);
        size += 4_i64 * ((*m).nflex) * (1_i64);
        size += 4_i64 * ((*m).nflex) * (1_i64);
        size += 4_i64 * ((*m).nflex) * (1_i64);
        size += 4_i64 * ((*m).nflex) * (1_i64);
        size += 4_i64 * ((*m).nflex) * (1_i64);
        size += 4_i64 * ((*m).nflex) * (1_i64);
        size += 4_i64 * ((*m).nflex) * (3_i64);
        size += 4_i64 * ((*m).nflex) * (1_i64);
        size += 4_i64 * ((*m).nflex) * (1_i64);
        size += 4_i64 * ((*m).nflex) * (1_i64);
        size += 4_i64 * ((*m).nflex) * (1_i64);
        size += 4_i64 * ((*m).nflex) * (1_i64);
        size += 4_i64 * ((*m).nflex) * (1_i64);
        size += 4_i64 * ((*m).nflex) * (1_i64);
        size += 4_i64 * ((*m).nflex) * (1_i64);
        size += 4_i64 * ((*m).nflex) * (1_i64);
        size += 4_i64 * ((*m).nflex) * (1_i64);
        size += 4_i64 * ((*m).nflex) * (1_i64);
        size += 4_i64 * ((*m).nflex) * (1_i64);
        size += 4_i64 * ((*m).nflex) * (1_i64);
        size += 4_i64 * ((*m).nflex) * (1_i64);
        size += 4_i64 * ((*m).nflex) * (1_i64);
        size += 4_i64 * ((*m).nflex) * (1_i64);
        size += 4_i64 * ((*m).nflex) * (1_i64);
        size += 4_i64 * ((*m).nflexnode) * (1_i64);
        size += 4_i64 * ((*m).nflexvert) * (1_i64);
        size += 4_i64 * ((*m).nflexvert) * (1_i64);
        size += 4_i64 * ((*m).nflexvert) * (1_i64);
        size += 4_i64 * ((*m).nflexedge) * (2_i64);
        size += 4_i64 * ((*m).nflexedge) * (2_i64);
        size += 4_i64 * ((*m).nflexedge) * (2_i64);
        size += 4_i64 * ((*m).nflexelemdata) * (1_i64);
        size += 4_i64 * ((*m).nflexelemdata) * (1_i64);
        size += 4_i64 * ((*m).nflexelemedge) * (1_i64);
        size += 4_i64 * ((*m).nflexelem) * (1_i64);
        size += 4_i64 * ((*m).nflexshelldata) * (1_i64);
        size += 4_i64 * ((*m).nflexevpair) * (2_i64);
        size += 8_i64 * ((*m).nflexvert) * (3_i64);
        size += 8_i64 * ((*m).nflexvert) * (3_i64);
        size += 8_i64 * ((*m).nflexvert) * (4_i64);
        size += 8_i64 * ((*m).nflexnode) * (3_i64);
        size += 8_i64 * ((*m).nflexnode) * (3_i64);
        size += 8_i64 * ((*m).nflexedge) * (1_i64);
        size += 8_i64 * ((*m).nflexedge) * (1_i64);
        size += 8_i64 * ((*m).nflex) * (1_i64);
        size += 8_i64 * ((*m).nflex) * (3_i64);
        size += 8_i64 * ((*m).nflexstiffness) * (1_i64);
        size += 8_i64 * ((*m).nflexbending) * (1_i64);
        size += 8_i64 * ((*m).nflex) * (1_i64);
        size += 8_i64 * ((*m).nflex) * (1_i64);
        size += 8_i64 * ((*m).nflex) * (1_i64);
        size += 4_i64 * ((*m).nflex) * (1_i64);
        size += 1_i64 * ((*m).nflex) * (1_i64);
        size += 1_i64 * ((*m).nflexedge) * (1_i64);
        size += 1_i64 * ((*m).nflex) * (1_i64);
        size += 1_i64 * ((*m).nflex) * (1_i64);
        size += 4_i64 * ((*m).nflex) * (1_i64);
        size += 4_i64 * ((*m).nflex) * (1_i64);
        size += 4_i64 * ((*m).nflexedge) * (1_i64);
        size += 4_i64 * ((*m).nflexedge) * (1_i64);
        size += 4_i64 * ((*m).nJfe) * (1_i64);
        size += 4_i64 * ((*m).nflexvert) * (2_i64);
        size += 4_i64 * ((*m).nflexvert) * (2_i64);
        size += 4_i64 * ((*m).nJfv) * (2_i64);
        size += 4_i64 * ((*m).nflex) * (4_i64);
        size += 4_i64 * ((*m).nflextexcoord) * (2_i64);
        size += 4_i64 * ((*m).nmesh) * (1_i64);
        size += 4_i64 * ((*m).nmesh) * (1_i64);
        size += 4_i64 * ((*m).nmesh) * (1_i64);
        size += 4_i64 * ((*m).nmesh) * (1_i64);
        size += 4_i64 * ((*m).nmesh) * (1_i64);
        size += 4_i64 * ((*m).nmesh) * (1_i64);
        size += 4_i64 * ((*m).nmesh) * (1_i64);
        size += 4_i64 * ((*m).nmesh) * (1_i64);
        size += 4_i64 * ((*m).nmesh) * (1_i64);
        size += 4_i64 * ((*m).nmesh) * (1_i64);
        size += 4_i64 * ((*m).nmesh) * (1_i64);
        size += 4_i64 * ((*m).nmesh) * (1_i64);
        size += 4_i64 * ((*m).nmesh) * (1_i64);
        size += 4_i64 * ((*m).nmeshvert) * (3_i64);
        size += 4_i64 * ((*m).nmeshnormal) * (3_i64);
        size += 4_i64 * ((*m).nmeshtexcoord) * (2_i64);
        size += 4_i64 * ((*m).nmeshface) * (3_i64);
        size += 4_i64 * ((*m).nmeshface) * (3_i64);
        size += 4_i64 * ((*m).nmeshface) * (3_i64);
        size += 4_i64 * ((*m).nmeshgraph) * (1_i64);
        size += 8_i64 * ((*m).nmesh) * (3_i64);
        size += 8_i64 * ((*m).nmesh) * (3_i64);
        size += 8_i64 * ((*m).nmesh) * (4_i64);
        size += 4_i64 * ((*m).nmesh) * (1_i64);
        size += 4_i64 * ((*m).nmesh) * (1_i64);
        size += 4_i64 * ((*m).nmesh) * (1_i64);
        size += 8_i64 * ((*m).nmeshpoly) * (3_i64);
        size += 4_i64 * ((*m).nmeshpoly) * (1_i64);
        size += 4_i64 * ((*m).nmeshpoly) * (1_i64);
        size += 4_i64 * ((*m).nmeshpolyvert) * (1_i64);
        size += 4_i64 * ((*m).nmeshvert) * (1_i64);
        size += 4_i64 * ((*m).nmeshvert) * (1_i64);
        size += 4_i64 * ((*m).nmeshpolymap) * (1_i64);
        size += 4_i64 * ((*m).nskin) * (1_i64);
        size += 4_i64 * ((*m).nskin) * (1_i64);
        size += 4_i64 * ((*m).nskin) * (4_i64);
        size += 4_i64 * ((*m).nskin) * (1_i64);
        size += 4_i64 * ((*m).nskin) * (1_i64);
        size += 4_i64 * ((*m).nskin) * (1_i64);
        size += 4_i64 * ((*m).nskin) * (1_i64);
        size += 4_i64 * ((*m).nskin) * (1_i64);
        size += 4_i64 * ((*m).nskin) * (1_i64);
        size += 4_i64 * ((*m).nskin) * (1_i64);
        size += 4_i64 * ((*m).nskin) * (1_i64);
        size += 4_i64 * ((*m).nskinvert) * (3_i64);
        size += 4_i64 * ((*m).nskintexvert) * (2_i64);
        size += 4_i64 * ((*m).nskinface) * (3_i64);
        size += 4_i64 * ((*m).nskinbone) * (1_i64);
        size += 4_i64 * ((*m).nskinbone) * (1_i64);
        size += 4_i64 * ((*m).nskinbone) * (3_i64);
        size += 4_i64 * ((*m).nskinbone) * (4_i64);
        size += 4_i64 * ((*m).nskinbone) * (1_i64);
        size += 4_i64 * ((*m).nskinbonevert) * (1_i64);
        size += 4_i64 * ((*m).nskinbonevert) * (1_i64);
        size += 4_i64 * ((*m).nskin) * (1_i64);
        size += 8_i64 * ((*m).nhfield) * (4_i64);
        size += 4_i64 * ((*m).nhfield) * (1_i64);
        size += 4_i64 * ((*m).nhfield) * (1_i64);
        size += 4_i64 * ((*m).nhfield) * (1_i64);
        size += 4_i64 * ((*m).nhfielddata) * (1_i64);
        size += 4_i64 * ((*m).nhfield) * (1_i64);
        size += 4_i64 * ((*m).ntex) * (1_i64);
        size += 4_i64 * ((*m).ntex) * (1_i64);
        size += 4_i64 * ((*m).ntex) * (1_i64);
        size += 4_i64 * ((*m).ntex) * (1_i64);
        size += 4_i64 * ((*m).ntex) * (1_i64);
        size += 8_i64 * ((*m).ntex) * (1_i64);
        size += 1_i64 * ((*m).ntexdata) * (1_i64);
        size += 4_i64 * ((*m).ntex) * (1_i64);
        size += 4_i64 * ((*m).nmat) * (10_i64);
        size += 1_i64 * ((*m).nmat) * (1_i64);
        size += 4_i64 * ((*m).nmat) * (2_i64);
        size += 4_i64 * ((*m).nmat) * (1_i64);
        size += 4_i64 * ((*m).nmat) * (1_i64);
        size += 4_i64 * ((*m).nmat) * (1_i64);
        size += 4_i64 * ((*m).nmat) * (1_i64);
        size += 4_i64 * ((*m).nmat) * (1_i64);
        size += 4_i64 * ((*m).nmat) * (1_i64);
        size += 4_i64 * ((*m).nmat) * (4_i64);
        size += 4_i64 * ((*m).npair) * (1_i64);
        size += 4_i64 * ((*m).npair) * (1_i64);
        size += 4_i64 * ((*m).npair) * (1_i64);
        size += 4_i64 * ((*m).npair) * (1_i64);
        size += 8_i64 * ((*m).npair) * (2_i64);
        size += 8_i64 * ((*m).npair) * (2_i64);
        size += 8_i64 * ((*m).npair) * (5_i64);
        size += 8_i64 * ((*m).npair) * (1_i64);
        size += 8_i64 * ((*m).npair) * (1_i64);
        size += 8_i64 * ((*m).npair) * (5_i64);
        size += 4_i64 * ((*m).nexclude) * (1_i64);
        size += 4_i64 * ((*m).neq) * (1_i64);
        size += 4_i64 * ((*m).neq) * (1_i64);
        size += 4_i64 * ((*m).neq) * (1_i64);
        size += 4_i64 * ((*m).neq) * (1_i64);
        size += 1_i64 * ((*m).neq) * (1_i64);
        size += 8_i64 * ((*m).neq) * (2_i64);
        size += 8_i64 * ((*m).neq) * (5_i64);
        size += 8_i64 * ((*m).neq) * (11_i64);
        size += 4_i64 * ((*m).ntendon) * (1_i64);
        size += 4_i64 * ((*m).ntendon) * (1_i64);
        size += 4_i64 * ((*m).ntendon) * (1_i64);
        size += 4_i64 * ((*m).ntendon) * (1_i64);
        size += 4_i64 * ((*m).ntendon) * (1_i64);
        size += 4_i64 * ((*m).ntendon) * (1_i64);
        size += 4_i64 * ((*m).ntendon) * (2_i64);
        size += 4_i64 * ((*m).ntendon) * (1_i64);
        size += 4_i64 * ((*m).ntendon) * (1_i64);
        size += 4_i64 * ((*m).nJten) * (1_i64);
        size += 1_i64 * ((*m).ntendon) * (1_i64);
        size += 1_i64 * ((*m).ntendon) * (1_i64);
        size += 8_i64 * ((*m).ntendon) * (1_i64);
        size += 8_i64 * ((*m).ntendon) * (2_i64);
        size += 8_i64 * ((*m).ntendon) * (5_i64);
        size += 8_i64 * ((*m).ntendon) * (2_i64);
        size += 8_i64 * ((*m).ntendon) * (5_i64);
        size += 8_i64 * ((*m).ntendon) * (2_i64);
        size += 8_i64 * ((*m).ntendon) * (2_i64);
        size += 8_i64 * ((*m).ntendon) * (1_i64);
        size += 8_i64 * ((*m).ntendon) * (1_i64);
        size += 8_i64 * ((*m).ntendon) * (2_i64);
        size += 8_i64 * ((*m).ntendon) * (1_i64);
        size += 8_i64 * ((*m).ntendon) * (2_i64);
        size += 8_i64 * ((*m).ntendon) * (1_i64);
        size += 8_i64 * ((*m).ntendon) * (1_i64);
        size += 8_i64 * ((*m).ntendon) * (2_i64);
        size += 8_i64 * ((*m).ntendon) * (1_i64);
        size += 8_i64 * ((*m).ntendon) * (1_i64);
        size += 8_i64 * ((*m).ntendon) * ((*m).nuser_tendon);
        size += 4_i64 * ((*m).ntendon) * (4_i64);
        size += 4_i64 * ((*m).nwrap) * (1_i64);
        size += 4_i64 * ((*m).nwrap) * (1_i64);
        size += 8_i64 * ((*m).nwrap) * (1_i64);
        size += 4_i64 * ((*m).nu) * (1_i64);
        size += 4_i64 * ((*m).nu) * (1_i64);
        size += 4_i64 * ((*m).nu) * (1_i64);
        size += 4_i64 * ((*m).nu) * (1_i64);
        size += 4_i64 * ((*m).nu) * (2_i64);
        size += 8_i64 * ((*m).nu) * (1_i64);
        size += 8_i64 * ((*m).nu) * (2_i64);
        size += 8_i64 * ((*m).nu) * (1_i64);
        size += 4_i64 * ((*m).nu) * (1_i64);
        size += 4_i64 * ((*m).nu) * (1_i64);
        size += 4_i64 * ((*m).nu) * (1_i64);
        size += 4_i64 * ((*m).nu) * (2_i64);
        size += 4_i64 * ((*m).nu) * (1_i64);
        size += 8_i64 * ((*m).nu) * (1_i64);
        size += 1_i64 * ((*m).nu) * (1_i64);
        size += 1_i64 * ((*m).nu) * (1_i64);
        size += 1_i64 * ((*m).nu) * (1_i64);
        size += 8_i64 * ((*m).nu) * (10_i64);
        size += 8_i64 * ((*m).nu) * (10_i64);
        size += 8_i64 * ((*m).nu) * (10_i64);
        size += 1_i64 * ((*m).nu) * (1_i64);
        size += 8_i64 * ((*m).nu) * (2_i64);
        size += 8_i64 * ((*m).nu) * (2_i64);
        size += 8_i64 * ((*m).nu) * (2_i64);
        size += 8_i64 * ((*m).nu) * (6_i64);
        size += 8_i64 * ((*m).nu) * (1_i64);
        size += 8_i64 * ((*m).nu) * (1_i64);
        size += 8_i64 * ((*m).nu) * (1_i64);
        size += 8_i64 * ((*m).nu) * (2_i64);
        size += 8_i64 * ((*m).nu) * ((*m).nuser_actuator);
        size += 4_i64 * ((*m).nu) * (1_i64);
        size += 4_i64 * ((*m).nsensor) * (1_i64);
        size += 4_i64 * ((*m).nsensor) * (1_i64);
        size += 4_i64 * ((*m).nsensor) * (1_i64);
        size += 4_i64 * ((*m).nsensor) * (1_i64);
        size += 4_i64 * ((*m).nsensor) * (1_i64);
        size += 4_i64 * ((*m).nsensor) * (1_i64);
        size += 4_i64 * ((*m).nsensor) * (1_i64);
        size += 4_i64 * ((*m).nsensor) * (3_i64);
        size += 4_i64 * ((*m).nsensor) * (1_i64);
        size += 4_i64 * ((*m).nsensor) * (1_i64);
        size += 8_i64 * ((*m).nsensor) * (1_i64);
        size += 8_i64 * ((*m).nsensor) * (1_i64);
        size += 4_i64 * ((*m).nsensor) * (2_i64);
        size += 4_i64 * ((*m).nsensor) * (1_i64);
        size += 8_i64 * ((*m).nsensor) * (1_i64);
        size += 8_i64 * ((*m).nsensor) * (2_i64);
        size += 8_i64 * ((*m).nsensor) * ((*m).nuser_sensor);
        size += 4_i64 * ((*m).nsensor) * (1_i64);
        size += 4_i64 * ((*m).nplugin) * (1_i64);
        size += 4_i64 * ((*m).nplugin) * (1_i64);
        size += 4_i64 * ((*m).nplugin) * (1_i64);
        size += 1_i64 * ((*m).npluginattr) * (1_i64);
        size += 4_i64 * ((*m).nplugin) * (1_i64);
        size += 4_i64 * ((*m).nnumeric) * (1_i64);
        size += 4_i64 * ((*m).nnumeric) * (1_i64);
        size += 8_i64 * ((*m).nnumericdata) * (1_i64);
        size += 4_i64 * ((*m).ntext) * (1_i64);
        size += 4_i64 * ((*m).ntext) * (1_i64);
        size += 1_i64 * ((*m).ntextdata) * (1_i64);
        size += 4_i64 * ((*m).ntuple) * (1_i64);
        size += 4_i64 * ((*m).ntuple) * (1_i64);
        size += 4_i64 * ((*m).ntupledata) * (1_i64);
        size += 4_i64 * ((*m).ntupledata) * (1_i64);
        size += 8_i64 * ((*m).ntupledata) * (1_i64);
        size += 8_i64 * ((*m).nkey) * (1_i64);
        size += 8_i64 * ((*m).nkey) * ((*m).nq);
        size += 8_i64 * ((*m).nkey) * ((*m).nv);
        size += 8_i64 * ((*m).nkey) * ((*m).na);
        size += 8_i64 * ((*m).nkey) * ((*m).nmocap * 3);
        size += 8_i64 * ((*m).nkey) * ((*m).nmocap * 4);
        size += 8_i64 * ((*m).nkey) * ((*m).nu);
        size += 4_i64 * ((*m).nbody) * (1_i64);
        size += 4_i64 * ((*m).njnt) * (1_i64);
        size += 4_i64 * ((*m).ngeom) * (1_i64);
        size += 4_i64 * ((*m).nsite) * (1_i64);
        size += 4_i64 * ((*m).ncam) * (1_i64);
        size += 4_i64 * ((*m).nlight) * (1_i64);
        size += 4_i64 * ((*m).nflex) * (1_i64);
        size += 4_i64 * ((*m).nmesh) * (1_i64);
        size += 4_i64 * ((*m).nskin) * (1_i64);
        size += 4_i64 * ((*m).nhfield) * (1_i64);
        size += 4_i64 * ((*m).ntex) * (1_i64);
        size += 4_i64 * ((*m).nmat) * (1_i64);
        size += 4_i64 * ((*m).npair) * (1_i64);
        size += 4_i64 * ((*m).nexclude) * (1_i64);
        size += 4_i64 * ((*m).neq) * (1_i64);
        size += 4_i64 * ((*m).ntendon) * (1_i64);
        size += 4_i64 * ((*m).nu) * (1_i64);
        size += 4_i64 * ((*m).nsensor) * (1_i64);
        size += 4_i64 * ((*m).nnumeric) * (1_i64);
        size += 4_i64 * ((*m).ntext) * (1_i64);
        size += 4_i64 * ((*m).ntuple) * (1_i64);
        size += 4_i64 * ((*m).nkey) * (1_i64);
        size += 4_i64 * ((*m).nplugin) * (1_i64);
        size += 1_i64 * ((*m).nnames) * (1_i64);
        size += 4_i64 * ((*m).nnames_map) * (1_i64);
        size += 1_i64 * ((*m).npaths) * (1_i64);
        size += 4_i64 * ((*m).nbody) * (1_i64);
        size += 4_i64 * ((*m).nbody) * (1_i64);
        size += 4_i64 * ((*m).nB) * (1_i64);
        size += 4_i64 * ((*m).nv) * (1_i64);
        size += 4_i64 * ((*m).nv) * (1_i64);
        size += 4_i64 * ((*m).nC) * (1_i64);
        size += 4_i64 * ((*m).nC) * (1_i64);
        size += 4_i64 * ((*m).nv) * (1_i64);
        size += 4_i64 * ((*m).nv) * (1_i64);
        size += 4_i64 * ((*m).nv) * (1_i64);
        size += 4_i64 * ((*m).nD) * (1_i64);
        size += 4_i64 * ((*m).nD) * (1_i64);
        size += 4_i64 * ((*m).nC) * (1_i64);

        size
    }
}

/// C: mj_validateReferences (engine/engine_io.h:87)
/// Calls: mjp_getPluginAtSlot, mju_message, numObjects, sensorSize
#[allow(unused_variables, non_snake_case)]
pub fn mj_validate_references(m: *const mjModel) -> *const i8 {
    todo!("mj_validateReferences depends on MJMODEL_REFERENCES X-macro expansion which enumerates ~100 reference fields for bounds checking. Cannot translate without codegen support for the field list.")
}

/// C: mj_makeDofDofSparse (engine/engine_io.h:90)
/// Calls: mju_copyInt, mju_message, mju_zeroInt
#[allow(unused_variables, non_snake_case)]
pub fn mj_make_dof_dof_sparse(nv: i32, nC: i32, nD: i32, nM: i32, dof_parentid: *const i32, dof_simplenum: *const i32, rownnz: *mut i32, rowadr: *mut i32, diag: *mut i32, colind: *mut i32, reduced: i32, upper: i32, remaining: *mut i32) {
    // SAFETY: all pointer args are valid arrays of appropriate sizes (caller contract)
    unsafe {
        // no dofs, nothing to do
        if nv == 0 {
            return;
        }

        // compute rownnz
        crate::engine::engine_util_misc::mju_zero_int(rownnz, nv);
        let mut i = nv - 1;
        while i >= 0 {
            // init at diagonal
            let mut j = i;
            *rownnz.add(i as usize) += 1;

            // process below diagonal unless reduced and dof is simple
            if !(reduced != 0 && *dof_simplenum.add(i as usize) != 0) {
                loop {
                    j = *dof_parentid.add(j as usize);
                    if j < 0 { break; }
                    // both reduced and non-reduced have lower triangle
                    *rownnz.add(i as usize) += 1;
                    // add upper triangle if requested
                    if upper != 0 {
                        *rownnz.add(j as usize) += 1;
                    }
                }
            }
            i -= 1;
        }

        // accumulate rowadr
        *rowadr.add(0) = 0;
        for i in 1..nv as usize {
            *rowadr.add(i) = *rowadr.add(i - 1) + *rownnz.add(i - 1);
        }

        // populate colind
        crate::engine::engine_util_misc::mju_copy_int(remaining, rownnz, nv);
        let mut i = nv - 1;
        while i >= 0 {
            // init at diagonal
            *remaining.add(i as usize) -= 1;
            *colind.add((*rowadr.add(i as usize) + *remaining.add(i as usize)) as usize) = i;

            // process below diagonal unless reduced and dof is simple
            if !(reduced != 0 && *dof_simplenum.add(i as usize) != 0) {
                let mut j = i;
                loop {
                    j = *dof_parentid.add(j as usize);
                    if j < 0 { break; }
                    *remaining.add(i as usize) -= 1;
                    *colind.add((*rowadr.add(i as usize) + *remaining.add(i as usize)) as usize) = j;

                    // add upper triangle if requested
                    if upper != 0 {
                        *remaining.add(j as usize) -= 1;
                        *colind.add((*rowadr.add(j as usize) + *remaining.add(j as usize)) as usize) = i;
                    }
                }
            }
            i -= 1;
        }

        // check for remaining; SHOULD NOT OCCUR
        for i in 0..nv as usize {
            if *remaining.add(i) != 0 {
                crate::engine::engine_util_errmem::mju_error(
                    b"unexpected remaining\0".as_ptr() as *const i8);
            }
        }

        // check total nnz; SHOULD NOT OCCUR
        let expected_nnz = if upper != 0 { nD } else if reduced != 0 { nC } else { nM };
        if *rowadr.add((nv - 1) as usize) + *rownnz.add((nv - 1) as usize) != expected_nnz {
            crate::engine::engine_util_errmem::mju_error(
                b"sum of rownnz different from expected\0".as_ptr() as *const i8);
        }

        // find diagonal indices
        if !diag.is_null() {
            for i in 0..nv as usize {
                let adr = *rowadr.add(i);
                let mut j = 0;
                while *colind.add((adr + j) as usize) < i as i32 && j < *rownnz.add(i) {
                    j += 1;
                }
                if *colind.add((adr + j) as usize) != i as i32 {
                    crate::engine::engine_util_errmem::mju_error(
                        b"diagonal index not found\0".as_ptr() as *const i8);
                }
                *diag.add(i) = j;
            }
        }
    }
}

/// C: mj_makeBSparse (engine/engine_io.h:96)
/// Calls: mju_insertionSortInt, mju_message, mju_zeroInt
#[allow(unused_variables, non_snake_case)]
pub fn mj_make_b_sparse(nv: i32, nbody: i32, nB: i32, body_dofnum: *const i32, body_parentid: *const i32, body_dofadr: *const i32, B_rownnz: *mut i32, B_rowadr: *mut i32, B_colind: *mut i32, count: *mut i32) {
    // SAFETY: all pointer args are valid arrays of appropriate sizes (caller contract)
    unsafe {
        // set rownnz to subtree dofs counts, including self
        crate::engine::engine_util_misc::mju_zero_int(B_rownnz, nbody);
        let mut i = nbody - 1;
        while i > 0 {
            *B_rownnz.add(i as usize) += *body_dofnum.add(i as usize);
            *B_rownnz.add(*body_parentid.add(i as usize) as usize) += *B_rownnz.add(i as usize);
            i -= 1;
        }

        // check if rownnz[0] != nv; SHOULD NOT OCCUR
        if *B_rownnz.add(0) != nv {
            crate::engine::engine_util_errmem::mju_error(
                b"rownnz[0] different from nv\0".as_ptr() as *const i8);
        }

        // add dofs in ancestors bodies
        for i in 0..nbody as usize {
            let mut j = *body_parentid.add(i) as usize;
            while j > 0 {
                *B_rownnz.add(i) += *body_dofnum.add(j);
                j = *body_parentid.add(j) as usize;
            }
        }

        // compute rowadr
        *B_rowadr.add(0) = 0;
        for i in 1..nbody as usize {
            *B_rowadr.add(i) = *B_rowadr.add(i - 1) + *B_rownnz.add(i - 1);
        }

        // check if total nnz != nB; SHOULD NOT OCCUR
        if nB != *B_rowadr.add((nbody - 1) as usize) + *B_rownnz.add((nbody - 1) as usize) {
            crate::engine::engine_util_errmem::mju_error(
                b"sum of rownnz different from nB\0".as_ptr() as *const i8);
        }

        // clear incremental row counts
        crate::engine::engine_util_misc::mju_zero_int(count, nbody);

        // add subtree dofs to colind
        let mut i = nbody - 1;
        while i > 0 {
            // add this body's dofs to subtree
            for n in 0..*body_dofnum.add(i as usize) {
                *B_colind.add((*B_rowadr.add(i as usize) + *count.add(i as usize)) as usize) =
                    *body_dofadr.add(i as usize) + n;
                *count.add(i as usize) += 1;
            }

            // add body subtree to parent
            let par = *body_parentid.add(i as usize);
            for n in 0..*count.add(i as usize) {
                *B_colind.add((*B_rowadr.add(par as usize) + *count.add(par as usize)) as usize) =
                    *B_colind.add((*B_rowadr.add(i as usize) + n) as usize);
                *count.add(par as usize) += 1;
            }
            i -= 1;
        }

        // add all ancestor dofs
        for i in 0..nbody as usize {
            let mut par = *body_parentid.add(i) as usize;
            while par > 0 {
                // add ancestor body dofs
                for n in 0..*body_dofnum.add(par) {
                    *B_colind.add((*B_rowadr.add(i) + *count.add(i)) as usize) =
                        *body_dofadr.add(par) + n;
                    *count.add(i) += 1;
                }
                // advance to parent
                par = *body_parentid.add(par) as usize;
            }
        }

        // process all bodies
        for i in 0..nbody as usize {
            // make sure cnt = rownnz; SHOULD NOT OCCUR
            if *B_rownnz.add(i) != *count.add(i) {
                crate::engine::engine_util_errmem::mju_error(
                    b"cnt different from rownnz\0".as_ptr() as *const i8);
            }

            // sort colind in each row
            if *count.add(i) > 1 {
                crate::engine::engine_util_misc::mju_insertion_sort_int(
                    B_colind.add(*B_rowadr.add(i) as usize) as *mut i32,
                    *count.add(i));
            }
        }
    }
}

/// C: mj_makeDofDofMaps (engine/engine_io.h:102)
/// Calls: copyM2Sparse, mju_fillInt, mju_lower2SymMap, mju_message, mju_sparseMap
#[allow(unused_variables, non_snake_case)]
pub fn mj_make_dof_dof_maps(nv: i32, nM: i32, nC: i32, nD: i32, dof_Madr: *const i32, dof_simplenum: *const i32, dof_parentid: *const i32, D_rownnz: *const i32, D_rowadr: *const i32, D_colind: *const i32, M_rownnz: *const i32, M_rowadr: *const i32, M_colind: *const i32, mapM2D: *mut i32, mapD2M: *mut i32, mapM2M: *mut i32, M: *mut i32, scratch: *mut i32) {
    // SAFETY: all pointer args are valid arrays of appropriate sizes (caller contract)
    unsafe {
        // make mapM2D: M -> D (lower to symmetric)
        crate::engine::engine_util_misc::mju_lower2sym_map(
            mapM2D, nv, D_rowadr, D_rownnz, D_colind, M_rowadr, M_rownnz, M_colind, scratch);

        // make mapD2M: D -> M (symmetric to lower)
        crate::engine::engine_util_misc::mju_sparse_map(
            mapD2M, nv, M_rowadr, M_rownnz, M_colind, D_rowadr, D_rownnz, D_colind);

        // make mapM2M
        for i in 0..nM as usize {
            *M.add(i) = i as i32;
        }
        crate::engine::engine_util_misc::mju_fill_int(mapM2M, -1, nC);
        copy_m2sparse(nv, dof_Madr, dof_simplenum, dof_parentid, M_rownnz,
                      M_rowadr, M, mapM2M, /*reduced=*/1, /*upper=*/0, scratch);

        // check that all indices are filled in
        for i in 0..nC as usize {
            if *mapM2M.add(i) < 0 {
                crate::engine::engine_util_errmem::mju_error(
                    b"unassigned index in mapM2M\0".as_ptr() as *const i8);
            }
        }
    }
}

/// C: mj_makeData (engine/engine_io.h:113)
/// Calls: mj_initPlugin, mj_makeRawData, mj_resetData
#[allow(unused_variables, non_snake_case)]
pub fn mj_make_data(m: *const mjModel) -> *mut mjData {
    todo!() // mj_makeData
}

/// C: mj_makeRawData (engine/engine_io.h:116)
/// Calls: freeDataBuffers, mj_setPtrData, mju_free, mju_malloc, mju_message, mju_warning, safeAddToBufferSize
#[allow(unused_variables, non_snake_case)]
pub fn mj_make_raw_data(dest: *mut *mut mjData, m: *const mjModel) {
    // SAFETY: dest, m are valid pointers (caller contract)
    unsafe {
        let mut offset: isize = 0;
        let allocate: i32 = if (*dest).is_null() { 1 } else { 0 };
        let d: *mut mjData;

        // allocate mjData
        if allocate == 0 {
            d = *dest;
            free_data_buffers(d);
        } else {
            d = crate::engine::engine_util_errmem::mju_malloc(
                std::mem::size_of::<mjData>()) as *mut mjData;
        }

        if d.is_null() {
            crate::engine::engine_util_errmem::mju_error(
                b"could not allocate mjData\0".as_ptr() as *const i8);
            return;
        }

        // prevent spurious timing print: timer[0].number = 0
        // timer is [u8; 240], mjTimerStat is 16 bytes, number at offset 8
        let timer_number_ptr = (*d).timer.as_mut_ptr().add(8) as *mut i32;
        *timer_number_ptr = 0;

        // compute buffer size
        (*d).nbuffer = 0;
        (*d).buffer = std::ptr::null_mut();
        (*d).arena = std::ptr::null_mut();
        // MJDATA_POINTERS: safeAddToBufferSize for each field
        if safe_add_to_buffer_size(&mut offset, &mut (*d).nbuffer, 8, (*m).nq, 1_i64) == 0 {
            if allocate != 0 { crate::engine::engine_util_errmem::mju_free(d as *mut ()); }
            crate::engine::engine_util_errmem::mju_warning(b"Invalid data: qpos too large. ".as_ptr() as *const i8);
            return;
        }
        if safe_add_to_buffer_size(&mut offset, &mut (*d).nbuffer, 8, (*m).nv, 1_i64) == 0 {
            if allocate != 0 { crate::engine::engine_util_errmem::mju_free(d as *mut ()); }
            crate::engine::engine_util_errmem::mju_warning(b"Invalid data: qvel too large. ".as_ptr() as *const i8);
            return;
        }
        if safe_add_to_buffer_size(&mut offset, &mut (*d).nbuffer, 8, (*m).na, 1_i64) == 0 {
            if allocate != 0 { crate::engine::engine_util_errmem::mju_free(d as *mut ()); }
            crate::engine::engine_util_errmem::mju_warning(b"Invalid data: act too large. ".as_ptr() as *const i8);
            return;
        }
        if safe_add_to_buffer_size(&mut offset, &mut (*d).nbuffer, 8, (*m).nhistory, 1_i64) == 0 {
            if allocate != 0 { crate::engine::engine_util_errmem::mju_free(d as *mut ()); }
            crate::engine::engine_util_errmem::mju_warning(b"Invalid data: history too large. ".as_ptr() as *const i8);
            return;
        }
        if safe_add_to_buffer_size(&mut offset, &mut (*d).nbuffer, 8, (*m).nv, 1_i64) == 0 {
            if allocate != 0 { crate::engine::engine_util_errmem::mju_free(d as *mut ()); }
            crate::engine::engine_util_errmem::mju_warning(b"Invalid data: qacc_warmstart too large. ".as_ptr() as *const i8);
            return;
        }
        if safe_add_to_buffer_size(&mut offset, &mut (*d).nbuffer, 8, (*m).npluginstate, 1_i64) == 0 {
            if allocate != 0 { crate::engine::engine_util_errmem::mju_free(d as *mut ()); }
            crate::engine::engine_util_errmem::mju_warning(b"Invalid data: plugin_state too large. ".as_ptr() as *const i8);
            return;
        }
        if safe_add_to_buffer_size(&mut offset, &mut (*d).nbuffer, 8, (*m).nu, 1_i64) == 0 {
            if allocate != 0 { crate::engine::engine_util_errmem::mju_free(d as *mut ()); }
            crate::engine::engine_util_errmem::mju_warning(b"Invalid data: ctrl too large. ".as_ptr() as *const i8);
            return;
        }
        if safe_add_to_buffer_size(&mut offset, &mut (*d).nbuffer, 8, (*m).nv, 1_i64) == 0 {
            if allocate != 0 { crate::engine::engine_util_errmem::mju_free(d as *mut ()); }
            crate::engine::engine_util_errmem::mju_warning(b"Invalid data: qfrc_applied too large. ".as_ptr() as *const i8);
            return;
        }
        if safe_add_to_buffer_size(&mut offset, &mut (*d).nbuffer, 8, (*m).nbody, 6_i64) == 0 {
            if allocate != 0 { crate::engine::engine_util_errmem::mju_free(d as *mut ()); }
            crate::engine::engine_util_errmem::mju_warning(b"Invalid data: xfrc_applied too large. ".as_ptr() as *const i8);
            return;
        }
        if safe_add_to_buffer_size(&mut offset, &mut (*d).nbuffer, 1, (*m).neq, 1_i64) == 0 {
            if allocate != 0 { crate::engine::engine_util_errmem::mju_free(d as *mut ()); }
            crate::engine::engine_util_errmem::mju_warning(b"Invalid data: eq_active too large. ".as_ptr() as *const i8);
            return;
        }
        if safe_add_to_buffer_size(&mut offset, &mut (*d).nbuffer, 8, (*m).nmocap, 3_i64) == 0 {
            if allocate != 0 { crate::engine::engine_util_errmem::mju_free(d as *mut ()); }
            crate::engine::engine_util_errmem::mju_warning(b"Invalid data: mocap_pos too large. ".as_ptr() as *const i8);
            return;
        }
        if safe_add_to_buffer_size(&mut offset, &mut (*d).nbuffer, 8, (*m).nmocap, 4_i64) == 0 {
            if allocate != 0 { crate::engine::engine_util_errmem::mju_free(d as *mut ()); }
            crate::engine::engine_util_errmem::mju_warning(b"Invalid data: mocap_quat too large. ".as_ptr() as *const i8);
            return;
        }
        if safe_add_to_buffer_size(&mut offset, &mut (*d).nbuffer, 8, (*m).nv, 1_i64) == 0 {
            if allocate != 0 { crate::engine::engine_util_errmem::mju_free(d as *mut ()); }
            crate::engine::engine_util_errmem::mju_warning(b"Invalid data: qacc too large. ".as_ptr() as *const i8);
            return;
        }
        if safe_add_to_buffer_size(&mut offset, &mut (*d).nbuffer, 8, (*m).na, 1_i64) == 0 {
            if allocate != 0 { crate::engine::engine_util_errmem::mju_free(d as *mut ()); }
            crate::engine::engine_util_errmem::mju_warning(b"Invalid data: act_dot too large. ".as_ptr() as *const i8);
            return;
        }
        if safe_add_to_buffer_size(&mut offset, &mut (*d).nbuffer, 8, (*m).nuserdata, 1_i64) == 0 {
            if allocate != 0 { crate::engine::engine_util_errmem::mju_free(d as *mut ()); }
            crate::engine::engine_util_errmem::mju_warning(b"Invalid data: userdata too large. ".as_ptr() as *const i8);
            return;
        }
        if safe_add_to_buffer_size(&mut offset, &mut (*d).nbuffer, 8, (*m).nsensordata, 1_i64) == 0 {
            if allocate != 0 { crate::engine::engine_util_errmem::mju_free(d as *mut ()); }
            crate::engine::engine_util_errmem::mju_warning(b"Invalid data: sensordata too large. ".as_ptr() as *const i8);
            return;
        }
        if safe_add_to_buffer_size(&mut offset, &mut (*d).nbuffer, 4, (*m).ntree, 1_i64) == 0 {
            if allocate != 0 { crate::engine::engine_util_errmem::mju_free(d as *mut ()); }
            crate::engine::engine_util_errmem::mju_warning(b"Invalid data: tree_asleep too large. ".as_ptr() as *const i8);
            return;
        }
        if safe_add_to_buffer_size(&mut offset, &mut (*d).nbuffer, 4, (*m).nplugin, 1_i64) == 0 {
            if allocate != 0 { crate::engine::engine_util_errmem::mju_free(d as *mut ()); }
            crate::engine::engine_util_errmem::mju_warning(b"Invalid data: plugin too large. ".as_ptr() as *const i8);
            return;
        }
        if safe_add_to_buffer_size(&mut offset, &mut (*d).nbuffer, 8, (*m).nplugin, 1_i64) == 0 {
            if allocate != 0 { crate::engine::engine_util_errmem::mju_free(d as *mut ()); }
            crate::engine::engine_util_errmem::mju_warning(b"Invalid data: plugin_data too large. ".as_ptr() as *const i8);
            return;
        }
        if safe_add_to_buffer_size(&mut offset, &mut (*d).nbuffer, 8, (*m).nbody, 3_i64) == 0 {
            if allocate != 0 { crate::engine::engine_util_errmem::mju_free(d as *mut ()); }
            crate::engine::engine_util_errmem::mju_warning(b"Invalid data: xpos too large. ".as_ptr() as *const i8);
            return;
        }
        if safe_add_to_buffer_size(&mut offset, &mut (*d).nbuffer, 8, (*m).nbody, 4_i64) == 0 {
            if allocate != 0 { crate::engine::engine_util_errmem::mju_free(d as *mut ()); }
            crate::engine::engine_util_errmem::mju_warning(b"Invalid data: xquat too large. ".as_ptr() as *const i8);
            return;
        }
        if safe_add_to_buffer_size(&mut offset, &mut (*d).nbuffer, 8, (*m).nbody, 9_i64) == 0 {
            if allocate != 0 { crate::engine::engine_util_errmem::mju_free(d as *mut ()); }
            crate::engine::engine_util_errmem::mju_warning(b"Invalid data: xmat too large. ".as_ptr() as *const i8);
            return;
        }
        if safe_add_to_buffer_size(&mut offset, &mut (*d).nbuffer, 8, (*m).nbody, 3_i64) == 0 {
            if allocate != 0 { crate::engine::engine_util_errmem::mju_free(d as *mut ()); }
            crate::engine::engine_util_errmem::mju_warning(b"Invalid data: xipos too large. ".as_ptr() as *const i8);
            return;
        }
        if safe_add_to_buffer_size(&mut offset, &mut (*d).nbuffer, 8, (*m).nbody, 9_i64) == 0 {
            if allocate != 0 { crate::engine::engine_util_errmem::mju_free(d as *mut ()); }
            crate::engine::engine_util_errmem::mju_warning(b"Invalid data: ximat too large. ".as_ptr() as *const i8);
            return;
        }
        if safe_add_to_buffer_size(&mut offset, &mut (*d).nbuffer, 8, (*m).njnt, 3_i64) == 0 {
            if allocate != 0 { crate::engine::engine_util_errmem::mju_free(d as *mut ()); }
            crate::engine::engine_util_errmem::mju_warning(b"Invalid data: xanchor too large. ".as_ptr() as *const i8);
            return;
        }
        if safe_add_to_buffer_size(&mut offset, &mut (*d).nbuffer, 8, (*m).njnt, 3_i64) == 0 {
            if allocate != 0 { crate::engine::engine_util_errmem::mju_free(d as *mut ()); }
            crate::engine::engine_util_errmem::mju_warning(b"Invalid data: xaxis too large. ".as_ptr() as *const i8);
            return;
        }
        if safe_add_to_buffer_size(&mut offset, &mut (*d).nbuffer, 8, (*m).ngeom, 3_i64) == 0 {
            if allocate != 0 { crate::engine::engine_util_errmem::mju_free(d as *mut ()); }
            crate::engine::engine_util_errmem::mju_warning(b"Invalid data: geom_xpos too large. ".as_ptr() as *const i8);
            return;
        }
        if safe_add_to_buffer_size(&mut offset, &mut (*d).nbuffer, 8, (*m).ngeom, 9_i64) == 0 {
            if allocate != 0 { crate::engine::engine_util_errmem::mju_free(d as *mut ()); }
            crate::engine::engine_util_errmem::mju_warning(b"Invalid data: geom_xmat too large. ".as_ptr() as *const i8);
            return;
        }
        if safe_add_to_buffer_size(&mut offset, &mut (*d).nbuffer, 8, (*m).nsite, 3_i64) == 0 {
            if allocate != 0 { crate::engine::engine_util_errmem::mju_free(d as *mut ()); }
            crate::engine::engine_util_errmem::mju_warning(b"Invalid data: site_xpos too large. ".as_ptr() as *const i8);
            return;
        }
        if safe_add_to_buffer_size(&mut offset, &mut (*d).nbuffer, 8, (*m).nsite, 9_i64) == 0 {
            if allocate != 0 { crate::engine::engine_util_errmem::mju_free(d as *mut ()); }
            crate::engine::engine_util_errmem::mju_warning(b"Invalid data: site_xmat too large. ".as_ptr() as *const i8);
            return;
        }
        if safe_add_to_buffer_size(&mut offset, &mut (*d).nbuffer, 8, (*m).ncam, 3_i64) == 0 {
            if allocate != 0 { crate::engine::engine_util_errmem::mju_free(d as *mut ()); }
            crate::engine::engine_util_errmem::mju_warning(b"Invalid data: cam_xpos too large. ".as_ptr() as *const i8);
            return;
        }
        if safe_add_to_buffer_size(&mut offset, &mut (*d).nbuffer, 8, (*m).ncam, 9_i64) == 0 {
            if allocate != 0 { crate::engine::engine_util_errmem::mju_free(d as *mut ()); }
            crate::engine::engine_util_errmem::mju_warning(b"Invalid data: cam_xmat too large. ".as_ptr() as *const i8);
            return;
        }
        if safe_add_to_buffer_size(&mut offset, &mut (*d).nbuffer, 8, (*m).nlight, 3_i64) == 0 {
            if allocate != 0 { crate::engine::engine_util_errmem::mju_free(d as *mut ()); }
            crate::engine::engine_util_errmem::mju_warning(b"Invalid data: light_xpos too large. ".as_ptr() as *const i8);
            return;
        }
        if safe_add_to_buffer_size(&mut offset, &mut (*d).nbuffer, 8, (*m).nlight, 3_i64) == 0 {
            if allocate != 0 { crate::engine::engine_util_errmem::mju_free(d as *mut ()); }
            crate::engine::engine_util_errmem::mju_warning(b"Invalid data: light_xdir too large. ".as_ptr() as *const i8);
            return;
        }
        if safe_add_to_buffer_size(&mut offset, &mut (*d).nbuffer, 8, (*m).nbody, 3_i64) == 0 {
            if allocate != 0 { crate::engine::engine_util_errmem::mju_free(d as *mut ()); }
            crate::engine::engine_util_errmem::mju_warning(b"Invalid data: subtree_com too large. ".as_ptr() as *const i8);
            return;
        }
        if safe_add_to_buffer_size(&mut offset, &mut (*d).nbuffer, 8, (*m).nv, 6_i64) == 0 {
            if allocate != 0 { crate::engine::engine_util_errmem::mju_free(d as *mut ()); }
            crate::engine::engine_util_errmem::mju_warning(b"Invalid data: cdof too large. ".as_ptr() as *const i8);
            return;
        }
        if safe_add_to_buffer_size(&mut offset, &mut (*d).nbuffer, 8, (*m).nbody, 10_i64) == 0 {
            if allocate != 0 { crate::engine::engine_util_errmem::mju_free(d as *mut ()); }
            crate::engine::engine_util_errmem::mju_warning(b"Invalid data: cinert too large. ".as_ptr() as *const i8);
            return;
        }
        if safe_add_to_buffer_size(&mut offset, &mut (*d).nbuffer, 8, (*m).nflexvert, 3_i64) == 0 {
            if allocate != 0 { crate::engine::engine_util_errmem::mju_free(d as *mut ()); }
            crate::engine::engine_util_errmem::mju_warning(b"Invalid data: flexvert_xpos too large. ".as_ptr() as *const i8);
            return;
        }
        if safe_add_to_buffer_size(&mut offset, &mut (*d).nbuffer, 8, (*m).nflexelem, 6_i64) == 0 {
            if allocate != 0 { crate::engine::engine_util_errmem::mju_free(d as *mut ()); }
            crate::engine::engine_util_errmem::mju_warning(b"Invalid data: flexelem_aabb too large. ".as_ptr() as *const i8);
            return;
        }
        if safe_add_to_buffer_size(&mut offset, &mut (*d).nbuffer, 8, (*m).nJfe, 1_i64) == 0 {
            if allocate != 0 { crate::engine::engine_util_errmem::mju_free(d as *mut ()); }
            crate::engine::engine_util_errmem::mju_warning(b"Invalid data: flexedge_J too large. ".as_ptr() as *const i8);
            return;
        }
        if safe_add_to_buffer_size(&mut offset, &mut (*d).nbuffer, 8, (*m).nflexedge, 1_i64) == 0 {
            if allocate != 0 { crate::engine::engine_util_errmem::mju_free(d as *mut ()); }
            crate::engine::engine_util_errmem::mju_warning(b"Invalid data: flexedge_length too large. ".as_ptr() as *const i8);
            return;
        }
        if safe_add_to_buffer_size(&mut offset, &mut (*d).nbuffer, 8, (*m).nJfv, 2_i64) == 0 {
            if allocate != 0 { crate::engine::engine_util_errmem::mju_free(d as *mut ()); }
            crate::engine::engine_util_errmem::mju_warning(b"Invalid data: flexvert_J too large. ".as_ptr() as *const i8);
            return;
        }
        if safe_add_to_buffer_size(&mut offset, &mut (*d).nbuffer, 8, (*m).nflexvert, 2_i64) == 0 {
            if allocate != 0 { crate::engine::engine_util_errmem::mju_free(d as *mut ()); }
            crate::engine::engine_util_errmem::mju_warning(b"Invalid data: flexvert_length too large. ".as_ptr() as *const i8);
            return;
        }
        if safe_add_to_buffer_size(&mut offset, &mut (*d).nbuffer, 8, (*m).nbvhdynamic, 6_i64) == 0 {
            if allocate != 0 { crate::engine::engine_util_errmem::mju_free(d as *mut ()); }
            crate::engine::engine_util_errmem::mju_warning(b"Invalid data: bvh_aabb_dyn too large. ".as_ptr() as *const i8);
            return;
        }
        if safe_add_to_buffer_size(&mut offset, &mut (*d).nbuffer, 4, (*m).ntendon, 1_i64) == 0 {
            if allocate != 0 { crate::engine::engine_util_errmem::mju_free(d as *mut ()); }
            crate::engine::engine_util_errmem::mju_warning(b"Invalid data: ten_wrapadr too large. ".as_ptr() as *const i8);
            return;
        }
        if safe_add_to_buffer_size(&mut offset, &mut (*d).nbuffer, 4, (*m).ntendon, 1_i64) == 0 {
            if allocate != 0 { crate::engine::engine_util_errmem::mju_free(d as *mut ()); }
            crate::engine::engine_util_errmem::mju_warning(b"Invalid data: ten_wrapnum too large. ".as_ptr() as *const i8);
            return;
        }
        if safe_add_to_buffer_size(&mut offset, &mut (*d).nbuffer, 8, (*m).nJten, 1_i64) == 0 {
            if allocate != 0 { crate::engine::engine_util_errmem::mju_free(d as *mut ()); }
            crate::engine::engine_util_errmem::mju_warning(b"Invalid data: ten_J too large. ".as_ptr() as *const i8);
            return;
        }
        if safe_add_to_buffer_size(&mut offset, &mut (*d).nbuffer, 8, (*m).ntendon, 1_i64) == 0 {
            if allocate != 0 { crate::engine::engine_util_errmem::mju_free(d as *mut ()); }
            crate::engine::engine_util_errmem::mju_warning(b"Invalid data: ten_length too large. ".as_ptr() as *const i8);
            return;
        }
        if safe_add_to_buffer_size(&mut offset, &mut (*d).nbuffer, 4, (*m).nwrap, 2_i64) == 0 {
            if allocate != 0 { crate::engine::engine_util_errmem::mju_free(d as *mut ()); }
            crate::engine::engine_util_errmem::mju_warning(b"Invalid data: wrap_obj too large. ".as_ptr() as *const i8);
            return;
        }
        if safe_add_to_buffer_size(&mut offset, &mut (*d).nbuffer, 8, (*m).nwrap, 6_i64) == 0 {
            if allocate != 0 { crate::engine::engine_util_errmem::mju_free(d as *mut ()); }
            crate::engine::engine_util_errmem::mju_warning(b"Invalid data: wrap_xpos too large. ".as_ptr() as *const i8);
            return;
        }
        if safe_add_to_buffer_size(&mut offset, &mut (*d).nbuffer, 8, (*m).nu, 1_i64) == 0 {
            if allocate != 0 { crate::engine::engine_util_errmem::mju_free(d as *mut ()); }
            crate::engine::engine_util_errmem::mju_warning(b"Invalid data: actuator_length too large. ".as_ptr() as *const i8);
            return;
        }
        if safe_add_to_buffer_size(&mut offset, &mut (*d).nbuffer, 4, (*m).nu, 1_i64) == 0 {
            if allocate != 0 { crate::engine::engine_util_errmem::mju_free(d as *mut ()); }
            crate::engine::engine_util_errmem::mju_warning(b"Invalid data: moment_rownnz too large. ".as_ptr() as *const i8);
            return;
        }
        if safe_add_to_buffer_size(&mut offset, &mut (*d).nbuffer, 4, (*m).nu, 1_i64) == 0 {
            if allocate != 0 { crate::engine::engine_util_errmem::mju_free(d as *mut ()); }
            crate::engine::engine_util_errmem::mju_warning(b"Invalid data: moment_rowadr too large. ".as_ptr() as *const i8);
            return;
        }
        if safe_add_to_buffer_size(&mut offset, &mut (*d).nbuffer, 4, (*m).nJmom, 1_i64) == 0 {
            if allocate != 0 { crate::engine::engine_util_errmem::mju_free(d as *mut ()); }
            crate::engine::engine_util_errmem::mju_warning(b"Invalid data: moment_colind too large. ".as_ptr() as *const i8);
            return;
        }
        if safe_add_to_buffer_size(&mut offset, &mut (*d).nbuffer, 8, (*m).nJmom, 1_i64) == 0 {
            if allocate != 0 { crate::engine::engine_util_errmem::mju_free(d as *mut ()); }
            crate::engine::engine_util_errmem::mju_warning(b"Invalid data: actuator_moment too large. ".as_ptr() as *const i8);
            return;
        }
        if safe_add_to_buffer_size(&mut offset, &mut (*d).nbuffer, 8, (*m).nbody, 10_i64) == 0 {
            if allocate != 0 { crate::engine::engine_util_errmem::mju_free(d as *mut ()); }
            crate::engine::engine_util_errmem::mju_warning(b"Invalid data: crb too large. ".as_ptr() as *const i8);
            return;
        }
        if safe_add_to_buffer_size(&mut offset, &mut (*d).nbuffer, 8, (*m).nM, 1_i64) == 0 {
            if allocate != 0 { crate::engine::engine_util_errmem::mju_free(d as *mut ()); }
            crate::engine::engine_util_errmem::mju_warning(b"Invalid data: qM too large. ".as_ptr() as *const i8);
            return;
        }
        if safe_add_to_buffer_size(&mut offset, &mut (*d).nbuffer, 8, (*m).nC, 1_i64) == 0 {
            if allocate != 0 { crate::engine::engine_util_errmem::mju_free(d as *mut ()); }
            crate::engine::engine_util_errmem::mju_warning(b"Invalid data: M too large. ".as_ptr() as *const i8);
            return;
        }
        if safe_add_to_buffer_size(&mut offset, &mut (*d).nbuffer, 8, (*m).nC, 1_i64) == 0 {
            if allocate != 0 { crate::engine::engine_util_errmem::mju_free(d as *mut ()); }
            crate::engine::engine_util_errmem::mju_warning(b"Invalid data: qLD too large. ".as_ptr() as *const i8);
            return;
        }
        if safe_add_to_buffer_size(&mut offset, &mut (*d).nbuffer, 8, (*m).nv, 1_i64) == 0 {
            if allocate != 0 { crate::engine::engine_util_errmem::mju_free(d as *mut ()); }
            crate::engine::engine_util_errmem::mju_warning(b"Invalid data: qLDiagInv too large. ".as_ptr() as *const i8);
            return;
        }
        if safe_add_to_buffer_size(&mut offset, &mut (*d).nbuffer, 1, (*m).nbvh, 1_i64) == 0 {
            if allocate != 0 { crate::engine::engine_util_errmem::mju_free(d as *mut ()); }
            crate::engine::engine_util_errmem::mju_warning(b"Invalid data: bvh_active too large. ".as_ptr() as *const i8);
            return;
        }
        if safe_add_to_buffer_size(&mut offset, &mut (*d).nbuffer, 4, (*m).ntree, 1_i64) == 0 {
            if allocate != 0 { crate::engine::engine_util_errmem::mju_free(d as *mut ()); }
            crate::engine::engine_util_errmem::mju_warning(b"Invalid data: tree_awake too large. ".as_ptr() as *const i8);
            return;
        }
        if safe_add_to_buffer_size(&mut offset, &mut (*d).nbuffer, 4, (*m).nbody, 1_i64) == 0 {
            if allocate != 0 { crate::engine::engine_util_errmem::mju_free(d as *mut ()); }
            crate::engine::engine_util_errmem::mju_warning(b"Invalid data: body_awake too large. ".as_ptr() as *const i8);
            return;
        }
        if safe_add_to_buffer_size(&mut offset, &mut (*d).nbuffer, 4, (*m).nbody, 1_i64) == 0 {
            if allocate != 0 { crate::engine::engine_util_errmem::mju_free(d as *mut ()); }
            crate::engine::engine_util_errmem::mju_warning(b"Invalid data: body_awake_ind too large. ".as_ptr() as *const i8);
            return;
        }
        if safe_add_to_buffer_size(&mut offset, &mut (*d).nbuffer, 4, (*m).nbody, 1_i64) == 0 {
            if allocate != 0 { crate::engine::engine_util_errmem::mju_free(d as *mut ()); }
            crate::engine::engine_util_errmem::mju_warning(b"Invalid data: parent_awake_ind too large. ".as_ptr() as *const i8);
            return;
        }
        if safe_add_to_buffer_size(&mut offset, &mut (*d).nbuffer, 4, (*m).nv, 1_i64) == 0 {
            if allocate != 0 { crate::engine::engine_util_errmem::mju_free(d as *mut ()); }
            crate::engine::engine_util_errmem::mju_warning(b"Invalid data: dof_awake_ind too large. ".as_ptr() as *const i8);
            return;
        }
        if safe_add_to_buffer_size(&mut offset, &mut (*d).nbuffer, 8, (*m).nflexedge, 1_i64) == 0 {
            if allocate != 0 { crate::engine::engine_util_errmem::mju_free(d as *mut ()); }
            crate::engine::engine_util_errmem::mju_warning(b"Invalid data: flexedge_velocity too large. ".as_ptr() as *const i8);
            return;
        }
        if safe_add_to_buffer_size(&mut offset, &mut (*d).nbuffer, 8, (*m).ntendon, 1_i64) == 0 {
            if allocate != 0 { crate::engine::engine_util_errmem::mju_free(d as *mut ()); }
            crate::engine::engine_util_errmem::mju_warning(b"Invalid data: ten_velocity too large. ".as_ptr() as *const i8);
            return;
        }
        if safe_add_to_buffer_size(&mut offset, &mut (*d).nbuffer, 8, (*m).nu, 1_i64) == 0 {
            if allocate != 0 { crate::engine::engine_util_errmem::mju_free(d as *mut ()); }
            crate::engine::engine_util_errmem::mju_warning(b"Invalid data: actuator_velocity too large. ".as_ptr() as *const i8);
            return;
        }
        if safe_add_to_buffer_size(&mut offset, &mut (*d).nbuffer, 8, (*m).nbody, 6_i64) == 0 {
            if allocate != 0 { crate::engine::engine_util_errmem::mju_free(d as *mut ()); }
            crate::engine::engine_util_errmem::mju_warning(b"Invalid data: cvel too large. ".as_ptr() as *const i8);
            return;
        }
        if safe_add_to_buffer_size(&mut offset, &mut (*d).nbuffer, 8, (*m).nv, 6_i64) == 0 {
            if allocate != 0 { crate::engine::engine_util_errmem::mju_free(d as *mut ()); }
            crate::engine::engine_util_errmem::mju_warning(b"Invalid data: cdof_dot too large. ".as_ptr() as *const i8);
            return;
        }
        if safe_add_to_buffer_size(&mut offset, &mut (*d).nbuffer, 8, (*m).nv, 1_i64) == 0 {
            if allocate != 0 { crate::engine::engine_util_errmem::mju_free(d as *mut ()); }
            crate::engine::engine_util_errmem::mju_warning(b"Invalid data: qfrc_bias too large. ".as_ptr() as *const i8);
            return;
        }
        if safe_add_to_buffer_size(&mut offset, &mut (*d).nbuffer, 8, (*m).nv, 1_i64) == 0 {
            if allocate != 0 { crate::engine::engine_util_errmem::mju_free(d as *mut ()); }
            crate::engine::engine_util_errmem::mju_warning(b"Invalid data: qfrc_spring too large. ".as_ptr() as *const i8);
            return;
        }
        if safe_add_to_buffer_size(&mut offset, &mut (*d).nbuffer, 8, (*m).nv, 1_i64) == 0 {
            if allocate != 0 { crate::engine::engine_util_errmem::mju_free(d as *mut ()); }
            crate::engine::engine_util_errmem::mju_warning(b"Invalid data: qfrc_damper too large. ".as_ptr() as *const i8);
            return;
        }
        if safe_add_to_buffer_size(&mut offset, &mut (*d).nbuffer, 8, (*m).nv, 1_i64) == 0 {
            if allocate != 0 { crate::engine::engine_util_errmem::mju_free(d as *mut ()); }
            crate::engine::engine_util_errmem::mju_warning(b"Invalid data: qfrc_gravcomp too large. ".as_ptr() as *const i8);
            return;
        }
        if safe_add_to_buffer_size(&mut offset, &mut (*d).nbuffer, 8, (*m).nv, 1_i64) == 0 {
            if allocate != 0 { crate::engine::engine_util_errmem::mju_free(d as *mut ()); }
            crate::engine::engine_util_errmem::mju_warning(b"Invalid data: qfrc_fluid too large. ".as_ptr() as *const i8);
            return;
        }
        if safe_add_to_buffer_size(&mut offset, &mut (*d).nbuffer, 8, (*m).nv, 1_i64) == 0 {
            if allocate != 0 { crate::engine::engine_util_errmem::mju_free(d as *mut ()); }
            crate::engine::engine_util_errmem::mju_warning(b"Invalid data: qfrc_passive too large. ".as_ptr() as *const i8);
            return;
        }
        if safe_add_to_buffer_size(&mut offset, &mut (*d).nbuffer, 8, (*m).nbody, 3_i64) == 0 {
            if allocate != 0 { crate::engine::engine_util_errmem::mju_free(d as *mut ()); }
            crate::engine::engine_util_errmem::mju_warning(b"Invalid data: subtree_linvel too large. ".as_ptr() as *const i8);
            return;
        }
        if safe_add_to_buffer_size(&mut offset, &mut (*d).nbuffer, 8, (*m).nbody, 3_i64) == 0 {
            if allocate != 0 { crate::engine::engine_util_errmem::mju_free(d as *mut ()); }
            crate::engine::engine_util_errmem::mju_warning(b"Invalid data: subtree_angmom too large. ".as_ptr() as *const i8);
            return;
        }
        if safe_add_to_buffer_size(&mut offset, &mut (*d).nbuffer, 8, (*m).nC, 1_i64) == 0 {
            if allocate != 0 { crate::engine::engine_util_errmem::mju_free(d as *mut ()); }
            crate::engine::engine_util_errmem::mju_warning(b"Invalid data: qH too large. ".as_ptr() as *const i8);
            return;
        }
        if safe_add_to_buffer_size(&mut offset, &mut (*d).nbuffer, 8, (*m).nv, 1_i64) == 0 {
            if allocate != 0 { crate::engine::engine_util_errmem::mju_free(d as *mut ()); }
            crate::engine::engine_util_errmem::mju_warning(b"Invalid data: qHDiagInv too large. ".as_ptr() as *const i8);
            return;
        }
        if safe_add_to_buffer_size(&mut offset, &mut (*d).nbuffer, 8, (*m).nD, 1_i64) == 0 {
            if allocate != 0 { crate::engine::engine_util_errmem::mju_free(d as *mut ()); }
            crate::engine::engine_util_errmem::mju_warning(b"Invalid data: qDeriv too large. ".as_ptr() as *const i8);
            return;
        }
        if safe_add_to_buffer_size(&mut offset, &mut (*d).nbuffer, 8, (*m).nD, 1_i64) == 0 {
            if allocate != 0 { crate::engine::engine_util_errmem::mju_free(d as *mut ()); }
            crate::engine::engine_util_errmem::mju_warning(b"Invalid data: qLU too large. ".as_ptr() as *const i8);
            return;
        }
        if safe_add_to_buffer_size(&mut offset, &mut (*d).nbuffer, 8, (*m).nu, 1_i64) == 0 {
            if allocate != 0 { crate::engine::engine_util_errmem::mju_free(d as *mut ()); }
            crate::engine::engine_util_errmem::mju_warning(b"Invalid data: actuator_force too large. ".as_ptr() as *const i8);
            return;
        }
        if safe_add_to_buffer_size(&mut offset, &mut (*d).nbuffer, 8, (*m).nv, 1_i64) == 0 {
            if allocate != 0 { crate::engine::engine_util_errmem::mju_free(d as *mut ()); }
            crate::engine::engine_util_errmem::mju_warning(b"Invalid data: qfrc_actuator too large. ".as_ptr() as *const i8);
            return;
        }
        if safe_add_to_buffer_size(&mut offset, &mut (*d).nbuffer, 8, (*m).nv, 1_i64) == 0 {
            if allocate != 0 { crate::engine::engine_util_errmem::mju_free(d as *mut ()); }
            crate::engine::engine_util_errmem::mju_warning(b"Invalid data: qfrc_smooth too large. ".as_ptr() as *const i8);
            return;
        }
        if safe_add_to_buffer_size(&mut offset, &mut (*d).nbuffer, 8, (*m).nv, 1_i64) == 0 {
            if allocate != 0 { crate::engine::engine_util_errmem::mju_free(d as *mut ()); }
            crate::engine::engine_util_errmem::mju_warning(b"Invalid data: qacc_smooth too large. ".as_ptr() as *const i8);
            return;
        }
        if safe_add_to_buffer_size(&mut offset, &mut (*d).nbuffer, 8, (*m).nv, 1_i64) == 0 {
            if allocate != 0 { crate::engine::engine_util_errmem::mju_free(d as *mut ()); }
            crate::engine::engine_util_errmem::mju_warning(b"Invalid data: qfrc_constraint too large. ".as_ptr() as *const i8);
            return;
        }
        if safe_add_to_buffer_size(&mut offset, &mut (*d).nbuffer, 8, (*m).nv, 1_i64) == 0 {
            if allocate != 0 { crate::engine::engine_util_errmem::mju_free(d as *mut ()); }
            crate::engine::engine_util_errmem::mju_warning(b"Invalid data: qfrc_inverse too large. ".as_ptr() as *const i8);
            return;
        }
        if safe_add_to_buffer_size(&mut offset, &mut (*d).nbuffer, 8, (*m).nbody, 6_i64) == 0 {
            if allocate != 0 { crate::engine::engine_util_errmem::mju_free(d as *mut ()); }
            crate::engine::engine_util_errmem::mju_warning(b"Invalid data: cacc too large. ".as_ptr() as *const i8);
            return;
        }
        if safe_add_to_buffer_size(&mut offset, &mut (*d).nbuffer, 8, (*m).nbody, 6_i64) == 0 {
            if allocate != 0 { crate::engine::engine_util_errmem::mju_free(d as *mut ()); }
            crate::engine::engine_util_errmem::mju_warning(b"Invalid data: cfrc_int too large. ".as_ptr() as *const i8);
            return;
        }
        if safe_add_to_buffer_size(&mut offset, &mut (*d).nbuffer, 8, (*m).nbody, 6_i64) == 0 {
            if allocate != 0 { crate::engine::engine_util_errmem::mju_free(d as *mut ()); }
            crate::engine::engine_util_errmem::mju_warning(b"Invalid data: cfrc_ext too large. ".as_ptr() as *const i8);
            return;
        }

        // copy stack size from model
        (*d).narena = (*m).narena;

        // allocate buffer
        (*d).buffer = crate::engine::engine_util_errmem::mju_malloc((*d).nbuffer as usize);
        if (*d).buffer.is_null() {
            if allocate != 0 { crate::engine::engine_util_errmem::mju_free(d as *mut ()); }
            crate::engine::engine_util_errmem::mju_error(
                b"could not allocate mjData buffer\0".as_ptr() as *const i8);
            return;
        }

        // allocate arena
        (*d).arena = crate::engine::engine_util_errmem::mju_malloc((*d).narena as usize);
        if (*d).arena.is_null() {
            crate::engine::engine_util_errmem::mju_free((*d).buffer);
            if allocate != 0 { crate::engine::engine_util_errmem::mju_free(d as *mut ()); }
            crate::engine::engine_util_errmem::mju_error(
                b"could not allocate mjData arena\0".as_ptr() as *const i8);
            return;
        }

        // set pointers into buffer
        mj_set_ptr_data(m, d);

        // clear threadpool
        (*d).threadpool = 0;
        (*d).threadlock = false;

        // clear nplugin
        (*d).nplugin = 0;

        // set awake array sizes to default (all awake)
        (*d).ntree_awake = (*m).ntree as i32;
        (*d).nbody_awake = (*m).nbody as i32;
        (*d).nparent_awake = (*m).nbody as i32;
        (*d).nv_awake = (*m).nv as i32;

        // copy pointer if allocated here
        if allocate != 0 {
            *dest = d;
        }
    }
}

/// C: mj_copyData (engine/engine_io.h:120)
/// Calls: mj_copyDataVisual
#[allow(unused_variables, non_snake_case)]
pub fn mj_copy_data(dest: *mut mjData, m: *const mjModel, src: *const mjData) -> *mut mjData {
    todo!() // mj_copyData
}

/// C: mjv_copyData (engine/engine_io.h:123)
/// Calls: mj_copyDataVisual
#[allow(unused_variables, non_snake_case)]
pub fn mjv_copy_data(dest: *mut mjData, m: *const mjModel, src: *const mjData) -> *mut mjData {
    todo!() // mjv_copyData
}

/// C: mj_resetData (engine/engine_io.h:126)
/// Calls: _resetData, mj_logTimingDiagnostics
#[allow(unused_variables, non_snake_case)]
pub fn mj_reset_data(m: *const mjModel, d: *mut mjData) {
    todo!() // mj_resetData
}

/// C: mj_resetDataDebug (engine/engine_io.h:129)
/// Calls: _resetData
#[allow(unused_variables, non_snake_case)]
pub fn mj_reset_data_debug(m: *const mjModel, d: *mut mjData, debug_value: u8) {
    todo!() // mj_resetDataDebug
}

/// C: mj_resetDataKeyframe (engine/engine_io.h:132)
/// Calls: _resetData, mju_copy
#[allow(unused_variables, non_snake_case)]
pub fn mj_reset_data_keyframe(m: *const mjModel, d: *mut mjData, key: i32) {
    todo!() // mj_resetDataKeyframe
}

/// C: mj_initPlugin (engine/engine_io.h:135)
/// Calls: mjp_getPluginAtSlot, mju_free, mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_init_plugin(m: *const mjModel, d: *mut mjData) {
    // SAFETY: m and d are valid pointers; plugin arrays are allocated (caller contract)
    unsafe {
        (*d).nplugin = (*m).nplugin as i32;
        for i in 0..(*m).nplugin as usize {
            *(*d).plugin.add(i) = *(*m).plugin.add(i);
            let plugin = crate::engine::engine_plugin::mjp_get_plugin_at_slot(
                *(*m).plugin.add(i));
            if let Some(init_fn) = (*plugin).init {
                // SAFETY: init is actually fn(*const mjModel, *mut mjData, i32) -> i32
                let init: unsafe extern "C" fn(*const mjModel, *mut mjData, i32) -> i32 =
                    std::mem::transmute(init_fn);
                if init(m, d, i as i32) < 0 {
                    crate::engine::engine_util_errmem::mju_free((*d).buffer);
                    crate::engine::engine_util_errmem::mju_free((*d).arena);
                    crate::engine::engine_util_errmem::mju_free(d as *mut ());
                    crate::engine::engine_util_errmem::mju_error(
                        b"plugin->init failed for plugin id %d\0".as_ptr() as *const i8);
                    return;
                }
            }
        }
    }
}

/// C: mj_deleteData (engine/engine_io.h:138)
/// Calls: freeDataBuffers, mju_free, mju_threadpool
#[allow(unused_variables, non_snake_case)]
pub fn mj_delete_data(d: *mut mjData) {
    todo!() // mj_deleteData
}

