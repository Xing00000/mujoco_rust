//! Port of: xml/mjz/mjz_decoder.cc
//! IR hash: fd5288ad7242f001
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: SetError (xml/mjz/mjz_decoder.cc:47)
#[allow(unused_variables, non_snake_case)]
pub fn set_error(error: *mut i8, error_sz: i32, format: *const i8) {
    todo!() // cxx-internal:mjz_decoder.cc.o:__ZL8SetErrorPciPKcz
}

/// C: ZipArchiveProvider::GetRootModelPath (xml/mjz/mjz_decoder.cc:153)
#[allow(unused_variables, non_snake_case)]
pub fn zip_archive_provider_get_root_model_path(self_ptr: *mut ZipArchiveProvider) -> std__string {
    todo!() // cxx:__ZNK18ZipArchiveProvider16GetRootModelPathEv
}

/// C: ZipArchiveProvider::Contains (xml/mjz/mjz_decoder.cc:156)
#[allow(unused_variables, non_snake_case)]
pub fn zip_archive_provider_contains(self_ptr: *mut ZipArchiveProvider, name: std__string_view) -> bool {
    todo!() // cxx:__ZNK18ZipArchiveProvider8ContainsENSt3__117basic_string_viewIcNS0_11char_traitsIcEEEE
}

/// C: ZipArchiveProvider::Read (xml/mjz/mjz_decoder.cc:172)
/// Calls: cxx:__ZNK18ZipArchiveProvider8ContainsENSt3__117basic_string_viewIcNS0_11char_traitsIcEEEE
#[allow(unused_variables, non_snake_case)]
pub fn zip_archive_provider_read(self_ptr: *mut ZipArchiveProvider, name: *const std__string) -> *const () {
    todo!() // cxx:__ZN18ZipArchiveProvider4ReadERKNSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEE
}

/// C: ParseZipBuffer (xml/mjz/mjz_decoder.cc:216)
/// Calls: cxx-internal:mjz_decoder.cc.o:__ZL8SetErrorPciPKcz, cxx:__ZNK18ZipArchiveProvider16GetRootModelPathEv, cxx:_mj_mountVFS, cxx:_mj_parseXML
#[allow(unused_variables, non_snake_case)]
pub fn parse_zip_buffer(buffer: *const (), nbuffer: i32, name: *const i8, vfs: *mut mjVFS, error: *mut i8, error_sz: i32) -> *mut mjSpec {
    todo!() // cxx-internal:mjz_decoder.cc.o:__ZL14ParseZipBufferPKviPKcP6mjVFS_Pci
}

/// C: _mj_init_mjz_decoder (xml/mjz/mjz_decoder.cc:274)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/xml/mjz/mjz_decoder.cc:__ZL14ParseZipBufferPKviPKcP6mjVFS_Pci, cxx:_mjp_registerDecoder, cxx:_mju_readResource, cxx:_mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn mj_init_mjz_decoder() {
    todo!() // cxx-internal:mjz_decoder.cc.o:__ZL20_mj_init_mjz_decoderv
}

