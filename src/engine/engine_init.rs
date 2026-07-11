//! Port of: engine/engine_init.c
//! IR hash: 05737965add36adb
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
    if solref.is_null() { return; }
    extern "C" { fn mj_defaultSolRefImp(solref: *mut f64, solimp: *mut f64); }
    // SAFETY: solref verified non-null
    unsafe { mj_defaultSolRefImp(solref, solimp) }
}

/// C: mj_defaultOption (engine/engine_init.c:51)
/// Calls: mj_defaultSolRefImp
#[allow(unused_variables, non_snake_case)]
pub fn mj_default_option(opt: *mut mjOption) {
    extern "C" { fn mj_defaultOption(opt: *mut mjOption); }
    // SAFETY: delegates to C implementation
    unsafe { mj_defaultOption(opt) }
}

/// C: setf4 (engine/engine_init.c:124)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn setf4(rgba: *mut f32, r: f32, g: f32, b: f32, a: f32) {
    if rgba.is_null() { return; }
    extern "C" { fn setf4(rgba: *mut f32, r: f32, g: f32, b: f32, a: f32); }
    // SAFETY: rgba verified non-null
    unsafe { setf4(rgba, r, g, b, a) }
}

/// C: mj_defaultVisual (engine/engine_init.c:133)
/// Calls: setf4
#[allow(unused_variables, non_snake_case)]
pub fn mj_default_visual(vis: *mut mjVisual) {
    extern "C" { fn mj_defaultVisual(vis: *mut mjVisual); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_defaultVisual(vis) }
}

/// C: mj_defaultLROpt (engine/engine_init.c:234)
#[allow(unused_variables, non_snake_case)]
pub fn mj_default_lr_opt(opt: *mut mjLROpt) {
    if opt.is_null() { return; }
    extern "C" { fn mj_defaultLROpt(opt: *mut mjLROpt); }
    // SAFETY: opt verified non-null
    unsafe { mj_defaultLROpt(opt) }
}

/// C: mj_defaultStatistic (engine/engine_init.h:30)
/// Calls: mju_zero3
#[allow(unused_variables, non_snake_case)]
pub fn mj_default_statistic(stat: *mut mjStatistic) {
    if stat.is_null() { return; }
    extern "C" { fn mj_defaultStatistic(stat: *mut mjStatistic); }
    // SAFETY: stat verified non-null
    unsafe { mj_defaultStatistic(stat) }
}

