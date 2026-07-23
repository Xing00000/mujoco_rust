//! Port of: xml/xml_urdf.h
//! IR hash: bd605ac8158c32d6
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mjXURDF::Parse (xml/xml_urdf.h:40)
/// Calls: mjXReader::Compiler, mjXReader::Option, mjXReader::Size, mjXURDF::AddBody, mjXURDF::AddToTree, mjXURDF::Body, mjXURDF::FindName, mjXURDF::GetPrefixedName, mjXURDF::Joint, mjXURDF::MakeMaterials, mjXUtil::FindSubElem, mjXUtil::ReadAttrTxt, mjs_addJoint, mjs_findBody, mjs_findChild, mjs_setName, mjs_setString, mjuu_copyvec
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_xurdf_parse(self_ptr: *mut mjXURDF, root: *mut tinyxml2__XMLElement, prefix: *const std__string, pos: *mut f64, quat: *mut f64, static_body: bool) {
    todo!() // mjXURDF::Parse
}

/// C: mjXURDF::GetPrefixedName (xml/xml_urdf.h:49)
#[allow(unused_variables, non_snake_case)]
pub fn mj_xurdf_get_prefixed_name(self_ptr: *mut mjXURDF, name: *const std__string) -> std__string {
    todo!() // mjXURDF::GetPrefixedName
}

/// C: mjXURDF::FindName (xml/xml_urdf.h:50)
#[allow(unused_variables, non_snake_case)]
pub fn mj_xurdf_find_name(self_ptr: *mut mjXURDF, name: std__string, list: *const ()) -> i32 {
    todo!() // mjXURDF::FindName
}

/// C: mjXURDF::AddName (xml/xml_urdf.h:51)
/// Calls: mjXURDF::FindName
#[allow(unused_variables, non_snake_case)]
pub fn mj_xurdf_add_name(self_ptr: *mut mjXURDF, name: std__string, list: *const ()) {
    todo!() // mjXURDF::AddName
}

/// C: mjXURDF::AddBody (xml/xml_urdf.h:52)
/// Calls: mjXURDF::AddName
#[allow(unused_variables, non_snake_case)]
pub fn mj_xurdf_add_body(self_ptr: *mut mjXURDF, name: std__string) {
    todo!() // mjXURDF::AddBody
}

/// C: mjXURDF::AddToTree (xml/xml_urdf.h:53)
/// Calls: mjs_addBody, mjs_findBody, mjs_findChild, mjs_setName
#[allow(unused_variables, non_snake_case)]
pub fn mj_xurdf_add_to_tree(self_ptr: *mut mjXURDF, n: i32) {
    todo!() // mjXURDF::AddToTree
}

/// C: mjXURDF::Body (xml/xml_urdf.h:54)
/// Calls: mjXURDF::FindName, mjXURDF::Geom, mjXURDF::GetPrefixedName, mjXURDF::Origin, mjXUtil::FindSubElem, mjXUtil::ReadAttr, mjXUtil::ReadAttrTxt, mjs_findBody, mjs_findChild, mjs_setName, mjuu_copyvec, mjuu_fullInertia, mjuu_mulquat, mjuu_setvec
#[allow(unused_variables, non_snake_case)]
pub fn mj_xurdf_body(self_ptr: *mut mjXURDF, body_elem: *mut tinyxml2__XMLElement) {
    todo!() // mjXURDF::Body
}

/// C: mjXURDF::Joint (xml/xml_urdf.h:55)
/// Calls: mjXURDF::GetPrefixedName, mjXURDF::Origin, mjXUtil::FindKey, mjXUtil::FindSubElem, mjXUtil::ReadAttr, mjXUtil::ReadAttrTxt, mjs_addJoint, mjs_findBody, mjs_findChild, mjs_setName, mjuu_copyvec, mjuu_quat2mat, mjuu_setvec, mjuu_z2quat
#[allow(unused_variables, non_snake_case)]
pub fn mj_xurdf_joint(self_ptr: *mut mjXURDF, joint_elem: *mut tinyxml2__XMLElement) {
    todo!() // mjXURDF::Joint
}

/// C: mjXURDF::Geom (xml/xml_urdf.h:56)
/// Calls: mjXURDF::Origin, mjXUtil::FindSubElem, mjXUtil::ReadAttr, mjXUtil::ReadAttrArr, mjXUtil::ReadAttrStr, mjs_addGeom, mjs_addMesh, mjs_setName, mjs_setString, mjuu_stripext, mjuu_strippath
#[allow(unused_variables, non_snake_case)]
pub fn mj_xurdf_geom(self_ptr: *mut mjXURDF, geom_elem: *mut tinyxml2__XMLElement, pbody: *mut mjsBody, collision: bool) -> *mut mjsGeom {
    todo!() // mjXURDF::Geom
}

/// C: mjXURDF::Origin (xml/xml_urdf.h:58)
/// Calls: mjXUtil::FindSubElem, mjXUtil::ReadAttr, mjs_defaultOrientation, mjs_resolveOrientation, mjuu_setvec
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_xurdf_origin(self_ptr: *mut mjXURDF, origin_elem: *mut tinyxml2__XMLElement, pos: *mut f64, quat: *mut f64) {
    todo!() // mjXURDF::Origin
}

/// C: mjXURDF::MakeMaterials (xml/xml_urdf.h:60)
/// Calls: mjXURDF::AddName, mjXURDF::FindName, mjXUtil::FindSubElem, mjXUtil::ReadAttr, mjXUtil::ReadAttrTxt
#[allow(unused_variables, non_snake_case)]
pub fn mj_xurdf_make_materials(self_ptr: *mut mjXURDF, elem: *mut tinyxml2__XMLElement) {
    todo!() // mjXURDF::MakeMaterials
}

/// C: mjXURDF::Clear (xml/xml_urdf.h:61)
#[allow(unused_variables, non_snake_case)]
pub fn mj_xurdf_clear(self_ptr: *mut mjXURDF) {
    todo!() // mjXURDF::Clear
}

