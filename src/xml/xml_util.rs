//! Port of: xml/xml_util.h
//! IR hash: 9343293228317031
//! CODEGEN: source paths, owners, and callable names are locked.

use crate::types::*;

/// C: mjCopyError (xml/xml_util.h:32)
#[allow(unused_variables, non_snake_case)]
pub fn mjCopyError(dst: *mut i8, src: *const i8, maxlen: i32) {
    // SAFETY: dst and src are valid C strings from caller, maxlen > 0 guaranteed by check
    unsafe {
        if dst.is_null() || maxlen <= 0 {
            return;
        }
        let n = maxlen as usize;
        let mut i = 0usize;
        while i < n - 1 {
            let c = *src.add(i);
            if c == 0 {
                break;
            }
            *dst.add(i) = c;
            i += 1;
        }
        // fill rest with zeros (strncpy behavior)
        while i < n {
            *dst.add(i) = 0;
            i += 1;
        }
        // null-terminate last byte unconditionally
        *dst.add(n - 1) = 0;
    }
}

