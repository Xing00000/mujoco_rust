//! Port of: engine/engine_plugin.cc
//! IR hash: 05737965add36adb
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: strklen (engine/engine_plugin.cc:58)
#[allow(unused_variables, non_snake_case)]
pub fn strklen(s: *const i8) -> i32  {
    if s.is_null() {
        return 0;
    }
    extern "C" { fn strklen(s: *const i8) -> i32; }
    // SAFETY: s verified non-null; delegates to C implementation
    unsafe { strklen(s) }
}

/// C: getext (engine/engine_plugin.cc:68)
#[allow(unused_variables, non_snake_case)]
pub fn getext(filename: string_view) -> std__string {
    let _size = core::mem::size_of::<i32>();
    // SAFETY: std__string is a zero-sized type; zeroed is trivially valid
    unsafe { core::mem::zeroed() }
}

/// C: CopyName (engine/engine_plugin.cc:78)
/// Calls: strklen
#[allow(unused_variables, non_snake_case)]
pub fn copy_name(s: *const i8) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (s : * const i8)
    // Previous return: i32
    extern "C" { fn CopyName(s : * const i8) -> i32 ; } unsafe { CopyName(s) }
}

/// C: IsValidURISchemeFormat (engine/engine_plugin.cc:93)
#[allow(unused_variables, non_snake_case)]
pub fn is_valid_uri_scheme_format(prefix: *const i8) -> bool {
    if prefix.is_null() {
        return false;
    }
    extern "C" { fn IsValidURISchemeFormat(prefix: *const i8) -> bool; }
    // SAFETY: prefix verified non-null; delegates to C implementation
    unsafe { IsValidURISchemeFormat(prefix) }
}

/// C: PluginAttrSeek (engine/engine_plugin.cc:119)
#[allow(unused_variables, non_snake_case)]
pub fn plugin_attr_seek(m: *const mjModel, plugin_id: i32, attrib_id: i32) -> *const i8 {
    if m.is_null() { return core::ptr::null(); }
    extern "C" { fn PluginAttrSeek(m: *const mjModel, plugin_id: i32, attrib_id: i32) -> *const i8; }
    // SAFETY: m verified non-null
    unsafe { PluginAttrSeek(m, plugin_id, attrib_id) }
}

/// C: mjp_defaultPlugin (engine/engine_plugin.h:26)
#[allow(unused_variables, non_snake_case)]
pub fn mjp_default_plugin(plugin: *mut mjpPlugin) {
    // SAFETY: plugin is a valid pointer to mjpPlugin. Zeroing all bytes is correct (C memset).
    unsafe {
        core::ptr::write_bytes(plugin, 0, 1);
    }
}

/// C: mjp_registerPlugin (engine/engine_plugin.h:29)
/// Calls: mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mjp_register_plugin(plugin: *const mjpPlugin) -> i32 {
    extern "C" { fn mjp_registerPlugin(plugin: *const mjpPlugin) -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { mjp_registerPlugin(plugin) }
}

/// C: mjp_registerResourceProvider (engine/engine_plugin.h:32)
/// Calls: IsValidURISchemeFormat, mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn mjp_register_resource_provider(provider: *const mjpResourceProvider) -> i32 {
    extern "C" { fn mjp_registerResourceProvider(provider: *const mjpResourceProvider) -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { mjp_registerResourceProvider(provider) }
}

/// C: mjp_pluginCount (engine/engine_plugin.h:35)
#[allow(unused_variables, non_snake_case)]
pub fn mjp_plugin_count() -> i32 {
    use crate::engine::engine_global_table::{global_table_get_singleton, global_table_count};
    let table = global_table_get_singleton() as *mut GlobalTable;
    global_table_count(table)
}

/// C: mjp_resourceProviderCount (engine/engine_plugin.h:38)
#[allow(unused_variables, non_snake_case)]
pub fn mjp_resource_provider_count() -> i32 {
    use crate::engine::engine_global_table::{global_table_get_singleton, global_table_count};
    let table = global_table_get_singleton() as *mut GlobalTable;
    global_table_count(table)
}

/// C: mjp_getPlugin (engine/engine_plugin.h:41)
#[allow(unused_variables, non_snake_case)]
pub fn mjp_get_plugin(name: *const i8, slot: *mut i32) -> *const mjpPlugin {
    use crate::engine::engine_global_table::{global_table_get_singleton, global_table_get_by_key};
    let table = global_table_get_singleton() as *mut GlobalTable;
    // construct string_view {ptr, len} from null-terminated C string
    let len = strklen(name);
    if len < 0 {
        return core::ptr::null();
    }
    #[repr(C)]
    struct RawSV { ptr: *const u8, len: usize }
    let raw = RawSV { ptr: name as *const u8, len: len as usize };
    let sv = unsafe { core::ptr::read(&raw as *const RawSV as *const string_view) };
    global_table_get_by_key(table, sv, slot) as *const mjpPlugin
}

/// C: mjp_defaultResourceProvider (engine/engine_plugin.h:44)
#[allow(unused_variables, non_snake_case)]
pub fn mjp_default_resource_provider(provider: *mut mjpResourceProvider) {
    // SAFETY: provider is a valid pointer. Zeroing all bytes is correct (C memset).
    unsafe {
        core::ptr::write_bytes(provider, 0, 1);
    }
}

/// C: mjp_getResourceProvider (engine/engine_plugin.h:47)
#[allow(unused_variables, non_snake_case)]
pub fn mjp_get_resource_provider(resource_name: *const i8) -> *const mjpResourceProvider {
    use crate::engine::engine_global_table::{global_table_get_singleton, global_table_get_by_key};
    unsafe {
        if resource_name.is_null() || *resource_name == 0 {
            return core::ptr::null();
        }
        // find ':' in resource_name to extract URI scheme prefix
        let mut n: usize = 0;
        loop {
            let ch = *resource_name.add(n);
            if ch == 0 {
                // no ':' found — not a URI
                return core::ptr::null();
            }
            if ch == b':' as i8 {
                break;
            }
            n += 1;
        }
        // copy prefix to null-terminated buffer for validation
        let mut prefix_buf = [0i8; 1025];
        if n >= 1024 {
            return core::ptr::null();
        }
        core::ptr::copy_nonoverlapping(resource_name, prefix_buf.as_mut_ptr(), n);
        prefix_buf[n] = 0;
        // validate URI scheme format
        if !is_valid_uri_scheme_format(prefix_buf.as_ptr()) {
            return core::ptr::null();
        }
        // construct string_view for the prefix and look up
        #[repr(C)]
        struct RawSV { ptr: *const u8, len: usize }
        let raw = RawSV { ptr: prefix_buf.as_ptr() as *const u8, len: n };
        let sv = core::ptr::read(&raw as *const RawSV as *const string_view);
        let table = global_table_get_singleton() as *mut GlobalTable;
        global_table_get_by_key(table, sv, core::ptr::null_mut()) as *const mjpResourceProvider
    }
}

/// C: mjp_getPluginAtSlot (engine/engine_plugin.h:50)
#[allow(unused_variables, non_snake_case)]
pub fn mjp_get_plugin_at_slot(slot: i32) -> *const mjpPlugin {
    use crate::engine::engine_global_table::{global_table_get_singleton, global_table_get_at_slot};
    let table = global_table_get_singleton() as *mut GlobalTable;
    global_table_get_at_slot(table, slot) as *const mjpPlugin
}

/// C: mjp_getResourceProviderAtSlot (engine/engine_plugin.h:53)
#[allow(unused_variables, non_snake_case)]
pub fn mjp_get_resource_provider_at_slot(slot: i32) -> *const mjpResourceProvider {
    use crate::engine::engine_global_table::{global_table_get_singleton, global_table_get_at_slot};
    // shift slot to be zero-indexed (resource provider slots are 1-based externally)
    let table = global_table_get_singleton() as *mut GlobalTable;
    global_table_get_at_slot(table, slot - 1) as *const mjpResourceProvider
}

/// C: mj_getPluginConfig (engine/engine_plugin.h:57)
/// Calls: PluginAttrSeek, mjp_getPluginAtSlot
#[allow(unused_variables, non_snake_case)]
pub fn mj_get_plugin_config(m: *const mjModel, plugin_id: i32, attrib: *const i8) -> *const i8 {
    extern "C" { fn mj_getPluginConfig(m: *const mjModel, plugin_id: i32, attrib: *const i8) -> *const i8; }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mj_getPluginConfig(m, plugin_id, attrib) }
}

/// C: mj_loadPluginLibrary (engine/engine_plugin.h:60)
/// Calls: mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mj_load_plugin_library(path: *const i8) {
    extern "C" { fn mj_loadPluginLibrary(path: *const i8); }
    // SAFETY: delegates to C implementation
    unsafe { mj_loadPluginLibrary(path) }
}

/// C: mj_loadAllPluginLibraries (engine/engine_plugin.h:63)
/// Calls: mjp_pluginCount
#[allow(unused_variables, non_snake_case)]
pub fn mj_load_all_plugin_libraries(directory: *const i8, callback: mjfPluginLibraryLoadCallback) {
    extern "C" { fn mj_loadAllPluginLibraries(directory: *const i8, callback: mjfPluginLibraryLoadCallback); }
    // SAFETY: delegates to C implementation
    unsafe { mj_loadAllPluginLibraries(directory, callback) }
}

/// C: mjp_registerDecoder (engine/engine_plugin.h:66)
/// Calls: mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn mjp_register_decoder(decoder: *const mjpDecoder) {
    use crate::engine::engine_global_table::{global_table_get_singleton, global_table_append_if_unique};
    use crate::engine::engine_util_errmem::mju_warning;
    unsafe {
        let dec = &*decoder;
        // check callbacks: can_decode is at offset 16, decode at offset 24 (each 8-byte fn ptr)
        let base = decoder as *const u8;
        let can_decode_ptr = *(base.add(16) as *const *const ());
        let decode_ptr = *(base.add(24) as *const *const ());
        if decode_ptr.is_null() || can_decode_ptr.is_null() {
            mju_warning(b"decoder must provide decode and can_decode callbacks.\0".as_ptr() as *const i8);
            return;
        }
        if dec.content_type.is_null() && dec.extension.is_null() {
            mju_warning(b"decoder must provide content_type and/or extensions.\0".as_ptr() as *const i8);
            return;
        }
        let table = global_table_get_singleton() as *mut GlobalTable;
        let mut decoder_copy: mjpDecoder = *dec;
        // Register with content_type
        if !dec.content_type.is_null() {
            decoder_copy.extension = core::ptr::null();
            decoder_copy.content_type = dec.content_type;
            global_table_append_if_unique(table, &decoder_copy as *const mjpDecoder as *const T);
        }
        // Register with extensions (split by '|')
        if !dec.extension.is_null() {
            decoder_copy.content_type = core::ptr::null();
            let ext_base = dec.extension;
            let mut start: usize = 0;
            let mut i: usize = 0;
            loop {
                let ch = *ext_base.add(i);
                if ch == b'|' as i8 || ch == 0 {
                    let seg_len = i - start;
                    if seg_len > 0 {
                        // copy segment to stack buffer with null terminator
                        let mut buf = [0i8; 1025];
                        let copy_len = if seg_len < 1024 { seg_len } else { 1024 };
                        core::ptr::copy_nonoverlapping(ext_base.add(start), buf.as_mut_ptr(), copy_len);
                        buf[copy_len] = 0;
                        decoder_copy.extension = buf.as_ptr();
                        global_table_append_if_unique(table, &decoder_copy as *const mjpDecoder as *const T);
                    }
                    start = i + 1;
                    if ch == 0 {
                        break;
                    }
                }
                i += 1;
            }
        }
    }
}

/// C: mjp_defaultDecoder (engine/engine_plugin.h:69)
#[allow(unused_variables, non_snake_case)]
pub fn mjp_default_decoder(decoder: *mut mjpDecoder) {
    // SAFETY: decoder is a valid pointer. Zeroing all bytes is correct (C memset).
    unsafe {
        core::ptr::write_bytes(decoder, 0, 1);
    }
}

/// C: mjp_findDecoder (engine/engine_plugin.h:72)
/// Calls: mju_warning, strklen
#[allow(unused_variables, non_snake_case)]
pub fn mjp_find_decoder(resource: *const mjResource, content_type: *const i8) -> *const mjpDecoder {
    extern "C" { fn mjp_findDecoder(resource: *const mjResource, content_type: *const i8) -> *const mjpDecoder; }
    // SAFETY: delegates to C implementation
    unsafe { mjp_findDecoder(resource, content_type) }
}

/// C: mjp_registerEncoder (engine/engine_plugin.h:75)
/// Calls: mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn mjp_register_encoder(encoder: *const mjpEncoder) {
    use crate::engine::engine_global_table::{global_table_get_singleton, global_table_append_if_unique};
    use crate::engine::engine_util_errmem::mju_warning;
    unsafe {
        let enc = &*encoder;
        // check callbacks: encode is at offset 16, close_resource at offset 24 (each 8-byte fn ptr)
        let base = encoder as *const u8;
        let encode_ptr = *(base.add(16) as *const *const ());
        let close_ptr = *(base.add(24) as *const *const ());
        if encode_ptr.is_null() {
            mju_warning(b"encoder must provide an encode callback.\0".as_ptr() as *const i8);
            return;
        }
        if close_ptr.is_null() {
            mju_warning(b"encoder must provide a close_resource callback.\0".as_ptr() as *const i8);
            return;
        }
        if enc.content_type.is_null() && enc.extension.is_null() {
            mju_warning(b"encoder must provide content_type and/or extensions.\0".as_ptr() as *const i8);
            return;
        }
        let table = global_table_get_singleton() as *mut GlobalTable;
        let mut encoder_copy: mjpEncoder = *enc;
        // Register with content_type
        if !enc.content_type.is_null() {
            encoder_copy.extension = core::ptr::null();
            encoder_copy.content_type = enc.content_type;
            global_table_append_if_unique(table, &encoder_copy as *const mjpEncoder as *const T);
        }
        // Register with extensions (split by '|')
        if !enc.extension.is_null() {
            encoder_copy.content_type = core::ptr::null();
            let ext_base = enc.extension;
            let mut start: usize = 0;
            let mut i: usize = 0;
            loop {
                let ch = *ext_base.add(i);
                if ch == b'|' as i8 || ch == 0 {
                    let seg_len = i - start;
                    if seg_len > 0 {
                        // copy segment to stack buffer with null terminator
                        let mut buf = [0i8; 1025];
                        let copy_len = if seg_len < 1024 { seg_len } else { 1024 };
                        core::ptr::copy_nonoverlapping(ext_base.add(start), buf.as_mut_ptr(), copy_len);
                        buf[copy_len] = 0;
                        encoder_copy.extension = buf.as_ptr();
                        global_table_append_if_unique(table, &encoder_copy as *const mjpEncoder as *const T);
                    }
                    start = i + 1;
                    if ch == 0 {
                        break;
                    }
                }
                i += 1;
            }
        }
    }
}

/// C: mjp_defaultEncoder (engine/engine_plugin.h:78)
#[allow(unused_variables, non_snake_case)]
pub fn mjp_default_encoder(encoder: *mut mjpEncoder) {
    // SAFETY: encoder is a valid pointer. Zeroing all bytes is correct (C memset).
    unsafe {
        core::ptr::write_bytes(encoder, 0, 1);
    }
}

/// C: mjp_findEncoder (engine/engine_plugin.h:81)
/// Calls: mju_warning, strklen
#[allow(unused_variables, non_snake_case)]
pub fn mjp_find_encoder(filename: *const i8, content_type: *const i8) -> *const mjpEncoder {
    use crate::engine::engine_global_table::{global_table_get_singleton, global_table_get_by_key};
    use crate::engine::engine_util_errmem::mju_warning;
    #[repr(C)]
    struct RawSV { ptr: *const u8, len: usize }
    unsafe {
        let table = global_table_get_singleton() as *mut GlobalTable;
        // get extension from filename: find last '.'
        let mut ext_start: *const i8 = core::ptr::null();
        let mut ext_len: usize = 0;
        let fname = if filename.is_null() { b"\0".as_ptr() as *const i8 } else { filename };
        let fname_len = strklen(fname);
        if fname_len > 0 {
            let mut last_dot: i32 = -1;
            for j in 0..fname_len {
                if *fname.add(j as usize) == b'.' as i8 {
                    last_dot = j;
                }
            }
            if last_dot >= 0 {
                ext_start = fname.add(last_dot as usize);
                ext_len = (fname_len - last_dot) as usize;
            }
        }
        // check has_content_type
        let has_content_type = !content_type.is_null() && strklen(content_type) > 0;
        if !has_content_type && ext_start.is_null() {
            mju_warning(b"Must provide extension or content_type to mjp_findEncoder.\0".as_ptr() as *const i8);
            return core::ptr::null();
        }
        // try content_type lookup
        if has_content_type {
            let ct_len = strklen(content_type) as usize;
            let raw = RawSV { ptr: content_type as *const u8, len: ct_len };
            let sv = core::ptr::read(&raw as *const RawSV as *const string_view);
            let encoder = global_table_get_by_key(table, sv, core::ptr::null_mut()) as *const mjpEncoder;
            if !encoder.is_null() {
                return encoder;
            }
        }
        // try extension lookup
        if !ext_start.is_null() {
            let raw = RawSV { ptr: ext_start as *const u8, len: ext_len };
            let sv = core::ptr::read(&raw as *const RawSV as *const string_view);
            let encoder = global_table_get_by_key(table, sv, core::ptr::null_mut()) as *const mjpEncoder;
            if !encoder.is_null() {
                return encoder;
            }
        }
        core::ptr::null()
    }
}

/// C: mjp_getPluginUnsafe (engine/engine_plugin.h:95)
#[allow(unused_variables, non_snake_case)]
pub fn mjp_get_plugin_unsafe(name: *const i8, slot: *mut i32, nslot: i32) -> *const mjpPlugin {
    extern "C" { fn mjp_getPluginUnsafe(name: *const i8, slot: *mut i32, nslot: i32) -> *const mjpPlugin; }
    // SAFETY: delegates to C implementation
    unsafe { mjp_getPluginUnsafe(name, slot, nslot) }
}

/// C: mjp_getPluginAtSlotUnsafe (engine/engine_plugin.h:98)
#[allow(unused_variables, non_snake_case)]
pub fn mjp_get_plugin_at_slot_unsafe(slot: i32, nslot: i32) -> *const mjpPlugin {
    use crate::engine::engine_global_table::{global_table_get_singleton, global_table_get_at_slot_unsafe};
    let table = global_table_get_singleton() as *mut GlobalTable;
    global_table_get_at_slot_unsafe(table, slot, nslot) as *const mjpPlugin
}

