//! Port of: xml/mjz/mjz_decoder.cc
//! IR hash: 1b139f44af8230f9
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: SetError (xml/mjz/mjz_decoder.cc:46)
#[allow(unused_variables, non_snake_case)]
pub fn set_error(error: *mut i8, error_sz: i32, format: *const i8) {
    todo!() // SetError
}

/// C: ZipArchiveProvider::GetRootModelPath (xml/mjz/mjz_decoder.cc:137)
#[allow(unused_variables, non_snake_case)]
pub fn zip_archive_provider_get_root_model_path() -> i32 {
    todo!() // ZipArchiveProvider::GetRootModelPath
}

/// C: _mj_init_mjz_decoder (xml/mjz/mjz_decoder.cc:138)
/// Calls: ParseZipBuffer, mjp_registerDecoder, mju_readResource
#[allow(unused_variables, non_snake_case)]
pub fn mj_init_mjz_decoder() {
    todo!() // _mj_init_mjz_decoder
}

/// C: ZipArchiveProvider::Contains (xml/mjz/mjz_decoder.cc:142)
#[allow(unused_variables, non_snake_case)]
pub fn zip_archive_provider_contains(name: std__string_view) -> bool {
    todo!() // ZipArchiveProvider::Contains
}

/// C: ZipArchiveProvider::Read (xml/mjz/mjz_decoder.cc:155)
#[allow(unused_variables, non_snake_case)]
pub fn zip_archive_provider_read(name: *const i32) -> i32 {
    todo!() // ZipArchiveProvider::Read
}

/// C: ParseZipBuffer (xml/mjz/mjz_decoder.cc:198)
/// Calls: SetError
#[allow(unused_variables, non_snake_case)]
pub fn parse_zip_buffer(buffer: *const (), nbuffer: i32, name: *const i8, vfs: *mut mjVFS, error: *mut i8, error_sz: i32) -> *mut mjSpec {
    todo!() // ParseZipBuffer
}

