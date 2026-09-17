//! Port of: xml/xml_base.cc
//! IR hash: fd5288ad7242f001
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mjXBase::SetModel (xml/xml_base.cc:46)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_base_set_model(self_ptr: *mut mjXBase, arg0: *mut mjSpec, arg1: *const mjModel) {
    todo!() // cxx:__ZN7mjXBase8SetModelEP7mjSpec_PK8mjModel_
}

/// C: mjXBase::ReadAlternative (xml/xml_base.cc:52)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_base_read_alternative(self_ptr: *mut mjXBase, elem: *mut tinyxml2__XMLElement, alt: *mut mjsOrientation) -> i32 {
    todo!() // cxx:__ZN7mjXBase15ReadAlternativeEPN8tinyxml210XMLElementER15mjsOrientation_
}

/// C: mjXBase::Parse (xml/xml_base.h:38)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_base_parse(self_ptr: *mut mjXBase, root: *mut tinyxml2__XMLElement, vfs: *const mjVFS) {
    todo!() // cxx:__ZN7mjXBase5ParseEPN8tinyxml210XMLElementEPK6mjVFS_
}

/// C: mjXBase::Write (xml/xml_base.h:41)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_base_write(self_ptr: *mut mjXBase, error: *mut i8, error_sz: u64) -> std__string {
    todo!() // cxx:__ZN7mjXBase5WriteEPcm
}

