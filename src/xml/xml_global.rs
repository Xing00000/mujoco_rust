//! Port of: xml/xml_global.cc
//! IR hash: 545f394232195ad9
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: GlobalModel::Set (xml/xml_global.cc:32)
/// Calls: GetGlobalModel, mj_deleteSpec
#[allow(unused_variables, non_snake_case)]
pub fn global_model_set(self_ptr: *mut GlobalModel, spec: *mut mjSpec) {
    extern "C" {
        fn GlobalModel_Set_impl(self_ptr: *mut GlobalModel, spec: *mut mjSpec);
    }
    // SAFETY: Forwarding to linked C/C++ implementation.
    unsafe { GlobalModel_Set_impl(self_ptr, spec) }
}

/// C: GlobalModel::ToXML (xml/xml_global.cc:35)
/// Calls: mjCopyError
#[allow(unused_variables, non_snake_case)]
pub fn global_model_to_xml(self_ptr: *mut GlobalModel, m: *const mjModel, error: *mut i8, error_sz: i32) -> std__string {
    extern "C" {
        fn GlobalModelToXml_impl(self_ptr: *mut GlobalModel, m: *const mjModel, error: *mut i8, error_sz: i32) -> std__string;
    }
    // SAFETY: Forwarding to linked C/C++ implementation.
    unsafe { GlobalModelToXml_impl(self_ptr, m, error, error_sz) }
}

/// C: GetGlobalModel (xml/xml_global.cc:53)
#[allow(unused_variables, non_snake_case)]
pub fn get_global_model() -> *mut GlobalModel {
    extern "C" {
        fn GetGlobalModel_impl() -> *mut GlobalModel;
    }
    // SAFETY: Forwarding to linked C++ implementation.
    unsafe { GetGlobalModel_impl() }
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
    extern "C" {
        fn GetGlobalXmlSpec_impl(m: *const mjModel, error: *mut i8, error_sz: i32) -> std__string;
    }
    // SAFETY: Forwarding to linked C/C++ implementation.
    unsafe { GetGlobalXmlSpec_impl(m, error, error_sz) }
}

