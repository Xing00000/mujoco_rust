//! Port of: user/user_objects.cc
//! IR hash: 9343293228317031
//! CODEGEN: source paths, owners, and callable names are locked.

use crate::types::*;

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

/// C: sensorDatatype (user/user_objects.cc:7480)
#[allow(unused_variables, non_snake_case)]
pub fn sensorDatatype(r#type: u32) -> u32 {
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
pub fn sensorNeedstage(r#type: u32) -> u32 {
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

pub use crate::types::PNGImage;
impl crate::types::PNGImage {
    /// C: PNGImage::Width (user/user_objects.cc:60)
    #[allow(unused_variables, non_snake_case)]
    pub fn Width(self_ptr: *mut PNGImage) -> i32 {
        // MACHINE-PROVEN: Clang AST semantics
        unsafe { (*self_ptr).width_ }
    }
    

    /// C: PNGImage::Height (user/user_objects.cc:61)
    #[allow(unused_variables, non_snake_case)]
    pub fn Height(self_ptr: *mut PNGImage) -> i32 {
        // MACHINE-PROVEN: Clang AST semantics
        unsafe { (*self_ptr).height_ }
    }
    

    /// C: PNGImage::IsSRGB (user/user_objects.cc:62)
    #[allow(unused_variables, non_snake_case)]
    pub fn IsSRGB(self_ptr: *mut PNGImage) -> bool {
        // MACHINE-PROVEN: Clang AST semantics
        unsafe { (*self_ptr).is_srgb_ }
    }
    

}

pub use crate::types::mjCActuator;
impl crate::types::mjCActuator {
    /// C: mjCActuator::get_userdata (user/user_objects.h:1843)
    #[allow(unused_variables, non_snake_case)]
    pub fn get_userdata(self_ptr: *mut mjCActuator) -> *const () {
        // MACHINE-PROVEN: Clang AST semantics
        unsafe { (*self_ptr).userdata_ }
    }
    

}

pub use crate::types::mjCBase;
impl crate::types::mjCBase {
    /// C: mjCBase::GetRef (user/user_objects.h:403)
    #[allow(unused_variables, non_snake_case)]
    pub fn GetRef(self_ptr: *mut mjCBase) -> i32 {
        // MACHINE-PROVEN: Clang AST semantics
        unsafe { (*self_ptr).refcount }
    }
    

}

pub use crate::types::mjCBody;
impl crate::types::mjCBody {
    /// C: mjCBody::get_userdata (user/user_objects.h:557)
    #[allow(unused_variables, non_snake_case)]
    pub fn get_userdata(self_ptr: *mut mjCBody) -> *const () {
        // MACHINE-PROVEN: Clang AST semantics
        unsafe { (*self_ptr).userdata_ }
    }
    

    /// C: mjCBody::SetParent (user/user_objects.h:579)
    #[allow(unused_variables, non_snake_case)]
    pub fn SetParent(self_ptr: *mut mjCBody, _body: *mut mjCBody) {
        // MACHINE-PROVEN: Clang AST semantics
        unsafe { (*self_ptr).parent = _body; }
    }
    

    /// C: mjCBody::GetParent (user/user_objects.h:580)
    #[allow(unused_variables, non_snake_case)]
    pub fn GetParent(self_ptr: *mut mjCBody) -> *mut mjCBody {
        // MACHINE-PROVEN: Clang AST semantics
        unsafe { (*self_ptr).parent }
    }
    

}

pub use crate::types::mjCBoundingVolume;
impl crate::types::mjCBoundingVolume {
    /// C: mjCBoundingVolume::Contype (user/user_objects.h:122)
    #[allow(unused_variables, non_snake_case)]
    pub fn Contype(self_ptr: *mut mjCBoundingVolume) -> i32 {
        // MACHINE-PROVEN: Clang AST semantics
        unsafe { (*self_ptr).contype_ }
    }
    

    /// C: mjCBoundingVolume::Conaffinity (user/user_objects.h:123)
    #[allow(unused_variables, non_snake_case)]
    pub fn Conaffinity(self_ptr: *mut mjCBoundingVolume) -> i32 {
        // MACHINE-PROVEN: Clang AST semantics
        unsafe { (*self_ptr).conaffinity_ }
    }
    

    /// C: mjCBoundingVolume::SetContype (user/user_objects.h:131)
    #[allow(unused_variables, non_snake_case)]
    pub fn SetContype(self_ptr: *mut mjCBoundingVolume, val: i32) {
        // MACHINE-PROVEN: Clang AST semantics
        unsafe { (*self_ptr).contype_ = val; }
    }
    

    /// C: mjCBoundingVolume::SetConaffinity (user/user_objects.h:132)
    #[allow(unused_variables, non_snake_case)]
    pub fn SetConaffinity(self_ptr: *mut mjCBoundingVolume, val: i32) {
        // MACHINE-PROVEN: Clang AST semantics
        unsafe { (*self_ptr).conaffinity_ = val; }
    }
    

}

pub use crate::types::mjCBoundingVolumeHierarchy;
impl crate::types::mjCBoundingVolumeHierarchy {
    /// C: mjCBoundingVolumeHierarchy::Nbvh (user/user_objects.h:186)
    #[allow(unused_variables, non_snake_case)]
    pub fn Nbvh(self_ptr: *mut mjCBoundingVolumeHierarchy) -> i32 {
        // MACHINE-PROVEN: Clang AST semantics
        unsafe { (*self_ptr).nbvh_ }
    }
    

    /// C: mjCBoundingVolumeHierarchy::Bvh (user/user_objects.h:187)
    #[allow(unused_variables, non_snake_case)]
    pub fn Bvh(self_ptr: *mut mjCBoundingVolumeHierarchy) -> *const () {
        // MACHINE-PROVEN: Clang AST semantics
        unsafe { (*self_ptr).bvh_ }
    }
    

    /// C: mjCBoundingVolumeHierarchy::Child (user/user_objects.h:188)
    #[allow(unused_variables, non_snake_case)]
    pub fn Child(self_ptr: *mut mjCBoundingVolumeHierarchy) -> *const () {
        // MACHINE-PROVEN: Clang AST semantics
        unsafe { (*self_ptr).child_ }
    }
    

    /// C: mjCBoundingVolumeHierarchy::Level (user/user_objects.h:192)
    #[allow(unused_variables, non_snake_case)]
    pub fn Level(self_ptr: *mut mjCBoundingVolumeHierarchy) -> *const () {
        // MACHINE-PROVEN: Clang AST semantics
        unsafe { (*self_ptr).level_ }
    }
    

}

pub use crate::types::mjCCamera;
impl crate::types::mjCCamera {
    /// C: mjCCamera::get_userdata (user/user_objects.h:900)
    #[allow(unused_variables, non_snake_case)]
    pub fn get_userdata(self_ptr: *mut mjCCamera) -> *const () {
        // MACHINE-PROVEN: Clang AST semantics
        unsafe { (*self_ptr).userdata_ }
    }
    

    /// C: mjCCamera::SetParent (user/user_objects.h:902)
    #[allow(unused_variables, non_snake_case)]
    pub fn SetParent(self_ptr: *mut mjCCamera, _body: *mut mjCBody) {
        // MACHINE-PROVEN: Clang AST semantics
        unsafe { (*self_ptr).body = _body; }
    }
    

    /// C: mjCCamera::GetParent (user/user_objects.h:903)
    #[allow(unused_variables, non_snake_case)]
    pub fn GetParent(self_ptr: *mut mjCCamera) -> *mut mjCBody {
        // MACHINE-PROVEN: Clang AST semantics
        unsafe { (*self_ptr).body }
    }
    

}

pub use crate::types::mjCFlex;
impl crate::types::mjCFlex {
    /// C: mjCFlex::get_vertbody (user/user_objects.h:1039)
    #[allow(unused_variables, non_snake_case)]
    pub fn get_vertbody(self_ptr: *mut mjCFlex) -> *const () {
        // MACHINE-PROVEN: Clang AST semantics
        unsafe { (*self_ptr).vertbody_ }
    }
    

    /// C: mjCFlex::get_vert (user/user_objects.h:1040)
    #[allow(unused_variables, non_snake_case)]
    pub fn get_vert(self_ptr: *mut mjCFlex) -> *const () {
        // MACHINE-PROVEN: Clang AST semantics
        unsafe { (*self_ptr).vert_ }
    }
    

    /// C: mjCFlex::get_elemaabb (user/user_objects.h:1041)
    #[allow(unused_variables, non_snake_case)]
    pub fn get_elemaabb(self_ptr: *mut mjCFlex) -> *const () {
        // MACHINE-PROVEN: Clang AST semantics
        unsafe { (*self_ptr).elemaabb_ }
    }
    

    /// C: mjCFlex::get_elem (user/user_objects.h:1042)
    #[allow(unused_variables, non_snake_case)]
    pub fn get_elem(self_ptr: *mut mjCFlex) -> *const () {
        // MACHINE-PROVEN: Clang AST semantics
        unsafe { (*self_ptr).elem_ }
    }
    

    /// C: mjCFlex::get_texcoord (user/user_objects.h:1043)
    #[allow(unused_variables, non_snake_case)]
    pub fn get_texcoord(self_ptr: *mut mjCFlex) -> *const () {
        // MACHINE-PROVEN: Clang AST semantics
        unsafe { (*self_ptr).texcoord_ }
    }
    

    /// C: mjCFlex::get_elemtexcoord (user/user_objects.h:1044)
    #[allow(unused_variables, non_snake_case)]
    pub fn get_elemtexcoord(self_ptr: *mut mjCFlex) -> *const () {
        // MACHINE-PROVEN: Clang AST semantics
        unsafe { (*self_ptr).elemtexcoord_ }
    }
    

    /// C: mjCFlex::get_nodebody (user/user_objects.h:1045)
    #[allow(unused_variables, non_snake_case)]
    pub fn get_nodebody(self_ptr: *mut mjCFlex) -> *const () {
        // MACHINE-PROVEN: Clang AST semantics
        unsafe { (*self_ptr).nodebody_ }
    }
    

}

pub use crate::types::mjCFrame;
impl crate::types::mjCFrame {
    /// C: mjCFrame::SetParent (user/user_objects.h:656)
    #[allow(unused_variables, non_snake_case)]
    pub fn SetParent(self_ptr: *mut mjCFrame, _body: *mut mjCBody) {
        // MACHINE-PROVEN: Clang AST semantics
        unsafe { (*self_ptr).body = _body; }
    }
    

    /// C: mjCFrame::GetParent (user/user_objects.h:657)
    #[allow(unused_variables, non_snake_case)]
    pub fn GetParent(self_ptr: *mut mjCFrame) -> *mut mjCBody {
        // MACHINE-PROVEN: Clang AST semantics
        unsafe { (*self_ptr).body }
    }
    

}

pub use crate::types::mjCGeom;
impl crate::types::mjCGeom {
    /// C: mjCGeom::IsVisual (user/user_objects.h:785)
    #[allow(unused_variables, non_snake_case)]
    pub fn IsVisual(self_ptr: *mut mjCGeom) -> bool {
        // MACHINE-PROVEN: Clang AST semantics
        unsafe { (*self_ptr).visual_ }
    }
    

    /// C: mjCGeom::SetParent (user/user_objects.h:787)
    #[allow(unused_variables, non_snake_case)]
    pub fn SetParent(self_ptr: *mut mjCGeom, _body: *mut mjCBody) {
        // MACHINE-PROVEN: Clang AST semantics
        unsafe { (*self_ptr).body = _body; }
    }
    

    /// C: mjCGeom::GetParent (user/user_objects.h:788)
    #[allow(unused_variables, non_snake_case)]
    pub fn GetParent(self_ptr: *mut mjCGeom) -> *mut mjCBody {
        // MACHINE-PROVEN: Clang AST semantics
        unsafe { (*self_ptr).body }
    }
    

    /// C: mjCGeom::Type (user/user_objects.h:789)
    #[allow(unused_variables, non_snake_case)]
    pub fn Type(self_ptr: *mut mjCGeom) -> u32 {
        // MACHINE-PROVEN: Clang AST semantics
        unsafe { (*self_ptr).r#type }
    }
    

    /// C: mjCGeom::get_userdata (user/user_objects.h:797)
    #[allow(unused_variables, non_snake_case)]
    pub fn get_userdata(self_ptr: *mut mjCGeom) -> *const () {
        // MACHINE-PROVEN: Clang AST semantics
        unsafe { (*self_ptr).userdata_ }
    }
    

}

pub use crate::types::mjCHField;
impl crate::types::mjCHField {
    /// C: mjCHField::get_userdata (user/user_objects.h:1424)
    #[allow(unused_variables, non_snake_case)]
    pub fn get_userdata(self_ptr: *mut mjCHField) -> *const () {
        // MACHINE-PROVEN: Clang AST semantics
        unsafe { (*self_ptr).userdata_ }
    }
    

}

pub use crate::types::mjCJoint;
impl crate::types::mjCJoint {
    /// C: mjCJoint::SetParent (user/user_objects.h:707)
    #[allow(unused_variables, non_snake_case)]
    pub fn SetParent(self_ptr: *mut mjCJoint, _body: *mut mjCBody) {
        // MACHINE-PROVEN: Clang AST semantics
        unsafe { (*self_ptr).body = _body; }
    }
    

    /// C: mjCJoint::GetParent (user/user_objects.h:708)
    #[allow(unused_variables, non_snake_case)]
    pub fn GetParent(self_ptr: *mut mjCJoint) -> *mut mjCBody {
        // MACHINE-PROVEN: Clang AST semantics
        unsafe { (*self_ptr).body }
    }
    

    /// C: mjCJoint::get_userdata (user/user_objects.h:711)
    #[allow(unused_variables, non_snake_case)]
    pub fn get_userdata(self_ptr: *mut mjCJoint) -> *const () {
        // MACHINE-PROVEN: Clang AST semantics
        unsafe { (*self_ptr).userdata_ }
    }
    

    /// C: mjCJoint::get_range (user/user_objects.h:712)
    /// ⚠️ BITEXACT RULES:
    ///   1. Copy exact C accumulation order (no iter().sum())
    ///   2. No f64::mul_add() (FMA changes precision)
    ///   3. No algebraic simplification
    ///   4. No iter().sum()/product() (order undefined)
    #[allow(unused_variables, non_snake_case)]
    pub fn get_range(self_ptr: *mut mjCJoint) -> *const f64 {
        // MACHINE-PROVEN: Clang AST semantics
        unsafe { core::ptr::addr_of!((*self_ptr).range).cast::<f64>() }
    }
    

}

pub use crate::types::mjCLight;
impl crate::types::mjCLight {
    /// C: mjCLight::SetParent (user/user_objects.h:947)
    #[allow(unused_variables, non_snake_case)]
    pub fn SetParent(self_ptr: *mut mjCLight, _body: *mut mjCBody) {
        // MACHINE-PROVEN: Clang AST semantics
        unsafe { (*self_ptr).body = _body; }
    }
    

    /// C: mjCLight::GetParent (user/user_objects.h:948)
    #[allow(unused_variables, non_snake_case)]
    pub fn GetParent(self_ptr: *mut mjCLight) -> *mut mjCBody {
        // MACHINE-PROVEN: Clang AST semantics
        unsafe { (*self_ptr).body }
    }
    

}

pub use crate::types::mjCMesh;
impl crate::types::mjCMesh {
    /// C: mjCMesh::Refpos (user/user_objects.h:1169)
    /// ⚠️ BITEXACT RULES:
    ///   1. Copy exact C accumulation order (no iter().sum())
    ///   2. No f64::mul_add() (FMA changes precision)
    ///   3. No algebraic simplification
    ///   4. No iter().sum()/product() (order undefined)
    #[allow(unused_variables, non_snake_case)]
    pub fn Refpos(self_ptr: *mut mjCMesh) -> *const f64 {
        // MACHINE-PROVEN: Clang AST semantics
        unsafe { core::ptr::addr_of!((*self_ptr).refpos).cast::<f64>() }
    }
    

    /// C: mjCMesh::Refquat (user/user_objects.h:1170)
    /// ⚠️ BITEXACT RULES:
    ///   1. Copy exact C accumulation order (no iter().sum())
    ///   2. No f64::mul_add() (FMA changes precision)
    ///   3. No algebraic simplification
    ///   4. No iter().sum()/product() (order undefined)
    #[allow(unused_variables, non_snake_case)]
    pub fn Refquat(self_ptr: *mut mjCMesh) -> *const f64 {
        // MACHINE-PROVEN: Clang AST semantics
        unsafe { core::ptr::addr_of!((*self_ptr).refquat).cast::<f64>() }
    }
    

    /// C: mjCMesh::Scale (user/user_objects.h:1171)
    /// ⚠️ BITEXACT RULES:
    ///   1. Copy exact C accumulation order (no iter().sum())
    ///   2. No f64::mul_add() (FMA changes precision)
    ///   3. No algebraic simplification
    ///   4. No iter().sum()/product() (order undefined)
    #[allow(unused_variables, non_snake_case)]
    pub fn Scale(self_ptr: *mut mjCMesh) -> *const f64 {
        // MACHINE-PROVEN: Clang AST semantics
        unsafe { core::ptr::addr_of!((*self_ptr).scale).cast::<f64>() }
    }
    

    /// C: mjCMesh::UserVert (user/user_objects.h:1175)
    #[allow(unused_variables, non_snake_case)]
    pub fn UserVert(self_ptr: *mut mjCMesh) -> *const () {
        // MACHINE-PROVEN: Clang AST semantics
        unsafe { (*self_ptr).spec_vert_ }
    }
    

    /// C: mjCMesh::UserNormal (user/user_objects.h:1176)
    #[allow(unused_variables, non_snake_case)]
    pub fn UserNormal(self_ptr: *mut mjCMesh) -> *const () {
        // MACHINE-PROVEN: Clang AST semantics
        unsafe { (*self_ptr).spec_normal_ }
    }
    

    /// C: mjCMesh::Texcoord (user/user_objects.h:1177)
    #[allow(unused_variables, non_snake_case)]
    pub fn Texcoord(self_ptr: *mut mjCMesh) -> *const () {
        // MACHINE-PROVEN: Clang AST semantics
        unsafe { (*self_ptr).texcoord_ }
    }
    

    /// C: mjCMesh::FaceTexcoord (user/user_objects.h:1178)
    #[allow(unused_variables, non_snake_case)]
    pub fn FaceTexcoord(self_ptr: *mut mjCMesh) -> *const () {
        // MACHINE-PROVEN: Clang AST semantics
        unsafe { (*self_ptr).facetexcoord_ }
    }
    

    /// C: mjCMesh::UserTexcoord (user/user_objects.h:1179)
    #[allow(unused_variables, non_snake_case)]
    pub fn UserTexcoord(self_ptr: *mut mjCMesh) -> *const () {
        // MACHINE-PROVEN: Clang AST semantics
        unsafe { (*self_ptr).spec_texcoord_ }
    }
    

    /// C: mjCMesh::Face (user/user_objects.h:1180)
    #[allow(unused_variables, non_snake_case)]
    pub fn Face(self_ptr: *mut mjCMesh) -> *const () {
        // MACHINE-PROVEN: Clang AST semantics
        unsafe { (*self_ptr).face_ }
    }
    

    /// C: mjCMesh::UserFace (user/user_objects.h:1181)
    #[allow(unused_variables, non_snake_case)]
    pub fn UserFace(self_ptr: *mut mjCMesh) -> *const () {
        // MACHINE-PROVEN: Clang AST semantics
        unsafe { (*self_ptr).spec_face_ }
    }
    

    /// C: mjCMesh::SetNeedHull (user/user_objects.h:1186)
    #[allow(unused_variables, non_snake_case)]
    pub fn SetNeedHull(self_ptr: *mut mjCMesh, needhull: bool) {
        // MACHINE-PROVEN: Clang AST semantics
        unsafe { (*self_ptr).needhull_ = needhull; }
    }
    

    /// C: mjCMesh::aamm (user/user_objects.h:1189)
    /// ⚠️ BITEXACT RULES:
    ///   1. Copy exact C accumulation order (no iter().sum())
    ///   2. No f64::mul_add() (FMA changes precision)
    ///   3. No algebraic simplification
    ///   4. No iter().sum()/product() (order undefined)
    #[allow(unused_variables, non_snake_case)]
    pub fn aamm(self_ptr: *mut mjCMesh) -> *const f64 {
        // MACHINE-PROVEN: Clang AST semantics
        unsafe { core::ptr::addr_of!((*self_ptr).aamm_).cast::<f64>() }
    }
    

    /// C: mjCMesh::szgraph (user/user_objects.h:1213)
    #[allow(unused_variables, non_snake_case)]
    pub fn szgraph(self_ptr: *mut mjCMesh) -> i32 {
        // MACHINE-PROVEN: Clang AST semantics
        unsafe { (*self_ptr).szgraph_ }
    }
    

    /// C: mjCMesh::GetPosPtr (user/user_objects.h:1223)
    /// ⚠️ BITEXACT RULES:
    ///   1. Copy exact C accumulation order (no iter().sum())
    ///   2. No f64::mul_add() (FMA changes precision)
    ///   3. No algebraic simplification
    ///   4. No iter().sum()/product() (order undefined)
    #[allow(unused_variables, non_snake_case)]
    pub fn GetPosPtr(self_ptr: *mut mjCMesh) -> *mut f64 {
        // MACHINE-PROVEN: Clang AST semantics
        unsafe { core::ptr::addr_of_mut!((*self_ptr).pos_).cast::<f64>() }
    }
    

    /// C: mjCMesh::GetQuatPtr (user/user_objects.h:1224)
    /// ⚠️ BITEXACT RULES:
    ///   1. Copy exact C accumulation order (no iter().sum())
    ///   2. No f64::mul_add() (FMA changes precision)
    ///   3. No algebraic simplification
    ///   4. No iter().sum()/product() (order undefined)
    #[allow(unused_variables, non_snake_case)]
    pub fn GetQuatPtr(self_ptr: *mut mjCMesh) -> *mut f64 {
        // MACHINE-PROVEN: Clang AST semantics
        unsafe { core::ptr::addr_of_mut!((*self_ptr).quat_).cast::<f64>() }
    }
    

    /// C: mjCMesh::GetInertiaBoxPtr (user/user_objects.h:1225)
    /// ⚠️ BITEXACT RULES:
    ///   1. Copy exact C accumulation order (no iter().sum())
    ///   2. No f64::mul_add() (FMA changes precision)
    ///   3. No algebraic simplification
    ///   4. No iter().sum()/product() (order undefined)
    #[allow(unused_variables, non_snake_case)]
    pub fn GetInertiaBoxPtr(self_ptr: *mut mjCMesh) -> *mut f64 {
        // MACHINE-PROVEN: Clang AST semantics
        unsafe { core::ptr::addr_of_mut!((*self_ptr).boxsz_).cast::<f64>() }
    }
    

    /// C: mjCMesh::IsVisual (user/user_objects.h:1230)
    #[allow(unused_variables, non_snake_case)]
    pub fn IsVisual(self_ptr: *mut mjCMesh) -> bool {
        // MACHINE-PROVEN: Clang AST semantics
        unsafe { (*self_ptr).visual_ }
    }
    

}

pub use crate::types::mjCOctree;
impl crate::types::mjCOctree {
    /// C: mjCOctree::NumNodes (user/user_objects.h:287)
    #[allow(unused_variables, non_snake_case)]
    pub fn NumNodes(self_ptr: *mut mjCOctree) -> i32 {
        // MACHINE-PROVEN: Clang AST semantics
        unsafe { (*self_ptr).nnode_ }
    }
    

    /// C: mjCOctree::NumVerts (user/user_objects.h:288)
    #[allow(unused_variables, non_snake_case)]
    pub fn NumVerts(self_ptr: *mut mjCOctree) -> i32 {
        // MACHINE-PROVEN: Clang AST semantics
        unsafe { (*self_ptr).nvert_ }
    }
    

    /// C: mjCOctree::SetMaxDepth (user/user_objects.h:313)
    #[allow(unused_variables, non_snake_case)]
    pub fn SetMaxDepth(self_ptr: *mut mjCOctree, depth: i32) {
        // MACHINE-PROVEN: Clang AST semantics
        unsafe { (*self_ptr).max_depth_ = depth; }
    }
    

    /// C: mjCOctree::MaxDepth (user/user_objects.h:314)
    #[allow(unused_variables, non_snake_case)]
    pub fn MaxDepth(self_ptr: *mut mjCOctree) -> i32 {
        // MACHINE-PROVEN: Clang AST semantics
        unsafe { (*self_ptr).max_depth_ }
    }
    

    /// C: mjCOctree::SetSmoothingIterations (user/user_objects.h:317)
    #[allow(unused_variables, non_snake_case)]
    pub fn SetSmoothingIterations(self_ptr: *mut mjCOctree, iterations: i32) {
        // MACHINE-PROVEN: Clang AST semantics
        unsafe { (*self_ptr).smoothing_iterations_ = iterations; }
    }
    

    /// C: mjCOctree::SmoothingIterations (user/user_objects.h:318)
    #[allow(unused_variables, non_snake_case)]
    pub fn SmoothingIterations(self_ptr: *mut mjCOctree) -> i32 {
        // MACHINE-PROVEN: Clang AST semantics
        unsafe { (*self_ptr).smoothing_iterations_ }
    }
    

}

pub use crate::types::mjCSensor;
impl crate::types::mjCSensor {
    /// C: mjCSensor::get_userdata (user/user_objects.h:1901)
    #[allow(unused_variables, non_snake_case)]
    pub fn get_userdata(self_ptr: *mut mjCSensor) -> *const () {
        // MACHINE-PROVEN: Clang AST semantics
        unsafe { (*self_ptr).userdata_ }
    }
    

}

pub use crate::types::mjCSite;
impl crate::types::mjCSite {
    /// C: mjCSite::Body (user/user_objects.h:849)
    #[allow(unused_variables, non_snake_case)]
    pub fn Body(self_ptr: *mut mjCSite) -> *mut mjCBody {
        // MACHINE-PROVEN: Clang AST semantics
        unsafe { (*self_ptr).body }
    }
    

    /// C: mjCSite::SetParent (user/user_objects.h:850)
    #[allow(unused_variables, non_snake_case)]
    pub fn SetParent(self_ptr: *mut mjCSite, _body: *mut mjCBody) {
        // MACHINE-PROVEN: Clang AST semantics
        unsafe { (*self_ptr).body = _body; }
    }
    

    /// C: mjCSite::GetParent (user/user_objects.h:851)
    #[allow(unused_variables, non_snake_case)]
    pub fn GetParent(self_ptr: *mut mjCSite) -> *mut mjCBody {
        // MACHINE-PROVEN: Clang AST semantics
        unsafe { (*self_ptr).body }
    }
    

    /// C: mjCSite::get_userdata (user/user_objects.h:857)
    #[allow(unused_variables, non_snake_case)]
    pub fn get_userdata(self_ptr: *mut mjCSite) -> *const () {
        // MACHINE-PROVEN: Clang AST semantics
        unsafe { (*self_ptr).userdata_ }
    }
    

}

pub use crate::types::mjCSkin;
impl crate::types::mjCSkin {
    /// C: mjCSkin::get_vert (user/user_objects.h:1366)
    #[allow(unused_variables, non_snake_case)]
    pub fn get_vert(self_ptr: *mut mjCSkin) -> *const () {
        // MACHINE-PROVEN: Clang AST semantics
        unsafe { (*self_ptr).vert_ }
    }
    

    /// C: mjCSkin::get_texcoord (user/user_objects.h:1367)
    #[allow(unused_variables, non_snake_case)]
    pub fn get_texcoord(self_ptr: *mut mjCSkin) -> *const () {
        // MACHINE-PROVEN: Clang AST semantics
        unsafe { (*self_ptr).texcoord_ }
    }
    

    /// C: mjCSkin::get_face (user/user_objects.h:1368)
    #[allow(unused_variables, non_snake_case)]
    pub fn get_face(self_ptr: *mut mjCSkin) -> *const () {
        // MACHINE-PROVEN: Clang AST semantics
        unsafe { (*self_ptr).face_ }
    }
    

    /// C: mjCSkin::get_bodyname (user/user_objects.h:1369)
    #[allow(unused_variables, non_snake_case)]
    pub fn get_bodyname(self_ptr: *mut mjCSkin) -> *const () {
        // MACHINE-PROVEN: Clang AST semantics
        unsafe { (*self_ptr).bodyname_ }
    }
    

    /// C: mjCSkin::get_bindpos (user/user_objects.h:1370)
    #[allow(unused_variables, non_snake_case)]
    pub fn get_bindpos(self_ptr: *mut mjCSkin) -> *const () {
        // MACHINE-PROVEN: Clang AST semantics
        unsafe { (*self_ptr).bindpos_ }
    }
    

    /// C: mjCSkin::get_bindquat (user/user_objects.h:1371)
    #[allow(unused_variables, non_snake_case)]
    pub fn get_bindquat(self_ptr: *mut mjCSkin) -> *const () {
        // MACHINE-PROVEN: Clang AST semantics
        unsafe { (*self_ptr).bindquat_ }
    }
    

    /// C: mjCSkin::get_vertid (user/user_objects.h:1372)
    #[allow(unused_variables, non_snake_case)]
    pub fn get_vertid(self_ptr: *mut mjCSkin) -> *const () {
        // MACHINE-PROVEN: Clang AST semantics
        unsafe { (*self_ptr).vertid_ }
    }
    

    /// C: mjCSkin::get_vertweight (user/user_objects.h:1373)
    #[allow(unused_variables, non_snake_case)]
    pub fn get_vertweight(self_ptr: *mut mjCSkin) -> *const () {
        // MACHINE-PROVEN: Clang AST semantics
        unsafe { (*self_ptr).vertweight_ }
    }
    

}

pub use crate::types::mjCTendon;
impl crate::types::mjCTendon {
    /// C: mjCTendon::get_userdata (user/user_objects.h:1713)
    #[allow(unused_variables, non_snake_case)]
    pub fn get_userdata(self_ptr: *mut mjCTendon) -> *const () {
        // MACHINE-PROVEN: Clang AST semantics
        unsafe { (*self_ptr).userdata_ }
    }
    

}

