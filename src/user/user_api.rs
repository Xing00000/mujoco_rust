//! Port of: user/user_api.cc
//! IR hash: 24663acedb849b3c
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;


/// C: mjs_asCamera (user/user_api.cc:2084)
/// # Safety
/// Raw pointer arguments and the receiver, when present, must be non-null and designate live memory for each generated access, with the alignment required by that operation. Any byte read must be initialized; any location written must be writable; pointer offsets must stay within the originating allocation or one past it. No other thread may perform an unsynchronized conflicting access to the same memory during this operation.
#[allow(unused_variables, non_snake_case)]
pub unsafe fn mjs_as_camera(element: *mut mjsElement) -> *mut mjsCamera {
    // MACHINE-PROVEN: Clang AST semantics
    if element.is_null() { return core::ptr::null_mut() as *mut mjsCamera; }
unsafe {
let tag = core::ptr::read_unaligned(element.cast::<u8>().add(0).cast::<u32>());
if tag == 0x00000007u32 { element.cast::<u8>().offset(648) as *mut mjsCamera } else { core::ptr::null_mut() as *mut mjsCamera }
}
}

#[doc(hidden)]
#[doc = "c2rust-bitexact: generated function alias"]
pub use self::mjs_as_camera as mjs_asCamera;
