//! Port of: xml/mjz/mjz_encoder.cc
//! IR hash: 32301b9dc9774d55
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: SanitizePath (xml/mjz/mjz_encoder.cc:78)
#[allow(unused_variables, non_snake_case)]
pub fn sanitize_path(path: *const fs__path) -> fs__path {
    // NOTE: signature changed from previous IR version
    // Previous params: (path : * const fs__path)
    // Previous return: fs__path
    todo!("re-translate: params renamed")
}

/// C: RemoveLeadingDotDot (xml/mjz/mjz_encoder.cc:92)
#[allow(unused_variables, non_snake_case)]
pub fn remove_leading_dot_dot(p: *const fs__path) -> std__string {
    // NOTE: signature changed from previous IR version
    // Previous params: (p : * const fs__path)
    // Previous return: std__string
    todo!("re-translate: params renamed")
}

/// C: _mj_init_mjz_encoder (xml/mjz/mjz_encoder.cc:99)
/// Calls: mjp_defaultEncoder, mjp_registerEncoder
#[allow(unused_variables, non_snake_case)]
pub fn mj_init_mjz_encoder() {
    todo ! ()
}

/// C: ApplyRewrites (xml/mjz/mjz_encoder.cc:109)
#[allow(unused_variables, non_snake_case)]
pub fn apply_rewrites(xml: *mut std__string, rewrites: *const i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (xml : * mut std__string, rewrites : * const i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: CollectAssets (xml/mjz/mjz_encoder.cc:157)
/// Calls: SanitizePath, mjs_asHField, mjs_asMesh, mjs_asSkin, mjs_asTexture, mjs_firstElement, mjs_getCompiler, mjs_getName, mjs_getOriginSpec, mjs_nextElement
#[allow(unused_variables, non_snake_case)]
pub fn collect_assets(spec: *const mjSpec, xml_rewrites: *mut i32) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (spec : * const mjSpec, xml_rewrites : * mut i32)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: MjzEncode (xml/mjz/mjz_encoder.cc:309)
/// Calls: mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn mjz_encode(spec: *const mjSpec, model: *const mjModel, vfs: *const mjVFS, resource: *mut mjResource) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (spec : * const mjSpec, model : * const mjModel, vfs : * const mjVFS, resource : * mut mjResource)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: MjzCloseResource (xml/mjz/mjz_encoder.cc:405)
#[allow(unused_variables, non_snake_case)]
pub fn mjz_close_resource(resource: *mut mjResource) {
    // NOTE: signature changed from previous IR version
    // Previous params: (resource : * mut mjResource)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

