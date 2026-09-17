//! Port of: Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_solve.c
//! IR hash: 1fa44043fe383ce9
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mulVecMatVecSym (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_solve.c:1400)
/// Calls: cxx:_mju_dot
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mul_vec_mat_vec_sym(vec: *const f64, mat: *const f64, n: i32) -> f64 {
    todo!() // cxx-internal:engine_util_solve.c.o:_mulVecMatVecSym
}

/// C: mulSymVec (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_solve.c:1412)
/// Calls: cxx:_mju_addToScl, cxx:_mju_dot
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mul_sym_vec(res: *mut f64, mat: *const f64, vec: *const f64, n: i32) {
    todo!() // cxx-internal:engine_util_solve.c.o:_mulSymVec
}

/// C: mju_cholFactor (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_solve.h:27)
/// Calls: cxx:_mju_dot
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_chol_factor(mat: *mut f64, n: i32, mindiag: f64) -> i32 {
    todo!() // cxx:_mju_cholFactor
}

/// C: mju_cholSolve (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_solve.h:30)
/// Calls: cxx:_mju_copy, cxx:_mju_dot
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_chol_solve(res: *mut f64, mat: *const f64, vec: *const f64, n: i32) {
    todo!() // cxx:_mju_cholSolve
}

/// C: mju_cholUpdate (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_solve.h:33)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_chol_update(mat: *mut f64, x: *mut f64, n: i32, flg_plus: i32) -> i32 {
    todo!() // cxx:_mju_cholUpdate
}

/// C: mju_cholFactorSparse (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_solve.h:37)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_chol_factor_sparse(mat: *mut f64, n: i32, mindiag: f64, rownnz: *mut i32, rowadr: *const i32, colind: *mut i32, d: *mut mjData) -> i32 {
    todo!() // cxx:_mju_cholFactorSparse
}

/// C: mju_cholFactorSymbolic (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_solve.h:45)
/// Calls: cxx:_mj_freeStack, cxx:_mj_markStack, cxx:_mj_stackAllocInfo
#[allow(unused_variables, non_snake_case)]
pub fn mju_chol_factor_symbolic(L_colind: *mut i32, L_rownnz: *mut i32, L_rowadr: *mut i32, LT_colind: *mut i32, LT_rownnz: *mut i32, LT_rowadr: *mut i32, LT_map: *mut i32, rownnz: *const i32, rowadr: *const i32, colind: *const i32, n: i32, d: *mut mjData) -> i32 {
    todo!() // cxx:_mju_cholFactorSymbolic
}

/// C: mju_cholFactorNumeric (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_solve.h:53)
/// Calls: cxx:_mj_freeStack, cxx:_mj_markStack, cxx:_mj_stackAllocInfo, cxx:_mju_scatter, cxx:_mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_chol_factor_numeric(L: *mut f64, n: i32, mindiag: f64, L_rownnz: *const i32, L_rowadr: *const i32, L_colind: *const i32, LT_rownnz: *const i32, LT_rowadr: *const i32, LT_colind: *const i32, LT_map: *const i32, H: *const f64, H_rownnz: *const i32, H_rowadr: *const i32, H_colind: *const i32, d: *mut mjData) -> i32 {
    todo!() // cxx:_mju_cholFactorNumeric
}

/// C: mju_cholSolveSparse (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_solve.h:61)
/// Calls: cxx:_mju_copy
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_chol_solve_sparse(res: *mut f64, mat: *const f64, vec: *const f64, n: i32, rownnz: *const i32, rowadr: *const i32, colind: *const i32) {
    todo!() // cxx:_mju_cholSolveSparse
}

/// C: mju_cholUpdateSparse (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_solve.h:66)
/// Calls: cxx:_mj_freeStack, cxx:_mj_markStack, cxx:_mj_stackAllocInfo, cxx:_mju_scatter, cxx:_mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_chol_update_sparse(mat: *mut f64, x: *const f64, n: i32, flg_plus: i32, rownnz: *const i32, rowadr: *const i32, colind: *const i32, x_nnz: i32, x_ind: *const i32, d: *mut mjData) -> i32 {
    todo!() // cxx:_mju_cholUpdateSparse
}

/// C: mju_cholFactorBand (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_solve.h:76)
/// Calls: cxx:_mju_dot
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_chol_factor_band(mat: *mut f64, ntotal: i32, nband: i32, ndense: i32, diagadd: f64, diagmul: f64) -> f64 {
    todo!() // cxx:_mju_cholFactorBand
}

/// C: mju_cholSolveBand (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_solve.h:80)
/// Calls: cxx:_mju_copy, cxx:_mju_dot
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_chol_solve_band(res: *mut f64, mat: *const f64, vec: *const f64, ntotal: i32, nband: i32, ndense: i32) {
    todo!() // cxx:_mju_cholSolveBand
}

/// C: mju_band2Dense (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_solve.h:84)
/// Calls: cxx:_mju_copy, cxx:_mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_band2dense(res: *mut f64, mat: *const f64, ntotal: i32, nband: i32, ndense: i32, flg_sym: bool) {
    todo!() // cxx:_mju_band2Dense
}

/// C: mju_dense2Band (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_solve.h:88)
/// Calls: cxx:_mju_copy
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_dense2band(res: *mut f64, mat: *const f64, ntotal: i32, nband: i32, ndense: i32) {
    todo!() // cxx:_mju_dense2Band
}

/// C: mju_bandMulMatVec (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_solve.h:91)
/// Calls: cxx:_mju_addToScl, cxx:_mju_dot
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_band_mul_mat_vec(res: *mut f64, mat: *const f64, vec: *const f64, ntotal: i32, nband: i32, ndense: i32, nvec: i32, flg_sym: bool) {
    todo!() // cxx:_mju_bandMulMatVec
}

/// C: mju_bandDiag (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_solve.h:95)
#[allow(unused_variables, non_snake_case)]
pub fn mju_band_diag(i: i32, ntotal: i32, nband: i32, ndense: i32) -> i32 {
    todo!() // cxx:_mju_bandDiag
}

/// C: mju_factorLU (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_solve.h:102)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_factor_lu(A: *mut f64, n: i32, pivot: *mut i32) -> i32 {
    todo!() // cxx:_mju_factorLU
}

/// C: mju_solveLU (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_solve.h:105)
/// Calls: cxx:_mju_copy
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_solve_lu(x: *mut f64, LU: *const f64, b: *const f64, pivot: *const i32, n: i32) {
    todo!() // cxx:_mju_solveLU
}

/// C: mju_factorLUSparse (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_solve.h:109)
/// Calls: cxx:_mju_copyInt, cxx:_mju_message
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_factor_lu_sparse(LU: *mut f64, n: i32, scratch: *mut i32, rownnz: *const i32, rowadr: *const i32, colind: *const i32, index: *const i32) {
    todo!() // cxx:_mju_factorLUSparse
}

/// C: mju_solveLUSparse (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_solve.h:113)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_solve_lu_sparse(res: *mut f64, LU: *const f64, vec: *const f64, n: i32, rownnz: *const i32, rowadr: *const i32, diag: *const i32, colind: *const i32, index: *const i32) {
    todo!() // cxx:_mju_solveLUSparse
}

/// C: mju_solve3 (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_solve.h:118)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_solve3(x: *mut f64, A: *const f64, b: *const f64) {
    todo!() // cxx:_mju_solve3
}

/// C: mju_eig3 (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_solve.h:121)
/// Calls: cxx:_mju_mulQuat, cxx:_mju_normalize4, cxx:_mju_quat2Mat
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_eig3(eigval: *mut f64, eigvec: *mut f64, quat: *mut f64, mat: *const f64) -> i32 {
    todo!() // cxx:_mju_eig3
}

/// C: mju_QCQP2 (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_solve.h:126)
/// Calls: cxx-internal:ui_main.c.o:_inside
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_qcqp2(res: *mut f64, Ain: *const f64, bin: *const f64, d: *const f64, r: f64) -> i32 {
    todo!() // cxx:_mju_QCQP2
}

/// C: mju_QCQP3 (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_solve.h:131)
/// Calls: cxx-internal:ui_main.c.o:_inside
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_qcqp3(res: *mut f64, Ain: *const f64, bin: *const f64, d: *const f64, r: f64) -> i32 {
    todo!() // cxx:_mju_QCQP3
}

/// C: mju_QCQP (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_solve.h:136)
/// Calls: cxx:_mju_cholFactor, cxx:_mju_cholSolve, cxx:_mju_copy, cxx:_mju_dot, cxx:_mju_message, cxx:_mju_scl, cxx:_mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_qcqp(res: *mut f64, Ain: *const f64, bin: *const f64, d: *const f64, r: f64, n: i32) -> i32 {
    todo!() // cxx:_mju_QCQP
}

/// C: mju_boxQP (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_solve.h:141)
/// Calls: cxx:_mju_boxQPoption
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_box_qp(res: *mut f64, R: *mut f64, index: *mut i32, H: *const f64, g: *const f64, n: i32, lower: *const f64, upper: *const f64) -> i32 {
    todo!() // cxx:_mju_boxQP
}

/// C: mju_boxQPmalloc (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_solve.h:146)
/// Calls: cxx:_mju_malloc
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_box_q_pmalloc(res: *mut *mut f64, R: *mut *mut f64, index: *mut *mut i32, H: *mut *mut f64, g: *mut *mut f64, n: i32, lower: *mut *mut f64, upper: *mut *mut f64) {
    todo!() // cxx:_mju_boxQPmalloc
}

/// C: mju_boxQPoption (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_solve.h:151)
/// Calls: cxx-internal:engine_util_solve.c.o:_mulSymVec, cxx-internal:engine_util_solve.c.o:_mulVecMatVecSym, cxx:_mju_addTo, cxx:_mju_cholFactor, cxx:_mju_cholSolve, cxx:_mju_copy, cxx:_mju_dot, cxx:_mju_max, cxx:_mju_message, cxx:_mju_min, cxx:_mju_scl, cxx:_mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_box_q_poption(res: *mut f64, R: *mut f64, index: *mut i32, H: *const f64, g: *const f64, n: i32, lower: *const f64, upper: *const f64, maxiter: i32, mingrad: f64, backtrack: f64, minstep: f64, armijo: f64, log: *mut i8, logsz: i32) -> i32 {
    todo!() // cxx:_mju_boxQPoption
}

