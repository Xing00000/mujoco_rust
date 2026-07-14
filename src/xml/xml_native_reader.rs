//! Port of: xml/xml_native_reader.cc
//! IR hash: 8cbd078414266fa8
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: GetAttrPtr (xml/xml_native_reader.cc:59)
#[allow(unused_variables, non_snake_case)]
pub fn get_attr_ptr(val: *mut T) -> *const () {
    todo!() // GetAttrPtr
}

/// C: Reader::txt (xml/xml_native_reader.cc:111)
/// Calls: mjXUtil::ReadAttrTxt
#[allow(unused_variables, non_snake_case)]
pub fn reader_txt(self_ptr: *mut Reader, attr: *const i8, target: *mut T, set_func: void_____T____const_char) -> bool {
    todo!() // Reader::txt
}

/// C: Reader::set_node (xml/xml_native_reader.cc:121)
#[allow(unused_variables, non_snake_case)]
pub fn reader_set_node(self_ptr: *mut Reader, node: *mut XMLElement) {
    todo!() // Reader::set_node
}

/// C: ReadPluginConfigs (xml/xml_native_reader.cc:128)
/// Calls: FirstChildElement, NextSiblingElement, mjXUtil::ReadAttrTxt, mjs_setPluginAttributes
#[allow(unused_variables, non_snake_case)]
pub fn read_plugin_configs(elem: *mut tinyxml2__XMLElement, p: *mut mjsPlugin) {
    todo!() // ReadPluginConfigs
}

/// C: UpdateString (xml/xml_native_reader.cc:155)
#[allow(unused_variables, non_snake_case)]
pub fn update_string(psuffix: *mut string, count: i32, i: i32) {
    todo!() // UpdateString
}

/// C: stripError (xml/xml_native_reader.cc:3878)
#[allow(unused_variables, non_snake_case)]
pub fn strip_error(err: *const i8) -> *const i8 {
    todo!() // stripError
}

/// C: mjXReader::Parse (xml/xml_native_reader.h:34)
/// Calls: FirstChildElement, NextSiblingElement, mjXReader::Actuator, mjXReader::Asset, mjXReader::Body, mjXReader::Compiler, mjXReader::Contact, mjXReader::Custom, mjXReader::Default, mjXReader::Deformable, mjXReader::Equality, mjXReader::Extension, mjXReader::Keyframe, mjXReader::Option, mjXReader::Sensor, mjXReader::Size, mjXReader::Statistic, mjXReader::Tendon, mjXReader::Visual, mjXSchema::Check, mjXSchema::GetError, mjXUtil::ReadAttrTxt, mjs_findBody, mjs_setDeepCopy, mjs_setString
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_parse(self_ptr: *mut mjXReader, root: *mut tinyxml2__XMLElement, vfs: *const mjVFS) {
    todo!() // mjXReader::Parse
}

/// C: mjXReader::PrintSchema (xml/xml_native_reader.h:35)
/// Calls: mjXSchema::Print, mjXSchema::PrintHTML
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_print_schema(self_ptr: *mut mjXReader, str: *mut std__stringstream, html: bool, pad: bool) {
    todo!() // mjXReader::PrintSchema
}

/// C: mjXReader::SetModelFileDir (xml/xml_native_reader.h:37)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_set_model_file_dir(self_ptr: *mut mjXReader, modelfiledir: *const std__string) {
    todo!() // mjXReader::SetModelFileDir
}

/// C: mjXReader::ModelFileDir (xml/xml_native_reader.h:38)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_model_file_dir(self_ptr: *mut mjXReader) -> *const mujoco__user__FilePath {
    // SAFETY: self_ptr is a valid pointer to mjXReader (caller contract)
    unsafe {
        &(*self_ptr).modelfiledir_ as *const mujoco__user__FilePath
    }
}

/// C: mjXReader::SetAssetDir (xml/xml_native_reader.h:41)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_set_asset_dir(self_ptr: *mut mjXReader, assetdir: *const std__string) {
    todo!() // mjXReader::SetAssetDir
}

/// C: mjXReader::SetMeshDir (xml/xml_native_reader.h:42)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_set_mesh_dir(self_ptr: *mut mjXReader, meshdir: *const std__string) {
    todo!() // mjXReader::SetMeshDir
}

/// C: mjXReader::SetTextureDir (xml/xml_native_reader.h:43)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_set_texture_dir(self_ptr: *mut mjXReader, texturedir: *const std__string) {
    todo!() // mjXReader::SetTextureDir
}

/// C: mjXReader::Compiler (xml/xml_native_reader.h:46)
/// Calls: mjXUtil::FindSubElem, mjXUtil::MapValue, mjXUtil::ReadAttr, mjXUtil::ReadAttrTxt, mjs_setAuthored, mjs_setString
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_compiler(section: *mut tinyxml2__XMLElement, s: *mut mjSpec) {
    todo!() // mjXReader::Compiler
}

/// C: mjXReader::Option (xml/xml_native_reader.h:47)
/// Calls: mjXUtil::FindSubElem, mjXUtil::MapValue
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_option(section: *mut tinyxml2__XMLElement, s: *mut mjSpec, opt: *mut mjOption) {
    todo!() // mjXReader::Option
}

/// C: mjXReader::Size (xml/xml_native_reader.h:48)
/// Calls: mjXUtil::ReadAttrInt
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_size(section: *mut tinyxml2__XMLElement, s: *mut mjSpec) {
    todo!() // mjXReader::Size
}

/// C: mjXReader::Default (xml/xml_native_reader.h:52)
/// Calls: FirstChildElement, NextSiblingElement, mjXReader::OneActuator, mjXReader::OneCamera, mjXReader::OneEquality, mjXReader::OneGeom, mjXReader::OneJoint, mjXReader::OneLight, mjXReader::OneMaterial, mjXReader::OneMesh, mjXReader::OnePair, mjXReader::OneSite, mjXReader::OneTendon, mjXUtil::ReadAttrTxt, mjs_addDefault, mjs_getSpecDefault
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_default(self_ptr: *mut mjXReader, section: *mut tinyxml2__XMLElement, def: *const mjsDefault, vfs: *const mjVFS) {
    todo!() // mjXReader::Default
}

/// C: mjXReader::Extension (xml/xml_native_reader.h:54)
/// Calls: FirstChildElement, NextSiblingElement, ReadPluginConfigs, mjXUtil::ReadAttrTxt, mjs_activatePlugin, mjs_addPlugin, mjs_setString
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_extension(self_ptr: *mut mjXReader, section: *mut tinyxml2__XMLElement) {
    todo!() // mjXReader::Extension
}

/// C: mjXReader::Custom (xml/xml_native_reader.h:55)
/// Calls: FirstChildElement, NextSiblingElement, mjXUtil::ReadAttr, mjXUtil::ReadAttrInt, mjXUtil::ReadAttrTxt, mjs_addNumeric, mjs_addText, mjs_addTuple, mjs_getError, mjs_setDouble, mjs_setInt, mjs_setName, mjs_setString, mjs_setStringVec, mju_str2Type
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_custom(self_ptr: *mut mjXReader, section: *mut tinyxml2__XMLElement) {
    todo!() // mjXReader::Custom
}

/// C: mjXReader::Visual (xml/xml_native_reader.h:56)
/// Calls: FirstChildElement, NextSiblingElement, Reader::set_node
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_visual(self_ptr: *mut mjXReader, section: *mut tinyxml2__XMLElement) {
    todo!() // mjXReader::Visual
}

/// C: mjXReader::Statistic (xml/xml_native_reader.h:57)
/// Calls: mjXUtil::ReadAttr, mjuu_defined
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_statistic(self_ptr: *mut mjXReader, section: *mut tinyxml2__XMLElement) {
    todo!() // mjXReader::Statistic
}

/// C: mjXReader::Asset (xml/xml_native_reader.h:58)
/// Calls: FilePath::Str, FilePath::c_str, FirstChildElement, NextSiblingElement, mjXReader::AssetDir, mjXReader::GetClass, mjXReader::OneMaterial, mjXReader::OneMesh, mjXReader::OneSkin, mjXReader::TextureDir, mjXUtil::MapValue, mjXUtil::ReadAttr, mjXUtil::ReadAttrFile, mjXUtil::ReadAttrInt, mjXUtil::ReadAttrTxt, mjXUtil::ReadAttrVec, mj_parse, mjs_addHField, mjs_addMaterial, mjs_addMesh, mjs_addSkin, mjs_addSpec, mjs_addTexture, mjs_getError, mjs_getSpecDefault, mjs_setFloat, mjs_setInStringVec, mjs_setName, mjs_setString
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_asset(self_ptr: *mut mjXReader, section: *mut tinyxml2__XMLElement, vfs: *const mjVFS) {
    todo!() // mjXReader::Asset
}

/// C: mjXReader::Body (xml/xml_native_reader.h:59)
/// Calls: FirstChildElement, NextSiblingElement, UpdateString, mjXBase::ReadAlternative, mjXReader::GetClass, mjXReader::OneCamera, mjXReader::OneComposite, mjXReader::OneFlexcomp, mjXReader::OneGeom, mjXReader::OneJoint, mjXReader::OneLight, mjXReader::OnePlugin, mjXReader::OneSite, mjXUtil::MapValue, mjXUtil::ReadAttr, mjXUtil::ReadAttrInt, mjXUtil::ReadAttrTxt, mjXUtil::ReadQuat, mjXUtil::ReadVector, mjs_addBody, mjs_addCamera, mjs_addFrame, mjs_addFreeJoint, mjs_addGeom, mjs_addJoint, mjs_addLight, mjs_addSite, mjs_attach, mjs_defaultOrientation, mjs_delete, mjs_findBody, mjs_findDefault, mjs_findSpec, mjs_getDefault, mjs_getError, mjs_getId, mjs_resolveOrientation, mjs_setDefault, mjs_setDouble, mjs_setFrame, mjs_setName, mjs_setString, mjuu_copyvec, mjuu_frameaccum, mjuu_setvec, stripError
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_body(self_ptr: *mut mjXReader, section: *mut tinyxml2__XMLElement, pbody: *mut mjsBody, pframe: *mut mjsFrame, vfs: *const mjVFS) {
    todo!() // mjXReader::Body
}

/// C: mjXReader::Contact (xml/xml_native_reader.h:61)
/// Calls: FirstChildElement, NextSiblingElement, mjXReader::GetClass, mjXReader::OnePair, mjXUtil::ReadAttrTxt, mjs_addExclude, mjs_addPair, mjs_getError, mjs_getSpecDefault, mjs_setName, mjs_setString
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_contact(self_ptr: *mut mjXReader, section: *mut tinyxml2__XMLElement) {
    todo!() // mjXReader::Contact
}

/// C: mjXReader::Deformable (xml/xml_native_reader.h:62)
/// Calls: FirstChildElement, NextSiblingElement, mjXReader::GetClass, mjXReader::OneFlex, mjXReader::OneSkin, mjs_addFlex, mjs_addSkin, mjs_getSpecDefault
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_deformable(self_ptr: *mut mjXReader, section: *mut tinyxml2__XMLElement, vfs: *const mjVFS) {
    todo!() // mjXReader::Deformable
}

/// C: mjXReader::Equality (xml/xml_native_reader.h:63)
/// Calls: FirstChildElement, NextSiblingElement, mjXReader::GetClass, mjXReader::OneEquality, mjs_addEquality, mjs_getSpecDefault
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_equality(self_ptr: *mut mjXReader, section: *mut tinyxml2__XMLElement) {
    todo!() // mjXReader::Equality
}

/// C: mjXReader::Tendon (xml/xml_native_reader.h:64)
/// Calls: FirstChildElement, NextSiblingElement, mjXReader::GetClass, mjXReader::OneTendon, mjXUtil::ReadAttr, mjXUtil::ReadAttrTxt, mjs_addTendon, mjs_getSpecDefault, mjs_setString, mjs_wrapGeom, mjs_wrapJoint, mjs_wrapPulley, mjs_wrapSite
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_tendon(self_ptr: *mut mjXReader, section: *mut tinyxml2__XMLElement) {
    todo!() // mjXReader::Tendon
}

/// C: mjXReader::Actuator (xml/xml_native_reader.h:65)
/// Calls: FirstChildElement, NextSiblingElement, mjXReader::GetClass, mjXReader::OneActuator, mjs_addActuator, mjs_getSpecDefault
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_actuator(self_ptr: *mut mjXReader, section: *mut tinyxml2__XMLElement) {
    todo!() // mjXReader::Actuator
}

/// C: mjXReader::Sensor (xml/xml_native_reader.h:66)
/// Calls: FirstChildElement, NextSiblingElement, mjXReader::OnePlugin, mjXUtil::MapValue, mjXUtil::MapValues, mjXUtil::ReadAttr, mjXUtil::ReadAttrInt, mjXUtil::ReadAttrTxt, mjXUtil::ReadVector, mjs_addSensor, mjs_getError, mjs_setDouble, mjs_setName, mjs_setString, mju_str2Type
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_sensor(self_ptr: *mut mjXReader, section: *mut tinyxml2__XMLElement) {
    todo!() // mjXReader::Sensor
}

/// C: mjXReader::Keyframe (xml/xml_native_reader.h:67)
/// Calls: FirstChildElement, NextSiblingElement, mjXUtil::ReadAttr, mjXUtil::ReadAttrTxt, mjXUtil::ReadAttrVec, mjs_addKey, mjs_getError, mjs_setDouble, mjs_setName
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_keyframe(self_ptr: *mut mjXReader, section: *mut tinyxml2__XMLElement) {
    todo!() // mjXReader::Keyframe
}

/// C: mjXReader::OneFlex (xml/xml_native_reader.h:70)
/// Calls: FirstChildElement, mjXUtil::MapValue, mjXUtil::ReadAttr, mjXUtil::ReadAttrInt, mjXUtil::ReadAttrTxt, mjXUtil::ReadAttrVec, mjs_getError, mjs_setDouble, mjs_setFloat, mjs_setInt, mjs_setName, mjs_setString, mjs_setStringVec
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_one_flex(self_ptr: *mut mjXReader, elem: *mut tinyxml2__XMLElement, pflex: *mut mjsFlex) {
    todo!() // mjXReader::OneFlex
}

/// C: mjXReader::OneMesh (xml/xml_native_reader.h:71)
/// Calls: FilePath::c_str, FirstChildElement, mjXReader::MeshDir, mjXReader::OnePlugin, mjXUtil::MapValue, mjXUtil::ReadAttr, mjXUtil::ReadAttrFile, mjXUtil::ReadAttrInt, mjXUtil::ReadAttrTxt, mjXUtil::ReadAttrVec, mjXUtil::ReadVector, mjs_getError, mjs_makeMesh, mjs_setFloat, mjs_setInt, mjs_setName, mjs_setString
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_one_mesh(self_ptr: *mut mjXReader, elem: *mut tinyxml2__XMLElement, pmesh: *mut mjsMesh, vfs: *const mjVFS) {
    todo!() // mjXReader::OneMesh
}

/// C: mjXReader::OneSkin (xml/xml_native_reader.h:72)
/// Calls: FilePath::c_str, FirstChildElement, NextSiblingElement, mjXReader::AssetDir, mjXUtil::ReadAttr, mjXUtil::ReadAttrFile, mjXUtil::ReadAttrInt, mjXUtil::ReadAttrTxt, mjXUtil::ReadAttrVec, mjs_appendFloatVec, mjs_appendIntVec, mjs_appendString, mjs_getError, mjs_setFloat, mjs_setInt, mjs_setName, mjs_setString
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_one_skin(self_ptr: *mut mjXReader, elem: *mut tinyxml2__XMLElement, pskin: *mut mjsSkin, vfs: *const mjVFS) {
    todo!() // mjXReader::OneSkin
}

/// C: mjXReader::OneMaterial (xml/xml_native_reader.h:73)
/// Calls: FirstChildElement, NextSiblingElement, mjXUtil::FindKey, mjXUtil::MapValue, mjXUtil::ReadAttr, mjXUtil::ReadAttrTxt, mjs_getError, mjs_setInStringVec, mjs_setName, mjs_setString
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_one_material(self_ptr: *mut mjXReader, elem: *mut tinyxml2__XMLElement, pmaterial: *mut mjsMaterial) {
    todo!() // mjXReader::OneMaterial
}

/// C: mjXReader::OneJoint (xml/xml_native_reader.h:74)
/// Calls: mjXUtil::MapValue, mjXUtil::ReadAttr, mjXUtil::ReadAttrInt, mjXUtil::ReadAttrTxt, mjXUtil::ReadVector, mjs_getError, mjs_setDouble, mjs_setName, mjs_setString
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_one_joint(self_ptr: *mut mjXReader, elem: *mut tinyxml2__XMLElement, pjoint: *mut mjsJoint) {
    todo!() // mjXReader::OneJoint
}

/// C: mjXReader::OneGeom (xml/xml_native_reader.h:75)
/// Calls: FirstChildElement, mjXBase::ReadAlternative, mjXReader::OnePlugin, mjXUtil::MapValue, mjXUtil::ReadAttr, mjXUtil::ReadAttrInt, mjXUtil::ReadAttrTxt, mjXUtil::ReadQuat, mjXUtil::ReadVector, mjs_getError, mjs_setDouble, mjs_setName, mjs_setString
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_one_geom(self_ptr: *mut mjXReader, elem: *mut tinyxml2__XMLElement, pgeom: *mut mjsGeom) {
    todo!() // mjXReader::OneGeom
}

/// C: mjXReader::OneSite (xml/xml_native_reader.h:76)
/// Calls: mjXBase::ReadAlternative, mjXUtil::MapValue, mjXUtil::ReadAttr, mjXUtil::ReadAttrInt, mjXUtil::ReadAttrTxt, mjXUtil::ReadQuat, mjXUtil::ReadVector, mjs_getError, mjs_setDouble, mjs_setName, mjs_setString
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_one_site(self_ptr: *mut mjXReader, elem: *mut tinyxml2__XMLElement, site: *mut mjsSite) {
    todo!() // mjXReader::OneSite
}

/// C: mjXReader::OneCamera (xml/xml_native_reader.h:77)
/// Calls: mjXBase::ReadAlternative, mjXUtil::MapValue, mjXUtil::MapValues, mjXUtil::ReadAttr, mjXUtil::ReadAttrTxt, mjXUtil::ReadQuat, mjXUtil::ReadVector, mjs_getError, mjs_setDouble, mjs_setName, mjs_setString
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_one_camera(self_ptr: *mut mjXReader, elem: *mut tinyxml2__XMLElement, pcamera: *mut mjsCamera) {
    todo!() // mjXReader::OneCamera
}

/// C: mjXReader::OneLight (xml/xml_native_reader.h:78)
/// Calls: mjXUtil::MapValue, mjXUtil::ReadAttr, mjXUtil::ReadAttrTxt, mjs_getError, mjs_setName, mjs_setString
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_one_light(self_ptr: *mut mjXReader, elem: *mut tinyxml2__XMLElement, plight: *mut mjsLight) {
    todo!() // mjXReader::OneLight
}

/// C: mjXReader::OnePair (xml/xml_native_reader.h:79)
/// Calls: mjXUtil::ReadAttr, mjXUtil::ReadAttrInt, mjXUtil::ReadAttrTxt, mjs_getError, mjs_setName, mjs_setString
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_one_pair(self_ptr: *mut mjXReader, elem: *mut tinyxml2__XMLElement, ppair: *mut mjsPair) {
    todo!() // mjXReader::OnePair
}

/// C: mjXReader::OneEquality (xml/xml_native_reader.h:80)
/// Calls: mjXUtil::FindKey, mjXUtil::MapValue, mjXUtil::ReadAttr, mjXUtil::ReadAttrStr, mjXUtil::ReadAttrTxt, mjs_getError, mjs_setName, mjs_setString, mjuu_zerovec
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_one_equality(self_ptr: *mut mjXReader, elem: *mut tinyxml2__XMLElement, pequality: *mut mjsEquality) {
    todo!() // mjXReader::OneEquality
}

/// C: mjXReader::OneTendon (xml/xml_native_reader.h:81)
/// Calls: mjXUtil::MapValue, mjXUtil::ReadAttr, mjXUtil::ReadAttrInt, mjXUtil::ReadAttrTxt, mjXUtil::ReadVector, mjs_getError, mjs_setDouble, mjs_setName, mjs_setString
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_one_tendon(self_ptr: *mut mjXReader, elem: *mut tinyxml2__XMLElement, ptendon: *mut mjsTendon) {
    todo!() // mjXReader::OneTendon
}

/// C: mjXReader::OneActuator (xml/xml_native_reader.h:82)
/// Calls: mjXReader::OnePlugin, mjXUtil::MapValue, mjXUtil::ReadAttr, mjXUtil::ReadAttrInt, mjXUtil::ReadAttrTxt, mjXUtil::ReadVector, mjs_getError, mjs_setDouble, mjs_setName, mjs_setString, mjs_setToAdhesion, mjs_setToCylinder, mjs_setToDCMotor, mjs_setToDamper, mjs_setToIntVelocity, mjs_setToMotor, mjs_setToMuscle, mjs_setToPosition, mjs_setToVelocity
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_one_actuator(self_ptr: *mut mjXReader, elem: *mut tinyxml2__XMLElement, pactuator: *mut mjsActuator) {
    todo!() // mjXReader::OneActuator
}

/// C: mjXReader::OneComposite (xml/xml_native_reader.h:83)
/// Calls: FirstChildElement, NextSiblingElement, mjCComposite::AddDefaultJoint, mjCComposite::Make, mjCComposite::SetDefault, mjXReader::OnePlugin, mjXUtil::FindKey, mjXUtil::MapValue, mjXUtil::ReadAttr, mjXUtil::ReadAttrInt, mjXUtil::ReadAttrTxt, mjXUtil::ReadAttrVec, mjs_setString
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_one_composite(self_ptr: *mut mjXReader, elem: *mut tinyxml2__XMLElement, pbody: *mut mjsBody, pframe: *mut mjsFrame, def: *const mjsDefault) {
    todo!() // mjXReader::OneComposite
}

/// C: mjXReader::OneFlexcomp (xml/xml_native_reader.h:85)
/// Calls: FilePath::Str, FirstChildElement, NextSiblingElement, mjCFlexcomp::Make, mjXBase::ReadAlternative, mjXReader::OnePlugin, mjXUtil::MapValue, mjXUtil::ReadAttr, mjXUtil::ReadAttrFile, mjXUtil::ReadAttrInt, mjXUtil::ReadAttrTxt, mjXUtil::ReadAttrVec, mjs_setString
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_one_flexcomp(self_ptr: *mut mjXReader, elem: *mut tinyxml2__XMLElement, pbody: *mut mjsBody, vfs: *const mjVFS) {
    todo!() // mjXReader::OneFlexcomp
}

/// C: mjXReader::OnePlugin (xml/xml_native_reader.h:86)
/// Calls: ReadPluginConfigs, mjXUtil::ReadAttrTxt, mjs_addPlugin, mjs_setString
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_one_plugin(self_ptr: *mut mjXReader, elem: *mut tinyxml2__XMLElement, plugin: *mut mjsPlugin) {
    todo!() // mjXReader::OnePlugin
}

/// C: mjXReader::GetClass (xml/xml_native_reader.h:89)
/// Calls: mjXUtil::ReadAttrTxt, mjs_findDefault
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_get_class(self_ptr: *mut mjXReader, section: *mut tinyxml2__XMLElement) -> *const mjsDefault {
    todo!() // mjXReader::GetClass
}

/// C: mjXReader::AssetDir (xml/xml_native_reader.h:94)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_asset_dir(self_ptr: *mut mjXReader) -> mujoco__user__FilePath {
    todo!() // mjXReader::AssetDir
}

/// C: mjXReader::MeshDir (xml/xml_native_reader.h:95)
/// Calls: FilePath::empty, mjXReader::AssetDir
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_mesh_dir(self_ptr: *mut mjXReader) -> mujoco__user__FilePath {
    todo!() // mjXReader::MeshDir
}

/// C: mjXReader::TextureDir (xml/xml_native_reader.h:96)
/// Calls: FilePath::empty, mjXReader::AssetDir
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_texture_dir(self_ptr: *mut mjXReader) -> mujoco__user__FilePath {
    todo!() // mjXReader::TextureDir
}

