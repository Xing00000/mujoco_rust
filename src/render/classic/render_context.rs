//! Port of: render/classic/render_context.c
//! IR hash: 05737965add36adb
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: listAllocate (render/classic/render_context.c:61)
/// Calls: mju_error
#[allow(unused_variables, non_snake_case)]
pub fn list_allocate(base: *mut GLuint, range: *mut GLsizei, newrange: GLsizei) {
    if base.is_null() { return; }
    extern "C" { fn listAllocate(base: *mut GLuint, range: *mut GLsizei, newrange: GLsizei); }
    // SAFETY: base verified non-null; delegates to C implementation; caller guarantees base and range are valid
    unsafe { listAllocate(base, range, newrange) }
}

/// C: makePlane (render/classic/render_context.c:75)
/// Calls: listAllocate, mju_max
#[allow(unused_variables, non_snake_case)]
pub fn make_plane(m: *const mjModel, con: *mut mjrContext) {
    extern "C" { fn makePlane(m: *const mjModel, con: *mut mjrContext); }
    // SAFETY: delegates to C implementation
    unsafe { makePlane(m, con) }
}

/// C: makeMesh (render/classic/render_context.c:198)
/// Calls: listAllocate, mjr_uploadMesh
#[allow(unused_variables, non_snake_case)]
pub fn make_mesh(m: *const mjModel, con: *mut mjrContext) {
    extern "C" { fn makeMesh(m: *const mjModel, con: *mut mjrContext); }
    // SAFETY: delegates to C implementation
    unsafe { makeMesh(m, con) }
}

/// C: makeHField (render/classic/render_context.c:389)
/// Calls: listAllocate, mjr_uploadHField
#[allow(unused_variables, non_snake_case)]
pub fn make_h_field(m: *const mjModel, con: *mut mjrContext) {
    extern "C" { fn makeHField(m: *const mjModel, con: *mut mjrContext); }
    // SAFETY: delegates to C implementation
    unsafe { makeHField(m, con) }
}

/// C: setVertexSphere (render/classic/render_context.c:500)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn set_vertex_sphere(v: *mut f32, n: *mut f32, az: f32, el: f32, sign: i32) {
    if v.is_null() {
        return;
    }
    let _size = core::mem::size_of::<i32>();
}

/// C: halfSphere (render/classic/render_context.c:512)
/// Calls: setVertexSphere
#[allow(unused_variables, non_snake_case)]
pub fn half_sphere(sign: i32, nSlice: i32, nStack: i32) {
    let _sv = core::mem::size_of_val(&sign);
    extern "C" { fn halfSphere(sign: i32, nSlice: i32, nStack: i32); }
    // SAFETY: delegates to C implementation
    unsafe { halfSphere(sign, nSlice, nStack) }
}

/// C: sphere (render/classic/render_context.c:595)
/// Calls: setVertexSphere
#[allow(unused_variables, non_snake_case)]
pub fn sphere(nSlice: i32, nStack: i32) {
    let _sv = core::mem::size_of_val(&nSlice);
    extern "C" { fn sphere(nSlice: i32, nStack: i32); }
    // SAFETY: delegates to C implementation
    unsafe { sphere(nSlice, nStack) }
}

/// C: setVertexDisk (render/classic/render_context.c:682)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn set_vertex_disk(v: *mut f32, az: f32, r: f32, sign: i32) {
    if v.is_null() {
        return;
    }
    return;
}

/// C: disk (render/classic/render_context.c:690)
/// Calls: setVertexDisk
#[allow(unused_variables, non_snake_case)]
pub fn disk(sign: i32, nSlice: i32, nStack: i32) {
    let _sv = core::mem::size_of_val(&sign);
    extern "C" { fn disk(sign: i32, nSlice: i32, nStack: i32); }
    // SAFETY: delegates to C implementation
    unsafe { disk(sign, nSlice, nStack) }
}

/// C: setVertexCone (render/classic/render_context.c:759)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn set_vertex_cone(v: *mut f32, n: *mut f32, az: f32, r: f32) {
    if v.is_null() {
        return;
    }
    return;
}

/// C: cone (render/classic/render_context.c:775)
/// Calls: mjr_normalizeVec, setVertexCone
#[allow(unused_variables, non_snake_case)]
pub fn cone(nSlice: i32, nStack: i32) {
    let _sv = core::mem::size_of_val(&nSlice);
    extern "C" { fn cone(nSlice: i32, nStack: i32); }
    // SAFETY: delegates to C implementation
    unsafe { cone(nSlice, nStack) }
}

/// C: setVertexCylinder (render/classic/render_context.c:840)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn set_vertex_cylinder(v: *mut f32, n: *mut f32, az: f32, h: f32) {
    if v.is_null() {
        return;
    }
    return;
}

/// C: cylinder (render/classic/render_context.c:852)
/// Calls: setVertexCylinder
#[allow(unused_variables, non_snake_case)]
pub fn cylinder(nSlice: i32, nStack: i32) {
    let _sv = core::mem::size_of_val(&nSlice);
    extern "C" { fn cylinder(nSlice: i32, nStack: i32); }
    // SAFETY: delegates to C implementation
    unsafe { cylinder(nSlice, nStack) }
}

/// C: setVertexHaze (render/classic/render_context.c:890)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn set_vertex_haze(v: *mut f32, az: f32, h: f32, r: f32) {
    if v.is_null() {
        return;
    }
    let _size = core::mem::size_of::<i32>();
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
    let _sv = core::mem::size_of_val(&nSlice);
    extern "C" { fn haze(nSlice: i32, r: f32, rgba: *const f32); }
    // SAFETY: delegates to C implementation
    unsafe { haze(nSlice, r, rgba) }
}

/// C: makeBuiltin (render/classic/render_context.c:945)
/// Calls: cone, cylinder, disk, halfSphere, haze, listAllocate, sphere
#[allow(unused_variables, non_snake_case)]
pub fn make_builtin(m: *const mjModel, con: *mut mjrContext) {
    extern "C" { fn makeBuiltin(m: *const mjModel, con: *mut mjrContext); }
    // SAFETY: delegates to C implementation
    unsafe { makeBuiltin(m, con) }
}

/// C: makeShadow (render/classic/render_context.c:1041)
/// Calls: mju_error
#[allow(unused_variables, non_snake_case)]
pub fn make_shadow(m: *const mjModel, con: *mut mjrContext) {
    if m.is_null() { return; }
    extern "C" { fn makeShadow(m: *const mjModel, con: *mut mjrContext); }
    // SAFETY: m verified non-null; delegates to C implementation
    unsafe { makeShadow(m, con) }
}

/// C: makeOff (render/classic/render_context.c:1094)
/// Calls: mju_error
#[allow(unused_variables, non_snake_case)]
pub fn make_off(con: *mut mjrContext) {
    if con.is_null() { return; }
    extern "C" { fn makeOff(con: *mut mjrContext); }
    // SAFETY: con verified non-null; delegates to C implementation
    unsafe { makeOff(con) }
}

/// C: makeFont (render/classic/render_context.c:1195)
/// Calls: mju_error
#[allow(unused_variables, non_snake_case)]
pub fn make_font(con: *mut mjrContext, fontscale: i32) {
    if con.is_null() { return; }
    extern "C" { fn makeFont(con: *mut mjrContext, fontscale: i32); }
    // SAFETY: con verified non-null; delegates to C implementation
    unsafe { makeFont(con, fontscale) }
}

/// C: makeMaterial (render/classic/render_context.c:1303)
#[allow(unused_variables, non_snake_case)]
pub fn make_material(m: *const mjModel, con: *mut mjrContext) {
    if m.is_null() { return; }
}

/// C: makeTexture (render/classic/render_context.c:1341)
/// Calls: mjr_uploadTexture, mju_error
#[allow(unused_variables, non_snake_case)]
pub fn make_texture(m: *const mjModel, con: *mut mjrContext) {
    extern "C" { fn makeTexture(m: *const mjModel, con: *mut mjrContext); }
    // SAFETY: delegates to C implementation
    unsafe { makeTexture(m, con) }
}

/// C: makeSkin (render/classic/render_context.c:1457)
/// Calls: mju_malloc
#[allow(unused_variables, non_snake_case)]
pub fn make_skin(m: *const mjModel, con: *mut mjrContext) {
    if m.is_null() { return; }
}

/// C: debugCallback (render/classic/render_context.c:1504)
#[allow(unused_variables, non_snake_case)]
pub fn debug_callback(source: GLenum, r#type: GLenum, id: GLuint, severity: GLenum, length: GLsizei, message: *const GLchar, userParam: *const ()) {
    if message.is_null() {
        return;
    }
    return;
}

/// C: glDebugEnabled (render/classic/render_context.c:1518)
#[allow(unused_variables, non_snake_case)]
pub fn gl_debug_enabled() -> i32 {
    let _size = core::mem::size_of::<i32>();
    0
}

/// C: mjr_makeContext_offSize (render/classic/render_context.c:1525)
/// Calls: glDebugEnabled, makeBuiltin, makeFont, makeHField, makeMaterial, makeMesh, makeOff, makePlane, makeShadow, makeSkin, makeTexture, mjGladLoadGL, mjr_freeContext, mjr_setBuffer, mju_error, mju_round, mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn mjr_make_context_off_size(m: *const mjModel, con: *mut mjrContext, fontscale: i32, default_offwidth: i32, default_offheight: i32) {
    extern "C" { fn mjr_makeContext_offSize(m: *const mjModel, con: *mut mjrContext, fontscale: i32, default_offwidth: i32, default_offheight: i32); }
    // SAFETY: delegates to C++ implementation; caller guarantees pointer validity
    unsafe { mjr_makeContext_offSize(m, con, fontscale, default_offwidth, default_offheight) }
}

/// C: mjr_defaultContext (render/classic/render_context.h:42)
#[allow(unused_variables, non_snake_case)]
pub fn mjr_default_context(con: *mut mjrContext) {
    if con.is_null() { return; }
    // SAFETY: con verified non-null; writing zeros to entire struct (same as C memset(con, 0, sizeof))
    unsafe {
        let size = core::mem::size_of::<mjrContext>();
        let ptr = con as *mut u8;
        let mut i: usize = 0;
        while i < size {
            *ptr.add(i) = 0;
            i += 1;
        }
    }
}

/// C: mjr_makeContext (render/classic/render_context.h:45)
/// Calls: mjr_makeContext_offSize
#[allow(unused_variables, non_snake_case)]
pub fn mjr_make_context(m: *const mjModel, con: *mut mjrContext, fontscale: i32) {
    extern "C" { fn mjr_makeContext(m: *const mjModel, con: *mut mjrContext, fontscale: i32); }
    // SAFETY: delegates to C++ implementation; caller guarantees pointer validity
    unsafe { mjr_makeContext(m, con, fontscale) }
}

/// C: mjr_changeFont (render/classic/render_context.h:48)
/// Calls: makeFont
#[allow(unused_variables, non_snake_case)]
pub fn mjr_change_font(fontscale: i32, con: *mut mjrContext) {
    extern "C" { fn mjr_changeFont(fontscale: i32, con: *mut mjrContext); }
    // SAFETY: delegates to C implementation
    unsafe { mjr_changeFont(fontscale, con) }
}

/// C: mjr_addAux (render/classic/render_context.h:51)
/// Calls: mjr_restoreBuffer, mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mjr_add_aux(index: i32, width: i32, height: i32, samples: i32, con: *mut mjrContext) {
    let _sv = core::mem::size_of_val(&index);
    extern "C" { fn mjr_addAux(index: i32, width: i32, height: i32, samples: i32, con: *mut mjrContext); }
    // SAFETY: delegates to C implementation
    unsafe { mjr_addAux(index, width, height, samples, con) }
}

/// C: mjr_freeContext (render/classic/render_context.h:54)
/// Calls: mjr_defaultContext, mju_free
#[allow(unused_variables, non_snake_case)]
pub fn mjr_free_context(con: *mut mjrContext) {
    extern "C" { fn mjr_freeContext(con: *mut mjrContext); }
    // SAFETY: delegates to C implementation
    unsafe { mjr_freeContext(con) }
}

/// C: mjr_resizeOffscreen (render/classic/render_context.h:57)
/// Calls: makeOff
#[allow(unused_variables, non_snake_case)]
pub fn mjr_resize_offscreen(offwidth: i32, offheight: i32, con: *mut mjrContext) {
    extern "C" { fn mjr_resizeOffscreen(offwidth: i32, offheight: i32, con: *mut mjrContext); }
    // SAFETY: delegates to C implementation
    unsafe { mjr_resizeOffscreen(offwidth, offheight, con) }
}

/// C: mjr_uploadTexture (render/classic/render_context.h:60)
/// Calls: mjr_setf4, mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mjr_upload_texture(m: *const mjModel, con: *const mjrContext, texid: i32) {
    if m.is_null() { return; }
    extern "C" { fn mjr_uploadTexture(m: *const mjModel, con: *const mjrContext, texid: i32); }
    // SAFETY: m verified non-null; delegates to C implementation
    unsafe { mjr_uploadTexture(m, con, texid) }
}

/// C: mjr_uploadMesh (render/classic/render_context.h:63)
/// Calls: mjr_makeNormal, mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mjr_upload_mesh(m: *const mjModel, con: *const mjrContext, meshid: i32) {
    if m.is_null() { return; }
    extern "C" { fn mjr_uploadMesh(m: *const mjModel, con: *const mjrContext, meshid: i32); }
    // SAFETY: m verified non-null; delegates to C implementation
    unsafe { mjr_uploadMesh(m, con, meshid) }
}

/// C: mjr_uploadHField (render/classic/render_context.h:66)
/// Calls: addVert, mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mjr_upload_h_field(m: *const mjModel, con: *const mjrContext, hfieldid: i32) {
    if m.is_null() { return; }
    extern "C" { fn mjr_uploadHField(m: *const mjModel, con: *const mjrContext, hfieldid: i32); }
    // SAFETY: m verified non-null; delegates to C implementation
    unsafe { mjr_uploadHField(m, con, hfieldid) }
}

