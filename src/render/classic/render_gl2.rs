//! Port of: render/classic/render_gl2.c
//! IR hash: bd605ac8158c32d6
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
    extern "C" {
        fn snprintf(s: *mut i8, n: usize, fmt: *const i8, ...) -> i32;
        fn strlen(s: *const i8) -> usize;
    }

    // SAFETY: format is a valid C string, txt points to buffer of at least txt_sz bytes
    unsafe {
        // full text
        snprintf(txt, txt_sz as usize, format, num as f64);

        // locate trailing zeros
        let mut i = strlen(txt) as i32;
        while i > 0 && *txt.add((i - 1) as usize) == b'0' as i8 {
            i -= 1;
        }
        if i <= 1 {
            return;
        }

        // strip if preceding char is '.'
        if *txt.add((i - 1) as usize) == b'.' as i8 {
            *txt.add((i - 1) as usize) = 0;
        }
        // otherwise find earlier '.'
        else {
            // find regular preceding digits
            let mut j = i - 1;
            while j >= 0 && *txt.add(j as usize) >= b'0' as i8 && *txt.add(j as usize) <= b'9' as i8 {
                j -= 1;
            }

            // '.' found: strip
            if j >= 0 && *txt.add(j as usize) == b'.' as i8 {
                *txt.add(i as usize) = 0;
            }
        }
    }
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
    const mjFONT_SHADOW: i32 = 0;
    const mjFONT_BIG: i32 = 2;
    const GL_UNSIGNED_BYTE: u32 = 0x1401;

    extern "C" {
        fn glListBase(base: u32);
        fn glColor4f(r: f32, g: f32, b: f32, a: f32);
        fn glRasterPos3f(x: f32, y: f32, z: f32);
        fn glCallLists(n: i32, type_: u32, lists: *const i8);
        fn strlen(s: *const i8) -> usize;
    }

    // SAFETY: con is a valid mjrContext pointer, txt is a valid C string (caller contract)
    unsafe {
        // return if font is not installed
        if (*con).rangeFont == 0 {
            return;
        }

        // shadow text
        if font == mjFONT_SHADOW {
            // blend shadow with black
            glListBase((*con).baseFontShadow);
            glColor4f(0.0, 0.0, 0.0, 0.5);
            glRasterPos3f(x, y, z);
            glCallLists(strlen(txt) as i32, GL_UNSIGNED_BYTE, txt);

            // render text
            glListBase((*con).baseFontNormal);
            glColor4f(r, g, b, 1.0);
            glRasterPos3f(x, y, z);
            glCallLists(strlen(txt) as i32, GL_UNSIGNED_BYTE, txt);
        }
        // regular text (normal or big)
        else {
            glListBase(if font == mjFONT_BIG { (*con).baseFontBig } else { (*con).baseFontNormal });
            glColor4f(r, g, b, 1.0);
            glRasterPos3f(x, y, z);
            glCallLists(strlen(txt) as i32, GL_UNSIGNED_BYTE, txt);
        }
    }
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
    const mjGRID_TOPLEFT: i32 = 0;
    const mjGRID_BOTTOMLEFT: i32 = 2;

    // empty viewport: nothing to do
    if viewport.width <= 0 || viewport.height <= 0 {
        return;
    }

    // init OpenGL once per overlay, set viewport later
    init2d();

    // SAFETY: overlay, overlay2, con are valid pointers per caller contract.
    // draw_overlay handles all GL calls internally.
    unsafe {
        let mut skip: i32 = 0;

        // two-column
        if !overlay2.is_null() && *overlay2 != 0 {
            // left side
            if gridpos == mjGRID_TOPLEFT || gridpos == mjGRID_BOTTOMLEFT {
                skip = draw_overlay(font, viewport, 0, gridpos, 0.7, 0.7, 0.7, overlay, con);
                draw_overlay(font, viewport, skip, gridpos, 1.0, 1.0, 1.0, overlay2, con);
            }
            // right side
            else {
                skip = draw_overlay(font, viewport, 0, gridpos, 1.0, 1.0, 1.0, overlay2, con);
                draw_overlay(font, viewport, skip, gridpos, 0.7, 0.7, 0.7, overlay, con);
            }
        }
        // one-column
        else {
            draw_overlay(font, viewport, 0, gridpos, 1.0, 1.0, 1.0, overlay, con);
        }
    }
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
    const GL_BLEND: u32 = 0x0BE2;
    const GL_QUADS: u32 = 0x0007;

    extern "C" {
        fn glViewport(x: i32, y: i32, width: i32, height: i32);
        fn glEnable(cap: u32);
        fn glDisable(cap: u32);
        fn glColor4f(r: f32, g: f32, b: f32, a: f32);
        fn glBegin(mode: u32);
        fn glEnd();
        fn glVertex2f(x: f32, y: f32);
    }

    // empty viewport: nothing to do
    if viewport.width <= 0 || viewport.height <= 0 {
        return;
    }

    // init OpenGL, set viewport
    init2d();

    // SAFETY: GL state-setting and draw calls. viewport fields are plain i32. Linked from mujoco C library.
    unsafe {
        glViewport(viewport.left, viewport.bottom, viewport.width, viewport.height);

        // draw rectangle
        glEnable(GL_BLEND);
        glColor4f(r, g, b, a);
        glBegin(GL_QUADS);
        glVertex2f(0.0, 0.0);
        glVertex2f(0.0, 1.0);
        glVertex2f(1.0, 1.0);
        glVertex2f(1.0, 0.0);
        glEnd();
        glDisable(GL_BLEND);
    }
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
    const GL_BLEND: u32 = 0x0BE2;
    const GL_QUADS: u32 = 0x0007;
    const GL_LINES: u32 = 0x0001;
    const GL_LINE_STRIP: u32 = 0x0003;
    const GL_FLOAT: u32 = 0x1406;
    const GL_PROJECTION: u32 = 0x1701;
    const GL_MODELVIEW: u32 = 0x1700;
    const GL_VERTEX_ARRAY: u32 = 0x8074;
    const GL_UNSIGNED_BYTE: u32 = 0x1401;
    const mjMAXLINE: i32 = 100;
    const mjFONT_NORMAL: i32 = 0;
    const mjFONT_SHADOW: i32 = 0;
    const STRING_BUFSIZE: usize = 100;

    extern "C" {
        fn glViewport(x: i32, y: i32, width: i32, height: i32);
        fn glEnable(cap: u32);
        fn glDisable(cap: u32);
        fn glColor4fv(v: *const f32);
        fn glColor4f(r: f32, g: f32, b: f32, a: f32);
        fn glColor3fv(v: *const f32);
        fn glColor3f(r: f32, g: f32, b: f32);
        fn glBegin(mode: u32);
        fn glEnd();
        fn glVertex2d(x: f64, y: f64);
        fn glVertex2f(x: f32, y: f32);
        fn glMatrixMode(mode: u32);
        fn glLoadIdentity();
        fn glOrtho(left: f64, right: f64, bottom: f64, top: f64, near: f64, far: f64);
        fn glEnableClientState(array: u32);
        fn glDisableClientState(array: u32);
        fn glVertexPointer(size: i32, type_: u32, stride: i32, pointer: *const f32);
        fn glLineWidth(width: f32);
        fn glDrawArrays(mode: u32, first: i32, count: i32);
        fn glListBase(base: u32);
        fn glRasterPos3f(x: f32, y: f32, z: f32);
        fn glCallLists(n: i32, type_: u32, lists: *const i8);
        fn snprintf(s: *mut i8, n: usize, fmt: *const i8, ...) -> i32;
        fn powf(base: f32, exp: f32) -> f32;
        fn floorf(x: f32) -> f32;
        fn log10f(x: f32) -> f32;
        fn ceilf(x: f32) -> f32;
        fn fabsf(x: f32) -> f32;
    }

    // empty viewport: nothing to do
    if viewport.width <= 0 || viewport.height <= 0 {
        return;
    }

    // SAFETY: fig and con are valid pointers per caller contract. All GL functions are
    // linked from the mujoco C library. Raw pointer arithmetic on fig fields matches
    // C memory layout (verified by existing translated code pattern).
    unsafe {
        let mut viewport = viewport; // make mutable copy for viewport adjustments
        let mut range: [[f32; 2]; 2] = [[0.0; 2]; 2];
        let mut griddata: [f32; 100] = [0.0; 100];
        let minrange: f32 = 1e-5;
        let offset: f32 = 0.001;
        let PAD: i32 = (*con).charHeight / 2;
        let mut datatxt: [i8; STRING_BUFSIZE] = [0; STRING_BUFSIZE];

        // init OpenGL, set viewport
        init2d();
        glViewport(viewport.left, viewport.bottom, viewport.width, viewport.height);

        // clear background and blend
        glEnable(GL_BLEND);
        glColor4fv((*fig).figurergba.as_ptr());
        glBegin(GL_QUADS);
        glVertex2d(0.0, 0.0);
        glVertex2d(0.0, 1.0);
        glVertex2d(1.0, 1.0);
        glVertex2d(1.0, 0.0);
        glEnd();
        glDisable(GL_BLEND);

        // Access linedata and linergb as raw pointers matching C layout
        let linedata_ptr = (*fig).linedata.as_ptr() as *const f32;
        let linergb_ptr = (*fig).linergb.as_ptr() as *const f32;

        // determine range along (x,y)
        for axis in 0..2_i32 {
            // init ranges, set flag
            range[axis as usize][0] = (*fig).range[axis as usize][0];
            range[axis as usize][1] = (*fig).range[axis as usize][1];
            let mut flg: i32;
            if (*fig).range[axis as usize][0] < (*fig).range[axis as usize][1] {
                flg = 1;
            } else {
                flg = 0;
            }

            // determine range: scan line data, find min and max
            if flg == 0 || (*fig).flg_extend != 0 {
                for n in 0..mjMAXLINE {
                    for i in 0..(*fig).linepnt[n as usize] {
                        // get data: C linedata[n][2*i+axis] = offset n*2002 + 2*i + axis
                        let ldata = *linedata_ptr.add((n as usize) * 2002 + (2 * i + axis) as usize);

                        // skip if not finite
                        if !ldata.is_finite() {
                            continue;
                        }

                        // copy first finite data point
                        if flg == 0 {
                            range[axis as usize][0] = ldata;
                            range[axis as usize][1] = ldata;
                            flg = 1;
                        }
                        // otherwise regular update
                        else {
                            // update minimum
                            if range[axis as usize][0] > ldata {
                                range[axis as usize][0] = ldata;
                            }
                            // update maximum
                            if range[axis as usize][1] < ldata {
                                range[axis as usize][1] = ldata;
                            }
                        }
                    }
                }
            }

            // make sure range is not too small
            if range[axis as usize][1] - range[axis as usize][0] < minrange {
                let needed: f32 = minrange - (range[axis as usize][1] - range[axis as usize][0]);
                range[axis as usize][0] -= 0.5f32 * needed;
                range[axis as usize][1] += 0.5f32 * needed;
            }

            // special handling for Y-axis
            if axis == 1 {
                // get current range
                let y_min: f32 = range[1][0];
                let y_max: f32 = range[1][1];
                let diff: f32 = y_max - y_min;

                // snap to nearest enclosing decimal bounds
                if diff > 0.0 {
                    let power: f32 = powf(10.0, floorf(log10f(diff)));
                    range[1][0] = floorf(y_min / power) * power;
                    range[1][1] = ceilf(y_max / power) * power;
                }

                // make range symmetric
                if (*fig).flg_symmetric != 0 {
                    let abs0 = fabsf(range[1][0]);
                    let abs1 = fabsf(range[1][1]);
                    let y_sym: f32 = if abs0 > abs1 { abs0 } else { abs1 };
                    range[1][0] = -y_sym;
                    range[1][1] = y_sym;
                }
            }
        }

        // save ranges
        (*fig).xaxisdata[0] = range[0][0];
        (*fig).xaxisdata[1] = range[0][1];
        (*fig).yaxisdata[0] = range[1][0];
        (*fig).yaxisdata[1] = range[1][1];

        // set projection in pixels
        glMatrixMode(GL_PROJECTION);
        glLoadIdentity();
        if viewport.width <= 1 || viewport.height <= 1 {
            return;
        }
        glOrtho(0.0, (viewport.width - 1) as f64, 0.0, (viewport.height - 1) as f64, -1.0, 1.0);

        // draw title, save vertical size
        let mut sy_title: i32 = 0;
        if (*fig).title[0] != 0 {
            // find address of selected subplot title
            let mut subadr: i32 = 0;
            let mut subcnt: i32 = 0;
            while subcnt < (*fig).subplot && subadr < 1000 && (*fig).title[subadr as usize] != 0 {
                // skip non-space
                while subadr < 1000 && (*fig).title[subadr as usize] != 0 && (*fig).title[subadr as usize] != b' ' as i8 {
                    subadr += 1;
                }

                // skip space, count
                let mut cntspace: i32 = 0;
                while subadr < 1000 && (*fig).title[subadr as usize] == b' ' as i8 {
                    subadr += 1;
                    cntspace += 1;
                }

                // count subplot if 2+ spaces
                if cntspace > 1 {
                    subcnt += 1;
                }
            }

            // find length of selected subplot title (skip non-space or single space)
            let mut sublen: i32 = 0;
            while (subadr + sublen < 1000 &&
                   (*fig).title[(subadr + sublen) as usize] != 0 &&
                   (*fig).title[(subadr + sublen) as usize] != b' ' as i8) ||
                  (subadr + sublen + 1 < 1000 && (*fig).title[(subadr + sublen + 1) as usize] != 0 &&
                   (*fig).title[(subadr + sublen + 1) as usize] != b' ' as i8) {
                sublen += 1;
            }

            // proceed if non-empty
            if sublen != 0 {
                // save vertical size
                sy_title = (*con).charHeight;

                // get title size
                let mut sx_title: i32 = 0;
                for i in subadr..(subadr + sublen) {
                    sx_title += (*con).charWidth[(*fig).title[i as usize] as u8 as usize];
                }

                // compute left edge of title
                let left: i32 = if 0 > viewport.width / 2 - sx_title / 2 { 0 } else { viewport.width / 2 - sx_title / 2 };

                // compute left edge and address of extended title
                let mut left1: i32 = left;
                let mut subadr1: i32 = subadr;
                while subadr1 > 0 && left1 >= (*con).charWidth[(*fig).title[(subadr1 - 1) as usize] as u8 as usize] {
                    left1 -= (*con).charWidth[(*fig).title[(subadr1 - 1) as usize] as u8 as usize];
                    subadr1 -= 1;
                }

                // compute right edge and length of extended title
                let mut sublen1: i32 = 0;
                let mut right1: i32 = left1;
                while sublen1 + subadr1 < 1000 && (*fig).title[(subadr1 + sublen1) as usize] != 0 &&
                      right1 + (*con).charWidth[(*fig).title[(subadr1 + sublen1) as usize] as u8 as usize] < viewport.width {
                    right1 += (*con).charWidth[(*fig).title[(subadr1 + sublen1) as usize] as u8 as usize];
                    sublen1 += 1;
                }

                // extended title
                glListBase((*con).baseFontNormal);
                glColor4f(0.4 * (*fig).textrgb[0] + 0.6 * (*fig).figurergba[0],
                          0.4 * (*fig).textrgb[1] + 0.6 * (*fig).figurergba[1],
                          0.4 * (*fig).textrgb[2] + 0.6 * (*fig).figurergba[2], 1.0);
                glRasterPos3f(left1 as f32, (viewport.height - PAD - sy_title) as f32, 0.0);
                glCallLists(sublen1 as i32, GL_UNSIGNED_BYTE, (*fig).title.as_ptr().add(subadr1 as usize));

                // actual title
                glListBase((*con).baseFontNormal);
                glColor4f((*fig).textrgb[0], (*fig).textrgb[1], (*fig).textrgb[2], 1.0);
                glRasterPos3f(left as f32, (viewport.height - PAD - sy_title) as f32, 0.0);
                glCallLists(sublen as i32, GL_UNSIGNED_BYTE, (*fig).title.as_ptr().add(subadr as usize));
            }
        }

        // draw xlabel, save size
        let mut sx_label: i32 = 0;
        let mut sy_label: i32 = 0;
        if (*fig).xlabel[0] != 0 {
            // get xlabel size
            sx_label = textwidth(con, (*fig).xlabel.as_ptr());
            sy_label = (*con).charHeight;

            // render xlabel
            mjr_text_actual(mjFONT_NORMAL, (*fig).xlabel.as_ptr(), con,
                           (if 0 > viewport.width / 2 - sx_label / 2 { 0 } else { viewport.width / 2 - sx_label / 2 }) as f32,
                           PAD as f32, 0.0,
                           (*fig).textrgb[0], (*fig).textrgb[1], (*fig).textrgb[2]);
        }

        // reduce viewport to account for title and xlabel vertical size
        if sy_title != 0 || sy_label != 0 {
            // reduce viewport
            viewport.bottom += if sy_label != 0 { sy_label + PAD / 2 } else { 0 };
            viewport.height -= (if sy_label != 0 { sy_label + PAD / 2 } else { 0 }) + (if sy_title != 0 { sy_title + PAD } else { 0 });
            if viewport.width <= 0 || viewport.height <= 0 {
                return;
            }
            glViewport(viewport.left, viewport.bottom, viewport.width, viewport.height);

            // set matrix projection in pixels
            glMatrixMode(GL_PROJECTION);
            glLoadIdentity();
            glOrtho(0.0, (viewport.width - 1) as f64, 0.0, (viewport.height - 1) as f64, -1.0, 1.0);
        }

        // draw optional tick labels, reduce viewport
        if (*fig).gridsize[0] > 1 && (*fig).gridsize[1] > 1 {
            let mut txt: [i8; STRING_BUFSIZE] = [0; STRING_BUFSIZE];

            // determine xtick height
            let mut xtick_height: i32 = 0;
            if (*fig).flg_ticklabel[0] != 0 {
                xtick_height = (*con).charHeight + PAD;
            }

            // determine ytick width
            let mut ytick_width: i32 = 0;
            if (*fig).flg_ticklabel[1] != 0 {
                // start with minwidth
                if (*fig).minwidth[0] != 0 {
                    ytick_width = textwidth(con, (*fig).minwidth.as_ptr());
                }

                // limit to 20 ticks
                let n: i32 = if 20 < (*fig).gridsize[1] { 20 } else { (*fig).gridsize[1] };

                // generate all strings, find max width
                for i in 0..n {
                    let txt_sz: i32 = STRING_BUFSIZE as i32;
                    maketext((*fig).yformat.as_ptr(), txt.as_mut_ptr(),
                             range[1][0] + (range[1][1] - range[1][0]) * i as f32 / (n - 1) as f32,
                             txt_sz);
                    let w = textwidth(con, txt.as_ptr());
                    ytick_width = if ytick_width > w { ytick_width } else { w };
                }

                ytick_width += PAD;
            }

            // draw xtick
            if (*fig).flg_ticklabel[0] != 0 {
                // limit to 20 ticks
                let n: i32 = if 20 < (*fig).gridsize[0] { 20 } else { (*fig).gridsize[0] };

                // process ticks
                for i in 0..n {
                    // make text, get width
                    let txt_sz: i32 = STRING_BUFSIZE as i32;
                    maketext((*fig).xformat.as_ptr(), txt.as_mut_ptr(),
                             range[0][0] + (range[0][1] - range[0][0]) * i as f32 / (n - 1) as f32,
                             txt_sz);
                    let w: i32 = textwidth(con, txt.as_ptr());

                    // draw
                    mjr_text_actual(mjFONT_NORMAL, txt.as_ptr(), con,
                                   (PAD + ytick_width +
                                    (if i == 0 { 0 } else if i == n - 1 { -w } else { -w / 2 }) +
                                    ((viewport.width - 2 * PAD - ytick_width) as f32 * i as f32 / (n - 1) as f32) as i32) as f32,
                                   PAD as f32, 0.0,
                                   (*fig).textrgb[0], (*fig).textrgb[0], (*fig).textrgb[0]);
                }
            }

            // draw ytick
            if (*fig).flg_ticklabel[1] != 0 {
                // limit to 20 ticks
                let n: i32 = if 20 < (*fig).gridsize[1] { 20 } else { (*fig).gridsize[1] };

                // process ticks
                for i in 0..n {
                    // make text, get width
                    let txt_sz: i32 = STRING_BUFSIZE as i32;
                    maketext((*fig).yformat.as_ptr(), txt.as_mut_ptr(),
                             range[1][0] + (range[1][1] - range[1][0]) * i as f32 / (n - 1) as f32,
                             txt_sz);
                    let w: i32 = textwidth(con, txt.as_ptr());

                    // draw
                    mjr_text_actual(mjFONT_NORMAL, txt.as_ptr(), con,
                                   (ytick_width - w) as f32,
                                   (PAD + xtick_height - (*con).charHeight / 2 +
                                    ((viewport.height - 2 * PAD - xtick_height) as f32 * i as f32 / (n - 1) as f32) as i32) as f32,
                                   0.0,
                                   (*fig).textrgb[0], (*fig).textrgb[0], (*fig).textrgb[0]);
                }
            }

            // adjust viewport size
            viewport.left += ytick_width;
            viewport.width -= ytick_width;
            viewport.bottom += xtick_height;
            viewport.height -= xtick_height;
        }

        // set plot viewport
        viewport.left += PAD;
        viewport.width -= 2 * PAD;
        viewport.bottom += PAD;
        viewport.height -= 2 * PAD;
        if viewport.width <= 0 || viewport.height <= 0 {
            return;
        }
        glViewport(viewport.left, viewport.bottom, viewport.width, viewport.height);

        // save range
        (*fig).xaxispixel[0] = viewport.left;
        (*fig).xaxispixel[1] = viewport.left + viewport.width - 1;
        (*fig).yaxispixel[0] = viewport.bottom;
        (*fig).yaxispixel[1] = viewport.bottom + viewport.height - 1;

        // set matrix projection to (0,1) + offset
        glMatrixMode(GL_PROJECTION);
        glLoadIdentity();
        glOrtho(-(offset as f64), 1.0 + offset as f64, -(offset as f64), 1.0 + offset as f64, -1.0, 1.0);

        // draw pane background
        glEnable(GL_BLEND);
        glColor4fv((*fig).panergba.as_ptr());
        glBegin(GL_QUADS);
        glVertex2d(0.0, 0.0);
        glVertex2d(0.0, 1.0);
        glVertex2d(1.0, 1.0);
        glVertex2d(1.0, 0.0);
        glEnd();
        glDisable(GL_BLEND);

        // prepare to render lines
        glEnableClientState(GL_VERTEX_ARRAY);

        // draw grid
        if (*fig).gridsize[0] > 1 && (*fig).gridsize[1] > 1 {
            // common GL state
            glVertexPointer(2, GL_FLOAT, 0, griddata.as_ptr());
            glColor3fv((*fig).gridrgb.as_ptr());
            glLineWidth((*fig).gridwidth);

            // prepare vertical lines
            let n: i32 = if 20 < (*fig).gridsize[0] { 20 } else { (*fig).gridsize[0] };
            for i in 0..n {
                griddata[(4 * i) as usize] = i as f32 / (n - 1) as f32;
                griddata[(4 * i + 1) as usize] = 0.0;
                griddata[(4 * i + 2) as usize] = griddata[(4 * i) as usize];
                griddata[(4 * i + 3) as usize] = 1.0;
            }

            // draw vertical lines
            glDrawArrays(GL_LINES, 0, 2 * n);

            // prepare horizontal lines
            let n: i32 = if 20 < (*fig).gridsize[1] { 20 } else { (*fig).gridsize[1] };
            for i in 0..n {
                griddata[(4 * i) as usize] = 0.0;
                griddata[(4 * i + 1) as usize] = i as f32 / (n - 1) as f32;
                griddata[(4 * i + 2) as usize] = 1.0;
                griddata[(4 * i + 3) as usize] = griddata[(4 * i + 1) as usize];
            }

            // draw horizontal lines
            glDrawArrays(GL_LINES, 0, 2 * n);
        }

        // set matrix projection to range, with small padding
        let difx: f32 = range[0][1] - range[0][0];
        let dify: f32 = range[1][1] - range[1][0];
        glMatrixMode(GL_PROJECTION);
        glLoadIdentity();
        glOrtho((range[0][0] - difx * offset) as f64, (range[0][1] + difx * offset) as f64,
                (range[1][0] - dify * offset) as f64, (range[1][1] + dify * offset) as f64, -1.0, 1.0);

        // draw lines: back to front
        for n in (0..mjMAXLINE).rev() {
            if (*fig).linepnt[n as usize] != 0 {
                // GL state: linedata[n] in C = offset n*2002 from start
                glVertexPointer(2, GL_FLOAT, 0, linedata_ptr.add((n as usize) * 2002));
                // linergb[n] in C = offset n*3 from start
                glColor3fv(linergb_ptr.add((n as usize) * 3));
                glLineWidth((*fig).linewidth);

                // draw line
                glDrawArrays(
                    if (*fig).flg_barplot != 0 { GL_LINES } else { GL_LINE_STRIP },
                    0,
                    (*fig).linepnt[n as usize]);
            }
        }

        // selection
        if (*fig).flg_selection != 0 {
            glColor3f(1.0, 1.0, 1.0);
            glLineWidth(3.0);
            glBegin(GL_LINES);
            glVertex2f((*fig).selection, range[1][0]);
            glVertex2f((*fig).selection, range[1][1]);
            glEnd();
        }

        // draw legend, find line to highlight
        let mut hlight: i32 = (*fig).highlightid;
        if (*fig).flg_legend != 0 {
            // set matrix projection in pixels
            glMatrixMode(GL_PROJECTION);
            glLoadIdentity();
            if viewport.width <= 1 || viewport.height <= 1 {
                return;
            }
            glOrtho(0.0, (viewport.width - 1) as f64, 0.0, (viewport.height - 1) as f64, -1.0, 1.0);

            // find legend size
            let mut lw: i32 = 0;
            let mut lh: i32 = 0;
            let mut cnt: i32 = 0;
            for n in (*fig).legendoffset..mjMAXLINE {
                if (*fig).linename[n as usize][0] != 0 {
                    // max width, accumulate height
                    let w = textwidth(con, (*fig).linename[n as usize].as_ptr());
                    lw = if lw > w { lw } else { w };
                    lh += (*con).charHeight;

                    // count
                    cnt += 1;

                    // too big: correct and break
                    if lh + 2 * PAD >= viewport.height {
                        cnt -= 1;
                        lh -= (*con).charHeight;
                        break;
                    }
                }
            }

            // detect highlighted line
            let mut hcnt: i32 = -1;
            let hx: i32 = (*fig).highlight[0] - viewport.left;
            let hy: i32 = (*fig).highlight[1] - viewport.bottom;
            if hx >= viewport.width - PAD - lw &&
               hx <= viewport.width - PAD &&
               hy >= viewport.height - PAD - lh &&
               hy <= viewport.height - PAD {
                hcnt = (viewport.height - PAD - hy) / (*con).charHeight;
            }

            // draw background
            if cnt != 0 {
                glEnable(GL_BLEND);
                glColor4fv((*fig).legendrgba.as_ptr());
                glBegin(GL_QUADS);
                glVertex2d((viewport.width - 2 * PAD - lw) as f64, (viewport.height - 2 * PAD - lh) as f64);
                glVertex2d((viewport.width - 2 * PAD - lw) as f64, viewport.height as f64);
                glVertex2d(viewport.width as f64, viewport.height as f64);
                glVertex2d(viewport.width as f64, (viewport.height - 2 * PAD - lh) as f64);
                glEnd();
                glDisable(GL_BLEND);
            }

            // find named lines and draw text
            cnt = 0;
            for n in (*fig).legendoffset..mjMAXLINE {
                if (*fig).linename[n as usize][0] != 0 {
                    // get width
                    let width: i32 = textwidth(con, (*fig).linename[n as usize].as_ptr());

                    // render right-aligned
                    // linergb[n] in C = offset n*3 from start
                    let lr = *linergb_ptr.add((n as usize) * 3 + 0);
                    let lg = *linergb_ptr.add((n as usize) * 3 + 1);
                    let lb = *linergb_ptr.add((n as usize) * 3 + 2);
                    mjr_text_actual(mjFONT_SHADOW, (*fig).linename[n as usize].as_ptr(), con,
                                   (viewport.width - PAD - width) as f32,
                                   (viewport.height - PAD - (cnt + 1) * (*con).charHeight) as f32,
                                   0.0, lr, lg, lb);

                    // save hlight
                    if hcnt == cnt {
                        hlight = n;
                    }

                    // count
                    cnt += 1;

                    // break if viewport is full
                    if (cnt + 1) * (*con).charHeight + 2 * PAD >= viewport.height {
                        break;
                    }
                }
            }
        }

        // draw highlight
        if hlight >= 0 && hlight < mjMAXLINE {
            // line-rendering projection
            glMatrixMode(GL_PROJECTION);
            glLoadIdentity();
            glOrtho((range[0][0] - difx * offset) as f64, (range[0][1] + difx * offset) as f64,
                    (range[1][0] - dify * offset) as f64, (range[1][1] + dify * offset) as f64, -1.0, 1.0);

            // render selected line with extra width
            glVertexPointer(2, GL_FLOAT, 0, linedata_ptr.add((hlight as usize) * 2002));
            // linergb[hlight] in C = offset hlight*3 from start
            glColor3fv(linergb_ptr.add((hlight as usize) * 3));
            glLineWidth(5.0 * (*fig).linewidth);
            glDrawArrays(
                if (*fig).flg_barplot != 0 { GL_LINES } else { GL_LINE_STRIP },
                0,
                (*fig).linepnt[hlight as usize]);

            // print data coordinates
            if (*fig).flg_selection != 0 {
                // find nearest x-value
                let mut ibest: i32 = -1;
                let mut best: f32 = 0.0;
                for i in 0..(*fig).linepnt[hlight as usize] {
                    // linedata[hlight][2*i] = offset hlight*2002 + 2*i
                    let xval = *linedata_ptr.add((hlight as usize) * 2002 + (2 * i) as usize);
                    let diff_val = fabsf((*fig).selection - xval);
                    if ibest < 0 || best > diff_val {
                        ibest = i;
                        best = diff_val;
                    }
                }

                // show text
                let xdata = *linedata_ptr.add((hlight as usize) * 2002 + (2 * ibest) as usize);
                let ydata = *linedata_ptr.add((hlight as usize) * 2002 + (2 * ibest + 1) as usize);
                snprintf(datatxt.as_mut_ptr(), STRING_BUFSIZE,
                         b"( %.4g : %.4g )\0".as_ptr() as *const i8,
                         xdata as f64, ydata as f64);
                mjr_text_actual(mjFONT_SHADOW, datatxt.as_ptr(), con,
                               0.9 * range[0][0] + 0.1 * range[0][1],
                               0.9 * range[1][0] + 0.1 * range[1][1],
                               0.0,
                               (*fig).textrgb[0], (*fig).textrgb[1], (*fig).textrgb[2]);
            }
        }

        // stop rendering lines
        glDisableClientState(GL_VERTEX_ARRAY);
    }
}

