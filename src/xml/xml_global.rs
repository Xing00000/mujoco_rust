//! Port of: xml/xml_global.cc
//! IR hash: 1b139f44af8230f9
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: GlobalModel::Set (xml/xml_global.cc:32)
/// Calls: GetGlobalModel, mj_deleteSpec
#[allow(unused_variables, non_snake_case)]
pub fn global_model_set(spec: *mut mjSpec) {
    todo!() // GlobalModel::Set
}

/// C: GlobalModel::ToXML (xml/xml_global.cc:43)
/// Calls: mjCopyError
#[allow(unused_variables, non_snake_case)]
pub fn global_model_to_xml(m: *const mjModel, error: *mut i8, error_sz: i32) -> i32 {
    todo!() // GlobalModel::ToXML
}

/// C: GetGlobalModel (xml/xml_global.cc:53)
#[allow(unused_variables, non_snake_case)]
pub fn get_global_model() -> *mut GlobalModel {
    todo!() // GetGlobalModel
}

/// C: GetGlobalXmlSpec (xml/xml_global.cc:84)
/// Calls: GetGlobalModel
#[allow(unused_variables, non_snake_case)]
pub fn get_global_xml_spec(m: *const mjModel, error: *mut i8, error_sz: i32) -> i32 {
    todo!() // GetGlobalXmlSpec
}

/// C: SetGlobalXmlSpec (xml/xml_global.h:23)
/// Calls: GetGlobalModel, GlobalModel::Set
#[allow(unused_variables, non_snake_case)]
pub fn set_global_xml_spec(spec: *mut mjSpec) {
    todo!() // SetGlobalXmlSpec
}

