//! Port of: user/user_cache.cc
//! IR hash: 05737965add36adb
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mjCCache::HasAsset (user/user_cache.cc:53)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_cache_has_asset(self_ptr: *mut mjCCache, id: *const i32) -> *const i32 {
    if self_ptr.is_null() { return core::ptr::null(); }
    extern "C" { fn mjCCache_HasAsset(self_ptr: *mut mjCCache, id: *const i32) -> *const i32; }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCCache_HasAsset(self_ptr, id) }
}

/// C: mjCCache::Insert (user/user_cache.cc:67)
/// Calls: mjCAsset::BytesCount, mjCAsset::ReplaceData, mjCAsset::SetInsertNum
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_cache_insert(self_ptr: *mut mjCCache, modelname: *const i32, id: *const i32, resource: *const mjResource, data: *const (), size: usize) -> bool {
    if self_ptr.is_null() { return false; }
    extern "C" { fn mj_c_cache_insert(self_ptr: *mut mjCCache, modelname: *const i32, id: *const i32, resource: *const mjResource, data: *const (), size: usize) -> bool; }
    // SAFETY: self_ptr verified non-null; delegates to C implementation
    unsafe { mj_c_cache_insert(self_ptr, modelname, id, resource, data, size) }
}

/// C: mjCCache::PopulateData (user/user_cache.cc:107)
/// Calls: mjCAsset::IncrementAccess, mjCAsset::PopulateData
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_cache_populate_data(self_ptr: *mut mjCCache, id: *const i32, resource: *const mjResource, r#fn: mjCDataFunc) -> bool {
    if self_ptr.is_null() { return false; }
    extern "C" { fn mjCCache_PopulateData(self_ptr: *mut mjCCache, id: *const i32, resource: *const mjResource, r#fn: mjCDataFunc) -> bool; }
    // SAFETY: self_ptr verified non-null; delegates to C implementation
    unsafe { mjCCache_PopulateData(self_ptr, id, resource, r#fn) }
}

/// C: mjCCache::RemoveModel (user/user_cache.cc:131)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_cache_remove_model(self_ptr: *mut mjCCache, filename: *const i32) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCCache_RemoveModel(self_ptr: *mut mjCCache, filename: *const i32); }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCCache_RemoveModel(self_ptr, filename) }
}

/// C: mjCCache::DeleteAsset (user/user_cache.cc:182)
/// Calls: mjCCache::Delete
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_cache_delete_asset(self_ptr: *mut mjCCache, id: *const i32) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCCache_DeleteAsset(self_ptr: *mut mjCCache, id: *const i32); }
    // SAFETY: delegates to C implementation
    unsafe { mjCCache_DeleteAsset(self_ptr, id) }
}

/// C: mjCAsset::Timestamp (user/user_cache.h:55)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_asset_timestamp(self_ptr: *mut mjCAsset) -> *const i32 {
    if self_ptr.is_null() { return core::ptr::null(); }
    extern "C" { fn mjCAsset_Timestamp(self_ptr: *mut mjCAsset) -> *const i32; }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCAsset_Timestamp(self_ptr) }
}

/// C: mjCAsset::Id (user/user_cache.h:56)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_asset_id(self_ptr: *mut mjCAsset) -> *const i32 {
    if self_ptr.is_null() { return core::ptr::null(); }
    extern "C" { fn mjCAsset_Id(self_ptr: *mut mjCAsset) -> *const i32; }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCAsset_Id(self_ptr) }
}

/// C: mjCAsset::InsertNum (user/user_cache.h:57)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_asset_insert_num(self_ptr: *mut mjCAsset) -> std__size_t {
    if self_ptr.is_null() { panic!("mj_c_asset_insert_num: null self_ptr"); }
    extern "C" { fn mjCAsset_InsertNum(self_ptr: *mut mjCAsset) -> std__size_t; }
    // SAFETY: self_ptr verified non-null, opaque struct return delegated to C
    unsafe { mjCAsset_InsertNum(self_ptr) }
}

/// C: mjCAsset::AccessCount (user/user_cache.h:58)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_asset_access_count(self_ptr: *mut mjCAsset) -> std__size_t {
    if self_ptr.is_null() { panic!("mj_c_asset_access_count: null self_ptr"); }
    extern "C" { fn mjCAsset_AccessCount(self_ptr: *mut mjCAsset) -> std__size_t; }
    // SAFETY: self_ptr verified non-null, opaque struct return delegated to C
    unsafe { mjCAsset_AccessCount(self_ptr) }
}

/// C: mjCAsset::PopulateData (user/user_cache.h:61)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_asset_populate_data(self_ptr: *mut mjCAsset, r#fn: mjCDataFunc) -> bool {
    if self_ptr.is_null() { return false; }
    extern "C" { fn mjCAsset_PopulateData(self_ptr: *mut mjCAsset, r#fn: mjCDataFunc) -> bool; }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCAsset_PopulateData(self_ptr, r#fn) }
}

/// C: mjCAsset::AddReference (user/user_cache.h:69)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_asset_add_reference(self_ptr: *mut mjCAsset, xml_file: i32) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCAsset_AddReference(self_ptr: *mut mjCAsset, xml_file: i32); }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCAsset_AddReference(self_ptr, xml_file) }
}

/// C: mjCAsset::RemoveReference (user/user_cache.h:70)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_asset_remove_reference(self_ptr: *mut mjCAsset, xml_file: *const i32) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCAsset_RemoveReference(self_ptr: *mut mjCAsset, xml_file: *const i32); }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCAsset_RemoveReference(self_ptr, xml_file) }
}

/// C: mjCAsset::ReplaceData (user/user_cache.h:74)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_asset_replace_data(self_ptr: *mut mjCAsset, other: *const mjCAsset) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCAsset_ReplaceData(self_ptr: *mut mjCAsset, other: *const mjCAsset); }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCAsset_ReplaceData(self_ptr, other) }
}

/// C: mjCAsset::HasReferences (user/user_cache.h:79)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_asset_has_references(self_ptr: *mut mjCAsset) -> bool {
    if self_ptr.is_null() { return false; }
    extern "C" { fn mjCAsset_HasReferences(self_ptr: *mut mjCAsset) -> bool; }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCAsset_HasReferences(self_ptr) }
}

/// C: mjCAsset::IncrementAccess (user/user_cache.h:81)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_asset_increment_access(self_ptr: *mut mjCAsset) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCAsset_IncrementAccess(self_ptr: *mut mjCAsset); }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCAsset_IncrementAccess(self_ptr) }
}

/// C: mjCAsset::Copy (user/user_cache.h:84)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_asset_copy(other: *const mjCAsset) -> mjCAsset {
    if other.is_null() { panic!("mj_c_asset_copy: null other"); }
    extern "C" { fn mjCAsset_Copy(other: *const mjCAsset) -> mjCAsset; }
    // SAFETY: other verified non-null, opaque struct return delegated to C
    unsafe { mjCAsset_Copy(other) }
}

/// C: mjCAsset::SetInsertNum (user/user_cache.h:87)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_asset_set_insert_num(self_ptr: *mut mjCAsset, num: usize) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCAsset_SetInsertNum(self_ptr: *mut mjCAsset, num: usize); }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCAsset_SetInsertNum(self_ptr, num) }
}

/// C: mjCAsset::SetTimestamp (user/user_cache.h:88)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_asset_set_timestamp(self_ptr: *mut mjCAsset, timestamp: i32) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCAsset_SetTimestamp(self_ptr: *mut mjCAsset, timestamp: i32); }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCAsset_SetTimestamp(self_ptr, timestamp) }
}

/// C: mjCAsset::BytesCount (user/user_cache.h:91)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_asset_bytes_count(self_ptr: *mut mjCAsset) -> std__size_t {
    if self_ptr.is_null() { panic!("mj_c_asset_bytes_count: null self_ptr"); }
    extern "C" { fn mjCAsset_BytesCount(self_ptr: *mut mjCAsset) -> std__size_t; }
    // SAFETY: self_ptr verified non-null, opaque struct return delegated to C
    unsafe { mjCAsset_BytesCount(self_ptr) }
}

/// C: mjCAsset::Data (user/user_cache.h:92)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_asset_data(self_ptr: *mut mjCAsset) -> *const () {
    if self_ptr.is_null() { return core::ptr::null(); }
    extern "C" { fn mjCAsset_Data(self_ptr: *mut mjCAsset) -> *const (); }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCAsset_Data(self_ptr) }
}

/// C: mjCAsset::References (user/user_cache.h:95)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_asset_references(self_ptr: *mut mjCAsset) -> *const i32 {
    if self_ptr.is_null() { return core::ptr::null(); }
    extern "C" { fn mjCAsset_References(self_ptr: *mut mjCAsset) -> *const i32; }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCAsset_References(self_ptr) }
}

/// C: mjCCache::SetCapacity (user/user_cache.h:131)
/// Calls: mjCCache::Trim
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_cache_set_capacity(self_ptr: *mut mjCCache, size: usize) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCCache_SetCapacity(self_ptr: *mut mjCCache, size: usize); }
    // SAFETY: self_ptr verified non-null; delegates to C implementation
    unsafe { mjCCache_SetCapacity(self_ptr, size) }
}

/// C: mjCCache::Reset (user/user_cache.h:156)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_cache_reset(self_ptr: *mut mjCCache) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCCache_Reset(self_ptr: *mut mjCCache); }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCCache_Reset(self_ptr) }
}

/// C: mjCCache::Capacity (user/user_cache.h:159)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_cache_capacity(self_ptr: *mut mjCCache) -> std__size_t {
    if self_ptr.is_null() {
        return unsafe { core::mem::zeroed() };
    }
    extern "C" { fn mjCCache_Capacity(self_ptr: *mut mjCCache) -> std__size_t; }
    // SAFETY: self_ptr verified non-null; delegates to C++ getter
    unsafe { mjCCache_Capacity(self_ptr) }
}

/// C: mjCCache::Size (user/user_cache.h:160)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_cache_size(self_ptr: *mut mjCCache) -> std__size_t {
    if self_ptr.is_null() { panic!("mj_c_cache_size: null self_ptr"); }
    extern "C" { fn mjCCache_Size(self_ptr: *mut mjCCache) -> std__size_t; }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCCache_Size(self_ptr) }
}

/// C: mjCCache::Delete (user/user_cache.h:163)
/// Calls: mjCAsset::BytesCount
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_cache_delete(self_ptr: *mut mjCCache, asset: *mut mjCAsset) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCCache_Delete(self_ptr: *mut mjCCache, asset: *mut mjCAsset); }
    // SAFETY: self_ptr verified non-null; delegates to C implementation
    unsafe { mjCCache_Delete(self_ptr, asset) }
}

/// C: mjCCache::Trim (user/user_cache.h:165)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_cache_trim(self_ptr: *mut mjCCache) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCCache_Trim(self_ptr: *mut mjCCache); }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCCache_Trim(self_ptr) }
}

