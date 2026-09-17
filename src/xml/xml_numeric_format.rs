//! Port of: xml/xml_numeric_format.cc
//! IR hash: fd5288ad7242f001
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

use std :: cell :: Cell ;
thread_local ! { static PRECISION : Cell < i32 > = Cell :: new (6) ; }

/// C: _mjPRIVATE__get_xml_precision (xml/xml_numeric_format.cc:23)
#[allow(unused_variables, non_snake_case)]
pub fn mj_private_get_xml_precision() -> i32 {
    todo!() // cxx:__mjPRIVATE__get_xml_precision
}

/// C: _mjPRIVATE__set_xml_precision (xml/xml_numeric_format.cc:27)
#[allow(unused_variables, non_snake_case)]
pub fn mj_private_set_xml_precision(precision: i32) {
    todo!() // cxx:__mjPRIVATE__set_xml_precision
}

