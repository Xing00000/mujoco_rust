//! Port of: xml/mjz/mjz_decoder.cc
//! IR hash: 05737965add36adb
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: SetError (xml/mjz/mjz_decoder.cc:46)
#[allow(unused_variables, non_snake_case)]
pub fn set_error(error: *mut i8, error_sz: i32, format: *const i8) {
    if error.is_null() { return; }
    extern "C" { fn SetError(error: *mut i8, error_sz: i32, format: *const i8); }
    // SAFETY: error verified non-null
    unsafe { SetError(error, error_sz, format) }
}

/// C: ZipArchiveProvider::GetRootModelPath (xml/mjz/mjz_decoder.cc:137)
#[allow(unused_variables, non_snake_case)]
pub fn zip_archive_provider_get_root_model_path(self_ptr: *mut ZipArchiveProvider) -> std__string {
    extern "C" { fn ZipArchiveProvider_GetRootModelPath(self_ptr: *mut ZipArchiveProvider) -> std__string; }
    // SAFETY: delegates to C implementation
    unsafe { ZipArchiveProvider_GetRootModelPath(self_ptr) }
}

/// C: ZipArchiveProvider::Contains (xml/mjz/mjz_decoder.cc:142)
#[allow(unused_variables, non_snake_case)]
pub fn zip_archive_provider_contains(self_ptr: *mut ZipArchiveProvider, name: string_view) -> bool {
    extern "C" { fn ZipArchiveProvider_Contains(self_ptr: *mut ZipArchiveProvider, name: string_view) -> bool; }
    // SAFETY: delegates to C implementation
    unsafe { ZipArchiveProvider_Contains(self_ptr, name) }
}

/// C: ZipArchiveProvider::Read (xml/mjz/mjz_decoder.cc:155)
#[allow(unused_variables, non_snake_case)]
pub fn zip_archive_provider_read(self_ptr: *mut ZipArchiveProvider, name: *const std__string) -> i32 {
    extern "C" { fn ZipArchiveProvider_Read(self_ptr: *mut ZipArchiveProvider, name: *const std__string) -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { ZipArchiveProvider_Read(self_ptr, name) }
}

/// C: ParseZipBuffer (xml/mjz/mjz_decoder.cc:198)
/// Calls: SetError
#[allow(unused_variables, non_snake_case)]
pub fn parse_zip_buffer(buffer: *const (), nbuffer: i32, name: *const i8, vfs: *mut mjVFS, error: *mut i8, error_sz: i32) -> *mut mjSpec {
    extern "C" { fn ParseZipBuffer(buffer: *const (), nbuffer: i32, name: *const i8, vfs: *mut mjVFS, error: *mut i8, error_sz: i32) -> *mut mjSpec; }
    // SAFETY: delegates to C++ implementation; caller guarantees pointer validity
    unsafe { ParseZipBuffer(buffer, nbuffer, name, vfs, error, error_sz) }
}

/// C: _mj_init_mjz_decoder (xml/mjz/mjz_decoder.cc:429)
/// Calls: ParseZipBuffer, mjp_registerDecoder, mju_readResource
#[allow(unused_variables, non_snake_case)]
pub fn mj_init_mjz_decoder() {
    extern "C" { fn _mj_init_mjz_decoder(); }
    // SAFETY: delegates to C++ implementation; caller guarantees pointer validity
    unsafe { _mj_init_mjz_decoder() }
}

