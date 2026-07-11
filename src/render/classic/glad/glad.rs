//! Port of: render/classic/glad/glad.c
//! IR hash: 05737965add36adb
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mjGlad_get_proc (render/classic/glad/glad.c:58)
#[allow(unused_variables, non_snake_case)]
pub fn mj_glad_get_proc(namez: *const i8) -> *mut () {
    extern "C" { fn mjGlad_get_proc(namez: *const i8) -> *mut (); }
    // SAFETY: delegates to C implementation
    unsafe { mjGlad_get_proc(namez) }
}

/// C: mjGlad_open_gl (render/classic/glad/glad.c:230)
#[allow(unused_variables, non_snake_case)]
pub fn mj_glad_open_gl(get_proc_address: *mut ()) -> i32 {
    extern "C" { fn mjGlad_open_gl(get_proc_address: *mut ()) -> i32; }
    // SAFETY: delegates to C++ implementation; caller guarantees pointer validity
    unsafe { mjGlad_open_gl(get_proc_address) }
}

/// C: mjGlad_close_gl (render/classic/glad/glad.c:252)
#[allow(unused_variables, non_snake_case)]
pub fn mj_glad_close_gl() {
    let _sv = 0_usize;
    extern "C" { fn mjGlad_close_gl(); }
    // SAFETY: delegates to C implementation
    unsafe { mjGlad_close_gl() }
}

/// C: mjGlad_get_exts (render/classic/glad/glad.c:294)
#[allow(unused_variables, non_snake_case)]
pub fn mj_glad_get_exts() -> i32 {
    extern "C" { fn mjGlad_get_exts() -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { mjGlad_get_exts() }
}

/// C: mjGlad_free_exts (render/classic/glad/glad.c:328)
#[allow(unused_variables, non_snake_case)]
pub fn mj_glad_free_exts() {
    extern "C" { fn mjGlad_free_exts(); }
    // SAFETY: delegates to C implementation
    unsafe { mjGlad_free_exts() }
}

/// C: mjGlad_has_ext (render/classic/glad/glad.c:339)
#[allow(unused_variables, non_snake_case)]
pub fn mj_glad_has_ext(ext: *const i8) -> i32 {
    extern "C" { fn mjGlad_has_ext(ext: *const i8) -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { mjGlad_has_ext(ext) }
}

/// C: mjGlad_load_GL_VERSION_1_0 (render/classic/glad/glad.c:898)
#[allow(unused_variables, non_snake_case)]
pub fn mj_glad_load_gl_version_1_0(load: GLADloadproc) {
    extern "C" { fn mjGlad_load_GL_VERSION_1_0(load: GLADloadproc); }
    // SAFETY: delegates to C implementation
    unsafe { mjGlad_load_GL_VERSION_1_0(load) }
}

/// C: mjGlad_load_GL_VERSION_1_1 (render/classic/glad/glad.c:1207)
#[allow(unused_variables, non_snake_case)]
pub fn mj_glad_load_gl_version_1_1(load: GLADloadproc) {
    extern "C" { fn mjGlad_load_GL_VERSION_1_1(load: GLADloadproc); }
    // SAFETY: delegates to C implementation
    unsafe { mjGlad_load_GL_VERSION_1_1(load) }
}

/// C: mjGlad_load_GL_VERSION_1_2 (render/classic/glad/glad.c:1240)
#[allow(unused_variables, non_snake_case)]
pub fn mj_glad_load_gl_version_1_2(load: GLADloadproc) {
    extern "C" { fn mjGlad_load_GL_VERSION_1_2(load: GLADloadproc); }
    // SAFETY: delegates to C implementation
    unsafe { mjGlad_load_GL_VERSION_1_2(load) }
}

/// C: mjGlad_load_GL_VERSION_1_3 (render/classic/glad/glad.c:1247)
#[allow(unused_variables, non_snake_case)]
pub fn mj_glad_load_gl_version_1_3(load: GLADloadproc) {
    extern "C" { fn mjGlad_load_GL_VERSION_1_3(load: GLADloadproc); }
    // SAFETY: delegates to C implementation
    unsafe { mjGlad_load_GL_VERSION_1_3(load) }
}

/// C: mjGlad_load_GL_VERSION_1_4 (render/classic/glad/glad.c:1296)
#[allow(unused_variables, non_snake_case)]
pub fn mj_glad_load_gl_version_1_4(load: GLADloadproc) {
    extern "C" { fn mjGlad_load_GL_VERSION_1_4(load: GLADloadproc); }
    // SAFETY: delegates to C implementation
    unsafe { mjGlad_load_GL_VERSION_1_4(load) }
}

/// C: mjGlad_load_GL_VERSION_1_5 (render/classic/glad/glad.c:1346)
#[allow(unused_variables, non_snake_case)]
pub fn mj_glad_load_gl_version_1_5(load: GLADloadproc) {
    extern "C" { fn mjGlad_load_GL_VERSION_1_5(load: GLADloadproc); }
    // SAFETY: delegates to C implementation
    unsafe { mjGlad_load_GL_VERSION_1_5(load) }
}

/// C: mjGlad_load_GL_ARB_clip_control (render/classic/glad/glad.c:1368)
#[allow(unused_variables, non_snake_case)]
pub fn mj_glad_load_gl_arb_clip_control(load: GLADloadproc) {
    extern "C" { fn mjGlad_load_GL_ARB_clip_control(load: GLADloadproc); }
    // SAFETY: delegates to C implementation
    unsafe { mjGlad_load_GL_ARB_clip_control(load) }
}

/// C: mjGlad_load_GL_ARB_framebuffer_object (render/classic/glad/glad.c:1372)
#[allow(unused_variables, non_snake_case)]
pub fn mj_glad_load_gl_arb_framebuffer_object(load: GLADloadproc) {
    extern "C" { fn mjGlad_load_GL_ARB_framebuffer_object(load: GLADloadproc); }
    // SAFETY: delegates to C implementation
    unsafe { mjGlad_load_GL_ARB_framebuffer_object(load) }
}

/// C: mjGlad_load_GL_ARB_vertex_buffer_object (render/classic/glad/glad.c:1395)
#[allow(unused_variables, non_snake_case)]
pub fn mj_glad_load_gl_arb_vertex_buffer_object(load: GLADloadproc) {
    extern "C" { fn mjGlad_load_GL_ARB_vertex_buffer_object(load: GLADloadproc); }
    // SAFETY: delegates to C implementation
    unsafe { mjGlad_load_GL_ARB_vertex_buffer_object(load) }
}

/// C: mjGlad_load_GL_KHR_debug (render/classic/glad/glad.c:1409)
#[allow(unused_variables, non_snake_case)]
pub fn mj_glad_load_gl_khr_debug(load: GLADloadproc) {
    extern "C" { fn mjGlad_load_GL_KHR_debug(load: GLADloadproc); }
    // SAFETY: delegates to C implementation
    unsafe { mjGlad_load_GL_KHR_debug(load) }
}

/// C: mjGlad_find_extensionsGL (render/classic/glad/glad.c:1434)
/// Calls: mjGlad_free_exts, mjGlad_get_exts, mjGlad_has_ext
#[allow(unused_variables, non_snake_case)]
pub fn mj_glad_find_extensions_gl() -> i32 {
    extern "C" { fn mjGlad_find_extensionsGL() -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { mjGlad_find_extensionsGL() }
}

/// C: mjGlad_find_coreGL (render/classic/glad/glad.c:1447)
#[allow(unused_variables, non_snake_case)]
pub fn mj_glad_find_core_gl() {
    let _sv = 0_usize;
    extern "C" { fn mjGlad_find_coreGL(); }
    // SAFETY: delegates to C implementation
    unsafe { mjGlad_find_coreGL() }
}

/// C: mjGladLoadGL (render/classic/glad/glad.h:115)
/// Calls: mjGladLoadGLUnsafe
#[allow(unused_variables, non_snake_case)]
pub fn mj_glad_load_gl() -> i32 {
    extern "C" { fn mjGladLoadGL() -> i32; }
    // SAFETY: delegates to C++ implementation; caller guarantees pointer validity
    unsafe { mjGladLoadGL() }
}

/// C: mjGladLoadGLUnsafe (render/classic/glad/glad.h:117)
/// Calls: mjGlad_close_gl, mjGlad_find_coreGL, mjGlad_find_extensionsGL, mjGlad_get_proc, mjGlad_load_GL_ARB_clip_control, mjGlad_load_GL_ARB_framebuffer_object, mjGlad_load_GL_ARB_vertex_buffer_object, mjGlad_load_GL_KHR_debug, mjGlad_load_GL_VERSION_1_0, mjGlad_load_GL_VERSION_1_1, mjGlad_load_GL_VERSION_1_2, mjGlad_load_GL_VERSION_1_3, mjGlad_load_GL_VERSION_1_4, mjGlad_load_GL_VERSION_1_5, mjGlad_open_gl
#[allow(unused_variables, non_snake_case)]
pub fn mj_glad_load_gl_unsafe(arg0: *mut ()) -> i32 {
    extern "C" { fn mjGladLoadGLUnsafe(arg0: *mut ()) -> i32; }
    // SAFETY: delegates to C++ implementation; caller guarantees pointer validity
    unsafe { mjGladLoadGLUnsafe(arg0) }
}

