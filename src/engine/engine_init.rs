//! Port of: engine/engine_init.c
//! IR hash: 9614bd3d92e7766b
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
    if !solref.is_null() {
        // SAFETY: solref points to at least 2 f64 (caller contract)
        unsafe {
            *solref.add(0) = 0.02;
            *solref.add(1) = 1.0;
        }
    }
    if !solimp.is_null() {
        // SAFETY: solimp points to at least 5 f64 (caller contract)
        unsafe {
            *solimp.add(0) = 0.9;
            *solimp.add(1) = 0.95;
            *solimp.add(2) = 0.001;
            *solimp.add(3) = 0.5;
            *solimp.add(4) = 2.0;
        }
    }
}

/// C: mj_defaultOption (engine/engine_init.c:51)
/// Calls: mj_defaultSolRefImp
#[allow(unused_variables, non_snake_case)]
pub fn mj_default_option(opt: *mut mjOption) {
    todo!() // mj_defaultOption
}

/// C: setf4 (engine/engine_init.c:124)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn setf4(rgba: *mut f32, r: f32, g: f32, b: f32, a: f32) {
    todo!() // setf4
}

/// C: mj_defaultVisual (engine/engine_init.c:133)
/// Calls: setf4
#[allow(unused_variables, non_snake_case)]
pub fn mj_default_visual(vis: *mut mjVisual) {
    todo!() // mj_defaultVisual
}

/// C: mj_defaultLROpt (engine/engine_init.c:234)
#[allow(unused_variables, non_snake_case)]
pub fn mj_default_lr_opt(opt: *mut mjLROpt) {
    // SAFETY: opt is a valid pointer to mjLROpt (caller contract)
    unsafe {
        (*opt).mode = 1; // mjLRMODE_MUSCLE
        (*opt).useexisting = 1;
        (*opt).uselimit = 0;

        (*opt).accel = 20.0;
        (*opt).maxforce = 0.0;
        (*opt).timeconst = 1.0;
        (*opt).timestep = 0.01;
        (*opt).inttotal = 10.0;
        (*opt).interval = 2.0;
        (*opt).tolrange = 0.05;
    }
}

/// C: mj_defaultStatistic (engine/engine_init.h:30)
/// Calls: mju_zero3
#[allow(unused_variables, non_snake_case)]
pub fn mj_default_statistic(stat: *mut mjStatistic) {
    todo!() // mj_defaultStatistic
}

