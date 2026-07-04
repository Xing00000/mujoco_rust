//! Port of: engine/engine_global_table.h
//! IR hash: 1b139f44af8230f9
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: CaseInsensitiveEqual (engine/engine_global_table.h:32)
#[allow(unused_variables, non_snake_case)]
pub fn case_insensitive_equal(s1: std__string_view, s2: std__string_view) -> bool {
    todo!() // CaseInsensitiveEqual
}

/// C: ReentrantWriteLock::LockCountOnCurrentThread (engine/engine_global_table.h:88)
#[allow(unused_variables, non_snake_case)]
pub fn reentrant_write_lock_lock_count_on_current_thread() -> *mut i32 {
    todo!() // ReentrantWriteLock::LockCountOnCurrentThread
}

/// C: GlobalTable::GetSingleton (engine/engine_global_table.h:97)
#[allow(unused_variables, non_snake_case)]
pub fn global_table_get_singleton() -> *const () {
    todo!() // GlobalTable::GetSingleton
}

/// C: GlobalTable::count (engine/engine_global_table.h:110)
#[allow(unused_variables, non_snake_case)]
pub fn global_table_count() -> i32 {
    todo!() // GlobalTable::count
}

/// C: GlobalTable::LockExclusively (engine/engine_global_table.h:114)
/// Calls: GlobalTable::mutex
#[allow(unused_variables, non_snake_case)]
pub fn global_table_lock_exclusively() -> ReentrantWriteLock {
    todo!() // GlobalTable::LockExclusively
}

/// C: GlobalTable::AppendIfUnique (engine/engine_global_table.h:118)
/// Calls: GlobalTable::LockExclusively, mju_error
#[allow(unused_variables, non_snake_case)]
pub fn global_table_append_if_unique(obj: *const T) -> i32 {
    todo!() // GlobalTable::AppendIfUnique
}

/// C: GlobalTable::GetAtSlotUnsafe (engine/engine_global_table.h:184)
#[allow(unused_variables, non_snake_case)]
pub fn global_table_get_at_slot_unsafe(slot: i32, nslot: i32) -> *const T {
    todo!() // GlobalTable::GetAtSlotUnsafe
}

/// C: GlobalTable::GetByKeyUnsafe (engine/engine_global_table.h:213)
/// Calls: CaseInsensitiveEqual
#[allow(unused_variables, non_snake_case)]
pub fn global_table_get_by_key_unsafe(key: std__string_view, slot: *mut i32, nslot: i32) -> *const T {
    todo!() // GlobalTable::GetByKeyUnsafe
}

/// C: GlobalTable::GetAtSlot (engine/engine_global_table.h:248)
/// Calls: GlobalTable::GetAtSlotUnsafe, GlobalTable::count
#[allow(unused_variables, non_snake_case)]
pub fn global_table_get_at_slot(slot: i32) -> *const T {
    todo!() // GlobalTable::GetAtSlot
}

/// C: GlobalTable::GetByKey (engine/engine_global_table.h:254)
/// Calls: GlobalTable::GetByKeyUnsafe, GlobalTable::count
#[allow(unused_variables, non_snake_case)]
pub fn global_table_get_by_key(key: std__string_view, slot: *mut i32) -> *const T {
    todo!() // GlobalTable::GetByKey
}

/// C: GlobalTable::mutex (engine/engine_global_table.h:265)
#[allow(unused_variables, non_snake_case)]
pub fn global_table_mutex() -> *mut Mutex {
    todo!() // GlobalTable::mutex
}

