//! Port of: user/user_cache.h
//! IR hash: 9343293228317031
//! CODEGEN: source paths, owners, and callable names are locked.

use crate::types::*;

pub use crate::types::mjCAsset;
impl crate::types::mjCAsset {
    /// C: mjCAsset::InsertNum (user/user_cache.h:57)
    #[allow(unused_variables, non_snake_case)]
    pub fn InsertNum(self_ptr: *mut mjCAsset) -> u64 {
        // MACHINE-PROVEN: Clang AST semantics
        unsafe { (*self_ptr).insert_num_ }
    }
    

    /// C: mjCAsset::AccessCount (user/user_cache.h:58)
    #[allow(unused_variables, non_snake_case)]
    pub fn AccessCount(self_ptr: *mut mjCAsset) -> u64 {
        // MACHINE-PROVEN: Clang AST semantics
        unsafe { (*self_ptr).access_count_ }
    }
    

    /// C: mjCAsset::BytesCount (user/user_cache.h:91)
    #[allow(unused_variables, non_snake_case)]
    pub fn BytesCount(self_ptr: *mut mjCAsset) -> u64 {
        // MACHINE-PROVEN: Clang AST semantics
        unsafe { (*self_ptr).size_ }
    }
    

    /// C: mjCAsset::References (user/user_cache.h:95)
    #[allow(unused_variables, non_snake_case)]
    pub fn References(self_ptr: *mut mjCAsset) -> *const () {
        // MACHINE-PROVEN: Clang AST semantics
        unsafe { (*self_ptr).references_ }
    }
    

}

