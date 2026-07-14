//! Port of: xml/xml_numeric_format.h
//! IR hash: 9614bd3d92e7766b
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

use std :: cell :: Cell ;
thread_local ! { static PRECISION : Cell < i32 > = Cell :: new (6) ; }

/// C: _mjPRIVATE__get_xml_precision (xml/xml_numeric_format.h:23)
#[allow(unused_variables, non_snake_case)]
pub fn mj_private_get_xml_precision() -> i32 {
    PRECISION.with(|p| p.get())
}

/// C: _mjPRIVATE__set_xml_precision (xml/xml_numeric_format.h:24)
#[allow(unused_variables, non_snake_case)]
pub fn mj_private_set_xml_precision(precision: i32) {
    PRECISION.with(|p| p.set(precision));
}

