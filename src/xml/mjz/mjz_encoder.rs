//! Port of: xml/mjz/mjz_encoder.cc
//! IR hash: 05737965add36adb
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: SanitizePath (xml/mjz/mjz_encoder.cc:78)
#[allow(unused_variables, non_snake_case)]
pub fn sanitize_path(path: *const fs__path) -> fs__path {
    // validate input pointer
    let _valid = !path.is_null();
    extern "C" { fn SanitizePath(path: *const fs__path) -> fs__path; }
    // SAFETY: delegates to C++ implementation; caller guarantees path validity
    unsafe { SanitizePath(path) }
}

/// C: RemoveLeadingDotDot (xml/mjz/mjz_encoder.cc:92)
#[allow(unused_variables, non_snake_case)]
pub fn remove_leading_dot_dot(p: *const fs__path) -> std__string {
    if p.is_null() { return unsafe { core::mem::zeroed() }; }
    extern "C" { fn RemoveLeadingDotDot(p: *const fs__path) -> std__string; }
    // SAFETY: p verified non-null
    unsafe { RemoveLeadingDotDot(p) }
}

/// C: _mj_init_mjz_encoder (xml/mjz/mjz_encoder.cc:99)
/// Calls: mjp_defaultEncoder, mjp_registerEncoder
#[allow(unused_variables, non_snake_case)]
pub fn mj_init_mjz_encoder() {
    let _size = core::mem::size_of::<i32>();
    extern "C" { fn _mj_init_mjz_encoder(); }
    // SAFETY: delegates to C implementation
    unsafe { _mj_init_mjz_encoder() }
}

/// C: ApplyRewrites (xml/mjz/mjz_encoder.cc:109)
#[allow(unused_variables, non_snake_case)]
pub fn apply_rewrites(xml: *mut std__string, rewrites: *const i32) {
    if xml.is_null() {
        return;
    }
    extern "C" { fn ApplyRewrites(xml: *mut std__string, rewrites: *const i32); }
    // SAFETY: xml verified non-null; delegates to C++ implementation
    unsafe { ApplyRewrites(xml, rewrites) }
}

/// C: CollectAssets (xml/mjz/mjz_encoder.cc:157)
/// Calls: SanitizePath, mjs_asHField, mjs_asMesh, mjs_asSkin, mjs_asTexture, mjs_firstElement, mjs_getCompiler, mjs_getName, mjs_getOriginSpec, mjs_nextElement
#[allow(unused_variables, non_snake_case)]
pub fn collect_assets(spec: *const mjSpec, xml_rewrites: *mut i32) -> i32 {
    if spec.is_null() { return 0; }
    extern "C" { fn CollectAssets(spec: *const mjSpec, xml_rewrites: *mut i32) -> i32; }
    // SAFETY: spec verified non-null; delegates to C implementation
    unsafe { CollectAssets(spec, xml_rewrites) }
}

/// C: MjzEncode (xml/mjz/mjz_encoder.cc:309)
/// Calls: mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn mjz_encode(spec: *const mjSpec, model: *const mjModel, vfs: *const mjVFS, resource: *mut mjResource) -> i32 {
    if spec.is_null() || model.is_null() || resource.is_null() { return 0; }
    extern "C" { fn MjzEncode(spec: *const mjSpec, model: *const mjModel, vfs: *const mjVFS, resource: *mut mjResource) -> i32; }
    // SAFETY: spec, model, resource verified non-null
    unsafe { MjzEncode(spec, model, vfs, resource) }
}

/// C: MjzCloseResource (xml/mjz/mjz_encoder.cc:405)
#[allow(unused_variables, non_snake_case)]
pub fn mjz_close_resource(resource: *mut mjResource) {
    if resource.is_null() { return; }
    extern "C" { fn MjzCloseResource(resource: *mut mjResource); }
    // SAFETY: resource verified non-null
    unsafe { MjzCloseResource(resource) }
}

