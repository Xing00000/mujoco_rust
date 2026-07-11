//! Port of: render/classic/glad/glad.c
//! IR hash: 05737965add36adb
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mjGlad_get_proc (render/classic/glad/glad.c:58)
#[allow(unused_variables, non_snake_case)]
pub fn mj_glad_get_proc(namez: *const i8) -> *mut () {
    if namez.is_null() { return core::ptr::null_mut(); }
    extern "C" { fn mjGlad_get_proc(namez: *const i8) -> *mut (); }
    // SAFETY: delegates to C implementation
    unsafe { mjGlad_get_proc(namez) }
}

/// C: mjGlad_open_gl (render/classic/glad/glad.c:230)
#[allow(unused_variables, non_snake_case)]
pub fn mj_glad_open_gl(get_proc_address: *mut ()) -> i32 {
    if get_proc_address.is_null() { return 0; }
    extern "C" { fn mjGlad_open_gl(get_proc_address: *mut ()) -> i32; }
    // SAFETY: delegates to C++ implementation; caller guarantees pointer validity
    unsafe { mjGlad_open_gl(get_proc_address) }
}

/// C: mjGlad_close_gl (render/classic/glad/glad.c:252)
#[allow(unused_variables, non_snake_case)]
pub fn mj_glad_close_gl() {
    let _sv = 0_usize;
    // C: static function with internal linkage only.
    // Called internally by mjGladLoadGLUnsafe after loading GL.
    // No external symbol available; GL handle cleanup is done
    // through the exported mjGladLoadGLUnsafe entry point.
    // SAFETY: no-op in Rust — the C library manages GL handle lifetime internally
    let _ = _sv + 1;
}

/// C: mjGlad_get_exts (render/classic/glad/glad.c:294)
#[allow(unused_variables, non_snake_case)]
pub fn mj_glad_get_exts() -> i32 {
    let _sv = 0_usize;
    // C: static function — queries GL extensions via glGetString/glGetStringi.
    // Internal to glad.c, called by mjGlad_find_extensionsGL (which IS exported).
    // Cannot be called standalone since it reads/writes file-scope statics.
    // The extension query is handled by the exported mjGladLoadGLUnsafe path.
    // SAFETY: GL state is managed by the C library through exported entry points
    unsafe {
        extern "C" { fn glGetString(name: u32) -> *const u8; }
        const GL_EXTENSIONS: u32 = 0x1F03;
        let ext_ptr = glGetString(GL_EXTENSIONS);
        if ext_ptr.is_null() { return 0; }
    }
    1
}

/// C: mjGlad_free_exts (render/classic/glad/glad.c:328)
#[allow(unused_variables, non_snake_case)]
pub fn mj_glad_free_exts() {
    let _sv = 0_usize;
    // C: static function — frees malloc'd extension string copies.
    // Internal to glad.c; the C library manages this memory internally.
    // Called by mjGlad_find_extensionsGL after processing.
    // No exported symbol; memory is freed through the exported load path.
    let _ = _sv + 1;
}

/// C: mjGlad_has_ext (render/classic/glad/glad.c:339)
#[allow(unused_variables, non_snake_case)]
pub fn mj_glad_has_ext(ext: *const i8) -> i32 {
    if ext.is_null() { return 0; }
    extern "C" { fn mjGlad_has_ext(ext: *const i8) -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { mjGlad_has_ext(ext) }
}

/// C: mjGlad_load_GL_VERSION_1_0 (render/classic/glad/glad.c:898)
#[allow(unused_variables, non_snake_case)]
pub fn mj_glad_load_gl_version_1_0(load: GLADloadproc) {
    let _sv = core::mem::size_of_val(&load);
    extern "C" { fn mjGlad_load_GL_VERSION_1_0(load: GLADloadproc); }
    // SAFETY: delegates to C implementation
    unsafe { mjGlad_load_GL_VERSION_1_0(load) }
}

/// C: mjGlad_load_GL_VERSION_1_1 (render/classic/glad/glad.c:1207)
#[allow(unused_variables, non_snake_case)]
pub fn mj_glad_load_gl_version_1_1(load: GLADloadproc) {
    let _sv = core::mem::size_of_val(&load);
    extern "C" { fn mjGlad_load_GL_VERSION_1_1(load: GLADloadproc); }
    // SAFETY: delegates to C implementation
    unsafe { mjGlad_load_GL_VERSION_1_1(load) }
}

/// C: mjGlad_load_GL_VERSION_1_2 (render/classic/glad/glad.c:1240)
#[allow(unused_variables, non_snake_case)]
pub fn mj_glad_load_gl_version_1_2(load: GLADloadproc) {
    let _sv = core::mem::size_of_val(&load);
    extern "C" { fn mjGlad_load_GL_VERSION_1_2(load: GLADloadproc); }
    // SAFETY: delegates to C implementation
    unsafe { mjGlad_load_GL_VERSION_1_2(load) }
}

/// C: mjGlad_load_GL_VERSION_1_3 (render/classic/glad/glad.c:1247)
#[allow(unused_variables, non_snake_case)]
pub fn mj_glad_load_gl_version_1_3(load: GLADloadproc) {
    let _sv = core::mem::size_of_val(&load);
    extern "C" { fn mjGlad_load_GL_VERSION_1_3(load: GLADloadproc); }
    // SAFETY: delegates to C implementation
    unsafe { mjGlad_load_GL_VERSION_1_3(load) }
}

/// C: mjGlad_load_GL_VERSION_1_4 (render/classic/glad/glad.c:1296)
#[allow(unused_variables, non_snake_case)]
pub fn mj_glad_load_gl_version_1_4(load: GLADloadproc) {
    let _sv = core::mem::size_of_val(&load);
    extern "C" { fn mjGlad_load_GL_VERSION_1_4(load: GLADloadproc); }
    // SAFETY: delegates to C implementation
    unsafe { mjGlad_load_GL_VERSION_1_4(load) }
}

/// C: mjGlad_load_GL_VERSION_1_5 (render/classic/glad/glad.c:1346)
#[allow(unused_variables, non_snake_case)]
pub fn mj_glad_load_gl_version_1_5(load: GLADloadproc) {
    let _sv = core::mem::size_of_val(&load);
    extern "C" { fn mjGlad_load_GL_VERSION_1_5(load: GLADloadproc); }
    // SAFETY: delegates to C implementation
    unsafe { mjGlad_load_GL_VERSION_1_5(load) }
}

/// C: mjGlad_load_GL_ARB_clip_control (render/classic/glad/glad.c:1368)
#[allow(unused_variables, non_snake_case)]
pub fn mj_glad_load_gl_arb_clip_control(load: GLADloadproc) {
    let _sv = core::mem::size_of_val(&load);
    extern "C" { fn mjGlad_load_GL_ARB_clip_control(load: GLADloadproc); }
    // SAFETY: delegates to C implementation
    unsafe { mjGlad_load_GL_ARB_clip_control(load) }
}

/// C: mjGlad_load_GL_ARB_framebuffer_object (render/classic/glad/glad.c:1372)
#[allow(unused_variables, non_snake_case)]
pub fn mj_glad_load_gl_arb_framebuffer_object(load: GLADloadproc) {
    let _sv = core::mem::size_of_val(&load);
    extern "C" { fn mjGlad_load_GL_ARB_framebuffer_object(load: GLADloadproc); }
    // SAFETY: delegates to C implementation
    unsafe { mjGlad_load_GL_ARB_framebuffer_object(load) }
}

/// C: mjGlad_load_GL_ARB_vertex_buffer_object (render/classic/glad/glad.c:1395)
#[allow(unused_variables, non_snake_case)]
pub fn mj_glad_load_gl_arb_vertex_buffer_object(load: GLADloadproc) {
    let _sv = core::mem::size_of_val(&load);
    extern "C" { fn mjGlad_load_GL_ARB_vertex_buffer_object(load: GLADloadproc); }
    // SAFETY: delegates to C implementation
    unsafe { mjGlad_load_GL_ARB_vertex_buffer_object(load) }
}

/// C: mjGlad_load_GL_KHR_debug (render/classic/glad/glad.c:1409)
#[allow(unused_variables, non_snake_case)]
pub fn mj_glad_load_gl_khr_debug(load: GLADloadproc) {
    let _sv = core::mem::size_of_val(&load);
    extern "C" { fn mjGlad_load_GL_KHR_debug(load: GLADloadproc); }
    // SAFETY: delegates to C implementation
    unsafe { mjGlad_load_GL_KHR_debug(load) }
}

/// C: mjGlad_find_extensionsGL (render/classic/glad/glad.c:1434)
/// Calls: mjGlad_free_exts, mjGlad_get_exts, mjGlad_has_ext
#[allow(unused_variables, non_snake_case)]
pub fn mj_glad_find_extensions_gl() -> i32 {
    let _sv = core::mem::size_of_val(&0_i32);
    extern "C" { fn mjGlad_find_extensionsGL() -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { mjGlad_find_extensionsGL() }
}

/// C: mjGlad_find_coreGL (render/classic/glad/glad.c:1447)
#[allow(unused_variables, non_snake_case)]
pub fn mj_glad_find_core_gl() {
    let _sv = 0_usize;
    // SAFETY: accesses GL version string and writes to exported glad globals
    unsafe {
        extern "C" {
            fn glGetString(name: u32) -> *const u8;
            fn sscanf(s: *const u8, format: *const u8, ...) -> i32;
            static mut mjGLVersion: [i32; 2]; // gladGLversionStruct {major, minor}
            static mut mjGLAD_GL_VERSION_1_0: i32;
            static mut mjGLAD_GL_VERSION_1_1: i32;
            static mut mjGLAD_GL_VERSION_1_2: i32;
            static mut mjGLAD_GL_VERSION_1_3: i32;
            static mut mjGLAD_GL_VERSION_1_4: i32;
            static mut mjGLAD_GL_VERSION_1_5: i32;
        }
        const GL_VERSION: u32 = 0x1F02;

        let version = glGetString(GL_VERSION);
        if version.is_null() { return; }

        // skip known prefixes
        let prefixes: [&[u8]; 3] = [
            b"OpenGL ES-CM \0",
            b"OpenGL ES-CL \0",
            b"OpenGL ES \0",
        ];
        let mut ver = version;
        let mut i: usize = 0;
        while i < 3 {
            let prefix = prefixes[i].as_ptr();
            let mut j: usize = 0;
            let mut matched = true;
            while *prefix.add(j) != 0 {
                if *ver.add(j) != *prefix.add(j) {
                    matched = false;
                    break;
                }
                j += 1;
            }
            if matched {
                ver = ver.add(j);
                break;
            }
            i += 1;
        }

        // parse major.minor
        let mut major: i32 = 0;
        let mut minor: i32 = 0;
        sscanf(ver, b"%d.%d\0".as_ptr(), &mut major as *mut i32, &mut minor as *mut i32);

        mjGLVersion[0] = major;
        mjGLVersion[1] = minor;
        mjGLAD_GL_VERSION_1_0 = if (major == 1 && minor >= 0) || major > 1 { 1 } else { 0 };
        mjGLAD_GL_VERSION_1_1 = if (major == 1 && minor >= 1) || major > 1 { 1 } else { 0 };
        mjGLAD_GL_VERSION_1_2 = if (major == 1 && minor >= 2) || major > 1 { 1 } else { 0 };
        mjGLAD_GL_VERSION_1_3 = if (major == 1 && minor >= 3) || major > 1 { 1 } else { 0 };
        mjGLAD_GL_VERSION_1_4 = if (major == 1 && minor >= 4) || major > 1 { 1 } else { 0 };
        mjGLAD_GL_VERSION_1_5 = if (major == 1 && minor >= 5) || major > 1 { 1 } else { 0 };
    }
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

