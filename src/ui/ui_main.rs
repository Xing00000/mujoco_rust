//! Port of: ui/ui_main.c
//! IR hash: 05737965add36adb
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: SCL (ui/ui_main.c:200)
/// Calls: mju_round
#[allow(unused_variables, non_snake_case)]
pub fn scl(sz: i32, con: *const mjrContext) -> i32 {
    // SAFETY: caller guarantees con is a valid pointer to mjrContext
    unsafe {
        let val = crate::engine::engine_util_misc::mju_round(sz as f64 * 0.01 * (*con).fontScale as f64);
        if val > 0 { val } else { 0 }
    }
}

/// C: initOpenGL (ui/ui_main.c:207)
#[allow(unused_variables, non_snake_case)]
pub fn init_open_gl(r: *const mjrRect, con: *const mjrContext) {
    extern "C" { fn initOpenGL_impl(r: *const mjrRect, con: *const mjrContext); }
    // SAFETY: delegates to C implementation, pointers valid per caller contract
    unsafe { initOpenGL_impl(r, con) }
}

/// C: drawtext (ui/ui_main.c:251)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn drawtext(txt: *const i8, x: i32, y: i32, maxwidth: i32, rgb: *const f32, con: *const mjrContext) {
    extern "C" { fn drawtext_impl(txt: *const i8, x: i32, y: i32, maxwidth: i32, rgb: *const f32, con: *const mjrContext); }
    // SAFETY: delegates to C implementation; caller guarantees all pointers are valid
    unsafe { drawtext_impl(txt, x, y, maxwidth, rgb, con) }
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
    extern "C" { fn drawtextrect_impl(rect: mjrRect, txt: *const i8, rgb: *const f32, con: *const mjrContext); }
    // SAFETY: delegates to C implementation
    unsafe { drawtextrect_impl(rect, txt, rgb, con) }
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
    extern "C" { fn drawrectangle_impl(rect: mjrRect, rgb: *const f32, rgbback: *const f32, con: *const mjrContext); }
    // SAFETY: delegates to C implementation
    unsafe { drawrectangle_impl(rect, rgb, rgbback, con) }
}

/// C: roundcorner (ui/ui_main.c:313)
#[allow(unused_variables, non_snake_case)]
pub fn roundcorner(rect: mjrRect, flg_skipbottom: i32, flg_separator: i32, ui: *const mjUI, con: *const mjrContext) {
    extern "C" { fn roundcorner_impl(rect: mjrRect, flg_skipbottom: i32, flg_separator: i32, ui: *const mjUI, con: *const mjrContext); }
    // SAFETY: delegates to C implementation
    unsafe { roundcorner_impl(rect, flg_skipbottom, flg_separator, ui, con) }
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
    extern "C" { fn drawoval_impl(rect: mjrRect, rgb: *const f32, rgbback: *const f32, con: *const mjrContext); }
    // SAFETY: delegates to C implementation
    unsafe { drawoval_impl(rect, rgb, rgbback, con) }
}

/// C: drawsymbol (ui/ui_main.c:434)
/// Calls: SCL, mju_round
#[allow(unused_variables, non_snake_case)]
pub fn drawsymbol(rect: mjrRect, flg_open: i32, r#type: i32, ui: *const mjUI, con: *const mjrContext) {
    extern "C" { fn drawsymbol_impl(rect: mjrRect, flg_open: i32, r#type: i32, ui: *const mjUI, con: *const mjrContext); }
    // SAFETY: delegates to C implementation
    unsafe { drawsymbol_impl(rect, flg_open, r#type, ui, con) }
}

/// C: radioelement (ui/ui_main.c:516)
/// Calls: SCL
#[allow(unused_variables, non_snake_case)]
pub fn radioelement(it: *const mjuiItem, n: i32, ui: *const mjUI, con: *const mjrContext) -> mjrRect {
    extern "C" { fn radioelement_impl(it: *const mjuiItem, n: i32, ui: *const mjUI, con: *const mjrContext) -> mjrRect; }
    // SAFETY: delegates to C implementation
    unsafe { radioelement_impl(it, n, ui, con) }
}

/// C: mouseinui (ui/ui_main.c:549)
#[allow(unused_variables, non_snake_case)]
pub fn mouseinui(ui: *const mjUI, ins: *const mjuiState, x: *mut i32, y: *mut i32) {
    extern "C" { fn mouseinui_impl(ui: *const mjUI, ins: *const mjuiState, x: *mut i32, y: *mut i32); }
    // SAFETY: delegates to C implementation
    unsafe { mouseinui_impl(ui, ins, x, y) }
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
    extern "C" { fn mouseinrect_impl(rect: mjrRect, ui: *const mjUI, ins: *const mjuiState, rx: *mut f64, ry: *mut f64); }
    // SAFETY: delegates to C implementation
    unsafe { mouseinrect_impl(rect, ui, ins, rx, ry) }
}

/// C: findradio (ui/ui_main.c:582)
/// Calls: mouseinrect
#[allow(unused_variables, non_snake_case)]
pub fn findradio(it: *const mjuiItem, ui: *const mjUI, ins: *const mjuiState, con: *const mjrContext) -> i32 {
    extern "C" { fn findradio_impl(it: *const mjuiItem, ui: *const mjUI, ins: *const mjuiState, con: *const mjrContext) -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { findradio_impl(it, ui, ins, con) }
}

/// C: makeradioline (ui/ui_main.c:613)
/// Calls: mju_round, textwidth
#[allow(unused_variables, non_snake_case)]
pub fn makeradioline(it: *const mjuiItem, con: *const mjrContext, sep: *mut i32) {
    extern "C" { fn makeradioline_impl(it: *const mjuiItem, con: *const mjrContext, sep: *mut i32); }
    // SAFETY: delegates to C implementation
    unsafe { makeradioline_impl(it, con, sep) }
}

/// C: findradioline (ui/ui_main.c:642)
/// Calls: makeradioline, mju_round, mouseinrect
#[allow(unused_variables, non_snake_case)]
pub fn findradioline(it: *const mjuiItem, ui: *const mjUI, ins: *const mjuiState, con: *const mjrContext) -> i32 {
    extern "C" { fn findradioline_impl(it: *const mjuiItem, ui: *const mjUI, ins: *const mjuiState, con: *const mjrContext) -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { findradioline_impl(it, ui, ins, con) }
}

/// C: findselect (ui/ui_main.c:667)
/// Calls: SCL, mouseinrect
#[allow(unused_variables, non_snake_case)]
pub fn findselect(it: *const mjuiItem, ui: *const mjUI, ins: *const mjuiState, con: *const mjrContext) -> i32 {
    extern "C" { fn findselect_impl(it: *const mjuiItem, ui: *const mjUI, ins: *const mjuiState, con: *const mjrContext) -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { findselect_impl(it, ui, ins, con) }
}

/// C: scrollrect (ui/ui_main.c:696)
/// Calls: SCL, mju_round
#[allow(unused_variables, non_snake_case)]
pub fn scrollrect(rect: mjrRect, ui: *const mjUI, con: *const mjrContext, bar: *mut mjrRect, thumb: *mut mjrRect) {
    extern "C" { fn scrollrect_impl(rect: mjrRect, ui: *const mjUI, con: *const mjrContext, bar: *mut mjrRect, thumb: *mut mjrRect); }
    // SAFETY: delegates to C implementation, pointers valid per caller contract
    unsafe { scrollrect_impl(rect, ui, con, bar, thumb) }
}

/// C: inside (ui/ui_main.c:716)
#[allow(unused_variables, non_snake_case)]
pub fn inside(x: i32, y: i32, r: mjrRect) -> i32 {
    extern "C" { fn inside_impl(x: i32, y: i32, r: mjrRect) -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { inside_impl(x, y, r) }
}

/// C: insideoval (ui/ui_main.c:723)
/// Calls: inside
#[allow(unused_variables, non_snake_case)]
pub fn insideoval(x: i32, y: i32, r: mjrRect) -> i32 {
    extern "C" { fn insideoval_impl(x: i32, y: i32, r: mjrRect) -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { insideoval_impl(x, y, r) }
}

/// C: findmouse (ui/ui_main.c:757)
/// Calls: inside, insideoval, mouseinui, scrollrect
#[allow(unused_variables, non_snake_case)]
pub fn findmouse(ui: *const mjUI, ins: *const mjuiState, con: *const mjrContext, sect: *mut i32, item: *mut i32) {
    extern "C" { fn findmouse_impl(ui: *const mjUI, ins: *const mjuiState, con: *const mjrContext, sect: *mut i32, item: *mut i32); }
    // SAFETY: delegates to C implementation
    unsafe { findmouse_impl(ui, ins, con, sect, item) }
}

/// C: setslider (ui/ui_main.c:841)
/// Calls: mju_clip, mju_round, mouseinrect
#[allow(unused_variables, non_snake_case)]
pub fn setslider(it: *mut mjuiItem, ui: *mut mjUI, ins: *const mjuiState, con: *const mjrContext) {
    extern "C" { fn setslider_impl(it: *mut mjuiItem, ui: *mut mjUI, ins: *const mjuiState, con: *const mjrContext); }
    // SAFETY: delegates to C implementation
    unsafe { setslider_impl(it, ui, ins, con) }
}

/// C: checkedit (ui/ui_main.c:868)
/// Calls: mju_error
#[allow(unused_variables, non_snake_case)]
pub fn checkedit(text: *const i8, it: *const mjuiItem) -> i32 {
    extern "C" { fn checkedit_impl(text: *const i8, it: *const mjuiItem) -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { checkedit_impl(text, it) }
}

/// C: text2array (ui/ui_main.c:914)
/// Calls: mju_error
#[allow(unused_variables, non_snake_case)]
pub fn text2array(text: *const i8, it: *const mjuiItem) -> i32 {
    extern "C" { fn text2array_impl(text: *const i8, it: *const mjuiItem) -> i32; }
    // SAFETY: delegates to C implementation, pointers valid per caller contract
    unsafe { text2array_impl(text, it) }
}

/// C: array2text (ui/ui_main.c:982)
/// Calls: mju_error
#[allow(unused_variables, non_snake_case)]
pub fn array2text(text: *mut i8, it: *const mjuiItem) {
    extern "C" { fn array2text_impl(text: *mut i8, it: *const mjuiItem); }
    // SAFETY: delegates to C implementation, pointers valid per caller contract
    unsafe { array2text_impl(text, it) }
}

/// C: validkey (ui/ui_main.c:1017)
#[allow(unused_variables, non_snake_case)]
pub fn validkey(key: i32, sz: i32, r#type: i32, state: *const mjuiState) -> i32 {
    extern "C" { fn validkey_impl(key: i32, sz: i32, r#type: i32, state: *const mjuiState) -> i32; }
    // SAFETY: delegates to C implementation, pointers valid per caller contract
    unsafe { validkey_impl(key, sz, r#type, state) }
}

/// C: revealcursor (ui/ui_main.c:1098)
/// Calls: SCL
#[allow(unused_variables, non_snake_case)]
pub fn revealcursor(r: mjrRect, ui: *mut mjUI, con: *const mjrContext) {
    extern "C" { fn revealcursor_impl(r: mjrRect, ui: *mut mjUI, con: *const mjrContext); }
    // SAFETY: delegates to C implementation, pointers valid per caller contract
    unsafe { revealcursor_impl(r, ui, con) }
}

/// C: setcursor (ui/ui_main.c:1124)
/// Calls: SCL, mju_round, mouseinrect, revealcursor
#[allow(unused_variables, non_snake_case)]
pub fn setcursor(r: mjrRect, ui: *mut mjUI, ins: *const mjuiState, con: *const mjrContext) {
    extern "C" { fn setcursor_impl(r: mjrRect, ui: *mut mjUI, ins: *const mjuiState, con: *const mjrContext); }
    // SAFETY: delegates to C implementation
    unsafe { setcursor_impl(r, ui, ins, con) }
}

/// C: parseshortcut (ui/ui_main.c:1169)
/// Calls: mju_error
#[allow(unused_variables, non_snake_case)]
pub fn parseshortcut(text: *const i8, r#mod: *mut i32, key: *mut i32) {
    extern "C" { fn parseshortcut_impl(text: *const i8, r#mod: *mut i32, key: *mut i32); }
    // SAFETY: delegates to C implementation
    unsafe { parseshortcut_impl(text, r#mod, key) }
}

/// C: matchshortcut (ui/ui_main.c:1222)
#[allow(unused_variables, non_snake_case)]
pub fn matchshortcut(ins: *const mjuiState, r#mod: i32, key: i32) -> i32 {
    extern "C" { fn matchshortcut_impl(ins: *const mjuiState, r#mod: i32, key: i32) -> i32; }
    // SAFETY: delegates to C implementation, pointers valid per caller contract
    unsafe { matchshortcut_impl(ins, r#mod, key) }
}

/// C: setitemskip (ui/ui_main.c:1500)
#[allow(unused_variables, non_snake_case)]
pub fn setitemskip(s: *mut mjuiSection, pass: i32) {
    extern "C" { fn setitemskip_impl(s: *mut mjuiSection, pass: i32); }
    // SAFETY: delegates to C implementation
    unsafe { setitemskip_impl(s, pass) }
}

/// C: tryresize (ui/ui_main.c:1528)
/// Calls: SCL, setitemskip
#[allow(unused_variables, non_snake_case)]
pub fn tryresize(ui: *mut mjUI, con: *const mjrContext) {
    extern "C" { fn tryresize_impl(ui: *mut mjUI, con: *const mjrContext); }
    // SAFETY: delegates to C implementation
    unsafe { tryresize_impl(ui, con) }
}

/// C: insertionsortgroup (ui/ui_main.c:1717)
/// Calls: mju_error
#[allow(unused_variables, non_snake_case)]
pub fn insertionsortgroup(list: *mut i32, num: i32, stride: i32) {
    extern "C" { fn insertionsortgroup_impl(list: *mut i32, num: i32, stride: i32); }
    // SAFETY: delegates to C implementation
    unsafe { insertionsortgroup_impl(list, num, stride) }
}

/// C: evalpredicate (ui/ui_main.c:1823)
#[allow(unused_variables, non_snake_case)]
pub fn evalpredicate(state: i32, predicate: mjfItemEnable, userdata: *mut ()) -> i32 {
    extern "C" { fn evalpredicate_impl(state: i32, predicate: mjfItemEnable, userdata: *mut ()) -> i32; }
    // SAFETY: delegates to C implementation, pointers valid per caller contract
    unsafe { evalpredicate_impl(state, predicate, userdata) }
}

/// C: shortcuthelp (ui/ui_main.c:1836)
/// Calls: SCL, drawrectangle, drawtext, mju_strncpy, textwidth
#[allow(unused_variables, non_snake_case)]
pub fn shortcuthelp(r: mjrRect, modifier: i32, shortcut: i32, ui: *const mjUI, con: *const mjrContext) {
    extern "C" { fn shortcuthelp_impl(r: mjrRect, modifier: i32, shortcut: i32, ui: *const mjUI, con: *const mjrContext); }
    // SAFETY: delegates to C implementation
    unsafe { shortcuthelp_impl(r, modifier, shortcut, ui, con) }
}

/// C: mjui_themeSpacing (ui/ui_main.h:26)
#[allow(unused_variables, non_snake_case)]
pub fn mjui_theme_spacing(ind: i32) -> mjuiThemeSpacing {
    extern "C" { fn mjui_themeSpacing_impl(ind: i32) -> mjuiThemeSpacing; }
    // SAFETY: delegates to C implementation
    unsafe { mjui_themeSpacing_impl(ind) }
}

/// C: mjui_themeColor (ui/ui_main.h:29)
#[allow(unused_variables, non_snake_case)]
pub fn mjui_theme_color(ind: i32) -> mjuiThemeColor {
    extern "C" { fn mjui_themeColor_impl(ind: i32) -> mjuiThemeColor; }
    // SAFETY: delegates to C implementation
    unsafe { mjui_themeColor_impl(ind) }
}

/// C: mjui_add (ui/ui_main.h:32)
/// Calls: mju_error, mju_round, mju_strncpy, parseshortcut
#[allow(unused_variables, non_snake_case)]
pub fn mjui_add(ui: *mut mjUI, def: *const mjuiDef) {
    extern "C" { fn mjui_add_impl(ui: *mut mjUI, def: *const mjuiDef); }
    // SAFETY: delegates to C implementation
    unsafe { mjui_add_impl(ui, def) }
}

/// C: mjui_addToSection (ui/ui_main.h:35)
/// Calls: mjui_add
#[allow(unused_variables, non_snake_case)]
pub fn mjui_add_to_section(ui: *mut mjUI, sect: i32, def: *const mjuiDef) {
    extern "C" { fn mjui_addToSection_impl(ui: *mut mjUI, sect: i32, def: *const mjuiDef); }
    // SAFETY: delegates to C implementation
    unsafe { mjui_addToSection_impl(ui, sect, def) }
}

/// C: mjui_resize (ui/ui_main.h:38)
/// Calls: SCL, insertionsortgroup, mju_error, tryresize
#[allow(unused_variables, non_snake_case)]
pub fn mjui_resize(ui: *mut mjUI, con: *const mjrContext) {
    extern "C" { fn mjui_resize_impl(ui: *mut mjUI, con: *const mjrContext); }
    // SAFETY: delegates to C implementation
    unsafe { mjui_resize_impl(ui, con) }
}

/// C: mjui_update (ui/ui_main.h:41)
/// Calls: SCL, array2text, checkedit, drawoval, drawrectangle, drawsymbol, drawtext, drawtextrect, evalpredicate, findmouse, initOpenGL, makeradioline, mjr_restoreBuffer, mjr_setAux, mju_error, mju_round, radioelement, roundcorner, shortcuthelp, textwidth
#[allow(unused_variables, non_snake_case)]
pub fn mjui_update(section: i32, item: i32, ui: *const mjUI, state: *const mjuiState, con: *const mjrContext) {
    // WARNING: signature changed — verify body
    // Previous params: (section : i32, item : i32, ui : * const mjUI, state : * const mjuiState, con : * const mjrContext)
    // Previous return: ()
    todo ! ()
}

/// C: mjui_event (ui/ui_main.h:45)
/// Calls: SCL, array2text, evalpredicate, findmouse, findradio, findradioline, findselect, matchshortcut, mju_round, mjui_resize, mjui_update, revealcursor, setcursor, setslider, text2array, validkey
#[allow(unused_variables, non_snake_case)]
pub fn mjui_event(ui: *mut mjUI, state: *mut mjuiState, con: *const mjrContext) -> *mut mjuiItem {
    // WARNING: signature changed — verify body
    // Previous params: (ui : * mut mjUI, state : * mut mjuiState, con : * const mjrContext)
    // Previous return: * mut mjuiItem
    todo ! ()
}

/// C: mjui_render (ui/ui_main.h:48)
/// Calls: SCL, drawtext, findselect, initOpenGL, mjr_blitAux, mjr_rectangle, scrollrect
#[allow(unused_variables, non_snake_case)]
pub fn mjui_render(ui: *mut mjUI, state: *const mjuiState, con: *const mjrContext) {
    // WARNING: signature changed — verify body
    // Previous params: (ui : * mut mjUI, state : * const mjuiState, con : * const mjrContext)
    // Previous return: ()
    todo ! ()
}

