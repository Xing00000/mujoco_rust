//! Port of: Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_resource.cc
//! IR hash: 1fa44043fe383ce9
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mju_openResource (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_resource.cc:34)
/// Calls: cxx:__ZN6mujoco4user3VFS17SetToSelfDestructENSt3__18functionIFvvEEE, cxx:__ZN6mujoco4user3VFS4OpenEPKcS3_, cxx:_mj_defaultVFS, cxx:_mj_deleteVFS, cxx:_mju_free, cxx:_mju_malloc
#[allow(unused_variables, non_snake_case)]
pub fn mju_open_resource(dir: *const i8, name: *const i8, vfs: *const mjVFS, error: *mut i8, nerror: usize) -> *mut mjResource {
    todo!() // cxx:_mju_openResource
}

/// C: mju_closeResource (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_resource.cc:69)
/// Calls: cxx:__ZN6mujoco4user3VFS5CloseEP11mjResource_
#[allow(unused_variables, non_snake_case)]
pub fn mju_close_resource(resource: *mut mjResource) {
    todo!() // cxx:_mju_closeResource
}

/// C: mju_readResource (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_resource.cc:75)
/// Calls: cxx:__ZN6mujoco4user3VFS4ReadEP11mjResource_PPKv
#[allow(unused_variables, non_snake_case)]
pub fn mju_read_resource(resource: *mut mjResource, buffer: *const *mut ()) -> i32 {
    todo!() // cxx:_mju_readResource
}

/// C: mju_getResourceDir (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_resource.cc:82)
#[allow(unused_variables, non_snake_case)]
pub fn mju_get_resource_dir(resource: *mut mjResource, dir: *const *mut i8, ndir: *mut i32) {
    todo!() // cxx:_mju_getResourceDir
}

/// C: mju_isModifiedResource (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_resource.cc:105)
#[allow(unused_variables, non_snake_case)]
pub fn mju_is_modified_resource(resource: *const mjResource, timestamp: *const i8) -> i32 {
    todo!() // cxx:_mju_isModifiedResource
}

/// C: mju_decodeResource (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_resource.cc:112)
/// Calls: cxx:__Z21mjuu_extToContentTypeNSt3__117basic_string_viewIcNS_11char_traitsIcEEEE, cxx:_mjp_findDecoder, cxx:_mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn mju_decode_resource(resource: *mut mjResource, content_type: *const i8, vfs: *const mjVFS) -> *mut mjSpec {
    todo!() // cxx:_mju_decodeResource
}

