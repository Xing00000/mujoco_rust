//! Port of: Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_gl3.c
//! IR hash: 1fa44043fe383ce9
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: isBehind (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_gl3.c:36)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn is_behind(headpos: *const f32, pos: *const f32, mat: *const f32) -> i32 {
    todo!() // cxx-internal:render_gl3.c.o:_isBehind
}

/// C: isReflective (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_gl3.c:45)
#[allow(unused_variables, non_snake_case)]
pub fn is_reflective(geom: *const mjvGeom) -> i32 {
    todo!() // cxx-internal:render_gl3.c.o:_isReflective
}

/// C: settexture (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_gl3.c:62)
/// Calls: cxx:_mjr_setf4, cxx:_mju_max
#[allow(unused_variables, non_snake_case)]
pub fn settexture(r#type: i32, state: i32, con: *const mjrContext, geom: *const mjvGeom) {
    todo!() // cxx-internal:render_gl3.c.o:_settexture
}

/// C: renderGeom (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_gl3.c:217)
/// Calls: cxx-internal:render_gl3.c.o:_isBehind, cxx-internal:render_gl3.c.o:_settexture, cxx:_mjr_setf4, cxx:_mju_Halton, cxx:_mju_negQuat, cxx:_mju_normalize3, cxx:_mju_quat2Mat, cxx:_mju_quatZ2Vec
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn render_geom(geom: *const mjvGeom, mode: i32, headpos: *const f32, scn: *const mjvScene, con: *const mjrContext) {
    todo!() // cxx-internal:render_gl3.c.o:_renderGeom
}

/// C: renderGeomReflection (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_gl3.c:590)
/// Calls: cxx-internal:render_gl3.c.o:_renderGeom
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn render_geom_reflection(id: i32, reflectance: f32, headpos: *mut f32, scn: *mut mjvScene, con: *const mjrContext) {
    todo!() // cxx:_renderGeomReflection
}

/// C: initGL3 (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_gl3.c:614)
/// Calls: cxx-internal:user_objects.cc.o:__ZNK12_GLOBAL__N_18PNGImage5WidthEv, cxx:__ZNK7mjCMesh4FaceEv
#[allow(unused_variables, non_snake_case)]
pub fn init_gl3(scn: *const mjvScene, con: *const mjrContext) {
    todo!() // cxx-internal:render_gl3.c.o:_initGL3
}

/// C: initLights (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_gl3.c:662)
#[allow(unused_variables, non_snake_case)]
pub fn init_lights(scn: *mut mjvScene) {
    todo!() // cxx-internal:render_gl3.c.o:_initLights
}

/// C: setView (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_gl3.c:711)
/// Calls: cxx:_mjr_lookAt, cxx:_mjr_transform, cxx:_mjv_averageCamera
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn set_view(view: i32, viewport: mjrRect, scn: *const mjvScene, con: *const mjrContext, camProject: *mut f32, camView: *mut f32) {
    todo!() // cxx-internal:render_gl3.c.o:_setView
}

/// C: geomcmp (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_gl3.c:778)
#[allow(unused_variables, non_snake_case)]
pub fn geomcmp(i: *mut i32, j: *mut i32, context: *mut ()) -> i32 {
    todo!() // cxx-internal:render_gl3.c.o:_geomcmp
}

/// C: geomSort (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_gl3.c:793)
/// Calls: cxx-internal:render_gl3.c.o:_geomcmp
#[allow(unused_variables, non_snake_case)]
pub fn geom_sort(arr: *mut i32, buf: *mut i32, n: i32, context: *mut ()) {
    todo!() // cxx-internal:render_gl3.c.o:_geomSort
}

/// C: adjustLight (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_gl3.c:798)
/// Calls: cxx:_mjr_setf4
#[allow(unused_variables, non_snake_case)]
pub fn adjust_light(thislight: *const mjvLight, n: i32) {
    todo!() // cxx-internal:render_gl3.c.o:_adjustLight
}

/// C: mjr_render (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_gl3.h:27)
/// Calls: cxx-internal:render_gl3.c.o:_adjustLight, cxx-internal:render_gl3.c.o:_geomSort, cxx-internal:render_gl3.c.o:_initGL3, cxx-internal:render_gl3.c.o:_initLights, cxx-internal:render_gl3.c.o:_isBehind, cxx-internal:render_gl3.c.o:_isReflective, cxx-internal:render_gl3.c.o:_renderGeom, cxx-internal:render_gl3.c.o:_setView, cxx-internal:render_gl3.c.o:_settexture, cxx:_mjr_getrow4, cxx:_mjr_lookAt, cxx:_mjr_mulMat44, cxx:_mjr_orthoVec, cxx:_mjr_perspective, cxx:_mjr_reflect, cxx:_mjr_restoreBuffer, cxx:_mjr_textActual, cxx:_mju_error, cxx:_mju_free, cxx:_mju_malloc, cxx:_mju_min, cxx:_mju_n2f, cxx:_mjv_averageCamera, cxx:_mjv_cameraInModel, cxx:_mjv_rbound, cxx:_renderGeomReflection
#[allow(unused_variables, non_snake_case)]
pub fn mjr_render(viewport: mjrRect, scn: *mut mjvScene, con: *const mjrContext) {
    todo!() // cxx:_mjr_render
}

/// C: mjr_finish (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_gl3.h:30)
#[allow(unused_variables, non_snake_case)]
pub fn mjr_finish() {
    todo!() // cxx:_mjr_finish
}

/// C: mjr_getError (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_gl3.h:33)
#[allow(unused_variables, non_snake_case)]
pub fn mjr_get_error() -> i32 {
    todo!() // cxx:_mjr_getError
}

