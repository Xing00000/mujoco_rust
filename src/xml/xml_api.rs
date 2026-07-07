//! Port of: xml/xml_api.h
//! IR hash: 05737965add36adb
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mj_loadXML (xml/xml_api.h:31)
#[allow(unused_variables, non_snake_case)]
pub fn mj_load_xml(filename: *const i8, vfs: *const mjVFS, error: *mut i8, error_sz: i32) -> *mut mjModel {
    // WARNING: signature changed — verify body
    // Previous params: (filename : * const i8, vfs : * const mjVFS, error : * mut i8, error_sz : i32)
    // Previous return: * mut mjModel
    todo ! ()
}

/// C: mj_saveLastXML (xml/xml_api.h:34)
/// Calls: mjCopyError
#[allow(unused_variables, non_snake_case)]
pub fn mj_save_last_xml(filename: *const i8, m: *const mjModel, error: *mut i8, error_sz: i32) -> i32 {
    extern "C" { fn mj_saveLastXML_impl(filename: *const i8, m: *const mjModel, error: *mut i8, error_sz: i32) -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { mj_saveLastXML_impl(filename, m, error, error_sz) }
}

/// C: mj_freeLastXML (xml/xml_api.h:37)
/// Calls: SetGlobalXmlSpec
#[allow(unused_variables, non_snake_case)]
pub fn mj_free_last_xml() {
    todo ! ()
}

/// C: mj_printSchema (xml/xml_api.h:40)
#[allow(unused_variables, non_snake_case)]
pub fn mj_print_schema(filename: *const i8, buffer: *mut i8, buffer_sz: i32, flg_html: i32, flg_pad: i32) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (filename : * const i8, buffer : * mut i8, buffer_sz : i32, flg_html : i32, flg_pad : i32)
    // Previous return: i32
    todo ! ()
}

/// C: mj_loadModel (xml/xml_api.h:45)
/// Calls: mj_loadModelBuffer, mju_closeResource, mju_readResource
#[allow(unused_variables, non_snake_case)]
pub fn mj_load_model(filename: *const i8, vfs: *const mjVFS) -> *mut mjModel {
    // WARNING: signature changed — verify body
    // Previous params: (filename : * const i8, vfs : * const mjVFS)
    // Previous return: * mut mjModel
    todo ! ()
}

/// C: mj_parseXML (xml/xml_api.h:48)
/// Calls: ParseXML
#[allow(unused_variables, non_snake_case)]
pub fn mj_parse_xml(filename: *const i8, vfs: *const mjVFS, error: *mut i8, error_sz: i32) -> *mut mjSpec {
    // WARNING: signature changed — verify body
    // Previous params: (filename : * const i8, vfs : * const mjVFS, error : * mut i8, error_sz : i32)
    // Previous return: * mut mjSpec
    todo ! ()
}

/// C: mj_parseXMLString (xml/xml_api.h:49)
#[allow(unused_variables, non_snake_case)]
pub fn mj_parse_xml_string(xml: *const i8, vfs: *const mjVFS, error: *mut i8, error_sz: i32) -> *mut mjSpec {
    // WARNING: signature changed — verify body
    // Previous params: (xml : * const i8, vfs : * const mjVFS, error : * mut i8, error_sz : i32)
    // Previous return: * mut mjSpec
    todo ! ()
}

/// C: mj_saveXML (xml/xml_api.h:52)
#[allow(unused_variables, non_snake_case)]
pub fn mj_save_xml(s: *const mjSpec, filename: *const i8, error: *mut i8, error_sz: i32) -> i32 {
    extern "C" { fn mj_saveXML_impl(s: *const mjSpec, filename: *const i8, error: *mut i8, error_sz: i32) -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { mj_saveXML_impl(s, filename, error, error_sz) }
}

/// C: mj_saveXMLString (xml/xml_api.h:53)
#[allow(unused_variables, non_snake_case)]
pub fn mj_save_xml_string(s: *const mjSpec, xml: *mut i8, xml_sz: i32, error: *mut i8, error_sz: i32) -> i32 {
    extern "C" { fn mj_saveXMLString_impl(s: *const mjSpec, xml: *mut i8, xml_sz: i32, error: *mut i8, error_sz: i32) -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { mj_saveXMLString_impl(s, xml, xml_sz, error, error_sz) }
}

