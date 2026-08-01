//! Port of: xml/xml_numeric_format.h
//! IR hash: 9343293228317031
//! CODEGEN: source paths, owners, and callable names are locked.

use crate::types::*;

use std :: cell :: Cell ;
thread_local ! { static PRECISION : Cell < i32 > = Cell :: new (6) ; }

