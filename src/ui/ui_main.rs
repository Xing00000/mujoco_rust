//! Port of: ui/ui_main.c
//! IR hash: d3ac8715281cd691
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: SCL (ui/ui_main.c:200)
/// Calls: mju_round
#[allow(unused_variables, non_snake_case)]
pub fn scl(sz: i32, con: *const mjrContext) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (sz : i32, con : * const mjrContext)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: initOpenGL (ui/ui_main.c:207)
#[allow(unused_variables, non_snake_case)]
pub fn init_open_gl(r: *const mjrRect, con: *const mjrContext) {
    // NOTE: signature changed from previous IR version
    // Previous params: (r : * const mjrRect, con : * const mjrContext)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: drawtext (ui/ui_main.c:251)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn drawtext(txt: *const i8, x: i32, y: i32, maxwidth: i32, rgb: *const f32, con: *const mjrContext) {
    // NOTE: signature changed from previous IR version
    // Previous params: (txt : * const i8, x : i32, y : i32, maxwidth : i32, rgb : * const f32, con : * const mjrContext)
    // Previous return: ()
    todo!("re-translate: params renamed")
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
    // NOTE: signature changed from previous IR version
    // Previous params: (rect : mjrRect, txt : * const i8, rgb : * const f32, con : * const mjrContext)
    // Previous return: ()
    todo!("re-translate: params renamed")
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
    // NOTE: signature changed from previous IR version
    // Previous params: (rect : mjrRect, rgb : * const f32, rgbback : * const f32, con : * const mjrContext)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: roundcorner (ui/ui_main.c:313)
#[allow(unused_variables, non_snake_case)]
pub fn roundcorner(rect: mjrRect, flg_skipbottom: i32, flg_separator: i32, ui: *const mjUI, con: *const mjrContext) {
    // NOTE: signature changed from previous IR version
    // Previous params: (rect : mjrRect, flg_skipbottom : i32, flg_separator : i32, ui : * const mjUI, con : * const mjrContext)
    // Previous return: ()
    todo!("re-translate: params renamed")
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
    // NOTE: signature changed from previous IR version
    // Previous params: (rect : mjrRect, rgb : * const f32, rgbback : * const f32, con : * const mjrContext)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: drawsymbol (ui/ui_main.c:434)
/// Calls: SCL, mju_round
#[allow(unused_variables, non_snake_case)]
pub fn drawsymbol(rect: mjrRect, flg_open: i32, r#type: i32, ui: *const mjUI, con: *const mjrContext) {
    // NOTE: signature changed from previous IR version
    // Previous params: (rect : mjrRect, flg_open : i32, r#type : i32, ui : * const mjUI, con : * const mjrContext)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: radioelement (ui/ui_main.c:516)
/// Calls: SCL
#[allow(unused_variables, non_snake_case)]
pub fn radioelement(it: *const mjuiItem, n: i32, ui: *const mjUI, con: *const mjrContext) -> mjrRect {
    // NOTE: signature changed from previous IR version
    // Previous params: (it : * const mjuiItem, n : i32, ui : * const mjUI, con : * const mjrContext)
    // Previous return: mjrRect
    todo!("re-translate: params renamed")
}

/// C: mouseinui (ui/ui_main.c:549)
#[allow(unused_variables, non_snake_case)]
pub fn mouseinui(ui: *const mjUI, ins: *const mjuiState, x: *mut i32, y: *mut i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (ui : * const mjUI, ins : * const mjuiState, x : * mut i32, y : * mut i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
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
    // NOTE: signature changed from previous IR version
    // Previous params: (rect : mjrRect, ui : * const mjUI, ins : * const mjuiState, rx : * mut f64, ry : * mut f64)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: findradio (ui/ui_main.c:582)
/// Calls: mouseinrect
#[allow(unused_variables, non_snake_case)]
pub fn findradio(it: *const mjuiItem, ui: *const mjUI, ins: *const mjuiState, con: *const mjrContext) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (it : * const mjuiItem, ui : * const mjUI, ins : * const mjuiState, con : * const mjrContext)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: makeradioline (ui/ui_main.c:613)
/// Calls: mju_round, textwidth
#[allow(unused_variables, non_snake_case)]
pub fn makeradioline(it: *const mjuiItem, con: *const mjrContext, sep: *mut i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (it : * const mjuiItem, con : * const mjrContext, sep : * mut i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: findradioline (ui/ui_main.c:642)
/// Calls: makeradioline, mju_round, mouseinrect
#[allow(unused_variables, non_snake_case)]
pub fn findradioline(it: *const mjuiItem, ui: *const mjUI, ins: *const mjuiState, con: *const mjrContext) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (it : * const mjuiItem, ui : * const mjUI, ins : * const mjuiState, con : * const mjrContext)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: findselect (ui/ui_main.c:667)
/// Calls: SCL, mouseinrect
#[allow(unused_variables, non_snake_case)]
pub fn findselect(it: *const mjuiItem, ui: *const mjUI, ins: *const mjuiState, con: *const mjrContext) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (it : * const mjuiItem, ui : * const mjUI, ins : * const mjuiState, con : * const mjrContext)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: scrollrect (ui/ui_main.c:696)
/// Calls: SCL, mju_round
#[allow(unused_variables, non_snake_case)]
pub fn scrollrect(rect: mjrRect, ui: *const mjUI, con: *const mjrContext, bar: *mut mjrRect, thumb: *mut mjrRect) {
    // NOTE: signature changed from previous IR version
    // Previous params: (rect : mjrRect, ui : * const mjUI, con : * const mjrContext, bar : * mut mjrRect, thumb : * mut mjrRect)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: inside (ui/ui_main.c:716)
#[allow(unused_variables, non_snake_case)]
pub fn inside(x: i32, y: i32, r: mjrRect) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (x : i32, y : i32, r : mjrRect)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: insideoval (ui/ui_main.c:723)
/// Calls: inside
#[allow(unused_variables, non_snake_case)]
pub fn insideoval(x: i32, y: i32, r: mjrRect) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (x : i32, y : i32, r : mjrRect)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: findmouse (ui/ui_main.c:757)
/// Calls: inside, insideoval, mouseinui, scrollrect
#[allow(unused_variables, non_snake_case)]
pub fn findmouse(ui: *const mjUI, ins: *const mjuiState, con: *const mjrContext, sect: *mut i32, item: *mut i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (ui : * const mjUI, ins : * const mjuiState, con : * const mjrContext, sect : * mut i32, item : * mut i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: setslider (ui/ui_main.c:841)
/// Calls: mju_clip, mju_round, mouseinrect
#[allow(unused_variables, non_snake_case)]
pub fn setslider(it: *mut mjuiItem, ui: *mut mjUI, ins: *const mjuiState, con: *const mjrContext) {
    // NOTE: signature changed from previous IR version
    // Previous params: (it : * mut mjuiItem, ui : * mut mjUI, ins : * const mjuiState, con : * const mjrContext)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: checkedit (ui/ui_main.c:868)
/// Calls: mju_error
#[allow(unused_variables, non_snake_case)]
pub fn checkedit(text: *const i8, it: *const mjuiItem) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (text : * const i8, it : * const mjuiItem)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: text2array (ui/ui_main.c:914)
/// Calls: mju_error
#[allow(unused_variables, non_snake_case)]
pub fn text2array(text: *const i8, it: *const mjuiItem) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (text : * const i8, it : * const mjuiItem)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: array2text (ui/ui_main.c:982)
/// Calls: mju_error
#[allow(unused_variables, non_snake_case)]
pub fn array2text(text: *mut i8, it: *const mjuiItem) {
    // NOTE: signature changed from previous IR version
    // Previous params: (text : * mut i8, it : * const mjuiItem)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: validkey (ui/ui_main.c:1017)
#[allow(unused_variables, non_snake_case)]
pub fn validkey(key: i32, sz: i32, r#type: i32, state: *const mjuiState) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (key : i32, sz : i32, r#type : i32, state : * const mjuiState)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: revealcursor (ui/ui_main.c:1098)
/// Calls: SCL
#[allow(unused_variables, non_snake_case)]
pub fn revealcursor(r: mjrRect, ui: *mut mjUI, con: *const mjrContext) {
    // NOTE: signature changed from previous IR version
    // Previous params: (r : mjrRect, ui : * mut mjUI, con : * const mjrContext)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: setcursor (ui/ui_main.c:1124)
/// Calls: SCL, mju_round, mouseinrect, revealcursor
#[allow(unused_variables, non_snake_case)]
pub fn setcursor(r: mjrRect, ui: *mut mjUI, ins: *const mjuiState, con: *const mjrContext) {
    // NOTE: signature changed from previous IR version
    // Previous params: (r : mjrRect, ui : * mut mjUI, ins : * const mjuiState, con : * const mjrContext)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: parseshortcut (ui/ui_main.c:1169)
/// Calls: mju_error
#[allow(unused_variables, non_snake_case)]
pub fn parseshortcut(text: *const i8, r#mod: *mut i32, key: *mut i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (text : * const i8, r#mod : * mut i32, key : * mut i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: matchshortcut (ui/ui_main.c:1222)
#[allow(unused_variables, non_snake_case)]
pub fn matchshortcut(ins: *const mjuiState, r#mod: i32, key: i32) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (ins : * const mjuiState, r#mod : i32, key : i32)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: setitemskip (ui/ui_main.c:1500)
#[allow(unused_variables, non_snake_case)]
pub fn setitemskip(s: *mut mjuiSection, pass: i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (s : * mut mjuiSection, pass : i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: tryresize (ui/ui_main.c:1528)
/// Calls: SCL, setitemskip
#[allow(unused_variables, non_snake_case)]
pub fn tryresize(ui: *mut mjUI, con: *const mjrContext) {
    // NOTE: signature changed from previous IR version
    // Previous params: (ui : * mut mjUI, con : * const mjrContext)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: insertionsortgroup (ui/ui_main.c:1717)
/// Calls: mju_error
#[allow(unused_variables, non_snake_case)]
pub fn insertionsortgroup(list: *mut i32, num: i32, stride: i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (list : * mut i32, num : i32, stride : i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: evalpredicate (ui/ui_main.c:1823)
#[allow(unused_variables, non_snake_case)]
pub fn evalpredicate(state: i32, predicate: mjfItemEnable, userdata: *mut ()) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (state : i32, predicate : mjfItemEnable, userdata : * mut ())
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: shortcuthelp (ui/ui_main.c:1836)
/// Calls: SCL, drawrectangle, drawtext, mju_strncpy, textwidth
#[allow(unused_variables, non_snake_case)]
pub fn shortcuthelp(r: mjrRect, modifier: i32, shortcut: i32, ui: *const mjUI, con: *const mjrContext) {
    // NOTE: signature changed from previous IR version
    // Previous params: (r : mjrRect, modifier : i32, shortcut : i32, ui : * const mjUI, con : * const mjrContext)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjui_themeSpacing (ui/ui_main.h:26)
#[allow(unused_variables, non_snake_case)]
pub fn mjui_theme_spacing(ind: i32) -> mjuiThemeSpacing {
    // NOTE: signature changed from previous IR version
    // Previous params: (ind : i32)
    // Previous return: mjuiThemeSpacing
    todo!("re-translate: params renamed")
}

/// C: mjui_themeColor (ui/ui_main.h:29)
#[allow(unused_variables, non_snake_case)]
pub fn mjui_theme_color(ind: i32) -> mjuiThemeColor {
    // NOTE: signature changed from previous IR version
    // Previous params: (ind : i32)
    // Previous return: mjuiThemeColor
    todo!("re-translate: params renamed")
}

/// C: mjui_add (ui/ui_main.h:32)
/// Calls: mju_error, mju_round, mju_strncpy, parseshortcut
#[allow(unused_variables, non_snake_case)]
pub fn mjui_add(ui: *mut mjUI, def: *const mjuiDef) {
    // NOTE: signature changed from previous IR version
    // Previous params: (ui : * mut mjUI, def : * const mjuiDef)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjui_addToSection (ui/ui_main.h:35)
/// Calls: mjui_add
#[allow(unused_variables, non_snake_case)]
pub fn mjui_add_to_section(ui: *mut mjUI, sect: i32, def: *const mjuiDef) {
    // NOTE: signature changed from previous IR version
    // Previous params: (ui : * mut mjUI, sect : i32, def : * const mjuiDef)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjui_resize (ui/ui_main.h:38)
/// Calls: SCL, insertionsortgroup, mju_error, tryresize
#[allow(unused_variables, non_snake_case)]
pub fn mjui_resize(ui: *mut mjUI, con: *const mjrContext) {
    // NOTE: signature changed from previous IR version
    // Previous params: (ui : * mut mjUI, con : * const mjrContext)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjui_update (ui/ui_main.h:41)
/// Calls: SCL, array2text, checkedit, drawoval, drawrectangle, drawsymbol, drawtext, drawtextrect, evalpredicate, findmouse, initOpenGL, makeradioline, mjr_restoreBuffer, mjr_setAux, mju_error, mju_round, radioelement, roundcorner, shortcuthelp, textwidth
#[allow(unused_variables, non_snake_case)]
pub fn mjui_update(section: i32, item: i32, ui: *const mjUI, state: *const mjuiState, con: *const mjrContext) {
    // NOTE: signature changed from previous IR version
    // Previous params: (section : i32, item : i32, ui : * const mjUI, state : * const mjuiState, con : * const mjrContext)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjui_event (ui/ui_main.h:45)
/// Calls: SCL, array2text, evalpredicate, findmouse, findradio, findradioline, findselect, matchshortcut, mju_round, mjui_resize, mjui_update, revealcursor, setcursor, setslider, text2array, validkey
#[allow(unused_variables, non_snake_case)]
pub fn mjui_event(ui: *mut mjUI, state: *mut mjuiState, con: *const mjrContext) -> *mut mjuiItem {
    // NOTE: signature changed from previous IR version
    // Previous params: (ui : * mut mjUI, state : * mut mjuiState, con : * const mjrContext)
    // Previous return: * mut mjuiItem
    todo!("re-translate: params renamed")
}

/// C: mjui_render (ui/ui_main.h:48)
/// Calls: SCL, drawtext, findselect, initOpenGL, mjr_blitAux, mjr_rectangle, scrollrect
#[allow(unused_variables, non_snake_case)]
pub fn mjui_render(ui: *mut mjUI, state: *const mjuiState, con: *const mjrContext) {
    // NOTE: signature changed from previous IR version
    // Previous params: (ui : * mut mjUI, state : * const mjuiState, con : * const mjrContext)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

