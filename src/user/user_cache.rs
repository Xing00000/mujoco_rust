//! Port of: user/user_cache.cc
//! IR hash: fd5288ad7242f001
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mjCAsset::Copy (user/user_cache.cc:30)
/// Calls: cxx:__ZNK8mjCAsset2IdEv, cxx:__ZNK8mjCAsset9TimestampEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_asset_copy(other: *const mjCAsset) -> mjCAsset {
    todo!() // cxx:__ZN8mjCAsset4CopyERKS_
}

/// C: mjCCache::SetCapacity (user/user_cache.cc:43)
/// Calls: cxx:__ZN8mjCCache4TrimEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_cache_set_capacity(self_ptr: *mut mjCCache, size: u64) {
    todo!() // cxx:__ZN8mjCCache11SetCapacityEm
}

/// C: mjCCache::HasAsset (user/user_cache.cc:51)
/// Calls: cxx:__ZNK8mjCAsset9TimestampEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_cache_has_asset(self_ptr: *mut mjCCache, id: *const std__string) -> *const std__string {
    todo!() // cxx:__ZN8mjCCache8HasAssetERKNSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEE
}

/// C: mjCCache::Insert (user/user_cache.cc:66)
/// Calls: cxx:__ZN8mjCAsset11ReplaceDataERKS_, cxx:__ZN8mjCAsset12AddReferenceENSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEE, cxx:__ZN8mjCAsset12SetInsertNumEm, cxx:__ZN8mjCAsset12SetTimestampENSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEE, cxx:__ZNK8mjCAsset10BytesCountEv, cxx:__ZNK8mjCAsset9TimestampEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_cache_insert(self_ptr: *mut mjCCache, modelname: *const std__string, id: *const std__string, resource: *const mjResource, data: *const (), size: u64) -> bool {
    todo!() // cxx:__ZN8mjCCache6InsertERKNSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEES8_PK11mjResource_NS0_10shared_ptrIKvEEm
}

/// C: mjCCache::PopulateData (user/user_cache.cc:97)
/// Calls: cxx:__ZN8mjCAsset15IncrementAccessEv, cxx:__ZNK8mjCAsset12PopulateDataENSt3__18functionIFbPKvEEE, cxx:__ZNK8mjCAsset9TimestampEv, cxx:_mju_isModifiedResource
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_cache_populate_data(self_ptr: *mut mjCCache, id: *const std__string, resource: *const mjResource, r#fn: mjCDataFunc) -> bool {
    todo!() // cxx:__ZN8mjCCache12PopulateDataERKNSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEEPK11mjResource_NS0_8functionIFbPKvEEE
}

/// C: mjCCache::RemoveModel (user/user_cache.cc:116)
/// Calls: cxx:__ZN8mjCAsset15RemoveReferenceERKNSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEE, cxx:__ZNK8mjCAsset13HasReferencesEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_cache_remove_model(self_ptr: *mut mjCCache, filename: *const std__string) {
    todo!() // cxx:__ZN8mjCCache11RemoveModelERKNSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEE
}

/// C: mjCCache::Reset (user/user_cache.cc:127)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_cache_reset__bd1ccbfeda80cbc0(self_ptr: *mut mjCCache, filename: *const std__string) {
    todo!() // cxx:__ZN8mjCCache5ResetERKNSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEE
}

/// C: mjCCache::Reset (user/user_cache.cc:135)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_cache_reset__357e7517f0240a05(self_ptr: *mut mjCCache) {
    todo!() // cxx:__ZN8mjCCache5ResetEv
}

/// C: mjCCache::Capacity (user/user_cache.cc:145)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_cache_capacity(self_ptr: *mut mjCCache) -> u64 {
    todo!() // cxx:__ZNK8mjCCache8CapacityEv
}

/// C: mjCCache::Size (user/user_cache.cc:151)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_cache_size(self_ptr: *mut mjCCache) -> u64 {
    todo!() // cxx:__ZNK8mjCCache4SizeEv
}

/// C: mjCCache::DeleteAsset (user/user_cache.cc:158)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_cache_delete_asset(self_ptr: *mut mjCCache, id: *const std__string) {
    todo!() // cxx:__ZN8mjCCache11DeleteAssetERKNSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEE
}

/// C: mjCCache::Delete (user/user_cache.cc:166)
/// Calls: cxx:__ZNK8mjCAsset10BytesCountEv, cxx:__ZNK8mjCAsset10ReferencesEv, cxx:__ZNK8mjCAsset2IdEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_cache_delete__e540c55e847655b2(self_ptr: *mut mjCCache, asset: *mut mjCAsset) {
    todo!() // cxx:__ZN8mjCCache6DeleteEP8mjCAsset
}

/// C: mjCCache::Delete (user/user_cache.cc:175)
/// Calls: cxx:__ZNK8mjCAsset10BytesCountEv, cxx:__ZNK8mjCAsset10ReferencesEv, cxx:__ZNK8mjCAsset2IdEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_cache_delete__0e05b340222a7fc6(self_ptr: *mut mjCCache, asset: *mut mjCAsset, skip: *const std__string) {
    todo!() // cxx:__ZN8mjCCache6DeleteEP8mjCAssetRKNSt3__112basic_stringIcNS2_11char_traitsIcEENS2_9allocatorIcEEEE
}

/// C: mjCCache::Trim (user/user_cache.cc:187)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_cache_trim(self_ptr: *mut mjCCache) {
    todo!() // cxx:__ZN8mjCCache4TrimEv
}

/// C: mjCAsset::Timestamp (user/user_cache.h:58)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_asset_timestamp(self_ptr: *mut mjCAsset) -> *const std__string {
    todo!() // cxx:__ZNK8mjCAsset9TimestampEv
}

/// C: mjCAsset::Id (user/user_cache.h:59)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_asset_id(self_ptr: *mut mjCAsset) -> *const std__string {
    todo!() // cxx:__ZNK8mjCAsset2IdEv
}

/// C: mjCAsset::InsertNum (user/user_cache.h:60)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_asset_insert_num(self_ptr: *mut mjCAsset) -> u64 {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).insert_num_ }
}

/// C: mjCAsset::AccessCount (user/user_cache.h:61)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_asset_access_count(self_ptr: *mut mjCAsset) -> u64 {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).access_count_ }
}

/// C: mjCAsset::PopulateData (user/user_cache.h:64)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_asset_populate_data(self_ptr: *mut mjCAsset, r#fn: mjCDataFunc) -> bool {
    todo!() // cxx:__ZNK8mjCAsset12PopulateDataENSt3__18functionIFbPKvEEE
}

/// C: mjCAsset::AddReference (user/user_cache.h:70)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_asset_add_reference(self_ptr: *mut mjCAsset, xml_file: std__string) {
    todo!() // cxx:__ZN8mjCAsset12AddReferenceENSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEE
}

/// C: mjCAsset::RemoveReference (user/user_cache.h:71)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_asset_remove_reference(self_ptr: *mut mjCAsset, xml_file: *const std__string) {
    todo!() // cxx:__ZN8mjCAsset15RemoveReferenceERKNSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEE
}

/// C: mjCAsset::ReplaceData (user/user_cache.h:73)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_asset_replace_data(self_ptr: *mut mjCAsset, other: *const mjCAsset) {
    todo!() // cxx:__ZN8mjCAsset11ReplaceDataERKS_
}

/// C: mjCAsset::HasReferences (user/user_cache.h:78)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_asset_has_references(self_ptr: *mut mjCAsset) -> bool {
    todo!() // cxx:__ZNK8mjCAsset13HasReferencesEv
}

/// C: mjCAsset::IncrementAccess (user/user_cache.h:80)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_asset_increment_access(self_ptr: *mut mjCAsset) {
    todo!() // cxx:__ZN8mjCAsset15IncrementAccessEv
}

/// C: mjCAsset::SetInsertNum (user/user_cache.h:86)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_asset_set_insert_num(self_ptr: *mut mjCAsset, num: u64) {
    todo!() // cxx:__ZN8mjCAsset12SetInsertNumEm
}

/// C: mjCAsset::SetTimestamp (user/user_cache.h:87)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_asset_set_timestamp(self_ptr: *mut mjCAsset, timestamp: std__string) {
    todo!() // cxx:__ZN8mjCAsset12SetTimestampENSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEE
}

/// C: mjCAsset::BytesCount (user/user_cache.h:90)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_asset_bytes_count(self_ptr: *mut mjCAsset) -> u64 {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).size_ }
}

/// C: mjCAsset::Data (user/user_cache.h:91)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_asset_data(self_ptr: *mut mjCAsset) -> *const () {
    todo!() // cxx:__ZNK8mjCAsset4DataEv
}

/// C: mjCAsset::References (user/user_cache.h:92)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_asset_references(self_ptr: *mut mjCAsset) -> *const () {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).references_ }
}

