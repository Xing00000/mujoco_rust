//! Port of: user/user_vfs.cc
//! IR hash: 1b139f44af8230f9
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: OpenFile (user/user_vfs.cc:50)
#[allow(unused_variables, non_snake_case)]
pub fn open_file(filename: *const i8, resource: *mut mjResource) -> i32 {
    todo!() // OpenFile
}

/// C: ReadFile (user/user_vfs.cc:64)
#[allow(unused_variables, non_snake_case)]
pub fn read_file(filename: *const i8, resource: *mut mjResource, buffer: *const *mut ()) -> i32 {
    todo!() // ReadFile
}

/// C: CloseFile (user/user_vfs.cc:74)
#[allow(unused_variables, non_snake_case)]
pub fn close_file(resource: *mut mjResource) {
    todo!() // CloseFile
}

/// C: FileModified (user/user_vfs.cc:79)
#[allow(unused_variables, non_snake_case)]
pub fn file_modified(resource: *const mjResource, timestamp: *const i8) -> i32 {
    todo!() // FileModified
}

/// C: StripPathAndLower (user/user_vfs.cc:94)
#[allow(unused_variables, non_snake_case)]
pub fn strip_path_and_lower(path: i32) -> i32 {
    todo!() // StripPathAndLower
}

/// C: VFS::FindMount (user/user_vfs.cc:287)
#[allow(unused_variables, non_snake_case)]
pub fn vfs_find_mount(fullpath: *const i32) -> *mut mjResource {
    todo!() // VFS::FindMount
}

/// C: mj_defaultVFS (user/user_vfs.cc:355)
/// Calls: mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mj_default_vfs(vfs: *mut mjVFS) {
    todo!() // mj_defaultVFS
}

/// C: mj_deleteVFS (user/user_vfs.cc:363)
/// Calls: VFS::Upcast
#[allow(unused_variables, non_snake_case)]
pub fn mj_delete_vfs(vfs: *mut mjVFS) {
    todo!() // mj_deleteVFS
}

/// C: mj_mountVFS (user/user_vfs.cc:371)
/// Calls: VFS::Mount, VFS::Upcast, mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mj_mount_vfs(vfs: *mut mjVFS, filepath: *const i8, provider: *const mjpResourceProvider) -> i32 {
    todo!() // mj_mountVFS
}

/// C: mj_unmountVFS (user/user_vfs.cc:386)
/// Calls: VFS::Unmount, VFS::Upcast, mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mj_unmount_vfs(vfs: *mut mjVFS, filename: *const i8) -> i32 {
    todo!() // mj_unmountVFS
}

/// C: BufferProvider::Mount (user/user_vfs.cc:407)
/// Calls: VFS::Mount, VFS::Upcast, mju_decodeBase64, mju_encodeBase64, mju_error, mju_isValidBase64
#[allow(unused_variables, non_snake_case)]
pub fn buffer_provider_mount(vfs: *mut mjVFS, args: Args) -> i32 {
    todo!() // BufferProvider::Mount
}

/// C: mj_addFileVFS (user/user_vfs.cc:496)
#[allow(unused_variables, non_snake_case)]
pub fn mj_add_file_vfs(vfs: *mut mjVFS, directory: *const i8, filename: *const i8) -> i32 {
    todo!() // mj_addFileVFS
}

/// C: mj_addBufferVFS (user/user_vfs.cc:503)
#[allow(unused_variables, non_snake_case)]
pub fn mj_add_buffer_vfs(vfs: *mut mjVFS, name: *const i8, buffer: *const (), nbuffer: i32) -> i32 {
    todo!() // mj_addBufferVFS
}

/// C: mj_deleteFileVFS (user/user_vfs.cc:508)
/// Calls: FilePath::Lower, FilePath::StripPath, FilePath::c_str, mj_unmountVFS
#[allow(unused_variables, non_snake_case)]
pub fn mj_delete_file_vfs(vfs: *mut mjVFS, filename: *const i8) -> i32 {
    todo!() // mj_deleteFileVFS
}

/// C: mj_containsBufferVFS (user/user_vfs.cc:520)
/// Calls: VFS::ContainsBuffer, VFS::Upcast, mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mj_contains_buffer_vfs(vfs: *mut mjVFS, name: *const i8) -> i32 {
    todo!() // mj_containsBufferVFS
}

/// C: mj_containsFileVFS (user/user_vfs.cc:529)
/// Calls: VFS::ContainsFile, VFS::Upcast, mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mj_contains_file_vfs(vfs: *mut mjVFS, directory: *const i8, filename: *const i8) -> i32 {
    todo!() // mj_containsFileVFS
}

/// C: VFS::Open (user/user_vfs.h:75)
/// Calls: VFS::MaybeSelfDestruct
#[allow(unused_variables, non_snake_case)]
pub fn vfs_open(dir: *const i8, name: *const i8) -> *mut mjResource {
    todo!() // VFS::Open
}

/// C: VFS::Read (user/user_vfs.h:80)
#[allow(unused_variables, non_snake_case)]
pub fn vfs_read(resource: *mut mjResource, buffer: *const *mut ()) -> i32 {
    todo!() // VFS::Read
}

/// C: VFS::Close (user/user_vfs.h:84)
/// Calls: VFS::MaybeSelfDestruct
#[allow(unused_variables, non_snake_case)]
pub fn vfs_close(resource: *mut mjResource) -> Status {
    todo!() // VFS::Close
}

/// C: VFS::Mount (user/user_vfs.h:88)
#[allow(unused_variables, non_snake_case)]
pub fn vfs_mount(path: *const FilePath, provider: *const mjpResourceProvider) -> Status {
    todo!() // VFS::Mount
}

/// C: VFS::Unmount (user/user_vfs.h:91)
#[allow(unused_variables, non_snake_case)]
pub fn vfs_unmount(path: *const FilePath) -> Status {
    todo!() // VFS::Unmount
}

/// C: VFS::ContainsBuffer (user/user_vfs.h:94)
#[allow(unused_variables, non_snake_case)]
pub fn vfs_contains_buffer(name: *const i8) -> bool {
    todo!() // VFS::ContainsBuffer
}

/// C: VFS::ContainsFile (user/user_vfs.h:97)
/// Calls: FilePath::Lower, FilePath::StripPath
#[allow(unused_variables, non_snake_case)]
pub fn vfs_contains_file(directory: *const i8, filename: *const i8) -> bool {
    todo!() // VFS::ContainsFile
}

/// C: VFS::SetToSelfDestruct (user/user_vfs.h:105)
#[allow(unused_variables, non_snake_case)]
pub fn vfs_set_to_self_destruct(destructor: *const ()) {
    todo!() // VFS::SetToSelfDestruct
}

/// C: VFS::Upcast (user/user_vfs.h:108)
#[allow(unused_variables, non_snake_case)]
pub fn vfs_upcast(vfs: *mut mjVFS) -> *mut VFS {
    todo!() // VFS::Upcast
}

/// C: VFS::CreateResource (user/user_vfs.h:113)
/// Calls: mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn vfs_create_resource(name: std__string_view, provider: *const mjpResourceProvider) -> ResourcePtr {
    todo!() // VFS::CreateResource
}

/// C: VFS::MaybeSelfDestruct (user/user_vfs.h:124)
#[allow(unused_variables, non_snake_case)]
pub fn vfs_maybe_self_destruct() {
    todo!() // VFS::MaybeSelfDestruct
}

