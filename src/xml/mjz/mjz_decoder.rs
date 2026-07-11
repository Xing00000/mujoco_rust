//! Port of: xml/mjz/mjz_decoder.cc
//! IR hash: c6d98e4f4b63b7f2
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: SetError (xml/mjz/mjz_decoder.cc:46)
#[allow(unused_variables, non_snake_case)]
pub fn set_error(error: *mut i8, error_sz: i32, format: *const i8) {
    // WARNING: signature changed — verify body
    // Previous params: (error : * mut i8, error_sz : i32, format : * const i8)
    // Previous return: ()
    todo ! ()
}

/// C: ZipArchiveProvider::GetRootModelPath (xml/mjz/mjz_decoder.cc:137)
#[allow(unused_variables, non_snake_case)]
pub fn zip_archive_provider_get_root_model_path(self_ptr: *mut ZipArchiveProvider) -> std__string {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut ZipArchiveProvider)
    // Previous return: std__string
    todo ! ()
}

/// C: ZipArchiveProvider::Contains (xml/mjz/mjz_decoder.cc:142)
#[allow(unused_variables, non_snake_case)]
pub fn zip_archive_provider_contains(self_ptr: *mut ZipArchiveProvider, name: string_view) -> bool {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut ZipArchiveProvider, name : string_view)
    // Previous return: bool
    todo ! ()
}

/// C: ZipArchiveProvider::Read (xml/mjz/mjz_decoder.cc:155)
#[allow(unused_variables, non_snake_case)]
pub fn zip_archive_provider_read(self_ptr: *mut ZipArchiveProvider, name: *const std__string) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut ZipArchiveProvider, name : * const std__string)
    // Previous return: i32
    todo ! ()
}

/// C: ParseZipBuffer (xml/mjz/mjz_decoder.cc:198)
/// Calls: SetError
#[allow(unused_variables, non_snake_case)]
pub fn parse_zip_buffer(buffer: *const (), nbuffer: i32, name: *const i8, vfs: *mut mjVFS, error: *mut i8, error_sz: i32) -> *mut mjSpec {
    // WARNING: signature changed — verify body
    // Previous params: (buffer : * const (), nbuffer : i32, name : * const i8, vfs : * mut mjVFS, error : * mut i8, error_sz : i32)
    // Previous return: * mut mjSpec
    todo ! ()
}

/// C: _mj_init_mjz_decoder (xml/mjz/mjz_decoder.cc:429)
/// Calls: ParseZipBuffer, mjp_registerDecoder, mju_readResource
#[allow(unused_variables, non_snake_case)]
pub fn mj_init_mjz_decoder() {
    todo ! ()
}

