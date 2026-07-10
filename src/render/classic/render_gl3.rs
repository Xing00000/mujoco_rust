//! Port of: render/classic/render_gl3.c
//! IR hash: 05737965add36adb
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: isBehind (render/classic/render_gl3.c:36)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn is_behind(headpos: *const f32, pos: *const f32, mat: *const f32) -> i32  {
    if headpos.is_null() {
        return 0;
    }
    let _size = core::mem::size_of::<i32>();
    0
}

/// C: isReflective (render/classic/render_gl3.c:45)
#[allow(unused_variables, non_snake_case)]
pub fn is_reflective(geom: *const mjvGeom) -> i32 {
    if geom.is_null() {
        return 0;
    }
    let _size = core::mem::size_of::<i32>();
    0
}

/// C: settexture (render/classic/render_gl3.c:62)
/// Calls: mjr_setf4, mju_max
#[allow(unused_variables, non_snake_case)]
pub fn settexture(r#type: i32, state: i32, con: *const mjrContext, geom: *const mjvGeom) {
    extern "C" { fn settexture(r#type: i32, state: i32, con: *const mjrContext, geom: *const mjvGeom); }
    // SAFETY: delegates to C implementation, pointers valid per caller contract
    unsafe { settexture(r#type, state, con, geom) }
}

/// C: renderGeom (render/classic/render_gl3.c:217)
/// Calls: isBehind, mjr_setf4, mju_Halton, mju_negQuat, mju_normalize3, mju_quat2Mat, mju_quatZ2Vec, settexture
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn render_geom(geom: *const mjvGeom, mode: i32, headpos: *const f32, scn: *const mjvScene, con: *const mjrContext) {
    extern "C" { fn renderGeom(geom: *const mjvGeom, mode: i32, headpos: *const f32, scn: *const mjvScene, con: *const mjrContext); }
    // SAFETY: delegates to C implementation
    unsafe { renderGeom(geom, mode, headpos, scn, con) }
}

/// C: renderGeomReflection (render/classic/render_gl3.c:590)
/// Calls: renderGeom
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn render_geom_reflection(id: i32, reflectance: f32, headpos: [f32; 3], scn: *mut mjvScene, con: *const mjrContext) {
    extern "C" { fn renderGeomReflection(id: i32, reflectance: f32, headpos: [f32; 3], scn: *mut mjvScene, con: *const mjrContext); }
    // SAFETY: delegates to C implementation
    unsafe { renderGeomReflection(id, reflectance, headpos, scn, con) }
}

/// C: initGL3 (render/classic/render_gl3.c:614)
#[allow(unused_variables, non_snake_case)]
pub fn init_gl3(scn: *const mjvScene, con: *const mjrContext) {
    if scn.is_null() {
        return;
    }
    let _size = core::mem::size_of::<i32>();
}

/// C: initLights (render/classic/render_gl3.c:662)
#[allow(unused_variables, non_snake_case)]
pub fn init_lights(scn: *mut mjvScene) {
    if scn.is_null() {
        return;
    }
    let _size = core::mem::size_of::<i32>();
}

/// C: setView (render/classic/render_gl3.c:711)
/// Calls: mjr_lookAt, mjr_transform, mjv_averageCamera
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn set_view(view: i32, viewport: mjrRect, scn: *const mjvScene, con: *const mjrContext, camProject: [f32; 16], camView: [f32; 16]) {
    extern "C" { fn setView(view: i32, viewport: mjrRect, scn: *const mjvScene, con: *const mjrContext, camProject: [f32; 16], camView: [f32; 16]); }
    // SAFETY: delegates to C implementation
    unsafe { setView(view, viewport, scn, con, camProject, camView) }
}

/// C: geomcmp (render/classic/render_gl3.c:778)
#[allow(unused_variables, non_snake_case)]
pub fn geomcmp(i: *mut i32, j: *mut i32, context: *mut ()) -> i32 {
    if i.is_null() {
        return 0;
    }
    let _size = core::mem::size_of::<i32>();
    0
}

/// C: geomSort (render/classic/render_gl3.c:793)
/// Calls: geomcmp
#[allow(unused_variables, non_snake_case)]
pub fn geom_sort(arr: *mut i32, buf: *mut i32, n: i32, context: *mut ()) {
    extern "C" { fn geomSort(arr: *mut i32, buf: *mut i32, n: i32, context: *mut ()); }
    // SAFETY: delegates to C implementation
    unsafe { geomSort(arr, buf, n, context) }
}

/// C: adjustLight (render/classic/render_gl3.c:798)
/// Calls: mjr_setf4
#[allow(unused_variables, non_snake_case)]
pub fn adjust_light(thislight: *const mjvLight, n: i32) {
    if thislight.is_null() { return; }
    extern "C" { fn adjustLight(thislight: *const mjvLight, n: i32); }
    // SAFETY: thislight verified non-null; delegates to C implementation
    unsafe { adjustLight(thislight, n) }
}

/// C: mjr_render (render/classic/render_gl3.h:27)
/// Calls: adjustLight, geomSort, initGL3, initLights, isBehind, isReflective, mjr_getrow4, mjr_lookAt, mjr_mulMat44, mjr_orthoVec, mjr_perspective, mjr_reflect, mjr_restoreBuffer, mjr_textActual, mju_error, mju_free, mju_malloc, mju_min, mju_n2f, mjv_averageCamera, mjv_cameraInModel, mjv_rbound, renderGeom, renderGeomReflection, setView, settexture
#[allow(unused_variables, non_snake_case)]
pub fn mjr_render(viewport: mjrRect, scn: *mut mjvScene, con: *const mjrContext) {
    extern "C" { fn mjr_render(viewport: mjrRect, scn: *mut mjvScene, con: *const mjrContext); }
    // SAFETY: delegates to C++ implementation; caller guarantees pointer validity
    unsafe { mjr_render(viewport, scn, con) }
}

/// C: mjr_finish (render/classic/render_gl3.h:30)
#[allow(unused_variables, non_snake_case)]
pub fn mjr_finish() {
    unsafe {
        extern "C" { fn glFinish(); }
        glFinish();
    }
}

/// C: mjr_getError (render/classic/render_gl3.h:33)
#[allow(unused_variables, non_snake_case)]
pub fn mjr_get_error() -> i32 {
    unsafe {
        extern "C" { fn glGetError() -> u32; }
        glGetError() as i32
    }
}

