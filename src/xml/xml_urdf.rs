//! Port of: xml/xml_urdf.cc
//! IR hash: 05737965add36adb
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mjXURDF::FindName (xml/xml_urdf.cc:675)
#[allow(unused_variables, non_snake_case)]
pub fn mj_xurdf_find_name(self_ptr: *mut mjXURDF, name: string, list: *mut i32) -> i32 {
    extern "C" { fn mjXURDF_FindName_impl(self_ptr: *mut mjXURDF, name: string, list: *mut i32) -> i32; }
    // SAFETY: delegates to C implementation
    unsafe { mjXURDF_FindName_impl(self_ptr, name, list) }
}

/// C: mjXURDF::AddName (xml/xml_urdf.cc:687)
#[allow(unused_variables, non_snake_case)]
pub fn mj_xurdf_add_name(self_ptr: *mut mjXURDF, name: string, list: *mut i32) {
    extern "C" { fn mjXURDF_AddName_impl(self_ptr: *mut mjXURDF, name: string, list: *mut i32); }
    // SAFETY: delegates to C implementation
    unsafe { mjXURDF_AddName_impl(self_ptr, name, list) }
}

/// C: mjXURDF::Parse (xml/xml_urdf.h:40)
/// Calls: mjXURDF::AddBody, mjXURDF::AddToTree, mjXURDF::Body, mjXURDF::Joint, mjXURDF::MakeMaterials, mjXUtil::ReadAttrTxt, mjs_addJoint
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_xurdf_parse(self_ptr: *mut mjXURDF, root: *mut tinyxml2__XMLElement, prefix: *const std__string, pos: *mut f64, quat: *mut f64, static_body: bool) {
    extern "C" { fn mjXURDF_Parse_impl(self_ptr: *mut mjXURDF, root: *mut tinyxml2__XMLElement, prefix: *const std__string, pos: *mut f64, quat: *mut f64, static_body: bool); }
    // SAFETY: delegates to C implementation
    unsafe { mjXURDF_Parse_impl(self_ptr, root, prefix, pos, quat, static_body) }
}

/// C: mjXURDF::GetPrefixedName (xml/xml_urdf.h:49)
#[allow(unused_variables, non_snake_case)]
pub fn mj_xurdf_get_prefixed_name(self_ptr: *mut mjXURDF, name: *const std__string) -> std__string {
    extern "C" { fn mjXURDF_GetPrefixedName_impl(self_ptr: *mut mjXURDF, name: *const std__string) -> std__string; }
    // SAFETY: delegates to C implementation
    unsafe { mjXURDF_GetPrefixedName_impl(self_ptr, name) }
}

/// C: mjXURDF::AddBody (xml/xml_urdf.h:52)
#[allow(unused_variables, non_snake_case)]
pub fn mj_xurdf_add_body(self_ptr: *mut mjXURDF, name: string) {
    extern "C" { fn mjXURDF_AddBody_impl(self_ptr: *mut mjXURDF, name: string); }
    // SAFETY: delegates to C implementation
    unsafe { mjXURDF_AddBody_impl(self_ptr, name) }
}

/// C: mjXURDF::AddToTree (xml/xml_urdf.h:53)
/// Calls: mjs_addBody
#[allow(unused_variables, non_snake_case)]
pub fn mj_xurdf_add_to_tree(self_ptr: *mut mjXURDF, n: i32) {
    extern "C" { fn mjXURDF_AddToTree_impl(self_ptr: *mut mjXURDF, n: i32); }
    // SAFETY: delegates to C implementation
    unsafe { mjXURDF_AddToTree_impl(self_ptr, n) }
}

/// C: mjXURDF::Body (xml/xml_urdf.h:54)
/// Calls: mjXURDF::Geom, mjXURDF::Origin, mjXUtil::ReadAttrTxt, mjuu_fullInertia, mjuu_mulquat, mjuu_setvec
#[allow(unused_variables, non_snake_case)]
pub fn mj_xurdf_body(self_ptr: *mut mjXURDF, body_elem: *mut tinyxml2__XMLElement) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjXURDF, body_elem : * mut tinyxml2__XMLElement)
    // Previous return: ()
    todo ! ()
}

/// C: mjXURDF::Joint (xml/xml_urdf.h:55)
/// Calls: mjXURDF::Origin, mjXUtil::FindKey, mjXUtil::ReadAttrTxt, mjs_addJoint, mjuu_quat2mat, mjuu_setvec, mjuu_z2quat
#[allow(unused_variables, non_snake_case)]
pub fn mj_xurdf_joint(self_ptr: *mut mjXURDF, joint_elem: *mut tinyxml2__XMLElement) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjXURDF, joint_elem : * mut tinyxml2__XMLElement)
    // Previous return: ()
    todo ! ()
}

/// C: mjXURDF::Geom (xml/xml_urdf.h:56)
/// Calls: mjXURDF::Origin, mjs_addGeom, mjs_setName
#[allow(unused_variables, non_snake_case)]
pub fn mj_xurdf_geom(self_ptr: *mut mjXURDF, geom_elem: *mut tinyxml2__XMLElement, pbody: *mut mjsBody, collision: bool) -> *mut mjsGeom {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjXURDF, geom_elem : * mut tinyxml2__XMLElement, pbody : * mut mjsBody, collision : bool)
    // Previous return: * mut mjsGeom
    todo ! ()
}

/// C: mjXURDF::Origin (xml/xml_urdf.h:58)
/// Calls: mjs_defaultOrientation, mjs_resolveOrientation, mjuu_setvec
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_xurdf_origin(self_ptr: *mut mjXURDF, origin_elem: *mut tinyxml2__XMLElement, pos: *mut f64, quat: *mut f64) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjXURDF, origin_elem : * mut tinyxml2__XMLElement, pos : * mut f64, quat : * mut f64)
    // Previous return: ()
    todo ! ()
}

/// C: mjXURDF::MakeMaterials (xml/xml_urdf.h:60)
/// Calls: mjXUtil::ReadAttrTxt
#[allow(unused_variables, non_snake_case)]
pub fn mj_xurdf_make_materials(self_ptr: *mut mjXURDF, elem: *mut tinyxml2__XMLElement) {
    extern "C" { fn mjXURDF_MakeMaterials_impl(self_ptr: *mut mjXURDF, elem: *mut tinyxml2__XMLElement); }
    // SAFETY: delegates to C implementation
    unsafe { mjXURDF_MakeMaterials_impl(self_ptr, elem) }
}

/// C: mjXURDF::Clear (xml/xml_urdf.h:61)
#[allow(unused_variables, non_snake_case)]
pub fn mj_xurdf_clear(self_ptr: *mut mjXURDF) {
    extern "C" { fn mjXURDF_Clear_impl(self_ptr: *mut mjXURDF); }
    // SAFETY: delegates to C implementation
    unsafe { mjXURDF_Clear_impl(self_ptr) }
}

