//! Port of: engine/engine_init.c
//! IR hash: fd5288ad7242f001
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mj_defaultSolRefImp (engine/engine_init.c:32)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_default_sol_ref_imp(solref: *mut f64, solimp: *mut f64) {
    todo!() // cxx:_mj_defaultSolRefImp
}

/// C: mj_defaultOption (engine/engine_init.c:51)
/// Calls: cxx:_mj_defaultSolRefImp
#[allow(unused_variables, non_snake_case)]
pub fn mj_default_option(opt: *mut mjOption) {
    todo!() // cxx:_mj_defaultOption
}

/// C: mj_defaultStatistic (engine/engine_init.c:112)
/// Calls: cxx:_mju_zero3
#[allow(unused_variables, non_snake_case)]
pub fn mj_default_statistic(stat: *mut mjStatistic) {
    todo!() // cxx:_mj_defaultStatistic
}

/// C: setf4 (engine/engine_init.c:124)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn setf4(rgba: *mut f32, r: f32, g: f32, b: f32, a: f32) {
    // SAFETY: caller guarantees rgba points to at least 4 contiguous floats
    unsafe {
        *rgba.add(0) = r;
        *rgba.add(1) = g;
        *rgba.add(2) = b;
        *rgba.add(3) = a;
    }
}

/// C: mj_defaultVisual (engine/engine_init.c:133)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_init.c:_setf4
#[allow(unused_variables, non_snake_case)]
pub fn mj_default_visual(vis: *mut mjVisual) {
    todo!() // cxx:_mj_defaultVisual
}

/// C: mj_defaultLROpt (engine/engine_init.c:234)
#[allow(unused_variables, non_snake_case)]
pub fn mj_default_lr_opt(opt: *mut mjLROpt) {
    todo!() // cxx:_mj_defaultLROpt
}

