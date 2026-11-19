//! Port of: user/user_objects.h
//! IR hash: ebd8b59db8c47e3b
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mjCSite::GetParent (user/user_objects.h:863)
/// # Safety
/// Raw pointer arguments and the receiver, when present, must be non-null and designate live memory for each generated access, with the alignment required by that operation. Any byte read must be initialized; any location written must be writable; pointer offsets must stay within the originating allocation or one past it. No other thread may perform an unsynchronized conflicting access to the same memory during this operation.
#[allow(unused_variables, non_snake_case)]
pub unsafe fn mj_c_site_get_parent(self_ptr: *mut mjCSite) -> *mut mjCBody {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).body }
}


/// C: mjCMesh::SmoothNormal (user/user_objects.h:1194)
/// # Safety
/// Raw pointer arguments and the receiver, when present, must be non-null and designate live memory for each generated access, with the alignment required by that operation. Any byte read must be initialized; any location written must be writable; pointer offsets must stay within the originating allocation or one past it. No other thread may perform an unsynchronized conflicting access to the same memory during this operation.
#[allow(unused_variables, non_snake_case)]
pub unsafe fn mj_c_mesh_smooth_normal(self_ptr: *mut mjCMesh) -> bool {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).smoothnormal }
}

/// C: mjCSite::SetParent (user/user_objects.h:862)
/// # Safety
/// Raw pointer arguments and the receiver, when present, must be non-null and designate live memory for each generated access, with the alignment required by that operation. Any byte read must be initialized; any location written must be writable; pointer offsets must stay within the originating allocation or one past it. No other thread may perform an unsynchronized conflicting access to the same memory during this operation.
#[allow(unused_variables, non_snake_case)]
pub unsafe fn mj_c_site_set_parent(self_ptr: *mut mjCSite, _body: *mut mjCBody) {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).body = _body; }
}

/// C: mjCSensor::get_obj (user/user_objects.h:1924)
/// # Safety
/// Raw pointer arguments and the receiver, when present, must be non-null and designate live memory for each generated access, with the alignment required by that operation. Any byte read must be initialized; any location written must be writable; pointer offsets must stay within the originating allocation or one past it. No other thread may perform an unsynchronized conflicting access to the same memory during this operation.
#[allow(unused_variables, non_snake_case)]
pub unsafe fn mj_c_sensor_get_obj(self_ptr: *mut mjCSensor) -> *const mjCBase {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { ((*self_ptr).obj) as *const mjCBase }
}

/// C: mjCPair::GetSignature (user/user_objects.h:1594)
/// # Safety
/// Raw pointer arguments and the receiver, when present, must be non-null and designate live memory for each generated access, with the alignment required by that operation. Any byte read must be initialized; any location written must be writable; pointer offsets must stay within the originating allocation or one past it. No other thread may perform an unsynchronized conflicting access to the same memory during this operation.
#[allow(unused_variables, non_snake_case)]
pub unsafe fn mj_c_pair_get_signature(self_ptr: *mut mjCPair) -> i32 {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).signature }
}

/// C: mjCDef::Light (user/user_objects.h:2102)
/// # Safety
/// Raw pointer arguments and the receiver, when present, must be non-null and designate live memory for each generated access, with the alignment required by that operation. Any byte read must be initialized; any location written must be writable; pointer offsets must stay within the originating allocation or one past it. No other thread may perform an unsynchronized conflicting access to the same memory during this operation.
#[allow(unused_variables, non_snake_case)]
pub unsafe fn mj_c_def_light(self_ptr: *mut mjCDef) -> *mut mjCLight {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { core::ptr::addr_of_mut!((*self_ptr).light_) }
}

/// C: mjCBodyPair::GetSignature (user/user_objects.h:1639)
/// # Safety
/// Raw pointer arguments and the receiver, when present, must be non-null and designate live memory for each generated access, with the alignment required by that operation. Any byte read must be initialized; any location written must be writable; pointer offsets must stay within the originating allocation or one past it. No other thread may perform an unsynchronized conflicting access to the same memory during this operation.
#[allow(unused_variables, non_snake_case)]
pub unsafe fn mj_c_body_pair_get_signature(self_ptr: *mut mjCBodyPair) -> i32 {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).signature }
}

/// C: mjCSkin::get_bindquat (user/user_objects.h:1387)
/// # Safety
/// Raw pointer arguments and the receiver, when present, must be non-null and designate live memory for each generated access, with the alignment required by that operation. Any byte read must be initialized; any location written must be writable; pointer offsets must stay within the originating allocation or one past it. No other thread may perform an unsynchronized conflicting access to the same memory during this operation.
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub unsafe fn mj_c_skin_get_bindquat(self_ptr: *mut mjCSkin) -> *const std__vector_float {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { core::ptr::addr_of!((*self_ptr).bindquat_) }
}

/// C: mjCSensor::get_objname (user/user_objects.h:1921)
/// # Safety
/// Raw pointer arguments and the receiver, when present, must be non-null and designate live memory for each generated access, with the alignment required by that operation. Any byte read must be initialized; any location written must be writable; pointer offsets must stay within the originating allocation or one past it. No other thread may perform an unsynchronized conflicting access to the same memory during this operation.
#[allow(unused_variables, non_snake_case)]
pub unsafe fn mj_c_sensor_get_objname(self_ptr: *mut mjCSensor) -> *const std__string {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { core::ptr::addr_of!((*self_ptr).spec_objname_) }
}

/// C: mjCSkin::get_face (user/user_objects.h:1384)
/// # Safety
/// Raw pointer arguments and the receiver, when present, must be non-null and designate live memory for each generated access, with the alignment required by that operation. Any byte read must be initialized; any location written must be writable; pointer offsets must stay within the originating allocation or one past it. No other thread may perform an unsynchronized conflicting access to the same memory during this operation.
#[allow(unused_variables, non_snake_case)]
pub unsafe fn mj_c_skin_get_face(self_ptr: *mut mjCSkin) -> *const std__vector_int {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { core::ptr::addr_of!((*self_ptr).face_) }
}

/// C: mjCMesh::octree (user/user_objects.h:1237)
/// # Safety
/// Raw pointer arguments and the receiver, when present, must be non-null and designate live memory for each generated access, with the alignment required by that operation. Any byte read must be initialized; any location written must be writable; pointer offsets must stay within the originating allocation or one past it. No other thread may perform an unsynchronized conflicting access to the same memory during this operation.
#[allow(unused_variables, non_snake_case)]
pub unsafe fn mj_c_mesh_octree(self_ptr: *mut mjCMesh) -> *const mjCOctree {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { core::ptr::addr_of!((*self_ptr).octree_) }
}

/// C: mjCFlex::get_vert (user/user_objects.h:1059)
/// # Safety
/// Raw pointer arguments and the receiver, when present, must be non-null and designate live memory for each generated access, with the alignment required by that operation. Any byte read must be initialized; any location written must be writable; pointer offsets must stay within the originating allocation or one past it. No other thread may perform an unsynchronized conflicting access to the same memory during this operation.
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub unsafe fn mj_c_flex_get_vert(self_ptr: *mut mjCFlex) -> *const std__vector_double {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { core::ptr::addr_of!((*self_ptr).vert_) }
}

/// C: mjCBase::UserValue::UserValue (user/user_objects.h:444)
/// # Safety
/// Raw pointer arguments and the receiver, when present, must be non-null and designate live memory for each generated access, with the alignment required by that operation. Any byte read must be initialized; any location written must be writable; pointer offsets must stay within the originating allocation or one past it. No other thread may perform an unsynchronized conflicting access to the same memory during this operation.
#[allow(unused_variables, non_snake_case)]
pub unsafe fn mj_c_base_user_value_user_value_6b313e7c(self_ptr: *mut mjCBase__UserValue) {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { core::ptr::write(core::ptr::addr_of_mut!((*self_ptr).value), ::core::ptr::null()); core::ptr::write(core::ptr::addr_of_mut!((*self_ptr).cleanup), ::core::option::Option::None); }
}

/// C: mjCBase::UserValue::UserValue (user/user_objects.h:444)
/// # Safety
/// Raw pointer arguments and the receiver, when present, must be non-null and designate live memory for each generated access, with the alignment required by that operation. Any byte read must be initialized; any location written must be writable; pointer offsets must stay within the originating allocation or one past it. No other thread may perform an unsynchronized conflicting access to the same memory during this operation.
#[allow(unused_variables, non_snake_case)]
pub unsafe fn mj_c_base_user_value_user_value_87331a34(self_ptr: *mut mjCBase__UserValue) {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { core::ptr::write(core::ptr::addr_of_mut!((*self_ptr).value), ::core::ptr::null()); core::ptr::write(core::ptr::addr_of_mut!((*self_ptr).cleanup), ::core::option::Option::None); }
}

/// C: mjCDef::Camera (user/user_objects.h:2101)
/// # Safety
/// Raw pointer arguments and the receiver, when present, must be non-null and designate live memory for each generated access, with the alignment required by that operation. Any byte read must be initialized; any location written must be writable; pointer offsets must stay within the originating allocation or one past it. No other thread may perform an unsynchronized conflicting access to the same memory during this operation.
#[allow(unused_variables, non_snake_case)]
pub unsafe fn mj_c_def_camera(self_ptr: *mut mjCDef) -> *mut mjCCamera {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { core::ptr::addr_of_mut!((*self_ptr).camera_) }
}

/// C: mjCJoint::GetParent (user/user_objects.h:719)
/// # Safety
/// Raw pointer arguments and the receiver, when present, must be non-null and designate live memory for each generated access, with the alignment required by that operation. Any byte read must be initialized; any location written must be writable; pointer offsets must stay within the originating allocation or one past it. No other thread may perform an unsynchronized conflicting access to the same memory during this operation.
#[allow(unused_variables, non_snake_case)]
pub unsafe fn mj_c_joint_get_parent(self_ptr: *mut mjCJoint) -> *mut mjCBody {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { ((*self_ptr).body) as *mut mjCBody }
}

/// C: mjCMesh::SetNeedHull (user/user_objects.h:1208)
/// # Safety
/// Raw pointer arguments and the receiver, when present, must be non-null and designate live memory for each generated access, with the alignment required by that operation. Any byte read must be initialized; any location written must be writable; pointer offsets must stay within the originating allocation or one past it. No other thread may perform an unsynchronized conflicting access to the same memory during this operation.
#[allow(unused_variables, non_snake_case)]
pub unsafe fn mj_c_mesh_set_need_hull(self_ptr: *mut mjCMesh, needhull: bool) {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).needhull_ = needhull; }
}

/// C: mjCFlex::get_nodebody (user/user_objects.h:1064)
/// # Safety
/// Raw pointer arguments and the receiver, when present, must be non-null and designate live memory for each generated access, with the alignment required by that operation. Any byte read must be initialized; any location written must be writable; pointer offsets must stay within the originating allocation or one past it. No other thread may perform an unsynchronized conflicting access to the same memory during this operation.
#[allow(unused_variables, non_snake_case)]
pub unsafe fn mj_c_flex_get_nodebody(self_ptr: *mut mjCFlex) -> *const std__vector_std__string {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { core::ptr::addr_of!((*self_ptr).nodebody_) }
}

/// C: mjCMesh::Face (user/user_objects.h:1202)
/// # Safety
/// Raw pointer arguments and the receiver, when present, must be non-null and designate live memory for each generated access, with the alignment required by that operation. Any byte read must be initialized; any location written must be writable; pointer offsets must stay within the originating allocation or one past it. No other thread may perform an unsynchronized conflicting access to the same memory during this operation.
#[allow(unused_variables, non_snake_case)]
pub unsafe fn mj_c_mesh_face(self_ptr: *mut mjCMesh) -> *const std__vector_int {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { core::ptr::addr_of!((*self_ptr).face_) }
}

/// C: mjCDef::Tendon (user/user_objects.h:2108)
/// # Safety
/// Raw pointer arguments and the receiver, when present, must be non-null and designate live memory for each generated access, with the alignment required by that operation. Any byte read must be initialized; any location written must be writable; pointer offsets must stay within the originating allocation or one past it. No other thread may perform an unsynchronized conflicting access to the same memory during this operation.
#[allow(unused_variables, non_snake_case)]
pub unsafe fn mj_c_def_tendon(self_ptr: *mut mjCDef) -> *mut mjCTendon {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { core::ptr::addr_of_mut!((*self_ptr).tendon_) }
}

/// C: mjCJoint::get_range (user/user_objects.h:723)
/// # Safety
/// Raw pointer arguments and the receiver, when present, must be non-null and designate live memory for each generated access, with the alignment required by that operation. Any byte read must be initialized; any location written must be writable; pointer offsets must stay within the originating allocation or one past it. No other thread may perform an unsynchronized conflicting access to the same memory during this operation.
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub unsafe fn mj_c_joint_get_range(self_ptr: *mut mjCJoint) -> *const f64 {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { core::ptr::addr_of!((*self_ptr).range).cast::<f64>() }
}

#[allow(unused_variables, non_snake_case)]
/// # Safety
/// Pointer arguments read by this function must designate live, initialized, aligned storage.
/// Storage written by this function must be aligned, writable, and free of unsynchronized conflicting accesses.
/// The derived pointer must stay in its originating live array or one past its end.
pub unsafe fn mj_c_frame_is_ancestor(self_ptr: *mut mjCFrame, child: *const mjCFrame) -> bool {
    let mut cursor = child;
    while !cursor.is_null() {
        if core::ptr::eq(cursor, self_ptr as *const mjCFrame) {
            return true;
        }
        // Compiler-proven mjCBase::frame storage in the current target IR.
        cursor = unsafe {
            core::ptr::read_unaligned(cursor.cast::<u8>().add(152).cast::<*mut mjCFrame>()) as *const mjCFrame
        };
    }
    false
}

/// C: mjCBody::GetParent (user/user_objects.h:591)
/// # Safety
/// Raw pointer arguments and the receiver, when present, must be non-null and designate live memory for each generated access, with the alignment required by that operation. Any byte read must be initialized; any location written must be writable; pointer offsets must stay within the originating allocation or one past it. No other thread may perform an unsynchronized conflicting access to the same memory during this operation.
#[allow(unused_variables, non_snake_case)]
pub unsafe fn mj_c_body_get_parent(self_ptr: *mut mjCBody) -> *mut mjCBody {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { ((*self_ptr).parent) as *mut mjCBody }
}

/// C: mjCBody::SetParent (user/user_objects.h:590)
/// # Safety
/// Raw pointer arguments and the receiver, when present, must be non-null and designate live memory for each generated access, with the alignment required by that operation. Any byte read must be initialized; any location written must be writable; pointer offsets must stay within the originating allocation or one past it. No other thread may perform an unsynchronized conflicting access to the same memory during this operation.
#[allow(unused_variables, non_snake_case)]
pub unsafe fn mj_c_body_set_parent(self_ptr: *mut mjCBody, _body: *mut mjCBody) {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).parent = _body; }
}

/// C: mjCWrap::Type (user/user_objects.h:1766)
/// # Safety
/// Raw pointer arguments and the receiver, when present, must be non-null and designate live memory for each generated access, with the alignment required by that operation. Any byte read must be initialized; any location written must be writable; pointer offsets must stay within the originating allocation or one past it. No other thread may perform an unsynchronized conflicting access to the same memory during this operation.
#[allow(unused_variables, non_snake_case)]
pub unsafe fn mj_c_wrap_type(self_ptr: *mut mjCWrap) -> u32 {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).spec.r#type }
}

/// C: mjCMesh::mutable_octree (user/user_objects.h:1238)
/// # Safety
/// Raw pointer arguments and the receiver, when present, must be non-null and designate live memory for each generated access, with the alignment required by that operation. Any byte read must be initialized; any location written must be writable; pointer offsets must stay within the originating allocation or one past it. No other thread may perform an unsynchronized conflicting access to the same memory during this operation.
#[allow(unused_variables, non_snake_case)]
pub unsafe fn mj_c_mesh_mutable_octree(self_ptr: *mut mjCMesh) -> *mut mjCOctree {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { core::ptr::addr_of_mut!((*self_ptr).octree_) }
}

/// C: mjCFrame::SetParent (user/user_objects.h:668)
/// # Safety
/// Raw pointer arguments and the receiver, when present, must be non-null and designate live memory for each generated access, with the alignment required by that operation. Any byte read must be initialized; any location written must be writable; pointer offsets must stay within the originating allocation or one past it. No other thread may perform an unsynchronized conflicting access to the same memory during this operation.
#[allow(unused_variables, non_snake_case)]
pub unsafe fn __c2rust_mj_c_frame_set_parent_0209f986822e6764e35e86ae523cf16334707bbfa80c5b8db4d6fc9b2a1171c2(self_ptr: *mut mjCFrame, _body: *mut mjCBody) {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).body = _body; }
}

/// C: mjCSkin::get_vertweight (user/user_objects.h:1389)
/// # Safety
/// Raw pointer arguments and the receiver, when present, must be non-null and designate live memory for each generated access, with the alignment required by that operation. Any byte read must be initialized; any location written must be writable; pointer offsets must stay within the originating allocation or one past it. No other thread may perform an unsynchronized conflicting access to the same memory during this operation.
#[allow(unused_variables, non_snake_case)]
pub unsafe fn __c2rust_mj_c_skin_get_vertweight_6485a88149c0ec2a47290cb727349e211061534e413f2659b1ddb3c033fe9502(self_ptr: *mut mjCSkin) -> *const std__vector_std__vector_float {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { core::ptr::addr_of!((*self_ptr).vertweight_) }
}

/// C: mjCPair::GetSignature (user/user_objects.h:1594)
/// # Safety
/// Raw pointer arguments and the receiver, when present, must be non-null and designate live memory for each generated access, with the alignment required by that operation. Any byte read must be initialized; any location written must be writable; pointer offsets must stay within the originating allocation or one past it. No other thread may perform an unsynchronized conflicting access to the same memory during this operation.
#[allow(unused_variables, non_snake_case)]
pub unsafe fn __c2rust_mj_c_pair_get_signature_78b831a250274ba68de7897e1007d1e6a0e9886963a37876c7c544deaa0cc6fa(self_ptr: *mut mjCPair) -> i32 {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).signature }
}

/// C: mjCJoint::get_range (user/user_objects.h:723)
/// # Safety
/// Raw pointer arguments and the receiver, when present, must be non-null and designate live memory for each generated access, with the alignment required by that operation. Any byte read must be initialized; any location written must be writable; pointer offsets must stay within the originating allocation or one past it. No other thread may perform an unsynchronized conflicting access to the same memory during this operation.
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub unsafe fn __c2rust_mj_c_joint_get_range_322485281a705ffd490bf6ffcbc54f82f2c2b24d7a101c1a69a0fb1758c63372(self_ptr: *mut mjCJoint) -> *const f64 {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { core::ptr::addr_of!((*self_ptr).range).cast::<f64>() }
}

/// C: mjCMesh::mutable_octree (user/user_objects.h:1238)
/// # Safety
/// Raw pointer arguments and the receiver, when present, must be non-null and designate live memory for each generated access, with the alignment required by that operation. Any byte read must be initialized; any location written must be writable; pointer offsets must stay within the originating allocation or one past it. No other thread may perform an unsynchronized conflicting access to the same memory during this operation.
#[allow(unused_variables, non_snake_case)]
pub unsafe fn __c2rust_mj_c_mesh_mutable_octree_b4985a9ccc531ec155b6ff9512ef44314c36a37490bc3486a0d5aa61bebea161(self_ptr: *mut mjCMesh) -> *mut mjCOctree {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { core::ptr::addr_of_mut!((*self_ptr).octree_) }
}

/// C: mjCTendon::del_material (user/user_objects.h:1713)
/// # Safety
/// Raw pointer arguments and the receiver, when present, must be non-null and designate live memory for each generated access, with the alignment required by that operation. Any byte read must be initialized; any location written must be writable; pointer offsets must stay within the originating allocation or one past it. No other thread may perform an unsynchronized conflicting access to the same memory during this operation.
#[allow(unused_variables, non_snake_case)]
pub unsafe fn __c2rust_mj_c_tendon_del_material_b52ea103de51e31d34af678e24e97ebbfdcc90d38b41a406f4f8bd11ce8daefb(self_ptr: *mut mjCTendon) {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe {
 extern "C" { fn c2rust_std_sequence_clear_3dea22fa08fbcc13870d4a29fcc48faa48ee7858bb2ec82fa493826dfa532a2c(sequence: *mut core::ffi::c_void); }
 c2rust_std_sequence_clear_3dea22fa08fbcc13870d4a29fcc48faa48ee7858bb2ec82fa493826dfa532a2c((self_ptr as *mut u8).add(232) as *mut core::ffi::c_void);
}
}

/// C: mjCBodyPair::get_bodyname1 (user/user_objects.h)
/// # Safety
/// Raw pointer arguments and the receiver, when present, must be non-null and designate live memory for each generated access, with the alignment required by that operation. Any byte read must be initialized; any location written must be writable; pointer offsets must stay within the originating allocation or one past it. No other thread may perform an unsynchronized conflicting access to the same memory during this operation.
pub unsafe fn __c2rust_mj_c_body_pair_get_bodyname1_c02f32c2dc50de4250ad1b8548b354b1ea3ca4fca91c006827db339a70120a0f(c2rust_result_out: *mut std__string, self_ptr: *mut mjCBodyPair) -> () {
    extern "C" { fn c2rust_std_string_copy_77903c6aa15af5bc136f5cc42a00618cfcce78616b72abc2bb6f1e34fd95fbce(destination: *mut core::ffi::c_void, source: *const core::ffi::c_void); }
    let source_ptr = (self_ptr as *const u8).add(240).cast::<core::ffi::c_void>();
    c2rust_std_string_copy_77903c6aa15af5bc136f5cc42a00618cfcce78616b72abc2bb6f1e34fd95fbce(c2rust_result_out.cast::<core::ffi::c_void>(), source_ptr);
}

/// C: mjCBoundingVolume::mjCBoundingVolume(const int* id, int contype, int conaffinity, const double pos[3], const double quat[4], const double aabb[6]) (user/user_objects.h:110) [C1 complete-object constructor]
/// # Safety
/// Raw pointer arguments and the receiver, when present, must be non-null and designate live memory for each generated access, with the alignment required by that operation. Any byte read must be initialized; any location written must be writable; pointer offsets must stay within the originating allocation or one past it. No other thread may perform an unsynchronized conflicting access to the same memory during this operation.
#[allow(unused_variables, non_snake_case)]
/// # Safety
/// Pointer arguments read by this function must designate live, initialized, aligned storage.
/// Storage written by this function must be aligned, writable, and free of unsynchronized conflicting accesses.
/// The derived pointer must stay in its originating live array or one past its end.
pub unsafe fn __c2rust_mj_c_bounding_volume_mj_c_bounding_volume_e150b31568c6a55f8202faa65eef46ba63b6c0269592293b00459c2ce1085e24(self_ptr: *mut mjCBoundingVolume, id: *const i32, contype: i32, conaffinity: i32, pos: *const f64, quat: *const f64, aabb: *const f64) -> () {
    unsafe {
        (*self_ptr).contype_ = contype;
        (*self_ptr).conaffinity_ = conaffinity;
        (*self_ptr).id_ = id;
        core::ptr::copy_nonoverlapping(aabb, core::ptr::addr_of_mut!((*self_ptr).aabb_) as *mut f64, 6);
        core::ptr::copy_nonoverlapping(pos, core::ptr::addr_of_mut!((*self_ptr).pos_) as *mut f64, 3);
        let quat_set = !quat.is_null();
        (*self_ptr).quat_set_ = quat_set;
        if quat_set {
            core::ptr::copy_nonoverlapping(quat, core::ptr::addr_of_mut!((*self_ptr).quat_) as *mut f64, 4);
        }
    }
}

/// C: mjCBoundingVolume::mjCBoundingVolume(const int* id, int contype, int conaffinity, const double pos[3], const double quat[4], const double aabb[6]) (user/user_objects.h:110) [C2 base-object constructor]
/// # Safety
/// Raw pointer arguments and the receiver, when present, must be non-null and designate live memory for each generated access, with the alignment required by that operation. Any byte read must be initialized; any location written must be writable; pointer offsets must stay within the originating allocation or one past it. No other thread may perform an unsynchronized conflicting access to the same memory during this operation.
#[allow(unused_variables, non_snake_case)]
/// # Safety
/// Pointer arguments read by this function must designate live, initialized, aligned storage.
/// Storage written by this function must be aligned, writable, and free of unsynchronized conflicting accesses.
/// The derived pointer must stay in its originating live array or one past its end.
pub unsafe fn __c2rust_mj_c_bounding_volume_mj_c_bounding_volume_520aaa174964aec1f5333844152b6b8111e5e428db922d0b23161a58fb95bc44(self_ptr: *mut mjCBoundingVolume, id: *const i32, contype: i32, conaffinity: i32, pos: *const f64, quat: *const f64, aabb: *const f64) -> () {
    unsafe {
        (*self_ptr).contype_ = contype;
        (*self_ptr).conaffinity_ = conaffinity;
        (*self_ptr).id_ = id;
        core::ptr::copy_nonoverlapping(aabb, core::ptr::addr_of_mut!((*self_ptr).aabb_) as *mut f64, 6);
        core::ptr::copy_nonoverlapping(pos, core::ptr::addr_of_mut!((*self_ptr).pos_) as *mut f64, 3);
        let quat_set = !quat.is_null();
        (*self_ptr).quat_set_ = quat_set;
        if quat_set {
            core::ptr::copy_nonoverlapping(quat, core::ptr::addr_of_mut!((*self_ptr).quat_) as *mut f64, 4);
        }
    }
}

/// C: mjCMesh::npolygon (user/user_objects.h:1218)
/// # Safety
/// Raw pointer arguments and the receiver, when present, must be non-null and designate live memory for each generated access, with the alignment required by that operation. Any byte read must be initialized; any location written must be writable; pointer offsets must stay within the originating allocation or one past it. No other thread may perform an unsynchronized conflicting access to the same memory during this operation.
#[allow(unused_variables, non_snake_case)]
pub unsafe fn __c2rust_mj_c_mesh_npolygon_38703406cfa30fe6fca31c55aeb354eefb32c0e01736d4baac052e72b354ebe3(self_ptr: *mut mjCMesh) -> i32 {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe {
 extern "C" { fn c2rust_std_sequence_size_1ee4741d6c3759c772c72dad425dd7cdc523c30841bd445c2c2acee687392421(sequence: *const core::ffi::c_void) -> usize; }
 c2rust_std_sequence_size_1ee4741d6c3759c772c72dad425dd7cdc523c30841bd445c2c2acee687392421((self_ptr as *const u8).add(1896) as *const core::ffi::c_void) as i32
}
}

pub unsafe fn __c2rust_mj_c_bounding_volume_mj_c_bounding_volume_af1bd0171d72ea49ca45b06f7ae24a20e52123659cbf817dc510f1e766fe3bd2(self_ptr: *mut mjCBoundingVolume, id: i32, contype: i32, conaffinity: i32, pos: *const f64, quat: *const f64, aabb: *const f64) -> () {
    unsafe { __c2rust_mj_c_bounding_volume_mj_c_bounding_volume_449138050b23981f27f7abcc2fce6ac48a8e32249a89d336ff66f8d3a5c323f5(self_ptr, id, contype, conaffinity, pos, quat, aabb); }
}

pub unsafe fn __c2rust_mj_c_bounding_volume_mj_c_bounding_volume_449138050b23981f27f7abcc2fce6ac48a8e32249a89d336ff66f8d3a5c323f5(self_ptr: *mut mjCBoundingVolume, id: i32, contype: i32, conaffinity: i32, pos: *const f64, quat: *const f64, aabb: *const f64) -> () {
    unsafe {
        core::ptr::addr_of_mut!((*self_ptr).contype_).write(contype);
        core::ptr::addr_of_mut!((*self_ptr).conaffinity_).write(conaffinity);
        core::ptr::addr_of_mut!((*self_ptr).idval_).write(id);
        core::ptr::addr_of_mut!((*self_ptr).id_).write(core::ptr::null());
        core::ptr::copy(pos, core::ptr::addr_of_mut!((*self_ptr).pos_).cast::<f64>(), 3);
        core::ptr::copy(aabb, core::ptr::addr_of_mut!((*self_ptr).aabb_).cast::<f64>(), 6);
        let quat_set = !quat.is_null();
        core::ptr::addr_of_mut!((*self_ptr).quat_set_).write(quat_set);
        if quat_set {
            core::ptr::copy(quat, core::ptr::addr_of_mut!((*self_ptr).quat_).cast::<f64>(), 4);
        }
    }
}

pub unsafe fn __c2rust_mj_c_body_pair_get_bodyname2_8028eae404c42d965253924f318910fece22b9d9eaaf216d781b65471155dd53(c2rust_result_out: *mut std__string, self_ptr: *mut mjCBodyPair) -> () {
    extern "C" {
        fn c2rust_std_string_copy_77903c6aa15af5bc136f5cc42a00618cfcce78616b72abc2bb6f1e34fd95fbce(destination: *mut core::ffi::c_void, source: *const core::ffi::c_void);
    }

    unsafe {
        let source_ptr = self_ptr.cast::<u8>().add(264).cast::<core::ffi::c_void>() as *const core::ffi::c_void;
        c2rust_std_string_copy_77903c6aa15af5bc136f5cc42a00618cfcce78616b72abc2bb6f1e34fd95fbce(
            c2rust_result_out.cast::<core::ffi::c_void>(),
            source_ptr,
        );
    }
}

/// C: mjCMesh::nnormal (user/user_objects.h:1215)
/// # Safety
/// Raw pointer arguments and the receiver, when present, must be non-null and designate live memory for each generated access, with the alignment required by that operation. Any byte read must be initialized; any location written must be writable; pointer offsets must stay within the originating allocation or one past it. No other thread may perform an unsynchronized conflicting access to the same memory during this operation.
#[allow(unused_variables, non_snake_case)]
pub unsafe fn __c2rust_mj_c_mesh_nnormal_e67acb2cd453cde6a21a38023b72fa26c6812d1041bc4ff34973e6a8504d2168(self_ptr: *mut mjCMesh) -> i32 {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe {
 extern "C" { fn c2rust_std_sequence_size_2435b07452a39ca59c527882cb7937c59937652c7a11e421e2530d830a8f1879(sequence: *const core::ffi::c_void) -> usize; }
 (c2rust_std_sequence_size_2435b07452a39ca59c527882cb7937c59937652c7a11e421e2530d830a8f1879((self_ptr as *const u8).add(352) as *const core::ffi::c_void) / 3usize) as i32
}
}

/// C: mjCTendon::get_userdata (user/user_objects.h:1727)
/// # Safety
/// Raw pointer arguments and the receiver, when present, must be non-null and designate live memory for each generated access, with the alignment required by that operation. Any byte read must be initialized; any location written must be writable; pointer offsets must stay within the originating allocation or one past it. No other thread may perform an unsynchronized conflicting access to the same memory during this operation.
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub unsafe fn __c2rust_mj_c_tendon_get_userdata_55bf0f65f3db8d6bf88a6020ed6e5f2a13bba05ae05c0f13333f488b4c161f23(self_ptr: *mut mjCTendon) -> *const std__vector_double {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { core::ptr::addr_of!((*self_ptr).userdata_) }
}

#[allow(unused_variables, non_snake_case)]
/// Clears every texture in the material.
///
/// # Safety
/// The caller must provide a live exclusive receiver.
pub unsafe fn __c2rust_mj_c_material_del_textures_514d82294d7eb55b47953e59ffb68a5919b424b987fbdf6a7fb7d1696095f537(self_ptr: *mut mjCMaterial) -> () {
    extern "C" {
        fn c2rust_std_sequence_clear_elements_045bf4587bb924f5fcf4463b7c5ec2ecc77923b392497a2580afe7386563e0d8_size(sequence: *const core::ffi::c_void) -> usize;
        fn c2rust_std_sequence_clear_elements_045bf4587bb924f5fcf4463b7c5ec2ecc77923b392497a2580afe7386563e0d8_clear_at(sequence: *mut core::ffi::c_void, index: usize);
    }
    unsafe {
        let sequence = self_ptr.cast::<u8>().add(264).cast::<core::ffi::c_void>();
        let count = c2rust_std_sequence_clear_elements_045bf4587bb924f5fcf4463b7c5ec2ecc77923b392497a2580afe7386563e0d8_size(sequence);
        for index in 0..count { c2rust_std_sequence_clear_elements_045bf4587bb924f5fcf4463b7c5ec2ecc77923b392497a2580afe7386563e0d8_clear_at(sequence, index); }
    }
}

/// C: mjCTendon::get_range (user/user_objects.h:1728)
/// # Safety
/// Raw pointer arguments and the receiver, when present, must be non-null and designate live memory for each generated access, with the alignment required by that operation. Any byte read must be initialized; any location written must be writable; pointer offsets must stay within the originating allocation or one past it. No other thread may perform an unsynchronized conflicting access to the same memory during this operation.
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub unsafe fn __c2rust_mj_c_tendon_get_range_28aebf78174d78d6d8e9be5364189a1592eae46781404cab9e89368614e98522(self_ptr: *mut mjCTendon) -> *const f64 {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { core::ptr::addr_of!((*self_ptr).range).cast::<f64>() }
}

/// C: mjCDef::Joint (user/user_objects.h:2098)
/// # Safety
/// Raw pointer arguments and the receiver, when present, must be non-null and designate live memory for each generated access, with the alignment required by that operation. Any byte read must be initialized; any location written must be writable; pointer offsets must stay within the originating allocation or one past it. No other thread may perform an unsynchronized conflicting access to the same memory during this operation.
#[allow(unused_variables, non_snake_case)]
pub unsafe fn __c2rust_mj_c_def_joint_d2e40d606e268fd09d03ad7f0690f7b8fd78897deb73fbcca2fc4646046036a9(self_ptr: *mut mjCDef) -> *mut mjCJoint {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { core::ptr::addr_of_mut!((*self_ptr).joint_) }
}
