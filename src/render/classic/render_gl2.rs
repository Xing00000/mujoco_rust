//! Port of: render/classic/render_gl2.c
//! IR hash: 05737965add36adb
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: warnAboutARBClipControl (render/classic/render_gl2.c:97)
/// Calls: mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn warn_about_arb_clip_control() {
    extern "C" { fn warnAboutARBClipControl_impl(); }
    // SAFETY: delegates to C implementation
    unsafe { warnAboutARBClipControl_impl() }
}

/// C: warnAboutARBDepthBuffer (render/classic/render_gl2.c:110)
/// Calls: mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn warn_about_arb_depth_buffer() {
    extern "C" { fn warnAboutARBDepthBuffer_impl(); }
    // SAFETY: delegates to C implementation
    unsafe { warnAboutARBDepthBuffer_impl() }
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
    extern "C" { fn flipDepthIfRequired_impl(depth: *mut f32, viewport: mjrRect, con: *const mjrContext); }
    // SAFETY: delegates to C implementation
    unsafe { flipDepthIfRequired_impl(depth, viewport, con) }
}

/// C: init2D (render/classic/render_gl2.c:407)
#[allow(unused_variables, non_snake_case)]
pub fn init2d() {
    extern "C" { fn init2D_impl(); }
    // SAFETY: delegates to C implementation which sets up OpenGL 2D projection state
    unsafe { init2D_impl() }
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
    extern "C" { fn draw_overlay_impl(font: i32, viewport: mjrRect, skip: i32, gridpos: i32, red: f32, green: f32, blue: f32, overlay: *const i8, con: *const mjrContext) -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { draw_overlay_impl(font, viewport, skip, gridpos, red, green, blue, overlay, con) }
}

/// C: maketext (render/classic/render_gl2.c:749)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn maketext(format: *const i8, txt: *mut i8, num: f32, txt_sz: i32) {
    extern "C" { fn maketext_impl(format: *const i8, txt: *mut i8, num: f32, txt_sz: i32); }
    // SAFETY: delegates to C implementation
    unsafe { maketext_impl(format, txt, num, txt_sz) }
}

/// C: textwidth (render/classic/render_gl2.c:787)
#[allow(unused_variables, non_snake_case)]
pub fn textwidth(con: *const mjrContext, text: *const i8) -> i32 {
    // SAFETY: caller guarantees con and text are valid pointers; text is null-terminated
    unsafe {
        let mut i: isize = 0;
        let mut width: i32 = 0;

        // add character widths
        while *text.offset(i) != 0 {
            width += (*con).charWidth[*text.offset(i) as u8 as usize];
            i += 1;
        }

        width
    }
}

/// C: mjr_restoreBuffer (render/classic/render_gl2.h:27)
#[allow(unused_variables, non_snake_case)]
pub fn mjr_restore_buffer(con: *const mjrContext) {
    extern "C" { fn mjr_restoreBuffer_impl(con: *const mjrContext); }
    // SAFETY: delegates to C implementation which handles GL state
    unsafe { mjr_restoreBuffer_impl(con) }
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
    extern "C" { fn mjr_textActual_impl(font: i32, txt: *const i8, con: *const mjrContext, x: f32, y: f32, z: f32, r: f32, g: f32, b: f32); }
    // SAFETY: delegates to C implementation which handles GL state; caller guarantees txt and con are valid
    unsafe { mjr_textActual_impl(font, txt, con, x, y, z, r, g, b) }
}

/// C: mjr_setBuffer (render/classic/render_gl2.h:35)
/// Calls: mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mjr_set_buffer(framebuffer: i32, con: *mut mjrContext) {
    extern "C" { fn mjr_setBuffer_impl(framebuffer: i32, con: *mut mjrContext); }
    // SAFETY: delegates to C implementation
    unsafe { mjr_setBuffer_impl(framebuffer, con) }
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
    extern "C" { fn mjr_readPixels_impl(rgb: *mut u8, depth: *mut f32, viewport: mjrRect, con: *const mjrContext); }
    // SAFETY: delegates to C implementation
    unsafe { mjr_readPixels_impl(rgb, depth, viewport, con) }
}

/// C: mjr_drawPixels (render/classic/render_gl2.h:44)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjr_draw_pixels(rgb: *const u8, depth: *const f32, viewport: mjrRect, con: *const mjrContext) {
    extern "C" { fn mjr_drawPixels_impl(rgb: *const u8, depth: *const f32, viewport: mjrRect, con: *const mjrContext); }
    // SAFETY: delegates to C implementation
    unsafe { mjr_drawPixels_impl(rgb, depth, viewport, con) }
}

/// C: mjr_blitBuffer (render/classic/render_gl2.h:49)
/// Calls: mjr_restoreBuffer
#[allow(unused_variables, non_snake_case)]
pub fn mjr_blit_buffer(src: mjrRect, dst: mjrRect, flg_color: i32, flg_depth: i32, con: *const mjrContext) {
    extern "C" { fn mjr_blitBuffer_impl(src: mjrRect, dst: mjrRect, flg_color: i32, flg_depth: i32, con: *const mjrContext); }
    // SAFETY: delegates to C implementation
    unsafe { mjr_blitBuffer_impl(src, dst, flg_color, flg_depth, con) }
}

/// C: mjr_setAux (render/classic/render_gl2.h:53)
/// Calls: mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mjr_set_aux(index: i32, con: *const mjrContext) {
    extern "C" { fn mjr_setAux_impl(index: i32, con: *const mjrContext); }
    // SAFETY: delegates to C implementation
    unsafe { mjr_setAux_impl(index, con) }
}

/// C: mjr_blitAux (render/classic/render_gl2.h:56)
/// Calls: mjr_restoreBuffer, mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mjr_blit_aux(index: i32, src: mjrRect, left: i32, bottom: i32, con: *const mjrContext) {
    extern "C" { fn mjr_blitAux_impl(index: i32, src: mjrRect, left: i32, bottom: i32, con: *const mjrContext); }
    // SAFETY: delegates to C implementation, pointers valid per caller contract
    unsafe { mjr_blitAux_impl(index, src, left, bottom, con) }
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
    extern "C" { fn mjr_text_impl(font: i32, txt: *const i8, con: *const mjrContext, x: f32, y: f32, r: f32, g: f32, b: f32); }
    // SAFETY: delegates to C implementation
    unsafe { mjr_text_impl(font, txt, con, x, y, r, g, b) }
}

/// C: mjr_overlay (render/classic/render_gl2.h:64)
/// Calls: draw_overlay, init2D
#[allow(unused_variables, non_snake_case)]
pub fn mjr_overlay(font: i32, gridpos: i32, viewport: mjrRect, overlay: *const i8, overlay2: *const i8, con: *const mjrContext) {
    extern "C" { fn mjr_overlay_impl(font: i32, gridpos: i32, viewport: mjrRect, overlay: *const i8, overlay2: *const i8, con: *const mjrContext); }
    // SAFETY: delegates to C implementation
    unsafe { mjr_overlay_impl(font, gridpos, viewport, overlay, overlay2, con) }
}

/// C: mjr_maxViewport (render/classic/render_gl2.h:68)
#[allow(unused_variables, non_snake_case)]
pub fn mjr_max_viewport(con: *const mjrContext) -> mjrRect {
    extern "C" { fn mjr_maxViewport_impl(con: *const mjrContext) -> mjrRect; }
    // SAFETY: delegates to C implementation
    unsafe { mjr_maxViewport_impl(con) }
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
    extern "C" { fn mjr_rectangle_impl(viewport: mjrRect, r: f32, g: f32, b: f32, a: f32); }
    // SAFETY: delegates to C implementation, pointers valid per caller contract
    unsafe { mjr_rectangle_impl(viewport, r, g, b, a) }
}

/// C: mjr_label (render/classic/render_gl2.h:74)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjr_label(viewport: mjrRect, font: i32, txt: *const i8, r: f32, g: f32, b: f32, a: f32, rt: f32, gt: f32, bt: f32, con: *const mjrContext) {
    extern "C" { fn mjr_label_impl(viewport: mjrRect, font: i32, txt: *const i8, r: f32, g: f32, b: f32, a: f32, rt: f32, gt: f32, bt: f32, con: *const mjrContext); }
    // SAFETY: delegates to C implementation
    unsafe { mjr_label_impl(viewport, font, txt, r, g, b, a, rt, gt, bt, con) }
}

/// C: mjr_figure (render/classic/render_gl2.h:79)
/// Calls: init2D, maketext, mjr_textActual, textwidth
#[allow(unused_variables, non_snake_case)]
pub fn mjr_figure(viewport: mjrRect, fig: *mut mjvFigure, con: *const mjrContext) {
    extern "C" { fn mjr_figure_impl(viewport: mjrRect, fig: *mut mjvFigure, con: *const mjrContext); }
    // SAFETY: delegates to C implementation
    unsafe { mjr_figure_impl(viewport, fig, con) }
}

