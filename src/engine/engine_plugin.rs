//! Port of: engine/engine_plugin.h
//! IR hash: 9343293228317031
//! CODEGEN: source paths, owners, and callable names are locked.

use crate::types::*;

/// C: mj_loadPluginLibrary (engine/engine_plugin.h:60)
/// Calls: cxx:_mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mj_loadPluginLibrary(path: *const i8) {
    extern "C" {
        fn dlopen(filename: *const i8, flags: i32) -> *mut ();
        fn dlerror() -> *const i8;
        fn mju_error(msg: *const i8, ...);
    }
    const RTLD_NOW: i32 = 0x2;
    const RTLD_LOCAL: i32 = 0x4;
    // SAFETY: path is a valid C string, dlopen/dlerror are standard POSIX
    unsafe {
        let handle = dlopen(path, RTLD_NOW | RTLD_LOCAL);
        if handle.is_null() {
            let error = dlerror();
            if !error.is_null() {
                mju_error(
                    b"Error loading plugin library '%s': %s\n\0".as_ptr() as *const i8,
                    path,
                    error,
                );
            } else {
                mju_error(
                    b"Unknown error loading plugin library '%s'\n\0".as_ptr() as *const i8,
                    path,
                );
            }
        }
    }
}

