//! Port of: render/classic/render_gl2.c
//! IR hash: 9614bd3d92e7766b
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
    todo!() // flipDepthIfRequired
}

/// C: init2D (render/classic/render_gl2.c:407)
#[allow(unused_variables, non_snake_case)]
pub fn init2d() {
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

    extern "C" {
        fn glDisable(cap: u32);
        fn glShadeModel(mode: u32);
        fn glBlendFunc(sfactor: u32, dfactor: u32);
        fn glPolygonMode(face: u32, mode: u32);
        fn glMatrixMode(mode: u32);
        fn glLoadIdentity();
        fn glOrtho(left: f64, right: f64, bottom: f64, top: f64, near: f64, far: f64);
    }

    // SAFETY: OpenGL state-setting calls. No pointers involved. Linked from mujoco C library.
    unsafe {
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
    const mjFONT_BIG: i32 = 2;
    const mjMAXOVERLAY: i32 = 500;
    const mjGRID_TOPLEFT: i32 = 0;
    const mjGRID_TOPRIGHT: i32 = 1;
    const mjGRID_BOTTOMLEFT: i32 = 2;
    const mjGRID_BOTTOMRIGHT: i32 = 3;
    const mjGRID_TOP: i32 = 4;
    const mjGRID_BOTTOM: i32 = 5;
    const mjGRID_LEFT: i32 = 6;
    const mjGRID_RIGHT: i32 = 7;
    const GL_PROJECTION: u32 = 0x1701;
    const GL_MODELVIEW: u32 = 0x1700;
    const GL_BLEND: u32 = 0x0BE2;
    const GL_QUADS: u32 = 0x0007;

    extern "C" {
        fn glViewport(x: i32, y: i32, width: i32, height: i32);
        fn glMatrixMode(mode: u32);
        fn glLoadIdentity();
        fn glOrtho(left: f64, right: f64, bottom: f64, top: f64, near: f64, far: f64);
        fn glEnable(cap: u32);
        fn glDisable(cap: u32);
        fn glColor4d(r: f64, g: f64, b: f64, a: f64);
        fn glBegin(mode: u32);
        fn glEnd();
        fn glNormal3d(nx: f64, ny: f64, nz: f64);
        fn glVertex2d(x: f64, y: f64);
        fn strlen(s: *const i8) -> usize;
    }

    // SAFETY: con, overlay are valid pointers; GL functions linked from mujoco (caller contract)
    unsafe {
        let flg_big: i32 = if font == mjFONT_BIG { 1 } else { 0 };
        let PAD: i32 = 5;
        let strlen_overlay = strlen(overlay) as i32;
        let sz: i32 = if mjMAXOVERLAY < strlen_overlay { mjMAXOVERLAY } else { strlen_overlay };
        let mut text: [i8; 500] = [0; 500];

        // count rows and columns of text rectangle in pixels
        let mut nr: i32 = if flg_big != 0 { (*con).charHeightBig } else { (*con).charHeight };
        let mut ncthis: i32 = 0;
        let mut nc: i32 = 0;
        let mut i: i32 = 0;
        while i < sz {
            if *overlay.offset(i as isize) != b'\n' as i8 {
                ncthis += if flg_big != 0 {
                    (*con).charWidthBig[*overlay.offset(i as isize) as u8 as usize]
                } else {
                    (*con).charWidth[*overlay.offset(i as isize) as u8 as usize]
                };
                if ncthis > nc { nc = ncthis; }
            } else {
                nr += PAD + if flg_big != 0 { (*con).charHeightBig } else { (*con).charHeight };
                ncthis = 0;
            }
            i += 1;
        }

        // viewport width and height
        let W: i32 = PAD + nc + 8;
        let H: i32 = PAD + nr;

        // set viewport to specific grid position
        match gridpos {
            mjGRID_TOPLEFT => {
                glViewport(viewport.left + skip + PAD,
                           viewport.bottom + viewport.height - 1 - PAD - H, W, H);
            }
            mjGRID_TOPRIGHT => {
                glViewport(viewport.left + viewport.width - skip - 1 - PAD - W,
                           viewport.bottom + viewport.height - 1 - PAD - H, W, H);
            }
            mjGRID_BOTTOMLEFT => {
                glViewport(viewport.left + skip + PAD,
                           viewport.bottom + PAD, W, H);
            }
            mjGRID_BOTTOMRIGHT => {
                glViewport(viewport.left + viewport.width - skip - 1 - PAD - W,
                           viewport.bottom + PAD, W, H);
            }
            mjGRID_TOP => {
                glViewport(viewport.left + (viewport.width - skip - W) / 2 - 1 - PAD,
                           viewport.bottom + viewport.height - 1 - PAD - H, W, H);
            }
            mjGRID_BOTTOM => {
                glViewport(viewport.left + (viewport.width - skip - W) / 2 - 1 - PAD,
                           viewport.bottom + PAD, W, H);
            }
            mjGRID_LEFT => {
                glViewport(viewport.left + skip + PAD,
                           viewport.bottom + (viewport.height - H) / 2 - 1 - PAD, W, H);
            }
            mjGRID_RIGHT => {
                glViewport(viewport.left + viewport.width - skip - 1 - PAD - W,
                           viewport.bottom + (viewport.height - H) / 2 - 1 - PAD, W, H);
            }
            _ => {}
        }

        // set projection in pixels
        glMatrixMode(GL_PROJECTION);
        glLoadIdentity();
        glOrtho(0.0, (W - 1) as f64, 0.0, (H - 1) as f64, -1.0, 1.0);
        glMatrixMode(GL_MODELVIEW);
        glLoadIdentity();

        // blend with black background
        glEnable(GL_BLEND);
        glColor4d(0.0, 0.0, 0.0, 0.5);
        glBegin(GL_QUADS);
        glNormal3d(0.0, 0.0, 1.0);
        glVertex2d(0.0, 0.0);
        glVertex2d(0.0, H as f64);
        glVertex2d(W as f64, H as f64);
        glVertex2d(W as f64, 0.0);
        glEnd();
        glDisable(GL_BLEND);

        // draw text line by line
        nr = if flg_big != 0 { (*con).charHeightBig } else { (*con).charHeight };
        let mut pos: i32 = 0;
        i = 0;
        while i < sz {
            if *overlay.offset(i as isize) == b'\n' as i8 || i == sz - 1 {
                if *overlay.offset(i as isize) == b'\n' as i8 {
                    text[pos as usize] = 0;
                } else {
                    text[pos as usize] = *overlay.offset(i as isize);
                    text[pos as usize + 1] = 0;
                }
                mjr_text_actual(font, text.as_ptr(), con, 3.0, (H - nr) as f32, 0.0, red, green, blue);

                nr += PAD + if flg_big != 0 { (*con).charHeightBig } else { (*con).charHeight };
                pos = 0;
            } else {
                text[pos as usize] = *overlay.offset(i as isize);
                pos += 1;
            }
            i += 1;
        }

        W - 2
    }
}

/// C: maketext (render/classic/render_gl2.c:749)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn maketext(format: *const i8, txt: *mut i8, num: f32, txt_sz: i32) {
    todo!() // maketext
}

/// C: textwidth (render/classic/render_gl2.c:787)
#[allow(unused_variables, non_snake_case)]
pub fn textwidth(con: *const mjrContext, text: *const i8) -> i32 {
    // SAFETY: con is a valid mjrContext pointer; text is a valid null-terminated C string (caller contract)
    unsafe {
        let mut i: i32 = 0;
        let mut width: i32 = 0;

        while *text.offset(i as isize) != 0 {
            width += (*con).charWidth[*text.offset(i as isize) as u8 as usize];
            i += 1;
        }

        width
    }
}

/// C: mjr_restoreBuffer (render/classic/render_gl2.h:27)
#[allow(unused_variables, non_snake_case)]
pub fn mjr_restore_buffer(con: *const mjrContext) {
    todo!() // mjr_restoreBuffer
}

/// C: mjr_textActual (render/classic/render_gl2.h:30)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjr_text_actual(font: i32, txt: *const i8, con: *const mjrContext, x: f32, y: f32, z: f32, r: f32, g: f32, b: f32) {
    todo!() // mjr_textActual
}

/// C: mjr_setBuffer (render/classic/render_gl2.h:35)
/// Calls: mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mjr_set_buffer(framebuffer: i32, con: *mut mjrContext) {
    todo!() // mjr_setBuffer
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
    todo!() // mjr_readPixels
}

/// C: mjr_drawPixels (render/classic/render_gl2.h:44)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjr_draw_pixels(rgb: *const u8, depth: *const f32, viewport: mjrRect, con: *const mjrContext) {
    todo!() // mjr_drawPixels
}

/// C: mjr_blitBuffer (render/classic/render_gl2.h:49)
/// Calls: mjr_restoreBuffer
#[allow(unused_variables, non_snake_case)]
pub fn mjr_blit_buffer(src: mjrRect, dst: mjrRect, flg_color: i32, flg_depth: i32, con: *const mjrContext) {
    todo!() // mjr_blitBuffer
}

/// C: mjr_setAux (render/classic/render_gl2.h:53)
/// Calls: mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mjr_set_aux(index: i32, con: *const mjrContext) {
    todo!() // mjr_setAux
}

/// C: mjr_blitAux (render/classic/render_gl2.h:56)
/// Calls: mjr_restoreBuffer, mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mjr_blit_aux(index: i32, src: mjrRect, left: i32, bottom: i32, con: *const mjrContext) {
    todo!() // mjr_blitAux
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
    todo!() // mjr_text
}

/// C: mjr_overlay (render/classic/render_gl2.h:64)
/// Calls: draw_overlay, init2D
#[allow(unused_variables, non_snake_case)]
pub fn mjr_overlay(font: i32, gridpos: i32, viewport: mjrRect, overlay: *const i8, overlay2: *const i8, con: *const mjrContext) {
    todo!() // mjr_overlay
}

/// C: mjr_maxViewport (render/classic/render_gl2.h:68)
#[allow(unused_variables, non_snake_case)]
pub fn mjr_max_viewport(con: *const mjrContext) -> mjrRect {
    todo!() // mjr_maxViewport
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
    todo!() // mjr_rectangle
}

/// C: mjr_label (render/classic/render_gl2.h:74)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mjr_label(viewport: mjrRect, font: i32, txt: *const i8, r: f32, g: f32, b: f32, a: f32, rt: f32, gt: f32, bt: f32, con: *const mjrContext) {
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
        fn glListBase(base: u32);
        fn glRasterPos2i(x: i32, y: i32);
        fn glCallLists(n: i32, type_: u32, lists: *const i8);
        fn strlen(s: *const i8) -> usize;
    }

    if viewport.width <= 0 || viewport.height <= 0 {
        return;
    }

    // SAFETY: Direct 1:1 translation of C OpenGL calls. All GL functions are linked
    // from the mujoco C library. viewport/con are valid per caller contract.
    unsafe {
        glDisable(GL_NORMALIZE);
        glDisable(GL_DEPTH_TEST);
        glDisable(GL_CULL_FACE);
        glDisable(GL_LIGHTING);
        glDisable(GL_COLOR_MATERIAL);
        glShadeModel(GL_FLAT);
        glEnable(GL_BLEND);
        glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);
        glPolygonMode(GL_FRONT_AND_BACK, GL_FILL);

        glMatrixMode(GL_PROJECTION);
        glLoadIdentity();
        glOrtho(0.0, viewport.width as f64, 0.0, viewport.height as f64, -1.0, 1.0);
        glMatrixMode(GL_MODELVIEW);
        glLoadIdentity();

        glViewport(viewport.left, viewport.bottom, viewport.width, viewport.height);

        let W: i32 = viewport.width;
        let H: i32 = viewport.height;

        glBegin(GL_QUADS);
        glColor4f(r, g, b, a);
        glVertex2i(0, 0);
        glVertex2i(W, 0);
        glVertex2i(W, H);
        glVertex2i(0, H);
        glEnd();

        if !txt.is_null() && (*con).rangeFont != 0 {
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

            let cx: i32 = (W - width) / 2;
            let cy: i32 = (H - if font == mjFONT_BIG { (*con).charHeightBig } else { (*con).charHeight }) / 2;

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
    todo!() // mjr_figure
}

