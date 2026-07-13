//! Port of: xml/xml_api.h
//! IR hash: e878892ca48fe222
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mj_loadXML (xml/xml_api.h:31)
/// Calls: ParseXML, SetGlobalXmlSpec, mjCopyError, mj_compile, mj_deleteSpec, mjs_getError, mjs_getWarning, mjs_numWarnings
#[allow(unused_variables, non_snake_case)]
pub fn mj_load_xml(filename: *const i8, vfs: *const mjVFS, error: *mut i8, error_sz: i32) -> *mut mjModel {
    todo!() // mj_loadXML
}

/// C: mj_saveLastXML (xml/xml_api.h:34)
/// Calls: GetGlobalXmlSpec, mjCopyError
#[allow(unused_variables, non_snake_case)]
pub fn mj_save_last_xml(filename: *const i8, m: *const mjModel, error: *mut i8, error_sz: i32) -> i32 {
    todo!() // mj_saveLastXML
}

/// C: mj_freeLastXML (xml/xml_api.h:37)
/// Calls: SetGlobalXmlSpec
#[allow(unused_variables, non_snake_case)]
pub fn mj_free_last_xml() {
    todo!() // mj_freeLastXML
}

/// C: mj_printSchema (xml/xml_api.h:40)
/// Calls: mjXReader::PrintSchema
#[allow(unused_variables, non_snake_case)]
pub fn mj_print_schema(filename: *const i8, buffer: *mut i8, buffer_sz: i32, flg_html: i32, flg_pad: i32) -> i32 {
    todo!() // mj_printSchema
}

/// C: mj_loadModel (xml/xml_api.h:45)
/// Calls: mj_loadModelBuffer, mju_closeResource, mju_openResource, mju_readResource, mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn mj_load_model(filename: *const i8, vfs: *const mjVFS) -> *mut mjModel {
    todo!() // mj_loadModel
}

/// C: mj_parseXML (xml/xml_api.h:48)
/// Calls: ParseXML
#[allow(unused_variables, non_snake_case)]
pub fn mj_parse_xml(filename: *const i8, vfs: *const mjVFS, error: *mut i8, error_sz: i32) -> *mut mjSpec {
    todo!() // mj_parseXML
}

/// C: mj_parseXMLString (xml/xml_api.h:49)
/// Calls: ParseSpecFromString
#[allow(unused_variables, non_snake_case)]
pub fn mj_parse_xml_string(xml: *const i8, vfs: *const mjVFS, error: *mut i8, error_sz: i32) -> *mut mjSpec {
    todo!() // mj_parseXMLString
}

/// C: mj_saveXML (xml/xml_api.h:52)
/// Calls: WriteXML
#[allow(unused_variables, non_snake_case)]
pub fn mj_save_xml(s: *const mjSpec, filename: *const i8, error: *mut i8, error_sz: i32) -> i32 {
    todo!() // mj_saveXML
}

/// C: mj_saveXMLString (xml/xml_api.h:53)
/// Calls: WriteXML, mjCopyError
#[allow(unused_variables, non_snake_case)]
pub fn mj_save_xml_string(s: *const mjSpec, xml: *mut i8, xml_sz: i32, error: *mut i8, error_sz: i32) -> i32 {
    todo!() // mj_saveXMLString
}

