//! Port of: Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_init.c
//! IR hash: 1fa44043fe383ce9
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mj_defaultSolRefImp (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_init.c:32)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_default_sol_ref_imp(solref: *mut f64, solimp: *mut f64) {
    todo!() // cxx:_mj_defaultSolRefImp
}

/// C: mj_defaultOption (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_init.c:51)
/// Calls: cxx:_mj_defaultSolRefImp
#[allow(unused_variables, non_snake_case)]
pub fn mj_default_option(opt: *mut mjOption) {
    todo!() // cxx:_mj_defaultOption
}

/// C: setf4 (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_init.c:124)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn setf4(rgba: *mut f32, r: f32, g: f32, b: f32, a: f32) {
    todo!() // cxx-internal:engine_init.c.o:_setf4
}

/// C: mj_defaultVisual (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_init.c:133)
/// Calls: cxx-internal:engine_init.c.o:_setf4
#[allow(unused_variables, non_snake_case)]
pub fn mj_default_visual(vis: *mut mjVisual) {
    todo!() // cxx:_mj_defaultVisual
}

/// C: mj_defaultLROpt (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_init.c:234)
#[allow(unused_variables, non_snake_case)]
pub fn mj_default_lr_opt(opt: *mut mjLROpt) {
    todo!() // cxx:_mj_defaultLROpt
}

/// C: mj_defaultStatistic (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_init.h:30)
/// Calls: cxx:_mju_zero3
#[allow(unused_variables, non_snake_case)]
pub fn mj_default_statistic(stat: *mut mjStatistic) {
    todo!() // cxx:_mj_defaultStatistic
}

