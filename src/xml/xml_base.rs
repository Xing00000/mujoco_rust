//! Port of: xml/xml_base.h
//! IR hash: d3ac8715281cd691
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mjXBase::Parse (xml/xml_base.h:100)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_base_parse(self_ptr: *mut mjXBase, root: *mut tinyxml2__XMLElement, vfs: *const mjVFS) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjXBase, root : * mut tinyxml2__XMLElement, vfs : * const mjVFS)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjXBase::Write (xml/xml_base.h:103)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_base_write(self_ptr: *mut mjXBase, error: *mut i8, error_sz: usize) -> std__string {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjXBase, error : * mut i8, error_sz : usize)
    // Previous return: std__string
    todo!("re-translate: params renamed")
}

/// C: mjXBase::SetModel (xml/xml_base.h:108)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_base_set_model(self_ptr: *mut mjXBase, arg0: *mut mjSpec, arg1: *const mjModel) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjXBase, arg0 : * mut mjSpec, arg1 : * const mjModel)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjXBase::ReadAlternative (xml/xml_base.h:111)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_base_read_alternative(elem: *mut tinyxml2__XMLElement, alt: *mut mjsOrientation) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (elem : * mut tinyxml2__XMLElement, alt : * mut mjsOrientation)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

