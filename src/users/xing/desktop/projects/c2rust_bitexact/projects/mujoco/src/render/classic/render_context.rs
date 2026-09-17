//! Port of: Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_context.c
//! IR hash: 1fa44043fe383ce9
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: listAllocate (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_context.c:61)
/// Calls: cxx:_mju_error
#[allow(unused_variables, non_snake_case)]
pub fn list_allocate(base: *mut u32, range: *mut i32, newrange: i32) {
    todo!() // cxx-internal:render_context.c.o:_listAllocate
}

/// C: makePlane (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_context.c:75)
/// Calls: cxx-internal:render_context.c.o:_listAllocate, cxx:_mju_max
#[allow(unused_variables, non_snake_case)]
pub fn make_plane(m: *const mjModel, con: *mut mjrContext) {
    todo!() // cxx-internal:render_context.c.o:_makePlane
}

/// C: makeMesh (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_context.c:198)
/// Calls: cxx-internal:render_context.c.o:_listAllocate, cxx:_mjr_uploadMesh
#[allow(unused_variables, non_snake_case)]
pub fn make_mesh(m: *const mjModel, con: *mut mjrContext) {
    todo!() // cxx-internal:render_context.c.o:_makeMesh
}

/// C: makeHField (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_context.c:389)
/// Calls: cxx-internal:render_context.c.o:_listAllocate, cxx:_mjr_uploadHField
#[allow(unused_variables, non_snake_case)]
pub fn make_h_field(m: *const mjModel, con: *mut mjrContext) {
    todo!() // cxx-internal:render_context.c.o:_makeHField
}

/// C: setVertexSphere (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_context.c:500)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn set_vertex_sphere(v: *mut f32, n: *mut f32, az: f32, el: f32, sign: i32) {
    todo!() // cxx-internal:render_context.c.o:_setVertexSphere
}

/// C: halfSphere (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_context.c:512)
/// Calls: cxx-internal:render_context.c.o:_setVertexSphere
#[allow(unused_variables, non_snake_case)]
pub fn half_sphere(sign: i32, nSlice: i32, nStack: i32) {
    todo!() // cxx-internal:render_context.c.o:_halfSphere
}

/// C: sphere (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_context.c:595)
/// Calls: cxx-internal:render_context.c.o:_setVertexSphere
#[allow(unused_variables, non_snake_case)]
pub fn sphere(nSlice: i32, nStack: i32) {
    todo!() // cxx-internal:render_context.c.o:_sphere
}

/// C: setVertexDisk (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_context.c:682)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn set_vertex_disk(v: *mut f32, az: f32, r: f32, sign: i32) {
    todo!() // cxx-internal:render_context.c.o:_setVertexDisk
}

/// C: disk (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_context.c:690)
/// Calls: cxx-internal:render_context.c.o:_setVertexDisk
#[allow(unused_variables, non_snake_case)]
pub fn disk(sign: i32, nSlice: i32, nStack: i32) {
    todo!() // cxx-internal:render_context.c.o:_disk
}

/// C: setVertexCone (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_context.c:759)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn set_vertex_cone(v: *mut f32, n: *mut f32, az: f32, r: f32) {
    todo!() // cxx-internal:render_context.c.o:_setVertexCone
}

/// C: cone (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_context.c:775)
/// Calls: cxx-internal:render_context.c.o:_setVertexCone, cxx:_mjr_normalizeVec
#[allow(unused_variables, non_snake_case)]
pub fn cone(nSlice: i32, nStack: i32) {
    todo!() // cxx-internal:render_context.c.o:_cone
}

/// C: setVertexCylinder (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_context.c:840)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn set_vertex_cylinder(v: *mut f32, n: *mut f32, az: f32, h: f32) {
    todo!() // cxx-internal:render_context.c.o:_setVertexCylinder
}

/// C: cylinder (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_context.c:852)
/// Calls: cxx-internal:render_context.c.o:_setVertexCylinder
#[allow(unused_variables, non_snake_case)]
pub fn cylinder(nSlice: i32, nStack: i32) {
    todo!() // cxx-internal:render_context.c.o:_cylinder
}

/// C: setVertexHaze (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_context.c:890)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn set_vertex_haze(v: *mut f32, az: f32, h: f32, r: f32) {
    todo!() // cxx-internal:render_context.c.o:_setVertexHaze
}

/// C: haze (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_context.c:898)
/// Calls: cxx-internal:render_context.c.o:_setVertexHaze
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn haze(nSlice: i32, r: f32, rgba: *const f32) {
    todo!() // cxx-internal:render_context.c.o:_haze
}

/// C: makeBuiltin (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_context.c:945)
/// Calls: cxx-internal:render_context.c.o:_cone, cxx-internal:render_context.c.o:_cylinder, cxx-internal:render_context.c.o:_disk, cxx-internal:render_context.c.o:_halfSphere, cxx-internal:render_context.c.o:_haze, cxx-internal:render_context.c.o:_listAllocate, cxx-internal:render_context.c.o:_sphere
#[allow(unused_variables, non_snake_case)]
pub fn make_builtin(m: *const mjModel, con: *mut mjrContext) {
    todo!() // cxx-internal:render_context.c.o:_makeBuiltin
}

/// C: makeShadow (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_context.c:1041)
/// Calls: cxx:_mju_error
#[allow(unused_variables, non_snake_case)]
pub fn make_shadow(m: *const mjModel, con: *mut mjrContext) {
    todo!() // cxx-internal:render_context.c.o:_makeShadow
}

/// C: makeOff (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_context.c:1094)
/// Calls: cxx:_mju_error
#[allow(unused_variables, non_snake_case)]
pub fn make_off(con: *mut mjrContext) {
    todo!() // cxx-internal:render_context.c.o:_makeOff
}

/// C: makeFont (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_context.c:1195)
/// Calls: cxx:_mju_error
#[allow(unused_variables, non_snake_case)]
pub fn make_font(con: *mut mjrContext, fontscale: i32) {
    todo!() // cxx-internal:render_context.c.o:_makeFont
}

/// C: makeMaterial (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_context.c:1303)
/// Calls: cxx:_mju_error
#[allow(unused_variables, non_snake_case)]
pub fn make_material(m: *const mjModel, con: *mut mjrContext) {
    todo!() // cxx-internal:render_context.c.o:_makeMaterial
}

/// C: makeTexture (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_context.c:1341)
/// Calls: cxx:_mjr_uploadTexture, cxx:_mju_error
#[allow(unused_variables, non_snake_case)]
pub fn make_texture(m: *const mjModel, con: *mut mjrContext) {
    todo!() // cxx-internal:render_context.c.o:_makeTexture
}

/// C: makeSkin (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_context.c:1457)
/// Calls: cxx:_mju_malloc
#[allow(unused_variables, non_snake_case)]
pub fn make_skin(m: *const mjModel, con: *mut mjrContext) {
    todo!() // cxx-internal:render_context.c.o:_makeSkin
}

/// C: debugCallback (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_context.c:1504)
#[allow(unused_variables, non_snake_case)]
pub fn debug_callback(source: u32, r#type: u32, id: u32, severity: u32, length: i32, message: *const i8, userParam: *const ()) {
    todo!() // cxx:_debugCallback
}

/// C: glDebugEnabled (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_context.c:1518)
#[allow(unused_variables, non_snake_case)]
pub fn gl_debug_enabled() -> i32 {
    todo!() // cxx-internal:render_context.c.o:_glDebugEnabled
}

/// C: mjr_makeContext_offSize (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_context.c:1525)
/// Calls: cxx-internal:render_context.c.o:_glDebugEnabled, cxx-internal:render_context.c.o:_makeBuiltin, cxx-internal:render_context.c.o:_makeFont, cxx-internal:render_context.c.o:_makeHField, cxx-internal:render_context.c.o:_makeMaterial, cxx-internal:render_context.c.o:_makeMesh, cxx-internal:render_context.c.o:_makeOff, cxx-internal:render_context.c.o:_makePlane, cxx-internal:render_context.c.o:_makeShadow, cxx-internal:render_context.c.o:_makeSkin, cxx-internal:render_context.c.o:_makeTexture, cxx:_mjGladLoadGL, cxx:_mjr_freeContext, cxx:_mjr_setBuffer, cxx:_mju_error, cxx:_mju_round, cxx:_mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn mjr_make_context_off_size(m: *const mjModel, con: *mut mjrContext, fontscale: i32, default_offwidth: i32, default_offheight: i32) {
    todo!() // cxx:_mjr_makeContext_offSize
}

/// C: mjr_defaultContext (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_context.h:42)
#[allow(unused_variables, non_snake_case)]
pub fn mjr_default_context(con: *mut mjrContext) {
    todo!() // cxx:_mjr_defaultContext
}

/// C: mjr_makeContext (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_context.h:45)
/// Calls: cxx:_mjr_makeContext_offSize
#[allow(unused_variables, non_snake_case)]
pub fn mjr_make_context(m: *const mjModel, con: *mut mjrContext, fontscale: i32) {
    todo!() // cxx:_mjr_makeContext
}

/// C: mjr_changeFont (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_context.h:48)
/// Calls: cxx-internal:render_context.c.o:_makeFont
#[allow(unused_variables, non_snake_case)]
pub fn mjr_change_font(fontscale: i32, con: *mut mjrContext) {
    todo!() // cxx:_mjr_changeFont
}

/// C: mjr_addAux (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_context.h:51)
/// Calls: cxx:_mjr_restoreBuffer, cxx:_mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mjr_add_aux(index: i32, width: i32, height: i32, samples: i32, con: *mut mjrContext) {
    todo!() // cxx:_mjr_addAux
}

/// C: mjr_freeContext (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_context.h:54)
/// Calls: cxx:_mjr_defaultContext, cxx:_mju_free
#[allow(unused_variables, non_snake_case)]
pub fn mjr_free_context(con: *mut mjrContext) {
    todo!() // cxx:_mjr_freeContext
}

/// C: mjr_resizeOffscreen (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_context.h:57)
/// Calls: cxx-internal:render_context.c.o:_makeOff
#[allow(unused_variables, non_snake_case)]
pub fn mjr_resize_offscreen(offwidth: i32, offheight: i32, con: *mut mjrContext) {
    todo!() // cxx:_mjr_resizeOffscreen
}

/// C: mjr_uploadTexture (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_context.h:60)
/// Calls: cxx:_mjr_setf4, cxx:_mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mjr_upload_texture(m: *const mjModel, con: *const mjrContext, texid: i32) {
    todo!() // cxx:_mjr_uploadTexture
}

/// C: mjr_uploadMesh (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_context.h:63)
/// Calls: cxx:_mjr_makeNormal, cxx:_mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mjr_upload_mesh(m: *const mjModel, con: *const mjrContext, meshid: i32) {
    todo!() // cxx:_mjr_uploadMesh
}

/// C: mjr_uploadHField (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_context.h:66)
/// Calls: cxx:_mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mjr_upload_h_field(m: *const mjModel, con: *const mjrContext, hfieldid: i32) {
    todo!() // cxx:_mjr_uploadHField
}

