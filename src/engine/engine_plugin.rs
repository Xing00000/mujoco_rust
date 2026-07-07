//! Port of: engine/engine_plugin.cc
//! IR hash: 05737965add36adb
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: strklen (engine/engine_plugin.cc:58)
#[allow(unused_variables, non_snake_case)]
pub fn strklen(s: *const i8) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (s : * const i8)
    // Previous return: i32
    const K_MAX_NAME_LENGTH : i32 = 1024 ; unsafe { for i in 0 .. K_MAX_NAME_LENGTH { if * s . add (i as usize) == 0 { return i ; } } - 1 }
}

/// C: getext (engine/engine_plugin.cc:68)
#[allow(unused_variables, non_snake_case)]
pub fn getext(filename: string_view) -> std__string {
    // WARNING: signature changed — verify body
    // Previous params: (filename : string_view)
    // Previous return: std__string
    todo ! ()
}

/// C: CopyName (engine/engine_plugin.cc:78)
/// Calls: strklen
#[allow(unused_variables, non_snake_case)]
pub fn copy_name(s: *const i8) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (s : * const i8)
    // Previous return: i32
    extern "C" { fn CopyName_impl (s : * const i8) -> i32 ; } unsafe { CopyName_impl (s) }
}

/// C: IsValidURISchemeFormat (engine/engine_plugin.cc:93)
#[allow(unused_variables, non_snake_case)]
pub fn is_valid_uri_scheme_format(prefix: *const i8) -> bool {
    // WARNING: signature changed — verify body
    // Previous params: (prefix : * const i8)
    // Previous return: bool
    extern "C" { fn IsValidURISchemeFormat_impl (prefix : * const i8) -> bool ; } unsafe { IsValidURISchemeFormat_impl (prefix) }
}

/// C: PluginAttrSeek (engine/engine_plugin.cc:119)
#[allow(unused_variables, non_snake_case)]
pub fn plugin_attr_seek(m: *const mjModel, plugin_id: i32, attrib_id: i32) -> *const i8 {
    extern "C" { fn PluginAttrSeek_impl(m: *const mjModel, plugin_id: i32, attrib_id: i32) -> *const i8; }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { PluginAttrSeek_impl(m, plugin_id, attrib_id) }
}

/// C: mjp_defaultPlugin (engine/engine_plugin.h:26)
#[allow(unused_variables, non_snake_case)]
pub fn mjp_default_plugin(plugin: *mut mjpPlugin) {
    // WARNING: signature changed — verify body
    // Previous params: (plugin : * mut mjpPlugin)
    // Previous return: ()
    todo ! ()
}

/// C: mjp_registerPlugin (engine/engine_plugin.h:29)
/// Calls: mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mjp_register_plugin(plugin: *const mjpPlugin) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (plugin : * const mjpPlugin)
    // Previous return: i32
    todo ! ()
}

/// C: mjp_registerResourceProvider (engine/engine_plugin.h:32)
/// Calls: IsValidURISchemeFormat, mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn mjp_register_resource_provider(provider: *const mjpResourceProvider) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (provider : * const mjpResourceProvider)
    // Previous return: i32
    todo ! ()
}

/// C: mjp_pluginCount (engine/engine_plugin.h:35)
#[allow(unused_variables, non_snake_case)]
pub fn mjp_plugin_count() -> i32 {
    extern "C" { fn mjp_pluginCount_impl () -> i32 ; } unsafe { mjp_pluginCount_impl () }
}

/// C: mjp_resourceProviderCount (engine/engine_plugin.h:38)
#[allow(unused_variables, non_snake_case)]
pub fn mjp_resource_provider_count() -> i32 {
    extern "C" { fn mjp_resourceProviderCount_impl() -> i32; }
    // SAFETY: delegates to C implementation, no pointers to validate
    unsafe { mjp_resourceProviderCount_impl() }
}

/// C: mjp_getPlugin (engine/engine_plugin.h:41)
#[allow(unused_variables, non_snake_case)]
pub fn mjp_get_plugin(name: *const i8, slot: *mut i32) -> *const mjpPlugin {
    extern "C" { fn mjp_getPlugin_impl(name: *const i8, slot: *mut i32) -> *const mjpPlugin; }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mjp_getPlugin_impl(name, slot) }
}

/// C: mjp_defaultResourceProvider (engine/engine_plugin.h:44)
#[allow(unused_variables, non_snake_case)]
pub fn mjp_default_resource_provider(provider: *mut mjpResourceProvider) {
    extern "C" { fn mjp_defaultResourceProvider_impl(provider: *mut mjpResourceProvider); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mjp_defaultResourceProvider_impl(provider) }
}

/// C: mjp_getResourceProvider (engine/engine_plugin.h:47)
#[allow(unused_variables, non_snake_case)]
pub fn mjp_get_resource_provider(resource_name: *const i8) -> *const mjpResourceProvider {
    extern "C" { fn mjp_getResourceProvider_impl(resource_name: *const i8) -> *const mjpResourceProvider; }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mjp_getResourceProvider_impl(resource_name) }
}

/// C: mjp_getPluginAtSlot (engine/engine_plugin.h:50)
#[allow(unused_variables, non_snake_case)]
pub fn mjp_get_plugin_at_slot(slot: i32) -> *const mjpPlugin {
    // WARNING: signature changed — verify body
    // Previous params: (slot : i32)
    // Previous return: * const mjpPlugin
    extern "C" { fn mjp_getPluginAtSlot_impl (slot : i32) -> * const mjpPlugin ; } unsafe { mjp_getPluginAtSlot_impl (slot) }
}

/// C: mjp_getResourceProviderAtSlot (engine/engine_plugin.h:53)
#[allow(unused_variables, non_snake_case)]
pub fn mjp_get_resource_provider_at_slot(slot: i32) -> *const mjpResourceProvider {
    extern "C" { fn mjp_getResourceProviderAtSlot_impl(slot: i32) -> *const mjpResourceProvider; }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mjp_getResourceProviderAtSlot_impl(slot) }
}

/// C: mj_getPluginConfig (engine/engine_plugin.h:57)
/// Calls: PluginAttrSeek, mjp_getPluginAtSlot
#[allow(unused_variables, non_snake_case)]
pub fn mj_get_plugin_config(m: *const mjModel, plugin_id: i32, attrib: *const i8) -> *const i8 {
    extern "C" { fn mj_getPluginConfig_impl(m: *const mjModel, plugin_id: i32, attrib: *const i8) -> *const i8; }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_getPluginConfig_impl(m, plugin_id, attrib) }
}

/// C: mj_loadPluginLibrary (engine/engine_plugin.h:60)
/// Calls: mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mj_load_plugin_library(path: *const i8) {
    // WARNING: signature changed — verify body
    // Previous params: (path : * const i8)
    // Previous return: ()
    todo ! ()
}

/// C: mj_loadAllPluginLibraries (engine/engine_plugin.h:63)
/// Calls: mjp_pluginCount
#[allow(unused_variables, non_snake_case)]
pub fn mj_load_all_plugin_libraries(directory: *const i8, callback: mjfPluginLibraryLoadCallback) {
    // WARNING: signature changed — verify body
    // Previous params: (directory : * const i8, callback : mjfPluginLibraryLoadCallback)
    // Previous return: ()
    todo ! ()
}

/// C: mjp_registerDecoder (engine/engine_plugin.h:66)
/// Calls: mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn mjp_register_decoder(decoder: *const mjpDecoder) {
    extern "C" { fn mjp_registerDecoder_impl(decoder: *const mjpDecoder); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mjp_registerDecoder_impl(decoder) }
}

/// C: mjp_defaultDecoder (engine/engine_plugin.h:69)
#[allow(unused_variables, non_snake_case)]
pub fn mjp_default_decoder(decoder: *mut mjpDecoder) {
    extern "C" { fn mjp_defaultDecoder_impl(decoder: *mut mjpDecoder); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mjp_defaultDecoder_impl(decoder) }
}

/// C: mjp_findDecoder (engine/engine_plugin.h:72)
/// Calls: mju_warning, strklen
#[allow(unused_variables, non_snake_case)]
pub fn mjp_find_decoder(resource: *const mjResource, content_type: *const i8) -> *const mjpDecoder {
    // WARNING: signature changed — verify body
    // Previous params: (resource : * const mjResource, content_type : * const i8)
    // Previous return: * const mjpDecoder
    todo ! ()
}

/// C: mjp_registerEncoder (engine/engine_plugin.h:75)
/// Calls: mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn mjp_register_encoder(encoder: *const mjpEncoder) {
    extern "C" { fn mjp_registerEncoder_impl(encoder: *const mjpEncoder); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mjp_registerEncoder_impl(encoder) }
}

/// C: mjp_defaultEncoder (engine/engine_plugin.h:78)
#[allow(unused_variables, non_snake_case)]
pub fn mjp_default_encoder(encoder: *mut mjpEncoder) {
    extern "C" { fn mjp_defaultEncoder_impl(encoder: *mut mjpEncoder); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mjp_defaultEncoder_impl(encoder) }
}

/// C: mjp_findEncoder (engine/engine_plugin.h:81)
/// Calls: mju_warning, strklen
#[allow(unused_variables, non_snake_case)]
pub fn mjp_find_encoder(filename: *const i8, content_type: *const i8) -> *const mjpEncoder {
    extern "C" { fn mjp_findEncoder_impl(filename: *const i8, content_type: *const i8) -> *const mjpEncoder; }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mjp_findEncoder_impl(filename, content_type) }
}

/// C: mjp_getPluginUnsafe (engine/engine_plugin.h:95)
#[allow(unused_variables, non_snake_case)]
pub fn mjp_get_plugin_unsafe(name: *const i8, slot: *mut i32, nslot: i32) -> *const mjpPlugin {
    // WARNING: signature changed — verify body
    // Previous params: (name : * const i8, slot : * mut i32, nslot : i32)
    // Previous return: * const mjpPlugin
    todo ! ()
}

/// C: mjp_getPluginAtSlotUnsafe (engine/engine_plugin.h:98)
#[allow(unused_variables, non_snake_case)]
pub fn mjp_get_plugin_at_slot_unsafe(slot: i32, nslot: i32) -> *const mjpPlugin {
    // WARNING: signature changed — verify body
    // Previous params: (slot : i32, nslot : i32)
    // Previous return: * const mjpPlugin
    extern "C" { fn mjp_getPluginAtSlotUnsafe_impl (slot : i32 , nslot : i32) -> * const mjpPlugin ; } unsafe { mjp_getPluginAtSlotUnsafe_impl (slot , nslot) }
}

