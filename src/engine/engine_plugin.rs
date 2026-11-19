//! Port of: engine/engine_plugin.cc
//! IR hash: 4b6a2812f57fb5ef
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;


/// # Safety
/// The records must be live. Text must be terminated or have enough readable bytes. When the count is positive, the list must be live and contain that many elements. All accessed byte ranges must be readable, and there must be no concurrent mutation.
#[allow(unused_variables, non_snake_case)]
pub unsafe fn __c2rust_global_table_mjp_plugin___object_equal_44f226ace34d71e21d03e87817e78b0ebe377c93b581eb64ea90d07298790c52(arg0: *const mjpPlugin, arg1: *const mjpPlugin) -> bool {
    extern "C" {
        fn memcmp(left: *const core::ffi::c_void, right: *const core::ffi::c_void, count: usize) -> core::ffi::c_int;
        fn strncmp(left: *const core::ffi::c_char, right: *const core::ffi::c_char, count: usize) -> core::ffi::c_int;
    }
    unsafe {
        let left = arg0.cast::<u8>();
        let right = arg1.cast::<u8>();
        let first = left.add(0).cast::<*const core::ffi::c_char>().read();
        let second = right.add(0).cast::<*const core::ffi::c_char>().read();
        if first.is_null() != second.is_null() { return false; }
        if !first.is_null() && strncmp(first, second, 1024) != 0 { return false; }
        let count = left.add(8).cast::<i32>().read();
        if count != right.add(8).cast::<i32>().read() { return false; }
        for i in 0..count {
            let first = left.add(16).cast::<*const *const core::ffi::c_char>().read().add(i as usize).read();
            let second = right.add(16).cast::<*const *const core::ffi::c_char>().read().add(i as usize).read();
            if !first.is_null() && second.is_null() { return false; }
            if !first.is_null() && !second.is_null() && strncmp(first, second, 1024) != 0 { return false; }
        }
        memcmp(left.add(24).cast(), right.add(24).cast(), 128) == 0
    }
}
