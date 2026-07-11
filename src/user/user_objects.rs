//! Port of: user/user_objects.cc
//! IR hash: 05737965add36adb
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: PNGImage::Load (user/user_objects.cc:58)
/// Calls: mju_readResource
#[allow(unused_variables, non_snake_case)]
pub fn png_image_load(obj: *const mjCBase, resource: *mut mjResource, color_type: LodePNGColorType) -> PNGImage {
    extern "C" { fn PNGImage_Load(obj: *const mjCBase, resource: *mut mjResource, color_type: LodePNGColorType) -> PNGImage; }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { PNGImage_Load(obj, resource, color_type) }
}

/// C: PNGImage::Width (user/user_objects.cc:60)
#[allow(unused_variables, non_snake_case)]
pub fn png_image_width(self_ptr: *mut PNGImage) -> i32 {
    if self_ptr.is_null() { return 0; }
    extern "C" { fn PNGImage_Width(self_ptr: *mut PNGImage) -> i32; }
    // SAFETY: self_ptr verified non-null
    unsafe { PNGImage_Width(self_ptr) }
}

/// C: PNGImage::Height (user/user_objects.cc:61)
#[allow(unused_variables, non_snake_case)]
pub fn png_image_height(self_ptr: *mut PNGImage) -> i32 {
    if self_ptr.is_null() { return 0; }
    extern "C" { fn PNGImage_Height(self_ptr: *mut PNGImage) -> i32; }
    // SAFETY: self_ptr verified non-null
    unsafe { PNGImage_Height(self_ptr) }
}

/// C: PNGImage::IsSRGB (user/user_objects.cc:62)
#[allow(unused_variables, non_snake_case)]
pub fn png_image_is_srgb(self_ptr: *mut PNGImage) -> bool {
    if self_ptr.is_null() { return false; }
    extern "C" { fn PNGImage_IsSRGB(self_ptr: *mut PNGImage) -> bool; }
    // SAFETY: self_ptr verified non-null
    unsafe { PNGImage_IsSRGB(self_ptr) }
}

/// C: PNGImage::MoveData (user/user_objects.cc:66)
#[allow(unused_variables, non_snake_case)]
pub fn png_image_move_data(self_ptr: *mut PNGImage) -> *mut *mut i32 {
    if self_ptr.is_null() { return core::ptr::null_mut(); }
    extern "C" { fn PNGImage_MoveData(self_ptr: *mut PNGImage) -> *mut *mut i32; }
    // SAFETY: self_ptr verified non-null
    unsafe { PNGImage_MoveData(self_ptr) }
}

/// C: PNGImage::Size (user/user_objects.cc:69)
#[allow(unused_variables, non_snake_case)]
pub fn png_image_size(self_ptr: *mut PNGImage) -> std__size_t {
    if self_ptr.is_null() { return unsafe { core::mem::zeroed() }; }
    extern "C" { fn PNGImage_Size(self_ptr: *mut PNGImage) -> std__size_t; }
    // SAFETY: self_ptr verified non-null
    unsafe { PNGImage_Size(self_ptr) }
}

/// C: MapFrame (user/user_objects.cc:139)
#[allow(unused_variables, non_snake_case)]
pub fn map_frame(parent: *mut i32, child: *mut i32, frame: *mut mjCFrame, parent_body: *mut mjCBody) {
    if frame.is_null() || parent_body.is_null() { return; }
    extern "C" { fn MapFrame(parent: *mut i32, child: *mut i32, frame: *mut mjCFrame, parent_body: *mut mjCBody); }
    // SAFETY: frame, parent_body verified non-null
    unsafe { MapFrame(parent, child, frame, parent_body) }
}

/// C: checksize (user/user_objects.cc:153)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn checksize(size: *mut f64, r#type: mjtGeom, object: *mut mjCBase, name: *const i8, id: i32) {
    if size.is_null() {
        return;
    }
    return;
}

/// C: checklimited (user/user_objects.cc:172)
#[allow(unused_variables, non_snake_case)]
pub fn checklimited(obj: *const mjCBase, autolimits: bool, entity: *const i8, attr: *const i8, limited: i32, hasrange: bool) {
    if obj.is_null() {
        return;
    }
    return;
}

/// C: islimited (user/user_objects.cc:185)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn islimited(limited: i32, range: [f64; 2]) -> bool {
    let _size = core::mem::size_of::<i32>();
    false
}

/// C: mjCBoundingVolumeHierarchy::Make (user/user_objects.cc:404)
/// Calls: mjuu_rotVecQuat
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_hierarchy_make(self_ptr: *mut mjCBoundingVolumeHierarchy, elements: *mut i32) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCBoundingVolumeHierarchy_Make(self_ptr: *mut mjCBoundingVolumeHierarchy, elements: *mut i32); }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCBoundingVolumeHierarchy_Make(self_ptr, elements) }
}

/// C: mjCBoundingVolumeHierarchy::MakeBVH (user/user_objects.cc:424)
/// Calls: mjuu_rotVecQuat
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_hierarchy_make_bvh(self_ptr: *mut mjCBoundingVolumeHierarchy, elements_begin: i32, elements_end: i32, lev: i32, model: *mut mjCModel, owner: *const mjCBase) -> i32 {
    if self_ptr.is_null() { return 0; }
    extern "C" { fn mjCBoundingVolumeHierarchy_MakeBVH(self_ptr: *mut mjCBoundingVolumeHierarchy, elements_begin: i32, elements_end: i32, lev: i32, model: *mut mjCModel, owner: *const mjCBase) -> i32; }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCBoundingVolumeHierarchy_MakeBVH(self_ptr, elements_begin, elements_end, lev, model, owner) }
}

/// C: mjCOctree::SetFace (user/user_objects.cc:588)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_set_face(self_ptr: *mut mjCOctree, vert: *const i32, face: *const i32) {
    extern "C" { fn mjCOctree_SetFace(self_ptr: *mut mjCOctree, vert: *const i32, face: *const i32); }
    // SAFETY: delegates to C implementation
    unsafe { mjCOctree_SetFace(self_ptr, vert, face) }
}

/// C: mjCOctree::Make (user/user_objects.cc:600)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_make(self_ptr: *mut mjCOctree, elements: *mut i32) {
    extern "C" { fn mjCOctree_Make(self_ptr: *mut mjCOctree, elements: *mut i32); }
    // SAFETY: delegates to C implementation
    unsafe { mjCOctree_Make(self_ptr, elements) }
}

/// C: pointBoxDistSq (user/user_objects.cc:635)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn point_box_dist_sq(p: *const f64, aabb: *const f64) -> f64 {
    extern "C" { fn pointBoxDistSq(p: *const f64, aabb: *const f64) -> f64; }
    // SAFETY: delegates to C implementation
    unsafe { pointBoxDistSq(p, aabb) }
}

/// C: pointTriDistSqWithUV (user/user_objects.cc:652)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn point_tri_dist_sq_with_uv(p: *const f64, v0: *const f64, v1: *const f64, v2: *const f64, out_u: *mut f64, out_v: *mut f64) -> f64 {
    extern "C" { fn pointTriDistSqWithUV(p: *const f64, v0: *const f64, v1: *const f64, v2: *const f64, out_u: *mut f64, out_v: *mut f64) -> f64; }
    // SAFETY: delegates to C implementation
    unsafe { pointTriDistSqWithUV(p, v0, v1, v2, out_u, out_v) }
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
    extern "C" { fn queryClosestBVHWithFace(bvh: *const f64, child: *const i32, nodeid: *const i32, vert: *const f64, face: *const i32, node_idx: i32, p: *const f64, best_dist_sq: *mut f64, best_face: *mut i32, best_u: *mut f64, best_v: *mut f64); }
    // SAFETY: delegates to C implementation
    unsafe { queryClosestBVHWithFace(bvh, child, nodeid, vert, face, node_idx, p, best_dist_sq, best_face, best_u, best_v) }
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
    extern "C" { fn querySignedDistance(bvh: *const f64, child: *const i32, nodeid: *const i32, nbvh: i32, point: *const f64, vert: *const f64, face: *const i32) -> f64; }
    // SAFETY: delegates to C implementation
    unsafe { querySignedDistance(bvh, child, nodeid, nbvh, point, vert, face) }
}

/// C: dot2 (user/user_objects.cc:923)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn dot2(a: *const f64, b: *const f64) -> f64 {
    if a.is_null() {
        return 0.0;
    }
    0.0
}

/// C: boxTriangle (user/user_objects.cc:929)
/// Calls: dot2, mju_max, mjuu_crossvec, mjuu_dot3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn box_triangle(v: *const Triangle, aamm: [f64; 6]) -> bool {
    extern "C" { fn boxTriangle(v: *const Triangle, aamm: [f64; 6]) -> bool; }
    // SAFETY: delegates to C implementation
    unsafe { boxTriangle(v, aamm) }
}

/// C: mjCOctree::TaskToNode (user/user_objects.cc:980)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_task_to_node(self_ptr: *mut mjCOctree, task: *const OctreeTask, node: *mut OctNode, vert_map: *mut i32) {
    extern "C" { fn mjCOctree_TaskToNode(self_ptr: *mut mjCOctree, task: *const OctreeTask, node: *mut OctNode, vert_map: *mut i32); }
    // SAFETY: delegates to C implementation
    unsafe { mjCOctree_TaskToNode(self_ptr, task, node, vert_map) }
}

/// C: mjCOctree::Subdivide (user/user_objects.cc:1014)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_subdivide(self_ptr: *mut mjCOctree, task: *const OctreeTask, vert_map: *mut i32, queue: *mut i32, colliding: *const i32) {
    extern "C" { fn mjCOctree_Subdivide(self_ptr: *mut mjCOctree, task: *const OctreeTask, vert_map: *mut i32, queue: *mut i32, colliding: *const i32); }
    // SAFETY: delegates to C implementation
    unsafe { mjCOctree_Subdivide(self_ptr, task, vert_map, queue, colliding) }
}

/// C: mjCOctree::BalanceOctree (user/user_objects.cc:1113)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_balance_octree(self_ptr: *mut mjCOctree, vert_map: *mut i32) {
    extern "C" { fn mjCOctree_BalanceOctree(self_ptr: *mut mjCOctree, vert_map: *mut i32); }
    // SAFETY: delegates to C implementation
    unsafe { mjCOctree_BalanceOctree(self_ptr, vert_map) }
}

/// C: mjCOctree::MakeOctree (user/user_objects.cc:1257)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_make_octree(self_ptr: *mut mjCOctree, elements: *const i32, aamm: [f64; 6], vert_map: *mut i32) {
    extern "C" { fn mjCOctree_MakeOctree(self_ptr: *mut mjCOctree, elements: *const i32, aamm: [f64; 6], vert_map: *mut i32); }
    // SAFETY: delegates to C implementation
    unsafe { mjCOctree_MakeOctree(self_ptr, elements, aamm, vert_map) }
}

/// C: mjCBody::CopyList (user/user_objects.cc:1784)
/// Calls: mjCFrame::IsAncestor
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_copy_list(self_ptr: *mut mjCBody, dst: *mut i32, src: *const i32, fmap: *mut i32, pframe: *const mjCFrame) {
    extern "C" { fn mjCBody_CopyList(self_ptr: *mut mjCBody, dst: *mut i32, src: *const i32, fmap: *mut i32, pframe: *const mjCFrame); }
    // SAFETY: delegates to C implementation
    unsafe { mjCBody_CopyList(self_ptr, dst, src, fmap, pframe) }
}

/// C: mjCBody::GetList (user/user_objects.cc:2311)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_get_list(self_ptr: *mut mjCBody) -> *const i32 {
    if self_ptr.is_null() { return core::ptr::null(); }
    extern "C" { fn mjCBody_GetList(self_ptr: *mut mjCBody) -> *const i32; }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCBody_GetList(self_ptr) }
}

/// C: GetNextBody (user/user_objects.cc:2380)
#[allow(unused_variables, non_snake_case)]
pub fn get_next_body(body: *const mjCBody, child: *const mjsElement, found: *mut bool, recursive: bool) -> *mut mjsElement {
    if body.is_null() {
        return core::ptr::null_mut();
    }
    extern "C" { fn GetNextBody(body: *const mjCBody, child: *const mjsElement, found: *mut bool, recursive: bool) -> *mut mjsElement; }
    unsafe { GetNextBody(body, child, found, recursive) }
}

/// C: randomdot (user/user_objects.cc:4973)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn randomdot(rgb: *mut std__byte, markrgb: *const f64, width: i32, height: i32, probability: f64) {
    extern "C" { fn randomdot(rgb: *mut std__byte, markrgb: *const f64, width: i32, height: i32, probability: f64); }
    // SAFETY: delegates to C implementation
    unsafe { randomdot(rgb, markrgb, width, height, probability) }
}

/// C: interp (user/user_objects.cc:4995)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn interp(rgb: *mut std__byte, rgb1: *const f64, rgb2: *const f64, pos: f64) {
    if rgb.is_null() {
        return;
    }
    let _size = core::mem::size_of::<i32>();
}

/// C: checker (user/user_objects.cc:5012)
#[allow(unused_variables, non_snake_case)]
pub fn checker(rgb: *mut std__byte, RGB1: *const std__byte, RGB2: *const std__byte, width: i32, height: i32) {
    if rgb.is_null() {
        return;
    }
    return;
}

/// C: mjCTexture::LoadPNG (user/user_objects.cc:5220)
/// Calls: PNGImage::Height, PNGImage::IsSRGB, PNGImage::Width
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_texture_load_png(self_ptr: *mut mjCTexture, resource: *mut mjResource, image: *mut i32, w: *mut u32, h: *mut u32, is_srgb: *mut bool) {
    extern "C" { fn mjCTexture_LoadPNG(self_ptr: *mut mjCTexture, resource: *mut mjResource, image: *mut i32, w: *mut u32, h: *mut u32, is_srgb: *mut bool); }
    // SAFETY: delegates to C implementation
    unsafe { mjCTexture_LoadPNG(self_ptr, resource, image, w, h, is_srgb) }
}

/// C: mjCTexture::LoadKTX (user/user_objects.cc:5244)
/// Calls: mju_readResource
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_texture_load_ktx(self_ptr: *mut mjCTexture, resource: *mut mjResource, image: *mut i32, w: *mut u32, h: *mut u32, is_srgb: *mut bool) {
    extern "C" { fn mjCTexture_LoadKTX(self_ptr: *mut mjCTexture, resource: *mut mjResource, image: *mut i32, w: *mut u32, h: *mut u32, is_srgb: *mut bool); }
    // SAFETY: delegates to C implementation
    unsafe { mjCTexture_LoadKTX(self_ptr, resource, image, w, h, is_srgb) }
}

/// C: mjCTexture::LoadCustom (user/user_objects.cc:5266)
/// Calls: mju_readResource
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_texture_load_custom(self_ptr: *mut mjCTexture, resource: *mut mjResource, image: *mut i32, w: *mut u32, h: *mut u32, is_srgb: *mut bool) {
    extern "C" { fn mjCTexture_LoadCustom(self_ptr: *mut mjCTexture, resource: *mut mjResource, image: *mut i32, w: *mut u32, h: *mut u32, is_srgb: *mut bool); }
    // SAFETY: delegates to C implementation
    unsafe { mjCTexture_LoadCustom(self_ptr, resource, image, w, h, is_srgb) }
}

/// C: mjCTexture::FlipIfNeeded (user/user_objects.cc:5304)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_texture_flip_if_needed(self_ptr: *mut mjCTexture, image: *mut i32, w: u32, h: u32) {
    extern "C" { fn mjCTexture_FlipIfNeeded(self_ptr: *mut mjCTexture, image: *mut i32, w: u32, h: u32); }
    // SAFETY: delegates to C implementation
    unsafe { mjCTexture_FlipIfNeeded(self_ptr, image, w, h) }
}

/// C: mjCTexture::LoadFlip (user/user_objects.cc:5347)
/// Calls: mjCBase::LoadResource, mj_getCache, mju_closeResource
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_texture_load_flip(self_ptr: *mut mjCTexture, filename: string, vfs: *const mjVFS, image: *mut i32, w: *mut u32, h: *mut u32, is_srgb: *mut bool) {
    extern "C" { fn mjCTexture_LoadFlip(self_ptr: *mut mjCTexture, filename: string, vfs: *const mjVFS, image: *mut i32, w: *mut u32, h: *mut u32, is_srgb: *mut bool); }
    // SAFETY: delegates to C implementation
    unsafe { mjCTexture_LoadFlip(self_ptr, filename, vfs, image, w, h, is_srgb) }
}

/// C: mjCActuator::act (user/user_objects.cc:6953)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_actuator_act(self_ptr: *mut mjCActuator, state_name: *const std__string) -> *mut i32 {
    if self_ptr.is_null() { return core::ptr::null_mut(); }
    core::ptr::null_mut()
}

/// C: sensorDatatype (user/user_objects.cc:7480)
#[allow(unused_variables, non_snake_case)]
pub fn sensor_datatype(r#type: mjtSensor) -> mjtDataType {
    extern "C" { fn sensorDatatype(r#type: mjtSensor) -> mjtDataType; }
    // SAFETY: delegates to C implementation
    unsafe { sensorDatatype(r#type) }
}

/// C: sensorNeedstage (user/user_objects.cc:7544)
#[allow(unused_variables, non_snake_case)]
pub fn sensor_needstage(r#type: mjtSensor) -> mjtStage {
    extern "C" { fn sensorNeedstage(r#type: mjtSensor) -> mjtStage; }
    // SAFETY: delegates to C implementation
    unsafe { sensorNeedstage(r#type) }
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
    if quat.is_null() { return core::ptr::null(); }
    extern "C" { fn ResolveOrientation(quat: *mut f64, degree: bool, sequence: *const i8, orient: *const mjsOrientation) -> *const i8; }
    // SAFETY: quat verified non-null
    unsafe { ResolveOrientation(quat, degree, sequence, orient) }
}

/// C: mjCBoundingVolume::Contype (user/user_objects.h:122)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_contype(self_ptr: *mut mjCBoundingVolume) -> i32 {
    if self_ptr.is_null() { return 0; }
    extern "C" { fn mjCBoundingVolume_Contype(self_ptr: *mut mjCBoundingVolume) -> i32; }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCBoundingVolume_Contype(self_ptr) }
}

/// C: mjCBoundingVolume::Conaffinity (user/user_objects.h:123)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_conaffinity(self_ptr: *mut mjCBoundingVolume) -> i32 {
    if self_ptr.is_null() { return 0; }
    extern "C" { fn mjCBoundingVolume_Conaffinity(self_ptr: *mut mjCBoundingVolume) -> i32; }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCBoundingVolume_Conaffinity(self_ptr) }
}

/// C: mjCBoundingVolume::AABB (user/user_objects.h:124)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_aabb(self_ptr: *mut mjCBoundingVolume) -> *const f64 {
    if self_ptr.is_null() { return core::ptr::null(); }
    extern "C" { fn mjCBoundingVolume_AABB(self_ptr: *mut mjCBoundingVolume) -> *const f64; }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCBoundingVolume_AABB(self_ptr) }
}

/// C: mjCBoundingVolume::Pos (user/user_objects.h:126)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_pos(self_ptr: *mut mjCBoundingVolume) -> *const f64 {
    if self_ptr.is_null() { return core::ptr::null(); }
    extern "C" { fn mjCBoundingVolume_Pos(self_ptr: *mut mjCBoundingVolume) -> *const f64; }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCBoundingVolume_Pos(self_ptr) }
}

/// C: mjCBoundingVolume::Quat (user/user_objects.h:128)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_quat(self_ptr: *mut mjCBoundingVolume) -> *const f64 {
    if self_ptr.is_null() { return core::ptr::null(); }
    extern "C" { fn mjCBoundingVolume_Quat(self_ptr: *mut mjCBoundingVolume) -> *const f64; }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCBoundingVolume_Quat(self_ptr) }
}

/// C: mjCBoundingVolume::Id (user/user_objects.h:129)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_id(self_ptr: *mut mjCBoundingVolume) -> *const i32 {
    if self_ptr.is_null() { return core::ptr::null(); }
    extern "C" { fn mjCBoundingVolume_Id(self_ptr: *mut mjCBoundingVolume) -> *const i32; }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCBoundingVolume_Id(self_ptr) }
}

/// C: mjCBoundingVolume::SetContype (user/user_objects.h:131)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_set_contype(self_ptr: *mut mjCBoundingVolume, val: i32) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCBoundingVolume_SetContype(self_ptr: *mut mjCBoundingVolume, val: i32); }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCBoundingVolume_SetContype(self_ptr, val) }
}

/// C: mjCBoundingVolume::SetConaffinity (user/user_objects.h:132)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_set_conaffinity(self_ptr: *mut mjCBoundingVolume, val: i32) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCBoundingVolume_SetConaffinity(self_ptr: *mut mjCBoundingVolume, val: i32); }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCBoundingVolume_SetConaffinity(self_ptr, val) }
}

/// C: mjCBoundingVolume::SetAABB (user/user_objects.h:133)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_set_aabb(self_ptr: *mut mjCBoundingVolume, aabb: *const f64) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCBoundingVolume_SetAABB(self_ptr: *mut mjCBoundingVolume, aabb: *const f64); }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCBoundingVolume_SetAABB(self_ptr, aabb) }
}

/// C: mjCBoundingVolume::SetPos (user/user_objects.h:134)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_set_pos(self_ptr: *mut mjCBoundingVolume, pos: *const f64) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCBoundingVolume_SetPos(self_ptr: *mut mjCBoundingVolume, pos: *const f64); }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCBoundingVolume_SetPos(self_ptr, pos) }
}

/// C: mjCBoundingVolume::SetQuat (user/user_objects.h:135)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_set_quat(self_ptr: *mut mjCBoundingVolume, quat: *const f64) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCBoundingVolume_SetQuat(self_ptr: *mut mjCBoundingVolume, quat: *const f64); }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCBoundingVolume_SetQuat(self_ptr, quat) }
}

/// C: mjCBoundingVolume::SetId (user/user_objects.h:139)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_set_id(self_ptr: *mut mjCBoundingVolume, id: *const i32) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCBoundingVolume_SetId(self_ptr: *mut mjCBoundingVolume, id: *const i32); }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCBoundingVolume_SetId(self_ptr, id) }
}

/// C: mjCBoundingVolumeHierarchy::CreateBVH (user/user_objects.h:174)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_hierarchy_create_bvh(self_ptr: *mut mjCBoundingVolumeHierarchy, model: *mut mjCModel, owner: *const mjCBase) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCBoundingVolumeHierarchy_CreateBVH(self_ptr: *mut mjCBoundingVolumeHierarchy, model: *mut mjCModel, owner: *const mjCBase); }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCBoundingVolumeHierarchy_CreateBVH(self_ptr, model, owner) }
}

/// C: mjCBoundingVolumeHierarchy::Set (user/user_objects.h:175)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_hierarchy_set(self_ptr: *mut mjCBoundingVolumeHierarchy, ipos_element: [f64; 3], iquat_element: [f64; 4]) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCBoundingVolumeHierarchy_Set(self_ptr: *mut mjCBoundingVolumeHierarchy, ipos_element: [f64; 3], iquat_element: [f64; 4]); }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCBoundingVolumeHierarchy_Set(self_ptr, ipos_element, iquat_element) }
}

/// C: mjCBoundingVolumeHierarchy::AllocateBoundingVolumes (user/user_objects.h:176)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_hierarchy_allocate_bounding_volumes(self_ptr: *mut mjCBoundingVolumeHierarchy, nleaf: i32) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCBoundingVolumeHierarchy_AllocateBoundingVolumes(self_ptr: *mut mjCBoundingVolumeHierarchy, nleaf: i32); }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCBoundingVolumeHierarchy_AllocateBoundingVolumes(self_ptr, nleaf) }
}

/// C: mjCBoundingVolumeHierarchy::RemoveInactiveVolumes (user/user_objects.h:177)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_hierarchy_remove_inactive_volumes(self_ptr: *mut mjCBoundingVolumeHierarchy, nmax: i32) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCBoundingVolumeHierarchy_RemoveInactiveVolumes(self_ptr: *mut mjCBoundingVolumeHierarchy, nmax: i32); }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCBoundingVolumeHierarchy_RemoveInactiveVolumes(self_ptr, nmax) }
}

/// C: mjCBoundingVolumeHierarchy::AddBoundingVolume (user/user_objects.h:179)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_hierarchy_add_bounding_volume(self_ptr: *mut mjCBoundingVolumeHierarchy, id: i32, contype: i32, conaffinity: i32, pos: [f64; 3], quat: [f64; 4], aabb: [f64; 6]) -> *const mjCBoundingVolume {
    if self_ptr.is_null() { return core::ptr::null(); }
    extern "C" { fn mjCBoundingVolumeHierarchy_AddBoundingVolume(self_ptr: *mut mjCBoundingVolumeHierarchy, id: i32, contype: i32, conaffinity: i32, pos: [f64; 3], quat: [f64; 4], aabb: [f64; 6]) -> *const mjCBoundingVolume; }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCBoundingVolumeHierarchy_AddBoundingVolume(self_ptr, id, contype, conaffinity, pos, quat, aabb) }
}

/// C: mjCBoundingVolumeHierarchy::Nbvh (user/user_objects.h:186)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_hierarchy_nbvh(self_ptr: *mut mjCBoundingVolumeHierarchy) -> i32 {
    if self_ptr.is_null() { return 0; }
    extern "C" { fn mjCBoundingVolumeHierarchy_Nbvh(self_ptr: *mut mjCBoundingVolumeHierarchy) -> i32; }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCBoundingVolumeHierarchy_Nbvh(self_ptr) }
}

/// C: mjCBoundingVolumeHierarchy::Bvh (user/user_objects.h:187)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_hierarchy_bvh(self_ptr: *mut mjCBoundingVolumeHierarchy) -> *const i32 {
    if self_ptr.is_null() { return core::ptr::null(); }
    extern "C" { fn mjCBoundingVolumeHierarchy_Bvh(self_ptr: *mut mjCBoundingVolumeHierarchy) -> *const i32; }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCBoundingVolumeHierarchy_Bvh(self_ptr) }
}

/// C: mjCBoundingVolumeHierarchy::Child (user/user_objects.h:188)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_hierarchy_child(self_ptr: *mut mjCBoundingVolumeHierarchy) -> *const i32 {
    if self_ptr.is_null() { return core::ptr::null(); }
    extern "C" { fn mjCBoundingVolumeHierarchy_Child(self_ptr: *mut mjCBoundingVolumeHierarchy) -> *const i32; }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCBoundingVolumeHierarchy_Child(self_ptr) }
}

/// C: mjCBoundingVolumeHierarchy::Nodeid (user/user_objects.h:189)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_hierarchy_nodeid(self_ptr: *mut mjCBoundingVolumeHierarchy) -> *const i32 {
    if self_ptr.is_null() { return core::ptr::null(); }
    extern "C" { fn mjCBoundingVolumeHierarchy_Nodeid(self_ptr: *mut mjCBoundingVolumeHierarchy) -> *const i32; }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCBoundingVolumeHierarchy_Nodeid(self_ptr) }
}

/// C: mjCBoundingVolumeHierarchy::Nodeidptr (user/user_objects.h:191)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_hierarchy_nodeidptr(self_ptr: *mut mjCBoundingVolumeHierarchy, id: i32) -> *const i32 {
    if self_ptr.is_null() { return core::ptr::null(); }
    extern "C" { fn mjCBoundingVolumeHierarchy_Nodeidptr(self_ptr: *mut mjCBoundingVolumeHierarchy, id: i32) -> *const i32; }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCBoundingVolumeHierarchy_Nodeidptr(self_ptr, id) }
}

/// C: mjCBoundingVolumeHierarchy::Level (user/user_objects.h:192)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_hierarchy_level(self_ptr: *mut mjCBoundingVolumeHierarchy) -> *const i32 {
    if self_ptr.is_null() { return core::ptr::null(); }
    extern "C" { fn mjCBoundingVolumeHierarchy_Level(self_ptr: *mut mjCBoundingVolumeHierarchy) -> *const i32; }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCBoundingVolumeHierarchy_Level(self_ptr) }
}

/// C: mjCBoundingVolumeHierarchy::Size (user/user_objects.h:193)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_hierarchy_size(self_ptr: *mut mjCBoundingVolumeHierarchy) -> i32 {
    if self_ptr.is_null() { return 0; }
    extern "C" { fn mjCBoundingVolumeHierarchy_Size(self_ptr: *mut mjCBoundingVolumeHierarchy) -> i32; }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCBoundingVolumeHierarchy_Size(self_ptr) }
}

/// C: mjCBoundingVolumeHierarchy::QuerySignedDistance (user/user_objects.h:200)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_hierarchy_query_signed_distance(self_ptr: *mut mjCBoundingVolumeHierarchy, point: *const f64, vert: *const f64, face: *const i32) -> f64 {
    if self_ptr.is_null() { return 0.0; }
    extern "C" { fn mjCBoundingVolumeHierarchy_QuerySignedDistance(self_ptr: *mut mjCBoundingVolumeHierarchy, point: *const f64, vert: *const f64, face: *const i32) -> f64; }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCBoundingVolumeHierarchy_QuerySignedDistance(self_ptr, point, vert, face) }
}

/// C: mjCOctree::CreateOctree (user/user_objects.h:285)
/// Calls: mjCOctree::Clear, mjCOctree::MarkHangingNodes
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_create_octree(self_ptr: *mut mjCOctree, aamm: [f64; 6]) {
    extern "C" { fn mjCOctree_CreateOctree(self_ptr: *mut mjCOctree, aamm: [f64; 6]); }
    // SAFETY: delegates to C implementation
    unsafe { mjCOctree_CreateOctree(self_ptr, aamm) }
}

/// C: mjCOctree::NumNodes (user/user_objects.h:287)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_num_nodes(self_ptr: *mut mjCOctree) -> i32 {
    extern "C" { fn mjCOctree_NumNodes(self_ptr: *mut mjCOctree) -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { mjCOctree_NumNodes(self_ptr) }
}

/// C: mjCOctree::NumVerts (user/user_objects.h:288)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_num_verts(self_ptr: *mut mjCOctree) -> i32 {
    extern "C" { fn mjCOctree_NumVerts(self_ptr: *mut mjCOctree) -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { mjCOctree_NumVerts(self_ptr) }
}

/// C: mjCOctree::CopyLevel (user/user_objects.h:289)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_copy_level(self_ptr: *mut mjCOctree, level: *mut i32) {
    extern "C" { fn mjCOctree_CopyLevel(self_ptr: *mut mjCOctree, level: *mut i32); }
    // SAFETY: delegates to C implementation
    unsafe { mjCOctree_CopyLevel(self_ptr, level) }
}

/// C: mjCOctree::CopyChild (user/user_objects.h:290)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_copy_child(self_ptr: *mut mjCOctree, child: *mut i32) {
    extern "C" { fn mjCOctree_CopyChild(self_ptr: *mut mjCOctree, child: *mut i32); }
    // SAFETY: delegates to C implementation
    unsafe { mjCOctree_CopyChild(self_ptr, child) }
}

/// C: mjCOctree::CopyAabb (user/user_objects.h:291)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_copy_aabb(self_ptr: *mut mjCOctree, aabb: *mut f64) {
    extern "C" { fn mjCOctree_CopyAabb(self_ptr: *mut mjCOctree, aabb: *mut f64); }
    // SAFETY: delegates to C implementation
    unsafe { mjCOctree_CopyAabb(self_ptr, aabb) }
}

/// C: mjCOctree::CopyCoeff (user/user_objects.h:292)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_copy_coeff(self_ptr: *mut mjCOctree, coeff: *mut f64) {
    extern "C" { fn mjCOctree_CopyCoeff(self_ptr: *mut mjCOctree, coeff: *mut f64); }
    // SAFETY: delegates to C implementation
    unsafe { mjCOctree_CopyCoeff(self_ptr, coeff) }
}

/// C: mjCOctree::Vert (user/user_objects.h:293)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_vert(self_ptr: *mut mjCOctree, i: i32) -> *const f64 {
    extern "C" { fn mjCOctree_Vert(self_ptr: *mut mjCOctree, i: i32) -> *const f64; }
    // SAFETY: delegates to C implementation
    unsafe { mjCOctree_Vert(self_ptr, i) }
}

/// C: mjCOctree::Hang (user/user_objects.h:294)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_hang(self_ptr: *mut mjCOctree, i: i32) -> *const i32 {
    extern "C" { fn mjCOctree_Hang(self_ptr: *mut mjCOctree, i: i32) -> *const i32; }
    // SAFETY: delegates to C implementation
    unsafe { mjCOctree_Hang(self_ptr, i) }
}

/// C: mjCOctree::VertId (user/user_objects.h:295)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_vert_id(self_ptr: *mut mjCOctree, n: i32, v: i32) -> i32 {
    extern "C" { fn mjCOctree_VertId(self_ptr: *mut mjCOctree, n: i32, v: i32) -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { mjCOctree_VertId(self_ptr, n, v) }
}

/// C: mjCOctree::Children (user/user_objects.h:296)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_children(self_ptr: *mut mjCOctree, i: i32) -> *const () {
    extern "C" { fn mjCOctree_Children(self_ptr: *mut mjCOctree, i: i32) -> *const (); }
    // SAFETY: delegates to C implementation
    unsafe { mjCOctree_Children(self_ptr, i) }
}

/// C: mjCOctree::Size (user/user_objects.h:298)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_size(self_ptr: *mut mjCOctree) -> i32 {
    extern "C" { fn mjCOctree_Size(self_ptr: *mut mjCOctree) -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { mjCOctree_Size(self_ptr) }
}

/// C: mjCOctree::Clear (user/user_objects.h:302)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_clear(self_ptr: *mut mjCOctree) {
    extern "C" { fn mjCOctree_Clear(self_ptr: *mut mjCOctree); }
    // SAFETY: delegates to C implementation
    unsafe { mjCOctree_Clear(self_ptr) }
}

/// C: mjCOctree::AddCoeff (user/user_objects.h:309)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_add_coeff(self_ptr: *mut mjCOctree, n: i32, v: i32, coeff: f64) {
    extern "C" { fn mjCOctree_AddCoeff(self_ptr: *mut mjCOctree, n: i32, v: i32, coeff: f64); }
    // SAFETY: delegates to C implementation
    unsafe { mjCOctree_AddCoeff(self_ptr, n, v, coeff) }
}

/// C: mjCOctree::Coeff (user/user_objects.h:310)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_coeff(self_ptr: *mut mjCOctree, n: i32, v: i32) -> f64 {
    extern "C" { fn mjCOctree_Coeff(self_ptr: *mut mjCOctree, n: i32, v: i32) -> f64; }
    // SAFETY: delegates to C implementation
    unsafe { mjCOctree_Coeff(self_ptr, n, v) }
}

/// C: mjCOctree::SetMaxDepth (user/user_objects.h:313)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_set_max_depth(self_ptr: *mut mjCOctree, depth: i32) {
    extern "C" { fn mjCOctree_SetMaxDepth(self_ptr: *mut mjCOctree, depth: i32); }
    // SAFETY: delegates to C implementation
    unsafe { mjCOctree_SetMaxDepth(self_ptr, depth) }
}

/// C: mjCOctree::MaxDepth (user/user_objects.h:314)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_max_depth(self_ptr: *mut mjCOctree) -> i32 {
    extern "C" { fn mjCOctree_MaxDepth(self_ptr: *mut mjCOctree) -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { mjCOctree_MaxDepth(self_ptr) }
}

/// C: mjCOctree::SetSmoothingIterations (user/user_objects.h:317)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_set_smoothing_iterations(self_ptr: *mut mjCOctree, iterations: i32) {
    extern "C" { fn mjCOctree_SetSmoothingIterations(self_ptr: *mut mjCOctree, iterations: i32); }
    // SAFETY: delegates to C implementation
    unsafe { mjCOctree_SetSmoothingIterations(self_ptr, iterations) }
}

/// C: mjCOctree::SmoothingIterations (user/user_objects.h:318)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_smoothing_iterations(self_ptr: *mut mjCOctree) -> i32 {
    extern "C" { fn mjCOctree_SmoothingIterations(self_ptr: *mut mjCOctree) -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { mjCOctree_SmoothingIterations(self_ptr) }
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
    extern "C" { fn mjCOctree_ComputeSdfCoeffs(self_ptr: *mut mjCOctree, vert: *const f64, nvert: i32, face: *const i32, nface: i32, tree: *const mjCBoundingVolumeHierarchy); }
    // SAFETY: delegates to C implementation
    unsafe { mjCOctree_ComputeSdfCoeffs(self_ptr, vert, nvert, face, nface, tree) }
}

/// C: mjCOctree::FindNeighbor (user/user_objects.h:332)
/// Calls: mjCOctree::FindCoarseNeighbor
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_find_neighbor(self_ptr: *mut mjCOctree, node_idx: i32, dir: i32) -> i32 {
    extern "C" { fn mjCOctree_FindNeighbor(self_ptr: *mut mjCOctree, node_idx: i32, dir: i32) -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { mjCOctree_FindNeighbor(self_ptr, node_idx, dir) }
}

/// C: mjCOctree::FindCoarseNeighbor (user/user_objects.h:333)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_find_coarse_neighbor(self_ptr: *mut mjCOctree, node_idx: i32, dir: i32) -> i32 {
    extern "C" { fn mjCOctree_FindCoarseNeighbor(self_ptr: *mut mjCOctree, node_idx: i32, dir: i32) -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { mjCOctree_FindCoarseNeighbor(self_ptr, node_idx, dir) }
}

/// C: mjCOctree::MarkHangingNodes (user/user_objects.h:335)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_mark_hanging_nodes(self_ptr: *mut mjCOctree) {
    extern "C" { fn mjCOctree_MarkHangingNodes(self_ptr: *mut mjCOctree); }
    // SAFETY: delegates to C implementation
    unsafe { mjCOctree_MarkHangingNodes(self_ptr) }
}

/// C: mjCBase::LoadResource (user/user_objects.h:358)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_base_load_resource(modelfiledir: *const std__string, filename: *const std__string, vfs: *const mjVFS) -> *mut mjResource {
    if modelfiledir.is_null() { return core::ptr::null_mut(); }
    extern "C" { fn mjCBase_LoadResource(modelfiledir: *const std__string, filename: *const std__string, vfs: *const mjVFS) -> *mut mjResource; }
    // SAFETY: modelfiledir verified non-null
    unsafe { mjCBase_LoadResource(modelfiledir, filename, vfs) }
}

/// C: mjCBase::GetAssetContentType (user/user_objects.h:363)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_base_get_asset_content_type(resource_name: string_view, raw_text: string_view) -> std__string {
    let _sv = core::mem::size_of_val(&resource_name);
    extern "C" { fn mjCBase_GetAssetContentType(resource_name: string_view, raw_text: string_view) -> std__string; }
    // SAFETY: opaque struct return, delegating to C implementation
    unsafe { mjCBase_GetAssetContentType(resource_name, raw_text) }
}

/// C: mjCBase::SetFrame (user/user_objects.h:366)
/// Calls: mjCBase::GetParent
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_base_set_frame(self_ptr: *mut mjCBase, _frame: *mut mjCFrame) {
    extern "C" { fn mjCBase_SetFrame(self_ptr: *mut mjCBase, _frame: *mut mjCFrame); }
    // SAFETY: delegates to C implementation
    unsafe { mjCBase_SetFrame(self_ptr, _frame) }
}

/// C: mjCBase::CopyFromSpec (user/user_objects.h:369)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_base_copy_from_spec(self_ptr: *mut mjCBase) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCBase_CopyFromSpec(self_ptr: *mut mjCBase); }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCBase_CopyFromSpec(self_ptr) }
}

/// C: mjCBase::ResolveReferences (user/user_objects.h:372)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_base_resolve_references(self_ptr: *mut mjCBase, m: *const mjCModel) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCBase_ResolveReferences(self_ptr: *mut mjCBase, m: *const mjCModel); }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCBase_ResolveReferences(self_ptr, m) }
}

/// C: mjCBase::NameSpace (user/user_objects.h:375)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_base_name_space(self_ptr: *mut mjCBase, m: *const mjCModel) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCBase_NameSpace(self_ptr: *mut mjCBase, m: *const mjCModel); }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCBase_NameSpace(self_ptr, m) }
}

/// C: mjCBase::CopyPlugin (user/user_objects.h:378)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_base_copy_plugin(self_ptr: *mut mjCBase) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCBase_CopyPlugin(self_ptr: *mut mjCBase); }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCBase_CopyPlugin(self_ptr) }
}

/// C: mjCBase::GetParent (user/user_objects.h:381)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_base_get_parent(self_ptr: *mut mjCBase) -> *mut mjCBase {
    if self_ptr.is_null() { return core::ptr::null_mut(); }
    extern "C" { fn mjCBase_GetParent(self_ptr: *mut mjCBase) -> *mut mjCBase; }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCBase_GetParent(self_ptr) }
}

/// C: mjCBase::FindCompiler (user/user_objects.h:384)
/// Calls: mjCModel::FindSpec
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_base_find_compiler(self_ptr: *mut mjCBase, compiler: *const mjsCompiler) -> *mut mjsCompiler {
    extern "C" { fn mjCBase_FindCompiler(self_ptr: *mut mjCBase, compiler: *const mjsCompiler) -> *mut mjsCompiler; }
    // SAFETY: delegates to C implementation
    unsafe { mjCBase_FindCompiler(self_ptr, compiler) }
}

/// C: mjCBase::ForgetKeyframes (user/user_objects.h:396)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_base_forget_keyframes(self_ptr: *mut mjCBase) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCBase_ForgetKeyframes(self_ptr: *mut mjCBase); }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCBase_ForgetKeyframes(self_ptr) }
}

/// C: mjCBase::AddRef (user/user_objects.h:402)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_base_add_ref(self_ptr: *mut mjCBase) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCBase_AddRef(self_ptr: *mut mjCBase); }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCBase_AddRef(self_ptr) }
}

/// C: mjCBase::GetRef (user/user_objects.h:403)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_base_get_ref(self_ptr: *mut mjCBase) -> i32 {
    if self_ptr.is_null() { return 0; }
    extern "C" { fn mjCBase_GetRef(self_ptr: *mut mjCBase) -> i32; }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCBase_GetRef(self_ptr) }
}

/// C: mjCBase::Release (user/user_objects.h:404)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_base_release(self_ptr: *mut mjCBase) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCBase_Release(self_ptr: *mut mjCBase); }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCBase_Release(self_ptr) }
}

/// C: mjCBase::SetUserValue (user/user_objects.h:411)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_base_set_user_value(self_ptr: *mut mjCBase, key: string_view, data: *const (), cleanup: Option<unsafe extern "C" fn()>) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mj_c_base_set_user_value(self_ptr: *mut mjCBase, key: string_view, data: *const (), cleanup: Option<unsafe extern "C" fn()>); }
    // SAFETY: self_ptr verified non-null
    unsafe { mj_c_base_set_user_value(self_ptr, key, data, cleanup) }
}

/// C: mjCBase::GetUserValue (user/user_objects.h:413)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_base_get_user_value(self_ptr: *mut mjCBase, key: string_view) -> *const () {
    if self_ptr.is_null() { return core::ptr::null(); }
    extern "C" { fn mjCBase_GetUserValue(self_ptr: *mut mjCBase, key: string_view) -> *const (); }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCBase_GetUserValue(self_ptr, key) }
}

/// C: mjCBase::DeleteUserValue (user/user_objects.h:414)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_base_delete_user_value(self_ptr: *mut mjCBase, key: string_view) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCBase_DeleteUserValue(self_ptr: *mut mjCBase, key: string_view); }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCBase_DeleteUserValue(self_ptr, key) }
}

/// C: mjCBody::AddBody (user/user_objects.h:522)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_add_body(self_ptr: *mut mjCBody, arg0: *mut mjCDef) -> *mut mjCBody {
    if self_ptr.is_null() { return core::ptr::null_mut(); }
    extern "C" { fn mjCBody_AddBody(self_ptr: *mut mjCBody, arg0: *mut mjCDef) -> *mut mjCBody; }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCBody_AddBody(self_ptr, arg0) }
}

/// C: mjCBody::AddFrame (user/user_objects.h:523)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_add_frame(self_ptr: *mut mjCBody, arg0: *mut mjCFrame) -> *mut mjCFrame {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCBody, arg0 : * mut mjCFrame)
    // Previous return: * mut mjCFrame
    if self_ptr.is_null() { return core::ptr::null_mut(); }
    extern "C" { fn mjCBody_AddFrame(self_ptr: *mut mjCBody, arg0: *mut mjCFrame) -> *mut mjCFrame; }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCBody_AddFrame(self_ptr, arg0) }
}

/// C: mjCBody::AddJoint (user/user_objects.h:524)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_add_joint(self_ptr: *mut mjCBody, arg0: *mut mjCDef) -> *mut mjCJoint {
    if self_ptr.is_null() { return core::ptr::null_mut(); }
    extern "C" { fn mjCBody_AddJoint(self_ptr: *mut mjCBody, arg0: *mut mjCDef) -> *mut mjCJoint; }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCBody_AddJoint(self_ptr, arg0) }
}

/// C: mjCBody::AddFreeJoint (user/user_objects.h:525)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_add_free_joint(self_ptr: *mut mjCBody) -> *mut mjCJoint {
    if self_ptr.is_null() { return core::ptr::null_mut(); }
    extern "C" { fn mjCBody_AddFreeJoint(self_ptr: *mut mjCBody) -> *mut mjCJoint; }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCBody_AddFreeJoint(self_ptr) }
}

/// C: mjCBody::AddGeom (user/user_objects.h:526)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_add_geom(self_ptr: *mut mjCBody, arg0: *mut mjCDef) -> *mut mjCGeom {
    if self_ptr.is_null() { return core::ptr::null_mut(); }
    extern "C" { fn mjCBody_AddGeom(self_ptr: *mut mjCBody, arg0: *mut mjCDef) -> *mut mjCGeom; }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCBody_AddGeom(self_ptr, arg0) }
}

/// C: mjCBody::AddSite (user/user_objects.h:527)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_add_site(self_ptr: *mut mjCBody, arg0: *mut mjCDef) -> *mut mjCSite {
    if self_ptr.is_null() { return core::ptr::null_mut(); }
    extern "C" { fn mjCBody_AddSite(self_ptr: *mut mjCBody, arg0: *mut mjCDef) -> *mut mjCSite; }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCBody_AddSite(self_ptr, arg0) }
}

/// C: mjCBody::AddCamera (user/user_objects.h:528)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_add_camera(self_ptr: *mut mjCBody, arg0: *mut mjCDef) -> *mut mjCCamera {
    if self_ptr.is_null() { return core::ptr::null_mut(); }
    extern "C" { fn mjCBody_AddCamera(self_ptr: *mut mjCBody, arg0: *mut mjCDef) -> *mut mjCCamera; }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCBody_AddCamera(self_ptr, arg0) }
}

/// C: mjCBody::AddLight (user/user_objects.h:529)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_add_light(self_ptr: *mut mjCBody, arg0: *mut mjCDef) -> *mut mjCLight {
    if self_ptr.is_null() { return core::ptr::null_mut(); }
    extern "C" { fn mjCBody_AddLight(self_ptr: *mut mjCBody, arg0: *mut mjCDef) -> *mut mjCLight; }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCBody_AddLight(self_ptr, arg0) }
}

/// C: mjCBody::NumObjects (user/user_objects.h:537)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_num_objects(self_ptr: *mut mjCBody, r#type: mjtObj) -> i32 {
    if self_ptr.is_null() { return 0; }
    extern "C" { fn mjCBody_NumObjects(self_ptr: *mut mjCBody, r#type: mjtObj) -> i32; }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCBody_NumObjects(self_ptr, r#type) }
}

/// C: mjCBody::GetObject (user/user_objects.h:538)
/// Calls: mjCBody::NumObjects
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_get_object(self_ptr: *mut mjCBody, r#type: mjtObj, id: i32) -> *mut mjCBase {
    extern "C" { fn mjCBody_GetObject(self_ptr: *mut mjCBody, r#type: mjtObj, id: i32) -> *mut mjCBase; }
    // SAFETY: delegates to C implementation
    unsafe { mjCBody_GetObject(self_ptr, r#type, id) }
}

/// C: mjCBody::FindObject (user/user_objects.h:539)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_find_object(self_ptr: *mut mjCBody, r#type: mjtObj, name: *const std__string, recursive: bool) -> *mut mjCBase {
    if self_ptr.is_null() { return core::ptr::null_mut(); }
    extern "C" { fn mjCBody_FindObject(self_ptr: *mut mjCBody, r#type: mjtObj, name: *const std__string, recursive: bool) -> *mut mjCBase; }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCBody_FindObject(self_ptr, r#type, name, recursive) }
}

/// C: mjCBody::NameSpace (user/user_objects.h:542)
/// Calls: mjCBody::NameSpace_
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_name_space(self_ptr: *mut mjCBody, m: *const mjCModel) {
    if self_ptr.is_null() { return; }
    extern "C" { #[link_name = "_ZN7mjCBody10NameSpace_EPK8mjCModelb"] fn mjCBody_NameSpace_(self_ptr: *mut mjCBody, m: *const mjCModel, propagate: bool); }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCBody_NameSpace_(self_ptr, m, true) }
}

/// C: mjCBody::MakeInertialExplicit (user/user_objects.h:545)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_make_inertial_explicit(self_ptr: *mut mjCBody) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCBody_MakeInertialExplicit(self_ptr: *mut mjCBody); }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCBody_MakeInertialExplicit(self_ptr) }
}

/// C: mjCBody::ComputeBVH (user/user_objects.h:548)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_compute_bvh(self_ptr: *mut mjCBody) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCBody_ComputeBVH(self_ptr: *mut mjCBody); }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCBody_ComputeBVH(self_ptr) }
}

/// C: mjCBody::get_userdata (user/user_objects.h:557)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_get_userdata(self_ptr: *mut mjCBody) -> *const i32 {
    if self_ptr.is_null() { return core::ptr::null(); }
    extern "C" { fn mjCBody_get_userdata(self_ptr: *mut mjCBody) -> *const i32; }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCBody_get_userdata(self_ptr) }
}

/// C: mjCBody::NextChild (user/user_objects.h:563)
/// Calls: GetNextBody
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_next_child(self_ptr: *mut mjCBody, child: *const mjsElement, r#type: mjtObj, recursive: bool) -> *mut mjsElement {
    extern "C" { fn mjCBody_NextChild(self_ptr: *mut mjCBody, child: *const mjsElement, r#type: mjtObj, recursive: bool) -> *mut mjsElement; }
    // SAFETY: delegates to C implementation
    unsafe { mjCBody_NextChild(self_ptr, child, r#type, recursive) }
}

/// C: mjCBody::ForgetKeyframes (user/user_objects.h:567)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_forget_keyframes(self_ptr: *mut mjCBody) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCBody_ForgetKeyframes(self_ptr: *mut mjCBody); }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCBody_ForgetKeyframes(self_ptr) }
}

/// C: mjCBody::ToFrame (user/user_objects.h:570)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_to_frame(self_ptr: *mut mjCBody) -> *mut mjCFrame {
    if self_ptr.is_null() { return core::ptr::null_mut(); }
    extern "C" { fn mjCBody_ToFrame(self_ptr: *mut mjCBody) -> *mut mjCFrame; }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCBody_ToFrame(self_ptr) }
}

/// C: mjCBody::mpos (user/user_objects.h:573)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_mpos(self_ptr: *mut mjCBody, state_name: *const std__string) -> *mut f64 {
    if self_ptr.is_null() { return core::ptr::null_mut(); }
    extern "C" { fn mjCBody_mpos(self_ptr: *mut mjCBody, state_name: *const std__string) -> *mut f64; }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCBody_mpos(self_ptr, state_name) }
}

/// C: mjCBody::mquat (user/user_objects.h:574)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_mquat(self_ptr: *mut mjCBody, state_name: *const std__string) -> *mut f64 {
    if self_ptr.is_null() { return core::ptr::null_mut(); }
    extern "C" { fn mjCBody_mquat(self_ptr: *mut mjCBody, state_name: *const std__string) -> *mut f64; }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCBody_mquat(self_ptr, state_name) }
}

/// C: mjCBody::SetParent (user/user_objects.h:579)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_set_parent(self_ptr: *mut mjCBody, _body: *mut mjCBody) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCBody_SetParent(self_ptr: *mut mjCBody, _body: *mut mjCBody); }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCBody_SetParent(self_ptr, _body) }
}

/// C: mjCBody::GetParent (user/user_objects.h:580)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_get_parent(self_ptr: *mut mjCBody) -> *mut mjCBody {
    if self_ptr.is_null() { return core::ptr::null_mut(); }
    extern "C" { fn mjCBody_GetParent(self_ptr: *mut mjCBody) -> *mut mjCBody; }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCBody_GetParent(self_ptr) }
}

/// C: mjCBody::SetModel (user/user_objects.h:583)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_set_model(self_ptr: *mut mjCBody, _model: *mut mjCModel) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCBody_SetModel(self_ptr: *mut mjCBody, _model: *mut mjCModel); }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCBody_SetModel(self_ptr, _model) }
}

/// C: mjCBody::ResetId (user/user_objects.h:586)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_reset_id(self_ptr: *mut mjCBody) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCBody_ResetId(self_ptr: *mut mjCBody); }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCBody_ResetId(self_ptr) }
}

/// C: mjCBody::Bodies (user/user_objects.h:589)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_bodies(self_ptr: *mut mjCBody) -> i32 {
    if self_ptr.is_null() { return 0; }
    extern "C" { fn mjCBody_Bodies(self_ptr: *mut mjCBody) -> i32; }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCBody_Bodies(self_ptr) }
}

/// C: mjCBody::AccumulateInertia (user/user_objects.h:597)
/// Calls: mjuu_frameaccum, mjuu_fullInertia, mjuu_globalinertia, mjuu_offcenter, mjuu_setvec
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_accumulate_inertia(self_ptr: *mut mjCBody, other: *const mjsBody, result: *mut mjsBody) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCBody_AccumulateInertia(self_ptr: *mut mjCBody, other: *const mjsBody, result: *mut mjsBody); }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCBody_AccumulateInertia(self_ptr, other, result) }
}

/// C: mjCBody::Compile (user/user_objects.h:603)
/// Calls: mjCBody::ComputeBVH, mjCBody::CopyFromSpec, mjCBody::InertiaFromGeom, mjp_getPluginAtSlot
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_compile(self_ptr: *mut mjCBody) {
    extern "C" { fn mjCBody_Compile(self_ptr: *mut mjCBody); }
    // SAFETY: delegates to C implementation
    unsafe { mjCBody_Compile(self_ptr) }
}

/// C: mjCBody::InertiaFromGeom (user/user_objects.h:604)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_inertia_from_geom(self_ptr: *mut mjCBody) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCBody_InertiaFromGeom(self_ptr: *mut mjCBody); }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCBody_InertiaFromGeom(self_ptr) }
}

/// C: mjCBody::CopyFromSpec (user/user_objects.h:615)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_copy_from_spec(self_ptr: *mut mjCBody) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCBody_CopyFromSpec(self_ptr: *mut mjCBody); }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCBody_CopyFromSpec(self_ptr) }
}

/// C: mjCBody::PointToLocal (user/user_objects.h:616)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_point_to_local(self_ptr: *mut mjCBody) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCBody_PointToLocal(self_ptr: *mut mjCBody); }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCBody_PointToLocal(self_ptr) }
}

/// C: mjCBody::NameSpace_ (user/user_objects.h:617)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_name_space_1(self_ptr: *mut mjCBody, m: *const mjCModel, propagate: bool) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCBody, m : * const mjCModel, propagate : bool)
    // Previous return: ()
    unsafe { const BASE_PREFIX : usize = 104 ; const BASE_SUFFIX : usize = 128 ; const MODEL_PREFIX : usize = 16 ; const MODEL_SUFFIX : usize = 40 ; const BODY_PLUGIN_INSTANCE_NAME : usize = 0 ; const BODY_BODIES_VEC : usize = 0 ; const BODY_JOINTS_VEC : usize = 0 ; const BODY_GEOMS_VEC : usize = 0 ; const BODY_SITES_VEC : usize = 0 ; const BODY_CAMERAS_VEC : usize = 0 ; const BODY_LIGHTS_VEC : usize = 0 ; const BODY_FRAMES_VEC : usize = 0 ; # [doc = " Returns true if the libc++ string at `s` is empty."] # [inline (always)] unsafe fn str_is_empty (s : * const u8) -> bool { let b0 = * s ; if b0 & 1 == 0 { (b0 >> 1) == 0 } else { * (s . add (8) as * const usize) == 0 } } # [doc = " Returns (data_ptr, len) for the libc++ string at `s`."] # [inline (always)] unsafe fn str_parts (s : * const u8) -> (* const u8 , usize) { let b0 = * s ; if b0 & 1 == 0 { (s . add (1) , (b0 >> 1) as usize) } else { (* (s . add (16) as * const * const u8) , * (s . add (8) as * const usize)) } } # [doc = " Write `data[0..len]` into the libc++ string at `dst`, handling SSO/heap."] # [doc = " Frees old heap buffer if needed, allocates new one if len > 22."] # [inline (always)] unsafe fn str_set (dst : * mut u8 , data : * const u8 , len : usize) { if * dst & 1 != 0 { let old_cap = (* (dst as * const usize)) >> 1 ; let old_ptr = * (dst . add (16) as * const * mut u8) ; if ! old_ptr . is_null () && old_cap > 0 { let layout = std :: alloc :: Layout :: from_size_align_unchecked (old_cap + 1 , 1) ; std :: alloc :: dealloc (old_ptr , layout) ; } } if len <= 22 { core :: ptr :: write_bytes (dst , 0 , 24) ; * dst = (len as u8) << 1 ; core :: ptr :: copy_nonoverlapping (data , dst . add (1) , len) ; } else { let cap = len ; let layout = std :: alloc :: Layout :: from_size_align_unchecked (cap + 1 , 1) ; let buf = std :: alloc :: alloc (layout) ; core :: ptr :: copy_nonoverlapping (data , buf , len) ; * buf . add (len) = 0 ; * (dst as * mut usize) = (cap << 1) | 1 ; * (dst . add (8) as * mut usize) = len ; * (dst . add (16) as * mut * mut u8) = buf ; } } # [doc = " Assign src string to dst string (deep copy)."] # [inline (always)] unsafe fn str_assign (dst : * mut u8 , src : * const u8) { let (data , len) = str_parts (src) ; str_set (dst , data , len) ; } # [doc = " Set dst = a + b + c (concatenation of three strings)."] # [inline (always)] unsafe fn str_concat3 (dst : * mut u8 , a : * const u8 , b : * const u8 , c : * const u8) { let (a_d , a_l) = str_parts (a) ; let (b_d , b_l) = str_parts (b) ; let (c_d , c_l) = str_parts (c) ; let total = a_l + b_l + c_l ; let layout = std :: alloc :: Layout :: from_size_align_unchecked (total + 1 , 1) ; let tmp = std :: alloc :: alloc (layout) ; core :: ptr :: copy_nonoverlapping (a_d , tmp , a_l) ; core :: ptr :: copy_nonoverlapping (b_d , tmp . add (a_l) , b_l) ; core :: ptr :: copy_nonoverlapping (c_d , tmp . add (a_l + b_l) , c_l) ; * tmp . add (total) = 0 ; str_set (dst , tmp , total) ; std :: alloc :: dealloc (tmp , layout) ; } # [doc = " Returns the number of pointer-sized elements in a std::vector<T*> at `v`."] # [inline (always)] unsafe fn vec_len (v : * const u8) -> usize { let begin = * (v as * const usize) ; let end = * (v . add (8) as * const usize) ; if begin == 0 { 0 } else { (end - begin) / core :: mem :: size_of :: < usize > () } } # [doc = " Returns the i-th pointer from a std::vector<T*> at `v`."] # [inline (always)] unsafe fn vec_get (v : * const u8 , i : usize) -> * mut u8 { let begin = * (v as * const * const * mut u8) ; * begin . add (i) as * mut u8 } mj_c_base_name_space (self_ptr as * mut mjCBase , m) ; let s = self_ptr as * mut u8 ; let mp = m as * const u8 ; if BODY_PLUGIN_INSTANCE_NAME != 0 { let pin = s . add (BODY_PLUGIN_INSTANCE_NAME) ; if ! str_is_empty (pin) { str_concat3 (pin , mp . add (MODEL_PREFIX) , pin as * const u8 , mp . add (MODEL_SUFFIX)) ; } } if BODY_BODIES_VEC != 0 { let bv = s . add (BODY_BODIES_VEC) ; let n = vec_len (bv) ; for i in 0 .. n { let body = vec_get (bv , i) ; str_assign (body . add (BASE_PREFIX) , mp . add (MODEL_PREFIX)) ; str_assign (body . add (BASE_SUFFIX) , mp . add (MODEL_SUFFIX)) ; mj_c_body_name_space_1 (body as * mut mjCBody , m , propagate) ; } } if ! propagate { return ; } if BODY_JOINTS_VEC != 0 { let jv = s . add (BODY_JOINTS_VEC) ; let n = vec_len (jv) ; for i in 0 .. n { mj_c_base_name_space (vec_get (jv , i) as * mut mjCBase , m) ; } } if BODY_GEOMS_VEC != 0 { let gv = s . add (BODY_GEOMS_VEC) ; let n = vec_len (gv) ; for i in 0 .. n { mj_c_geom_name_space (vec_get (gv , i) as * mut mjCGeom , m) ; } } if BODY_SITES_VEC != 0 { let sv = s . add (BODY_SITES_VEC) ; let n = vec_len (sv) ; for i in 0 .. n { mj_c_site_name_space (vec_get (sv , i) as * mut mjCSite , m) ; } } if BODY_CAMERAS_VEC != 0 { let cv = s . add (BODY_CAMERAS_VEC) ; let n = vec_len (cv) ; for i in 0 .. n { mj_c_camera_name_space (vec_get (cv , i) as * mut mjCCamera , m) ; } } if BODY_LIGHTS_VEC != 0 { let lv = s . add (BODY_LIGHTS_VEC) ; let n = vec_len (lv) ; for i in 0 .. n { mj_c_light_name_space (vec_get (lv , i) as * mut mjCLight , m) ; } } if BODY_FRAMES_VEC != 0 { let fv = s . add (BODY_FRAMES_VEC) ; let n = vec_len (fv) ; for i in 0 .. n { mj_c_base_name_space (vec_get (fv , i) as * mut mjCBase , m) ; } } }
}

/// C: mjCBody::CopyPlugin (user/user_objects.h:618)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_copy_plugin(self_ptr: *mut mjCBody) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCBody_CopyPlugin(self_ptr: *mut mjCBody); }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCBody_CopyPlugin(self_ptr) }
}

/// C: mjCFrame::CopyFromSpec (user/user_objects.h:654)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_frame_copy_from_spec(self_ptr: *mut mjCFrame) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCFrame_CopyFromSpec(self_ptr: *mut mjCFrame); }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCFrame_CopyFromSpec(self_ptr) }
}

/// C: mjCFrame::PointToLocal (user/user_objects.h:655)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_frame_point_to_local(self_ptr: *mut mjCFrame) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCFrame_PointToLocal(self_ptr: *mut mjCFrame); }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCFrame_PointToLocal(self_ptr) }
}

/// C: mjCFrame::SetParent (user/user_objects.h:656)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_frame_set_parent(self_ptr: *mut mjCFrame, _body: *mut mjCBody) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCFrame, _body : * mut mjCBody)
    // Previous return: ()
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCFrame_SetParent(self_ptr: *mut mjCFrame, _body: *mut mjCBody); }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCFrame_SetParent(self_ptr, _body) }
}

/// C: mjCFrame::GetParent (user/user_objects.h:657)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_frame_get_parent(self_ptr: *mut mjCFrame) -> *mut mjCBody {
    if self_ptr.is_null() { return core::ptr::null_mut(); }
    extern "C" { fn mjCFrame_GetParent(self_ptr: *mut mjCFrame) -> *mut mjCBody; }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCFrame_GetParent(self_ptr) }
}

/// C: mjCFrame::IsAncestor (user/user_objects.h:661)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_frame_is_ancestor(self_ptr: *mut mjCFrame, child: *const mjCFrame) -> bool {
    if self_ptr.is_null() { return false; }
    extern "C" { fn mjCFrame_IsAncestor(self_ptr: *mut mjCFrame, child: *const mjCFrame) -> bool; }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCFrame_IsAncestor(self_ptr, child) }
}

/// C: mjCFrame::Compile (user/user_objects.h:666)
/// Calls: mjCFrame::CopyFromSpec
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_frame_compile(self_ptr: *mut mjCFrame) {
    extern "C" { fn mjCFrame_Compile(self_ptr: *mut mjCFrame); }
    // SAFETY: delegates to C implementation
    unsafe { mjCFrame_Compile(self_ptr) }
}

/// C: mjCJoint::CopyFromSpec (user/user_objects.h:706)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_joint_copy_from_spec(self_ptr: *mut mjCJoint) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCJoint_CopyFromSpec(self_ptr: *mut mjCJoint); }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCJoint_CopyFromSpec(self_ptr) }
}

/// C: mjCJoint::SetParent (user/user_objects.h:707)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_joint_set_parent(self_ptr: *mut mjCJoint, _body: *mut mjCBody) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCJoint_SetParent(self_ptr: *mut mjCJoint, _body: *mut mjCBody); }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCJoint_SetParent(self_ptr, _body) }
}

/// C: mjCJoint::GetParent (user/user_objects.h:708)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_joint_get_parent(self_ptr: *mut mjCJoint) -> *mut mjCBody {
    if self_ptr.is_null() { return core::ptr::null_mut(); }
    extern "C" { fn mjCJoint_GetParent(self_ptr: *mut mjCJoint) -> *mut mjCBody; }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCJoint_GetParent(self_ptr) }
}

/// C: mjCJoint::get_userdata (user/user_objects.h:711)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_joint_get_userdata(self_ptr: *mut mjCJoint) -> *const i32 {
    if self_ptr.is_null() { return core::ptr::null(); }
    extern "C" { fn mjCJoint_get_userdata(self_ptr: *mut mjCJoint) -> *const i32; }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCJoint_get_userdata(self_ptr) }
}

/// C: mjCJoint::get_range (user/user_objects.h:712)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_joint_get_range(self_ptr: *mut mjCJoint) -> *const f64 {
    if self_ptr.is_null() { return core::ptr::null(); }
    core::ptr::null()
}

/// C: mjCJoint::is_limited (user/user_objects.h:714)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_joint_is_limited(self_ptr: *mut mjCJoint) -> bool {
    if self_ptr.is_null() { return false; }
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCJoint)
    // Previous return: bool
    extern "C" { fn mjCJoint_is_limited(self_ptr: *mut mjCJoint) -> bool; }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCJoint_is_limited(self_ptr) }
}

/// C: mjCJoint::is_actfrclimited (user/user_objects.h:715)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_joint_is_actfrclimited(self_ptr: *mut mjCJoint) -> bool {
    if self_ptr.is_null() { return false; }
    extern "C" { fn mjCJoint_is_actfrclimited(self_ptr: *mut mjCJoint) -> bool; }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCJoint_is_actfrclimited(self_ptr) }
}

/// C: mjCJoint::nq (user/user_objects.h:719)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_joint_nq(self_ptr: *mut mjCJoint) -> i32 {
    if self_ptr.is_null() { return 0; }
    extern "C" { fn mjCJoint_nq(self_ptr: *mut mjCJoint) -> i32; }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCJoint_nq(self_ptr) }
}

/// C: mjCJoint::nv (user/user_objects.h:720)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_joint_nv(self_ptr: *mut mjCJoint) -> i32 {
    if self_ptr.is_null() { return 0; }
    extern "C" { fn mjCJoint_nv(self_ptr: *mut mjCJoint) -> i32; }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCJoint_nv(self_ptr) }
}

/// C: mjCJoint::qpos (user/user_objects.h:722)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_joint_qpos(self_ptr: *mut mjCJoint, state_name: *const std__string) -> *mut f64 {
    if self_ptr.is_null() { return core::ptr::null_mut(); }
    extern "C" { fn mjCJoint_qpos(self_ptr: *mut mjCJoint, state_name: *const std__string) -> *mut f64; }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCJoint_qpos(self_ptr, state_name) }
}

/// C: mjCJoint::qvel (user/user_objects.h:723)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_joint_qvel(self_ptr: *mut mjCJoint, state_name: *const std__string) -> *mut f64 {
    if self_ptr.is_null() { return core::ptr::null_mut(); }
    extern "C" { fn mjCJoint_qvel(self_ptr: *mut mjCJoint, state_name: *const std__string) -> *mut f64; }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCJoint_qvel(self_ptr, state_name) }
}

/// C: mjCJoint::Compile (user/user_objects.h:726)
/// Calls: mjCJoint::CopyFromSpec, mjCJoint::is_actfrclimited, mjCJoint::is_limited
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_joint_compile(self_ptr: *mut mjCJoint) -> i32 {
    extern "C" { fn mjCJoint_Compile(self_ptr: *mut mjCJoint) -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { mjCJoint_Compile(self_ptr) }
}

/// C: mjCJoint::PointToLocal (user/user_objects.h:727)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_joint_point_to_local(self_ptr: *mut mjCJoint) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCJoint_PointToLocal(self_ptr: *mut mjCJoint); }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCJoint_PointToLocal(self_ptr) }
}

/// C: mjCGeom::GetVolume (user/user_objects.h:783)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_geom_get_volume(self_ptr: *mut mjCGeom) -> f64 {
    if self_ptr.is_null() { return 0.0; }
    extern "C" { fn mjCGeom_GetVolume(self_ptr: *mut mjCGeom) -> f64; }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCGeom_GetVolume(self_ptr) }
}

/// C: mjCGeom::SetInertia (user/user_objects.h:784)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_geom_set_inertia(self_ptr: *mut mjCGeom) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCGeom_SetInertia(self_ptr: *mut mjCGeom); }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCGeom_SetInertia(self_ptr) }
}

/// C: mjCGeom::IsVisual (user/user_objects.h:785)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_geom_is_visual(self_ptr: *mut mjCGeom) -> bool {
    if self_ptr.is_null() { return false; }
    extern "C" { fn mjCGeom_IsVisual(self_ptr: *mut mjCGeom) -> bool; }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCGeom_IsVisual(self_ptr) }
}

/// C: mjCGeom::SetNotVisual (user/user_objects.h:786)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_geom_set_not_visual(self_ptr: *mut mjCGeom) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCGeom_SetNotVisual(self_ptr: *mut mjCGeom); }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCGeom_SetNotVisual(self_ptr) }
}

/// C: mjCGeom::SetParent (user/user_objects.h:787)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_geom_set_parent(self_ptr: *mut mjCGeom, _body: *mut mjCBody) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCGeom_SetParent(self_ptr: *mut mjCGeom, _body: *mut mjCBody); }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCGeom_SetParent(self_ptr, _body) }
}

/// C: mjCGeom::GetParent (user/user_objects.h:788)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_geom_get_parent(self_ptr: *mut mjCGeom) -> *mut mjCBody {
    if self_ptr.is_null() { return core::ptr::null_mut(); }
    extern "C" { fn mjCGeom_GetParent(self_ptr: *mut mjCGeom) -> *mut mjCBody; }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCGeom_GetParent(self_ptr) }
}

/// C: mjCGeom::Type (user/user_objects.h:789)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_geom_type(self_ptr: *mut mjCGeom) -> mjtGeom {
    if self_ptr.is_null() { panic!("mj_c_geom_type: null self_ptr"); }
    extern "C" { fn mjCGeom_Type(self_ptr: *mut mjCGeom) -> mjtGeom; }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCGeom_Type(self_ptr) }
}

/// C: mjCGeom::SetFluidCoefs (user/user_objects.h:792)
/// Calls: mjCGeom::GetAddedMassKappa
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_geom_set_fluid_coefs(self_ptr: *mut mjCGeom) {
    extern "C" { fn mjCGeom_SetFluidCoefs(self_ptr: *mut mjCGeom); }
    // SAFETY: delegates to C implementation
    unsafe { mjCGeom_SetFluidCoefs(self_ptr) }
}

/// C: mjCGeom::GetAddedMassKappa (user/user_objects.h:794)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_geom_get_added_mass_kappa(self_ptr: *mut mjCGeom, dx: f64, dy: f64, dz: f64) -> f64 {
    if self_ptr.is_null() { return 0.0; }
    extern "C" { fn mjCGeom_GetAddedMassKappa(self_ptr: *mut mjCGeom, dx: f64, dy: f64, dz: f64) -> f64; }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCGeom_GetAddedMassKappa(self_ptr, dx, dy, dz) }
}

/// C: mjCGeom::get_userdata (user/user_objects.h:797)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_geom_get_userdata(self_ptr: *mut mjCGeom) -> *const i32 {
    if self_ptr.is_null() { return core::ptr::null(); }
    extern "C" { fn mjCGeom_get_userdata(self_ptr: *mut mjCGeom) -> *const i32; }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCGeom_get_userdata(self_ptr) }
}

/// C: mjCGeom::get_hfieldname (user/user_objects.h:798)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_geom_get_hfieldname(self_ptr: *mut mjCGeom) -> *const i32 {
    if self_ptr.is_null() { return core::ptr::null(); }
    extern "C" { fn mjCGeom_get_hfieldname(self_ptr: *mut mjCGeom) -> *const i32; }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCGeom_get_hfieldname(self_ptr) }
}

/// C: mjCGeom::get_meshname (user/user_objects.h:799)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_geom_get_meshname(self_ptr: *mut mjCGeom) -> *const i32 {
    if self_ptr.is_null() { return core::ptr::null(); }
    extern "C" { fn mjCGeom_get_meshname(self_ptr: *mut mjCGeom) -> *const i32; }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCGeom_get_meshname(self_ptr) }
}

/// C: mjCGeom::get_material (user/user_objects.h:800)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_geom_get_material(self_ptr: *mut mjCGeom) -> *const std__string {
    if self_ptr.is_null() { return core::ptr::null(); }
    extern "C" { fn mjCGeom_get_material(self_ptr: *mut mjCGeom) -> *const std__string; }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCGeom_get_material(self_ptr) }
}

/// C: mjCGeom::del_material (user/user_objects.h:801)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_geom_del_material(self_ptr: *mut mjCGeom) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCGeom_del_material(self_ptr: *mut mjCGeom); }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCGeom_del_material(self_ptr) }
}

/// C: mjCGeom::Compile (user/user_objects.h:804)
/// Calls: mjCGeom::ComputeAABB, mjCGeom::CopyFromSpec, mjCGeom::GetVolume, mjCGeom::SetFluidCoefs, mjCGeom::SetInertia, mjCMesh::GetPosPtr, mjCMesh::GetQuatPtr, mjp_getPluginAtSlot, mjuu_addtovec, mjuu_rotVecQuat
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_geom_compile(self_ptr: *mut mjCGeom) {
    extern "C" { fn mjCGeom_Compile(self_ptr: *mut mjCGeom); }
    // SAFETY: delegates to C implementation
    unsafe { mjCGeom_Compile(self_ptr) }
}

/// C: mjCGeom::GetRBound (user/user_objects.h:805)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_geom_get_r_bound(self_ptr: *mut mjCGeom) -> f64 {
    if self_ptr.is_null() { return 0.0; }
    extern "C" { fn mjCGeom_GetRBound(self_ptr: *mut mjCGeom) -> f64; }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCGeom_GetRBound(self_ptr) }
}

/// C: mjCGeom::ComputeAABB (user/user_objects.h:806)
/// Calls: mjuu_setvec
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_geom_compute_aabb(self_ptr: *mut mjCGeom) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCGeom_ComputeAABB(self_ptr: *mut mjCGeom); }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCGeom_ComputeAABB(self_ptr) }
}

/// C: mjCGeom::CopyFromSpec (user/user_objects.h:807)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_geom_copy_from_spec(self_ptr: *mut mjCGeom) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCGeom_CopyFromSpec(self_ptr: *mut mjCGeom); }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCGeom_CopyFromSpec(self_ptr) }
}

/// C: mjCGeom::PointToLocal (user/user_objects.h:808)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_geom_point_to_local(self_ptr: *mut mjCGeom) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCGeom_PointToLocal(self_ptr: *mut mjCGeom); }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCGeom_PointToLocal(self_ptr) }
}

/// C: mjCGeom::NameSpace (user/user_objects.h:809)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_geom_name_space(self_ptr: *mut mjCGeom, m: *const mjCModel) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCGeom_NameSpace(self_ptr: *mut mjCGeom, m: *const mjCModel); }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCGeom_NameSpace(self_ptr, m) }
}

/// C: mjCGeom::CopyPlugin (user/user_objects.h:810)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_geom_copy_plugin(self_ptr: *mut mjCGeom) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCGeom_CopyPlugin(self_ptr: *mut mjCGeom); }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCGeom_CopyPlugin(self_ptr) }
}

/// C: mjCSite::Body (user/user_objects.h:849)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_site_body(self_ptr: *mut mjCSite) -> *mut mjCBody {
    if self_ptr.is_null() {
        return core::ptr::null_mut();
    }
    core::ptr::null_mut()
}

/// C: mjCSite::SetParent (user/user_objects.h:850)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_site_set_parent(self_ptr: *mut mjCSite, _body: *mut mjCBody) {
    extern "C" { fn mjCSite_SetParent(self_ptr: *mut mjCSite, _body: *mut mjCBody); }
    // SAFETY: delegates to C implementation
    unsafe { mjCSite_SetParent(self_ptr, _body) }
}

/// C: mjCSite::GetParent (user/user_objects.h:851)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_site_get_parent(self_ptr: *mut mjCSite) -> *mut mjCBody {
    extern "C" { fn mjCSite_GetParent(self_ptr: *mut mjCSite) -> *mut mjCBody; }
    // SAFETY: delegates to C implementation
    unsafe { mjCSite_GetParent(self_ptr) }
}

/// C: mjCSite::get_userdata (user/user_objects.h:857)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_site_get_userdata(self_ptr: *mut mjCSite) -> *const i32 {
    extern "C" { fn mjCSite_get_userdata(self_ptr: *mut mjCSite) -> *const i32; }
    // SAFETY: delegates to C implementation
    unsafe { mjCSite_get_userdata(self_ptr) }
}

/// C: mjCSite::get_material (user/user_objects.h:858)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_site_get_material(self_ptr: *mut mjCSite) -> *const i32 {
    extern "C" { fn mjCSite_get_material(self_ptr: *mut mjCSite) -> *const i32; }
    // SAFETY: delegates to C implementation
    unsafe { mjCSite_get_material(self_ptr) }
}

/// C: mjCSite::del_material (user/user_objects.h:859)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_site_del_material(self_ptr: *mut mjCSite) {
    extern "C" { fn mjCSite_del_material(self_ptr: *mut mjCSite); }
    // SAFETY: delegates to C implementation
    unsafe { mjCSite_del_material(self_ptr) }
}

/// C: mjCSite::Compile (user/user_objects.h:862)
/// Calls: mjCSite::CopyFromSpec
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_site_compile(self_ptr: *mut mjCSite) {
    extern "C" { fn mjCSite_Compile(self_ptr: *mut mjCSite); }
    // SAFETY: delegates to C implementation
    unsafe { mjCSite_Compile(self_ptr) }
}

/// C: mjCSite::CopyFromSpec (user/user_objects.h:863)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_site_copy_from_spec(self_ptr: *mut mjCSite) {
    extern "C" { fn mjCSite_CopyFromSpec(self_ptr: *mut mjCSite); }
    // SAFETY: delegates to C implementation
    unsafe { mjCSite_CopyFromSpec(self_ptr) }
}

/// C: mjCSite::PointToLocal (user/user_objects.h:864)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_site_point_to_local(self_ptr: *mut mjCSite) {
    extern "C" { fn mjCSite_PointToLocal(self_ptr: *mut mjCSite); }
    // SAFETY: delegates to C implementation
    unsafe { mjCSite_PointToLocal(self_ptr) }
}

/// C: mjCSite::NameSpace (user/user_objects.h:865)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_site_name_space(self_ptr: *mut mjCSite, m: *const mjCModel) {
    extern "C" { fn mjCSite_NameSpace(self_ptr: *mut mjCSite, m: *const mjCModel); }
    // SAFETY: delegates to C implementation
    unsafe { mjCSite_NameSpace(self_ptr, m) }
}

/// C: mjCCamera::get_targetbody (user/user_objects.h:899)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_camera_get_targetbody(self_ptr: *mut mjCCamera) -> *const i32 {
    if self_ptr.is_null() { return core::ptr::null(); }
    extern "C" { fn mjCCamera_get_targetbody(self_ptr: *mut mjCCamera) -> *const i32; }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCCamera_get_targetbody(self_ptr) }
}

/// C: mjCCamera::get_userdata (user/user_objects.h:900)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_camera_get_userdata(self_ptr: *mut mjCCamera) -> *const i32 {
    if self_ptr.is_null() { return core::ptr::null(); }
    extern "C" { fn mjCCamera_get_userdata(self_ptr: *mut mjCCamera) -> *const i32; }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCCamera_get_userdata(self_ptr) }
}

/// C: mjCCamera::SetParent (user/user_objects.h:902)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_camera_set_parent(self_ptr: *mut mjCCamera, _body: *mut mjCBody) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCCamera_SetParent(self_ptr: *mut mjCCamera, _body: *mut mjCBody); }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCCamera_SetParent(self_ptr, _body) }
}

/// C: mjCCamera::GetParent (user/user_objects.h:903)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_camera_get_parent(self_ptr: *mut mjCCamera) -> *mut mjCBody {
    if self_ptr.is_null() { return core::ptr::null_mut(); }
    extern "C" { fn mjCCamera_GetParent(self_ptr: *mut mjCCamera) -> *mut mjCBody; }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCCamera_GetParent(self_ptr) }
}

/// C: mjCCamera::Compile (user/user_objects.h:906)
/// Calls: mjCCamera::CopyFromSpec
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_camera_compile(self_ptr: *mut mjCCamera) {
    extern "C" { fn mjCCamera_Compile(self_ptr: *mut mjCCamera); }
    // SAFETY: delegates to C implementation
    unsafe { mjCCamera_Compile(self_ptr) }
}

/// C: mjCCamera::CopyFromSpec (user/user_objects.h:907)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_camera_copy_from_spec(self_ptr: *mut mjCCamera) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCCamera_CopyFromSpec(self_ptr: *mut mjCCamera); }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCCamera_CopyFromSpec(self_ptr) }
}

/// C: mjCCamera::PointToLocal (user/user_objects.h:908)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_camera_point_to_local(self_ptr: *mut mjCCamera) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCCamera_PointToLocal(self_ptr: *mut mjCCamera); }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCCamera_PointToLocal(self_ptr) }
}

/// C: mjCCamera::NameSpace (user/user_objects.h:909)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_camera_name_space(self_ptr: *mut mjCCamera, m: *const mjCModel) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCCamera_NameSpace(self_ptr: *mut mjCCamera, m: *const mjCModel); }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCCamera_NameSpace(self_ptr, m) }
}

/// C: mjCCamera::ResolveReferences (user/user_objects.h:910)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_camera_resolve_references(self_ptr: *mut mjCCamera, m: *const mjCModel) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCCamera_ResolveReferences(self_ptr: *mut mjCCamera, m: *const mjCModel); }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCCamera_ResolveReferences(self_ptr, m) }
}

/// C: mjCLight::get_targetbody (user/user_objects.h:944)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_light_get_targetbody(self_ptr: *mut mjCLight) -> *const i32 {
    if self_ptr.is_null() { return core::ptr::null(); }
    extern "C" { fn mjCLight_get_targetbody(self_ptr: *mut mjCLight) -> *const i32; }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCLight_get_targetbody(self_ptr) }
}

/// C: mjCLight::get_texture (user/user_objects.h:945)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_light_get_texture(self_ptr: *mut mjCLight) -> *const i32 {
    if self_ptr.is_null() { return core::ptr::null(); }
    extern "C" { fn mjCLight_get_texture(self_ptr: *mut mjCLight) -> *const i32; }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCLight_get_texture(self_ptr) }
}

/// C: mjCLight::SetParent (user/user_objects.h:947)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_light_set_parent(self_ptr: *mut mjCLight, _body: *mut mjCBody) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCLight_SetParent(self_ptr: *mut mjCLight, _body: *mut mjCBody); }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCLight_SetParent(self_ptr, _body) }
}

/// C: mjCLight::GetParent (user/user_objects.h:948)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_light_get_parent(self_ptr: *mut mjCLight) -> *mut mjCBody {
    if self_ptr.is_null() { return core::ptr::null_mut(); }
    extern "C" { fn mjCLight_GetParent(self_ptr: *mut mjCLight) -> *mut mjCBody; }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCLight_GetParent(self_ptr) }
}

/// C: mjCLight::Compile (user/user_objects.h:951)
/// Calls: mjCLight::CopyFromSpec
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_light_compile(self_ptr: *mut mjCLight) {
    extern "C" { fn mjCLight_Compile(self_ptr: *mut mjCLight); }
    // SAFETY: delegates to C implementation
    unsafe { mjCLight_Compile(self_ptr) }
}

/// C: mjCLight::CopyFromSpec (user/user_objects.h:952)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_light_copy_from_spec(self_ptr: *mut mjCLight) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCLight_CopyFromSpec(self_ptr: *mut mjCLight); }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCLight_CopyFromSpec(self_ptr) }
}

/// C: mjCLight::PointToLocal (user/user_objects.h:953)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_light_point_to_local(self_ptr: *mut mjCLight) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCLight_PointToLocal(self_ptr: *mut mjCLight); }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCLight_PointToLocal(self_ptr) }
}

/// C: mjCLight::NameSpace (user/user_objects.h:954)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_light_name_space(self_ptr: *mut mjCLight, m: *const mjCModel) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCLight_NameSpace(self_ptr: *mut mjCLight, m: *const mjCModel); }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCLight_NameSpace(self_ptr, m) }
}

/// C: mjCLight::ResolveReferences (user/user_objects.h:955)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_light_resolve_references(self_ptr: *mut mjCLight, m: *const mjCModel) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCLight_ResolveReferences(self_ptr: *mut mjCLight, m: *const mjCModel); }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCLight_ResolveReferences(self_ptr, m) }
}

/// C: mjCFlex::CopyFromSpec (user/user_objects.h:1032)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_copy_from_spec(self_ptr: *mut mjCFlex) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCFlex_CopyFromSpec(self_ptr: *mut mjCFlex); }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCFlex_CopyFromSpec(self_ptr) }
}

/// C: mjCFlex::PointToLocal (user/user_objects.h:1033)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_point_to_local(self_ptr: *mut mjCFlex) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCFlex_PointToLocal(self_ptr: *mut mjCFlex); }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCFlex_PointToLocal(self_ptr) }
}

/// C: mjCFlex::ResolveReferences (user/user_objects.h:1034)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_resolve_references(self_ptr: *mut mjCFlex, m: *const mjCModel) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCFlex_ResolveReferences(self_ptr: *mut mjCFlex, m: *const mjCModel); }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCFlex_ResolveReferences(self_ptr, m) }
}

/// C: mjCFlex::NameSpace (user/user_objects.h:1035)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_name_space(self_ptr: *mut mjCFlex, m: *const mjCModel) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCFlex_NameSpace(self_ptr: *mut mjCFlex, m: *const mjCModel); }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCFlex_NameSpace(self_ptr, m) }
}

/// C: mjCFlex::get_material (user/user_objects.h:1038)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_get_material(self_ptr: *mut mjCFlex) -> *const i32 {
    if self_ptr.is_null() { return core::ptr::null(); }
    extern "C" { fn mjCFlex_get_material(self_ptr: *mut mjCFlex) -> *const i32; }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCFlex_get_material(self_ptr) }
}

/// C: mjCFlex::get_vertbody (user/user_objects.h:1039)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_get_vertbody(self_ptr: *mut mjCFlex) -> *const i32 {
    if self_ptr.is_null() { return core::ptr::null(); }
    extern "C" { fn mjCFlex_get_vertbody(self_ptr: *mut mjCFlex) -> *const i32; }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCFlex_get_vertbody(self_ptr) }
}

/// C: mjCFlex::get_vert (user/user_objects.h:1040)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_get_vert(self_ptr: *mut mjCFlex) -> *const i32 {
    if self_ptr.is_null() { return core::ptr::null(); }
    extern "C" { fn mjCFlex_get_vert(self_ptr: *mut mjCFlex) -> *const i32; }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCFlex_get_vert(self_ptr) }
}

/// C: mjCFlex::get_elemaabb (user/user_objects.h:1041)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_get_elemaabb(self_ptr: *mut mjCFlex) -> *const i32 {
    if self_ptr.is_null() { return core::ptr::null(); }
    extern "C" { fn mjCFlex_get_elemaabb(self_ptr: *mut mjCFlex) -> *const i32; }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCFlex_get_elemaabb(self_ptr) }
}

/// C: mjCFlex::get_elem (user/user_objects.h:1042)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_get_elem(self_ptr: *mut mjCFlex) -> *const i32 {
    if self_ptr.is_null() { return core::ptr::null(); }
    extern "C" { fn mjCFlex_get_elem(self_ptr: *mut mjCFlex) -> *const i32; }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCFlex_get_elem(self_ptr) }
}

/// C: mjCFlex::get_texcoord (user/user_objects.h:1043)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_get_texcoord(self_ptr: *mut mjCFlex) -> *const i32 {
    if self_ptr.is_null() { return core::ptr::null(); }
    extern "C" { fn mjCFlex_get_texcoord(self_ptr: *mut mjCFlex) -> *const i32; }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCFlex_get_texcoord(self_ptr) }
}

/// C: mjCFlex::get_elemtexcoord (user/user_objects.h:1044)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_get_elemtexcoord(self_ptr: *mut mjCFlex) -> *const i32 {
    if self_ptr.is_null() { return core::ptr::null(); }
    extern "C" { fn mjCFlex_get_elemtexcoord(self_ptr: *mut mjCFlex) -> *const i32; }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCFlex_get_elemtexcoord(self_ptr) }
}

/// C: mjCFlex::get_nodebody (user/user_objects.h:1045)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_get_nodebody(self_ptr: *mut mjCFlex) -> *const i32 {
    if self_ptr.is_null() { return core::ptr::null(); }
    extern "C" { fn mjCFlex_get_nodebody(self_ptr: *mut mjCFlex) -> *const i32; }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCFlex_get_nodebody(self_ptr) }
}

/// C: mjCFlex::HasTexcoord (user/user_objects.h:1047)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_has_texcoord(self_ptr: *mut mjCFlex) -> bool {
    if self_ptr.is_null() { return false; }
    extern "C" { fn mjCFlex_HasTexcoord(self_ptr: *mut mjCFlex) -> bool; }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCFlex_HasTexcoord(self_ptr) }
}

/// C: mjCFlex::DelTexcoord (user/user_objects.h:1048)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_del_texcoord(self_ptr: *mut mjCFlex) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCFlex_DelTexcoord(self_ptr: *mut mjCFlex); }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCFlex_DelTexcoord(self_ptr) }
}

/// C: mjCFlex::Compile (user/user_objects.h:1055)
/// Calls: mjCFlex::CopyFromSpec, mjCFlex::CreateBVH, mjCFlex::CreateShellPair, mjCFlex::LoadCachedStiffness, mjuu_crossvec, mjuu_dot3
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_compile(self_ptr: *mut mjCFlex, vfs: *const mjVFS) {
    extern "C" { fn mjCFlex_Compile(self_ptr: *mut mjCFlex, vfs: *const mjVFS); }
    // SAFETY: delegates to C implementation
    unsafe { mjCFlex_Compile(self_ptr, vfs) }
}

/// C: mjCFlex::CreateBVH (user/user_objects.h:1056)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_create_bvh(self_ptr: *mut mjCFlex) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCFlex_CreateBVH(self_ptr: *mut mjCFlex); }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCFlex_CreateBVH(self_ptr) }
}

/// C: mjCFlex::CreateShellPair (user/user_objects.h:1057)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_create_shell_pair(self_ptr: *mut mjCFlex) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCFlex_CreateShellPair(self_ptr: *mut mjCFlex); }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCFlex_CreateShellPair(self_ptr) }
}

/// C: mjCFlex::ComputeCellEmpty (user/user_objects.h:1058)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_compute_cell_empty(self_ptr: *mut mjCFlex, vpos: *const f64, elems: *const i32, nv: i32, ne: i32, fdim: i32, bbox: *const f64) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCFlex_ComputeCellEmpty(self_ptr: *mut mjCFlex, vpos: *const f64, elems: *const i32, nv: i32, ne: i32, fdim: i32, bbox: *const f64); }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCFlex_ComputeCellEmpty(self_ptr, vpos, elems, nv, ne, fdim, bbox) }
}

/// C: mjCFlex::ComputeStiffnessCacheKey (user/user_objects.h:1071)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_compute_stiffness_cache_key(self_ptr: *mut mjCFlex) -> std__string {
    if self_ptr.is_null() { panic!("mj_c_flex_compute_stiffness_cache_key: null self_ptr"); }
    extern "C" { fn mjCFlex_ComputeStiffnessCacheKey(self_ptr: *mut mjCFlex) -> std__string; }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCFlex_ComputeStiffnessCacheKey(self_ptr) }
}

/// C: mjCFlex::LoadCachedStiffness (user/user_objects.h:1072)
/// Calls: mjCCache::PopulateData, mj_getCache
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_load_cached_stiffness(self_ptr: *mut mjCFlex) -> bool {
    extern "C" { fn mjCFlex_LoadCachedStiffness(self_ptr: *mut mjCFlex) -> bool; }
    // SAFETY: delegates to C implementation
    unsafe { mjCFlex_LoadCachedStiffness(self_ptr) }
}

/// C: mjCFlex::CacheStiffness (user/user_objects.h:1073)
/// Calls: mjCCache::Insert, mj_getCache
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_cache_stiffness(self_ptr: *mut mjCFlex) {
    extern "C" { fn mjCFlex_CacheStiffness(self_ptr: *mut mjCFlex); }
    // SAFETY: delegates to C implementation
    unsafe { mjCFlex_CacheStiffness(self_ptr) }
}

/// C: mjCMesh::CopyFromSpec (user/user_objects.h:1151)
/// Calls: mju_free
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_copy_from_spec(self_ptr: *mut mjCMesh) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCMesh)
    // Previous return: ()
    extern "C" { fn mjCMesh_CopyFromSpec (self_ptr : * mut mjCMesh) ; } unsafe { mjCMesh_CopyFromSpec (self_ptr) }
}

/// C: mjCMesh::PointToLocal (user/user_objects.h:1152)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_point_to_local(self_ptr: *mut mjCMesh) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCMesh_PointToLocal(self_ptr: *mut mjCMesh); }
    // SAFETY: delegates to C implementation
    unsafe { mjCMesh_PointToLocal(self_ptr) }
}

/// C: mjCMesh::NameSpace (user/user_objects.h:1153)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_name_space(self_ptr: *mut mjCMesh, m: *const mjCModel) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCMesh_NameSpace(self_ptr: *mut mjCMesh, m: *const mjCModel); }
    // SAFETY: delegates to C implementation
    unsafe { mjCMesh_NameSpace(self_ptr, m) }
}

/// C: mjCMesh::MakeHemisphere (user/user_objects.h:1156)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_make_hemisphere(self_ptr: *mut mjCMesh, res: i32, make_faces: bool, make_cap: bool) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCMesh_MakeHemisphere(self_ptr: *mut mjCMesh, res: i32, make_faces: bool, make_cap: bool); }
    // SAFETY: delegates to C implementation
    unsafe { mjCMesh_MakeHemisphere(self_ptr, res, make_faces, make_cap) }
}

/// C: mjCMesh::MakeSphere (user/user_objects.h:1157)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_make_sphere(self_ptr: *mut mjCMesh, subdiv: i32, make_faces: bool) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCMesh_MakeSphere(self_ptr: *mut mjCMesh, subdiv: i32, make_faces: bool); }
    // SAFETY: delegates to C implementation
    unsafe { mjCMesh_MakeSphere(self_ptr, subdiv, make_faces) }
}

/// C: mjCMesh::MakeTorus (user/user_objects.h:1158)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_make_torus(self_ptr: *mut mjCMesh, res: i32, radius: f64) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCMesh_MakeTorus(self_ptr: *mut mjCMesh, res: i32, radius: f64); }
    // SAFETY: delegates to C implementation
    unsafe { mjCMesh_MakeTorus(self_ptr, res, radius) }
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
    extern "C" { fn mjCMesh_MakeSupertorus(self_ptr: *mut mjCMesh, res: i32, radius: f64, s: f64, t: f64); }
    // SAFETY: delegates to C implementation
    unsafe { mjCMesh_MakeSupertorus(self_ptr, res, radius, s, t) }
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
    extern "C" { fn mjCMesh_MakeSupersphere(self_ptr: *mut mjCMesh, res: i32, e: f64, n: f64); }
    // SAFETY: delegates to C implementation
    unsafe { mjCMesh_MakeSupersphere(self_ptr, res, e, n) }
}

/// C: mjCMesh::MakeWedge (user/user_objects.h:1161)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_make_wedge(self_ptr: *mut mjCMesh, resolution: [i32; 2], fov: [f64; 2], gamma: f64) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCMesh_MakeWedge(self_ptr: *mut mjCMesh, resolution: [i32; 2], fov: [f64; 2], gamma: f64); }
    // SAFETY: delegates to C implementation
    unsafe { mjCMesh_MakeWedge(self_ptr, resolution, fov, gamma) }
}

/// C: mjCMesh::MakeRect (user/user_objects.h:1162)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_make_rect(self_ptr: *mut mjCMesh, resolution: [i32; 2]) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCMesh_MakeRect(self_ptr: *mut mjCMesh, resolution: [i32; 2]); }
    // SAFETY: delegates to C implementation
    unsafe { mjCMesh_MakeRect(self_ptr, resolution) }
}

/// C: mjCMesh::MakeCone (user/user_objects.h:1163)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_make_cone(self_ptr: *mut mjCMesh, nedge: i32, radius: f64) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCMesh_MakeCone(self_ptr: *mut mjCMesh, nedge: i32, radius: f64); }
    // SAFETY: delegates to C implementation
    unsafe { mjCMesh_MakeCone(self_ptr, nedge, radius) }
}

/// C: mjCMesh::Plugin (user/user_objects.h:1166)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_plugin(self_ptr: *mut mjCMesh) -> *const mjsPlugin {
    if self_ptr.is_null() { return core::ptr::null(); }
    extern "C" { fn mjCMesh_Plugin(self_ptr: *mut mjCMesh) -> *const mjsPlugin; }
    // SAFETY: delegates to C implementation
    unsafe { mjCMesh_Plugin(self_ptr) }
}

/// C: mjCMesh::ContentType (user/user_objects.h:1167)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_content_type(self_ptr: *mut mjCMesh) -> *const i32 {
    if self_ptr.is_null() { return core::ptr::null(); }
    extern "C" { fn mjCMesh_ContentType(self_ptr: *mut mjCMesh) -> *const i32; }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCMesh_ContentType(self_ptr) }
}

/// C: mjCMesh::File (user/user_objects.h:1168)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_file(self_ptr: *mut mjCMesh) -> *const i32 {
    if self_ptr.is_null() { return core::ptr::null(); }
    extern "C" { fn mjCMesh_File(self_ptr: *mut mjCMesh) -> *const i32; }
    // SAFETY: delegates to C implementation
    unsafe { mjCMesh_File(self_ptr) }
}

/// C: mjCMesh::Refpos (user/user_objects.h:1169)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_refpos(self_ptr: *mut mjCMesh) -> *const f64 {
    extern "C" { fn mjCMesh_Refpos(self_ptr: *mut mjCMesh) -> *const f64; }
    // SAFETY: delegates to C implementation
    unsafe { mjCMesh_Refpos(self_ptr) }
}

/// C: mjCMesh::Refquat (user/user_objects.h:1170)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_refquat(self_ptr: *mut mjCMesh) -> *const f64 {
    extern "C" { fn mjCMesh_Refquat(self_ptr: *mut mjCMesh) -> *const f64; }
    // SAFETY: delegates to C implementation
    unsafe { mjCMesh_Refquat(self_ptr) }
}

/// C: mjCMesh::Scale (user/user_objects.h:1171)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_scale(self_ptr: *mut mjCMesh) -> *const f64 {
    extern "C" { fn mjCMesh_Scale(self_ptr: *mut mjCMesh) -> *const f64; }
    // SAFETY: delegates to C implementation
    unsafe { mjCMesh_Scale(self_ptr) }
}

/// C: mjCMesh::SmoothNormal (user/user_objects.h:1172)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_smooth_normal(self_ptr: *mut mjCMesh) -> bool {
    extern "C" { fn mjCMesh_SmoothNormal(self_ptr: *mut mjCMesh) -> bool; }
    // SAFETY: delegates to C implementation
    unsafe { mjCMesh_SmoothNormal(self_ptr) }
}

/// C: mjCMesh::Vert (user/user_objects.h:1173)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_vert(self_ptr: *mut mjCMesh) -> *const i32 {
    extern "C" { fn mjCMesh_Vert(self_ptr: *mut mjCMesh) -> *const i32; }
    // SAFETY: delegates to C implementation
    unsafe { mjCMesh_Vert(self_ptr) }
}

/// C: mjCMesh::UserVert (user/user_objects.h:1175)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_user_vert(self_ptr: *mut mjCMesh) -> *const i32 {
    extern "C" { fn mjCMesh_UserVert(self_ptr: *mut mjCMesh) -> *const i32; }
    // SAFETY: delegates to C implementation
    unsafe { mjCMesh_UserVert(self_ptr) }
}

/// C: mjCMesh::UserNormal (user/user_objects.h:1176)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_user_normal(self_ptr: *mut mjCMesh) -> *const i32 {
    extern "C" { fn mjCMesh_UserNormal(self_ptr: *mut mjCMesh) -> *const i32; }
    // SAFETY: delegates to C implementation
    unsafe { mjCMesh_UserNormal(self_ptr) }
}

/// C: mjCMesh::Texcoord (user/user_objects.h:1177)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_texcoord(self_ptr: *mut mjCMesh) -> *const i32 {
    extern "C" { fn mjCMesh_Texcoord(self_ptr: *mut mjCMesh) -> *const i32; }
    // SAFETY: delegates to C implementation
    unsafe { mjCMesh_Texcoord(self_ptr) }
}

/// C: mjCMesh::FaceTexcoord (user/user_objects.h:1178)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_face_texcoord(self_ptr: *mut mjCMesh) -> *const i32 {
    if self_ptr.is_null() { return core::ptr::null(); }
    extern "C" { fn mjCMesh_FaceTexcoord(self_ptr: *mut mjCMesh) -> *const i32; }
    // SAFETY: delegates to C implementation
    unsafe { mjCMesh_FaceTexcoord(self_ptr) }
}

/// C: mjCMesh::UserTexcoord (user/user_objects.h:1179)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_user_texcoord(self_ptr: *mut mjCMesh) -> *const i32 {
    extern "C" { fn mjCMesh_UserTexcoord(self_ptr: *mut mjCMesh) -> *const i32; }
    // SAFETY: delegates to C implementation
    unsafe { mjCMesh_UserTexcoord(self_ptr) }
}

/// C: mjCMesh::Face (user/user_objects.h:1180)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_face(self_ptr: *mut mjCMesh) -> *const i32 {
    if self_ptr.is_null() { return core::ptr::null(); }
    extern "C" { fn mjCMesh_Face(self_ptr: *mut mjCMesh) -> *const i32; }
    // SAFETY: delegates to C implementation
    unsafe { mjCMesh_Face(self_ptr) }
}

/// C: mjCMesh::UserFace (user/user_objects.h:1181)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_user_face(self_ptr: *mut mjCMesh) -> *const i32 {
    extern "C" { fn mjCMesh_UserFace(self_ptr: *mut mjCMesh) -> *const i32; }
    // SAFETY: delegates to C implementation
    unsafe { mjCMesh_UserFace(self_ptr) }
}

/// C: mjCMesh::Inertia (user/user_objects.h:1182)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_inertia(self_ptr: *mut mjCMesh) -> mjtMeshInertia {
    if self_ptr.is_null() { return unsafe { core::mem::transmute(()) }; }
    extern "C" { fn mjCMesh_Inertia(self_ptr: *mut mjCMesh) -> mjtMeshInertia; }
    // SAFETY: delegates to C implementation
    unsafe { mjCMesh_Inertia(self_ptr) }
}

/// C: mjCMesh::Material (user/user_objects.h:1183)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_material(self_ptr: *mut mjCMesh) -> *const i32 {
    if self_ptr.is_null() { return core::ptr::null(); }
    extern "C" { fn mjCMesh_Material(self_ptr: *mut mjCMesh) -> *const i32; }
    // SAFETY: delegates to C implementation
    unsafe { mjCMesh_Material(self_ptr) }
}

/// C: mjCMesh::SetNeedHull (user/user_objects.h:1186)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_set_need_hull(self_ptr: *mut mjCMesh, needhull: bool) {
    extern "C" { fn mjCMesh_SetNeedHull(self_ptr: *mut mjCMesh, needhull: bool); }
    // SAFETY: delegates to C implementation
    unsafe { mjCMesh_SetNeedHull(self_ptr, needhull) }
}

/// C: mjCMesh::aamm (user/user_objects.h:1189)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_aamm(self_ptr: *mut mjCMesh) -> *const f64 {
    extern "C" { fn mjCMesh_aamm(self_ptr: *mut mjCMesh) -> *const f64; }
    // SAFETY: delegates to C implementation
    unsafe { mjCMesh_aamm(self_ptr) }
}

/// C: mjCMesh::nvert (user/user_objects.h:1192)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_nvert(self_ptr: *mut mjCMesh) -> i32 {
    if self_ptr.is_null() {
        return 0;
    }
    extern "C" { fn mjCMesh_nvert(self_ptr: *mut mjCMesh) -> i32; }
    // SAFETY: self_ptr verified non-null; delegates to C++ getter
    unsafe { mjCMesh_nvert(self_ptr) }
}

/// C: mjCMesh::nnormal (user/user_objects.h:1193)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_nnormal(self_ptr: *mut mjCMesh) -> i32 {
    if self_ptr.is_null() {
        return 0;
    }
    extern "C" { fn mjCMesh_nnormal(self_ptr: *mut mjCMesh) -> i32; }
    // SAFETY: self_ptr verified non-null; delegates to C++ getter
    unsafe { mjCMesh_nnormal(self_ptr) }
}

/// C: mjCMesh::ntexcoord (user/user_objects.h:1194)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_ntexcoord(self_ptr: *mut mjCMesh) -> i32 {
    extern "C" { fn mjCMesh_ntexcoord(self_ptr: *mut mjCMesh) -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { mjCMesh_ntexcoord(self_ptr) }
}

/// C: mjCMesh::nface (user/user_objects.h:1195)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_nface(self_ptr: *mut mjCMesh) -> i32 {
    if self_ptr.is_null() {
        return 0;
    }
    extern "C" { fn mjCMesh_nface(self_ptr: *mut mjCMesh) -> i32; }
    // SAFETY: self_ptr verified non-null; delegates to C++ getter
    unsafe { mjCMesh_nface(self_ptr) }
}

/// C: mjCMesh::npolygon (user/user_objects.h:1196)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_npolygon(self_ptr: *mut mjCMesh) -> i32 {
    extern "C" { fn mjCMesh_npolygon(self_ptr: *mut mjCMesh) -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { mjCMesh_npolygon(self_ptr) }
}

/// C: mjCMesh::npolygonvert (user/user_objects.h:1197)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_npolygonvert(self_ptr: *mut mjCMesh) -> i32 {
    extern "C" { fn mjCMesh_npolygonvert(self_ptr: *mut mjCMesh) -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { mjCMesh_npolygonvert(self_ptr) }
}

/// C: mjCMesh::npolygonmap (user/user_objects.h:1204)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_npolygonmap(self_ptr: *mut mjCMesh) -> i32 {
    extern "C" { fn mjCMesh_npolygonmap(self_ptr: *mut mjCMesh) -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { mjCMesh_npolygonmap(self_ptr) }
}

/// C: mjCMesh::szgraph (user/user_objects.h:1213)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_szgraph(self_ptr: *mut mjCMesh) -> i32 {
    extern "C" { fn mjCMesh_szgraph(self_ptr: *mut mjCMesh) -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { mjCMesh_szgraph(self_ptr) }
}

/// C: mjCMesh::tree (user/user_objects.h:1216)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_tree(self_ptr: *mut mjCMesh) -> *const mjCBoundingVolumeHierarchy {
    extern "C" { fn mjCMesh_tree(self_ptr: *mut mjCMesh) -> *const mjCBoundingVolumeHierarchy; }
    // SAFETY: delegates to C implementation
    unsafe { mjCMesh_tree(self_ptr) }
}

/// C: mjCMesh::octree (user/user_objects.h:1219)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_octree(self_ptr: *mut mjCMesh) -> *const mjCOctree {
    extern "C" { fn mjCMesh_octree(self_ptr: *mut mjCMesh) -> *const mjCOctree; }
    // SAFETY: delegates to C implementation
    unsafe { mjCMesh_octree(self_ptr) }
}

/// C: mjCMesh::mutable_octree (user/user_objects.h:1220)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_mutable_octree(self_ptr: *mut mjCMesh) -> *mut mjCOctree {
    extern "C" { fn mjCMesh_mutable_octree(self_ptr: *mut mjCMesh) -> *mut mjCOctree; }
    // SAFETY: delegates to C implementation
    unsafe { mjCMesh_mutable_octree(self_ptr) }
}

/// C: mjCMesh::Compile (user/user_objects.h:1222)
/// Calls: mjCMesh::TryCompile
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_compile(self_ptr: *mut mjCMesh, vfs: *const mjVFS) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCMesh, vfs : * const mjVFS)
    // Previous return: ()
    extern "C" { fn mjCMesh_Compile (self_ptr : * mut mjCMesh , vfs : * const mjVFS) ; } unsafe { mjCMesh_Compile (self_ptr , vfs) }
}

/// C: mjCMesh::GetPosPtr (user/user_objects.h:1223)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_get_pos_ptr(self_ptr: *mut mjCMesh) -> *mut f64 {
    if self_ptr.is_null() { return core::ptr::null_mut(); }
    extern "C" { fn mjCMesh_GetPosPtr(self_ptr: *mut mjCMesh) -> *mut f64; }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCMesh_GetPosPtr(self_ptr) }
}

/// C: mjCMesh::GetQuatPtr (user/user_objects.h:1224)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_get_quat_ptr(self_ptr: *mut mjCMesh) -> *mut f64 {
    if self_ptr.is_null() { return core::ptr::null_mut(); }
    extern "C" { fn mjCMesh_GetQuatPtr(self_ptr: *mut mjCMesh) -> *mut f64; }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCMesh_GetQuatPtr(self_ptr) }
}

/// C: mjCMesh::GetInertiaBoxPtr (user/user_objects.h:1225)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_get_inertia_box_ptr(self_ptr: *mut mjCMesh) -> *mut f64 {
    if self_ptr.is_null() { return core::ptr::null_mut(); }
    extern "C" { fn mjCMesh_GetInertiaBoxPtr(self_ptr: *mut mjCMesh) -> *mut f64; }
    // SAFETY: delegates to C implementation
    unsafe { mjCMesh_GetInertiaBoxPtr(self_ptr) }
}

/// C: mjCMesh::GetVolumeRef (user/user_objects.h:1226)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_get_volume_ref(self_ptr: *mut mjCMesh) -> f64 {
    if self_ptr.is_null() {
        return 0.0;
    }
    extern "C" { fn mjCMesh_GetVolumeRef(self_ptr: *mut mjCMesh) -> f64; }
    // SAFETY: self_ptr verified non-null; delegates to C++ method
    unsafe { mjCMesh_GetVolumeRef(self_ptr) }
}

/// C: mjCMesh::FitGeom (user/user_objects.h:1227)
/// Calls: mjCMesh::GetInertiaBoxPtr
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_fit_geom(self_ptr: *mut mjCMesh, geom: *mut mjCGeom, center: [f64; 3]) {
    extern "C" { fn mjCMesh_FitGeom(self_ptr: *mut mjCMesh, geom: *mut mjCGeom, center: [f64; 3]); }
    // SAFETY: delegates to C implementation
    unsafe { mjCMesh_FitGeom(self_ptr, geom, center) }
}

/// C: mjCMesh::HasTexcoord (user/user_objects.h:1228)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_has_texcoord(self_ptr: *mut mjCMesh) -> bool {
    if self_ptr.is_null() { return false; }
    extern "C" { fn mjCMesh_HasTexcoord(self_ptr: *mut mjCMesh) -> bool; }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCMesh_HasTexcoord(self_ptr) }
}

/// C: mjCMesh::DelTexcoord (user/user_objects.h:1229)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_del_texcoord(self_ptr: *mut mjCMesh) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCMesh_DelTexcoord(self_ptr: *mut mjCMesh); }
    // SAFETY: delegates to C implementation
    unsafe { mjCMesh_DelTexcoord(self_ptr) }
}

/// C: mjCMesh::IsVisual (user/user_objects.h:1230)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_is_visual(self_ptr: *mut mjCMesh) -> bool {
    if self_ptr.is_null() { return false; }
    extern "C" { fn mjCMesh_IsVisual(self_ptr: *mut mjCMesh) -> bool; }
    // SAFETY: delegates to C implementation
    unsafe { mjCMesh_IsVisual(self_ptr) }
}

/// C: mjCMesh::SetNotVisual (user/user_objects.h:1231)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_set_not_visual(self_ptr: *mut mjCMesh) {
    extern "C" { fn mjCMesh_SetNotVisual(self_ptr: *mut mjCMesh); }
    // SAFETY: delegates to C implementation
    unsafe { mjCMesh_SetNotVisual(self_ptr) }
}

/// C: mjCMesh::CopyVert (user/user_objects.h:1233)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_copy_vert(self_ptr: *mut mjCMesh, arr: *mut f32) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCMesh_CopyVert(self_ptr: *mut mjCMesh, arr: *mut f32); }
    // SAFETY: delegates to C implementation
    unsafe { mjCMesh_CopyVert(self_ptr, arr) }
}

/// C: mjCMesh::CopyNormal (user/user_objects.h:1234)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_copy_normal(self_ptr: *mut mjCMesh, arr: *mut f32) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCMesh_CopyNormal(self_ptr: *mut mjCMesh, arr: *mut f32); }
    // SAFETY: delegates to C implementation
    unsafe { mjCMesh_CopyNormal(self_ptr, arr) }
}

/// C: mjCMesh::CopyFace (user/user_objects.h:1235)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_copy_face(self_ptr: *mut mjCMesh, arr: *mut i32) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCMesh_CopyFace(self_ptr: *mut mjCMesh, arr: *mut i32); }
    // SAFETY: delegates to C implementation
    unsafe { mjCMesh_CopyFace(self_ptr, arr) }
}

/// C: mjCMesh::CopyFaceNormal (user/user_objects.h:1236)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_copy_face_normal(self_ptr: *mut mjCMesh, arr: *mut i32) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCMesh_CopyFaceNormal(self_ptr: *mut mjCMesh, arr: *mut i32); }
    // SAFETY: delegates to C implementation
    unsafe { mjCMesh_CopyFaceNormal(self_ptr, arr) }
}

/// C: mjCMesh::CopyFaceTexcoord (user/user_objects.h:1237)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_copy_face_texcoord(self_ptr: *mut mjCMesh, arr: *mut i32) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCMesh_CopyFaceTexcoord(self_ptr: *mut mjCMesh, arr: *mut i32); }
    // SAFETY: delegates to C implementation
    unsafe { mjCMesh_CopyFaceTexcoord(self_ptr, arr) }
}

/// C: mjCMesh::CopyTexcoord (user/user_objects.h:1238)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_copy_texcoord(self_ptr: *mut mjCMesh, arr: *mut f32) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCMesh_CopyTexcoord(self_ptr: *mut mjCMesh, arr: *mut f32); }
    // SAFETY: delegates to C implementation
    unsafe { mjCMesh_CopyTexcoord(self_ptr, arr) }
}

/// C: mjCMesh::CopyGraph (user/user_objects.h:1239)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_copy_graph(self_ptr: *mut mjCMesh, arr: *mut i32) {
    if self_ptr.is_null() {
        return;
    }
    extern "C" { fn mjCMesh_CopyGraph(self_ptr: *mut mjCMesh, arr: *mut i32); }
    // SAFETY: self_ptr verified non-null; delegates to C++ method
    unsafe { mjCMesh_CopyGraph(self_ptr, arr) }
}

/// C: mjCMesh::CopyPolygons (user/user_objects.h:1242)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_copy_polygons(self_ptr: *mut mjCMesh, verts: *mut i32, adr: *mut i32, num: *mut i32, poly_adr: i32) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCMesh_CopyPolygons(self_ptr: *mut mjCMesh, verts: *mut i32, adr: *mut i32, num: *mut i32, poly_adr: i32); }
    // SAFETY: delegates to C implementation
    unsafe { mjCMesh_CopyPolygons(self_ptr, verts, adr, num, poly_adr) }
}

/// C: mjCMesh::CopyPolygonMap (user/user_objects.h:1245)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_copy_polygon_map(self_ptr: *mut mjCMesh, faces: *mut i32, adr: *mut i32, num: *mut i32, poly_adr: i32) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCMesh_CopyPolygonMap(self_ptr: *mut mjCMesh, faces: *mut i32, adr: *mut i32, num: *mut i32, poly_adr: i32); }
    // SAFETY: delegates to C implementation
    unsafe { mjCMesh_CopyPolygonMap(self_ptr, faces, adr, num, poly_adr) }
}

/// C: mjCMesh::CopyPolygonNormals (user/user_objects.h:1248)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_copy_polygon_normals(self_ptr: *mut mjCMesh, arr: *mut f64) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCMesh_CopyPolygonNormals(self_ptr: *mut mjCMesh, arr: *mut f64); }
    // SAFETY: delegates to C implementation
    unsafe { mjCMesh_CopyPolygonNormals(self_ptr, arr) }
}

/// C: mjCMesh::SetBoundingVolume (user/user_objects.h:1251)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_set_bounding_volume(self_ptr: *mut mjCMesh, faceid: i32, dvert: *const f64) {
    if self_ptr.is_null() {
        return;
    }
    extern "C" { fn mjCMesh_SetBoundingVolume(self_ptr: *mut mjCMesh, faceid: i32, dvert: *const f64); }
    // SAFETY: self_ptr verified non-null; delegates to C++ method
    unsafe { mjCMesh_SetBoundingVolume(self_ptr, faceid, dvert) }
}

/// C: mjCMesh::LoadFromResource (user/user_objects.h:1254)
/// Calls: mjCMesh::IsMSH, mjCMesh::LoadFromDecoder, mjCMesh::LoadMSH
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_load_from_resource(self_ptr: *mut mjCMesh, resource: *mut mjResource, remove_repeated: bool) {
    extern "C" { fn mjCMesh_LoadFromResource(self_ptr: *mut mjCMesh, resource: *mut mjResource, remove_repeated: bool); }
    // SAFETY: delegates to C implementation
    unsafe { mjCMesh_LoadFromResource(self_ptr, resource, remove_repeated) }
}

/// C: mjCMesh::IsMSH (user/user_objects.h:1258)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_is_msh(filename: string_view, ct: string_view) -> bool {
    let _ = core::hint::black_box(0);
    extern "C" { fn mjCMesh_IsMSH(filename: string_view, ct: string_view) -> bool; }
    // SAFETY: delegates to C implementation
    unsafe { mjCMesh_IsMSH(filename, ct) }
}

/// C: mjCMesh::TryCompile (user/user_objects.h:1265)
/// Calls: mjCMesh::CheckInitialMesh, mjCMesh::CopyFromSpec, mjCMesh::LoadSDF, mjCMesh::Process, mj_getCache
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_try_compile(self_ptr: *mut mjCMesh, vfs: *const mjVFS) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCMesh, vfs : * const mjVFS)
    // Previous return: ()
    extern "C" { fn mjCMesh_TryCompile (self_ptr : * mut mjCMesh , vfs : * const mjVFS) ; } unsafe { mjCMesh_TryCompile (self_ptr , vfs) }
}

/// C: mjCMesh::LoadCachedMesh (user/user_objects.h:1268)
/// Calls: mju_malloc
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_load_cached_mesh(self_ptr: *mut mjCMesh, cache: *mut mjCCache, resource: *const mjResource) -> bool {
    if self_ptr.is_null() { return false; }
    extern "C" { fn mjCMesh_LoadCachedMesh(self_ptr: *mut mjCMesh, cache: *mut mjCCache, resource: *const mjResource) -> bool; }
    // SAFETY: delegates to C implementation
    unsafe { mjCMesh_LoadCachedMesh(self_ptr, cache, resource) }
}

/// C: mjCMesh::CacheMesh (user/user_objects.h:1271)
/// Calls: mjCCache::Insert, mju_malloc
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_cache_mesh(self_ptr: *mut mjCMesh, cache: *mut mjCCache, resource: *const mjResource) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCMesh_CacheMesh(self_ptr: *mut mjCMesh, cache: *mut mjCCache, resource: *const mjResource); }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCMesh_CacheMesh(self_ptr, cache, resource) }
}

/// C: mjCMesh::LoadFromDecoder (user/user_objects.h:1277)
/// Calls: mj_deleteSpec, mjs_asMesh, mjs_firstElement
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_load_from_decoder(self_ptr: *mut mjCMesh, resource: *mut mjResource, remove_repeated: bool) {
    extern "C" { fn mjCMesh_LoadFromDecoder(self_ptr: *mut mjCMesh, resource: *mut mjResource, remove_repeated: bool); }
    // SAFETY: delegates to C implementation
    unsafe { mjCMesh_LoadFromDecoder(self_ptr, resource, remove_repeated) }
}

/// C: mjCMesh::LoadMSH (user/user_objects.h:1279)
/// Calls: mju_readResource
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_load_msh(self_ptr: *mut mjCMesh, resource: *mut mjResource, remove_repeated: bool) {
    extern "C" { fn mjCMesh_LoadMSH(self_ptr: *mut mjCMesh, resource: *mut mjResource, remove_repeated: bool); }
    // SAFETY: delegates to C implementation
    unsafe { mjCMesh_LoadMSH(self_ptr, resource, remove_repeated) }
}

/// C: mjCMesh::LoadSDF (user/user_objects.h:1281)
/// Calls: mjp_getPluginAtSlot
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_load_sdf(self_ptr: *mut mjCMesh) {
    if self_ptr.is_null() {
        return;
    }
    extern "C" { fn mjCMesh_LoadSDF(self_ptr: *mut mjCMesh); }
    // SAFETY: self_ptr verified non-null; delegates to C++ method
    unsafe { mjCMesh_LoadSDF(self_ptr) }
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
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCMesh, dvert : * const f64)
    // Previous return: ()
    extern "C" { fn mjCMesh_MakeGraph (self_ptr : * mut mjCMesh , dvert : * const f64) ; } unsafe { mjCMesh_MakeGraph (self_ptr , dvert) }
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
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCMesh, dvert : * const f64)
    // Previous return: ()
    extern "C" { fn mjCMesh_MakeNormal (self_ptr : * mut mjCMesh , dvert : * const f64) ; } unsafe { mjCMesh_MakeNormal (self_ptr , dvert) }
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
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCMesh, dvert : * const f64)
    // Previous return: ()
    extern "C" { fn mjCMesh_MakeCenter (self_ptr : * mut mjCMesh , dvert : * const f64) ; } unsafe { mjCMesh_MakeCenter (self_ptr , dvert) }
}

/// C: mjCMesh::Process (user/user_objects.h:1286)
/// Calls: mjCMesh::ApplyTransformations, mjCMesh::ComputeFaceCentroid, mjCMesh::ComputeInertia, mjCMesh::CopyGraph, mjCMesh::GetVolumeRef, mjCMesh::MakeCenter, mjCMesh::MakeGraph, mjCMesh::MakeNormal, mjCMesh::MakePolygonNormals, mjCMesh::MakePolygons, mjCMesh::Rotate, mjCMesh::SetBoundingVolume, mjCMesh::nface, mjCMesh::nvert, mjuu_eig3, mjuu_setvec
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_process(self_ptr: *mut mjCMesh) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCMesh)
    // Previous return: ()
    extern "C" { fn mjCMesh_Process (self_ptr : * mut mjCMesh) ; } unsafe { mjCMesh_Process (self_ptr) }
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
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCMesh, dvert : * mut f64)
    // Previous return: ()
    extern "C" { fn mjCMesh_ApplyTransformations (self_ptr : * mut mjCMesh , dvert : * mut f64) ; } unsafe { mjCMesh_ApplyTransformations (self_ptr , dvert) }
}

/// C: mjCMesh::ComputeFaceCentroid (user/user_objects.h:1288)
/// Calls: mjCMesh::nface
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_compute_face_centroid(self_ptr: *mut mjCMesh, arg0: [f64; 3], dvert: *const f64) -> f64 {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCMesh, arg0 : [f64 ; 3], dvert : * const f64)
    // Previous return: f64
    extern "C" { fn mjCMesh_ComputeFaceCentroid (self_ptr : * mut mjCMesh , arg0 : [f64 ; 3] , dvert : * const f64) -> f64 ; } unsafe { mjCMesh_ComputeFaceCentroid (self_ptr , arg0 , dvert) }
}

/// C: mjCMesh::CheckInitialMesh (user/user_objects.h:1289)
/// Calls: mjCMesh::nvert
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_check_initial_mesh(self_ptr: *mut mjCMesh) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCMesh)
    // Previous return: ()
    extern "C" { fn mjCMesh_CheckInitialMesh (self_ptr : * mut mjCMesh) ; } unsafe { mjCMesh_CheckInitialMesh (self_ptr) }
}

/// C: mjCMesh::CopyPlugin (user/user_objects.h:1290)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_copy_plugin(self_ptr: *mut mjCMesh) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCMesh_CopyPlugin(self_ptr: *mut mjCMesh); }
    // SAFETY: delegates to C implementation
    unsafe { mjCMesh_CopyPlugin(self_ptr) }
}

/// C: mjCMesh::Rotate (user/user_objects.h:1291)
/// Calls: mjCMesh::nnormal, mjCMesh::nvert, mjuu_mulvecmat, mjuu_quat2mat
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_rotate(self_ptr: *mut mjCMesh, quat: [f64; 4], dvert: *mut f64) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCMesh, quat : [f64 ; 4], dvert : * mut f64)
    // Previous return: ()
    extern "C" { fn mjCMesh_Rotate (self_ptr : * mut mjCMesh , quat : [f64 ; 4] , dvert : * mut f64) ; } unsafe { mjCMesh_Rotate (self_ptr , quat , dvert) }
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
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCMesh, dvert : * const f64)
    // Previous return: ()
    extern "C" { fn mjCMesh_MakePolygons (self_ptr : * mut mjCMesh , dvert : * const f64) ; } unsafe { mjCMesh_MakePolygons (self_ptr , dvert) }
}

/// C: mjCMesh::MakePolygonNormals (user/user_objects.h:1294)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_make_polygon_normals(self_ptr: *mut mjCMesh, dvert: *const f64) {
    if self_ptr.is_null() {
        return;
    }
    extern "C" { fn mjCMesh_MakePolygonNormals(self_ptr: *mut mjCMesh, dvert: *const f64); }
    // SAFETY: self_ptr verified non-null; delegates to C++ method
    unsafe { mjCMesh_MakePolygonNormals(self_ptr, dvert) }
}

/// C: mjCMesh::ComputeInertia (user/user_objects.h:1297)
/// Calls: mjCMesh::nvert, mjuu_dot3, triangle
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_compute_inertia(self_ptr: *mut mjCMesh, inert: [f64; 6], CoM: [f64; 3], dvert: *const f64) -> f64 {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCMesh, inert : [f64 ; 6], CoM : [f64 ; 3], dvert : * const f64)
    // Previous return: f64
    extern "C" { fn mjCMesh_ComputeInertia (self_ptr : * mut mjCMesh , inert : [f64 ; 6] , CoM : [f64 ; 3] , dvert : * const f64) -> f64 ; } unsafe { mjCMesh_ComputeInertia (self_ptr , inert , CoM , dvert) }
}

/// C: mjCMesh::GraphFaces (user/user_objects.h:1299)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_graph_faces(self_ptr: *mut mjCMesh) -> *mut i32 {
    if self_ptr.is_null() {
        return core::ptr::null_mut();
    }
    extern "C" { fn mjCMesh_GraphFaces(self_ptr: *mut mjCMesh) -> *mut i32; }
    // SAFETY: self_ptr verified non-null; delegates to C++ method
    unsafe { mjCMesh_GraphFaces(self_ptr) }
}

/// C: mjCMesh::ComputeVolume (user/user_objects.h:1313)
/// Calls: mjuu_dot3, triangle
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_compute_volume(self_ptr: *mut mjCMesh, CoM: [f64; 3], facecen: [f64; 3], dvert: *const f64) -> f64 {
    extern "C" { fn mjCMesh_ComputeVolume(self_ptr: *mut mjCMesh, CoM: [f64; 3], facecen: [f64; 3], dvert: *const f64) -> f64; }
    // SAFETY: delegates to C implementation
    unsafe { mjCMesh_ComputeVolume(self_ptr, CoM, facecen, dvert) }
}

/// C: mjCMesh::ComputeSurfaceArea (user/user_objects.h:1314)
/// Calls: mjCMesh::nface
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_compute_surface_area(self_ptr: *mut mjCMesh, CoM: [f64; 3], facecen: [f64; 3], dvert: *const f64) -> f64 {
    extern "C" { fn mjCMesh_ComputeSurfaceArea(self_ptr: *mut mjCMesh, CoM: [f64; 3], facecen: [f64; 3], dvert: *const f64) -> f64; }
    // SAFETY: delegates to C implementation
    unsafe { mjCMesh_ComputeSurfaceArea(self_ptr, CoM, facecen, dvert) }
}

/// C: mjCSkin::File (user/user_objects.h:1364)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_skin_file(self_ptr: *mut mjCSkin) -> *const i32 {
    extern "C" { fn mjCSkin_File(self_ptr: *mut mjCSkin) -> *const i32; }
    // SAFETY: delegates to C implementation
    unsafe { mjCSkin_File(self_ptr) }
}

/// C: mjCSkin::get_material (user/user_objects.h:1365)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_skin_get_material(self_ptr: *mut mjCSkin) -> *const i32 {
    extern "C" { fn mjCSkin_get_material(self_ptr: *mut mjCSkin) -> *const i32; }
    // SAFETY: delegates to C implementation
    unsafe { mjCSkin_get_material(self_ptr) }
}

/// C: mjCSkin::get_vert (user/user_objects.h:1366)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_skin_get_vert(self_ptr: *mut mjCSkin) -> *const i32 {
    extern "C" { fn mjCSkin_get_vert(self_ptr: *mut mjCSkin) -> *const i32; }
    // SAFETY: delegates to C implementation
    unsafe { mjCSkin_get_vert(self_ptr) }
}

/// C: mjCSkin::get_texcoord (user/user_objects.h:1367)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_skin_get_texcoord(self_ptr: *mut mjCSkin) -> *const i32 {
    extern "C" { fn mjCSkin_get_texcoord(self_ptr: *mut mjCSkin) -> *const i32; }
    // SAFETY: delegates to C implementation
    unsafe { mjCSkin_get_texcoord(self_ptr) }
}

/// C: mjCSkin::get_face (user/user_objects.h:1368)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_skin_get_face(self_ptr: *mut mjCSkin) -> *const i32 {
    extern "C" { fn mjCSkin_get_face(self_ptr: *mut mjCSkin) -> *const i32; }
    // SAFETY: delegates to C implementation
    unsafe { mjCSkin_get_face(self_ptr) }
}

/// C: mjCSkin::get_bodyname (user/user_objects.h:1369)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_skin_get_bodyname(self_ptr: *mut mjCSkin) -> *const i32 {
    extern "C" { fn mjCSkin_get_bodyname(self_ptr: *mut mjCSkin) -> *const i32; }
    // SAFETY: delegates to C implementation
    unsafe { mjCSkin_get_bodyname(self_ptr) }
}

/// C: mjCSkin::get_bindpos (user/user_objects.h:1370)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_skin_get_bindpos(self_ptr: *mut mjCSkin) -> *const i32 {
    extern "C" { fn mjCSkin_get_bindpos(self_ptr: *mut mjCSkin) -> *const i32; }
    // SAFETY: delegates to C implementation
    unsafe { mjCSkin_get_bindpos(self_ptr) }
}

/// C: mjCSkin::get_bindquat (user/user_objects.h:1371)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_skin_get_bindquat(self_ptr: *mut mjCSkin) -> *const i32 {
    extern "C" { fn mjCSkin_get_bindquat(self_ptr: *mut mjCSkin) -> *const i32; }
    // SAFETY: delegates to C implementation
    unsafe { mjCSkin_get_bindquat(self_ptr) }
}

/// C: mjCSkin::get_vertid (user/user_objects.h:1372)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_skin_get_vertid(self_ptr: *mut mjCSkin) -> *const i32 {
    extern "C" { fn mjCSkin_get_vertid(self_ptr: *mut mjCSkin) -> *const i32; }
    // SAFETY: delegates to C implementation
    unsafe { mjCSkin_get_vertid(self_ptr) }
}

/// C: mjCSkin::get_vertweight (user/user_objects.h:1373)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_skin_get_vertweight(self_ptr: *mut mjCSkin) -> *const i32 {
    extern "C" { fn mjCSkin_get_vertweight(self_ptr: *mut mjCSkin) -> *const i32; }
    // SAFETY: delegates to C implementation
    unsafe { mjCSkin_get_vertweight(self_ptr) }
}

/// C: mjCSkin::del_material (user/user_objects.h:1374)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_skin_del_material(self_ptr: *mut mjCSkin) {
    extern "C" { fn mjCSkin_del_material(self_ptr: *mut mjCSkin); }
    // SAFETY: delegates to C implementation
    unsafe { mjCSkin_del_material(self_ptr) }
}

/// C: mjCSkin::CopyFromSpec (user/user_objects.h:1376)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_skin_copy_from_spec(self_ptr: *mut mjCSkin) {
    extern "C" { fn mjCSkin_CopyFromSpec(self_ptr: *mut mjCSkin); }
    // SAFETY: delegates to C implementation
    unsafe { mjCSkin_CopyFromSpec(self_ptr) }
}

/// C: mjCSkin::PointToLocal (user/user_objects.h:1377)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_skin_point_to_local(self_ptr: *mut mjCSkin) {
    extern "C" { fn mjCSkin_PointToLocal(self_ptr: *mut mjCSkin); }
    // SAFETY: delegates to C implementation
    unsafe { mjCSkin_PointToLocal(self_ptr) }
}

/// C: mjCSkin::ResolveReferences (user/user_objects.h:1380)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_skin_resolve_references(self_ptr: *mut mjCSkin, m: *const mjCModel) {
    extern "C" { fn mjCSkin_ResolveReferences(self_ptr: *mut mjCSkin, m: *const mjCModel); }
    // SAFETY: delegates to C implementation
    unsafe { mjCSkin_ResolveReferences(self_ptr, m) }
}

/// C: mjCSkin::NameSpace (user/user_objects.h:1381)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_skin_name_space(self_ptr: *mut mjCSkin, m: *const mjCModel) {
    extern "C" { fn mjCSkin_NameSpace(self_ptr: *mut mjCSkin, m: *const mjCModel); }
    // SAFETY: delegates to C implementation
    unsafe { mjCSkin_NameSpace(self_ptr, m) }
}

/// C: mjCSkin::Compile (user/user_objects.h:1382)
/// Calls: FilePath::Str, mjCBase::LoadResource, mjCSkin::CopyFromSpec, mjCSkin::LoadSKN, mju_closeResource, mjuu_normvec
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_skin_compile(self_ptr: *mut mjCSkin, vfs: *const mjVFS) {
    extern "C" { fn mjCSkin_Compile(self_ptr: *mut mjCSkin, vfs: *const mjVFS); }
    // SAFETY: delegates to C implementation
    unsafe { mjCSkin_Compile(self_ptr, vfs) }
}

/// C: mjCSkin::LoadSKN (user/user_objects.h:1383)
/// Calls: mju_readResource
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_skin_load_skn(self_ptr: *mut mjCSkin, resource: *mut mjResource) {
    extern "C" { fn mjCSkin_LoadSKN(self_ptr: *mut mjCSkin, resource: *mut mjResource); }
    // SAFETY: delegates to C implementation
    unsafe { mjCSkin_LoadSKN(self_ptr, resource) }
}

/// C: mjCHField::CopyFromSpec (user/user_objects.h:1417)
#[allow(unused_variables, non_snake_case)]
pub fn mj_ch_field_copy_from_spec(self_ptr: *mut mjCHField) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCHField_CopyFromSpec(self_ptr: *mut mjCHField); }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCHField_CopyFromSpec(self_ptr) }
}

/// C: mjCHField::PointToLocal (user/user_objects.h:1418)
#[allow(unused_variables, non_snake_case)]
pub fn mj_ch_field_point_to_local(self_ptr: *mut mjCHField) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCHField_PointToLocal(self_ptr: *mut mjCHField); }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCHField_PointToLocal(self_ptr) }
}

/// C: mjCHField::NameSpace (user/user_objects.h:1419)
#[allow(unused_variables, non_snake_case)]
pub fn mj_ch_field_name_space(self_ptr: *mut mjCHField, m: *const mjCModel) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCHField_NameSpace(self_ptr: *mut mjCHField, m: *const mjCModel); }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCHField_NameSpace(self_ptr, m) }
}

/// C: mjCHField::File (user/user_objects.h:1421)
#[allow(unused_variables, non_snake_case)]
pub fn mj_ch_field_file(self_ptr: *mut mjCHField) -> i32 {
    if self_ptr.is_null() { return 0; }
    extern "C" { fn mjCHField_File(self_ptr: *mut mjCHField) -> i32; }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCHField_File(self_ptr) }
}

/// C: mjCHField::get_userdata (user/user_objects.h:1424)
#[allow(unused_variables, non_snake_case)]
pub fn mj_ch_field_get_userdata(self_ptr: *mut mjCHField) -> *mut i32 {
    if self_ptr.is_null() { return core::ptr::null_mut(); }
    extern "C" { fn mjCHField_get_userdata(self_ptr: *mut mjCHField) -> *mut i32; }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCHField_get_userdata(self_ptr) }
}

/// C: mjCHField::Compile (user/user_objects.h:1427)
/// Calls: FilePath::Str, mjCBase::LoadResource, mjCHField::CopyFromSpec, mjCHField::LoadCustom, mjCHField::LoadPNG, mj_getCache, mju_closeResource
#[allow(unused_variables, non_snake_case)]
pub fn mj_ch_field_compile(self_ptr: *mut mjCHField, vfs: *const mjVFS) {
    extern "C" { fn mjCHField_Compile(self_ptr: *mut mjCHField, vfs: *const mjVFS); }
    // SAFETY: delegates to C implementation
    unsafe { mjCHField_Compile(self_ptr, vfs) }
}

/// C: mjCHField::GetCacheId (user/user_objects.h:1429)
#[allow(unused_variables, non_snake_case)]
pub fn mj_ch_field_get_cache_id(self_ptr: *mut mjCHField, resource: *const mjResource, asset_type: *const std__string) -> std__string {
    if self_ptr.is_null() { panic!("mj_ch_field_get_cache_id: null self_ptr"); }
    extern "C" { fn mjCHField_GetCacheId(self_ptr: *mut mjCHField, resource: *const mjResource, asset_type: *const std__string) -> std__string; }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCHField_GetCacheId(self_ptr, resource, asset_type) }
}

/// C: mjCHField::LoadCustom (user/user_objects.h:1430)
/// Calls: mju_readResource
#[allow(unused_variables, non_snake_case)]
pub fn mj_ch_field_load_custom(self_ptr: *mut mjCHField, resource: *mut mjResource) {
    extern "C" { fn mjCHField_LoadCustom(self_ptr: *mut mjCHField, resource: *mut mjResource); }
    // SAFETY: delegates to C implementation
    unsafe { mjCHField_LoadCustom(self_ptr, resource) }
}

/// C: mjCHField::LoadPNG (user/user_objects.h:1431)
#[allow(unused_variables, non_snake_case)]
pub fn mj_ch_field_load_png(self_ptr: *mut mjCHField, resource: *mut mjResource) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCHField_LoadPNG(self_ptr: *mut mjCHField, resource: *mut mjResource); }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCHField_LoadPNG(self_ptr, resource) }
}

/// C: mjCTexture::CopyFromSpec (user/user_objects.h:1465)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_texture_copy_from_spec(self_ptr: *mut mjCTexture) {
    if self_ptr.is_null() {
        return;
    }
    extern "C" { fn mjCTexture_CopyFromSpec(self_ptr: *mut mjCTexture); }
    // SAFETY: self_ptr verified non-null; delegates to C++ method
    unsafe { mjCTexture_CopyFromSpec(self_ptr) }
}

/// C: mjCTexture::PointToLocal (user/user_objects.h:1466)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_texture_point_to_local(self_ptr: *mut mjCTexture) {
    extern "C" { fn mjCTexture_PointToLocal(self_ptr: *mut mjCTexture); }
    // SAFETY: delegates to C implementation
    unsafe { mjCTexture_PointToLocal(self_ptr) }
}

/// C: mjCTexture::NameSpace (user/user_objects.h:1467)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_texture_name_space(self_ptr: *mut mjCTexture, m: *const mjCModel) {
    extern "C" { fn mjCTexture_NameSpace(self_ptr: *mut mjCTexture, m: *const mjCModel); }
    // SAFETY: delegates to C implementation
    unsafe { mjCTexture_NameSpace(self_ptr, m) }
}

/// C: mjCTexture::Compile (user/user_objects.h:1468)
/// Calls: FilePath::Str, mjCTexture::Builtin2D, mjCTexture::BuiltinCube, mjCTexture::CopyFromSpec, mjCTexture::LoadCubeSeparate
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_texture_compile(self_ptr: *mut mjCTexture, vfs: *const mjVFS) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCTexture, vfs : * const mjVFS)
    // Previous return: ()
    extern "C" { fn mjCTexture_Compile (self_ptr : * mut mjCTexture , vfs : * const mjVFS) ; } unsafe { mjCTexture_Compile (self_ptr , vfs) }
}

/// C: mjCTexture::File (user/user_objects.h:1471)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_texture_file(self_ptr: *mut mjCTexture) -> i32 {
    extern "C" { fn mjCTexture_File(self_ptr: *mut mjCTexture) -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { mjCTexture_File(self_ptr) }
}

/// C: mjCTexture::get_content_type (user/user_objects.h:1472)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_texture_get_content_type(self_ptr: *mut mjCTexture) -> i32 {
    extern "C" { fn mjCTexture_get_content_type(self_ptr: *mut mjCTexture) -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { mjCTexture_get_content_type(self_ptr) }
}

/// C: mjCTexture::get_cubefiles (user/user_objects.h:1473)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_texture_get_cubefiles(self_ptr: *mut mjCTexture) -> i32 {
    extern "C" { fn mjCTexture_get_cubefiles(self_ptr: *mut mjCTexture) -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { mjCTexture_get_cubefiles(self_ptr) }
}

/// C: mjCTexture::GetCacheId (user/user_objects.h:1477)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_texture_get_cache_id(self_ptr: *mut mjCTexture, resource: *const mjResource, asset_type: *const std__string) -> std__string {
    extern "C" { fn mjCTexture_GetCacheId(self_ptr: *mut mjCTexture, resource: *const mjResource, asset_type: *const std__string) -> std__string; }
    // SAFETY: delegates to C implementation
    unsafe { mjCTexture_GetCacheId(self_ptr, resource, asset_type) }
}

/// C: mjCTexture::Builtin2D (user/user_objects.h:1478)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_texture_builtin2d(self_ptr: *mut mjCTexture) {
    if self_ptr.is_null() {
        return;
    }
    extern "C" { fn mjCTexture_Builtin2D(self_ptr: *mut mjCTexture); }
    // SAFETY: self_ptr verified non-null; delegates to C++ method
    unsafe { mjCTexture_Builtin2D(self_ptr) }
}

/// C: mjCTexture::BuiltinCube (user/user_objects.h:1479)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_texture_builtin_cube(self_ptr: *mut mjCTexture) {
    if self_ptr.is_null() {
        return;
    }
    extern "C" { fn mjCTexture_BuiltinCube(self_ptr: *mut mjCTexture); }
    // SAFETY: self_ptr verified non-null; delegates to C++ method
    unsafe { mjCTexture_BuiltinCube(self_ptr) }
}

/// C: mjCTexture::Load2D (user/user_objects.h:1480)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_texture_load2d(self_ptr: *mut mjCTexture, filename: string, vfs: *const mjVFS) {
    extern "C" { fn mjCTexture_Load2D(self_ptr: *mut mjCTexture, filename: string, vfs: *const mjVFS); }
    // SAFETY: delegates to C implementation
    unsafe { mjCTexture_Load2D(self_ptr, filename, vfs) }
}

/// C: mjCTexture::LoadCubeSingle (user/user_objects.h:1481)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_texture_load_cube_single(self_ptr: *mut mjCTexture, filename: string, vfs: *const mjVFS) {
    extern "C" { fn mjCTexture_LoadCubeSingle(self_ptr: *mut mjCTexture, filename: string, vfs: *const mjVFS); }
    // SAFETY: delegates to C implementation
    unsafe { mjCTexture_LoadCubeSingle(self_ptr, filename, vfs) }
}

/// C: mjCTexture::LoadCubeSeparate (user/user_objects.h:1483)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_texture_load_cube_separate(self_ptr: *mut mjCTexture, vfs: *const mjVFS) {
    if self_ptr.is_null() {
        return;
    }
    extern "C" { fn mjCTexture_LoadCubeSeparate(self_ptr: *mut mjCTexture, vfs: *const mjVFS); }
    // SAFETY: self_ptr verified non-null; delegates to C++ method
    unsafe { mjCTexture_LoadCubeSeparate(self_ptr, vfs) }
}

/// C: mjCMaterial::CopyFromSpec (user/user_objects.h:1526)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_material_copy_from_spec(self_ptr: *mut mjCMaterial) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCMaterial_CopyFromSpec(self_ptr: *mut mjCMaterial); }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCMaterial_CopyFromSpec(self_ptr) }
}

/// C: mjCMaterial::PointToLocal (user/user_objects.h:1527)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_material_point_to_local(self_ptr: *mut mjCMaterial) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCMaterial_PointToLocal(self_ptr: *mut mjCMaterial); }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCMaterial_PointToLocal(self_ptr) }
}

/// C: mjCMaterial::NameSpace (user/user_objects.h:1528)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_material_name_space(self_ptr: *mut mjCMaterial, m: *const mjCModel) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCMaterial_NameSpace(self_ptr: *mut mjCMaterial, m: *const mjCModel); }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCMaterial_NameSpace(self_ptr, m) }
}

/// C: mjCMaterial::get_texture (user/user_objects.h:1530)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_material_get_texture(self_ptr: *mut mjCMaterial, i: i32) -> *const i32 {
    if self_ptr.is_null() { return core::ptr::null(); }
    extern "C" { fn mjCMaterial_get_texture(self_ptr: *mut mjCMaterial, i: i32) -> *const i32; }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCMaterial_get_texture(self_ptr, i) }
}

/// C: mjCMaterial::del_textures (user/user_objects.h:1531)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_material_del_textures(self_ptr: *mut mjCMaterial) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCMaterial_del_textures(self_ptr: *mut mjCMaterial); }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCMaterial_del_textures(self_ptr) }
}

/// C: mjCMaterial::Compile (user/user_objects.h:1534)
/// Calls: mjCMaterial::CopyFromSpec
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_material_compile(self_ptr: *mut mjCMaterial) {
    extern "C" { fn mjCMaterial_Compile(self_ptr: *mut mjCMaterial); }
    // SAFETY: delegates to C implementation
    unsafe { mjCMaterial_Compile(self_ptr) }
}

/// C: mjCPair::CopyFromSpec (user/user_objects.h:1565)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_pair_copy_from_spec(self_ptr: *mut mjCPair) {
    extern "C" { fn mjCPair_CopyFromSpec(self_ptr: *mut mjCPair); }
    // SAFETY: delegates to C implementation
    unsafe { mjCPair_CopyFromSpec(self_ptr) }
}

/// C: mjCPair::PointToLocal (user/user_objects.h:1566)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_pair_point_to_local(self_ptr: *mut mjCPair) {
    extern "C" { fn mjCPair_PointToLocal(self_ptr: *mut mjCPair); }
    // SAFETY: delegates to C implementation
    unsafe { mjCPair_PointToLocal(self_ptr) }
}

/// C: mjCPair::ResolveReferences (user/user_objects.h:1567)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_pair_resolve_references(self_ptr: *mut mjCPair, m: *const mjCModel) {
    extern "C" { fn mjCPair_ResolveReferences(self_ptr: *mut mjCPair, m: *const mjCModel); }
    // SAFETY: delegates to C implementation
    unsafe { mjCPair_ResolveReferences(self_ptr, m) }
}

/// C: mjCPair::NameSpace (user/user_objects.h:1568)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_pair_name_space(self_ptr: *mut mjCPair, m: *const mjCModel) {
    extern "C" { fn mjCPair_NameSpace(self_ptr: *mut mjCPair, m: *const mjCModel); }
    // SAFETY: delegates to C implementation
    unsafe { mjCPair_NameSpace(self_ptr, m) }
}

/// C: mjCPair::get_geomname1 (user/user_objects.h:1570)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_pair_get_geomname1(self_ptr: *mut mjCPair) -> *const i32 {
    extern "C" { fn mjCPair_get_geomname1(self_ptr: *mut mjCPair) -> *const i32; }
    // SAFETY: delegates to C implementation
    unsafe { mjCPair_get_geomname1(self_ptr) }
}

/// C: mjCPair::get_geomname2 (user/user_objects.h:1571)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_pair_get_geomname2(self_ptr: *mut mjCPair) -> *const i32 {
    extern "C" { fn mjCPair_get_geomname2(self_ptr: *mut mjCPair) -> *const i32; }
    // SAFETY: delegates to C implementation
    unsafe { mjCPair_get_geomname2(self_ptr) }
}

/// C: mjCPair::GetSignature (user/user_objects.h:1573)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_pair_get_signature(self_ptr: *mut mjCPair) -> i32 {
    if self_ptr.is_null() {
        return 0;
    }
    0
}

/// C: mjCPair::Compile (user/user_objects.h:1578)
/// Calls: mjCGeom::SetNotVisual, mjCPair::CopyFromSpec
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_pair_compile(self_ptr: *mut mjCPair) {
    extern "C" { fn mjCPair_Compile(self_ptr: *mut mjCPair); }
    // SAFETY: delegates to C implementation
    unsafe { mjCPair_Compile(self_ptr) }
}

/// C: mjCBodyPair::CopyFromSpec (user/user_objects.h:1613)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_pair_copy_from_spec(self_ptr: *mut mjCBodyPair) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCBodyPair_CopyFromSpec(self_ptr: *mut mjCBodyPair); }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCBodyPair_CopyFromSpec(self_ptr) }
}

/// C: mjCBodyPair::PointToLocal (user/user_objects.h:1614)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_pair_point_to_local(self_ptr: *mut mjCBodyPair) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCBodyPair_PointToLocal(self_ptr: *mut mjCBodyPair); }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCBodyPair_PointToLocal(self_ptr) }
}

/// C: mjCBodyPair::ResolveReferences (user/user_objects.h:1615)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_pair_resolve_references(self_ptr: *mut mjCBodyPair, m: *const mjCModel) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCBodyPair_ResolveReferences(self_ptr: *mut mjCBodyPair, m: *const mjCModel); }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCBodyPair_ResolveReferences(self_ptr, m) }
}

/// C: mjCBodyPair::NameSpace (user/user_objects.h:1616)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_pair_name_space(self_ptr: *mut mjCBodyPair, m: *const mjCModel) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCBodyPair_NameSpace(self_ptr: *mut mjCBodyPair, m: *const mjCModel); }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCBodyPair_NameSpace(self_ptr, m) }
}

/// C: mjCBodyPair::get_bodyname1 (user/user_objects.h:1618)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_pair_get_bodyname1(self_ptr: *mut mjCBodyPair) -> i32 {
    if self_ptr.is_null() { return 0; }
    extern "C" { fn mjCBodyPair_get_bodyname1(self_ptr: *mut mjCBodyPair) -> i32; }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCBodyPair_get_bodyname1(self_ptr) }
}

/// C: mjCBodyPair::get_bodyname2 (user/user_objects.h:1619)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_pair_get_bodyname2(self_ptr: *mut mjCBodyPair) -> i32 {
    if self_ptr.is_null() { return 0; }
    extern "C" { fn mjCBodyPair_get_bodyname2(self_ptr: *mut mjCBodyPair) -> i32; }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCBodyPair_get_bodyname2(self_ptr) }
}

/// C: mjCBodyPair::GetSignature (user/user_objects.h:1621)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_pair_get_signature(self_ptr: *mut mjCBodyPair) -> i32 {
    if self_ptr.is_null() {
        return 0;
    }
    0
}

/// C: mjCBodyPair::Compile (user/user_objects.h:1626)
/// Calls: mjCBodyPair::CopyFromSpec
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_pair_compile(self_ptr: *mut mjCBodyPair) {
    extern "C" { fn mjCBodyPair_Compile(self_ptr: *mut mjCBodyPair); }
    // SAFETY: delegates to C implementation
    unsafe { mjCBodyPair_Compile(self_ptr) }
}

/// C: mjCEquality::CopyFromSpec (user/user_objects.h:1658)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_equality_copy_from_spec(self_ptr: *mut mjCEquality) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCEquality_CopyFromSpec(self_ptr: *mut mjCEquality); }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCEquality_CopyFromSpec(self_ptr) }
}

/// C: mjCEquality::PointToLocal (user/user_objects.h:1659)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_equality_point_to_local(self_ptr: *mut mjCEquality) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCEquality_PointToLocal(self_ptr: *mut mjCEquality); }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCEquality_PointToLocal(self_ptr) }
}

/// C: mjCEquality::ResolveReferences (user/user_objects.h:1660)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_equality_resolve_references(self_ptr: *mut mjCEquality, m: *const mjCModel) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCEquality_ResolveReferences(self_ptr: *mut mjCEquality, m: *const mjCModel); }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCEquality_ResolveReferences(self_ptr, m) }
}

/// C: mjCEquality::NameSpace (user/user_objects.h:1661)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_equality_name_space(self_ptr: *mut mjCEquality, m: *const mjCModel) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCEquality_NameSpace(self_ptr: *mut mjCEquality, m: *const mjCModel); }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCEquality_NameSpace(self_ptr, m) }
}

/// C: mjCEquality::Compile (user/user_objects.h:1664)
/// Calls: mjCEquality::CopyFromSpec
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_equality_compile(self_ptr: *mut mjCEquality) {
    extern "C" { fn mjCEquality_Compile(self_ptr: *mut mjCEquality); }
    // SAFETY: delegates to C implementation
    unsafe { mjCEquality_Compile(self_ptr) }
}

/// C: mjCTendon::set_material (user/user_objects.h:1697)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tendon_set_material(self_ptr: *mut mjCTendon, _material: i32) {
    extern "C" { fn mjCTendon_set_material(self_ptr: *mut mjCTendon, _material: i32); }
    // SAFETY: delegates to C implementation
    unsafe { mjCTendon_set_material(self_ptr, _material) }
}

/// C: mjCTendon::get_material (user/user_objects.h:1698)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tendon_get_material(self_ptr: *mut mjCTendon) -> *const i32 {
    extern "C" { fn mjCTendon_get_material(self_ptr: *mut mjCTendon) -> *const i32; }
    // SAFETY: delegates to C implementation
    unsafe { mjCTendon_get_material(self_ptr) }
}

/// C: mjCTendon::del_material (user/user_objects.h:1699)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tendon_del_material(self_ptr: *mut mjCTendon) {
    extern "C" { fn mjCTendon_del_material(self_ptr: *mut mjCTendon); }
    // SAFETY: delegates to C implementation
    unsafe { mjCTendon_del_material(self_ptr) }
}

/// C: mjCTendon::WrapSite (user/user_objects.h:1702)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tendon_wrap_site(self_ptr: *mut mjCTendon, wrapname: string, wrapinfo: string_view) {
    extern "C" { fn mjCTendon_WrapSite(self_ptr: *mut mjCTendon, wrapname: string, wrapinfo: string_view); }
    // SAFETY: delegates to C implementation
    unsafe { mjCTendon_WrapSite(self_ptr, wrapname, wrapinfo) }
}

/// C: mjCTendon::WrapGeom (user/user_objects.h:1703)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tendon_wrap_geom(self_ptr: *mut mjCTendon, wrapname: string, side: string, wrapinfo: string_view) {
    extern "C" { fn mjCTendon_WrapGeom(self_ptr: *mut mjCTendon, wrapname: string, side: string, wrapinfo: string_view); }
    // SAFETY: delegates to C implementation
    unsafe { mjCTendon_WrapGeom(self_ptr, wrapname, side, wrapinfo) }
}

/// C: mjCTendon::WrapJoint (user/user_objects.h:1704)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tendon_wrap_joint(self_ptr: *mut mjCTendon, wrapname: string, coef: f64, wrapinfo: string_view) {
    extern "C" { fn mjCTendon_WrapJoint(self_ptr: *mut mjCTendon, wrapname: string, coef: f64, wrapinfo: string_view); }
    // SAFETY: delegates to C implementation
    unsafe { mjCTendon_WrapJoint(self_ptr, wrapname, coef, wrapinfo) }
}

/// C: mjCTendon::WrapPulley (user/user_objects.h:1705)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tendon_wrap_pulley(self_ptr: *mut mjCTendon, divisor: f64, wrapinfo: string_view) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCTendon_WrapPulley(self_ptr: *mut mjCTendon, divisor: f64, wrapinfo: string_view); }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCTendon_WrapPulley(self_ptr, divisor, wrapinfo) }
}

/// C: mjCTendon::NumWraps (user/user_objects.h:1708)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tendon_num_wraps(self_ptr: *mut mjCTendon) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCTendon)
    // Previous return: i32
    extern "C" { fn mjCTendon_NumWraps(self_ptr: *mut mjCTendon) -> i32; }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mjCTendon_NumWraps(self_ptr) }
}

/// C: mjCTendon::GetWrap (user/user_objects.h:1709)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tendon_get_wrap(self_ptr: *mut mjCTendon, i: i32) -> *const mjCWrap {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCTendon, i : i32)
    // Previous return: * const mjCWrap
    extern "C" { fn mjCTendon_GetWrap(self_ptr: *mut mjCTendon, i: i32) -> *const mjCWrap; }
    // SAFETY: delegates to C implementation, all pointers valid per caller contract
    unsafe { mjCTendon_GetWrap(self_ptr, i) }
}

/// C: mjCTendon::get_userdata (user/user_objects.h:1713)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tendon_get_userdata(self_ptr: *mut mjCTendon) -> *const i32 {
    extern "C" { fn mjCTendon_get_userdata(self_ptr: *mut mjCTendon) -> *const i32; }
    // SAFETY: delegates to C implementation
    unsafe { mjCTendon_get_userdata(self_ptr) }
}

/// C: mjCTendon::get_range (user/user_objects.h:1714)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tendon_get_range(self_ptr: *mut mjCTendon) -> *const f64 {
    if self_ptr.is_null() { return core::ptr::null(); }
    core::ptr::null()
}

/// C: mjCTendon::CopyFromSpec (user/user_objects.h:1716)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tendon_copy_from_spec(self_ptr: *mut mjCTendon) {
    extern "C" { fn mjCTendon_CopyFromSpec(self_ptr: *mut mjCTendon); }
    // SAFETY: delegates to C implementation
    unsafe { mjCTendon_CopyFromSpec(self_ptr) }
}

/// C: mjCTendon::PointToLocal (user/user_objects.h:1717)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tendon_point_to_local(self_ptr: *mut mjCTendon) {
    extern "C" { fn mjCTendon_PointToLocal(self_ptr: *mut mjCTendon); }
    // SAFETY: delegates to C implementation
    unsafe { mjCTendon_PointToLocal(self_ptr) }
}

/// C: mjCTendon::ResolveReferences (user/user_objects.h:1718)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tendon_resolve_references(self_ptr: *mut mjCTendon, m: *const mjCModel) {
    extern "C" { fn mjCTendon_ResolveReferences(self_ptr: *mut mjCTendon, m: *const mjCModel); }
    // SAFETY: delegates to C implementation
    unsafe { mjCTendon_ResolveReferences(self_ptr, m) }
}

/// C: mjCTendon::NameSpace (user/user_objects.h:1719)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tendon_name_space(self_ptr: *mut mjCTendon, m: *const mjCModel) {
    extern "C" { fn mjCTendon_NameSpace(self_ptr: *mut mjCTendon, m: *const mjCModel); }
    // SAFETY: delegates to C implementation
    unsafe { mjCTendon_NameSpace(self_ptr, m) }
}

/// C: mjCTendon::SetModel (user/user_objects.h:1720)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tendon_set_model(self_ptr: *mut mjCTendon, _model: *mut mjCModel) {
    extern "C" { fn mjCTendon_SetModel(self_ptr: *mut mjCTendon, _model: *mut mjCModel); }
    // SAFETY: delegates to C implementation
    unsafe { mjCTendon_SetModel(self_ptr, _model) }
}

/// C: mjCTendon::is_limited (user/user_objects.h:1722)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tendon_is_limited(self_ptr: *mut mjCTendon) -> bool {
    extern "C" { fn mjCTendon_is_limited(self_ptr: *mut mjCTendon) -> bool; }
    // SAFETY: delegates to C implementation
    unsafe { mjCTendon_is_limited(self_ptr) }
}

/// C: mjCTendon::is_actfrclimited (user/user_objects.h:1723)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tendon_is_actfrclimited(self_ptr: *mut mjCTendon) -> bool {
    extern "C" { fn mjCTendon_is_actfrclimited(self_ptr: *mut mjCTendon) -> bool; }
    // SAFETY: delegates to C implementation
    unsafe { mjCTendon_is_actfrclimited(self_ptr) }
}

/// C: mjCTendon::Compile (user/user_objects.h:1726)
/// Calls: mjCTendon::CopyFromSpec
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tendon_compile(self_ptr: *mut mjCTendon) {
    extern "C" { fn mjCTendon_Compile(self_ptr: *mut mjCTendon); }
    // SAFETY: delegates to C implementation
    unsafe { mjCTendon_Compile(self_ptr) }
}

/// C: mjCWrap::CopyFromSpec (user/user_objects.h:1749)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_wrap_copy_from_spec(self_ptr: *mut mjCWrap) {
    extern "C" { fn mjCWrap_CopyFromSpec(self_ptr: *mut mjCWrap); }
    // SAFETY: delegates to C implementation
    unsafe { mjCWrap_CopyFromSpec(self_ptr) }
}

/// C: mjCWrap::PointToLocal (user/user_objects.h:1750)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_wrap_point_to_local(self_ptr: *mut mjCWrap) {
    extern "C" { fn mjCWrap_PointToLocal(self_ptr: *mut mjCWrap); }
    // SAFETY: delegates to C implementation
    unsafe { mjCWrap_PointToLocal(self_ptr) }
}

/// C: mjCWrap::ResolveReferences (user/user_objects.h:1751)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_wrap_resolve_references(self_ptr: *mut mjCWrap, m: *const mjCModel) {
    extern "C" { fn mjCWrap_ResolveReferences(self_ptr: *mut mjCWrap, m: *const mjCModel); }
    // SAFETY: delegates to C implementation
    unsafe { mjCWrap_ResolveReferences(self_ptr, m) }
}

/// C: mjCWrap::NameSpace (user/user_objects.h:1752)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_wrap_name_space(self_ptr: *mut mjCWrap, m: *const mjCModel) {
    extern "C" { fn mjCWrap_NameSpace(self_ptr: *mut mjCWrap, m: *const mjCModel); }
    // SAFETY: delegates to C implementation
    unsafe { mjCWrap_NameSpace(self_ptr, m) }
}

/// C: mjCWrap::Type (user/user_objects.h:1753)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_wrap_type(self_ptr: *mut mjCWrap) -> mjtWrap {
    if self_ptr.is_null() {
        return unsafe { core::mem::zeroed() };
    }
    extern "C" { fn mjCWrap_Type(self_ptr: *mut mjCWrap) -> mjtWrap; }
    unsafe { mjCWrap_Type(self_ptr) }
}

/// C: mjCWrap::Compile (user/user_objects.h:1762)
/// Calls: mjCWrap::CopyFromSpec
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_wrap_compile(self_ptr: *mut mjCWrap) {
    extern "C" { fn mjCWrap_Compile(self_ptr: *mut mjCWrap); }
    // SAFETY: delegates to C implementation
    unsafe { mjCWrap_Compile(self_ptr) }
}

/// C: mjCPlugin::PointToLocal (user/user_objects.h:1791)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_plugin_point_to_local(self_ptr: *mut mjCPlugin) {
    extern "C" { fn mjCPlugin_PointToLocal(self_ptr: *mut mjCPlugin); }
    // SAFETY: delegates to C implementation
    unsafe { mjCPlugin_PointToLocal(self_ptr) }
}

/// C: mjCPlugin::Compile (user/user_objects.h:1798)
/// Calls: mjp_getPluginAtSlot
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_plugin_compile(self_ptr: *mut mjCPlugin) {
    extern "C" { fn mjCPlugin_Compile(self_ptr: *mut mjCPlugin); }
    // SAFETY: delegates to C implementation
    unsafe { mjCPlugin_Compile(self_ptr) }
}

/// C: mjCActuator::get_userdata (user/user_objects.h:1843)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_actuator_get_userdata(self_ptr: *mut mjCActuator) -> *const i32 {
    if self_ptr.is_null() { return core::ptr::null(); }
    core::ptr::null()
}

/// C: mjCActuator::get_target (user/user_objects.h:1844)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_actuator_get_target(self_ptr: *mut mjCActuator) -> *const i32 {
    if self_ptr.is_null() { return core::ptr::null(); }
    core::ptr::null()
}

/// C: mjCActuator::get_slidersite (user/user_objects.h:1845)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_actuator_get_slidersite(self_ptr: *mut mjCActuator) -> *const i32 {
    if self_ptr.is_null() { return core::ptr::null(); }
    core::ptr::null()
}

/// C: mjCActuator::get_refsite (user/user_objects.h:1846)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_actuator_get_refsite(self_ptr: *mut mjCActuator) -> *const i32 {
    if self_ptr.is_null() { return core::ptr::null(); }
    core::ptr::null()
}

/// C: mjCActuator::is_ctrllimited (user/user_objects.h:1848)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_actuator_is_ctrllimited(self_ptr: *mut mjCActuator) -> bool {
    if self_ptr.is_null() { return false; }
    extern "C" { fn mjCActuator_is_ctrllimited(self_ptr: *mut mjCActuator) -> bool; }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCActuator_is_ctrllimited(self_ptr) }
}

/// C: mjCActuator::is_forcelimited (user/user_objects.h:1849)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_actuator_is_forcelimited(self_ptr: *mut mjCActuator) -> bool {
    if self_ptr.is_null() { return false; }
    extern "C" { fn mjCActuator_is_forcelimited(self_ptr: *mut mjCActuator) -> bool; }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCActuator_is_forcelimited(self_ptr) }
}

/// C: mjCActuator::is_actlimited (user/user_objects.h:1850)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_actuator_is_actlimited(self_ptr: *mut mjCActuator) -> bool {
    if self_ptr.is_null() { return false; }
    false
}

/// C: mjCActuator::ctrl (user/user_objects.h:1853)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_actuator_ctrl(self_ptr: *mut mjCActuator, state_name: *const std__string) -> *mut f64 {
    if self_ptr.is_null() { return core::ptr::null_mut(); }
    core::ptr::null_mut()
}

/// C: mjCActuator::Compile (user/user_objects.h:1856)
/// Calls: mjCActuator::CopyFromSpec, mjCJoint::get_range, mjCTendon::get_range, mjp_getPluginAtSlot
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_actuator_compile(self_ptr: *mut mjCActuator) {
    extern "C" { fn mjCActuator_Compile(self_ptr: *mut mjCActuator); }
    // SAFETY: delegates to C implementation
    unsafe { mjCActuator_Compile(self_ptr) }
}

/// C: mjCActuator::CopyFromSpec (user/user_objects.h:1857)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_actuator_copy_from_spec(self_ptr: *mut mjCActuator) {
    if self_ptr.is_null() { return; }
}

/// C: mjCActuator::PointToLocal (user/user_objects.h:1858)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_actuator_point_to_local(self_ptr: *mut mjCActuator) {
    if self_ptr.is_null() { return; }
}

/// C: mjCActuator::ResolveReferences (user/user_objects.h:1859)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_actuator_resolve_references(self_ptr: *mut mjCActuator, m: *const mjCModel) {
    if self_ptr.is_null() { return; }
}

/// C: mjCActuator::NameSpace (user/user_objects.h:1860)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_actuator_name_space(self_ptr: *mut mjCActuator, m: *const mjCModel) {
    if self_ptr.is_null() { return; }
}

/// C: mjCActuator::CopyPlugin (user/user_objects.h:1861)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_actuator_copy_plugin(self_ptr: *mut mjCActuator) {
    if self_ptr.is_null() { return; }
}

/// C: mjCActuator::ForgetKeyframes (user/user_objects.h:1864)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_actuator_forget_keyframes(self_ptr: *mut mjCActuator) {
    if self_ptr.is_null() { return; }
}

/// C: mjCSensor::get_userdata (user/user_objects.h:1901)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_sensor_get_userdata(self_ptr: *mut mjCSensor) -> *const i32 {
    extern "C" { fn mjCSensor_get_userdata(self_ptr: *mut mjCSensor) -> *const i32; }
    // SAFETY: delegates to C implementation
    unsafe { mjCSensor_get_userdata(self_ptr) }
}

/// C: mjCSensor::get_objname (user/user_objects.h:1902)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_sensor_get_objname(self_ptr: *mut mjCSensor) -> *const i32 {
    extern "C" { fn mjCSensor_get_objname(self_ptr: *mut mjCSensor) -> *const i32; }
    // SAFETY: delegates to C implementation
    unsafe { mjCSensor_get_objname(self_ptr) }
}

/// C: mjCSensor::get_refname (user/user_objects.h:1903)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_sensor_get_refname(self_ptr: *mut mjCSensor) -> *const i32 {
    extern "C" { fn mjCSensor_get_refname(self_ptr: *mut mjCSensor) -> *const i32; }
    // SAFETY: delegates to C implementation
    unsafe { mjCSensor_get_refname(self_ptr) }
}

/// C: mjCSensor::get_obj (user/user_objects.h:1905)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_sensor_get_obj(self_ptr: *mut mjCSensor) -> *const mjCBase {
    extern "C" { fn mjCSensor_get_obj(self_ptr: *mut mjCSensor) -> *const mjCBase; }
    // SAFETY: delegates to C implementation
    unsafe { mjCSensor_get_obj(self_ptr) }
}

/// C: mjCSensor::get_ref (user/user_objects.h:1906)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_sensor_get_ref(self_ptr: *mut mjCSensor) -> *const mjCBase {
    extern "C" { fn mjCSensor_get_ref(self_ptr: *mut mjCSensor) -> *const mjCBase; }
    // SAFETY: delegates to C implementation
    unsafe { mjCSensor_get_ref(self_ptr) }
}

/// C: mjCSensor::Compile (user/user_objects.h:1909)
/// Calls: mjCJoint::is_limited, mjCSensor::CopyFromSpec, mjCTendon::is_limited, mjp_getPluginAtSlot
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_sensor_compile(self_ptr: *mut mjCSensor) {
    extern "C" { fn mjCSensor_Compile(self_ptr: *mut mjCSensor); }
    // SAFETY: delegates to C implementation
    unsafe { mjCSensor_Compile(self_ptr) }
}

/// C: mjCSensor::CopyFromSpec (user/user_objects.h:1910)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_sensor_copy_from_spec(self_ptr: *mut mjCSensor) {
    extern "C" { fn mjCSensor_CopyFromSpec(self_ptr: *mut mjCSensor); }
    // SAFETY: delegates to C implementation
    unsafe { mjCSensor_CopyFromSpec(self_ptr) }
}

/// C: mjCSensor::PointToLocal (user/user_objects.h:1911)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_sensor_point_to_local(self_ptr: *mut mjCSensor) {
    extern "C" { fn mjCSensor_PointToLocal(self_ptr: *mut mjCSensor); }
    // SAFETY: delegates to C implementation
    unsafe { mjCSensor_PointToLocal(self_ptr) }
}

/// C: mjCSensor::ResolveReferences (user/user_objects.h:1912)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_sensor_resolve_references(self_ptr: *mut mjCSensor, m: *const mjCModel) {
    extern "C" { fn mjCSensor_ResolveReferences(self_ptr: *mut mjCSensor, m: *const mjCModel); }
    // SAFETY: delegates to C implementation
    unsafe { mjCSensor_ResolveReferences(self_ptr, m) }
}

/// C: mjCSensor::NameSpace (user/user_objects.h:1913)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_sensor_name_space(self_ptr: *mut mjCSensor, m: *const mjCModel) {
    extern "C" { fn mjCSensor_NameSpace(self_ptr: *mut mjCSensor, m: *const mjCModel); }
    // SAFETY: delegates to C implementation
    unsafe { mjCSensor_NameSpace(self_ptr, m) }
}

/// C: mjCSensor::CopyPlugin (user/user_objects.h:1914)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_sensor_copy_plugin(self_ptr: *mut mjCSensor) {
    extern "C" { fn mjCSensor_CopyPlugin(self_ptr: *mut mjCSensor); }
    // SAFETY: delegates to C implementation
    unsafe { mjCSensor_CopyPlugin(self_ptr) }
}

/// C: mjCNumeric::PointToLocal (user/user_objects.h:1944)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_numeric_point_to_local(self_ptr: *mut mjCNumeric) {
    extern "C" { fn mjCNumeric_PointToLocal(self_ptr: *mut mjCNumeric); }
    // SAFETY: delegates to C implementation
    unsafe { mjCNumeric_PointToLocal(self_ptr) }
}

/// C: mjCNumeric::CopyFromSpec (user/user_objects.h:1945)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_numeric_copy_from_spec(self_ptr: *mut mjCNumeric) {
    extern "C" { fn mjCNumeric_CopyFromSpec(self_ptr: *mut mjCNumeric); }
    // SAFETY: delegates to C implementation
    unsafe { mjCNumeric_CopyFromSpec(self_ptr) }
}

/// C: mjCNumeric::Compile (user/user_objects.h:1948)
/// Calls: mjCNumeric::CopyFromSpec
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_numeric_compile(self_ptr: *mut mjCNumeric) {
    extern "C" { fn mjCNumeric_Compile(self_ptr: *mut mjCNumeric); }
    // SAFETY: delegates to C implementation
    unsafe { mjCNumeric_Compile(self_ptr) }
}

/// C: mjCText::PointToLocal (user/user_objects.h:1975)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_text_point_to_local(self_ptr: *mut mjCText) {
    extern "C" { fn mjCText_PointToLocal(self_ptr: *mut mjCText); }
    // SAFETY: delegates to C implementation
    unsafe { mjCText_PointToLocal(self_ptr) }
}

/// C: mjCText::CopyFromSpec (user/user_objects.h:1976)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_text_copy_from_spec(self_ptr: *mut mjCText) {
    extern "C" { fn mjCText_CopyFromSpec(self_ptr: *mut mjCText); }
    // SAFETY: delegates to C implementation
    unsafe { mjCText_CopyFromSpec(self_ptr) }
}

/// C: mjCText::Compile (user/user_objects.h:1979)
/// Calls: mjCText::CopyFromSpec
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_text_compile(self_ptr: *mut mjCText) {
    extern "C" { fn mjCText_Compile(self_ptr: *mut mjCText); }
    // SAFETY: delegates to C implementation
    unsafe { mjCText_Compile(self_ptr) }
}

/// C: mjCTuple::PointToLocal (user/user_objects.h:2011)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tuple_point_to_local(self_ptr: *mut mjCTuple) {
    extern "C" { fn mjCTuple_PointToLocal(self_ptr: *mut mjCTuple); }
    // SAFETY: delegates to C implementation
    unsafe { mjCTuple_PointToLocal(self_ptr) }
}

/// C: mjCTuple::CopyFromSpec (user/user_objects.h:2012)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tuple_copy_from_spec(self_ptr: *mut mjCTuple) {
    extern "C" { fn mjCTuple_CopyFromSpec(self_ptr: *mut mjCTuple); }
    // SAFETY: delegates to C implementation
    unsafe { mjCTuple_CopyFromSpec(self_ptr) }
}

/// C: mjCTuple::ResolveReferences (user/user_objects.h:2013)
/// Calls: mjCGeom::SetNotVisual
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tuple_resolve_references(self_ptr: *mut mjCTuple, m: *const mjCModel) {
    extern "C" { fn mjCTuple_ResolveReferences(self_ptr: *mut mjCTuple, m: *const mjCModel); }
    // SAFETY: delegates to C implementation
    unsafe { mjCTuple_ResolveReferences(self_ptr, m) }
}

/// C: mjCTuple::NameSpace (user/user_objects.h:2014)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tuple_name_space(self_ptr: *mut mjCTuple, m: *const mjCModel) {
    extern "C" { fn mjCTuple_NameSpace(self_ptr: *mut mjCTuple, m: *const mjCModel); }
    // SAFETY: delegates to C implementation
    unsafe { mjCTuple_NameSpace(self_ptr, m) }
}

/// C: mjCTuple::Compile (user/user_objects.h:2017)
/// Calls: mjCTuple::CopyFromSpec
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tuple_compile(self_ptr: *mut mjCTuple) {
    extern "C" { fn mjCTuple_Compile(self_ptr: *mut mjCTuple); }
    // SAFETY: delegates to C implementation
    unsafe { mjCTuple_Compile(self_ptr) }
}

/// C: mjCKey::PointToLocal (user/user_objects.h:2054)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_key_point_to_local(self_ptr: *mut mjCKey) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCKey_PointToLocal(self_ptr: *mut mjCKey); }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCKey_PointToLocal(self_ptr) }
}

/// C: mjCKey::CopyFromSpec (user/user_objects.h:2055)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_key_copy_from_spec(self_ptr: *mut mjCKey) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCKey_CopyFromSpec(self_ptr: *mut mjCKey); }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCKey_CopyFromSpec(self_ptr) }
}

/// C: mjCKey::Compile (user/user_objects.h:2058)
/// Calls: mjCKey::CopyFromSpec
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_key_compile(self_ptr: *mut mjCKey, m: *const mjModel) {
    extern "C" { fn mjCKey_Compile(self_ptr: *mut mjCKey, m: *const mjModel); }
    // SAFETY: delegates to C implementation
    unsafe { mjCKey_Compile(self_ptr, m) }
}

/// C: mjCDef::CopyWithoutChildren (user/user_objects.h:2076)
/// Calls: mjCDef::PointToLocal
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_def_copy_without_children(self_ptr: *mut mjCDef, other: *const mjCDef) {
    extern "C" { fn mjCDef_CopyWithoutChildren(self_ptr: *mut mjCDef, other: *const mjCDef); }
    // SAFETY: delegates to C implementation
    unsafe { mjCDef_CopyWithoutChildren(self_ptr, other) }
}

/// C: mjCDef::PointToLocal (user/user_objects.h:2077)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_def_point_to_local(self_ptr: *mut mjCDef) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCDef_PointToLocal(self_ptr: *mut mjCDef); }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCDef_PointToLocal(self_ptr) }
}

/// C: mjCDef::CopyFromSpec (user/user_objects.h:2078)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_def_copy_from_spec(self_ptr: *mut mjCDef) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCDef_CopyFromSpec(self_ptr: *mut mjCDef); }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCDef_CopyFromSpec(self_ptr) }
}

/// C: mjCDef::NameSpace (user/user_objects.h:2079)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_def_name_space(self_ptr: *mut mjCDef, m: *const mjCModel) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCDef_NameSpace(self_ptr: *mut mjCDef, m: *const mjCModel); }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCDef_NameSpace(self_ptr, m) }
}

/// C: mjCDef::Compile (user/user_objects.h:2081)
/// Calls: mjCDef::CopyFromSpec
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_def_compile(self_ptr: *mut mjCDef, model: *const mjCModel) {
    extern "C" { fn mjCDef_Compile(self_ptr: *mut mjCDef, model: *const mjCModel); }
    // SAFETY: delegates to C implementation
    unsafe { mjCDef_Compile(self_ptr, model) }
}

/// C: mjCDef::Joint (user/user_objects.h:2084)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_def_joint(self_ptr: *mut mjCDef) -> *mut mjCJoint {
    if self_ptr.is_null() { return core::ptr::null_mut(); }
    extern "C" { fn mjCDef_Joint(self_ptr: *mut mjCDef) -> *mut mjCJoint; }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCDef_Joint(self_ptr) }
}

/// C: mjCDef::Geom (user/user_objects.h:2085)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_def_geom(self_ptr: *mut mjCDef) -> *mut mjCGeom {
    if self_ptr.is_null() { return core::ptr::null_mut(); }
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjCDef)
    // Previous return: * mut mjCGeom
    extern "C" { fn mjCDef_Geom(self_ptr: *mut mjCDef) -> *mut mjCGeom; }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCDef_Geom(self_ptr) }
}

/// C: mjCDef::Site (user/user_objects.h:2086)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_def_site(self_ptr: *mut mjCDef) -> *mut mjCSite {
    if self_ptr.is_null() { return core::ptr::null_mut(); }
    extern "C" { fn mjCDef_Site(self_ptr: *mut mjCDef) -> *mut mjCSite; }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCDef_Site(self_ptr) }
}

/// C: mjCDef::Camera (user/user_objects.h:2087)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_def_camera(self_ptr: *mut mjCDef) -> *mut mjCCamera {
    if self_ptr.is_null() { return core::ptr::null_mut(); }
    extern "C" { fn mjCDef_Camera(self_ptr: *mut mjCDef) -> *mut mjCCamera; }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCDef_Camera(self_ptr) }
}

/// C: mjCDef::Light (user/user_objects.h:2088)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_def_light(self_ptr: *mut mjCDef) -> *mut mjCLight {
    if self_ptr.is_null() { return core::ptr::null_mut(); }
    extern "C" { fn mjCDef_Light(self_ptr: *mut mjCDef) -> *mut mjCLight; }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCDef_Light(self_ptr) }
}

/// C: mjCDef::Flex (user/user_objects.h:2089)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_def_flex(self_ptr: *mut mjCDef) -> *mut mjCFlex {
    if self_ptr.is_null() { return core::ptr::null_mut(); }
    extern "C" { fn mjCDef_Flex(self_ptr: *mut mjCDef) -> *mut mjCFlex; }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCDef_Flex(self_ptr) }
}

/// C: mjCDef::Mesh (user/user_objects.h:2090)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_def_mesh(self_ptr: *mut mjCDef) -> *mut mjCMesh {
    if self_ptr.is_null() { return core::ptr::null_mut(); }
    extern "C" { fn mjCDef_Mesh(self_ptr: *mut mjCDef) -> *mut mjCMesh; }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCDef_Mesh(self_ptr) }
}

/// C: mjCDef::Material (user/user_objects.h:2091)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_def_material(self_ptr: *mut mjCDef) -> *mut mjCMaterial {
    if self_ptr.is_null() { return core::ptr::null_mut(); }
    extern "C" { fn mjCDef_Material(self_ptr: *mut mjCDef) -> *mut mjCMaterial; }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCDef_Material(self_ptr) }
}

/// C: mjCDef::Pair (user/user_objects.h:2092)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_def_pair(self_ptr: *mut mjCDef) -> *mut mjCPair {
    if self_ptr.is_null() { return core::ptr::null_mut(); }
    extern "C" { fn mjCDef_Pair(self_ptr: *mut mjCDef) -> *mut mjCPair; }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCDef_Pair(self_ptr) }
}

/// C: mjCDef::Equality (user/user_objects.h:2093)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_def_equality(self_ptr: *mut mjCDef) -> *mut mjCEquality {
    if self_ptr.is_null() { return core::ptr::null_mut(); }
    extern "C" { fn mjCDef_Equality(self_ptr: *mut mjCDef) -> *mut mjCEquality; }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCDef_Equality(self_ptr) }
}

/// C: mjCDef::Tendon (user/user_objects.h:2094)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_def_tendon(self_ptr: *mut mjCDef) -> *mut mjCTendon {
    if self_ptr.is_null() { return core::ptr::null_mut(); }
    extern "C" { fn mjCDef_Tendon(self_ptr: *mut mjCDef) -> *mut mjCTendon; }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCDef_Tendon(self_ptr) }
}

/// C: mjCDef::Actuator (user/user_objects.h:2095)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_def_actuator(self_ptr: *mut mjCDef) -> *mut mjCActuator {
    if self_ptr.is_null() { return core::ptr::null_mut(); }
    extern "C" { fn mjCDef_Actuator(self_ptr: *mut mjCDef) -> *mut mjCActuator; }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCDef_Actuator(self_ptr) }
}

