//! Port of: render/classic/glad/glad.c
//! IR hash: 73a9f665ec0ecfc0
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mjGlad_get_proc (render/classic/glad/glad.c:58)
#[allow(unused_variables, non_snake_case)]
pub fn mj_glad_get_proc(namez: *const i8) -> *mut () {
    // SAFETY: Accesses module-level static mjGlad_libGL (pointer stored as [u8;8]).
    // On macOS (__APPLE__): no mjGladGetProcAddressPtr, just dlsym on libGL handle.
    unsafe {
        extern "C" { fn dlsym(handle: *mut std::ffi::c_void, symbol: *const i8) -> *mut std::ffi::c_void; }

        let lib_guard = MJGLAD_LIBGL.lock().unwrap();
        let lib_gl: *mut std::ffi::c_void = std::ptr::read(lib_guard.as_ptr() as *const *mut std::ffi::c_void);
        drop(lib_guard);

        if lib_gl.is_null() {
            return std::ptr::null_mut();
        }
        dlsym(lib_gl, namez) as *mut ()
    }
}

/// C: mjGlad_open_gl (render/classic/glad/glad.c:230)
#[allow(unused_variables, non_snake_case)]
pub fn mj_glad_open_gl(get_proc_address: *mut ()) -> i32 {
    // SAFETY: macOS version: tries dlopen on known OpenGL framework paths.
    // Stores handle in mjGlad_libGL static.
    unsafe {
        extern "C" { fn dlopen(filename: *const i8, flags: i32) -> *mut std::ffi::c_void; }
        const RTLD_NOW: i32 = 0x2;
        const RTLD_GLOBAL: i32 = 0x8;

        static NAMES: [&[u8]; 4] = [
            b"../Frameworks/OpenGL.framework/OpenGL\0",
            b"/Library/Frameworks/OpenGL.framework/OpenGL\0",
            b"/System/Library/Frameworks/OpenGL.framework/OpenGL\0",
            b"/System/Library/Frameworks/OpenGL.framework/Versions/Current/OpenGL\0",
        ];

        let mut index: u32 = 0;
        while (index as usize) < NAMES.len() {
            let handle = dlopen(NAMES[index as usize].as_ptr() as *const i8, RTLD_NOW | RTLD_GLOBAL);
            if !handle.is_null() {
                let mut lib_guard = MJGLAD_LIBGL.lock().unwrap();
                std::ptr::write(lib_guard.as_mut_ptr() as *mut *mut std::ffi::c_void, handle);
                return 1;
            }
            index += 1;
        }
        0
    }
}

/// C: mjGlad_close_gl (render/classic/glad/glad.c:252)
#[allow(unused_variables, non_snake_case)]
pub fn mj_glad_close_gl() {
    // SAFETY: Closes the OpenGL framework handle and sets mjGlad_libGL to NULL.
    unsafe {
        extern "C" { fn dlclose(handle: *mut std::ffi::c_void) -> i32; }

        let mut lib_guard = MJGLAD_LIBGL.lock().unwrap();
        let lib_gl: *mut std::ffi::c_void = std::ptr::read(lib_guard.as_ptr() as *const *mut std::ffi::c_void);
        if !lib_gl.is_null() {
            dlclose(lib_gl);
            std::ptr::write(lib_guard.as_mut_ptr() as *mut *mut std::ffi::c_void, std::ptr::null_mut());
        }
    }
}

/// C: mjGlad_get_exts (render/classic/glad/glad.c:294)
#[allow(unused_variables, non_snake_case)]
pub fn mj_glad_get_exts() -> i32 {
    // SAFETY: Reads glGetString fn ptr from static, calls it with GL_EXTENSIONS,
    // stores result in MJGLAD_EXTS static.
    // _GLAD_IS_SOME_NEW_VERSION is not defined, so only the simple path applies.
    unsafe {
        const GL_EXTENSIONS: u32 = 0x1F03;

        // Read glGetString function pointer
        let gs_guard = MJGLAD_GLGETSTRING.lock().unwrap();
        let gl_get_string: Option<unsafe extern "C" fn(u32) -> *const u8> =
            std::ptr::read(gs_guard.as_ptr() as *const Option<unsafe extern "C" fn(u32) -> *const u8>);
        drop(gs_guard);

        if let Some(get_string) = gl_get_string {
            let result = get_string(GL_EXTENSIONS);
            let mut exts_guard = MJGLAD_EXTS.lock().unwrap();
            std::ptr::write(exts_guard.as_mut_ptr() as *mut *const u8, result);
        }
    }
    1
}

/// C: mjGlad_free_exts (render/classic/glad/glad.c:328)
#[allow(unused_variables, non_snake_case)]
pub fn mj_glad_free_exts() {
    // SAFETY: Frees the extension string array allocated in mjGlad_get_exts.
    // Reads mjGlad_exts_i (char**) and mjGlad_num_exts_i (int) from statics.
    unsafe {
        extern "C" { fn free(ptr: *mut std::ffi::c_void); }

        let mut exts_guard = MJGLAD_EXTS_I.lock().unwrap();
        let exts_i: *mut *mut i8 = std::ptr::read(exts_guard.as_ptr() as *const *mut *mut i8);

        if !exts_i.is_null() {
            let num_guard = MJGLAD_NUM_EXTS_I.lock().unwrap();
            let num_exts_i: i32 = std::ptr::read(num_guard.as_ptr() as *const i32);
            drop(num_guard);

            let mut index: i32 = 0;
            while index < num_exts_i {
                free(*exts_i.offset(index as isize) as *mut std::ffi::c_void);
                index += 1;
            }
            free(exts_i as *mut std::ffi::c_void);
            std::ptr::write(exts_guard.as_mut_ptr() as *mut *mut *mut i8, std::ptr::null_mut());
        }
    }
}

/// C: mjGlad_has_ext (render/classic/glad/glad.c:339)
#[allow(unused_variables, non_snake_case)]
pub fn mj_glad_has_ext(ext: *const i8) -> i32 {
    // SAFETY: Searches for an extension string in the loaded extensions list.
    // On macOS (GL < 3 path): linear search in mjGlad_exts string.
    // On GL >= 3 path: search in mjGlad_exts_i array.
    unsafe {
        extern "C" {
            fn strstr(haystack: *const i8, needle: *const i8) -> *const i8;
            fn strlen(s: *const i8) -> usize;
            fn strcmp(s1: *const i8, s2: *const i8) -> i32;
        }

        let max_guard = MJGLAD_MAX_LOADED_MAJOR.lock().unwrap();
        let max_loaded_major: i32 = std::ptr::read(max_guard.as_ptr() as *const i32);
        drop(max_guard);

        if max_loaded_major < 3 {
            // Simple string search path
            let exts_guard = MJGLAD_EXTS.lock().unwrap();
            let extensions: *const i8 = std::ptr::read(exts_guard.as_ptr() as *const *const i8);
            drop(exts_guard);

            if extensions.is_null() || ext.is_null() {
                return 0;
            }

            let ext_len = strlen(ext);
            let mut search = extensions;
            loop {
                let loc = strstr(search, ext);
                if loc.is_null() {
                    return 0;
                }
                let terminator = loc.add(ext_len);
                if (loc == search || *loc.sub(1) == b' ' as i8)
                    && (*terminator == b' ' as i8 || *terminator == 0)
                {
                    return 1;
                }
                search = terminator;
            }
        } else {
            // Array search path (GL >= 3)
            let exts_guard = MJGLAD_EXTS_I.lock().unwrap();
            let exts_i: *mut *mut i8 = std::ptr::read(exts_guard.as_ptr() as *const *mut *mut i8);
            drop(exts_guard);

            if exts_i.is_null() {
                return 0;
            }

            let num_guard = MJGLAD_NUM_EXTS_I.lock().unwrap();
            let num_exts_i: i32 = std::ptr::read(num_guard.as_ptr() as *const i32);
            drop(num_guard);

            let mut index: i32 = 0;
            while index < num_exts_i {
                let e = *exts_i.offset(index as isize);
                if !e.is_null() && strcmp(e, ext) == 0 {
                    return 1;
                }
                index += 1;
            }
        }
        0
    }
}

/// C: mjGlad_load_GL_VERSION_1_0 (render/classic/glad/glad.c:898)
#[allow(unused_variables, non_snake_case)]
pub fn mj_glad_load_gl_version_1_0(load: GLADloadproc) {
    todo!() // mjGlad_load_GL_VERSION_1_0
}

/// C: mjGlad_load_GL_VERSION_1_1 (render/classic/glad/glad.c:1207)
/// Calls: mjCModel::Textures
#[allow(unused_variables, non_snake_case)]
pub fn mj_glad_load_gl_version_1_1(load: GLADloadproc) {
    todo!() // mjGlad_load_GL_VERSION_1_1
}

/// C: mjGlad_load_GL_VERSION_1_2 (render/classic/glad/glad.c:1240)
#[allow(unused_variables, non_snake_case)]
pub fn mj_glad_load_gl_version_1_2(load: GLADloadproc) {
    todo!() // mjGlad_load_GL_VERSION_1_2
}

/// C: mjGlad_load_GL_VERSION_1_3 (render/classic/glad/glad.c:1247)
#[allow(unused_variables, non_snake_case)]
pub fn mj_glad_load_gl_version_1_3(load: GLADloadproc) {
    todo!() // mjGlad_load_GL_VERSION_1_3
}

/// C: mjGlad_load_GL_VERSION_1_4 (render/classic/glad/glad.c:1296)
#[allow(unused_variables, non_snake_case)]
pub fn mj_glad_load_gl_version_1_4(load: GLADloadproc) {
    todo!() // mjGlad_load_GL_VERSION_1_4
}

/// C: mjGlad_load_GL_VERSION_1_5 (render/classic/glad/glad.c:1346)
/// Calls: mjCAsset::Data
#[allow(unused_variables, non_snake_case)]
pub fn mj_glad_load_gl_version_1_5(load: GLADloadproc) {
    todo!() // mjGlad_load_GL_VERSION_1_5
}

/// C: mjGlad_load_GL_ARB_clip_control (render/classic/glad/glad.c:1368)
#[allow(unused_variables, non_snake_case)]
pub fn mj_glad_load_gl_arb_clip_control(load: GLADloadproc) {
    todo!() // mjGlad_load_GL_ARB_clip_control
}

/// C: mjGlad_load_GL_ARB_framebuffer_object (render/classic/glad/glad.c:1372)
#[allow(unused_variables, non_snake_case)]
pub fn mj_glad_load_gl_arb_framebuffer_object(load: GLADloadproc) {
    todo!() // mjGlad_load_GL_ARB_framebuffer_object
}

/// C: mjGlad_load_GL_ARB_vertex_buffer_object (render/classic/glad/glad.c:1395)
#[allow(unused_variables, non_snake_case)]
pub fn mj_glad_load_gl_arb_vertex_buffer_object(load: GLADloadproc) {
    todo!() // mjGlad_load_GL_ARB_vertex_buffer_object
}

/// C: mjGlad_load_GL_KHR_debug (render/classic/glad/glad.c:1409)
/// Calls: mjCCache::Insert
#[allow(unused_variables, non_snake_case)]
pub fn mj_glad_load_gl_khr_debug(load: GLADloadproc) {
    todo!() // mjGlad_load_GL_KHR_debug
}

/// C: mjGlad_find_extensionsGL (render/classic/glad/glad.c:1434)
/// Calls: mjGlad_free_exts, mjGlad_get_exts, mjGlad_has_ext
#[allow(unused_variables, non_snake_case)]
pub fn mj_glad_find_extensions_gl() -> i32 {
    if mj_glad_get_exts() == 0 {
        return 0;
    }

    // SAFETY: Writing i32 results from mj_glad_has_ext into the extension statics.
    unsafe {
        let val = mj_glad_has_ext(b"GL_ARB_clip_control\0".as_ptr() as *const i8);
        let mut g = MJGLAD_GL_ARB_CLIP_CONTROL.lock().unwrap();
        std::ptr::write(g.as_mut_ptr() as *mut i32, val);
        drop(g);

        let val = mj_glad_has_ext(b"GL_ARB_depth_buffer_float\0".as_ptr() as *const i8);
        let mut g = MJGLAD_GL_ARB_DEPTH_BUFFER_FLOAT.lock().unwrap();
        std::ptr::write(g.as_mut_ptr() as *mut i32, val);
        drop(g);

        let val = mj_glad_has_ext(b"GL_ARB_framebuffer_object\0".as_ptr() as *const i8);
        let mut g = MJGLAD_GL_ARB_FRAMEBUFFER_OBJECT.lock().unwrap();
        std::ptr::write(g.as_mut_ptr() as *mut i32, val);
        drop(g);

        let val = mj_glad_has_ext(b"GL_ARB_seamless_cube_map\0".as_ptr() as *const i8);
        let mut g = MJGLAD_GL_ARB_SEAMLESS_CUBE_MAP.lock().unwrap();
        std::ptr::write(g.as_mut_ptr() as *mut i32, val);
        drop(g);

        let val = mj_glad_has_ext(b"GL_ARB_vertex_buffer_object\0".as_ptr() as *const i8);
        let mut g = MJGLAD_GL_ARB_VERTEX_BUFFER_OBJECT.lock().unwrap();
        std::ptr::write(g.as_mut_ptr() as *mut i32, val);
        drop(g);

        let val = mj_glad_has_ext(b"GL_EXT_texture_sRGB\0".as_ptr() as *const i8);
        let mut g = MJGLAD_GL_EXT_TEXTURE_SRGB.lock().unwrap();
        std::ptr::write(g.as_mut_ptr() as *mut i32, val);
        drop(g);

        let val = mj_glad_has_ext(b"GL_KHR_debug\0".as_ptr() as *const i8);
        let mut g = MJGLAD_GL_KHR_DEBUG.lock().unwrap();
        std::ptr::write(g.as_mut_ptr() as *mut i32, val);
        drop(g);
    }

    mj_glad_free_exts();
    1
}

/// C: mjGlad_find_coreGL (render/classic/glad/glad.c:1447)
#[allow(unused_variables, non_snake_case)]
pub fn mj_glad_find_core_gl() {
    // SAFETY: Calls glGetString(GL_VERSION), parses version, sets statics.
    unsafe {
        extern "C" {
            fn strlen(s: *const i8) -> usize;
            fn strncmp(s1: *const i8, s2: *const i8, n: usize) -> i32;
            fn sscanf(s: *const i8, fmt: *const i8, ...) -> i32;
        }

        const GL_VERSION: u32 = 0x1F02;

        // Read glGetString function pointer
        let gs_guard = MJGLAD_GLGETSTRING.lock().unwrap();
        let gl_get_string: Option<unsafe extern "C" fn(u32) -> *const i8> =
            std::ptr::read(gs_guard.as_ptr() as *const Option<unsafe extern "C" fn(u32) -> *const i8>);
        drop(gs_guard);

        let get_string = match gl_get_string {
            Some(f) => f,
            None => return,
        };

        let mut version: *const i8 = get_string(GL_VERSION);
        if version.is_null() {
            return;
        }

        let prefixes: [*const i8; 4] = [
            b"OpenGL ES-CM \0".as_ptr() as *const i8,
            b"OpenGL ES-CL \0".as_ptr() as *const i8,
            b"OpenGL ES \0".as_ptr() as *const i8,
            std::ptr::null(),
        ];

        let mut i: i32 = 0;
        while !prefixes[i as usize].is_null() {
            let length = strlen(prefixes[i as usize]);
            if strncmp(version, prefixes[i as usize], length) == 0 {
                version = version.add(length);
                break;
            }
            i += 1;
        }

        let mut major: i32 = 0;
        let mut minor: i32 = 0;
        sscanf(version, b"%d.%d\0".as_ptr() as *const i8, &mut major as *mut i32, &mut minor as *mut i32);

        // mjGLVersion.major = major; mjGLVersion.minor = minor;
        let mut ver_guard = MJGLVERSION.lock().unwrap();
        std::ptr::write(ver_guard.as_mut_ptr() as *mut i32, major);
        std::ptr::write((ver_guard.as_mut_ptr() as *mut i32).add(1), minor);
        drop(ver_guard);

        // mjGlad_max_loaded_major = major; mjGlad_max_loaded_minor = minor;
        let mut maj_guard = MJGLAD_MAX_LOADED_MAJOR.lock().unwrap();
        std::ptr::write(maj_guard.as_mut_ptr() as *mut i32, major);
        drop(maj_guard);
        let mut min_guard = MJGLAD_MAX_LOADED_MINOR.lock().unwrap();
        std::ptr::write(min_guard.as_mut_ptr() as *mut i32, minor);
        drop(min_guard);

        // Set version booleans
        let v1_0: i32 = ((major == 1 && minor >= 0) || major > 1) as i32;
        let v1_1: i32 = ((major == 1 && minor >= 1) || major > 1) as i32;
        let v1_2: i32 = ((major == 1 && minor >= 2) || major > 1) as i32;
        let v1_3: i32 = ((major == 1 && minor >= 3) || major > 1) as i32;
        let v1_4: i32 = ((major == 1 && minor >= 4) || major > 1) as i32;
        let v1_5: i32 = ((major == 1 && minor >= 5) || major > 1) as i32;

        let mut g = MJGLAD_GL_VERSION_1_0.lock().unwrap();
        std::ptr::write(g.as_mut_ptr() as *mut i32, v1_0);
        drop(g);
        let mut g = MJGLAD_GL_VERSION_1_1.lock().unwrap();
        std::ptr::write(g.as_mut_ptr() as *mut i32, v1_1);
        drop(g);
        let mut g = MJGLAD_GL_VERSION_1_2.lock().unwrap();
        std::ptr::write(g.as_mut_ptr() as *mut i32, v1_2);
        drop(g);
        let mut g = MJGLAD_GL_VERSION_1_3.lock().unwrap();
        std::ptr::write(g.as_mut_ptr() as *mut i32, v1_3);
        drop(g);
        let mut g = MJGLAD_GL_VERSION_1_4.lock().unwrap();
        std::ptr::write(g.as_mut_ptr() as *mut i32, v1_4);
        drop(g);
        let mut g = MJGLAD_GL_VERSION_1_5.lock().unwrap();
        std::ptr::write(g.as_mut_ptr() as *mut i32, v1_5);
        drop(g);

        // Re-read mjGLVersion for the final check
        let ver_guard = MJGLVERSION.lock().unwrap();
        let gl_major: i32 = std::ptr::read(ver_guard.as_ptr() as *const i32);
        let gl_minor: i32 = std::ptr::read((ver_guard.as_ptr() as *const i32).add(1));
        drop(ver_guard);

        if gl_major > 1 || (gl_major >= 1 && gl_minor >= 5) {
            let mut maj_guard = MJGLAD_MAX_LOADED_MAJOR.lock().unwrap();
            std::ptr::write(maj_guard.as_mut_ptr() as *mut i32, 1);
            drop(maj_guard);
            let mut min_guard = MJGLAD_MAX_LOADED_MINOR.lock().unwrap();
            std::ptr::write(min_guard.as_mut_ptr() as *mut i32, 5);
            drop(min_guard);
        }
    }
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
    todo!() // mjGladLoadGLUnsafe
}

