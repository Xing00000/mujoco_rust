//! Port of: user/user_vfs.cc
//! IR hash: 05737965add36adb
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: OpenFile (user/user_vfs.cc:50)
#[allow(unused_variables, non_snake_case)]
pub fn open_file(filename: *const i8, resource: *mut mjResource) -> i32 {
    extern "C" { fn OpenFile_impl(filename: *const i8, resource: *mut mjResource) -> i32; }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { OpenFile_impl(filename, resource) }
}

/// C: ReadFile (user/user_vfs.cc:64)
#[allow(unused_variables, non_snake_case)]
pub fn read_file(filename: *const i8, resource: *mut mjResource, buffer: *const *mut ()) -> i32 {
    extern "C" { fn ReadFile_impl(filename: *const i8, resource: *mut mjResource, buffer: *const *mut ()) -> i32; }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { ReadFile_impl(filename, resource, buffer) }
}

/// C: CloseFile (user/user_vfs.cc:74)
#[allow(unused_variables, non_snake_case)]
pub fn close_file(resource: *mut mjResource) {
    // WARNING: signature changed — verify body
    // Previous params: (resource : * mut mjResource)
    // Previous return: ()
    extern "C" { fn CloseFile_impl (resource : * mut mjResource) ; } unsafe { CloseFile_impl (resource) }
}

/// C: FileModified (user/user_vfs.cc:79)
/// Calls: mju_decodeBase64
#[allow(unused_variables, non_snake_case)]
pub fn file_modified(resource: *const mjResource, timestamp: *const i8) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (resource : * const mjResource, timestamp : * const i8)
    // Previous return: i32
    extern "C" { fn FileModified_impl (resource : * const mjResource , timestamp : * const i8) -> i32 ; } unsafe { FileModified_impl (resource , timestamp) }
}

/// C: StripPathAndLower (user/user_vfs.cc:94)
#[allow(unused_variables, non_snake_case)]
pub fn strip_path_and_lower(path: i32) -> i32 {
    extern "C" { fn StripPathAndLower_impl(path: i32) -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { StripPathAndLower_impl(path) }
}

/// C: VFS::FindMount (user/user_vfs.cc:287)
#[allow(unused_variables, non_snake_case)]
pub fn vfs_find_mount(self_ptr: *mut VFS, fullpath: *const i32) -> *mut mjResource {
    extern "C" { fn VFS_FindMount_impl(self_ptr: *mut VFS, fullpath: *const i32) -> *mut mjResource; }
    // SAFETY: delegates to C implementation
    unsafe { VFS_FindMount_impl(self_ptr, fullpath) }
}

/// C: mj_defaultVFS (user/user_vfs.cc:355)
/// Calls: mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mj_default_vfs(vfs: *mut mjVFS) {
    extern "C" { fn mj_defaultVFS_impl(vfs: *mut mjVFS); }
    // SAFETY: delegates to C implementation
    unsafe { mj_defaultVFS_impl(vfs) }
}

/// C: mj_deleteVFS (user/user_vfs.cc:363)
/// Calls: VFS::Upcast
#[allow(unused_variables, non_snake_case)]
pub fn mj_delete_vfs(vfs: *mut mjVFS) {
    extern "C" { fn mj_deleteVFS_impl(vfs: *mut mjVFS); }
    // SAFETY: delegates to C implementation
    unsafe { mj_deleteVFS_impl(vfs) }
}

/// C: mj_mountVFS (user/user_vfs.cc:371)
/// Calls: VFS::Mount, VFS::Upcast, mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mj_mount_vfs(vfs: *mut mjVFS, filepath: *const i8, provider: *const mjpResourceProvider) -> i32 {
    extern "C" { fn mj_mountVFS_impl(vfs: *mut mjVFS, filepath: *const i8, provider: *const mjpResourceProvider) -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { mj_mountVFS_impl(vfs, filepath, provider) }
}

/// C: mj_unmountVFS (user/user_vfs.cc:386)
/// Calls: VFS::Unmount, VFS::Upcast, mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mj_unmount_vfs(vfs: *mut mjVFS, filename: *const i8) -> i32 {
    extern "C" { fn mj_unmountVFS_impl(vfs: *mut mjVFS, filename: *const i8) -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { mj_unmountVFS_impl(vfs, filename) }
}

/// C: BufferProvider::Mount (user/user_vfs.cc:407)
/// Calls: VFS::Mount, VFS::Upcast, mju_decodeBase64, mju_encodeBase64, mju_error, mju_isValidBase64
#[allow(unused_variables, non_snake_case)]
pub fn buffer_provider_mount(vfs: *mut mjVFS, args: Args) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (vfs : * mut mjVFS, args : Args)
    // Previous return: i32
    extern "C" { fn BufferProvider_Mount_impl (vfs : * mut mjVFS , args : Args) -> i32 ; } unsafe { BufferProvider_Mount_impl (vfs , args) }
}

/// C: mj_addFileVFS (user/user_vfs.cc:496)
#[allow(unused_variables, non_snake_case)]
pub fn mj_add_file_vfs(vfs: *mut mjVFS, directory: *const i8, filename: *const i8) -> i32 {
    extern "C" { fn mj_addFileVFS_impl(vfs: *mut mjVFS, directory: *const i8, filename: *const i8) -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { mj_addFileVFS_impl(vfs, directory, filename) }
}

/// C: mj_addBufferVFS (user/user_vfs.cc:503)
#[allow(unused_variables, non_snake_case)]
pub fn mj_add_buffer_vfs(vfs: *mut mjVFS, name: *const i8, buffer: *const (), nbuffer: i32) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (vfs : * mut mjVFS, name : * const i8, buffer : * const (), nbuffer : i32)
    // Previous return: i32
    todo ! ()
}

/// C: mj_deleteFileVFS (user/user_vfs.cc:508)
/// Calls: FilePath::Lower, FilePath::StripPath, FilePath::c_str, mj_unmountVFS
#[allow(unused_variables, non_snake_case)]
pub fn mj_delete_file_vfs(vfs: *mut mjVFS, filename: *const i8) -> i32 {
    extern "C" { fn mj_deleteFileVFS_impl(vfs: *mut mjVFS, filename: *const i8) -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { mj_deleteFileVFS_impl(vfs, filename) }
}

/// C: mj_containsBufferVFS (user/user_vfs.cc:520)
/// Calls: VFS::ContainsBuffer, VFS::Upcast, mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mj_contains_buffer_vfs(vfs: *mut mjVFS, name: *const i8) -> i32 {
    extern "C" { fn mj_containsBufferVFS_impl(vfs: *mut mjVFS, name: *const i8) -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { mj_containsBufferVFS_impl(vfs, name) }
}

/// C: mj_containsFileVFS (user/user_vfs.cc:529)
/// Calls: VFS::ContainsFile, VFS::Upcast, mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mj_contains_file_vfs(vfs: *mut mjVFS, directory: *const i8, filename: *const i8) -> i32 {
    extern "C" { fn mj_containsFileVFS_impl(vfs: *mut mjVFS, directory: *const i8, filename: *const i8) -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { mj_containsFileVFS_impl(vfs, directory, filename) }
}

/// C: VFS::Open (user/user_vfs.h:75)
/// Calls: VFS::MaybeSelfDestruct
#[allow(unused_variables, non_snake_case)]
pub fn vfs_open(self_ptr: *mut VFS, dir: *const i8, name: *const i8) -> *mut mjResource {
    extern "C" { fn VFS_Open_impl(self_ptr: *mut VFS, dir: *const i8, name: *const i8) -> *mut mjResource; }
    // SAFETY: delegates to C implementation
    unsafe { VFS_Open_impl(self_ptr, dir, name) }
}

/// C: VFS::Read (user/user_vfs.h:80)
#[allow(unused_variables, non_snake_case)]
pub fn vfs_read(self_ptr: *mut VFS, resource: *mut mjResource, buffer: *const *mut ()) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut VFS, resource : * mut mjResource, buffer : * const * mut ())
    // Previous return: i32
    extern "C" { fn VFS_Read_impl (self_ptr : * mut VFS , resource : * mut mjResource , buffer : * const * mut ()) -> i32 ; } unsafe { VFS_Read_impl (self_ptr , resource , buffer) }
}

/// C: VFS::Close (user/user_vfs.h:84)
/// Calls: VFS::MaybeSelfDestruct
#[allow(unused_variables, non_snake_case)]
pub fn vfs_close(self_ptr: *mut VFS, resource: *mut mjResource) -> Status {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut VFS, resource : * mut mjResource)
    // Previous return: Status
    extern "C" { fn VFS_Close_impl (self_ptr : * mut VFS , resource : * mut mjResource) -> Status ; } unsafe { VFS_Close_impl (self_ptr , resource) }
}

/// C: VFS::Mount (user/user_vfs.h:88)
#[allow(unused_variables, non_snake_case)]
pub fn vfs_mount(self_ptr: *mut VFS, path: *const FilePath, provider: *const mjpResourceProvider) -> Status {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut VFS, path : * const FilePath, provider : * const mjpResourceProvider)
    // Previous return: Status
    extern "C" { fn VFS_Mount_impl (self_ptr : * mut VFS , path : * const FilePath , provider : * const mjpResourceProvider) -> Status ; } unsafe { VFS_Mount_impl (self_ptr , path , provider) }
}

/// C: VFS::Unmount (user/user_vfs.h:91)
#[allow(unused_variables, non_snake_case)]
pub fn vfs_unmount(self_ptr: *mut VFS, path: *const FilePath) -> Status {
    extern "C" { fn VFS_Unmount_impl(self_ptr: *mut VFS, path: *const FilePath) -> Status; }
    // SAFETY: delegates to C implementation
    unsafe { VFS_Unmount_impl(self_ptr, path) }
}

/// C: VFS::ContainsBuffer (user/user_vfs.h:94)
#[allow(unused_variables, non_snake_case)]
pub fn vfs_contains_buffer(self_ptr: *mut VFS, name: *const i8) -> bool {
    extern "C" { fn VFS_ContainsBuffer_impl(self_ptr: *mut VFS, name: *const i8) -> bool; }
    // SAFETY: delegates to C implementation
    unsafe { VFS_ContainsBuffer_impl(self_ptr, name) }
}

/// C: VFS::ContainsFile (user/user_vfs.h:97)
/// Calls: FilePath::Lower, FilePath::StripPath
#[allow(unused_variables, non_snake_case)]
pub fn vfs_contains_file(self_ptr: *mut VFS, directory: *const i8, filename: *const i8) -> bool {
    extern "C" { fn VFS_ContainsFile_impl(self_ptr: *mut VFS, directory: *const i8, filename: *const i8) -> bool; }
    // SAFETY: delegates to C implementation
    unsafe { VFS_ContainsFile_impl(self_ptr, directory, filename) }
}

/// C: VFS::SetToSelfDestruct (user/user_vfs.h:105)
#[allow(unused_variables, non_snake_case)]
pub fn vfs_set_to_self_destruct(self_ptr: *mut VFS, destructor: *const ()) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut VFS, destructor : * const ())
    // Previous return: ()
    todo ! ()
}

/// C: VFS::Upcast (user/user_vfs.h:108)
#[allow(unused_variables, non_snake_case)]
pub fn vfs_upcast(vfs: *mut mjVFS) -> *mut VFS {
    // WARNING: signature changed — verify body
    // Previous params: (vfs : * mut mjVFS)
    // Previous return: * mut VFS
    extern "C" { fn VFS_Upcast_impl (vfs : * mut mjVFS) -> * mut VFS ; } unsafe { VFS_Upcast_impl (vfs) }
}

/// C: VFS::CreateResource (user/user_vfs.h:113)
/// Calls: mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn vfs_create_resource(self_ptr: *mut VFS, name: string_view, provider: *const mjpResourceProvider) -> ResourcePtr {
    extern "C" { fn VFS_CreateResource_impl(self_ptr: *mut VFS, name: string_view, provider: *const mjpResourceProvider) -> ResourcePtr; }
    // SAFETY: delegates to C implementation
    unsafe { VFS_CreateResource_impl(self_ptr, name, provider) }
}

/// C: VFS::MaybeSelfDestruct (user/user_vfs.h:124)
#[allow(unused_variables, non_snake_case)]
pub fn vfs_maybe_self_destruct(self_ptr: *mut VFS) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut VFS)
    // Previous return: ()
    extern "C" { fn VFS_MaybeSelfDestruct_impl (self_ptr : * mut VFS) ; } unsafe { VFS_MaybeSelfDestruct_impl (self_ptr) }
}

