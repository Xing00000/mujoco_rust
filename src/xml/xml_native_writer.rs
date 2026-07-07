//! Port of: xml/xml_native_writer.cc
//! IR hash: 05737965add36adb
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mj_XMLPrinter::PrintSpace (xml/xml_native_writer.cc:59)
#[allow(unused_variables, non_snake_case)]
pub fn mj_xml_printer_print_space(self_ptr: *mut mj_XMLPrinter, depth: i32) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mj_XMLPrinter, depth : i32)
    // Previous return: ()
    todo ! ()
}

/// C: WriteDoc (xml/xml_native_writer.cc:68)
/// Calls: mjCopyError
#[allow(unused_variables, non_snake_case)]
pub fn write_doc(doc: *mut XMLDocument, error: *mut i8, error_sz: usize) -> string {
    // WARNING: signature changed — verify body
    // Previous params: (doc : * mut XMLDocument, error : * mut i8, error_sz : usize)
    // Previous return: string
    todo ! ()
}

/// C: mjXWriter::SetModel (xml/xml_native_writer.h:32)
/// Calls: mj_copyBack
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_set_model(self_ptr: *mut mjXWriter, _spec: *mut mjSpec, m: *const mjModel) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjXWriter, _spec : * mut mjSpec, m : * const mjModel)
    // Previous return: ()
    todo ! ()
}

/// C: mjXWriter::Write (xml/xml_native_writer.h:35)
/// Calls: mjCModel::Default, mjCModel::GetWorld, mjCModel::IsCompiled, mjCopyError, mjXWriter::Actuator, mjXWriter::Asset, mjXWriter::Body, mjXWriter::Compiler, mjXWriter::Contact, mjXWriter::Custom, mjXWriter::Default, mjXWriter::Deformable, mjXWriter::Equality, mjXWriter::Extension, mjXWriter::InsertEnd, mjXWriter::Keyframe, mjXWriter::Option, mjXWriter::Sensor, mjXWriter::Size, mjXWriter::Statistic, mjXWriter::Tendon, mjXWriter::Visual
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_write(self_ptr: *mut mjXWriter, error: *mut i8, error_sz: usize) -> std__string {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjXWriter, error : * mut i8, error_sz : usize)
    // Previous return: std__string
    todo ! ()
}

/// C: mjXWriter::InsertEnd (xml/xml_native_writer.h:39)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_insert_end(self_ptr: *mut mjXWriter, parent: *mut tinyxml2__XMLElement, name: *const i8) -> *mut tinyxml2__XMLElement {
    extern "C" { fn mjXWriter_InsertEnd_impl(self_ptr: *mut mjXWriter, parent: *mut tinyxml2__XMLElement, name: *const i8) -> *mut tinyxml2__XMLElement; }
    // SAFETY: delegates to C++ implementation, all pointers valid per caller contract
    unsafe { mjXWriter_InsertEnd_impl(self_ptr, parent, name) }
}

/// C: mjXWriter::Compiler (xml/xml_native_writer.h:45)
/// Calls: mjXUtil::WriteAttrKey, mjXUtil::WriteAttrTxt, mjXWriter::InsertEnd
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_compiler(self_ptr: *mut mjXWriter, root: *mut tinyxml2__XMLElement) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjXWriter, root : * mut tinyxml2__XMLElement)
    // Previous return: ()
    todo ! ()
}

/// C: mjXWriter::Option (xml/xml_native_writer.h:46)
/// Calls: mjXUtil::WriteAttrInt, mjXUtil::WriteAttrKey, mjXWriter::InsertEnd, mj_defaultOption
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_option(self_ptr: *mut mjXWriter, root: *mut tinyxml2__XMLElement) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjXWriter, root : * mut tinyxml2__XMLElement)
    // Previous return: ()
    todo ! ()
}

/// C: mjXWriter::Size (xml/xml_native_writer.h:47)
/// Calls: mjXUtil::WriteAttrInt, mjXUtil::WriteAttrTxt, mjXWriter::InsertEnd
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_size(self_ptr: *mut mjXWriter, root: *mut tinyxml2__XMLElement) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjXWriter, root : * mut tinyxml2__XMLElement)
    // Previous return: ()
    todo ! ()
}

/// C: mjXWriter::Visual (xml/xml_native_writer.h:48)
/// Calls: mjXWriter::InsertEnd, mj_defaultVisual
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_visual(self_ptr: *mut mjXWriter, root: *mut tinyxml2__XMLElement) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjXWriter, root : * mut tinyxml2__XMLElement)
    // Previous return: ()
    todo ! ()
}

/// C: mjXWriter::Statistic (xml/xml_native_writer.h:49)
/// Calls: mjXWriter::InsertEnd, mjuu_defined
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_statistic(self_ptr: *mut mjXWriter, root: *mut tinyxml2__XMLElement) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjXWriter, root : * mut tinyxml2__XMLElement)
    // Previous return: ()
    todo ! ()
}

/// C: mjXWriter::Default (xml/xml_native_writer.h:50)
/// Calls: mjCDef::Actuator, mjCDef::Camera, mjCDef::Equality, mjCDef::Geom, mjCDef::Joint, mjCDef::Light, mjCDef::Material, mjCDef::Mesh, mjCDef::Pair, mjCDef::Site, mjCDef::Tendon, mjXUtil::WriteAttrTxt, mjXWriter::InsertEnd, mjXWriter::OneActuator, mjXWriter::OneCamera, mjXWriter::OneEquality, mjXWriter::OneGeom, mjXWriter::OneJoint, mjXWriter::OneLight, mjXWriter::OneMaterial, mjXWriter::OneMesh, mjXWriter::OnePair, mjXWriter::OneSite, mjXWriter::OneTendon
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_default(self_ptr: *mut mjXWriter, root: *mut tinyxml2__XMLElement, def: *mut mjCDef) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjXWriter, root : * mut tinyxml2__XMLElement, def : * mut mjCDef)
    // Previous return: ()
    todo ! ()
}

/// C: mjXWriter::Extension (xml/xml_native_writer.h:51)
/// Calls: mjXUtil::WriteAttrTxt, mjXWriter::InsertEnd, mjp_getPluginAtSlot
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_extension(self_ptr: *mut mjXWriter, root: *mut tinyxml2__XMLElement) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjXWriter, root : * mut tinyxml2__XMLElement)
    // Previous return: ()
    todo ! ()
}

/// C: mjXWriter::Custom (xml/xml_native_writer.h:52)
/// Calls: mjCModel::GetObject, mjCModel::NumObjects, mjXUtil::WriteAttrInt, mjXUtil::WriteAttrTxt, mjXWriter::InsertEnd, mju_type2Str
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_custom(self_ptr: *mut mjXWriter, root: *mut tinyxml2__XMLElement) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjXWriter, root : * mut tinyxml2__XMLElement)
    // Previous return: ()
    todo ! ()
}

/// C: mjXWriter::Asset (xml/xml_native_writer.h:53)
/// Calls: mjCMesh::File, mjCMesh::Plugin, mjCModel::GetObject, mjCModel::NumObjects, mjXUtil::WriteAttrInt, mjXUtil::WriteAttrKey, mjXUtil::WriteAttrTxt, mjXWriter::InsertEnd, mjXWriter::OneMaterial, mjXWriter::OneMesh, mjXWriter::OnePlugin
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_asset(self_ptr: *mut mjXWriter, root: *mut tinyxml2__XMLElement) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjXWriter, root : * mut tinyxml2__XMLElement)
    // Previous return: ()
    todo ! ()
}

/// C: mjXWriter::Contact (xml/xml_native_writer.h:54)
/// Calls: mjCModel::GetObject, mjCModel::NumObjects, mjXUtil::WriteAttrTxt, mjXWriter::InsertEnd, mjXWriter::OnePair
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_contact(self_ptr: *mut mjXWriter, root: *mut tinyxml2__XMLElement) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjXWriter, root : * mut tinyxml2__XMLElement)
    // Previous return: ()
    todo ! ()
}

/// C: mjXWriter::Deformable (xml/xml_native_writer.h:55)
/// Calls: mjCModel::GetObject, mjCModel::NumObjects, mjXWriter::InsertEnd, mjXWriter::OneFlex, mjXWriter::OneSkin
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_deformable(self_ptr: *mut mjXWriter, root: *mut tinyxml2__XMLElement) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjXWriter, root : * mut tinyxml2__XMLElement)
    // Previous return: ()
    todo ! ()
}

/// C: mjXWriter::Equality (xml/xml_native_writer.h:56)
/// Calls: mjCModel::GetObject, mjCModel::NumObjects, mjXUtil::FindValue, mjXWriter::InsertEnd, mjXWriter::OneEquality
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_equality(self_ptr: *mut mjXWriter, root: *mut tinyxml2__XMLElement) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjXWriter, root : * mut tinyxml2__XMLElement)
    // Previous return: ()
    todo ! ()
}

/// C: mjXWriter::Tendon (xml/xml_native_writer.h:57)
/// Calls: mjCModel::GetObject, mjCModel::NumObjects, mjCTendon::GetWrap, mjCTendon::NumWraps, mjCWrap::Type, mjXUtil::WriteAttrTxt, mjXWriter::InsertEnd, mjXWriter::OneTendon
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_tendon(self_ptr: *mut mjXWriter, root: *mut tinyxml2__XMLElement) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjXWriter, root : * mut tinyxml2__XMLElement)
    // Previous return: ()
    todo ! ()
}

/// C: mjXWriter::Actuator (xml/xml_native_writer.h:58)
/// Calls: mjCModel::GetObject, mjCModel::NumObjects, mjXWriter::InsertEnd, mjXWriter::OneActuator
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_actuator(self_ptr: *mut mjXWriter, root: *mut tinyxml2__XMLElement) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjXWriter, root : * mut tinyxml2__XMLElement)
    // Previous return: ()
    todo ! ()
}

/// C: mjXWriter::Sensor (xml/xml_native_writer.h:59)
/// Calls: mjCModel::NumObjects, mjCSensor::get_objname, mjCSensor::get_refname, mjXUtil::WriteAttrInt, mjXUtil::WriteAttrKey, mjXUtil::WriteAttrTxt, mjXWriter::InsertEnd, mjXWriter::OnePlugin, mju_condataSize, mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_sensor(self_ptr: *mut mjXWriter, root: *mut tinyxml2__XMLElement) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjXWriter, root : * mut tinyxml2__XMLElement)
    // Previous return: ()
    todo ! ()
}

/// C: mjXWriter::Keyframe (xml/xml_native_writer.h:60)
/// Calls: mjXUtil::WriteAttrTxt, mjXWriter::InsertEnd
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_keyframe(self_ptr: *mut mjXWriter, root: *mut tinyxml2__XMLElement) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjXWriter, root : * mut tinyxml2__XMLElement)
    // Previous return: ()
    todo ! ()
}

/// C: mjXWriter::Body (xml/xml_native_writer.h:63)
/// Calls: mjCModel::GetWorld, mjXUtil::WriteAttrKey, mjXUtil::WriteAttrTxt, mjXWriter::InsertEnd, mjXWriter::OneCamera, mjXWriter::OneFrame, mjXWriter::OneGeom, mjXWriter::OneJoint, mjXWriter::OneLight, mjXWriter::OnePlugin, mjXWriter::OneSite
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_body(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, body: *mut mjCBody, frame: *mut mjCFrame, childclass: string_view) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjXWriter, elem : * mut tinyxml2__XMLElement, body : * mut mjCBody, frame : * mut mjCFrame, childclass : string_view)
    // Previous return: ()
    todo ! ()
}

/// C: mjXWriter::OneFlex (xml/xml_native_writer.h:66)
/// Calls: mjCFlex::get_material, mjXUtil::WriteAttrInt, mjXUtil::WriteAttrKey, mjXUtil::WriteAttrTxt, mjXWriter::InsertEnd
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_one_flex(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, pflex: *const mjCFlex) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjXWriter, elem : * mut tinyxml2__XMLElement, pflex : * const mjCFlex)
    // Previous return: ()
    todo ! ()
}

/// C: mjXWriter::OneMesh (xml/xml_native_writer.h:67)
/// Calls: mjCDef::Mesh, mjCMesh::ContentType, mjCMesh::File, mjCMesh::Inertia, mjCMesh::Refpos, mjCMesh::Refquat, mjCMesh::Scale, mjCMesh::SmoothNormal, mjXUtil::WriteAttrTxt
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_one_mesh(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, pmesh: *const mjCMesh, def: *mut mjCDef) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjXWriter, elem : * mut tinyxml2__XMLElement, pmesh : * const mjCMesh, def : * mut mjCDef)
    // Previous return: ()
    todo ! ()
}

/// C: mjXWriter::OneSkin (xml/xml_native_writer.h:68)
/// Calls: mjCDef::Geom, mjCSkin::File, mjCSkin::get_material, mjXUtil::WriteAttrInt, mjXUtil::WriteAttrTxt, mjXWriter::InsertEnd
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_one_skin(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, pskin: *const mjCSkin) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjXWriter, elem : * mut tinyxml2__XMLElement, pskin : * const mjCSkin)
    // Previous return: ()
    todo ! ()
}

/// C: mjXWriter::OneMaterial (xml/xml_native_writer.h:69)
/// Calls: mjCDef::Material, mjCMaterial::get_texture, mjXUtil::WriteAttrKey, mjXUtil::WriteAttrTxt, mjXWriter::InsertEnd
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_one_material(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, pmaterial: *const mjCMaterial, def: *mut mjCDef) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjXWriter, elem : * mut tinyxml2__XMLElement, pmaterial : * const mjCMaterial, def : * mut mjCDef)
    // Previous return: ()
    todo ! ()
}

/// C: mjXWriter::OneJoint (xml/xml_native_writer.h:70)
/// Calls: mjCDef::Joint, mjXUtil::FindValue, mjXUtil::WriteAttrInt, mjXUtil::WriteAttrKey, mjXUtil::WriteAttrTxt
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_one_joint(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, pjoint: *const mjCJoint, def: *mut mjCDef, classname: string_view) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjXWriter, elem : * mut tinyxml2__XMLElement, pjoint : * const mjCJoint, def : * mut mjCDef, classname : string_view)
    // Previous return: ()
    todo ! ()
}

/// C: mjXWriter::OneGeom (xml/xml_native_writer.h:72)
/// Calls: mjCDef::Geom, mjCGeom::GetVolume, mjCGeom::get_hfieldname, mjCGeom::get_material, mjCGeom::get_meshname, mjCMesh::GetPosPtr, mjCMesh::GetQuatPtr, mjXUtil::WriteAttrInt, mjXUtil::WriteAttrKey, mjXUtil::WriteAttrTxt, mjXWriter::InsertEnd, mjXWriter::OnePlugin, mjuu_frameaccuminv
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_one_geom(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, pgeom: *const mjCGeom, def: *mut mjCDef, classname: string_view) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjXWriter, elem : * mut tinyxml2__XMLElement, pgeom : * const mjCGeom, def : * mut mjCDef, classname : string_view)
    // Previous return: ()
    todo ! ()
}

/// C: mjXWriter::OneSite (xml/xml_native_writer.h:74)
/// Calls: mjCDef::Site, mjCSite::get_material, mjXUtil::WriteAttrInt, mjXUtil::WriteAttrKey, mjXUtil::WriteAttrTxt
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_one_site(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, psite: *const mjCSite, def: *mut mjCDef, classname: string_view) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjXWriter, elem : * mut tinyxml2__XMLElement, psite : * const mjCSite, def : * mut mjCDef, classname : string_view)
    // Previous return: ()
    todo ! ()
}

/// C: mjXWriter::OneCamera (xml/xml_native_writer.h:76)
/// Calls: mjCCamera::get_targetbody, mjCDef::Camera, mjXUtil::WriteAttrKey, mjXUtil::WriteAttrTxt
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_one_camera(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, pcamera: *const mjCCamera, def: *mut mjCDef, classname: string_view) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjXWriter, elem : * mut tinyxml2__XMLElement, pcamera : * const mjCCamera, def : * mut mjCDef, classname : string_view)
    // Previous return: ()
    todo ! ()
}

/// C: mjXWriter::OneLight (xml/xml_native_writer.h:78)
/// Calls: mjCDef::Light, mjCLight::get_targetbody, mjCLight::get_texture, mjXUtil::WriteAttrKey, mjXUtil::WriteAttrTxt
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_one_light(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, plight: *const mjCLight, def: *mut mjCDef, classname: string_view) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjXWriter, elem : * mut tinyxml2__XMLElement, plight : * const mjCLight, def : * mut mjCDef, classname : string_view)
    // Previous return: ()
    todo ! ()
}

/// C: mjXWriter::OnePair (xml/xml_native_writer.h:80)
/// Calls: mjCDef::Pair, mjCPair::get_geomname1, mjCPair::get_geomname2, mjXUtil::WriteAttrInt, mjXUtil::WriteAttrTxt
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_one_pair(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, ppair: *const mjCPair, def: *mut mjCDef) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjXWriter, elem : * mut tinyxml2__XMLElement, ppair : * const mjCPair, def : * mut mjCDef)
    // Previous return: ()
    todo ! ()
}

/// C: mjXWriter::OneEquality (xml/xml_native_writer.h:81)
/// Calls: mjCDef::Equality, mjXUtil::WriteAttrKey, mjXUtil::WriteAttrTxt, mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_one_equality(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, pequality: *const mjCEquality, def: *mut mjCDef) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjXWriter, elem : * mut tinyxml2__XMLElement, pequality : * const mjCEquality, def : * mut mjCDef)
    // Previous return: ()
    todo ! ()
}

/// C: mjXWriter::OneTendon (xml/xml_native_writer.h:82)
/// Calls: mjCDef::Tendon, mjCTendon::GetWrap, mjCTendon::get_material, mjCWrap::Type, mjXUtil::WriteAttrInt, mjXUtil::WriteAttrKey, mjXUtil::WriteAttrTxt
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_one_tendon(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, ptendon: *const mjCTendon, def: *mut mjCDef) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjXWriter, elem : * mut tinyxml2__XMLElement, ptendon : * const mjCTendon, def : * mut mjCDef)
    // Previous return: ()
    todo ! ()
}

/// C: mjXWriter::OneActuator (xml/xml_native_writer.h:83)
/// Calls: mjCActuator::get_refsite, mjCActuator::get_slidersite, mjCActuator::get_target, mjCDef::Actuator, mjXUtil::WriteAttrInt, mjXUtil::WriteAttrKey, mjXUtil::WriteAttrTxt, mjXWriter::OnePlugin
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_one_actuator(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, pactuator: *const mjCActuator, def: *mut mjCDef) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjXWriter, elem : * mut tinyxml2__XMLElement, pactuator : * const mjCActuator, def : * mut mjCDef)
    // Previous return: ()
    todo ! ()
}

/// C: mjXWriter::OnePlugin (xml/xml_native_writer.h:84)
/// Calls: mjXUtil::WriteAttrTxt, mjXWriter::InsertEnd
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_one_plugin(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, plugin: *const mjsPlugin) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjXWriter, elem : * mut tinyxml2__XMLElement, plugin : * const mjsPlugin)
    // Previous return: ()
    todo ! ()
}

/// C: mjXWriter::OneFrame (xml/xml_native_writer.h:85)
/// Calls: mjXUtil::WriteAttrTxt, mjXWriter::InsertEnd
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_one_frame(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, frame: *mut mjCFrame) -> *mut tinyxml2__XMLElement {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjXWriter, elem : * mut tinyxml2__XMLElement, frame : * mut mjCFrame)
    // Previous return: * mut tinyxml2__XMLElement
    todo ! ()
}

