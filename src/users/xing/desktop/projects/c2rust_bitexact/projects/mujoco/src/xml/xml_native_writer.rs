//! Port of: Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/xml/xml_native_writer.cc
//! IR hash: 1fa44043fe383ce9
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mj_XMLPrinter::PrintSpace (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/xml/xml_native_writer.cc:59)
#[allow(unused_variables, non_snake_case)]
pub fn mj_xml_printer_print_space(self_ptr: *mut mj_XMLPrinter, depth: i32) {
    todo!() // cxx:__ZN13mj_XMLPrinter10PrintSpaceEi
}

/// C: WriteDoc (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/xml/xml_native_writer.cc:68)
/// Calls: cxx:__Z11mjCopyErrorPcPKci
#[allow(unused_variables, non_snake_case)]
pub fn write_doc(doc: *mut tinyxml2__XMLDocument, error: *mut i8, error_sz: usize) -> anonymous_namespace___string {
    todo!() // cxx-internal:xml_native_writer.cc.o:__ZL8WriteDocRN8tinyxml211XMLDocumentEPcm
}

/// C: mjXWriter::InsertEnd (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/xml/xml_native_writer.cc:119)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_insert_end(self_ptr: *mut mjXWriter, parent: *mut tinyxml2__XMLElement, name: *const i8) -> *mut tinyxml2__XMLElement {
    todo!() // cxx:__ZN9mjXWriter9InsertEndEPN8tinyxml210XMLElementEPKc
}

/// C: mjXWriter::OneFlex (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/xml/xml_native_writer.cc:130)
/// Calls: cxx:__ZNK7mjCFlex12get_materialEv, cxx:__ZNK7mjCFlex12get_nodebodyEv, cxx:__ZNK7mjCFlex12get_texcoordEv, cxx:__ZNK7mjCFlex12get_vertbodyEv, cxx:__ZNK7mjCFlex16get_elemtexcoordEv, cxx:__ZNK7mjCFlex8get_elemEv, cxx:__ZNK7mjCFlex8get_vertEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_one_flex(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, flex: *const mjCFlex) {
    todo!() // cxx:__ZN9mjXWriter7OneFlexEPN8tinyxml210XMLElementEPK7mjCFlex
}

/// C: mjXWriter::OneMesh (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/xml/xml_native_writer.cc:222)
/// Calls: cxx:__ZN6mjCDef4MeshEv, cxx:__ZN7mjXUtil9FindValueEPK6_mjMapii, cxx:__ZNK7mjCMesh10UserNormalEv, cxx:__ZNK7mjCMesh11ContentTypeEv, cxx:__ZNK7mjCMesh12SmoothNormalEv, cxx:__ZNK7mjCMesh12UserTexcoordEv, cxx:__ZNK7mjCMesh4FileEv, cxx:__ZNK7mjCMesh5ScaleEv, cxx:__ZNK7mjCMesh6RefposEv, cxx:__ZNK7mjCMesh7InertiaEv, cxx:__ZNK7mjCMesh7RefquatEv, cxx:__ZNK7mjCMesh8UserFaceEv, cxx:__ZNK7mjCMesh8UserVertEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_one_mesh(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, mesh: *const mjCMesh, def: *mut mjCDef) {
    todo!() // cxx:__ZN9mjXWriter7OneMeshEPN8tinyxml210XMLElementEPK7mjCMeshP6mjCDef
}

/// C: mjXWriter::OneSkin (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/xml/xml_native_writer.cc:273)
/// Calls: cxx:__ZN6mjCDef4GeomEv, cxx:__ZNK7mjCSkin10get_vertidEv, cxx:__ZNK7mjCSkin11get_bindposEv, cxx:__ZNK7mjCSkin12get_bindquatEv, cxx:__ZNK7mjCSkin12get_bodynameEv, cxx:__ZNK7mjCSkin12get_materialEv, cxx:__ZNK7mjCSkin12get_texcoordEv, cxx:__ZNK7mjCSkin14get_vertweightEv, cxx:__ZNK7mjCSkin4FileEv, cxx:__ZNK7mjCSkin8get_faceEv, cxx:__ZNK7mjCSkin8get_vertEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_one_skin(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, skin: *const mjCSkin) {
    todo!() // cxx:__ZN9mjXWriter7OneSkinEPN8tinyxml210XMLElementEPK7mjCSkin
}

/// C: mjXWriter::OneMaterial (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/xml/xml_native_writer.cc:326)
/// Calls: cxx:__ZN6mjCDef8MaterialEv, cxx:__ZN7mjXUtil9FindValueEPK6_mjMapii, cxx:__ZNK11mjCMaterial11get_textureEi
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_one_material(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, material: *const mjCMaterial, def: *mut mjCDef) {
    todo!() // cxx:__ZN9mjXWriter11OneMaterialEPN8tinyxml210XMLElementEPK11mjCMaterialP6mjCDef
}

/// C: mjXWriter::OneJoint (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/xml/xml_native_writer.cc:375)
/// Calls: cxx:__ZN6mjCDef5JointEv, cxx:__ZN7mjXUtil9FindValueEPK6_mjMapii, cxx:__ZNK8mjCJoint12get_userdataEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_one_joint(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, joint: *const mjCJoint, def: *mut mjCDef, classname: std__string_view) {
    todo!() // cxx:__ZN9mjXWriter8OneJointEPN8tinyxml210XMLElementEPK8mjCJointP6mjCDefNSt3__117basic_string_viewIcNS8_11char_traitsIcEEEE
}

/// C: mjXWriter::OneGeom (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/xml/xml_native_writer.cc:441)
/// Calls: cxx:__Z12mjuu_definedd, cxx:__Z18mjuu_frameaccuminvPdS_PKdS1_, cxx:__ZN6mjCDef4GeomEv, cxx:__ZN7mjCMesh10GetQuatPtrEv, cxx:__ZN7mjCMesh9GetPosPtrEv, cxx:__ZNK7mjCGeom12get_materialEv, cxx:__ZNK7mjCGeom12get_meshnameEv, cxx:__ZNK7mjCGeom12get_userdataEv, cxx:__ZNK7mjCGeom14get_hfieldnameEv, cxx:__ZNK7mjCGeom9GetVolumeEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_one_geom(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, geom: *const mjCGeom, def: *mut mjCDef, classname: std__string_view) {
    todo!() // cxx:__ZN9mjXWriter7OneGeomEPN8tinyxml210XMLElementEPK7mjCGeomP6mjCDefNSt3__117basic_string_viewIcNS8_11char_traitsIcEEEE
}

/// C: mjXWriter::OneSite (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/xml/xml_native_writer.cc:540)
/// Calls: cxx:__ZN6mjCDef4SiteEv, cxx:__ZNK7mjCSite12get_materialEv, cxx:__ZNK7mjCSite12get_userdataEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_one_site(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, site: *const mjCSite, def: *mut mjCDef, classname: std__string_view) {
    todo!() // cxx:__ZN9mjXWriter7OneSiteEPN8tinyxml210XMLElementEPK7mjCSiteP6mjCDefNSt3__117basic_string_viewIcNS8_11char_traitsIcEEEE
}

/// C: mjXWriter::OneCamera (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/xml/xml_native_writer.cc:575)
/// Calls: cxx:__ZN6mjCDef6CameraEv, cxx:__ZN7mjXUtil13WriteAttrKeysEPN8tinyxml210XMLElementENSt3__112basic_stringIcNS3_11char_traitsIcEENS3_9allocatorIcEEEEPK6_mjMapiPiii, cxx:__ZNK9mjCCamera12get_userdataEv, cxx:__ZNK9mjCCamera14get_targetbodyEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_one_camera(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, camera: *const mjCCamera, def: *mut mjCDef, classname: std__string_view) {
    todo!() // cxx:__ZN9mjXWriter9OneCameraEPN8tinyxml210XMLElementEPK9mjCCameraP6mjCDefNSt3__117basic_string_viewIcNS8_11char_traitsIcEEEE
}

/// C: mjXWriter::OneLight (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/xml/xml_native_writer.cc:629)
/// Calls: cxx:__ZN6mjCDef5LightEv, cxx:__ZNK8mjCLight11get_textureEv, cxx:__ZNK8mjCLight14get_targetbodyEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_one_light(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, light: *const mjCLight, def: *mut mjCDef, classname: std__string_view) {
    todo!() // cxx:__ZN9mjXWriter8OneLightEPN8tinyxml210XMLElementEPK8mjCLightP6mjCDefNSt3__117basic_string_viewIcNS8_11char_traitsIcEEEE
}

/// C: mjXWriter::OnePair (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/xml/xml_native_writer.cc:660)
/// Calls: cxx:__ZN6mjCDef4PairEv, cxx:__ZNK7mjCPair13get_geomname1Ev, cxx:__ZNK7mjCPair13get_geomname2Ev
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_one_pair(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, pair: *const mjCPair, def: *mut mjCDef) {
    todo!() // cxx:__ZN9mjXWriter7OnePairEPN8tinyxml210XMLElementEPK7mjCPairP6mjCDef
}

/// C: mjXWriter::OneEquality (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/xml/xml_native_writer.cc:685)
/// Calls: cxx:__ZN6mjCDef8EqualityEv, cxx:_mjs_getString, cxx:_mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_one_equality(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, equality: *const mjCEquality, def: *mut mjCDef) {
    todo!() // cxx:__ZN9mjXWriter11OneEqualityEPN8tinyxml210XMLElementEPK11mjCEqualityP6mjCDef
}

/// C: mjXWriter::OneTendon (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/xml/xml_native_writer.cc:754)
/// Calls: cxx:__ZN6mjCDef6TendonEv, cxx:__ZNK7mjCWrap4TypeEv, cxx:__ZNK9mjCTendon12get_materialEv, cxx:__ZNK9mjCTendon12get_userdataEv, cxx:__ZNK9mjCTendon7GetWrapEi
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_one_tendon(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, tendon: *const mjCTendon, def: *mut mjCDef) {
    todo!() // cxx:__ZN9mjXWriter9OneTendonEPN8tinyxml210XMLElementEPK9mjCTendonP6mjCDef
}

/// C: mjXWriter::OneActuator (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/xml/xml_native_writer.cc:818)
/// Calls: cxx:__ZN6mjCDef8ActuatorEv, cxx:__ZNK11mjCActuator10get_targetEv, cxx:__ZNK11mjCActuator11get_refsiteEv, cxx:__ZNK11mjCActuator12get_userdataEv, cxx:__ZNK11mjCActuator14get_slidersiteEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_one_actuator(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, actuator: *const mjCActuator, def: *mut mjCDef) {
    todo!() // cxx:__ZN9mjXWriter11OneActuatorEPN8tinyxml210XMLElementEPK11mjCActuatorP6mjCDef
}

/// C: mjXWriter::OnePlugin (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/xml/xml_native_writer.cc:916)
/// Calls: cxx:_mjp_getPluginAtSlot, cxx:_mjs_getString
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_one_plugin(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, plugin: *const mjsPlugin) {
    todo!() // cxx:__ZN9mjXWriter9OnePluginEPN8tinyxml210XMLElementEPK10mjsPlugin_
}

/// C: mjXWriter::Write (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/xml/xml_native_writer.cc:961)
/// Calls: cxx-internal:xml_native_writer.cc.o:__ZL8WriteDocRN8tinyxml211XMLDocumentEPcm, cxx:__Z11mjCopyErrorPcPKci, cxx:__ZN8mjCModel8GetWorldEv, cxx:__ZNK8mjCModel10IsCompiledEv, cxx:__ZNK8mjCModel7DefaultEv, cxx:_mjs_getString
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_write(self_ptr: *mut mjXWriter, error: *mut i8, error_sz: usize) -> anonymous_namespace___string {
    todo!() // cxx:__ZN9mjXWriter5WriteEPcm
}

/// C: mjXWriter::Compiler (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/xml/xml_native_writer.cc:1010)
/// Calls: cxx:__ZNK8mjCModel11get_meshdirEv, cxx:__ZNK8mjCModel14get_texturedirEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_compiler(self_ptr: *mut mjXWriter, root: *mut tinyxml2__XMLElement) {
    todo!() // cxx:__ZN9mjXWriter8CompilerEPN8tinyxml210XMLElementE
}

/// C: mjXWriter::Option (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/xml/xml_native_writer.cc:1044)
/// Calls: cxx:_mj_defaultOption
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_option(self_ptr: *mut mjXWriter, root: *mut tinyxml2__XMLElement) {
    todo!() // cxx:__ZN9mjXWriter6OptionEPN8tinyxml210XMLElementE
}

/// C: mjXWriter::Size (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/xml/xml_native_writer.cc:1144)
/// Calls: cxx:_mju_writeNumBytes
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_size(self_ptr: *mut mjXWriter, root: *mut tinyxml2__XMLElement) {
    todo!() // cxx:__ZN9mjXWriter4SizeEPN8tinyxml210XMLElementE
}

/// C: mjXWriter::Statistic (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/xml/xml_native_writer.cc:1174)
/// Calls: cxx:__Z12mjuu_definedd
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_statistic(self_ptr: *mut mjXWriter, root: *mut tinyxml2__XMLElement) {
    todo!() // cxx:__ZN9mjXWriter9StatisticEPN8tinyxml210XMLElementE
}

/// C: mjXWriter::Visual (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/xml/xml_native_writer.cc:1191)
/// Calls: cxx:_mj_defaultVisual
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_visual(self_ptr: *mut mjXWriter, root: *mut tinyxml2__XMLElement) {
    todo!() // cxx:__ZN9mjXWriter6VisualEPN8tinyxml210XMLElementE
}

/// C: mjXWriter::Default (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/xml/xml_native_writer.cc:1322)
/// Calls: cxx:__ZN6mjCDef4GeomEv, cxx:__ZN6mjCDef4MeshEv, cxx:__ZN6mjCDef4PairEv, cxx:__ZN6mjCDef4SiteEv, cxx:__ZN6mjCDef5JointEv, cxx:__ZN6mjCDef5LightEv, cxx:__ZN6mjCDef6CameraEv, cxx:__ZN6mjCDef6TendonEv, cxx:__ZN6mjCDef8ActuatorEv, cxx:__ZN6mjCDef8EqualityEv, cxx:__ZN6mjCDef8MaterialEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_default(self_ptr: *mut mjXWriter, root: *mut tinyxml2__XMLElement, def: *mut mjCDef) {
    todo!() // cxx:__ZN9mjXWriter7DefaultEPN8tinyxml210XMLElementEP6mjCDef
}

/// C: mjXWriter::Extension (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/xml/xml_native_writer.cc:1416)
/// Calls: cxx:__ZN8mjCModel9GetObjectE6mjtObji, cxx:__ZNK8mjCModel13ActivePluginsEv, cxx:__ZNK8mjCModel7PluginsEv, cxx:_mjp_getPluginAtSlot
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_extension(self_ptr: *mut mjXWriter, root: *mut tinyxml2__XMLElement) {
    todo!() // cxx:__ZN9mjXWriter9ExtensionEPN8tinyxml210XMLElementE
}

/// C: mjXWriter::Custom (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/xml/xml_native_writer.cc:1479)
/// Calls: cxx:__ZN8mjCModel10NumObjectsE6mjtObj, cxx:__ZN8mjCModel9GetObjectE6mjtObji, cxx:_mju_type2Str
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_custom(self_ptr: *mut mjXWriter, root: *mut tinyxml2__XMLElement) {
    todo!() // cxx:__ZN9mjXWriter6CustomEPN8tinyxml210XMLElementE
}

/// C: mjXWriter::Asset (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/xml/xml_native_writer.cc:1534)
/// Calls: cxx:__ZN7mjXUtil13Vector2StringERNSt3__112basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEERKNS0_6vectorIfNS4_IfEEEEi, cxx:__ZN8mjCModel10NumObjectsE6mjtObj, cxx:__ZN8mjCModel9GetObjectE6mjtObji, cxx:__ZN9mjCHField12get_userdataEv, cxx:__ZNK10mjCTexture13get_cubefilesEv, cxx:__ZNK10mjCTexture16get_content_typeEv, cxx:__ZNK10mjCTexture4FileEv, cxx:__ZNK7mjCMesh4FileEv, cxx:__ZNK7mjCMesh6PluginEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_asset(self_ptr: *mut mjXWriter, root: *mut tinyxml2__XMLElement) {
    todo!() // cxx:__ZN9mjXWriter5AssetEPN8tinyxml210XMLElementE
}

/// C: mjXWriter::OneFrame (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/xml/xml_native_writer.cc:1673)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_one_frame(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, frame: *mut mjCFrame) -> *mut tinyxml2__XMLElement {
    todo!() // cxx:__ZN9mjXWriter8OneFrameEPN8tinyxml210XMLElementEP8mjCFrame
}

/// C: mjXWriter::Body (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/xml/xml_native_writer.cc:1694)
/// Calls: cxx:__ZN7mjCBody12get_userdataEv, cxx:__ZN8mjCModel8GetWorldEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_body(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, body: *mut mjCBody, frame: *mut mjCFrame, childclass: std__string_view) {
    todo!() // cxx:__ZN9mjXWriter4BodyEPN8tinyxml210XMLElementEP7mjCBodyP8mjCFrameNSt3__117basic_string_viewIcNS7_11char_traitsIcEEEE
}

/// C: mjXWriter::Contact (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/xml/xml_native_writer.cc:1857)
/// Calls: cxx:__ZN8mjCModel10NumObjectsE6mjtObj, cxx:__ZN8mjCModel9GetObjectE6mjtObji, cxx:__ZNK11mjCBodyPair13get_bodyname1Ev, cxx:__ZNK11mjCBodyPair13get_bodyname2Ev
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_contact(self_ptr: *mut mjXWriter, root: *mut tinyxml2__XMLElement) {
    todo!() // cxx:__ZN9mjXWriter7ContactEPN8tinyxml210XMLElementE
}

/// C: mjXWriter::Equality (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/xml/xml_native_writer.cc:1896)
/// Calls: cxx:__ZN7mjXUtil9FindValueEPK6_mjMapii, cxx:__ZN8mjCModel10NumObjectsE6mjtObj, cxx:__ZN8mjCModel9GetObjectE6mjtObji
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_equality(self_ptr: *mut mjXWriter, root: *mut tinyxml2__XMLElement) {
    todo!() // cxx:__ZN9mjXWriter8EqualityEPN8tinyxml210XMLElementE
}

/// C: mjXWriter::Deformable (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/xml/xml_native_writer.cc:1918)
/// Calls: cxx:__ZN8mjCModel10NumObjectsE6mjtObj, cxx:__ZN8mjCModel9GetObjectE6mjtObji
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_deformable(self_ptr: *mut mjXWriter, root: *mut tinyxml2__XMLElement) {
    todo!() // cxx:__ZN9mjXWriter10DeformableEPN8tinyxml210XMLElementE
}

/// C: mjXWriter::Tendon (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/xml/xml_native_writer.cc:1953)
/// Calls: cxx:__ZN8mjCModel10NumObjectsE6mjtObj, cxx:__ZN8mjCModel9GetObjectE6mjtObji, cxx:__ZNK7mjCWrap4TypeEv, cxx:__ZNK9mjCTendon7GetWrapEi, cxx:__ZNK9mjCTendon8NumWrapsEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_tendon(self_ptr: *mut mjXWriter, root: *mut tinyxml2__XMLElement) {
    todo!() // cxx:__ZN9mjXWriter6TendonEPN8tinyxml210XMLElementE
}

/// C: mjXWriter::Actuator (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/xml/xml_native_writer.cc:2014)
/// Calls: cxx:__ZN8mjCModel10NumObjectsE6mjtObj, cxx:__ZN8mjCModel9GetObjectE6mjtObji
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_actuator(self_ptr: *mut mjXWriter, root: *mut tinyxml2__XMLElement) {
    todo!() // cxx:__ZN9mjXWriter8ActuatorEPN8tinyxml210XMLElementE
}

/// C: mjXWriter::Sensor (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/xml/xml_native_writer.cc:2040)
/// Calls: cxx:__ZN7mjXUtil13WriteAttrKeysEPN8tinyxml210XMLElementENSt3__112basic_stringIcNS3_11char_traitsIcEENS3_9allocatorIcEEEEPK6_mjMapiPiii, cxx:__ZN8mjCModel10NumObjectsE6mjtObj, cxx:__ZN9mjCSensor11get_objnameEv, cxx:__ZN9mjCSensor11get_refnameEv, cxx:__ZN9mjCSensor12get_userdataEv, cxx:__ZNK8mjCModel7SensorsEv, cxx:_mju_condataSize, cxx:_mju_error, cxx:_mju_type2Str
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_sensor(self_ptr: *mut mjXWriter, root: *mut tinyxml2__XMLElement) {
    todo!() // cxx:__ZN9mjXWriter6SensorEPN8tinyxml210XMLElementE
}

/// C: mjXWriter::Keyframe (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/xml/xml_native_writer.cc:2404)
/// Calls: cxx:__ZNK8mjCModel4KeysEv, cxx:__ZNK8mjCModel6BodiesEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_keyframe(self_ptr: *mut mjXWriter, root: *mut tinyxml2__XMLElement) {
    todo!() // cxx:__ZN9mjXWriter8KeyframeEPN8tinyxml210XMLElementE
}

/// C: mjXWriter::SetModel (Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/xml/xml_native_writer.h:32)
/// Calls: cxx:_mj_copyBack
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_set_model(self_ptr: *mut mjXWriter, _spec: *mut mjSpec, m: *const mjModel) {
    todo!() // cxx:__ZN9mjXWriter8SetModelEP7mjSpec_PK8mjModel_
}

