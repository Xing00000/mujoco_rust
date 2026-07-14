//! Port of: xml/mjz/mjz_encoder.cc
//! IR hash: 8cbd078414266fa8
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: SanitizePath (xml/mjz/mjz_encoder.cc:78)
#[allow(unused_variables, non_snake_case)]
pub fn sanitize_path(path: *const fs__path) -> fs__path {
    todo!() // SanitizePath
}

/// C: RemoveLeadingDotDot (xml/mjz/mjz_encoder.cc:92)
#[allow(unused_variables, non_snake_case)]
pub fn remove_leading_dot_dot(p: *const fs__path) -> std__string {
    todo!() // RemoveLeadingDotDot
}

/// C: ApplyRewrites (xml/mjz/mjz_encoder.cc:109)
#[allow(unused_variables, non_snake_case)]
pub fn apply_rewrites(xml: *mut std__string, rewrites: *const RewriteMap) {
    todo!() // ApplyRewrites
}

/// C: _mj_init_mjz_encoder (xml/mjz/mjz_encoder.cc:110)
/// Calls: mjp_defaultEncoder, mjp_registerEncoder
#[allow(unused_variables, non_snake_case)]
pub fn mj_init_mjz_encoder() {
    todo ! ()
}

/// C: CollectAssets (xml/mjz/mjz_encoder.cc:157)
/// Calls: RemoveLeadingDotDot, SanitizePath, mjs_asHField, mjs_asMesh, mjs_asSkin, mjs_asTexture, mjs_firstElement, mjs_getCompiler, mjs_getName, mjs_getOriginSpec, mjs_nextElement, mju_error
#[allow(unused_variables, non_snake_case)]
pub fn collect_assets(spec: *const mjSpec, xml_rewrites: *mut RewriteMap) -> *const () {
    todo!() // CollectAssets
}

/// C: MjzEncode (xml/mjz/mjz_encoder.cc:309)
/// Calls: ApplyRewrites, CollectAssets, mj_saveXMLString, mju_closeResource, mju_openResource, mju_readResource, mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn mjz_encode(spec: *const mjSpec, model: *const mjModel, vfs: *const mjVFS, resource: *mut mjResource) -> i32 {
    todo!() // MjzEncode
}

/// C: MjzCloseResource (xml/mjz/mjz_encoder.cc:405)
#[allow(unused_variables, non_snake_case)]
pub fn mjz_close_resource(resource: *mut mjResource) {
    todo!() // MjzCloseResource
}

