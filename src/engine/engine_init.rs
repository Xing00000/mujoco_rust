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
    extern "C" { fn mj_defaultSolRefImp_impl(solref: *mut f64, solimp: *mut f64); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_defaultSolRefImp_impl(solref, solimp) }
}

/// C: mj_defaultOption (engine/engine_init.c:51)
/// Calls: mj_defaultSolRefImp
#[allow(unused_variables, non_snake_case)]
pub fn mj_default_option(opt: *mut mjOption) {
    // WARNING: signature changed — verify body
    // Previous params: (opt : * mut mjOption)
    // Previous return: ()
    todo ! ()
}

/// C: setf4 (engine/engine_init.c:124)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn setf4(rgba: *mut f32, r: f32, g: f32, b: f32, a: f32) {
    extern "C" { fn setf4_impl(rgba: *mut f32, r: f32, g: f32, b: f32, a: f32); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { setf4_impl(rgba, r, g, b, a) }
}

/// C: mj_defaultVisual (engine/engine_init.c:133)
/// Calls: setf4
#[allow(unused_variables, non_snake_case)]
pub fn mj_default_visual(vis: *mut mjVisual) {
    // WARNING: signature changed — verify body
    // Previous params: (vis : * mut mjVisual)
    // Previous return: ()
    todo ! ()
}

/// C: mj_defaultLROpt (engine/engine_init.c:234)
#[allow(unused_variables, non_snake_case)]
pub fn mj_default_lr_opt(opt: *mut mjLROpt) {
    extern "C" { fn mj_defaultLROpt_impl(opt: *mut mjLROpt); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_defaultLROpt_impl(opt) }
}

/// C: mj_defaultStatistic (engine/engine_init.h:30)
/// Calls: mju_zero3
#[allow(unused_variables, non_snake_case)]
pub fn mj_default_statistic(stat: *mut mjStatistic) {
    // WARNING: signature changed — verify body
    // Previous params: (stat : * mut mjStatistic)
    // Previous return: ()
    todo ! ()
}

