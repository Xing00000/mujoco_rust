//! Port of: engine/engine_forward.c
//! IR hash: 73a9f665ec0ecfc0
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: dcmotorVoltage (engine/engine_forward.c:222)
/// Calls: mju_clip
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn dcmotor_voltage(ctrl: f64, length: f64, velocity: f64, x_I: f64, gainprm: *const f64) -> f64 {
    use crate::engine::engine_util_misc::mju_clip;
    // SAFETY: gainprm points to at least 9 f64 elements (caller contract)
    unsafe {
        let input_mode = *gainprm.add(8) as i32;
        let Vmax = *gainprm.add(7);
        let mut voltage;
        if input_mode > 0 {
            let kp = *gainprm.add(4);
            let ki = *gainprm.add(5);
            let kd = *gainprm.add(6);
            if input_mode == 1 {
                voltage = kp * (ctrl - length) + ki * x_I - kd * velocity;
            } else {
                voltage = kp * (ctrl - velocity) + ki * (x_I - length);
            }
        } else {
            voltage = ctrl;
        }
        if Vmax > 0.0 {
            voltage = mju_clip(voltage, -Vmax, Vmax);
        }
        voltage
    }
}

/// C: clampVec (engine/engine_forward.c:253)
/// Calls: mju_clip
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn clamp_vec(vec: *mut f64, range: *const f64, limited: *const bool, n: i32, index: *const i32) {
    // SAFETY: caller guarantees vec, range, limited point to arrays of at least n elements,
    // and index (if non-null) also has at least n elements. All pointers are valid.
    unsafe {
        for i in 0..n as usize {
            let j = if !index.is_null() {
                *index.add(i) as usize
            } else {
                i
            };
            if *limited.add(i) {
                *vec.add(j) = crate::engine::engine_util_misc::mju_clip(
                    *vec.add(j),
                    *range.add(2 * i),
                    *range.add(2 * i + 1),
                );
            }
        }
    }
}

/// C: warmstart (engine/engine_forward.c:786)
/// Calls: mj_constraintUpdate, mj_freeStack, mj_isSparse, mj_markStack, mj_mulJacVec, mj_mulM, mj_stackAllocInfo, mju_copy, mju_dot, mju_mulMatVec, mju_mulMatVecSparse, mju_subFrom, mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn warmstart(m: *const mjModel, d: *mut mjData) {
    const mjDSBL_WARMSTART: i32 = 1 << 9;
    const mjSOL_PGS: i32 = 0;

    // SAFETY: m, d are valid pointers (caller contract).
    unsafe {
        let nv = (*m).nv as i32;
        let nefc = (*d).nefc;

        // warmstart with best of (qacc_warmstart, qacc_smooth)
        if ((*m).opt.disableflags & mjDSBL_WARMSTART) == 0 {
            crate::engine::engine_memory::mj_mark_stack(d);
            let jar: *mut f64 = crate::engine::engine_memory::mj_stack_alloc_num(d, nefc as usize);

            // start with qacc = qacc_warmstart
            crate::engine::engine_util_blas::mju_copy(
                (*d).qacc, (*d).qacc_warmstart as *const f64, nv);

            // compute jar(qacc_warmstart)
            crate::engine::engine_core_constraint::mj_mul_jac_vec(
                m, d as *const crate::types::mjData, jar, (*d).qacc_warmstart as *const f64);
            crate::engine::engine_util_blas::mju_sub_from(jar, (*d).efc_aref as *const f64, nefc);

            // update constraints, save cost(qacc_warmstart)
            let mut cost_warmstart: f64 = 0.0;
            crate::engine::engine_core_constraint::mj_constraint_update(
                m, d, jar as *const f64, &mut cost_warmstart as *mut f64, 0);

            // PGS
            if (*m).opt.solver == mjSOL_PGS {
                // cost(force_warmstart)
                let mut PGS_warmstart: f64 = crate::engine::engine_util_blas::mju_dot(
                    (*d).efc_force as *const f64, (*d).efc_b as *const f64, nefc);
                let ARf: *mut f64 = crate::engine::engine_memory::mj_stack_alloc_num(d, nefc as usize);
                if crate::engine::engine_core_util::mj_is_sparse(m) != 0 {
                    crate::engine::engine_util_sparse::mju_mul_mat_vec_sparse(
                        ARf, (*d).efc_AR as *const f64, (*d).efc_force as *const f64,
                        nefc, (*d).efc_AR_rownnz as *const i32,
                        (*d).efc_AR_rowadr as *const i32,
                        (*d).efc_AR_colind as *const i32, std::ptr::null());
                } else {
                    crate::engine::engine_util_blas::mju_mul_mat_vec(
                        ARf, (*d).efc_AR as *const f64, (*d).efc_force as *const f64,
                        nefc, nefc);
                }
                PGS_warmstart += 0.5 * crate::engine::engine_util_blas::mju_dot(
                    (*d).efc_force as *const f64, ARf as *const f64, nefc);

                // use zero if better
                if PGS_warmstart > 0.0 {
                    crate::engine::engine_util_blas::mju_zero((*d).efc_force, nefc);
                    crate::engine::engine_util_blas::mju_zero((*d).qfrc_constraint, nv);
                }
            }
            // non-PGS
            else {
                // add Gauss to cost(qacc_warmstart)
                let Ma: *mut f64 = crate::engine::engine_memory::mj_stack_alloc_num(d, nv as usize);
                crate::engine::engine_support::mj_mul_m(
                    m, d as *const crate::types::mjData, Ma, (*d).qacc_warmstart as *const f64);
                for i in 0..nv {
                    cost_warmstart += 0.5
                        * (*Ma.add(i as usize) - *(*d).qfrc_smooth.add(i as usize))
                        * (*(*d).qacc_warmstart.add(i as usize) - *(*d).qacc_smooth.add(i as usize));
                }

                // cost(qacc_smooth)
                let mut cost_smooth: f64 = 0.0;
                crate::engine::engine_core_constraint::mj_constraint_update(
                    m, d, (*d).efc_b as *const f64, &mut cost_smooth as *mut f64, 0);

                // use qacc_smooth if better
                if cost_warmstart > cost_smooth {
                    crate::engine::engine_util_blas::mju_copy(
                        (*d).qacc, (*d).qacc_smooth as *const f64, nv);
                }
            }

            // have island structure: unconstrained qacc = qacc_smooth
            if (*d).nisland > 0 {
                // loop over unconstrained dofs in map_idof2dof[nidof, nv)
                for i in (*d).nidof..nv {
                    let dof = *(*d).map_idof2dof.add(i as usize);
                    *(*d).qacc.add(dof as usize) = *(*d).qacc_smooth.add(dof as usize);
                }
            }

            crate::engine::engine_memory::mj_free_stack(d);
        }
        // coldstart with qacc = qacc_smooth, efc_force = 0
        else {
            crate::engine::engine_util_blas::mju_copy(
                (*d).qacc, (*d).qacc_smooth as *const f64, nv);
            crate::engine::engine_util_blas::mju_zero((*d).efc_force, nefc);
        }
    }
}

/// C: solveIslandTask (engine/engine_forward.c:866)
/// Calls: mj_solCG_island, mj_solNewton_island, mj_solPGS_island
#[allow(unused_variables, non_snake_case)]
pub fn solve_island_task(m: *const mjModel, d: *mut mjData, arg: *mut (), thread_id: i32, island: i32) {
    const MJ_SOL_PGS: i32 = 0;
    const MJ_SOL_CG: i32 = 1;
    const MJ_SOL_NEWTON: i32 = 2;

    // SAFETY: m, d are valid pointers (caller contract).
    unsafe {
        if (*m).opt.solver == MJ_SOL_NEWTON {
            crate::engine::engine_solver::mj_sol_newton_island(m, d, island, (*m).opt.iterations);
        } else if (*m).opt.solver == MJ_SOL_CG {
            crate::engine::engine_solver::mj_sol_cg_island(m, d, island, (*m).opt.iterations);
        } else {
            crate::engine::engine_solver::mj_sol_pgs_island(m, d, island, (*m).opt.iterations);
        }
    }
}

/// C: mj_advance (engine/engine_forward.c:981)
/// Calls: mj_actuatorDisabled, mj_computeSensor, mj_forwardSkip, mj_integratePosInd, mj_nextActivation, mj_sleep, mj_updateSleep, mjp_getPluginAtSlotUnsafe, mjp_pluginCount, mju_addToScl, mju_addToSclInd, mju_copy, mju_historyInsert, mju_message
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_advance(m: *const mjModel, d: *mut mjData, act_dot: *const f64, qacc: *const f64, qvel: *const f64) {
    const mjDSBL_ACTUATION: i32 = 1 << 11;
    const mjENBL_SLEEP: i32 = 1 << 4;
    const mjSTAGE_POS: i32 = 1;

    // SAFETY: m, d are valid pointers (caller contract). All array accesses within bounds.
    unsafe {
        let nu = (*m).nu as i32;
        let nsensor = (*m).nsensor as i32;

        // advance history buffers
        if (*m).nhistory > 0 {
            // advance ctrl history buffers
            for i in 0..nu {
                let nsample = *(*m).actuator_history.add((2 * i) as usize);
                if nsample == 0 {
                    continue;
                }

                // get history buffer pointer and insert ctrl at current time
                let buf: *mut f64 = (*d).history.add(*(*m).actuator_historyadr.add(i as usize) as usize);
                let slot = crate::engine::engine_util_misc::mju_history_insert(
                    buf, nsample, 1, (*d).time);
                *slot = *(*d).ctrl.add(i as usize);
            }

            // advance sensor history buffers
            for i in 0..nsensor {
                let nsample = *(*m).sensor_history.add((2 * i) as usize);
                if nsample == 0 {
                    continue;
                }

                // get history buffer parameters
                let dim = *(*m).sensor_dim.add(i as usize);
                let buf: *mut f64 = (*d).history.add(*(*m).sensor_historyadr.add(i as usize) as usize);
                let delay = *(*m).sensor_delay.add(i as usize);
                let interval = *(*m).sensor_interval.add((2 * i) as usize);

                if interval > 0.0 {
                    // interval mode: if condition is satisfied, compute; otherwise copy
                    let time_prev = *buf; // first slot stores previous sensor tick
                    if time_prev + interval <= (*d).time {
                        *buf += interval; // advance by exact interval
                        let slot = crate::engine::engine_util_misc::mju_history_insert(
                            buf, nsample, dim, (*d).time);
                        if delay > 0.0 {
                            crate::engine::engine_sensor::mj_compute_sensor(m, d, i, slot);
                        } else {
                            crate::engine::engine_util_blas::mju_copy(
                                slot, (*d).sensordata.add(*(*m).sensor_adr.add(i as usize) as usize) as *const f64, dim);
                        }
                    }
                } else if delay > 0.0 {
                    // delay-only mode: always compute and insert
                    let slot = crate::engine::engine_util_misc::mju_history_insert(
                        buf, nsample, dim, (*d).time);
                    crate::engine::engine_sensor::mj_compute_sensor(m, d, i, slot);
                } else {
                    // history-only mode: copy from sensordata
                    let slot = crate::engine::engine_util_misc::mju_history_insert(
                        buf, nsample, dim, (*d).time);
                    crate::engine::engine_util_blas::mju_copy(
                        slot, (*d).sensordata.add(*(*m).sensor_adr.add(i as usize) as usize) as *const f64, dim);
                }
            }
        }

        // advance activations
        if (*m).na > 0 && ((*m).opt.disableflags & mjDSBL_ACTUATION) == 0 {
            for i in 0..nu {
                let actadr = *(*m).actuator_actadr.add(i as usize);
                let actadr_end = actadr + *(*m).actuator_actnum.add(i as usize);
                for j in actadr..actadr_end {
                    let dot_val = if crate::engine::engine_support::mj_actuator_disabled(m, i) != 0 {
                        0.0
                    } else {
                        *act_dot.add(j as usize)
                    };
                    *(*d).act.add(j as usize) = crate::engine::engine_support::mj_next_activation(
                        m, d as *const crate::types::mjData, i, j, dot_val);
                }
            }
        }

        // put islands to sleep according to velocity tolerance
        if crate::engine::engine_sleep::mj_sleep(m, d) != 0 {
            // if any trees put to sleep, recompute all velocity-dependent quantities
            mj_forward_skip(m, d, mjSTAGE_POS, 0);

            // update sleep indices
            crate::engine::engine_sleep::mj_update_sleep(m, d);
        }

        // advance velocities
        let sleep_filter = (((*m).opt.enableflags & mjENBL_SLEEP) != 0)
            && ((*d).ntree_awake < (*m).ntree as i32);
        if sleep_filter {
            crate::engine::engine_util_blas::mju_add_to_scl_ind(
                (*d).qvel, qacc, (*d).dof_awake_ind as *const i32,
                (*m).opt.timestep, (*d).nv_awake);
        } else {
            crate::engine::engine_util_blas::mju_add_to_scl(
                (*d).qvel, qacc, (*m).opt.timestep, (*m).nv as i32);
        }

        // advance positions with qvel if given, d->qvel otherwise (semi-implicit)
        let index: *const i32 = if sleep_filter { (*d).body_awake_ind as *const i32 } else { std::ptr::null() };
        let nbody = if sleep_filter { (*d).nbody_awake } else { (*m).nbody as i32 };
        crate::engine::engine_support::mj_integrate_pos_ind(
            m, (*d).qpos,
            if !qvel.is_null() { qvel } else { (*d).qvel as *const f64 },
            (*m).opt.timestep, index, nbody);

        // advance time
        (*d).time += (*m).opt.timestep;

        // advance plugin states
        if (*m).nplugin > 0 {
            let nslot = crate::engine::engine_plugin::mjp_plugin_count();
            for i in 0..(*m).nplugin as i32 {
                let slot = *(*m).plugin.add(i as usize);
                let plugin = crate::engine::engine_plugin::mjp_get_plugin_at_slot_unsafe(slot, nslot);
                if plugin.is_null() {
                    crate::engine::engine_util_errmem::mju_error(
                        b"invalid plugin slot: %d\0".as_ptr() as *const i8);
                }
                if let Some(advance_fn) = (*plugin).advance {
                    // SAFETY: plugin->advance has signature (m, d, i) per mujoco API
                    let f: unsafe extern "C" fn(*const crate::types::mjModel, *mut crate::types::mjData, i32) =
                        std::mem::transmute(advance_fn);
                    f(m, d, i);
                }
            }
        }

        // save qacc for next step warmstart
        crate::engine::engine_util_blas::mju_copy(
            (*d).qacc_warmstart, (*d).qacc as *const f64, (*m).nv as i32);
    }
}

/// C: flex_has_implicit_stiffness (engine/engine_forward.c:1284)
#[allow(unused_variables, non_snake_case)]
pub fn flex_has_implicit_stiffness(m: *const mjModel) -> i32 {
    // SAFETY: m is a valid pointer to mjModel (caller contract)
    unsafe {
        for f in 0..(*m).nflex as isize {
            if *(*m).flex_rigid.offset(f) {
                continue;
            }

            // interpolated flex with stiffness
            if *(*m).flex_interp.offset(f) != 0
                && *(*m).flex_edgeequality.offset(f) != 3
                && *(*m).flex_stiffness.offset(*(*m).flex_stiffnessadr.offset(f) as isize) != 0.0
            {
                return 1;
            }

            // standard flex with bending
            if *(*m).flex_interp.offset(f) == 0
                && *(*m).flex_dim.offset(f) == 2
                && *(*m).flex_bendingadr.offset(f) >= 0
            {
                return 1;
            }
        }
        0
    }
}

/// C: flexInterp_cgsolve (engine/engine_forward.c:1311)
/// Calls: mj_freeStack, mj_markStack, mj_solveLD, mj_stackAllocInfo, mjd_flexBend_mul, mjd_flexInterp_cacheKrot, mjd_flexInterp_mul, mju_addScl, mju_addToScl, mju_copy, mju_dot, mju_max, mju_mulMatVecSparse, mju_mulSymVecSparse, mju_solveLUSparse, mju_sub, mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn flex_interp_cgsolve(m: *const mjModel, d: *mut mjData, qacc: *mut f64, qfrc: *const f64, nv: i32) {
    todo!() // flexInterp_cgsolve
}

/// C: midpoint_eligible (engine/engine_forward.c:1421)
/// Calls: mjCMesh::tree
#[allow(unused_variables, non_snake_case)]
pub fn midpoint_eligible(m: *const mjModel, d: *const mjData, jnt: i32) -> i32 {
    // Constants from C enums
    const MJ_JNT_FREE: i32 = 0;
    const MJ_DSBL_ISLAND: i32 = 1 << 18;
    const MJ_CNSTR_EQUALITY: i32 = 0;
    const MJ_CNSTR_LIMIT_TENDON: i32 = 4;
    const MJ_CNSTR_CONTACT_FRICTIONLESS: i32 = 5;
    const MJ_CNSTR_CONTACT_PYRAMIDAL: i32 = 6;
    const MJ_CNSTR_CONTACT_ELLIPTIC: i32 = 7;
    const MJ_CNSTR_FRICTION_TENDON: i32 = 2;
    const MJ_EQ_CONNECT: i32 = 0;
    const MJ_EQ_WELD: i32 = 1;
    const MJ_OBJ_SITE: i32 = 6;

    // SAFETY: m, d are valid mjModel/mjData pointers (caller contract)
    unsafe {
        if *(*m).jnt_type.add(jnt as usize) != MJ_JNT_FREE as i32 {
            return 0;
        }

        let body = *(*m).jnt_bodyid.add(jnt as usize);
        let adr = *(*m).jnt_dofadr.add(jnt as usize);
        let tree = *(*m).dof_treeid.add(adr as usize);

        // must be standalone 6-DOF tree with no children
        if *(*m).tree_dofnum.add(tree as usize) != 6
            || *(*m).body_subtreemass.add(body as usize) != *(*m).body_mass.add(body as usize)
        {
            return 0;
        }

        // must be awake
        if *(*d).tree_awake.add(tree as usize) == 0 {
            return 0;
        }

        // must be unconstrained
        if (*d).nefc != 0 {
            // islands enabled: O(1) lookup
            if ((*m).opt.disableflags & MJ_DSBL_ISLAND) == 0 {
                if *(*d).dof_island.add(adr as usize) >= 0 {
                    return 0;
                }
            } else {
                // islands disabled: check if any constraint involves this tree
                for c in 0..(*d).nefc {
                    let etype = *(*d).efc_type.add(c as usize);
                    let id = *(*d).efc_id.add(c as usize);

                    // contact: check if either geom belongs to this body
                    if etype == MJ_CNSTR_CONTACT_FRICTIONLESS
                        || etype == MJ_CNSTR_CONTACT_PYRAMIDAL
                        || etype == MJ_CNSTR_CONTACT_ELLIPTIC
                    {
                        let g1 = (*(*d).contact.add(id as usize)).geom[0];
                        let g2 = (*(*d).contact.add(id as usize)).geom[1];
                        if g1 >= 0 && *(*m).geom_bodyid.add(g1 as usize) == body {
                            return 0;
                        }
                        if g2 >= 0 && *(*m).geom_bodyid.add(g2 as usize) == body {
                            return 0;
                        }
                    }
                    // connect or weld: check if either body is this body
                    else if etype == MJ_CNSTR_EQUALITY
                        && (*(*m).eq_type.add(id as usize) == MJ_EQ_CONNECT
                            || *(*m).eq_type.add(id as usize) == MJ_EQ_WELD)
                    {
                        let mut b1 = *(*m).eq_obj1id.add(id as usize);
                        let mut b2 = *(*m).eq_obj2id.add(id as usize);
                        if *(*m).eq_objtype.add(id as usize) == MJ_OBJ_SITE {
                            b1 = *(*m).site_bodyid.add(b1 as usize);
                            b2 = *(*m).site_bodyid.add(b2 as usize);
                        }
                        if b1 == body || b2 == body {
                            return 0;
                        }
                    }
                    // tendon limit or friction: check first two trees
                    else if etype == MJ_CNSTR_LIMIT_TENDON
                        || etype == MJ_CNSTR_FRICTION_TENDON
                    {
                        if *(*m).tendon_treeid.add((2 * id) as usize) == tree
                            || *(*m).tendon_treeid.add((2 * id + 1) as usize) == tree
                        {
                            return 0;
                        }
                    }
                }
            }
        }

        // otherwise eligible
        1
    }
}

/// C: midpoint_aligned (engine/engine_forward.c:1493)
#[allow(unused_variables, non_snake_case)]
pub fn midpoint_aligned(m: *const mjModel, jnt: i32) -> i32 {
    // SAFETY: caller guarantees m is valid and jnt is within bounds
    unsafe {
        let body = *(*m).jnt_bodyid.add(jnt as usize);
        let ipos = (*m).body_ipos;
        let aligned = *ipos.add(3 * body as usize + 0) == 0.0
            && *ipos.add(3 * body as usize + 1) == 0.0
            && *ipos.add(3 * body as usize + 2) == 0.0;
        aligned as i32
    }
}

/// C: midpointNewton (engine/engine_forward.c:1515)
/// Calls: mji_copy3, mji_cross, mju_norm3, mju_solve3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn midpoint_newton(inertia: *const f64, w: *const f64, tau: *const f64, h: f64, w_mid: *mut f64) -> i32 {
    use crate::engine::engine_inline::{mji_copy3, mji_cross};
    use crate::engine::engine_util_blas::mju_norm3;
    use crate::engine::engine_util_solve::mju_solve3;

    // SAFETY: all pointers are arrays of 3 f64 passed from caller; h is nonzero by contract
    unsafe {
        // precompute constants
        let i2h: f64 = 2.0 / h;
        let dI: [f64; 3] = [
            *inertia.add(2) - *inertia.add(1),
            *inertia.add(0) - *inertia.add(2),
            *inertia.add(1) - *inertia.add(0),
        ];
        let i2h_I: [f64; 3] = [
            i2h * *inertia.add(0),
            i2h * *inertia.add(1),
            i2h * *inertia.add(2),
        ];

        // initialize solution to previous angular velocity
        mji_copy3(w_mid, w);

        // Newton iteration
        let mut niter: i32 = 0;
        while niter < 100 {
            // compute Coriolis term
            let Iw: [f64; 3] = [
                *inertia.add(0) * *w_mid.add(0),
                *inertia.add(1) * *w_mid.add(1),
                *inertia.add(2) * *w_mid.add(2),
            ];
            let mut coriolis: [f64; 3] = [0.0; 3];
            mji_cross(coriolis.as_mut_ptr(), w_mid, Iw.as_ptr());

            // residual: f = i2h*I*(w_mid - w) + w_mid x (I*w_mid) - tau
            let mut f: [f64; 3] = [0.0; 3];
            for k in 0..3 {
                f[k] = i2h_I[k] * (*w_mid.add(k) - *w.add(k)) + coriolis[k] - *tau.add(k);
            }

            // check convergence
            let fnorm: f64 = mju_norm3(f.as_ptr());
            let tol: f64 = 1e-13;
            if fnorm < tol * (1.0 + i2h * mju_norm3(Iw.as_ptr())) {
                break;
            }

            // Jacobian: J = i2h*diag(I) + d(w x Iw)/dw
            let mut J: [f64; 9] = [0.0; 9];
            J[0] = i2h_I[0];                    J[1] = *w_mid.add(2) * dI[0]; J[2] = *w_mid.add(1) * dI[0];
            J[3] = *w_mid.add(2) * dI[1];       J[4] = i2h_I[1];              J[5] = *w_mid.add(0) * dI[1];
            J[6] = *w_mid.add(1) * dI[2];       J[7] = *w_mid.add(0) * dI[2]; J[8] = i2h_I[2];

            // solve J*delta = -f for search direction delta
            let neg_f: [f64; 3] = [-f[0], -f[1], -f[2]];
            let mut delta: [f64; 3] = [0.0; 3];
            mju_solve3(delta.as_mut_ptr(), J.as_ptr(), neg_f.as_ptr());

            // backtracking line search
            let mut step: f64 = 1.0;
            for _ls in 0..20 {
                let mut w_try: [f64; 3] = [0.0; 3];
                let mut Iw_try: [f64; 3] = [0.0; 3];
                for k in 0..3 {
                    w_try[k] = *w_mid.add(k) + step * delta[k];
                    Iw_try[k] = *inertia.add(k) * w_try[k];
                }
                let mut coriolis_try: [f64; 3] = [0.0; 3];
                mji_cross(coriolis_try.as_mut_ptr(), w_try.as_ptr(), Iw_try.as_ptr());

                let mut f_try: [f64; 3] = [0.0; 3];
                for k in 0..3 {
                    f_try[k] = i2h_I[k] * (w_try[k] - *w.add(k)) + coriolis_try[k] - *tau.add(k);
                }

                if mju_norm3(f_try.as_ptr()) < fnorm {
                    mji_copy3(w_mid, w_try.as_ptr());
                    break;
                }
                step *= 0.5;
            }

            niter += 1;
        }

        niter
    }
}

/// C: midpoint (engine/engine_forward.c:1736)
/// Calls: mj_midpoint, mju_add, mju_copy
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn midpoint(m: *const mjModel, d: *const mjData, qfrc: *const f64, free_jntid: *const i32, nfree: i32, qvel_old: *mut f64, qvel_new: *mut f64, dofadr: *mut i32) {
    const MJ_DSBL_GRAVITY: i32 = 1 << 7;
    // SAFETY: caller guarantees all pointers are valid and arrays are properly sized
    unsafe {
        for i in 0..nfree as usize {
            let j = *free_jntid.add(i);
            let body = *(*m).jnt_bodyid.add(j as usize);

            // save DOF address
            let adr = *(*m).jnt_dofadr.add(j as usize);
            *dofadr.add(i) = adr;

            // save old (current) velocity
            crate::engine::engine_util_blas::mju_copy(
                qvel_old.add(6 * i), (*d).qvel.add(adr as usize), 6);

            // compute external force = qfrc + qfrc_bias
            let mut qfrc_total: [f64; 6] = [0.0; 6];
            crate::engine::engine_util_blas::mju_add(
                qfrc_total.as_mut_ptr(), qfrc.add(adr as usize),
                (*d).qfrc_bias.add(adr as usize), 6);

            // gravity
            let gravity: *const f64 = if ((*m).opt.disableflags & MJ_DSBL_GRAVITY) != 0 {
                std::ptr::null()
            } else {
                (*m).opt.gravity.as_ptr()
            };

            // midpoint solver for free joint
            mj_midpoint(
                *(*m).body_mass.add(body as usize),
                (*m).body_inertia.add(3 * body as usize),
                (*m).body_ipos.add(3 * body as usize),
                (*m).body_iquat.add(4 * body as usize),
                (*d).xquat.add(4 * body as usize),
                (*d).qvel.add(adr as usize),
                qfrc_total.as_ptr(),
                gravity,
                (*m).opt.timestep,
                qvel_new.add(6 * i),
            );
        }
    }
}

/// C: mj_checkPos (engine/engine_forward.h:27)
/// Calls: mj_resetData, mj_warning, mju_isBad
#[allow(unused_variables, non_snake_case)]
pub fn mj_check_pos(m: *const mjModel, d: *mut mjData) {
    const MJ_DSBL_AUTORESET: i32 = 1 << 16;
    const MJ_WARN_BADQPOS: i32 = 3;

    // SAFETY: m, d are valid pointers (caller contract).
    unsafe {
        let nq = (*m).nq as i32;
        let qpos = (*d).qpos;
        for i in 0..nq {
            if crate::engine::engine_util_misc::mju_is_bad(*qpos.add(i as usize)) != 0 {
                crate::engine::engine_core_util::mj_warning(d, MJ_WARN_BADQPOS, i);
                if ((*m).opt.disableflags & MJ_DSBL_AUTORESET) == 0 {
                    crate::engine::engine_io::mj_reset_data(m, d);
                }
                let warn_ptr = (*d).warning.as_mut_ptr() as *mut i32;
                *warn_ptr.add(MJ_WARN_BADQPOS as usize * 2 + 1) += 1; // number
                *warn_ptr.add(MJ_WARN_BADQPOS as usize * 2) = i;      // lastinfo
                return;
            }
        }
    }
}

/// C: mj_checkVel (engine/engine_forward.h:28)
/// Calls: mj_resetData, mj_warning, mju_isBad
#[allow(unused_variables, non_snake_case)]
pub fn mj_check_vel(m: *const mjModel, d: *mut mjData) {
    const MJ_ENBL_SLEEP: i32 = 1 << 4;
    const MJ_DSBL_AUTORESET: i32 = 1 << 16;
    const MJ_WARN_BADQVEL: i32 = 4;

    // SAFETY: m, d are valid pointers (caller contract).
    unsafe {
        let sleep_filter = ((*m).opt.enableflags & MJ_ENBL_SLEEP) != 0
            && (*d).nv_awake < (*m).nv as i32;
        let nv = if sleep_filter { (*d).nv_awake } else { (*m).nv as i32 };

        for j in 0..nv {
            let i = if sleep_filter { *(*d).dof_awake_ind.add(j as usize) } else { j };

            if crate::engine::engine_util_misc::mju_is_bad(*(*d).qvel.add(i as usize)) != 0 {
                crate::engine::engine_core_util::mj_warning(d, MJ_WARN_BADQVEL, i);
                if ((*m).opt.disableflags & MJ_DSBL_AUTORESET) == 0 {
                    crate::engine::engine_io::mj_reset_data(m, d);
                }
                let warn_ptr = (*d).warning.as_mut_ptr() as *mut i32;
                *warn_ptr.add(MJ_WARN_BADQVEL as usize * 2 + 1) += 1;
                *warn_ptr.add(MJ_WARN_BADQVEL as usize * 2) = i;
                return;
            }
        }
    }
}

/// C: mj_checkAcc (engine/engine_forward.h:29)
/// Calls: mj_forward, mj_resetData, mj_warning, mju_isBad
#[allow(unused_variables, non_snake_case)]
pub fn mj_check_acc(m: *const mjModel, d: *mut mjData) {
    const MJ_ENBL_SLEEP: i32 = 1 << 4;
    const MJ_DSBL_AUTORESET: i32 = 1 << 16;
    const MJ_WARN_BADQACC: i32 = 5;

    // SAFETY: m, d are valid pointers (caller contract).
    unsafe {
        let sleep_filter = ((*m).opt.enableflags & MJ_ENBL_SLEEP) != 0
            && (*d).nv_awake < (*m).nv as i32;
        let nv = if sleep_filter { (*d).nv_awake } else { (*m).nv as i32 };

        for j in 0..nv {
            let i = if sleep_filter { *(*d).dof_awake_ind.add(j as usize) } else { j };

            if crate::engine::engine_util_misc::mju_is_bad(*(*d).qacc.add(i as usize)) != 0 {
                crate::engine::engine_core_util::mj_warning(d, MJ_WARN_BADQACC, i);
                if ((*m).opt.disableflags & MJ_DSBL_AUTORESET) == 0 {
                    crate::engine::engine_io::mj_reset_data(m, d);
                }
                let warn_ptr = (*d).warning.as_mut_ptr() as *mut i32;
                *warn_ptr.add(MJ_WARN_BADQACC as usize * 2 + 1) += 1;
                *warn_ptr.add(MJ_WARN_BADQACC as usize * 2) = i;
                if ((*m).opt.disableflags & MJ_DSBL_AUTORESET) == 0 {
                    mj_forward(m, d);
                }
                return;
            }
        }
    }
}

/// C: mj_step (engine/engine_forward.h:35)
/// Calls: mj_Euler, mj_RungeKutta, mj_checkAcc, mj_checkPos, mj_checkVel, mj_compareFwdInv, mj_forward, mj_implicit, mju_message
#[allow(unused_variables, non_snake_case)]
pub fn mj_step(m: *const mjModel, d: *mut mjData) {
    const MJ_ENBL_FWDINV: i32 = 1 << 2;
    const MJ_INT_EULER: i32 = 0;
    const MJ_INT_RK4: i32 = 1;
    const MJ_INT_IMPLICIT: i32 = 2;
    const MJ_INT_IMPLICITFAST: i32 = 3;

    // SAFETY: m, d are valid pointers (caller contract).
    unsafe {
        // common to all integrators
        mj_check_pos(m, d);
        mj_check_vel(m, d);
        mj_forward(m, d);
        mj_check_acc(m, d);

        // compare forward and inverse solutions if enabled
        if ((*m).opt.enableflags & MJ_ENBL_FWDINV) != 0 {
            crate::engine::engine_inverse::mj_compare_fwd_inv(m, d);
        }

        // use selected integrator
        let integrator = (*m).opt.integrator;
        if integrator == MJ_INT_EULER {
            mj_euler(m, d);
        } else if integrator == MJ_INT_RK4 {
            mj_runge_kutta(m, d, 4);
        } else if integrator == MJ_INT_IMPLICIT || integrator == MJ_INT_IMPLICITFAST {
            mj_implicit(m, d);
        } else {
            crate::engine::engine_util_errmem::mju_error(
                b"invalid integrator\0".as_ptr() as *const i8);
        }
    }
}

/// C: mj_step1 (engine/engine_forward.h:38)
/// Calls: mj_checkPos, mj_checkVel, mj_energyPos, mj_energyVel, mj_fwdPosition, mj_fwdVelocity, mj_sensorPos, mj_sensorVel
#[allow(unused_variables, non_snake_case)]
pub fn mj_step1(m: *const mjModel, d: *mut mjData) {
    const MJ_ENBL_ENERGY: i32 = 1 << 1;

    // SAFETY: m, d are valid pointers (caller contract).
    unsafe {
        mj_check_pos(m, d);
        mj_check_vel(m, d);
        mj_fwd_position(m, d);
        crate::engine::engine_sensor::mj_sensor_pos(m, d);

        if !(*d).flg_energypos {
            if ((*m).opt.enableflags & MJ_ENBL_ENERGY) != 0 {
                crate::engine::engine_sensor::mj_energy_pos(m, d);
            } else {
                (*d).energy[0] = 0.0;
                (*d).energy[1] = 0.0;
            }
        }

        mj_fwd_velocity(m, d);
        crate::engine::engine_sensor::mj_sensor_vel(m, d);
        if ((*m).opt.enableflags & MJ_ENBL_ENERGY) != 0 && !(*d).flg_energyvel {
            crate::engine::engine_sensor::mj_energy_vel(m, d);
        }

        if let Some(cb) = crate::engine::engine_callback::mjcb_control {
            cb(m, d);
        }
    }
}

/// C: mj_step2 (engine/engine_forward.h:41)
/// Calls: mj_Euler, mj_checkAcc, mj_compareFwdInv, mj_fwdAcceleration, mj_fwdActuation, mj_fwdConstraint, mj_implicit, mj_sensorAcc
#[allow(unused_variables, non_snake_case)]
pub fn mj_step2(m: *const mjModel, d: *mut mjData) {
    const MJ_ENBL_FWDINV: i32 = 1 << 2;
    const MJ_INT_IMPLICIT: i32 = 2;
    const MJ_INT_IMPLICITFAST: i32 = 3;

    // SAFETY: m, d are valid pointers (caller contract).
    unsafe {
        mj_fwd_actuation(m, d);
        mj_fwd_acceleration(m, d);
        mj_fwd_constraint(m, d);
        (*d).flg_rnepost = false; // clear flag for lazy evaluation
        crate::engine::engine_sensor::mj_sensor_acc(m, d);
        mj_check_acc(m, d);

        // compare forward and inverse solutions if enabled
        if ((*m).opt.enableflags & MJ_ENBL_FWDINV) != 0 {
            crate::engine::engine_inverse::mj_compare_fwd_inv(m, d);
        }

        // integrate with Euler or implicit; RK4 defaults to Euler
        if (*m).opt.integrator == MJ_INT_IMPLICIT || (*m).opt.integrator == MJ_INT_IMPLICITFAST {
            mj_implicit(m, d);
        } else {
            mj_euler(m, d);
        }
    }
}

/// C: mj_forward (engine/engine_forward.h:44)
/// Calls: mj_forwardSkip
#[allow(unused_variables, non_snake_case)]
pub fn mj_forward(m: *const mjModel, d: *mut mjData) {
    mj_forward_skip(m, d, 0, 0); // mjSTAGE_NONE = 0
}

/// C: mj_forwardSkip (engine/engine_forward.h:47)
/// Calls: mj_energyPos, mj_energyVel, mj_fwdAcceleration, mj_fwdActuation, mj_fwdConstraint, mj_fwdPosition, mj_fwdVelocity, mj_sensorAcc, mj_sensorPos, mj_sensorVel
#[allow(unused_variables, non_snake_case)]
pub fn mj_forward_skip(m: *const mjModel, d: *mut mjData, skipstage: i32, skipsensor: i32) {
    const MJ_STAGE_POS: i32 = 1;
    const MJ_STAGE_VEL: i32 = 2;
    const MJ_ENBL_ENERGY: i32 = 1 << 1;
    const MJ_DSBL_ACTUATION: i32 = 1 << 11;

    // SAFETY: m, d are valid pointers (caller contract).
    unsafe {
        // position-dependent
        if skipstage < MJ_STAGE_POS {
            mj_fwd_position(m, d);

            if skipsensor == 0 {
                crate::engine::engine_sensor::mj_sensor_pos(m, d);
            }

            if !(*d).flg_energypos {
                if ((*m).opt.enableflags & MJ_ENBL_ENERGY) != 0 {
                    crate::engine::engine_sensor::mj_energy_pos(m, d);
                } else {
                    (*d).energy[0] = 0.0;
                    (*d).energy[1] = 0.0;
                }
            }
        }

        // velocity-dependent
        if skipstage < MJ_STAGE_VEL {
            mj_fwd_velocity(m, d);

            if skipsensor == 0 {
                crate::engine::engine_sensor::mj_sensor_vel(m, d);
            }

            if ((*m).opt.enableflags & MJ_ENBL_ENERGY) != 0 && !(*d).flg_energyvel {
                crate::engine::engine_sensor::mj_energy_vel(m, d);
            }
        }

        // acceleration-dependent
        if crate::engine::engine_callback::mjcb_control.is_some()
            && ((*m).opt.disableflags & MJ_DSBL_ACTUATION) == 0
        {
            if let Some(cb) = crate::engine::engine_callback::mjcb_control {
                cb(m, d);
            }
        }

        mj_fwd_actuation(m, d);
        mj_fwd_acceleration(m, d);
        mj_fwd_constraint(m, d);
        if skipsensor == 0 {
            (*d).flg_rnepost = false;
            crate::engine::engine_sensor::mj_sensor_acc(m, d);
        }
    }
}

/// C: mj_RungeKutta (engine/engine_forward.h:53)
/// Calls: mj_advance, mj_forwardSkip, mj_freeStack, mj_integratePos, mj_markStack, mj_stackAllocInfo, mju_addToScl, mju_copy, mju_message, mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mj_runge_kutta(m: *const mjModel, d: *mut mjData, N: i32) {
    todo!() // mj_RungeKutta
}

/// C: mj_Euler (engine/engine_forward.h:56)
/// Calls: mj_EulerSkip
#[allow(unused_variables, non_snake_case)]
pub fn mj_euler(m: *const mjModel, d: *mut mjData) {
    mj_euler_skip(m, d, 0);
}

/// C: mj_EulerSkip (engine/engine_forward.h:59)
/// Calls: mj_actuatorDamping, mj_advance, mj_factorI, mj_freeStack, mj_markStack, mj_solveLD, mj_stackAllocInfo, mjd_xPolyForce, mju_add, mju_addInd, mju_copy, mju_copyInd, mju_copySparse, mju_isZero
#[allow(unused_variables, non_snake_case)]
pub fn mj_euler_skip(m: *const mjModel, d: *mut mjData, skipfactor: i32) {
    const MJ_ENBL_SLEEP: i32 = 1 << 4;
    const MJ_DSBL_EULERDAMP: i32 = 1 << 15;
    const MJ_DSBL_DAMPER: i32 = 1 << 6;
    const MJ_OBJ_JOINT: u32 = 3;
    const MJ_NPOLY: i32 = 2;

    // SAFETY: m, d are valid pointers (caller contract).
    unsafe {
        crate::engine::engine_memory::mj_mark_stack(d);
        let qfrc = crate::engine::engine_memory::mj_stack_alloc_num(d, (*m).nv as usize);
        let qacc = crate::engine::engine_memory::mj_stack_alloc_num(d, (*m).nv as usize);

        // sleep filtering
        let sleep_filter = ((*m).opt.enableflags & MJ_ENBL_SLEEP) != 0
            && (*d).nv_awake < (*m).nv as i32;
        let nv = if sleep_filter { (*d).nv_awake } else { (*m).nv as i32 };
        let dof_awake_ind: *const i32 = if sleep_filter { (*d).dof_awake_ind } else { std::ptr::null() };

        // check for dof damping if disable flag is not set
        let mut dof_damping: i32 = 0;
        if ((*m).opt.disableflags & MJ_DSBL_EULERDAMP) == 0
            && ((*m).opt.disableflags & MJ_DSBL_DAMPER) == 0
        {
            for v in 0..nv {
                let i = if sleep_filter { *dof_awake_ind.add(v as usize) } else { v };
                if *(*m).dof_damping.add(i as usize) > 0.0
                    || crate::engine::engine_util_misc::mju_is_zero(
                        (*m).dof_dampingpoly.add((MJ_NPOLY * i) as usize), MJ_NPOLY) == 0
                    || *(*m).jnt_actuatorid.add(*(*m).dof_jntid.add(i as usize) as usize) != -1
                {
                    dof_damping = 1;
                    break;
                }
            }
        }

        // no damping or disabled: explicit velocity integration
        if dof_damping == 0 {
            if sleep_filter {
                crate::engine::engine_util_blas::mju_copy_ind(qacc, (*d).qacc, dof_awake_ind, nv);
            } else {
                crate::engine::engine_util_blas::mju_copy(qacc, (*d).qacc, nv);
            }
        }
        // damping: integrate implicitly
        else {
            if skipfactor == 0 {
                // qH = M
                if sleep_filter {
                    crate::engine::engine_util_sparse::mju_copy_sparse(
                        (*d).qH, (*d).M, (*m).M_rownnz, (*m).M_rowadr, dof_awake_ind, (*d).nv_awake);
                } else {
                    crate::engine::engine_util_blas::mju_copy((*d).qH, (*d).M, (*m).nC as i32);
                }

                // qH += h*diag(B)
                for v in 0..nv {
                    let i = if sleep_filter { *dof_awake_ind.add(v as usize) } else { v };
                    let qv = *(*d).qvel.add(i as usize);
                    let mut poly: [f64; 2] = [0.0; 2];
                    crate::engine::engine_util_blas::mju_copy(
                        poly.as_mut_ptr(), (*m).dof_dampingpoly.add((MJ_NPOLY * i) as usize), MJ_NPOLY);
                    let damping = *(*m).dof_damping.add(i as usize)
                        + crate::engine::engine_core_util::mj_actuator_damping(
                            m, MJ_OBJ_JOINT, *(*m).dof_jntid.add(i as usize), poly.as_mut_ptr());
                    let damp_deriv = crate::engine::engine_util_misc::mjd_x_poly_force(
                        damping, poly.as_ptr(), qv, MJ_NPOLY, 1);
                    let diag_idx = *(*m).M_rowadr.add(i as usize) + *(*m).M_rownnz.add(i as usize) - 1;
                    *(*d).qH.add(diag_idx as usize) += (*m).opt.timestep * damp_deriv;
                }

                // factorize in-place
                crate::engine::engine_core_smooth::mj_factor_i(
                    (*d).qH, (*d).qHDiagInv, nv,
                    (*m).M_rownnz, (*m).M_rowadr, (*m).M_colind, dof_awake_ind);
            }

            // solve
            if sleep_filter {
                crate::engine::engine_util_blas::mju_add_ind(
                    qfrc, (*d).qfrc_smooth, (*d).qfrc_constraint, dof_awake_ind, nv);
                crate::engine::engine_util_blas::mju_copy_ind(qacc, qfrc, dof_awake_ind, nv);
            } else {
                crate::engine::engine_util_blas::mju_add(
                    qfrc, (*d).qfrc_smooth, (*d).qfrc_constraint, nv);
                crate::engine::engine_util_blas::mju_copy(qacc, qfrc, nv);
            }
            crate::engine::engine_core_smooth::mj_solve_ld(
                qacc, (*d).qH, (*d).qHDiagInv, nv, 1,
                (*m).M_rownnz, (*m).M_rowadr, (*m).M_colind, dof_awake_ind);
        }

        // advance state and time
        mj_advance(m, d, (*d).act_dot, qacc, std::ptr::null());

        crate::engine::engine_memory::mj_free_stack(d);
    }
}

/// C: mj_implicit (engine/engine_forward.h:62)
/// Calls: mj_implicitSkip
#[allow(unused_variables, non_snake_case)]
pub fn mj_implicit(m: *const mjModel, d: *mut mjData) {
    mj_implicit_skip(m, d, 0);
}

/// C: mj_implicitSkip (engine/engine_forward.h:65)
/// Calls: flexInterp_cgsolve, flex_has_implicit_stiffness, midpoint, midpoint_aligned, midpoint_eligible, mj_advance, mj_factorI, mj_freeStack, mj_markStack, mj_solveLD, mj_stackAllocInfo, mjd_smooth_vel, mju_add, mju_addInd, mju_addScl, mju_addToScl, mju_copy, mju_copyInd, mju_factorLUSparse, mju_gather, mju_gatherMasked, mju_message, mju_solveLUSparse
#[allow(unused_variables, non_snake_case)]
pub fn mj_implicit_skip(m: *const mjModel, d: *mut mjData, skipfactor: i32) {
    todo!() // mj_implicitSkip
}

/// C: mj_midpoint (engine/engine_forward.h:69)
/// Calls: midpointNewton, mji_add3, mji_addTo3, mji_axisAngle2Quat, mji_copy3, mji_cross, mji_mulQuat, mji_negQuat, mji_rotVecQuat, mji_sub3, mju_dot3, mju_normalize3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_midpoint(mass: f64, inertia: *const f64, ipos: *const f64, iquat: *const f64, xquat: *const f64, qvel: *const f64, qfrc: *const f64, gravity: *const f64, h: f64, qvel_new: *mut f64) -> i32 {
    // SAFETY: all pointer args point to valid f64 arrays of documented sizes
    // inertia[3], ipos[3], iquat[4], xquat[4], qvel[6], qfrc[6], gravity[3], qvel_new[6]
    unsafe {
        use crate::engine::engine_inline::{
            mji_neg_quat, mji_rot_vec_quat, mji_copy3, mji_mul_quat,
            mji_cross, mji_sub3, mji_add3, mji_add_to3, mji_axis_angle2quat,
        };
        use crate::engine::engine_util_blas::{mju_dot3, mju_normalize3};

        // transform angular velocity and torque to inertial frame
        let mut iquat_neg: [f64; 4] = [0.0; 4];
        let mut w: [f64; 3] = [0.0; 3];
        let mut tau: [f64; 3] = [0.0; 3];
        mji_neg_quat(iquat_neg.as_mut_ptr(), iquat);
        mji_rot_vec_quat(w.as_mut_ptr(), qvel.add(3), iquat_neg.as_ptr());
        mji_rot_vec_quat(tau.as_mut_ptr(), qfrc.add(3), iquat_neg.as_ptr());

        // check for translational-rotational coupling
        let aligned = *ipos.add(0) == 0.0 && *ipos.add(1) == 0.0 && *ipos.add(2) == 0.0;

        let mut r_com: [f64; 3] = [0.0; 3];
        let mut tau_com: [f64; 3] = [0.0; 3];
        let mut rot_x2i: [f64; 4] = [0.0; 4];
        let mut force: [f64; 3] = [0.0; 3];

        // compute torque at CoM in inertial frame
        if aligned {
            mji_copy3(tau_com.as_mut_ptr(), tau.as_ptr());
        } else {
            // rotation from world to inertial frame
            let mut xquat_neg: [f64; 4] = [0.0; 4];
            mji_neg_quat(xquat_neg.as_mut_ptr(), xquat);
            mji_mul_quat(rot_x2i.as_mut_ptr(), iquat_neg.as_ptr(), xquat_neg.as_ptr());

            // force and CoM offset in inertial frame
            mji_rot_vec_quat(force.as_mut_ptr(), qfrc, rot_x2i.as_ptr());
            mji_rot_vec_quat(r_com.as_mut_ptr(), ipos, iquat_neg.as_ptr());

            // torque at CoM in inertial frame
            let mut rxf: [f64; 3] = [0.0; 3];
            mji_cross(rxf.as_mut_ptr(), r_com.as_ptr(), force.as_ptr());
            mji_sub3(tau_com.as_mut_ptr(), tau.as_ptr(), rxf.as_ptr());
        }

        // solve for midpoint angular velocity
        let mut w_mid: [f64; 3] = [0.0; 3];
        let niter = midpoint_newton(inertia, w.as_ptr(), tau_com.as_ptr(), h, w_mid.as_mut_ptr());

        // next and mid angular velocities in inertial frame, rotate both to body frame
        let mut w_new: [f64; 3] = [0.0; 3];
        let mut w_new_body: [f64; 3] = [0.0; 3];
        let mut w_mid_body: [f64; 3] = [0.0; 3];
        for k in 0..3 {
            w_new[k] = 2.0 * w_mid[k] - w[k];
        }
        mji_rot_vec_quat(w_new_body.as_mut_ptr(), w_new.as_ptr(), iquat);
        mji_rot_vec_quat(w_mid_body.as_mut_ptr(), w_mid.as_ptr(), iquat);
        mji_copy3(qvel_new.add(3), w_new_body.as_ptr());

        // aligned: return
        if aligned {
            return niter;
        }

        // non-aligned: solve for translational velocity

        // rotate linear velocity to inertial frame
        let mut v: [f64; 3] = [0.0; 3];
        mji_rot_vec_quat(v.as_mut_ptr(), qvel, rot_x2i.as_ptr());

        // current CoM velocities (rot, lin) in inertial frame
        let mut wxr: [f64; 3] = [0.0; 3];
        mji_cross(wxr.as_mut_ptr(), w.as_ptr(), r_com.as_ptr());
        let mut vcom: [f64; 3] = [0.0; 3];
        mji_add3(vcom.as_mut_ptr(), v.as_ptr(), wxr.as_ptr());

        // right-hand side for midpoint CoM velocity
        let i2h = 2.0 / h;
        let mut b: [f64; 3] = [0.0; 3];
        for k in 0..3 {
            b[k] = force[k] / mass + i2h * vcom[k];
        }

        // add gravity, if any
        if !gravity.is_null() {
            let mut g_inertial: [f64; 3] = [0.0; 3];
            mji_rot_vec_quat(g_inertial.as_mut_ptr(), gravity, rot_x2i.as_ptr());
            mji_add_to3(b.as_mut_ptr(), g_inertial.as_ptr());
        }

        // analytic solution for (i2h*Id + [w_mid]x) * vcom_mid = b
        let wnorm2 = mju_dot3(w_mid.as_ptr(), w_mid.as_ptr());
        let denom = i2h * i2h + wnorm2;
        let w_dot_b = mju_dot3(w_mid.as_ptr(), b.as_ptr());
        let mut w_cross_b: [f64; 3] = [0.0; 3];
        mji_cross(w_cross_b.as_mut_ptr(), w_mid.as_ptr(), b.as_ptr());
        let mut vcom_mid: [f64; 3] = [0.0; 3];
        for k in 0..3 {
            vcom_mid[k] = (i2h * b[k] + (w_dot_b / i2h) * w_mid[k] - w_cross_b[k]) / denom;
        }

        // recover midpoint and new joint velocity in inertial frame
        let mut wxr_mid: [f64; 3] = [0.0; 3];
        mji_cross(wxr_mid.as_mut_ptr(), w_mid.as_ptr(), r_com.as_ptr());
        let mut v_mid: [f64; 3] = [0.0; 3];
        let mut v_new: [f64; 3] = [0.0; 3];
        for k in 0..3 {
            v_mid[k] = vcom_mid[k] - wxr_mid[k];
            v_new[k] = 2.0 * v_mid[k] - v[k];
        }

        // estimate new orientation
        let mut axis: [f64; 3] = [0.0; 3];
        mji_copy3(axis.as_mut_ptr(), w_mid_body.as_ptr());
        let wnorm = mju_normalize3(axis.as_mut_ptr());
        let mut qrot_new: [f64; 4] = [0.0; 4];
        mji_axis_angle2quat(qrot_new.as_mut_ptr(), axis.as_ptr(), h * wnorm);
        let mut xquat_new: [f64; 4] = [0.0; 4];
        mji_mul_quat(xquat_new.as_mut_ptr(), xquat, qrot_new.as_ptr());

        // v_new (linear): inertial → body → world using new orientation
        let mut v_body: [f64; 3] = [0.0; 3];
        mji_rot_vec_quat(v_body.as_mut_ptr(), v_new.as_ptr(), iquat);
        mji_rot_vec_quat(qvel_new, v_body.as_ptr(), xquat_new.as_ptr());

        niter
    }
}

/// C: mj_fwdKinematics (engine/engine_forward.h:78)
/// Calls: mj_camlight, mj_comPos, mj_flex, mj_kinematics, mj_tendon, mj_updateSleep, mj_wakeTendon
#[allow(unused_variables, non_snake_case)]
pub fn mj_fwd_kinematics(m: *const mjModel, d: *mut mjData) {
    // SAFETY: m, d are valid pointers (caller contract).
    unsafe {
        crate::engine::engine_core_smooth::mj_kinematics(m, d);
        crate::engine::engine_core_smooth::mj_com_pos(m, d);
        crate::engine::engine_core_smooth::mj_camlight(m, d);
        crate::engine::engine_core_smooth::mj_flex(m, d);
        crate::engine::engine_core_smooth::mj_tendon(m, d);
        if crate::engine::engine_sleep::mj_wake_tendon(m, d) != 0 {
            crate::engine::engine_sleep::mj_update_sleep(m, d);
        }
    }
}

/// C: mj_fwdPosition (engine/engine_forward.h:81)
/// Calls: mj_collision, mj_factorM, mj_fwdKinematics, mj_island, mj_makeConstraint, mj_makeM, mj_projectConstraint, mj_transmission, mj_updateSleep, mj_wakeCollision, mj_wakeEquality
#[allow(unused_variables, non_snake_case)]
pub fn mj_fwd_position(m: *const mjModel, d: *mut mjData) {
    // SAFETY: m, d are valid pointers (caller contract).
    unsafe {
        // clear position-dependent flags for lazy evaluation
        (*d).flg_energypos = false;

        mj_fwd_kinematics(m, d);

        // inertia
        crate::engine::engine_core_smooth::mj_make_m(m, d);
        crate::engine::engine_core_smooth::mj_factor_m(m, d);

        // collision
        crate::engine::engine_collision_driver::mj_collision(m, d);

        if crate::engine::engine_sleep::mj_wake_collision(m, d) != 0 {
            crate::engine::engine_sleep::mj_update_sleep(m, d);
            crate::engine::engine_collision_driver::mj_collision(m, d);
        }

        if crate::engine::engine_sleep::mj_wake_equality(m, d) != 0 {
            crate::engine::engine_sleep::mj_update_sleep(m, d);
        }

        crate::engine::engine_core_constraint::mj_make_constraint(m, d);
        crate::engine::engine_island::mj_island(m, d);

        crate::engine::engine_core_constraint::mj_project_constraint(m, d);

        crate::engine::engine_core_smooth::mj_transmission(m, d);
    }
}

/// C: mj_fwdVelocity (engine/engine_forward.h:84)
/// Calls: mj_comVel, mj_passive, mj_referenceConstraint, mj_rne, mj_tendonBias, mju_mulMatVecSparse, mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mj_fwd_velocity(m: *const mjModel, d: *mut mjData) {
    const MJ_DSBL_ACTUATION: i32 = 1 << 11;

    // SAFETY: m, d are valid pointers (caller contract).
    unsafe {
        // clear velocity-dependent flags for lazy evaluation
        (*d).flg_subtreevel = false;
        (*d).flg_energyvel = false;

        // flexedge velocity: skip interp and rigid flexes
        crate::engine::engine_util_blas::mju_zero((*d).flexedge_velocity, (*m).nflexedge as i32);
        for f in 0..(*m).nflex as i32 {
            if *(*m).flex_rigid.add(f as usize) || *(*m).flex_interp.add(f as usize) != 0 {
                continue;
            }
            let adr = *(*m).flex_edgeadr.add(f as usize);
            let num = *(*m).flex_edgenum.add(f as usize);
            crate::engine::engine_util_sparse::mju_mul_mat_vec_sparse(
                (*d).flexedge_velocity.add(adr as usize),
                (*d).flexedge_J,
                (*d).qvel,
                num,
                (*m).flexedge_J_rownnz.add(adr as usize),
                (*m).flexedge_J_rowadr.add(adr as usize),
                (*m).flexedge_J_colind,
                std::ptr::null(),
            );
        }

        // tendon velocity: always sparse
        crate::engine::engine_util_sparse::mju_mul_mat_vec_sparse(
            (*d).ten_velocity, (*d).ten_J, (*d).qvel, (*m).ntendon as i32,
            (*m).ten_J_rownnz, (*m).ten_J_rowadr, (*m).ten_J_colind, std::ptr::null());

        // actuator velocity: always sparse
        if ((*m).opt.disableflags & MJ_DSBL_ACTUATION) == 0 {
            crate::engine::engine_util_sparse::mju_mul_mat_vec_sparse(
                (*d).actuator_velocity, (*d).actuator_moment, (*d).qvel, (*m).nu as i32,
                (*d).moment_rownnz, (*d).moment_rowadr, (*d).moment_colind, std::ptr::null());
        } else {
            crate::engine::engine_util_blas::mju_zero((*d).actuator_velocity, (*m).nu as i32);
        }

        // com-based velocities, passive forces, constraint references
        crate::engine::engine_core_smooth::mj_com_vel(m, d);
        crate::engine::engine_passive::mj_passive(m, d);
        crate::engine::engine_core_constraint::mj_reference_constraint(m, d);

        // compute qfrc_bias with abbreviated RNE (without acceleration)
        crate::engine::engine_core_smooth::mj_rne(m, d, 0, (*d).qfrc_bias);

        // add bias force due to tendon armature
        crate::engine::engine_core_smooth::mj_tendon_bias(m, d, (*d).qfrc_bias);
    }
}

/// C: mj_fwdActuation (engine/engine_forward.h:87)
/// Calls: clampVec, dcmotorVoltage, mj_actuatorDisabled, mj_dcmotorSlots, mj_freeStack, mj_lugreStribeck, mj_markStack, mj_nextActivation, mj_readCtrl, mj_sleepState, mj_stackAllocInfo, mj_warning, mjp_getPluginAtSlotUnsafe, mjp_pluginCount, mju_addTo, mju_clip, mju_isBad, mju_max, mju_message, mju_min, mju_mulMatTVecSparse, mju_muscleBias, mju_muscleDynamics, mju_muscleGain, mju_norm3, mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn mj_fwd_actuation(m: *const mjModel, d: *mut mjData) {
    const MJNDYN: i32 = 10;
    const MJNGAIN: i32 = 10;
    const MJNBIAS: i32 = 10;
    const MJMINVAL: f64 = 1e-15;
    const MJDSBL_ACTUATION: i32 = 1 << 11;
    const MJDSBL_CLAMPCTRL: i32 = 1 << 8;
    const MJDSBL_GRAVITY: i32 = 1 << 7;
    const MJENBL_SLEEP: i32 = 1 << 4;
    const MJOBJ_ACTUATOR: i32 = 19;
    const MJS_ASLEEP: i32 = 0;
    const MJDYN_INTEGRATOR: i32 = 0;
    const MJDYN_FILTER: i32 = 1;
    const MJDYN_FILTEREXACT: i32 = 2;
    const MJDYN_MUSCLE: i32 = 3;
    const MJDYN_DCMOTOR: i32 = 4;
    const MJGAIN_FIXED: i32 = 0;
    const MJGAIN_AFFINE: i32 = 1;
    const MJGAIN_MUSCLE: i32 = 2;
    const MJGAIN_DCMOTOR: i32 = 3;
    const MJBIAS_NONE: i32 = 0;
    const MJBIAS_AFFINE: i32 = 1;
    const MJBIAS_MUSCLE: i32 = 2;
    const MJBIAS_DCMOTOR: i32 = 3;
    const MJTRN_TENDON: i32 = 3;
    const MJWARN_BADCTRL: i32 = 6;
    const MJPLUGIN_ACTUATOR: i32 = 1;

    // SAFETY: m, d are valid pointers (caller contract).
    unsafe {
        let nv = (*m).nv as i32;
        let nu = (*m).nu as i32;
        let ntendon = (*m).ntendon as i32;
        let force = (*d).actuator_force;

        // clear actuator_force
        crate::engine::engine_util_blas::mju_zero(force, nu);

        let sleep_filter = ((*m).opt.enableflags & MJENBL_SLEEP) != 0;

        // disabled or no actuation: return
        if nu == 0 || ((*m).opt.disableflags & MJDSBL_ACTUATION) != 0 {
            crate::engine::engine_util_blas::mju_zero((*d).qfrc_actuator, nv);
            return;
        }

        // any tendon transmission targets with force limits
        let mut tendon_frclimited: i32 = 0;

        // local copy of ctrl
        crate::engine::engine_memory::mj_mark_stack(d);
        let ctrl = crate::engine::engine_memory::mj_stack_alloc_num(d, nu as usize);

        // read from ctrl or history buffer for delayed actuators
        for i in 0..nu {
            let interp = *(*m).actuator_history.add(2 * i as usize + 1);
            if *(*m).actuator_delay.add(i as usize) != 0.0 {
                *ctrl.add(i as usize) = crate::engine::engine_support::mj_read_ctrl(
                    m, d as *const mjData, i, (*d).time, interp,
                );
            } else {
                *ctrl.add(i as usize) = *(*d).ctrl.add(i as usize);
            }
        }

        // clamp local copy
        if ((*m).opt.disableflags & MJDSBL_CLAMPCTRL) == 0 {
            clamp_vec(
                ctrl, (*m).actuator_ctrlrange, (*m).actuator_ctrllimited,
                nu, std::ptr::null(),
            );
        }

        // check controls, set all to 0 if any are bad
        for i in 0..nu {
            if crate::engine::engine_util_misc::mju_is_bad(*ctrl.add(i as usize)) != 0 {
                crate::engine::engine_core_util::mj_warning(d, MJWARN_BADCTRL, i);
                crate::engine::engine_util_blas::mju_zero(ctrl, nu);
                break;
            }
        }

        // act_dot for stateful actuators
        for i in 0..nu {
            if sleep_filter
                && crate::engine::engine_sleep::mj_sleep_state(
                    m, d as *const mjData, MJOBJ_ACTUATOR as u32, i,
                ) == MJS_ASLEEP
            {
                continue;
            }

            let act_first = *(*m).actuator_actadr.add(i as usize);
            if act_first < 0 {
                continue;
            }

            let actnum = *(*m).actuator_actnum.add(i as usize);
            if actnum != 0 {
                crate::engine::engine_util_blas::mju_zero(
                    (*d).act_dot.add(act_first as usize), actnum,
                );
            }

            let dynprm = (*m).actuator_dynprm.add((i * MJNDYN) as usize);
            let dyntype = *(*m).actuator_dyntype.add(i as usize);
            let act_last = act_first + actnum - 1;

            if dyntype == MJDYN_INTEGRATOR {
                *(*d).act_dot.add(act_last as usize) = *ctrl.add(i as usize);
            } else if dyntype == MJDYN_FILTER || dyntype == MJDYN_FILTEREXACT {
                let tau = crate::engine::engine_util_misc::mju_max(MJMINVAL, *dynprm.add(0));
                *(*d).act_dot.add(act_last as usize) =
                    (*ctrl.add(i as usize) - *(*d).act.add(act_last as usize)) / tau;
            } else if dyntype == MJDYN_MUSCLE {
                *(*d).act_dot.add(act_last as usize) =
                    crate::engine::engine_util_misc::mju_muscle_dynamics(
                        *ctrl.add(i as usize), *(*d).act.add(act_last as usize), dynprm,
                    );
            } else if dyntype == MJDYN_DCMOTOR {
                let gainprm = (*m).actuator_gainprm.add((MJNGAIN * i) as usize);
                let slots = crate::engine::engine_util_misc::mj_dcmotor_slots(dynprm, gainprm);

                let mut adr = act_first;
                let velocity = *(*d).actuator_velocity.add(i as usize);
                let R = *gainprm.add(0);
                let K = *gainprm.add(1);
                let ki = *gainprm.add(5);
                let te = *dynprm.add(0);

                // slew rate limiting
                let slew_s = *dynprm.add(7);
                if slew_s > 0.0 {
                    let u_prev = *(*d).act.add(adr as usize);
                    let slew = slew_s * (*m).opt.timestep;
                    let u_eff = crate::engine::engine_util_misc::mju_clip(
                        *ctrl.add(i as usize), u_prev - slew, u_prev + slew,
                    );
                    *(*d).act_dot.add(adr as usize) = (u_eff - u_prev) / (*m).opt.timestep;
                    *ctrl.add(i as usize) = u_eff;
                    adr += 1;
                }

                // integral state
                let mut x_I: f64 = 0.0;
                if ki > 0.0 {
                    x_I = *(*d).act.add(adr as usize);
                    let input_mode = *gainprm.add(8) as i32;
                    let Imax = *dynprm.add(8);
                    let mut act_dot_val = *ctrl.add(i as usize);

                    // position mode
                    if input_mode == 1 {
                        act_dot_val = *ctrl.add(i as usize) - *(*d).actuator_length.add(i as usize);
                    }

                    // clamp act_dot based on integral state
                    if Imax > 0.0 {
                        if x_I >= Imax {
                            act_dot_val = crate::engine::engine_util_misc::mju_min(act_dot_val, 0.0);
                        } else if x_I <= -Imax {
                            act_dot_val = crate::engine::engine_util_misc::mju_max(act_dot_val, 0.0);
                        }
                    }
                    *(*d).act_dot.add(adr as usize) = act_dot_val;
                    adr += 1;
                }

                // compute physical voltage
                let V = dcmotor_voltage(
                    *ctrl.add(i as usize), *(*d).actuator_length.add(i as usize),
                    velocity, x_I, gainprm,
                );

                // temperature
                let RT = *dynprm.add(2);
                if RT > 0.0 {
                    let C = *dynprm.add(3);
                    let Ta = *dynprm.add(4);
                    let alpha = *gainprm.add(2);
                    let T0 = *gainprm.add(3);
                    let T = *(*d).act.add(adr as usize);
                    let R_adj = R * (1.0 + alpha * (T + Ta - T0));

                    let current = if te > 0.0 {
                        *(*d).act.add(act_last as usize)
                    } else {
                        (V - K * velocity) / R_adj
                    };
                    *(*d).act_dot.add(adr as usize) =
                        (R_adj * current * current - T / RT) / C;
                    adr += 1;
                }

                // LuGre bristle state
                let sigma0 = *dynprm.add(5);
                if sigma0 > 0.0 {
                    let biasprm = (*m).actuator_biasprm.add((MJNBIAS * i) as usize);
                    let F_C = *biasprm.add(3);
                    let F_S = *biasprm.add(4);
                    let v_S = *biasprm.add(5);
                    let z = *(*d).act.add(adr as usize);
                    let g = crate::engine::engine_util_misc::mj_lugre_stribeck(
                        velocity, F_C, F_S, v_S,
                    );
                    let a = -sigma0 * velocity.abs()
                        / crate::engine::engine_util_misc::mju_max(MJMINVAL, g);
                    *(*d).act_dot.add(adr as usize) = a * z + velocity;
                    adr += 1;
                }

                // current state
                if te > 0.0 {
                    let dimax = *dynprm.add(1);
                    let mut i_dot = (V / R - K / R * velocity
                        - *(*d).act.add(act_last as usize))
                        / te;
                    if dimax > 0.0 {
                        i_dot = crate::engine::engine_util_misc::mju_clip(i_dot, -dimax, dimax);
                    }
                    *(*d).act_dot.add(act_last as usize) = i_dot;
                }
            }
            // user dynamics: skip (requires callback mjcb_act_dyn which is an opaque fn pointer)
        }

        // get act_dot from actuator plugins
        if (*m).nplugin > 0 {
            let nslot = crate::engine::engine_plugin::mjp_plugin_count();
            for i in 0..(*m).nplugin as i32 {
                let slot = *(*m).plugin.add(i as usize);
                let plugin = crate::engine::engine_plugin::mjp_get_plugin_at_slot_unsafe(slot, nslot);
                if !plugin.is_null() && ((*plugin).capabilityflags & MJPLUGIN_ACTUATOR) != 0 {
                    if let Some(act_dot_fn) = (*plugin).actuator_act_dot {
                        let f: unsafe extern "C" fn(*const mjModel, *mut mjData, i32) =
                            std::mem::transmute(act_dot_fn);
                        f(m, d, i);
                    }
                }
            }
        }

        // force = gain .* [ctrl/act] + bias
        let mut gain: f64;
        let mut bias: f64;
        for i in 0..nu {
            // skip if sleeping
            if sleep_filter
                && crate::engine::engine_sleep::mj_sleep_state(
                    m, d as *const mjData, MJOBJ_ACTUATOR as u32, i,
                ) == MJS_ASLEEP
            {
                continue;
            }

            // skip if disabled
            if crate::engine::engine_support::mj_actuator_disabled(m, i) != 0 {
                continue;
            }

            // skip actuator plugins
            if *(*m).actuator_plugin.add(i as usize) >= 0 {
                continue;
            }

            // check for tendon transmission with force limits
            if ntendon != 0 && tendon_frclimited == 0
                && *(*m).actuator_trntype.add(i as usize) == MJTRN_TENDON
            {
                tendon_frclimited =
                    *(*m).tendon_actfrclimited.add(*(*m).actuator_trnid.add(2 * i as usize) as usize) as i32;
            }

            // extract info
            let dynprm = (*m).actuator_dynprm.add((MJNDYN * i) as usize);
            let gainprm = (*m).actuator_gainprm.add((MJNGAIN * i) as usize);
            let gaintype = *(*m).actuator_gaintype.add(i as usize);
            let actnum = *(*m).actuator_actnum.add(i as usize);

            // handle according to gain type
            if gaintype == MJGAIN_FIXED {
                gain = *gainprm.add(0);
            } else if gaintype == MJGAIN_AFFINE {
                gain = *gainprm.add(0)
                    + *gainprm.add(1) * *(*d).actuator_length.add(i as usize)
                    + *gainprm.add(2) * *(*d).actuator_velocity.add(i as usize);
            } else if gaintype == MJGAIN_MUSCLE {
                gain = crate::engine::engine_util_misc::mju_muscle_gain(
                    *(*d).actuator_length.add(i as usize),
                    *(*d).actuator_velocity.add(i as usize),
                    (*m).actuator_lengthrange.add(2 * i as usize),
                    *(*m).actuator_acc0.add(i as usize),
                    gainprm,
                );
            } else if gaintype == MJGAIN_DCMOTOR {
                let mut R_val = *gainprm.add(0);
                let K_val = *gainprm.add(1);
                let slots = crate::engine::engine_util_misc::mj_dcmotor_slots(dynprm, gainprm);

                let adr = *(*m).actuator_actadr.add(i as usize);

                // adjust R for temperature if enabled
                if slots.temperature >= 0 {
                    let T = *(*d).act.add((adr + slots.temperature) as usize);
                    let alpha = *gainprm.add(2);
                    let T0 = *gainprm.add(3);
                    let Ta = *dynprm.add(4);
                    R_val *= 1.0 + alpha * (T + Ta - T0);
                }

                gain = if *dynprm.add(0) > 0.0 {
                    K_val
                } else {
                    K_val / crate::engine::engine_util_misc::mju_max(MJMINVAL, R_val)
                };

                // controller: compute voltage, override ctrl[i]
                if (*gainprm.add(8) as i32) > 0 {
                    let x_I = if slots.integral >= 0 {
                        *(*d).act.add((adr + slots.integral) as usize)
                    } else {
                        0.0
                    };
                    *ctrl.add(i as usize) = dcmotor_voltage(
                        *ctrl.add(i as usize), *(*d).actuator_length.add(i as usize),
                        *(*d).actuator_velocity.add(i as usize), x_I, gainprm,
                    );
                }
            } else {
                // user gain: default to 1
                gain = 1.0;
            }

            // set force = gain .* [ctrl/act]
            let dcmotor_no_current =
                gaintype == MJGAIN_DCMOTOR && *dynprm.add(0) <= 0.0;
            if actnum == 0 || dcmotor_no_current {
                *force.add(i as usize) = gain * *ctrl.add(i as usize);
            } else {
                let act_adr = *(*m).actuator_actadr.add(i as usize) + actnum - 1;
                let act_val: f64;
                if *(*m).actuator_actearly.add(i as usize) {
                    act_val = crate::engine::engine_support::mj_next_activation(
                        m, d as *const mjData, i, act_adr, *(*d).act_dot.add(act_adr as usize),
                    );
                } else {
                    act_val = *(*d).act.add(act_adr as usize);
                }
                *force.add(i as usize) = gain * act_val;
            }

            // extract bias info
            let biasprm = (*m).actuator_biasprm.add((MJNBIAS * i) as usize);
            let biastype = *(*m).actuator_biastype.add(i as usize);

            if biastype == MJBIAS_NONE {
                bias = 0.0;
            } else if biastype == MJBIAS_AFFINE {
                bias = *biasprm.add(0)
                    + *biasprm.add(1) * *(*d).actuator_length.add(i as usize)
                    + *biasprm.add(2) * *(*d).actuator_velocity.add(i as usize);
            } else if biastype == MJBIAS_MUSCLE {
                bias = crate::engine::engine_util_misc::mju_muscle_bias(
                    *(*d).actuator_length.add(i as usize),
                    (*m).actuator_lengthrange.add(2 * i as usize),
                    *(*m).actuator_acc0.add(i as usize),
                    biasprm,
                );
            } else if biastype == MJBIAS_DCMOTOR {
                bias = 0.0;
                let te = *(*m).actuator_dynprm.add((MJNDYN * i) as usize);
                if te <= 0.0 {
                    let K_val = *gainprm.add(1);
                    bias -= gain * K_val * *(*d).actuator_velocity.add(i as usize);
                }
            } else {
                bias = 0.0;
            }

            // add bias
            *force.add(i as usize) += bias;
        }

        // handle actuator plugins (compute)
        if (*m).nplugin > 0 {
            let nslot = crate::engine::engine_plugin::mjp_plugin_count();
            for i in 0..(*m).nplugin as i32 {
                let slot = *(*m).plugin.add(i as usize);
                let plugin = crate::engine::engine_plugin::mjp_get_plugin_at_slot_unsafe(slot, nslot);
                if !plugin.is_null() && ((*plugin).capabilityflags & MJPLUGIN_ACTUATOR) != 0 {
                    if let Some(compute_fn) = (*plugin).compute {
                        let f: unsafe extern "C" fn(*const mjModel, *mut mjData, i32, i32) =
                            std::mem::transmute(compute_fn);
                        f(m, d, i, MJPLUGIN_ACTUATOR);
                    }
                }
            }
        }

        // clamp tendon total actuator force
        if tendon_frclimited != 0 {
            let tendon_total_force =
                crate::engine::engine_memory::mj_stack_alloc_num(d, ntendon as usize);
            crate::engine::engine_util_blas::mju_zero(tendon_total_force, ntendon);
            for i in 0..nu {
                if *(*m).actuator_trntype.add(i as usize) == MJTRN_TENDON {
                    let tendon_id = *(*m).actuator_trnid.add(2 * i as usize);
                    if *(*m).tendon_actfrclimited.add(tendon_id as usize) {
                        *tendon_total_force.add(tendon_id as usize) += *force.add(i as usize);
                    }
                }
            }

            // scale tendon actuator forces if limited and outside range
            for i in 0..nu {
                if *(*m).actuator_trntype.add(i as usize) != MJTRN_TENDON {
                    continue;
                }
                let tendon_id = *(*m).actuator_trnid.add(2 * i as usize);
                let tendon_force = *tendon_total_force.add(tendon_id as usize);
                if *(*m).tendon_actfrclimited.add(tendon_id as usize) && tendon_force != 0.0 {
                    let range = (*m).tendon_actfrcrange.add(2 * tendon_id as usize);
                    if tendon_force < *range.add(0) {
                        *force.add(i as usize) *= *range.add(0) / tendon_force;
                    } else if tendon_force > *range.add(1) {
                        *force.add(i as usize) *= *range.add(1) / tendon_force;
                    }
                }
            }
        }

        // clamp actuator_force
        clamp_vec(force, (*m).actuator_forcerange, (*m).actuator_forcelimited, nu, std::ptr::null());

        // add DC motor mechanical forces
        for i in 0..nu {
            if *(*m).actuator_biastype.add(i as usize) != MJBIAS_DCMOTOR {
                continue;
            }
            if sleep_filter
                && crate::engine::engine_sleep::mj_sleep_state(
                    m, d as *const mjData, MJOBJ_ACTUATOR as u32, i,
                ) == MJS_ASLEEP
            {
                continue;
            }
            if crate::engine::engine_support::mj_actuator_disabled(m, i) != 0
                || *(*m).actuator_plugin.add(i as usize) >= 0
            {
                continue;
            }

            let biasprm = (*m).actuator_biasprm.add((MJNBIAS * i) as usize);
            let dynprm = (*m).actuator_dynprm.add((MJNDYN * i) as usize);

            // cogging torque
            let A = *biasprm.add(0);
            if A != 0.0 {
                let Np = *biasprm.add(1);
                let phi = *biasprm.add(2);
                *force.add(i as usize) +=
                    A * (Np * *(*d).actuator_length.add(i as usize) + phi).sin();
            }

            // LuGre friction
            let sigma0 = *dynprm.add(5);
            if sigma0 > 0.0 {
                let sigma1 = *dynprm.add(6);
                let gainprm = (*m).actuator_gainprm.add((MJNGAIN * i) as usize);
                let slots = crate::engine::engine_util_misc::mj_dcmotor_slots(dynprm, gainprm);
                let adr = *(*m).actuator_actadr.add(i as usize) + slots.bristle;
                let z = *(*d).act.add(adr as usize);
                let z_dot = *(*d).act_dot.add(adr as usize);
                *force.add(i as usize) -= sigma0 * z + sigma1 * z_dot;
            }
        }

        // qfrc_actuator = moment' * force
        crate::engine::engine_util_sparse::mju_mul_mat_t_vec_sparse(
            (*d).qfrc_actuator, (*d).actuator_moment, force, nu, nv,
            (*d).moment_rownnz, (*d).moment_rowadr, (*d).moment_colind,
        );

        // actuator-level gravity compensation
        if (*m).ngravcomp > 0
            && ((*m).opt.disableflags & MJDSBL_GRAVITY) == 0
            && crate::engine::engine_util_blas::mju_norm3((*m).opt.gravity.as_ptr()) > MJMINVAL
        {
            let jnt_dofnum: [i32; 4] = [6, 3, 1, 1];
            let njnt = (*m).njnt as i32;
            for i in 0..njnt {
                if !*(*m).jnt_actgravcomp.add(i as usize) {
                    continue;
                }
                let dofnum = jnt_dofnum[*(*m).jnt_type.add(i as usize) as usize];
                let dofadr = *(*m).jnt_dofadr.add(i as usize);
                crate::engine::engine_util_blas::mju_add_to(
                    (*d).qfrc_actuator.add(dofadr as usize),
                    (*d).qfrc_gravcomp.add(dofadr as usize),
                    dofnum,
                );
            }
        }

        // clamp qfrc_actuator to joint-level actuator force limits
        clamp_vec(
            (*d).qfrc_actuator, (*m).jnt_actfrcrange,
            (*m).jnt_actfrclimited, (*m).njnt as i32, (*m).jnt_dofadr,
        );

        crate::engine::engine_memory::mj_free_stack(d);
    }
}

/// C: mj_fwdAcceleration (engine/engine_forward.h:90)
/// Calls: mj_solveLD, mj_xfrcAccumulate, mju_addTo, mju_addToInd, mju_copy, mju_copyInd, mju_sub, mju_subInd
#[allow(unused_variables, non_snake_case)]
pub fn mj_fwd_acceleration(m: *const mjModel, d: *mut mjData) {
    const MJ_ENBL_SLEEP: i32 = 1 << 4;

    // SAFETY: m, d are valid pointers (caller contract).
    unsafe {
        let sleep_filter = ((*m).opt.enableflags & MJ_ENBL_SLEEP) != 0
            && (*d).nv_awake < (*m).nv as i32;
        let nv: i32;
        let index: *const i32;

        // qfrc_smooth = qfrc_passive - qfrc_bias + qfrc_applied + qfrc_actuator
        if !sleep_filter {
            nv = (*m).nv as i32;
            index = std::ptr::null();
            crate::engine::engine_util_blas::mju_sub(
                (*d).qfrc_smooth, (*d).qfrc_passive, (*d).qfrc_bias, nv);
            crate::engine::engine_util_blas::mju_add_to(
                (*d).qfrc_smooth, (*d).qfrc_applied, nv);
            crate::engine::engine_util_blas::mju_add_to(
                (*d).qfrc_smooth, (*d).qfrc_actuator, nv);
        } else {
            nv = (*d).nv_awake;
            index = (*d).dof_awake_ind;
            crate::engine::engine_util_blas::mju_sub_ind(
                (*d).qfrc_smooth, (*d).qfrc_passive, (*d).qfrc_bias, index, nv);
            crate::engine::engine_util_blas::mju_add_to_ind(
                (*d).qfrc_smooth, (*d).qfrc_applied, index, nv);
            crate::engine::engine_util_blas::mju_add_to_ind(
                (*d).qfrc_smooth, (*d).qfrc_actuator, index, nv);
        }

        // qfrc_smooth += project(xfrc_applied)
        crate::engine::engine_support::mj_xfrc_accumulate(m, d, (*d).qfrc_smooth);

        // copy for in-place solve: qacc_smooth = qfrc_smooth
        if !sleep_filter {
            crate::engine::engine_util_blas::mju_copy((*d).qacc_smooth, (*d).qfrc_smooth, nv);
        } else {
            crate::engine::engine_util_blas::mju_copy_ind(
                (*d).qacc_smooth, (*d).qfrc_smooth, index, nv);
        }

        // qacc_smooth = M \ qfrc_smooth
        crate::engine::engine_core_smooth::mj_solve_ld(
            (*d).qacc_smooth, (*d).qLD, (*d).qLDiagInv, nv, 1,
            (*m).M_rownnz, (*m).M_rowadr, (*m).M_colind, index);
    }
}

/// C: mj_fwdConstraint (engine/engine_forward.h:93)
/// Calls: mj_dualFinish, mj_mulJacVec, mj_solCG, mj_solNewton, mj_solNoSlip, mj_solNoSlip_island, mj_solPGS, mju_copy, mju_dispatch, mju_gather, mju_message, mju_scatter, mju_subFrom, mju_zero, mju_zeroInt, warmstart
#[allow(unused_variables, non_snake_case)]
pub fn mj_fwd_constraint(m: *const mjModel, d: *mut mjData) {
    todo!() // mj_fwdConstraint
}

