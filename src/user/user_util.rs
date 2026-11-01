//! Port of: user/user_util.h
//! IR hash: 585a87f8e6aa2405
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: FilePath::FilePath (user/user_util.h:199)
/// # Safety
/// Raw pointer arguments and the receiver, when present, must be non-null and designate live memory for each generated access, with the alignment required by that operation. Any byte read must be initialized; any location written must be writable; pointer offsets must stay within the originating allocation or one past it. No other thread may perform an unsynchronized conflicting access to the same memory during this operation.
#[allow(unused_variables, non_snake_case)]
pub unsafe fn file_path_file_path_mujoco__user__filepath_ptr_79e08bc4(self_ptr: *mut mujoco__user__FilePath, arg0: *const mujoco__user__FilePath) {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe {
    extern "C" {
        fn c2rust_std_string_copy_77903c6aa15af5bc136f5cc42a00618cfcce78616b72abc2bb6f1e34fd95fbce(destination: *mut core::ffi::c_void, source: *const core::ffi::c_void);
    }
    c2rust_std_string_copy_77903c6aa15af5bc136f5cc42a00618cfcce78616b72abc2bb6f1e34fd95fbce((self_ptr as *mut u8).add(0) as *mut core::ffi::c_void, (arg0 as *const u8).add(0) as *const core::ffi::c_void);
}
}

/// C: FilePath::FilePath (user/user_util.h:199)
/// # Safety
/// Raw pointer arguments and the receiver, when present, must be non-null and designate live memory for each generated access, with the alignment required by that operation. Any byte read must be initialized; any location written must be writable; pointer offsets must stay within the originating allocation or one past it. No other thread may perform an unsynchronized conflicting access to the same memory during this operation.
#[allow(unused_variables, non_snake_case)]
pub unsafe fn file_path_file_path_mujoco__user__filepath_ptr_d3b46433(self_ptr: *mut mujoco__user__FilePath, arg0: *const mujoco__user__FilePath) {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe {
    extern "C" {
        fn c2rust_std_string_copy_77903c6aa15af5bc136f5cc42a00618cfcce78616b72abc2bb6f1e34fd95fbce(destination: *mut core::ffi::c_void, source: *const core::ffi::c_void);
    }
    c2rust_std_string_copy_77903c6aa15af5bc136f5cc42a00618cfcce78616b72abc2bb6f1e34fd95fbce((self_ptr as *mut u8).add(0) as *mut core::ffi::c_void, (arg0 as *const u8).add(0) as *const core::ffi::c_void);
}
}

