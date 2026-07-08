//! Port of: engine/engine_solver.c
//! IR hash: 05737965add36adb
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
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, island : i32, iter : i32, improvement : f64, gradient : f64, lineslope : f64, nactive : i32, nchange : i32, neval : i32, nupdate : i32)
    // Previous return: ()
    extern "C" { fn saveStats_impl (m : * const mjModel , d : * mut mjData , island : i32 , iter : i32 , improvement : f64 , gradient : f64 , lineslope : f64 , nactive : i32 , nchange : i32 , neval : i32 , nupdate : i32) ; } unsafe { saveStats_impl (m , d , island , iter , improvement , gradient , lineslope , nactive , nchange , neval , nupdate) }
}

/// C: dualFinish (engine/engine_solver.c:71)
/// Calls: mj_mulJacTVec, mj_solveM, mju_addTo
#[allow(unused_variables, non_snake_case)]
pub fn dual_finish(m: *const mjModel, d: *mut mjData) {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData)
    // Previous return: ()
    extern "C" { fn dualFinish_impl (m : * const mjModel , d : * mut mjData) ; } unsafe { dualFinish_impl (m , d) }
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
    const MJ_MINVAL: f64 = 1e-15;
    // SAFETY: d points to valid mjData. Field offsets verified against ast_manifest.json.
    unsafe {
        let d_ptr = d as *const u8;
        let AR: *const f64 = *(d_ptr.add(161784) as *const *const f64);
        let R: *const f64 = *(d_ptr.add(161512) as *const *const f64);

        if crate::engine::engine_core_util::mj_is_sparse(m) != 0 {
            let rowadr: *const i32 = *(d_ptr.add(161768) as *const *const i32);
            let rownnz: *const i32 = *(d_ptr.add(161760) as *const *const i32);
            let colind: *const i32 = *(d_ptr.add(161776) as *const *const i32);

            let mut c: i32 = 0;
            while c < nefc {
                let i = if !efclist.is_null() { *efclist.add(c as usize) } else { c };
                let nnz = *rownnz.add(i as usize);
                let mut j: i32 = 0;
                while j < nnz {
                    let adr = *rowadr.add(i as usize) + j;
                    if i == *colind.add(adr as usize) {
                        let ar_val = *AR.add(adr as usize);
                        let denom = if flg_subR != 0 {
                            crate::engine::engine_util_misc::mju_max(MJ_MINVAL, ar_val - *R.add(i as usize))
                        } else {
                            ar_val
                        };
                        *res.add(c as usize) = 1.0 / denom;
                        break;
                    }
                    j += 1;
                }
                c += 1;
            }
        } else {
            let d_nefc: i32 = *(d_ptr.add(160576) as *const i32);

            let mut c: i32 = 0;
            while c < nefc {
                let i = if !efclist.is_null() { *efclist.add(c as usize) } else { c };
                let adr = i * (d_nefc + 1);
                let ar_val = *AR.add(adr as usize);
                let denom = if flg_subR != 0 {
                    crate::engine::engine_util_misc::mju_max(MJ_MINVAL, ar_val - *R.add(i as usize))
                } else {
                    ar_val
                };
                *res.add(c as usize) = 1.0 / denom;
                c += 1;
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
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * const mjData, Ac : * mut f64, start : i32, n : i32, flg_subR : i32)
    // Previous return: ()
    extern "C" { fn extractBlock_impl (m : * const mjModel , d : * const mjData , Ac : * mut f64 , start : i32 , n : i32 , flg_subR : i32) ; } unsafe { extractBlock_impl (m , d , Ac , start , n , flg_subR) }
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
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * const mjData, res : * mut f64, i : i32, dim : i32, flg_subR : i32)
    // Previous return: ()
    extern "C" { fn residual_impl (m : * const mjModel , d : * const mjData , res : * mut f64 , i : i32 , dim : i32 , flg_subR : i32) ; } unsafe { residual_impl (m , d , res , i , dim , flg_subR) }
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
    // WARNING: signature changed — verify body
    // Previous params: (A : * const f64, force : * mut f64, oldforce : * const f64, res : * const f64, dim : i32)
    // Previous return: f64
    unsafe { use crate :: engine :: engine_util_blas :: { mju_sub , mju_dot , mju_copy , mju_mul_vec_mat_vec } ; let change : f64 ; if dim == 1 { let delta : f64 = * force . add (0) - * oldforce . add (0) ; change = 0.5 * delta * delta * * A . add (0) + delta * * res . add (0) ; } else { let mut delta : [f64 ; 6] = [0.0 ; 6] ; mju_sub (delta . as_mut_ptr () , force as * const f64 , oldforce , dim) ; change = 0.5 * mju_mul_vec_mat_vec (delta . as_ptr () , A , delta . as_ptr () , dim) + mju_dot (delta . as_ptr () , res , dim) ; } if change > 1e-10 { mju_copy (force , oldforce , dim) ; return 0.0 ; } change }
}

/// C: pcg32_next (engine/engine_solver.c:247)
#[allow(unused_variables, non_snake_case)]
pub fn pcg32_next(rng: *mut pcg32_state) -> u32 {
    extern "C" { fn pcg32_next_impl(rng: *mut pcg32_state) -> u32; }
    // SAFETY: delegates to C implementation
    unsafe { pcg32_next_impl(rng) }
}

/// C: shuffle_int (engine/engine_solver.c:257)
/// Calls: pcg32_next
#[allow(unused_variables, non_snake_case)]
pub fn shuffle_int(array: *mut i32, n: i32, rng: *mut pcg32_state) {
    extern "C" { fn shuffle_int_impl(array: *mut i32, n: i32, rng: *mut pcg32_state); }
    // SAFETY: delegates to C implementation
    unsafe { shuffle_int_impl(array, n, rng) }
}

/// C: dualState (engine/engine_solver.c:269)
/// Calls: mju_fillInt, mju_norm
#[allow(unused_variables, non_snake_case)]
pub fn dual_state(d: *const mjData, state: *mut i32, ne: i32, nf: i32, nefc: i32, efclist: *const i32) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (d : * const mjData, state : * mut i32, ne : i32, nf : i32, nefc : i32, efclist : * const i32)
    // Previous return: i32
    extern "C" { fn dualState_impl (d : * const mjData , state : * mut i32 , ne : i32 , nf : i32 , nefc : i32 , efclist : * const i32) -> i32 ; } unsafe { dualState_impl (d , state , ne , nf , nefc , efclist) }
}

/// C: dualStateChange (engine/engine_solver.c:356)
/// Calls: dualState
#[allow(unused_variables, non_snake_case)]
pub fn dual_state_change(d: *const mjData, state: *mut i32, oldstate: *mut i32, ne: i32, nf: i32, nefc: i32, efclist: *const i32, nchange: *mut i32) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (d : * const mjData, state : * mut i32, oldstate : * mut i32, ne : i32, nf : i32, nefc : i32, efclist : * const i32, nchange : * mut i32)
    // Previous return: i32
    extern "C" { fn dualStateChange_impl (d : * const mjData , state : * mut i32 , oldstate : * mut i32 , ne : i32 , nf : i32 , nefc : i32 , efclist : * const i32 , nchange : * mut i32) -> i32 ; } unsafe { dualStateChange_impl (d , state , oldstate , ne , nf , nefc , efclist , nchange) }
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
    const MJ_MINVAL: f64 = 1e-15;

    // SAFETY: friction[0..dim-1] and mu[0..dim-1] are valid arrays per caller contract.
    unsafe {
        let mut s: f64 = 0.0;
        for j in 0..(dim - 1) as usize {
            s += *friction.add(j) * *friction.add(j) / (*mu.add(j) * *mu.add(j));
        }

        let normal2: f64 = normal * normal;

        if feasible == 0 || s > normal2 {
            let scl: f64 = (normal2 / crate::engine::engine_util_misc::mju_max(MJ_MINVAL, s)).sqrt();
            for j in 0..(dim - 1) as usize {
                *friction.add(j) *= scl;
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
    // SAFETY: force points to at least i+dim f64; Ac, bc, mu valid per caller contract
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
            crate::engine::engine_solver::project_ellipsoid(
                v.as_mut_ptr(), *force.add(i as usize), mu, dim, 0);
        }

        // assign
        crate::engine::engine_util_blas::mju_copy(
            force.add(i as usize + 1), v.as_ptr(), dim - 1);
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
    const MJ_CNSTR_CONTACT_ELLIPTIC: i32 = 7;
    // SAFETY: force points to at least dim f64; mu valid per caller contract
    unsafe {
        // elliptic cone: project onto friction ellipsoid
        if r#type == MJ_CNSTR_CONTACT_ELLIPTIC {
            // clamp normal force
            if *force.add(0) < 0.0 {
                crate::engine::engine_util_blas::mju_zero(force, dim);
            } else {
                crate::engine::engine_solver::project_ellipsoid(
                    force.add(1), *force.add(0), mu, dim, 1);
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
    extern "C" { fn solPGS_impl(m: *const mjModel, d: *mut mjData, island: i32, ne: i32, nf: i32, nefc: i32, efclist: *const i32, maxiter: i32); }
    // SAFETY: delegates to C implementation
    unsafe { solPGS_impl(m, d, island, ne, nf, nefc, efclist, maxiter) }
}

/// C: solNoSlip (engine/engine_solver.c:766)
/// Calls: ARdiaginv, costChange, dualState, dualStateChange, extractBlock, mj_freeStack, mj_markStack, mj_stackAllocInfo, mju_copy, mju_dot, mju_zero, residual, saveStats, solveQCQP
#[allow(unused_variables, non_snake_case)]
pub fn sol_no_slip(m: *const mjModel, d: *mut mjData, island: i32, ne: i32, nf: i32, nefc: i32, efclist: *const i32, maxiter: i32) {
    const MJ_MINVAL: f64 = 1e-15;
    const MJ_NISLAND: i32 = 20;

    // SAFETY: m, d are valid pointers. All byte offsets verified against compiled mujoco headers.
    unsafe {
        let d_ptr = d as *mut u8;
        let m_ptr = m as *const u8;

        // mjData field offsets (64-bit)
        let floss: *const f64 = *(d_ptr.add(161480) as *const *const f64);   // efc_frictionloss
        let force: *mut f64 = *(d_ptr.add(161848) as *const *mut f64);       // efc_force
        let efc_R: *const f64 = *(d_ptr.add(161512) as *const *const f64);   // efc_R
        let efc_type: *const i32 = *(d_ptr.add(161408) as *const *const i32); // efc_type
        let efc_id: *const i32 = *(d_ptr.add(161416) as *const *const i32);   // efc_id
        let contact: *const mjContact = *(d_ptr.add(161400) as *const *const mjContact); // contact
        let efc_state: *mut i32 = *(d_ptr.add(161840) as *const *mut i32);   // efc_state
        let solver_niter: *mut i32 = d_ptr.add(160088) as *mut i32;          // solver_niter[20]

        // mjModel field offsets
        let noslip_tolerance: f64 = *(m_ptr.add(768) as *const f64);         // opt.noslip_tolerance
        let meaninertia: f64 = *(m_ptr.add(1672) as *const f64);            // stat.meaninertia
        let m_nv: i32 = *(m_ptr.add(8) as *const i32);                      // nv

        let mut iter: i32 = 0;
        let mut improvement: f64;
        let mut Ac: [f64; 25] = [0.0; 25];
        let mut bc: [f64; 5] = [0.0; 5];
        let mut res: [f64; 5] = [0.0; 5];
        let mut oldforce: [f64; 5] = [0.0; 5];
        let mut delta: [f64; 5] = [0.0; 5];
        let mut mid: f64;
        let mut y: f64;
        let mut K0: f64;
        let mut K1: f64;

        crate::engine::engine_memory::mj_mark_stack(d);
        let ARinv: *mut f64 = crate::engine::engine_memory::mj_stack_alloc_num(d, nefc as usize);
        let oldstate: *mut i32 = crate::engine::engine_memory::mj_stack_alloc_int(d, nefc as usize);

        let island_stat: i32 = if island > 0 { island } else { 0 };
        let scale: f64 = 1.0 / (meaninertia * (if 1 > m_nv { 1 } else { m_nv }) as f64);

        // precompute inverse diagonal of A
        a_rdiaginv(m, d as *const mjData, ARinv, nefc, efclist, 1);

        // initial constraint state
        dual_state(d as *const mjData, efc_state, ne, nf, nefc, efclist);

        // main iteration
        while iter < maxiter {
            // clear improvement
            improvement = 0.0;

            // correct for cost change at iter 0
            if iter == 0 {
                let mut c: i32 = 0;
                while c < nefc {
                    let i = if !efclist.is_null() { *efclist.add(c as usize) } else { c };
                    improvement += 0.5 * *force.add(i as usize) * *force.add(i as usize) * *efc_R.add(i as usize);
                    c += 1;
                }
            }

            // perform one sweep: dry friction
            {
                let mut c: i32 = ne;
                while c < ne + nf {
                    let i = if !efclist.is_null() { *efclist.add(c as usize) } else { c };

                    // compute residual, save old
                    residual(m, d as *const mjData, res.as_mut_ptr(), i, 1, 1);
                    oldforce[0] = *force.add(i as usize);

                    // unconstrained minimum
                    *force.add(i as usize) -= res[0] * *ARinv.add(c as usize);

                    // impose interval constraints
                    if *force.add(i as usize) < -*floss.add(i as usize) {
                        *force.add(i as usize) = -*floss.add(i as usize);
                    } else if *force.add(i as usize) > *floss.add(i as usize) {
                        *force.add(i as usize) = *floss.add(i as usize);
                    }

                    // add to improvement
                    delta[0] = *force.add(i as usize) - oldforce[0];
                    improvement -= 0.5 * delta[0] * delta[0] / *ARinv.add(c as usize) + delta[0] * res[0];

                    c += 1;
                }
            }

            // perform one sweep: contact friction
            {
                let mut c: i32 = ne + nf;
                while c < nefc {
                    let i = if !efclist.is_null() { *efclist.add(c as usize) } else { c };

                    // pyramidal contact
                    if *efc_type.add(i as usize) == 6 { // mjCNSTR_CONTACT_PYRAMIDAL
                        // get contact info
                        let con: *const mjContact = contact.add(*efc_id.add(i as usize) as usize);
                        let dim: i32 = (*con).dim;
                        let mu: *const f64 = (*con).friction.as_ptr();

                        // loop over pairs of opposing pyramid edges
                        let mut j: i32 = i;
                        while j < i + 2 * (dim - 1) {
                            // compute residual, save old
                            residual(m, d as *const mjData, res.as_mut_ptr(), j, 2, 1);
                            crate::engine::engine_util_blas::mju_copy(oldforce.as_mut_ptr(), force.add(j as usize) as *const f64, 2);

                            // Ac = AR-submatrix
                            extract_block(m, d as *const mjData, Ac.as_mut_ptr(), j, 2, 1);

                            // bc = b-subvector + Ac,rest * f_rest
                            crate::engine::engine_util_blas::mju_copy(bc.as_mut_ptr(), res.as_ptr(), 2);
                            let mut k: i32 = 0;
                            while k < 2 {
                                bc[k as usize] -= crate::engine::engine_util_blas::mju_dot(Ac.as_ptr().add((k * 2) as usize), oldforce.as_ptr(), 2);
                                k += 1;
                            }

                            // f0 = mid+y, f1 = mid-y
                            mid = 0.5 * (*force.add(j as usize) + *force.add((j + 1) as usize));
                            y = 0.5 * (*force.add(j as usize) - *force.add((j + 1) as usize));

                            // K1 = A00 + A11 - 2*A01,  K0 = mid*A00 - mid*A11 + b0 - b1
                            K1 = Ac[0] + Ac[3] - Ac[1] - Ac[2];
                            K0 = mid * (Ac[0] - Ac[3]) + bc[0] - bc[1];

                            // guard against Ac==0
                            if K1 < MJ_MINVAL {
                                *force.add(j as usize) = mid;
                                *force.add((j + 1) as usize) = mid;
                            }
                            // otherwise minimize over y \in [-mid, mid]
                            else {
                                // unconstrained minimum
                                y = -K0 / K1;

                                // clamp and assign
                                if y < -mid {
                                    *force.add(j as usize) = 0.0;
                                    *force.add((j + 1) as usize) = 2.0 * mid;
                                } else if y > mid {
                                    *force.add(j as usize) = 2.0 * mid;
                                    *force.add((j + 1) as usize) = 0.0;
                                } else {
                                    *force.add(j as usize) = mid + y;
                                    *force.add((j + 1) as usize) = mid - y;
                                }
                            }

                            // accumulate improvement
                            improvement -= cost_change(Ac.as_ptr(), force.add(j as usize), oldforce.as_ptr(), res.as_ptr(), 2);

                            j += 2;
                        }

                        // skip the rest of this contact
                        c += 2 * (dim - 1) - 1;
                    }
                    // elliptic contact
                    else if *efc_type.add(i as usize) == 7 { // mjCNSTR_CONTACT_ELLIPTIC
                        // get contact info
                        let con: *const mjContact = contact.add(*efc_id.add(i as usize) as usize);
                        let dim: i32 = (*con).dim;
                        let mu: *const f64 = (*con).friction.as_ptr();

                        // compute residual, save old
                        residual(m, d as *const mjData, res.as_mut_ptr(), i + 1, dim - 1, 1);
                        crate::engine::engine_util_blas::mju_copy(oldforce.as_mut_ptr(), force.add((i + 1) as usize) as *const f64, dim - 1);

                        // Ac = AR-submatrix
                        extract_block(m, d as *const mjData, Ac.as_mut_ptr(), i + 1, dim - 1, 1);

                        // bc = b-subvector + Ac,rest * f_rest
                        crate::engine::engine_util_blas::mju_copy(bc.as_mut_ptr(), res.as_ptr(), dim - 1);
                        let mut j: i32 = 0;
                        while j < dim - 1 {
                            bc[j as usize] -= crate::engine::engine_util_blas::mju_dot(Ac.as_ptr().add((j * (dim - 1)) as usize), oldforce.as_ptr(), dim - 1);
                            j += 1;
                        }

                        // guard for f_normal==0
                        if *force.add(i as usize) < MJ_MINVAL {
                            crate::engine::engine_util_blas::mju_zero(force.add((i + 1) as usize), dim - 1);
                        }
                        // QCQP
                        else {
                            solve_qcqp(force, i, dim, Ac.as_mut_ptr(), bc.as_mut_ptr(), mu);
                        }

                        // accumulate improvement
                        improvement -= cost_change(Ac.as_ptr(), force.add((i + 1) as usize), oldforce.as_ptr(), res.as_ptr(), dim - 1);

                        // skip the rest of this contact
                        c += dim - 1;
                    }

                    c += 1;
                }
            }

            // update constraint state
            let mut nchange: i32 = 0;
            let nactive: i32 = dual_state_change(d as *const mjData, efc_state, oldstate, ne, nf, nefc, efclist, &mut nchange as *mut i32);

            // scale improvement
            improvement *= scale;

            // save noslip stats after all the entries from regular solver
            if island_stat < MJ_NISLAND {
                let stats_iter: i32 = iter + *solver_niter.add(island_stat as usize);
                save_stats(m, d, island_stat, stats_iter, improvement, 0.0, 0.0, nactive, nchange, 0, 0);
            }

            // increment iteration count
            iter += 1;

            // terminate
            if improvement < noslip_tolerance {
                break;
            }
        }

        // update solver iterations
        if island_stat < MJ_NISLAND {
            *solver_niter.add(island_stat as usize) += iter;
        }

        crate::engine::engine_memory::mj_free_stack(d);
    }
}

/// C: PrimalPointers (engine/engine_solver.c:1087)
/// Calls: mj_isSparse
#[allow(unused_variables, non_snake_case)]
pub fn primal_pointers(m: *const mjModel, d: *const mjData, ctx: *mut mjPrimalContext, island: i32) {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * const mjData, ctx : * mut mjPrimalContext, island : i32)
    // Previous return: ()
    extern "C" { fn PrimalPointers_impl (m : * const mjModel , d : * const mjData , ctx : * mut mjPrimalContext , island : i32) ; } unsafe { PrimalPointers_impl (m , d , ctx , island) }
}

/// C: PrimalAllocate (engine/engine_solver.c:1171)
/// Calls: mj_stackAllocInfo, mju_block, mju_blockSparse, mju_gather, mju_superSparse, mju_transposeSparse
#[allow(unused_variables, non_snake_case)]
pub fn primal_allocate(m: *const mjModel, d: *mut mjData, ctx: *mut mjPrimalContext, flg_Newton: i32) {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, ctx : * mut mjPrimalContext, flg_Newton : i32)
    // Previous return: ()
    extern "C" { fn PrimalAllocate_impl (m : * const mjModel , d : * mut mjData , ctx : * mut mjPrimalContext , flg_Newton : i32) ; } unsafe { PrimalAllocate_impl (m , d , ctx , flg_Newton) }
}

/// C: PrimalUpdateConstraint (engine/engine_solver.c:1343)
/// Calls: mj_constraintUpdate_impl, mju_mulMatTVec, mju_mulMatVecSparse
#[allow(unused_variables, non_snake_case)]
pub fn primal_update_constraint(ctx: *mut mjPrimalContext, flg_HessianCone: i32) {
    // WARNING: signature changed — verify body
    // Previous params: (ctx : * mut mjPrimalContext, flg_HessianCone : i32)
    // Previous return: ()
    extern "C" { fn PrimalUpdateConstraint_impl (ctx : * mut mjPrimalContext , flg_HessianCone : i32) ; } unsafe { PrimalUpdateConstraint_impl (ctx , flg_HessianCone) }
}

/// C: PrimalUpdateGradient (engine/engine_solver.c:1380)
/// Calls: mj_solveLD, mju_cholSolve, mju_cholSolveSparse, mju_copy
#[allow(unused_variables, non_snake_case)]
pub fn primal_update_gradient(ctx: *mut mjPrimalContext, flg_Newton: i32) {
    // WARNING: signature changed — verify body
    // Previous params: (ctx : * mut mjPrimalContext, flg_Newton : i32)
    // Previous return: ()
    extern "C" { fn PrimalUpdateGradient_impl (ctx : * mut mjPrimalContext , flg_Newton : i32) ; } unsafe { PrimalUpdateGradient_impl (ctx , flg_Newton) }
}

/// C: PrimalPrepare (engine/engine_solver.c:1408)
/// Calls: mju_dot
#[allow(unused_variables, non_snake_case)]
pub fn primal_prepare(ctx: *mut mjPrimalContext) {
    // WARNING: signature changed — verify body
    // Previous params: (ctx : * mut mjPrimalContext)
    // Previous return: ()
    extern "C" { fn PrimalPrepare_impl (ctx : * mut mjPrimalContext) ; } unsafe { PrimalPrepare_impl (ctx) }
}

/// C: frictionCost (engine/engine_solver.c:1493)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn friction_cost(x: f64, f: f64, Rf: f64, D: f64) -> f64 {
    // WARNING: signature changed — verify body
    // Previous params: (x : f64, f : f64, Rf : f64, D : f64)
    // Previous return: f64
    if - Rf < x && x < Rf { 0.5 * D * x * x } else if x <= - Rf { f * (- 0.5 * Rf - x) } else { f * (- 0.5 * Rf + x) }
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
    // WARNING: signature changed — verify body
    // Previous params: (start : f64, x : f64, f : f64, Rf : f64, D : f64)
    // Previous return: f64
    let state_start : i32 = if - Rf < start && start < Rf { 0 } else if start <= - Rf { - 1 } else { 1 } ; let state_x : i32 = if - Rf < x && x < Rf { 0 } else if x <= - Rf { - 1 } else { 1 } ; if state_start == 0 && state_x == 0 { return 0.5 * D * (x - start) * (x + start) ; } if state_start == - 1 && state_x == - 1 { return f * (start - x) ; } if state_start == 1 && state_x == 1 { return f * (x - start) ; } friction_cost (x , f , Rf , D) - friction_cost (start , f , Rf , D)
}

/// C: ellipticCost (engine/engine_solver.c:1531)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn elliptic_cost(quad: *const f64, alpha: f64, mu: f64, Dm: f64) -> f64 {
    // SAFETY: quad[0..7] is a valid array per caller contract.
    unsafe {
        let U0: f64 = *quad.add(3);
        let V0: f64 = *quad.add(4);
        let UU: f64 = *quad.add(5);
        let UV: f64 = *quad.add(6);
        let VV: f64 = *quad.add(7);

        let N: f64 = U0 + alpha * V0;
        let Tsqr: f64 = UU + alpha * (2.0 * UV + alpha * VV);

        if Tsqr <= 0.0 {
            if N < 0.0 {
                return alpha * alpha * *quad.add(2) + alpha * *quad.add(1) + *quad.add(0);
            }
        } else {
            let T: f64 = Tsqr.sqrt();
            if N >= mu * T {
                // nothing
            } else if mu * N + T <= 0.0 {
                return alpha * alpha * *quad.add(2) + alpha * *quad.add(1) + *quad.add(0);
            } else {
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
    // SAFETY: quad[0..7] is a valid array per caller contract.
    unsafe {
        let U0: f64 = *quad.add(3);
        let V0: f64 = *quad.add(4);
        let UU: f64 = *quad.add(5);
        let UV: f64 = *quad.add(6);
        let VV: f64 = *quad.add(7);

        // Determine zone0
        let zone0: i32;
        let mut T0: f64 = 0.0;
        if UU <= 0.0 {
            zone0 = if U0 < 0.0 { 2 } else { 1 };
        } else {
            T0 = UU.sqrt();
            if U0 >= mu * T0 {
                zone0 = 1;
            } else if mu * U0 + T0 <= 0.0 {
                zone0 = 2;
            } else {
                zone0 = 3;
            }
        }

        // Determine zone_alpha
        let N: f64 = U0 + alpha * V0;
        let Tsqr: f64 = UU + alpha * (2.0 * UV + alpha * VV);
        let zone_alpha: i32;
        let mut T: f64 = 0.0;
        if Tsqr <= 0.0 {
            zone_alpha = if N < 0.0 { 2 } else { 1 };
        } else {
            T = Tsqr.sqrt();
            if N >= mu * T {
                zone_alpha = 1;
            } else if mu * N + T <= 0.0 {
                zone_alpha = 2;
            } else {
                zone_alpha = 3;
            }
        }

        // Compute result based on zone combinations
        if zone0 == 1 && zone_alpha == 1 {
            return 0.0;
        }
        if zone0 == 2 && zone_alpha == 2 {
            return alpha * alpha * *quad.add(2) + alpha * *quad.add(1);
        }
        if zone0 == 3 && zone_alpha == 3 {
            let diff_alpha: f64 = N - mu * T;
            let diff0: f64 = U0 - mu * T0;
            return 0.5 * Dm * (diff_alpha - diff0) * (diff_alpha + diff0);
        }

        elliptic_cost(quad, alpha, mu, Dm) - elliptic_cost(quad, 0.0, mu, Dm)
    }
}

/// C: PrimalEval (engine/engine_solver.c:1631)
/// Calls: ellipticCostDif, frictionCostDif, mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn primal_eval(ctx: *mut mjPrimalContext, p: *mut mjPrimalPnt) {
    const MJ_MINVAL: f64 = 1e-15;

    // SAFETY: ctx points to valid mjPrimalContext, p points to valid mjPrimalPnt.
    // Byte offsets for mjPrimalContext fields verified against compiled struct layout.
    unsafe {
        let ctx_ptr = ctx as *const u8;

        // mjPrimalContext field offsets (64-bit macOS)
        let ne: i32 = *(ctx_ptr.add(16) as *const i32);
        let nf: i32 = *(ctx_ptr.add(20) as *const i32);
        let nefc: i32 = *(ctx_ptr.add(24) as *const i32);
        let contact: *const mjContact = *(ctx_ptr.add(32) as *const *const mjContact);
        let efc_D: *const f64 = *(ctx_ptr.add(120) as *const *const f64);
        let efc_R: *const f64 = *(ctx_ptr.add(128) as *const *const f64);
        let efc_frictionloss: *const f64 = *(ctx_ptr.add(136) as *const *const f64);
        let efc_id: *const i32 = *(ctx_ptr.add(152) as *const *const i32);
        let efc_type: *const i32 = *(ctx_ptr.add(160) as *const *const i32);
        let Jaref: *const f64 = *(ctx_ptr.add(264) as *const *const f64);
        let Jv: *const f64 = *(ctx_ptr.add(272) as *const *const f64);
        let quad: *const f64 = *(ctx_ptr.add(320) as *const *const f64);
        let quadGauss: *const f64 = ctx_ptr.add(544) as *const f64; // inline array [3]

        // clear result
        let mut cost: f64 = 0.0;
        let alpha: f64 = (*p).alpha;
        let mut deriv: [f64; 2] = [0.0, 0.0];

        // init quad with Gauss, shifted: drop quadGauss[0]
        let mut quadTotal: [f64; 3] = [0.0, *quadGauss.add(1), *quadGauss.add(2)];

        // process constraints
        let mut i: i32 = 0;
        while i < nefc {
            // equality: shifted quad (skip quad[0])
            if i < ne {
                quadTotal[1] += *quad.add((3 * i + 1) as usize);
                quadTotal[2] += *quad.add((3 * i + 2) as usize);
                i += 1;
                continue;
            }

            // friction: compute cost(alpha) - cost(0) directly
            if i < ne + nf {
                // search point, friction loss, bound (Rf)
                let start: f64 = *Jaref.add(i as usize);
                let dir: f64 = *Jv.add(i as usize);
                let x: f64 = start + alpha * dir;
                let f: f64 = *efc_frictionloss.add(i as usize);
                let D: f64 = *efc_D.add(i as usize);
                let Rf: f64 = *efc_R.add(i as usize) * f;

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
            if *efc_type.add(i as usize) == 7 { // mjCNSTR_CONTACT_ELLIPTIC
                // extract contact info
                let con: *const mjContact = contact.add(*efc_id.add(i as usize) as usize);
                let quad_i: *const f64 = quad.add((3 * i) as usize);
                let dim: i32 = (*con).dim;
                let mu: f64 = (*con).mu;

                // unpack quad
                let U0: f64 = *quad_i.add(3);
                let V0: f64 = *quad_i.add(4);
                let UU: f64 = *quad_i.add(5);
                let UV: f64 = *quad_i.add(6);
                let VV: f64 = *quad_i.add(7);
                let Dm: f64 = *quad_i.add(8);

                // shifted cost
                cost += elliptic_cost_dif(quad_i, alpha, mu, Dm);

                // compute N, Tsqr for derivatives
                let N: f64 = U0 + alpha * V0;
                let Tsqr: f64 = UU + alpha * (2.0 * UV + alpha * VV);

                // no tangential force : top or bottom zone
                if Tsqr <= 0.0 {
                    // bottom zone: quadratic derivatives
                    if N < 0.0 {
                        deriv[0] += 2.0 * alpha * *quad_i.add(2) + *quad_i.add(1);
                        deriv[1] += 2.0 * *quad_i.add(2);
                    }
                    // top zone: nothing to do
                }
                // otherwise regular processing
                else {
                    // tangential force
                    let T: f64 = Tsqr.sqrt();

                    // N>=mu*T : top zone
                    if N >= mu * T {
                        // nothing to do
                    }
                    // mu*N+T<=0 : bottom zone
                    else if mu * N + T <= 0.0 {
                        deriv[0] += 2.0 * alpha * *quad_i.add(2) + *quad_i.add(1);
                        deriv[1] += 2.0 * *quad_i.add(2);
                    }
                    // otherwise middle zone
                    else {
                        // derivatives
                        let N1: f64 = V0;
                        let T1: f64 = (UV + alpha * VV) / T;
                        let T2: f64 = VV / T - (UV + alpha * VV) * T1 / (T * T);
                        deriv[0] += Dm * (N - mu * T) * (N1 - mu * T1);
                        deriv[1] += Dm * ((N1 - mu * T1) * (N1 - mu * T1) + (N - mu * T) * (-mu * T2));
                    }
                }

                // advance to next constraint
                i += dim - 1;
            } else { // inequality
                // search point
                let start: f64 = *Jaref.add(i as usize);
                let x: f64 = start + alpha * *Jv.add(i as usize);
                let cost0: f64 = if start < 0.0 { *quad.add((3 * i) as usize) } else { 0.0 };

                // active
                if x < 0.0 {
                    // shifted quad: add quad[1], quad[2] and (quad[0] - cost0)
                    quadTotal[0] += *quad.add((3 * i) as usize) - cost0;
                    quadTotal[1] += *quad.add((3 * i + 1) as usize);
                    quadTotal[2] += *quad.add((3 * i + 2) as usize);
                } else {
                    cost -= cost0;
                }
            }

            i += 1;
        }

        // add total quadratic (quadTotal[0] contains only shifted residuals)
        cost += alpha * alpha * quadTotal[2] + alpha * quadTotal[1] + quadTotal[0];
        deriv[0] += 2.0 * alpha * quadTotal[2] + quadTotal[1];
        deriv[1] += 2.0 * quadTotal[2];

        // check for convexity; SHOULD NOT OCCUR
        if deriv[1] <= 0.0 {
            crate::engine::engine_util_errmem::mju_warning(b"Linesearch objective is not convex\0".as_ptr() as *const i8);
            deriv[1] = MJ_MINVAL;
        }

        // assign and count
        (*p).cost = cost;
        (*p).deriv[0] = deriv[0];
        (*p).deriv[1] = deriv[1];

        // ctx->LSiter++
        let ls_iter_ptr = (ctx as *mut u8).add(588) as *mut i32;
        *ls_iter_ptr += 1;
    }
}

/// C: updateBracket (engine/engine_solver.c:1782)
/// Calls: PrimalEval
#[allow(unused_variables, non_snake_case)]
pub fn update_bracket(ctx: *mut mjPrimalContext, p: *mut mjPrimalPnt, candidates: *const mjPrimalPnt, pnext: *mut mjPrimalPnt) -> i32 {
    // SAFETY: ctx, p, candidates, pnext are valid pointers. candidates points to array of 3.
    unsafe {
        let mut flag: i32 = 0;
        for i in 0..3 {
            // negative deriv
            if (*p).deriv[0] < 0.0
                && (*candidates.add(i)).deriv[0] < 0.0
                && (*p).deriv[0] < (*candidates.add(i)).deriv[0]
            {
                *p = *candidates.add(i);
                flag = 1;
            }
            // positive deriv
            else if (*p).deriv[0] > 0.0
                && (*candidates.add(i)).deriv[0] > 0.0
                && (*p).deriv[0] > (*candidates.add(i)).deriv[0]
            {
                *p = *candidates.add(i);
                flag = 2;
            }
        }

        // compute next point if updated
        if flag != 0 {
            (*pnext).alpha = (*p).alpha - (*p).deriv[0] / (*p).deriv[1];
            primal_eval(ctx, pnext);
        }

        flag
    }
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
    extern "C" {
        fn PrimalSearch_impl(ctx: *mut mjPrimalContext, tolerance: f64, ls_iterations: f64, improvement: *mut f64) -> f64;
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { PrimalSearch_impl(ctx, tolerance, ls_iterations, improvement) }
}

/// C: MakeHessian (engine/engine_solver.c:2010)
/// Calls: mj_stackAllocInfo, mju_addToMatSparse, mju_addToSymSparse, mju_cholFactorSymbolic, mju_sqrMatTDSparseNumeric, mju_sqrMatTDSparseSymbolic, mju_sqrMatTD_impl, mju_transposeSparse
#[allow(unused_variables, non_snake_case)]
pub fn make_hessian(d: *mut mjData, ctx: *mut mjPrimalContext) {
    // WARNING: signature changed — verify body
    // Previous params: (d : * mut mjData, ctx : * mut mjPrimalContext)
    // Previous return: ()
    extern "C" { fn MakeHessian_impl (d : * mut mjData , ctx : * mut mjPrimalContext) ; } unsafe { MakeHessian_impl (d , ctx) }
}

/// C: HessianCone (engine/engine_solver.c:2099)
/// Calls: mju_addToScl, mju_cholFactor, mju_cholUpdate, mju_cholUpdateSparse, mju_copy, mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn hessian_cone(d: *mut mjData, ctx: *mut mjPrimalContext) {
    // WARNING: signature changed — verify body
    // Previous params: (d : * mut mjData, ctx : * mut mjPrimalContext)
    // Previous return: ()
    extern "C" { fn HessianCone_impl (d : * mut mjData , ctx : * mut mjPrimalContext) ; } unsafe { HessianCone_impl (d , ctx) }
}

/// C: FactorizeHessian (engine/engine_solver.c:2102)
/// Calls: HessianCone, mju_addToMatSparse, mju_addToSymSparse, mju_cholFactor, mju_cholFactorNumeric, mju_message, mju_sqrMatTDSparseNumeric, mju_sqrMatTDSparseSymbolic, mju_sqrMatTD_impl
#[allow(unused_variables, non_snake_case)]
pub fn factorize_hessian(d: *mut mjData, ctx: *mut mjPrimalContext, flg_recompute: i32) {
    // WARNING: signature changed — verify body
    // Previous params: (d : * mut mjData, ctx : * mut mjPrimalContext, flg_recompute : i32)
    // Previous return: ()
    extern "C" { fn FactorizeHessian_impl (d : * mut mjData , ctx : * mut mjPrimalContext , flg_recompute : i32) ; } unsafe { FactorizeHessian_impl (d , ctx , flg_recompute) }
}

/// C: HessianIncremental (engine/engine_solver.c:2238)
/// Calls: FactorizeHessian, HessianCone, mju_cholUpdate, mju_cholUpdateSparse, mju_scl
#[allow(unused_variables, non_snake_case)]
pub fn hessian_incremental(d: *mut mjData, ctx: *mut mjPrimalContext, oldstate: *const i32) {
    // WARNING: signature changed — verify body
    // Previous params: (d : * mut mjData, ctx : * mut mjPrimalContext, oldstate : * const i32)
    // Previous return: ()
    extern "C" { fn HessianIncremental_impl (d : * mut mjData , ctx : * mut mjPrimalContext , oldstate : * const i32) ; } unsafe { HessianIncremental_impl (d , ctx , oldstate) }
}

/// C: mj_solPrimal (engine/engine_solver.c:2297)
/// Calls: FactorizeHessian, HessianIncremental, MakeHessian, PrimalAllocate, PrimalPointers, PrimalSearch, PrimalUpdateConstraint, PrimalUpdateGradient, mj_freeStack, mj_isSparse, mj_markStack, mju_addToScl, mju_copy, mju_copyInt, mju_dot, mju_max, mju_min, mju_mulMatVec, mju_mulMatVecSparse, mju_mulSymVecSparse, mju_norm, mju_scl, mju_sub, mju_subFrom, saveStats
#[allow(unused_variables, non_snake_case)]
pub fn mj_sol_primal(m: *const mjModel, d: *mut mjData, island: i32, maxiter: i32, flg_Newton: i32) {
    extern "C" {
        fn mj_solPrimal_impl(m: *const mjModel, d: *mut mjData, island: i32, maxiter: i32, flg_Newton: i32);
    }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_solPrimal_impl(m, d, island, maxiter, flg_Newton) }
}

/// C: mj_solPGS (engine/engine_solver.h:24)
/// Calls: solPGS
#[allow(unused_variables, non_snake_case)]
pub fn mj_sol_pgs(m: *const mjModel, d: *mut mjData, maxiter: i32) {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, maxiter : i32)
    // Previous return: ()
    extern "C" { fn mj_solPGS_impl (m : * const mjModel , d : * mut mjData , maxiter : i32) ; } unsafe { mj_solPGS_impl (m , d , maxiter) }
}

/// C: mj_solNoSlip (engine/engine_solver.h:27)
/// Calls: solNoSlip
#[allow(unused_variables, non_snake_case)]
pub fn mj_sol_no_slip(m: *const mjModel, d: *mut mjData, maxiter: i32) {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, maxiter : i32)
    // Previous return: ()
    extern "C" { fn mj_solNoSlip_impl (m : * const mjModel , d : * mut mjData , maxiter : i32) ; } unsafe { mj_solNoSlip_impl (m , d , maxiter) }
}

/// C: mj_solCG (engine/engine_solver.h:30)
/// Calls: mj_solPrimal
#[allow(unused_variables, non_snake_case)]
pub fn mj_sol_cg(m: *const mjModel, d: *mut mjData, maxiter: i32) {
    // SAFETY: delegates to mj_sol_primal in same module
    mj_sol_primal(m, d, -1, maxiter, 0);
}

/// C: mj_solNewton (engine/engine_solver.h:33)
/// Calls: mj_solPrimal
#[allow(unused_variables, non_snake_case)]
pub fn mj_sol_newton(m: *const mjModel, d: *mut mjData, maxiter: i32) {
    // SAFETY: delegates to mj_sol_primal in same module
    mj_sol_primal(m, d, -1, maxiter, 1);
}

/// C: mj_solPGS_island (engine/engine_solver.h:39)
/// Calls: solPGS
#[allow(unused_variables, non_snake_case)]
pub fn mj_sol_pgs_island(m: *const mjModel, d: *mut mjData, island: i32, maxiter: i32) {
    extern "C" { fn mj_solPGS_island_impl(m: *const mjModel, d: *mut mjData, island: i32, maxiter: i32); }
    // SAFETY: delegates to C implementation
    unsafe { mj_solPGS_island_impl(m, d, island, maxiter) }
}

/// C: mj_solNoSlip_island (engine/engine_solver.h:42)
/// Calls: solNoSlip
#[allow(unused_variables, non_snake_case)]
pub fn mj_sol_no_slip_island(m: *const mjModel, d: *mut mjData, island: i32, maxiter: i32) {
    // SAFETY: m, d are valid pointers; island is a valid island index.
    unsafe {
        let ne = *(*d).island_ne.add(island as usize);
        let nf = *(*d).island_nf.add(island as usize);
        let nefc = *(*d).island_nefc.add(island as usize);
        let iefcadr = *(*d).island_iefcadr.add(island as usize);

        sol_no_slip(m, d, island, ne, nf, nefc, (*d).map_iefc2efc.add(iefcadr as usize), maxiter);
    }
}

/// C: mj_solCG_island (engine/engine_solver.h:45)
/// Calls: mj_solPrimal
#[allow(unused_variables, non_snake_case)]
pub fn mj_sol_cg_island(m: *const mjModel, d: *mut mjData, island: i32, maxiter: i32) {
    extern "C" { fn mj_solCG_island_impl(m: *const mjModel, d: *mut mjData, island: i32, maxiter: i32); }
    // SAFETY: delegates to C implementation
    unsafe { mj_solCG_island_impl(m, d, island, maxiter) }
}

/// C: mj_solNewton_island (engine/engine_solver.h:48)
/// Calls: mj_solPrimal
#[allow(unused_variables, non_snake_case)]
pub fn mj_sol_newton_island(m: *const mjModel, d: *mut mjData, island: i32, maxiter: i32) {
    extern "C" { fn mj_solNewton_island_impl(m: *const mjModel, d: *mut mjData, island: i32, maxiter: i32); }
    // SAFETY: delegates to C implementation
    unsafe { mj_solNewton_island_impl(m, d, island, maxiter) }
}

/// C: mj_dualFinish (engine/engine_solver.h:51)
/// Calls: dualFinish
#[allow(unused_variables, non_snake_case)]
pub fn mj_dual_finish(m: *const mjModel, d: *mut mjData) {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData)
    // Previous return: ()
    extern "C" { fn mj_dualFinish_impl (m : * const mjModel , d : * mut mjData) ; } unsafe { mj_dualFinish_impl (m , d) }
}

