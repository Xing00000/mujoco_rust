//! Port of: xml/mjz/mjz_encoder.cc
//! IR hash: fd5288ad7242f001
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: SanitizePath (xml/mjz/mjz_encoder.cc:83)
#[allow(unused_variables, non_snake_case)]
pub fn sanitize_path(path: *const std__filesystem__path) -> std__filesystem__path {
    todo!() // cxx-internal:mjz_encoder.cc.o:__ZN12_GLOBAL__N_112SanitizePathERKNSt3__14__fs10filesystem4pathE
}

/// C: RemoveLeadingDotDot (xml/mjz/mjz_encoder.cc:101)
#[allow(unused_variables, non_snake_case)]
pub fn remove_leading_dot_dot(p: *const std__filesystem__path) -> std__string {
    todo!() // cxx-internal:mjz_encoder.cc.o:__ZN12_GLOBAL__N_119RemoveLeadingDotDotERKNSt3__14__fs10filesystem4pathE
}

/// C: ApplyRewrites (xml/mjz/mjz_encoder.cc:117)
#[allow(unused_variables, non_snake_case)]
pub fn apply_rewrites(xml: *mut std__string, rewrites: *const anonymous_namespace___RewriteMap) {
    todo!() // cxx-internal:mjz_encoder.cc.o:__ZN12_GLOBAL__N_113ApplyRewritesERNSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEERKNS0_13unordered_mapINS_10RewriteKeyES6_NS_14RewriteKeyHashENS0_8equal_toIS9_EENS4_INS0_4pairIKS9_S6_EEEEEE
}

/// C: CollectAssets (xml/mjz/mjz_encoder.cc:162)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/xml/mjz/mjz_encoder.cc:__ZN12_GLOBAL__N_112SanitizePathERKNSt3__14__fs10filesystem4pathE, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/xml/mjz/mjz_encoder.cc:__ZN12_GLOBAL__N_119RemoveLeadingDotDotERKNSt3__14__fs10filesystem4pathE, cxx:_mjs_asHField, cxx:_mjs_asMesh, cxx:_mjs_asSkin, cxx:_mjs_asTexture, cxx:_mjs_firstElement, cxx:_mjs_getCompiler, cxx:_mjs_getName, cxx:_mjs_getOriginSpec, cxx:_mjs_nextElement, cxx:_mju_error
#[allow(unused_variables, non_snake_case)]
pub fn collect_assets(spec: *const mjSpec, xml_rewrites: *mut anonymous_namespace___RewriteMap) -> *const () {
    todo!() // cxx-internal:mjz_encoder.cc.o:__ZN12_GLOBAL__N_113CollectAssetsEPK7mjSpec_RNSt3__113unordered_mapINS_10RewriteKeyENS3_12basic_stringIcNS3_11char_traitsIcEENS3_9allocatorIcEEEENS_14RewriteKeyHashENS3_8equal_toIS5_EENS9_INS3_4pairIKS5_SB_EEEEEE
}

/// C: MjzEncode (xml/mjz/mjz_encoder.cc:303)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/xml/mjz/mjz_encoder.cc:__ZN12_GLOBAL__N_113ApplyRewritesERNSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEERKNS0_13unordered_mapINS_10RewriteKeyES6_NS_14RewriteKeyHashENS0_8equal_toIS9_EENS4_INS0_4pairIKS9_S6_EEEEEE, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/xml/mjz/mjz_encoder.cc:__ZN12_GLOBAL__N_113CollectAssetsEPK7mjSpec_RNSt3__113unordered_mapINS_10RewriteKeyENS3_12basic_stringIcNS3_11char_traitsIcEENS3_9allocatorIcEEEENS_14RewriteKeyHashENS3_8equal_toIS5_EENS9_INS3_4pairIKS5_SB_EEEEEE, cxx:_mj_saveXMLString, cxx:_mju_closeResource, cxx:_mju_openResource, cxx:_mju_readResource, cxx:_mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn mjz_encode(spec: *const mjSpec, model: *const mjModel, vfs: *const mjVFS, resource: *mut mjResource) -> i64 {
    todo!() // cxx-internal:mjz_encoder.cc.o:__ZN12_GLOBAL__N_19MjzEncodeEPK7mjSpec_PK8mjModel_PK6mjVFS_P11mjResource_
}

/// C: _mj_init_mjz_encoder (xml/mjz/mjz_encoder.cc:391)
/// Calls: cxx:_mjp_defaultEncoder, cxx:_mjp_registerEncoder
#[allow(unused_variables, non_snake_case)]
pub fn mj_init_mjz_encoder() {
    todo!() // cxx-internal:mjz_encoder.cc.o:__ZL20_mj_init_mjz_encoderv
}

/// C: MjzCloseResource (xml/mjz/mjz_encoder.cc:403)
#[allow(unused_variables, non_snake_case)]
pub fn mjz_close_resource(resource: *mut mjResource) {
    todo!() // cxx-internal:mjz_encoder.cc.o:__ZN12_GLOBAL__N_116MjzCloseResourceEP11mjResource_
}

