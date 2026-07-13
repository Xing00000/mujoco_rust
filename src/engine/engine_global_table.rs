//! Port of: engine/engine_global_table.h
//! IR hash: d3ac8715281cd691
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: CaseInsensitiveEqual (engine/engine_global_table.h:32)
#[allow(unused_variables, non_snake_case)]
pub fn case_insensitive_equal(s1: string_view, s2: string_view) -> bool {
    // NOTE: signature changed from previous IR version
    // Previous params: (s1 : string_view, s2 : string_view)
    // Previous return: bool
    todo!("re-translate: params renamed")
}

/// C: ReentrantWriteLock::LockCountOnCurrentThread (engine/engine_global_table.h:88)
#[allow(unused_variables, non_snake_case)]
pub fn reentrant_write_lock_lock_count_on_current_thread() -> *mut i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: ()
    // Previous return: * mut i32
    todo!("re-translate: params renamed")
}

/// C: GlobalTable::GetSingleton (engine/engine_global_table.h:97)
#[allow(unused_variables, non_snake_case)]
pub fn global_table_get_singleton() -> *const () {
    // NOTE: signature changed from previous IR version
    // Previous params: ()
    // Previous return: * const ()
    todo!("re-translate: params renamed")
}

/// C: GlobalTable::count (engine/engine_global_table.h:110)
#[allow(unused_variables, non_snake_case)]
pub fn global_table_count(self_ptr: *mut GlobalTable) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut GlobalTable)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: GlobalTable::LockExclusively (engine/engine_global_table.h:114)
/// Calls: GlobalTable::mutex
#[allow(unused_variables, non_snake_case)]
pub fn global_table_lock_exclusively(self_ptr: *mut GlobalTable) -> ReentrantWriteLock {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut GlobalTable)
    // Previous return: ReentrantWriteLock
    todo!("re-translate: params renamed")
}

/// C: GlobalTable::AppendIfUnique (engine/engine_global_table.h:118)
/// Calls: GlobalTable::LockExclusively, mju_error
#[allow(unused_variables, non_snake_case)]
pub fn global_table_append_if_unique(self_ptr: *mut GlobalTable, obj: *const T) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut GlobalTable, obj : * const T)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: GlobalTable::GetAtSlotUnsafe (engine/engine_global_table.h:184)
#[allow(unused_variables, non_snake_case)]
pub fn global_table_get_at_slot_unsafe(self_ptr: *mut GlobalTable, slot: i32, nslot: i32) -> *const T {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut GlobalTable, slot : i32, nslot : i32)
    // Previous return: * const T
    todo!("re-translate: params renamed")
}

/// C: GlobalTable::GetByKeyUnsafe (engine/engine_global_table.h:213)
/// Calls: CaseInsensitiveEqual
#[allow(unused_variables, non_snake_case)]
pub fn global_table_get_by_key_unsafe(self_ptr: *mut GlobalTable, key: string_view, slot: *mut i32, nslot: i32) -> *const T {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut GlobalTable, key : string_view, slot : * mut i32, nslot : i32)
    // Previous return: * const T
    todo!("re-translate: params renamed")
}

/// C: GlobalTable::GetAtSlot (engine/engine_global_table.h:248)
/// Calls: GlobalTable::GetAtSlotUnsafe, GlobalTable::count
#[allow(unused_variables, non_snake_case)]
pub fn global_table_get_at_slot(self_ptr: *mut GlobalTable, slot: i32) -> *const T {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut GlobalTable, slot : i32)
    // Previous return: * const T
    todo!("re-translate: params renamed")
}

/// C: GlobalTable::GetByKey (engine/engine_global_table.h:254)
/// Calls: GlobalTable::GetByKeyUnsafe, GlobalTable::count
#[allow(unused_variables, non_snake_case)]
pub fn global_table_get_by_key(self_ptr: *mut GlobalTable, key: string_view, slot: *mut i32) -> *const T {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut GlobalTable, key : string_view, slot : * mut i32)
    // Previous return: * const T
    todo!("re-translate: params renamed")
}

/// C: GlobalTable::mutex (engine/engine_global_table.h:265)
#[allow(unused_variables, non_snake_case)]
pub fn global_table_mutex(self_ptr: *mut GlobalTable) -> *mut Mutex {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut GlobalTable)
    // Previous return: * mut Mutex
    todo!("re-translate: params renamed")
}

