//! Port of: ui/ui_main.c
//! IR hash: 9614bd3d92e7766b
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: SCL (ui/ui_main.c:200)
/// Calls: mju_round
#[allow(unused_variables, non_snake_case)]
pub fn scl(sz: i32, con: *const mjrContext) -> i32 {
    // SAFETY: caller guarantees con is a valid pointer to mjrContext
    unsafe {
        let val = crate::engine::engine_util_misc::mju_round(sz as f64 * 0.01 * (*con).fontScale as f64);
        if 0 > val { 0 } else { val }
    }
}

/// C: initOpenGL (ui/ui_main.c:207)
#[allow(unused_variables, non_snake_case)]
pub fn init_open_gl(r: *const mjrRect, con: *const mjrContext) {
    const GL_NORMALIZE: u32 = 0x0BA1;
    const GL_DEPTH_TEST: u32 = 0x0B71;
    const GL_CULL_FACE: u32 = 0x0B44;
    const GL_LIGHTING: u32 = 0x0B50;
    const GL_COLOR_MATERIAL: u32 = 0x0B57;
    const GL_BLEND: u32 = 0x0BE2;
    const GL_SMOOTH: u32 = 0x1D01;
    const GL_FRONT_AND_BACK: u32 = 0x0408;
    const GL_FILL: u32 = 0x1B02;
    const GL_PROJECTION: u32 = 0x1701;
    const GL_MODELVIEW: u32 = 0x1700;

    extern "C" {
        fn glDisable(cap: u32);
        fn glShadeModel(mode: u32);
        fn glPolygonMode(face: u32, mode: u32);
        fn glMatrixMode(mode: u32);
        fn glLoadIdentity();
        fn glOrtho(left: f64, right: f64, bottom: f64, top: f64, near: f64, far: f64);
        fn glViewport(x: i32, y: i32, width: i32, height: i32);
    }

    // SAFETY: r is a valid mjrRect pointer; GL functions are linked from mujoco rendering context
    unsafe {
        glDisable(GL_NORMALIZE);
        glDisable(GL_DEPTH_TEST);
        glDisable(GL_CULL_FACE);
        glDisable(GL_LIGHTING);
        glDisable(GL_COLOR_MATERIAL);
        glDisable(GL_BLEND);
        glShadeModel(GL_SMOOTH);
        glPolygonMode(GL_FRONT_AND_BACK, GL_FILL);

        // standard 2D projection, in framebuffer units
        glMatrixMode(GL_PROJECTION);
        glLoadIdentity();
        glOrtho(0.0, (*r).width as f64, 0.0, (*r).height as f64, -1.0, 1.0);
        glMatrixMode(GL_MODELVIEW);
        glLoadIdentity();

        // set viewport
        glViewport((*r).left, (*r).bottom, (*r).width, (*r).height);
    }
}

/// C: drawtext (ui/ui_main.c:251)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn drawtext(txt: *const i8, x: i32, y: i32, maxwidth: i32, rgb: *const f32, con: *const mjrContext) {
    extern "C" {
        fn glListBase(base: u32);
        fn glColor3fv(v: *const f32);
        fn glRasterPos3i(x: i32, y: i32, z: i32);
        fn glCallLists(n: i32, r#type: u32, lists: *const i8);
    }
    const GL_UNSIGNED_BYTE: u32 = 0x1401;

    // SAFETY: txt is a valid C string; con is a valid mjrContext pointer; rgb is valid f32[3]
    unsafe {
        // determine string length that fits in maxwidth
        let mut len: i32 = 0;
        let mut width: i32 = 0;
        while *txt.add(len as usize) != 0 {
            width += (*con).charWidth[*txt.add(len as usize) as u8 as usize];
            if width >= maxwidth {
                break;
            } else {
                len += 1;
            }
        }

        // draw normal text
        glListBase((*con).baseFontNormal);
        glColor3fv(rgb);
        glRasterPos3i(x, y, 0);
        glCallLists(len, GL_UNSIGNED_BYTE, txt);
    }
}

/// C: drawtextrect (ui/ui_main.c:274)
/// Calls: drawtext, textwidth
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn drawtextrect(rect: mjrRect, txt: *const i8, rgb: *const f32, con: *const mjrContext) {
    // SAFETY: txt is a valid C string; con is valid mjrContext pointer; rgb is valid f32[3] (caller contract)
    unsafe {
        // inline textwidth(txt, con, -1): sum all char widths
        let mut tw: i32 = 0;
        let mut i: usize = 0;
        while *txt.add(i) != 0 {
            tw += (*con).charWidth[*txt.add(i) as u8 as usize];
            i += 1;
        }

        let dy = (rect.height - (*con).charHeight) / 2;
        let mut dx = (rect.width - tw) / 2;
        if dx < 0 {
            dx = 0;
        }

        drawtext(txt, rect.left + dx, rect.bottom + dy, rect.width - dx, rgb, con);
    }
}

/// C: drawrectangle (ui/ui_main.c:286)
/// Calls: SCL
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn drawrectangle(rect: mjrRect, rgb: *const f32, rgbback: *const f32, con: *const mjrContext) {
    const GL_QUADS: u32 = 0x0007;

    extern "C" {
        fn glColor3fv(v: *const f32);
        fn glBegin(mode: u32);
        fn glEnd();
        fn glVertex2i(x: i32, y: i32);
    }

    // SAFETY: GL functions linked from mujoco; rgb/rgbback are valid f32[3] arrays (caller contract)
    unsafe {
        // outside
        glColor3fv(rgb);
        glBegin(GL_QUADS);
        glVertex2i(rect.left, rect.bottom);
        glVertex2i(rect.left + rect.width, rect.bottom);
        glVertex2i(rect.left + rect.width, rect.bottom + rect.height);
        glVertex2i(rect.left, rect.bottom + rect.height);
        glEnd();

        // inside
        if !rgbback.is_null() {
            let margin: i32 = scl(2, con);
            glColor3fv(rgbback);
            glBegin(GL_QUADS);
            glVertex2i(rect.left + margin, rect.bottom + margin);
            glVertex2i(rect.left + rect.width - margin, rect.bottom + margin);
            glVertex2i(rect.left + rect.width - margin, rect.bottom + rect.height - margin);
            glVertex2i(rect.left + margin, rect.bottom + rect.height - margin);
            glEnd();
        }
    }
}

/// C: roundcorner (ui/ui_main.c:313)
#[allow(unused_variables, non_snake_case)]
pub fn roundcorner(rect: mjrRect, flg_skipbottom: i32, flg_separator: i32, ui: *const mjUI, con: *const mjrContext) {
    todo!() // roundcorner
}

/// C: drawoval (ui/ui_main.c:375)
/// Calls: SCL
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn drawoval(rect: mjrRect, rgb: *const f32, rgbback: *const f32, con: *const mjrContext) {
    const NDIVIDE: i32 = 15;
    const GL_POLYGON: u32 = 0x0009;

    extern "C" {
        fn glColor3fv(v: *const f32);
        fn glBegin(mode: u32);
        fn glEnd();
        fn glVertex2d(x: f64, y: f64);
    }

    // require horizontal
    if rect.height > rect.width {
        return;
    }

    // circle info
    let radius: f64 = 0.5 * rect.height as f64;
    let lcenter: [f64; 2] = [rect.left as f64 + radius, rect.bottom as f64 + radius];
    let rcenter: [f64; 2] = [rect.left as f64 + rect.width as f64 - radius, rect.bottom as f64 + radius];

    // SAFETY: rgb/rgbback are valid f32[3] pointers (caller contract); GL functions linked from mujoco
    unsafe {
        // filled
        glColor3fv(rgb);
        glBegin(GL_POLYGON);

        // draw left half-circle
        for i in 0..=NDIVIDE {
            let angle: f64 = std::f64::consts::PI * (0.5 + i as f64 / NDIVIDE as f64);
            glVertex2d(lcenter[0] + radius * f64::cos(angle), lcenter[1] + radius * f64::sin(angle));
        }

        // draw right half-circle
        for i in 0..=NDIVIDE {
            let angle: f64 = std::f64::consts::PI * (1.5 + i as f64 / NDIVIDE as f64);
            glVertex2d(rcenter[0] + radius * f64::cos(angle), rcenter[1] + radius * f64::sin(angle));
        }
        glEnd();

        // inside
        if !rgbback.is_null() {
            let margin: i32 = scl(2, con);
            let radius_inner: f64 = radius - margin as f64;

            glColor3fv(rgbback);
            glBegin(GL_POLYGON);

            // draw left half-circle
            for i in 0..=NDIVIDE {
                let angle: f64 = std::f64::consts::PI * (0.5 + i as f64 / NDIVIDE as f64);
                glVertex2d(lcenter[0] + radius_inner * f64::cos(angle), lcenter[1] + radius_inner * f64::sin(angle));
            }

            // draw right half-circle
            for i in 0..=NDIVIDE {
                let angle: f64 = std::f64::consts::PI * (1.5 + i as f64 / NDIVIDE as f64);
                glVertex2d(rcenter[0] + radius_inner * f64::cos(angle), rcenter[1] + radius_inner * f64::sin(angle));
            }
            glEnd();
        }
    }
}

/// C: drawsymbol (ui/ui_main.c:434)
/// Calls: SCL, mju_round
#[allow(unused_variables, non_snake_case)]
pub fn drawsymbol(rect: mjrRect, flg_open: i32, r#type: i32, ui: *const mjUI, con: *const mjrContext) {
    const GL_TRIANGLES: u32 = 0x0004;

    extern "C" {
        fn glColor3fv(v: *const f32);
        fn glColor3f(r: f32, g: f32, b: f32);
        fn glBegin(mode: u32);
        fn glEnd();
        fn glVertex2i(x: i32, y: i32);
        fn glVertex2d(x: f64, y: f64);
    }

    // SAFETY: ui and con are valid pointers (caller contract); GL functions linked from mujoco
    unsafe {
        // access spacing.texthor (offset 36 in spacing byte array)
        let spacing_ptr = (*ui).spacing.as_ptr() as *const i32;
        let texthor: i32 = scl(*spacing_ptr.add(9), con);

        // access color.sectsymbol (offset 108 in color byte array)
        let color_ptr = (*ui).color.as_ptr();
        let sectsymbol = color_ptr.add(108) as *const f32;

        let cx = rect.left + rect.width - texthor;
        let cy = rect.bottom + rect.height / 2;
        let mut d = crate::engine::engine_util_misc::mju_round((*con).charHeight as f64 * 0.33);

        // separator size
        if r#type == 3 {
            d = crate::engine::engine_util_misc::mju_round((*con).charHeight as f64 * 0.28);
        }

        // open
        if flg_open != 0 {
            glColor3fv(sectsymbol);
            glBegin(GL_TRIANGLES);
            glVertex2i(cx, cy + d);
            glVertex2i(cx - 2 * d, cy + d);
            glVertex2i(cx - d, cy - d);
            glEnd();
        } else {
            // solid outside
            glColor3fv(sectsymbol);
            glBegin(GL_TRIANGLES);
            glVertex2i(cx, cy - d);
            glVertex2i(cx, cy + d);
            glVertex2i(cx - 2 * d, cy);
            glEnd();

            // set color for inside
            match r#type {
                0 => {
                    // section: secttitle at offset 24, secttitle2 at offset 36
                    let secttitle = color_ptr.add(24) as *const f32;
                    let secttitle2 = color_ptr.add(36) as *const f32;
                    glColor3f(
                        (*secttitle.add(0) + *secttitle2.add(0)) * 0.5,
                        (*secttitle.add(1) + *secttitle2.add(1)) * 0.5,
                        (*secttitle.add(2) + *secttitle2.add(2)) * 0.5,
                    );
                }
                1 => {
                    // section with unchecked box: secttitleuncheck at offset 48, secttitleuncheck2 at offset 60
                    let a = color_ptr.add(48) as *const f32;
                    let b = color_ptr.add(60) as *const f32;
                    glColor3f(
                        (*a.add(0) + *b.add(0)) * 0.5,
                        (*a.add(1) + *b.add(1)) * 0.5,
                        (*a.add(2) + *b.add(2)) * 0.5,
                    );
                }
                2 => {
                    // section with checked box: secttitlecheck at offset 72, secttitlecheck2 at offset 84
                    let a = color_ptr.add(72) as *const f32;
                    let b = color_ptr.add(84) as *const f32;
                    glColor3f(
                        (*a.add(0) + *b.add(0)) * 0.5,
                        (*a.add(1) + *b.add(1)) * 0.5,
                        (*a.add(2) + *b.add(2)) * 0.5,
                    );
                }
                3 => {
                    // separator: separator at offset 132, separator2 at offset 144
                    let a = color_ptr.add(132) as *const f32;
                    let b = color_ptr.add(144) as *const f32;
                    glColor3f(
                        (*a.add(0) + *b.add(0)) * 0.5,
                        (*a.add(1) + *b.add(1)) * 0.5,
                        (*a.add(2) + *b.add(2)) * 0.5,
                    );
                }
                _ => {}
            }

            // draw inside
            let margin: f64 = (*con).fontScale as f64 * 0.015;
            let u: f64 = 0.5 * f64::sqrt(5.0) * margin;
            let y: f64 = d as f64 - u - 0.5 * margin;
            glBegin(GL_TRIANGLES);
            glVertex2d(cx as f64 - margin, cy as f64 - y);
            glVertex2d(cx as f64 - margin, cy as f64 + y);
            glVertex2d(cx as f64 - 2.0 * d as f64 + 2.0 * u, cy as f64);
            glEnd();
        }
    }
}

/// C: radioelement (ui/ui_main.c:516)
/// Calls: SCL
#[allow(unused_variables, non_snake_case)]
pub fn radioelement(it: *const mjuiItem, n: i32, ui: *const mjUI, con: *const mjrContext) -> mjrRect {
    todo!() // radioelement
}

/// C: mouseinui (ui/ui_main.c:549)
/// Calls: mjCActuator::act
#[allow(unused_variables, non_snake_case)]
pub fn mouseinui(ui: *const mjUI, ins: *const mjuiState, x: *mut i32, y: *mut i32) {
    todo!() // mouseinui
}

/// C: mouseinrect (ui/ui_main.c:568)
/// Calls: mouseinui
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mouseinrect(rect: mjrRect, ui: *const mjUI, ins: *const mjuiState, rx: *mut f64, ry: *mut f64) {
    todo!() // mouseinrect
}

/// C: findradio (ui/ui_main.c:582)
/// Calls: mouseinrect
#[allow(unused_variables, non_snake_case)]
pub fn findradio(it: *const mjuiItem, ui: *const mjUI, ins: *const mjuiState, con: *const mjrContext) -> i32 {
    todo!() // findradio
}

/// C: makeradioline (ui/ui_main.c:613)
/// Calls: mju_round, textwidth
#[allow(unused_variables, non_snake_case)]
pub fn makeradioline(it: *const mjuiItem, con: *const mjrContext, sep: *mut i32) {
    todo!() // makeradioline
}

/// C: findradioline (ui/ui_main.c:642)
/// Calls: makeradioline, mju_round, mouseinrect
#[allow(unused_variables, non_snake_case)]
pub fn findradioline(it: *const mjuiItem, ui: *const mjUI, ins: *const mjuiState, con: *const mjrContext) -> i32 {
    todo!() // findradioline
}

/// C: findselect (ui/ui_main.c:667)
/// Calls: SCL, mouseinrect
#[allow(unused_variables, non_snake_case)]
pub fn findselect(it: *const mjuiItem, ui: *const mjUI, ins: *const mjuiState, con: *const mjrContext) -> i32 {
    todo!() // findselect
}

/// C: scrollrect (ui/ui_main.c:696)
/// Calls: SCL, mju_round
#[allow(unused_variables, non_snake_case)]
pub fn scrollrect(rect: mjrRect, ui: *const mjUI, con: *const mjrContext, bar: *mut mjrRect, thumb: *mut mjrRect) {
    todo!() // scrollrect
}

/// C: inside (ui/ui_main.c:716)
#[allow(unused_variables, non_snake_case)]
pub fn inside(x: i32, y: i32, r: mjrRect) -> i32 {
    (x >= r.left && x <= r.left + r.width && y >= r.bottom && y <= r.bottom + r.height) as i32
}

/// C: insideoval (ui/ui_main.c:723)
/// Calls: inside
#[allow(unused_variables, non_snake_case)]
pub fn insideoval(x: i32, y: i32, r: mjrRect) -> i32 {
    todo!() // insideoval
}

/// C: findmouse (ui/ui_main.c:757)
/// Calls: inside, insideoval, mouseinui, scrollrect
#[allow(unused_variables, non_snake_case)]
pub fn findmouse(ui: *const mjUI, ins: *const mjuiState, con: *const mjrContext, sect: *mut i32, item: *mut i32) {
    todo!() // findmouse
}

/// C: setslider (ui/ui_main.c:841)
/// Calls: mju_clip, mju_round, mouseinrect
#[allow(unused_variables, non_snake_case)]
pub fn setslider(it: *mut mjuiItem, ui: *mut mjUI, ins: *const mjuiState, con: *const mjrContext) {
    todo!() // setslider
}

/// C: checkedit (ui/ui_main.c:868)
/// Calls: mju_error
#[allow(unused_variables, non_snake_case)]
pub fn checkedit(text: *const i8, it: *const mjuiItem) -> i32 {
    todo!() // checkedit
}

/// C: text2array (ui/ui_main.c:914)
/// Calls: mju_error
#[allow(unused_variables, non_snake_case)]
pub fn text2array(text: *const i8, it: *const mjuiItem) -> i32 {
    todo!() // text2array
}

/// C: array2text (ui/ui_main.c:982)
/// Calls: mju_error
#[allow(unused_variables, non_snake_case)]
pub fn array2text(text: *mut i8, it: *const mjuiItem) {
    todo!() // array2text
}

/// C: validkey (ui/ui_main.c:1017)
#[allow(unused_variables, non_snake_case)]
pub fn validkey(key: i32, sz: i32, r#type: i32, state: *const mjuiState) -> i32 {
    todo!() // validkey
}

/// C: revealcursor (ui/ui_main.c:1098)
/// Calls: SCL
#[allow(unused_variables, non_snake_case)]
pub fn revealcursor(r: mjrRect, ui: *mut mjUI, con: *const mjrContext) {
    todo!() // revealcursor
}

/// C: setcursor (ui/ui_main.c:1124)
/// Calls: SCL, mju_round, mouseinrect, revealcursor
#[allow(unused_variables, non_snake_case)]
pub fn setcursor(r: mjrRect, ui: *mut mjUI, ins: *const mjuiState, con: *const mjrContext) {
    todo!() // setcursor
}

/// C: parseshortcut (ui/ui_main.c:1169)
/// Calls: mju_error
#[allow(unused_variables, non_snake_case)]
pub fn parseshortcut(text: *const i8, r#mod: *mut i32, key: *mut i32) {
    todo!() // parseshortcut
}

/// C: matchshortcut (ui/ui_main.c:1222)
#[allow(unused_variables, non_snake_case)]
pub fn matchshortcut(ins: *const mjuiState, r#mod: i32, key: i32) -> i32 {
    // SAFETY: ins is a valid mjuiState pointer (caller contract)
    unsafe {
        // match key
        if key == 0 || (*ins).key != key {
            return 0;
        }

        // match modifier
        if ((*ins).control != 0) as i32 + 2 * ((*ins).shift != 0) as i32 + 4 * ((*ins).alt != 0) as i32 != r#mod {
            return 0;
        }

        1
    }
}

/// C: setitemskip (ui/ui_main.c:1500)
#[allow(unused_variables, non_snake_case)]
pub fn setitemskip(s: *mut mjuiSection, pass: i32) {
    todo!() // setitemskip
}

/// C: tryresize (ui/ui_main.c:1528)
/// Calls: SCL, setitemskip
#[allow(unused_variables, non_snake_case)]
pub fn tryresize(ui: *mut mjUI, con: *const mjrContext) {
    todo!() // tryresize
}

/// C: insertionsortgroup (ui/ui_main.c:1717)
/// Calls: mju_error
#[allow(unused_variables, non_snake_case)]
pub fn insertionsortgroup(list: *mut i32, num: i32, stride: i32) {
    todo!() // insertionsortgroup
}

/// C: evalpredicate (ui/ui_main.c:1823)
#[allow(unused_variables, non_snake_case)]
pub fn evalpredicate(state: i32, predicate: mjfItemEnable, userdata: *mut ()) -> i32 {
    todo!() // evalpredicate
}

/// C: shortcuthelp (ui/ui_main.c:1836)
/// Calls: SCL, drawrectangle, drawtext, mju_strncpy, textwidth
#[allow(unused_variables, non_snake_case)]
pub fn shortcuthelp(r: mjrRect, modifier: i32, shortcut: i32, ui: *const mjUI, con: *const mjrContext) {
    todo!() // shortcuthelp
}

/// C: mjui_themeSpacing (ui/ui_main.h:26)
#[allow(unused_variables, non_snake_case)]
pub fn mjui_theme_spacing(ind: i32) -> mjuiThemeSpacing {
    todo!() // mjui_themeSpacing
}

/// C: mjui_themeColor (ui/ui_main.h:29)
#[allow(unused_variables, non_snake_case)]
pub fn mjui_theme_color(ind: i32) -> mjuiThemeColor {
    todo!() // mjui_themeColor
}

/// C: mjui_add (ui/ui_main.h:32)
/// Calls: mju_error, mju_round, mju_strncpy, parseshortcut
#[allow(unused_variables, non_snake_case)]
pub fn mjui_add(ui: *mut mjUI, def: *const mjuiDef) {
    todo!() // mjui_add
}

/// C: mjui_addToSection (ui/ui_main.h:35)
/// Calls: mjui_add
#[allow(unused_variables, non_snake_case)]
pub fn mjui_add_to_section(ui: *mut mjUI, sect: i32, def: *const mjuiDef) {
    todo!() // mjui_addToSection
}

/// C: mjui_resize (ui/ui_main.h:38)
/// Calls: SCL, insertionsortgroup, mju_error, tryresize
#[allow(unused_variables, non_snake_case)]
pub fn mjui_resize(ui: *mut mjUI, con: *const mjrContext) {
    todo!() // mjui_resize
}

/// C: mjui_update (ui/ui_main.h:41)
/// Calls: SCL, array2text, checkedit, drawoval, drawrectangle, drawsymbol, drawtext, drawtextrect, evalpredicate, findmouse, initOpenGL, makeradioline, mjr_restoreBuffer, mjr_setAux, mju_error, mju_round, radioelement, roundcorner, shortcuthelp, textwidth
#[allow(unused_variables, non_snake_case)]
pub fn mjui_update(section: i32, item: i32, ui: *const mjUI, state: *const mjuiState, con: *const mjrContext) {
    todo!() // mjui_update
}

/// C: mjui_event (ui/ui_main.h:45)
/// Calls: SCL, array2text, evalpredicate, findmouse, findradio, findradioline, findselect, matchshortcut, mju_round, mjui_resize, mjui_update, revealcursor, setcursor, setslider, text2array, validkey
#[allow(unused_variables, non_snake_case)]
pub fn mjui_event(ui: *mut mjUI, state: *mut mjuiState, con: *const mjrContext) -> *mut mjuiItem {
    todo!() // mjui_event
}

/// C: mjui_render (ui/ui_main.h:48)
/// Calls: SCL, drawtext, findselect, initOpenGL, mjr_blitAux, mjr_rectangle, scrollrect
#[allow(unused_variables, non_snake_case)]
pub fn mjui_render(ui: *mut mjUI, state: *const mjuiState, con: *const mjrContext) {
    todo!() // mjui_render
}

