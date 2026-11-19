//! Port of: user/user_cache.h
//! IR hash: 123cb05b449ff99b
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mjCAsset::mjCAsset (user/user_cache.h:53)
/// # Safety
/// Raw pointer arguments and the receiver, when present, must be non-null and designate live memory for each generated access, with the alignment required by that operation. Any byte read must be initialized; any location written must be writable; pointer offsets must stay within the originating allocation or one past it. No other thread may perform an unsynchronized conflicting access to the same memory during this operation.
#[allow(unused_variables, non_snake_case)]
pub unsafe fn __c2rust_mj_c_asset_mj_c_asset_dac78337fe552a772155ca4c24d3b11259c0d7a868cdaa072cf489b61c5b1ce7(self_ptr: *mut mjCAsset, other: *mut mjCAsset) {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe {
extern "C" { fn c2rust_std_member_3dc71e31aa811d35a543fd6e2a43457212f28d73a808f65a92ff464ab4ac24ff(destination: *mut core::ffi::c_void, source: *mut core::ffi::c_void); }
extern "C" { fn c2rust_std_member_6968c10ea83e8107f2c22973ac59f34df6e76ee4dd55e4739114bbaecac8ff1d(destination: *mut core::ffi::c_void, source: *mut core::ffi::c_void); }
extern "C" { fn c2rust_std_member_87ca51cdc842df1eefa209b43bd821893c726406c2bedfa6dca423126987fa3c(destination: *mut core::ffi::c_void, source: *mut core::ffi::c_void); }
c2rust_std_member_3dc71e31aa811d35a543fd6e2a43457212f28d73a808f65a92ff464ab4ac24ff((self_ptr as *mut u8).add(0).cast(), (other as *mut u8).add(0).cast());
c2rust_std_member_3dc71e31aa811d35a543fd6e2a43457212f28d73a808f65a92ff464ab4ac24ff((self_ptr as *mut u8).add(24).cast(), (other as *mut u8).add(24).cast());
core::ptr::copy_nonoverlapping((other as *const u8).add(48), (self_ptr as *mut u8).add(48), 8);
core::ptr::copy_nonoverlapping((other as *const u8).add(56), (self_ptr as *mut u8).add(56), 8);
core::ptr::copy_nonoverlapping((other as *const u8).add(64), (self_ptr as *mut u8).add(64), 8);
c2rust_std_member_6968c10ea83e8107f2c22973ac59f34df6e76ee4dd55e4739114bbaecac8ff1d((self_ptr as *mut u8).add(72).cast(), (other as *mut u8).add(72).cast());
c2rust_std_member_87ca51cdc842df1eefa209b43bd821893c726406c2bedfa6dca423126987fa3c((self_ptr as *mut u8).add(88).cast(), (other as *mut u8).add(88).cast());
}
}

/// C: mjCAsset::mjCAsset (user/user_cache.h:53)
/// # Safety
/// Raw pointer arguments and the receiver, when present, must be non-null and designate live memory for each generated access, with the alignment required by that operation. Any byte read must be initialized; any location written must be writable; pointer offsets must stay within the originating allocation or one past it. No other thread may perform an unsynchronized conflicting access to the same memory during this operation.
#[allow(unused_variables, non_snake_case)]
pub unsafe fn __c2rust_mj_c_asset_mj_c_asset_fb973707d05dd4b66b3850759ea70051e171446c2efad46cf26819f398be31f6(self_ptr: *mut mjCAsset, other: *mut mjCAsset) {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe {
extern "C" { fn c2rust_std_member_3dc71e31aa811d35a543fd6e2a43457212f28d73a808f65a92ff464ab4ac24ff(destination: *mut core::ffi::c_void, source: *mut core::ffi::c_void); }
extern "C" { fn c2rust_std_member_6968c10ea83e8107f2c22973ac59f34df6e76ee4dd55e4739114bbaecac8ff1d(destination: *mut core::ffi::c_void, source: *mut core::ffi::c_void); }
extern "C" { fn c2rust_std_member_87ca51cdc842df1eefa209b43bd821893c726406c2bedfa6dca423126987fa3c(destination: *mut core::ffi::c_void, source: *mut core::ffi::c_void); }
c2rust_std_member_3dc71e31aa811d35a543fd6e2a43457212f28d73a808f65a92ff464ab4ac24ff((self_ptr as *mut u8).add(0).cast(), (other as *mut u8).add(0).cast());
c2rust_std_member_3dc71e31aa811d35a543fd6e2a43457212f28d73a808f65a92ff464ab4ac24ff((self_ptr as *mut u8).add(24).cast(), (other as *mut u8).add(24).cast());
core::ptr::copy_nonoverlapping((other as *const u8).add(48), (self_ptr as *mut u8).add(48), 8);
core::ptr::copy_nonoverlapping((other as *const u8).add(56), (self_ptr as *mut u8).add(56), 8);
core::ptr::copy_nonoverlapping((other as *const u8).add(64), (self_ptr as *mut u8).add(64), 8);
c2rust_std_member_6968c10ea83e8107f2c22973ac59f34df6e76ee4dd55e4739114bbaecac8ff1d((self_ptr as *mut u8).add(72).cast(), (other as *mut u8).add(72).cast());
c2rust_std_member_87ca51cdc842df1eefa209b43bd821893c726406c2bedfa6dca423126987fa3c((self_ptr as *mut u8).add(88).cast(), (other as *mut u8).add(88).cast());
}
}


pub unsafe fn __c2rust_mj_c_asset__mj_c_asset_1379749c095f535202056c63066d424a1084fb0e2ba2b71acd0a261c62bcc8fd(self_ptr: *mut mjCAsset) -> () {
    unsafe { __c2rust_mj_c_asset__mj_c_asset_2bb4769b12fda6a8277634d69d470e6bbaa53fc21c75c694f5b33bfcf7b59bda(self_ptr); }
}

pub unsafe fn __c2rust_mj_c_asset__mj_c_asset_2bb4769b12fda6a8277634d69d470e6bbaa53fc21c75c694f5b33bfcf7b59bda(self_ptr: *mut mjCAsset) -> () {
    extern "C" {
        fn c2rust_library_call_3bc2f2f28a64c5765bf73152a7112fd3b578283242b5e1a9234ce8dc9c3a2fcd(arg_0: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
        fn c2rust_library_call_4119a4c79c2dd7901b25ef4825bc1066b11abbed25c011a58dec9df393037226(arg_0: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
        fn c2rust_library_call_3946a35ea15999e79d317bf84d20ec02f429b5c0c0a05b317855aca2bd2f049e(arg_0: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    }
    let receiver = self_ptr.cast::<u8>();
    unsafe {
        let _ = c2rust_library_call_3bc2f2f28a64c5765bf73152a7112fd3b578283242b5e1a9234ce8dc9c3a2fcd(receiver.add(88).cast());
        let _ = c2rust_library_call_4119a4c79c2dd7901b25ef4825bc1066b11abbed25c011a58dec9df393037226(receiver.add(72).cast());
        let _ = c2rust_library_call_3946a35ea15999e79d317bf84d20ec02f429b5c0c0a05b317855aca2bd2f049e(receiver.add(24).cast());
        let _ = c2rust_library_call_3946a35ea15999e79d317bf84d20ec02f429b5c0c0a05b317855aca2bd2f049e(receiver.add(0).cast());
    }
}
