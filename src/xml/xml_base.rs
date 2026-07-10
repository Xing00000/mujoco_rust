//! Port of: xml/xml_base.h
//! IR hash: 05737965add36adb
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mjXBase::Parse (xml/xml_base.h:100)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_base_parse(self_ptr: *mut mjXBase, root: *mut tinyxml2__XMLElement, vfs: *const mjVFS) {
    extern "C" { fn mjXBase_Parse(self_ptr: *mut mjXBase, root: *mut tinyxml2__XMLElement, vfs: *const mjVFS); }
    // SAFETY: delegates to C implementation
    unsafe { mjXBase_Parse(self_ptr, root, vfs) }
}

/// C: mjXBase::Write (xml/xml_base.h:103)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_base_write(self_ptr: *mut mjXBase, error: *mut i8, error_sz: usize) -> std__string {
    extern "C" { fn mjXBase_Write(self_ptr: *mut mjXBase, error: *mut i8, error_sz: usize) -> std__string; }
    // SAFETY: delegates to C implementation
    unsafe { mjXBase_Write(self_ptr, error, error_sz) }
}

/// C: mjXBase::SetModel (xml/xml_base.h:108)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_base_set_model(self_ptr: *mut mjXBase, arg0: *mut mjSpec, arg1: *const mjModel) {
    extern "C" { fn mjXBase_SetModel(self_ptr: *mut mjXBase, arg0: *mut mjSpec, arg1: *const mjModel); }
    // SAFETY: delegates to C implementation
    unsafe { mjXBase_SetModel(self_ptr, arg0, arg1) }
}

/// C: mjXBase::ReadAlternative (xml/xml_base.h:111)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_base_read_alternative(elem: *mut tinyxml2__XMLElement, alt: *mut mjsOrientation) -> i32 {
    if elem.is_null() { return 0; }
    extern "C" { fn mjXBase_ReadAlternative(elem: *mut tinyxml2__XMLElement, alt: *mut mjsOrientation) -> i32; }
    // SAFETY: elem verified non-null
    unsafe { mjXBase_ReadAlternative(elem, alt) }
}

