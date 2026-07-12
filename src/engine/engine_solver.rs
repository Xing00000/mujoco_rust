//! Port of: engine/engine_solver.c
//! IR hash: c6d98e4f4b63b7f2
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
    // NOTE: signature changed from previous IR version
    // Previous params: (m : * const mjModel, d : * mut mjData, island : i32, iter : i32, improvement : f64, gradient : f64, lineslope : f64, nactive : i32, nchange : i32, neval : i32, nupdate : i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: dualFinish (engine/engine_solver.c:71)
/// Calls: mj_mulJacTVec, mj_solveM, mju_addTo
#[allow(unused_variables, non_snake_case)]
pub fn dual_finish(m: *const mjModel, d: *mut mjData) {
    // NOTE: signature changed from previous IR version
    // Previous params: (m : * const mjModel, d : * mut mjData)
    // Previous return: ()
    todo!("re-translate: params renamed")
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
    // NOTE: signature changed from previous IR version
    // Previous params: (m : * const mjModel, d : * const mjData, res : * mut f64, nefc : i32, efclist : * const i32, flg_subR : i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
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
    // NOTE: signature changed from previous IR version
    // Previous params: (m : * const mjModel, d : * const mjData, Ac : * mut f64, start : i32, n : i32, flg_subR : i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
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
    // NOTE: signature changed from previous IR version
    // Previous params: (m : * const mjModel, d : * const mjData, res : * mut f64, i : i32, dim : i32, flg_subR : i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
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
    // NOTE: signature changed from previous IR version
    // Previous params: (A : * const f64, force : * mut f64, oldforce : * const f64, res : * const f64, dim : i32)
    // Previous return: f64
    todo!("re-translate: params renamed")
}

/// C: pcg32_next (engine/engine_solver.c:247)
#[allow(unused_variables, non_snake_case)]
pub fn pcg32_next(rng: *mut pcg32_state) -> u32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (rng : * mut pcg32_state)
    // Previous return: u32
    todo!("re-translate: params renamed")
}

/// C: shuffle_int (engine/engine_solver.c:257)
/// Calls: pcg32_next
#[allow(unused_variables, non_snake_case)]
pub fn shuffle_int(array: *mut i32, n: i32, rng: *mut pcg32_state) {
    // NOTE: signature changed from previous IR version
    // Previous params: (array : * mut i32, n : i32, rng : * mut pcg32_state)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: dualState (engine/engine_solver.c:269)
/// Calls: mju_fillInt, mju_norm
#[allow(unused_variables, non_snake_case)]
pub fn dual_state(d: *const mjData, state: *mut i32, ne: i32, nf: i32, nefc: i32, efclist: *const i32) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (d : * const mjData, state : * mut i32, ne : i32, nf : i32, nefc : i32, efclist : * const i32)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: dualStateChange (engine/engine_solver.c:356)
/// Calls: dualState
#[allow(unused_variables, non_snake_case)]
pub fn dual_state_change(d: *const mjData, state: *mut i32, oldstate: *mut i32, ne: i32, nf: i32, nefc: i32, efclist: *const i32, nchange: *mut i32) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (d : * const mjData, state : * mut i32, oldstate : * mut i32, ne : i32, nf : i32, nefc : i32, efclist : * const i32, nchange : * mut i32)
    // Previous return: i32
    todo!("re-translate: params renamed")
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
    // NOTE: signature changed from previous IR version
    // Previous params: (friction : * mut f64, normal : f64, mu : * const f64, dim : i32, feasible : i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
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
    // NOTE: signature changed from previous IR version
    // Previous params: (force : * mut f64, i : i32, dim : i32, Ac : * mut f64, bc : * mut f64, mu : * const f64)
    // Previous return: ()
    todo!("re-translate: params renamed")
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
    // NOTE: signature changed from previous IR version
    // Previous params: (force : * mut f64, mu : * const f64, dim : i32, r#type : i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: solPGS (engine/engine_solver.c:456)
/// Calls: ARdiaginv, costChange, dualState, dualStateChange, extractBlock, mj_freeStack, mj_isSparse, mj_markStack, mj_stackAllocInfo, mju_clip, mju_copy, mju_dot, mju_gather, mju_mulMatVec, mju_zero, pcg32_next, projectCone, residual, saveStats, shuffle_int, solveQCQP
#[allow(unused_variables, non_snake_case)]
pub fn sol_pgs(m: *const mjModel, d: *mut mjData, island: i32, ne: i32, nf: i32, nefc: i32, efclist: *const i32, maxiter: i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (m : * const mjModel, d : * mut mjData, island : i32, ne : i32, nf : i32, nefc : i32, efclist : * const i32, maxiter : i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: solNoSlip (engine/engine_solver.c:766)
/// Calls: ARdiaginv, costChange, dualState, dualStateChange, extractBlock, mj_freeStack, mj_markStack, mj_stackAllocInfo, mju_copy, mju_dot, mju_zero, residual, saveStats, solveQCQP
#[allow(unused_variables, non_snake_case)]
pub fn sol_no_slip(m: *const mjModel, d: *mut mjData, island: i32, ne: i32, nf: i32, nefc: i32, efclist: *const i32, maxiter: i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (m : * const mjModel, d : * mut mjData, island : i32, ne : i32, nf : i32, nefc : i32, efclist : * const i32, maxiter : i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: PrimalPointers (engine/engine_solver.c:1087)
/// Calls: mj_isSparse
#[allow(unused_variables, non_snake_case)]
pub fn primal_pointers(m: *const mjModel, d: *const mjData, ctx: *mut mjPrimalContext, island: i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (m : * const mjModel, d : * const mjData, ctx : * mut mjPrimalContext, island : i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: PrimalAllocate (engine/engine_solver.c:1171)
/// Calls: mj_stackAllocInfo, mju_block, mju_blockSparse, mju_gather, mju_superSparse, mju_transposeSparse
#[allow(unused_variables, non_snake_case)]
pub fn primal_allocate(m: *const mjModel, d: *mut mjData, ctx: *mut mjPrimalContext, flg_Newton: i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (m : * const mjModel, d : * mut mjData, ctx : * mut mjPrimalContext, flg_Newton : i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: PrimalUpdateConstraint (engine/engine_solver.c:1343)
/// Calls: mj_constraintUpdate_impl, mju_mulMatTVec, mju_mulMatVecSparse
#[allow(unused_variables, non_snake_case)]
pub fn primal_update_constraint(ctx: *mut mjPrimalContext, flg_HessianCone: i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (ctx : * mut mjPrimalContext, flg_HessianCone : i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: PrimalUpdateGradient (engine/engine_solver.c:1380)
/// Calls: mj_solveLD, mju_cholSolve, mju_cholSolveSparse, mju_copy
#[allow(unused_variables, non_snake_case)]
pub fn primal_update_gradient(ctx: *mut mjPrimalContext, flg_Newton: i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (ctx : * mut mjPrimalContext, flg_Newton : i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: PrimalPrepare (engine/engine_solver.c:1408)
/// Calls: mju_dot
#[allow(unused_variables, non_snake_case)]
pub fn primal_prepare(ctx: *mut mjPrimalContext) {
    // NOTE: signature changed from previous IR version
    // Previous params: (ctx : * mut mjPrimalContext)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: frictionCost (engine/engine_solver.c:1493)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn friction_cost(x: f64, f: f64, Rf: f64, D: f64) -> f64 {
    // NOTE: signature changed from previous IR version
    // Previous params: (x : f64, f : f64, Rf : f64, D : f64)
    // Previous return: f64
    todo!("re-translate: params renamed")
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
    // NOTE: signature changed from previous IR version
    // Previous params: (start : f64, x : f64, f : f64, Rf : f64, D : f64)
    // Previous return: f64
    todo!("re-translate: params renamed")
}

/// C: ellipticCost (engine/engine_solver.c:1531)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn elliptic_cost(quad: *const f64, alpha: f64, mu: f64, Dm: f64) -> f64 {
    // NOTE: signature changed from previous IR version
    // Previous params: (quad : * const f64, alpha : f64, mu : f64, Dm : f64)
    // Previous return: f64
    todo!("re-translate: params renamed")
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
    // NOTE: signature changed from previous IR version
    // Previous params: (quad : * const f64, alpha : f64, mu : f64, Dm : f64)
    // Previous return: f64
    todo!("re-translate: params renamed")
}

/// C: PrimalEval (engine/engine_solver.c:1631)
/// Calls: ellipticCostDif, frictionCostDif, mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn primal_eval(ctx: *mut mjPrimalContext, p: *mut mjPrimalPnt) {
    // NOTE: signature changed from previous IR version
    // Previous params: (ctx : * mut mjPrimalContext, p : * mut mjPrimalPnt)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: updateBracket (engine/engine_solver.c:1782)
/// Calls: PrimalEval
#[allow(unused_variables, non_snake_case)]
pub fn update_bracket(ctx: *mut mjPrimalContext, p: *mut mjPrimalPnt, candidates: *const mjPrimalPnt, pnext: *mut mjPrimalPnt) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (ctx : * mut mjPrimalContext, p : * mut mjPrimalPnt, candidates : * const mjPrimalPnt, pnext : * mut mjPrimalPnt)
    // Previous return: i32
    todo!("re-translate: params renamed")
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
    // NOTE: signature changed from previous IR version
    // Previous params: (ctx : * mut mjPrimalContext, tolerance : f64, ls_iterations : f64, improvement : * mut f64)
    // Previous return: f64
    todo!("re-translate: params renamed")
}

/// C: MakeHessian (engine/engine_solver.c:2010)
/// Calls: mj_stackAllocInfo, mju_addToMatSparse, mju_addToSymSparse, mju_cholFactorSymbolic, mju_sqrMatTDSparseNumeric, mju_sqrMatTDSparseSymbolic, mju_sqrMatTD_impl, mju_transposeSparse
#[allow(unused_variables, non_snake_case)]
pub fn make_hessian(d: *mut mjData, ctx: *mut mjPrimalContext) {
    // NOTE: signature changed from previous IR version
    // Previous params: (d : * mut mjData, ctx : * mut mjPrimalContext)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: HessianCone (engine/engine_solver.c:2099)
/// Calls: mju_addToScl, mju_cholFactor, mju_cholUpdate, mju_cholUpdateSparse, mju_copy, mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn hessian_cone(d: *mut mjData, ctx: *mut mjPrimalContext) {
    // NOTE: signature changed from previous IR version
    // Previous params: (d : * mut mjData, ctx : * mut mjPrimalContext)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: FactorizeHessian (engine/engine_solver.c:2102)
/// Calls: HessianCone, mju_addToMatSparse, mju_addToSymSparse, mju_cholFactor, mju_cholFactorNumeric, mju_message, mju_sqrMatTDSparseNumeric, mju_sqrMatTDSparseSymbolic, mju_sqrMatTD_impl
#[allow(unused_variables, non_snake_case)]
pub fn factorize_hessian(d: *mut mjData, ctx: *mut mjPrimalContext, flg_recompute: i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (d : * mut mjData, ctx : * mut mjPrimalContext, flg_recompute : i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: HessianIncremental (engine/engine_solver.c:2238)
/// Calls: FactorizeHessian, HessianCone, mju_cholUpdate, mju_cholUpdateSparse, mju_scl
#[allow(unused_variables, non_snake_case)]
pub fn hessian_incremental(d: *mut mjData, ctx: *mut mjPrimalContext, oldstate: *const i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (d : * mut mjData, ctx : * mut mjPrimalContext, oldstate : * const i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mj_solPrimal (engine/engine_solver.c:2297)
/// Calls: FactorizeHessian, HessianIncremental, MakeHessian, PrimalAllocate, PrimalPointers, PrimalSearch, PrimalUpdateConstraint, PrimalUpdateGradient, mj_freeStack, mj_isSparse, mj_markStack, mju_addToScl, mju_copy, mju_copyInt, mju_dot, mju_max, mju_min, mju_mulMatVec, mju_mulMatVecSparse, mju_mulSymVecSparse, mju_norm, mju_scl, mju_sub, mju_subFrom, saveStats
#[allow(unused_variables, non_snake_case)]
pub fn mj_sol_primal(m: *const mjModel, d: *mut mjData, island: i32, maxiter: i32, flg_Newton: i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (m : * const mjModel, d : * mut mjData, island : i32, maxiter : i32, flg_Newton : i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mj_solPGS (engine/engine_solver.h:24)
/// Calls: solPGS
#[allow(unused_variables, non_snake_case)]
pub fn mj_sol_pgs(m: *const mjModel, d: *mut mjData, maxiter: i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (m : * const mjModel, d : * mut mjData, maxiter : i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mj_solNoSlip (engine/engine_solver.h:27)
/// Calls: solNoSlip
#[allow(unused_variables, non_snake_case)]
pub fn mj_sol_no_slip(m: *const mjModel, d: *mut mjData, maxiter: i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (m : * const mjModel, d : * mut mjData, maxiter : i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mj_solCG (engine/engine_solver.h:30)
/// Calls: mj_solPrimal
#[allow(unused_variables, non_snake_case)]
pub fn mj_sol_cg(m: *const mjModel, d: *mut mjData, maxiter: i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (m : * const mjModel, d : * mut mjData, maxiter : i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mj_solNewton (engine/engine_solver.h:33)
/// Calls: mj_solPrimal
#[allow(unused_variables, non_snake_case)]
pub fn mj_sol_newton(m: *const mjModel, d: *mut mjData, maxiter: i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (m : * const mjModel, d : * mut mjData, maxiter : i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mj_solPGS_island (engine/engine_solver.h:39)
/// Calls: solPGS
#[allow(unused_variables, non_snake_case)]
pub fn mj_sol_pgs_island(m: *const mjModel, d: *mut mjData, island: i32, maxiter: i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (m : * const mjModel, d : * mut mjData, island : i32, maxiter : i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mj_solNoSlip_island (engine/engine_solver.h:42)
/// Calls: solNoSlip
#[allow(unused_variables, non_snake_case)]
pub fn mj_sol_no_slip_island(m: *const mjModel, d: *mut mjData, island: i32, maxiter: i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (m : * const mjModel, d : * mut mjData, island : i32, maxiter : i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mj_solCG_island (engine/engine_solver.h:45)
/// Calls: mj_solPrimal
#[allow(unused_variables, non_snake_case)]
pub fn mj_sol_cg_island(m: *const mjModel, d: *mut mjData, island: i32, maxiter: i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (m : * const mjModel, d : * mut mjData, island : i32, maxiter : i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mj_solNewton_island (engine/engine_solver.h:48)
/// Calls: mj_solPrimal
#[allow(unused_variables, non_snake_case)]
pub fn mj_sol_newton_island(m: *const mjModel, d: *mut mjData, island: i32, maxiter: i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (m : * const mjModel, d : * mut mjData, island : i32, maxiter : i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mj_dualFinish (engine/engine_solver.h:51)
/// Calls: dualFinish
#[allow(unused_variables, non_snake_case)]
pub fn mj_dual_finish(m: *const mjModel, d: *mut mjData) {
    // NOTE: signature changed from previous IR version
    // Previous params: (m : * const mjModel, d : * mut mjData)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

