//! Port of: engine/engine_vis_init.c
//! IR hash: 409d973b564d7553
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;


/// C: mjv_defaultScene (engine/engine_vis_init.c:310)
/// # Safety
/// Raw pointer arguments and the receiver, when present, must be non-null and designate live memory for each generated access, with the alignment required by that operation. Any byte read must be initialized; any location written must be writable; pointer offsets must stay within the originating allocation or one past it. No other thread may perform an unsynchronized conflicting access to the same memory during this operation.
#[allow(unused_variables, non_snake_case)]
pub unsafe fn __c2rust_mjv_default_scene_e38cae16e6b64bda6588d51bbfecb48c85c77d48f80ae8af68f6ced734054811(scn: *mut mjvScene) {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { core::ptr::write_bytes(scn.cast::<u8>(), 0u8, 11584usize); }
}

#[doc(hidden)]
#[doc = "c2rust-bitexact: generated function alias"]
pub use self::__c2rust_mjv_default_scene_e38cae16e6b64bda6588d51bbfecb48c85c77d48f80ae8af68f6ced734054811 as mjv_defaultScene;
