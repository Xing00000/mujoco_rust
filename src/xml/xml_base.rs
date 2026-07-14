//! Port of: xml/xml_base.h
//! IR hash: 8cbd078414266fa8
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mjXBase::Parse (xml/xml_base.h:100)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_base_parse(self_ptr: *mut mjXBase, root: *mut tinyxml2__XMLElement, vfs: *const mjVFS) {
    todo!() // mjXBase::Parse
}

/// C: mjXBase::Write (xml/xml_base.h:103)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_base_write(self_ptr: *mut mjXBase, error: *mut i8, error_sz: usize) -> std__string {
    todo!() // mjXBase::Write
}

/// C: mjXBase::SetModel (xml/xml_base.h:108)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_base_set_model(self_ptr: *mut mjXBase, arg0: *mut mjSpec, arg1: *const mjModel) {
    todo!() // mjXBase::SetModel
}

/// C: mjXBase::ReadAlternative (xml/xml_base.h:111)
/// Calls: mjXUtil::ReadAttr
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_base_read_alternative(elem: *mut tinyxml2__XMLElement, alt: *mut mjsOrientation) -> i32 {
    todo!() // mjXBase::ReadAlternative
}

