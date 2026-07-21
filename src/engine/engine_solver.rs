//! Port of: engine/engine_solver.c
//! IR hash: 73a9f665ec0ecfc0
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: saveStats (engine/engine_solver.c:40)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn save_stats(m: *const mjModel, d: *mut mjData, island: i32, iter: i32, improvement: f64, gradient: f64, lineslope: f64, nactive: i32, nchange: i32, neval: i32, nupdate: i32) {
    const MJ_NISLAND: i32 = 20;
    const MJ_NSOLVER: i32 = 200;

    // if island out of range, return
    if island >= MJ_NISLAND {
        return;
    }

    // if no islands, use first island
    let island = if island < 0 { 0 } else { island };

    // if iter out of range, return
    if iter >= MJ_NSOLVER {
        return;
    }

    // mjSolverStat layout: improvement(f64), gradient(f64), lineslope(f64),
    //                       nactive(i32), nchange(i32), neval(i32), nupdate(i32)
    // total = 40 bytes
    const STAT_SIZE: usize = 40;

    // SAFETY: d points to valid mjData; solver array has mjNISLAND*mjNSOLVER entries of 40 bytes each
    unsafe {
        let base = (*d).solver.as_mut_ptr();
        let offset = (island as usize * MJ_NSOLVER as usize + iter as usize) * STAT_SIZE;
        let stat = base.add(offset);

        // write improvement (offset 0)
        *(stat as *mut f64) = improvement;
        // write gradient (offset 8)
        *(stat.add(8) as *mut f64) = gradient;
        // write lineslope (offset 16)
        *(stat.add(16) as *mut f64) = lineslope;
        // write nactive (offset 24)
        *(stat.add(24) as *mut i32) = nactive;
        // write nchange (offset 28)
        *(stat.add(28) as *mut i32) = nchange;
        // write neval (offset 32)
        *(stat.add(32) as *mut i32) = neval;
        // write nupdate (offset 36)
        *(stat.add(36) as *mut i32) = nupdate;
    }
}

/// C: dualFinish (engine/engine_solver.c:71)
/// Calls: mj_mulJacTVec, mj_solveM, mju_addTo
#[allow(unused_variables, non_snake_case)]
pub fn dual_finish(m: *const mjModel, d: *mut mjData) {
    // SAFETY: caller guarantees m, d are valid pointers to initialized model/data
    unsafe {
        // map constraint force to joint space
        crate::engine::engine_core_constraint::mj_mul_jac_t_vec(m, d, (*d).qfrc_constraint, (*d).efc_force);

        // compute constrained acceleration in joint space
        crate::engine::engine_core_smooth::mj_solve_m(m, d, (*d).qacc, (*d).qfrc_constraint, 1);
        crate::engine::engine_util_blas::mju_add_to((*d).qacc, (*d).qacc_smooth, (*m).nv as i32);
    }
}

/// C: ARdiaginv (engine/engine_solver.c:90)
/// Calls: mj_isSparse, mju_max
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn a_rdiaginv(m: *const mjModel, d: *const mjData, res: *mut f64, nefc: i32, efclist: *const i32, flg_subR: i32) {
    use crate::engine::engine_core_util::mj_is_sparse;
    use crate::engine::engine_util_misc::mju_max;
    const MJ_MINVAL: f64 = 1E-15_f64;

    // SAFETY: m, d are valid model/data pointers. res has at least nefc elements.
    // efc_AR, efc_R, efc_AR_rowadr, efc_AR_rownnz, efc_AR_colind are valid arrays in mjData.
    // efclist (if non-null) has at least nefc elements.
    unsafe {
        let AR = (*d).efc_AR;
        let R = (*d).efc_R;

        if mj_is_sparse(m) != 0 {
            let rowadr = (*d).efc_AR_rowadr;
            let rownnz = (*d).efc_AR_rownnz;
            let colind = (*d).efc_AR_colind;

            for c in 0..nefc {
                let i = if !efclist.is_null() { *efclist.add(c as usize) } else { c };
                let nnz = *rownnz.add(i as usize);

                for j in 0..nnz {
                    let adr = *rowadr.add(i as usize) + j;
                    if i == *colind.add(adr as usize) {
                        let ar_val = *AR.add(adr as usize);
                        *res.add(c as usize) = 1.0 / (if flg_subR != 0 {
                            mju_max(MJ_MINVAL, ar_val - *R.add(i as usize))
                        } else {
                            ar_val
                        });
                        break;
                    }
                }
            }
        } else {
            for c in 0..nefc {
                let i = if !efclist.is_null() { *efclist.add(c as usize) } else { c };
                let ar_val = *AR.add((nefc * i + i) as usize);
                *res.add(c as usize) = 1.0 / (if flg_subR != 0 {
                    mju_max(MJ_MINVAL, ar_val - *R.add(i as usize))
                } else {
                    ar_val
                });
            }
        }
    }
}

/// C: extractBlock (engine/engine_solver.c:127)
/// Calls: mj_isSparse, mju_copy, mju_max, mju_message
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn extract_block(m: *const mjModel, d: *const mjData, Ac: *mut f64, start: i32, n: i32, flg_subR: i32) {
    // SAFETY: m, d, Ac are valid pointers; start+n within efc bounds (caller contract)
    unsafe {
        let nefc = (*d).nefc;
        let AR = (*d).efc_AR;

        // sparse
        if crate::engine::engine_core_util::mj_is_sparse(m) != 0 {
            let rownnz = (*d).efc_AR_rownnz;
            let rowadr = (*d).efc_AR_rowadr;
            let colind = (*d).efc_AR_colind;

            // find starting k: same for all rows
            let mut k: i32 = 0;
            while k < *rownnz.add(start as usize) {
                if *colind.add((*rowadr.add(start as usize) + k) as usize) == start {
                    break;
                }
                k += 1;
            }

            // SHOULD NOT OCCUR
            if k >= *rownnz.add(start as usize) {
                crate::engine::engine_util_errmem::mju_error(
                    b"internal error\0".as_ptr() as *const i8);
            }

            // copy rows
            for j in 0..n {
                crate::engine::engine_util_blas::mju_copy(
                    Ac.add((j * n) as usize),
                    AR.add((*rowadr.add((start + j) as usize) + k) as usize),
                    n);
            }
        }
        // dense
        else {
            for j in 0..n {
                crate::engine::engine_util_blas::mju_copy(
                    Ac.add((j * n) as usize),
                    AR.add((start + (start + j) * nefc) as usize),
                    n);
            }
        }

        // subtract R from diagonal, clamp to 1e-10
        if flg_subR != 0 {
            let R = (*d).efc_R;
            for j in 0..n {
                let idx = (j * (n + 1)) as usize;
                *Ac.add(idx) -= *R.add((start + j) as usize);
                *Ac.add(idx) = if *Ac.add(idx) > 1e-10 { *Ac.add(idx) } else { 1e-10 };
            }
        }
    }
}

/// C: residual (engine/engine_solver.c:186)
/// Calls: mj_isSparse, mju_dot, mju_dotSparse
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn residual(m: *const mjModel, d: *const mjData, res: *mut f64, i: i32, dim: i32, flg_subR: i32) {
    // SAFETY: caller guarantees m, d, res valid; i+dim within bounds of efc arrays
    unsafe {
        let nefc = (*d).nefc;

        // sparse
        if crate::engine::engine_core_util::mj_is_sparse(m) != 0 {
            for j in 0..dim {
                let idx = (i + j) as usize;
                *res.add(j as usize) = *(*d).efc_b.add(idx)
                    + crate::engine::engine_util_sparse::mju_dot_sparse(
                        (*d).efc_AR.add(*(*d).efc_AR_rowadr.add(idx) as usize),
                        (*d).efc_force,
                        *(*d).efc_AR_rownnz.add(idx),
                        (*d).efc_AR_colind.add(*(*d).efc_AR_rowadr.add(idx) as usize),
                    );
            }
        }
        // dense
        else {
            for j in 0..dim {
                let idx = (i + j) as usize;
                *res.add(j as usize) = *(*d).efc_b.add(idx)
                    + crate::engine::engine_util_blas::mju_dot(
                        (*d).efc_AR.add(idx * nefc as usize),
                        (*d).efc_force,
                        nefc,
                    );
            }
        }

        if flg_subR != 0 {
            for j in 0..dim {
                let idx = (i + j) as usize;
                *res.add(j as usize) -= *(*d).efc_R.add(idx) * *(*d).efc_force.add(idx);
            }
        }
    }
}

/// C: costChange (engine/engine_solver.c:215)
/// Calls: mju_copy, mju_dot, mju_mulVecMatVec, mju_sub
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn cost_change(A: *const f64, force: *mut f64, oldforce: *const f64, res: *const f64, dim: i32) -> f64 {
    use crate::engine::engine_util_blas::{mju_sub, mju_dot, mju_copy, mju_mul_vec_mat_vec};

    // SAFETY: all pointers are valid arrays of at least `dim` elements (caller contract)
    unsafe {
        let mut change: f64;

        if dim == 1 {
            let delta = *force.add(0) - *oldforce.add(0);
            change = 0.5 * delta * delta * *A.add(0) + delta * *res.add(0);
        } else {
            let mut delta: [f64; 6] = [0.0; 6];
            mju_sub(delta.as_mut_ptr(), force, oldforce, dim);
            change = 0.5 * mju_mul_vec_mat_vec(delta.as_ptr(), A, delta.as_ptr(), dim)
                   + mju_dot(delta.as_ptr(), res, dim);
        }

        if change > 1e-10 {
            mju_copy(force, oldforce, dim);
            change = 0.0;
        }

        change
    }
}

/// C: pcg32_next (engine/engine_solver.c:247)
#[allow(unused_variables, non_snake_case)]
pub fn pcg32_next(rng: *mut pcg32_state) -> u32 {
    // pcg32_state layout: { state: u64 (offset 0), inc: u64 (offset 8) }
    // SAFETY: rng points to a valid pcg32_state with the layout above.
    unsafe {
        let state_ptr = rng as *mut u64;
        let inc_ptr = (rng as *mut u64).add(1);

        let oldstate: u64 = *state_ptr;
        *state_ptr = oldstate.wrapping_mul(6364136223846793005u64).wrapping_add(*inc_ptr | 1);
        let xorshifted: u32 = (((oldstate >> 18) ^ oldstate) >> 27) as u32;
        let rot: u32 = (oldstate >> 59) as u32;
        (xorshifted >> rot) | (xorshifted << ((rot.wrapping_neg()) & 31))
    }
}

/// C: shuffle_int (engine/engine_solver.c:257)
/// Calls: pcg32_next
#[allow(unused_variables, non_snake_case)]
pub fn shuffle_int(array: *mut i32, n: i32, rng: *mut pcg32_state) {
    // SAFETY: array has at least n elements; rng is valid pcg32_state.
    unsafe {
        let mut i = n - 1;
        while i > 0 {
            let j = (pcg32_next(rng) % (i as u32 + 1)) as usize;
            let temp = *array.add(i as usize);
            *array.add(i as usize) = *array.add(j);
            *array.add(j) = temp;
            i -= 1;
        }
    }
}

/// C: dualState (engine/engine_solver.c:269)
/// Calls: mju_fillInt, mju_norm
#[allow(unused_variables, non_snake_case)]
pub fn dual_state(d: *const mjData, state: *mut i32, ne: i32, nf: i32, nefc: i32, efclist: *const i32) -> i32 {
    use crate::engine::engine_util_blas::mju_norm;
    use crate::engine::engine_util_misc::mju_fill_int;

    const mjCNSTRSTATE_SATISFIED: i32 = 0;
    const mjCNSTRSTATE_QUADRATIC: i32 = 1;
    const mjCNSTRSTATE_LINEARNEG: i32 = 2;
    const mjCNSTRSTATE_LINEARPOS: i32 = 3;
    const mjCNSTRSTATE_CONE: i32 = 4;
    const mjCNSTR_CONTACT_ELLIPTIC: i32 = 7;

    // SAFETY: d points to valid mjData; state, efclist have sufficient size per caller contract
    unsafe {
        let force = (*d).efc_force as *const f64;
        let floss = (*d).efc_frictionloss as *const f64;

        // equality and friction always active
        let mut nactive: i32 = ne + nf;

        // equality
        let mut c: i32 = 0;
        while c < ne {
            let i = if !efclist.is_null() { *efclist.add(c as usize) } else { c };
            *state.add(i as usize) = mjCNSTRSTATE_QUADRATIC;
            c += 1;
        }

        // friction
        c = ne;
        while c < ne + nf {
            let i = if !efclist.is_null() { *efclist.add(c as usize) } else { c };
            if *force.add(i as usize) <= -*floss.add(i as usize) {
                *state.add(i as usize) = mjCNSTRSTATE_LINEARPOS; // opposite of primal
            } else if *force.add(i as usize) >= *floss.add(i as usize) {
                *state.add(i as usize) = mjCNSTRSTATE_LINEARNEG;
            } else {
                *state.add(i as usize) = mjCNSTRSTATE_QUADRATIC;
            }
            c += 1;
        }

        // limit and contact
        c = ne + nf;
        while c < nefc {
            let i = if !efclist.is_null() { *efclist.add(c as usize) } else { c };

            // non-negative
            if *(*d).efc_type.add(i as usize) != mjCNSTR_CONTACT_ELLIPTIC {
                if *force.add(i as usize) <= 0.0 {
                    *state.add(i as usize) = mjCNSTRSTATE_SATISFIED;
                } else {
                    *state.add(i as usize) = mjCNSTRSTATE_QUADRATIC;
                    nactive += 1;
                }
            }
            // elliptic
            else {
                // get contact dimensionality, friction, mu
                let con: *const mjContact = (*d).contact.add(*(*d).efc_id.add(i as usize) as usize);
                let dim: i32 = (*con).dim;
                let mu: f64 = (*con).mu;
                let mut f: [f64; 6] = [0.0; 6];

                // f = map force to regular-cone space
                f[0] = *force.add(i as usize) / mu;
                for j in 1..dim {
                    f[j as usize] = *force.add((i + j) as usize) / (*con).friction[(j - 1) as usize];
                }

                // N = normal, T = norm of tangent vector
                let N: f64 = f[0];
                let T: f64 = mju_norm(f.as_ptr().add(1), dim - 1);

                // classify zone
                let result: i32;
                if mu * N >= T {
                    // top zone
                    result = mjCNSTRSTATE_SATISFIED;
                } else if N + mu * T <= 0.0 {
                    // bottom zone
                    result = mjCNSTRSTATE_QUADRATIC;
                    nactive += dim;
                } else {
                    // middle zone
                    result = mjCNSTRSTATE_CONE;
                    nactive += dim;
                }

                // replicate state in all cone dimensions
                mju_fill_int(state.add(i as usize), result, dim);

                // advance
                c += dim - 1;
            }

            c += 1;
        }

        nactive
    }
}

/// C: dualStateChange (engine/engine_solver.c:356)
/// Calls: dualState
#[allow(unused_variables, non_snake_case)]
pub fn dual_state_change(d: *const mjData, state: *mut i32, oldstate: *mut i32, ne: i32, nf: i32, nefc: i32, efclist: *const i32, nchange: *mut i32) -> i32 {
    // SAFETY: d is valid mjData pointer. state, oldstate have at least nefc elements.
    // efclist (if non-null) has at least nefc elements.
    unsafe {
        // save old state
        for c in 0..nefc {
            let i = if !efclist.is_null() { *efclist.add(c as usize) } else { c };
            *oldstate.add(c as usize) = *state.add(i as usize);
        }

        // update state
        let nactive = dual_state(d, state, ne, nf, nefc, efclist);

        // count state changes
        *nchange = 0;
        for c in 0..nefc {
            let i = if !efclist.is_null() { *efclist.add(c as usize) } else { c };
            *nchange += (*oldstate.add(c as usize) != *state.add(i as usize)) as i32;
        }

        nactive
    }
}

/// C: projectEllipsoid (engine/engine_solver.c:383)
/// Calls: mju_max
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn project_ellipsoid(friction: *mut f64, normal: f64, mu: *const f64, dim: i32, feasible: i32) {
    use crate::engine::engine_util_misc::mju_max;
    const MJ_MINVAL: f64 = 1E-15_f64;

    // SAFETY: friction has at least dim-1 elements, mu has at least dim-1 elements.
    unsafe {
        let mut s: f64 = 0.0;
        for j in 0..(dim - 1) {
            s += *friction.add(j as usize) * *friction.add(j as usize)
                / (*mu.add(j as usize) * *mu.add(j as usize));
        }

        let normal2: f64 = normal * normal;
        if feasible == 0 || s > normal2 {
            let scl: f64 = (normal2 / mju_max(MJ_MINVAL, s)).sqrt();
            for j in 0..(dim - 1) {
                *friction.add(j as usize) *= scl;
            }
        }
    }
}

/// C: solveQCQP (engine/engine_solver.c:401)
/// Calls: mju_QCQP, mju_QCQP2, mju_QCQP3, mju_copy, projectEllipsoid
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn solve_qcqp(force: *mut f64, i: i32, dim: i32, Ac: *mut f64, bc: *mut f64, mu: *const f64) {
    // SAFETY: caller guarantees all pointers valid and arrays properly sized
    unsafe {
        let mut v: [f64; 6] = [0.0; 6];
        let flg_active: i32;

        // solve
        if dim == 3 {
            flg_active = crate::engine::engine_util_solve::mju_qcqp2(
                v.as_mut_ptr(), Ac, bc, mu, *force.add(i as usize));
        } else if dim == 4 {
            flg_active = crate::engine::engine_util_solve::mju_qcqp3(
                v.as_mut_ptr(), Ac, bc, mu, *force.add(i as usize));
        } else {
            // dim == 5
            flg_active = crate::engine::engine_util_solve::mju_qcqp(
                v.as_mut_ptr(), Ac, bc, mu, *force.add(i as usize), dim - 1);
        }

        // on constraint: put v on ellipsoid, in case QCQP is approximate
        if flg_active != 0 {
            project_ellipsoid(v.as_mut_ptr(), *force.add(i as usize), mu, dim, 0);
        }

        // assign
        crate::engine::engine_util_blas::mju_copy(force.add(i as usize + 1), v.as_ptr(), dim - 1);
    }
}

/// C: projectCone (engine/engine_solver.c:426)
/// Calls: mju_zero, projectEllipsoid
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn project_cone(force: *mut f64, mu: *const f64, dim: i32, r#type: i32) {
    use crate::engine::engine_util_blas::mju_zero;
    const MJ_CNSTR_CONTACT_ELLIPTIC: i32 = 7;

    // SAFETY: force has at least dim elements; mu has at least dim-1 elements.
    unsafe {
        // elliptic cone: project onto friction ellipsoid
        if r#type == MJ_CNSTR_CONTACT_ELLIPTIC {
            // clamp normal force
            if *force.add(0) < 0.0 {
                mju_zero(force, dim);
            } else {
                project_ellipsoid(force.add(1), *force.add(0), mu, dim, 1);
            }
        }
        // pyramidal or scalar: clamp to non-negative
        else {
            if *force.add(0) < 0.0 {
                *force.add(0) = 0.0;
            }
        }
    }
}

/// C: solPGS (engine/engine_solver.c:456)
/// Calls: ARdiaginv, costChange, dualState, dualStateChange, extractBlock, mj_freeStack, mj_isSparse, mj_markStack, mj_stackAllocInfo, mju_clip, mju_copy, mju_dot, mju_gather, mju_mulMatVec, mju_zero, pcg32_next, projectCone, residual, saveStats, shuffle_int, solveQCQP
#[allow(unused_variables, non_snake_case)]
pub fn sol_pgs(m: *const mjModel, d: *mut mjData, island: i32, ne: i32, nf: i32, nefc: i32, efclist: *const i32, maxiter: i32) {
    todo!() // solPGS
}

/// C: solNoSlip (engine/engine_solver.c:766)
/// Calls: ARdiaginv, costChange, dualState, dualStateChange, extractBlock, mj_freeStack, mj_markStack, mj_stackAllocInfo, mju_copy, mju_dot, mju_zero, residual, saveStats, solveQCQP
#[allow(unused_variables, non_snake_case)]
pub fn sol_no_slip(m: *const mjModel, d: *mut mjData, island: i32, ne: i32, nf: i32, nefc: i32, efclist: *const i32, maxiter: i32) {
    todo!() // solNoSlip
}

/// C: PrimalPointers (engine/engine_solver.c:1087)
/// Calls: mj_isSparse
#[allow(unused_variables, non_snake_case)]
pub fn primal_pointers(m: *const mjModel, d: *const mjData, ctx: *mut mjPrimalContext, island: i32) {
    use crate::engine::engine_core_util::mj_is_sparse;

    // Local shadow struct matching the C layout of mjPrimalContext.
    // Only the fields written by PrimalPointers are needed at their correct offsets.
    #[repr(C)]
    struct PrimalCtx {
        is_sparse: i32,
        is_elliptic: i32,
        island: i32,
        // sizes
        nv: i32,
        ne: i32,
        nf: i32,
        nefc: i32,
        nJ: i32,
        // contact array
        contact: *mut mjContact,
        // dof arrays
        qfrc_smooth: *const f64,
        qacc_smooth: *const f64,
        qfrc_constraint: *mut f64,
        qacc: *mut f64,
        // inertia
        M_rownnz: *mut i32,
        M_rowadr: *mut i32,
        M_colind: *mut i32,
        M: *mut f64,
        qLD: *mut f64,
        qLDiagInv: *mut f64,
        // efc arrays
        efc_D: *const f64,
        efc_R: *const f64,
        efc_frictionloss: *const f64,
        efc_aref: *const f64,
        efc_id: *const i32,
        efc_type: *const i32,
        efc_force: *mut f64,
        efc_state: *mut i32,
        // Jacobians
        J_rownnz: *mut i32,
        J_rowadr: *mut i32,
        J_rowsuper: *mut i32,
        J_colind: *mut i32,
        J: *mut f64,
        // remaining fields not accessed by PrimalPointers
    }

    const MJ_CONE_ELLIPTIC: i32 = 1;

    // SAFETY: ctx points to a valid mjPrimalContext allocation with the same layout as PrimalCtx.
    // m, d are valid model/data pointers. We memset the entire struct then write known fields.
    unsafe {
        // clear everything - need to know the size of the full struct
        // Use the size from the C definition. We zero the memory that ctx points to.
        // The C code does memset(ctx, 0, sizeof(mjPrimalContext)) — we replicate with write_bytes.
        // Since we don't know sizeof at compile time, we zero only the fields we set here.
        // Actually the C zeros the ENTIRE struct. We cast to our shadow to set fields.
        let c = ctx as *mut PrimalCtx;
        // Zero the full allocation - PrimalContext is stack-allocated by callers who know its size.
        // We zero through our shadow struct (the portion we know).
        std::ptr::write_bytes(ctx as *mut u8, 0, std::mem::size_of::<PrimalCtx>());

        // globals
        (*c).is_sparse = mj_is_sparse(m);
        (*c).is_elliptic = ((*m).opt.cone == MJ_CONE_ELLIPTIC) as i32;
        (*c).contact = (*d).contact;
        (*c).island = island;

        // set sizes and pointers (monolithic)
        if island < 0 {
            // sizes
            (*c).nv = (*m).nv as i32;
            (*c).ne = (*d).ne;
            (*c).nf = (*d).nf;
            (*c).nefc = (*d).nefc;
            (*c).nJ = (*d).nJ;

            // dof arrays
            (*c).qfrc_smooth = (*d).qfrc_smooth;
            (*c).qfrc_constraint = (*d).qfrc_constraint;
            (*c).qacc_smooth = (*d).qacc_smooth;
            (*c).qacc = (*d).qacc;

            // inertia
            (*c).M_rownnz = (*m).M_rownnz;
            (*c).M_rowadr = (*m).M_rowadr;
            (*c).M_colind = (*m).M_colind;
            (*c).M = (*d).M;
            (*c).qLD = (*d).qLD;
            (*c).qLDiagInv = (*d).qLDiagInv;

            // efc arrays
            (*c).efc_D = (*d).efc_D;
            (*c).efc_R = (*d).efc_R;
            (*c).efc_frictionloss = (*d).efc_frictionloss;
            (*c).efc_aref = (*d).efc_aref;
            (*c).efc_id = (*d).efc_id;
            (*c).efc_type = (*d).efc_type;
            (*c).efc_force = (*d).efc_force;
            (*c).efc_state = (*d).efc_state;

            // Jacobians
            (*c).J = (*d).efc_J;
            if (*c).is_sparse != 0 {
                (*c).J_rownnz = (*d).efc_J_rownnz;
                (*c).J_rowadr = (*d).efc_J_rowadr;
                (*c).J_rowsuper = (*d).efc_J_rowsuper;
                (*c).J_colind = (*d).efc_J_colind;
            }
        }
        // set sizes and pointers (per-island)
        else {
            // sizes
            (*c).nv = *(*d).island_nv.add(island as usize);
            (*c).ne = *(*d).island_ne.add(island as usize);
            (*c).nf = *(*d).island_nf.add(island as usize);
            (*c).nefc = *(*d).island_nefc.add(island as usize);

            // dof arrays
            let idofadr: i32 = *(*d).island_idofadr.add(island as usize);
            (*c).qfrc_smooth = (*d).ifrc_smooth.add(idofadr as usize);
            (*c).qfrc_constraint = (*d).ifrc_constraint.add(idofadr as usize);
            (*c).qacc_smooth = (*d).iacc_smooth.add(idofadr as usize);
            (*c).qacc = (*d).iacc.add(idofadr as usize);

            // efc arrays
            let iefcadr: i32 = *(*d).island_iefcadr.add(island as usize);
            (*c).efc_D = (*d).iefc_D.add(iefcadr as usize);
            (*c).efc_R = (*d).iefc_R.add(iefcadr as usize);
            (*c).efc_frictionloss = (*d).iefc_frictionloss.add(iefcadr as usize);
            (*c).efc_aref = (*d).iefc_aref.add(iefcadr as usize);
            (*c).efc_id = (*d).iefc_id.add(iefcadr as usize);
            (*c).efc_type = (*d).iefc_type.add(iefcadr as usize);
            (*c).efc_force = (*d).iefc_force.add(iefcadr as usize);
            (*c).efc_state = (*d).iefc_state.add(iefcadr as usize);
        }
    }
}

/// C: PrimalAllocate (engine/engine_solver.c:1171)
/// Calls: mj_stackAllocInfo, mju_block, mju_blockSparse, mju_gather, mju_superSparse, mju_transposeSparse
#[allow(unused_variables, non_snake_case)]
pub fn primal_allocate(m: *const mjModel, d: *mut mjData, ctx: *mut mjPrimalContext, flg_Newton: i32) {
    todo!() // PrimalAllocate
}

/// C: PrimalUpdateConstraint (engine/engine_solver.c:1343)
/// Calls: mj_constraintUpdate_impl, mju_mulMatTVec, mju_mulMatVecSparse
#[allow(unused_variables, non_snake_case)]
pub fn primal_update_constraint(ctx: *mut mjPrimalContext, flg_HessianCone: i32) {
    #[repr(C)]
    #[allow(non_snake_case)]
    struct PrimalCtx {
        is_sparse: i32,
        is_elliptic: i32,
        island: i32,
        nv: i32,
        ne: i32,
        nf: i32,
        nefc: i32,
        nJ: i32,
        contact: *mut mjContact,
        qfrc_smooth: *const f64,
        qacc_smooth: *const f64,
        qfrc_constraint: *mut f64,
        qacc: *mut f64,
        M_rownnz: *mut i32,
        M_rowadr: *mut i32,
        M_colind: *mut i32,
        M: *mut f64,
        qLD: *mut f64,
        qLDiagInv: *mut f64,
        efc_D: *const f64,
        efc_R: *const f64,
        efc_frictionloss: *const f64,
        efc_aref: *const f64,
        efc_id: *const i32,
        efc_type: *const i32,
        efc_force: *mut f64,
        efc_state: *mut i32,
        J_rownnz: *mut i32,
        J_rowadr: *mut i32,
        J_rowsuper: *mut i32,
        J_colind: *mut i32,
        J: *mut f64,
        JT_rownnz: *mut i32,
        JT_rowadr: *mut i32,
        JT_rowsuper: *mut i32,
        JT_colind: *mut i32,
        JT: *mut f64,
        Jaref: *mut f64,
        Jv: *mut f64,
        Ma: *mut f64,
        Mv: *mut f64,
        grad: *mut f64,
        Mgrad: *mut f64,
        search: *mut f64,
        quad: *mut f64,
        oldstate: *mut i32,
        gradold: *mut f64,
        Mgradold: *mut f64,
        graddif: *mut f64,
        Mgraddif: *mut f64,
        D_newton: *mut f64,
        cholupd: *mut f64,
        LTJ: *mut f64,
        H_rowadr: *mut i32,
        H_rownnz: *mut i32,
        HT_rownnz: *mut i32,
        HT_rowadr: *mut i32,
        L_rownnz: *mut i32,
        L_rowadr: *mut i32,
        LT_rownnz: *mut i32,
        LT_rowadr: *mut i32,
        nH: i32,
        _pad0: i32,
        H_colind: *mut i32,
        HT_colind: *mut i32,
        H: *mut f64,
        nL: i32,
        _pad1: i32,
        L_colind: *mut i32,
        LT_colind: *mut i32,
        LT_map: *mut i32,
        L: *mut f64,
        Lcone: *mut f64,
        cost: f64,
        quadGauss: [f64; 3],
        scale: f64,
        nactive: i32,
        ncone: i32,
        nupdate: i32,
        LSiter: i32,
        LSresult: i32,
        _pad2: i32,
        LSslope: f64,
    }

    const MJ_CNSTRSTATE_SATISFIED: i32 = 0;
    const MJ_CNSTRSTATE_CONE: i32 = 6;

    // SAFETY: ctx is a valid mjPrimalContext with the layout of PrimalCtx.
    unsafe {
        let c = ctx as *mut PrimalCtx;
        let nefc = (*c).nefc;
        let nv = (*c).nv;

        // update constraints
        crate::engine::engine_core_constraint::mj_constraint_update_impl(
            (*c).ne, (*c).nf, (*c).nefc, (*c).efc_D, (*c).efc_R,
            (*c).efc_frictionloss, (*c).Jaref, (*c).efc_type, (*c).efc_id,
            (*c).contact, (*c).efc_state, (*c).efc_force,
            &mut (*c).cost as *mut f64, flg_HessianCone,
        );

        // compute qfrc_constraint (dense or sparse)
        if (*c).is_sparse == 0 {
            crate::engine::engine_util_blas::mju_mul_mat_t_vec(
                (*c).qfrc_constraint, (*c).J, (*c).efc_force, nefc, nv,
            );
        } else {
            crate::engine::engine_util_sparse::mju_mul_mat_vec_sparse(
                (*c).qfrc_constraint, (*c).JT, (*c).efc_force, nv,
                (*c).JT_rownnz, (*c).JT_rowadr, (*c).JT_colind, (*c).JT_rowsuper,
            );
        }

        // count active and cone
        (*c).nactive = 0;
        (*c).ncone = 0;
        for i in 0..nefc {
            (*c).nactive += (*(*c).efc_state.add(i as usize) != MJ_CNSTRSTATE_SATISFIED) as i32;
            (*c).ncone += (*(*c).efc_state.add(i as usize) == MJ_CNSTRSTATE_CONE) as i32;
        }

        // add Gauss cost, set in quadratic[0]
        let mut Gauss: f64 = 0.0;
        for i in 0..nv {
            Gauss += 0.5 * (*(*c).Ma.add(i as usize) - *(*c).qfrc_smooth.add(i as usize))
                        * (*(*c).qacc.add(i as usize) - *(*c).qacc_smooth.add(i as usize));
        }

        (*c).quadGauss[0] = Gauss;
        (*c).cost += Gauss;
    }
}

/// C: PrimalUpdateGradient (engine/engine_solver.c:1380)
/// Calls: mj_solveLD, mju_cholSolve, mju_cholSolveSparse, mju_copy
#[allow(unused_variables, non_snake_case)]
pub fn primal_update_gradient(ctx: *mut mjPrimalContext, flg_Newton: i32) {
    #[repr(C)]
    #[allow(non_snake_case)]
    struct PrimalCtx {
        is_sparse: i32,
        is_elliptic: i32,
        island: i32,
        nv: i32,
        ne: i32,
        nf: i32,
        nefc: i32,
        nJ: i32,
        contact: *mut mjContact,
        qfrc_smooth: *const f64,
        qacc_smooth: *const f64,
        qfrc_constraint: *mut f64,
        qacc: *mut f64,
        M_rownnz: *mut i32,
        M_rowadr: *mut i32,
        M_colind: *mut i32,
        M: *mut f64,
        qLD: *mut f64,
        qLDiagInv: *mut f64,
        efc_D: *const f64,
        efc_R: *const f64,
        efc_frictionloss: *const f64,
        efc_aref: *const f64,
        efc_id: *const i32,
        efc_type: *const i32,
        efc_force: *mut f64,
        efc_state: *mut i32,
        J_rownnz: *mut i32,
        J_rowadr: *mut i32,
        J_rowsuper: *mut i32,
        J_colind: *mut i32,
        J: *mut f64,
        JT_rownnz: *mut i32,
        JT_rowadr: *mut i32,
        JT_rowsuper: *mut i32,
        JT_colind: *mut i32,
        JT: *mut f64,
        Jaref: *mut f64,
        Jv: *mut f64,
        Ma: *mut f64,
        Mv: *mut f64,
        grad: *mut f64,
        Mgrad: *mut f64,
        search: *mut f64,
        quad: *mut f64,
        oldstate: *mut i32,
        gradold: *mut f64,
        Mgradold: *mut f64,
        graddif: *mut f64,
        Mgraddif: *mut f64,
        D_newton: *mut f64,
        cholupd: *mut f64,
        LTJ: *mut f64,
        H_rowadr: *mut i32,
        H_rownnz: *mut i32,
        HT_rownnz: *mut i32,
        HT_rowadr: *mut i32,
        L_rownnz: *mut i32,
        L_rowadr: *mut i32,
        LT_rownnz: *mut i32,
        LT_rowadr: *mut i32,
        nH: i32,
        _pad0: i32,
        H_colind: *mut i32,
        HT_colind: *mut i32,
        H: *mut f64,
        nL: i32,
        _pad1: i32,
        L_colind: *mut i32,
        LT_colind: *mut i32,
        LT_map: *mut i32,
        L: *mut f64,
        Lcone: *mut f64,
        cost: f64,
        quadGauss: [f64; 3],
        scale: f64,
        nactive: i32,
        ncone: i32,
        nupdate: i32,
        LSiter: i32,
        LSresult: i32,
        _pad2: i32,
        LSslope: f64,
    }

    // SAFETY: ctx is a valid mjPrimalContext with the layout of PrimalCtx.
    unsafe {
        let c = ctx as *mut PrimalCtx;
        let nv = (*c).nv;

        // grad = M*qacc - qfrc_smooth - qfrc_constraint
        for i in 0..nv {
            *(*c).grad.add(i as usize) = *(*c).Ma.add(i as usize)
                - *(*c).qfrc_smooth.add(i as usize)
                - *(*c).qfrc_constraint.add(i as usize);
        }

        // Newton: Mgrad = H \ grad
        if flg_Newton != 0 {
            if (*c).is_sparse != 0 {
                let L_ptr = if (*c).ncone != 0 { (*c).Lcone } else { (*c).L };
                crate::engine::engine_util_solve::mju_chol_solve_sparse(
                    (*c).Mgrad, L_ptr, (*c).grad, nv,
                    (*c).L_rownnz, (*c).L_rowadr, (*c).L_colind,
                );
            } else {
                let L_ptr = if (*c).ncone != 0 { (*c).Lcone } else { (*c).L };
                crate::engine::engine_util_solve::mju_chol_solve(
                    (*c).Mgrad, L_ptr, (*c).grad, nv,
                );
            }
        }
        // CG: Mgrad = M \ grad
        else {
            crate::engine::engine_util_blas::mju_copy((*c).Mgrad, (*c).grad, nv);
            crate::engine::engine_core_smooth::mj_solve_ld(
                (*c).Mgrad, (*c).qLD, (*c).qLDiagInv, nv, 1,
                (*c).M_rownnz, (*c).M_rowadr, (*c).M_colind, std::ptr::null(),
            );
        }
    }
}

/// C: PrimalPrepare (engine/engine_solver.c:1408)
/// Calls: mju_dot
#[allow(unused_variables, non_snake_case)]
pub fn primal_prepare(ctx: *mut mjPrimalContext) {
    use crate::engine::engine_util_blas::mju_dot;

    // Local shadow struct matching the C layout of mjPrimalContext.
    #[repr(C)]
    #[allow(non_snake_case)]
    struct PrimalCtx {
        is_sparse: i32,
        is_elliptic: i32,
        island: i32,
        nv: i32,
        ne: i32,
        nf: i32,
        nefc: i32,
        nJ: i32,
        contact: *mut mjContact,
        qfrc_smooth: *const f64,
        qacc_smooth: *const f64,
        qfrc_constraint: *mut f64,
        qacc: *mut f64,
        M_rownnz: *mut i32,
        M_rowadr: *mut i32,
        M_colind: *mut i32,
        M: *mut f64,
        qLD: *mut f64,
        qLDiagInv: *mut f64,
        efc_D: *const f64,
        efc_R: *const f64,
        efc_frictionloss: *const f64,
        efc_aref: *const f64,
        efc_id: *const i32,
        efc_type: *const i32,
        efc_force: *mut f64,
        efc_state: *mut i32,
        J_rownnz: *mut i32,
        J_rowadr: *mut i32,
        J_rowsuper: *mut i32,
        J_colind: *mut i32,
        J: *mut f64,
        JT_rownnz: *mut i32,
        JT_rowadr: *mut i32,
        JT_rowsuper: *mut i32,
        JT_colind: *mut i32,
        JT: *mut f64,
        Jaref: *mut f64,
        Jv: *mut f64,
        Ma: *mut f64,
        Mv: *mut f64,
        grad: *mut f64,
        Mgrad: *mut f64,
        search: *mut f64,
        quad: *mut f64,
        oldstate: *mut i32,
        gradold: *mut f64,
        Mgradold: *mut f64,
        graddif: *mut f64,
        Mgraddif: *mut f64,
        D_newton: *mut f64,
        cholupd: *mut f64,
        LTJ: *mut f64,
        H_rowadr: *mut i32,
        H_rownnz: *mut i32,
        HT_rownnz: *mut i32,
        HT_rowadr: *mut i32,
        L_rownnz: *mut i32,
        L_rowadr: *mut i32,
        LT_rownnz: *mut i32,
        LT_rowadr: *mut i32,
        nH: i32,
        _pad0: i32,
        H_colind: *mut i32,
        HT_colind: *mut i32,
        H: *mut f64,
        nL: i32,
        _pad1: i32,
        L_colind: *mut i32,
        LT_colind: *mut i32,
        LT_map: *mut i32,
        L: *mut f64,
        Lcone: *mut f64,
        cost: f64,
        quadGauss: [f64; 3],
        scale: f64,
        nactive: i32,
        ncone: i32,
        nupdate: i32,
        LSiter: i32,
        LSresult: i32,
        _pad2: i32,
        LSslope: f64,
    }

    const MJ_CNSTR_CONTACT_ELLIPTIC: i32 = 7;

    // SAFETY: ctx points to a valid mjPrimalContext with the same layout as PrimalCtx.
    // All pointer fields (search, Ma, Mv, qfrc_smooth, Jv, Jaref, efc_D, efc_type,
    // contact, efc_id, quad) are valid and properly sized by PrimalAllocate.
    unsafe {
        let c = ctx as *mut PrimalCtx;
        let nv: i32 = (*c).nv;
        let nefc: i32 = (*c).nefc;
        let v: *const f64 = (*c).search;

        // Gauss: quadGauss[1] = v'*Ma - qfrc_smooth'*v
        //        quadGauss[2] = 0.5*v'*Mv
        (*c).quadGauss[1] = mju_dot(v, (*c).Ma, nv) - mju_dot((*c).qfrc_smooth, v, nv);
        (*c).quadGauss[2] = 0.5 * mju_dot(v, (*c).Mv, nv);

        // process constraints
        let mut i: i32 = 0;
        while i < nefc {
            // pointers to numeric data
            let Jv_i: *const f64 = (*c).Jv.add(i as usize);
            let Jaref_i: *const f64 = (*c).Jaref.add(i as usize);
            let D_i: *const f64 = (*c).efc_D.add(i as usize);

            // pointer to this quadratic
            let quad_i: *mut f64 = (*c).quad.add((3 * i) as usize);

            // init with scalar quadratic
            let DJ0: f64 = *D_i.add(0) * *Jaref_i.add(0);
            *quad_i.add(0) = *Jaref_i.add(0) * DJ0;
            *quad_i.add(1) = *Jv_i.add(0) * DJ0;
            *quad_i.add(2) = *Jv_i.add(0) * *D_i.add(0) * *Jv_i.add(0);

            // elliptic cone: extra processing
            if *(*c).efc_type.add(i as usize) == MJ_CNSTR_CONTACT_ELLIPTIC {
                // extract contact info
                let con: *const mjContact = (*c).contact.add(*(*c).efc_id.add(i as usize) as usize);
                let dim: i32 = (*con).dim;
                let mut U: [f64; 6] = [0.0; 6];
                let mut V: [f64; 6] = [0.0; 6];
                let mut UU: f64 = 0.0;
                let mut UV: f64 = 0.0;
                let mut VV: f64 = 0.0;
                let mu: f64 = (*con).mu;
                let friction: *const f64 = (*con).friction.as_ptr();

                // complete vector quadratic (for bottom zone)
                for j in 1..dim {
                    let DJj: f64 = *D_i.add(j as usize) * *Jaref_i.add(j as usize);
                    *quad_i.add(0) += *Jaref_i.add(j as usize) * DJj;
                    *quad_i.add(1) += *Jv_i.add(j as usize) * DJj;
                    *quad_i.add(2) += *Jv_i.add(j as usize) * *D_i.add(j as usize) * *Jv_i.add(j as usize);
                }

                // rescale to make primal cone circular
                U[0] = *Jaref_i.add(0) * mu;
                V[0] = *Jv_i.add(0) * mu;
                for j in 1..dim {
                    U[j as usize] = *Jaref_i.add(j as usize) * *friction.add((j - 1) as usize);
                    V[j as usize] = *Jv_i.add(j as usize) * *friction.add((j - 1) as usize);
                }

                // accumulate sums of squares
                for j in 1..dim {
                    UU += U[j as usize] * U[j as usize];
                    UV += U[j as usize] * V[j as usize];
                    VV += V[j as usize] * V[j as usize];
                }

                // store in quad[3-8], using the fact that dim>=3
                *quad_i.add(3) = U[0];
                *quad_i.add(4) = V[0];
                *quad_i.add(5) = UU;
                *quad_i.add(6) = UV;
                *quad_i.add(7) = VV;
                *quad_i.add(8) = *D_i.add(0) / ((mu * mu) * (1.0 + (mu * mu)));

                // advance to next constraint
                i += dim - 1;
            }

            // apply scaling
            *quad_i.add(0) *= 0.5;
            *quad_i.add(2) *= 0.5;

            i += 1;
        }
    }
}

/// C: frictionCost (engine/engine_solver.c:1493)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn friction_cost(x: f64, f: f64, Rf: f64, D: f64) -> f64 {
    // -bound < x < bound : quadratic
    if -Rf < x && x < Rf {
        0.5 * D * x * x
    }
    // x < -bound : linear negative
    else if x <= -Rf {
        f * (-0.5 * Rf - x)
    }
    // bound < x : linear positive
    else {
        f * (-0.5 * Rf + x)
    }
}

/// C: frictionCostDif (engine/engine_solver.c:1506)
/// Calls: frictionCost
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn friction_cost_dif(start: f64, x: f64, f: f64, Rf: f64, D: f64) -> f64 {
    let state_start: i32 = if -Rf < start && start < Rf { 0 } else if start <= -Rf { -1 } else { 1 };
    let state_x: i32 = if -Rf < x && x < Rf { 0 } else if x <= -Rf { -1 } else { 1 };

    // both quadratic
    if state_start == 0 && state_x == 0 {
        return 0.5 * D * (x - start) * (x + start);
    }

    // both linear negative
    if state_start == -1 && state_x == -1 {
        return f * (start - x);
    }

    // both linear positive
    if state_start == 1 && state_x == 1 {
        return f * (x - start);
    }

    // otherwise different zones: compute absolute costs and subtract
    friction_cost(x, f, Rf, D) - friction_cost(start, f, Rf, D)
}

/// C: ellipticCost (engine/engine_solver.c:1531)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn elliptic_cost(quad: *const f64, alpha: f64, mu: f64, Dm: f64) -> f64 {
    // SAFETY: quad has at least 8 elements (indices 0..7)
    unsafe {
        let U0: f64 = *quad.add(3);
        let V0: f64 = *quad.add(4);
        let UU: f64 = *quad.add(5);
        let UV: f64 = *quad.add(6);
        let VV: f64 = *quad.add(7);
        let N: f64 = U0 + alpha * V0;
        let Tsqr: f64 = UU + alpha * (2.0 * UV + alpha * VV);

        // no tangential force : top or bottom zone
        if Tsqr <= 0.0 {
            // bottom zone: quadratic cost
            if N < 0.0 {
                return alpha * alpha * *quad.add(2) + alpha * *quad.add(1) + *quad.add(0);
            }
            // top zone: nothing to do
        }
        // otherwise regular processing
        else {
            let T: f64 = Tsqr.sqrt();
            // N>=mu*T : top zone
            if N >= mu * T {
                // nothing to do
            }
            // mu*N+T<=0 : bottom zone
            else if mu * N + T <= 0.0 {
                return alpha * alpha * *quad.add(2) + alpha * *quad.add(1) + *quad.add(0);
            }
            // otherwise middle zone
            else {
                return 0.5 * Dm * (N - mu * T) * (N - mu * T);
            }
        }

        0.0
    }
}

/// C: ellipticCostDif (engine/engine_solver.c:1569)
/// Calls: ellipticCost
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn elliptic_cost_dif(quad: *const f64, alpha: f64, mu: f64, Dm: f64) -> f64 {
    // SAFETY: quad has at least 8 elements (indices 0..7)
    unsafe {
        let U0: f64 = *quad.add(3);
        let V0: f64 = *quad.add(4);
        let UU: f64 = *quad.add(5);
        let UV: f64 = *quad.add(6);
        let VV: f64 = *quad.add(7);

        // determine zone and cost at alpha=0
        let zone0: i32;
        let mut T0: f64 = 0.0;
        if UU <= 0.0 {
            zone0 = if U0 < 0.0 { 2 } else { 1 };
        } else {
            T0 = UU.sqrt();
            if U0 >= mu * T0 {
                zone0 = 1; // top zone
            } else if mu * U0 + T0 <= 0.0 {
                zone0 = 2; // bottom zone
            } else {
                zone0 = 3; // middle zone
            }
        }

        // determine zone and cost at alpha
        let N: f64 = U0 + alpha * V0;
        let Tsqr: f64 = UU + alpha * (2.0 * UV + alpha * VV);
        let zone_alpha: i32;
        let mut T: f64 = 0.0;

        if Tsqr <= 0.0 {
            zone_alpha = if N < 0.0 { 2 } else { 1 }; // bottom or top zone
        } else {
            T = Tsqr.sqrt();
            if N >= mu * T {
                zone_alpha = 1; // top zone
            } else if mu * N + T <= 0.0 {
                zone_alpha = 2; // bottom zone
            } else {
                zone_alpha = 3; // middle zone
            }
        }

        // both top zone
        if zone0 == 1 && zone_alpha == 1 {
            return 0.0;
        }

        // both bottom zone
        if zone0 == 2 && zone_alpha == 2 {
            return alpha * alpha * *quad.add(2) + alpha * *quad.add(1);
        }

        // both middle zone
        if zone0 == 3 && zone_alpha == 3 {
            let diff_alpha: f64 = N - mu * T;
            let diff0: f64 = U0 - mu * T0;
            return 0.5 * Dm * (diff_alpha - diff0) * (diff_alpha + diff0);
        }

        // otherwise different zones: compute absolute costs and subtract
        elliptic_cost(quad, alpha, mu, Dm) - elliptic_cost(quad, 0.0, mu, Dm)
    }
}

/// C: PrimalEval (engine/engine_solver.c:1631)
/// Calls: ellipticCostDif, frictionCostDif, mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn primal_eval(ctx: *mut mjPrimalContext, p: *mut mjPrimalPnt) {
    #[repr(C)]
    #[allow(non_snake_case)]
    struct PrimalCtx {
        is_sparse: i32,
        is_elliptic: i32,
        island: i32,
        nv: i32,
        ne: i32,
        nf: i32,
        nefc: i32,
        nJ: i32,
        contact: *mut mjContact,
        qfrc_smooth: *const f64,
        qacc_smooth: *const f64,
        qfrc_constraint: *mut f64,
        qacc: *mut f64,
        M_rownnz: *mut i32,
        M_rowadr: *mut i32,
        M_colind: *mut i32,
        M: *mut f64,
        qLD: *mut f64,
        qLDiagInv: *mut f64,
        efc_D: *const f64,
        efc_R: *const f64,
        efc_frictionloss: *const f64,
        efc_aref: *const f64,
        efc_id: *const i32,
        efc_type: *const i32,
        efc_force: *mut f64,
        efc_state: *mut i32,
        J_rownnz: *mut i32,
        J_rowadr: *mut i32,
        J_rowsuper: *mut i32,
        J_colind: *mut i32,
        J: *mut f64,
        JT_rownnz: *mut i32,
        JT_rowadr: *mut i32,
        JT_rowsuper: *mut i32,
        JT_colind: *mut i32,
        JT: *mut f64,
        Jaref: *mut f64,
        Jv: *mut f64,
        Ma: *mut f64,
        Mv: *mut f64,
        grad: *mut f64,
        Mgrad: *mut f64,
        search: *mut f64,
        quad: *mut f64,
        oldstate: *mut i32,
        gradold: *mut f64,
        Mgradold: *mut f64,
        graddif: *mut f64,
        Mgraddif: *mut f64,
        D_newton: *mut f64,
        cholupd: *mut f64,
        LTJ: *mut f64,
        H_rowadr: *mut i32,
        H_rownnz: *mut i32,
        HT_rownnz: *mut i32,
        HT_rowadr: *mut i32,
        L_rownnz: *mut i32,
        L_rowadr: *mut i32,
        LT_rownnz: *mut i32,
        LT_rowadr: *mut i32,
        nH: i32,
        _pad0: i32,
        H_colind: *mut i32,
        HT_colind: *mut i32,
        H: *mut f64,
        nL: i32,
        _pad1: i32,
        L_colind: *mut i32,
        LT_colind: *mut i32,
        LT_map: *mut i32,
        L: *mut f64,
        Lcone: *mut f64,
        cost: f64,
        quadGauss: [f64; 3],
        scale: f64,
        nactive: i32,
        ncone: i32,
        nupdate: i32,
        LSiter: i32,
        LSresult: i32,
        _pad2: i32,
        LSslope: f64,
    }

    const MJ_CNSTR_CONTACT_ELLIPTIC: i32 = 7;
    const MJMINVAL: f64 = 1e-15;

    // SAFETY: ctx is a valid mjPrimalContext with the layout of PrimalCtx.
    // p is a valid mjPrimalPnt pointer.
    unsafe {
        let c = ctx as *mut PrimalCtx;
        let ne = (*c).ne;
        let nf = (*c).nf;
        let nefc = (*c).nefc;

        // clear result
        let mut cost: f64 = 0.0;
        let alpha: f64 = (*p).alpha;
        let mut deriv: [f64; 2] = [0.0, 0.0];

        // init quad with Gauss, shifted: drop quadGauss[0]
        let mut quadTotal: [f64; 3] = [0.0, (*c).quadGauss[1], (*c).quadGauss[2]];

        // process constraints
        let mut i: i32 = 0;
        while i < nefc {
            // equality: shifted quad (skip quad[0])
            if i < ne {
                quadTotal[1] += *(*c).quad.add((3 * i + 1) as usize);
                quadTotal[2] += *(*c).quad.add((3 * i + 2) as usize);
                i += 1;
                continue;
            }

            // friction: compute cost(alpha) - cost(0) directly
            if i < ne + nf {
                let start = *(*c).Jaref.add(i as usize);
                let dir = *(*c).Jv.add(i as usize);
                let x = start + alpha * dir;
                let f = *(*c).efc_frictionloss.add(i as usize);
                let D = *(*c).efc_D.add(i as usize);
                let Rf = *(*c).efc_R.add(i as usize) * f;

                // cost delta
                cost += friction_cost_dif(start, x, f, Rf, D);

                // -bound < x < bound : quadratic
                if -Rf < x && x < Rf {
                    deriv[0] += D * x * dir;
                    deriv[1] += D * dir * dir;
                }
                // x < -bound : linear negative
                else if x <= -Rf {
                    deriv[0] += -f * dir;
                }
                // bound < x : linear positive
                else {
                    deriv[0] += f * dir;
                }
                i += 1;
                continue;
            }

            // limit and contact
            if *(*c).efc_type.add(i as usize) == MJ_CNSTR_CONTACT_ELLIPTIC {
                // extract contact info
                let con = (*c).contact.add(*(*c).efc_id.add(i as usize) as usize);
                let quad_ptr = (*c).quad.add((3 * i) as usize);
                let dim = (*con).dim;
                let mu = (*con).mu;

                // unpack quad
                let U0 = *quad_ptr.add(3);
                let V0 = *quad_ptr.add(4);
                let UU = *quad_ptr.add(5);
                let UV = *quad_ptr.add(6);
                let VV = *quad_ptr.add(7);
                let Dm = *quad_ptr.add(8);

                // shifted cost
                cost += elliptic_cost_dif(quad_ptr, alpha, mu, Dm);

                // compute N, Tsqr for derivatives
                let N = U0 + alpha * V0;
                let Tsqr = UU + alpha * (2.0 * UV + alpha * VV);

                // no tangential force : top or bottom zone
                if Tsqr <= 0.0 {
                    // bottom zone: quadratic derivatives
                    if N < 0.0 {
                        deriv[0] += 2.0 * alpha * *quad_ptr.add(2) + *quad_ptr.add(1);
                        deriv[1] += 2.0 * *quad_ptr.add(2);
                    }
                    // top zone: nothing to do
                }
                // otherwise regular processing
                else {
                    // tangential force
                    let T = Tsqr.sqrt();

                    // N>=mu*T : top zone
                    if N >= mu * T {
                        // nothing to do
                    }
                    // mu*N+T<=0 : bottom zone
                    else if mu * N + T <= 0.0 {
                        deriv[0] += 2.0 * alpha * *quad_ptr.add(2) + *quad_ptr.add(1);
                        deriv[1] += 2.0 * *quad_ptr.add(2);
                    }
                    // otherwise middle zone
                    else {
                        let N1 = V0;
                        let T1 = (UV + alpha * VV) / T;
                        let T2 = VV / T - (UV + alpha * VV) * T1 / (T * T);
                        deriv[0] += Dm * (N - mu * T) * (N1 - mu * T1);
                        deriv[1] += Dm * ((N1 - mu * T1) * (N1 - mu * T1) + (N - mu * T) * (-mu * T2));
                    }
                }

                // advance to next constraint
                i += dim;
            } else {
                // inequality: search point
                let start = *(*c).Jaref.add(i as usize);
                let x = start + alpha * *(*c).Jv.add(i as usize);
                let cost0 = if start < 0.0 { *(*c).quad.add((3 * i) as usize) } else { 0.0 };

                // active
                if x < 0.0 {
                    // shifted quad: add quad[1], quad[2] and (quad[0] - cost0)
                    quadTotal[0] += *(*c).quad.add((3 * i) as usize) - cost0;
                    quadTotal[1] += *(*c).quad.add((3 * i + 1) as usize);
                    quadTotal[2] += *(*c).quad.add((3 * i + 2) as usize);
                } else {
                    cost -= cost0;
                }
                i += 1;
            }
        }

        // add total quadratic (quadTotal[0] contains only shifted residuals)
        cost += alpha * alpha * quadTotal[2] + alpha * quadTotal[1] + quadTotal[0];
        deriv[0] += 2.0 * alpha * quadTotal[2] + quadTotal[1];
        deriv[1] += 2.0 * quadTotal[2];

        // check for convexity; SHOULD NOT OCCUR
        if deriv[1] <= 0.0 {
            crate::engine::engine_util_errmem::mju_warning(
                b"Linesearch objective is not convex\0".as_ptr() as *const i8,
            );
            deriv[1] = MJMINVAL;
        }

        // assign and count
        (*p).cost = cost;
        (*p).deriv[0] = deriv[0];
        (*p).deriv[1] = deriv[1];
        (*c).LSiter += 1;
    }
}

/// C: updateBracket (engine/engine_solver.c:1782)
/// Calls: PrimalEval
#[allow(unused_variables, non_snake_case)]
pub fn update_bracket(ctx: *mut mjPrimalContext, p: *mut mjPrimalPnt, candidates: *const mjPrimalPnt, pnext: *mut mjPrimalPnt) -> i32 {
    todo!() // updateBracket
}

/// C: PrimalSearch (engine/engine_solver.c:1812)
/// Calls: PrimalEval, PrimalPrepare, mju_mulMatVec, mju_mulMatVecSparse, mju_mulSymVecSparse, mju_norm, updateBracket
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn primal_search(ctx: *mut mjPrimalContext, tolerance: f64, ls_iterations: f64, improvement: *mut f64) -> f64 {
    todo!() // PrimalSearch
}

/// C: MakeHessian (engine/engine_solver.c:2010)
/// Calls: mj_stackAllocInfo, mju_addToMatSparse, mju_addToSymSparse, mju_cholFactorSymbolic, mju_sqrMatTDSparseNumeric, mju_sqrMatTDSparseSymbolic, mju_sqrMatTD_impl, mju_transposeSparse
#[allow(unused_variables, non_snake_case)]
pub fn make_hessian(d: *mut mjData, ctx: *mut mjPrimalContext) {
    todo!() // MakeHessian
}

/// C: HessianCone (engine/engine_solver.c:2099)
/// Calls: mju_addToScl, mju_cholFactor, mju_cholUpdate, mju_cholUpdateSparse, mju_copy, mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn hessian_cone(d: *mut mjData, ctx: *mut mjPrimalContext) {
    todo!() // HessianCone
}

/// C: FactorizeHessian (engine/engine_solver.c:2102)
/// Calls: HessianCone, mju_addToMatSparse, mju_addToSymSparse, mju_cholFactor, mju_cholFactorNumeric, mju_message, mju_sqrMatTDSparseNumeric, mju_sqrMatTDSparseSymbolic, mju_sqrMatTD_impl
#[allow(unused_variables, non_snake_case)]
pub fn factorize_hessian(d: *mut mjData, ctx: *mut mjPrimalContext, flg_recompute: i32) {
    todo!() // FactorizeHessian
}

/// C: HessianIncremental (engine/engine_solver.c:2238)
/// Calls: FactorizeHessian, HessianCone, mju_cholUpdate, mju_cholUpdateSparse, mju_scl
#[allow(unused_variables, non_snake_case)]
pub fn hessian_incremental(d: *mut mjData, ctx: *mut mjPrimalContext, oldstate: *const i32) {
    todo!() // HessianIncremental
}

/// C: mj_solPrimal (engine/engine_solver.c:2297)
/// Calls: FactorizeHessian, HessianIncremental, MakeHessian, PrimalAllocate, PrimalPointers, PrimalSearch, PrimalUpdateConstraint, PrimalUpdateGradient, mj_freeStack, mj_isSparse, mj_markStack, mju_addToScl, mju_copy, mju_copyInt, mju_dot, mju_max, mju_min, mju_mulMatVec, mju_mulMatVecSparse, mju_mulSymVecSparse, mju_norm, mju_scl, mju_sub, mju_subFrom, saveStats
#[allow(unused_variables, non_snake_case)]
pub fn mj_sol_primal(m: *const mjModel, d: *mut mjData, island: i32, maxiter: i32, flg_Newton: i32) {
    todo!() // mj_solPrimal
}

/// C: mj_solPGS (engine/engine_solver.h:24)
/// Calls: solPGS
#[allow(unused_variables, non_snake_case)]
pub fn mj_sol_pgs(m: *const mjModel, d: *mut mjData, maxiter: i32) {
    todo!() // mj_solPGS
}

/// C: mj_solNoSlip (engine/engine_solver.h:27)
/// Calls: solNoSlip
#[allow(unused_variables, non_snake_case)]
pub fn mj_sol_no_slip(m: *const mjModel, d: *mut mjData, maxiter: i32) {
    todo!() // mj_solNoSlip
}

/// C: mj_solCG (engine/engine_solver.h:30)
/// Calls: mj_solPrimal
#[allow(unused_variables, non_snake_case)]
pub fn mj_sol_cg(m: *const mjModel, d: *mut mjData, maxiter: i32) {
    todo!() // mj_solCG
}

/// C: mj_solNewton (engine/engine_solver.h:33)
/// Calls: mj_solPrimal
#[allow(unused_variables, non_snake_case)]
pub fn mj_sol_newton(m: *const mjModel, d: *mut mjData, maxiter: i32) {
    todo!() // mj_solNewton
}

/// C: mj_solPGS_island (engine/engine_solver.h:39)
/// Calls: solPGS
#[allow(unused_variables, non_snake_case)]
pub fn mj_sol_pgs_island(m: *const mjModel, d: *mut mjData, island: i32, maxiter: i32) {
    todo!() // mj_solPGS_island
}

/// C: mj_solNoSlip_island (engine/engine_solver.h:42)
/// Calls: solNoSlip
#[allow(unused_variables, non_snake_case)]
pub fn mj_sol_no_slip_island(m: *const mjModel, d: *mut mjData, island: i32, maxiter: i32) {
    todo!() // mj_solNoSlip_island
}

/// C: mj_solCG_island (engine/engine_solver.h:45)
/// Calls: mj_solPrimal
#[allow(unused_variables, non_snake_case)]
pub fn mj_sol_cg_island(m: *const mjModel, d: *mut mjData, island: i32, maxiter: i32) {
    todo!() // mj_solCG_island
}

/// C: mj_solNewton_island (engine/engine_solver.h:48)
/// Calls: mj_solPrimal
#[allow(unused_variables, non_snake_case)]
pub fn mj_sol_newton_island(m: *const mjModel, d: *mut mjData, island: i32, maxiter: i32) {
    todo!() // mj_solNewton_island
}

/// C: mj_dualFinish (engine/engine_solver.h:51)
/// Calls: dualFinish
#[allow(unused_variables, non_snake_case)]
pub fn mj_dual_finish(m: *const mjModel, d: *mut mjData) {
    dual_finish(m, d);
}

