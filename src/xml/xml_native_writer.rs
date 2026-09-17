//! Port of: xml/xml_native_writer.cc
//! IR hash: fd5288ad7242f001
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: mj_XMLPrinter::PrintSpace (xml/xml_native_writer.cc:62)
#[allow(unused_variables, non_snake_case)]
pub fn mj_xml_printer_print_space(self_ptr: *mut mj_XMLPrinter, depth: i32) {
    todo!() // cxx:__ZN13mj_XMLPrinter10PrintSpaceEi
}

/// C: WriteDoc (xml/xml_native_writer.cc:69)
/// Calls: cxx:__Z11mjCopyErrorPcPKci
#[allow(unused_variables, non_snake_case)]
pub fn write_doc(doc: *mut tinyxml2__XMLDocument, error: *mut i8, error_sz: usize) -> anonymous_namespace___string {
    todo!() // cxx-internal:xml_native_writer.cc.o:__ZL8WriteDocRN8tinyxml211XMLDocumentEPcm
}

/// C: mjXWriter::InsertEnd (xml/xml_native_writer.cc:130)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_insert_end(self_ptr: *mut mjXWriter, parent: *mut tinyxml2__XMLElement, name: *const i8) -> *mut tinyxml2__XMLElement {
    todo!() // cxx:__ZN9mjXWriter9InsertEndEPN8tinyxml210XMLElementEPKc
}

/// C: mjXWriter::OneFlex (xml/xml_native_writer.cc:141)
/// Calls: cxx:__ZNK7mjCFlex12get_materialEv, cxx:__ZNK7mjCFlex12get_nodebodyEv, cxx:__ZNK7mjCFlex12get_texcoordEv, cxx:__ZNK7mjCFlex12get_vertbodyEv, cxx:__ZNK7mjCFlex16get_elemtexcoordEv, cxx:__ZNK7mjCFlex8get_elemEv, cxx:__ZNK7mjCFlex8get_nodeEv, cxx:__ZNK7mjCFlex8get_vertEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_one_flex(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, flex: *const mjCFlex) {
    todo!() // cxx:__ZN9mjXWriter7OneFlexEPN8tinyxml210XMLElementEPK7mjCFlex
}

/// C: mjXWriter::OneMesh (xml/xml_native_writer.cc:225)
/// Calls: cxx:__ZN6mjCDef4MeshEv, cxx:__ZNK7mjCMesh10UserNormalEv, cxx:__ZNK7mjCMesh11ContentTypeEv, cxx:__ZNK7mjCMesh12UserTexcoordEv, cxx:__ZNK7mjCMesh4FileEv, cxx:__ZNK7mjCMesh8MaterialEv, cxx:__ZNK7mjCMesh8UserFaceEv, cxx:__ZNK7mjCMesh8UserVertEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_one_mesh(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, mesh: *const mjCMesh, def: *mut mjCDef) {
    todo!() // cxx:__ZN9mjXWriter7OneMeshEPN8tinyxml210XMLElementEPK7mjCMeshP6mjCDef
}

/// C: mjXWriter::OneSkin (xml/xml_native_writer.cc:273)
/// Calls: cxx:__ZNK7mjCSkin10get_vertidEv, cxx:__ZNK7mjCSkin11get_bindposEv, cxx:__ZNK7mjCSkin12get_bindquatEv, cxx:__ZNK7mjCSkin12get_bodynameEv, cxx:__ZNK7mjCSkin12get_materialEv, cxx:__ZNK7mjCSkin12get_texcoordEv, cxx:__ZNK7mjCSkin14get_vertweightEv, cxx:__ZNK7mjCSkin4FileEv, cxx:__ZNK7mjCSkin8get_faceEv, cxx:__ZNK7mjCSkin8get_vertEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_one_skin(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, skin: *const mjCSkin) {
    todo!() // cxx:__ZN9mjXWriter7OneSkinEPN8tinyxml210XMLElementEPK7mjCSkin
}

/// C: mjXWriter::OneMaterial (xml/xml_native_writer.cc:327)
/// Calls: cxx:__ZN6mjCDef8MaterialEv, cxx:__ZNK11mjCMaterial11get_textureEi
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_one_material(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, material: *const mjCMaterial, def: *mut mjCDef) {
    todo!() // cxx:__ZN9mjXWriter11OneMaterialEPN8tinyxml210XMLElementEPK11mjCMaterialP6mjCDef
}

/// C: mjXWriter::OneJoint (xml/xml_native_writer.cc:367)
/// Calls: cxx:__ZN6mjCDef5JointEv, cxx:__ZNK8mjCJoint12get_userdataEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_one_joint(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, joint: *const mjCJoint, def: *mut mjCDef, classname: std__string_view) {
    todo!() // cxx:__ZN9mjXWriter8OneJointEPN8tinyxml210XMLElementEPK8mjCJointP6mjCDefNSt3__117basic_string_viewIcNS8_11char_traitsIcEEEE
}

/// C: mjXWriter::OneGeom (xml/xml_native_writer.cc:410)
/// Calls: cxx:__Z12mjuu_definedd, cxx:__Z18mjuu_frameaccuminvPdS_PKdS1_, cxx:__ZN6mjCDef4GeomEv, cxx:__ZN7mjCMesh10GetQuatPtrEv, cxx:__ZN7mjCMesh9GetPosPtrEv, cxx:__ZNK7mjCGeom12get_materialEv, cxx:__ZNK7mjCGeom12get_meshnameEv, cxx:__ZNK7mjCGeom12get_userdataEv, cxx:__ZNK7mjCGeom14get_hfieldnameEv, cxx:__ZNK7mjCGeom9GetVolumeEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_one_geom(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, geom: *const mjCGeom, def: *mut mjCDef, classname: std__string_view) {
    todo!() // cxx:__ZN9mjXWriter7OneGeomEPN8tinyxml210XMLElementEPK7mjCGeomP6mjCDefNSt3__117basic_string_viewIcNS8_11char_traitsIcEEEE
}

/// C: mjXWriter::OneSite (xml/xml_native_writer.cc:495)
/// Calls: cxx:__Z18mjuu_frameaccuminvPdS_PKdS1_, cxx:__ZN6mjCDef4SiteEv, cxx:__ZN7mjCMesh10GetQuatPtrEv, cxx:__ZN7mjCMesh9GetPosPtrEv, cxx:__ZNK7mjCSite12get_materialEv, cxx:__ZNK7mjCSite12get_meshnameEv, cxx:__ZNK7mjCSite12get_userdataEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_one_site(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, site: *const mjCSite, def: *mut mjCDef, classname: std__string_view) {
    todo!() // cxx:__ZN9mjXWriter7OneSiteEPN8tinyxml210XMLElementEPK7mjCSiteP6mjCDefNSt3__117basic_string_viewIcNS8_11char_traitsIcEEEE
}

/// C: mjXWriter::OneCamera (xml/xml_native_writer.cc:559)
/// Calls: cxx:__ZN6mjCDef6CameraEv, cxx:__ZNK9mjCCamera12get_userdataEv, cxx:__ZNK9mjCCamera14get_targetbodyEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_one_camera(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, camera: *const mjCCamera, def: *mut mjCDef, classname: std__string_view) {
    todo!() // cxx:__ZN9mjXWriter9OneCameraEPN8tinyxml210XMLElementEPK9mjCCameraP6mjCDefNSt3__117basic_string_viewIcNS8_11char_traitsIcEEEE
}

/// C: mjXWriter::OneLight (xml/xml_native_writer.cc:602)
/// Calls: cxx:__ZN6mjCDef5LightEv, cxx:__ZNK8mjCLight11get_textureEv, cxx:__ZNK8mjCLight14get_targetbodyEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_one_light(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, light: *const mjCLight, def: *mut mjCDef, classname: std__string_view) {
    todo!() // cxx:__ZN9mjXWriter8OneLightEPN8tinyxml210XMLElementEPK8mjCLightP6mjCDefNSt3__117basic_string_viewIcNS8_11char_traitsIcEEEE
}

/// C: mjXWriter::WriteAttrTable (xml/xml_native_writer.cc:629)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_write_attr_table__5e4574ea6a08c8c0(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, obj: *const mjsCamera, def: *const mjsCamera, rows: *const struct_mjXAttr, nrow: i32) {
    todo!() // cxx:__ZN9mjXWriter14WriteAttrTableI10mjsCamera_EEvPN8tinyxml210XMLElementEPKT_S7_PK7mjXAttri
}

/// C: mjXWriter::WriteAttrTable (xml/xml_native_writer.cc:629)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_write_attr_table__6880f54fe968c3cd(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, obj: *const mjsTendon, def: *const mjsTendon, rows: *const mjXAttr, nrow: i32) {
    todo!() // cxx:__ZN9mjXWriter14WriteAttrTableI10mjsTendon_EEvPN8tinyxml210XMLElementEPKT_S7_PK7mjXAttri
}

/// C: mjXWriter::WriteAttrTable (xml/xml_native_writer.cc:629)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_write_attr_table__2ba4a02b00a82e6a(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, obj: *const mjStatistic, def: *const mjStatistic, rows: *const mjXAttr, nrow: i32) {
    todo!() // cxx:__ZN9mjXWriter14WriteAttrTableI12mjStatistic_EEvPN8tinyxml210XMLElementEPKT_S7_PK7mjXAttri
}

/// C: mjXWriter::WriteAttrTable (xml/xml_native_writer.cc:629)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_write_attr_table__15bca5edcfa1cb9c(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, obj: *const mjsActuator, def: *const mjsActuator, rows: *const mjXAttr, nrow: i32) {
    todo!() // cxx:__ZN9mjXWriter14WriteAttrTableI12mjsActuator_EEvPN8tinyxml210XMLElementEPKT_S7_PK7mjXAttri
}

/// C: mjXWriter::WriteAttrTable (xml/xml_native_writer.cc:629)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_write_attr_table__5077aa30ee6203ea(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, obj: *const mjsEquality, def: *const mjsEquality, rows: *const mjXAttr, nrow: i32) {
    todo!() // cxx:__ZN9mjXWriter14WriteAttrTableI12mjsEquality_EEvPN8tinyxml210XMLElementEPKT_S7_PK7mjXAttri
}

/// C: mjXWriter::WriteAttrTable (xml/xml_native_writer.cc:629)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_write_attr_table__dfeee26590c45e64(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, obj: *const mjsMaterial, def: *const mjsMaterial, rows: *const struct_mjXAttr, nrow: i32) {
    todo!() // cxx:__ZN9mjXWriter14WriteAttrTableI12mjsMaterial_EEvPN8tinyxml210XMLElementEPKT_S7_PK7mjXAttri
}

/// C: mjXWriter::WriteAttrTable (xml/xml_native_writer.cc:629)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_write_attr_table__d04bb8fb75ffedfe(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, obj: *const mjSpec, def: *const mjSpec, rows: *const mjXAttr, nrow: i32) {
    todo!() // cxx:__ZN9mjXWriter14WriteAttrTableI7mjSpec_EEvPN8tinyxml210XMLElementEPKT_S7_PK7mjXAttri
}

/// C: mjXWriter::WriteAttrTable (xml/xml_native_writer.cc:629)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_write_attr_table__094e305707a4ff75(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, obj: *const mjsFlex, def: *const mjsFlex, rows: *const struct_mjXAttr, nrow: i32) {
    todo!() // cxx:__ZN9mjXWriter14WriteAttrTableI8mjsFlex_EEvPN8tinyxml210XMLElementEPKT_S7_PK7mjXAttri
}

/// C: mjXWriter::WriteAttrTable (xml/xml_native_writer.cc:629)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_write_attr_table__1dc032ada1835483(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, obj: *const mjsGeom, def: *const mjsGeom, rows: *const struct_mjXAttr, nrow: i32) {
    todo!() // cxx:__ZN9mjXWriter14WriteAttrTableI8mjsGeom_EEvPN8tinyxml210XMLElementEPKT_S7_PK7mjXAttri
}

/// C: mjXWriter::WriteAttrTable (xml/xml_native_writer.cc:629)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_write_attr_table__67d4b067f07c1744(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, obj: *const mjsMesh, def: *const mjsMesh, rows: *const struct_mjXAttr, nrow: i32) {
    todo!() // cxx:__ZN9mjXWriter14WriteAttrTableI8mjsMesh_EEvPN8tinyxml210XMLElementEPKT_S7_PK7mjXAttri
}

/// C: mjXWriter::WriteAttrTable (xml/xml_native_writer.cc:629)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_write_attr_table__b42de51207c855bb(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, obj: *const mjsPair, def: *const mjsPair, rows: *const mjXAttr, nrow: i32) {
    todo!() // cxx:__ZN9mjXWriter14WriteAttrTableI8mjsPair_EEvPN8tinyxml210XMLElementEPKT_S7_PK7mjXAttri
}

/// C: mjXWriter::WriteAttrTable (xml/xml_native_writer.cc:629)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_write_attr_table__c18829d13f509cf1(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, obj: *const mjsSite, def: *const mjsSite, rows: *const struct_mjXAttr, nrow: i32) {
    todo!() // cxx:__ZN9mjXWriter14WriteAttrTableI8mjsSite_EEvPN8tinyxml210XMLElementEPKT_S7_PK7mjXAttri
}

/// C: mjXWriter::WriteAttrTable (xml/xml_native_writer.cc:629)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_write_attr_table__25037eeab10d358a(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, obj: *const mjsSkin, def: *const mjsSkin, rows: *const struct_mjXAttr, nrow: i32) {
    todo!() // cxx:__ZN9mjXWriter14WriteAttrTableI8mjsSkin_EEvPN8tinyxml210XMLElementEPKT_S7_PK7mjXAttri
}

/// C: mjXWriter::WriteAttrTable (xml/xml_native_writer.cc:629)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_write_attr_table__1ef2499029e4db48(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, obj: *const mjOption, def: *const mjOption, rows: *const mjXAttr, nrow: i32) {
    todo!() // cxx:__ZN9mjXWriter14WriteAttrTableI9mjOption_EEvPN8tinyxml210XMLElementEPKT_S7_PK7mjXAttri
}

/// C: mjXWriter::WriteAttrTable (xml/xml_native_writer.cc:629)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_write_attr_table__b744df6527b4ae7f(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, obj: *const mjVisual, def: *const mjVisual, rows: *const mjXAttr, nrow: i32) {
    todo!() // cxx:__ZN9mjXWriter14WriteAttrTableI9mjVisual_EEvPN8tinyxml210XMLElementEPKT_S7_PK7mjXAttri
}

/// C: mjXWriter::WriteAttrTable (xml/xml_native_writer.cc:629)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_write_attr_table__654922d014945e8f(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, obj: *const mjsJoint, def: *const mjsJoint, rows: *const struct_mjXAttr, nrow: i32) {
    todo!() // cxx:__ZN9mjXWriter14WriteAttrTableI9mjsJoint_EEvPN8tinyxml210XMLElementEPKT_S7_PK7mjXAttri
}

/// C: mjXWriter::WriteAttrTable (xml/xml_native_writer.cc:629)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_write_attr_table__31e5f469c1df6e62(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, obj: *const mjsLight, def: *const mjsLight, rows: *const struct_mjXAttr, nrow: i32) {
    todo!() // cxx:__ZN9mjXWriter14WriteAttrTableI9mjsLight_EEvPN8tinyxml210XMLElementEPKT_S7_PK7mjXAttri
}

/// C: mjXWriter::OnePair (xml/xml_native_writer.cc:718)
/// Calls: cxx:__ZN6mjCDef4PairEv, cxx:__ZNK7mjCPair13get_geomname1Ev, cxx:__ZNK7mjCPair13get_geomname2Ev
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_one_pair(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, pair: *const mjCPair, def: *mut mjCDef) {
    todo!() // cxx:__ZN9mjXWriter7OnePairEPN8tinyxml210XMLElementEPK7mjCPairP6mjCDef
}

/// C: mjXWriter::OneEquality (xml/xml_native_writer.cc:737)
/// Calls: cxx:__ZN6mjCDef8EqualityEv, cxx:_mjs_getString, cxx:_mju_error
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_one_equality(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, equality: *const mjCEquality, def: *mut mjCDef) {
    todo!() // cxx:__ZN9mjXWriter11OneEqualityEPN8tinyxml210XMLElementEPK11mjCEqualityP6mjCDef
}

/// C: mjXWriter::OneTendon (xml/xml_native_writer.cc:809)
/// Calls: cxx:__ZN6mjCDef6TendonEv, cxx:__ZNK7mjCWrap4TypeEv, cxx:__ZNK9mjCTendon12get_materialEv, cxx:__ZNK9mjCTendon12get_userdataEv, cxx:__ZNK9mjCTendon7GetWrapEi
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_one_tendon(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, tendon: *const mjCTendon, def: *mut mjCDef) {
    todo!() // cxx:__ZN9mjXWriter9OneTendonEPN8tinyxml210XMLElementEPK9mjCTendonP6mjCDef
}

/// C: mjXWriter::OneActuator (xml/xml_native_writer.cc:853)
/// Calls: cxx:__ZN6mjCDef8ActuatorEv, cxx:__ZNK11mjCActuator10get_targetEv, cxx:__ZNK11mjCActuator11get_refsiteEv, cxx:__ZNK11mjCActuator12get_userdataEv, cxx:__ZNK11mjCActuator14get_slidersiteEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_one_actuator(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, actuator: *const mjCActuator, def: *mut mjCDef) {
    todo!() // cxx:__ZN9mjXWriter11OneActuatorEPN8tinyxml210XMLElementEPK11mjCActuatorP6mjCDef
}

/// C: mjXWriter::OnePlugin (xml/xml_native_writer.cc:950)
/// Calls: cxx:_mjp_getPluginAtSlot, cxx:_mjs_getString
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_one_plugin(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, plugin: *const mjsPlugin) {
    todo!() // cxx:__ZN9mjXWriter9OnePluginEPN8tinyxml210XMLElementEPK10mjsPlugin_
}

/// C: mjXWriter::SetModel (xml/xml_native_writer.cc:983)
/// Calls: cxx:_mj_copyBack
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_set_model(self_ptr: *mut mjXWriter, _spec: *mut mjSpec, m: *const mjModel) {
    todo!() // cxx:__ZN9mjXWriter8SetModelEP7mjSpec_PK8mjModel_
}

/// C: mjXWriter::Write (xml/xml_native_writer.cc:990)
/// Calls: cxx-internal:/Users/xing/Desktop/projects/c2rust_bitexact/projects/mujoco/src/xml/xml_native_writer.cc:__ZL8WriteDocRN8tinyxml211XMLDocumentEPcm, cxx:__Z11mjCopyErrorPcPKci, cxx:__ZN8mjCModel8GetWorldEv, cxx:__ZNK8mjCModel10IsCompiledEv, cxx:__ZNK8mjCModel7DefaultEv, cxx:_mjs_getString
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_write(self_ptr: *mut mjXWriter, error: *mut i8, error_sz: usize) -> anonymous_namespace___string {
    todo!() // cxx:__ZN9mjXWriter5WriteEPcm
}

/// C: mjXWriter::Compiler (xml/xml_native_writer.cc:1038)
/// Calls: cxx:__ZNK8mjCModel11get_meshdirEv, cxx:__ZNK8mjCModel14get_texturedirEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_compiler(self_ptr: *mut mjXWriter, root: *mut tinyxml2__XMLElement) {
    todo!() // cxx:__ZN9mjXWriter8CompilerEPN8tinyxml210XMLElementE
}

/// C: mjXWriter::Option (xml/xml_native_writer.cc:1065)
/// Calls: cxx:_mj_defaultOption
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_option(self_ptr: *mut mjXWriter, root: *mut tinyxml2__XMLElement) {
    todo!() // cxx:__ZN9mjXWriter6OptionEPN8tinyxml210XMLElementE
}

/// C: mjXWriter::Size (xml/xml_native_writer.cc:1131)
/// Calls: cxx:_mju_writeNumBytes
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_size(self_ptr: *mut mjXWriter, root: *mut tinyxml2__XMLElement) {
    todo!() // cxx:__ZN9mjXWriter4SizeEPN8tinyxml210XMLElementE
}

/// C: mjXWriter::Statistic (xml/xml_native_writer.cc:1153)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_statistic(self_ptr: *mut mjXWriter, root: *mut tinyxml2__XMLElement) {
    todo!() // cxx:__ZN9mjXWriter9StatisticEPN8tinyxml210XMLElementE
}

/// C: mjXWriter::Visual (xml/xml_native_writer.cc:1170)
/// Calls: cxx:_mj_defaultVisual
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_visual(self_ptr: *mut mjXWriter, root: *mut tinyxml2__XMLElement) {
    todo!() // cxx:__ZN9mjXWriter6VisualEPN8tinyxml210XMLElementE
}

/// C: mjXWriter::Default (xml/xml_native_writer.cc:1202)
/// Calls: cxx:__ZN6mjCDef4GeomEv, cxx:__ZN6mjCDef4MeshEv, cxx:__ZN6mjCDef4PairEv, cxx:__ZN6mjCDef4SiteEv, cxx:__ZN6mjCDef5JointEv, cxx:__ZN6mjCDef5LightEv, cxx:__ZN6mjCDef6CameraEv, cxx:__ZN6mjCDef6TendonEv, cxx:__ZN6mjCDef8ActuatorEv, cxx:__ZN6mjCDef8EqualityEv, cxx:__ZN6mjCDef8MaterialEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_default(self_ptr: *mut mjXWriter, root: *mut tinyxml2__XMLElement, def: *mut mjCDef) {
    todo!() // cxx:__ZN9mjXWriter7DefaultEPN8tinyxml210XMLElementEP6mjCDef
}

/// C: mjXWriter::Extension (xml/xml_native_writer.cc:1289)
/// Calls: cxx:__ZN8mjCModel9GetObjectE6mjtObji, cxx:__ZNK8mjCModel13ActivePluginsEv, cxx:__ZNK8mjCModel7PluginsEv, cxx:_mjp_getPluginAtSlot
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_extension(self_ptr: *mut mjXWriter, root: *mut tinyxml2__XMLElement) {
    todo!() // cxx:__ZN9mjXWriter9ExtensionEPN8tinyxml210XMLElementE
}

/// C: mjXWriter::Custom (xml/xml_native_writer.cc:1349)
/// Calls: cxx:__ZN8mjCModel10NumObjectsE6mjtObj, cxx:__ZN8mjCModel9GetObjectE6mjtObji, cxx:_mju_type2Str
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_custom(self_ptr: *mut mjXWriter, root: *mut tinyxml2__XMLElement) {
    todo!() // cxx:__ZN9mjXWriter6CustomEPN8tinyxml210XMLElementE
}

/// C: mjXWriter::Asset (xml/xml_native_writer.cc:1408)
/// Calls: cxx:__ZN8mjCModel10NumObjectsE6mjtObj, cxx:__ZN8mjCModel9GetObjectE6mjtObji, cxx:__ZN9mjCHField12get_userdataEv, cxx:__ZNK10mjCTexture13get_cubefilesEv, cxx:__ZNK10mjCTexture16get_content_typeEv, cxx:__ZNK10mjCTexture4FileEv, cxx:__ZNK7mjCMesh4FileEv, cxx:__ZNK7mjCMesh6PluginEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_asset(self_ptr: *mut mjXWriter, root: *mut tinyxml2__XMLElement) {
    todo!() // cxx:__ZN9mjXWriter5AssetEPN8tinyxml210XMLElementE
}

/// C: mjXWriter::OneFrame (xml/xml_native_writer.cc:1546)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_one_frame(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, frame: *mut mjCFrame) -> *mut tinyxml2__XMLElement {
    todo!() // cxx:__ZN9mjXWriter8OneFrameEPN8tinyxml210XMLElementEP8mjCFrame
}

/// C: mjXWriter::Body (xml/xml_native_writer.cc:1562)
/// Calls: cxx:__ZN7mjCBody12get_userdataEv, cxx:__ZN8mjCModel8GetWorldEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_body(self_ptr: *mut mjXWriter, elem: *mut tinyxml2__XMLElement, body: *mut mjCBody, frame: *mut mjCFrame, childclass: std__string_view) {
    todo!() // cxx:__ZN9mjXWriter4BodyEPN8tinyxml210XMLElementEP7mjCBodyP8mjCFrameNSt3__117basic_string_viewIcNS7_11char_traitsIcEEEE
}

/// C: mjXWriter::Contact (xml/xml_native_writer.cc:1709)
/// Calls: cxx:__ZN8mjCModel10NumObjectsE6mjtObj, cxx:__ZN8mjCModel9GetObjectE6mjtObji, cxx:__ZNK11mjCBodyPair13get_bodyname1Ev, cxx:__ZNK11mjCBodyPair13get_bodyname2Ev
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_contact(self_ptr: *mut mjXWriter, root: *mut tinyxml2__XMLElement) {
    todo!() // cxx:__ZN9mjXWriter7ContactEPN8tinyxml210XMLElementE
}

/// C: mjXWriter::Equality (xml/xml_native_writer.cc:1747)
/// Calls: cxx:__ZN8mjCModel10NumObjectsE6mjtObj, cxx:__ZN8mjCModel9GetObjectE6mjtObji
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_equality(self_ptr: *mut mjXWriter, root: *mut tinyxml2__XMLElement) {
    todo!() // cxx:__ZN9mjXWriter8EqualityEPN8tinyxml210XMLElementE
}

/// C: mjXWriter::Deformable (xml/xml_native_writer.cc:1766)
/// Calls: cxx:__ZN8mjCModel10NumObjectsE6mjtObj, cxx:__ZN8mjCModel9GetObjectE6mjtObji
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_deformable(self_ptr: *mut mjXWriter, root: *mut tinyxml2__XMLElement) {
    todo!() // cxx:__ZN9mjXWriter10DeformableEPN8tinyxml210XMLElementE
}

/// C: mjXWriter::Tendon (xml/xml_native_writer.cc:1798)
/// Calls: cxx:__ZN8mjCModel10NumObjectsE6mjtObj, cxx:__ZN8mjCModel9GetObjectE6mjtObji, cxx:__ZNK7mjCWrap4TypeEv, cxx:__ZNK9mjCTendon7GetWrapEi, cxx:__ZNK9mjCTendon8NumWrapsEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_tendon(self_ptr: *mut mjXWriter, root: *mut tinyxml2__XMLElement) {
    todo!() // cxx:__ZN9mjXWriter6TendonEPN8tinyxml210XMLElementE
}

/// C: mjXWriter::Actuator (xml/xml_native_writer.cc:1854)
/// Calls: cxx:__ZN8mjCModel10NumObjectsE6mjtObj, cxx:__ZN8mjCModel9GetObjectE6mjtObji
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_actuator(self_ptr: *mut mjXWriter, root: *mut tinyxml2__XMLElement) {
    todo!() // cxx:__ZN9mjXWriter8ActuatorEPN8tinyxml210XMLElementE
}

/// C: mjXWriter::Sensor (xml/xml_native_writer.cc:1877)
/// Calls: cxx:__ZN8mjCModel10NumObjectsE6mjtObj, cxx:__ZN9mjCSensor11get_objnameEv, cxx:__ZN9mjCSensor11get_refnameEv, cxx:__ZN9mjCSensor12get_userdataEv, cxx:__ZNK8mjCModel7SensorsEv, cxx:_mju_condataSize, cxx:_mju_error, cxx:_mju_type2Str
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_sensor(self_ptr: *mut mjXWriter, root: *mut tinyxml2__XMLElement) {
    todo!() // cxx:__ZN9mjXWriter6SensorEPN8tinyxml210XMLElementE
}

/// C: mjXWriter::Keyframe (xml/xml_native_writer.cc:2240)
/// Calls: cxx:__ZNK8mjCModel4KeysEv, cxx:__ZNK8mjCModel6BodiesEv
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_writer_keyframe(self_ptr: *mut mjXWriter, root: *mut tinyxml2__XMLElement) {
    todo!() // cxx:__ZN9mjXWriter8KeyframeEPN8tinyxml210XMLElementE
}

