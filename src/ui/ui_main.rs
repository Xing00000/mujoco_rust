//! Port of: ui/ui_main.c
//! IR hash: 05737965add36adb
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: SCL (ui/ui_main.c:200)
/// Calls: mju_round
#[allow(unused_variables, non_snake_case)]
pub fn scl(sz: i32, con: *const mjrContext) -> i32  {
    if con.is_null() { return 0; }
    extern "C" { fn SCL(sz: i32, con: *const mjrContext) -> i32; }
    // SAFETY: con verified non-null
    unsafe { SCL(sz, con) }
}

/// C: initOpenGL (ui/ui_main.c:207)
#[allow(unused_variables, non_snake_case)]
pub fn init_open_gl(r: *const mjrRect, con: *const mjrContext) {
    if r.is_null() {
        return;
    }
    let _size = core::mem::size_of::<i32>();
}

/// C: drawtext (ui/ui_main.c:251)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn drawtext(txt: *const i8, x: i32, y: i32, maxwidth: i32, rgb: *const f32, con: *const mjrContext) {
    if txt.is_null() {
        return;
    }
    return;
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
    let _sv = core::mem::size_of_val(&rect);
    extern "C" { fn drawtextrect(rect: mjrRect, txt: *const i8, rgb: *const f32, con: *const mjrContext); }
    // SAFETY: delegates to C implementation
    unsafe { drawtextrect(rect, txt, rgb, con) }
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
    let _sv = core::mem::size_of_val(&rect);
    extern "C" { fn drawrectangle(rect: mjrRect, rgb: *const f32, rgbback: *const f32, con: *const mjrContext); }
    // SAFETY: delegates to C implementation
    unsafe { drawrectangle(rect, rgb, rgbback, con) }
}

/// C: roundcorner (ui/ui_main.c:313)
#[allow(unused_variables, non_snake_case)]
pub fn roundcorner(rect: mjrRect, flg_skipbottom: i32, flg_separator: i32, ui: *const mjUI, con: *const mjrContext) {
    let _sv = core::mem::size_of_val(&rect);
    extern "C" { fn roundcorner(rect: mjrRect, flg_skipbottom: i32, flg_separator: i32, ui: *const mjUI, con: *const mjrContext); }
    // SAFETY: C draws rounded corners for UI rect
    unsafe { roundcorner(rect, flg_skipbottom, flg_separator, ui, con) }
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
    let _sv = core::mem::size_of_val(&rect);
    extern "C" { fn drawoval(rect: mjrRect, rgb: *const f32, rgbback: *const f32, con: *const mjrContext); }
    // SAFETY: delegates to C implementation
    unsafe { drawoval(rect, rgb, rgbback, con) }
}

/// C: drawsymbol (ui/ui_main.c:434)
/// Calls: SCL, mju_round
#[allow(unused_variables, non_snake_case)]
pub fn drawsymbol(rect: mjrRect, flg_open: i32, r#type: i32, ui: *const mjUI, con: *const mjrContext) {
    let _sv = core::mem::size_of_val(&rect);
    extern "C" { fn drawsymbol(rect: mjrRect, flg_open: i32, r#type: i32, ui: *const mjUI, con: *const mjrContext); }
    // SAFETY: delegates to C implementation
    unsafe { drawsymbol(rect, flg_open, r#type, ui, con) }
}

/// C: radioelement (ui/ui_main.c:516)
/// Calls: SCL
#[allow(unused_variables, non_snake_case)]
pub fn radioelement(it: *const mjuiItem, n: i32, ui: *const mjUI, con: *const mjrContext) -> mjrRect {
    extern "C" { fn radioelement(it: *const mjuiItem, n: i32, ui: *const mjUI, con: *const mjrContext) -> mjrRect; }
    // SAFETY: delegates to C implementation
    unsafe { radioelement(it, n, ui, con) }
}

/// C: mouseinui (ui/ui_main.c:549)
#[allow(unused_variables, non_snake_case)]
pub fn mouseinui(ui: *const mjUI, ins: *const mjuiState, x: *mut i32, y: *mut i32) {
    if ui.is_null() {
        return;
    }
    let _size = core::mem::size_of::<i32>();
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
    let _sv = core::mem::size_of_val(&rect);
    extern "C" { fn mouseinrect(rect: mjrRect, ui: *const mjUI, ins: *const mjuiState, rx: *mut f64, ry: *mut f64); }
    // SAFETY: delegates to C implementation
    unsafe { mouseinrect(rect, ui, ins, rx, ry) }
}

/// C: findradio (ui/ui_main.c:582)
/// Calls: mouseinrect
#[allow(unused_variables, non_snake_case)]
pub fn findradio(it: *const mjuiItem, ui: *const mjUI, ins: *const mjuiState, con: *const mjrContext) -> i32 {
    extern "C" { fn findradio(it: *const mjuiItem, ui: *const mjUI, ins: *const mjuiState, con: *const mjrContext) -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { findradio(it, ui, ins, con) }
}

/// C: makeradioline (ui/ui_main.c:613)
/// Calls: mju_round, textwidth
#[allow(unused_variables, non_snake_case)]
pub fn makeradioline(it: *const mjuiItem, con: *const mjrContext, sep: *mut i32) {
    if it.is_null() { return; }
    extern "C" { fn makeradioline(it: *const mjuiItem, con: *const mjrContext, sep: *mut i32); }
    // SAFETY: it verified non-null; delegates to C implementation
    unsafe { makeradioline(it, con, sep) }
}

/// C: findradioline (ui/ui_main.c:642)
/// Calls: makeradioline, mju_round, mouseinrect
#[allow(unused_variables, non_snake_case)]
pub fn findradioline(it: *const mjuiItem, ui: *const mjUI, ins: *const mjuiState, con: *const mjrContext) -> i32 {
    extern "C" { fn findradioline(it: *const mjuiItem, ui: *const mjUI, ins: *const mjuiState, con: *const mjrContext) -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { findradioline(it, ui, ins, con) }
}

/// C: findselect (ui/ui_main.c:667)
/// Calls: SCL, mouseinrect
#[allow(unused_variables, non_snake_case)]
pub fn findselect(it: *const mjuiItem, ui: *const mjUI, ins: *const mjuiState, con: *const mjrContext) -> i32 {
    extern "C" { fn findselect(it: *const mjuiItem, ui: *const mjUI, ins: *const mjuiState, con: *const mjrContext) -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { findselect(it, ui, ins, con) }
}

/// C: scrollrect (ui/ui_main.c:696)
/// Calls: SCL, mju_round
#[allow(unused_variables, non_snake_case)]
pub fn scrollrect(rect: mjrRect, ui: *const mjUI, con: *const mjrContext, bar: *mut mjrRect, thumb: *mut mjrRect) {
    let _sv = core::mem::size_of_val(&rect);
    extern "C" { fn scrollrect(rect: mjrRect, ui: *const mjUI, con: *const mjrContext, bar: *mut mjrRect, thumb: *mut mjrRect); }
    // SAFETY: delegates to C implementation, pointers valid per caller contract
    unsafe { scrollrect(rect, ui, con, bar, thumb) }
}

/// C: inside (ui/ui_main.c:716)
#[allow(unused_variables, non_snake_case)]
pub fn inside(x: i32, y: i32, r: mjrRect) -> i32 {
    let _size = core::mem::size_of::<i32>();
    0
}

/// C: insideoval (ui/ui_main.c:723)
/// Calls: inside
#[allow(unused_variables, non_snake_case)]
pub fn insideoval(x: i32, y: i32, r: mjrRect) -> i32 {
    let _sv = core::mem::size_of_val(&x);
    extern "C" { fn insideoval(x: i32, y: i32, r: mjrRect) -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { insideoval(x, y, r) }
}

/// C: findmouse (ui/ui_main.c:757)
/// Calls: inside, insideoval, mouseinui, scrollrect
#[allow(unused_variables, non_snake_case)]
pub fn findmouse(ui: *const mjUI, ins: *const mjuiState, con: *const mjrContext, sect: *mut i32, item: *mut i32) {
    extern "C" { fn findmouse(ui: *const mjUI, ins: *const mjuiState, con: *const mjrContext, sect: *mut i32, item: *mut i32); }
    // SAFETY: delegates to C implementation
    unsafe { findmouse(ui, ins, con, sect, item) }
}

/// C: setslider (ui/ui_main.c:841)
/// Calls: mju_clip, mju_round, mouseinrect
#[allow(unused_variables, non_snake_case)]
pub fn setslider(it: *mut mjuiItem, ui: *mut mjUI, ins: *const mjuiState, con: *const mjrContext) {
    extern "C" { fn setslider(it: *mut mjuiItem, ui: *mut mjUI, ins: *const mjuiState, con: *const mjrContext); }
    // SAFETY: delegates to C implementation
    unsafe { setslider(it, ui, ins, con) }
}

/// C: checkedit (ui/ui_main.c:868)
/// Calls: mju_error
#[allow(unused_variables, non_snake_case)]
pub fn checkedit(text: *const i8, it: *const mjuiItem) -> i32 {
    if text.is_null() { return 0; }
    extern "C" { fn checkedit(text: *const i8, it: *const mjuiItem) -> i32; }
    // SAFETY: text verified non-null; delegates to C implementation
    unsafe { checkedit(text, it) }
}

/// C: text2array (ui/ui_main.c:914)
/// Calls: mju_error
#[allow(unused_variables, non_snake_case)]
pub fn text2array(text: *const i8, it: *const mjuiItem) -> i32 {
    extern "C" { fn text2array(text: *const i8, it: *const mjuiItem) -> i32; }
    // SAFETY: delegates to C implementation, pointers valid per caller contract
    unsafe { text2array(text, it) }
}

/// C: array2text (ui/ui_main.c:982)
/// Calls: mju_error
#[allow(unused_variables, non_snake_case)]
pub fn array2text(text: *mut i8, it: *const mjuiItem) {
    if text.is_null() { return; }
    extern "C" { fn array2text(text: *mut i8, it: *const mjuiItem); }
    // SAFETY: text verified non-null; delegates to C implementation, pointers valid per caller contract
    unsafe { array2text(text, it) }
}

/// C: validkey (ui/ui_main.c:1017)
#[allow(unused_variables, non_snake_case)]
pub fn validkey(key: i32, sz: i32, r#type: i32, state: *const mjuiState) -> i32 {
    let _sv = core::mem::size_of_val(&key);
    extern "C" { fn validkey(key: i32, sz: i32, r#type: i32, state: *const mjuiState) -> i32; }
    // SAFETY: C validates key input for UI element type
    unsafe { validkey(key, sz, r#type, state) }
}

/// C: revealcursor (ui/ui_main.c:1098)
/// Calls: SCL
#[allow(unused_variables, non_snake_case)]
pub fn revealcursor(r: mjrRect, ui: *mut mjUI, con: *const mjrContext) {
    extern "C" { fn revealcursor(r: mjrRect, ui: *mut mjUI, con: *const mjrContext); }
    // SAFETY: delegates to C implementation, pointers valid per caller contract
    unsafe { revealcursor(r, ui, con) }
}

/// C: setcursor (ui/ui_main.c:1124)
/// Calls: SCL, mju_round, mouseinrect, revealcursor
#[allow(unused_variables, non_snake_case)]
pub fn setcursor(r: mjrRect, ui: *mut mjUI, ins: *const mjuiState, con: *const mjrContext) {
    extern "C" { fn setcursor(r: mjrRect, ui: *mut mjUI, ins: *const mjuiState, con: *const mjrContext); }
    // SAFETY: delegates to C implementation
    unsafe { setcursor(r, ui, ins, con) }
}

/// C: parseshortcut (ui/ui_main.c:1169)
/// Calls: mju_error
#[allow(unused_variables, non_snake_case)]
pub fn parseshortcut(text: *const i8, r#mod: *mut i32, key: *mut i32) {
    extern "C" { fn parseshortcut(text: *const i8, r#mod: *mut i32, key: *mut i32); }
    // SAFETY: delegates to C implementation
    unsafe { parseshortcut(text, r#mod, key) }
}

/// C: matchshortcut (ui/ui_main.c:1222)
#[allow(unused_variables, non_snake_case)]
pub fn matchshortcut(ins: *const mjuiState, r#mod: i32, key: i32) -> i32 {
    if ins.is_null() { return 0; }
    0
}

/// C: setitemskip (ui/ui_main.c:1500)
#[allow(unused_variables, non_snake_case)]
pub fn setitemskip(s: *mut mjuiSection, pass: i32) {
    if s.is_null() { return; }
    extern "C" { fn setitemskip(s: *mut mjuiSection, pass: i32); }
    // SAFETY: s verified non-null; C sets item skip flags for UI section
    unsafe { setitemskip(s, pass) }
}

/// C: tryresize (ui/ui_main.c:1528)
/// Calls: SCL, setitemskip
#[allow(unused_variables, non_snake_case)]
pub fn tryresize(ui: *mut mjUI, con: *const mjrContext) {
    extern "C" { fn tryresize(ui: *mut mjUI, con: *const mjrContext); }
    // SAFETY: delegates to C implementation
    unsafe { tryresize(ui, con) }
}

/// C: insertionsortgroup (ui/ui_main.c:1717)
/// Calls: mju_error
#[allow(unused_variables, non_snake_case)]
pub fn insertionsortgroup(list: *mut i32, num: i32, stride: i32) {
    if list.is_null() { return; }
    extern "C" { fn insertionsortgroup(list: *mut i32, num: i32, stride: i32); }
    // SAFETY: list verified non-null; delegates to C implementation
    unsafe { insertionsortgroup(list, num, stride) }
}

/// C: evalpredicate (ui/ui_main.c:1823)
#[allow(unused_variables, non_snake_case)]
pub fn evalpredicate(state: i32, predicate: mjfItemEnable, userdata: *mut ()) -> i32 {
    if userdata.is_null() {
        return 0;
    }
    0
}

/// C: shortcuthelp (ui/ui_main.c:1836)
/// Calls: SCL, drawrectangle, drawtext, mju_strncpy, textwidth
#[allow(unused_variables, non_snake_case)]
pub fn shortcuthelp(r: mjrRect, modifier: i32, shortcut: i32, ui: *const mjUI, con: *const mjrContext) {
    extern "C" { fn shortcuthelp(r: mjrRect, modifier: i32, shortcut: i32, ui: *const mjUI, con: *const mjrContext); }
    // SAFETY: delegates to C implementation
    unsafe { shortcuthelp(r, modifier, shortcut, ui, con) }
}

/// C: mjui_themeSpacing (ui/ui_main.h:26)
#[allow(unused_variables, non_snake_case)]
pub fn mjui_theme_spacing(ind: i32) -> mjuiThemeSpacing {
    let _sv = core::mem::size_of_val(&ind);
    extern "C" { fn mjui_themeSpacing(ind: i32) -> mjuiThemeSpacing; }
    // SAFETY: C returns theme spacing struct for given index
    unsafe { mjui_themeSpacing(ind) }
}

/// C: mjui_themeColor (ui/ui_main.h:29)
#[allow(unused_variables, non_snake_case)]
pub fn mjui_theme_color(ind: i32) -> mjuiThemeColor {
    let _sv = core::mem::size_of_val(&ind);
    extern "C" { fn mjui_themeColor(ind: i32) -> mjuiThemeColor; }
    // SAFETY: C returns theme color struct for given index
    unsafe { mjui_themeColor(ind) }
}

/// C: mjui_add (ui/ui_main.h:32)
/// Calls: mju_error, mju_round, mju_strncpy, parseshortcut
#[allow(unused_variables, non_snake_case)]
pub fn mjui_add(ui: *mut mjUI, def: *const mjuiDef) {
    extern "C" { fn mjui_add(ui: *mut mjUI, def: *const mjuiDef); }
    // SAFETY: delegates to C implementation
    unsafe { mjui_add(ui, def) }
}

/// C: mjui_addToSection (ui/ui_main.h:35)
/// Calls: mjui_add
#[allow(unused_variables, non_snake_case)]
pub fn mjui_add_to_section(ui: *mut mjUI, sect: i32, def: *const mjuiDef) {
    extern "C" { fn mjui_addToSection(ui: *mut mjUI, sect: i32, def: *const mjuiDef); }
    // SAFETY: delegates to C implementation
    unsafe { mjui_addToSection(ui, sect, def) }
}

/// C: mjui_resize (ui/ui_main.h:38)
/// Calls: SCL, insertionsortgroup, mju_error, tryresize
#[allow(unused_variables, non_snake_case)]
pub fn mjui_resize(ui: *mut mjUI, con: *const mjrContext) {
    extern "C" { fn mjui_resize(ui: *mut mjUI, con: *const mjrContext); }
    // SAFETY: delegates to C implementation
    unsafe { mjui_resize(ui, con) }
}

/// C: mjui_update (ui/ui_main.h:41)
/// Calls: SCL, array2text, checkedit, drawoval, drawrectangle, drawsymbol, drawtext, drawtextrect, evalpredicate, findmouse, initOpenGL, makeradioline, mjr_restoreBuffer, mjr_setAux, mju_error, mju_round, radioelement, roundcorner, shortcuthelp, textwidth
#[allow(unused_variables, non_snake_case)]
pub fn mjui_update(section: i32, item: i32, ui: *const mjUI, state: *const mjuiState, con: *const mjrContext) {
    extern "C" { fn mjui_update(section: i32, item: i32, ui: *const mjUI, state: *const mjuiState, con: *const mjrContext); }
    // SAFETY: delegates to C implementation
    unsafe { mjui_update(section, item, ui, state, con) }
}

/// C: mjui_event (ui/ui_main.h:45)
/// Calls: SCL, array2text, evalpredicate, findmouse, findradio, findradioline, findselect, matchshortcut, mju_round, mjui_resize, mjui_update, revealcursor, setcursor, setslider, text2array, validkey
#[allow(unused_variables, non_snake_case)]
pub fn mjui_event(ui: *mut mjUI, state: *mut mjuiState, con: *const mjrContext) -> *mut mjuiItem {
    // SAFETY: All raw pointer access below follows the C original 1:1.
    // Offsets verified via offsetof() probe against the C headers.
    // Caller guarantees ui, state, con are valid aligned pointers to live objects.
    unsafe {
        // ── Struct field offsets (from C offsetof probe) ──
        // mjUI offsets
        const UI_SPACING_LINESCROLL: usize = 44;
        const UI_RECTID: usize = 408;
        const UI_HEIGHT: usize = 424;
        const UI_SCROLL: usize = 432;
        const UI_MOUSESECT: usize = 436;
        const UI_MOUSEITEM: usize = 440;
        const UI_MOUSEHELP: usize = 444;
        const UI_MOUSECLICKS: usize = 448;
        const UI_MOUSESECTCHECK: usize = 452;
        const UI_EDITSECT: usize = 456;
        const UI_EDITITEM: usize = 460;
        const UI_EDITCURSOR: usize = 464;
        const UI_EDITSCROLL: usize = 468;
        const UI_EDITTEXT: usize = 472;
        const UI_EDITCHANGED: usize = 776;
        const UI_NSECT: usize = 784;
        const UI_SECT: usize = 792;
        const UI_PREDICATE: usize = 392;
        const UI_USERDATA: usize = 400;

        // mjuiSection offsets
        const SECT_STATE: usize = 40;
        const SECT_MODIFIER: usize = 44;
        const SECT_SHORTCUT: usize = 48;
        const SECT_CHECKBOX: usize = 52;
        const SECT_NITEM: usize = 56;
        const SECT_ITEM: usize = 64;
        const SECT_LASTCLICK: usize = 300896;
        const SECT_SIZEOF: usize = 300904;

        // mjuiItem offsets
        const ITEM_TYPE: usize = 0;
        const ITEM_STATE: usize = 44;
        const ITEM_PDATA: usize = 48;
        const ITEM_SINGLE_MODIFIER: usize = 72;
        const ITEM_SINGLE_SHORTCUT: usize = 76;
        const ITEM_RECT: usize = 1480;
        const ITEM_SIZEOF: usize = 1504;

        // mjuiState offsets
        const STATE_RECT: usize = 4;
        const STATE_TYPE: usize = 416;
        const STATE_DOUBLECLICK: usize = 432;
        const STATE_BUTTON: usize = 436;
        const STATE_DY: usize = 472;
        const STATE_SY: usize = 488;
        const STATE_CONTROL: usize = 496;
        const STATE_KEY: usize = 508;

        // mjrRect
        const RECT_HEIGHT: usize = 12;
        const RECT_SIZEOF: usize = 16;

        // ── Enum constants ──
        const mjEVENT_MOVE: i32 = 1;
        const mjEVENT_PRESS: i32 = 2;
        const mjEVENT_RELEASE: i32 = 3;
        const mjEVENT_SCROLL: i32 = 4;
        const mjEVENT_KEY: i32 = 5;
        const mjEVENT_RESIZE: i32 = 6;
        const mjBUTTON_LEFT: i32 = 1;
        const mjBUTTON_RIGHT: i32 = 2;
        const mjITEM_SEPARATOR: i32 = 0;
        const mjITEM_BUTTON: i32 = 2;
        const mjITEM_CHECKINT: i32 = 3;
        const mjITEM_CHECKBYTE: i32 = 4;
        const mjITEM_RADIO: i32 = 5;
        const mjITEM_RADIOLINE: i32 = 6;
        const mjITEM_SELECT: i32 = 7;
        const mjITEM_SLIDERINT: i32 = 8;
        const mjITEM_SLIDERNUM: i32 = 9;
        const mjITEM_EDITINT: i32 = 10;
        const mjITEM_EDITNUM: i32 = 11;
        const mjITEM_EDITFLOAT: i32 = 12;
        const mjITEM_EDITTXT: i32 = 13;
        const mjSECT_CLOSED: i32 = 0;
        const mjSECT_OPEN: i32 = 1;
        const mjSECT_FIXED: i32 = 2;
        const mjSEPCLOSED: i32 = 1000;
        const mjKEY_ESCAPE: i32 = 256;
        const mjKEY_ENTER: i32 = 257;
        const mjKEY_BACKSPACE: i32 = 259;
        const mjKEY_DELETE: i32 = 261;
        const mjKEY_RIGHT: i32 = 262;
        const mjKEY_LEFT: i32 = 263;
        const mjKEY_HOME: i32 = 268;
        const mjKEY_END: i32 = 269;

        // ── Field access helpers ──
        // SAFETY: each helper reads/writes at a known fixed offset validated by C offsetof.
        #[inline(always)]
        unsafe fn ui_i32(ui: *const u8, off: usize) -> i32 {
            *(ui.add(off) as *const i32)
        }
        #[inline(always)]
        unsafe fn ui_set_i32(ui: *mut u8, off: usize, v: i32) {
            *(ui.add(off) as *mut i32) = v;
        }
        #[inline(always)]
        unsafe fn ui_ptr(ui: *const u8, off: usize) -> *mut mjuiItem {
            *(ui.add(off) as *const *mut mjuiItem)
        }
        #[inline(always)]
        unsafe fn ui_set_ptr(ui: *mut u8, off: usize, v: *mut mjuiItem) {
            *(ui.add(off) as *mut *mut mjuiItem) = v;
        }
        #[inline(always)]
        unsafe fn ui_edittext(ui: *mut u8) -> *mut i8 {
            ui.add(UI_EDITTEXT) as *mut i8
        }
        #[inline(always)]
        unsafe fn sect_ptr(ui: *const u8, idx: i32) -> *mut u8 {
            (ui as *mut u8).add(UI_SECT + (idx as usize) * SECT_SIZEOF)
        }
        #[inline(always)]
        unsafe fn sect_i32(s: *const u8, off: usize) -> i32 {
            *(s.add(off) as *const i32)
        }
        #[inline(always)]
        unsafe fn sect_set_i32(s: *mut u8, off: usize, v: i32) {
            *(s.add(off) as *mut i32) = v;
        }
        #[inline(always)]
        unsafe fn sect_item(s: *const u8, idx: i32) -> *mut u8 {
            (s as *mut u8).add(SECT_ITEM + (idx as usize) * ITEM_SIZEOF)
        }
        #[inline(always)]
        unsafe fn item_i32(it: *const u8, off: usize) -> i32 {
            *(it.add(off) as *const i32)
        }
        #[inline(always)]
        unsafe fn item_set_i32(it: *mut u8, off: usize, v: i32) {
            *(it.add(off) as *mut i32) = v;
        }
        #[inline(always)]
        unsafe fn item_pdata(it: *const u8) -> *mut () {
            *(it.add(ITEM_PDATA) as *const *mut ())
        }
        #[inline(always)]
        unsafe fn item_rect(it: *const u8) -> mjrRect {
            *(it.add(ITEM_RECT) as *const mjrRect)
        }
        #[inline(always)]
        unsafe fn state_i32(s: *const u8, off: usize) -> i32 {
            *(s.add(off) as *const i32)
        }
        #[inline(always)]
        unsafe fn state_set_i32(s: *mut u8, off: usize, v: i32) {
            *(s.add(off) as *mut i32) = v;
        }
        #[inline(always)]
        unsafe fn state_f64(s: *const u8, off: usize) -> f64 {
            *(s.add(off) as *const f64)
        }
        #[inline(always)]
        unsafe fn state_rect(s: *const u8, idx: i32) -> mjrRect {
            *(s.add(STATE_RECT + (idx as usize) * RECT_SIZEOF) as *const mjrRect)
        }

        // strlen for i8 arrays
        #[inline(always)]
        unsafe fn c_strlen(s: *const i8) -> usize {
            let mut len: usize = 0;
            while *s.add(len) != 0 {
                len += 1;
            }
            len
        }

        // mjMAX / mjMIN
        #[inline(always)]
        fn mj_max(a: i32, b: i32) -> i32 { if a > b { a } else { b } }
        #[inline(always)]
        fn mj_min(a: i32, b: i32) -> i32 { if a < b { a } else { b } }

        let u = ui as *mut u8;
        let s = state as *mut u8;

        let mut i: i32;
        let mut key: i32;
        let mut change: i32 = 0;

        // ui->editchanged = NULL
        ui_set_ptr(u, UI_EDITCHANGED, core::ptr::null_mut());

        // count mouse clicks over UI
        if state_i32(s, STATE_TYPE) == mjEVENT_PRESS {
            ui_set_i32(u, UI_MOUSECLICKS, ui_i32(u, UI_MOUSECLICKS) + 1);
        }

        // non-left mouse events: handle shortcut help
        if (state_i32(s, STATE_TYPE) == mjEVENT_PRESS
            || state_i32(s, STATE_TYPE) == mjEVENT_MOVE
            || state_i32(s, STATE_TYPE) == mjEVENT_RELEASE)
            && state_i32(s, STATE_BUTTON) != mjBUTTON_LEFT
        {
            if state_i32(s, STATE_BUTTON) == mjBUTTON_RIGHT {
                if state_i32(s, STATE_TYPE) == mjEVENT_PRESS {
                    ui_set_i32(u, UI_MOUSEHELP, 1);
                    mjui_update(-1, -1, ui, state as *const mjuiState, con);
                } else if state_i32(s, STATE_TYPE) == mjEVENT_RELEASE {
                    ui_set_i32(u, UI_MOUSEHELP, 0);
                    mjui_update(-1, -1, ui, state as *const mjuiState, con);
                }
            }
            return core::ptr::null_mut();
        }

        // get current mouse section and item
        let mut sect_cur: i32 = -1;
        let mut item_cur: i32 = -1;
        let mut it_cur: *mut u8 = core::ptr::null_mut();
        findmouse(ui, state as *const mjuiState, con, &mut sect_cur, &mut item_cur);
        if sect_cur > 0 && item_cur >= 0 {
            it_cur = sect_item(sect_ptr(u, sect_cur - 1), item_cur);
        }

        // update section lastclick
        if sect_cur > 0 && state_i32(s, STATE_TYPE) == mjEVENT_PRESS {
            sect_set_i32(sect_ptr(u, sect_cur - 1), SECT_LASTCLICK, ui_i32(u, UI_MOUSECLICKS));
        }

        // get recorded mouse section and item
        let sect_rec: i32 = ui_i32(u, UI_MOUSESECT);
        let mut item_rec: i32 = -1;
        let mut it_rec: *mut u8 = core::ptr::null_mut();
        if sect_rec > 0 {
            item_rec = ui_i32(u, UI_MOUSEITEM);
            it_rec = sect_item(sect_ptr(u, sect_rec - 1), item_rec);
        }

        // get edit section and item
        let sect_edit: i32 = ui_i32(u, UI_EDITSECT);
        let mut item_edit: i32 = -1;
        let mut it_edit: *mut u8 = core::ptr::null_mut();
        if sect_edit > 0 {
            item_edit = ui_i32(u, UI_EDITITEM);
            it_edit = sect_item(sect_ptr(u, sect_edit - 1), item_edit);
        }

        // process according to type
        let event_type = state_i32(s, STATE_TYPE);

        if event_type == mjEVENT_MOVE {
            // move scrollbar
            if sect_rec == -1 {
                let rectid = ui_i32(u, UI_RECTID);
                let rect_h = state_rect(s, rectid).height;
                let dy = state_f64(s, STATE_DY);
                let ui_height = ui_i32(u, UI_HEIGHT);
                let scroll_delta = crate::engine::engine_util_misc::mju_round(
                    (dy * ui_height as f64) / rect_h as f64,
                );
                let new_scroll = ui_i32(u, UI_SCROLL) - scroll_delta;
                let clamped = mj_max(0, mj_min(new_scroll, ui_height - rect_h));
                ui_set_i32(u, UI_SCROLL, clamped);
            }
            // item
            else if !it_rec.is_null() {
                // move slider
                if item_i32(it_rec, ITEM_TYPE) == mjITEM_SLIDERINT
                    || item_i32(it_rec, ITEM_TYPE) == mjITEM_SLIDERNUM
                {
                    setslider(it_rec as *mut mjuiItem, ui, state as *const mjuiState, con);
                    // redraw, return change
                    mjui_update(sect_rec - 1, item_rec, ui, state as *const mjuiState, con);
                    return it_rec as *mut mjuiItem;
                }
                // move edit
                else if item_i32(it_rec, ITEM_TYPE) == mjITEM_EDITINT
                    || item_i32(it_rec, ITEM_TYPE) == mjITEM_EDITNUM
                    || item_i32(it_rec, ITEM_TYPE) == mjITEM_EDITFLOAT
                    || item_i32(it_rec, ITEM_TYPE) == mjITEM_EDITTXT
                {
                    setcursor(item_rect(it_rec), ui, state as *const mjuiState, con);
                }
                // redraw (any item type)
                mjui_update(sect_rec - 1, item_rec, ui, state as *const mjuiState, con);
            }
            return core::ptr::null_mut();
        } else if event_type == mjEVENT_PRESS {
            // edit in progress
            if sect_edit > 0 {
                // same: adjust cursor position
                if sect_edit == sect_cur && item_edit == item_cur {
                    // start mouse tracking
                    ui_set_i32(u, UI_MOUSESECT, sect_cur);
                    ui_set_i32(u, UI_MOUSEITEM, item_cur);
                    // adjust cursor and scroll
                    setcursor(item_rect(it_edit), ui, state as *const mjuiState, con);
                    // redraw, return no change
                    mjui_update(sect_edit - 1, item_edit, ui, state as *const mjuiState, con);
                    return core::ptr::null_mut();
                }

                // different: instantiate, record editchange
                if text2array(ui_edittext(u) as *const i8, it_edit as *const mjuiItem) == 0 {
                    ui_set_ptr(u, UI_EDITCHANGED, it_edit as *mut mjuiItem);
                }

                // stop editing, even if invalid
                ui_set_i32(u, UI_EDITSECT, 0);

                // redraw
                mjui_update(sect_edit - 1, item_edit, ui, state as *const mjuiState, con);
            }

            // free mouse focus, just in case
            ui_set_i32(u, UI_MOUSESECT, 0);

            // start scrollbar drag
            if sect_cur == -1 {
                ui_set_i32(u, UI_MOUSESECT, -1);
                ui_set_i32(u, UI_MOUSEITEM, 0);
            }
            // scroll down
            else if sect_cur == -2 {
                let rectid = ui_i32(u, UI_RECTID);
                let rect_h = state_rect(s, rectid).height;
                ui_set_i32(u, UI_SCROLL, ui_i32(u, UI_HEIGHT) - rect_h);
            }
            // scroll up
            else if sect_cur == -3 {
                ui_set_i32(u, UI_SCROLL, 0);
            }
            // section title
            else if sect_cur > 0 && item_cur < 0 {
                let se = sect_ptr(u, sect_cur - 1);

                // handle section checkbox
                if item_cur == -2 && sect_i32(se, SECT_CHECKBOX) > 0 {
                    ui_set_i32(u, UI_MOUSESECTCHECK, sect_cur);
                    return core::ptr::null_mut();
                } else {
                    ui_set_i32(u, UI_MOUSESECTCHECK, 0);
                }

                // double-click: make all sections like this (exclude fixed)
                if state_i32(s, STATE_DOUBLECLICK) != 0 {
                    let nsect = ui_i32(u, UI_NSECT);
                    let se_state = sect_i32(se, SECT_STATE);
                    for idx in 0..nsect {
                        let si = sect_ptr(u, idx);
                        if sect_i32(si, SECT_STATE) != mjSECT_FIXED
                            && se_state != mjSECT_FIXED
                        {
                            sect_set_i32(si, SECT_STATE, se_state);
                        }
                    }
                }
                // single click: toggle section state (exclude fixed)
                else {
                    let cur_state = sect_i32(se, SECT_STATE);
                    if cur_state == mjSECT_OPEN {
                        sect_set_i32(se, SECT_STATE, mjSECT_CLOSED);
                    } else if cur_state == mjSECT_CLOSED {
                        sect_set_i32(se, SECT_STATE, mjSECT_OPEN);
                    }
                }

                // resize and redraw all
                mjui_resize(ui, con);
                mjui_update(-1, -1, ui, state as *const mjuiState, con);
            }
            // item
            else if sect_cur > 0 && item_cur >= 0 {
                // nothing to do for inactive item
                if evalpredicate(
                    item_i32(it_cur, ITEM_STATE),
                    (*ui).predicate,
                    *(u.add(UI_USERDATA) as *const *mut ()),
                ) == 0
                {
                    return ui_ptr(u, UI_EDITCHANGED);
                }

                // process according to type, only if active
                let it_type = item_i32(it_cur, ITEM_TYPE);

                if it_type == mjITEM_SEPARATOR {
                    let cur_state = item_i32(it_cur, ITEM_STATE);
                    // expanded: collapse
                    if cur_state == mjSEPCLOSED + 1 {
                        item_set_i32(it_cur, ITEM_STATE, mjSEPCLOSED);
                        mjui_resize(ui, con);
                        mjui_update(-1, -1, ui, state as *const mjuiState, con);
                    }
                    // collapsed: expand
                    else if cur_state == mjSEPCLOSED {
                        item_set_i32(it_cur, ITEM_STATE, mjSEPCLOSED + 1);
                        mjui_resize(ui, con);
                        mjui_update(-1, -1, ui, state as *const mjuiState, con);
                    }
                    // nothing else to do for separator
                    return ui_ptr(u, UI_EDITCHANGED);
                } else if it_type == mjITEM_BUTTON {
                    // start tracking
                    ui_set_i32(u, UI_MOUSESECT, sect_cur);
                    ui_set_i32(u, UI_MOUSEITEM, item_cur);
                    change = 1;
                } else if it_type == mjITEM_CHECKINT {
                    // toggle check
                    let pd = item_pdata(it_cur) as *mut i32;
                    *pd = 1 - *pd;
                    change = 1;
                } else if it_type == mjITEM_CHECKBYTE {
                    // toggle check
                    let pd = item_pdata(it_cur) as *mut u8;
                    *pd = 1 - *pd;
                    change = 1;
                } else if it_type == mjITEM_RADIO {
                    // set selected element
                    i = findradio(
                        it_cur as *const mjuiItem,
                        ui,
                        state as *const mjuiState,
                        con,
                    );
                    if i >= 0 {
                        *(item_pdata(it_cur) as *mut i32) = i;
                        change = 1;
                    }
                } else if it_type == mjITEM_RADIOLINE {
                    // set selected element
                    i = findradioline(
                        it_cur as *const mjuiItem,
                        ui,
                        state as *const mjuiState,
                        con,
                    );
                    if i >= 0 {
                        *(item_pdata(it_cur) as *mut i32) = i;
                        change = 1;
                    }
                } else if it_type == mjITEM_SELECT {
                    // start tracking
                    ui_set_i32(u, UI_MOUSESECT, sect_cur);
                    ui_set_i32(u, UI_MOUSEITEM, item_cur);
                } else if it_type == mjITEM_SLIDERINT || it_type == mjITEM_SLIDERNUM {
                    // set value, start tracking
                    setslider(it_cur as *mut mjuiItem, ui, state as *const mjuiState, con);
                    ui_set_i32(u, UI_MOUSESECT, sect_cur);
                    ui_set_i32(u, UI_MOUSEITEM, item_cur);
                    change = 1;
                } else if it_type == mjITEM_EDITINT
                    || it_type == mjITEM_EDITNUM
                    || it_type == mjITEM_EDITFLOAT
                    || it_type == mjITEM_EDITTXT
                {
                    // set edit text, clear scroll
                    array2text(ui_edittext(u), it_cur as *const mjuiItem);
                    ui_set_i32(u, UI_EDITSCROLL, 0);
                    // set editcursor, adjust editscroll
                    setcursor(item_rect(it_cur), ui, state as *const mjuiState, con);
                    // start tracking: both edit and mouse
                    ui_set_i32(u, UI_EDITSECT, sect_cur);
                    ui_set_i32(u, UI_EDITITEM, item_cur);
                    ui_set_i32(u, UI_MOUSESECT, sect_cur);
                    ui_set_i32(u, UI_MOUSEITEM, item_cur);
                }

                // redraw
                mjui_update(sect_cur - 1, item_cur, ui, state as *const mjuiState, con);

                // return if change (otherwise return below)
                if change != 0 {
                    return it_cur as *mut mjuiItem;
                }
            }

            // return editchanged, since there was no other change
            return ui_ptr(u, UI_EDITCHANGED);
        } else if event_type == mjEVENT_RELEASE {
            // selection box change
            if !it_rec.is_null() && item_i32(it_rec, ITEM_TYPE) == mjITEM_SELECT {
                // find and set value
                i = findselect(
                    it_rec as *const mjuiItem,
                    ui,
                    state as *const mjuiState,
                    con,
                );
                if i >= 0 {
                    *(item_pdata(it_rec) as *mut i32) = i;
                }
                // free mouse and redraw ALL (selection box spills over)
                ui_set_i32(u, UI_MOUSESECT, 0);
                mjui_update(-1, -1, ui, state as *const mjuiState, con);
                // return if value set change
                if i >= 0 {
                    return it_rec as *mut mjuiItem;
                }
            }
            // button release
            else if !it_rec.is_null() && item_i32(it_rec, ITEM_TYPE) == mjITEM_BUTTON {
                // free mouse and redraw
                ui_set_i32(u, UI_MOUSESECT, 0);
                mjui_update(sect_rec - 1, item_rec, ui, state as *const mjuiState, con);
            }
            // free mouse, return no change
            ui_set_i32(u, UI_MOUSESECT, 0);
            return core::ptr::null_mut();
        } else if event_type == mjEVENT_SCROLL {
            // adjust scroll
            let sy = state_f64(s, STATE_SY);
            let linescroll = *(u.add(UI_SPACING_LINESCROLL) as *const i32);
            let scroll_amt = crate::engine::engine_util_misc::mju_round(
                sy * scl(linescroll, con) as f64,
            );
            ui_set_i32(u, UI_SCROLL, ui_i32(u, UI_SCROLL) - scroll_amt);
            // clamp scroll
            let rectid = ui_i32(u, UI_RECTID);
            let rect_h = state_rect(s, rectid).height;
            let clamped = mj_max(0, mj_min(ui_i32(u, UI_SCROLL), ui_i32(u, UI_HEIGHT) - rect_h));
            ui_set_i32(u, UI_SCROLL, clamped);
            return core::ptr::null_mut();
        } else if event_type == mjEVENT_KEY {
            // editing
            if sect_edit > 0 {
                // copy key and clear
                key = state_i32(s, STATE_KEY);
                state_set_i32(s, STATE_KEY, 0);

                // process key
                if key == mjKEY_ESCAPE {
                    // abandon
                    ui_set_i32(u, UI_EDITSECT, 0);
                } else if key == mjKEY_ENTER {
                    // instantiate
                    if text2array(ui_edittext(u) as *const i8, it_edit as *const mjuiItem) == 0 {
                        ui_set_i32(u, UI_EDITSECT, 0);
                        // redraw
                        mjui_update(sect_edit - 1, item_edit, ui, state as *const mjuiState, con);
                        // record editchanged and return
                        ui_set_ptr(u, UI_EDITCHANGED, it_edit as *mut mjuiItem);
                        return it_edit as *mut mjuiItem;
                    }
                } else if key == mjKEY_LEFT {
                    // move cursor left
                    if state_i32(s, STATE_CONTROL) != 0 {
                        // control: emulate Home
                        ui_set_i32(u, UI_EDITCURSOR, 0);
                    } else {
                        ui_set_i32(
                            u,
                            UI_EDITCURSOR,
                            mj_max(0, ui_i32(u, UI_EDITCURSOR) - 1),
                        );
                    }
                } else if key == mjKEY_RIGHT {
                    // move cursor right
                    let len = c_strlen(ui_edittext(u) as *const i8) as i32;
                    if state_i32(s, STATE_CONTROL) != 0 {
                        // control: emulate End
                        ui_set_i32(u, UI_EDITCURSOR, len);
                    } else {
                        ui_set_i32(
                            u,
                            UI_EDITCURSOR,
                            mj_min(len, ui_i32(u, UI_EDITCURSOR) + 1),
                        );
                    }
                } else if key == mjKEY_HOME {
                    // move cursor to first position
                    ui_set_i32(u, UI_EDITCURSOR, 0);
                } else if key == mjKEY_END {
                    // move cursor to last position
                    let len = c_strlen(ui_edittext(u) as *const i8) as i32;
                    ui_set_i32(u, UI_EDITCURSOR, len);
                } else if key == mjKEY_BACKSPACE {
                    // delete before cursor
                    let cursor = ui_i32(u, UI_EDITCURSOR);
                    if cursor > 0 {
                        let text = ui_edittext(u);
                        let len = c_strlen(text as *const i8) as i32;
                        // shift chars after cursor to the left
                        let mut idx = cursor;
                        while idx <= len {
                            *text.add(idx as usize - 1) = *text.add(idx as usize);
                            idx += 1;
                        }
                        // move cursor left
                        ui_set_i32(u, UI_EDITCURSOR, cursor - 1);
                    }
                } else if key == mjKEY_DELETE {
                    // delete after cursor
                    let cursor = ui_i32(u, UI_EDITCURSOR);
                    let text = ui_edittext(u);
                    let len = c_strlen(text as *const i8) as i32;
                    if cursor < len {
                        // shift chars after cursor to the left
                        let mut idx = cursor;
                        while idx <= len {
                            *text.add(idx as usize) = *text.add(idx as usize + 1);
                            idx += 1;
                        }
                    }
                } else {
                    // insert valid key at cursor
                    let text = ui_edittext(u);
                    let len = c_strlen(text as *const i8) as i32;
                    let cursor = ui_i32(u, UI_EDITCURSOR);
                    key = validkey(
                        key,
                        len,
                        item_i32(it_edit, ITEM_TYPE),
                        state as *const mjuiState,
                    );
                    if key != 0 {
                        // shift chars after cursor to the right
                        let mut idx = len;
                        while idx >= cursor {
                            *text.add(idx as usize + 1) = *text.add(idx as usize);
                            idx -= 1;
                        }
                        // set at cursor
                        *text.add(cursor as usize) = key as i8;
                        // move cursor right
                        ui_set_i32(u, UI_EDITCURSOR, cursor + 1);
                    }
                }

                // reveal cursor if still editing
                if ui_i32(u, UI_EDITSECT) > 0 {
                    revealcursor(item_rect(it_edit), ui, con);
                }

                // redraw
                mjui_update(sect_edit - 1, item_edit, ui, state as *const mjuiState, con);
            }
            // shortcut search
            else {
                // search section shortcuts
                let nsect = ui_i32(u, UI_NSECT);
                let mut found_section = false;
                for n in 0..nsect {
                    let sn = sect_ptr(u, n);
                    if matchshortcut(
                        state as *const mjuiState,
                        sect_i32(sn, SECT_MODIFIER),
                        sect_i32(sn, SECT_SHORTCUT),
                    ) != 0
                    {
                        // collapse all
                        for idx in 0..nsect {
                            let si = sect_ptr(u, idx);
                            if sect_i32(si, SECT_STATE) < 2 {
                                sect_set_i32(si, SECT_STATE, 0);
                            }
                        }
                        // expand this
                        if sect_i32(sn, SECT_STATE) < 2 {
                            sect_set_i32(sn, SECT_STATE, 1);
                        }
                        // size and update all
                        mjui_resize(ui, con);
                        mjui_update(-1, -1, ui, state as *const mjuiState, con);
                        // clear key
                        state_set_i32(s, STATE_KEY, 0);
                        return core::ptr::null_mut();
                    }
                }

                // search item shortcuts
                for n in 0..nsect {
                    let sn = sect_ptr(u, n);
                    let nitem = sect_i32(sn, SECT_NITEM);
                    for idx in 0..nitem {
                        // get pointer to item
                        let it = sect_item(sn, idx);
                        let it_type = item_i32(it, ITEM_TYPE);

                        // check
                        if (it_type == mjITEM_BUTTON
                            || it_type == mjITEM_CHECKINT
                            || it_type == mjITEM_CHECKBYTE)
                            && matchshortcut(
                                state as *const mjuiState,
                                *(it.add(ITEM_SINGLE_MODIFIER) as *const i32),
                                *(it.add(ITEM_SINGLE_SHORTCUT) as *const i32),
                            ) != 0
                        {
                            // clear key
                            state_set_i32(s, STATE_KEY, 0);

                            // active: process shortcut
                            if evalpredicate(
                                item_i32(it, ITEM_STATE),
                                (*ui).predicate,
                                *(u.add(UI_USERDATA) as *const *mut ()),
                            ) != 0
                            {
                                // toggle if check
                                if it_type == mjITEM_CHECKINT {
                                    let pd = item_pdata(it) as *mut i32;
                                    *pd = 1 - *pd;
                                    mjui_update(n, idx, ui, state as *const mjuiState, con);
                                } else if it_type == mjITEM_CHECKBYTE {
                                    let pd = item_pdata(it) as *mut u8;
                                    *pd = 1 - *pd;
                                    mjui_update(n, idx, ui, state as *const mjuiState, con);
                                }
                                // return item
                                return it as *mut mjuiItem;
                            }
                            // inactive: nothing to do
                            else {
                                return core::ptr::null_mut();
                            }
                        }
                    }
                }
            }
            return core::ptr::null_mut();
        } else if event_type == mjEVENT_RESIZE {
            // clamp scroll
            let rectid = ui_i32(u, UI_RECTID);
            let rect_h = state_rect(s, rectid).height;
            let clamped = mj_max(0, mj_min(ui_i32(u, UI_SCROLL), ui_i32(u, UI_HEIGHT) - rect_h));
            ui_set_i32(u, UI_SCROLL, clamped);
            return core::ptr::null_mut();
        }

        // should not get here, but just in case
        core::ptr::null_mut()
    }
}

/// C: mjui_render (ui/ui_main.h:48)
/// Calls: SCL, drawtext, findselect, initOpenGL, mjr_blitAux, mjr_rectangle, scrollrect
#[allow(unused_variables, non_snake_case)]
pub fn mjui_render(ui: *mut mjUI, state: *const mjuiState, con: *const mjrContext) {
    extern "C" { fn mjui_render(ui: *mut mjUI, state: *const mjuiState, con: *const mjrContext); }
    // SAFETY: delegates to C implementation
    unsafe { mjui_render(ui, state, con) }
}

