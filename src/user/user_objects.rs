//! Port of: user/user_objects.cc
//! IR hash: fd5288ad7242f001
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: PNGImage::Load (user/user_objects.cc:58)
/// Calls: cxx:_mju_readResource
#[allow(unused_variables, non_snake_case)]
pub fn png_image_load(obj: *const mjCBase, resource: *mut mjResource, color_type: u32) -> PNGImage {
    todo!() // cxx-internal:user_objects.cc.o:__ZN12_GLOBAL__N_18PNGImage4LoadEPK7mjCBaseP11mjResource_16LodePNGColorType
}

/// C: PNGImage::Width (user/user_objects.cc:60)
#[allow(unused_variables, non_snake_case)]
pub fn png_image_width(self_ptr: *mut PNGImage) -> i32 {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).width_ }
}

/// C: PNGImage::Height (user/user_objects.cc:61)
#[allow(unused_variables, non_snake_case)]
pub fn png_image_height(self_ptr: *mut PNGImage) -> i32 {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).height_ }
}

/// C: PNGImage::IsSRGB (user/user_objects.cc:62)
#[allow(unused_variables, non_snake_case)]
pub fn png_image_is_srgb(self_ptr: *mut PNGImage) -> bool {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).is_srgb_ }
}

/// C: PNGImage::MoveData (user/user_objects.cc:66)
#[allow(unused_variables, non_snake_case)]
pub fn png_image_move_data(self_ptr: *mut PNGImage) -> *mut *mut mjByteVec {
    todo!() // cxx-internal:user_objects.cc.o:__ZNO12_GLOBAL__N_18PNGImage8MoveDataEv
}

/// C: PNGImage::Size (user/user_objects.cc:69)
#[allow(unused_variables, non_snake_case)]
pub fn png_image_size(self_ptr: *mut PNGImage) -> u64 {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.cc:__ZNK12_GLOBAL__N_18PNGImage4SizeEv
}

/// C: MapFrame (user/user_objects.cc:134)
/// Calls: cxx:__ZN7mjCBase8SetFrameEP8mjCFrame, cxx:__ZN7mjCBody9SetParentEPS_
#[allow(unused_variables, non_snake_case)]
pub fn map_frame__2b9b425667946bd2(parent: *const (), child: *const (), frame: *mut mjCFrame, parent_body: *mut mjCBody) {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.cc:__ZN12_GLOBAL__N_18MapFrameI7mjCBodyEEvRNSt3__16vectorIPT_NS2_9allocatorIS5_EEEES9_P8mjCFramePS1_
}

/// C: MapFrame (user/user_objects.cc:134)
/// Calls: cxx:__ZN7mjCBase8SetFrameEP8mjCFrame, cxx:__ZN7mjCGeom9SetParentEP7mjCBody
#[allow(unused_variables, non_snake_case)]
pub fn map_frame__c722cf8e7cdebcc5(parent: *const (), child: *const (), frame: *mut mjCFrame, parent_body: *mut mjCBody) {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.cc:__ZN12_GLOBAL__N_18MapFrameI7mjCGeomEEvRNSt3__16vectorIPT_NS2_9allocatorIS5_EEEES9_P8mjCFrameP7mjCBody
}

/// C: MapFrame (user/user_objects.cc:134)
/// Calls: cxx:__ZN7mjCBase8SetFrameEP8mjCFrame, cxx:__ZN7mjCSite9SetParentEP7mjCBody
#[allow(unused_variables, non_snake_case)]
pub fn map_frame__a014059b583f0355(parent: *const (), child: *const (), frame: *mut mjCFrame, parent_body: *mut mjCBody) {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.cc:__ZN12_GLOBAL__N_18MapFrameI7mjCSiteEEvRNSt3__16vectorIPT_NS2_9allocatorIS5_EEEES9_P8mjCFrameP7mjCBody
}

/// C: MapFrame (user/user_objects.cc:134)
/// Calls: cxx:__ZN7mjCBase8SetFrameEP8mjCFrame, cxx:__ZN8mjCFrame9SetParentEP7mjCBody
#[allow(unused_variables, non_snake_case)]
pub fn map_frame__14b8b24d43042c2e(parent: *const (), child: *const (), frame: *mut mjCFrame, parent_body: *mut mjCBody) {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.cc:__ZN12_GLOBAL__N_18MapFrameI8mjCFrameEEvRNSt3__16vectorIPT_NS2_9allocatorIS5_EEEES9_PS1_P7mjCBody
}

/// C: MapFrame (user/user_objects.cc:134)
/// Calls: cxx:__ZN7mjCBase8SetFrameEP8mjCFrame, cxx:__ZN8mjCJoint9SetParentEP7mjCBody
#[allow(unused_variables, non_snake_case)]
pub fn map_frame__e91049cb8c10739e(parent: *const (), child: *const (), frame: *mut mjCFrame, parent_body: *mut mjCBody) {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.cc:__ZN12_GLOBAL__N_18MapFrameI8mjCJointEEvRNSt3__16vectorIPT_NS2_9allocatorIS5_EEEES9_P8mjCFrameP7mjCBody
}

/// C: MapFrame (user/user_objects.cc:134)
/// Calls: cxx:__ZN7mjCBase8SetFrameEP8mjCFrame, cxx:__ZN8mjCLight9SetParentEP7mjCBody
#[allow(unused_variables, non_snake_case)]
pub fn map_frame__11ba1c2088b8bffa(parent: *const (), child: *const (), frame: *mut mjCFrame, parent_body: *mut mjCBody) {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.cc:__ZN12_GLOBAL__N_18MapFrameI8mjCLightEEvRNSt3__16vectorIPT_NS2_9allocatorIS5_EEEES9_P8mjCFrameP7mjCBody
}

/// C: MapFrame (user/user_objects.cc:134)
/// Calls: cxx:__ZN7mjCBase8SetFrameEP8mjCFrame, cxx:__ZN9mjCCamera9SetParentEP7mjCBody
#[allow(unused_variables, non_snake_case)]
pub fn map_frame__16b68a23a2391410(parent: *const (), child: *const (), frame: *mut mjCFrame, parent_body: *mut mjCBody) {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.cc:__ZN12_GLOBAL__N_18MapFrameI9mjCCameraEEvRNSt3__16vectorIPT_NS2_9allocatorIS5_EEEES9_P8mjCFrameP7mjCBody
}

/// C: checksize (user/user_objects.cc:150)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn checksize(size: *mut f64, r#type: u32, object: *mut mjCBase, name: *const i8, id: i32) {
    todo!() // cxx-internal:user_objects.cc.o:__ZL9checksizePd7mjtGeomP7mjCBasePKci
}

/// C: checklimited (user/user_objects.cc:165)
#[allow(unused_variables, non_snake_case)]
pub fn checklimited(obj: *const mjCBase, autolimits: bool, entity: *const i8, attr: *const i8, limited: i32, hasrange: bool) {
    todo!() // cxx-internal:user_objects.cc.o:__ZL12checklimitedPK7mjCBasebPKcS3_ib
}

/// C: islimited (user/user_objects.cc:181)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn islimited(limited: i32, range: *const f64) -> bool {
    // SAFETY: caller guarantees range points to at least 2 contiguous f64 elements
    unsafe {
        if limited == 1 || (limited == 2 && *range.add(0) < *range.add(1)) {
            return true;
        }
        false
    }
}

/// C: ResolveOrientation (user/user_objects.cc:240)
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

/// C: mjCBoundingVolumeHierarchy::Set (user/user_objects.cc:340)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_hierarchy_set(self_ptr: *mut mjCBoundingVolumeHierarchy, ipos_element: *mut f64, iquat_element: *mut f64) {
    todo!() // cxx:__ZN26mjCBoundingVolumeHierarchy3SetEPdS0_
}

/// C: mjCBoundingVolumeHierarchy::AllocateBoundingVolumes (user/user_objects.cc:346)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_hierarchy_allocate_bounding_volumes(self_ptr: *mut mjCBoundingVolumeHierarchy, nleaf: i32) {
    todo!() // cxx:__ZN26mjCBoundingVolumeHierarchy23AllocateBoundingVolumesEi
}

/// C: mjCBoundingVolumeHierarchy::RemoveInactiveVolumes (user/user_objects.cc:357)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_hierarchy_remove_inactive_volumes(self_ptr: *mut mjCBoundingVolumeHierarchy, nmax: i32) {
    todo!() // cxx:__ZN26mjCBoundingVolumeHierarchy21RemoveInactiveVolumesEi
}

/// C: mjCBoundingVolumeHierarchy::AddBoundingVolume (user/user_objects.cc:367)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_hierarchy_add_bounding_volume__4bfb235aafdb0eeb(self_ptr: *mut mjCBoundingVolumeHierarchy, id: i32, contype: i32, conaffinity: i32, pos: *const f64, quat: *const f64, aabb: *const f64) -> *const mjCBoundingVolume {
    todo!() // cxx:__ZN26mjCBoundingVolumeHierarchy17AddBoundingVolumeEiiiPKdS1_S1_
}

/// C: mjCBoundingVolumeHierarchy::AddBoundingVolume (user/user_objects.cc:378)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_hierarchy_add_bounding_volume__a0616ff3267405ea(self_ptr: *mut mjCBoundingVolumeHierarchy, id: *const i32, contype: i32, conaffinity: i32, pos: *const f64, quat: *const f64, aabb: *const f64) -> *const mjCBoundingVolume {
    todo!() // cxx:__ZN26mjCBoundingVolumeHierarchy17AddBoundingVolumeEPKiiiPKdS3_S3_
}

/// C: mjCBoundingVolumeHierarchy::CreateBVH (user/user_objects.cc:385)
/// Calls: cxx:__ZN26mjCBoundingVolumeHierarchy4MakeERNSt3__16vectorINS_9BVElementENS0_9allocatorIS2_EEEE, cxx:__ZN26mjCBoundingVolumeHierarchy7MakeBVHENSt3__111__wrap_iterIPNS_9BVElementEEES4_iP8mjCModelPK7mjCBase
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_hierarchy_create_bvh(self_ptr: *mut mjCBoundingVolumeHierarchy, model: *mut mjCModel, owner: *const mjCBase) {
    todo!() // cxx:__ZN26mjCBoundingVolumeHierarchy9CreateBVHEP8mjCModelPK7mjCBase
}

/// C: mjCBoundingVolumeHierarchy::Make (user/user_objects.cc:391)
/// Calls: cxx:__Z15mjuu_rotVecQuatPdPKdS1_, cxx:__ZNK17mjCBoundingVolume11ConaffinityEv, cxx:__ZNK17mjCBoundingVolume7ContypeEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_hierarchy_make(self_ptr: *mut mjCBoundingVolumeHierarchy, elements: *const ()) {
    todo!() // cxx:__ZN26mjCBoundingVolumeHierarchy4MakeERNSt3__16vectorINS_9BVElementENS0_9allocatorIS2_EEEE
}

/// C: mjCBoundingVolumeHierarchy::MakeBVH (user/user_objects.cc:416)
/// Calls: cxx:__Z15mjuu_rotVecQuatPdPKdS1_, cxx:__ZN8mjCModel10AddWarningENSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEEPK7mjCBase, cxx:__ZNK17mjCBoundingVolume2IdEv, cxx:__ZNK17mjCBoundingVolume4QuatEv, cxx:_mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_hierarchy_make_bvh(self_ptr: *mut mjCBoundingVolumeHierarchy, elements_begin: *const (), elements_end: *const (), lev: i32, model: *mut mjCModel, owner: *const mjCBase) -> i32 {
    todo!() // cxx:__ZN26mjCBoundingVolumeHierarchy7MakeBVHENSt3__111__wrap_iterIPNS_9BVElementEEES4_iP8mjCModelPK7mjCBase
}

/// C: mjCOctree::CopyLevel (user/user_objects.cc:531)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_copy_level(self_ptr: *mut mjCOctree, level: *mut i32) {
    todo!() // cxx:__ZNK9mjCOctree9CopyLevelEPi
}

/// C: mjCOctree::CopyChild (user/user_objects.cc:536)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_copy_child(self_ptr: *mut mjCOctree, child: *mut i32) {
    todo!() // cxx:__ZNK9mjCOctree9CopyChildEPi
}

/// C: mjCOctree::CopyAabb (user/user_objects.cc:543)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_copy_aabb(self_ptr: *mut mjCOctree, aabb: *mut f64) {
    todo!() // cxx:__ZNK9mjCOctree8CopyAabbEPd
}

/// C: mjCOctree::CopyCoeff (user/user_objects.cc:555)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_copy_coeff(self_ptr: *mut mjCOctree, coeff: *mut f64) {
    todo!() // cxx:__ZNK9mjCOctree9CopyCoeffEPd
}

/// C: mjCOctree::SetFace (user/user_objects.cc:562)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_set_face(self_ptr: *mut mjCOctree, vert: *const (), face: *const ()) {
    todo!() // cxx:__ZN9mjCOctree7SetFaceERKNSt3__16vectorIdNS0_9allocatorIdEEEERKNS1_IiNS2_IiEEEE
}

/// C: mjCOctree::Make (user/user_objects.cc:580)
/// Calls: cxx:__Z15mjuu_rotVecQuatPdPKdS1_
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_make(self_ptr: *mut mjCOctree, elements: *const ()) {
    todo!() // cxx:__ZN9mjCOctree4MakeERNSt3__16vectorINS0_5arrayINS2_IdLm3EEELm3EEENS0_9allocatorIS4_EEEE
}

/// C: mjCOctree::CreateOctree (user/user_objects.cc:595)
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

/// C: pointBoxDistSq (user/user_objects.cc:625)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn point_box_dist_sq(p: *const f64, aabb: *const f64) -> f64 {
    todo!() // cxx-internal:user_objects.cc.o:__ZN12_GLOBAL__N_114pointBoxDistSqEPKdS1_
}

/// C: pointTriDistSqWithUV (user/user_objects.cc:642)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/ui/ui_main.c:_inside, cxx-internal:user_mesh.cc.o:__ZL8trianglePdS_PKdS1_S1_
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn point_tri_dist_sq_with_uv(p: *const f64, v0: *const f64, v1: *const f64, v2: *const f64, out_u: *mut f64, out_v: *mut f64) -> f64 {
    todo!() // cxx-internal:user_objects.cc.o:__ZN12_GLOBAL__N_120pointTriDistSqWithUVEPKdS1_S1_S1_RdS2_
}

/// C: queryClosestBVHWithFace (user/user_objects.cc:738)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.cc:__ZN12_GLOBAL__N_114pointBoxDistSqEPKdS1_, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.cc:__ZN12_GLOBAL__N_120pointTriDistSqWithUVEPKdS1_S1_S1_RdS2_
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn query_closest_bvh_with_face(bvh: *const f64, child: *const i32, nodeid: *const i32, vert: *const f64, face: *const i32, node_idx: i32, p: *const f64, best_dist_sq: *mut f64, best_face: *mut i32, best_u: *mut f64, best_v: *mut f64) {
    todo!() // cxx-internal:user_objects.cc.o:__ZN12_GLOBAL__N_123queryClosestBVHWithFaceEPKdPKiS3_S1_S3_iS1_RdRiS4_S4_
}

/// C: querySignedDistance (user/user_objects.cc:802)
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

/// C: mjCBoundingVolumeHierarchy::QuerySignedDistance (user/user_objects.cc:858)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.cc:__ZN12_GLOBAL__N_119querySignedDistanceEPKdPKiS3_iS1_S1_S3_
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_hierarchy_query_signed_distance(self_ptr: *mut mjCBoundingVolumeHierarchy, point: *const f64, vert: *const f64, face: *const i32) -> f64 {
    todo!() // cxx:__ZNK26mjCBoundingVolumeHierarchy19QuerySignedDistanceEPKdS1_PKi
}

/// C: mjCOctree::ComputeSdfCoeffs (user/user_objects.cc:867)
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

/// C: dot2 (user/user_objects.cc:958)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn dot2(a: *const f64, b: *const f64) -> f64 {
    // SAFETY: a and b are valid pointers to at least 2 f64 elements
    unsafe {
        *a.offset(0) * *b.offset(0) + *a.offset(1) * *b.offset(1)
    }
}

/// C: boxTriangle (user/user_objects.cc:964)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.cc:__ZL4dot2PKdS0_, cxx:__Z13mjuu_crossvecPdPKdS1_, cxx:__Z9mjuu_dot3PKdS0_, cxx:_mju_max
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn box_triangle(v: *const Triangle, aamm: *const f64) -> bool {
    todo!() // cxx-internal:user_objects.cc.o:__ZL11boxTriangleRKNSt3__15arrayINS0_IdLm3EEELm3EEEPKd
}

/// C: mjCOctree::TaskToNode (user/user_objects.cc:1009)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_task_to_node(self_ptr: *mut mjCOctree, task: *const OctreeTask, node: *mut OctNode, vert_map: *const ()) {
    todo!() // cxx:__ZN9mjCOctree10TaskToNodeERK10OctreeTaskR7OctNodeRNSt3__113unordered_mapI5PointiNS5_4hashIS7_EENS5_8equal_toIS7_EENS5_9allocatorINS5_4pairIKS7_iEEEEEE
}

/// C: mjCOctree::Subdivide (user/user_objects.cc:1048)
/// Calls: cxx:__ZN9mjCOctree10TaskToNodeERK10OctreeTaskR7OctNodeRNSt3__113unordered_mapI5PointiNS5_4hashIS7_EENS5_8equal_toIS7_EENS5_9allocatorINS5_4pairIKS7_iEEEEEE
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_subdivide(self_ptr: *mut mjCOctree, task: *const OctreeTask, vert_map: *const (), queue: *const (), colliding: *const ()) {
    todo!() // cxx:__ZN9mjCOctree9SubdivideERK10OctreeTaskRNSt3__113unordered_mapI5PointiNS3_4hashIS5_EENS3_8equal_toIS5_EENS3_9allocatorINS3_4pairIKS5_iEEEEEEPNS3_5dequeIS0_NSA_IS0_EEEERKNS3_6vectorIPNS3_5arrayINSM_IdLm3EEELm3EEENSA_ISP_EEEE
}

/// C: mjCOctree::FindCoarseNeighbor (user/user_objects.cc:1065)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_find_coarse_neighbor(self_ptr: *mut mjCOctree, node_idx: i32, dir: i32) -> i32 {
    todo!() // cxx:__ZN9mjCOctree18FindCoarseNeighborEii
}

/// C: mjCOctree::FindNeighbor (user/user_objects.cc:1090)
/// Calls: cxx:__ZN9mjCOctree18FindCoarseNeighborEii
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_find_neighbor(self_ptr: *mut mjCOctree, node_idx: i32, dir: i32) -> i32 {
    todo!() // cxx:__ZN9mjCOctree12FindNeighborEii
}

/// C: mjCOctree::BalanceOctree (user/user_objects.cc:1138)
/// Calls: cxx:__ZN9mjCOctree12FindNeighborEii, cxx:__ZN9mjCOctree9SubdivideERK10OctreeTaskRNSt3__113unordered_mapI5PointiNS3_4hashIS5_EENS3_8equal_toIS5_EENS3_9allocatorINS3_4pairIKS5_iEEEEEEPNS3_5dequeIS0_NSA_IS0_EEEERKNS3_6vectorIPNS3_5arrayINSM_IdLm3EEELm3EEENSA_ISP_EEEE
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_balance_octree(self_ptr: *mut mjCOctree, vert_map: *const ()) {
    todo!() // cxx:__ZN9mjCOctree13BalanceOctreeERNSt3__113unordered_mapI5PointiNS0_4hashIS2_EENS0_8equal_toIS2_EENS0_9allocatorINS0_4pairIKS2_iEEEEEE
}

/// C: mjCOctree::MarkHangingNodes (user/user_objects.cc:1180)
/// Calls: cxx:__ZN9mjCOctree12FindNeighborEii
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_mark_hanging_nodes(self_ptr: *mut mjCOctree) {
    todo!() // cxx:__ZN9mjCOctree16MarkHangingNodesEv
}

/// C: mjCOctree::MakeOctree (user/user_objects.cc:1269)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.cc:__ZL11boxTriangleRKNSt3__15arrayINS0_IdLm3EEELm3EEEPKd, cxx:__ZN9mjCOctree10TaskToNodeERK10OctreeTaskR7OctNodeRNSt3__113unordered_mapI5PointiNS5_4hashIS7_EENS5_8equal_toIS7_EENS5_9allocatorINS5_4pairIKS7_iEEEEEE, cxx:__ZN9mjCOctree13BalanceOctreeERNSt3__113unordered_mapI5PointiNS0_4hashIS2_EENS0_8equal_toIS2_EENS0_9allocatorINS0_4pairIKS2_iEEEEEE, cxx:__ZN9mjCOctree9SubdivideERK10OctreeTaskRNSt3__113unordered_mapI5PointiNS3_4hashIS5_EENS3_8equal_toIS5_EENS3_9allocatorINS3_4pairIKS5_iEEEEEEPNS3_5dequeIS0_NSA_IS0_EEEERKNS3_6vectorIPNS3_5arrayINSM_IdLm3EEELm3EEENSA_ISP_EEEE
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_make_octree(self_ptr: *mut mjCOctree, elements: *const (), aamm: *const f64, vert_map: *const ()) {
    todo!() // cxx:__ZN9mjCOctree10MakeOctreeERKNSt3__16vectorIPNS0_5arrayINS2_IdLm3EEELm3EEENS0_9allocatorIS5_EEEEPKdRNS0_13unordered_mapI5PointiNS0_4hashISE_EENS0_8equal_toISE_EENS6_INS0_4pairIKSE_iEEEEEE
}

/// C: mjCDef::Compile (user/user_objects.cc:1349)
/// Calls: cxx:__ZN6mjCDef12CopyFromSpecEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_def_compile(self_ptr: *mut mjCDef, model: *const mjCModel) {
    todo!() // cxx:__ZN6mjCDef7CompileEPK8mjCModel
}

/// C: mjCDef::NameSpace (user/user_objects.cc:1383)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_def_name_space(self_ptr: *mut mjCDef, m: *const mjCModel) {
    todo!() // cxx:__ZN6mjCDef9NameSpaceEPK8mjCModel
}

/// C: mjCDef::CopyWithoutChildren (user/user_objects.cc:1389)
/// Calls: cxx:__ZN6mjCDef12PointToLocalEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_def_copy_without_children(self_ptr: *mut mjCDef, other: *const mjCDef) {
    todo!() // cxx:__ZN6mjCDef19CopyWithoutChildrenERKS_
}

/// C: mjCDef::PointToLocal (user/user_objects.cc:1410)
/// Calls: cxx:__ZN11mjCActuator12PointToLocalEv, cxx:__ZN11mjCEquality12PointToLocalEv, cxx:__ZN11mjCMaterial12PointToLocalEv, cxx:__ZN7mjCFlex12PointToLocalEv, cxx:__ZN7mjCGeom12PointToLocalEv, cxx:__ZN7mjCMesh12PointToLocalEv, cxx:__ZN7mjCPair12PointToLocalEv, cxx:__ZN7mjCSite12PointToLocalEv, cxx:__ZN8mjCJoint12PointToLocalEv, cxx:__ZN8mjCLight12PointToLocalEv, cxx:__ZN9mjCCamera12PointToLocalEv, cxx:__ZN9mjCTendon12PointToLocalEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_def_point_to_local(self_ptr: *mut mjCDef) {
    todo!() // cxx:__ZN6mjCDef12PointToLocalEv
}

/// C: mjCDef::CopyFromSpec (user/user_objects.cc:1439)
/// Calls: cxx:__ZN11mjCActuator12CopyFromSpecEv, cxx:__ZN11mjCEquality12CopyFromSpecEv, cxx:__ZN11mjCMaterial12CopyFromSpecEv, cxx:__ZN7mjCFlex12CopyFromSpecEv, cxx:__ZN7mjCGeom12CopyFromSpecEv, cxx:__ZN7mjCMesh12CopyFromSpecEv, cxx:__ZN7mjCPair12CopyFromSpecEv, cxx:__ZN7mjCSite12CopyFromSpecEv, cxx:__ZN8mjCJoint12CopyFromSpecEv, cxx:__ZN8mjCLight12CopyFromSpecEv, cxx:__ZN9mjCCamera12CopyFromSpecEv, cxx:__ZN9mjCTendon12CopyFromSpecEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_def_copy_from_spec(self_ptr: *mut mjCDef) {
    todo!() // cxx:__ZN6mjCDef12CopyFromSpecEv
}

/// C: mjCBase::NameSpace (user/user_objects.cc:1479)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_base_name_space(self_ptr: *mut mjCBase, m: *const mjCModel) {
    todo!() // cxx:__ZN7mjCBase9NameSpaceEPK8mjCModel
}

/// C: mjCBase::FindCompiler (user/user_objects.cc:1487)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_base_find_compiler(self_ptr: *mut mjCBase, compiler: *const mjsCompiler) -> *mut mjsCompiler {
    todo!() // cxx:__ZNK7mjCBase12FindCompilerEPK12mjsCompiler_
}

/// C: mjCBase::LoadResource (user/user_objects.cc:1496)
/// Calls: cxx:_mju_openResource
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_base_load_resource(modelfiledir: *const std__string, filename: *const std__string, vfs: *const mjVFS) -> *mut mjResource {
    todo!() // cxx:__ZN7mjCBase12LoadResourceERKNSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEES8_PK6mjVFS_
}

/// C: mjCBase::GetAssetContentType (user/user_objects.cc:1509)
/// Calls: cxx:__Z21mjuu_extToContentTypeNSt3__117basic_string_viewIcNS_11char_traitsIcEEEE, cxx:__Z29mjuu_parseContentTypeAttrTypeNSt3__117basic_string_viewIcNS_11char_traitsIcEEEE, cxx:__Z32mjuu_parseContentTypeAttrSubtypeNSt3__117basic_string_viewIcNS_11char_traitsIcEEEE
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_base_get_asset_content_type(resource_name: std__string_view, raw_text: std__string_view) -> std__string {
    todo!() // cxx:__ZN7mjCBase19GetAssetContentTypeENSt3__117basic_string_viewIcNS0_11char_traitsIcEEEES4_
}

/// C: mjCBase::SetFrame (user/user_objects.cc:1521)
/// Calls: cxx:__ZNK7mjCBase9GetParentEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_base_set_frame(self_ptr: *mut mjCBase, _frame: *mut mjCFrame) {
    todo!() // cxx:__ZN7mjCBase8SetFrameEP8mjCFrame
}

/// C: mjCBase::SetUserValue (user/user_objects.cc:1529)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_base_set_user_value(self_ptr: *mut mjCBase, key: std__string_view, data: *const (), cleanup: Option<unsafe extern "C" fn(*const ())>) {
    todo!() // cxx:__ZN7mjCBase12SetUserValueENSt3__117basic_string_viewIcNS0_11char_traitsIcEEEEPKvPFvS6_E
}

/// C: mjCBase::GetUserValue (user/user_objects.cc:1533)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_base_get_user_value(self_ptr: *mut mjCBase, key: std__string_view) -> *const () {
    todo!() // cxx:__ZN7mjCBase12GetUserValueENSt3__117basic_string_viewIcNS0_11char_traitsIcEEEE
}

/// C: mjCBase::DeleteUserValue (user/user_objects.cc:1539)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_base_delete_user_value(self_ptr: *mut mjCBase, key: std__string_view) {
    todo!() // cxx:__ZN7mjCBase15DeleteUserValueENSt3__117basic_string_viewIcNS0_11char_traitsIcEEEE
}

/// C: mjCBody::CopyList (user/user_objects.cc:1753)
/// Calls: cxx:__ZN7mjCBase6AddRefEv, cxx:__ZN7mjCGeom10CopyPluginEv, cxx:__ZN7mjCGeom9NameSpaceEPK8mjCModel, cxx:__ZNK8mjCFrame10IsAncestorEPKS_
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_copy_list__bc53ba57fed02259(self_ptr: *mut mjCBody, dst: *const (), src: *const (), fmap: *const (), pframe: *const mjCFrame) {
    todo!() // cxx:__ZN7mjCBody8CopyListI7mjCGeomEEvRNSt3__16vectorIPT_NS2_9allocatorIS5_EEEERKS8_RNS2_3mapIP8mjCFrameiNS2_4lessISE_EENS6_INS2_4pairIKSE_iEEEEEEPKSD_
}

/// C: mjCBody::CopyList (user/user_objects.cc:1753)
/// Calls: cxx:__ZN7mjCBase10CopyPluginEv, cxx:__ZN7mjCBase6AddRefEv, cxx:__ZN7mjCSite9NameSpaceEPK8mjCModel, cxx:__ZNK8mjCFrame10IsAncestorEPKS_
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_copy_list__c0a715032930d1f1(self_ptr: *mut mjCBody, dst: *const (), src: *const (), fmap: *const (), pframe: *const mjCFrame) {
    todo!() // cxx:__ZN7mjCBody8CopyListI7mjCSiteEEvRNSt3__16vectorIPT_NS2_9allocatorIS5_EEEERKS8_RNS2_3mapIP8mjCFrameiNS2_4lessISE_EENS6_INS2_4pairIKSE_iEEEEEEPKSD_
}

/// C: mjCBody::CopyList (user/user_objects.cc:1753)
/// Calls: cxx:__ZN7mjCBase10CopyPluginEv, cxx:__ZN7mjCBase6AddRefEv, cxx:__ZN7mjCBase9NameSpaceEPK8mjCModel, cxx:__ZNK8mjCFrame10IsAncestorEPKS_
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_copy_list__733baccf8cccec17(self_ptr: *mut mjCBody, dst: *const (), src: *const (), fmap: *const (), pframe: *const mjCFrame) {
    todo!() // cxx:__ZN7mjCBody8CopyListI8mjCFrameEEvRNSt3__16vectorIPT_NS2_9allocatorIS5_EEEERKS8_RNS2_3mapIPS1_iNS2_4lessISD_EENS6_INS2_4pairIKSD_iEEEEEEPKS1_
}

/// C: mjCBody::CopyList (user/user_objects.cc:1753)
/// Calls: cxx:__ZN7mjCBase10CopyPluginEv, cxx:__ZN7mjCBase6AddRefEv, cxx:__ZN7mjCBase9NameSpaceEPK8mjCModel, cxx:__ZNK8mjCFrame10IsAncestorEPKS_
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_copy_list__fbb0595f60076a09(self_ptr: *mut mjCBody, dst: *const (), src: *const (), fmap: *const (), pframe: *const mjCFrame) {
    todo!() // cxx:__ZN7mjCBody8CopyListI8mjCJointEEvRNSt3__16vectorIPT_NS2_9allocatorIS5_EEEERKS8_RNS2_3mapIP8mjCFrameiNS2_4lessISE_EENS6_INS2_4pairIKSE_iEEEEEEPKSD_
}

/// C: mjCBody::CopyList (user/user_objects.cc:1753)
/// Calls: cxx:__ZN7mjCBase10CopyPluginEv, cxx:__ZN7mjCBase6AddRefEv, cxx:__ZN8mjCLight9NameSpaceEPK8mjCModel, cxx:__ZNK8mjCFrame10IsAncestorEPKS_
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_copy_list__b67f5a9c4cc5ece2(self_ptr: *mut mjCBody, dst: *const (), src: *const (), fmap: *const (), pframe: *const mjCFrame) {
    todo!() // cxx:__ZN7mjCBody8CopyListI8mjCLightEEvRNSt3__16vectorIPT_NS2_9allocatorIS5_EEEERKS8_RNS2_3mapIP8mjCFrameiNS2_4lessISE_EENS6_INS2_4pairIKSE_iEEEEEEPKSD_
}

/// C: mjCBody::CopyList (user/user_objects.cc:1753)
/// Calls: cxx:__ZN7mjCBase10CopyPluginEv, cxx:__ZN7mjCBase6AddRefEv, cxx:__ZN9mjCCamera9NameSpaceEPK8mjCModel, cxx:__ZNK8mjCFrame10IsAncestorEPKS_
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_copy_list__6f380ec65bebab18(self_ptr: *mut mjCBody, dst: *const (), src: *const (), fmap: *const (), pframe: *const mjCFrame) {
    todo!() // cxx:__ZN7mjCBody8CopyListI9mjCCameraEEvRNSt3__16vectorIPT_NS2_9allocatorIS5_EEEERKS8_RNS2_3mapIP8mjCFrameiNS2_4lessISE_EENS6_INS2_4pairIKSE_iEEEEEEPKSD_
}

/// C: mjCBody::SetModel (user/user_objects.cc:1807)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_set_model(self_ptr: *mut mjCBody, _model: *mut mjCModel) {
    todo!() // cxx:__ZN7mjCBody8SetModelEP8mjCModel
}

/// C: mjCBody::ResetId (user/user_objects.cc:1847)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_reset_id(self_ptr: *mut mjCBody) {
    todo!() // cxx:__ZN7mjCBody7ResetIdEv
}

/// C: mjCBody::PointToLocal (user/user_objects.cc:1863)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_point_to_local(self_ptr: *mut mjCBody) {
    todo!() // cxx:__ZN7mjCBody12PointToLocalEv
}

/// C: mjCBody::CopyFromSpec (user/user_objects.cc:1874)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_copy_from_spec(self_ptr: *mut mjCBody) {
    todo!() // cxx:__ZN7mjCBody12CopyFromSpecEv
}

/// C: mjCBody::CopyPlugin (user/user_objects.cc:1885)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_copy_plugin(self_ptr: *mut mjCBody) {
    todo!() // cxx:__ZN7mjCBody10CopyPluginEv
}

/// C: mjCBody::NameSpace (user/user_objects.cc:1903)
/// Calls: cxx:__ZN7mjCBody10NameSpace_EPK8mjCModelb
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_name_space__77a7259720d03cd3(self_ptr: *mut mjCBody, m: *const mjCModel) {
    todo!() // cxx:__ZN7mjCBody9NameSpaceEPK8mjCModel
}

/// C: mjCBody::NameSpace_ (user/user_objects.cc:1909)
/// Calls: cxx:__ZN7mjCBase9NameSpaceEPK8mjCModel, cxx:__ZN7mjCGeom9NameSpaceEPK8mjCModel, cxx:__ZN7mjCSite9NameSpaceEPK8mjCModel, cxx:__ZN8mjCLight9NameSpaceEPK8mjCModel, cxx:__ZN9mjCCamera9NameSpaceEPK8mjCModel
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_name_space__2b35bbc8767711c4(self_ptr: *mut mjCBody, m: *const mjCModel, propagate: bool) {
    todo!() // cxx:__ZN7mjCBody10NameSpace_EPK8mjCModelb
}

/// C: mjCBody::AddBody (user/user_objects.cc:1938)
/// Calls: cxx:__ZN8mjCModel13MakeTreeListsEP7mjCBody, cxx:__ZN8mjCModel14ResetTreeListsEv, cxx:__ZN8mjCModel9SignatureEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_add_body(self_ptr: *mut mjCBody, arg0: *mut mjCDef) -> *mut mjCBody {
    todo!() // cxx:__ZN7mjCBody7AddBodyEP6mjCDef
}

/// C: mjCBody::AddFrame (user/user_objects.cc:1960)
/// Calls: cxx:__ZN8mjCModel13MakeTreeListsEP7mjCBody, cxx:__ZN8mjCModel14ResetTreeListsEv, cxx:__ZN8mjCModel9SignatureEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_add_frame(self_ptr: *mut mjCBody, arg0: *mut mjCFrame) -> *mut mjCFrame {
    todo!() // cxx:__ZN7mjCBody8AddFrameEP8mjCFrame
}

/// C: mjCBody::AddFreeJoint (user/user_objects.cc:1973)
/// Calls: cxx:__ZN8mjCModel13MakeTreeListsEP7mjCBody, cxx:__ZN8mjCModel14ResetTreeListsEv, cxx:__ZN8mjCModel9SignatureEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_add_free_joint(self_ptr: *mut mjCBody) -> *mut mjCJoint {
    todo!() // cxx:__ZN7mjCBody12AddFreeJointEv
}

/// C: mjCBody::AddJoint (user/user_objects.cc:1994)
/// Calls: cxx:__ZN8mjCModel13MakeTreeListsEP7mjCBody, cxx:__ZN8mjCModel14ResetTreeListsEv, cxx:__ZN8mjCModel9SignatureEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_add_joint(self_ptr: *mut mjCBody, arg0: *mut mjCDef) -> *mut mjCJoint {
    todo!() // cxx:__ZN7mjCBody8AddJointEP6mjCDef
}

/// C: mjCBody::AddGeom (user/user_objects.cc:2014)
/// Calls: cxx:__ZN8mjCModel13MakeTreeListsEP7mjCBody, cxx:__ZN8mjCModel14ResetTreeListsEv, cxx:__ZN8mjCModel9SignatureEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_add_geom(self_ptr: *mut mjCBody, arg0: *mut mjCDef) -> *mut mjCGeom {
    todo!() // cxx:__ZN7mjCBody7AddGeomEP6mjCDef
}

/// C: mjCBody::AddSite (user/user_objects.cc:2034)
/// Calls: cxx:__ZN8mjCModel13MakeTreeListsEP7mjCBody, cxx:__ZN8mjCModel14ResetTreeListsEv, cxx:__ZN8mjCModel9SignatureEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_add_site(self_ptr: *mut mjCBody, arg0: *mut mjCDef) -> *mut mjCSite {
    todo!() // cxx:__ZN7mjCBody7AddSiteEP6mjCDef
}

/// C: mjCBody::AddCamera (user/user_objects.cc:2054)
/// Calls: cxx:__ZN8mjCModel13MakeTreeListsEP7mjCBody, cxx:__ZN8mjCModel14ResetTreeListsEv, cxx:__ZN8mjCModel9SignatureEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_add_camera(self_ptr: *mut mjCBody, arg0: *mut mjCDef) -> *mut mjCCamera {
    todo!() // cxx:__ZN7mjCBody9AddCameraEP6mjCDef
}

/// C: mjCBody::AddLight (user/user_objects.cc:2074)
/// Calls: cxx:__ZN8mjCModel13MakeTreeListsEP7mjCBody, cxx:__ZN8mjCModel14ResetTreeListsEv, cxx:__ZN8mjCModel9SignatureEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_add_light(self_ptr: *mut mjCBody, arg0: *mut mjCDef) -> *mut mjCLight {
    todo!() // cxx:__ZN7mjCBody8AddLightEP6mjCDef
}

/// C: mjCBody::ToFrame (user/user_objects.cc:2094)
/// Calls: cxx:__ZN7mjCBody17AccumulateInertiaEPK8mjsBody_PS0_, cxx:__ZN7mjCBody20MakeInertialExplicitEv, cxx:__ZN7mjCBody8AddFrameEP8mjCFrame, cxx:__ZN8mjCModel13MakeTreeListsEP7mjCBody, cxx:__ZN8mjCModel14ResetTreeListsEv, cxx:__ZN8mjCModel9SignatureEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_to_frame(self_ptr: *mut mjCBody) -> *mut mjCFrame {
    todo!() // cxx:__ZN7mjCBody7ToFrameEv
}

/// C: mjCBody::NumObjects (user/user_objects.cc:2126)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_num_objects(self_ptr: *mut mjCBody, r#type: u32) -> i32 {
    todo!() // cxx:__ZN7mjCBody10NumObjectsE6mjtObj
}

/// C: mjCBody::GetObject (user/user_objects.cc:2148)
/// Calls: cxx:__ZN7mjCBody10NumObjectsE6mjtObj
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_get_object(self_ptr: *mut mjCBody, r#type: u32, id: i32) -> *mut mjCBase {
    todo!() // cxx:__ZN7mjCBody9GetObjectE6mjtObji
}

/// C: findobject (user/user_objects.cc:2175)
#[allow(unused_variables, non_snake_case)]
pub fn findobject__a9ea0e561d19f85f(name: *const std__string, list: *const ()) -> *mut mjCBody {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.cc:__ZL10findobjectI7mjCBodyEPT_RKNSt3__112basic_stringIcNS3_11char_traitsIcEENS3_9allocatorIcEEEERKNS3_6vectorIS2_NS7_IS2_EEEE
}

/// C: findobject (user/user_objects.cc:2175)
#[allow(unused_variables, non_snake_case)]
pub fn findobject__9cc1755d9cc24202(name: *const std__string, list: *const ()) -> *mut mjCGeom {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.cc:__ZL10findobjectI7mjCGeomEPT_RKNSt3__112basic_stringIcNS3_11char_traitsIcEENS3_9allocatorIcEEEERKNS3_6vectorIS2_NS7_IS2_EEEE
}

/// C: findobject (user/user_objects.cc:2175)
#[allow(unused_variables, non_snake_case)]
pub fn findobject__23c443e66d50fe5f(name: *const std__string, list: *const ()) -> *mut mjCSite {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.cc:__ZL10findobjectI7mjCSiteEPT_RKNSt3__112basic_stringIcNS3_11char_traitsIcEENS3_9allocatorIcEEEERKNS3_6vectorIS2_NS7_IS2_EEEE
}

/// C: findobject (user/user_objects.cc:2175)
#[allow(unused_variables, non_snake_case)]
pub fn findobject__b885f60782b6767e(name: *const std__string, list: *const ()) -> *mut mjCJoint {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.cc:__ZL10findobjectI8mjCJointEPT_RKNSt3__112basic_stringIcNS3_11char_traitsIcEENS3_9allocatorIcEEEERKNS3_6vectorIS2_NS7_IS2_EEEE
}

/// C: findobject (user/user_objects.cc:2175)
#[allow(unused_variables, non_snake_case)]
pub fn findobject__8979a370afca01ef(name: *const std__string, list: *const ()) -> *mut mjCLight {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.cc:__ZL10findobjectI8mjCLightEPT_RKNSt3__112basic_stringIcNS3_11char_traitsIcEENS3_9allocatorIcEEEERKNS3_6vectorIS2_NS7_IS2_EEEE
}

/// C: findobject (user/user_objects.cc:2175)
#[allow(unused_variables, non_snake_case)]
pub fn findobject__0fc1cbd2eec6e71e(name: *const std__string, list: *const ()) -> *mut mjCCamera {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.cc:__ZL10findobjectI9mjCCameraEPT_RKNSt3__112basic_stringIcNS3_11char_traitsIcEENS3_9allocatorIcEEEERKNS3_6vectorIS2_NS7_IS2_EEEE
}

/// C: mjCBody::FindObject (user/user_objects.cc:2185)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_find_object(self_ptr: *mut mjCBody, r#type: u32, name: *const std__string, recursive: bool) -> *mut mjCBase {
    todo!() // cxx:__ZNK7mjCBody10FindObjectE6mjtObjRKNSt3__112basic_stringIcNS1_11char_traitsIcEENS1_9allocatorIcEEEEb
}

/// C: mjCBody::GetList (user/user_objects.cc:2223)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_get_list__975cf469c2eb9cdf(self_ptr: *mut mjCBody) -> *const () {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).bodies }
}

/// C: mjCBody::GetList (user/user_objects.cc:2228)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_get_list__367a34b7b6ac3342(self_ptr: *mut mjCBody) -> *const () {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).joints }
}

/// C: mjCBody::GetList (user/user_objects.cc:2233)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_get_list__a51bce796e18c18d(self_ptr: *mut mjCBody) -> *const () {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).geoms }
}

/// C: mjCBody::GetList (user/user_objects.cc:2238)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_get_list__9e70d65d8d0d1c48(self_ptr: *mut mjCBody) -> *const () {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).sites }
}

/// C: mjCBody::GetList (user/user_objects.cc:2243)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_get_list__81e628945d5100cf(self_ptr: *mut mjCBody) -> *const () {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).cameras }
}

/// C: mjCBody::GetList (user/user_objects.cc:2248)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_get_list__975841557671111b(self_ptr: *mut mjCBody) -> *const () {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).lights }
}

/// C: mjCBody::GetList (user/user_objects.cc:2253)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_get_list__7100ea1c5c87a8e7(self_ptr: *mut mjCBody) -> *const () {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).frames }
}

/// C: GetNext (user/user_objects.cc:2260)
/// Calls: cxx:__ZNK7mjCBody6BodiesEv
#[allow(unused_variables, non_snake_case)]
pub fn get_next__9778a2bd498fb167(body: *const mjCBody, child: *const mjsElement, found: *mut bool, recursive: bool) -> *mut mjsElement {
    todo!() // cxx-internal:user_objects.cc.o:__ZL7GetNextI7mjCGeomEP11mjsElement_PK7mjCBodyPKS1_Pbb
}

/// C: GetNext (user/user_objects.cc:2260)
/// Calls: cxx:__ZNK7mjCBody6BodiesEv
#[allow(unused_variables, non_snake_case)]
pub fn get_next__61bf10225c82118d(body: *const mjCBody, child: *const mjsElement, found: *mut bool, recursive: bool) -> *mut mjsElement {
    todo!() // cxx-internal:user_objects.cc.o:__ZL7GetNextI7mjCSiteEP11mjsElement_PK7mjCBodyPKS1_Pbb
}

/// C: GetNext (user/user_objects.cc:2260)
/// Calls: cxx:__ZNK7mjCBody6BodiesEv
#[allow(unused_variables, non_snake_case)]
pub fn get_next__7af41d7c88174834(body: *const mjCBody, child: *const mjsElement, found: *mut bool, recursive: bool) -> *mut mjsElement {
    todo!() // cxx-internal:user_objects.cc.o:__ZL7GetNextI8mjCFrameEP11mjsElement_PK7mjCBodyPKS1_Pbb
}

/// C: GetNext (user/user_objects.cc:2260)
/// Calls: cxx:__ZNK7mjCBody6BodiesEv
#[allow(unused_variables, non_snake_case)]
pub fn get_next__84801a70ca3a7568(body: *const mjCBody, child: *const mjsElement, found: *mut bool, recursive: bool) -> *mut mjsElement {
    todo!() // cxx-internal:user_objects.cc.o:__ZL7GetNextI8mjCJointEP11mjsElement_PK7mjCBodyPKS1_Pbb
}

/// C: GetNext (user/user_objects.cc:2260)
/// Calls: cxx:__ZNK7mjCBody6BodiesEv
#[allow(unused_variables, non_snake_case)]
pub fn get_next__47549d91155d4bc8(body: *const mjCBody, child: *const mjsElement, found: *mut bool, recursive: bool) -> *mut mjsElement {
    todo!() // cxx-internal:user_objects.cc.o:__ZL7GetNextI8mjCLightEP11mjsElement_PK7mjCBodyPKS1_Pbb
}

/// C: GetNext (user/user_objects.cc:2260)
/// Calls: cxx:__ZNK7mjCBody6BodiesEv
#[allow(unused_variables, non_snake_case)]
pub fn get_next__898c28a1a4face8d(body: *const mjCBody, child: *const mjsElement, found: *mut bool, recursive: bool) -> *mut mjsElement {
    todo!() // cxx-internal:user_objects.cc.o:__ZL7GetNextI9mjCCameraEP11mjsElement_PK7mjCBodyPKS1_Pbb
}

/// C: GetNextBody (user/user_objects.cc:2284)
/// Calls: cxx:__ZNK7mjCBody6BodiesEv
#[allow(unused_variables, non_snake_case)]
pub fn get_next_body(body: *const mjCBody, child: *const mjsElement, found: *mut bool, recursive: bool) -> *mut mjsElement {
    todo!() // cxx-internal:user_objects.cc.o:__ZL11GetNextBodyPK7mjCBodyPK11mjsElement_Pbb
}

/// C: mjCBody::NextChild (user/user_objects.cc:2304)
/// Calls: cxx-internal:user_objects.cc.o:__ZL11GetNextBodyPK7mjCBodyPK11mjsElement_Pbb
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_next_child(self_ptr: *mut mjCBody, child: *const mjsElement, r#type: u32, recursive: bool) -> *mut mjsElement {
    todo!() // cxx:__ZNK7mjCBody9NextChildEPK11mjsElement_6mjtObjb
}

/// C: mjCBody::InertiaFromGeom (user/user_objects.cc:2352)
/// Calls: cxx:__Z16mjuu_fullInertiaPdS_PKd, cxx:__Z18mjuu_globalinertiaPdPKdS1_
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_inertia_from_geom(self_ptr: *mut mjCBody) {
    todo!() // cxx:__ZN7mjCBody15InertiaFromGeomEv
}

/// C: mjCBody::MakeInertialExplicit (user/user_objects.cc:2420)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_make_inertial_explicit(self_ptr: *mut mjCBody) {
    todo!() // cxx:__ZN7mjCBody20MakeInertialExplicitEv
}

/// C: mjCBody::AccumulateInertia (user/user_objects.cc:2426)
/// Calls: cxx:__Z15mjuu_frameaccumPdS_PKdS1_, cxx:__Z16mjuu_fullInertiaPdS_PKd, cxx:__Z18mjuu_globalinertiaPdPKdS1_
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_accumulate_inertia(self_ptr: *mut mjCBody, other: *const mjsBody, result: *mut mjsBody) {
    todo!() // cxx:__ZN7mjCBody17AccumulateInertiaEPK8mjsBody_PS0_
}

/// C: mjCBody::ComputeBVH (user/user_objects.cc:2500)
/// Calls: cxx:__ZN26mjCBoundingVolumeHierarchy23AllocateBoundingVolumesEi, cxx:__ZN26mjCBoundingVolumeHierarchy3SetEPdS0_, cxx:__ZN26mjCBoundingVolumeHierarchy9CreateBVHEP8mjCModelPK7mjCBase
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_compute_bvh(self_ptr: *mut mjCBody) {
    todo!() // cxx:__ZN7mjCBody10ComputeBVHEv
}

/// C: mjCBody::ForgetKeyframes (user/user_objects.cc:2518)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_forget_keyframes(self_ptr: *mut mjCBody) {
    todo!() // cxx:__ZNK7mjCBody15ForgetKeyframesEv
}

/// C: mjCBody::mpos (user/user_objects.cc:2529)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_mpos(self_ptr: *mut mjCBody, state_name: *const std__string) -> *mut f64 {
    todo!() // cxx:__ZN7mjCBody4mposERKNSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEE
}

/// C: mjCBody::mquat (user/user_objects.cc:2535)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_mquat(self_ptr: *mut mjCBody, state_name: *const std__string) -> *mut f64 {
    todo!() // cxx:__ZN7mjCBody5mquatERKNSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEE
}

/// C: mjCBody::Compile (user/user_objects.cc:2542)
/// Calls: cxx:__Z12mjuu_definedd, cxx:__Z12mjuu_mulquatPdPKdS1_, cxx:__Z13mjuu_addtovecPdPKdi, cxx:__Z15mjuu_frameaccumPdS_PKdS1_, cxx:__Z15mjuu_rotVecQuatPdPKdS1_, cxx:__Z16mjuu_frameinvertPdS_PKdS1_, cxx:__Z16mjuu_fullInertiaPdS_PKd, cxx:__Z18ResolveOrientationPdbPKcRK15mjsOrientation_, cxx:__Z20mjuu_frameaccumChildPKdS0_PdS1_, cxx:__ZN7mjCBody10ComputeBVHEv, cxx:__ZN7mjCBody12CopyFromSpecEv, cxx:__ZN7mjCBody15InertiaFromGeomEv, cxx:__ZN7mjCGeom7CompileEv, cxx:__ZN7mjCSite7CompileEv, cxx:__ZN8mjCFrame7CompileEv, cxx:__ZN8mjCJoint7CompileEv, cxx:__ZN8mjCLight7CompileEv, cxx:__ZN8mjCModel13ResolvePluginEP7mjCBaseRKNSt3__112basic_stringIcNS2_11char_traitsIcEENS2_9allocatorIcEEEESA_PP9mjCPlugin, cxx:__ZN9mjCCamera7CompileEv, cxx:_mjp_getPluginAtSlot
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_compile(self_ptr: *mut mjCBody) {
    todo!() // cxx:__ZN7mjCBody7CompileEv
}

/// C: mjCFrame::IsAncestor (user/user_objects.cc:2848)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_frame_is_ancestor(self_ptr: *mut mjCFrame, child: *const mjCFrame) -> bool {
    todo!() // cxx:__ZNK8mjCFrame10IsAncestorEPKS_
}

/// C: mjCFrame::PointToLocal (user/user_objects.cc:2857)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_frame_point_to_local(self_ptr: *mut mjCFrame) {
    todo!() // cxx:__ZN8mjCFrame12PointToLocalEv
}

/// C: mjCFrame::CopyFromSpec (user/user_objects.cc:2864)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_frame_copy_from_spec(self_ptr: *mut mjCFrame) {
    todo!() // cxx:__ZN8mjCFrame12CopyFromSpecEv
}

/// C: mjCFrame::Compile (user/user_objects.cc:2871)
/// Calls: cxx:__Z18ResolveOrientationPdbPKcRK15mjsOrientation_, cxx:__Z20mjuu_frameaccumChildPKdS0_PdS1_, cxx:__ZN8mjCFrame12CopyFromSpecEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_frame_compile(self_ptr: *mut mjCFrame) {
    todo!() // cxx:__ZN8mjCFrame7CompileEv
}

/// C: mjCJoint::is_limited (user/user_objects.cc:2940)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.cc:__ZL9islimitediPKd
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_joint_is_limited(self_ptr: *mut mjCJoint) -> bool {
    todo!() // cxx:__ZNK8mjCJoint10is_limitedEv
}

/// C: mjCJoint::is_actfrclimited (user/user_objects.cc:2943)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.cc:__ZL9islimitediPKd
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_joint_is_actfrclimited(self_ptr: *mut mjCJoint) -> bool {
    todo!() // cxx:__ZNK8mjCJoint16is_actfrclimitedEv
}

/// C: mjCJoint::nq (user/user_objects.cc:2948)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_joint_nq__6edc07bc8acead00(joint_type: u32) -> i32 {
    todo!() // cxx:__ZN8mjCJoint2nqE8mjtJoint
}

/// C: mjCJoint::nv (user/user_objects.cc:2962)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_joint_nv__19436c073d3cf9fa(joint_type: u32) -> i32 {
    todo!() // cxx:__ZN8mjCJoint2nvE8mjtJoint
}

/// C: mjCJoint::qpos (user/user_objects.cc:2976)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_joint_qpos(self_ptr: *mut mjCJoint, state_name: *const std__string) -> *mut f64 {
    todo!() // cxx:__ZN8mjCJoint4qposERKNSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEE
}

/// C: mjCJoint::qvel (user/user_objects.cc:2982)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_joint_qvel(self_ptr: *mut mjCJoint, state_name: *const std__string) -> *mut f64 {
    todo!() // cxx:__ZN8mjCJoint4qvelERKNSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEE
}

/// C: mjCJoint::PointToLocal (user/user_objects.cc:2988)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_joint_point_to_local(self_ptr: *mut mjCJoint) {
    todo!() // cxx:__ZN8mjCJoint12PointToLocalEv
}

/// C: mjCJoint::CopyFromSpec (user/user_objects.cc:2996)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_joint_copy_from_spec(self_ptr: *mut mjCJoint) {
    todo!() // cxx:__ZN8mjCJoint12CopyFromSpecEv
}

/// C: mjCJoint::Compile (user/user_objects.cc:3004)
/// Calls: cxx-internal:user_objects.cc.o:__ZL12checklimitedPK7mjCBasebPKcS3_ib, cxx:__Z15mjuu_rotVecQuatPdPKdS1_, cxx:__Z20mjuu_frameaccumChildPKdS0_PdS1_, cxx:__ZN8mjCJoint12CopyFromSpecEv, cxx:__ZNK8mjCJoint10is_limitedEv, cxx:__ZNK8mjCJoint16is_actfrclimitedEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_joint_compile(self_ptr: *mut mjCJoint) -> i32 {
    todo!() // cxx:__ZN8mjCJoint7CompileEv
}

/// C: mjCGeom::PointToLocal (user/user_objects.cc:3170)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_geom_point_to_local(self_ptr: *mut mjCGeom) {
    todo!() // cxx:__ZN7mjCGeom12PointToLocalEv
}

/// C: mjCGeom::CopyFromSpec (user/user_objects.cc:3186)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_geom_copy_from_spec(self_ptr: *mut mjCGeom) {
    todo!() // cxx:__ZN7mjCGeom12CopyFromSpecEv
}

/// C: mjCGeom::CopyPlugin (user/user_objects.cc:3200)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_geom_copy_plugin(self_ptr: *mut mjCGeom) {
    todo!() // cxx:__ZN7mjCGeom10CopyPluginEv
}

/// C: mjCGeom::NameSpace (user/user_objects.cc:3205)
/// Calls: cxx:__ZN7mjCBase9NameSpaceEPK8mjCModel
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_geom_name_space(self_ptr: *mut mjCGeom, m: *const mjCModel) {
    todo!() // cxx:__ZN7mjCGeom9NameSpaceEPK8mjCModel
}

/// C: mjCGeom::GetVolume (user/user_objects.cc:3223)
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

/// C: mjCGeom::SetInertia (user/user_objects.cc:3301)
/// Calls: cxx:__ZN7mjCMesh16GetInertiaBoxPtrEv, cxx:__ZNK8mjCModel6MeshesEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_geom_set_inertia(self_ptr: *mut mjCGeom) {
    todo!() // cxx:__ZN7mjCGeom10SetInertiaEv
}

/// C: mjCGeom::GetRBound (user/user_objects.cc:3522)
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

/// C: mjCGeom::GetAddedMassKappa (user/user_objects.cc:3563)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_geom_get_added_mass_kappa(self_ptr: *mut mjCGeom, dx: f64, dy: f64, dz: f64) -> f64 {
    todo!() // cxx:__ZN7mjCGeom17GetAddedMassKappaEddd
}

/// C: mjCGeom::SetFluidCoefs (user/user_objects.cc:3642)
/// Calls: cxx:__ZN7mjCGeom17GetAddedMassKappaEddd, cxx:_writeFluidGeomInteraction
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_geom_set_fluid_coefs(self_ptr: *mut mjCGeom) {
    todo!() // cxx:__ZN7mjCGeom13SetFluidCoefsEv
}

/// C: mjCGeom::ComputeAABB (user/user_objects.cc:3717)
/// Calls: cxx:__ZNK7mjCMesh4aammEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_geom_compute_aabb(self_ptr: *mut mjCGeom) {
    todo!() // cxx:__ZN7mjCGeom11ComputeAABBEv
}

/// C: mjCGeom::get_material (user/user_objects.cc:3770)
/// Calls: cxx:__ZNK7mjCMesh8MaterialEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_geom_get_material(self_ptr: *mut mjCGeom) -> *const std__string {
    todo!() // cxx:__ZNK7mjCGeom12get_materialEv
}

/// C: mjCGeom::Compile (user/user_objects.cc:3776)
/// Calls: cxx-internal:user_objects.cc.o:__ZL9checksizePd7mjtGeomP7mjCBasePKci, cxx:__Z11mjuu_z2quatPdPKd, cxx:__Z12mjuu_definedd, cxx:__Z13mjuu_addtovecPdPKdi, cxx:__Z13mjuu_crossvecPdPKdS1_, cxx:__Z15mjuu_frameaccumPdS_PKdS1_, cxx:__Z15mjuu_rotVecQuatPdPKdS1_, cxx:__Z18ResolveOrientationPdbPKcRK15mjsOrientation_, cxx:__Z20mjuu_frameaccumChildPKdS0_PdS1_, cxx:__ZN7mjCGeom10SetInertiaEv, cxx:__ZN7mjCGeom11ComputeAABBEv, cxx:__ZN7mjCGeom12CopyFromSpecEv, cxx:__ZN7mjCGeom13SetFluidCoefsEv, cxx:__ZN7mjCMesh10GetQuatPtrEv, cxx:__ZN7mjCMesh7FitGeomEP7mjCGeomPd, cxx:__ZN7mjCMesh9GetPosPtrEv, cxx:__ZN8mjCModel13ResolvePluginEP7mjCBaseRKNSt3__112basic_stringIcNS2_11char_traitsIcEENS2_9allocatorIcEEEESA_PP9mjCPlugin, cxx:__ZNK7mjCGeom9GetVolumeEv, cxx:__ZNK7mjCMesh4aammEv, cxx:_mjp_getPluginAtSlot
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_geom_compile(self_ptr: *mut mjCGeom) {
    todo!() // cxx:__ZN7mjCGeom7CompileEv
}

/// C: mjCSite::PointToLocal (user/user_objects.cc:4031)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_site_point_to_local(self_ptr: *mut mjCSite) {
    todo!() // cxx:__ZN7mjCSite12PointToLocalEv
}

/// C: mjCSite::CopyFromSpec (user/user_objects.cc:4043)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_site_copy_from_spec(self_ptr: *mut mjCSite) {
    todo!() // cxx:__ZN7mjCSite12CopyFromSpecEv
}

/// C: mjCSite::NameSpace (user/user_objects.cc:4052)
/// Calls: cxx:__ZN7mjCBase9NameSpaceEPK8mjCModel
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_site_name_space(self_ptr: *mut mjCSite, m: *const mjCModel) {
    todo!() // cxx:__ZN7mjCSite9NameSpaceEPK8mjCModel
}

/// C: mjCSite::get_material (user/user_objects.cc:4063)
/// Calls: cxx:__ZNK7mjCMesh8MaterialEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_site_get_material(self_ptr: *mut mjCSite) -> *const std__string {
    todo!() // cxx:__ZNK7mjCSite12get_materialEv
}

/// C: mjCSite::Compile (user/user_objects.cc:4070)
/// Calls: cxx-internal:user_objects.cc.o:__ZL9checksizePd7mjtGeomP7mjCBasePKci, cxx:__Z11mjuu_z2quatPdPKd, cxx:__Z12mjuu_definedd, cxx:__Z13mjuu_addtovecPdPKdi, cxx:__Z15mjuu_frameaccumPdS_PKdS1_, cxx:__Z15mjuu_rotVecQuatPdPKdS1_, cxx:__Z18ResolveOrientationPdbPKcRK15mjsOrientation_, cxx:__Z20mjuu_frameaccumChildPKdS0_PdS1_, cxx:__ZN7mjCMesh10GetQuatPtrEv, cxx:__ZN7mjCMesh9GetPosPtrEv, cxx:__ZN7mjCSite12CopyFromSpecEv, cxx:__ZNK7mjCMesh4aammEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_site_compile(self_ptr: *mut mjCSite) {
    todo!() // cxx:__ZN7mjCSite7CompileEv
}

/// C: mjCCamera::PointToLocal (user/user_objects.cc:4209)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_camera_point_to_local(self_ptr: *mut mjCCamera) {
    todo!() // cxx:__ZN9mjCCamera12PointToLocalEv
}

/// C: mjCCamera::NameSpace (user/user_objects.cc:4219)
/// Calls: cxx:__ZN7mjCBase9NameSpaceEPK8mjCModel
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_camera_name_space(self_ptr: *mut mjCCamera, m: *const mjCModel) {
    todo!() // cxx:__ZN9mjCCamera9NameSpaceEPK8mjCModel
}

/// C: mjCCamera::CopyFromSpec (user/user_objects.cc:4225)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_camera_copy_from_spec(self_ptr: *mut mjCCamera) {
    todo!() // cxx:__ZN9mjCCamera12CopyFromSpecEv
}

/// C: mjCCamera::ResolveReferences (user/user_objects.cc:4233)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_camera_resolve_references(self_ptr: *mut mjCCamera, m: *const mjCModel) {
    todo!() // cxx:__ZN9mjCCamera17ResolveReferencesEPK8mjCModel
}

/// C: mjCCamera::Compile (user/user_objects.cc:4246)
/// Calls: cxx:__Z18ResolveOrientationPdbPKcRK15mjsOrientation_, cxx:__Z20mjuu_frameaccumChildPKdS0_PdS1_, cxx:__ZN9mjCCamera12CopyFromSpecEv, cxx:__ZN9mjCCamera17ResolveReferencesEPK8mjCModel
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_camera_compile(self_ptr: *mut mjCCamera) {
    todo!() // cxx:__ZN9mjCCamera7CompileEv
}

/// C: mjCLight::PointToLocal (user/user_objects.cc:4371)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_light_point_to_local(self_ptr: *mut mjCLight) {
    todo!() // cxx:__ZN8mjCLight12PointToLocalEv
}

/// C: mjCLight::NameSpace (user/user_objects.cc:4380)
/// Calls: cxx:__ZN7mjCBase9NameSpaceEPK8mjCModel
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_light_name_space(self_ptr: *mut mjCLight, m: *const mjCModel) {
    todo!() // cxx:__ZN8mjCLight9NameSpaceEPK8mjCModel
}

/// C: mjCLight::CopyFromSpec (user/user_objects.cc:4387)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_light_copy_from_spec(self_ptr: *mut mjCLight) {
    todo!() // cxx:__ZN8mjCLight12CopyFromSpecEv
}

/// C: mjCLight::ResolveReferences (user/user_objects.cc:4395)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_light_resolve_references(self_ptr: *mut mjCLight, m: *const mjCModel) {
    todo!() // cxx:__ZN8mjCLight17ResolveReferencesEPK8mjCModel
}

/// C: mjCLight::Compile (user/user_objects.cc:4416)
/// Calls: cxx:__Z15mjuu_rotVecQuatPdPKdS1_, cxx:__Z20mjuu_frameaccumChildPKdS0_PdS1_, cxx:__ZN8mjCLight12CopyFromSpecEv, cxx:__ZN8mjCLight17ResolveReferencesEPK8mjCModel
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_light_compile(self_ptr: *mut mjCLight) {
    todo!() // cxx:__ZN8mjCLight7CompileEv
}

/// C: mjCHField::PointToLocal (user/user_objects.cc:4481)
#[allow(unused_variables, non_snake_case)]
pub fn mj_ch_field_point_to_local(self_ptr: *mut mjCHField) {
    todo!() // cxx:__ZN9mjCHField12PointToLocalEv
}

/// C: mjCHField::CopyFromSpec (user/user_objects.cc:4493)
/// Calls: cxx:__Z13mjuu_stripextNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEE, cxx:__Z14mjuu_strippathNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEE
#[allow(unused_variables, non_snake_case)]
pub fn mj_ch_field_copy_from_spec(self_ptr: *mut mjCHField) {
    todo!() // cxx:__ZN9mjCHField12CopyFromSpecEv
}

/// C: mjCHField::NameSpace (user/user_objects.cc:4516)
/// Calls: cxx:__Z13mjuu_stripextNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEE, cxx:__Z14mjuu_strippathNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEE, cxx:__ZN7mjCBase9NameSpaceEPK8mjCModel
#[allow(unused_variables, non_snake_case)]
pub fn mj_ch_field_name_space(self_ptr: *mut mjCHField, m: *const mjCModel) {
    todo!() // cxx:__ZN9mjCHField9NameSpaceEPK8mjCModel
}

/// C: mjCHField::GetCacheId (user/user_objects.cc:4535)
#[allow(unused_variables, non_snake_case)]
pub fn mj_ch_field_get_cache_id(self_ptr: *mut mjCHField, resource: *const mjResource, asset_type: *const std__string) -> std__string {
    todo!() // cxx:__ZN9mjCHField10GetCacheIdEPK11mjResource_RKNSt3__112basic_stringIcNS3_11char_traitsIcEENS3_9allocatorIcEEEE
}

/// C: mjCHField::LoadCustom (user/user_objects.cc:4543)
/// Calls: cxx:_mju_readResource
#[allow(unused_variables, non_snake_case)]
pub fn mj_ch_field_load_custom(self_ptr: *mut mjCHField, resource: *mut mjResource) {
    todo!() // cxx:__ZN9mjCHField10LoadCustomEP11mjResource_
}

/// C: mjCHField::LoadPNG (user/user_objects.cc:4584)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.cc:__ZNK12_GLOBAL__N_18PNGImage5WidthEv, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.cc:__ZNK12_GLOBAL__N_18PNGImage6HeightEv, cxx-internal:user_objects.cc.o:__ZN12_GLOBAL__N_18PNGImage4LoadEPK7mjCBaseP11mjResource_16LodePNGColorType
#[allow(unused_variables, non_snake_case)]
pub fn mj_ch_field_load_png(self_ptr: *mut mjCHField, resource: *mut mjResource) {
    todo!() // cxx:__ZN9mjCHField7LoadPNGEP11mjResource_
}

/// C: mjCHField::Compile (user/user_objects.cc:4599)
/// Calls: cxx:__Z14mjuu_strippathNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEE, cxx:__ZN7mjCBase12LoadResourceERKNSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEES8_PK6mjVFS_, cxx:__ZN7mjCBase19GetAssetContentTypeENSt3__117basic_string_viewIcNS0_11char_traitsIcEEEES4_, cxx:__ZN8mjCCache12PopulateDataERKNSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEEPK11mjResource_NS0_8functionIFbPKvEEE, cxx:__ZN8mjCCache6InsertERKNSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEES8_PK11mjResource_NS0_10shared_ptrIKvEEm, cxx:__ZN9mjCHField10GetCacheIdEPK11mjResource_RKNSt3__112basic_stringIcNS3_11char_traitsIcEENS3_9allocatorIcEEEE, cxx:__ZN9mjCHField10LoadCustomEP11mjResource_, cxx:__ZN9mjCHField12CopyFromSpecEv, cxx:__ZN9mjCHField7LoadPNGEP11mjResource_, cxx:__ZNK6mujoco4user8FilePath3StrEv, cxx:_mj_getCache, cxx:_mjs_getString, cxx:_mju_closeResource
#[allow(unused_variables, non_snake_case)]
pub fn mj_ch_field_compile(self_ptr: *mut mjCHField, vfs: *const mjVFS) {
    todo!() // cxx:__ZN9mjCHField7CompileEPK6mjVFS_
}

/// C: mjCTexture::PointToLocal (user/user_objects.cc:4751)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_texture_point_to_local(self_ptr: *mut mjCTexture) {
    todo!() // cxx:__ZN10mjCTexture12PointToLocalEv
}

/// C: mjCTexture::CopyFromSpec (user/user_objects.cc:4764)
/// Calls: cxx:__Z13mjuu_stripextNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEE, cxx:__Z14mjuu_strippathNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEE
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_texture_copy_from_spec(self_ptr: *mut mjCTexture) {
    todo!() // cxx:__ZN10mjCTexture12CopyFromSpecEv
}

/// C: mjCTexture::NameSpace (user/user_objects.cc:4785)
/// Calls: cxx:__Z13mjuu_stripextNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEE, cxx:__Z14mjuu_strippathNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEE, cxx:__ZN7mjCBase9NameSpaceEPK8mjCModel
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_texture_name_space(self_ptr: *mut mjCTexture, m: *const mjCModel) {
    todo!() // cxx:__ZN10mjCTexture9NameSpaceEPK8mjCModel
}

/// C: randomdot (user/user_objects.cc:4803)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn randomdot(rgb: *mut u8, markrgb: *const f64, width: i32, height: i32, probability: f64) {
    todo!() // cxx-internal:user_objects.cc.o:__ZL9randomdotPSt4bytePKdiid
}

/// C: interp (user/user_objects.cc:4824)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn interp(rgb: *mut u8, rgb1: *const f64, rgb2: *const f64, pos: f64) {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.cc:__ZL6interpPSt4bytePKdS2_d
}

/// C: checker (user/user_objects.cc:4840)
#[allow(unused_variables, non_snake_case)]
pub fn checker(rgb: *mut u8, RGB1: *const u8, RGB2: *const u8, width: i32, height: i32) {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.cc:__ZL7checkerPSt4bytePKS_S2_ii
}

/// C: mjCTexture::Builtin2D (user/user_objects.cc:4858)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.cc:__ZL6interpPSt4bytePKdS2_d, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.cc:__ZL7checkerPSt4bytePKS_S2_ii, cxx-internal:user_objects.cc.o:__ZL9randomdotPSt4bytePKdiid
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_texture_builtin2d(self_ptr: *mut mjCTexture) {
    todo!() // cxx:__ZN10mjCTexture9Builtin2DEv
}

/// C: mjCTexture::BuiltinCube (user/user_objects.cc:4928)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.cc:__ZL6interpPSt4bytePKdS2_d, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.cc:__ZL7checkerPSt4bytePKS_S2_ii, cxx-internal:user_objects.cc.o:__ZL9randomdotPSt4bytePKdiid
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_texture_builtin_cube(self_ptr: *mut mjCTexture) {
    todo!() // cxx:__ZN10mjCTexture11BuiltinCubeEv
}

/// C: mjCTexture::LoadPNG (user/user_objects.cc:5040)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.cc:__ZNK12_GLOBAL__N_18PNGImage5WidthEv, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.cc:__ZNK12_GLOBAL__N_18PNGImage6HeightEv, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.cc:__ZNK12_GLOBAL__N_18PNGImage6IsSRGBEv, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.cc:__ZNO12_GLOBAL__N_18PNGImage8MoveDataEv, cxx-internal:user_objects.cc.o:__ZN12_GLOBAL__N_18PNGImage4LoadEPK7mjCBaseP11mjResource_16LodePNGColorType
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_texture_load_png(self_ptr: *mut mjCTexture, resource: *mut mjResource, image: *const (), w: *mut u32, h: *mut u32, is_srgb: *mut bool) {
    todo!() // cxx:__ZN10mjCTexture7LoadPNGEP11mjResource_RNSt3__16vectorISt4byteNS2_9allocatorIS4_EEEERjS9_Rb
}

/// C: mjCTexture::LoadKTX (user/user_objects.cc:5065)
/// Calls: cxx:_mju_readResource
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_texture_load_ktx(self_ptr: *mut mjCTexture, resource: *mut mjResource, image: *const (), w: *mut u32, h: *mut u32, is_srgb: *mut bool) {
    todo!() // cxx:__ZN10mjCTexture7LoadKTXEP11mjResource_RNSt3__16vectorISt4byteNS2_9allocatorIS4_EEEERjS9_Rb
}

/// C: mjCTexture::FlipIfNeeded (user/user_objects.cc:5087)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_texture_flip_if_needed(self_ptr: *mut mjCTexture, image: *const (), w: u32, h: u32) {
    todo!() // cxx:__ZN10mjCTexture12FlipIfNeededERNSt3__16vectorISt4byteNS0_9allocatorIS2_EEEEjj
}

/// C: mjCTexture::GetCacheId (user/user_objects.cc:5121)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_texture_get_cache_id(self_ptr: *mut mjCTexture, resource: *const mjResource, asset_type: *const std__string) -> std__string {
    todo!() // cxx:__ZN10mjCTexture10GetCacheIdEPK11mjResource_RKNSt3__112basic_stringIcNS3_11char_traitsIcEENS3_9allocatorIcEEEE
}

/// C: mjCTexture::LoadFlip (user/user_objects.cc:5134)
/// Calls: cxx:__ZN10mjCTexture10GetCacheIdEPK11mjResource_RKNSt3__112basic_stringIcNS3_11char_traitsIcEENS3_9allocatorIcEEEE, cxx:__ZN10mjCTexture12FlipIfNeededERNSt3__16vectorISt4byteNS0_9allocatorIS2_EEEEjj, cxx:__ZN10mjCTexture7LoadKTXEP11mjResource_RNSt3__16vectorISt4byteNS2_9allocatorIS4_EEEERjS9_Rb, cxx:__ZN10mjCTexture7LoadPNGEP11mjResource_RNSt3__16vectorISt4byteNS2_9allocatorIS4_EEEERjS9_Rb, cxx:__ZN7mjCBase12LoadResourceERKNSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEES8_PK6mjVFS_, cxx:__ZN7mjCBase19GetAssetContentTypeENSt3__117basic_string_viewIcNS0_11char_traitsIcEEEES4_, cxx:__ZN8mjCCache12PopulateDataERKNSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEEPK11mjResource_NS0_8functionIFbPKvEEE, cxx:__ZN8mjCCache6InsertERKNSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEES8_PK11mjResource_NS0_10shared_ptrIKvEEm, cxx:_mj_getCache, cxx:_mju_closeResource
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_texture_load_flip(self_ptr: *mut mjCTexture, filename: std__string, vfs: *const mjVFS, image: *const (), w: *mut u32, h: *mut u32, is_srgb: *mut bool) {
    todo!() // cxx:__ZN10mjCTexture8LoadFlipENSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEEPK6mjVFS_RNS0_6vectorISt4byteNS4_ISB_EEEERjSF_Rb
}

/// C: mjCTexture::Load2D (user/user_objects.cc:5196)
/// Calls: cxx:__ZN10mjCTexture8LoadFlipENSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEEPK6mjVFS_RNS0_6vectorISt4byteNS4_ISB_EEEERjSF_Rb
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_texture_load2d(self_ptr: *mut mjCTexture, filename: std__string, vfs: *const mjVFS) {
    todo!() // cxx:__ZN10mjCTexture6Load2DENSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEEPK6mjVFS_
}

/// C: mjCTexture::LoadCubeSingle (user/user_objects.cc:5212)
/// Calls: cxx:__ZN10mjCTexture8LoadFlipENSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEEPK6mjVFS_RNS0_6vectorISt4byteNS4_ISB_EEEERjSF_Rb
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_texture_load_cube_single(self_ptr: *mut mjCTexture, filename: std__string, vfs: *const mjVFS) {
    todo!() // cxx:__ZN10mjCTexture14LoadCubeSingleENSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEEPK6mjVFS_
}

/// C: mjCTexture::LoadCubeSeparate (user/user_objects.cc:5325)
/// Calls: cxx:__Z14mjuu_strippathNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEE, cxx:__ZN10mjCTexture8LoadFlipENSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEEPK6mjVFS_RNS0_6vectorISt4byteNS4_ISB_EEEERjSF_Rb, cxx:__ZNK6mujoco4user8FilePath3StrEv, cxx:_mjs_getString
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_texture_load_cube_separate(self_ptr: *mut mjCTexture, vfs: *const mjVFS) {
    todo!() // cxx:__ZN10mjCTexture16LoadCubeSeparateEPK6mjVFS_
}

/// C: mjCTexture::Compile (user/user_objects.cc:5411)
/// Calls: cxx:__Z14mjuu_strippathNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEE, cxx:__ZN10mjCTexture11BuiltinCubeEv, cxx:__ZN10mjCTexture12CopyFromSpecEv, cxx:__ZN10mjCTexture12FlipIfNeededERNSt3__16vectorISt4byteNS0_9allocatorIS2_EEEEjj, cxx:__ZN10mjCTexture14LoadCubeSingleENSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEEPK6mjVFS_, cxx:__ZN10mjCTexture16LoadCubeSeparateEPK6mjVFS_, cxx:__ZN10mjCTexture6Load2DENSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEEPK6mjVFS_, cxx:__ZN10mjCTexture9Builtin2DEv, cxx:__ZNK6mujoco4user8FilePath3StrEv, cxx:_mjs_getString
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_texture_compile(self_ptr: *mut mjCTexture, vfs: *const mjVFS) {
    todo!() // cxx:__ZN10mjCTexture7CompileEPK6mjVFS_
}

/// C: mjCMaterial::PointToLocal (user/user_objects.cc:5556)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_material_point_to_local(self_ptr: *mut mjCMaterial) {
    todo!() // cxx:__ZN11mjCMaterial12PointToLocalEv
}

/// C: mjCMaterial::CopyFromSpec (user/user_objects.cc:5564)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_material_copy_from_spec(self_ptr: *mut mjCMaterial) {
    todo!() // cxx:__ZN11mjCMaterial12CopyFromSpecEv
}

/// C: mjCMaterial::NameSpace (user/user_objects.cc:5571)
/// Calls: cxx:__ZN7mjCBase9NameSpaceEPK8mjCModel
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_material_name_space(self_ptr: *mut mjCMaterial, m: *const mjCModel) {
    todo!() // cxx:__ZN11mjCMaterial9NameSpaceEPK8mjCModel
}

/// C: mjCMaterial::Compile (user/user_objects.cc:5582)
/// Calls: cxx:__ZN11mjCMaterial12CopyFromSpecEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_material_compile(self_ptr: *mut mjCMaterial) {
    todo!() // cxx:__ZN11mjCMaterial7CompileEv
}

/// C: mjCPair::PointToLocal (user/user_objects.cc:5639)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_pair_point_to_local(self_ptr: *mut mjCPair) {
    todo!() // cxx:__ZN7mjCPair12PointToLocalEv
}

/// C: mjCPair::NameSpace (user/user_objects.cc:5649)
/// Calls: cxx:__ZN7mjCBase9NameSpaceEPK8mjCModel
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_pair_name_space(self_ptr: *mut mjCPair, m: *const mjCModel) {
    todo!() // cxx:__ZN7mjCPair9NameSpaceEPK8mjCModel
}

/// C: mjCPair::CopyFromSpec (user/user_objects.cc:5656)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_pair_copy_from_spec(self_ptr: *mut mjCPair) {
    todo!() // cxx:__ZN7mjCPair12CopyFromSpecEv
}

/// C: mjCPair::ResolveReferences (user/user_objects.cc:5664)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_pair_resolve_references(self_ptr: *mut mjCPair, m: *const mjCModel) {
    todo!() // cxx:__ZN7mjCPair17ResolveReferencesEPK8mjCModel
}

/// C: mjCPair::Compile (user/user_objects.cc:5710)
/// Calls: cxx:__Z12mjuu_definedd, cxx:__ZN7mjCGeom12SetNotVisualEv, cxx:__ZN7mjCPair12CopyFromSpecEv, cxx:__ZN7mjCPair17ResolveReferencesEPK8mjCModel
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_pair_compile(self_ptr: *mut mjCPair) {
    todo!() // cxx:__ZN7mjCPair7CompileEv
}

/// C: mjCBodyPair::PointToLocal (user/user_objects.cc:5844)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_pair_point_to_local(self_ptr: *mut mjCBodyPair) {
    todo!() // cxx:__ZN11mjCBodyPair12PointToLocalEv
}

/// C: mjCBodyPair::NameSpace (user/user_objects.cc:5854)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_pair_name_space(self_ptr: *mut mjCBodyPair, m: *const mjCModel) {
    todo!() // cxx:__ZN11mjCBodyPair9NameSpaceEPK8mjCModel
}

/// C: mjCBodyPair::CopyFromSpec (user/user_objects.cc:5861)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_pair_copy_from_spec(self_ptr: *mut mjCBodyPair) {
    todo!() // cxx:__ZN11mjCBodyPair12CopyFromSpecEv
}

/// C: mjCBodyPair::ResolveReferences (user/user_objects.cc:5869)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_pair_resolve_references(self_ptr: *mut mjCBodyPair, m: *const mjCModel) {
    todo!() // cxx:__ZN11mjCBodyPair17ResolveReferencesEPK8mjCModel
}

/// C: mjCBodyPair::Compile (user/user_objects.cc:5913)
/// Calls: cxx:__ZN11mjCBodyPair12CopyFromSpecEv, cxx:__ZN11mjCBodyPair17ResolveReferencesEPK8mjCModel
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_pair_compile(self_ptr: *mut mjCBodyPair) {
    todo!() // cxx:__ZN11mjCBodyPair7CompileEv
}

/// C: mjCEquality::PointToLocal (user/user_objects.cc:5966)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_equality_point_to_local(self_ptr: *mut mjCEquality) {
    todo!() // cxx:__ZN11mjCEquality12PointToLocalEv
}

/// C: mjCEquality::NameSpace (user/user_objects.cc:5976)
/// Calls: cxx:__ZN7mjCBase9NameSpaceEPK8mjCModel
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_equality_name_space(self_ptr: *mut mjCEquality, m: *const mjCModel) {
    todo!() // cxx:__ZN11mjCEquality9NameSpaceEPK8mjCModel
}

/// C: mjCEquality::CopyFromSpec (user/user_objects.cc:5983)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_equality_copy_from_spec(self_ptr: *mut mjCEquality) {
    todo!() // cxx:__ZN11mjCEquality12CopyFromSpecEv
}

/// C: mjCEquality::ResolveReferences (user/user_objects.cc:5991)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_equality_resolve_references(self_ptr: *mut mjCEquality, m: *const mjCModel) {
    todo!() // cxx:__ZN11mjCEquality17ResolveReferencesEPK8mjCModel
}

/// C: mjCEquality::Compile (user/user_objects.cc:6055)
/// Calls: cxx:__ZN11mjCEquality12CopyFromSpecEv, cxx:__ZN11mjCEquality17ResolveReferencesEPK8mjCModel, cxx:__ZNK8mjCModel6FlexesEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_equality_compile(self_ptr: *mut mjCEquality) {
    todo!() // cxx:__ZN11mjCEquality7CompileEv
}

/// C: mjCTendon::is_limited (user/user_objects.cc:6119)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.cc:__ZL9islimitediPKd
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tendon_is_limited(self_ptr: *mut mjCTendon) -> bool {
    todo!() // cxx:__ZNK9mjCTendon10is_limitedEv
}

/// C: mjCTendon::is_actfrclimited (user/user_objects.cc:6122)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.cc:__ZL9islimitediPKd
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tendon_is_actfrclimited(self_ptr: *mut mjCTendon) -> bool {
    todo!() // cxx:__ZNK9mjCTendon16is_actfrclimitedEv
}

/// C: mjCTendon::PointToLocal (user/user_objects.cc:6126)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tendon_point_to_local(self_ptr: *mut mjCTendon) {
    todo!() // cxx:__ZN9mjCTendon12PointToLocalEv
}

/// C: mjCTendon::NameSpace (user/user_objects.cc:6136)
/// Calls: cxx:__ZN7mjCBase9NameSpaceEPK8mjCModel
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tendon_name_space(self_ptr: *mut mjCTendon, m: *const mjCModel) {
    todo!() // cxx:__ZN9mjCTendon9NameSpaceEPK8mjCModel
}

/// C: mjCTendon::CopyFromSpec (user/user_objects.cc:6143)
/// Calls: cxx:__ZNK7mjCWrap4TypeEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tendon_copy_from_spec(self_ptr: *mut mjCTendon) {
    todo!() // cxx:__ZN9mjCTendon12CopyFromSpecEv
}

/// C: mjCTendon::SetModel (user/user_objects.cc:6166)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tendon_set_model(self_ptr: *mut mjCTendon, _model: *mut mjCModel) {
    todo!() // cxx:__ZN9mjCTendon8SetModelEP8mjCModel
}

/// C: mjCTendon::WrapSite (user/user_objects.cc:6174)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tendon_wrap_site(self_ptr: *mut mjCTendon, wrapname: std__string, wrapinfo: std__string_view) {
    todo!() // cxx:__ZN9mjCTendon8WrapSiteENSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEENS0_17basic_string_viewIcS3_EE
}

/// C: mjCTendon::WrapGeom (user/user_objects.cc:6188)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tendon_wrap_geom(self_ptr: *mut mjCTendon, wrapname: std__string, side: std__string, wrapinfo: std__string_view) {
    todo!() // cxx:__ZN9mjCTendon8WrapGeomENSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEES6_NS0_17basic_string_viewIcS3_EE
}

/// C: mjCTendon::WrapJoint (user/user_objects.cc:6203)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tendon_wrap_joint(self_ptr: *mut mjCTendon, wrapname: std__string, coef: f64, wrapinfo: std__string_view) {
    todo!() // cxx:__ZN9mjCTendon9WrapJointENSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEEdNS0_17basic_string_viewIcS3_EE
}

/// C: mjCTendon::WrapPulley (user/user_objects.cc:6218)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tendon_wrap_pulley(self_ptr: *mut mjCTendon, divisor: f64, wrapinfo: std__string_view) {
    todo!() // cxx:__ZN9mjCTendon10WrapPulleyEdNSt3__117basic_string_viewIcNS0_11char_traitsIcEEEE
}

/// C: mjCTendon::NumWraps (user/user_objects.cc:6232)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tendon_num_wraps(self_ptr: *mut mjCTendon) -> i32 {
    todo!() // cxx:__ZNK9mjCTendon8NumWrapsEv
}

/// C: mjCTendon::GetWrap (user/user_objects.cc:6238)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tendon_get_wrap(self_ptr: *mut mjCTendon, i: i32) -> *const mjCWrap {
    todo!() // cxx:__ZNK9mjCTendon7GetWrapEi
}

/// C: mjCTendon::ResolveReferences (user/user_objects.cc:6244)
/// Calls: cxx:__ZN7mjCWrap17ResolveReferencesEPK8mjCModel, cxx:__ZNK7mjCWrap4TypeEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tendon_resolve_references(self_ptr: *mut mjCTendon, m: *const mjCModel) {
    todo!() // cxx:__ZN9mjCTendon17ResolveReferencesEPK8mjCModel
}

/// C: mjCTendon::Compile (user/user_objects.cc:6273)
/// Calls: cxx-internal:user_objects.cc.o:__ZL12checklimitedPK7mjCBasebPKcS3_ib, cxx:__ZN7mjCGeom12SetNotVisualEv, cxx:__ZN7mjCWrap7CompileEv, cxx:__ZN9mjCTendon12CopyFromSpecEv, cxx:__ZN9mjCTendon17ResolveReferencesEPK8mjCModel, cxx:__ZNK7mjCWrap4TypeEv, cxx:__ZNK8mjCModel5GeomsEv, cxx:__ZNK9mjCTendon10is_limitedEv, cxx:__ZNK9mjCTendon16is_actfrclimitedEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tendon_compile(self_ptr: *mut mjCTendon) {
    todo!() // cxx:__ZN9mjCTendon7CompileEv
}

/// C: mjCWrap::PointToLocal (user/user_objects.cc:6481)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_wrap_point_to_local(self_ptr: *mut mjCWrap) {
    todo!() // cxx:__ZN7mjCWrap12PointToLocalEv
}

/// C: mjCWrap::CopyFromSpec (user/user_objects.cc:6486)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_wrap_copy_from_spec(self_ptr: *mut mjCWrap) {
    todo!() // cxx:__ZN7mjCWrap12CopyFromSpecEv
}

/// C: mjCWrap::NameSpace (user/user_objects.cc:6490)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_wrap_name_space(self_ptr: *mut mjCWrap, m: *const mjCModel) {
    todo!() // cxx:__ZN7mjCWrap9NameSpaceEPK8mjCModel
}

/// C: mjCWrap::Compile (user/user_objects.cc:6495)
/// Calls: cxx:__ZN7mjCWrap12CopyFromSpecEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_wrap_compile(self_ptr: *mut mjCWrap) {
    todo!() // cxx:__ZN7mjCWrap7CompileEv
}

/// C: mjCWrap::ResolveReferences (user/user_objects.cc:6499)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_wrap_resolve_references(self_ptr: *mut mjCWrap, m: *const mjCModel) {
    todo!() // cxx:__ZN7mjCWrap17ResolveReferencesEPK8mjCModel
}

/// C: mjCActuator::ForgetKeyframes (user/user_objects.cc:6642)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_actuator_forget_keyframes(self_ptr: *mut mjCActuator) {
    todo!() // cxx:__ZN11mjCActuator15ForgetKeyframesEv
}

/// C: mjCActuator::is_ctrllimited (user/user_objects.cc:6648)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.cc:__ZL9islimitediPKd
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_actuator_is_ctrllimited(self_ptr: *mut mjCActuator) -> bool {
    todo!() // cxx:__ZNK11mjCActuator14is_ctrllimitedEv
}

/// C: mjCActuator::is_forcelimited (user/user_objects.cc:6651)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.cc:__ZL9islimitediPKd
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_actuator_is_forcelimited(self_ptr: *mut mjCActuator) -> bool {
    todo!() // cxx:__ZNK11mjCActuator15is_forcelimitedEv
}

/// C: mjCActuator::is_actlimited (user/user_objects.cc:6654)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_objects.cc:__ZL9islimitediPKd
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_actuator_is_actlimited(self_ptr: *mut mjCActuator) -> bool {
    todo!() // cxx:__ZNK11mjCActuator13is_actlimitedEv
}

/// C: mjCActuator::act (user/user_objects.cc:6659)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_actuator_act(self_ptr: *mut mjCActuator, state_name: *const std__string) -> *const () {
    todo!() // cxx:__ZN11mjCActuator3actERKNSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEE
}

/// C: mjCActuator::ctrl (user/user_objects.cc:6667)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_actuator_ctrl(self_ptr: *mut mjCActuator, state_name: *const std__string) -> *mut f64 {
    todo!() // cxx:__ZN11mjCActuator4ctrlERKNSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEE
}

/// C: mjCActuator::PointToLocal (user/user_objects.cc:6673)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_actuator_point_to_local(self_ptr: *mut mjCActuator) {
    todo!() // cxx:__ZN11mjCActuator12PointToLocalEv
}

/// C: mjCActuator::NameSpace (user/user_objects.cc:6689)
/// Calls: cxx:__ZN7mjCBase9NameSpaceEPK8mjCModel
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_actuator_name_space(self_ptr: *mut mjCActuator, m: *const mjCModel) {
    todo!() // cxx:__ZN11mjCActuator9NameSpaceEPK8mjCModel
}

/// C: mjCActuator::CopyFromSpec (user/user_objects.cc:6700)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_actuator_copy_from_spec(self_ptr: *mut mjCActuator) {
    todo!() // cxx:__ZN11mjCActuator12CopyFromSpecEv
}

/// C: mjCActuator::CopyPlugin (user/user_objects.cc:6714)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_actuator_copy_plugin(self_ptr: *mut mjCActuator) {
    todo!() // cxx:__ZN11mjCActuator10CopyPluginEv
}

/// C: mjCActuator::ResolveReferences (user/user_objects.cc:6719)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_actuator_resolve_references(self_ptr: *mut mjCActuator, m: *const mjCModel) {
    todo!() // cxx:__ZN11mjCActuator17ResolveReferencesEPK8mjCModel
}

/// C: mjCActuator::Compile (user/user_objects.cc:6797)
/// Calls: cxx-internal:user_objects.cc.o:__ZL12checklimitedPK7mjCBasebPKcS3_ib, cxx:__ZN11mjCActuator12CopyFromSpecEv, cxx:__ZN11mjCActuator17ResolveReferencesEPK8mjCModel, cxx:__ZN8mjCModel13ResolvePluginEP7mjCBaseRKNSt3__112basic_stringIcNS2_11char_traitsIcEENS2_9allocatorIcEEEESA_PP9mjCPlugin, cxx:__ZN9mjCTendon9get_rangeEv, cxx:__ZNK11mjCActuator13is_actlimitedEv, cxx:__ZNK11mjCActuator14is_ctrllimitedEv, cxx:__ZNK11mjCActuator15is_forcelimitedEv, cxx:__ZNK8mjCJoint9get_rangeEv, cxx:_mjp_getPluginAtSlot
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_actuator_compile(self_ptr: *mut mjCActuator) {
    todo!() // cxx:__ZN11mjCActuator7CompileEv
}

/// C: mjCSensor::PointToLocal (user/user_objects.cc:7311)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_sensor_point_to_local(self_ptr: *mut mjCSensor) {
    todo!() // cxx:__ZN9mjCSensor12PointToLocalEv
}

/// C: mjCSensor::NameSpace (user/user_objects.cc:7326)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_sensor_name_space(self_ptr: *mut mjCSensor, m: *const mjCModel) {
    todo!() // cxx:__ZN9mjCSensor9NameSpaceEPK8mjCModel
}

/// C: mjCSensor::CopyFromSpec (user/user_objects.cc:7336)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_sensor_copy_from_spec(self_ptr: *mut mjCSensor) {
    todo!() // cxx:__ZN9mjCSensor12CopyFromSpecEv
}

/// C: mjCSensor::CopyPlugin (user/user_objects.cc:7349)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_sensor_copy_plugin(self_ptr: *mut mjCSensor) {
    todo!() // cxx:__ZN9mjCSensor10CopyPluginEv
}

/// C: mjCSensor::ResolveReferences (user/user_objects.cc:7354)
/// Calls: cxx:__ZN7mjCGeom12SetNotVisualEv, cxx:__ZN7mjCMesh12SetNotVisualEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_sensor_resolve_references(self_ptr: *mut mjCSensor, m: *const mjCModel) {
    todo!() // cxx:__ZN9mjCSensor17ResolveReferencesEPK8mjCModel
}

/// C: sensorDatatype (user/user_objects.cc:7429)
#[allow(unused_variables, non_snake_case)]
pub fn sensor_datatype(r#type: u32) -> u32 {
    todo!() // cxx:__Z14sensorDatatype9mjtSensor
}

/// C: sensorNeedstage (user/user_objects.cc:7493)
#[allow(unused_variables, non_snake_case)]
pub fn sensor_needstage(r#type: u32) -> u32 {
    todo!() // cxx:__Z15sensorNeedstage9mjtSensor
}

/// C: mjCSensor::Compile (user/user_objects.cc:7555)
/// Calls: cxx:__Z14sensorDatatype9mjtSensor, cxx:__Z15sensorNeedstage9mjtSensor, cxx:__ZN8mjCModel13ResolvePluginEP7mjCBaseRKNSt3__112basic_stringIcNS2_11char_traitsIcEENS2_9allocatorIcEEEESA_PP9mjCPlugin, cxx:__ZN9mjCSensor12CopyFromSpecEv, cxx:__ZN9mjCSensor17ResolveReferencesEPK8mjCModel, cxx:__ZNK7mjCGeom4TypeEv, cxx:__ZNK8mjCJoint10is_limitedEv, cxx:__ZNK9mjCTendon10is_limitedEv, cxx:_mjp_getPluginAtSlot, cxx:_mjs_sensorDim
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_sensor_compile(self_ptr: *mut mjCSensor) {
    todo!() // cxx:__ZN9mjCSensor7CompileEv
}

/// C: mjCNumeric::PointToLocal (user/user_objects.cc:7929)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_numeric_point_to_local(self_ptr: *mut mjCNumeric) {
    todo!() // cxx:__ZN10mjCNumeric12PointToLocalEv
}

/// C: mjCNumeric::CopyFromSpec (user/user_objects.cc:7937)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_numeric_copy_from_spec(self_ptr: *mut mjCNumeric) {
    todo!() // cxx:__ZN10mjCNumeric12CopyFromSpecEv
}

/// C: mjCNumeric::Compile (user/user_objects.cc:7951)
/// Calls: cxx:__ZN10mjCNumeric12CopyFromSpecEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_numeric_compile(self_ptr: *mut mjCNumeric) {
    todo!() // cxx:__ZN10mjCNumeric7CompileEv
}

/// C: mjCText::PointToLocal (user/user_objects.cc:8010)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_text_point_to_local(self_ptr: *mut mjCText) {
    todo!() // cxx:__ZN7mjCText12PointToLocalEv
}

/// C: mjCText::CopyFromSpec (user/user_objects.cc:8018)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_text_copy_from_spec(self_ptr: *mut mjCText) {
    todo!() // cxx:__ZN7mjCText12CopyFromSpecEv
}

/// C: mjCText::Compile (user/user_objects.cc:8032)
/// Calls: cxx:__ZN7mjCText12CopyFromSpecEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_text_compile(self_ptr: *mut mjCText) {
    todo!() // cxx:__ZN7mjCText7CompileEv
}

/// C: mjCTuple::PointToLocal (user/user_objects.cc:8083)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tuple_point_to_local(self_ptr: *mut mjCTuple) {
    todo!() // cxx:__ZN8mjCTuple12PointToLocalEv
}

/// C: mjCTuple::NameSpace (user/user_objects.cc:8094)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tuple_name_space(self_ptr: *mut mjCTuple, m: *const mjCModel) {
    todo!() // cxx:__ZN8mjCTuple9NameSpaceEPK8mjCModel
}

/// C: mjCTuple::CopyFromSpec (user/user_objects.cc:8102)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tuple_copy_from_spec(self_ptr: *mut mjCTuple) {
    todo!() // cxx:__ZN8mjCTuple12CopyFromSpecEv
}

/// C: mjCTuple::ResolveReferences (user/user_objects.cc:8124)
/// Calls: cxx:__ZN7mjCGeom12SetNotVisualEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tuple_resolve_references(self_ptr: *mut mjCTuple, m: *const mjCModel) {
    todo!() // cxx:__ZN8mjCTuple17ResolveReferencesEPK8mjCModel
}

/// C: mjCTuple::Compile (user/user_objects.cc:8157)
/// Calls: cxx:__ZN8mjCTuple12CopyFromSpecEv, cxx:__ZN8mjCTuple17ResolveReferencesEPK8mjCModel
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tuple_compile(self_ptr: *mut mjCTuple) {
    todo!() // cxx:__ZN8mjCTuple7CompileEv
}

/// C: mjCKey::PointToLocal (user/user_objects.cc:8207)
/// Calls: cxx:__ZN11mjCActuator3actERKNSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEE, cxx:__ZN11mjCActuator4ctrlERKNSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEE, cxx:__ZN7mjCBody4mposERKNSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEE, cxx:__ZN7mjCBody5mquatERKNSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEE, cxx:__ZN8mjCJoint4qposERKNSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEE, cxx:__ZN8mjCJoint4qvelERKNSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEE
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_key_point_to_local(self_ptr: *mut mjCKey) {
    todo!() // cxx:__ZN6mjCKey12PointToLocalEv
}

/// C: mjCKey::CopyFromSpec (user/user_objects.cc:8225)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_key_copy_from_spec(self_ptr: *mut mjCKey) {
    todo!() // cxx:__ZN6mjCKey12CopyFromSpecEv
}

/// C: mjCKey::Compile (user/user_objects.cc:8255)
/// Calls: cxx:__ZN6mjCKey12CopyFromSpecEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_key_compile(self_ptr: *mut mjCKey, m: *const mjModel) {
    todo!() // cxx:__ZN6mjCKey7CompileEPK8mjModel_
}

/// C: mjCPlugin::PointToLocal (user/user_objects.cc:8390)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_plugin_point_to_local(self_ptr: *mut mjCPlugin) {
    todo!() // cxx:__ZN9mjCPlugin12PointToLocalEv
}

/// C: mjCPlugin::Compile (user/user_objects.cc:8398)
/// Calls: cxx:__ZN8mjCModel13ResolvePluginEP7mjCBaseRKNSt3__112basic_stringIcNS2_11char_traitsIcEENS2_9allocatorIcEEEESA_PP9mjCPlugin, cxx:_mjp_getPluginAtSlot
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_plugin_compile(self_ptr: *mut mjCPlugin) {
    todo!() // cxx:__ZN9mjCPlugin7CompileEv
}

/// C: mjCBoundingVolume::Contype (user/user_objects.h:123)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_contype(self_ptr: *mut mjCBoundingVolume) -> i32 {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).contype_ }
}

/// C: mjCBoundingVolume::Conaffinity (user/user_objects.h:124)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_conaffinity(self_ptr: *mut mjCBoundingVolume) -> i32 {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).conaffinity_ }
}

/// C: mjCBoundingVolume::AABB (user/user_objects.h:125)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_aabb__f91f459ff16927a5(self_ptr: *mut mjCBoundingVolume) -> *const f64 {
    todo!() // cxx:__ZNK17mjCBoundingVolume4AABBEv
}

/// C: mjCBoundingVolume::AABB (user/user_objects.h:126)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_aabb__bef70a2abaacd237(self_ptr: *mut mjCBoundingVolume, i: i32) -> f64 {
    todo!() // cxx:__ZNK17mjCBoundingVolume4AABBEi
}

/// C: mjCBoundingVolume::Pos (user/user_objects.h:127)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_pos__d9995d865a4070c0(self_ptr: *mut mjCBoundingVolume) -> *const f64 {
    todo!() // cxx:__ZNK17mjCBoundingVolume3PosEv
}

/// C: mjCBoundingVolume::Pos (user/user_objects.h:128)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_pos__95aecb8f440b706f(self_ptr: *mut mjCBoundingVolume, i: i32) -> f64 {
    todo!() // cxx:__ZNK17mjCBoundingVolume3PosEi
}

/// C: mjCBoundingVolume::Quat (user/user_objects.h:129)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_quat(self_ptr: *mut mjCBoundingVolume) -> *const f64 {
    todo!() // cxx:__ZNK17mjCBoundingVolume4QuatEv
}

/// C: mjCBoundingVolume::Id (user/user_objects.h:130)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_id(self_ptr: *mut mjCBoundingVolume) -> *const i32 {
    todo!() // cxx:__ZNK17mjCBoundingVolume2IdEv
}

/// C: mjCBoundingVolume::SetContype (user/user_objects.h:132)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_set_contype(self_ptr: *mut mjCBoundingVolume, val: i32) {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).contype_ = val; }
}

/// C: mjCBoundingVolume::SetConaffinity (user/user_objects.h:133)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_set_conaffinity(self_ptr: *mut mjCBoundingVolume, val: i32) {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).conaffinity_ = val; }
}

/// C: mjCBoundingVolume::SetAABB (user/user_objects.h:134)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_set_aabb(self_ptr: *mut mjCBoundingVolume, aabb: *const f64) {
    todo!() // cxx:__ZN17mjCBoundingVolume7SetAABBEPKd
}

/// C: mjCBoundingVolume::SetPos (user/user_objects.h:135)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_set_pos(self_ptr: *mut mjCBoundingVolume, pos: *const f64) {
    todo!() // cxx:__ZN17mjCBoundingVolume6SetPosEPKd
}

/// C: mjCBoundingVolume::SetQuat (user/user_objects.h:136)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_set_quat(self_ptr: *mut mjCBoundingVolume, quat: *const f64) {
    todo!() // cxx:__ZN17mjCBoundingVolume7SetQuatEPKd
}

/// C: mjCBoundingVolume::SetId (user/user_objects.h:140)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_set_id__36523affc3f9a04e(self_ptr: *mut mjCBoundingVolume, id: *const i32) {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).id_ = id; }
}

/// C: mjCBoundingVolume::SetId (user/user_objects.h:141)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_set_id__9fa4dc447dfb6240(self_ptr: *mut mjCBoundingVolume, val: i32) {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).idval_ = val; }
}

/// C: mjCBoundingVolumeHierarchy::Nbvh (user/user_objects.h:194)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_hierarchy_nbvh(self_ptr: *mut mjCBoundingVolumeHierarchy) -> i32 {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).nbvh_ }
}

/// C: mjCBoundingVolumeHierarchy::Bvh (user/user_objects.h:195)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_hierarchy_bvh(self_ptr: *mut mjCBoundingVolumeHierarchy) -> *const () {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).bvh_ }
}

/// C: mjCBoundingVolumeHierarchy::Child (user/user_objects.h:196)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_hierarchy_child(self_ptr: *mut mjCBoundingVolumeHierarchy) -> *const () {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).child_ }
}

/// C: mjCBoundingVolumeHierarchy::Nodeid (user/user_objects.h:197)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_hierarchy_nodeid__c5962117dca39b32(self_ptr: *mut mjCBoundingVolumeHierarchy) -> *const () {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).nodeid_ }
}

/// C: mjCBoundingVolumeHierarchy::Nodeid (user/user_objects.h:198)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_hierarchy_nodeid__5535530f702bd01d(self_ptr: *mut mjCBoundingVolumeHierarchy, id: i32) -> i32 {
    todo!() // cxx:__ZNK26mjCBoundingVolumeHierarchy6NodeidEi
}

/// C: mjCBoundingVolumeHierarchy::Nodeidptr (user/user_objects.h:199)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_hierarchy_nodeidptr(self_ptr: *mut mjCBoundingVolumeHierarchy, id: i32) -> *const i32 {
    todo!() // cxx:__ZNK26mjCBoundingVolumeHierarchy9NodeidptrEi
}

/// C: mjCBoundingVolumeHierarchy::Level (user/user_objects.h:200)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_hierarchy_level(self_ptr: *mut mjCBoundingVolumeHierarchy) -> *const () {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).level_ }
}

/// C: mjCBoundingVolumeHierarchy::Size (user/user_objects.h:201)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_hierarchy_size(self_ptr: *mut mjCBoundingVolumeHierarchy) -> i32 {
    todo!() // cxx:__ZNK26mjCBoundingVolumeHierarchy4SizeEv
}

/// C: mjCOctree::NumNodes (user/user_objects.h:300)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_num_nodes(self_ptr: *mut mjCOctree) -> i32 {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).nnode_ }
}

/// C: mjCOctree::NumVerts (user/user_objects.h:301)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_num_verts(self_ptr: *mut mjCOctree) -> i32 {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).nvert_ }
}

/// C: mjCOctree::Vert (user/user_objects.h:306)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_vert(self_ptr: *mut mjCOctree, i: i32) -> *const f64 {
    todo!() // cxx:__ZNK9mjCOctree4VertEi
}

/// C: mjCOctree::Hang (user/user_objects.h:307)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_hang(self_ptr: *mut mjCOctree, i: i32) -> *const () {
    todo!() // cxx:__ZNK9mjCOctree4HangEi
}

/// C: mjCOctree::VertId (user/user_objects.h:308)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_vert_id(self_ptr: *mut mjCOctree, n: i32, v: i32) -> i32 {
    todo!() // cxx:__ZNK9mjCOctree6VertIdEii
}

/// C: mjCOctree::Children (user/user_objects.h:309)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_children(self_ptr: *mut mjCOctree, i: i32) -> *const () {
    todo!() // cxx:__ZNK9mjCOctree8ChildrenEi
}

/// C: mjCOctree::Size (user/user_objects.h:311)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_size(self_ptr: *mut mjCOctree) -> i32 {
    todo!() // cxx:__ZNK9mjCOctree4SizeEv
}

/// C: mjCOctree::Clear (user/user_objects.h:316)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_clear(self_ptr: *mut mjCOctree) {
    todo!() // cxx:__ZN9mjCOctree5ClearEv
}

/// C: mjCOctree::AddCoeff (user/user_objects.h:323)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_add_coeff(self_ptr: *mut mjCOctree, n: i32, v: i32, coeff: f64) {
    todo!() // cxx:__ZN9mjCOctree8AddCoeffEiid
}

/// C: mjCOctree::Coeff (user/user_objects.h:324)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_coeff(self_ptr: *mut mjCOctree, n: i32, v: i32) -> f64 {
    todo!() // cxx:__ZNK9mjCOctree5CoeffEii
}

/// C: mjCOctree::SetMaxDepth (user/user_objects.h:327)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_set_max_depth(self_ptr: *mut mjCOctree, depth: i32) {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).max_depth_ = depth; }
}

/// C: mjCOctree::MaxDepth (user/user_objects.h:328)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_max_depth(self_ptr: *mut mjCOctree) -> i32 {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).max_depth_ }
}

/// C: mjCOctree::SetSmoothingIterations (user/user_objects.h:331)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_set_smoothing_iterations(self_ptr: *mut mjCOctree, iterations: i32) {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).smoothing_iterations_ = iterations; }
}

/// C: mjCOctree::SmoothingIterations (user/user_objects.h:332)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_smoothing_iterations(self_ptr: *mut mjCOctree) -> i32 {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).smoothing_iterations_ }
}

/// C: mjCBase::CopyFromSpec (user/user_objects.h:388)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_base_copy_from_spec(self_ptr: *mut mjCBase) {
    todo!() // cxx:__ZN7mjCBase12CopyFromSpecEv
}

/// C: mjCBase::ResolveReferences (user/user_objects.h:391)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_base_resolve_references(self_ptr: *mut mjCBase, m: *const mjCModel) {
    todo!() // cxx:__ZN7mjCBase17ResolveReferencesEPK8mjCModel
}

/// C: mjCBase::CopyPlugin (user/user_objects.h:397)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_base_copy_plugin(self_ptr: *mut mjCBase) {
    todo!() // cxx:__ZN7mjCBase10CopyPluginEv
}

/// C: mjCBase::GetParent (user/user_objects.h:400)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_base_get_parent(self_ptr: *mut mjCBase) -> *mut mjCBase {
    todo!() // cxx:__ZNK7mjCBase9GetParentEv
}

/// C: mjCBase::ForgetKeyframes (user/user_objects.h:415)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_base_forget_keyframes__6ef1231ae2d4431f(self_ptr: *mut mjCBase) {
    todo!() // cxx:__ZN7mjCBase15ForgetKeyframesEv
}

/// C: mjCBase::ForgetKeyframes (user/user_objects.h:416)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_base_forget_keyframes__e3a86f9f7a718487(self_ptr: *mut mjCBase) {
    todo!() // cxx:__ZNK7mjCBase15ForgetKeyframesEv
}

/// C: mjCBase::AddRef (user/user_objects.h:421)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_base_add_ref(self_ptr: *mut mjCBase) {
    todo!() // cxx:__ZN7mjCBase6AddRefEv
}

/// C: mjCBase::GetRef (user/user_objects.h:422)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_base_get_ref(self_ptr: *mut mjCBase) -> i32 {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).refcount }
}

/// C: mjCBase::Release (user/user_objects.h:423)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_base_release(self_ptr: *mut mjCBase) {
    todo!() // cxx:__ZN7mjCBase7ReleaseEv
}

/// C: mjCBody::get_userdata (user/user_objects.h:567)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_get_userdata(self_ptr: *mut mjCBody) -> *const () {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).userdata_ }
}

/// C: mjCBody::SetParent (user/user_objects.h:590)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_set_parent(self_ptr: *mut mjCBody, _body: *mut mjCBody) {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).parent = _body; }
}

/// C: mjCBody::GetParent (user/user_objects.h:591)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_get_parent(self_ptr: *mut mjCBody) -> *mut mjCBody {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).parent }
}

/// C: mjCBody::Bodies (user/user_objects.h:600)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_bodies(self_ptr: *mut mjCBody) -> *const () {
    todo!() // cxx:__ZNK7mjCBody6BodiesEv
}

/// C: mjCFrame::SetParent (user/user_objects.h:668)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_frame_set_parent(self_ptr: *mut mjCFrame, _body: *mut mjCBody) {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).body = _body; }
}

/// C: mjCFrame::GetParent (user/user_objects.h:669)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_frame_get_parent(self_ptr: *mut mjCFrame) -> *mut mjCBody {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).body }
}

/// C: mjCJoint::SetParent (user/user_objects.h:718)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_joint_set_parent(self_ptr: *mut mjCJoint, _body: *mut mjCBody) {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).body = _body; }
}

/// C: mjCJoint::GetParent (user/user_objects.h:719)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_joint_get_parent(self_ptr: *mut mjCJoint) -> *mut mjCBody {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).body }
}

/// C: mjCJoint::get_userdata (user/user_objects.h:722)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_joint_get_userdata(self_ptr: *mut mjCJoint) -> *const () {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).userdata_ }
}

/// C: mjCJoint::get_range (user/user_objects.h:723)
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

/// C: mjCJoint::nq (user/user_objects.h:730)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_joint_nq__e7b2a6296d35ace9(self_ptr: *mut mjCJoint) -> i32 {
    todo!() // cxx:__ZNK8mjCJoint2nqEv
}

/// C: mjCJoint::nv (user/user_objects.h:731)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_joint_nv__5bcf64a4f65e6fbe(self_ptr: *mut mjCJoint) -> i32 {
    todo!() // cxx:__ZNK8mjCJoint2nvEv
}

/// C: mjCGeom::IsVisual (user/user_objects.h:795)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_geom_is_visual(self_ptr: *mut mjCGeom) -> bool {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).visual_ }
}

/// C: mjCGeom::SetNotVisual (user/user_objects.h:796)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_geom_set_not_visual(self_ptr: *mut mjCGeom) {
    todo!() // cxx:__ZN7mjCGeom12SetNotVisualEv
}

/// C: mjCGeom::SetParent (user/user_objects.h:797)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_geom_set_parent(self_ptr: *mut mjCGeom, _body: *mut mjCBody) {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).body = _body; }
}

/// C: mjCGeom::GetParent (user/user_objects.h:798)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_geom_get_parent(self_ptr: *mut mjCGeom) -> *mut mjCBody {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).body }
}

/// C: mjCGeom::Type (user/user_objects.h:799)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_geom_type(self_ptr: *mut mjCGeom) -> u32 {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).r#type }
}

/// C: mjCGeom::get_userdata (user/user_objects.h:807)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_geom_get_userdata(self_ptr: *mut mjCGeom) -> *const () {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).userdata_ }
}

/// C: mjCGeom::get_hfieldname (user/user_objects.h:808)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_geom_get_hfieldname(self_ptr: *mut mjCGeom) -> *const std__string {
    todo!() // cxx:__ZNK7mjCGeom14get_hfieldnameEv
}

/// C: mjCGeom::get_meshname (user/user_objects.h:809)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_geom_get_meshname(self_ptr: *mut mjCGeom) -> *const std__string {
    todo!() // cxx:__ZNK7mjCGeom12get_meshnameEv
}

/// C: mjCGeom::del_material (user/user_objects.h:811)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_geom_del_material(self_ptr: *mut mjCGeom) {
    todo!() // cxx:__ZN7mjCGeom12del_materialEv
}

/// C: mjCSite::Body (user/user_objects.h:861)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_site_body(self_ptr: *mut mjCSite) -> *mut mjCBody {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).body }
}

/// C: mjCSite::SetParent (user/user_objects.h:862)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_site_set_parent(self_ptr: *mut mjCSite, _body: *mut mjCBody) {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).body = _body; }
}

/// C: mjCSite::GetParent (user/user_objects.h:863)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_site_get_parent(self_ptr: *mut mjCSite) -> *mut mjCBody {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).body }
}

/// C: mjCSite::Mesh (user/user_objects.h:866)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_site_mesh(self_ptr: *mut mjCSite) -> *mut mjCMesh {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).mesh }
}

/// C: mjCSite::SetMesh (user/user_objects.h:867)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_site_set_mesh(self_ptr: *mut mjCSite, _mesh: *mut mjCMesh) {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).mesh = _mesh; }
}

/// C: mjCSite::get_userdata (user/user_objects.h:873)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_site_get_userdata(self_ptr: *mut mjCSite) -> *const () {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).userdata_ }
}

/// C: mjCSite::del_material (user/user_objects.h:875)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_site_del_material(self_ptr: *mut mjCSite) {
    todo!() // cxx:__ZN7mjCSite12del_materialEv
}

/// C: mjCSite::get_meshname (user/user_objects.h:879)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_site_get_meshname(self_ptr: *mut mjCSite) -> *const std__string {
    todo!() // cxx:__ZNK7mjCSite12get_meshnameEv
}

/// C: mjCSite::del_meshname (user/user_objects.h:880)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_site_del_meshname(self_ptr: *mut mjCSite) {
    todo!() // cxx:__ZN7mjCSite12del_meshnameEv
}

/// C: mjCCamera::get_targetbody (user/user_objects.h:920)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_camera_get_targetbody(self_ptr: *mut mjCCamera) -> *const std__string {
    todo!() // cxx:__ZNK9mjCCamera14get_targetbodyEv
}

/// C: mjCCamera::get_userdata (user/user_objects.h:921)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_camera_get_userdata(self_ptr: *mut mjCCamera) -> *const () {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).userdata_ }
}

/// C: mjCCamera::SetParent (user/user_objects.h:923)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_camera_set_parent(self_ptr: *mut mjCCamera, _body: *mut mjCBody) {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).body = _body; }
}

/// C: mjCCamera::GetParent (user/user_objects.h:924)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_camera_get_parent(self_ptr: *mut mjCCamera) -> *mut mjCBody {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).body }
}

/// C: mjCLight::get_targetbody (user/user_objects.h:964)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_light_get_targetbody(self_ptr: *mut mjCLight) -> *const std__string {
    todo!() // cxx:__ZNK8mjCLight14get_targetbodyEv
}

/// C: mjCLight::get_texture (user/user_objects.h:965)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_light_get_texture(self_ptr: *mut mjCLight) -> *const std__string {
    todo!() // cxx:__ZNK8mjCLight11get_textureEv
}

/// C: mjCLight::SetParent (user/user_objects.h:967)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_light_set_parent(self_ptr: *mut mjCLight, _body: *mut mjCBody) {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).body = _body; }
}

/// C: mjCLight::GetParent (user/user_objects.h:968)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_light_get_parent(self_ptr: *mut mjCLight) -> *mut mjCBody {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).body }
}

/// C: mjCFlex::get_material (user/user_objects.h:1057)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_get_material(self_ptr: *mut mjCFlex) -> *const std__string {
    todo!() // cxx:__ZNK7mjCFlex12get_materialEv
}

/// C: mjCFlex::get_vertbody (user/user_objects.h:1058)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_get_vertbody(self_ptr: *mut mjCFlex) -> *const () {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).vertbody_ }
}

/// C: mjCFlex::get_vert (user/user_objects.h:1059)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_get_vert(self_ptr: *mut mjCFlex) -> *const () {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).vert_ }
}

/// C: mjCFlex::get_elemaabb (user/user_objects.h:1060)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_get_elemaabb(self_ptr: *mut mjCFlex) -> *const () {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).elemaabb_ }
}

/// C: mjCFlex::get_elem (user/user_objects.h:1061)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_get_elem(self_ptr: *mut mjCFlex) -> *const () {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).elem_ }
}

/// C: mjCFlex::get_texcoord (user/user_objects.h:1062)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_get_texcoord(self_ptr: *mut mjCFlex) -> *const () {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).texcoord_ }
}

/// C: mjCFlex::get_elemtexcoord (user/user_objects.h:1063)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_get_elemtexcoord(self_ptr: *mut mjCFlex) -> *const () {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).elemtexcoord_ }
}

/// C: mjCFlex::get_nodebody (user/user_objects.h:1064)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_get_nodebody(self_ptr: *mut mjCFlex) -> *const () {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).nodebody_ }
}

/// C: mjCFlex::get_node (user/user_objects.h:1065)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_get_node(self_ptr: *mut mjCFlex) -> *const () {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).node_ }
}

/// C: mjCMesh::Plugin (user/user_objects.h:1188)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_plugin(self_ptr: *mut mjCMesh) -> *const mjsPlugin {
    todo!() // cxx:__ZNK7mjCMesh6PluginEv
}

/// C: mjCMesh::ContentType (user/user_objects.h:1189)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_content_type(self_ptr: *mut mjCMesh) -> *const std__string {
    todo!() // cxx:__ZNK7mjCMesh11ContentTypeEv
}

/// C: mjCMesh::File (user/user_objects.h:1190)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_file(self_ptr: *mut mjCMesh) -> *const std__string {
    todo!() // cxx:__ZNK7mjCMesh4FileEv
}

/// C: mjCMesh::Refpos (user/user_objects.h:1191)
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

/// C: mjCMesh::Refquat (user/user_objects.h:1192)
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

/// C: mjCMesh::Scale (user/user_objects.h:1193)
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

/// C: mjCMesh::SmoothNormal (user/user_objects.h:1194)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_smooth_normal(self_ptr: *mut mjCMesh) -> bool {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).smoothnormal }
}

/// C: mjCMesh::Vert (user/user_objects.h:1195)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_vert__9705cebee72c48c9(self_ptr: *mut mjCMesh) -> *const () {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).vert_ }
}

/// C: mjCMesh::Vert (user/user_objects.h:1196)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_vert__9d7db7f8a5d289af(self_ptr: *mut mjCMesh, i: i32) -> f32 {
    todo!() // cxx:__ZNK7mjCMesh4VertEi
}

/// C: mjCMesh::UserVert (user/user_objects.h:1197)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_user_vert(self_ptr: *mut mjCMesh) -> *const () {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).spec_vert_ }
}

/// C: mjCMesh::UserNormal (user/user_objects.h:1198)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_user_normal(self_ptr: *mut mjCMesh) -> *const () {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).spec_normal_ }
}

/// C: mjCMesh::Texcoord (user/user_objects.h:1199)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_texcoord(self_ptr: *mut mjCMesh) -> *const () {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).texcoord_ }
}

/// C: mjCMesh::FaceTexcoord (user/user_objects.h:1200)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_face_texcoord(self_ptr: *mut mjCMesh) -> *const () {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).facetexcoord_ }
}

/// C: mjCMesh::UserTexcoord (user/user_objects.h:1201)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_user_texcoord(self_ptr: *mut mjCMesh) -> *const () {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).spec_texcoord_ }
}

/// C: mjCMesh::Face (user/user_objects.h:1202)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_face(self_ptr: *mut mjCMesh) -> *const () {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).face_ }
}

/// C: mjCMesh::UserFace (user/user_objects.h:1203)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_user_face(self_ptr: *mut mjCMesh) -> *const () {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).spec_face_ }
}

/// C: mjCMesh::Inertia (user/user_objects.h:1204)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_inertia(self_ptr: *mut mjCMesh) -> u32 {
    todo!() // cxx:__ZNK7mjCMesh7InertiaEv
}

/// C: mjCMesh::Material (user/user_objects.h:1205)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_material(self_ptr: *mut mjCMesh) -> *const std__string {
    todo!() // cxx:__ZNK7mjCMesh8MaterialEv
}

/// C: mjCMesh::SetNeedHull (user/user_objects.h:1208)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_set_need_hull(self_ptr: *mut mjCMesh, needhull: bool) {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).needhull_ = needhull; }
}

/// C: mjCMesh::aamm (user/user_objects.h:1211)
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

/// C: mjCMesh::nvert (user/user_objects.h:1214)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_nvert(self_ptr: *mut mjCMesh) -> i32 {
    todo!() // cxx:__ZNK7mjCMesh5nvertEv
}

/// C: mjCMesh::nnormal (user/user_objects.h:1215)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_nnormal(self_ptr: *mut mjCMesh) -> i32 {
    todo!() // cxx:__ZNK7mjCMesh7nnormalEv
}

/// C: mjCMesh::ntexcoord (user/user_objects.h:1216)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_ntexcoord(self_ptr: *mut mjCMesh) -> i32 {
    todo!() // cxx:__ZNK7mjCMesh9ntexcoordEv
}

/// C: mjCMesh::nface (user/user_objects.h:1217)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_nface(self_ptr: *mut mjCMesh) -> i32 {
    todo!() // cxx:__ZNK7mjCMesh5nfaceEv
}

/// C: mjCMesh::npolygon (user/user_objects.h:1218)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_npolygon(self_ptr: *mut mjCMesh) -> i32 {
    todo!() // cxx:__ZNK7mjCMesh8npolygonEv
}

/// C: mjCMesh::npolygonvert (user/user_objects.h:1219)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_npolygonvert(self_ptr: *mut mjCMesh) -> i32 {
    todo!() // cxx:__ZNK7mjCMesh12npolygonvertEv
}

/// C: mjCMesh::npolygonmap (user/user_objects.h:1224)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_npolygonmap(self_ptr: *mut mjCMesh) -> i32 {
    todo!() // cxx:__ZNK7mjCMesh11npolygonmapEv
}

/// C: mjCMesh::szgraph (user/user_objects.h:1231)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_szgraph(self_ptr: *mut mjCMesh) -> i32 {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).szgraph_ }
}

/// C: mjCMesh::tree (user/user_objects.h:1234)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_tree(self_ptr: *mut mjCMesh) -> *const mjCBoundingVolumeHierarchy {
    todo!() // cxx:__ZN7mjCMesh4treeEv
}

/// C: mjCMesh::octree (user/user_objects.h:1237)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_octree(self_ptr: *mut mjCMesh) -> *const mjCOctree {
    todo!() // cxx:__ZN7mjCMesh6octreeEv
}

/// C: mjCMesh::mutable_octree (user/user_objects.h:1238)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_mutable_octree(self_ptr: *mut mjCMesh) -> *mut mjCOctree {
    todo!() // cxx:__ZN7mjCMesh14mutable_octreeEv
}

/// C: mjCMesh::IsVisual (user/user_objects.h:1248)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_is_visual(self_ptr: *mut mjCMesh) -> bool {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).visual_ }
}

/// C: mjCMesh::SetNotVisual (user/user_objects.h:1249)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_set_not_visual(self_ptr: *mut mjCMesh) {
    todo!() // cxx:__ZN7mjCMesh12SetNotVisualEv
}

/// C: mjCMesh::GraphFaces (user/user_objects.h:1318)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_graph_faces(self_ptr: *mut mjCMesh) -> *mut i32 {
    todo!() // cxx:__ZNK7mjCMesh10GraphFacesEv
}

/// C: mjCSkin::File (user/user_objects.h:1380)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_skin_file(self_ptr: *mut mjCSkin) -> *const std__string {
    todo!() // cxx:__ZNK7mjCSkin4FileEv
}

/// C: mjCSkin::get_material (user/user_objects.h:1381)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_skin_get_material(self_ptr: *mut mjCSkin) -> *const std__string {
    todo!() // cxx:__ZNK7mjCSkin12get_materialEv
}

/// C: mjCSkin::get_vert (user/user_objects.h:1382)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_skin_get_vert(self_ptr: *mut mjCSkin) -> *const () {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).vert_ }
}

/// C: mjCSkin::get_texcoord (user/user_objects.h:1383)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_skin_get_texcoord(self_ptr: *mut mjCSkin) -> *const () {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).texcoord_ }
}

/// C: mjCSkin::get_face (user/user_objects.h:1384)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_skin_get_face(self_ptr: *mut mjCSkin) -> *const () {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).face_ }
}

/// C: mjCSkin::get_bodyname (user/user_objects.h:1385)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_skin_get_bodyname(self_ptr: *mut mjCSkin) -> *const () {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).bodyname_ }
}

/// C: mjCSkin::get_bindpos (user/user_objects.h:1386)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_skin_get_bindpos(self_ptr: *mut mjCSkin) -> *const () {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).bindpos_ }
}

/// C: mjCSkin::get_bindquat (user/user_objects.h:1387)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_skin_get_bindquat(self_ptr: *mut mjCSkin) -> *const () {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).bindquat_ }
}

/// C: mjCSkin::get_vertid (user/user_objects.h:1388)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_skin_get_vertid(self_ptr: *mut mjCSkin) -> *const () {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).vertid_ }
}

/// C: mjCSkin::get_vertweight (user/user_objects.h:1389)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_skin_get_vertweight(self_ptr: *mut mjCSkin) -> *const () {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).vertweight_ }
}

/// C: mjCSkin::del_material (user/user_objects.h:1390)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_skin_del_material(self_ptr: *mut mjCSkin) {
    todo!() // cxx:__ZN7mjCSkin12del_materialEv
}

/// C: mjCHField::File (user/user_objects.h:1436)
#[allow(unused_variables, non_snake_case)]
pub fn mj_ch_field_file(self_ptr: *mut mjCHField) -> std__string {
    todo!() // cxx:__ZNK9mjCHField4FileEv
}

/// C: mjCHField::get_userdata (user/user_objects.h:1439)
#[allow(unused_variables, non_snake_case)]
pub fn mj_ch_field_get_userdata(self_ptr: *mut mjCHField) -> *const () {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).userdata_ }
}

/// C: mjCTexture::File (user/user_objects.h:1485)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_texture_file(self_ptr: *mut mjCTexture) -> std__string {
    todo!() // cxx:__ZNK10mjCTexture4FileEv
}

/// C: mjCTexture::get_content_type (user/user_objects.h:1486)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_texture_get_content_type(self_ptr: *mut mjCTexture) -> std__string {
    todo!() // cxx:__ZNK10mjCTexture16get_content_typeEv
}

/// C: mjCTexture::get_cubefiles (user/user_objects.h:1487)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_texture_get_cubefiles(self_ptr: *mut mjCTexture) -> *const () {
    todo!() // cxx:__ZNK10mjCTexture13get_cubefilesEv
}

/// C: mjCMaterial::get_texture (user/user_objects.h:1550)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_material_get_texture(self_ptr: *mut mjCMaterial, i: i32) -> *const std__string {
    todo!() // cxx:__ZNK11mjCMaterial11get_textureEi
}

/// C: mjCMaterial::del_textures (user/user_objects.h:1551)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_material_del_textures(self_ptr: *mut mjCMaterial) {
    todo!() // cxx:__ZN11mjCMaterial12del_texturesEv
}

/// C: mjCPair::get_geomname1 (user/user_objects.h:1591)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_pair_get_geomname1(self_ptr: *mut mjCPair) -> *const std__string {
    todo!() // cxx:__ZNK7mjCPair13get_geomname1Ev
}

/// C: mjCPair::get_geomname2 (user/user_objects.h:1592)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_pair_get_geomname2(self_ptr: *mut mjCPair) -> *const std__string {
    todo!() // cxx:__ZNK7mjCPair13get_geomname2Ev
}

/// C: mjCPair::GetSignature (user/user_objects.h:1594)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_pair_get_signature(self_ptr: *mut mjCPair) -> i32 {
    todo!() // cxx:__ZN7mjCPair12GetSignatureEv
}

/// C: mjCBodyPair::get_bodyname1 (user/user_objects.h:1636)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_pair_get_bodyname1(self_ptr: *mut mjCBodyPair) -> std__string {
    todo!() // cxx:__ZNK11mjCBodyPair13get_bodyname1Ev
}

/// C: mjCBodyPair::get_bodyname2 (user/user_objects.h:1637)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_pair_get_bodyname2(self_ptr: *mut mjCBodyPair) -> std__string {
    todo!() // cxx:__ZNK11mjCBodyPair13get_bodyname2Ev
}

/// C: mjCBodyPair::GetSignature (user/user_objects.h:1639)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_pair_get_signature(self_ptr: *mut mjCBodyPair) -> i32 {
    todo!() // cxx:__ZN11mjCBodyPair12GetSignatureEv
}

/// C: mjCTendon::set_material (user/user_objects.h:1711)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tendon_set_material(self_ptr: *mut mjCTendon, _material: std__string) {
    todo!() // cxx:__ZN9mjCTendon12set_materialENSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEE
}

/// C: mjCTendon::get_material (user/user_objects.h:1712)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tendon_get_material(self_ptr: *mut mjCTendon) -> *const std__string {
    todo!() // cxx:__ZNK9mjCTendon12get_materialEv
}

/// C: mjCTendon::del_material (user/user_objects.h:1713)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tendon_del_material(self_ptr: *mut mjCTendon) {
    todo!() // cxx:__ZN9mjCTendon12del_materialEv
}

/// C: mjCTendon::get_userdata (user/user_objects.h:1727)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tendon_get_userdata(self_ptr: *mut mjCTendon) -> *const () {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).userdata_ }
}

/// C: mjCTendon::get_range (user/user_objects.h:1728)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tendon_get_range(self_ptr: *mut mjCTendon) -> *const f64 {
    todo!() // cxx:__ZN9mjCTendon9get_rangeEv
}

/// C: mjCWrap::Type (user/user_objects.h:1766)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_wrap_type(self_ptr: *mut mjCWrap) -> u32 {
    todo!() // cxx:__ZNK7mjCWrap4TypeEv
}

/// C: mjCActuator::get_userdata (user/user_objects.h:1863)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_actuator_get_userdata(self_ptr: *mut mjCActuator) -> *const () {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).userdata_ }
}

/// C: mjCActuator::get_target (user/user_objects.h:1864)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_actuator_get_target(self_ptr: *mut mjCActuator) -> *const std__string {
    todo!() // cxx:__ZNK11mjCActuator10get_targetEv
}

/// C: mjCActuator::get_slidersite (user/user_objects.h:1865)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_actuator_get_slidersite(self_ptr: *mut mjCActuator) -> *const std__string {
    todo!() // cxx:__ZNK11mjCActuator14get_slidersiteEv
}

/// C: mjCActuator::get_refsite (user/user_objects.h:1866)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_actuator_get_refsite(self_ptr: *mut mjCActuator) -> *const std__string {
    todo!() // cxx:__ZNK11mjCActuator11get_refsiteEv
}

/// C: mjCSensor::get_userdata (user/user_objects.h:1920)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_sensor_get_userdata(self_ptr: *mut mjCSensor) -> *const () {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).userdata_ }
}

/// C: mjCSensor::get_objname (user/user_objects.h:1921)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_sensor_get_objname(self_ptr: *mut mjCSensor) -> *const std__string {
    todo!() // cxx:__ZN9mjCSensor11get_objnameEv
}

/// C: mjCSensor::get_refname (user/user_objects.h:1922)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_sensor_get_refname(self_ptr: *mut mjCSensor) -> *const std__string {
    todo!() // cxx:__ZN9mjCSensor11get_refnameEv
}

/// C: mjCSensor::get_obj (user/user_objects.h:1924)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_sensor_get_obj(self_ptr: *mut mjCSensor) -> *const mjCBase {
    todo!() // cxx:__ZN9mjCSensor7get_objEv
}

/// C: mjCSensor::get_ref (user/user_objects.h:1925)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_sensor_get_ref(self_ptr: *mut mjCSensor) -> *const mjCBase {
    todo!() // cxx:__ZN9mjCSensor7get_refEv
}

/// C: mjCDef::Joint (user/user_objects.h:2098)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_def_joint(self_ptr: *mut mjCDef) -> *mut mjCJoint {
    todo!() // cxx:__ZN6mjCDef5JointEv
}

/// C: mjCDef::Geom (user/user_objects.h:2099)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_def_geom(self_ptr: *mut mjCDef) -> *mut mjCGeom {
    todo!() // cxx:__ZN6mjCDef4GeomEv
}

/// C: mjCDef::Site (user/user_objects.h:2100)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_def_site(self_ptr: *mut mjCDef) -> *mut mjCSite {
    todo!() // cxx:__ZN6mjCDef4SiteEv
}

/// C: mjCDef::Camera (user/user_objects.h:2101)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_def_camera(self_ptr: *mut mjCDef) -> *mut mjCCamera {
    todo!() // cxx:__ZN6mjCDef6CameraEv
}

/// C: mjCDef::Light (user/user_objects.h:2102)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_def_light(self_ptr: *mut mjCDef) -> *mut mjCLight {
    todo!() // cxx:__ZN6mjCDef5LightEv
}

/// C: mjCDef::Flex (user/user_objects.h:2103)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_def_flex(self_ptr: *mut mjCDef) -> *mut mjCFlex {
    todo!() // cxx:__ZN6mjCDef4FlexEv
}

/// C: mjCDef::Mesh (user/user_objects.h:2104)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_def_mesh(self_ptr: *mut mjCDef) -> *mut mjCMesh {
    todo!() // cxx:__ZN6mjCDef4MeshEv
}

/// C: mjCDef::Material (user/user_objects.h:2105)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_def_material(self_ptr: *mut mjCDef) -> *mut mjCMaterial {
    todo!() // cxx:__ZN6mjCDef8MaterialEv
}

/// C: mjCDef::Pair (user/user_objects.h:2106)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_def_pair(self_ptr: *mut mjCDef) -> *mut mjCPair {
    todo!() // cxx:__ZN6mjCDef4PairEv
}

/// C: mjCDef::Equality (user/user_objects.h:2107)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_def_equality(self_ptr: *mut mjCDef) -> *mut mjCEquality {
    todo!() // cxx:__ZN6mjCDef8EqualityEv
}

/// C: mjCDef::Tendon (user/user_objects.h:2108)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_def_tendon(self_ptr: *mut mjCDef) -> *mut mjCTendon {
    todo!() // cxx:__ZN6mjCDef6TendonEv
}

/// C: mjCDef::Actuator (user/user_objects.h:2109)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_def_actuator(self_ptr: *mut mjCDef) -> *mut mjCActuator {
    todo!() // cxx:__ZN6mjCDef8ActuatorEv
}

