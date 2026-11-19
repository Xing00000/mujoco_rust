//! Port of: xml/mjz/mjz_encoder.cc
//! IR hash: 3e5fbc9a9a5746f6
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;


#[allow(unused_variables, non_snake_case)]
/// Hashes the three live string views in the input record.
///
/// # Safety
/// `k` must point to the live input record with live readonly character views and backing bytes for the duration of this call.
pub unsafe fn __c2rust_rewrite_key_hash_operator_8a2ddc529b6694ec3e9cb88d6e81f2e600140f17f884bd483139bc86bc40dfce(self_ptr: *mut RewriteKeyHash, k: *const anonymous_namespace___RewriteKey) -> u64 {
    extern "C" {
        fn c2rust_std_view_hash_f9ea417637ee6a60402d725bffb2b986f49eca68841d335f8122d01a689aa35d(view:*const core::ffi::c_void) -> u64;
    }
    unsafe {
        let h1 = c2rust_std_view_hash_f9ea417637ee6a60402d725bffb2b986f49eca68841d335f8122d01a689aa35d(k.cast::<u8>().add(0).cast());
        let h2 = c2rust_std_view_hash_f9ea417637ee6a60402d725bffb2b986f49eca68841d335f8122d01a689aa35d(k.cast::<u8>().add(16).cast());
        let h3 = c2rust_std_view_hash_f9ea417637ee6a60402d725bffb2b986f49eca68841d335f8122d01a689aa35d(k.cast::<u8>().add(32).cast());
        let mut seed = h1;
        seed ^= h2.wrapping_add(0x9e3779b9).wrapping_add(seed << 6).wrapping_add(seed >> 2);
        seed ^= h3.wrapping_add(0x9e3779b9).wrapping_add(seed << 6).wrapping_add(seed >> 2);
        seed
    }
}
