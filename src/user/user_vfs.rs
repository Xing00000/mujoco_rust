//! Port of: user/user_vfs.cc
//! IR hash: fd5288ad7242f001
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: OpenFile (user/user_vfs.cc:51)
/// Calls: cxx:_mju_encodeBase64
#[allow(unused_variables, non_snake_case)]
pub fn open_file(filename: *const i8, resource: *mut mjResource) -> i32 {
    todo!() // cxx-internal:user_vfs.cc.o:__ZN12_GLOBAL__N_18OpenFileEPKcP11mjResource_
}

/// C: ReadFile (user/user_vfs.cc:64)
/// Calls: cxx:__ZN6mujoco4user12FileToMemoryEPKc
#[allow(unused_variables, non_snake_case)]
pub fn read_file(filename: *const i8, resource: *mut mjResource, buffer: *const *mut ()) -> i32 {
    todo!() // cxx-internal:user_vfs.cc.o:__ZN12_GLOBAL__N_18ReadFileEPKcP11mjResource_PPKv
}

/// C: CloseFile (user/user_vfs.cc:74)
#[allow(unused_variables, non_snake_case)]
pub fn close_file(resource: *mut mjResource) {
    todo!() // cxx-internal:user_vfs.cc.o:__ZN12_GLOBAL__N_19CloseFileEP11mjResource_
}

/// C: FileModified (user/user_vfs.cc:79)
/// Calls: cxx:_mju_decodeBase64, cxx:_mju_isValidBase64
#[allow(unused_variables, non_snake_case)]
pub fn file_modified(resource: *const mjResource, timestamp: *const i8) -> i32 {
    todo!() // cxx-internal:user_vfs.cc.o:__ZN12_GLOBAL__N_112FileModifiedEPK11mjResource_PKc
}

/// C: StripPathAndLower (user/user_vfs.cc:92)
#[allow(unused_variables, non_snake_case)]
pub fn strip_path_and_lower(path: std__string) -> std__string {
    todo!() // cxx-internal:user_vfs.cc.o:__ZN12_GLOBAL__N_117StripPathAndLowerENSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEE
}

/// C: VFS::Open (user/user_vfs.cc:147)
/// Calls: cxx:__ZN6mujoco4user3VFS14CreateResourceENSt3__117basic_string_viewIcNS2_11char_traitsIcEEEEPK19mjpResourceProvider, cxx:__ZN6mujoco4user3VFS17MaybeSelfDestructEv, cxx:__ZN6mujoco4user3VFS9FindMountERKNSt3__112basic_stringIcNS2_11char_traitsIcEENS2_9allocatorIcEEEE, cxx:__ZNK6mujoco4user8FilePath3StrEv
#[allow(unused_variables, non_snake_case)]
pub fn vfs_open(self_ptr: *mut mujoco__user__VFS, dir: *const i8, name: *const i8, error: *mut i8, nerror: usize) -> *mut mjResource {
    todo!() // cxx:__ZN6mujoco4user3VFS4OpenEPKcS3_Pcm
}

/// C: VFS::Mount (user/user_vfs.cc:182)
/// Calls: cxx:__ZN6mujoco4user3VFS14CreateResourceENSt3__117basic_string_viewIcNS2_11char_traitsIcEEEEPK19mjpResourceProvider, cxx:__ZNK6mujoco4user8FilePath3StrEv, cxx:__ZNK6mujoco4user8FilePath5c_strEv
#[allow(unused_variables, non_snake_case)]
pub fn vfs_mount(self_ptr: *mut mujoco__user__VFS, path: *const mujoco__user__FilePath, provider: *const mjpResourceProvider) -> i32 {
    todo!() // cxx:__ZN6mujoco4user3VFS5MountERKNS0_8FilePathEPK19mjpResourceProvider
}

/// C: VFS::Close (user/user_vfs.cc:197)
/// Calls: cxx:__ZN6mujoco4user3VFS17MaybeSelfDestructEv
#[allow(unused_variables, non_snake_case)]
pub fn vfs_close(self_ptr: *mut mujoco__user__VFS, resource: *mut mjResource) -> i32 {
    todo!() // cxx:__ZN6mujoco4user3VFS5CloseEP11mjResource_
}

/// C: VFS::Unmount (user/user_vfs.cc:214)
/// Calls: cxx:__ZNK6mujoco4user8FilePath3StrEv
#[allow(unused_variables, non_snake_case)]
pub fn vfs_unmount(self_ptr: *mut mujoco__user__VFS, path: *const mujoco__user__FilePath) -> i32 {
    todo!() // cxx:__ZN6mujoco4user3VFS7UnmountERKNS0_8FilePathE
}

/// C: VFS::ContainsBuffer (user/user_vfs.cc:224)
#[allow(unused_variables, non_snake_case)]
pub fn vfs_contains_buffer(self_ptr: *mut mujoco__user__VFS, name: *const i8) -> bool {
    todo!() // cxx:__ZN6mujoco4user3VFS14ContainsBufferEPKc
}

/// C: VFS::ContainsFile (user/user_vfs.cc:230)
/// Calls: cxx:__ZNK6mujoco4user8FilePath3StrEv, cxx:__ZNK6mujoco4user8FilePath5LowerEv, cxx:__ZNK6mujoco4user8FilePath9StripPathEv
#[allow(unused_variables, non_snake_case)]
pub fn vfs_contains_file(self_ptr: *mut mujoco__user__VFS, directory: *const i8, filename: *const i8) -> bool {
    todo!() // cxx:__ZN6mujoco4user3VFS12ContainsFileEPKcS3_
}

/// C: VFS::Read (user/user_vfs.cc:238)
#[allow(unused_variables, non_snake_case)]
pub fn vfs_read(self_ptr: *mut mujoco__user__VFS, resource: *mut mjResource, buffer: *const *mut ()) -> i32 {
    todo!() // cxx:__ZN6mujoco4user3VFS4ReadEP11mjResource_PPKv
}

/// C: VFS::Write (user/user_vfs.cc:245)
/// Calls: cxx:__ZN6mujoco4user3VFS9FindMountERKNSt3__112basic_stringIcNS2_11char_traitsIcEENS2_9allocatorIcEEEE
#[allow(unused_variables, non_snake_case)]
pub fn vfs_write(self_ptr: *mut mujoco__user__VFS, resource: *mut mjResource, buffer: *const (), nbytes: i64) -> i64 {
    todo!() // cxx:__ZN6mujoco4user3VFS5WriteEP11mjResource_PKvx
}

/// C: VFS::CreateResource (user/user_vfs.cc:262)
/// Calls: cxx:_mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn vfs_create_resource(self_ptr: *mut mujoco__user__VFS, name: std__string_view, provider: *const mjpResourceProvider) -> mujoco__user__VFS__ResourcePtr {
    todo!() // cxx:__ZN6mujoco4user3VFS14CreateResourceENSt3__117basic_string_viewIcNS2_11char_traitsIcEEEEPK19mjpResourceProvider
}

/// C: VFS::FindMount (user/user_vfs.cc:283)
/// Calls: cxx-internal:user_vfs.cc.o:__ZN12_GLOBAL__N_117StripPathAndLowerENSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEE, cxx:__ZN6mujoco4user3VFS14CreateResourceENSt3__117basic_string_viewIcNS2_11char_traitsIcEEEEPK19mjpResourceProvider, cxx:_mjp_getResourceProvider
#[allow(unused_variables, non_snake_case)]
pub fn vfs_find_mount(self_ptr: *mut mujoco__user__VFS, fullpath: *const std__string) -> *mut mjResource {
    todo!() // cxx:__ZN6mujoco4user3VFS9FindMountERKNSt3__112basic_stringIcNS2_11char_traitsIcEENS2_9allocatorIcEEEE
}

/// C: VFS::MaybeSelfDestruct (user/user_vfs.cc:319)
#[allow(unused_variables, non_snake_case)]
pub fn vfs_maybe_self_destruct(self_ptr: *mut mujoco__user__VFS) {
    todo!() // cxx:__ZN6mujoco4user3VFS17MaybeSelfDestructEv
}

/// C: VFS::SetToSelfDestruct (user/user_vfs.cc:328)
#[allow(unused_variables, non_snake_case)]
pub fn vfs_set_to_self_destruct(self_ptr: *mut mujoco__user__VFS, destructor: *const ()) {
    todo!() // cxx:__ZN6mujoco4user3VFS17SetToSelfDestructENSt3__18functionIFvvEEE
}

/// C: VFS::Upcast (user/user_vfs.cc:332)
#[allow(unused_variables, non_snake_case)]
pub fn vfs_upcast__0658fcab706b98dd(vfs: *mut mjVFS) -> *mut mujoco__user__VFS {
    todo!() // cxx:__ZN6mujoco4user3VFS6UpcastEP6mjVFS_
}

/// C: VFS::Upcast (user/user_vfs.cc:336)
#[allow(unused_variables, non_snake_case)]
pub fn vfs_upcast__f48059fed1388a56(vfs: *const mjVFS) -> *const mujoco__user__VFS {
    todo!() // cxx:__ZN6mujoco4user3VFS6UpcastEPK6mjVFS_
}

/// C: mj_defaultVFS (user/user_vfs.cc:342)
/// Calls: cxx:_mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mj_default_vfs(vfs: *mut mjVFS) {
    todo!() // cxx:_mj_defaultVFS
}

/// C: mj_deleteVFS (user/user_vfs.cc:350)
#[allow(unused_variables, non_snake_case)]
pub fn mj_delete_vfs(vfs: *mut mjVFS) {
    todo!() // cxx:_mj_deleteVFS
}

/// C: mj_mountVFS (user/user_vfs.cc:357)
/// Calls: cxx:__ZN6mujoco4user3VFS5MountERKNS0_8FilePathEPK19mjpResourceProvider, cxx:_mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mj_mount_vfs(vfs: *mut mjVFS, filepath: *const i8, provider: *const mjpResourceProvider) -> i32 {
    todo!() // cxx:_mj_mountVFS
}

/// C: mj_unmountVFS (user/user_vfs.cc:370)
/// Calls: cxx:__ZN6mujoco4user3VFS7UnmountERKNS0_8FilePathE, cxx:_mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mj_unmount_vfs(vfs: *mut mjVFS, filename: *const i8) -> i32 {
    todo!() // cxx:_mj_unmountVFS
}

/// C: BufferProvider::Mount (user/user_vfs.cc:389)
/// Calls: cxx:__ZN6mujoco4user3VFS5MountERKNS0_8FilePathEPK19mjpResourceProvider, cxx:_mju_decodeBase64, cxx:_mju_encodeBase64, cxx:_mju_error, cxx:_mju_isValidBase64
#[allow(unused_variables, non_snake_case)]
pub fn buffer_provider_mount__c9323ce6536f601f(vfs: *mut mjVFS, args: *const *mut i8, args_1: *const *mut (), args_2: *mut i32) -> i32 {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_vfs.cc:__ZN12_GLOBAL__N_114BufferProvider5MountIJRPKcRPKvRiEEEiP6mjVFS_DpOT_
}

/// C: BufferProvider::Mount (user/user_vfs.cc:389)
/// Calls: cxx:__ZN6mujoco4user3VFS5MountERKNS0_8FilePathEPK19mjpResourceProvider, cxx:_mju_decodeBase64, cxx:_mju_encodeBase64, cxx:_mju_error, cxx:_mju_isValidBase64
#[allow(unused_variables, non_snake_case)]
pub fn buffer_provider_mount__7060258e13d7689a(vfs: *mut mjVFS, args: *const *mut i8, args_1: *const *mut i8) -> i32 {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_vfs.cc:__ZN12_GLOBAL__N_114BufferProvider5MountIJRPKcS4_EEEiP6mjVFS_DpOT_
}

/// C: mj_addFileVFS (user/user_vfs.cc:471)
#[allow(unused_variables, non_snake_case)]
pub fn mj_add_file_vfs(vfs: *mut mjVFS, directory: *const i8, filename: *const i8) -> i32 {
    todo!() // cxx:_mj_addFileVFS
}

/// C: mj_addBufferVFS (user/user_vfs.cc:477)
#[allow(unused_variables, non_snake_case)]
pub fn mj_add_buffer_vfs(vfs: *mut mjVFS, name: *const i8, buffer: *const (), nbuffer: i32) -> i32 {
    todo!() // cxx:_mj_addBufferVFS
}

/// C: mj_deleteFileVFS (user/user_vfs.cc:482)
/// Calls: cxx:__ZNK6mujoco4user8FilePath5LowerEv, cxx:__ZNK6mujoco4user8FilePath5c_strEv, cxx:__ZNK6mujoco4user8FilePath9StripPathEv, cxx:_mj_unmountVFS
#[allow(unused_variables, non_snake_case)]
pub fn mj_delete_file_vfs(vfs: *mut mjVFS, filename: *const i8) -> i32 {
    todo!() // cxx:_mj_deleteFileVFS
}

/// C: mj_containsBufferVFS (user/user_vfs.cc:492)
/// Calls: cxx:__ZN6mujoco4user3VFS14ContainsBufferEPKc, cxx:_mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mj_contains_buffer_vfs(vfs: *mut mjVFS, name: *const i8) -> i32 {
    todo!() // cxx:_mj_containsBufferVFS
}

/// C: mj_containsFileVFS (user/user_vfs.cc:501)
/// Calls: cxx:__ZN6mujoco4user3VFS12ContainsFileEPKcS3_, cxx:_mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mj_contains_file_vfs(vfs: *mut mjVFS, directory: *const i8, filename: *const i8) -> i32 {
    todo!() // cxx:_mj_containsFileVFS
}

