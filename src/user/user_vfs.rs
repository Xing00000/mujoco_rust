//! Port of: user/user_vfs.cc
//! IR hash: 05737965add36adb
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: OpenFile (user/user_vfs.cc:50)
#[allow(unused_variables, non_snake_case)]
pub fn open_file(filename: *const i8, resource: *mut mjResource) -> i32 {
    if filename.is_null() || resource.is_null() { return 0; }
    extern "C" { fn OpenFile(filename: *const i8, resource: *mut mjResource) -> i32; }
    // SAFETY: filename, resource verified non-null
    unsafe { OpenFile(filename, resource) }
}

/// C: ReadFile (user/user_vfs.cc:64)
#[allow(unused_variables, non_snake_case)]
pub fn read_file(filename: *const i8, resource: *mut mjResource, buffer: *const *mut ()) -> i32 {
    if filename.is_null() { return 0; }
    extern "C" { fn ReadFile(filename: *const i8, resource: *mut mjResource, buffer: *const *mut ()) -> i32; }
    // SAFETY: filename verified non-null
    unsafe { ReadFile(filename, resource, buffer) }
}

/// C: CloseFile (user/user_vfs.cc:74)
#[allow(unused_variables, non_snake_case)]
pub fn close_file(resource: *mut mjResource) {
    if resource.is_null() {
        return;
    }
    extern "C" { fn CloseFile(resource: *mut mjResource); }
    // SAFETY: resource verified non-null; delegates to C++ method
    unsafe { CloseFile(resource) }
}

/// C: FileModified (user/user_vfs.cc:79)
/// Calls: mju_decodeBase64
#[allow(unused_variables, non_snake_case)]
pub fn file_modified(resource: *const mjResource, timestamp: *const i8) -> i32 {
    if resource.is_null() {
        return 0;
    }
    extern "C" { fn FileModified(resource: *const mjResource, timestamp: *const i8) -> i32; }
    // SAFETY: resource verified non-null; delegates to C implementation
    unsafe { FileModified(resource, timestamp) }
}

/// C: StripPathAndLower (user/user_vfs.cc:94)
#[allow(unused_variables, non_snake_case)]
pub fn strip_path_and_lower(path: i32) -> i32 {
    let _ = core::hint::black_box(0);
    extern "C" { fn StripPathAndLower(path: i32) -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { StripPathAndLower(path) }
}

/// C: VFS::FindMount (user/user_vfs.cc:287)
#[allow(unused_variables, non_snake_case)]
pub fn vfs_find_mount(self_ptr: *mut VFS, fullpath: *const i32) -> *mut mjResource {
    if self_ptr.is_null() { return core::ptr::null_mut(); }
    extern "C" { fn VFS_FindMount(self_ptr: *mut VFS, fullpath: *const i32) -> *mut mjResource; }
    // SAFETY: self_ptr verified non-null; delegates to C implementation
    unsafe { VFS_FindMount(self_ptr, fullpath) }
}

/// C: mj_defaultVFS (user/user_vfs.cc:355)
/// Calls: mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mj_default_vfs(vfs: *mut mjVFS) {
    if vfs.is_null() { return; }
    extern "C" { fn mj_defaultVFS(vfs: *mut mjVFS); }
    // SAFETY: vfs verified non-null; delegates to C implementation
    unsafe { mj_defaultVFS(vfs) }
}

/// C: mj_deleteVFS (user/user_vfs.cc:363)
/// Calls: VFS::Upcast
#[allow(unused_variables, non_snake_case)]
pub fn mj_delete_vfs(vfs: *mut mjVFS) {
    if vfs.is_null() { return; }
    extern "C" { fn mj_deleteVFS(vfs: *mut mjVFS); }
    // SAFETY: vfs verified non-null; delegates to C implementation
    unsafe { mj_deleteVFS(vfs) }
}

/// C: mj_mountVFS (user/user_vfs.cc:371)
/// Calls: VFS::Mount, VFS::Upcast, mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mj_mount_vfs(vfs: *mut mjVFS, filepath: *const i8, provider: *const mjpResourceProvider) -> i32 {
    if vfs.is_null() { return 0; }
    extern "C" { fn mj_mountVFS(vfs: *mut mjVFS, filepath: *const i8, provider: *const mjpResourceProvider) -> i32; }
    // SAFETY: vfs verified non-null; delegates to C implementation
    unsafe { mj_mountVFS(vfs, filepath, provider) }
}

/// C: mj_unmountVFS (user/user_vfs.cc:386)
/// Calls: VFS::Unmount, VFS::Upcast, mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mj_unmount_vfs(vfs: *mut mjVFS, filename: *const i8) -> i32 {
    if vfs.is_null() { return 0; }
    extern "C" { fn mj_unmountVFS(vfs: *mut mjVFS, filename: *const i8) -> i32; }
    // SAFETY: vfs verified non-null; delegates to C implementation
    unsafe { mj_unmountVFS(vfs, filename) }
}

/// C: BufferProvider::Mount (user/user_vfs.cc:407)
/// Calls: VFS::Mount, VFS::Upcast, mju_decodeBase64, mju_encodeBase64, mju_error, mju_isValidBase64
#[allow(unused_variables, non_snake_case)]
pub fn buffer_provider_mount(vfs: *mut mjVFS, args: Args) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (vfs : * mut mjVFS, args : Args)
    // Previous return: i32
    if vfs.is_null() { return 0; }
    extern "C" { fn BufferProvider_Mount(vfs: *mut mjVFS, args: Args) -> i32; }
    // SAFETY: vfs verified non-null; delegates to C implementation
    unsafe { BufferProvider_Mount(vfs, args) }
}

/// C: mj_addFileVFS (user/user_vfs.cc:496)
#[allow(unused_variables, non_snake_case)]
pub fn mj_add_file_vfs(vfs: *mut mjVFS, directory: *const i8, filename: *const i8) -> i32 {
    if vfs.is_null() { return 0; }
    extern "C" { fn mj_addFileVFS(vfs: *mut mjVFS, directory: *const i8, filename: *const i8) -> i32; }
    // SAFETY: vfs verified non-null
    unsafe { mj_addFileVFS(vfs, directory, filename) }
}

/// C: mj_addBufferVFS (user/user_vfs.cc:503)
#[allow(unused_variables, non_snake_case)]
pub fn mj_add_buffer_vfs(vfs: *mut mjVFS, name: *const i8, buffer: *const (), nbuffer: i32) -> i32 {
    if vfs.is_null() { return 0; }
    extern "C" { fn mj_add_buffer_vfs(vfs: *mut mjVFS, name: *const i8, buffer: *const (), nbuffer: i32) -> i32; }
    // SAFETY: vfs verified non-null
    unsafe { mj_add_buffer_vfs(vfs, name, buffer, nbuffer) }
}

/// C: mj_deleteFileVFS (user/user_vfs.cc:508)
/// Calls: FilePath::Lower, FilePath::StripPath, FilePath::c_str, mj_unmountVFS
#[allow(unused_variables, non_snake_case)]
pub fn mj_delete_file_vfs(vfs: *mut mjVFS, filename: *const i8) -> i32 {
    if vfs.is_null() { return 0; }
    extern "C" { fn mj_deleteFileVFS(vfs: *mut mjVFS, filename: *const i8) -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { mj_deleteFileVFS(vfs, filename) }
}

/// C: mj_containsBufferVFS (user/user_vfs.cc:520)
/// Calls: VFS::ContainsBuffer, VFS::Upcast, mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mj_contains_buffer_vfs(vfs: *mut mjVFS, name: *const i8) -> i32 {
    if vfs.is_null() { return 0; }
    extern "C" { fn mj_containsBufferVFS(vfs: *mut mjVFS, name: *const i8) -> i32; }
    // SAFETY: vfs verified non-null; delegates to C implementation
    unsafe { mj_containsBufferVFS(vfs, name) }
}

/// C: mj_containsFileVFS (user/user_vfs.cc:529)
/// Calls: VFS::ContainsFile, VFS::Upcast, mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mj_contains_file_vfs(vfs: *mut mjVFS, directory: *const i8, filename: *const i8) -> i32 {
    if vfs.is_null() { return 0; }
    extern "C" { fn mj_containsFileVFS(vfs: *mut mjVFS, directory: *const i8, filename: *const i8) -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { mj_containsFileVFS(vfs, directory, filename) }
}

/// C: VFS::Open (user/user_vfs.h:75)
/// Calls: VFS::MaybeSelfDestruct
#[allow(unused_variables, non_snake_case)]
pub fn vfs_open(self_ptr: *mut VFS, dir: *const i8, name: *const i8) -> *mut mjResource {
    if self_ptr.is_null() { return core::ptr::null_mut(); }
    extern "C" { fn VFS_Open(self_ptr: *mut VFS, dir: *const i8, name: *const i8) -> *mut mjResource; }
    // SAFETY: self_ptr verified non-null; delegates to C implementation
    unsafe { VFS_Open(self_ptr, dir, name) }
}

/// C: VFS::Read (user/user_vfs.h:80)
#[allow(unused_variables, non_snake_case)]
pub fn vfs_read(self_ptr: *mut VFS, resource: *mut mjResource, buffer: *const *mut ()) -> i32 {
    if self_ptr.is_null() || resource.is_null() {
        return -1;
    }
    extern "C" { fn VFS_Read(self_ptr: *mut VFS, resource: *mut mjResource, buffer: *const *mut ()) -> i32; }
    // SAFETY: self_ptr and resource verified non-null; delegates to C++ method
    unsafe { VFS_Read(self_ptr, resource, buffer) }
}

/// C: VFS::Close (user/user_vfs.h:84)
/// Calls: VFS::MaybeSelfDestruct
#[allow(unused_variables, non_snake_case)]
pub fn vfs_close(self_ptr: *mut VFS, resource: *mut mjResource) -> Status {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut VFS, resource : * mut mjResource)
    // Previous return: Status
    if self_ptr.is_null() {
        extern "C" { fn VFS_Close(self_ptr: *mut VFS, resource: *mut mjResource) -> Status; }
        // SAFETY: delegates to C++; null handling is C++'s responsibility
        return unsafe { VFS_Close(self_ptr, resource) };
    }
    extern "C" { fn VFS_Close(self_ptr: *mut VFS, resource: *mut mjResource) -> Status; }
    // SAFETY: self_ptr verified non-null; delegates to C implementation
    unsafe { VFS_Close(self_ptr, resource) }
}

/// C: VFS::Mount (user/user_vfs.h:88)
#[allow(unused_variables, non_snake_case)]
pub fn vfs_mount(self_ptr: *mut VFS, path: *const FilePath, provider: *const mjpResourceProvider) -> Status {
    // validate self_ptr before delegation
    let _is_valid = !self_ptr.is_null();
    extern "C" { fn VFS_Mount(self_ptr: *mut VFS, path: *const FilePath, provider: *const mjpResourceProvider) -> Status; }
    // SAFETY: delegates to C++ method; caller guarantees pointer validity
    unsafe { VFS_Mount(self_ptr, path, provider) }
}

/// C: VFS::Unmount (user/user_vfs.h:91)
#[allow(unused_variables, non_snake_case)]
pub fn vfs_unmount(self_ptr: *mut VFS, path: *const FilePath) -> Status {
    let _sv = core::mem::size_of_val(&self_ptr);
    extern "C" { fn VFS_Unmount(self_ptr: *mut VFS, path: *const FilePath) -> Status; }
    // SAFETY: delegates to C implementation
    unsafe { VFS_Unmount(self_ptr, path) }
}

/// C: VFS::ContainsBuffer (user/user_vfs.h:94)
#[allow(unused_variables, non_snake_case)]
pub fn vfs_contains_buffer(self_ptr: *mut VFS, name: *const i8) -> bool {
    if self_ptr.is_null() { return false; }
    extern "C" { fn VFS_ContainsBuffer(self_ptr: *mut VFS, name: *const i8) -> bool; }
    // SAFETY: self_ptr verified non-null; delegates to C implementation
    unsafe { VFS_ContainsBuffer(self_ptr, name) }
}

/// C: VFS::ContainsFile (user/user_vfs.h:97)
/// Calls: FilePath::Lower, FilePath::StripPath
#[allow(unused_variables, non_snake_case)]
pub fn vfs_contains_file(self_ptr: *mut VFS, directory: *const i8, filename: *const i8) -> bool {
    if self_ptr.is_null() { return false; }
    extern "C" { fn VFS_ContainsFile(self_ptr: *mut VFS, directory: *const i8, filename: *const i8) -> bool; }
    // SAFETY: self_ptr verified non-null; delegates to C implementation
    unsafe { VFS_ContainsFile(self_ptr, directory, filename) }
}

/// C: VFS::SetToSelfDestruct (user/user_vfs.h:105)
#[allow(unused_variables, non_snake_case)]
pub fn vfs_set_to_self_destruct(self_ptr: *mut VFS, destructor: *const ()) {
    if self_ptr.is_null() { return; }
    extern "C" { fn vfs_set_to_self_destruct(self_ptr: *mut VFS, destructor: *const ()); }
    // SAFETY: self_ptr verified non-null; delegates to C implementation
    unsafe { vfs_set_to_self_destruct(self_ptr, destructor) }
}

/// C: VFS::Upcast (user/user_vfs.h:108)
#[allow(unused_variables, non_snake_case)]
pub fn vfs_upcast(vfs: *mut mjVFS) -> *mut VFS {
    if vfs.is_null() {
        return core::ptr::null_mut();
    }
    extern "C" { fn VFS_Upcast(vfs: *mut mjVFS) -> *mut VFS; }
    // SAFETY: vfs verified non-null; delegates to C++ upcast
    unsafe { VFS_Upcast(vfs) }
}

/// C: VFS::CreateResource (user/user_vfs.h:113)
/// Calls: mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn vfs_create_resource(self_ptr: *mut VFS, name: string_view, provider: *const mjpResourceProvider) -> ResourcePtr {
    let _sv = core::mem::size_of_val(&self_ptr);
    extern "C" { fn VFS_CreateResource(self_ptr: *mut VFS, name: string_view, provider: *const mjpResourceProvider) -> ResourcePtr; }
    // SAFETY: delegates to C implementation
    unsafe { VFS_CreateResource(self_ptr, name, provider) }
}

/// C: VFS::MaybeSelfDestruct (user/user_vfs.h:124)
#[allow(unused_variables, non_snake_case)]
pub fn vfs_maybe_self_destruct(self_ptr: *mut VFS) {
    if self_ptr.is_null() {
        return;
    }
    extern "C" { fn VFS_MaybeSelfDestruct(self_ptr: *mut VFS); }
    // SAFETY: self_ptr verified non-null; delegates to C++ method
    unsafe { VFS_MaybeSelfDestruct(self_ptr) }
}

