//! Port of: xml/mjz/mjz_encoder.cc
//! IR hash: 699b5f0da57e8d78
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: SanitizePath (xml/mjz/mjz_encoder.cc:78)
#[allow(unused_variables, non_snake_case)]
pub fn sanitize_path(path: *const fs__path) -> fs__path {
    // WARNING: signature changed — verify body
    // Previous params: (path : * const fs__path)
    // Previous return: fs__path
    todo ! ()
}

/// C: RemoveLeadingDotDot (xml/mjz/mjz_encoder.cc:92)
#[allow(unused_variables, non_snake_case)]
pub fn remove_leading_dot_dot(p: *const fs__path) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (p : * const fs__path)
    // Previous return: i32
    todo ! ()
}

/// C: ApplyRewrites (xml/mjz/mjz_encoder.cc:109)
#[allow(unused_variables, non_snake_case)]
pub fn apply_rewrites(xml: *mut i32, rewrites: *const i32) {
    // WARNING: signature changed — verify body
    // Previous params: (xml : * mut i32, rewrites : * const i32)
    // Previous return: ()
    todo ! ()
}

/// C: CollectAssets (xml/mjz/mjz_encoder.cc:157)
/// Calls: SanitizePath, mjs_asHField, mjs_asMesh, mjs_asSkin, mjs_asTexture, mjs_firstElement, mjs_getCompiler, mjs_getOriginSpec, mjs_nextElement
#[allow(unused_variables, non_snake_case)]
pub fn collect_assets(spec: *const mjSpec, xml_rewrites: *mut i32) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (spec : * const mjSpec, xml_rewrites : * mut i32)
    // Previous return: i32
    todo ! ()
}

/// C: MjzEncode (xml/mjz/mjz_encoder.cc:309)
/// Calls: mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn mjz_encode(spec: *const mjSpec, model: *const mjModel, vfs: *const mjVFS, resource: *mut mjResource) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (spec : * const mjSpec, model : * const mjModel, vfs : * const mjVFS, resource : * mut mjResource)
    // Previous return: i32
    todo ! ()
}

/// C: _mj_init_mjz_encoder (xml/mjz/mjz_encoder.cc:329)
/// Calls: mjp_defaultEncoder, mjp_registerEncoder
#[allow(unused_variables, non_snake_case)]
pub fn mj_init_mjz_encoder() {
    todo ! ()
}

/// C: MjzCloseResource (xml/mjz/mjz_encoder.cc:405)
#[allow(unused_variables, non_snake_case)]
pub fn mjz_close_resource(resource: *mut mjResource) {
    // WARNING: signature changed — verify body
    // Previous params: (resource : * mut mjResource)
    // Previous return: ()
    todo ! ()
}

