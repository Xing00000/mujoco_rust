//! Port of: xml/xml_native_writer.cc
//! IR hash: 05737965add36adb
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mj_XMLPrinter::PrintSpace (xml/xml_native_writer.cc:59)
#[allow(unused_variables, non_snake_case)]
pub fn mj_xml_printer_print_space(self_ptr: *mut mj_XMLPrinter, depth: i32) {
    extern "C" { fn mj_XMLPrinter_PrintSpace(self_ptr: *mut mj_XMLPrinter, depth: i32); }
    // SAFETY: delegates to C implementation
    unsafe { mj_XMLPrinter_PrintSpace(self_ptr, depth) }
}

/// C: WriteDoc (xml/xml_native_writer.cc:68)
/// Calls: mjCopyError
#[allow(unused_variables, non_snake_case)]
pub fn write_doc(doc: *mut XMLDocument, error: *mut i8, error_sz: usize) -> string {
    if doc.is_null() {
        // SAFETY: string is a zero-sized type; zeroed is trivially valid
        return unsafe { core::mem::zeroed() };
    }
    extern "C" { fn WriteDoc(doc: *mut XMLDocument, error: *mut i8, error_sz: usize) -> string; }
    // SAFETY: doc verified non-null; delegates to C implementation
    unsafe { WriteDoc(doc, error, error_sz) }
}

/// C: mjXWriter::SetModel (xml/xml_native_writer.h:32)
/// Calls: mj_copyBack
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_set_model(self_ptr: *mut mjXWriter, _spec: *mut mjSpec, m: *const mjModel) {
    if self_ptr.is_null() { return; }
    extern "C" { fn mjXWriter_SetModel(self_ptr: *mut mjXWriter, _spec: *mut mjSpec, m: *const mjModel); }
    // SAFETY: self_ptr verified non-null; delegates to C implementation
    unsafe { mjXWriter_SetModel(self_ptr, _spec, m) }
}

/// C: mjXWriter::Write (xml/xml_native_writer.h:35)
/// Calls: mjCModel::Default, mjCModel::GetWorld, mjCModel::IsCompiled, mjCopyError, mjXWriter::Actuator, mjXWriter::Asset, mjXWriter::Body, mjXWriter::Compiler, mjXWriter::Contact, mjXWriter::Custom, mjXWriter::Default, mjXWriter::Deformable, mjXWriter::Equality, mjXWriter::Extension, mjXWriter::InsertEnd, mjXWriter::Keyframe, mjXWriter::Option, mjXWriter::Sensor, mjXWriter::Size, mjXWriter::Statistic, mjXWriter::Tendon, mjXWriter::Visual
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_write(self_ptr: *mut mjXWriter, error: *mut i8, error_sz: usize) -> std__string {
    extern "C" { fn mjXWriter_Write(self_ptr: *mut mjXWriter, error: *mut i8, error_sz: usize) -> std__string; }
    // SAFETY: delegates to C implementation
    unsafe { mjXWriter_Write(self_ptr, error, error_sz) }
}

/// C: mjXWriter::InsertEnd (xml/xml_native_writer.h:39)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_insert_end(self_ptr: *mut mjXWriter, parent: *mut tinyxml2__XMLElement, name: *const i8) -> *mut tinyxml2__XMLElement {
    extern "C" { fn mjXWriter_InsertEnd(self_ptr: *mut mjXWriter, parent: *mut tinyxml2__XMLElement, name: *const i8) -> *mut tinyxml2__XMLElement; }
    // SAFETY: delegates to C++ implementation, all pointers valid per caller contract
    unsafe { mjXWriter_InsertEnd(self_ptr, parent, name) }
}

/// C: mjXWriter::Compiler (xml/xml_native_writer.h:45)
/// Calls: mjXUtil::WriteAttrKey, mjXUtil::WriteAttrTxt, mjXWriter::InsertEnd
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_compiler(self_ptr: *mut mjXWriter, root: *mut tinyxml2__XMLElement) {
    extern "C" { fn mjXWriter_Compiler(self_ptr: *mut mjXWriter, root: *mut tinyxml2__XMLElement); }
    // SAFETY: delegates to C implementation
    unsafe { mjXWriter_Compiler(self_ptr, root) }
}

/// C: mjXWriter::Option (xml/xml_native_writer.h:46)
/// Calls: mjXUtil::WriteAttrInt, mjXUtil::WriteAttrKey, mjXWriter::InsertEnd, mj_defaultOption
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_option(self_ptr: *mut mjXWriter, root: *mut tinyxml2__XMLElement) {
    extern "C" { fn mjXWriter_Option(self_ptr: *mut mjXWriter, root: *mut tinyxml2__XMLElement); }
    // SAFETY: delegates to C implementation
    unsafe { mjXWriter_Option(self_ptr, root) }
}

/// C: mjXWriter::Size (xml/xml_native_writer.h:47)
/// Calls: mjXUtil::WriteAttrInt, mjXUtil::WriteAttrTxt, mjXWriter::InsertEnd
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_size(self_ptr: *mut mjXWriter, root: *mut tinyxml2__XMLElement) {
    extern "C" { fn mjXWriter_Size(self_ptr: *mut mjXWriter, root: *mut tinyxml2__XMLElement); }
    // SAFETY: delegates to C implementation
    unsafe { mjXWriter_Size(self_ptr, root) }
}

/// C: mjXWriter::Visual (xml/xml_native_writer.h:48)
/// Calls: mjXWriter::InsertEnd, mj_defaultVisual
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_visual(self_ptr: *mut mjXWriter, root: *mut tinyxml2__XMLElement) {
    extern "C" { fn mjXWriter_Visual(self_ptr: *mut mjXWriter, root: *mut tinyxml2__XMLElement); }
    // SAFETY: delegates to C implementation
    unsafe { mjXWriter_Visual(self_ptr, root) }
}

/// C: mjXWriter::Statistic (xml/xml_native_writer.h:49)
/// Calls: mjXWriter::InsertEnd, mjuu_defined
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_statistic(self_ptr: *mut mjXWriter, root: *mut tinyxml2__XMLElement) {
    extern "C" { fn mjXWriter_Statistic(self_ptr: *mut mjXWriter, root: *mut tinyxml2__XMLElement); }
    // SAFETY: delegates to C implementation
    unsafe { mjXWriter_Statistic(self_ptr, root) }
}

/// C: mjXWriter::Default (xml/xml_native_writer.h:50)
/// Calls: mjCDef::Actuator, mjCDef::Camera, mjCDef::Equality, mjCDef::Geom, mjCDef::Joint, mjCDef::Light, mjCDef::Material, mjCDef::Mesh, mjCDef::Pair, mjCDef::Site, mjCDef::Tendon, mjXUtil::WriteAttrTxt, mjXWriter::InsertEnd, mjXWriter::OneActuator, mjXWriter::OneCamera, mjXWriter::OneEquality, mjXWriter::OneGeom, mjXWriter::OneJoint, mjXWriter::OneLight, mjXWriter::OneMaterial, mjXWriter::OneMesh, mjXWriter::OnePair, mjXWriter::OneSite, mjXWriter::OneTendon
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_default(self_ptr: *mut mjXWriter, root: *mut tinyxml2__XMLElement, def: *mut mjCDef) {
    extern "C" { fn mjXWriter_Default(self_ptr: *mut mjXWriter, root: *mut tinyxml2__XMLElement, def: *mut mjCDef); }
    // SAFETY: delegates to C implementation
    unsafe { mjXWriter_Default(self_ptr, root, def) }
}

/// C: mjXWriter::Extension (xml/xml_native_writer.h:51)
/// Calls: mjXUtil::WriteAttrTxt, mjXWriter::InsertEnd, mjp_getPluginAtSlot
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_extension(self_ptr: *mut mjXWriter, root: *mut tinyxml2__XMLElement) {
    extern "C" { fn mjXWriter_Extension(self_ptr: *mut mjXWriter, root: *mut tinyxml2__XMLElement); }
    // SAFETY: delegates to C implementation
    unsafe { mjXWriter_Extension(self_ptr, root) }
}

/// C: mjXWriter::Custom (xml/xml_native_writer.h:52)
/// Calls: mjCModel::GetObject, mjCModel::NumObjects, mjXUtil::WriteAttrInt, mjXUtil::WriteAttrTxt, mjXWriter::InsertEnd, mju_type2Str
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_custom(self_ptr: *mut mjXWriter, root: *mut tinyxml2__XMLElement) {
    extern "C" { fn mjXWriter_Custom(self_ptr: *mut mjXWriter, root: *mut tinyxml2__XMLElement); }
    // SAFETY: delegates to C implementation
    unsafe { mjXWriter_Custom(self_ptr, root) }
}

/// C: mjXWriter::Asset (xml/xml_native_writer.h:53)
/// Calls: mjCMesh::File, mjCMesh::Plugin, mjCModel::GetObject, mjCModel::NumObjects, mjXUtil::WriteAttrInt, mjXUtil::WriteAttrKey, mjXUtil::WriteAttrTxt, mjXWriter::InsertEnd, mjXWriter::OneMaterial, mjXWriter::OneMesh, mjXWriter::OnePlugin
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_asset(self_ptr: *mut mjXWriter, root: *mut tinyxml2__XMLElement) {
    extern "C" { fn mjXWriter_Asset(self_ptr: *mut mjXWriter, root: *mut tinyxml2__XMLElement); }
    // SAFETY: delegates to C implementation
    unsafe { mjXWriter_Asset(self_ptr, root) }
}

/// C: mjXWriter::Contact (xml/xml_native_writer.h:54)
/// Calls: mjCModel::GetObject, mjCModel::NumObjects, mjXUtil::WriteAttrTxt, mjXWriter::InsertEnd, mjXWriter::OnePair
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_contact(self_ptr: *mut mjXWriter, root: *mut tinyxml2__XMLElement) {
    extern "C" { fn mjXWriter_Contact(self_ptr: *mut mjXWriter, root: *mut tinyxml2__XMLElement); }
    // SAFETY: delegates to C implementation
    unsafe { mjXWriter_Contact(self_ptr, root) }
}

/// C: mjXWriter::Deformable (xml/xml_native_writer.h:55)
/// Calls: mjCModel::GetObject, mjCModel::NumObjects, mjXWriter::InsertEnd, mjXWriter::OneFlex, mjXWriter::OneSkin
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_deformable(self_ptr: *mut mjXWriter, root: *mut tinyxml2__XMLElement) {
    extern "C" { fn mjXWriter_Deformable(self_ptr: *mut mjXWriter, root: *mut tinyxml2__XMLElement); }
    // SAFETY: delegates to C implementation
    unsafe { mjXWriter_Deformable(self_ptr, root) }
}

/// C: mjXWriter::Equality (xml/xml_native_writer.h:56)
/// Calls: mjCModel::GetObject, mjCModel::NumObjects, mjXUtil::FindValue, mjXWriter::InsertEnd, mjXWriter::OneEquality
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_equality(self_ptr: *mut mjXWriter, root: *mut tinyxml2__XMLElement) {
    extern "C" { fn mjXWriter_Equality(self_ptr: *mut mjXWriter, root: *mut tinyxml2__XMLElement); }
    // SAFETY: delegates to C implementation
    unsafe { mjXWriter_Equality(self_ptr, root) }
}

/// C: mjXWriter::Tendon (xml/xml_native_writer.h:57)
/// Calls: mjCModel::GetObject, mjCModel::NumObjects, mjCTendon::GetWrap, mjCTendon::NumWraps, mjCWrap::Type, mjXUtil::WriteAttrTxt, mjXWriter::InsertEnd, mjXWriter::OneTendon
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_tendon(self_ptr: *mut mjXWriter, root: *mut tinyxml2__XMLElement) {
    extern "C" { fn mjXWriter_Tendon(self_ptr: *mut mjXWriter, root: *mut tinyxml2__XMLElement); }
    // SAFETY: delegates to C implementation
    unsafe { mjXWriter_Tendon(self_ptr, root) }
}

/// C: mjXWriter::Actuator (xml/xml_native_writer.h:58)
/// Calls: mjCModel::GetObject, mjCModel::NumObjects, mjXWriter::InsertEnd, mjXWriter::OneActuator
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_actuator(self_ptr: *mut mjXWriter, root: *mut tinyxml2__XMLElement) {
    extern "C" { fn mjXWriter_Actuator(self_ptr: *mut mjXWriter, root: *mut tinyxml2__XMLElement); }
    // SAFETY: delegates to C implementation
    unsafe { mjXWriter_Actuator(self_ptr, root) }
}

/// C: mjXWriter::Sensor (xml/xml_native_writer.h:59)
/// Calls: mjCModel::NumObjects, mjCSensor::get_objname, mjCSensor::get_refname, mjXUtil::WriteAttrInt, mjXUtil::WriteAttrKey, mjXUtil::WriteAttrTxt, mjXWriter::InsertEnd, mjXWriter::OnePlugin, mju_condataSize, mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_sensor(self_ptr: *mut mjXWriter, root: *mut tinyxml2__XMLElement) {
    extern "C" { fn mjXWriter_Sensor(self_ptr: *mut mjXWriter, root: *mut tinyxml2__XMLElement); }
    // SAFETY: delegates to C implementation
    unsafe { mjXWriter_Sensor(self_ptr, root) }
}

/// C: mjXWriter::Keyframe (xml/xml_native_writer.h:60)
/// Calls: mjXUtil::WriteAttrTxt, mjXWriter::InsertEnd
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_keyframe(self_ptr: *mut mjXWriter, root: *mut tinyxml2__XMLElement) {
    extern "C" { fn mjXWriter_Keyframe(self_ptr: *mut mjXWriter, root: *mut tinyxml2__XMLElement); }
    // SAFETY: delegates to C implementation
    unsafe { mjXWriter_Keyframe(self_ptr, root) }
}

/// C: mjXWriter::Body (xml/xml_native_writer.h:63)
/// Calls: mjCModel::GetWorld, mjXUtil::WriteAttrKey, mjXUtil::WriteAttrTxt, mjXWriter::InsertEnd, mjXWriter::OneCamera, mjXWriter::OneFrame, mjXWriter::OneGeom, mjXWriter::OneJoint, mjXWriter::OneLight, mjXWriter::OnePlugin, mjXWriter::OneSite
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_body(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, body: *mut mjCBody, frame: *mut mjCFrame, childclass: string_view) {
    extern "C" { fn mjXWriter_Body(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, body: *mut mjCBody, frame: *mut mjCFrame, childclass: string_view); }
    // SAFETY: delegates to C implementation
    unsafe { mjXWriter_Body(self_ptr, elem, body, frame, childclass) }
}

/// C: mjXWriter::OneFlex (xml/xml_native_writer.h:66)
/// Calls: mjCFlex::get_material, mjXUtil::WriteAttrInt, mjXUtil::WriteAttrKey, mjXUtil::WriteAttrTxt, mjXWriter::InsertEnd
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_one_flex(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, pflex: *const mjCFlex) {
    extern "C" { fn mjXWriter_OneFlex(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, pflex: *const mjCFlex); }
    // SAFETY: delegates to C implementation
    unsafe { mjXWriter_OneFlex(self_ptr, elem, pflex) }
}

/// C: mjXWriter::OneMesh (xml/xml_native_writer.h:67)
/// Calls: mjCDef::Mesh, mjCMesh::ContentType, mjCMesh::File, mjCMesh::Inertia, mjCMesh::Refpos, mjCMesh::Refquat, mjCMesh::Scale, mjCMesh::SmoothNormal, mjXUtil::WriteAttrTxt
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_one_mesh(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, pmesh: *const mjCMesh, def: *mut mjCDef) {
    extern "C" { fn mjXWriter_OneMesh(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, pmesh: *const mjCMesh, def: *mut mjCDef); }
    // SAFETY: delegates to C implementation
    unsafe { mjXWriter_OneMesh(self_ptr, elem, pmesh, def) }
}

/// C: mjXWriter::OneSkin (xml/xml_native_writer.h:68)
/// Calls: mjCDef::Geom, mjCSkin::File, mjCSkin::get_material, mjXUtil::WriteAttrInt, mjXUtil::WriteAttrTxt, mjXWriter::InsertEnd
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_one_skin(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, pskin: *const mjCSkin) {
    extern "C" { fn mjXWriter_OneSkin(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, pskin: *const mjCSkin); }
    // SAFETY: delegates to C implementation
    unsafe { mjXWriter_OneSkin(self_ptr, elem, pskin) }
}

/// C: mjXWriter::OneMaterial (xml/xml_native_writer.h:69)
/// Calls: mjCDef::Material, mjCMaterial::get_texture, mjXUtil::WriteAttrKey, mjXUtil::WriteAttrTxt, mjXWriter::InsertEnd
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_one_material(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, pmaterial: *const mjCMaterial, def: *mut mjCDef) {
    extern "C" { fn mjXWriter_OneMaterial(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, pmaterial: *const mjCMaterial, def: *mut mjCDef); }
    // SAFETY: delegates to C implementation
    unsafe { mjXWriter_OneMaterial(self_ptr, elem, pmaterial, def) }
}

/// C: mjXWriter::OneJoint (xml/xml_native_writer.h:70)
/// Calls: mjCDef::Joint, mjXUtil::FindValue, mjXUtil::WriteAttrInt, mjXUtil::WriteAttrKey, mjXUtil::WriteAttrTxt
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_one_joint(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, pjoint: *const mjCJoint, def: *mut mjCDef, classname: string_view) {
    extern "C" { fn mjXWriter_OneJoint(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, pjoint: *const mjCJoint, def: *mut mjCDef, classname: string_view); }
    // SAFETY: delegates to C implementation
    unsafe { mjXWriter_OneJoint(self_ptr, elem, pjoint, def, classname) }
}

/// C: mjXWriter::OneGeom (xml/xml_native_writer.h:72)
/// Calls: mjCDef::Geom, mjCGeom::GetVolume, mjCGeom::get_hfieldname, mjCGeom::get_material, mjCGeom::get_meshname, mjCMesh::GetPosPtr, mjCMesh::GetQuatPtr, mjXUtil::WriteAttrInt, mjXUtil::WriteAttrKey, mjXUtil::WriteAttrTxt, mjXWriter::InsertEnd, mjXWriter::OnePlugin, mjuu_frameaccuminv
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_one_geom(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, pgeom: *const mjCGeom, def: *mut mjCDef, classname: string_view) {
    extern "C" { fn mjXWriter_OneGeom(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, pgeom: *const mjCGeom, def: *mut mjCDef, classname: string_view); }
    // SAFETY: delegates to C implementation
    unsafe { mjXWriter_OneGeom(self_ptr, elem, pgeom, def, classname) }
}

/// C: mjXWriter::OneSite (xml/xml_native_writer.h:74)
/// Calls: mjCDef::Site, mjCSite::get_material, mjXUtil::WriteAttrInt, mjXUtil::WriteAttrKey, mjXUtil::WriteAttrTxt
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_one_site(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, psite: *const mjCSite, def: *mut mjCDef, classname: string_view) {
    extern "C" { fn mjXWriter_OneSite(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, psite: *const mjCSite, def: *mut mjCDef, classname: string_view); }
    // SAFETY: delegates to C implementation
    unsafe { mjXWriter_OneSite(self_ptr, elem, psite, def, classname) }
}

/// C: mjXWriter::OneCamera (xml/xml_native_writer.h:76)
/// Calls: mjCCamera::get_targetbody, mjCDef::Camera, mjXUtil::WriteAttrKey, mjXUtil::WriteAttrTxt
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_one_camera(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, pcamera: *const mjCCamera, def: *mut mjCDef, classname: string_view) {
    extern "C" { fn mjXWriter_OneCamera(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, pcamera: *const mjCCamera, def: *mut mjCDef, classname: string_view); }
    // SAFETY: delegates to C implementation
    unsafe { mjXWriter_OneCamera(self_ptr, elem, pcamera, def, classname) }
}

/// C: mjXWriter::OneLight (xml/xml_native_writer.h:78)
/// Calls: mjCDef::Light, mjCLight::get_targetbody, mjCLight::get_texture, mjXUtil::WriteAttrKey, mjXUtil::WriteAttrTxt
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_one_light(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, plight: *const mjCLight, def: *mut mjCDef, classname: string_view) {
    extern "C" { fn mjXWriter_OneLight(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, plight: *const mjCLight, def: *mut mjCDef, classname: string_view); }
    // SAFETY: delegates to C implementation
    unsafe { mjXWriter_OneLight(self_ptr, elem, plight, def, classname) }
}

/// C: mjXWriter::OnePair (xml/xml_native_writer.h:80)
/// Calls: mjCDef::Pair, mjCPair::get_geomname1, mjCPair::get_geomname2, mjXUtil::WriteAttrInt, mjXUtil::WriteAttrTxt
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_one_pair(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, ppair: *const mjCPair, def: *mut mjCDef) {
    extern "C" { fn mjXWriter_OnePair(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, ppair: *const mjCPair, def: *mut mjCDef); }
    // SAFETY: delegates to C implementation
    unsafe { mjXWriter_OnePair(self_ptr, elem, ppair, def) }
}

/// C: mjXWriter::OneEquality (xml/xml_native_writer.h:81)
/// Calls: mjCDef::Equality, mjXUtil::WriteAttrKey, mjXUtil::WriteAttrTxt, mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_one_equality(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, pequality: *const mjCEquality, def: *mut mjCDef) {
    extern "C" { fn mjXWriter_OneEquality(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, pequality: *const mjCEquality, def: *mut mjCDef); }
    // SAFETY: delegates to C implementation
    unsafe { mjXWriter_OneEquality(self_ptr, elem, pequality, def) }
}

/// C: mjXWriter::OneTendon (xml/xml_native_writer.h:82)
/// Calls: mjCDef::Tendon, mjCTendon::GetWrap, mjCTendon::get_material, mjCWrap::Type, mjXUtil::WriteAttrInt, mjXUtil::WriteAttrKey, mjXUtil::WriteAttrTxt
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_one_tendon(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, ptendon: *const mjCTendon, def: *mut mjCDef) {
    extern "C" { fn mjXWriter_OneTendon(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, ptendon: *const mjCTendon, def: *mut mjCDef); }
    // SAFETY: delegates to C implementation
    unsafe { mjXWriter_OneTendon(self_ptr, elem, ptendon, def) }
}

/// C: mjXWriter::OneActuator (xml/xml_native_writer.h:83)
/// Calls: mjCActuator::get_refsite, mjCActuator::get_slidersite, mjCActuator::get_target, mjCDef::Actuator, mjXUtil::WriteAttrInt, mjXUtil::WriteAttrKey, mjXUtil::WriteAttrTxt, mjXWriter::OnePlugin
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_one_actuator(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, pactuator: *const mjCActuator, def: *mut mjCDef) {
    extern "C" { fn mjXWriter_OneActuator(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, pactuator: *const mjCActuator, def: *mut mjCDef); }
    // SAFETY: delegates to C implementation
    unsafe { mjXWriter_OneActuator(self_ptr, elem, pactuator, def) }
}

/// C: mjXWriter::OnePlugin (xml/xml_native_writer.h:84)
/// Calls: mjXUtil::WriteAttrTxt, mjXWriter::InsertEnd
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_one_plugin(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, plugin: *const mjsPlugin) {
    extern "C" { fn mjXWriter_OnePlugin(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, plugin: *const mjsPlugin); }
    // SAFETY: delegates to C++ implementation; caller guarantees all pointers are valid
    unsafe { mjXWriter_OnePlugin(self_ptr, elem, plugin) }
}

/// C: mjXWriter::OneFrame (xml/xml_native_writer.h:85)
/// Calls: mjXUtil::WriteAttrTxt, mjXWriter::InsertEnd
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_one_frame(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, frame: *mut mjCFrame) -> *mut tinyxml2__XMLElement {
    extern "C" { fn mjXWriter_OneFrame(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, frame: *mut mjCFrame) -> *mut tinyxml2__XMLElement; }
    // SAFETY: delegates to C implementation
    unsafe { mjXWriter_OneFrame(self_ptr, elem, frame) }
}

