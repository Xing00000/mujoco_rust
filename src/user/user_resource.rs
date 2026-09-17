//! Port of: user/user_resource.cc
//! IR hash: fd5288ad7242f001
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: openResourceInternal (user/user_resource.cc:33)
/// Calls: cxx:__ZN6mujoco4user3VFS17SetToSelfDestructENSt3__18functionIFvvEEE, cxx:__ZN6mujoco4user3VFS4OpenEPKcS3_Pcm, cxx:_mj_defaultVFS, cxx:_mj_deleteVFS, cxx:_mju_free, cxx:_mju_malloc
#[allow(unused_variables, non_snake_case)]
pub fn open_resource_internal(dir: *const i8, name: *const i8, vfs: *const mjVFS, error: *mut i8, nerror: usize) -> *mut mjResource {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_resource.cc:__ZL20openResourceInternalPKcS0_PK6mjVFS_Pcm
}

/// C: mju_openResource (user/user_resource.cc:68)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_resource.cc:__ZL20openResourceInternalPKcS0_PK6mjVFS_Pcm
#[allow(unused_variables, non_snake_case)]
pub fn mju_open_resource(dir: *const i8, name: *const i8, vfs: *const mjVFS, error: *mut i8, nerror: usize) -> *mut mjResource {
    todo!() // cxx:_mju_openResource
}

/// C: mju_closeResource (user/user_resource.cc:72)
/// Calls: cxx:__ZN6mujoco4user3VFS5CloseEP11mjResource_
#[allow(unused_variables, non_snake_case)]
pub fn mju_close_resource(resource: *mut mjResource) {
    todo!() // cxx:_mju_closeResource
}

/// C: mju_readResource (user/user_resource.cc:76)
/// Calls: cxx:__ZN6mujoco4user3VFS4ReadEP11mjResource_PPKv
#[allow(unused_variables, non_snake_case)]
pub fn mju_read_resource(resource: *mut mjResource, buffer: *const *mut ()) -> i32 {
    todo!() // cxx:_mju_readResource
}

/// C: mju_writeResource (user/user_resource.cc:88)
/// Calls: cxx:__ZN6mujoco4user3VFS5WriteEP11mjResource_PKvx, cxx:_mj_defaultVFS, cxx:_mj_deleteVFS, cxx:_mju_free, cxx:_mju_malloc
#[allow(unused_variables, non_snake_case)]
pub fn mju_write_resource(name: *const i8, buffer: *const (), nbytes: i64, vfs: *const mjVFS, error: *mut i8, nerror: usize) -> i64 {
    todo!() // cxx:_mju_writeResource
}

/// C: mju_getResourceDir (user/user_resource.cc:123)
#[allow(unused_variables, non_snake_case)]
pub fn mju_get_resource_dir(resource: *mut mjResource, dir: *const *mut i8, ndir: *mut i32) {
    todo!() // cxx:_mju_getResourceDir
}

/// C: mju_isModifiedResource (user/user_resource.cc:143)
#[allow(unused_variables, non_snake_case)]
pub fn mju_is_modified_resource(resource: *const mjResource, timestamp: *const i8) -> i32 {
    todo!() // cxx:_mju_isModifiedResource
}

/// C: mju_decodeResource (user/user_resource.cc:150)
/// Calls: cxx:__Z21mjuu_extToContentTypeNSt3__117basic_string_viewIcNS_11char_traitsIcEEEE, cxx:_mjp_findDecoder, cxx:_mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn mju_decode_resource(resource: *mut mjResource, content_type: *const i8, vfs: *const mjVFS) -> *mut mjSpec {
    todo!() // cxx:_mju_decodeResource
}

