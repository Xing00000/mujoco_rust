//! Port of: xml/xml_native_reader.cc
//! IR hash: 545f394232195ad9
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: GetAttrPtr (xml/xml_native_reader.cc:59)
#[allow(unused_variables, non_snake_case)]
pub fn get_attr_ptr(val: *mut T) -> i32 {
    // WARNING: signature changed — verify body
    // Previous params: (val : * mut T)
    // Previous return: i32
    todo ! ()
}

/// C: Reader::txt (xml/xml_native_reader.cc:111)
/// Calls: mjXUtil::ReadAttrTxt
#[allow(unused_variables, non_snake_case)]
pub fn reader_txt(self_ptr: *mut Reader, attr: *const i8, target: *mut T, set_func: void_____T____const_char) -> bool {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut Reader, attr : * const i8, target : * mut T, set_func : void_____T____const_char)
    // Previous return: bool
    todo ! ()
}

/// C: Reader::set_node (xml/xml_native_reader.cc:121)
#[allow(unused_variables, non_snake_case)]
pub fn reader_set_node(self_ptr: *mut Reader, node: *mut XMLElement) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut Reader, node : * mut XMLElement)
    // Previous return: ()
    todo ! ()
}

/// C: ReadPluginConfigs (xml/xml_native_reader.cc:128)
/// Calls: FirstChildElement, NextSiblingElement, mjXUtil::ReadAttrTxt
#[allow(unused_variables, non_snake_case)]
pub fn read_plugin_configs(elem: *mut tinyxml2__XMLElement, p: *mut mjsPlugin) {
    // WARNING: signature changed — verify body
    // Previous params: (elem : * mut tinyxml2__XMLElement, p : * mut mjsPlugin)
    // Previous return: ()
    todo ! ()
}

/// C: UpdateString (xml/xml_native_reader.cc:155)
#[allow(unused_variables, non_snake_case)]
pub fn update_string(psuffix: *mut string, count: i32, i: i32) {
    // WARNING: signature changed — verify body
    // Previous params: (psuffix : * mut string, count : i32, i : i32)
    // Previous return: ()
    todo ! ()
}

/// C: stripError (xml/xml_native_reader.cc:3878)
#[allow(unused_variables, non_snake_case)]
pub fn strip_error(err: *const i8) -> *const i8 {
    // WARNING: signature changed — verify body
    // Previous params: (err : * const i8)
    // Previous return: * const i8
    todo ! ()
}

/// C: mjXReader::Parse (xml/xml_native_reader.h:34)
/// Calls: FirstChildElement, NextSiblingElement, mjXReader::Actuator, mjXReader::Asset, mjXReader::Body, mjXReader::Contact, mjXReader::Custom, mjXReader::Default, mjXReader::Deformable, mjXReader::Equality, mjXReader::Extension, mjXReader::Keyframe, mjXReader::Sensor, mjXReader::Statistic, mjXReader::Tendon, mjXReader::Visual, mjXUtil::ReadAttrTxt
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_parse(self_ptr: *mut mjXReader, root: *mut tinyxml2__XMLElement, vfs: *const mjVFS) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjXReader, root : * mut tinyxml2__XMLElement, vfs : * const mjVFS)
    // Previous return: ()
    todo ! ()
}

/// C: mjXReader::PrintSchema (xml/xml_native_reader.h:35)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_print_schema(self_ptr: *mut mjXReader, str: *mut std__stringstream, html: bool, pad: bool) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjXReader, str : * mut std__stringstream, html : bool, pad : bool)
    // Previous return: ()
    todo ! ()
}

/// C: mjXReader::SetModelFileDir (xml/xml_native_reader.h:37)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_set_model_file_dir(self_ptr: *mut mjXReader, modelfiledir: *const std__string) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjXReader, modelfiledir : * const std__string)
    // Previous return: ()
    todo ! ()
}

/// C: mjXReader::ModelFileDir (xml/xml_native_reader.h:38)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_model_file_dir(self_ptr: *mut mjXReader) -> *const mujoco__user__FilePath {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjXReader)
    // Previous return: * const mujoco__user__FilePath
    todo ! ()
}

/// C: mjXReader::SetAssetDir (xml/xml_native_reader.h:41)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_set_asset_dir(self_ptr: *mut mjXReader, assetdir: *const std__string) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjXReader, assetdir : * const std__string)
    // Previous return: ()
    todo ! ()
}

/// C: mjXReader::SetMeshDir (xml/xml_native_reader.h:42)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_set_mesh_dir(self_ptr: *mut mjXReader, meshdir: *const std__string) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjXReader, meshdir : * const std__string)
    // Previous return: ()
    todo ! ()
}

/// C: mjXReader::SetTextureDir (xml/xml_native_reader.h:43)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_set_texture_dir(self_ptr: *mut mjXReader, texturedir: *const std__string) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjXReader, texturedir : * const std__string)
    // Previous return: ()
    todo ! ()
}

/// C: mjXReader::Compiler (xml/xml_native_reader.h:46)
/// Calls: mjXUtil::MapValue, mjXUtil::ReadAttrTxt, mjs_setAuthored
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_compiler(section: *mut tinyxml2__XMLElement, s: *mut mjSpec) {
    // WARNING: signature changed — verify body
    // Previous params: (section : * mut tinyxml2__XMLElement, s : * mut mjSpec)
    // Previous return: ()
    todo ! ()
}

/// C: mjXReader::Option (xml/xml_native_reader.h:47)
/// Calls: mjXUtil::MapValue
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_option(section: *mut tinyxml2__XMLElement, s: *mut mjSpec, opt: *mut mjOption) {
    // WARNING: signature changed — verify body
    // Previous params: (section : * mut tinyxml2__XMLElement, s : * mut mjSpec, opt : * mut mjOption)
    // Previous return: ()
    todo ! ()
}

/// C: mjXReader::Size (xml/xml_native_reader.h:48)
/// Calls: mjXUtil::ReadAttrInt
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_size(section: *mut tinyxml2__XMLElement, s: *mut mjSpec) {
    // WARNING: signature changed — verify body
    // Previous params: (section : * mut tinyxml2__XMLElement, s : * mut mjSpec)
    // Previous return: ()
    todo ! ()
}

/// C: mjXReader::Default (xml/xml_native_reader.h:52)
/// Calls: FirstChildElement, NextSiblingElement, mjXReader::OneActuator, mjXReader::OneCamera, mjXReader::OneEquality, mjXReader::OneGeom, mjXReader::OneJoint, mjXReader::OneLight, mjXReader::OneMaterial, mjXReader::OneMesh, mjXReader::OnePair, mjXReader::OneSite, mjXReader::OneTendon, mjXUtil::ReadAttrTxt
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_default(self_ptr: *mut mjXReader, section: *mut tinyxml2__XMLElement, def: *const mjsDefault, vfs: *const mjVFS) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjXReader, section : * mut tinyxml2__XMLElement, def : * const mjsDefault, vfs : * const mjVFS)
    // Previous return: ()
    todo ! ()
}

/// C: mjXReader::Extension (xml/xml_native_reader.h:54)
/// Calls: FirstChildElement, NextSiblingElement, ReadPluginConfigs, mjXUtil::ReadAttrTxt
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_extension(self_ptr: *mut mjXReader, section: *mut tinyxml2__XMLElement) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjXReader, section : * mut tinyxml2__XMLElement)
    // Previous return: ()
    todo ! ()
}

/// C: mjXReader::Custom (xml/xml_native_reader.h:55)
/// Calls: FirstChildElement, NextSiblingElement, mjXUtil::ReadAttrInt, mjXUtil::ReadAttrTxt
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_custom(self_ptr: *mut mjXReader, section: *mut tinyxml2__XMLElement) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjXReader, section : * mut tinyxml2__XMLElement)
    // Previous return: ()
    todo ! ()
}

/// C: mjXReader::Visual (xml/xml_native_reader.h:56)
/// Calls: FirstChildElement, NextSiblingElement, Reader::set_node
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_visual(self_ptr: *mut mjXReader, section: *mut tinyxml2__XMLElement) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjXReader, section : * mut tinyxml2__XMLElement)
    // Previous return: ()
    todo ! ()
}

/// C: mjXReader::Statistic (xml/xml_native_reader.h:57)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_statistic(self_ptr: *mut mjXReader, section: *mut tinyxml2__XMLElement) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjXReader, section : * mut tinyxml2__XMLElement)
    // Previous return: ()
    todo ! ()
}

/// C: mjXReader::Asset (xml/xml_native_reader.h:58)
/// Calls: FirstChildElement, NextSiblingElement, mjXReader::AssetDir, mjXReader::GetClass, mjXReader::OneMaterial, mjXReader::OneMesh, mjXReader::OneSkin, mjXReader::TextureDir, mjXUtil::MapValue, mjXUtil::ReadAttrFile, mjXUtil::ReadAttrInt, mjXUtil::ReadAttrTxt
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_asset(self_ptr: *mut mjXReader, section: *mut tinyxml2__XMLElement, vfs: *const mjVFS) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjXReader, section : * mut tinyxml2__XMLElement, vfs : * const mjVFS)
    // Previous return: ()
    todo ! ()
}

/// C: mjXReader::Body (xml/xml_native_reader.h:59)
/// Calls: FirstChildElement, NextSiblingElement, mjXBase::ReadAlternative, mjXReader::GetClass, mjXReader::OneCamera, mjXReader::OneComposite, mjXReader::OneFlexcomp, mjXReader::OneGeom, mjXReader::OneJoint, mjXReader::OneLight, mjXReader::OnePlugin, mjXReader::OneSite, mjXUtil::MapValue, mjXUtil::ReadAttrInt, mjXUtil::ReadAttrTxt, mjXUtil::ReadQuat, mjs_addBody, mjs_addCamera, mjs_addFrame, mjs_addFreeJoint, mjs_addGeom, mjs_addJoint, mjs_addLight, mjs_addSite, mjs_defaultOrientation, mjs_getDefault, mjs_getId, mjs_setDefault, mjs_setFrame, mjuu_frameaccum, mjuu_setvec
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_body(self_ptr: *mut mjXReader, section: *mut tinyxml2__XMLElement, pbody: *mut mjsBody, pframe: *mut mjsFrame, vfs: *const mjVFS) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjXReader, section : * mut tinyxml2__XMLElement, pbody : * mut mjsBody, pframe : * mut mjsFrame, vfs : * const mjVFS)
    // Previous return: ()
    todo ! ()
}

/// C: mjXReader::Contact (xml/xml_native_reader.h:61)
/// Calls: FirstChildElement, NextSiblingElement, mjXReader::GetClass, mjXReader::OnePair, mjXUtil::ReadAttrTxt
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_contact(self_ptr: *mut mjXReader, section: *mut tinyxml2__XMLElement) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjXReader, section : * mut tinyxml2__XMLElement)
    // Previous return: ()
    todo ! ()
}

/// C: mjXReader::Deformable (xml/xml_native_reader.h:62)
/// Calls: FirstChildElement, NextSiblingElement, mjXReader::GetClass, mjXReader::OneFlex, mjXReader::OneSkin
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_deformable(self_ptr: *mut mjXReader, section: *mut tinyxml2__XMLElement, vfs: *const mjVFS) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjXReader, section : * mut tinyxml2__XMLElement, vfs : * const mjVFS)
    // Previous return: ()
    todo ! ()
}

/// C: mjXReader::Equality (xml/xml_native_reader.h:63)
/// Calls: FirstChildElement, NextSiblingElement, mjXReader::GetClass, mjXReader::OneEquality
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_equality(self_ptr: *mut mjXReader, section: *mut tinyxml2__XMLElement) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjXReader, section : * mut tinyxml2__XMLElement)
    // Previous return: ()
    todo ! ()
}

/// C: mjXReader::Tendon (xml/xml_native_reader.h:64)
/// Calls: FirstChildElement, NextSiblingElement, mjXReader::GetClass, mjXReader::OneTendon, mjXUtil::ReadAttrTxt, mjs_wrapPulley
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_tendon(self_ptr: *mut mjXReader, section: *mut tinyxml2__XMLElement) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjXReader, section : * mut tinyxml2__XMLElement)
    // Previous return: ()
    todo ! ()
}

/// C: mjXReader::Actuator (xml/xml_native_reader.h:65)
/// Calls: FirstChildElement, NextSiblingElement, mjXReader::GetClass, mjXReader::OneActuator
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_actuator(self_ptr: *mut mjXReader, section: *mut tinyxml2__XMLElement) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjXReader, section : * mut tinyxml2__XMLElement)
    // Previous return: ()
    todo ! ()
}

/// C: mjXReader::Sensor (xml/xml_native_reader.h:66)
/// Calls: FirstChildElement, NextSiblingElement, mjXReader::OnePlugin, mjXUtil::MapValue, mjXUtil::MapValues, mjXUtil::ReadAttrInt, mjXUtil::ReadAttrTxt
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_sensor(self_ptr: *mut mjXReader, section: *mut tinyxml2__XMLElement) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjXReader, section : * mut tinyxml2__XMLElement)
    // Previous return: ()
    todo ! ()
}

/// C: mjXReader::Keyframe (xml/xml_native_reader.h:67)
/// Calls: FirstChildElement, NextSiblingElement, mjXUtil::ReadAttrTxt
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_keyframe(self_ptr: *mut mjXReader, section: *mut tinyxml2__XMLElement) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjXReader, section : * mut tinyxml2__XMLElement)
    // Previous return: ()
    todo ! ()
}

/// C: mjXReader::OneFlex (xml/xml_native_reader.h:70)
/// Calls: FirstChildElement, mjXUtil::MapValue, mjXUtil::ReadAttrInt, mjXUtil::ReadAttrTxt
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_one_flex(self_ptr: *mut mjXReader, elem: *mut tinyxml2__XMLElement, pflex: *mut mjsFlex) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjXReader, elem : * mut tinyxml2__XMLElement, pflex : * mut mjsFlex)
    // Previous return: ()
    todo ! ()
}

/// C: mjXReader::OneMesh (xml/xml_native_reader.h:71)
/// Calls: FirstChildElement, mjXReader::MeshDir, mjXReader::OnePlugin, mjXUtil::MapValue, mjXUtil::ReadAttrInt, mjXUtil::ReadAttrTxt
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_one_mesh(self_ptr: *mut mjXReader, elem: *mut tinyxml2__XMLElement, pmesh: *mut mjsMesh, vfs: *const mjVFS) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjXReader, elem : * mut tinyxml2__XMLElement, pmesh : * mut mjsMesh, vfs : * const mjVFS)
    // Previous return: ()
    todo ! ()
}

/// C: mjXReader::OneSkin (xml/xml_native_reader.h:72)
/// Calls: FirstChildElement, NextSiblingElement, mjXReader::AssetDir, mjXUtil::ReadAttrInt, mjXUtil::ReadAttrTxt
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_one_skin(self_ptr: *mut mjXReader, elem: *mut tinyxml2__XMLElement, pskin: *mut mjsSkin, vfs: *const mjVFS) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjXReader, elem : * mut tinyxml2__XMLElement, pskin : * mut mjsSkin, vfs : * const mjVFS)
    // Previous return: ()
    todo ! ()
}

/// C: mjXReader::OneMaterial (xml/xml_native_reader.h:73)
/// Calls: FirstChildElement, NextSiblingElement, mjXUtil::FindKey, mjXUtil::MapValue, mjXUtil::ReadAttrTxt
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_one_material(self_ptr: *mut mjXReader, elem: *mut tinyxml2__XMLElement, pmaterial: *mut mjsMaterial) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjXReader, elem : * mut tinyxml2__XMLElement, pmaterial : * mut mjsMaterial)
    // Previous return: ()
    todo ! ()
}

/// C: mjXReader::OneJoint (xml/xml_native_reader.h:74)
/// Calls: mjXUtil::MapValue, mjXUtil::ReadAttrInt, mjXUtil::ReadAttrTxt
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_one_joint(self_ptr: *mut mjXReader, elem: *mut tinyxml2__XMLElement, pjoint: *mut mjsJoint) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjXReader, elem : * mut tinyxml2__XMLElement, pjoint : * mut mjsJoint)
    // Previous return: ()
    todo ! ()
}

/// C: mjXReader::OneGeom (xml/xml_native_reader.h:75)
/// Calls: FirstChildElement, mjXBase::ReadAlternative, mjXReader::OnePlugin, mjXUtil::MapValue, mjXUtil::ReadAttrInt, mjXUtil::ReadAttrTxt, mjXUtil::ReadQuat
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_one_geom(self_ptr: *mut mjXReader, elem: *mut tinyxml2__XMLElement, pgeom: *mut mjsGeom) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjXReader, elem : * mut tinyxml2__XMLElement, pgeom : * mut mjsGeom)
    // Previous return: ()
    todo ! ()
}

/// C: mjXReader::OneSite (xml/xml_native_reader.h:76)
/// Calls: mjXBase::ReadAlternative, mjXUtil::MapValue, mjXUtil::ReadAttrInt, mjXUtil::ReadAttrTxt, mjXUtil::ReadQuat
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_one_site(self_ptr: *mut mjXReader, elem: *mut tinyxml2__XMLElement, site: *mut mjsSite) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjXReader, elem : * mut tinyxml2__XMLElement, site : * mut mjsSite)
    // Previous return: ()
    todo ! ()
}

/// C: mjXReader::OneCamera (xml/xml_native_reader.h:77)
/// Calls: mjXBase::ReadAlternative, mjXUtil::MapValue, mjXUtil::MapValues, mjXUtil::ReadAttrTxt, mjXUtil::ReadQuat
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_one_camera(self_ptr: *mut mjXReader, elem: *mut tinyxml2__XMLElement, pcamera: *mut mjsCamera) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjXReader, elem : * mut tinyxml2__XMLElement, pcamera : * mut mjsCamera)
    // Previous return: ()
    todo ! ()
}

/// C: mjXReader::OneLight (xml/xml_native_reader.h:78)
/// Calls: mjXUtil::MapValue, mjXUtil::ReadAttrTxt
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_one_light(self_ptr: *mut mjXReader, elem: *mut tinyxml2__XMLElement, plight: *mut mjsLight) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjXReader, elem : * mut tinyxml2__XMLElement, plight : * mut mjsLight)
    // Previous return: ()
    todo ! ()
}

/// C: mjXReader::OnePair (xml/xml_native_reader.h:79)
/// Calls: mjXUtil::ReadAttrInt, mjXUtil::ReadAttrTxt
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_one_pair(self_ptr: *mut mjXReader, elem: *mut tinyxml2__XMLElement, ppair: *mut mjsPair) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjXReader, elem : * mut tinyxml2__XMLElement, ppair : * mut mjsPair)
    // Previous return: ()
    todo ! ()
}

/// C: mjXReader::OneEquality (xml/xml_native_reader.h:80)
/// Calls: mjXUtil::FindKey, mjXUtil::MapValue, mjXUtil::ReadAttrTxt, mjuu_zerovec
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_one_equality(self_ptr: *mut mjXReader, elem: *mut tinyxml2__XMLElement, pequality: *mut mjsEquality) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjXReader, elem : * mut tinyxml2__XMLElement, pequality : * mut mjsEquality)
    // Previous return: ()
    todo ! ()
}

/// C: mjXReader::OneTendon (xml/xml_native_reader.h:81)
/// Calls: mjXUtil::MapValue, mjXUtil::ReadAttrInt, mjXUtil::ReadAttrTxt
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_one_tendon(self_ptr: *mut mjXReader, elem: *mut tinyxml2__XMLElement, ptendon: *mut mjsTendon) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjXReader, elem : * mut tinyxml2__XMLElement, ptendon : * mut mjsTendon)
    // Previous return: ()
    todo ! ()
}

/// C: mjXReader::OneActuator (xml/xml_native_reader.h:82)
/// Calls: mjXReader::OnePlugin, mjXUtil::MapValue, mjXUtil::ReadAttrInt, mjXUtil::ReadAttrTxt, mjs_setToAdhesion, mjs_setToCylinder, mjs_setToDCMotor, mjs_setToDamper, mjs_setToIntVelocity, mjs_setToMotor, mjs_setToMuscle, mjs_setToPosition, mjs_setToVelocity
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_one_actuator(self_ptr: *mut mjXReader, elem: *mut tinyxml2__XMLElement, pactuator: *mut mjsActuator) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjXReader, elem : * mut tinyxml2__XMLElement, pactuator : * mut mjsActuator)
    // Previous return: ()
    todo ! ()
}

/// C: mjXReader::OneComposite (xml/xml_native_reader.h:83)
/// Calls: FirstChildElement, NextSiblingElement, mjCComposite::AddDefaultJoint, mjCComposite::SetDefault, mjXReader::OnePlugin, mjXUtil::FindKey, mjXUtil::MapValue, mjXUtil::ReadAttrInt, mjXUtil::ReadAttrTxt
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_one_composite(self_ptr: *mut mjXReader, elem: *mut tinyxml2__XMLElement, pbody: *mut mjsBody, pframe: *mut mjsFrame, def: *const mjsDefault) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjXReader, elem : * mut tinyxml2__XMLElement, pbody : * mut mjsBody, pframe : * mut mjsFrame, def : * const mjsDefault)
    // Previous return: ()
    todo ! ()
}

/// C: mjXReader::OneFlexcomp (xml/xml_native_reader.h:85)
/// Calls: FirstChildElement, NextSiblingElement, mjCFlexcomp::Make, mjXBase::ReadAlternative, mjXReader::OnePlugin, mjXUtil::MapValue, mjXUtil::ReadAttrInt, mjXUtil::ReadAttrTxt
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_one_flexcomp(self_ptr: *mut mjXReader, elem: *mut tinyxml2__XMLElement, pbody: *mut mjsBody, vfs: *const mjVFS) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjXReader, elem : * mut tinyxml2__XMLElement, pbody : * mut mjsBody, vfs : * const mjVFS)
    // Previous return: ()
    todo ! ()
}

/// C: mjXReader::OnePlugin (xml/xml_native_reader.h:86)
/// Calls: ReadPluginConfigs, mjXUtil::ReadAttrTxt
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_one_plugin(self_ptr: *mut mjXReader, elem: *mut tinyxml2__XMLElement, plugin: *mut mjsPlugin) {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjXReader, elem : * mut tinyxml2__XMLElement, plugin : * mut mjsPlugin)
    // Previous return: ()
    todo ! ()
}

/// C: mjXReader::GetClass (xml/xml_native_reader.h:89)
/// Calls: mjXUtil::ReadAttrTxt
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_get_class(self_ptr: *mut mjXReader, section: *mut tinyxml2__XMLElement) -> *const mjsDefault {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjXReader, section : * mut tinyxml2__XMLElement)
    // Previous return: * const mjsDefault
    todo ! ()
}

/// C: mjXReader::AssetDir (xml/xml_native_reader.h:94)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_asset_dir(self_ptr: *mut mjXReader) -> mujoco__user__FilePath {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjXReader)
    // Previous return: mujoco__user__FilePath
    todo ! ()
}

/// C: mjXReader::MeshDir (xml/xml_native_reader.h:95)
/// Calls: mjXReader::AssetDir
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_mesh_dir(self_ptr: *mut mjXReader) -> mujoco__user__FilePath {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjXReader)
    // Previous return: mujoco__user__FilePath
    todo ! ()
}

/// C: mjXReader::TextureDir (xml/xml_native_reader.h:96)
/// Calls: mjXReader::AssetDir
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_texture_dir(self_ptr: *mut mjXReader) -> mujoco__user__FilePath {
    // WARNING: signature changed — verify body
    // Previous params: (self_ptr : * mut mjXReader)
    // Previous return: mujoco__user__FilePath
    todo ! ()
}

