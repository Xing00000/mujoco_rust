//! Port of: engine/engine_plugin.cc
//! IR hash: 32301b9dc9774d55
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: strklen (engine/engine_plugin.cc:58)
#[allow(unused_variables, non_snake_case)]
pub fn strklen(s: *const i8) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (s : * const i8)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: getext (engine/engine_plugin.cc:68)
#[allow(unused_variables, non_snake_case)]
pub fn getext(filename: string_view) -> std__string {
    // NOTE: signature changed from previous IR version
    // Previous params: (filename : string_view)
    // Previous return: std__string
    todo!("re-translate: params renamed")
}

/// C: CopyName (engine/engine_plugin.cc:78)
/// Calls: strklen
#[allow(unused_variables, non_snake_case)]
pub fn copy_name(s: *const i8) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (s : * const i8)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: IsValidURISchemeFormat (engine/engine_plugin.cc:93)
#[allow(unused_variables, non_snake_case)]
pub fn is_valid_uri_scheme_format(prefix: *const i8) -> bool {
    // NOTE: signature changed from previous IR version
    // Previous params: (prefix : * const i8)
    // Previous return: bool
    todo!("re-translate: params renamed")
}

/// C: PluginAttrSeek (engine/engine_plugin.cc:119)
#[allow(unused_variables, non_snake_case)]
pub fn plugin_attr_seek(m: *const mjModel, plugin_id: i32, attrib_id: i32) -> *const i8 {
    // NOTE: signature changed from previous IR version
    // Previous params: (m : * const mjModel, plugin_id : i32, attrib_id : i32)
    // Previous return: * const i8
    todo!("re-translate: params renamed")
}

/// C: mjp_defaultPlugin (engine/engine_plugin.h:26)
#[allow(unused_variables, non_snake_case)]
pub fn mjp_default_plugin(plugin: *mut mjpPlugin) {
    // SAFETY: caller guarantees plugin points to valid mjpPlugin (152 bytes)
    unsafe {
        std::ptr::write_bytes(plugin as *mut u8, 0, std::mem::size_of::<mjpPlugin>());
    }
}

/// C: mjp_registerPlugin (engine/engine_plugin.h:29)
/// Calls: mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mjp_register_plugin(plugin: *const mjpPlugin) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (plugin : * const mjpPlugin)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: mjp_registerResourceProvider (engine/engine_plugin.h:32)
/// Calls: IsValidURISchemeFormat, mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn mjp_register_resource_provider(provider: *const mjpResourceProvider) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (provider : * const mjpResourceProvider)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: mjp_pluginCount (engine/engine_plugin.h:35)
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
#[allow(unused_variables, non_snake_case)]
pub fn mjp_get_plugin(name: *const i8, slot: *mut i32) -> *const mjpPlugin {
    todo!("requires global plugin table")
}

/// C: mjp_defaultResourceProvider (engine/engine_plugin.h:44)
#[allow(unused_variables, non_snake_case)]
pub fn mjp_default_resource_provider(provider: *mut mjpResourceProvider) {
    // SAFETY: caller guarantees provider points to valid mjpResourceProvider (64 bytes)
    unsafe {
        std::ptr::write_bytes(provider as *mut u8, 0, std::mem::size_of::<mjpResourceProvider>());
    }
}

/// C: mjp_getResourceProvider (engine/engine_plugin.h:47)
#[allow(unused_variables, non_snake_case)]
pub fn mjp_get_resource_provider(resource_name: *const i8) -> *const mjpResourceProvider {
    todo!("requires global resource provider table")
}

/// C: mjp_getPluginAtSlot (engine/engine_plugin.h:50)
#[allow(unused_variables, non_snake_case)]
pub fn mjp_get_plugin_at_slot(slot: i32) -> *const mjpPlugin {
    // SAFETY: delegates to GlobalTable<mjpPlugin>::GetSingleton().GetAtSlot(slot)
    unsafe {
        crate::engine::engine_global_table::global_table_get_at_slot(
            crate::engine::engine_global_table::global_table_get_singleton() as *mut GlobalTable,
            slot,
        ) as *const mjpPlugin
    }
}

/// C: mjp_getResourceProviderAtSlot (engine/engine_plugin.h:53)
#[allow(unused_variables, non_snake_case)]
pub fn mjp_get_resource_provider_at_slot(slot: i32) -> *const mjpResourceProvider {
    todo!("requires global resource provider table")
}

/// C: mj_getPluginConfig (engine/engine_plugin.h:57)
/// Calls: PluginAttrSeek, mjp_getPluginAtSlot
#[allow(unused_variables, non_snake_case)]
pub fn mj_get_plugin_config(m: *const mjModel, plugin_id: i32, attrib: *const i8) -> *const i8 {
    // NOTE: signature changed from previous IR version
    // Previous params: (m : * const mjModel, plugin_id : i32, attrib : * const i8)
    // Previous return: * const i8
    todo!("re-translate: params renamed")
}

/// C: mj_loadPluginLibrary (engine/engine_plugin.h:60)
/// Calls: mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mj_load_plugin_library(path: *const i8) {
    // NOTE: signature changed from previous IR version
    // Previous params: (path : * const i8)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mj_loadAllPluginLibraries (engine/engine_plugin.h:63)
/// Calls: mjp_pluginCount
#[allow(unused_variables, non_snake_case)]
pub fn mj_load_all_plugin_libraries(directory: *const i8, callback: mjfPluginLibraryLoadCallback) {
    // NOTE: signature changed from previous IR version
    // Previous params: (directory : * const i8, callback : mjfPluginLibraryLoadCallback)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjp_registerDecoder (engine/engine_plugin.h:66)
/// Calls: mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn mjp_register_decoder(decoder: *const mjpDecoder) {
    // NOTE: signature changed from previous IR version
    // Previous params: (decoder : * const mjpDecoder)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjp_defaultDecoder (engine/engine_plugin.h:69)
#[allow(unused_variables, non_snake_case)]
pub fn mjp_default_decoder(decoder: *mut mjpDecoder) {
    // SAFETY: caller guarantees decoder points to valid mjpDecoder (32 bytes)
    unsafe {
        std::ptr::write_bytes(decoder as *mut u8, 0, std::mem::size_of::<mjpDecoder>());
    }
}

/// C: mjp_findDecoder (engine/engine_plugin.h:72)
/// Calls: mju_warning, strklen
#[allow(unused_variables, non_snake_case)]
pub fn mjp_find_decoder(resource: *const mjResource, content_type: *const i8) -> *const mjpDecoder {
    // NOTE: signature changed from previous IR version
    // Previous params: (resource : * const mjResource, content_type : * const i8)
    // Previous return: * const mjpDecoder
    todo!("re-translate: params renamed")
}

/// C: mjp_registerEncoder (engine/engine_plugin.h:75)
/// Calls: mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn mjp_register_encoder(encoder: *const mjpEncoder) {
    // NOTE: signature changed from previous IR version
    // Previous params: (encoder : * const mjpEncoder)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjp_defaultEncoder (engine/engine_plugin.h:78)
#[allow(unused_variables, non_snake_case)]
pub fn mjp_default_encoder(encoder: *mut mjpEncoder) {
    // SAFETY: caller guarantees encoder points to valid mjpEncoder (32 bytes)
    unsafe {
        std::ptr::write_bytes(encoder as *mut u8, 0, std::mem::size_of::<mjpEncoder>());
    }
}

/// C: mjp_findEncoder (engine/engine_plugin.h:81)
/// Calls: mju_warning, strklen
#[allow(unused_variables, non_snake_case)]
pub fn mjp_find_encoder(filename: *const i8, content_type: *const i8) -> *const mjpEncoder {
    // NOTE: signature changed from previous IR version
    // Previous params: (filename : * const i8, content_type : * const i8)
    // Previous return: * const mjpEncoder
    todo!("re-translate: params renamed")
}

/// C: mjp_getPluginUnsafe (engine/engine_plugin.h:95)
#[allow(unused_variables, non_snake_case)]
pub fn mjp_get_plugin_unsafe(name: *const i8, slot: *mut i32, nslot: i32) -> *const mjpPlugin {
    todo!("requires global plugin table")
}

/// C: mjp_getPluginAtSlotUnsafe (engine/engine_plugin.h:98)
#[allow(unused_variables, non_snake_case)]
pub fn mjp_get_plugin_at_slot_unsafe(slot: i32, nslot: i32) -> *const mjpPlugin {
    todo!("requires global plugin table")
}

