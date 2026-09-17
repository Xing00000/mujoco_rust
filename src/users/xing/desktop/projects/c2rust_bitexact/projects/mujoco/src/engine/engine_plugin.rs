//! Port of: Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_plugin.cc
//! IR hash: 1fa44043fe383ce9
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: strklen (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_plugin.cc:58)
#[allow(unused_variables, non_snake_case)]
pub fn strklen(s: *const i8) -> i32 {
    todo!() // cxx-internal:engine_plugin.cc.o:__ZN12_GLOBAL__N_17strklenEPKc
}

/// C: getext (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_plugin.cc:68)
#[allow(unused_variables, non_snake_case)]
pub fn getext(filename: std__string_view) -> std__string {
    todo!() // cxx-internal:engine_plugin.cc.o:__ZN12_GLOBAL__N_16getextENSt3__117basic_string_viewIcNS0_11char_traitsIcEEEE
}

/// C: CopyName (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_plugin.cc:78)
/// Calls: cxx-internal:engine_plugin.cc.o:__ZN12_GLOBAL__N_17strklenEPKc
#[allow(unused_variables, non_snake_case)]
pub fn copy_name(s: *const i8) -> *const () {
    todo!() // cxx-internal:engine_plugin.cc.o:__ZN12_GLOBAL__N_18CopyNameEPKc
}

/// C: IsValidURISchemeFormat (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_plugin.cc:93)
#[allow(unused_variables, non_snake_case)]
pub fn is_valid_uri_scheme_format(prefix: *const i8) -> bool {
    todo!() // cxx-internal:engine_plugin.cc.o:__ZN12_GLOBAL__N_122IsValidURISchemeFormatEPKc
}

/// C: PluginAttrSeek (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_plugin.cc:119)
#[allow(unused_variables, non_snake_case)]
pub fn plugin_attr_seek(m: *const mjModel, plugin_id: i32, attrib_id: i32) -> *const i8 {
    todo!() // cxx-internal:engine_plugin.cc.o:__ZN12_GLOBAL__N_114PluginAttrSeekEPK8mjModel_ii
}

/// C: GlobalTable::HumanReadableTypeName (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_plugin.cc:132)
#[allow(unused_variables, non_snake_case)]
pub fn global_table_human_readable_type_name__a859f6e6c9ea296d() -> *const i8 {
    todo!() // cxx:__ZN6mujoco11GlobalTableI10mjpPlugin_E21HumanReadableTypeNameEv
}

/// C: GlobalTable::ObjectKey (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_plugin.cc:137)
/// Calls: cxx-internal:engine_plugin.cc.o:__ZN12_GLOBAL__N_17strklenEPKc
#[allow(unused_variables, non_snake_case)]
pub fn global_table_object_key__127feace325390fb(self_ptr: *mut mujoco__GlobalTable, plugin: *const mjpPlugin) -> std__string_view {
    todo!() // cxx:__ZN6mujoco11GlobalTableI10mjpPlugin_E9ObjectKeyERKS1_
}

/// C: GlobalTable::ObjectEqual (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_plugin.cc:143)
#[allow(unused_variables, non_snake_case)]
pub fn global_table_object_equal__f677bea122217dfb(self_ptr: *mut mujoco__GlobalTable, plugin1: *const mjpPlugin, plugin2: *const mjpPlugin) -> bool {
    todo!() // cxx:__ZN6mujoco11GlobalTableI10mjpPlugin_E11ObjectEqualERKS1_S4_
}

/// C: GlobalTable::CopyObject (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_plugin.cc:182)
/// Calls: cxx-internal:engine_plugin.cc.o:__ZN12_GLOBAL__N_17strklenEPKc, cxx-internal:engine_plugin.cc.o:__ZN12_GLOBAL__N_18CopyNameEPKc
#[allow(unused_variables, non_snake_case)]
pub fn global_table_copy_object__367a751c9c18cddc(self_ptr: *mut mujoco__GlobalTable, dst: *mut mjpPlugin, src: *const mjpPlugin, err: *const ()) -> bool {
    todo!() // cxx:__ZN6mujoco11GlobalTableI10mjpPlugin_E10CopyObjectERS1_RKS1_RA512_c
}

/// C: GlobalTable::HumanReadableTypeName (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_plugin.cc:240)
#[allow(unused_variables, non_snake_case)]
pub fn global_table_human_readable_type_name__8f17fe67b5f2dfc2() -> *const i8 {
    todo!() // cxx:__ZN6mujoco11GlobalTableI19mjpResourceProviderE21HumanReadableTypeNameEv
}

/// C: GlobalTable::ObjectKey (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_plugin.cc:245)
/// Calls: cxx-internal:engine_plugin.cc.o:__ZN12_GLOBAL__N_17strklenEPKc
#[allow(unused_variables, non_snake_case)]
pub fn global_table_object_key__fa5e93cfece610d0(arg0: *const mjpResourceProvider) -> std__string_view {
    todo!() // cxx:__ZN6mujoco11GlobalTableI19mjpResourceProviderE9ObjectKeyERKS1_
}

/// C: GlobalTable::ObjectEqual (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_plugin.cc:251)
/// Calls: cxx-internal:engine_plugin.cc.o:__ZN6mujocoL20CaseInsensitiveEqualENSt3__117basic_string_viewIcNS0_11char_traitsIcEEEES4_
#[allow(unused_variables, non_snake_case)]
pub fn global_table_object_equal__c39aa00c5797d7c9(arg0: *const mjpResourceProvider, arg1: *const mjpResourceProvider) -> bool {
    todo!() // cxx:__ZN6mujoco11GlobalTableI19mjpResourceProviderE11ObjectEqualERKS1_S4_
}

/// C: GlobalTable::CopyObject (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_plugin.cc:261)
/// Calls: cxx-internal:engine_plugin.cc.o:__ZN12_GLOBAL__N_17strklenEPKc, cxx-internal:engine_plugin.cc.o:__ZN12_GLOBAL__N_18CopyNameEPKc
#[allow(unused_variables, non_snake_case)]
pub fn global_table_copy_object__4869ae6e7c1f1182(dst: *mut mjpResourceProvider, src: *const mjpResourceProvider, err: *mut mujoco__GlobalTable__ErrorMessage) -> bool {
    todo!() // cxx:__ZN6mujoco11GlobalTableI19mjpResourceProviderE10CopyObjectERS1_RKS1_RA512_c
}

/// C: GlobalTable::HumanReadableTypeName (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_plugin.cc:280)
#[allow(unused_variables, non_snake_case)]
pub fn global_table_human_readable_type_name__4421c26f5dc783a5() -> *const i8 {
    todo!() // cxx:__ZN6mujoco11GlobalTableI10mjpDecoderE21HumanReadableTypeNameEv
}

/// C: GlobalTable::ObjectKey (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_plugin.cc:285)
/// Calls: cxx-internal:engine_plugin.cc.o:__ZN12_GLOBAL__N_17strklenEPKc
#[allow(unused_variables, non_snake_case)]
pub fn global_table_object_key__a7cddc2cde183f63(arg0: *const mjpDecoder) -> std__string_view {
    todo!() // cxx:__ZN6mujoco11GlobalTableI10mjpDecoderE9ObjectKeyERKS1_
}

/// C: GlobalTable::ObjectEqual (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_plugin.cc:299)
/// Calls: cxx-internal:engine_plugin.cc.o:__ZN6mujocoL20CaseInsensitiveEqualENSt3__117basic_string_viewIcNS0_11char_traitsIcEEEES4_
#[allow(unused_variables, non_snake_case)]
pub fn global_table_object_equal__38d25435ffc097ab(arg0: *const mjpDecoder, arg1: *const mjpDecoder) -> bool {
    todo!() // cxx:__ZN6mujoco11GlobalTableI10mjpDecoderE11ObjectEqualERKS1_S4_
}

/// C: GlobalTable::CopyObject (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_plugin.cc:321)
/// Calls: cxx-internal:engine_plugin.cc.o:__ZN12_GLOBAL__N_17strklenEPKc, cxx-internal:engine_plugin.cc.o:__ZN12_GLOBAL__N_18CopyNameEPKc
#[allow(unused_variables, non_snake_case)]
pub fn global_table_copy_object__c806fb4e6aed02dd(dst: *mut mjpDecoder, src: *const mjpDecoder, err: *mut mujoco__GlobalTable__ErrorMessage) -> bool {
    todo!() // cxx:__ZN6mujoco11GlobalTableI10mjpDecoderE10CopyObjectERS1_RKS1_RA512_c
}

/// C: GlobalTable::HumanReadableTypeName (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_plugin.cc:361)
#[allow(unused_variables, non_snake_case)]
pub fn global_table_human_readable_type_name__fb2c21b4c1476efd() -> *const i8 {
    todo!() // cxx:__ZN6mujoco11GlobalTableI10mjpEncoderE21HumanReadableTypeNameEv
}

/// C: GlobalTable::ObjectKey (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_plugin.cc:366)
/// Calls: cxx-internal:engine_plugin.cc.o:__ZN12_GLOBAL__N_17strklenEPKc
#[allow(unused_variables, non_snake_case)]
pub fn global_table_object_key__4f3261c8d61fa8fe(arg0: *const mjpEncoder) -> std__string_view {
    todo!() // cxx:__ZN6mujoco11GlobalTableI10mjpEncoderE9ObjectKeyERKS1_
}

/// C: GlobalTable::ObjectEqual (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_plugin.cc:376)
/// Calls: cxx-internal:engine_plugin.cc.o:__ZN6mujocoL20CaseInsensitiveEqualENSt3__117basic_string_viewIcNS0_11char_traitsIcEEEES4_
#[allow(unused_variables, non_snake_case)]
pub fn global_table_object_equal__868003341f6e52d0(arg0: *const mjpEncoder, arg1: *const mjpEncoder) -> bool {
    todo!() // cxx:__ZN6mujoco11GlobalTableI10mjpEncoderE11ObjectEqualERKS1_S4_
}

/// C: GlobalTable::CopyObject (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_plugin.cc:397)
/// Calls: cxx-internal:engine_plugin.cc.o:__ZN12_GLOBAL__N_17strklenEPKc, cxx-internal:engine_plugin.cc.o:__ZN12_GLOBAL__N_18CopyNameEPKc
#[allow(unused_variables, non_snake_case)]
pub fn global_table_copy_object__c8716337fc4bfcf2(dst: *mut mjpEncoder, src: *const mjpEncoder, err: *mut mujoco__GlobalTable__ErrorMessage) -> bool {
    todo!() // cxx:__ZN6mujoco11GlobalTableI10mjpEncoderE10CopyObjectERS1_RKS1_RA512_c
}

/// C: mjp_defaultPlugin (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_plugin.h:26)
#[allow(unused_variables, non_snake_case)]
pub fn mjp_default_plugin(plugin: *mut mjpPlugin) {
    todo!() // cxx:_mjp_defaultPlugin
}

/// C: mjp_registerPlugin (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_plugin.h:29)
/// Calls: cxx:_mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mjp_register_plugin(plugin: *const mjpPlugin) -> i32 {
    todo!() // cxx:_mjp_registerPlugin
}

/// C: mjp_registerResourceProvider (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_plugin.h:32)
/// Calls: cxx-internal:engine_plugin.cc.o:__ZN12_GLOBAL__N_122IsValidURISchemeFormatEPKc, cxx:_mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn mjp_register_resource_provider(provider: *const mjpResourceProvider) -> i32 {
    todo!() // cxx:_mjp_registerResourceProvider
}

/// C: mjp_pluginCount (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_plugin.h:35)
#[allow(unused_variables, non_snake_case)]
pub fn mjp_plugin_count() -> i32 {
    todo!() // cxx:_mjp_pluginCount
}

/// C: mjp_resourceProviderCount (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_plugin.h:38)
#[allow(unused_variables, non_snake_case)]
pub fn mjp_resource_provider_count() -> i32 {
    todo!() // cxx:_mjp_resourceProviderCount
}

/// C: mjp_getPlugin (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_plugin.h:41)
#[allow(unused_variables, non_snake_case)]
pub fn mjp_get_plugin(name: *const i8, slot: *mut i32) -> *const mjpPlugin {
    todo!() // cxx:_mjp_getPlugin
}

/// C: mjp_defaultResourceProvider (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_plugin.h:44)
#[allow(unused_variables, non_snake_case)]
pub fn mjp_default_resource_provider(provider: *mut mjpResourceProvider) {
    todo!() // cxx:_mjp_defaultResourceProvider
}

/// C: mjp_getResourceProvider (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_plugin.h:47)
/// Calls: cxx-internal:engine_plugin.cc.o:__ZN12_GLOBAL__N_122IsValidURISchemeFormatEPKc
#[allow(unused_variables, non_snake_case)]
pub fn mjp_get_resource_provider(resource_name: *const i8) -> *const mjpResourceProvider {
    todo!() // cxx:_mjp_getResourceProvider
}

/// C: mjp_getPluginAtSlot (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_plugin.h:50)
#[allow(unused_variables, non_snake_case)]
pub fn mjp_get_plugin_at_slot(slot: i32) -> *const mjpPlugin {
    todo!() // cxx:_mjp_getPluginAtSlot
}

/// C: mjp_getResourceProviderAtSlot (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_plugin.h:53)
#[allow(unused_variables, non_snake_case)]
pub fn mjp_get_resource_provider_at_slot(slot: i32) -> *const mjpResourceProvider {
    todo!() // cxx:_mjp_getResourceProviderAtSlot
}

/// C: mj_getPluginConfig (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_plugin.h:57)
/// Calls: cxx-internal:engine_plugin.cc.o:__ZN12_GLOBAL__N_114PluginAttrSeekEPK8mjModel_ii, cxx:_mjp_getPluginAtSlot
#[allow(unused_variables, non_snake_case)]
pub fn mj_get_plugin_config(m: *const mjModel, plugin_id: i32, attrib: *const i8) -> *const i8 {
    todo!() // cxx:_mj_getPluginConfig
}

/// C: mj_loadPluginLibrary (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_plugin.h:60)
/// Calls: cxx:_mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mj_load_plugin_library(path: *const i8) {
    todo!() // cxx:_mj_loadPluginLibrary
}

/// C: mj_loadAllPluginLibraries (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_plugin.h:63)
/// Calls: cxx:_mj_loadPluginLibrary, cxx:_mjp_pluginCount
#[allow(unused_variables, non_snake_case)]
pub fn mj_load_all_plugin_libraries(directory: *const i8, callback: mjfPluginLibraryLoadCallback) {
    todo!() // cxx:_mj_loadAllPluginLibraries
}

/// C: mjp_registerDecoder (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_plugin.h:66)
/// Calls: cxx:_mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn mjp_register_decoder(decoder: *const mjpDecoder) {
    todo!() // cxx:_mjp_registerDecoder
}

/// C: mjp_defaultDecoder (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_plugin.h:69)
#[allow(unused_variables, non_snake_case)]
pub fn mjp_default_decoder(decoder: *mut mjpDecoder) {
    todo!() // cxx:_mjp_defaultDecoder
}

/// C: mjp_findDecoder (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_plugin.h:72)
/// Calls: cxx-internal:engine_plugin.cc.o:__ZN12_GLOBAL__N_16getextENSt3__117basic_string_viewIcNS0_11char_traitsIcEEEE, cxx-internal:engine_plugin.cc.o:__ZN12_GLOBAL__N_17strklenEPKc, cxx:_mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn mjp_find_decoder(resource: *const mjResource, content_type: *const i8) -> *const mjpDecoder {
    todo!() // cxx:_mjp_findDecoder
}

/// C: mjp_registerEncoder (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_plugin.h:75)
/// Calls: cxx:_mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn mjp_register_encoder(encoder: *const mjpEncoder) {
    todo!() // cxx:_mjp_registerEncoder
}

/// C: mjp_defaultEncoder (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_plugin.h:78)
#[allow(unused_variables, non_snake_case)]
pub fn mjp_default_encoder(encoder: *mut mjpEncoder) {
    todo!() // cxx:_mjp_defaultEncoder
}

/// C: mjp_findEncoder (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_plugin.h:81)
/// Calls: cxx-internal:engine_plugin.cc.o:__ZN12_GLOBAL__N_16getextENSt3__117basic_string_viewIcNS0_11char_traitsIcEEEE, cxx-internal:engine_plugin.cc.o:__ZN12_GLOBAL__N_17strklenEPKc, cxx:_mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn mjp_find_encoder(filename: *const i8, content_type: *const i8) -> *const mjpEncoder {
    todo!() // cxx:_mjp_findEncoder
}

/// C: mjp_getPluginUnsafe (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_plugin.h:95)
#[allow(unused_variables, non_snake_case)]
pub fn mjp_get_plugin_unsafe(name: *const i8, slot: *mut i32, nslot: i32) -> *const mjpPlugin {
    todo!() // cxx:_mjp_getPluginUnsafe
}

/// C: mjp_getPluginAtSlotUnsafe (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_plugin.h:98)
#[allow(unused_variables, non_snake_case)]
pub fn mjp_get_plugin_at_slot_unsafe(slot: i32, nslot: i32) -> *const mjpPlugin {
    todo!() // cxx:_mjp_getPluginAtSlotUnsafe
}

