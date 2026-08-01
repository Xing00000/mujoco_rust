//! Port of: user/user_composite.cc
//! IR hash: 9343293228317031
//! CODEGEN: source paths, owners, and callable names are locked.

use crate::types::*;

/// C: comperr (user/user_composite.cc:41)
/// Calls: cxx:_mju_strncpy
#[allow(unused_variables, non_snake_case)]
pub fn comperr(error: *mut i8, msg: *const i8, error_sz: i32) -> bool {
    crate::engine::engine_util_misc::mju_strncpy(error, msg, error_sz);
    false
}

