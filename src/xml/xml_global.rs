//! Port of: xml/xml_global.cc
//! IR hash: 05737965add36adb
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: GlobalModel::Set (xml/xml_global.cc:32)
/// Calls: GetGlobalModel, mj_deleteSpec
#[allow(unused_variables, non_snake_case)]
pub fn global_model_set(self_ptr: *mut GlobalModel, spec: *mut mjSpec) {
    if self_ptr.is_null() {
        return;
    }
    extern "C" { fn GlobalModel_Set(self_ptr: *mut GlobalModel, spec: *mut mjSpec); }
    unsafe { GlobalModel_Set(self_ptr, spec) }
}

/// C: GlobalModel::ToXML (xml/xml_global.cc:35)
/// Calls: mjCopyError
#[allow(unused_variables, non_snake_case)]
pub fn global_model_to_xml(self_ptr: *mut GlobalModel, m: *const mjModel, error: *mut i8, error_sz: i32) -> std__string {
    if self_ptr.is_null() {
        return unsafe { core::mem::zeroed() };
    }
    extern "C" { fn GlobalModelToXml(self_ptr: *mut GlobalModel, m: *const mjModel, error: *mut i8, error_sz: i32) -> std__string; }
    unsafe { GlobalModelToXml(self_ptr, m, error, error_sz) }
}

/// C: GetGlobalModel (xml/xml_global.cc:53)
#[allow(unused_variables, non_snake_case)]
pub fn get_global_model() -> *mut GlobalModel {
    extern "C" { fn GetGlobalModel() -> *mut GlobalModel; }
    let result = unsafe { GetGlobalModel() };
    // singleton should never be null; return as-is
    result
}

/// C: SetGlobalXmlSpec (xml/xml_global.h:23)
/// Calls: GetGlobalModel, GlobalModel::Set
#[allow(unused_variables, non_snake_case)]
pub fn set_global_xml_spec(spec: *mut mjSpec) {
    extern "C" { fn SetGlobalXmlSpec(spec: *mut mjSpec); }
    // SAFETY: delegates to C implementation
    unsafe { SetGlobalXmlSpec(spec) }
}

/// C: GetGlobalXmlSpec (xml/xml_global.h:25)
/// Calls: GetGlobalModel
#[allow(unused_variables, non_snake_case)]
pub fn get_global_xml_spec(m: *const mjModel, error: *mut i8, error_sz: i32) -> std__string {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, error : * mut i8, error_sz : i32)
    // Previous return: std__string
    extern "C" { fn GetGlobalXmlSpec(m : * const mjModel , error : * mut i8 , error_sz : i32) -> std__string ; } unsafe { GetGlobalXmlSpec(m , error , error_sz) }
}

