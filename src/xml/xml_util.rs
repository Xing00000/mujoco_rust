//! Port of: xml/xml_util.cc
//! IR hash: 1b139f44af8230f9
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: ParseInfOrNan (xml/xml_util.cc:54)
#[allow(unused_variables, non_snake_case)]
pub fn parse_inf_or_nan(s: *const i32) -> *const () {
    todo!() // ParseInfOrNan
}

/// C: ResolveFilePath (xml/xml_util.cc:79)
/// Calls: FilePath::IsAbs, mju_closeResource
#[allow(unused_variables, non_snake_case)]
pub fn resolve_file_path(e: *mut XMLElement, filename: *const FilePath, dir: *const FilePath, vfs: *const mjVFS) -> FilePath {
    todo!() // ResolveFilePath
}

/// C: AccumulateFiles (xml/xml_util.cc:109)
#[allow(unused_variables, non_snake_case)]
pub fn accumulate_files(files: *mut i32, root: *mut tinyxml2__XMLElement, model_dir: *const FilePath) {
    todo!() // AccumulateFiles
}

/// C: mju_getXMLDependencies (xml/xml_util.cc:224)
/// Calls: mju_closeResource, mju_error, mju_getResourceDir, mju_readResource
#[allow(unused_variables, non_snake_case)]
pub fn mju_get_xml_dependencies(filename: *const i8, dependencies: *mut i32) {
    todo!() // mju_getXMLDependencies
}

/// C: mjXSchema::GetError (xml/xml_util.cc:384)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_schema_get_error() -> i32 {
    todo!() // mjXSchema::GetError
}

/// C: printspace (xml/xml_util.cc:391)
#[allow(unused_variables, non_snake_case)]
pub fn printspace(str: *mut i32, n: i32, space: *const i8) {
    todo!() // printspace
}

/// C: mjXSchema::Print (xml/xml_util.cc:400)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_schema_print(str: *mut i32, level: i32) {
    todo!() // mjXSchema::Print
}

/// C: mjXSchema::PrintHTML (xml/xml_util.cc:434)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_schema_print_html(str: *mut i32, level: i32, pad: bool) {
    todo!() // mjXSchema::PrintHTML
}

/// C: mjXUtil::FindKey (xml/xml_util.cc:687)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_find_key(map: *const mjMap, mapsz: i32, key: i32) -> i32 {
    todo!() // mjXUtil::FindKey
}

/// C: mjXUtil::FindValue (xml/xml_util.cc:700)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_find_value(map: *const mjMap, mapsz: i32, value: i32) -> i32 {
    todo!() // mjXUtil::FindValue
}

/// C: mjXUtil::ReadAttrVec (xml/xml_util.cc:714)
/// Calls: mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_read_attr_vec(elem: *mut XMLElement, attr: *const i8, required: bool) -> i32 {
    todo!() // mjXUtil::ReadAttrVec
}

/// C: mjXUtil::ReadAttrStr (xml/xml_util.cc:755)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_read_attr_str(elem: *mut XMLElement, attr: *const i8, required: bool) -> i32 {
    todo!() // mjXUtil::ReadAttrStr
}

/// C: mjXUtil::ReadAttr (xml/xml_util.cc:810)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_read_attr(elem: *mut XMLElement, attr: *const i8, len: i32, data: *mut T, text: *mut i32, required: bool, exact: bool) -> i32 {
    todo!() // mjXUtil::ReadAttr
}

/// C: mjXUtil::ReadQuat (xml/xml_util.cc:846)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_read_quat(elem: *mut XMLElement, attr: *const i8, data: *mut f64, text: *mut i32, required: bool) -> i32 {
    todo!() // mjXUtil::ReadQuat
}

/// C: mjXUtil::ReadVector (xml/xml_util.cc:860)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_read_vector(elem: *mut XMLElement, attr: *const i8, vec: *mut i32, text: *mut i32, required: bool) -> i32 {
    todo!() // mjXUtil::ReadVector
}

/// C: mjXUtil::ReadAttrTxt (xml/xml_util.cc:874)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_read_attr_txt(elem: *mut tinyxml2__XMLElement, attr: *const i8, text: *mut i32, required: bool) -> bool {
    todo!() // mjXUtil::ReadAttrTxt
}

/// C: mjXUtil::Vector2String (xml/xml_util.cc:898)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_vector2string(txt: *mut i32, vec: *const i32, ncol: i32) {
    todo!() // mjXUtil::Vector2String
}

/// C: mjXUtil::FindSubElem (xml/xml_util.cc:914)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_find_sub_elem(elem: *mut XMLElement, name: i32, required: bool) -> *mut XMLElement {
    todo!() // mjXUtil::FindSubElem
}

/// C: isint (xml/xml_util.cc:1004)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn isint(x: f64) -> bool {
    todo!() // isint
}

/// C: Round (xml/xml_util.cc:1010)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn round(x: f64) -> i32 {
    todo!() // Round
}

/// C: mjXUtil::WriteAttr (xml/xml_util.cc:1021)
/// Calls: _mjPRIVATE__get_xml_precision
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_write_attr(elem: *mut XMLElement, name: i32, n: i32, data: *const T, def: *const T, trim: bool) {
    todo!() // mjXUtil::WriteAttr
}

/// C: mjXUtil::WriteVector (xml/xml_util.cc:1084)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_write_vector(elem: *mut XMLElement, name: i32, vec: *const i32) {
    todo!() // mjXUtil::WriteVector
}

/// C: mjXUtil::WriteAttrTxt (xml/xml_util.cc:1123)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_write_attr_txt(elem: *mut XMLElement, name: i32, value: i32) {
    todo!() // mjXUtil::WriteAttrTxt
}

/// C: mjXUtil::WriteAttrInt (xml/xml_util.cc:1136)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_write_attr_int(elem: *mut XMLElement, name: i32, data: i32, def: i32) {
    todo!() // mjXUtil::WriteAttrInt
}

/// C: mjXUtil::WriteAttrKey (xml/xml_util.cc:1148)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_write_attr_key(elem: *mut XMLElement, name: i32, map: *const mjMap, mapsz: i32, data: i32, def: i32) {
    todo!() // mjXUtil::WriteAttrKey
}

/// C: mjXUtil::WriteAttrKeys (xml/xml_util.cc:1160)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_write_attr_keys(elem: *mut XMLElement, name: i32, map: *const mjMap, mapsz: i32, data: *mut i32, ndata: i32, def: i32) {
    todo!() // mjXUtil::WriteAttrKeys
}

/// C: mjCopyError (xml/xml_util.h:32)
#[allow(unused_variables, non_snake_case)]
pub fn mj_copy_error(dst: *mut i8, src: *const i8, maxlen: i32) {
    todo!() // mjCopyError
}

/// C: FirstChildElement (xml/xml_util.h:36)
#[allow(unused_variables, non_snake_case)]
pub fn first_child_element(e: *mut XMLElement, name: *const i8) -> *mut XMLElement {
    todo!() // FirstChildElement
}

/// C: NextSiblingElement (xml/xml_util.h:37)
/// Calls: FirstChildElement
#[allow(unused_variables, non_snake_case)]
pub fn next_sibling_element(e: *mut XMLElement, name: *const i8) -> *mut XMLElement {
    todo!() // NextSiblingElement
}

/// C: mjXSchema::NameMatch (xml/xml_util.h:61)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_schema_name_match(elem: *mut tinyxml2__XMLElement, level: i32) -> bool {
    todo!() // mjXSchema::NameMatch
}

/// C: mjXSchema::Check (xml/xml_util.h:62)
/// Calls: FirstChildElement, NextSiblingElement, mjXSchema::NameMatch
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_schema_check(elem: *mut tinyxml2__XMLElement, level: i32) -> *mut tinyxml2__XMLElement {
    todo!() // mjXSchema::Check
}

/// C: mjXUtil::SameVector (xml/xml_util.h:91)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_same_vector(vec1: *const T, vec2: *const T, n: i32) -> bool {
    todo!() // mjXUtil::SameVector
}

/// C: mjXUtil::ReadAttrFile (xml/xml_util.h:112)
/// Calls: ResolveFilePath
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_read_attr_file(elem: *mut tinyxml2__XMLElement, attr: *const i8, vfs: *const mjVFS, dir: *const mujoco__user__FilePath, required: bool) -> *const () {
    todo!() // mjXUtil::ReadAttrFile
}

/// C: mjXUtil::ReadAttrNum (xml/xml_util.h:119)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_read_attr_num(elem: *mut tinyxml2__XMLElement, attr: *const i8, required: bool) -> *const () {
    todo!() // mjXUtil::ReadAttrNum
}

/// C: mjXUtil::ReadAttrArr (xml/xml_util.h:126)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_read_attr_arr(elem: *mut tinyxml2__XMLElement, attr: *const i8, required: bool) -> *const () {
    todo!() // mjXUtil::ReadAttrArr
}

/// C: mjXUtil::ReadAttrInt (xml/xml_util.h:167)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_read_attr_int(elem: *mut tinyxml2__XMLElement, attr: *const i8, data: *mut i32, required: bool) -> bool {
    todo!() // mjXUtil::ReadAttrInt
}

/// C: mjXUtil::MapValue (xml/xml_util.h:179)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_map_value(elem: *mut tinyxml2__XMLElement, attr: *const i8, data: *mut i32, map: *const mjMap, mapSz: i32, required: bool) -> bool {
    todo!() // mjXUtil::MapValue
}

/// C: mjXUtil::MapValues (xml/xml_util.h:183)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_map_values(elem: *mut tinyxml2__XMLElement, attr: *const i8, data: *mut i32, map: *const mjMap, mapSz: i32, required: bool) -> i32 {
    todo!() // mjXUtil::MapValues
}

/// C: mjXUtil::ReadAttrValues (xml/xml_util.h:213)
/// Calls: mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_read_attr_values(elem: *mut tinyxml2__XMLElement, attr: *const i8, push: *const (), max: i32) -> bool {
    todo!() // mjXUtil::ReadAttrValues
}

