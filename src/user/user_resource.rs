//! Port of: user/user_resource.cc
//! IR hash: 9343293228317031
//! CODEGEN: source paths, owners, and callable names are locked.

use crate::types::*;

/// C: mju_getResourceDir (user/user_resource.cc:82)
#[allow(unused_variables, non_snake_case)]
pub fn mju_getResourceDir(resource: *mut mjResource, dir: *const *mut i8, ndir: *mut i32) {
    // SAFETY: resource, dir, ndir are valid pointers (caller contract)
    unsafe {
        *(dir as *mut *const i8) = std::ptr::null();
        *ndir = 0;

        if resource.is_null() {
            return;
        }
        let name = (*resource).name;
        if name.is_null() {
            return;
        }

        // ensure prefix is included even if there is no separator
        let mut prefix_len: i32 = 0;
        let provider = (*resource).provider as *const mjpResourceProvider;
        if !provider.is_null() && !(*provider).prefix.is_null() {
            // SAFETY: prefix is a valid C string — compute strlen manually
            let mut len: i32 = 0;
            while *(*provider).prefix.add(len as usize) != 0 {
                len += 1;
            }
            prefix_len = len + 1;
        }

        *(dir as *mut *const i8) = name as *const i8;
        *ndir = prefix_len;
        let mut i = prefix_len;
        while *name.add(i as usize) != 0 {
            let ch = *name.add(i as usize);
            if ch == b'/' as i8 || ch == b'\\' as i8 {
                *ndir = i + 1;
            }
            i += 1;
        }
    }
}

/// C: mju_isModifiedResource (user/user_resource.cc:105)
#[allow(unused_variables, non_snake_case)]
pub fn mju_isModifiedResource(resource: *const mjResource, timestamp: *const i8) -> i32 {
    // SAFETY: resource may be null (checked). provider is accessed through mjResource.provider field.
    // The mjpResourceProvider C struct has 'modified' function pointer at offset 48:
    //   prefix(8) + open(8) + read(8) + close(8) + mount(8) + unmount(8) = 48
    // modified type: int (*)(const mjResource*, const char*)
    unsafe {
        if resource.is_null() {
            return 1;
        }
        let provider = (*resource).provider as *const u8;
        if provider.is_null() {
            return 1;
        }
        // read the 'modified' function pointer at offset 48
        let modified_fn_ptr = *(provider.add(48)
            as *const Option<unsafe extern "C" fn(*const mjResource, *const i8) -> i32>);
        if let Some(modified) = modified_fn_ptr {
            return modified(resource, timestamp);
        }
        1 // default: assume modified
    }
}

