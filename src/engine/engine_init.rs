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
    // SAFETY: opt is a valid pointer per caller contract. write_bytes zeros padding.
    unsafe {
        core::ptr::write_bytes(opt, 0, 1);

        // timing parameters
        (*opt).timestep           = 0.002;

        // solver parameters
        (*opt).impratio           = 1.0;
        (*opt).tolerance          = 1e-8;
        (*opt).ls_tolerance       = 0.01;
        (*opt).noslip_tolerance   = 1e-6;
        (*opt).ccd_tolerance      = 1e-6;

        // sleep settings
        (*opt).sleep_tolerance    = 1e-4;

        // physical constants
        (*opt).gravity[0]         = 0.0;
        (*opt).gravity[1]         = 0.0;
        (*opt).gravity[2]         = -9.81;
        (*opt).wind[0]            = 0.0;
        (*opt).wind[1]            = 0.0;
        (*opt).wind[2]            = 0.0;
        (*opt).magnetic[0]        = 0.0;
        (*opt).magnetic[1]        = -0.5;
        (*opt).magnetic[2]        = 0.0;
        (*opt).density            = 0.0;
        (*opt).viscosity          = 0.0;

        // solver overrides
        (*opt).o_margin           = 0.0;
        mj_default_sol_ref_imp((*opt).o_solref.as_mut_ptr(), (*opt).o_solimp.as_mut_ptr());
        (*opt).o_friction[0] = 1.0;
        (*opt).o_friction[1] = 1.0;
        (*opt).o_friction[2] = 0.005;
        (*opt).o_friction[3] = 0.0001;
        (*opt).o_friction[4] = 0.0001;

        // discrete options
        (*opt).integrator         = 0;   // mjINT_EULER
        (*opt).cone               = 0;   // mjCONE_PYRAMIDAL
        (*opt).jacobian           = 2;   // mjJAC_AUTO
        (*opt).solver             = 2;   // mjSOL_NEWTON
        (*opt).iterations         = 100;
        (*opt).ls_iterations      = 50;
        (*opt).noslip_iterations  = 0;
        (*opt).ccd_iterations     = 35;
        (*opt).disableflags       = 0;
        (*opt).enableflags        = 0;
        (*opt).disableactuator    = 0;

        // sdf collisions
        (*opt).sdf_initpoints     = 40;
        (*opt).sdf_iterations     = 10;
    }
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
    extern "C" { fn mj_defaultVisual(vis: *mut mjVisual); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_defaultVisual(vis) }
}

/// C: mj_defaultLROpt (engine/engine_init.c:234)
#[allow(unused_variables, non_snake_case)]
pub fn mj_default_lr_opt(opt: *mut mjLROpt) {
    // SAFETY: opt is a valid pointer per caller contract.
    unsafe {
        (*opt).mode           = 1;     // mjLRMODE_MUSCLE
        (*opt).useexisting    = 1;
        (*opt).uselimit       = 0;

        (*opt).accel          = 20.0;
        (*opt).maxforce       = 0.0;
        (*opt).timeconst      = 1.0;
        (*opt).timestep       = 0.01;
        (*opt).inttotal       = 10.0;
        (*opt).interval       = 2.0;
        (*opt).tolrange       = 0.05;
    }
}

/// C: mj_defaultStatistic (engine/engine_init.h:30)
/// Calls: mju_zero3
#[allow(unused_variables, non_snake_case)]
pub fn mj_default_statistic(stat: *mut mjStatistic) {
    // SAFETY: stat is a valid pointer per caller contract.
    unsafe {
        crate::engine::engine_util_blas::mju_zero3((*stat).center.as_mut_ptr());
        (*stat).extent = 2.0;
        (*stat).meaninertia = 1.0;
        (*stat).meanmass = 1.0;
        (*stat).meansize = 0.2;
    }
}

