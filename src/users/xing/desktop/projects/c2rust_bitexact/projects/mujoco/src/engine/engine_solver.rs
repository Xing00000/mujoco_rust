//! Port of: Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_solver.c
//! IR hash: 1fa44043fe383ce9
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: saveStats (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_solver.c:40)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn save_stats(m: *const mjModel, d: *mut mjData, island: i32, iter: i32, improvement: f64, gradient: f64, lineslope: f64, nactive: i32, nchange: i32, neval: i32, nupdate: i32) {
    todo!() // cxx-internal:engine_solver.c.o:_saveStats
}

/// C: dualFinish (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_solver.c:71)
/// Calls: cxx:_mj_mulJacTVec, cxx:_mj_solveM, cxx:_mju_addTo
#[allow(unused_variables, non_snake_case)]
pub fn dual_finish(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx-internal:engine_solver.c.o:_dualFinish
}

/// C: ARdiaginv (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_solver.c:90)
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

/// C: extractBlock (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_solver.c:127)
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

/// C: residual (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_solver.c:186)
/// Calls: cxx:_mj_isSparse, cxx:_mju_dot
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn residual(m: *const mjModel, d: *const mjData, res: *mut f64, i: i32, dim: i32, flg_subR: i32) {
    todo!() // cxx-internal:engine_solver.c.o:_residual
}

/// C: costChange (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_solver.c:215)
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

/// C: pcg32_next (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_solver.c:247)
#[allow(unused_variables, non_snake_case)]
pub fn pcg32_next(rng: *mut pcg32_state) -> u32 {
    todo!() // cxx-internal:engine_solver.c.o:_pcg32_next
}

/// C: shuffle_int (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_solver.c:257)
/// Calls: cxx-internal:engine_solver.c.o:_pcg32_next
#[allow(unused_variables, non_snake_case)]
pub fn shuffle_int(array: *mut i32, n: i32, rng: *mut pcg32_state) {
    todo!() // cxx-internal:engine_solver.c.o:_shuffle_int
}

/// C: dualState (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_solver.c:269)
/// Calls: cxx:_mju_fillInt, cxx:_mju_norm
#[allow(unused_variables, non_snake_case)]
pub fn dual_state(d: *const mjData, state: *mut i32, ne: i32, nf: i32, nefc: i32, efclist: *const i32) -> i32 {
    todo!() // cxx-internal:engine_solver.c.o:_dualState
}

/// C: dualStateChange (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_solver.c:356)
/// Calls: cxx-internal:engine_solver.c.o:_dualState
#[allow(unused_variables, non_snake_case)]
pub fn dual_state_change(d: *const mjData, state: *mut i32, oldstate: *mut i32, ne: i32, nf: i32, nefc: i32, efclist: *const i32, nchange: *mut i32) -> i32 {
    todo!() // cxx-internal:engine_solver.c.o:_dualStateChange
}

/// C: projectEllipsoid (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_solver.c:383)
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

/// C: solveQCQP (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_solver.c:401)
/// Calls: cxx-internal:engine_solver.c.o:_projectEllipsoid, cxx:_mju_QCQP, cxx:_mju_QCQP2, cxx:_mju_QCQP3, cxx:_mju_copy
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn solve_qcqp(force: *mut f64, i: i32, dim: i32, Ac: *mut f64, bc: *mut f64, mu: *const f64) {
    todo!() // cxx-internal:engine_solver.c.o:_solveQCQP
}

/// C: projectCone (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_solver.c:426)
/// Calls: cxx-internal:engine_solver.c.o:_projectEllipsoid, cxx:_mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn project_cone(force: *mut f64, mu: *const f64, dim: i32, r#type: i32) {
    todo!() // cxx-internal:engine_solver.c.o:_projectCone
}

/// C: solPGS (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_solver.c:456)
/// Calls: cxx-internal:engine_solver.c.o:_ARdiaginv, cxx-internal:engine_solver.c.o:_costChange, cxx-internal:engine_solver.c.o:_dualState, cxx-internal:engine_solver.c.o:_dualStateChange, cxx-internal:engine_solver.c.o:_extractBlock, cxx-internal:engine_solver.c.o:_pcg32_next, cxx-internal:engine_solver.c.o:_projectCone, cxx-internal:engine_solver.c.o:_residual, cxx-internal:engine_solver.c.o:_saveStats, cxx-internal:engine_solver.c.o:_shuffle_int, cxx-internal:engine_solver.c.o:_solveQCQP, cxx:_mj_freeStack, cxx:_mj_isSparse, cxx:_mj_markStack, cxx:_mj_stackAllocInfo, cxx:_mju_clip, cxx:_mju_copy, cxx:_mju_dot, cxx:_mju_gather, cxx:_mju_mulMatVec, cxx:_mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn sol_pgs(m: *const mjModel, d: *mut mjData, island: i32, ne: i32, nf: i32, nefc: i32, efclist: *const i32, maxiter: i32) {
    todo!() // cxx-internal:engine_solver.c.o:_solPGS
}

/// C: solNoSlip (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_solver.c:766)
/// Calls: cxx-internal:engine_solver.c.o:_ARdiaginv, cxx-internal:engine_solver.c.o:_costChange, cxx-internal:engine_solver.c.o:_dualState, cxx-internal:engine_solver.c.o:_dualStateChange, cxx-internal:engine_solver.c.o:_extractBlock, cxx-internal:engine_solver.c.o:_residual, cxx-internal:engine_solver.c.o:_saveStats, cxx-internal:engine_solver.c.o:_solveQCQP, cxx:_mj_freeStack, cxx:_mj_markStack, cxx:_mj_stackAllocInfo, cxx:_mju_copy, cxx:_mju_dot, cxx:_mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn sol_no_slip(m: *const mjModel, d: *mut mjData, island: i32, ne: i32, nf: i32, nefc: i32, efclist: *const i32, maxiter: i32) {
    todo!() // cxx-internal:engine_solver.c.o:_solNoSlip
}

/// C: PrimalPointers (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_solver.c:1087)
/// Calls: cxx:_mj_isSparse
#[allow(unused_variables, non_snake_case)]
pub fn primal_pointers(m: *const mjModel, d: *const mjData, ctx: *mut mjPrimalContext, island: i32) {
    todo!() // cxx-internal:engine_solver.c.o:_PrimalPointers
}

/// C: PrimalAllocate (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_solver.c:1171)
/// Calls: cxx:_mj_stackAllocInfo, cxx:_mju_block, cxx:_mju_blockSparse, cxx:_mju_gather, cxx:_mju_superSparse, cxx:_mju_transposeSparse
#[allow(unused_variables, non_snake_case)]
pub fn primal_allocate(m: *const mjModel, d: *mut mjData, ctx: *mut mjPrimalContext, flg_Newton: i32) {
    todo!() // cxx-internal:engine_solver.c.o:_PrimalAllocate
}

/// C: PrimalUpdateConstraint (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_solver.c:1343)
/// Calls: cxx:_mj_constraintUpdate_impl, cxx:_mju_mulMatTVec, cxx:_mju_mulMatVecSparse
#[allow(unused_variables, non_snake_case)]
pub fn primal_update_constraint(ctx: *mut mjPrimalContext, flg_HessianCone: i32) {
    todo!() // cxx-internal:engine_solver.c.o:_PrimalUpdateConstraint
}

/// C: PrimalUpdateGradient (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_solver.c:1380)
/// Calls: cxx:_mj_solveLD, cxx:_mju_cholSolve, cxx:_mju_cholSolveSparse, cxx:_mju_copy
#[allow(unused_variables, non_snake_case)]
pub fn primal_update_gradient(ctx: *mut mjPrimalContext, flg_Newton: i32) {
    todo!() // cxx-internal:engine_solver.c.o:_PrimalUpdateGradient
}

/// C: PrimalPrepare (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_solver.c:1408)
/// Calls: cxx:_mju_dot
#[allow(unused_variables, non_snake_case)]
pub fn primal_prepare(ctx: *mut mjPrimalContext) {
    todo!() // cxx-internal:engine_solver.c.o:_PrimalPrepare
}

/// C: frictionCost (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_solver.c:1493)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn friction_cost(x: f64, f: f64, Rf: f64, D: f64) -> f64 {
    todo!() // cxx-internal:engine_solver.c.o:_frictionCost
}

/// C: frictionCostDif (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_solver.c:1506)
/// Calls: cxx-internal:engine_solver.c.o:_frictionCost
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn friction_cost_dif(start: f64, x: f64, f: f64, Rf: f64, D: f64) -> f64 {
    todo!() // cxx-internal:engine_solver.c.o:_frictionCostDif
}

/// C: ellipticCost (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_solver.c:1531)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn elliptic_cost(quad: *const f64, alpha: f64, mu: f64, Dm: f64) -> f64 {
    todo!() // cxx-internal:engine_solver.c.o:_ellipticCost
}

/// C: ellipticCostDif (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_solver.c:1569)
/// Calls: cxx-internal:engine_solver.c.o:_ellipticCost
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn elliptic_cost_dif(quad: *const f64, alpha: f64, mu: f64, Dm: f64) -> f64 {
    todo!() // cxx-internal:engine_solver.c.o:_ellipticCostDif
}

/// C: PrimalEval (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_solver.c:1631)
/// Calls: cxx-internal:engine_solver.c.o:_ellipticCostDif, cxx-internal:engine_solver.c.o:_frictionCostDif, cxx:_mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn primal_eval(ctx: *mut mjPrimalContext, p: *mut mjPrimalPnt) {
    todo!() // cxx-internal:engine_solver.c.o:_PrimalEval
}

/// C: updateBracket (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_solver.c:1782)
/// Calls: cxx-internal:engine_solver.c.o:_PrimalEval
#[allow(unused_variables, non_snake_case)]
pub fn update_bracket(ctx: *mut mjPrimalContext, p: *mut mjPrimalPnt, candidates: *const mjPrimalPnt, pnext: *mut mjPrimalPnt) -> i32 {
    todo!() // cxx-internal:engine_solver.c.o:_updateBracket
}

/// C: PrimalSearch (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_solver.c:1812)
/// Calls: cxx-internal:engine_solver.c.o:_PrimalEval, cxx-internal:engine_solver.c.o:_PrimalPrepare, cxx-internal:engine_solver.c.o:_updateBracket, cxx:_mju_mulMatVec, cxx:_mju_mulMatVecSparse, cxx:_mju_mulSymVecSparse, cxx:_mju_norm
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn primal_search(ctx: *mut mjPrimalContext, tolerance: f64, ls_iterations: f64, improvement: *mut f64) -> f64 {
    todo!() // cxx-internal:engine_solver.c.o:_PrimalSearch
}

/// C: MakeHessian (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_solver.c:2010)
/// Calls: cxx:_mj_stackAllocInfo, cxx:_mju_addToMatSparse, cxx:_mju_addToSymSparse, cxx:_mju_cholFactorSymbolic, cxx:_mju_sqrMatTDSparseNumeric, cxx:_mju_sqrMatTDSparseSymbolic, cxx:_mju_sqrMatTD_impl, cxx:_mju_transposeSparse
#[allow(unused_variables, non_snake_case)]
pub fn make_hessian(d: *mut mjData, ctx: *mut mjPrimalContext) {
    todo!() // cxx-internal:engine_solver.c.o:_MakeHessian
}

/// C: HessianCone (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_solver.c:2099)
/// Calls: cxx:_mju_addToScl, cxx:_mju_cholFactor, cxx:_mju_cholUpdate, cxx:_mju_cholUpdateSparse, cxx:_mju_copy, cxx:_mju_zero
#[allow(unused_variables, non_snake_case)]
pub fn hessian_cone(d: *mut mjData, ctx: *mut mjPrimalContext) {
    todo!() // cxx-internal:engine_solver.c.o:_HessianCone
}

/// C: FactorizeHessian (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_solver.c:2102)
/// Calls: cxx-internal:engine_solver.c.o:_HessianCone, cxx:_mju_addToMatSparse, cxx:_mju_addToSymSparse, cxx:_mju_cholFactor, cxx:_mju_cholFactorNumeric, cxx:_mju_message, cxx:_mju_sqrMatTDSparseNumeric, cxx:_mju_sqrMatTDSparseSymbolic, cxx:_mju_sqrMatTD_impl
#[allow(unused_variables, non_snake_case)]
pub fn factorize_hessian(d: *mut mjData, ctx: *mut mjPrimalContext, flg_recompute: i32) {
    todo!() // cxx-internal:engine_solver.c.o:_FactorizeHessian
}

/// C: HessianIncremental (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_solver.c:2238)
/// Calls: cxx-internal:engine_solver.c.o:_FactorizeHessian, cxx-internal:engine_solver.c.o:_HessianCone, cxx:_mju_cholUpdate, cxx:_mju_cholUpdateSparse, cxx:_mju_scl
#[allow(unused_variables, non_snake_case)]
pub fn hessian_incremental(d: *mut mjData, ctx: *mut mjPrimalContext, oldstate: *const i32) {
    todo!() // cxx-internal:engine_solver.c.o:_HessianIncremental
}

/// C: mj_solPrimal (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_solver.c:2297)
/// Calls: cxx-internal:engine_solver.c.o:_FactorizeHessian, cxx-internal:engine_solver.c.o:_HessianIncremental, cxx-internal:engine_solver.c.o:_MakeHessian, cxx-internal:engine_solver.c.o:_PrimalAllocate, cxx-internal:engine_solver.c.o:_PrimalPointers, cxx-internal:engine_solver.c.o:_PrimalSearch, cxx-internal:engine_solver.c.o:_PrimalUpdateConstraint, cxx-internal:engine_solver.c.o:_PrimalUpdateGradient, cxx-internal:engine_solver.c.o:_saveStats, cxx:_mj_freeStack, cxx:_mj_isSparse, cxx:_mj_markStack, cxx:_mju_addToScl, cxx:_mju_copy, cxx:_mju_copyInt, cxx:_mju_dot, cxx:_mju_max, cxx:_mju_min, cxx:_mju_mulMatVec, cxx:_mju_mulMatVecSparse, cxx:_mju_mulSymVecSparse, cxx:_mju_norm, cxx:_mju_scl, cxx:_mju_sub, cxx:_mju_subFrom
#[allow(unused_variables, non_snake_case)]
pub fn mj_sol_primal(m: *const mjModel, d: *mut mjData, island: i32, maxiter: i32, flg_Newton: i32) {
    todo!() // cxx-internal:engine_solver.c.o:_mj_solPrimal
}

/// C: mj_solPGS (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_solver.h:24)
/// Calls: cxx-internal:engine_solver.c.o:_solPGS
#[allow(unused_variables, non_snake_case)]
pub fn mj_sol_pgs(m: *const mjModel, d: *mut mjData, maxiter: i32) {
    todo!() // cxx:_mj_solPGS
}

/// C: mj_solNoSlip (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_solver.h:27)
/// Calls: cxx-internal:engine_solver.c.o:_solNoSlip
#[allow(unused_variables, non_snake_case)]
pub fn mj_sol_no_slip(m: *const mjModel, d: *mut mjData, maxiter: i32) {
    todo!() // cxx:_mj_solNoSlip
}

/// C: mj_solCG (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_solver.h:30)
/// Calls: cxx-internal:engine_solver.c.o:_mj_solPrimal
#[allow(unused_variables, non_snake_case)]
pub fn mj_sol_cg(m: *const mjModel, d: *mut mjData, maxiter: i32) {
    todo!() // cxx:_mj_solCG
}

/// C: mj_solNewton (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_solver.h:33)
/// Calls: cxx-internal:engine_solver.c.o:_mj_solPrimal
#[allow(unused_variables, non_snake_case)]
pub fn mj_sol_newton(m: *const mjModel, d: *mut mjData, maxiter: i32) {
    todo!() // cxx:_mj_solNewton
}

/// C: mj_solPGS_island (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_solver.h:39)
/// Calls: cxx-internal:engine_solver.c.o:_solPGS
#[allow(unused_variables, non_snake_case)]
pub fn mj_sol_pgs_island(m: *const mjModel, d: *mut mjData, island: i32, maxiter: i32) {
    todo!() // cxx:_mj_solPGS_island
}

/// C: mj_solNoSlip_island (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_solver.h:42)
/// Calls: cxx-internal:engine_solver.c.o:_solNoSlip
#[allow(unused_variables, non_snake_case)]
pub fn mj_sol_no_slip_island(m: *const mjModel, d: *mut mjData, island: i32, maxiter: i32) {
    todo!() // cxx:_mj_solNoSlip_island
}

/// C: mj_solCG_island (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_solver.h:45)
/// Calls: cxx-internal:engine_solver.c.o:_mj_solPrimal
#[allow(unused_variables, non_snake_case)]
pub fn mj_sol_cg_island(m: *const mjModel, d: *mut mjData, island: i32, maxiter: i32) {
    todo!() // cxx:_mj_solCG_island
}

/// C: mj_solNewton_island (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_solver.h:48)
/// Calls: cxx-internal:engine_solver.c.o:_mj_solPrimal
#[allow(unused_variables, non_snake_case)]
pub fn mj_sol_newton_island(m: *const mjModel, d: *mut mjData, island: i32, maxiter: i32) {
    todo!() // cxx:_mj_solNewton_island
}

/// C: mj_dualFinish (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_solver.h:51)
/// Calls: cxx-internal:engine_solver.c.o:_dualFinish
#[allow(unused_variables, non_snake_case)]
pub fn mj_dual_finish(m: *const mjModel, d: *mut mjData) {
    todo!() // cxx:_mj_dualFinish
}

