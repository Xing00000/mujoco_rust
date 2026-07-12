//! Port of: xml/xml_util.cc
//! IR hash: c6d98e4f4b63b7f2
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: ParseInfOrNan (xml/xml_util.cc:54)
#[allow(unused_variables, non_snake_case)]
pub fn parse_inf_or_nan(s: *const std__string) -> *const () {
    // NOTE: signature changed from previous IR version
    // Previous params: (s : * const std__string)
    // Previous return: * const ()
    todo!("re-translate: params renamed")
}

/// C: ResolveFilePath (xml/xml_util.cc:79)
/// Calls: FilePath::IsAbs, FilePath::c_str, mju_closeResource, mju_openResource
#[allow(unused_variables, non_snake_case)]
pub fn resolve_file_path(e: *mut XMLElement, filename: *const FilePath, dir: *const FilePath, vfs: *const mjVFS) -> FilePath {
    // NOTE: signature changed from previous IR version
    // Previous params: (e : * mut XMLElement, filename : * const FilePath, dir : * const FilePath, vfs : * const mjVFS)
    // Previous return: FilePath
    todo!("re-translate: params renamed")
}

/// C: AccumulateFiles (xml/xml_util.cc:109)
#[allow(unused_variables, non_snake_case)]
pub fn accumulate_files(files: *mut i32, root: *mut tinyxml2__XMLElement, model_dir: *const FilePath) {
    // NOTE: signature changed from previous IR version
    // Previous params: (files : * mut i32, root : * mut tinyxml2__XMLElement, model_dir : * const FilePath)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mju_getXMLDependencies (xml/xml_util.cc:224)
/// Calls: mju_closeResource, mju_error, mju_getResourceDir, mju_openResource, mju_readResource
#[allow(unused_variables, non_snake_case)]
pub fn mju_get_xml_dependencies(filename: *const i8, dependencies: *mut i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (filename : * const i8, dependencies : * mut i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: printspace (xml/xml_util.cc:391)
#[allow(unused_variables, non_snake_case)]
pub fn printspace(str: *mut std__stringstream, n: i32, space: *const i8) {
    // NOTE: signature changed from previous IR version
    // Previous params: (str : * mut std__stringstream, n : i32, space : * const i8)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjXUtil::ReadAttrVec (xml/xml_util.cc:714)
/// Calls: mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_read_attr_vec(self_ptr: *mut mjXUtil, elem: *mut XMLElement, attr: *const i8, required: bool) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjXUtil, elem : * mut XMLElement, attr : * const i8, required : bool)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: mjXUtil::ReadVector (xml/xml_util.cc:860)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_read_vector(self_ptr: *mut mjXUtil, elem: *mut XMLElement, attr: *const i8, vec: *mut i32, text: *mut std__string, required: bool) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjXUtil, elem : * mut XMLElement, attr : * const i8, vec : * mut i32, text : * mut std__string, required : bool)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: mjXUtil::Vector2String (xml/xml_util.cc:898)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_vector2string(self_ptr: *mut mjXUtil, txt: *mut std__string, vec: *const i32, ncol: i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjXUtil, txt : * mut std__string, vec : * const i32, ncol : i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: isint (xml/xml_util.cc:1004)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn isint(x: f64) -> bool {
    // NOTE: signature changed from previous IR version
    // Previous params: (x : f64)
    // Previous return: bool
    todo!("re-translate: params renamed")
}

/// C: Round (xml/xml_util.cc:1010)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn round(x: f64) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (x : f64)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: mjXUtil::WriteVector (xml/xml_util.cc:1084)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_write_vector(self_ptr: *mut mjXUtil, elem: *mut XMLElement, name: string, vec: *const i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjXUtil, elem : * mut XMLElement, name : string, vec : * const i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjCopyError (xml/xml_util.h:32)
#[allow(unused_variables, non_snake_case)]
pub fn mj_copy_error(dst: *mut i8, src: *const i8, maxlen: i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (dst : * mut i8, src : * const i8, maxlen : i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: FirstChildElement (xml/xml_util.h:36)
#[allow(unused_variables, non_snake_case)]
pub fn first_child_element(e: *mut XMLElement, name: *const i8) -> *mut XMLElement {
    // NOTE: signature changed from previous IR version
    // Previous params: (e : * mut XMLElement, name : * const i8)
    // Previous return: * mut XMLElement
    todo!("re-translate: params renamed")
}

/// C: NextSiblingElement (xml/xml_util.h:37)
/// Calls: FirstChildElement
#[allow(unused_variables, non_snake_case)]
pub fn next_sibling_element(e: *mut XMLElement, name: *const i8) -> *mut XMLElement {
    // NOTE: signature changed from previous IR version
    // Previous params: (e : * mut XMLElement, name : * const i8)
    // Previous return: * mut XMLElement
    todo!("re-translate: params renamed")
}

/// C: mjXSchema::GetError (xml/xml_util.h:57)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_schema_get_error(self_ptr: *mut mjXSchema) -> std__string {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjXSchema)
    // Previous return: std__string
    todo!("re-translate: params renamed")
}

/// C: mjXSchema::Print (xml/xml_util.h:58)
/// Calls: printspace
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_schema_print(self_ptr: *mut mjXSchema, str: *mut std__stringstream, level: i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjXSchema, str : * mut std__stringstream, level : i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjXSchema::PrintHTML (xml/xml_util.h:59)
/// Calls: printspace
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_schema_print_html(self_ptr: *mut mjXSchema, str: *mut std__stringstream, level: i32, pad: bool) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjXSchema, str : * mut std__stringstream, level : i32, pad : bool)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjXSchema::NameMatch (xml/xml_util.h:61)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_schema_name_match(self_ptr: *mut mjXSchema, elem: *mut tinyxml2__XMLElement, level: i32) -> bool {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjXSchema, elem : * mut tinyxml2__XMLElement, level : i32)
    // Previous return: bool
    todo!("re-translate: params renamed")
}

/// C: mjXSchema::Check (xml/xml_util.h:62)
/// Calls: FirstChildElement, NextSiblingElement, mjXSchema::NameMatch
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_schema_check(self_ptr: *mut mjXSchema, elem: *mut tinyxml2__XMLElement, level: i32) -> *mut tinyxml2__XMLElement {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjXSchema, elem : * mut tinyxml2__XMLElement, level : i32)
    // Previous return: * mut tinyxml2__XMLElement
    todo!("re-translate: params renamed")
}

/// C: mjXUtil::SameVector (xml/xml_util.h:91)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_same_vector(vec1: *const T, vec2: *const T, n: i32) -> bool {
    // NOTE: signature changed from previous IR version
    // Previous params: (vec1 : * const T, vec2 : * const T, n : i32)
    // Previous return: bool
    todo!("re-translate: params renamed")
}

/// C: mjXUtil::FindKey (xml/xml_util.h:94)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_find_key(map: *const mjMap, mapsz: i32, key: string) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (map : * const mjMap, mapsz : i32, key : string)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: mjXUtil::FindValue (xml/xml_util.h:97)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_find_value(map: *const mjMap, mapsz: i32, value: i32) -> std__string {
    // NOTE: signature changed from previous IR version
    // Previous params: (map : * const mjMap, mapsz : i32, value : i32)
    // Previous return: std__string
    todo!("re-translate: params renamed")
}

/// C: mjXUtil::ReadAttrStr (xml/xml_util.h:106)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_read_attr_str(elem: *mut tinyxml2__XMLElement, attr: *const i8, required: bool) -> *const () {
    // NOTE: signature changed from previous IR version
    // Previous params: (elem : * mut tinyxml2__XMLElement, attr : * const i8, required : bool)
    // Previous return: * const ()
    todo!("re-translate: params renamed")
}

/// C: mjXUtil::ReadAttrFile (xml/xml_util.h:112)
/// Calls: ResolveFilePath
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_read_attr_file(elem: *mut tinyxml2__XMLElement, attr: *const i8, vfs: *const mjVFS, dir: *const mujoco__user__FilePath, required: bool) -> *const () {
    // NOTE: signature changed from previous IR version
    // Previous params: (elem : * mut tinyxml2__XMLElement, attr : * const i8, vfs : * const mjVFS, dir : * const mujoco__user__FilePath, required : bool)
    // Previous return: * const ()
    todo!("re-translate: params renamed")
}

/// C: mjXUtil::ReadAttrNum (xml/xml_util.h:119)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_read_attr_num(elem: *mut tinyxml2__XMLElement, attr: *const i8, required: bool) -> *const () {
    // NOTE: signature changed from previous IR version
    // Previous params: (elem : * mut tinyxml2__XMLElement, attr : * const i8, required : bool)
    // Previous return: * const ()
    todo!("re-translate: params renamed")
}

/// C: mjXUtil::ReadAttrArr (xml/xml_util.h:126)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_read_attr_arr(elem: *mut tinyxml2__XMLElement, attr: *const i8, required: bool) -> *const () {
    // NOTE: signature changed from previous IR version
    // Previous params: (elem : * mut tinyxml2__XMLElement, attr : * const i8, required : bool)
    // Previous return: * const ()
    todo!("re-translate: params renamed")
}

/// C: mjXUtil::ReadAttr (xml/xml_util.h:151)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_read_attr(elem: *mut tinyxml2__XMLElement, attr: *const i8, len: i32, data: *mut T, text: *mut std__string, required: bool, exact: bool) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (elem : * mut tinyxml2__XMLElement, attr : * const i8, len : i32, data : * mut T, text : * mut std__string, required : bool, exact : bool)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: mjXUtil::ReadQuat (xml/xml_util.h:155)
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_read_quat(elem: *mut tinyxml2__XMLElement, attr: *const i8, data: *mut f64, text: *mut std__string, required: bool) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (elem : * mut tinyxml2__XMLElement, attr : * const i8, data : * mut f64, text : * mut std__string, required : bool)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: mjXUtil::ReadAttrTxt (xml/xml_util.h:163)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_read_attr_txt(elem: *mut tinyxml2__XMLElement, attr: *const i8, text: *mut std__string, required: bool) -> bool {
    // NOTE: signature changed from previous IR version
    // Previous params: (elem : * mut tinyxml2__XMLElement, attr : * const i8, text : * mut std__string, required : bool)
    // Previous return: bool
    todo!("re-translate: params renamed")
}

/// C: mjXUtil::ReadAttrInt (xml/xml_util.h:167)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_read_attr_int(elem: *mut tinyxml2__XMLElement, attr: *const i8, data: *mut i32, required: bool) -> bool {
    // NOTE: signature changed from previous IR version
    // Previous params: (elem : * mut tinyxml2__XMLElement, attr : * const i8, data : * mut i32, required : bool)
    // Previous return: bool
    todo!("re-translate: params renamed")
}

/// C: mjXUtil::FindSubElem (xml/xml_util.h:175)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_find_sub_elem(elem: *mut tinyxml2__XMLElement, name: string, required: bool) -> *mut tinyxml2__XMLElement {
    // NOTE: signature changed from previous IR version
    // Previous params: (elem : * mut tinyxml2__XMLElement, name : string, required : bool)
    // Previous return: * mut tinyxml2__XMLElement
    todo!("re-translate: params renamed")
}

/// C: mjXUtil::MapValue (xml/xml_util.h:179)
/// Calls: mjXUtil::FindKey
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_map_value(elem: *mut tinyxml2__XMLElement, attr: *const i8, data: *mut i32, map: *const mjMap, mapSz: i32, required: bool) -> bool {
    // NOTE: signature changed from previous IR version
    // Previous params: (elem : * mut tinyxml2__XMLElement, attr : * const i8, data : * mut i32, map : * const mjMap, mapSz : i32, required : bool)
    // Previous return: bool
    todo!("re-translate: params renamed")
}

/// C: mjXUtil::MapValues (xml/xml_util.h:183)
/// Calls: mjXUtil::FindKey
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_map_values(elem: *mut tinyxml2__XMLElement, attr: *const i8, data: *mut i32, map: *const mjMap, mapSz: i32, required: bool) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (elem : * mut tinyxml2__XMLElement, attr : * const i8, data : * mut i32, map : * const mjMap, mapSz : i32, required : bool)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: mjXUtil::WriteAttr (xml/xml_util.h:188)
/// Calls: _mjPRIVATE__get_xml_precision, mjXUtil::WriteAttrTxt
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_write_attr(elem: *mut tinyxml2__XMLElement, name: string, n: i32, data: *const T, def: *const T, trim: bool) {
    // NOTE: signature changed from previous IR version
    // Previous params: (elem : * mut tinyxml2__XMLElement, name : string, n : i32, data : * const T, def : * const T, trim : bool)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjXUtil::WriteAttrTxt (xml/xml_util.h:198)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_write_attr_txt(elem: *mut tinyxml2__XMLElement, name: string, value: string) {
    // NOTE: signature changed from previous IR version
    // Previous params: (elem : * mut tinyxml2__XMLElement, name : string, value : string)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjXUtil::WriteAttrInt (xml/xml_util.h:201)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_write_attr_int(elem: *mut tinyxml2__XMLElement, name: string, data: i32, def: i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (elem : * mut tinyxml2__XMLElement, name : string, data : i32, def : i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjXUtil::WriteAttrKey (xml/xml_util.h:204)
/// Calls: mjXUtil::WriteAttrTxt
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_write_attr_key(elem: *mut tinyxml2__XMLElement, name: string, map: *const mjMap, mapsz: i32, data: i32, def: i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (elem : * mut tinyxml2__XMLElement, name : string, map : * const mjMap, mapsz : i32, data : i32, def : i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjXUtil::WriteAttrKeys (xml/xml_util.h:208)
/// Calls: mjXUtil::WriteAttrTxt
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_write_attr_keys(elem: *mut XMLElement, name: string, map: *const mjMap, mapsz: i32, data: *mut i32, ndata: i32, def: i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (elem : * mut XMLElement, name : string, map : * const mjMap, mapsz : i32, data : * mut i32, ndata : i32, def : i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjXUtil::ReadAttrValues (xml/xml_util.h:213)
/// Calls: mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_read_attr_values(elem: *mut tinyxml2__XMLElement, attr: *const i8, push: *const (), max: i32) -> bool {
    // NOTE: signature changed from previous IR version
    // Previous params: (elem : * mut tinyxml2__XMLElement, attr : * const i8, push : * const (), max : i32)
    // Previous return: bool
    todo!("re-translate: params renamed")
}

