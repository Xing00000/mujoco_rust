//! Port of: engine/engine_plugin.cc
//! IR hash: 1b139f44af8230f9
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: strklen (engine/engine_plugin.cc:58)
#[allow(unused_variables, non_snake_case)]
pub fn strklen(s: *const i8) -> i32 {
    todo!() // strklen
}

/// C: getext (engine/engine_plugin.cc:68)
#[allow(unused_variables, non_snake_case)]
pub fn getext(filename: std__string_view) -> i32 {
    todo!() // getext
}

/// C: CopyName (engine/engine_plugin.cc:78)
/// Calls: strklen
#[allow(unused_variables, non_snake_case)]
pub fn copy_name(s: *const i8) -> i32 {
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

/// C: mjp_defaultPlugin (engine/engine_plugin.h:26)
#[allow(unused_variables, non_snake_case)]
pub fn mjp_default_plugin(plugin: *mut mjpPlugin) {
    todo!() // mjp_defaultPlugin
}

/// C: mjp_registerPlugin (engine/engine_plugin.h:29)
/// Calls: mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mjp_register_plugin(plugin: *const mjpPlugin) -> i32 {
    todo!() // mjp_registerPlugin
}

/// C: mjp_registerResourceProvider (engine/engine_plugin.h:32)
/// Calls: IsValidURISchemeFormat, mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn mjp_register_resource_provider(provider: *const mjpResourceProvider) -> i32 {
    todo!() // mjp_registerResourceProvider
}

/// C: mjp_pluginCount (engine/engine_plugin.h:35)
#[allow(unused_variables, non_snake_case)]
pub fn mjp_plugin_count() -> i32 {
    todo!() // mjp_pluginCount
}

/// C: mjp_resourceProviderCount (engine/engine_plugin.h:38)
#[allow(unused_variables, non_snake_case)]
pub fn mjp_resource_provider_count() -> i32 {
    todo!() // mjp_resourceProviderCount
}

/// C: mjp_getPlugin (engine/engine_plugin.h:41)
#[allow(unused_variables, non_snake_case)]
pub fn mjp_get_plugin(name: *const i8, slot: *mut i32) -> *const mjpPlugin {
    todo!() // mjp_getPlugin
}

/// C: mjp_defaultResourceProvider (engine/engine_plugin.h:44)
#[allow(unused_variables, non_snake_case)]
pub fn mjp_default_resource_provider(provider: *mut mjpResourceProvider) {
    todo!() // mjp_defaultResourceProvider
}

/// C: mjp_getResourceProvider (engine/engine_plugin.h:47)
#[allow(unused_variables, non_snake_case)]
pub fn mjp_get_resource_provider(resource_name: *const i8) -> *const mjpResourceProvider {
    todo!() // mjp_getResourceProvider
}

/// C: mjp_getPluginAtSlot (engine/engine_plugin.h:50)
#[allow(unused_variables, non_snake_case)]
pub fn mjp_get_plugin_at_slot(slot: i32) -> *const mjpPlugin {
    todo!() // mjp_getPluginAtSlot
}

/// C: mjp_getResourceProviderAtSlot (engine/engine_plugin.h:53)
#[allow(unused_variables, non_snake_case)]
pub fn mjp_get_resource_provider_at_slot(slot: i32) -> *const mjpResourceProvider {
    todo!() // mjp_getResourceProviderAtSlot
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
/// Calls: mjp_pluginCount
#[allow(unused_variables, non_snake_case)]
pub fn mj_load_all_plugin_libraries(directory: *const i8, callback: mjfPluginLibraryLoadCallback) {
    todo!() // mj_loadAllPluginLibraries
}

/// C: mjp_registerDecoder (engine/engine_plugin.h:66)
/// Calls: mju_warning
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
/// Calls: mju_warning, strklen
#[allow(unused_variables, non_snake_case)]
pub fn mjp_find_decoder(resource: *const mjResource, content_type: *const i8) -> *const mjpDecoder {
    todo!() // mjp_findDecoder
}

/// C: mjp_registerEncoder (engine/engine_plugin.h:75)
/// Calls: mju_warning
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
/// Calls: mju_warning, strklen
#[allow(unused_variables, non_snake_case)]
pub fn mjp_find_encoder(filename: *const i8, content_type: *const i8) -> *const mjpEncoder {
    todo!() // mjp_findEncoder
}

/// C: mjp_getPluginUnsafe (engine/engine_plugin.h:95)
#[allow(unused_variables, non_snake_case)]
pub fn mjp_get_plugin_unsafe(name: *const i8, slot: *mut i32, nslot: i32) -> *const mjpPlugin {
    todo!() // mjp_getPluginUnsafe
}

/// C: mjp_getPluginAtSlotUnsafe (engine/engine_plugin.h:98)
#[allow(unused_variables, non_snake_case)]
pub fn mjp_get_plugin_at_slot_unsafe(slot: i32, nslot: i32) -> *const mjpPlugin {
    todo!() // mjp_getPluginAtSlotUnsafe
}

