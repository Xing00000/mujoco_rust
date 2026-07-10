//! Port of: xml/xml_native_reader.cc
//! IR hash: 05737965add36adb
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: GetAttrPtr (xml/xml_native_reader.cc:59)
#[allow(unused_variables, non_snake_case)]
pub fn get_attr_ptr(val: *mut T) -> i32 {
    if val.is_null() {
        return 0;
    }
    extern "C" { fn GetAttrPtr(val: *mut T) -> i32; }
    unsafe { GetAttrPtr(val) }
}

/// C: Reader::txt (xml/xml_native_reader.cc:111)
/// Calls: mjXUtil::ReadAttrTxt
#[allow(unused_variables, non_snake_case)]
pub fn reader_txt(self_ptr: *mut Reader, attr: *const i8, target: *mut T, set_func: void_____T____const_char) -> bool {
    extern "C" { fn Reader_txt(self_ptr: *mut Reader, attr: *const i8, target: *mut T, set_func: void_____T____const_char) -> bool; }
    // SAFETY: delegates to C implementation
    unsafe { Reader_txt(self_ptr, attr, target, set_func) }
}

/// C: Reader::set_node (xml/xml_native_reader.cc:121)
#[allow(unused_variables, non_snake_case)]
pub fn reader_set_node(self_ptr: *mut Reader, node: *mut XMLElement) {
    if self_ptr.is_null() { return; }
    extern "C" { fn Reader_set_node(self_ptr: *mut Reader, node: *mut XMLElement); }
    // SAFETY: self_ptr verified non-null
    unsafe { Reader_set_node(self_ptr, node) }
}

/// C: ReadPluginConfigs (xml/xml_native_reader.cc:128)
/// Calls: FirstChildElement, NextSiblingElement, mjXUtil::ReadAttrTxt
#[allow(unused_variables, non_snake_case)]
pub fn read_plugin_configs(elem: *mut tinyxml2__XMLElement, p: *mut mjsPlugin) {
    extern "C" { fn ReadPluginConfigs(elem: *mut tinyxml2__XMLElement, p: *mut mjsPlugin); }
    // SAFETY: delegates to C implementation
    unsafe { ReadPluginConfigs(elem, p) }
}

/// C: UpdateString (xml/xml_native_reader.cc:155)
#[allow(unused_variables, non_snake_case)]
pub fn update_string(psuffix: *mut string, count: i32, i: i32) {
    extern "C" { fn UpdateString(psuffix: *mut string, count: i32, i: i32); }
    // SAFETY: delegates to C implementation
    unsafe { UpdateString(psuffix, count, i) }
}

/// C: stripError (xml/xml_native_reader.cc:3878)
#[allow(unused_variables, non_snake_case)]
pub fn strip_error(err: *const i8) -> *const i8 {
    extern "C" { fn stripError(err: *const i8) -> *const i8; }
    // SAFETY: delegates to C implementation
    unsafe { stripError(err) }
}

/// C: mjXReader::Parse (xml/xml_native_reader.h:34)
/// Calls: FirstChildElement, NextSiblingElement, mjXReader::Actuator, mjXReader::Asset, mjXReader::Body, mjXReader::Contact, mjXReader::Custom, mjXReader::Default, mjXReader::Deformable, mjXReader::Equality, mjXReader::Extension, mjXReader::Keyframe, mjXReader::Sensor, mjXReader::Statistic, mjXReader::Tendon, mjXReader::Visual, mjXUtil::ReadAttrTxt
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_parse(self_ptr: *mut mjXReader, root: *mut tinyxml2__XMLElement, vfs: *const mjVFS) {
    extern "C" { fn mjXReader_Parse(self_ptr: *mut mjXReader, root: *mut tinyxml2__XMLElement, vfs: *const mjVFS); }
    // SAFETY: delegates to C++ implementation; caller guarantees pointer validity
    unsafe { mjXReader_Parse(self_ptr, root, vfs) }
}

/// C: mjXReader::PrintSchema (xml/xml_native_reader.h:35)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_print_schema(self_ptr: *mut mjXReader, str: *mut std__stringstream, html: bool, pad: bool) {
    extern "C" { fn mjXReader_PrintSchema(self_ptr: *mut mjXReader, str: *mut std__stringstream, html: bool, pad: bool); }
    // SAFETY: delegates to C implementation
    unsafe { mjXReader_PrintSchema(self_ptr, str, html, pad) }
}

/// C: mjXReader::SetModelFileDir (xml/xml_native_reader.h:37)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_set_model_file_dir(self_ptr: *mut mjXReader, modelfiledir: *const std__string) {
    extern "C" { fn mjXReader_SetModelFileDir(self_ptr: *mut mjXReader, modelfiledir: *const std__string); }
    // SAFETY: delegates to C implementation
    unsafe { mjXReader_SetModelFileDir(self_ptr, modelfiledir) }
}

/// C: mjXReader::ModelFileDir (xml/xml_native_reader.h:38)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_model_file_dir(self_ptr: *mut mjXReader) -> *const mujoco__user__FilePath {
    if self_ptr.is_null() {
        return core::ptr::null();
    }
    extern "C" { fn mjXReader_ModelFileDir(self_ptr: *mut mjXReader) -> *const mujoco__user__FilePath; }
    // SAFETY: self_ptr verified non-null; delegates to C++ method
    unsafe { mjXReader_ModelFileDir(self_ptr) }
}

/// C: mjXReader::SetAssetDir (xml/xml_native_reader.h:41)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_set_asset_dir(self_ptr: *mut mjXReader, assetdir: *const std__string) {
    extern "C" { fn mjXReader_SetAssetDir(self_ptr: *mut mjXReader, assetdir: *const std__string); }
    // SAFETY: delegates to C implementation
    unsafe { mjXReader_SetAssetDir(self_ptr, assetdir) }
}

/// C: mjXReader::SetMeshDir (xml/xml_native_reader.h:42)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_set_mesh_dir(self_ptr: *mut mjXReader, meshdir: *const std__string) {
    extern "C" { fn mjXReader_SetMeshDir(self_ptr: *mut mjXReader, meshdir: *const std__string); }
    // SAFETY: delegates to C implementation
    unsafe { mjXReader_SetMeshDir(self_ptr, meshdir) }
}

/// C: mjXReader::SetTextureDir (xml/xml_native_reader.h:43)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_set_texture_dir(self_ptr: *mut mjXReader, texturedir: *const std__string) {
    extern "C" { fn mjXReader_SetTextureDir(self_ptr: *mut mjXReader, texturedir: *const std__string); }
    // SAFETY: delegates to C implementation
    unsafe { mjXReader_SetTextureDir(self_ptr, texturedir) }
}

/// C: mjXReader::Compiler (xml/xml_native_reader.h:46)
/// Calls: mjXUtil::MapValue, mjXUtil::ReadAttrTxt, mjs_setAuthored
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_compiler(section: *mut tinyxml2__XMLElement, s: *mut mjSpec) {
    extern "C" { fn mjXReader_Compiler(section: *mut tinyxml2__XMLElement, s: *mut mjSpec); }
    // SAFETY: delegates to C implementation
    unsafe { mjXReader_Compiler(section, s) }
}

/// C: mjXReader::Option (xml/xml_native_reader.h:47)
/// Calls: mjXUtil::MapValue
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_option(section: *mut tinyxml2__XMLElement, s: *mut mjSpec, opt: *mut mjOption) {
    extern "C" { fn mjXReader_Option(section: *mut tinyxml2__XMLElement, s: *mut mjSpec, opt: *mut mjOption); }
    // SAFETY: delegates to C implementation
    unsafe { mjXReader_Option(section, s, opt) }
}

/// C: mjXReader::Size (xml/xml_native_reader.h:48)
/// Calls: mjXUtil::ReadAttrInt
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_size(section: *mut tinyxml2__XMLElement, s: *mut mjSpec) {
    extern "C" { fn mjXReader_Size(section: *mut tinyxml2__XMLElement, s: *mut mjSpec); }
    // SAFETY: delegates to C implementation
    unsafe { mjXReader_Size(section, s) }
}

/// C: mjXReader::Default (xml/xml_native_reader.h:52)
/// Calls: FirstChildElement, NextSiblingElement, mjXReader::OneActuator, mjXReader::OneCamera, mjXReader::OneEquality, mjXReader::OneGeom, mjXReader::OneJoint, mjXReader::OneLight, mjXReader::OneMaterial, mjXReader::OneMesh, mjXReader::OnePair, mjXReader::OneSite, mjXReader::OneTendon, mjXUtil::ReadAttrTxt
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_default(self_ptr: *mut mjXReader, section: *mut tinyxml2__XMLElement, def: *const mjsDefault, vfs: *const mjVFS) {
    extern "C" { fn mjXReader_Default(self_ptr: *mut mjXReader, section: *mut tinyxml2__XMLElement, def: *const mjsDefault, vfs: *const mjVFS); }
    // SAFETY: delegates to C implementation
    unsafe { mjXReader_Default(self_ptr, section, def, vfs) }
}

/// C: mjXReader::Extension (xml/xml_native_reader.h:54)
/// Calls: FirstChildElement, NextSiblingElement, ReadPluginConfigs, mjXUtil::ReadAttrTxt
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_extension(self_ptr: *mut mjXReader, section: *mut tinyxml2__XMLElement) {
    extern "C" { fn mjXReader_Extension(self_ptr: *mut mjXReader, section: *mut tinyxml2__XMLElement); }
    // SAFETY: delegates to C implementation
    unsafe { mjXReader_Extension(self_ptr, section) }
}

/// C: mjXReader::Custom (xml/xml_native_reader.h:55)
/// Calls: FirstChildElement, NextSiblingElement, mjXUtil::ReadAttrInt, mjXUtil::ReadAttrTxt
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_custom(self_ptr: *mut mjXReader, section: *mut tinyxml2__XMLElement) {
    extern "C" { fn mjXReader_Custom(self_ptr: *mut mjXReader, section: *mut tinyxml2__XMLElement); }
    // SAFETY: delegates to C implementation
    unsafe { mjXReader_Custom(self_ptr, section) }
}

/// C: mjXReader::Visual (xml/xml_native_reader.h:56)
/// Calls: FirstChildElement, NextSiblingElement, Reader::set_node
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_visual(self_ptr: *mut mjXReader, section: *mut tinyxml2__XMLElement) {
    extern "C" { fn mjXReader_Visual(self_ptr: *mut mjXReader, section: *mut tinyxml2__XMLElement); }
    // SAFETY: delegates to C implementation
    unsafe { mjXReader_Visual(self_ptr, section) }
}

/// C: mjXReader::Statistic (xml/xml_native_reader.h:57)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_statistic(self_ptr: *mut mjXReader, section: *mut tinyxml2__XMLElement) {
    extern "C" { fn mjXReader_Statistic(self_ptr: *mut mjXReader, section: *mut tinyxml2__XMLElement); }
    // SAFETY: delegates to C implementation
    unsafe { mjXReader_Statistic(self_ptr, section) }
}

/// C: mjXReader::Asset (xml/xml_native_reader.h:58)
/// Calls: FirstChildElement, NextSiblingElement, mjXReader::AssetDir, mjXReader::GetClass, mjXReader::OneMaterial, mjXReader::OneMesh, mjXReader::OneSkin, mjXReader::TextureDir, mjXUtil::MapValue, mjXUtil::ReadAttrFile, mjXUtil::ReadAttrInt, mjXUtil::ReadAttrTxt
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_asset(self_ptr: *mut mjXReader, section: *mut tinyxml2__XMLElement, vfs: *const mjVFS) {
    extern "C" { fn mjXReader_Asset(self_ptr: *mut mjXReader, section: *mut tinyxml2__XMLElement, vfs: *const mjVFS); }
    // SAFETY: delegates to C implementation
    unsafe { mjXReader_Asset(self_ptr, section, vfs) }
}

/// C: mjXReader::Body (xml/xml_native_reader.h:59)
/// Calls: FirstChildElement, NextSiblingElement, mjXBase::ReadAlternative, mjXReader::GetClass, mjXReader::OneCamera, mjXReader::OneComposite, mjXReader::OneFlexcomp, mjXReader::OneGeom, mjXReader::OneJoint, mjXReader::OneLight, mjXReader::OnePlugin, mjXReader::OneSite, mjXUtil::MapValue, mjXUtil::ReadAttrInt, mjXUtil::ReadAttrTxt, mjXUtil::ReadQuat, mjs_addBody, mjs_addCamera, mjs_addFrame, mjs_addFreeJoint, mjs_addGeom, mjs_addJoint, mjs_addLight, mjs_addSite, mjs_defaultOrientation, mjs_getDefault, mjs_getId, mjs_setDefault, mjs_setFrame, mjuu_frameaccum, mjuu_setvec
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_body(self_ptr: *mut mjXReader, section: *mut tinyxml2__XMLElement, pbody: *mut mjsBody, pframe: *mut mjsFrame, vfs: *const mjVFS) {
    extern "C" { fn mjXReader_Body(self_ptr: *mut mjXReader, section: *mut tinyxml2__XMLElement, pbody: *mut mjsBody, pframe: *mut mjsFrame, vfs: *const mjVFS); }
    // SAFETY: delegates to C++ implementation; caller guarantees pointer validity
    unsafe { mjXReader_Body(self_ptr, section, pbody, pframe, vfs) }
}

/// C: mjXReader::Contact (xml/xml_native_reader.h:61)
/// Calls: FirstChildElement, NextSiblingElement, mjXReader::GetClass, mjXReader::OnePair, mjXUtil::ReadAttrTxt
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_contact(self_ptr: *mut mjXReader, section: *mut tinyxml2__XMLElement) {
    extern "C" { fn mjXReader_Contact(self_ptr: *mut mjXReader, section: *mut tinyxml2__XMLElement); }
    // SAFETY: delegates to C implementation
    unsafe { mjXReader_Contact(self_ptr, section) }
}

/// C: mjXReader::Deformable (xml/xml_native_reader.h:62)
/// Calls: FirstChildElement, NextSiblingElement, mjXReader::GetClass, mjXReader::OneFlex, mjXReader::OneSkin
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_deformable(self_ptr: *mut mjXReader, section: *mut tinyxml2__XMLElement, vfs: *const mjVFS) {
    extern "C" { fn mjXReader_Deformable(self_ptr: *mut mjXReader, section: *mut tinyxml2__XMLElement, vfs: *const mjVFS); }
    // SAFETY: delegates to C implementation
    unsafe { mjXReader_Deformable(self_ptr, section, vfs) }
}

/// C: mjXReader::Equality (xml/xml_native_reader.h:63)
/// Calls: FirstChildElement, NextSiblingElement, mjXReader::GetClass, mjXReader::OneEquality
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_equality(self_ptr: *mut mjXReader, section: *mut tinyxml2__XMLElement) {
    extern "C" { fn mjXReader_Equality(self_ptr: *mut mjXReader, section: *mut tinyxml2__XMLElement); }
    // SAFETY: delegates to C implementation
    unsafe { mjXReader_Equality(self_ptr, section) }
}

/// C: mjXReader::Tendon (xml/xml_native_reader.h:64)
/// Calls: FirstChildElement, NextSiblingElement, mjXReader::GetClass, mjXReader::OneTendon, mjXUtil::ReadAttrTxt, mjs_wrapPulley
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_tendon(self_ptr: *mut mjXReader, section: *mut tinyxml2__XMLElement) {
    extern "C" { fn mjXReader_Tendon(self_ptr: *mut mjXReader, section: *mut tinyxml2__XMLElement); }
    // SAFETY: delegates to C implementation
    unsafe { mjXReader_Tendon(self_ptr, section) }
}

/// C: mjXReader::Actuator (xml/xml_native_reader.h:65)
/// Calls: FirstChildElement, NextSiblingElement, mjXReader::GetClass, mjXReader::OneActuator
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_actuator(self_ptr: *mut mjXReader, section: *mut tinyxml2__XMLElement) {
    extern "C" { fn mjXReader_Actuator(self_ptr: *mut mjXReader, section: *mut tinyxml2__XMLElement); }
    // SAFETY: delegates to C implementation
    unsafe { mjXReader_Actuator(self_ptr, section) }
}

/// C: mjXReader::Sensor (xml/xml_native_reader.h:66)
/// Calls: FirstChildElement, NextSiblingElement, mjXReader::OnePlugin, mjXUtil::MapValue, mjXUtil::MapValues, mjXUtil::ReadAttrInt, mjXUtil::ReadAttrTxt
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_sensor(self_ptr: *mut mjXReader, section: *mut tinyxml2__XMLElement) {
    extern "C" { fn mjXReader_Sensor(self_ptr: *mut mjXReader, section: *mut tinyxml2__XMLElement); }
    // SAFETY: delegates to C implementation
    unsafe { mjXReader_Sensor(self_ptr, section) }
}

/// C: mjXReader::Keyframe (xml/xml_native_reader.h:67)
/// Calls: FirstChildElement, NextSiblingElement, mjXUtil::ReadAttrTxt
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_keyframe(self_ptr: *mut mjXReader, section: *mut tinyxml2__XMLElement) {
    extern "C" { fn mjXReader_Keyframe(self_ptr: *mut mjXReader, section: *mut tinyxml2__XMLElement); }
    // SAFETY: delegates to C implementation
    unsafe { mjXReader_Keyframe(self_ptr, section) }
}

/// C: mjXReader::OneFlex (xml/xml_native_reader.h:70)
/// Calls: FirstChildElement, mjXUtil::MapValue, mjXUtil::ReadAttrInt, mjXUtil::ReadAttrTxt
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_one_flex(self_ptr: *mut mjXReader, elem: *mut tinyxml2__XMLElement, pflex: *mut mjsFlex) {
    extern "C" { fn mjXReader_OneFlex(self_ptr: *mut mjXReader, elem: *mut tinyxml2__XMLElement, pflex: *mut mjsFlex); }
    // SAFETY: delegates to C implementation
    unsafe { mjXReader_OneFlex(self_ptr, elem, pflex) }
}

/// C: mjXReader::OneMesh (xml/xml_native_reader.h:71)
/// Calls: FirstChildElement, mjXReader::MeshDir, mjXReader::OnePlugin, mjXUtil::MapValue, mjXUtil::ReadAttrInt, mjXUtil::ReadAttrTxt
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_one_mesh(self_ptr: *mut mjXReader, elem: *mut tinyxml2__XMLElement, pmesh: *mut mjsMesh, vfs: *const mjVFS) {
    extern "C" { fn mjXReader_OneMesh(self_ptr: *mut mjXReader, elem: *mut tinyxml2__XMLElement, pmesh: *mut mjsMesh, vfs: *const mjVFS); }
    // SAFETY: delegates to C implementation
    unsafe { mjXReader_OneMesh(self_ptr, elem, pmesh, vfs) }
}

/// C: mjXReader::OneSkin (xml/xml_native_reader.h:72)
/// Calls: FirstChildElement, NextSiblingElement, mjXReader::AssetDir, mjXUtil::ReadAttrInt, mjXUtil::ReadAttrTxt
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_one_skin(self_ptr: *mut mjXReader, elem: *mut tinyxml2__XMLElement, pskin: *mut mjsSkin, vfs: *const mjVFS) {
    extern "C" { fn mjXReader_OneSkin(self_ptr: *mut mjXReader, elem: *mut tinyxml2__XMLElement, pskin: *mut mjsSkin, vfs: *const mjVFS); }
    // SAFETY: delegates to C implementation
    unsafe { mjXReader_OneSkin(self_ptr, elem, pskin, vfs) }
}

/// C: mjXReader::OneMaterial (xml/xml_native_reader.h:73)
/// Calls: FirstChildElement, NextSiblingElement, mjXUtil::FindKey, mjXUtil::MapValue, mjXUtil::ReadAttrTxt
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_one_material(self_ptr: *mut mjXReader, elem: *mut tinyxml2__XMLElement, pmaterial: *mut mjsMaterial) {
    extern "C" { fn mjXReader_OneMaterial(self_ptr: *mut mjXReader, elem: *mut tinyxml2__XMLElement, pmaterial: *mut mjsMaterial); }
    // SAFETY: delegates to C implementation
    unsafe { mjXReader_OneMaterial(self_ptr, elem, pmaterial) }
}

/// C: mjXReader::OneJoint (xml/xml_native_reader.h:74)
/// Calls: mjXUtil::MapValue, mjXUtil::ReadAttrInt, mjXUtil::ReadAttrTxt
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_one_joint(self_ptr: *mut mjXReader, elem: *mut tinyxml2__XMLElement, pjoint: *mut mjsJoint) {
    extern "C" { fn mjXReader_OneJoint(self_ptr: *mut mjXReader, elem: *mut tinyxml2__XMLElement, pjoint: *mut mjsJoint); }
    // SAFETY: delegates to C implementation
    unsafe { mjXReader_OneJoint(self_ptr, elem, pjoint) }
}

/// C: mjXReader::OneGeom (xml/xml_native_reader.h:75)
/// Calls: FirstChildElement, mjXBase::ReadAlternative, mjXReader::OnePlugin, mjXUtil::MapValue, mjXUtil::ReadAttrInt, mjXUtil::ReadAttrTxt, mjXUtil::ReadQuat
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_one_geom(self_ptr: *mut mjXReader, elem: *mut tinyxml2__XMLElement, pgeom: *mut mjsGeom) {
    extern "C" { fn mjXReader_OneGeom(self_ptr: *mut mjXReader, elem: *mut tinyxml2__XMLElement, pgeom: *mut mjsGeom); }
    // SAFETY: delegates to C implementation
    unsafe { mjXReader_OneGeom(self_ptr, elem, pgeom) }
}

/// C: mjXReader::OneSite (xml/xml_native_reader.h:76)
/// Calls: mjXBase::ReadAlternative, mjXUtil::MapValue, mjXUtil::ReadAttrInt, mjXUtil::ReadAttrTxt, mjXUtil::ReadQuat
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_one_site(self_ptr: *mut mjXReader, elem: *mut tinyxml2__XMLElement, site: *mut mjsSite) {
    extern "C" { fn mjXReader_OneSite(self_ptr: *mut mjXReader, elem: *mut tinyxml2__XMLElement, site: *mut mjsSite); }
    // SAFETY: delegates to C implementation
    unsafe { mjXReader_OneSite(self_ptr, elem, site) }
}

/// C: mjXReader::OneCamera (xml/xml_native_reader.h:77)
/// Calls: mjXBase::ReadAlternative, mjXUtil::MapValue, mjXUtil::MapValues, mjXUtil::ReadAttrTxt, mjXUtil::ReadQuat
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_one_camera(self_ptr: *mut mjXReader, elem: *mut tinyxml2__XMLElement, pcamera: *mut mjsCamera) {
    extern "C" { fn mjXReader_OneCamera(self_ptr: *mut mjXReader, elem: *mut tinyxml2__XMLElement, pcamera: *mut mjsCamera); }
    // SAFETY: delegates to C implementation
    unsafe { mjXReader_OneCamera(self_ptr, elem, pcamera) }
}

/// C: mjXReader::OneLight (xml/xml_native_reader.h:78)
/// Calls: mjXUtil::MapValue, mjXUtil::ReadAttrTxt
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_one_light(self_ptr: *mut mjXReader, elem: *mut tinyxml2__XMLElement, plight: *mut mjsLight) {
    extern "C" { fn mjXReader_OneLight(self_ptr: *mut mjXReader, elem: *mut tinyxml2__XMLElement, plight: *mut mjsLight); }
    // SAFETY: delegates to C implementation
    unsafe { mjXReader_OneLight(self_ptr, elem, plight) }
}

/// C: mjXReader::OnePair (xml/xml_native_reader.h:79)
/// Calls: mjXUtil::ReadAttrInt, mjXUtil::ReadAttrTxt
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_one_pair(self_ptr: *mut mjXReader, elem: *mut tinyxml2__XMLElement, ppair: *mut mjsPair) {
    extern "C" { fn mjXReader_OnePair(self_ptr: *mut mjXReader, elem: *mut tinyxml2__XMLElement, ppair: *mut mjsPair); }
    // SAFETY: delegates to C implementation
    unsafe { mjXReader_OnePair(self_ptr, elem, ppair) }
}

/// C: mjXReader::OneEquality (xml/xml_native_reader.h:80)
/// Calls: mjXUtil::FindKey, mjXUtil::MapValue, mjXUtil::ReadAttrTxt, mjuu_zerovec
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_one_equality(self_ptr: *mut mjXReader, elem: *mut tinyxml2__XMLElement, pequality: *mut mjsEquality) {
    extern "C" { fn mjXReader_OneEquality(self_ptr: *mut mjXReader, elem: *mut tinyxml2__XMLElement, pequality: *mut mjsEquality); }
    // SAFETY: delegates to C implementation
    unsafe { mjXReader_OneEquality(self_ptr, elem, pequality) }
}

/// C: mjXReader::OneTendon (xml/xml_native_reader.h:81)
/// Calls: mjXUtil::MapValue, mjXUtil::ReadAttrInt, mjXUtil::ReadAttrTxt
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_one_tendon(self_ptr: *mut mjXReader, elem: *mut tinyxml2__XMLElement, ptendon: *mut mjsTendon) {
    extern "C" { fn mjXReader_OneTendon(self_ptr: *mut mjXReader, elem: *mut tinyxml2__XMLElement, ptendon: *mut mjsTendon); }
    // SAFETY: delegates to C implementation
    unsafe { mjXReader_OneTendon(self_ptr, elem, ptendon) }
}

/// C: mjXReader::OneActuator (xml/xml_native_reader.h:82)
/// Calls: mjXReader::OnePlugin, mjXUtil::MapValue, mjXUtil::ReadAttrInt, mjXUtil::ReadAttrTxt, mjs_setToAdhesion, mjs_setToCylinder, mjs_setToDCMotor, mjs_setToDamper, mjs_setToIntVelocity, mjs_setToMotor, mjs_setToMuscle, mjs_setToPosition, mjs_setToVelocity
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_one_actuator(self_ptr: *mut mjXReader, elem: *mut tinyxml2__XMLElement, pactuator: *mut mjsActuator) {
    extern "C" { fn mjXReader_OneActuator(self_ptr: *mut mjXReader, elem: *mut tinyxml2__XMLElement, pactuator: *mut mjsActuator); }
    // SAFETY: delegates to C implementation
    unsafe { mjXReader_OneActuator(self_ptr, elem, pactuator) }
}

/// C: mjXReader::OneComposite (xml/xml_native_reader.h:83)
/// Calls: FirstChildElement, NextSiblingElement, mjCComposite::AddDefaultJoint, mjCComposite::SetDefault, mjXReader::OnePlugin, mjXUtil::FindKey, mjXUtil::MapValue, mjXUtil::ReadAttrInt, mjXUtil::ReadAttrTxt
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_one_composite(self_ptr: *mut mjXReader, elem: *mut tinyxml2__XMLElement, pbody: *mut mjsBody, pframe: *mut mjsFrame, def: *const mjsDefault) {
    extern "C" { fn mjXReader_OneComposite(self_ptr: *mut mjXReader, elem: *mut tinyxml2__XMLElement, pbody: *mut mjsBody, pframe: *mut mjsFrame, def: *const mjsDefault); }
    // SAFETY: delegates to C implementation
    unsafe { mjXReader_OneComposite(self_ptr, elem, pbody, pframe, def) }
}

/// C: mjXReader::OneFlexcomp (xml/xml_native_reader.h:85)
/// Calls: FirstChildElement, NextSiblingElement, mjCFlexcomp::Make, mjXBase::ReadAlternative, mjXReader::OnePlugin, mjXUtil::MapValue, mjXUtil::ReadAttrInt, mjXUtil::ReadAttrTxt
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_one_flexcomp(self_ptr: *mut mjXReader, elem: *mut tinyxml2__XMLElement, pbody: *mut mjsBody, vfs: *const mjVFS) {
    extern "C" { fn mjXReader_OneFlexcomp(self_ptr: *mut mjXReader, elem: *mut tinyxml2__XMLElement, pbody: *mut mjsBody, vfs: *const mjVFS); }
    // SAFETY: delegates to C++ implementation; caller guarantees pointer validity
    unsafe { mjXReader_OneFlexcomp(self_ptr, elem, pbody, vfs) }
}

/// C: mjXReader::OnePlugin (xml/xml_native_reader.h:86)
/// Calls: ReadPluginConfigs, mjXUtil::ReadAttrTxt
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_one_plugin(self_ptr: *mut mjXReader, elem: *mut tinyxml2__XMLElement, plugin: *mut mjsPlugin) {
    extern "C" { fn mjXReader_OnePlugin(self_ptr: *mut mjXReader, elem: *mut tinyxml2__XMLElement, plugin: *mut mjsPlugin); }
    // SAFETY: delegates to C implementation
    unsafe { mjXReader_OnePlugin(self_ptr, elem, plugin) }
}

/// C: mjXReader::GetClass (xml/xml_native_reader.h:89)
/// Calls: mjXUtil::ReadAttrTxt
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_get_class(self_ptr: *mut mjXReader, section: *mut tinyxml2__XMLElement) -> *const mjsDefault {
    extern "C" { fn mjXReader_GetClass(self_ptr: *mut mjXReader, section: *mut tinyxml2__XMLElement) -> *const mjsDefault; }
    // SAFETY: delegates to C++ implementation, all pointers valid per caller contract
    unsafe { mjXReader_GetClass(self_ptr, section) }
}

/// C: mjXReader::AssetDir (xml/xml_native_reader.h:94)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_asset_dir(self_ptr: *mut mjXReader) -> mujoco__user__FilePath {
    extern "C" { fn mjXReader_AssetDir(self_ptr: *mut mjXReader) -> mujoco__user__FilePath; }
    // SAFETY: delegates to C++ implementation; caller guarantees self_ptr is valid
    unsafe { mjXReader_AssetDir(self_ptr) }
}

/// C: mjXReader::MeshDir (xml/xml_native_reader.h:95)
/// Calls: mjXReader::AssetDir
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_mesh_dir(self_ptr: *mut mjXReader) -> mujoco__user__FilePath {
    extern "C" { fn mjXReader_MeshDir(self_ptr: *mut mjXReader) -> mujoco__user__FilePath; }
    // SAFETY: delegates to C implementation
    unsafe { mjXReader_MeshDir(self_ptr) }
}

/// C: mjXReader::TextureDir (xml/xml_native_reader.h:96)
/// Calls: mjXReader::AssetDir
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_texture_dir(self_ptr: *mut mjXReader) -> mujoco__user__FilePath {
    extern "C" { fn mjXReader_TextureDir(self_ptr: *mut mjXReader) -> mujoco__user__FilePath; }
    // SAFETY: delegates to C implementation
    unsafe { mjXReader_TextureDir(self_ptr) }
}

