//! Port of: render/classic/glad/glad.c
//! IR hash: d3ac8715281cd691
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mjGlad_get_proc (render/classic/glad/glad.c:58)
#[allow(unused_variables, non_snake_case)]
pub fn mj_glad_get_proc(namez: *const i8) -> *mut () {
    // NOTE: signature changed from previous IR version
    // Previous params: (namez : * const i8)
    // Previous return: * mut ()
    todo!("re-translate: params renamed")
}

/// C: mjGlad_open_gl (render/classic/glad/glad.c:230)
#[allow(unused_variables, non_snake_case)]
pub fn mj_glad_open_gl(get_proc_address: *mut ()) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (get_proc_address : * mut ())
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: mjGlad_close_gl (render/classic/glad/glad.c:252)
#[allow(unused_variables, non_snake_case)]
pub fn mj_glad_close_gl() {
    todo ! ()
}

/// C: mjGlad_get_exts (render/classic/glad/glad.c:294)
#[allow(unused_variables, non_snake_case)]
pub fn mj_glad_get_exts() -> i32 {
    todo ! ()
}

/// C: mjGlad_free_exts (render/classic/glad/glad.c:328)
#[allow(unused_variables, non_snake_case)]
pub fn mj_glad_free_exts() {
    todo ! ()
}

/// C: mjGlad_has_ext (render/classic/glad/glad.c:339)
#[allow(unused_variables, non_snake_case)]
pub fn mj_glad_has_ext(ext: *const i8) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (ext : * const i8)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: mjGlad_load_GL_VERSION_1_0 (render/classic/glad/glad.c:898)
#[allow(unused_variables, non_snake_case)]
pub fn mj_glad_load_gl_version_1_0(load: GLADloadproc) {
    // NOTE: signature changed from previous IR version
    // Previous params: (load : GLADloadproc)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjGlad_load_GL_VERSION_1_1 (render/classic/glad/glad.c:1207)
#[allow(unused_variables, non_snake_case)]
pub fn mj_glad_load_gl_version_1_1(load: GLADloadproc) {
    // NOTE: signature changed from previous IR version
    // Previous params: (load : GLADloadproc)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjGlad_load_GL_VERSION_1_2 (render/classic/glad/glad.c:1240)
#[allow(unused_variables, non_snake_case)]
pub fn mj_glad_load_gl_version_1_2(load: GLADloadproc) {
    // NOTE: signature changed from previous IR version
    // Previous params: (load : GLADloadproc)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjGlad_load_GL_VERSION_1_3 (render/classic/glad/glad.c:1247)
#[allow(unused_variables, non_snake_case)]
pub fn mj_glad_load_gl_version_1_3(load: GLADloadproc) {
    // NOTE: signature changed from previous IR version
    // Previous params: (load : GLADloadproc)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjGlad_load_GL_VERSION_1_4 (render/classic/glad/glad.c:1296)
#[allow(unused_variables, non_snake_case)]
pub fn mj_glad_load_gl_version_1_4(load: GLADloadproc) {
    // NOTE: signature changed from previous IR version
    // Previous params: (load : GLADloadproc)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjGlad_load_GL_VERSION_1_5 (render/classic/glad/glad.c:1346)
#[allow(unused_variables, non_snake_case)]
pub fn mj_glad_load_gl_version_1_5(load: GLADloadproc) {
    // NOTE: signature changed from previous IR version
    // Previous params: (load : GLADloadproc)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjGlad_load_GL_ARB_clip_control (render/classic/glad/glad.c:1368)
#[allow(unused_variables, non_snake_case)]
pub fn mj_glad_load_gl_arb_clip_control(load: GLADloadproc) {
    // NOTE: signature changed from previous IR version
    // Previous params: (load : GLADloadproc)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjGlad_load_GL_ARB_framebuffer_object (render/classic/glad/glad.c:1372)
#[allow(unused_variables, non_snake_case)]
pub fn mj_glad_load_gl_arb_framebuffer_object(load: GLADloadproc) {
    // NOTE: signature changed from previous IR version
    // Previous params: (load : GLADloadproc)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjGlad_load_GL_ARB_vertex_buffer_object (render/classic/glad/glad.c:1395)
#[allow(unused_variables, non_snake_case)]
pub fn mj_glad_load_gl_arb_vertex_buffer_object(load: GLADloadproc) {
    // NOTE: signature changed from previous IR version
    // Previous params: (load : GLADloadproc)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjGlad_load_GL_KHR_debug (render/classic/glad/glad.c:1409)
#[allow(unused_variables, non_snake_case)]
pub fn mj_glad_load_gl_khr_debug(load: GLADloadproc) {
    // NOTE: signature changed from previous IR version
    // Previous params: (load : GLADloadproc)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjGlad_find_extensionsGL (render/classic/glad/glad.c:1434)
/// Calls: mjGlad_free_exts, mjGlad_get_exts, mjGlad_has_ext
#[allow(unused_variables, non_snake_case)]
pub fn mj_glad_find_extensions_gl() -> i32 {
    todo ! ()
}

/// C: mjGlad_find_coreGL (render/classic/glad/glad.c:1447)
#[allow(unused_variables, non_snake_case)]
pub fn mj_glad_find_core_gl() {
    todo ! ()
}

/// C: mjGladLoadGL (render/classic/glad/glad.h:115)
/// Calls: mjGladLoadGLUnsafe
#[allow(unused_variables, non_snake_case)]
pub fn mj_glad_load_gl() -> i32 {
    todo ! ()
}

/// C: mjGladLoadGLUnsafe (render/classic/glad/glad.h:117)
/// Calls: mjGlad_close_gl, mjGlad_find_coreGL, mjGlad_find_extensionsGL, mjGlad_get_proc, mjGlad_load_GL_ARB_clip_control, mjGlad_load_GL_ARB_framebuffer_object, mjGlad_load_GL_ARB_vertex_buffer_object, mjGlad_load_GL_KHR_debug, mjGlad_load_GL_VERSION_1_0, mjGlad_load_GL_VERSION_1_1, mjGlad_load_GL_VERSION_1_2, mjGlad_load_GL_VERSION_1_3, mjGlad_load_GL_VERSION_1_4, mjGlad_load_GL_VERSION_1_5, mjGlad_open_gl
#[allow(unused_variables, non_snake_case)]
pub fn mj_glad_load_gl_unsafe(arg0: *mut ()) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (arg0 : * mut ())
    // Previous return: i32
    todo!("re-translate: params renamed")
}

