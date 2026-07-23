//! Port of: user/user_cache.h
//! IR hash: bd605ac8158c32d6
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mjCAsset::Timestamp (user/user_cache.h:55)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_asset_timestamp(self_ptr: *mut mjCAsset) -> *const std__string {
    todo!() // mjCAsset::Timestamp
}

/// C: mjCAsset::Id (user/user_cache.h:56)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_asset_id(self_ptr: *mut mjCAsset) -> *const std__string {
    todo!() // mjCAsset::Id
}

/// C: mjCAsset::InsertNum (user/user_cache.h:57)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_asset_insert_num(self_ptr: *mut mjCAsset) -> u64 {
    todo!() // mjCAsset::InsertNum
}

/// C: mjCAsset::AccessCount (user/user_cache.h:58)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_asset_access_count(self_ptr: *mut mjCAsset) -> u64 {
    todo!() // mjCAsset::AccessCount
}

/// C: mjCAsset::PopulateData (user/user_cache.h:61)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_asset_populate_data(self_ptr: *mut mjCAsset, r#fn: mjCDataFunc) -> bool {
    todo!() // mjCAsset::PopulateData
}

/// C: mjCAsset::AddReference (user/user_cache.h:69)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_asset_add_reference(self_ptr: *mut mjCAsset, xml_file: std__string) {
    todo!() // mjCAsset::AddReference
}

/// C: mjCAsset::RemoveReference (user/user_cache.h:70)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_asset_remove_reference(self_ptr: *mut mjCAsset, xml_file: *const std__string) {
    todo!() // mjCAsset::RemoveReference
}

/// C: mjCAsset::ReplaceData (user/user_cache.h:74)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_asset_replace_data(self_ptr: *mut mjCAsset, other: *const mjCAsset) {
    todo!() // mjCAsset::ReplaceData
}

/// C: mjCAsset::HasReferences (user/user_cache.h:79)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_asset_has_references(self_ptr: *mut mjCAsset) -> bool {
    todo!() // mjCAsset::HasReferences
}

/// C: mjCAsset::IncrementAccess (user/user_cache.h:81)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_asset_increment_access(self_ptr: *mut mjCAsset) {
    todo!() // mjCAsset::IncrementAccess
}

/// C: mjCAsset::Copy (user/user_cache.h:84)
/// Calls: mjCAsset::Id, mjCAsset::Timestamp
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_asset_copy(other: *const mjCAsset) -> mjCAsset {
    todo!() // mjCAsset::Copy
}

/// C: mjCAsset::SetInsertNum (user/user_cache.h:87)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_asset_set_insert_num(self_ptr: *mut mjCAsset, num: u64) {
    todo!() // mjCAsset::SetInsertNum
}

/// C: mjCAsset::SetTimestamp (user/user_cache.h:88)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_asset_set_timestamp(self_ptr: *mut mjCAsset, timestamp: std__string) {
    todo!() // mjCAsset::SetTimestamp
}

/// C: mjCAsset::BytesCount (user/user_cache.h:91)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_asset_bytes_count(self_ptr: *mut mjCAsset) -> u64 {
    todo!() // mjCAsset::BytesCount
}

/// C: mjCAsset::Data (user/user_cache.h:92)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_asset_data(self_ptr: *mut mjCAsset) -> *const () {
    todo!() // mjCAsset::Data
}

/// C: mjCAsset::References (user/user_cache.h:95)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_asset_references(self_ptr: *mut mjCAsset) -> *const () {
    todo!() // mjCAsset::References
}

/// C: mjCCache::SetCapacity (user/user_cache.h:131)
/// Calls: mjCCache::Trim
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_cache_set_capacity(self_ptr: *mut mjCCache, size: u64) {
    todo!() // mjCCache::SetCapacity
}

/// C: mjCCache::HasAsset (user/user_cache.h:135)
/// Calls: mjCAsset::Timestamp
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_cache_has_asset(self_ptr: *mut mjCCache, id: *const std__string) -> *const std__string {
    todo!() // mjCCache::HasAsset
}

/// C: mjCCache::Insert (user/user_cache.h:139)
/// Calls: mjCAsset::AddReference, mjCAsset::BytesCount, mjCAsset::ReplaceData, mjCAsset::SetInsertNum, mjCAsset::SetTimestamp, mjCAsset::Timestamp
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_cache_insert(self_ptr: *mut mjCCache, modelname: *const std__string, id: *const std__string, resource: *const mjResource, data: *const (), size: u64) -> bool {
    todo!() // mjCCache::Insert
}

/// C: mjCCache::PopulateData (user/user_cache.h:143)
/// Calls: mjCAsset::IncrementAccess, mjCAsset::PopulateData, mjCAsset::Timestamp, mju_isModifiedResource
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_cache_populate_data(self_ptr: *mut mjCCache, id: *const std__string, resource: *const mjResource, r#fn: mjCDataFunc) -> bool {
    todo!() // mjCCache::PopulateData
}

/// C: mjCCache::DeleteAsset (user/user_cache.h:146)
/// Calls: mjCCache::Delete
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_cache_delete_asset(self_ptr: *mut mjCCache, id: *const std__string) {
    todo!() // mjCCache::DeleteAsset
}

/// C: mjCCache::RemoveModel (user/user_cache.h:150)
/// Calls: mjCAsset::HasReferences, mjCAsset::RemoveReference, mjCCache::Delete
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_cache_remove_model(self_ptr: *mut mjCCache, filename: *const std__string) {
    todo!() // mjCCache::RemoveModel
}

/// C: mjCCache::Reset (user/user_cache.h:153)
/// Calls: mjCCache::Delete
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_cache_reset(self_ptr: *mut mjCCache, filename: *const std__string) {
    todo!() // mjCCache::Reset
}

/// C: mjCCache::Capacity (user/user_cache.h:159)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_cache_capacity(self_ptr: *mut mjCCache) -> u64 {
    todo!() // mjCCache::Capacity
}

/// C: mjCCache::Size (user/user_cache.h:160)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_cache_size(self_ptr: *mut mjCCache) -> u64 {
    todo!() // mjCCache::Size
}

/// C: mjCCache::Delete (user/user_cache.h:163)
/// Calls: mjCAsset::BytesCount, mjCAsset::Id, mjCAsset::References
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_cache_delete(self_ptr: *mut mjCCache, asset: *mut mjCAsset) {
    todo!() // mjCCache::Delete
}

/// C: mjCCache::Trim (user/user_cache.h:165)
/// Calls: mjCCache::Delete
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_cache_trim(self_ptr: *mut mjCCache) {
    todo!() // mjCCache::Trim
}

