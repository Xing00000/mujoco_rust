//! Port of: user/user_cache.cc
//! IR hash: d3ac8715281cd691
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mjCCache::HasAsset (user/user_cache.cc:53)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_cache_has_asset(self_ptr: *mut mjCCache, id: *const i32) -> *const i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCCache, id : * const i32)
    // Previous return: * const i32
    todo!("re-translate: params renamed")
}

/// C: mjCCache::Insert (user/user_cache.cc:67)
/// Calls: mjCAsset::BytesCount, mjCAsset::ReplaceData, mjCAsset::SetInsertNum
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_cache_insert(self_ptr: *mut mjCCache, modelname: *const i32, id: *const i32, resource: *const mjResource, data: *const (), size: usize) -> bool {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCCache, modelname : * const i32, id : * const i32, resource : * const mjResource, data : * const (), size : usize)
    // Previous return: bool
    todo!("re-translate: params renamed")
}

/// C: mjCCache::PopulateData (user/user_cache.cc:107)
/// Calls: mjCAsset::IncrementAccess, mjCAsset::PopulateData
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_cache_populate_data(self_ptr: *mut mjCCache, id: *const i32, resource: *const mjResource, r#fn: mjCDataFunc) -> bool {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCCache, id : * const i32, resource : * const mjResource, r#fn : mjCDataFunc)
    // Previous return: bool
    todo!("re-translate: params renamed")
}

/// C: mjCCache::RemoveModel (user/user_cache.cc:131)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_cache_remove_model(self_ptr: *mut mjCCache, filename: *const i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCCache, filename : * const i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCCache::DeleteAsset (user/user_cache.cc:182)
/// Calls: mjCCache::Delete
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_cache_delete_asset(self_ptr: *mut mjCCache, id: *const i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCCache, id : * const i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCAsset::Timestamp (user/user_cache.h:55)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_asset_timestamp(self_ptr: *mut mjCAsset) -> *const i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCAsset)
    // Previous return: * const i32
    todo!("re-translate: params renamed")
}

/// C: mjCAsset::Id (user/user_cache.h:56)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_asset_id(self_ptr: *mut mjCAsset) -> *const i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCAsset)
    // Previous return: * const i32
    todo!("re-translate: params renamed")
}

/// C: mjCAsset::InsertNum (user/user_cache.h:57)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_asset_insert_num(self_ptr: *mut mjCAsset) -> std__size_t {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCAsset)
    // Previous return: std__size_t
    todo!("re-translate: params renamed")
}

/// C: mjCAsset::AccessCount (user/user_cache.h:58)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_asset_access_count(self_ptr: *mut mjCAsset) -> std__size_t {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCAsset)
    // Previous return: std__size_t
    todo!("re-translate: params renamed")
}

/// C: mjCAsset::PopulateData (user/user_cache.h:61)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_asset_populate_data(self_ptr: *mut mjCAsset, r#fn: mjCDataFunc) -> bool {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCAsset, r#fn : mjCDataFunc)
    // Previous return: bool
    todo!("re-translate: params renamed")
}

/// C: mjCAsset::AddReference (user/user_cache.h:69)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_asset_add_reference(self_ptr: *mut mjCAsset, xml_file: i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCAsset, xml_file : i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCAsset::RemoveReference (user/user_cache.h:70)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_asset_remove_reference(self_ptr: *mut mjCAsset, xml_file: *const i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCAsset, xml_file : * const i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCAsset::ReplaceData (user/user_cache.h:74)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_asset_replace_data(self_ptr: *mut mjCAsset, other: *const mjCAsset) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCAsset, other : * const mjCAsset)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCAsset::HasReferences (user/user_cache.h:79)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_asset_has_references(self_ptr: *mut mjCAsset) -> bool {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCAsset)
    // Previous return: bool
    todo!("re-translate: params renamed")
}

/// C: mjCAsset::IncrementAccess (user/user_cache.h:81)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_asset_increment_access(self_ptr: *mut mjCAsset) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCAsset)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCAsset::Copy (user/user_cache.h:84)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_asset_copy(other: *const mjCAsset) -> mjCAsset {
    // NOTE: signature changed from previous IR version
    // Previous params: (other : * const mjCAsset)
    // Previous return: mjCAsset
    todo!("re-translate: params renamed")
}

/// C: mjCAsset::SetInsertNum (user/user_cache.h:87)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_asset_set_insert_num(self_ptr: *mut mjCAsset, num: usize) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCAsset, num : usize)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCAsset::SetTimestamp (user/user_cache.h:88)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_asset_set_timestamp(self_ptr: *mut mjCAsset, timestamp: i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCAsset, timestamp : i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCAsset::BytesCount (user/user_cache.h:91)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_asset_bytes_count(self_ptr: *mut mjCAsset) -> std__size_t {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCAsset)
    // Previous return: std__size_t
    todo!("re-translate: params renamed")
}

/// C: mjCAsset::Data (user/user_cache.h:92)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_asset_data(self_ptr: *mut mjCAsset) -> *const () {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCAsset)
    // Previous return: * const ()
    todo!("re-translate: params renamed")
}

/// C: mjCAsset::References (user/user_cache.h:95)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_asset_references(self_ptr: *mut mjCAsset) -> *const i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCAsset)
    // Previous return: * const i32
    todo!("re-translate: params renamed")
}

/// C: mjCCache::SetCapacity (user/user_cache.h:131)
/// Calls: mjCCache::Trim
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_cache_set_capacity(self_ptr: *mut mjCCache, size: usize) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCCache, size : usize)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCCache::Reset (user/user_cache.h:156)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_cache_reset(self_ptr: *mut mjCCache) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCCache)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCCache::Capacity (user/user_cache.h:159)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_cache_capacity(self_ptr: *mut mjCCache) -> std__size_t {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCCache)
    // Previous return: std__size_t
    todo!("re-translate: params renamed")
}

/// C: mjCCache::Size (user/user_cache.h:160)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_cache_size(self_ptr: *mut mjCCache) -> std__size_t {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCCache)
    // Previous return: std__size_t
    todo!("re-translate: params renamed")
}

/// C: mjCCache::Delete (user/user_cache.h:163)
/// Calls: mjCAsset::BytesCount
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_cache_delete(self_ptr: *mut mjCCache, asset: *mut mjCAsset) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCCache, asset : * mut mjCAsset)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCCache::Trim (user/user_cache.h:165)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_cache_trim(self_ptr: *mut mjCCache) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCCache)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

