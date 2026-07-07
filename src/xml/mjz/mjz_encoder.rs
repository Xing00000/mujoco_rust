//! Port of: xml/mjz/mjz_encoder.cc
//! IR hash: 05737965add36adb
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: SanitizePath (xml/mjz/mjz_encoder.cc:78)
#[allow(unused_variables, non_snake_case)]
pub fn sanitize_path(path: *const fs__path) -> fs__path {
    // WARNING: signature changed — verify body
    // Previous params: (path : * const fs__path)
    // Previous return: fs__path
    extern "C" { fn SanitizePath_impl (path : * const fs__path) -> fs__path ; } unsafe { SanitizePath_impl (path) }
}

/// C: RemoveLeadingDotDot (xml/mjz/mjz_encoder.cc:92)
#[allow(unused_variables, non_snake_case)]
pub fn remove_leading_dot_dot(p: *const fs__path) -> std__string {
    extern "C" { fn RemoveLeadingDotDot_impl(p: *const fs__path) -> std__string; }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { RemoveLeadingDotDot_impl(p) }
}

/// C: _mj_init_mjz_encoder (xml/mjz/mjz_encoder.cc:99)
/// Calls: mjp_defaultEncoder, mjp_registerEncoder
#[allow(unused_variables, non_snake_case)]
pub fn mj_init_mjz_encoder() {
    extern "C" { fn _mj_init_mjz_encoder_impl(); }
    // SAFETY: delegates to C implementation
    unsafe { _mj_init_mjz_encoder_impl() }
}

/// C: ApplyRewrites (xml/mjz/mjz_encoder.cc:109)
#[allow(unused_variables, non_snake_case)]
pub fn apply_rewrites(xml: *mut std__string, rewrites: *const i32) {
    // WARNING: signature changed — verify body
    // Previous params: (xml : * mut std__string, rewrites : * const i32)
    // Previous return: ()
    extern "C" { fn ApplyRewrites_impl (xml : * mut std__string , rewrites : * const i32) ; } unsafe { ApplyRewrites_impl (xml , rewrites) }
}

/// C: CollectAssets (xml/mjz/mjz_encoder.cc:157)
/// Calls: SanitizePath, mjs_asHField, mjs_asMesh, mjs_asSkin, mjs_asTexture, mjs_firstElement, mjs_getCompiler, mjs_getName, mjs_getOriginSpec, mjs_nextElement
#[allow(unused_variables, non_snake_case)]
pub fn collect_assets(spec: *const mjSpec, xml_rewrites: *mut i32) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (spec : * const mjSpec, xml_rewrites : * mut i32)
    // Previous return: i32
    extern "C" { fn CollectAssets_impl (spec : * const mjSpec , xml_rewrites : * mut i32) -> i32 ; } unsafe { CollectAssets_impl (spec , xml_rewrites) }
}

/// C: MjzEncode (xml/mjz/mjz_encoder.cc:309)
/// Calls: mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn mjz_encode(spec: *const mjSpec, model: *const mjModel, vfs: *const mjVFS, resource: *mut mjResource) -> i32 {
    extern "C" { fn MjzEncode_impl(spec: *const mjSpec, model: *const mjModel, vfs: *const mjVFS, resource: *mut mjResource) -> i32; }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { MjzEncode_impl(spec, model, vfs, resource) }
}

/// C: MjzCloseResource (xml/mjz/mjz_encoder.cc:405)
#[allow(unused_variables, non_snake_case)]
pub fn mjz_close_resource(resource: *mut mjResource) {
    extern "C" { fn MjzCloseResource_impl(resource: *mut mjResource); }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { MjzCloseResource_impl(resource) }
}

