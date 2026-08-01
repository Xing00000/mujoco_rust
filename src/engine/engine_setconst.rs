//! Port of: engine/engine_setconst.c
//! IR hash: 9343293228317031
//! CODEGEN: source paths, owners, and callable names are locked.

use crate::types::*;

/// C: GetWrapBodyTreeId (engine/engine_setconst.c:64)
#[allow(unused_variables, non_snake_case)]
pub fn GetWrapBodyTreeId(m: *const mjModel, wrap_index: i32) -> i32 {
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

/// C: makeTendonSparse (engine/engine_setconst.c:337)
/// Calls: cxx:_mju_zeroInt
#[allow(unused_variables, non_snake_case)]
pub fn makeTendonSparse(m: *mut mjModel) {
    use crate::types::*;
    const MJ_WRAP_JOINT: i32 = mjtWrap_mjWRAP_JOINT as i32;
    const MJ_WRAP_SITE: i32 = mjtWrap_mjWRAP_SITE as i32;
    const MJ_WRAP_SPHERE: i32 = mjtWrap_mjWRAP_SPHERE as i32;
    const MJ_WRAP_CYLINDER: i32 = mjtWrap_mjWRAP_CYLINDER as i32;

    // SAFETY: m is a valid mjModel pointer. All field accesses follow the struct layout.
    unsafe {
        let ntendon = (*m).ntendon as i32;
        if ntendon == 0 { return; }

        let rownnz = (*m).ten_J_rownnz;
        let rowadr = (*m).ten_J_rowadr;
        let colind = (*m).ten_J_colind;

        crate::engine::engine_util_misc::mju_zeroInt(rownnz, ntendon);
        crate::engine::engine_util_misc::mju_zeroInt(rowadr, ntendon);

        for i in 0..ntendon as usize {
            *rowadr.add(i) = if i > 0 {
                *rowadr.add(i - 1) + *rownnz.add(i - 1)
            } else { 0 };

            let adr = *(*m).tendon_adr.add(i) as usize;
            let num = *(*m).tendon_num.add(i) as usize;

            // joint tendon: each wrap object is a joint, colind is its dofadr
            if *(*m).wrap_type.add(adr) == MJ_WRAP_JOINT {
                for j in 0..num {
                    let jnt_id = *(*m).wrap_objid.add(adr + j) as usize;
                    *colind.add(*rowadr.add(i) as usize + j) = *(*m).jnt_dofadr.add(jnt_id);
                }
                *rownnz.add(i) = num as i32;
            } else {
                // spatial tendon: collect used dofs from wrap object bodies
                let mut nnz: i32 = 0;
                for j in 0..num {
                    let wrap_t = *(*m).wrap_type.add(adr + j);
                    let obj_id = *(*m).wrap_objid.add(adr + j) as usize;

                    let bodyid = if wrap_t == MJ_WRAP_SITE {
                        *(*m).site_bodyid.add(obj_id)
                    } else if wrap_t == MJ_WRAP_SPHERE || wrap_t == MJ_WRAP_CYLINDER {
                        *(*m).geom_bodyid.add(obj_id)
                    } else {
                        -1
                    };

                    if bodyid > 0 {
                        let mut bid = bodyid;
                        while bid > 0 {
                            let bdofadr = *(*m).body_dofadr.add(bid as usize);
                            let bdofnum = *(*m).body_dofnum.add(bid as usize);
                            for k in 0..bdofnum {
                                let dof = bdofadr + k;
                                let base = *rowadr.add(i) as usize;
                                let mut found = false;
                                for l in 0..nnz as usize {
                                    if *colind.add(base + l) == dof { found = true; break; }
                                }
                                if !found {
                                    *colind.add(base + nnz as usize) = dof;
                                    nnz += 1;
                                }
                            }
                            bid = *(*m).body_parentid.add(bid as usize);
                        }
                    }
                }
                *rownnz.add(i) = nnz;

                // bubble sort colind for this tendon
                let base = *rowadr.add(i) as usize;
                for j in 0..(nnz as usize - 1).saturating_add(1) {
                    for k in (j + 1)..(nnz as usize) {
                        if *colind.add(base + k) < *colind.add(base + j) {
                            let tmp = *colind.add(base + j);
                            *colind.add(base + j) = *colind.add(base + k);
                            *colind.add(base + k) = tmp;
                        }
                    }
                }
            }
        }
    }
}

/// C: updateBox (engine/engine_setconst.c:1041)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn updateBox(xmin: *mut f64, xmax: *mut f64, pos: *mut f64, radius: f64) {
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
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_setconst.c:_updateBox, cxx:_mj_freeStack, cxx:_mj_markStack, cxx:_mj_stackAllocInfo, cxx:_mju_add3, cxx:_mju_dist3, cxx:_mju_max, cxx:_mju_scl3, cxx:_mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn setStat(m: *mut mjModel, d: *mut mjData) {
    // SAFETY: m and d are valid pointers with all arrays allocated (caller contract)
    unsafe {
        let mut xmin = [1e10f64; 3];
        let mut xmax = [-1e10f64; 3];
        let mut rbound: f64;
        crate::engine::engine_memory::mj_markStack(d);

        // approximate length associated with each body
        let body = crate::engine::engine_memory::mj_stackAllocNum(d, (*m).nbody as usize);

        // compute bounding box of bodies, joint centers, geoms and sites
        for i in 1..(*m).nbody as usize {
            updateBox(xmin.as_mut_ptr(), xmax.as_mut_ptr(), (*d).xpos.add(3 * i), 0.0);
            updateBox(xmin.as_mut_ptr(), xmax.as_mut_ptr(), (*d).xipos.add(3 * i), 0.0);
        }
        for i in 0..(*m).njnt as usize {
            updateBox(xmin.as_mut_ptr(), xmax.as_mut_ptr(), (*d).xanchor.add(3 * i), 0.0);
        }
        for i in 0..(*m).nsite as usize {
            updateBox(xmin.as_mut_ptr(), xmax.as_mut_ptr(), (*d).site_xpos.add(3 * i), 0.0);
        }
        for i in 0..(*m).ngeom as usize {
            // set rbound: regular geom rbound, or 0.1 of plane or hfield max size
            rbound = 0.0;
            if *(*m).geom_rbound.add(i) > 0.0 {
                rbound = *(*m).geom_rbound.add(i);
            } else if *(*m).geom_type.add(i) == mjtGeom_mjGEOM_PLANE as i32 {
                // finite in at least one direction
                if *(*m).geom_size.add(3 * i) != 0.0 || *(*m).geom_size.add(3 * i + 1) != 0.0 {
                    rbound = crate::engine::engine_util_misc::mju_max(
                        *(*m).geom_size.add(3 * i),
                        *(*m).geom_size.add(3 * i + 1)) * 0.1;
                } else {
                    // infinite in both directions
                    rbound = 0.01;
                }
            } else if *(*m).geom_type.add(i) == mjtGeom_mjGEOM_HFIELD as i32 {
                let j = *(*m).geom_dataid.add(i) as usize;
                rbound = crate::engine::engine_util_misc::mju_max(
                    *(*m).hfield_size.add(4 * j),
                    crate::engine::engine_util_misc::mju_max(
                        *(*m).hfield_size.add(4 * j + 1),
                        crate::engine::engine_util_misc::mju_max(
                            *(*m).hfield_size.add(4 * j + 2),
                            *(*m).hfield_size.add(4 * j + 3)))) * 0.1;
            }

            updateBox(xmin.as_mut_ptr(), xmax.as_mut_ptr(), (*d).geom_xpos.add(3 * i), rbound);
        }

        // compute center
        crate::engine::engine_util_blas::mju_add3(
            (*m).stat.center.as_mut_ptr(), xmin.as_ptr(), xmax.as_ptr());
        crate::engine::engine_util_blas::mju_scl3(
            (*m).stat.center.as_mut_ptr(), (*m).stat.center.as_ptr(), 0.5);

        // compute bounding box size
        if xmax[0] > xmin[0] {
            (*m).stat.extent = crate::engine::engine_util_misc::mju_max(
                1e-5,
                crate::engine::engine_util_misc::mju_max(
                    xmax[0] - xmin[0],
                    crate::engine::engine_util_misc::mju_max(
                        xmax[1] - xmin[1], xmax[2] - xmin[2])));
        }

        // set body size to max com-joint distance
        crate::engine::engine_util_blas::mju_zero(body, (*m).nbody as i32);
        for i in 0..(*m).njnt as usize {
            // handle this body
            let id = *(*m).jnt_bodyid.add(i) as usize;
            *body.add(id) = crate::engine::engine_util_misc::mju_max(
                *body.add(id),
                crate::engine::engine_util_blas::mju_dist3(
                    (*d).xipos.add(3 * id), (*d).xanchor.add(3 * i)));

            // handle parent body
            let id = *(*m).body_parentid.add(id) as usize;
            *body.add(id) = crate::engine::engine_util_misc::mju_max(
                *body.add(id),
                crate::engine::engine_util_blas::mju_dist3(
                    (*d).xipos.add(3 * id), (*d).xanchor.add(3 * i)));
        }
        *body.add(0) = 0.0;

        // set body size to max of old value, and geom rbound + com-geom dist
        for i in 1..(*m).nbody as usize {
            let gadr = *(*m).body_geomadr.add(i) as usize;
            let gnum = *(*m).body_geomnum.add(i) as usize;
            for id in gadr..gadr + gnum {
                if *(*m).geom_rbound.add(id) > 0.0 {
                    *body.add(i) = crate::engine::engine_util_misc::mju_max(
                        *body.add(i),
                        *(*m).geom_rbound.add(id) + crate::engine::engine_util_blas::mju_dist3(
                            (*d).xipos.add(3 * i), (*d).geom_xpos.add(3 * id)));
                }
            }
        }

        // adjust body size for flex edges involving body
        for f in 0..(*m).nflex as usize {
            if *(*m).flex_interp.add(f) != 0 {
                let nadr = *(*m).flex_nodeadr.add(f) as usize;
                let nnum = *(*m).flex_nodenum.add(f) as usize;
                for v1 in nadr..nadr + nnum {
                    for v2 in nadr..nadr + nnum {
                        let edge = crate::engine::engine_util_blas::mju_dist3(
                            (*d).xpos.add(3 * *(*m).flex_nodebodyid.add(v1) as usize),
                            (*d).xpos.add(3 * *(*m).flex_nodebodyid.add(v2) as usize));
                        let bid = *(*m).flex_nodebodyid.add(v1) as usize;
                        *body.add(bid) = crate::engine::engine_util_misc::mju_max(
                            *body.add(bid), edge);
                    }
                }
                continue;
            }
            let eadr = *(*m).flex_edgeadr.add(f) as usize;
            let enm = *(*m).flex_edgenum.add(f) as usize;
            for e in eadr..eadr + enm {
                let vadr = *(*m).flex_vertadr.add(f) as usize;
                let b1 = *(*m).flex_vertbodyid.add(vadr + *(*m).flex_edge.add(2 * e) as usize) as usize;
                let b2 = *(*m).flex_vertbodyid.add(vadr + *(*m).flex_edge.add(2 * e + 1) as usize) as usize;

                *body.add(b1) = crate::engine::engine_util_misc::mju_max(
                    *body.add(b1), *(*m).flexedge_length0.add(e));
                *body.add(b2) = crate::engine::engine_util_misc::mju_max(
                    *body.add(b2), *(*m).flexedge_length0.add(e));
            }
        }

        // compute meansize, make sure all sizes are above min
        if (*m).nbody > 1 {
            (*m).stat.meansize = 0.0;
            for i in 1..(*m).nbody as usize {
                *body.add(i) = crate::engine::engine_util_misc::mju_max(*body.add(i), 1e-5);
                (*m).stat.meansize += *body.add(i) / ((*m).nbody - 1) as f64;
            }
        }

        // inherit dof length from parent body
        for i in 0..(*m).nv as usize {
            // default to linear dof, already has length units
            *(*m).dof_length.add(i) = 1.0;

            // if rotational dof, inherit from body
            let jnt = *(*m).dof_jntid.add(i) as usize;
            let jtype = *(*m).jnt_type.add(jnt);
            let offset = i as i32 - *(*m).jnt_dofadr.add(jnt);
            if jtype == mjtJoint_mjJNT_BALL as i32
                || jtype == mjtJoint_mjJNT_HINGE as i32
                || (jtype == mjtJoint_mjJNT_FREE as i32 && offset >= 3)
            {
                *(*m).dof_length.add(i) = *body.add(*(*m).dof_bodyid.add(i) as usize);
            }
        }

        // fix extent if too small compared to meanbody
        (*m).stat.extent = crate::engine::engine_util_misc::mju_max(
            (*m).stat.extent, 2.0 * (*m).stat.meansize);

        // compute meanmass
        if (*m).nbody > 1 {
            (*m).stat.meanmass = 0.0;
            for i in 1..(*m).nbody as usize {
                (*m).stat.meanmass += *(*m).body_mass.add(i);
            }
            (*m).stat.meanmass /= ((*m).nbody - 1) as f64;
        }

        // compute meaninertia
        if (*m).nv > 0 {
            (*m).stat.meaninertia = 0.0;
            for i in 0..(*m).nv as usize {
                (*m).stat.meaninertia += *(*d).M.add(
                    (*(*m).M_rowadr.add(i) + *(*m).M_rownnz.add(i) - 1) as usize);
            }
            (*m).stat.meaninertia /= (*m).nv as f64;
        }

        crate::engine::engine_memory::mj_freeStack(d);
    }
}

