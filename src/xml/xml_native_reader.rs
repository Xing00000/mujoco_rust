//! Port of: xml/xml_native_reader.cc
//! IR hash: fd5288ad7242f001
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: GetAttrPtr (xml/xml_native_reader.cc:68)
#[allow(unused_variables, non_snake_case)]
pub fn get_attr_ptr(arr: [int; 31]) -> *mut i32 {
    todo!() // cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/xml/xml_native_reader.cc:__ZN12_GLOBAL__N_110GetAttrPtrIiLm31EEEPT_RAT0__S1_
}

/// C: Reader::set_node (xml/xml_native_reader.cc:122)
#[allow(unused_variables, non_snake_case)]
pub fn reader_set_node(self_ptr: *mut Reader, node: *mut tinyxml2__XMLElement) {
    // MACHINE-PROVEN: Clang AST semantics
    unsafe { (*self_ptr).xml_node_ = node; }
}

/// C: ReadPluginConfigs (xml/xml_native_reader.cc:129)
/// Calls: cxx:__Z17FirstChildElementPN8tinyxml210XMLElementEPKc, cxx:__Z18NextSiblingElementPN8tinyxml210XMLElementEPKc, cxx:_mjs_setPluginAttributes
#[allow(unused_variables, non_snake_case)]
pub fn read_plugin_configs(elem: *mut tinyxml2__XMLElement, p: *mut mjsPlugin) {
    todo!() // cxx-internal:xml_native_reader.cc.o:__ZN12_GLOBAL__N_117ReadPluginConfigsEPN8tinyxml210XMLElementEP10mjsPlugin_
}

/// C: UpdateString (xml/xml_native_reader.cc:157)
#[allow(unused_variables, non_snake_case)]
pub fn update_string(psuffix: *mut anonymous_namespace___string, count: i32, i: i32) {
    todo!() // cxx-internal:xml_native_reader.cc.o:__ZN12_GLOBAL__N_112UpdateStringERNSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEEii
}

/// C: mjXReader::PrintSchema (xml/xml_native_reader.cc:183)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_print_schema(self_ptr: *mut mjXReader, str: *mut std__stringstream, html: bool, pad: bool) {
    todo!() // cxx:__ZN9mjXReader11PrintSchemaERNSt3__118basic_stringstreamIcNS0_11char_traitsIcEENS0_9allocatorIcEEEEbb
}

/// C: mjXReader::Parse (xml/xml_native_reader.cc:194)
/// Calls: cxx:__Z17FirstChildElementPN8tinyxml210XMLElementEPKc, cxx:__Z18NextSiblingElementPN8tinyxml210XMLElementEPKc, cxx:_mjs_findBody, cxx:_mjs_setDeepCopy, cxx:_mjs_setString
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_parse(self_ptr: *mut mjXReader, root: *mut tinyxml2__XMLElement, vfs: *const mjVFS) {
    todo!() // cxx:__ZN9mjXReader5ParseEPN8tinyxml210XMLElementEPK6mjVFS_
}

/// C: mjXReader::Compiler (xml/xml_native_reader.cc:317)
/// Calls: cxx:_mjs_setString
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_compiler(self_ptr: *mut mjXReader, section: *mut tinyxml2__XMLElement, s: *mut mjSpec) {
    todo!() // cxx:__ZN9mjXReader8CompilerEPN8tinyxml210XMLElementEP7mjSpec_
}

/// C: mjXReader::Option (xml/xml_native_reader.cc:356)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_option(self_ptr: *mut mjXReader, section: *mut tinyxml2__XMLElement, s: *mut mjSpec, opt: *mut mjOption) {
    todo!() // cxx:__ZN9mjXReader6OptionEPN8tinyxml210XMLElementEP7mjSpec_P9mjOption_
}

/// C: mjXReader::Size (xml/xml_native_reader.cc:437)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_size(self_ptr: *mut mjXReader, section: *mut tinyxml2__XMLElement, s: *mut mjSpec) {
    todo!() // cxx:__ZN9mjXReader4SizeEPN8tinyxml210XMLElementEP7mjSpec_
}

/// C: mjXReader::Statistic (xml/xml_native_reader.cc:559)
/// Calls: cxx:__Z12mjuu_definedd
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_statistic(self_ptr: *mut mjXReader, section: *mut tinyxml2__XMLElement) {
    todo!() // cxx:__ZN9mjXReader9StatisticEPN8tinyxml210XMLElementE
}

/// C: mjXReader::OneFlex (xml/xml_native_reader.cc:575)
/// Calls: cxx:__Z17FirstChildElementPN8tinyxml210XMLElementEPKc, cxx:_mjs_setString
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_one_flex(self_ptr: *mut mjXReader, elem: *mut tinyxml2__XMLElement, flex: *mut mjsFlex) {
    todo!() // cxx:__ZN9mjXReader7OneFlexEPN8tinyxml210XMLElementEP8mjsFlex_
}

/// C: mjXReader::OneMesh (xml/xml_native_reader.cc:612)
/// Calls: cxx:__Z17FirstChildElementPN8tinyxml210XMLElementEPKc, cxx:__ZNK6mujoco4user8FilePath5c_strEv, cxx:_mjs_getError, cxx:_mjs_makeMesh, cxx:_mjs_setString
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_one_mesh(self_ptr: *mut mjXReader, elem: *mut tinyxml2__XMLElement, mesh: *mut mjsMesh, vfs: *const mjVFS) {
    todo!() // cxx:__ZN9mjXReader7OneMeshEPN8tinyxml210XMLElementEP8mjsMesh_PK6mjVFS_
}

/// C: mjXReader::OneSkin (xml/xml_native_reader.cc:648)
/// Calls: cxx:__Z17FirstChildElementPN8tinyxml210XMLElementEPKc, cxx:__Z18NextSiblingElementPN8tinyxml210XMLElementEPKc, cxx:__ZNK6mujoco4user8FilePath5c_strEv, cxx:_mjs_appendFloatVec, cxx:_mjs_appendIntVec, cxx:_mjs_appendString, cxx:_mjs_setFloat, cxx:_mjs_setString
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_one_skin(self_ptr: *mut mjXReader, elem: *mut tinyxml2__XMLElement, skin: *mut mjsSkin, vfs: *const mjVFS) {
    todo!() // cxx:__ZN9mjXReader7OneSkinEPN8tinyxml210XMLElementEP8mjsSkin_PK6mjVFS_
}

/// C: mjXReader::OneMaterial (xml/xml_native_reader.cc:712)
/// Calls: cxx:__Z17FirstChildElementPN8tinyxml210XMLElementEPKc, cxx:__Z18NextSiblingElementPN8tinyxml210XMLElementEPKc, cxx:_mjs_setInStringVec, cxx:_mjs_setString
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_one_material(self_ptr: *mut mjXReader, elem: *mut tinyxml2__XMLElement, material: *mut mjsMaterial) {
    todo!() // cxx:__ZN9mjXReader11OneMaterialEPN8tinyxml210XMLElementEP12mjsMaterial_
}

/// C: mjXReader::OneJoint (xml/xml_native_reader.cc:744)
/// Calls: cxx:_mjs_setString
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_one_joint(self_ptr: *mut mjXReader, elem: *mut tinyxml2__XMLElement, joint: *mut mjsJoint) {
    todo!() // cxx:__ZN9mjXReader8OneJointEPN8tinyxml210XMLElementEP9mjsJoint_
}

/// C: mjXReader::OneGeom (xml/xml_native_reader.cc:754)
/// Calls: cxx:__Z17FirstChildElementPN8tinyxml210XMLElementEPKc, cxx:_mjs_setString
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_one_geom(self_ptr: *mut mjXReader, elem: *mut tinyxml2__XMLElement, geom: *mut mjsGeom) {
    todo!() // cxx:__ZN9mjXReader7OneGeomEPN8tinyxml210XMLElementEP8mjsGeom_
}

/// C: mjXReader::OneSite (xml/xml_native_reader.cc:778)
/// Calls: cxx:_mjs_setString
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_one_site(self_ptr: *mut mjXReader, elem: *mut tinyxml2__XMLElement, site: *mut mjsSite) {
    todo!() // cxx:__ZN9mjXReader7OneSiteEPN8tinyxml210XMLElementEP8mjsSite_
}

/// C: mjXReader::OneCamera (xml/xml_native_reader.cc:794)
/// Calls: cxx:_mjs_setString
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_one_camera(self_ptr: *mut mjXReader, elem: *mut tinyxml2__XMLElement, camera: *mut mjsCamera) {
    todo!() // cxx:__ZN9mjXReader9OneCameraEPN8tinyxml210XMLElementEP10mjsCamera_
}

/// C: mjXReader::OneLight (xml/xml_native_reader.cc:810)
/// Calls: cxx:_mjs_setString
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_one_light(self_ptr: *mut mjXReader, elem: *mut tinyxml2__XMLElement, light: *mut mjsLight) {
    todo!() // cxx:__ZN9mjXReader8OneLightEPN8tinyxml210XMLElementEP9mjsLight_
}

/// C: mjXReader::ReadAttrTable (xml/xml_native_reader.cc:829)
/// Calls: cxx:_mjs_getError, cxx:_mjs_setName
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_read_attr_table(self_ptr: *mut mjXReader, elem: *mut tinyxml2__XMLElement, obj: *mut (), el: *mut mjsElement, rows: *const mjXAttr, nrow: i32) {
    todo!() // cxx:__ZN9mjXReader13ReadAttrTableEPN8tinyxml210XMLElementEPvP11mjsElement_PK7mjXAttri
}

/// C: mjXReader::ReadAttrTableCore (xml/xml_native_reader.cc:845)
/// Calls: cxx:_mjs_setAuthored, cxx:_mjs_setDouble, cxx:_mjs_setFloat, cxx:_mjs_setInt, cxx:_mjs_setString, cxx:_mjs_setStringVec
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_read_attr_table_core(self_ptr: *mut mjXReader, elem: *mut tinyxml2__XMLElement, obj: *mut (), rows: *const mjXAttr, nrow: i32, skipnodefault: bool, authored: *const ()) {
    todo!() // cxx:__ZN9mjXReader17ReadAttrTableCoreEPN8tinyxml210XMLElementEPvPK7mjXAttribPKv
}

/// C: mjXReader::OnePair (xml/xml_native_reader.cc:974)
/// Calls: cxx:_mjs_setString
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_one_pair(self_ptr: *mut mjXReader, elem: *mut tinyxml2__XMLElement, pair: *mut mjsPair) {
    todo!() // cxx:__ZN9mjXReader7OnePairEPN8tinyxml210XMLElementEP8mjsPair_
}

/// C: mjXReader::OneEquality (xml/xml_native_reader.cc:983)
/// Calls: cxx:_mjs_setString
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_one_equality(self_ptr: *mut mjXReader, elem: *mut tinyxml2__XMLElement, equality: *mut mjsEquality) {
    todo!() // cxx:__ZN9mjXReader11OneEqualityEPN8tinyxml210XMLElementEP12mjsEquality_
}

/// C: mjXReader::OneTendon (xml/xml_native_reader.cc:1081)
/// Calls: cxx:_mjs_setString
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_one_tendon(self_ptr: *mut mjXReader, elem: *mut tinyxml2__XMLElement, tendon: *mut mjsTendon) {
    todo!() // cxx:__ZN9mjXReader9OneTendonEPN8tinyxml210XMLElementEP10mjsTendon_
}

/// C: ReadInputSpec (xml/xml_native_reader.cc:1099)
#[allow(unused_variables, non_snake_case)]
pub fn read_input_spec(elem: *mut tinyxml2__XMLElement, ctrlspec: *mut i32) -> bool {
    todo!() // cxx-internal:xml_native_reader.cc.o:__ZL13ReadInputSpecPN8tinyxml210XMLElementEPi
}

/// C: mjXReader::OneActuator (xml/xml_native_reader.cc:1133)
/// Calls: cxx-internal:xml_native_reader.cc.o:__ZL13ReadInputSpecPN8tinyxml210XMLElementEPi, cxx:_mjs_setString, cxx:_mjs_setToAdhesion, cxx:_mjs_setToCylinder, cxx:_mjs_setToDCMotor, cxx:_mjs_setToDamper, cxx:_mjs_setToIntVelocity, cxx:_mjs_setToMotor, cxx:_mjs_setToMuscle, cxx:_mjs_setToOrientation, cxx:_mjs_setToPID, cxx:_mjs_setToPosition, cxx:_mjs_setToVelocity
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_one_actuator(self_ptr: *mut mjXReader, elem: *mut tinyxml2__XMLElement, actuator: *mut mjsActuator) {
    todo!() // cxx:__ZN9mjXReader11OneActuatorEPN8tinyxml210XMLElementEP12mjsActuator_
}

/// C: mjXReader::OneComposite (xml/xml_native_reader.cc:1425)
/// Calls: cxx:__Z17FirstChildElementPN8tinyxml210XMLElementEPKc, cxx:__Z18NextSiblingElementPN8tinyxml210XMLElementEPKc, cxx:__ZN12mjCComposite10SetDefaultEv, cxx:__ZN12mjCComposite15AddDefaultJointEPci, cxx:__ZN12mjCComposite4MakeEP7mjSpec_P8mjsBody_Pci, cxx:_mjs_setString
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_one_composite(self_ptr: *mut mjXReader, elem: *mut tinyxml2__XMLElement, body: *mut mjsBody, frame: *mut mjsFrame, def: *const mjsDefault) {
    todo!() // cxx:__ZN9mjXReader12OneCompositeEPN8tinyxml210XMLElementEP8mjsBody_P9mjsFrame_PK11mjsDefault_
}

/// C: mjXReader::OneFlexcomp (xml/xml_native_reader.cc:1583)
/// Calls: cxx:__Z17FirstChildElementPN8tinyxml210XMLElementEPKc, cxx:__Z18NextSiblingElementPN8tinyxml210XMLElementEPKc, cxx:__ZN11mjCFlexcomp4MakeEP8mjsBody_PciPK6mjVFS_, cxx:__ZNK6mujoco4user8FilePath3StrEv, cxx:_mjs_setString
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_one_flexcomp(self_ptr: *mut mjXReader, elem: *mut tinyxml2__XMLElement, body: *mut mjsBody, vfs: *const mjVFS) {
    todo!() // cxx:__ZN9mjXReader11OneFlexcompEPN8tinyxml210XMLElementEP8mjsBody_PK6mjVFS_
}

/// C: mjXReader::OnePlugin (xml/xml_native_reader.cc:1713)
/// Calls: cxx-internal:xml_native_reader.cc.o:__ZN12_GLOBAL__N_117ReadPluginConfigsEPN8tinyxml210XMLElementEP10mjsPlugin_, cxx:_mjs_addPlugin, cxx:_mjs_setString
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_one_plugin(self_ptr: *mut mjXReader, elem: *mut tinyxml2__XMLElement, plugin: *mut mjsPlugin) {
    todo!() // cxx:__ZN9mjXReader9OnePluginEPN8tinyxml210XMLElementEP10mjsPlugin_
}

/// C: mjXReader::Default (xml/xml_native_reader.cc:1733)
/// Calls: cxx:__Z17FirstChildElementPN8tinyxml210XMLElementEPKc, cxx:__Z18NextSiblingElementPN8tinyxml210XMLElementEPKc, cxx:_mjs_addDefault, cxx:_mjs_getSpecDefault
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_default(self_ptr: *mut mjXReader, section: *mut tinyxml2__XMLElement, def: *const mjsDefault, vfs: *const mjVFS) {
    todo!() // cxx:__ZN9mjXReader7DefaultEPN8tinyxml210XMLElementEPK11mjsDefault_PK6mjVFS_
}

/// C: mjXReader::Extension (xml/xml_native_reader.cc:1834)
/// Calls: cxx-internal:xml_native_reader.cc.o:__ZN12_GLOBAL__N_117ReadPluginConfigsEPN8tinyxml210XMLElementEP10mjsPlugin_, cxx:__Z17FirstChildElementPN8tinyxml210XMLElementEPKc, cxx:__Z18NextSiblingElementPN8tinyxml210XMLElementEPKc, cxx:_mjs_activatePlugin, cxx:_mjs_addPlugin, cxx:_mjs_setString
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_extension(self_ptr: *mut mjXReader, section: *mut tinyxml2__XMLElement) {
    todo!() // cxx:__ZN9mjXReader9ExtensionEPN8tinyxml210XMLElementE
}

/// C: mjXReader::Custom (xml/xml_native_reader.cc:1875)
/// Calls: cxx:__Z17FirstChildElementPN8tinyxml210XMLElementEPKc, cxx:__Z18NextSiblingElementPN8tinyxml210XMLElementEPKc, cxx:_mjs_addNumeric, cxx:_mjs_addText, cxx:_mjs_addTuple, cxx:_mjs_getError, cxx:_mjs_setDouble, cxx:_mjs_setInt, cxx:_mjs_setName, cxx:_mjs_setString, cxx:_mjs_setStringVec, cxx:_mju_str2Type
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_custom(self_ptr: *mut mjXReader, section: *mut tinyxml2__XMLElement) {
    todo!() // cxx:__ZN9mjXReader6CustomEPN8tinyxml210XMLElementE
}

/// C: mjXReader::Visual (xml/xml_native_reader.cc:2023)
/// Calls: cxx:__Z17FirstChildElementPN8tinyxml210XMLElementEPKc, cxx:__Z18NextSiblingElementPN8tinyxml210XMLElementEPKc
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_visual(self_ptr: *mut mjXReader, section: *mut tinyxml2__XMLElement) {
    todo!() // cxx:__ZN9mjXReader6VisualEPN8tinyxml210XMLElementE
}

/// C: mjXReader::Asset (xml/xml_native_reader.cc:2103)
/// Calls: cxx:__Z17FirstChildElementPN8tinyxml210XMLElementEPKc, cxx:__Z18NextSiblingElementPN8tinyxml210XMLElementEPKc, cxx:__ZNK6mujoco4user8FilePath3StrEv, cxx:__ZNK6mujoco4user8FilePath5c_strEv, cxx:_mj_parse, cxx:_mjs_addHField, cxx:_mjs_addMaterial, cxx:_mjs_addMesh, cxx:_mjs_addSkin, cxx:_mjs_addSpec, cxx:_mjs_addTexture, cxx:_mjs_getSpecDefault, cxx:_mjs_setFloat, cxx:_mjs_setInStringVec, cxx:_mjs_setString
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_asset(self_ptr: *mut mjXReader, section: *mut tinyxml2__XMLElement, vfs: *const mjVFS) {
    todo!() // cxx:__ZN9mjXReader5AssetEPN8tinyxml210XMLElementEPK6mjVFS_
}

/// C: stripError (xml/xml_native_reader.cc:2254)
#[allow(unused_variables, non_snake_case)]
pub fn strip_error(err: *const i8) -> *const i8 {
    todo!() // cxx-internal:xml_native_reader.cc.o:__ZL10stripErrorPKc
}

/// C: mjXReader::Body (xml/xml_native_reader.cc:2260)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/xml/xml_native_reader.cc:__ZL10stripErrorPKc, cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/xml/xml_native_reader.cc:__ZN12_GLOBAL__N_112UpdateStringERNSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEEii, cxx:__Z15mjuu_frameaccumPdS_PKdS1_, cxx:__Z17FirstChildElementPN8tinyxml210XMLElementEPKc, cxx:__Z18NextSiblingElementPN8tinyxml210XMLElementEPKc, cxx:_mjs_addBody, cxx:_mjs_addCamera, cxx:_mjs_addFrame, cxx:_mjs_addFreeJoint, cxx:_mjs_addGeom, cxx:_mjs_addJoint, cxx:_mjs_addLight, cxx:_mjs_addSite, cxx:_mjs_attach, cxx:_mjs_defaultOrientation, cxx:_mjs_delete, cxx:_mjs_findDefault, cxx:_mjs_findElement, cxx:_mjs_findSpec, cxx:_mjs_getDefault, cxx:_mjs_getError, cxx:_mjs_getId, cxx:_mjs_resolveOrientation, cxx:_mjs_setDefault, cxx:_mjs_setFrame, cxx:_mjs_setName, cxx:_mjs_setString, cxx:_mju_type2Str
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_body(self_ptr: *mut mjXReader, section: *mut tinyxml2__XMLElement, body: *mut mjsBody, frame: *mut mjsFrame, vfs: *const mjVFS) {
    todo!() // cxx:__ZN9mjXReader4BodyEPN8tinyxml210XMLElementEP8mjsBody_P9mjsFrame_PK6mjVFS_
}

/// C: mjXReader::Contact (xml/xml_native_reader.cc:2604)
/// Calls: cxx:__Z17FirstChildElementPN8tinyxml210XMLElementEPKc, cxx:__Z18NextSiblingElementPN8tinyxml210XMLElementEPKc, cxx:_mjs_addExclude, cxx:_mjs_addPair, cxx:_mjs_getSpecDefault, cxx:_mjs_setString
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_contact(self_ptr: *mut mjXReader, section: *mut tinyxml2__XMLElement) {
    todo!() // cxx:__ZN9mjXReader7ContactEPN8tinyxml210XMLElementE
}

/// C: mjXReader::Equality (xml/xml_native_reader.cc:2643)
/// Calls: cxx:__Z17FirstChildElementPN8tinyxml210XMLElementEPKc, cxx:__Z18NextSiblingElementPN8tinyxml210XMLElementEPKc, cxx:_mjs_addEquality, cxx:_mjs_getSpecDefault
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_equality(self_ptr: *mut mjXReader, section: *mut tinyxml2__XMLElement) {
    todo!() // cxx:__ZN9mjXReader8EqualityEPN8tinyxml210XMLElementE
}

/// C: mjXReader::Deformable (xml/xml_native_reader.cc:2664)
/// Calls: cxx:__Z17FirstChildElementPN8tinyxml210XMLElementEPKc, cxx:__Z18NextSiblingElementPN8tinyxml210XMLElementEPKc, cxx:_mjs_addFlex, cxx:_mjs_addSkin, cxx:_mjs_getSpecDefault
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_deformable(self_ptr: *mut mjXReader, section: *mut tinyxml2__XMLElement, vfs: *const mjVFS) {
    todo!() // cxx:__ZN9mjXReader10DeformableEPN8tinyxml210XMLElementEPK6mjVFS_
}

/// C: mjXReader::Tendon (xml/xml_native_reader.cc:2699)
/// Calls: cxx:__Z17FirstChildElementPN8tinyxml210XMLElementEPKc, cxx:__Z18NextSiblingElementPN8tinyxml210XMLElementEPKc, cxx:_mjs_addTendon, cxx:_mjs_getSpecDefault, cxx:_mjs_setString, cxx:_mjs_wrapGeom, cxx:_mjs_wrapJoint, cxx:_mjs_wrapPulley, cxx:_mjs_wrapSite
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_tendon(self_ptr: *mut mjXReader, section: *mut tinyxml2__XMLElement) {
    todo!() // cxx:__ZN9mjXReader6TendonEPN8tinyxml210XMLElementE
}

/// C: mjXReader::Actuator (xml/xml_native_reader.cc:2763)
/// Calls: cxx:__Z17FirstChildElementPN8tinyxml210XMLElementEPKc, cxx:__Z18NextSiblingElementPN8tinyxml210XMLElementEPKc, cxx:_mjs_addActuator, cxx:_mjs_getSpecDefault
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_actuator(self_ptr: *mut mjXReader, section: *mut tinyxml2__XMLElement) {
    todo!() // cxx:__ZN9mjXReader8ActuatorEPN8tinyxml210XMLElementE
}

/// C: mjXReader::Sensor (xml/xml_native_reader.cc:2784)
/// Calls: cxx:__Z17FirstChildElementPN8tinyxml210XMLElementEPKc, cxx:__Z18NextSiblingElementPN8tinyxml210XMLElementEPKc, cxx:_mjs_addSensor, cxx:_mjs_setString, cxx:_mju_str2Type
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_sensor(self_ptr: *mut mjXReader, section: *mut tinyxml2__XMLElement) {
    todo!() // cxx:__ZN9mjXReader6SensorEPN8tinyxml210XMLElementE
}

/// C: mjXReader::Keyframe (xml/xml_native_reader.cc:2979)
/// Calls: cxx:__Z17FirstChildElementPN8tinyxml210XMLElementEPKc, cxx:__Z18NextSiblingElementPN8tinyxml210XMLElementEPKc, cxx:_mjs_addKey, cxx:_mjs_getError, cxx:_mjs_setName
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_keyframe(self_ptr: *mut mjXReader, section: *mut tinyxml2__XMLElement) {
    todo!() // cxx:__ZN9mjXReader8KeyframeEPN8tinyxml210XMLElementE
}

/// C: mjXReader::GetClass (xml/xml_native_reader.cc:3004)
/// Calls: cxx:_mjs_findDefault
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_get_class(self_ptr: *mut mjXReader, section: *mut tinyxml2__XMLElement) -> *const mjsDefault {
    todo!() // cxx:__ZN9mjXReader8GetClassEPN8tinyxml210XMLElementE
}

/// C: mjXReader::SetModelFileDir (xml/xml_native_reader.cc:3016)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_set_model_file_dir(self_ptr: *mut mjXReader, modelfiledir: *const anonymous_namespace___string) {
    todo!() // cxx:__ZN9mjXReader15SetModelFileDirERKNSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEE
}

/// C: mjXReader::SetAssetDir (xml/xml_native_reader.cc:3020)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_set_asset_dir(self_ptr: *mut mjXReader, assetdir: *const anonymous_namespace___string) {
    todo!() // cxx:__ZN9mjXReader11SetAssetDirERKNSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEE
}

/// C: mjXReader::SetMeshDir (xml/xml_native_reader.cc:3024)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_set_mesh_dir(self_ptr: *mut mjXReader, meshdir: *const anonymous_namespace___string) {
    todo!() // cxx:__ZN9mjXReader10SetMeshDirERKNSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEE
}

/// C: mjXReader::SetTextureDir (xml/xml_native_reader.cc:3028)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_set_texture_dir(self_ptr: *mut mjXReader, texturedir: *const anonymous_namespace___string) {
    todo!() // cxx:__ZN9mjXReader13SetTextureDirERKNSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEE
}

/// C: mjXReader::AssetDir (xml/xml_native_reader.cc:3032)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_asset_dir(self_ptr: *mut mjXReader) -> mujoco__user__FilePath {
    todo!() // cxx:__ZNK9mjXReader8AssetDirEv
}

/// C: mjXReader::MeshDir (xml/xml_native_reader.cc:3036)
/// Calls: cxx:__ZNK6mujoco4user8FilePath5emptyEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_mesh_dir(self_ptr: *mut mjXReader) -> mujoco__user__FilePath {
    todo!() // cxx:__ZNK9mjXReader7MeshDirEv
}

/// C: mjXReader::TextureDir (xml/xml_native_reader.cc:3040)
/// Calls: cxx:__ZNK6mujoco4user8FilePath5emptyEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_texture_dir(self_ptr: *mut mjXReader) -> mujoco__user__FilePath {
    todo!() // cxx:__ZNK9mjXReader10TextureDirEv
}

/// C: mjXReader::ModelFileDir (xml/xml_native_reader.h:38)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_reader_model_file_dir(self_ptr: *mut mjXReader) -> *const mujoco__user__FilePath {
    todo!() // cxx:__ZNK9mjXReader12ModelFileDirEv
}

