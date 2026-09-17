//! Port of: engine/engine_vis_init.c
//! IR hash: fd5288ad7242f001
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mjv_makeScene (engine/engine_vis_init.c:126)
/// Calls: cxx:_mju_copyInt, cxx:_mju_error, cxx:_mju_malloc, cxx:_mju_message, cxx:_mjv_freeScene
#[allow(unused_variables, non_snake_case)]
pub fn mjv_make_scene(m: *const mjModel, scn: *mut mjvScene, maxgeom: i32) {
    todo!() // cxx:_mjv_makeScene
}

/// C: mjv_freeScene (engine/engine_vis_init.c:280)
/// Calls: cxx:_mju_free, cxx:_mjv_defaultScene
#[allow(unused_variables, non_snake_case)]
pub fn mjv_free_scene(scn: *mut mjvScene) {
    todo!() // cxx:_mjv_freeScene
}

/// C: mjv_defaultScene (engine/engine_vis_init.c:310)
#[allow(unused_variables, non_snake_case)]
pub fn mjv_default_scene(scn: *mut mjvScene) {
    todo!() // cxx:_mjv_defaultScene
}

/// C: mjv_defaultOption (engine/engine_vis_init.c:316)
#[allow(unused_variables, non_snake_case)]
pub fn mjv_default_option(vopt: *mut mjvOption) {
    todo!() // cxx:_mjv_defaultOption
}

/// C: mjv_defaultCamera (engine/engine_vis_init.c:341)
#[allow(unused_variables, non_snake_case)]
pub fn mjv_default_camera(cam: *mut mjvCamera) {
    todo!() // cxx:_mjv_defaultCamera
}

/// C: mjv_defaultFreeCamera (engine/engine_vis_init.c:354)
#[allow(unused_variables, non_snake_case)]
pub fn mjv_default_free_camera(m: *const mjModel, cam: *mut mjvCamera) {
    todo!() // cxx:_mjv_defaultFreeCamera
}

/// C: mjv_defaultPerturb (engine/engine_vis_init.c:371)
#[allow(unused_variables, non_snake_case)]
pub fn mjv_default_perturb(pert: *mut mjvPerturb) {
    todo!() // cxx:_mjv_defaultPerturb
}

/// C: mjv_defaultFigure (engine/engine_vis_init.c:396)
/// Calls: cxx:_mju_Halton, cxx:_mju_strncpy
#[allow(unused_variables, non_snake_case)]
pub fn mjv_default_figure(fig: *mut mjvFigure) {
    todo!() // cxx:_mjv_defaultFigure
}

/// C: mjv_rbound (engine/engine_vis_init.c:451)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjv_rbound(geom: *const mjvGeom) -> f32 {
    const mjOBJ_GEOM: i32 = 5;
    const mjGEOM_SPHERE: i32 = 2;
    const mjGEOM_CAPSULE: i32 = 3;
    const mjGEOM_CYLINDER: i32 = 5;
    const mjGEOM_BOX: i32 = 6;

    // SAFETY: caller guarantees geom is a valid pointer to mjvGeom
    unsafe {
        // model geom: return
        if (*geom).objtype == mjOBJ_GEOM {
            return (*geom).modelrbound;
        }

        // compute rbound according to type
        let s = &(*geom).size;
        match (*geom).r#type {
            mjGEOM_SPHERE => s[0],

            mjGEOM_CAPSULE => s[0] + s[2],

            mjGEOM_CYLINDER => {
                (s[0] * s[0] + s[2] * s[2]).sqrt()
            }

            mjGEOM_BOX => {
                (s[0] * s[0] + s[1] * s[1] + s[2] * s[2]).sqrt()
            }

            _ => {
                // not accurate for arrows, but they are not transparent
                let max_s1_s2 = if s[1] > s[2] { s[1] } else { s[2] };
                if s[0] > max_s1_s2 { s[0] } else { max_s1_s2 }
            }
        }
    }
}

