//! Port of: xml/xml_urdf.cc
//! IR hash: d3ac8715281cd691
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mjXURDF::FindName (xml/xml_urdf.cc:675)
#[allow(unused_variables, non_snake_case)]
pub fn mj_xurdf_find_name(self_ptr: *mut mjXURDF, name: string, list: *mut i32) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjXURDF, name : string, list : * mut i32)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: mjXURDF::AddName (xml/xml_urdf.cc:687)
#[allow(unused_variables, non_snake_case)]
pub fn mj_xurdf_add_name(self_ptr: *mut mjXURDF, name: string, list: *mut i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjXURDF, name : string, list : * mut i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
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
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjXURDF, root : * mut tinyxml2__XMLElement, prefix : * const std__string, pos : * mut f64, quat : * mut f64, static_body : bool)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjXURDF::GetPrefixedName (xml/xml_urdf.h:49)
#[allow(unused_variables, non_snake_case)]
pub fn mj_xurdf_get_prefixed_name(self_ptr: *mut mjXURDF, name: *const std__string) -> std__string {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjXURDF, name : * const std__string)
    // Previous return: std__string
    todo!("re-translate: params renamed")
}

/// C: mjXURDF::AddBody (xml/xml_urdf.h:52)
#[allow(unused_variables, non_snake_case)]
pub fn mj_xurdf_add_body(self_ptr: *mut mjXURDF, name: string) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjXURDF, name : string)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjXURDF::AddToTree (xml/xml_urdf.h:53)
/// Calls: mjs_addBody
#[allow(unused_variables, non_snake_case)]
pub fn mj_xurdf_add_to_tree(self_ptr: *mut mjXURDF, n: i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjXURDF, n : i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjXURDF::Body (xml/xml_urdf.h:54)
/// Calls: mjXURDF::Geom, mjXURDF::Origin, mjXUtil::ReadAttrTxt, mjuu_fullInertia, mjuu_mulquat, mjuu_setvec
#[allow(unused_variables, non_snake_case)]
pub fn mj_xurdf_body(self_ptr: *mut mjXURDF, body_elem: *mut tinyxml2__XMLElement) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjXURDF, body_elem : * mut tinyxml2__XMLElement)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjXURDF::Joint (xml/xml_urdf.h:55)
/// Calls: mjXURDF::Origin, mjXUtil::FindKey, mjXUtil::ReadAttrTxt, mjs_addJoint, mjuu_quat2mat, mjuu_setvec, mjuu_z2quat
#[allow(unused_variables, non_snake_case)]
pub fn mj_xurdf_joint(self_ptr: *mut mjXURDF, joint_elem: *mut tinyxml2__XMLElement) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjXURDF, joint_elem : * mut tinyxml2__XMLElement)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjXURDF::Geom (xml/xml_urdf.h:56)
/// Calls: mjXURDF::Origin, mjs_addGeom, mjs_setName
#[allow(unused_variables, non_snake_case)]
pub fn mj_xurdf_geom(self_ptr: *mut mjXURDF, geom_elem: *mut tinyxml2__XMLElement, pbody: *mut mjsBody, collision: bool) -> *mut mjsGeom {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjXURDF, geom_elem : * mut tinyxml2__XMLElement, pbody : * mut mjsBody, collision : bool)
    // Previous return: * mut mjsGeom
    todo!("re-translate: params renamed")
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
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjXURDF, origin_elem : * mut tinyxml2__XMLElement, pos : * mut f64, quat : * mut f64)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjXURDF::MakeMaterials (xml/xml_urdf.h:60)
/// Calls: mjXUtil::ReadAttrTxt
#[allow(unused_variables, non_snake_case)]
pub fn mj_xurdf_make_materials(self_ptr: *mut mjXURDF, elem: *mut tinyxml2__XMLElement) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjXURDF, elem : * mut tinyxml2__XMLElement)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjXURDF::Clear (xml/xml_urdf.h:61)
#[allow(unused_variables, non_snake_case)]
pub fn mj_xurdf_clear(self_ptr: *mut mjXURDF) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjXURDF)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

