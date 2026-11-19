//! Port of: render/classic/glad/glad.c
//! IR hash: 782799f14c988ce4
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;


pub fn __c2rust_mj_glad_close_gl_2b1cf588ee860ec0fa498bb9071d51c6321fd84a4d1042a1ad4d2ab5cbc2c282() -> () {
    extern "C" {
        fn c2rust_system_dlclose(handle: *mut core::ffi::c_void) -> core::ffi::c_int;
    }

    unsafe {
        let slot = ::core::ptr::addr_of_mut!(crate::types::c2rust_static_61630855e0d688d9985a42844ad04cc83f9befafc9aae305b12decf8522f52f1);
        let handle = *slot;
        if !handle.is_null() {
            let _ = c2rust_system_dlclose(handle.cast::<core::ffi::c_void>());
            *slot = ::core::ptr::null_mut();
        }
    }
}
