//! Port of: xml/xml_api.h
//! IR hash: 05737965add36adb
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mj_loadXML (xml/xml_api.h:31)
#[allow(unused_variables, non_snake_case)]
pub fn mj_load_xml(filename: *const i8, vfs: *const mjVFS, error: *mut i8, error_sz: i32) -> *mut mjModel {
    extern "C" { fn mj_loadXML(filename: *const i8, vfs: *const mjVFS, error: *mut i8, error_sz: i32) -> *mut mjModel; }
    // SAFETY: delegates to C implementation
    unsafe { mj_loadXML(filename, vfs, error, error_sz) }
}

/// C: mj_saveLastXML (xml/xml_api.h:34)
/// Calls: mjCopyError
#[allow(unused_variables, non_snake_case)]
pub fn mj_save_last_xml(filename: *const i8, m: *const mjModel, error: *mut i8, error_sz: i32) -> i32 {
    // Validate: filename must not be null (C++ would segfault on std::string(nullptr))
    if filename.is_null() {
        if !error.is_null() && error_sz > 0 {
            // SAFETY: caller guarantees error buffer is valid for error_sz bytes
            unsafe { *error = 0; }
        }
        return 0;
    }
    extern "C" { fn mj_saveLastXML(filename: *const i8, m: *const mjModel, error: *mut i8, error_sz: i32) -> i32; }
    // SAFETY: filename verified non-null; m, error, error_sz passed through per caller contract
    unsafe { mj_saveLastXML(filename, m, error, error_sz) }
}

/// C: mj_freeLastXML (xml/xml_api.h:37)
/// Calls: SetGlobalXmlSpec
#[allow(unused_variables, non_snake_case)]
pub fn mj_free_last_xml() {
    extern "C" { fn mj_freeLastXML(); }
    // SAFETY: delegates to C++ implementation which calls SetGlobalXmlSpec()
    unsafe { mj_freeLastXML() }
}

/// C: mj_printSchema (xml/xml_api.h:40)
#[allow(unused_variables, non_snake_case)]
pub fn mj_print_schema(filename: *const i8, buffer: *mut i8, buffer_sz: i32, flg_html: i32, flg_pad: i32) -> i32 {
    // Validate: if buffer is provided, buffer_sz must be positive
    if !buffer.is_null() && buffer_sz <= 0 {
        return 0;
    }
    extern "C" { fn mj_printSchema(filename: *const i8, buffer: *mut i8, buffer_sz: i32, flg_html: i32, flg_pad: i32) -> i32; }
    // SAFETY: buffer_sz validated positive when buffer non-null; C++ uses std::stringstream internally
    unsafe { mj_printSchema(filename, buffer, buffer_sz, flg_html, flg_pad) }
}

/// C: mj_loadModel (xml/xml_api.h:45)
/// Calls: mj_loadModelBuffer, mju_closeResource, mju_readResource
#[allow(unused_variables, non_snake_case)]
pub fn mj_load_model(filename: *const i8, vfs: *const mjVFS) -> *mut mjModel {
    extern "C" { fn mj_loadModel(filename: *const i8, vfs: *const mjVFS) -> *mut mjModel; }
    // SAFETY: delegates to C implementation
    unsafe { mj_loadModel(filename, vfs) }
}

/// C: mj_parseXML (xml/xml_api.h:48)
/// Calls: ParseXML
#[allow(unused_variables, non_snake_case)]
pub fn mj_parse_xml(filename: *const i8, vfs: *const mjVFS, error: *mut i8, error_sz: i32) -> *mut mjSpec {
    extern "C" { fn mj_parseXML(filename: *const i8, vfs: *const mjVFS, error: *mut i8, error_sz: i32) -> *mut mjSpec; }
    // SAFETY: delegates to C implementation
    unsafe { mj_parseXML(filename, vfs, error, error_sz) }
}

/// C: mj_parseXMLString (xml/xml_api.h:49)
#[allow(unused_variables, non_snake_case)]
pub fn mj_parse_xml_string(xml: *const i8, vfs: *const mjVFS, error: *mut i8, error_sz: i32) -> *mut mjSpec {
    // Validate: xml string must not be null (C++ ParseSpecFromString dereferences it)
    if xml.is_null() {
        if !error.is_null() && error_sz > 0 {
            // SAFETY: caller guarantees error buffer is valid for error_sz bytes
            unsafe { *error = 0; }
        }
        return core::ptr::null_mut();
    }
    extern "C" { fn mj_parseXMLString(xml: *const i8, vfs: *const mjVFS, error: *mut i8, error_sz: i32) -> *mut mjSpec; }
    // SAFETY: xml verified non-null; vfs may be null (optional in C API); error buffer passed through
    unsafe { mj_parseXMLString(xml, vfs, error, error_sz) }
}

/// C: mj_saveXML (xml/xml_api.h:52)
#[allow(unused_variables, non_snake_case)]
pub fn mj_save_xml(s: *const mjSpec, filename: *const i8, error: *mut i8, error_sz: i32) -> i32 {
    // Validate: error buffer size must be non-negative; clamp to 0 if negative
    let error_sz = if error_sz < 0 { 0 } else { error_sz };
    extern "C" { fn mj_saveXML(s: *const mjSpec, filename: *const i8, error: *mut i8, error_sz: i32) -> i32; }
    // SAFETY: error_sz validated non-negative; C++ uses WriteXML + std::ofstream internally
    unsafe { mj_saveXML(s, filename, error, error_sz) }
}

/// C: mj_saveXMLString (xml/xml_api.h:53)
#[allow(unused_variables, non_snake_case)]
pub fn mj_save_xml_string(s: *const mjSpec, xml: *mut i8, xml_sz: i32, error: *mut i8, error_sz: i32) -> i32 {
    // Validate: output buffer must have positive size to be useful
    if !xml.is_null() && xml_sz <= 0 {
        return 0;
    }
    extern "C" { fn mj_saveXMLString(s: *const mjSpec, xml: *mut i8, xml_sz: i32, error: *mut i8, error_sz: i32) -> i32; }
    // SAFETY: xml_sz validated positive when xml non-null; C++ WriteXML writes to std::string then copies
    unsafe { mj_saveXMLString(s, xml, xml_sz, error, error_sz) }
}

