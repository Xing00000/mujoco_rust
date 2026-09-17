//! Port of: engine/engine_plugin.cc
//! IR hash: fd5288ad7242f001
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mjp_defaultPlugin (engine/engine_plugin.cc:46)
#[allow(unused_variables, non_snake_case)]
pub fn mjp_default_plugin(plugin: *mut mjpPlugin) {
    todo!() // cxx:_mjp_defaultPlugin
}

/// C: strklen (engine/engine_plugin.cc:57)
#[allow(unused_variables, non_snake_case)]
pub fn strklen(s: *const i8) -> i32 {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_plugin.cc:__ZN12_GLOBAL__N_17strklenEPKc
}

/// C: getext (engine/engine_plugin.cc:67)
#[allow(unused_variables, non_snake_case)]
pub fn getext(filename: std__string_view) -> std__string {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_plugin.cc:__ZN12_GLOBAL__N_16getextENSt3__117basic_string_viewIcNS0_11char_traitsIcEEEE
}

/// C: CopyName (engine/engine_plugin.cc:77)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_plugin.cc:__ZN12_GLOBAL__N_17strklenEPKc
#[allow(unused_variables, non_snake_case)]
pub fn copy_name(s: *const i8) -> *const () {
    todo!() // cxx-internal:engine_plugin.cc.o:__ZN12_GLOBAL__N_18CopyNameEPKc
}

/// C: IsValidURISchemeFormat (engine/engine_plugin.cc:92)
#[allow(unused_variables, non_snake_case)]
pub fn is_valid_uri_scheme_format(prefix: *const i8) -> bool {
    todo!() // cxx-internal:engine_plugin.cc.o:__ZN12_GLOBAL__N_122IsValidURISchemeFormatEPKc
}

/// C: PluginAttrSeek (engine/engine_plugin.cc:118)
#[allow(unused_variables, non_snake_case)]
pub fn plugin_attr_seek(m: *const mjModel, plugin_id: i32, attrib_id: i32) -> *const i8 {
    todo!() // cxx-internal:engine_plugin.cc.o:__ZN12_GLOBAL__N_114PluginAttrSeekEPK8mjModel_ii
}

/// C: ForEachPipeSeparated (engine/engine_plugin.cc:130)
#[allow(unused_variables, non_snake_case)]
pub fn for_each_pipe_separated__99a14c086091bed0(str: std__string_view, r#fn: *mut *mut lambda_at__Users_xing_Desktop_projects_c2rust_bitexact_projects_mujoco_src_engine_engine_plugin_cc_608_46) {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_plugin.cc:__ZN12_GLOBAL__N_120ForEachPipeSeparatedIZ19mjp_registerDecoderE3$_0EEvNSt3__117basic_string_viewIcNS2_11char_traitsIcEEEEOT_
}

/// C: ForEachPipeSeparated (engine/engine_plugin.cc:130)
#[allow(unused_variables, non_snake_case)]
pub fn for_each_pipe_separated__aad592093dd38bc9(str: std__string_view, r#fn: *mut *mut lambda_at__Users_xing_Desktop_projects_c2rust_bitexact_projects_mujoco_src_engine_engine_plugin_cc_670_46) {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_plugin.cc:__ZN12_GLOBAL__N_120ForEachPipeSeparatedIZ19mjp_registerEncoderE3$_0EEvNSt3__117basic_string_viewIcNS2_11char_traitsIcEEEEOT_
}

/// C: ResourceProviderKey (engine/engine_plugin.cc:145)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_plugin.cc:__ZN12_GLOBAL__N_17strklenEPKc
#[allow(unused_variables, non_snake_case)]
pub fn resource_provider_key(provider: *const mjpResourceProvider) -> std__string_view {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_plugin.cc:__ZN12_GLOBAL__N_119ResourceProviderKeyERK19mjpResourceProvider
}

/// C: ResourceProviderEqual (engine/engine_plugin.cc:149)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_global_table.h:__ZN6mujocoL20CaseInsensitiveEqualENSt3__117basic_string_viewIcNS0_11char_traitsIcEEEES4_
#[allow(unused_variables, non_snake_case)]
pub fn resource_provider_equal(p1: *const mjpResourceProvider, p2: *const mjpResourceProvider) -> bool {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_plugin.cc:__ZN12_GLOBAL__N_121ResourceProviderEqualERK19mjpResourceProviderS2_
}

/// C: ResourceProviderCopy (engine/engine_plugin.cc:159)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_plugin.cc:__ZN12_GLOBAL__N_17strklenEPKc, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_plugin.cc:__ZN12_GLOBAL__N_18CopyNameEPKc
#[allow(unused_variables, non_snake_case)]
pub fn resource_provider_copy(dst: *mut mjpResourceProvider, src: *const mjpResourceProvider, err: [char; 512]) -> bool {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_plugin.cc:__ZN12_GLOBAL__N_120ResourceProviderCopyER19mjpResourceProviderRKS0_RA512_c
}

/// C: GlobalTable::HumanReadableTypeName (engine/engine_plugin.cc:179)
#[allow(unused_variables, non_snake_case)]
pub fn global_table_human_readable_type_name__2c8ef9482c887844() -> *const i8 {
    todo!() // cxx:__ZN6mujoco11GlobalTableI10mjpPlugin_vE21HumanReadableTypeNameEv
}

/// C: GlobalTable::ObjectKey (engine/engine_plugin.cc:184)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_plugin.cc:__ZN12_GLOBAL__N_17strklenEPKc
#[allow(unused_variables, non_snake_case)]
pub fn global_table_object_key__b7d0b901512f90d0(self_ptr: *mut mujoco__GlobalTable, plugin: *const mjpPlugin) -> std__string_view {
    todo!() // cxx:__ZN6mujoco11GlobalTableI10mjpPlugin_vE9ObjectKeyERKS1_
}

/// C: GlobalTable::ObjectEqual (engine/engine_plugin.cc:190)
#[allow(unused_variables, non_snake_case)]
pub fn global_table_object_equal__44f226ace34d71e2(self_ptr: *mut mujoco__GlobalTable, plugin1: *const mjpPlugin, plugin2: *const mjpPlugin) -> bool {
    todo!() // cxx:__ZN6mujoco11GlobalTableI10mjpPlugin_vE11ObjectEqualERKS1_S4_
}

/// C: GlobalTable::CopyObject (engine/engine_plugin.cc:229)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_plugin.cc:__ZN12_GLOBAL__N_17strklenEPKc, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_plugin.cc:__ZN12_GLOBAL__N_18CopyNameEPKc
#[allow(unused_variables, non_snake_case)]
pub fn global_table_copy_object__92d7a010db82143b(self_ptr: *mut mujoco__GlobalTable, dst: *mut mjpPlugin, src: *const mjpPlugin, err: *const ()) -> bool {
    todo!() // cxx:__ZN6mujoco11GlobalTableI10mjpPlugin_vE10CopyObjectERS1_RKS1_RA512_c
}

/// C: GlobalTable::HumanReadableTypeName (engine/engine_plugin.cc:287)
#[allow(unused_variables, non_snake_case)]
pub fn global_table_human_readable_type_name__17ca4a1ef15912e7() -> *const i8 {
    todo!() // cxx:__ZN6mujoco11GlobalTableI19mjpResourceProvidervE21HumanReadableTypeNameEv
}

/// C: GlobalTable::ObjectKey (engine/engine_plugin.cc:292)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_plugin.cc:__ZN12_GLOBAL__N_119ResourceProviderKeyERK19mjpResourceProvider
#[allow(unused_variables, non_snake_case)]
pub fn global_table_object_key__ce55bd7923a60727(arg0: *const mjpResourceProvider) -> std__string_view {
    todo!() // cxx:__ZN6mujoco11GlobalTableI19mjpResourceProvidervE9ObjectKeyERKS1_
}

/// C: GlobalTable::ObjectEqual (engine/engine_plugin.cc:298)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_plugin.cc:__ZN12_GLOBAL__N_121ResourceProviderEqualERK19mjpResourceProviderS2_
#[allow(unused_variables, non_snake_case)]
pub fn global_table_object_equal__20d26531e7001ed4(arg0: *const mjpResourceProvider, arg1: *const mjpResourceProvider) -> bool {
    todo!() // cxx:__ZN6mujoco11GlobalTableI19mjpResourceProvidervE11ObjectEqualERKS1_S4_
}

/// C: GlobalTable::CopyObject (engine/engine_plugin.cc:303)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_plugin.cc:__ZN12_GLOBAL__N_120ResourceProviderCopyER19mjpResourceProviderRKS0_RA512_c
#[allow(unused_variables, non_snake_case)]
pub fn global_table_copy_object__bce5822fbc8a57db(dst: *mut mjpResourceProvider, src: *const mjpResourceProvider, err: *mut mujoco__GlobalTable__ErrorMessage) -> bool {
    todo!() // cxx:__ZN6mujoco11GlobalTableI19mjpResourceProvidervE10CopyObjectERS1_RKS1_RA512_c
}

/// C: GlobalTable::HumanReadableTypeName (engine/engine_plugin.cc:308)
#[allow(unused_variables, non_snake_case)]
pub fn global_table_human_readable_type_name__3772758d6fb50de5() -> *const i8 {
    todo!() // cxx:__ZN6mujoco11GlobalTableI10mjpDecodervE21HumanReadableTypeNameEv
}

/// C: GlobalTable::ObjectKey (engine/engine_plugin.cc:313)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_plugin.cc:__ZN12_GLOBAL__N_17strklenEPKc
#[allow(unused_variables, non_snake_case)]
pub fn global_table_object_key__514ea0118d9cb3ff(arg0: *const mjpDecoder) -> std__string_view {
    todo!() // cxx:__ZN6mujoco11GlobalTableI10mjpDecodervE9ObjectKeyERKS1_
}

/// C: GlobalTable::ObjectEqual (engine/engine_plugin.cc:327)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_global_table.h:__ZN6mujocoL20CaseInsensitiveEqualENSt3__117basic_string_viewIcNS0_11char_traitsIcEEEES4_
#[allow(unused_variables, non_snake_case)]
pub fn global_table_object_equal__a317b1f137a9b7dd(arg0: *const mjpDecoder, arg1: *const mjpDecoder) -> bool {
    todo!() // cxx:__ZN6mujoco11GlobalTableI10mjpDecodervE11ObjectEqualERKS1_S4_
}

/// C: GlobalTable::CopyObject (engine/engine_plugin.cc:349)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_plugin.cc:__ZN12_GLOBAL__N_17strklenEPKc, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_plugin.cc:__ZN12_GLOBAL__N_18CopyNameEPKc
#[allow(unused_variables, non_snake_case)]
pub fn global_table_copy_object__dcb45b17e3d53f19(dst: *mut mjpDecoder, src: *const mjpDecoder, err: *mut mujoco__GlobalTable__ErrorMessage) -> bool {
    todo!() // cxx:__ZN6mujoco11GlobalTableI10mjpDecodervE10CopyObjectERS1_RKS1_RA512_c
}

/// C: GlobalTable::HumanReadableTypeName (engine/engine_plugin.cc:389)
#[allow(unused_variables, non_snake_case)]
pub fn global_table_human_readable_type_name__d52d1790f56b3045() -> *const i8 {
    todo!() // cxx:__ZN6mujoco11GlobalTableI10mjpEncodervE21HumanReadableTypeNameEv
}

/// C: GlobalTable::ObjectKey (engine/engine_plugin.cc:394)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_plugin.cc:__ZN12_GLOBAL__N_17strklenEPKc
#[allow(unused_variables, non_snake_case)]
pub fn global_table_object_key__70aaaa2902e2c197(arg0: *const mjpEncoder) -> std__string_view {
    todo!() // cxx:__ZN6mujoco11GlobalTableI10mjpEncodervE9ObjectKeyERKS1_
}

/// C: GlobalTable::ObjectEqual (engine/engine_plugin.cc:404)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_global_table.h:__ZN6mujocoL20CaseInsensitiveEqualENSt3__117basic_string_viewIcNS0_11char_traitsIcEEEES4_
#[allow(unused_variables, non_snake_case)]
pub fn global_table_object_equal__83d7c3c80df45ef7(arg0: *const mjpEncoder, arg1: *const mjpEncoder) -> bool {
    todo!() // cxx:__ZN6mujoco11GlobalTableI10mjpEncodervE11ObjectEqualERKS1_S4_
}

/// C: GlobalTable::CopyObject (engine/engine_plugin.cc:426)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_plugin.cc:__ZN12_GLOBAL__N_17strklenEPKc, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_plugin.cc:__ZN12_GLOBAL__N_18CopyNameEPKc
#[allow(unused_variables, non_snake_case)]
pub fn global_table_copy_object__381cc9d5e204bb14(dst: *mut mjpEncoder, src: *const mjpEncoder, err: *mut mujoco__GlobalTable__ErrorMessage) -> bool {
    todo!() // cxx:__ZN6mujoco11GlobalTableI10mjpEncodervE10CopyObjectERS1_RKS1_RA512_c
}

/// C: mjp_registerPlugin (engine/engine_plugin.cc:469)
/// Calls: cxx:_mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mjp_register_plugin(plugin: *const mjpPlugin) -> i32 {
    todo!() // cxx:_mjp_registerPlugin
}

/// C: mjp_getPluginAtSlotUnsafe (engine/engine_plugin.cc:485)
#[allow(unused_variables, non_snake_case)]
pub fn mjp_get_plugin_at_slot_unsafe(slot: i32, nslot: i32) -> *const mjpPlugin {
    todo!() // cxx:_mjp_getPluginAtSlotUnsafe
}

/// C: mjp_getPluginUnsafe (engine/engine_plugin.cc:490)
#[allow(unused_variables, non_snake_case)]
pub fn mjp_get_plugin_unsafe(name: *const i8, slot: *mut i32, nslot: i32) -> *const mjpPlugin {
    todo!() // cxx:_mjp_getPluginUnsafe
}

/// C: mjp_pluginCount (engine/engine_plugin.cc:495)
#[allow(unused_variables, non_snake_case)]
pub fn mjp_plugin_count() -> i32 {
    todo!() // cxx:_mjp_pluginCount
}

/// C: mjp_getPluginAtSlot (engine/engine_plugin.cc:500)
#[allow(unused_variables, non_snake_case)]
pub fn mjp_get_plugin_at_slot(slot: i32) -> *const mjpPlugin {
    todo!() // cxx:_mjp_getPluginAtSlot
}

/// C: mjp_getPlugin (engine/engine_plugin.cc:505)
#[allow(unused_variables, non_snake_case)]
pub fn mjp_get_plugin(name: *const i8, slot: *mut i32) -> *const mjpPlugin {
    todo!() // cxx:_mjp_getPlugin
}

/// C: mj_getPluginConfig (engine/engine_plugin.cc:511)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_plugin.cc:__ZN12_GLOBAL__N_114PluginAttrSeekEPK8mjModel_ii, cxx:_mjp_getPluginAtSlot
#[allow(unused_variables, non_snake_case)]
pub fn mj_get_plugin_config(m: *const mjModel, plugin_id: i32, attrib: *const i8) -> *const i8 {
    todo!() // cxx:_mj_getPluginConfig
}

/// C: mjp_defaultResourceProvider (engine/engine_plugin.cc:531)
#[allow(unused_variables, non_snake_case)]
pub fn mjp_default_resource_provider(provider: *mut mjpResourceProvider) {
    todo!() // cxx:_mjp_defaultResourceProvider
}

/// C: mjp_registerResourceProvider (engine/engine_plugin.cc:536)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_plugin.cc:__ZN12_GLOBAL__N_122IsValidURISchemeFormatEPKc, cxx:_mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn mjp_register_resource_provider(provider: *const mjpResourceProvider) -> i32 {
    todo!() // cxx:_mjp_registerResourceProvider
}

/// C: mjp_resourceProviderCount (engine/engine_plugin.cc:553)
#[allow(unused_variables, non_snake_case)]
pub fn mjp_resource_provider_count() -> i32 {
    todo!() // cxx:_mjp_resourceProviderCount
}

/// C: mjp_getResourceProvider (engine/engine_plugin.cc:558)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_plugin.cc:__ZN12_GLOBAL__N_122IsValidURISchemeFormatEPKc
#[allow(unused_variables, non_snake_case)]
pub fn mjp_get_resource_provider(resource_name: *const i8) -> *const mjpResourceProvider {
    todo!() // cxx:_mjp_getResourceProvider
}

/// C: mjp_getResourceProviderAtSlot (engine/engine_plugin.cc:580)
#[allow(unused_variables, non_snake_case)]
pub fn mjp_get_resource_provider_at_slot(slot: i32) -> *const mjpResourceProvider {
    todo!() // cxx:_mjp_getResourceProviderAtSlot
}

/// C: mjp_registerDecoder (engine/engine_plugin.cc:586)
/// Calls: cxx:_mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn mjp_register_decoder(decoder: *const mjpDecoder) {
    todo!() // cxx:_mjp_registerDecoder
}

/// C: mjp_defaultDecoder (engine/engine_plugin.cc:616)
#[allow(unused_variables, non_snake_case)]
pub fn mjp_default_decoder(decoder: *mut mjpDecoder) {
    todo!() // cxx:_mjp_defaultDecoder
}

/// C: mjp_findDecoder (engine/engine_plugin.cc:621)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_plugin.cc:__ZN12_GLOBAL__N_16getextENSt3__117basic_string_viewIcNS0_11char_traitsIcEEEE, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_plugin.cc:__ZN12_GLOBAL__N_17strklenEPKc, cxx:_mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn mjp_find_decoder(resource: *const mjResource, content_type: *const i8) -> *const mjpDecoder {
    todo!() // cxx:_mjp_findDecoder
}

/// C: mjp_registerEncoder (engine/engine_plugin.cc:645)
/// Calls: cxx:_mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn mjp_register_encoder(encoder: *const mjpEncoder) {
    todo!() // cxx:_mjp_registerEncoder
}

/// C: mjp_defaultEncoder (engine/engine_plugin.cc:677)
#[allow(unused_variables, non_snake_case)]
pub fn mjp_default_encoder(encoder: *mut mjpEncoder) {
    todo!() // cxx:_mjp_defaultEncoder
}

/// C: mjp_findEncoder (engine/engine_plugin.cc:682)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_plugin.cc:__ZN12_GLOBAL__N_16getextENSt3__117basic_string_viewIcNS0_11char_traitsIcEEEE, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/engine/engine_plugin.cc:__ZN12_GLOBAL__N_17strklenEPKc, cxx:_mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn mjp_find_encoder(filename: *const i8, content_type: *const i8) -> *const mjpEncoder {
    todo!() // cxx:_mjp_findEncoder
}

/// C: mj_loadPluginLibrary (engine/engine_plugin.cc:710)
/// Calls: cxx:_mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mj_load_plugin_library(path: *const i8) {
    todo!() // cxx:_mj_loadPluginLibrary
}

/// C: mj_loadAllPluginLibraries (engine/engine_plugin.cc:728)
/// Calls: cxx:_mj_loadPluginLibrary, cxx:_mjp_pluginCount
#[allow(unused_variables, non_snake_case)]
pub fn mj_load_all_plugin_libraries(directory: *const i8, callback: mjfPluginLibraryLoadCallback) {
    todo!() // cxx:_mj_loadAllPluginLibraries
}

