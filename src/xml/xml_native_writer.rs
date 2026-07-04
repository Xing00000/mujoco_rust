//! Port of: xml/xml_native_writer.cc
//! IR hash: 1b139f44af8230f9
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mj_XMLPrinter::PrintSpace (xml/xml_native_writer.cc:59)
#[allow(unused_variables, non_snake_case)]
pub fn mj_xml_printer_print_space(depth: i32) {
    todo!() // mj_XMLPrinter::PrintSpace
}

/// C: WriteDoc (xml/xml_native_writer.cc:68)
#[allow(unused_variables, non_snake_case)]
pub fn write_doc(doc: *mut XMLDocument, error: *mut i8, error_sz: i32) -> i32 {
    todo!() // WriteDoc
}

/// C: mjXWriter::Write (xml/xml_native_writer.cc:961)
/// Calls: mjCModel::Default, mjCModel::GetWorld, mjCModel::IsCompiled, mjXWriter::Actuator, mjXWriter::Asset, mjXWriter::Body, mjXWriter::Compiler, mjXWriter::Contact, mjXWriter::Custom, mjXWriter::Default, mjXWriter::Deformable, mjXWriter::Equality, mjXWriter::Extension, mjXWriter::InsertEnd, mjXWriter::Keyframe, mjXWriter::Option, mjXWriter::Sensor, mjXWriter::Size, mjXWriter::Statistic, mjXWriter::Tendon, mjXWriter::Visual
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_write(error: *mut i8, error_sz: i32) -> i32 {
    todo!() // mjXWriter::Write
}

/// C: mjXWriter::SetModel (xml/xml_native_writer.h:32)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_set_model(_spec: *mut mjSpec, m: *const mjModel) {
    todo!() // mjXWriter::SetModel
}

/// C: mjXWriter::InsertEnd (xml/xml_native_writer.h:39)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_insert_end(parent: *mut tinyxml2__XMLElement, name: *const i8) -> *mut tinyxml2__XMLElement {
    todo!() // mjXWriter::InsertEnd
}

/// C: mjXWriter::Compiler (xml/xml_native_writer.h:45)
/// Calls: mjXWriter::InsertEnd
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_compiler(root: *mut tinyxml2__XMLElement) {
    todo!() // mjXWriter::Compiler
}

/// C: mjXWriter::Option (xml/xml_native_writer.h:46)
/// Calls: mjXWriter::InsertEnd, mj_defaultOption
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_option(root: *mut tinyxml2__XMLElement) {
    todo!() // mjXWriter::Option
}

/// C: mjXWriter::Size (xml/xml_native_writer.h:47)
/// Calls: mjXWriter::InsertEnd
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_size(root: *mut tinyxml2__XMLElement) {
    todo!() // mjXWriter::Size
}

/// C: mjXWriter::Visual (xml/xml_native_writer.h:48)
/// Calls: mjXWriter::InsertEnd, mj_defaultVisual
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_visual(root: *mut tinyxml2__XMLElement) {
    todo!() // mjXWriter::Visual
}

/// C: mjXWriter::Statistic (xml/xml_native_writer.h:49)
/// Calls: mjXWriter::InsertEnd, mjuu_defined
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_statistic(root: *mut tinyxml2__XMLElement) {
    todo!() // mjXWriter::Statistic
}

/// C: mjXWriter::Default (xml/xml_native_writer.h:50)
/// Calls: mjCDef::Actuator, mjCDef::Camera, mjCDef::Equality, mjCDef::Geom, mjCDef::Joint, mjCDef::Light, mjCDef::Material, mjCDef::Mesh, mjCDef::Pair, mjCDef::Site, mjCDef::Tendon, mjXWriter::InsertEnd, mjXWriter::OneActuator, mjXWriter::OneCamera, mjXWriter::OneEquality, mjXWriter::OneGeom, mjXWriter::OneJoint, mjXWriter::OneLight, mjXWriter::OneMaterial, mjXWriter::OneMesh, mjXWriter::OnePair, mjXWriter::OneSite, mjXWriter::OneTendon
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_default(root: *mut tinyxml2__XMLElement, def: *mut mjCDef) {
    todo!() // mjXWriter::Default
}

/// C: mjXWriter::Extension (xml/xml_native_writer.h:51)
/// Calls: mjXWriter::InsertEnd, mjp_getPluginAtSlot
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_extension(root: *mut tinyxml2__XMLElement) {
    todo!() // mjXWriter::Extension
}

/// C: mjXWriter::Custom (xml/xml_native_writer.h:52)
/// Calls: mjCModel::GetObject, mjCModel::NumObjects, mjXWriter::InsertEnd, mju_type2Str
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_custom(root: *mut tinyxml2__XMLElement) {
    todo!() // mjXWriter::Custom
}

/// C: mjXWriter::Asset (xml/xml_native_writer.h:53)
/// Calls: mjCMesh::Plugin, mjCModel::GetObject, mjCModel::NumObjects, mjXWriter::InsertEnd, mjXWriter::OneMaterial, mjXWriter::OneMesh, mjXWriter::OnePlugin
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_asset(root: *mut tinyxml2__XMLElement) {
    todo!() // mjXWriter::Asset
}

/// C: mjXWriter::Contact (xml/xml_native_writer.h:54)
/// Calls: mjCModel::GetObject, mjCModel::NumObjects, mjXWriter::InsertEnd, mjXWriter::OnePair
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_contact(root: *mut tinyxml2__XMLElement) {
    todo!() // mjXWriter::Contact
}

/// C: mjXWriter::Deformable (xml/xml_native_writer.h:55)
/// Calls: mjCModel::GetObject, mjCModel::NumObjects, mjXWriter::InsertEnd, mjXWriter::OneFlex, mjXWriter::OneSkin
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_deformable(root: *mut tinyxml2__XMLElement) {
    todo!() // mjXWriter::Deformable
}

/// C: mjXWriter::Equality (xml/xml_native_writer.h:56)
/// Calls: mjCModel::GetObject, mjCModel::NumObjects, mjXWriter::InsertEnd, mjXWriter::OneEquality
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_equality(root: *mut tinyxml2__XMLElement) {
    todo!() // mjXWriter::Equality
}

/// C: mjXWriter::Tendon (xml/xml_native_writer.h:57)
/// Calls: mjCModel::GetObject, mjCModel::NumObjects, mjCTendon::GetWrap, mjCTendon::NumWraps, mjCWrap::Type, mjXWriter::InsertEnd, mjXWriter::OneTendon
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_tendon(root: *mut tinyxml2__XMLElement) {
    todo!() // mjXWriter::Tendon
}

/// C: mjXWriter::Actuator (xml/xml_native_writer.h:58)
/// Calls: mjCModel::GetObject, mjCModel::NumObjects, mjXWriter::InsertEnd, mjXWriter::OneActuator
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_actuator(root: *mut tinyxml2__XMLElement) {
    todo!() // mjXWriter::Actuator
}

/// C: mjXWriter::Sensor (xml/xml_native_writer.h:59)
/// Calls: mjCModel::NumObjects, mjXWriter::InsertEnd, mjXWriter::OnePlugin, mju_condataSize, mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_sensor(root: *mut tinyxml2__XMLElement) {
    todo!() // mjXWriter::Sensor
}

/// C: mjXWriter::Keyframe (xml/xml_native_writer.h:60)
/// Calls: mjXWriter::InsertEnd
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_keyframe(root: *mut tinyxml2__XMLElement) {
    todo!() // mjXWriter::Keyframe
}

/// C: mjXWriter::Body (xml/xml_native_writer.h:63)
/// Calls: mjCModel::GetWorld, mjXWriter::InsertEnd, mjXWriter::OneCamera, mjXWriter::OneFrame, mjXWriter::OneGeom, mjXWriter::OneJoint, mjXWriter::OneLight, mjXWriter::OnePlugin, mjXWriter::OneSite
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_body(elem: *mut tinyxml2__XMLElement, body: *mut mjCBody, frame: *mut mjCFrame, childclass: std__string_view) {
    todo!() // mjXWriter::Body
}

/// C: mjXWriter::OneFlex (xml/xml_native_writer.h:66)
/// Calls: mjXWriter::InsertEnd
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_one_flex(elem: *mut tinyxml2__XMLElement, pflex: *const mjCFlex) {
    todo!() // mjXWriter::OneFlex
}

/// C: mjXWriter::OneMesh (xml/xml_native_writer.h:67)
/// Calls: mjCDef::Mesh, mjCMesh::Inertia, mjCMesh::Refpos, mjCMesh::Refquat, mjCMesh::Scale, mjCMesh::SmoothNormal
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_one_mesh(elem: *mut tinyxml2__XMLElement, pmesh: *const mjCMesh, def: *mut mjCDef) {
    todo!() // mjXWriter::OneMesh
}

/// C: mjXWriter::OneSkin (xml/xml_native_writer.h:68)
/// Calls: mjCDef::Geom, mjXWriter::InsertEnd
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_one_skin(elem: *mut tinyxml2__XMLElement, pskin: *const mjCSkin) {
    todo!() // mjXWriter::OneSkin
}

/// C: mjXWriter::OneMaterial (xml/xml_native_writer.h:69)
/// Calls: mjCDef::Material, mjXWriter::InsertEnd
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_one_material(elem: *mut tinyxml2__XMLElement, pmaterial: *const mjCMaterial, def: *mut mjCDef) {
    todo!() // mjXWriter::OneMaterial
}

/// C: mjXWriter::OneJoint (xml/xml_native_writer.h:70)
/// Calls: mjCDef::Joint
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_one_joint(elem: *mut tinyxml2__XMLElement, pjoint: *const mjCJoint, def: *mut mjCDef, classname: std__string_view) {
    todo!() // mjXWriter::OneJoint
}

/// C: mjXWriter::OneGeom (xml/xml_native_writer.h:72)
/// Calls: mjCDef::Geom, mjCGeom::GetVolume, mjCMesh::GetPosPtr, mjCMesh::GetQuatPtr, mjXWriter::InsertEnd, mjXWriter::OnePlugin, mjuu_frameaccuminv
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_one_geom(elem: *mut tinyxml2__XMLElement, pgeom: *const mjCGeom, def: *mut mjCDef, classname: std__string_view) {
    todo!() // mjXWriter::OneGeom
}

/// C: mjXWriter::OneSite (xml/xml_native_writer.h:74)
/// Calls: mjCDef::Site
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_one_site(elem: *mut tinyxml2__XMLElement, psite: *const mjCSite, def: *mut mjCDef, classname: std__string_view) {
    todo!() // mjXWriter::OneSite
}

/// C: mjXWriter::OneCamera (xml/xml_native_writer.h:76)
/// Calls: mjCDef::Camera
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_one_camera(elem: *mut tinyxml2__XMLElement, pcamera: *const mjCCamera, def: *mut mjCDef, classname: std__string_view) {
    todo!() // mjXWriter::OneCamera
}

/// C: mjXWriter::OneLight (xml/xml_native_writer.h:78)
/// Calls: mjCDef::Light
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_one_light(elem: *mut tinyxml2__XMLElement, plight: *const mjCLight, def: *mut mjCDef, classname: std__string_view) {
    todo!() // mjXWriter::OneLight
}

/// C: mjXWriter::OnePair (xml/xml_native_writer.h:80)
/// Calls: mjCDef::Pair
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_one_pair(elem: *mut tinyxml2__XMLElement, ppair: *const mjCPair, def: *mut mjCDef) {
    todo!() // mjXWriter::OnePair
}

/// C: mjXWriter::OneEquality (xml/xml_native_writer.h:81)
/// Calls: mjCDef::Equality, mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_one_equality(elem: *mut tinyxml2__XMLElement, pequality: *const mjCEquality, def: *mut mjCDef) {
    todo!() // mjXWriter::OneEquality
}

/// C: mjXWriter::OneTendon (xml/xml_native_writer.h:82)
/// Calls: mjCDef::Tendon, mjCTendon::GetWrap, mjCWrap::Type
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_one_tendon(elem: *mut tinyxml2__XMLElement, ptendon: *const mjCTendon, def: *mut mjCDef) {
    todo!() // mjXWriter::OneTendon
}

/// C: mjXWriter::OneActuator (xml/xml_native_writer.h:83)
/// Calls: mjCDef::Actuator, mjXWriter::OnePlugin
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_one_actuator(elem: *mut tinyxml2__XMLElement, pactuator: *const mjCActuator, def: *mut mjCDef) {
    todo!() // mjXWriter::OneActuator
}

/// C: mjXWriter::OnePlugin (xml/xml_native_writer.h:84)
/// Calls: mjXWriter::InsertEnd
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_one_plugin(elem: *mut tinyxml2__XMLElement, plugin: *const mjsPlugin) {
    todo!() // mjXWriter::OnePlugin
}

/// C: mjXWriter::OneFrame (xml/xml_native_writer.h:85)
/// Calls: mjXWriter::InsertEnd
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_one_frame(elem: *mut tinyxml2__XMLElement, frame: *mut mjCFrame) -> *mut tinyxml2__XMLElement {
    todo!() // mjXWriter::OneFrame
}

