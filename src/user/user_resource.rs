//! Port of: user/user_resource.cc
//! IR hash: 699b5f0da57e8d78
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mju_openResource (user/user_resource.cc:33)
/// Calls: VFS::Open, VFS::Upcast, mj_defaultVFS, mj_deleteVFS, mju_free
#[allow(unused_variables, non_snake_case)]
pub fn mju_open_resource(dir: *const i8, name: *const i8, vfs: *const mjVFS, error: *mut i8, nerror: i32) -> *mut mjResource {
    // WARNING: signature changed — verify body
    // Previous params: (dir : * const i8, name : * const i8, vfs : * const mjVFS, error : * mut i8, nerror : i32)
    // Previous return: * mut mjResource
    todo ! ()
}

/// C: mju_closeResource (user/user_resource.cc:69)
/// Calls: VFS::Close, VFS::Upcast
#[allow(unused_variables, non_snake_case)]
pub fn mju_close_resource(resource: *mut mjResource) {
    // WARNING: signature changed — verify body
    // Previous params: (resource : * mut mjResource)
    // Previous return: ()
    todo ! ()
}

/// C: mju_readResource (user/user_resource.cc:75)
/// Calls: VFS::Read, VFS::Upcast
#[allow(unused_variables, non_snake_case)]
pub fn mju_read_resource(resource: *mut mjResource, buffer: *const *mut ()) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (resource : * mut mjResource, buffer : * const * mut ())
    // Previous return: i32
    todo ! ()
}

/// C: mju_getResourceDir (user/user_resource.cc:82)
#[allow(unused_variables, non_snake_case)]
pub fn mju_get_resource_dir(resource: *mut mjResource, dir: *const *mut i8, ndir: *mut i32) {
    // WARNING: signature changed — verify body
    // Previous params: (resource : * mut mjResource, dir : * const * mut i8, ndir : * mut i32)
    // Previous return: ()
    todo ! ()
}

/// C: mju_isModifiedResource (user/user_resource.cc:105)
#[allow(unused_variables, non_snake_case)]
pub fn mju_is_modified_resource(resource: *const mjResource, timestamp: *const i8) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (resource : * const mjResource, timestamp : * const i8)
    // Previous return: i32
    todo ! ()
}

/// C: mju_decodeResource (user/user_resource.cc:112)
/// Calls: mjp_findDecoder, mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn mju_decode_resource(resource: *mut mjResource, content_type: *const i8, vfs: *const mjVFS) -> *mut mjSpec {
    // WARNING: signature changed — verify body
    // Previous params: (resource : * mut mjResource, content_type : * const i8, vfs : * const mjVFS)
    // Previous return: * mut mjSpec
    todo ! ()
}

