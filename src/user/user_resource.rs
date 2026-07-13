//! Port of: user/user_resource.cc
//! IR hash: d3ac8715281cd691
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mju_openResource (user/user_resource.cc:34)
/// Calls: VFS::Open, VFS::Upcast, mj_defaultVFS, mj_deleteVFS, mju_free, mju_malloc
#[allow(unused_variables, non_snake_case)]
pub fn mju_open_resource(dir: *const i8, name: *const i8, vfs: *const mjVFS, error: *mut i8, nerror: usize) -> *mut mjResource {
    // NOTE: signature changed from previous IR version
    // Previous params: (dir : * const i8, name : * const i8, vfs : * const mjVFS, error : * mut i8, nerror : usize)
    // Previous return: * mut mjResource
    todo!("re-translate: params renamed")
}

/// C: mju_closeResource (user/user_resource.cc:69)
/// Calls: VFS::Close, VFS::Upcast
#[allow(unused_variables, non_snake_case)]
pub fn mju_close_resource(resource: *mut mjResource) {
    // NOTE: signature changed from previous IR version
    // Previous params: (resource : * mut mjResource)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mju_readResource (user/user_resource.cc:75)
/// Calls: VFS::Read, VFS::Upcast
#[allow(unused_variables, non_snake_case)]
pub fn mju_read_resource(resource: *mut mjResource, buffer: *const *mut ()) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (resource : * mut mjResource, buffer : * const * mut ())
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: mju_getResourceDir (user/user_resource.cc:82)
#[allow(unused_variables, non_snake_case)]
pub fn mju_get_resource_dir(resource: *mut mjResource, dir: *const *mut i8, ndir: *mut i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (resource : * mut mjResource, dir : * const * mut i8, ndir : * mut i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mju_isModifiedResource (user/user_resource.cc:105)
#[allow(unused_variables, non_snake_case)]
pub fn mju_is_modified_resource(resource: *const mjResource, timestamp: *const i8) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (resource : * const mjResource, timestamp : * const i8)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: mju_decodeResource (user/user_resource.cc:112)
/// Calls: mjp_findDecoder, mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn mju_decode_resource(resource: *mut mjResource, content_type: *const i8, vfs: *const mjVFS) -> *mut mjSpec {
    // NOTE: signature changed from previous IR version
    // Previous params: (resource : * mut mjResource, content_type : * const i8, vfs : * const mjVFS)
    // Previous return: * mut mjSpec
    todo!("re-translate: params renamed")
}

