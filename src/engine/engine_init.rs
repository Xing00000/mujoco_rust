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
    // SAFETY: caller guarantees solref (if non-null) points to array of at least 2 elements,
    //         solimp (if non-null) points to array of at least 5 elements.
    unsafe {
        if !solref.is_null() {
            *solref.add(0) = 0.02;   // timeconst
            *solref.add(1) = 1.0;    // dampratio
        }

        if !solimp.is_null() {
            *solimp.add(0) = 0.9;    // dmin
            *solimp.add(1) = 0.95;   // dmax
            *solimp.add(2) = 0.001;  // width
            *solimp.add(3) = 0.5;    // midpoint
            *solimp.add(4) = 2.0;    // power
        }
    }
}

/// C: mj_defaultOption (engine/engine_init.c:51)
/// Calls: mj_defaultSolRefImp
#[allow(unused_variables, non_snake_case)]
pub fn mj_default_option(opt: *mut mjOption) {
    extern "C" { fn mj_defaultOption_impl(opt: *mut mjOption); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_defaultOption_impl(opt) }
}

/// C: setf4 (engine/engine_init.c:124)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn setf4(rgba: *mut f32, r: f32, g: f32, b: f32, a: f32) {
    // SAFETY: caller guarantees rgba points to a valid array of at least 4 f32 elements.
    unsafe {
        *rgba.add(0) = r;
        *rgba.add(1) = g;
        *rgba.add(2) = b;
        *rgba.add(3) = a;
    }
}

/// C: mj_defaultVisual (engine/engine_init.c:133)
/// Calls: setf4
#[allow(unused_variables, non_snake_case)]
pub fn mj_default_visual(vis: *mut mjVisual) {
    extern "C" { fn mj_defaultVisual_impl(vis: *mut mjVisual); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_defaultVisual_impl(vis) }
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
    extern "C" { fn mj_defaultStatistic_impl(stat: *mut mjStatistic); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_defaultStatistic_impl(stat) }
}

