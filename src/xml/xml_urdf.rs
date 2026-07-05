//! Port of: xml/xml_urdf.cc
//! IR hash: 699b5f0da57e8d78
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mjXURDF::GetPrefixedName (xml/xml_urdf.cc:74)
#[allow(unused_variables, non_snake_case)]
pub fn mj_xurdf_get_prefixed_name(name: *const i32) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (name : * const i32)
    // Previous return: i32
    todo ! ()
}

/// C: mjXURDF::FindName (xml/xml_urdf.cc:675)
#[allow(unused_variables, non_snake_case)]
pub fn mj_xurdf_find_name(name: i32, list: *mut i32) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (name : i32, list : * mut i32)
    // Previous return: i32
    todo ! ()
}

/// C: mjXURDF::AddName (xml/xml_urdf.cc:687)
#[allow(unused_variables, non_snake_case)]
pub fn mj_xurdf_add_name(name: i32, list: *mut i32) {
    // WARNING: signature changed — verify body
    // Previous params: (name : i32, list : * mut i32)
    // Previous return: ()
    todo ! ()
}

/// C: mjXURDF::AddBody (xml/xml_urdf.cc:699)
#[allow(unused_variables, non_snake_case)]
pub fn mj_xurdf_add_body(name: i32) {
    // WARNING: signature changed — verify body
    // Previous params: (name : i32)
    // Previous return: ()
    todo ! ()
}

/// C: mjXURDF::Parse (xml/xml_urdf.h:46)
/// Calls: mjuu_setvec
#[allow(unused_variables, non_snake_case)]
pub fn mj_xurdf_parse(root: *mut tinyxml2__XMLElement, vfs: *const mjVFS) {
    // WARNING: signature changed — verify body
    // Previous params: (root : * mut tinyxml2__XMLElement, vfs : * const mjVFS)
    // Previous return: ()
    todo ! ()
}

/// C: mjXURDF::AddToTree (xml/xml_urdf.h:53)
/// Calls: mjs_addBody
#[allow(unused_variables, non_snake_case)]
pub fn mj_xurdf_add_to_tree(n: i32) {
    // WARNING: signature changed — verify body
    // Previous params: (n : i32)
    // Previous return: ()
    todo ! ()
}

/// C: mjXURDF::Body (xml/xml_urdf.h:54)
/// Calls: mjXURDF::Geom, mjXURDF::Origin, mjuu_fullInertia, mjuu_mulquat, mjuu_setvec
#[allow(unused_variables, non_snake_case)]
pub fn mj_xurdf_body(body_elem: *mut tinyxml2__XMLElement) {
    // WARNING: signature changed — verify body
    // Previous params: (body_elem : * mut tinyxml2__XMLElement)
    // Previous return: ()
    todo ! ()
}

/// C: mjXURDF::Joint (xml/xml_urdf.h:55)
/// Calls: mjXURDF::Origin, mjs_addJoint, mjuu_quat2mat, mjuu_setvec, mjuu_z2quat
#[allow(unused_variables, non_snake_case)]
pub fn mj_xurdf_joint(joint_elem: *mut tinyxml2__XMLElement) {
    // WARNING: signature changed — verify body
    // Previous params: (joint_elem : * mut tinyxml2__XMLElement)
    // Previous return: ()
    todo ! ()
}

/// C: mjXURDF::Geom (xml/xml_urdf.h:56)
/// Calls: mjXURDF::Origin, mjs_addGeom, mjs_setName
#[allow(unused_variables, non_snake_case)]
pub fn mj_xurdf_geom(geom_elem: *mut tinyxml2__XMLElement, pbody: *mut mjsBody, collision: bool) -> *mut mjsGeom {
    // WARNING: signature changed — verify body
    // Previous params: (geom_elem : * mut tinyxml2__XMLElement, pbody : * mut mjsBody, collision : bool)
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
pub fn mj_xurdf_origin(origin_elem: *mut tinyxml2__XMLElement, pos: *mut f64, quat: *mut f64) {
    // WARNING: signature changed — verify body
    // Previous params: (origin_elem : * mut tinyxml2__XMLElement, pos : * mut f64, quat : * mut f64)
    // Previous return: ()
    todo ! ()
}

/// C: mjXURDF::MakeMaterials (xml/xml_urdf.h:60)
#[allow(unused_variables, non_snake_case)]
pub fn mj_xurdf_make_materials(elem: *mut tinyxml2__XMLElement) {
    // WARNING: signature changed — verify body
    // Previous params: (elem : * mut tinyxml2__XMLElement)
    // Previous return: ()
    todo ! ()
}

/// C: mjXURDF::Clear (xml/xml_urdf.h:61)
#[allow(unused_variables, non_snake_case)]
pub fn mj_xurdf_clear() {
    todo ! ()
}

