//! Port of: xml/xml_api.cc
//! IR hash: fd5288ad7242f001
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mj_loadXML (xml/xml_api.cc:40)
/// Calls: cxx:__Z11mjCopyErrorPcPKci, cxx:__Z16SetGlobalXmlSpecP7mjSpec_, cxx:__Z8ParseXMLPKcPK6mjVFS_Pci, cxx:_mj_compile, cxx:_mj_deleteSpec, cxx:_mjs_getError, cxx:_mjs_getWarning, cxx:_mjs_numWarnings
#[allow(unused_variables, non_snake_case)]
pub fn mj_load_xml(filename: *const i8, vfs: *const mjVFS, error: *mut i8, error_sz: i32) -> *mut mjModel {
    todo!() // cxx:_mj_loadXML
}

/// C: mj_saveLastXML (xml/xml_api.cc:76)
/// Calls: cxx:__Z11mjCopyErrorPcPKci, cxx:__Z16GetGlobalXmlSpecPK8mjModel_Pci
#[allow(unused_variables, non_snake_case)]
pub fn mj_save_last_xml(filename: *const i8, m: *const mjModel, error: *mut i8, error_sz: i32) -> i32 {
    todo!() // cxx:_mj_saveLastXML
}

/// C: mj_freeLastXML (xml/xml_api.cc:96)
/// Calls: cxx:__Z16SetGlobalXmlSpecP7mjSpec_
#[allow(unused_variables, non_snake_case)]
pub fn mj_free_last_xml() {
    todo!() // cxx:_mj_freeLastXML
}

/// C: mj_printSchema (xml/xml_api.cc:102)
/// Calls: cxx:__ZN9mjXReader11PrintSchemaERNSt3__118basic_stringstreamIcNS0_11char_traitsIcEENS0_9allocatorIcEEEEbb
#[allow(unused_variables, non_snake_case)]
pub fn mj_print_schema(filename: *const i8, buffer: *mut i8, buffer_sz: i32, flg_html: i32, flg_pad: i32) -> i32 {
    todo!() // cxx:_mj_printSchema
}

/// C: mj_loadModel (xml/xml_api.cc:128)
/// Calls: cxx:_mj_loadModelBuffer, cxx:_mju_closeResource, cxx:_mju_openResource, cxx:_mju_readResource, cxx:_mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn mj_load_model(filename: *const i8, vfs: *const mjVFS) -> *mut mjModel {
    todo!() // cxx:_mj_loadModel
}

/// C: mj_parseXML (xml/xml_api.cc:151)
/// Calls: cxx:__Z8ParseXMLPKcPK6mjVFS_Pci
#[allow(unused_variables, non_snake_case)]
pub fn mj_parse_xml(filename: *const i8, vfs: *const mjVFS, error: *mut i8, error_sz: i32) -> *mut mjSpec {
    todo!() // cxx:_mj_parseXML
}

/// C: mj_parseXMLString (xml/xml_api.cc:157)
/// Calls: cxx:__Z19ParseSpecFromStringNSt3__117basic_string_viewIcNS_11char_traitsIcEEEEPK6mjVFS_Pci
#[allow(unused_variables, non_snake_case)]
pub fn mj_parse_xml_string(xml: *const i8, vfs: *const mjVFS, error: *mut i8, error_sz: i32) -> *mut mjSpec {
    todo!() // cxx:_mj_parseXMLString
}

/// C: mj_saveXML (xml/xml_api.cc:163)
/// Calls: cxx:__Z8WriteXMLPK8mjModel_P7mjSpec_Pci, cxx:_mju_writeResource
#[allow(unused_variables, non_snake_case)]
pub fn mj_save_xml(s: *const mjSpec, filename: *const i8, error: *mut i8, error_sz: i32) -> i32 {
    todo!() // cxx:_mj_saveXML
}

/// C: mj_saveXMLString (xml/xml_api.cc:182)
/// Calls: cxx:__Z11mjCopyErrorPcPKci, cxx:__Z8WriteXMLPK8mjModel_P7mjSpec_Pci
#[allow(unused_variables, non_snake_case)]
pub fn mj_save_xml_string(s: *const mjSpec, xml: *mut i8, xml_sz: i32, error: *mut i8, error_sz: i32) -> i32 {
    todo!() // cxx:_mj_saveXMLString
}

