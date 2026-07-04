//! Port of: xml/xml.cc
//! IR hash: 1b139f44af8230f9
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: LocaleOverride::PosixLocale (xml/xml.cc:85)
#[allow(unused_variables, non_snake_case)]
pub fn locale_override_posix_locale() -> i32 {
    todo!() // LocaleOverride::PosixLocale
}

/// C: IncludeXML (xml/xml.cc:102)
/// Calls: FilePath::IsAbs, FilePath::c_str, mjXReader::ModelFileDir, mju_closeResource, mju_getResourceDir, mju_readResource
#[allow(unused_variables, non_snake_case)]
pub fn include_xml(reader: *mut mjXReader, elem: *mut XMLElement, dir: *const FilePath, vfs: *const mjVFS, included: *mut i32) {
    todo!() // IncludeXML
}

/// C: SpecFromXML (xml/xml.cc:243)
/// Calls: mjCopyError, mjXReader::Parse, mjXURDF::Parse, mj_deleteSpec, mj_makeSpec
#[allow(unused_variables, non_snake_case)]
pub fn spec_from_xml(xml: std__string_view, dir: std__string_view, filename: std__string_view, vfs: *const mjVFS, error: *mut i8, nerror: i32) -> *mut mjSpec {
    todo!() // SpecFromXML
}

/// C: WriteXML (xml/xml.cc:376)
/// Calls: mjCopyError, mjXWriter::SetModel
#[allow(unused_variables, non_snake_case)]
pub fn write_xml(m: *const mjModel, spec: *mut mjSpec, error: *mut i8, nerror: i32) -> i32 {
    todo!() // WriteXML
}

/// C: ParseXML (xml/xml.h:25)
/// Calls: mju_closeResource, mju_getResourceDir, mju_readResource
#[allow(unused_variables, non_snake_case)]
pub fn parse_xml(filename: *const i8, vfs: *const mjVFS, error: *mut i8, nerror: i32) -> *mut mjSpec {
    todo!() // ParseXML
}

/// C: ParseSpecFromString (xml/xml.h:29)
#[allow(unused_variables, non_snake_case)]
pub fn parse_spec_from_string(xml: std__string_view, vfs: *const mjVFS, error: *mut i8, nerror: i32) -> *mut mjSpec {
    todo!() // ParseSpecFromString
}

