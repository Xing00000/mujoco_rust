//! Port of: xml/xml_util.cc
//! IR hash: e878892ca48fe222
//! CODEGEN: signatures locked. Only fill todo!() bodies.

use crate::types::*;

/// C: ParseInfOrNan (xml/xml_util.cc:54)
#[allow(unused_variables, non_snake_case)]
pub fn parse_inf_or_nan(s: *const std__string) -> *const () {
    todo!() // ParseInfOrNan
}

/// C: ResolveFilePath (xml/xml_util.cc:79)
/// Calls: FilePath::IsAbs, FilePath::c_str, mjXUtil::ReadAttrStr, mju_closeResource, mju_openResource
#[allow(unused_variables, non_snake_case)]
pub fn resolve_file_path(e: *mut XMLElement, filename: *const FilePath, dir: *const FilePath, vfs: *const mjVFS) -> FilePath {
    todo!() // ResolveFilePath
}

/// C: AccumulateFiles (xml/xml_util.cc:109)
/// Calls: FilePath::IsAbs, FilePath::Str, mjXUtil::ReadAttrFile, mjXUtil::ReadAttrStr, mju_getXMLDependencies
#[allow(unused_variables, non_snake_case)]
pub fn accumulate_files(files: *const (), root: *mut tinyxml2__XMLElement, model_dir: *const FilePath) {
    // NOTE: signature changed from previous IR version
    // Previous params: (files : * mut i32, root : * mut tinyxml2__XMLElement, model_dir : * const FilePath)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mju_getXMLDependencies (xml/xml_util.cc:224)
/// Calls: AccumulateFiles, mju_closeResource, mju_error, mju_getResourceDir, mju_openResource, mju_readResource
#[allow(unused_variables, non_snake_case)]
pub fn mju_get_xml_dependencies(filename: *const i8, dependencies: *mut mjStringVec) {
    // NOTE: signature changed from previous IR version
    // Previous params: (filename : * const i8, dependencies : * mut i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: printspace (xml/xml_util.cc:391)
#[allow(unused_variables, non_snake_case)]
pub fn printspace(str: *mut std__stringstream, n: i32, space: *const i8) {
    todo!() // printspace
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

/// C: mjCopyError (xml/xml_util.h:32)
#[allow(unused_variables, non_snake_case)]
pub fn mj_copy_error(dst: *mut i8, src: *const i8, maxlen: i32) {
    // SAFETY: dst and src are valid C strings from caller, maxlen > 0 guaranteed by check
    unsafe {
        if dst.is_null() || maxlen <= 0 {
            return;
        }
        let n = maxlen as usize;
        let mut i = 0usize;
        while i < n - 1 {
            let c = *src.add(i);
            if c == 0 {
                break;
            }
            *dst.add(i) = c;
            i += 1;
        }
        // fill rest with zeros (strncpy behavior)
        while i < n {
            *dst.add(i) = 0;
            i += 1;
        }
        // null-terminate last byte unconditionally
        *dst.add(n - 1) = 0;
    }
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

/// C: mjXSchema::GetError (xml/xml_util.h:57)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_schema_get_error(self_ptr: *mut mjXSchema) -> std__string {
    todo!() // mjXSchema::GetError
}

/// C: mjXSchema::Print (xml/xml_util.h:58)
/// Calls: printspace
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_schema_print(self_ptr: *mut mjXSchema, str: *mut std__stringstream, level: i32) {
    todo!() // mjXSchema::Print
}

/// C: mjXSchema::PrintHTML (xml/xml_util.h:59)
/// Calls: printspace
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_schema_print_html(self_ptr: *mut mjXSchema, str: *mut std__stringstream, level: i32, pad: bool) {
    todo!() // mjXSchema::PrintHTML
}

/// C: mjXSchema::NameMatch (xml/xml_util.h:61)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_schema_name_match(self_ptr: *mut mjXSchema, elem: *mut tinyxml2__XMLElement, level: i32) -> bool {
    todo!() // mjXSchema::NameMatch
}

/// C: mjXSchema::Check (xml/xml_util.h:62)
/// Calls: FirstChildElement, NextSiblingElement, mjXSchema::NameMatch, sprintf_arr
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_schema_check(self_ptr: *mut mjXSchema, elem: *mut tinyxml2__XMLElement, level: i32) -> *mut tinyxml2__XMLElement {
    todo!() // mjXSchema::Check
}

/// C: mjXUtil::SameVector (xml/xml_util.h:91)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_same_vector(vec1: *const T, vec2: *const T, n: i32) -> bool {
    todo!() // mjXUtil::SameVector
}

/// C: mjXUtil::FindKey (xml/xml_util.h:94)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_find_key(map: *const mjMap, mapsz: i32, key: string) -> i32 {
    todo!() // mjXUtil::FindKey
}

/// C: mjXUtil::FindValue (xml/xml_util.h:97)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_find_value(map: *const mjMap, mapsz: i32, value: i32) -> std__string {
    todo!() // mjXUtil::FindValue
}

/// C: mjXUtil::ReadAttrVec (xml/xml_util.h:101)
/// Calls: mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_read_attr_vec(elem: *mut tinyxml2__XMLElement, attr: *const i8, required: bool) -> *const () {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjXUtil, elem : * mut XMLElement, attr : * const i8, required : bool)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: mjXUtil::ReadAttrStr (xml/xml_util.h:106)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_read_attr_str(elem: *mut tinyxml2__XMLElement, attr: *const i8, required: bool) -> *const () {
    todo!() // mjXUtil::ReadAttrStr
}

/// C: mjXUtil::ReadAttrFile (xml/xml_util.h:112)
/// Calls: ResolveFilePath, mjXUtil::ReadAttrStr
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_read_attr_file(elem: *mut tinyxml2__XMLElement, attr: *const i8, vfs: *const mjVFS, dir: *const mujoco__user__FilePath, required: bool) -> *const () {
    todo!() // mjXUtil::ReadAttrFile
}

/// C: mjXUtil::ReadAttrNum (xml/xml_util.h:119)
/// Calls: mjXUtil::ReadAttrArr
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_read_attr_num(elem: *mut tinyxml2__XMLElement, attr: *const i8, required: bool) -> *const () {
    todo!() // mjXUtil::ReadAttrNum
}

/// C: mjXUtil::ReadAttrArr (xml/xml_util.h:126)
/// Calls: mjXUtil::ReadAttrValues
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_read_attr_arr(elem: *mut tinyxml2__XMLElement, attr: *const i8, required: bool) -> *const () {
    todo!() // mjXUtil::ReadAttrArr
}

/// C: mjXUtil::ReadAttr (xml/xml_util.h:151)
/// Calls: FilePath::size, mjCActuator::act, mjXUtil::ReadAttrVec
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_read_attr(elem: *mut tinyxml2__XMLElement, attr: *const i8, len: i32, data: *mut T, text: *mut std__string, required: bool, exact: bool) -> i32 {
    todo!() // mjXUtil::ReadAttr
}

/// C: mjXUtil::ReadQuat (xml/xml_util.h:155)
/// Calls: mjXUtil::ReadAttr
/// ⚠️ BITEXACT RULES:
///   1. Copy exact C accumulation order (no iter().sum())
///   2. No f64::mul_add() (FMA changes precision)
///   3. No algebraic simplification
///   4. No iter().sum()/product() (order undefined)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_read_quat(elem: *mut tinyxml2__XMLElement, attr: *const i8, data: *mut f64, text: *mut std__string, required: bool) -> i32 {
    todo!() // mjXUtil::ReadQuat
}

/// C: mjXUtil::ReadVector (xml/xml_util.h:159)
/// Calls: mjXUtil::ReadAttrVec
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_read_vector(elem: *mut tinyxml2__XMLElement, attr: *const i8, vec: *const (), text: *mut std__string, required: bool) -> i32 {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjXUtil, elem : * mut XMLElement, attr : * const i8, vec : * mut i32, text : * mut std__string, required : bool)
    // Previous return: i32
    todo!("re-translate: params renamed")
}

/// C: mjXUtil::ReadAttrTxt (xml/xml_util.h:163)
/// Calls: mjXUtil::ReadAttrStr
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_read_attr_txt(elem: *mut tinyxml2__XMLElement, attr: *const i8, text: *mut std__string, required: bool) -> bool {
    todo!() // mjXUtil::ReadAttrTxt
}

/// C: mjXUtil::ReadAttrInt (xml/xml_util.h:167)
/// Calls: mjXUtil::ReadAttrNum
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_read_attr_int(elem: *mut tinyxml2__XMLElement, attr: *const i8, data: *mut i32, required: bool) -> bool {
    todo!() // mjXUtil::ReadAttrInt
}

/// C: mjXUtil::Vector2String (xml/xml_util.h:171)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_vector2string(txt: *mut std__string, vec: *const (), ncol: i32) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjXUtil, txt : * mut std__string, vec : * const i32, ncol : i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjXUtil::FindSubElem (xml/xml_util.h:175)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_find_sub_elem(elem: *mut tinyxml2__XMLElement, name: string, required: bool) -> *mut tinyxml2__XMLElement {
    todo!() // mjXUtil::FindSubElem
}

/// C: mjXUtil::MapValue (xml/xml_util.h:179)
/// Calls: mjXUtil::FindKey, mjXUtil::ReadAttrStr
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_map_value(elem: *mut tinyxml2__XMLElement, attr: *const i8, data: *mut i32, map: *const mjMap, mapSz: i32, required: bool) -> bool {
    todo!() // mjXUtil::MapValue
}

/// C: mjXUtil::MapValues (xml/xml_util.h:183)
/// Calls: mjXUtil::FindKey, mjXUtil::ReadAttrStr
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_map_values(elem: *mut tinyxml2__XMLElement, attr: *const i8, data: *mut i32, map: *const mjMap, mapSz: i32, required: bool) -> i32 {
    todo!() // mjXUtil::MapValues
}

/// C: mjXUtil::WriteAttr (xml/xml_util.h:188)
/// Calls: _mjPRIVATE__get_xml_precision, mjXUtil::WriteAttrTxt
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_write_attr(elem: *mut tinyxml2__XMLElement, name: string, n: i32, data: *const T, def: *const T, trim: bool) {
    todo!() // mjXUtil::WriteAttr
}

/// C: mjXUtil::WriteVector (xml/xml_util.h:192)
/// Calls: mjXUtil::WriteAttr
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_write_vector(elem: *mut tinyxml2__XMLElement, name: string, vec: *const ()) {
    // NOTE: signature changed from previous IR version
    // Previous params: (self_ptr : * mut mjXUtil, elem : * mut XMLElement, name : string, vec : * const i32)
    // Previous return: ()
    todo!("re-translate: params renamed")
}

/// C: mjXUtil::WriteAttrTxt (xml/xml_util.h:198)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_write_attr_txt(elem: *mut tinyxml2__XMLElement, name: string, value: string) {
    todo!() // mjXUtil::WriteAttrTxt
}

/// C: mjXUtil::WriteAttrInt (xml/xml_util.h:201)
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_write_attr_int(elem: *mut tinyxml2__XMLElement, name: string, data: i32, def: i32) {
    todo!() // mjXUtil::WriteAttrInt
}

/// C: mjXUtil::WriteAttrKey (xml/xml_util.h:204)
/// Calls: mjXUtil::FindValue, mjXUtil::WriteAttrTxt
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_write_attr_key(elem: *mut tinyxml2__XMLElement, name: string, map: *const mjMap, mapsz: i32, data: i32, def: i32) {
    todo!() // mjXUtil::WriteAttrKey
}

/// C: mjXUtil::WriteAttrKeys (xml/xml_util.h:208)
/// Calls: mjXUtil::FindValue, mjXUtil::WriteAttrTxt
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_write_attr_keys(elem: *mut XMLElement, name: string, map: *const mjMap, mapsz: i32, data: *mut i32, ndata: i32, def: i32) {
    todo!() // mjXUtil::WriteAttrKeys
}

/// C: mjXUtil::ReadAttrValues (xml/xml_util.h:213)
/// Calls: mju_warning
#[allow(unused_variables, non_snake_case)]
pub fn mj_x_util_read_attr_values(elem: *mut tinyxml2__XMLElement, attr: *const i8, push: *const (), max: i32) -> bool {
    todo!() // mjXUtil::ReadAttrValues
}

