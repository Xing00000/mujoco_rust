//! Port of: user/user_cache.cc
//! IR hash: 05737965add36adb
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mjCCache::HasAsset (user/user_cache.cc:53)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_cache_has_asset(self_ptr: *mut mjCCache, id: *const i32) -> *const i32 {
    extern "C" { fn mjCCache_HasAsset_impl(self_ptr: *mut mjCCache, id: *const i32) -> *const i32; }
    // SAFETY: delegates to C implementation
    unsafe { mjCCache_HasAsset_impl(self_ptr, id) }
}

/// C: mjCCache::Insert (user/user_cache.cc:67)
/// Calls: mjCAsset::BytesCount, mjCAsset::ReplaceData, mjCAsset::SetInsertNum
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_cache_insert(self_ptr: *mut mjCCache, modelname: *const i32, id: *const i32, resource: *const mjResource, data: *const (), size: usize) -> bool {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCCache, modelname : * const i32, id : * const i32, resource : * const mjResource, data : * const (), size : usize)
    // Previous return: bool
    todo ! ()
}

/// C: mjCCache::PopulateData (user/user_cache.cc:107)
/// Calls: mjCAsset::IncrementAccess, mjCAsset::PopulateData
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_cache_populate_data(self_ptr: *mut mjCCache, id: *const i32, resource: *const mjResource, r#fn: mjCDataFunc) -> bool {
    extern "C" { fn mjCCache_PopulateData_impl(self_ptr: *mut mjCCache, id: *const i32, resource: *const mjResource, r#fn: mjCDataFunc) -> bool; }
    // SAFETY: delegates to C implementation
    unsafe { mjCCache_PopulateData_impl(self_ptr, id, resource, r#fn) }
}

/// C: mjCCache::RemoveModel (user/user_cache.cc:131)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_cache_remove_model(self_ptr: *mut mjCCache, filename: *const i32) {
    extern "C" { fn mjCCache_RemoveModel_impl(self_ptr: *mut mjCCache, filename: *const i32); }
    // SAFETY: delegates to C implementation
    unsafe { mjCCache_RemoveModel_impl(self_ptr, filename) }
}

/// C: mjCCache::DeleteAsset (user/user_cache.cc:182)
/// Calls: mjCCache::Delete
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_cache_delete_asset(self_ptr: *mut mjCCache, id: *const i32) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCCache, id : * const i32)
    // Previous return: ()
    todo ! ()
}

/// C: mjCAsset::Timestamp (user/user_cache.h:55)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_asset_timestamp(self_ptr: *mut mjCAsset) -> *const i32 {
    extern "C" { fn mjCAsset_Timestamp_impl(self_ptr: *mut mjCAsset) -> *const i32; }
    // SAFETY: delegates to C implementation
    unsafe { mjCAsset_Timestamp_impl(self_ptr) }
}

/// C: mjCAsset::Id (user/user_cache.h:56)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_asset_id(self_ptr: *mut mjCAsset) -> *const i32 {
    extern "C" { fn mjCAsset_Id_impl(self_ptr: *mut mjCAsset) -> *const i32; }
    // SAFETY: delegates to C implementation
    unsafe { mjCAsset_Id_impl(self_ptr) }
}

/// C: mjCAsset::InsertNum (user/user_cache.h:57)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_asset_insert_num(self_ptr: *mut mjCAsset) -> std__size_t {
    extern "C" { fn mjCAsset_InsertNum_impl(self_ptr: *mut mjCAsset) -> std__size_t; }
    // SAFETY: delegates to C implementation
    unsafe { mjCAsset_InsertNum_impl(self_ptr) }
}

/// C: mjCAsset::AccessCount (user/user_cache.h:58)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_asset_access_count(self_ptr: *mut mjCAsset) -> std__size_t {
    extern "C" { fn mjCAsset_AccessCount_impl(self_ptr: *mut mjCAsset) -> std__size_t; }
    // SAFETY: delegates to C implementation
    unsafe { mjCAsset_AccessCount_impl(self_ptr) }
}

/// C: mjCAsset::PopulateData (user/user_cache.h:61)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_asset_populate_data(self_ptr: *mut mjCAsset, r#fn: mjCDataFunc) -> bool {
    extern "C" { fn mjCAsset_PopulateData_impl(self_ptr: *mut mjCAsset, r#fn: mjCDataFunc) -> bool; }
    // SAFETY: delegates to C implementation
    unsafe { mjCAsset_PopulateData_impl(self_ptr, r#fn) }
}

/// C: mjCAsset::AddReference (user/user_cache.h:69)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_asset_add_reference(self_ptr: *mut mjCAsset, xml_file: i32) {
    extern "C" { fn mjCAsset_AddReference_impl(self_ptr: *mut mjCAsset, xml_file: i32); }
    // SAFETY: delegates to C implementation
    unsafe { mjCAsset_AddReference_impl(self_ptr, xml_file) }
}

/// C: mjCAsset::RemoveReference (user/user_cache.h:70)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_asset_remove_reference(self_ptr: *mut mjCAsset, xml_file: *const i32) {
    extern "C" { fn mjCAsset_RemoveReference_impl(self_ptr: *mut mjCAsset, xml_file: *const i32); }
    // SAFETY: delegates to C implementation
    unsafe { mjCAsset_RemoveReference_impl(self_ptr, xml_file) }
}

/// C: mjCAsset::ReplaceData (user/user_cache.h:74)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_asset_replace_data(self_ptr: *mut mjCAsset, other: *const mjCAsset) {
    extern "C" { fn mjCAsset_ReplaceData_impl(self_ptr: *mut mjCAsset, other: *const mjCAsset); }
    // SAFETY: delegates to C implementation
    unsafe { mjCAsset_ReplaceData_impl(self_ptr, other) }
}

/// C: mjCAsset::HasReferences (user/user_cache.h:79)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_asset_has_references(self_ptr: *mut mjCAsset) -> bool {
    extern "C" { fn mjCAsset_HasReferences_impl(self_ptr: *mut mjCAsset) -> bool; }
    // SAFETY: delegates to C implementation
    unsafe { mjCAsset_HasReferences_impl(self_ptr) }
}

/// C: mjCAsset::IncrementAccess (user/user_cache.h:81)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_asset_increment_access(self_ptr: *mut mjCAsset) {
    extern "C" { fn mjCAsset_IncrementAccess_impl(self_ptr: *mut mjCAsset); }
    // SAFETY: delegates to C implementation
    unsafe { mjCAsset_IncrementAccess_impl(self_ptr) }
}

/// C: mjCAsset::Copy (user/user_cache.h:84)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_asset_copy(other: *const mjCAsset) -> mjCAsset {
    extern "C" { fn mjCAsset_Copy_impl(other: *const mjCAsset) -> mjCAsset; }
    // SAFETY: delegates to C implementation
    unsafe { mjCAsset_Copy_impl(other) }
}

/// C: mjCAsset::SetInsertNum (user/user_cache.h:87)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_asset_set_insert_num(self_ptr: *mut mjCAsset, num: usize) {
    extern "C" { fn mjCAsset_SetInsertNum_impl(self_ptr: *mut mjCAsset, num: usize); }
    // SAFETY: delegates to C implementation
    unsafe { mjCAsset_SetInsertNum_impl(self_ptr, num) }
}

/// C: mjCAsset::SetTimestamp (user/user_cache.h:88)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_asset_set_timestamp(self_ptr: *mut mjCAsset, timestamp: i32) {
    extern "C" { fn mjCAsset_SetTimestamp_impl(self_ptr: *mut mjCAsset, timestamp: i32); }
    // SAFETY: delegates to C implementation
    unsafe { mjCAsset_SetTimestamp_impl(self_ptr, timestamp) }
}

/// C: mjCAsset::BytesCount (user/user_cache.h:91)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_asset_bytes_count(self_ptr: *mut mjCAsset) -> std__size_t {
    extern "C" { fn mjCAsset_BytesCount_impl(self_ptr: *mut mjCAsset) -> std__size_t; }
    // SAFETY: delegates to C implementation
    unsafe { mjCAsset_BytesCount_impl(self_ptr) }
}

/// C: mjCAsset::Data (user/user_cache.h:92)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_asset_data(self_ptr: *mut mjCAsset) -> *const () {
    extern "C" { fn mjCAsset_Data_impl(self_ptr: *mut mjCAsset) -> *const (); }
    // SAFETY: delegates to C implementation
    unsafe { mjCAsset_Data_impl(self_ptr) }
}

/// C: mjCAsset::References (user/user_cache.h:95)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_asset_references(self_ptr: *mut mjCAsset) -> *const i32 {
    extern "C" { fn mjCAsset_References_impl(self_ptr: *mut mjCAsset) -> *const i32; }
    // SAFETY: delegates to C implementation
    unsafe { mjCAsset_References_impl(self_ptr) }
}

/// C: mjCCache::SetCapacity (user/user_cache.h:131)
/// Calls: mjCCache::Trim
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_cache_set_capacity(self_ptr: *mut mjCCache, size: usize) {
    extern "C" { fn mjCCache_SetCapacity_impl(self_ptr: *mut mjCCache, size: usize); }
    // SAFETY: delegates to C implementation
    unsafe { mjCCache_SetCapacity_impl(self_ptr, size) }
}

/// C: mjCCache::Reset (user/user_cache.h:156)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_cache_reset(self_ptr: *mut mjCCache) {
    extern "C" { fn mjCCache_Reset_impl(self_ptr: *mut mjCCache); }
    // SAFETY: delegates to C implementation
    unsafe { mjCCache_Reset_impl(self_ptr) }
}

/// C: mjCCache::Capacity (user/user_cache.h:159)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_cache_capacity(self_ptr: *mut mjCCache) -> std__size_t {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCCache)
    // Previous return: std__size_t
    extern "C" { fn mjCCache_Capacity_impl (self_ptr : * mut mjCCache) -> std__size_t ; } unsafe { mjCCache_Capacity_impl (self_ptr) }
}

/// C: mjCCache::Size (user/user_cache.h:160)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_cache_size(self_ptr: *mut mjCCache) -> std__size_t {
    extern "C" { fn mjCCache_Size_impl(self_ptr: *mut mjCCache) -> std__size_t; }
    // SAFETY: delegates to C implementation
    unsafe { mjCCache_Size_impl(self_ptr) }
}

/// C: mjCCache::Delete (user/user_cache.h:163)
/// Calls: mjCAsset::BytesCount
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_cache_delete(self_ptr: *mut mjCCache, asset: *mut mjCAsset) {
    extern "C" { fn mjCCache_Delete_impl(self_ptr: *mut mjCCache, asset: *mut mjCAsset); }
    // SAFETY: delegates to C implementation
    unsafe { mjCCache_Delete_impl(self_ptr, asset) }
}

/// C: mjCCache::Trim (user/user_cache.h:165)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_cache_trim(self_ptr: *mut mjCCache) {
    extern "C" { fn mjCCache_Trim_impl(self_ptr: *mut mjCCache); }
    // SAFETY: delegates to C implementation
    unsafe { mjCCache_Trim_impl(self_ptr) }
}

