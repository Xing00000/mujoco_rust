//! Port of: xml/xml_global.cc
//! IR hash: 545f394232195ad9
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: GlobalModel::Set (xml/xml_global.cc:32)
/// Calls: GetGlobalModel, mj_deleteSpec
#[allow(unused_variables, non_snake_case)]
pub fn global_model_set(self_ptr: *mut GlobalModel, spec: *mut mjSpec) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut GlobalModel, spec : * mut mjSpec)
    // Previous return: ()
    todo ! ()
}

/// C: GlobalModel::ToXML (xml/xml_global.cc:35)
/// Calls: mjCopyError
#[allow(unused_variables, non_snake_case)]
pub fn global_model_to_xml(self_ptr: *mut GlobalModel, m: *const mjModel, error: *mut i8, error_sz: i32) -> std__string {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut GlobalModel, m : * const mjModel, error : * mut i8, error_sz : i32)
    // Previous return: std__string
    todo ! ()
}

/// C: GetGlobalModel (xml/xml_global.cc:53)
#[allow(unused_variables, non_snake_case)]
pub fn get_global_model() -> *mut GlobalModel {
    // WARNING: signature changed — verify body
    // Previous params: ()
    // Previous return: * mut GlobalModel
    todo ! ()
}

/// C: SetGlobalXmlSpec (xml/xml_global.h:23)
/// Calls: GetGlobalModel, GlobalModel::Set
#[allow(unused_variables, non_snake_case)]
pub fn set_global_xml_spec(spec: *mut mjSpec) {
    // WARNING: signature changed — verify body
    // Previous params: (spec : * mut mjSpec)
    // Previous return: ()
    todo ! ()
}

/// C: GetGlobalXmlSpec (xml/xml_global.h:25)
/// Calls: GetGlobalModel
#[allow(unused_variables, non_snake_case)]
pub fn get_global_xml_spec(m: *const mjModel, error: *mut i8, error_sz: i32) -> std__string {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, error : * mut i8, error_sz : i32)
    // Previous return: std__string
    todo ! ()
}

