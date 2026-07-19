//! Port of: xml/xml.cc
//! IR hash: adc2f24e872d94f7
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: LocaleOverride::PosixLocale (xml/xml.cc:85)
#[allow(unused_variables, non_snake_case)]
pub fn locale_override_posix_locale() -> *mut xlocale {
    todo!() // LocaleOverride::PosixLocale
}

/// C: IncludeXML (xml/xml.cc:102)
/// Calls: FilePath::IsAbs, FilePath::Str, FilePath::c_str, mjXReader::ModelFileDir, mjXReader::SetAssetDir, mjXReader::SetMeshDir, mjXReader::SetTextureDir, mjXUtil::ReadAttrFile, mjXUtil::ReadAttrStr, mju_closeResource, mju_getResourceDir, mju_openResource, mju_readResource, sprintf_arr
#[allow(unused_variables, non_snake_case)]
pub fn include_xml(reader: *mut mjXReader, elem: *mut anonymous_namespace___XMLElement, dir: *const anonymous_namespace___FilePath, vfs: *const mjVFS, included: *const ()) {
    todo!() // IncludeXML
}

/// C: SpecFromXML (xml/xml.cc:243)
/// Calls: IncludeXML, mjCopyError, mjXBase::SetModel, mjXReader::Parse, mjXReader::SetModelFileDir, mjXURDF::Parse, mj_deleteSpec, mj_makeSpec, mjs_getString, mjs_setString
#[allow(unused_variables, non_snake_case)]
pub fn spec_from_xml(xml: std__string_view, dir: std__string_view, filename: std__string_view, vfs: *const mjVFS, error: *mut i8, nerror: i32) -> *mut mjSpec {
    todo!() // SpecFromXML
}

/// C: ParseXML (xml/xml.h:25)
/// Calls: SpecFromXML, mju_closeResource, mju_getResourceDir, mju_openResource, mju_readResource
#[allow(unused_variables, non_snake_case)]
pub fn parse_xml(filename: *const i8, vfs: *const mjVFS, error: *mut i8, nerror: i32) -> *mut mjSpec {
    todo!() // ParseXML
}

/// C: ParseSpecFromString (xml/xml.h:29)
/// Calls: SpecFromXML
#[allow(unused_variables, non_snake_case)]
pub fn parse_spec_from_string(xml: std__string_view, vfs: *const mjVFS, error: *mut i8, nerror: i32) -> *mut mjSpec {
    todo!() // ParseSpecFromString
}

/// C: WriteXML (xml/xml.h:33)
/// Calls: mjCopyError, mjXWriter::SetModel, mjXWriter::Write
#[allow(unused_variables, non_snake_case)]
pub fn write_xml(m: *const mjModel, spec: *mut mjSpec, error: *mut i8, nerror: i32) -> std__string {
    todo!() // WriteXML
}

