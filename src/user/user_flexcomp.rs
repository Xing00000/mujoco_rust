//! Port of: user/user_flexcomp.cc
//! IR hash: 9343293228317031
//! CODEGEN: source paths, owners, and callable names are locked.

use crate::types::*;

/// C: ReadStrFromBuffer (user/user_flexcomp.cc:61)
#[allow(unused_variables, non_snake_case)]
pub fn ReadStrFromBuffer(dest: *mut i8, src: *const i8, maxlen: i32) {
    // C: std::strncpy(dest, src, maxlen)
    // SAFETY: dest and src are valid pointers; maxlen bounds the copy.
    unsafe {
        if maxlen <= 0 { return; }
        let n = maxlen as usize;
        let mut i = 0usize;
        // strncpy: copy up to maxlen chars, padding with null if shorter
        while i < n {
            let c = *src.add(i);
            *dest.add(i) = c;
            if c == 0 { i += 1; break; }
            i += 1;
        }
        // pad remaining with null
        while i < n {
            *dest.add(i) = 0;
            i += 1;
        }
    }
}

/// C: mat2lin (user/user_flexcomp.cc:1103)
#[allow(unused_variables, non_snake_case)]
pub fn mat2lin(ix: i32, iy: i32, iz: i32, count: *const i32) -> i32 {
    // SAFETY: count points to an array of at least 3 ints (caller contract)
    unsafe {
        ix * *count.add(1) * *count.add(2) + iy * *count.add(2) + iz
    }
}

/// C: findstring (user/user_flexcomp.cc:1426)
#[allow(unused_variables, non_snake_case)]
pub fn findstring(buffer: *const i8, buffer_sz: i32, str: *const i8) -> i32 {
    // SAFETY: buffer and str are valid C strings/buffers from mujoco
    unsafe {
        // manual strlen
        let mut len: i32 = 0;
        while *str.offset(len as isize) != 0 {
            len += 1;
        }

        // scan buffer
        let mut i: i32 = 0;
        while i < buffer_sz - len {
            // check for string at position i
            let mut found = true;
            let mut k: i32 = 0;
            while k < len {
                if *buffer.offset((i + k) as isize) != *str.offset(k as isize) {
                    found = false;
                    break;
                }
                k += 1;
            }

            // string found
            if found {
                return i;
            }
            i += 1;
        }

        // not found
        -1
    }
}

