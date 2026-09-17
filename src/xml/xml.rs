//! Port of: xml/xml.cc
//! IR hash: fd5288ad7242f001
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: LocaleOverride::PosixLocale (xml/xml.cc:85)
#[allow(unused_variables, non_snake_case)]
pub fn locale_override_posix_locale() -> *mut xlocale {
    todo!() // cxx-internal:xml.cc.o:__ZN12_GLOBAL__N_114LocaleOverride11PosixLocaleEv
}

/// C: IncludeXML (xml/xml.cc:100)
/// Calls: cxx:__ZNK6mujoco4user8FilePath3StrEv, cxx:__ZNK6mujoco4user8FilePath5IsAbsEv, cxx:__ZNK6mujoco4user8FilePath5c_strEv, cxx:__ZNK9mjXReader12ModelFileDirEv, cxx:_mju_closeResource, cxx:_mju_getResourceDir, cxx:_mju_openResource, cxx:_mju_readResource
#[allow(unused_variables, non_snake_case)]
pub fn include_xml(reader: *mut mjXReader, elem: *mut tinyxml2__XMLElement, dir: *const mujoco__user__FilePath, vfs: *const mjVFS, included: *const ()) {
    todo!() // cxx-internal:xml.cc.o:__ZN12_GLOBAL__N_110IncludeXMLER9mjXReaderPN8tinyxml210XMLElementERKN6mujoco4user8FilePathEPK6mjVFS_RNSt3__113unordered_setINSD_12basic_stringIcNSD_11char_traitsIcEENSD_9allocatorIcEEEENSD_4hashISK_EENSD_8equal_toISK_EENSI_ISK_EEEE
}

/// C: SpecFromXML (xml/xml.cc:231)
/// Calls: cxx-internal:xml.cc.o:__ZN12_GLOBAL__N_110IncludeXMLER9mjXReaderPN8tinyxml210XMLElementERKN6mujoco4user8FilePathEPK6mjVFS_RNSt3__113unordered_setINSD_12basic_stringIcNSD_11char_traitsIcEENSD_9allocatorIcEEEENSD_4hashISK_EENSD_8equal_toISK_EENSI_ISK_EEEE, cxx:__Z11mjCopyErrorPcPKci, cxx:__ZN7mjXBase8SetModelEP7mjSpec_PK8mjModel_, cxx:_mj_deleteSpec, cxx:_mj_makeSpec, cxx:_mjs_getString, cxx:_mjs_setString
#[allow(unused_variables, non_snake_case)]
pub fn spec_from_xml(xml: std__string_view, dir: std__string_view, filename: std__string_view, vfs: *const mjVFS, error: *mut i8, nerror: i32) -> *mut mjSpec {
    todo!() // cxx-internal:xml.cc.o:__ZN12_GLOBAL__N_111SpecFromXMLENSt3__117basic_string_viewIcNS0_11char_traitsIcEEEES4_S4_PK6mjVFS_Pci
}

/// C: ParseXML (xml/xml.cc:310)
/// Calls: cxx-internal:xml.cc.o:__ZN12_GLOBAL__N_111SpecFromXMLENSt3__117basic_string_viewIcNS0_11char_traitsIcEEEES4_S4_PK6mjVFS_Pci, cxx:_mju_closeResource, cxx:_mju_getResourceDir, cxx:_mju_openResource, cxx:_mju_readResource
#[allow(unused_variables, non_snake_case)]
pub fn parse_xml(filename: *const i8, vfs: *const mjVFS, error: *mut i8, nerror: i32) -> *mut mjSpec {
    todo!() // cxx:__Z8ParseXMLPKcPK6mjVFS_Pci
}

/// C: ParseSpecFromString (xml/xml.cc:348)
/// Calls: cxx-internal:xml.cc.o:__ZN12_GLOBAL__N_111SpecFromXMLENSt3__117basic_string_viewIcNS0_11char_traitsIcEEEES4_S4_PK6mjVFS_Pci
#[allow(unused_variables, non_snake_case)]
pub fn parse_spec_from_string(xml: std__string_view, vfs: *const mjVFS, error: *mut i8, nerror: i32) -> *mut mjSpec {
    todo!() // cxx:__Z19ParseSpecFromStringNSt3__117basic_string_viewIcNS_11char_traitsIcEEEEPK6mjVFS_Pci
}

/// C: WriteXML (xml/xml.cc:353)
/// Calls: cxx:__Z11mjCopyErrorPcPKci, cxx:__ZN9mjXWriter8SetModelEP7mjSpec_PK8mjModel_
#[allow(unused_variables, non_snake_case)]
pub fn write_xml(m: *const mjModel, spec: *mut mjSpec, error: *mut i8, nerror: i32) -> std__string {
    todo!() // cxx:__Z8WriteXMLPK8mjModel_P7mjSpec_Pci
}

