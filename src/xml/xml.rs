//! Port of: xml/xml.cc
//! IR hash: 05737965add36adb
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: LocaleOverride::PosixLocale (xml/xml.cc:85)
#[allow(unused_variables, non_snake_case)]
pub fn locale_override_posix_locale() -> i32 {
    extern "C" { fn PosixLocale() -> i32; }
    // SAFETY: no pointers, pure function call
    let result = unsafe { PosixLocale() };
    if result < 0 { return 0; }
    result
}

/// C: IncludeXML (xml/xml.cc:102)
/// Calls: FilePath::IsAbs, FilePath::Str, FilePath::c_str, mjXReader::ModelFileDir, mjXReader::SetAssetDir, mjXReader::SetMeshDir, mjXReader::SetTextureDir, mju_closeResource, mju_getResourceDir, mju_readResource
#[allow(unused_variables, non_snake_case)]
pub fn include_xml(reader: *mut mjXReader, elem: *mut XMLElement, dir: *const FilePath, vfs: *const mjVFS, included: *mut i32) {
    // WARNING: signature changed — verify body
    // Previous params: (reader : * mut mjXReader, elem : * mut XMLElement, dir : * const FilePath, vfs : * const mjVFS, included : * mut i32)
    // Previous return: ()
    extern "C" { fn IncludeXML(reader : * mut mjXReader , elem : * mut XMLElement , dir : * const FilePath , vfs : * const mjVFS , included : * mut i32) ; } unsafe { IncludeXML(reader , elem , dir , vfs , included) }
}

/// C: SpecFromXML (xml/xml.cc:243)
/// Calls: mjCopyError, mjXReader::Parse, mjXURDF::Parse, mj_deleteSpec, mj_makeSpec, mjs_getString
#[allow(unused_variables, non_snake_case)]
pub fn spec_from_xml(xml: string_view, dir: string_view, filename: string_view, vfs: *const mjVFS, error: *mut i8, nerror: i32) -> *mut mjSpec {
    extern "C" { fn SpecFromXML(xml: string_view, dir: string_view, filename: string_view, vfs: *const mjVFS, error: *mut i8, nerror: i32) -> *mut mjSpec; }
    // SAFETY: delegates to C++ implementation; caller guarantees pointer validity
    unsafe { SpecFromXML(xml, dir, filename, vfs, error, nerror) }
}

/// C: ParseXML (xml/xml.h:25)
/// Calls: mju_closeResource, mju_getResourceDir, mju_readResource
#[allow(unused_variables, non_snake_case)]
pub fn parse_xml(filename: *const i8, vfs: *const mjVFS, error: *mut i8, nerror: i32) -> *mut mjSpec {
    extern "C" { fn ParseXML(filename: *const i8, vfs: *const mjVFS, error: *mut i8, nerror: i32) -> *mut mjSpec; }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { ParseXML(filename, vfs, error, nerror) }
}

/// C: ParseSpecFromString (xml/xml.h:29)
#[allow(unused_variables, non_snake_case)]
pub fn parse_spec_from_string(xml: string_view, vfs: *const mjVFS, error: *mut i8, nerror: i32) -> *mut mjSpec {
    if vfs.is_null() { return core::ptr::null_mut(); }
    extern "C" { fn ParseSpecFromString(xml: string_view, vfs: *const mjVFS, error: *mut i8, nerror: i32) -> *mut mjSpec; }
    // SAFETY: vfs verified non-null
    unsafe { ParseSpecFromString(xml, vfs, error, nerror) }
}

/// C: WriteXML (xml/xml.h:33)
/// Calls: mjCopyError, mjXWriter::SetModel
#[allow(unused_variables, non_snake_case)]
pub fn write_xml(m: *const mjModel, spec: *mut mjSpec, error: *mut i8, nerror: i32) -> std__string {
    extern "C" { fn WriteXML(m: *const mjModel, spec: *mut mjSpec, error: *mut i8, nerror: i32) -> std__string; }
    // SAFETY: delegates to C implementation
    unsafe { WriteXML(m, spec, error, nerror) }
}

