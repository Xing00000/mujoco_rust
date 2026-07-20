//! Port of: user/user_objects.cc
//! IR hash: adc2f24e872d94f7
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: PNGImage::Load (user/user_objects.cc:58)
/// Calls: mju_readResource
#[allow(unused_variables, non_snake_case)]
pub fn png_image_load(obj: *const mjCBase, resource: *mut mjResource, color_type: u32) -> anonymous_namespace___PNGImage {
    todo!() // PNGImage::Load
}

/// C: PNGImage::Width (user/user_objects.cc:60)
#[allow(unused_variables, non_snake_case)]
pub fn png_image_width(self_ptr: *mut anonymous_namespace___PNGImage) -> i32 {
    todo!() // PNGImage::Width
}

/// C: PNGImage::Height (user/user_objects.cc:61)
#[allow(unused_variables, non_snake_case)]
pub fn png_image_height(self_ptr: *mut anonymous_namespace___PNGImage) -> i32 {
    todo!() // PNGImage::Height
}

/// C: PNGImage::IsSRGB (user/user_objects.cc:62)
#[allow(unused_variables, non_snake_case)]
pub fn png_image_is_srgb(self_ptr: *mut anonymous_namespace___PNGImage) -> bool {
    todo!() // PNGImage::IsSRGB
}

/// C: PNGImage::MoveData (user/user_objects.cc:66)
#[allow(unused_variables, non_snake_case)]
pub fn png_image_move_data(self_ptr: *mut anonymous_namespace___PNGImage) -> *mut *mut mjByteVec {
    todo!() // PNGImage::MoveData
}

/// C: PNGImage::Size (user/user_objects.cc:69)
#[allow(unused_variables, non_snake_case)]
pub fn png_image_size(self_ptr: *mut anonymous_namespace___PNGImage) -> u64 {
    todo!() // PNGImage::Size
}

/// C: MapFrame (user/user_objects.cc:139)
#[allow(unused_variables, non_snake_case)]
pub fn map_frame(parent: *const (), child: *const (), frame: *mut mjCFrame, parent_body: *mut mjCBody) {
    todo!() // MapFrame
}

/// C: checksize (user/user_objects.cc:153)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn checksize(size: *mut f64, r#type: u32, object: *mut mjCBase, name: *const i8, id: i32) {
    todo!() // checksize
}

/// C: checklimited (user/user_objects.cc:172)
#[allow(unused_variables, non_snake_case)]
pub fn checklimited(obj: *const mjCBase, autolimits: bool, entity: *const i8, attr: *const i8, limited: i32, hasrange: bool) {
    todo!() // checklimited
}

/// C: islimited (user/user_objects.cc:185)
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

/// C: pointBoxDistSq (user/user_objects.cc:635)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn point_box_dist_sq(p: *const f64, aabb: *const f64) -> f64 {
    todo!() // pointBoxDistSq
}

/// C: pointTriDistSqWithUV (user/user_objects.cc:652)
/// Calls: inside, triangle
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn point_tri_dist_sq_with_uv(p: *const f64, v0: *const f64, v1: *const f64, v2: *const f64, out_u: *mut f64, out_v: *mut f64) -> f64 {
    todo!() // pointTriDistSqWithUV
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
    todo!() // queryClosestBVHWithFace
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
    todo!() // querySignedDistance
}

/// C: dot2 (user/user_objects.cc:923)
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

/// C: boxTriangle (user/user_objects.cc:929)
/// Calls: dot2, mju_max, mjuu_crossvec, mjuu_dot3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn box_triangle(v: *const Triangle, aamm: *const f64) -> bool {
    todo!() // boxTriangle
}

/// C: mjCBody::GetList (user/user_objects.cc:2311)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_get_list(self_ptr: *mut mjCBody) -> *const () {
    todo!() // mjCBody::GetList
}

/// C: GetNextBody (user/user_objects.cc:2380)
/// Calls: mjCBody::Bodies
#[allow(unused_variables, non_snake_case)]
pub fn get_next_body(body: *const mjCBody, child: *const mjsElement, found: *mut bool, recursive: bool) -> *mut mjsElement {
    todo!() // GetNextBody
}

/// C: randomdot (user/user_objects.cc:4973)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn randomdot(rgb: *mut u8, markrgb: *const f64, width: i32, height: i32, probability: f64) {
    todo!() // randomdot
}

/// C: interp (user/user_objects.cc:4995)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn interp(rgb: *mut u8, rgb1: *const f64, rgb2: *const f64, pos: f64) {
    todo!() // interp
}

/// C: checker (user/user_objects.cc:5012)
#[allow(unused_variables, non_snake_case)]
pub fn checker(rgb: *mut u8, RGB1: *const u8, RGB2: *const u8, width: i32, height: i32) {
    todo!() // checker
}

/// C: sensorDatatype (user/user_objects.cc:7480)
#[allow(unused_variables, non_snake_case)]
pub fn sensor_datatype(r#type: u32) -> u32 {
    match r#type {
        0 |  // mjSENS_TOUCH
        38   // mjSENS_INSIDESITE
            => 1,  // mjDATATYPE_POSITIVE

        28 |  // mjSENS_FRAMEXAXIS
        29 |  // mjSENS_FRAMEYAXIS
        30 |  // mjSENS_FRAMEZAXIS
        40    // mjSENS_GEOMNORMAL
            => 2,  // mjDATATYPE_AXIS

        18 |  // mjSENS_BALLQUAT
        27    // mjSENS_FRAMEQUAT
            => 3,  // mjDATATYPE_QUATERNION

        _ => 0,  // mjDATATYPE_REAL
    }
}

/// C: sensorNeedstage (user/user_objects.cc:7544)
#[allow(unused_variables, non_snake_case)]
pub fn sensor_needstage(r#type: u32) -> u32 {
    match r#type {
        0  |  // mjSENS_TOUCH
        1  |  // mjSENS_ACCELEROMETER
        4  |  // mjSENS_FORCE
        5  |  // mjSENS_TORQUE
        15 |  // mjSENS_ACTUATORFRC
        16 |  // mjSENS_JOINTACTFRC
        17 |  // mjSENS_TENDONACTFRC
        22 |  // mjSENS_JOINTLIMITFRC
        25 |  // mjSENS_TENDONLIMITFRC
        33 |  // mjSENS_FRAMELINACC
        34 |  // mjSENS_FRAMEANGACC
        42 |  // mjSENS_CONTACT
        46    // mjSENS_TACTILE
            => 3,  // mjSTAGE_ACC

        2  |  // mjSENS_VELOCIMETER
        3  |  // mjSENS_GYRO
        10 |  // mjSENS_JOINTVEL
        12 |  // mjSENS_TENDONVEL
        14 |  // mjSENS_ACTUATORVEL
        19 |  // mjSENS_BALLANGVEL
        21 |  // mjSENS_JOINTLIMITVEL
        24 |  // mjSENS_TENDONLIMITVEL
        31 |  // mjSENS_FRAMELINVEL
        32 |  // mjSENS_FRAMEANGVEL
        36 |  // mjSENS_SUBTREELINVEL
        37    // mjSENS_SUBTREEANGMOM
            => 2,  // mjSTAGE_VEL

        _ => 1,  // mjSTAGE_POS
    }
}

/// C: ResolveOrientation (user/user_objects.h:89)
/// Calls: mjuu_copyvec, mjuu_crossvec, mjuu_dot3, mjuu_frame2quat, mjuu_mulquat, mjuu_normvec, mjuu_setvec, mjuu_z2quat
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn resolve_orientation(quat: *mut f64, degree: bool, sequence: *const i8, orient: *const mjsOrientation) -> *const i8 {
    todo!() // ResolveOrientation
}

/// C: mjCBoundingVolume::Contype (user/user_objects.h:122)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_contype(self_ptr: *mut mjCBoundingVolume) -> i32 {
    // MACHINE-PROVEN: Clang AST direct field access
    unsafe { (*self_ptr).contype_ }
}

/// C: mjCBoundingVolume::Conaffinity (user/user_objects.h:123)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_conaffinity(self_ptr: *mut mjCBoundingVolume) -> i32 {
    // MACHINE-PROVEN: Clang AST direct field access
    unsafe { (*self_ptr).conaffinity_ }
}

/// C: mjCBoundingVolume::AABB (user/user_objects.h:124)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_aabb(self_ptr: *mut mjCBoundingVolume) -> *const f64 {
    todo!() // mjCBoundingVolume::AABB
}

/// C: mjCBoundingVolume::Pos (user/user_objects.h:126)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_pos(self_ptr: *mut mjCBoundingVolume) -> *const f64 {
    todo!() // mjCBoundingVolume::Pos
}

/// C: mjCBoundingVolume::Quat (user/user_objects.h:128)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_quat(self_ptr: *mut mjCBoundingVolume) -> *const f64 {
    todo!() // mjCBoundingVolume::Quat
}

/// C: mjCBoundingVolume::Id (user/user_objects.h:129)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_id(self_ptr: *mut mjCBoundingVolume) -> *const i32 {
    todo!() // mjCBoundingVolume::Id
}

/// C: mjCBoundingVolume::SetContype (user/user_objects.h:131)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_set_contype(self_ptr: *mut mjCBoundingVolume, val: i32) {
    // MACHINE-PROVEN: Clang AST direct field access
    unsafe { (*self_ptr).contype_ = val; }
}

/// C: mjCBoundingVolume::SetConaffinity (user/user_objects.h:132)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_set_conaffinity(self_ptr: *mut mjCBoundingVolume, val: i32) {
    // MACHINE-PROVEN: Clang AST direct field access
    unsafe { (*self_ptr).conaffinity_ = val; }
}

/// C: mjCBoundingVolume::SetAABB (user/user_objects.h:133)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_set_aabb(self_ptr: *mut mjCBoundingVolume, aabb: *const f64) {
    todo!() // mjCBoundingVolume::SetAABB
}

/// C: mjCBoundingVolume::SetPos (user/user_objects.h:134)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_set_pos(self_ptr: *mut mjCBoundingVolume, pos: *const f64) {
    todo!() // mjCBoundingVolume::SetPos
}

/// C: mjCBoundingVolume::SetQuat (user/user_objects.h:135)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_set_quat(self_ptr: *mut mjCBoundingVolume, quat: *const f64) {
    todo!() // mjCBoundingVolume::SetQuat
}

/// C: mjCBoundingVolume::SetId (user/user_objects.h:139)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_set_id(self_ptr: *mut mjCBoundingVolume, id: *const i32) {
    // MACHINE-PROVEN: Clang AST direct field access
    unsafe { (*self_ptr).id_ = id; }
}

/// C: mjCBoundingVolumeHierarchy::CreateBVH (user/user_objects.h:174)
/// Calls: mjCBoundingVolumeHierarchy::Make, mjCBoundingVolumeHierarchy::MakeBVH
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_hierarchy_create_bvh(self_ptr: *mut mjCBoundingVolumeHierarchy, model: *mut mjCModel, owner: *const mjCBase) {
    todo!() // mjCBoundingVolumeHierarchy::CreateBVH
}

/// C: mjCBoundingVolumeHierarchy::Set (user/user_objects.h:175)
/// Calls: mjuu_copyvec
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_hierarchy_set(self_ptr: *mut mjCBoundingVolumeHierarchy, ipos_element: *mut f64, iquat_element: *mut f64) {
    todo!() // mjCBoundingVolumeHierarchy::Set
}

/// C: mjCBoundingVolumeHierarchy::AllocateBoundingVolumes (user/user_objects.h:176)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_hierarchy_allocate_bounding_volumes(self_ptr: *mut mjCBoundingVolumeHierarchy, nleaf: i32) {
    todo!() // mjCBoundingVolumeHierarchy::AllocateBoundingVolumes
}

/// C: mjCBoundingVolumeHierarchy::RemoveInactiveVolumes (user/user_objects.h:177)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_hierarchy_remove_inactive_volumes(self_ptr: *mut mjCBoundingVolumeHierarchy, nmax: i32) {
    todo!() // mjCBoundingVolumeHierarchy::RemoveInactiveVolumes
}

/// C: mjCBoundingVolumeHierarchy::AddBoundingVolume (user/user_objects.h:179)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_hierarchy_add_bounding_volume(self_ptr: *mut mjCBoundingVolumeHierarchy, id: i32, contype: i32, conaffinity: i32, pos: *const f64, quat: *const f64, aabb: *const f64) -> *const mjCBoundingVolume {
    todo!() // mjCBoundingVolumeHierarchy::AddBoundingVolume
}

/// C: mjCBoundingVolumeHierarchy::Nbvh (user/user_objects.h:186)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_hierarchy_nbvh(self_ptr: *mut mjCBoundingVolumeHierarchy) -> i32 {
    // MACHINE-PROVEN: Clang AST direct field access
    unsafe { (*self_ptr).nbvh_ }
}

/// C: mjCBoundingVolumeHierarchy::Bvh (user/user_objects.h:187)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_hierarchy_bvh(self_ptr: *mut mjCBoundingVolumeHierarchy) -> *const () {
    todo!() // mjCBoundingVolumeHierarchy::Bvh
}

/// C: mjCBoundingVolumeHierarchy::Child (user/user_objects.h:188)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_hierarchy_child(self_ptr: *mut mjCBoundingVolumeHierarchy) -> *const () {
    todo!() // mjCBoundingVolumeHierarchy::Child
}

/// C: mjCBoundingVolumeHierarchy::Nodeid (user/user_objects.h:189)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_hierarchy_nodeid(self_ptr: *mut mjCBoundingVolumeHierarchy) -> *const () {
    todo!() // mjCBoundingVolumeHierarchy::Nodeid
}

/// C: mjCBoundingVolumeHierarchy::Nodeidptr (user/user_objects.h:191)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_hierarchy_nodeidptr(self_ptr: *mut mjCBoundingVolumeHierarchy, id: i32) -> *const i32 {
    todo!() // mjCBoundingVolumeHierarchy::Nodeidptr
}

/// C: mjCBoundingVolumeHierarchy::Level (user/user_objects.h:192)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_hierarchy_level(self_ptr: *mut mjCBoundingVolumeHierarchy) -> *const () {
    todo!() // mjCBoundingVolumeHierarchy::Level
}

/// C: mjCBoundingVolumeHierarchy::Size (user/user_objects.h:193)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_hierarchy_size(self_ptr: *mut mjCBoundingVolumeHierarchy) -> i32 {
    todo!() // mjCBoundingVolumeHierarchy::Size
}

/// C: mjCBoundingVolumeHierarchy::QuerySignedDistance (user/user_objects.h:200)
/// Calls: querySignedDistance
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_hierarchy_query_signed_distance(self_ptr: *mut mjCBoundingVolumeHierarchy, point: *const f64, vert: *const f64, face: *const i32) -> f64 {
    todo!() // mjCBoundingVolumeHierarchy::QuerySignedDistance
}

/// C: mjCBoundingVolumeHierarchy::Make (user/user_objects.h:210)
/// Calls: mjCBoundingVolume::Conaffinity, mjCBoundingVolume::Contype, mjCBoundingVolume::Pos, mjuu_rotVecQuat
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_hierarchy_make(self_ptr: *mut mjCBoundingVolumeHierarchy, elements: *const ()) {
    todo!() // mjCBoundingVolumeHierarchy::Make
}

/// C: mjCBoundingVolumeHierarchy::MakeBVH (user/user_objects.h:211)
/// Calls: mjCBoundingVolume::AABB, mjCBoundingVolume::Id, mjCBoundingVolume::Pos, mjCBoundingVolume::Quat, mjCModel::AddWarning, mju_error, mjuu_rotVecQuat
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_bounding_volume_hierarchy_make_bvh(self_ptr: *mut mjCBoundingVolumeHierarchy, elements_begin: *const (), elements_end: *const (), lev: i32, model: *mut mjCModel, owner: *const mjCBase) -> i32 {
    todo!() // mjCBoundingVolumeHierarchy::MakeBVH
}

/// C: mjCOctree::CreateOctree (user/user_objects.h:285)
/// Calls: mjCOctree::Clear, mjCOctree::Make, mjCOctree::MakeOctree, mjCOctree::MarkHangingNodes
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_create_octree(self_ptr: *mut mjCOctree, aamm: *const f64) {
    todo!() // mjCOctree::CreateOctree
}

/// C: mjCOctree::NumNodes (user/user_objects.h:287)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_num_nodes(self_ptr: *mut mjCOctree) -> i32 {
    // MACHINE-PROVEN: Clang AST direct field access
    unsafe { (*self_ptr).nnode_ }
}

/// C: mjCOctree::NumVerts (user/user_objects.h:288)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_num_verts(self_ptr: *mut mjCOctree) -> i32 {
    // MACHINE-PROVEN: Clang AST direct field access
    unsafe { (*self_ptr).nvert_ }
}

/// C: mjCOctree::CopyLevel (user/user_objects.h:289)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_copy_level(self_ptr: *mut mjCOctree, level: *mut i32) {
    todo!() // mjCOctree::CopyLevel
}

/// C: mjCOctree::CopyChild (user/user_objects.h:290)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_copy_child(self_ptr: *mut mjCOctree, child: *mut i32) {
    todo!() // mjCOctree::CopyChild
}

/// C: mjCOctree::CopyAabb (user/user_objects.h:291)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_copy_aabb(self_ptr: *mut mjCOctree, aabb: *mut f64) {
    todo!() // mjCOctree::CopyAabb
}

/// C: mjCOctree::CopyCoeff (user/user_objects.h:292)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_copy_coeff(self_ptr: *mut mjCOctree, coeff: *mut f64) {
    todo!() // mjCOctree::CopyCoeff
}

/// C: mjCOctree::Vert (user/user_objects.h:293)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_vert(self_ptr: *mut mjCOctree, i: i32) -> *const f64 {
    todo!() // mjCOctree::Vert
}

/// C: mjCOctree::Hang (user/user_objects.h:294)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_hang(self_ptr: *mut mjCOctree, i: i32) -> *const () {
    todo!() // mjCOctree::Hang
}

/// C: mjCOctree::VertId (user/user_objects.h:295)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_vert_id(self_ptr: *mut mjCOctree, n: i32, v: i32) -> i32 {
    todo!() // mjCOctree::VertId
}

/// C: mjCOctree::Children (user/user_objects.h:296)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_children(self_ptr: *mut mjCOctree, i: i32) -> *const () {
    todo!() // mjCOctree::Children
}

/// C: mjCOctree::SetFace (user/user_objects.h:297)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_set_face(self_ptr: *mut mjCOctree, vert: *const (), face: *const ()) {
    todo!() // mjCOctree::SetFace
}

/// C: mjCOctree::Size (user/user_objects.h:298)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_size(self_ptr: *mut mjCOctree) -> i32 {
    todo!() // mjCOctree::Size
}

/// C: mjCOctree::Clear (user/user_objects.h:302)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_clear(self_ptr: *mut mjCOctree) {
    todo!() // mjCOctree::Clear
}

/// C: mjCOctree::AddCoeff (user/user_objects.h:309)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_add_coeff(self_ptr: *mut mjCOctree, n: i32, v: i32, coeff: f64) {
    todo!() // mjCOctree::AddCoeff
}

/// C: mjCOctree::Coeff (user/user_objects.h:310)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_coeff(self_ptr: *mut mjCOctree, n: i32, v: i32) -> f64 {
    todo!() // mjCOctree::Coeff
}

/// C: mjCOctree::SetMaxDepth (user/user_objects.h:313)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_set_max_depth(self_ptr: *mut mjCOctree, depth: i32) {
    // MACHINE-PROVEN: Clang AST direct field access
    unsafe { (*self_ptr).max_depth_ = depth; }
}

/// C: mjCOctree::MaxDepth (user/user_objects.h:314)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_max_depth(self_ptr: *mut mjCOctree) -> i32 {
    // MACHINE-PROVEN: Clang AST direct field access
    unsafe { (*self_ptr).max_depth_ }
}

/// C: mjCOctree::SetSmoothingIterations (user/user_objects.h:317)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_set_smoothing_iterations(self_ptr: *mut mjCOctree, iterations: i32) {
    // MACHINE-PROVEN: Clang AST direct field access
    unsafe { (*self_ptr).smoothing_iterations_ = iterations; }
}

/// C: mjCOctree::SmoothingIterations (user/user_objects.h:318)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_smoothing_iterations(self_ptr: *mut mjCOctree) -> i32 {
    // MACHINE-PROVEN: Clang AST direct field access
    unsafe { (*self_ptr).smoothing_iterations_ }
}

/// C: mjCOctree::ComputeSdfCoeffs (user/user_objects.h:321)
/// Calls: mjCBoundingVolumeHierarchy::QuerySignedDistance, mjCOctree::AddCoeff, mjCOctree::Children, mjCOctree::Hang, mjCOctree::NumNodes, mjCOctree::Vert, mjCOctree::VertId, mjuu_rotVecQuat
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_compute_sdf_coeffs(self_ptr: *mut mjCOctree, vert: *const f64, nvert: i32, face: *const i32, nface: i32, tree: *const mjCBoundingVolumeHierarchy) {
    todo!() // mjCOctree::ComputeSdfCoeffs
}

/// C: mjCOctree::Make (user/user_objects.h:325)
/// Calls: mjuu_rotVecQuat
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_make(self_ptr: *mut mjCOctree, elements: *const ()) {
    todo!() // mjCOctree::Make
}

/// C: mjCOctree::MakeOctree (user/user_objects.h:326)
/// Calls: boxTriangle, mjCOctree::BalanceOctree, mjCOctree::Subdivide, mjCOctree::TaskToNode
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_make_octree(self_ptr: *mut mjCOctree, elements: *const (), aamm: *const f64, vert_map: *const ()) {
    todo!() // mjCOctree::MakeOctree
}

/// C: mjCOctree::TaskToNode (user/user_objects.h:328)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_task_to_node(self_ptr: *mut mjCOctree, task: *const OctreeTask, node: *mut OctNode, vert_map: *const ()) {
    todo!() // mjCOctree::TaskToNode
}

/// C: mjCOctree::Subdivide (user/user_objects.h:329)
/// Calls: mjCOctree::TaskToNode
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_subdivide(self_ptr: *mut mjCOctree, task: *const OctreeTask, vert_map: *const (), queue: *const (), colliding: *const ()) {
    todo!() // mjCOctree::Subdivide
}

/// C: mjCOctree::FindNeighbor (user/user_objects.h:332)
/// Calls: mjCOctree::FindCoarseNeighbor
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_find_neighbor(self_ptr: *mut mjCOctree, node_idx: i32, dir: i32) -> i32 {
    todo!() // mjCOctree::FindNeighbor
}

/// C: mjCOctree::FindCoarseNeighbor (user/user_objects.h:333)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_find_coarse_neighbor(self_ptr: *mut mjCOctree, node_idx: i32, dir: i32) -> i32 {
    todo!() // mjCOctree::FindCoarseNeighbor
}

/// C: mjCOctree::BalanceOctree (user/user_objects.h:334)
/// Calls: mjCOctree::FindNeighbor, mjCOctree::Subdivide
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_balance_octree(self_ptr: *mut mjCOctree, vert_map: *const ()) {
    todo!() // mjCOctree::BalanceOctree
}

/// C: mjCOctree::MarkHangingNodes (user/user_objects.h:335)
/// Calls: mjCOctree::FindNeighbor
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_octree_mark_hanging_nodes(self_ptr: *mut mjCOctree) {
    todo!() // mjCOctree::MarkHangingNodes
}

/// C: mjCBase::LoadResource (user/user_objects.h:358)
/// Calls: mju_openResource
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_base_load_resource(modelfiledir: *const std__string, filename: *const std__string, vfs: *const mjVFS) -> *mut mjResource {
    todo!() // mjCBase::LoadResource
}

/// C: mjCBase::GetAssetContentType (user/user_objects.h:363)
/// Calls: mjuu_extToContentType, mjuu_parseContentTypeAttrSubtype, mjuu_parseContentTypeAttrType
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_base_get_asset_content_type(resource_name: std__string_view, raw_text: std__string_view) -> std__string {
    todo!() // mjCBase::GetAssetContentType
}

/// C: mjCBase::SetFrame (user/user_objects.h:366)
/// Calls: mjCBase::GetParent
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_base_set_frame(self_ptr: *mut mjCBase, _frame: *mut mjCFrame) {
    todo!() // mjCBase::SetFrame
}

/// C: mjCBase::CopyFromSpec (user/user_objects.h:369)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_base_copy_from_spec(self_ptr: *mut mjCBase) {
    todo!() // mjCBase::CopyFromSpec
}

/// C: mjCBase::ResolveReferences (user/user_objects.h:372)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_base_resolve_references(self_ptr: *mut mjCBase, m: *const mjCModel) {
    todo!() // mjCBase::ResolveReferences
}

/// C: mjCBase::NameSpace (user/user_objects.h:375)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_base_name_space(self_ptr: *mut mjCBase, m: *const mjCModel) {
    todo!() // mjCBase::NameSpace
}

/// C: mjCBase::CopyPlugin (user/user_objects.h:378)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_base_copy_plugin(self_ptr: *mut mjCBase) {
    todo!() // mjCBase::CopyPlugin
}

/// C: mjCBase::GetParent (user/user_objects.h:381)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_base_get_parent(self_ptr: *mut mjCBase) -> *mut mjCBase {
    todo!() // mjCBase::GetParent
}

/// C: mjCBase::FindCompiler (user/user_objects.h:384)
/// Calls: mjCModel::FindSpec
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_base_find_compiler(self_ptr: *mut mjCBase, compiler: *const mjsCompiler) -> *mut mjsCompiler {
    todo!() // mjCBase::FindCompiler
}

/// C: mjCBase::ForgetKeyframes (user/user_objects.h:396)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_base_forget_keyframes(self_ptr: *mut mjCBase) {
    todo!() // mjCBase::ForgetKeyframes
}

/// C: mjCBase::AddRef (user/user_objects.h:402)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_base_add_ref(self_ptr: *mut mjCBase) {
    todo!() // mjCBase::AddRef
}

/// C: mjCBase::GetRef (user/user_objects.h:403)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_base_get_ref(self_ptr: *mut mjCBase) -> i32 {
    // MACHINE-PROVEN: Clang AST direct field access
    unsafe { (*self_ptr).refcount }
}

/// C: mjCBase::Release (user/user_objects.h:404)
/// Calls: GlobalTable::count
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_base_release(self_ptr: *mut mjCBase) {
    todo!() // mjCBase::Release
}

/// C: mjCBase::SetUserValue (user/user_objects.h:411)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_base_set_user_value(self_ptr: *mut mjCBase, key: std__string_view, data: *const (), cleanup: Option<unsafe extern "C" fn()>) {
    todo!() // mjCBase::SetUserValue
}

/// C: mjCBase::GetUserValue (user/user_objects.h:413)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_base_get_user_value(self_ptr: *mut mjCBase, key: std__string_view) -> *const () {
    todo!() // mjCBase::GetUserValue
}

/// C: mjCBase::DeleteUserValue (user/user_objects.h:414)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_base_delete_user_value(self_ptr: *mut mjCBase, key: std__string_view) {
    todo!() // mjCBase::DeleteUserValue
}

/// C: mjCBody::AddBody (user/user_objects.h:522)
/// Calls: mjCModel::MakeTreeLists, mjCModel::ResetTreeLists, mjCModel::Signature
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_add_body(self_ptr: *mut mjCBody, arg0: *mut mjCDef) -> *mut mjCBody {
    todo!() // mjCBody::AddBody
}

/// C: mjCBody::AddFrame (user/user_objects.h:523)
/// Calls: mjCModel::MakeTreeLists, mjCModel::ResetTreeLists, mjCModel::Signature
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_add_frame(self_ptr: *mut mjCBody, arg0: *mut mjCFrame) -> *mut mjCFrame {
    todo!() // mjCBody::AddFrame
}

/// C: mjCBody::AddJoint (user/user_objects.h:524)
/// Calls: mjCModel::MakeTreeLists, mjCModel::ResetTreeLists, mjCModel::Signature
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_add_joint(self_ptr: *mut mjCBody, arg0: *mut mjCDef) -> *mut mjCJoint {
    todo!() // mjCBody::AddJoint
}

/// C: mjCBody::AddFreeJoint (user/user_objects.h:525)
/// Calls: mjCModel::MakeTreeLists, mjCModel::ResetTreeLists, mjCModel::Signature
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_add_free_joint(self_ptr: *mut mjCBody) -> *mut mjCJoint {
    todo!() // mjCBody::AddFreeJoint
}

/// C: mjCBody::AddGeom (user/user_objects.h:526)
/// Calls: mjCModel::MakeTreeLists, mjCModel::ResetTreeLists, mjCModel::Signature
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_add_geom(self_ptr: *mut mjCBody, arg0: *mut mjCDef) -> *mut mjCGeom {
    todo!() // mjCBody::AddGeom
}

/// C: mjCBody::AddSite (user/user_objects.h:527)
/// Calls: mjCModel::MakeTreeLists, mjCModel::ResetTreeLists, mjCModel::Signature
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_add_site(self_ptr: *mut mjCBody, arg0: *mut mjCDef) -> *mut mjCSite {
    todo!() // mjCBody::AddSite
}

/// C: mjCBody::AddCamera (user/user_objects.h:528)
/// Calls: mjCModel::MakeTreeLists, mjCModel::ResetTreeLists, mjCModel::Signature
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_add_camera(self_ptr: *mut mjCBody, arg0: *mut mjCDef) -> *mut mjCCamera {
    todo!() // mjCBody::AddCamera
}

/// C: mjCBody::AddLight (user/user_objects.h:529)
/// Calls: mjCModel::MakeTreeLists, mjCModel::ResetTreeLists, mjCModel::Signature
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_add_light(self_ptr: *mut mjCBody, arg0: *mut mjCDef) -> *mut mjCLight {
    todo!() // mjCBody::AddLight
}

/// C: mjCBody::NumObjects (user/user_objects.h:537)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_num_objects(self_ptr: *mut mjCBody, r#type: u32) -> i32 {
    todo!() // mjCBody::NumObjects
}

/// C: mjCBody::GetObject (user/user_objects.h:538)
/// Calls: mjCBody::NumObjects
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_get_object(self_ptr: *mut mjCBody, r#type: u32, id: i32) -> *mut mjCBase {
    todo!() // mjCBody::GetObject
}

/// C: mjCBody::FindObject (user/user_objects.h:539)
/// Calls: findobject
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_find_object(self_ptr: *mut mjCBody, r#type: u32, name: *const std__string, recursive: bool) -> *mut mjCBase {
    todo!() // mjCBody::FindObject
}

/// C: mjCBody::NameSpace (user/user_objects.h:542)
/// Calls: mjCBody::NameSpace_
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_name_space(self_ptr: *mut mjCBody, m: *const mjCModel) {
    todo!() // mjCBody::NameSpace
}

/// C: mjCBody::MakeInertialExplicit (user/user_objects.h:545)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_make_inertial_explicit(self_ptr: *mut mjCBody) {
    // SAFETY: caller guarantees self_ptr is a valid pointer to mjCBody
    unsafe {
        (*self_ptr).spec.explicitinertial = 1;
    }
}

/// C: mjCBody::ComputeBVH (user/user_objects.h:548)
/// Calls: mjCBoundingVolumeHierarchy::AddBoundingVolume, mjCBoundingVolumeHierarchy::AllocateBoundingVolumes, mjCBoundingVolumeHierarchy::CreateBVH, mjCBoundingVolumeHierarchy::Set
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_compute_bvh(self_ptr: *mut mjCBody) {
    todo!() // mjCBody::ComputeBVH
}

/// C: mjCBody::get_userdata (user/user_objects.h:557)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_get_userdata(self_ptr: *mut mjCBody) -> *const () {
    todo!() // mjCBody::get_userdata
}

/// C: mjCBody::NextChild (user/user_objects.h:563)
/// Calls: GetNext, GetNextBody
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_next_child(self_ptr: *mut mjCBody, child: *const mjsElement, r#type: u32, recursive: bool) -> *mut mjsElement {
    todo!() // mjCBody::NextChild
}

/// C: mjCBody::ForgetKeyframes (user/user_objects.h:567)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_forget_keyframes(self_ptr: *mut mjCBody) {
    todo!() // mjCBody::ForgetKeyframes
}

/// C: mjCBody::ToFrame (user/user_objects.h:570)
/// Calls: MapFrame, mjCBody::AccumulateInertia, mjCBody::AddFrame, mjCBody::MakeInertialExplicit, mjCModel::MakeTreeLists, mjCModel::ResetTreeLists, mjCModel::Signature, mjuu_copyvec, mjuu_zerovec
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_to_frame(self_ptr: *mut mjCBody) -> *mut mjCFrame {
    todo!() // mjCBody::ToFrame
}

/// C: mjCBody::mpos (user/user_objects.h:573)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_mpos(self_ptr: *mut mjCBody, state_name: *const std__string) -> *mut f64 {
    todo!() // mjCBody::mpos
}

/// C: mjCBody::mquat (user/user_objects.h:574)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_mquat(self_ptr: *mut mjCBody, state_name: *const std__string) -> *mut f64 {
    todo!() // mjCBody::mquat
}

/// C: mjCBody::SetParent (user/user_objects.h:579)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_set_parent(self_ptr: *mut mjCBody, _body: *mut mjCBody) {
    // MACHINE-PROVEN: Clang AST direct field access
    unsafe { (*self_ptr).parent = _body; }
}

/// C: mjCBody::GetParent (user/user_objects.h:580)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_get_parent(self_ptr: *mut mjCBody) -> *mut mjCBody {
    // MACHINE-PROVEN: Clang AST direct field access
    unsafe { (*self_ptr).parent }
}

/// C: mjCBody::SetModel (user/user_objects.h:583)
/// Calls: mjCModel::FindSpec
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_set_model(self_ptr: *mut mjCBody, _model: *mut mjCModel) {
    todo!() // mjCBody::SetModel
}

/// C: mjCBody::ResetId (user/user_objects.h:586)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_reset_id(self_ptr: *mut mjCBody) {
    todo!() // mjCBody::ResetId
}

/// C: mjCBody::Bodies (user/user_objects.h:589)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_bodies(self_ptr: *mut mjCBody) -> *const () {
    todo!() // mjCBody::Bodies
}

/// C: mjCBody::AccumulateInertia (user/user_objects.h:597)
/// Calls: mjuu_copyvec, mjuu_frameaccum, mjuu_fullInertia, mjuu_globalinertia, mjuu_offcenter, mjuu_setvec
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_accumulate_inertia(self_ptr: *mut mjCBody, other: *const mjsBody, result: *mut mjsBody) {
    todo!() // mjCBody::AccumulateInertia
}

/// C: mjCBody::Compile (user/user_objects.h:603)
/// Calls: ResolveOrientation, mjCBody::ComputeBVH, mjCBody::CopyFromSpec, mjCBody::InertiaFromGeom, mjCCamera::Compile, mjCFrame::Compile, mjCGeom::Compile, mjCJoint::Compile, mjCLight::Compile, mjCModel::ResolvePlugin, mjCSite::Compile, mjp_getPluginAtSlot, mjuu_addtovec, mjuu_copyvec, mjuu_defined, mjuu_frameaccum, mjuu_frameaccumChild, mjuu_frameinvert, mjuu_fullInertia, mjuu_mulquat, mjuu_normvec, mjuu_rotVecQuat, mjuu_setvec
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_compile(self_ptr: *mut mjCBody) {
    todo!() // mjCBody::Compile
}

/// C: mjCBody::InertiaFromGeom (user/user_objects.h:604)
/// Calls: mjuu_copyvec, mjuu_fullInertia, mjuu_globalinertia, mjuu_offcenter
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_inertia_from_geom(self_ptr: *mut mjCBody) {
    todo!() // mjCBody::InertiaFromGeom
}

/// C: mjCBody::CopyFromSpec (user/user_objects.h:615)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_copy_from_spec(self_ptr: *mut mjCBody) {
    todo!() // mjCBody::CopyFromSpec
}

/// C: mjCBody::PointToLocal (user/user_objects.h:616)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_point_to_local(self_ptr: *mut mjCBody) {
    // SAFETY: caller guarantees self_ptr is a valid pointer to mjCBody.
    // Offsets: mjsElement at +8, classname at +56, info at +80 (in _pad_0/mjCBase).
    unsafe {
        let base = self_ptr as *mut u8;
        (*self_ptr).spec.element = base.add(8) as *mut mjsElement;
        (*self_ptr).spec.childclass = base.add(56) as *mut mjString;
        (*self_ptr).spec.userdata = std::ptr::addr_of_mut!((*self_ptr).spec_userdata_) as *mut mjDoubleVec;
        (*self_ptr).spec.plugin.plugin_name = std::ptr::addr_of_mut!((*self_ptr).plugin_name) as *mut mjString;
        (*self_ptr).spec.plugin.name = std::ptr::addr_of_mut!((*self_ptr).plugin_instance_name) as *mut mjString;
        (*self_ptr).spec.info = base.add(80) as *mut mjString;
        (*self_ptr).userdata = std::ptr::null_mut();
    }
}

/// C: mjCBody::NameSpace_ (user/user_objects.h:617)
/// Calls: mjCBase::NameSpace, mjCCamera::NameSpace, mjCGeom::NameSpace, mjCLight::NameSpace, mjCSite::NameSpace
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_name_space_1(self_ptr: *mut mjCBody, m: *const mjCModel, propagate: bool) {
    todo!() // mjCBody::NameSpace_
}

/// C: mjCBody::CopyPlugin (user/user_objects.h:618)
/// Calls: mjCModel::CopyExplicitPlugin
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_copy_plugin(self_ptr: *mut mjCBody) {
    todo!() // mjCBody::CopyPlugin
}

/// C: mjCBody::CopyList (user/user_objects.h:622)
/// Calls: mjCFrame::IsAncestor
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_copy_list(self_ptr: *mut mjCBody, dst: *const (), src: *const (), fmap: *const (), pframe: *const mjCFrame) {
    todo!() // mjCBody::CopyList
}

/// C: mjCFrame::CopyFromSpec (user/user_objects.h:654)
/// Calls: mjuu_copyvec
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_frame_copy_from_spec(self_ptr: *mut mjCFrame) {
    todo!() // mjCFrame::CopyFromSpec
}

/// C: mjCFrame::PointToLocal (user/user_objects.h:655)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_frame_point_to_local(self_ptr: *mut mjCFrame) {
    // SAFETY: caller guarantees self_ptr is a valid pointer to mjCFrame.
    // Offsets: mjsElement base at +8, classname at +56, info at +80 (within _pad_0/mjCBase).
    unsafe {
        let base = self_ptr as *mut u8;
        (*self_ptr).spec.element = base.add(8) as *mut mjsElement;
        (*self_ptr).spec.childclass = base.add(56) as *mut mjString;
        (*self_ptr).spec.info = base.add(80) as *mut mjString;
    }
}

/// C: mjCFrame::SetParent (user/user_objects.h:656)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_frame_set_parent(self_ptr: *mut mjCFrame, _body: *mut mjCBody) {
    // MACHINE-PROVEN: Clang AST direct field access
    unsafe { (*self_ptr).body = _body; }
}

/// C: mjCFrame::GetParent (user/user_objects.h:657)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_frame_get_parent(self_ptr: *mut mjCFrame) -> *mut mjCBody {
    // MACHINE-PROVEN: Clang AST direct field access
    unsafe { (*self_ptr).body }
}

/// C: mjCFrame::IsAncestor (user/user_objects.h:661)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_frame_is_ancestor(self_ptr: *mut mjCFrame, child: *const mjCFrame) -> bool {
    // SAFETY: caller guarantees self_ptr is valid; child may be null (checked below).
    // The frame pointer is at offset 152 within the object (mjCBase::frame field inside _pad_0).
    unsafe {
        if child.is_null() {
            return false;
        }
        if child as *const u8 == self_ptr as *const u8 {
            return true;
        }
        // Access mjCBase::frame at offset 8 (vtable) + 144 (within _pad_0) = 152 from struct start
        let child_frame = *((child as *const u8).add(152) as *const *mut mjCFrame);
        mj_c_frame_is_ancestor(self_ptr, child_frame)
    }
}

/// C: mjCFrame::Compile (user/user_objects.h:666)
/// Calls: ResolveOrientation, mjCFrame::CopyFromSpec, mjuu_frameaccumChild, mjuu_normvec
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_frame_compile(self_ptr: *mut mjCFrame) {
    todo!() // mjCFrame::Compile
}

/// C: mjCJoint::CopyFromSpec (user/user_objects.h:706)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_joint_copy_from_spec(self_ptr: *mut mjCJoint) {
    todo!() // mjCJoint::CopyFromSpec
}

/// C: mjCJoint::SetParent (user/user_objects.h:707)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_joint_set_parent(self_ptr: *mut mjCJoint, _body: *mut mjCBody) {
    // MACHINE-PROVEN: Clang AST direct field access
    unsafe { (*self_ptr).body = _body; }
}

/// C: mjCJoint::GetParent (user/user_objects.h:708)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_joint_get_parent(self_ptr: *mut mjCJoint) -> *mut mjCBody {
    // MACHINE-PROVEN: Clang AST direct field access
    unsafe { (*self_ptr).body }
}

/// C: mjCJoint::get_userdata (user/user_objects.h:711)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_joint_get_userdata(self_ptr: *mut mjCJoint) -> *const () {
    todo!() // mjCJoint::get_userdata
}

/// C: mjCJoint::get_range (user/user_objects.h:712)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_joint_get_range(self_ptr: *mut mjCJoint) -> *const f64 {
    // MACHINE-PROVEN: Clang AST direct field access
    unsafe { core::ptr::addr_of!((*self_ptr).range).cast::<f64>() }
}

/// C: mjCJoint::is_limited (user/user_objects.h:714)
/// Calls: islimited
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_joint_is_limited(self_ptr: *mut mjCJoint) -> bool {
    // SAFETY: self_ptr is valid pointer to mjCJoint
    unsafe {
        islimited((*self_ptr).limited, (*self_ptr).range.as_ptr())
    }
}

/// C: mjCJoint::is_actfrclimited (user/user_objects.h:715)
/// Calls: islimited
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_joint_is_actfrclimited(self_ptr: *mut mjCJoint) -> bool {
    // SAFETY: self_ptr is valid pointer to mjCJoint
    unsafe {
        islimited((*self_ptr).actfrclimited, (*self_ptr).actfrcrange.as_ptr())
    }
}

/// C: mjCJoint::nq (user/user_objects.h:719)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_joint_nq(self_ptr: *mut mjCJoint) -> i32 {
    // SAFETY: self_ptr is a valid mjCJoint pointer (caller contract).
    // spec.type is stored as [u8; 8], first 4 bytes encode mjtJoint (u32).
    unsafe {
        let joint_type: u32 = std::ptr::read((*self_ptr).spec.r#type.as_ptr() as *const u32);
        match joint_type {
            mjtJoint_mjJNT_FREE => 7,
            mjtJoint_mjJNT_BALL => 4,
            mjtJoint_mjJNT_SLIDE | mjtJoint_mjJNT_HINGE => 1,
            _ => 1,
        }
    }
}

/// C: mjCJoint::nv (user/user_objects.h:720)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_joint_nv(self_ptr: *mut mjCJoint) -> i32 {
    // SAFETY: self_ptr is a valid mjCJoint pointer (caller contract).
    // spec.type is stored as [u8; 8], first 4 bytes encode mjtJoint (u32).
    unsafe {
        let joint_type: u32 = std::ptr::read((*self_ptr).spec.r#type.as_ptr() as *const u32);
        match joint_type {
            mjtJoint_mjJNT_FREE => 6,
            mjtJoint_mjJNT_BALL => 3,
            mjtJoint_mjJNT_SLIDE | mjtJoint_mjJNT_HINGE => 1,
            _ => 1,
        }
    }
}

/// C: mjCJoint::qpos (user/user_objects.h:722)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_joint_qpos(self_ptr: *mut mjCJoint, state_name: *const std__string) -> *mut f64 {
    todo!() // mjCJoint::qpos
}

/// C: mjCJoint::qvel (user/user_objects.h:723)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_joint_qvel(self_ptr: *mut mjCJoint, state_name: *const std__string) -> *mut f64 {
    todo!() // mjCJoint::qvel
}

/// C: mjCJoint::Compile (user/user_objects.h:726)
/// Calls: checklimited, mjCJoint::CopyFromSpec, mjCJoint::is_actfrclimited, mjCJoint::is_limited, mjuu_frameaccumChild, mjuu_normvec, mjuu_rotVecQuat, mjuu_zerovec
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_joint_compile(self_ptr: *mut mjCJoint) -> i32 {
    todo!() // mjCJoint::Compile
}

/// C: mjCJoint::PointToLocal (user/user_objects.h:727)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_joint_point_to_local(self_ptr: *mut mjCJoint) {
    // SAFETY: caller guarantees self_ptr is a valid pointer to mjCJoint.
    // Offsets: mjsElement at +8, info at +80 (in _pad_0/mjCBase).
    unsafe {
        let base = self_ptr as *mut u8;
        (*self_ptr).spec.element = base.add(8) as *mut mjsElement;
        (*self_ptr).spec.userdata = std::ptr::addr_of_mut!((*self_ptr).spec_userdata_) as *mut mjDoubleVec;
        (*self_ptr).spec.info = base.add(80) as *mut mjString;
        (*self_ptr).userdata = std::ptr::null_mut();
    }
}

/// C: mjCGeom::GetVolume (user/user_objects.h:783)
/// Calls: mjCMesh::GetVolumeRef, mjCModel::Meshes
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_geom_get_volume(self_ptr: *mut mjCGeom) -> f64 {
    todo!() // mjCGeom::GetVolume
}

/// C: mjCGeom::SetInertia (user/user_objects.h:784)
/// Calls: mjCMesh::GetInertiaBoxPtr, mjCModel::Meshes
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_geom_set_inertia(self_ptr: *mut mjCGeom) {
    todo!() // mjCGeom::SetInertia
}

/// C: mjCGeom::IsVisual (user/user_objects.h:785)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_geom_is_visual(self_ptr: *mut mjCGeom) -> bool {
    // MACHINE-PROVEN: Clang AST direct field access
    unsafe { (*self_ptr).visual_ }
}

/// C: mjCGeom::SetNotVisual (user/user_objects.h:786)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_geom_set_not_visual(self_ptr: *mut mjCGeom) {
    // SAFETY: self_ptr is a valid pointer to mjCGeom provided by the caller
    unsafe { (*self_ptr).visual_ = false; }
}

/// C: mjCGeom::SetParent (user/user_objects.h:787)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_geom_set_parent(self_ptr: *mut mjCGeom, _body: *mut mjCBody) {
    // MACHINE-PROVEN: Clang AST direct field access
    unsafe { (*self_ptr).body = _body; }
}

/// C: mjCGeom::GetParent (user/user_objects.h:788)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_geom_get_parent(self_ptr: *mut mjCGeom) -> *mut mjCBody {
    // MACHINE-PROVEN: Clang AST direct field access
    unsafe { (*self_ptr).body }
}

/// C: mjCGeom::Type (user/user_objects.h:789)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_geom_type(self_ptr: *mut mjCGeom) -> u32 {
    // SAFETY: self_ptr is a valid pointer; type field stores mjtGeom (u32) in first 4 bytes
    unsafe {
        let type_bytes = &(*self_ptr).r#type;
        u32::from_ne_bytes([type_bytes[0], type_bytes[1], type_bytes[2], type_bytes[3]])
    }
}

/// C: mjCGeom::SetFluidCoefs (user/user_objects.h:792)
/// Calls: mjCGeom::GetAddedMassKappa, writeFluidGeomInteraction
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_geom_set_fluid_coefs(self_ptr: *mut mjCGeom) {
    todo!() // mjCGeom::SetFluidCoefs
}

/// C: mjCGeom::GetAddedMassKappa (user/user_objects.h:794)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_geom_get_added_mass_kappa(self_ptr: *mut mjCGeom, dx: f64, dy: f64, dz: f64) -> f64 {
    todo!() // mjCGeom::GetAddedMassKappa
}

/// C: mjCGeom::get_userdata (user/user_objects.h:797)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_geom_get_userdata(self_ptr: *mut mjCGeom) -> *const () {
    todo!() // mjCGeom::get_userdata
}

/// C: mjCGeom::get_hfieldname (user/user_objects.h:798)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_geom_get_hfieldname(self_ptr: *mut mjCGeom) -> *const std__string {
    todo!() // mjCGeom::get_hfieldname
}

/// C: mjCGeom::get_meshname (user/user_objects.h:799)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_geom_get_meshname(self_ptr: *mut mjCGeom) -> *const std__string {
    todo!() // mjCGeom::get_meshname
}

/// C: mjCGeom::get_material (user/user_objects.h:800)
/// Calls: mjCMesh::Material
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_geom_get_material(self_ptr: *mut mjCGeom) -> *const std__string {
    todo!() // mjCGeom::get_material
}

/// C: mjCGeom::del_material (user/user_objects.h:801)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_geom_del_material(self_ptr: *mut mjCGeom) {
    todo!() // mjCGeom::del_material
}

/// C: mjCGeom::Compile (user/user_objects.h:804)
/// Calls: ResolveOrientation, checksize, mjCGeom::ComputeAABB, mjCGeom::CopyFromSpec, mjCGeom::GetVolume, mjCGeom::SetFluidCoefs, mjCGeom::SetInertia, mjCMesh::FitGeom, mjCMesh::GetPosPtr, mjCMesh::GetQuatPtr, mjCMesh::aamm, mjCModel::ResolvePlugin, mjp_getPluginAtSlot, mjuu_addtovec, mjuu_defined, mjuu_frameaccum, mjuu_frameaccumChild, mjuu_normvec, mjuu_rotVecQuat, mjuu_z2quat
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_geom_compile(self_ptr: *mut mjCGeom) {
    todo!() // mjCGeom::Compile
}

/// C: mjCGeom::GetRBound (user/user_objects.h:805)
/// Calls: mjCMesh::aamm
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_geom_get_r_bound(self_ptr: *mut mjCGeom) -> f64 {
    todo!() // mjCGeom::GetRBound
}

/// C: mjCGeom::ComputeAABB (user/user_objects.h:806)
/// Calls: mjCMesh::aamm, mjuu_copyvec, mjuu_setvec
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_geom_compute_aabb(self_ptr: *mut mjCGeom) {
    todo!() // mjCGeom::ComputeAABB
}

/// C: mjCGeom::CopyFromSpec (user/user_objects.h:807)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_geom_copy_from_spec(self_ptr: *mut mjCGeom) {
    todo!() // mjCGeom::CopyFromSpec
}

/// C: mjCGeom::PointToLocal (user/user_objects.h:808)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_geom_point_to_local(self_ptr: *mut mjCGeom) {
    // SAFETY: caller guarantees self_ptr is a valid pointer to mjCGeom.
    // Offsets: mjsElement at +8, info at +80 (in _pad_0/mjCBase).
    unsafe {
        let base = self_ptr as *mut u8;
        (*self_ptr).spec.element = base.add(8) as *mut mjsElement;
        (*self_ptr).spec.info = base.add(80) as *mut mjString;
        (*self_ptr).spec.userdata = std::ptr::addr_of_mut!((*self_ptr).spec_userdata_) as *mut mjDoubleVec;
        (*self_ptr).spec.material = std::ptr::addr_of_mut!((*self_ptr).spec_material_) as *mut mjString;
        (*self_ptr).spec.meshname = std::ptr::addr_of_mut!((*self_ptr).spec_meshname_) as *mut mjString;
        (*self_ptr).spec.hfieldname = std::ptr::addr_of_mut!((*self_ptr).spec_hfieldname_) as *mut mjString;
        (*self_ptr).spec.plugin.plugin_name = std::ptr::addr_of_mut!((*self_ptr).plugin_name) as *mut mjString;
        (*self_ptr).spec.plugin.name = std::ptr::addr_of_mut!((*self_ptr).plugin_instance_name) as *mut mjString;
        (*self_ptr).userdata = std::ptr::null_mut();
        (*self_ptr).hfieldname = std::ptr::null_mut();
        (*self_ptr).meshname = std::ptr::null_mut();
        (*self_ptr).material = std::ptr::null_mut();
    }
}

/// C: mjCGeom::NameSpace (user/user_objects.h:809)
/// Calls: mjCBase::NameSpace
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_geom_name_space(self_ptr: *mut mjCGeom, m: *const mjCModel) {
    todo!() // mjCGeom::NameSpace
}

/// C: mjCGeom::CopyPlugin (user/user_objects.h:810)
/// Calls: mjCModel::CopyExplicitPlugin
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_geom_copy_plugin(self_ptr: *mut mjCGeom) {
    todo!() // mjCGeom::CopyPlugin
}

/// C: mjCSite::Body (user/user_objects.h:849)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_site_body(self_ptr: *mut mjCSite) -> *mut mjCBody {
    // MACHINE-PROVEN: Clang AST direct field access
    unsafe { (*self_ptr).body }
}

/// C: mjCSite::SetParent (user/user_objects.h:850)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_site_set_parent(self_ptr: *mut mjCSite, _body: *mut mjCBody) {
    // MACHINE-PROVEN: Clang AST direct field access
    unsafe { (*self_ptr).body = _body; }
}

/// C: mjCSite::GetParent (user/user_objects.h:851)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_site_get_parent(self_ptr: *mut mjCSite) -> *mut mjCBody {
    // MACHINE-PROVEN: Clang AST direct field access
    unsafe { (*self_ptr).body }
}

/// C: mjCSite::get_userdata (user/user_objects.h:857)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_site_get_userdata(self_ptr: *mut mjCSite) -> *const () {
    todo!() // mjCSite::get_userdata
}

/// C: mjCSite::get_material (user/user_objects.h:858)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_site_get_material(self_ptr: *mut mjCSite) -> *const std__string {
    todo!() // mjCSite::get_material
}

/// C: mjCSite::del_material (user/user_objects.h:859)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_site_del_material(self_ptr: *mut mjCSite) {
    todo!() // mjCSite::del_material
}

/// C: mjCSite::Compile (user/user_objects.h:862)
/// Calls: ResolveOrientation, checksize, mjCSite::CopyFromSpec, mjuu_defined, mjuu_frameaccumChild, mjuu_normvec, mjuu_z2quat
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_site_compile(self_ptr: *mut mjCSite) {
    todo!() // mjCSite::Compile
}

/// C: mjCSite::CopyFromSpec (user/user_objects.h:863)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_site_copy_from_spec(self_ptr: *mut mjCSite) {
    todo!() // mjCSite::CopyFromSpec
}

/// C: mjCSite::PointToLocal (user/user_objects.h:864)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_site_point_to_local(self_ptr: *mut mjCSite) {
    // SAFETY: caller guarantees self_ptr is a valid pointer to mjCSite.
    // Offsets: mjsElement at +8, info at +80 (in _pad_0/mjCBase).
    unsafe {
        let base = self_ptr as *mut u8;
        (*self_ptr).spec.element = base.add(8) as *mut mjsElement;
        (*self_ptr).spec.info = base.add(80) as *mut mjString;
        (*self_ptr).spec.material = std::ptr::addr_of_mut!((*self_ptr).spec_material_) as *mut mjString;
        (*self_ptr).spec.userdata = std::ptr::addr_of_mut!((*self_ptr).spec_userdata_) as *mut mjDoubleVec;
        (*self_ptr).userdata = std::ptr::null_mut();
        (*self_ptr).material = std::ptr::null_mut();
    }
}

/// C: mjCSite::NameSpace (user/user_objects.h:865)
/// Calls: mjCBase::NameSpace
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_site_name_space(self_ptr: *mut mjCSite, m: *const mjCModel) {
    todo!() // mjCSite::NameSpace
}

/// C: mjCCamera::get_targetbody (user/user_objects.h:899)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_camera_get_targetbody(self_ptr: *mut mjCCamera) -> *const std__string {
    todo!() // mjCCamera::get_targetbody
}

/// C: mjCCamera::get_userdata (user/user_objects.h:900)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_camera_get_userdata(self_ptr: *mut mjCCamera) -> *const () {
    todo!() // mjCCamera::get_userdata
}

/// C: mjCCamera::SetParent (user/user_objects.h:902)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_camera_set_parent(self_ptr: *mut mjCCamera, _body: *mut mjCBody) {
    // MACHINE-PROVEN: Clang AST direct field access
    unsafe { (*self_ptr).body = _body; }
}

/// C: mjCCamera::GetParent (user/user_objects.h:903)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_camera_get_parent(self_ptr: *mut mjCCamera) -> *mut mjCBody {
    // MACHINE-PROVEN: Clang AST direct field access
    unsafe { (*self_ptr).body }
}

/// C: mjCCamera::Compile (user/user_objects.h:906)
/// Calls: ResolveOrientation, mjCCamera::CopyFromSpec, mjCCamera::ResolveReferences, mjuu_frameaccumChild, mjuu_normvec
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_camera_compile(self_ptr: *mut mjCCamera) {
    todo!() // mjCCamera::Compile
}

/// C: mjCCamera::CopyFromSpec (user/user_objects.h:907)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_camera_copy_from_spec(self_ptr: *mut mjCCamera) {
    todo!() // mjCCamera::CopyFromSpec
}

/// C: mjCCamera::PointToLocal (user/user_objects.h:908)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_camera_point_to_local(self_ptr: *mut mjCCamera) {
    // SAFETY: caller guarantees self_ptr is a valid pointer to mjCCamera.
    // Offsets: mjsElement at +8, info at +80 (in _pad_0/mjCBase).
    unsafe {
        let base = self_ptr as *mut u8;
        (*self_ptr).spec.element = base.add(8) as *mut mjsElement;
        (*self_ptr).spec.userdata = std::ptr::addr_of_mut!((*self_ptr).spec_userdata_) as *mut mjDoubleVec;
        (*self_ptr).spec.targetbody = std::ptr::addr_of_mut!((*self_ptr).spec_targetbody_) as *mut mjString;
        (*self_ptr).spec.info = base.add(80) as *mut mjString;
        (*self_ptr).userdata = std::ptr::null_mut();
        (*self_ptr).targetbody = std::ptr::null_mut();
    }
}

/// C: mjCCamera::NameSpace (user/user_objects.h:909)
/// Calls: mjCBase::NameSpace
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_camera_name_space(self_ptr: *mut mjCCamera, m: *const mjCModel) {
    todo!() // mjCCamera::NameSpace
}

/// C: mjCCamera::ResolveReferences (user/user_objects.h:910)
/// Calls: mjCModel::FindObject
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_camera_resolve_references(self_ptr: *mut mjCCamera, m: *const mjCModel) {
    todo!() // mjCCamera::ResolveReferences
}

/// C: mjCLight::get_targetbody (user/user_objects.h:944)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_light_get_targetbody(self_ptr: *mut mjCLight) -> *const std__string {
    todo!() // mjCLight::get_targetbody
}

/// C: mjCLight::get_texture (user/user_objects.h:945)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_light_get_texture(self_ptr: *mut mjCLight) -> *const std__string {
    todo!() // mjCLight::get_texture
}

/// C: mjCLight::SetParent (user/user_objects.h:947)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_light_set_parent(self_ptr: *mut mjCLight, _body: *mut mjCBody) {
    // MACHINE-PROVEN: Clang AST direct field access
    unsafe { (*self_ptr).body = _body; }
}

/// C: mjCLight::GetParent (user/user_objects.h:948)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_light_get_parent(self_ptr: *mut mjCLight) -> *mut mjCBody {
    // MACHINE-PROVEN: Clang AST direct field access
    unsafe { (*self_ptr).body }
}

/// C: mjCLight::Compile (user/user_objects.h:951)
/// Calls: mjCLight::CopyFromSpec, mjCLight::ResolveReferences, mjuu_frameaccumChild, mjuu_normvec, mjuu_rotVecQuat
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_light_compile(self_ptr: *mut mjCLight) {
    todo!() // mjCLight::Compile
}

/// C: mjCLight::CopyFromSpec (user/user_objects.h:952)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_light_copy_from_spec(self_ptr: *mut mjCLight) {
    todo!() // mjCLight::CopyFromSpec
}

/// C: mjCLight::PointToLocal (user/user_objects.h:953)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_light_point_to_local(self_ptr: *mut mjCLight) {
    // SAFETY: caller guarantees self_ptr is a valid pointer to mjCLight.
    // Offsets: mjsElement at +8, info at +80 (in _pad_0/mjCBase).
    unsafe {
        let base = self_ptr as *mut u8;
        (*self_ptr).spec.element = base.add(8) as *mut mjsElement;
        (*self_ptr).spec.targetbody = std::ptr::addr_of_mut!((*self_ptr).spec_targetbody_) as *mut mjString;
        (*self_ptr).spec.texture = std::ptr::addr_of_mut!((*self_ptr).spec_texture_) as *mut mjString;
        (*self_ptr).spec.info = base.add(80) as *mut mjString;
        (*self_ptr).targetbody = std::ptr::null_mut();
    }
}

/// C: mjCLight::NameSpace (user/user_objects.h:954)
/// Calls: mjCBase::NameSpace
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_light_name_space(self_ptr: *mut mjCLight, m: *const mjCModel) {
    todo!() // mjCLight::NameSpace
}

/// C: mjCLight::ResolveReferences (user/user_objects.h:955)
/// Calls: mjCModel::FindObject
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_light_resolve_references(self_ptr: *mut mjCLight, m: *const mjCModel) {
    todo!() // mjCLight::ResolveReferences
}

/// C: mjCFlex::CopyFromSpec (user/user_objects.h:1032)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_copy_from_spec(self_ptr: *mut mjCFlex) {
    todo!() // mjCFlex::CopyFromSpec
}

/// C: mjCFlex::PointToLocal (user/user_objects.h:1033)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_point_to_local(self_ptr: *mut mjCFlex) {
    // SAFETY: caller guarantees self_ptr is a valid pointer to mjCFlex.
    // Offsets: mjsElement at +8, info at +80 (in _pad_0/mjCBase).
    unsafe {
        let base = self_ptr as *mut u8;
        (*self_ptr).spec.element = base.add(8) as *mut mjsElement;
        (*self_ptr).spec.material = std::ptr::addr_of_mut!((*self_ptr).spec_material_) as *mut mjString;
        (*self_ptr).spec.vertbody = std::ptr::addr_of_mut!((*self_ptr).spec_vertbody_) as *mut mjStringVec;
        (*self_ptr).spec.nodebody = std::ptr::addr_of_mut!((*self_ptr).spec_nodebody_) as *mut mjStringVec;
        (*self_ptr).spec.vert = std::ptr::addr_of_mut!((*self_ptr).spec_vert_) as *mut mjDoubleVec;
        (*self_ptr).spec.node = std::ptr::addr_of_mut!((*self_ptr).spec_node_) as *mut mjDoubleVec;
        (*self_ptr).spec.texcoord = std::ptr::addr_of_mut!((*self_ptr).spec_texcoord_) as *mut mjFloatVec;
        (*self_ptr).spec.elemtexcoord = std::ptr::addr_of_mut!((*self_ptr).spec_elemtexcoord_) as *mut mjIntVec;
        (*self_ptr).spec.elem = std::ptr::addr_of_mut!((*self_ptr).spec_elem_) as *mut mjIntVec;
        (*self_ptr).spec.info = base.add(80) as *mut mjString;
        (*self_ptr).material = std::ptr::null_mut();
        (*self_ptr).vertbody = std::ptr::null_mut();
        (*self_ptr).nodebody = std::ptr::null_mut();
        (*self_ptr).vert = std::ptr::null_mut();
        (*self_ptr).node = std::ptr::null_mut();
        (*self_ptr).texcoord = std::ptr::null_mut();
        (*self_ptr).elemtexcoord = std::ptr::null_mut();
        (*self_ptr).elem = std::ptr::null_mut();
    }
}

/// C: mjCFlex::ResolveReferences (user/user_objects.h:1034)
/// Calls: mjCModel::FindObject
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_resolve_references(self_ptr: *mut mjCFlex, m: *const mjCModel) {
    todo!() // mjCFlex::ResolveReferences
}

/// C: mjCFlex::NameSpace (user/user_objects.h:1035)
/// Calls: mjCBase::NameSpace
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_name_space(self_ptr: *mut mjCFlex, m: *const mjCModel) {
    todo!() // mjCFlex::NameSpace
}

/// C: mjCFlex::get_material (user/user_objects.h:1038)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_get_material(self_ptr: *mut mjCFlex) -> *const std__string {
    todo!() // mjCFlex::get_material
}

/// C: mjCFlex::get_vertbody (user/user_objects.h:1039)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_get_vertbody(self_ptr: *mut mjCFlex) -> *const () {
    todo!() // mjCFlex::get_vertbody
}

/// C: mjCFlex::get_vert (user/user_objects.h:1040)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_get_vert(self_ptr: *mut mjCFlex) -> *const () {
    todo!() // mjCFlex::get_vert
}

/// C: mjCFlex::get_elemaabb (user/user_objects.h:1041)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_get_elemaabb(self_ptr: *mut mjCFlex) -> *const () {
    todo!() // mjCFlex::get_elemaabb
}

/// C: mjCFlex::get_elem (user/user_objects.h:1042)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_get_elem(self_ptr: *mut mjCFlex) -> *const () {
    todo!() // mjCFlex::get_elem
}

/// C: mjCFlex::get_texcoord (user/user_objects.h:1043)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_get_texcoord(self_ptr: *mut mjCFlex) -> *const () {
    todo!() // mjCFlex::get_texcoord
}

/// C: mjCFlex::get_elemtexcoord (user/user_objects.h:1044)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_get_elemtexcoord(self_ptr: *mut mjCFlex) -> *const () {
    todo!() // mjCFlex::get_elemtexcoord
}

/// C: mjCFlex::get_nodebody (user/user_objects.h:1045)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_get_nodebody(self_ptr: *mut mjCFlex) -> *const () {
    todo!() // mjCFlex::get_nodebody
}

/// C: mjCFlex::HasTexcoord (user/user_objects.h:1047)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_has_texcoord(self_ptr: *mut mjCFlex) -> bool {
    todo!() // mjCFlex::HasTexcoord
}

/// C: mjCFlex::DelTexcoord (user/user_objects.h:1048)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_del_texcoord(self_ptr: *mut mjCFlex) {
    todo!() // mjCFlex::DelTexcoord
}

/// C: mjCFlex::Compile (user/user_objects.h:1055)
/// Calls: ComputeBending, ComputeInterpBending, ComputeLinearStiffness, ComputeLinearStiffness2D, ComputeStiffness, ComputeWarpMode, ComputeWarpStiffness, CreateFlapStencil, EigendecomposeStiffness, mjCBoundingVolumeHierarchy::Bvh, mjCFlex::ComputeCellEmpty, mjCFlex::ComputeUnrotatedNodePositions, mjCFlex::CopyFromSpec, mjCFlex::CreateBVH, mjCFlex::CreateShellPair, mjCFlex::LoadCachedStiffness, mjCFlex::ResolveReferences, mjCModel::Bodies, mjCModel::Equalities, mjCModel::FindObject, mjuu_addtovec, mjuu_copyvec, mjuu_crossvec, mjuu_dot3, mjuu_mulvecmat, mjuu_rotVecQuat
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_compile(self_ptr: *mut mjCFlex, vfs: *const mjVFS) {
    todo!() // mjCFlex::Compile
}

/// C: mjCFlex::CreateBVH (user/user_objects.h:1056)
/// Calls: mjCBoundingVolumeHierarchy::AddBoundingVolume, mjCBoundingVolumeHierarchy::AllocateBoundingVolumes, mjCBoundingVolumeHierarchy::CreateBVH, mjCBoundingVolumeHierarchy::RemoveInactiveVolumes, mjuu_copyvec
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_create_bvh(self_ptr: *mut mjCFlex) {
    todo!() // mjCFlex::CreateBVH
}

/// C: mjCFlex::CreateShellPair (user/user_objects.h:1057)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_create_shell_pair(self_ptr: *mut mjCFlex) {
    todo!() // mjCFlex::CreateShellPair
}

/// C: mjCFlex::ComputeCellEmpty (user/user_objects.h:1058)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_compute_cell_empty(self_ptr: *mut mjCFlex, vpos: *const f64, elems: *const i32, nv: i32, ne: i32, fdim: i32, bbox: *const f64) {
    todo!() // mjCFlex::ComputeCellEmpty
}

/// C: mjCFlex::ComputeUnrotatedNodePositions (user/user_objects.h:1067)
/// Calls: mjuu_copyvec, mjuu_dot3, mjuu_mulvecmat, mjuu_normvec
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_compute_unrotated_node_positions(self_ptr: *mut mjCFlex, nodexpos: *const (), R0_out: *mut f64) -> *const () {
    todo!() // mjCFlex::ComputeUnrotatedNodePositions
}

/// C: mjCFlex::ComputeStiffnessCacheKey (user/user_objects.h:1071)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_compute_stiffness_cache_key(self_ptr: *mut mjCFlex) -> std__string {
    todo!() // mjCFlex::ComputeStiffnessCacheKey
}

/// C: mjCFlex::LoadCachedStiffness (user/user_objects.h:1072)
/// Calls: mjCCache::PopulateData, mjCFlex::ComputeStiffnessCacheKey, mj_getCache
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_load_cached_stiffness(self_ptr: *mut mjCFlex) -> bool {
    todo!() // mjCFlex::LoadCachedStiffness
}

/// C: mjCFlex::CacheStiffness (user/user_objects.h:1073)
/// Calls: mjCCache::Insert, mjCFlex::ComputeStiffnessCacheKey, mj_getCache
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_flex_cache_stiffness(self_ptr: *mut mjCFlex) {
    todo!() // mjCFlex::CacheStiffness
}

/// C: mjCMesh::CopyFromSpec (user/user_objects.h:1151)
/// Calls: mjCMesh::ProcessVertices, mju_free, mjuu_stripext, mjuu_strippath
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_copy_from_spec(self_ptr: *mut mjCMesh) {
    todo!() // mjCMesh::CopyFromSpec
}

/// C: mjCMesh::PointToLocal (user/user_objects.h:1152)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_point_to_local(self_ptr: *mut mjCMesh) {
    // SAFETY: caller guarantees self_ptr is a valid pointer to mjCMesh.
    // Offsets: mjsElement at +8, info at +80 (in _pad_0/mjCBase).
    unsafe {
        let base = self_ptr as *mut u8;
        (*self_ptr).spec.element = base.add(8) as *mut mjsElement;
        (*self_ptr).spec.file = std::ptr::addr_of_mut!((*self_ptr).spec_file_) as *mut mjString;
        (*self_ptr).spec.content_type = std::ptr::addr_of_mut!((*self_ptr).spec_content_type_) as *mut mjString;
        (*self_ptr).spec.uservert = std::ptr::addr_of_mut!((*self_ptr).spec_vert_) as *mut mjFloatVec;
        (*self_ptr).spec.usernormal = std::ptr::addr_of_mut!((*self_ptr).spec_normal_) as *mut mjFloatVec;
        (*self_ptr).spec.userface = std::ptr::addr_of_mut!((*self_ptr).spec_face_) as *mut mjIntVec;
        (*self_ptr).spec.userfacenormal = std::ptr::addr_of_mut!((*self_ptr).spec_facenormal_) as *mut mjIntVec;
        (*self_ptr).spec.usertexcoord = std::ptr::addr_of_mut!((*self_ptr).spec_texcoord_) as *mut mjFloatVec;
        (*self_ptr).spec.userfacetexcoord = std::ptr::addr_of_mut!((*self_ptr).spec_facetexcoord_) as *mut mjIntVec;
        (*self_ptr).spec.material = std::ptr::addr_of_mut!((*self_ptr).spec_material_) as *mut mjString;
        (*self_ptr).spec.plugin.plugin_name = std::ptr::addr_of_mut!((*self_ptr).plugin_name) as *mut mjString;
        (*self_ptr).spec.plugin.name = std::ptr::addr_of_mut!((*self_ptr).plugin_instance_name) as *mut mjString;
        (*self_ptr).spec.info = base.add(80) as *mut mjString;
        (*self_ptr).file = std::ptr::null_mut();
        (*self_ptr).content_type = std::ptr::null_mut();
        (*self_ptr).uservert = std::ptr::null_mut();
        (*self_ptr).usernormal = std::ptr::null_mut();
        (*self_ptr).userface = std::ptr::null_mut();
        (*self_ptr).userfacenormal = std::ptr::null_mut();
        (*self_ptr).usertexcoord = std::ptr::null_mut();
        (*self_ptr).userfacetexcoord = std::ptr::null_mut();
    }
}

/// C: mjCMesh::NameSpace (user/user_objects.h:1153)
/// Calls: mjCBase::NameSpace, mjuu_stripext, mjuu_strippath
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_name_space(self_ptr: *mut mjCMesh, m: *const mjCModel) {
    todo!() // mjCMesh::NameSpace
}

/// C: mjCMesh::MakeHemisphere (user/user_objects.h:1156)
/// Calls: mjs_setFloat, mjs_setInt
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_make_hemisphere(self_ptr: *mut mjCMesh, res: i32, make_faces: bool, make_cap: bool) {
    todo!() // mjCMesh::MakeHemisphere
}

/// C: mjCMesh::MakeSphere (user/user_objects.h:1157)
/// Calls: mjs_setFloat, mjs_setInt
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_make_sphere(self_ptr: *mut mjCMesh, subdiv: i32, make_faces: bool) {
    todo!() // mjCMesh::MakeSphere
}

/// C: mjCMesh::MakeTorus (user/user_objects.h:1158)
/// Calls: mjs_setFloat, mjs_setInt
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_make_torus(self_ptr: *mut mjCMesh, res: i32, radius: f64) {
    todo!() // mjCMesh::MakeTorus
}

/// C: mjCMesh::MakeSupertorus (user/user_objects.h:1159)
/// Calls: aux_c, aux_s, mjs_setFloat, mjs_setInt
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_make_supertorus(self_ptr: *mut mjCMesh, res: i32, radius: f64, s: f64, t: f64) {
    todo!() // mjCMesh::MakeSupertorus
}

/// C: mjCMesh::MakeSupersphere (user/user_objects.h:1160)
/// Calls: aux_c, aux_s, mjs_setFloat, mjs_setInt
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_make_supersphere(self_ptr: *mut mjCMesh, res: i32, e: f64, n: f64) {
    todo!() // mjCMesh::MakeSupersphere
}

/// C: mjCMesh::MakeWedge (user/user_objects.h:1161)
/// Calls: BinEdges, SphericalToCartesian, TangentFrame, mjs_setFloat
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_make_wedge(self_ptr: *mut mjCMesh, resolution: *mut i32, fov: *mut f64, gamma: f64) {
    todo!() // mjCMesh::MakeWedge
}

/// C: mjCMesh::MakeRect (user/user_objects.h:1162)
/// Calls: LinSpace, mjs_setFloat, mjs_setInt
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_make_rect(self_ptr: *mut mjCMesh, resolution: *mut i32) {
    todo!() // mjCMesh::MakeRect
}

/// C: mjCMesh::MakeCone (user/user_objects.h:1163)
/// Calls: mjs_setFloat
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_make_cone(self_ptr: *mut mjCMesh, nedge: i32, radius: f64) {
    todo!() // mjCMesh::MakeCone
}

/// C: mjCMesh::Plugin (user/user_objects.h:1166)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_plugin(self_ptr: *mut mjCMesh) -> *const mjsPlugin {
    // MACHINE-PROVEN: Clang AST direct field access
    unsafe { core::ptr::addr_of!((*self_ptr).plugin).cast::<mjsPlugin>() }
}

/// C: mjCMesh::ContentType (user/user_objects.h:1167)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_content_type(self_ptr: *mut mjCMesh) -> *const std__string {
    todo!() // mjCMesh::ContentType
}

/// C: mjCMesh::File (user/user_objects.h:1168)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_file(self_ptr: *mut mjCMesh) -> *const std__string {
    todo!() // mjCMesh::File
}

/// C: mjCMesh::Refpos (user/user_objects.h:1169)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_refpos(self_ptr: *mut mjCMesh) -> *const f64 {
    // MACHINE-PROVEN: Clang AST direct field access
    unsafe { core::ptr::addr_of!((*self_ptr).refpos).cast::<f64>() }
}

/// C: mjCMesh::Refquat (user/user_objects.h:1170)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_refquat(self_ptr: *mut mjCMesh) -> *const f64 {
    // MACHINE-PROVEN: Clang AST direct field access
    unsafe { core::ptr::addr_of!((*self_ptr).refquat).cast::<f64>() }
}

/// C: mjCMesh::Scale (user/user_objects.h:1171)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_scale(self_ptr: *mut mjCMesh) -> *const f64 {
    // MACHINE-PROVEN: Clang AST direct field access
    unsafe { core::ptr::addr_of!((*self_ptr).scale).cast::<f64>() }
}

/// C: mjCMesh::SmoothNormal (user/user_objects.h:1172)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_smooth_normal(self_ptr: *mut mjCMesh) -> bool {
    todo!() // mjCMesh::SmoothNormal
}

/// C: mjCMesh::Vert (user/user_objects.h:1173)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_vert(self_ptr: *mut mjCMesh) -> *const () {
    todo!() // mjCMesh::Vert
}

/// C: mjCMesh::UserVert (user/user_objects.h:1175)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_user_vert(self_ptr: *mut mjCMesh) -> *const () {
    todo!() // mjCMesh::UserVert
}

/// C: mjCMesh::UserNormal (user/user_objects.h:1176)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_user_normal(self_ptr: *mut mjCMesh) -> *const () {
    todo!() // mjCMesh::UserNormal
}

/// C: mjCMesh::Texcoord (user/user_objects.h:1177)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_texcoord(self_ptr: *mut mjCMesh) -> *const () {
    todo!() // mjCMesh::Texcoord
}

/// C: mjCMesh::FaceTexcoord (user/user_objects.h:1178)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_face_texcoord(self_ptr: *mut mjCMesh) -> *const () {
    todo!() // mjCMesh::FaceTexcoord
}

/// C: mjCMesh::UserTexcoord (user/user_objects.h:1179)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_user_texcoord(self_ptr: *mut mjCMesh) -> *const () {
    todo!() // mjCMesh::UserTexcoord
}

/// C: mjCMesh::Face (user/user_objects.h:1180)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_face(self_ptr: *mut mjCMesh) -> *const () {
    todo!() // mjCMesh::Face
}

/// C: mjCMesh::UserFace (user/user_objects.h:1181)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_user_face(self_ptr: *mut mjCMesh) -> *const () {
    todo!() // mjCMesh::UserFace
}

/// C: mjCMesh::Inertia (user/user_objects.h:1182)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_inertia(self_ptr: *mut mjCMesh) -> u32 {
    // SAFETY: self_ptr is valid; inertia field stores mjtMeshInertia (u32) as [u8; 4]
    unsafe {
        let bytes = &(*self_ptr).inertia;
        u32::from_ne_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
    }
}

/// C: mjCMesh::Material (user/user_objects.h:1183)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_material(self_ptr: *mut mjCMesh) -> *const std__string {
    todo!() // mjCMesh::Material
}

/// C: mjCMesh::SetNeedHull (user/user_objects.h:1186)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_set_need_hull(self_ptr: *mut mjCMesh, needhull: bool) {
    // MACHINE-PROVEN: Clang AST direct field access
    unsafe { (*self_ptr).needhull_ = needhull; }
}

/// C: mjCMesh::aamm (user/user_objects.h:1189)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_aamm(self_ptr: *mut mjCMesh) -> *const f64 {
    // MACHINE-PROVEN: Clang AST direct field access
    unsafe { core::ptr::addr_of!((*self_ptr).aamm_).cast::<f64>() }
}

/// C: mjCMesh::nvert (user/user_objects.h:1192)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_nvert(self_ptr: *mut mjCMesh) -> i32 {
    todo!() // mjCMesh::nvert
}

/// C: mjCMesh::nnormal (user/user_objects.h:1193)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_nnormal(self_ptr: *mut mjCMesh) -> i32 {
    todo!() // mjCMesh::nnormal
}

/// C: mjCMesh::ntexcoord (user/user_objects.h:1194)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_ntexcoord(self_ptr: *mut mjCMesh) -> i32 {
    todo!() // mjCMesh::ntexcoord
}

/// C: mjCMesh::nface (user/user_objects.h:1195)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_nface(self_ptr: *mut mjCMesh) -> i32 {
    todo!() // mjCMesh::nface
}

/// C: mjCMesh::npolygon (user/user_objects.h:1196)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_npolygon(self_ptr: *mut mjCMesh) -> i32 {
    todo!() // mjCMesh::npolygon
}

/// C: mjCMesh::npolygonvert (user/user_objects.h:1197)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_npolygonvert(self_ptr: *mut mjCMesh) -> i32 {
    todo!() // mjCMesh::npolygonvert
}

/// C: mjCMesh::npolygonmap (user/user_objects.h:1204)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_npolygonmap(self_ptr: *mut mjCMesh) -> i32 {
    todo!() // mjCMesh::npolygonmap
}

/// C: mjCMesh::szgraph (user/user_objects.h:1213)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_szgraph(self_ptr: *mut mjCMesh) -> i32 {
    // MACHINE-PROVEN: Clang AST direct field access
    unsafe { (*self_ptr).szgraph_ }
}

/// C: mjCMesh::tree (user/user_objects.h:1216)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_tree(self_ptr: *mut mjCMesh) -> *const mjCBoundingVolumeHierarchy {
    todo!() // mjCMesh::tree
}

/// C: mjCMesh::octree (user/user_objects.h:1219)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_octree(self_ptr: *mut mjCMesh) -> *const mjCOctree {
    todo!() // mjCMesh::octree
}

/// C: mjCMesh::mutable_octree (user/user_objects.h:1220)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_mutable_octree(self_ptr: *mut mjCMesh) -> *mut mjCOctree {
    // MACHINE-PROVEN: Clang AST direct field access
    unsafe { core::ptr::addr_of_mut!((*self_ptr).octree_).cast::<mjCOctree>() }
}

/// C: mjCMesh::Compile (user/user_objects.h:1222)
/// Calls: mjCMesh::TryCompile, mju_closeResource
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_compile(self_ptr: *mut mjCMesh, vfs: *const mjVFS) {
    todo!() // mjCMesh::Compile
}

/// C: mjCMesh::GetPosPtr (user/user_objects.h:1223)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_get_pos_ptr(self_ptr: *mut mjCMesh) -> *mut f64 {
    // MACHINE-PROVEN: Clang AST direct field access
    unsafe { core::ptr::addr_of_mut!((*self_ptr).pos_).cast::<f64>() }
}

/// C: mjCMesh::GetQuatPtr (user/user_objects.h:1224)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_get_quat_ptr(self_ptr: *mut mjCMesh) -> *mut f64 {
    // MACHINE-PROVEN: Clang AST direct field access
    unsafe { core::ptr::addr_of_mut!((*self_ptr).quat_).cast::<f64>() }
}

/// C: mjCMesh::GetInertiaBoxPtr (user/user_objects.h:1225)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_get_inertia_box_ptr(self_ptr: *mut mjCMesh) -> *mut f64 {
    // MACHINE-PROVEN: Clang AST direct field access
    unsafe { core::ptr::addr_of_mut!((*self_ptr).boxsz_).cast::<f64>() }
}

/// C: mjCMesh::GetVolumeRef (user/user_objects.h:1226)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_get_volume_ref(self_ptr: *mut mjCMesh) -> f64 {
    // SAFETY: self_ptr is valid pointer to mjCMesh
    unsafe {
        // inertia field is [u8; 4] representing mjtMeshInertia enum
        // mjMESH_INERTIA_SHELL = 3
        let inertia_val = i32::from_ne_bytes((*self_ptr).inertia);
        if inertia_val == 3 {
            (*self_ptr).surface_
        } else {
            (*self_ptr).volume_
        }
    }
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
    todo!() // mjCMesh::FitGeom
}

/// C: mjCMesh::HasTexcoord (user/user_objects.h:1228)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_has_texcoord(self_ptr: *mut mjCMesh) -> bool {
    todo!() // mjCMesh::HasTexcoord
}

/// C: mjCMesh::DelTexcoord (user/user_objects.h:1229)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_del_texcoord(self_ptr: *mut mjCMesh) {
    todo!() // mjCMesh::DelTexcoord
}

/// C: mjCMesh::IsVisual (user/user_objects.h:1230)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_is_visual(self_ptr: *mut mjCMesh) -> bool {
    // MACHINE-PROVEN: Clang AST direct field access
    unsafe { (*self_ptr).visual_ }
}

/// C: mjCMesh::SetNotVisual (user/user_objects.h:1231)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_set_not_visual(self_ptr: *mut mjCMesh) {
    // SAFETY: self_ptr is a valid pointer to mjCMesh provided by the caller
    unsafe { (*self_ptr).visual_ = false; }
}

/// C: mjCMesh::CopyVert (user/user_objects.h:1233)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_copy_vert(self_ptr: *mut mjCMesh, arr: *mut f32) {
    todo!() // mjCMesh::CopyVert
}

/// C: mjCMesh::CopyNormal (user/user_objects.h:1234)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_copy_normal(self_ptr: *mut mjCMesh, arr: *mut f32) {
    todo!() // mjCMesh::CopyNormal
}

/// C: mjCMesh::CopyFace (user/user_objects.h:1235)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_copy_face(self_ptr: *mut mjCMesh, arr: *mut i32) {
    todo!() // mjCMesh::CopyFace
}

/// C: mjCMesh::CopyFaceNormal (user/user_objects.h:1236)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_copy_face_normal(self_ptr: *mut mjCMesh, arr: *mut i32) {
    todo!() // mjCMesh::CopyFaceNormal
}

/// C: mjCMesh::CopyFaceTexcoord (user/user_objects.h:1237)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_copy_face_texcoord(self_ptr: *mut mjCMesh, arr: *mut i32) {
    todo!() // mjCMesh::CopyFaceTexcoord
}

/// C: mjCMesh::CopyTexcoord (user/user_objects.h:1238)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_copy_texcoord(self_ptr: *mut mjCMesh, arr: *mut f32) {
    todo!() // mjCMesh::CopyTexcoord
}

/// C: mjCMesh::CopyGraph (user/user_objects.h:1239)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_copy_graph(self_ptr: *mut mjCMesh, arr: *mut i32) {
    todo!() // mjCMesh::CopyGraph
}

/// C: mjCMesh::CopyPolygons (user/user_objects.h:1242)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_copy_polygons(self_ptr: *mut mjCMesh, verts: *mut i32, adr: *mut i32, num: *mut i32, poly_adr: i32) {
    todo!() // mjCMesh::CopyPolygons
}

/// C: mjCMesh::CopyPolygonMap (user/user_objects.h:1245)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_copy_polygon_map(self_ptr: *mut mjCMesh, faces: *mut i32, adr: *mut i32, num: *mut i32, poly_adr: i32) {
    todo!() // mjCMesh::CopyPolygonMap
}

/// C: mjCMesh::CopyPolygonNormals (user/user_objects.h:1248)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_copy_polygon_normals(self_ptr: *mut mjCMesh, arr: *mut f64) {
    todo!() // mjCMesh::CopyPolygonNormals
}

/// C: mjCMesh::SetBoundingVolume (user/user_objects.h:1251)
/// Calls: mjCBoundingVolumeHierarchy::AddBoundingVolume
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_set_bounding_volume(self_ptr: *mut mjCMesh, faceid: i32, dvert: *const f64) {
    todo!() // mjCMesh::SetBoundingVolume
}

/// C: mjCMesh::LoadFromResource (user/user_objects.h:1254)
/// Calls: mjCBase::GetAssetContentType, mjCMesh::IsMSH, mjCMesh::LoadFromDecoder, mjCMesh::LoadMSH
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_load_from_resource(self_ptr: *mut mjCMesh, resource: *mut mjResource, remove_repeated: bool) {
    todo!() // mjCMesh::LoadFromResource
}

/// C: mjCMesh::IsMSH (user/user_objects.h:1258)
/// Calls: mjCBase::GetAssetContentType
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_is_msh(filename: std__string_view, ct: std__string_view) -> bool {
    todo!() // mjCMesh::IsMSH
}

/// C: mjCMesh::TryCompile (user/user_objects.h:1265)
/// Calls: FilePath::Str, mjCBase::LoadResource, mjCMesh::CacheMesh, mjCMesh::CheckInitialMesh, mjCMesh::CopyFromSpec, mjCMesh::LoadCachedMesh, mjCMesh::LoadFromResource, mjCMesh::LoadSDF, mjCMesh::Process, mjCMesh::ProcessVertices, mjCMesh::nface, mjCMesh::nvert, mjCModel::FindSpec, mjCOctree::Clear, mjCOctree::ComputeSdfCoeffs, mjCOctree::CreateOctree, mjCOctree::NumNodes, mjCOctree::SetFace, mjCOctree::SetMaxDepth, mj_getCache, mjs_getString, mju_closeResource, mjuu_strippath
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_try_compile(self_ptr: *mut mjCMesh, vfs: *const mjVFS) {
    todo!() // mjCMesh::TryCompile
}

/// C: mjCMesh::LoadCachedMesh (user/user_objects.h:1268)
/// Calls: mjCCache::PopulateData, mjCMesh::nvert, mju_malloc
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_load_cached_mesh(self_ptr: *mut mjCMesh, cache: *mut mjCCache, resource: *const mjResource) -> bool {
    todo!() // mjCMesh::LoadCachedMesh
}

/// C: mjCMesh::CacheMesh (user/user_objects.h:1271)
/// Calls: mjCBoundingVolumeHierarchy::Size, mjCCache::Insert, mjCMesh::npolygonmap, mjCMesh::npolygonvert, mjCOctree::Size, mju_malloc
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_cache_mesh(self_ptr: *mut mjCMesh, cache: *mut mjCCache, resource: *const mjResource) {
    todo!() // mjCMesh::CacheMesh
}

/// C: mjCMesh::ProcessVertices (user/user_objects.h:1274)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_process_vertices(self_ptr: *mut mjCMesh, vert: *const (), remove_repeated: bool) {
    todo!() // mjCMesh::ProcessVertices
}

/// C: mjCMesh::LoadFromDecoder (user/user_objects.h:1277)
/// Calls: mjCMesh::ProcessVertices, mj_deleteSpec, mjp_findDecoder, mjs_asMesh, mjs_firstElement
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_load_from_decoder(self_ptr: *mut mjCMesh, resource: *mut mjResource, remove_repeated: bool) {
    todo!() // mjCMesh::LoadFromDecoder
}

/// C: mjCMesh::LoadMSH (user/user_objects.h:1279)
/// Calls: ReadFromBuffer, mjCMesh::ProcessVertices, mju_readResource
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_load_msh(self_ptr: *mut mjCMesh, resource: *mut mjResource, remove_repeated: bool) {
    todo!() // mjCMesh::LoadMSH
}

/// C: mjCMesh::LoadSDF (user/user_objects.h:1281)
/// Calls: mjCMesh::ProcessVertices, mjCModel::ResolvePlugin, mjp_getPluginAtSlot
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_load_sdf(self_ptr: *mut mjCMesh) {
    todo!() // mjCMesh::LoadSDF
}

/// C: mjCMesh::MakeGraph (user/user_objects.h:1282)
/// Calls: mjCMesh::nvert, mju_error, mju_free, mju_malloc, mju_warning, mjuu_crossvec, mjuu_dist3, mjuu_dot3
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_make_graph(self_ptr: *mut mjCMesh, dvert: *const f64) {
    todo!() // mjCMesh::MakeGraph
}

/// C: mjCMesh::MakeNormal (user/user_objects.h:1284)
/// Calls: mjCMesh::nface, mjCMesh::nnormal, mjCMesh::nvert, mju_free, mju_malloc, mjuu_crossvec, mjuu_dot3, mjuu_normvec
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_make_normal(self_ptr: *mut mjCMesh, dvert: *const f64) {
    todo!() // mjCMesh::MakeNormal
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
    todo!() // mjCMesh::MakeCenter
}

/// C: mjCMesh::Process (user/user_objects.h:1286)
/// Calls: mjCBoundingVolumeHierarchy::AllocateBoundingVolumes, mjCBoundingVolumeHierarchy::Bvh, mjCBoundingVolumeHierarchy::CreateBVH, mjCMesh::ApplyTransformations, mjCMesh::ComputeFaceCentroid, mjCMesh::ComputeInertia, mjCMesh::ComputeSurfaceArea, mjCMesh::ComputeVolume, mjCMesh::CopyGraph, mjCMesh::GetVolumeRef, mjCMesh::MakeCenter, mjCMesh::MakeGraph, mjCMesh::MakeNormal, mjCMesh::MakePolygonNormals, mjCMesh::MakePolygons, mjCMesh::Rotate, mjCMesh::SetBoundingVolume, mjCMesh::nface, mjCMesh::nvert, mjCOctree::ComputeSdfCoeffs, mjCOctree::CreateOctree, mjCOctree::SetFace, mjCOctree::SetMaxDepth, mjuu_copyvec, mjuu_eig3, mjuu_setvec, triangle
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_process(self_ptr: *mut mjCMesh) {
    todo!() // mjCMesh::Process
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
    todo!() // mjCMesh::ApplyTransformations
}

/// C: mjCMesh::ComputeFaceCentroid (user/user_objects.h:1288)
/// Calls: mjCMesh::nface, triangle
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_compute_face_centroid(self_ptr: *mut mjCMesh, arg0: *mut f64, dvert: *const f64) -> f64 {
    todo!() // mjCMesh::ComputeFaceCentroid
}

/// C: mjCMesh::CheckInitialMesh (user/user_objects.h:1289)
/// Calls: mjCMesh::nvert
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_check_initial_mesh(self_ptr: *mut mjCMesh) {
    todo!() // mjCMesh::CheckInitialMesh
}

/// C: mjCMesh::CopyPlugin (user/user_objects.h:1290)
/// Calls: mjCModel::CopyExplicitPlugin
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_copy_plugin(self_ptr: *mut mjCMesh) {
    todo!() // mjCMesh::CopyPlugin
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
    todo!() // mjCMesh::Rotate
}

/// C: mjCMesh::MakePolygons (user/user_objects.h:1293)
/// Calls: MeshPolygon::InsertFace, MeshPolygon::Normal, MeshPolygon::Paths, MeshPolygonKey, mjCMesh::GraphFaces, mjCMesh::nvert
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_make_polygons(self_ptr: *mut mjCMesh, dvert: *const f64) {
    todo!() // mjCMesh::MakePolygons
}

/// C: mjCMesh::MakePolygonNormals (user/user_objects.h:1294)
/// Calls: mjuu_makenormal
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_make_polygon_normals(self_ptr: *mut mjCMesh, dvert: *const f64) {
    todo!() // mjCMesh::MakePolygonNormals
}

/// C: mjCMesh::ComputeInertia (user/user_objects.h:1297)
/// Calls: mjCMesh::GraphFaces, mjCMesh::nface, mjCMesh::nvert, mjuu_dot3, triangle
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_compute_inertia(self_ptr: *mut mjCMesh, inert: *mut f64, CoM: *const f64, dvert: *const f64) -> f64 {
    todo!() // mjCMesh::ComputeInertia
}

/// C: mjCMesh::GraphFaces (user/user_objects.h:1299)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_graph_faces(self_ptr: *mut mjCMesh) -> *mut i32 {
    todo!() // mjCMesh::GraphFaces
}

/// C: mjCMesh::ComputeVolume (user/user_objects.h:1313)
/// Calls: mjCMesh::GraphFaces, mjCMesh::nface, mjuu_dot3, triangle
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_compute_volume(self_ptr: *mut mjCMesh, CoM: *mut f64, facecen: *const f64, dvert: *const f64) -> f64 {
    todo!() // mjCMesh::ComputeVolume
}

/// C: mjCMesh::ComputeSurfaceArea (user/user_objects.h:1314)
/// Calls: mjCMesh::nface, triangle
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_mesh_compute_surface_area(self_ptr: *mut mjCMesh, CoM: *mut f64, facecen: *const f64, dvert: *const f64) -> f64 {
    todo!() // mjCMesh::ComputeSurfaceArea
}

/// C: mjCSkin::File (user/user_objects.h:1364)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_skin_file(self_ptr: *mut mjCSkin) -> *const std__string {
    todo!() // mjCSkin::File
}

/// C: mjCSkin::get_material (user/user_objects.h:1365)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_skin_get_material(self_ptr: *mut mjCSkin) -> *const std__string {
    todo!() // mjCSkin::get_material
}

/// C: mjCSkin::get_vert (user/user_objects.h:1366)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_skin_get_vert(self_ptr: *mut mjCSkin) -> *const () {
    todo!() // mjCSkin::get_vert
}

/// C: mjCSkin::get_texcoord (user/user_objects.h:1367)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_skin_get_texcoord(self_ptr: *mut mjCSkin) -> *const () {
    todo!() // mjCSkin::get_texcoord
}

/// C: mjCSkin::get_face (user/user_objects.h:1368)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_skin_get_face(self_ptr: *mut mjCSkin) -> *const () {
    todo!() // mjCSkin::get_face
}

/// C: mjCSkin::get_bodyname (user/user_objects.h:1369)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_skin_get_bodyname(self_ptr: *mut mjCSkin) -> *const () {
    todo!() // mjCSkin::get_bodyname
}

/// C: mjCSkin::get_bindpos (user/user_objects.h:1370)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_skin_get_bindpos(self_ptr: *mut mjCSkin) -> *const () {
    todo!() // mjCSkin::get_bindpos
}

/// C: mjCSkin::get_bindquat (user/user_objects.h:1371)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_skin_get_bindquat(self_ptr: *mut mjCSkin) -> *const () {
    todo!() // mjCSkin::get_bindquat
}

/// C: mjCSkin::get_vertid (user/user_objects.h:1372)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_skin_get_vertid(self_ptr: *mut mjCSkin) -> *const () {
    todo!() // mjCSkin::get_vertid
}

/// C: mjCSkin::get_vertweight (user/user_objects.h:1373)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_skin_get_vertweight(self_ptr: *mut mjCSkin) -> *const () {
    todo!() // mjCSkin::get_vertweight
}

/// C: mjCSkin::del_material (user/user_objects.h:1374)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_skin_del_material(self_ptr: *mut mjCSkin) {
    todo!() // mjCSkin::del_material
}

/// C: mjCSkin::CopyFromSpec (user/user_objects.h:1376)
/// Calls: mjuu_stripext, mjuu_strippath
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_skin_copy_from_spec(self_ptr: *mut mjCSkin) {
    todo!() // mjCSkin::CopyFromSpec
}

/// C: mjCSkin::PointToLocal (user/user_objects.h:1377)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_skin_point_to_local(self_ptr: *mut mjCSkin) {
    // SAFETY: caller guarantees self_ptr is a valid pointer to mjCSkin.
    // Offsets: mjsElement at +8, info at +80 (in _pad_0/mjCBase).
    unsafe {
        let base = self_ptr as *mut u8;
        (*self_ptr).spec.element = base.add(8) as *mut mjsElement;
        (*self_ptr).spec.file = std::ptr::addr_of_mut!((*self_ptr).spec_file_) as *mut mjString;
        (*self_ptr).spec.material = std::ptr::addr_of_mut!((*self_ptr).spec_material_) as *mut mjString;
        (*self_ptr).spec.vert = std::ptr::addr_of_mut!((*self_ptr).spec_vert_) as *mut mjFloatVec;
        (*self_ptr).spec.texcoord = std::ptr::addr_of_mut!((*self_ptr).spec_texcoord_) as *mut mjFloatVec;
        (*self_ptr).spec.face = std::ptr::addr_of_mut!((*self_ptr).spec_face_) as *mut mjIntVec;
        (*self_ptr).spec.bodyname = std::ptr::addr_of_mut!((*self_ptr).spec_bodyname_) as *mut mjStringVec;
        (*self_ptr).spec.bindpos = std::ptr::addr_of_mut!((*self_ptr).spec_bindpos_) as *mut mjFloatVec;
        (*self_ptr).spec.bindquat = std::ptr::addr_of_mut!((*self_ptr).spec_bindquat_) as *mut mjFloatVec;
        (*self_ptr).spec.vertid = std::ptr::addr_of_mut!((*self_ptr).spec_vertid_) as *mut mjIntVecVec;
        (*self_ptr).spec.vertweight = std::ptr::addr_of_mut!((*self_ptr).spec_vertweight_) as *mut mjFloatVecVec;
        (*self_ptr).spec.info = base.add(80) as *mut mjString;
        (*self_ptr).file = std::ptr::null_mut();
        (*self_ptr).material = std::ptr::null_mut();
        (*self_ptr).vert = std::ptr::null_mut();
        (*self_ptr).texcoord = std::ptr::null_mut();
        (*self_ptr).face = std::ptr::null_mut();
        (*self_ptr).bodyname = std::ptr::null_mut();
        (*self_ptr).bindpos = std::ptr::null_mut();
        (*self_ptr).bindquat = std::ptr::null_mut();
        (*self_ptr).vertid = std::ptr::null_mut();
        (*self_ptr).vertweight = std::ptr::null_mut();
    }
}

/// C: mjCSkin::ResolveReferences (user/user_objects.h:1380)
/// Calls: mjCModel::FindObject
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_skin_resolve_references(self_ptr: *mut mjCSkin, m: *const mjCModel) {
    todo!() // mjCSkin::ResolveReferences
}

/// C: mjCSkin::NameSpace (user/user_objects.h:1381)
/// Calls: mjuu_stripext, mjuu_strippath
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_skin_name_space(self_ptr: *mut mjCSkin, m: *const mjCModel) {
    todo!() // mjCSkin::NameSpace
}

/// C: mjCSkin::Compile (user/user_objects.h:1382)
/// Calls: FilePath::Str, mjCBase::LoadResource, mjCModel::FindObject, mjCModel::FindSpec, mjCSkin::CopyFromSpec, mjCSkin::LoadSKN, mjCSkin::ResolveReferences, mjs_getString, mju_closeResource, mjuu_getext, mjuu_normvec, mjuu_strippath
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_skin_compile(self_ptr: *mut mjCSkin, vfs: *const mjVFS) {
    todo!() // mjCSkin::Compile
}

/// C: mjCSkin::LoadSKN (user/user_objects.h:1383)
/// Calls: mju_readResource
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_skin_load_skn(self_ptr: *mut mjCSkin, resource: *mut mjResource) {
    todo!() // mjCSkin::LoadSKN
}

/// C: mjCHField::CopyFromSpec (user/user_objects.h:1417)
/// Calls: mjuu_stripext, mjuu_strippath
#[allow(unused_variables, non_snake_case)]
pub fn mj_ch_field_copy_from_spec(self_ptr: *mut mjCHField) {
    todo!() // mjCHField::CopyFromSpec
}

/// C: mjCHField::PointToLocal (user/user_objects.h:1418)
#[allow(unused_variables, non_snake_case)]
pub fn mj_ch_field_point_to_local(self_ptr: *mut mjCHField) {
    // SAFETY: caller guarantees self_ptr is a valid pointer to mjCHField.
    // Offsets: mjsElement at +8, info at +80 (in _pad_0/mjCBase).
    unsafe {
        let base = self_ptr as *mut u8;
        (*self_ptr).spec.element = base.add(8) as *mut mjsElement;
        (*self_ptr).spec.file = std::ptr::addr_of_mut!((*self_ptr).spec_file_) as *mut mjString;
        (*self_ptr).spec.content_type = std::ptr::addr_of_mut!((*self_ptr).spec_content_type_) as *mut mjString;
        (*self_ptr).spec.userdata = std::ptr::addr_of_mut!((*self_ptr).spec_userdata_) as *mut mjFloatVec;
        (*self_ptr).spec.info = base.add(80) as *mut mjString;
        (*self_ptr).file = std::ptr::null_mut();
        (*self_ptr).content_type = std::ptr::null_mut();
        (*self_ptr).userdata = std::ptr::null_mut();
    }
}

/// C: mjCHField::NameSpace (user/user_objects.h:1419)
/// Calls: mjCBase::NameSpace, mjuu_stripext, mjuu_strippath
#[allow(unused_variables, non_snake_case)]
pub fn mj_ch_field_name_space(self_ptr: *mut mjCHField, m: *const mjCModel) {
    todo!() // mjCHField::NameSpace
}

/// C: mjCHField::File (user/user_objects.h:1421)
#[allow(unused_variables, non_snake_case)]
pub fn mj_ch_field_file(self_ptr: *mut mjCHField) -> std__string {
    todo!() // mjCHField::File
}

/// C: mjCHField::get_userdata (user/user_objects.h:1424)
#[allow(unused_variables, non_snake_case)]
pub fn mj_ch_field_get_userdata(self_ptr: *mut mjCHField) -> *const () {
    todo!() // mjCHField::get_userdata
}

/// C: mjCHField::Compile (user/user_objects.h:1427)
/// Calls: FilePath::Str, mjCBase::GetAssetContentType, mjCBase::LoadResource, mjCCache::Insert, mjCCache::PopulateData, mjCHField::CopyFromSpec, mjCHField::GetCacheId, mjCHField::LoadCustom, mjCHField::LoadPNG, mjCModel::FindSpec, mj_getCache, mjs_getString, mju_closeResource, mjuu_strippath
#[allow(unused_variables, non_snake_case)]
pub fn mj_ch_field_compile(self_ptr: *mut mjCHField, vfs: *const mjVFS) {
    todo!() // mjCHField::Compile
}

/// C: mjCHField::GetCacheId (user/user_objects.h:1429)
#[allow(unused_variables, non_snake_case)]
pub fn mj_ch_field_get_cache_id(self_ptr: *mut mjCHField, resource: *const mjResource, asset_type: *const std__string) -> std__string {
    todo!() // mjCHField::GetCacheId
}

/// C: mjCHField::LoadCustom (user/user_objects.h:1430)
/// Calls: mju_readResource
#[allow(unused_variables, non_snake_case)]
pub fn mj_ch_field_load_custom(self_ptr: *mut mjCHField, resource: *mut mjResource) {
    todo!() // mjCHField::LoadCustom
}

/// C: mjCHField::LoadPNG (user/user_objects.h:1431)
/// Calls: PNGImage::Height, PNGImage::Load, PNGImage::Width
#[allow(unused_variables, non_snake_case)]
pub fn mj_ch_field_load_png(self_ptr: *mut mjCHField, resource: *mut mjResource) {
    todo!() // mjCHField::LoadPNG
}

/// C: mjCTexture::CopyFromSpec (user/user_objects.h:1465)
/// Calls: mjuu_stripext, mjuu_strippath
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_texture_copy_from_spec(self_ptr: *mut mjCTexture) {
    todo!() // mjCTexture::CopyFromSpec
}

/// C: mjCTexture::PointToLocal (user/user_objects.h:1466)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_texture_point_to_local(self_ptr: *mut mjCTexture) {
    // SAFETY: caller guarantees self_ptr is a valid pointer to mjCTexture.
    // Offsets: mjsElement at +8, info at +80 (in _pad_0/mjCBase).
    unsafe {
        let base = self_ptr as *mut u8;
        (*self_ptr).spec.element = base.add(8) as *mut mjsElement;
        (*self_ptr).spec.file = std::ptr::addr_of_mut!((*self_ptr).spec_file_) as *mut mjString;
        (*self_ptr).spec.data = std::ptr::addr_of_mut!((*self_ptr).data_) as *mut mjByteVec;
        (*self_ptr).spec.content_type = std::ptr::addr_of_mut!((*self_ptr).spec_content_type_) as *mut mjString;
        (*self_ptr).spec.cubefiles = std::ptr::addr_of_mut!((*self_ptr).spec_cubefiles_) as *mut mjStringVec;
        (*self_ptr).spec.info = base.add(80) as *mut mjString;
        (*self_ptr).file = std::ptr::null_mut();
        (*self_ptr).content_type = std::ptr::null_mut();
        (*self_ptr).cubefiles = std::ptr::null_mut();
    }
}

/// C: mjCTexture::NameSpace (user/user_objects.h:1467)
/// Calls: mjCBase::NameSpace, mjuu_stripext, mjuu_strippath
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_texture_name_space(self_ptr: *mut mjCTexture, m: *const mjCModel) {
    todo!() // mjCTexture::NameSpace
}

/// C: mjCTexture::Compile (user/user_objects.h:1468)
/// Calls: FilePath::Str, mjCTexture::Builtin2D, mjCTexture::BuiltinCube, mjCTexture::CopyFromSpec, mjCTexture::FlipIfNeeded, mjCTexture::Load2D, mjCTexture::LoadCubeSeparate, mjCTexture::LoadCubeSingle, mjs_getString, mjuu_strippath
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_texture_compile(self_ptr: *mut mjCTexture, vfs: *const mjVFS) {
    todo!() // mjCTexture::Compile
}

/// C: mjCTexture::File (user/user_objects.h:1471)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_texture_file(self_ptr: *mut mjCTexture) -> std__string {
    todo!() // mjCTexture::File
}

/// C: mjCTexture::get_content_type (user/user_objects.h:1472)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_texture_get_content_type(self_ptr: *mut mjCTexture) -> std__string {
    todo!() // mjCTexture::get_content_type
}

/// C: mjCTexture::get_cubefiles (user/user_objects.h:1473)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_texture_get_cubefiles(self_ptr: *mut mjCTexture) -> *const () {
    todo!() // mjCTexture::get_cubefiles
}

/// C: mjCTexture::GetCacheId (user/user_objects.h:1477)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_texture_get_cache_id(self_ptr: *mut mjCTexture, resource: *const mjResource, asset_type: *const std__string) -> std__string {
    todo!() // mjCTexture::GetCacheId
}

/// C: mjCTexture::Builtin2D (user/user_objects.h:1478)
/// Calls: checker, interp, randomdot
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_texture_builtin2d(self_ptr: *mut mjCTexture) {
    todo!() // mjCTexture::Builtin2D
}

/// C: mjCTexture::BuiltinCube (user/user_objects.h:1479)
/// Calls: checker, interp, randomdot
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_texture_builtin_cube(self_ptr: *mut mjCTexture) {
    todo!() // mjCTexture::BuiltinCube
}

/// C: mjCTexture::Load2D (user/user_objects.h:1480)
/// Calls: mjCTexture::LoadFlip
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_texture_load2d(self_ptr: *mut mjCTexture, filename: std__string, vfs: *const mjVFS) {
    todo!() // mjCTexture::Load2D
}

/// C: mjCTexture::LoadCubeSingle (user/user_objects.h:1481)
/// Calls: mjCTexture::LoadFlip
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_texture_load_cube_single(self_ptr: *mut mjCTexture, filename: std__string, vfs: *const mjVFS) {
    todo!() // mjCTexture::LoadCubeSingle
}

/// C: mjCTexture::LoadCubeSeparate (user/user_objects.h:1483)
/// Calls: FilePath::Str, mjCTexture::LoadFlip, mjs_getString, mjuu_strippath
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_texture_load_cube_separate(self_ptr: *mut mjCTexture, vfs: *const mjVFS) {
    todo!() // mjCTexture::LoadCubeSeparate
}

/// C: mjCTexture::FlipIfNeeded (user/user_objects.h:1485)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_texture_flip_if_needed(self_ptr: *mut mjCTexture, image: *const (), w: u32, h: u32) {
    todo!() // mjCTexture::FlipIfNeeded
}

/// C: mjCTexture::LoadFlip (user/user_objects.h:1487)
/// Calls: mjCBase::GetAssetContentType, mjCBase::LoadResource, mjCCache::Insert, mjCCache::PopulateData, mjCModel::FindSpec, mjCTexture::FlipIfNeeded, mjCTexture::GetCacheId, mjCTexture::LoadCustom, mjCTexture::LoadKTX, mjCTexture::LoadPNG, mj_getCache, mju_closeResource
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_texture_load_flip(self_ptr: *mut mjCTexture, filename: std__string, vfs: *const mjVFS, image: *const (), w: *mut u32, h: *mut u32, is_srgb: *mut bool) {
    todo!() // mjCTexture::LoadFlip
}

/// C: mjCTexture::LoadPNG (user/user_objects.h:1491)
/// Calls: PNGImage::Height, PNGImage::IsSRGB, PNGImage::Load, PNGImage::MoveData, PNGImage::Width
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_texture_load_png(self_ptr: *mut mjCTexture, resource: *mut mjResource, image: *const (), w: *mut u32, h: *mut u32, is_srgb: *mut bool) {
    todo!() // mjCTexture::LoadPNG
}

/// C: mjCTexture::LoadKTX (user/user_objects.h:1493)
/// Calls: mju_readResource
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_texture_load_ktx(self_ptr: *mut mjCTexture, resource: *mut mjResource, image: *const (), w: *mut u32, h: *mut u32, is_srgb: *mut bool) {
    todo!() // mjCTexture::LoadKTX
}

/// C: mjCTexture::LoadCustom (user/user_objects.h:1495)
/// Calls: mju_readResource
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_texture_load_custom(self_ptr: *mut mjCTexture, resource: *mut mjResource, image: *const (), w: *mut u32, h: *mut u32, is_srgb: *mut bool) {
    todo!() // mjCTexture::LoadCustom
}

/// C: mjCMaterial::CopyFromSpec (user/user_objects.h:1526)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_material_copy_from_spec(self_ptr: *mut mjCMaterial) {
    todo!() // mjCMaterial::CopyFromSpec
}

/// C: mjCMaterial::PointToLocal (user/user_objects.h:1527)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_material_point_to_local(self_ptr: *mut mjCMaterial) {
    // SAFETY: caller guarantees self_ptr is a valid pointer to mjCMaterial.
    // Offsets: mjsElement at +8, info at +80 (in _pad_0/mjCBase).
    unsafe {
        let base = self_ptr as *mut u8;
        (*self_ptr).spec.element = base.add(8) as *mut mjsElement;
        (*self_ptr).spec.textures = std::ptr::addr_of_mut!((*self_ptr).spec_textures_) as *mut mjStringVec;
        (*self_ptr).spec.info = base.add(80) as *mut mjString;
        (*self_ptr).textures = std::ptr::null_mut();
    }
}

/// C: mjCMaterial::NameSpace (user/user_objects.h:1528)
/// Calls: mjCBase::NameSpace
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_material_name_space(self_ptr: *mut mjCMaterial, m: *const mjCModel) {
    todo!() // mjCMaterial::NameSpace
}

/// C: mjCMaterial::get_texture (user/user_objects.h:1530)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_material_get_texture(self_ptr: *mut mjCMaterial, i: i32) -> *const std__string {
    todo!() // mjCMaterial::get_texture
}

/// C: mjCMaterial::del_textures (user/user_objects.h:1531)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_material_del_textures(self_ptr: *mut mjCMaterial) {
    todo!() // mjCMaterial::del_textures
}

/// C: mjCMaterial::Compile (user/user_objects.h:1534)
/// Calls: mjCMaterial::CopyFromSpec
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_material_compile(self_ptr: *mut mjCMaterial) {
    todo!() // mjCMaterial::Compile
}

/// C: mjCPair::CopyFromSpec (user/user_objects.h:1565)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_pair_copy_from_spec(self_ptr: *mut mjCPair) {
    todo!() // mjCPair::CopyFromSpec
}

/// C: mjCPair::PointToLocal (user/user_objects.h:1566)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_pair_point_to_local(self_ptr: *mut mjCPair) {
    // SAFETY: caller guarantees self_ptr is a valid pointer to mjCPair.
    // Offsets: mjsElement at +8, info at +80 (in _pad_0/mjCBase).
    unsafe {
        let base = self_ptr as *mut u8;
        (*self_ptr).spec.element = base.add(8) as *mut mjsElement;
        (*self_ptr).spec.geomname1 = std::ptr::addr_of_mut!((*self_ptr).spec_geomname1_) as *mut mjString;
        (*self_ptr).spec.geomname2 = std::ptr::addr_of_mut!((*self_ptr).spec_geomname2_) as *mut mjString;
        (*self_ptr).spec.info = base.add(80) as *mut mjString;
        (*self_ptr).geomname1 = std::ptr::null_mut();
        (*self_ptr).geomname2 = std::ptr::null_mut();
    }
}

/// C: mjCPair::ResolveReferences (user/user_objects.h:1567)
/// Calls: mjCModel::FindObject
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_pair_resolve_references(self_ptr: *mut mjCPair, m: *const mjCModel) {
    todo!() // mjCPair::ResolveReferences
}

/// C: mjCPair::NameSpace (user/user_objects.h:1568)
/// Calls: mjCBase::NameSpace
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_pair_name_space(self_ptr: *mut mjCPair, m: *const mjCModel) {
    todo!() // mjCPair::NameSpace
}

/// C: mjCPair::get_geomname1 (user/user_objects.h:1570)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_pair_get_geomname1(self_ptr: *mut mjCPair) -> *const std__string {
    todo!() // mjCPair::get_geomname1
}

/// C: mjCPair::get_geomname2 (user/user_objects.h:1571)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_pair_get_geomname2(self_ptr: *mut mjCPair) -> *const std__string {
    todo!() // mjCPair::get_geomname2
}

/// C: mjCPair::GetSignature (user/user_objects.h:1573)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_pair_get_signature(self_ptr: *mut mjCPair) -> i32 {
    // MACHINE-PROVEN: Clang AST direct field access
    unsafe { (*self_ptr).signature }
}

/// C: mjCPair::Compile (user/user_objects.h:1578)
/// Calls: mjCGeom::SetNotVisual, mjCPair::CopyFromSpec, mjCPair::ResolveReferences, mjuu_defined
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_pair_compile(self_ptr: *mut mjCPair) {
    todo!() // mjCPair::Compile
}

/// C: mjCBodyPair::CopyFromSpec (user/user_objects.h:1613)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_pair_copy_from_spec(self_ptr: *mut mjCBodyPair) {
    todo!() // mjCBodyPair::CopyFromSpec
}

/// C: mjCBodyPair::PointToLocal (user/user_objects.h:1614)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_pair_point_to_local(self_ptr: *mut mjCBodyPair) {
    // SAFETY: caller guarantees self_ptr is a valid pointer to mjCBodyPair.
    // Offsets: mjsElement at +8, info at +80 (in _pad_0/mjCBase).
    // mjsExclude layout: element(0), bodyname1(8), bodyname2(16), info(24) — all *mut pointers.
    unsafe {
        let base = self_ptr as *mut u8;
        let spec_ptr = std::ptr::addr_of_mut!((*self_ptr).spec) as *mut u8;
        // spec.element = self_ptr + 8 (mjsElement base)
        *(spec_ptr.add(0) as *mut *mut mjsElement) = base.add(8) as *mut mjsElement;
        // spec.bodyname1 = &spec_bodyname1_
        *(spec_ptr.add(8) as *mut *mut mjString) = std::ptr::addr_of_mut!((*self_ptr).spec_bodyname1_) as *mut mjString;
        // spec.bodyname2 = &spec_bodyname2_
        *(spec_ptr.add(16) as *mut *mut mjString) = std::ptr::addr_of_mut!((*self_ptr).spec_bodyname2_) as *mut mjString;
        // spec.info = &info (in mjCBase at offset 80)
        *(spec_ptr.add(24) as *mut *mut mjString) = base.add(80) as *mut mjString;
        (*self_ptr).bodyname1 = std::ptr::null_mut();
        (*self_ptr).bodyname2 = std::ptr::null_mut();
    }
}

/// C: mjCBodyPair::ResolveReferences (user/user_objects.h:1615)
/// Calls: mjCModel::FindObject
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_pair_resolve_references(self_ptr: *mut mjCBodyPair, m: *const mjCModel) {
    todo!() // mjCBodyPair::ResolveReferences
}

/// C: mjCBodyPair::NameSpace (user/user_objects.h:1616)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_pair_name_space(self_ptr: *mut mjCBodyPair, m: *const mjCModel) {
    todo!() // mjCBodyPair::NameSpace
}

/// C: mjCBodyPair::get_bodyname1 (user/user_objects.h:1618)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_pair_get_bodyname1(self_ptr: *mut mjCBodyPair) -> std__string {
    todo!() // mjCBodyPair::get_bodyname1
}

/// C: mjCBodyPair::get_bodyname2 (user/user_objects.h:1619)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_pair_get_bodyname2(self_ptr: *mut mjCBodyPair) -> std__string {
    todo!() // mjCBodyPair::get_bodyname2
}

/// C: mjCBodyPair::GetSignature (user/user_objects.h:1621)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_pair_get_signature(self_ptr: *mut mjCBodyPair) -> i32 {
    // MACHINE-PROVEN: Clang AST direct field access
    unsafe { (*self_ptr).signature }
}

/// C: mjCBodyPair::Compile (user/user_objects.h:1626)
/// Calls: mjCBodyPair::CopyFromSpec, mjCBodyPair::ResolveReferences
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_body_pair_compile(self_ptr: *mut mjCBodyPair) {
    todo!() // mjCBodyPair::Compile
}

/// C: mjCEquality::CopyFromSpec (user/user_objects.h:1658)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_equality_copy_from_spec(self_ptr: *mut mjCEquality) {
    todo!() // mjCEquality::CopyFromSpec
}

/// C: mjCEquality::PointToLocal (user/user_objects.h:1659)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_equality_point_to_local(self_ptr: *mut mjCEquality) {
    // SAFETY: caller guarantees self_ptr is a valid pointer to mjCEquality.
    // Offsets: mjsElement at +8, info at +80 (in _pad_0/mjCBase).
    unsafe {
        let base = self_ptr as *mut u8;
        (*self_ptr).spec.element = base.add(8) as *mut mjsElement;
        (*self_ptr).spec.name1 = std::ptr::addr_of_mut!((*self_ptr).spec_name1_) as *mut mjString;
        (*self_ptr).spec.name2 = std::ptr::addr_of_mut!((*self_ptr).spec_name2_) as *mut mjString;
        (*self_ptr).spec.info = base.add(80) as *mut mjString;
        (*self_ptr).name1 = std::ptr::null_mut();
        (*self_ptr).name2 = std::ptr::null_mut();
    }
}

/// C: mjCEquality::ResolveReferences (user/user_objects.h:1660)
/// Calls: mjCModel::FindObject
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_equality_resolve_references(self_ptr: *mut mjCEquality, m: *const mjCModel) {
    todo!() // mjCEquality::ResolveReferences
}

/// C: mjCEquality::NameSpace (user/user_objects.h:1661)
/// Calls: mjCBase::NameSpace
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_equality_name_space(self_ptr: *mut mjCEquality, m: *const mjCModel) {
    todo!() // mjCEquality::NameSpace
}

/// C: mjCEquality::Compile (user/user_objects.h:1664)
/// Calls: mjCEquality::CopyFromSpec, mjCEquality::ResolveReferences, mjCModel::Flexes
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_equality_compile(self_ptr: *mut mjCEquality) {
    todo!() // mjCEquality::Compile
}

/// C: mjCTendon::set_material (user/user_objects.h:1697)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tendon_set_material(self_ptr: *mut mjCTendon, _material: std__string) {
    todo!() // mjCTendon::set_material
}

/// C: mjCTendon::get_material (user/user_objects.h:1698)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tendon_get_material(self_ptr: *mut mjCTendon) -> *const std__string {
    todo!() // mjCTendon::get_material
}

/// C: mjCTendon::del_material (user/user_objects.h:1699)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tendon_del_material(self_ptr: *mut mjCTendon) {
    todo!() // mjCTendon::del_material
}

/// C: mjCTendon::WrapSite (user/user_objects.h:1702)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tendon_wrap_site(self_ptr: *mut mjCTendon, wrapname: std__string, wrapinfo: std__string_view) {
    todo!() // mjCTendon::WrapSite
}

/// C: mjCTendon::WrapGeom (user/user_objects.h:1703)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tendon_wrap_geom(self_ptr: *mut mjCTendon, wrapname: std__string, side: std__string, wrapinfo: std__string_view) {
    todo!() // mjCTendon::WrapGeom
}

/// C: mjCTendon::WrapJoint (user/user_objects.h:1704)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tendon_wrap_joint(self_ptr: *mut mjCTendon, wrapname: std__string, coef: f64, wrapinfo: std__string_view) {
    todo!() // mjCTendon::WrapJoint
}

/// C: mjCTendon::WrapPulley (user/user_objects.h:1705)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tendon_wrap_pulley(self_ptr: *mut mjCTendon, divisor: f64, wrapinfo: std__string_view) {
    todo!() // mjCTendon::WrapPulley
}

/// C: mjCTendon::NumWraps (user/user_objects.h:1708)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tendon_num_wraps(self_ptr: *mut mjCTendon) -> i32 {
    todo!() // mjCTendon::NumWraps
}

/// C: mjCTendon::GetWrap (user/user_objects.h:1709)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tendon_get_wrap(self_ptr: *mut mjCTendon, i: i32) -> *const mjCWrap {
    todo!() // mjCTendon::GetWrap
}

/// C: mjCTendon::get_userdata (user/user_objects.h:1713)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tendon_get_userdata(self_ptr: *mut mjCTendon) -> *const () {
    todo!() // mjCTendon::get_userdata
}

/// C: mjCTendon::get_range (user/user_objects.h:1714)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tendon_get_range(self_ptr: *mut mjCTendon) -> *const f64 {
    todo!() // mjCTendon::get_range
}

/// C: mjCTendon::CopyFromSpec (user/user_objects.h:1716)
/// Calls: mjCWrap::Type
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tendon_copy_from_spec(self_ptr: *mut mjCTendon) {
    todo!() // mjCTendon::CopyFromSpec
}

/// C: mjCTendon::PointToLocal (user/user_objects.h:1717)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tendon_point_to_local(self_ptr: *mut mjCTendon) {
    // SAFETY: caller guarantees self_ptr is a valid pointer to mjCTendon.
    // Offsets: mjsElement at +8, info at +80 (in _pad_0/mjCBase).
    unsafe {
        let base = self_ptr as *mut u8;
        (*self_ptr).spec.element = base.add(8) as *mut mjsElement;
        (*self_ptr).spec.material = std::ptr::addr_of_mut!((*self_ptr).spec_material_) as *mut mjString;
        (*self_ptr).spec.userdata = std::ptr::addr_of_mut!((*self_ptr).spec_userdata_) as *mut mjDoubleVec;
        (*self_ptr).spec.info = base.add(80) as *mut mjString;
        (*self_ptr).material = std::ptr::null_mut();
        (*self_ptr).userdata = std::ptr::null_mut();
    }
}

/// C: mjCTendon::ResolveReferences (user/user_objects.h:1718)
/// Calls: mjCWrap::ResolveReferences, mjCWrap::Type
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tendon_resolve_references(self_ptr: *mut mjCTendon, m: *const mjCModel) {
    todo!() // mjCTendon::ResolveReferences
}

/// C: mjCTendon::NameSpace (user/user_objects.h:1719)
/// Calls: mjCBase::NameSpace
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tendon_name_space(self_ptr: *mut mjCTendon, m: *const mjCModel) {
    todo!() // mjCTendon::NameSpace
}

/// C: mjCTendon::SetModel (user/user_objects.h:1720)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tendon_set_model(self_ptr: *mut mjCTendon, _model: *mut mjCModel) {
    todo!() // mjCTendon::SetModel
}

/// C: mjCTendon::is_limited (user/user_objects.h:1722)
/// Calls: islimited
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tendon_is_limited(self_ptr: *mut mjCTendon) -> bool {
    // SAFETY: self_ptr is valid pointer to mjCTendon
    unsafe {
        islimited((*self_ptr).limited, (*self_ptr).range.as_ptr())
    }
}

/// C: mjCTendon::is_actfrclimited (user/user_objects.h:1723)
/// Calls: islimited
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tendon_is_actfrclimited(self_ptr: *mut mjCTendon) -> bool {
    // SAFETY: self_ptr is valid pointer to mjCTendon
    unsafe {
        islimited((*self_ptr).actfrclimited, (*self_ptr).actfrcrange.as_ptr())
    }
}

/// C: mjCTendon::Compile (user/user_objects.h:1726)
/// Calls: checklimited, mjCGeom::SetNotVisual, mjCModel::Geoms, mjCTendon::CopyFromSpec, mjCTendon::ResolveReferences, mjCTendon::is_actfrclimited, mjCTendon::is_limited, mjCWrap::Compile, mjCWrap::Type
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tendon_compile(self_ptr: *mut mjCTendon) {
    todo!() // mjCTendon::Compile
}

/// C: mjCWrap::CopyFromSpec (user/user_objects.h:1749)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_wrap_copy_from_spec(self_ptr: *mut mjCWrap) {
    todo!() // mjCWrap::CopyFromSpec
}

/// C: mjCWrap::PointToLocal (user/user_objects.h:1750)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_wrap_point_to_local(self_ptr: *mut mjCWrap) {
    // SAFETY: caller guarantees self_ptr is a valid pointer to mjCWrap.
    // Offsets: mjsElement at +8, info at +80 (in _pad_0/mjCBase).
    unsafe {
        let base = self_ptr as *mut u8;
        (*self_ptr).spec.element = base.add(8) as *mut mjsElement;
        (*self_ptr).spec.info = base.add(80) as *mut mjString;
    }
}

/// C: mjCWrap::ResolveReferences (user/user_objects.h:1751)
/// Calls: mjCModel::FindObject
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_wrap_resolve_references(self_ptr: *mut mjCWrap, m: *const mjCModel) {
    todo!() // mjCWrap::ResolveReferences
}

/// C: mjCWrap::NameSpace (user/user_objects.h:1752)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_wrap_name_space(self_ptr: *mut mjCWrap, m: *const mjCModel) {
    todo!() // mjCWrap::NameSpace
}

/// C: mjCWrap::Type (user/user_objects.h:1753)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_wrap_type(self_ptr: *mut mjCWrap) -> u32 {
    todo!() // mjCWrap::Type
}

/// C: mjCWrap::Compile (user/user_objects.h:1762)
/// Calls: mjCWrap::CopyFromSpec
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_wrap_compile(self_ptr: *mut mjCWrap) {
    todo!() // mjCWrap::Compile
}

/// C: mjCPlugin::PointToLocal (user/user_objects.h:1791)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_plugin_point_to_local(self_ptr: *mut mjCPlugin) {
    // SAFETY: caller guarantees self_ptr is a valid pointer to mjCPlugin.
    // Offsets: mjsElement at +8, name at +32, info at +80 (in _pad_0/mjCBase).
    unsafe {
        let base = self_ptr as *mut u8;
        (*self_ptr).spec.element = base.add(8) as *mut mjsElement;
        (*self_ptr).spec.name = base.add(32) as *mut mjString;
        (*self_ptr).spec.info = base.add(80) as *mut mjString;
    }
}

/// C: mjCPlugin::Compile (user/user_objects.h:1798)
/// Calls: mjCModel::ResolvePlugin, mjp_getPluginAtSlot
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_plugin_compile(self_ptr: *mut mjCPlugin) {
    todo!() // mjCPlugin::Compile
}

/// C: mjCActuator::get_userdata (user/user_objects.h:1843)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_actuator_get_userdata(self_ptr: *mut mjCActuator) -> *const () {
    todo!() // mjCActuator::get_userdata
}

/// C: mjCActuator::get_target (user/user_objects.h:1844)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_actuator_get_target(self_ptr: *mut mjCActuator) -> *const std__string {
    todo!() // mjCActuator::get_target
}

/// C: mjCActuator::get_slidersite (user/user_objects.h:1845)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_actuator_get_slidersite(self_ptr: *mut mjCActuator) -> *const std__string {
    todo!() // mjCActuator::get_slidersite
}

/// C: mjCActuator::get_refsite (user/user_objects.h:1846)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_actuator_get_refsite(self_ptr: *mut mjCActuator) -> *const std__string {
    todo!() // mjCActuator::get_refsite
}

/// C: mjCActuator::is_ctrllimited (user/user_objects.h:1848)
/// Calls: islimited
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_actuator_is_ctrllimited(self_ptr: *mut mjCActuator) -> bool {
    // SAFETY: self_ptr is valid pointer to mjCActuator
    unsafe {
        islimited((*self_ptr).ctrllimited, (*self_ptr).ctrlrange.as_ptr())
    }
}

/// C: mjCActuator::is_forcelimited (user/user_objects.h:1849)
/// Calls: islimited
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_actuator_is_forcelimited(self_ptr: *mut mjCActuator) -> bool {
    // SAFETY: self_ptr is valid pointer to mjCActuator
    unsafe {
        islimited((*self_ptr).forcelimited, (*self_ptr).forcerange.as_ptr())
    }
}

/// C: mjCActuator::is_actlimited (user/user_objects.h:1850)
/// Calls: islimited
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_actuator_is_actlimited(self_ptr: *mut mjCActuator) -> bool {
    // SAFETY: self_ptr is valid pointer to mjCActuator
    unsafe {
        islimited((*self_ptr).actlimited, (*self_ptr).actrange.as_ptr())
    }
}

/// C: mjCActuator::act (user/user_objects.h:1852)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_actuator_act(self_ptr: *mut mjCActuator, state_name: *const std__string) -> *const () {
    todo!() // mjCActuator::act
}

/// C: mjCActuator::ctrl (user/user_objects.h:1853)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_actuator_ctrl(self_ptr: *mut mjCActuator, state_name: *const std__string) -> *mut f64 {
    todo!() // mjCActuator::ctrl
}

/// C: mjCActuator::Compile (user/user_objects.h:1856)
/// Calls: checklimited, mjCActuator::CopyFromSpec, mjCActuator::ResolveReferences, mjCActuator::is_actlimited, mjCActuator::is_ctrllimited, mjCActuator::is_forcelimited, mjCJoint::get_range, mjCModel::ResolvePlugin, mjCTendon::get_range, mjp_getPluginAtSlot
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_actuator_compile(self_ptr: *mut mjCActuator) {
    todo!() // mjCActuator::Compile
}

/// C: mjCActuator::CopyFromSpec (user/user_objects.h:1857)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_actuator_copy_from_spec(self_ptr: *mut mjCActuator) {
    todo!() // mjCActuator::CopyFromSpec
}

/// C: mjCActuator::PointToLocal (user/user_objects.h:1858)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_actuator_point_to_local(self_ptr: *mut mjCActuator) {
    // SAFETY: caller guarantees self_ptr is a valid pointer to mjCActuator.
    // Offsets: mjsElement at +8, info at +80 (in _pad_0/mjCBase).
    unsafe {
        let base = self_ptr as *mut u8;
        (*self_ptr).spec.element = base.add(8) as *mut mjsElement;
        (*self_ptr).spec.userdata = std::ptr::addr_of_mut!((*self_ptr).spec_userdata_) as *mut mjDoubleVec;
        (*self_ptr).spec.target = std::ptr::addr_of_mut!((*self_ptr).spec_target_) as *mut mjString;
        (*self_ptr).spec.refsite = std::ptr::addr_of_mut!((*self_ptr).spec_refsite_) as *mut mjString;
        (*self_ptr).spec.slidersite = std::ptr::addr_of_mut!((*self_ptr).spec_slidersite_) as *mut mjString;
        (*self_ptr).spec.plugin.plugin_name = std::ptr::addr_of_mut!((*self_ptr).plugin_name) as *mut mjString;
        (*self_ptr).spec.plugin.name = std::ptr::addr_of_mut!((*self_ptr).plugin_instance_name) as *mut mjString;
        (*self_ptr).spec.info = base.add(80) as *mut mjString;
        (*self_ptr).userdata = std::ptr::null_mut();
        (*self_ptr).target = std::ptr::null_mut();
        (*self_ptr).refsite = std::ptr::null_mut();
        (*self_ptr).slidersite = std::ptr::null_mut();
    }
}

/// C: mjCActuator::ResolveReferences (user/user_objects.h:1859)
/// Calls: mjCModel::FindObject
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_actuator_resolve_references(self_ptr: *mut mjCActuator, m: *const mjCModel) {
    todo!() // mjCActuator::ResolveReferences
}

/// C: mjCActuator::NameSpace (user/user_objects.h:1860)
/// Calls: mjCBase::NameSpace
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_actuator_name_space(self_ptr: *mut mjCActuator, m: *const mjCModel) {
    todo!() // mjCActuator::NameSpace
}

/// C: mjCActuator::CopyPlugin (user/user_objects.h:1861)
/// Calls: mjCModel::CopyExplicitPlugin
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_actuator_copy_plugin(self_ptr: *mut mjCActuator) {
    todo!() // mjCActuator::CopyPlugin
}

/// C: mjCActuator::ForgetKeyframes (user/user_objects.h:1864)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_actuator_forget_keyframes(self_ptr: *mut mjCActuator) {
    todo!() // mjCActuator::ForgetKeyframes
}

/// C: mjCSensor::get_userdata (user/user_objects.h:1901)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_sensor_get_userdata(self_ptr: *mut mjCSensor) -> *const () {
    todo!() // mjCSensor::get_userdata
}

/// C: mjCSensor::get_objname (user/user_objects.h:1902)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_sensor_get_objname(self_ptr: *mut mjCSensor) -> *const std__string {
    todo!() // mjCSensor::get_objname
}

/// C: mjCSensor::get_refname (user/user_objects.h:1903)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_sensor_get_refname(self_ptr: *mut mjCSensor) -> *const std__string {
    todo!() // mjCSensor::get_refname
}

/// C: mjCSensor::get_obj (user/user_objects.h:1905)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_sensor_get_obj(self_ptr: *mut mjCSensor) -> *const mjCBase {
    todo!() // mjCSensor::get_obj
}

/// C: mjCSensor::get_ref (user/user_objects.h:1906)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_sensor_get_ref(self_ptr: *mut mjCSensor) -> *const mjCBase {
    todo!() // mjCSensor::get_ref
}

/// C: mjCSensor::Compile (user/user_objects.h:1909)
/// Calls: mjCGeom::Type, mjCJoint::is_limited, mjCModel::ResolvePlugin, mjCSensor::CopyFromSpec, mjCSensor::ResolveReferences, mjCTendon::is_limited, mjp_getPluginAtSlot, mjs_sensorDim, sensorDatatype, sensorNeedstage
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_sensor_compile(self_ptr: *mut mjCSensor) {
    todo!() // mjCSensor::Compile
}

/// C: mjCSensor::CopyFromSpec (user/user_objects.h:1910)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_sensor_copy_from_spec(self_ptr: *mut mjCSensor) {
    todo!() // mjCSensor::CopyFromSpec
}

/// C: mjCSensor::PointToLocal (user/user_objects.h:1911)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_sensor_point_to_local(self_ptr: *mut mjCSensor) {
    // SAFETY: caller guarantees self_ptr is a valid pointer to mjCSensor.
    // Offsets: mjsElement at +8, info at +80 (in _pad_0/mjCBase).
    unsafe {
        let base = self_ptr as *mut u8;
        (*self_ptr).spec.element = base.add(8) as *mut mjsElement;
        (*self_ptr).spec.userdata = std::ptr::addr_of_mut!((*self_ptr).spec_userdata_) as *mut mjDoubleVec;
        (*self_ptr).spec.objname = std::ptr::addr_of_mut!((*self_ptr).spec_objname_) as *mut mjString;
        (*self_ptr).spec.refname = std::ptr::addr_of_mut!((*self_ptr).spec_refname_) as *mut mjString;
        (*self_ptr).spec.plugin.plugin_name = std::ptr::addr_of_mut!((*self_ptr).plugin_name) as *mut mjString;
        (*self_ptr).spec.plugin.name = std::ptr::addr_of_mut!((*self_ptr).plugin_instance_name) as *mut mjString;
        (*self_ptr).spec.info = base.add(80) as *mut mjString;
        (*self_ptr).userdata = std::ptr::null_mut();
        (*self_ptr).objname = std::ptr::null_mut();
        (*self_ptr).refname = std::ptr::null_mut();
    }
}

/// C: mjCSensor::ResolveReferences (user/user_objects.h:1912)
/// Calls: mjCGeom::SetNotVisual, mjCMesh::SetNotVisual, mjCModel::FindObject
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_sensor_resolve_references(self_ptr: *mut mjCSensor, m: *const mjCModel) {
    todo!() // mjCSensor::ResolveReferences
}

/// C: mjCSensor::NameSpace (user/user_objects.h:1913)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_sensor_name_space(self_ptr: *mut mjCSensor, m: *const mjCModel) {
    todo!() // mjCSensor::NameSpace
}

/// C: mjCSensor::CopyPlugin (user/user_objects.h:1914)
/// Calls: mjCModel::CopyExplicitPlugin
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_sensor_copy_plugin(self_ptr: *mut mjCSensor) {
    todo!() // mjCSensor::CopyPlugin
}

/// C: mjCNumeric::PointToLocal (user/user_objects.h:1944)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_numeric_point_to_local(self_ptr: *mut mjCNumeric) {
    // SAFETY: caller guarantees self_ptr is a valid pointer to mjCNumeric.
    // Offsets: mjsElement at +8, info at +80 (in _pad_0/mjCBase).
    unsafe {
        let base = self_ptr as *mut u8;
        (*self_ptr).spec.element = base.add(8) as *mut mjsElement;
        (*self_ptr).spec.data = std::ptr::addr_of_mut!((*self_ptr).spec_data_) as *mut mjDoubleVec;
        (*self_ptr).spec.info = base.add(80) as *mut mjString;
        (*self_ptr).data = std::ptr::null_mut();
    }
}

/// C: mjCNumeric::CopyFromSpec (user/user_objects.h:1945)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_numeric_copy_from_spec(self_ptr: *mut mjCNumeric) {
    todo!() // mjCNumeric::CopyFromSpec
}

/// C: mjCNumeric::Compile (user/user_objects.h:1948)
/// Calls: mjCNumeric::CopyFromSpec
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_numeric_compile(self_ptr: *mut mjCNumeric) {
    todo!() // mjCNumeric::Compile
}

/// C: mjCText::PointToLocal (user/user_objects.h:1975)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_text_point_to_local(self_ptr: *mut mjCText) {
    // SAFETY: caller guarantees self_ptr is a valid pointer to mjCText.
    // Offsets: mjsElement at +8, info at +80 (in _pad_0/mjCBase).
    unsafe {
        let base = self_ptr as *mut u8;
        (*self_ptr).spec.element = base.add(8) as *mut mjsElement;
        (*self_ptr).spec.data = std::ptr::addr_of_mut!((*self_ptr).spec_data_) as *mut mjString;
        (*self_ptr).spec.info = base.add(80) as *mut mjString;
        (*self_ptr).data = std::ptr::null_mut();
    }
}

/// C: mjCText::CopyFromSpec (user/user_objects.h:1976)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_text_copy_from_spec(self_ptr: *mut mjCText) {
    todo!() // mjCText::CopyFromSpec
}

/// C: mjCText::Compile (user/user_objects.h:1979)
/// Calls: mjCText::CopyFromSpec
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_text_compile(self_ptr: *mut mjCText) {
    todo!() // mjCText::Compile
}

/// C: mjCTuple::PointToLocal (user/user_objects.h:2011)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tuple_point_to_local(self_ptr: *mut mjCTuple) {
    // SAFETY: caller guarantees self_ptr is a valid pointer to mjCTuple.
    // Offsets: mjsElement at +8, info at +80 (in _pad_0/mjCBase).
    unsafe {
        let base = self_ptr as *mut u8;
        (*self_ptr).spec.element = base.add(8) as *mut mjsElement;
        (*self_ptr).spec.objtype = std::ptr::addr_of_mut!((*self_ptr).spec_objtype_) as *mut mjIntVec;
        (*self_ptr).spec.objname = std::ptr::addr_of_mut!((*self_ptr).spec_objname_) as *mut mjStringVec;
        (*self_ptr).spec.objprm = std::ptr::addr_of_mut!((*self_ptr).spec_objprm_) as *mut mjDoubleVec;
        (*self_ptr).spec.info = base.add(80) as *mut mjString;
        (*self_ptr).objname = std::ptr::null_mut();
        (*self_ptr).objprm = std::ptr::null_mut();
    }
}

/// C: mjCTuple::CopyFromSpec (user/user_objects.h:2012)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tuple_copy_from_spec(self_ptr: *mut mjCTuple) {
    todo!() // mjCTuple::CopyFromSpec
}

/// C: mjCTuple::ResolveReferences (user/user_objects.h:2013)
/// Calls: mjCGeom::SetNotVisual, mjCModel::FindObject
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tuple_resolve_references(self_ptr: *mut mjCTuple, m: *const mjCModel) {
    todo!() // mjCTuple::ResolveReferences
}

/// C: mjCTuple::NameSpace (user/user_objects.h:2014)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tuple_name_space(self_ptr: *mut mjCTuple, m: *const mjCModel) {
    todo!() // mjCTuple::NameSpace
}

/// C: mjCTuple::Compile (user/user_objects.h:2017)
/// Calls: mjCTuple::CopyFromSpec, mjCTuple::ResolveReferences
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_tuple_compile(self_ptr: *mut mjCTuple) {
    todo!() // mjCTuple::Compile
}

/// C: mjCKey::PointToLocal (user/user_objects.h:2054)
/// Calls: mjCActuator::act, mjCActuator::ctrl, mjCBody::mpos, mjCBody::mquat, mjCJoint::qpos, mjCJoint::qvel
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_key_point_to_local(self_ptr: *mut mjCKey) {
    // SAFETY: caller guarantees self_ptr is a valid pointer to mjCKey.
    // Offsets: mjsElement at +8, info at +80 (in _pad_0/mjCBase).
    unsafe {
        let base = self_ptr as *mut u8;
        (*self_ptr).spec.element = base.add(8) as *mut mjsElement;
        (*self_ptr).spec.qpos = std::ptr::addr_of_mut!((*self_ptr).spec_qpos_) as *mut mjDoubleVec;
        (*self_ptr).spec.qvel = std::ptr::addr_of_mut!((*self_ptr).spec_qvel_) as *mut mjDoubleVec;
        (*self_ptr).spec.act = std::ptr::addr_of_mut!((*self_ptr).spec_act_) as *mut mjDoubleVec;
        (*self_ptr).spec.mpos = std::ptr::addr_of_mut!((*self_ptr).spec_mpos_) as *mut mjDoubleVec;
        (*self_ptr).spec.mquat = std::ptr::addr_of_mut!((*self_ptr).spec_mquat_) as *mut mjDoubleVec;
        (*self_ptr).spec.ctrl = std::ptr::addr_of_mut!((*self_ptr).spec_ctrl_) as *mut mjDoubleVec;
        (*self_ptr).spec.info = base.add(80) as *mut mjString;
        (*self_ptr).qpos = std::ptr::null_mut();
        (*self_ptr).qvel = std::ptr::null_mut();
        (*self_ptr).act = std::ptr::null_mut();
        (*self_ptr).mpos = std::ptr::null_mut();
        (*self_ptr).mquat = std::ptr::null_mut();
        (*self_ptr).ctrl = std::ptr::null_mut();
    }
}

/// C: mjCKey::CopyFromSpec (user/user_objects.h:2055)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_key_copy_from_spec(self_ptr: *mut mjCKey) {
    todo!() // mjCKey::CopyFromSpec
}

/// C: mjCKey::Compile (user/user_objects.h:2058)
/// Calls: mjCKey::CopyFromSpec
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_key_compile(self_ptr: *mut mjCKey, m: *const mjModel) {
    todo!() // mjCKey::Compile
}

/// C: mjCDef::CopyWithoutChildren (user/user_objects.h:2076)
/// Calls: mjCDef::PointToLocal
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_def_copy_without_children(self_ptr: *mut mjCDef, other: *const mjCDef) {
    todo!() // mjCDef::CopyWithoutChildren
}

/// C: mjCDef::PointToLocal (user/user_objects.h:2077)
/// Calls: mjCActuator::PointToLocal, mjCCamera::PointToLocal, mjCEquality::PointToLocal, mjCFlex::PointToLocal, mjCGeom::PointToLocal, mjCJoint::PointToLocal, mjCLight::PointToLocal, mjCMaterial::PointToLocal, mjCMesh::PointToLocal, mjCPair::PointToLocal, mjCSite::PointToLocal, mjCTendon::PointToLocal
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_def_point_to_local(self_ptr: *mut mjCDef) {
    // SAFETY: caller guarantees self_ptr is a valid pointer to mjCDef.
    // mjCDef starts with mjsElement directly (no vtable), so element is at offset 0.
    unsafe {
        // Call PointToLocal on each embedded sub-object
        mj_c_joint_point_to_local(std::ptr::addr_of_mut!((*self_ptr).joint_));
        mj_c_geom_point_to_local(std::ptr::addr_of_mut!((*self_ptr).geom_));
        mj_c_site_point_to_local(std::ptr::addr_of_mut!((*self_ptr).site_));
        mj_c_camera_point_to_local(std::ptr::addr_of_mut!((*self_ptr).camera_));
        mj_c_light_point_to_local(std::ptr::addr_of_mut!((*self_ptr).light_));
        mj_c_flex_point_to_local(std::ptr::addr_of_mut!((*self_ptr).flex_));
        mj_c_mesh_point_to_local(std::ptr::addr_of_mut!((*self_ptr).mesh_));
        mj_c_material_point_to_local(std::ptr::addr_of_mut!((*self_ptr).material_));
        mj_c_pair_point_to_local(std::ptr::addr_of_mut!((*self_ptr).pair_));
        mj_c_equality_point_to_local(std::ptr::addr_of_mut!((*self_ptr).equality_));
        mj_c_tendon_point_to_local(std::ptr::addr_of_mut!((*self_ptr).tendon_));
        mj_c_actuator_point_to_local(std::ptr::addr_of_mut!((*self_ptr).actuator_));

        // Set spec pointer fields
        (*self_ptr).spec.element = self_ptr as *mut mjsElement;
        (*self_ptr).spec.joint = std::ptr::addr_of_mut!((*self_ptr).joint_.spec) as *mut mjsJoint;
        (*self_ptr).spec.geom = std::ptr::addr_of_mut!((*self_ptr).geom_.spec) as *mut mjsGeom;
        (*self_ptr).spec.site = std::ptr::addr_of_mut!((*self_ptr).site_.spec) as *mut mjsSite;
        (*self_ptr).spec.camera = std::ptr::addr_of_mut!((*self_ptr).camera_.spec) as *mut mjsCamera;
        (*self_ptr).spec.light = std::ptr::addr_of_mut!((*self_ptr).light_.spec) as *mut mjsLight;
        (*self_ptr).spec.flex = std::ptr::addr_of_mut!((*self_ptr).flex_.spec) as *mut mjsFlex;
        (*self_ptr).spec.mesh = std::ptr::addr_of_mut!((*self_ptr).mesh_.spec) as *mut mjsMesh;
        (*self_ptr).spec.material = std::ptr::addr_of_mut!((*self_ptr).material_.spec) as *mut mjsMaterial;
        (*self_ptr).spec.pair = std::ptr::addr_of_mut!((*self_ptr).pair_.spec) as *mut mjsPair;
        (*self_ptr).spec.equality = std::ptr::addr_of_mut!((*self_ptr).equality_.spec) as *mut mjsEquality;
        (*self_ptr).spec.tendon = std::ptr::addr_of_mut!((*self_ptr).tendon_.spec) as *mut mjsTendon;
        (*self_ptr).spec.actuator = std::ptr::addr_of_mut!((*self_ptr).actuator_.spec) as *mut mjsActuator;
    }
}

/// C: mjCDef::CopyFromSpec (user/user_objects.h:2078)
/// Calls: mjCActuator::CopyFromSpec, mjCCamera::CopyFromSpec, mjCEquality::CopyFromSpec, mjCFlex::CopyFromSpec, mjCGeom::CopyFromSpec, mjCJoint::CopyFromSpec, mjCLight::CopyFromSpec, mjCMaterial::CopyFromSpec, mjCMesh::CopyFromSpec, mjCPair::CopyFromSpec, mjCSite::CopyFromSpec, mjCTendon::CopyFromSpec
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_def_copy_from_spec(self_ptr: *mut mjCDef) {
    todo!() // mjCDef::CopyFromSpec
}

/// C: mjCDef::NameSpace (user/user_objects.h:2079)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_def_name_space(self_ptr: *mut mjCDef, m: *const mjCModel) {
    todo!() // mjCDef::NameSpace
}

/// C: mjCDef::Compile (user/user_objects.h:2081)
/// Calls: mjCDef::CopyFromSpec
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_def_compile(self_ptr: *mut mjCDef, model: *const mjCModel) {
    todo!() // mjCDef::Compile
}

/// C: mjCDef::Joint (user/user_objects.h:2084)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_def_joint(self_ptr: *mut mjCDef) -> *mut mjCJoint {
    // MACHINE-PROVEN: Clang AST direct field access
    unsafe { core::ptr::addr_of_mut!((*self_ptr).joint_).cast::<mjCJoint>() }
}

/// C: mjCDef::Geom (user/user_objects.h:2085)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_def_geom(self_ptr: *mut mjCDef) -> *mut mjCGeom {
    // MACHINE-PROVEN: Clang AST direct field access
    unsafe { core::ptr::addr_of_mut!((*self_ptr).geom_).cast::<mjCGeom>() }
}

/// C: mjCDef::Site (user/user_objects.h:2086)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_def_site(self_ptr: *mut mjCDef) -> *mut mjCSite {
    // MACHINE-PROVEN: Clang AST direct field access
    unsafe { core::ptr::addr_of_mut!((*self_ptr).site_).cast::<mjCSite>() }
}

/// C: mjCDef::Camera (user/user_objects.h:2087)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_def_camera(self_ptr: *mut mjCDef) -> *mut mjCCamera {
    // MACHINE-PROVEN: Clang AST direct field access
    unsafe { core::ptr::addr_of_mut!((*self_ptr).camera_).cast::<mjCCamera>() }
}

/// C: mjCDef::Light (user/user_objects.h:2088)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_def_light(self_ptr: *mut mjCDef) -> *mut mjCLight {
    // MACHINE-PROVEN: Clang AST direct field access
    unsafe { core::ptr::addr_of_mut!((*self_ptr).light_).cast::<mjCLight>() }
}

/// C: mjCDef::Flex (user/user_objects.h:2089)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_def_flex(self_ptr: *mut mjCDef) -> *mut mjCFlex {
    // MACHINE-PROVEN: Clang AST direct field access
    unsafe { core::ptr::addr_of_mut!((*self_ptr).flex_).cast::<mjCFlex>() }
}

/// C: mjCDef::Mesh (user/user_objects.h:2090)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_def_mesh(self_ptr: *mut mjCDef) -> *mut mjCMesh {
    // MACHINE-PROVEN: Clang AST direct field access
    unsafe { core::ptr::addr_of_mut!((*self_ptr).mesh_).cast::<mjCMesh>() }
}

/// C: mjCDef::Material (user/user_objects.h:2091)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_def_material(self_ptr: *mut mjCDef) -> *mut mjCMaterial {
    // MACHINE-PROVEN: Clang AST direct field access
    unsafe { core::ptr::addr_of_mut!((*self_ptr).material_).cast::<mjCMaterial>() }
}

/// C: mjCDef::Pair (user/user_objects.h:2092)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_def_pair(self_ptr: *mut mjCDef) -> *mut mjCPair {
    // MACHINE-PROVEN: Clang AST direct field access
    unsafe { core::ptr::addr_of_mut!((*self_ptr).pair_).cast::<mjCPair>() }
}

/// C: mjCDef::Equality (user/user_objects.h:2093)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_def_equality(self_ptr: *mut mjCDef) -> *mut mjCEquality {
    // MACHINE-PROVEN: Clang AST direct field access
    unsafe { core::ptr::addr_of_mut!((*self_ptr).equality_).cast::<mjCEquality>() }
}

/// C: mjCDef::Tendon (user/user_objects.h:2094)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_def_tendon(self_ptr: *mut mjCDef) -> *mut mjCTendon {
    // MACHINE-PROVEN: Clang AST direct field access
    unsafe { core::ptr::addr_of_mut!((*self_ptr).tendon_).cast::<mjCTendon>() }
}

/// C: mjCDef::Actuator (user/user_objects.h:2095)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_def_actuator(self_ptr: *mut mjCDef) -> *mut mjCActuator {
    // MACHINE-PROVEN: Clang AST direct field access
    unsafe { core::ptr::addr_of_mut!((*self_ptr).actuator_).cast::<mjCActuator>() }
}

