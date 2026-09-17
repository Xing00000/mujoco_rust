//! Port of: Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/ui/ui_main.c
//! IR hash: 1fa44043fe383ce9
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: SCL (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/ui/ui_main.c:200)
/// Calls: cxx:_mju_round
#[allow(unused_variables, non_snake_case)]
pub fn scl(sz: i32, con: *const mjrContext) -> i32 {
    todo!() // cxx-internal:ui_main.c.o:_SCL
}

/// C: initOpenGL (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/ui/ui_main.c:207)
#[allow(unused_variables, non_snake_case)]
pub fn init_open_gl(r: *const mjrRect, con: *const mjrContext) {
    todo!() // cxx-internal:ui_main.c.o:_initOpenGL
}

/// C: drawtext (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/ui/ui_main.c:251)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn drawtext(txt: *const i8, x: i32, y: i32, maxwidth: i32, rgb: *const f32, con: *const mjrContext) {
    todo!() // cxx-internal:ui_main.c.o:_drawtext
}

/// C: drawtextrect (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/ui/ui_main.c:274)
/// Calls: cxx-internal:ui_main.c.o:_drawtext
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn drawtextrect(rect: mjrRect, txt: *const i8, rgb: *const f32, con: *const mjrContext) {
    todo!() // cxx-internal:ui_main.c.o:_drawtextrect
}

/// C: drawrectangle (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/ui/ui_main.c:286)
/// Calls: cxx-internal:ui_main.c.o:_SCL
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn drawrectangle(rect: mjrRect, rgb: *const f32, rgbback: *const f32, con: *const mjrContext) {
    todo!() // cxx-internal:ui_main.c.o:_drawrectangle
}

/// C: roundcorner (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/ui/ui_main.c:313)
#[allow(unused_variables, non_snake_case)]
pub fn roundcorner(rect: mjrRect, flg_skipbottom: i32, flg_separator: i32, ui: *const mjUI, con: *const mjrContext) {
    todo!() // cxx-internal:ui_main.c.o:_roundcorner
}

/// C: drawoval (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/ui/ui_main.c:375)
/// Calls: cxx-internal:ui_main.c.o:_SCL
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn drawoval(rect: mjrRect, rgb: *const f32, rgbback: *const f32, con: *const mjrContext) {
    todo!() // cxx-internal:ui_main.c.o:_drawoval
}

/// C: drawsymbol (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/ui/ui_main.c:434)
/// Calls: cxx-internal:ui_main.c.o:_SCL, cxx:_mju_round
#[allow(unused_variables, non_snake_case)]
pub fn drawsymbol(rect: mjrRect, flg_open: i32, r#type: i32, ui: *const mjUI, con: *const mjrContext) {
    todo!() // cxx-internal:ui_main.c.o:_drawsymbol
}

/// C: radioelement (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/ui/ui_main.c:516)
/// Calls: cxx-internal:ui_main.c.o:_SCL
#[allow(unused_variables, non_snake_case)]
pub fn radioelement(it: *const mjuiItem, n: i32, ui: *const mjUI, con: *const mjrContext) -> mjrRect {
    todo!() // cxx-internal:ui_main.c.o:_radioelement
}

/// C: mouseinui (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/ui/ui_main.c:549)
/// Calls: cxx:__ZN11mjCActuator3actERKNSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEE
#[allow(unused_variables, non_snake_case)]
pub fn mouseinui(ui: *const mjUI, ins: *const mjuiState, x: *mut i32, y: *mut i32) {
    todo!() // cxx-internal:ui_main.c.o:_mouseinui
}

/// C: mouseinrect (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/ui/ui_main.c:568)
/// Calls: cxx-internal:ui_main.c.o:_mouseinui
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mouseinrect(rect: mjrRect, ui: *const mjUI, ins: *const mjuiState, rx: *mut f64, ry: *mut f64) {
    todo!() // cxx-internal:ui_main.c.o:_mouseinrect
}

/// C: findradio (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/ui/ui_main.c:582)
/// Calls: cxx-internal:ui_main.c.o:_mouseinrect
#[allow(unused_variables, non_snake_case)]
pub fn findradio(it: *const mjuiItem, ui: *const mjUI, ins: *const mjuiState, con: *const mjrContext) -> i32 {
    todo!() // cxx-internal:ui_main.c.o:_findradio
}

/// C: makeradioline (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/ui/ui_main.c:613)
/// Calls: cxx:_mju_round
#[allow(unused_variables, non_snake_case)]
pub fn makeradioline(it: *const mjuiItem, con: *const mjrContext, sep: *mut i32) {
    todo!() // cxx-internal:ui_main.c.o:_makeradioline
}

/// C: findradioline (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/ui/ui_main.c:642)
/// Calls: cxx-internal:ui_main.c.o:_makeradioline, cxx-internal:ui_main.c.o:_mouseinrect, cxx:_mju_round
#[allow(unused_variables, non_snake_case)]
pub fn findradioline(it: *const mjuiItem, ui: *const mjUI, ins: *const mjuiState, con: *const mjrContext) -> i32 {
    todo!() // cxx-internal:ui_main.c.o:_findradioline
}

/// C: findselect (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/ui/ui_main.c:667)
/// Calls: cxx-internal:ui_main.c.o:_SCL, cxx-internal:ui_main.c.o:_mouseinrect
#[allow(unused_variables, non_snake_case)]
pub fn findselect(it: *const mjuiItem, ui: *const mjUI, ins: *const mjuiState, con: *const mjrContext) -> i32 {
    todo!() // cxx-internal:ui_main.c.o:_findselect
}

/// C: scrollrect (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/ui/ui_main.c:696)
/// Calls: cxx-internal:ui_main.c.o:_SCL, cxx:_mju_round
#[allow(unused_variables, non_snake_case)]
pub fn scrollrect(rect: mjrRect, ui: *const mjUI, con: *const mjrContext, bar: *mut mjrRect, thumb: *mut mjrRect) {
    todo!() // cxx:_scrollrect
}

/// C: inside (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/ui/ui_main.c:716)
#[allow(unused_variables, non_snake_case)]
pub fn inside(x: i32, y: i32, r: mjrRect) -> i32 {
    todo!() // cxx-internal:ui_main.c.o:_inside
}

/// C: insideoval (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/ui/ui_main.c:723)
/// Calls: cxx-internal:ui_main.c.o:_inside
#[allow(unused_variables, non_snake_case)]
pub fn insideoval(x: i32, y: i32, r: mjrRect) -> i32 {
    todo!() // cxx-internal:ui_main.c.o:_insideoval
}

/// C: findmouse (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/ui/ui_main.c:757)
/// Calls: cxx-internal:ui_main.c.o:_inside, cxx-internal:ui_main.c.o:_insideoval, cxx-internal:ui_main.c.o:_mouseinui, cxx:_scrollrect
#[allow(unused_variables, non_snake_case)]
pub fn findmouse(ui: *const mjUI, ins: *const mjuiState, con: *const mjrContext, sect: *mut i32, item: *mut i32) {
    todo!() // cxx-internal:ui_main.c.o:_findmouse
}

/// C: setslider (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/ui/ui_main.c:841)
/// Calls: cxx-internal:ui_main.c.o:_mouseinrect, cxx:_mju_clip, cxx:_mju_round
#[allow(unused_variables, non_snake_case)]
pub fn setslider(it: *mut mjuiItem, ui: *mut mjUI, ins: *const mjuiState, con: *const mjrContext) {
    todo!() // cxx-internal:ui_main.c.o:_setslider
}

/// C: checkedit (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/ui/ui_main.c:868)
/// Calls: cxx:_mju_error
#[allow(unused_variables, non_snake_case)]
pub fn checkedit(text: *const i8, it: *const mjuiItem) -> i32 {
    todo!() // cxx-internal:ui_main.c.o:_checkedit
}

/// C: text2array (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/ui/ui_main.c:914)
/// Calls: cxx:_mju_error
#[allow(unused_variables, non_snake_case)]
pub fn text2array(text: *const i8, it: *const mjuiItem) -> i32 {
    todo!() // cxx-internal:ui_main.c.o:_text2array
}

/// C: array2text (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/ui/ui_main.c:982)
/// Calls: cxx:_mju_error
#[allow(unused_variables, non_snake_case)]
pub fn array2text(text: *mut i8, it: *const mjuiItem) {
    todo!() // cxx-internal:ui_main.c.o:_array2text
}

/// C: validkey (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/ui/ui_main.c:1017)
#[allow(unused_variables, non_snake_case)]
pub fn validkey(key: i32, sz: i32, r#type: i32, state: *const mjuiState) -> i32 {
    todo!() // cxx-internal:ui_main.c.o:_validkey
}

/// C: revealcursor (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/ui/ui_main.c:1098)
/// Calls: cxx-internal:ui_main.c.o:_SCL
#[allow(unused_variables, non_snake_case)]
pub fn revealcursor(r: mjrRect, ui: *mut mjUI, con: *const mjrContext) {
    todo!() // cxx-internal:ui_main.c.o:_revealcursor
}

/// C: setcursor (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/ui/ui_main.c:1124)
/// Calls: cxx-internal:ui_main.c.o:_SCL, cxx-internal:ui_main.c.o:_mouseinrect, cxx-internal:ui_main.c.o:_revealcursor, cxx:_mju_round
#[allow(unused_variables, non_snake_case)]
pub fn setcursor(r: mjrRect, ui: *mut mjUI, ins: *const mjuiState, con: *const mjrContext) {
    todo!() // cxx-internal:ui_main.c.o:_setcursor
}

/// C: parseshortcut (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/ui/ui_main.c:1169)
/// Calls: cxx:_mju_error
#[allow(unused_variables, non_snake_case)]
pub fn parseshortcut(text: *const i8, r#mod: *mut i32, key: *mut i32) {
    todo!() // cxx-internal:ui_main.c.o:_parseshortcut
}

/// C: matchshortcut (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/ui/ui_main.c:1222)
#[allow(unused_variables, non_snake_case)]
pub fn matchshortcut(ins: *const mjuiState, r#mod: i32, key: i32) -> i32 {
    todo!() // cxx-internal:ui_main.c.o:_matchshortcut
}

/// C: setitemskip (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/ui/ui_main.c:1500)
#[allow(unused_variables, non_snake_case)]
pub fn setitemskip(s: *mut mjuiSection, pass: i32) {
    todo!() // cxx-internal:ui_main.c.o:_setitemskip
}

/// C: tryresize (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/ui/ui_main.c:1528)
/// Calls: cxx-internal:ui_main.c.o:_SCL, cxx-internal:ui_main.c.o:_setitemskip
#[allow(unused_variables, non_snake_case)]
pub fn tryresize(ui: *mut mjUI, con: *const mjrContext) {
    todo!() // cxx-internal:ui_main.c.o:_tryresize
}

/// C: insertionsortgroup (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/ui/ui_main.c:1717)
/// Calls: cxx:_mju_error
#[allow(unused_variables, non_snake_case)]
pub fn insertionsortgroup(list: *mut i32, num: i32, stride: i32) {
    todo!() // cxx-internal:ui_main.c.o:_insertionsortgroup
}

/// C: evalpredicate (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/ui/ui_main.c:1823)
#[allow(unused_variables, non_snake_case)]
pub fn evalpredicate(state: i32, predicate: mjfItemEnable, userdata: *mut ()) -> i32 {
    todo!() // cxx-internal:ui_main.c.o:_evalpredicate
}

/// C: shortcuthelp (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/ui/ui_main.c:1836)
/// Calls: cxx-internal:ui_main.c.o:_SCL, cxx-internal:ui_main.c.o:_drawrectangle, cxx-internal:ui_main.c.o:_drawtext, cxx:_mju_strncpy
#[allow(unused_variables, non_snake_case)]
pub fn shortcuthelp(r: mjrRect, modifier: i32, shortcut: i32, ui: *const mjUI, con: *const mjrContext) {
    todo!() // cxx-internal:ui_main.c.o:_shortcuthelp
}

/// C: mjui_themeSpacing (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/ui/ui_main.h:26)
#[allow(unused_variables, non_snake_case)]
pub fn mjui_theme_spacing(ind: i32) -> mjuiThemeSpacing {
    todo!() // cxx:_mjui_themeSpacing
}

/// C: mjui_themeColor (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/ui/ui_main.h:29)
#[allow(unused_variables, non_snake_case)]
pub fn mjui_theme_color(ind: i32) -> mjuiThemeColor {
    todo!() // cxx:_mjui_themeColor
}

/// C: mjui_add (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/ui/ui_main.h:32)
/// Calls: cxx-internal:ui_main.c.o:_parseshortcut, cxx:_mju_error, cxx:_mju_round, cxx:_mju_strncpy
#[allow(unused_variables, non_snake_case)]
pub fn mjui_add(ui: *mut mjUI, def: *const mjuiDef) {
    todo!() // cxx:_mjui_add
}

/// C: mjui_addToSection (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/ui/ui_main.h:35)
/// Calls: cxx:_mjui_add
#[allow(unused_variables, non_snake_case)]
pub fn mjui_add_to_section(ui: *mut mjUI, sect: i32, def: *const mjuiDef) {
    todo!() // cxx:_mjui_addToSection
}

/// C: mjui_resize (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/ui/ui_main.h:38)
/// Calls: cxx-internal:ui_main.c.o:_SCL, cxx-internal:ui_main.c.o:_insertionsortgroup, cxx-internal:ui_main.c.o:_tryresize, cxx:_mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mjui_resize(ui: *mut mjUI, con: *const mjrContext) {
    todo!() // cxx:_mjui_resize
}

/// C: mjui_update (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/ui/ui_main.h:41)
/// Calls: cxx-internal:ui_main.c.o:_SCL, cxx-internal:ui_main.c.o:_array2text, cxx-internal:ui_main.c.o:_checkedit, cxx-internal:ui_main.c.o:_drawoval, cxx-internal:ui_main.c.o:_drawrectangle, cxx-internal:ui_main.c.o:_drawsymbol, cxx-internal:ui_main.c.o:_drawtext, cxx-internal:ui_main.c.o:_drawtextrect, cxx-internal:ui_main.c.o:_evalpredicate, cxx-internal:ui_main.c.o:_findmouse, cxx-internal:ui_main.c.o:_initOpenGL, cxx-internal:ui_main.c.o:_makeradioline, cxx-internal:ui_main.c.o:_radioelement, cxx-internal:ui_main.c.o:_roundcorner, cxx-internal:ui_main.c.o:_shortcuthelp, cxx:_mjr_restoreBuffer, cxx:_mjr_setAux, cxx:_mju_error, cxx:_mju_round
#[allow(unused_variables, non_snake_case)]
pub fn mjui_update(section: i32, item: i32, ui: *const mjUI, state: *const mjuiState, con: *const mjrContext) {
    todo!() // cxx:_mjui_update
}

/// C: mjui_event (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/ui/ui_main.h:45)
/// Calls: cxx-internal:ui_main.c.o:_SCL, cxx-internal:ui_main.c.o:_array2text, cxx-internal:ui_main.c.o:_evalpredicate, cxx-internal:ui_main.c.o:_findmouse, cxx-internal:ui_main.c.o:_findradio, cxx-internal:ui_main.c.o:_findradioline, cxx-internal:ui_main.c.o:_findselect, cxx-internal:ui_main.c.o:_matchshortcut, cxx-internal:ui_main.c.o:_revealcursor, cxx-internal:ui_main.c.o:_setcursor, cxx-internal:ui_main.c.o:_setslider, cxx-internal:ui_main.c.o:_text2array, cxx-internal:ui_main.c.o:_validkey, cxx:_mju_round, cxx:_mjui_resize, cxx:_mjui_update
#[allow(unused_variables, non_snake_case)]
pub fn mjui_event(ui: *mut mjUI, state: *mut mjuiState, con: *const mjrContext) -> *mut mjuiItem {
    todo!() // cxx:_mjui_event
}

/// C: mjui_render (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/ui/ui_main.h:48)
/// Calls: cxx-internal:ui_main.c.o:_SCL, cxx-internal:ui_main.c.o:_drawtext, cxx-internal:ui_main.c.o:_findselect, cxx-internal:ui_main.c.o:_initOpenGL, cxx:_mjr_blitAux, cxx:_mjr_rectangle, cxx:_scrollrect
#[allow(unused_variables, non_snake_case)]
pub fn mjui_render(ui: *mut mjUI, state: *const mjuiState, con: *const mjrContext) {
    todo!() // cxx:_mjui_render
}

