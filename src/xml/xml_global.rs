//! Port of: xml/xml_global.cc
//! IR hash: 6ff71909dacce27f
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: GlobalModel::Set (xml/xml_global.cc:32)
/// Calls: GetGlobalModel, mj_deleteSpec
#[allow(unused_variables, non_snake_case)]
pub fn global_model_set(self_ptr: *mut GlobalModel, spec: *mut mjSpec) {
    todo!() // GlobalModel::Set
}

/// C: GlobalModel::ToXML (xml/xml_global.cc:35)
/// Calls: mjCopyError
#[allow(unused_variables, non_snake_case)]
pub fn global_model_to_xml(self_ptr: *mut GlobalModel, m: *const mjModel, error: *mut i8, error_sz: i32) -> std__string {
    todo!() // GlobalModel::ToXML
}

/// C: GetGlobalModel (xml/xml_global.cc:53)
#[allow(unused_variables, non_snake_case)]
pub fn get_global_model() -> *mut GlobalModel {
    todo!() // GetGlobalModel
}

/// C: SetGlobalXmlSpec (xml/xml_global.h:23)
/// Calls: GetGlobalModel, GlobalModel::Set
#[allow(unused_variables, non_snake_case)]
pub fn set_global_xml_spec(spec: *mut mjSpec) {
    todo!() // SetGlobalXmlSpec
}

/// C: GetGlobalXmlSpec (xml/xml_global.h:25)
/// Calls: GetGlobalModel
#[allow(unused_variables, non_snake_case)]
pub fn get_global_xml_spec(m: *const mjModel, error: *mut i8, error_sz: i32) -> std__string {
    todo!() // GetGlobalXmlSpec
}

