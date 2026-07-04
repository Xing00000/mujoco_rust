//! Port of: user/user_cache.cc
//! IR hash: 1b139f44af8230f9
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mjCCache::HasAsset (user/user_cache.cc:53)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_cache_has_asset(id: *const i32) -> *const i32 {
    todo!() // mjCCache::HasAsset
}

/// C: mjCCache::Insert (user/user_cache.cc:67)
/// Calls: mjCAsset::BytesCount, mjCAsset::ReplaceData, mjCAsset::SetInsertNum
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_cache_insert(modelname: *const i32, id: *const i32, resource: *const mjResource, data: *const (), size: std__size_t) -> bool {
    todo!() // mjCCache::Insert
}

/// C: mjCCache::PopulateData (user/user_cache.cc:107)
/// Calls: mjCAsset::IncrementAccess, mjCAsset::PopulateData
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_cache_populate_data(id: *const i32, resource: *const mjResource, r#fn: mjCDataFunc) -> bool {
    todo!() // mjCCache::PopulateData
}

/// C: mjCCache::RemoveModel (user/user_cache.cc:131)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_cache_remove_model(filename: *const i32) {
    todo!() // mjCCache::RemoveModel
}

/// C: mjCCache::DeleteAsset (user/user_cache.cc:182)
/// Calls: mjCCache::Delete
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_cache_delete_asset(id: *const i32) {
    todo!() // mjCCache::DeleteAsset
}

/// C: mjCAsset::Timestamp (user/user_cache.h:55)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_asset_timestamp() -> *const i32 {
    todo!() // mjCAsset::Timestamp
}

/// C: mjCAsset::Id (user/user_cache.h:56)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_asset_id() -> *const i32 {
    todo!() // mjCAsset::Id
}

/// C: mjCAsset::InsertNum (user/user_cache.h:57)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_asset_insert_num() -> std__size_t {
    todo!() // mjCAsset::InsertNum
}

/// C: mjCAsset::AccessCount (user/user_cache.h:58)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_asset_access_count() -> std__size_t {
    todo!() // mjCAsset::AccessCount
}

/// C: mjCAsset::PopulateData (user/user_cache.h:61)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_asset_populate_data(r#fn: mjCDataFunc) -> bool {
    todo!() // mjCAsset::PopulateData
}

/// C: mjCAsset::AddReference (user/user_cache.h:69)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_asset_add_reference(xml_file: i32) {
    todo!() // mjCAsset::AddReference
}

/// C: mjCAsset::RemoveReference (user/user_cache.h:70)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_asset_remove_reference(xml_file: *const i32) {
    todo!() // mjCAsset::RemoveReference
}

/// C: mjCAsset::ReplaceData (user/user_cache.h:74)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_asset_replace_data(other: *const mjCAsset) {
    todo!() // mjCAsset::ReplaceData
}

/// C: mjCAsset::HasReferences (user/user_cache.h:79)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_asset_has_references() -> bool {
    todo!() // mjCAsset::HasReferences
}

/// C: mjCAsset::IncrementAccess (user/user_cache.h:81)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_asset_increment_access() {
    todo!() // mjCAsset::IncrementAccess
}

/// C: mjCAsset::Copy (user/user_cache.h:84)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_asset_copy(other: *const mjCAsset) -> mjCAsset {
    todo!() // mjCAsset::Copy
}

/// C: mjCAsset::SetInsertNum (user/user_cache.h:87)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_asset_set_insert_num(num: std__size_t) {
    todo!() // mjCAsset::SetInsertNum
}

/// C: mjCAsset::SetTimestamp (user/user_cache.h:88)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_asset_set_timestamp(timestamp: i32) {
    todo!() // mjCAsset::SetTimestamp
}

/// C: mjCAsset::BytesCount (user/user_cache.h:91)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_asset_bytes_count() -> std__size_t {
    todo!() // mjCAsset::BytesCount
}

/// C: mjCAsset::Data (user/user_cache.h:92)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_asset_data() -> *const () {
    todo!() // mjCAsset::Data
}

/// C: mjCAsset::References (user/user_cache.h:95)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_asset_references() -> *const i32 {
    todo!() // mjCAsset::References
}

/// C: mjCCache::SetCapacity (user/user_cache.h:131)
/// Calls: mjCCache::Trim
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_cache_set_capacity(size: std__size_t) {
    todo!() // mjCCache::SetCapacity
}

/// C: mjCCache::Reset (user/user_cache.h:156)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_cache_reset() {
    todo!() // mjCCache::Reset
}

/// C: mjCCache::Capacity (user/user_cache.h:159)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_cache_capacity() -> std__size_t {
    todo!() // mjCCache::Capacity
}

/// C: mjCCache::Size (user/user_cache.h:160)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_cache_size() -> std__size_t {
    todo!() // mjCCache::Size
}

/// C: mjCCache::Delete (user/user_cache.h:163)
/// Calls: mjCAsset::BytesCount
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_cache_delete(asset: *mut mjCAsset) {
    todo!() // mjCCache::Delete
}

/// C: mjCCache::Trim (user/user_cache.h:165)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_cache_trim() {
    todo!() // mjCCache::Trim
}

