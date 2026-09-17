//! Port of: Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_blas.h
//! IR hash: 1fa44043fe383ce9
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mju_zero3 (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_blas.h:68)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_zero3(res: *mut f64) {
    todo!() // cxx:_mju_zero3
}

/// C: mju_equal3 (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_blas.h:71)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_equal3(vec1: *const f64, vec2: *const f64) -> i32 {
    todo!() // cxx:_mju_equal3
}

/// C: mju_copy3 (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_blas.h:74)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_copy3(res: *mut f64, vec: *const f64) {
    todo!() // cxx:_mju_copy3
}

/// C: mju_copy9 (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_blas.h:77)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_copy9(res: *mut f64, mat: *const f64) {
    todo!() // cxx:_mju_copy9
}

/// C: mju_scl3 (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_blas.h:80)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_scl3(res: *mut f64, vec: *const f64, scl: f64) {
    todo!() // cxx:_mju_scl3
}

/// C: mju_add3 (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_blas.h:83)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_add3(res: *mut f64, vec1: *const f64, vec2: *const f64) {
    todo!() // cxx:_mju_add3
}

/// C: mju_sub3 (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_blas.h:86)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_sub3(res: *mut f64, vec1: *const f64, vec2: *const f64) {
    todo!() // cxx:_mju_sub3
}

/// C: mju_addTo3 (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_blas.h:89)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_add_to3(res: *mut f64, vec: *const f64) {
    todo!() // cxx:_mju_addTo3
}

/// C: mju_subFrom3 (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_blas.h:92)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_sub_from3(res: *mut f64, vec: *const f64) {
    todo!() // cxx:_mju_subFrom3
}

/// C: mju_addToScl3 (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_blas.h:95)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_add_to_scl3(res: *mut f64, vec: *const f64, scl: f64) {
    todo!() // cxx:_mju_addToScl3
}

/// C: mju_addScl3 (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_blas.h:98)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_add_scl3(res: *mut f64, vec1: *const f64, vec2: *const f64, scl: f64) {
    todo!() // cxx:_mju_addScl3
}

/// C: mju_normalize3 (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_blas.h:101)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_normalize3(vec: *mut f64) -> f64 {
    todo!() // cxx:_mju_normalize3
}

/// C: mju_norm3 (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_blas.h:104)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_norm3(vec: *const f64) -> f64 {
    todo!() // cxx:_mju_norm3
}

/// C: mju_dot3 (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_blas.h:107)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_dot3(vec1: *const f64, vec2: *const f64) -> f64 {
    todo!() // cxx:_mju_dot3
}

/// C: mju_dist3 (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_blas.h:110)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_dist3(pos1: *const f64, pos2: *const f64) -> f64 {
    todo!() // cxx:_mju_dist3
}

/// C: mju_mulMatVec3 (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_blas.h:113)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_mul_mat_vec3(res: *mut f64, mat: *const f64, vec: *const f64) {
    todo!() // cxx:_mju_mulMatVec3
}

/// C: mju_mulMatTVec3 (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_blas.h:116)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_mul_mat_t_vec3(res: *mut f64, mat: *const f64, vec: *const f64) {
    todo!() // cxx:_mju_mulMatTVec3
}

/// C: mju_mulMatMat3 (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_blas.h:119)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_mul_mat_mat3(res: *mut f64, mat1: *const f64, mat2: *const f64) {
    todo!() // cxx:_mju_mulMatMat3
}

/// C: mju_mulMatTMat3 (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_blas.h:122)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_mul_mat_t_mat3(res: *mut f64, mat1: *const f64, mat2: *const f64) {
    todo!() // cxx:_mju_mulMatTMat3
}

/// C: mju_mulMatMatT3 (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_blas.h:125)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_mul_mat_mat_t3(res: *mut f64, mat1: *const f64, mat2: *const f64) {
    todo!() // cxx:_mju_mulMatMatT3
}

/// C: mju_zero4 (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_blas.h:130)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_zero4(res: *mut f64) {
    todo!() // cxx:_mju_zero4
}

/// C: mju_unit4 (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_blas.h:133)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_unit4(res: *mut f64) {
    todo!() // cxx:_mju_unit4
}

/// C: mju_copy4 (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_blas.h:136)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_copy4(res: *mut f64, data: *const f64) {
    todo!() // cxx:_mju_copy4
}

/// C: mju_normalize4 (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_blas.h:139)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_normalize4(vec: *mut f64) -> f64 {
    todo!() // cxx:_mju_normalize4
}

/// C: mju_zero (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_blas.h:145)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_zero(res: *mut f64, n: i32) {
    todo!() // cxx:_mju_zero
}

/// C: mju_zeroInd (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_blas.h:148)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_zero_ind(res: *mut f64, n: i32, ind: *const i32) {
    todo!() // cxx:_mju_zeroInd
}

/// C: mju_fill (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_blas.h:151)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_fill(res: *mut f64, val: f64, n: i32) {
    todo!() // cxx:_mju_fill
}

/// C: mju_copy (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_blas.h:154)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_copy(res: *mut f64, vec: *const f64, n: i32) {
    todo!() // cxx:_mju_copy
}

/// C: mju_copyInd (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_blas.h:157)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_copy_ind(res: *mut f64, vec: *const f64, ind: *const i32, n: i32) {
    todo!() // cxx:_mju_copyInd
}

/// C: mju_sum (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_blas.h:160)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_sum(vec: *const f64, n: i32) -> f64 {
    todo!() // cxx:_mju_sum
}

/// C: mju_L1 (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_blas.h:163)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_l1(vec: *const f64, n: i32) -> f64 {
    todo!() // cxx:_mju_L1
}

/// C: mju_scl (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_blas.h:166)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_scl(res: *mut f64, vec: *const f64, scl: f64, n: i32) {
    todo!() // cxx:_mju_scl
}

/// C: mju_add (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_blas.h:169)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_add(res: *mut f64, vec1: *const f64, vec2: *const f64, n: i32) {
    todo!() // cxx:_mju_add
}

/// C: mju_addInd (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_blas.h:172)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_add_ind(res: *mut f64, vec1: *const f64, vec2: *const f64, ind: *const i32, n: i32) {
    todo!() // cxx:_mju_addInd
}

/// C: mju_sub (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_blas.h:175)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_sub(res: *mut f64, vec1: *const f64, vec2: *const f64, n: i32) {
    todo!() // cxx:_mju_sub
}

/// C: mju_subInd (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_blas.h:178)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_sub_ind(res: *mut f64, vec1: *const f64, vec2: *const f64, ind: *const i32, n: i32) {
    todo!() // cxx:_mju_subInd
}

/// C: mju_addTo (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_blas.h:181)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_add_to(res: *mut f64, vec: *const f64, n: i32) {
    todo!() // cxx:_mju_addTo
}

/// C: mju_addToInd (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_blas.h:184)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_add_to_ind(res: *mut f64, vec: *const f64, ind: *const i32, n: i32) {
    todo!() // cxx:_mju_addToInd
}

/// C: mju_subFrom (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_blas.h:187)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_sub_from(res: *mut f64, vec: *const f64, n: i32) {
    todo!() // cxx:_mju_subFrom
}

/// C: mju_addToScl (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_blas.h:190)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_add_to_scl(res: *mut f64, vec: *const f64, scl: f64, n: i32) {
    todo!() // cxx:_mju_addToScl
}

/// C: mju_addToSclInd (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_blas.h:193)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_add_to_scl_ind(res: *mut f64, vec: *const f64, ind: *const i32, scl: f64, n: i32) {
    todo!() // cxx:_mju_addToSclInd
}

/// C: mju_addScl (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_blas.h:196)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_add_scl(res: *mut f64, vec1: *const f64, vec2: *const f64, scl: f64, n: i32) {
    todo!() // cxx:_mju_addScl
}

/// C: mju_normalize (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_blas.h:199)
/// Calls: cxx:_mju_dot, cxx:_mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_normalize(res: *mut f64, n: i32) -> f64 {
    todo!() // cxx:_mju_normalize
}

/// C: mju_norm (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_blas.h:202)
/// Calls: cxx:_mju_dot
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_norm(res: *const f64, n: i32) -> f64 {
    todo!() // cxx:_mju_norm
}

/// C: mju_dot (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_blas.h:205)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_dot(vec1: *const f64, vec2: *const f64, n: i32) -> f64 {
    todo!() // cxx:_mju_dot
}

/// C: mju_dotInd (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_blas.h:208)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_dot_ind(vec1: *const f64, vec2: *const f64, ind: *const i32, n: i32) -> f64 {
    todo!() // cxx:_mju_dotInd
}

/// C: mju_mulMatVec (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_blas.h:213)
/// Calls: cxx:_mju_dot
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_mul_mat_vec(res: *mut f64, mat: *const f64, vec: *const f64, nr: i32, nc: i32) {
    todo!() // cxx:_mju_mulMatVec
}

/// C: mju_mulMatTVec (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_blas.h:216)
/// Calls: cxx:_mju_addToScl, cxx:_mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_mul_mat_t_vec(res: *mut f64, mat: *const f64, vec: *const f64, nr: i32, nc: i32) {
    todo!() // cxx:_mju_mulMatTVec
}

/// C: mju_mulVecMatVec (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_blas.h:219)
/// Calls: cxx:_mju_dot
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_mul_vec_mat_vec(vec1: *const f64, mat: *const f64, vec2: *const f64, n: i32) -> f64 {
    todo!() // cxx:_mju_mulVecMatVec
}

/// C: mju_transpose (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_blas.h:225)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_transpose(res: *mut f64, mat: *const f64, nr: i32, nc: i32) {
    todo!() // cxx:_mju_transpose
}

/// C: mju_symmetrize (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_blas.h:228)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_symmetrize(res: *mut f64, mat: *const f64, n: i32) {
    todo!() // cxx:_mju_symmetrize
}

/// C: mju_eye (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_blas.h:231)
/// Calls: cxx:_mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_eye(mat: *mut f64, n: i32) {
    todo!() // cxx:_mju_eye
}

/// C: mju_copyRows (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_blas.h:234)
/// Calls: cxx:_mju_copy
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_copy_rows(res: *mut f64, mat: *const f64, ind: *const i32, n: i32, nc: i32) {
    todo!() // cxx:_mju_copyRows
}

/// C: mju_mulMatMat (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_blas.h:239)
/// Calls: cxx:_mju_addToScl, cxx:_mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_mul_mat_mat(res: *mut f64, mat1: *const f64, mat2: *const f64, r1: i32, c1: i32, c2: i32) {
    todo!() // cxx:_mju_mulMatMat
}

/// C: mju_mulMatMatT (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_blas.h:243)
/// Calls: cxx:_mju_dot
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_mul_mat_mat_t(res: *mut f64, mat1: *const f64, mat2: *const f64, r1: i32, c1: i32, r2: i32) {
    todo!() // cxx:_mju_mulMatMatT
}

/// C: mju_mulMatTMat (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_blas.h:247)
/// Calls: cxx:_mju_addToScl, cxx:_mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_mul_mat_t_mat(res: *mut f64, mat1: *const f64, mat2: *const f64, r1: i32, c1: i32, c2: i32) {
    todo!() // cxx:_mju_mulMatTMat
}

/// C: mju_sqrMatTD_impl (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_blas.h:251)
/// Calls: cxx:_mju_addToScl, cxx:_mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_sqr_mat_td_impl(res: *mut f64, mat: *const f64, diag: *const f64, nr: i32, nc: i32, flg_upper: i32) {
    todo!() // cxx:_mju_sqrMatTD_impl
}

/// C: mju_sqrMatTD (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_util_blas.h:255)
/// Calls: cxx:_mju_sqrMatTD_impl
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mju_sqr_mat_td(res: *mut f64, mat: *const f64, diag: *const f64, nr: i32, nc: i32) {
    todo!() // cxx:_mju_sqrMatTD
}

