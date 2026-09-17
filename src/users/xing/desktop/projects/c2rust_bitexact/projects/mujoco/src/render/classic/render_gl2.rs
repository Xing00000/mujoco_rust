//! Port of: Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_gl2.c
//! IR hash: 1fa44043fe383ce9
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: warnAboutARBClipControl (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_gl2.c:97)
/// Calls: cxx:_mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn warn_about_arb_clip_control() {
    todo!() // cxx-internal:render_gl2.c.o:_warnAboutARBClipControl
}

/// C: warnAboutARBDepthBuffer (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_gl2.c:110)
/// Calls: cxx:_mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn warn_about_arb_depth_buffer() {
    todo!() // cxx-internal:render_gl2.c.o:_warnAboutARBDepthBuffer
}

/// C: flipDepthIfRequired (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_gl2.c:122)
/// Calls: cxx-internal:render_gl2.c.o:_warnAboutARBClipControl, cxx-internal:render_gl2.c.o:_warnAboutARBDepthBuffer
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn flip_depth_if_required(depth: *mut f32, viewport: mjrRect, con: *const mjrContext) {
    todo!() // cxx-internal:render_gl2.c.o:_flipDepthIfRequired
}

/// C: init2D (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_gl2.c:407)
#[allow(unused_variables, non_snake_case)]
pub fn init2d() {
    todo!() // cxx-internal:render_gl2.c.o:_init2D
}

/// C: draw_overlay (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_gl2.c:476)
/// Calls: cxx:_mjr_textActual
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn draw_overlay(font: i32, viewport: mjrRect, skip: i32, gridpos: i32, red: f32, green: f32, blue: f32, overlay: *const i8, con: *const mjrContext) -> i32 {
    todo!() // cxx-internal:render_gl2.c.o:_draw_overlay
}

/// C: maketext (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_gl2.c:749)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn maketext(format: *const i8, txt: *mut i8, num: f32, txt_sz: i32) {
    todo!() // cxx-internal:render_gl2.c.o:_maketext
}

/// C: mjr_restoreBuffer (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_gl2.h:27)
#[allow(unused_variables, non_snake_case)]
pub fn mjr_restore_buffer(con: *const mjrContext) {
    todo!() // cxx:_mjr_restoreBuffer
}

/// C: mjr_textActual (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_gl2.h:30)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjr_text_actual(font: i32, txt: *const i8, con: *const mjrContext, x: f32, y: f32, z: f32, r: f32, g: f32, b: f32) {
    todo!() // cxx:_mjr_textActual
}

/// C: mjr_setBuffer (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_gl2.h:35)
/// Calls: cxx:_mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mjr_set_buffer(framebuffer: i32, con: *mut mjrContext) {
    todo!() // cxx:_mjr_setBuffer
}

/// C: mjr_readPixels (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_gl2.h:39)
/// Calls: cxx-internal:render_gl2.c.o:_flipDepthIfRequired, cxx:_mjr_restoreBuffer
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjr_read_pixels(rgb: *mut u8, depth: *mut f32, viewport: mjrRect, con: *const mjrContext) {
    todo!() // cxx:_mjr_readPixels
}

/// C: mjr_drawPixels (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_gl2.h:44)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjr_draw_pixels(rgb: *const u8, depth: *const f32, viewport: mjrRect, con: *const mjrContext) {
    todo!() // cxx:_mjr_drawPixels
}

/// C: mjr_blitBuffer (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_gl2.h:49)
/// Calls: cxx:_mjr_restoreBuffer
#[allow(unused_variables, non_snake_case)]
pub fn mjr_blit_buffer(src: mjrRect, dst: mjrRect, flg_color: i32, flg_depth: i32, con: *const mjrContext) {
    todo!() // cxx:_mjr_blitBuffer
}

/// C: mjr_setAux (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_gl2.h:53)
/// Calls: cxx:_mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mjr_set_aux(index: i32, con: *const mjrContext) {
    todo!() // cxx:_mjr_setAux
}

/// C: mjr_blitAux (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_gl2.h:56)
/// Calls: cxx:_mjr_restoreBuffer, cxx:_mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mjr_blit_aux(index: i32, src: mjrRect, left: i32, bottom: i32, con: *const mjrContext) {
    todo!() // cxx:_mjr_blitAux
}

/// C: mjr_text (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_gl2.h:60)
/// Calls: cxx-internal:render_gl2.c.o:_init2D, cxx:_mjr_textActual
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjr_text(font: i32, txt: *const i8, con: *const mjrContext, x: f32, y: f32, r: f32, g: f32, b: f32) {
    todo!() // cxx:_mjr_text
}

/// C: mjr_overlay (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_gl2.h:64)
/// Calls: cxx-internal:render_gl2.c.o:_draw_overlay, cxx-internal:render_gl2.c.o:_init2D
#[allow(unused_variables, non_snake_case)]
pub fn mjr_overlay(font: i32, gridpos: i32, viewport: mjrRect, overlay: *const i8, overlay2: *const i8, con: *const mjrContext) {
    todo!() // cxx:_mjr_overlay
}

/// C: mjr_maxViewport (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_gl2.h:68)
#[allow(unused_variables, non_snake_case)]
pub fn mjr_max_viewport(con: *const mjrContext) -> mjrRect {
    todo!() // cxx:_mjr_maxViewport
}

/// C: mjr_rectangle (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_gl2.h:71)
/// Calls: cxx-internal:render_gl2.c.o:_init2D
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjr_rectangle(viewport: mjrRect, r: f32, g: f32, b: f32, a: f32) {
    todo!() // cxx:_mjr_rectangle
}

/// C: mjr_label (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_gl2.h:74)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjr_label(viewport: mjrRect, font: i32, txt: *const i8, r: f32, g: f32, b: f32, a: f32, rt: f32, gt: f32, bt: f32, con: *const mjrContext) {
    todo!() // cxx:_mjr_label
}

/// C: mjr_figure (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/render/classic/render_gl2.h:79)
/// Calls: cxx-internal:render_gl2.c.o:_init2D, cxx-internal:render_gl2.c.o:_maketext, cxx:_mjr_textActual
#[allow(unused_variables, non_snake_case)]
pub fn mjr_figure(viewport: mjrRect, fig: *mut mjvFigure, con: *const mjrContext) {
    todo!() // cxx:_mjr_figure
}

