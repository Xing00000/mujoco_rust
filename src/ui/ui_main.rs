//! Port of: ui/ui_main.c
//! IR hash: 9343293228317031
//! CODEGEN: source paths, owners, and callable names are locked.

use crate::types::*;

/// C: SCL (ui/ui_main.c:200)
/// Calls: cxx:_mju_round
#[allow(unused_variables, non_snake_case)]
pub fn SCL(sz: i32, con: *const mjrContext) -> i32 {
    // SAFETY: caller guarantees con is a valid pointer to mjrContext
    unsafe {
        let val = crate::engine::engine_util_misc::mju_round(sz as f64 * 0.01 * (*con).fontScale as f64);
        if 0 > val { 0 } else { val }
    }
}

/// C: initOpenGL (ui/ui_main.c:207)
#[allow(unused_variables, non_snake_case)]
pub fn initOpenGL(r: *const mjrRect, con: *const mjrContext) {
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

/// C: drawrectangle (ui/ui_main.c:286)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/ui/ui_main.c:_SCL
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
            let margin: i32 = SCL(2, con);
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
    const GL_TRIANGLE_FAN: u32 = 0x0006;

    extern "C" {
        fn glColor3fv(v: *const f32);
        fn glBegin(mode: u32);
        fn glEnd();
        fn glVertex2d(x: f64, y: f64);
    }

    // SAFETY: ui and con are valid pointers (caller contract); GL functions linked from mujoco
    unsafe {
        // get rounding from theme, exit if disabled
        let spacing_ptr = std::ptr::addr_of!((*ui).spacing) as *const i32;
        let cornerspec: i32 = if flg_separator != 0 {
            *spacing_ptr.add(5) // spacing.cornersep
        } else {
            *spacing_ptr.add(4) // spacing.cornersect
        };
        if cornerspec == 0 {
            return;
        }

        // quarter-circle divisions and radius
        let ndivide: i32 = 10;
        let radius: f64 = cornerspec as f64 * 0.01 * (*con).fontScale as f64;

        // color pointer: sectpane or master
        let color_ptr = (*ui).color.as_ptr();
        let erase_color: *const f32 = if flg_separator != 0 {
            color_ptr.add(120) as *const f32 // color.sectpane
        } else {
            color_ptr as *const f32 // color.master
        };

        // draw fans in the four corners, optionally skip bottom corners
        let start: i32 = if flg_skipbottom != 0 { 2 } else { 0 };
        for ic in start..4 {
            // set corner
            let mut corner: [f64; 2] = [0.0; 2];
            match ic {
                0 => {
                    // bottom-left
                    corner[0] = rect.left as f64;
                    corner[1] = rect.bottom as f64;
                }
                1 => {
                    // bottom-right
                    corner[0] = (rect.left + rect.width) as f64;
                    corner[1] = rect.bottom as f64;
                }
                2 => {
                    // top-right
                    corner[0] = (rect.left + rect.width) as f64;
                    corner[1] = (rect.bottom + rect.height) as f64;
                }
                _ => {
                    // top-left
                    corner[0] = rect.left as f64;
                    corner[1] = (rect.bottom + rect.height) as f64;
                }
            }

            // orient fan to point inside
            let angle: f64 = ic as f64 * 0.5 * std::f64::consts::PI;

            // compute circle center: opposite to corner
            let mut center: [f64; 2] = [0.0; 2];
            center[0] = corner[0] + f64::sqrt(2.0) * radius * f64::cos(angle + 0.25 * std::f64::consts::PI);
            center[1] = corner[1] + f64::sqrt(2.0) * radius * f64::sin(angle + 0.25 * std::f64::consts::PI);

            // fill with erase color, start triangle_fan from corner
            glColor3fv(erase_color);
            glBegin(GL_TRIANGLE_FAN);
            glVertex2d(corner[0], corner[1]);

            // compute vertices of quarter-circle
            for i in 0..=ndivide {
                let a: f64 = angle + std::f64::consts::PI + 0.5 * std::f64::consts::PI * i as f64 / ndivide as f64;
                glVertex2d(center[0] + radius * f64::cos(a), center[1] + radius * f64::sin(a));
            }
            glEnd();
        }
    }
}

/// C: drawoval (ui/ui_main.c:375)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/ui/ui_main.c:_SCL
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
            let margin: i32 = SCL(2, con);
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
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/ui/ui_main.c:_SCL, cxx:_mju_round
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
        let spacing_ptr = std::ptr::addr_of!((*ui).spacing) as *const i32;
        let texthor: i32 = SCL(*spacing_ptr.add(9), con);

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
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/ui/ui_main.c:_SCL
#[allow(unused_variables, non_snake_case)]
pub fn radioelement(it: *const mjuiItem, n: i32, ui: *const mjUI, con: *const mjrContext) -> mjrRect {
    // SAFETY: it, ui, con are valid pointers (caller contract)
    unsafe {
        // scale sizes from theme
        let spacing_ptr = std::ptr::addr_of!((*ui).spacing) as *const i32;
        let g_itemmid: i32 = SCL(*spacing_ptr.add(7), con);
        let g_textver: i32 = SCL(*spacing_ptr.add(10), con);
        let ncol: i32 = if (*ui).radiocol != 0 { (*ui).radiocol } else { 2 };

        // access multi.nelem from the union (offset 0 in __anon_7)
        let nelem: i32 = *((*it).__anon_7._data.as_ptr() as *const i32);
        let nrow: i32 = (nelem - 1) / ncol + 1;

        // compute elements
        let cellwidth: i32 = ((*it).rect.width - (ncol - 1) * g_itemmid) / ncol;
        let cellheight: i32 = (*con).charHeight + 2 * g_textver;
        let row: i32 = n / ncol;
        let col: i32 = n % ncol;

        // construct rectangle
        let mut r = mjrRect {
            left: (*it).rect.left + col * (cellwidth + g_itemmid),
            bottom: (*it).rect.bottom + (nrow - 1 - row) * cellheight,
            width: cellwidth,
            height: cellheight,
        };

        // adjust width of last column
        if col == ncol - 1 {
            r.width = (*it).rect.width - r.left + (*it).rect.left;
        }

        r
    }
}

/// C: scrollrect (ui/ui_main.c:696)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/ui/ui_main.c:_SCL, cxx:_mju_round
#[allow(unused_variables, non_snake_case)]
pub fn scrollrect(rect: mjrRect, ui: *const mjUI, con: *const mjrContext, bar: *mut mjrRect, thumb: *mut mjrRect) {
    // SAFETY: ui, con, bar, thumb are valid pointers (caller contract)
    unsafe {
        let spacing_ptr = std::ptr::addr_of!((*ui).spacing) as *const i32;
        let w_scroll: i32 = SCL(*spacing_ptr.add(1), con); // spacing.scroll

        // bar
        *bar = rect;
        (*bar).left = rect.left + rect.width - w_scroll;
        (*bar).width = w_scroll;

        // thumb
        let tstart: f64 = (*ui).scroll as f64 / (*ui).height as f64;
        let tend: f64 = ((*ui).scroll + rect.height) as f64 / (*ui).height as f64;
        *thumb = *bar;
        (*thumb).bottom = rect.bottom + crate::engine::engine_util_misc::mju_round(
            rect.height as f64 * (1.0 - tend));
        (*thumb).height = crate::engine::engine_util_misc::mju_round(
            rect.height as f64 * (tend - tstart));
    }
}

/// C: inside (ui/ui_main.c:716)
#[allow(unused_variables, non_snake_case)]
pub fn inside(x: i32, y: i32, r: mjrRect) -> i32 {
    (x >= r.left && x <= r.left + r.width && y >= r.bottom && y <= r.bottom + r.height) as i32
}

/// C: insideoval (ui/ui_main.c:723)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/ui/ui_main.c:_inside
#[allow(unused_variables, non_snake_case)]
pub fn insideoval(x: i32, y: i32, r: mjrRect) -> i32 {
    // exclude if not in rectangle
    if inside(x, y, r) == 0 {
        return 0;
    }

    // check center
    let radius: i32 = r.height / 2;
    if x >= r.left + radius && x <= r.left + r.width - radius {
        return 1;
    }

    // left circle
    let mut dx: i32 = x - (r.left + radius);
    let dy: i32 = y - (r.bottom + radius);
    if dx < 0 && (dx * dx + dy * dy < radius * radius) {
        return 1;
    }

    // right circle
    dx = x - (r.left + r.width - radius);
    if dx > 0 && (dx * dx + dy * dy < radius * radius) {
        return 1;
    }

    0
}

/// C: validkey (ui/ui_main.c:1017)
#[allow(unused_variables, non_snake_case)]
pub fn validkey(key: i32, sz: i32, r#type: i32, state: *const mjuiState) -> i32 {
    const mjITEM_EDITTXT: i32 = 15;
    const mjITEM_EDITINT: i32 = 12;
    const mjITEM_EDITNUM: i32 = 13;
    const mjITEM_EDITFLOAT: i32 = 14;
    const mjMAXUINAME: i32 = 40;
    const mjMAXUITEXT: i32 = 300;
    const mjKEY_NUMPAD_0: i32 = 320;
    const mjKEY_NUMPAD_9: i32 = 329;

    let mut key = key;

    // text
    if r#type == mjITEM_EDITTXT {
        if sz < mjMAXUINAME - 1 && key >= 32 && key <= 127 {
            // SAFETY: state is a valid mjuiState pointer (caller contract)
            unsafe {
                // lower case if Shift (Caps Lock cannot be detected in GLFW)
                if key >= 'A' as i32 && key <= 'Z' as i32 && (*state).shift == 0 {
                    key = key + 'a' as i32 - 'A' as i32;
                }
                // Shift: remap remaining keys
                else if (*state).shift != 0 {
                    if key == '`' as i32 {
                        key = '~' as i32;
                    } else if key == '-' as i32 {
                        key = '_' as i32;
                    } else if key == '=' as i32 {
                        key = '+' as i32;
                    } else if key == '[' as i32 {
                        key = '{' as i32;
                    } else if key == ']' as i32 {
                        key = '}' as i32;
                    } else if key == '\\' as i32 {
                        key = '|' as i32;
                    } else if key == ';' as i32 {
                        key = ':' as i32;
                    } else if key == '\'' as i32 {
                        key = '"' as i32;
                    } else if key == ',' as i32 {
                        key = '<' as i32;
                    } else if key == '.' as i32 {
                        key = '>' as i32;
                    } else if key == '/' as i32 {
                        key = '?' as i32;
                    }
                }
            }
            return key;
        } else {
            return 0;
        }
    }

    // numeric
    else if r#type == mjITEM_EDITINT || r#type == mjITEM_EDITNUM || r#type == mjITEM_EDITFLOAT {
        if sz < (mjMAXUITEXT - 1)
            && (key == ' ' as i32
                || key == '+' as i32
                || key == '=' as i32
                || key == '-' as i32
                || (key >= '0' as i32 && key <= '9' as i32)
                || (key >= mjKEY_NUMPAD_0 && key <= mjKEY_NUMPAD_9)
                || ((key == 'e' as i32 || key == 'E' as i32 || key == '.' as i32)
                    && (r#type == mjITEM_EDITNUM || r#type == mjITEM_EDITFLOAT)))
        {
            // remap '=' to '+'
            if key == '=' as i32 {
                key = '+' as i32;
            }

            // remap 'E' to 'e'
            if key == 'E' as i32 {
                key = 'e' as i32;
            }

            // remap numberpad to top row
            if key >= mjKEY_NUMPAD_0 && key <= mjKEY_NUMPAD_9 {
                key = key - mjKEY_NUMPAD_0 + '0' as i32;
            }

            return key;
        } else {
            return 0;
        }
    }

    // other
    else {
        return 0;
    }
}

/// C: revealcursor (ui/ui_main.c:1098)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/ui/ui_main.c:_SCL
#[allow(unused_variables, non_snake_case)]
pub fn revealcursor(r: mjrRect, ui: *mut mjUI, con: *const mjrContext) {
    // SAFETY: ui and con are valid pointers (caller contract)
    unsafe {
        // scroll left
        if (*ui).editcursor <= (*ui).editscroll {
            (*ui).editscroll = (*ui).editcursor;
            return;
        }

        // width of available text area
        let spacing_ptr = std::ptr::addr_of!((*ui).spacing) as *const i32;
        let texthor: i32 = *spacing_ptr.add(9); // spacing.texthor
        let mut width: i32 = r.width - 2 * SCL(texthor, con);

        // scan backwards
        let mut i: i32 = (*ui).editcursor;
        while width >= 0 && i >= (*ui).editscroll && i > 0 {
            i -= 1;
            width -= (*con).charWidth[(*ui).edittext[i as usize] as usize];
        }

        // adjust scroll if out of width
        if width < 0 {
            (*ui).editscroll = i + 1;
        }
    }
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
    const mjITEM_SEPARATOR: i32 = 0;
    const mjSEPCLOSED: i32 = 1000;

    // SAFETY: s is a valid pointer to mjuiSection (caller contract)
    unsafe {
        let mut skip: i32 = 0;

        // process section items
        for i in 0..(*s).nitem {
            let it: *mut mjuiItem = (*s).item.as_mut_ptr().add(i as usize);

            // pass 0: nothing is skipped
            if pass == 0 {
                (*it).skip = 0;
                continue;
            }

            // item is a separator: update skip state for subsequent items
            if (*it).r#type == mjITEM_SEPARATOR {
                skip = ((*it).state == mjSEPCLOSED) as i32;
            }
            // item is not a separator: set skip state
            else {
                (*it).skip = skip;
            }
        }
    }
}

/// C: tryresize (ui/ui_main.c:1528)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/ui/ui_main.c:_SCL, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/ui/ui_main.c:_setitemskip
#[allow(unused_variables, non_snake_case)]
pub fn tryresize(ui: *mut mjUI, con: *const mjrContext) {
    const mjITEM_SEPARATOR: i32 = 0;
    const mjITEM_STATIC: i32 = 1;
    const mjITEM_BUTTON: i32 = 2;
    const mjITEM_CHECKINT: i32 = 3;
    const mjITEM_CHECKBYTE: i32 = 4;
    const mjITEM_RADIO: i32 = 5;
    const mjITEM_RADIOLINE: i32 = 6;
    const mjSECT_FIXED: i32 = 2;
    const mjSECT_CLOSED: i32 = 0;

    // SAFETY: ui, con are valid pointers (caller contract)
    unsafe {
        // scale theme sizes
        let spacing_ptr = std::ptr::addr_of!((*ui).spacing) as *const i32;
        let w_master: i32 = SCL(*spacing_ptr.add(0), con);   // spacing.total
        let w_scroll: i32 = SCL(*spacing_ptr.add(1), con);   // spacing.scroll
        let g_section: i32 = SCL(*spacing_ptr.add(3), con);  // spacing.section
        let g_itemside: i32 = SCL(*spacing_ptr.add(6), con); // spacing.itemside
        let g_itemmid: i32 = SCL(*spacing_ptr.add(7), con);  // spacing.itemmid
        let g_itemver: i32 = SCL(*spacing_ptr.add(8), con);  // spacing.itemver
        let g_textver: i32 = SCL(*spacing_ptr.add(10), con); // spacing.textver
        let g_label: i32 = SCL(*spacing_ptr.add(2), con);    // spacing.label

        // text element height, with gap above and below
        let textheight: i32 = (*con).charHeight + 2 * g_textver;

        // column width
        let colwidth: i32 = (w_master - w_scroll - 2 * g_section - 2 * g_itemside - g_itemmid) / 2;

        // pass 0 includes skipped items, pass 1 does not
        let mut Height: i32 = 0;
        let mut MaxHeight: i32 = 0;
        for pass in 0..2 {
            // init UI heights
            let mut height: i32 = 0;
            let mut maxheight: i32 = 0;

            // process sections
            for n in 0..(*ui).nsect {
                // vertical padding before section
                height += g_section;
                maxheight += g_section;

                // get section pointer
                let s: *mut mjuiSection = (*ui).sect.as_mut_ptr().add(n as usize);

                // set item skip flags for section, depending on pass
                setitemskip(s, pass);

                // title rectangle
                (*s).rtitle.left = g_section;
                (*s).rtitle.width = w_master - w_scroll - 2 * g_section;
                if (*s).state == mjSECT_FIXED {
                    // fixed section: no title
                    (*s).rtitle.bottom = height;
                    (*s).rtitle.height = 0;
                } else {
                    // regular section with title
                    (*s).rtitle.bottom = height + textheight;
                    (*s).rtitle.height = textheight;
                }

                // count title height
                height += (*s).rtitle.height;
                maxheight += (*s).rtitle.height;

                // init content rectangle
                (*s).rcontent.left = (*s).rtitle.left;
                (*s).rcontent.width = (*s).rtitle.width;
                (*s).rcontent.height = 0;
                (*s).rcontent.bottom = 0;

                // process items within section
                let mut i: i32 = 0;
                while i < (*s).nitem {
                    // get item pointer, clear rectangle
                    let it: *mut mjuiItem = (*s).item.as_mut_ptr().add(i as usize);
                    (*it).rect = mjrRect { left: 0, bottom: 0, width: 0, height: 0 };

                    // item is skipped: nothing to do
                    if (*it).skip != 0 {
                        i += 1;
                        continue;
                    }

                    // vertical padding before item
                    if (*it).r#type == mjITEM_SEPARATOR {
                        (*s).rcontent.height += g_section;
                    } else {
                        (*s).rcontent.height += g_itemver;
                    }

                    // packed pair of items
                    if i < (*s).nitem - 1
                        && (*(*s).item.as_ptr().add((i + 1) as usize)).r#type == (*it).r#type
                        && ((*it).r#type == mjITEM_BUTTON
                            || (*it).r#type == mjITEM_CHECKINT
                            || (*it).r#type == mjITEM_CHECKBYTE)
                    {
                        // get next item pointer
                        let it1: *mut mjuiItem = (*s).item.as_mut_ptr().add((i + 1) as usize);

                        // this item rectangle
                        (*it).rect.left = (*s).rcontent.left + g_itemside;
                        (*it).rect.width = colwidth;
                        (*it).rect.height = textheight;

                        // next item rectangle (set bottom here)
                        (*it1).rect.left = (*s).rcontent.left + g_itemside + colwidth + g_itemmid;
                        (*it1).rect.width = colwidth;
                        (*it1).rect.height = textheight;
                        (*it1).rect.bottom = height + (*s).rcontent.height + (*it).rect.height;

                        // advance
                        i += 1;
                    }
                    // single-line item
                    else {
                        // common left border
                        (*it).rect.left = (*s).rcontent.left + g_itemside;

                        // static
                        if (*it).r#type == mjITEM_STATIC {
                            (*it).rect.width = (*s).rcontent.width - 2 * g_itemside;
                            // access multi.nelem (offset 0 in union)
                            let nelem: i32 = *((*it).__anon_7._data.as_ptr() as *const i32);
                            (*it).rect.height = ((*con).charHeight + g_textver) * nelem;
                        }
                        // single column
                        else if (*it).r#type == mjITEM_BUTTON
                            || (*it).r#type == mjITEM_CHECKINT
                            || (*it).r#type == mjITEM_CHECKBYTE
                        {
                            (*it).rect.width = colwidth;
                            (*it).rect.height = textheight;
                        }
                        // radio
                        else if (*it).r#type == mjITEM_RADIO {
                            let ncol: i32 = if (*ui).radiocol != 0 { (*ui).radiocol } else { 2 };
                            let nelem: i32 = *((*it).__anon_7._data.as_ptr() as *const i32);
                            let nrow: i32 = (nelem - 1) / ncol + 1;
                            (*it).rect.width = (*s).rcontent.width - 2 * g_itemside;
                            (*it).rect.height = textheight * nrow;
                        }
                        // separator, select, slider, edit, radioline
                        else {
                            (*it).rect.width = (*s).rcontent.width - 2 * g_itemside;
                            (*it).rect.height = textheight;
                        }

                        // add room for label
                        if (*it).name[0] != 0
                            && ((*it).r#type >= mjITEM_RADIO
                                || (*it).r#type >= mjITEM_RADIOLINE
                                || (*it).r#type == mjITEM_STATIC)
                        {
                            (*it).rect.left = (*s).rcontent.left + g_itemside + g_label;
                            (*it).rect.width = (*s).rcontent.width - (2 * g_itemside + g_label);
                        }
                    }

                    // set bottom, count height
                    (*it).rect.bottom = height + (*s).rcontent.height + (*it).rect.height;
                    (*s).rcontent.height += (*it).rect.height;

                    i += 1;
                }

                // vertical padding after last item, compute bottom
                (*s).rcontent.height += g_itemver;
                (*s).rcontent.bottom = height + (*s).rcontent.height;

                // count content height
                if (*s).state != mjSECT_CLOSED {
                    height += (*s).rcontent.height;
                }
                maxheight += (*s).rcontent.height;
            }

            // vertical padding after last section
            height += g_section;
            maxheight += g_section;

            // save data: maxheight from pass 0, height from pass 1
            if pass == 0 {
                MaxHeight = maxheight;
            } else {
                Height = height;
            }
        }

        // invert bottom for all sections and items
        for n in 0..(*ui).nsect {
            // section
            let s: *mut mjuiSection = (*ui).sect.as_mut_ptr().add(n as usize);
            (*s).rtitle.bottom = Height - (*s).rtitle.bottom;
            (*s).rcontent.bottom = Height - (*s).rcontent.bottom;

            // items
            for i in 0..(*s).nitem {
                (*s).item[i as usize].rect.bottom = Height - (*s).item[i as usize].rect.bottom;
            }
        }

        // assign UI sizes
        (*ui).width = w_master;
        (*ui).height = Height;
        (*ui).maxheight = MaxHeight;
    }
}

/// C: evalpredicate (ui/ui_main.c:1823)
#[allow(unused_variables, non_snake_case)]
pub fn evalpredicate(state: i32, predicate: mjfItemEnable, userdata: *mut ()) -> i32 {
    if state <= 0 {
        return 0;
    } else if state == 1 {
        return 1;
    }

    // SAFETY: predicate._data is an 8-byte function pointer; if null (all zeros), treat as no predicate
    unsafe {
        let fptr = *(predicate._data.as_ptr() as *const Option<unsafe extern "C" fn(i32, *mut ()) -> i32>);
        match fptr {
            None => 1,
            Some(f) => f(state, userdata),
        }
    }
}

/// C: mjui_themeSpacing (ui/ui_main.h:26)
#[allow(unused_variables, non_snake_case)]
pub fn mjui_themeSpacing(ind: i32) -> mjuiThemeSpacing {
    // C: if (ind == 0) return themeSpacing0; else return themeSpacing1;
    // Named field init — exact values from ui_main.c:29-62
    if ind == 0 {
        mjuiThemeSpacing {
            total:      270,
            scroll:     15,
            label:      120,
            section:    8,
            cornersect: 6,
            cornersep:  6,
            itemside:   4,
            itemmid:    4,
            itemver:    4,
            texthor:    8,
            textver:    4,
            linescroll: 30,
            samples:    4,
        }
    } else {
        mjuiThemeSpacing {
            total:      310,
            scroll:     15,
            label:      120,
            section:    10,
            cornersect: 10,
            cornersep:  10,
            itemside:   7,
            itemmid:    7,
            itemver:    7,
            texthor:    10,
            textver:    5,
            linescroll: 30,
            samples:    4,
        }
    }
}

/// C: mjui_themeColor (ui/ui_main.h:29)
#[allow(unused_variables, non_snake_case)]
pub fn mjui_themeColor(ind: i32) -> mjuiThemeColor {
    // C: if (ind == 0) return themeColor0; ... else return themeColor3;
    // Values from ui_main.c:65-196. Named field init — no unsafe, no zeroed().
    match ind {
        0 => mjuiThemeColor {
            master: [0.25, 0.25, 0.25], thumb: [0.12, 0.12, 0.12],
            secttitle: [0.6, 0.2, 0.2], secttitle2: [0.1, 0.1, 0.1],
            secttitleuncheck: [0.45, 0.17, 0.17], secttitleuncheck2: [0.45, 0.17, 0.17],
            secttitlecheck: [0.45, 0.17, 0.17], secttitlecheck2: [0.45, 0.17, 0.17],
            sectfont: [1.0, 1.0, 1.0], sectsymbol: [0.7, 0.7, 0.7],
            sectpane: [0.1, 0.1, 0.1], separator: [0.25, 0.25, 0.25],
            separator2: [0.1, 0.1, 0.1], shortcut: [0.0, 0.0, 1.0],
            fontactive: [1.0, 1.0, 1.0], fontinactive: [0.5, 0.5, 0.5],
            decorinactive: [0.3, 0.3, 0.3], decorinactive2: [0.4, 0.4, 0.4],
            button: [0.6, 0.4, 0.4], check: [0.4, 0.4, 0.7],
            radio: [0.4, 0.6, 0.4], select: [0.4, 0.6, 0.6],
            select2: [0.2, 0.3, 0.3], slider: [0.3, 0.2, 0.3],
            slider2: [0.6, 0.4, 0.6], edit: [0.6, 0.6, 0.4],
            edit2: [0.7, 0.0, 0.0], cursor: [0.9, 0.9, 0.9],
        },
        1 => mjuiThemeColor {
            master: [0.2, 0.2, 0.2], thumb: [0.12, 0.12, 0.12],
            secttitle: [0.3, 0.3, 0.3], secttitle2: [0.15, 0.15, 0.15],
            secttitleuncheck: [0.25, 0.25, 0.25], secttitleuncheck2: [0.25, 0.25, 0.25],
            secttitlecheck: [0.25, 0.25, 0.25], secttitlecheck2: [0.25, 0.25, 0.25],
            sectfont: [0.8, 0.8, 0.8], sectsymbol: [0.7, 0.7, 0.7],
            sectpane: [0.15, 0.15, 0.15], separator: [0.2, 0.2, 0.2],
            separator2: [0.15, 0.15, 0.15], shortcut: [0.0, 0.0, 1.0],
            fontactive: [0.9, 0.9, 0.9], fontinactive: [0.5, 0.5, 0.5],
            decorinactive: [0.2, 0.2, 0.2], decorinactive2: [0.25, 0.25, 0.25],
            button: [0.6, 0.4, 0.2], check: [0.6, 0.4, 0.2],
            radio: [0.6, 0.4, 0.2], select: [0.6, 0.4, 0.2],
            select2: [0.3, 0.2, 0.1], slider: [0.2, 0.2, 0.2],
            slider2: [0.6, 0.4, 0.2], edit: [0.6, 0.4, 0.2],
            edit2: [0.7, 0.0, 0.0], cursor: [0.9, 0.9, 0.9],
        },
        2 => mjuiThemeColor {
            master: [0.9, 0.9, 0.9], thumb: [0.7, 0.7, 0.7],
            secttitle: [0.8, 0.8, 0.8], secttitle2: [1.0, 1.0, 1.0],
            secttitleuncheck: [0.95, 0.95, 0.95], secttitleuncheck2: [0.95, 0.95, 0.95],
            secttitlecheck: [0.95, 0.95, 0.95], secttitlecheck2: [0.95, 0.95, 0.95],
            sectfont: [0.0, 0.0, 0.8], sectsymbol: [0.0, 0.0, 0.8],
            sectpane: [1.0, 1.0, 1.0], separator: [0.9, 0.9, 0.9],
            separator2: [1.0, 1.0, 1.0], shortcut: [0.0, 1.0, 1.0],
            fontactive: [0.0, 0.0, 0.0], fontinactive: [0.7, 0.7, 0.7],
            decorinactive: [0.95, 0.95, 0.95], decorinactive2: [0.9, 0.9, 0.9],
            button: [0.8, 0.8, 0.8], check: [0.8, 0.8, 0.8],
            radio: [0.8, 0.8, 0.8], select: [0.8, 0.8, 0.8],
            select2: [0.9, 0.9, 0.9], slider: [0.95, 0.95, 0.95],
            slider2: [0.8, 0.8, 0.8], edit: [0.8, 0.8, 0.8],
            edit2: [1.0, 0.3, 0.3], cursor: [0.2, 0.2, 0.2],
        },
        _ => mjuiThemeColor {
            master: [0.15, 0.15, 0.15], thumb: [0.3, 0.3, 0.3],
            secttitle: [0.25, 0.25, 0.25], secttitle2: [0.0, 0.0, 0.0],
            secttitleuncheck: [0.2, 0.2, 0.2], secttitleuncheck2: [0.2, 0.2, 0.2],
            secttitlecheck: [0.2, 0.2, 0.2], secttitlecheck2: [0.2, 0.2, 0.2],
            sectfont: [1.0, 0.3, 0.3], sectsymbol: [1.0, 0.3, 0.3],
            sectpane: [0.0, 0.0, 0.0], separator: [0.15, 0.15, 0.15],
            separator2: [0.0, 0.0, 0.0], shortcut: [0.0, 0.0, 1.0],
            fontactive: [1.0, 1.0, 1.0], fontinactive: [0.4, 0.4, 0.4],
            decorinactive: [0.1, 0.1, 0.1], decorinactive2: [0.15, 0.15, 0.15],
            button: [0.3, 0.3, 0.3], check: [0.3, 0.3, 0.3],
            radio: [0.3, 0.3, 0.3], select: [0.3, 0.3, 0.3],
            select2: [0.15, 0.15, 0.15], slider: [0.15, 0.15, 0.15],
            slider2: [0.3, 0.3, 0.3], edit: [0.3, 0.3, 0.3],
            edit2: [0.8, 0.2, 0.2], cursor: [0.8, 0.8, 0.8],
        },
    }
}

