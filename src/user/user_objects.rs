//! Port of: user/user_objects.cc
//! IR hash: c6d98e4f4b63b7f2
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: PNGImage::Load (user/user_objects.cc:58)
/// Calls: mju_readResource
#[allow(unused_variables, non_snake_case)]
pub fn png_image_load(obj: *const mjCBase, resource: *mut mjResource, color_type: u32) -> PNGImage {
    // NOTE: signature changed from previous IR version
    // Previous params: (obj : * const mjCBase, resource : * mut mjResource, color_type : u32)
    // Previous return: PNGImage
    todo!("re-translate: params renamed")
}

/// C: PNGImage::Width (user/user_objects.cc:60)
#[allow(unused_variables, non_snake_case)]
pub fn png_image_width(self_ptr: *mut PNGImage) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut PNGImage)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: PNGImage::Height (user/user_objects.cc:61)
#[allow(unused_variables, non_snake_case)]
pub fn png_image_height(self_ptr: *mut PNGImage) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut PNGImage)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: PNGImage::IsSRGB (user/user_objects.cc:62)
#[allow(unused_variables, non_snake_case)]
pub fn png_image_is_srgb(self_ptr: *mut PNGImage) -> bool {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut PNGImage)
    // Previous return: bool
    todo!("re-translate: params renamed")
}

/// C: PNGImage::MoveData (user/user_objects.cc:66)
#[allow(unused_variables, non_snake_case)]
pub fn png_image_move_data(self_ptr: *mut PNGImage) -> *mut *mut i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut PNGImage)
    // Previous return: * mut * mut i32
    todo!("re-translate: params renamed")
}

/// C: PNGImage::Size (user/user_objects.cc:69)
#[allow(unused_variables, non_snake_case)]
pub fn png_image_size(self_ptr: *mut PNGImage) -> std__size_t {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut PNGImage)
    // Previous return: std__size_t
    todo!("re-translate: params renamed")
}

/// C: MapFrame (user/user_objects.cc:139)
#[allow(unused_variables, non_snake_case)]
pub fn map_frame(parent: *mut i32, child: *mut i32, frame: *mut mjCFrame, parent_body: *mut mjCBody) {
    // NOTE: signature changed from previous IR version
    // Previous params: (parent : * mut i32, child : * mut i32, frame : * mut mjCFrame, parent_body : * mut mjCBody)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: checksize (user/user_objects.cc:153)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn checksize(size: *mut f64, r#type: u32, object: *mut mjCBase, name: *const i8, id: i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (size : * mut f64, r#type : u32, object : * mut mjCBase, name : * const i8, id : i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: checklimited (user/user_objects.cc:172)
#[allow(unused_variables, non_snake_case)]
pub fn checklimited(obj: *const mjCBase, autolimits: bool, entity: *const i8, attr: *const i8, limited: i32, hasrange: bool) {
    // NOTE: signature changed from previous IR version
    // Previous params: (obj : * const mjCBase, autolimits : bool, entity : * const i8, attr : * const i8, limited : i32, hasrange : bool)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: islimited (user/user_objects.cc:185)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn islimited(limited: i32, range: *const f64) -> bool {
    // NOTE: signature changed from previous IR version
    // Previous params: (limited : i32, range : * const f64)
    // Previous return: bool
    todo!("re-translate: params renamed")
}

/// C: mjCBoundingVolumeHierarchy::Make (user/user_objects.cc:404)
/// Calls: mjuu_rotVecQuat
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_hierarchy_make(self_ptr: *mut mjCBoundingVolumeHierarchy, elements: *mut i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCBoundingVolumeHierarchy, elements : * mut i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCBoundingVolumeHierarchy::MakeBVH (user/user_objects.cc:424)
/// Calls: mjuu_rotVecQuat
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_hierarchy_make_bvh(self_ptr: *mut mjCBoundingVolumeHierarchy, elements_begin: i32, elements_end: i32, lev: i32, model: *mut mjCModel, owner: *const mjCBase) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCBoundingVolumeHierarchy, elements_begin : i32, elements_end : i32, lev : i32, model : * mut mjCModel, owner : * const mjCBase)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: mjCOctree::SetFace (user/user_objects.cc:588)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_set_face(self_ptr: *mut mjCOctree, vert: *const i32, face: *const i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCOctree, vert : * const i32, face : * const i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCOctree::Make (user/user_objects.cc:600)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_make(self_ptr: *mut mjCOctree, elements: *mut i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCOctree, elements : * mut i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: pointBoxDistSq (user/user_objects.cc:635)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn point_box_dist_sq(p: *const f64, aabb: *const f64) -> f64 {
    // NOTE: signature changed from previous IR version
    // Previous params: (p : * const f64, aabb : * const f64)
    // Previous return: f64
    todo!("re-translate: params renamed")
}

/// C: pointTriDistSqWithUV (user/user_objects.cc:652)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn point_tri_dist_sq_with_uv(p: *const f64, v0: *const f64, v1: *const f64, v2: *const f64, out_u: *mut f64, out_v: *mut f64) -> f64 {
    // NOTE: signature changed from previous IR version
    // Previous params: (p : * const f64, v0 : * const f64, v1 : * const f64, v2 : * const f64, out_u : * mut f64, out_v : * mut f64)
    // Previous return: f64
    todo!("re-translate: params renamed")
}

/// C: queryClosestBVHWithFace (user/user_objects.cc:737)
/// Calls: pointBoxDistSq, pointTriDistSqWithUV
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn query_closest_bvh_with_face(bvh: *const f64, child: *const i32, nodeid: *const i32, vert: *const f64, face: *const i32, node_idx: i32, p: *const f64, best_dist_sq: *mut f64, best_face: *mut i32, best_u: *mut f64, best_v: *mut f64) {
    // NOTE: signature changed from previous IR version
    // Previous params: (bvh : * const f64, child : * const i32, nodeid : * const i32, vert : * const f64, face : * const i32, node_idx : i32, p : * const f64, best_dist_sq : * mut f64, best_face : * mut i32, best_u : * mut f64, best_v : * mut f64)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: querySignedDistance (user/user_objects.cc:776)
/// Calls: mjuu_normvec, queryClosestBVHWithFace
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn query_signed_distance(bvh: *const f64, child: *const i32, nodeid: *const i32, nbvh: i32, point: *const f64, vert: *const f64, face: *const i32) -> f64 {
    // NOTE: signature changed from previous IR version
    // Previous params: (bvh : * const f64, child : * const i32, nodeid : * const i32, nbvh : i32, point : * const f64, vert : * const f64, face : * const i32)
    // Previous return: f64
    todo!("re-translate: params renamed")
}

/// C: dot2 (user/user_objects.cc:923)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn dot2(a: *const f64, b: *const f64) -> f64 {
    // NOTE: signature changed from previous IR version
    // Previous params: (a : * const f64, b : * const f64)
    // Previous return: f64
    todo!("re-translate: params renamed")
}

/// C: boxTriangle (user/user_objects.cc:929)
/// Calls: dot2, mju_max, mjuu_crossvec, mjuu_dot3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn box_triangle(v: *const Triangle, aamm: *const f64) -> bool {
    // NOTE: signature changed from previous IR version
    // Previous params: (v : * const Triangle, aamm : * const f64)
    // Previous return: bool
    todo!("re-translate: params renamed")
}

/// C: mjCOctree::TaskToNode (user/user_objects.cc:980)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_task_to_node(self_ptr: *mut mjCOctree, task: *const OctreeTask, node: *mut OctNode, vert_map: *mut i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCOctree, task : * const OctreeTask, node : * mut OctNode, vert_map : * mut i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCOctree::Subdivide (user/user_objects.cc:1014)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_subdivide(self_ptr: *mut mjCOctree, task: *const OctreeTask, vert_map: *mut i32, queue: *mut i32, colliding: *const i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCOctree, task : * const OctreeTask, vert_map : * mut i32, queue : * mut i32, colliding : * const i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCOctree::BalanceOctree (user/user_objects.cc:1113)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_balance_octree(self_ptr: *mut mjCOctree, vert_map: *mut i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCOctree, vert_map : * mut i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCOctree::MakeOctree (user/user_objects.cc:1257)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_make_octree(self_ptr: *mut mjCOctree, elements: *const i32, aamm: *const f64, vert_map: *mut i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCOctree, elements : * const i32, aamm : * const f64, vert_map : * mut i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCBody::CopyList (user/user_objects.cc:1784)
/// Calls: mjCFrame::IsAncestor
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_copy_list(self_ptr: *mut mjCBody, dst: *mut i32, src: *const i32, fmap: *mut i32, pframe: *const mjCFrame) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCBody, dst : * mut i32, src : * const i32, fmap : * mut i32, pframe : * const mjCFrame)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCBody::GetList (user/user_objects.cc:2311)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_get_list(self_ptr: *mut mjCBody) -> *const i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCBody)
    // Previous return: * const i32
    todo!("re-translate: params renamed")
}

/// C: GetNextBody (user/user_objects.cc:2380)
#[allow(unused_variables, non_snake_case)]
pub fn get_next_body(body: *const mjCBody, child: *const mjsElement, found: *mut bool, recursive: bool) -> *mut mjsElement {
    // NOTE: signature changed from previous IR version
    // Previous params: (body : * const mjCBody, child : * const mjsElement, found : * mut bool, recursive : bool)
    // Previous return: * mut mjsElement
    todo!("re-translate: params renamed")
}

/// C: randomdot (user/user_objects.cc:4973)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn randomdot(rgb: *mut std__byte, markrgb: *const f64, width: i32, height: i32, probability: f64) {
    // NOTE: signature changed from previous IR version
    // Previous params: (rgb : * mut std__byte, markrgb : * const f64, width : i32, height : i32, probability : f64)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: interp (user/user_objects.cc:4995)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn interp(rgb: *mut std__byte, rgb1: *const f64, rgb2: *const f64, pos: f64) {
    // NOTE: signature changed from previous IR version
    // Previous params: (rgb : * mut std__byte, rgb1 : * const f64, rgb2 : * const f64, pos : f64)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: checker (user/user_objects.cc:5012)
#[allow(unused_variables, non_snake_case)]
pub fn checker(rgb: *mut std__byte, RGB1: *const std__byte, RGB2: *const std__byte, width: i32, height: i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (rgb : * mut std__byte, RGB1 : * const std__byte, RGB2 : * const std__byte, width : i32, height : i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCTexture::LoadPNG (user/user_objects.cc:5220)
/// Calls: PNGImage::Height, PNGImage::IsSRGB, PNGImage::Width
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_texture_load_png(self_ptr: *mut mjCTexture, resource: *mut mjResource, image: *mut i32, w: *mut u32, h: *mut u32, is_srgb: *mut bool) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCTexture, resource : * mut mjResource, image : * mut i32, w : * mut u32, h : * mut u32, is_srgb : * mut bool)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCTexture::LoadKTX (user/user_objects.cc:5244)
/// Calls: mju_readResource
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_texture_load_ktx(self_ptr: *mut mjCTexture, resource: *mut mjResource, image: *mut i32, w: *mut u32, h: *mut u32, is_srgb: *mut bool) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCTexture, resource : * mut mjResource, image : * mut i32, w : * mut u32, h : * mut u32, is_srgb : * mut bool)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCTexture::LoadCustom (user/user_objects.cc:5266)
/// Calls: mju_readResource
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_texture_load_custom(self_ptr: *mut mjCTexture, resource: *mut mjResource, image: *mut i32, w: *mut u32, h: *mut u32, is_srgb: *mut bool) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCTexture, resource : * mut mjResource, image : * mut i32, w : * mut u32, h : * mut u32, is_srgb : * mut bool)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCTexture::FlipIfNeeded (user/user_objects.cc:5304)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_texture_flip_if_needed(self_ptr: *mut mjCTexture, image: *mut i32, w: u32, h: u32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCTexture, image : * mut i32, w : u32, h : u32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCTexture::LoadFlip (user/user_objects.cc:5347)
/// Calls: mjCBase::LoadResource, mj_getCache, mju_closeResource
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_texture_load_flip(self_ptr: *mut mjCTexture, filename: string, vfs: *const mjVFS, image: *mut i32, w: *mut u32, h: *mut u32, is_srgb: *mut bool) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCTexture, filename : string, vfs : * const mjVFS, image : * mut i32, w : * mut u32, h : * mut u32, is_srgb : * mut bool)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCActuator::act (user/user_objects.cc:6953)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_actuator_act(self_ptr: *mut mjCActuator, state_name: *const std__string) -> *mut i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCActuator, state_name : * const std__string)
    // Previous return: * mut i32
    todo!("re-translate: params renamed")
}

/// C: sensorDatatype (user/user_objects.cc:7480)
#[allow(unused_variables, non_snake_case)]
pub fn sensor_datatype(r#type: u32) -> u32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (r#type : u32)
    // Previous return: u32
    todo!("re-translate: params renamed")
}

/// C: sensorNeedstage (user/user_objects.cc:7544)
#[allow(unused_variables, non_snake_case)]
pub fn sensor_needstage(r#type: u32) -> u32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (r#type : u32)
    // Previous return: u32
    todo!("re-translate: params renamed")
}

/// C: ResolveOrientation (user/user_objects.h:89)
/// Calls: mjuu_crossvec, mjuu_dot3, mjuu_frame2quat, mjuu_mulquat, mjuu_normvec, mjuu_setvec, mjuu_z2quat
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn resolve_orientation(quat: *mut f64, degree: bool, sequence: *const i8, orient: *const mjsOrientation) -> *const i8 {
    // NOTE: signature changed from previous IR version
    // Previous params: (quat : * mut f64, degree : bool, sequence : * const i8, orient : * const mjsOrientation)
    // Previous return: * const i8
    todo!("re-translate: params renamed")
}

/// C: mjCBoundingVolume::Contype (user/user_objects.h:122)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_contype(self_ptr: *mut mjCBoundingVolume) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCBoundingVolume)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: mjCBoundingVolume::Conaffinity (user/user_objects.h:123)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_conaffinity(self_ptr: *mut mjCBoundingVolume) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCBoundingVolume)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: mjCBoundingVolume::AABB (user/user_objects.h:124)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_aabb(self_ptr: *mut mjCBoundingVolume) -> *const f64 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCBoundingVolume)
    // Previous return: * const f64
    todo!("re-translate: params renamed")
}

/// C: mjCBoundingVolume::Pos (user/user_objects.h:126)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_pos(self_ptr: *mut mjCBoundingVolume) -> *const f64 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCBoundingVolume)
    // Previous return: * const f64
    todo!("re-translate: params renamed")
}

/// C: mjCBoundingVolume::Quat (user/user_objects.h:128)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_quat(self_ptr: *mut mjCBoundingVolume) -> *const f64 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCBoundingVolume)
    // Previous return: * const f64
    todo!("re-translate: params renamed")
}

/// C: mjCBoundingVolume::Id (user/user_objects.h:129)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_id(self_ptr: *mut mjCBoundingVolume) -> *const i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCBoundingVolume)
    // Previous return: * const i32
    todo!("re-translate: params renamed")
}

/// C: mjCBoundingVolume::SetContype (user/user_objects.h:131)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_set_contype(self_ptr: *mut mjCBoundingVolume, val: i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCBoundingVolume, val : i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCBoundingVolume::SetConaffinity (user/user_objects.h:132)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_set_conaffinity(self_ptr: *mut mjCBoundingVolume, val: i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCBoundingVolume, val : i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCBoundingVolume::SetAABB (user/user_objects.h:133)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_set_aabb(self_ptr: *mut mjCBoundingVolume, aabb: *const f64) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCBoundingVolume, aabb : * const f64)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCBoundingVolume::SetPos (user/user_objects.h:134)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_set_pos(self_ptr: *mut mjCBoundingVolume, pos: *const f64) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCBoundingVolume, pos : * const f64)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCBoundingVolume::SetQuat (user/user_objects.h:135)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_set_quat(self_ptr: *mut mjCBoundingVolume, quat: *const f64) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCBoundingVolume, quat : * const f64)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCBoundingVolume::SetId (user/user_objects.h:139)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_set_id(self_ptr: *mut mjCBoundingVolume, id: *const i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCBoundingVolume, id : * const i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCBoundingVolumeHierarchy::CreateBVH (user/user_objects.h:174)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_hierarchy_create_bvh(self_ptr: *mut mjCBoundingVolumeHierarchy, model: *mut mjCModel, owner: *const mjCBase) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCBoundingVolumeHierarchy, model : * mut mjCModel, owner : * const mjCBase)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCBoundingVolumeHierarchy::Set (user/user_objects.h:175)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_hierarchy_set(self_ptr: *mut mjCBoundingVolumeHierarchy, ipos_element: *mut f64, iquat_element: *mut f64) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCBoundingVolumeHierarchy, ipos_element : * mut f64, iquat_element : * mut f64)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCBoundingVolumeHierarchy::AllocateBoundingVolumes (user/user_objects.h:176)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_hierarchy_allocate_bounding_volumes(self_ptr: *mut mjCBoundingVolumeHierarchy, nleaf: i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCBoundingVolumeHierarchy, nleaf : i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCBoundingVolumeHierarchy::RemoveInactiveVolumes (user/user_objects.h:177)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_hierarchy_remove_inactive_volumes(self_ptr: *mut mjCBoundingVolumeHierarchy, nmax: i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCBoundingVolumeHierarchy, nmax : i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCBoundingVolumeHierarchy::AddBoundingVolume (user/user_objects.h:179)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_hierarchy_add_bounding_volume(self_ptr: *mut mjCBoundingVolumeHierarchy, id: i32, contype: i32, conaffinity: i32, pos: *const f64, quat: *const f64, aabb: *const f64) -> *const mjCBoundingVolume {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCBoundingVolumeHierarchy, id : i32, contype : i32, conaffinity : i32, pos : * const f64, quat : * const f64, aabb : * const f64)
    // Previous return: * const mjCBoundingVolume
    todo!("re-translate: params renamed")
}

/// C: mjCBoundingVolumeHierarchy::Nbvh (user/user_objects.h:186)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_hierarchy_nbvh(self_ptr: *mut mjCBoundingVolumeHierarchy) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCBoundingVolumeHierarchy)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: mjCBoundingVolumeHierarchy::Bvh (user/user_objects.h:187)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_hierarchy_bvh(self_ptr: *mut mjCBoundingVolumeHierarchy) -> *const i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCBoundingVolumeHierarchy)
    // Previous return: * const i32
    todo!("re-translate: params renamed")
}

/// C: mjCBoundingVolumeHierarchy::Child (user/user_objects.h:188)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_hierarchy_child(self_ptr: *mut mjCBoundingVolumeHierarchy) -> *const i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCBoundingVolumeHierarchy)
    // Previous return: * const i32
    todo!("re-translate: params renamed")
}

/// C: mjCBoundingVolumeHierarchy::Nodeid (user/user_objects.h:189)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_hierarchy_nodeid(self_ptr: *mut mjCBoundingVolumeHierarchy) -> *const i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCBoundingVolumeHierarchy)
    // Previous return: * const i32
    todo!("re-translate: params renamed")
}

/// C: mjCBoundingVolumeHierarchy::Nodeidptr (user/user_objects.h:191)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_hierarchy_nodeidptr(self_ptr: *mut mjCBoundingVolumeHierarchy, id: i32) -> *const i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCBoundingVolumeHierarchy, id : i32)
    // Previous return: * const i32
    todo!("re-translate: params renamed")
}

/// C: mjCBoundingVolumeHierarchy::Level (user/user_objects.h:192)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_hierarchy_level(self_ptr: *mut mjCBoundingVolumeHierarchy) -> *const i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCBoundingVolumeHierarchy)
    // Previous return: * const i32
    todo!("re-translate: params renamed")
}

/// C: mjCBoundingVolumeHierarchy::Size (user/user_objects.h:193)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_hierarchy_size(self_ptr: *mut mjCBoundingVolumeHierarchy) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCBoundingVolumeHierarchy)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: mjCBoundingVolumeHierarchy::QuerySignedDistance (user/user_objects.h:200)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_hierarchy_query_signed_distance(self_ptr: *mut mjCBoundingVolumeHierarchy, point: *const f64, vert: *const f64, face: *const i32) -> f64 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCBoundingVolumeHierarchy, point : * const f64, vert : * const f64, face : * const i32)
    // Previous return: f64
    todo!("re-translate: params renamed")
}

/// C: mjCOctree::CreateOctree (user/user_objects.h:285)
/// Calls: mjCOctree::Clear, mjCOctree::MarkHangingNodes
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_create_octree(self_ptr: *mut mjCOctree, aamm: *const f64) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCOctree, aamm : * const f64)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCOctree::NumNodes (user/user_objects.h:287)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_num_nodes(self_ptr: *mut mjCOctree) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCOctree)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: mjCOctree::NumVerts (user/user_objects.h:288)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_num_verts(self_ptr: *mut mjCOctree) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCOctree)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: mjCOctree::CopyLevel (user/user_objects.h:289)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_copy_level(self_ptr: *mut mjCOctree, level: *mut i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCOctree, level : * mut i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCOctree::CopyChild (user/user_objects.h:290)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_copy_child(self_ptr: *mut mjCOctree, child: *mut i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCOctree, child : * mut i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCOctree::CopyAabb (user/user_objects.h:291)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_copy_aabb(self_ptr: *mut mjCOctree, aabb: *mut f64) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCOctree, aabb : * mut f64)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCOctree::CopyCoeff (user/user_objects.h:292)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_copy_coeff(self_ptr: *mut mjCOctree, coeff: *mut f64) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCOctree, coeff : * mut f64)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCOctree::Vert (user/user_objects.h:293)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_vert(self_ptr: *mut mjCOctree, i: i32) -> *const f64 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCOctree, i : i32)
    // Previous return: * const f64
    todo!("re-translate: params renamed")
}

/// C: mjCOctree::Hang (user/user_objects.h:294)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_hang(self_ptr: *mut mjCOctree, i: i32) -> *const i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCOctree, i : i32)
    // Previous return: * const i32
    todo!("re-translate: params renamed")
}

/// C: mjCOctree::VertId (user/user_objects.h:295)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_vert_id(self_ptr: *mut mjCOctree, n: i32, v: i32) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCOctree, n : i32, v : i32)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: mjCOctree::Children (user/user_objects.h:296)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_children(self_ptr: *mut mjCOctree, i: i32) -> *const () {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCOctree, i : i32)
    // Previous return: * const ()
    todo!("re-translate: params renamed")
}

/// C: mjCOctree::Size (user/user_objects.h:298)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_size(self_ptr: *mut mjCOctree) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCOctree)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: mjCOctree::Clear (user/user_objects.h:302)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_clear(self_ptr: *mut mjCOctree) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCOctree)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCOctree::AddCoeff (user/user_objects.h:309)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_add_coeff(self_ptr: *mut mjCOctree, n: i32, v: i32, coeff: f64) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCOctree, n : i32, v : i32, coeff : f64)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCOctree::Coeff (user/user_objects.h:310)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_coeff(self_ptr: *mut mjCOctree, n: i32, v: i32) -> f64 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCOctree, n : i32, v : i32)
    // Previous return: f64
    todo!("re-translate: params renamed")
}

/// C: mjCOctree::SetMaxDepth (user/user_objects.h:313)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_set_max_depth(self_ptr: *mut mjCOctree, depth: i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCOctree, depth : i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCOctree::MaxDepth (user/user_objects.h:314)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_max_depth(self_ptr: *mut mjCOctree) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCOctree)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: mjCOctree::SetSmoothingIterations (user/user_objects.h:317)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_set_smoothing_iterations(self_ptr: *mut mjCOctree, iterations: i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCOctree, iterations : i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCOctree::SmoothingIterations (user/user_objects.h:318)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_smoothing_iterations(self_ptr: *mut mjCOctree) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCOctree)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: mjCOctree::ComputeSdfCoeffs (user/user_objects.h:321)
/// Calls: mjCBoundingVolumeHierarchy::QuerySignedDistance, mjCOctree::AddCoeff, mjCOctree::NumNodes, mjCOctree::Vert, mjCOctree::VertId
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_compute_sdf_coeffs(self_ptr: *mut mjCOctree, vert: *const f64, nvert: i32, face: *const i32, nface: i32, tree: *const mjCBoundingVolumeHierarchy) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCOctree, vert : * const f64, nvert : i32, face : * const i32, nface : i32, tree : * const mjCBoundingVolumeHierarchy)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCOctree::FindNeighbor (user/user_objects.h:332)
/// Calls: mjCOctree::FindCoarseNeighbor
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_find_neighbor(self_ptr: *mut mjCOctree, node_idx: i32, dir: i32) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCOctree, node_idx : i32, dir : i32)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: mjCOctree::FindCoarseNeighbor (user/user_objects.h:333)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_find_coarse_neighbor(self_ptr: *mut mjCOctree, node_idx: i32, dir: i32) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCOctree, node_idx : i32, dir : i32)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: mjCOctree::MarkHangingNodes (user/user_objects.h:335)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_mark_hanging_nodes(self_ptr: *mut mjCOctree) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCOctree)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCBase::LoadResource (user/user_objects.h:358)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_base_load_resource(modelfiledir: *const std__string, filename: *const std__string, vfs: *const mjVFS) -> *mut mjResource {
    // NOTE: signature changed from previous IR version
    // Previous params: (modelfiledir : * const std__string, filename : * const std__string, vfs : * const mjVFS)
    // Previous return: * mut mjResource
    todo!("re-translate: params renamed")
}

/// C: mjCBase::GetAssetContentType (user/user_objects.h:363)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_base_get_asset_content_type(resource_name: string_view, raw_text: string_view) -> std__string {
    // NOTE: signature changed from previous IR version
    // Previous params: (resource_name : string_view, raw_text : string_view)
    // Previous return: std__string
    todo!("re-translate: params renamed")
}

/// C: mjCBase::SetFrame (user/user_objects.h:366)
/// Calls: mjCBase::GetParent
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_base_set_frame(self_ptr: *mut mjCBase, _frame: *mut mjCFrame) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCBase, _frame : * mut mjCFrame)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCBase::CopyFromSpec (user/user_objects.h:369)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_base_copy_from_spec(self_ptr: *mut mjCBase) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCBase)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCBase::ResolveReferences (user/user_objects.h:372)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_base_resolve_references(self_ptr: *mut mjCBase, m: *const mjCModel) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCBase, m : * const mjCModel)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCBase::NameSpace (user/user_objects.h:375)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_base_name_space(self_ptr: *mut mjCBase, m: *const mjCModel) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCBase, m : * const mjCModel)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCBase::CopyPlugin (user/user_objects.h:378)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_base_copy_plugin(self_ptr: *mut mjCBase) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCBase)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCBase::GetParent (user/user_objects.h:381)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_base_get_parent(self_ptr: *mut mjCBase) -> *mut mjCBase {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCBase)
    // Previous return: * mut mjCBase
    todo!("re-translate: params renamed")
}

/// C: mjCBase::FindCompiler (user/user_objects.h:384)
/// Calls: mjCModel::FindSpec
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_base_find_compiler(self_ptr: *mut mjCBase, compiler: *const mjsCompiler) -> *mut mjsCompiler {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCBase, compiler : * const mjsCompiler)
    // Previous return: * mut mjsCompiler
    todo!("re-translate: params renamed")
}

/// C: mjCBase::ForgetKeyframes (user/user_objects.h:396)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_base_forget_keyframes(self_ptr: *mut mjCBase) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCBase)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCBase::AddRef (user/user_objects.h:402)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_base_add_ref(self_ptr: *mut mjCBase) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCBase)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCBase::GetRef (user/user_objects.h:403)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_base_get_ref(self_ptr: *mut mjCBase) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCBase)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: mjCBase::Release (user/user_objects.h:404)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_base_release(self_ptr: *mut mjCBase) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCBase)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCBase::SetUserValue (user/user_objects.h:411)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_base_set_user_value(self_ptr: *mut mjCBase, key: string_view, data: *const (), cleanup: Option<unsafe extern "C" fn()>) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCBase, key : string_view, data : * const (), cleanup : Option < unsafe extern "C" fn () >)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCBase::GetUserValue (user/user_objects.h:413)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_base_get_user_value(self_ptr: *mut mjCBase, key: string_view) -> *const () {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCBase, key : string_view)
    // Previous return: * const ()
    todo!("re-translate: params renamed")
}

/// C: mjCBase::DeleteUserValue (user/user_objects.h:414)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_base_delete_user_value(self_ptr: *mut mjCBase, key: string_view) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCBase, key : string_view)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCBody::AddBody (user/user_objects.h:522)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_add_body(self_ptr: *mut mjCBody, arg0: *mut mjCDef) -> *mut mjCBody {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCBody, arg0 : * mut mjCDef)
    // Previous return: * mut mjCBody
    todo!("re-translate: params renamed")
}

/// C: mjCBody::AddFrame (user/user_objects.h:523)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_add_frame(self_ptr: *mut mjCBody, arg0: *mut mjCFrame) -> *mut mjCFrame {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCBody, arg0 : * mut mjCFrame)
    // Previous return: * mut mjCFrame
    todo!("re-translate: params renamed")
}

/// C: mjCBody::AddJoint (user/user_objects.h:524)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_add_joint(self_ptr: *mut mjCBody, arg0: *mut mjCDef) -> *mut mjCJoint {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCBody, arg0 : * mut mjCDef)
    // Previous return: * mut mjCJoint
    todo!("re-translate: params renamed")
}

/// C: mjCBody::AddFreeJoint (user/user_objects.h:525)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_add_free_joint(self_ptr: *mut mjCBody) -> *mut mjCJoint {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCBody)
    // Previous return: * mut mjCJoint
    todo!("re-translate: params renamed")
}

/// C: mjCBody::AddGeom (user/user_objects.h:526)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_add_geom(self_ptr: *mut mjCBody, arg0: *mut mjCDef) -> *mut mjCGeom {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCBody, arg0 : * mut mjCDef)
    // Previous return: * mut mjCGeom
    todo!("re-translate: params renamed")
}

/// C: mjCBody::AddSite (user/user_objects.h:527)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_add_site(self_ptr: *mut mjCBody, arg0: *mut mjCDef) -> *mut mjCSite {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCBody, arg0 : * mut mjCDef)
    // Previous return: * mut mjCSite
    todo!("re-translate: params renamed")
}

/// C: mjCBody::AddCamera (user/user_objects.h:528)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_add_camera(self_ptr: *mut mjCBody, arg0: *mut mjCDef) -> *mut mjCCamera {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCBody, arg0 : * mut mjCDef)
    // Previous return: * mut mjCCamera
    todo!("re-translate: params renamed")
}

/// C: mjCBody::AddLight (user/user_objects.h:529)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_add_light(self_ptr: *mut mjCBody, arg0: *mut mjCDef) -> *mut mjCLight {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCBody, arg0 : * mut mjCDef)
    // Previous return: * mut mjCLight
    todo!("re-translate: params renamed")
}

/// C: mjCBody::NumObjects (user/user_objects.h:537)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_num_objects(self_ptr: *mut mjCBody, r#type: u32) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCBody, r#type : u32)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: mjCBody::GetObject (user/user_objects.h:538)
/// Calls: mjCBody::NumObjects
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_get_object(self_ptr: *mut mjCBody, r#type: u32, id: i32) -> *mut mjCBase {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCBody, r#type : u32, id : i32)
    // Previous return: * mut mjCBase
    todo!("re-translate: params renamed")
}

/// C: mjCBody::FindObject (user/user_objects.h:539)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_find_object(self_ptr: *mut mjCBody, r#type: u32, name: *const std__string, recursive: bool) -> *mut mjCBase {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCBody, r#type : u32, name : * const std__string, recursive : bool)
    // Previous return: * mut mjCBase
    todo!("re-translate: params renamed")
}

/// C: mjCBody::NameSpace (user/user_objects.h:542)
/// Calls: mjCBody::NameSpace_
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_name_space(self_ptr: *mut mjCBody, m: *const mjCModel) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCBody, m : * const mjCModel)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCBody::MakeInertialExplicit (user/user_objects.h:545)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_make_inertial_explicit(self_ptr: *mut mjCBody) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCBody)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCBody::ComputeBVH (user/user_objects.h:548)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_compute_bvh(self_ptr: *mut mjCBody) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCBody)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCBody::get_userdata (user/user_objects.h:557)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_get_userdata(self_ptr: *mut mjCBody) -> *const i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCBody)
    // Previous return: * const i32
    todo!("re-translate: params renamed")
}

/// C: mjCBody::NextChild (user/user_objects.h:563)
/// Calls: GetNextBody
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_next_child(self_ptr: *mut mjCBody, child: *const mjsElement, r#type: u32, recursive: bool) -> *mut mjsElement {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCBody, child : * const mjsElement, r#type : u32, recursive : bool)
    // Previous return: * mut mjsElement
    todo!("re-translate: params renamed")
}

/// C: mjCBody::ForgetKeyframes (user/user_objects.h:567)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_forget_keyframes(self_ptr: *mut mjCBody) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCBody)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCBody::ToFrame (user/user_objects.h:570)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_to_frame(self_ptr: *mut mjCBody) -> *mut mjCFrame {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCBody)
    // Previous return: * mut mjCFrame
    todo!("re-translate: params renamed")
}

/// C: mjCBody::mpos (user/user_objects.h:573)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_mpos(self_ptr: *mut mjCBody, state_name: *const std__string) -> *mut f64 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCBody, state_name : * const std__string)
    // Previous return: * mut f64
    todo!("re-translate: params renamed")
}

/// C: mjCBody::mquat (user/user_objects.h:574)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_mquat(self_ptr: *mut mjCBody, state_name: *const std__string) -> *mut f64 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCBody, state_name : * const std__string)
    // Previous return: * mut f64
    todo!("re-translate: params renamed")
}

/// C: mjCBody::SetParent (user/user_objects.h:579)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_set_parent(self_ptr: *mut mjCBody, _body: *mut mjCBody) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCBody, _body : * mut mjCBody)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCBody::GetParent (user/user_objects.h:580)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_get_parent(self_ptr: *mut mjCBody) -> *mut mjCBody {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCBody)
    // Previous return: * mut mjCBody
    todo!("re-translate: params renamed")
}

/// C: mjCBody::SetModel (user/user_objects.h:583)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_set_model(self_ptr: *mut mjCBody, _model: *mut mjCModel) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCBody, _model : * mut mjCModel)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCBody::ResetId (user/user_objects.h:586)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_reset_id(self_ptr: *mut mjCBody) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCBody)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCBody::Bodies (user/user_objects.h:589)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_bodies(self_ptr: *mut mjCBody) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCBody)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: mjCBody::AccumulateInertia (user/user_objects.h:597)
/// Calls: mjuu_frameaccum, mjuu_fullInertia, mjuu_globalinertia, mjuu_offcenter, mjuu_setvec
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_accumulate_inertia(self_ptr: *mut mjCBody, other: *const mjsBody, result: *mut mjsBody) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCBody, other : * const mjsBody, result : * mut mjsBody)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCBody::Compile (user/user_objects.h:603)
/// Calls: mjCBody::ComputeBVH, mjCBody::CopyFromSpec, mjCBody::InertiaFromGeom, mjp_getPluginAtSlot
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_compile(self_ptr: *mut mjCBody) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCBody)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCBody::InertiaFromGeom (user/user_objects.h:604)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_inertia_from_geom(self_ptr: *mut mjCBody) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCBody)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCBody::CopyFromSpec (user/user_objects.h:615)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_copy_from_spec(self_ptr: *mut mjCBody) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCBody)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCBody::PointToLocal (user/user_objects.h:616)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_point_to_local(self_ptr: *mut mjCBody) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCBody)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCBody::NameSpace_ (user/user_objects.h:617)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_name_space_1(self_ptr: *mut mjCBody, m: *const mjCModel, propagate: bool) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCBody, m : * const mjCModel, propagate : bool)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCBody::CopyPlugin (user/user_objects.h:618)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_copy_plugin(self_ptr: *mut mjCBody) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCBody)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCFrame::CopyFromSpec (user/user_objects.h:654)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_frame_copy_from_spec(self_ptr: *mut mjCFrame) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCFrame)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCFrame::PointToLocal (user/user_objects.h:655)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_frame_point_to_local(self_ptr: *mut mjCFrame) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCFrame)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCFrame::SetParent (user/user_objects.h:656)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_frame_set_parent(self_ptr: *mut mjCFrame, _body: *mut mjCBody) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCFrame, _body : * mut mjCBody)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCFrame::GetParent (user/user_objects.h:657)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_frame_get_parent(self_ptr: *mut mjCFrame) -> *mut mjCBody {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCFrame)
    // Previous return: * mut mjCBody
    todo!("re-translate: params renamed")
}

/// C: mjCFrame::IsAncestor (user/user_objects.h:661)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_frame_is_ancestor(self_ptr: *mut mjCFrame, child: *const mjCFrame) -> bool {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCFrame, child : * const mjCFrame)
    // Previous return: bool
    todo!("re-translate: params renamed")
}

/// C: mjCFrame::Compile (user/user_objects.h:666)
/// Calls: mjCFrame::CopyFromSpec
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_frame_compile(self_ptr: *mut mjCFrame) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCFrame)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCJoint::CopyFromSpec (user/user_objects.h:706)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_joint_copy_from_spec(self_ptr: *mut mjCJoint) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCJoint)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCJoint::SetParent (user/user_objects.h:707)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_joint_set_parent(self_ptr: *mut mjCJoint, _body: *mut mjCBody) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCJoint, _body : * mut mjCBody)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCJoint::GetParent (user/user_objects.h:708)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_joint_get_parent(self_ptr: *mut mjCJoint) -> *mut mjCBody {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCJoint)
    // Previous return: * mut mjCBody
    todo!("re-translate: params renamed")
}

/// C: mjCJoint::get_userdata (user/user_objects.h:711)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_joint_get_userdata(self_ptr: *mut mjCJoint) -> *const i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCJoint)
    // Previous return: * const i32
    todo!("re-translate: params renamed")
}

/// C: mjCJoint::get_range (user/user_objects.h:712)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_joint_get_range(self_ptr: *mut mjCJoint) -> *const f64 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCJoint)
    // Previous return: * const f64
    todo!("re-translate: params renamed")
}

/// C: mjCJoint::is_limited (user/user_objects.h:714)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_joint_is_limited(self_ptr: *mut mjCJoint) -> bool {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCJoint)
    // Previous return: bool
    todo!("re-translate: params renamed")
}

/// C: mjCJoint::is_actfrclimited (user/user_objects.h:715)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_joint_is_actfrclimited(self_ptr: *mut mjCJoint) -> bool {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCJoint)
    // Previous return: bool
    todo!("re-translate: params renamed")
}

/// C: mjCJoint::nq (user/user_objects.h:719)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_joint_nq(self_ptr: *mut mjCJoint) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCJoint)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: mjCJoint::nv (user/user_objects.h:720)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_joint_nv(self_ptr: *mut mjCJoint) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCJoint)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: mjCJoint::qpos (user/user_objects.h:722)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_joint_qpos(self_ptr: *mut mjCJoint, state_name: *const std__string) -> *mut f64 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCJoint, state_name : * const std__string)
    // Previous return: * mut f64
    todo!("re-translate: params renamed")
}

/// C: mjCJoint::qvel (user/user_objects.h:723)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_joint_qvel(self_ptr: *mut mjCJoint, state_name: *const std__string) -> *mut f64 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCJoint, state_name : * const std__string)
    // Previous return: * mut f64
    todo!("re-translate: params renamed")
}

/// C: mjCJoint::Compile (user/user_objects.h:726)
/// Calls: mjCJoint::CopyFromSpec, mjCJoint::is_actfrclimited, mjCJoint::is_limited
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_joint_compile(self_ptr: *mut mjCJoint) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCJoint)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: mjCJoint::PointToLocal (user/user_objects.h:727)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_joint_point_to_local(self_ptr: *mut mjCJoint) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCJoint)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCGeom::GetVolume (user/user_objects.h:783)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_geom_get_volume(self_ptr: *mut mjCGeom) -> f64 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCGeom)
    // Previous return: f64
    todo!("re-translate: params renamed")
}

/// C: mjCGeom::SetInertia (user/user_objects.h:784)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_geom_set_inertia(self_ptr: *mut mjCGeom) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCGeom)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCGeom::IsVisual (user/user_objects.h:785)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_geom_is_visual(self_ptr: *mut mjCGeom) -> bool {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCGeom)
    // Previous return: bool
    todo!("re-translate: params renamed")
}

/// C: mjCGeom::SetNotVisual (user/user_objects.h:786)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_geom_set_not_visual(self_ptr: *mut mjCGeom) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCGeom)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCGeom::SetParent (user/user_objects.h:787)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_geom_set_parent(self_ptr: *mut mjCGeom, _body: *mut mjCBody) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCGeom, _body : * mut mjCBody)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCGeom::GetParent (user/user_objects.h:788)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_geom_get_parent(self_ptr: *mut mjCGeom) -> *mut mjCBody {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCGeom)
    // Previous return: * mut mjCBody
    todo!("re-translate: params renamed")
}

/// C: mjCGeom::Type (user/user_objects.h:789)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_geom_type(self_ptr: *mut mjCGeom) -> u32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCGeom)
    // Previous return: u32
    todo!("re-translate: params renamed")
}

/// C: mjCGeom::SetFluidCoefs (user/user_objects.h:792)
/// Calls: mjCGeom::GetAddedMassKappa
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_geom_set_fluid_coefs(self_ptr: *mut mjCGeom) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCGeom)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCGeom::GetAddedMassKappa (user/user_objects.h:794)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_geom_get_added_mass_kappa(self_ptr: *mut mjCGeom, dx: f64, dy: f64, dz: f64) -> f64 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCGeom, dx : f64, dy : f64, dz : f64)
    // Previous return: f64
    todo!("re-translate: params renamed")
}

/// C: mjCGeom::get_userdata (user/user_objects.h:797)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_geom_get_userdata(self_ptr: *mut mjCGeom) -> *const i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCGeom)
    // Previous return: * const i32
    todo!("re-translate: params renamed")
}

/// C: mjCGeom::get_hfieldname (user/user_objects.h:798)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_geom_get_hfieldname(self_ptr: *mut mjCGeom) -> *const i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCGeom)
    // Previous return: * const i32
    todo!("re-translate: params renamed")
}

/// C: mjCGeom::get_meshname (user/user_objects.h:799)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_geom_get_meshname(self_ptr: *mut mjCGeom) -> *const i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCGeom)
    // Previous return: * const i32
    todo!("re-translate: params renamed")
}

/// C: mjCGeom::get_material (user/user_objects.h:800)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_geom_get_material(self_ptr: *mut mjCGeom) -> *const std__string {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCGeom)
    // Previous return: * const std__string
    todo!("re-translate: params renamed")
}

/// C: mjCGeom::del_material (user/user_objects.h:801)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_geom_del_material(self_ptr: *mut mjCGeom) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCGeom)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCGeom::Compile (user/user_objects.h:804)
/// Calls: mjCGeom::ComputeAABB, mjCGeom::CopyFromSpec, mjCGeom::GetVolume, mjCGeom::SetFluidCoefs, mjCGeom::SetInertia, mjCMesh::GetPosPtr, mjCMesh::GetQuatPtr, mjp_getPluginAtSlot, mjuu_addtovec, mjuu_rotVecQuat
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_geom_compile(self_ptr: *mut mjCGeom) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCGeom)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCGeom::GetRBound (user/user_objects.h:805)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_geom_get_r_bound(self_ptr: *mut mjCGeom) -> f64 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCGeom)
    // Previous return: f64
    todo!("re-translate: params renamed")
}

/// C: mjCGeom::ComputeAABB (user/user_objects.h:806)
/// Calls: mjuu_setvec
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_geom_compute_aabb(self_ptr: *mut mjCGeom) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCGeom)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCGeom::CopyFromSpec (user/user_objects.h:807)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_geom_copy_from_spec(self_ptr: *mut mjCGeom) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCGeom)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCGeom::PointToLocal (user/user_objects.h:808)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_geom_point_to_local(self_ptr: *mut mjCGeom) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCGeom)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCGeom::NameSpace (user/user_objects.h:809)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_geom_name_space(self_ptr: *mut mjCGeom, m: *const mjCModel) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCGeom, m : * const mjCModel)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCGeom::CopyPlugin (user/user_objects.h:810)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_geom_copy_plugin(self_ptr: *mut mjCGeom) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCGeom)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCSite::Body (user/user_objects.h:849)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_site_body(self_ptr: *mut mjCSite) -> *mut mjCBody {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCSite)
    // Previous return: * mut mjCBody
    todo!("re-translate: params renamed")
}

/// C: mjCSite::SetParent (user/user_objects.h:850)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_site_set_parent(self_ptr: *mut mjCSite, _body: *mut mjCBody) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCSite, _body : * mut mjCBody)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCSite::GetParent (user/user_objects.h:851)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_site_get_parent(self_ptr: *mut mjCSite) -> *mut mjCBody {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCSite)
    // Previous return: * mut mjCBody
    todo!("re-translate: params renamed")
}

/// C: mjCSite::get_userdata (user/user_objects.h:857)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_site_get_userdata(self_ptr: *mut mjCSite) -> *const i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCSite)
    // Previous return: * const i32
    todo!("re-translate: params renamed")
}

/// C: mjCSite::get_material (user/user_objects.h:858)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_site_get_material(self_ptr: *mut mjCSite) -> *const i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCSite)
    // Previous return: * const i32
    todo!("re-translate: params renamed")
}

/// C: mjCSite::del_material (user/user_objects.h:859)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_site_del_material(self_ptr: *mut mjCSite) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCSite)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCSite::Compile (user/user_objects.h:862)
/// Calls: mjCSite::CopyFromSpec
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_site_compile(self_ptr: *mut mjCSite) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCSite)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCSite::CopyFromSpec (user/user_objects.h:863)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_site_copy_from_spec(self_ptr: *mut mjCSite) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCSite)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCSite::PointToLocal (user/user_objects.h:864)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_site_point_to_local(self_ptr: *mut mjCSite) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCSite)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCSite::NameSpace (user/user_objects.h:865)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_site_name_space(self_ptr: *mut mjCSite, m: *const mjCModel) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCSite, m : * const mjCModel)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCCamera::get_targetbody (user/user_objects.h:899)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_camera_get_targetbody(self_ptr: *mut mjCCamera) -> *const i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCCamera)
    // Previous return: * const i32
    todo!("re-translate: params renamed")
}

/// C: mjCCamera::get_userdata (user/user_objects.h:900)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_camera_get_userdata(self_ptr: *mut mjCCamera) -> *const i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCCamera)
    // Previous return: * const i32
    todo!("re-translate: params renamed")
}

/// C: mjCCamera::SetParent (user/user_objects.h:902)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_camera_set_parent(self_ptr: *mut mjCCamera, _body: *mut mjCBody) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCCamera, _body : * mut mjCBody)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCCamera::GetParent (user/user_objects.h:903)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_camera_get_parent(self_ptr: *mut mjCCamera) -> *mut mjCBody {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCCamera)
    // Previous return: * mut mjCBody
    todo!("re-translate: params renamed")
}

/// C: mjCCamera::Compile (user/user_objects.h:906)
/// Calls: mjCCamera::CopyFromSpec
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_camera_compile(self_ptr: *mut mjCCamera) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCCamera)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCCamera::CopyFromSpec (user/user_objects.h:907)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_camera_copy_from_spec(self_ptr: *mut mjCCamera) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCCamera)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCCamera::PointToLocal (user/user_objects.h:908)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_camera_point_to_local(self_ptr: *mut mjCCamera) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCCamera)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCCamera::NameSpace (user/user_objects.h:909)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_camera_name_space(self_ptr: *mut mjCCamera, m: *const mjCModel) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCCamera, m : * const mjCModel)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCCamera::ResolveReferences (user/user_objects.h:910)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_camera_resolve_references(self_ptr: *mut mjCCamera, m: *const mjCModel) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCCamera, m : * const mjCModel)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCLight::get_targetbody (user/user_objects.h:944)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_light_get_targetbody(self_ptr: *mut mjCLight) -> *const i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCLight)
    // Previous return: * const i32
    todo!("re-translate: params renamed")
}

/// C: mjCLight::get_texture (user/user_objects.h:945)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_light_get_texture(self_ptr: *mut mjCLight) -> *const i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCLight)
    // Previous return: * const i32
    todo!("re-translate: params renamed")
}

/// C: mjCLight::SetParent (user/user_objects.h:947)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_light_set_parent(self_ptr: *mut mjCLight, _body: *mut mjCBody) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCLight, _body : * mut mjCBody)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCLight::GetParent (user/user_objects.h:948)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_light_get_parent(self_ptr: *mut mjCLight) -> *mut mjCBody {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCLight)
    // Previous return: * mut mjCBody
    todo!("re-translate: params renamed")
}

/// C: mjCLight::Compile (user/user_objects.h:951)
/// Calls: mjCLight::CopyFromSpec
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_light_compile(self_ptr: *mut mjCLight) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCLight)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCLight::CopyFromSpec (user/user_objects.h:952)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_light_copy_from_spec(self_ptr: *mut mjCLight) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCLight)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCLight::PointToLocal (user/user_objects.h:953)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_light_point_to_local(self_ptr: *mut mjCLight) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCLight)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCLight::NameSpace (user/user_objects.h:954)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_light_name_space(self_ptr: *mut mjCLight, m: *const mjCModel) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCLight, m : * const mjCModel)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCLight::ResolveReferences (user/user_objects.h:955)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_light_resolve_references(self_ptr: *mut mjCLight, m: *const mjCModel) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCLight, m : * const mjCModel)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCFlex::CopyFromSpec (user/user_objects.h:1032)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_copy_from_spec(self_ptr: *mut mjCFlex) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCFlex)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCFlex::PointToLocal (user/user_objects.h:1033)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_point_to_local(self_ptr: *mut mjCFlex) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCFlex)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCFlex::ResolveReferences (user/user_objects.h:1034)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_resolve_references(self_ptr: *mut mjCFlex, m: *const mjCModel) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCFlex, m : * const mjCModel)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCFlex::NameSpace (user/user_objects.h:1035)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_name_space(self_ptr: *mut mjCFlex, m: *const mjCModel) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCFlex, m : * const mjCModel)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCFlex::get_material (user/user_objects.h:1038)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_get_material(self_ptr: *mut mjCFlex) -> *const i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCFlex)
    // Previous return: * const i32
    todo!("re-translate: params renamed")
}

/// C: mjCFlex::get_vertbody (user/user_objects.h:1039)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_get_vertbody(self_ptr: *mut mjCFlex) -> *const i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCFlex)
    // Previous return: * const i32
    todo!("re-translate: params renamed")
}

/// C: mjCFlex::get_vert (user/user_objects.h:1040)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_get_vert(self_ptr: *mut mjCFlex) -> *const i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCFlex)
    // Previous return: * const i32
    todo!("re-translate: params renamed")
}

/// C: mjCFlex::get_elemaabb (user/user_objects.h:1041)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_get_elemaabb(self_ptr: *mut mjCFlex) -> *const i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCFlex)
    // Previous return: * const i32
    todo!("re-translate: params renamed")
}

/// C: mjCFlex::get_elem (user/user_objects.h:1042)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_get_elem(self_ptr: *mut mjCFlex) -> *const i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCFlex)
    // Previous return: * const i32
    todo!("re-translate: params renamed")
}

/// C: mjCFlex::get_texcoord (user/user_objects.h:1043)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_get_texcoord(self_ptr: *mut mjCFlex) -> *const i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCFlex)
    // Previous return: * const i32
    todo!("re-translate: params renamed")
}

/// C: mjCFlex::get_elemtexcoord (user/user_objects.h:1044)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_get_elemtexcoord(self_ptr: *mut mjCFlex) -> *const i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCFlex)
    // Previous return: * const i32
    todo!("re-translate: params renamed")
}

/// C: mjCFlex::get_nodebody (user/user_objects.h:1045)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_get_nodebody(self_ptr: *mut mjCFlex) -> *const i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCFlex)
    // Previous return: * const i32
    todo!("re-translate: params renamed")
}

/// C: mjCFlex::HasTexcoord (user/user_objects.h:1047)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_has_texcoord(self_ptr: *mut mjCFlex) -> bool {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCFlex)
    // Previous return: bool
    todo!("re-translate: params renamed")
}

/// C: mjCFlex::DelTexcoord (user/user_objects.h:1048)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_del_texcoord(self_ptr: *mut mjCFlex) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCFlex)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCFlex::Compile (user/user_objects.h:1055)
/// Calls: mjCFlex::CopyFromSpec, mjCFlex::CreateBVH, mjCFlex::CreateShellPair, mjCFlex::LoadCachedStiffness, mjuu_crossvec, mjuu_dot3
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_compile(self_ptr: *mut mjCFlex, vfs: *const mjVFS) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCFlex, vfs : * const mjVFS)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCFlex::CreateBVH (user/user_objects.h:1056)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_create_bvh(self_ptr: *mut mjCFlex) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCFlex)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCFlex::CreateShellPair (user/user_objects.h:1057)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_create_shell_pair(self_ptr: *mut mjCFlex) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCFlex)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCFlex::ComputeCellEmpty (user/user_objects.h:1058)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_compute_cell_empty(self_ptr: *mut mjCFlex, vpos: *const f64, elems: *const i32, nv: i32, ne: i32, fdim: i32, bbox: *const f64) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCFlex, vpos : * const f64, elems : * const i32, nv : i32, ne : i32, fdim : i32, bbox : * const f64)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCFlex::ComputeStiffnessCacheKey (user/user_objects.h:1071)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_compute_stiffness_cache_key(self_ptr: *mut mjCFlex) -> std__string {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCFlex)
    // Previous return: std__string
    todo!("re-translate: params renamed")
}

/// C: mjCFlex::LoadCachedStiffness (user/user_objects.h:1072)
/// Calls: mjCCache::PopulateData, mj_getCache
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_load_cached_stiffness(self_ptr: *mut mjCFlex) -> bool {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCFlex)
    // Previous return: bool
    todo!("re-translate: params renamed")
}

/// C: mjCFlex::CacheStiffness (user/user_objects.h:1073)
/// Calls: mjCCache::Insert, mj_getCache
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_cache_stiffness(self_ptr: *mut mjCFlex) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCFlex)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCMesh::CopyFromSpec (user/user_objects.h:1151)
/// Calls: mju_free
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_copy_from_spec(self_ptr: *mut mjCMesh) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCMesh)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCMesh::PointToLocal (user/user_objects.h:1152)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_point_to_local(self_ptr: *mut mjCMesh) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCMesh)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCMesh::NameSpace (user/user_objects.h:1153)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_name_space(self_ptr: *mut mjCMesh, m: *const mjCModel) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCMesh, m : * const mjCModel)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCMesh::MakeHemisphere (user/user_objects.h:1156)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_make_hemisphere(self_ptr: *mut mjCMesh, res: i32, make_faces: bool, make_cap: bool) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCMesh, res : i32, make_faces : bool, make_cap : bool)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCMesh::MakeSphere (user/user_objects.h:1157)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_make_sphere(self_ptr: *mut mjCMesh, subdiv: i32, make_faces: bool) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCMesh, subdiv : i32, make_faces : bool)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCMesh::MakeTorus (user/user_objects.h:1158)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_make_torus(self_ptr: *mut mjCMesh, res: i32, radius: f64) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCMesh, res : i32, radius : f64)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCMesh::MakeSupertorus (user/user_objects.h:1159)
/// Calls: aux_c, aux_s
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_make_supertorus(self_ptr: *mut mjCMesh, res: i32, radius: f64, s: f64, t: f64) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCMesh, res : i32, radius : f64, s : f64, t : f64)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCMesh::MakeSupersphere (user/user_objects.h:1160)
/// Calls: aux_c, aux_s
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_make_supersphere(self_ptr: *mut mjCMesh, res: i32, e: f64, n: f64) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCMesh, res : i32, e : f64, n : f64)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCMesh::MakeWedge (user/user_objects.h:1161)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_make_wedge(self_ptr: *mut mjCMesh, resolution: *mut i32, fov: *mut f64, gamma: f64) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCMesh, resolution : * mut i32, fov : * mut f64, gamma : f64)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCMesh::MakeRect (user/user_objects.h:1162)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_make_rect(self_ptr: *mut mjCMesh, resolution: *mut i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCMesh, resolution : * mut i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCMesh::MakeCone (user/user_objects.h:1163)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_make_cone(self_ptr: *mut mjCMesh, nedge: i32, radius: f64) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCMesh, nedge : i32, radius : f64)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCMesh::Plugin (user/user_objects.h:1166)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_plugin(self_ptr: *mut mjCMesh) -> *const mjsPlugin {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCMesh)
    // Previous return: * const mjsPlugin
    todo!("re-translate: params renamed")
}

/// C: mjCMesh::ContentType (user/user_objects.h:1167)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_content_type(self_ptr: *mut mjCMesh) -> *const i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCMesh)
    // Previous return: * const i32
    todo!("re-translate: params renamed")
}

/// C: mjCMesh::File (user/user_objects.h:1168)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_file(self_ptr: *mut mjCMesh) -> *const i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCMesh)
    // Previous return: * const i32
    todo!("re-translate: params renamed")
}

/// C: mjCMesh::Refpos (user/user_objects.h:1169)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_refpos(self_ptr: *mut mjCMesh) -> *const f64 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCMesh)
    // Previous return: * const f64
    todo!("re-translate: params renamed")
}

/// C: mjCMesh::Refquat (user/user_objects.h:1170)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_refquat(self_ptr: *mut mjCMesh) -> *const f64 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCMesh)
    // Previous return: * const f64
    todo!("re-translate: params renamed")
}

/// C: mjCMesh::Scale (user/user_objects.h:1171)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_scale(self_ptr: *mut mjCMesh) -> *const f64 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCMesh)
    // Previous return: * const f64
    todo!("re-translate: params renamed")
}

/// C: mjCMesh::SmoothNormal (user/user_objects.h:1172)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_smooth_normal(self_ptr: *mut mjCMesh) -> bool {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCMesh)
    // Previous return: bool
    todo!("re-translate: params renamed")
}

/// C: mjCMesh::Vert (user/user_objects.h:1173)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_vert(self_ptr: *mut mjCMesh) -> *const i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCMesh)
    // Previous return: * const i32
    todo!("re-translate: params renamed")
}

/// C: mjCMesh::UserVert (user/user_objects.h:1175)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_user_vert(self_ptr: *mut mjCMesh) -> *const i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCMesh)
    // Previous return: * const i32
    todo!("re-translate: params renamed")
}

/// C: mjCMesh::UserNormal (user/user_objects.h:1176)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_user_normal(self_ptr: *mut mjCMesh) -> *const i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCMesh)
    // Previous return: * const i32
    todo!("re-translate: params renamed")
}

/// C: mjCMesh::Texcoord (user/user_objects.h:1177)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_texcoord(self_ptr: *mut mjCMesh) -> *const i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCMesh)
    // Previous return: * const i32
    todo!("re-translate: params renamed")
}

/// C: mjCMesh::FaceTexcoord (user/user_objects.h:1178)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_face_texcoord(self_ptr: *mut mjCMesh) -> *const i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCMesh)
    // Previous return: * const i32
    todo!("re-translate: params renamed")
}

/// C: mjCMesh::UserTexcoord (user/user_objects.h:1179)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_user_texcoord(self_ptr: *mut mjCMesh) -> *const i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCMesh)
    // Previous return: * const i32
    todo!("re-translate: params renamed")
}

/// C: mjCMesh::Face (user/user_objects.h:1180)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_face(self_ptr: *mut mjCMesh) -> *const i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCMesh)
    // Previous return: * const i32
    todo!("re-translate: params renamed")
}

/// C: mjCMesh::UserFace (user/user_objects.h:1181)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_user_face(self_ptr: *mut mjCMesh) -> *const i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCMesh)
    // Previous return: * const i32
    todo!("re-translate: params renamed")
}

/// C: mjCMesh::Inertia (user/user_objects.h:1182)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_inertia(self_ptr: *mut mjCMesh) -> u32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCMesh)
    // Previous return: u32
    todo!("re-translate: params renamed")
}

/// C: mjCMesh::Material (user/user_objects.h:1183)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_material(self_ptr: *mut mjCMesh) -> *const i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCMesh)
    // Previous return: * const i32
    todo!("re-translate: params renamed")
}

/// C: mjCMesh::SetNeedHull (user/user_objects.h:1186)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_set_need_hull(self_ptr: *mut mjCMesh, needhull: bool) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCMesh, needhull : bool)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCMesh::aamm (user/user_objects.h:1189)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_aamm(self_ptr: *mut mjCMesh) -> *const f64 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCMesh)
    // Previous return: * const f64
    todo!("re-translate: params renamed")
}

/// C: mjCMesh::nvert (user/user_objects.h:1192)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_nvert(self_ptr: *mut mjCMesh) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCMesh)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: mjCMesh::nnormal (user/user_objects.h:1193)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_nnormal(self_ptr: *mut mjCMesh) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCMesh)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: mjCMesh::ntexcoord (user/user_objects.h:1194)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_ntexcoord(self_ptr: *mut mjCMesh) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCMesh)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: mjCMesh::nface (user/user_objects.h:1195)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_nface(self_ptr: *mut mjCMesh) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCMesh)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: mjCMesh::npolygon (user/user_objects.h:1196)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_npolygon(self_ptr: *mut mjCMesh) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCMesh)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: mjCMesh::npolygonvert (user/user_objects.h:1197)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_npolygonvert(self_ptr: *mut mjCMesh) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCMesh)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: mjCMesh::npolygonmap (user/user_objects.h:1204)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_npolygonmap(self_ptr: *mut mjCMesh) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCMesh)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: mjCMesh::szgraph (user/user_objects.h:1213)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_szgraph(self_ptr: *mut mjCMesh) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCMesh)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: mjCMesh::tree (user/user_objects.h:1216)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_tree(self_ptr: *mut mjCMesh) -> *const mjCBoundingVolumeHierarchy {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCMesh)
    // Previous return: * const mjCBoundingVolumeHierarchy
    todo!("re-translate: params renamed")
}

/// C: mjCMesh::octree (user/user_objects.h:1219)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_octree(self_ptr: *mut mjCMesh) -> *const mjCOctree {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCMesh)
    // Previous return: * const mjCOctree
    todo!("re-translate: params renamed")
}

/// C: mjCMesh::mutable_octree (user/user_objects.h:1220)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_mutable_octree(self_ptr: *mut mjCMesh) -> *mut mjCOctree {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCMesh)
    // Previous return: * mut mjCOctree
    todo!("re-translate: params renamed")
}

/// C: mjCMesh::Compile (user/user_objects.h:1222)
/// Calls: mjCMesh::TryCompile
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_compile(self_ptr: *mut mjCMesh, vfs: *const mjVFS) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCMesh, vfs : * const mjVFS)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCMesh::GetPosPtr (user/user_objects.h:1223)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_get_pos_ptr(self_ptr: *mut mjCMesh) -> *mut f64 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCMesh)
    // Previous return: * mut f64
    todo!("re-translate: params renamed")
}

/// C: mjCMesh::GetQuatPtr (user/user_objects.h:1224)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_get_quat_ptr(self_ptr: *mut mjCMesh) -> *mut f64 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCMesh)
    // Previous return: * mut f64
    todo!("re-translate: params renamed")
}

/// C: mjCMesh::GetInertiaBoxPtr (user/user_objects.h:1225)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_get_inertia_box_ptr(self_ptr: *mut mjCMesh) -> *mut f64 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCMesh)
    // Previous return: * mut f64
    todo!("re-translate: params renamed")
}

/// C: mjCMesh::GetVolumeRef (user/user_objects.h:1226)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_get_volume_ref(self_ptr: *mut mjCMesh) -> f64 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCMesh)
    // Previous return: f64
    todo!("re-translate: params renamed")
}

/// C: mjCMesh::FitGeom (user/user_objects.h:1227)
/// Calls: mjCMesh::GetInertiaBoxPtr
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_fit_geom(self_ptr: *mut mjCMesh, geom: *mut mjCGeom, center: *mut f64) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCMesh, geom : * mut mjCGeom, center : * mut f64)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCMesh::HasTexcoord (user/user_objects.h:1228)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_has_texcoord(self_ptr: *mut mjCMesh) -> bool {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCMesh)
    // Previous return: bool
    todo!("re-translate: params renamed")
}

/// C: mjCMesh::DelTexcoord (user/user_objects.h:1229)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_del_texcoord(self_ptr: *mut mjCMesh) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCMesh)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCMesh::IsVisual (user/user_objects.h:1230)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_is_visual(self_ptr: *mut mjCMesh) -> bool {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCMesh)
    // Previous return: bool
    todo!("re-translate: params renamed")
}

/// C: mjCMesh::SetNotVisual (user/user_objects.h:1231)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_set_not_visual(self_ptr: *mut mjCMesh) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCMesh)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCMesh::CopyVert (user/user_objects.h:1233)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_copy_vert(self_ptr: *mut mjCMesh, arr: *mut f32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCMesh, arr : * mut f32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCMesh::CopyNormal (user/user_objects.h:1234)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_copy_normal(self_ptr: *mut mjCMesh, arr: *mut f32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCMesh, arr : * mut f32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCMesh::CopyFace (user/user_objects.h:1235)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_copy_face(self_ptr: *mut mjCMesh, arr: *mut i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCMesh, arr : * mut i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCMesh::CopyFaceNormal (user/user_objects.h:1236)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_copy_face_normal(self_ptr: *mut mjCMesh, arr: *mut i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCMesh, arr : * mut i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCMesh::CopyFaceTexcoord (user/user_objects.h:1237)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_copy_face_texcoord(self_ptr: *mut mjCMesh, arr: *mut i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCMesh, arr : * mut i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCMesh::CopyTexcoord (user/user_objects.h:1238)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_copy_texcoord(self_ptr: *mut mjCMesh, arr: *mut f32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCMesh, arr : * mut f32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCMesh::CopyGraph (user/user_objects.h:1239)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_copy_graph(self_ptr: *mut mjCMesh, arr: *mut i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCMesh, arr : * mut i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCMesh::CopyPolygons (user/user_objects.h:1242)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_copy_polygons(self_ptr: *mut mjCMesh, verts: *mut i32, adr: *mut i32, num: *mut i32, poly_adr: i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCMesh, verts : * mut i32, adr : * mut i32, num : * mut i32, poly_adr : i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCMesh::CopyPolygonMap (user/user_objects.h:1245)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_copy_polygon_map(self_ptr: *mut mjCMesh, faces: *mut i32, adr: *mut i32, num: *mut i32, poly_adr: i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCMesh, faces : * mut i32, adr : * mut i32, num : * mut i32, poly_adr : i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCMesh::CopyPolygonNormals (user/user_objects.h:1248)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_copy_polygon_normals(self_ptr: *mut mjCMesh, arr: *mut f64) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCMesh, arr : * mut f64)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCMesh::SetBoundingVolume (user/user_objects.h:1251)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_set_bounding_volume(self_ptr: *mut mjCMesh, faceid: i32, dvert: *const f64) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCMesh, faceid : i32, dvert : * const f64)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCMesh::LoadFromResource (user/user_objects.h:1254)
/// Calls: mjCMesh::IsMSH, mjCMesh::LoadFromDecoder, mjCMesh::LoadMSH
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_load_from_resource(self_ptr: *mut mjCMesh, resource: *mut mjResource, remove_repeated: bool) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCMesh, resource : * mut mjResource, remove_repeated : bool)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCMesh::IsMSH (user/user_objects.h:1258)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_is_msh(filename: string_view, ct: string_view) -> bool {
    // NOTE: signature changed from previous IR version
    // Previous params: (filename : string_view, ct : string_view)
    // Previous return: bool
    todo!("re-translate: params renamed")
}

/// C: mjCMesh::TryCompile (user/user_objects.h:1265)
/// Calls: mjCMesh::CheckInitialMesh, mjCMesh::CopyFromSpec, mjCMesh::LoadSDF, mjCMesh::Process, mj_getCache
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_try_compile(self_ptr: *mut mjCMesh, vfs: *const mjVFS) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCMesh, vfs : * const mjVFS)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCMesh::LoadCachedMesh (user/user_objects.h:1268)
/// Calls: mju_malloc
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_load_cached_mesh(self_ptr: *mut mjCMesh, cache: *mut mjCCache, resource: *const mjResource) -> bool {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCMesh, cache : * mut mjCCache, resource : * const mjResource)
    // Previous return: bool
    todo!("re-translate: params renamed")
}

/// C: mjCMesh::CacheMesh (user/user_objects.h:1271)
/// Calls: mjCCache::Insert, mju_malloc
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_cache_mesh(self_ptr: *mut mjCMesh, cache: *mut mjCCache, resource: *const mjResource) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCMesh, cache : * mut mjCCache, resource : * const mjResource)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCMesh::LoadFromDecoder (user/user_objects.h:1277)
/// Calls: mj_deleteSpec, mjs_asMesh, mjs_firstElement
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_load_from_decoder(self_ptr: *mut mjCMesh, resource: *mut mjResource, remove_repeated: bool) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCMesh, resource : * mut mjResource, remove_repeated : bool)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCMesh::LoadMSH (user/user_objects.h:1279)
/// Calls: mju_readResource
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_load_msh(self_ptr: *mut mjCMesh, resource: *mut mjResource, remove_repeated: bool) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCMesh, resource : * mut mjResource, remove_repeated : bool)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCMesh::LoadSDF (user/user_objects.h:1281)
/// Calls: mjp_getPluginAtSlot
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_load_sdf(self_ptr: *mut mjCMesh) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCMesh)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCMesh::MakeGraph (user/user_objects.h:1282)
/// Calls: mjCMesh::nvert, mju_error, mju_free, mju_warning, mjuu_crossvec, mjuu_dist3, mjuu_dot3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_make_graph(self_ptr: *mut mjCMesh, dvert: *const f64) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCMesh, dvert : * const f64)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCMesh::MakeNormal (user/user_objects.h:1284)
/// Calls: mjCMesh::nface, mjCMesh::nnormal, mju_free, mju_malloc, mjuu_crossvec, mjuu_dot3, mjuu_normvec
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_make_normal(self_ptr: *mut mjCMesh, dvert: *const f64) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCMesh, dvert : * const f64)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCMesh::MakeCenter (user/user_objects.h:1285)
/// Calls: mjCMesh::nface, mju_malloc, mjuu_crossvec, mjuu_dot3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_make_center(self_ptr: *mut mjCMesh, dvert: *const f64) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCMesh, dvert : * const f64)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCMesh::Process (user/user_objects.h:1286)
/// Calls: mjCMesh::ApplyTransformations, mjCMesh::ComputeFaceCentroid, mjCMesh::ComputeInertia, mjCMesh::CopyGraph, mjCMesh::GetVolumeRef, mjCMesh::MakeCenter, mjCMesh::MakeGraph, mjCMesh::MakeNormal, mjCMesh::MakePolygonNormals, mjCMesh::MakePolygons, mjCMesh::Rotate, mjCMesh::SetBoundingVolume, mjCMesh::nface, mjCMesh::nvert, mjuu_eig3, mjuu_setvec
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_process(self_ptr: *mut mjCMesh) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCMesh)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCMesh::ApplyTransformations (user/user_objects.h:1287)
/// Calls: mjCMesh::nnormal, mjCMesh::nvert, mjuu_mulvecmatT, mjuu_normvec, mjuu_quat2mat
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_apply_transformations(self_ptr: *mut mjCMesh, dvert: *mut f64) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCMesh, dvert : * mut f64)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCMesh::ComputeFaceCentroid (user/user_objects.h:1288)
/// Calls: mjCMesh::nface
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_compute_face_centroid(self_ptr: *mut mjCMesh, arg0: *mut f64, dvert: *const f64) -> f64 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCMesh, arg0 : * mut f64, dvert : * const f64)
    // Previous return: f64
    todo!("re-translate: params renamed")
}

/// C: mjCMesh::CheckInitialMesh (user/user_objects.h:1289)
/// Calls: mjCMesh::nvert
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_check_initial_mesh(self_ptr: *mut mjCMesh) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCMesh)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCMesh::CopyPlugin (user/user_objects.h:1290)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_copy_plugin(self_ptr: *mut mjCMesh) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCMesh)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCMesh::Rotate (user/user_objects.h:1291)
/// Calls: mjCMesh::nnormal, mjCMesh::nvert, mjuu_mulvecmat, mjuu_quat2mat
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_rotate(self_ptr: *mut mjCMesh, quat: *mut f64, dvert: *mut f64) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCMesh, quat : * mut f64, dvert : * mut f64)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCMesh::MakePolygons (user/user_objects.h:1293)
/// Calls: mjCMesh::GraphFaces
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_make_polygons(self_ptr: *mut mjCMesh, dvert: *const f64) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCMesh, dvert : * const f64)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCMesh::MakePolygonNormals (user/user_objects.h:1294)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_make_polygon_normals(self_ptr: *mut mjCMesh, dvert: *const f64) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCMesh, dvert : * const f64)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCMesh::ComputeInertia (user/user_objects.h:1297)
/// Calls: mjCMesh::nvert, mjuu_dot3, triangle
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_compute_inertia(self_ptr: *mut mjCMesh, inert: *mut f64, CoM: *const f64, dvert: *const f64) -> f64 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCMesh, inert : * mut f64, CoM : * const f64, dvert : * const f64)
    // Previous return: f64
    todo!("re-translate: params renamed")
}

/// C: mjCMesh::GraphFaces (user/user_objects.h:1299)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_graph_faces(self_ptr: *mut mjCMesh) -> *mut i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCMesh)
    // Previous return: * mut i32
    todo!("re-translate: params renamed")
}

/// C: mjCMesh::ComputeVolume (user/user_objects.h:1313)
/// Calls: mjuu_dot3, triangle
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_compute_volume(self_ptr: *mut mjCMesh, CoM: *mut f64, facecen: *const f64, dvert: *const f64) -> f64 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCMesh, CoM : * mut f64, facecen : * const f64, dvert : * const f64)
    // Previous return: f64
    todo!("re-translate: params renamed")
}

/// C: mjCMesh::ComputeSurfaceArea (user/user_objects.h:1314)
/// Calls: mjCMesh::nface
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_compute_surface_area(self_ptr: *mut mjCMesh, CoM: *mut f64, facecen: *const f64, dvert: *const f64) -> f64 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCMesh, CoM : * mut f64, facecen : * const f64, dvert : * const f64)
    // Previous return: f64
    todo!("re-translate: params renamed")
}

/// C: mjCSkin::File (user/user_objects.h:1364)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_skin_file(self_ptr: *mut mjCSkin) -> *const i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCSkin)
    // Previous return: * const i32
    todo!("re-translate: params renamed")
}

/// C: mjCSkin::get_material (user/user_objects.h:1365)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_skin_get_material(self_ptr: *mut mjCSkin) -> *const i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCSkin)
    // Previous return: * const i32
    todo!("re-translate: params renamed")
}

/// C: mjCSkin::get_vert (user/user_objects.h:1366)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_skin_get_vert(self_ptr: *mut mjCSkin) -> *const i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCSkin)
    // Previous return: * const i32
    todo!("re-translate: params renamed")
}

/// C: mjCSkin::get_texcoord (user/user_objects.h:1367)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_skin_get_texcoord(self_ptr: *mut mjCSkin) -> *const i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCSkin)
    // Previous return: * const i32
    todo!("re-translate: params renamed")
}

/// C: mjCSkin::get_face (user/user_objects.h:1368)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_skin_get_face(self_ptr: *mut mjCSkin) -> *const i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCSkin)
    // Previous return: * const i32
    todo!("re-translate: params renamed")
}

/// C: mjCSkin::get_bodyname (user/user_objects.h:1369)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_skin_get_bodyname(self_ptr: *mut mjCSkin) -> *const i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCSkin)
    // Previous return: * const i32
    todo!("re-translate: params renamed")
}

/// C: mjCSkin::get_bindpos (user/user_objects.h:1370)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_skin_get_bindpos(self_ptr: *mut mjCSkin) -> *const i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCSkin)
    // Previous return: * const i32
    todo!("re-translate: params renamed")
}

/// C: mjCSkin::get_bindquat (user/user_objects.h:1371)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_skin_get_bindquat(self_ptr: *mut mjCSkin) -> *const i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCSkin)
    // Previous return: * const i32
    todo!("re-translate: params renamed")
}

/// C: mjCSkin::get_vertid (user/user_objects.h:1372)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_skin_get_vertid(self_ptr: *mut mjCSkin) -> *const i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCSkin)
    // Previous return: * const i32
    todo!("re-translate: params renamed")
}

/// C: mjCSkin::get_vertweight (user/user_objects.h:1373)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_skin_get_vertweight(self_ptr: *mut mjCSkin) -> *const i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCSkin)
    // Previous return: * const i32
    todo!("re-translate: params renamed")
}

/// C: mjCSkin::del_material (user/user_objects.h:1374)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_skin_del_material(self_ptr: *mut mjCSkin) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCSkin)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCSkin::CopyFromSpec (user/user_objects.h:1376)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_skin_copy_from_spec(self_ptr: *mut mjCSkin) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCSkin)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCSkin::PointToLocal (user/user_objects.h:1377)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_skin_point_to_local(self_ptr: *mut mjCSkin) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCSkin)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCSkin::ResolveReferences (user/user_objects.h:1380)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_skin_resolve_references(self_ptr: *mut mjCSkin, m: *const mjCModel) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCSkin, m : * const mjCModel)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCSkin::NameSpace (user/user_objects.h:1381)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_skin_name_space(self_ptr: *mut mjCSkin, m: *const mjCModel) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCSkin, m : * const mjCModel)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCSkin::Compile (user/user_objects.h:1382)
/// Calls: FilePath::Str, mjCBase::LoadResource, mjCSkin::CopyFromSpec, mjCSkin::LoadSKN, mju_closeResource, mjuu_normvec
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_skin_compile(self_ptr: *mut mjCSkin, vfs: *const mjVFS) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCSkin, vfs : * const mjVFS)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCSkin::LoadSKN (user/user_objects.h:1383)
/// Calls: mju_readResource
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_skin_load_skn(self_ptr: *mut mjCSkin, resource: *mut mjResource) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCSkin, resource : * mut mjResource)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCHField::CopyFromSpec (user/user_objects.h:1417)
#[allow(unused_variables, non_snake_case)]
pub fn mj_ch_field_copy_from_spec(self_ptr: *mut mjCHField) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCHField)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCHField::PointToLocal (user/user_objects.h:1418)
#[allow(unused_variables, non_snake_case)]
pub fn mj_ch_field_point_to_local(self_ptr: *mut mjCHField) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCHField)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCHField::NameSpace (user/user_objects.h:1419)
#[allow(unused_variables, non_snake_case)]
pub fn mj_ch_field_name_space(self_ptr: *mut mjCHField, m: *const mjCModel) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCHField, m : * const mjCModel)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCHField::File (user/user_objects.h:1421)
#[allow(unused_variables, non_snake_case)]
pub fn mj_ch_field_file(self_ptr: *mut mjCHField) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCHField)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: mjCHField::get_userdata (user/user_objects.h:1424)
#[allow(unused_variables, non_snake_case)]
pub fn mj_ch_field_get_userdata(self_ptr: *mut mjCHField) -> *mut i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCHField)
    // Previous return: * mut i32
    todo!("re-translate: params renamed")
}

/// C: mjCHField::Compile (user/user_objects.h:1427)
/// Calls: FilePath::Str, mjCBase::LoadResource, mjCHField::CopyFromSpec, mjCHField::LoadCustom, mjCHField::LoadPNG, mj_getCache, mju_closeResource
#[allow(unused_variables, non_snake_case)]
pub fn mj_ch_field_compile(self_ptr: *mut mjCHField, vfs: *const mjVFS) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCHField, vfs : * const mjVFS)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCHField::GetCacheId (user/user_objects.h:1429)
#[allow(unused_variables, non_snake_case)]
pub fn mj_ch_field_get_cache_id(self_ptr: *mut mjCHField, resource: *const mjResource, asset_type: *const std__string) -> std__string {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCHField, resource : * const mjResource, asset_type : * const std__string)
    // Previous return: std__string
    todo!("re-translate: params renamed")
}

/// C: mjCHField::LoadCustom (user/user_objects.h:1430)
/// Calls: mju_readResource
#[allow(unused_variables, non_snake_case)]
pub fn mj_ch_field_load_custom(self_ptr: *mut mjCHField, resource: *mut mjResource) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCHField, resource : * mut mjResource)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCHField::LoadPNG (user/user_objects.h:1431)
#[allow(unused_variables, non_snake_case)]
pub fn mj_ch_field_load_png(self_ptr: *mut mjCHField, resource: *mut mjResource) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCHField, resource : * mut mjResource)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCTexture::CopyFromSpec (user/user_objects.h:1465)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_texture_copy_from_spec(self_ptr: *mut mjCTexture) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCTexture)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCTexture::PointToLocal (user/user_objects.h:1466)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_texture_point_to_local(self_ptr: *mut mjCTexture) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCTexture)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCTexture::NameSpace (user/user_objects.h:1467)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_texture_name_space(self_ptr: *mut mjCTexture, m: *const mjCModel) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCTexture, m : * const mjCModel)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCTexture::Compile (user/user_objects.h:1468)
/// Calls: FilePath::Str, mjCTexture::Builtin2D, mjCTexture::BuiltinCube, mjCTexture::CopyFromSpec, mjCTexture::LoadCubeSeparate
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_texture_compile(self_ptr: *mut mjCTexture, vfs: *const mjVFS) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCTexture, vfs : * const mjVFS)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCTexture::File (user/user_objects.h:1471)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_texture_file(self_ptr: *mut mjCTexture) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCTexture)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: mjCTexture::get_content_type (user/user_objects.h:1472)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_texture_get_content_type(self_ptr: *mut mjCTexture) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCTexture)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: mjCTexture::get_cubefiles (user/user_objects.h:1473)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_texture_get_cubefiles(self_ptr: *mut mjCTexture) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCTexture)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: mjCTexture::GetCacheId (user/user_objects.h:1477)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_texture_get_cache_id(self_ptr: *mut mjCTexture, resource: *const mjResource, asset_type: *const std__string) -> std__string {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCTexture, resource : * const mjResource, asset_type : * const std__string)
    // Previous return: std__string
    todo!("re-translate: params renamed")
}

/// C: mjCTexture::Builtin2D (user/user_objects.h:1478)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_texture_builtin2d(self_ptr: *mut mjCTexture) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCTexture)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCTexture::BuiltinCube (user/user_objects.h:1479)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_texture_builtin_cube(self_ptr: *mut mjCTexture) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCTexture)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCTexture::Load2D (user/user_objects.h:1480)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_texture_load2d(self_ptr: *mut mjCTexture, filename: string, vfs: *const mjVFS) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCTexture, filename : string, vfs : * const mjVFS)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCTexture::LoadCubeSingle (user/user_objects.h:1481)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_texture_load_cube_single(self_ptr: *mut mjCTexture, filename: string, vfs: *const mjVFS) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCTexture, filename : string, vfs : * const mjVFS)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCTexture::LoadCubeSeparate (user/user_objects.h:1483)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_texture_load_cube_separate(self_ptr: *mut mjCTexture, vfs: *const mjVFS) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCTexture, vfs : * const mjVFS)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCMaterial::CopyFromSpec (user/user_objects.h:1526)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_material_copy_from_spec(self_ptr: *mut mjCMaterial) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCMaterial)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCMaterial::PointToLocal (user/user_objects.h:1527)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_material_point_to_local(self_ptr: *mut mjCMaterial) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCMaterial)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCMaterial::NameSpace (user/user_objects.h:1528)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_material_name_space(self_ptr: *mut mjCMaterial, m: *const mjCModel) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCMaterial, m : * const mjCModel)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCMaterial::get_texture (user/user_objects.h:1530)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_material_get_texture(self_ptr: *mut mjCMaterial, i: i32) -> *const i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCMaterial, i : i32)
    // Previous return: * const i32
    todo!("re-translate: params renamed")
}

/// C: mjCMaterial::del_textures (user/user_objects.h:1531)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_material_del_textures(self_ptr: *mut mjCMaterial) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCMaterial)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCMaterial::Compile (user/user_objects.h:1534)
/// Calls: mjCMaterial::CopyFromSpec
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_material_compile(self_ptr: *mut mjCMaterial) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCMaterial)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCPair::CopyFromSpec (user/user_objects.h:1565)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_pair_copy_from_spec(self_ptr: *mut mjCPair) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCPair)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCPair::PointToLocal (user/user_objects.h:1566)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_pair_point_to_local(self_ptr: *mut mjCPair) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCPair)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCPair::ResolveReferences (user/user_objects.h:1567)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_pair_resolve_references(self_ptr: *mut mjCPair, m: *const mjCModel) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCPair, m : * const mjCModel)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCPair::NameSpace (user/user_objects.h:1568)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_pair_name_space(self_ptr: *mut mjCPair, m: *const mjCModel) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCPair, m : * const mjCModel)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCPair::get_geomname1 (user/user_objects.h:1570)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_pair_get_geomname1(self_ptr: *mut mjCPair) -> *const i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCPair)
    // Previous return: * const i32
    todo!("re-translate: params renamed")
}

/// C: mjCPair::get_geomname2 (user/user_objects.h:1571)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_pair_get_geomname2(self_ptr: *mut mjCPair) -> *const i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCPair)
    // Previous return: * const i32
    todo!("re-translate: params renamed")
}

/// C: mjCPair::GetSignature (user/user_objects.h:1573)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_pair_get_signature(self_ptr: *mut mjCPair) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCPair)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: mjCPair::Compile (user/user_objects.h:1578)
/// Calls: mjCGeom::SetNotVisual, mjCPair::CopyFromSpec
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_pair_compile(self_ptr: *mut mjCPair) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCPair)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCBodyPair::CopyFromSpec (user/user_objects.h:1613)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_pair_copy_from_spec(self_ptr: *mut mjCBodyPair) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCBodyPair)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCBodyPair::PointToLocal (user/user_objects.h:1614)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_pair_point_to_local(self_ptr: *mut mjCBodyPair) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCBodyPair)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCBodyPair::ResolveReferences (user/user_objects.h:1615)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_pair_resolve_references(self_ptr: *mut mjCBodyPair, m: *const mjCModel) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCBodyPair, m : * const mjCModel)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCBodyPair::NameSpace (user/user_objects.h:1616)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_pair_name_space(self_ptr: *mut mjCBodyPair, m: *const mjCModel) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCBodyPair, m : * const mjCModel)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCBodyPair::get_bodyname1 (user/user_objects.h:1618)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_pair_get_bodyname1(self_ptr: *mut mjCBodyPair) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCBodyPair)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: mjCBodyPair::get_bodyname2 (user/user_objects.h:1619)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_pair_get_bodyname2(self_ptr: *mut mjCBodyPair) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCBodyPair)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: mjCBodyPair::GetSignature (user/user_objects.h:1621)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_pair_get_signature(self_ptr: *mut mjCBodyPair) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCBodyPair)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: mjCBodyPair::Compile (user/user_objects.h:1626)
/// Calls: mjCBodyPair::CopyFromSpec
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_pair_compile(self_ptr: *mut mjCBodyPair) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCBodyPair)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCEquality::CopyFromSpec (user/user_objects.h:1658)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_equality_copy_from_spec(self_ptr: *mut mjCEquality) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCEquality)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCEquality::PointToLocal (user/user_objects.h:1659)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_equality_point_to_local(self_ptr: *mut mjCEquality) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCEquality)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCEquality::ResolveReferences (user/user_objects.h:1660)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_equality_resolve_references(self_ptr: *mut mjCEquality, m: *const mjCModel) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCEquality, m : * const mjCModel)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCEquality::NameSpace (user/user_objects.h:1661)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_equality_name_space(self_ptr: *mut mjCEquality, m: *const mjCModel) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCEquality, m : * const mjCModel)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCEquality::Compile (user/user_objects.h:1664)
/// Calls: mjCEquality::CopyFromSpec
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_equality_compile(self_ptr: *mut mjCEquality) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCEquality)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCTendon::set_material (user/user_objects.h:1697)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tendon_set_material(self_ptr: *mut mjCTendon, _material: i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCTendon, _material : i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCTendon::get_material (user/user_objects.h:1698)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tendon_get_material(self_ptr: *mut mjCTendon) -> *const i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCTendon)
    // Previous return: * const i32
    todo!("re-translate: params renamed")
}

/// C: mjCTendon::del_material (user/user_objects.h:1699)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tendon_del_material(self_ptr: *mut mjCTendon) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCTendon)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCTendon::WrapSite (user/user_objects.h:1702)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tendon_wrap_site(self_ptr: *mut mjCTendon, wrapname: string, wrapinfo: string_view) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCTendon, wrapname : string, wrapinfo : string_view)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCTendon::WrapGeom (user/user_objects.h:1703)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tendon_wrap_geom(self_ptr: *mut mjCTendon, wrapname: string, side: string, wrapinfo: string_view) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCTendon, wrapname : string, side : string, wrapinfo : string_view)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCTendon::WrapJoint (user/user_objects.h:1704)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tendon_wrap_joint(self_ptr: *mut mjCTendon, wrapname: string, coef: f64, wrapinfo: string_view) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCTendon, wrapname : string, coef : f64, wrapinfo : string_view)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCTendon::WrapPulley (user/user_objects.h:1705)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tendon_wrap_pulley(self_ptr: *mut mjCTendon, divisor: f64, wrapinfo: string_view) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCTendon, divisor : f64, wrapinfo : string_view)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCTendon::NumWraps (user/user_objects.h:1708)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tendon_num_wraps(self_ptr: *mut mjCTendon) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCTendon)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: mjCTendon::GetWrap (user/user_objects.h:1709)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tendon_get_wrap(self_ptr: *mut mjCTendon, i: i32) -> *const mjCWrap {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCTendon, i : i32)
    // Previous return: * const mjCWrap
    todo!("re-translate: params renamed")
}

/// C: mjCTendon::get_userdata (user/user_objects.h:1713)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tendon_get_userdata(self_ptr: *mut mjCTendon) -> *const i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCTendon)
    // Previous return: * const i32
    todo!("re-translate: params renamed")
}

/// C: mjCTendon::get_range (user/user_objects.h:1714)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tendon_get_range(self_ptr: *mut mjCTendon) -> *const f64 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCTendon)
    // Previous return: * const f64
    todo!("re-translate: params renamed")
}

/// C: mjCTendon::CopyFromSpec (user/user_objects.h:1716)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tendon_copy_from_spec(self_ptr: *mut mjCTendon) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCTendon)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCTendon::PointToLocal (user/user_objects.h:1717)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tendon_point_to_local(self_ptr: *mut mjCTendon) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCTendon)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCTendon::ResolveReferences (user/user_objects.h:1718)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tendon_resolve_references(self_ptr: *mut mjCTendon, m: *const mjCModel) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCTendon, m : * const mjCModel)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCTendon::NameSpace (user/user_objects.h:1719)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tendon_name_space(self_ptr: *mut mjCTendon, m: *const mjCModel) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCTendon, m : * const mjCModel)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCTendon::SetModel (user/user_objects.h:1720)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tendon_set_model(self_ptr: *mut mjCTendon, _model: *mut mjCModel) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCTendon, _model : * mut mjCModel)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCTendon::is_limited (user/user_objects.h:1722)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tendon_is_limited(self_ptr: *mut mjCTendon) -> bool {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCTendon)
    // Previous return: bool
    todo!("re-translate: params renamed")
}

/// C: mjCTendon::is_actfrclimited (user/user_objects.h:1723)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tendon_is_actfrclimited(self_ptr: *mut mjCTendon) -> bool {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCTendon)
    // Previous return: bool
    todo!("re-translate: params renamed")
}

/// C: mjCTendon::Compile (user/user_objects.h:1726)
/// Calls: mjCTendon::CopyFromSpec
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tendon_compile(self_ptr: *mut mjCTendon) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCTendon)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCWrap::CopyFromSpec (user/user_objects.h:1749)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_wrap_copy_from_spec(self_ptr: *mut mjCWrap) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCWrap)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCWrap::PointToLocal (user/user_objects.h:1750)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_wrap_point_to_local(self_ptr: *mut mjCWrap) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCWrap)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCWrap::ResolveReferences (user/user_objects.h:1751)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_wrap_resolve_references(self_ptr: *mut mjCWrap, m: *const mjCModel) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCWrap, m : * const mjCModel)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCWrap::NameSpace (user/user_objects.h:1752)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_wrap_name_space(self_ptr: *mut mjCWrap, m: *const mjCModel) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCWrap, m : * const mjCModel)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCWrap::Type (user/user_objects.h:1753)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_wrap_type(self_ptr: *mut mjCWrap) -> u32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCWrap)
    // Previous return: u32
    todo!("re-translate: params renamed")
}

/// C: mjCWrap::Compile (user/user_objects.h:1762)
/// Calls: mjCWrap::CopyFromSpec
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_wrap_compile(self_ptr: *mut mjCWrap) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCWrap)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCPlugin::PointToLocal (user/user_objects.h:1791)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_plugin_point_to_local(self_ptr: *mut mjCPlugin) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCPlugin)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCPlugin::Compile (user/user_objects.h:1798)
/// Calls: mjp_getPluginAtSlot
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_plugin_compile(self_ptr: *mut mjCPlugin) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCPlugin)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCActuator::get_userdata (user/user_objects.h:1843)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_actuator_get_userdata(self_ptr: *mut mjCActuator) -> *const i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCActuator)
    // Previous return: * const i32
    todo!("re-translate: params renamed")
}

/// C: mjCActuator::get_target (user/user_objects.h:1844)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_actuator_get_target(self_ptr: *mut mjCActuator) -> *const i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCActuator)
    // Previous return: * const i32
    todo!("re-translate: params renamed")
}

/// C: mjCActuator::get_slidersite (user/user_objects.h:1845)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_actuator_get_slidersite(self_ptr: *mut mjCActuator) -> *const i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCActuator)
    // Previous return: * const i32
    todo!("re-translate: params renamed")
}

/// C: mjCActuator::get_refsite (user/user_objects.h:1846)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_actuator_get_refsite(self_ptr: *mut mjCActuator) -> *const i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCActuator)
    // Previous return: * const i32
    todo!("re-translate: params renamed")
}

/// C: mjCActuator::is_ctrllimited (user/user_objects.h:1848)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_actuator_is_ctrllimited(self_ptr: *mut mjCActuator) -> bool {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCActuator)
    // Previous return: bool
    todo!("re-translate: params renamed")
}

/// C: mjCActuator::is_forcelimited (user/user_objects.h:1849)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_actuator_is_forcelimited(self_ptr: *mut mjCActuator) -> bool {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCActuator)
    // Previous return: bool
    todo!("re-translate: params renamed")
}

/// C: mjCActuator::is_actlimited (user/user_objects.h:1850)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_actuator_is_actlimited(self_ptr: *mut mjCActuator) -> bool {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCActuator)
    // Previous return: bool
    todo!("re-translate: params renamed")
}

/// C: mjCActuator::ctrl (user/user_objects.h:1853)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_actuator_ctrl(self_ptr: *mut mjCActuator, state_name: *const std__string) -> *mut f64 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCActuator, state_name : * const std__string)
    // Previous return: * mut f64
    todo!("re-translate: params renamed")
}

/// C: mjCActuator::Compile (user/user_objects.h:1856)
/// Calls: mjCActuator::CopyFromSpec, mjCJoint::get_range, mjCTendon::get_range, mjp_getPluginAtSlot
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_actuator_compile(self_ptr: *mut mjCActuator) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCActuator)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCActuator::CopyFromSpec (user/user_objects.h:1857)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_actuator_copy_from_spec(self_ptr: *mut mjCActuator) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCActuator)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCActuator::PointToLocal (user/user_objects.h:1858)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_actuator_point_to_local(self_ptr: *mut mjCActuator) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCActuator)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCActuator::ResolveReferences (user/user_objects.h:1859)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_actuator_resolve_references(self_ptr: *mut mjCActuator, m: *const mjCModel) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCActuator, m : * const mjCModel)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCActuator::NameSpace (user/user_objects.h:1860)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_actuator_name_space(self_ptr: *mut mjCActuator, m: *const mjCModel) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCActuator, m : * const mjCModel)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCActuator::CopyPlugin (user/user_objects.h:1861)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_actuator_copy_plugin(self_ptr: *mut mjCActuator) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCActuator)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCActuator::ForgetKeyframes (user/user_objects.h:1864)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_actuator_forget_keyframes(self_ptr: *mut mjCActuator) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCActuator)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCSensor::get_userdata (user/user_objects.h:1901)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_sensor_get_userdata(self_ptr: *mut mjCSensor) -> *const i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCSensor)
    // Previous return: * const i32
    todo!("re-translate: params renamed")
}

/// C: mjCSensor::get_objname (user/user_objects.h:1902)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_sensor_get_objname(self_ptr: *mut mjCSensor) -> *const i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCSensor)
    // Previous return: * const i32
    todo!("re-translate: params renamed")
}

/// C: mjCSensor::get_refname (user/user_objects.h:1903)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_sensor_get_refname(self_ptr: *mut mjCSensor) -> *const i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCSensor)
    // Previous return: * const i32
    todo!("re-translate: params renamed")
}

/// C: mjCSensor::get_obj (user/user_objects.h:1905)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_sensor_get_obj(self_ptr: *mut mjCSensor) -> *const mjCBase {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCSensor)
    // Previous return: * const mjCBase
    todo!("re-translate: params renamed")
}

/// C: mjCSensor::get_ref (user/user_objects.h:1906)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_sensor_get_ref(self_ptr: *mut mjCSensor) -> *const mjCBase {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCSensor)
    // Previous return: * const mjCBase
    todo!("re-translate: params renamed")
}

/// C: mjCSensor::Compile (user/user_objects.h:1909)
/// Calls: mjCJoint::is_limited, mjCSensor::CopyFromSpec, mjCTendon::is_limited, mjp_getPluginAtSlot
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_sensor_compile(self_ptr: *mut mjCSensor) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCSensor)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCSensor::CopyFromSpec (user/user_objects.h:1910)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_sensor_copy_from_spec(self_ptr: *mut mjCSensor) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCSensor)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCSensor::PointToLocal (user/user_objects.h:1911)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_sensor_point_to_local(self_ptr: *mut mjCSensor) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCSensor)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCSensor::ResolveReferences (user/user_objects.h:1912)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_sensor_resolve_references(self_ptr: *mut mjCSensor, m: *const mjCModel) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCSensor, m : * const mjCModel)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCSensor::NameSpace (user/user_objects.h:1913)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_sensor_name_space(self_ptr: *mut mjCSensor, m: *const mjCModel) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCSensor, m : * const mjCModel)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCSensor::CopyPlugin (user/user_objects.h:1914)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_sensor_copy_plugin(self_ptr: *mut mjCSensor) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCSensor)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCNumeric::PointToLocal (user/user_objects.h:1944)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_numeric_point_to_local(self_ptr: *mut mjCNumeric) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCNumeric)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCNumeric::CopyFromSpec (user/user_objects.h:1945)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_numeric_copy_from_spec(self_ptr: *mut mjCNumeric) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCNumeric)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCNumeric::Compile (user/user_objects.h:1948)
/// Calls: mjCNumeric::CopyFromSpec
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_numeric_compile(self_ptr: *mut mjCNumeric) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCNumeric)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCText::PointToLocal (user/user_objects.h:1975)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_text_point_to_local(self_ptr: *mut mjCText) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCText)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCText::CopyFromSpec (user/user_objects.h:1976)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_text_copy_from_spec(self_ptr: *mut mjCText) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCText)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCText::Compile (user/user_objects.h:1979)
/// Calls: mjCText::CopyFromSpec
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_text_compile(self_ptr: *mut mjCText) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCText)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCTuple::PointToLocal (user/user_objects.h:2011)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tuple_point_to_local(self_ptr: *mut mjCTuple) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCTuple)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCTuple::CopyFromSpec (user/user_objects.h:2012)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tuple_copy_from_spec(self_ptr: *mut mjCTuple) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCTuple)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCTuple::ResolveReferences (user/user_objects.h:2013)
/// Calls: mjCGeom::SetNotVisual
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tuple_resolve_references(self_ptr: *mut mjCTuple, m: *const mjCModel) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCTuple, m : * const mjCModel)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCTuple::NameSpace (user/user_objects.h:2014)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tuple_name_space(self_ptr: *mut mjCTuple, m: *const mjCModel) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCTuple, m : * const mjCModel)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCTuple::Compile (user/user_objects.h:2017)
/// Calls: mjCTuple::CopyFromSpec
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tuple_compile(self_ptr: *mut mjCTuple) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCTuple)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCKey::PointToLocal (user/user_objects.h:2054)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_key_point_to_local(self_ptr: *mut mjCKey) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCKey)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCKey::CopyFromSpec (user/user_objects.h:2055)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_key_copy_from_spec(self_ptr: *mut mjCKey) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCKey)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCKey::Compile (user/user_objects.h:2058)
/// Calls: mjCKey::CopyFromSpec
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_key_compile(self_ptr: *mut mjCKey, m: *const mjModel) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCKey, m : * const mjModel)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCDef::CopyWithoutChildren (user/user_objects.h:2076)
/// Calls: mjCDef::PointToLocal
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_def_copy_without_children(self_ptr: *mut mjCDef, other: *const mjCDef) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCDef, other : * const mjCDef)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCDef::PointToLocal (user/user_objects.h:2077)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_def_point_to_local(self_ptr: *mut mjCDef) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCDef)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCDef::CopyFromSpec (user/user_objects.h:2078)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_def_copy_from_spec(self_ptr: *mut mjCDef) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCDef)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCDef::NameSpace (user/user_objects.h:2079)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_def_name_space(self_ptr: *mut mjCDef, m: *const mjCModel) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCDef, m : * const mjCModel)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCDef::Compile (user/user_objects.h:2081)
/// Calls: mjCDef::CopyFromSpec
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_def_compile(self_ptr: *mut mjCDef, model: *const mjCModel) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCDef, model : * const mjCModel)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCDef::Joint (user/user_objects.h:2084)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_def_joint(self_ptr: *mut mjCDef) -> *mut mjCJoint {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCDef)
    // Previous return: * mut mjCJoint
    todo!("re-translate: params renamed")
}

/// C: mjCDef::Geom (user/user_objects.h:2085)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_def_geom(self_ptr: *mut mjCDef) -> *mut mjCGeom {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCDef)
    // Previous return: * mut mjCGeom
    todo!("re-translate: params renamed")
}

/// C: mjCDef::Site (user/user_objects.h:2086)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_def_site(self_ptr: *mut mjCDef) -> *mut mjCSite {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCDef)
    // Previous return: * mut mjCSite
    todo!("re-translate: params renamed")
}

/// C: mjCDef::Camera (user/user_objects.h:2087)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_def_camera(self_ptr: *mut mjCDef) -> *mut mjCCamera {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCDef)
    // Previous return: * mut mjCCamera
    todo!("re-translate: params renamed")
}

/// C: mjCDef::Light (user/user_objects.h:2088)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_def_light(self_ptr: *mut mjCDef) -> *mut mjCLight {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCDef)
    // Previous return: * mut mjCLight
    todo!("re-translate: params renamed")
}

/// C: mjCDef::Flex (user/user_objects.h:2089)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_def_flex(self_ptr: *mut mjCDef) -> *mut mjCFlex {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCDef)
    // Previous return: * mut mjCFlex
    todo!("re-translate: params renamed")
}

/// C: mjCDef::Mesh (user/user_objects.h:2090)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_def_mesh(self_ptr: *mut mjCDef) -> *mut mjCMesh {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCDef)
    // Previous return: * mut mjCMesh
    todo!("re-translate: params renamed")
}

/// C: mjCDef::Material (user/user_objects.h:2091)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_def_material(self_ptr: *mut mjCDef) -> *mut mjCMaterial {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCDef)
    // Previous return: * mut mjCMaterial
    todo!("re-translate: params renamed")
}

/// C: mjCDef::Pair (user/user_objects.h:2092)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_def_pair(self_ptr: *mut mjCDef) -> *mut mjCPair {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCDef)
    // Previous return: * mut mjCPair
    todo!("re-translate: params renamed")
}

/// C: mjCDef::Equality (user/user_objects.h:2093)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_def_equality(self_ptr: *mut mjCDef) -> *mut mjCEquality {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCDef)
    // Previous return: * mut mjCEquality
    todo!("re-translate: params renamed")
}

/// C: mjCDef::Tendon (user/user_objects.h:2094)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_def_tendon(self_ptr: *mut mjCDef) -> *mut mjCTendon {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCDef)
    // Previous return: * mut mjCTendon
    todo!("re-translate: params renamed")
}

/// C: mjCDef::Actuator (user/user_objects.h:2095)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_def_actuator(self_ptr: *mut mjCDef) -> *mut mjCActuator {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjCDef)
    // Previous return: * mut mjCActuator
    todo!("re-translate: params renamed")
}

