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
    // WARNING: signature changed — verify body
    // Previous params: (rng : * mut pcg32_state)
    // Previous return: u32
    todo ! ()
}

/// C: shuffle_int (engine/engine_solver.c:257)
/// Calls: pcg32_next
#[allow(unused_variables, non_snake_case)]
pub fn shuffle_int(array: *mut i32, n: i32, rng: *mut pcg32_state) {
    // WARNING: signature changed — verify body
    // Previous params: (array : * mut i32, n : i32, rng : * mut pcg32_state)
    // Previous return: ()
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (force : * mut f64, i : i32, dim : i32, Ac : * mut f64, bc : * mut f64, mu : * const f64)
    // Previous return: ()
    extern "C" { fn solveQCQP_impl (force : * mut f64 , i : i32 , dim : i32 , Ac : * mut f64 , bc : * mut f64 , mu : * const f64) ; } unsafe { solveQCQP_impl (force , i , dim , Ac , bc , mu) }
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
    // WARNING: signature changed — verify body
    // Previous params: (force : * mut f64, mu : * const f64, dim : i32, r#type : i32)
    // Previous return: ()
    todo ! ()
}

/// C: solPGS (engine/engine_solver.c:456)
/// Calls: ARdiaginv, costChange, dualState, dualStateChange, extractBlock, mj_freeStack, mj_isSparse, mj_markStack, mj_stackAllocInfo, mju_clip, mju_copy, mju_dot, mju_gather, mju_mulMatVec, mju_zero, pcg32_next, projectCone, residual, saveStats, shuffle_int, solveQCQP
#[allow(unused_variables, non_snake_case)]
pub fn sol_pgs(m: *const mjModel, d: *mut mjData, island: i32, ne: i32, nf: i32, nefc: i32, efclist: *const i32, maxiter: i32) {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, island : i32, ne : i32, nf : i32, nefc : i32, efclist : * const i32, maxiter : i32)
    // Previous return: ()
    todo ! ()
}

/// C: solNoSlip (engine/engine_solver.c:766)
/// Calls: ARdiaginv, costChange, dualState, dualStateChange, extractBlock, mj_freeStack, mj_markStack, mj_stackAllocInfo, mju_copy, mju_dot, mju_zero, residual, saveStats, solveQCQP
#[allow(unused_variables, non_snake_case)]
pub fn sol_no_slip(m: *const mjModel, d: *mut mjData, island: i32, ne: i32, nf: i32, nefc: i32, efclist: *const i32, maxiter: i32) {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, island : i32, ne : i32, nf : i32, nefc : i32, efclist : * const i32, maxiter : i32)
    // Previous return: ()
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (ctx : * mut mjPrimalContext, p : * mut mjPrimalPnt)
    // Previous return: ()
    todo ! ()
}

/// C: updateBracket (engine/engine_solver.c:1782)
/// Calls: PrimalEval
#[allow(unused_variables, non_snake_case)]
pub fn update_bracket(ctx: *mut mjPrimalContext, p: *mut mjPrimalPnt, candidates: *const mjPrimalPnt, pnext: *mut mjPrimalPnt) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (ctx : * mut mjPrimalContext, p : * mut mjPrimalPnt, candidates : * const mjPrimalPnt, pnext : * mut mjPrimalPnt)
    // Previous return: i32
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (ctx : * mut mjPrimalContext, tolerance : f64, ls_iterations : f64, improvement : * mut f64)
    // Previous return: f64
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, island : i32, maxiter : i32, flg_Newton : i32)
    // Previous return: ()
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, maxiter : i32)
    // Previous return: ()
    todo ! ()
}

/// C: mj_solNewton (engine/engine_solver.h:33)
/// Calls: mj_solPrimal
#[allow(unused_variables, non_snake_case)]
pub fn mj_sol_newton(m: *const mjModel, d: *mut mjData, maxiter: i32) {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, maxiter : i32)
    // Previous return: ()
    todo ! ()
}

/// C: mj_solPGS_island (engine/engine_solver.h:39)
/// Calls: solPGS
#[allow(unused_variables, non_snake_case)]
pub fn mj_sol_pgs_island(m: *const mjModel, d: *mut mjData, island: i32, maxiter: i32) {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, island : i32, maxiter : i32)
    // Previous return: ()
    todo ! ()
}

/// C: mj_solNoSlip_island (engine/engine_solver.h:42)
/// Calls: solNoSlip
#[allow(unused_variables, non_snake_case)]
pub fn mj_sol_no_slip_island(m: *const mjModel, d: *mut mjData, island: i32, maxiter: i32) {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, island : i32, maxiter : i32)
    // Previous return: ()
    todo ! ()
}

/// C: mj_solCG_island (engine/engine_solver.h:45)
/// Calls: mj_solPrimal
#[allow(unused_variables, non_snake_case)]
pub fn mj_sol_cg_island(m: *const mjModel, d: *mut mjData, island: i32, maxiter: i32) {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, island : i32, maxiter : i32)
    // Previous return: ()
    todo ! ()
}

/// C: mj_solNewton_island (engine/engine_solver.h:48)
/// Calls: mj_solPrimal
#[allow(unused_variables, non_snake_case)]
pub fn mj_sol_newton_island(m: *const mjModel, d: *mut mjData, island: i32, maxiter: i32) {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, d : * mut mjData, island : i32, maxiter : i32)
    // Previous return: ()
    todo ! ()
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

