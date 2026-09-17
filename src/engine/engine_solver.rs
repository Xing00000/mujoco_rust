//! Port of: engine/engine_solver.c
//! IR hash: fd5288ad7242f001
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: saveStats (engine/engine_solver.c:41)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn save_stats(m: *const mjModel, d: *mut mjData, island: i32, iter: i32, improvement: f64, gradient: f64, lineslope: f64, nactive: i32, nchange: i32, neval: i32, nupdate: i32) {
    todo!() // cxx-internal:engine_solver.c.o:_saveStats
}

/// C: dualFinish (engine/engine_solver.c:72)
/// Calls: cxx:_mj_mulJacTVec, cxx:_mjd_effSolve, cxx:_mju_addTo
#[allow(unused_variables, non_snake_case)]
pub fn dual_finish(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx-internal:engine_solver.c.o:_dualFinish
}

/// C: mj_dualFinish (engine/engine_solver.c:85)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_solver.c:_dualFinish
#[allow(unused_variables, non_snake_case)]
pub fn mj_dual_finish(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx:_mj_dualFinish
}

/// C: ARdiaginv (engine/engine_solver.c:93)
/// Calls: cxx:_mj_isSparse, cxx:_mju_max
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn a_rdiaginv(m: *const mjModel, d: *const mjData, res: *mut f64, nefc: i32, efclist: *const i32, flg_subR: i32) {
    todo!() // cxx-internal:engine_solver.c.o:_ARdiaginv
}

/// C: extractBlock (engine/engine_solver.c:130)
/// Calls: cxx:_mj_isSparse, cxx:_mju_copy, cxx:_mju_max, cxx:_mju_message
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn extract_block(m: *const mjModel, d: *const mjData, Ac: *mut f64, start: i32, n: i32, flg_subR: i32) {
    todo!() // cxx-internal:engine_solver.c.o:_extractBlock
}

/// C: residual (engine/engine_solver.c:189)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_sparse.h:_mju_dotSparse, cxx:_mj_isSparse, cxx:_mju_dot
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
        if crate::engine::engine_core_util::mj_isSparse(m) != 0 {
            for j in 0..dim {
                let idx = (i + j) as usize;
                *res.add(j as usize) = *(*d).efc_b.add(idx)
                    + crate::engine::engine_util_sparse::mju_dotSparse(
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

/// C: costChange (engine/engine_solver.c:218)
/// Calls: cxx:_mju_copy, cxx:_mju_dot, cxx:_mju_mulVecMatVec, cxx:_mju_sub
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn cost_change(A: *const f64, force: *mut f64, oldforce: *const f64, res: *const f64, dim: i32) -> f64 {
    todo!() // cxx-internal:engine_solver.c.o:_costChange
}

/// C: pcg32_next (engine/engine_solver.c:250)
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

/// C: shuffle_int (engine/engine_solver.c:260)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_solver.c:_pcg32_next
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

/// C: dualState (engine/engine_solver.c:272)
/// Calls: cxx:_mju_fillInt, cxx:_mju_norm
#[allow(unused_variables, non_snake_case)]
pub fn dual_state(d: *const mjData, state: *mut i32, ne: i32, nf: i32, nefc: i32, efclist: *const i32) -> i32 {
    todo!() // cxx-internal:engine_solver.c.o:_dualState
}

/// C: dualStateChange (engine/engine_solver.c:359)
/// Calls: cxx-internal:engine_solver.c.o:_dualState
#[allow(unused_variables, non_snake_case)]
pub fn dual_state_change(d: *const mjData, state: *mut i32, oldstate: *mut i32, ne: i32, nf: i32, nefc: i32, efclist: *const i32, nchange: *mut i32) -> i32 {
    todo!() // cxx-internal:engine_solver.c.o:_dualStateChange
}

/// C: projectEllipsoid (engine/engine_solver.c:386)
/// Calls: cxx:_mju_max
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn project_ellipsoid(friction: *mut f64, normal: f64, mu: *const f64, dim: i32, feasible: i32) {
    todo!() // cxx-internal:engine_solver.c.o:_projectEllipsoid
}

/// C: solveQCQP (engine/engine_solver.c:404)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_solver.c:_projectEllipsoid, cxx:_mju_QCQP, cxx:_mju_QCQP2, cxx:_mju_QCQP3, cxx:_mju_copy
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn solve_qcqp(force: *mut f64, i: i32, dim: i32, Ac: *mut f64, bc: *mut f64, mu: *const f64) {
    todo!() // cxx-internal:engine_solver.c.o:_solveQCQP
}

/// C: projectCone (engine/engine_solver.c:429)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_solver.c:_projectEllipsoid, cxx:_mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn project_cone(force: *mut f64, mu: *const f64, dim: i32, r#type: i32) {
    todo!() // cxx-internal:engine_solver.c.o:_projectCone
}

/// C: solPGS (engine/engine_solver.c:459)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_solver.c:_ARdiaginv, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_solver.c:_costChange, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_solver.c:_extractBlock, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_solver.c:_pcg32_next, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_solver.c:_projectCone, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_solver.c:_residual, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_solver.c:_saveStats, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_solver.c:_shuffle_int, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_solver.c:_solveQCQP, cxx-internal:engine_solver.c.o:_dualState, cxx-internal:engine_solver.c.o:_dualStateChange, cxx:_mj_freeStack, cxx:_mj_isSparse, cxx:_mj_markStack, cxx:_mj_stackAllocInfo, cxx:_mju_clip, cxx:_mju_copy, cxx:_mju_dot, cxx:_mju_gather, cxx:_mju_mulMatVec, cxx:_mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn sol_pgs(m: *const mjModel, d: *mut mjData, island: i32, ne: i32, nf: i32, nefc: i32, efclist: *const i32, maxiter: i32) {
    todo!() // cxx-internal:engine_solver.c.o:_solPGS
}

/// C: mj_solPGS (engine/engine_solver.c:747)
/// Calls: cxx-internal:engine_solver.c.o:_solPGS
#[allow(unused_variables, non_snake_case)]
pub fn mj_sol_pgs(m: *const mjModel, d: *mut mjData, maxiter: i32) {
    todo!() // cxx:_mj_solPGS
}

/// C: mj_solPGS_island (engine/engine_solver.c:753)
/// Calls: cxx-internal:engine_solver.c.o:_solPGS
#[allow(unused_variables, non_snake_case)]
pub fn mj_sol_pgs_island(m: *const mjModel, d: *mut mjData, island: i32, maxiter: i32) {
    todo!() // cxx:_mj_solPGS_island
}

/// C: solNoSlip (engine/engine_solver.c:769)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_solver.c:_ARdiaginv, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_solver.c:_costChange, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_solver.c:_extractBlock, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_solver.c:_residual, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_solver.c:_saveStats, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_solver.c:_solveQCQP, cxx-internal:engine_solver.c.o:_dualState, cxx-internal:engine_solver.c.o:_dualStateChange, cxx:_mj_freeStack, cxx:_mj_markStack, cxx:_mj_stackAllocInfo, cxx:_mju_copy, cxx:_mju_dot, cxx:_mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn sol_no_slip(m: *const mjModel, d: *mut mjData, island: i32, ne: i32, nf: i32, nefc: i32, efclist: *const i32, maxiter: i32) {
    todo!() // cxx-internal:engine_solver.c.o:_solNoSlip
}

/// C: mj_solNoSlip (engine/engine_solver.c:962)
/// Calls: cxx-internal:engine_solver.c.o:_solNoSlip
#[allow(unused_variables, non_snake_case)]
pub fn mj_sol_no_slip(m: *const mjModel, d: *mut mjData, maxiter: i32) {
    todo!() // cxx:_mj_solNoSlip
}

/// C: mj_solNoSlip_island (engine/engine_solver.c:968)
/// Calls: cxx-internal:engine_solver.c.o:_solNoSlip
#[allow(unused_variables, non_snake_case)]
pub fn mj_sol_no_slip_island(m: *const mjModel, d: *mut mjData, island: i32, maxiter: i32) {
    todo!() // cxx:_mj_solNoSlip_island
}

/// C: PrimalPointers (engine/engine_solver.c:1121)
/// Calls: cxx:_mj_isSparse
#[allow(unused_variables, non_snake_case)]
pub fn primal_pointers(m: *const mjModel, d: *const mjData, ctx: *mut mjPrimalContext, island: i32) {
    todo!() // cxx-internal:engine_solver.c.o:_PrimalPointers
}

/// C: PrimalAllocate (engine/engine_solver.c:1206)
/// Calls: cxx:_mj_stackAllocInfo, cxx:_mjd_effRank1Next, cxx:_mju_add, cxx:_mju_addTo, cxx:_mju_block, cxx:_mju_blockSparse, cxx:_mju_gather, cxx:_mju_superSparse, cxx:_mju_transposeSparse, cxx:_mju_zeroInt
#[allow(unused_variables, non_snake_case)]
pub fn primal_allocate(m: *const mjModel, d: *mut mjData, ctx: *mut mjPrimalContext, flg_Newton: i32) {
    todo!() // cxx-internal:engine_solver.c.o:_PrimalAllocate
}

/// C: PrimalUpdateConstraint (engine/engine_solver.c:1456)
/// Calls: cxx:_mj_constraintUpdate_impl, cxx:_mju_mulMatTVec, cxx:_mju_mulMatVecSparse
#[allow(unused_variables, non_snake_case)]
pub fn primal_update_constraint(ctx: *mut mjPrimalContext, flg_HessianCone: i32) {
    todo!() // cxx-internal:engine_solver.c.o:_PrimalUpdateConstraint
}

/// C: PrimalUpdateGrad (engine/engine_solver.c:1493)
#[allow(unused_variables, non_snake_case)]
pub fn primal_update_grad(ctx: *mut mjPrimalContext) {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_solver.c:_PrimalUpdateGrad
}

/// C: PrimalUpdateMgrad (engine/engine_solver.c:1502)
/// Calls: cxx:_mj_solveLD, cxx:_mjd_effPrec, cxx:_mju_cholSolve, cxx:_mju_cholSolveSparse, cxx:_mju_copy
#[allow(unused_variables, non_snake_case)]
pub fn primal_update_mgrad(ctx: *mut mjPrimalContext, flg_Newton: i32) {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_solver.c:_PrimalUpdateMgrad
}

/// C: PrimalUpdateGradient (engine/engine_solver.c:1531)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_solver.c:_PrimalUpdateGrad, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_solver.c:_PrimalUpdateMgrad
#[allow(unused_variables, non_snake_case)]
pub fn primal_update_gradient(ctx: *mut mjPrimalContext, flg_Newton: i32) {
    todo!() // cxx-internal:engine_solver.c.o:_PrimalUpdateGradient
}

/// C: PrimalPrepare (engine/engine_solver.c:1538)
/// Calls: cxx:_mju_dot
#[allow(unused_variables, non_snake_case)]
pub fn primal_prepare(ctx: *mut mjPrimalContext) {
    todo!() // cxx-internal:engine_solver.c.o:_PrimalPrepare
}

/// C: frictionCost (engine/engine_solver.c:1623)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn friction_cost(x: f64, f: f64, Rf: f64, D: f64) -> f64 {
    todo!() // cxx-internal:engine_solver.c.o:_frictionCost
}

/// C: frictionCostDif (engine/engine_solver.c:1636)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_solver.c:_frictionCost
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn friction_cost_dif(start: f64, x: f64, f: f64, Rf: f64, D: f64) -> f64 {
    todo!() // cxx-internal:engine_solver.c.o:_frictionCostDif
}

/// C: ellipticCostDif (engine/engine_solver.c:1661)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn elliptic_cost_dif(quad: *const f64, alpha: f64, mu: f64, Dm: f64) -> f64 {
    todo!() // cxx-internal:engine_solver.c.o:_ellipticCostDif
}

/// C: PrimalEval (engine/engine_solver.c:1762)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_solver.c:_ellipticCostDif, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_solver.c:_frictionCostDif, cxx:_mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn primal_eval(ctx: *mut mjPrimalContext, p: *mut mjPrimalPnt) {
    todo!() // cxx-internal:engine_solver.c.o:_PrimalEval
}

/// C: updateBracket (engine/engine_solver.c:1913)
/// Calls: cxx-internal:engine_solver.c.o:_PrimalEval
#[allow(unused_variables, non_snake_case)]
pub fn update_bracket(ctx: *mut mjPrimalContext, p: *mut mjPrimalPnt, candidates: *const mjPrimalPnt, pnext: *mut mjPrimalPnt) -> i32 {
    todo!() // cxx-internal:engine_solver.c.o:_updateBracket
}

/// C: PrimalSearch (engine/engine_solver.c:1943)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_solver.c:_PrimalPrepare, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_solver.c:_updateBracket, cxx-internal:engine_solver.c.o:_PrimalEval, cxx:_mjd_effMulAdd, cxx:_mjd_effMulAddIsland, cxx:_mju_mulMatVec, cxx:_mju_mulMatVecSparse, cxx:_mju_mulSymVecSparse, cxx:_mju_norm
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn primal_search(ctx: *mut mjPrimalContext, tolerance: f64, ls_iterations: f64, improvement: *mut f64) -> f64 {
    todo!() // cxx-internal:engine_solver.c.o:_PrimalSearch
}

/// C: MakeMetricLower (engine/engine_solver.c:2153)
/// Calls: cxx:_mj_freeStack, cxx:_mj_markStack, cxx:_mj_stackAllocInfo, cxx:_mjd_effRank1Next, cxx:_mju_message, cxx:_mju_zeroInt
#[allow(unused_variables, non_snake_case)]
pub fn make_metric_lower(d: *mut mjData, ctx: *mut mjPrimalContext) {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_solver.c:_MakeMetricLower
}

/// C: MakeHessian (engine/engine_solver.c:2311)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_solver.c:_MakeMetricLower, cxx:_mj_stackAllocInfo, cxx:_mju_addToMatSparse, cxx:_mju_addToSymSparse, cxx:_mju_cholFactorSymbolic, cxx:_mju_sqrMatTDSparseNumeric, cxx:_mju_sqrMatTDSparseSymbolic, cxx:_mju_sqrMatTD_impl, cxx:_mju_transposeSparse
#[allow(unused_variables, non_snake_case)]
pub fn make_hessian(d: *mut mjData, ctx: *mut mjPrimalContext) {
    todo!() // cxx-internal:engine_solver.c.o:_MakeHessian
}

/// C: HessianCone (engine/engine_solver.c:2437)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_solver.c:_HessianConeFoldFaster, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_solver.c:_HessianConeFolded, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_solver.c:_HessianConeUpdate
#[allow(unused_variables, non_snake_case)]
pub fn hessian_cone(d: *mut mjData, ctx: *mut mjPrimalContext) {
    todo!() // cxx-internal:engine_solver.c.o:_HessianCone
}

/// C: FactorizeHessian (engine/engine_solver.c:2440)
/// Calls: cxx-internal:engine_solver.c.o:_HessianCone, cxx:_mju_addToMatSparse, cxx:_mju_addToSymSparse, cxx:_mju_cholFactor, cxx:_mju_cholFactorNumeric, cxx:_mju_message, cxx:_mju_sqrMatTDSparseNumeric, cxx:_mju_sqrMatTDSparseSymbolic, cxx:_mju_sqrMatTD_impl
#[allow(unused_variables, non_snake_case)]
pub fn factorize_hessian(d: *mut mjData, ctx: *mut mjPrimalContext, flg_recompute: i32) {
    todo!() // cxx-internal:engine_solver.c.o:_FactorizeHessian
}

/// C: HessianConeFolded (engine/engine_solver.c:2522)
/// Calls: cxx:_mj_stackAllocInfo, cxx:_mju_addToMatSparse, cxx:_mju_addToScl, cxx:_mju_cholFactor, cxx:_mju_cholFactorNumeric, cxx:_mju_copy, cxx:_mju_message, cxx:_mju_sqrMatTDSparseNumeric, cxx:_mju_transposeSparse, cxx:_mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn hessian_cone_folded(d: *mut mjData, ctx: *mut mjPrimalContext) {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_solver.c:_HessianConeFolded
}

/// C: HessianConeUpdate (engine/engine_solver.c:2594)
/// Calls: cxx:_mju_addToScl, cxx:_mju_cholFactor, cxx:_mju_cholUpdate, cxx:_mju_cholUpdateSparse, cxx:_mju_copy, cxx:_mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn hessian_cone_update(d: *mut mjData, ctx: *mut mjPrimalContext) {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_solver.c:_HessianConeUpdate
}

/// C: HessianConeFoldFaster (engine/engine_solver.c:2664)
#[allow(unused_variables, non_snake_case)]
pub fn hessian_cone_fold_faster(ctx: *const mjPrimalContext) -> i32 {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_solver.c:_HessianConeFoldFaster
}

/// C: HessianIncremental (engine/engine_solver.c:2706)
/// Calls: cxx-internal:engine_solver.c.o:_FactorizeHessian, cxx-internal:engine_solver.c.o:_HessianCone, cxx:_mju_cholUpdate, cxx:_mju_cholUpdateSparse, cxx:_mju_scl
#[allow(unused_variables, non_snake_case)]
pub fn hessian_incremental(d: *mut mjData, ctx: *mut mjPrimalContext, oldstate: *const i32) {
    todo!() // cxx-internal:engine_solver.c.o:_HessianIncremental
}

/// C: mj_solPrimal (engine/engine_solver.c:2770)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_solver.c:_HessianIncremental, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_solver.c:_MakeHessian, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_solver.c:_PrimalAllocate, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_solver.c:_PrimalPointers, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_solver.c:_PrimalSearch, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_solver.c:_PrimalUpdateGrad, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_solver.c:_PrimalUpdateGradient, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_solver.c:_PrimalUpdateMgrad, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_solver.c:_saveStats, cxx-internal:engine_solver.c.o:_FactorizeHessian, cxx-internal:engine_solver.c.o:_PrimalUpdateConstraint, cxx:_mj_freeStack, cxx:_mj_isSparse, cxx:_mj_markStack, cxx:_mjd_effMulAdd, cxx:_mjd_effMulAddIsland, cxx:_mju_addToScl, cxx:_mju_copy, cxx:_mju_copyInt, cxx:_mju_dot, cxx:_mju_max, cxx:_mju_min, cxx:_mju_mulMatVec, cxx:_mju_mulMatVecSparse, cxx:_mju_mulSymVecSparse, cxx:_mju_norm, cxx:_mju_scl, cxx:_mju_sub, cxx:_mju_subFrom
#[allow(unused_variables, non_snake_case)]
pub fn mj_sol_primal(m: *const mjModel, d: *mut mjData, island: i32, maxiter: i32, flg_Newton: i32) {
    todo!() // cxx-internal:engine_solver.c.o:_mj_solPrimal
}

/// C: mj_solCG (engine/engine_solver.c:2997)
/// Calls: cxx-internal:engine_solver.c.o:_mj_solPrimal
#[allow(unused_variables, non_snake_case)]
pub fn mj_sol_cg(m: *const mjModel, d: *mut mjData, maxiter: i32) {
    todo!() // cxx:_mj_solCG
}

/// C: mj_solCG_island (engine/engine_solver.c:3003)
/// Calls: cxx-internal:engine_solver.c.o:_mj_solPrimal
#[allow(unused_variables, non_snake_case)]
pub fn mj_sol_cg_island(m: *const mjModel, d: *mut mjData, island: i32, maxiter: i32) {
    todo!() // cxx:_mj_solCG_island
}

/// C: mj_solNewton (engine/engine_solver.c:3009)
/// Calls: cxx-internal:engine_solver.c.o:_mj_solPrimal
#[allow(unused_variables, non_snake_case)]
pub fn mj_sol_newton(m: *const mjModel, d: *mut mjData, maxiter: i32) {
    todo!() // cxx:_mj_solNewton
}

/// C: mj_solNewton_island (engine/engine_solver.c:3015)
/// Calls: cxx-internal:engine_solver.c.o:_mj_solPrimal
#[allow(unused_variables, non_snake_case)]
pub fn mj_sol_newton_island(m: *const mjModel, d: *mut mjData, island: i32, maxiter: i32) {
    todo!() // cxx:_mj_solNewton_island
}

