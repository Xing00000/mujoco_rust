//! Port of: Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_composite.h
//! IR hash: 1fa44043fe383ce9
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mjCComposite::SetDefault (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_composite.h:61)
/// Calls: cxx:__ZN12mjCComposite15AddDefaultJointEPci
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_composite_set_default(self_ptr: *mut mjCComposite) {
    todo!() // cxx:__ZN12mjCComposite10SetDefaultEv
}

/// C: mjCComposite::AddDefaultJoint (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_composite.h:62)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_composite_add_default_joint(self_ptr: *mut mjCComposite, error: *mut i8, error_sz: i32) -> bool {
    todo!() // cxx:__ZN12mjCComposite15AddDefaultJointEPci
}

/// C: mjCComposite::Make (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_composite.h:64)
/// Calls: cxx:__Z9mjuu_dot3PKdS0_, cxx:__ZN12mjCComposite9MakeCableEP8mjCModelP8mjsBody_Pci
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_composite_make(self_ptr: *mut mjCComposite, spec: *mut mjSpec, body: *mut mjsBody, error: *mut i8, error_sz: i32) -> bool {
    todo!() // cxx:__ZN12mjCComposite4MakeEP7mjSpec_P8mjsBody_Pci
}

/// C: mjCComposite::MakeCable (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_composite.h:65)
/// Calls: cxx:__Z15mjuu_rotVecQuatPdPKdS1_, cxx:__ZN12mjCComposite12AddCableBodyEP8mjCModelP8mjsBody_iPdS4_, cxx:__ZN12mjCComposite16MakeSkin2SubgridEP8mjCModeld, cxx:__ZN12mjCComposite9MakeSkin2EP8mjCModeld, cxx:_mjs_addText, cxx:_mjs_setName, cxx:_mjs_setString, cxx:_mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_composite_make_cable(self_ptr: *mut mjCComposite, model: *mut mjCModel, body: *mut mjsBody, error: *mut i8, error_sz: i32) -> bool {
    todo!() // cxx:__ZN12mjCComposite9MakeCableEP8mjCModelP8mjsBody_Pci
}

/// C: mjCComposite::MakeSkin2 (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_composite.h:67)
/// Calls: cxx:__ZN12mjCComposite12CopyIntoSkinEP8mjsSkin_, cxx:__ZN12mjCComposite14MakeCableBonesEP8mjCModelP8mjsSkin_, cxx:_mjs_addSkin, cxx:_mjs_setName, cxx:_mjs_setString
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_composite_make_skin2(self_ptr: *mut mjCComposite, model: *mut mjCModel, inflate: f64) {
    todo!() // cxx:__ZN12mjCComposite9MakeSkin2EP8mjCModeld
}

/// C: mjCComposite::MakeSkin2Subgrid (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_composite.h:68)
/// Calls: cxx:__ZN12mjCComposite12CopyIntoSkinEP8mjsSkin_, cxx:__ZN12mjCComposite21MakeCableBonesSubgridEP8mjCModelP8mjsSkin_, cxx:_mjs_addSkin, cxx:_mjs_setName, cxx:_mjs_setString, cxx:_mju_free, cxx:_mju_malloc, cxx:_mju_mulMatMat, cxx:_mju_round, cxx:_mju_zero
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_composite_make_skin2subgrid(self_ptr: *mut mjCComposite, model: *mut mjCModel, inflate: f64) {
    todo!() // cxx:__ZN12mjCComposite16MakeSkin2SubgridEP8mjCModeld
}

/// C: mjCComposite::MakeCableBones (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_composite.h:69)
/// Calls: cxx:_mjs_appendString
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_composite_make_cable_bones(self_ptr: *mut mjCComposite, model: *mut mjCModel, skin: *mut mjsSkin) {
    todo!() // cxx:__ZN12mjCComposite14MakeCableBonesEP8mjCModelP8mjsSkin_
}

/// C: mjCComposite::MakeCableBonesSubgrid (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_composite.h:70)
/// Calls: cxx:__ZN6mjCDef4GeomEv, cxx:_mjs_appendString
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_composite_make_cable_bones_subgrid(self_ptr: *mut mjCComposite, model: *mut mjCModel, skin: *mut mjsSkin) {
    todo!() // cxx:__ZN12mjCComposite21MakeCableBonesSubgridEP8mjCModelP8mjsSkin_
}

/// C: mjCComposite::AddCableBody (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_composite.h:112)
/// Calls: cxx:__Z12mjuu_mulquatPdPKdS1_, cxx:__Z16mjuu_updateFramePdS_PKdS1_S1_i, cxx:_mjs_addBody, cxx:_mjs_addExclude, cxx:_mjs_addGeom, cxx:_mjs_addJoint, cxx:_mjs_addSite, cxx:_mjs_getDefault, cxx:_mjs_getString, cxx:_mjs_setDefault, cxx:_mjs_setFrame, cxx:_mjs_setName, cxx:_mjs_setString
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_composite_add_cable_body(self_ptr: *mut mjCComposite, model: *mut mjCModel, body: *mut mjsBody, ix: i32, normal: *mut f64, prev_quat: *mut f64) -> *mut mjsBody {
    todo!() // cxx:__ZN12mjCComposite12AddCableBodyEP8mjCModelP8mjsBody_iPdS4_
}

/// C: mjCComposite::CopyIntoSkin (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/user/user_composite.h:115)
/// Calls: cxx:_mjs_appendFloatVec, cxx:_mjs_appendIntVec, cxx:_mjs_setFloat, cxx:_mjs_setInt
#[allow(unused_variables, non_snake_case)]
pub fn mj_c_composite_copy_into_skin(self_ptr: *mut mjCComposite, skin: *mut mjsSkin) {
    todo!() // cxx:__ZN12mjCComposite12CopyIntoSkinEP8mjsSkin_
}

