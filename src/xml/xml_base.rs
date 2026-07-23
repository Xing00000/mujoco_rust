//! Port of: xml/xml_base.h
//! IR hash: 3fb6da908ad9d71c
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mjXBase::Parse (xml/xml_base.h:100)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_base_parse(self_ptr: *mut mjXBase, root: *mut tinyxml2__XMLElement, vfs: *const mjVFS) {
    todo!() // mjXBase::Parse
}

/// C: mjXBase::Write (xml/xml_base.h:103)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_base_write(self_ptr: *mut mjXBase, error: *mut i8, error_sz: u64) -> std__string {
    todo!() // mjXBase::Write
}

/// C: mjXBase::SetModel (xml/xml_base.h:108)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_base_set_model(self_ptr: *mut mjXBase, arg0: *mut mjSpec, arg1: *const mjModel) {
    // SAFETY: self_ptr is valid; sets the spec pointer
    unsafe { (*self_ptr).spec = arg0; }
}

/// C: mjXBase::ReadAlternative (xml/xml_base.h:111)
/// Calls: mjXUtil::ReadAttr
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_base_read_alternative(elem: *mut tinyxml2__XMLElement, alt: *mut mjsOrientation) -> i32 {
    todo!() // mjXBase::ReadAlternative
}

