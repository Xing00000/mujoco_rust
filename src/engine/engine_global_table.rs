//! Port of: engine/engine_global_table.h
//! IR hash: 05737965add36adb
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: CaseInsensitiveEqual (engine/engine_global_table.h:32)
#[allow(unused_variables, non_snake_case)]
pub fn case_insensitive_equal(s1: string_view, s2: string_view) -> bool  {
    extern "C" { fn CaseInsensitiveEqual(s1: string_view, s2: string_view) -> bool; }
    // SAFETY: delegates to C implementation
    unsafe { CaseInsensitiveEqual(s1, s2) }
}

/// C: ReentrantWriteLock::LockCountOnCurrentThread (engine/engine_global_table.h:88)
#[allow(unused_variables, non_snake_case)]
pub fn reentrant_write_lock_lock_count_on_current_thread() -> *mut i32 {
    extern "C" { fn ReentrantWriteLock_LockCountOnCurrentThread() -> *mut i32; }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { ReentrantWriteLock_LockCountOnCurrentThread() }
}

/// C: GlobalTable::GetSingleton (engine/engine_global_table.h:97)
#[allow(unused_variables, non_snake_case)]
pub fn global_table_get_singleton() -> *const () {
    // WARNING: signature changed — verify body
    // Previous params: ()
    // Previous return: * const ()
    extern "C" { fn GlobalTable_GetSingleton() -> * const () ; } unsafe { GlobalTable_GetSingleton() }
}

/// C: GlobalTable::count (engine/engine_global_table.h:110)
#[allow(unused_variables, non_snake_case)]
pub fn global_table_count(self_ptr: *mut GlobalTable) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut GlobalTable)
    // Previous return: i32
    extern "C" { fn GlobalTable_count(self_ptr : * mut GlobalTable) -> i32 ; } unsafe { GlobalTable_count(self_ptr) }
}

/// C: GlobalTable::LockExclusively (engine/engine_global_table.h:114)
/// Calls: GlobalTable::mutex
#[allow(unused_variables, non_snake_case)]
pub fn global_table_lock_exclusively(self_ptr: *mut GlobalTable) -> ReentrantWriteLock {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut GlobalTable)
    // Previous return: ReentrantWriteLock
    extern "C" { fn GlobalTable_LockExclusively(self_ptr : * mut GlobalTable) -> ReentrantWriteLock ; } unsafe { GlobalTable_LockExclusively(self_ptr) }
}

/// C: GlobalTable::AppendIfUnique (engine/engine_global_table.h:118)
/// Calls: GlobalTable::LockExclusively, mju_error
#[allow(unused_variables, non_snake_case)]
pub fn global_table_append_if_unique(self_ptr: *mut GlobalTable, obj: *const T) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut GlobalTable, obj : * const T)
    // Previous return: i32
    extern "C" { fn GlobalTable_AppendIfUnique(self_ptr : * mut GlobalTable , obj : * const T) -> i32 ; } unsafe { GlobalTable_AppendIfUnique(self_ptr , obj) }
}

/// C: GlobalTable::GetAtSlotUnsafe (engine/engine_global_table.h:184)
#[allow(unused_variables, non_snake_case)]
pub fn global_table_get_at_slot_unsafe(self_ptr: *mut GlobalTable, slot: i32, nslot: i32) -> *const T {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut GlobalTable, slot : i32, nslot : i32)
    // Previous return: * const T
    extern "C" { fn GlobalTable_GetAtSlotUnsafe(self_ptr : * mut GlobalTable , slot : i32 , nslot : i32) -> * const T ; } unsafe { GlobalTable_GetAtSlotUnsafe(self_ptr , slot , nslot) }
}

/// C: GlobalTable::GetByKeyUnsafe (engine/engine_global_table.h:213)
/// Calls: CaseInsensitiveEqual
#[allow(unused_variables, non_snake_case)]
pub fn global_table_get_by_key_unsafe(self_ptr: *mut GlobalTable, key: string_view, slot: *mut i32, nslot: i32) -> *const T {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut GlobalTable, key : string_view, slot : * mut i32, nslot : i32)
    // Previous return: * const T
    extern "C" { fn GlobalTable_GetByKeyUnsafe(self_ptr : * mut GlobalTable , key : string_view , slot : * mut i32 , nslot : i32) -> * const T ; } unsafe { GlobalTable_GetByKeyUnsafe(self_ptr , key , slot , nslot) }
}

/// C: GlobalTable::GetAtSlot (engine/engine_global_table.h:248)
/// Calls: GlobalTable::GetAtSlotUnsafe, GlobalTable::count
#[allow(unused_variables, non_snake_case)]
pub fn global_table_get_at_slot(self_ptr: *mut GlobalTable, slot: i32) -> *const T {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut GlobalTable, slot : i32)
    // Previous return: * const T
    extern "C" { fn GlobalTable_GetAtSlot(self_ptr : * mut GlobalTable , slot : i32) -> * const T ; } unsafe { GlobalTable_GetAtSlot(self_ptr , slot) }
}

/// C: GlobalTable::GetByKey (engine/engine_global_table.h:254)
/// Calls: GlobalTable::GetByKeyUnsafe, GlobalTable::count
#[allow(unused_variables, non_snake_case)]
pub fn global_table_get_by_key(self_ptr: *mut GlobalTable, key: string_view, slot: *mut i32) -> *const T {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut GlobalTable, key : string_view, slot : * mut i32)
    // Previous return: * const T
    extern "C" { fn GlobalTable_GetByKey(self_ptr : * mut GlobalTable , key : string_view , slot : * mut i32) -> * const T ; } unsafe { GlobalTable_GetByKey(self_ptr , key , slot) }
}

/// C: GlobalTable::mutex (engine/engine_global_table.h:265)
#[allow(unused_variables, non_snake_case)]
pub fn global_table_mutex(self_ptr: *mut GlobalTable) -> *mut Mutex {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut GlobalTable)
    // Previous return: * mut Mutex
    extern "C" { fn GlobalTable_mutex(self_ptr : * mut GlobalTable) -> * mut Mutex ; } unsafe { GlobalTable_mutex(self_ptr) }
}

