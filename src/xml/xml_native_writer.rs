//! Port of: xml/xml_native_writer.cc
//! IR hash: adc2f24e872d94f7
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mj_XMLPrinter::PrintSpace (xml/xml_native_writer.cc:59)
#[allow(unused_variables, non_snake_case)]
pub fn mj_xml_printer_print_space(self_ptr: *mut mj_XMLPrinter, depth: i32) {
    todo!() // mj_XMLPrinter::PrintSpace
}

/// C: WriteDoc (xml/xml_native_writer.cc:68)
/// Calls: mjCopyError
#[allow(unused_variables, non_snake_case)]
pub fn write_doc(doc: *mut anonymous_namespace___XMLDocument, error: *mut i8, error_sz: usize) -> anonymous_namespace___string {
    todo!() // WriteDoc
}

/// C: mjXWriter::SetModel (xml/xml_native_writer.h:32)
/// Calls: mj_copyBack
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_set_model(self_ptr: *mut mjXWriter, _spec: *mut mjSpec, m: *const mjModel) {
    todo!() // mjXWriter::SetModel
}

/// C: mjXWriter::Write (xml/xml_native_writer.h:35)
/// Calls: WriteDoc, mjCModel::Default, mjCModel::GetWorld, mjCModel::IsCompiled, mjCopyError, mjXWriter::Actuator, mjXWriter::Asset, mjXWriter::Body, mjXWriter::Compiler, mjXWriter::Contact, mjXWriter::Custom, mjXWriter::Default, mjXWriter::Deformable, mjXWriter::Equality, mjXWriter::Extension, mjXWriter::InsertEnd, mjXWriter::Keyframe, mjXWriter::Option, mjXWriter::Sensor, mjXWriter::Size, mjXWriter::Statistic, mjXWriter::Tendon, mjXWriter::Visual, mjs_getString
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_write(self_ptr: *mut mjXWriter, error: *mut i8, error_sz: u64) -> std__string {
    todo!() // mjXWriter::Write
}

/// C: mjXWriter::InsertEnd (xml/xml_native_writer.h:39)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_insert_end(self_ptr: *mut mjXWriter, parent: *mut tinyxml2__XMLElement, name: *const i8) -> *mut tinyxml2__XMLElement {
    todo!() // mjXWriter::InsertEnd
}

/// C: mjXWriter::Compiler (xml/xml_native_writer.h:45)
/// Calls: mjCModel::get_meshdir, mjCModel::get_texturedir, mjXUtil::WriteAttr, mjXUtil::WriteAttrKey, mjXUtil::WriteAttrTxt, mjXWriter::InsertEnd
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_compiler(self_ptr: *mut mjXWriter, root: *mut tinyxml2__XMLElement) {
    todo!() // mjXWriter::Compiler
}

/// C: mjXWriter::Option (xml/xml_native_writer.h:46)
/// Calls: mjXUtil::WriteAttr, mjXUtil::WriteAttrInt, mjXUtil::WriteAttrKey, mjXWriter::InsertEnd, mj_defaultOption
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_option(self_ptr: *mut mjXWriter, root: *mut tinyxml2__XMLElement) {
    todo!() // mjXWriter::Option
}

/// C: mjXWriter::Size (xml/xml_native_writer.h:47)
/// Calls: mjXUtil::WriteAttrInt, mjXUtil::WriteAttrTxt, mjXWriter::InsertEnd, mju_writeNumBytes
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_size(self_ptr: *mut mjXWriter, root: *mut tinyxml2__XMLElement) {
    todo!() // mjXWriter::Size
}

/// C: mjXWriter::Visual (xml/xml_native_writer.h:48)
/// Calls: mjXUtil::WriteAttr, mjXUtil::WriteAttrInt, mjXUtil::WriteAttrKey, mjXWriter::InsertEnd, mj_defaultVisual
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_visual(self_ptr: *mut mjXWriter, root: *mut tinyxml2__XMLElement) {
    todo!() // mjXWriter::Visual
}

/// C: mjXWriter::Statistic (xml/xml_native_writer.h:49)
/// Calls: mjXUtil::WriteAttr, mjXWriter::InsertEnd, mjuu_defined
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_statistic(self_ptr: *mut mjXWriter, root: *mut tinyxml2__XMLElement) {
    todo!() // mjXWriter::Statistic
}

/// C: mjXWriter::Default (xml/xml_native_writer.h:50)
/// Calls: mjCDef::Actuator, mjCDef::Camera, mjCDef::Equality, mjCDef::Geom, mjCDef::Joint, mjCDef::Light, mjCDef::Material, mjCDef::Mesh, mjCDef::Pair, mjCDef::Site, mjCDef::Tendon, mjXUtil::WriteAttrTxt, mjXWriter::InsertEnd, mjXWriter::OneActuator, mjXWriter::OneCamera, mjXWriter::OneEquality, mjXWriter::OneGeom, mjXWriter::OneJoint, mjXWriter::OneLight, mjXWriter::OneMaterial, mjXWriter::OneMesh, mjXWriter::OnePair, mjXWriter::OneSite, mjXWriter::OneTendon
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_default(self_ptr: *mut mjXWriter, root: *mut tinyxml2__XMLElement, def: *mut mjCDef) {
    todo!() // mjXWriter::Default
}

/// C: mjXWriter::Extension (xml/xml_native_writer.h:51)
/// Calls: mjCModel::ActivePlugins, mjCModel::GetObject, mjCModel::Plugins, mjXUtil::WriteAttrTxt, mjXWriter::InsertEnd, mjp_getPluginAtSlot
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_extension(self_ptr: *mut mjXWriter, root: *mut tinyxml2__XMLElement) {
    todo!() // mjXWriter::Extension
}

/// C: mjXWriter::Custom (xml/xml_native_writer.h:52)
/// Calls: mjCModel::GetObject, mjCModel::NumObjects, mjXUtil::WriteAttr, mjXUtil::WriteAttrInt, mjXUtil::WriteAttrTxt, mjXWriter::InsertEnd, mju_type2Str
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_custom(self_ptr: *mut mjXWriter, root: *mut tinyxml2__XMLElement) {
    todo!() // mjXWriter::Custom
}

/// C: mjXWriter::Asset (xml/xml_native_writer.h:53)
/// Calls: mjCHField::get_userdata, mjCMesh::File, mjCMesh::Plugin, mjCModel::GetObject, mjCModel::NumObjects, mjCTexture::File, mjCTexture::get_content_type, mjCTexture::get_cubefiles, mjXUtil::Vector2String, mjXUtil::WriteAttr, mjXUtil::WriteAttrInt, mjXUtil::WriteAttrKey, mjXUtil::WriteAttrTxt, mjXWriter::InsertEnd, mjXWriter::OneMaterial, mjXWriter::OneMesh, mjXWriter::OnePlugin
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_asset(self_ptr: *mut mjXWriter, root: *mut tinyxml2__XMLElement) {
    todo!() // mjXWriter::Asset
}

/// C: mjXWriter::Contact (xml/xml_native_writer.h:54)
/// Calls: mjCBodyPair::get_bodyname1, mjCBodyPair::get_bodyname2, mjCModel::GetObject, mjCModel::NumObjects, mjXUtil::WriteAttrTxt, mjXWriter::InsertEnd, mjXWriter::OnePair
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_contact(self_ptr: *mut mjXWriter, root: *mut tinyxml2__XMLElement) {
    todo!() // mjXWriter::Contact
}

/// C: mjXWriter::Deformable (xml/xml_native_writer.h:55)
/// Calls: mjCModel::GetObject, mjCModel::NumObjects, mjXWriter::InsertEnd, mjXWriter::OneFlex, mjXWriter::OneSkin
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_deformable(self_ptr: *mut mjXWriter, root: *mut tinyxml2__XMLElement) {
    todo!() // mjXWriter::Deformable
}

/// C: mjXWriter::Equality (xml/xml_native_writer.h:56)
/// Calls: mjCModel::GetObject, mjCModel::NumObjects, mjXUtil::FindValue, mjXWriter::InsertEnd, mjXWriter::OneEquality
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_equality(self_ptr: *mut mjXWriter, root: *mut tinyxml2__XMLElement) {
    todo!() // mjXWriter::Equality
}

/// C: mjXWriter::Tendon (xml/xml_native_writer.h:57)
/// Calls: mjCModel::GetObject, mjCModel::NumObjects, mjCTendon::GetWrap, mjCTendon::NumWraps, mjCWrap::Type, mjXUtil::WriteAttr, mjXUtil::WriteAttrTxt, mjXWriter::InsertEnd, mjXWriter::OneTendon
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_tendon(self_ptr: *mut mjXWriter, root: *mut tinyxml2__XMLElement) {
    todo!() // mjXWriter::Tendon
}

/// C: mjXWriter::Actuator (xml/xml_native_writer.h:58)
/// Calls: mjCModel::GetObject, mjCModel::NumObjects, mjXWriter::InsertEnd, mjXWriter::OneActuator
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_actuator(self_ptr: *mut mjXWriter, root: *mut tinyxml2__XMLElement) {
    todo!() // mjXWriter::Actuator
}

/// C: mjXWriter::Sensor (xml/xml_native_writer.h:59)
/// Calls: mjCModel::NumObjects, mjCModel::Sensors, mjCSensor::get_objname, mjCSensor::get_refname, mjCSensor::get_userdata, mjXUtil::WriteAttr, mjXUtil::WriteAttrInt, mjXUtil::WriteAttrKey, mjXUtil::WriteAttrKeys, mjXUtil::WriteAttrTxt, mjXUtil::WriteVector, mjXWriter::InsertEnd, mjXWriter::OnePlugin, mju_condataSize, mju_error, mju_type2Str
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_sensor(self_ptr: *mut mjXWriter, root: *mut tinyxml2__XMLElement) {
    todo!() // mjXWriter::Sensor
}

/// C: mjXWriter::Keyframe (xml/xml_native_writer.h:60)
/// Calls: mjCModel::Bodies, mjCModel::Keys, mjXUtil::WriteAttr, mjXUtil::WriteAttrTxt, mjXWriter::InsertEnd
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_keyframe(self_ptr: *mut mjXWriter, root: *mut tinyxml2__XMLElement) {
    todo!() // mjXWriter::Keyframe
}

/// C: mjXWriter::Body (xml/xml_native_writer.h:63)
/// Calls: mjCBody::get_userdata, mjCModel::GetWorld, mjXUtil::WriteAttr, mjXUtil::WriteAttrKey, mjXUtil::WriteAttrTxt, mjXUtil::WriteVector, mjXWriter::InsertEnd, mjXWriter::OneCamera, mjXWriter::OneFrame, mjXWriter::OneGeom, mjXWriter::OneJoint, mjXWriter::OneLight, mjXWriter::OnePlugin, mjXWriter::OneSite
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_body(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, body: *mut mjCBody, frame: *mut mjCFrame, childclass: std__string_view) {
    todo!() // mjXWriter::Body
}

/// C: mjXWriter::OneFlex (xml/xml_native_writer.h:66)
/// Calls: VectorToString, mjCFlex::get_elem, mjCFlex::get_elemtexcoord, mjCFlex::get_material, mjCFlex::get_nodebody, mjCFlex::get_texcoord, mjCFlex::get_vert, mjCFlex::get_vertbody, mjXUtil::WriteAttr, mjXUtil::WriteAttrInt, mjXUtil::WriteAttrKey, mjXUtil::WriteAttrTxt, mjXWriter::InsertEnd
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_one_flex(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, pflex: *const mjCFlex) {
    todo!() // mjXWriter::OneFlex
}

/// C: mjXWriter::OneMesh (xml/xml_native_writer.h:67)
/// Calls: VectorToString, mjCDef::Mesh, mjCMesh::ContentType, mjCMesh::File, mjCMesh::Inertia, mjCMesh::Refpos, mjCMesh::Refquat, mjCMesh::Scale, mjCMesh::SmoothNormal, mjCMesh::UserFace, mjCMesh::UserNormal, mjCMesh::UserTexcoord, mjCMesh::UserVert, mjXUtil::FindValue, mjXUtil::WriteAttr, mjXUtil::WriteAttrKey, mjXUtil::WriteAttrTxt
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_one_mesh(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, pmesh: *const mjCMesh, def: *mut mjCDef) {
    todo!() // mjXWriter::OneMesh
}

/// C: mjXWriter::OneSkin (xml/xml_native_writer.h:68)
/// Calls: VectorToString, mjCDef::Geom, mjCSkin::File, mjCSkin::get_bindpos, mjCSkin::get_bindquat, mjCSkin::get_bodyname, mjCSkin::get_face, mjCSkin::get_material, mjCSkin::get_texcoord, mjCSkin::get_vert, mjCSkin::get_vertid, mjCSkin::get_vertweight, mjXUtil::WriteAttr, mjXUtil::WriteAttrInt, mjXUtil::WriteAttrTxt, mjXWriter::InsertEnd
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_one_skin(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, pskin: *const mjCSkin) {
    todo!() // mjXWriter::OneSkin
}

/// C: mjXWriter::OneMaterial (xml/xml_native_writer.h:69)
/// Calls: mjCDef::Material, mjCMaterial::get_texture, mjXUtil::FindValue, mjXUtil::WriteAttr, mjXUtil::WriteAttrKey, mjXUtil::WriteAttrTxt, mjXWriter::InsertEnd
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_one_material(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, pmaterial: *const mjCMaterial, def: *mut mjCDef) {
    todo!() // mjXWriter::OneMaterial
}

/// C: mjXWriter::OneJoint (xml/xml_native_writer.h:70)
/// Calls: mjCDef::Joint, mjCJoint::get_userdata, mjXUtil::FindValue, mjXUtil::WriteAttr, mjXUtil::WriteAttrInt, mjXUtil::WriteAttrKey, mjXUtil::WriteAttrTxt, mjXUtil::WriteVector
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_one_joint(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, pjoint: *const mjCJoint, def: *mut mjCDef, classname: std__string_view) {
    todo!() // mjXWriter::OneJoint
}

/// C: mjXWriter::OneGeom (xml/xml_native_writer.h:72)
/// Calls: mjCDef::Geom, mjCGeom::GetVolume, mjCGeom::get_hfieldname, mjCGeom::get_material, mjCGeom::get_meshname, mjCGeom::get_userdata, mjCMesh::GetPosPtr, mjCMesh::GetQuatPtr, mjXUtil::SameVector, mjXUtil::WriteAttr, mjXUtil::WriteAttrInt, mjXUtil::WriteAttrKey, mjXUtil::WriteAttrTxt, mjXUtil::WriteVector, mjXWriter::InsertEnd, mjXWriter::OnePlugin, mjuu_copyvec, mjuu_defined, mjuu_frameaccuminv
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_one_geom(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, pgeom: *const mjCGeom, def: *mut mjCDef, classname: std__string_view) {
    todo!() // mjXWriter::OneGeom
}

/// C: mjXWriter::OneSite (xml/xml_native_writer.h:74)
/// Calls: mjCDef::Site, mjCSite::get_material, mjCSite::get_userdata, mjXUtil::WriteAttr, mjXUtil::WriteAttrInt, mjXUtil::WriteAttrKey, mjXUtil::WriteAttrTxt, mjXUtil::WriteVector
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_one_site(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, psite: *const mjCSite, def: *mut mjCDef, classname: std__string_view) {
    todo!() // mjXWriter::OneSite
}

/// C: mjXWriter::OneCamera (xml/xml_native_writer.h:76)
/// Calls: mjCCamera::get_targetbody, mjCCamera::get_userdata, mjCDef::Camera, mjXUtil::WriteAttr, mjXUtil::WriteAttrKey, mjXUtil::WriteAttrKeys, mjXUtil::WriteAttrTxt, mjXUtil::WriteVector
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_one_camera(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, pcamera: *const mjCCamera, def: *mut mjCDef, classname: std__string_view) {
    todo!() // mjXWriter::OneCamera
}

/// C: mjXWriter::OneLight (xml/xml_native_writer.h:78)
/// Calls: mjCDef::Light, mjCLight::get_targetbody, mjCLight::get_texture, mjXUtil::WriteAttr, mjXUtil::WriteAttrKey, mjXUtil::WriteAttrTxt
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_one_light(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, plight: *const mjCLight, def: *mut mjCDef, classname: std__string_view) {
    todo!() // mjXWriter::OneLight
}

/// C: mjXWriter::OnePair (xml/xml_native_writer.h:80)
/// Calls: mjCDef::Pair, mjCPair::get_geomname1, mjCPair::get_geomname2, mjXUtil::WriteAttr, mjXUtil::WriteAttrInt, mjXUtil::WriteAttrTxt
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_one_pair(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, ppair: *const mjCPair, def: *mut mjCDef) {
    todo!() // mjXWriter::OnePair
}

/// C: mjXWriter::OneEquality (xml/xml_native_writer.h:81)
/// Calls: mjCDef::Equality, mjXUtil::WriteAttr, mjXUtil::WriteAttrKey, mjXUtil::WriteAttrTxt, mjs_getString, mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_one_equality(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, pequality: *const mjCEquality, def: *mut mjCDef) {
    todo!() // mjXWriter::OneEquality
}

/// C: mjXWriter::OneTendon (xml/xml_native_writer.h:82)
/// Calls: mjCDef::Tendon, mjCTendon::GetWrap, mjCTendon::get_material, mjCTendon::get_userdata, mjCWrap::Type, mjXUtil::WriteAttr, mjXUtil::WriteAttrInt, mjXUtil::WriteAttrKey, mjXUtil::WriteAttrTxt, mjXUtil::WriteVector
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_one_tendon(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, ptendon: *const mjCTendon, def: *mut mjCDef) {
    todo!() // mjXWriter::OneTendon
}

/// C: mjXWriter::OneActuator (xml/xml_native_writer.h:83)
/// Calls: mjCActuator::get_refsite, mjCActuator::get_slidersite, mjCActuator::get_target, mjCActuator::get_userdata, mjCDef::Actuator, mjXUtil::WriteAttr, mjXUtil::WriteAttrInt, mjXUtil::WriteAttrKey, mjXUtil::WriteAttrTxt, mjXUtil::WriteVector, mjXWriter::OnePlugin
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_one_actuator(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, pactuator: *const mjCActuator, def: *mut mjCDef) {
    todo!() // mjXWriter::OneActuator
}

/// C: mjXWriter::OnePlugin (xml/xml_native_writer.h:84)
/// Calls: mjXUtil::WriteAttrTxt, mjXWriter::InsertEnd, mjp_getPluginAtSlot, mjs_getString
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_one_plugin(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, plugin: *const mjsPlugin) {
    todo!() // mjXWriter::OnePlugin
}

/// C: mjXWriter::OneFrame (xml/xml_native_writer.h:85)
/// Calls: mjXUtil::WriteAttrTxt, mjXWriter::InsertEnd
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_one_frame(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, frame: *mut mjCFrame) -> *mut tinyxml2__XMLElement {
    todo!() // mjXWriter::OneFrame
}

