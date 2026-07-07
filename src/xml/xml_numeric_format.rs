//! Port of: xml/xml_numeric_format.h
//! IR hash: 05737965add36adb
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: _mjPRIVATE__get_xml_precision (xml/xml_numeric_format.h:23)
#[allow(unused_variables, non_snake_case)]
pub fn mj_private_get_xml_precision() -> i32 {
    extern "C" { fn _mjPRIVATE__get_xml_precision_impl() -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { _mjPRIVATE__get_xml_precision_impl() }
}

/// C: _mjPRIVATE__set_xml_precision (xml/xml_numeric_format.h:24)
#[allow(unused_variables, non_snake_case)]
pub fn mj_private_set_xml_precision(precision: i32) {
    extern "C" { fn _mjPRIVATE__set_xml_precision_impl(precision: i32); }
    // SAFETY: delegates to C implementation
    unsafe { _mjPRIVATE__set_xml_precision_impl(precision) }
}

