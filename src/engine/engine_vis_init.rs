//! Port of: engine/engine_vis_init.h
//! IR hash: 05737965add36adb
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mjv_defaultScene (engine/engine_vis_init.h:34)
#[allow(unused_variables, non_snake_case)]
pub fn mjv_default_scene(scn: *mut mjvScene) {
    extern "C" { fn mjv_defaultScene_impl(scn: *mut mjvScene); }
    // SAFETY: delegates to C implementation
    unsafe { mjv_defaultScene_impl(scn) }
}

/// C: mjv_makeScene (engine/engine_vis_init.h:37)
/// Calls: mju_copyInt, mju_error, mju_malloc, mju_message, mjv_freeScene
#[allow(unused_variables, non_snake_case)]
pub fn mjv_make_scene(m: *const mjModel, scn: *mut mjvScene, maxgeom: i32) {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, scn : * mut mjvScene, maxgeom : i32)
    // Previous return: ()
    todo ! ()
}

/// C: mjv_freeScene (engine/engine_vis_init.h:40)
/// Calls: mju_free, mjv_defaultScene
#[allow(unused_variables, non_snake_case)]
pub fn mjv_free_scene(scn: *mut mjvScene) {
    // WARNING: signature changed — verify body
    // Previous params: (scn : * mut mjvScene)
    // Previous return: ()
    todo ! ()
}

/// C: mjv_defaultOption (engine/engine_vis_init.h:43)
#[allow(unused_variables, non_snake_case)]
pub fn mjv_default_option(vopt: *mut mjvOption) {
    extern "C" { fn mjv_defaultOption_impl(vopt: *mut mjvOption); }
    // SAFETY: delegates to C implementation
    unsafe { mjv_defaultOption_impl(vopt) }
}

/// C: mjv_defaultFreeCamera (engine/engine_vis_init.h:46)
#[allow(unused_variables, non_snake_case)]
pub fn mjv_default_free_camera(m: *const mjModel, cam: *mut mjvCamera) {
    extern "C" { fn mjv_defaultFreeCamera_impl(m: *const mjModel, cam: *mut mjvCamera); }
    // SAFETY: delegates to C implementation
    unsafe { mjv_defaultFreeCamera_impl(m, cam) }
}

/// C: mjv_defaultCamera (engine/engine_vis_init.h:49)
#[allow(unused_variables, non_snake_case)]
pub fn mjv_default_camera(cam: *mut mjvCamera) {
    extern "C" { fn mjv_defaultCamera_impl(cam: *mut mjvCamera); }
    // SAFETY: delegates to C implementation
    unsafe { mjv_defaultCamera_impl(cam) }
}

/// C: mjv_defaultPerturb (engine/engine_vis_init.h:52)
#[allow(unused_variables, non_snake_case)]
pub fn mjv_default_perturb(pert: *mut mjvPerturb) {
    extern "C" { fn mjv_defaultPerturb_impl(pert: *mut mjvPerturb); }
    // SAFETY: delegates to C implementation
    unsafe { mjv_defaultPerturb_impl(pert) }
}

/// C: mjv_defaultFigure (engine/engine_vis_init.h:55)
/// Calls: mju_Halton, mju_strncpy
#[allow(unused_variables, non_snake_case)]
pub fn mjv_default_figure(fig: *mut mjvFigure) {
    extern "C" { fn mjv_defaultFigure_impl(fig: *mut mjvFigure); }
    // SAFETY: delegates to C implementation
    unsafe { mjv_defaultFigure_impl(fig) }
}

/// C: mjv_rbound (engine/engine_vis_init.h:58)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjv_rbound(geom: *const mjvGeom) -> f32 {
    extern "C" { fn mjv_rbound_impl(geom: *const mjvGeom) -> f32; }
    // SAFETY: delegates to C implementation
    unsafe { mjv_rbound_impl(geom) }
}

