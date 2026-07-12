//! Port of: xml/xml.cc
//! IR hash: c6d98e4f4b63b7f2
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: LocaleOverride::PosixLocale (xml/xml.cc:85)
#[allow(unused_variables, non_snake_case)]
pub fn locale_override_posix_locale() -> i32 {
    todo ! ()
}

/// C: IncludeXML (xml/xml.cc:102)
/// Calls: FilePath::IsAbs, FilePath::Str, FilePath::c_str, mjXReader::ModelFileDir, mjXReader::SetAssetDir, mjXReader::SetMeshDir, mjXReader::SetTextureDir, mju_closeResource, mju_getResourceDir, mju_readResource
#[allow(unused_variables, non_snake_case)]
pub fn include_xml(reader: *mut mjXReader, elem: *mut XMLElement, dir: *const FilePath, vfs: *const mjVFS, included: *mut i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (reader : * mut mjXReader, elem : * mut XMLElement, dir : * const FilePath, vfs : * const mjVFS, included : * mut i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: SpecFromXML (xml/xml.cc:243)
/// Calls: mjCopyError, mjXReader::Parse, mjXURDF::Parse, mj_deleteSpec, mj_makeSpec, mjs_getString
#[allow(unused_variables, non_snake_case)]
pub fn spec_from_xml(xml: string_view, dir: string_view, filename: string_view, vfs: *const mjVFS, error: *mut i8, nerror: i32) -> *mut mjSpec {
    // NOTE: signature changed from previous IR version
    // Previous params: (xml : string_view, dir : string_view, filename : string_view, vfs : * const mjVFS, error : * mut i8, nerror : i32)
    // Previous return: * mut mjSpec
    todo!("re-translate: params renamed")
}

/// C: ParseXML (xml/xml.h:25)
/// Calls: mju_closeResource, mju_getResourceDir, mju_readResource
#[allow(unused_variables, non_snake_case)]
pub fn parse_xml(filename: *const i8, vfs: *const mjVFS, error: *mut i8, nerror: i32) -> *mut mjSpec {
    // NOTE: signature changed from previous IR version
    // Previous params: (filename : * const i8, vfs : * const mjVFS, error : * mut i8, nerror : i32)
    // Previous return: * mut mjSpec
    todo!("re-translate: params renamed")
}

/// C: ParseSpecFromString (xml/xml.h:29)
#[allow(unused_variables, non_snake_case)]
pub fn parse_spec_from_string(xml: string_view, vfs: *const mjVFS, error: *mut i8, nerror: i32) -> *mut mjSpec {
    // NOTE: signature changed from previous IR version
    // Previous params: (xml : string_view, vfs : * const mjVFS, error : * mut i8, nerror : i32)
    // Previous return: * mut mjSpec
    todo!("re-translate: params renamed")
}

/// C: WriteXML (xml/xml.h:33)
/// Calls: mjCopyError, mjXWriter::SetModel
#[allow(unused_variables, non_snake_case)]
pub fn write_xml(m: *const mjModel, spec: *mut mjSpec, error: *mut i8, nerror: i32) -> std__string {
    // NOTE: signature changed from previous IR version
    // Previous params: (m : * const mjModel, spec : * mut mjSpec, error : * mut i8, nerror : i32)
    // Previous return: std__string
    todo!("re-translate: params renamed")
}

