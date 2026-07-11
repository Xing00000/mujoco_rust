//! Port of: render/classic/render_gl2.c
//! IR hash: c6d98e4f4b63b7f2
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: warnAboutARBClipControl (render/classic/render_gl2.c:97)
/// Calls: mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn warn_about_arb_clip_control() {
    todo ! ()
}

/// C: warnAboutARBDepthBuffer (render/classic/render_gl2.c:110)
/// Calls: mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn warn_about_arb_depth_buffer() {
    todo ! ()
}

/// C: flipDepthIfRequired (render/classic/render_gl2.c:122)
/// Calls: warnAboutARBClipControl, warnAboutARBDepthBuffer
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn flip_depth_if_required(depth: *mut f32, viewport: mjrRect, con: *const mjrContext) {
    // WARNING: signature changed — verify body
    // Previous params: (depth : * mut f32, viewport : mjrRect, con : * const mjrContext)
    // Previous return: ()
    todo ! ()
}

/// C: init2D (render/classic/render_gl2.c:407)
#[allow(unused_variables, non_snake_case)]
pub fn init2d() {
    todo ! ()
}

/// C: draw_overlay (render/classic/render_gl2.c:476)
/// Calls: mjr_textActual
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn draw_overlay(font: i32, viewport: mjrRect, skip: i32, gridpos: i32, red: f32, green: f32, blue: f32, overlay: *const i8, con: *const mjrContext) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (font : i32, viewport : mjrRect, skip : i32, gridpos : i32, red : f32, green : f32, blue : f32, overlay : * const i8, con : * const mjrContext)
    // Previous return: i32
    todo ! ()
}

/// C: maketext (render/classic/render_gl2.c:749)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn maketext(format: *const i8, txt: *mut i8, num: f32, txt_sz: i32) {
    // WARNING: signature changed — verify body
    // Previous params: (format : * const i8, txt : * mut i8, num : f32, txt_sz : i32)
    // Previous return: ()
    todo ! ()
}

/// C: textwidth (render/classic/render_gl2.c:787)
#[allow(unused_variables, non_snake_case)]
pub fn textwidth(con: *const mjrContext, text: *const i8) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (con : * const mjrContext, text : * const i8)
    // Previous return: i32
    todo ! ()
}

/// C: mjr_restoreBuffer (render/classic/render_gl2.h:27)
#[allow(unused_variables, non_snake_case)]
pub fn mjr_restore_buffer(con: *const mjrContext) {
    // WARNING: signature changed — verify body
    // Previous params: (con : * const mjrContext)
    // Previous return: ()
    todo ! ()
}

/// C: mjr_textActual (render/classic/render_gl2.h:30)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjr_text_actual(font: i32, txt: *const i8, con: *const mjrContext, x: f32, y: f32, z: f32, r: f32, g: f32, b: f32) {
    // WARNING: signature changed — verify body
    // Previous params: (font : i32, txt : * const i8, con : * const mjrContext, x : f32, y : f32, z : f32, r : f32, g : f32, b : f32)
    // Previous return: ()
    todo ! ()
}

/// C: mjr_setBuffer (render/classic/render_gl2.h:35)
/// Calls: mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mjr_set_buffer(framebuffer: i32, con: *mut mjrContext) {
    // WARNING: signature changed — verify body
    // Previous params: (framebuffer : i32, con : * mut mjrContext)
    // Previous return: ()
    todo ! ()
}

/// C: mjr_readPixels (render/classic/render_gl2.h:39)
/// Calls: flipDepthIfRequired, mjr_restoreBuffer
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjr_read_pixels(rgb: *mut u8, depth: *mut f32, viewport: mjrRect, con: *const mjrContext) {
    // WARNING: signature changed — verify body
    // Previous params: (rgb : * mut u8, depth : * mut f32, viewport : mjrRect, con : * const mjrContext)
    // Previous return: ()
    todo ! ()
}

/// C: mjr_drawPixels (render/classic/render_gl2.h:44)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjr_draw_pixels(rgb: *const u8, depth: *const f32, viewport: mjrRect, con: *const mjrContext) {
    // WARNING: signature changed — verify body
    // Previous params: (rgb : * const u8, depth : * const f32, viewport : mjrRect, con : * const mjrContext)
    // Previous return: ()
    todo ! ()
}

/// C: mjr_blitBuffer (render/classic/render_gl2.h:49)
/// Calls: mjr_restoreBuffer
#[allow(unused_variables, non_snake_case)]
pub fn mjr_blit_buffer(src: mjrRect, dst: mjrRect, flg_color: i32, flg_depth: i32, con: *const mjrContext) {
    // WARNING: signature changed — verify body
    // Previous params: (src : mjrRect, dst : mjrRect, flg_color : i32, flg_depth : i32, con : * const mjrContext)
    // Previous return: ()
    todo ! ()
}

/// C: mjr_setAux (render/classic/render_gl2.h:53)
/// Calls: mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mjr_set_aux(index: i32, con: *const mjrContext) {
    // WARNING: signature changed — verify body
    // Previous params: (index : i32, con : * const mjrContext)
    // Previous return: ()
    todo ! ()
}

/// C: mjr_blitAux (render/classic/render_gl2.h:56)
/// Calls: mjr_restoreBuffer, mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mjr_blit_aux(index: i32, src: mjrRect, left: i32, bottom: i32, con: *const mjrContext) {
    // WARNING: signature changed — verify body
    // Previous params: (index : i32, src : mjrRect, left : i32, bottom : i32, con : * const mjrContext)
    // Previous return: ()
    todo ! ()
}

/// C: mjr_text (render/classic/render_gl2.h:60)
/// Calls: init2D, mjr_textActual
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjr_text(font: i32, txt: *const i8, con: *const mjrContext, x: f32, y: f32, r: f32, g: f32, b: f32) {
    // WARNING: signature changed — verify body
    // Previous params: (font : i32, txt : * const i8, con : * const mjrContext, x : f32, y : f32, r : f32, g : f32, b : f32)
    // Previous return: ()
    todo ! ()
}

/// C: mjr_overlay (render/classic/render_gl2.h:64)
/// Calls: draw_overlay, init2D
#[allow(unused_variables, non_snake_case)]
pub fn mjr_overlay(font: i32, gridpos: i32, viewport: mjrRect, overlay: *const i8, overlay2: *const i8, con: *const mjrContext) {
    // WARNING: signature changed — verify body
    // Previous params: (font : i32, gridpos : i32, viewport : mjrRect, overlay : * const i8, overlay2 : * const i8, con : * const mjrContext)
    // Previous return: ()
    todo ! ()
}

/// C: mjr_maxViewport (render/classic/render_gl2.h:68)
#[allow(unused_variables, non_snake_case)]
pub fn mjr_max_viewport(con: *const mjrContext) -> mjrRect {
    // WARNING: signature changed — verify body
    // Previous params: (con : * const mjrContext)
    // Previous return: mjrRect
    todo ! ()
}

/// C: mjr_rectangle (render/classic/render_gl2.h:71)
/// Calls: init2D
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjr_rectangle(viewport: mjrRect, r: f32, g: f32, b: f32, a: f32) {
    // WARNING: signature changed — verify body
    // Previous params: (viewport : mjrRect, r : f32, g : f32, b : f32, a : f32)
    // Previous return: ()
    todo ! ()
}

/// C: mjr_label (render/classic/render_gl2.h:74)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
/// C: mjr_label (render/classic/render_gl2.h:76)
#[allow(unused_variables, non_snake_case)]
pub fn mjr_label(viewport: mjrRect, font: i32, txt: *const i8, r: f32, g: f32, b: f32, a: f32, rt: f32, gt: f32, bt: f32, con: *const mjrContext) {
    todo!("C++: requires OpenGL API (glDisable, glEnable, glBegin, glEnd, glOrtho, glViewport, glColor4f, glVertex2i, glRasterPos2i, glCallLists, glListBase)")
}

/// C: mjr_figure (render/classic/render_gl2.h:79)
/// Calls: init2D, maketext, mjr_textActual, textwidth
#[allow(unused_variables, non_snake_case)]
pub fn mjr_figure(viewport: mjrRect, fig: *mut mjvFigure, con: *const mjrContext) {
    // WARNING: signature changed — verify body
    // Previous params: (viewport : mjrRect, fig : * mut mjvFigure, con : * const mjrContext)
    // Previous return: ()
    todo ! ()
}

