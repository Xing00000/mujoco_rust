//! Port of: Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.cc
//! IR hash: 1fa44043fe383ce9
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: PNGImage::Load (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.cc:58)
/// Calls: cxx:_mju_readResource
#[allow(unused_variables, non_snake_case)]
pub fn png_image_load(obj: *const mjCBase, resource: *mut mjResource, color_type: u32) -> PNGImage {
    todo!() // cxx-internal:user_objects.cc.o:__ZN12_GLOBAL__N_18PNGImage4LoadEPK7mjCBaseP11mjResource_16LodePNGColorType
}

/// C: PNGImage::Width (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.cc:60)
#[allow(unused_variables, non_snake_case)]
pub fn png_image_width(self_ptr: *mut PNGImage) -> i32 {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).width_ }
}

/// C: PNGImage::Height (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.cc:61)
#[allow(unused_variables, non_snake_case)]
pub fn png_image_height(self_ptr: *mut PNGImage) -> i32 {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).height_ }
}

/// C: PNGImage::IsSRGB (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.cc:62)
#[allow(unused_variables, non_snake_case)]
pub fn png_image_is_srgb(self_ptr: *mut PNGImage) -> bool {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).is_srgb_ }
}

/// C: PNGImage::MoveData (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.cc:66)
#[allow(unused_variables, non_snake_case)]
pub fn png_image_move_data(self_ptr: *mut PNGImage) -> *mut *mut mjByteVec {
    todo!() // cxx-internal:user_objects.cc.o:__ZNO12_GLOBAL__N_18PNGImage8MoveDataEv
}

/// C: PNGImage::Size (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.cc:69)
#[allow(unused_variables, non_snake_case)]
pub fn png_image_size(self_ptr: *mut PNGImage) -> u64 {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.cc:__ZNK12_GLOBAL__N_18PNGImage4SizeEv
}

/// C: MapFrame (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.cc:139)
/// Calls: cxx:__ZN7mjCBase8SetFrameEP8mjCFrame, cxx:__ZN7mjCBody9SetParentEPS_
#[allow(unused_variables, non_snake_case)]
pub fn map_frame__316e9adc0402e5bc(parent: *const (), child: *const (), frame: *mut mjCFrame, parent_body: *mut mjCBody) {
    todo!() // cxx-internal:user_objects.cc.o:__ZN12_GLOBAL__N_18MapFrameI7mjCBodyEEvRNSt3__16vectorIPT_NS2_9allocatorIS5_EEEES9_P8mjCFramePS1_
}

/// C: MapFrame (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.cc:139)
/// Calls: cxx:__ZN7mjCBase8SetFrameEP8mjCFrame, cxx:__ZN7mjCGeom9SetParentEP7mjCBody
#[allow(unused_variables, non_snake_case)]
pub fn map_frame__37ba4c1df19dd949(parent: *const (), child: *const (), frame: *mut mjCFrame, parent_body: *mut mjCBody) {
    todo!() // cxx-internal:user_objects.cc.o:__ZN12_GLOBAL__N_18MapFrameI7mjCGeomEEvRNSt3__16vectorIPT_NS2_9allocatorIS5_EEEES9_P8mjCFrameP7mjCBody
}

/// C: MapFrame (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.cc:139)
/// Calls: cxx:__ZN7mjCBase8SetFrameEP8mjCFrame, cxx:__ZN7mjCSite9SetParentEP7mjCBody
#[allow(unused_variables, non_snake_case)]
pub fn map_frame__7fef07fa95f42a06(parent: *const (), child: *const (), frame: *mut mjCFrame, parent_body: *mut mjCBody) {
    todo!() // cxx-internal:user_objects.cc.o:__ZN12_GLOBAL__N_18MapFrameI7mjCSiteEEvRNSt3__16vectorIPT_NS2_9allocatorIS5_EEEES9_P8mjCFrameP7mjCBody
}

/// C: MapFrame (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.cc:139)
/// Calls: cxx:__ZN7mjCBase8SetFrameEP8mjCFrame, cxx:__ZN8mjCFrame9SetParentEP7mjCBody
#[allow(unused_variables, non_snake_case)]
pub fn map_frame__af49662c142e5ec8(parent: *const (), child: *const (), frame: *mut mjCFrame, parent_body: *mut mjCBody) {
    todo!() // cxx-internal:user_objects.cc.o:__ZN12_GLOBAL__N_18MapFrameI8mjCFrameEEvRNSt3__16vectorIPT_NS2_9allocatorIS5_EEEES9_PS1_P7mjCBody
}

/// C: MapFrame (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.cc:139)
/// Calls: cxx:__ZN7mjCBase8SetFrameEP8mjCFrame, cxx:__ZN8mjCJoint9SetParentEP7mjCBody
#[allow(unused_variables, non_snake_case)]
pub fn map_frame__d8bc1c9d02264ed8(parent: *const (), child: *const (), frame: *mut mjCFrame, parent_body: *mut mjCBody) {
    todo!() // cxx-internal:user_objects.cc.o:__ZN12_GLOBAL__N_18MapFrameI8mjCJointEEvRNSt3__16vectorIPT_NS2_9allocatorIS5_EEEES9_P8mjCFrameP7mjCBody
}

/// C: MapFrame (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.cc:139)
/// Calls: cxx:__ZN7mjCBase8SetFrameEP8mjCFrame, cxx:__ZN8mjCLight9SetParentEP7mjCBody
#[allow(unused_variables, non_snake_case)]
pub fn map_frame__996b6d0bcce77566(parent: *const (), child: *const (), frame: *mut mjCFrame, parent_body: *mut mjCBody) {
    todo!() // cxx-internal:user_objects.cc.o:__ZN12_GLOBAL__N_18MapFrameI8mjCLightEEvRNSt3__16vectorIPT_NS2_9allocatorIS5_EEEES9_P8mjCFrameP7mjCBody
}

/// C: MapFrame (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.cc:139)
/// Calls: cxx:__ZN7mjCBase8SetFrameEP8mjCFrame, cxx:__ZN9mjCCamera9SetParentEP7mjCBody
#[allow(unused_variables, non_snake_case)]
pub fn map_frame__04fe39535b289c1f(parent: *const (), child: *const (), frame: *mut mjCFrame, parent_body: *mut mjCBody) {
    todo!() // cxx-internal:user_objects.cc.o:__ZN12_GLOBAL__N_18MapFrameI9mjCCameraEEvRNSt3__16vectorIPT_NS2_9allocatorIS5_EEEES9_P8mjCFrameP7mjCBody
}

/// C: checksize (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.cc:153)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn checksize(size: *mut f64, r#type: u32, object: *mut mjCBase, name: *const i8, id: i32) {
    todo!() // cxx-internal:user_objects.cc.o:__ZL9checksizePd7mjtGeomP7mjCBasePKci
}

/// C: checklimited (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.cc:172)
#[allow(unused_variables, non_snake_case)]
pub fn checklimited(obj: *const mjCBase, autolimits: bool, entity: *const i8, attr: *const i8, limited: i32, hasrange: bool) {
    todo!() // cxx-internal:user_objects.cc.o:__ZL12checklimitedPK7mjCBasebPKcS3_ib
}

/// C: islimited (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.cc:185)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn islimited(limited: i32, range: *const f64) -> bool {
    todo!() // cxx-internal:user_objects.cc.o:__ZL9islimitediPKd
}

/// C: pointBoxDistSq (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.cc:635)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn point_box_dist_sq(p: *const f64, aabb: *const f64) -> f64 {
    todo!() // cxx-internal:user_objects.cc.o:__ZN12_GLOBAL__N_114pointBoxDistSqEPKdS1_
}

/// C: pointTriDistSqWithUV (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.cc:652)
/// Calls: cxx-internal:ui_main.c.o:_inside, cxx-internal:user_mesh.cc.o:__ZL8trianglePdS_PKdS1_S1_
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn point_tri_dist_sq_with_uv(p: *const f64, v0: *const f64, v1: *const f64, v2: *const f64, out_u: *mut f64, out_v: *mut f64) -> f64 {
    todo!() // cxx-internal:user_objects.cc.o:__ZN12_GLOBAL__N_120pointTriDistSqWithUVEPKdS1_S1_S1_RdS2_
}

/// C: queryClosestBVHWithFace (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.cc:737)
/// Calls: cxx-internal:user_objects.cc.o:__ZN12_GLOBAL__N_114pointBoxDistSqEPKdS1_, cxx-internal:user_objects.cc.o:__ZN12_GLOBAL__N_120pointTriDistSqWithUVEPKdS1_S1_S1_RdS2_
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn query_closest_bvh_with_face(bvh: *const f64, child: *const i32, nodeid: *const i32, vert: *const f64, face: *const i32, node_idx: i32, p: *const f64, best_dist_sq: *mut f64, best_face: *mut i32, best_u: *mut f64, best_v: *mut f64) {
    todo!() // cxx-internal:user_objects.cc.o:__ZN12_GLOBAL__N_123queryClosestBVHWithFaceEPKdPKiS3_S1_S3_iS1_RdRiS4_S4_
}

/// C: querySignedDistance (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.cc:776)
/// Calls: cxx-internal:user_objects.cc.o:__ZN12_GLOBAL__N_123queryClosestBVHWithFaceEPKdPKiS3_S1_S3_iS1_RdRiS4_S4_
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn query_signed_distance(bvh: *const f64, child: *const i32, nodeid: *const i32, nbvh: i32, point: *const f64, vert: *const f64, face: *const i32) -> f64 {
    todo!() // cxx-internal:user_objects.cc.o:__ZN12_GLOBAL__N_119querySignedDistanceEPKdPKiS3_iS1_S1_S3_
}

/// C: dot2 (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.cc:923)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn dot2(a: *const f64, b: *const f64) -> f64 {
    todo!() // cxx-internal:user_objects.cc.o:__ZL4dot2PKdS0_
}

/// C: boxTriangle (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.cc:929)
/// Calls: cxx-internal:user_objects.cc.o:__ZL4dot2PKdS0_, cxx:__Z13mjuu_crossvecPdPKdS1_, cxx:__Z9mjuu_dot3PKdS0_, cxx:_mju_max
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn box_triangle(v: *const Triangle, aamm: *const f64) -> bool {
    todo!() // cxx-internal:user_objects.cc.o:__ZL11boxTriangleRKNSt3__15arrayINS0_IdLm3EEELm3EEEPKd
}

/// C: mjCBody::CopyList (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.cc:1784)
/// Calls: cxx:__ZN7mjCBase6AddRefEv, cxx:__ZN7mjCGeom10CopyPluginEv, cxx:__ZN7mjCGeom9NameSpaceEPK8mjCModel, cxx:__ZNK8mjCFrame10IsAncestorEPKS_
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_copy_list__bc53ba57fed02259(self_ptr: *mut mjCBody, dst: *const (), src: *const (), fmap: *const (), pframe: *const mjCFrame) {
    todo!() // cxx:__ZN7mjCBody8CopyListI7mjCGeomEEvRNSt3__16vectorIPT_NS2_9allocatorIS5_EEEERKS8_RNS2_3mapIP8mjCFrameiNS2_4lessISE_EENS6_INS2_4pairIKSE_iEEEEEEPKSD_
}

/// C: mjCBody::CopyList (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.cc:1784)
/// Calls: cxx:__ZN7mjCBase10CopyPluginEv, cxx:__ZN7mjCBase6AddRefEv, cxx:__ZN7mjCSite9NameSpaceEPK8mjCModel, cxx:__ZNK8mjCFrame10IsAncestorEPKS_
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_copy_list__c0a715032930d1f1(self_ptr: *mut mjCBody, dst: *const (), src: *const (), fmap: *const (), pframe: *const mjCFrame) {
    todo!() // cxx:__ZN7mjCBody8CopyListI7mjCSiteEEvRNSt3__16vectorIPT_NS2_9allocatorIS5_EEEERKS8_RNS2_3mapIP8mjCFrameiNS2_4lessISE_EENS6_INS2_4pairIKSE_iEEEEEEPKSD_
}

/// C: mjCBody::CopyList (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.cc:1784)
/// Calls: cxx:__ZN7mjCBase10CopyPluginEv, cxx:__ZN7mjCBase6AddRefEv, cxx:__ZN7mjCBase9NameSpaceEPK8mjCModel, cxx:__ZNK8mjCFrame10IsAncestorEPKS_
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_copy_list__733baccf8cccec17(self_ptr: *mut mjCBody, dst: *const (), src: *const (), fmap: *const (), pframe: *const mjCFrame) {
    todo!() // cxx:__ZN7mjCBody8CopyListI8mjCFrameEEvRNSt3__16vectorIPT_NS2_9allocatorIS5_EEEERKS8_RNS2_3mapIPS1_iNS2_4lessISD_EENS6_INS2_4pairIKSD_iEEEEEEPKS1_
}

/// C: mjCBody::CopyList (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.cc:1784)
/// Calls: cxx:__ZN7mjCBase10CopyPluginEv, cxx:__ZN7mjCBase6AddRefEv, cxx:__ZN7mjCBase9NameSpaceEPK8mjCModel, cxx:__ZNK8mjCFrame10IsAncestorEPKS_
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_copy_list__fbb0595f60076a09(self_ptr: *mut mjCBody, dst: *const (), src: *const (), fmap: *const (), pframe: *const mjCFrame) {
    todo!() // cxx:__ZN7mjCBody8CopyListI8mjCJointEEvRNSt3__16vectorIPT_NS2_9allocatorIS5_EEEERKS8_RNS2_3mapIP8mjCFrameiNS2_4lessISE_EENS6_INS2_4pairIKSE_iEEEEEEPKSD_
}

/// C: mjCBody::CopyList (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.cc:1784)
/// Calls: cxx:__ZN7mjCBase10CopyPluginEv, cxx:__ZN7mjCBase6AddRefEv, cxx:__ZN8mjCLight9NameSpaceEPK8mjCModel, cxx:__ZNK8mjCFrame10IsAncestorEPKS_
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_copy_list__b67f5a9c4cc5ece2(self_ptr: *mut mjCBody, dst: *const (), src: *const (), fmap: *const (), pframe: *const mjCFrame) {
    todo!() // cxx:__ZN7mjCBody8CopyListI8mjCLightEEvRNSt3__16vectorIPT_NS2_9allocatorIS5_EEEERKS8_RNS2_3mapIP8mjCFrameiNS2_4lessISE_EENS6_INS2_4pairIKSE_iEEEEEEPKSD_
}

/// C: mjCBody::CopyList (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.cc:1784)
/// Calls: cxx:__ZN7mjCBase10CopyPluginEv, cxx:__ZN7mjCBase6AddRefEv, cxx:__ZN9mjCCamera9NameSpaceEPK8mjCModel, cxx:__ZNK8mjCFrame10IsAncestorEPKS_
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_copy_list__6f380ec65bebab18(self_ptr: *mut mjCBody, dst: *const (), src: *const (), fmap: *const (), pframe: *const mjCFrame) {
    todo!() // cxx:__ZN7mjCBody8CopyListI9mjCCameraEEvRNSt3__16vectorIPT_NS2_9allocatorIS5_EEEERKS8_RNS2_3mapIP8mjCFrameiNS2_4lessISE_EENS6_INS2_4pairIKSE_iEEEEEEPKSD_
}

/// C: findobject (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.cc:2253)
#[allow(unused_variables, non_snake_case)]
pub fn findobject__4684b5bab57a5ce4(name: *const std__string, list: *const ()) -> *mut mjCBody {
    todo!() // cxx-internal:user_objects.cc.o:__ZL10findobjectI7mjCBodyEPT_RKNSt3__112basic_stringIcNS3_11char_traitsIcEENS3_9allocatorIcEEEERKNS3_6vectorIS2_NS7_IS2_EEEE
}

/// C: findobject (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.cc:2253)
#[allow(unused_variables, non_snake_case)]
pub fn findobject__edd52ed32462d623(name: *const std__string, list: *const ()) -> *mut mjCGeom {
    todo!() // cxx-internal:user_objects.cc.o:__ZL10findobjectI7mjCGeomEPT_RKNSt3__112basic_stringIcNS3_11char_traitsIcEENS3_9allocatorIcEEEERKNS3_6vectorIS2_NS7_IS2_EEEE
}

/// C: findobject (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.cc:2253)
#[allow(unused_variables, non_snake_case)]
pub fn findobject__50488e5aa1ac3043(name: *const std__string, list: *const ()) -> *mut mjCSite {
    todo!() // cxx-internal:user_objects.cc.o:__ZL10findobjectI7mjCSiteEPT_RKNSt3__112basic_stringIcNS3_11char_traitsIcEENS3_9allocatorIcEEEERKNS3_6vectorIS2_NS7_IS2_EEEE
}

/// C: findobject (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.cc:2253)
#[allow(unused_variables, non_snake_case)]
pub fn findobject__8fda3b41c57fa4a2(name: *const std__string, list: *const ()) -> *mut mjCJoint {
    todo!() // cxx-internal:user_objects.cc.o:__ZL10findobjectI8mjCJointEPT_RKNSt3__112basic_stringIcNS3_11char_traitsIcEENS3_9allocatorIcEEEERKNS3_6vectorIS2_NS7_IS2_EEEE
}

/// C: findobject (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.cc:2253)
#[allow(unused_variables, non_snake_case)]
pub fn findobject__c1d53a912036d940(name: *const std__string, list: *const ()) -> *mut mjCLight {
    todo!() // cxx-internal:user_objects.cc.o:__ZL10findobjectI8mjCLightEPT_RKNSt3__112basic_stringIcNS3_11char_traitsIcEENS3_9allocatorIcEEEERKNS3_6vectorIS2_NS7_IS2_EEEE
}

/// C: findobject (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.cc:2253)
#[allow(unused_variables, non_snake_case)]
pub fn findobject__8496c14c78547ac7(name: *const std__string, list: *const ()) -> *mut mjCCamera {
    todo!() // cxx-internal:user_objects.cc.o:__ZL10findobjectI9mjCCameraEPT_RKNSt3__112basic_stringIcNS3_11char_traitsIcEENS3_9allocatorIcEEEERKNS3_6vectorIS2_NS7_IS2_EEEE
}

/// C: mjCBody::GetList (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.cc:2311)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_get_list__975cf469c2eb9cdf(self_ptr: *mut mjCBody) -> *const () {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).bodies }
}

/// C: mjCBody::GetList (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.cc:2316)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_get_list__367a34b7b6ac3342(self_ptr: *mut mjCBody) -> *const () {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).joints }
}

/// C: mjCBody::GetList (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.cc:2321)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_get_list__a51bce796e18c18d(self_ptr: *mut mjCBody) -> *const () {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).geoms }
}

/// C: mjCBody::GetList (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.cc:2326)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_get_list__9e70d65d8d0d1c48(self_ptr: *mut mjCBody) -> *const () {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).sites }
}

/// C: mjCBody::GetList (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.cc:2331)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_get_list__81e628945d5100cf(self_ptr: *mut mjCBody) -> *const () {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).cameras }
}

/// C: mjCBody::GetList (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.cc:2336)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_get_list__975841557671111b(self_ptr: *mut mjCBody) -> *const () {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).lights }
}

/// C: mjCBody::GetList (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.cc:2341)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_get_list__7100ea1c5c87a8e7(self_ptr: *mut mjCBody) -> *const () {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).frames }
}

/// C: GetNext (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.cc:2349)
/// Calls: cxx:__ZNK7mjCBody6BodiesEv
#[allow(unused_variables, non_snake_case)]
pub fn get_next__9778a2bd498fb167(body: *const mjCBody, child: *const mjsElement, found: *mut bool, recursive: bool) -> *mut mjsElement {
    todo!() // cxx-internal:user_objects.cc.o:__ZL7GetNextI7mjCGeomEP11mjsElement_PK7mjCBodyPKS1_Pbb
}

/// C: GetNext (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.cc:2349)
/// Calls: cxx:__ZNK7mjCBody6BodiesEv
#[allow(unused_variables, non_snake_case)]
pub fn get_next__61bf10225c82118d(body: *const mjCBody, child: *const mjsElement, found: *mut bool, recursive: bool) -> *mut mjsElement {
    todo!() // cxx-internal:user_objects.cc.o:__ZL7GetNextI7mjCSiteEP11mjsElement_PK7mjCBodyPKS1_Pbb
}

/// C: GetNext (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.cc:2349)
/// Calls: cxx:__ZNK7mjCBody6BodiesEv
#[allow(unused_variables, non_snake_case)]
pub fn get_next__7af41d7c88174834(body: *const mjCBody, child: *const mjsElement, found: *mut bool, recursive: bool) -> *mut mjsElement {
    todo!() // cxx-internal:user_objects.cc.o:__ZL7GetNextI8mjCFrameEP11mjsElement_PK7mjCBodyPKS1_Pbb
}

/// C: GetNext (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.cc:2349)
/// Calls: cxx:__ZNK7mjCBody6BodiesEv
#[allow(unused_variables, non_snake_case)]
pub fn get_next__84801a70ca3a7568(body: *const mjCBody, child: *const mjsElement, found: *mut bool, recursive: bool) -> *mut mjsElement {
    todo!() // cxx-internal:user_objects.cc.o:__ZL7GetNextI8mjCJointEP11mjsElement_PK7mjCBodyPKS1_Pbb
}

/// C: GetNext (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.cc:2349)
/// Calls: cxx:__ZNK7mjCBody6BodiesEv
#[allow(unused_variables, non_snake_case)]
pub fn get_next__47549d91155d4bc8(body: *const mjCBody, child: *const mjsElement, found: *mut bool, recursive: bool) -> *mut mjsElement {
    todo!() // cxx-internal:user_objects.cc.o:__ZL7GetNextI8mjCLightEP11mjsElement_PK7mjCBodyPKS1_Pbb
}

/// C: GetNext (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.cc:2349)
/// Calls: cxx:__ZNK7mjCBody6BodiesEv
#[allow(unused_variables, non_snake_case)]
pub fn get_next__898c28a1a4face8d(body: *const mjCBody, child: *const mjsElement, found: *mut bool, recursive: bool) -> *mut mjsElement {
    todo!() // cxx-internal:user_objects.cc.o:__ZL7GetNextI9mjCCameraEP11mjsElement_PK7mjCBodyPKS1_Pbb
}

/// C: GetNextBody (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.cc:2380)
/// Calls: cxx:__ZNK7mjCBody6BodiesEv
#[allow(unused_variables, non_snake_case)]
pub fn get_next_body(body: *const mjCBody, child: *const mjsElement, found: *mut bool, recursive: bool) -> *mut mjsElement {
    todo!() // cxx-internal:user_objects.cc.o:__ZL11GetNextBodyPK7mjCBodyPK11mjsElement_Pbb
}

/// C: randomdot (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.cc:4973)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn randomdot(rgb: *mut u8, markrgb: *const f64, width: i32, height: i32, probability: f64) {
    todo!() // cxx-internal:user_objects.cc.o:__ZL9randomdotPSt4bytePKdiid
}

/// C: interp (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.cc:4995)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn interp(rgb: *mut u8, rgb1: *const f64, rgb2: *const f64, pos: f64) {
    todo!() // cxx-internal:user_objects.cc.o:__ZL6interpPSt4bytePKdS2_d
}

/// C: checker (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.cc:5012)
#[allow(unused_variables, non_snake_case)]
pub fn checker(rgb: *mut u8, RGB1: *const u8, RGB2: *const u8, width: i32, height: i32) {
    todo!() // cxx-internal:user_objects.cc.o:__ZL7checkerPSt4bytePKS_S2_ii
}

/// C: sensorDatatype (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.cc:7480)
#[allow(unused_variables, non_snake_case)]
pub fn sensor_datatype(r#type: u32) -> u32 {
    todo!() // cxx:__Z14sensorDatatype9mjtSensor
}

/// C: sensorNeedstage (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.cc:7544)
#[allow(unused_variables, non_snake_case)]
pub fn sensor_needstage(r#type: u32) -> u32 {
    todo!() // cxx:__Z15sensorNeedstage9mjtSensor
}

/// C: ResolveOrientation (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:89)
/// Calls: cxx:__Z11mjuu_z2quatPdPKd, cxx:__Z12mjuu_mulquatPdPKdS1_, cxx:__Z13mjuu_crossvecPdPKdS1_, cxx:__Z15mjuu_frame2quatPdPKdS1_S1_, cxx:__Z9mjuu_dot3PKdS0_
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn resolve_orientation(quat: *mut f64, degree: bool, sequence: *const i8, orient: *const mjsOrientation) -> *const i8 {
    todo!() // cxx:__Z18ResolveOrientationPdbPKcRK15mjsOrientation_
}

/// C: mjCBoundingVolume::Contype (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:122)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_contype(self_ptr: *mut mjCBoundingVolume) -> i32 {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).contype_ }
}

/// C: mjCBoundingVolume::Conaffinity (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:123)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_conaffinity(self_ptr: *mut mjCBoundingVolume) -> i32 {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).conaffinity_ }
}

/// C: mjCBoundingVolume::AABB (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:124)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_aabb__f91f459ff16927a5(self_ptr: *mut mjCBoundingVolume) -> *const f64 {
    todo!() // cxx:__ZNK17mjCBoundingVolume4AABBEv
}

/// C: mjCBoundingVolume::AABB (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:125)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_aabb__bef70a2abaacd237(self_ptr: *mut mjCBoundingVolume, i: i32) -> f64 {
    todo!() // cxx:__ZNK17mjCBoundingVolume4AABBEi
}

/// C: mjCBoundingVolume::Pos (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:126)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_pos__d9995d865a4070c0(self_ptr: *mut mjCBoundingVolume) -> *const f64 {
    todo!() // cxx:__ZNK17mjCBoundingVolume3PosEv
}

/// C: mjCBoundingVolume::Pos (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:127)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_pos__95aecb8f440b706f(self_ptr: *mut mjCBoundingVolume, i: i32) -> f64 {
    todo!() // cxx:__ZNK17mjCBoundingVolume3PosEi
}

/// C: mjCBoundingVolume::Quat (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:128)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_quat(self_ptr: *mut mjCBoundingVolume) -> *const f64 {
    todo!() // cxx:__ZNK17mjCBoundingVolume4QuatEv
}

/// C: mjCBoundingVolume::Id (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:129)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_id(self_ptr: *mut mjCBoundingVolume) -> *const i32 {
    todo!() // cxx:__ZNK17mjCBoundingVolume2IdEv
}

/// C: mjCBoundingVolume::SetContype (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:131)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_set_contype(self_ptr: *mut mjCBoundingVolume, val: i32) {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).contype_ = val; }
}

/// C: mjCBoundingVolume::SetConaffinity (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:132)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_set_conaffinity(self_ptr: *mut mjCBoundingVolume, val: i32) {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).conaffinity_ = val; }
}

/// C: mjCBoundingVolume::SetAABB (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:133)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_set_aabb(self_ptr: *mut mjCBoundingVolume, aabb: *const f64) {
    todo!() // cxx:__ZN17mjCBoundingVolume7SetAABBEPKd
}

/// C: mjCBoundingVolume::SetPos (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:134)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_set_pos(self_ptr: *mut mjCBoundingVolume, pos: *const f64) {
    todo!() // cxx:__ZN17mjCBoundingVolume6SetPosEPKd
}

/// C: mjCBoundingVolume::SetQuat (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:135)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_set_quat(self_ptr: *mut mjCBoundingVolume, quat: *const f64) {
    todo!() // cxx:__ZN17mjCBoundingVolume7SetQuatEPKd
}

/// C: mjCBoundingVolume::SetId (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:139)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_set_id__36523affc3f9a04e(self_ptr: *mut mjCBoundingVolume, id: *const i32) {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).id_ = id; }
}

/// C: mjCBoundingVolume::SetId (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:140)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_set_id__9fa4dc447dfb6240(self_ptr: *mut mjCBoundingVolume, val: i32) {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).idval_ = val; }
}

/// C: mjCBoundingVolumeHierarchy::CreateBVH (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:174)
/// Calls: cxx:__ZN26mjCBoundingVolumeHierarchy4MakeERNSt3__16vectorINS_9BVElementENS0_9allocatorIS2_EEEE, cxx:__ZN26mjCBoundingVolumeHierarchy7MakeBVHENSt3__111__wrap_iterIPNS_9BVElementEEES4_iP8mjCModelPK7mjCBase
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_hierarchy_create_bvh(self_ptr: *mut mjCBoundingVolumeHierarchy, model: *mut mjCModel, owner: *const mjCBase) {
    todo!() // cxx:__ZN26mjCBoundingVolumeHierarchy9CreateBVHEP8mjCModelPK7mjCBase
}

/// C: mjCBoundingVolumeHierarchy::Set (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:175)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_hierarchy_set(self_ptr: *mut mjCBoundingVolumeHierarchy, ipos_element: *mut f64, iquat_element: *mut f64) {
    todo!() // cxx:__ZN26mjCBoundingVolumeHierarchy3SetEPdS0_
}

/// C: mjCBoundingVolumeHierarchy::AllocateBoundingVolumes (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:176)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_hierarchy_allocate_bounding_volumes(self_ptr: *mut mjCBoundingVolumeHierarchy, nleaf: i32) {
    todo!() // cxx:__ZN26mjCBoundingVolumeHierarchy23AllocateBoundingVolumesEi
}

/// C: mjCBoundingVolumeHierarchy::RemoveInactiveVolumes (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:177)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_hierarchy_remove_inactive_volumes(self_ptr: *mut mjCBoundingVolumeHierarchy, nmax: i32) {
    todo!() // cxx:__ZN26mjCBoundingVolumeHierarchy21RemoveInactiveVolumesEi
}

/// C: mjCBoundingVolumeHierarchy::AddBoundingVolume (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:179)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_hierarchy_add_bounding_volume__4bfb235aafdb0eeb(self_ptr: *mut mjCBoundingVolumeHierarchy, id: i32, contype: i32, conaffinity: i32, pos: *const f64, quat: *const f64, aabb: *const f64) -> *const mjCBoundingVolume {
    todo!() // cxx:__ZN26mjCBoundingVolumeHierarchy17AddBoundingVolumeEiiiPKdS1_S1_
}

/// C: mjCBoundingVolumeHierarchy::AddBoundingVolume (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:182)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_hierarchy_add_bounding_volume__a0616ff3267405ea(self_ptr: *mut mjCBoundingVolumeHierarchy, id: *const i32, contype: i32, conaffinity: i32, pos: *const f64, quat: *const f64, aabb: *const f64) -> *const mjCBoundingVolume {
    todo!() // cxx:__ZN26mjCBoundingVolumeHierarchy17AddBoundingVolumeEPKiiiPKdS3_S3_
}

/// C: mjCBoundingVolumeHierarchy::Nbvh (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:186)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_hierarchy_nbvh(self_ptr: *mut mjCBoundingVolumeHierarchy) -> i32 {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).nbvh_ }
}

/// C: mjCBoundingVolumeHierarchy::Bvh (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:187)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_hierarchy_bvh(self_ptr: *mut mjCBoundingVolumeHierarchy) -> *const () {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).bvh_ }
}

/// C: mjCBoundingVolumeHierarchy::Child (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:188)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_hierarchy_child(self_ptr: *mut mjCBoundingVolumeHierarchy) -> *const () {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).child_ }
}

/// C: mjCBoundingVolumeHierarchy::Nodeid (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:189)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_hierarchy_nodeid__c5962117dca39b32(self_ptr: *mut mjCBoundingVolumeHierarchy) -> *const () {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).nodeid_ }
}

/// C: mjCBoundingVolumeHierarchy::Nodeid (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:190)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_hierarchy_nodeid__5535530f702bd01d(self_ptr: *mut mjCBoundingVolumeHierarchy, id: i32) -> i32 {
    todo!() // cxx:__ZNK26mjCBoundingVolumeHierarchy6NodeidEi
}

/// C: mjCBoundingVolumeHierarchy::Nodeidptr (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:191)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_hierarchy_nodeidptr(self_ptr: *mut mjCBoundingVolumeHierarchy, id: i32) -> *const i32 {
    todo!() // cxx:__ZNK26mjCBoundingVolumeHierarchy9NodeidptrEi
}

/// C: mjCBoundingVolumeHierarchy::Level (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:192)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_hierarchy_level(self_ptr: *mut mjCBoundingVolumeHierarchy) -> *const () {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).level_ }
}

/// C: mjCBoundingVolumeHierarchy::Size (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:193)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_hierarchy_size(self_ptr: *mut mjCBoundingVolumeHierarchy) -> i32 {
    todo!() // cxx:__ZNK26mjCBoundingVolumeHierarchy4SizeEv
}

/// C: mjCBoundingVolumeHierarchy::QuerySignedDistance (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:200)
/// Calls: cxx-internal:user_objects.cc.o:__ZN12_GLOBAL__N_119querySignedDistanceEPKdPKiS3_iS1_S1_S3_
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_hierarchy_query_signed_distance(self_ptr: *mut mjCBoundingVolumeHierarchy, point: *const f64, vert: *const f64, face: *const i32) -> f64 {
    todo!() // cxx:__ZNK26mjCBoundingVolumeHierarchy19QuerySignedDistanceEPKdS1_PKi
}

/// C: mjCBoundingVolumeHierarchy::Make (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:210)
/// Calls: cxx:__Z15mjuu_rotVecQuatPdPKdS1_, cxx:__ZNK17mjCBoundingVolume11ConaffinityEv, cxx:__ZNK17mjCBoundingVolume7ContypeEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_hierarchy_make(self_ptr: *mut mjCBoundingVolumeHierarchy, elements: *const ()) {
    todo!() // cxx:__ZN26mjCBoundingVolumeHierarchy4MakeERNSt3__16vectorINS_9BVElementENS0_9allocatorIS2_EEEE
}

/// C: mjCBoundingVolumeHierarchy::MakeBVH (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:211)
/// Calls: cxx:__Z15mjuu_rotVecQuatPdPKdS1_, cxx:__ZN8mjCModel10AddWarningENSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEEPK7mjCBase, cxx:__ZNK17mjCBoundingVolume2IdEv, cxx:__ZNK17mjCBoundingVolume4QuatEv, cxx:_mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_hierarchy_make_bvh(self_ptr: *mut mjCBoundingVolumeHierarchy, elements_begin: *const (), elements_end: *const (), lev: i32, model: *mut mjCModel, owner: *const mjCBase) -> i32 {
    todo!() // cxx:__ZN26mjCBoundingVolumeHierarchy7MakeBVHENSt3__111__wrap_iterIPNS_9BVElementEEES4_iP8mjCModelPK7mjCBase
}

/// C: mjCOctree::CreateOctree (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:285)
/// Calls: cxx:__ZN9mjCOctree10MakeOctreeERKNSt3__16vectorIPNS0_5arrayINS2_IdLm3EEELm3EEENS0_9allocatorIS5_EEEEPKdRNS0_13unordered_mapI5PointiNS0_4hashISE_EENS0_8equal_toISE_EENS6_INS0_4pairIKSE_iEEEEEE, cxx:__ZN9mjCOctree16MarkHangingNodesEv, cxx:__ZN9mjCOctree4MakeERNSt3__16vectorINS0_5arrayINS2_IdLm3EEELm3EEENS0_9allocatorIS4_EEEE, cxx:__ZN9mjCOctree5ClearEv
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_create_octree(self_ptr: *mut mjCOctree, aamm: *const f64) {
    todo!() // cxx:__ZN9mjCOctree12CreateOctreeEPKd
}

/// C: mjCOctree::NumNodes (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:287)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_num_nodes(self_ptr: *mut mjCOctree) -> i32 {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).nnode_ }
}

/// C: mjCOctree::NumVerts (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:288)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_num_verts(self_ptr: *mut mjCOctree) -> i32 {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).nvert_ }
}

/// C: mjCOctree::CopyLevel (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:289)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_copy_level(self_ptr: *mut mjCOctree, level: *mut i32) {
    todo!() // cxx:__ZNK9mjCOctree9CopyLevelEPi
}

/// C: mjCOctree::CopyChild (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:290)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_copy_child(self_ptr: *mut mjCOctree, child: *mut i32) {
    todo!() // cxx:__ZNK9mjCOctree9CopyChildEPi
}

/// C: mjCOctree::CopyAabb (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:291)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_copy_aabb(self_ptr: *mut mjCOctree, aabb: *mut f64) {
    todo!() // cxx:__ZNK9mjCOctree8CopyAabbEPd
}

/// C: mjCOctree::CopyCoeff (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:292)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_copy_coeff(self_ptr: *mut mjCOctree, coeff: *mut f64) {
    todo!() // cxx:__ZNK9mjCOctree9CopyCoeffEPd
}

/// C: mjCOctree::Vert (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:293)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_vert(self_ptr: *mut mjCOctree, i: i32) -> *const f64 {
    todo!() // cxx:__ZNK9mjCOctree4VertEi
}

/// C: mjCOctree::Hang (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:294)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_hang(self_ptr: *mut mjCOctree, i: i32) -> *const () {
    todo!() // cxx:__ZNK9mjCOctree4HangEi
}

/// C: mjCOctree::VertId (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:295)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_vert_id(self_ptr: *mut mjCOctree, n: i32, v: i32) -> i32 {
    todo!() // cxx:__ZNK9mjCOctree6VertIdEii
}

/// C: mjCOctree::Children (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:296)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_children(self_ptr: *mut mjCOctree, i: i32) -> *const () {
    todo!() // cxx:__ZNK9mjCOctree8ChildrenEi
}

/// C: mjCOctree::SetFace (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:297)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_set_face(self_ptr: *mut mjCOctree, vert: *const (), face: *const ()) {
    todo!() // cxx:__ZN9mjCOctree7SetFaceERKNSt3__16vectorIdNS0_9allocatorIdEEEERKNS1_IiNS2_IiEEEE
}

/// C: mjCOctree::Size (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:298)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_size(self_ptr: *mut mjCOctree) -> i32 {
    todo!() // cxx:__ZNK9mjCOctree4SizeEv
}

/// C: mjCOctree::Clear (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:302)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_clear(self_ptr: *mut mjCOctree) {
    todo!() // cxx:__ZN9mjCOctree5ClearEv
}

/// C: mjCOctree::AddCoeff (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:309)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_add_coeff(self_ptr: *mut mjCOctree, n: i32, v: i32, coeff: f64) {
    todo!() // cxx:__ZN9mjCOctree8AddCoeffEiid
}

/// C: mjCOctree::Coeff (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:310)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_coeff(self_ptr: *mut mjCOctree, n: i32, v: i32) -> f64 {
    todo!() // cxx:__ZNK9mjCOctree5CoeffEii
}

/// C: mjCOctree::SetMaxDepth (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:313)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_set_max_depth(self_ptr: *mut mjCOctree, depth: i32) {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).max_depth_ = depth; }
}

/// C: mjCOctree::MaxDepth (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:314)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_max_depth(self_ptr: *mut mjCOctree) -> i32 {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).max_depth_ }
}

/// C: mjCOctree::SetSmoothingIterations (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:317)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_set_smoothing_iterations(self_ptr: *mut mjCOctree, iterations: i32) {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).smoothing_iterations_ = iterations; }
}

/// C: mjCOctree::SmoothingIterations (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:318)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_smoothing_iterations(self_ptr: *mut mjCOctree) -> i32 {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).smoothing_iterations_ }
}

/// C: mjCOctree::ComputeSdfCoeffs (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:321)
/// Calls: cxx:__Z15mjuu_rotVecQuatPdPKdS1_, cxx:__ZN9mjCOctree8AddCoeffEiid, cxx:__ZNK26mjCBoundingVolumeHierarchy19QuerySignedDistanceEPKdS1_PKi, cxx:__ZNK9mjCOctree4HangEi, cxx:__ZNK9mjCOctree4VertEi, cxx:__ZNK9mjCOctree6VertIdEii, cxx:__ZNK9mjCOctree8ChildrenEi, cxx:__ZNK9mjCOctree8NumNodesEv
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_compute_sdf_coeffs(self_ptr: *mut mjCOctree, vert: *const f64, nvert: i32, face: *const i32, nface: i32, tree: *const mjCBoundingVolumeHierarchy) {
    todo!() // cxx:__ZN9mjCOctree16ComputeSdfCoeffsEPKdiPKiiRK26mjCBoundingVolumeHierarchy
}

/// C: mjCOctree::Make (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:325)
/// Calls: cxx:__Z15mjuu_rotVecQuatPdPKdS1_
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_make(self_ptr: *mut mjCOctree, elements: *const ()) {
    todo!() // cxx:__ZN9mjCOctree4MakeERNSt3__16vectorINS0_5arrayINS2_IdLm3EEELm3EEENS0_9allocatorIS4_EEEE
}

/// C: mjCOctree::MakeOctree (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:326)
/// Calls: cxx-internal:user_objects.cc.o:__ZL11boxTriangleRKNSt3__15arrayINS0_IdLm3EEELm3EEEPKd, cxx:__ZN9mjCOctree10TaskToNodeERK10OctreeTaskR7OctNodeRNSt3__113unordered_mapI5PointiNS5_4hashIS7_EENS5_8equal_toIS7_EENS5_9allocatorINS5_4pairIKS7_iEEEEEE, cxx:__ZN9mjCOctree13BalanceOctreeERNSt3__113unordered_mapI5PointiNS0_4hashIS2_EENS0_8equal_toIS2_EENS0_9allocatorINS0_4pairIKS2_iEEEEEE, cxx:__ZN9mjCOctree9SubdivideERK10OctreeTaskRNSt3__113unordered_mapI5PointiNS3_4hashIS5_EENS3_8equal_toIS5_EENS3_9allocatorINS3_4pairIKS5_iEEEEEEPNS3_5dequeIS0_NSA_IS0_EEEERKNS3_6vectorIPNS3_5arrayINSM_IdLm3EEELm3EEENSA_ISP_EEEE
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_make_octree(self_ptr: *mut mjCOctree, elements: *const (), aamm: *const f64, vert_map: *const ()) {
    todo!() // cxx:__ZN9mjCOctree10MakeOctreeERKNSt3__16vectorIPNS0_5arrayINS2_IdLm3EEELm3EEENS0_9allocatorIS5_EEEEPKdRNS0_13unordered_mapI5PointiNS0_4hashISE_EENS0_8equal_toISE_EENS6_INS0_4pairIKSE_iEEEEEE
}

/// C: mjCOctree::TaskToNode (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:328)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_task_to_node(self_ptr: *mut mjCOctree, task: *const OctreeTask, node: *mut OctNode, vert_map: *const ()) {
    todo!() // cxx:__ZN9mjCOctree10TaskToNodeERK10OctreeTaskR7OctNodeRNSt3__113unordered_mapI5PointiNS5_4hashIS7_EENS5_8equal_toIS7_EENS5_9allocatorINS5_4pairIKS7_iEEEEEE
}

/// C: mjCOctree::Subdivide (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:329)
/// Calls: cxx:__ZN9mjCOctree10TaskToNodeERK10OctreeTaskR7OctNodeRNSt3__113unordered_mapI5PointiNS5_4hashIS7_EENS5_8equal_toIS7_EENS5_9allocatorINS5_4pairIKS7_iEEEEEE
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_subdivide(self_ptr: *mut mjCOctree, task: *const OctreeTask, vert_map: *const (), queue: *const (), colliding: *const ()) {
    todo!() // cxx:__ZN9mjCOctree9SubdivideERK10OctreeTaskRNSt3__113unordered_mapI5PointiNS3_4hashIS5_EENS3_8equal_toIS5_EENS3_9allocatorINS3_4pairIKS5_iEEEEEEPNS3_5dequeIS0_NSA_IS0_EEEERKNS3_6vectorIPNS3_5arrayINSM_IdLm3EEELm3EEENSA_ISP_EEEE
}

/// C: mjCOctree::FindNeighbor (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:332)
/// Calls: cxx:__ZN9mjCOctree18FindCoarseNeighborEii
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_find_neighbor(self_ptr: *mut mjCOctree, node_idx: i32, dir: i32) -> i32 {
    todo!() // cxx:__ZN9mjCOctree12FindNeighborEii
}

/// C: mjCOctree::FindCoarseNeighbor (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:333)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_find_coarse_neighbor(self_ptr: *mut mjCOctree, node_idx: i32, dir: i32) -> i32 {
    todo!() // cxx:__ZN9mjCOctree18FindCoarseNeighborEii
}

/// C: mjCOctree::BalanceOctree (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:334)
/// Calls: cxx:__ZN9mjCOctree12FindNeighborEii, cxx:__ZN9mjCOctree9SubdivideERK10OctreeTaskRNSt3__113unordered_mapI5PointiNS3_4hashIS5_EENS3_8equal_toIS5_EENS3_9allocatorINS3_4pairIKS5_iEEEEEEPNS3_5dequeIS0_NSA_IS0_EEEERKNS3_6vectorIPNS3_5arrayINSM_IdLm3EEELm3EEENSA_ISP_EEEE
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_balance_octree(self_ptr: *mut mjCOctree, vert_map: *const ()) {
    todo!() // cxx:__ZN9mjCOctree13BalanceOctreeERNSt3__113unordered_mapI5PointiNS0_4hashIS2_EENS0_8equal_toIS2_EENS0_9allocatorINS0_4pairIKS2_iEEEEEE
}

/// C: mjCOctree::MarkHangingNodes (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:335)
/// Calls: cxx:__ZN9mjCOctree12FindNeighborEii
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_mark_hanging_nodes(self_ptr: *mut mjCOctree) {
    todo!() // cxx:__ZN9mjCOctree16MarkHangingNodesEv
}

/// C: mjCBase::LoadResource (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:358)
/// Calls: cxx:_mju_openResource
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_base_load_resource(modelfiledir: *const std__string, filename: *const std__string, vfs: *const mjVFS) -> *mut mjResource {
    todo!() // cxx:__ZN7mjCBase12LoadResourceERKNSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEES8_PK6mjVFS_
}

/// C: mjCBase::GetAssetContentType (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:363)
/// Calls: cxx:__Z21mjuu_extToContentTypeNSt3__117basic_string_viewIcNS_11char_traitsIcEEEE, cxx:__Z29mjuu_parseContentTypeAttrTypeNSt3__117basic_string_viewIcNS_11char_traitsIcEEEE, cxx:__Z32mjuu_parseContentTypeAttrSubtypeNSt3__117basic_string_viewIcNS_11char_traitsIcEEEE
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_base_get_asset_content_type(resource_name: std__string_view, raw_text: std__string_view) -> std__string {
    todo!() // cxx:__ZN7mjCBase19GetAssetContentTypeENSt3__117basic_string_viewIcNS0_11char_traitsIcEEEES4_
}

/// C: mjCBase::SetFrame (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:366)
/// Calls: cxx:__ZNK7mjCBase9GetParentEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_base_set_frame(self_ptr: *mut mjCBase, _frame: *mut mjCFrame) {
    todo!() // cxx:__ZN7mjCBase8SetFrameEP8mjCFrame
}

/// C: mjCBase::CopyFromSpec (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:369)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_base_copy_from_spec(self_ptr: *mut mjCBase) {
    todo!() // cxx:__ZN7mjCBase12CopyFromSpecEv
}

/// C: mjCBase::ResolveReferences (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:372)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_base_resolve_references(self_ptr: *mut mjCBase, m: *const mjCModel) {
    todo!() // cxx:__ZN7mjCBase17ResolveReferencesEPK8mjCModel
}

/// C: mjCBase::NameSpace (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:375)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_base_name_space(self_ptr: *mut mjCBase, m: *const mjCModel) {
    todo!() // cxx:__ZN7mjCBase9NameSpaceEPK8mjCModel
}

/// C: mjCBase::CopyPlugin (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:378)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_base_copy_plugin(self_ptr: *mut mjCBase) {
    todo!() // cxx:__ZN7mjCBase10CopyPluginEv
}

/// C: mjCBase::GetParent (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:381)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_base_get_parent(self_ptr: *mut mjCBase) -> *mut mjCBase {
    todo!() // cxx:__ZNK7mjCBase9GetParentEv
}

/// C: mjCBase::FindCompiler (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:384)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_base_find_compiler(self_ptr: *mut mjCBase, compiler: *const mjsCompiler) -> *mut mjsCompiler {
    todo!() // cxx:__ZNK7mjCBase12FindCompilerEPK12mjsCompiler_
}

/// C: mjCBase::ForgetKeyframes (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:396)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_base_forget_keyframes__6ef1231ae2d4431f(self_ptr: *mut mjCBase) {
    todo!() // cxx:__ZN7mjCBase15ForgetKeyframesEv
}

/// C: mjCBase::ForgetKeyframes (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:397)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_base_forget_keyframes__e3a86f9f7a718487(self_ptr: *mut mjCBase) {
    todo!() // cxx:__ZNK7mjCBase15ForgetKeyframesEv
}

/// C: mjCBase::AddRef (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:402)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_base_add_ref(self_ptr: *mut mjCBase) {
    todo!() // cxx:__ZN7mjCBase6AddRefEv
}

/// C: mjCBase::GetRef (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:403)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_base_get_ref(self_ptr: *mut mjCBase) -> i32 {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).refcount }
}

/// C: mjCBase::Release (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:404)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_base_release(self_ptr: *mut mjCBase) {
    todo!() // cxx:__ZN7mjCBase7ReleaseEv
}

/// C: mjCBase::SetUserValue (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:411)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_base_set_user_value(self_ptr: *mut mjCBase, key: std__string_view, data: *const (), cleanup: Option<unsafe extern "C" fn(*const ())>) {
    todo!() // cxx:__ZN7mjCBase12SetUserValueENSt3__117basic_string_viewIcNS0_11char_traitsIcEEEEPKvPFvS6_E
}

/// C: mjCBase::GetUserValue (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:413)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_base_get_user_value(self_ptr: *mut mjCBase, key: std__string_view) -> *const () {
    todo!() // cxx:__ZN7mjCBase12GetUserValueENSt3__117basic_string_viewIcNS0_11char_traitsIcEEEE
}

/// C: mjCBase::DeleteUserValue (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:414)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_base_delete_user_value(self_ptr: *mut mjCBase, key: std__string_view) {
    todo!() // cxx:__ZN7mjCBase15DeleteUserValueENSt3__117basic_string_viewIcNS0_11char_traitsIcEEEE
}

/// C: mjCBody::AddBody (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:522)
/// Calls: cxx:__ZN8mjCModel13MakeTreeListsEP7mjCBody, cxx:__ZN8mjCModel14ResetTreeListsEv, cxx:__ZN8mjCModel9SignatureEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_add_body(self_ptr: *mut mjCBody, arg0: *mut mjCDef) -> *mut mjCBody {
    todo!() // cxx:__ZN7mjCBody7AddBodyEP6mjCDef
}

/// C: mjCBody::AddFrame (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:523)
/// Calls: cxx:__ZN8mjCModel13MakeTreeListsEP7mjCBody, cxx:__ZN8mjCModel14ResetTreeListsEv, cxx:__ZN8mjCModel9SignatureEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_add_frame(self_ptr: *mut mjCBody, arg0: *mut mjCFrame) -> *mut mjCFrame {
    todo!() // cxx:__ZN7mjCBody8AddFrameEP8mjCFrame
}

/// C: mjCBody::AddJoint (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:524)
/// Calls: cxx:__ZN8mjCModel13MakeTreeListsEP7mjCBody, cxx:__ZN8mjCModel14ResetTreeListsEv, cxx:__ZN8mjCModel9SignatureEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_add_joint(self_ptr: *mut mjCBody, arg0: *mut mjCDef) -> *mut mjCJoint {
    todo!() // cxx:__ZN7mjCBody8AddJointEP6mjCDef
}

/// C: mjCBody::AddFreeJoint (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:525)
/// Calls: cxx:__ZN8mjCModel13MakeTreeListsEP7mjCBody, cxx:__ZN8mjCModel14ResetTreeListsEv, cxx:__ZN8mjCModel9SignatureEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_add_free_joint(self_ptr: *mut mjCBody) -> *mut mjCJoint {
    todo!() // cxx:__ZN7mjCBody12AddFreeJointEv
}

/// C: mjCBody::AddGeom (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:526)
/// Calls: cxx:__ZN8mjCModel13MakeTreeListsEP7mjCBody, cxx:__ZN8mjCModel14ResetTreeListsEv, cxx:__ZN8mjCModel9SignatureEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_add_geom(self_ptr: *mut mjCBody, arg0: *mut mjCDef) -> *mut mjCGeom {
    todo!() // cxx:__ZN7mjCBody7AddGeomEP6mjCDef
}

/// C: mjCBody::AddSite (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:527)
/// Calls: cxx:__ZN8mjCModel13MakeTreeListsEP7mjCBody, cxx:__ZN8mjCModel14ResetTreeListsEv, cxx:__ZN8mjCModel9SignatureEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_add_site(self_ptr: *mut mjCBody, arg0: *mut mjCDef) -> *mut mjCSite {
    todo!() // cxx:__ZN7mjCBody7AddSiteEP6mjCDef
}

/// C: mjCBody::AddCamera (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:528)
/// Calls: cxx:__ZN8mjCModel13MakeTreeListsEP7mjCBody, cxx:__ZN8mjCModel14ResetTreeListsEv, cxx:__ZN8mjCModel9SignatureEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_add_camera(self_ptr: *mut mjCBody, arg0: *mut mjCDef) -> *mut mjCCamera {
    todo!() // cxx:__ZN7mjCBody9AddCameraEP6mjCDef
}

/// C: mjCBody::AddLight (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:529)
/// Calls: cxx:__ZN8mjCModel13MakeTreeListsEP7mjCBody, cxx:__ZN8mjCModel14ResetTreeListsEv, cxx:__ZN8mjCModel9SignatureEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_add_light(self_ptr: *mut mjCBody, arg0: *mut mjCDef) -> *mut mjCLight {
    todo!() // cxx:__ZN7mjCBody8AddLightEP6mjCDef
}

/// C: mjCBody::NumObjects (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:537)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_num_objects(self_ptr: *mut mjCBody, r#type: u32) -> i32 {
    todo!() // cxx:__ZN7mjCBody10NumObjectsE6mjtObj
}

/// C: mjCBody::GetObject (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:538)
/// Calls: cxx:__ZN7mjCBody10NumObjectsE6mjtObj
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_get_object(self_ptr: *mut mjCBody, r#type: u32, id: i32) -> *mut mjCBase {
    todo!() // cxx:__ZN7mjCBody9GetObjectE6mjtObji
}

/// C: mjCBody::FindObject (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:539)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_find_object(self_ptr: *mut mjCBody, r#type: u32, name: *const std__string, recursive: bool) -> *mut mjCBase {
    todo!() // cxx:__ZNK7mjCBody10FindObjectE6mjtObjRKNSt3__112basic_stringIcNS1_11char_traitsIcEENS1_9allocatorIcEEEEb
}

/// C: mjCBody::NameSpace (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:542)
/// Calls: cxx:__ZN7mjCBody10NameSpace_EPK8mjCModelb
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_name_space__77a7259720d03cd3(self_ptr: *mut mjCBody, m: *const mjCModel) {
    todo!() // cxx:__ZN7mjCBody9NameSpaceEPK8mjCModel
}

/// C: mjCBody::MakeInertialExplicit (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:545)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_make_inertial_explicit(self_ptr: *mut mjCBody) {
    todo!() // cxx:__ZN7mjCBody20MakeInertialExplicitEv
}

/// C: mjCBody::ComputeBVH (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:548)
/// Calls: cxx:__ZN26mjCBoundingVolumeHierarchy23AllocateBoundingVolumesEi, cxx:__ZN26mjCBoundingVolumeHierarchy3SetEPdS0_, cxx:__ZN26mjCBoundingVolumeHierarchy9CreateBVHEP8mjCModelPK7mjCBase
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_compute_bvh(self_ptr: *mut mjCBody) {
    todo!() // cxx:__ZN7mjCBody10ComputeBVHEv
}

/// C: mjCBody::get_userdata (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:557)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_get_userdata(self_ptr: *mut mjCBody) -> *const () {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).userdata_ }
}

/// C: mjCBody::NextChild (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:563)
/// Calls: cxx-internal:user_objects.cc.o:__ZL11GetNextBodyPK7mjCBodyPK11mjsElement_Pbb
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_next_child(self_ptr: *mut mjCBody, child: *const mjsElement, r#type: u32, recursive: bool) -> *mut mjsElement {
    todo!() // cxx:__ZNK7mjCBody9NextChildEPK11mjsElement_6mjtObjb
}

/// C: mjCBody::ForgetKeyframes (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:567)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_forget_keyframes(self_ptr: *mut mjCBody) {
    todo!() // cxx:__ZNK7mjCBody15ForgetKeyframesEv
}

/// C: mjCBody::ToFrame (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:570)
/// Calls: cxx:__ZN7mjCBody17AccumulateInertiaEPK8mjsBody_PS0_, cxx:__ZN7mjCBody20MakeInertialExplicitEv, cxx:__ZN7mjCBody8AddFrameEP8mjCFrame, cxx:__ZN8mjCModel13MakeTreeListsEP7mjCBody, cxx:__ZN8mjCModel14ResetTreeListsEv, cxx:__ZN8mjCModel9SignatureEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_to_frame(self_ptr: *mut mjCBody) -> *mut mjCFrame {
    todo!() // cxx:__ZN7mjCBody7ToFrameEv
}

/// C: mjCBody::mpos (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:573)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_mpos(self_ptr: *mut mjCBody, state_name: *const std__string) -> *mut f64 {
    todo!() // cxx:__ZN7mjCBody4mposERKNSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEE
}

/// C: mjCBody::mquat (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:574)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_mquat(self_ptr: *mut mjCBody, state_name: *const std__string) -> *mut f64 {
    todo!() // cxx:__ZN7mjCBody5mquatERKNSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEE
}

/// C: mjCBody::SetParent (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:579)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_set_parent(self_ptr: *mut mjCBody, _body: *mut mjCBody) {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).parent = _body; }
}

/// C: mjCBody::GetParent (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:580)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_get_parent(self_ptr: *mut mjCBody) -> *mut mjCBody {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).parent }
}

/// C: mjCBody::SetModel (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:583)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_set_model(self_ptr: *mut mjCBody, _model: *mut mjCModel) {
    todo!() // cxx:__ZN7mjCBody8SetModelEP8mjCModel
}

/// C: mjCBody::ResetId (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:586)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_reset_id(self_ptr: *mut mjCBody) {
    todo!() // cxx:__ZN7mjCBody7ResetIdEv
}

/// C: mjCBody::Bodies (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:589)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_bodies(self_ptr: *mut mjCBody) -> *const () {
    todo!() // cxx:__ZNK7mjCBody6BodiesEv
}

/// C: mjCBody::AccumulateInertia (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:597)
/// Calls: cxx:__Z15mjuu_frameaccumPdS_PKdS1_, cxx:__Z16mjuu_fullInertiaPdS_PKd, cxx:__Z18mjuu_globalinertiaPdPKdS1_
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_accumulate_inertia(self_ptr: *mut mjCBody, other: *const mjsBody, result: *mut mjsBody) {
    todo!() // cxx:__ZN7mjCBody17AccumulateInertiaEPK8mjsBody_PS0_
}

/// C: mjCBody::Compile (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:603)
/// Calls: cxx:__Z12mjuu_definedd, cxx:__Z12mjuu_mulquatPdPKdS1_, cxx:__Z13mjuu_addtovecPdPKdi, cxx:__Z15mjuu_frameaccumPdS_PKdS1_, cxx:__Z15mjuu_rotVecQuatPdPKdS1_, cxx:__Z16mjuu_frameinvertPdS_PKdS1_, cxx:__Z16mjuu_fullInertiaPdS_PKd, cxx:__Z18ResolveOrientationPdbPKcRK15mjsOrientation_, cxx:__Z20mjuu_frameaccumChildPKdS0_PdS1_, cxx:__ZN7mjCBody10ComputeBVHEv, cxx:__ZN7mjCBody12CopyFromSpecEv, cxx:__ZN7mjCBody15InertiaFromGeomEv, cxx:__ZN7mjCGeom7CompileEv, cxx:__ZN7mjCSite7CompileEv, cxx:__ZN8mjCFrame7CompileEv, cxx:__ZN8mjCJoint7CompileEv, cxx:__ZN8mjCLight7CompileEv, cxx:__ZN8mjCModel13ResolvePluginEP7mjCBaseRKNSt3__112basic_stringIcNS2_11char_traitsIcEENS2_9allocatorIcEEEESA_PP9mjCPlugin, cxx:__ZN9mjCCamera7CompileEv, cxx:_mjp_getPluginAtSlot
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_compile(self_ptr: *mut mjCBody) {
    todo!() // cxx:__ZN7mjCBody7CompileEv
}

/// C: mjCBody::InertiaFromGeom (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:604)
/// Calls: cxx:__Z16mjuu_fullInertiaPdS_PKd, cxx:__Z18mjuu_globalinertiaPdPKdS1_
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_inertia_from_geom(self_ptr: *mut mjCBody) {
    todo!() // cxx:__ZN7mjCBody15InertiaFromGeomEv
}

/// C: mjCBody::CopyFromSpec (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:615)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_copy_from_spec(self_ptr: *mut mjCBody) {
    todo!() // cxx:__ZN7mjCBody12CopyFromSpecEv
}

/// C: mjCBody::PointToLocal (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:616)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_point_to_local(self_ptr: *mut mjCBody) {
    todo!() // cxx:__ZN7mjCBody12PointToLocalEv
}

/// C: mjCBody::NameSpace_ (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:617)
/// Calls: cxx:__ZN7mjCBase9NameSpaceEPK8mjCModel, cxx:__ZN7mjCGeom9NameSpaceEPK8mjCModel, cxx:__ZN7mjCSite9NameSpaceEPK8mjCModel, cxx:__ZN8mjCLight9NameSpaceEPK8mjCModel, cxx:__ZN9mjCCamera9NameSpaceEPK8mjCModel
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_name_space__2b35bbc8767711c4(self_ptr: *mut mjCBody, m: *const mjCModel, propagate: bool) {
    todo!() // cxx:__ZN7mjCBody10NameSpace_EPK8mjCModelb
}

/// C: mjCBody::CopyPlugin (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:618)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_copy_plugin(self_ptr: *mut mjCBody) {
    todo!() // cxx:__ZN7mjCBody10CopyPluginEv
}

/// C: mjCFrame::CopyFromSpec (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:654)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_frame_copy_from_spec(self_ptr: *mut mjCFrame) {
    todo!() // cxx:__ZN8mjCFrame12CopyFromSpecEv
}

/// C: mjCFrame::PointToLocal (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:655)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_frame_point_to_local(self_ptr: *mut mjCFrame) {
    todo!() // cxx:__ZN8mjCFrame12PointToLocalEv
}

/// C: mjCFrame::SetParent (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:656)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_frame_set_parent(self_ptr: *mut mjCFrame, _body: *mut mjCBody) {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).body = _body; }
}

/// C: mjCFrame::GetParent (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:657)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_frame_get_parent(self_ptr: *mut mjCFrame) -> *mut mjCBody {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).body }
}

/// C: mjCFrame::IsAncestor (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:661)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_frame_is_ancestor(self_ptr: *mut mjCFrame, child: *const mjCFrame) -> bool {
    todo!() // cxx:__ZNK8mjCFrame10IsAncestorEPKS_
}

/// C: mjCFrame::Compile (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:666)
/// Calls: cxx:__Z18ResolveOrientationPdbPKcRK15mjsOrientation_, cxx:__Z20mjuu_frameaccumChildPKdS0_PdS1_, cxx:__ZN8mjCFrame12CopyFromSpecEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_frame_compile(self_ptr: *mut mjCFrame) {
    todo!() // cxx:__ZN8mjCFrame7CompileEv
}

/// C: mjCJoint::CopyFromSpec (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:706)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_joint_copy_from_spec(self_ptr: *mut mjCJoint) {
    todo!() // cxx:__ZN8mjCJoint12CopyFromSpecEv
}

/// C: mjCJoint::SetParent (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:707)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_joint_set_parent(self_ptr: *mut mjCJoint, _body: *mut mjCBody) {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).body = _body; }
}

/// C: mjCJoint::GetParent (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:708)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_joint_get_parent(self_ptr: *mut mjCJoint) -> *mut mjCBody {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).body }
}

/// C: mjCJoint::get_userdata (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:711)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_joint_get_userdata(self_ptr: *mut mjCJoint) -> *const () {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).userdata_ }
}

/// C: mjCJoint::get_range (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:712)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_joint_get_range(self_ptr: *mut mjCJoint) -> *const f64 {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { core::ptr::addr_of!((*self_ptr).range).cast::<f64>() }
}

/// C: mjCJoint::is_limited (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:714)
/// Calls: cxx-internal:user_objects.cc.o:__ZL9islimitediPKd
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_joint_is_limited(self_ptr: *mut mjCJoint) -> bool {
    todo!() // cxx:__ZNK8mjCJoint10is_limitedEv
}

/// C: mjCJoint::is_actfrclimited (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:715)
/// Calls: cxx-internal:user_objects.cc.o:__ZL9islimitediPKd
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_joint_is_actfrclimited(self_ptr: *mut mjCJoint) -> bool {
    todo!() // cxx:__ZNK8mjCJoint16is_actfrclimitedEv
}

/// C: mjCJoint::nq (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:717)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_joint_nq__6edc07bc8acead00(joint_type: u32) -> i32 {
    todo!() // cxx:__ZN8mjCJoint2nqE8mjtJoint
}

/// C: mjCJoint::nv (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:718)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_joint_nv__19436c073d3cf9fa(joint_type: u32) -> i32 {
    todo!() // cxx:__ZN8mjCJoint2nvE8mjtJoint
}

/// C: mjCJoint::nq (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:719)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_joint_nq__e7b2a6296d35ace9(self_ptr: *mut mjCJoint) -> i32 {
    todo!() // cxx:__ZNK8mjCJoint2nqEv
}

/// C: mjCJoint::nv (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:720)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_joint_nv__5bcf64a4f65e6fbe(self_ptr: *mut mjCJoint) -> i32 {
    todo!() // cxx:__ZNK8mjCJoint2nvEv
}

/// C: mjCJoint::qpos (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:722)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_joint_qpos(self_ptr: *mut mjCJoint, state_name: *const std__string) -> *mut f64 {
    todo!() // cxx:__ZN8mjCJoint4qposERKNSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEE
}

/// C: mjCJoint::qvel (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:723)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_joint_qvel(self_ptr: *mut mjCJoint, state_name: *const std__string) -> *mut f64 {
    todo!() // cxx:__ZN8mjCJoint4qvelERKNSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEE
}

/// C: mjCJoint::Compile (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:726)
/// Calls: cxx-internal:user_objects.cc.o:__ZL12checklimitedPK7mjCBasebPKcS3_ib, cxx:__Z15mjuu_rotVecQuatPdPKdS1_, cxx:__Z20mjuu_frameaccumChildPKdS0_PdS1_, cxx:__ZN8mjCJoint12CopyFromSpecEv, cxx:__ZNK8mjCJoint10is_limitedEv, cxx:__ZNK8mjCJoint16is_actfrclimitedEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_joint_compile(self_ptr: *mut mjCJoint) -> i32 {
    todo!() // cxx:__ZN8mjCJoint7CompileEv
}

/// C: mjCJoint::PointToLocal (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:727)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_joint_point_to_local(self_ptr: *mut mjCJoint) {
    todo!() // cxx:__ZN8mjCJoint12PointToLocalEv
}

/// C: mjCGeom::GetVolume (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:783)
/// Calls: cxx:__ZNK7mjCMesh12GetVolumeRefEv, cxx:__ZNK8mjCModel6MeshesEv
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_geom_get_volume(self_ptr: *mut mjCGeom) -> f64 {
    todo!() // cxx:__ZNK7mjCGeom9GetVolumeEv
}

/// C: mjCGeom::SetInertia (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:784)
/// Calls: cxx:__ZN7mjCMesh16GetInertiaBoxPtrEv, cxx:__ZNK8mjCModel6MeshesEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_geom_set_inertia(self_ptr: *mut mjCGeom) {
    todo!() // cxx:__ZN7mjCGeom10SetInertiaEv
}

/// C: mjCGeom::IsVisual (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:785)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_geom_is_visual(self_ptr: *mut mjCGeom) -> bool {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).visual_ }
}

/// C: mjCGeom::SetNotVisual (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:786)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_geom_set_not_visual(self_ptr: *mut mjCGeom) {
    todo!() // cxx:__ZN7mjCGeom12SetNotVisualEv
}

/// C: mjCGeom::SetParent (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:787)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_geom_set_parent(self_ptr: *mut mjCGeom, _body: *mut mjCBody) {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).body = _body; }
}

/// C: mjCGeom::GetParent (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:788)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_geom_get_parent(self_ptr: *mut mjCGeom) -> *mut mjCBody {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).body }
}

/// C: mjCGeom::Type (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:789)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_geom_type(self_ptr: *mut mjCGeom) -> u32 {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).r#type }
}

/// C: mjCGeom::SetFluidCoefs (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:792)
/// Calls: cxx:__ZN7mjCGeom17GetAddedMassKappaEddd, cxx:_writeFluidGeomInteraction
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_geom_set_fluid_coefs(self_ptr: *mut mjCGeom) {
    todo!() // cxx:__ZN7mjCGeom13SetFluidCoefsEv
}

/// C: mjCGeom::GetAddedMassKappa (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:794)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_geom_get_added_mass_kappa(self_ptr: *mut mjCGeom, dx: f64, dy: f64, dz: f64) -> f64 {
    todo!() // cxx:__ZN7mjCGeom17GetAddedMassKappaEddd
}

/// C: mjCGeom::get_userdata (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:797)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_geom_get_userdata(self_ptr: *mut mjCGeom) -> *const () {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).userdata_ }
}

/// C: mjCGeom::get_hfieldname (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:798)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_geom_get_hfieldname(self_ptr: *mut mjCGeom) -> *const std__string {
    todo!() // cxx:__ZNK7mjCGeom14get_hfieldnameEv
}

/// C: mjCGeom::get_meshname (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:799)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_geom_get_meshname(self_ptr: *mut mjCGeom) -> *const std__string {
    todo!() // cxx:__ZNK7mjCGeom12get_meshnameEv
}

/// C: mjCGeom::get_material (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:800)
/// Calls: cxx:__ZNK7mjCMesh8MaterialEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_geom_get_material(self_ptr: *mut mjCGeom) -> *const std__string {
    todo!() // cxx:__ZNK7mjCGeom12get_materialEv
}

/// C: mjCGeom::del_material (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:801)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_geom_del_material(self_ptr: *mut mjCGeom) {
    todo!() // cxx:__ZN7mjCGeom12del_materialEv
}

/// C: mjCGeom::Compile (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:804)
/// Calls: cxx-internal:user_objects.cc.o:__ZL9checksizePd7mjtGeomP7mjCBasePKci, cxx:__Z11mjuu_z2quatPdPKd, cxx:__Z12mjuu_definedd, cxx:__Z13mjuu_addtovecPdPKdi, cxx:__Z15mjuu_frameaccumPdS_PKdS1_, cxx:__Z15mjuu_rotVecQuatPdPKdS1_, cxx:__Z18ResolveOrientationPdbPKcRK15mjsOrientation_, cxx:__Z20mjuu_frameaccumChildPKdS0_PdS1_, cxx:__ZN7mjCGeom10SetInertiaEv, cxx:__ZN7mjCGeom11ComputeAABBEv, cxx:__ZN7mjCGeom12CopyFromSpecEv, cxx:__ZN7mjCGeom13SetFluidCoefsEv, cxx:__ZN7mjCMesh10GetQuatPtrEv, cxx:__ZN7mjCMesh7FitGeomEP7mjCGeomPd, cxx:__ZN7mjCMesh9GetPosPtrEv, cxx:__ZN8mjCModel13ResolvePluginEP7mjCBaseRKNSt3__112basic_stringIcNS2_11char_traitsIcEENS2_9allocatorIcEEEESA_PP9mjCPlugin, cxx:__ZNK7mjCGeom9GetVolumeEv, cxx:__ZNK7mjCMesh4aammEv, cxx:_mjp_getPluginAtSlot
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_geom_compile(self_ptr: *mut mjCGeom) {
    todo!() // cxx:__ZN7mjCGeom7CompileEv
}

/// C: mjCGeom::GetRBound (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:805)
/// Calls: cxx:__ZNK7mjCMesh4aammEv
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_geom_get_r_bound(self_ptr: *mut mjCGeom) -> f64 {
    todo!() // cxx:__ZN7mjCGeom9GetRBoundEv
}

/// C: mjCGeom::ComputeAABB (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:806)
/// Calls: cxx:__ZNK7mjCMesh4aammEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_geom_compute_aabb(self_ptr: *mut mjCGeom) {
    todo!() // cxx:__ZN7mjCGeom11ComputeAABBEv
}

/// C: mjCGeom::CopyFromSpec (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:807)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_geom_copy_from_spec(self_ptr: *mut mjCGeom) {
    todo!() // cxx:__ZN7mjCGeom12CopyFromSpecEv
}

/// C: mjCGeom::PointToLocal (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:808)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_geom_point_to_local(self_ptr: *mut mjCGeom) {
    todo!() // cxx:__ZN7mjCGeom12PointToLocalEv
}

/// C: mjCGeom::NameSpace (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:809)
/// Calls: cxx:__ZN7mjCBase9NameSpaceEPK8mjCModel
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_geom_name_space(self_ptr: *mut mjCGeom, m: *const mjCModel) {
    todo!() // cxx:__ZN7mjCGeom9NameSpaceEPK8mjCModel
}

/// C: mjCGeom::CopyPlugin (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:810)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_geom_copy_plugin(self_ptr: *mut mjCGeom) {
    todo!() // cxx:__ZN7mjCGeom10CopyPluginEv
}

/// C: mjCSite::Body (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:849)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_site_body(self_ptr: *mut mjCSite) -> *mut mjCBody {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).body }
}

/// C: mjCSite::SetParent (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:850)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_site_set_parent(self_ptr: *mut mjCSite, _body: *mut mjCBody) {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).body = _body; }
}

/// C: mjCSite::GetParent (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:851)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_site_get_parent(self_ptr: *mut mjCSite) -> *mut mjCBody {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).body }
}

/// C: mjCSite::get_userdata (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:857)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_site_get_userdata(self_ptr: *mut mjCSite) -> *const () {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).userdata_ }
}

/// C: mjCSite::get_material (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:858)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_site_get_material(self_ptr: *mut mjCSite) -> *const std__string {
    todo!() // cxx:__ZNK7mjCSite12get_materialEv
}

/// C: mjCSite::del_material (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:859)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_site_del_material(self_ptr: *mut mjCSite) {
    todo!() // cxx:__ZN7mjCSite12del_materialEv
}

/// C: mjCSite::Compile (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:862)
/// Calls: cxx-internal:user_objects.cc.o:__ZL9checksizePd7mjtGeomP7mjCBasePKci, cxx:__Z11mjuu_z2quatPdPKd, cxx:__Z12mjuu_definedd, cxx:__Z18ResolveOrientationPdbPKcRK15mjsOrientation_, cxx:__Z20mjuu_frameaccumChildPKdS0_PdS1_, cxx:__ZN7mjCSite12CopyFromSpecEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_site_compile(self_ptr: *mut mjCSite) {
    todo!() // cxx:__ZN7mjCSite7CompileEv
}

/// C: mjCSite::CopyFromSpec (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:863)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_site_copy_from_spec(self_ptr: *mut mjCSite) {
    todo!() // cxx:__ZN7mjCSite12CopyFromSpecEv
}

/// C: mjCSite::PointToLocal (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:864)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_site_point_to_local(self_ptr: *mut mjCSite) {
    todo!() // cxx:__ZN7mjCSite12PointToLocalEv
}

/// C: mjCSite::NameSpace (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:865)
/// Calls: cxx:__ZN7mjCBase9NameSpaceEPK8mjCModel
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_site_name_space(self_ptr: *mut mjCSite, m: *const mjCModel) {
    todo!() // cxx:__ZN7mjCSite9NameSpaceEPK8mjCModel
}

/// C: mjCCamera::get_targetbody (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:899)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_camera_get_targetbody(self_ptr: *mut mjCCamera) -> *const std__string {
    todo!() // cxx:__ZNK9mjCCamera14get_targetbodyEv
}

/// C: mjCCamera::get_userdata (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:900)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_camera_get_userdata(self_ptr: *mut mjCCamera) -> *const () {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).userdata_ }
}

/// C: mjCCamera::SetParent (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:902)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_camera_set_parent(self_ptr: *mut mjCCamera, _body: *mut mjCBody) {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).body = _body; }
}

/// C: mjCCamera::GetParent (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:903)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_camera_get_parent(self_ptr: *mut mjCCamera) -> *mut mjCBody {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).body }
}

/// C: mjCCamera::Compile (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:906)
/// Calls: cxx:__Z18ResolveOrientationPdbPKcRK15mjsOrientation_, cxx:__Z20mjuu_frameaccumChildPKdS0_PdS1_, cxx:__ZN9mjCCamera12CopyFromSpecEv, cxx:__ZN9mjCCamera17ResolveReferencesEPK8mjCModel
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_camera_compile(self_ptr: *mut mjCCamera) {
    todo!() // cxx:__ZN9mjCCamera7CompileEv
}

/// C: mjCCamera::CopyFromSpec (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:907)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_camera_copy_from_spec(self_ptr: *mut mjCCamera) {
    todo!() // cxx:__ZN9mjCCamera12CopyFromSpecEv
}

/// C: mjCCamera::PointToLocal (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:908)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_camera_point_to_local(self_ptr: *mut mjCCamera) {
    todo!() // cxx:__ZN9mjCCamera12PointToLocalEv
}

/// C: mjCCamera::NameSpace (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:909)
/// Calls: cxx:__ZN7mjCBase9NameSpaceEPK8mjCModel
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_camera_name_space(self_ptr: *mut mjCCamera, m: *const mjCModel) {
    todo!() // cxx:__ZN9mjCCamera9NameSpaceEPK8mjCModel
}

/// C: mjCCamera::ResolveReferences (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:910)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_camera_resolve_references(self_ptr: *mut mjCCamera, m: *const mjCModel) {
    todo!() // cxx:__ZN9mjCCamera17ResolveReferencesEPK8mjCModel
}

/// C: mjCLight::get_targetbody (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:944)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_light_get_targetbody(self_ptr: *mut mjCLight) -> *const std__string {
    todo!() // cxx:__ZNK8mjCLight14get_targetbodyEv
}

/// C: mjCLight::get_texture (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:945)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_light_get_texture(self_ptr: *mut mjCLight) -> *const std__string {
    todo!() // cxx:__ZNK8mjCLight11get_textureEv
}

/// C: mjCLight::SetParent (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:947)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_light_set_parent(self_ptr: *mut mjCLight, _body: *mut mjCBody) {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).body = _body; }
}

/// C: mjCLight::GetParent (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:948)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_light_get_parent(self_ptr: *mut mjCLight) -> *mut mjCBody {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).body }
}

/// C: mjCLight::Compile (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:951)
/// Calls: cxx:__Z15mjuu_rotVecQuatPdPKdS1_, cxx:__Z20mjuu_frameaccumChildPKdS0_PdS1_, cxx:__ZN8mjCLight12CopyFromSpecEv, cxx:__ZN8mjCLight17ResolveReferencesEPK8mjCModel
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_light_compile(self_ptr: *mut mjCLight) {
    todo!() // cxx:__ZN8mjCLight7CompileEv
}

/// C: mjCLight::CopyFromSpec (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:952)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_light_copy_from_spec(self_ptr: *mut mjCLight) {
    todo!() // cxx:__ZN8mjCLight12CopyFromSpecEv
}

/// C: mjCLight::PointToLocal (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:953)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_light_point_to_local(self_ptr: *mut mjCLight) {
    todo!() // cxx:__ZN8mjCLight12PointToLocalEv
}

/// C: mjCLight::NameSpace (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:954)
/// Calls: cxx:__ZN7mjCBase9NameSpaceEPK8mjCModel
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_light_name_space(self_ptr: *mut mjCLight, m: *const mjCModel) {
    todo!() // cxx:__ZN8mjCLight9NameSpaceEPK8mjCModel
}

/// C: mjCLight::ResolveReferences (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:955)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_light_resolve_references(self_ptr: *mut mjCLight, m: *const mjCModel) {
    todo!() // cxx:__ZN8mjCLight17ResolveReferencesEPK8mjCModel
}

/// C: mjCFlex::CopyFromSpec (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1032)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_copy_from_spec(self_ptr: *mut mjCFlex) {
    todo!() // cxx:__ZN7mjCFlex12CopyFromSpecEv
}

/// C: mjCFlex::PointToLocal (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1033)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_point_to_local(self_ptr: *mut mjCFlex) {
    todo!() // cxx:__ZN7mjCFlex12PointToLocalEv
}

/// C: mjCFlex::ResolveReferences (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1034)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_resolve_references(self_ptr: *mut mjCFlex, m: *const mjCModel) {
    todo!() // cxx:__ZN7mjCFlex17ResolveReferencesEPK8mjCModel
}

/// C: mjCFlex::NameSpace (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1035)
/// Calls: cxx:__ZN7mjCBase9NameSpaceEPK8mjCModel
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_name_space(self_ptr: *mut mjCFlex, m: *const mjCModel) {
    todo!() // cxx:__ZN7mjCFlex9NameSpaceEPK8mjCModel
}

/// C: mjCFlex::get_material (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1038)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_get_material(self_ptr: *mut mjCFlex) -> *const std__string {
    todo!() // cxx:__ZNK7mjCFlex12get_materialEv
}

/// C: mjCFlex::get_vertbody (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1039)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_get_vertbody(self_ptr: *mut mjCFlex) -> *const () {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).vertbody_ }
}

/// C: mjCFlex::get_vert (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1040)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_get_vert(self_ptr: *mut mjCFlex) -> *const () {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).vert_ }
}

/// C: mjCFlex::get_elemaabb (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1041)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_get_elemaabb(self_ptr: *mut mjCFlex) -> *const () {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).elemaabb_ }
}

/// C: mjCFlex::get_elem (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1042)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_get_elem(self_ptr: *mut mjCFlex) -> *const () {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).elem_ }
}

/// C: mjCFlex::get_texcoord (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1043)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_get_texcoord(self_ptr: *mut mjCFlex) -> *const () {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).texcoord_ }
}

/// C: mjCFlex::get_elemtexcoord (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1044)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_get_elemtexcoord(self_ptr: *mut mjCFlex) -> *const () {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).elemtexcoord_ }
}

/// C: mjCFlex::get_nodebody (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1045)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_get_nodebody(self_ptr: *mut mjCFlex) -> *const () {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).nodebody_ }
}

/// C: mjCFlex::HasTexcoord (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1047)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_has_texcoord(self_ptr: *mut mjCFlex) -> bool {
    todo!() // cxx:__ZNK7mjCFlex11HasTexcoordEv
}

/// C: mjCFlex::DelTexcoord (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1048)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_del_texcoord(self_ptr: *mut mjCFlex) {
    todo!() // cxx:__ZN7mjCFlex11DelTexcoordEv
}

/// C: mjCFlex::Compile (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1055)
/// Calls: cxx-internal:user_mesh.cc.o:__ZL15ComputeWarpModePdPKdiii, cxx-internal:user_mesh.cc.o:__ZL17CreateFlapStencilRNSt3__16vectorI11StencilFlapNS_9allocatorIS1_EEEERKNS0_IiNS2_IiEEEES9_, cxx-internal:user_mesh.cc.o:__ZL20ComputeInterpBendingRNSt3__16vectorIdNS_9allocatorIdEEEERKS3_iPKiddd, cxx-internal:user_mesh.cc.o:__ZL20ComputeWarpStiffnessPKdiiddd, cxx-internal:user_mesh.cc.o:__ZL23EigendecomposeStiffnessPKdPdi, cxx:__Z13mjuu_addtovecPdPKdi, cxx:__Z13mjuu_crossvecPdPKdS1_, cxx:__Z14ComputeBendingI11StencilFlapEvPdS1_PKidd, cxx:__Z14mjuu_mulvecmatPdPKdS1_, cxx:__Z15mjuu_rotVecQuatPdPKdS1_, cxx:__Z22ComputeLinearStiffnessRNSt3__16vectorIdNS_9allocatorIdEEEEPKdddi, cxx:__Z24ComputeLinearStiffness2DRNSt3__16vectorIdNS_9allocatorIdEEEEPKdddidi, cxx:__Z9mjuu_dot3PKdS0_, cxx:__ZN7mjCFlex12CopyFromSpecEv, cxx:__ZN7mjCFlex15CreateShellPairEv, cxx:__ZN7mjCFlex16ComputeCellEmptyEPKdPKiiiiS1_, cxx:__ZN7mjCFlex17ResolveReferencesEPK8mjCModel, cxx:__ZN7mjCFlex19LoadCachedStiffnessEv, cxx:__ZN7mjCFlex9CreateBVHEv, cxx:__ZNK26mjCBoundingVolumeHierarchy3BvhEv, cxx:__ZNK7mjCFlex29ComputeUnrotatedNodePositionsERKNSt3__16vectorIdNS0_9allocatorIdEEEEPd, cxx:__ZNK8mjCModel10EqualitiesEv, cxx:__ZNK8mjCModel6BodiesEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_compile(self_ptr: *mut mjCFlex, vfs: *const mjVFS) {
    todo!() // cxx:__ZN7mjCFlex7CompileEPK6mjVFS_
}

/// C: mjCFlex::CreateBVH (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1056)
/// Calls: cxx:__ZN26mjCBoundingVolumeHierarchy21RemoveInactiveVolumesEi, cxx:__ZN26mjCBoundingVolumeHierarchy23AllocateBoundingVolumesEi, cxx:__ZN26mjCBoundingVolumeHierarchy9CreateBVHEP8mjCModelPK7mjCBase
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_create_bvh(self_ptr: *mut mjCFlex) {
    todo!() // cxx:__ZN7mjCFlex9CreateBVHEv
}

/// C: mjCFlex::CreateShellPair (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1057)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_create_shell_pair(self_ptr: *mut mjCFlex) {
    todo!() // cxx:__ZN7mjCFlex15CreateShellPairEv
}

/// C: mjCFlex::ComputeCellEmpty (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1058)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_compute_cell_empty(self_ptr: *mut mjCFlex, vpos: *const f64, elems: *const i32, nv: i32, ne: i32, fdim: i32, bbox: *const f64) {
    todo!() // cxx:__ZN7mjCFlex16ComputeCellEmptyEPKdPKiiiiS1_
}

/// C: mjCFlex::ComputeUnrotatedNodePositions (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1067)
/// Calls: cxx:__Z14mjuu_mulvecmatPdPKdS1_, cxx:__Z9mjuu_dot3PKdS0_
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_compute_unrotated_node_positions(self_ptr: *mut mjCFlex, nodexpos: *const (), R0_out: *mut f64) -> *const () {
    todo!() // cxx:__ZNK7mjCFlex29ComputeUnrotatedNodePositionsERKNSt3__16vectorIdNS0_9allocatorIdEEEEPd
}

/// C: mjCFlex::ComputeStiffnessCacheKey (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1071)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_compute_stiffness_cache_key(self_ptr: *mut mjCFlex) -> std__string {
    todo!() // cxx:__ZNK7mjCFlex24ComputeStiffnessCacheKeyEv
}

/// C: mjCFlex::LoadCachedStiffness (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1072)
/// Calls: cxx:__ZN8mjCCache12PopulateDataERKNSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEEPK11mjResource_NS0_8functionIFbPKvEEE, cxx:__ZNK7mjCFlex24ComputeStiffnessCacheKeyEv, cxx:_mj_getCache
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_load_cached_stiffness(self_ptr: *mut mjCFlex) -> bool {
    todo!() // cxx:__ZN7mjCFlex19LoadCachedStiffnessEv
}

/// C: mjCFlex::CacheStiffness (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1073)
/// Calls: cxx:__ZN8mjCCache6InsertERKNSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEES8_PK11mjResource_NS0_10shared_ptrIKvEEm, cxx:__ZNK7mjCFlex24ComputeStiffnessCacheKeyEv, cxx:_mj_getCache
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_cache_stiffness(self_ptr: *mut mjCFlex) {
    todo!() // cxx:__ZN7mjCFlex14CacheStiffnessEv
}

/// C: mjCMesh::CopyFromSpec (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1151)
/// Calls: cxx:__Z13mjuu_stripextNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEE, cxx:__Z14mjuu_strippathNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEE, cxx:__ZN7mjCMesh15ProcessVerticesERKNSt3__16vectorIfNS0_9allocatorIfEEEEb, cxx:_mju_free
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_copy_from_spec(self_ptr: *mut mjCMesh) {
    todo!() // cxx:__ZN7mjCMesh12CopyFromSpecEv
}

/// C: mjCMesh::PointToLocal (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1152)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_point_to_local(self_ptr: *mut mjCMesh) {
    todo!() // cxx:__ZN7mjCMesh12PointToLocalEv
}

/// C: mjCMesh::NameSpace (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1153)
/// Calls: cxx:__Z13mjuu_stripextNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEE, cxx:__Z14mjuu_strippathNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEE, cxx:__ZN7mjCBase9NameSpaceEPK8mjCModel
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_name_space(self_ptr: *mut mjCMesh, m: *const mjCModel) {
    todo!() // cxx:__ZN7mjCMesh9NameSpaceEPK8mjCModel
}

/// C: mjCMesh::MakeHemisphere (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1156)
/// Calls: cxx:_mjs_setFloat, cxx:_mjs_setInt
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_make_hemisphere(self_ptr: *mut mjCMesh, res: i32, make_faces: bool, make_cap: bool) {
    todo!() // cxx:__ZN7mjCMesh14MakeHemisphereEibb
}

/// C: mjCMesh::MakeSphere (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1157)
/// Calls: cxx:_mjs_setFloat, cxx:_mjs_setInt
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_make_sphere(self_ptr: *mut mjCMesh, subdiv: i32, make_faces: bool) {
    todo!() // cxx:__ZN7mjCMesh10MakeSphereEib
}

/// C: mjCMesh::MakeTorus (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1158)
/// Calls: cxx:_mjs_setFloat, cxx:_mjs_setInt
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_make_torus(self_ptr: *mut mjCMesh, res: i32, radius: f64) {
    todo!() // cxx:__ZN7mjCMesh9MakeTorusEid
}

/// C: mjCMesh::MakeSupertorus (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1159)
/// Calls: cxx-internal:user_mesh.cc.o:__ZN12_GLOBAL__N_15aux_cEdd, cxx-internal:user_mesh.cc.o:__ZN12_GLOBAL__N_15aux_sEdd, cxx:_mjs_setFloat, cxx:_mjs_setInt
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_make_supertorus(self_ptr: *mut mjCMesh, res: i32, radius: f64, s: f64, t: f64) {
    todo!() // cxx:__ZN7mjCMesh14MakeSupertorusEiddd
}

/// C: mjCMesh::MakeSupersphere (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1160)
/// Calls: cxx-internal:user_mesh.cc.o:__ZN12_GLOBAL__N_15aux_cEdd, cxx-internal:user_mesh.cc.o:__ZN12_GLOBAL__N_15aux_sEdd, cxx:_mjs_setFloat, cxx:_mjs_setInt
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_make_supersphere(self_ptr: *mut mjCMesh, res: i32, e: f64, n: f64) {
    todo!() // cxx:__ZN7mjCMesh15MakeSupersphereEidd
}

/// C: mjCMesh::MakeWedge (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1161)
/// Calls: cxx-internal:user_mesh.cc.o:__ZN12_GLOBAL__N_112TangentFrameEPKdPf, cxx-internal:user_mesh.cc.o:__ZN12_GLOBAL__N_120SphericalToCartesianEPKdPf, cxx-internal:user_mesh.cc.o:__ZN12_GLOBAL__N_18BinEdgesEPdS0_PiS0_d, cxx:_mjs_setFloat
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_make_wedge(self_ptr: *mut mjCMesh, resolution: *mut i32, fov: *mut f64, gamma: f64) {
    todo!() // cxx:__ZN7mjCMesh9MakeWedgeEPiPdd
}

/// C: mjCMesh::MakeRect (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1162)
/// Calls: cxx-internal:user_mesh.cc.o:__ZN12_GLOBAL__N_18LinSpaceEddiPd, cxx:_mjs_setFloat, cxx:_mjs_setInt
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_make_rect(self_ptr: *mut mjCMesh, resolution: *mut i32) {
    todo!() // cxx:__ZN7mjCMesh8MakeRectEPi
}

/// C: mjCMesh::MakeCone (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1163)
/// Calls: cxx:_mjs_setFloat
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_make_cone(self_ptr: *mut mjCMesh, nedge: i32, radius: f64) {
    todo!() // cxx:__ZN7mjCMesh8MakeConeEid
}

/// C: mjCMesh::Plugin (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1166)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_plugin(self_ptr: *mut mjCMesh) -> *const mjsPlugin {
    todo!() // cxx:__ZNK7mjCMesh6PluginEv
}

/// C: mjCMesh::ContentType (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1167)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_content_type(self_ptr: *mut mjCMesh) -> *const std__string {
    todo!() // cxx:__ZNK7mjCMesh11ContentTypeEv
}

/// C: mjCMesh::File (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1168)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_file(self_ptr: *mut mjCMesh) -> *const std__string {
    todo!() // cxx:__ZNK7mjCMesh4FileEv
}

/// C: mjCMesh::Refpos (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1169)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_refpos(self_ptr: *mut mjCMesh) -> *const f64 {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { core::ptr::addr_of!((*self_ptr).refpos).cast::<f64>() }
}

/// C: mjCMesh::Refquat (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1170)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_refquat(self_ptr: *mut mjCMesh) -> *const f64 {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { core::ptr::addr_of!((*self_ptr).refquat).cast::<f64>() }
}

/// C: mjCMesh::Scale (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1171)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_scale(self_ptr: *mut mjCMesh) -> *const f64 {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { core::ptr::addr_of!((*self_ptr).scale).cast::<f64>() }
}

/// C: mjCMesh::SmoothNormal (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1172)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_smooth_normal(self_ptr: *mut mjCMesh) -> bool {
    todo!() // cxx:__ZNK7mjCMesh12SmoothNormalEv
}

/// C: mjCMesh::Vert (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1173)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_vert__9705cebee72c48c9(self_ptr: *mut mjCMesh) -> *const () {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).vert_ }
}

/// C: mjCMesh::Vert (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1174)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_vert__9d7db7f8a5d289af(self_ptr: *mut mjCMesh, i: i32) -> f32 {
    todo!() // cxx:__ZNK7mjCMesh4VertEi
}

/// C: mjCMesh::UserVert (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1175)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_user_vert(self_ptr: *mut mjCMesh) -> *const () {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).spec_vert_ }
}

/// C: mjCMesh::UserNormal (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1176)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_user_normal(self_ptr: *mut mjCMesh) -> *const () {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).spec_normal_ }
}

/// C: mjCMesh::Texcoord (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1177)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_texcoord(self_ptr: *mut mjCMesh) -> *const () {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).texcoord_ }
}

/// C: mjCMesh::FaceTexcoord (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1178)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_face_texcoord(self_ptr: *mut mjCMesh) -> *const () {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).facetexcoord_ }
}

/// C: mjCMesh::UserTexcoord (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1179)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_user_texcoord(self_ptr: *mut mjCMesh) -> *const () {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).spec_texcoord_ }
}

/// C: mjCMesh::Face (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1180)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_face(self_ptr: *mut mjCMesh) -> *const () {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).face_ }
}

/// C: mjCMesh::UserFace (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1181)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_user_face(self_ptr: *mut mjCMesh) -> *const () {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).spec_face_ }
}

/// C: mjCMesh::Inertia (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1182)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_inertia(self_ptr: *mut mjCMesh) -> u32 {
    todo!() // cxx:__ZNK7mjCMesh7InertiaEv
}

/// C: mjCMesh::Material (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1183)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_material(self_ptr: *mut mjCMesh) -> *const std__string {
    todo!() // cxx:__ZNK7mjCMesh8MaterialEv
}

/// C: mjCMesh::SetNeedHull (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1186)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_set_need_hull(self_ptr: *mut mjCMesh, needhull: bool) {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).needhull_ = needhull; }
}

/// C: mjCMesh::aamm (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1189)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_aamm(self_ptr: *mut mjCMesh) -> *const f64 {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { core::ptr::addr_of!((*self_ptr).aamm_).cast::<f64>() }
}

/// C: mjCMesh::nvert (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1192)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_nvert(self_ptr: *mut mjCMesh) -> i32 {
    todo!() // cxx:__ZNK7mjCMesh5nvertEv
}

/// C: mjCMesh::nnormal (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1193)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_nnormal(self_ptr: *mut mjCMesh) -> i32 {
    todo!() // cxx:__ZNK7mjCMesh7nnormalEv
}

/// C: mjCMesh::ntexcoord (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1194)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_ntexcoord(self_ptr: *mut mjCMesh) -> i32 {
    todo!() // cxx:__ZNK7mjCMesh9ntexcoordEv
}

/// C: mjCMesh::nface (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1195)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_nface(self_ptr: *mut mjCMesh) -> i32 {
    todo!() // cxx:__ZNK7mjCMesh5nfaceEv
}

/// C: mjCMesh::npolygon (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1196)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_npolygon(self_ptr: *mut mjCMesh) -> i32 {
    todo!() // cxx:__ZNK7mjCMesh8npolygonEv
}

/// C: mjCMesh::npolygonvert (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1197)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_npolygonvert(self_ptr: *mut mjCMesh) -> i32 {
    todo!() // cxx:__ZNK7mjCMesh12npolygonvertEv
}

/// C: mjCMesh::npolygonmap (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1204)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_npolygonmap(self_ptr: *mut mjCMesh) -> i32 {
    todo!() // cxx:__ZNK7mjCMesh11npolygonmapEv
}

/// C: mjCMesh::szgraph (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1213)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_szgraph(self_ptr: *mut mjCMesh) -> i32 {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).szgraph_ }
}

/// C: mjCMesh::tree (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1216)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_tree(self_ptr: *mut mjCMesh) -> *const mjCBoundingVolumeHierarchy {
    todo!() // cxx:__ZN7mjCMesh4treeEv
}

/// C: mjCMesh::octree (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1219)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_octree(self_ptr: *mut mjCMesh) -> *const mjCOctree {
    todo!() // cxx:__ZN7mjCMesh6octreeEv
}

/// C: mjCMesh::mutable_octree (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1220)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_mutable_octree(self_ptr: *mut mjCMesh) -> *mut mjCOctree {
    todo!() // cxx:__ZN7mjCMesh14mutable_octreeEv
}

/// C: mjCMesh::Compile (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1222)
/// Calls: cxx:__ZN7mjCMesh10TryCompileEPK6mjVFS_, cxx:_mju_closeResource
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_compile(self_ptr: *mut mjCMesh, vfs: *const mjVFS) {
    todo!() // cxx:__ZN7mjCMesh7CompileEPK6mjVFS_
}

/// C: mjCMesh::GetPosPtr (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1223)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_get_pos_ptr(self_ptr: *mut mjCMesh) -> *mut f64 {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { core::ptr::addr_of_mut!((*self_ptr).pos_).cast::<f64>() }
}

/// C: mjCMesh::GetQuatPtr (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1224)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_get_quat_ptr(self_ptr: *mut mjCMesh) -> *mut f64 {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { core::ptr::addr_of_mut!((*self_ptr).quat_).cast::<f64>() }
}

/// C: mjCMesh::GetInertiaBoxPtr (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1225)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_get_inertia_box_ptr(self_ptr: *mut mjCMesh) -> *mut f64 {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { core::ptr::addr_of_mut!((*self_ptr).boxsz_).cast::<f64>() }
}

/// C: mjCMesh::GetVolumeRef (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1226)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_get_volume_ref(self_ptr: *mut mjCMesh) -> f64 {
    todo!() // cxx:__ZNK7mjCMesh12GetVolumeRefEv
}

/// C: mjCMesh::FitGeom (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1227)
/// Calls: cxx:__ZN7mjCMesh16GetInertiaBoxPtrEv
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_fit_geom(self_ptr: *mut mjCMesh, geom: *mut mjCGeom, center: *mut f64) {
    todo!() // cxx:__ZN7mjCMesh7FitGeomEP7mjCGeomPd
}

/// C: mjCMesh::HasTexcoord (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1228)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_has_texcoord(self_ptr: *mut mjCMesh) -> bool {
    todo!() // cxx:__ZNK7mjCMesh11HasTexcoordEv
}

/// C: mjCMesh::DelTexcoord (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1229)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_del_texcoord(self_ptr: *mut mjCMesh) {
    todo!() // cxx:__ZN7mjCMesh11DelTexcoordEv
}

/// C: mjCMesh::IsVisual (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1230)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_is_visual(self_ptr: *mut mjCMesh) -> bool {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).visual_ }
}

/// C: mjCMesh::SetNotVisual (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1231)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_set_not_visual(self_ptr: *mut mjCMesh) {
    todo!() // cxx:__ZN7mjCMesh12SetNotVisualEv
}

/// C: mjCMesh::CopyVert (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1233)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_copy_vert(self_ptr: *mut mjCMesh, arr: *mut f32) {
    todo!() // cxx:__ZNK7mjCMesh8CopyVertEPf
}

/// C: mjCMesh::CopyNormal (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1234)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_copy_normal(self_ptr: *mut mjCMesh, arr: *mut f32) {
    todo!() // cxx:__ZNK7mjCMesh10CopyNormalEPf
}

/// C: mjCMesh::CopyFace (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1235)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_copy_face(self_ptr: *mut mjCMesh, arr: *mut i32) {
    todo!() // cxx:__ZNK7mjCMesh8CopyFaceEPi
}

/// C: mjCMesh::CopyFaceNormal (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1236)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_copy_face_normal(self_ptr: *mut mjCMesh, arr: *mut i32) {
    todo!() // cxx:__ZNK7mjCMesh14CopyFaceNormalEPi
}

/// C: mjCMesh::CopyFaceTexcoord (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1237)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_copy_face_texcoord(self_ptr: *mut mjCMesh, arr: *mut i32) {
    todo!() // cxx:__ZNK7mjCMesh16CopyFaceTexcoordEPi
}

/// C: mjCMesh::CopyTexcoord (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1238)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_copy_texcoord(self_ptr: *mut mjCMesh, arr: *mut f32) {
    todo!() // cxx:__ZNK7mjCMesh12CopyTexcoordEPf
}

/// C: mjCMesh::CopyGraph (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1239)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_copy_graph__7fa6fcbf55e93b6d(self_ptr: *mut mjCMesh, arr: *mut i32) {
    todo!() // cxx:__ZNK7mjCMesh9CopyGraphEPi
}

/// C: mjCMesh::CopyPolygons (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1242)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_copy_polygons(self_ptr: *mut mjCMesh, verts: *mut i32, adr: *mut i32, num: *mut i32, poly_adr: i32) {
    todo!() // cxx:__ZNK7mjCMesh12CopyPolygonsEPiS0_S0_i
}

/// C: mjCMesh::CopyPolygonMap (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1245)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_copy_polygon_map(self_ptr: *mut mjCMesh, faces: *mut i32, adr: *mut i32, num: *mut i32, poly_adr: i32) {
    todo!() // cxx:__ZNK7mjCMesh14CopyPolygonMapEPiS0_S0_i
}

/// C: mjCMesh::CopyPolygonNormals (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1248)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_copy_polygon_normals(self_ptr: *mut mjCMesh, arr: *mut f64) {
    todo!() // cxx:__ZN7mjCMesh18CopyPolygonNormalsEPd
}

/// C: mjCMesh::SetBoundingVolume (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1251)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_set_bounding_volume(self_ptr: *mut mjCMesh, faceid: i32, dvert: *const f64) {
    todo!() // cxx:__ZN7mjCMesh17SetBoundingVolumeEiPKd
}

/// C: mjCMesh::LoadFromResource (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1254)
/// Calls: cxx:__ZN7mjCBase19GetAssetContentTypeENSt3__117basic_string_viewIcNS0_11char_traitsIcEEEES4_, cxx:__ZN7mjCMesh15LoadFromDecoderEP11mjResource_b, cxx:__ZN7mjCMesh7LoadMSHEP11mjResource_b
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_load_from_resource(self_ptr: *mut mjCMesh, resource: *mut mjResource, remove_repeated: bool) {
    todo!() // cxx:__ZN7mjCMesh16LoadFromResourceEP11mjResource_b
}

/// C: mjCMesh::IsMSH (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1258)
/// Calls: cxx:__ZN7mjCBase19GetAssetContentTypeENSt3__117basic_string_viewIcNS0_11char_traitsIcEEEES4_
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_is_msh__8a61bee78e2aafea(filename: std__string_view, ct: std__string_view) -> bool {
    todo!() // cxx:__ZN7mjCMesh5IsMSHENSt3__117basic_string_viewIcNS0_11char_traitsIcEEEES4_
}

/// C: mjCMesh::IsMSH (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1262)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_is_msh__dc9ef7fb9cd3bbba(self_ptr: *mut mjCMesh) -> bool {
    todo!() // cxx:__ZNK7mjCMesh5IsMSHEv
}

/// C: mjCMesh::TryCompile (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1265)
/// Calls: cxx:__Z14mjuu_strippathNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEE, cxx:__ZN7mjCBase12LoadResourceERKNSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEES8_PK6mjVFS_, cxx:__ZN7mjCMesh12CopyFromSpecEv, cxx:__ZN7mjCMesh14LoadCachedMeshEP8mjCCachePK11mjResource_, cxx:__ZN7mjCMesh15ProcessVerticesERKNSt3__16vectorIfNS0_9allocatorIfEEEEb, cxx:__ZN7mjCMesh16LoadFromResourceEP11mjResource_b, cxx:__ZN7mjCMesh7LoadSDFEv, cxx:__ZN7mjCMesh7ProcessEv, cxx:__ZN7mjCMesh9CacheMeshEP8mjCCachePK11mjResource_, cxx:__ZN9mjCOctree11SetMaxDepthEi, cxx:__ZN9mjCOctree12CreateOctreeEPKd, cxx:__ZN9mjCOctree16ComputeSdfCoeffsEPKdiPKiiRK26mjCBoundingVolumeHierarchy, cxx:__ZN9mjCOctree5ClearEv, cxx:__ZN9mjCOctree7SetFaceERKNSt3__16vectorIdNS0_9allocatorIdEEEERKNS1_IiNS2_IiEEEE, cxx:__ZNK6mujoco4user8FilePath3StrEv, cxx:__ZNK7mjCMesh16CheckInitialMeshEv, cxx:__ZNK7mjCMesh5nfaceEv, cxx:__ZNK7mjCMesh5nvertEv, cxx:__ZNK9mjCOctree8NumNodesEv, cxx:_mj_getCache, cxx:_mjs_getString, cxx:_mju_closeResource
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_try_compile(self_ptr: *mut mjCMesh, vfs: *const mjVFS) {
    todo!() // cxx:__ZN7mjCMesh10TryCompileEPK6mjVFS_
}

/// C: mjCMesh::LoadCachedMesh (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1268)
/// Calls: cxx:__ZN8mjCCache12PopulateDataERKNSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEEPK11mjResource_NS0_8functionIFbPKvEEE, cxx:__ZNK7mjCMesh5nvertEv, cxx:_mju_malloc
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_load_cached_mesh(self_ptr: *mut mjCMesh, cache: *mut mjCCache, resource: *const mjResource) -> bool {
    todo!() // cxx:__ZN7mjCMesh14LoadCachedMeshEP8mjCCachePK11mjResource_
}

/// C: mjCMesh::CacheMesh (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1271)
/// Calls: cxx:__ZN8mjCCache6InsertERKNSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEES8_PK11mjResource_NS0_10shared_ptrIKvEEm, cxx:__ZNK26mjCBoundingVolumeHierarchy4SizeEv, cxx:__ZNK7mjCMesh11npolygonmapEv, cxx:__ZNK7mjCMesh12npolygonvertEv, cxx:__ZNK9mjCOctree4SizeEv, cxx:_mju_malloc
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_cache_mesh(self_ptr: *mut mjCMesh, cache: *mut mjCCache, resource: *const mjResource) {
    todo!() // cxx:__ZN7mjCMesh9CacheMeshEP8mjCCachePK11mjResource_
}

/// C: mjCMesh::ProcessVertices (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1274)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_process_vertices(self_ptr: *mut mjCMesh, vert: *const (), remove_repeated: bool) {
    todo!() // cxx:__ZN7mjCMesh15ProcessVerticesERKNSt3__16vectorIfNS0_9allocatorIfEEEEb
}

/// C: mjCMesh::LoadFromDecoder (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1277)
/// Calls: cxx:__ZN7mjCMesh15ProcessVerticesERKNSt3__16vectorIfNS0_9allocatorIfEEEEb, cxx:_mj_deleteSpec, cxx:_mjp_findDecoder, cxx:_mjs_asMesh, cxx:_mjs_firstElement
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_load_from_decoder(self_ptr: *mut mjCMesh, resource: *mut mjResource, remove_repeated: bool) {
    todo!() // cxx:__ZN7mjCMesh15LoadFromDecoderEP11mjResource_b
}

/// C: mjCMesh::LoadMSH (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1279)
/// Calls: cxx:__ZN7mjCMesh15ProcessVerticesERKNSt3__16vectorIfNS0_9allocatorIfEEEEb, cxx:_mju_readResource
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_load_msh(self_ptr: *mut mjCMesh, resource: *mut mjResource, remove_repeated: bool) {
    todo!() // cxx:__ZN7mjCMesh7LoadMSHEP11mjResource_b
}

/// C: mjCMesh::LoadSDF (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1281)
/// Calls: cxx:__ZN7mjCMesh15ProcessVerticesERKNSt3__16vectorIfNS0_9allocatorIfEEEEb, cxx:__ZN8mjCModel13ResolvePluginEP7mjCBaseRKNSt3__112basic_stringIcNS2_11char_traitsIcEENS2_9allocatorIcEEEESA_PP9mjCPlugin, cxx:_mjp_getPluginAtSlot
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_load_sdf(self_ptr: *mut mjCMesh) {
    todo!() // cxx:__ZN7mjCMesh7LoadSDFEv
}

/// C: mjCMesh::MakeGraph (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1282)
/// Calls: cxx:__Z10mjuu_dist3PKdS0_, cxx:__Z13mjuu_crossvecPdPKdS1_, cxx:__Z9mjuu_dot3PKdS0_, cxx:__ZNK7mjCMesh5nvertEv, cxx:_mju_error, cxx:_mju_free, cxx:_mju_malloc, cxx:_mju_warning
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_make_graph(self_ptr: *mut mjCMesh, dvert: *const f64) {
    todo!() // cxx:__ZN7mjCMesh9MakeGraphEPKd
}

/// C: mjCMesh::CopyGraph (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1283)
/// Calls: cxx:__ZNK7mjCMesh5nfaceEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_copy_graph__8a22f29b846e120f(self_ptr: *mut mjCMesh) {
    todo!() // cxx:__ZN7mjCMesh9CopyGraphEv
}

/// C: mjCMesh::MakeNormal (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1284)
/// Calls: cxx:__Z13mjuu_crossvecPdPKdS1_, cxx:__Z9mjuu_dot3PKdS0_, cxx:__ZNK7mjCMesh5nfaceEv, cxx:__ZNK7mjCMesh5nvertEv, cxx:__ZNK7mjCMesh7nnormalEv, cxx:_mju_free, cxx:_mju_malloc
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_make_normal(self_ptr: *mut mjCMesh, dvert: *const f64) {
    todo!() // cxx:__ZN7mjCMesh10MakeNormalEPKd
}

/// C: mjCMesh::MakeCenter (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1285)
/// Calls: cxx:__Z13mjuu_crossvecPdPKdS1_, cxx:__Z9mjuu_dot3PKdS0_, cxx:__ZNK7mjCMesh5nfaceEv, cxx:_mju_malloc
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_make_center(self_ptr: *mut mjCMesh, dvert: *const f64) {
    todo!() // cxx:__ZN7mjCMesh10MakeCenterEPKd
}

/// C: mjCMesh::Process (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1286)
/// Calls: cxx-internal:user_mesh.cc.o:__ZL8trianglePdS_PKdS1_S1_, cxx:__Z9mjuu_eig3PdS_S_PKd, cxx:__ZN26mjCBoundingVolumeHierarchy23AllocateBoundingVolumesEi, cxx:__ZN26mjCBoundingVolumeHierarchy9CreateBVHEP8mjCModelPK7mjCBase, cxx:__ZN7mjCMesh10MakeCenterEPKd, cxx:__ZN7mjCMesh10MakeNormalEPKd, cxx:__ZN7mjCMesh12MakePolygonsEPKd, cxx:__ZN7mjCMesh17SetBoundingVolumeEiPKd, cxx:__ZN7mjCMesh18MakePolygonNormalsEPKd, cxx:__ZN7mjCMesh20ApplyTransformationsEPd, cxx:__ZN7mjCMesh6RotateEPdS0_, cxx:__ZN7mjCMesh9MakeGraphEPKd, cxx:__ZN9mjCOctree11SetMaxDepthEi, cxx:__ZN9mjCOctree12CreateOctreeEPKd, cxx:__ZN9mjCOctree16ComputeSdfCoeffsEPKdiPKiiRK26mjCBoundingVolumeHierarchy, cxx:__ZN9mjCOctree7SetFaceERKNSt3__16vectorIdNS0_9allocatorIdEEEERKNS1_IiNS2_IiEEEE, cxx:__ZNK26mjCBoundingVolumeHierarchy3BvhEv, cxx:__ZNK7mjCMesh12GetVolumeRefEv, cxx:__ZNK7mjCMesh13ComputeVolumeEPdPKdS2_, cxx:__ZNK7mjCMesh14ComputeInertiaEPdPKdS2_, cxx:__ZNK7mjCMesh18ComputeSurfaceAreaEPdPKdS2_, cxx:__ZNK7mjCMesh19ComputeFaceCentroidEPdPKd, cxx:__ZNK7mjCMesh5nfaceEv, cxx:__ZNK7mjCMesh5nvertEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_process(self_ptr: *mut mjCMesh) {
    todo!() // cxx:__ZN7mjCMesh7ProcessEv
}

/// C: mjCMesh::ApplyTransformations (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1287)
/// Calls: cxx:__Z13mjuu_quat2matPdPKd, cxx:__Z15mjuu_mulvecmatTPdPKdS1_, cxx:__ZNK7mjCMesh5nvertEv, cxx:__ZNK7mjCMesh7nnormalEv
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_apply_transformations(self_ptr: *mut mjCMesh, dvert: *mut f64) {
    todo!() // cxx:__ZN7mjCMesh20ApplyTransformationsEPd
}

/// C: mjCMesh::ComputeFaceCentroid (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1288)
/// Calls: cxx-internal:user_mesh.cc.o:__ZL8trianglePdS_PKdS1_S1_, cxx:__ZNK7mjCMesh5nfaceEv
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_compute_face_centroid(self_ptr: *mut mjCMesh, arg0: *mut f64, dvert: *const f64) -> f64 {
    todo!() // cxx:__ZNK7mjCMesh19ComputeFaceCentroidEPdPKd
}

/// C: mjCMesh::CheckInitialMesh (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1289)
/// Calls: cxx:__ZNK7mjCMesh5nvertEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_check_initial_mesh(self_ptr: *mut mjCMesh) {
    todo!() // cxx:__ZNK7mjCMesh16CheckInitialMeshEv
}

/// C: mjCMesh::CopyPlugin (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1290)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_copy_plugin(self_ptr: *mut mjCMesh) {
    todo!() // cxx:__ZN7mjCMesh10CopyPluginEv
}

/// C: mjCMesh::Rotate (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1291)
/// Calls: cxx:__Z13mjuu_quat2matPdPKd, cxx:__Z14mjuu_mulvecmatPdPKdS1_, cxx:__ZNK7mjCMesh5nvertEv, cxx:__ZNK7mjCMesh7nnormalEv
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_rotate(self_ptr: *mut mjCMesh, quat: *mut f64, dvert: *mut f64) {
    todo!() // cxx:__ZN7mjCMesh6RotateEPdS0_
}

/// C: mjCMesh::MakePolygons (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1293)
/// Calls: cxx:__Z14MeshPolygonKeyRNSt3__14pairIddEEPKdS4_S4_d, cxx:__ZN11MeshPolygon10InsertFaceEiii, cxx:__ZNK11MeshPolygon5PathsEv, cxx:__ZNK7mjCMesh10GraphFacesEv, cxx:__ZNK7mjCMesh5nvertEv
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_make_polygons(self_ptr: *mut mjCMesh, dvert: *const f64) {
    todo!() // cxx:__ZN7mjCMesh12MakePolygonsEPKd
}

/// C: mjCMesh::MakePolygonNormals (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1294)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_make_polygon_normals(self_ptr: *mut mjCMesh, dvert: *const f64) {
    todo!() // cxx:__ZN7mjCMesh18MakePolygonNormalsEPKd
}

/// C: mjCMesh::ComputeInertia (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1297)
/// Calls: cxx-internal:user_mesh.cc.o:__ZL8trianglePdS_PKdS1_S1_, cxx:__Z9mjuu_dot3PKdS0_, cxx:__ZNK7mjCMesh10GraphFacesEv, cxx:__ZNK7mjCMesh5nfaceEv, cxx:__ZNK7mjCMesh5nvertEv
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_compute_inertia(self_ptr: *mut mjCMesh, inert: *mut f64, CoM: *const f64, dvert: *const f64) -> f64 {
    todo!() // cxx:__ZNK7mjCMesh14ComputeInertiaEPdPKdS2_
}

/// C: mjCMesh::GraphFaces (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1299)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_graph_faces(self_ptr: *mut mjCMesh) -> *mut i32 {
    todo!() // cxx:__ZNK7mjCMesh10GraphFacesEv
}

/// C: mjCMesh::ComputeVolume (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1313)
/// Calls: cxx-internal:user_mesh.cc.o:__ZL8trianglePdS_PKdS1_S1_, cxx:__Z9mjuu_dot3PKdS0_, cxx:__ZNK7mjCMesh10GraphFacesEv, cxx:__ZNK7mjCMesh5nfaceEv
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_compute_volume(self_ptr: *mut mjCMesh, CoM: *mut f64, facecen: *const f64, dvert: *const f64) -> f64 {
    todo!() // cxx:__ZNK7mjCMesh13ComputeVolumeEPdPKdS2_
}

/// C: mjCMesh::ComputeSurfaceArea (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1314)
/// Calls: cxx-internal:user_mesh.cc.o:__ZL8trianglePdS_PKdS1_S1_, cxx:__ZNK7mjCMesh5nfaceEv
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_compute_surface_area(self_ptr: *mut mjCMesh, CoM: *mut f64, facecen: *const f64, dvert: *const f64) -> f64 {
    todo!() // cxx:__ZNK7mjCMesh18ComputeSurfaceAreaEPdPKdS2_
}

/// C: mjCSkin::File (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1364)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_skin_file(self_ptr: *mut mjCSkin) -> *const std__string {
    todo!() // cxx:__ZNK7mjCSkin4FileEv
}

/// C: mjCSkin::get_material (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1365)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_skin_get_material(self_ptr: *mut mjCSkin) -> *const std__string {
    todo!() // cxx:__ZNK7mjCSkin12get_materialEv
}

/// C: mjCSkin::get_vert (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1366)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_skin_get_vert(self_ptr: *mut mjCSkin) -> *const () {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).vert_ }
}

/// C: mjCSkin::get_texcoord (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1367)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_skin_get_texcoord(self_ptr: *mut mjCSkin) -> *const () {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).texcoord_ }
}

/// C: mjCSkin::get_face (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1368)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_skin_get_face(self_ptr: *mut mjCSkin) -> *const () {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).face_ }
}

/// C: mjCSkin::get_bodyname (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1369)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_skin_get_bodyname(self_ptr: *mut mjCSkin) -> *const () {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).bodyname_ }
}

/// C: mjCSkin::get_bindpos (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1370)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_skin_get_bindpos(self_ptr: *mut mjCSkin) -> *const () {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).bindpos_ }
}

/// C: mjCSkin::get_bindquat (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1371)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_skin_get_bindquat(self_ptr: *mut mjCSkin) -> *const () {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).bindquat_ }
}

/// C: mjCSkin::get_vertid (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1372)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_skin_get_vertid(self_ptr: *mut mjCSkin) -> *const () {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).vertid_ }
}

/// C: mjCSkin::get_vertweight (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1373)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_skin_get_vertweight(self_ptr: *mut mjCSkin) -> *const () {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).vertweight_ }
}

/// C: mjCSkin::del_material (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1374)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_skin_del_material(self_ptr: *mut mjCSkin) {
    todo!() // cxx:__ZN7mjCSkin12del_materialEv
}

/// C: mjCSkin::CopyFromSpec (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1376)
/// Calls: cxx:__Z13mjuu_stripextNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEE, cxx:__Z14mjuu_strippathNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEE
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_skin_copy_from_spec(self_ptr: *mut mjCSkin) {
    todo!() // cxx:__ZN7mjCSkin12CopyFromSpecEv
}

/// C: mjCSkin::PointToLocal (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1377)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_skin_point_to_local(self_ptr: *mut mjCSkin) {
    todo!() // cxx:__ZN7mjCSkin12PointToLocalEv
}

/// C: mjCSkin::ResolveReferences (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1380)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_skin_resolve_references(self_ptr: *mut mjCSkin, m: *const mjCModel) {
    todo!() // cxx:__ZN7mjCSkin17ResolveReferencesEPK8mjCModel
}

/// C: mjCSkin::NameSpace (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1381)
/// Calls: cxx:__Z13mjuu_stripextNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEE, cxx:__Z14mjuu_strippathNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEE
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_skin_name_space(self_ptr: *mut mjCSkin, m: *const mjCModel) {
    todo!() // cxx:__ZN7mjCSkin9NameSpaceEPK8mjCModel
}

/// C: mjCSkin::Compile (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1382)
/// Calls: cxx:__Z11mjuu_getextNSt3__117basic_string_viewIcNS_11char_traitsIcEEEE, cxx:__Z14mjuu_strippathNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEE, cxx:__ZN7mjCBase12LoadResourceERKNSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEES8_PK6mjVFS_, cxx:__ZN7mjCSkin12CopyFromSpecEv, cxx:__ZN7mjCSkin17ResolveReferencesEPK8mjCModel, cxx:__ZN7mjCSkin7LoadSKNEP11mjResource_, cxx:__ZNK6mujoco4user8FilePath3StrEv, cxx:_mjs_getString, cxx:_mju_closeResource
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_skin_compile(self_ptr: *mut mjCSkin, vfs: *const mjVFS) {
    todo!() // cxx:__ZN7mjCSkin7CompileEPK6mjVFS_
}

/// C: mjCSkin::LoadSKN (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1383)
/// Calls: cxx:_mju_readResource
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_skin_load_skn(self_ptr: *mut mjCSkin, resource: *mut mjResource) {
    todo!() // cxx:__ZN7mjCSkin7LoadSKNEP11mjResource_
}

/// C: mjCHField::CopyFromSpec (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1417)
/// Calls: cxx:__Z13mjuu_stripextNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEE, cxx:__Z14mjuu_strippathNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEE
#[allow(unused_variables, non_snake_case)]
pub fn mj_ch_field_copy_from_spec(self_ptr: *mut mjCHField) {
    todo!() // cxx:__ZN9mjCHField12CopyFromSpecEv
}

/// C: mjCHField::PointToLocal (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1418)
#[allow(unused_variables, non_snake_case)]
pub fn mj_ch_field_point_to_local(self_ptr: *mut mjCHField) {
    todo!() // cxx:__ZN9mjCHField12PointToLocalEv
}

/// C: mjCHField::NameSpace (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1419)
/// Calls: cxx:__Z13mjuu_stripextNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEE, cxx:__Z14mjuu_strippathNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEE, cxx:__ZN7mjCBase9NameSpaceEPK8mjCModel
#[allow(unused_variables, non_snake_case)]
pub fn mj_ch_field_name_space(self_ptr: *mut mjCHField, m: *const mjCModel) {
    todo!() // cxx:__ZN9mjCHField9NameSpaceEPK8mjCModel
}

/// C: mjCHField::File (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1421)
#[allow(unused_variables, non_snake_case)]
pub fn mj_ch_field_file(self_ptr: *mut mjCHField) -> std__string {
    todo!() // cxx:__ZNK9mjCHField4FileEv
}

/// C: mjCHField::get_userdata (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1424)
#[allow(unused_variables, non_snake_case)]
pub fn mj_ch_field_get_userdata(self_ptr: *mut mjCHField) -> *const () {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).userdata_ }
}

/// C: mjCHField::Compile (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1427)
/// Calls: cxx:__Z14mjuu_strippathNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEE, cxx:__ZN7mjCBase12LoadResourceERKNSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEES8_PK6mjVFS_, cxx:__ZN7mjCBase19GetAssetContentTypeENSt3__117basic_string_viewIcNS0_11char_traitsIcEEEES4_, cxx:__ZN8mjCCache12PopulateDataERKNSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEEPK11mjResource_NS0_8functionIFbPKvEEE, cxx:__ZN8mjCCache6InsertERKNSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEES8_PK11mjResource_NS0_10shared_ptrIKvEEm, cxx:__ZN9mjCHField10GetCacheIdEPK11mjResource_RKNSt3__112basic_stringIcNS3_11char_traitsIcEENS3_9allocatorIcEEEE, cxx:__ZN9mjCHField10LoadCustomEP11mjResource_, cxx:__ZN9mjCHField12CopyFromSpecEv, cxx:__ZN9mjCHField7LoadPNGEP11mjResource_, cxx:__ZNK6mujoco4user8FilePath3StrEv, cxx:_mj_getCache, cxx:_mjs_getString, cxx:_mju_closeResource
#[allow(unused_variables, non_snake_case)]
pub fn mj_ch_field_compile(self_ptr: *mut mjCHField, vfs: *const mjVFS) {
    todo!() // cxx:__ZN9mjCHField7CompileEPK6mjVFS_
}

/// C: mjCHField::GetCacheId (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1429)
#[allow(unused_variables, non_snake_case)]
pub fn mj_ch_field_get_cache_id(self_ptr: *mut mjCHField, resource: *const mjResource, asset_type: *const std__string) -> std__string {
    todo!() // cxx:__ZN9mjCHField10GetCacheIdEPK11mjResource_RKNSt3__112basic_stringIcNS3_11char_traitsIcEENS3_9allocatorIcEEEE
}

/// C: mjCHField::LoadCustom (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1430)
/// Calls: cxx:_mju_readResource
#[allow(unused_variables, non_snake_case)]
pub fn mj_ch_field_load_custom(self_ptr: *mut mjCHField, resource: *mut mjResource) {
    todo!() // cxx:__ZN9mjCHField10LoadCustomEP11mjResource_
}

/// C: mjCHField::LoadPNG (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1431)
/// Calls: cxx-internal:user_objects.cc.o:__ZN12_GLOBAL__N_18PNGImage4LoadEPK7mjCBaseP11mjResource_16LodePNGColorType, cxx-internal:user_objects.cc.o:__ZNK12_GLOBAL__N_18PNGImage5WidthEv, cxx-internal:user_objects.cc.o:__ZNK12_GLOBAL__N_18PNGImage6HeightEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_ch_field_load_png(self_ptr: *mut mjCHField, resource: *mut mjResource) {
    todo!() // cxx:__ZN9mjCHField7LoadPNGEP11mjResource_
}

/// C: mjCTexture::CopyFromSpec (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1465)
/// Calls: cxx:__Z13mjuu_stripextNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEE, cxx:__Z14mjuu_strippathNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEE
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_texture_copy_from_spec(self_ptr: *mut mjCTexture) {
    todo!() // cxx:__ZN10mjCTexture12CopyFromSpecEv
}

/// C: mjCTexture::PointToLocal (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1466)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_texture_point_to_local(self_ptr: *mut mjCTexture) {
    todo!() // cxx:__ZN10mjCTexture12PointToLocalEv
}

/// C: mjCTexture::NameSpace (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1467)
/// Calls: cxx:__Z13mjuu_stripextNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEE, cxx:__Z14mjuu_strippathNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEE, cxx:__ZN7mjCBase9NameSpaceEPK8mjCModel
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_texture_name_space(self_ptr: *mut mjCTexture, m: *const mjCModel) {
    todo!() // cxx:__ZN10mjCTexture9NameSpaceEPK8mjCModel
}

/// C: mjCTexture::Compile (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1468)
/// Calls: cxx:__Z14mjuu_strippathNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEE, cxx:__ZN10mjCTexture11BuiltinCubeEv, cxx:__ZN10mjCTexture12CopyFromSpecEv, cxx:__ZN10mjCTexture12FlipIfNeededERNSt3__16vectorISt4byteNS0_9allocatorIS2_EEEEjj, cxx:__ZN10mjCTexture14LoadCubeSingleENSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEEPK6mjVFS_, cxx:__ZN10mjCTexture16LoadCubeSeparateEPK6mjVFS_, cxx:__ZN10mjCTexture6Load2DENSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEEPK6mjVFS_, cxx:__ZN10mjCTexture9Builtin2DEv, cxx:__ZNK6mujoco4user8FilePath3StrEv, cxx:_mjs_getString
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_texture_compile(self_ptr: *mut mjCTexture, vfs: *const mjVFS) {
    todo!() // cxx:__ZN10mjCTexture7CompileEPK6mjVFS_
}

/// C: mjCTexture::File (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1471)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_texture_file(self_ptr: *mut mjCTexture) -> std__string {
    todo!() // cxx:__ZNK10mjCTexture4FileEv
}

/// C: mjCTexture::get_content_type (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1472)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_texture_get_content_type(self_ptr: *mut mjCTexture) -> std__string {
    todo!() // cxx:__ZNK10mjCTexture16get_content_typeEv
}

/// C: mjCTexture::get_cubefiles (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1473)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_texture_get_cubefiles(self_ptr: *mut mjCTexture) -> *const () {
    todo!() // cxx:__ZNK10mjCTexture13get_cubefilesEv
}

/// C: mjCTexture::GetCacheId (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1477)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_texture_get_cache_id(self_ptr: *mut mjCTexture, resource: *const mjResource, asset_type: *const std__string) -> std__string {
    todo!() // cxx:__ZN10mjCTexture10GetCacheIdEPK11mjResource_RKNSt3__112basic_stringIcNS3_11char_traitsIcEENS3_9allocatorIcEEEE
}

/// C: mjCTexture::Builtin2D (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1478)
/// Calls: cxx-internal:user_objects.cc.o:__ZL6interpPSt4bytePKdS2_d, cxx-internal:user_objects.cc.o:__ZL7checkerPSt4bytePKS_S2_ii, cxx-internal:user_objects.cc.o:__ZL9randomdotPSt4bytePKdiid
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_texture_builtin2d(self_ptr: *mut mjCTexture) {
    todo!() // cxx:__ZN10mjCTexture9Builtin2DEv
}

/// C: mjCTexture::BuiltinCube (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1479)
/// Calls: cxx-internal:user_objects.cc.o:__ZL6interpPSt4bytePKdS2_d, cxx-internal:user_objects.cc.o:__ZL7checkerPSt4bytePKS_S2_ii, cxx-internal:user_objects.cc.o:__ZL9randomdotPSt4bytePKdiid
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_texture_builtin_cube(self_ptr: *mut mjCTexture) {
    todo!() // cxx:__ZN10mjCTexture11BuiltinCubeEv
}

/// C: mjCTexture::Load2D (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1480)
/// Calls: cxx:__ZN10mjCTexture8LoadFlipENSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEEPK6mjVFS_RNS0_6vectorISt4byteNS4_ISB_EEEERjSF_Rb
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_texture_load2d(self_ptr: *mut mjCTexture, filename: std__string, vfs: *const mjVFS) {
    todo!() // cxx:__ZN10mjCTexture6Load2DENSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEEPK6mjVFS_
}

/// C: mjCTexture::LoadCubeSingle (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1481)
/// Calls: cxx:__ZN10mjCTexture8LoadFlipENSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEEPK6mjVFS_RNS0_6vectorISt4byteNS4_ISB_EEEERjSF_Rb
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_texture_load_cube_single(self_ptr: *mut mjCTexture, filename: std__string, vfs: *const mjVFS) {
    todo!() // cxx:__ZN10mjCTexture14LoadCubeSingleENSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEEPK6mjVFS_
}

/// C: mjCTexture::LoadCubeSeparate (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1483)
/// Calls: cxx:__Z14mjuu_strippathNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEE, cxx:__ZN10mjCTexture8LoadFlipENSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEEPK6mjVFS_RNS0_6vectorISt4byteNS4_ISB_EEEERjSF_Rb, cxx:__ZNK6mujoco4user8FilePath3StrEv, cxx:_mjs_getString
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_texture_load_cube_separate(self_ptr: *mut mjCTexture, vfs: *const mjVFS) {
    todo!() // cxx:__ZN10mjCTexture16LoadCubeSeparateEPK6mjVFS_
}

/// C: mjCTexture::FlipIfNeeded (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1485)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_texture_flip_if_needed(self_ptr: *mut mjCTexture, image: *const (), w: u32, h: u32) {
    todo!() // cxx:__ZN10mjCTexture12FlipIfNeededERNSt3__16vectorISt4byteNS0_9allocatorIS2_EEEEjj
}

/// C: mjCTexture::LoadFlip (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1487)
/// Calls: cxx:__ZN10mjCTexture10GetCacheIdEPK11mjResource_RKNSt3__112basic_stringIcNS3_11char_traitsIcEENS3_9allocatorIcEEEE, cxx:__ZN10mjCTexture10LoadCustomEP11mjResource_RNSt3__16vectorISt4byteNS2_9allocatorIS4_EEEERjS9_Rb, cxx:__ZN10mjCTexture12FlipIfNeededERNSt3__16vectorISt4byteNS0_9allocatorIS2_EEEEjj, cxx:__ZN10mjCTexture7LoadKTXEP11mjResource_RNSt3__16vectorISt4byteNS2_9allocatorIS4_EEEERjS9_Rb, cxx:__ZN10mjCTexture7LoadPNGEP11mjResource_RNSt3__16vectorISt4byteNS2_9allocatorIS4_EEEERjS9_Rb, cxx:__ZN7mjCBase12LoadResourceERKNSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEES8_PK6mjVFS_, cxx:__ZN7mjCBase19GetAssetContentTypeENSt3__117basic_string_viewIcNS0_11char_traitsIcEEEES4_, cxx:__ZN8mjCCache12PopulateDataERKNSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEEPK11mjResource_NS0_8functionIFbPKvEEE, cxx:__ZN8mjCCache6InsertERKNSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEES8_PK11mjResource_NS0_10shared_ptrIKvEEm, cxx:_mj_getCache, cxx:_mju_closeResource
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_texture_load_flip(self_ptr: *mut mjCTexture, filename: std__string, vfs: *const mjVFS, image: *const (), w: *mut u32, h: *mut u32, is_srgb: *mut bool) {
    todo!() // cxx:__ZN10mjCTexture8LoadFlipENSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEEPK6mjVFS_RNS0_6vectorISt4byteNS4_ISB_EEEERjSF_Rb
}

/// C: mjCTexture::LoadPNG (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1491)
/// Calls: cxx-internal:user_objects.cc.o:__ZN12_GLOBAL__N_18PNGImage4LoadEPK7mjCBaseP11mjResource_16LodePNGColorType, cxx-internal:user_objects.cc.o:__ZNK12_GLOBAL__N_18PNGImage5WidthEv, cxx-internal:user_objects.cc.o:__ZNK12_GLOBAL__N_18PNGImage6HeightEv, cxx-internal:user_objects.cc.o:__ZNK12_GLOBAL__N_18PNGImage6IsSRGBEv, cxx-internal:user_objects.cc.o:__ZNO12_GLOBAL__N_18PNGImage8MoveDataEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_texture_load_png(self_ptr: *mut mjCTexture, resource: *mut mjResource, image: *const (), w: *mut u32, h: *mut u32, is_srgb: *mut bool) {
    todo!() // cxx:__ZN10mjCTexture7LoadPNGEP11mjResource_RNSt3__16vectorISt4byteNS2_9allocatorIS4_EEEERjS9_Rb
}

/// C: mjCTexture::LoadKTX (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1493)
/// Calls: cxx:_mju_readResource
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_texture_load_ktx(self_ptr: *mut mjCTexture, resource: *mut mjResource, image: *const (), w: *mut u32, h: *mut u32, is_srgb: *mut bool) {
    todo!() // cxx:__ZN10mjCTexture7LoadKTXEP11mjResource_RNSt3__16vectorISt4byteNS2_9allocatorIS4_EEEERjS9_Rb
}

/// C: mjCTexture::LoadCustom (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1495)
/// Calls: cxx:_mju_readResource
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_texture_load_custom(self_ptr: *mut mjCTexture, resource: *mut mjResource, image: *const (), w: *mut u32, h: *mut u32, is_srgb: *mut bool) {
    todo!() // cxx:__ZN10mjCTexture10LoadCustomEP11mjResource_RNSt3__16vectorISt4byteNS2_9allocatorIS4_EEEERjS9_Rb
}

/// C: mjCMaterial::CopyFromSpec (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1526)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_material_copy_from_spec(self_ptr: *mut mjCMaterial) {
    todo!() // cxx:__ZN11mjCMaterial12CopyFromSpecEv
}

/// C: mjCMaterial::PointToLocal (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1527)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_material_point_to_local(self_ptr: *mut mjCMaterial) {
    todo!() // cxx:__ZN11mjCMaterial12PointToLocalEv
}

/// C: mjCMaterial::NameSpace (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1528)
/// Calls: cxx:__ZN7mjCBase9NameSpaceEPK8mjCModel
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_material_name_space(self_ptr: *mut mjCMaterial, m: *const mjCModel) {
    todo!() // cxx:__ZN11mjCMaterial9NameSpaceEPK8mjCModel
}

/// C: mjCMaterial::get_texture (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1530)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_material_get_texture(self_ptr: *mut mjCMaterial, i: i32) -> *const std__string {
    todo!() // cxx:__ZNK11mjCMaterial11get_textureEi
}

/// C: mjCMaterial::del_textures (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1531)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_material_del_textures(self_ptr: *mut mjCMaterial) {
    todo!() // cxx:__ZN11mjCMaterial12del_texturesEv
}

/// C: mjCMaterial::Compile (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1534)
/// Calls: cxx:__ZN11mjCMaterial12CopyFromSpecEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_material_compile(self_ptr: *mut mjCMaterial) {
    todo!() // cxx:__ZN11mjCMaterial7CompileEv
}

/// C: mjCPair::CopyFromSpec (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1565)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_pair_copy_from_spec(self_ptr: *mut mjCPair) {
    todo!() // cxx:__ZN7mjCPair12CopyFromSpecEv
}

/// C: mjCPair::PointToLocal (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1566)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_pair_point_to_local(self_ptr: *mut mjCPair) {
    todo!() // cxx:__ZN7mjCPair12PointToLocalEv
}

/// C: mjCPair::ResolveReferences (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1567)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_pair_resolve_references(self_ptr: *mut mjCPair, m: *const mjCModel) {
    todo!() // cxx:__ZN7mjCPair17ResolveReferencesEPK8mjCModel
}

/// C: mjCPair::NameSpace (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1568)
/// Calls: cxx:__ZN7mjCBase9NameSpaceEPK8mjCModel
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_pair_name_space(self_ptr: *mut mjCPair, m: *const mjCModel) {
    todo!() // cxx:__ZN7mjCPair9NameSpaceEPK8mjCModel
}

/// C: mjCPair::get_geomname1 (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1570)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_pair_get_geomname1(self_ptr: *mut mjCPair) -> *const std__string {
    todo!() // cxx:__ZNK7mjCPair13get_geomname1Ev
}

/// C: mjCPair::get_geomname2 (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1571)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_pair_get_geomname2(self_ptr: *mut mjCPair) -> *const std__string {
    todo!() // cxx:__ZNK7mjCPair13get_geomname2Ev
}

/// C: mjCPair::GetSignature (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1573)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_pair_get_signature(self_ptr: *mut mjCPair) -> i32 {
    todo!() // cxx:__ZN7mjCPair12GetSignatureEv
}

/// C: mjCPair::Compile (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1578)
/// Calls: cxx:__Z12mjuu_definedd, cxx:__ZN7mjCGeom12SetNotVisualEv, cxx:__ZN7mjCPair12CopyFromSpecEv, cxx:__ZN7mjCPair17ResolveReferencesEPK8mjCModel
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_pair_compile(self_ptr: *mut mjCPair) {
    todo!() // cxx:__ZN7mjCPair7CompileEv
}

/// C: mjCBodyPair::CopyFromSpec (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1613)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_pair_copy_from_spec(self_ptr: *mut mjCBodyPair) {
    todo!() // cxx:__ZN11mjCBodyPair12CopyFromSpecEv
}

/// C: mjCBodyPair::PointToLocal (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1614)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_pair_point_to_local(self_ptr: *mut mjCBodyPair) {
    todo!() // cxx:__ZN11mjCBodyPair12PointToLocalEv
}

/// C: mjCBodyPair::ResolveReferences (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1615)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_pair_resolve_references(self_ptr: *mut mjCBodyPair, m: *const mjCModel) {
    todo!() // cxx:__ZN11mjCBodyPair17ResolveReferencesEPK8mjCModel
}

/// C: mjCBodyPair::NameSpace (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1616)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_pair_name_space(self_ptr: *mut mjCBodyPair, m: *const mjCModel) {
    todo!() // cxx:__ZN11mjCBodyPair9NameSpaceEPK8mjCModel
}

/// C: mjCBodyPair::get_bodyname1 (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1618)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_pair_get_bodyname1(self_ptr: *mut mjCBodyPair) -> std__string {
    todo!() // cxx:__ZNK11mjCBodyPair13get_bodyname1Ev
}

/// C: mjCBodyPair::get_bodyname2 (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1619)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_pair_get_bodyname2(self_ptr: *mut mjCBodyPair) -> std__string {
    todo!() // cxx:__ZNK11mjCBodyPair13get_bodyname2Ev
}

/// C: mjCBodyPair::GetSignature (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1621)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_pair_get_signature(self_ptr: *mut mjCBodyPair) -> i32 {
    todo!() // cxx:__ZN11mjCBodyPair12GetSignatureEv
}

/// C: mjCBodyPair::Compile (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1626)
/// Calls: cxx:__ZN11mjCBodyPair12CopyFromSpecEv, cxx:__ZN11mjCBodyPair17ResolveReferencesEPK8mjCModel
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_pair_compile(self_ptr: *mut mjCBodyPair) {
    todo!() // cxx:__ZN11mjCBodyPair7CompileEv
}

/// C: mjCEquality::CopyFromSpec (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1658)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_equality_copy_from_spec(self_ptr: *mut mjCEquality) {
    todo!() // cxx:__ZN11mjCEquality12CopyFromSpecEv
}

/// C: mjCEquality::PointToLocal (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1659)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_equality_point_to_local(self_ptr: *mut mjCEquality) {
    todo!() // cxx:__ZN11mjCEquality12PointToLocalEv
}

/// C: mjCEquality::ResolveReferences (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1660)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_equality_resolve_references(self_ptr: *mut mjCEquality, m: *const mjCModel) {
    todo!() // cxx:__ZN11mjCEquality17ResolveReferencesEPK8mjCModel
}

/// C: mjCEquality::NameSpace (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1661)
/// Calls: cxx:__ZN7mjCBase9NameSpaceEPK8mjCModel
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_equality_name_space(self_ptr: *mut mjCEquality, m: *const mjCModel) {
    todo!() // cxx:__ZN11mjCEquality9NameSpaceEPK8mjCModel
}

/// C: mjCEquality::Compile (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1664)
/// Calls: cxx:__ZN11mjCEquality12CopyFromSpecEv, cxx:__ZN11mjCEquality17ResolveReferencesEPK8mjCModel, cxx:__ZNK8mjCModel6FlexesEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_equality_compile(self_ptr: *mut mjCEquality) {
    todo!() // cxx:__ZN11mjCEquality7CompileEv
}

/// C: mjCTendon::set_material (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1697)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tendon_set_material(self_ptr: *mut mjCTendon, _material: std__string) {
    todo!() // cxx:__ZN9mjCTendon12set_materialENSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEE
}

/// C: mjCTendon::get_material (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1698)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tendon_get_material(self_ptr: *mut mjCTendon) -> *const std__string {
    todo!() // cxx:__ZNK9mjCTendon12get_materialEv
}

/// C: mjCTendon::del_material (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1699)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tendon_del_material(self_ptr: *mut mjCTendon) {
    todo!() // cxx:__ZN9mjCTendon12del_materialEv
}

/// C: mjCTendon::WrapSite (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1702)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tendon_wrap_site(self_ptr: *mut mjCTendon, wrapname: std__string, wrapinfo: std__string_view) {
    todo!() // cxx:__ZN9mjCTendon8WrapSiteENSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEENS0_17basic_string_viewIcS3_EE
}

/// C: mjCTendon::WrapGeom (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1703)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tendon_wrap_geom(self_ptr: *mut mjCTendon, wrapname: std__string, side: std__string, wrapinfo: std__string_view) {
    todo!() // cxx:__ZN9mjCTendon8WrapGeomENSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEES6_NS0_17basic_string_viewIcS3_EE
}

/// C: mjCTendon::WrapJoint (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1704)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tendon_wrap_joint(self_ptr: *mut mjCTendon, wrapname: std__string, coef: f64, wrapinfo: std__string_view) {
    todo!() // cxx:__ZN9mjCTendon9WrapJointENSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEEdNS0_17basic_string_viewIcS3_EE
}

/// C: mjCTendon::WrapPulley (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1705)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tendon_wrap_pulley(self_ptr: *mut mjCTendon, divisor: f64, wrapinfo: std__string_view) {
    todo!() // cxx:__ZN9mjCTendon10WrapPulleyEdNSt3__117basic_string_viewIcNS0_11char_traitsIcEEEE
}

/// C: mjCTendon::NumWraps (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1708)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tendon_num_wraps(self_ptr: *mut mjCTendon) -> i32 {
    todo!() // cxx:__ZNK9mjCTendon8NumWrapsEv
}

/// C: mjCTendon::GetWrap (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1709)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tendon_get_wrap(self_ptr: *mut mjCTendon, i: i32) -> *const mjCWrap {
    todo!() // cxx:__ZNK9mjCTendon7GetWrapEi
}

/// C: mjCTendon::get_userdata (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1713)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tendon_get_userdata(self_ptr: *mut mjCTendon) -> *const () {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).userdata_ }
}

/// C: mjCTendon::get_range (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1714)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tendon_get_range(self_ptr: *mut mjCTendon) -> *const f64 {
    todo!() // cxx:__ZN9mjCTendon9get_rangeEv
}

/// C: mjCTendon::CopyFromSpec (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1716)
/// Calls: cxx:__ZNK7mjCWrap4TypeEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tendon_copy_from_spec(self_ptr: *mut mjCTendon) {
    todo!() // cxx:__ZN9mjCTendon12CopyFromSpecEv
}

/// C: mjCTendon::PointToLocal (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1717)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tendon_point_to_local(self_ptr: *mut mjCTendon) {
    todo!() // cxx:__ZN9mjCTendon12PointToLocalEv
}

/// C: mjCTendon::ResolveReferences (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1718)
/// Calls: cxx:__ZN7mjCWrap17ResolveReferencesEPK8mjCModel, cxx:__ZNK7mjCWrap4TypeEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tendon_resolve_references(self_ptr: *mut mjCTendon, m: *const mjCModel) {
    todo!() // cxx:__ZN9mjCTendon17ResolveReferencesEPK8mjCModel
}

/// C: mjCTendon::NameSpace (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1719)
/// Calls: cxx:__ZN7mjCBase9NameSpaceEPK8mjCModel
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tendon_name_space(self_ptr: *mut mjCTendon, m: *const mjCModel) {
    todo!() // cxx:__ZN9mjCTendon9NameSpaceEPK8mjCModel
}

/// C: mjCTendon::SetModel (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1720)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tendon_set_model(self_ptr: *mut mjCTendon, _model: *mut mjCModel) {
    todo!() // cxx:__ZN9mjCTendon8SetModelEP8mjCModel
}

/// C: mjCTendon::is_limited (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1722)
/// Calls: cxx-internal:user_objects.cc.o:__ZL9islimitediPKd
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tendon_is_limited(self_ptr: *mut mjCTendon) -> bool {
    todo!() // cxx:__ZNK9mjCTendon10is_limitedEv
}

/// C: mjCTendon::is_actfrclimited (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1723)
/// Calls: cxx-internal:user_objects.cc.o:__ZL9islimitediPKd
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tendon_is_actfrclimited(self_ptr: *mut mjCTendon) -> bool {
    todo!() // cxx:__ZNK9mjCTendon16is_actfrclimitedEv
}

/// C: mjCTendon::Compile (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1726)
/// Calls: cxx-internal:user_objects.cc.o:__ZL12checklimitedPK7mjCBasebPKcS3_ib, cxx:__ZN7mjCGeom12SetNotVisualEv, cxx:__ZN7mjCWrap7CompileEv, cxx:__ZN9mjCTendon12CopyFromSpecEv, cxx:__ZN9mjCTendon17ResolveReferencesEPK8mjCModel, cxx:__ZNK7mjCWrap4TypeEv, cxx:__ZNK8mjCModel5GeomsEv, cxx:__ZNK9mjCTendon10is_limitedEv, cxx:__ZNK9mjCTendon16is_actfrclimitedEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tendon_compile(self_ptr: *mut mjCTendon) {
    todo!() // cxx:__ZN9mjCTendon7CompileEv
}

/// C: mjCWrap::CopyFromSpec (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1749)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_wrap_copy_from_spec(self_ptr: *mut mjCWrap) {
    todo!() // cxx:__ZN7mjCWrap12CopyFromSpecEv
}

/// C: mjCWrap::PointToLocal (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1750)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_wrap_point_to_local(self_ptr: *mut mjCWrap) {
    todo!() // cxx:__ZN7mjCWrap12PointToLocalEv
}

/// C: mjCWrap::ResolveReferences (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1751)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_wrap_resolve_references(self_ptr: *mut mjCWrap, m: *const mjCModel) {
    todo!() // cxx:__ZN7mjCWrap17ResolveReferencesEPK8mjCModel
}

/// C: mjCWrap::NameSpace (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1752)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_wrap_name_space(self_ptr: *mut mjCWrap, m: *const mjCModel) {
    todo!() // cxx:__ZN7mjCWrap9NameSpaceEPK8mjCModel
}

/// C: mjCWrap::Type (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1753)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_wrap_type(self_ptr: *mut mjCWrap) -> u32 {
    todo!() // cxx:__ZNK7mjCWrap4TypeEv
}

/// C: mjCWrap::Compile (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1762)
/// Calls: cxx:__ZN7mjCWrap12CopyFromSpecEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_wrap_compile(self_ptr: *mut mjCWrap) {
    todo!() // cxx:__ZN7mjCWrap7CompileEv
}

/// C: mjCPlugin::PointToLocal (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1791)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_plugin_point_to_local(self_ptr: *mut mjCPlugin) {
    todo!() // cxx:__ZN9mjCPlugin12PointToLocalEv
}

/// C: mjCPlugin::Compile (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1798)
/// Calls: cxx:__ZN8mjCModel13ResolvePluginEP7mjCBaseRKNSt3__112basic_stringIcNS2_11char_traitsIcEENS2_9allocatorIcEEEESA_PP9mjCPlugin, cxx:_mjp_getPluginAtSlot
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_plugin_compile(self_ptr: *mut mjCPlugin) {
    todo!() // cxx:__ZN9mjCPlugin7CompileEv
}

/// C: mjCActuator::get_userdata (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1843)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_actuator_get_userdata(self_ptr: *mut mjCActuator) -> *const () {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).userdata_ }
}

/// C: mjCActuator::get_target (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1844)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_actuator_get_target(self_ptr: *mut mjCActuator) -> *const std__string {
    todo!() // cxx:__ZNK11mjCActuator10get_targetEv
}

/// C: mjCActuator::get_slidersite (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1845)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_actuator_get_slidersite(self_ptr: *mut mjCActuator) -> *const std__string {
    todo!() // cxx:__ZNK11mjCActuator14get_slidersiteEv
}

/// C: mjCActuator::get_refsite (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1846)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_actuator_get_refsite(self_ptr: *mut mjCActuator) -> *const std__string {
    todo!() // cxx:__ZNK11mjCActuator11get_refsiteEv
}

/// C: mjCActuator::is_ctrllimited (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1848)
/// Calls: cxx-internal:user_objects.cc.o:__ZL9islimitediPKd
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_actuator_is_ctrllimited(self_ptr: *mut mjCActuator) -> bool {
    todo!() // cxx:__ZNK11mjCActuator14is_ctrllimitedEv
}

/// C: mjCActuator::is_forcelimited (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1849)
/// Calls: cxx-internal:user_objects.cc.o:__ZL9islimitediPKd
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_actuator_is_forcelimited(self_ptr: *mut mjCActuator) -> bool {
    todo!() // cxx:__ZNK11mjCActuator15is_forcelimitedEv
}

/// C: mjCActuator::is_actlimited (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1850)
/// Calls: cxx-internal:user_objects.cc.o:__ZL9islimitediPKd
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_actuator_is_actlimited(self_ptr: *mut mjCActuator) -> bool {
    todo!() // cxx:__ZNK11mjCActuator13is_actlimitedEv
}

/// C: mjCActuator::act (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1852)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_actuator_act(self_ptr: *mut mjCActuator, state_name: *const std__string) -> *const () {
    todo!() // cxx:__ZN11mjCActuator3actERKNSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEE
}

/// C: mjCActuator::ctrl (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1853)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_actuator_ctrl(self_ptr: *mut mjCActuator, state_name: *const std__string) -> *mut f64 {
    todo!() // cxx:__ZN11mjCActuator4ctrlERKNSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEE
}

/// C: mjCActuator::Compile (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1856)
/// Calls: cxx-internal:user_objects.cc.o:__ZL12checklimitedPK7mjCBasebPKcS3_ib, cxx:__ZN11mjCActuator12CopyFromSpecEv, cxx:__ZN11mjCActuator17ResolveReferencesEPK8mjCModel, cxx:__ZN8mjCModel13ResolvePluginEP7mjCBaseRKNSt3__112basic_stringIcNS2_11char_traitsIcEENS2_9allocatorIcEEEESA_PP9mjCPlugin, cxx:__ZN9mjCTendon9get_rangeEv, cxx:__ZNK11mjCActuator13is_actlimitedEv, cxx:__ZNK11mjCActuator14is_ctrllimitedEv, cxx:__ZNK11mjCActuator15is_forcelimitedEv, cxx:__ZNK8mjCJoint9get_rangeEv, cxx:_mjp_getPluginAtSlot
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_actuator_compile(self_ptr: *mut mjCActuator) {
    todo!() // cxx:__ZN11mjCActuator7CompileEv
}

/// C: mjCActuator::CopyFromSpec (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1857)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_actuator_copy_from_spec(self_ptr: *mut mjCActuator) {
    todo!() // cxx:__ZN11mjCActuator12CopyFromSpecEv
}

/// C: mjCActuator::PointToLocal (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1858)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_actuator_point_to_local(self_ptr: *mut mjCActuator) {
    todo!() // cxx:__ZN11mjCActuator12PointToLocalEv
}

/// C: mjCActuator::ResolveReferences (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1859)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_actuator_resolve_references(self_ptr: *mut mjCActuator, m: *const mjCModel) {
    todo!() // cxx:__ZN11mjCActuator17ResolveReferencesEPK8mjCModel
}

/// C: mjCActuator::NameSpace (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1860)
/// Calls: cxx:__ZN7mjCBase9NameSpaceEPK8mjCModel
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_actuator_name_space(self_ptr: *mut mjCActuator, m: *const mjCModel) {
    todo!() // cxx:__ZN11mjCActuator9NameSpaceEPK8mjCModel
}

/// C: mjCActuator::CopyPlugin (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1861)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_actuator_copy_plugin(self_ptr: *mut mjCActuator) {
    todo!() // cxx:__ZN11mjCActuator10CopyPluginEv
}

/// C: mjCActuator::ForgetKeyframes (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1864)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_actuator_forget_keyframes(self_ptr: *mut mjCActuator) {
    todo!() // cxx:__ZN11mjCActuator15ForgetKeyframesEv
}

/// C: mjCSensor::get_userdata (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1901)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_sensor_get_userdata(self_ptr: *mut mjCSensor) -> *const () {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).userdata_ }
}

/// C: mjCSensor::get_objname (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1902)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_sensor_get_objname(self_ptr: *mut mjCSensor) -> *const std__string {
    todo!() // cxx:__ZN9mjCSensor11get_objnameEv
}

/// C: mjCSensor::get_refname (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1903)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_sensor_get_refname(self_ptr: *mut mjCSensor) -> *const std__string {
    todo!() // cxx:__ZN9mjCSensor11get_refnameEv
}

/// C: mjCSensor::get_obj (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1905)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_sensor_get_obj(self_ptr: *mut mjCSensor) -> *const mjCBase {
    todo!() // cxx:__ZN9mjCSensor7get_objEv
}

/// C: mjCSensor::get_ref (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1906)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_sensor_get_ref(self_ptr: *mut mjCSensor) -> *const mjCBase {
    todo!() // cxx:__ZN9mjCSensor7get_refEv
}

/// C: mjCSensor::Compile (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1909)
/// Calls: cxx:__Z14sensorDatatype9mjtSensor, cxx:__Z15sensorNeedstage9mjtSensor, cxx:__ZN8mjCModel13ResolvePluginEP7mjCBaseRKNSt3__112basic_stringIcNS2_11char_traitsIcEENS2_9allocatorIcEEEESA_PP9mjCPlugin, cxx:__ZN9mjCSensor12CopyFromSpecEv, cxx:__ZN9mjCSensor17ResolveReferencesEPK8mjCModel, cxx:__ZNK7mjCGeom4TypeEv, cxx:__ZNK8mjCJoint10is_limitedEv, cxx:__ZNK9mjCTendon10is_limitedEv, cxx:_mjp_getPluginAtSlot, cxx:_mjs_sensorDim
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_sensor_compile(self_ptr: *mut mjCSensor) {
    todo!() // cxx:__ZN9mjCSensor7CompileEv
}

/// C: mjCSensor::CopyFromSpec (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1910)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_sensor_copy_from_spec(self_ptr: *mut mjCSensor) {
    todo!() // cxx:__ZN9mjCSensor12CopyFromSpecEv
}

/// C: mjCSensor::PointToLocal (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1911)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_sensor_point_to_local(self_ptr: *mut mjCSensor) {
    todo!() // cxx:__ZN9mjCSensor12PointToLocalEv
}

/// C: mjCSensor::ResolveReferences (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1912)
/// Calls: cxx:__ZN7mjCGeom12SetNotVisualEv, cxx:__ZN7mjCMesh12SetNotVisualEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_sensor_resolve_references(self_ptr: *mut mjCSensor, m: *const mjCModel) {
    todo!() // cxx:__ZN9mjCSensor17ResolveReferencesEPK8mjCModel
}

/// C: mjCSensor::NameSpace (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1913)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_sensor_name_space(self_ptr: *mut mjCSensor, m: *const mjCModel) {
    todo!() // cxx:__ZN9mjCSensor9NameSpaceEPK8mjCModel
}

/// C: mjCSensor::CopyPlugin (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1914)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_sensor_copy_plugin(self_ptr: *mut mjCSensor) {
    todo!() // cxx:__ZN9mjCSensor10CopyPluginEv
}

/// C: mjCNumeric::PointToLocal (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1944)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_numeric_point_to_local(self_ptr: *mut mjCNumeric) {
    todo!() // cxx:__ZN10mjCNumeric12PointToLocalEv
}

/// C: mjCNumeric::CopyFromSpec (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1945)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_numeric_copy_from_spec(self_ptr: *mut mjCNumeric) {
    todo!() // cxx:__ZN10mjCNumeric12CopyFromSpecEv
}

/// C: mjCNumeric::Compile (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1948)
/// Calls: cxx:__ZN10mjCNumeric12CopyFromSpecEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_numeric_compile(self_ptr: *mut mjCNumeric) {
    todo!() // cxx:__ZN10mjCNumeric7CompileEv
}

/// C: mjCText::PointToLocal (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1975)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_text_point_to_local(self_ptr: *mut mjCText) {
    todo!() // cxx:__ZN7mjCText12PointToLocalEv
}

/// C: mjCText::CopyFromSpec (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1976)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_text_copy_from_spec(self_ptr: *mut mjCText) {
    todo!() // cxx:__ZN7mjCText12CopyFromSpecEv
}

/// C: mjCText::Compile (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:1979)
/// Calls: cxx:__ZN7mjCText12CopyFromSpecEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_text_compile(self_ptr: *mut mjCText) {
    todo!() // cxx:__ZN7mjCText7CompileEv
}

/// C: mjCTuple::PointToLocal (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:2011)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tuple_point_to_local(self_ptr: *mut mjCTuple) {
    todo!() // cxx:__ZN8mjCTuple12PointToLocalEv
}

/// C: mjCTuple::CopyFromSpec (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:2012)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tuple_copy_from_spec(self_ptr: *mut mjCTuple) {
    todo!() // cxx:__ZN8mjCTuple12CopyFromSpecEv
}

/// C: mjCTuple::ResolveReferences (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:2013)
/// Calls: cxx:__ZN7mjCGeom12SetNotVisualEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tuple_resolve_references(self_ptr: *mut mjCTuple, m: *const mjCModel) {
    todo!() // cxx:__ZN8mjCTuple17ResolveReferencesEPK8mjCModel
}

/// C: mjCTuple::NameSpace (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:2014)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tuple_name_space(self_ptr: *mut mjCTuple, m: *const mjCModel) {
    todo!() // cxx:__ZN8mjCTuple9NameSpaceEPK8mjCModel
}

/// C: mjCTuple::Compile (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:2017)
/// Calls: cxx:__ZN8mjCTuple12CopyFromSpecEv, cxx:__ZN8mjCTuple17ResolveReferencesEPK8mjCModel
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tuple_compile(self_ptr: *mut mjCTuple) {
    todo!() // cxx:__ZN8mjCTuple7CompileEv
}

/// C: mjCKey::PointToLocal (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:2054)
/// Calls: cxx:__ZN11mjCActuator3actERKNSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEE, cxx:__ZN11mjCActuator4ctrlERKNSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEE, cxx:__ZN7mjCBody4mposERKNSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEE, cxx:__ZN7mjCBody5mquatERKNSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEE, cxx:__ZN8mjCJoint4qposERKNSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEE, cxx:__ZN8mjCJoint4qvelERKNSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEE
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_key_point_to_local(self_ptr: *mut mjCKey) {
    todo!() // cxx:__ZN6mjCKey12PointToLocalEv
}

/// C: mjCKey::CopyFromSpec (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:2055)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_key_copy_from_spec(self_ptr: *mut mjCKey) {
    todo!() // cxx:__ZN6mjCKey12CopyFromSpecEv
}

/// C: mjCKey::Compile (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:2058)
/// Calls: cxx:__ZN6mjCKey12CopyFromSpecEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_key_compile(self_ptr: *mut mjCKey, m: *const mjModel) {
    todo!() // cxx:__ZN6mjCKey7CompileEPK8mjModel_
}

/// C: mjCDef::CopyWithoutChildren (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:2076)
/// Calls: cxx:__ZN6mjCDef12PointToLocalEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_def_copy_without_children(self_ptr: *mut mjCDef, other: *const mjCDef) {
    todo!() // cxx:__ZN6mjCDef19CopyWithoutChildrenERKS_
}

/// C: mjCDef::PointToLocal (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:2077)
/// Calls: cxx:__ZN11mjCActuator12PointToLocalEv, cxx:__ZN11mjCEquality12PointToLocalEv, cxx:__ZN11mjCMaterial12PointToLocalEv, cxx:__ZN7mjCFlex12PointToLocalEv, cxx:__ZN7mjCGeom12PointToLocalEv, cxx:__ZN7mjCMesh12PointToLocalEv, cxx:__ZN7mjCPair12PointToLocalEv, cxx:__ZN7mjCSite12PointToLocalEv, cxx:__ZN8mjCJoint12PointToLocalEv, cxx:__ZN8mjCLight12PointToLocalEv, cxx:__ZN9mjCCamera12PointToLocalEv, cxx:__ZN9mjCTendon12PointToLocalEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_def_point_to_local(self_ptr: *mut mjCDef) {
    todo!() // cxx:__ZN6mjCDef12PointToLocalEv
}

/// C: mjCDef::CopyFromSpec (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:2078)
/// Calls: cxx:__ZN11mjCActuator12CopyFromSpecEv, cxx:__ZN11mjCEquality12CopyFromSpecEv, cxx:__ZN11mjCMaterial12CopyFromSpecEv, cxx:__ZN7mjCFlex12CopyFromSpecEv, cxx:__ZN7mjCGeom12CopyFromSpecEv, cxx:__ZN7mjCMesh12CopyFromSpecEv, cxx:__ZN7mjCPair12CopyFromSpecEv, cxx:__ZN7mjCSite12CopyFromSpecEv, cxx:__ZN8mjCJoint12CopyFromSpecEv, cxx:__ZN8mjCLight12CopyFromSpecEv, cxx:__ZN9mjCCamera12CopyFromSpecEv, cxx:__ZN9mjCTendon12CopyFromSpecEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_def_copy_from_spec(self_ptr: *mut mjCDef) {
    todo!() // cxx:__ZN6mjCDef12CopyFromSpecEv
}

/// C: mjCDef::NameSpace (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:2079)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_def_name_space(self_ptr: *mut mjCDef, m: *const mjCModel) {
    todo!() // cxx:__ZN6mjCDef9NameSpaceEPK8mjCModel
}

/// C: mjCDef::Compile (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:2081)
/// Calls: cxx:__ZN6mjCDef12CopyFromSpecEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_def_compile(self_ptr: *mut mjCDef, model: *const mjCModel) {
    todo!() // cxx:__ZN6mjCDef7CompileEPK8mjCModel
}

/// C: mjCDef::Joint (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:2084)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_def_joint(self_ptr: *mut mjCDef) -> *mut mjCJoint {
    todo!() // cxx:__ZN6mjCDef5JointEv
}

/// C: mjCDef::Geom (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:2085)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_def_geom(self_ptr: *mut mjCDef) -> *mut mjCGeom {
    todo!() // cxx:__ZN6mjCDef4GeomEv
}

/// C: mjCDef::Site (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:2086)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_def_site(self_ptr: *mut mjCDef) -> *mut mjCSite {
    todo!() // cxx:__ZN6mjCDef4SiteEv
}

/// C: mjCDef::Camera (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:2087)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_def_camera(self_ptr: *mut mjCDef) -> *mut mjCCamera {
    todo!() // cxx:__ZN6mjCDef6CameraEv
}

/// C: mjCDef::Light (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:2088)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_def_light(self_ptr: *mut mjCDef) -> *mut mjCLight {
    todo!() // cxx:__ZN6mjCDef5LightEv
}

/// C: mjCDef::Flex (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:2089)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_def_flex(self_ptr: *mut mjCDef) -> *mut mjCFlex {
    todo!() // cxx:__ZN6mjCDef4FlexEv
}

/// C: mjCDef::Mesh (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:2090)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_def_mesh(self_ptr: *mut mjCDef) -> *mut mjCMesh {
    todo!() // cxx:__ZN6mjCDef4MeshEv
}

/// C: mjCDef::Material (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:2091)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_def_material(self_ptr: *mut mjCDef) -> *mut mjCMaterial {
    todo!() // cxx:__ZN6mjCDef8MaterialEv
}

/// C: mjCDef::Pair (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:2092)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_def_pair(self_ptr: *mut mjCDef) -> *mut mjCPair {
    todo!() // cxx:__ZN6mjCDef4PairEv
}

/// C: mjCDef::Equality (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:2093)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_def_equality(self_ptr: *mut mjCDef) -> *mut mjCEquality {
    todo!() // cxx:__ZN6mjCDef8EqualityEv
}

/// C: mjCDef::Tendon (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:2094)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_def_tendon(self_ptr: *mut mjCDef) -> *mut mjCTendon {
    todo!() // cxx:__ZN6mjCDef6TendonEv
}

/// C: mjCDef::Actuator (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.h:2095)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_def_actuator(self_ptr: *mut mjCDef) -> *mut mjCActuator {
    todo!() // cxx:__ZN6mjCDef8ActuatorEv
}

