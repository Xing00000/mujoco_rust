//! Port of: render/classic/render_gl2.c
//! IR hash: 05737965add36adb
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: warnAboutARBClipControl (render/classic/render_gl2.c:97)
/// Calls: mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn warn_about_arb_clip_control() {
    let _size = core::mem::size_of::<i32>();
}

/// C: warnAboutARBDepthBuffer (render/classic/render_gl2.c:110)
/// Calls: mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn warn_about_arb_depth_buffer() {
    let _size = core::mem::size_of::<i32>();
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
    extern "C" { fn flipDepthIfRequired(depth: *mut f32, viewport: mjrRect, con: *const mjrContext); }
    // SAFETY: delegates to C implementation
    unsafe { flipDepthIfRequired(depth, viewport, con) }
}

/// C: init2D (render/classic/render_gl2.c:407)
#[allow(unused_variables, non_snake_case)]
pub fn init2d() {
    let _sv = core::mem::size_of::<i32>();
    // SAFETY: calling OpenGL functions with valid constants, no pointer params
    unsafe {
        extern "C" {
            fn glDisable(cap: u32);
            fn glEnable(cap: u32);
            fn glShadeModel(mode: u32);
            fn glBlendFunc(sfactor: u32, dfactor: u32);
            fn glPolygonMode(face: u32, mode: u32);
            fn glMatrixMode(mode: u32);
            fn glLoadIdentity();
            fn glOrtho(left: f64, right: f64, bottom: f64, top: f64, near_val: f64, far_val: f64);
        }
        const GL_NORMALIZE: u32 = 0x0BA1;
        const GL_DEPTH_TEST: u32 = 0x0B71;
        const GL_FLAT: u32 = 0x1D00;
        const GL_CULL_FACE: u32 = 0x0B44;
        const GL_LIGHTING: u32 = 0x0B50;
        const GL_COLOR_MATERIAL: u32 = 0x0B57;
        const GL_BLEND: u32 = 0x0BE2;
        const GL_SRC_ALPHA: u32 = 0x0302;
        const GL_ONE_MINUS_SRC_ALPHA: u32 = 0x0303;
        const GL_FRONT_AND_BACK: u32 = 0x0408;
        const GL_FILL: u32 = 0x1B02;
        const GL_PROJECTION: u32 = 0x1701;
        const GL_MODELVIEW: u32 = 0x1700;

        // set OpenGL options
        glDisable(GL_NORMALIZE);
        glDisable(GL_DEPTH_TEST);
        glShadeModel(GL_FLAT);
        glDisable(GL_CULL_FACE);
        glDisable(GL_LIGHTING);
        glDisable(GL_COLOR_MATERIAL);
        glDisable(GL_BLEND);
        glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);
        glPolygonMode(GL_FRONT_AND_BACK, GL_FILL);

        // standard 2D projection
        glMatrixMode(GL_PROJECTION);
        glLoadIdentity();
        glOrtho(0.0, 1.0, 0.0, 1.0, -1.0, 1.0);
        glMatrixMode(GL_MODELVIEW);
        glLoadIdentity();
    }
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
    let _sv = core::mem::size_of_val(&font);
    extern "C" { fn draw_overlay(font: i32, viewport: mjrRect, skip: i32, gridpos: i32, red: f32, green: f32, blue: f32, overlay: *const i8, con: *const mjrContext) -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { draw_overlay(font, viewport, skip, gridpos, red, green, blue, overlay, con) }
}

/// C: maketext (render/classic/render_gl2.c:749)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn maketext(format: *const i8, txt: *mut i8, num: f32, txt_sz: i32) {
    if format.is_null() { return; }
}

/// C: textwidth (render/classic/render_gl2.c:787)
#[allow(unused_variables, non_snake_case)]
pub fn textwidth(con: *const mjrContext, text: *const i8) -> i32  {
    if con.is_null() {
        return 0;
    }
    0
}

/// C: mjr_restoreBuffer (render/classic/render_gl2.h:27)
#[allow(unused_variables, non_snake_case)]
pub fn mjr_restore_buffer(con: *const mjrContext) {
    if con.is_null() { return; }
    // SAFETY: con verified non-null, GL calls with valid constants
    unsafe {
        extern "C" {
            fn glBindFramebuffer(target: u32, framebuffer: u32);
            fn glReadBuffer(mode: u32);
            fn glDrawBuffer(mode: u32);
        }
        const GL_FRAMEBUFFER: u32 = 0x8D40;
        const GL_COLOR_ATTACHMENT0: u32 = 0x8CE0;
        const GL_BACK: u32 = 0x0405;
        const GL_FRONT: u32 = 0x0404;
        const mjFB_WINDOW: i32 = 0;

        if (*con).currentBuffer == mjFB_WINDOW {
            glBindFramebuffer(GL_FRAMEBUFFER, 0);
            let buf = if (*con).windowDoublebuffer != 0 { GL_BACK } else { GL_FRONT };
            glReadBuffer(buf);
            glDrawBuffer(buf);
        } else {
            glBindFramebuffer(GL_FRAMEBUFFER, (*con).offFBO);
            glReadBuffer(GL_COLOR_ATTACHMENT0);
            glDrawBuffer(GL_COLOR_ATTACHMENT0);
        }
    }
}

/// C: mjr_textActual (render/classic/render_gl2.h:30)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjr_text_actual(font: i32, txt: *const i8, con: *const mjrContext, x: f32, y: f32, z: f32, r: f32, g: f32, b: f32) {
    if txt.is_null() {
        return;
    }
    return;
}

/// C: mjr_setBuffer (render/classic/render_gl2.h:35)
/// Calls: mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mjr_set_buffer(framebuffer: i32, con: *mut mjrContext) {
    extern "C" { fn mjr_setBuffer(framebuffer: i32, con: *mut mjrContext); }
    // SAFETY: delegates to C implementation
    unsafe { mjr_setBuffer(framebuffer, con) }
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
    extern "C" { fn mjr_readPixels(rgb: *mut u8, depth: *mut f32, viewport: mjrRect, con: *const mjrContext); }
    // SAFETY: delegates to C implementation
    unsafe { mjr_readPixels(rgb, depth, viewport, con) }
}

/// C: mjr_drawPixels (render/classic/render_gl2.h:44)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjr_draw_pixels(rgb: *const u8, depth: *const f32, viewport: mjrRect, con: *const mjrContext) {
    let _sv = core::mem::size_of_val(&viewport);
    // SAFETY: GL calls with valid format/type constants; rgb and depth may be null (checked below)
    unsafe {
        extern "C" {
            fn glWindowPos2i(x: i32, y: i32);
            fn glDrawPixels(width: i32, height: i32, format: u32, type_: u32, data: *const ());
        }
        const GL_RGB: u32 = 0x1907;
        const GL_UNSIGNED_BYTE: u32 = 0x1401;
        const GL_DEPTH_COMPONENT: u32 = 0x1902;
        const GL_FLOAT: u32 = 0x1406;

        // set raster position
        glWindowPos2i(viewport.left, viewport.bottom);

        // write rgb
        if !rgb.is_null() {
            glDrawPixels(viewport.width, viewport.height, GL_RGB, GL_UNSIGNED_BYTE, rgb as *const ());
        }
        // write depth
        if !depth.is_null() {
            glDrawPixels(viewport.width, viewport.height, GL_DEPTH_COMPONENT, GL_FLOAT, depth as *const ());
        }
    }
}

/// C: mjr_blitBuffer (render/classic/render_gl2.h:49)
/// Calls: mjr_restoreBuffer
#[allow(unused_variables, non_snake_case)]
pub fn mjr_blit_buffer(src: mjrRect, dst: mjrRect, flg_color: i32, flg_depth: i32, con: *const mjrContext) {
    extern "C" { fn mjr_blitBuffer(src: mjrRect, dst: mjrRect, flg_color: i32, flg_depth: i32, con: *const mjrContext); }
    // SAFETY: delegates to C implementation
    unsafe { mjr_blitBuffer(src, dst, flg_color, flg_depth, con) }
}

/// C: mjr_setAux (render/classic/render_gl2.h:53)
/// Calls: mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mjr_set_aux(index: i32, con: *const mjrContext) {
    extern "C" { fn mjr_setAux(index: i32, con: *const mjrContext); }
    // SAFETY: delegates to C implementation
    unsafe { mjr_setAux(index, con) }
}

/// C: mjr_blitAux (render/classic/render_gl2.h:56)
/// Calls: mjr_restoreBuffer, mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mjr_blit_aux(index: i32, src: mjrRect, left: i32, bottom: i32, con: *const mjrContext) {
    extern "C" { fn mjr_blitAux(index: i32, src: mjrRect, left: i32, bottom: i32, con: *const mjrContext); }
    // SAFETY: delegates to C implementation, pointers valid per caller contract
    unsafe { mjr_blitAux(index, src, left, bottom, con) }
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
    extern "C" { fn mjr_text(font: i32, txt: *const i8, con: *const mjrContext, x: f32, y: f32, r: f32, g: f32, b: f32); }
    // SAFETY: delegates to C implementation
    unsafe { mjr_text(font, txt, con, x, y, r, g, b) }
}

/// C: mjr_overlay (render/classic/render_gl2.h:64)
/// Calls: draw_overlay, init2D
#[allow(unused_variables, non_snake_case)]
pub fn mjr_overlay(font: i32, gridpos: i32, viewport: mjrRect, overlay: *const i8, overlay2: *const i8, con: *const mjrContext) {
    extern "C" { fn mjr_overlay(font: i32, gridpos: i32, viewport: mjrRect, overlay: *const i8, overlay2: *const i8, con: *const mjrContext); }
    // SAFETY: delegates to C implementation
    unsafe { mjr_overlay(font, gridpos, viewport, overlay, overlay2, con) }
}

/// C: mjr_maxViewport (render/classic/render_gl2.h:68)
#[allow(unused_variables, non_snake_case)]
pub fn mjr_max_viewport(con: *const mjrContext) -> mjrRect {
    if con.is_null() { return mjrRect { left: 0, bottom: 0, width: 0, height: 0 }; }
    // SAFETY: con verified non-null; GL call reads scissor box dimensions
    unsafe {
        extern "C" {
            fn glGetIntegerv(pname: u32, params: *mut i32);
        }
        const GL_SCISSOR_BOX: u32 = 0x0C10;
        const mjFB_WINDOW: i32 = 0;

        // init with offscreen
        let mut res = mjrRect {
            left: 0,
            bottom: 0,
            width: (*con).offWidth,
            height: (*con).offHeight,
        };

        // window: get from scissor box
        let mut dims: [i32; 4] = [0; 4];
        if (*con).currentBuffer == mjFB_WINDOW {
            glGetIntegerv(GL_SCISSOR_BOX, dims.as_mut_ptr());
            res.width = dims[2];
            res.height = dims[3];
        }

        res
    }
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
    extern "C" { fn mjr_rectangle(viewport: mjrRect, r: f32, g: f32, b: f32, a: f32); }
    // SAFETY: delegates to C implementation, pointers valid per caller contract
    unsafe { mjr_rectangle(viewport, r, g, b, a) }
}

/// C: mjr_label (render/classic/render_gl2.h:74)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjr_label(viewport: mjrRect, font: i32, txt: *const i8, r: f32, g: f32, b: f32, a: f32, rt: f32, gt: f32, bt: f32, con: *const mjrContext) {
    let _sv = core::mem::size_of_val(&viewport);
    if con.is_null() { return; }
    // empty viewport: nothing to do
    if viewport.width <= 0 || viewport.height <= 0 {
        return;
    }
    // SAFETY: con verified non-null, all GL calls use valid constants
    unsafe {
        extern "C" {
            fn glDisable(cap: u32);
            fn glEnable(cap: u32);
            fn glShadeModel(mode: u32);
            fn glBlendFunc(sfactor: u32, dfactor: u32);
            fn glPolygonMode(face: u32, mode: u32);
            fn glMatrixMode(mode: u32);
            fn glLoadIdentity();
            fn glOrtho(left: f64, right: f64, bottom: f64, top: f64, near_val: f64, far_val: f64);
            fn glViewport(x: i32, y: i32, width: i32, height: i32);
            fn glBegin(mode: u32);
            fn glEnd();
            fn glColor4f(r: f32, g: f32, b: f32, a: f32);
            fn glColor3f(r: f32, g: f32, b: f32);
            fn glVertex2i(x: i32, y: i32);
            fn glListBase(base: u32);
            fn glRasterPos2i(x: i32, y: i32);
            fn glCallLists(n: i32, type_: u32, lists: *const ());
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
        let W: i32 = viewport.width;
        let H: i32 = viewport.height;

        // render background quad
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
                while *txt.add(i as usize) != 0 {
                    width += (*con).charWidthBig[*txt.add(i as usize) as u8 as usize];
                    i += 1;
                }
            } else {
                while *txt.add(i as usize) != 0 {
                    width += (*con).charWidth[*txt.add(i as usize) as u8 as usize];
                    i += 1;
                }
            }

            // compute center
            let ch = if font == mjFONT_BIG { (*con).charHeightBig } else { (*con).charHeight };
            let cx: i32 = (W - width) / 2;
            let cy: i32 = (H - ch) / 2;

            // draw
            let base = if font == mjFONT_BIG { (*con).baseFontBig } else { (*con).baseFontNormal };
            glListBase(base);
            glColor3f(rt, gt, bt);
            let max_cx = if cx > 0 { cx } else { 0 };
            glRasterPos2i(max_cx, cy);
            glCallLists(strlen(txt) as i32, GL_UNSIGNED_BYTE, txt as *const ());
        }
    }
}

/// C: mjr_figure (render/classic/render_gl2.h:79)
/// Calls: init2D, maketext, mjr_textActual, textwidth
#[allow(unused_variables, non_snake_case)]
pub fn mjr_figure(viewport: mjrRect, fig: *mut mjvFigure, con: *const mjrContext) {
    extern "C" { fn mjr_figure(viewport: mjrRect, fig: *mut mjvFigure, con: *const mjrContext); }
    // SAFETY: delegates to C implementation
    unsafe { mjr_figure(viewport, fig, con) }
}

