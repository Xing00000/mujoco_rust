//! Port of: engine/engine_plugin.cc
//! IR hash: c6d98e4f4b63b7f2
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: strklen (engine/engine_plugin.cc:58)
#[allow(unused_variables, non_snake_case)]
pub fn strklen(s: *const i8) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (s : * const i8)
    // Previous return: i32
    todo ! ()
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
    todo ! ()
}

/// C: IsValidURISchemeFormat (engine/engine_plugin.cc:93)
#[allow(unused_variables, non_snake_case)]
pub fn is_valid_uri_scheme_format(prefix: *const i8) -> bool {
    // WARNING: signature changed — verify body
    // Previous params: (prefix : * const i8)
    // Previous return: bool
    todo ! ()
}

/// C: PluginAttrSeek (engine/engine_plugin.cc:119)
#[allow(unused_variables, non_snake_case)]
pub fn plugin_attr_seek(m: *const mjModel, plugin_id: i32, attrib_id: i32) -> *const i8 {
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, plugin_id : i32, attrib_id : i32)
    // Previous return: * const i8
    todo ! ()
}

/// C: mjp_defaultPlugin (engine/engine_plugin.h:26)
#[allow(unused_variables, non_snake_case)]
pub fn mjp_default_plugin(plugin: *mut mjpPlugin) {
    // SAFETY: caller guarantees plugin is a valid pointer to mjpPlugin
    unsafe {
        std::ptr::write_bytes(plugin as *mut u8, 0, std::mem::size_of::<mjpPlugin>());
    }
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
    // SAFETY: delegates to GlobalTable<mjpPlugin>::GetSingleton().GetByKey(name, slot)
    unsafe {
        crate::engine::engine_global_table::global_table_get_by_key(
            crate::engine::engine_global_table::global_table_get_singleton() as *mut GlobalTable,
            *((&name) as *const *const i8 as *const string_view),
            slot,
        ) as *const mjpPlugin
    }
}

/// C: mjp_defaultResourceProvider (engine/engine_plugin.h:44)
#[allow(unused_variables, non_snake_case)]
pub fn mjp_default_resource_provider(provider: *mut mjpResourceProvider) {
    // SAFETY: caller guarantees provider is a valid pointer to mjpResourceProvider
    unsafe {
        std::ptr::write_bytes(provider as *mut u8, 0, std::mem::size_of::<mjpResourceProvider>());
    }
}

/// C: mjp_getResourceProvider (engine/engine_plugin.h:47)
#[allow(unused_variables, non_snake_case)]
pub fn mjp_get_resource_provider(resource_name: *const i8) -> *const mjpResourceProvider {
    // SAFETY: pointer dereferences follow C semantics.
    // C source: parse URI scheme prefix, validate, then GetByKey on ResourceProvider table.
    unsafe {
        if resource_name.is_null() || *resource_name == 0 {
            return std::ptr::null();
        }

        // find ':'
        let mut ch: *const i8 = resource_name;
        while *ch != 0 && *ch != b':' as i8 {
            ch = ch.add(1);
        }
        if *ch == 0 {
            return std::ptr::null();
        }

        let n = ch.offset_from(resource_name) as usize;

        // validate URI scheme format: first char alpha, rest alnum/+/./- 
        if n == 0 {
            return std::ptr::null();
        }
        if !(*resource_name as u8 as char).is_ascii_alphabetic() {
            return std::ptr::null();
        }
        for i in 1..n {
            let c = *resource_name.add(i) as u8 as char;
            if !c.is_ascii_alphanumeric() && c != '+' && c != '.' && c != '-' {
                return std::ptr::null();
            }
        }

        // delegate to GetByKey with prefix
        crate::engine::engine_global_table::global_table_get_by_key(
            crate::engine::engine_global_table::global_table_get_singleton() as *mut GlobalTable,
            *((&resource_name) as *const *const i8 as *const string_view),
            std::ptr::null_mut(),
        ) as *const mjpResourceProvider
    }
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
    // SAFETY: delegates to GlobalTable<mjpResourceProvider>::GetSingleton().GetAtSlot(slot - 1)
    // Note: slot is 1-indexed for resource providers (shifted in C source)
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
    // WARNING: signature changed — verify body
    // Previous params: (m : * const mjModel, plugin_id : i32, attrib : * const i8)
    // Previous return: * const i8
    todo ! ()
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
    // WARNING: signature changed — verify body
    // Previous params: (decoder : * const mjpDecoder)
    // Previous return: ()
    todo ! ()
}

/// C: mjp_defaultDecoder (engine/engine_plugin.h:69)
#[allow(unused_variables, non_snake_case)]
pub fn mjp_default_decoder(decoder: *mut mjpDecoder) {
    // SAFETY: caller guarantees decoder is a valid pointer to mjpDecoder
    unsafe {
        std::ptr::write_bytes(decoder as *mut u8, 0, std::mem::size_of::<mjpDecoder>());
    }
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
    // WARNING: signature changed — verify body
    // Previous params: (encoder : * const mjpEncoder)
    // Previous return: ()
    todo ! ()
}

/// C: mjp_defaultEncoder (engine/engine_plugin.h:78)
#[allow(unused_variables, non_snake_case)]
pub fn mjp_default_encoder(encoder: *mut mjpEncoder) {
    // SAFETY: caller guarantees encoder is a valid pointer to mjpEncoder
    unsafe {
        std::ptr::write_bytes(encoder as *mut u8, 0, std::mem::size_of::<mjpEncoder>());
    }
}

/// C: mjp_findEncoder (engine/engine_plugin.h:81)
/// Calls: mju_warning, strklen
#[allow(unused_variables, non_snake_case)]
pub fn mjp_find_encoder(filename: *const i8, content_type: *const i8) -> *const mjpEncoder {
    // WARNING: signature changed — verify body
    // Previous params: (filename : * const i8, content_type : * const i8)
    // Previous return: * const mjpEncoder
    todo ! ()
}

/// C: mjp_getPluginUnsafe (engine/engine_plugin.h:95)
#[allow(unused_variables, non_snake_case)]
pub fn mjp_get_plugin_unsafe(name: *const i8, slot: *mut i32, nslot: i32) -> *const mjpPlugin {
    // SAFETY: delegates to GlobalTable<mjpPlugin>::GetSingleton().GetByKeyUnsafe(name, slot, nslot)
    unsafe {
        crate::engine::engine_global_table::global_table_get_by_key_unsafe(
            crate::engine::engine_global_table::global_table_get_singleton() as *mut GlobalTable,
            *((&name) as *const *const i8 as *const string_view),
            slot,
            nslot,
        ) as *const mjpPlugin
    }
}

/// C: mjp_getPluginAtSlotUnsafe (engine/engine_plugin.h:98)
#[allow(unused_variables, non_snake_case)]
pub fn mjp_get_plugin_at_slot_unsafe(slot: i32, nslot: i32) -> *const mjpPlugin {
    // SAFETY: delegates to GlobalTable<mjpPlugin>::GetSingleton().GetAtSlotUnsafe(slot, nslot)
    unsafe {
        crate::engine::engine_global_table::global_table_get_at_slot_unsafe(
            crate::engine::engine_global_table::global_table_get_singleton() as *mut GlobalTable,
            slot,
            nslot,
        ) as *const mjpPlugin
    }
}

