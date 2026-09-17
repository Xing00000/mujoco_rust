//! Port of: Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/xml/xml_urdf.cc
//! IR hash: 1fa44043fe383ce9
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mjXURDF::Parse (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/xml/xml_urdf.cc:85)
/// Calls: cxx:__ZN7mjXURDF15GetPrefixedNameERKNSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEE, cxx:__ZN7mjXURDF7AddBodyENSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEE, cxx:__ZN7mjXURDF8FindNameENSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEERNS0_6vectorIS6_NS4_IS6_EEEE, cxx:__ZN7mjXURDF9AddToTreeEi, cxx:__ZN7mjXUtil11ReadAttrTxtEPN8tinyxml210XMLElementEPKcRNSt3__112basic_stringIcNS5_11char_traitsIcEENS5_9allocatorIcEEEEb, cxx:_mjs_addJoint, cxx:_mjs_findBody, cxx:_mjs_findChild, cxx:_mjs_setName, cxx:_mjs_setString
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_xurdf_parse__378e446e4bf3a86a(self_ptr: *mut mjXURDF, root: *mut tinyxml2__XMLElement, prefix: *const std__string, pos: *mut f64, quat: *mut f64, static_body: bool) {
    todo!() // cxx:__ZN7mjXURDF5ParseEPN8tinyxml210XMLElementERKNSt3__112basic_stringIcNS3_11char_traitsIcEENS3_9allocatorIcEEEEPdSC_b
}

/// C: mjXURDF::Body (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/xml/xml_urdf.cc:227)
/// Calls: cxx:__Z12mjuu_mulquatPdPKdS1_, cxx:__Z16mjuu_fullInertiaPdS_PKd, cxx:__ZN7mjXURDF15GetPrefixedNameERKNSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEE, cxx:__ZN7mjXURDF8FindNameENSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEERNS0_6vectorIS6_NS4_IS6_EEEE, cxx:__ZN7mjXUtil11ReadAttrTxtEPN8tinyxml210XMLElementEPKcRNSt3__112basic_stringIcNS5_11char_traitsIcEENS5_9allocatorIcEEEEb, cxx:_mjs_findBody, cxx:_mjs_findChild, cxx:_mjs_setName
#[allow(unused_variables, non_snake_case)]
pub fn mj_xurdf_body(self_ptr: *mut mjXURDF, body_elem: *mut tinyxml2__XMLElement) {
    todo!() // cxx:__ZN7mjXURDF4BodyEPN8tinyxml210XMLElementE
}

/// C: mjXURDF::Parse (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/xml/xml_urdf.cc:366)
#[allow(unused_variables, non_snake_case)]
pub fn mj_xurdf_parse__13bd18156d939fb8(self_ptr: *mut mjXURDF, root: *mut tinyxml2__XMLElement, vfs: *const mjVFS) {
    todo!() // cxx:__ZN7mjXURDF5ParseEPN8tinyxml210XMLElementEPK6mjVFS_
}

/// C: mjXURDF::Joint (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/xml/xml_urdf.cc:375)
/// Calls: cxx:__Z11mjuu_z2quatPdPKd, cxx:__Z13mjuu_quat2matPdPKd, cxx:__ZN7mjXURDF15GetPrefixedNameERKNSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEE, cxx:__ZN7mjXUtil11ReadAttrTxtEPN8tinyxml210XMLElementEPKcRNSt3__112basic_stringIcNS5_11char_traitsIcEENS5_9allocatorIcEEEEb, cxx:__ZN7mjXUtil7FindKeyEPK6_mjMapiNSt3__112basic_stringIcNS3_11char_traitsIcEENS3_9allocatorIcEEEE, cxx:_mjs_addJoint, cxx:_mjs_findBody, cxx:_mjs_findChild, cxx:_mjs_setName
#[allow(unused_variables, non_snake_case)]
pub fn mj_xurdf_joint(self_ptr: *mut mjXURDF, joint_elem: *mut tinyxml2__XMLElement) {
    todo!() // cxx:__ZN7mjXURDF5JointEPN8tinyxml210XMLElementE
}

/// C: mjXURDF::Geom (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/xml/xml_urdf.cc:525)
/// Calls: cxx:__Z13mjuu_stripextNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEE, cxx:__Z14mjuu_strippathNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEE, cxx:_mjs_addGeom, cxx:_mjs_addMesh, cxx:_mjs_setName, cxx:_mjs_setString
#[allow(unused_variables, non_snake_case)]
pub fn mj_xurdf_geom(self_ptr: *mut mjXURDF, geom_elem: *mut tinyxml2__XMLElement, pbody: *mut mjsBody, collision: bool) -> *mut mjsGeom {
    todo!() // cxx:__ZN7mjXURDF4GeomEPN8tinyxml210XMLElementEP8mjsBody_b
}

/// C: mjXURDF::Origin (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/xml/xml_urdf.cc:649)
/// Calls: cxx:_mjs_defaultOrientation, cxx:_mjs_resolveOrientation
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_xurdf_origin(self_ptr: *mut mjXURDF, origin_elem: *mut tinyxml2__XMLElement, pos: *mut f64, quat: *mut f64) {
    todo!() // cxx:__ZN7mjXURDF6OriginEPN8tinyxml210XMLElementEPdS3_
}

/// C: mjXURDF::MakeMaterials (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/xml/xml_urdf.cc:742)
/// Calls: cxx:__ZN7mjXURDF7AddNameENSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEERNS0_6vectorIS6_NS4_IS6_EEEE, cxx:__ZN7mjXURDF8FindNameENSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEERNS0_6vectorIS6_NS4_IS6_EEEE, cxx:__ZN7mjXUtil11ReadAttrTxtEPN8tinyxml210XMLElementEPKcRNSt3__112basic_stringIcNS5_11char_traitsIcEENS5_9allocatorIcEEEEb
#[allow(unused_variables, non_snake_case)]
pub fn mj_xurdf_make_materials(self_ptr: *mut mjXURDF, elem: *mut tinyxml2__XMLElement) {
    todo!() // cxx:__ZN7mjXURDF13MakeMaterialsEPN8tinyxml210XMLElementE
}

/// C: mjXURDF::GetPrefixedName (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/xml/xml_urdf.h:49)
#[allow(unused_variables, non_snake_case)]
pub fn mj_xurdf_get_prefixed_name(self_ptr: *mut mjXURDF, name: *const std__string) -> std__string {
    todo!() // cxx:__ZN7mjXURDF15GetPrefixedNameERKNSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEE
}

/// C: mjXURDF::FindName (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/xml/xml_urdf.h:50)
#[allow(unused_variables, non_snake_case)]
pub fn mj_xurdf_find_name(self_ptr: *mut mjXURDF, name: std__string, list: *const ()) -> i32 {
    todo!() // cxx:__ZN7mjXURDF8FindNameENSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEERNS0_6vectorIS6_NS4_IS6_EEEE
}

/// C: mjXURDF::AddName (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/xml/xml_urdf.h:51)
/// Calls: cxx:__ZN7mjXURDF8FindNameENSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEERNS0_6vectorIS6_NS4_IS6_EEEE
#[allow(unused_variables, non_snake_case)]
pub fn mj_xurdf_add_name(self_ptr: *mut mjXURDF, name: std__string, list: *const ()) {
    todo!() // cxx:__ZN7mjXURDF7AddNameENSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEERNS0_6vectorIS6_NS4_IS6_EEEE
}

/// C: mjXURDF::AddBody (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/xml/xml_urdf.h:52)
/// Calls: cxx:__ZN7mjXURDF7AddNameENSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEERNS0_6vectorIS6_NS4_IS6_EEEE
#[allow(unused_variables, non_snake_case)]
pub fn mj_xurdf_add_body(self_ptr: *mut mjXURDF, name: std__string) {
    todo!() // cxx:__ZN7mjXURDF7AddBodyENSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEE
}

/// C: mjXURDF::AddToTree (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/xml/xml_urdf.h:53)
/// Calls: cxx:_mjs_addBody, cxx:_mjs_findBody, cxx:_mjs_findChild, cxx:_mjs_setName
#[allow(unused_variables, non_snake_case)]
pub fn mj_xurdf_add_to_tree(self_ptr: *mut mjXURDF, n: i32) {
    todo!() // cxx:__ZN7mjXURDF9AddToTreeEi
}

/// C: mjXURDF::Clear (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/xml/xml_urdf.h:61)
#[allow(unused_variables, non_snake_case)]
pub fn mj_xurdf_clear(self_ptr: *mut mjXURDF) {
    todo!() // cxx:__ZN7mjXURDF5ClearEv
}

