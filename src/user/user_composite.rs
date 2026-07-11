//! Port of: user/user_composite.cc
//! IR hash: 05737965add36adb
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: comperr (user/user_composite.cc:41)
/// Calls: mju_strncpy
#[allow(unused_variables, non_snake_case)]
pub fn comperr(error: *mut i8, msg: *const i8, error_sz: i32) -> bool  {
    if error.is_null() { return false; }
    extern "C" { fn comperr(error: *mut i8, msg: *const i8, error_sz: i32) -> bool; }
    // SAFETY: error verified non-null
    unsafe { comperr(error, msg, error_sz) }
}

/// C: mjCComposite::SetDefault (user/user_composite.h:61)
/// Calls: mjCComposite::AddDefaultJoint
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_composite_set_default(self_ptr: *mut mjCComposite) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCComposite_SetDefault(self_ptr: *mut mjCComposite); }
    // SAFETY: self_ptr verified non-null; delegates to C implementation
    unsafe { mjCComposite_SetDefault(self_ptr) }
}

/// C: mjCComposite::AddDefaultJoint (user/user_composite.h:62)
/// Calls: comperr
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_composite_add_default_joint(self_ptr: *mut mjCComposite, error: *mut i8, error_sz: i32) -> bool {
    if self_ptr.is_null() { return false; }
    extern "C" { fn mjCComposite_AddDefaultJoint(self_ptr: *mut mjCComposite, error: *mut i8, error_sz: i32) -> bool; }
    // SAFETY: self_ptr verified non-null; delegates to C implementation
    unsafe { mjCComposite_AddDefaultJoint(self_ptr, error, error_sz) }
}

/// C: mjCComposite::Make (user/user_composite.h:64)
/// Calls: comperr, mjCComposite::MakeCable
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_composite_make(self_ptr: *mut mjCComposite, spec: *mut mjSpec, body: *mut mjsBody, error: *mut i8, error_sz: i32) -> bool {
    extern "C" { fn mjCComposite_Make(self_ptr: *mut mjCComposite, spec: *mut mjSpec, body: *mut mjsBody, error: *mut i8, error_sz: i32) -> bool; }
    // SAFETY: delegates to C++ implementation; caller guarantees pointer validity
    unsafe { mjCComposite_Make(self_ptr, spec, body, error, error_sz) }
}

/// C: mjCComposite::MakeCable (user/user_composite.h:65)
/// Calls: comperr, mjCComposite::AddCableBody, mjs_addText, mju_error, mjuu_rotVecQuat, mjuu_setvec
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_composite_make_cable(self_ptr: *mut mjCComposite, model: *mut mjCModel, body: *mut mjsBody, error: *mut i8, error_sz: i32) -> bool {
    extern "C" { fn mjCComposite_MakeCable(self_ptr: *mut mjCComposite, model: *mut mjCModel, body: *mut mjsBody, error: *mut i8, error_sz: i32) -> bool; }
    // SAFETY: delegates to C++ implementation; caller guarantees pointer validity
    unsafe { mjCComposite_MakeCable(self_ptr, model, body, error, error_sz) }
}

/// C: mjCComposite::MakeSkin2 (user/user_composite.h:67)
/// Calls: mjCComposite::CopyIntoSkin, mjCComposite::MakeCableBones, mjs_addSkin, mjs_setName
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_composite_make_skin2(self_ptr: *mut mjCComposite, model: *mut mjCModel, inflate: f64) {
    extern "C" { fn mjCComposite_MakeSkin2(self_ptr: *mut mjCComposite, model: *mut mjCModel, inflate: f64); }
    // SAFETY: delegates to C implementation
    unsafe { mjCComposite_MakeSkin2(self_ptr, model, inflate) }
}

/// C: mjCComposite::MakeSkin2Subgrid (user/user_composite.h:68)
/// Calls: mjCComposite::CopyIntoSkin, mjCComposite::MakeCableBonesSubgrid, mjs_addSkin, mjs_setName, mju_free, mju_malloc, mju_mulMatMat, mju_round, mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_composite_make_skin2subgrid(self_ptr: *mut mjCComposite, model: *mut mjCModel, inflate: f64) {
    extern "C" { fn mjCComposite_MakeSkin2Subgrid(self_ptr: *mut mjCComposite, model: *mut mjCModel, inflate: f64); }
    // SAFETY: delegates to C implementation
    unsafe { mjCComposite_MakeSkin2Subgrid(self_ptr, model, inflate) }
}

/// C: mjCComposite::MakeCableBones (user/user_composite.h:69)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_composite_make_cable_bones(self_ptr: *mut mjCComposite, model: *mut mjCModel, skin: *mut mjsSkin) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCComposite_MakeCableBones(self_ptr: *mut mjCComposite, model: *mut mjCModel, skin: *mut mjsSkin); }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCComposite_MakeCableBones(self_ptr, model, skin) }
}

/// C: mjCComposite::MakeCableBonesSubgrid (user/user_composite.h:70)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_composite_make_cable_bones_subgrid(self_ptr: *mut mjCComposite, model: *mut mjCModel, skin: *mut mjsSkin) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCComposite_MakeCableBonesSubgrid(self_ptr: *mut mjCComposite, model: *mut mjCModel, skin: *mut mjsSkin); }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCComposite_MakeCableBonesSubgrid(self_ptr, model, skin) }
}

/// C: mjCComposite::AddCableBody (user/user_composite.h:112)
/// Calls: mjs_addBody, mjs_addExclude, mjs_getDefault, mjs_getString, mjs_setDefault, mjs_setFrame, mjs_setName, mjs_setString, mjuu_mulquat, mjuu_normvec, mjuu_setvec, mjuu_updateFrame, mjuu_zerovec
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_composite_add_cable_body(self_ptr: *mut mjCComposite, model: *mut mjCModel, body: *mut mjsBody, ix: i32, normal: [f64; 3], prev_quat: [f64; 4]) -> *mut mjsBody {
    extern "C" { fn mjCComposite_AddCableBody(self_ptr: *mut mjCComposite, model: *mut mjCModel, body: *mut mjsBody, ix: i32, normal: [f64; 3], prev_quat: [f64; 4]) -> *mut mjsBody; }
    // SAFETY: delegates to C implementation
    unsafe { mjCComposite_AddCableBody(self_ptr, model, body, ix, normal, prev_quat) }
}

/// C: mjCComposite::CopyIntoSkin (user/user_composite.h:115)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_composite_copy_into_skin(self_ptr: *mut mjCComposite, skin: *mut mjsSkin) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjCComposite_CopyIntoSkin(self_ptr: *mut mjCComposite, skin: *mut mjsSkin); }
    // SAFETY: self_ptr verified non-null
    unsafe { mjCComposite_CopyIntoSkin(self_ptr, skin) }
}

