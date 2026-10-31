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
