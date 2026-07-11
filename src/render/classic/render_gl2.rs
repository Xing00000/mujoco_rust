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
    extern "C" {
        fn glDisable(cap: u32);
        fn glEnable(cap: u32);
        fn glShadeModel(mode: u32);
        fn glBlendFunc(sfactor: u32, dfactor: u32);
        fn glPolygonMode(face: u32, mode: u32);
        fn glMatrixMode(mode: u32);
        fn glLoadIdentity();
        fn glOrtho(left: f64, right: f64, bottom: f64, top: f64, near: f64, far: f64);
        fn glViewport(x: i32, y: i32, width: i32, height: i32);
        fn glBegin(mode: u32);
        fn glEnd();
        fn glColor4f(r: f32, g: f32, b: f32, a: f32);
        fn glColor3f(r: f32, g: f32, b: f32);
        fn glVertex2i(x: i32, y: i32);
        fn glRasterPos2i(x: i32, y: i32);
        fn glListBase(base: u32);
        fn glCallLists(n: i32, r#type: u32, lists: *const i8);
        fn strlen(s: *const i8) -> usize;
    }

    const GL_NORMALIZE: u32 = 0x0BA1;
    const GL_DEPTH_TEST: u32 = 0x0B71;
    const GL_CULL_FACE: u32 = 0x0B44;
    const GL_LIGHTING: u32 = 0x0B50;
    const GL_COLOR_MATERIAL: u32 = 0x0B57;
    const GL_FLAT: u32 = 0x1D00;
    const GL_BLEND: u32 = 0x0BE2;
    const GL_SRC_ALPHA: u32 = 0x0302;
    const GL_ONE_MINUS_SRC_ALPHA: u32 = 0x0303;
    const GL_FRONT_AND_BACK: u32 = 0x0408;
    const GL_FILL: u32 = 0x1B02;
    const GL_PROJECTION: u32 = 0x1701;
    const GL_MODELVIEW: u32 = 0x1700;
    const GL_QUADS: u32 = 0x0007;
    const GL_UNSIGNED_BYTE: u32 = 0x1401;
    const mjFONT_BIG: i32 = 2;

    // empty viewport: nothing to do
    if viewport.width <= 0 || viewport.height <= 0 {
        return;
    }

    // SAFETY: all GL calls require valid OpenGL context. Caller guarantees
    // context is current. con pointer is valid for reads.
    unsafe {
        // set OpenGL options
        glDisable(GL_NORMALIZE);
        glDisable(GL_DEPTH_TEST);
        glDisable(GL_CULL_FACE);
        glDisable(GL_LIGHTING);
        glDisable(GL_COLOR_MATERIAL);
        glShadeModel(GL_FLAT);
        glEnable(GL_BLEND);
        glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);
        glPolygonMode(GL_FRONT_AND_BACK, GL_FILL);

        // standard 2D projection, in framebuffer units
        glMatrixMode(GL_PROJECTION);
        glLoadIdentity();
        glOrtho(0.0, viewport.width as f64, 0.0, viewport.height as f64, -1.0, 1.0);
        glMatrixMode(GL_MODELVIEW);
        glLoadIdentity();

        // set viewport
        glViewport(viewport.left, viewport.bottom, viewport.width, viewport.height);

        // get sizes
        let W = viewport.width;
        let H = viewport.height;

        // render
        glBegin(GL_QUADS);
        glColor4f(r, g, b, a);
        glVertex2i(0, 0);
        glVertex2i(W, 0);
        glVertex2i(W, H);
        glVertex2i(0, H);
        glEnd();

        // draw text
        if !txt.is_null() && (*con).rangeFont != 0 {
            // compute width
            let mut i: i32 = 0;
            let mut width: i32 = 0;
            if font == mjFONT_BIG {
                while *txt.offset(i as isize) != 0 {
                    width += (*con).charWidthBig[*txt.offset(i as isize) as u8 as usize];
                    i += 1;
                }
            } else {
                while *txt.offset(i as isize) != 0 {
                    width += (*con).charWidth[*txt.offset(i as isize) as u8 as usize];
                    i += 1;
                }
            }

            // compute center
            let cx = (W - width) / 2;
            let cy = (H - (if font == mjFONT_BIG { (*con).charHeightBig } else { (*con).charHeight })) / 2;

            // draw
            glListBase(if font == mjFONT_BIG { (*con).baseFontBig } else { (*con).baseFontNormal });
            glColor3f(rt, gt, bt);
            glRasterPos2i(if 0 > cx { 0 } else { cx }, cy);
            glCallLists(strlen(txt) as i32, GL_UNSIGNED_BYTE, txt);
        }
    }
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

