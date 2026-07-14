//! Port of: engine/engine_plugin.cc
//! IR hash: 9614bd3d92e7766b
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: strklen (engine/engine_plugin.cc:58)
#[allow(unused_variables, non_snake_case)]
pub fn strklen(s: *const i8) -> i32 {
    todo!() // strklen
}

/// C: getext (engine/engine_plugin.cc:68)
#[allow(unused_variables, non_snake_case)]
pub fn getext(filename: string_view) -> std__string {
    todo!() // getext
}

/// C: CopyName (engine/engine_plugin.cc:78)
/// Calls: strklen
#[allow(unused_variables, non_snake_case)]
pub fn copy_name(s: *const i8) -> *const () {
    todo!() // CopyName
}

/// C: IsValidURISchemeFormat (engine/engine_plugin.cc:93)
#[allow(unused_variables, non_snake_case)]
pub fn is_valid_uri_scheme_format(prefix: *const i8) -> bool {
    todo!() // IsValidURISchemeFormat
}

/// C: PluginAttrSeek (engine/engine_plugin.cc:119)
#[allow(unused_variables, non_snake_case)]
pub fn plugin_attr_seek(m: *const mjModel, plugin_id: i32, attrib_id: i32) -> *const i8 {
    todo!() // PluginAttrSeek
}

/// C: GlobalTable::HumanReadableTypeName (engine/engine_plugin.cc:132)
#[allow(unused_variables, non_snake_case)]
pub fn global_table_human_readable_type_name() -> *const i8 {
    todo!() // GlobalTable::HumanReadableTypeName
}

/// C: GlobalTable::ObjectKey (engine/engine_plugin.cc:137)
/// Calls: strklen
#[allow(unused_variables, non_snake_case)]
pub fn global_table_object_key(arg0: *const mjpPlugin) -> std__string_view {
    todo!() // GlobalTable::ObjectKey
}

/// C: GlobalTable::ObjectEqual (engine/engine_plugin.cc:143)
#[allow(unused_variables, non_snake_case)]
pub fn global_table_object_equal(arg0: *const mjpPlugin, arg1: *const mjpPlugin) -> bool {
    todo!() // GlobalTable::ObjectEqual
}

/// C: GlobalTable::CopyObject (engine/engine_plugin.cc:182)
/// Calls: CopyName, strklen
#[allow(unused_variables, non_snake_case)]
pub fn global_table_copy_object(dst: *mut mjpPlugin, src: *const mjpPlugin, err: *mut ErrorMessage) -> bool {
    todo!() // GlobalTable::CopyObject
}

/// C: mjp_defaultPlugin (engine/engine_plugin.h:26)
#[allow(unused_variables, non_snake_case)]
pub fn mjp_default_plugin(plugin: *mut mjpPlugin) {
    todo!() // mjp_defaultPlugin
}

/// C: mjp_registerPlugin (engine/engine_plugin.h:29)
/// Calls: GlobalTable::AppendIfUnique, GlobalTable::GetSingleton, mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mjp_register_plugin(plugin: *const mjpPlugin) -> i32 {
    todo!() // mjp_registerPlugin
}

/// C: mjp_registerResourceProvider (engine/engine_plugin.h:32)
/// Calls: GlobalTable::AppendIfUnique, GlobalTable::GetSingleton, IsValidURISchemeFormat, mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn mjp_register_resource_provider(provider: *const mjpResourceProvider) -> i32 {
    todo!() // mjp_registerResourceProvider
}

/// C: mjp_pluginCount (engine/engine_plugin.h:35)
/// Calls: GlobalTable::GetSingleton, GlobalTable::count
#[allow(unused_variables, non_snake_case)]
pub fn mjp_plugin_count() -> i32 {
    // SAFETY: delegates to GlobalTable<mjpPlugin>::GetSingleton().count()
    // The singleton is always valid once initialized.
    unsafe {
        crate::engine::engine_global_table::global_table_count(
            crate::engine::engine_global_table::global_table_get_singleton() as *mut GlobalTable,
        )
    }
}

/// C: mjp_resourceProviderCount (engine/engine_plugin.h:38)
/// Calls: GlobalTable::GetSingleton, GlobalTable::count
#[allow(unused_variables, non_snake_case)]
pub fn mjp_resource_provider_count() -> i32 {
    // SAFETY: delegates to GlobalTable<mjpResourceProvider>::GetSingleton().count()
    unsafe {
        crate::engine::engine_global_table::global_table_count(
            crate::engine::engine_global_table::global_table_get_singleton() as *mut GlobalTable,
        )
    }
}

/// C: mjp_getPlugin (engine/engine_plugin.h:41)
/// Calls: GlobalTable::GetByKey, GlobalTable::GetSingleton
#[allow(unused_variables, non_snake_case)]
pub fn mjp_get_plugin(name: *const i8, slot: *mut i32) -> *const mjpPlugin {
    // SAFETY: delegates to GlobalTable<mjpPlugin>::GetSingleton().GetByKey(name, slot)
    // Singleton is always valid. name is a C string passed from caller.
    unsafe {
        crate::engine::engine_global_table::global_table_get_by_key(
            crate::engine::engine_global_table::global_table_get_singleton() as *mut GlobalTable,
            *(name as *const string_view),
            slot,
        ) as *const mjpPlugin
    }
}

/// C: mjp_defaultResourceProvider (engine/engine_plugin.h:44)
#[allow(unused_variables, non_snake_case)]
pub fn mjp_default_resource_provider(provider: *mut mjpResourceProvider) {
    todo!() // mjp_defaultResourceProvider
}

/// C: mjp_getResourceProvider (engine/engine_plugin.h:47)
/// Calls: GlobalTable::GetByKey, GlobalTable::GetSingleton, IsValidURISchemeFormat
#[allow(unused_variables, non_snake_case)]
pub fn mjp_get_resource_provider(resource_name: *const i8) -> *const mjpResourceProvider {
    todo!() // mjp_getResourceProvider
}

/// C: mjp_getPluginAtSlot (engine/engine_plugin.h:50)
/// Calls: GlobalTable::GetAtSlot, GlobalTable::GetSingleton
#[allow(unused_variables, non_snake_case)]
pub fn mjp_get_plugin_at_slot(slot: i32) -> *const mjpPlugin {
    // SAFETY: delegates to GlobalTable<mjpPlugin>::GetSingleton().GetAtSlot(slot)
    // Singleton is always valid. Slot bounds checked inside GetAtSlot.
    unsafe {
        crate::engine::engine_global_table::global_table_get_at_slot(
            crate::engine::engine_global_table::global_table_get_singleton() as *mut GlobalTable,
            slot,
        ) as *const mjpPlugin
    }
}

/// C: mjp_getResourceProviderAtSlot (engine/engine_plugin.h:53)
/// Calls: GlobalTable::GetAtSlot, GlobalTable::GetSingleton
#[allow(unused_variables, non_snake_case)]
pub fn mjp_get_resource_provider_at_slot(slot: i32) -> *const mjpResourceProvider {
    // SAFETY: delegates to GlobalTable<mjpResourceProvider>::GetSingleton().GetAtSlot(slot - 1)
    // Singleton is always valid. slot-1 shifts to zero-indexed as in C++ original.
    unsafe {
        crate::engine::engine_global_table::global_table_get_at_slot(
            crate::engine::engine_global_table::global_table_get_singleton() as *mut GlobalTable,
            slot - 1,
        ) as *const mjpResourceProvider
    }
}

/// C: mj_getPluginConfig (engine/engine_plugin.h:57)
/// Calls: PluginAttrSeek, mjp_getPluginAtSlot
#[allow(unused_variables, non_snake_case)]
pub fn mj_get_plugin_config(m: *const mjModel, plugin_id: i32, attrib: *const i8) -> *const i8 {
    todo!() // mj_getPluginConfig
}

/// C: mj_loadPluginLibrary (engine/engine_plugin.h:60)
/// Calls: mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mj_load_plugin_library(path: *const i8) {
    todo!() // mj_loadPluginLibrary
}

/// C: mj_loadAllPluginLibraries (engine/engine_plugin.h:63)
/// Calls: GlobalTable::GetSingleton, GlobalTable::LockExclusively, mj_loadPluginLibrary, mjp_pluginCount
#[allow(unused_variables, non_snake_case)]
pub fn mj_load_all_plugin_libraries(directory: *const i8, callback: mjfPluginLibraryLoadCallback) {
    todo!() // mj_loadAllPluginLibraries
}

/// C: mjp_registerDecoder (engine/engine_plugin.h:66)
/// Calls: GlobalTable::AppendIfUnique, GlobalTable::GetSingleton, mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn mjp_register_decoder(decoder: *const mjpDecoder) {
    todo!() // mjp_registerDecoder
}

/// C: mjp_defaultDecoder (engine/engine_plugin.h:69)
#[allow(unused_variables, non_snake_case)]
pub fn mjp_default_decoder(decoder: *mut mjpDecoder) {
    todo!() // mjp_defaultDecoder
}

/// C: mjp_findDecoder (engine/engine_plugin.h:72)
/// Calls: GlobalTable::GetByKey, GlobalTable::GetSingleton, getext, mju_warning, strklen
#[allow(unused_variables, non_snake_case)]
pub fn mjp_find_decoder(resource: *const mjResource, content_type: *const i8) -> *const mjpDecoder {
    todo!() // mjp_findDecoder
}

/// C: mjp_registerEncoder (engine/engine_plugin.h:75)
/// Calls: GlobalTable::AppendIfUnique, GlobalTable::GetSingleton, mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn mjp_register_encoder(encoder: *const mjpEncoder) {
    todo!() // mjp_registerEncoder
}

/// C: mjp_defaultEncoder (engine/engine_plugin.h:78)
#[allow(unused_variables, non_snake_case)]
pub fn mjp_default_encoder(encoder: *mut mjpEncoder) {
    todo!() // mjp_defaultEncoder
}

/// C: mjp_findEncoder (engine/engine_plugin.h:81)
/// Calls: GlobalTable::GetByKey, GlobalTable::GetSingleton, getext, mju_warning, strklen
#[allow(unused_variables, non_snake_case)]
pub fn mjp_find_encoder(filename: *const i8, content_type: *const i8) -> *const mjpEncoder {
    todo!() // mjp_findEncoder
}

/// C: mjp_getPluginUnsafe (engine/engine_plugin.h:95)
/// Calls: GlobalTable::GetByKeyUnsafe, GlobalTable::GetSingleton
#[allow(unused_variables, non_snake_case)]
pub fn mjp_get_plugin_unsafe(name: *const i8, slot: *mut i32, nslot: i32) -> *const mjpPlugin {
    // SAFETY: delegates to GlobalTable<mjpPlugin>::GetSingleton().GetByKeyUnsafe(name, slot, nslot)
    // Caller guarantees nslot was obtained from a prior mjp_pluginCount call.
    unsafe {
        crate::engine::engine_global_table::global_table_get_by_key_unsafe(
            crate::engine::engine_global_table::global_table_get_singleton() as *mut GlobalTable,
            *(name as *const string_view),
            slot,
            nslot,
        ) as *const mjpPlugin
    }
}

/// C: mjp_getPluginAtSlotUnsafe (engine/engine_plugin.h:98)
/// Calls: GlobalTable::GetAtSlotUnsafe, GlobalTable::GetSingleton
#[allow(unused_variables, non_snake_case)]
pub fn mjp_get_plugin_at_slot_unsafe(slot: i32, nslot: i32) -> *const mjpPlugin {
    // SAFETY: delegates to GlobalTable<mjpPlugin>::GetSingleton().GetAtSlotUnsafe(slot, nslot)
    // Caller guarantees nslot was obtained from a prior mjp_pluginCount call.
    unsafe {
        crate::engine::engine_global_table::global_table_get_at_slot_unsafe(
            crate::engine::engine_global_table::global_table_get_singleton() as *mut GlobalTable,
            slot,
            nslot,
        ) as *const mjpPlugin
    }
}

