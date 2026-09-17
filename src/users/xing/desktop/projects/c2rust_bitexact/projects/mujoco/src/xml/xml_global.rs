//! Port of: Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/xml/xml_global.cc
//! IR hash: 1fa44043fe383ce9
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: GlobalModel::Set (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/xml/xml_global.cc:32)
/// Calls: cxx-internal:xml_global.cc.o:__ZN12_GLOBAL__N_114GetGlobalModelEv, cxx:_mj_deleteSpec
#[allow(unused_variables, non_snake_case)]
pub fn global_model_set(self_ptr: *mut GlobalModel, spec: *mut mjSpec) {
    todo!() // cxx-internal:xml_global.cc.o:__ZN12_GLOBAL__N_111GlobalModel3SetEP7mjSpec_
}

/// C: GlobalModel::ToXML (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/xml/xml_global.cc:35)
/// Calls: cxx:__Z11mjCopyErrorPcPKci, cxx:__Z8WriteXMLPK8mjModel_P7mjSpec_Pci
#[allow(unused_variables, non_snake_case)]
pub fn global_model_to_xml(self_ptr: *mut GlobalModel, m: *const mjModel, error: *mut i8, error_sz: i32) -> std__string {
    todo!() // cxx-internal:xml_global.cc.o:__ZN12_GLOBAL__N_111GlobalModel5ToXMLEPK8mjModel_Pci
}

/// C: GetGlobalModel (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/xml/xml_global.cc:53)
#[allow(unused_variables, non_snake_case)]
pub fn get_global_model() -> *mut GlobalModel {
    todo!() // cxx-internal:xml_global.cc.o:__ZN12_GLOBAL__N_114GetGlobalModelEv
}

/// C: SetGlobalXmlSpec (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/xml/xml_global.h:23)
/// Calls: cxx-internal:xml_global.cc.o:__ZN12_GLOBAL__N_111GlobalModel3SetEP7mjSpec_, cxx-internal:xml_global.cc.o:__ZN12_GLOBAL__N_114GetGlobalModelEv
#[allow(unused_variables, non_snake_case)]
pub fn set_global_xml_spec(spec: *mut mjSpec) {
    todo!() // cxx:__Z16SetGlobalXmlSpecP7mjSpec_
}

/// C: GetGlobalXmlSpec (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/xml/xml_global.h:25)
/// Calls: cxx-internal:xml_global.cc.o:__ZN12_GLOBAL__N_111GlobalModel5ToXMLEPK8mjModel_Pci, cxx-internal:xml_global.cc.o:__ZN12_GLOBAL__N_114GetGlobalModelEv
#[allow(unused_variables, non_snake_case)]
pub fn get_global_xml_spec(m: *const mjModel, error: *mut i8, error_sz: i32) -> std__string {
    todo!() // cxx:__Z16GetGlobalXmlSpecPK8mjModel_Pci
}

