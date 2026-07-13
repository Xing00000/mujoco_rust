//! Port of: render/classic/render_context.c
//! IR hash: d3ac8715281cd691
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: listAllocate (render/classic/render_context.c:61)
/// Calls: mju_error
#[allow(unused_variables, non_snake_case)]
pub fn list_allocate(base: *mut GLuint, range: *mut GLsizei, newrange: GLsizei) {
    // NOTE: signature changed from previous IR version
    // Previous params: (base : * mut GLuint, range : * mut GLsizei, newrange : GLsizei)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: makePlane (render/classic/render_context.c:75)
/// Calls: listAllocate, mju_max
#[allow(unused_variables, non_snake_case)]
pub fn make_plane(m: *const mjModel, con: *mut mjrContext) {
    // NOTE: signature changed from previous IR version
    // Previous params: (m : * const mjModel, con : * mut mjrContext)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: makeMesh (render/classic/render_context.c:198)
/// Calls: listAllocate, mjr_uploadMesh
#[allow(unused_variables, non_snake_case)]
pub fn make_mesh(m: *const mjModel, con: *mut mjrContext) {
    // NOTE: signature changed from previous IR version
    // Previous params: (m : * const mjModel, con : * mut mjrContext)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: makeHField (render/classic/render_context.c:389)
/// Calls: listAllocate, mjr_uploadHField
#[allow(unused_variables, non_snake_case)]
pub fn make_h_field(m: *const mjModel, con: *mut mjrContext) {
    // NOTE: signature changed from previous IR version
    // Previous params: (m : * const mjModel, con : * mut mjrContext)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: setVertexSphere (render/classic/render_context.c:500)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn set_vertex_sphere(v: *mut f32, n: *mut f32, az: f32, el: f32, sign: i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (v : * mut f32, n : * mut f32, az : f32, el : f32, sign : i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: halfSphere (render/classic/render_context.c:512)
/// Calls: setVertexSphere
#[allow(unused_variables, non_snake_case)]
pub fn half_sphere(sign: i32, nSlice: i32, nStack: i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (sign : i32, nSlice : i32, nStack : i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: sphere (render/classic/render_context.c:595)
/// Calls: setVertexSphere
#[allow(unused_variables, non_snake_case)]
pub fn sphere(nSlice: i32, nStack: i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (nSlice : i32, nStack : i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: setVertexDisk (render/classic/render_context.c:682)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn set_vertex_disk(v: *mut f32, az: f32, r: f32, sign: i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (v : * mut f32, az : f32, r : f32, sign : i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: disk (render/classic/render_context.c:690)
/// Calls: setVertexDisk
#[allow(unused_variables, non_snake_case)]
pub fn disk(sign: i32, nSlice: i32, nStack: i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (sign : i32, nSlice : i32, nStack : i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: setVertexCone (render/classic/render_context.c:759)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn set_vertex_cone(v: *mut f32, n: *mut f32, az: f32, r: f32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (v : * mut f32, n : * mut f32, az : f32, r : f32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: cone (render/classic/render_context.c:775)
/// Calls: mjr_normalizeVec, setVertexCone
#[allow(unused_variables, non_snake_case)]
pub fn cone(nSlice: i32, nStack: i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (nSlice : i32, nStack : i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: setVertexCylinder (render/classic/render_context.c:840)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn set_vertex_cylinder(v: *mut f32, n: *mut f32, az: f32, h: f32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (v : * mut f32, n : * mut f32, az : f32, h : f32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: cylinder (render/classic/render_context.c:852)
/// Calls: setVertexCylinder
#[allow(unused_variables, non_snake_case)]
pub fn cylinder(nSlice: i32, nStack: i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (nSlice : i32, nStack : i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: setVertexHaze (render/classic/render_context.c:890)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn set_vertex_haze(v: *mut f32, az: f32, h: f32, r: f32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (v : * mut f32, az : f32, h : f32, r : f32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: haze (render/classic/render_context.c:898)
/// Calls: setVertexHaze
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn haze(nSlice: i32, r: f32, rgba: *const f32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (nSlice : i32, r : f32, rgba : * const f32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: makeBuiltin (render/classic/render_context.c:945)
/// Calls: cone, cylinder, disk, halfSphere, haze, listAllocate, sphere
#[allow(unused_variables, non_snake_case)]
pub fn make_builtin(m: *const mjModel, con: *mut mjrContext) {
    // NOTE: signature changed from previous IR version
    // Previous params: (m : * const mjModel, con : * mut mjrContext)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: makeShadow (render/classic/render_context.c:1041)
/// Calls: mju_error
#[allow(unused_variables, non_snake_case)]
pub fn make_shadow(m: *const mjModel, con: *mut mjrContext) {
    // NOTE: signature changed from previous IR version
    // Previous params: (m : * const mjModel, con : * mut mjrContext)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: makeOff (render/classic/render_context.c:1094)
/// Calls: mju_error
#[allow(unused_variables, non_snake_case)]
pub fn make_off(con: *mut mjrContext) {
    // NOTE: signature changed from previous IR version
    // Previous params: (con : * mut mjrContext)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: makeFont (render/classic/render_context.c:1195)
/// Calls: mju_error
#[allow(unused_variables, non_snake_case)]
pub fn make_font(con: *mut mjrContext, fontscale: i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (con : * mut mjrContext, fontscale : i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: makeMaterial (render/classic/render_context.c:1303)
#[allow(unused_variables, non_snake_case)]
pub fn make_material(m: *const mjModel, con: *mut mjrContext) {
    // NOTE: signature changed from previous IR version
    // Previous params: (m : * const mjModel, con : * mut mjrContext)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: makeTexture (render/classic/render_context.c:1341)
/// Calls: mjr_uploadTexture, mju_error
#[allow(unused_variables, non_snake_case)]
pub fn make_texture(m: *const mjModel, con: *mut mjrContext) {
    // NOTE: signature changed from previous IR version
    // Previous params: (m : * const mjModel, con : * mut mjrContext)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: makeSkin (render/classic/render_context.c:1457)
/// Calls: mju_malloc
#[allow(unused_variables, non_snake_case)]
pub fn make_skin(m: *const mjModel, con: *mut mjrContext) {
    // NOTE: signature changed from previous IR version
    // Previous params: (m : * const mjModel, con : * mut mjrContext)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: debugCallback (render/classic/render_context.c:1504)
#[allow(unused_variables, non_snake_case)]
pub fn debug_callback(source: GLenum, r#type: GLenum, id: GLuint, severity: GLenum, length: GLsizei, message: *const GLchar, userParam: *const ()) {
    // NOTE: signature changed from previous IR version
    // Previous params: (source : GLenum, r#type : GLenum, id : GLuint, severity : GLenum, length : GLsizei, message : * const GLchar, userParam : * const ())
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: glDebugEnabled (render/classic/render_context.c:1518)
#[allow(unused_variables, non_snake_case)]
pub fn gl_debug_enabled() -> i32 {
    todo ! ()
}

/// C: mjr_makeContext_offSize (render/classic/render_context.c:1525)
/// Calls: glDebugEnabled, makeBuiltin, makeFont, makeHField, makeMaterial, makeMesh, makeOff, makePlane, makeShadow, makeSkin, makeTexture, mjGladLoadGL, mjr_freeContext, mjr_setBuffer, mju_error, mju_round, mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn mjr_make_context_off_size(m: *const mjModel, con: *mut mjrContext, fontscale: i32, default_offwidth: i32, default_offheight: i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (m : * const mjModel, con : * mut mjrContext, fontscale : i32, default_offwidth : i32, default_offheight : i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjr_defaultContext (render/classic/render_context.h:42)
#[allow(unused_variables, non_snake_case)]
pub fn mjr_default_context(con: *mut mjrContext) {
    // NOTE: signature changed from previous IR version
    // Previous params: (con : * mut mjrContext)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjr_makeContext (render/classic/render_context.h:45)
/// Calls: mjr_makeContext_offSize
#[allow(unused_variables, non_snake_case)]
pub fn mjr_make_context(m: *const mjModel, con: *mut mjrContext, fontscale: i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (m : * const mjModel, con : * mut mjrContext, fontscale : i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjr_changeFont (render/classic/render_context.h:48)
/// Calls: makeFont
#[allow(unused_variables, non_snake_case)]
pub fn mjr_change_font(fontscale: i32, con: *mut mjrContext) {
    // NOTE: signature changed from previous IR version
    // Previous params: (fontscale : i32, con : * mut mjrContext)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjr_addAux (render/classic/render_context.h:51)
/// Calls: mjr_restoreBuffer, mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mjr_add_aux(index: i32, width: i32, height: i32, samples: i32, con: *mut mjrContext) {
    // NOTE: signature changed from previous IR version
    // Previous params: (index : i32, width : i32, height : i32, samples : i32, con : * mut mjrContext)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjr_freeContext (render/classic/render_context.h:54)
/// Calls: mjr_defaultContext, mju_free
#[allow(unused_variables, non_snake_case)]
pub fn mjr_free_context(con: *mut mjrContext) {
    // NOTE: signature changed from previous IR version
    // Previous params: (con : * mut mjrContext)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjr_resizeOffscreen (render/classic/render_context.h:57)
/// Calls: makeOff
#[allow(unused_variables, non_snake_case)]
pub fn mjr_resize_offscreen(offwidth: i32, offheight: i32, con: *mut mjrContext) {
    // NOTE: signature changed from previous IR version
    // Previous params: (offwidth : i32, offheight : i32, con : * mut mjrContext)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjr_uploadTexture (render/classic/render_context.h:60)
/// Calls: mjr_setf4, mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mjr_upload_texture(m: *const mjModel, con: *const mjrContext, texid: i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (m : * const mjModel, con : * const mjrContext, texid : i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjr_uploadMesh (render/classic/render_context.h:63)
/// Calls: mjr_makeNormal, mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mjr_upload_mesh(m: *const mjModel, con: *const mjrContext, meshid: i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (m : * const mjModel, con : * const mjrContext, meshid : i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjr_uploadHField (render/classic/render_context.h:66)
/// Calls: addVert, mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mjr_upload_h_field(m: *const mjModel, con: *const mjrContext, hfieldid: i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (m : * const mjModel, con : * const mjrContext, hfieldid : i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

