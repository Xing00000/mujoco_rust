//! Port of: engine/engine_global_table.h
//! IR hash: fd5288ad7242f001
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: CaseInsensitiveEqual (engine/engine_global_table.h:32)
#[allow(unused_variables, non_snake_case)]
pub fn case_insensitive_equal(s1: std__string_view, s2: std__string_view) -> bool {
    todo!() // cxx-internal:engine_plugin.cc.o:__ZN6mujocoL20CaseInsensitiveEqualENSt3__117basic_string_viewIcNS0_11char_traitsIcEEEES4_
}

/// C: ReentrantWriteLock::LockCountOnCurrentThread (engine/engine_global_table.h:88)
#[allow(unused_variables, non_snake_case)]
pub fn reentrant_write_lock_lock_count_on_current_thread() -> *mut i32 {
    todo!() // cxx:__ZN6mujoco18ReentrantWriteLock24LockCountOnCurrentThreadEv
}

/// C: GlobalTable::GetSingleton (engine/engine_global_table.h:97)
#[allow(unused_variables, non_snake_case)]
pub fn global_table_get_singleton__072a351d4d17f016() -> *const () {
    todo!() // cxx:__ZN6mujoco11GlobalTableI10mjpDecodervE12GetSingletonEv
}

/// C: GlobalTable::GetSingleton (engine/engine_global_table.h:97)
#[allow(unused_variables, non_snake_case)]
pub fn global_table_get_singleton__199b3b13c4958179() -> *const () {
    todo!() // cxx:__ZN6mujoco11GlobalTableI10mjpEncodervE12GetSingletonEv
}

/// C: GlobalTable::GetSingleton (engine/engine_global_table.h:97)
#[allow(unused_variables, non_snake_case)]
pub fn global_table_get_singleton__ffcee3f005e51432() -> *const () {
    todo!() // cxx:__ZN6mujoco11GlobalTableI10mjpPlugin_vE12GetSingletonEv
}

/// C: GlobalTable::GetSingleton (engine/engine_global_table.h:97)
#[allow(unused_variables, non_snake_case)]
pub fn global_table_get_singleton__5ce6d214dc477e04() -> *const () {
    todo!() // cxx:__ZN6mujoco11GlobalTableI19mjpResourceProvidervE12GetSingletonEv
}

/// C: GlobalTable::count (engine/engine_global_table.h:110)
#[allow(unused_variables, non_snake_case)]
pub fn global_table_count__989c04cb80e5011b(self_ptr: *mut mujoco__GlobalTable) -> i32 {
    todo!() // cxx:__ZN6mujoco11GlobalTableI10mjpDecodervE5countEv
}

/// C: GlobalTable::count (engine/engine_global_table.h:110)
#[allow(unused_variables, non_snake_case)]
pub fn global_table_count__b52ac1706de055e0(self_ptr: *mut mujoco__GlobalTable) -> i32 {
    todo!() // cxx:__ZN6mujoco11GlobalTableI10mjpEncodervE5countEv
}

/// C: GlobalTable::count (engine/engine_global_table.h:110)
#[allow(unused_variables, non_snake_case)]
pub fn global_table_count__b707aeab3825c1f5(self_ptr: *mut mujoco__GlobalTable) -> i32 {
    todo!() // cxx:__ZN6mujoco11GlobalTableI10mjpPlugin_vE5countEv
}

/// C: GlobalTable::count (engine/engine_global_table.h:110)
#[allow(unused_variables, non_snake_case)]
pub fn global_table_count__4ef83823fdb4793b(self_ptr: *mut mujoco__GlobalTable) -> i32 {
    todo!() // cxx:__ZN6mujoco11GlobalTableI19mjpResourceProvidervE5countEv
}

/// C: GlobalTable::LockExclusively (engine/engine_global_table.h:114)
#[allow(unused_variables, non_snake_case)]
pub fn global_table_lock_exclusively__7e57de41af570be0(self_ptr: *mut mujoco__GlobalTable) -> mujoco__ReentrantWriteLock {
    todo!() // cxx:__ZN6mujoco11GlobalTableI10mjpDecodervE15LockExclusivelyEv
}

/// C: GlobalTable::LockExclusively (engine/engine_global_table.h:114)
#[allow(unused_variables, non_snake_case)]
pub fn global_table_lock_exclusively__7ee920dee033dfbf(self_ptr: *mut mujoco__GlobalTable) -> mujoco__ReentrantWriteLock {
    todo!() // cxx:__ZN6mujoco11GlobalTableI10mjpEncodervE15LockExclusivelyEv
}

/// C: GlobalTable::LockExclusively (engine/engine_global_table.h:114)
#[allow(unused_variables, non_snake_case)]
pub fn global_table_lock_exclusively__e7cae0b3efb89732(self_ptr: *mut mujoco__GlobalTable) -> mujoco__ReentrantWriteLock {
    todo!() // cxx:__ZN6mujoco11GlobalTableI10mjpPlugin_vE15LockExclusivelyEv
}

/// C: GlobalTable::LockExclusively (engine/engine_global_table.h:114)
#[allow(unused_variables, non_snake_case)]
pub fn global_table_lock_exclusively__2686bc315fcf7a1a(self_ptr: *mut mujoco__GlobalTable) -> mujoco__ReentrantWriteLock {
    todo!() // cxx:__ZN6mujoco11GlobalTableI19mjpResourceProvidervE15LockExclusivelyEv
}

/// C: GlobalTable::AppendIfUnique (engine/engine_global_table.h:118)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_global_table.h:__ZN6mujocoL20CaseInsensitiveEqualENSt3__117basic_string_viewIcNS0_11char_traitsIcEEEES4_, cxx:_mju_error
#[allow(unused_variables, non_snake_case)]
pub fn global_table_append_if_unique__f0aa49ee983d77d1(self_ptr: *mut mujoco__GlobalTable, obj: *const mjpDecoder) -> i32 {
    todo!() // cxx:__ZN6mujoco11GlobalTableI10mjpDecodervE14AppendIfUniqueERKS1_
}

/// C: GlobalTable::AppendIfUnique (engine/engine_global_table.h:118)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_global_table.h:__ZN6mujocoL20CaseInsensitiveEqualENSt3__117basic_string_viewIcNS0_11char_traitsIcEEEES4_, cxx:_mju_error
#[allow(unused_variables, non_snake_case)]
pub fn global_table_append_if_unique__282a66ed7cc56385(self_ptr: *mut mujoco__GlobalTable, obj: *const mjpEncoder) -> i32 {
    todo!() // cxx:__ZN6mujoco11GlobalTableI10mjpEncodervE14AppendIfUniqueERKS1_
}

/// C: GlobalTable::AppendIfUnique (engine/engine_global_table.h:118)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_global_table.h:__ZN6mujocoL20CaseInsensitiveEqualENSt3__117basic_string_viewIcNS0_11char_traitsIcEEEES4_, cxx:_mju_error
#[allow(unused_variables, non_snake_case)]
pub fn global_table_append_if_unique__7ac2f54bee6d5cdc(self_ptr: *mut mujoco__GlobalTable, obj: *const mjpPlugin) -> i32 {
    todo!() // cxx:__ZN6mujoco11GlobalTableI10mjpPlugin_vE14AppendIfUniqueERKS1_
}

/// C: GlobalTable::AppendIfUnique (engine/engine_global_table.h:118)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_global_table.h:__ZN6mujocoL20CaseInsensitiveEqualENSt3__117basic_string_viewIcNS0_11char_traitsIcEEEES4_, cxx:_mju_error
#[allow(unused_variables, non_snake_case)]
pub fn global_table_append_if_unique__cefee1f775098f71(self_ptr: *mut mujoco__GlobalTable, obj: *const mjpResourceProvider) -> i32 {
    todo!() // cxx:__ZN6mujoco11GlobalTableI19mjpResourceProvidervE14AppendIfUniqueERKS1_
}

/// C: GlobalTable::GetAtSlotUnsafe (engine/engine_global_table.h:184)
#[allow(unused_variables, non_snake_case)]
pub fn global_table_get_at_slot_unsafe__da2211c8f356d2a0(self_ptr: *mut mujoco__GlobalTable, slot: i32, nslot: i32) -> *const mjpPlugin {
    todo!() // cxx:__ZN6mujoco11GlobalTableI10mjpPlugin_vE15GetAtSlotUnsafeEii
}

/// C: GlobalTable::GetAtSlotUnsafe (engine/engine_global_table.h:184)
#[allow(unused_variables, non_snake_case)]
pub fn global_table_get_at_slot_unsafe__1a76b74aa39bb757(self_ptr: *mut mujoco__GlobalTable, slot: i32, nslot: i32) -> *const mjpResourceProvider {
    todo!() // cxx:__ZN6mujoco11GlobalTableI19mjpResourceProvidervE15GetAtSlotUnsafeEii
}

/// C: GlobalTable::GetByKeyUnsafe (engine/engine_global_table.h:213)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_global_table.h:__ZN6mujocoL20CaseInsensitiveEqualENSt3__117basic_string_viewIcNS0_11char_traitsIcEEEES4_
#[allow(unused_variables, non_snake_case)]
pub fn global_table_get_by_key_unsafe__911a1519087639b3(self_ptr: *mut mujoco__GlobalTable, key: std__string_view, slot: *mut i32, nslot: i32) -> *const mjpDecoder {
    todo!() // cxx:__ZN6mujoco11GlobalTableI10mjpDecodervE14GetByKeyUnsafeENSt3__117basic_string_viewIcNS3_11char_traitsIcEEEEPii
}

/// C: GlobalTable::GetByKeyUnsafe (engine/engine_global_table.h:213)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_global_table.h:__ZN6mujocoL20CaseInsensitiveEqualENSt3__117basic_string_viewIcNS0_11char_traitsIcEEEES4_
#[allow(unused_variables, non_snake_case)]
pub fn global_table_get_by_key_unsafe__dad1b6055a9eab67(self_ptr: *mut mujoco__GlobalTable, key: std__string_view, slot: *mut i32, nslot: i32) -> *const mjpEncoder {
    todo!() // cxx:__ZN6mujoco11GlobalTableI10mjpEncodervE14GetByKeyUnsafeENSt3__117basic_string_viewIcNS3_11char_traitsIcEEEEPii
}

/// C: GlobalTable::GetByKeyUnsafe (engine/engine_global_table.h:213)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_global_table.h:__ZN6mujocoL20CaseInsensitiveEqualENSt3__117basic_string_viewIcNS0_11char_traitsIcEEEES4_
#[allow(unused_variables, non_snake_case)]
pub fn global_table_get_by_key_unsafe__4c80c65cf919c902(self_ptr: *mut mujoco__GlobalTable, key: std__string_view, slot: *mut i32, nslot: i32) -> *const mjpPlugin {
    todo!() // cxx:__ZN6mujoco11GlobalTableI10mjpPlugin_vE14GetByKeyUnsafeENSt3__117basic_string_viewIcNS3_11char_traitsIcEEEEPii
}

/// C: GlobalTable::GetByKeyUnsafe (engine/engine_global_table.h:213)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_global_table.h:__ZN6mujocoL20CaseInsensitiveEqualENSt3__117basic_string_viewIcNS0_11char_traitsIcEEEES4_
#[allow(unused_variables, non_snake_case)]
pub fn global_table_get_by_key_unsafe__b01667618fc164e8(self_ptr: *mut mujoco__GlobalTable, key: std__string_view, slot: *mut i32, nslot: i32) -> *const mjpResourceProvider {
    todo!() // cxx:__ZN6mujoco11GlobalTableI19mjpResourceProvidervE14GetByKeyUnsafeENSt3__117basic_string_viewIcNS3_11char_traitsIcEEEEPii
}

/// C: GlobalTable::GetAtSlot (engine/engine_global_table.h:248)
#[allow(unused_variables, non_snake_case)]
pub fn global_table_get_at_slot__05569da43bd05720(self_ptr: *mut mujoco__GlobalTable, slot: i32) -> *const mjpPlugin {
    todo!() // cxx:__ZN6mujoco11GlobalTableI10mjpPlugin_vE9GetAtSlotEi
}

/// C: GlobalTable::GetAtSlot (engine/engine_global_table.h:248)
#[allow(unused_variables, non_snake_case)]
pub fn global_table_get_at_slot__c5b3fb38e69c66a9(self_ptr: *mut mujoco__GlobalTable, slot: i32) -> *const mjpResourceProvider {
    todo!() // cxx:__ZN6mujoco11GlobalTableI19mjpResourceProvidervE9GetAtSlotEi
}

/// C: GlobalTable::GetByKey (engine/engine_global_table.h:254)
#[allow(unused_variables, non_snake_case)]
pub fn global_table_get_by_key__c3a4b4a11c0beac4(self_ptr: *mut mujoco__GlobalTable, key: std__string_view, slot: *mut i32) -> *const mjpDecoder {
    todo!() // cxx:__ZN6mujoco11GlobalTableI10mjpDecodervE8GetByKeyENSt3__117basic_string_viewIcNS3_11char_traitsIcEEEEPi
}

/// C: GlobalTable::GetByKey (engine/engine_global_table.h:254)
#[allow(unused_variables, non_snake_case)]
pub fn global_table_get_by_key__345fb897d41f92f7(self_ptr: *mut mujoco__GlobalTable, key: std__string_view, slot: *mut i32) -> *const mjpEncoder {
    todo!() // cxx:__ZN6mujoco11GlobalTableI10mjpEncodervE8GetByKeyENSt3__117basic_string_viewIcNS3_11char_traitsIcEEEEPi
}

/// C: GlobalTable::GetByKey (engine/engine_global_table.h:254)
#[allow(unused_variables, non_snake_case)]
pub fn global_table_get_by_key__d3b39d0fded63db9(self_ptr: *mut mujoco__GlobalTable, key: std__string_view, slot: *mut i32) -> *const mjpPlugin {
    todo!() // cxx:__ZN6mujoco11GlobalTableI10mjpPlugin_vE8GetByKeyENSt3__117basic_string_viewIcNS3_11char_traitsIcEEEEPi
}

/// C: GlobalTable::GetByKey (engine/engine_global_table.h:254)
#[allow(unused_variables, non_snake_case)]
pub fn global_table_get_by_key__f722c3d5d7d18433(self_ptr: *mut mujoco__GlobalTable, key: std__string_view, slot: *mut i32) -> *const mjpResourceProvider {
    todo!() // cxx:__ZN6mujoco11GlobalTableI19mjpResourceProvidervE8GetByKeyENSt3__117basic_string_viewIcNS3_11char_traitsIcEEEEPi
}

/// C: GlobalTable::mutex (engine/engine_global_table.h:265)
#[allow(unused_variables, non_snake_case)]
pub fn global_table_mutex__686528513442cfa1(self_ptr: *mut mujoco__GlobalTable) -> *mut mujoco__Mutex {
    todo!() // cxx:__ZN6mujoco11GlobalTableI10mjpDecodervE5mutexEv
}

/// C: GlobalTable::mutex (engine/engine_global_table.h:265)
#[allow(unused_variables, non_snake_case)]
pub fn global_table_mutex__8273347826c909c4(self_ptr: *mut mujoco__GlobalTable) -> *mut mujoco__Mutex {
    todo!() // cxx:__ZN6mujoco11GlobalTableI10mjpEncodervE5mutexEv
}

/// C: GlobalTable::mutex (engine/engine_global_table.h:265)
#[allow(unused_variables, non_snake_case)]
pub fn global_table_mutex__5a3b2e74bbeb8892(self_ptr: *mut mujoco__GlobalTable) -> *mut mujoco__Mutex {
    todo!() // cxx:__ZN6mujoco11GlobalTableI10mjpPlugin_vE5mutexEv
}

/// C: GlobalTable::mutex (engine/engine_global_table.h:265)
#[allow(unused_variables, non_snake_case)]
pub fn global_table_mutex__f7bd3e65735482bb(self_ptr: *mut mujoco__GlobalTable) -> *mut mujoco__Mutex {
    todo!() // cxx:__ZN6mujoco11GlobalTableI19mjpResourceProvidervE5mutexEv
}

