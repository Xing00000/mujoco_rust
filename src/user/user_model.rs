//! Port of: user/user_model.h
//! IR hash: 61e4c7a3dcebc7d6
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;


/// C: mjCModel::HFields (user/user_model.h:302)
/// # Safety
/// Raw pointer arguments and the receiver, when present, must be non-null and designate live memory for each generated access, with the alignment required by that operation. Any byte read must be initialized; any location written must be writable; pointer offsets must stay within the originating allocation or one past it. No other thread may perform an unsynchronized conflicting access to the same memory during this operation.
#[allow(unused_variables, non_snake_case)]
pub unsafe fn mj_c_model_h_fields(self_ptr: *mut mjCModel) -> *const std__vector_mjCHField {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { core::ptr::addr_of!((*self_ptr).hfields_) }
}

/// C: mjCModel::Texts (user/user_model.h:312)
/// # Safety
/// Raw pointer arguments and the receiver, when present, must be non-null and designate live memory for each generated access, with the alignment required by that operation. Any byte read must be initialized; any location written must be writable; pointer offsets must stay within the originating allocation or one past it. No other thread may perform an unsynchronized conflicting access to the same memory during this operation.
#[allow(unused_variables, non_snake_case)]
pub unsafe fn mj_c_model_texts(self_ptr: *mut mjCModel) -> *const std__vector_mjCText {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { core::ptr::addr_of!((*self_ptr).texts_) }
}

/// C: mjCModel::Bodies (user/user_model.h:316)
/// # Safety
/// Raw pointer arguments and the receiver, when present, must be non-null and designate live memory for each generated access, with the alignment required by that operation. Any byte read must be initialized; any location written must be writable; pointer offsets must stay within the originating allocation or one past it. No other thread may perform an unsynchronized conflicting access to the same memory during this operation.
#[allow(unused_variables, non_snake_case)]
pub unsafe fn mj_c_model_bodies(self_ptr: *mut mjCModel) -> *const std__vector_mjCBody {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { core::ptr::addr_of!((*self_ptr).bodies_) }
}

/// C: mjCModel::GetRef (user/user_model.h:378)
/// # Safety
/// Raw pointer arguments and the receiver, when present, must be non-null and designate live memory for each generated access, with the alignment required by that operation. Any byte read must be initialized; any location written must be writable; pointer offsets must stay within the originating allocation or one past it. No other thread may perform an unsynchronized conflicting access to the same memory during this operation.
#[allow(unused_variables, non_snake_case)]
pub unsafe fn mj_c_model_get_ref(self_ptr: *mut mjCModel) -> i32 {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).refcount }
}

/// C: mjCModel::Materials (user/user_model.h:304)
/// # Safety
/// Raw pointer arguments and the receiver, when present, must be non-null and designate live memory for each generated access, with the alignment required by that operation. Any byte read must be initialized; any location written must be writable; pointer offsets must stay within the originating allocation or one past it. No other thread may perform an unsynchronized conflicting access to the same memory during this operation.
#[allow(unused_variables, non_snake_case)]
pub unsafe fn mj_c_model_materials(self_ptr: *mut mjCModel) -> *const std__vector_mjCMaterial {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { core::ptr::addr_of!((*self_ptr).materials_) }
}

/// C: mjCModel::ClearCompileWarnings (user/user_model.h:270)
/// # Safety
/// Raw pointer arguments and the receiver, when present, must be non-null and designate live memory for each generated access, with the alignment required by that operation. Any byte read must be initialized; any location written must be writable; pointer offsets must stay within the originating allocation or one past it. No other thread may perform an unsynchronized conflicting access to the same memory during this operation.
#[allow(unused_variables, non_snake_case)]
pub unsafe fn mj_c_model_clear_compile_warnings(self_ptr: *mut mjCModel) {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe {
 extern "C" { fn c2rust_std_sequence_resize_e236d82e964305831c6167fa951b9fb7ac3f9bfe4efc68e6f954626866c201a1(sequence: *mut core::ffi::c_void, count: usize); }
 let length = core::ptr::read_unaligned((self_ptr as *const u8).add(5832) as *const i32) as u64;
 c2rust_std_sequence_resize_e236d82e964305831c6167fa951b9fb7ac3f9bfe4efc68e6f954626866c201a1((self_ptr as *mut u8).add(5808) as *mut core::ffi::c_void, length as usize);
}
}

/// C: mjCModel::Keys (user/user_model.h:314)
/// # Safety
/// Raw pointer arguments and the receiver, when present, must be non-null and designate live memory for each generated access, with the alignment required by that operation. Any byte read must be initialized; any location written must be writable; pointer offsets must stay within the originating allocation or one past it. No other thread may perform an unsynchronized conflicting access to the same memory during this operation.
#[allow(unused_variables, non_snake_case)]
pub unsafe fn __c2rust_mj_c_model_keys_5f3b94e0317ba018356aca6a1707169ab614af7e4c44166893063252275be553(self_ptr: *mut mjCModel) -> *const std__vector_mjCKey {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { core::ptr::addr_of!((*self_ptr).keys_) }
}
